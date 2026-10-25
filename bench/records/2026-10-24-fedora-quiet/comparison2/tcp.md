- relay: pid 277994, CPUs 8-15
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 20.7 | 25.6 | 67.6 | 147.5 | 1822.8 | 0 | 0 | 1/1 | 37.0 | 34.7 | 34.0 | 2.23 / 3.06 |
| best | tcp | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 20.7 | 25.6 | 67.6 | 147.5 | 1822.8 | 0 | 0 | 1/1 | 37.0 | 34.7 | 34.0 | 2.23 / 3.06 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 299499 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 31.32G | 104.6k | 104.6k | 97.88k | 6700 | - | - | - |
| waiting for a CPU, ns | - | - | - | 5.038 | 34.57 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 268.5k | 0.896 | 0.896 | 0 | 0.896 | - | - | - |
| involuntary context switches | 309.0 | 0.00103 | 0.00103 | 0.000524 | 0.000508 | - | - | - |
| cycles | - | - | - | 532.7k | 7593 | - | - | - |
| instructions | - | - | - | 987.1k | 7007 | - | - | - |
| cache references | - | - | - | 15.51 | 24.70 | - | - | - |
| cache misses | - | - | - | 5.801 | 3.792 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0435 | 0.0861 | - | - | - |
| data TLB read misses | - | - | - | 0.0293 | 0.538 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.173M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.173M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 86.0 | 137.2 | 170.0 | 409.6 | 11265.6 | 0 | 0 | 100/100 | 194.8 | 34.3 | 172.0 | 2.71 / 10.41 |
| best | tcp | 22 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 86.0 | 137.2 | 170.0 | 409.6 | 11265.6 | 0 | 0 | 100/100 | 194.8 | 34.3 | 172.0 | 2.71 / 10.41 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 29949498 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 164.6G | 549.5k | 5495 | 96.80k | 4527 | - | - | - |
| waiting for a CPU, ns | - | - | - | 38.40 | 52.73 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 9.000 | 0.0000301 | 0.000000301 | 0 | 0.000000301 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 9.772M | 32.63 | 0.326 | 0 | 0.326 | - | - | - |
| involuntary context switches | 24.31k | 0.0812 | 0.000812 | 0.00361 | 0.000776 | - | - | - |
| cycles | - | - | - | 462.0k | 6242 | - | - | - |
| instructions | - | - | - | 812.1k | 5413 | - | - | - |
| cache references | - | - | - | 6.678 | 4.176 | - | - | - |
| cache misses | - | - | - | 0.700 | 0.292 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0164 | 0.00432 | - | - | - |
| data TLB read misses | - | - | - | 0.101 | 0.134 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 11.12M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 11.12M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × wait | 354590 | 9988 | 0 | 0 | 354590000 | 299497000 | 0 | 942.1 | 1540.1 | 2293.8 | 15990.8 | 26401.7 | 0 | 0 | 1000/1000 | 208.4 | 34.4 | 97.4 | 4.65 / 11.05 |
| best | tcp | 22 × wait | 354590 | 9988 | 0 | 0 | 354590000 | 299497000 | 0 | 942.1 | 1540.1 | 2293.8 | 15990.8 | 26401.7 | 0 | 0 | 1000/1000 | 208.4 | 34.4 | 97.4 | 4.65 / 11.05 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 299494790 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 175.8G | 587.0k | 587.0 | 97.26k | 489.7 | - | - | - |
| waiting for a CPU, ns | - | - | - | 7.272 | 6.068 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 414.0 | 0.00138 | 0.00000138 | 0 | 0.00000138 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 6.530M | 21.80 | 0.0218 | 0 | 0.0218 | - | - | - |
| involuntary context switches | 8821 | 0.0295 | 0.0000295 | 0.000387 | 0.0000291 | - | - | - |
| cycles | - | - | - | 435.6k | 986.5 | - | - | - |
| instructions | - | - | - | 786.9k | 1624 | - | - | - |
| cache references | - | - | - | 8.459 | 3.333 | - | - | - |
| cache misses | - | - | - | 2.223 | 0.321 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0294 | 0.00449 | - | - | - |
| data TLB read misses | - | - | - | 0.0920 | 0.0487 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 31.86M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 31.86M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 17.4 | 23.6 | 35.3 | 127.0 | 1742.4 | 0 | 0 | 1/1 | 39.2 | 28.3 | 22.4 | 6.61 / 5.21 |
| best | tcp | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 17.4 | 23.6 | 35.3 | 127.0 | 1742.4 | 0 | 0 | 1/1 | 39.2 | 28.3 | 22.4 | 6.61 / 5.21 |

Run 1's profile:

| profile: the window, 30.0 s, 2998275 messages, 2998275 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 33.15G | 11.06k | 11.06k | 7960 | 3097 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.315 | 5.193 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 1.000 | 0.000000334 | 0.000000334 | 0 | 0.000000334 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 2.240M | 0.747 | 0.747 | 0 | 0.747 | - | - | - |
| involuntary context switches | 1097 | 0.000366 | 0.000366 | 0.000190 | 0.000175 | - | - | - |
| cycles | - | - | - | 30.62k | 4080 | - | - | - |
| instructions | - | - | - | 53.49k | 4600 | - | - | - |
| cache references | - | - | - | 2.449 | 2.819 | - | - | - |
| cache misses | - | - | - | 0.823 | 0.611 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00823 | 0.0104 | - | - | - |
| data TLB read misses | - | - | - | 0.0302 | 0.0593 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.160M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.160M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × wait | 3547539 | 99928 | 4 | 0 | 354753900 | 299827400 | 0 | 143.4 | 219.1 | 327.7 | 1589.2 | 12089.4 | 0 | 0 | 100/100 | 264.6 | 19.1 | 107.4 | 9.09 / 15.92 |
| best | tcp | 22 × wait | 3547539 | 99928 | 4 | 0 | 354753900 | 299827400 | 0 | 143.4 | 219.1 | 327.7 | 1589.2 | 12089.4 | 0 | 0 | 100/100 | 264.6 | 19.1 | 107.4 | 9.09 / 15.92 |

Run 1's profile:

