# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 16 over 4 threads, which poll without waiting; window 3 s after 1 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 4 × busy | 45029 | 10005 | 0 | 0 | 720464 | 480528 | 0 | 0.6 | 0.9 | 2.7 | 8.3 | 183.3 | 0 | 0 | 16/16 | 22.5 | 4.5 | - | 1.95 / 2.19 |
| best | quiver | 4 × busy | 45029 | 10005 | 0 | 0 | 720464 | 480528 | 0 | 0.6 | 0.9 | 2.7 | 8.3 | 183.3 | 0 | 0 | 16/16 | 22.5 | 4.5 | - | 1.95 / 2.19 |

Run 1's profile:

| profile: the window, 3.0 s, 30031 messages, 480496 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 14.97G | 498.5k | 31.15k | 99.72k | 24.92k | - | - | - |
| waiting for a CPU, ns | - | - | - | 6.574 | 14.11 | - | - | - |
| allocations | 540.8k | 18.01 | 1.126 | 2.010 | 1.000 | - | - | - |
| reallocations | 2.000 | 0.0000666 | 0.00000416 | 0.0000666 | 0 | - | - | - |
| reallocations that moved their block | 2.000 | 0.0000666 | 0.00000416 | 0.0000666 | 0 | - | - | - |
| frees | 510.5k | 17.00 | 1.062 | 1.000 | 1.000 | - | - | - |
| bytes allocated | 18.35M | 611.0 | 38.19 | 483.0 | 8.000 | - | - | - |
| minor page faults | 3337 | 0.111 | 0.00694 | 0.111 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 1505 | 0.0501 | 0.00313 | 0.000633 | 0.00309 | - | - | - |
| cycles | - | - | - | 506.3k | 120.4k | - | - | - |
| instructions | - | - | - | 292.2k | 304.8k | - | - | - |
| cache references | - | - | - | 26.98 | 343.6 | - | - | - |
| cache misses | - | - | - | 0.464 | 0.0227 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0572 | 0.00152 | - | - | - |
| data TLB read misses | - | - | - | 0.0697 | 0.0201 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 24.35M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 24.35M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 12.65M bytes, holding 40043 messages: 316.0 bytes a message
