//! The persisted class under deterministic simulation (kiln's CP-T2.2 step 6, AC-T2.2.5;
//! `docs/design/persisted-sim.md`): three nodes, each host on a shard of hyper-rt's simulation
//! runtime, every member's log on hyper-sim's block device. A seed draws a run: publishes through
//! every node; kill -9 of any node at an await of its host or at a cut of its disk (before it, in
//! it, after it), its devices then kept as a kill leaves them or as a power cut does; the network
//! between two nodes cut one way or both; disks stalled, one or all at once; datagrams lost,
//! jittered and reordered. Then the network mends, every node starts again and catches up, and
//! every acknowledged publish is checked at its offset in every node's view, each producer's in its
//! order, none twice, the views alike.
//!
//! `QUIVER_SIM_SEEDS` names the seeds `faults_over_many_seeds` runs (`first..end`, by default a few
//! for the gate's time); a failing seed's trace is written under the test's directory, and
//! `QUIVER_SIM_REPLAY` names a trace to run again.
#![cfg(feature = "sim")]
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::disallowed_macros,
    clippy::disallowed_methods,
    clippy::cast_possible_truncation,
    clippy::too_many_lines,
    clippy::cognitive_complexity
)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::PathBuf;
use std::time::Duration;

use quiver::persisted::sim::{
    Answer, Crash, Doom, Kill, Settings, SimLoss, SimPath, When, World,
};
use quiver_api::{Class, Offset, Topic};

const TOPIC: &str = "sim";
const MS: u64 = 1_000_000;
/// Shape: the run's span of publishes and faults, before the network mends.
const RUN_NS: u64 = 6_000 * MS;
/// Bound: what a run waits, once mended, for every node to catch up: past it, it's said stuck.
const SETTLE_NS: u64 = 60_000 * MS;

/// The test's directory: each run's under it, and the cluster's identities, made once.
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("sim")
}

fn settings(name: &str, seed: u64, path: SimPath) -> Settings {
    let dir = root().join(format!("{name}-{seed}"));
    let _ = std::fs::remove_dir_all(&dir);
    Settings {
        seed,
        nodes: 3,
        dir,
        identities: root().join("identities-3"),
        path,
        quorum_wait: Duration::from_secs(3),
    }
}

fn topic() -> Topic {
    Topic {
        class: Class::Persisted,
        retain_messages: 1_000_000,
        retain_bytes: 1 << 30,
    }
}

/// SplitMix64: the run's draws.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: u64) -> u64 {
        self.next() % n.max(1)
    }

    fn range(&mut self, lo: u64, hi: u64) -> u64 {
        lo + self.below(hi.saturating_sub(lo))
    }

    fn chance(&mut self, per_mille: u64) -> bool {
        self.below(1000) < per_mille
    }
}

/// Runs `world` a millisecond at a time until `done`, at most `most_ns` of virtual time.
fn run_until_done(
    world: &mut World,
    most_ns: u64,
    mut done: impl FnMut(&mut World) -> bool,
) -> bool {
    let end = world.now_ns() + most_ns;
    while world.now_ns() < end {
        if done(world) {
            return true;
        }
        let next = world.now_ns() + MS;
        world.run_until(next);
    }
    done(world)
}

/// One publish: its payload, the node and the start of that node it went through, and its answer.
struct Publish {
    payload: Vec<u8>,
    node: u64,
    start: u64,
    answer: Answer<Offset>,
    outcome: Option<Result<Offset, String>>,
}

/// What a run saw and left: its trace, every publish's outcome, each node's view, and its digest.
struct Report {
    seed: u64,
    trace: String,
    /// What each node said of itself at the end of a run that never settled (not digested).
    diagnosis: String,
    /// Payload, node, start, outcome.
    publishes: Vec<(Vec<u8>, u64, u64, Result<Offset, String>)>,
    views: Vec<Vec<(Offset, Vec<u8>)>>,
    digest: u64,
    /// Why the run's checks failed, if they did.
    broken: Vec<String>,
}

