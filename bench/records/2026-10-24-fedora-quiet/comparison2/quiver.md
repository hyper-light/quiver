# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 4.4 | 8.1 | 29.4 | 102.4 | 1703.4 | 0 | 0 | 1/1 | 36.9 | 35.4 | - | 3.06 / 2.61 |
| best | quiver | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 4.4 | 8.1 | 29.4 | 102.4 | 1703.4 | 0 | 0 | 1/1 | 36.9 | 35.4 | - | 3.06 / 2.61 |

Run 1's profile:

| profile: the window, 30.0 s, 299500 messages, 299500 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 31.18G | 104.1k | 104.1k | 99.99k | 4121 | - | - | - |
| waiting for a CPU, ns | - | - | - | 28.16 | 9.166 | - | - | - |
| allocations | 601.9k | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.00000334 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.00000334 | 0.00000334 | 0 | - | - | - |
| frees | 586.2k | 1.957 | 1.957 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.1 | 483.1 | 483.1 | 0 | - | - | - |
| minor page faults | 1771 | 0.00591 | 0.00591 | 0.00591 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 282.8k | 0.944 | 0.944 | 0 | 0.944 | - | - | - |
| involuntary context switches | 393.0 | 0.00131 | 0.00131 | 0.00122 | 0.0000935 | - | - | - |
| cycles | - | - | - | 929.6k | 3032 | - | - | - |
| instructions | - | - | - | 694.5k | 1867 | - | - | - |
| cache references | - | - | - | 61.75 | 45.36 | - | - | - |
| cache misses | - | - | - | 23.44 | 1.037 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.177 | 0.0387 | - | - | - |
| data TLB read misses | - | - | - | 0.157 | 0.718 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.87M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.87M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 10.4 | 20.2 | 39.4 | 78.8 | 2365.9 | 0 | 0 | 100/100 | 95.1 | 35.4 | - | 10.41 / 6.77 |
| best | quiver | 22 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 10.4 | 20.2 | 39.4 | 78.8 | 2365.9 | 0 | 0 | 100/100 | 95.1 | 35.4 | - | 10.41 / 6.77 |

Run 1's profile:

| profile: the window, 30.0 s, 299498 messages, 29949795 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 80.41G | 268.5k | 2685 | 99.84k | 1686 | - | - | - |
| waiting for a CPU, ns | - | - | - | 5.013 | 2.410 | - | - | - |
| allocations | 601.9k | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | - | - | - |
| frees | 586.2k | 1.957 | 0.0196 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.1 | 4.831 | 483.1 | 0 | - | - | - |
| minor page faults | 1770 | 0.00591 | 0.0000591 | 0.00591 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 8.038M | 26.84 | 0.268 | 0.00700 | 0.268 | - | - | - |
| involuntary context switches | 1265 | 0.00422 | 0.0000422 | 0.000791 | 0.0000343 | - | - | - |
| cycles | - | - | - | 791.3k | 2366 | - | - | - |
| instructions | - | - | - | 603.0k | 1193 | - | - | - |
| cache references | - | - | - | 200.2 | 11.57 | - | - | - |
| cache misses | - | - | - | 13.72 | 0.0505 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.569 | 0.00180 | - | - | - |
| data TLB read misses | - | - | - | 23.77 | 0.395 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.69M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.69M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 354590 | 9989 | 0 | 0 | 354590000 | 299497000 | 0 | 11.5 | 19.2 | 48.6 | 137.2 | 1994.9 | 0 | 0 | 1000/1000 | 110.1 | 35.4 | - | 11.05 / 8.52 |
| best | quiver | 22 × wait | 354590 | 9989 | 0 | 0 | 354590000 | 299497000 | 0 | 11.5 | 19.2 | 48.6 | 137.2 | 1994.9 | 0 | 0 | 1000/1000 | 110.1 | 35.4 | - | 11.05 / 8.52 |

Run 1's profile:

| profile: the window, 30.0 s, 299498 messages, 299498000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 93.12G | 310.9k | 310.9 | 99.85k | 211.1 | - | - | - |
| waiting for a CPU, ns | - | - | - | 11.86 | 0.283 | - | - | - |
| allocations | 601.9k | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.000000003 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.000000003 | 0.00000334 | 0 | - | - | - |
| frees | 586.2k | 1.957 | 0.00196 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.1 | 0.483 | 483.1 | 0 | - | - | - |
| minor page faults | 1770 | 0.00591 | 0.00000591 | 0.00591 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 6.274M | 20.95 | 0.0209 | 0.00362 | 0.0209 | - | - | - |
| involuntary context switches | 1893 | 0.00632 | 0.00000632 | 0.00194 | 0.00000438 | - | - | - |
| cycles | - | - | - | 787.1k | 826.3 | - | - | - |
| instructions | - | - | - | 593.9k | 992.5 | - | - | - |
| cache references | - | - | - | 221.9 | 8.019 | - | - | - |
| cache misses | - | - | - | 15.49 | 0.00690 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.571 | 0.000179 | - | - | - |
| data TLB read misses | - | - | - | 34.80 | 0.0820 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.52M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.52M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 4.2 | 4.7 | 9.6 | 80.9 | 2150.4 | 0 | 0 | 1/1 | 43.4 | 35.4 | - | 5.21 / 3.72 |
| best | quiver | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 4.2 | 4.7 | 9.6 | 80.9 | 2150.4 | 0 | 0 | 1/1 | 43.4 | 35.4 | - | 5.21 / 3.72 |

Run 1's profile:

| profile: the window, 30.0 s, 2998279 messages, 2998279 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 36.63G | 12.22k | 12.22k | 9989 | 2227 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.910 | 4.218 | - | - | - |
| allocations | 6.026M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.026M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.448G | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 6.000 | 0.00000200 | 0.00000200 | 0.00000200 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 2.109M | 0.704 | 0.704 | 0 | 0.704 | - | - | - |
| involuntary context switches | 342.0 | 0.000114 | 0.000114 | 0.0000870 | 0.0000270 | - | - | - |
| cycles | - | - | - | 93.14k | 2545 | - | - | - |
| instructions | - | - | - | 73.71k | 1614 | - | - | - |
| cache references | - | - | - | 39.75 | 31.91 | - | - | - |
| cache misses | - | - | - | 14.51 | 0.238 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.174 | 0.00703 | - | - | - |
| data TLB read misses | - | - | - | 0.142 | 0.299 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.89M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.89M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 3547543 | 99930 | 0 | 0 | 354754300 | 299827400 | 0 | 13.7 | 40.4 | 85.0 | 147.5 | 5007.0 | 0 | 0 | 100/100 | 325.7 | 34.6 | - | 15.92 / 14.35 |
| best | quiver | 22 × wait | 3547543 | 99930 | 0 | 0 | 354754300 | 299827400 | 0 | 13.7 | 40.4 | 85.0 | 147.5 | 5007.0 | 0 | 0 | 100/100 | 325.7 | 34.6 | - | 15.92 / 14.35 |

