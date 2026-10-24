# The hour's soak on adacachy, 2026-10-23: no frame lost to a subscriber that keeps up

kiln's CP-T2.1, AC-T2.1.4. `fanout soak` at 08d3dc9 (the profile's first commit, before step 2's
changes to the ephemeral class), 19:08 to 20:09 PDT, on adacachy (AMD Ryzen 9 9950X3D2, Linux
7.2.9-2-cachyos) beside kiln's production, as Ada asked: Quiver alone on cores 8 to 15 (CPUs 8-15,
24-31), a watchdog on production's `/healthz` (it never failed). 100,000 messages a second on a
Poisson schedule to 1,000 subscribers on 14 threads, waiting, the topic keeping a million messages,
a 3,600 s window after 5 s of warm-up. `soak.md` is the run as the bench printed it, its profile
under it; `soak.tsv` its line.

- **Every message delivered to every subscriber, none lost**: 360,529,149 published (none held
  back, no publish refused), 359,979,959,000 deliveries of the window's messages measured, none
  undelivered, no frame lost, none doubled, no lag notice: all 1,000 subscribers kept up for the
  hour.
- **Not a latency record**: the load average was 35.9 as it began and 61.6 as it ended. Its tail,
  p50 17.2 µs, p99 8.7 ms, p99.9 75 ms, carries that.
- **The profile on AMD** (the first record's was Intel's): cycles, instructions, cache references
  and misses, L1 data and data-TLB read misses all counted on the publisher's and the subscribers'
  threads; the last-level cache's read misses refused, as the profile says them,
  `refused: cpu's PMU has no such event`: Zen's L3 is outside the core's PMU. The node held 316 bytes
  a 256 B message at its peak, a million messages kept.
- What step 2 then took out shows here too: a publish's 4.01 allocations and 3.00 reallocations a
  message, a waiting subscriber's allocation a wait (0.0031 a delivery), and 155 minor page faults a
  million messages (`../2026-10-23-fedora-step2/`).