| profile: the window, 30.0 s, 2998279 messages, 299828056 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 223.7G | 74.60k | 746.0 | 5382 | 692.2 | - | - | - |
| waiting for a CPU, ns | - | - | - | 8.753 | 57.66 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 2.000 | 0.000000667 | 0.000000007 | 0 | 0.000000007 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 15.48M | 5.164 | 0.0516 | 0 | 0.0516 | - | - | - |
| involuntary context switches | 128.7k | 0.0429 | 0.000429 | 0.000567 | 0.000424 | - | - | - |
| cycles | - | - | - | 18.61k | 1157 | - | - | - |
| instructions | - | - | - | 29.26k | 1658 | - | - | - |
| cache references | - | - | - | 1.095 | 1.794 | - | - | - |
| cache misses | - | - | - | 0.249 | 0.0901 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00766 | 0.00102 | - | - | - |
| data TLB read misses | - | - | - | 0.0102 | 0.0570 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 16.41M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 16.41M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × wait | 3547538 | 99928 | 5 | 0 | 3547538000 | 2998274000 | 0 | 6815.7 | 15859.7 | 38797.3 | 92274.7 | 220401.7 | 0 | 0 | 1000/1000 | 521.5 | 16.6 | 177.2 | 8.14 / 18.32 |
| best | tcp | 22 × wait | 3547538 | 99928 | 5 | 0 | 3547538000 | 2998274000 | 0 | 6815.7 | 15859.7 | 38797.3 | 92274.7 | 220401.7 | 0 | 0 | 1000/1000 | 521.5 | 16.6 | 177.2 | 8.14 / 18.32 |

Run 1's profile:

| profile: the window, 30.0 s, 2998280 messages, 2997719021 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 447.9G | 149.4k | 149.4 | 4674 | 144.7 | - | - | - |
| waiting for a CPU, ns | - | - | - | 121.9 | 14.04 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 24.00 | 0.00000800 | 0.000000008 | 0 | 0.000000008 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 335.4k | 0.112 | 0.000112 | 0 | 0.000112 | - | - | - |
| involuntary context switches | 122.0k | 0.0407 | 0.0000407 | 0.000757 | 0.0000399 | - | - | - |
| cycles | - | - | - | 14.13k | 433.3 | - | - | - |
| instructions | - | - | - | 20.75k | 1169 | - | - | - |
| cache references | - | - | - | 3.617 | 0.428 | - | - | - |
| cache misses | - | - | - | 1.524 | 0.241 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0712 | 0.0222 | - | - | - |
| data TLB read misses | - | - | - | 0.0278 | 0.00976 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 85.80M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 85.80M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 12.5 | 13.2 | 18.2 | 20.5 | 133.7 | 0 | 0 | 1/1 | 69.5 | 34.7 | 34.1 | 7.57 / 5.74 |
| best | tcp | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 12.5 | 13.2 | 18.2 | 20.5 | 133.7 | 0 | 0 | 1/1 | 69.5 | 34.7 | 34.1 | 7.57 / 5.74 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 299499 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 58.71G | 196.0k | 196.0k | 97.96k | 98.06k | - | - | - |
| waiting for a CPU, ns | - | - | - | 2.259 | 9.264 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 306.0 | 0.00102 | 0.00102 | 0.000250 | 0.000771 | - | - | - |
| cycles | - | - | - | 350.4k | 649.8k | - | - | - |
| instructions | - | - | - | 657.7k | 1.122M | - | - | - |
| cache references | - | - | - | 1.148 | 6.554 | - | - | - |
| cache misses | - | - | - | 0.126 | 2.156 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0142 | 0.0162 | - | - | - |
| data TLB read misses | - | - | - | 0.000377 | 0.0653 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.164M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.164M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × busy | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 90.1 | 147.5 | 2523.1 | 36700.2 | 73849.8 | 0 | 0 | 100/100 | 670.5 | 33.6 | 172.4 | 12.16 / 20.50 |
| best | tcp | 22 × busy | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 90.1 | 147.5 | 2523.1 | 36700.2 | 73849.8 | 0 | 0 | 100/100 | 670.5 | 33.6 | 172.4 | 12.16 / 20.50 |

Run 1's profile:

| profile: the window, 30.0 s, 299498 messages, 29949638 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 564.6G | 1.885M | 18.85k | 94.77k | 17.90k | - | - | - |
| waiting for a CPU, ns | - | - | - | 756.4 | 273.3 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 524.0 | 0.00175 | 0.0000175 | 0 | 0.0000175 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 32.19k | 0.107 | 0.00107 | 0.00778 | 0.000997 | - | - | - |
| cycles | - | - | - | 418.1k | 57.52k | - | - | - |
| instructions | - | - | - | 706.8k | 85.40k | - | - | - |
| cache references | - | - | - | 4.253 | 0.740 | - | - | - |
| cache misses | - | - | - | 0.389 | 0.149 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00919 | 0.00285 | - | - | - |
| data TLB read misses | - | - | - | 0.169 | 0.0148 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 14.06M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 14.06M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × busy | 354590 | 9988 | 0 | 0 | 354590000 | 299497000 | 0 | 1622.0 | 276824.1 | 369098.8 | 398458.9 | 404304.1 | 0 | 0 | 1000/1000 | 680.3 | 33.7 | 96.3 | 29.50 / 29.88 |
| best | tcp | 22 × busy | 354590 | 9988 | 0 | 0 | 354590000 | 299497000 | 0 | 1622.0 | 276824.1 | 369098.8 | 398458.9 | 404304.1 | 0 | 0 | 1000/1000 | 680.3 | 33.7 | 96.3 | 29.50 / 29.88 |

Run 1's profile:

| profile: the window, 30.0 s, 299505 messages, 296909159 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 548.9G | 1.833M | 1849 | 94.91k | 1753 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1009 | 36.55 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 4437 | 0.0148 | 0.0000149 | 0 | 0.0000149 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 25.05k | 0.0836 | 0.0000844 | 0.00284 | 0.0000815 | - | - | - |
| cycles | - | - | - | 228.6k | 4842 | - | - | - |
| instructions | - | - | - | 341.5k | 7601 | - | - | - |
| cache references | - | - | - | 2.675 | 0.889 | - | - | - |
| cache misses | - | - | - | 0.386 | 0.143 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0171 | 0.00128 | - | - | - |
| data TLB read misses | - | - | - | 0.148 | 0.00386 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 72.27M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 72.27M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 13.3 | 18.4 | 23.0 | 29.7 | 215.4 | 0 | 0 | 1/1 | 57.7 | 28.4 | 23.1 | 54.31 / 31.60 |
| best | tcp | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 13.3 | 18.4 | 23.0 | 29.7 | 215.4 | 0 | 0 | 1/1 | 57.7 | 28.4 | 23.1 | 54.31 / 31.60 |

