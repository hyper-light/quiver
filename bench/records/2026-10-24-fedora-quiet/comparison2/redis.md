- broker: public.ecr.aws/docker/library/redis:8.2-alpine@sha256:b51665e66f00759be7c3152ad5ac3c66fb2f619c13ef62dea7cc1f9914524635, CPUs 8-15
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 20.2 | 48.1 | 107.5 | 229.4 | 2488.0 | 0 | 0 | 1/1 | 37.1 | 34.5 | 2.0 | 2.02 / 1.98 |
| best | redis | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 20.2 | 48.1 | 107.5 | 229.4 | 2488.0 | 0 | 0 | 1/1 | 37.1 | 34.5 | 2.0 | 2.02 / 1.98 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 299499 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 31.32G | 104.6k | 104.6k | 97.48k | 7104 | 1.727G | 5671 | 5671 |
| waiting for a CPU, ns | - | - | - | 9.332 | 14.45 | - | - | - |
| allocations | 3.899M | 13.02 | 13.02 | 7.984 | 5.036 | 913.9k | 3.000 | 3.000 |
| reallocations | 610.5k | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 610.5k | 2.038 | 2.038 | 2.000 | 0.0385 | - | - | - |
| frees | 3.899M | 13.02 | 13.02 | 7.984 | 5.036 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 852.2M | 2846 | 2846 | 2298 | 547.1 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 1.000 | 0.00000334 | 0.00000334 | 0 | 0.00000334 | 37.00 | 0.000121 | 0.000121 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 275.2k | 0.919 | 0.919 | 0 | 0.919 | - | - | - |
| involuntary context switches | 275.0 | 0.000918 | 0.000918 | 0.000765 | 0.000154 | - | - | - |
| cycles | - | - | - | 461.2k | 15.59k | - | - | - |
| instructions | - | - | - | 709.7k | 19.44k | - | - | - |
| cache references | - | - | - | 3931 | 25.49 | - | - | - |
| cache misses | - | - | - | 963.2 | 3.112 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0533 | 0.0585 | - | - | - |
| data TLB read misses | - | - | - | 53.43 | 0.534 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.758M | - | - | - | - | 55.95M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.758M | - | - | - | - | 55.37M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.192M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304610 messages, 304610 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-289159-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-289159-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 745.5 | 1130.5 | 1474.6 | 1753.1 | 3692.7 | 0 | 0 | 100/100 | 114.1 | 34.6 | 17.8 | 10.93 / 9.43 |
| best | redis | 22 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 745.5 | 1130.5 | 1474.6 | 1753.1 | 3692.7 | 0 | 0 | 100/100 | 114.1 | 34.6 | 17.8 | 10.93 / 9.43 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 29949223 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 96.43G | 322.0k | 3220 | 97.73k | 2243 | 15.29G | 50.19k | 501.9 |
| waiting for a CPU, ns | - | - | - | 8.978 | 2.825 | - | - | - |
| allocations | 129.3M | 431.6 | 4.316 | 7.168 | 4.244 | 914.1k | 3.001 | 0.0300 |
| reallocations | 1.751M | 5.846 | 0.0585 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 1.750M | 5.844 | 0.0584 | 1.998 | 0.0385 | - | - | - |
| frees | 129.3M | 431.6 | 4.316 | 7.168 | 4.244 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 16.31G | 54.44k | 544.5 | 2272 | 521.7 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 2.000 | 0.00000668 | 0.000000067 | 0 | 0.000000067 | 89.00 | 0.000292 | 0.00000292 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 4.734M | 15.81 | 0.158 | 0 | 0.158 | - | - | - |
| involuntary context switches | 4647 | 0.0155 | 0.000155 | 0.000621 | 0.000149 | - | - | - |
| cycles | - | - | - | 510.5k | 8057 | - | - | - |
| instructions | - | - | - | 931.8k | 14.02k | - | - | - |
| cache references | - | - | - | 40.85 | 9.004 | - | - | - |
| cache misses | - | - | - | 13.16 | 0.690 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0481 | 0.0161 | - | - | - |
| data TLB read misses | - | - | - | 0.363 | 0.370 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 14.44M | - | - | - | - | 62.06M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 14.44M | - | - | - | - | 59.88M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 2.225M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304639 messages, 30464038 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-294337-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-294337-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 93657 | 2636 | 260933 | 0 | 80007000 | 37153000 | 262344000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 6440000 | 0/1000 | 90.8 | 0.4 | 31.0 | 11.48 / 8.69 |
| best | redis | 22 × wait | 93657 | 2636 | 260933 | 0 | 80007000 | 37153000 | 262344000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 6440000 | 0/1000 | 90.8 | 0.4 | 31.0 | 11.48 / 8.69 |

Run 1's profile:

| profile: the window, 30.0 s, 78316 messages, 66749054 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 75.26G | 961.0k | 1127 | 2953 | 1124 | 26.44G | 331.1k | 388.7 |
| waiting for a CPU, ns | - | - | - | 2.493 | 1.815 | - | - | - |
| allocations | 321.1M | 4100 | 4.811 | 7.017 | 4.803 | 2.675M | 33.49 | 0.0393 |
| reallocations | 3.172M | 40.50 | 0.0475 | 2.000 | 0.0452 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 3.158M | 40.33 | 0.0473 | 1.980 | 0.0450 | - | - | - |
| frees | 321.1M | 4101 | 4.811 | 7.017 | 4.803 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 40.70G | 519.7k | 609.8 | 2268 | 607.1 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 9572 | 0.122 | 0.000143 | 0.000204 | 0.000143 | 1.686M | 21.11 | 0.0248 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 829.6k | 10.59 | 0.0124 | 0.0164 | 0.0124 | - | - | - |
| involuntary context switches | 1752 | 0.0224 | 0.0000262 | 0.0000383 | 0.0000262 | - | - | - |
| cycles | - | - | - | 20.29k | 7909 | - | - | - |
| instructions | - | - | - | 17.92k | 15.63k | - | - | - |
| cache references | - | - | - | 264.4 | 20.39 | - | - | - |
| cache misses | - | - | - | 229.2 | 10.69 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 21.62 | 0.317 | - | - | - |
| data TLB read misses | - | - | - | 9.674 | 0.511 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 107.0M | - | - | - | - | 110.7M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 107.0M | - | - | - | - | 105.1M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.58 s from a quarter second before the window to a quarter second after it, 79863 messages, 68019073 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-303525-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-303525-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 24.8 | 34.3 | 60.4 | 241.7 | 2496.2 | 0 | 0 | 1/1 | 40.7 | 27.7 | 12.6 | 4.63 / 4.18 |
| best | redis | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 24.8 | 34.3 | 60.4 | 241.7 | 2496.2 | 0 | 0 | 1/1 | 40.7 | 27.7 | 12.6 | 4.63 / 4.18 |

Run 1's profile:

| profile: the window, 30.0 s, 2998281 messages, 2998280 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 34.47G | 11.50k | 11.50k | 7809 | 3689 | 10.82G | 3551 | 3551 |
| waiting for a CPU, ns | - | - | - | 0.849 | 6.946 | - | - | - |
| allocations | 37.50M | 12.51 | 12.51 | 7.723 | 4.786 | 9.145M | 3.000 | 3.000 |
| reallocations | 6.112M | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 6.111M | 2.038 | 2.038 | 2.000 | 0.0385 | - | - | - |
| frees | 37.50M | 12.51 | 12.51 | 7.723 | 4.785 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 8.480G | 2828 | 2828 | 2289 | 539.1 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 20.00 | 0.00000667 | 0.00000667 | 0 | 0.00000667 | 49.00 | 0.0000161 | 0.0000161 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.868M | 0.623 | 0.623 | 0 | 0.623 | - | - | - |
| involuntary context switches | 892.0 | 0.000298 | 0.000298 | 0.000147 | 0.000150 | - | - | - |
| cycles | - | - | - | 39.04k | 11.64k | - | - | - |
| instructions | - | - | - | 61.95k | 16.52k | - | - | - |
| cache references | - | - | - | 18.54 | 4.528 | - | - | - |
| cache misses | - | - | - | 3.093 | 0.857 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0198 | 0.0166 | - | - | - |
| data TLB read misses | - | - | - | 0.291 | 0.154 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.025M | - | - | - | - | 58.04M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.025M | - | - | - | - | 58.37M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.243M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3048265 messages, 3048267 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-333433-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-333433-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 934051 | 26252 | 2613492 | 0 | 79912100 | 37181000 | 262646400 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 6287000 | 0/100 | 99.6 | 2.5 | 29.2 | 16.42 / 11.10 |
| best | redis | 22 × wait | 934051 | 26252 | 2613492 | 0 | 79912100 | 37181000 | 262646400 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 6287000 | 0/100 | 99.6 | 2.5 | 29.2 | 16.42 / 11.10 |

Run 1's profile:

| profile: the window, 30.0 s, 791641 messages, 67715100 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 84.30G | 106.5k | 1245 | 2508 | 1216 | 25.02G | 31.10k | 363.6 |
| waiting for a CPU, ns | - | - | - | 5.676 | 2.053 | - | - | - |
| allocations | 329.7M | 416.4 | 4.869 | 7.018 | 4.787 | 4.740M | 5.891 | 0.0689 |
| reallocations | 4.628M | 5.847 | 0.0684 | 2.000 | 0.0450 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 4.611M | 5.825 | 0.0681 | 1.978 | 0.0450 | - | - | - |
| frees | 329.7M | 416.4 | 4.869 | 7.018 | 4.787 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 42.75G | 54.00k | 631.4 | 2268 | 604.8 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 8547 | 0.0108 | 0.000126 | 0.00000253 | 0.000126 | 35.00 | 0.0000435 | 0.000000509 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.162M | 1.468 | 0.0172 | 0.0174 | 0.0170 | - | - | - |
| involuntary context switches | 3857 | 0.00487 | 0.0000570 | 0.0000682 | 0.0000562 | - | - | - |
| cycles | - | - | - | 13.85k | 7274 | - | - | - |
| instructions | - | - | - | 22.73k | 14.76k | - | - | - |
| cache references | - | - | - | 143.1 | 4.473 | - | - | - |
| cache misses | - | - | - | 19.58 | 0.382 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 1.153 | 0.0118 | - | - | - |
| data TLB read misses | - | - | - | 5.668 | 0.157 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 54.21M | - | - | - | - | 61.26M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 54.21M | - | - | - | - | 58.26M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 5.075M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 804564 messages, 68816182 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-365905-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-365905-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 82034 | 2302 | 3465509 | 0 | 70324000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 82.4 | 0.5 | 30.2 | 16.87 / 11.20 |
| best | redis | 22 × wait | 82034 | 2302 | 3465509 | 0 | 70324000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 82.4 | 0.5 | 30.2 | 16.87 / 11.20 |

Run 1's profile:

| profile: the window, 30.0 s, 68715 messages, 58771584 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 68.48G | 996.6k | 1165 | 4404 | 1160 | 25.66G | 366.5k | 428.2 |
| waiting for a CPU, ns | - | - | - | 6.725 | 1.499 | - | - | - |
| allocations | 281.8M | 4101 | 4.794 | 7.018 | 4.786 | 2.302M | 32.88 | 0.0384 |
| reallocations | 2.783M | 40.50 | 0.0474 | 2.000 | 0.0450 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.782M | 40.48 | 0.0473 | 1.991 | 0.0450 | - | - | - |
| frees | 281.8M | 4101 | 4.794 | 7.018 | 4.786 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 35.72G | 519.8k | 607.7 | 2268 | 605.1 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 8894 | 0.129 | 0.000151 | 0.0000291 | 0.000151 | 1.406M | 20.09 | 0.0235 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.145M | 16.67 | 0.0195 | 0.0169 | 0.0195 | - | - | - |
| involuntary context switches | 1558 | 0.0227 | 0.0000265 | 0.0000873 | 0.0000264 | - | - | - |
| cycles | - | - | - | 28.83k | 7640 | - | - | - |
| instructions | - | - | - | 25.12k | 15.51k | - | - | - |
| cache references | - | - | - | 371.9 | 19.69 | - | - | - |
| cache misses | - | - | - | 319.4 | 9.896 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 28.33 | 0.312 | - | - | - |
| data TLB read misses | - | - | - | 12.47 | 0.445 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 101.2M | - | - | - | - | 119.0M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 101.2M | - | - | - | - | 97.53M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.58 s from a quarter second before the window to a quarter second after it, 69998 messages, 59917608 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-428076-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-428076-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 17.2 | 23.0 | 73.7 | 206.8 | 331.0 | 0 | 0 | 1/1 | 69.7 | 34.5 | 2.1 | 3.22 / 2.95 |
| best | redis | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 17.2 | 23.0 | 73.7 | 206.8 | 331.0 | 0 | 0 | 1/1 | 69.7 | 34.5 | 2.1 | 3.22 / 2.95 |

Run 1's profile:

| profile: the window, 30.0 s, 299500 messages, 299500 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 58.82G | 196.4k | 196.4k | 97.45k | 98.96k | 1.827G | 5998 | 5998 |
| waiting for a CPU, ns | - | - | - | 1.382 | 8.215 | - | - | - |
| allocations | 3.905M | 13.04 | 13.04 | 7.980 | 5.057 | 913.9k | 3.000 | 3.000 |
| reallocations | 610.5k | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 610.5k | 2.038 | 2.038 | 2.000 | 0.0385 | - | - | - |
| frees | 3.905M | 13.04 | 13.04 | 7.980 | 5.057 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 852.4M | 2846 | 2846 | 2298 | 547.7 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 0 | 0 | 0 | 0 | 0 | 39.00 | 0.000128 | 0.000128 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 301.0 | 0.00101 | 0.00101 | 0.000157 | 0.000848 | - | - | - |
| cycles | - | - | - | 351.5k | 401.8k | - | - | - |
| instructions | - | - | - | 650.7k | 695.4k | - | - | - |
| cache references | - | - | - | 2.620 | 1.751 | - | - | - |
| cache misses | - | - | - | 0.572 | 0.384 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0494 | 0.0406 | - | - | - |
| data TLB read misses | - | - | - | 0.111 | 0.00941 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.672M | - | - | - | - | 56.22M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.672M | - | - | - | - | 55.14M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.216M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304608 messages, 304608 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-440060-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-440060-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 884.7 | 1359.9 | 1802.2 | 2228.2 | 3704.2 | 0 | 0 | 100/100 | 804.6 | 34.1 | 19.4 | 25.13 / 24.86 |
| best | redis | 22 × busy | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 884.7 | 1359.9 | 1802.2 | 2228.2 | 3704.2 | 0 | 0 | 100/100 | 804.6 | 34.1 | 19.4 | 25.13 / 24.86 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 29949512 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 679.2G | 2.268M | 22.68k | 96.26k | 21.72k | 16.64G | 54.61k | 546.0 |
| waiting for a CPU, ns | - | - | - | 10.08 | 2.183 | - | - | - |
| allocations | 128.5M | 429.1 | 4.291 | 7.143 | 4.219 | 914.2k | 3.001 | 0.0300 |
| reallocations | 1.751M | 5.846 | 0.0585 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 1.749M | 5.840 | 0.0584 | 1.994 | 0.0385 | - | - | - |
| frees | 128.5M | 429.1 | 4.291 | 7.143 | 4.219 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 16.28G | 54.36k | 543.7 | 2272 | 520.9 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | 232.0 | 0.000762 | 0.00000762 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 4948 | 0.0165 | 0.000165 | 0.000661 | 0.000159 | - | - | - |
| cycles | - | - | - | 262.6k | 54.96k | - | - | - |
| instructions | - | - | - | 387.9k | 87.21k | - | - | - |
| cache references | - | - | - | 2.534 | 0.380 | - | - | - |
| cache misses | - | - | - | 0.654 | 0.0825 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0281 | 0.000740 | - | - | - |
| data TLB read misses | - | - | - | 0.385 | 0.0124 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 13.42M | - | - | - | - | 62.05M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 13.42M | - | - | - | - | 58.61M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 2.346M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304643 messages, 30464808 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-445242-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-445242-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 73584 | 2072 | 281006 | 0 | 63104000 | 20120000 | 279377000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3400000 | 0/1000 | 790.4 | 0.4 | 31.0 | 28.66 / 26.21 |
| best | redis | 22 × busy | 73584 | 2072 | 281006 | 0 | 63104000 | 20120000 | 279377000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3400000 | 0/1000 | 790.4 | 0.4 | 31.0 | 28.66 / 26.21 |

Run 1's profile:

| profile: the window, 30.0 s, 61299 messages, 52423922 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 657.5G | 10.73M | 12.54k | 3477 | 12.54k | 26.38G | 421.8k | 493.5 |
| waiting for a CPU, ns | - | - | - | 4.816 | 0.826 | - | - | - |
| allocations | 251.5M | 4102 | 4.796 | 7.016 | 4.788 | 2.164M | 34.59 | 0.0405 |
| reallocations | 2.484M | 40.53 | 0.0474 | 2.000 | 0.0451 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.483M | 40.50 | 0.0474 | 1.987 | 0.0450 | - | - | - |
| frees | 251.5M | 4102 | 4.796 | 7.016 | 4.788 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 31.86G | 519.8k | 607.8 | 2268 | 605.1 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 4706 | 0.0768 | 0.0000898 | 0.0000163 | 0.0000897 | 1.260M | 20.14 | 0.0236 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 941.0 | 0.0154 | 0.0000179 | 0.0154 | 0 | - | - | - |
| involuntary context switches | 2861 | 0.0467 | 0.0000546 | 0.0000163 | 0.0000546 | - | - | - |
| cycles | - | - | - | 12.09k | 32.23k | - | - | - |
| instructions | - | - | - | 10.40k | 53.33k | - | - | - |
| cache references | - | - | - | 151.5 | 8.060 | - | - | - |
| cache misses | - | - | - | 135.4 | 4.475 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 18.35 | 0.0551 | - | - | - |
| data TLB read misses | - | - | - | 5.805 | 0.0187 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 84.08M | - | - | - | - | 120.0M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 84.08M | - | - | - | - | 94.88M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.60 s from a quarter second before the window to a quarter second after it, 62539 messages, 53449133 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-451851-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-451851-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 19.7 | 26.1 | 33.3 | 170.0 | 355.9 | 0 | 0 | 1/1 | 60.4 | 27.7 | 12.3 | 10.91 / 7.33 |
| best | redis | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 19.7 | 26.1 | 33.3 | 170.0 | 355.9 | 0 | 0 | 1/1 | 60.4 | 27.7 | 12.3 | 10.91 / 7.33 |

