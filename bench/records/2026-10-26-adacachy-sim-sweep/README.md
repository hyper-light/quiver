# The persisted class's simulation over 10,000 seeds, 2026-10-26: AC-T2.2.5

kiln's CP-T2.2, step 6. Quiver at `a52fb28` (branch `cp/t2.2-sim`), Focal's bus at `b6cf8e3` and
hyper-raft at `c6baec2`, each with the changes its `UPSTREAM.md` records (the bus's five,
hyper-raft's six). On adacachy (AMD Ryzen 9 9950X3D2, 32 threads, 124 GB, Linux
7.2.9-2-cachyos), production's host, under its caps: each process a systemd unit at Nice 19 and
CPUWeight 1, 4 GiB, its disk at 50 MB/s each way, with a watchdog on each pass that would stop it
were production's `/healthz` over 3 s or the system's disk pressure high twice running (it never
acted). fedora, the machine meant for it, had stopped answering runs that morning. One release
build; the cluster's identities made once, by one process, before the passes started; each pass in
a directory of its own holding a copy of them (`QUIVER_SIM_ROOT`). 11:51 to 12:36 PDT.

## What it measured

Seeds 1 to 10,000 (`docs/design/persisted-sim.md`: 6 s of publishes and faults on virtual time,
then everything mended and every node caught up, at most a minute), twice: pass A in eight
processes of 1,250 seeds, pass B in nine shifted by 625, so every seed ran after other seeds in
another process.

| | pass A | pass B |
|---|---|---|
| seeds | 10,000 | 10,000 |
| broken (any check) | 0 | 0 |
| publishes | 3,226,261 | 3,226,261 |
| acknowledged | 3,181,568 (98.61%) | 3,181,568 |
| **acknowledged and lost** | **0** | **0** |
| digests alike with the other pass | 10,000 | 10,000 |

The faults each pass drew (alike in both; `seeds.tsv`, a seed a line, is pass A's, and pass B's is
byte for byte the same):

- **kill -9**: 18,987 of the 19,446 planned (one planned at an await or a cut its node never
  reached before the run ended does not fall), in 9,207 seeds: 7,594 at a host's await, 3,738 before a disk's cut,
  3,883 during one, 3,772 after one. 12,502 took the leader, in 7,822 seeds; 2,445 of those fell
  between the leader's flush and its acknowledgement, in 2,219 seeds.
- **crash states** of the killed node's devices: kept 6,334, torn at random 6,341, lost 6,312.
- **partitions**: 3,958 one way, 9,365 both ways, in 6,801 seeds.
- **disk stalls**: 7,573 of one disk, 3,228 of every disk at once, in 8,557 seeds.
- **the network**: 4,984 seeds reordering, 5,010 losing in bursts (a burst ending in time on a
  quiet flow), the rest one by one; 1,858,622 datagrams delivered twice, 3,361,555 lost.
- **resends**: 3,830,692 batches sent again, 962,516 of them recognised by a leader that had them.
- **checks**: 1,713,975 views checked while the runs went on, every 100 ms of virtual time, and
  every node's at each run's end.

## AC-T2.2.5

- **Met: acknowledged publishes lost under kill -9 of any node, over 10,000 seeds: exactly 0,** in
  each pass. Every acknowledged publish was at its offset in every node's view at the end and in
  every view checked while it ran, each producer's in its order, none twice.
- **Each seed replayable from its trace:** every seed ran to the same digest in both passes, in
  processes that had run other seeds before it (10,000 of 10,000 alike). A run's digest folds its
  trace, every publish's outcome, every view, the devices' operations and the fabric's counts.
- **The planted bug** (`--features plant-own-log-commit`, a group committing what its leader's own
  log holds before a quorum's flush) is found at seed 3, publishes it applied changed (nodes 1 and
  2 differ at offsets 173 and 174), and replayed to the same digest (`tests/sim.rs`).
- **The example:** a leader killed between its flush and its acknowledgement came 2,445 times; the
  node that took over had what was acknowledged, and a resend it already held was recognised, not
  taken twice. (Seed 4711 of this generator kills its leader at an await: 318 publishes, 318
  acknowledged, 0 lost, digest `5887efe3b95f8d7c` in both passes.)

## Before this run

The sweep at `bf54d1e` (both passes, the same seeds) also lost no acknowledged publish, but broke
three seeds and ran 1,067 seeds apart from one process to the other. The three (3053, 6302 and
7452) were a publisher's handshake caught in a loss burst that never ended: the simulated path's
chain stepped only per datagram, and the handshake's probes, backing off, sent one every few
seconds. A burst now ends in time on a quiet flow (hyper-raft's recorded change 0005). The 1,067
were two things, found by logging every datagram and every QUIC packet of a run in both passes: a
certificate verify a byte longer in one run than another (ECDSA's DER signatures; the simulation's
identities are now Ed25519, focal-bus's recorded change 0005), and a token a server's log refused
in one run and took in the other (hyper-quic's token log refused a valid token issued, by the wall
clock, before the first it checked: a bug, fixed in hyper-raft's recorded change 0006).
`determinism-evidence.txt` holds the runs that showed it: seed 8 twelve times in one process with
each kind of identity, seed 19's token checks in both passes, and seeds 1 to 400 in the two
partitions with each cause closed or not.
