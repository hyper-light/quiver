- broker: public.ecr.aws/docker/library/nats:2.12-alpine@sha256:b270f5e2428354c0335612694d7dd2fb588148e567a5757fdff325ef9c9332e6, CPUs 8-15
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 22.0 | 58.4 | 113.7 | 165.9 | 304.3 | 0 | 0 | 1/1 | 37.0 | 34.6 | 5.0 | 2.61 / 2.02 |
| best | nats | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 22.0 | 58.4 | 113.7 | 165.9 | 304.3 | 0 | 0 | 1/1 | 37.0 | 34.6 | 5.0 | 2.61 / 2.02 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 299498 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 31.33G | 104.6k | 104.6k | 97.59k | 7005 | 4.282G | 14.06k | 14.06k |
| waiting for a CPU, ns | - | - | - | 1.983 | 5.987 | - | - | - |
| allocations | 898.9k | 3.001 | 3.001 | 2.000 | 1.001 | 919.6k | 3.019 | 3.019 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 898.9k | 3.001 | 3.001 | 2.000 | 1.001 | 947.8k | 3.111 | 3.111 |
| bytes allocated | 97.57M | 325.8 | 325.8 | 280.0 | 45.79 | 68.88M | 226.1 | 226.1 |
| minor page faults | 3.000 | 0.0000100 | 0.0000100 | 0 | 0 | 2340 | 0.00768 | 0.00768 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 261.9k | 0.875 | 0.875 | 0 | 0.875 | - | - | - |
| involuntary context switches | 143.0 | 0.000477 | 0.000477 | 0.000337 | 0.000140 | - | - | - |
| cycles | - | - | - | 367.9k | 12.52k | - | - | - |
| instructions | - | - | - | 715.6k | 15.02k | - | - | - |
| cache references | - | - | - | 1.796 | 24.14 | - | - | - |
| cache misses | - | - | - | 0.449 | 2.484 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0337 | 0.0399 | - | - | - |
| data TLB read misses | - | - | - | 0.0501 | 0.453 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.0137 | 0.0137 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 23.00 | 0.0000755 | 0.0000755 |
| collectors' pauses, ns | - | - | - | - | - | 659.8k | 2.166 | 2.166 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.406M | - | - | - | - | 31.49M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.406M | - | - | - | - | 30.47M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 2.490M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304617 messages, 304621 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-288393-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-288393-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 153.6 | 274.4 | 770.0 | 1228.8 | 3531.0 | 0 | 0 | 100/100 | 201.0 | 34.3 | 123.0 | 6.77 / 10.93 |
| best | nats | 22 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 153.6 | 274.4 | 770.0 | 1228.8 | 3531.0 | 0 | 0 | 100/100 | 201.0 | 34.3 | 123.0 | 6.77 / 10.93 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 29949424 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 170.0G | 567.6k | 5676 | 96.75k | 4709 | 105.8G | 347.3k | 3473 |
| waiting for a CPU, ns | - | - | - | 23.43 | 23.68 | - | - | - |
| allocations | 30.77M | 102.7 | 1.027 | 2.000 | 1.007 | 45.89M | 150.7 | 1.507 |
| reallocations | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | - | - | - |
| frees | 30.77M | 102.7 | 1.027 | 2.000 | 1.007 | 45.87M | 150.6 | 1.506 |
| bytes allocated | 4.375G | 14.61k | 146.1 | 280.0 | 143.3 | 1.545G | 5071 | 50.71 |
| minor page faults | 76.00 | 0.000254 | 0.00000254 | 0 | 0.00000240 | 60.16k | 0.197 | 0.00197 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 12.53M | 41.84 | 0.418 | 0 | 0.418 | - | - | - |
| involuntary context switches | 18.95k | 0.0633 | 0.000633 | 0.00220 | 0.000611 | - | - | - |
| cycles | - | - | - | 489.8k | 11.37k | - | - | - |
| instructions | - | - | - | 922.3k | 13.41k | - | - | - |
| cache references | - | - | - | 11.81 | 23.45 | - | - | - |
| cache misses | - | - | - | 2.299 | 1.105 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0441 | 0.0245 | - | - | - |
| data TLB read misses | - | - | - | 0.279 | 0.834 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.0684 | 0.000684 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 933.0 | 0.00306 | 0.0000306 |
| collectors' pauses, ns | - | - | - | - | - | refused: 933 collections in the span, past the 256 pauses Go keeps | refused | refused |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 22.40M | - | - | - | - | 40.21M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 22.40M | - | - | - | - | 39.91M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 5.218M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304621 messages, 30462514 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-293505-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-293505-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 116045 | 3169 | 238545 | 0 | 116045000 | 65981000 | 233516000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 228.3 | 2.5 | 195.3 | 8.52 / 12.40 |
| best | nats | 22 × wait | 116045 | 3169 | 238545 | 0 | 116045000 | 65981000 | 233516000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 228.3 | 2.5 | 195.3 | 8.52 / 12.40 |

Run 1's profile:

| profile: the window, 30.0 s, 86640 messages, 83966133 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 163.1G | 1.883M | 1943 | 461.4 | 1943 | 143.9G | 1.661M | 1686 |
| waiting for a CPU, ns | - | - | - | 0.150 | 3.997 | - | - | - |
| allocations | 85.33M | 984.9 | 1.016 | 2.016 | 1.014 | 70.03M | 808.3 | 0.820 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 85.33M | 984.9 | 1.016 | 2.016 | 1.014 | 70.04M | 808.4 | 0.820 |
| bytes allocated | 21.49G | 248.0k | 255.9 | 362.9 | 255.5 | 3.564G | 41.14k | 41.75 |
| minor page faults | 12.00 | 0.000139 | 0.000000143 | 0 | 0.000000095 | 9096 | 0.105 | 0.000107 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 12.15M | 140.2 | 0.145 | 0.000185 | 0.145 | - | - | - |
| involuntary context switches | 10.06k | 0.116 | 0.000120 | 0.0000115 | 0.000120 | - | - | - |
| cycles | - | - | - | 1644 | 6037 | - | - | - |
| instructions | - | - | - | 3536 | 8400 | - | - | - |
| cache references | - | - | - | 6.091 | 34.98 | - | - | - |
| cache misses | - | - | - | 2.722 | 20.33 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.179 | 0.709 | - | - | - |
| data TLB read misses | - | - | - | 0.0256 | 1.313 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 53.25k | 0.615 | 0.000634 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 281.0 | 0.00324 | 0.00000329 |
| collectors' pauses, ns | - | - | - | - | - | refused: 281 collections in the span, past the 256 pauses Go keeps | refused | refused |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 119.8M | - | - | - | - | 115.4M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 119.8M | - | - | - | - | 113.1M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 28.21M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 86640 messages, 85385617 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-302653-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-302653-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 21.5 | 29.2 | 60.4 | 167.9 | 1379.0 | 0 | 0 | 1/1 | 38.9 | 28.2 | 22.6 | 3.72 / 4.63 |
| best | nats | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 21.5 | 29.2 | 60.4 | 167.9 | 1379.0 | 0 | 0 | 1/1 | 38.9 | 28.2 | 22.6 | 3.72 / 4.63 |

Run 1's profile:

| profile: the window, 30.0 s, 2998268 messages, 2998272 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 32.82G | 10.95k | 10.95k | 7973 | 2974 | 19.32G | 6338 | 6338 |
| waiting for a CPU, ns | - | - | - | 0.823 | 2.635 | - | - | - |
| allocations | 9.012M | 3.006 | 3.006 | 2.000 | 1.006 | 8.844M | 2.901 | 2.901 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 9.012M | 3.006 | 3.006 | 2.000 | 1.006 | 8.863M | 2.907 | 2.907 |
| bytes allocated | 1.189G | 396.5 | 396.5 | 280.0 | 116.5 | 1.035G | 339.5 | 339.5 |
| minor page faults | 5.000 | 0.00000167 | 0.00000167 | 0 | 0 | 20.99k | 0.00689 | 0.00689 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.546M | 0.516 | 0.516 | 0 | 0.516 | - | - | - |
| involuntary context switches | 789.0 | 0.000263 | 0.000263 | 0.000149 | 0.000114 | - | - | - |
| cycles | - | - | - | 40.69k | 7762 | - | - | - |
| instructions | - | - | - | 71.19k | 10.26k | - | - | - |
| cache references | - | - | - | 14.28 | 3.573 | - | - | - |
| cache misses | - | - | - | 2.789 | 0.590 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00987 | 0.00917 | - | - | - |
| data TLB read misses | - | - | - | 1.180 | 0.0801 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.00137 | 0.00137 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 431.0 | 0.000141 | 0.000141 |
| collectors' pauses, ns | - | - | - | - | - | refused: 431 collections in the span, past the 256 pauses Go keeps | refused | refused |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.869M | - | - | - | - | 38.87M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.869M | - | - | - | - | 36.59M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 5.751M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3048352 messages, 3048354 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-328126-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-328126-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 925649 | 25899 | 2621894 | 0 | 92564900 | 42627800 | 257199600 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 218.5 | 0.9 | 163.4 | 14.35 / 16.42 |
| best | nats | 22 × wait | 925649 | 25899 | 2621894 | 0 | 92564900 | 42627800 | 257199600 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 218.5 | 0.9 | 163.4 | 14.35 / 16.42 |

Run 1's profile:

| profile: the window, 30.0 s, 777248 messages, 77202219 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 181.6G | 233.7k | 2353 | 589.8 | 2347 | 138.3G | 175.5k | 1762 |
| waiting for a CPU, ns | - | - | - | 1.958 | 45.09 | - | - | - |
| allocations | 79.82M | 102.7 | 1.034 | 2.016 | 1.014 | 71.52M | 90.76 | 0.911 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 79.82M | 102.7 | 1.034 | 2.016 | 1.014 | 71.54M | 90.78 | 0.911 |
| bytes allocated | 19.24G | 24.75k | 249.2 | 363.8 | 245.5 | 3.383G | 4293 | 43.10 |
| minor page faults | 215.0 | 0.000277 | 0.00000278 | 0 | 0.00000275 | 39.36k | 0.0499 | 0.000501 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 13.35M | 17.18 | 0.173 | 0.000189 | 0.173 | - | - | - |
| involuntary context switches | 43.93k | 0.0565 | 0.000569 | 0.0000566 | 0.000568 | - | - | - |
| cycles | - | - | - | 3692 | 6456 | - | - | - |
| instructions | - | - | - | 7671 | 9056 | - | - | - |
| cache references | - | - | - | 13.90 | 25.00 | - | - | - |
| cache misses | - | - | - | 6.214 | 1.473 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.271 | 0.0201 | - | - | - |
| data TLB read misses | - | - | - | 0.0726 | 0.952 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.0263 | 0.000265 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 863.0 | 0.00110 | 0.0000110 |
| collectors' pauses, ns | - | - | - | - | - | refused: 863 collections in the span, past the 256 pauses Go keeps | refused | refused |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 23.99M | - | - | - | - | 48.44M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 23.99M | - | - | - | - | 47.82M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 9.200M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 788048 messages, 78493556 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-360692-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-360692-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 105480 | 2885 | 3442063 | 0 | 105480000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 226.6 | 0.4 | 195.8 | 16.91 / 16.87 |
| best | nats | 22 × wait | 105480 | 2885 | 3442063 | 0 | 105480000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 226.6 | 0.4 | 195.8 | 16.91 / 16.87 |

Run 1's profile:

| profile: the window, 30.0 s, 76736 messages, 76521000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 165.2G | 2.152M | 2159 | 566.7 | 2158 | 145.0G | 1.890M | 1863 |
| waiting for a CPU, ns | - | - | - | 0.0979 | 19.66 | - | - | - |
| allocations | 77.76M | 1013 | 1.016 | 2.017 | 1.014 | 64.00M | 834.0 | 0.822 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 77.76M | 1013 | 1.016 | 2.017 | 1.014 | 63.89M | 832.6 | 0.821 |
| bytes allocated | 19.55G | 254.8k | 255.5 | 365.1 | 255.1 | 3.300G | 43.01k | 42.39 |
| minor page faults | 500.0 | 0.00652 | 0.00000653 | 0 | 0.00000648 | 5933 | 0.0773 | 0.0000762 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 9.459M | 123.3 | 0.124 | 0.000195 | 0.124 | - | - | - |
| involuntary context switches | 25.71k | 0.335 | 0.000336 | 0.0000130 | 0.000336 | - | - | - |
| cycles | - | - | - | 3415 | 6574 | - | - | - |
| instructions | - | - | - | 6942 | 8483 | - | - | - |
| cache references | - | - | - | 14.16 | 54.61 | - | - | - |
| cache misses | - | - | - | 7.192 | 27.94 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.222 | 1.094 | - | - | - |
| data TLB read misses | - | - | - | 0.0562 | 3.041 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.0534 | 0.0000535 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 232.0 | 0.00302 | 0.00000298 |
| collectors' pauses, ns | - | - | - | - | - | 16.21M | 211.2 | 0.208 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 121.8M | - | - | - | - | 117.2M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 121.8M | - | - | - | - | 117.0M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 31.85M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.52 s from a quarter second before the window to a quarter second after it, 76736 messages, 77842014 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-427016-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-427016-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 17.2 | 45.1 | 71.7 | 104.4 | 325.1 | 0 | 0 | 1/1 | 69.8 | 34.6 | 5.1 | 4.09 / 3.22 |
| best | nats | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 17.2 | 45.1 | 71.7 | 104.4 | 325.1 | 0 | 0 | 1/1 | 69.8 | 34.6 | 5.1 | 4.09 / 3.22 |

Run 1's profile:

| profile: the window, 30.0 s, 299500 messages, 299500 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 58.90G | 196.7k | 196.7k | 97.64k | 99.01k | 4.376G | 14.36k | 14.36k |
| waiting for a CPU, ns | - | - | - | 1.933 | 7.797 | - | - | - |
| allocations | 898.8k | 3.001 | 3.001 | 2.000 | 1.001 | 938.7k | 3.082 | 3.082 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 898.8k | 3.001 | 3.001 | 2.000 | 1.001 | 954.8k | 3.134 | 3.134 |
| bytes allocated | 95.93M | 320.3 | 320.3 | 280.0 | 40.31 | 70.66M | 232.0 | 232.0 |
| minor page faults | 4.000 | 0.0000134 | 0.0000134 | 0 | 0 | 1210 | 0.00397 | 0.00397 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 338.0 | 0.00113 | 0.00113 | 0.000307 | 0.000821 | - | - | - |
| cycles | - | - | - | 366.3k | 398.7k | - | - | - |
| instructions | - | - | - | 709.5k | 690.3k | - | - | - |
| cache references | - | - | - | 2.660 | 1.637 | - | - | - |
| cache misses | - | - | - | 0.389 | 0.371 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0355 | 0.0299 | - | - | - |
| data TLB read misses | - | - | - | 0.0634 | 0.00197 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.0137 | 0.0137 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 21.00 | 0.0000689 | 0.0000689 |
| collectors' pauses, ns | - | - | - | - | - | 565.9k | 1.858 | 1.858 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.857M | - | - | - | - | 39.93M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.857M | - | - | - | - | 40.96M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 10.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304615 messages, 304615 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-439161-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-439161-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 184.3 | 340.0 | 901.1 | 1572.9 | 11368.4 | 0 | 0 | 100/100 | 764.4 | 33.3 | 119.4 | 21.75 / 25.13 |
| best | nats | 22 × busy | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 184.3 | 340.0 | 901.1 | 1572.9 | 11368.4 | 0 | 0 | 100/100 | 764.4 | 33.3 | 119.4 | 21.75 / 25.13 |

Run 1's profile:

| profile: the window, 30.0 s, 299500 messages, 29949808 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 645.4G | 2.155M | 21.55k | 93.97k | 20.61k | 102.8G | 337.5k | 3375 |
| waiting for a CPU, ns | - | - | - | 2.741 | 10.63 | - | - | - |
| allocations | 30.81M | 102.9 | 1.029 | 2.000 | 1.009 | 42.44M | 139.3 | 1.393 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 30.81M | 102.9 | 1.029 | 2.000 | 1.009 | 42.44M | 139.3 | 1.393 |
| bytes allocated | 4.989G | 16.66k | 166.6 | 280.0 | 163.8 | 1.496G | 4910 | 49.10 |
| minor page faults | 51.00 | 0.000170 | 0.00000170 | 0 | 0.00000160 | 20.35k | 0.0668 | 0.000668 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.000 | 0.00000334 | 0.000000033 | 0 | 0 | - | - | - |
| involuntary context switches | 8968 | 0.0299 | 0.000299 | 0.000424 | 0.000295 | - | - | - |
| cycles | - | - | - | 172.7k | 48.07k | - | - | - |
| instructions | - | - | - | 321.2k | 72.81k | - | - | - |
| cache references | - | - | - | 0.574 | 0.753 | - | - | - |
| cache misses | - | - | - | 0.331 | 0.138 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.00137 | - | - | - |
| data TLB read misses | - | - | - | 0.000357 | 0.00760 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.0137 | 0.000137 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 338.0 | 0.00111 | 0.0000111 |
| collectors' pauses, ns | - | - | - | - | - | refused: 338 collections in the span, past the 256 pauses Go keeps | refused | refused |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 22.29M | - | - | - | - | 50.05M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 22.29M | - | - | - | - | 48.77M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 12.16M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304633 messages, 30463130 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-444511-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-444511-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 110801 | 2992 | 243789 | 0 | 110801000 | 60737000 | 238760000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 913.9 | 2.9 | 207.3 | 26.96 / 28.66 |
| best | nats | 22 × busy | 110801 | 2992 | 243789 | 0 | 110801000 | 60737000 | 238760000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 913.9 | 2.9 | 207.3 | 26.96 / 28.66 |

Run 1's profile:

| profile: the window, 30.0 s, 76960 messages, 76135929 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 624.4G | 8.113M | 8201 | 613.9 | 8201 | 144.4G | 1.876M | 1865 |
| waiting for a CPU, ns | - | - | - | 765.5 | 14.51 | - | - | - |
| allocations | 77.36M | 1005 | 1.016 | 2.016 | 1.014 | 65.75M | 854.3 | 0.849 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 77.36M | 1005 | 1.016 | 2.016 | 1.014 | 65.72M | 853.9 | 0.849 |
| bytes allocated | 19.33G | 251.2k | 253.9 | 365.0 | 253.6 | 3.351G | 43.54k | 43.28 |
| minor page faults | 1236 | 0.0161 | 0.0000162 | 0 | 0.0000162 | 13.20k | 0.172 | 0.000171 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 16.00 | 0.000208 | 0.000000210 | 0.000195 | 0 | - | - | - |
| involuntary context switches | 16.77k | 0.218 | 0.000220 | 0.000169 | 0.000220 | - | - | - |
| cycles | - | - | - | 2041 | 20.12k | - | - | - |
| instructions | - | - | - | 3536 | 31.63k | - | - | - |
| cache references | - | - | - | 6.570 | 10.82 | - | - | - |
| cache misses | - | - | - | 3.467 | 7.776 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.213 | 0.102 | - | - | - |
| data TLB read misses | - | - | - | 0.0326 | 0.120 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.0532 | 0.0000538 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 211.0 | 0.00274 | 0.00000273 |
| collectors' pauses, ns | - | - | - | - | - | 18.11M | 235.4 | 0.234 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 125.3M | - | - | - | - | 126.6M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 125.3M | - | - | - | - | 117.7M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 31.43M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 76960 messages, 77413659 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-450791-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-450791-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 17.2 | 22.0 | 43.0 | 101.4 | 556.2 | 0 | 0 | 1/1 | 61.3 | 28.3 | 22.3 | 17.23 / 10.91 |
| best | nats | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 17.2 | 22.0 | 43.0 | 101.4 | 556.2 | 0 | 0 | 1/1 | 61.3 | 28.3 | 22.3 | 17.23 / 10.91 |

