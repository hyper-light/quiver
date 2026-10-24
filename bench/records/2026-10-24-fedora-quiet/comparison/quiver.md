# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 4.3 | 6.5 | 21.2 | 69.6 | 280.9 | 0 | 0 | 1/1 | 36.9 | 35.4 | - | 1.86 / 1.54 |
| best | quiver | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 4.3 | 6.5 | 21.2 | 69.6 | 280.9 | 0 | 0 | 1/1 | 36.9 | 35.4 | - | 1.86 / 1.54 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 299499 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 31.12G | 103.9k | 103.9k | 100.0k | 3881 | - | - | - |
| waiting for a CPU, ns | - | - | - | 15.04 | 4.010 | - | - | - |
| allocations | 601.9k | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.00000334 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.00000334 | 0.00000334 | 0 | - | - | - |
| frees | 586.2k | 1.957 | 1.957 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.1 | 483.1 | 483.1 | 0 | - | - | - |
| minor page faults | 1771 | 0.00591 | 0.00591 | 0.00591 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 284.6k | 0.950 | 0.950 | 0 | 0.950 | - | - | - |
| involuntary context switches | 417.0 | 0.00139 | 0.00139 | 0.00137 | 0.0000200 | - | - | - |
| cycles | - | - | - | 964.6k | 2811 | - | - | - |
| instructions | - | - | - | 750.9k | 1844 | - | - | - |
| cache references | - | - | - | 113.3 | 42.97 | - | - | - |
| cache misses | - | - | - | 22.59 | 0.304 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.141 | 0.00914 | - | - | - |
| data TLB read misses | - | - | - | 2.021 | 0.387 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.60M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.60M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 354590 | 9989 | 0 | 0 | 35459000 | 29949700 | 0 | 10.5 | 21.0 | 42.0 | 99.3 | 2194.9 | 0 | 0 | 100/100 | 94.5 | 35.4 | - | 2.59 / 3.08 |
| best | quiver | 22 × wait | 354590 | 9989 | 0 | 0 | 35459000 | 29949700 | 0 | 10.5 | 21.0 | 42.0 | 99.3 | 2194.9 | 0 | 0 | 100/100 | 94.5 | 35.4 | - | 2.59 / 3.08 |

Run 1's profile:

| profile: the window, 30.0 s, 299498 messages, 29949790 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 79.88G | 266.7k | 2667 | 99.83k | 1669 | - | - | - |
| waiting for a CPU, ns | - | - | - | 13.61 | 2.865 | - | - | - |
| allocations | 601.9k | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | - | - | - |
| frees | 586.2k | 1.957 | 0.0196 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.1 | 4.831 | 483.1 | 0 | - | - | - |
| minor page faults | 1770 | 0.00591 | 0.0000591 | 0.00591 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 8.015M | 26.76 | 0.268 | 0.00755 | 0.268 | - | - | - |
| involuntary context switches | 1763 | 0.00589 | 0.0000589 | 0.00258 | 0.0000330 | - | - | - |
| cycles | - | - | - | 792.9k | 2334 | - | - | - |
| instructions | - | - | - | 603.3k | 1192 | - | - | - |
| cache references | - | - | - | 203.2 | 11.95 | - | - | - |
| cache misses | - | - | - | 13.00 | 0.0430 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.556 | 0.000938 | - | - | - |
| data TLB read misses | - | - | - | 18.63 | 0.340 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.69M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.69M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 354590 | 9988 | 0 | 0 | 354590000 | 299497000 | 0 | 11.4 | 19.2 | 49.2 | 141.3 | 3105.7 | 0 | 0 | 1000/1000 | 109.6 | 35.4 | - | 4.99 / 5.85 |
| best | quiver | 22 × wait | 354590 | 9988 | 0 | 0 | 354590000 | 299497000 | 0 | 11.4 | 19.2 | 49.2 | 141.3 | 3105.7 | 0 | 0 | 1000/1000 | 109.6 | 35.4 | - | 4.99 / 5.85 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 299498627 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 92.70G | 309.5k | 309.5 | 99.86k | 209.7 | - | - | - |
| waiting for a CPU, ns | - | - | - | 8.505 | 0.264 | - | - | - |
| allocations | 601.9k | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.000000003 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.000000003 | 0.00000334 | 0 | - | - | - |
| frees | 586.2k | 1.957 | 0.00196 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.1 | 0.483 | 483.1 | 0 | - | - | - |
| minor page faults | 1770 | 0.00591 | 0.00000591 | 0.00591 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 6.275M | 20.95 | 0.0210 | 0.00409 | 0.0209 | - | - | - |
| involuntary context switches | 1241 | 0.00414 | 0.00000414 | 0.00163 | 0.00000252 | - | - | - |
| cycles | - | - | - | 798.0k | 826.7 | - | - | - |
| instructions | - | - | - | 604.1k | 992.2 | - | - | - |
| cache references | - | - | - | 208.6 | 7.856 | - | - | - |
| cache misses | - | - | - | 12.31 | 0.00494 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.555 | 0.000112 | - | - | - |
| data TLB read misses | - | - | - | 32.34 | 0.0884 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.37M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.37M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 4.0 | 4.5 | 6.9 | 53.8 | 246.6 | 0 | 0 | 1/1 | 43.3 | 35.4 | - | 6.36 / 4.22 |
| best | quiver | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 4.0 | 4.5 | 6.9 | 53.8 | 246.6 | 0 | 0 | 1/1 | 43.3 | 35.4 | - | 6.36 / 4.22 |

Run 1's profile:

| profile: the window, 30.0 s, 2998268 messages, 2998268 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 36.61G | 12.21k | 12.21k | 9991 | 2219 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.122 | 1.087 | - | - | - |
| allocations | 6.026M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.026M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.448G | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 6.000 | 0.00000200 | 0.00000200 | 0.00000200 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 2.122M | 0.708 | 0.708 | 0.000000334 | 0.708 | - | - | - |
| involuntary context switches | 312.0 | 0.000104 | 0.000104 | 0.0000997 | 0.00000434 | - | - | - |
| cycles | - | - | - | 54.85k | 2490 | - | - | - |
| instructions | - | - | - | 33.06k | 1644 | - | - | - |
| cache references | - | - | - | 18.57 | 33.80 | - | - | - |
| cache misses | - | - | - | 5.596 | 0.172 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.141 | 0.000953 | - | - | - |
| data TLB read misses | - | - | - | 0.0264 | 0.230 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.89M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.89M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 3547543 | 99931 | 0 | 0 | 354754300 | 299827400 | 0 | 13.1 | 36.9 | 75.8 | 123.9 | 2993.3 | 0 | 0 | 100/100 | 339.8 | 34.7 | - | 6.24 / 7.93 |
| best | quiver | 22 × wait | 3547543 | 99931 | 0 | 0 | 354754300 | 299827400 | 0 | 13.1 | 36.9 | 75.8 | 123.9 | 2993.3 | 0 | 0 | 100/100 | 339.8 | 34.7 | - | 6.24 / 7.93 |

