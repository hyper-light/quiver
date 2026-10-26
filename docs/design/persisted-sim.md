# The persisted class under simulation (kiln's CP-T2.2, step 6)

Status: 2026-10-26, proposed and being built for CP-T2.2 step 6 (AC-T2.2.5, kiln plan
`01a16c5b7e8c75acbdfafd51315c3885`). How Quiver's persisted class runs in a deterministic
simulation, what the simulation does to it, and what each run checks. CLAUDE.md §5: each decision
with its sources, its evidence and what would falsify it.

## What it must show

The step: Quiver's persisted class under kill -9 of any node at every write and await,
partitions, duplicated and lost messages, disk stalls and the storage model's crash states;
every acknowledged publish checked present, in its producer's order, after every recovery; each
failing seed replayable from its trace. Done when no acknowledged publish is lost over 10,000
seeds, and a planted bug (acknowledgement before a quorum's flush, a test build) is found and
replayed from its seed.

The kill drills of step 4 (`tests/persisted_kill.rs`, `tests/persisted_processes.rs`) run in real
time on real disks. They kill at each frame of one run of publishes and cannot replay what they
find. This simulation runs the same code on virtual time, a simulated network and simulated
devices, so a seed decides every event and a failing seed runs again the same.

## The world

- **Runtime and network:** hyper-rt's simulation runtime (`hyper_rt::sim::SimRuntime`,
  hyper-raft `docs/runtime.md` §11). It is one thread with virtual time and a UDP fabric that
  models one-way delay with seeded jitter, reordering, Gilbert–Elliott loss, a path MTU and
  bottleneck links. hyper-rt-transport's `Driver` binds on its ports, as on real sockets
  (hyper-rt-transport `tests/simulation.rs`).
- **Nodes:** three, each a shard of its own running the node's host task (`persisted::host`). This
  is Quiver's code as it ships: the catalog, the members, the publishers, the views. Focal's
  member and client run over hyper-transport's endpoints.
- **Disks:** each member's hyper-log sits on hyper-sim's block device
  (`hyper_block::sim::SimDevice`). It has power-loss semantics: unflushed sectors survive a crash
  per `Crash` (`Random`, `LoseAll`, `KeepAll`), and `Fault::PowerCut` cuts power at a stated
  write or flush. A part's image and the node's small files (its run, its ports, the log's keys)
  are real files in the seed's directory under `~/.cache`. Their writes are synchronous and made
  durable before use, so a kill leaves them as a real kill would.
- **Clients:** each node's API publishes records at virtual times the seed draws. Each record
  names its producer (a node and its run) and its number in the producer's order.

## Determinism

A seed must decide every event. Four sources of variation are closed:

1. **Time.** Every `Instant` in a node comes from one process-wide anchor and the shard's
   virtual clock: the transport driver anchors to it on a simulated shard, and the host reads its
   clock from its driver rather than `Instant::now()`. Today each driver anchors to
   `Instant::now()` when it is made, so a host's drivers would disagree by the real time between
   their making.
2. **Randomness.** Endpoints are made seeded (`Endpoint::new_seeded`). TLS draws its own bytes
   from AWS-LC, but their sizes, and so the flow, are fixed while the certificates are the same
   (hyper-rt-transport `tests/simulation.rs`). The cluster's identities are made once in the run's
   directory, and a trace records their digest.
3. **Threads.** hyper-log runs an owner thread and an I/O thread, and the simulated device a
   thread of its own. When a log answers depends on them, not on the seed, and so does what one
   frame holds. In the simulation each log runs under `Waits::Told`, so the owner writes a frame
   only when told. The host tells it at the virtual time its disk model draws for the write, then
   waits on the shard's thread until that frame is answered. It also waits for every read it
   asked of the log. So every answer reaches the member at a virtual instant the seed chose, and
   each frame holds what was submitted before its cut, in order. A stall is a cut drawn late.
4. **Hash-map order.** Focal's member redirects the acknowledgements it owed in a `HashMap`'s
   order when it stops leading, which differs from run to run. The simulation needs them sorted.

**What would falsify it:** a seed whose run differs from itself. `twice` runs each seed twice
and refuses a run whose digest differs. The digest covers every node's events, acknowledgements
and terms, the fabric's counts and every device's operation counts.

## What the simulation does

- **kill -9 of a node**, the moment drawn from the seed: at its host's n-th await; before a cut
  (what it hadn't cut is lost, as hyper-log's queue is with a process); during a cut (power cut at
  the cut's j-th write or flush, a torn frame); or after a cut, before the member takes its
  answers (the leader killed between its flush and its acknowledgement). The node's devices are
  then crashed with `Crash::KeepAll`, for a kill (the OS kept what was written), or with
  `Crash::Random` or `LoseAll`, for a power loss. Each restarts after a drawn downtime from what
  its devices and files kept.
- **Partitions:** each pair of nodes cut one or both ways for a drawn time, then healed.
- **Loss, reordering and jitter** on every path, from the fabric's model. Duplicated messages are
  the publishers' resends: after a redirect, a silent leader, a lost connection or a lost answer.
  The part's sessions must recognise each.
- **Disk stalls:** a cut drawn 0.5 to 3 s late on one node or on all at once (doc 34's shared
  disk).

After the schedule, every partition heals, every node is up, and the run goes on until each node
has applied the same offsets.

## What each run checks

- every acknowledged publish is at the offset it was acknowledged with, in every node's view;
- each producer's acknowledged publishes are in its order;
- no record is there twice;
- the nodes' views agree.

A run's trace is its seed, the schedule the seed drew, and the digest at each slice. A failing
seed is written with its trace, and it replays from either.

## The planted bug

A test build (`--features plant-own-flush-commit`) has the group commit an entry once the
leader's own log holds it, before a quorum's flush. Without faults no acknowledged publish is
lost. A leader that loses power after its own flush, before the followers have the entry, loses
it once the followers elect a leader without it. The simulation must find this and replay it from
its seed.

## The changes to vendored code

Each is recorded in the project's `UPSTREAM.md` and offered upstream. None changes what a
production build does.

| Project | Change |
|---|---|
| hyper-raft `hyper-log` | `Waits::Told`: the owner writes a frame only when told; `Log::cut`, answered once that frame is |
| hyper-raft `hyper-rt-transport` | on a simulated shard, the driver anchors to one process-wide instant |
| Focal `focal-bus` | a member's log on a given device and waits (`sim` feature); its cut and its writes and reads in flight; the owed acknowledgements redirected in order |
| hyper-raft `hyper-durable` | the planted bug, under its feature |

## Runs

Each seed takes three nodes through a few seconds of virtual time. The 10,000 seeds are spread over
the Mac Studio (adas-mac-studio-3) and fedora, never production's adacachy, and their record goes
in `bench/records/`.