Run 1's profile:

| profile: the window, 30.0 s, 2998279 messages, 299827715 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 277.2G | 92.45k | 924.5 | 9750 | 827.0 | - | - | - |
| waiting for a CPU, ns | - | - | - | 6.115 | 1.929 | - | - | - |
| allocations | 6.026M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.026M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.448G | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 35.68M | 11.90 | 0.119 | 0.0566 | 0.118 | - | - | - |
| involuntary context switches | 14.81k | 0.00494 | 0.0000494 | 0.000390 | 0.0000455 | - | - | - |
| cycles | - | - | - | 52.07k | 1516 | - | - | - |
| instructions | - | - | - | 33.85k | 961.9 | - | - | - |
| cache references | - | - | - | 144.1 | 7.830 | - | - | - |
| cache misses | - | - | - | 12.56 | 0.0346 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.483 | 0.000307 | - | - | - |
| data TLB read misses | - | - | - | 7.040 | 0.0518 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.87M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.87M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 3547543 | 99930 | 0 | 0 | 3547543000 | 2998274000 | 0 | 12.9 | 50.2 | 157.7 | 315.4 | 4066.1 | 0 | 0 | 1000/1000 | 425.1 | 35.2 | - | 18.32 / 16.91 |
| best | quiver | 22 × wait | 3547543 | 99930 | 0 | 0 | 3547543000 | 2998274000 | 0 | 12.9 | 50.2 | 157.7 | 315.4 | 4066.1 | 0 | 0 | 1000/1000 | 425.1 | 35.2 | - | 18.32 / 16.91 |

Run 1's profile:

| profile: the window, 30.0 s, 2998282 messages, 2998282000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 360.1G | 120.1k | 120.1 | 9926 | 110.2 | - | - | - |
| waiting for a CPU, ns | - | - | - | 7.098 | 1.562 | - | - | - |
| allocations | 6.026M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.026M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.448G | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 12.00 | 0.00000400 | 0.000000004 | 0.00000200 | 0.000000002 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 22.25M | 7.420 | 0.00742 | 0.00811 | 0.00741 | - | - | - |
| involuntary context switches | 18.47k | 0.00616 | 0.00000616 | 0.000223 | 0.00000594 | - | - | - |
| cycles | - | - | - | 55.06k | 594.7 | - | - | - |
| instructions | - | - | - | 37.36k | 839.6 | - | - | - |
| cache references | - | - | - | 124.8 | 3.766 | - | - | - |
| cache misses | - | - | - | 12.88 | 0.00733 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.494 | 0.000121 | - | - | - |
| data TLB read misses | - | - | - | 9.473 | 0.0108 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.53M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.53M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 0.4 | 0.4 | 1.3 | 5.4 | 137.5 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 5.74 / 4.09 |
| best | quiver | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 0.4 | 0.4 | 1.3 | 5.4 | 137.5 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 5.74 / 4.09 |

Run 1's profile:

| profile: the window, 30.0 s, 299500 messages, 299500 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.88G | 199.9k | 199.9k | 99.97k | 99.95k | - | - | - |
| waiting for a CPU, ns | - | - | - | 3.274 | 9.374 | - | - | - |
| allocations | 601.9k | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.00000334 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.00000334 | 0.00000334 | 0 | - | - | - |
| frees | 586.2k | 1.957 | 1.957 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.1 | 483.1 | 483.1 | 0 | - | - | - |
| minor page faults | 1771 | 0.00591 | 0.00591 | 0.00591 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 414.0 | 0.00138 | 0.00138 | 0.000441 | 0.000942 | - | - | - |
| cycles | - | - | - | 516.3k | 515.9k | - | - | - |
| instructions | - | - | - | 296.7k | 2.494M | - | - | - |
| cache references | - | - | - | 21.84 | 15.16 | - | - | - |
| cache misses | - | - | - | 6.251 | 0.318 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.132 | 0.00924 | - | - | - |
| data TLB read misses | - | - | - | 0.0209 | 0.0219 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.88M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.88M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 354590 | 9989 | 0 | 0 | 35459000 | 29949700 | 0 | 0.9 | 1.2 | 3.1 | 15.4 | 939.9 | 0 | 0 | 100/100 | 813.9 | 35.4 | - | 20.50 / 21.75 |
| best | quiver | 22 × busy | 354590 | 9989 | 0 | 0 | 35459000 | 29949700 | 0 | 0.9 | 1.2 | 3.1 | 15.4 | 939.9 | 0 | 0 | 100/100 | 813.9 | 35.4 | - | 20.50 / 21.75 |

Run 1's profile:

| profile: the window, 30.0 s, 299498 messages, 29949800 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.2G | 2.298M | 22.98k | 99.92k | 21.98k | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.662 | 3.940 | - | - | - |
| allocations | 601.9k | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | - | - | - |
| frees | 586.2k | 1.957 | 0.0196 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.1 | 4.831 | 483.1 | 0 | - | - | - |
| minor page faults | 1770 | 0.00591 | 0.0000591 | 0.00591 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 7531 | 0.0251 | 0.000251 | 0.000204 | 0.000249 | - | - | - |
| cycles | - | - | - | 366.8k | 86.77k | - | - | - |
| instructions | - | - | - | 378.4k | 76.57k | - | - | - |
| cache references | - | - | - | 19.65 | 99.65 | - | - | - |
| cache misses | - | - | - | 5.182 | 0.0220 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000621 | 0.000114 | - | - | - |
| data TLB read misses | - | - | - | 0.0390 | 0.0145 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.77M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.77M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 354590 | 9988 | 0 | 0 | 354590000 | 299497000 | 0 | 2.3 | 3.8 | 14.1 | 161.8 | 2050.5 | 0 | 0 | 1000/1000 | 813.0 | 35.4 | - | 29.88 / 26.96 |
| best | quiver | 22 × busy | 354590 | 9988 | 0 | 0 | 354590000 | 299497000 | 0 | 2.3 | 3.8 | 14.1 | 161.8 | 2050.5 | 0 | 0 | 1000/1000 | 813.0 | 35.4 | - | 29.88 / 26.96 |

Run 1's profile:

| profile: the window, 30.0 s, 299498 messages, 299498000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 687.7G | 2.296M | 2296 | 99.87k | 2196 | - | - | - |
| waiting for a CPU, ns | - | - | - | 2.664 | 0.389 | - | - | - |
| allocations | 601.9k | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.000000003 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.000000003 | 0.00000334 | 0 | - | - | - |
| frees | 586.2k | 1.957 | 0.00196 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.1 | 0.483 | 483.1 | 0 | - | - | - |
| minor page faults | 1770 | 0.00591 | 0.00000591 | 0.00591 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 6787 | 0.0227 | 0.0000227 | 0.000270 | 0.0000224 | - | - | - |
| cycles | - | - | - | 362.2k | 7079 | - | - | - |
| instructions | - | - | - | 209.2k | 19.50k | - | - | - |
| cache references | - | - | - | 26.44 | 7.718 | - | - | - |
| cache misses | - | - | - | 6.114 | 0.000909 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.108 | 0.0000137 | - | - | - |
| data TLB read misses | - | - | - | 0.0260 | 0.0127 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.52M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.52M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 0.4 | 0.4 | 1.5 | 7.4 | 95.9 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 31.60 / 17.23 |
| best | quiver | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 0.4 | 0.4 | 1.5 | 7.4 | 95.9 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 31.60 / 17.23 |

Run 1's profile:

| profile: the window, 30.0 s, 2998267 messages, 2998267 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.88G | 19.97k | 19.97k | 9985 | 9988 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.190 | 0.438 | - | - | - |
| allocations | 6.026M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.026M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.448G | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 60.00 | 0.0000200 | 0.0000200 | 0.0000200 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 500.0 | 0.000167 | 0.000167 | 0.000110 | 0.0000567 | - | - | - |
| cycles | - | - | - | 51.28k | 51.29k | - | - | - |
| instructions | - | - | - | 31.37k | 258.2k | - | - | - |
| cache references | - | - | - | 20.07 | 13.47 | - | - | - |
| cache misses | - | - | - | 5.197 | 0.165 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.114 | 0.000598 | - | - | - |
| data TLB read misses | - | - | - | 0.0215 | 0.0360 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.92M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.92M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 3547543 | 99930 | 0 | 0 | 354754300 | 299827400 | 0 | 0.9 | 1.3 | 7.5 | 19.5 | 308.8 | 0 | 0 | 100/100 | 814.3 | 35.4 | - | 21.74 / 22.48 |
| best | quiver | 22 × busy | 3547543 | 99930 | 0 | 0 | 354754300 | 299827400 | 0 | 0.9 | 1.3 | 7.5 | 19.5 | 308.8 | 0 | 0 | 100/100 | 814.3 | 35.4 | - | 21.74 / 22.48 |

Run 1's profile:

| profile: the window, 30.0 s, 2998275 messages, 299827500 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.7G | 229.7k | 2297 | 9985 | 2197 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.586 | 0.160 | - | - | - |
| allocations | 6.026M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.026M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.448G | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 12.00 | 0.00000400 | 0.000000040 | 0.00000400 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 3124 | 0.00104 | 0.0000104 | 0.0000394 | 0.0000100 | - | - | - |
| cycles | - | - | - | 36.55k | 8705 | - | - | - |
| instructions | - | - | - | 35.87k | 7001 | - | - | - |
| cache references | - | - | - | 29.27 | 11.86 | - | - | - |
| cache misses | - | - | - | 5.842 | 0.0164 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00236 | 0.0000286 | - | - | - |
| data TLB read misses | - | - | - | 0.0484 | 0.0151 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.82M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.82M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 3547543 | 99933 | 0 | 0 | 3547543000 | 2998274000 | 0 | 3.1 | 9.2 | 147.5 | 227.3 | 4492.4 | 0 | 0 | 1000/1000 | 810.9 | 35.3 | - | 28.32 / 26.48 |
| best | quiver | 22 × busy | 3547543 | 99933 | 0 | 0 | 3547543000 | 2998274000 | 0 | 3.1 | 9.2 | 147.5 | 227.3 | 4492.4 | 0 | 0 | 1000/1000 | 810.9 | 35.3 | - | 28.32 / 26.48 |

Run 1's profile:

| profile: the window, 30.0 s, 2998278 messages, 2998278000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 687.9G | 229.4k | 229.4 | 9969 | 219.5 | - | - | - |
| waiting for a CPU, ns | - | - | - | 2.762 | 0.0113 | - | - | - |
| allocations | 6.026M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.026M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.448G | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 28.00 | 0.00000934 | 0.000000009 | 0.00000934 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 3117 | 0.00104 | 0.00000104 | 0.000156 | 0.000000884 | - | - | - |
| cycles | - | - | - | 32.19k | 748.0 | - | - | - |
| instructions | - | - | - | 32.34k | 1677 | - | - | - |
| cache references | - | - | - | 18.36 | 2.402 | - | - | - |
| cache misses | - | - | - | 4.054 | 0.00341 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00183 | 0.00000308 | - | - | - |
| data TLB read misses | - | - | - | 0.0504 | 0.00229 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.77M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.77M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 4.3 | 9.1 | 21.5 | 73.7 | 209.8 | 0 | 0 | 1/1 | 36.9 | 35.4 | - | 16.09 / 9.41 |
| best | quiver | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 4.3 | 9.1 | 21.5 | 73.7 | 209.8 | 0 | 0 | 1/1 | 36.9 | 35.4 | - | 16.09 / 9.41 |

Run 1's profile:

| profile: the window, 30.0 s, 299587 messages, 299587 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 31.17G | 104.0k | 104.0k | 99.99k | 4063 | - | - | - |
| waiting for a CPU, ns | - | - | - | 10.64 | 4.306 | - | - | - |
| allocations | 602.1k | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.00000334 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.00000334 | 0.00000334 | 0 | - | - | - |
| frees | 586.3k | 1.957 | 1.957 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 1757 | 0.00586 | 0.00586 | 0.00586 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 283.7k | 0.947 | 0.947 | 0 | 0.947 | - | - | - |
| involuntary context switches | 315.0 | 0.00105 | 0.00105 | 0.00101 | 0.0000401 | - | - | - |
| cycles | - | - | - | 546.3k | 2822 | - | - | - |
| instructions | - | - | - | 310.2k | 1814 | - | - | - |
| cache references | - | - | - | 21.15 | 40.52 | - | - | - |
| cache misses | - | - | - | 6.432 | 0.253 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.129 | 0.00981 | - | - | - |
| data TLB read misses | - | - | - | 0.0240 | 0.362 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.86M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.86M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 10.4 | 20.5 | 40.4 | 88.1 | 2551.3 | 0 | 0 | 100/100 | 95.0 | 35.4 | - | 9.47 / 6.04 |
| best | quiver | 22 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 10.4 | 20.5 | 40.4 | 88.1 | 2551.3 | 0 | 0 | 100/100 | 95.0 | 35.4 | - | 9.47 / 6.04 |