Run 1's profile:

| profile: the window, 30.0 s, 2998274 messages, 2998275 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 51.77G | 17.27k | 17.27k | 7967 | 9301 | 19.17G | 6290 | 6290 |
| waiting for a CPU, ns | - | - | - | 0.829 | 0.762 | - | - | - |
| allocations | 9.011M | 3.005 | 3.005 | 2.000 | 1.005 | 8.871M | 2.910 | 2.910 |
| reallocations | 1.000 | 0.000000334 | 0.000000334 | 0.000000334 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 9.011M | 3.005 | 3.005 | 2.000 | 1.005 | 8.879M | 2.913 | 2.913 |
| bytes allocated | 1.166G | 389.0 | 389.0 | 280.0 | 109.0 | 1.020G | 334.5 | 334.5 |
| minor page faults | 6.000 | 0.00000200 | 0.00000200 | 0.000000667 | 0 | 4725 | 0.00155 | 0.00155 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 683.0 | 0.000228 | 0.000228 | 0.000138 | 0.0000894 | - | - | - |
| cycles | - | - | - | 39.91k | 35.96k | - | - | - |
| instructions | - | - | - | 70.71k | 62.61k | - | - | - |
| cache references | - | - | - | 2.977 | 0.276 | - | - | - |
| cache misses | - | - | - | 0.696 | 0.0483 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00356 | 0.00293 | - | - | - |
| data TLB read misses | - | - | - | 0.117 | 0.00221 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.00683 | 0.00683 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 242.0 | 0.0000794 | 0.0000794 |
| collectors' pauses, ns | - | - | - | - | - | 8.794M | 2.885 | 2.885 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.873M | - | - | - | - | 42.09M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.873M | - | - | - | - | 40.27M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 9.167M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3048275 messages, 3048277 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-461597-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-461597-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 934814 | 26132 | 2612729 | 0 | 93481400 | 43544300 | 256283100 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 740.9 | 1.0 | 167.4 | 22.48 / 25.75 |
| best | nats | 22 × busy | 934814 | 26132 | 2612729 | 0 | 93481400 | 43544300 | 256283100 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 740.9 | 1.0 | 167.4 | 22.48 / 25.75 |

Run 1's profile:

| profile: the window, 30.0 s, 772752 messages, 76968858 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 611.3G | 791.1k | 7943 | 618.5 | 7937 | 140.7G | 179.4k | 1797 |
| waiting for a CPU, ns | - | - | - | 509.1 | 17.79 | - | - | - |
| allocations | 79.57M | 103.0 | 1.034 | 2.016 | 1.014 | 71.48M | 91.17 | 0.913 |
| reallocations | 6.000 | 0.00000776 | 0.000000078 | 0 | 0.000000078 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 6.000 | 0.00000776 | 0.000000078 | 0 | 0.000000078 | - | - | - |
| frees | 79.57M | 103.0 | 1.034 | 2.016 | 1.014 | 71.54M | 91.25 | 0.914 |
| bytes allocated | 19.17G | 24.81k | 249.1 | 363.1 | 245.5 | 3.426G | 4370 | 43.77 |
| minor page faults | 540.0 | 0.000699 | 0.00000702 | 0 | 0.00000696 | 25.22k | 0.0322 | 0.000322 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 144.0 | 0.000186 | 0.00000187 | 0.000186 | 0 | - | - | - |
| involuntary context switches | 19.77k | 0.0256 | 0.000257 | 0.000136 | 0.000256 | - | - | - |
| cycles | - | - | - | 1701 | 17.59k | - | - | - |
| instructions | - | - | - | 3536 | 27.48k | - | - | - |
| cache references | - | - | - | 6.219 | 0.547 | - | - | - |
| cache misses | - | - | - | 2.848 | 0.0693 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00222 | 0.000613 | - | - | - |
| data TLB read misses | - | - | - | 0.00660 | 0.00839 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.0265 | 0.000266 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 575.0 | 0.000733 | 0.00000735 |
| collectors' pauses, ns | - | - | - | - | - | refused: 575 collections in the span, past the 256 pauses Go keeps | refused | refused |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 26.69M | - | - | - | - | 62.89M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 26.69M | - | - | - | - | 55.09M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 12.00M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 784000 messages, 78270545 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-471972-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-471972-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 98259 | 2735 | 3449284 | 0 | 98259000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 839.5 | 0.4 | 190.4 | 26.48 / 28.86 |
| best | nats | 22 × busy | 98259 | 2735 | 3449284 | 0 | 98259000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 839.5 | 0.4 | 190.4 | 26.48 / 28.86 |

Run 1's profile:

| profile: the window, 30.0 s, 70432 messages, 72972563 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 622.2G | 8.835M | 8527 | 678.5 | 8526 | 143.6G | 2.039M | 1939 |
| waiting for a CPU, ns | - | - | - | 679.6 | 47.85 | - | - | - |
| allocations | 74.13M | 1052 | 1.016 | 2.016 | 1.014 | 63.13M | 896.3 | 0.853 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 74.13M | 1052 | 1.016 | 2.016 | 1.014 | 63.02M | 894.7 | 0.851 |
| bytes allocated | 18.35G | 260.5k | 251.4 | 362.7 | 251.1 | 3.170G | 45.01k | 42.81 |
| minor page faults | 6902 | 0.0980 | 0.0000946 | 0 | 0.0000945 | 13.23k | 0.188 | 0.000179 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 14.00 | 0.000199 | 0.000000192 | 0.000185 | 0 | - | - | - |
| involuntary context switches | 27.04k | 0.384 | 0.000370 | 0.000227 | 0.000370 | - | - | - |
| cycles | - | - | - | 1753 | 31.75k | - | - | - |
| instructions | - | - | - | 3535 | 47.74k | - | - | - |
| cache references | - | - | - | 8.414 | 28.52 | - | - | - |
| cache misses | - | - | - | 5.040 | 17.41 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00281 | 0.486 | - | - | - |
| data TLB read misses | - | - | - | 0.0293 | 0.957 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.291 | 0.000281 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 194.0 | 0.00275 | 0.00000262 |
| collectors' pauses, ns | - | - | - | - | - | 17.93M | 254.6 | 0.242 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 127.5M | - | - | - | - | 128.6M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 125.3M | - | - | - | - | 121.5M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 33.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 70432 messages, 74049460 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-488110-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-488110-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 21.8 | 57.9 | 111.6 | 157.7 | 416.4 | 0 | 0 | 1/1 | 37.1 | 34.6 | 5.0 | 9.41 / 6.20 |
| best | nats | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 21.8 | 57.9 | 111.6 | 157.7 | 416.4 | 0 | 0 | 1/1 | 37.1 | 34.6 | 5.0 | 9.41 / 6.20 |

Run 1's profile:

| profile: the window, 30.0 s, 299586 messages, 299586 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 31.32G | 104.6k | 104.6k | 97.58k | 6982 | 4.319G | 14.18k | 14.18k |
| waiting for a CPU, ns | - | - | - | 3.617 | 4.787 | - | - | - |
| allocations | 899.2k | 3.002 | 3.002 | 2.000 | 1.002 | 947.8k | 3.112 | 3.112 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 899.2k | 3.002 | 3.002 | 2.000 | 1.002 | 1.006M | 3.303 | 3.303 |
| bytes allocated | 98.32M | 328.2 | 328.2 | 280.0 | 48.19 | 70.71M | 232.1 | 232.1 |
| minor page faults | 5.000 | 0.0000167 | 0.0000167 | 0 | 0 | 1145 | 0.00376 | 0.00376 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 262.8k | 0.877 | 0.877 | 0 | 0.877 | - | - | - |
| involuntary context switches | 177.0 | 0.000591 | 0.000591 | 0.000494 | 0.0000968 | - | - | - |
| cycles | - | - | - | 366.0k | 12.59k | - | - | - |
| instructions | - | - | - | 712.1k | 14.76k | - | - | - |
| cache references | - | - | - | 1.674 | 19.17 | - | - | - |
| cache misses | - | - | - | 0.290 | 1.380 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0205 | 0.0243 | - | - | - |
| data TLB read misses | - | - | - | 0.0893 | 0.347 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.0684 | 0.0684 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 15.00 | 0.0000492 | 0.0000492 |
| collectors' pauses, ns | - | - | - | - | - | 418.0k | 1.372 | 1.372 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.816M | - | - | - | - | 46.37M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.816M | - | - | - | - | 43.70M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 9.396M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304594 messages, 304594 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-501059-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-501059-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 151.6 | 260.1 | 737.3 | 1261.6 | 7918.1 | 0 | 0 | 100/100 | 200.2 | 34.4 | 121.2 | 6.04 / 9.15 |
| best | nats | 22 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 151.6 | 260.1 | 737.3 | 1261.6 | 7918.1 | 0 | 0 | 100/100 | 200.2 | 34.4 | 121.2 | 6.04 / 9.15 |

Run 1's profile:

| profile: the window, 30.0 s, 299584 messages, 29958100 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 169.2G | 564.7k | 5647 | 97.09k | 4676 | 104.3G | 342.3k | 3423 |
| waiting for a CPU, ns | - | - | - | 8.077 | 20.19 | - | - | - |
| allocations | 30.77M | 102.7 | 1.027 | 2.000 | 1.007 | 45.69M | 150.0 | 1.500 |
| reallocations | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 30.77M | 102.7 | 1.027 | 2.000 | 1.007 | 45.66M | 149.9 | 1.499 |
| bytes allocated | 4.330G | 14.45k | 144.5 | 280.0 | 141.7 | 1.493G | 4903 | 49.03 |
| minor page faults | 124.0 | 0.000414 | 0.00000414 | 0.00000334 | 0.00000401 | 6358 | 0.0209 | 0.000209 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 12.30M | 41.05 | 0.411 | 0 | 0.411 | - | - | - |
| involuntary context switches | 17.85k | 0.0596 | 0.000596 | 0.000841 | 0.000587 | - | - | - |
| cycles | - | - | - | 488.1k | 11.39k | - | - | - |
| instructions | - | - | - | 922.9k | 13.32k | - | - | - |
| cache references | - | - | - | 12.98 | 21.47 | - | - | - |
| cache misses | - | - | - | 2.273 | 1.139 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0292 | 0.0251 | - | - | - |
| data TLB read misses | - | - | - | 0.841 | 0.754 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.0684 | 0.000684 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 235.0 | 0.000771 | 0.00000771 |
| collectors' pauses, ns | - | - | - | - | - | 9.803M | 32.18 | 0.322 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 22.46M | - | - | - | - | 55.00M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 22.46M | - | - | - | - | 53.05M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 14.54M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304620 messages, 30461800 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-508544-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-508544-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 114658 | 3177 | 240140 | 0 | 114658000 | 64507000 | 235076000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 227.9 | 2.5 | 196.2 | 9.53 / 12.27 |
| best | nats | 22 × wait | 114658 | 3177 | 240140 | 0 | 114658000 | 64507000 | 235076000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 227.9 | 2.5 | 196.2 | 9.53 / 12.27 |

Run 1's profile:

| profile: the window, 30.0 s, 83936 messages, 84125632 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 163.7G | 1.951M | 1946 | 473.5 | 1946 | 145.5G | 1.734M | 1701 |
| waiting for a CPU, ns | - | - | - | 0.113 | 10.59 | - | - | - |
| allocations | 85.50M | 1019 | 1.016 | 2.016 | 1.014 | 69.02M | 822.3 | 0.807 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 85.50M | 1019 | 1.016 | 2.016 | 1.014 | 68.97M | 821.6 | 0.806 |
| bytes allocated | 21.76G | 259.2k | 258.6 | 361.1 | 258.3 | 3.612G | 43.03k | 42.23 |
| minor page faults | 7.000 | 0.0000834 | 0.000000083 | 0 | 0.000000036 | 5668 | 0.0675 | 0.0000663 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 11.84M | 141.0 | 0.141 | 0.000179 | 0.141 | - | - | - |
| involuntary context switches | 15.93k | 0.190 | 0.000189 | 0.0000119 | 0.000189 | - | - | - |
| cycles | - | - | - | 3075 | 6146 | - | - | - |
| instructions | - | - | - | 6402 | 8428 | - | - | - |
| cache references | - | - | - | 12.46 | 40.58 | - | - | - |
| cache misses | - | - | - | 6.896 | 22.32 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.237 | 0.868 | - | - | - |
| data TLB read misses | - | - | - | 0.0508 | 1.823 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.244 | 0.000243 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 214.0 | 0.00255 | 0.00000250 |
| collectors' pauses, ns | - | - | - | - | - | 14.15M | 168.6 | 0.165 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 119.7M | - | - | - | - | 128.3M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 119.7M | - | - | - | - | 123.3M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 36.08M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 83936 messages, 85525326 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-525113-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-525113-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 21.0 | 26.4 | 50.7 | 123.9 | 834.3 | 0 | 0 | 1/1 | 38.6 | 28.2 | 21.9 | 3.01 / 2.82 |
| best | nats | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 21.0 | 26.4 | 50.7 | 123.9 | 834.3 | 0 | 0 | 1/1 | 38.6 | 28.2 | 21.9 | 3.01 / 2.82 |

Run 1's profile:

| profile: the window, 30.0 s, 3000440 messages, 3000439 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 32.55G | 10.85k | 10.85k | 7946 | 2902 | 18.85G | 6179 | 6179 |
| waiting for a CPU, ns | - | - | - | 1.131 | 1.617 | - | - | - |
| allocations | 9.018M | 3.006 | 3.006 | 2.000 | 1.006 | 8.914M | 2.922 | 2.922 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 9.018M | 3.006 | 3.006 | 2.000 | 1.006 | 8.927M | 2.926 | 2.926 |
| bytes allocated | 1.181G | 393.7 | 393.7 | 280.0 | 113.7 | 1.023G | 335.3 | 335.3 |
| minor page faults | 4.000 | 0.00000133 | 0.00000133 | 0 | 0 | 1887 | 0.000619 | 0.000619 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.619M | 0.540 | 0.540 | 0 | 0.540 | - | - | - |
| involuntary context switches | 790.0 | 0.000263 | 0.000263 | 0.000171 | 0.0000927 | - | - | - |
| cycles | - | - | - | 35.16k | 7610 | - | - | - |
| instructions | - | - | - | 57.78k | 10.03k | - | - | - |
| cache references | - | - | - | 205.3 | 2.521 | - | - | - |
| cache misses | - | - | - | 40.28 | 0.166 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00199 | 0.00194 | - | - | - |
| data TLB read misses | - | - | - | 3.888 | 0.0506 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.00683 | 0.00683 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 169.0 | 0.0000554 | 0.0000554 |
| collectors' pauses, ns | - | - | - | - | - | 6.241M | 2.046 | 2.046 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.902M | - | - | - | - | 46.81M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.902M | - | - | - | - | 44.59M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 13.84M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3050536 messages, 3050538 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-549492-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-549492-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 971319 | 27200 | 2578719 | 0 | 97131900 | 47149000 | 252894700 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 212.2 | 0.9 | 163.2 | 10.62 / 12.86 |
| best | nats | 22 × wait | 971319 | 27200 | 2578719 | 0 | 97131900 | 47149000 | 252894700 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 212.2 | 0.9 | 163.2 | 10.62 / 12.86 |

Run 1's profile:

| profile: the window, 30.0 s, 802448 messages, 80511444 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 175.9G | 219.2k | 2184 | 554.8 | 2179 | 137.9G | 168.5k | 1684 |
| waiting for a CPU, ns | - | - | - | 0.252 | 4.826 | - | - | - |
| allocations | 83.23M | 103.7 | 1.034 | 2.016 | 1.014 | 73.54M | 89.88 | 0.898 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 83.23M | 103.7 | 1.034 | 2.016 | 1.014 | 73.44M | 89.76 | 0.897 |
| bytes allocated | 20.22G | 25.19k | 251.1 | 362.4 | 247.5 | 3.511G | 4291 | 42.89 |
| minor page faults | 85.00 | 0.000106 | 0.00000106 | 0 | 0.00000102 | 14.74k | 0.0180 | 0.000180 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 14.06M | 17.53 | 0.175 | 0.000184 | 0.175 | - | - | - |
| involuntary context switches | 10.75k | 0.0134 | 0.000133 | 0.0000162 | 0.000133 | - | - | - |
| cycles | - | - | - | 3584 | 6085 | - | - | - |
| instructions | - | - | - | 7555 | 9029 | - | - | - |
| cache references | - | - | - | 12.40 | 8.337 | - | - | - |
| cache misses | - | - | - | 5.762 | 0.486 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.226 | 0.0110 | - | - | - |
| data TLB read misses | - | - | - | 0.0569 | 0.226 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.0255 | 0.000254 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 443.0 | 0.000541 | 0.00000541 |
| collectors' pauses, ns | - | - | - | - | - | refused: 443 collections in the span, past the 256 pauses Go keeps | refused | refused |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 23.56M | - | - | - | - | 57.93M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 23.56M | - | - | - | - | 56.99M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 17.77M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 818192 messages, 81851022 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-554570-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-554570-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 108878 | 3053 | 3441160 | 0 | 108878000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 220.8 | 0.4 | 193.7 | 16.42 / 15.94 |
| best | nats | 22 × wait | 108878 | 3053 | 3441160 | 0 | 108878000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 220.8 | 0.4 | 193.7 | 16.42 / 15.94 |

Run 1's profile:

| profile: the window, 30.0 s, 80336 messages, 79998109 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 161.8G | 2.014M | 2022 | 557.1 | 2022 | 144.5G | 1.799M | 1777 |
| waiting for a CPU, ns | - | - | - | 0.481 | 4.705 | - | - | - |
| allocations | 81.29M | 1012 | 1.016 | 2.016 | 1.014 | 67.40M | 838.9 | 0.829 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 81.29M | 1012 | 1.016 | 2.016 | 1.014 | 67.55M | 840.9 | 0.831 |
| bytes allocated | 20.48G | 255.0k | 256.0 | 360.4 | 255.7 | 3.440G | 42.81k | 42.30 |
| minor page faults | 68.00 | 0.000846 | 0.000000850 | 0 | 0.000000813 | 6695 | 0.0833 | 0.0000823 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 11.34M | 141.1 | 0.142 | 0.000174 | 0.142 | - | - | - |
| involuntary context switches | 9508 | 0.118 | 0.000119 | 0.0000498 | 0.000119 | - | - | - |
| cycles | - | - | - | 3441 | 6042 | - | - | - |
| instructions | - | - | - | 7052 | 8381 | - | - | - |
| cache references | - | - | - | 12.96 | 35.74 | - | - | - |
| cache misses | - | - | - | 6.179 | 20.90 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.170 | 0.757 | - | - | - |
| data TLB read misses | - | - | - | 0.0459 | 1.500 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.255 | 0.000256 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 194.0 | 0.00241 | 0.00000239 |
| collectors' pauses, ns | - | - | - | - | - | 13.11M | 163.2 | 0.161 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 120.7M | - | - | - | - | 127.4M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 120.7M | - | - | - | - | 126.6M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 28.80M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 80336 messages, 81313386 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-562495-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-562495-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 17.2 | 45.6 | 71.7 | 103.4 | 616.7 | 0 | 0 | 1/1 | 69.8 | 34.6 | 5.1 | 5.29 / 4.00 |
| best | nats | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 17.2 | 45.6 | 71.7 | 103.4 | 616.7 | 0 | 0 | 1/1 | 69.8 | 34.6 | 5.1 | 5.29 / 4.00 |

Run 1's profile:

| profile: the window, 30.0 s, 299580 messages, 299580 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 58.90G | 196.6k | 196.6k | 97.61k | 98.99k | 4.388G | 14.40k | 14.40k |
| waiting for a CPU, ns | - | - | - | 7.324 | 6.993 | - | - | - |
| allocations | 899.0k | 3.001 | 3.001 | 2.000 | 1.001 | 957.4k | 3.143 | 3.143 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 899.0k | 3.001 | 3.001 | 2.000 | 1.001 | 982.7k | 3.226 | 3.226 |
| bytes allocated | 95.30M | 318.1 | 318.1 | 280.0 | 38.11 | 71.91M | 236.1 | 236.1 |
| minor page faults | 4.000 | 0.0000134 | 0.0000134 | 0 | 0 | 1115 | 0.00366 | 0.00366 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 448.0 | 0.00150 | 0.00150 | 0.000868 | 0.000628 | - | - | - |
| cycles | - | - | - | 366.5k | 399.5k | - | - | - |
| instructions | - | - | - | 711.7k | 692.8k | - | - | - |
| cache references | - | - | - | 1.366 | 1.442 | - | - | - |
| cache misses | - | - | - | 0.189 | 0.169 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0155 | 0.0126 | - | - | - |
| data TLB read misses | - | - | - | 0.0465 | 0.00109 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.0684 | 0.0684 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 11.00 | 0.0000361 | 0.0000361 |
| collectors' pauses, ns | - | - | - | - | - | 221.0k | 0.726 | 0.726 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.951M | - | - | - | - | 50.09M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.951M | - | - | - | - | 50.25M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 16.26M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304592 messages, 304592 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-573318-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-573318-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 176.1 | 315.4 | 811.0 | 1376.3 | 6768.0 | 0 | 0 | 100/100 | 769.4 | 34.0 | 120.1 | 18.83 / 24.41 |
| best | nats | 22 × busy | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 176.1 | 315.4 | 811.0 | 1376.3 | 6768.0 | 0 | 0 | 100/100 | 769.4 | 34.0 | 120.1 | 18.83 / 24.41 |

Run 1's profile:

| profile: the window, 30.0 s, 299587 messages, 29958313 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 649.5G | 2.168M | 21.68k | 95.81k | 20.72k | 103.3G | 339.3k | 3393 |
| waiting for a CPU, ns | - | - | - | 14.70 | 3.796 | - | - | - |
| allocations | 30.81M | 102.8 | 1.028 | 2.000 | 1.008 | 42.73M | 140.3 | 1.403 |
| reallocations | 2.000 | 0.00000668 | 0.000000067 | 0.00000668 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | - | - | - |
| frees | 30.81M | 102.8 | 1.028 | 2.000 | 1.008 | 42.81M | 140.6 | 1.406 |
| bytes allocated | 4.860G | 16.22k | 162.2 | 280.0 | 159.4 | 1.472G | 4833 | 48.33 |
| minor page faults | 87.00 | 0.000290 | 0.00000290 | 0.0000100 | 0.00000267 | 3313 | 0.0109 | 0.000109 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 3864 | 0.0129 | 0.000129 | 0.000274 | 0.000126 | - | - | - |
| cycles | - | - | - | 258.5k | 48.23k | - | - | - |
| instructions | - | - | - | 400.3k | 74.05k | - | - | - |
| cache references | - | - | - | 0.906 | 0.527 | - | - | - |
| cache misses | - | - | - | 0.297 | 0.104 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0133 | 0.000704 | - | - | - |
| data TLB read misses | - | - | - | 0.0809 | 0.00507 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.0684 | 0.000684 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 175.0 | 0.000575 | 0.00000575 |
| collectors' pauses, ns | - | - | - | - | - | 10.55M | 34.63 | 0.346 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 22.34M | - | - | - | - | 59.20M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 22.34M | - | - | - | - | 57.29M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 14.90M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304599 messages, 30460000 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-578270-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-578270-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 108872 | 3046 | 245926 | 0 | 108872000 | 58721000 | 240862000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 899.4 | 2.9 | 204.4 | 27.24 / 28.97 |
| best | nats | 22 × busy | 108872 | 3046 | 245926 | 0 | 108872000 | 58721000 | 240862000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 899.4 | 2.9 | 204.4 | 27.24 / 28.97 |

Run 1's profile:

| profile: the window, 30.0 s, 74032 messages, 76024169 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 625.1G | 8.444M | 8223 | 634.1 | 8222 | 144.9G | 1.958M | 1876 |
| waiting for a CPU, ns | - | - | - | 700.7 | 10.20 | - | - | - |
| allocations | 77.24M | 1043 | 1.016 | 2.016 | 1.014 | 66.33M | 895.9 | 0.858 |
| reallocations | 4.000 | 0.0000540 | 0.000000053 | 0 | 0.000000053 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 3.000 | 0.0000405 | 0.000000039 | 0 | 0.000000039 | - | - | - |
| frees | 77.24M | 1043 | 1.016 | 2.016 | 1.014 | 66.36M | 896.4 | 0.859 |
| bytes allocated | 19.22G | 259.6k | 252.8 | 363.7 | 252.4 | 3.288G | 44.41k | 42.55 |
| minor page faults | 1932 | 0.0261 | 0.0000254 | 0 | 0.0000254 | 19.02k | 0.257 | 0.000246 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 15.00 | 0.000203 | 0.000000197 | 0.000189 | 0 | - | - | - |
| involuntary context switches | 9733 | 0.131 | 0.000128 | 0.000216 | 0.000128 | - | - | - |
| cycles | - | - | - | 2083 | 21.75k | - | - | - |
| instructions | - | - | - | 3537 | 33.84k | - | - | - |
| cache references | - | - | - | 6.297 | 12.49 | - | - | - |
| cache misses | - | - | - | 3.252 | 8.953 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.159 | 0.153 | - | - | - |
| data TLB read misses | - | - | - | 0.0263 | 0.208 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.277 | 0.000269 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 174.0 | 0.00235 | 0.00000225 |
| collectors' pauses, ns | - | - | - | - | - | 12.27M | 165.8 | 0.159 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 127.2M | - | - | - | - | 134.5M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 127.2M | - | - | - | - | 124.7M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 36.24M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 74032 messages, 77264792 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-584616-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-584616-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × busy | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 20.0 | 30.0 | 52.7 | 190.5 | 1220.8 | 0 | 0 | 1/1 | 60.9 | 27.8 | 24.2 | 15.07 / 11.01 |
| best | nats | 1 × busy | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 20.0 | 30.0 | 52.7 | 190.5 | 1220.8 | 0 | 0 | 1/1 | 60.9 | 27.8 | 24.2 | 15.07 / 11.01 |

Run 1's profile:

| profile: the window, 30.0 s, 3000397 messages, 3000396 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 51.30G | 17.10k | 17.10k | 7828 | 9269 | 21.06G | 6905 | 6905 |
| waiting for a CPU, ns | - | - | - | 1.370 | 1.126 | - | - | - |
| allocations | 9.021M | 3.007 | 3.007 | 2.000 | 1.007 | 8.718M | 2.858 | 2.858 |
| reallocations | 1.000 | 0.000000333 | 0.000000333 | 0.000000333 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 9.021M | 3.007 | 3.007 | 2.000 | 1.007 | 8.743M | 2.866 | 2.866 |
| bytes allocated | 1.230G | 409.8 | 409.8 | 280.0 | 129.8 | 1.090G | 357.3 | 357.3 |
| minor page faults | 6.000 | 0.00000200 | 0.00000200 | 0.000001000 | 0 | 1119 | 0.000367 | 0.000367 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 678.0 | 0.000226 | 0.000226 | 0.000150 | 0.0000757 | - | - | - |
| cycles | - | - | - | 35.90k | 52.33k | - | - | - |
| instructions | - | - | - | 62.36k | 87.13k | - | - | - |
| cache references | - | - | - | 6.210 | 2.305 | - | - | - |
| cache misses | - | - | - | 1.864 | 0.824 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0293 | 0.0265 | - | - | - |
| data TLB read misses | - | - | - | 0.143 | 0.0140 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.00683 | 0.00683 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 135.0 | 0.0000443 | 0.0000443 |
| collectors' pauses, ns | - | - | - | - | - | 5.589M | 1.832 | 1.832 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.963M | - | - | - | - | 51.61M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.963M | - | - | - | - | 49.19M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 12.77M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3050588 messages, 3050590 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-601260-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-601260-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 928898 | 25980 | 2621140 | 0 | 92889800 | 42906900 | 257136800 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 738.3 | 0.9 | 167.9 | 19.77 / 24.59 |
| best | nats | 22 × busy | 928898 | 25980 | 2621140 | 0 | 92889800 | 42906900 | 257136800 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 738.3 | 0.9 | 167.9 | 19.77 / 24.59 |

Run 1's profile:

| profile: the window, 30.0 s, 767792 messages, 76892668 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 612.0G | 797.0k | 7959 | 605.7 | 7952 | 142.0G | 182.2k | 1817 |
| waiting for a CPU, ns | - | - | - | 91.05 | 6.860 | - | - | - |
| allocations | 79.48M | 103.5 | 1.034 | 2.017 | 1.013 | 72.03M | 92.42 | 0.921 |
| reallocations | 7.000 | 0.00000912 | 0.000000091 | 0 | 0.000000091 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 7.000 | 0.00000912 | 0.000000091 | 0 | 0.000000091 | - | - | - |
| frees | 79.48M | 103.5 | 1.034 | 2.017 | 1.013 | 72.04M | 92.43 | 0.922 |
| bytes allocated | 19.06G | 24.82k | 247.8 | 366.4 | 244.2 | 3.369G | 4323 | 43.09 |
| minor page faults | 629.0 | 0.000819 | 0.00000818 | 0 | 0.00000813 | 23.67k | 0.0304 | 0.000303 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 156.0 | 0.000203 | 0.00000203 | 0.000202 | 0 | - | - | - |
| involuntary context switches | 8810 | 0.0115 | 0.000115 | 0.0000755 | 0.000114 | - | - | - |
| cycles | - | - | - | 2059 | 19.46k | - | - | - |
| instructions | - | - | - | 3539 | 30.30k | - | - | - |
| cache references | - | - | - | 6.288 | 0.465 | - | - | - |
| cache misses | - | - | - | 2.626 | 0.0603 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.181 | 0.000554 | - | - | - |
| data TLB read misses | - | - | - | 0.0346 | 0.00604 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 53.25k | 0.0694 | 0.000692 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 338.0 | 0.000434 | 0.00000432 |
| collectors' pauses, ns | - | - | - | - | - | refused: 338 collections in the span, past the 256 pauses Go keeps | refused | refused |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 26.38M | - | - | - | - | 65.26M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 26.38M | - | - | - | - | 61.92M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 17.30M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 779360 messages, 78173663 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-607694-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-607694-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 104108 | 2779 | 3445930 | 0 | 104108000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 884.1 | 0.4 | 192.2 | 27.08 / 29.60 |
| best | nats | 22 × busy | 104108 | 2779 | 3445930 | 0 | 104108000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 884.1 | 0.4 | 192.2 | 27.08 / 29.60 |

Run 1's profile:

| profile: the window, 30.0 s, 70432 messages, 73689869 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 625.0G | 8.873M | 8481 | 630.1 | 8480 | 137.9G | 1.830M | 1841 |
| waiting for a CPU, ns | - | - | - | 348.8 | 14.58 | - | - | - |
| allocations | 74.88M | 1063 | 1.016 | 2.017 | 1.014 | 60.95M | 808.6 | 0.814 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 74.88M | 1063 | 1.016 | 2.017 | 1.014 | 60.74M | 805.8 | 0.811 |
| bytes allocated | 18.90G | 268.4k | 256.5 | 365.7 | 256.2 | 3.202G | 42.48k | 42.75 |
| minor page faults | 1910 | 0.0271 | 0.0000259 | 0 | 0.0000259 | 9236 | 0.123 | 0.000123 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 14.00 | 0.000199 | 0.000000190 | 0.000199 | 0 | - | - | - |
| involuntary context switches | 11.97k | 0.170 | 0.000162 | 0.0000994 | 0.000162 | - | - | - |
| cycles | - | - | - | 2038 | 22.11k | - | - | - |
| instructions | - | - | - | 3535 | 34.61k | - | - | - |
| cache references | - | - | - | 6.607 | 11.88 | - | - | - |
| cache misses | - | - | - | 3.351 | 8.379 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.164 | 0.131 | - | - | - |
| data TLB read misses | - | - | - | 0.0242 | 0.201 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.291 | 0.000278 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 154.0 | 0.00204 | 0.00000206 |
| collectors' pauses, ns | - | - | - | - | - | 21.47M | 284.9 | 0.287 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 128.2M | - | - | - | - | 138.5M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 128.2M | - | - | - | - | 131.9M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 44.41M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 75376 messages, 74905608 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-616111-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-616111-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 27.6 | 63.0 | 123.9 | 188.4 | 1550.1 | 0 | 0 | 1/1 | 37.5 | 34.5 | 5.4 | 19.41 / 12.62 |
| best | nats | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 27.6 | 63.0 | 123.9 | 188.4 | 1550.1 | 0 | 0 | 1/1 | 37.5 | 34.5 | 5.4 | 19.41 / 12.62 |

Run 1's profile:

| profile: the window, 30.0 s, 300766 messages, 300766 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 31.71G | 105.4k | 105.4k | 96.95k | 8492 | 4.694G | 15.35k | 15.35k |
| waiting for a CPU, ns | - | - | - | 4.163 | 13.67 | - | - | - |
| allocations | 902.9k | 3.002 | 3.002 | 2.000 | 1.002 | 968.7k | 3.167 | 3.167 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 902.9k | 3.002 | 3.002 | 2.000 | 1.002 | 952.9k | 3.115 | 3.115 |
| bytes allocated | 100.2M | 333.2 | 333.2 | 280.0 | 53.22 | 73.70M | 240.9 | 240.9 |
| minor page faults | 5.000 | 0.0000166 | 0.0000166 | 0.00000332 | 0 | 725.0 | 0.00237 | 0.00237 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 255.3k | 0.849 | 0.849 | 0 | 0.849 | - | - | - |
| involuntary context switches | 226.0 | 0.000751 | 0.000751 | 0.000542 | 0.000209 | - | - | - |
| cycles | - | - | - | 349.6k | 15.89k | - | - | - |
| instructions | - | - | - | 678.0k | 17.69k | - | - | - |
| cache references | - | - | - | 4.273 | 45.22 | - | - | - |
| cache misses | - | - | - | 2.482 | 7.635 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.282 | 0.221 | - | - | - |
| data TLB read misses | - | - | - | 0.0805 | 1.229 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.0681 | 0.0681 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 8.000 | 0.0000262 | 0.0000262 |
| collectors' pauses, ns | - | - | - | - | - | 201.1k | 0.657 | 0.657 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.865M | - | - | - | - | 54.80M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.865M | - | - | - | - | 55.21M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 16.25M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305873 messages, 305874 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-626147-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-626147-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 151.6 | 260.1 | 622.6 | 1179.6 | 2620.6 | 0 | 0 | 100/100 | 200.5 | 34.4 | 121.0 | 3.36 / 8.03 |
| best | nats | 22 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 151.6 | 260.1 | 622.6 | 1179.6 | 2620.6 | 0 | 0 | 100/100 | 200.5 | 34.4 | 121.0 | 3.36 / 8.03 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 30076741 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 169.9G | 564.8k | 5648 | 96.70k | 4681 | 104.2G | 340.7k | 3407 |
| waiting for a CPU, ns | - | - | - | 4.136 | 29.45 | - | - | - |
| allocations | 30.90M | 102.7 | 1.027 | 2.000 | 1.007 | 45.73M | 149.5 | 1.495 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 30.90M | 102.7 | 1.027 | 2.000 | 1.007 | 46.00M | 150.4 | 1.504 |
| bytes allocated | 4.397G | 14.62k | 146.2 | 280.0 | 143.4 | 1.488G | 4866 | 48.66 |
| minor page faults | 60.00 | 0.000199 | 0.00000199 | 0 | 0.00000186 | 1907 | 0.00623 | 0.0000623 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 12.38M | 41.15 | 0.411 | 0 | 0.411 | - | - | - |
| involuntary context switches | 22.83k | 0.0759 | 0.000759 | 0.000612 | 0.000753 | - | - | - |
| cycles | - | - | - | 485.2k | 11.42k | - | - | - |
| instructions | - | - | - | 915.8k | 13.26k | - | - | - |
| cache references | - | - | - | 13.51 | 29.83 | - | - | - |
| cache misses | - | - | - | 2.719 | 1.201 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0309 | 0.0230 | - | - | - |
| data TLB read misses | - | - | - | 0.244 | 1.448 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.0681 | 0.000681 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 147.0 | 0.000481 | 0.00000481 |
| collectors' pauses, ns | - | - | - | - | - | 6.917M | 22.62 | 0.226 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 22.41M | - | - | - | - | 64.76M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 22.41M | - | - | - | - | 61.66M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 16.54M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305869 messages, 30586662 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-632610-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-632610-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 115775 | 3233 | 240326 | 0 | 115775000 | 65371000 | 235398000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 237.4 | 2.9 | 201.4 | 9.38 / 13.40 |
| best | nats | 22 × wait | 115775 | 3233 | 240326 | 0 | 115775000 | 65371000 | 235398000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 237.4 | 2.9 | 201.4 | 9.38 / 13.40 |

Run 1's profile:

| profile: the window, 30.0 s, 81680 messages, 82842900 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 169.0G | 2.069M | 2040 | 608.7 | 2039 | 147.4G | 1.805M | 1750 |
| waiting for a CPU, ns | - | - | - | 3.174 | 68.36 | - | - | - |
| allocations | 84.21M | 1031 | 1.016 | 2.016 | 1.014 | 69.27M | 848.0 | 0.822 |
| reallocations | 76.00 | 0.000930 | 0.000000917 | 0 | 0.000000917 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 76.00 | 0.000930 | 0.000000917 | 0 | 0.000000917 | - | - | - |
| frees | 84.20M | 1031 | 1.016 | 2.016 | 1.014 | 69.31M | 848.5 | 0.823 |
| bytes allocated | 21.44G | 262.5k | 258.8 | 362.5 | 258.4 | 3.520G | 43.10k | 41.79 |
| minor page faults | 45.49k | 0.557 | 0.000549 | 0 | 0.000549 | 7288 | 0.0892 | 0.0000865 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 9.846M | 120.5 | 0.119 | 0.000184 | 0.119 | - | - | - |
| involuntary context switches | 25.00k | 0.306 | 0.000302 | 0.0000367 | 0.000302 | - | - | - |
| cycles | - | - | - | 3927 | 6702 | - | - | - |
| instructions | - | - | - | 7095 | 8514 | - | - | - |
| cache references | - | - | - | 13.85 | 52.77 | - | - | - |
| cache misses | - | - | - | 6.853 | 28.20 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.217 | 1.187 | - | - | - |
| data TLB read misses | - | - | - | 0.0515 | 3.245 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.251 | 0.000247 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 175.0 | 0.00214 | 0.00000208 |
| collectors' pauses, ns | - | - | - | - | - | 11.06M | 135.4 | 0.131 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 300.0M | - | - | - | - | 156.6M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 300.0M | - | - | - | - | 129.3M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 41.39M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 81680 messages, 84248459 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-639472-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-639472-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × wait | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 19.5 | 25.1 | 47.6 | 122.9 | 1533.3 | 0 | 0 | 1/1 | 38.1 | 28.3 | 21.9 | 7.55 / 5.67 |
| best | nats | 1 × wait | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 19.5 | 25.1 | 47.6 | 122.9 | 1533.3 | 0 | 0 | 1/1 | 38.1 | 28.3 | 21.9 | 7.55 / 5.67 |

