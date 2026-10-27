//! `quiver-lab`: kiln's CP-T2.2 step 7 (AC-T2.2.4, AC-T2.2.5), the persisted class's lab on three
//! machines: a `quiver-drill` node on each, killed -9 at random persistence points (TerminateProcess
//! on Windows) and started again on what its directory holds, every acknowledged publish read back
//! from every node after each drill. A lab's program, never a product's.
//!
//! `quiver-lab host --listen ADDRESS --dir DIR --token-file PATH [--drill PATH]`
//!
//! One machine's part. Every connection says the lab's token first (`auth TOKEN`, the token the
//! file holds, which the driver is given too); one that doesn't is closed. It starts its node when told, passes the driver's lines to it and its answers
//! back, and kills it, at once or at its K-th persistence point from now. Each connection is
//! answered on a thread of its own, its lines in order, so a publish can wait on one while the kill
//! comes on another. Lines: `ping`; `put PATH HEX` (a file under DIR); `wipe` (the node killed, its
//! directory gone); `spawn ID NODES` and `ready`; `ask LINE` (a line for the node, answered as it
//! answers, or `ended` if it died first, `down` if none runs); `kill`; `arm K`; `status`.
//!
//! `quiver-lab host … --stall-request PATH --stall-done PATH`: the host of the node whose disk is
//! stalled. `stall` writes the request a root helper waits on (it stalls the disk's device 2 s
//! with dm-delay) and answers once the helper says it's done.
//!
//! `quiver-lab drive --hosts 1=ADDRESS,2=ADDRESS,3=ADDRESS --nodes 1=ADDRESS,2=ADDRESS,3=ADDRESS
//! --token-file PATH --drills N [--first F] [--seed S] [--fresh-every M] [--out FILE] [--work DIR]`
//!
//! The drills. Each kills one node, or two in turn, while a run of publishes goes through another:
//! the leader (publishing through a follower), a follower, the leader with the publisher's
//! connection to it, and a follower then the leader. The victim is killed at a random persistence
//! point of the run, or at its end if the run ends first. Once it's back, a publish goes through
//! every node; once every node holds every acknowledged publish, each node's log is read back
//! whole and held to what the publishers heard (every acknowledged publish at its offset, nothing
//! twice, the three logs alike), and the topic group's term is read for the elections the drill
//! cost. A line of JSON per drill goes to `--out`; the first drill that fails stops the lab.
//!
//! `quiver-lab stall --hosts … --nodes … --stalled ID [--stalls K] [--every-ms R] [--seconds D]
//! [--at-ms T] [--out FILE] [--work DIR]`
//!
//! The stall drills: node ID made the leader (its leaders killed until it leads), then K times a
//! publish every R ms through a follower for D seconds, and T ms in, node ID's disk stalled; each
//! held to no election (the topic group's term unchanged, ID still leading), what was published
//! during the stall acknowledged, and every acknowledged publish read back.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::io::{BufRead, BufReader, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStdin, ChildStdout, Command, ExitCode, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime};

/// The drills' topic.
const TOPIC: &str = "drill";
/// Publishes in a drill's run.
const RUN: u64 = 16;
/// The latest persistence point a victim is killed at, counted from the run's start.
const POINTS: u64 = 40;
/// How long anything the lab waits for may take before the drill fails.
const STUCK: Duration = Duration::from_secs(120);
/// How often a waiting driver or host looks again.
const POLL: Duration = Duration::from_millis(2);
/// What a failure of the lab's own connections starts with: it stops the lab, never taken for a
/// node's refusal.
const LAB: &str = "lab: ";

type Fault = String;

fn out(line: &str) {
    let _ = writeln!(std::io::stdout().lock(), "{line}");
}

fn err(line: &str) {
    let _ = writeln!(std::io::stderr().lock(), "{line}");
}

/// Waits `d`, or less should the thread be woken: callers look again either way.
fn pause(d: Duration) {
    std::thread::park_timeout(d);
}

fn flags(args: &[String]) -> Result<BTreeMap<String, String>, Fault> {
    let mut found = BTreeMap::new();
    let mut at = args.iter();
    while let Some(flag) = at.next() {
        let name = flag
            .strip_prefix("--")
            .ok_or_else(|| format!("{flag}: a flag was expected"))?;
        let value = at.next().ok_or_else(|| format!("{flag} takes a value"))?;
        found.insert(name.to_owned(), value.clone());
    }
    Ok(found)
}

/// The lab's token, from the file `--token-file` names: what each connection to a host says first.
fn token(flags: &BTreeMap<String, String>) -> Result<String, Fault> {
    let path = flags.get("token-file").ok_or("--token-file is needed")?;
    let token = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let token = token.trim().to_owned();
    if token.len() < 32 {
        return Err(format!("{path}: a token of 32 characters at least"));
    }
    Ok(token)
}

/// Whether `said` is `token`, every byte looked at whatever the first that differs.
fn same(said: &str, token: &str) -> bool {
    said.len() == token.len()
        && said
            .bytes()
            .zip(token.bytes())
            .fold(0u8, |differ, (a, b)| differ | (a ^ b))
            == 0
}

fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len().saturating_mul(2));
    for b in bytes {
        let _ = write!(s, "{b:02x}");
    }
    s
}

fn unhex(s: &str) -> Result<Vec<u8>, Fault> {
    let digits: Vec<u8> = s
        .bytes()
        .map(|c| match c {
            b'0'..=b'9' => Ok(c.wrapping_sub(b'0')),
            b'a'..=b'f' => Ok(c.wrapping_sub(b'a').wrapping_add(10)),
            _ => Err("not hex".to_owned()),
        })
        .collect::<Result<_, _>>()?;
    if !digits.len().is_multiple_of(2) {
        return Err("odd hex".into());
    }
    Ok(digits
        .chunks(2)
        .map(|pair| match pair {
            [hi, lo] => hi.wrapping_shl(4) | lo,
            _ => 0,
        })
        .collect())
}

// The host.

/// One start of the node: what its standard error says of its persistence points, and its kill.
struct Run {
    frames: Arc<AtomicU64>,
    /// The frame count the armed kill fires at; 0: none armed.
    arm: Arc<AtomicU64>,
    /// The frames when the armed kill fired; `u64::MAX` until it does.
    fired: Arc<AtomicU64>,
}

/// The node's process and its input and output.
struct Talk {
    input: ChildStdin,
    output: BufReader<ChildStdout>,
}

