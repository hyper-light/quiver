# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 16 over 4 threads, which poll without waiting; window 3 s after 1 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 4 × busy | 45029 | 10005 | 0 | 0 | 720464 | 480528 | 0 | 1.0 | 1.8 | 4.5 | 22.8 | 617.4 | 0 | 0 | 16/16 | 22.4 | 4.5 | - | 13.62 / 14.22 |
| best | quiver | 4 × busy | 45029 | 10005 | 0 | 0 | 720464 | 480528 | 0 | 1.0 | 1.8 | 4.5 | 22.8 | 617.4 | 0 | 0 | 16/16 | 22.4 | 4.5 | - | 13.62 / 14.22 |

Run 1's profile:

| profile: the window, 3.0 s, 30030 messages, 480480 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 14.95G | 497.9k | 31.12k | 99.58k | 24.89k | - | - | - |
| waiting for a CPU, ns | - | - | - | 35.93 | 14.23 | - | - | - |
| allocations | 120.4k | 4.010 | 0.251 | 4.010 | 0 | - | - | - |
| reallocations | 90.09k | 3.000 | 0.188 | 3.000 | 0 | - | - | - |
| reallocations that moved their block | 60.47k | 2.014 | 0.126 | 2.014 | 0 | - | - | - |
| frees | 90.09k | 3.000 | 0.188 | 3.000 | 0 | - | - | - |
| bytes allocated | 30.78M | 1025 | 64.06 | 1025 | 0 | - | - | - |
| minor page faults | 3337 | 0.111 | 0.00695 | 0.111 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 256.0 | 0.00852 | 0.000533 | 0.000799 | 0.000483 | - | - | - |
| cycles | - | - | - | 753.9k | 137.1k | - | - | - |
| instructions | - | - | - | 598.8k | 156.4k | - | - | - |
| cache references | - | - | - | 57.87 | 239.7 | - | - | - |
| cache misses | - | - | - | 9.918 | 0.340 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.162 | 0.00612 | - | - | - |
| data TLB read misses | - | - | - | 0.0916 | 0.0153 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 24.25M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 24.25M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 12.65M bytes, holding 40043 messages: 316.0 bytes a message