Run 1's profile:

| profile: the window, 30.0 s, 3002622 messages, 3002626 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 32.15G | 10.71k | 10.71k | 7960 | 2748 | 18.75G | 6143 | 6143 |
| waiting for a CPU, ns | - | - | - | 0.799 | 1.594 | - | - | - |
| allocations | 9.024M | 3.005 | 3.005 | 2.000 | 1.005 | 8.938M | 2.928 | 2.928 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 9.024M | 3.005 | 3.005 | 2.000 | 1.005 | 8.989M | 2.945 | 2.945 |
| bytes allocated | 1.169G | 389.4 | 389.4 | 280.0 | 109.4 | 1.024G | 335.4 | 335.4 |
| minor page faults | 12.00 | 0.00000400 | 0.00000400 | 0 | 0.00000266 | 828.0 | 0.000271 | 0.000271 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.767M | 0.588 | 0.588 | 0 | 0.588 | - | - | - |
| involuntary context switches | 515.0 | 0.000172 | 0.000172 | 0.000154 | 0.0000177 | - | - | - |
| cycles | - | - | - | 27.06k | 7227 | - | - | - |
| instructions | - | - | - | 52.42k | 10.06k | - | - | - |
| cache references | - | - | - | 0.549 | 2.136 | - | - | - |
| cache misses | - | - | - | 0.0445 | 0.170 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00249 | 0.00273 | - | - | - |
| data TLB read misses | - | - | - | 0.0980 | 0.0530 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.00136 | 0.00136 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 106.0 | 0.0000347 | 0.0000347 |
| collectors' pauses, ns | - | - | - | - | - | 3.978M | 1.303 | 1.303 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.722M | - | - | - | - | 55.89M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.722M | - | - | - | - | 53.77M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 15.50M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3052605 messages, 3052603 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-655797-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-655797-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 1003064 | 28128 | 2550954 | 0 | 100306400 | 50176800 | 250086300 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 214.2 | 0.8 | 163.2 | 7.43 / 11.03 |
| best | nats | 22 × wait | 1003064 | 28128 | 2550954 | 0 | 100306400 | 50176800 | 250086300 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 214.2 | 0.8 | 163.2 | 7.43 / 11.03 |

Run 1's profile:

| profile: the window, 30.0 s, 833280 messages, 83456793 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 178.2G | 213.9k | 2136 | 570.0 | 2130 | 138.5G | 162.9k | 1633 |
| waiting for a CPU, ns | - | - | - | 0.319 | 4.072 | - | - | - |
| allocations | 86.29M | 103.5 | 1.034 | 2.016 | 1.014 | 75.41M | 88.66 | 0.889 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 86.28M | 103.5 | 1.034 | 2.016 | 1.014 | 75.41M | 88.65 | 0.889 |
| bytes allocated | 21.06G | 25.27k | 252.3 | 360.7 | 248.7 | 3.649G | 4290 | 43.00 |
| minor page faults | 131.0 | 0.000157 | 0.00000157 | 0 | 0.00000151 | 16.19k | 0.0190 | 0.000191 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 14.13M | 16.96 | 0.169 | 0.000176 | 0.169 | - | - | - |
| involuntary context switches | 9932 | 0.0119 | 0.000119 | 0.0000276 | 0.000119 | - | - | - |
| cycles | - | - | - | 3745 | 6119 | - | - | - |
| instructions | - | - | - | 7726 | 8960 | - | - | - |
| cache references | - | - | - | 12.81 | 7.100 | - | - | - |
| cache misses | - | - | - | 5.698 | 0.439 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.203 | 0.0107 | - | - | - |
| data TLB read misses | - | - | - | 0.0553 | 0.173 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.00492 | 0.0000491 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 313.0 | 0.000368 | 0.00000369 |
| collectors' pauses, ns | - | - | - | - | - | refused: 313 collections in the span, past the 256 pauses Go keeps | refused | refused |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 24.15M | - | - | - | - | 71.96M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 24.15M | - | - | - | - | 65.98M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 20.85M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 850592 messages, 84859804 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-668589-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-668589-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 115016 | 3087 | 3439002 | 0 | 115016000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 233.5 | 0.4 | 202.5 | 13.05 / 14.36 |
| best | nats | 22 × wait | 115016 | 3087 | 3439002 | 0 | 115016000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 233.5 | 0.4 | 202.5 | 13.05 / 14.36 |

Run 1's profile:

| profile: the window, 30.0 s, 81920 messages, 80913740 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 163.0G | 1.990M | 2014 | 641.4 | 2014 | 144.5G | 1.764M | 1756 |
| waiting for a CPU, ns | - | - | - | 0.0944 | 6.275 | - | - | - |
| allocations | 82.23M | 1004 | 1.016 | 2.016 | 1.014 | 67.77M | 827.3 | 0.824 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 82.23M | 1004 | 1.016 | 2.016 | 1.014 | 67.76M | 827.1 | 0.823 |
| bytes allocated | 20.77G | 253.5k | 256.7 | 362.4 | 256.3 | 3.411G | 41.64k | 41.46 |
| minor page faults | 775.0 | 0.00946 | 0.00000958 | 0 | 0.00000954 | 4423 | 0.0540 | 0.0000538 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 11.62M | 141.9 | 0.144 | 0.000183 | 0.144 | - | - | - |
| involuntary context switches | 12.62k | 0.154 | 0.000156 | 0.0000122 | 0.000156 | - | - | - |
| cycles | - | - | - | 3758 | 6015 | - | - | - |
| instructions | - | - | - | 7985 | 8381 | - | - | - |
| cache references | - | - | - | 28.64 | 36.63 | - | - | - |
| cache misses | - | - | - | 15.29 | 20.93 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 1.023 | 0.764 | - | - | - |
| data TLB read misses | - | - | - | 0.138 | 1.556 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.0500 | 0.0000506 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 163.0 | 0.00199 | 0.00000198 |
| collectors' pauses, ns | - | - | - | - | - | 10.64M | 129.8 | 0.129 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 122.2M | - | - | - | - | 135.5M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 122.2M | - | - | - | - | 131.1M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 41.39M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 81920 messages, 82284953 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-679055-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-679055-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 16.9 | 44.0 | 69.6 | 95.2 | 729.1 | 0 | 0 | 1/1 | 69.8 | 34.6 | 5.1 | 11.37 / 7.22 |
| best | nats | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 16.9 | 44.0 | 69.6 | 95.2 | 729.1 | 0 | 0 | 1/1 | 69.8 | 34.6 | 5.1 | 11.37 / 7.22 |

Run 1's profile:

| profile: the window, 30.0 s, 300766 messages, 300766 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 58.90G | 195.8k | 195.8k | 97.22k | 98.62k | 4.347G | 14.21k | 14.21k |
| waiting for a CPU, ns | - | - | - | 7.267 | 5.156 | - | - | - |
| allocations | 902.6k | 3.001 | 3.001 | 2.000 | 1.001 | 975.8k | 3.190 | 3.190 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 902.6k | 3.001 | 3.001 | 2.000 | 1.001 | 1.067M | 3.489 | 3.489 |
| bytes allocated | 96.58M | 321.1 | 321.1 | 280.0 | 41.11 | 72.67M | 237.6 | 237.6 |
| minor page faults | 3.000 | 0.00000997 | 0.00000997 | 0 | 0 | 743.0 | 0.00243 | 0.00243 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 433.0 | 0.00144 | 0.00144 | 0.000815 | 0.000625 | - | - | - |
| cycles | - | - | - | 365.1k | 663.8k | - | - | - |
| instructions | - | - | - | 708.9k | 1.147M | - | - | - |
| cache references | - | - | - | 1.294 | 18.97 | - | - | - |
| cache misses | - | - | - | 0.234 | 5.888 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0194 | 0.0149 | - | - | - |
| data TLB read misses | - | - | - | 0.154 | 0.233 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 53.25k | 0.177 | 0.177 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 8.000 | 0.0000262 | 0.0000262 |
| collectors' pauses, ns | - | - | - | - | - | 190.5k | 0.623 | 0.623 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.889M | - | - | - | - | 57.56M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.889M | - | - | - | - | 55.54M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 21.50M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305856 messages, 305855 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-691544-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-691544-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 200.7 | 434.2 | 1048.6 | 2031.6 | 12670.1 | 0 | 0 | 100/100 | 774.5 | 33.4 | 93.3 | 12.16 / 19.41 |
| best | nats | 22 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 200.7 | 434.2 | 1048.6 | 2031.6 | 12670.1 | 0 | 0 | 100/100 | 774.5 | 33.4 | 93.3 | 12.16 / 19.41 |

Run 1's profile:

| profile: the window, 30.0 s, 300766 messages, 30076563 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 654.1G | 2.175M | 21.75k | 93.84k | 20.81k | 79.54G | 260.0k | 2600 |
| waiting for a CPU, ns | - | - | - | 32.01 | 4.231 | - | - | - |
| allocations | 30.99M | 103.0 | 1.030 | 2.000 | 1.010 | 37.71M | 123.3 | 1.233 |
| reallocations | 1.000 | 0.00000332 | 0.000000033 | 0.00000332 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 1.000 | 0.00000332 | 0.000000033 | 0.00000332 | 0 | - | - | - |
| frees | 30.99M | 103.0 | 1.030 | 2.000 | 1.010 | 37.73M | 123.4 | 1.234 |
| bytes allocated | 5.830G | 19.38k | 193.8 | 280.0 | 191.0 | 1.457G | 4764 | 47.64 |
| minor page faults | 125.0 | 0.000416 | 0.00000416 | 0.00000332 | 0.00000399 | 3161 | 0.0103 | 0.000103 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.000 | 0.00000332 | 0.000000033 | 0 | 0 | - | - | - |
| involuntary context switches | 4276 | 0.0142 | 0.000142 | 0.00132 | 0.000129 | - | - | - |
| cycles | - | - | - | 180.6k | 50.57k | - | - | - |
| instructions | - | - | - | 330.3k | 77.30k | - | - | - |
| cache references | - | - | - | 1.780 | 1.077 | - | - | - |
| cache misses | - | - | - | 1.180 | 0.423 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.00299 | - | - | - |
| data TLB read misses | - | - | - | 0.00982 | 0.0142 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.0136 | 0.000136 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 128.0 | 0.000418 | 0.00000418 |
| collectors' pauses, ns | - | - | - | - | - | 17.37M | 56.80 | 0.568 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 22.42M | - | - | - | - | 67.64M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 22.42M | - | - | - | - | 65.08M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 20.55M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305855 messages, 30585454 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-697021-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-697021-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 104048 | 2833 | 252053 | 0 | 104048000 | 53644000 | 247125000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 885.4 | 2.4 | 187.3 | 27.49 / 28.89 |
| best | nats | 22 × busy | 104048 | 2833 | 252053 | 0 | 104048000 | 53644000 | 247125000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 885.4 | 2.4 | 187.3 | 27.49 / 28.89 |