/// FNV-1a over `bytes`, folded into `hash`.
fn fold(hash: u64, bytes: &[u8]) -> u64 {
    bytes.iter().fold(hash, |h, b| (h ^ u64::from(*b)).wrapping_mul(0x0100_0000_01b3))
}

/// The run `seed` draws, from the world's making to its checks, in the directory `name-<seed>`.
fn run(name: &str, seed: u64) -> Report {
    let mut rng = Rng(seed ^ 0x5157_4552_5349_4d00);
    let mut trace = String::new();
    let path = SimPath::in_order(rng.range(100_000, 2_000_000), rng.range(0, 400_000))
        .with_loss(SimLoss::random(rng.below(20_000) as u32));
    let _ = writeln!(trace, "seed {seed}: path {path:?}");
    let mut world = World::new(settings(name, seed, path)).unwrap();
    for node in 1..=3 {
        world.start(node, Kill::Never).unwrap();
    }
    world.run_until(300 * MS);
    // The topic, through node 1, asked again where refused, until every node holds it.
    let mut creating = world.create(1, TOPIC, topic());
    let created = run_until_done(&mut world, 30_000 * MS, |w| {
        if let Some(Err(_)) = creating.take() {
            creating = w.create(1, TOPIC, topic());
        }
        (1..=3).all(|n| w.holds(n, TOPIC))
    });
    assert!(created, "seed {seed}: the topic was never held everywhere");

    let mut publishes: Vec<Publish> = Vec::new();
    let mut starts = [1_u64; 3];
    // Each node down: when it may start again.
    let mut down: [Option<u64>; 3] = [None; 3];
    let mut mended_at: BTreeMap<(u64, u64), u64> = BTreeMap::new();
    let t0 = world.now_ns();
    let end = t0 + RUN_NS;
    let mut next_publish = t0;
    let mut next_fault = t0 + rng.range(100 * MS, 1_500 * MS);
    let mut sequence = 0_u64;
    while world.now_ns() < end {
        let now = world.now_ns();
        for node in 1..=3_u64 {
            let at = (node - 1) as usize;
            if down[at].is_none() && !world.alive(node) {
                let crash = match rng.below(3) {
                    0 => Crash::KeepAll,
                    1 => Crash::Random,
                    _ => Crash::LoseAll,
                };
                let why = world.ended(node).unwrap_or_default();
                world.crash(node, crash).unwrap();
                let back = now + rng.range(20 * MS, 2_500 * MS);
                let _ = writeln!(trace, "{now}: node {node} down ({why}), {crash:?}, back at {back}");
                down[at] = Some(back);
            }
            if let Some(back) = down[at]
                && now >= back
            {
                world.start(node, Kill::Never).unwrap();
                starts[at] += 1;
                down[at] = None;
                let _ = writeln!(trace, "{now}: node {node} starts again");
            }
        }
        for (&(a, b), &at) in &mended_at {
            if now >= at {
                world.mend(a, b).unwrap();
                let _ = writeln!(trace, "{now}: {a}-{b} mended");
            }
        }
        mended_at.retain(|_, at| now < *at);
        if now >= next_publish {
            let node = rng.range(1, 4);
            if world.alive(node) && world.holds(node, TOPIC) {
                sequence += 1;
                let payload = format!("{seed}/{node}/{}/{sequence}", starts[(node - 1) as usize]);
                let answer = world.publish(node, TOPIC, payload.as_bytes());
                publishes.push(Publish {
                    payload: payload.into_bytes(),
                    node,
                    start: starts[(node - 1) as usize],
                    answer,
                    outcome: None,
                });
            }
            next_publish = now + rng.range(2 * MS, 30 * MS);
        }
        if now >= next_fault {
            fault(&mut world, &mut rng, &mut trace, &mut mended_at, now);
            next_fault = now + rng.range(200 * MS, 2_000 * MS);
        }
        for p in &mut publishes {
            if p.outcome.is_none() {
                p.outcome = p.answer.take().map(|a| a.map_err(|e| e.to_string()));
            }
        }
        world.run_until(now + MS);
    }

    // Mended: every link, every node started again, until all have caught up.
    let now = world.now_ns();
    for (a, b) in [(1, 2), (1, 3), (2, 3)] {
        world.mend(a, b).unwrap();
    }
    for node in 1..=3_u64 {
        if !world.alive(node) {
            if down[(node - 1) as usize].is_none() {
                world.crash(node, Crash::KeepAll).unwrap();
            }
            world.start(node, Kill::Never).unwrap();
        }
        let _ = world.doom(node, Doom::Awaits(u64::MAX));
    }
    let _ = writeln!(trace, "{now}: mended, every node up");
    let settled = run_until_done(&mut world, SETTLE_NS, |w| {
        for p in &mut publishes {
            if p.outcome.is_none() {
                p.outcome = p.answer.take().map(|a| a.map_err(|e| e.to_string()));
            }
        }
        let applied: Vec<u64> = (1..=3)
            .map(|n| {
                w.stats(n)
                    .and_then(|s| s.topic(TOPIC).map(|g| g.applied))
                    .unwrap_or(0)
            })
            .collect();
        publishes.iter().all(|p| p.outcome.is_some())
            && applied.iter().all(|a| *a == applied[0])
    });
    // A little longer, for the views to take what was applied.
    let next = world.now_ns() + 500 * MS;
    world.run_until(next);
    // What each node says of itself, for a run that never settled: kept apart from the trace, as
    // it holds real-time measures (each log's flush times) no run repeats.
    let mut diagnosis = String::new();
    if !settled {
        for node in 1..=3 {
            let _ = writeln!(diagnosis, "node {node} at the end: {:?}", world.stats(node));
            let _ = writeln!(diagnosis, "node {node}'s course: {:?}", world.course(node));
            let _ = writeln!(diagnosis, "node {node}: {}", world.diagnose(node).unwrap_or_default());
        }
    }
    let views: Vec<Vec<(Offset, Vec<u8>)>> = (1..=3)
        .map(|node| {
            world
                .read(node, TOPIC)
                .unwrap_or_default()
                .into_iter()
                .map(|m| (m.offset, m.payload))
                .collect()
        })
        .collect();
    let mut digest = fold(0xcbf2_9ce4_8422_2325, trace.as_bytes());
    for node in 1..=3 {
        let (writes, flushes) = world.operations(node).unwrap();
        digest = fold(digest, &writes.to_le_bytes());
        digest = fold(digest, &flushes.to_le_bytes());
    }
    let fabric = format!("{:?}", world.fabric());
    digest = fold(digest, fabric.as_bytes());
    world.stop();
    let publishes: Vec<(Vec<u8>, u64, u64, Result<Offset, String>)> = publishes
        .into_iter()
        .map(|p| {
            let outcome = p.outcome.unwrap_or_else(|| Err("unanswered".to_owned()));
            (p.payload, p.node, p.start, outcome)
        })
        .collect();
    for (payload, node, start, outcome) in &publishes {
        digest = fold(digest, payload);
        digest = fold(digest, &[*node as u8, *start as u8]);
        digest = fold(digest, format!("{outcome:?}").as_bytes());
    }
    for view in &views {
        for (offset, payload) in view {
            digest = fold(digest, &offset.to_le_bytes());
            digest = fold(digest, payload);
        }
    }
    let mut broken = check(&publishes, &views);
    if !settled {
        broken.push("the nodes never caught up once mended".to_owned());
    }
    Report {
        seed,
        trace,
        diagnosis,
        publishes,
        views,
        digest,
        broken,
    }
}

