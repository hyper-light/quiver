# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 3.9 | 5.1 | 20.5 | 69.6 | 226.0 | 0 | 0 | 1/1 | 36.8 | 35.4 | - | 2.43 / 1.86 |
| best | wake | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 3.9 | 5.1 | 20.5 | 69.6 | 226.0 | 0 | 0 | 1/1 | 36.8 | 35.4 | - | 2.43 / 1.86 |

Run 1's profile:

| profile: the window, 30.0 s, 299500 messages, 299500 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 31.09G | 103.8k | 103.8k | 100.0k | 3779 | - | - | - |
| waiting for a CPU, ns | - | - | - | 17.30 | 3.349 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000104 | 0.000104 | 0.000104 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 284.7k | 0.951 | 0.951 | 0 | 0.951 | - | - | - |
| involuntary context switches | 431.0 | 0.00144 | 0.00144 | 0.00143 | 0.00000668 | - | - | - |
| cycles | - | - | - | 565.0k | 1647 | - | - | - |
| instructions | - | - | - | 322.5k | 740.8 | - | - | - |
| cache references | - | - | - | 4.832 | 11.66 | - | - | - |
| cache misses | - | - | - | 0.0250 | 0.215 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00173 | 0.0102 | - | - | - |
| data TLB read misses | - | - | - | 0.000601 | 0.137 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.111M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.111M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 354590 | 9988 | 0 | 0 | 7800980 | 6588934 | 0 | 8.8 | 21.0 | 33.3 | 69.6 | 1234.6 | 0 | 0 | 22/22 | 75.5 | 35.4 | - | 2.07 / 2.59 |
| best | wake | 22 × wait | 354590 | 9988 | 0 | 0 | 7800980 | 6588934 | 0 | 8.8 | 21.0 | 33.3 | 69.6 | 1234.6 | 0 | 0 | 22/22 | 75.5 | 35.4 | - | 2.07 / 2.59 |

Run 1's profile:

| profile: the window, 30.0 s, 299498 messages, 6588956 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 63.80G | 213.0k | 9683 | 99.86k | 5144 | - | - | - |
| waiting for a CPU, ns | - | - | - | 13.32 | 10.79 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000104 | 0.00000470 | 0.000104 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 6.436M | 21.49 | 0.977 | 0 | 0.977 | - | - | - |
| involuntary context switches | 1049 | 0.00350 | 0.000159 | 0.00206 | 0.0000654 | - | - | - |
| cycles | - | - | - | 793.1k | 1684 | - | - | - |
| instructions | - | - | - | 607.7k | 665.2 | - | - | - |
| cache references | - | - | - | 49.61 | 9.120 | - | - | - |
| cache misses | - | - | - | 0.505 | 0.103 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00462 | 0.00467 | - | - | - |
| data TLB read misses | - | - | - | 0.0857 | 0.793 | - | - | - |
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
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 354590 | 9988 | 0 | 0 | 7800980 | 6588934 | 0 | 8.8 | 21.0 | 33.3 | 67.6 | 2000.3 | 0 | 0 | 22/22 | 75.5 | 35.4 | - | 6.55 / 4.99 |
| best | wake | 22 × wait | 354590 | 9988 | 0 | 0 | 7800980 | 6588934 | 0 | 8.8 | 21.0 | 33.3 | 67.6 | 2000.3 | 0 | 0 | 22/22 | 75.5 | 35.4 | - | 6.55 / 4.99 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 6588971 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 63.84G | 213.1k | 9688 | 99.86k | 5149 | - | - | - |
| waiting for a CPU, ns | - | - | - | 9.966 | 12.19 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000104 | 0.00000470 | 0.000104 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 6.435M | 21.49 | 0.977 | 0 | 0.977 | - | - | - |
| involuntary context switches | 1028 | 0.00343 | 0.000156 | 0.00185 | 0.0000721 | - | - | - |
| cycles | - | - | - | 786.8k | 1641 | - | - | - |
| instructions | - | - | - | 597.4k | 662.1 | - | - | - |
| cache references | - | - | - | 44.12 | 10.04 | - | - | - |
| cache misses | - | - | - | 0.523 | 0.0950 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00372 | 0.00421 | - | - | - |
| data TLB read misses | - | - | - | 0.0827 | 0.804 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.696M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.696M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 3.8 | 4.2 | 5.6 | 60.9 | 360.5 | 0 | 0 | 1/1 | 43.0 | 35.4 | - | 10.30 / 6.36 |
| best | wake | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 3.8 | 4.2 | 5.6 | 60.9 | 360.5 | 0 | 0 | 1/1 | 43.0 | 35.4 | - | 10.30 / 6.36 |

Run 1's profile:

| profile: the window, 30.0 s, 2998273 messages, 2998272 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 36.25G | 12.09k | 12.09k | 9991 | 2098 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.615 | 1.117 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 2.158M | 0.720 | 0.720 | 0 | 0.720 | - | - | - |
| involuntary context switches | 233.0 | 0.0000777 | 0.0000777 | 0.0000740 | 0.00000367 | - | - | - |
| cycles | - | - | - | 54.56k | 1394 | - | - | - |
| instructions | - | - | - | 31.19k | 689.3 | - | - | - |
| cache references | - | - | - | 3.208 | 6.378 | - | - | - |
| cache misses | - | - | - | 0.0186 | 0.0632 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000234 | 0.000570 | - | - | - |
| data TLB read misses | - | - | - | 0.0000447 | 0.0167 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.074M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.074M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 3421040 | 96357 | 126503 | 0 | 75262880 | 64276718 | 1685310 | 671088.6 | 1476395.0 | undelivered | undelivered | undelivered | 0 | 0 | 22/22 | 358.4 | 35.4 | - | 2.66 / 6.24 |
| best | wake | 22 × wait | 3421040 | 96357 | 126503 | 0 | 75262880 | 64276718 | 1685310 | 671088.6 | 1476395.0 | undelivered | undelivered | undelivered | 0 | 0 | 22/22 | 358.4 | 35.4 | - | 2.66 / 6.24 |