Run 1's profile:

| profile: the window, 30.0 s, 2998279 messages, 299827648 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 288.5G | 96.23k | 962.3 | 9781 | 864.5 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.662 | 0.756 | - | - | - |
| allocations | 6.026M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.026M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.448G | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 36.00 | 0.0000120 | 0.000000120 | 0.0000120 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 36.58M | 12.20 | 0.122 | 0.0569 | 0.121 | - | - | - |
| involuntary context switches | 7002 | 0.00234 | 0.0000234 | 0.000224 | 0.0000211 | - | - | - |
| cycles | - | - | - | 53.14k | 1579 | - | - | - |
| instructions | - | - | - | 34.65k | 973.0 | - | - | - |
| cache references | - | - | - | 150.9 | 8.025 | - | - | - |
| cache misses | - | - | - | 12.17 | 0.0301 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.483 | 0.000208 | - | - | - |
| data TLB read misses | - | - | - | 7.378 | 0.0424 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.53M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.53M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 3547542 | 99930 | 1 | 0 | 3547542000 | 2998274000 | 0 | 11.5 | 39.9 | 128.0 | 208.9 | 5446.1 | 0 | 0 | 1000/1000 | 408.9 | 35.3 | - | 7.82 / 9.02 |
| best | quiver | 22 × wait | 3547542 | 99930 | 1 | 0 | 3547542000 | 2998274000 | 0 | 11.5 | 39.9 | 128.0 | 208.9 | 5446.1 | 0 | 0 | 1000/1000 | 408.9 | 35.3 | - | 7.82 / 9.02 |

Run 1's profile:

| profile: the window, 30.0 s, 2998279 messages, 2998278199 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 347.3G | 115.8k | 115.8 | 9943 | 105.9 | - | - | - |
| waiting for a CPU, ns | - | - | - | 3.266 | 0.207 | - | - | - |
| allocations | 6.026M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.026M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.448G | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 24.60M | 8.205 | 0.00821 | 0.00950 | 0.00820 | - | - | - |
| involuntary context switches | 13.35k | 0.00445 | 0.00000445 | 0.000324 | 0.00000413 | - | - | - |
| cycles | - | - | - | 60.17k | 605.9 | - | - | - |
| instructions | - | - | - | 40.85k | 856.0 | - | - | - |
| cache references | - | - | - | 134.6 | 3.999 | - | - | - |
| cache misses | - | - | - | 12.53 | 0.00550 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.488 | 0.0000678 | - | - | - |
| data TLB read misses | - | - | - | 10.27 | 0.00462 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.56M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.56M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 0.4 | 0.5 | 1.1 | 6.1 | 246.8 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 8.05 / 6.07 |
| best | quiver | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 0.4 | 0.5 | 1.1 | 6.1 | 246.8 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 8.05 / 6.07 |

Run 1's profile:

| profile: the window, 30.0 s, 299500 messages, 299500 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.86G | 199.9k | 199.9k | 99.94k | 99.92k | - | - | - |
| waiting for a CPU, ns | - | - | - | 9.476 | 12.63 | - | - | - |
| allocations | 601.9k | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.00000334 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.00000334 | 0.00000334 | 0 | - | - | - |
| frees | 586.2k | 1.957 | 1.957 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.1 | 483.1 | 483.1 | 0 | - | - | - |
| minor page faults | 1771 | 0.00591 | 0.00591 | 0.00591 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 633.0 | 0.00211 | 0.00211 | 0.000958 | 0.00116 | - | - | - |
| cycles | - | - | - | 852.3k | 869.1k | - | - | - |
| instructions | - | - | - | 672.3k | 4.077M | - | - | - |
| cache references | - | - | - | 57.53 | 26.34 | - | - | - |
| cache misses | - | - | - | 18.84 | 1.240 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.168 | 0.0174 | - | - | - |
| data TLB read misses | - | - | - | 0.317 | 0.148 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 36.03M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 36.03M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 0.9 | 1.3 | 3.4 | 14.7 | 3166.0 | 0 | 0 | 100/100 | 814.4 | 35.4 | - | 12.71 / 17.33 |
| best | quiver | 22 × busy | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 0.9 | 1.3 | 3.4 | 14.7 | 3166.0 | 0 | 0 | 100/100 | 814.4 | 35.4 | - | 12.71 / 17.33 |

Run 1's profile:

| profile: the window, 30.0 s, 299498 messages, 29949800 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.8G | 2.300M | 23.00k | 99.98k | 22.00k | - | - | - |
| waiting for a CPU, ns | - | - | - | 2.015 | 0.956 | - | - | - |
| allocations | 601.9k | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | - | - | - |
| frees | 586.2k | 1.957 | 0.0196 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.1 | 4.831 | 483.1 | 0 | - | - | - |
| minor page faults | 1770 | 0.00591 | 0.0000591 | 0.00591 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 2000 | 0.00668 | 0.0000668 | 0.000157 | 0.0000652 | - | - | - |
| cycles | - | - | - | 367.5k | 87.34k | - | - | - |
| instructions | - | - | - | 358.5k | 68.95k | - | - | - |
| cache references | - | - | - | 20.08 | 114.1 | - | - | - |
| cache misses | - | - | - | 5.462 | 0.0326 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000584 | 0.000365 | - | - | - |
| data TLB read misses | - | - | - | 0.0265 | 0.00677 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.91M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.91M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 354590 | 9989 | 0 | 0 | 354590000 | 299497000 | 0 | 2.6 | 4.2 | 14.1 | 167.9 | 3607.2 | 0 | 0 | 1000/1000 | 813.2 | 35.4 | - | 28.78 / 27.86 |
| best | quiver | 22 × busy | 354590 | 9989 | 0 | 0 | 354590000 | 299497000 | 0 | 2.6 | 4.2 | 14.1 | 167.9 | 3607.2 | 0 | 0 | 1000/1000 | 813.2 | 35.4 | - | 28.78 / 27.86 |

Run 1's profile:

| profile: the window, 30.0 s, 299496 messages, 299495345 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 687.7G | 2.296M | 2296 | 99.86k | 2196 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.466 | 0.142 | - | - | - |
| allocations | 601.9k | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.000000003 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.000000003 | 0.00000334 | 0 | - | - | - |
| frees | 586.2k | 1.957 | 0.00196 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.1 | 0.483 | 483.1 | 0 | - | - | - |
| minor page faults | 1769 | 0.00591 | 0.00000591 | 0.00591 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 3058 | 0.0102 | 0.0000102 | 0.000264 | 0.00000995 | - | - | - |
| cycles | - | - | - | 306.9k | 7191 | - | - | - |
| instructions | - | - | - | 312.1k | 19.36k | - | - | - |
| cache references | - | - | - | 17.77 | 8.019 | - | - | - |
| cache misses | - | - | - | 4.514 | 0.00534 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00328 | 0.000103 | - | - | - |
| data TLB read misses | - | - | - | 0.0301 | 0.0133 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.50M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.50M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 0.4 | 0.4 | 0.8 | 6.7 | 153.6 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 17.00 / 10.41 |
| best | quiver | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 0.4 | 0.4 | 0.8 | 6.7 | 153.6 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 17.00 / 10.41 |