Run 1's profile:

| profile: the window, 30.0 s, 2998275 messages, 2998275 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 48.67G | 16.23k | 16.23k | 7998 | 8236 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.409 | 1.163 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 1148 | 0.000383 | 0.000383 | 0.000198 | 0.000185 | - | - | - |
| cycles | - | - | - | 32.91k | 42.35k | - | - | - |
| instructions | - | - | - | 58.84k | 70.05k | - | - | - |
| cache references | - | - | - | 0.526 | 0.805 | - | - | - |
| cache misses | - | - | - | 0.0959 | 0.209 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000934 | 0.00115 | - | - | - |
| data TLB read misses | - | - | - | 0.00953 | 0.0113 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.017M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.017M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × busy | 3547539 | 99928 | 4 | 0 | 354753900 | 299827400 | 0 | 147.5 | 233.5 | 19660.8 | 30408.7 | 36600.5 | 0 | 0 | 100/100 | 637.5 | 18.2 | 117.2 | 14.29 / 21.74 |
| best | tcp | 22 × busy | 3547539 | 99928 | 4 | 0 | 354753900 | 299827400 | 0 | 147.5 | 233.5 | 19660.8 | 30408.7 | 36600.5 | 0 | 0 | 100/100 | 637.5 | 18.2 | 117.2 | 14.29 / 21.74 |

Run 1's profile:

| profile: the window, 30.0 s, 2998047 messages, 299805378 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 536.1G | 178.8k | 1788 | 4801 | 1740 | - | - | - |
| waiting for a CPU, ns | - | - | - | 56.32 | 20.61 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 500.0 | 0.000167 | 0.00000167 | 0 | 0.00000167 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 115.3k | 0.0384 | 0.000384 | 0.000543 | 0.000379 | - | - | - |
| cycles | - | - | - | 17.80k | 5162 | - | - | - |
| instructions | - | - | - | 27.06k | 7827 | - | - | - |
| cache references | - | - | - | 0.565 | 0.0874 | - | - | - |
| cache misses | - | - | - | 0.0663 | 0.0119 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00136 | 0.000187 | - | - | - |
| data TLB read misses | - | - | - | 0.0120 | 0.00274 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 16.41M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 16.41M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × busy | 3547543 | 99928 | 0 | 0 | 3547543000 | 2998274000 | 0 | 4915.2 | 26476.5 | 100663.3 | 148897.8 | 284379.0 | 0 | 0 | 1000/1000 | 726.9 | 20.7 | 175.8 | 24.57 / 28.32 |
| best | tcp | 22 × busy | 3547543 | 99928 | 0 | 0 | 3547543000 | 2998274000 | 0 | 4915.2 | 26476.5 | 100663.3 | 148897.8 | 284379.0 | 0 | 0 | 1000/1000 | 726.9 | 20.7 | 175.8 | 24.57 / 28.32 |

Run 1's profile:

| profile: the window, 30.0 s, 2998273 messages, 2995625035 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 594.6G | 198.3k | 198.5 | 5750 | 192.7 | - | - | - |
| waiting for a CPU, ns | - | - | - | 122.7 | 9.664 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 3.000 | 0.00000100 | 0.000000001 | 0 | 0.000000001 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 89.74k | 0.0299 | 0.0000300 | 0.000568 | 0.0000294 | - | - | - |
| cycles | - | - | - | 8525 | 660.3 | - | - | - |
| instructions | - | - | - | 11.88k | 1398 | - | - | - |
| cache references | - | - | - | 2.311 | 0.361 | - | - | - |
| cache misses | - | - | - | 0.979 | 0.222 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0573 | 0.0126 | - | - | - |
| data TLB read misses | - | - | - | 0.00759 | 0.00298 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 85.94M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 85.94M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 15.4 | 22.8 | 54.8 | 101.4 | 576.3 | 0 | 0 | 1/1 | 36.6 | 34.7 | 33.9 | 27.04 / 16.09 |
| best | tcp | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 15.4 | 22.8 | 54.8 | 101.4 | 576.3 | 0 | 0 | 1/1 | 36.6 | 34.7 | 33.9 | 27.04 / 16.09 |

Run 1's profile:

| profile: the window, 30.0 s, 299586 messages, 299585 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 30.96G | 103.4k | 103.4k | 97.87k | 5484 | - | - | - |
| waiting for a CPU, ns | - | - | - | 13.07 | 4.334 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 280.0k | 0.935 | 0.935 | 0 | 0.935 | - | - | - |
| involuntary context switches | 234.0 | 0.000781 | 0.000781 | 0.000714 | 0.0000668 | - | - | - |
| cycles | - | - | - | 546.8k | 5822 | - | - | - |
| instructions | - | - | - | 1.007M | 5603 | - | - | - |
| cache references | - | - | - | 125.4 | 6.846 | - | - | - |
| cache misses | - | - | - | 18.75 | 0.559 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00524 | 0.00683 | - | - | - |
| data TLB read misses | - | - | - | 2.778 | 0.297 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 5.939M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 5.939M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 86.0 | 137.2 | 174.1 | 352.3 | 32481.9 | 0 | 0 | 100/100 | 179.4 | 34.3 | 172.5 | 2.33 / 9.47 |
| best | tcp | 22 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 86.0 | 137.2 | 174.1 | 352.3 | 32481.9 | 0 | 0 | 100/100 | 179.4 | 34.3 | 172.5 | 2.33 / 9.47 |

Run 1's profile:

| profile: the window, 30.0 s, 299581 messages, 29958260 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 151.5G | 505.7k | 5057 | 96.60k | 4091 | - | - | - |
| waiting for a CPU, ns | - | - | - | 40.26 | 55.16 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 6.000 | 0.0000200 | 0.000000200 | 0 | 0.000000200 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 7.717M | 25.76 | 0.258 | 0 | 0.258 | - | - | - |
| involuntary context switches | 24.54k | 0.0819 | 0.000819 | 0.00406 | 0.000779 | - | - | - |
| cycles | - | - | - | 461.4k | 5692 | - | - | - |
| instructions | - | - | - | 817.1k | 5217 | - | - | - |
| cache references | - | - | - | 7.238 | 5.311 | - | - | - |
| cache misses | - | - | - | 0.641 | 0.276 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0142 | 0.00347 | - | - | - |
| data TLB read misses | - | - | - | 0.100 | 0.191 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 10.93M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 10.93M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × wait | 354798 | 9994 | 0 | 0 | 354798000 | 299583000 | 0 | 1097.7 | 436207.6 | 964689.9 | 968690.7 | 968690.7 | 0 | 0 | 1000/1000 | 222.1 | 34.3 | 93.5 | 4.06 / 11.87 |
| best | tcp | 22 × wait | 354798 | 9994 | 0 | 0 | 354798000 | 299583000 | 0 | 1097.7 | 436207.6 | 964689.9 | 968690.7 | 968690.7 | 0 | 0 | 1000/1000 | 222.1 | 34.3 | 93.5 | 4.06 / 11.87 |

Run 1's profile:

| profile: the window, 30.0 s, 299583 messages, 298619711 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 187.5G | 626.0k | 628.0 | 96.66k | 531.0 | - | - | - |
| waiting for a CPU, ns | - | - | - | 26.57 | 12.43 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 632.0 | 0.00211 | 0.00000212 | 0.00000334 | 0.00000211 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 8.543M | 28.52 | 0.0286 | 0.00000334 | 0.0286 | - | - | - |
| involuntary context switches | 22.02k | 0.0735 | 0.0000737 | 0.00125 | 0.0000725 | - | - | - |
| cycles | - | - | - | 434.7k | 1031 | - | - | - |
| instructions | - | - | - | 793.3k | 1635 | - | - | - |
| cache references | - | - | - | 5.535 | 4.184 | - | - | - |
| cache misses | - | - | - | 1.515 | 0.396 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0577 | 0.00537 | - | - | - |
| data TLB read misses | - | - | - | 0.0209 | 0.110 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 33.90M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 33.90M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 16.3 | 21.8 | 26.6 | 65.5 | 287.3 | 0 | 0 | 1/1 | 39.0 | 28.4 | 22.4 | 6.28 / 4.62 |
| best | tcp | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 16.3 | 21.8 | 26.6 | 65.5 | 287.3 | 0 | 0 | 1/1 | 39.0 | 28.4 | 22.4 | 6.28 / 4.62 |

Run 1's profile:

| profile: the window, 30.0 s, 3000437 messages, 3000436 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 32.95G | 10.98k | 10.98k | 8008 | 2973 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.274 | 1.671 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 2.415M | 0.805 | 0.805 | 0 | 0.805 | - | - | - |
| involuntary context switches | 896.0 | 0.000299 | 0.000299 | 0.000192 | 0.000107 | - | - | - |
| cycles | - | - | - | 25.24k | 3797 | - | - | - |
| instructions | - | - | - | 46.94k | 4479 | - | - | - |
| cache references | - | - | - | 0.137 | 0.952 | - | - | - |
| cache misses | - | - | - | 0.0120 | 0.0521 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000695 | 0.000588 | - | - | - |
| data TLB read misses | - | - | - | 0.00318 | 0.0369 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.156M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.156M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × wait | 3550038 | 99998 | 0 | 0 | 355003800 | 300043700 | 0 | 126.0 | 190.5 | 274.4 | 450.6 | 7607.9 | 0 | 0 | 100/100 | 263.1 | 21.3 | 107.3 | 1.65 / 9.75 |
| best | tcp | 22 × wait | 3550038 | 99998 | 0 | 0 | 355003800 | 300043700 | 0 | 126.0 | 190.5 | 274.4 | 450.6 | 7607.9 | 0 | 0 | 100/100 | 263.1 | 21.3 | 107.3 | 1.65 / 9.75 |

Run 1's profile:

| profile: the window, 30.0 s, 3000434 messages, 300042895 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 222.5G | 74.15k | 741.5 | 6084 | 680.7 | - | - | - |
| waiting for a CPU, ns | - | - | - | 6.922 | 11.34 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 582.0 | 0.000194 | 0.00000194 | 0 | 0.00000194 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 17.33M | 5.775 | 0.0577 | 0 | 0.0577 | - | - | - |
| involuntary context switches | 38.61k | 0.0129 | 0.000129 | 0.000508 | 0.000124 | - | - | - |
| cycles | - | - | - | 22.45k | 1208 | - | - | - |
| instructions | - | - | - | 36.42k | 1712 | - | - | - |
| cache references | - | - | - | 0.762 | 0.586 | - | - | - |
| cache misses | - | - | - | 0.0901 | 0.0324 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00154 | 0.000427 | - | - | - |
| data TLB read misses | - | - | - | 0.00905 | 0.0212 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 15.44M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 15.44M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × wait | 3550038 | 99998 | 0 | 0 | 3550038000 | 3000437000 | 0 | 5701.6 | 9568.3 | 12976.1 | 15859.7 | 40899.0 | 0 | 0 | 1000/1000 | 590.0 | 18.3 | 184.5 | 8.43 / 17.28 |
| best | tcp | 22 × wait | 3550038 | 99998 | 0 | 0 | 3550038000 | 3000437000 | 0 | 5701.6 | 9568.3 | 12976.1 | 15859.7 | 40899.0 | 0 | 0 | 1000/1000 | 590.0 | 18.3 | 184.5 | 8.43 / 17.28 |

Run 1's profile:

| profile: the window, 30.0 s, 3000424 messages, 3000548799 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 493.9G | 164.6k | 164.6 | 5052 | 159.6 | - | - | - |
| waiting for a CPU, ns | - | - | - | 41.95 | 1.173 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 269.9k | 0.0899 | 0.0000899 | 0 | 0.0000899 | - | - | - |
| involuntary context switches | 15.59k | 0.00520 | 0.00000520 | 0.000201 | 0.00000500 | - | - | - |
| cycles | - | - | - | 17.59k | 454.3 | - | - | - |
| instructions | - | - | - | 27.82k | 1218 | - | - | - |
| cache references | - | - | - | 2.799 | 0.498 | - | - | - |
| cache misses | - | - | - | 1.445 | 0.258 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0603 | 0.0280 | - | - | - |
| data TLB read misses | - | - | - | 0.00918 | 0.0107 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 86.06M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 86.06M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 12.5 | 12.9 | 17.9 | 19.7 | 134.5 | 0 | 0 | 1/1 | 69.5 | 34.7 | 34.1 | 11.22 / 7.70 |
| best | tcp | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 12.5 | 12.9 | 17.9 | 19.7 | 134.5 | 0 | 0 | 1/1 | 69.5 | 34.7 | 34.1 | 11.22 / 7.70 |

Run 1's profile:

| profile: the window, 30.0 s, 299587 messages, 299587 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 58.72G | 196.0k | 196.0k | 97.94k | 98.06k | - | - | - |
| waiting for a CPU, ns | - | - | - | 9.991 | 1.283 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 369.0 | 0.00123 | 0.00123 | 0.00106 | 0.000174 | - | - | - |
| cycles | - | - | - | 350.4k | 392.6k | - | - | - |
| instructions | - | - | - | 657.6k | 680.9k | - | - | - |
| cache references | - | - | - | 0.465 | 0.420 | - | - | - |
| cache misses | - | - | - | 0.0527 | 0.0756 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00496 | 0.00604 | - | - | - |
| data TLB read misses | - | - | - | 0.000127 | 0.0000267 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 5.951M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 5.951M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × busy | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 86.0 | 139.3 | 180.2 | 3473.4 | 23906.9 | 0 | 0 | 100/100 | 676.1 | 34.3 | 175.0 | 2.91 / 15.42 |
| best | tcp | 22 × busy | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 86.0 | 139.3 | 180.2 | 3473.4 | 23906.9 | 0 | 0 | 100/100 | 676.1 | 34.3 | 175.0 | 2.91 / 15.42 |

Run 1's profile:

| profile: the window, 30.0 s, 299579 messages, 29957876 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 570.6G | 1.905M | 19.05k | 96.55k | 18.08k | - | - | - |
| waiting for a CPU, ns | - | - | - | 360.8 | 37.29 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 149.0 | 0.000497 | 0.00000497 | 0 | 0.00000497 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 17.89k | 0.0597 | 0.000597 | 0.00521 | 0.000545 | - | - | - |
| cycles | - | - | - | 267.1k | 44.82k | - | - | - |
| instructions | - | - | - | 468.4k | 66.11k | - | - | - |
| cache references | - | - | - | 2.002 | 0.200 | - | - | - |
| cache misses | - | - | - | 0.121 | 0.0175 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00228 | 0.000233 | - | - | - |
| data TLB read misses | - | - | - | 0.0969 | 0.00744 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 12.21M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 12.21M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × busy | 354798 | 9994 | 0 | 0 | 354798000 | 299583000 | 0 | 1146.9 | 2785.3 | 27000.8 | 36700.2 | 39477.2 | 0 | 0 | 1000/1000 | 682.7 | 33.7 | 95.3 | 30.19 / 30.60 |
| best | tcp | 22 × busy | 354798 | 9994 | 0 | 0 | 354798000 | 299583000 | 0 | 1146.9 | 2785.3 | 27000.8 | 36700.2 | 39477.2 | 0 | 0 | 1000/1000 | 682.7 | 33.7 | 95.3 | 30.19 / 30.60 |

Run 1's profile:

| profile: the window, 30.0 s, 299581 messages, 299582408 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 556.2G | 1.857M | 1857 | 95.01k | 1762 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1082 | 6.663 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 8471 | 0.0283 | 0.0000283 | 0 | 0.0000283 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 12.83k | 0.0428 | 0.0000428 | 0.00824 | 0.0000346 | - | - | - |
| cycles | - | - | - | 232.0k | 4264 | - | - | - |
| instructions | - | - | - | 360.5k | 6751 | - | - | - |
| cache references | - | - | - | 3.172 | 0.554 | - | - | - |
| cache misses | - | - | - | 0.327 | 0.0479 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0156 | 0.000256 | - | - | - |
| data TLB read misses | - | - | - | 0.197 | 0.00204 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 70.57M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 70.57M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 1 × busy | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 14.3 | 19.7 | 25.3 | 38.9 | 295.9 | 0 | 0 | 1/1 | 57.1 | 28.0 | 22.7 | 44.27 / 26.45 |
| best | tcp | 1 × busy | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 14.3 | 19.7 | 25.3 | 38.9 | 295.9 | 0 | 0 | 1/1 | 57.1 | 28.0 | 22.7 | 44.27 / 26.45 |

Run 1's profile:

| profile: the window, 30.0 s, 3000450 messages, 3000449 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 48.22G | 16.07k | 16.07k | 7898 | 8175 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.189 | 0.795 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 1047 | 0.000349 | 0.000349 | 0.000193 | 0.000156 | - | - | - |
| cycles | - | - | - | 31.12k | 40.03k | - | - | - |
| instructions | - | - | - | 55.27k | 66.01k | - | - | - |
| cache references | - | - | - | 0.753 | 1.324 | - | - | - |
| cache misses | - | - | - | 0.186 | 0.382 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00604 | 0.00622 | - | - | - |
| data TLB read misses | - | - | - | 0.00926 | 0.0190 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.095M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.095M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × busy | 3550038 | 99998 | 0 | 0 | 355003800 | 300043700 | 0 | 124.9 | 190.5 | 1212.4 | 21495.8 | 51714.8 | 0 | 0 | 100/100 | 640.0 | 22.6 | 113.5 | 5.08 / 17.11 |
| best | tcp | 22 × busy | 3550038 | 99998 | 0 | 0 | 355003800 | 300043700 | 0 | 124.9 | 190.5 | 1212.4 | 21495.8 | 51714.8 | 0 | 0 | 100/100 | 640.0 | 22.6 | 113.5 | 5.08 / 17.11 |

Run 1's profile:

| profile: the window, 30.0 s, 3000414 messages, 300041673 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 539.7G | 179.9k | 1799 | 6334 | 1735 | - | - | - |
| waiting for a CPU, ns | - | - | - | 12.74 | 12.34 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 21.93k | 0.00731 | 0.0000731 | 0.000182 | 0.0000713 | - | - | - |
| cycles | - | - | - | 11.54k | 4214 | - | - | - |
| instructions | - | - | - | 15.95k | 6459 | - | - | - |
| cache references | - | - | - | 0.175 | 0.128 | - | - | - |
| cache misses | - | - | - | 0.0195 | 0.0407 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000361 | 0.000150 | - | - | - |
| data TLB read misses | - | - | - | 0.00994 | 0.00339 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 16.62M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 16.62M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × busy | 3548533 | 99956 | 1505 | 0 | 3548533000 | 3000437000 | 0 | 4030.5 | 7340.0 | 76546.0 | 385876.0 | 538566.3 | 0 | 0 | 1000/1000 | 748.1 | 20.7 | 174.3 | 24.90 / 28.17 |
| best | tcp | 22 × busy | 3548533 | 99956 | 1505 | 0 | 3548533000 | 3000437000 | 0 | 4030.5 | 7340.0 | 76546.0 | 385876.0 | 538566.3 | 0 | 0 | 1000/1000 | 748.1 | 20.7 | 174.3 | 24.90 / 28.17 |

Run 1's profile:

| profile: the window, 30.0 s, 3000214 messages, 2992133484 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 607.6G | 202.5k | 203.1 | 5864 | 197.2 | - | - | - |
| waiting for a CPU, ns | - | - | - | 32.22 | 1.710 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 2.000 | 0.000000667 | 0.000000001 | 0 | 0.000000001 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 16.94k | 0.00564 | 0.00000566 | 0.000222 | 0.00000544 | - | - | - |
| cycles | - | - | - | 10.60k | 449.4 | - | - | - |
| instructions | - | - | - | 13.51k | 953.4 | - | - | - |
| cache references | - | - | - | 3.902 | 0.259 | - | - | - |
| cache misses | - | - | - | 1.717 | 0.156 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0399 | 0.00670 | - | - | - |
| data TLB read misses | - | - | - | 0.0140 | 0.00144 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 86.00M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 86.00M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 15.4 | 22.5 | 54.8 | 105.5 | 904.8 | 0 | 0 | 1/1 | 36.7 | 34.7 | 33.9 | 3.36 / 2.81 |
| best | tcp | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 15.4 | 22.5 | 54.8 | 105.5 | 904.8 | 0 | 0 | 1/1 | 36.7 | 34.7 | 33.9 | 3.36 / 2.81 |

Run 1's profile:

| profile: the window, 30.0 s, 300766 messages, 300766 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 30.94G | 102.9k | 102.9k | 97.56k | 5325 | - | - | - |
| waiting for a CPU, ns | - | - | - | 2.591 | 6.422 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 285.2k | 0.948 | 0.948 | 0 | 0.948 | - | - | - |
| involuntary context switches | 113.0 | 0.000376 | 0.000376 | 0.000319 | 0.0000565 | - | - | - |
| cycles | - | - | - | 349.8k | 5473 | - | - | - |
| instructions | - | - | - | 654.4k | 5473 | - | - | - |
| cache references | - | - | - | 1.368 | 8.445 | - | - | - |
| cache misses | - | - | - | 0.0507 | 0.574 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00480 | 0.00691 | - | - | - |
| data TLB read misses | - | - | - | 0.000628 | 0.191 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.091M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.091M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 89.1 | 139.3 | 184.3 | 348.2 | 16394.4 | 0 | 0 | 100/100 | 177.0 | 34.4 | 171.9 | 4.70 / 11.31 |
| best | tcp | 22 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 89.1 | 139.3 | 184.3 | 348.2 | 16394.4 | 0 | 0 | 100/100 | 177.0 | 34.4 | 171.9 | 4.70 / 11.31 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 30076890 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 149.4G | 496.9k | 4969 | 96.68k | 4002 | - | - | - |
| waiting for a CPU, ns | - | - | - | 15.46 | 257.1 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 26.00 | 0.0000864 | 0.000000864 | 0 | 0.000000864 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 6.746M | 22.43 | 0.224 | 0 | 0.224 | - | - | - |
| involuntary context switches | 59.12k | 0.197 | 0.00197 | 0.00155 | 0.00195 | - | - | - |
| cycles | - | - | - | 460.0k | 5511 | - | - | - |
| instructions | - | - | - | 825.1k | 5070 | - | - | - |
| cache references | - | - | - | 6.493 | 12.33 | - | - | - |
| cache misses | - | - | - | 1.014 | 0.409 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0179 | 0.00403 | - | - | - |
| data TLB read misses | - | - | - | 0.0905 | 0.632 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 11.06M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 11.06M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × wait | 356101 | 10031 | 0 | 0 | 356101000 | 300769000 | 0 | 1032.2 | 118489.1 | 236978.2 | 249561.1 | 251844.0 | 0 | 0 | 1000/1000 | 218.3 | 34.1 | 89.1 | 8.40 / 12.57 |
| best | tcp | 22 × wait | 356101 | 10031 | 0 | 0 | 356101000 | 300769000 | 0 | 1032.2 | 118489.1 | 236978.2 | 249561.1 | 251844.0 | 0 | 0 | 1000/1000 | 218.3 | 34.1 | 89.1 | 8.40 / 12.57 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 300847960 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 184.1G | 612.1k | 612.0 | 95.83k | 516.1 | - | - | - |
| waiting for a CPU, ns | - | - | - | 33.58 | 10.70 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 569.0 | 0.00189 | 0.00000189 | 0 | 0.00000189 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 7.976M | 26.52 | 0.0265 | 0 | 0.0265 | - | - | - |
| involuntary context switches | 17.72k | 0.0589 | 0.0000589 | 0.00177 | 0.0000571 | - | - | - |
| cycles | - | - | - | 429.9k | 1007 | - | - | - |
| instructions | - | - | - | 760.2k | 1614 | - | - | - |
| cache references | - | - | - | 7.475 | 3.557 | - | - | - |
| cache misses | - | - | - | 1.852 | 0.405 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0529 | 0.00723 | - | - | - |
| data TLB read misses | - | - | - | 0.0344 | 0.0748 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 32.33M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 32.33M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 1 × wait | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 17.4 | 23.6 | 39.9 | 352.3 | 2554.7 | 0 | 0 | 1/1 | 39.2 | 28.3 | 22.4 | 4.22 / 4.07 |
| best | tcp | 1 × wait | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 17.4 | 23.6 | 39.9 | 352.3 | 2554.7 | 0 | 0 | 1/1 | 39.2 | 28.3 | 22.4 | 4.22 / 4.07 |