/// One fault the run draws at `now`.
fn fault(
    world: &mut World,
    rng: &mut Rng,
    trace: &mut String,
    mended_at: &mut BTreeMap<(u64, u64), u64>,
    now: u64,
) {
    match rng.below(100) {
        0..=39 => {
            // A kill: of the topic's leader half the time, else of any node.
            let leader = (1..=3).find(|n| {
                world
                    .stats(*n)
                    .and_then(|s| s.topic(TOPIC).map(|g| g.leads))
                    .unwrap_or(false)
            });
            let node = match leader {
                Some(leader) if rng.chance(500) => leader,
                _ => rng.range(1, 4),
            };
            if !world.alive(node) {
                return;
            }
            let doom = if rng.chance(400) {
                Doom::Awaits(rng.below(400))
            } else {
                let when = match rng.below(3) {
                    0 => When::Before,
                    1 => When::During { ops: rng.below(4) },
                    _ => When::After,
                };
                Doom::Cuts {
                    after: rng.below(12),
                    when,
                }
            };
            world.doom(node, doom).unwrap();
            let _ = writeln!(trace, "{now}: node {node} doomed {doom:?}");
        }
        40..=64 => {
            let a = rng.range(1, 4);
            let b = (a % 3) + 1;
            let both = rng.chance(700);
            let until = now + rng.range(50 * MS, 2_000 * MS);
            world.cut(a, b, both).unwrap();
            mended_at.insert((a.min(b), a.max(b)), until);
            let _ = writeln!(trace, "{now}: {a}-{b} cut (both {both}) until {until}");
        }
        65..=84 => {
            let until = now + rng.range(200 * MS, 2_500 * MS);
            if rng.chance(300) {
                for node in 1..=3 {
                    world.stall(node, now, until).unwrap();
                }
                let _ = writeln!(trace, "{now}: every disk stalled until {until}");
            } else {
                let node = rng.range(1, 4);
                world.stall(node, now, until).unwrap();
                let _ = writeln!(trace, "{now}: node {node}'s disk stalled until {until}");
            }
        }
        _ => {}
    }
}