Run 1's profile:

| profile: the window, 30.0 s, 299581 messages, 29958116 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 80.19G | 267.7k | 2677 | 99.80k | 1679 | - | - | - |
| waiting for a CPU, ns | - | - | - | 13.02 | 2.081 | - | - | - |
| allocations | 602.1k | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | - | - | - |
| frees | 586.3k | 1.957 | 0.0196 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 1756 | 0.00586 | 0.0000586 | 0.00586 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 7.988M | 26.66 | 0.267 | 0.00738 | 0.267 | - | - | - |
| involuntary context switches | 1549 | 0.00517 | 0.0000517 | 0.00214 | 0.0000302 | - | - | - |
| cycles | - | - | - | 791.7k | 2351 | - | - | - |
| instructions | - | - | - | 602.2k | 1193 | - | - | - |
| cache references | - | - | - | 203.5 | 11.58 | - | - | - |
| cache misses | - | - | - | 13.17 | 0.0349 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.543 | 0.000985 | - | - | - |
| data TLB read misses | - | - | - | 23.39 | 0.349 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.76M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.76M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 354798 | 9994 | 0 | 0 | 354798000 | 299583000 | 0 | 12.8 | 24.8 | 65.5 | 188.4 | 2013.3 | 0 | 0 | 1000/1000 | 113.0 | 35.4 | - | 11.87 / 9.53 |
| best | quiver | 22 × wait | 354798 | 9994 | 0 | 0 | 354798000 | 299583000 | 0 | 12.8 | 24.8 | 65.5 | 188.4 | 2013.3 | 0 | 0 | 1000/1000 | 113.0 | 35.4 | - | 11.87 / 9.53 |

Run 1's profile:

| profile: the window, 30.0 s, 299581 messages, 299581000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 95.78G | 319.7k | 319.7 | 99.75k | 220.0 | - | - | - |
| waiting for a CPU, ns | - | - | - | 58.13 | 2.792 | - | - | - |
| allocations | 602.1k | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.000000003 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.000000003 | 0.00000334 | 0 | - | - | - |
| frees | 586.3k | 1.957 | 0.00196 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 1758 | 0.00587 | 0.00000587 | 0.00587 | 0.000000003 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 6.669M | 22.26 | 0.0223 | 0.00326 | 0.0223 | - | - | - |
| involuntary context switches | 9949 | 0.0332 | 0.0000332 | 0.00592 | 0.0000273 | - | - | - |
| cycles | - | - | - | 743.3k | 822.0 | - | - | - |
| instructions | - | - | - | 562.6k | 997.9 | - | - | - |
| cache references | - | - | - | 217.5 | 8.122 | - | - | - |
| cache misses | - | - | - | 17.99 | 0.0281 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.659 | 0.000583 | - | - | - |
| data TLB read misses | - | - | - | 33.12 | 0.101 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.60M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.60M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 2.8 | 4.3 | 6.7 | 55.8 | 605.5 | 0 | 0 | 1/1 | 42.0 | 35.4 | - | 4.25 / 3.01 |
| best | quiver | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 2.8 | 4.3 | 6.7 | 55.8 | 605.5 | 0 | 0 | 1/1 | 42.0 | 35.4 | - | 4.25 / 3.01 |

Run 1's profile:

| profile: the window, 30.0 s, 3000434 messages, 3000434 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 35.55G | 11.85k | 11.85k | 9984 | 1863 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.261 | 0.834 | - | - | - |
| allocations | 6.030M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.030M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.449G | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 6.000 | 0.00000200 | 0.00000200 | 0.00000200 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 2.262M | 0.754 | 0.754 | 0 | 0.754 | - | - | - |
| involuntary context switches | 341.0 | 0.000114 | 0.000114 | 0.000110 | 0.00000333 | - | - | - |
| cycles | - | - | - | 54.67k | 2120 | - | - | - |
| instructions | - | - | - | 32.77k | 1398 | - | - | - |
| cache references | - | - | - | 20.60 | 27.12 | - | - | - |
| cache misses | - | - | - | 5.513 | 0.0758 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.108 | 0.00102 | - | - | - |
| data TLB read misses | - | - | - | 0.0210 | 0.220 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 36.09M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 36.09M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 3550038 | 100001 | 0 | 0 | 355003800 | 300043700 | 0 | 13.2 | 37.9 | 77.8 | 131.1 | 1632.4 | 0 | 0 | 100/100 | 326.8 | 34.6 | - | 9.75 / 10.62 |
| best | quiver | 22 × wait | 3550038 | 100001 | 0 | 0 | 355003800 | 300043700 | 0 | 13.2 | 37.9 | 77.8 | 131.1 | 1632.4 | 0 | 0 | 100/100 | 326.8 | 34.6 | - | 9.75 / 10.62 |

Run 1's profile:

| profile: the window, 30.0 s, 3000437 messages, 300043735 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 282.0G | 93.97k | 939.7 | 9752 | 842.2 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.222 | 0.690 | - | - | - |
| allocations | 6.030M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.030M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.449G | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 36.35M | 12.12 | 0.121 | 0.0604 | 0.121 | - | - | - |
| involuntary context switches | 7216 | 0.00240 | 0.0000240 | 0.000188 | 0.0000222 | - | - | - |
| cycles | - | - | - | 53.37k | 1560 | - | - | - |
| instructions | - | - | - | 34.80k | 969.7 | - | - | - |
| cache references | - | - | - | 147.0 | 7.852 | - | - | - |
| cache misses | - | - | - | 12.36 | 0.0254 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.447 | 0.000173 | - | - | - |
| data TLB read misses | - | - | - | 7.591 | 0.0447 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.85M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.85M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 3550038 | 100000 | 0 | 0 | 3550038000 | 3000437000 | 0 | 11.8 | 42.0 | 122.9 | 192.5 | 5368.6 | 0 | 0 | 1000/1000 | 400.2 | 35.3 | - | 17.28 / 16.42 |
| best | quiver | 22 × wait | 3550038 | 100000 | 0 | 0 | 3550038000 | 3000437000 | 0 | 11.8 | 42.0 | 122.9 | 192.5 | 5368.6 | 0 | 0 | 1000/1000 | 400.2 | 35.3 | - | 17.28 / 16.42 |

Run 1's profile:

| profile: the window, 30.0 s, 3000436 messages, 3000436569 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 338.7G | 112.9k | 112.9 | 9936 | 103.0 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.661 | 0.0820 | - | - | - |
| allocations | 6.030M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.030M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.449G | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 35.00 | 0.0000117 | 0.000000012 | 0.0000117 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 24.72M | 8.240 | 0.00824 | 0.00959 | 0.00823 | - | - | - |
| involuntary context switches | 6217 | 0.00207 | 0.00000207 | 0.000228 | 0.00000184 | - | - | - |
| cycles | - | - | - | 61.05k | 595.0 | - | - | - |
| instructions | - | - | - | 41.38k | 857.3 | - | - | - |
| cache references | - | - | - | 132.9 | 3.847 | - | - | - |
| cache misses | - | - | - | 12.20 | 0.00432 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.453 | 0.0000257 | - | - | - |
| data TLB read misses | - | - | - | 10.50 | 0.00459 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.65M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.65M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 0.4 | 0.5 | 0.9 | 2.8 | 181.2 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 7.70 / 5.29 |
| best | quiver | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 0.4 | 0.5 | 0.9 | 2.8 | 181.2 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 7.70 / 5.29 |

Run 1's profile:

| profile: the window, 30.0 s, 299586 messages, 299586 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.88G | 199.9k | 199.9k | 99.95k | 99.93k | - | - | - |
| waiting for a CPU, ns | - | - | - | 6.986 | 10.39 | - | - | - |
| allocations | 602.1k | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.00000334 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.00000334 | 0.00000334 | 0 | - | - | - |
| frees | 586.3k | 1.957 | 1.957 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 1757 | 0.00586 | 0.00586 | 0.00586 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 513.0 | 0.00171 | 0.00171 | 0.000724 | 0.000988 | - | - | - |
| cycles | - | - | - | 501.5k | 501.4k | - | - | - |
| instructions | - | - | - | 288.4k | 2.484M | - | - | - |
| cache references | - | - | - | 19.99 | 14.00 | - | - | - |
| cache misses | - | - | - | 6.015 | 0.218 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.113 | 0.00296 | - | - | - |
| data TLB read misses | - | - | - | 0.0225 | 0.0519 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.94M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.94M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 0.7 | 1.0 | 2.7 | 13.7 | 231.8 | 0 | 0 | 100/100 | 814.7 | 35.4 | - | 15.42 / 18.83 |
| best | quiver | 22 × busy | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 0.7 | 1.0 | 2.7 | 13.7 | 231.8 | 0 | 0 | 100/100 | 814.7 | 35.4 | - | 15.42 / 18.83 |

Run 1's profile:

| profile: the window, 30.0 s, 299581 messages, 29958100 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.9G | 2.300M | 23.00k | 99.96k | 22.00k | - | - | - |
| waiting for a CPU, ns | - | - | - | 8.706 | 0.710 | - | - | - |
| allocations | 602.1k | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | - | - | - |
| frees | 586.3k | 1.957 | 0.0196 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 1757 | 0.00586 | 0.0000586 | 0.00586 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 2240 | 0.00748 | 0.0000748 | 0.000851 | 0.0000663 | - | - | - |
| cycles | - | - | - | 471.8k | 88.22k | - | - | - |
| instructions | - | - | - | 268.0k | 62.12k | - | - | - |
| cache references | - | - | - | 19.39 | 108.5 | - | - | - |
| cache misses | - | - | - | 5.409 | 0.00405 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0965 | 0.0000571 | - | - | - |
| data TLB read misses | - | - | - | 0.0176 | 0.0151 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.90M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.90M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 354798 | 9994 | 0 | 0 | 354798000 | 299583000 | 0 | 2.6 | 4.1 | 13.4 | 157.7 | 426.9 | 0 | 0 | 1000/1000 | 813.6 | 35.4 | - | 30.60 / 27.24 |
| best | quiver | 22 × busy | 354798 | 9994 | 0 | 0 | 354798000 | 299583000 | 0 | 2.6 | 4.1 | 13.4 | 157.7 | 426.9 | 0 | 0 | 1000/1000 | 813.6 | 35.4 | - | 30.60 / 27.24 |

Run 1's profile:

| profile: the window, 30.0 s, 299581 messages, 299581000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.1G | 2.297M | 2297 | 99.82k | 2197 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.619 | 0.103 | - | - | - |
| allocations | 602.1k | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.000000003 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.000000003 | 0.00000334 | 0 | - | - | - |
| frees | 586.3k | 1.957 | 0.00196 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 1757 | 0.00586 | 0.00000586 | 0.00586 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 2758 | 0.00921 | 0.00000921 | 0.000300 | 0.00000891 | - | - | - |
| cycles | - | - | - | 317.5k | 7393 | - | - | - |
| instructions | - | - | - | 320.7k | 20.66k | - | - | - |
| cache references | - | - | - | 21.13 | 8.712 | - | - | - |
| cache misses | - | - | - | 4.217 | 0.00299 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00364 | 0.0000122 | - | - | - |
| data TLB read misses | - | - | - | 0.0405 | 0.0139 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.46M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.46M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × busy | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 0.4 | 0.5 | 0.8 | 6.8 | 188.6 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 26.45 / 15.07 |
| best | quiver | 1 × busy | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 0.4 | 0.5 | 0.8 | 6.8 | 188.6 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 26.45 / 15.07 |

Run 1's profile:

| profile: the window, 30.0 s, 3000438 messages, 3000438 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.88G | 19.96k | 19.96k | 9977 | 9979 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.870 | 1.024 | - | - | - |
| allocations | 6.030M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.030M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.449G | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 60.00 | 0.0000200 | 0.0000200 | 0.0000200 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 432.0 | 0.000144 | 0.000144 | 0.0000800 | 0.0000640 | - | - | - |
| cycles | - | - | - | 85.79k | 88.84k | - | - | - |
| instructions | - | - | - | 68.59k | 408.6k | - | - | - |
| cache references | - | - | - | 44.00 | 22.07 | - | - | - |
| cache misses | - | - | - | 12.23 | 0.407 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.154 | 0.00421 | - | - | - |
| data TLB read misses | - | - | - | 0.0784 | 0.143 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.98M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.98M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 3550038 | 100000 | 0 | 0 | 355003800 | 300043700 | 0 | 1.0 | 1.4 | 6.3 | 16.6 | 1895.3 | 0 | 0 | 100/100 | 813.7 | 35.4 | - | 17.11 / 19.77 |
| best | quiver | 22 × busy | 3550038 | 100000 | 0 | 0 | 355003800 | 300043700 | 0 | 1.0 | 1.4 | 6.3 | 16.6 | 1895.3 | 0 | 0 | 100/100 | 813.7 | 35.4 | - | 17.11 / 19.77 |

Run 1's profile:

| profile: the window, 30.0 s, 3000439 messages, 300043900 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.2G | 229.4k | 2294 | 9970 | 2194 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.184 | 0.149 | - | - | - |
| allocations | 6.030M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.030M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.449G | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 3890 | 0.00130 | 0.0000130 | 0.0000310 | 0.0000127 | - | - | - |
| cycles | - | - | - | 36.31k | 8519 | - | - | - |
| instructions | - | - | - | 38.73k | 6415 | - | - | - |
| cache references | - | - | - | 27.22 | 10.98 | - | - | - |
| cache misses | - | - | - | 5.923 | 0.0145 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00237 | 0.0000129 | - | - | - |
| data TLB read misses | - | - | - | 0.0497 | 0.0152 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.62M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.62M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 3550038 | 100001 | 0 | 0 | 3550038000 | 3000437000 | 0 | 3.0 | 10.1 | 153.6 | 235.5 | 8715.3 | 0 | 0 | 1000/1000 | 812.7 | 35.4 | - | 28.17 / 27.08 |
| best | quiver | 22 × busy | 3550038 | 100001 | 0 | 0 | 3550038000 | 3000437000 | 0 | 3.0 | 10.1 | 153.6 | 235.5 | 8715.3 | 0 | 0 | 1000/1000 | 812.7 | 35.4 | - | 28.17 / 27.08 |

Run 1's profile:

| profile: the window, 30.0 s, 3000437 messages, 3000437889 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 687.8G | 229.2k | 229.2 | 9969 | 219.3 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.236 | 0.0204 | - | - | - |
| allocations | 6.030M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.030M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.449G | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 35.00 | 0.0000117 | 0.000000012 | 0.0000117 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 3522 | 0.00117 | 0.00000117 | 0.0000237 | 0.00000115 | - | - | - |
| cycles | - | - | - | 31.40k | 733.2 | - | - | - |
| instructions | - | - | - | 31.75k | 1597 | - | - | - |
| cache references | - | - | - | 18.53 | 2.248 | - | - | - |
| cache misses | - | - | - | 4.275 | 0.00387 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00224 | 0.0000113 | - | - | - |
| data TLB read misses | - | - | - | 0.0504 | 0.00243 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.71M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.71M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 4.4 | 7.3 | 22.8 | 94.2 | 225.1 | 0 | 0 | 1/1 | 36.9 | 35.4 | - | 33.22 / 19.41 |
| best | quiver | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 4.4 | 7.3 | 22.8 | 94.2 | 225.1 | 0 | 0 | 1/1 | 36.9 | 35.4 | - | 33.22 / 19.41 |

Run 1's profile:

| profile: the window, 30.0 s, 300766 messages, 300766 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 31.13G | 103.5k | 103.5k | 99.53k | 3991 | - | - | - |
| waiting for a CPU, ns | - | - | - | 13.57 | 4.231 | - | - | - |
| allocations | 604.4k | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000332 | 0.00000332 | 0.00000332 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000332 | 0.00000332 | 0.00000332 | 0 | - | - | - |
| frees | 589.0k | 1.958 | 1.958 | 1.958 | 0 | - | - | - |
| bytes allocated | 145.3M | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 1730 | 0.00575 | 0.00575 | 0.00576 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 284.8k | 0.947 | 0.947 | 0 | 0.947 | - | - | - |
| involuntary context switches | 307.0 | 0.00102 | 0.00102 | 0.00101 | 0.0000133 | - | - | - |
| cycles | - | - | - | 516.8k | 2810 | - | - | - |
| instructions | - | - | - | 293.8k | 1797 | - | - | - |
| cache references | - | - | - | 23.66 | 44.01 | - | - | - |
| cache misses | - | - | - | 7.128 | 0.714 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.211 | 0.0408 | - | - | - |
| data TLB read misses | - | - | - | 0.0254 | 0.468 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.83M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.83M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 10.6 | 21.2 | 43.0 | 95.2 | 1999.5 | 0 | 0 | 100/100 | 95.7 | 35.4 | - | 2.81 / 3.36 |
| best | quiver | 22 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 10.6 | 21.2 | 43.0 | 95.2 | 1999.5 | 0 | 0 | 100/100 | 95.7 | 35.4 | - | 2.81 / 3.36 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 30076800 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 80.78G | 268.6k | 2686 | 99.40k | 1692 | - | - | - |
| waiting for a CPU, ns | - | - | - | 13.23 | 2.615 | - | - | - |
| allocations | 604.4k | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000332 | 0.000000033 | 0.00000332 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000332 | 0.000000033 | 0.00000332 | 0 | - | - | - |
| frees | 589.0k | 1.958 | 0.0196 | 1.958 | 0 | - | - | - |
| bytes allocated | 145.3M | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 1729 | 0.00575 | 0.0000575 | 0.00575 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 7.993M | 26.57 | 0.266 | 0.00773 | 0.266 | - | - | - |
| involuntary context switches | 1876 | 0.00624 | 0.0000624 | 0.00294 | 0.0000330 | - | - | - |
| cycles | - | - | - | 772.7k | 2356 | - | - | - |
| instructions | - | - | - | 587.3k | 1198 | - | - | - |
| cache references | - | - | - | 201.9 | 11.65 | - | - | - |
| cache misses | - | - | - | 13.04 | 0.0341 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.540 | 0.000730 | - | - | - |
| data TLB read misses | - | - | - | 19.34 | 0.324 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.64M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.64M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 356101 | 10031 | 0 | 0 | 356101000 | 300769000 | 0 | 11.6 | 19.2 | 49.2 | 137.2 | 3034.1 | 0 | 0 | 1000/1000 | 110.2 | 35.4 | - | 11.31 / 9.38 |
| best | quiver | 22 × wait | 356101 | 10031 | 0 | 0 | 356101000 | 300769000 | 0 | 11.6 | 19.2 | 49.2 | 137.2 | 3034.1 | 0 | 0 | 1000/1000 | 110.2 | 35.4 | - | 11.31 / 9.38 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 300768000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 93.20G | 309.9k | 309.9 | 99.43k | 210.4 | - | - | - |
| waiting for a CPU, ns | - | - | - | 11.72 | 0.417 | - | - | - |
| allocations | 604.4k | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000332 | 0.000000003 | 0.00000332 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000332 | 0.000000003 | 0.00000332 | 0 | - | - | - |
| frees | 589.0k | 1.958 | 0.00196 | 1.958 | 0 | - | - | - |
| bytes allocated | 145.3M | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 1730 | 0.00575 | 0.00000575 | 0.00576 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 6.285M | 20.90 | 0.0209 | 0.00409 | 0.0209 | - | - | - |
| involuntary context switches | 2204 | 0.00733 | 0.00000733 | 0.00215 | 0.00000518 | - | - | - |
| cycles | - | - | - | 781.6k | 826.6 | - | - | - |
| instructions | - | - | - | 590.7k | 994.8 | - | - | - |
| cache references | - | - | - | 213.8 | 7.947 | - | - | - |
| cache misses | - | - | - | 17.19 | 0.0144 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.607 | 0.000368 | - | - | - |
| data TLB read misses | - | - | - | 34.92 | 0.145 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.53M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.53M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × wait | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 3.9 | 4.6 | 9.0 | 88.1 | 1458.1 | 0 | 0 | 1/1 | 43.2 | 35.4 | - | 12.57 / 7.55 |
| best | quiver | 1 × wait | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 3.9 | 4.6 | 9.0 | 88.1 | 1458.1 | 0 | 0 | 1/1 | 43.2 | 35.4 | - | 12.57 / 7.55 |

