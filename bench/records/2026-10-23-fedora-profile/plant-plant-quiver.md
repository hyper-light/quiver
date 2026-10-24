# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 16 over 4 threads, which poll without waiting; window 3 s after 1 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 4 × busy | 45029 | 10005 | 0 | 0 | 720464 | 480528 | 0 | 1.2 | 2.2 | 5.7 | 18.7 | 242.4 | 0 | 0 | 16/16 | 22.4 | 4.5 | - | 17.73 / 17.83 |
| best | quiver | 4 × busy | 45029 | 10005 | 0 | 0 | 720464 | 480528 | 0 | 1.2 | 2.2 | 5.7 | 18.7 | 242.4 | 0 | 0 | 16/16 | 22.4 | 4.5 | - | 17.73 / 17.83 |

Run 1's profile:

| profile: the window, 3.0 s, 30032 messages, 480512 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 14.95G | 497.9k | 31.12k | 99.57k | 24.89k | - | - | - |
| waiting for a CPU, ns | - | - | - | 3.785 | 1.538 | - | - | - |
| allocations | 600.9k | 20.01 | 1.251 | 4.010 | 1.000 | - | - | - |
| reallocations | 90.10k | 3.000 | 0.188 | 3.000 | 0 | - | - | - |
| reallocations that moved their block | 60.48k | 2.014 | 0.126 | 2.014 | 0 | - | - | - |
| frees | 570.6k | 19.00 | 1.188 | 3.000 | 1.000 | - | - | - |
| bytes allocated | 34.63M | 1153 | 72.06 | 1025 | 8.000 | - | - | - |
| minor page faults | 3339 | 0.111 | 0.00695 | 0.111 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 2.000 | 0.0000666 | 0.00000416 | 0.0000666 | 0 | - | - | - |
| involuntary context switches | 73.00 | 0.00243 | 0.000152 | 0.000333 | 0.000131 | - | - | - |
| cycles | - | - | - | 336.4k | 114.0k | - | - | - |
| instructions | - | - | - | 359.6k | 127.1k | - | - | - |
| cache references | - | - | - | 26.70 | 198.3 | - | - | - |
| cache misses | - | - | - | 6.744 | 0.424 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.00939 | - | - | - |
| data TLB read misses | - | - | - | 0.0538 | 0.0152 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 24.23M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 24.23M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 12.65M bytes, holding 40044 messages: 316.0 bytes a message
