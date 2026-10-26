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

A seed must decide every event. Five sources of variation are closed:

1. **Time.** Every `Instant` in a node comes from one process-wide anchor
   (`hyper_rt::sim::anchor`) and the shard's virtual clock. On a simulated shard
   (`ShardContext::driver_is_sim`) the transport's driver anchors to it, and
   `hyper_rt_transport::shard_instant` gives the same reading to an endpoint made before its
   driver and to the host's own clock. Off a simulated shard nothing changes. Before this, each
   driver anchored to `Instant::now()` when it was made, so a host's drivers disagreed by the real
   time between their making.
2. **Randomness.** Each endpoint's QUIC layer is seeded (`Endpoint::new_seeded`) from the world's
   seed, the node, its start and the endpoint's place in the start's order. TLS draws its own bytes
   from AWS-LC, but their sizes, and so the flow, are fixed while the certificates are the same
   (hyper-rt-transport `tests/simulation.rs`). The cluster's identities are made once and kept, so
   a run replays across processes that share them. hyper-durable's member incarnation is drawn
   from the system, but it is fixed-width on the wire and only ever compared for equality.
3. **Threads.** hyper-log runs an owner thread and an I/O thread, and the simulated device a
   thread of its own. When a log answers depends on them, not on the seed, and so does what one
   frame holds. In the simulation each log runs under `Waits::Told`:
   - the owner writes a frame only when its caller cuts (`Log::cut`), and the frame holds
     everything submitted before the cut, in order;
   - the host cuts at the virtual time its disk model draws for the write, then waits on the
     shard's thread until the cut is answered, which is once that frame is durable and confirmed;
   - each confirmation is made durable one way, not the way the device measured cheaper in real
     time;
   - a caller that waits on its own answer from the moment it asks tells the log by waiting.
     That covers the restore an open writes, `Replica::open`'s reconciling write
     (`GroupStore::write_now`), and every blocking fetch: a replica reads its entries through
     `GroupLog::fetch`, which waits. Its thread is blocked until the answer, so the order of what
     the log does is still its own.
   - A fetch made with `fetch_waking` waits for a serve (`Log::serve`). The member never makes
     one.

   So every answer reaches the member at a virtual instant the seed chose. A stall is a cut drawn
   late.
4. **Hash-map order.** Focal's member redirected the acknowledgements it owed in a `HashMap`'s
   order when it stopped leading, which differs from run to run; now they're sorted first.
5. **Ports.** A simulated bind took the next free port of the shard, whatever was asked, from a
   stack. A node started again took its sockets back in another order, so its catalog's member
   listened where its peers never dial. Now a port of the shard's own range asked for by number is
   taken if free and refused if bound, as an OS binds one.

**What would falsify it:** a seed whose run differs from itself. Each failing seed is run again,
and its digest must match. `a_seeds_run_is_the_same_run_twice` compares two runs' traces,
outcomes, views and digests. The digest covers the trace (every fault and kill, with when), every
publish's outcome, every node's view, the fabric's counts and every device's writes and flushes.

## What the simulation found

- **A moved peer was unreachable for two minutes.** A topic's member started again binds the port
  it kept, unless another socket took it while the node was down. Then it binds another and
  records it in the catalog. Its peers took the new address, but went on with what they held or
  were dialing at the old one. A dial in flight holds off every new dial to that peer until its
  handshake times out (`Endpoint::connect`), which is two minutes. In seed 3, node 3 started
  again while node 2 was down, and the two dialed each other's old ports. Node 3 never caught up
  within the run's 60 seconds once the network mended. Now a member and a publisher close what
  they hold to a peer that moved and dial it at its new address
  (`Seat::peers_changed`, `Publisher::set_addresses`). A real network moves ports rarely, as an
  OS's ephemeral ports rarely collide. The simulation's ports collide often, so it found this.

- **A node back from a kill could not publish for two minutes.** A transport keeps a peer's
  connections until they have carried nothing for its serve period, two minutes. A process back
  from a kill dials with the same identity, and the dead process's connections hold both of its
  places, so it is refused Busy. Quiver's members closed a suspected peer's connections for this
  (step 4's kill drills), but only its member's. The node's publisher, a client identity of its
  own, was refused by every member until the old connections aged out: in seed 8, 4,576 times in
  a minute. Now a member that suspects a node closes that node's publisher's connections too
  (`Seat::turn`, `Plan::publishers`).
- **A node's own publisher kept its own node's old port.** A topic's publisher took the catalog's
  new addresses only when its member's peers changed, and a member's peers leave its own node
  out. When only this node's member moved, its publisher went on dialing the old port, and a node
  that led never reached itself: in seeds 9, 13 and 14 the leader's own publishes were redirected
  thousands of times, never sent to the leader. Now the publisher takes every address the catalog
  changes, its own node's included (`Host::reconcile`).

Each of these runs again as its seed, and seeds 1 to 20 pass with all three fixed.

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

A test build (`--features plant-own-log-commit`) plants hyper-raft's `Mutant::OwnLogCommit` in
every member's replica. The group then commits what its leader's own durable log holds, before a
quorum's flush. Without faults no acknowledged publish is lost. A leader killed after its own flush,
before the followers have the entry, loses it once they elect a leader without it. The simulation
must find this and replay it from its seed
(`the_planted_acknowledgement_before_a_quorums_flush_is_found_and_replayed`). hyper-raft's own
oracles catch it too (`tests/check.rs`, seed 0).

## The changes to vendored code

Each is recorded in its project's `UPSTREAM.md` and offered upstream (hyper-raft's on the local
branch `cp/t2.2-told` at 25cf889). None changes what a production build does: the told and serve
paths need `Waits::Told`, the anchor and binds a simulated shard, and the rest a feature.

| Project | Change |
|---|---|
| hyper-raft `hyper-log` | `Waits::Told` and `Log::cut`: a frame only when told, its confirmation made one way. `Log::serve` for waking fetches. A restore at open, and a caller that waits on its write or fetch, tell the log by waiting |
| hyper-raft `hyper-rt`, `hyper-rt-transport` | `sim::anchor` and `shard_instant`: one clock for every driver on a simulated shard. A simulated bind takes the port asked for in its shard |
| hyper-raft `hyper-raft`, `hyper-durable` | `Mutant::OwnLogCommit`, and `Replica::plant` to reach it (`mutants` features) |
| Focal `focal-bus` | `NodeFile` and `Member::open_simulated` (`sim` feature): a member's log on a simulated device. `Member::cut`, `serve`, `in_flight` and `plant` (`mutants` feature). The owed acknowledgements redirected in order |

## Runs

Each seed takes three nodes through a few seconds of virtual time. The 10,000 seeds are spread over
the Mac Studio (adas-mac-studio-3) and fedora, never production's adacachy, and their record goes
in `bench/records/`.