Run 1's profile:

| profile: the window, 30.0 s, 2901817 messages, 63839955 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 305.1G | 105.2k | 4780 | 10.32k | 4310 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.440 | 4.811 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 63.81M | 21.99 | 1.000 | 0 | 1.000 | - | - | - |
| involuntary context switches | 2985 | 0.00103 | 0.0000468 | 0.0000544 | 0.0000443 | - | - | - |
| cycles | - | - | - | 10.45k | 1953 | - | - | - |
| instructions | - | - | - | 2580 | 712.4 | - | - | - |
| cache references | - | - | - | 40.67 | 18.66 | - | - | - |
| cache misses | - | - | - | 0.107 | 0.0240 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000185 | 0.000324 | - | - | - |
| data TLB read misses | - | - | - | 0.00263 | 0.533 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.586M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.586M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 3473949 | 97852 | 73594 | 0 | 76426878 | 65440716 | 521312 | 226492.4 | 494927.9 | 729808.9 | undelivered | undelivered | 0 | 0 | 22/22 | 357.8 | 35.4 | - | 5.03 / 7.82 |
| best | wake | 22 × wait | 3473949 | 97852 | 73594 | 0 | 76426878 | 65440716 | 521312 | 226492.4 | 494927.9 | 729808.9 | undelivered | undelivered | 0 | 0 | 22/22 | 357.8 | 35.4 | - | 5.03 / 7.82 |

Run 1's profile:

| profile: the window, 30.0 s, 2963086 messages, 65187901 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 304.0G | 102.6k | 4663 | 10.11k | 4203 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.418 | 4.140 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 65.17M | 21.99 | 1.000 | 0 | 1.000 | - | - | - |
| involuntary context switches | 2109 | 0.000712 | 0.0000324 | 0.0000415 | 0.0000305 | - | - | - |
| cycles | - | - | - | 10.93k | 1867 | - | - | - |
| instructions | - | - | - | 2807 | 705.6 | - | - | - |
| cache references | - | - | - | 38.63 | 16.77 | - | - | - |
| cache misses | - | - | - | 0.0541 | 0.0205 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000173 | 0.000254 | - | - | - |
| data TLB read misses | - | - | - | 0.00266 | 0.465 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.578M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.578M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 0.2 | 0.2 | 0.2 | 2.1 | 951.7 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 11.07 / 8.05 |
| best | wake | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 0.2 | 0.2 | 0.2 | 2.1 | 951.7 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 11.07 / 8.05 |

Run 1's profile:

| profile: the window, 30.0 s, 299500 messages, 299500 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.89G | 200.0k | 200.0k | 99.99k | 99.98k | - | - | - |
| waiting for a CPU, ns | - | - | - | 12.60 | 8.933 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000104 | 0.000104 | 0.000104 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 387.0 | 0.00129 | 0.00129 | 0.000691 | 0.000601 | - | - | - |
| cycles | - | - | - | 499.7k | 901.1k | - | - | - |
| instructions | - | - | - | 287.0k | 225.8k | - | - | - |
| cache references | - | - | - | 4.136 | 7.595 | - | - | - |
| cache misses | - | - | - | 0.126 | 0.908 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0183 | 0.0113 | - | - | - |
| data TLB read misses | - | - | - | 0.000775 | 0.0506 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 5.976M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 5.976M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 354590 | 9988 | 0 | 0 | 7800980 | 6588934 | 0 | 0.3 | 0.4 | 0.7 | 3.7 | 9193.0 | 0 | 0 | 22/22 | 814.3 | 35.4 | - | 3.83 / 12.71 |
| best | wake | 22 × busy | 354590 | 9988 | 0 | 0 | 7800980 | 6588934 | 0 | 0.3 | 0.4 | 0.7 | 3.7 | 9193.0 | 0 | 0 | 22/22 | 814.3 | 35.4 | - | 3.83 / 12.71 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 6588978 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.5G | 2.299M | 104.5k | 99.94k | 99.96k | - | - | - |
| waiting for a CPU, ns | - | - | - | 31.49 | 19.76 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000104 | 0.00000470 | 0.000104 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 2924 | 0.00976 | 0.000444 | 0.000457 | 0.000423 | - | - | - |
| cycles | - | - | - | 451.3k | 422.7k | - | - | - |
| instructions | - | - | - | 249.9k | 125.9k | - | - | - |
| cache references | - | - | - | 5.754 | 385.8 | - | - | - |
| cache misses | - | - | - | 0.425 | 0.0489 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0365 | 0.00116 | - | - | - |
| data TLB read misses | - | - | - | 0.0278 | 0.00166 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.582M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.582M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 354590 | 9989 | 0 | 0 | 7800980 | 6588934 | 0 | 0.3 | 0.4 | 0.7 | 8.7 | 3357.3 | 0 | 0 | 22/22 | 813.4 | 35.4 | - | 32.16 / 28.78 |
| best | wake | 22 × busy | 354590 | 9989 | 0 | 0 | 7800980 | 6588934 | 0 | 0.3 | 0.4 | 0.7 | 8.7 | 3357.3 | 0 | 0 | 22/22 | 813.4 | 35.4 | - | 32.16 / 28.78 |

Run 1's profile:

| profile: the window, 30.0 s, 299498 messages, 6588956 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 687.6G | 2.296M | 104.4k | 99.82k | 99.82k | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.104 | 6.353 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000104 | 0.00000470 | 0.000104 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 2801 | 0.00935 | 0.000425 | 0.000184 | 0.000417 | - | - | - |
| cycles | - | - | - | 408.6k | 356.1k | - | - | - |
| instructions | - | - | - | 216.7k | 108.5k | - | - | - |
| cache references | - | - | - | 3.357 | 311.5 | - | - | - |
| cache misses | - | - | - | 0.0667 | 0.0455 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0138 | 0.00180 | - | - | - |
| data TLB read misses | - | - | - | 0.0000467 | 0.00249 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.635M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.635M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 0.1 | 0.2 | 0.5 | 2.5 | 243.9 | 0 | 0 | 1/1 | 70.9 | 35.5 | - | 28.90 / 17.00 |
| best | wake | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 0.1 | 0.2 | 0.5 | 2.5 | 243.9 | 0 | 0 | 1/1 | 70.9 | 35.5 | - | 28.90 / 17.00 |