Run 1's profile:

| profile: the window, 30.0 s, 2998274 messages, 2998273 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 51.01G | 17.01k | 17.01k | 7807 | 9208 | 10.56G | 3464 | 3464 |
| waiting for a CPU, ns | - | - | - | 0.949 | 1.033 | - | - | - |
| allocations | 37.66M | 12.56 | 12.56 | 7.741 | 4.818 | 9.145M | 3.000 | 3.000 |
| reallocations | 6.112M | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 6.104M | 2.036 | 2.036 | 1.997 | 0.0385 | - | - | - |
| frees | 37.66M | 12.56 | 12.56 | 7.741 | 4.818 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 8.487G | 2831 | 2831 | 2291 | 540.1 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 0 | 0 | 0 | 0 | 0 | 48.00 | 0.0000157 | 0.0000157 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 770.0 | 0.000257 | 0.000257 | 0.000147 | 0.000109 | - | - | - |
| cycles | - | - | - | 24.89k | 35.58k | - | - | - |
| instructions | - | - | - | 45.51k | 62.29k | - | - | - |
| cache references | - | - | - | 0.410 | 0.284 | - | - | - |
| cache misses | - | - | - | 0.0754 | 0.0446 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00443 | 0.00323 | - | - | - |
| data TLB read misses | - | - | - | 0.0465 | 0.0237 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.127M | - | - | - | - | 56.72M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.127M | - | - | - | - | 54.88M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.240M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3048269 messages, 3048271 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-462383-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-462383-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 780926 | 21931 | 2766617 | 0 | 66724600 | 24050500 | 275776900 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 4105000 | 0/100 | 782.7 | 1.8 | 30.0 | 25.75 / 24.85 |
| best | redis | 22 × busy | 780926 | 21931 | 2766617 | 0 | 66724600 | 24050500 | 275776900 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 4105000 | 0/100 | 782.7 | 1.8 | 30.0 | 25.75 / 24.85 |

Run 1's profile:

| profile: the window, 30.0 s, 657763 messages, 56184176 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 658.8G | 1.002M | 11.73k | 2198 | 11.70k | 25.75G | 38.50k | 450.7 |
| waiting for a CPU, ns | - | - | - | 3.260 | 0.697 | - | - | - |
| allocations | 273.9M | 416.4 | 4.875 | 7.018 | 4.793 | 3.956M | 5.914 | 0.0692 |
| reallocations | 3.846M | 5.846 | 0.0684 | 2.000 | 0.0450 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 3.830M | 5.822 | 0.0682 | 1.976 | 0.0450 | - | - | - |
| frees | 273.9M | 416.4 | 4.875 | 7.018 | 4.793 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 35.53G | 54.01k | 632.3 | 2268 | 605.8 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 7717 | 0.0117 | 0.000137 | 0.00000304 | 0.000137 | 97.00 | 0.000145 | 0.00000170 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 11.37k | 0.0173 | 0.000202 | 0.0170 | 0.00000317 | - | - | - |
| involuntary context switches | 3148 | 0.00479 | 0.0000560 | 0.0000106 | 0.0000559 | - | - | - |
| cycles | - | - | - | 7823 | 40.37k | - | - | - |
| instructions | - | - | - | 10.20k | 65.84k | - | - | - |
| cache references | - | - | - | 67.23 | 1.416 | - | - | - |
| cache misses | - | - | - | 6.282 | 0.171 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.573 | 0.00295 | - | - | - |
| data TLB read misses | - | - | - | 6.253 | 0.0208 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 48.80M | - | - | - | - | 65.02M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 48.80M | - | - | - | - | 60.52M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 5.075M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 668875 messages, 57137265 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-473724-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-473724-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 72274 | 2028 | 3475269 | 0 | 61404000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 789.7 | 0.4 | 31.2 | 28.86 / 26.34 |
| best | redis | 22 × busy | 72274 | 2028 | 3475269 | 0 | 61404000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 789.7 | 0.4 | 31.2 | 28.86 / 26.34 |

Run 1's profile:

| profile: the window, 30.0 s, 60653 messages, 51355490 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 656.8G | 10.83M | 12.79k | 3614 | 12.79k | 26.51G | 428.7k | 506.0 |
| waiting for a CPU, ns | - | - | - | 10.60 | 0.913 | - | - | - |
| allocations | 248.5M | 4096 | 4.838 | 7.016 | 4.830 | 2.129M | 34.42 | 0.0406 |
| reallocations | 2.456M | 40.49 | 0.0478 | 2.000 | 0.0455 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.454M | 40.46 | 0.0478 | 1.978 | 0.0454 | - | - | - |
| frees | 248.4M | 4096 | 4.838 | 7.016 | 4.830 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 31.51G | 519.5k | 613.5 | 2268 | 610.8 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 8418 | 0.139 | 0.000164 | 0.0000165 | 0.000164 | 1.305M | 21.11 | 0.0249 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 926.0 | 0.0153 | 0.0000180 | 0.0153 | 0 | - | - | - |
| involuntary context switches | 3067 | 0.0506 | 0.0000597 | 0 | 0.0000597 | - | - | - |
| cycles | - | - | - | 12.07k | 32.30k | - | - | - |
| instructions | - | - | - | 10.39k | 53.43k | - | - | - |
| cache references | - | - | - | 152.0 | 8.001 | - | - | - |
| cache misses | - | - | - | 134.4 | 4.522 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 18.05 | 0.0600 | - | - | - |
| data TLB read misses | - | - | - | 5.738 | 0.0305 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 98.30M | - | - | - | - | 116.5M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 98.30M | - | - | - | - | 93.17M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.59 s from a quarter second before the window to a quarter second after it, 61832 messages, 52392261 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-491919-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-491919-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 20.7 | 45.6 | 108.5 | 217.1 | 438.7 | 0 | 0 | 1/1 | 37.1 | 34.5 | 2.0 | 6.20 / 4.18 |
| best | redis | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 20.7 | 45.6 | 108.5 | 217.1 | 438.7 | 0 | 0 | 1/1 | 37.1 | 34.5 | 2.0 | 6.20 / 4.18 |

Run 1's profile:

| profile: the window, 30.0 s, 299584 messages, 299584 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 31.34G | 104.6k | 104.6k | 97.41k | 7199 | 1.750G | 5744 | 5744 |
| waiting for a CPU, ns | - | - | - | 2.856 | 5.251 | - | - | - |
| allocations | 3.901M | 13.02 | 13.02 | 7.983 | 5.038 | 913.8k | 3.000 | 3.000 |
| reallocations | 610.7k | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 610.7k | 2.038 | 2.038 | 2.000 | 0.0385 | - | - | - |
| frees | 3.901M | 13.02 | 13.02 | 7.983 | 5.038 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 852.5M | 2846 | 2846 | 2298 | 547.1 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 0 | 0 | 0 | 0 | 0 | 40.00 | 0.000131 | 0.000131 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 274.8k | 0.917 | 0.917 | 0 | 0.917 | - | - | - |
| involuntary context switches | 172.0 | 0.000574 | 0.000574 | 0.000431 | 0.000144 | - | - | - |
| cycles | - | - | - | 561.1k | 15.94k | - | - | - |
| instructions | - | - | - | 1.035M | 20.09k | - | - | - |
| cache references | - | - | - | 26.47 | 20.21 | - | - | - |
| cache misses | - | - | - | 7.579 | 1.640 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0320 | 0.0297 | - | - | - |
| data TLB read misses | - | - | - | 0.345 | 0.648 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.332M | - | - | - | - | 56.53M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.332M | - | - | - | - | 55.10M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.217M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304595 messages, 304595 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-502175-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-502175-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 745.5 | 1146.9 | 1490.9 | 1703.9 | 3556.1 | 0 | 0 | 100/100 | 115.0 | 34.6 | 17.8 | 9.15 / 6.98 |
| best | redis | 22 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 745.5 | 1146.9 | 1490.9 | 1703.9 | 3556.1 | 0 | 0 | 100/100 | 115.0 | 34.6 | 17.8 | 9.15 / 6.98 |

Run 1's profile:

| profile: the window, 30.0 s, 299582 messages, 29957744 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 97.29G | 324.8k | 3248 | 97.62k | 2272 | 15.25G | 50.06k | 500.6 |
| waiting for a CPU, ns | - | - | - | 5.210 | 1.539 | - | - | - |
| allocations | 129.3M | 431.6 | 4.316 | 7.168 | 4.244 | 915.5k | 3.005 | 0.0301 |
| reallocations | 1.751M | 5.846 | 0.0585 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 1.751M | 5.844 | 0.0584 | 1.998 | 0.0385 | - | - | - |
| frees | 129.3M | 431.6 | 4.316 | 7.167 | 4.244 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 16.31G | 54.44k | 544.4 | 2272 | 521.7 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 1.000 | 0.00000334 | 0.000000033 | 0 | 0.000000033 | 344.0 | 0.00113 | 0.0000113 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 4.785M | 15.97 | 0.160 | 0 | 0.160 | - | - | - |
| involuntary context switches | 1967 | 0.00657 | 0.0000657 | 0.000634 | 0.0000593 | - | - | - |
| cycles | - | - | - | 513.9k | 8055 | - | - | - |
| instructions | - | - | - | 940.5k | 14.11k | - | - | - |
| cache references | - | - | - | 19.35 | 5.637 | - | - | - |
| cache misses | - | - | - | 2.852 | 0.339 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0337 | 0.00828 | - | - | - |
| data TLB read misses | - | - | - | 0.264 | 0.153 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 13.72M | - | - | - | - | 61.89M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 13.72M | - | - | - | - | 59.97M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 2.221M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304609 messages, 30461872 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-509307-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-509307-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 85313 | 2401 | 269485 | 0 | 72943000 | 29982000 | 269601000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 5180000 | 0/1000 | 83.8 | 0.5 | 30.2 | 12.27 / 8.81 |
| best | redis | 22 × wait | 85313 | 2401 | 269485 | 0 | 72943000 | 29982000 | 269601000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 5180000 | 0/1000 | 83.8 | 0.5 | 30.2 | 12.27 / 8.81 |

Run 1's profile:

| profile: the window, 30.0 s, 71713 messages, 61153000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 70.00G | 976.1k | 1145 | 4576 | 1139 | 25.62G | 350.6k | 411.2 |
| waiting for a CPU, ns | - | - | - | 27.36 | 1.733 | - | - | - |
| allocations | 294.1M | 4101 | 4.809 | 7.017 | 4.800 | 2.440M | 33.38 | 0.0391 |
| reallocations | 2.905M | 40.51 | 0.0475 | 2.000 | 0.0452 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.901M | 40.46 | 0.0474 | 1.980 | 0.0451 | - | - | - |
| frees | 294.1M | 4101 | 4.809 | 7.017 | 4.800 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 37.28G | 519.8k | 609.5 | 2268 | 606.9 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 9129 | 0.127 | 0.000149 | 0.000377 | 0.000149 | 1.434M | 19.62 | 0.0230 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.157M | 16.14 | 0.0189 | 0.0166 | 0.0189 | - | - | - |
| involuntary context switches | 1434 | 0.0200 | 0.0000234 | 0.000112 | 0.0000233 | - | - | - |
| cycles | - | - | - | 29.79k | 7684 | - | - | - |
| instructions | - | - | - | 24.89k | 15.39k | - | - | - |
| cache references | - | - | - | 353.0 | 18.90 | - | - | - |
| cache misses | - | - | - | 304.0 | 10.03 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 28.22 | 0.336 | - | - | - |
| data TLB read misses | - | - | - | 12.54 | 0.430 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 104.7M | - | - | - | - | 121.5M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 104.7M | - | - | - | - | 115.0M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.57 s from a quarter second before the window to a quarter second after it, 73081 messages, 62319262 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-527735-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-527735-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 23.6 | 30.2 | 41.5 | 184.3 | 2354.7 | 0 | 0 | 1/1 | 40.0 | 27.8 | 12.4 | 2.82 / 2.58 |
| best | redis | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 23.6 | 30.2 | 41.5 | 184.3 | 2354.7 | 0 | 0 | 1/1 | 40.0 | 27.8 | 12.4 | 2.82 / 2.58 |

Run 1's profile:

| profile: the window, 30.0 s, 3000420 messages, 3000420 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 33.80G | 11.27k | 11.27k | 7820 | 3445 | 10.66G | 3494 | 3494 |
| waiting for a CPU, ns | - | - | - | 1.340 | 1.857 | - | - | - |
| allocations | 37.62M | 12.54 | 12.54 | 7.733 | 4.806 | 9.151M | 3.000 | 3.000 |
| reallocations | 6.116M | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 6.116M | 2.038 | 2.038 | 2.000 | 0.0385 | - | - | - |
| frees | 37.62M | 12.54 | 12.54 | 7.733 | 4.806 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 8.491G | 2830 | 2830 | 2290 | 539.7 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 15.00 | 0.00000500 | 0.00000500 | 0 | 0.00000500 | 48.00 | 0.0000157 | 0.0000157 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 2.128M | 0.709 | 0.709 | 0 | 0.709 | - | - | - |
| involuntary context switches | 726.0 | 0.000242 | 0.000242 | 0.000184 | 0.0000577 | - | - | - |
| cycles | - | - | - | 25.01k | 10.50k | - | - | - |
| instructions | - | - | - | 46.00k | 14.93k | - | - | - |
| cache references | - | - | - | 0.503 | 2.380 | - | - | - |
| cache misses | - | - | - | 0.0547 | 0.108 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00220 | 0.00107 | - | - | - |
| data TLB read misses | - | - | - | 0.0657 | 0.152 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.115M | - | - | - | - | 57.83M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.115M | - | - | - | - | 56.03M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.241M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3050376 messages, 3050372 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-550405-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-550405-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 947826 | 26648 | 2602212 | 0 | 80408600 | 37997700 | 262046000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 6802000 | 0/100 | 97.0 | 2.3 | 29.8 | 12.86 / 9.84 |
| best | redis | 22 × wait | 947826 | 26648 | 2602212 | 0 | 80408600 | 37997700 | 262046000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 6802000 | 0/100 | 97.0 | 2.3 | 29.8 | 12.86 / 9.84 |

Run 1's profile:

| profile: the window, 30.0 s, 800708 messages, 67918009 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 81.86G | 102.2k | 1205 | 2364 | 1177 | 25.58G | 31.42k | 370.4 |
| waiting for a CPU, ns | - | - | - | 3.032 | 0.898 | - | - | - |
| allocations | 333.3M | 416.3 | 4.908 | 7.016 | 4.825 | 4.914M | 6.035 | 0.0712 |
| reallocations | 4.682M | 5.847 | 0.0689 | 2.000 | 0.0454 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 4.674M | 5.838 | 0.0688 | 1.991 | 0.0454 | - | - | - |
| frees | 333.3M | 416.3 | 4.908 | 7.016 | 4.825 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 43.24G | 54.00k | 636.6 | 2268 | 609.9 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 9792 | 0.0122 | 0.000144 | 0 | 0.000144 | 45.00 | 0.0000553 | 0.000000652 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.143M | 1.427 | 0.0168 | 0.0154 | 0.0166 | - | - | - |
| involuntary context switches | 1387 | 0.00173 | 0.0000204 | 0.0000312 | 0.0000201 | - | - | - |
| cycles | - | - | - | 13.10k | 7278 | - | - | - |
| instructions | - | - | - | 22.01k | 14.99k | - | - | - |
| cache references | - | - | - | 154.7 | 1.900 | - | - | - |
| cache misses | - | - | - | 7.031 | 0.150 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.436 | 0.00602 | - | - | - |
| data TLB read misses | - | - | - | 6.068 | 0.0823 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 54.81M | - | - | - | - | 64.00M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 54.81M | - | - | - | - | 60.84M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 5.075M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 814163 messages, 69055055 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-555692-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-555692-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 85292 | 2396 | 3464746 | 0 | 72652000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 83.6 | 0.4 | 29.9 | 15.94 / 11.18 |
| best | redis | 22 × wait | 85292 | 2396 | 3464746 | 0 | 72652000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 83.6 | 0.4 | 29.9 | 15.94 / 11.18 |

Run 1's profile:

| profile: the window, 30.0 s, 71677 messages, 60964744 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 69.85G | 974.5k | 1146 | 4115 | 1141 | 25.34G | 347.4k | 408.6 |
| waiting for a CPU, ns | - | - | - | 7.610 | 1.175 | - | - | - |
| allocations | 294.3M | 4105 | 4.827 | 7.020 | 4.818 | 2.475M | 33.93 | 0.0399 |
| reallocations | 2.905M | 40.53 | 0.0477 | 2.000 | 0.0453 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.903M | 40.50 | 0.0476 | 1.981 | 0.0453 | - | - | - |
| frees | 294.3M | 4105 | 4.827 | 7.020 | 4.818 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 37.29G | 520.2k | 611.7 | 2268 | 609.0 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 9894 | 0.138 | 0.000162 | 0.000126 | 0.000162 | 1.386M | 19.00 | 0.0223 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.282M | 17.88 | 0.0210 | 0.0188 | 0.0210 | - | - | - |
| involuntary context switches | 1167 | 0.0163 | 0.0000191 | 0.0000558 | 0.0000191 | - | - | - |
| cycles | - | - | - | 26.51k | 7738 | - | - | - |
| instructions | - | - | - | 21.22k | 15.35k | - | - | - |
| cache references | - | - | - | 304.7 | 18.22 | - | - | - |
| cache misses | - | - | - | 259.6 | 9.622 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 24.23 | 0.329 | - | - | - |
| data TLB read misses | - | - | - | 10.29 | 0.377 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 105.7M | - | - | - | - | 116.9M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 105.7M | - | - | - | - | 95.90M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.54 s from a quarter second before the window to a quarter second after it, 72945 messages, 62005108 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-563555-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-563555-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 17.4 | 22.0 | 64.0 | 198.7 | 298.8 | 0 | 0 | 1/1 | 69.7 | 34.5 | 2.1 | 4.00 / 3.36 |
| best | redis | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 17.4 | 22.0 | 64.0 | 198.7 | 298.8 | 0 | 0 | 1/1 | 69.7 | 34.5 | 2.1 | 4.00 / 3.36 |

