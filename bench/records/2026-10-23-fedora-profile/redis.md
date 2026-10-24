- broker: public.ecr.aws/docker/library/redis:8.2-alpine@sha256:b51665e66f00759be7c3152ad5ac3c66fb2f619c13ef62dea7cc1f9914524635, CPUs 8-15
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 10 over 10 threads, which wait when there's nothing to take; window 10 s after 2 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 10 × wait | 124861 | 9988 | 0 | 0 | 1248610 | 998040 | 0 | 52.7 | 87.0 | 145.4 | 323.6 | 2161.2 | 0 | 0 | 10/10 | 25.3 | 12.2 | 2.5 | 16.42 / 15.63 |
| best | redis | 10 × wait | 124861 | 9988 | 0 | 0 | 1248610 | 998040 | 0 | 52.7 | 87.0 | 145.4 | 323.6 | 2161.2 | 0 | 0 | 10/10 | 25.3 | 12.2 | 2.5 | 16.42 / 15.63 |

Run 1's profile:

| profile: the window, 10.0 s, 99804 messages, 998045 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 20.23G | 202.7k | 20.27k | 97.63k | 10.50k | 2.078G | 19.80k | 1980 |
| waiting for a CPU, ns | - | - | - | 4.642 | 28.85 | - | - | - |
| allocations | 5.765M | 57.77 | 5.777 | 7.911 | 4.985 | 327.4k | 3.120 | 0.312 |
| reallocations | 238.0k | 2.385 | 0.238 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 238.0k | 2.385 | 0.238 | 2.000 | 0.0385 | - | - | - |
| frees | 5.765M | 57.77 | 5.777 | 7.911 | 4.985 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 773.6M | 7751 | 775.1 | 2296 | 545.5 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 0 | 0 | 0 | 0 | 0 | 37.00 | 0.000353 | 0.0000353 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 900.0k | 9.018 | 0.902 | 0 | 0.902 | - | - | - |
| involuntary context switches | 249.0 | 0.00249 | 0.000249 | 0.000451 | 0.000204 | - | - | - |
| cycles | - | - | - | 461.9k | 21.25k | - | - | - |
| instructions | - | - | - | 830.5k | 24.95k | - | - | - |
| cache references | - | - | - | 14.48 | 102.0 | - | - | - |
| cache misses | - | - | - | 6.905 | 13.51 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.190 | 0.340 | - | - | - |
| data TLB read misses | - | - | - | 0.442 | 3.849 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 8.143M | - | - | - | - | 58.32M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 8.143M | - | - | - | - | 56.71M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.234M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 10.50 s from a quarter second before the window to a quarter second after it, 104923 messages, 1049235 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/profile-record-1911/comparison/profiles/redis-591549-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/profile-record-1911/comparison/profiles/redis-591549-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 10 s after 2 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 124861 | 9988 | 0 | 0 | 12486100 | 9980400 | 0 | 720.9 | 1097.7 | 1441.8 | 1671.2 | 2765.7 | 0 | 0 | 100/100 | 42.0 | 12.2 | 7.3 | 16.41 / 16.30 |
| best | redis | 22 × wait | 124861 | 9988 | 0 | 0 | 12486100 | 9980400 | 0 | 720.9 | 1097.7 | 1441.8 | 1671.2 | 2765.7 | 0 | 0 | 100/100 | 42.0 | 12.2 | 7.3 | 16.41 / 16.30 |

Run 1's profile:

| profile: the window, 10.0 s, 99804 messages, 9980718 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 33.63G | 336.9k | 3369 | 98.23k | 2387 | 6.175G | 58.83k | 588.3 |
| waiting for a CPU, ns | - | - | - | 9.300 | 3.530 | - | - | - |
| allocations | 43.19M | 432.8 | 4.328 | 7.180 | 4.256 | 314.9k | 3.000 | 0.0300 |
| reallocations | 583.5k | 5.847 | 0.0585 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 583.4k | 5.845 | 0.0585 | 1.999 | 0.0385 | - | - | - |
| frees | 43.19M | 432.8 | 4.328 | 7.180 | 4.256 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 5.438G | 54.48k | 544.8 | 2272 | 522.1 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 14.00 | 0.000140 | 0.00000140 | 0 | 0.00000140 | 59.00 | 0.000562 | 0.00000562 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.301M | 13.04 | 0.130 | 0 | 0.130 | - | - | - |
| involuntary context switches | 487.0 | 0.00488 | 0.0000488 | 0.000521 | 0.0000436 | - | - | - |
| cycles | - | - | - | 455.3k | 8380 | - | - | - |
| instructions | - | - | - | 833.7k | 14.27k | - | - | - |
| cache references | - | - | - | 21.28 | 10.65 | - | - | - |
| cache misses | - | - | - | 9.237 | 2.183 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.268 | 0.0847 | - | - | - |
| data TLB read misses | - | - | - | 0.305 | 0.556 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 13.08M | - | - | - | - | 60.13M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 13.08M | - | - | - | - | 58.92M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 2.065M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 10.50 s from a quarter second before the window to a quarter second after it, 104959 messages, 10496858 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/profile-record-1911/comparison/profiles/redis-593827-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/profile-record-1911/comparison/profiles/redis-593827-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 10 over 10 threads, which poll without waiting; window 10 s after 2 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 10 × busy | 124861 | 9988 | 0 | 0 | 1248610 | 998040 | 0 | 53.8 | 93.2 | 147.5 | 1245.2 | 5317.9 | 0 | 0 | 10/10 | 134.9 | 12.0 | 3.0 | 24.19 / 25.62 |
| best | redis | 10 × busy | 124861 | 9988 | 0 | 0 | 1248610 | 998040 | 0 | 53.8 | 93.2 | 147.5 | 1245.2 | 5317.9 | 0 | 0 | 10/10 | 134.9 | 12.0 | 3.0 | 24.19 / 25.62 |