impl Talk {
    /// The node's next line; `None` once it ended.
    fn line(&mut self) -> Option<String> {
        let mut line = String::new();
        match self.output.read_line(&mut line) {
            Ok(0) | Err(_) => None,
            Ok(_) => Some(line.trim_end().to_owned()),
        }
    }
}

struct Host {
    dir: PathBuf,
    drill: PathBuf,
    /// The request a root helper waits on to stall this node's disk, and what it writes once done.
    stall: Option<(PathBuf, PathBuf)>,
    /// Where each line passed to the node and its answer's first line go, with QUIVER_LAB_TRACE set.
    trace: Option<Mutex<std::fs::File>>,
    child: Arc<Mutex<Option<Child>>>,
    talk: Mutex<Option<Talk>>,
    run: Mutex<Run>,
    log: Arc<Mutex<std::fs::File>>,
}

fn kill_now(child: &Mutex<Option<Child>>) -> bool {
    let Ok(mut held) = child.lock() else {
        return false;
    };
    match held.take() {
        Some(mut process) => {
            let _ = process.kill();
            let _ = process.wait();
            true
        }
        None => false,
    }
}

impl Host {
    fn new_run() -> Run {
        Run {
            frames: Arc::new(AtomicU64::new(0)),
            arm: Arc::new(AtomicU64::new(0)),
            fired: Arc::new(AtomicU64::new(u64::MAX)),
        }
    }

    fn frames(&self) -> u64 {
        self.run
            .lock()
            .map_or(0, |run| run.frames.load(Ordering::Acquire))
    }

    fn up(&self) -> bool {
        self.child.lock().is_ok_and(|held| held.is_some())
    }

    /// How the node's process ended by itself, if it did: no kill of the lab's ended it.
    fn exited(&self) -> Option<String> {
        let mut held = self.child.lock().ok()?;
        match held.as_mut().map(Child::try_wait) {
            Some(Ok(Some(status))) => Some(status.to_string().replace(' ', "_")),
            _ => None,
        }
    }

    fn spawn(&self, id: &str, nodes: &str) -> Result<(), Fault> {
        let mut held = self.child.lock().map_err(|_| "poisoned".to_owned())?;
        if held.is_some() {
            return Err("the node runs".into());
        }
        let mut child = Command::new(&self.drill)
            .args(["--id", id, "--nodes", nodes])
            .arg("--data")
            .arg(self.dir.join("node"))
            .arg("--identities")
            .arg(self.dir.join("identities"))
            .arg("--identities-key")
            .arg(self.dir.join("identities.key"))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("{}: {e}", self.drill.display()))?;
        let (Some(input), Some(output), Some(errors)) =
            (child.stdin.take(), child.stdout.take(), child.stderr.take())
        else {
            let _ = child.kill();
            return Err("the node's pipes".into());
        };
        let run = Self::new_run();
        let (frames, arm, fired) = (
            Arc::clone(&run.frames),
            Arc::clone(&run.arm),
            Arc::clone(&run.fired),
        );
        let (child_handle, log) = (Arc::clone(&self.child), Arc::clone(&self.log));
        let began = Instant::now();
        // The node's persistence points, told on its standard error, and the armed kill fired on
        // the one it waits for: at once, from here.
        std::thread::spawn(move || {
            for line in BufReader::new(errors).lines() {
                let Ok(line) = line else { return };
                match line.strip_prefix("frames ").map(str::parse::<u64>) {
                    Some(Ok(n)) => {
                        frames.store(n, Ordering::Release);
                        let at = arm.load(Ordering::Acquire);
                        if at != 0 && n >= at && fired.load(Ordering::Acquire) == u64::MAX {
                            fired.store(n, Ordering::Release);
                            kill_now(&child_handle);
                        }
                    }
                    _ => {
                        if let Ok(mut file) = log.lock() {
                            let _ = writeln!(file, "{:>9.3} {line}", began.elapsed().as_secs_f64());
                        }
                    }
                }
            }
        });
        *held = Some(child);
        drop(held);
        *self.talk.lock().map_err(|_| "poisoned".to_owned())? = Some(Talk {
            input,
            output: BufReader::new(output),
        });
        *self.run.lock().map_err(|_| "poisoned".to_owned())? = run;
        Ok(())
    }

    /// Writes `data` (hex, `-` for nothing: a trailing space would be lost) to `path` under the
    /// host's directory, owner-only as Focal makes every key file, in directories of the same: a
    /// root key file any other may read is refused.
    fn put(&self, path: &str, data: &str) -> Result<(), Fault> {
        let target = self.dir.join(path);
        if let Some(parent) = target.parent() {
            focal_platform::fs::create_dir_all_durable(
                parent,
                focal_platform::fs::create_dir_private,
            )
            .map_err(|e| format!("{}: {e}", parent.display()))?;
        }
        let bytes = if data == "-" {
            Vec::new()
        } else {
            unhex(data)?
        };
        match std::fs::remove_file(&target) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(format!("{}: {e}", target.display())),
        }
        let mut file = focal_platform::fs::create_private_new(&target, false, true)
            .map_err(|e| format!("{}: {e}", target.display()))?;
        file.write_all(&bytes)
            .and_then(|()| file.sync_all())
            .map_err(|e| format!("{}: {e}", target.display()))
    }

    /// The answer to one of the driver's lines, a line or more.
    fn answer(&self, line: &str) -> String {
        let (verb, rest) = line.split_once(' ').unwrap_or((line, ""));
        match verb {
            "ping" => format!("pong {}", std::env::consts::OS),
            "put" => match rest.split_once(' ') {
                // Only a path below the host's directory: never `..`, a root or a prefix, which
                // `Path::join` would take in its place.
                Some((path, data))
                    if Path::new(path)
                        .components()
                        .all(|c| matches!(c, std::path::Component::Normal(_))) =>
                {
                    self.put(path, data)
                        .map_or_else(|e| format!("error {e}"), |()| "ok".to_owned())
                }
                _ => "error put PATH HEX".into(),
            },
            "wipe" => {
                kill_now(&self.child);
                match std::fs::remove_dir_all(self.dir.join("node")) {
                    Ok(()) => "ok".into(),
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => "ok".into(),
                    Err(e) => format!("error {e}"),
                }
            }
            "spawn" => match rest.split_once(' ') {
                Some((id, nodes)) => self
                    .spawn(id, nodes)
                    .map_or_else(|e| format!("error {e}"), |()| "spawned".to_owned()),
                None => "error spawn ID NODES".into(),
            },
            "ready" => match self.talk.lock() {
                Ok(mut talk) => match talk.as_mut().map(Talk::line) {
                    Some(Some(ready)) => ready,
                    Some(None) => "ended".into(),
                    None => "down".into(),
                },
                Err(_) => "error poisoned".into(),
            },
            "ask" => self.ask(rest),
            "stall" => self
                .stall()
                .map_or_else(|e| format!("error {e}"), |ms| format!("stalled {ms}")),
            "kill" => {
                let frames = self.frames();
                if kill_now(&self.child) {
                    format!("killed {frames}")
                } else {
                    "down".into()
                }
            }
            "arm" => match (rest.parse::<u64>(), self.run.lock()) {
                (Ok(k), Ok(run)) if self.up() => {
                    let base = run.frames.load(Ordering::Acquire);
                    run.fired.store(u64::MAX, Ordering::Release);
                    run.arm.store(base.saturating_add(k), Ordering::Release);
                    format!("armed {base}")
                }
                (Ok(_), Ok(_)) => "down".into(),
                _ => "error arm K".into(),
            },
            "status" => match self.run.lock() {
                Ok(_) if self.exited().is_some() => {
                    format!("exited {}", self.exited().unwrap_or_default())
                }
                Ok(run) => {
                    let fired = run.fired.load(Ordering::Acquire);
                    format!(
                        "{} {} {}",
                        if self.up() { "up" } else { "down" },
                        run.frames.load(Ordering::Acquire),
                        if fired == u64::MAX {
                            "-".to_owned()
                        } else {
                            fired.to_string()
                        }
                    )
                }
                Err(_) => "error poisoned".into(),
            },
            _ => format!("error unknown {verb:?}"),
        }
    }

    /// Asks the root helper for a stall of the disk and waits for its end: the milliseconds from
    /// the request to the helper's word.
    fn stall(&self) -> Result<u128, Fault> {
        let (request, done) = self
            .stall
            .as_ref()
            .ok_or("no stall: --stall-request and --stall-done")?;
        match std::fs::remove_file(done) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(format!("{}: {e}", done.display())),
        }
        let began = Instant::now();
        std::fs::write(request, b"stall").map_err(|e| format!("{}: {e}", request.display()))?;
        while !done.exists() {
            if began.elapsed() > Duration::from_secs(30) {
                return Err("the root helper didn't answer within 30 s".into());
            }
            pause(Duration::from_millis(5));
        }
        Ok(began.elapsed().as_millis())
    }

    fn ask(&self, line: &str) -> String {
        let answer = self.ask_node(line);
        if let Some(trace) = &self.trace
            && let Ok(mut file) = trace.lock()
        {
            let first = answer.lines().next().unwrap_or_default();
            let _ = writeln!(
                file,
                "{:?} {line} => {first}",
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
            );
        }
        answer
    }

    fn ask_node(&self, line: &str) -> String {
        let Ok(mut held) = self.talk.lock() else {
            return "error poisoned".into();
        };
        let Some(talk) = held.as_mut() else {
            return "down".into();
        };
        if writeln!(talk.input, "{line}")
            .and_then(|()| talk.input.flush())
            .is_err()
        {
            return "ended".into();
        }
        let Some(first) = talk.line() else {
            return "ended".into();
        };
        // A read's answer is its head and a line for each message.
        let count = first
            .strip_prefix("messages ")
            .and_then(|r| r.split(' ').next())
            .and_then(|c| c.parse::<u64>().ok())
            .unwrap_or(0);
        let mut answer = first;
        for _ in 0..count {
            let Some(next) = talk.line() else {
                return "ended".into();
            };
            answer.push('\n');
            answer.push_str(&next);
        }
        answer
    }
}

