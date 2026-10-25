# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 4.0 | 7.1 | 30.0 | 103.4 | 1846.5 | 0 | 0 | 1/1 | 36.9 | 35.4 | - | 2.62 / 2.23 |
| best | wake | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 4.0 | 7.1 | 30.0 | 103.4 | 1846.5 | 0 | 0 | 1/1 | 36.9 | 35.4 | - | 2.62 / 2.23 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 299499 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 31.13G | 103.9k | 103.9k | 100.0k | 3947 | - | - | - |
| waiting for a CPU, ns | - | - | - | 12.57 | 24.71 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000104 | 0.000104 | 0.000104 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 283.3k | 0.946 | 0.946 | 0 | 0.946 | - | - | - |
| involuntary context switches | 337.0 | 0.00113 | 0.00113 | 0.00108 | 0.0000501 | - | - | - |
| cycles | - | - | - | 961.9k | 1711 | - | - | - |
| instructions | - | - | - | 746.3k | 718.4 | - | - | - |
| cache references | - | - | - | 12.73 | 15.73 | - | - | - |
| cache misses | - | - | - | 0.966 | 0.634 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0126 | 0.0326 | - | - | - |
| data TLB read misses | - | - | - | 0.0411 | 0.289 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.119M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.119M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 354590 | 9988 | 0 | 0 | 7800980 | 6588934 | 0 | 8.8 | 20.7 | 32.5 | 45.1 | 1243.8 | 0 | 0 | 22/22 | 75.9 | 35.4 | - | 2.61 / 2.71 |
| best | wake | 22 × wait | 354590 | 9988 | 0 | 0 | 7800980 | 6588934 | 0 | 8.8 | 20.7 | 32.5 | 45.1 | 1243.8 | 0 | 0 | 22/22 | 75.9 | 35.4 | - | 2.61 / 2.71 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 6588956 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 64.24G | 214.5k | 9749 | 99.86k | 5210 | - | - | - |
| waiting for a CPU, ns | - | - | - | 6.123 | 11.44 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000104 | 0.00000470 | 0.000104 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 6.438M | 21.50 | 0.977 | 0 | 0.977 | - | - | - |
| involuntary context switches | 746.0 | 0.00249 | 0.000113 | 0.000925 | 0.0000712 | - | - | - |
| cycles | - | - | - | 785.5k | 1813 | - | - | - |
| instructions | - | - | - | 600.8k | 661.8 | - | - | - |
| cache references | - | - | - | 43.92 | 9.406 | - | - | - |
| cache misses | - | - | - | 1.046 | 0.132 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00419 | 0.00669 | - | - | - |
| data TLB read misses | - | - | - | 0.112 | 0.733 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.660M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.660M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 354590 | 9988 | 0 | 0 | 7800980 | 6588934 | 0 | 9.1 | 21.0 | 32.8 | 52.2 | 3136.1 | 0 | 0 | 22/22 | 76.3 | 35.4 | - | 6.73 / 4.65 |
| best | wake | 22 × wait | 354590 | 9988 | 0 | 0 | 7800980 | 6588934 | 0 | 9.1 | 21.0 | 32.8 | 52.2 | 3136.1 | 0 | 0 | 22/22 | 76.3 | 35.4 | - | 6.73 / 4.65 |

Run 1's profile:

| profile: the window, 30.0 s, 299498 messages, 6588956 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 64.43G | 215.1k | 9778 | 99.85k | 5240 | - | - | - |
| waiting for a CPU, ns | - | - | - | 15.13 | 12.88 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000104 | 0.00000470 | 0.000104 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 6.436M | 21.49 | 0.977 | 0 | 0.977 | - | - | - |
| involuntary context switches | 1037 | 0.00346 | 0.000157 | 0.00172 | 0.0000792 | - | - | - |
| cycles | - | - | - | 776.2k | 1731 | - | - | - |
| instructions | - | - | - | 596.7k | 664.1 | - | - | - |
| cache references | - | - | - | 38.47 | 9.714 | - | - | - |
| cache misses | - | - | - | 0.577 | 0.144 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00469 | 0.00664 | - | - | - |
| data TLB read misses | - | - | - | 0.0730 | 0.578 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.737M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.737M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 3.8 | 4.4 | 7.2 | 106.5 | 924.0 | 0 | 0 | 1/1 | 43.0 | 35.4 | - | 10.78 / 6.61 |
| best | wake | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 3.8 | 4.4 | 7.2 | 106.5 | 924.0 | 0 | 0 | 1/1 | 43.0 | 35.4 | - | 10.78 / 6.61 |

Run 1's profile:

| profile: the window, 30.0 s, 2998288 messages, 2998288 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 36.30G | 12.11k | 12.11k | 9988 | 2118 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.473 | 4.643 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 2.143M | 0.715 | 0.715 | 0 | 0.715 | - | - | - |
| involuntary context switches | 261.0 | 0.0000870 | 0.0000870 | 0.0000650 | 0.0000220 | - | - | - |
| cycles | - | - | - | 87.01k | 1327 | - | - | - |
| instructions | - | - | - | 65.53k | 672.6 | - | - | - |
| cache references | - | - | - | 6.434 | 6.301 | - | - | - |
| cache misses | - | - | - | 0.325 | 0.143 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00200 | 0.00588 | - | - | - |
| data TLB read misses | - | - | - | 0.00374 | 0.0329 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 5.984M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 5.984M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 2944281 | 82900 | 603262 | 0 | 64774182 | 53788020 | 12174008 | 5167382.5 | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 22/22 | 312.3 | 35.0 | - | 4.37 / 9.79 |
| best | wake | 22 × wait | 2944281 | 82900 | 603262 | 0 | 64774182 | 53788020 | 12174008 | 5167382.5 | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 22/22 | 312.3 | 35.0 | - | 4.37 / 9.79 |

Run 1's profile:

| profile: the window, 30.0 s, 2495110 messages, 54892421 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 260.7G | 104.5k | 4750 | 11.83k | 4212 | - | - | - |
| waiting for a CPU, ns | - | - | - | 155.7 | 373.1 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 51.46M | 20.63 | 0.938 | 0 | 0.938 | - | - | - |
| involuntary context switches | 45.62k | 0.0183 | 0.000831 | 0.0158 | 0.000112 | - | - | - |
| cycles | - | - | - | 9400 | 1878 | - | - | - |
| instructions | - | - | - | 1972 | 689.1 | - | - | - |
| cache references | - | - | - | 44.19 | 15.06 | - | - | - |
| cache misses | - | - | - | 0.301 | 0.132 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00240 | 0.00483 | - | - | - |
| data TLB read misses | - | - | - | 0.00198 | 0.396 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.672M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.672M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 3544787 | 99854 | 2756 | 0 | 77985314 | 65962028 | 0 | 142606.3 | 306184.2 | 406847.5 | 415236.1 | 416303.7 | 0 | 0 | 22/22 | 358.8 | 35.4 | - | 6.02 / 8.14 |
| best | wake | 22 × wait | 3544787 | 99854 | 2756 | 0 | 77985314 | 65962028 | 0 | 142606.3 | 306184.2 | 406847.5 | 415236.1 | 416303.7 | 0 | 0 | 22/22 | 358.8 | 35.4 | - | 6.02 / 8.14 |

Run 1's profile:

| profile: the window, 30.0 s, 3000951 messages, 66020929 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 306.4G | 102.1k | 4641 | 9978 | 4188 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.610 | 8.722 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 65.96M | 21.98 | 0.999 | 0 | 0.999 | - | - | - |
| involuntary context switches | 3115 | 0.00104 | 0.0000472 | 0.0000620 | 0.0000444 | - | - | - |
| cycles | - | - | - | 10.06k | 1846 | - | - | - |
| instructions | - | - | - | 2290 | 700.1 | - | - | - |
| cache references | - | - | - | 44.59 | 14.86 | - | - | - |
| cache misses | - | - | - | 0.0865 | 0.0394 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000393 | 0.000632 | - | - | - |
| data TLB read misses | - | - | - | 0.000948 | 0.313 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.524M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.524M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 0.1 | 0.2 | 0.6 | 2.6 | 161.1 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 11.99 / 7.57 |
| best | wake | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 0.1 | 0.2 | 0.6 | 2.6 | 161.1 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 11.99 / 7.57 |

Run 1's profile:

| profile: the window, 30.0 s, 299500 messages, 299500 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.91G | 200.0k | 200.0k | 100.0k | 100.0k | - | - | - |
| waiting for a CPU, ns | - | - | - | 8.511 | 8.111 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000104 | 0.000104 | 0.000104 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 521.0 | 0.00174 | 0.00174 | 0.000865 | 0.000875 | - | - | - |
| cycles | - | - | - | 940.5k | 935.9k | - | - | - |
| instructions | - | - | - | 727.1k | 226.8k | - | - | - |
| cache references | - | - | - | 9.538 | 8.417 | - | - | - |
| cache misses | - | - | - | 0.854 | 0.685 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00544 | 0.00605 | - | - | - |
| data TLB read misses | - | - | - | 0.0396 | 0.0534 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.001M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.001M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 354590 | 9988 | 0 | 0 | 7800980 | 6588934 | 0 | 0.2 | 0.3 | 0.7 | 2.7 | 342.3 | 0 | 0 | 22/22 | 814.5 | 35.4 | - | 3.21 / 12.16 |
| best | wake | 22 × busy | 354590 | 9988 | 0 | 0 | 7800980 | 6588934 | 0 | 0.2 | 0.3 | 0.7 | 2.7 | 342.3 | 0 | 0 | 22/22 | 814.5 | 35.4 | - | 3.21 / 12.16 |

Run 1's profile:

| profile: the window, 30.0 s, 299510 messages, 6589220 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.8G | 2.300M | 104.5k | 99.98k | 99.99k | - | - | - |
| waiting for a CPU, ns | - | - | - | 6.285 | 16.82 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000104 | 0.00000470 | 0.000104 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 7694 | 0.0257 | 0.00117 | 0.000831 | 0.00113 | - | - | - |
| cycles | - | - | - | 472.1k | 401.0k | - | - | - |
| instructions | - | - | - | 251.1k | 121.5k | - | - | - |
| cache references | - | - | - | 3.059 | 374.7 | - | - | - |
| cache misses | - | - | - | 0.0309 | 0.0146 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00525 | 0.000340 | - | - | - |
| data TLB read misses | - | - | - | 0.0000267 | 0.00118 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.672M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.672M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 354590 | 9989 | 0 | 0 | 7800980 | 6588934 | 0 | 0.3 | 0.3 | 0.6 | 2.7 | 211.8 | 0 | 0 | 22/22 | 813.9 | 35.4 | - | 34.32 / 29.50 |
| best | wake | 22 × busy | 354590 | 9989 | 0 | 0 | 7800980 | 6588934 | 0 | 0.3 | 0.3 | 0.6 | 2.7 | 211.8 | 0 | 0 | 22/22 | 813.9 | 35.4 | - | 34.32 / 29.50 |

Run 1's profile:

| profile: the window, 30.0 s, 299498 messages, 6588956 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.2G | 2.298M | 104.4k | 99.95k | 99.90k | - | - | - |
| waiting for a CPU, ns | - | - | - | 4.743 | 20.68 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000104 | 0.00000470 | 0.000104 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 9241 | 0.0309 | 0.00140 | 0.000341 | 0.00139 | - | - | - |
| cycles | - | - | - | 367.2k | 394.3k | - | - | - |
| instructions | - | - | - | 384.8k | 115.1k | - | - | - |
| cache references | - | - | - | 3.125 | 370.4 | - | - | - |
| cache misses | - | - | - | 0.0131 | 0.0156 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.000375 | - | - | - |
| data TLB read misses | - | - | - | 0 | 0.000540 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.651M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.651M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 0.1 | 0.2 | 0.4 | 2.4 | 139.0 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 95.62 / 54.31 |
| best | wake | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 0.1 | 0.2 | 0.4 | 2.4 | 139.0 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 95.62 / 54.31 |