Run 1's profile:

| profile: the window, 30.0 s, 2998278 messages, 2998278 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.89G | 19.97k | 19.97k | 9985 | 9988 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.918 | 0.703 | - | - | - |
| allocations | 6.026M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.026M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.448G | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 60.00 | 0.0000200 | 0.0000200 | 0.0000200 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 491.0 | 0.000164 | 0.000164 | 0.0000951 | 0.0000687 | - | - | - |
| cycles | - | - | - | 91.16k | 91.42k | - | - | - |
| instructions | - | - | - | 74.92k | 440.2k | - | - | - |
| cache references | - | - | - | 60.37 | 22.27 | - | - | - |
| cache misses | - | - | - | 11.77 | 0.293 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.118 | 0.000441 | - | - | - |
| data TLB read misses | - | - | - | 0.741 | 0.106 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.65M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.65M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 3547543 | 99931 | 0 | 0 | 354754300 | 299827400 | 0 | 1.0 | 1.5 | 1851.4 | 47185.9 | 122924.4 | 0 | 0 | 100/100 | 809.7 | 35.0 | - | 13.95 / 20.08 |
| best | quiver | 22 × busy | 3547543 | 99931 | 0 | 0 | 354754300 | 299827400 | 0 | 1.0 | 1.5 | 1851.4 | 47185.9 | 122924.4 | 0 | 0 | 100/100 | 809.7 | 35.0 | - | 13.95 / 20.08 |

Run 1's profile:

| profile: the window, 30.0 s, 2998279 messages, 299827900 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 684.2G | 228.2k | 2282 | 9836 | 2184 | - | - | - |
| waiting for a CPU, ns | - | - | - | 145.5 | 12.50 | - | - | - |
| allocations | 6.026M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.026M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.448G | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 12.00 | 0.00000400 | 0.000000040 | 0.00000400 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 1.000 | 0.000000334 | 0.000000003 | 0 | 0.000000003 | - | - | - |
| involuntary context switches | 4688 | 0.00156 | 0.0000156 | 0.0000747 | 0.0000149 | - | - | - |
| cycles | - | - | - | 34.41k | 8953 | - | - | - |
| instructions | - | - | - | 35.34k | 7109 | - | - | - |
| cache references | - | - | - | 26.92 | 11.44 | - | - | - |
| cache misses | - | - | - | 6.493 | 0.0241 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00378 | 0.000117 | - | - | - |
| data TLB read misses | - | - | - | 0.0322 | 0.00892 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.60M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.60M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 3547543 | 99930 | 0 | 0 | 3547543000 | 2998274000 | 0 | 3.4 | 78.8 | 2850.8 | 6094.8 | 138832.5 | 0 | 0 | 1000/1000 | 802.3 | 34.1 | - | 29.45 / 28.26 |
| best | quiver | 22 × busy | 3547543 | 99930 | 0 | 0 | 3547543000 | 2998274000 | 0 | 3.4 | 78.8 | 2850.8 | 6094.8 | 138832.5 | 0 | 0 | 1000/1000 | 802.3 | 34.1 | - | 29.45 / 28.26 |

Run 1's profile:

| profile: the window, 30.0 s, 2998201 messages, 2998201138 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 676.6G | 225.7k | 225.7 | 9563 | 216.1 | - | - | - |
| waiting for a CPU, ns | - | - | - | 412.9 | 3.274 | - | - | - |
| allocations | 6.025M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.025M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.448G | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 4.000 | 0.00000133 | 0.000000001 | 0.00000133 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 9.000 | 0.00000300 | 0.000000003 | 0.000000334 | 0.000000003 | - | - | - |
| involuntary context switches | 12.13k | 0.00405 | 0.00000405 | 0.000227 | 0.00000382 | - | - | - |
| cycles | - | - | - | 29.49k | 1009 | - | - | - |
| instructions | - | - | - | 29.84k | 2182 | - | - | - |
| cache references | - | - | - | 19.73 | 3.255 | - | - | - |
| cache misses | - | - | - | 4.946 | 0.0107 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00145 | 0.0000748 | - | - | - |
| data TLB read misses | - | - | - | 0.0309 | 0.00305 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.64M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.64M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 4.5 | 7.0 | 23.0 | 89.1 | 371.7 | 0 | 0 | 1/1 | 36.9 | 35.4 | - | 39.28 / 22.79 |
| best | quiver | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 4.5 | 7.0 | 23.0 | 89.1 | 371.7 | 0 | 0 | 1/1 | 36.9 | 35.4 | - | 39.28 / 22.79 |

Run 1's profile:

| profile: the window, 30.0 s, 299585 messages, 299585 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 31.20G | 104.2k | 104.2k | 99.93k | 4231 | - | - | - |
| waiting for a CPU, ns | - | - | - | 16.28 | 3.448 | - | - | - |
| allocations | 602.1k | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.00000334 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.00000334 | 0.00000334 | 0 | - | - | - |
| frees | 586.3k | 1.957 | 1.957 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 1757 | 0.00586 | 0.00586 | 0.00586 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 283.0k | 0.945 | 0.945 | 0 | 0.945 | - | - | - |
| involuntary context switches | 337.0 | 0.00112 | 0.00112 | 0.00111 | 0.0000134 | - | - | - |
| cycles | - | - | - | 921.8k | 3046 | - | - | - |
| instructions | - | - | - | 729.6k | 1885 | - | - | - |
| cache references | - | - | - | 54.55 | 48.10 | - | - | - |
| cache misses | - | - | - | 18.27 | 1.018 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.181 | 0.0517 | - | - | - |
| data TLB read misses | - | - | - | 0.413 | 0.408 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.71M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.71M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 354797 | 9994 | 1 | 0 | 35479700 | 29958300 | 0 | 17.2 | 90.1 | 213.0 | 524.3 | 4096.3 | 0 | 0 | 100/100 | 90.7 | 35.3 | - | 6.99 / 10.63 |
| best | quiver | 22 × wait | 354797 | 9994 | 1 | 0 | 35479700 | 29958300 | 0 | 17.2 | 90.1 | 213.0 | 524.3 | 4096.3 | 0 | 0 | 100/100 | 90.7 | 35.3 | - | 6.99 / 10.63 |

Run 1's profile:

| profile: the window, 30.0 s, 299581 messages, 29958164 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 75.78G | 253.0k | 2530 | 99.67k | 1533 | - | - | - |
| waiting for a CPU, ns | - | - | - | 28.74 | 217.7 | - | - | - |
| allocations | 602.1k | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | - | - | - |
| frees | 586.3k | 1.957 | 0.0196 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 1756 | 0.00586 | 0.0000586 | 0.00586 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 6.961M | 23.24 | 0.232 | 0.00292 | 0.232 | - | - | - |
| involuntary context switches | 18.52k | 0.0618 | 0.000618 | 0.00297 | 0.000589 | - | - | - |
| cycles | - | - | - | 669.4k | 1863 | - | - | - |
| instructions | - | - | - | 488.2k | 1122 | - | - | - |
| cache references | - | - | - | 260.8 | 13.03 | - | - | - |
| cache misses | - | - | - | 41.29 | 1.080 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.821 | 0.119 | - | - | - |
| data TLB read misses | - | - | - | 19.39 | 0.541 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.61M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.61M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 354798 | 9994 | 0 | 0 | 354798000 | 299583000 | 0 | 13.1 | 29.7 | 79.9 | 194.6 | 4792.1 | 0 | 0 | 1000/1000 | 112.8 | 35.3 | - | 9.77 / 8.89 |
| best | quiver | 22 × wait | 354798 | 9994 | 0 | 0 | 354798000 | 299583000 | 0 | 13.1 | 29.7 | 79.9 | 194.6 | 4792.1 | 0 | 0 | 1000/1000 | 112.8 | 35.3 | - | 9.77 / 8.89 |

Run 1's profile:

| profile: the window, 30.0 s, 299581 messages, 299581002 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 95.28G | 318.1k | 318.1 | 99.73k | 218.3 | - | - | - |
| waiting for a CPU, ns | - | - | - | 50.54 | 3.843 | - | - | - |
| allocations | 602.1k | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.000000003 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.000000003 | 0.00000334 | 0 | - | - | - |
| frees | 586.3k | 1.957 | 0.00196 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 1756 | 0.00586 | 0.00000586 | 0.00586 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 6.704M | 22.38 | 0.0224 | 0.00275 | 0.0224 | - | - | - |
| involuntary context switches | 12.66k | 0.0423 | 0.0000423 | 0.00513 | 0.0000371 | - | - | - |
| cycles | - | - | - | 740.9k | 812.8 | - | - | - |
| instructions | - | - | - | 556.2k | 997.8 | - | - | - |
| cache references | - | - | - | 226.0 | 8.488 | - | - | - |
| cache misses | - | - | - | 24.99 | 0.0681 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.766 | 0.00158 | - | - | - |
| data TLB read misses | - | - | - | 30.78 | 0.153 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.23M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.23M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 5.6 | 204.8 | 3637.2 | 6750.2 | 8078.6 | 0 | 0 | 1/1 | 43.6 | 34.8 | - | 46.27 / 42.22 |
| best | quiver | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 5.6 | 204.8 | 3637.2 | 6750.2 | 8078.6 | 0 | 0 | 1/1 | 43.6 | 34.8 | - | 46.27 / 42.22 |

Run 1's profile:

| profile: the window, 30.0 s, 3000358 messages, 3000358 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 36.69G | 12.23k | 12.23k | 9788 | 2441 | - | - | - |
| waiting for a CPU, ns | - | - | - | 154.3 | 1333 | - | - | - |
| allocations | 6.030M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.030M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.449G | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 6.000 | 0.00000200 | 0.00000200 | 0.00000200 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 1.656M | 0.552 | 0.552 | 0 | 0.552 | - | - | - |
| involuntary context switches | 62.44k | 0.0208 | 0.0208 | 0.0208 | 0.0000213 | - | - | - |
| cycles | - | - | - | 67.99k | 2556 | - | - | - |
| instructions | - | - | - | 53.47k | 1253 | - | - | - |
| cache references | - | - | - | 40.94 | 18.24 | - | - | - |
| cache misses | - | - | - | 19.47 | 2.032 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.966 | 0.172 | - | - | - |
| data TLB read misses | - | - | - | 0.0947 | 0.244 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 36.00M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 36.00M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 3550038 | 100001 | 0 | 0 | 355003800 | 300043700 | 0 | 14.7 | 42.5 | 86.0 | 151.6 | 3202.2 | 0 | 0 | 100/100 | 334.4 | 34.6 | - | 18.05 / 15.56 |
| best | quiver | 22 × wait | 3550038 | 100001 | 0 | 0 | 355003800 | 300043700 | 0 | 14.7 | 42.5 | 86.0 | 151.6 | 3202.2 | 0 | 0 | 100/100 | 334.4 | 34.6 | - | 18.05 / 15.56 |

Run 1's profile:

| profile: the window, 30.0 s, 3000442 messages, 300044034 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 283.7G | 94.57k | 945.7 | 9762 | 848.0 | - | - | - |
| waiting for a CPU, ns | - | - | - | 3.650 | 1.986 | - | - | - |
| allocations | 6.030M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.030M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.449G | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 34.53M | 11.51 | 0.115 | 0.0493 | 0.115 | - | - | - |
| involuntary context switches | 10.74k | 0.00358 | 0.0000358 | 0.000340 | 0.0000324 | - | - | - |
| cycles | - | - | - | 49.76k | 1468 | - | - | - |
| instructions | - | - | - | 32.08k | 951.1 | - | - | - |
| cache references | - | - | - | 144.4 | 7.702 | - | - | - |
| cache misses | - | - | - | 12.18 | 0.0329 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.483 | 0.000186 | - | - | - |
| data TLB read misses | - | - | - | 7.770 | 0.0485 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.81M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.81M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 3550038 | 100002 | 0 | 0 | 3550038000 | 3000437000 | 0 | 12.8 | 47.6 | 147.5 | 241.7 | 5072.4 | 0 | 0 | 1000/1000 | 426.1 | 35.2 | - | 10.78 / 11.38 |
| best | quiver | 22 × wait | 3550038 | 100002 | 0 | 0 | 3550038000 | 3000437000 | 0 | 12.8 | 47.6 | 147.5 | 241.7 | 5072.4 | 0 | 0 | 1000/1000 | 426.1 | 35.2 | - | 10.78 / 11.38 |

Run 1's profile:

| profile: the window, 30.0 s, 3000439 messages, 3000437828 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 360.0G | 120.0k | 120.0 | 9917 | 110.1 | - | - | - |
| waiting for a CPU, ns | - | - | - | 4.512 | 1.129 | - | - | - |
| allocations | 6.030M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.030M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.449G | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 11.00 | 0.00000367 | 0.000000004 | 0.00000367 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 22.69M | 7.561 | 0.00756 | 0.00916 | 0.00755 | - | - | - |
| involuntary context switches | 41.58k | 0.0139 | 0.0000139 | 0.000610 | 0.0000132 | - | - | - |
| cycles | - | - | - | 55.29k | 593.7 | - | - | - |
| instructions | - | - | - | 37.62k | 841.6 | - | - | - |
| cache references | - | - | - | 129.2 | 3.785 | - | - | - |
| cache misses | - | - | - | 12.58 | 0.00564 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.480 | 0.0000584 | - | - | - |
| data TLB read misses | - | - | - | 9.414 | 0.00601 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.76M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.76M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 0.6 | 0.8 | 1.4 | 5.3 | 283.5 | 0 | 0 | 1/1 | 70.8 | 35.4 | - | 56.87 / 36.14 |
| best | quiver | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 0.6 | 0.8 | 1.4 | 5.3 | 283.5 | 0 | 0 | 1/1 | 70.8 | 35.4 | - | 56.87 / 36.14 |

