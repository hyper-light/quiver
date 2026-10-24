# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 3547543 | 99936 | 0 | 0 | 354754300 | 299827400 | 0 | 0.8 | 1.1 | 3.8 | 14.2 | 1001.3 | 0 | 0 | 100/100 | 814.4 | 35.4 | - | 1.09 / 10.83 |
| 2 | quiver | 22 × busy | 3550038 | 100000 | 0 | 0 | 355003800 | 300043700 | 0 | 0.8 | 1.1 | 4.5 | 16.1 | 10272.3 | 0 | 0 | 100/100 | 815.0 | 35.4 | - | 10.83 / 16.33 |
| 3 | quiver | 22 × busy | 3554018 | 100111 | 0 | 0 | 355401800 | 300263100 | 0 | 0.7 | 1.0 | 4.2 | 15.6 | 18752.2 | 0 | 0 | 100/100 | 815.1 | 35.4 | - | 16.33 / 19.28 |
| best | quiver | 22 × busy | 3547543 | 99936 | 0 | 0 | 354754300 | 299827400 | 0 | 0.8 | 1.1 | 3.8 | 14.2 | 1001.3 | 0 | 0 | 100/100 | 814.4 | 35.4 | - | 1.09 / 10.83 |

Run 1's profile:

| profile: the window, 30.0 s, 2998278 messages, 299827800 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.8G | 229.7k | 2297 | 9991 | 2198 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.243 | 0.0911 | - | - | - |
| allocations | 6.026M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.026M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.448G | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 7.000 | 0.00000233 | 0.000000023 | 0.00000233 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 2112 | 0.000704 | 0.00000704 | 0.0000437 | 0.00000660 | - | - | - |
| cycles | - | - | - | 47.44k | 8861 | - | - | - |
| instructions | - | - | - | 28.84k | 6574 | - | - | - |
| cache references | - | - | - | 26.53 | 11.86 | - | - | - |
| cache misses | - | - | - | 4.718 | 0.00268 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0818 | 0.00000728 | - | - | - |
| data TLB read misses | - | - | - | 0.0288 | 0.00775 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.70M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.70M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message

Run 2's profile:

| profile: the window, 30.0 s, 3000439 messages, 300043900 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.7G | 229.5k | 2295 | 9981 | 2196 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.621 | 0.205 | - | - | - |
| allocations | 6.030M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.030M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.449G | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 34.00 | 0.0000113 | 0.000000113 | 0.0000113 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 2335 | 0.000778 | 0.00000778 | 0.0000440 | 0.00000734 | - | - | - |
| cycles | - | - | - | 46.96k | 8783 | - | - | - |
| instructions | - | - | - | 28.13k | 6208 | - | - | - |
| cache references | - | - | - | 26.07 | 12.47 | - | - | - |
| cache misses | - | - | - | 4.771 | 0.00209 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0963 | 0.0000142 | - | - | - |
| data TLB read misses | - | - | - | 0.0275 | 0.00770 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 38.11M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 38.11M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message

Run 3's profile:

| profile: the window, 30.0 s, 3002637 messages, 300263700 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.8G | 229.4k | 2294 | 9974 | 2194 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.062 | 0.172 | - | - | - |
| allocations | 6.034M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.034M | 2.010 | 0.0201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.450G | 483.0 | 4.830 | 483.0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 2405 | 0.000801 | 0.00000801 | 0.0000530 | 0.00000748 | - | - | - |
| cycles | - | - | - | 47.36k | 8846 | - | - | - |
| instructions | - | - | - | 28.57k | 6027 | - | - | - |
| cache references | - | - | - | 21.46 | 13.12 | - | - | - |
| cache misses | - | - | - | 4.611 | 0.00157 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0930 | 0.0000123 | - | - | - |
| data TLB read misses | - | - | - | 0.0248 | 0.00810 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 38.26M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 38.26M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