fn host(flags: &BTreeMap<String, String>) -> Result<(), Fault> {
    let listen = flags.get("listen").ok_or("--listen is needed")?;
    let dir = PathBuf::from(flags.get("dir").ok_or("--dir is needed")?);
    let drill = match flags.get("drill") {
        Some(path) => PathBuf::from(path),
        None => std::env::current_exe()
            .map_err(|e| e.to_string())?
            .with_file_name(format!("quiver-drill{}", std::env::consts::EXE_SUFFIX)),
    };
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let log = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join("node.log"))
        .map_err(|e| e.to_string())?;
    let stall = match (flags.get("stall-request"), flags.get("stall-done")) {
        (Some(request), Some(done)) => Some((PathBuf::from(request), PathBuf::from(done))),
        (None, None) => None,
        _ => return Err("--stall-request and --stall-done go together".into()),
    };
    let trace = match std::env::var_os("QUIVER_LAB_TRACE") {
        Some(_) => Some(Mutex::new(
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(dir.join("trace.log"))
                .map_err(|e| e.to_string())?,
        )),
        None => None,
    };
    let host = Arc::new(Host {
        dir,
        drill,
        stall,
        trace,
        child: Arc::new(Mutex::new(None)),
        talk: Mutex::new(None),
        run: Mutex::new(Host::new_run()),
        log: Arc::new(Mutex::new(log)),
    });
    let token = Arc::new(token(flags)?);
    let listener = TcpListener::bind(listen).map_err(|e| format!("{listen}: {e}"))?;
    out(&format!("listening {listen}"));
    for stream in listener.incoming() {
        let Ok(stream) = stream else { continue };
        let (host, token) = (Arc::clone(&host), Arc::clone(&token));
        std::thread::spawn(move || {
            let _ = stream.set_nodelay(true);
            let Ok(write) = stream.try_clone() else {
                return;
            };
            let mut write = std::io::BufWriter::new(write);
            let mut lines = BufReader::new(stream).lines();
            // The lab's token first, or nothing.
            let said = lines.next().and_then(Result::ok).unwrap_or_default();
            let authorized = said
                .strip_prefix("auth ")
                .is_some_and(|t| same(t.trim_end(), &token));
            let first = if authorized { "ok" } else { "error auth" };
            if writeln!(write, "{first}")
                .and_then(|()| write.flush())
                .is_err()
                || !authorized
            {
                return;
            }
            for line in lines {
                let Ok(line) = line else { return };
                let answer = host.answer(line.trim_end());
                if writeln!(write, "{answer}")
                    .and_then(|()| write.flush())
                    .is_err()
                {
                    return;
                }
            }
        });
    }
    Ok(())
}

// The driver.

/// A connection to a host: one line out, its answer in.
struct Conn {
    write: TcpStream,
    read: BufReader<TcpStream>,
}

