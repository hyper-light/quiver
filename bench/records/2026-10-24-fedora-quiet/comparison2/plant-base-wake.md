# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 16 over 4 threads, which poll without waiting; window 3 s after 1 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 4 × busy | 45029 | 10005 | 0 | 0 | 180116 | 120132 | 0 | 0.2 | 0.2 | 0.3 | 2.2 | 51.6 | 0 | 0 | 4/4 | 22.5 | 4.5 | - | 1.59 / 1.95 |
| best | wake | 4 × busy | 45029 | 10005 | 0 | 0 | 180116 | 120132 | 0 | 0.2 | 0.2 | 0.3 | 2.2 | 51.6 | 0 | 0 | 4/4 | 22.5 | 4.5 | - | 1.59 / 1.95 |

Run 1's profile:

| profile: the window, 3.0 s, 30033 messages, 120132 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 14.97G | 498.6k | 124.6k | 99.73k | 99.71k | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.585 | 5.019 | - | - | - |
| allocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 0 | 0 | 0 | 0 | 0 | - | - | - |
| bytes allocated | 0 | 0 | 0 | 0 | 0 | - | - | - |
| minor page faults | 59.00 | 0.00196 | 0.000491 | 0.00196 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 59.00 | 0.00196 | 0.000491 | 0.000166 | 0.000450 | - | - | - |
| cycles | - | - | - | 504.5k | 579.4k | - | - | - |
| instructions | - | - | - | 288.9k | 131.7k | - | - | - |
| cache references | - | - | - | 3.263 | 1663 | - | - | - |
| cache misses | - | - | - | 0.0911 | 3.448 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0171 | 0.00537 | - | - | - |
| data TLB read misses | - | - | - | 0.000166 | 0.311 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.078M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.078M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