Run 1's profile:

| profile: the window, 30.0 s, 299587 messages, 299586 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 58.82G | 196.3k | 196.3k | 97.38k | 98.95k | 1.770G | 5810 | 5810 |
| waiting for a CPU, ns | - | - | - | 8.326 | 7.884 | - | - | - |
| allocations | 3.907M | 13.04 | 13.04 | 7.982 | 5.059 | 913.8k | 3.000 | 3.000 |
| reallocations | 610.7k | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 610.7k | 2.038 | 2.038 | 2.000 | 0.0385 | - | - | - |
| frees | 3.907M | 13.04 | 13.04 | 7.982 | 5.059 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 852.7M | 2846 | 2846 | 2298 | 547.8 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 0 | 0 | 0 | 0 | 0 | 41.00 | 0.000135 | 0.000135 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 450.0 | 0.00150 | 0.00150 | 0.000754 | 0.000748 | - | - | - |
| cycles | - | - | - | 349.9k | 659.8k | - | - | - |
| instructions | - | - | - | 652.1k | 1.127M | - | - | - |
| cache references | - | - | - | 1.621 | 562.3 | - | - | - |
| cache misses | - | - | - | 0.227 | 174.1 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0158 | 0.0141 | - | - | - |
| data TLB read misses | - | - | - | 0.157 | 7.488 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.541M | - | - | - | - | 56.02M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.541M | - | - | - | - | 55.00M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.220M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304593 messages, 304593 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-574106-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-574106-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 868.4 | 1343.5 | 1769.5 | 2097.2 | 3219.3 | 0 | 0 | 100/100 | 804.0 | 33.5 | 19.3 | 24.41 / 24.57 |
| best | redis | 22 × busy | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 868.4 | 1343.5 | 1769.5 | 2097.2 | 3219.3 | 0 | 0 | 100/100 | 804.0 | 33.5 | 19.3 | 24.41 / 24.57 |

Run 1's profile:

| profile: the window, 30.0 s, 299581 messages, 29957593 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 678.7G | 2.266M | 22.66k | 94.60k | 21.71k | 16.54G | 54.30k | 543.0 |
| waiting for a CPU, ns | - | - | - | 12.39 | 0.992 | - | - | - |
| allocations | 128.6M | 429.4 | 4.294 | 7.146 | 4.223 | 914.0k | 3.001 | 0.0300 |
| reallocations | 1.751M | 5.846 | 0.0585 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 1.750M | 5.842 | 0.0584 | 1.996 | 0.0385 | - | - | - |
| frees | 128.6M | 429.4 | 4.294 | 7.146 | 4.223 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 16.29G | 54.38k | 543.8 | 2272 | 521.0 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 0 | 0 | 0 | 0 | 0 | 61.00 | 0.000200 | 0.00000200 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 3344 | 0.0112 | 0.000112 | 0.000598 | 0.000106 | - | - | - |
| cycles | - | - | - | 175.2k | 56.32k | - | - | - |
| instructions | - | - | - | 317.8k | 88.04k | - | - | - |
| cache references | - | - | - | 0.900 | 0.312 | - | - | - |
| cache misses | - | - | - | 0.275 | 0.0450 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.000441 | - | - | - |
| data TLB read misses | - | - | - | 0.00696 | 0.00976 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 13.50M | - | - | - | - | 62.31M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 13.50M | - | - | - | - | 58.16M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 2.082M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304610 messages, 30462744 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-578982-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-578982-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 72609 | 2044 | 282189 | 0 | 62009000 | 19118000 | 280465000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3340000 | 0/1000 | 790.9 | 0.4 | 30.8 | 28.97 / 26.43 |
| best | redis | 22 × busy | 72609 | 2044 | 282189 | 0 | 62009000 | 19118000 | 280465000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3340000 | 0/1000 | 790.9 | 0.4 | 30.8 | 28.97 / 26.43 |

Run 1's profile:

| profile: the window, 30.0 s, 60584 messages, 51584000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 657.5G | 10.85M | 12.75k | 3576 | 12.74k | 26.10G | 422.8k | 496.5 |
| waiting for a CPU, ns | - | - | - | 7.748 | 1.610 | - | - | - |
| allocations | 248.4M | 4100 | 4.816 | 7.017 | 4.808 | 2.074M | 33.60 | 0.0395 |
| reallocations | 2.454M | 40.51 | 0.0476 | 2.000 | 0.0452 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.452M | 40.48 | 0.0475 | 1.982 | 0.0452 | - | - | - |
| frees | 248.4M | 4100 | 4.816 | 7.017 | 4.808 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 31.50G | 519.9k | 610.6 | 2268 | 607.9 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 8542 | 0.141 | 0.000166 | 0.000248 | 0.000165 | 1.258M | 20.38 | 0.0239 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1000 | 0.0165 | 0.0000194 | 0.0165 | 0 | - | - | - |
| involuntary context switches | 4511 | 0.0745 | 0.0000874 | 0.0000495 | 0.0000874 | - | - | - |
| cycles | - | - | - | 12.50k | 32.83k | - | - | - |
| instructions | - | - | - | 10.25k | 54.28k | - | - | - |
| cache references | - | - | - | 161.2 | 8.104 | - | - | - |
| cache misses | - | - | - | 143.8 | 4.406 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 20.45 | 0.0587 | - | - | - |
| data TLB read misses | - | - | - | 6.577 | 0.0206 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 98.99M | - | - | - | - | 118.3M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 98.99M | - | - | - | - | 99.76M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 19.12M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.57 s from a quarter second before the window to a quarter second after it, 61729 messages, 52561176 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-585615-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-585615-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × busy | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 21.2 | 29.2 | 47.6 | 208.9 | 743.4 | 0 | 0 | 1/1 | 60.2 | 27.5 | 12.9 | 11.01 / 8.42 |
| best | redis | 1 × busy | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 21.2 | 29.2 | 47.6 | 208.9 | 743.4 | 0 | 0 | 1/1 | 60.2 | 27.5 | 12.9 | 11.01 / 8.42 |

Run 1's profile:

| profile: the window, 30.0 s, 3000428 messages, 3000429 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 50.87G | 16.95k | 16.95k | 7754 | 9201 | 11.00G | 3605 | 3605 |
| waiting for a CPU, ns | - | - | - | 1.101 | 0.818 | - | - | - |
| allocations | 37.53M | 12.51 | 12.51 | 7.716 | 4.793 | 9.151M | 3.000 | 3.000 |
| reallocations | 6.116M | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 6.081M | 2.027 | 2.027 | 1.988 | 0.0385 | - | - | - |
| frees | 37.53M | 12.51 | 12.51 | 7.716 | 4.793 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 8.489G | 2829 | 2829 | 2290 | 539.3 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 0 | 0 | 0 | 0 | 0 | 47.00 | 0.0000154 | 0.0000154 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 743.0 | 0.000248 | 0.000248 | 0.000166 | 0.0000813 | - | - | - |
| cycles | - | - | - | 23.90k | 54.04k | - | - | - |
| instructions | - | - | - | 43.54k | 89.91k | - | - | - |
| cache references | - | - | - | 1.099 | 4.277 | - | - | - |
| cache misses | - | - | - | 0.483 | 1.357 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0476 | 0.0336 | - | - | - |
| data TLB read misses | - | - | - | 0.0456 | 0.0292 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.898M | - | - | - | - | 57.39M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.898M | - | - | - | - | 55.25M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.250M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3050386 messages, 3050388 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-602276-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-602276-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 807454 | 22678 | 2742584 | 0 | 69131400 | 26330500 | 273713200 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 4432000 | 0/100 | 782.9 | 1.8 | 30.2 | 24.59 / 23.89 |
| best | redis | 22 × busy | 807454 | 22678 | 2742584 | 0 | 69131400 | 26330500 | 273713200 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 4432000 | 0/100 | 782.9 | 1.8 | 30.2 | 24.59 / 23.89 |

Run 1's profile:

| profile: the window, 30.0 s, 678525 messages, 58078500 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 659.1G | 971.3k | 11.35k | 2117 | 11.32k | 25.96G | 37.61k | 439.4 |
| waiting for a CPU, ns | - | - | - | 3.235 | 0.810 | - | - | - |
| allocations | 282.4M | 416.3 | 4.863 | 7.016 | 4.781 | 4.058M | 5.879 | 0.0687 |
| reallocations | 3.967M | 5.847 | 0.0683 | 2.000 | 0.0449 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 3.961M | 5.838 | 0.0682 | 1.991 | 0.0449 | - | - | - |
| frees | 282.4M | 416.3 | 4.863 | 7.016 | 4.781 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 36.64G | 54.01k | 630.9 | 2268 | 604.5 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 8038 | 0.0118 | 0.000138 | 0.00000884 | 0.000138 | 99.00 | 0.000143 | 0.00000168 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 10.76k | 0.0159 | 0.000185 | 0.0156 | 0.00000293 | - | - | - |
| involuntary context switches | 3938 | 0.00580 | 0.0000678 | 0.00000737 | 0.0000677 | - | - | - |
| cycles | - | - | - | 7757 | 33.48k | - | - | - |
| instructions | - | - | - | 10.22k | 54.41k | - | - | - |
| cache references | - | - | - | 67.73 | 35.25 | - | - | - |
| cache misses | - | - | - | 4.401 | 13.65 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.408 | 3.981 | - | - | - |
| data TLB read misses | - | - | - | 6.143 | 0.453 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 49.56M | - | - | - | - | 65.42M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 49.56M | - | - | - | - | 63.70M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 5.075M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 690188 messages, 59076371 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-608398-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-608398-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 68796 | 1928 | 3481242 | 0 | 58506000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 791.5 | 0.4 | 30.5 | 29.60 / 27.34 |
| best | redis | 22 × busy | 68796 | 1928 | 3481242 | 0 | 58506000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 791.5 | 0.4 | 30.5 | 29.60 / 27.34 |

Run 1's profile:

| profile: the window, 30.0 s, 57620 messages, 48818778 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 657.7G | 11.41M | 13.47k | 3611 | 13.47k | 25.88G | 441.0k | 520.0 |
| waiting for a CPU, ns | - | - | - | 23.68 | 1.239 | - | - | - |
| allocations | 236.1M | 4097 | 4.835 | 7.018 | 4.827 | 2.012M | 34.30 | 0.0404 |
| reallocations | 2.331M | 40.46 | 0.0478 | 2.000 | 0.0454 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.330M | 40.43 | 0.0477 | 1.985 | 0.0454 | - | - | - |
| frees | 236.1M | 4097 | 4.835 | 7.018 | 4.827 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 29.90G | 519.0k | 612.5 | 2268 | 609.9 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 4676 | 0.0812 | 0.0000958 | 0.0000174 | 0.0000958 | 1.230M | 20.97 | 0.0247 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 984.0 | 0.0171 | 0.0000202 | 0.0171 | 0 | - | - | - |
| involuntary context switches | 3451 | 0.0599 | 0.0000707 | 0.0000174 | 0.0000707 | - | - | - |
| cycles | - | - | - | 12.18k | 33.68k | - | - | - |
| instructions | - | - | - | 10.35k | 54.95k | - | - | - |
| cache references | - | - | - | 146.9 | 8.455 | - | - | - |
| cache misses | - | - | - | 132.6 | 4.825 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 18.51 | 0.0649 | - | - | - |
| data TLB read misses | - | - | - | 5.910 | 0.0348 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 85.03M | - | - | - | - | 120.6M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 85.03M | - | - | - | - | 113.8M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.61 s from a quarter second before the window to a quarter second after it, 58676 messages, 49766715 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-616966-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-616966-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 20.7 | 52.7 | 118.8 | 380.9 | 4474.7 | 0 | 0 | 1/1 | 37.2 | 34.5 | 2.2 | 12.62 / 8.43 |
| best | redis | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 20.7 | 52.7 | 118.8 | 380.9 | 4474.7 | 0 | 0 | 1/1 | 37.2 | 34.5 | 2.2 | 12.62 / 8.43 |

Run 1's profile:

| profile: the window, 30.0 s, 300766 messages, 300766 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 31.37G | 104.3k | 104.3k | 96.96k | 7351 | 1.856G | 6070 | 6070 |
| waiting for a CPU, ns | - | - | - | 5.189 | 48.54 | - | - | - |
| allocations | 3.913M | 13.01 | 13.01 | 7.979 | 5.032 | 949.4k | 3.104 | 3.104 |
| reallocations | 613.1k | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 613.1k | 2.038 | 2.038 | 2.000 | 0.0385 | - | - | - |
| frees | 3.913M | 13.01 | 13.01 | 7.979 | 5.032 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 855.8M | 2845 | 2845 | 2298 | 546.9 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 16.00 | 0.0000532 | 0.0000532 | 0.0000499 | 0.00000332 | 40.00 | 0.000131 | 0.000131 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 273.7k | 0.910 | 0.910 | 0 | 0.910 | - | - | - |
| involuntary context switches | 246.0 | 0.000818 | 0.000818 | 0.000625 | 0.000193 | - | - | - |
| cycles | - | - | - | 348.3k | 16.14k | - | - | - |
| instructions | - | - | - | 646.7k | 20.10k | - | - | - |
| cache references | - | - | - | 3.194 | 29.33 | - | - | - |
| cache misses | - | - | - | 1.665 | 4.955 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.173 | 0.0599 | - | - | - |
| data TLB read misses | - | - | - | 0.0869 | 0.723 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.824M | - | - | - | - | 58.45M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.824M | - | - | - | - | 55.30M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.216M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305841 messages, 305840 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-627440-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-627440-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 761.9 | 1179.6 | 1540.1 | 1769.5 | 2675.4 | 0 | 0 | 100/100 | 113.6 | 34.6 | 17.9 | 8.03 / 7.05 |
| best | redis | 22 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 761.9 | 1179.6 | 1540.1 | 1769.5 | 2675.4 | 0 | 0 | 100/100 | 113.6 | 34.6 | 17.9 | 8.03 / 7.05 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 30076057 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 96.00G | 319.2k | 3192 | 97.25k | 2219 | 15.39G | 50.32k | 503.2 |
| waiting for a CPU, ns | - | - | - | 1.713 | 1.311 | - | - | - |
| allocations | 129.7M | 431.1 | 4.311 | 7.163 | 4.239 | 918.2k | 3.002 | 0.0300 |
| reallocations | 1.758M | 5.846 | 0.0585 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 1.758M | 5.844 | 0.0584 | 1.998 | 0.0385 | - | - | - |
| frees | 129.7M | 431.1 | 4.311 | 7.163 | 4.239 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 16.37G | 54.43k | 544.3 | 2272 | 521.6 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 0 | 0 | 0 | 0 | 0 | 238.0 | 0.000778 | 0.00000778 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 4.620M | 15.36 | 0.154 | 0 | 0.154 | - | - | - |
| involuntary context switches | 1614 | 0.00537 | 0.0000537 | 0.000269 | 0.0000510 | - | - | - |
| cycles | - | - | - | 504.9k | 8079 | - | - | - |
| instructions | - | - | - | 930.6k | 14.20k | - | - | - |
| cache references | - | - | - | 25.78 | 5.581 | - | - | - |
| cache misses | - | - | - | 5.300 | 0.336 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0215 | 0.00713 | - | - | - |
| data TLB read misses | - | - | - | 0.388 | 0.187 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 14.44M | - | - | - | - | 62.64M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 14.44M | - | - | - | - | 60.24M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 2.200M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305884 messages, 30589496 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-633409-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-633409-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 81994 | 2309 | 274107 | 0 | 70214000 | 26970000 | 273799000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 4620000 | 0/1000 | 80.2 | 0.5 | 30.3 | 13.40 / 8.77 |
| best | redis | 22 × wait | 81994 | 2309 | 274107 | 0 | 70214000 | 26970000 | 273799000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 4620000 | 0/1000 | 80.2 | 0.5 | 30.3 | 13.40 / 8.77 |

Run 1's profile:

| profile: the window, 30.0 s, 68748 messages, 58755016 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 66.79G | 971.5k | 1137 | 4165 | 1132 | 25.75G | 366.9k | 429.9 |
| waiting for a CPU, ns | - | - | - | 2.360 | 1.117 | - | - | - |
| allocations | 282.0M | 4103 | 4.800 | 7.017 | 4.792 | 2.324M | 33.11 | 0.0388 |
| reallocations | 2.787M | 40.54 | 0.0474 | 2.000 | 0.0451 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.785M | 40.51 | 0.0474 | 1.986 | 0.0451 | - | - | - |
| frees | 282.0M | 4103 | 4.800 | 7.017 | 4.792 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 35.76G | 520.1k | 608.6 | 2268 | 605.9 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 8733 | 0.127 | 0.000149 | 0.0000873 | 0.000149 | 1.388M | 19.78 | 0.0232 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.105M | 16.07 | 0.0188 | 0.0161 | 0.0188 | - | - | - |
| involuntary context switches | 1419 | 0.0206 | 0.0000242 | 0.0000582 | 0.0000241 | - | - | - |
| cycles | - | - | - | 27.70k | 7703 | - | - | - |
| instructions | - | - | - | 23.98k | 15.41k | - | - | - |
| cache references | - | - | - | 351.5 | 18.93 | - | - | - |
| cache misses | - | - | - | 314.2 | 9.886 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 29.51 | 0.338 | - | - | - |
| data TLB read misses | - | - | - | 12.43 | 0.411 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 100.6M | - | - | - | - | 129.8M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 100.6M | - | - | - | - | 122.3M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.60 s from a quarter second before the window to a quarter second after it, 70184 messages, 59898437 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-641578-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-641578-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × wait | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 23.0 | 29.7 | 42.0 | 186.4 | 2527.4 | 0 | 0 | 1/1 | 39.6 | 27.7 | 12.4 | 5.67 / 3.88 |
| best | redis | 1 × wait | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 23.0 | 29.7 | 42.0 | 186.4 | 2527.4 | 0 | 0 | 1/1 | 39.6 | 27.7 | 12.4 | 5.67 / 3.88 |

