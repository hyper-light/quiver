//! `quiver-drill`: one node of a persisted cluster, driven by lines on its standard input and
//! answering each on its standard output: the node the kill drills run as processes (kiln's
//! CP-T2.2, `tests/persisted_processes.rs`), each killed with SIGKILL at a persistence point and
//! started again on what its directory holds.
//!
//! `quiver-drill --id N --nodes 1=ADDRESS,2=ADDRESS,… --data DIR --identities DIR
//! [--identities-key FILE]`
//!
//! It says `ready RUN` once it has caught up with its catalog's group. Each line it reads is answered by one line:
//! `create NAME` by `ok` or `refused WHY`; `publish NAME PAYLOAD` by `acked OFFSET` or
//! `refused WHY`; `read NAME FROM` by `messages COUNT NEXT` and the messages, a line each
//! (`OFFSET PAYLOAD`); `figures NAME` by `figures LEADS TERM END`; `wire NAME` by `wire …`, what
//! the topic's endpoints hold and did since they began. Standard error carries
//! `frames N` each time the frames its logs wrote and flushed reach N: its persistence points;
//! `ended WHY` if its host ended; and, every 5 s while a publish waits past 5 s, `slow NAME: …`
//! with what the topic's publisher and member know, since the node takes no line meanwhile, and
//! the same report every 5 s while the topic's member holds a parked publish older than 5 s; and,
//! every 5 s while the topic's member lacks a connection to a peer or a publish waits past 5 s,
//! `wire NAME: …` with what its endpoints hold and their sockets did in those 5 s; and each time a
//! topic's member's term, leader or lead changes, `leader NAME: term T, leader L, leads B`.
//! It ends at the end of its input.

use std::future::Future;
use std::io::{BufRead, Write};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::pin::pin;
use std::process::ExitCode;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll, Wake, Waker};
use std::time::Duration;

use quiver::{GroupFigures, Node, NodeConfig, PersistedConfig, PersistedStats, WireFigures};
use quiver_api::{Class, Topic, Topics};

/// How often the frames its logs wrote are looked at: a persistence point is told within this.
const WATCH: Duration = Duration::from_millis(1);
/// How long a publish waits before the publisher's figures are told on standard error, and how
/// often again while it waits.
const SLOW: Duration = Duration::from_secs(5);

struct Unpark(std::thread::Thread);

impl Wake for Unpark {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

/// Runs `future` on this thread to its end.
fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let waker = Waker::from(Arc::new(Unpark(std::thread::current())));
    let mut context = Context::from_waker(&waker);
    loop {
        if let Poll::Ready(out) = future.as_mut().poll(&mut context) {
            return out;
        }
        std::thread::park();
    }
}

fn config(args: &[String]) -> Result<PersistedConfig, String> {
    let mut id = None;
    let mut nodes = Vec::new();
    let mut data = None;
    let mut identities = None;
    let mut identities_key = None;
    let mut at = args.iter();
    while let Some(flag) = at.next() {
        let value = at.next().ok_or_else(|| format!("{flag} takes a value"))?;
        match flag.as_str() {
            "--id" => id = Some(value.parse::<u64>().map_err(|e| format!("--id: {e}"))?),
            "--nodes" => {
                for node in value.split(',') {
                    let (n, address) = node.split_once('=').ok_or("--nodes: ID=ADDRESS,…")?;
                    let n = n.parse::<u64>().map_err(|e| format!("--nodes: {e}"))?;
                    let address = address
                        .parse::<SocketAddr>()
                        .map_err(|e| format!("--nodes: {e}"))?;
                    nodes.push((n, address));
                }
            }
            "--data" => data = Some(PathBuf::from(value)),
            "--identities" => identities = Some(PathBuf::from(value)),
            "--identities-key" => identities_key = Some(PathBuf::from(value)),
            other => return Err(format!("unknown argument {other}")),
        }
    }
    Ok(PersistedConfig {
        id: id.ok_or("--id is needed")?,
        nodes,
        data: data.ok_or("--data is needed")?,
        identities: identities.ok_or("--identities is needed")?,
        identities_key,
        part_memory: PersistedConfig::PART_MEMORY_MIN,
        topics: 16,
        publishes: 4096,
        quorum_wait: Duration::from_secs(10),
    })
}