Run 1's profile:

| profile: the window, 30.0 s, 75152 messages, 73497886 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 624.3G | 8.308M | 8495 | 638.4 | 8494 | 135.0G | 1.796M | 1805 |
| waiting for a CPU, ns | - | - | - | 917.8 | 22.97 | - | - | - |
| allocations | 74.70M | 994.0 | 1.016 | 2.017 | 1.014 | 59.55M | 792.4 | 0.796 |
| reallocations | 4.000 | 0.0000532 | 0.000000054 | 0 | 0.000000054 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 4.000 | 0.0000532 | 0.000000054 | 0 | 0.000000054 | - | - | - |
| frees | 74.70M | 994.0 | 1.016 | 2.017 | 1.014 | 59.46M | 791.2 | 0.795 |
| bytes allocated | 18.94G | 252.0k | 257.6 | 366.4 | 257.3 | 3.231G | 42.99k | 43.21 |
| minor page faults | 4380 | 0.0583 | 0.0000596 | 0 | 0.0000595 | 10.64k | 0.142 | 0.000142 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 16.00 | 0.000213 | 0.000000218 | 0.000200 | 0 | - | - | - |
| involuntary context switches | 10.95k | 0.146 | 0.000149 | 0.000186 | 0.000149 | - | - | - |
| cycles | - | - | - | 2087 | 21.36k | - | - | - |
| instructions | - | - | - | 3535 | 33.34k | - | - | - |
| cache references | - | - | - | 6.620 | 11.43 | - | - | - |
| cache misses | - | - | - | 3.403 | 8.136 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.205 | 0.115 | - | - | - |
| data TLB read misses | - | - | - | 0.0345 | 0.194 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.0545 | 0.0000557 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 140.0 | 0.00186 | 0.00000187 |
| collectors' pauses, ns | - | - | - | - | - | 29.84M | 397.1 | 0.399 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 130.3M | - | - | - | - | 142.1M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 127.7M | - | - | - | - | 132.7M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 38.81M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 75152 messages, 74772948 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-702702-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-702702-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × busy | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 18.7 | 25.9 | 48.1 | 153.6 | 1819.3 | 0 | 0 | 1/1 | 60.9 | 27.9 | 23.3 | 28.93 / 18.25 |
| best | nats | 1 × busy | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 18.7 | 25.9 | 48.1 | 153.6 | 1819.3 | 0 | 0 | 1/1 | 60.9 | 27.9 | 23.3 | 28.93 / 18.25 |

Run 1's profile:

| profile: the window, 30.0 s, 3002640 messages, 3002638 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 51.36G | 17.10k | 17.10k | 7849 | 9254 | 20.09G | 6580 | 6580 |
| waiting for a CPU, ns | - | - | - | 0.912 | 0.548 | - | - | - |
| allocations | 9.026M | 3.006 | 3.006 | 2.000 | 1.006 | 8.829M | 2.892 | 2.892 |
| reallocations | 1.000 | 0.000000333 | 0.000000333 | 0.000000333 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 1.000 | 0.000000333 | 0.000000333 | 0.000000333 | 0 | - | - | - |
| frees | 9.026M | 3.006 | 3.006 | 2.000 | 1.006 | 8.780M | 2.876 | 2.876 |
| bytes allocated | 1.204G | 401.0 | 401.0 | 280.0 | 121.0 | 1.063G | 348.1 | 348.1 |
| minor page faults | 9.000 | 0.00000300 | 0.00000300 | 0.00000133 | 0.000000333 | 813.0 | 0.000266 | 0.000266 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 614.0 | 0.000204 | 0.000204 | 0.000148 | 0.0000563 | - | - | - |
| cycles | - | - | - | 38.21k | 33.92k | - | - | - |
| instructions | - | - | - | 66.39k | 59.07k | - | - | - |
| cache references | - | - | - | 2.351 | 0.630 | - | - | - |
| cache misses | - | - | - | 0.815 | 0.236 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0260 | 0.0214 | - | - | - |
| data TLB read misses | - | - | - | 0.150 | 0.00241 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.00682 | 0.00682 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 106.0 | 0.0000347 | 0.0000347 |
| collectors' pauses, ns | - | - | - | - | - | 4.598M | 1.506 | 1.506 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.746M | - | - | - | - | 47.20M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.746M | - | - | - | - | 45.02M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 20.62M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3052608 messages, 3052609 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-714587-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-714587-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 953055 | 26587 | 2600963 | 0 | 95305500 | 45175900 | 255087200 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 732.7 | 1.0 | 170.3 | 12.83 / 20.43 |
| best | nats | 22 × busy | 953055 | 26587 | 2600963 | 0 | 95305500 | 45175900 | 255087200 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 732.7 | 1.0 | 170.3 | 12.83 / 20.43 |

Run 1's profile:

| profile: the window, 30.0 s, 786480 messages, 78694756 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 607.2G | 772.0k | 7716 | 604.8 | 7710 | 143.7G | 179.2k | 1796 |
| waiting for a CPU, ns | - | - | - | 439.7 | 11.42 | - | - | - |
| allocations | 81.34M | 103.4 | 1.034 | 2.016 | 1.013 | 74.62M | 93.04 | 0.932 |
| reallocations | 1.000 | 0.00000127 | 0.000000013 | 0 | 0.000000013 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 1.000 | 0.00000127 | 0.000000013 | 0 | 0.000000013 | - | - | - |
| frees | 81.34M | 103.4 | 1.034 | 2.016 | 1.013 | 74.74M | 93.19 | 0.934 |
| bytes allocated | 19.40G | 24.67k | 246.6 | 364.4 | 242.9 | 3.444G | 4294 | 43.03 |
| minor page faults | 156.0 | 0.000198 | 0.00000198 | 0 | 0.00000194 | 18.71k | 0.0233 | 0.000234 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 151.0 | 0.000192 | 0.00000192 | 0.000192 | 0 | - | - | - |
| involuntary context switches | 9889 | 0.0126 | 0.000126 | 0.000116 | 0.000124 | - | - | - |
| cycles | - | - | - | 1731 | 19.34k | - | - | - |
| instructions | - | - | - | 3536 | 29.98k | - | - | - |
| cache references | - | - | - | 6.134 | 0.460 | - | - | - |
| cache misses | - | - | - | 2.941 | 0.0713 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00258 | 0.000674 | - | - | - |
| data TLB read misses | - | - | - | 0.00805 | 0.00861 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.0260 | 0.000260 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 288.0 | 0.000359 | 0.00000360 |
| collectors' pauses, ns | - | - | - | - | - | refused: 288 collections in the span, past the 256 pauses Go keeps | refused | refused |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.65M | - | - | - | - | 59.87M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.65M | - | - | - | - | 55.73M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 20.23M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 802000 messages, 80028515 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-719761-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-719761-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 105081 | 2857 | 3448937 | 0 | 105081000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 865.1 | 0.4 | 197.8 | 26.03 / 28.17 |
| best | nats | 22 × busy | 105081 | 2857 | 3448937 | 0 | 105081000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 865.1 | 0.4 | 197.8 | 26.03 / 28.17 |

Run 1's profile:

| profile: the window, 30.0 s, 77904 messages, 76039504 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 624.8G | 8.020M | 8216 | 614.2 | 8216 | 145.2G | 1.864M | 1878 |
| waiting for a CPU, ns | - | - | - | 135.9 | 22.92 | - | - | - |
| allocations | 77.26M | 991.7 | 1.016 | 2.017 | 1.014 | 66.14M | 849.0 | 0.855 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 77.26M | 991.7 | 1.016 | 2.017 | 1.014 | 65.97M | 846.8 | 0.853 |
| bytes allocated | 19.14G | 245.7k | 251.7 | 367.6 | 251.4 | 3.270G | 41.98k | 42.29 |
| minor page faults | 3204 | 0.0411 | 0.0000421 | 0 | 0.0000421 | 4911 | 0.0630 | 0.0000635 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 17.00 | 0.000218 | 0.000000224 | 0.000205 | 0 | - | - | - |
| involuntary context switches | 8371 | 0.107 | 0.000110 | 0.0000257 | 0.000110 | - | - | - |
| cycles | - | - | - | 2001 | 19.92k | - | - | - |
| instructions | - | - | - | 3536 | 31.08k | - | - | - |
| cache references | - | - | - | 6.544 | 10.89 | - | - | - |
| cache misses | - | - | - | 3.292 | 7.989 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.167 | 0.112 | - | - | - |
| data TLB read misses | - | - | - | 0.0233 | 0.117 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.263 | 0.000269 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 151.0 | 0.00194 | 0.00000195 |
| collectors' pauses, ns | - | - | - | - | - | 11.15M | 143.2 | 0.144 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 133.9M | - | - | - | - | 127.8M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 133.9M | - | - | - | - | 124.1M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 46.18M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 77904 messages, 77327893 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-725956-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/nats-725956-0-nats-allocs.pb.gz
