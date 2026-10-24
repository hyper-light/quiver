# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 3547543 | 99932 | 0 | 0 | 354754300 | 299827400 | 0 | 13.2 | 38.4 | 78.8 | 130.0 | 1755.1 | 0 | 0 | 100/100 | 329.3 | 34.6 | - | 0.91 / 4.31 |
| 2 | quiver | 22 × wait | 3550038 | 100000 | 0 | 0 | 355003800 | 300043700 | 0 | 12.5 | 35.3 | 71.7 | 119.8 | 2033.9 | 0 | 0 | 100/100 | 341.0 | 34.6 | - | 4.31 / 6.91 |
| 3 | quiver | 22 × wait | 3554018 | 100111 | 0 | 0 | 355401800 | 300263100 | 0 | 12.7 | 36.4 | 78.8 | 137.2 | 2255.7 | 0 | 0 | 100/100 | 326.0 | 35.0 | - | 6.91 / 7.42 |
| best | quiver | 22 × wait | 3550038 | 100000 | 0 | 0 | 355003800 | 300043700 | 0 | 12.5 | 35.3 | 71.7 | 119.8 | 2033.9 | 0 | 0 | 100/100 | 341.0 | 34.6 | - | 4.31 / 6.91 |

Run 1's profile:

| profile: the window, 30.0 s, 2998278 messages, 299827584 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 278.0G | 92.73k | 927.3 | 9753 | 829.8 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.784 | 0.899 | - | - | - |
| allocations | 6.026M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.026M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.448G | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 23.00 | 0.00000767 | 0.000000077 | 0.00000767 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 36.24M | 12.09 | 0.121 | 0.0594 | 0.120 | - | - | - |
| involuntary context switches | 7992 | 0.00267 | 0.0000267 | 0.000280 | 0.0000239 | - | - | - |
| cycles | - | - | - | 53.76k | 1516 | - | - | - |
| instructions | - | - | - | 35.85k | 962.1 | - | - | - |
| cache references | - | - | - | 138.7 | 7.828 | - | - | - |
| cache misses | - | - | - | 11.98 | 0.0317 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.479 | 0.000159 | - | - | - |
| data TLB read misses | - | - | - | 8.554 | 0.0439 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.90M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.90M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message

Run 2's profile:

| profile: the window, 30.0 s, 3000438 messages, 300043937 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 290.4G | 96.78k | 967.8 | 9754 | 870.3 | - | - | - |
| waiting for a CPU, ns | - | - | - | 2.296 | 0.730 | - | - | - |
| allocations | 6.030M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.030M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.449G | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 23.00 | 0.00000767 | 0.000000077 | 0.00000767 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 36.89M | 12.30 | 0.123 | 0.0621 | 0.122 | - | - | - |
| involuntary context switches | 6056 | 0.00202 | 0.0000202 | 0.000217 | 0.0000180 | - | - | - |
| cycles | - | - | - | 53.55k | 1608 | - | - | - |
| instructions | - | - | - | 35.68k | 969.7 | - | - | - |
| cache references | - | - | - | 140.4 | 9.672 | - | - | - |
| cache misses | - | - | - | 11.97 | 0.0222 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.476 | 0.000138 | - | - | - |
| data TLB read misses | - | - | - | 8.872 | 0.0402 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 38.35M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 38.35M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message

Run 3's profile:

| profile: the window, 30.0 s, 3002639 messages, 300264019 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 274.9G | 91.54k | 915.4 | 9849 | 816.9 | - | - | - |
| waiting for a CPU, ns | - | - | - | 2.422 | 1.026 | - | - | - |
| allocations | 6.034M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.034M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.450G | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 17.00 | 0.00000566 | 0.000000057 | 0.00000566 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 35.16M | 11.71 | 0.117 | 0.0333 | 0.117 | - | - | - |
| involuntary context switches | 7452 | 0.00248 | 0.0000248 | 0.000254 | 0.0000223 | - | - | - |
| cycles | - | - | - | 53.15k | 1541 | - | - | - |
| instructions | - | - | - | 35.73k | 971.3 | - | - | - |
| cache references | - | - | - | 136.8 | 8.639 | - | - | - |
| cache misses | - | - | - | 12.09 | 0.0283 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.473 | 0.000206 | - | - | - |
| data TLB read misses | - | - | - | 8.351 | 0.0417 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 38.45M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 38.45M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