Run 1's profile:

| profile: the window, 30.0 s, 2998282 messages, 2998282 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.92G | 19.98k | 19.98k | 9991 | 9992 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.568 | 0.794 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 492.0 | 0.000164 | 0.000164 | 0.0000817 | 0.0000824 | - | - | - |
| cycles | - | - | - | 54.88k | 95.67k | - | - | - |
| instructions | - | - | - | 31.70k | 23.77k | - | - | - |
| cache references | - | - | - | 3.194 | 4.797 | - | - | - |
| cache misses | - | - | - | 0.00679 | 0.228 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000426 | 0.000466 | - | - | - |
| data TLB read misses | - | - | - | 0.0000367 | 0.00976 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.193M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.193M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 3547543 | 99930 | 0 | 0 | 78045946 | 65962028 | 0 | 0.3 | 0.3 | 0.6 | 2.6 | 4216.2 | 0 | 0 | 22/22 | 814.8 | 35.4 | - | 6.95 / 14.29 |
| best | wake | 22 × busy | 3547543 | 99930 | 0 | 0 | 78045946 | 65962028 | 0 | 0.3 | 0.3 | 0.6 | 2.6 | 4216.2 | 0 | 0 | 22/22 | 814.8 | 35.4 | - | 6.95 / 14.29 |

Run 1's profile:

| profile: the window, 30.0 s, 2998278 messages, 65962116 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.9G | 229.8k | 10.44k | 9990 | 9989 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.0953 | 0.560 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 2625 | 0.000876 | 0.0000398 | 0.0000133 | 0.0000392 | - | - | - |
| cycles | - | - | - | 36.81k | 39.86k | - | - | - |
| instructions | - | - | - | 38.59k | 11.58k | - | - | - |
| cache references | - | - | - | 2.135 | 38.02 | - | - | - |
| cache misses | - | - | - | 0.000914 | 0.00542 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.0000211 | - | - | - |
| data TLB read misses | - | - | - | 0.00000367 | 0.0000597 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.672M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.672M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 3547543 | 99930 | 0 | 0 | 78045946 | 65962028 | 0 | 0.3 | 0.3 | 0.6 | 2.7 | 3203.1 | 0 | 0 | 22/22 | 814.0 | 35.4 | - | 25.93 / 24.57 |
| best | wake | 22 × busy | 3547543 | 99930 | 0 | 0 | 78045946 | 65962028 | 0 | 0.3 | 0.3 | 0.6 | 2.7 | 3203.1 | 0 | 0 | 22/22 | 814.0 | 35.4 | - | 25.93 / 24.57 |

Run 1's profile:

| profile: the window, 30.0 s, 2998278 messages, 65962116 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.4G | 229.6k | 10.44k | 9979 | 9982 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.087 | 0.483 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 3104 | 0.00104 | 0.0000471 | 0.0000604 | 0.0000443 | - | - | - |
| cycles | - | - | - | 36.17k | 38.85k | - | - | - |
| instructions | - | - | - | 37.48k | 11.32k | - | - | - |
| cache references | - | - | - | 2.167 | 36.83 | - | - | - |
| cache misses | - | - | - | 0.00103 | 0.00546 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.0000331 | - | - | - |
| data TLB read misses | - | - | - | 0.0000177 | 0.0000803 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.729M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.729M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 4.0 | 8.1 | 22.5 | 87.0 | 241.3 | 0 | 0 | 1/1 | 36.9 | 35.4 | - | 3.27 / 2.33 |
| best | wake | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 4.0 | 8.1 | 22.5 | 87.0 | 241.3 | 0 | 0 | 1/1 | 36.9 | 35.4 | - | 3.27 / 2.33 |

Run 1's profile:

| profile: the window, 30.0 s, 299580 messages, 299580 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 31.15G | 104.0k | 104.0k | 99.98k | 4001 | - | - | - |
| waiting for a CPU, ns | - | - | - | 11.69 | 5.514 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000103 | 0.000103 | 0.000103 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 283.7k | 0.947 | 0.947 | 0 | 0.947 | - | - | - |
| involuntary context switches | 326.0 | 0.00109 | 0.00109 | 0.00105 | 0.0000367 | - | - | - |
| cycles | - | - | - | 549.6k | 1689 | - | - | - |
| instructions | - | - | - | 314.1k | 729.6 | - | - | - |
| cache references | - | - | - | 5.352 | 14.58 | - | - | - |
| cache misses | - | - | - | 0.0472 | 0.258 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00410 | 0.00593 | - | - | - |
| data TLB read misses | - | - | - | 0.000935 | 0.189 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.103M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.103M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 354798 | 9994 | 0 | 0 | 7805556 | 6590826 | 0 | 8.8 | 20.7 | 32.5 | 49.7 | 350.9 | 0 | 0 | 22/22 | 75.9 | 35.4 | - | 6.17 / 4.06 |
| best | wake | 22 × wait | 354798 | 9994 | 0 | 0 | 7805556 | 6590826 | 0 | 8.8 | 20.7 | 32.5 | 49.7 | 350.9 | 0 | 0 | 22/22 | 75.9 | 35.4 | - | 6.17 / 4.06 |

Run 1's profile:

| profile: the window, 30.0 s, 299581 messages, 6590784 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 64.08G | 213.9k | 9723 | 99.83k | 5186 | - | - | - |
| waiting for a CPU, ns | - | - | - | 6.409 | 9.172 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000103 | 0.00000470 | 0.000103 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 6.440M | 21.50 | 0.977 | 0 | 0.977 | - | - | - |
| involuntary context switches | 606.0 | 0.00202 | 0.0000919 | 0.00110 | 0.0000419 | - | - | - |
| cycles | - | - | - | 787.2k | 1744 | - | - | - |
| instructions | - | - | - | 604.8k | 661.9 | - | - | - |
| cache references | - | - | - | 39.58 | 9.057 | - | - | - |
| cache misses | - | - | - | 0.758 | 0.0885 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00326 | 0.00404 | - | - | - |
| data TLB read misses | - | - | - | 0.0856 | 0.804 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.668M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.668M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 354798 | 9994 | 0 | 0 | 7805556 | 6590826 | 0 | 9.2 | 20.5 | 33.8 | 94.2 | 1861.8 | 0 | 0 | 22/22 | 75.2 | 35.4 | - | 9.57 / 6.28 |
| best | wake | 22 × wait | 354798 | 9994 | 0 | 0 | 7805556 | 6590826 | 0 | 9.2 | 20.5 | 33.8 | 94.2 | 1861.8 | 0 | 0 | 22/22 | 75.2 | 35.4 | - | 9.57 / 6.28 |

Run 1's profile:

| profile: the window, 30.0 s, 299581 messages, 6590782 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 63.68G | 212.6k | 9662 | 99.79k | 5126 | - | - | - |
| waiting for a CPU, ns | - | - | - | 22.83 | 11.86 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000103 | 0.00000470 | 0.000103 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 6.452M | 21.54 | 0.979 | 0 | 0.979 | - | - | - |
| involuntary context switches | 1265 | 0.00422 | 0.000192 | 0.00240 | 0.0000830 | - | - | - |
| cycles | - | - | - | 769.2k | 1736 | - | - | - |
| instructions | - | - | - | 589.5k | 668.8 | - | - | - |
| cache references | - | - | - | 50.81 | 11.03 | - | - | - |
| cache misses | - | - | - | 0.508 | 0.170 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00604 | 0.00809 | - | - | - |
| data TLB read misses | - | - | - | 0.0615 | 1.255 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.786M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.786M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 3.6 | 4.1 | 5.3 | 56.3 | 608.0 | 0 | 0 | 1/1 | 42.7 | 35.4 | - | 2.08 / 1.65 |
| best | wake | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 3.6 | 4.1 | 5.3 | 56.3 | 608.0 | 0 | 0 | 1/1 | 42.7 | 35.4 | - | 2.08 / 1.65 |

Run 1's profile:

| profile: the window, 30.0 s, 3000433 messages, 3000433 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 36.08G | 12.02k | 12.02k | 9983 | 2041 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.554 | 0.966 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 2.182M | 0.727 | 0.727 | 0 | 0.727 | - | - | - |
| involuntary context switches | 426.0 | 0.000142 | 0.000142 | 0.000139 | 0.00000333 | - | - | - |
| cycles | - | - | - | 93.79k | 1342 | - | - | - |
| instructions | - | - | - | 72.31k | 682.1 | - | - | - |
| cache references | - | - | - | 6.508 | 5.885 | - | - | - |
| cache misses | - | - | - | 0.185 | 0.0416 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000310 | 0.000854 | - | - | - |
| data TLB read misses | - | - | - | 0.00530 | 0.0108 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.124M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.124M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 3550038 | 100003 | 0 | 0 | 78100836 | 66009614 | 0 | 31981.6 | 360710.1 | 562036.7 | 587202.6 | 588422.9 | 0 | 0 | 22/22 | 337.2 | 35.4 | - | 6.08 / 8.43 |
| best | wake | 22 × wait | 3550038 | 100003 | 0 | 0 | 78100836 | 66009614 | 0 | 31981.6 | 360710.1 | 562036.7 | 587202.6 | 588422.9 | 0 | 0 | 22/22 | 337.2 | 35.4 | - | 6.08 / 8.43 |

Run 1's profile:

| profile: the window, 30.0 s, 3023342 messages, 66513522 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 285.1G | 94.28k | 4286 | 9906 | 3835 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.120 | 3.176 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 66.50M | 21.99 | 1.000 | 0 | 1.000 | - | - | - |
| involuntary context switches | 1465 | 0.000485 | 0.0000220 | 0.0000728 | 0.0000187 | - | - | - |
| cycles | - | - | - | 12.07k | 1810 | - | - | - |
| instructions | - | - | - | 3651 | 699.6 | - | - | - |
| cache references | - | - | - | 38.78 | 12.01 | - | - | - |
| cache misses | - | - | - | 0.0521 | 0.0179 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000126 | 0.000168 | - | - | - |
| data TLB read misses | - | - | - | 0.00117 | 0.241 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.545M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.545M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 3527092 | 99354 | 22946 | 0 | 77596024 | 66009614 | 0 | 562036.7 | 1090519.0 | 1291845.6 | 1321996.0 | 1321996.0 | 0 | 0 | 22/22 | 363.7 | 35.4 | - | 9.36 / 11.22 |
| best | wake | 22 × wait | 3527092 | 99354 | 22946 | 0 | 77596024 | 66009614 | 0 | 562036.7 | 1090519.0 | 1291845.6 | 1321996.0 | 1321996.0 | 0 | 0 | 22/22 | 363.7 | 35.4 | - | 9.36 / 11.22 |

Run 1's profile:

| profile: the window, 30.0 s, 3046033 messages, 67012729 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 308.5G | 101.3k | 4603 | 9830 | 4156 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.289 | 4.176 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 66.99M | 21.99 | 1.000 | 0 | 1.000 | - | - | - |
| involuntary context switches | 2118 | 0.000695 | 0.0000316 | 0.0000456 | 0.0000295 | - | - | - |
| cycles | - | - | - | 8531 | 1836 | - | - | - |
| instructions | - | - | - | 2137 | 697.5 | - | - | - |
| cache references | - | - | - | 34.35 | 15.03 | - | - | - |
| cache misses | - | - | - | 0.0404 | 0.0206 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0000948 | 0.000168 | - | - | - |
| data TLB read misses | - | - | - | 0.000770 | 0.425 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.733M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.733M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 0.1 | 0.2 | 0.4 | 2.4 | 41.6 | 0 | 0 | 1/1 | 70.9 | 35.5 | - | 3.46 / 2.91 |
| best | wake | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 0.1 | 0.2 | 0.4 | 2.4 | 41.6 | 0 | 0 | 1/1 | 70.9 | 35.5 | - | 3.46 / 2.91 |

Run 1's profile:

| profile: the window, 30.0 s, 299586 messages, 299586 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.92G | 200.0k | 200.0k | 100.0k | 100.0k | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.170 | 12.37 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000103 | 0.000103 | 0.000103 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 453.0 | 0.00151 | 0.00151 | 0.000210 | 0.00130 | - | - | - |
| cycles | - | - | - | 557.5k | 557.4k | - | - | - |
| instructions | - | - | - | 319.7k | 81.42k | - | - | - |
| cache references | - | - | - | 4.446 | 3.502 | - | - | - |
| cache misses | - | - | - | 0.0365 | 0.0997 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00116 | 0.00161 | - | - | - |
| data TLB read misses | - | - | - | 0.000234 | 0.00255 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.062M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.062M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 354798 | 9994 | 0 | 0 | 7805556 | 6590826 | 0 | 0.3 | 0.3 | 0.5 | 2.7 | 153.0 | 0 | 0 | 22/22 | 813.9 | 35.4 | - | 39.68 / 32.74 |
| best | wake | 22 × busy | 354798 | 9994 | 0 | 0 | 7805556 | 6590826 | 0 | 0.3 | 0.3 | 0.5 | 2.7 | 153.0 | 0 | 0 | 22/22 | 813.9 | 35.4 | - | 39.68 / 32.74 |

Run 1's profile:

| profile: the window, 30.0 s, 299581 messages, 6590782 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.3G | 2.298M | 104.4k | 99.94k | 99.89k | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.660 | 3.923 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000103 | 0.00000470 | 0.000103 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 2786 | 0.00930 | 0.000423 | 0.000210 | 0.000413 | - | - | - |
| cycles | - | - | - | 372.2k | 400.7k | - | - | - |
| instructions | - | - | - | 373.7k | 116.9k | - | - | - |
| cache references | - | - | - | 3.005 | 356.1 | - | - | - |
| cache misses | - | - | - | 0.00544 | 0.00842 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.000127 | - | - | - |
| data TLB read misses | - | - | - | 0 | 0.000504 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.557M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.557M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 354798 | 9994 | 0 | 0 | 7805556 | 6590826 | 0 | 0.3 | 0.4 | 0.6 | 4.7 | 3425.3 | 0 | 0 | 22/22 | 812.9 | 35.4 | - | 59.18 / 44.27 |
| best | wake | 22 × busy | 354798 | 9994 | 0 | 0 | 7805556 | 6590826 | 0 | 0.3 | 0.4 | 0.6 | 4.7 | 3425.3 | 0 | 0 | 22/22 | 812.9 | 35.4 | - | 59.18 / 44.27 |

Run 1's profile:

| profile: the window, 30.0 s, 299584 messages, 6590849 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 687.9G | 2.296M | 104.4k | 99.80k | 99.84k | - | - | - |
| waiting for a CPU, ns | - | - | - | 36.71 | 34.26 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000103 | 0.00000470 | 0.000103 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 4151 | 0.0139 | 0.000630 | 0.000524 | 0.000606 | - | - | - |
| cycles | - | - | - | 408.9k | 391.0k | - | - | - |
| instructions | - | - | - | 229.6k | 117.3k | - | - | - |
| cache references | - | - | - | 5.307 | 388.8 | - | - | - |
| cache misses | - | - | - | 0.157 | 0.0570 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0217 | 0.00140 | - | - | - |
| data TLB read misses | - | - | - | 0.00972 | 0.0190 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.606M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.606M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × busy | 3550038 | 99999 | 0 | 0 | 3550038 | 3000437 | 0 | 0.2 | 0.2 | 0.2 | 2.3 | 472.6 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 6.53 / 5.08 |
| best | wake | 1 × busy | 3550038 | 99999 | 0 | 0 | 3550038 | 3000437 | 0 | 0.2 | 0.2 | 0.2 | 2.3 | 472.6 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 6.53 / 5.08 |

Run 1's profile:

| profile: the window, 30.0 s, 3000441 messages, 3000441 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.90G | 19.96k | 19.96k | 9981 | 9982 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.181 | 1.122 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 443.0 | 0.000148 | 0.000148 | 0.0000913 | 0.0000563 | - | - | - |
| cycles | - | - | - | 52.44k | 92.97k | - | - | - |
| instructions | - | - | - | 30.31k | 23.30k | - | - | - |
| cache references | - | - | - | 3.197 | 4.988 | - | - | - |
| cache misses | - | - | - | 0.0120 | 0.231 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00162 | 0.000795 | - | - | - |
| data TLB read misses | - | - | - | 0.0000187 | 0.00381 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.062M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.062M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 3550038 | 99999 | 0 | 0 | 78100836 | 66009614 | 0 | 0.2 | 0.3 | 0.6 | 2.6 | 304.6 | 0 | 0 | 22/22 | 814.0 | 35.4 | - | 26.22 / 24.90 |
| best | wake | 22 × busy | 3550038 | 99999 | 0 | 0 | 78100836 | 66009614 | 0 | 0.2 | 0.3 | 0.6 | 2.6 | 304.6 | 0 | 0 | 22/22 | 814.0 | 35.4 | - | 26.22 / 24.90 |

Run 1's profile:

| profile: the window, 30.0 s, 3000439 messages, 66009658 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.4G | 229.4k | 10.43k | 9976 | 9974 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.194 | 0.566 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 3984 | 0.00133 | 0.0000604 | 0.0000263 | 0.0000591 | - | - | - |
| cycles | - | - | - | 45.65k | 39.21k | - | - | - |
| instructions | - | - | - | 26.44k | 11.91k | - | - | - |
| cache references | - | - | - | 3.376 | 35.55 | - | - | - |
| cache misses | - | - | - | 0.00465 | 0.00493 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000338 | 0.00000779 | - | - | - |
| data TLB read misses | - | - | - | 0.000128 | 0.0000691 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.643M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.643M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 3550038 | 100002 | 0 | 0 | 78100836 | 66009614 | 0 | 0.3 | 0.4 | 0.6 | 2.8 | 17950.8 | 0 | 0 | 22/22 | 813.5 | 35.4 | - | 39.29 / 33.22 |
| best | wake | 22 × busy | 3550038 | 100002 | 0 | 0 | 78100836 | 66009614 | 0 | 0.3 | 0.4 | 0.6 | 2.8 | 17950.8 | 0 | 0 | 22/22 | 813.5 | 35.4 | - | 39.29 / 33.22 |

Run 1's profile:

| profile: the window, 30.0 s, 3000359 messages, 66007900 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.2G | 229.4k | 10.43k | 9974 | 9972 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.769 | 2.086 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 3215 | 0.00107 | 0.0000487 | 0.0000363 | 0.0000471 | - | - | - |
| cycles | - | - | - | 35.77k | 38.40k | - | - | - |
| instructions | - | - | - | 37.48k | 11.21k | - | - | - |
| cache references | - | - | - | 2.183 | 34.84 | - | - | - |
| cache misses | - | - | - | 0.00384 | 0.00457 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.0000425 | - | - | - |
| data TLB read misses | - | - | - | 0 | 0.000118 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.803M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.803M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 4.0 | 7.2 | 21.2 | 86.0 | 2058.5 | 0 | 0 | 1/1 | 36.9 | 35.4 | - | 5.14 / 3.36 |
| best | wake | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 4.0 | 7.2 | 21.2 | 86.0 | 2058.5 | 0 | 0 | 1/1 | 36.9 | 35.4 | - | 5.14 / 3.36 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 300768 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 31.16G | 103.6k | 103.6k | 99.60k | 4010 | - | - | - |
| waiting for a CPU, ns | - | - | - | 9.417 | 10.69 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 30.00 | 0.0000997 | 0.0000997 | 0.0000997 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 284.8k | 0.947 | 0.947 | 0 | 0.947 | - | - | - |
| involuntary context switches | 279.0 | 0.000928 | 0.000928 | 0.000898 | 0.0000299 | - | - | - |
| cycles | - | - | - | 552.5k | 1588 | - | - | - |
| instructions | - | - | - | 315.4k | 732.2 | - | - | - |
| cache references | - | - | - | 4.993 | 16.15 | - | - | - |
| cache misses | - | - | - | 0.0285 | 0.229 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00223 | 0.00515 | - | - | - |
| data TLB read misses | - | - | - | 0.000326 | 0.308 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.124M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.124M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 356101 | 10031 | 0 | 0 | 7834222 | 6616918 | 0 | 9.1 | 20.7 | 32.8 | 68.6 | 2923.4 | 0 | 0 | 22/22 | 75.8 | 35.4 | - | 5.67 / 4.70 |
| best | wake | 22 × wait | 356101 | 10031 | 0 | 0 | 7834222 | 6616918 | 0 | 9.1 | 20.7 | 32.8 | 68.6 | 2923.4 | 0 | 0 | 22/22 | 75.8 | 35.4 | - | 5.67 / 4.70 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 6616896 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 63.95G | 212.6k | 9665 | 99.43k | 5146 | - | - | - |
| waiting for a CPU, ns | - | - | - | 22.02 | 13.57 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 30.00 | 0.0000997 | 0.00000453 | 0.0000997 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 6.467M | 21.50 | 0.977 | 0 | 0.977 | - | - | - |
| involuntary context switches | 1323 | 0.00440 | 0.000200 | 0.00224 | 0.0000979 | - | - | - |
| cycles | - | - | - | 770.8k | 1684 | - | - | - |
| instructions | - | - | - | 587.4k | 664.9 | - | - | - |
| cache references | - | - | - | 39.71 | 10.06 | - | - | - |
| cache misses | - | - | - | 0.898 | 0.149 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00484 | 0.00918 | - | - | - |
| data TLB read misses | - | - | - | 0.0750 | 0.956 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.713M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.713M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 356101 | 10031 | 0 | 0 | 7834222 | 6616918 | 0 | 9.0 | 21.0 | 33.3 | 65.0 | 2971.3 | 0 | 0 | 22/22 | 75.8 | 35.4 | - | 9.16 / 8.40 |
| best | wake | 22 × wait | 356101 | 10031 | 0 | 0 | 7834222 | 6616918 | 0 | 9.0 | 21.0 | 33.3 | 65.0 | 2971.3 | 0 | 0 | 22/22 | 75.8 | 35.4 | - | 9.16 / 8.40 |

Run 1's profile:

| profile: the window, 30.0 s, 300764 messages, 6616830 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 64.05G | 213.0k | 9680 | 99.44k | 5160 | - | - | - |
| waiting for a CPU, ns | - | - | - | 10.71 | 12.65 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 30.00 | 0.0000997 | 0.00000453 | 0.0000997 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 6.462M | 21.48 | 0.977 | 0 | 0.977 | - | - | - |
| involuntary context switches | 989.0 | 0.00329 | 0.000149 | 0.00172 | 0.0000712 | - | - | - |
| cycles | - | - | - | 786.7k | 1730 | - | - | - |
| instructions | - | - | - | 606.3k | 662.2 | - | - | - |
| cache references | - | - | - | 38.09 | 9.181 | - | - | - |
| cache misses | - | - | - | 0.414 | 0.0864 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00248 | 0.00353 | - | - | - |
| data TLB read misses | - | - | - | 0.0664 | 0.783 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.668M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.668M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × wait | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 3.8 | 4.3 | 6.2 | 91.1 | 1012.3 | 0 | 0 | 1/1 | 43.1 | 35.4 | - | 3.35 / 4.22 |
| best | wake | 1 × wait | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 3.8 | 4.3 | 6.2 | 91.1 | 1012.3 | 0 | 0 | 1/1 | 43.1 | 35.4 | - | 3.35 / 4.22 |