/// What a run broke: an acknowledged publish missing from a node's view or at another offset, a
/// producer's acknowledged publishes out of its order, a payload twice in a view, two views apart.
fn check(
    publishes: &[(Vec<u8>, u64, u64, Result<Offset, String>)],
    views: &[Vec<(Offset, Vec<u8>)>],
) -> Vec<String> {
    let mut broken = Vec::new();
    let at: Vec<BTreeMap<Offset, &Vec<u8>>> = views
        .iter()
        .map(|v| v.iter().map(|(o, p)| (*o, p)).collect())
        .collect();
    for (payload, node, start, outcome) in publishes {
        let Ok(offset) = outcome else { continue };
        for (n, view) in at.iter().enumerate() {
            if view.get(offset) != Some(&payload) {
                broken.push(format!(
                    "acknowledged {:?} (through node {node}, start {start}) at {offset}: node {} holds {:?} there",
                    String::from_utf8_lossy(payload),
                    n + 1,
                    view.get(offset).map(|p| String::from_utf8_lossy(p).into_owned())
                ));
            }
        }
    }
    let mut last: BTreeMap<(u64, u64), Offset> = BTreeMap::new();
    for (payload, node, start, outcome) in publishes {
        let Ok(offset) = outcome else { continue };
        if let Some(before) = last.insert((*node, *start), *offset)
            && before >= *offset
        {
            broken.push(format!(
                "{:?} acknowledged at {offset}, after its producer's at {before}",
                String::from_utf8_lossy(payload)
            ));
        }
    }
    for (n, view) in views.iter().enumerate() {
        let mut seen = BTreeSet::new();
        for (offset, payload) in view {
            if !seen.insert(payload) {
                broken.push(format!(
                    "node {} holds {:?} twice, again at {offset}",
                    n + 1,
                    String::from_utf8_lossy(payload)
                ));
            }
        }
    }
    for a in 0..at.len() {
        for b in a + 1..at.len() {
            for (offset, payload) in &at[a] {
                if let Some(other) = at[b].get(offset)
                    && other != payload
                {
                    broken.push(format!("nodes {} and {} differ at {offset}", a + 1, b + 1));
                }
            }
        }
    }
    broken
}

/// The seeds `QUIVER_SIM_SEEDS` names (`first..end`), or the gate's few.
fn seeds() -> std::ops::Range<u64> {
    std::env::var("QUIVER_SIM_SEEDS")
        .ok()
        .and_then(|s| {
            let (a, b) = s.split_once("..")?;
            Some(a.parse().ok()?..b.parse().ok()?)
        })
        .unwrap_or(1..4)
}

