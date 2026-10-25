# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 16 over 4 threads, which poll without waiting; window 3 s after 1 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 4 × busy | 45029 | 10006 | 0 | 0 | 720464 | 480528 | 0 | 0.6 | 0.9 | 2.5 | 7.7 | 86.0 | 0 | 0 | 16/16 | 22.5 | 4.5 | - | 1.21 / 1.59 |
| best | quiver | 4 × busy | 45029 | 10006 | 0 | 0 | 720464 | 480528 | 0 | 0.6 | 0.9 | 2.5 | 7.7 | 86.0 | 0 | 0 | 16/16 | 22.5 | 4.5 | - | 1.21 / 1.59 |

Run 1's profile:

| profile: the window, 3.0 s, 30032 messages, 480512 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 14.98G | 498.8k | 31.17k | 99.71k | 24.93k | - | - | - |
| waiting for a CPU, ns | - | - | - | 2.305 | 1.420 | - | - | - |
| allocations | 60.35k | 2.010 | 0.126 | 2.010 | 0 | - | - | - |
| reallocations | 2.000 | 0.0000666 | 0.00000416 | 0.0000666 | 0 | - | - | - |
| reallocations that moved their block | 2.000 | 0.0000666 | 0.00000416 | 0.0000666 | 0 | - | - | - |
| frees | 30.03k | 1.000 | 0.0625 | 1.000 | 0 | - | - | - |
| bytes allocated | 14.51M | 483.0 | 30.19 | 483.0 | 0 | - | - | - |
| minor page faults | 3337 | 0.111 | 0.00694 | 0.111 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 73.00 | 0.00243 | 0.000152 | 0.000300 | 0.000133 | - | - | - |
| cycles | - | - | - | 506.6k | 120.5k | - | - | - |
| instructions | - | - | - | 291.5k | 304.1k | - | - | - |
| cache references | - | - | - | 27.08 | 396.7 | - | - | - |
| cache misses | - | - | - | 0.443 | 0.0234 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0434 | 0.00176 | - | - | - |
| data TLB read misses | - | - | - | 0.0724 | 0.0229 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 24.33M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 24.33M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 12.65M bytes, holding 40044 messages: 316.0 bytes a message