Run 1's profile:

| profile: the window, 10.0 s, 99804 messages, 998052 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 107.7G | 1.079M | 107.9k | 96.65k | 98.22k | 2.458G | 23.43k | 2343 |
| waiting for a CPU, ns | - | - | - | 19.11 | 38.77 | - | - | - |
| allocations | 5.743M | 57.54 | 5.754 | 7.889 | 4.965 | 318.8k | 3.038 | 0.304 |
| reallocations | 238.0k | 2.385 | 0.238 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 238.0k | 2.384 | 0.238 | 2.000 | 0.0385 | - | - | - |
| frees | 5.743M | 57.54 | 5.754 | 7.889 | 4.965 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 772.8M | 7744 | 774.4 | 2295 | 544.8 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 17.00 | 0.000170 | 0.0000170 | 0.000140 | 0.00000301 | 22.00 | 0.000210 | 0.0000210 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.000 | 0.0000100 | 0.00000100 | 0 | 0.00000100 | - | - | - |
| involuntary context switches | 1244 | 0.0125 | 0.00125 | 0.00124 | 0.00112 | - | - | - |
| cycles | - | - | - | 437.4k | 337.6k | - | - | - |
| instructions | - | - | - | 776.6k | 560.5k | - | - | - |
| cache references | - | - | - | 29.83 | 33.98 | - | - | - |
| cache misses | - | - | - | 15.36 | 8.635 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.485 | 0.163 | - | - | - |
| data TLB read misses | - | - | - | 0.396 | 0.0747 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 8.417M | - | - | - | - | 58.22M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 8.417M | - | - | - | - | 57.32M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.352M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 10.50 s from a quarter second before the window to a quarter second after it, 104916 messages, 1049184 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/profile-record-1911/comparison/profiles/redis-599248-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/profile-record-1911/comparison/profiles/redis-599248-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 10 s after 2 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 124861 | 9988 | 0 | 0 | 12480000 | 9974300 | 6100 | 1589.2 | 3899.4 | 6357.0 | 28573.7 | undelivered | 0 | 6100 | 0/100 | 241.6 | 9.7 | 7.7 | 30.74 / 32.86 |
| best | redis | 22 × busy | 124861 | 9988 | 0 | 0 | 12480000 | 9974300 | 6100 | 1589.2 | 3899.4 | 6357.0 | 28573.7 | undelivered | 0 | 6100 | 0/100 | 241.6 | 9.7 | 7.7 | 30.74 / 32.86 |

Run 1's profile:

| profile: the window, 10.0 s, 99803 messages, 9973411 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 193.5G | 1.939M | 19.40k | 79.24k | 18.61k | 6.575G | 62.59k | 626.2 |
| waiting for a CPU, ns | - | - | - | 17.79k | 3246 | - | - | - |
| allocations | 42.31M | 424.0 | 4.243 | 7.102 | 4.172 | 239.6k | 2.280 | 0.0228 |
| reallocations | 583.6k | 5.847 | 0.0585 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 579.4k | 5.805 | 0.0581 | 1.964 | 0.0384 | - | - | - |
| frees | 42.31M | 424.0 | 4.243 | 7.102 | 4.172 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 5.411G | 54.21k | 542.5 | 2270 | 519.8 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 976.0 | 0.00978 | 0.0000979 | 0.000521 | 0.0000926 | 223.0 | 0.00212 | 0.0000212 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 15.99k | 0.160 | 0.00160 | 0.0107 | 0.00150 | - | - | - |
| cycles | - | - | - | 139.1k | 64.57k | - | - | - |
| instructions | - | - | - | 243.3k | 101.0k | - | - | - |
| cache references | - | - | - | 5.724 | 3.755 | - | - | - |
| cache misses | - | - | - | 3.454 | 1.119 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0000401 | 0.0212 | - | - | - |
| data TLB read misses | - | - | - | 0.0465 | 0.0589 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 19.03M | - | - | - | - | 60.55M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 19.03M | - | - | - | - | 60.90M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 3.024M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 10.51 s from a quarter second before the window to a quarter second after it, 105049 messages, 10500852 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/profile-record-1911/comparison/profiles/redis-602342-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/profile-record-1911/comparison/profiles/redis-602342-0-redis-malloc-stats.txt