Run 1's profile:

| profile: the window, 30.0 s, 299586 messages, 299586 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.83G | 199.7k | 199.7k | 99.87k | 99.85k | - | - | - |
| waiting for a CPU, ns | - | - | - | 10.69 | 17.42 | - | - | - |
| allocations | 602.1k | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.00000334 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.00000334 | 0.00000334 | 0 | - | - | - |
| frees | 586.3k | 1.957 | 1.957 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 1757 | 0.00586 | 0.00586 | 0.00586 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 406.0 | 0.00136 | 0.00136 | 0.000561 | 0.000794 | - | - | - |
| cycles | - | - | - | 728.4k | 738.0k | - | - | - |
| instructions | - | - | - | 565.4k | 3.427M | - | - | - |
| cache references | - | - | - | 46.30 | 24.89 | - | - | - |
| cache misses | - | - | - | 15.15 | 1.130 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.268 | 0.0574 | - | - | - |
| data TLB read misses | - | - | - | 0.0566 | 0.0603 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.77M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.77M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 1.4 | 3014.7 | 105906.2 | 289407.0 | 453792.9 | 0 | 0 | 100/100 | 623.3 | 26.5 | - | 22.97 / 45.44 |
| best | quiver | 22 × busy | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 1.4 | 3014.7 | 105906.2 | 289407.0 | 453792.9 | 0 | 0 | 100/100 | 623.3 | 26.5 | - | 22.97 / 45.44 |

Run 1's profile:

| profile: the window, 30.0 s, 299583 messages, 29956561 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 539.2G | 1.800M | 18.00k | 76.59k | 17.23k | - | - | - |
| waiting for a CPU, ns | - | - | - | 23.20k | 4726 | - | - | - |
| allocations | 602.1k | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | - | - | - |
| frees | 586.3k | 1.957 | 0.0196 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 1803 | 0.00602 | 0.0000602 | 0.00591 | 0.00000110 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 49.00 | 0.000164 | 0.00000164 | 0.0000467 | 0.00000117 | - | - | - |
| involuntary context switches | 94.33k | 0.315 | 0.00315 | 0.0141 | 0.00301 | - | - | - |
| cycles | - | - | - | 506.4k | 102.4k | - | - | - |
| instructions | - | - | - | 393.5k | 101.6k | - | - | - |
| cache references | - | - | - | 71.36 | 143.6 | - | - | - |
| cache misses | - | - | - | 22.12 | 0.649 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.643 | 0.0312 | - | - | - |
| data TLB read misses | - | - | - | 1.116 | 0.115 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.80M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.80M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 354798 | 9994 | 0 | 0 | 354798000 | 299583000 | 0 | 2.8 | 4.5 | 18.7 | 182.3 | 10883.7 | 0 | 0 | 1000/1000 | 813.1 | 35.4 | - | 38.23 / 33.02 |
| best | quiver | 22 × busy | 354798 | 9994 | 0 | 0 | 354798000 | 299583000 | 0 | 2.8 | 4.5 | 18.7 | 182.3 | 10883.7 | 0 | 0 | 1000/1000 | 813.1 | 35.4 | - | 38.23 / 33.02 |

Run 1's profile:

| profile: the window, 30.0 s, 299576 messages, 299576090 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 687.5G | 2.295M | 2295 | 99.74k | 2195 | - | - | - |
| waiting for a CPU, ns | - | - | - | 30.58 | 0.483 | - | - | - |
| allocations | 602.0k | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000334 | 0.000000003 | 0.00000334 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.000000003 | 0.00000334 | 0 | - | - | - |
| frees | 586.3k | 1.957 | 0.00196 | 1.957 | 0 | - | - | - |
| bytes allocated | 144.7M | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 1755 | 0.00586 | 0.00000586 | 0.00586 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 3839 | 0.0128 | 0.0000128 | 0.000601 | 0.0000122 | - | - | - |
| cycles | - | - | - | 302.2k | 6970 | - | - | - |
| instructions | - | - | - | 302.3k | 19.31k | - | - | - |
| cache references | - | - | - | 18.09 | 8.403 | - | - | - |
| cache misses | - | - | - | 4.602 | 0.00591 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00323 | 0.000109 | - | - | - |
| data TLB read misses | - | - | - | 0.0302 | 0.0138 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.36M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.36M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × busy | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 0.4 | 0.5 | 0.8 | 6.8 | 145.7 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 34.16 / 20.01 |
| best | quiver | 1 × busy | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 0.4 | 0.5 | 0.8 | 6.8 | 145.7 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 34.16 / 20.01 |

Run 1's profile:

| profile: the window, 30.0 s, 3000422 messages, 3000422 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.87G | 19.95k | 19.95k | 9978 | 9976 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.641 | 0.850 | - | - | - |
| allocations | 6.030M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.030M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.449G | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 60.00 | 0.0000200 | 0.0000200 | 0.0000200 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 494.0 | 0.000165 | 0.000165 | 0.0000640 | 0.000101 | - | - | - |
| cycles | - | - | - | 82.31k | 47.59k | - | - | - |
| instructions | - | - | - | 66.92k | 260.1k | - | - | - |
| cache references | - | - | - | 44.79 | 13.80 | - | - | - |
| cache misses | - | - | - | 11.68 | 0.145 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.125 | 0.000633 | - | - | - |
| data TLB read misses | - | - | - | 0.0865 | 0.0324 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.83M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.83M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 3550038 | 100002 | 0 | 0 | 355003800 | 300043700 | 0 | 1.0 | 1.4 | 7.0 | 20.0 | 75348.8 | 0 | 0 | 100/100 | 814.0 | 35.4 | - | 5.40 / 13.73 |
| best | quiver | 22 × busy | 3550038 | 100002 | 0 | 0 | 355003800 | 300043700 | 0 | 1.0 | 1.4 | 7.0 | 20.0 | 75348.8 | 0 | 0 | 100/100 | 814.0 | 35.4 | - | 5.40 / 13.73 |

Run 1's profile:

| profile: the window, 30.0 s, 3000441 messages, 300044100 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.1G | 229.3k | 2293 | 9976 | 2194 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.414 | 1.506 | - | - | - |
| allocations | 6.030M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.030M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.449G | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 11.00 | 0.00000367 | 0.000000037 | 0.00000267 | 0.000000010 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 5.000 | 0.00000167 | 0.000000017 | 0.000000667 | 0.000000010 | - | - | - |
| involuntary context switches | 3288 | 0.00110 | 0.0000110 | 0.0000260 | 0.0000107 | - | - | - |
| cycles | - | - | - | 36.29k | 8609 | - | - | - |
| instructions | - | - | - | 36.94k | 7240 | - | - | - |
| cache references | - | - | - | 27.54 | 11.37 | - | - | - |
| cache misses | - | - | - | 6.411 | 0.0193 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00337 | 0.0000674 | - | - | - |
| data TLB read misses | - | - | - | 0.0390 | 0.00909 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.72M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.72M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 3550038 | 100000 | 0 | 0 | 3541211352 | 2991610352 | 8826648 | 3.5 | 1359.9 | 478150.7 | undelivered | undelivered | 564 | 0 | 578/1000 | 773.0 | 32.9 | - | 30.71 / 28.14 |
| best | quiver | 22 × busy | 3550038 | 100000 | 0 | 0 | 3541211352 | 2991610352 | 8826648 | 3.5 | 1359.9 | 478150.7 | undelivered | undelivered | 564 | 0 | 578/1000 | 773.0 | 32.9 | - | 30.71 / 28.14 |

Run 1's profile:

| profile: the window, 30.0 s, 3000363 messages, 2991543057 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 647.7G | 215.9k | 216.5 | 9131 | 207.4 | - | - | - |
| waiting for a CPU, ns | - | - | - | 839.9 | 12.56 | - | - | - |
| allocations | 6.030M | 2.010 | 0.00202 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.030M | 2.010 | 0.00202 | 1.998 | 0.0000119 | - | - | - |
| bytes allocated | 1.449G | 483.0 | 0.484 | 483.0 | 0 | - | - | - |
| minor page faults | 1095 | 0.000365 | 0.000000366 | 0.000364 | 0.000000001 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 106.0 | 0.0000353 | 0.000000035 | 0.00000300 | 0.000000032 | - | - | - |
| involuntary context switches | 20.33k | 0.00678 | 0.00000680 | 0.000577 | 0.00000622 | - | - | - |
| cycles | - | - | - | 61.05k | 846.7 | - | - | - |
| instructions | - | - | - | 50.32k | 1904 | - | - | - |
| cache references | - | - | - | 58.84 | 2.990 | - | - | - |
| cache misses | - | - | - | 24.87 | 0.132 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 1.210 | 0.000351 | - | - | - |
| data TLB read misses | - | - | - | 0.363 | 0.0116 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 44.59M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 44.59M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 23.85M bytes, holding 65536 messages: 363.9 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 8.4 | 15.4 | 376.8 | 1884.2 | 3550.8 | 0 | 0 | 1/1 | 37.4 | 35.1 | - | 35.70 / 35.45 |
| best | quiver | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 8.4 | 15.4 | 376.8 | 1884.2 | 3550.8 | 0 | 0 | 1/1 | 37.4 | 35.1 | - | 35.70 / 35.45 |

Run 1's profile:

| profile: the window, 30.0 s, 300767 messages, 300768 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 31.51G | 104.8k | 104.8k | 98.40k | 6361 | - | - | - |
| waiting for a CPU, ns | - | - | - | 908.0 | 3113 | - | - | - |
| allocations | 604.4k | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000332 | 0.00000332 | 0.00000332 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000332 | 0.00000332 | 0.00000332 | 0 | - | - | - |
| frees | 589.0k | 1.958 | 1.958 | 1.958 | 0 | - | - | - |
| bytes allocated | 145.3M | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 1730 | 0.00575 | 0.00575 | 0.00576 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 291.6k | 0.969 | 0.969 | 0 | 0.969 | - | - | - |
| involuntary context switches | 41.10k | 0.137 | 0.137 | 0.136 | 0.000555 | - | - | - |
| cycles | - | - | - | 686.6k | 6324 | - | - | - |
| instructions | - | - | - | 528.0k | 1704 | - | - | - |
| cache references | - | - | - | 111.4 | 107.9 | - | - | - |
| cache misses | - | - | - | 38.89 | 18.37 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 1.230 | 0.867 | - | - | - |
| data TLB read misses | - | - | - | 0.405 | 7.381 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.81M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.81M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 12.4 | 26.6 | 68.6 | 127.0 | 2931.2 | 0 | 0 | 100/100 | 94.8 | 35.3 | - | 13.69 / 10.31 |
| best | quiver | 22 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 12.4 | 26.6 | 68.6 | 127.0 | 2931.2 | 0 | 0 | 100/100 | 94.8 | 35.3 | - | 13.69 / 10.31 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 30076800 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 80.09G | 266.3k | 2663 | 99.25k | 1670 | - | - | - |
| waiting for a CPU, ns | - | - | - | 34.37 | 11.33 | - | - | - |
| allocations | 604.4k | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000332 | 0.000000033 | 0.00000332 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000332 | 0.000000033 | 0.00000332 | 0 | - | - | - |
| frees | 589.0k | 1.958 | 0.0196 | 1.958 | 0 | - | - | - |
| bytes allocated | 145.3M | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 1729 | 0.00575 | 0.0000575 | 0.00575 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 7.988M | 26.56 | 0.266 | 0.00400 | 0.266 | - | - | - |
| involuntary context switches | 4159 | 0.0138 | 0.000138 | 0.00387 | 0.0000996 | - | - | - |
| cycles | - | - | - | 696.7k | 2116 | - | - | - |
| instructions | - | - | - | 535.6k | 1185 | - | - | - |
| cache references | - | - | - | 198.9 | 14.76 | - | - | - |
| cache misses | - | - | - | 17.56 | 0.342 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.765 | 0.0133 | - | - | - |
| data TLB read misses | - | - | - | 20.84 | 0.967 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.73M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.73M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 356101 | 10031 | 0 | 0 | 356101000 | 300769000 | 0 | 13.7 | 27.1 | 69.6 | 163.8 | 4035.7 | 0 | 0 | 1000/1000 | 116.1 | 35.3 | - | 17.15 / 12.40 |
| best | quiver | 22 × wait | 356101 | 10031 | 0 | 0 | 356101000 | 300769000 | 0 | 13.7 | 27.1 | 69.6 | 163.8 | 4035.7 | 0 | 0 | 1000/1000 | 116.1 | 35.3 | - | 17.15 / 12.40 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 300768000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 98.26G | 326.7k | 326.7 | 99.30k | 227.4 | - | - | - |
| waiting for a CPU, ns | - | - | - | 47.75 | 4.299 | - | - | - |
| allocations | 604.4k | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000332 | 0.000000003 | 0.00000332 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000332 | 0.000000003 | 0.00000332 | 0 | - | - | - |
| frees | 589.0k | 1.958 | 0.00196 | 1.958 | 0 | - | - | - |
| bytes allocated | 145.3M | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 1729 | 0.00575 | 0.00000575 | 0.00576 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 7.048M | 23.43 | 0.0234 | 0.00239 | 0.0234 | - | - | - |
| involuntary context switches | 13.53k | 0.0450 | 0.0000450 | 0.00853 | 0.0000365 | - | - | - |
| cycles | - | - | - | 708.3k | 811.1 | - | - | - |
| instructions | - | - | - | 540.4k | 1005 | - | - | - |
| cache references | - | - | - | 216.9 | 8.423 | - | - | - |
| cache misses | - | - | - | 19.11 | 0.0395 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.749 | 0.000814 | - | - | - |
| data TLB read misses | - | - | - | 28.31 | 0.148 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.69M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.69M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × wait | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 4.2 | 4.9 | 10.1 | 110.6 | 429.9 | 0 | 0 | 1/1 | 43.8 | 35.4 | - | 13.01 / 8.74 |
| best | quiver | 1 × wait | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 4.2 | 4.9 | 10.1 | 110.6 | 429.9 | 0 | 0 | 1/1 | 43.8 | 35.4 | - | 13.01 / 8.74 |

