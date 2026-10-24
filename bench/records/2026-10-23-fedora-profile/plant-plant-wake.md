# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 16 over 4 threads, which poll without waiting; window 3 s after 1 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 4 × busy | 45029 | 10005 | 0 | 0 | 180116 | 120132 | 0 | 0.2 | 0.3 | 0.5 | 9.7 | 289.0 | 0 | 0 | 4/4 | 22.4 | 4.5 | - | 17.83 / 18.01 |
| best | wake | 4 × busy | 45029 | 10005 | 0 | 0 | 180116 | 120132 | 0 | 0.2 | 0.3 | 0.5 | 9.7 | 289.0 | 0 | 0 | 4/4 | 22.4 | 4.5 | - | 17.83 / 18.01 |

Run 1's profile:

| profile: the window, 3.0 s, 30030 messages, 120120 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 14.95G | 497.9k | 124.5k | 99.59k | 99.58k | - | - | - |
| waiting for a CPU, ns | - | - | - | 6.748 | 19.94 | - | - | - |
| allocations | 120.1k | 4.000 | 1.000 | 0 | 1.000 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 120.1k | 4.000 | 1.000 | 0 | 1.000 | - | - | - |
| bytes allocated | 961.0k | 32.00 | 8.000 | 0 | 8.000 | - | - | - |
| minor page faults | 59.00 | 0.00196 | 0.000491 | 0.00196 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 89.00 | 0.00296 | 0.000741 | 0.000599 | 0.000591 | - | - | - |
| cycles | - | - | - | 746.8k | 558.4k | - | - | - |
| instructions | - | - | - | 574.3k | 141.0k | - | - | - |
| cache references | - | - | - | 12.11 | 1721 | - | - | - |
| cache misses | - | - | - | 2.933 | 1.011 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.186 | 0.0567 | - | - | - |
| data TLB read misses | - | - | - | 0.0549 | 0.0234 | - | - | - |
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