Run 1's profile:

| profile: the window, 30.0 s, 2998272 messages, 2998272 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.92G | 19.98k | 19.98k | 9992 | 9992 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.708 | 1.348 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 593.0 | 0.000198 | 0.000198 | 0.0000927 | 0.000105 | - | - | - |
| cycles | - | - | - | 55.44k | 93.39k | - | - | - |
| instructions | - | - | - | 31.97k | 22.72k | - | - | - |
| cache references | - | - | - | 3.230 | 9.271 | - | - | - |
| cache misses | - | - | - | 0.00588 | 1.630 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000175 | 0.000207 | - | - | - |
| data TLB read misses | - | - | - | 0.0000430 | 0.155 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.038M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.038M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 3547543 | 99929 | 0 | 0 | 78045946 | 65962028 | 0 | 0.3 | 0.3 | 0.6 | 3.9 | 10891.0 | 0 | 0 | 22/22 | 812.4 | 35.4 | - | 5.18 / 13.95 |
| best | wake | 22 × busy | 3547543 | 99929 | 0 | 0 | 78045946 | 65962028 | 0 | 0.3 | 0.3 | 0.6 | 3.9 | 10891.0 | 0 | 0 | 22/22 | 812.4 | 35.4 | - | 5.18 / 13.95 |

Run 1's profile:

| profile: the window, 30.0 s, 2998279 messages, 65962138 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.1G | 229.5k | 10.43k | 9977 | 9979 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.391 | 1.516 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 2541 | 0.000847 | 0.0000385 | 0.0000440 | 0.0000365 | - | - | - |
| cycles | - | - | - | 43.21k | 37.34k | - | - | - |
| instructions | - | - | - | 22.96k | 11.37k | - | - | - |
| cache references | - | - | - | 2.317 | 36.70 | - | - | - |
| cache misses | - | - | - | 0.0104 | 0.00791 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00203 | 0.0000730 | - | - | - |
| data TLB read misses | - | - | - | 0.0000414 | 0.000139 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.635M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.635M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 3547543 | 99932 | 0 | 0 | 78045946 | 65962028 | 0 | 0.3 | 0.4 | 0.7 | 1359.9 | 61003.8 | 0 | 0 | 22/22 | 810.8 | 35.4 | - | 31.57 / 29.45 |
| best | wake | 22 × busy | 3547543 | 99932 | 0 | 0 | 78045946 | 65962028 | 0 | 0.3 | 0.4 | 0.7 | 1359.9 | 61003.8 | 0 | 0 | 22/22 | 810.8 | 35.4 | - | 31.57 / 29.45 |

Run 1's profile:

| profile: the window, 30.0 s, 2998209 messages, 65960598 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 686.9G | 229.1k | 10.41k | 9976 | 9960 | - | - | - |
| waiting for a CPU, ns | - | - | - | 4.566 | 15.83 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 4353 | 0.00145 | 0.0000660 | 0.000123 | 0.0000604 | - | - | - |
| cycles | - | - | - | 33.22k | 38.43k | - | - | - |
| instructions | - | - | - | 34.56k | 11.07k | - | - | - |
| cache references | - | - | - | 2.264 | 37.22 | - | - | - |
| cache misses | - | - | - | 0.00894 | 0.0101 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.000244 | - | - | - |
| data TLB read misses | - | - | - | 0.0000320 | 0.000402 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.840M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.840M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 4.4 | 5.9 | 23.0 | 94.2 | 2026.6 | 0 | 0 | 1/1 | 36.9 | 35.4 | - | 8.45 / 6.99 |
| best | wake | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 4.4 | 5.9 | 23.0 | 94.2 | 2026.6 | 0 | 0 | 1/1 | 36.9 | 35.4 | - | 8.45 / 6.99 |

Run 1's profile:

| profile: the window, 30.0 s, 299585 messages, 299585 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 31.16G | 104.0k | 104.0k | 99.90k | 4116 | - | - | - |
| waiting for a CPU, ns | - | - | - | 14.94 | 6.461 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000103 | 0.000103 | 0.000103 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 283.0k | 0.945 | 0.945 | 0 | 0.945 | - | - | - |
| involuntary context switches | 156.0 | 0.000521 | 0.000521 | 0.000501 | 0.0000200 | - | - | - |
| cycles | - | - | - | 858.0k | 1624 | - | - | - |
| instructions | - | - | - | 671.3k | 642.3 | - | - | - |
| cache references | - | - | - | 12.63 | 24.59 | - | - | - |
| cache misses | - | - | - | 1.496 | 3.076 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0457 | 0.155 | - | - | - |
| data TLB read misses | - | - | - | 0.0226 | 0.908 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 5.964M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 5.964M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 354798 | 9994 | 0 | 0 | 7805556 | 6590826 | 0 | 9.7 | 20.2 | 34.3 | 92.2 | 3001.2 | 0 | 0 | 22/22 | 75.8 | 35.4 | - | 13.14 / 9.77 |
| best | wake | 22 × wait | 354798 | 9994 | 0 | 0 | 7805556 | 6590826 | 0 | 9.7 | 20.2 | 34.3 | 92.2 | 3001.2 | 0 | 0 | 22/22 | 75.8 | 35.4 | - | 13.14 / 9.77 |

Run 1's profile:

| profile: the window, 30.0 s, 299583 messages, 6590814 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 63.92G | 213.4k | 9699 | 99.75k | 5165 | - | - | - |
| waiting for a CPU, ns | - | - | - | 64.78 | 33.41 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000103 | 0.00000470 | 0.000103 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 6.480M | 21.63 | 0.983 | 0 | 0.983 | - | - | - |
| involuntary context switches | 3253 | 0.0109 | 0.000494 | 0.00545 | 0.000246 | - | - | - |
| cycles | - | - | - | 737.8k | 1867 | - | - | - |
| instructions | - | - | - | 560.0k | 670.6 | - | - | - |
| cache references | - | - | - | 54.90 | 16.07 | - | - | - |
| cache misses | - | - | - | 1.159 | 0.559 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0196 | 0.0283 | - | - | - |
| data TLB read misses | - | - | - | 0.0902 | 1.343 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.680M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.680M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 354798 | 9994 | 0 | 0 | 7805556 | 6590826 | 0 | 14.0 | 679.9 | 218103.8 | 247463.9 | 251528.8 | 0 | 0 | 22/22 | 72.0 | 33.6 | - | 66.64 / 46.27 |
| best | wake | 22 × wait | 354798 | 9994 | 0 | 0 | 7805556 | 6590826 | 0 | 14.0 | 679.9 | 218103.8 | 247463.9 | 251528.8 | 0 | 0 | 22/22 | 72.0 | 33.6 | - | 66.64 / 46.27 |

Run 1's profile:

| profile: the window, 30.0 s, 299582 messages, 6590782 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 60.49G | 201.9k | 9178 | 94.02k | 4904 | - | - | - |
| waiting for a CPU, ns | - | - | - | 5764 | 6497 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 38.00 | 0.000127 | 0.00000577 | 0.000107 | 0.000000910 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 6.109M | 20.39 | 0.927 | 0.00000334 | 0.927 | - | - | - |
| involuntary context switches | 218.8k | 0.730 | 0.0332 | 0.716 | 0.000660 | - | - | - |
| cycles | - | - | - | 563.7k | 1874 | - | - | - |
| instructions | - | - | - | 421.3k | 654.0 | - | - | - |
| cache references | - | - | - | 47.01 | 22.11 | - | - | - |
| cache misses | - | - | - | 1.172 | 0.726 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0209 | 0.0414 | - | - | - |
| data TLB read misses | - | - | - | 0.0690 | 1.718 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.635M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.635M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × wait | 3550038 | 99999 | 0 | 0 | 3550038 | 3000437 | 0 | 4.2 | 5.1 | 62.0 | 933.9 | 3477.6 | 0 | 0 | 1/1 | 43.7 | 35.4 | - | 27.87 / 18.05 |
| best | wake | 1 × wait | 3550038 | 99999 | 0 | 0 | 3550038 | 3000437 | 0 | 4.2 | 5.1 | 62.0 | 933.9 | 3477.6 | 0 | 0 | 1/1 | 43.7 | 35.4 | - | 27.87 / 18.05 |

Run 1's profile:

| profile: the window, 30.0 s, 3000425 messages, 3000426 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 36.88G | 12.29k | 12.29k | 9966 | 2327 | - | - | - |
| waiting for a CPU, ns | - | - | - | 9.021 | 115.8 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 2.023M | 0.674 | 0.674 | 0 | 0.674 | - | - | - |
| involuntary context switches | 2346 | 0.000782 | 0.000782 | 0.000768 | 0.0000137 | - | - | - |
| cycles | - | - | - | 75.19k | 1193 | - | - | - |
| instructions | - | - | - | 57.63k | 613.2 | - | - | - |
| cache references | - | - | - | 5.558 | 6.012 | - | - | - |
| cache misses | - | - | - | 0.214 | 0.297 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00454 | 0.0139 | - | - | - |
| data TLB read misses | - | - | - | 0.00326 | 0.0685 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 5.915M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 5.915M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 2192357 | 61680 | 1357681 | 0 | 48231854 | 37235616 | 28773998 | 12213813.2 | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 22/22 | 284.7 | 35.1 | - | 6.81 / 10.78 |
| best | wake | 22 × wait | 2192357 | 61680 | 1357681 | 0 | 48231854 | 37235616 | 28773998 | 12213813.2 | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 22/22 | 284.7 | 35.1 | - | 6.81 / 10.78 |

Run 1's profile:

| profile: the window, 30.0 s, 1731734 messages, 38098148 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 231.5G | 133.7k | 6078 | 17.10k | 5300 | - | - | - |
| waiting for a CPU, ns | - | - | - | 164.8 | 336.3 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 37.39M | 21.59 | 0.982 | 0 | 0.982 | - | - | - |
| involuntary context switches | 38.83k | 0.0224 | 0.00102 | 0.0209 | 0.0000685 | - | - | - |
| cycles | - | - | - | 10.12k | 2165 | - | - | - |
| instructions | - | - | - | 2100 | 700.3 | - | - | - |
| cache references | - | - | - | 45.24 | 21.87 | - | - | - |
| cache misses | - | - | - | 0.252 | 0.149 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00200 | 0.00408 | - | - | - |
| data TLB read misses | - | - | - | 0.00114 | 0.615 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.705M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.705M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 1917556 | 53935 | 1632482 | 0 | 42186232 | 31189994 | 34819620 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 22/22 | 258.4 | 35.4 | - | 87.00 / 56.87 |
| best | wake | 22 × wait | 1917556 | 53935 | 1632482 | 0 | 42186232 | 31189994 | 34819620 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 22/22 | 258.4 | 35.4 | - | 87.00 / 56.87 |

Run 1's profile:

| profile: the window, 30.0 s, 1604959 messages, 35309090 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 218.3G | 136.0k | 6183 | 18.63k | 5337 | - | - | - |
| waiting for a CPU, ns | - | - | - | 2.349 | 20.49 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 35.28M | 21.98 | 0.999 | 0 | 0.999 | - | - | - |
| involuntary context switches | 1280 | 0.000798 | 0.0000363 | 0.000192 | 0.0000275 | - | - | - |
| cycles | - | - | - | 11.37k | 2011 | - | - | - |
| instructions | - | - | - | 2166 | 701.6 | - | - | - |
| cache references | - | - | - | 48.12 | 19.83 | - | - | - |
| cache misses | - | - | - | 0.320 | 0.256 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00583 | 0.00985 | - | - | - |
| data TLB read misses | - | - | - | 0.000341 | 0.285 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.639M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.639M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 0.2 | 0.3 | 4.3 | 9044.0 | 17158.5 | 0 | 0 | 1/1 | 70.5 | 35.1 | - | 17.17 / 22.97 |
| best | wake | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 0.2 | 0.3 | 4.3 | 9044.0 | 17158.5 | 0 | 0 | 1/1 | 70.5 | 35.1 | - | 17.17 / 22.97 |

Run 1's profile:

| profile: the window, 30.0 s, 299584 messages, 299584 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.48G | 198.5k | 198.5k | 98.96k | 99.57k | - | - | - |
| waiting for a CPU, ns | - | - | - | 703.7 | 110.0 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000103 | 0.000103 | 0.000103 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 408.0 | 0.00136 | 0.00136 | 0.000991 | 0.000371 | - | - | - |
| cycles | - | - | - | 695.0k | 734.9k | - | - | - |
| instructions | - | - | - | 526.7k | 178.2k | - | - | - |
| cache references | - | - | - | 26.97 | 13.30 | - | - | - |
| cache misses | - | - | - | 7.755 | 2.608 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.343 | 0.0694 | - | - | - |
| data TLB read misses | - | - | - | 0.0902 | 0.0225 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 5.898M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 5.898M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 354798 | 9996 | 0 | 0 | 7805556 | 6590826 | 0 | 0.4 | 0.6 | 8519.7 | 112197.6 | 215723.7 | 0 | 0 | 22/22 | 772.5 | 33.0 | - | 42.24 / 38.23 |
| best | wake | 22 × busy | 354798 | 9996 | 0 | 0 | 7805556 | 6590826 | 0 | 0.4 | 0.6 | 8519.7 | 112197.6 | 215723.7 | 0 | 0 | 22/22 | 772.5 | 33.0 | - | 42.24 / 38.23 |

Run 1's profile:

| profile: the window, 30.0 s, 299581 messages, 6590789 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 661.7G | 2.209M | 100.4k | 94.63k | 96.10k | - | - | - |
| waiting for a CPU, ns | - | - | - | 5218 | 3730 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 31.00 | 0.000103 | 0.00000470 | 0.000103 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 26.74k | 0.0893 | 0.00406 | 0.00315 | 0.00391 | - | - | - |
| cycles | - | - | - | 282.6k | 510.1k | - | - | - |
| instructions | - | - | - | 274.7k | 126.9k | - | - | - |
| cache references | - | - | - | 5.144 | 452.4 | - | - | - |
| cache misses | - | - | - | 0.367 | 0.275 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.0166 | - | - | - |
| data TLB read misses | - | - | - | 0.00781 | 0.0381 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.361M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.361M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 354798 | 9994 | 0 | 0 | 7805556 | 6590826 | 0 | 0.3 | 0.4 | 0.6 | 52.7 | 19005.8 | 0 | 0 | 22/22 | 810.6 | 34.9 | - | 40.91 / 34.16 |
| best | wake | 22 × busy | 354798 | 9994 | 0 | 0 | 7805556 | 6590826 | 0 | 0.3 | 0.4 | 0.6 | 52.7 | 19005.8 | 0 | 0 | 22/22 | 810.6 | 34.9 | - | 40.91 / 34.16 |

Run 1's profile:

| profile: the window, 30.0 s, 299573 messages, 6590607 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 687.5G | 2.295M | 104.3k | 99.86k | 99.78k | - | - | - |
| waiting for a CPU, ns | - | - | - | 56.74 | 70.95 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 30.00 | 0.000100 | 0.00000455 | 0.000103 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 4305 | 0.0144 | 0.000653 | 0.000257 | 0.000642 | - | - | - |
| cycles | - | - | - | 357.9k | 382.6k | - | - | - |
| instructions | - | - | - | 371.9k | 111.6k | - | - | - |
| cache references | - | - | - | 3.475 | 338.2 | - | - | - |
| cache misses | - | - | - | 0.0213 | 0.0250 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.000811 | - | - | - |
| data TLB read misses | - | - | - | 0.000277 | 0.00451 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.754M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.754M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × busy | 3550038 | 99999 | 0 | 0 | 3550038 | 3000437 | 0 | 0.2 | 0.2 | 0.7 | 8.7 | 482.6 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 6.39 / 5.40 |
| best | wake | 1 × busy | 3550038 | 99999 | 0 | 0 | 3550038 | 3000437 | 0 | 0.2 | 0.2 | 0.7 | 8.7 | 482.6 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 6.39 / 5.40 |

Run 1's profile:

| profile: the window, 30.0 s, 3000455 messages, 3000455 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.88G | 19.96k | 19.96k | 9980 | 9978 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.893 | 1.850 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 530.0 | 0.000177 | 0.000177 | 0.0000747 | 0.000102 | - | - | - |
| cycles | - | - | - | 87.35k | 88.48k | - | - | - |
| instructions | - | - | - | 67.39k | 21.57k | - | - | - |
| cache references | - | - | - | 7.934 | 5.304 | - | - | - |
| cache misses | - | - | - | 0.555 | 0.283 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00205 | 0.00135 | - | - | - |
| data TLB read misses | - | - | - | 0.0216 | 0.00223 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.107M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.107M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 3550038 | 100002 | 0 | 0 | 78100836 | 66009614 | 0 | 0.3 | 0.4 | 0.7 | 3.4 | 4401.6 | 0 | 0 | 22/22 | 813.0 | 35.4 | - | 34.75 / 30.71 |
| best | wake | 22 × busy | 3550038 | 100002 | 0 | 0 | 78100836 | 66009614 | 0 | 0.3 | 0.4 | 0.7 | 3.4 | 4401.6 | 0 | 0 | 22/22 | 813.0 | 35.4 | - | 34.75 / 30.71 |

