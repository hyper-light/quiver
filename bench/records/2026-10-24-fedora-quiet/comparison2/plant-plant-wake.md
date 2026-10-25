# quiver-bench fanout wake

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 16 over 4 threads, which poll without waiting; window 3 s after 1 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 4 × busy | 45029 | 10005 | 0 | 0 | 180116 | 120132 | 0 | 0.2 | 0.2 | 0.3 | 2.0 | 80.5 | 0 | 0 | 4/4 | 22.5 | 4.5 | - | 2.19 / 2.50 |
| best | wake | 4 × busy | 45029 | 10005 | 0 | 0 | 180116 | 120132 | 0 | 0.2 | 0.2 | 0.3 | 2.0 | 80.5 | 0 | 0 | 4/4 | 22.5 | 4.5 | - | 2.19 / 2.50 |

Run 1's profile:

| profile: the window, 3.0 s, 30032 messages, 120128 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 14.98G | 498.6k | 124.7k | 99.73k | 99.73k | - | - | - |
| waiting for a CPU, ns | - | - | - | 2.377 | 8.230 | - | - | - |
| allocations | 120.1k | 4.000 | 1.000 | 0 | 1.000 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 120.1k | 4.000 | 1.000 | 0 | 1.000 | - | - | - |
| bytes allocated | 961.0k | 32.00 | 8.000 | 0 | 8.000 | - | - | - |
| minor page faults | 59.00 | 0.00196 | 0.000491 | 0.00196 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 98.00 | 0.00326 | 0.000816 | 0.000266 | 0.000749 | - | - | - |
| cycles | - | - | - | 507.2k | 682.7k | - | - | - |
| instructions | - | - | - | 291.5k | 170.7k | - | - | - |
| cache references | - | - | - | 3.247 | 2108 | - | - | - |
| cache misses | - | - | - | 0.0456 | 4.188 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00906 | 0.00629 | - | - | - |
| data TLB read misses | - | - | - | 0.000200 | 0.425 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.214M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.214M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