impl Conn {
    fn open(address: &str, token: &str) -> Result<Self, Fault> {
        let stream = TcpStream::connect(address).map_err(|e| format!("{address}: {e}"))?;
        let _ = stream.set_nodelay(true);
        stream
            .set_read_timeout(Some(STUCK))
            .map_err(|e| e.to_string())?;
        let read = BufReader::new(stream.try_clone().map_err(|e| e.to_string())?);
        let mut conn = Self {
            write: stream,
            read,
        };
        match conn.ask(&format!("auth {token}"))?.as_str() {
            "ok" => Ok(conn),
            other => Err(format!("{address}: {other}")),
        }
    }

    fn line(&mut self) -> Result<String, Fault> {
        let mut line = String::new();
        // An error here leaves the connection's lines unknown: the lab's own failure (`LAB`), never
        // taken for the node's answer.
        match self.read.read_line(&mut line) {
            Ok(0) => Err(format!("{LAB}the host closed its connection")),
            Ok(_) => Ok(line.trim_end().to_owned()),
            Err(e) => Err(format!("{LAB}the host's answer: {e}")),
        }
    }

    /// The host's answer to `line`: a line, or a read's head and a line for each of its messages.
    fn ask(&mut self, line: &str) -> Result<String, Fault> {
        writeln!(self.write, "{line}")
            .and_then(|()| self.write.flush())
            .map_err(|e| format!("{LAB}to the host: {e}"))?;
        let mut answer = self.line()?;
        let count = answer
            .strip_prefix("messages ")
            .and_then(|r| r.split(' ').next())
            .and_then(|c| c.parse::<u64>().ok())
            .unwrap_or(0);
        for _ in 0..count {
            let next = self.line()?;
            answer.push('\n');
            answer.push_str(&next);
        }
        Ok(answer)
    }

    /// The node's answer to `line`: what the host passed back, or why there's none.
    fn node(&mut self, line: &str) -> Result<String, Fault> {
        match self.ask(&format!("ask {line}"))?.as_str() {
            "ended" => Err("the node's process ended".into()),
            "down" => Err("the node is down".into()),
            answer => Ok(answer.to_owned()),
        }
    }

    /// (leads, term, end) of the topic at this node.
    fn figures(&mut self) -> Result<(bool, u64, u64), Fault> {
        let answer = self.node(&format!("figures {TOPIC}"))?;
        let mut words = answer.split(' ');
        match (words.next(), words.next(), words.next(), words.next()) {
            (Some("figures"), Some(leads), Some(term), Some(end)) => Ok((
                leads == "1",
                term.parse().map_err(|_| answer.clone())?,
                end.parse().map_err(|_| answer.clone())?,
            )),
            _ => Err(format!("figures: {answer}")),
        }
    }

    fn publish(&mut self, payload: &str) -> Result<u64, Fault> {
        let answer = self.node(&format!("publish {TOPIC} {payload}"))?;
        answer
            .strip_prefix("acked ")
            .and_then(|o| o.parse().ok())
            .ok_or(answer)
    }

    /// The topic's messages at this node, from offset 0: (offset, payload).
    fn log(&mut self) -> Result<Vec<(u64, String)>, Fault> {
        let mut log = Vec::new();
        let mut from = 0u64;
        loop {
            let answer = self.node(&format!("read {TOPIC} {from}"))?;
            let mut lines = answer.lines();
            let head = lines.next().unwrap_or_default();
            let mut words = head.split(' ');
            let (Some("messages"), Some(count), Some(next)) =
                (words.next(), words.next(), words.next())
            else {
                return Err(format!("read: {head}"));
            };
            if count == "0" {
                return Ok(log);
            }
            for message in lines {
                let (offset, payload) = message
                    .split_once(' ')
                    .ok_or_else(|| format!("read: {message}"))?;
                log.push((
                    offset.parse().map_err(|_| message.to_owned())?,
                    payload.to_owned(),
                ));
            }
            from = next.parse().map_err(|_| head.to_owned())?;
        }
    }
}

/// What the publishers heard: each payload acknowledged, at its offset; each refused or lost with
/// its node, which may or may not have been appended (never twice).
#[derive(Default)]
struct Heard {
    acked: BTreeMap<String, u64>,
    unknown: BTreeSet<String>,
}

/// The kill drill's check (Quiver's `tests/cluster/drill.rs`), once the cluster is whole again:
/// every acknowledged publish in every node's log at the offset it was acknowledged at, no payload
/// twice, the three logs alike.
fn check(heard: &Heard, logs: &[(u64, Vec<(u64, String)>)]) -> Result<(), Fault> {
    let Some((first_node, first)) = logs.first() else {
        return Err("no logs".into());
    };
    for (node, log) in logs {
        if log != first {
            return Err(format!(
                "node {node}'s log differs from node {first_node}'s: {} against {} messages",
                log.len(),
                first.len()
            ));
        }
        let mut seen = BTreeSet::new();
        for (at, (offset, payload)) in (0u64..).zip(log) {
            if *offset != at {
                return Err(format!("node {node}: offset {offset} at place {at}"));
            }
            if !seen.insert(payload.as_str()) {
                return Err(format!("node {node}: {payload:?} appended twice"));
            }
            if !heard.acked.contains_key(payload) && !heard.unknown.contains(payload) {
                return Err(format!("node {node}: {payload:?} was never published"));
            }
        }
        for (payload, offset) in &heard.acked {
            let kept = usize::try_from(*offset).ok().and_then(|o| log.get(o));
            match kept {
                Some((_, kept)) if kept == payload => {}
                other => {
                    return Err(format!(
                        "node {node}: {payload:?}, acknowledged at {offset}, lost (found {other:?})"
                    ));
                }
            }
        }
    }
    Ok(())
}

/// splitmix64: the drills' choices, the same for the same seed.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ z.wrapping_shr(30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ z.wrapping_shr(27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ z.wrapping_shr(31)
    }

    /// One of 1..=n.
    fn upto(&mut self, n: u64) -> u64 {
        self.next()
            .checked_rem(n.max(1))
            .unwrap_or(0)
            .saturating_add(1)
    }
}

/// One machine's host, as the driver holds it: a connection for the node's lines and one for its
/// process.
struct Machine {
    id: u64,
    os: String,
    talk: Option<Conn>,
    control: Conn,
}

struct Lab {
    nodes: String,
    machines: Vec<Machine>,
    identities: Vec<(String, Vec<u8>)>,
}

#[derive(Clone, Copy, Debug)]
enum Kind {
    Leader,
    Follower,
    Own,
    TwoInTurn,
}

impl Kind {
    const fn name(self) -> &'static str {
        match self {
            Self::Leader => "leader",
            Self::Follower => "follower",
            Self::Own => "own",
            Self::TwoInTurn => "two-in-turn",
        }
    }
}

