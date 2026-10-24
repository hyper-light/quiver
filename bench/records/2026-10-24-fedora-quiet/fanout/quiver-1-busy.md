# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 0.4 | 0.5 | 1.7 | 8.1 | 172.0 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 0.63 / 1.24 |
| 2 | quiver | 1 × busy | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 0.4 | 0.4 | 1.7 | 9.0 | 478.0 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 1.24 / 1.75 |
| 3 | quiver | 1 × busy | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 0.4 | 0.4 | 1.7 | 8.6 | 173.6 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 1.75 / 1.92 |
| best | quiver | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 0.4 | 0.5 | 1.7 | 8.1 | 172.0 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 0.63 / 1.24 |

Run 1's profile:

| profile: the window, 30.0 s, 2998274 messages, 2998274 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.88G | 19.97k | 19.97k | 9986 | 9985 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.831 | 1.186 | - | - | - |
| allocations | 6.026M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.026M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.448G | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 11.00 | 0.00000367 | 0.00000367 | 0.00000367 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 494.0 | 0.000165 | 0.000165 | 0.0000814 | 0.0000834 | - | - | - |
| cycles | - | - | - | 88.46k | 51.47k | - | - | - |
| instructions | - | - | - | 69.44k | 281.3k | - | - | - |
| cache references | - | - | - | 42.12 | 13.76 | - | - | - |
| cache misses | - | - | - | 12.33 | 0.119 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.114 | 0.000600 | - | - | - |
| data TLB read misses | - | - | - | 0.135 | 0.0303 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.92M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.92M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message

Run 2's profile:

| profile: the window, 30.0 s, 3000414 messages, 3000414 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.88G | 19.96k | 19.96k | 9978 | 9980 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.754 | 0.615 | - | - | - |
| allocations | 6.030M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.030M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.449G | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 11.00 | 0.00000367 | 0.00000367 | 0.00000367 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 490.0 | 0.000163 | 0.000163 | 0.000112 | 0.0000513 | - | - | - |
| cycles | - | - | - | 51.48k | 51.49k | - | - | - |
| instructions | - | - | - | 31.47k | 281.3k | - | - | - |
| cache references | - | - | - | 20.55 | 13.73 | - | - | - |
| cache misses | - | - | - | 4.992 | 0.0976 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.105 | 0.000520 | - | - | - |
| data TLB read misses | - | - | - | 0.0237 | 0.0310 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 36.28M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 36.28M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message

Run 3's profile:

| profile: the window, 30.0 s, 3002646 messages, 3002646 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.89G | 19.95k | 19.95k | 9972 | 9974 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.911 | 0.920 | - | - | - |
| allocations | 6.034M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.034M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.450G | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 17.00 | 0.00000566 | 0.00000566 | 0.00000566 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 499.0 | 0.000166 | 0.000166 | 0.0000739 | 0.0000923 | - | - | - |
| cycles | - | - | - | 92.40k | 92.39k | - | - | - |
| instructions | - | - | - | 75.53k | 447.6k | - | - | - |
| cache references | - | - | - | 44.94 | 21.87 | - | - | - |
| cache misses | - | - | - | 11.25 | 0.211 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.106 | 0.000397 | - | - | - |
| data TLB read misses | - | - | - | 0.0959 | 0.0856 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 36.36M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 36.36M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
