# The profile's first record: fedora, 2026-10-23, every system profiled

`bench/compare.sh` at 20e1c94 from a clean clone, 19:11 to 19:17 PDT, on fedora (Intel i9-14900HX:
8 P-cores, CPUs 0-15, and 16 E-cores, CPUs 16-31; 64 GiB; Linux 6.19.10), at `compare.sh`'s fedora
placement (each broker on CPUs 8-15, the generator and Quiver on CPUs 0-7 and 16-31): 10,000
messages a second to 10 and to 100 subscribers, waiting and polling, every system once, 10 s
windows after 2 s of warm-up, 256 B payloads. 20 runs, all completed.

**What it shows is the profile, not latency**: the load average rose from 13.6 to 35.3 through it
(other sessions' work), and one run a load is no best of three. Each system's `.md` has every run
with its profile table under it (`Run 1's profile:`); `report.md`'s third table sets them side by
side, load by load, per delivery, this process (Quiver, or a broker's clients) beside the broker.

- **The check passed** (`plant.md`): the build that plants one allocation in every delivery showed
  +1.0000 allocations and frees a delivery against the bench as built, for Quiver and the floor
  alike, every one on the subscribers' threads.
- **A refused counter is shown refused**: the L1 data cache's read misses, which fedora's E-cores'
  PMU (`cpu_atom`) has no event for, on every thread that may run there: `refused: cpu_atom's PMU
  has no such event (No such file or directory (os error 2))`, in each record's notes under its
  table; never a zero.
- **Each broker's own profile** was read and saved beside the run: NATS's Go `MemStats` (its
  allocations, frees, bytes, collections and their pauses) with its window's allocation profile
  and its heap profile (`profiles/*.pb.gz`, for `go tool pprof`); Redis's jemalloc requests, with
  `MEMORY STATS` (`profiles/*memory-stats.txt`); Kafka's JVM through a Flight Recorder recording
  (its threads' allocated bytes, its collections and pauses, its heap). The recordings
  (`kafka-*.jfr`, 1.2 to 1.4 MB each) and Redis's allocator statistics (`*malloc-stats.txt`, about
  60 kB each) stay on fedora in `~/.cache/quiver-bench/profile-record-1911/comparison/profiles/`.

Where Quiver stands, the next step's starting point (`quiver.md`):

- Its publish allocates 4.010 times and reallocates 3.000 times a message (2.27 of them moving
  their block) on the publisher's thread, at every load: the record it frames, a batch builder
  grown from nothing, the batch's own box.
- Its take allocates nothing: polling subscribers' threads made 0 allocations in 998,040 and
  9,980,822 deliveries. A waiting subscriber allocates once each time it waits (0.993 a delivery
  with 10 subscribers on 10 threads, 0.171 with 100 on 22): the listener `poll_ready` takes.
- The process took 0.051 minor page faults a message (51,000 a million; the next step's budget is
  100), all on the publisher's thread; the node held 317 bytes a 256 B message at its peak.