/// One kill of a drill: who, at which point, and what it found.
struct Kill {
    victim: u64,
    leader: bool,
    point: u64,
    /// The persistence point the kill came at, counted from the run's start, had it come before
    /// the run's end.
    fired: Option<u64>,
    ready_ms: u128,
}

fn wait_for<T>(what: &str, mut f: impl FnMut() -> Result<Option<T>, Fault>) -> Result<T, Fault> {
    let began = Instant::now();
    loop {
        if let Some(found) = f()? {
            return Ok(found);
        }
        if began.elapsed() > STUCK {
            return Err(format!("{what} didn't come within {STUCK:?}"));
        }
        pause(Duration::from_millis(20));
    }
}

impl Lab {
    fn machine(&mut self, id: u64) -> Result<&mut Machine, Fault> {
        self.machines
            .iter_mut()
            .find(|m| m.id == id)
            .ok_or_else(|| format!("no node {id}"))
    }

    fn talk(&mut self, id: u64) -> Result<&mut Conn, Fault> {
        self.machine(id)?
            .talk
            .as_mut()
            .ok_or_else(|| format!("node {id}'s talk is lent"))
    }

    fn ids(&self) -> Vec<u64> {
        self.machines.iter().map(|m| m.id).collect()
    }

    fn os(&self, id: u64) -> String {
        self.machines
            .iter()
            .find(|m| m.id == id)
            .map_or_else(String::new, |m| m.os.clone())
    }

    fn control(&mut self, id: u64, line: &str) -> Result<String, Fault> {
        self.machine(id)?.control.ask(line)
    }

    fn spawn(&mut self, id: u64) -> Result<(), Fault> {
        let nodes = self.nodes.clone();
        match self.control(id, &format!("spawn {id} {nodes}"))?.as_str() {
            "spawned" => Ok(()),
            other => Err(format!("node {id}'s spawn: {other}")),
        }
    }

    fn ready(&mut self, id: u64) -> Result<(), Fault> {
        let answer = self.control(id, "ready")?;
        if answer.starts_with("ready ") {
            Ok(())
        } else {
            Err(format!("node {id} isn't ready: {answer}"))
        }
    }

    /// A fresh cluster: every node's directory gone, the identities given again, each started,
    /// the topic made and led.
    fn fresh(&mut self) -> Result<(), Fault> {
        for id in self.ids() {
            match self.control(id, "wipe")?.as_str() {
                "ok" => {}
                other => return Err(format!("node {id}'s wipe: {other}")),
            }
            for (path, bytes) in self.identities.clone() {
                let data = if bytes.is_empty() {
                    "-".to_owned()
                } else {
                    hex(&bytes)
                };
                match self.control(id, &format!("put {path} {data}"))?.as_str() {
                    "ok" => {}
                    other => return Err(format!("node {id}'s {path}: {other}")),
                }
            }
        }
        for id in self.ids() {
            self.spawn(id)?;
        }
        for id in self.ids() {
            self.ready(id)?;
        }
        let first = self.ids().first().copied().ok_or("no nodes")?;
        match self.talk(first)?.node(&format!("create {TOPIC}"))?.as_str() {
            "ok" => {}
            other => return Err(format!("create: {other}")),
        }
        self.leader().map(|_| ())
    }

    /// The topic's leader, once one leads.
    fn leader(&mut self) -> Result<u64, Fault> {
        wait_for("a leader of the topic's group", || {
            for id in self.ids() {
                if self.talk(id)?.figures()?.0 {
                    return Ok(Some(id));
                }
            }
            Ok(None)
        })
    }

    /// The topic group's term: the highest any node knows.
    fn term(&mut self) -> Result<u64, Fault> {
        let mut term = 0;
        for id in self.ids() {
            term = term.max(self.talk(id)?.figures()?.1);
        }
        Ok(term)
    }

    /// Waits until every node holds every message up to `end`, and as many as each other.
    fn converge(&mut self, end: u64) -> Result<(), Fault> {
        wait_for("every node to hold every acknowledged publish", || {
            let mut ends = Vec::new();
            for id in self.ids() {
                ends.push(self.talk(id)?.figures()?.2);
            }
            let first = ends.first().copied().unwrap_or(0);
            Ok(if first >= end && ends.iter().all(|e| *e == first) {
                Some(())
            } else {
                None
            })
        })
    }

    /// Node `victim` killed at its `point`-th persistence point from now while a run of publishes
    /// goes through `through` (and, once that one fails it, through the third node), then started
    /// again.
    fn kill_during_run(
        &mut self,
        drill: u64,
        phase: char,
        victim: u64,
        through: u64,
        point: u64,
        heard: &mut Heard,
    ) -> Result<Kill, Fault> {
        let leader = self.talk(victim)?.figures()?.0;
        let fallback = self
            .ids()
            .into_iter()
            .find(|id| *id != victim && *id != through)
            .ok_or("no third node")?;
        let armed = self.control(victim, &format!("arm {point}"))?;
        let base = armed
            .strip_prefix("armed ")
            .and_then(|b| b.parse::<u64>().ok())
            .ok_or_else(|| format!("node {victim}'s arm: {armed}"))?;
        let mut first = self.machine(through)?.talk.take().ok_or("lent")?;
        let mut second = self.machine(fallback)?.talk.take().ok_or("lent")?;
        let publisher = std::thread::spawn(move || {
            let (mut acked, mut unknown, mut failed) = (Vec::new(), Vec::new(), None);
            let mut on_second = false;
            for i in 0..RUN {
                let payload = format!("{drill}-{phase}{i}");
                let talk = if on_second { &mut second } else { &mut first };
                match talk.publish(&payload) {
                    Ok(offset) => acked.push((payload, offset)),
                    Err(why) if why.starts_with(LAB) => {
                        unknown.push(payload);
                        failed = Some(why);
                        break;
                    }
                    // Refused, or its process killed under it: its fate is unknown.
                    Err(_) => {
                        unknown.push(payload);
                        on_second = true;
                    }
                }
            }
            (acked, unknown, failed, first, second)
        });
        // The kill comes at its point, from the victim's host, or once the run ended before it.
        let mut fired = None;
        let began = Instant::now();
        while fired.is_none() && !publisher.is_finished() {
            let status = self.control(victim, "status")?;
            if status.starts_with("exited") {
                return Err(format!(
                    "drill {drill}: node {victim} ended by itself ({status})"
                ));
            }
            if let Some(at) = status
                .strip_prefix("down ")
                .and_then(|rest| rest.split(' ').nth(1))
                .and_then(|f| f.parse::<u64>().ok())
            {
                fired = Some(at.saturating_sub(base));
            }
            if began.elapsed() > STUCK {
                return Err(format!(
                    "drill {drill}: the run didn't end within {STUCK:?}"
                ));
            }
            pause(POLL);
        }
        if fired.is_none() {
            let killed = self.control(victim, "kill")?;
            if !killed.starts_with("killed") && killed != "down" {
                return Err(format!("node {victim}'s kill: {killed}"));
            }
        }
        let (acked, unknown, failed, first, second) = publisher
            .join()
            .map_err(|_| "the publisher's thread panicked".to_owned())?;
        self.machine(through)?.talk = Some(first);
        self.machine(fallback)?.talk = Some(second);
        heard.acked.extend(acked);
        heard.unknown.extend(unknown);
        if let Some(why) = failed {
            return Err(why);
        }
        let began = Instant::now();
        self.spawn(victim)?;
        self.ready(victim)?;
        Ok(Kill {
            victim,
            leader,
            point,
            fired,
            ready_ms: began.elapsed().as_millis(),
        })
    }