/// Writes a failing run's trace, and what it broke, where its seed's run was.
fn keep_trace(report: &Report) -> PathBuf {
    let planted = if cfg!(feature = "plant-own-log-commit") { "plant-" } else { "" };
    let path = root().join(format!("{planted}trace-{}.txt", report.seed));
    let mut text = format!(
        "seed {}\ndigest {:016x}\n{}\nbroken:\n",
        report.seed, report.digest, report.trace
    );
    for b in &report.broken {
        let _ = writeln!(text, "  {b}");
    }
    text.push_str(&report.diagnosis);
    let _ = std::fs::write(&path, text);
    path
}

/// With no fault, every publish through any node is acknowledged, at its own offset, and every
/// node's view holds each at that offset; and the run is the same run twice from its seed.
#[cfg(not(feature = "plant-own-log-commit"))]
#[test]
fn a_seeds_run_is_the_same_run_twice() {
    let first = run("twice", 7);
    let second = run("twice", 7);
    assert_eq!(first.trace, second.trace);
    assert_eq!(first.publishes, second.publishes);
    assert_eq!(first.views, second.views);
    assert_eq!(first.digest, second.digest, "seed 7 ran apart:\n{}", first.trace);
    assert!(first.broken.is_empty(), "{:#?}", first.broken);
}

/// Every seed `seeds` names: no acknowledged publish lost, out of order, twice or apart; a failing
/// seed's trace kept, and the seed run again to the same digest. (Not in the planted bug's build,
/// which fails it by design.)
#[cfg(not(feature = "plant-own-log-commit"))]
#[test]
fn faults_over_many_seeds() {
    let mut failed = Vec::new();
    for seed in seeds() {
        let report = run("run", seed);
        let acked = report.publishes.iter().filter(|p| p.3.is_ok()).count();
        eprintln!(
            "seed {seed}: {} publishes, {acked} acknowledged, digest {:016x}, {}",
            report.publishes.len(),
            report.digest,
            if report.broken.is_empty() { "ok" } else { "BROKEN" }
        );
        if !report.broken.is_empty() {
            let path = keep_trace(&report);
            let again = run("run", seed);
            failed.push(format!(
                "seed {seed} ({} broken, trace {}; run again: digest {})",
                report.broken.len(),
                path.display(),
                if again.digest == report.digest { "the same" } else { "DIFFERENT" }
            ));
        } else {
            let _ = std::fs::remove_dir_all(root().join(format!("run-{seed}")));
        }
    }
    assert!(failed.is_empty(), "{failed:#?}");
}


/// The planted bug (AC-T2.2.5's test; `--features plant-own-log-commit`): every group commits what
/// its leader's own log holds, before a quorum's flush. The simulation finds a seed whose run loses
/// an acknowledged publish, and runs that seed again to the same digest and the same loss.
#[cfg(feature = "plant-own-log-commit")]
#[test]
fn the_planted_acknowledgement_before_a_quorums_flush_is_found_and_replayed() {
    let first = std::env::var("QUIVER_SIM_PLANT_FROM")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1_u64);
    for seed in first..first + 200 {
        let report = run("plant", seed);
        let lost: Vec<&String> = report
            .broken
            .iter()
            .filter(|b| b.starts_with("acknowledged"))
            .collect();
        eprintln!("seed {seed}: {} broken, {} of them acknowledged publishes lost", report.broken.len(), lost.len());
        if lost.is_empty() {
            let _ = std::fs::remove_dir_all(root().join(format!("plant-{seed}")));
            continue;
        }
        let path = keep_trace(&report);
        let again = run("plant", seed);
        assert_eq!(again.digest, report.digest, "seed {seed} ran apart on its replay");
        assert_eq!(again.broken, report.broken);
        eprintln!("found at seed {seed}, replayed alike (trace {}): {}", path.display(), lost[0]);
        return;
    }
    panic!("no seed from {first} found the planted bug");
}