Run 1's profile:

| profile: the window, 30.0 s, 3002646 messages, 3002647 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 36.45G | 12.14k | 12.14k | 9974 | 2165 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.587 | 3.178 | - | - | - |
| allocations | 6.034M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.034M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.450G | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 6.000 | 0.00000200 | 0.00000200 | 0.00000200 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 2.136M | 0.711 | 0.711 | 0.000000666 | 0.711 | - | - | - |
| involuntary context switches | 359.0 | 0.000120 | 0.000120 | 0.000111 | 0.00000866 | - | - | - |
| cycles | - | - | - | 53.74k | 2468 | - | - | - |
| instructions | - | - | - | 32.41k | 1596 | - | - | - |
| cache references | - | - | - | 18.68 | 31.45 | - | - | - |
| cache misses | - | - | - | 5.740 | 0.173 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.144 | 0.00419 | - | - | - |
| data TLB read misses | - | - | - | 0.0207 | 0.288 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.89M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.89M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 3554018 | 100115 | 0 | 0 | 355401800 | 300263100 | 0 | 13.8 | 39.4 | 79.9 | 135.2 | 2029.4 | 0 | 0 | 100/100 | 328.3 | 34.7 | - | 4.07 / 7.43 |
| best | quiver | 22 × wait | 3554018 | 100115 | 0 | 0 | 355401800 | 300263100 | 0 | 13.8 | 39.4 | 79.9 | 135.2 | 2029.4 | 0 | 0 | 100/100 | 328.3 | 34.7 | - | 4.07 / 7.43 |

Run 1's profile:

| profile: the window, 30.0 s, 3002637 messages, 300263767 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 280.1G | 93.29k | 932.9 | 9769 | 835.2 | - | - | - |
| waiting for a CPU, ns | - | - | - | 2.497 | 1.519 | - | - | - |
| allocations | 6.034M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.034M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.450G | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 35.79M | 11.92 | 0.119 | 0.0510 | 0.119 | - | - | - |
| involuntary context switches | 10.30k | 0.00343 | 0.0000343 | 0.000215 | 0.0000322 | - | - | - |
| cycles | - | - | - | 52.47k | 1519 | - | - | - |
| instructions | - | - | - | 34.01k | 958.9 | - | - | - |
| cache references | - | - | - | 141.7 | 7.906 | - | - | - |
| cache misses | - | - | - | 12.41 | 0.0293 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.491 | 0.000195 | - | - | - |
| data TLB read misses | - | - | - | 7.428 | 0.0511 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.88M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.88M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 3554018 | 100113 | 0 | 0 | 3554018000 | 3002631000 | 0 | 12.3 | 46.1 | 139.3 | 237.6 | 4559.9 | 0 | 0 | 1000/1000 | 404.8 | 35.3 | - | 13.52 / 13.05 |
| best | quiver | 22 × wait | 3554018 | 100113 | 0 | 0 | 3554018000 | 3002631000 | 0 | 12.3 | 46.1 | 139.3 | 237.6 | 4559.9 | 0 | 0 | 1000/1000 | 404.8 | 35.3 | - | 13.52 / 13.05 |

Run 1's profile:

| profile: the window, 30.0 s, 3002637 messages, 3002637134 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 344.1G | 114.6k | 114.6 | 9928 | 104.7 | - | - | - |
| waiting for a CPU, ns | - | - | - | 2.719 | 0.665 | - | - | - |
| allocations | 6.034M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.034M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.450G | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 23.63M | 7.869 | 0.00787 | 0.00796 | 0.00786 | - | - | - |
| involuntary context switches | 18.64k | 0.00621 | 0.00000621 | 0.000334 | 0.00000588 | - | - | - |
| cycles | - | - | - | 58.87k | 592.2 | - | - | - |
| instructions | - | - | - | 40.04k | 848.4 | - | - | - |
| cache references | - | - | - | 130.2 | 3.804 | - | - | - |
| cache misses | - | - | - | 12.62 | 0.00638 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.463 | 0.000122 | - | - | - |
| data TLB read misses | - | - | - | 9.967 | 0.00571 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.71M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.71M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 0.4 | 0.5 | 1.0 | 2.9 | 389.5 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 18.16 / 11.37 |
| best | quiver | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 0.4 | 0.5 | 1.0 | 2.9 | 389.5 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 18.16 / 11.37 |

Run 1's profile:

| profile: the window, 30.0 s, 300766 messages, 300766 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.87G | 199.1k | 199.1k | 99.53k | 99.54k | - | - | - |
| waiting for a CPU, ns | - | - | - | 9.557 | 5.749 | - | - | - |
| allocations | 604.4k | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000332 | 0.00000332 | 0.00000332 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000332 | 0.00000332 | 0.00000332 | 0 | - | - | - |
| frees | 589.0k | 1.958 | 1.958 | 1.958 | 0 | - | - | - |
| bytes allocated | 145.3M | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 1730 | 0.00575 | 0.00575 | 0.00576 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 382.0 | 0.00127 | 0.00127 | 0.000592 | 0.000678 | - | - | - |
| cycles | - | - | - | 837.6k | 850.3k | - | - | - |
| instructions | - | - | - | 639.7k | 3.842M | - | - | - |
| cache references | - | - | - | 298.5 | 52.42 | - | - | - |
| cache misses | - | - | - | 124.5 | 10.04 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.167 | 0.0161 | - | - | - |
| data TLB read misses | - | - | - | 9.042 | 2.013 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.93M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.93M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 0.7 | 1.0 | 2.5 | 13.1 | 1732.1 | 0 | 0 | 100/100 | 814.5 | 35.4 | - | 3.15 / 12.16 |
| best | quiver | 22 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 0.7 | 1.0 | 2.5 | 13.1 | 1732.1 | 0 | 0 | 100/100 | 814.5 | 35.4 | - | 3.15 / 12.16 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 30076800 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.7G | 2.290M | 22.90k | 99.59k | 21.90k | - | - | - |
| waiting for a CPU, ns | - | - | - | 2.220 | 1.019 | - | - | - |
| allocations | 604.4k | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000332 | 0.000000033 | 0.00000332 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000332 | 0.000000033 | 0.00000332 | 0 | - | - | - |
| frees | 589.0k | 1.958 | 0.0196 | 1.958 | 0 | - | - | - |
| bytes allocated | 145.3M | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 1729 | 0.00575 | 0.0000575 | 0.00575 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 2591 | 0.00861 | 0.0000861 | 0.000216 | 0.0000840 | - | - | - |
| cycles | - | - | - | 465.2k | 86.92k | - | - | - |
| instructions | - | - | - | 268.2k | 69.91k | - | - | - |
| cache references | - | - | - | 25.03 | 111.9 | - | - | - |
| cache misses | - | - | - | 6.365 | 0.0127 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.131 | 0.0000640 | - | - | - |
| data TLB read misses | - | - | - | 0.0513 | 0.0151 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.63M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.63M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 356101 | 10031 | 0 | 0 | 356101000 | 300769000 | 0 | 2.7 | 4.4 | 16.4 | 167.9 | 1634.4 | 0 | 0 | 1000/1000 | 813.6 | 35.4 | - | 30.29 / 27.49 |
| best | quiver | 22 × busy | 356101 | 10031 | 0 | 0 | 356101000 | 300769000 | 0 | 2.7 | 4.4 | 16.4 | 167.9 | 1634.4 | 0 | 0 | 1000/1000 | 813.6 | 35.4 | - | 30.29 / 27.49 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 300768000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 687.9G | 2.287M | 2287 | 99.47k | 2188 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.630 | 0.145 | - | - | - |
| allocations | 604.4k | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000332 | 0.000000003 | 0.00000332 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000332 | 0.000000003 | 0.00000332 | 0 | - | - | - |
| frees | 589.0k | 1.958 | 0.00196 | 1.958 | 0 | - | - | - |
| bytes allocated | 145.3M | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 1730 | 0.00575 | 0.00000575 | 0.00576 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 3210 | 0.0107 | 0.0000107 | 0.000216 | 0.0000105 | - | - | - |
| cycles | - | - | - | 299.9k | 7047 | - | - | - |
| instructions | - | - | - | 311.9k | 19.29k | - | - | - |
| cache references | - | - | - | 21.48 | 8.268 | - | - | - |
| cache misses | - | - | - | 4.309 | 0.00405 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00341 | 0.0000488 | - | - | - |
| data TLB read misses | - | - | - | 0.0402 | 0.0128 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.50M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.50M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × busy | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 0.4 | 0.5 | 0.9 | 7.2 | 405.1 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 48.62 / 28.93 |
| best | quiver | 1 × busy | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 0.4 | 0.5 | 0.9 | 7.2 | 405.1 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 48.62 / 28.93 |

Run 1's profile:

| profile: the window, 30.0 s, 3002653 messages, 3002653 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.87G | 19.94k | 19.94k | 9968 | 9969 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.476 | 0.924 | - | - | - |
| allocations | 6.034M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.034M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.450G | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 60.00 | 0.0000200 | 0.0000200 | 0.0000200 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 471.0 | 0.000157 | 0.000157 | 0.0000706 | 0.0000863 | - | - | - |
| cycles | - | - | - | 82.03k | 83.31k | - | - | - |
| instructions | - | - | - | 65.48k | 380.2k | - | - | - |
| cache references | - | - | - | 43.66 | 22.19 | - | - | - |
| cache misses | - | - | - | 13.22 | 0.433 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.171 | 0.00671 | - | - | - |
| data TLB read misses | - | - | - | 0.118 | 0.0830 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.91M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.91M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 3554018 | 100113 | 0 | 0 | 355401800 | 300263100 | 0 | 0.9 | 1.3 | 6.0 | 15.6 | 175.5 | 0 | 0 | 100/100 | 814.7 | 35.4 | - | 4.64 / 12.83 |
| best | quiver | 22 × busy | 3554018 | 100113 | 0 | 0 | 355401800 | 300263100 | 0 | 0.9 | 1.3 | 6.0 | 15.6 | 175.5 | 0 | 0 | 100/100 | 814.7 | 35.4 | - | 4.64 / 12.83 |

Run 1's profile:

| profile: the window, 30.0 s, 3002637 messages, 300263700 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.9G | 229.4k | 2294 | 9974 | 2194 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.216 | 0.101 | - | - | - |
| allocations | 6.034M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.034M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.450G | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 2548 | 0.000849 | 0.00000849 | 0.0000157 | 0.00000833 | - | - | - |
| cycles | - | - | - | 37.49k | 8934 | - | - | - |
| instructions | - | - | - | 40.06k | 7130 | - | - | - |
| cache references | - | - | - | 26.35 | 11.63 | - | - | - |
| cache misses | - | - | - | 5.936 | 0.0149 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00279 | 0.0000163 | - | - | - |
| data TLB read misses | - | - | - | 0.0493 | 0.0153 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.86M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.86M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 3554018 | 100114 | 0 | 0 | 3554018000 | 3002631000 | 0 | 2.9 | 8.2 | 139.3 | 221.2 | 496.5 | 0 | 0 | 1000/1000 | 813.7 | 35.4 | - | 27.44 / 26.03 |
| best | quiver | 22 × busy | 3554018 | 100114 | 0 | 0 | 3554018000 | 3002631000 | 0 | 2.9 | 8.2 | 139.3 | 221.2 | 496.5 | 0 | 0 | 1000/1000 | 813.7 | 35.4 | - | 27.44 / 26.03 |

Run 1's profile:

| profile: the window, 30.0 s, 3002632 messages, 3002632000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.0G | 229.1k | 229.1 | 9957 | 219.2 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.272 | 0.0122 | - | - | - |
| allocations | 6.034M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.034M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.450G | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 23.00 | 0.00000766 | 0.000000008 | 0.00000766 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 3655 | 0.00122 | 0.00000122 | 0.0000323 | 0.00000118 | - | - | - |
| cycles | - | - | - | 32.96k | 766.8 | - | - | - |
| instructions | - | - | - | 34.53k | 1683 | - | - | - |
| cache references | - | - | - | 18.47 | 2.461 | - | - | - |
| cache misses | - | - | - | 4.027 | 0.00339 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00223 | 0.00000408 | - | - | - |
| data TLB read misses | - | - | - | 0.0497 | 0.00253 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.55M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.55M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