    /// Node `id` made the topic's leader: each other leader killed and started again until it
    /// leads. The elections it took, none if it led already.
    fn lead(&mut self, id: u64) -> Result<u64, Fault> {
        let mut kills = 0u64;
        loop {
            let leader = self.leader()?;
            if leader == id {
                return Ok(kills);
            }
            if kills >= 40 {
                return Err(format!("node {id} didn't come to lead in 40 elections"));
            }
            let killed = self.control(leader, "kill")?;
            if !killed.starts_with("killed") {
                return Err(format!("node {leader}'s kill: {killed}"));
            }
            // Its successor chosen before it's back, so it can't win again at once.
            wait_for("a leader among the others", || {
                for other in self.ids().into_iter().filter(|o| *o != leader) {
                    if self.talk(other)?.figures()?.0 {
                        return Ok(Some(()));
                    }
                }
                Ok(None)
            })?;
            self.spawn(leader)?;
            self.ready(leader)?;
            kills = kills.saturating_add(1);
        }
    }

    fn drill(
        &mut self,
        n: u64,
        kind: Kind,
        rng: &mut Rng,
        heard: &mut Heard,
    ) -> Result<String, Fault> {
        let began = Instant::now();
        let term_before = self.term()?;
        let leader_before = self.leader()?;
        let ids = self.ids();
        let followers: Vec<u64> = ids
            .iter()
            .copied()
            .filter(|id| *id != leader_before)
            .collect();
        let (Some(&one), Some(&other)) = (followers.first(), followers.get(1)) else {
            return Err("fewer than two followers".into());
        };
        let mut kills = Vec::new();
        match kind {
            Kind::Leader => kills.push(self.kill_during_run(
                n,
                'a',
                leader_before,
                one,
                rng.upto(POINTS),
                heard,
            )?),
            Kind::Follower => {
                kills.push(self.kill_during_run(n, 'a', one, other, rng.upto(POINTS), heard)?)
            }
            Kind::Own => kills.push(self.kill_during_run(
                n,
                'a',
                leader_before,
                leader_before,
                rng.upto(POINTS),
                heard,
            )?),
            Kind::TwoInTurn => {
                kills.push(self.kill_during_run(
                    n,
                    'a',
                    one,
                    leader_before,
                    rng.upto(POINTS),
                    heard,
                )?);
                let end = heard
                    .acked
                    .values()
                    .max()
                    .map_or(0, |o| o.saturating_add(1));
                self.converge(end)?;
                let leader = self.leader()?;
                let through = ids
                    .iter()
                    .copied()
                    .find(|id| *id != leader)
                    .ok_or("no follower")?;
                kills.push(self.kill_during_run(
                    n,
                    'c',
                    leader,
                    through,
                    rng.upto(POINTS),
                    heard,
                )?);
            }
        }
        // Once it's back, a publish through every node.
        for id in self.ids() {
            let mut attempt = 0u64;
            wait_for(
                &format!("a publish through node {id} after the kill"),
                || {
                    attempt = attempt.saturating_add(1);
                    let payload = format!("{n}-b{id}-{attempt}");
                    match self.talk(id)?.publish(&payload) {
                        Ok(offset) => {
                            heard.acked.insert(payload, offset);
                            Ok(Some(()))
                        }
                        Err(why)
                            if why.starts_with(LAB)
                                || why.contains("ended")
                                || why.contains("down") =>
                        {
                            Err(why)
                        }
                        // Refused while a leader is chosen: it may be appended yet.
                        Err(_) => {
                            heard.unknown.insert(payload);
                            Ok(None)
                        }
                    }
                },
            )?;
        }
        let end = heard
            .acked
            .values()
            .max()
            .map_or(0, |o| o.saturating_add(1));
        self.converge(end)?;
        let mut logs = Vec::new();
        for id in self.ids() {
            logs.push((id, self.talk(id)?.log()?));
        }
        check(heard, &logs).map_err(|why| format!("drill {n} ({}): {why}", kind.name()))?;
        let term_after = self.term()?;
        let leader_after = self.leader()?;
        let mut record = format!(
            "{{\"drill\":{n},\"kind\":\"{}\",\"term_before\":{term_before},\"term_after\":{term_after},\"elections\":{},\"leader_before\":{leader_before},\"leader_after\":{leader_after},\"messages\":{},\"acked\":{},\"unknown\":{},\"ms\":{},\"kills\":[",
            kind.name(),
            term_after.saturating_sub(term_before),
            logs.first().map_or(0, |(_, l)| l.len()),
            heard.acked.len(),
            heard.unknown.len(),
            began.elapsed().as_millis()
        );
        for (i, kill) in kills.iter().enumerate() {
            let _ = write!(
                record,
                "{}{{\"victim\":{},\"os\":\"{}\",\"leader\":{},\"point\":{},\"fired\":{},\"ready_ms\":{}}}",
                if i == 0 { "" } else { "," },
                kill.victim,
                self.os(kill.victim),
                kill.leader,
                kill.point,
                kill.fired
                    .map_or_else(|| "null".to_owned(), |f| f.to_string()),
                kill.ready_ms
            );
        }
        record.push_str("]}");
        Ok(record)
    }
}

