//! The persisted class under deterministic simulation (kiln's CP-T2.2 step 6, AC-T2.2.5;
//! `docs/design/persisted-sim.md`): three nodes, each host on a shard of hyper-rt's simulation
//! runtime, every member's log on hyper-sim's block device; publishes through every node, and
//! every acknowledged publish checked at its offset in every node's view.
#![cfg(feature = "sim")]
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::disallowed_macros,
    clippy::disallowed_methods,
    clippy::cast_possible_truncation
)]

use std::path::PathBuf;
use std::time::Duration;

use quiver::persisted::sim::{Answer, Kill, SimPath, Settings, World};
use quiver_api::{Class, Offset, Topic};

const TOPIC: &str = "sim";
const MS: u64 = 1_000_000;

/// A world of three nodes from `seed`, its run's directory made empty.
fn settings(name: &str, seed: u64) -> Settings {
    let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("sim");
    let dir = root.join(format!("{name}-{seed}"));
    let _ = std::fs::remove_dir_all(&dir);
    Settings {
        seed,
        nodes: 3,
        dir,
        identities: root.join("identities-3"),
        path: SimPath::in_order(200_000, 50_000),
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

/// Runs `world` in steps of a millisecond until `done`, at most `most_ns` of virtual time.
fn run_until_done(world: &mut World, most_ns: u64, mut done: impl FnMut(&mut World) -> bool) -> bool {
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

/// What a run saw: each publish's payload, and the offset it was acknowledged at or its refusal.
type Outcomes = Vec<(Vec<u8>, Result<Offset, String>)>;

/// Three nodes from `seed`, a topic, `count` publishes through each node in turn 5 ms apart; every
/// answer, and each node's view of the topic at the end.
fn run_quietly(seed: u64, count: usize) -> (Outcomes, Vec<Vec<(Offset, Vec<u8>)>>) {
    let mut world = World::new(settings("quiet", seed)).unwrap();
    for node in 1..=3 {
        world.start(node, Kill::Never).unwrap();
    }
    world.run_until(500 * MS);
    let mut created = world.create(1, TOPIC, topic());
    let mut answer = None;
    assert!(
        run_until_done(&mut world, 30_000 * MS, |_| {
            answer = answer.take().or_else(|| created.take());
            answer.is_some()
        }),
        "the topic was created"
    );
    assert!(matches!(answer, Some(Ok(()))), "{answer:?}");
    assert!(
        run_until_done(&mut world, 30_000 * MS, |w| (1..=3).all(|n| w.holds(n, TOPIC))),
        "every node holds the topic"
    );
    let mut sent: Vec<(Vec<u8>, Answer<Offset>)> = Vec::new();
    for at in 0..count {
        let node = (at % 3) as u64 + 1;
        let payload = format!("{seed}:{at}").into_bytes();
        let answer = world.publish(node, TOPIC, &payload);
        sent.push((payload, answer));
        let next = world.now_ns() + 5 * MS;
        world.run_until(next);
    }
    let mut outcomes: Vec<Option<Result<Offset, String>>> = vec![None; sent.len()];
    run_until_done(&mut world, 60_000 * MS, |_| {
        for ((_, answer), outcome) in sent.iter_mut().zip(outcomes.iter_mut()) {
            if outcome.is_none() {
                *outcome = answer.take().map(|a| a.map_err(|e| e.to_string()));
            }
        }
        outcomes.iter().all(Option::is_some)
    });
    let next = world.now_ns() + 2_000 * MS;
    world.run_until(next);
    let views = (1..=3)
        .map(|node| {
            world
                .read(node, TOPIC)
                .unwrap_or_default()
                .into_iter()
                .map(|m| (m.offset, m.payload))
                .collect()
        })
        .collect();
    world.stop();
    let outcomes = sent
        .into_iter()
        .zip(outcomes)
        .map(|((payload, _), outcome)| (payload, outcome.unwrap_or_else(|| Err("unanswered".to_owned()))))
        .collect();
    (outcomes, views)
}

/// With no fault, every publish through any node is acknowledged, at its own offset, and every
/// node's view holds each at that offset; and the run is the same run twice from its seed.
#[test]
fn without_faults_every_publish_is_acknowledged_and_kept_and_a_seed_runs_the_same() {
    let (outcomes, views) = run_quietly(1, 30);
    for (payload, outcome) in &outcomes {
        let offset = outcome.as_ref().unwrap_or_else(|e| panic!("{payload:?}: {e}"));
        for (node, view) in views.iter().enumerate() {
            assert!(
                view.iter().any(|(o, p)| o == offset && p == payload),
                "node {}'s view lacks {payload:?} at {offset}",
                node + 1
            );
        }
    }
    let again = run_quietly(1, 30);
    assert_eq!(again, (outcomes, views), "the same seed, the same run");
}
