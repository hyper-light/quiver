# The ephemeral hot path allocation-free: fedora, 2026-10-23

kiln's CP-T2.1 step plan, step 2. `bench/budget.sh` at 1790a7c from a clean clone on fedora (Intel
i9-14900HX, Linux 6.19.10), 20:01 to 20:03 PDT, at a load average of 16 to 24 (other sessions'
builds and Mantle's runs beside it, so `LOAD_LIMIT` was raised: the figures below don't move with
load, the latencies do): Quiver at 100,000 messages a second to 1,000 subscribers on 22 threads,
waiting and polling, each a 60 s window after 5 s of warm-up, held to `bench/budgets.tsv`. Both
met every budget (`budget.md`, every run as the bench printed it; `runs.tsv`).

| | waiting | polling |
|---|---|---|
| deliveries in the window | 5,999,684,000 | 5,999,684,000 |
| lag notices, lost, subscribers that kept up | 0, 0, 1,000 of 1,000 | 0, 0, 1,000 of 1,000 |
| allocations and reallocations a delivery, the subscribers' threads | 0 and 0 | 0 and 0 |
| allocations and reallocations a message, the publisher's thread | 2.010 and 0 | 2.010 and 0 |
| minor page faults a million messages | 5.8 | 5.8 |
| last-level cache read misses a delivery, the subscribers' threads | 0.000168 | |
| CPU a delivery, the subscribers' threads | 106.2 ns | |

One subscriber at 100,000 messages a second, 30 s windows (`quiver-1-wait.md`, `quiver-1-busy.md`;
from fedora's synced copy at 88409dd, clean, at 20:15 PDT at a load average of 2.5 to 3.6, their
`commit:` line the script's label): no allocation or reallocation on the subscriber's thread
either, a publish's 2.010 allocations, 2 minor page faults a million messages; and the tail of a
machine near quiet: p50, p99 and p99.9 of 0.4, 1.8 and 9.9 µs polling, 4.0, 7.2 and 62 µs waiting.

Where it started (`../2026-10-23-fedora-profile/`): a waiting subscriber allocated once a wait,
a publish allocated 4.01 times and reallocated 3.00 times, and 51,000 minor page faults came a
million messages (with retention still filling in that record's 2 s warm-up).

## Against main (A/B, the same minutes)

`main` (99abda1, `event_listener`'s waits) and the change, alternating on fedora, Quiver at 100,000
messages a second to 100 waiting subscribers, two rounds each, 19:44 PDT (fedora at load 9 to 14):

| | p50 µs | p99 µs | p99.9 µs | wake-ups a message | the subscribers' allocations a delivery |
|---|---|---|---|---|---|
| main, rounds 1 and 2 | 26.6, 27.9 | 137.2, 159.7 | 950.3, 1507.3 | 9.80, 9.78 | 0.062, 0.061 |
| the change, rounds 1 and 2 | 16.3, 16.9 | 127.0, 135.2 | 925.7, 1245.2 | 9.73, 9.54 | 0, 0 |

Two earlier versions of the change lost that A/B, and why is in the bell's comment
(`crates/quiver/src/ephemeral.rs`): waking its waiters outside its lock, the waiters woke 14.2 times
a message and the publisher fell behind its schedule (152,000 to 392,000 messages never offered in
10 s). The same A/B at 1,000 waiting subscribers (two rounds each, load 22 with Mantle's runs at
400% CPU) was noise for both: main's own p99 was 223 µs one round and 7.5 ms the next. It's for a
quiet machine, step 3.