fn pairs(value: &str, flag: &str) -> Result<Vec<(u64, String)>, Fault> {
    value
        .split(',')
        .map(|pair| {
            let (id, address) = pair
                .split_once('=')
                .ok_or_else(|| format!("--{flag}: ID=ADDRESS,…"))?;
            Ok((
                id.parse().map_err(|_| format!("--{flag}: {id}"))?,
                address.to_owned(),
            ))
        })
        .collect()
}

/// Every file under `dir`, by its path from `dir` with `/` between its parts.
fn files(dir: &Path, under: &str, found: &mut Vec<(String, Vec<u8>)>) -> Result<(), Fault> {
    for entry in std::fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|_| "a file name isn't Unicode")?;
        let path = if under.is_empty() {
            name
        } else {
            format!("{under}/{name}")
        };
        if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            files(&entry.path(), &path, found)?;
        } else {
            found.push((
                path,
                std::fs::read(entry.path()).map_err(|e| e.to_string())?,
            ));
        }
    }
    Ok(())
}

/// The lab `flags` name: each host reached, the cluster's identities made once, and the file the
/// records go to.
fn open_lab(flags: &BTreeMap<String, String>) -> Result<(Lab, Option<std::fs::File>), Fault> {
    let hosts = pairs(flags.get("hosts").ok_or("--hosts is needed")?, "hosts")?;
    let nodes = flags.get("nodes").ok_or("--nodes is needed")?.clone();
    for (_, address) in pairs(&nodes, "nodes")? {
        address
            .parse::<SocketAddr>()
            .map_err(|e| format!("--nodes: {address}: {e}"))?;
    }
    let work = PathBuf::from(
        flags
            .get("work")
            .cloned()
            .unwrap_or_else(|| format!("quiver-lab-{}", std::process::id())),
    );
    // The cluster's identities, made once here and given to every host: as an operator makes them.
    focal_bus::pki::load_or_make(
        &work.join("identities"),
        Some(&work.join("identities.key")),
        3,
        3,
    )
    .map_err(|e| format!("the identities: {e:?}"))?;
    let mut identities = Vec::new();
    files(&work.join("identities"), "identities", &mut identities)?;
    identities.push((
        "identities.key".to_owned(),
        std::fs::read(work.join("identities.key")).map_err(|e| e.to_string())?,
    ));
    let token = token(flags)?;
    let mut machines = Vec::new();
    for (id, address) in &hosts {
        let mut control = Conn::open(address, &token)?;
        let pong = control.ask("ping")?;
        let os = pong
            .strip_prefix("pong ")
            .ok_or_else(|| format!("{address}: {pong}"))?
            .to_owned();
        machines.push(Machine {
            id: *id,
            os,
            talk: Some(Conn::open(address, &token)?),
            control,
        });
    }
    let record = match flags.get("out") {
        Some(path) => Some(
            std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .map_err(|e| format!("{path}: {e}"))?,
        ),
        None => None,
    };
    Ok((
        Lab {
            nodes,
            machines,
            identities,
        },
        record,
    ))
}

fn drive(flags: &BTreeMap<String, String>) -> Result<(), Fault> {
    let drills: u64 = flags
        .get("drills")
        .ok_or("--drills is needed")?
        .parse()
        .map_err(|_| "--drills: a count")?;
    let fresh_every: u64 = flags
        .get("fresh-every")
        .map_or(Ok(100), |v| v.parse())
        .map_err(|_| "--fresh-every: a count")?;
    // A lab stopped part way goes on from the drill after its last (`--first`), on a fresh cluster,
    // its numbers and its round of victims where they were.
    let first: u64 = flags
        .get("first")
        .map_or(Ok(1), |v| v.parse())
        .map_err(|_| "--first: a drill's number")?;
    let seed: u64 = match flags.get("seed") {
        Some(s) => s.parse().map_err(|_| "--seed: a number")?,
        None => SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_or(1, |d| {
                u64::try_from(d.as_nanos() & u128::from(u64::MAX)).unwrap_or(1)
            }),
    };
    let (mut lab, mut record) = open_lab(flags)?;
    let mut rng = Rng(seed);
    out(&format!(
        "seed {seed}, {drills} drills, a fresh cluster every {fresh_every}"
    ));
    let kinds = [Kind::Leader, Kind::Follower, Kind::Own, Kind::TwoInTurn];
    let mut heard = Heard::default();
    let (mut lost_any, mut follower_elections, mut leader_elections) = (false, 0u64, Vec::new());
    for n in first..first.saturating_add(drills) {
        if n == first || n.saturating_sub(1).checked_rem(fresh_every.max(1)) == Some(0) {
            lab.fresh()?;
            heard = Heard::default();
        }
        let kind = usize::try_from(n.saturating_sub(1).checked_rem(4).unwrap_or(0))
            .ok()
            .and_then(|i| kinds.get(i))
            .copied()
            .unwrap_or(Kind::Leader);
        let line = match lab.drill(n, kind, &mut rng, &mut heard) {
            Ok(line) => line,
            Err(why) => {
                lost_any = true;
                let line = format!(
                    "{{\"drill\":{n},\"kind\":\"{}\",\"failed\":{:?}}}",
                    kind.name(),
                    why
                );
                if let Some(file) = record.as_mut() {
                    let _ = writeln!(file, "{line}");
                }
                out(&line);
                break;
            }
        };
        let elections = line
            .split("\"elections\":")
            .nth(1)
            .and_then(|r| r.split(',').next())
            .and_then(|e| e.parse::<u64>().ok())
            .unwrap_or(0);
        match kind {
            Kind::Follower => follower_elections = follower_elections.saturating_add(elections),
            _ => leader_elections.push(elections),
        }
        if let Some(file) = record.as_mut() {
            let _ = writeln!(file, "{line}");
        }
        out(&line);
    }
    let leader_kills = leader_elections.len();
    let without = leader_elections.iter().filter(|e| **e == 0).count();
    out(&format!(
        "summary: {} drills, lost {}, elections at follower kills {follower_elections}, drills killing a leader {leader_kills} ({without} without an election)",
        drills,
        if lost_any {
            "see the failure above"
        } else {
            "0"
        }
    ));
    if lost_any {
        Err("a drill failed".into())
    } else {
        Ok(())
    }
}

/// What one publish of a stall drill came to: its payload, when it went and was answered (from the
/// drill's start), and its offset if it was acknowledged.
type Sent = (String, Duration, Duration, Option<u64>);

/// The duration at `per_mille` of the sorted `sorted`, in milliseconds.
fn at_ms(sorted: &[Duration], per_mille: usize) -> f64 {
    let last = sorted.len().saturating_sub(1);
    let at = last
        .saturating_mul(per_mille)
        .checked_div(1000)
        .unwrap_or(0);
    sorted.get(at).map_or(0.0, |d| d.as_secs_f64() * 1000.0)
}