Run 1's profile:

| profile: the window, 30.0 s, 3002639 messages, 3002639 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 33.16G | 11.04k | 11.04k | 7950 | 3094 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.090 | 15.37 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 2.160M | 0.719 | 0.719 | 0 | 0.719 | - | - | - |
| involuntary context switches | 946.0 | 0.000315 | 0.000315 | 0.000162 | 0.000153 | - | - | - |
| cycles | - | - | - | 31.13k | 4108 | - | - | - |
| instructions | - | - | - | 55.30k | 4771 | - | - | - |
| cache references | - | - | - | 1.285 | 1.692 | - | - | - |
| cache misses | - | - | - | 0.440 | 0.448 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00988 | 0.00815 | - | - | - |
| data TLB read misses | - | - | - | 0.00825 | 0.0299 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.267M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.267M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × wait | 3554018 | 100110 | 0 | 0 | 355401800 | 300263100 | 0 | 145.4 | 231.4 | 581.6 | 1900.5 | 5742.3 | 0 | 0 | 100/100 | 226.6 | 21.3 | 108.8 | 7.22 / 13.52 |
| best | tcp | 22 × wait | 3554018 | 100110 | 0 | 0 | 355401800 | 300263100 | 0 | 145.4 | 231.4 | 581.6 | 1900.5 | 5742.3 | 0 | 0 | 100/100 | 226.6 | 21.3 | 108.8 | 7.22 / 13.52 |

Run 1's profile:

| profile: the window, 30.0 s, 3002639 messages, 300263075 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 191.6G | 63.81k | 638.1 | 5860 | 579.5 | - | - | - |
| waiting for a CPU, ns | - | - | - | 9.949 | 50.62 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 771.0 | 0.000257 | 0.00000257 | 0 | 0.00000257 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 10.75M | 3.579 | 0.0358 | 0 | 0.0358 | - | - | - |
| involuntary context switches | 141.3k | 0.0471 | 0.000471 | 0.000506 | 0.000466 | - | - | - |
| cycles | - | - | - | 20.11k | 1047 | - | - | - |
| instructions | - | - | - | 33.61k | 1641 | - | - | - |
| cache references | - | - | - | 1.034 | 1.967 | - | - | - |
| cache misses | - | - | - | 0.280 | 0.160 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00909 | 0.00281 | - | - | - |
| data TLB read misses | - | - | - | 0.00856 | 0.0662 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 16.10M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 16.10M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × wait | 3554016 | 100110 | 2 | 0 | 3554016000 | 3002631000 | 0 | 5439.5 | 9044.0 | 13369.3 | 23068.7 | 64687.7 | 0 | 0 | 1000/1000 | 556.8 | 16.4 | 185.2 | 10.24 / 18.16 |
| best | tcp | 22 × wait | 3554016 | 100110 | 2 | 0 | 3554016000 | 3002631000 | 0 | 5439.5 | 9044.0 | 13369.3 | 23068.7 | 64687.7 | 0 | 0 | 1000/1000 | 556.8 | 16.4 | 185.2 | 10.24 / 18.16 |

Run 1's profile:

| profile: the window, 30.0 s, 3002629 messages, 3002749185 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 465.9G | 155.2k | 155.2 | 4580 | 150.6 | - | - | - |
| waiting for a CPU, ns | - | - | - | 63.31 | 2.669 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 276.2k | 0.0920 | 0.0000920 | 0 | 0.0000920 | - | - | - |
| involuntary context switches | 20.76k | 0.00691 | 0.00000691 | 0.000251 | 0.00000666 | - | - | - |
| cycles | - | - | - | 14.89k | 439.5 | - | - | - |
| instructions | - | - | - | 22.33k | 1179 | - | - | - |
| cache references | - | - | - | 3.060 | 0.493 | - | - | - |
| cache misses | - | - | - | 1.509 | 0.255 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0659 | 0.0267 | - | - | - |
| data TLB read misses | - | - | - | 0.220 | 0.0126 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 86.01M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 86.01M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 12.4 | 12.8 | 17.7 | 19.7 | 60.5 | 0 | 0 | 1/1 | 69.5 | 34.7 | 34.1 | 3.27 / 3.15 |
| best | tcp | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 12.4 | 12.8 | 17.7 | 19.7 | 60.5 | 0 | 0 | 1/1 | 69.5 | 34.7 | 34.1 | 3.27 / 3.15 |

Run 1's profile:

| profile: the window, 30.0 s, 300766 messages, 300766 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 58.72G | 195.2k | 195.2k | 97.57k | 97.66k | - | - | - |
| waiting for a CPU, ns | - | - | - | 8.688 | 6.258 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 433.0 | 0.00144 | 0.00144 | 0.000841 | 0.000598 | - | - | - |
| cycles | - | - | - | 349.5k | 393.8k | - | - | - |
| instructions | - | - | - | 653.2k | 681.2k | - | - | - |
| cache references | - | - | - | 1.229 | 1.292 | - | - | - |
| cache misses | - | - | - | 0.0444 | 0.0733 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00455 | 0.00612 | - | - | - |
| data TLB read misses | - | - | - | 0.0000266 | 0.000166 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.156M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.156M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 97.3 | 155.6 | 2424.8 | 20185.1 | 49578.7 | 0 | 0 | 100/100 | 669.9 | 33.0 | 168.6 | 28.92 / 30.29 |
| best | tcp | 22 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 97.3 | 155.6 | 2424.8 | 20185.1 | 49578.7 | 0 | 0 | 100/100 | 669.9 | 33.0 | 168.6 | 28.92 / 30.29 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 30076854 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 565.9G | 1.881M | 18.81k | 93.31k | 17.88k | - | - | - |
| waiting for a CPU, ns | - | - | - | 649.2 | 73.99 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 181.0 | 0.000602 | 0.00000602 | 0 | 0.00000602 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 20.29k | 0.0675 | 0.000675 | 0.00229 | 0.000652 | - | - | - |
| cycles | - | - | - | 157.8k | 42.06k | - | - | - |
| instructions | - | - | - | 287.8k | 61.96k | - | - | - |
| cache references | - | - | - | 0.423 | 0.814 | - | - | - |
| cache misses | - | - | - | 0.0847 | 0.0802 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.00162 | - | - | - |
| data TLB read misses | - | - | - | 0.00755 | 0.0212 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 13.19M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 13.19M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × busy | 356087 | 10029 | 14 | 0 | 356087000 | 300769000 | 0 | 1900.5 | 620757.0 | 1031798.8 | 1107296.3 | 1157516.2 | 0 | 0 | 1000/1000 | 643.4 | 32.1 | 99.5 | 64.14 / 48.62 |
| best | tcp | 22 × busy | 356087 | 10029 | 14 | 0 | 356087000 | 300769000 | 0 | 1900.5 | 620757.0 | 1031798.8 | 1107296.3 | 1157516.2 | 0 | 0 | 1000/1000 | 643.4 | 32.1 | 99.5 | 64.14 / 48.62 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 303329888 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 522.8G | 1.738M | 1724 | 90.65k | 1634 | - | - | - |
| waiting for a CPU, ns | - | - | - | 3793 | 134.7 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 43.38k | 0.144 | 0.000143 | 0.00402 | 0.000139 | - | - | - |
| cycles | - | - | - | 363.0k | 6127 | - | - | - |
| instructions | - | - | - | 610.7k | 9390 | - | - | - |
| cache references | - | - | - | 14.38 | 1.855 | - | - | - |
| cache misses | - | - | - | 2.933 | 0.403 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0988 | 0.00708 | - | - | - |
| data TLB read misses | - | - | - | 0.240 | 0.0178 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 85.74M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 85.74M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 1 × busy | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 13.3 | 18.4 | 23.0 | 27.9 | 385.1 | 0 | 0 | 1/1 | 57.7 | 28.4 | 23.2 | 5.83 / 4.64 |
| best | tcp | 1 × busy | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 13.3 | 18.4 | 23.0 | 27.9 | 385.1 | 0 | 0 | 1/1 | 57.7 | 28.4 | 23.2 | 5.83 / 4.64 |

Run 1's profile:

| profile: the window, 30.0 s, 3002636 messages, 3002638 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 48.72G | 16.22k | 16.22k | 7978 | 8247 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.034 | 1.916 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 1078 | 0.000359 | 0.000359 | 0.000172 | 0.000188 | - | - | - |
| cycles | - | - | - | 25.32k | 43.33k | - | - | - |
| instructions | - | - | - | 46.37k | 71.17k | - | - | - |
| cache references | - | - | - | 0.112 | 25.22 | - | - | - |
| cache misses | - | - | - | 0.0135 | 6.602 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000878 | 0.00109 | - | - | - |
| data TLB read misses | - | - | - | 0.00269 | 0.494 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.156M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.156M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × busy | 3554018 | 100110 | 0 | 0 | 355401800 | 300263100 | 0 | 130.0 | 196.6 | 2555.9 | 19136.5 | 65859.8 | 0 | 0 | 100/100 | 638.1 | 23.5 | 115.7 | 24.25 / 27.44 |
| best | tcp | 22 × busy | 3554018 | 100110 | 0 | 0 | 355401800 | 300263100 | 0 | 130.0 | 196.6 | 2555.9 | 19136.5 | 65859.8 | 0 | 0 | 100/100 | 638.1 | 23.5 | 115.7 | 24.25 / 27.44 |

Run 1's profile:

| profile: the window, 30.0 s, 3002636 messages, 300262621 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 536.8G | 178.8k | 1788 | 6693 | 1721 | - | - | - |
| waiting for a CPU, ns | - | - | - | 8.873 | 9.883 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 987.0 | 0.000329 | 0.00000329 | 0 | 0.00000329 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 19.77k | 0.00658 | 0.0000658 | 0.000182 | 0.0000640 | - | - | - |
| cycles | - | - | - | 12.85k | 4263 | - | - | - |
| instructions | - | - | - | 21.05k | 6432 | - | - | - |
| cache references | - | - | - | 0.267 | 0.0525 | - | - | - |
| cache misses | - | - | - | 0.0237 | 0.00675 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000462 | 0.000116 | - | - | - |
| data TLB read misses | - | - | - | 0.00377 | 0.000812 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 16.41M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 16.41M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout tcp

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: tcp (127.0.0.1:14300)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | tcp | 22 × busy | 3554017 | 100110 | 1 | 0 | 3554017000 | 3002631000 | 0 | 4587.5 | 27000.8 | 55050.2 | 106954.8 | 327001.0 | 0 | 0 | 1000/1000 | 722.8 | 20.0 | 171.4 | 26.86 / 28.81 |
| best | tcp | 22 × busy | 3554017 | 100110 | 1 | 0 | 3554017000 | 3002631000 | 0 | 4587.5 | 27000.8 | 55050.2 | 106954.8 | 327001.0 | 0 | 0 | 1000/1000 | 722.8 | 20.0 | 171.4 | 26.86 / 28.81 |

Run 1's profile:

| profile: the window, 30.0 s, 3002621 messages, 3001351779 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 589.6G | 196.4k | 196.5 | 5619 | 190.8 | - | - | - |
| waiting for a CPU, ns | - | - | - | 66.09 | 3.619 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 2.000 | 0.000000666 | 0.000000001 | 0 | 0.000000001 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 2.000 | 0.000000666 | 0.000000001 | 0 | 0.000000001 | - | - | - |
| involuntary context switches | 18.85k | 0.00628 | 0.00000628 | 0.000310 | 0.00000597 | - | - | - |
| cycles | - | - | - | 10.06k | 505.8 | - | - | - |
| instructions | - | - | - | 12.12k | 1093 | - | - | - |
| cache references | - | - | - | 4.210 | 0.404 | - | - | - |
| cache misses | - | - | - | 1.763 | 0.235 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0567 | 0.0109 | - | - | - |
| data TLB read misses | - | - | - | 0.00673 | 0.00222 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 85.74M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 85.74M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