Run 1's profile:

| profile: the window, 30.0 s, 3002638 messages, 3002638 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 36.34G | 12.10k | 12.10k | 9975 | 2129 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.755 | 3.322 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 2.138M | 0.712 | 0.712 | 0 | 0.712 | - | - | - |
| involuntary context switches | 284.0 | 0.0000946 | 0.0000946 | 0.0000866 | 0.00000799 | - | - | - |
| cycles | - | - | - | 92.24k | 1363 | - | - | - |
| instructions | - | - | - | 71.55k | 683.5 | - | - | - |
| cache references | - | - | - | 6.573 | 6.433 | - | - | - |
| cache misses | - | - | - | 0.150 | 0.107 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00196 | 0.00534 | - | - | - |
| data TLB read misses | - | - | - | 0.00425 | 0.0230 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.255M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.255M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 3374010 | 95029 | 180008 | 0 | 74228220 | 63199708 | 2858174 | 872415.2 | 1677721.6 | undelivered | undelivered | undelivered | 0 | 0 | 22/22 | 349.7 | 35.4 | - | 5.26 / 7.22 |
| best | wake | 22 × wait | 3374010 | 95029 | 180008 | 0 | 74228220 | 63199708 | 2858174 | 872415.2 | 1677721.6 | undelivered | undelivered | undelivered | 0 | 0 | 22/22 | 349.7 | 35.4 | - | 5.26 / 7.22 |

Run 1's profile:

| profile: the window, 30.0 s, 2831275 messages, 62288037 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 294.5G | 104.0k | 4728 | 10.58k | 4247 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.149 | 5.042 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 62.26M | 21.99 | 1.000 | 0 | 1.000 | - | - | - |
| involuntary context switches | 2485 | 0.000878 | 0.0000399 | 0.0000244 | 0.0000388 | - | - | - |
| cycles | - | - | - | 10.06k | 2024 | - | - | - |
| instructions | - | - | - | 2921 | 718.1 | - | - | - |
| cache references | - | - | - | 36.58 | 18.23 | - | - | - |
| cache misses | - | - | - | 0.0241 | 0.0228 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000116 | 0.000237 | - | - | - |
| data TLB read misses | - | - | - | 0.00140 | 0.502 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.594M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.594M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 3135930 | 88307 | 418088 | 0 | 68990460 | 57961948 | 8095934 | 473956.4 | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 22/22 | 343.1 | 35.4 | - | 9.25 / 10.24 |
| best | wake | 22 × wait | 3135930 | 88307 | 418088 | 0 | 68990460 | 57961948 | 8095934 | 473956.4 | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 22/22 | 343.1 | 35.4 | - | 9.25 / 10.24 |

Run 1's profile:

| profile: the window, 30.0 s, 2602040 messages, 57244887 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 289.3G | 111.2k | 5053 | 11.51k | 4530 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.276 | 9.350 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 57.20M | 21.98 | 0.999 | 0 | 0.999 | - | - | - |
| involuntary context switches | 1924 | 0.000739 | 0.0000336 | 0.0000438 | 0.0000316 | - | - | - |
| cycles | - | - | - | 10.42k | 2002 | - | - | - |
| instructions | - | - | - | 2232 | 714.0 | - | - | - |
| cache references | - | - | - | 50.64 | 18.68 | - | - | - |
| cache misses | - | - | - | 0.0634 | 0.0434 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000310 | 0.000773 | - | - | - |
| data TLB read misses | - | - | - | 0.00104 | 0.570 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.668M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.668M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 0.1 | 0.2 | 0.5 | 2.4 | 85.8 | 0 | 0 | 1/1 | 70.9 | 35.5 | - | 4.14 / 3.27 |
| best | wake | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 0.1 | 0.2 | 0.5 | 2.4 | 85.8 | 0 | 0 | 1/1 | 70.9 | 35.5 | - | 4.14 / 3.27 |

Run 1's profile:

| profile: the window, 30.0 s, 300766 messages, 300766 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.92G | 199.2k | 199.2k | 99.62k | 99.62k | - | - | - |
| waiting for a CPU, ns | - | - | - | 8.084 | 9.206 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 30.00 | 0.0000997 | 0.0000997 | 0.0000997 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 448.0 | 0.00149 | 0.00149 | 0.000791 | 0.000698 | - | - | - |
| cycles | - | - | - | 557.6k | 935.2k | - | - | - |
| instructions | - | - | - | 320.5k | 221.5k | - | - | - |
| cache references | - | - | - | 4.444 | 31.62 | - | - | - |
| cache misses | - | - | - | 0.0423 | 13.99 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00164 | 0.00232 | - | - | - |
| data TLB read misses | - | - | - | 0.000163 | 1.296 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.005M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.005M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 356101 | 10031 | 0 | 0 | 7834222 | 6616918 | 0 | 0.3 | 0.4 | 0.6 | 2.8 | 2010.8 | 0 | 0 | 22/22 | 813.5 | 35.4 | - | 33.61 / 28.92 |
| best | wake | 22 × busy | 356101 | 10031 | 0 | 0 | 7834222 | 6616918 | 0 | 0.3 | 0.4 | 0.6 | 2.8 | 2010.8 | 0 | 0 | 22/22 | 813.5 | 35.4 | - | 33.61 / 28.92 |