fn stall(flags: &BTreeMap<String, String>) -> Result<(), Fault> {
    let number = |name: &str, default: u64| -> Result<u64, Fault> {
        flags.get(name).map_or(Ok(default), |v| {
            v.parse().map_err(|_| format!("--{name}: a number"))
        })
    };
    let stalled = flags
        .get("stalled")
        .ok_or("--stalled is needed")?
        .parse::<u64>()
        .map_err(|_| "--stalled: a node")?;
    let stalls = number("stalls", 10)?;
    let every = Duration::from_millis(number("every-ms", 10)?);
    let seconds = Duration::from_secs(number("seconds", 8)?);
    let at = Duration::from_millis(number("at-ms", 2000)?);
    let (mut lab, mut record) = open_lab(flags)?;
    lab.fresh()?;
    out(&format!(
        "{stalls} stalls of node {stalled}'s disk ({}), a publish every {every:?} for {seconds:?}, the stall {at:?} in",
        lab.os(stalled)
    ));
    let mut heard = Heard::default();
    let (mut elections, mut moved, mut during_offered, mut during_acked) = (0u64, 0u64, 0u64, 0u64);
    for k in 1..=stalls {
        let setup = lab.lead(stalled)?;
        let end = heard
            .acked
            .values()
            .max()
            .map_or(0, |o| o.saturating_add(1));
        lab.converge(end)?;
        let term_before = lab.term()?;
        let through = lab
            .ids()
            .into_iter()
            .find(|id| *id != stalled)
            .ok_or("no follower")?;
        let mut talk = lab.machine(through)?.talk.take().ok_or("lent")?;
        let start = Instant::now();
        let publisher = std::thread::spawn(move || {
            let mut sent: Vec<Sent> = Vec::new();
            let mut i = 0u32;
            while start.elapsed() < seconds {
                let due = every.saturating_mul(i);
                let now = start.elapsed();
                if now < due {
                    pause(due.saturating_sub(now));
                    continue;
                }
                let payload = format!("s{k}-{i}");
                let went = start.elapsed();
                let acked = talk.publish(&payload).ok();
                sent.push((payload, went, start.elapsed(), acked));
                i = i.saturating_add(1);
            }
            (sent, talk)
        });
        while start.elapsed() < at {
            pause(at.saturating_sub(start.elapsed()));
        }
        let began = start.elapsed();
        let answer = lab.control(stalled, "stall");
        let ended = start.elapsed();
        let (sent, talk) = publisher
            .join()
            .map_err(|_| "the publisher's thread panicked".to_owned())?;
        lab.machine(through)?.talk = Some(talk);
        let answer = answer?;
        if !answer.starts_with("stalled ") {
            return Err(format!("stall {k}: {answer}"));
        }
        let term_after = lab.term()?;
        let leader_after = lab.leader()?;
        let (mut offered, mut acked, mut in_offered, mut in_acked) = (0u64, 0u64, 0u64, 0u64);
        let (mut latencies, mut in_latencies) = (Vec::new(), Vec::new());
        for (payload, went, answered, offset) in sent {
            let during = went >= began && went <= ended;
            offered = offered.saturating_add(1);
            if during {
                in_offered = in_offered.saturating_add(1);
            }
            match offset {
                Some(offset) => {
                    acked = acked.saturating_add(1);
                    latencies.push(answered.saturating_sub(went));
                    if during {
                        in_acked = in_acked.saturating_add(1);
                        in_latencies.push(answered.saturating_sub(went));
                    }
                    heard.acked.insert(payload, offset);
                }
                None => {
                    heard.unknown.insert(payload);
                }
            }
        }
        latencies.sort_unstable();
        in_latencies.sort_unstable();
        let end = heard
            .acked
            .values()
            .max()
            .map_or(0, |o| o.saturating_add(1));
        lab.converge(end)?;
        let mut logs = Vec::new();
        for id in lab.ids() {
            logs.push((id, lab.talk(id)?.log()?));
        }
        check(&heard, &logs).map_err(|why| format!("stall {k}: {why}"))?;
        let cost = term_after.saturating_sub(term_before);
        elections = elections.saturating_add(cost);
        if leader_after != stalled {
            moved = moved.saturating_add(1);
        }
        during_offered = during_offered.saturating_add(in_offered);
        during_acked = during_acked.saturating_add(in_acked);
        let line = format!(
            "{{\"stall\":{k},\"stalled\":{stalled},\"os\":\"{}\",\"setup_elections\":{setup},\"term_before\":{term_before},\"term_after\":{term_after},\"elections\":{cost},\"leader_after\":{leader_after},\"helper\":\"{answer}\",\"began_ms\":{},\"ended_ms\":{},\"offered\":{offered},\"acked\":{acked},\"during_offered\":{in_offered},\"during_acked\":{in_acked},\"during_max_ms\":{:.1},\"p50_ms\":{:.1},\"p99_ms\":{:.1},\"max_ms\":{:.1}}}",
            lab.os(stalled),
            began.as_millis(),
            ended.as_millis(),
            at_ms(&in_latencies, 1000),
            at_ms(&latencies, 500),
            at_ms(&latencies, 990),
            at_ms(&latencies, 1000)
        );
        if let Some(file) = record.as_mut() {
            let _ = writeln!(file, "{line}");
        }
        out(&line);
    }
    out(&format!(
        "summary: {stalls} stalls of node {stalled}'s disk, {elections} elections, the leader moved {moved} times, {during_acked} of {during_offered} publishes made during the stalls acknowledged"
    ));
    if elections == 0 && moved == 0 {
        Ok(())
    } else {
        Err("a stall cost an election".into())
    }
}

fn main() -> ExitCode {
    let args: Result<Vec<String>, _> = std::env::args_os()
        .skip(1)
        .map(|a| a.into_string())
        .collect();
    let outcome = match args.as_deref() {
        Ok([mode, rest @ ..]) => flags(rest).and_then(|f| match mode.as_str() {
            "host" => host(&f),
            "drive" => drive(&f),
            "stall" => stall(&f),
            other => Err(format!("unknown mode {other}: host or drive")),
        }),
        Ok([]) => Err("quiver-lab host|drive …".into()),
        Err(_) => Err("an argument isn't Unicode".into()),
    };
    match outcome {
        Ok(()) => ExitCode::SUCCESS,
        Err(why) => {
            err(&format!("quiver-lab: {why}"));
            ExitCode::from(2)
        }
    }
}
