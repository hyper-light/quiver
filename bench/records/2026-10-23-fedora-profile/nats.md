- broker: public.ecr.aws/docker/library/nats:2.12-alpine@sha256:b270f5e2428354c0335612694d7dd2fb588148e567a5757fdff325ef9c9332e6, CPUs 8-15
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 10 over 10 threads, which wait when there's nothing to take; window 10 s after 2 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 10 × wait | 124861 | 9988 | 0 | 0 | 1248610 | 998040 | 0 | 42.0 | 57.9 | 137.2 | 176.1 | 2075.5 | 0 | 0 | 10/10 | 24.5 | 12.1 | 9.9 | 17.15 / 16.42 |
| best | nats | 10 × wait | 124861 | 9988 | 0 | 0 | 1248610 | 998040 | 0 | 42.0 | 57.9 | 137.2 | 176.1 | 2075.5 | 0 | 0 | 10/10 | 24.5 | 12.1 | 9.9 | 17.15 / 16.42 |

Run 1's profile:

| profile: the window, 10.0 s, 99803 messages, 998040 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 19.59G | 196.3k | 19.63k | 97.20k | 9909 | 8.345G | 79.53k | 7953 |
| waiting for a CPU, ns | - | - | - | 29.64 | 46.01 | - | - | - |
| allocations | 1.200M | 12.02 | 1.202 | 2.000 | 1.002 | 2.113M | 20.14 | 2.014 |
| reallocations | 2.000 | 0.0000200 | 0.00000200 | 0.0000200 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 1.000 | 0.0000100 | 0.00000100 | 0.0000100 | 0 | - | - | - |
| frees | 1.200M | 12.02 | 1.202 | 2.000 | 1.002 | 2.183M | 20.80 | 2.080 |
| bytes allocated | 89.71M | 898.9 | 89.89 | 280.0 | 61.89 | 72.17M | 687.7 | 68.77 |
| minor page faults | 7.000 | 0.0000701 | 0.00000701 | 0.0000200 | 0 | 2300 | 0.0219 | 0.00219 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 837.7k | 8.393 | 0.839 | 0 | 0.839 | - | - | - |
| involuntary context switches | 514.0 | 0.00515 | 0.000515 | 0.00261 | 0.000254 | - | - | - |
| cycles | - | - | - | 478.4k | 18.38k | - | - | - |
| instructions | - | - | - | 899.4k | 17.74k | - | - | - |
| cache references | - | - | - | 15.03 | 85.69 | - | - | - |
| cache misses | - | - | - | 6.671 | 8.665 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.241 | 0.212 | - | - | - |
| data TLB read misses | - | - | - | 0.162 | 7.987 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.205 | 0.0205 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 27.00 | 0.000257 | 0.0000257 |
| collectors' pauses, ns | - | - | - | - | - | 825.7k | 7.869 | 0.787 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 9.495M | - | - | - | - | 33.52M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 9.495M | - | - | - | - | 32.41M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 2.908M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 10.50 s from a quarter second before the window to a quarter second after it, 104935 messages, 1049370 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/profile-record-1911/comparison/profiles/nats-591114-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/profile-record-1911/comparison/profiles/nats-591114-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 10 s after 2 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 124861 | 9988 | 0 | 0 | 12486100 | 9980400 | 0 | 151.6 | 266.2 | 745.5 | 1196.0 | 3615.7 | 0 | 0 | 100/100 | 81.5 | 12.1 | 48.1 | 14.77 / 16.41 |
| best | nats | 22 × wait | 124861 | 9988 | 0 | 0 | 12486100 | 9980400 | 0 | 151.6 | 266.2 | 745.5 | 1196.0 | 3615.7 | 0 | 0 | 100/100 | 81.5 | 12.1 | 48.1 | 14.77 / 16.41 |

Run 1's profile:

| profile: the window, 10.0 s, 99804 messages, 9980680 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 65.47G | 655.9k | 6559 | 96.86k | 5591 | 40.78G | 388.7k | 3886 |
| waiting for a CPU, ns | - | - | - | 63.75 | 185.5 | - | - | - |
| allocations | 10.24M | 102.6 | 1.026 | 2.000 | 1.006 | 16.67M | 158.9 | 1.589 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 10.24M | 102.6 | 1.026 | 2.000 | 1.006 | 16.65M | 158.7 | 1.587 |
| bytes allocated | 1.283G | 12.86k | 128.6 | 280.0 | 125.8 | 543.0M | 5175 | 51.75 |
| minor page faults | 75.00 | 0.000751 | 0.00000751 | 0 | 0.00000721 | 22.73k | 0.217 | 0.00217 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 4.060M | 40.68 | 0.407 | 0 | 0.407 | - | - | - |
| involuntary context switches | 15.51k | 0.155 | 0.00155 | 0.00346 | 0.00152 | - | - | - |
| cycles | - | - | - | 484.1k | 13.58k | - | - | - |
| instructions | - | - | - | 872.4k | 13.86k | - | - | - |
| cache references | - | - | - | 15.33 | 75.30 | - | - | - |
| cache misses | - | - | - | 6.428 | 7.401 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.238 | 0.107 | - | - | - |
| data TLB read misses | - | - | - | 0.195 | 5.627 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.205 | 0.00205 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 329.0 | 0.00314 | 0.0000314 |
| collectors' pauses, ns | - | - | - | - | - | refused: 329 collections in the span, past the 256 pauses Go keeps | refused | refused |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 22.02M | - | - | - | - | 39.79M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 22.02M | - | - | - | - | 39.69M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 5.292M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 10.50 s from a quarter second before the window to a quarter second after it, 104930 messages, 10493605 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/profile-record-1911/comparison/profiles/nats-593453-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/profile-record-1911/comparison/profiles/nats-593453-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 10 over 10 threads, which poll without waiting; window 10 s after 2 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 10 × busy | 124861 | 9988 | 0 | 0 | 1248610 | 998040 | 0 | 97.3 | 1024.0 | 3964.9 | 24379.4 | 109820.2 | 0 | 0 | 10/10 | 127.4 | 11.2 | 7.2 | 19.22 / 24.19 |
| best | nats | 10 × busy | 124861 | 9988 | 0 | 0 | 1248610 | 998040 | 0 | 97.3 | 1024.0 | 3964.9 | 24379.4 | 109820.2 | 0 | 0 | 10/10 | 127.4 | 11.2 | 7.2 | 19.22 / 24.19 |

