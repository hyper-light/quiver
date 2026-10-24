# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 16 over 4 threads, which poll without waiting; window 3 s after 1 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 4 × busy | 45029 | 10005 | 0 | 0 | 720464 | 480528 | 0 | 0.6 | 0.9 | 2.3 | 7.2 | 68.3 | 0 | 0 | 16/16 | 22.5 | 4.5 | - | 1.49 / 1.78 |
| best | quiver | 4 × busy | 45029 | 10005 | 0 | 0 | 720464 | 480528 | 0 | 0.6 | 0.9 | 2.3 | 7.2 | 68.3 | 0 | 0 | 16/16 | 22.5 | 4.5 | - | 1.49 / 1.78 |

Run 1's profile:

| profile: the window, 3.0 s, 30033 messages, 480528 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 14.97G | 498.5k | 31.16k | 99.71k | 24.93k | - | - | - |
| waiting for a CPU, ns | - | - | - | 2.797 | 1.740 | - | - | - |
| allocations | 60.36k | 2.010 | 0.126 | 2.010 | 0 | - | - | - |
| reallocations | 2.000 | 0.0000666 | 0.00000416 | 0.0000666 | 0 | - | - | - |
| reallocations that moved their block | 2.000 | 0.0000666 | 0.00000416 | 0.0000666 | 0 | - | - | - |
| frees | 30.03k | 1.000 | 0.0625 | 1.000 | 0 | - | - | - |
| bytes allocated | 14.51M | 483.0 | 30.19 | 483.0 | 0 | - | - | - |
| minor page faults | 3337 | 0.111 | 0.00694 | 0.111 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 92.00 | 0.00306 | 0.000191 | 0.000400 | 0.000166 | - | - | - |
| cycles | - | - | - | 503.0k | 144.9k | - | - | - |
| instructions | - | - | - | 289.5k | 339.0k | - | - | - |
| cache references | - | - | - | 25.25 | 440.5 | - | - | - |
| cache misses | - | - | - | 0.125 | 0.0176 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00972 | 0.000403 | - | - | - |
| data TLB read misses | - | - | - | 0.0732 | 0.0240 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 24.18M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 24.18M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 12.65M bytes, holding 40044 messages: 316.0 bytes a message
