# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: step2-working-tree
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 0.4 | 0.5 | 1.8 | 9.9 | 161.1 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 2.45 / 2.33 |
| best | quiver | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 0.4 | 0.5 | 1.8 | 9.9 | 161.1 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 2.45 / 2.33 |

Run 1's profile:

| profile: the window, 30.0 s, 2998274 messages, 2998274 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.88G | 19.97k | 19.97k | 9986 | 9984 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.069 | 1.100 | - | - | - |
| allocations | 6.026M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.026M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.448G | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 6.000 | 0.00000200 | 0.00000200 | 0.00000200 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 615.0 | 0.000205 | 0.000205 | 0.0000924 | 0.000113 | - | - | - |
| cycles | - | - | - | 91.99k | 51.45k | - | - | - |
| instructions | - | - | - | 73.49k | 281.7k | - | - | - |
| cache references | - | - | - | 53.41 | 16.16 | - | - | - |
| cache misses | - | - | - | 11.94 | 0.152 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.125 | 0.000428 | - | - | - |
| data TLB read misses | - | - | - | 0.153 | 0.0309 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.82M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.82M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