Run 1's profile:

| profile: the window, 30.0 s, 300759 messages, 6616698 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.2G | 2.288M | 104.0k | 99.48k | 99.49k | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.623 | 7.753 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 30.00 | 0.0000997 | 0.00000453 | 0.0000997 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 3530 | 0.0117 | 0.000533 | 0.000236 | 0.000523 | - | - | - |
| cycles | - | - | - | 358.8k | 383.8k | - | - | - |
| instructions | - | - | - | 373.6k | 112.1k | - | - | - |
| cache references | - | - | - | 3.982 | 363.6 | - | - | - |
| cache misses | - | - | - | 0.0128 | 0.0351 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.00139 | - | - | - |
| data TLB read misses | - | - | - | 0.0000465 | 0.0659 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.668M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.668M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 356101 | 10031 | 0 | 0 | 7834222 | 6616918 | 0 | 0.3 | 0.4 | 0.5 | 3.1 | 5123.0 | 0 | 0 | 22/22 | 813.8 | 35.4 | - | 94.85 / 64.14 |
| best | wake | 22 × busy | 356101 | 10031 | 0 | 0 | 7834222 | 6616918 | 0 | 0.3 | 0.4 | 0.5 | 3.1 | 5123.0 | 0 | 0 | 22/22 | 813.8 | 35.4 | - | 94.85 / 64.14 |

Run 1's profile:

| profile: the window, 30.0 s, 300757 messages, 6616656 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.1G | 2.288M | 104.0k | 99.43k | 99.48k | - | - | - |
| waiting for a CPU, ns | - | - | - | 4.570 | 10.35 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 30.00 | 0.0000997 | 0.00000453 | 0.0000997 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 3070 | 0.0102 | 0.000464 | 0.000273 | 0.000452 | - | - | - |
| cycles | - | - | - | 395.8k | 348.6k | - | - | - |
| instructions | - | - | - | 210.5k | 106.5k | - | - | - |
| cache references | - | - | - | 3.331 | 313.7 | - | - | - |
| cache misses | - | - | - | 0.111 | 0.0344 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0191 | 0.000727 | - | - | - |
| data TLB read misses | - | - | - | 0.0000698 | 0.0113 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.676M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.676M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × busy | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 0.1 | 0.2 | 0.5 | 2.4 | 74.0 | 0 | 0 | 1/1 | 70.9 | 35.5 | - | 9.23 / 5.83 |
| best | wake | 1 × busy | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 0.1 | 0.2 | 0.5 | 2.4 | 74.0 | 0 | 0 | 1/1 | 70.9 | 35.5 | - | 9.23 / 5.83 |

Run 1's profile:

| profile: the window, 30.0 s, 3002635 messages, 3002635 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.92G | 19.96k | 19.96k | 9980 | 9977 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.0488 | 1.368 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 423.0 | 0.000141 | 0.000141 | 0.0000120 | 0.000129 | - | - | - |
| cycles | - | - | - | 55.67k | 55.65k | - | - | - |
| instructions | - | - | - | 32.16k | 8284 | - | - | - |
| cache references | - | - | - | 3.324 | 2.404 | - | - | - |
| cache misses | - | - | - | 0.00303 | 0.0855 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000279 | 0.000516 | - | - | - |
| data TLB read misses | - | - | - | 0.0000143 | 0.00124 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.136M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.136M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 3554018 | 100113 | 0 | 0 | 78188396 | 66057882 | 0 | 0.2 | 0.3 | 0.8 | 4.5 | 126.1 | 0 | 0 | 22/22 | 814.0 | 35.4 | - | 25.05 / 24.25 |
| best | wake | 22 × busy | 3554018 | 100113 | 0 | 0 | 78188396 | 66057882 | 0 | 0.2 | 0.3 | 0.8 | 4.5 | 126.1 | 0 | 0 | 22/22 | 814.0 | 35.4 | - | 25.05 / 24.25 |

Run 1's profile:

| profile: the window, 30.0 s, 3002637 messages, 66058014 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.2G | 229.2k | 10.42k | 9959 | 9965 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.387 | 0.454 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 3724 | 0.00124 | 0.0000564 | 0.0000346 | 0.0000548 | - | - | - |
| cycles | - | - | - | 45.91k | 39.37k | - | - | - |
| instructions | - | - | - | 26.50k | 11.94k | - | - | - |
| cache references | - | - | - | 3.382 | 36.07 | - | - | - |
| cache misses | - | - | - | 0.00349 | 0.00458 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000256 | 0.00000612 | - | - | - |
| data TLB read misses | - | - | - | 0.000101 | 0.0000457 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.467M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.467M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 3554018 | 100114 | 0 | 0 | 78188396 | 66057882 | 0 | 0.2 | 0.3 | 0.6 | 3.5 | 1003.1 | 0 | 0 | 22/22 | 813.7 | 35.4 | - | 29.93 / 26.86 |
| best | wake | 22 × busy | 3554018 | 100114 | 0 | 0 | 78188396 | 66057882 | 0 | 0.2 | 0.3 | 0.6 | 3.5 | 1003.1 | 0 | 0 | 22/22 | 813.7 | 35.4 | - | 29.93 / 26.86 |

Run 1's profile:

| profile: the window, 30.0 s, 3002637 messages, 66058014 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.1G | 229.2k | 10.42k | 9966 | 9963 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.008 | 1.338 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 5094 | 0.00170 | 0.0000771 | 0.0000446 | 0.0000751 | - | - | - |
| cycles | - | - | - | 45.73k | 39.26k | - | - | - |
| instructions | - | - | - | 26.39k | 11.92k | - | - | - |
| cache references | - | - | - | 3.394 | 36.29 | - | - | - |
| cache misses | - | - | - | 0.00427 | 0.00461 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000274 | 0.00000975 | - | - | - |
| data TLB read misses | - | - | - | 0.000127 | 0.0000490 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.533M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.533M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
