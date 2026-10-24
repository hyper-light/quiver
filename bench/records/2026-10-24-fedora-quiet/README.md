# The quiet window on fedora, 2026-10-24: AC-T2.1.2 and AC-T2.1.3 on a quiet machine

kiln's CP-T2.1, step 3 (and step 4's comparison, below as it lands). Quiver at 88409dd (its main,
the commit kiln vendors), on fedora (Intel i9-14900HX, Linux 6.19.10-300.fc44, 64 GB), in a window
Ada gave it with nothing else on the machine: Hex's trials held, the window's unit
`quiver-bench-window`, which Hex's own quiet checks see as a bench. The bench on CPUs 0-7 and 16-31
(24 the process may use). Each load waited until the 1-minute load had been under 2 for four readings
15 s apart and began at a load of 0.5 to 1.1; the load within a load's three runs is the bench's own
(up to 20 with 1,000 subscribers polling on 22 threads).

## The fan-out bench (`fanout/`)

100,000 messages a second on a Poisson schedule, 256 B payloads, one topic keeping 65,536 messages,
30 s windows after 5 s of warm-up, three runs of each load, every run held to `bench/budgets.tsv`
(each met every budget). `quiver-<subscribers>-<wait|busy>.md` is each load as the bench printed it,
every run's profile under it; `runs.tsv` their lines.

| subscribers | subscribers' threads | runs: p99 us | best of 3 by p99: p50 / p99 / p99.9 us | undelivered | lost silently | kept up | the bar (p99 <= 300 us at 1,000) |
|---|---|---|---|---|---|---|---|
| 1 | polling | 1.7, 1.7, 1.7 | 0.4 / 1.7 / 8.1 | 0 | 0 | 1, 1, 1 |  |
| 1 | waiting | 6.7, 6.7, 6.7 | 3.9 / 6.7 / 56.3 | 0 | 0 | 1, 1, 1 |  |
| 100 | polling | 3.8, 4.5, 4.2 | 0.8 / 3.8 / 14.2 | 0 | 0 | 100, 100, 100 |  |
| 100 | waiting | 78.8, 71.7, 78.8 | 12.5 / 71.7 / 119.8 | 0 | 0 | 100, 100, 100 |  |
| 1000 | polling | 133.1, 137.2, 174.1 | 2.4 / 133.1 / 210.9 | 0 | 0 | 1000, 1000, 1000 | **met** |
| 1000 | waiting | 122.9, 126.0, 118.8 | 10.1 / 118.8 / 188.4 | 0 | 0 | 1000, 1000, 1000 | **met** |

- **AC-T2.1.2, met**: publish-to-deliver p99 at 100,000 messages a second to 1,000 subscribers,
  best of 3, 133.1 µs polling and 118.8 µs waiting, under the 300 µs bar, every run of both under
  it too (at most 174.1 µs).
- **AC-T2.1.3, met**: 1,000 subscribers fanned out to with that p99 kept.
- **Nothing lost**: every run delivered every message of its window to every subscriber; none
  undelivered, none lost silently, no lag notice, every subscriber kept up. (Waiting's first run
  left 2 of 3.5 million messages never offered: the publisher two messages behind its schedule.)
- **The profile** (each run's, in its `.md`): the subscribers' threads allocated and reallocated
  nothing; a publish allocated 2.010 times; 16 minor page faults in a 30 s window of 3 million
  messages; the subscribers' last-level cache read misses 0.0000185 a delivery waiting and 0.00000185
  polling; a waiting subscriber's CPU 104.5 ns a delivery (the budget's 106.2).
- **AC-T2.1.4** (the hour's soak, no frame lost) stands from `../2026-10-23-adacachy-soak-88409dd/`
  at this commit: 359,979,959,000 deliveries, none lost, doubled or undelivered.
