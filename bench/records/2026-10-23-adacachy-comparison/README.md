# The comparison on adacachy, 2026-10-23: a loaded machine's, not a record

`bench/compare.sh` at d8e5d54 (the interleaved runner), 15:10 to 17:15 PDT, on adacachy (AMD Ryzen 9
9950X3D2, 16 cores, 32 threads, 128 GiB, Linux 7.2.9-2-cachyos) beside kiln's production, as Ada
asked ("Use adacachy"): the brokers on cores 4 to 7 (CPUs 4-7, 20-23), the generator and Quiver on
cores 8 to 15 (CPUs 8-15, 24-31), cores 0 to 3 left to everything else. Every load (10,000 and
100,000 messages a second, 1, 100 and 1,000 subscribers, subscribers waiting and polling), every
system (the wake-up's floor, Quiver, NATS, Redis, Kafka), three rounds, each round's systems back to
back load by load with their order turned each round; 30 s windows after 5 s of warm-up, 256 B
payloads, seeds 0x5eed to 0x5eef. 180 runs, all completed, none failed.

**The load average over the runs was 20.9 to 142.5, 60.7 on average** (360 readings, each run's
before and after): CLAUDE.md §7 asks under 2. The floor shows what that did: a polling waiter's p99
was 75.8 µs at 100,000 messages a second to one subscriber, where a quiet core's is about 1 µs.
Every tail here carries the machine's load, so this measures no claim against Q-1.1; it says where
to look, under that load:

- Quiver led the best competitor by an order of magnitude at p99 and p99.9 in 8 of the 12 loads:
  every load of 100 or 1,000 subscribers but one (`wait`, 10,000 a second, 100 subscribers: 10.6×
  at p99, 1.7× at p99.9), and one subscriber polling at 10,000 a second (37.4× and 85.1×).
- It didn't with one subscriber polling at 100,000 a second (4.1× at p99) or waiting at either
  rate (3.0× and 2.4×): a waiting subscriber is bound by the kernel's wake-up, which the floor
  beside it shows was itself 170 µs and 754 µs at p99 here; the polling one's p99 of 479 µs against
  the floor's 75.8 µs is Quiver's own to find.
- At 100,000 a second to 1,000 subscribers NATS and Redis delivered none of the window's messages
  within their 30 s of catch-up, and Kafka 1.3 to 1.5 million a second, against 100 million
  offered; at 10,000 a second, NATS 2.3 to 2.5 million, Redis 1.5 to 1.7 million and Kafka 1.9 to
  8.8 million of 10 million (the second table: a run's deliveries of its window's messages over
  the window, against the rate times the subscribers; a Poisson schedule's count runs a little
  under or over the rate).
- Every one of Quiver's 36 runs delivered every message of its window to every subscriber: none
  undelivered, none lost silently, every subscriber kept up (`runs.tsv`).

`report.md` is `fanout report runs.tsv`; each system's `.md` is every run as the harness printed it.
These runs predate the profile (`bench/README.md`, "The profile").