Run 1's profile:

| profile: the window, 30.0 s, 3002642 messages, 3002643 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 36.84G | 12.27k | 12.27k | 9971 | 2297 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.052 | 1.345 | - | - | - |
| allocations | 6.034M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.034M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.450G | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 6.000 | 0.00000200 | 0.00000200 | 0.00000200 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 2.081M | 0.693 | 0.693 | 0 | 0.693 | - | - | - |
| involuntary context switches | 265.0 | 0.0000883 | 0.0000883 | 0.0000729 | 0.0000153 | - | - | - |
| cycles | - | - | - | 89.51k | 2521 | - | - | - |
| instructions | - | - | - | 71.96k | 1624 | - | - | - |
| cache references | - | - | - | 42.62 | 33.98 | - | - | - |
| cache misses | - | - | - | 14.31 | 0.222 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.169 | 0.00195 | - | - | - |
| data TLB read misses | - | - | - | 0.126 | 0.236 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.73M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.73M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 3554018 | 100113 | 0 | 0 | 355401800 | 300263100 | 0 | 18.9 | 64.5 | 137.2 | 565.2 | 3594.9 | 0 | 0 | 100/100 | 274.3 | 34.6 | - | 15.15 / 16.50 |
| best | quiver | 22 × wait | 3554018 | 100113 | 0 | 0 | 355401800 | 300263100 | 0 | 18.9 | 64.5 | 137.2 | 565.2 | 3594.9 | 0 | 0 | 100/100 | 274.3 | 34.6 | - | 15.15 / 16.50 |

Run 1's profile:

| profile: the window, 30.0 s, 3002641 messages, 300263788 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 233.1G | 77.62k | 776.2 | 9738 | 678.9 | - | - | - |
| waiting for a CPU, ns | - | - | - | 26.23 | 31.73 | - | - | - |
| allocations | 6.034M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.034M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.450G | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 27.80M | 9.259 | 0.0926 | 0.0260 | 0.0923 | - | - | - |
| involuntary context switches | 109.1k | 0.0363 | 0.000363 | 0.00208 | 0.000342 | - | - | - |
| cycles | - | - | - | 45.46k | 1171 | - | - | - |
| instructions | - | - | - | 30.05k | 897.0 | - | - | - |
| cache references | - | - | - | 137.0 | 8.437 | - | - | - |
| cache misses | - | - | - | 15.33 | 0.0945 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.595 | 0.000948 | - | - | - |
| data TLB read misses | - | - | - | 6.349 | 0.138 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.61M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.61M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 3554018 | 100113 | 0 | 0 | 3554018000 | 3002631000 | 0 | 27.4 | 149.5 | 647.2 | 2654.2 | 35334.5 | 0 | 0 | 1000/1000 | 343.6 | 35.1 | - | 19.18 / 23.89 |
| best | quiver | 22 × wait | 3554018 | 100113 | 0 | 0 | 3554018000 | 3002631000 | 0 | 27.4 | 149.5 | 647.2 | 2654.2 | 35334.5 | 0 | 0 | 1000/1000 | 343.6 | 35.1 | - | 19.18 / 23.89 |

Run 1's profile:

| profile: the window, 30.0 s, 3002642 messages, 3002643481 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 304.2G | 101.3k | 101.3 | 9898 | 91.42 | - | - | - |
| waiting for a CPU, ns | - | - | - | 24.86 | 32.74 | - | - | - |
| allocations | 6.034M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.034M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.450G | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 13.65M | 4.546 | 0.00455 | 0.00211 | 0.00454 | - | - | - |
| involuntary context switches | 132.1k | 0.0440 | 0.0000440 | 0.000704 | 0.0000433 | - | - | - |
| cycles | - | - | - | 45.84k | 457.7 | - | - | - |
| instructions | - | - | - | 32.36k | 775.0 | - | - | - |
| cache references | - | - | - | 115.9 | 2.520 | - | - | - |
| cache misses | - | - | - | 16.28 | 0.0187 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.583 | 0.000419 | - | - | - |
| data TLB read misses | - | - | - | 5.462 | 0.0144 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.63M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.63M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 0.4 | 0.5 | 1.1 | 3.9 | 177.2 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 6.24 / 5.27 |
| best | quiver | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 0.4 | 0.5 | 1.1 | 3.9 | 177.2 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 6.24 / 5.27 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 300768 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.87G | 199.1k | 199.1k | 99.53k | 99.54k | - | - | - |
| waiting for a CPU, ns | - | - | - | 5.333 | 8.457 | - | - | - |
| allocations | 604.4k | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000332 | 0.00000332 | 0.00000332 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000332 | 0.00000332 | 0.00000332 | 0 | - | - | - |
| frees | 589.0k | 1.958 | 1.958 | 1.958 | 0 | - | - | - |
| bytes allocated | 145.3M | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 1730 | 0.00575 | 0.00575 | 0.00576 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 501.0 | 0.00167 | 0.00167 | 0.000708 | 0.000958 | - | - | - |
| cycles | - | - | - | 498.3k | 885.9k | - | - | - |
| instructions | - | - | - | 287.1k | 4.155M | - | - | - |
| cache references | - | - | - | 21.01 | 25.27 | - | - | - |
| cache misses | - | - | - | 6.098 | 1.002 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.135 | 0.00780 | - | - | - |
| data TLB read misses | - | - | - | 0.0193 | 0.0883 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.71M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.71M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 1.0 | 1.3 | 3.2 | 15.6 | 2601.8 | 0 | 0 | 100/100 | 811.9 | 35.4 | - | 28.25 / 26.61 |
| best | quiver | 22 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 1.0 | 1.3 | 3.2 | 15.6 | 2601.8 | 0 | 0 | 100/100 | 811.9 | 35.4 | - | 28.25 / 26.61 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 30076800 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.3G | 2.288M | 22.88k | 99.46k | 21.89k | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.157 | 1.605 | - | - | - |
| allocations | 604.4k | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000332 | 0.000000033 | 0.00000332 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000332 | 0.000000033 | 0.00000332 | 0 | - | - | - |
| frees | 589.0k | 1.958 | 0.0196 | 1.958 | 0 | - | - | - |
| bytes allocated | 145.3M | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 1730 | 0.00575 | 0.0000575 | 0.00575 | 0.000000033 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 1.000 | 0.00000332 | 0.000000033 | 0 | 0.000000033 | - | - | - |
| involuntary context switches | 3337 | 0.0111 | 0.000111 | 0.000266 | 0.000108 | - | - | - |
| cycles | - | - | - | 356.2k | 84.26k | - | - | - |
| instructions | - | - | - | 363.9k | 62.40k | - | - | - |
| cache references | - | - | - | 19.20 | 100.1 | - | - | - |
| cache misses | - | - | - | 5.405 | 0.0329 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000864 | 0.000431 | - | - | - |
| data TLB read misses | - | - | - | 0.0437 | 0.00885 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.73M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.73M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 356101 | 10031 | 0 | 0 | 356101000 | 300769000 | 0 | 2.6 | 4.2 | 15.4 | 167.9 | 2236.0 | 0 | 0 | 1000/1000 | 813.5 | 35.4 | - | 33.48 / 29.77 |
| best | quiver | 22 × busy | 356101 | 10031 | 0 | 0 | 356101000 | 300769000 | 0 | 2.6 | 4.2 | 15.4 | 167.9 | 2236.0 | 0 | 0 | 1000/1000 | 813.5 | 35.4 | - | 33.48 / 29.77 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 300768000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 687.9G | 2.287M | 2287 | 99.39k | 2188 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.458 | 0.117 | - | - | - |
| allocations | 604.4k | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.00000332 | 0.000000003 | 0.00000332 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.00000332 | 0.000000003 | 0.00000332 | 0 | - | - | - |
| frees | 589.0k | 1.958 | 0.00196 | 1.958 | 0 | - | - | - |
| bytes allocated | 145.3M | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 1730 | 0.00575 | 0.00000575 | 0.00576 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 2922 | 0.00972 | 0.00000972 | 0.000283 | 0.00000943 | - | - | - |
| cycles | - | - | - | 300.8k | 6915 | - | - | - |
| instructions | - | - | - | 312.4k | 19.74k | - | - | - |
| cache references | - | - | - | 17.19 | 7.290 | - | - | - |
| cache misses | - | - | - | 4.307 | 0.00310 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00277 | 0.0000154 | - | - | - |
| data TLB read misses | - | - | - | 0.0273 | 0.0119 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.44M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.44M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × busy | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 0.5 | 0.5 | 1.0 | 7.6 | 152.3 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 13.81 / 10.88 |
| best | quiver | 1 × busy | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 0.5 | 0.5 | 1.0 | 7.6 | 152.3 | 0 | 0 | 1/1 | 70.9 | 35.4 | - | 13.81 / 10.88 |

Run 1's profile:

| profile: the window, 30.0 s, 3002634 messages, 3002634 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.86G | 19.94k | 19.94k | 9969 | 9967 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.289 | 0.660 | - | - | - |
| allocations | 6.034M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.034M | 2.010 | 2.010 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.450G | 483.0 | 483.0 | 483.0 | 0 | - | - | - |
| minor page faults | 60.00 | 0.0000200 | 0.0000200 | 0.0000200 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 343.0 | 0.000114 | 0.000114 | 0.0000300 | 0.0000843 | - | - | - |
| cycles | - | - | - | 79.78k | 79.98k | - | - | - |
| instructions | - | - | - | 64.45k | 384.3k | - | - | - |
| cache references | - | - | - | 44.88 | 22.41 | - | - | - |
| cache misses | - | - | - | 13.01 | 0.304 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.200 | 0.00757 | - | - | - |
| data TLB read misses | - | - | - | 0.0651 | 0.0587 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 35.77M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 35.77M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 3554018 | 100114 | 0 | 0 | 355401800 | 300263100 | 0 | 1.2 | 1.6 | 8.1 | 23.3 | 7425.6 | 0 | 0 | 100/100 | 807.4 | 35.3 | - | 39.71 / 34.97 |
| best | quiver | 22 × busy | 3554018 | 100114 | 0 | 0 | 355401800 | 300263100 | 0 | 1.2 | 1.6 | 8.1 | 23.3 | 7425.6 | 0 | 0 | 100/100 | 807.4 | 35.3 | - | 39.71 / 34.97 |

Run 1's profile:

| profile: the window, 30.0 s, 3002590 messages, 300258975 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 687.6G | 229.0k | 2290 | 9951 | 2190 | - | - | - |
| waiting for a CPU, ns | - | - | - | 6.198 | 0.823 | - | - | - |
| allocations | 6.034M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.034M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.450G | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 5530 | 0.00184 | 0.0000184 | 0.000186 | 0.0000166 | - | - | - |
| cycles | - | - | - | 32.36k | 8240 | - | - | - |
| instructions | - | - | - | 33.92k | 6213 | - | - | - |
| cache references | - | - | - | 27.78 | 10.74 | - | - | - |
| cache misses | - | - | - | 6.795 | 0.0256 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00360 | 0.000152 | - | - | - |
| data TLB read misses | - | - | - | 0.0313 | 0.0114 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.75M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.75M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 3554018 | 100113 | 0 | 0 | 3554018000 | 3002631000 | 0 | 2.9 | 12.7 | 176.1 | 260.1 | 7380.9 | 0 | 0 | 1000/1000 | 811.5 | 35.3 | - | 32.80 / 30.07 |
| best | quiver | 22 × busy | 3554018 | 100113 | 0 | 0 | 3554018000 | 3002631000 | 0 | 2.9 | 12.7 | 176.1 | 260.1 | 7380.9 | 0 | 0 | 1000/1000 | 811.5 | 35.3 | - | 32.80 / 30.07 |

Run 1's profile:

| profile: the window, 30.0 s, 3002637 messages, 3002637000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 687.7G | 229.0k | 229.0 | 9956 | 219.1 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.507 | 0.0176 | - | - | - |
| allocations | 6.034M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.034M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.450G | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 12.00 | 0.00000400 | 0.000000004 | 0.00000400 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 2576 | 0.000858 | 0.000000858 | 0.0000659 | 0.000000792 | - | - | - |
| cycles | - | - | - | 36.16k | 703.2 | - | - | - |
| instructions | - | - | - | 20.95k | 1584 | - | - | - |
| cache references | - | - | - | 21.30 | 2.152 | - | - | - |
| cache misses | - | - | - | 5.291 | 0.000915 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.140 | 0.0000137 | - | - | - |
| data TLB read misses | - | - | - | 0.0264 | 0.00138 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.51M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.51M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