Run 1's profile:

| profile: the window, 30.0 s, 3000386 messages, 66008495 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 687.9G | 229.3k | 10.42k | 9970 | 9968 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.117 | 1.363 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 3605 | 0.00120 | 0.0000546 | 0.0000177 | 0.0000538 | - | - | - |
| cycles | - | - | - | 34.44k | 36.77k | - | - | - |
| instructions | - | - | - | 35.31k | 10.76k | - | - | - |
| cache references | - | - | - | 2.281 | 35.08 | - | - | - |
| cache misses | - | - | - | 0.00484 | 0.00782 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.000152 | - | - | - |
| data TLB read misses | - | - | - | 0.00000400 | 0.000126 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.586M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.586M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 3544764 | 99808 | 5274 | 0 | 77984808 | 66009614 | 0 | 0.3 | 0.5 | 4653.1 | 66584.6 | 182023.2 | 0 | 0 | 22/22 | 787.0 | 34.1 | - | 33.74 / 31.74 |
| best | wake | 22 × busy | 3544764 | 99808 | 5274 | 0 | 77984808 | 66009614 | 0 | 0.3 | 0.5 | 4653.1 | 66584.6 | 182023.2 | 0 | 0 | 22/22 | 787.0 | 34.1 | - | 33.74 / 31.74 |

Run 1's profile:

| profile: the window, 30.0 s, 3000415 messages, 66008938 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 667.1G | 222.3k | 10.11k | 9637 | 9668 | - | - | - |
| waiting for a CPU, ns | - | - | - | 327.8 | 294.0 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 14.62k | 0.00487 | 0.000222 | 0.000233 | 0.000211 | - | - | - |
| cycles | - | - | - | 31.27k | 46.79k | - | - | - |
| instructions | - | - | - | 32.28k | 13.24k | - | - | - |
| cache references | - | - | - | 2.503 | 47.28 | - | - | - |
| cache misses | - | - | - | 0.0269 | 0.0508 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.00309 | - | - | - |
| data TLB read misses | - | - | - | 0.000617 | 0.00344 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.373M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.373M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 7.2 | 95.2 | 884.7 | 1933.3 | 3982.5 | 0 | 0 | 1/1 | 36.7 | 34.9 | - | 28.71 / 35.70 |
| best | wake | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 7.2 | 95.2 | 884.7 | 1933.3 | 3982.5 | 0 | 0 | 1/1 | 36.7 | 34.9 | - | 28.71 / 35.70 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 300768 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 30.92G | 102.8k | 102.8k | 97.72k | 5087 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1532 | 10.52k | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 37.00 | 0.000123 | 0.000123 | 0.000110 | 0.0000133 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 256.6k | 0.853 | 0.853 | 0.00000997 | 0.853 | - | - | - |
| involuntary context switches | 81.63k | 0.271 | 0.271 | 0.271 | 0.000283 | - | - | - |
| cycles | - | - | - | 642.5k | 2394 | - | - | - |
| instructions | - | - | - | 493.8k | 632.2 | - | - | - |
| cache references | - | - | - | 22.29 | 30.45 | - | - | - |
| cache misses | - | - | - | 6.163 | 8.026 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.285 | 0.513 | - | - | - |
| data TLB read misses | - | - | - | 0.269 | 2.592 | - | - | - |
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
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 356101 | 10031 | 0 | 0 | 7834222 | 6616918 | 0 | 11.9 | 31.2 | 13762.6 | 47710.2 | 62973.9 | 0 | 0 | 22/22 | 74.3 | 33.7 | - | 18.77 / 13.69 |
| best | wake | 22 × wait | 356101 | 10031 | 0 | 0 | 7834222 | 6616918 | 0 | 11.9 | 31.2 | 13762.6 | 47710.2 | 62973.9 | 0 | 0 | 22/22 | 74.3 | 33.7 | - | 18.77 / 13.69 |

Run 1's profile:

| profile: the window, 30.0 s, 300776 messages, 6617078 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 63.69G | 211.8k | 9625 | 96.38k | 5244 | - | - | - |
| waiting for a CPU, ns | - | - | - | 2969 | 2233 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 30.00 | 0.0000997 | 0.00000453 | 0.0000997 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 6.399M | 21.28 | 0.967 | 0 | 0.967 | - | - | - |
| involuntary context switches | 90.00k | 0.299 | 0.0136 | 0.294 | 0.000265 | - | - | - |
| cycles | - | - | - | 583.9k | 1895 | - | - | - |
| instructions | - | - | - | 442.8k | 668.4 | - | - | - |
| cache references | - | - | - | 50.63 | 17.82 | - | - | - |
| cache misses | - | - | - | 2.189 | 0.948 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0345 | 0.0738 | - | - | - |
| data TLB read misses | - | - | - | 0.0301 | 1.872 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.365M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.365M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 356101 | 10031 | 0 | 0 | 7834222 | 6616918 | 0 | 11.3 | 23.8 | 51.2 | 344.1 | 2958.5 | 0 | 0 | 22/22 | 76.5 | 35.3 | - | 22.29 / 17.15 |
| best | wake | 22 × wait | 356101 | 10031 | 0 | 0 | 7834222 | 6616918 | 0 | 11.3 | 23.8 | 51.2 | 344.1 | 2958.5 | 0 | 0 | 22/22 | 76.5 | 35.3 | - | 22.29 / 17.15 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 6616896 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 64.70G | 215.1k | 9778 | 99.31k | 5264 | - | - | - |
| waiting for a CPU, ns | - | - | - | 18.55 | 205.2 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 30.00 | 0.0000997 | 0.00000453 | 0.0000997 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 6.554M | 21.79 | 0.991 | 0 | 0.991 | - | - | - |
| involuntary context switches | 2675 | 0.00889 | 0.000404 | 0.00229 | 0.000300 | - | - | - |
| cycles | - | - | - | 618.9k | 1877 | - | - | - |
| instructions | - | - | - | 471.5k | 664.7 | - | - | - |
| cache references | - | - | - | 49.33 | 19.42 | - | - | - |
| cache misses | - | - | - | 0.776 | 0.705 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0387 | 0.0408 | - | - | - |
| data TLB read misses | - | - | - | 0.0254 | 1.936 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.578M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.578M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × wait | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 4.0 | 12.0 | 393.2 | 1949.7 | 3606.1 | 0 | 0 | 1/1 | 43.1 | 35.3 | - | 7.41 / 13.01 |
| best | wake | 1 × wait | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 4.0 | 12.0 | 393.2 | 1949.7 | 3606.1 | 0 | 0 | 1/1 | 43.1 | 35.3 | - | 7.41 / 13.01 |