Run 1's profile:

| profile: the window, 30.0 s, 3002626 messages, 3002628 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 33.42G | 11.13k | 11.13k | 7808 | 3323 | 10.61G | 3476 | 3476 |
| waiting for a CPU, ns | - | - | - | 1.161 | 2.287 | - | - | - |
| allocations | 37.65M | 12.54 | 12.54 | 7.732 | 4.806 | 9.158M | 3.000 | 3.000 |
| reallocations | 6.121M | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 6.120M | 2.038 | 2.038 | 2.000 | 0.0385 | - | - | - |
| frees | 37.65M | 12.54 | 12.54 | 7.732 | 4.806 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 8.495G | 2829 | 2829 | 2289 | 539.7 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 18.00 | 0.00000599 | 0.00000599 | 0 | 0.00000599 | 44.00 | 0.0000144 | 0.0000144 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 2.163M | 0.720 | 0.720 | 0 | 0.720 | - | - | - |
| involuntary context switches | 611.0 | 0.000203 | 0.000203 | 0.000170 | 0.0000333 | - | - | - |
| cycles | - | - | - | 39.34k | 9992 | - | - | - |
| instructions | - | - | - | 63.39k | 14.37k | - | - | - |
| cache references | - | - | - | 3.556 | 3.734 | - | - | - |
| cache misses | - | - | - | 0.609 | 0.249 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00293 | 0.00167 | - | - | - |
| data TLB read misses | - | - | - | 0.750 | 0.262 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.971M | - | - | - | - | 57.49M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.971M | - | - | - | - | 55.34M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.248M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3052570 messages, 3052567 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-656606-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-656606-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 931079 | 26170 | 2622939 | 0 | 79657900 | 36762300 | 263500800 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 6216000 | 0/100 | 98.3 | 2.3 | 29.6 | 11.03 / 9.65 |
| best | redis | 22 × wait | 931079 | 26170 | 2622939 | 0 | 79657900 | 36762300 | 263500800 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 6216000 | 0/100 | 98.3 | 2.3 | 29.6 | 11.03 / 9.65 |

Run 1's profile:

| profile: the window, 30.0 s, 785433 messages, 67183002 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 83.35G | 106.1k | 1241 | 2490 | 1212 | 25.37G | 31.76k | 371.3 |
| waiting for a CPU, ns | - | - | - | 3.300 | 0.798 | - | - | - |
| allocations | 327.0M | 416.4 | 4.868 | 7.018 | 4.786 | 4.706M | 5.891 | 0.0689 |
| reallocations | 4.592M | 5.846 | 0.0684 | 2.000 | 0.0450 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 4.586M | 5.839 | 0.0683 | 1.992 | 0.0450 | - | - | - |
| frees | 327.0M | 416.4 | 4.868 | 7.018 | 4.786 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 42.42G | 54.00k | 631.3 | 2268 | 604.8 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 8908 | 0.0113 | 0.000133 | 0.00000509 | 0.000133 | 97.00 | 0.000121 | 0.00000142 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.139M | 1.451 | 0.0170 | 0.0169 | 0.0168 | - | - | - |
| involuntary context switches | 1090 | 0.00139 | 0.0000162 | 0.0000267 | 0.0000159 | - | - | - |
| cycles | - | - | - | 12.90k | 7263 | - | - | - |
| instructions | - | - | - | 21.16k | 14.83k | - | - | - |
| cache references | - | - | - | 170.1 | 1.869 | - | - | - |
| cache misses | - | - | - | 6.695 | 0.177 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.515 | 0.00750 | - | - | - |
| data TLB read misses | - | - | - | 7.042 | 0.0859 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 53.27M | - | - | - | - | 63.83M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 53.27M | - | - | - | - | 60.42M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 5.075M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 798840 messages, 68331636 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-669339-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-669339-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 85727 | 2406 | 3468291 | 0 | 72797000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 81.1 | 0.5 | 30.4 | 14.36 / 9.25 |
| best | redis | 22 × wait | 85727 | 2406 | 3468291 | 0 | 72797000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 81.1 | 0.5 | 30.4 | 14.36 / 9.25 |

Run 1's profile:

| profile: the window, 30.0 s, 71264 messages, 60384000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 67.04G | 940.8k | 1110 | 4607 | 1105 | 25.84G | 355.4k | 419.5 |
| waiting for a CPU, ns | - | - | - | 8.402 | 0.989 | - | - | - |
| allocations | 292.1M | 4099 | 4.838 | 7.016 | 4.829 | 2.504M | 34.44 | 0.0406 |
| reallocations | 2.886M | 40.49 | 0.0478 | 2.000 | 0.0454 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.884M | 40.46 | 0.0478 | 1.985 | 0.0454 | - | - | - |
| frees | 292.1M | 4099 | 4.838 | 7.016 | 4.829 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 37.04G | 519.7k | 613.4 | 2268 | 610.7 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 9065 | 0.127 | 0.000150 | 0.0000140 | 0.000150 | 1.373M | 18.88 | 0.0223 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.063M | 14.92 | 0.0176 | 0.0153 | 0.0176 | - | - | - |
| involuntary context switches | 1224 | 0.0172 | 0.0000203 | 0.0000421 | 0.0000202 | - | - | - |
| cycles | - | - | - | 13.41k | 7695 | - | - | - |
| instructions | - | - | - | 10.38k | 15.50k | - | - | - |
| cache references | - | - | - | 125.8 | 17.79 | - | - | - |
| cache misses | - | - | - | 106.6 | 9.499 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000421 | 0.314 | - | - | - |
| data TLB read misses | - | - | - | 5.691 | 0.450 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 101.5M | - | - | - | - | 125.2M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 101.5M | - | - | - | - | 121.4M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.59 s from a quarter second before the window to a quarter second after it, 72705 messages, 61604921 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-680381-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-680381-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 17.2 | 21.8 | 62.5 | 196.6 | 621.5 | 0 | 0 | 1/1 | 69.7 | 34.5 | 2.1 | 7.22 / 4.79 |
| best | redis | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 17.2 | 21.8 | 62.5 | 196.6 | 621.5 | 0 | 0 | 1/1 | 69.7 | 34.5 | 2.1 | 7.22 / 4.79 |

Run 1's profile:

| profile: the window, 30.0 s, 300766 messages, 300766 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 58.83G | 195.6k | 195.6k | 97.02k | 98.58k | 1.773G | 5796 | 5796 |
| waiting for a CPU, ns | - | - | - | 11.23 | 0.651 | - | - | - |
| allocations | 3.923M | 13.04 | 13.04 | 7.983 | 5.060 | 949.9k | 3.106 | 3.106 |
| reallocations | 613.1k | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 613.1k | 2.038 | 2.038 | 2.000 | 0.0385 | - | - | - |
| frees | 3.923M | 13.04 | 13.04 | 7.983 | 5.060 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 856.1M | 2846 | 2846 | 2298 | 547.8 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 0 | 0 | 0 | 0 | 0 | 41.00 | 0.000134 | 0.000134 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 367.0 | 0.00122 | 0.00122 | 0.00108 | 0.000143 | - | - | - |
| cycles | - | - | - | 349.4k | 398.3k | - | - | - |
| instructions | - | - | - | 651.2k | 689.1k | - | - | - |
| cache references | - | - | - | 1.894 | 1.762 | - | - | - |
| cache misses | - | - | - | 0.412 | 0.241 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0339 | 0.0251 | - | - | - |
| data TLB read misses | - | - | - | 0.101 | 0.00688 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.758M | - | - | - | - | 56.10M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.758M | - | - | - | - | 54.69M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.216M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305838 messages, 305838 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-692313-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-692313-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 1024.0 | 1835.0 | 2719.7 | 3276.8 | 5330.7 | 0 | 0 | 100/100 | 805.7 | 33.6 | 20.4 | 19.41 / 22.26 |
| best | redis | 22 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 1024.0 | 1835.0 | 2719.7 | 3276.8 | 5330.7 | 0 | 0 | 100/100 | 805.7 | 33.6 | 20.4 | 19.41 / 22.26 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 30076820 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 680.1G | 2.261M | 22.61k | 94.28k | 21.67k | 17.34G | 56.68k | 566.8 |
| waiting for a CPU, ns | - | - | - | 2.482 | 1.511 | - | - | - |
| allocations | 128.5M | 427.2 | 4.272 | 7.125 | 4.201 | 918.3k | 3.002 | 0.0300 |
| reallocations | 1.758M | 5.846 | 0.0585 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 1.756M | 5.837 | 0.0584 | 1.991 | 0.0385 | - | - | - |
| frees | 128.5M | 427.2 | 4.272 | 7.125 | 4.201 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 16.33G | 54.31k | 543.1 | 2271 | 520.3 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 25.00 | 0.0000831 | 0.000000831 | 0.0000166 | 0.000000665 | 246.0 | 0.000804 | 0.00000804 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 3223 | 0.0107 | 0.000107 | 0.000276 | 0.000104 | - | - | - |
| cycles | - | - | - | 175.1k | 56.49k | - | - | - |
| instructions | - | - | - | 315.0k | 88.46k | - | - | - |
| cache references | - | - | - | 2.613 | 0.799 | - | - | - |
| cache misses | - | - | - | 1.537 | 0.273 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.00172 | - | - | - |
| data TLB read misses | - | - | - | 0.00651 | 0.00547 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 13.54M | - | - | - | - | 61.86M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 13.54M | - | - | - | - | 59.95M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 2.202M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305869 messages, 30589019 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-697733-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-697733-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 71545 | 2014 | 284556 | 0 | 61075000 | 17991000 | 282778000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3150000 | 0/1000 | 789.4 | 0.4 | 30.6 | 28.89 / 26.56 |
| best | redis | 22 × busy | 71545 | 2014 | 284556 | 0 | 61075000 | 17991000 | 282778000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3150000 | 0/1000 | 789.4 | 0.4 | 30.6 | 28.89 / 26.56 |