/// The answer to one line.
fn answer(node: &Node, line: &str) -> String {
    let words: Vec<&str> = line.split_whitespace().collect();
    // One line whatever the refusal says: every answer but a read's is a line.
    let refused =
        |e: quiver_api::Refusal| format!("refused {}", e.to_string().replace(['\n', '\r'], " "));
    match words.as_slice() {
        ["create", name] => {
            let topic = Topic {
                class: Class::Persisted,
                retain_messages: 100_000,
                retain_bytes: 1 << 26,
            };
            block_on(node.create(name, topic)).map_or_else(refused, |()| "ok".to_owned())
        }
        ["publish", name, payload] => block_on(node.publish(name, None, payload.as_bytes()))
            .map_or_else(refused, |offset| format!("acked {offset}")),
        ["read", name, from] => {
            let Ok(from) = from.parse::<u64>() else {
                return "refused FROM is an offset".to_owned();
            };
            match block_on(node.read(name, from, quiver_api::READ_MAX)) {
                Ok(batch) => {
                    let mut out = format!("messages {} {}", batch.messages.len(), batch.next);
                    for m in batch.messages {
                        out.push_str(&format!(
                            "\n{} {}",
                            m.offset,
                            String::from_utf8_lossy(&m.payload)
                        ));
                    }
                    out
                }
                Err(e) => refused(e),
            }
        }
        ["figures", name] => {
            let stats = node.persisted_stats().unwrap_or_default();
            let topic = stats.topic(name).copied().unwrap_or_default();
            let end = block_on(node.bounds(name)).map_or(0, |(_, end)| end);
            format!("figures {} {} {end}", u8::from(topic.leads), topic.term)
        }
        ["wire", name] => {
            let stats = node.persisted_stats().unwrap_or_default();
            let topic = stats.topic(name).copied().unwrap_or_default();
            format!("wire {}", wires(&topic, &GroupFigures::default()))
        }
        _ => format!("refused unknown command {line:?}"),
    }
}

/// A topic's member's term, the leader it knows, and whether it leads.
type Led = (u64, Option<u64>, bool);

/// The ids a mask of [`WireFigures`] holds, in order.
fn ids(mask: u64) -> Vec<u64> {
    (0..64).filter(|id| mask & (1 << id) != 0).collect()
}

/// What one endpoint holds now, and what it did since `was`.
fn wire(now: &WireFigures, was: &WireFigures) -> String {
    let grew = |now: u64, then: u64| now.saturating_sub(then);
    format!(
        "up {:?} of {:?}, redials {}, connections {}, pending {}; admitted +{}, replaced +{}, busy +{}, out +{}, in +{} listener +{} peers, send errors +{}, astray +{}",
        ids(now.up),
        ids(now.peers),
        now.redials,
        now.connections,
        now.pending,
        grew(now.admitted, was.admitted),
        grew(now.replaced, was.replaced),
        grew(now.refused_busy, was.refused_busy),
        grew(now.out, was.out),
        grew(now.listener_in, was.listener_in),
        grew(now.peers_in, was.peers_in),
        grew(now.send_errors, was.send_errors),
        grew(now.astray, was.astray),
    )
}

/// What a topic's member's endpoint and its publisher's hold now, and what they did since `was`.
fn wires(now: &GroupFigures, was: &GroupFigures) -> String {
    format!(
        "member {}; publisher {}",
        wire(&now.member_wire, &was.member_wire),
        wire(&now.publisher_wire, &was.publisher_wire)
    )
}

