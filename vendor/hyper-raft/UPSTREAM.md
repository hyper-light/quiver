# hyper-raft's shared crates, vendored

The crates Focal's bus, Quiver and Mantle are built on (`vendor/focal-bus`, `vendor/quiver`,
`vendor/mantle`), at one revision of hyper-raft for all of them.

- **Upstream:** https://github.com/hyper-light/hyper-raft, commit
  `fb74b59533908472ade7e526eb08117601cede0e` (2026-11-07), MIT (`LICENSE`): hyper-raft's `main`, which
  holds what Mantle's `main` at `87ab09e1` snapshots its hyper-* crates at (`5e40eb9`: `0a182ae` and the
  driver fix `main` landed as `fb74b59`, a driver reading what its sockets took before it judges its
  timers, upstream's docs/transport.md §4ap), taken for kiln's refresh of Mantle so kiln holds one copy
  (CP-M1). Before it, `main` at `f45fc56` and `8203466` (2026-11-07, a member's catch-up read off the
  log's device and kept a run ahead, for kiln's API runtime program), and `0856e05` (2026-11-02), the
  merge of hyper-raft's `main` (`22d5037`, kiln's CP-T2.2 line) and Mantle's
  `progress-notes-reachable` (`0a182ae`, what Mantle's `main` at `17eb8032` snapshots), made for kiln's
  CP-T10.2; before that, `25cf889` (2026-10-22), `main` since its `main` and `fate-commit` lines met
  (the merge `7d5e536`), which Focal's `bus-replicated` at `f8052a5` took.
- **What is here:** upstream's root manifest and the fifteen crates, whole, at upstream's paths:
  `hyper-block`, `hyper-datagram`, `hyper-durable`, `hyper-liveness`, `hyper-log`, `hyper-measure`,
  `hyper-quic`, `hyper-raft`, `hyper-rt`, `hyper-rt-transport`, `hyper-seal`, `hyper-sim`,
  `hyper-timing`, `hyper-tls`, `hyper-transport` (kiln's copy takes Mantle's `hyper-swim` too).
- **How it's built:** Quiver's root `Cargo.toml` points the names Focal's bus takes them by at these
  folders (`[patch.crates-io]`); kiln, vendoring Quiver and Mantle, holds the same revision with the
  same changes, its own crates built on it too.

## Changes from upstream

Each change is recorded in `vendor/patches/hyper-raft/`, applied in this order on upstream's tree.

| File | Change | Why | Recorded as |
|---|---|---|---|
| `crates/hyper-durable/src/lib.rs`, `crates/hyper-durable/Cargo.toml` | `RamStore` compiled only for the crate's own tests or under the `test-store` feature, which a dev-dependency asks for (the crate's own tests ask through one on itself) | the write-ahead log is mandatory in Quiver and kiln: `GroupStore` on a real device, `RamStore` test-only, and a build that names it outside a test fails (kiln's CP-T2.1, T-T2.1.2; Quiver's `crates/quiver/tests/ram_store.rs`) | `0001-ramstore-for-tests-alone-the-write-ahead-log-mandatory.patch` |
| `crates/hyper-rt-transport/src/driver.rs`, `crates/hyper-rt-transport/tests/received.rs` | `Driver::poll_received` and `Driver::received`: `poll_event`'s turn split at its seam, an event the turn's receive half made handed out before the turn's sends, which the caller then makes with `flush` in the same turn (the method's doc says what it owes, and what holding it costs); `poll_event` built from the same pieces (`receive`, the receive half, given the turn's context and whether it's the poll's last, as upstream's turn takes them since `fb74b59`; `turns`, the loop), doing what it did; a test of both | kiln's (its net thread hands what comes in to its shards before the turn's sends: kiln's goal 10, plan 01a1a87b2dae step 5), recorded here as kiln records it, so the one copy kiln holds is Quiver's; nothing in Quiver or Focal's bus calls it | `0003-driver-poll-received-a-turn-s-events-before-its-sends.patch` |
| `crates/hyper-tls/src/conn.rs` | `ConnectionCommon::set_max_fragment_size`: the largest record a connection sends, set as it goes (header and all; `None`, TLS's own), as the configuration's `max_fragment_size` sets it when the connection is made | kiln's browser protocol socket keeps each TLS record at most its chunk, which follows the path's rate (kiln's goal 3, plan 01a1a832eb2f711f9c6f51d88effe89d step 3): the configuration's is fixed when the connection is made, before the server knows the connection carries that socket or what its chunk is, and a 16 KiB record holds an urgent frame 2.6 s at 50 kbit/s | `0004-hyper-tls-a-connection-s-largest-record-set-as-it-goes.patch` |

The first change is Quiver's and kiln's policy, not hyper-raft's, so it isn't offered upstream: hyper-durable
calls `RamStore` slates' case (each write complete as it's submitted), and gating it in hyper-raft
would reach slates. Focal's agent agrees (2026-10-22), and notes that Focal names `RamStore` only in a
test module (`focal-consensus`'s `shell_tests.rs`): sharing this copy, it would ask for `test-store`
as a dev-dependency, as kiln's sync tests do.

The change numbered 0003 is kiln's, under kiln's number (its 0002 is kiln's layout alone), offered
upstream as it stands: it adds a method and changes no other, and Quiver and Focal's bus call `event`
and `poll_event`, whose turns are the same.

An earlier change, IOCP's widened AFD poll issued only once the old one's cancel completes (Jupiter,
2026-10-24), was a fix of hyper-raft's own: hyper-raft carries it since `53a28a0` (with its test,
`392407f`), so the re-vendor at `0856e05` dropped it.

## Taking a newer upstream

`python3 scripts/vendor.py hyper-raft <commit>` takes these crates at that commit and re-applies the
changes above by three-way merge; take it at the revision Quiver and Mantle take, which move with it.