Run 1's profile:

| profile: the window, 30.0 s, 59595 messages, 50703621 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 657.9G | 11.04M | 12.97k | 3569 | 12.97k | 26.03G | 428.5k | 503.5 |
| waiting for a CPU, ns | - | - | - | 30.64 | 0.860 | - | - | - |
| allocations | 244.2M | 4098 | 4.817 | 7.017 | 4.809 | 2.046M | 33.68 | 0.0396 |
| reallocations | 2.412M | 40.47 | 0.0476 | 2.000 | 0.0452 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.410M | 40.44 | 0.0475 | 1.983 | 0.0452 | - | - | - |
| frees | 244.2M | 4098 | 4.817 | 7.017 | 4.809 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 30.95G | 519.3k | 610.3 | 2268 | 607.6 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 4438 | 0.0745 | 0.0000875 | 0.000285 | 0.0000872 | 1.283M | 21.13 | 0.0248 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 956.0 | 0.0160 | 0.0000189 | 0.0160 | 0 | - | - | - |
| involuntary context switches | 2872 | 0.0482 | 0.0000566 | 0.0000336 | 0.0000566 | - | - | - |
| cycles | - | - | - | 12.35k | 34.96k | - | - | - |
| instructions | - | - | - | 10.29k | 56.08k | - | - | - |
| cache references | - | - | - | 153.9 | 8.759 | - | - | - |
| cache misses | - | - | - | 140.2 | 5.011 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 19.16 | 0.0703 | - | - | - |
| data TLB read misses | - | - | - | 6.194 | 0.0376 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 83.37M | - | - | - | - | 124.2M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 83.37M | - | - | - | - | 97.80M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.61 s from a quarter second before the window to a quarter second after it, 60752 messages, 51702307 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-703556-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-703556-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × busy | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 20.7 | 27.9 | 42.0 | 202.8 | 1665.9 | 0 | 0 | 1/1 | 60.2 | 27.5 | 12.6 | 18.25 / 11.83 |
| best | redis | 1 × busy | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 20.7 | 27.9 | 42.0 | 202.8 | 1665.9 | 0 | 0 | 1/1 | 60.2 | 27.5 | 12.6 | 18.25 / 11.83 |

Run 1's profile:

| profile: the window, 30.0 s, 3002644 messages, 3002643 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 50.79G | 16.91k | 16.91k | 7740 | 9174 | 10.86G | 3558 | 3558 |
| waiting for a CPU, ns | - | - | - | 1.095 | 0.626 | - | - | - |
| allocations | 37.62M | 12.53 | 12.53 | 7.726 | 4.802 | 9.158M | 3.000 | 3.000 |
| reallocations | 6.121M | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 6.120M | 2.038 | 2.038 | 2.000 | 0.0385 | - | - | - |
| frees | 37.62M | 12.53 | 12.53 | 7.726 | 4.802 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 8.497G | 2830 | 2830 | 2290 | 539.6 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 2.000 | 0.000000666 | 0.000000666 | 0.000000333 | 0.000000333 | 46.00 | 0.0000151 | 0.0000151 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 683.0 | 0.000227 | 0.000227 | 0.000159 | 0.0000683 | - | - | - |
| cycles | - | - | - | 37.44k | 34.55k | - | - | - |
| instructions | - | - | - | 59.85k | 60.44k | - | - | - |
| cache references | - | - | - | 3.929 | 0.688 | - | - | - |
| cache misses | - | - | - | 1.923 | 0.231 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0383 | 0.0261 | - | - | - |
| data TLB read misses | - | - | - | 0.644 | 0.0130 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.111M | - | - | - | - | 47.34M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.111M | - | - | - | - | 45.85M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.243M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3052609 messages, 3052599 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-715623-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-715623-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 824856 | 23168 | 2729162 | 0 | 70517600 | 27654000 | 272609100 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 4702000 | 0/100 | 783.2 | 1.8 | 30.2 | 20.43 / 21.62 |
| best | redis | 22 × busy | 824856 | 23168 | 2729162 | 0 | 70517600 | 27654000 | 272609100 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 4702000 | 0/100 | 783.2 | 1.8 | 30.2 | 20.43 / 21.62 |

Run 1's profile:

| profile: the window, 30.0 s, 694674 messages, 59373300 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 659.1G | 948.8k | 11.10k | 2059 | 11.08k | 25.91G | 36.67k | 429.0 |
| waiting for a CPU, ns | - | - | - | 0.162 | 0.806 | - | - | - |
| allocations | 289.2M | 416.3 | 4.871 | 7.017 | 4.789 | 4.172M | 5.904 | 0.0691 |
| reallocations | 4.061M | 5.847 | 0.0684 | 2.000 | 0.0450 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 4.051M | 5.831 | 0.0682 | 1.985 | 0.0450 | - | - | - |
| frees | 289.2M | 416.3 | 4.871 | 7.017 | 4.789 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 37.52G | 54.01k | 631.9 | 2268 | 605.3 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 7781 | 0.0112 | 0.000131 | 0.00000432 | 0.000131 | 103.0 | 0.000146 | 0.00000171 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 11.26k | 0.0162 | 0.000190 | 0.0160 | 0.00000258 | - | - | - |
| involuntary context switches | 4305 | 0.00620 | 0.0000725 | 0.00000720 | 0.0000724 | - | - | - |
| cycles | - | - | - | 7738 | 36.73k | - | - | - |
| instructions | - | - | - | 10.25k | 59.91k | - | - | - |
| cache references | - | - | - | 63.64 | 0.326 | - | - | - |
| cache misses | - | - | - | 3.715 | 0.0508 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.339 | 0.00177 | - | - | - |
| data TLB read misses | - | - | - | 6.180 | 0.0128 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 50.94M | - | - | - | - | 55.35M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 50.94M | - | - | - | - | 52.82M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 5.075M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 706570 messages, 60387965 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-720483-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-720483-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 72426 | 2031 | 3481592 | 0 | 62266000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 789.7 | 0.4 | 30.7 | 28.17 / 25.85 |
| best | redis | 22 × busy | 72426 | 2031 | 3481592 | 0 | 62266000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 789.7 | 0.4 | 30.7 | 28.17 / 25.85 |

Run 1's profile:

| profile: the window, 30.0 s, 60618 messages, 51987445 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 657.0G | 10.84M | 12.64k | 3479 | 12.63k | 26.05G | 421.5k | 491.7 |
| waiting for a CPU, ns | - | - | - | 1.551 | 1.377 | - | - | - |
| allocations | 248.6M | 4101 | 4.782 | 7.018 | 4.774 | 2.000M | 32.37 | 0.0378 |
| reallocations | 2.454M | 40.49 | 0.0472 | 2.000 | 0.0449 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.453M | 40.46 | 0.0472 | 1.988 | 0.0449 | - | - | - |
| frees | 248.6M | 4101 | 4.782 | 7.018 | 4.774 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 31.49G | 519.5k | 605.7 | 2268 | 603.1 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 4691 | 0.0774 | 0.0000902 | 0 | 0.0000902 | 1.168M | 18.89 | 0.0220 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1036 | 0.0171 | 0.0000199 | 0.0171 | 0 | - | - | - |
| involuntary context switches | 4426 | 0.0730 | 0.0000851 | 0.0000495 | 0.0000851 | - | - | - |
| cycles | - | - | - | 12.11k | 32.48k | - | - | - |
| instructions | - | - | - | 10.32k | 53.77k | - | - | - |
| cache references | - | - | - | 153.0 | 7.906 | - | - | - |
| cache misses | - | - | - | 131.3 | 4.289 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 18.82 | 0.0525 | - | - | - |
| data TLB read misses | - | - | - | 6.109 | 0.0197 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 83.10M | - | - | - | - | 116.0M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 83.10M | - | - | - | - | 93.09M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.57 s from a quarter second before the window to a quarter second after it, 61800 messages, 52980229 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-726781-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/redis-726781-0-redis-malloc-stats.txt