Run 1's profile:

| profile: the window, 30.0 s, 3002637 messages, 3002637 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 36.28G | 12.08k | 12.08k | 9937 | 2147 | - | - | - |
| waiting for a CPU, ns | - | - | - | 24.26 | 806.2 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 1.891M | 0.630 | 0.630 | 0 | 0.630 | - | - | - |
| involuntary context switches | 8793 | 0.00293 | 0.00293 | 0.00286 | 0.0000713 | - | - | - |
| cycles | - | - | - | 78.58k | 1271 | - | - | - |
| instructions | - | - | - | 58.78k | 613.5 | - | - | - |
| cache references | - | - | - | 7.085 | 11.82 | - | - | - |
| cache misses | - | - | - | 0.480 | 0.610 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00275 | 0.0276 | - | - | - |
| data TLB read misses | - | - | - | 0.00363 | 0.324 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 5.923M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 5.923M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 2532403 | 71271 | 1021615 | 0 | 55712866 | 44684354 | 21373528 | 7247757.3 | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 22/22 | 326.5 | 35.4 | - | 16.51 / 15.15 |
| best | wake | 22 × wait | 2532403 | 71271 | 1021615 | 0 | 55712866 | 44684354 | 21373528 | 7247757.3 | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 22/22 | 326.5 | 35.4 | - | 16.51 / 15.15 |

Run 1's profile:

| profile: the window, 30.0 s, 2163355 messages, 47593800 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 277.6G | 128.3k | 5833 | 13.83k | 5205 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.819 | 10.27 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 47.56M | 21.99 | 0.999 | 0 | 0.999 | - | - | - |
| involuntary context switches | 1343 | 0.000621 | 0.0000282 | 0.0000601 | 0.0000255 | - | - | - |
| cycles | - | - | - | 10.74k | 2184 | - | - | - |
| instructions | - | - | - | 2115 | 734.5 | - | - | - |
| cache references | - | - | - | 49.88 | 21.65 | - | - | - |
| cache misses | - | - | - | 0.126 | 0.117 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00221 | 0.00359 | - | - | - |
| data TLB read misses | - | - | - | 0.00139 | 0.426 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.569M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.569M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 1915249 | 53811 | 1638769 | 0 | 42135478 | 31106966 | 34950916 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 22/22 | 267.4 | 34.9 | - | 19.44 / 19.18 |
| best | wake | 22 × wait | 1915249 | 53811 | 1638769 | 0 | 42135478 | 31106966 | 34950916 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 22/22 | 267.4 | 34.9 | - | 19.44 / 19.18 |

Run 1's profile:

| profile: the window, 30.0 s, 1609137 messages, 35401004 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 224.1G | 139.3k | 6331 | 18.23k | 5502 | - | - | - |
| waiting for a CPU, ns | - | - | - | 343.8 | 213.9 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 35.33M | 21.96 | 0.998 | 0 | 0.998 | - | - | - |
| involuntary context switches | 93.90k | 0.0584 | 0.00265 | 0.0575 | 0.0000395 | - | - | - |
| cycles | - | - | - | 9789 | 1956 | - | - | - |
| instructions | - | - | - | 1912 | 676.6 | - | - | - |
| cache references | - | - | - | 49.19 | 20.39 | - | - | - |
| cache misses | - | - | - | 0.902 | 0.320 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00340 | 0.0249 | - | - | - |
| data TLB read misses | - | - | - | 0.000653 | 0.498 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.451M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.451M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 0.1 | 0.2 | 0.4 | 2.6 | 767.2 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 9.24 / 6.24 |
| best | wake | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 0.1 | 0.2 | 0.4 | 2.6 | 767.2 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 9.24 / 6.24 |

Run 1's profile:

| profile: the window, 30.0 s, 300766 messages, 300766 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.90G | 199.2k | 199.2k | 99.57k | 99.59k | - | - | - |
| waiting for a CPU, ns | - | - | - | 30.46 | 9.878 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 30.00 | 0.0000997 | 0.0000997 | 0.0000997 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 513.0 | 0.00171 | 0.00171 | 0.00105 | 0.000658 | - | - | - |
| cycles | - | - | - | 938.1k | 534.5k | - | - | - |
| instructions | - | - | - | 727.4k | 78.11k | - | - | - |
| cache references | - | - | - | 22.17 | 3.288 | - | - | - |
| cache misses | - | - | - | 7.233 | 0.131 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00804 | 0.00480 | - | - | - |
| data TLB read misses | - | - | - | 0.456 | 0.00328 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 5.837M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 5.837M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 356101 | 10031 | 0 | 0 | 7834222 | 6616918 | 0 | 0.3 | 0.4 | 0.6 | 3.4 | 4563.2 | 0 | 0 | 22/22 | 809.5 | 35.2 | - | 30.15 / 28.25 |
| best | wake | 22 × busy | 356101 | 10031 | 0 | 0 | 7834222 | 6616918 | 0 | 0.3 | 0.4 | 0.6 | 3.4 | 4563.2 | 0 | 0 | 22/22 | 809.5 | 35.2 | - | 30.15 / 28.25 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 6616896 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 687.8G | 2.287M | 103.9k | 99.45k | 99.42k | - | - | - |
| waiting for a CPU, ns | - | - | - | 3.843 | 15.71 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 30.00 | 0.0000997 | 0.00000453 | 0.0000997 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 4252 | 0.0141 | 0.000643 | 0.000186 | 0.000634 | - | - | - |
| cycles | - | - | - | 334.2k | 356.0k | - | - | - |
| instructions | - | - | - | 352.6k | 104.4k | - | - | - |
| cache references | - | - | - | 3.554 | 302.8 | - | - | - |
| cache misses | - | - | - | 0.0679 | 0.0614 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.00220 | - | - | - |
| data TLB read misses | - | - | - | 0.000289 | 0.00135 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.680M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.680M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 356101 | 10031 | 0 | 0 | 7834222 | 6616918 | 0 | 0.3 | 282.6 | 2883.6 | 5374.0 | 7968.2 | 0 | 0 | 22/22 | 777.3 | 32.5 | - | 39.91 / 33.48 |
| best | wake | 22 × busy | 356101 | 10031 | 0 | 0 | 7834222 | 6616918 | 0 | 0.3 | 282.6 | 2883.6 | 5374.0 | 7968.2 | 0 | 0 | 22/22 | 777.3 | 32.5 | - | 39.91 / 33.48 |

Run 1's profile:

| profile: the window, 30.0 s, 300763 messages, 6616860 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 661.5G | 2.199M | 99.97k | 92.00k | 95.79k | - | - | - |
| waiting for a CPU, ns | - | - | - | 7511 | 3720 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 30.00 | 0.0000997 | 0.00000453 | 0.0000997 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 12.04k | 0.0400 | 0.00182 | 0.00286 | 0.00169 | - | - | - |
| cycles | - | - | - | 724.7k | 534.8k | - | - | - |
| instructions | - | - | - | 548.2k | 144.2k | - | - | - |
| cache references | - | - | - | 8.437 | 550.5 | - | - | - |
| cache misses | - | - | - | 0.0342 | 0.0470 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00311 | 0.00140 | - | - | - |
| data TLB read misses | - | - | - | 0.00209 | 0.0519 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.438M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.438M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × busy | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 0.2 | 0.2 | 0.3 | 2.2 | 458.0 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 16.89 / 13.81 |
| best | wake | 1 × busy | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 0.2 | 0.2 | 0.3 | 2.2 | 458.0 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 16.89 / 13.81 |

Run 1's profile:

| profile: the window, 30.0 s, 3002634 messages, 3002634 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.86G | 19.94k | 19.94k | 9969 | 9967 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.794 | 1.133 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 281.0 | 0.0000936 | 0.0000936 | 0.0000450 | 0.0000486 | - | - | - |
| cycles | - | - | - | 77.60k | 77.96k | - | - | - |
| instructions | - | - | - | 61.11k | 19.69k | - | - | - |
| cache references | - | - | - | 6.896 | 4.623 | - | - | - |
| cache misses | - | - | - | 0.151 | 0.152 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00277 | 0.00119 | - | - | - |
| data TLB read misses | - | - | - | 0.00125 | 0.00149 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.263M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.263M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 3554018 | 100114 | 0 | 0 | 78188396 | 66057882 | 0 | 0.3 | 0.5 | 2883.6 | 77594.6 | 179224.6 | 0 | 0 | 22/22 | 793.0 | 34.5 | - | 47.15 / 39.71 |
| best | wake | 22 × busy | 3554018 | 100114 | 0 | 0 | 78188396 | 66057882 | 0 | 0.3 | 0.5 | 2883.6 | 77594.6 | 179224.6 | 0 | 0 | 22/22 | 793.0 | 34.5 | - | 47.15 / 39.71 |

Run 1's profile:

| profile: the window, 30.0 s, 3002594 messages, 66057073 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 674.0G | 224.5k | 10.20k | 9784 | 9759 | - | - | - |
| waiting for a CPU, ns | - | - | - | 180.4 | 200.2 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 13.04k | 0.00434 | 0.000197 | 0.000118 | 0.000192 | - | - | - |
| cycles | - | - | - | 65.53k | 45.59k | - | - | - |
| instructions | - | - | - | 50.57k | 12.57k | - | - | - |
| cache references | - | - | - | 6.127 | 43.55 | - | - | - |
| cache misses | - | - | - | 0.0734 | 0.0178 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00431 | 0.000561 | - | - | - |
| data TLB read misses | - | - | - | 0.00311 | 0.00104 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.578M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.578M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 3554018 | 100114 | 0 | 0 | 78188396 | 66057882 | 0 | 0.3 | 0.5 | 100.4 | 14811.1 | 114374.0 | 0 | 0 | 22/22 | 805.0 | 35.2 | - | 37.63 / 32.80 |
| best | wake | 22 × busy | 3554018 | 100114 | 0 | 0 | 78188396 | 66057882 | 0 | 0.3 | 0.5 | 100.4 | 14811.1 | 114374.0 | 0 | 0 | 22/22 | 805.0 | 35.2 | - | 37.63 / 32.80 |

Run 1's profile:

| profile: the window, 30.0 s, 3002637 messages, 66058017 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 682.6G | 227.3k | 10.33k | 9921 | 9883 | - | - | - |
| waiting for a CPU, ns | - | - | - | 38.21 | 82.10 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 12.44k | 0.00414 | 0.000188 | 0.000978 | 0.000144 | - | - | - |
| cycles | - | - | - | 40.14k | 41.24k | - | - | - |
| instructions | - | - | - | 21.29k | 12.21k | - | - | - |
| cache references | - | - | - | 2.827 | 38.61 | - | - | - |
| cache misses | - | - | - | 0.0179 | 0.0203 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00264 | 0.000641 | - | - | - |
| data TLB read misses | - | - | - | 0.000820 | 0.00151 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.684M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.684M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
