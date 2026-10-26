# hyper-raft's shared crates, vendored

The crates Focal's bus is built on (`vendor/focal-bus`), at hyper-raft's `main`.

- **Upstream:** https://github.com/hyper-light/hyper-raft, commit
  `25cf88924895ee55976caa061a48789355fb3354` (2026-10-22), MIT (`LICENSE`): hyper-raft's `main` since
  its `main` and `fate-commit` lines met (the merge `7d5e536`), and the revision Focal's
  `bus-replicated` at `f8052a5` takes its shared crates at (Focal's `vendor/README.md`): Mantle takes
  it too, so kiln holds one.
- **What is here:** upstream's root manifest and the fifteen crates, whole, at upstream's paths:
  `hyper-block`, `hyper-datagram`, `hyper-durable`, `hyper-liveness`, `hyper-log`, `hyper-measure`,
  `hyper-quic`, `hyper-raft`, `hyper-rt`, `hyper-rt-transport`, `hyper-seal`, `hyper-sim`,
  `hyper-timing`, `hyper-tls`, `hyper-transport`.
- **How it's built:** Quiver's root `Cargo.toml` points the names Focal's bus takes them by at these
  folders (`[patch.crates-io]`); kiln, vendoring Quiver, holds the same revision with the same
  changes, its own crates built on it too.

## Changes from upstream

Each change is recorded in `vendor/patches/hyper-raft/`, applied in this order on upstream's tree.

| File | Change | Why | Recorded as |
|---|---|---|---|
| `crates/hyper-durable/src/lib.rs`, `crates/hyper-durable/Cargo.toml` | `RamStore` compiled only for the crate's own tests or under the `test-store` feature, which a dev-dependency asks for (the crate's own tests ask through one on itself) | the write-ahead log is mandatory in Quiver and kiln: `GroupStore` on a real device, `RamStore` test-only, and a build that names it outside a test fails (kiln's CP-T2.1, T-T2.1.2; Quiver's `crates/quiver/tests/ram_store.rs`) | `0001-ramstore-for-tests-alone-the-write-ahead-log-mandatory.patch` |
| `crates/hyper-log` (`lib.rs`, `group.rs`, `device.rs`, `owner/`, `tests/told.rs`, `tests/log.rs`); `crates/hyper-rt` (`sim.rs`, `udp/mod.rs`, `tests/udp.rs`); `crates/hyper-rt-transport` (`driver.rs`, `lib.rs`, `tests/simulation.rs`); `crates/hyper-raft` (`mutant.rs`, `raft.rs`, `tests/check.rs`); `crates/hyper-durable` (`replica.rs`, `Cargo.toml`) | The simulation's seams. hyper-log's `Waits::Told`: a frame only at a cut (`Log::cut`), its confirmation made one way, a waking fetch only at a serve (`Log::serve`); its restore at open and a caller that waits on its write or fetch tell it by waiting. hyper-rt's `sim::anchor` and hyper-rt-transport's `shard_instant`: one clock for every driver on a simulated shard. A simulated bind takes the port asked for in its shard. `Mutant::OwnLogCommit`, reached through `Replica::plant` (`mutants` features) | Quiver's persisted class runs in a deterministic simulation (kiln's CP-T2.2 step 6, `docs/design/persisted-sim.md`). No production build takes these paths: told and serve need `Waits::Told`, the anchor and binds a simulated shard, the mutant a feature no consumer enables. Offered upstream as hyper-raft's local `cp/t2.2-told` | `0002-the-simulation-s-seams-a-told-log-one-clock-on-simulated.patch` |

The first change is Quiver's and kiln's policy, not hyper-raft's, so it isn't offered upstream: hyper-durable
calls `RamStore` slates' case (each write complete as it's submitted), and gating it in hyper-raft
would reach slates. Focal's agent agrees (2026-10-22), and notes that Focal names `RamStore` only in a
test module (`focal-consensus`'s `shell_tests.rs`): sharing this copy, it would ask for `test-store`
as a dev-dependency, as kiln's sync tests do.

## Taking a newer upstream

`python3 scripts/vendor.py hyper-raft <commit>` takes these crates at that commit and re-applies the
change above by three-way merge; take it at the revision Focal's bus takes.
