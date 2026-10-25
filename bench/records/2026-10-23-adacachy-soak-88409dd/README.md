# The hour's soak on adacachy at 88409dd, 2026-10-23: no frame lost, step 2's hot path

kiln's CP-T2.1, AC-T2.1.4. `fanout soak` at 88409dd (Quiver's main: step 2's allocation-free take
path, a publish's allocations halved, the budgets), 22:06 to 23:07 PDT, on adacachy (AMD Ryzen 9
9950X3D2, Linux 7.2.9-2-cachyos) beside kiln's production, as the soak before it ran
(`../2026-10-23-adacachy-soak/`, at 08d3dc9): Quiver alone on cores 8 to 15 (CPUs 8-15, 24-31), a
watchdog on production's `/healthz` (it never failed). 100,000 messages a second on a Poisson
schedule to 1,000 subscribers on 14 threads, waiting, the topic keeping a million messages, a
3,600 s window after 5 s of warm-up, seed 0x5eed: the same schedule, message for message, as that
soak's. `soak.md` is the run as the bench printed it, its profile under it; `soak.tsv` its line.

- **Every message delivered to every subscriber, none lost**: 360,529,149 published (none held
  back, no publish refused), 359,979,959,000 deliveries of the window's messages measured, none
  undelivered, no frame lost, none doubled, no lag notice: all 1,000 subscribers kept up for the
  hour.
- **Not a latency record**: the load average was 39.1 as it began and 61.1 as it ended (soak.md's row). Its tail,
  p50 33.8 µs, p99 19.4 ms, p99.9 241 ms, carries that.
- **Step 2's changes held for the hour**: the subscribers' threads allocated nothing and
  reallocated nothing in 360 billion deliveries; a publish allocated 2.010 times (the boxed future
  `Topics::publish` answers with and its batch's block), with one reallocation in the hour. At
  08d3dc9 a publish allocated 4.01 times and reallocated 3.00 times, and a waiting subscriber
  allocated once a wait.
- **Page faults**: 56,870 minor in the hour, 158 a million messages (step 2's 60 s windows on
  fedora: 5.8), and 479 major, adacachy swapping (46 MB swapped as it began); where in the hour
  they fell isn't measured, and the topic filled to its million messages in the window's first 5 s.
- The node held 316 bytes a 256 B message at its peak, a million messages kept, as before; the
  last-level cache's read misses refused on Zen, as there.
