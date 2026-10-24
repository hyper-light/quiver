# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 16 over 4 threads, which poll without waiting; window 3 s after 1 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 4 × busy | 45029 | 10005 | 0 | 0 | 180116 | 120132 | 0 | 0.2 | 0.2 | 0.3 | 1.8 | 106.5 | 0 | 0 | 4/4 | 22.5 | 4.5 | - | 2.35 / 2.56 |
| best | wake | 4 × busy | 45029 | 10005 | 0 | 0 | 180116 | 120132 | 0 | 0.2 | 0.2 | 0.3 | 1.8 | 106.5 | 0 | 0 | 4/4 | 22.5 | 4.5 | - | 2.35 / 2.56 |

Run 1's profile:

| profile: the window, 3.0 s, 30030 messages, 120120 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 14.98G | 498.7k | 124.7k | 99.76k | 99.73k | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.657 | 16.77 | - | - | - |
| allocations | 120.1k | 4.000 | 1.000 | 0 | 1.000 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 120.1k | 4.000 | 1.000 | 0 | 1.000 | - | - | - |
| bytes allocated | 961.0k | 32.00 | 8.000 | 0 | 8.000 | - | - | - |
| minor page faults | 59.00 | 0.00196 | 0.000491 | 0.00196 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 140.0 | 0.00466 | 0.00117 | 0.000200 | 0.00112 | - | - | - |
| cycles | - | - | - | 505.0k | 479.7k | - | - | - |
| instructions | - | - | - | 291.5k | 93.90k | - | - | - |
| cache references | - | - | - | 3.170 | 1309 | - | - | - |
| cache misses | - | - | - | 0.0131 | 0.0120 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00110 | 0.00136 | - | - | - |
| data TLB read misses | - | - | - | 0.00107 | 0.00285 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.250M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.250M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