fn run(args: &[String]) -> Result<(), String> {
    let config = config(args)?;
    let node = Node::with_persisted(
        NodeConfig {
            memory: 256 << 20,
            topics: 16,
            subscriptions: 16,
        },
        config,
    )
    .map_err(|e| e.to_string())?;
    // Ready once it caught up with its catalog's group: until then it refuses what it can't tell.
    while !node
        .persisted_stats()
        .is_some_and(|s| s.settled || s.ended.is_some())
    {
        std::thread::park_timeout(WATCH);
    }
    let run = node.persisted_stats().map_or(0, |s| s.run);
    let mut out = std::io::stdout().lock();
    writeln!(out, "ready {run}")
        .and_then(|()| out.flush())
        .map_err(|e| e.to_string())?;
    let done = Arc::new(AtomicBool::new(false));
    let watcher = std::thread::spawn({
        let node = node.clone();
        let done = Arc::clone(&done);
        move || {
            let mut told = 0;
            let mut ended = false;
            // Each topic whose publisher owes or holds a publish, since when, and when it was last
            // told: a publish that waits past SLOW is told every SLOW, with what the publisher
            // knows, since the node takes no line while one waits (kiln's CP-T2.2 step 7).
            let mut waiting: Vec<(String, std::time::Instant, std::time::Instant)> = Vec::new();
            // Each topic whose member's oldest parked publish waits past SLOW, and when it was last
            // told.
            let mut parked_told: Vec<(String, std::time::Instant)> = Vec::new();
            let slow_ns = u64::try_from(SLOW.as_nanos()).unwrap_or(u64::MAX);
            // Each topic's figures as they were SLOW ago: a member that answered publishes with
            // the leader, or parked them, since then is told, with what else it did meanwhile.
            let mut sampled: Option<(std::time::Instant, PersistedStats)> = None;
            // Each topic's member's term, leader and lead as last told.
            let mut led: Vec<(String, Led)> = Vec::new();
            while !done.load(Ordering::Relaxed) {
                let stats = node.persisted_stats().unwrap_or_default();
                if let (Some(why), false) = (&stats.ended, ended) {
                    ended = true;
                    let _ = writeln!(std::io::stderr().lock(), "ended {why}");
                }
                for (name, g) in &stats.topics {
                    let state = (g.term, g.leader, g.leads);
                    match led.iter_mut().find(|(n, _)| n == name) {
                        Some((_, was)) if *was == state => continue,
                        Some((_, was)) => *was = state,
                        None => led.push((name.clone(), state)),
                    }
                    let _ = writeln!(
                        std::io::stderr().lock(),
                        "leader {name}: term {}, leader {:?}, leads {}",
                        g.term,
                        g.leader,
                        g.leads
                    );
                }
                let now = std::time::Instant::now();
                waiting.retain(|(name, _, _)| {
                    stats
                        .topic(name)
                        .is_some_and(|g| g.owed > 0 || g.queued > 0)
                });
                if sampled
                    .as_ref()
                    .is_none_or(|(at, _)| now.saturating_duration_since(*at) >= SLOW)
                {
                    if let Some((_, before)) = &sampled {
                        for (name, g) in &stats.topics {
                            let Some(was) = before.topic(name) else {
                                continue;
                            };
                            let grew = |now: u64, then: u64| now.saturating_sub(then);
                            let short = g.member_wire.up & g.member_wire.peers != g.member_wire.peers;
                            let slow = waiting.iter().any(|(n, since, _)| {
                                n == name && now.saturating_duration_since(*since) >= SLOW
                            });
                            if short || slow {
                                let _ = writeln!(
                                    std::io::stderr().lock(),
                                    "wire {name}: in {:.0} s {}",
                                    SLOW.as_secs_f64(),
                                    wires(g, was)
                                );
                            }
                            let (redirected, parked) = (
                                grew(g.redirected, was.redirected),
                                grew(g.parked, was.parked),
                            );
                            if redirected == 0 && parked == 0 {
                                continue;
                            }
                            let _ = writeln!(
                                std::io::stderr().lock(),
                                "member {name}: leads {}, leader {:?} in term {}; in {:.0} s proposed +{}, parked +{parked}, redirected +{redirected}, refused drives +{}, relieved +{}, answers waited +{}, terms +{}; applied {}",
                                g.leads,
                                g.leader,
                                g.term,
                                SLOW.as_secs_f64(),
                                grew(g.proposed, was.proposed),
                                grew(g.refused_drives, was.refused_drives),
                                grew(g.relieved, was.relieved),
                                grew(g.answers_waited, was.answers_waited),
                                grew(g.terms, was.terms),
                                g.applied
                            );
                        }
                    }
                    sampled = Some((now, stats.clone()));
                }
                for (name, g) in &stats.topics {
                    if g.owed == 0 && g.queued == 0 {
                        continue;
                    }
                    let Some(at) = waiting.iter_mut().find(|(n, _, _)| n == name) else {
                        waiting.push((name.clone(), now, now));
                        continue;
                    };
                    if now.saturating_duration_since(at.1) >= SLOW
                        && now.saturating_duration_since(at.2) >= SLOW
                    {
                        at.2 = now;
                        let waited = now.saturating_duration_since(at.1);
                        let _ = writeln!(
                            std::io::stderr().lock(),
                            "{}",
                            g.slow_report(name, Some(waited))                        );
                    }
                }
                // A member whose oldest parked publish waits past SLOW: its report, every SLOW, so
                // the leader's side of a stall is told as the publisher's is.
                parked_told.retain(|(name, _)| {
                    stats
                        .topic(name)
                        .is_some_and(|g| g.parked_oldest_ns >= slow_ns)
                });
                for (name, g) in &stats.topics {
                    if g.parked_oldest_ns < slow_ns {
                        continue;
                    }
                    match parked_told.iter_mut().find(|(n, _)| n == name) {
                        Some((_, told)) if now.saturating_duration_since(*told) < SLOW => {}
                        Some((_, told)) => {
                            *told = now;
                            let _ = writeln!(std::io::stderr().lock(), "{}", g.slow_report(name, None));
                        }
                        None => {
                            parked_told.push((name.clone(), now));
                            let _ = writeln!(std::io::stderr().lock(), "{}", g.slow_report(name, None));
                        }
                    }
                }
                let frames = stats.frames();
                if frames != told {
                    told = frames;
                    let mut err = std::io::stderr().lock();
                    if writeln!(err, "frames {frames}").is_err() {
                        return;
                    }
                }
                std::thread::park_timeout(WATCH);
            }
        }
    });
    for line in std::io::stdin().lock().lines() {
        let line = line.map_err(|e| e.to_string())?;
        let reply = answer(&node, &line);
        writeln!(out, "{reply}")
            .and_then(|()| out.flush())
            .map_err(|e| e.to_string())?;
    }
    done.store(true, Ordering::Relaxed);
    watcher.thread().unpark();
    let _ = watcher.join();
    node.halt_persisted();
    Ok(())
}

fn main() -> ExitCode {
    let args: Result<Vec<String>, _> = std::env::args_os()
        .skip(1)
        .map(|a| a.into_string())
        .collect();
    let outcome = match args {
        Ok(args) => run(&args),
        Err(_) => Err("an argument isn't Unicode".to_owned()),
    };
    match outcome {
        Ok(()) => ExitCode::SUCCESS,
        Err(why) => {
            let _ = writeln!(std::io::stderr().lock(), "quiver-drill: {why}");
            ExitCode::from(2)
        }
    }
}
