# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × wait | 3547543 | 99929 | 0 | 0 | 3547543 | 2998274 | 0 | 3.9 | 4.4 | 6.7 | 56.3 | 1009.6 | 0 | 0 | 1/1 | 42.9 | 35.4 | - | 0.53 / 1.11 |
| 2 | quiver | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 3.0 | 4.4 | 6.7 | 56.8 | 389.3 | 0 | 0 | 1/1 | 42.7 | 35.4 | - | 1.11 / 1.16 |
| 3 | quiver | 1 × wait | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 4.0 | 4.5 | 6.7 | 52.7 | 1017.5 | 0 | 0 | 1/1 | 43.4 | 35.4 | - | 1.16 / 1.14 |
| best | quiver | 1 × wait | 3547543 | 99929 | 0 | 0 | 3547543 | 2998274 | 0 | 3.9 | 4.4 | 6.7 | 56.3 | 1009.6 | 0 | 0 | 1/1 | 42.9 | 35.4 | - | 0.53 / 1.11 |

Run 1's profile:

| profile: the window, 30.0 s, 2998275 messages, 2998275 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 36.31G | 12.11k | 12.11k | 9991 | 2119 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.078 | 1.297 | - | - | - |
| allocations | 6.026M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.026M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.448G | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 11.00 | 0.00000367 | 0.00000367 | 0.00000367 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 2.161M | 0.721 | 0.721 | 0 | 0.721 | - | - | - |
| involuntary context switches | 360.0 | 0.000120 | 0.000120 | 0.000114 | 0.00000634 | - | - | - |
| cycles | - | - | - | 93.18k | 2554 | - | - | - |
| instructions | - | - | - | 71.95k | 1613 | - | - | - |
| cache references | - | - | - | 54.08 | 31.90 | - | - | - |
| cache misses | - | - | - | 11.28 | 0.141 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.126 | 0.00100 | - | - | - |
| data TLB read misses | - | - | - | 0.548 | 0.172 | - | - | - |
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

Run 2's profile:

| profile: the window, 30.0 s, 3000434 messages, 3000434 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 36.00G | 12.00k | 12.00k | 9984 | 2015 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.528 | 0.843 | - | - | - |
| allocations | 6.030M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.030M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.449G | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 41.00 | 0.0000137 | 0.0000137 | 0.0000137 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 2.194M | 0.731 | 0.731 | 0 | 0.731 | - | - | - |
| involuntary context switches | 414.0 | 0.000138 | 0.000138 | 0.000133 | 0.00000533 | - | - | - |
| cycles | - | - | - | 55.24k | 2347 | - | - | - |
| instructions | - | - | - | 33.00k | 1529 | - | - | - |
| cache references | - | - | - | 20.18 | 30.42 | - | - | - |
| cache misses | - | - | - | 5.257 | 0.124 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.117 | 0.000782 | - | - | - |
| data TLB read misses | - | - | - | 0.0244 | 0.207 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 36.16M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 36.16M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message

Run 3's profile:

| profile: the window, 30.0 s, 3002638 messages, 3002638 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 36.59G | 12.19k | 12.19k | 9976 | 2211 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.581 | 1.464 | - | - | - |
| allocations | 6.034M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.034M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.450G | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 24.00 | 0.00000799 | 0.00000799 | 0.00000799 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 2.130M | 0.709 | 0.709 | 0 | 0.709 | - | - | - |
| involuntary context switches | 422.0 | 0.000141 | 0.000141 | 0.000137 | 0.00000333 | - | - | - |
| cycles | - | - | - | 94.20k | 2456 | - | - | - |
| instructions | - | - | - | 73.30k | 1680 | - | - | - |
| cache references | - | - | - | 41.82 | 27.14 | - | - | - |
| cache misses | - | - | - | 11.74 | 0.139 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.133 | 0.00102 | - | - | - |
| data TLB read misses | - | - | - | 0.258 | 0.203 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 36.21M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 36.21M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
