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

## The comparison (`comparison/`): Q-1.1, the first grid

`bench/compare.sh` at 88409dd, 09:18 to 11:19:49 PDT: the wake-up's floor, Quiver, NATS 2.12, Redis
8.2 and Kafka 4.1.0 at 10,000 and 100,000 messages a second to 1, 100 and 1,000 subscribers, waiting
and polling, three rounds, 30 s windows after 5 s of warm-up, 256 B payloads. The brokers in Docker
on CPUs 8-15 (P-cores 4 to 7, both threads each), on loopback; the generator (the publisher, the
subscribers, Quiver's node, the floor) on the other 24. `report.md` is the bench's report (each
system's best of 3 by p99, over every run), `<system>.md` every run with its profile, `runs.tsv`
their lines; the brokers' saved profiles (91 MB: Kafka's Flight Recordings, NATS's pprof, Redis's
allocator) stay on fedora in `~/.cache/quiver-bench/quiet-1024-0915/comparison/profiles`.

- **The profile's check passed** (`plant.md`): a planted allocation shows as one a delivery, on the
  subscribers' threads, for Quiver and the floor.
- **Interleaved**: each round of a load ran every system back to back, their order turned by one
  each round, so every system's runs of a load come from the same minutes; the widest round of a
  load took 3 min 58 s, and no system's cells come from a different hour than another's.
- **The window was shared.** Ada's word (2026-10-24): the comparison runs beside whatever starts,
  the overlaps recorded, the cells they touched marked, and those whose verdict rests on them run
  again. Other sessions' work overlapped 131 of the 180 runs: kiln CI's UI checks (09:35-09:45),
  a kiln build (09:54-10:20), Focal's builds and its cold-stream bench (10:05-10:52), Mantle's gate
  and Amphora's engine build (10:07-10:20), then Amphora's tests and others' clippy to the end
  (`overlaps.tsv` before 10:20; after it `busy.log`, a snapshot a minute of every process over 60%
  of a CPU that wasn't the comparison's). 41 runs overlapped nothing known; 8 fell in minutes whose
  snapshot didn't answer. `clean.md` (`compare-clean.py`) judges every cell twice, over every run
  and over the clean runs alone.

| subscribers' threads | rate /s | subscribers | the floor, p99 / p99.9 µs | Quiver | the best competitor | Quiver's lead, p99 / p99.9 | an order of magnitude | over clean runs alone | rests on runs not known clean |
|---|---|---|---|---|---|---|---|---|---|
| polling | 10,000 | 1 | 0.2 / 2.1 | 1.1 / 3.9 | Redis 63.5 / 258.0 | 59.3× / 66.1× | yes | no clean competitor run | yes |
| polling | 10,000 | 100 | 0.6 / 3.4 | 3.2 / 15.6 | NATS 909.3 / 2162.7 | 281.4× / 138.5× | yes | no clean Quiver run | yes |
| polling | 10,000 | 1,000 | 0.6 / 52.7 | 14.1 / 167.9 | none delivered every message | ∞ | yes | no clean Quiver run | yes |
| polling | 100,000 | 1 | 0.3 / 2.2 | 0.8 / 6.7 | Redis 26.4 / 157.7 | 31.4× / 23.5× | yes | the same | no |
| polling | 100,000 | 100 | 0.6 / 3.9 | 7.0 / 20.0 | none delivered every message | ∞ | yes | ∞ | no |
| polling | 100,000 | 1,000 | 0.7 / 1359.9 | 176.1 / 260.1 | none delivered every message | ∞ | yes | no clean runs | yes |
| waiting | 10,000 | 1 | 20.5 / 69.6 | 21.2 / 69.6 | NATS 101.4 / 149.5 | 4.8× / 2.1× | **no** | the same | no |
| waiting | 10,000 | 100 | 33.3 / 69.6 | 42.0 / 99.3 | NATS 663.6 / 1065.0 | 15.8× / 10.7× | yes | the same | no |
| waiting | 10,000 | 1,000 | 33.3 / 67.6 | 49.2 / 141.3 | none delivered every message | ∞ | yes | ∞ | no |
| waiting | 100,000 | 1 | 5.6 / 60.9 | 6.9 / 53.8 | Redis 36.4 / 176.1 | 5.3× / 3.3× | **no** | the same | no |
| waiting | 100,000 | 100 | none delivered every message | 75.8 / 123.9 | none delivered every message | ∞ | yes | ∞ | no |
| waiting | 100,000 | 1,000 | none delivered every message | 128.0 / 208.9 | none delivered every message | ∞ | yes | no clean runs | yes |

Where no competitor delivered every message, the best fell far short of the load (the report's
deliveries table): at 10,000 a second to 1,000 subscribers, 10 million deliveries a second offered,
Kafka made 9.26 million polling and NATS 2.06 million waiting; at 100,000 to 100, NATS 1.46 million
and Kafka 2.84 million of 10 million, polling; at 100,000 to 1,000, 100 million offered, Kafka 4.41 million
polling and none of them any waiting. Quiver delivered every message of every run in every cell.
The floor itself fails at 100,000 a second to 100 and 1,000 waiting subscribers: its publisher wakes
each waiter thread for every message, more wake-ups than one thread makes.

- **Where subscribers poll** (the step's bar): every load at most a tenth of the best competitor's,
  at p99 and p99.9, over every run: met. Four of the six cells rest on runs no one has shown clean
  (10,000 a second to 1, 100 and 1,000, and 100,000 to 1,000), so they run again.
- **Where subscribers wait**: four of six met; the waiting cell at 100,000 to 1,000 rests on
  overlapped runs and runs again. **Two missed, both one subscriber**: 4.8× / 2.1× over NATS at
  10,000 a second, 5.3× / 3.3× over Redis at 100,000. Their runs, and the competitors' they're
  judged against, are clean.
- **The misses' cause is the floor, measured beside them.** The floor is the kernel's wake-up with
  no system around it: the same open loop, its publisher waking a waiter thread (`Thread::unpark`,
  the futex Quiver's waits come down to), in the same minutes. At 10,000 a second it took 20.5 µs at
  p99 and 69.6 µs at p99.9; Quiver took 21.2 and 69.6, 0.7 µs over it at p99 and nothing at p99.9.
  An order of magnitude under NATS's 101.4 / 149.5 µs is 10.1 / 15.0 µs, under the floor: no
  subscriber the kernel wakes goes there, NATS itself is 4.9× / 2.1× the floor. At 100,000 a second
  the floor took 5.6 / 60.9 µs and Quiver 6.9 / 53.8; a tenth of Redis's is 3.6 / 17.6 µs, under the
  floor again. The floor is the CPU's way out of idle: fedora's idle states (intel_idle from ACPI's
  `_CST`, the menu governor; `/sys/devices/system/cpu/cpu0/cpuidle`) exit in 0 µs (POLL), 1 µs (C1,
  1 µs target residency), 127 µs (C2, 381 µs) and 1,048 µs (C3, 3,144 µs), as the kernel states
  them. At 10,000 a second the gap before a message averages 100 µs and 2.2% of the Poisson gaps
  outlast C2's target residency, more than the 1% past the p99: the waiting subscriber's CPU has
  gone idle, now and then deeply, when its message comes. At 100,000 a second (10 µs gaps) none
  do, and the floor's p99 falls from 20.5 to 5.6 µs. (Which states the CPUs entered in each run
  isn't counted here: the floor's own latency, measured beside Quiver, is the figure.)
- **Ada's decision on the sleeping-subscriber cells** (2026-10-24, 14:30 PDT), between polling
  subscribers where latency matters and the idle states held shallow for the run (root): **polling
  where latency matters.** The two cells stand as misses bound by the kernel's wake-up, with the
  floor's figures above, and Quiver's guidance is that a subscriber that needs Q-1.1's order of
  magnitude polls, as every polling cell met it (docs/design/topics.md). fedora's idle states stay
  as they are.
- **The deployment.** Quiver ran in the generator's process, a library, as kiln runs it; each
  competitor ran as a server over loopback TCP, reached by its own client. Every competitor's
  figure includes a hop to its server and on to the subscriber, and the server's own wake-up;
  Quiver's includes neither. This grid doesn't separate the hop from the brokers' work: the floor
  over TCP (`--system tcp`, a relay that only forwards, added after it) measures the hop alone,
  and the grid runs again with it, so the lead splits into the deployment's part and the broker's
  in the same minutes.