Run 1's profile:

| profile: the window, 10.0 s, 99804 messages, 998054 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 100.1G | 1.003M | 100.3k | 88.57k | 91.44k | 5.541G | 52.80k | 5280 |
| waiting for a CPU, ns | - | - | - | 7547 | 7175 | - | - | - |
| allocations | 1.206M | 12.08 | 1.208 | 2.000 | 1.008 | 1.648M | 15.70 | 1.570 |
| reallocations | 5.000 | 0.0000501 | 0.00000501 | 0.0000501 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 3.000 | 0.0000301 | 0.00000301 | 0.0000301 | 0 | - | - | - |
| frees | 1.206M | 12.08 | 1.208 | 2.000 | 1.008 | 1.632M | 15.55 | 1.555 |
| bytes allocated | 187.6M | 1879 | 187.9 | 280.8 | 159.9 | 89.91M | 856.7 | 85.67 |
| minor page faults | 227.0 | 0.00227 | 0.000227 | 0.000351 | 0.000189 | 3086 | 0.0294 | 0.00294 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.000 | 0.0000301 | 0.00000301 | 0 | 0.00000301 | - | - | - |
| involuntary context switches | 10.60k | 0.106 | 0.0106 | 0.00857 | 0.00976 | - | - | - |
| cycles | - | - | - | 168.7k | 282.7k | - | - | - |
| instructions | - | - | - | 306.0k | 462.7k | - | - | - |
| cache references | - | - | - | 28.18 | 55.90 | - | - | - |
| cache misses | - | - | - | 17.07 | 17.69 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0000501 | 0.252 | - | - | - |
| data TLB read misses | - | - | - | 0.118 | 0.384 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.205 | 0.0205 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 40.00 | 0.000381 | 0.0000381 |
| collectors' pauses, ns | - | - | - | - | - | 8.063M | 76.82 | 7.682 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 10.45M | - | - | - | - | 32.78M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 10.45M | - | - | - | - | 31.90M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 4.751M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 10.50 s from a quarter second before the window to a quarter second after it, 104959 messages, 1049580 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/profile-record-1911/comparison/profiles/nats-596443-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/profile-record-1911/comparison/profiles/nats-596443-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 10 s after 2 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 124861 | 9988 | 0 | 0 | 12486100 | 9980400 | 0 | 258.0 | 1900.5 | 3702.8 | 6357.0 | 35866.1 | 0 | 0 | 100/100 | 239.2 | 11.6 | 31.8 | 29.54 / 30.74 |
| best | nats | 22 × busy | 124861 | 9988 | 0 | 0 | 12486100 | 9980400 | 0 | 258.0 | 1900.5 | 3702.8 | 6357.0 | 35866.1 | 0 | 0 | 100/100 | 239.2 | 11.6 | 31.8 | 29.54 / 30.74 |

Run 1's profile:

| profile: the window, 10.0 s, 99803 messages, 9980742 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 191.2G | 1.916M | 19.16k | 93.01k | 18.23k | 26.72G | 254.6k | 2546 |
| waiting for a CPU, ns | - | - | - | 3351 | 3123 | - | - | - |
| allocations | 10.30M | 103.2 | 1.032 | 2.000 | 1.012 | 12.21M | 116.3 | 1.163 |
| reallocations | 6.000 | 0.0000601 | 0.000000601 | 0 | 0.000000601 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 6.000 | 0.0000601 | 0.000000601 | 0 | 0.000000601 | - | - | - |
| frees | 10.30M | 103.2 | 1.032 | 2.000 | 1.012 | 12.21M | 116.3 | 1.163 |
| bytes allocated | 2.195G | 21.99k | 219.9 | 280.0 | 217.1 | 545.7M | 5200 | 52.00 |
| minor page faults | 482.0 | 0.00483 | 0.0000483 | 0 | 0.0000480 | 30.17k | 0.287 | 0.00287 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 23.85k | 0.239 | 0.00239 | 0.00400 | 0.00235 | - | - | - |
| cycles | - | - | - | 174.5k | 58.59k | - | - | - |
| instructions | - | - | - | 313.9k | 88.27k | - | - | - |
| cache references | - | - | - | 4.897 | 3.503 | - | - | - |
| cache misses | - | - | - | 2.794 | 1.398 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.0159 | - | - | - |
| data TLB read misses | - | - | - | 0.0311 | 0.0265 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.205 | 0.00205 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 284.0 | 0.00271 | 0.0000271 |
| collectors' pauses, ns | - | - | - | - | - | refused: 284 collections in the span, past the 256 pauses Go keeps | refused | refused |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 25.80M | - | - | - | - | 40.78M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 25.80M | - | - | - | - | 39.37M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 6.513M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 10.50 s from a quarter second before the window to a quarter second after it, 104948 messages, 10494517 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/profile-record-1911/comparison/profiles/nats-601992-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/profile-record-1911/comparison/profiles/nats-601992-0-nats-allocs.pb.gz
