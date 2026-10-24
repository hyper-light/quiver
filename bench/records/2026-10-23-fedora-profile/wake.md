# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 10 over 10 threads, which wait when there's nothing to take; window 10 s after 2 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 10 × wait | 124861 | 9989 | 0 | 0 | 1248610 | 998040 | 0 | 7.6 | 15.4 | 41.5 | 532.5 | 2946.8 | 0 | 0 | 10/10 | 18.7 | 12.5 | - | 18.41 / 18.18 |
| best | wake | 10 × wait | 124861 | 9989 | 0 | 0 | 1248610 | 998040 | 0 | 7.6 | 15.4 | 41.5 | 532.5 | 2946.8 | 0 | 0 | 10/10 | 18.7 | 12.5 | - | 18.41 / 18.18 |

Run 1's profile:

| profile: the window, 10.0 s, 99804 messages, 998030 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 14.96G | 149.9k | 14.99k | 99.82k | 5009 | - | - | - |
| waiting for a CPU, ns | - | - | - | 30.85 | 321.2 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 90.00 | 0.000902 | 0.0000902 | 0.000902 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 939.0k | 9.408 | 0.941 | 0 | 0.941 | - | - | - |
| involuntary context switches | 445.0 | 0.00446 | 0.000446 | 0.00295 | 0.000151 | - | - | - |
| cycles | - | - | - | 689.7k | 1971 | - | - | - |
| instructions | - | - | - | 525.1k | 655.5 | - | - | - |
| cache references | - | - | - | 42.12 | 22.32 | - | - | - |
| cache misses | - | - | - | 3.051 | 2.210 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0449 | 0.210 | - | - | - |
| data TLB read misses | - | - | - | 0.0176 | 2.995 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.586M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.586M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 10 s after 2 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × wait | 124861 | 9989 | 0 | 0 | 2746942 | 2195688 | 0 | 10.8 | 20.2 | 39.9 | 66.6 | 3317.3 | 0 | 0 | 22/22 | 27.0 | 12.4 | - | 16.39 / 15.20 |
| best | wake | 22 × wait | 124861 | 9989 | 0 | 0 | 2746942 | 2195688 | 0 | 10.8 | 20.2 | 39.9 | 66.6 | 3317.3 | 0 | 0 | 22/22 | 27.0 | 12.4 | - | 16.39 / 15.20 |

Run 1's profile:

| profile: the window, 10.0 s, 99803 messages, 2195677 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 21.63G | 216.7k | 9851 | 99.86k | 5312 | - | - | - |
| waiting for a CPU, ns | - | - | - | 7.464 | 54.80 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 90.00 | 0.000902 | 0.0000410 | 0.000902 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 2.185M | 21.89 | 0.995 | 0 | 0.995 | - | - | - |
| involuntary context switches | 335.0 | 0.00336 | 0.000153 | 0.000892 | 0.000112 | - | - | - |
| cycles | - | - | - | 641.2k | 1996 | - | - | - |
| instructions | - | - | - | 496.8k | 715.2 | - | - | - |
| cache references | - | - | - | 47.89 | 21.79 | - | - | - |
| cache misses | - | - | - | 0.802 | 0.928 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0122 | 0.0467 | - | - | - |
| data TLB read misses | - | - | - | 0.0303 | 4.212 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.336M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.336M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 10 over 10 threads, which poll without waiting; window 10 s after 2 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 10 × busy | 124861 | 9988 | 0 | 0 | 1248610 | 998040 | 0 | 0.3 | 0.3 | 0.4 | 4.4 | 1669.2 | 0 | 0 | 10/10 | 136.9 | 12.5 | - | 17.34 / 18.21 |
| best | wake | 10 × busy | 124861 | 9988 | 0 | 0 | 1248610 | 998040 | 0 | 0.3 | 0.3 | 0.4 | 4.4 | 1669.2 | 0 | 0 | 10/10 | 136.9 | 12.5 | - | 17.34 / 18.21 |

Run 1's profile:

| profile: the window, 10.0 s, 99804 messages, 998040 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 109.7G | 1.099M | 109.9k | 99.89k | 99.88k | - | - | - |
| waiting for a CPU, ns | - | - | - | 26.31 | 31.04 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 90.00 | 0.000902 | 0.0000902 | 0.000902 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 1784 | 0.0179 | 0.00179 | 0.000501 | 0.00174 | - | - | - |
| cycles | - | - | - | 737.9k | 454.5k | - | - | - |
| instructions | - | - | - | 568.5k | 144.6k | - | - | - |
| cache references | - | - | - | 8.354 | 666.2 | - | - | - |
| cache misses | - | - | - | 0.784 | 0.209 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0423 | 0.00644 | - | - | - |
| data TLB read misses | - | - | - | 0.00364 | 0.00281 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.459M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.459M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 10 s after 2 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 22 × busy | 124861 | 9989 | 0 | 0 | 2746942 | 2195688 | 0 | 0.4 | 2359.3 | 4653.1 | 5898.2 | 12758.0 | 0 | 0 | 22/22 | 236.4 | 10.2 | - | 27.34 / 28.73 |
| best | wake | 22 × busy | 124861 | 9989 | 0 | 0 | 2746942 | 2195688 | 0 | 0.4 | 2359.3 | 4653.1 | 5898.2 | 12758.0 | 0 | 0 | 22/22 | 236.4 | 10.2 | - | 27.34 / 28.73 |

Run 1's profile:

| profile: the window, 10.0 s, 99803 messages, 2195636 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 189.4G | 1.897M | 86.24k | 79.17k | 82.65k | - | - | - |
| waiting for a CPU, ns | - | - | - | 20.79k | 17.29k | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 90.00 | 0.000902 | 0.0000410 | 0.000902 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 15.73k | 0.158 | 0.00717 | 0.00840 | 0.00678 | - | - | - |
| cycles | - | - | - | 257.9k | 357.7k | - | - | - |
| instructions | - | - | - | 258.0k | 106.2k | - | - | - |
| cache references | - | - | - | 4.262 | 395.9 | - | - | - |
| cache misses | - | - | - | 0.0619 | 0.125 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.00544 | - | - | - |
| data TLB read misses | - | - | - | 0.000912 | 0.0560 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.537M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.537M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
