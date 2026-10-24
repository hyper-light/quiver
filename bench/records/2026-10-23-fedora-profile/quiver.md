# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 10 over 10 threads, which wait when there's nothing to take; window 10 s after 2 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 10 × wait | 124861 | 9988 | 0 | 0 | 1248610 | 998040 | 0 | 9.5 | 16.9 | 46.1 | 93.2 | 3381.4 | 0 | 0 | 10/10 | 22.2 | 12.5 | - | 18.18 / 17.15 |
| best | quiver | 10 × wait | 124861 | 9988 | 0 | 0 | 1248610 | 998040 | 0 | 9.5 | 16.9 | 46.1 | 93.2 | 3381.4 | 0 | 0 | 10/10 | 22.2 | 12.5 | - | 18.18 / 17.15 |

Run 1's profile:

| profile: the window, 10.0 s, 99803 messages, 998040 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 17.74G | 177.8k | 17.78k | 99.86k | 7793 | - | - | - |
| waiting for a CPU, ns | - | - | - | 11.36 | 49.17 | - | - | - |
| allocations | 1.391M | 13.94 | 1.394 | 4.010 | 0.993 | - | - | - |
| reallocations | 299.4k | 3.000 | 0.300 | 3.000 | 0 | - | - | - |
| reallocations that moved their block | 226.7k | 2.271 | 0.227 | 2.271 | 0 | - | - | - |
| frees | 1.345M | 13.47 | 1.347 | 3.547 | 0.993 | - | - | - |
| bytes allocated | 159.4M | 1597 | 159.7 | 1041 | 55.58 | - | - | - |
| minor page faults | 5092 | 0.0510 | 0.00510 | 0.0510 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 1.124M | 11.26 | 1.126 | 0.0132 | 1.124 | - | - | - |
| involuntary context switches | 210.0 | 0.00210 | 0.000210 | 0.000641 | 0.000146 | - | - | - |
| cycles | - | - | - | 704.0k | 12.89k | - | - | - |
| instructions | - | - | - | 542.3k | 2870 | - | - | - |
| cache references | - | - | - | 125.8 | 89.03 | - | - | - |
| cache misses | - | - | - | 13.92 | 1.811 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.251 | 0.0788 | - | - | - |
| data TLB read misses | - | - | - | 17.38 | 5.618 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 36.51M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 36.51M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 10 s after 2 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 124861 | 9989 | 0 | 0 | 12486100 | 9980400 | 0 | 28.4 | 58.4 | 101.4 | 149.5 | 2072.9 | 0 | 0 | 100/100 | 36.4 | 12.4 | - | 15.20 / 14.77 |
| best | quiver | 22 × wait | 124861 | 9989 | 0 | 0 | 12486100 | 9980400 | 0 | 28.4 | 58.4 | 101.4 | 149.5 | 2072.9 | 0 | 0 | 100/100 | 36.4 | 12.4 | - | 15.20 / 14.77 |

Run 1's profile:

| profile: the window, 10.0 s, 99804 messages, 9980384 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 29.11G | 291.6k | 2916 | 99.69k | 1920 | - | - | - |
| waiting for a CPU, ns | - | - | - | 38.41 | 36.07 | - | - | - |
| allocations | 2.112M | 21.16 | 0.212 | 4.010 | 0.171 | - | - | - |
| reallocations | 299.4k | 3.000 | 0.0300 | 3.000 | 0 | - | - | - |
| reallocations that moved their block | 226.6k | 2.270 | 0.0227 | 2.270 | 0 | - | - | - |
| frees | 2.065M | 20.70 | 0.207 | 3.547 | 0.171 | - | - | - |
| bytes allocated | 199.7M | 2001 | 20.01 | 1041 | 9.603 | - | - | - |
| minor page faults | 5091 | 0.0510 | 0.000510 | 0.0510 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 2.871M | 28.77 | 0.288 | 0.0136 | 0.288 | - | - | - |
| involuntary context switches | 3471 | 0.0348 | 0.000348 | 0.00613 | 0.000286 | - | - | - |
| cycles | - | - | - | 679.3k | 2797 | - | - | - |
| instructions | - | - | - | 516.5k | 1295 | - | - | - |
| cache references | - | - | - | 236.6 | 27.93 | - | - | - |
| cache misses | - | - | - | 14.24 | 0.345 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.523 | 0.00780 | - | - | - |
| data TLB read misses | - | - | - | 48.16 | 0.852 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 36.87M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 36.87M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 10 over 10 threads, which poll without waiting; window 10 s after 2 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 10 × busy | 124861 | 9988 | 0 | 0 | 1248610 | 998040 | 0 | 1.3 | 1.8 | 4.7 | 43.5 | 2843.5 | 0 | 0 | 10/10 | 136.7 | 12.5 | - | 18.21 / 19.22 |
| best | quiver | 10 × busy | 124861 | 9988 | 0 | 0 | 1248610 | 998040 | 0 | 1.3 | 1.8 | 4.7 | 43.5 | 2843.5 | 0 | 0 | 10/10 | 136.7 | 12.5 | - | 18.21 / 19.22 |

Run 1's profile:

| profile: the window, 10.0 s, 99804 messages, 998040 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 109.4G | 1.096M | 109.6k | 99.86k | 99.63k | - | - | - |
| waiting for a CPU, ns | - | - | - | 40.57 | 281.5 | - | - | - |
| allocations | 400.2k | 4.010 | 0.401 | 4.010 | 0 | - | - | - |
| reallocations | 299.4k | 3.000 | 0.300 | 3.000 | 0 | - | - | - |
| reallocations that moved their block | 226.7k | 2.271 | 0.227 | 2.271 | 0 | - | - | - |
| frees | 354.0k | 3.547 | 0.355 | 3.547 | 0 | - | - | - |
| bytes allocated | 103.9M | 1041 | 104.1 | 1041 | 0 | - | - | - |
| minor page faults | 5092 | 0.0510 | 0.00510 | 0.0510 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 7551 | 0.0757 | 0.00757 | 0.000321 | 0.00753 | - | - | - |
| cycles | - | - | - | 733.5k | 453.2k | - | - | - |
| instructions | - | - | - | 581.2k | 314.6k | - | - | - |
| cache references | - | - | - | 77.81 | 655.5 | - | - | - |
| cache misses | - | - | - | 12.13 | 0.578 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.153 | 0.00992 | - | - | - |
| data TLB read misses | - | - | - | 0.126 | 0.0381 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 36.37M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 36.37M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 22 threads, which poll without waiting; window 10 s after 2 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 124861 | 9989 | 0 | 0 | 12486100 | 9980400 | 0 | 2.1 | 2687.0 | 5308.4 | 6488.1 | 37009.2 | 0 | 0 | 100/100 | 236.1 | 8.5 | - | 28.73 / 29.54 |
| best | quiver | 22 × busy | 124861 | 9989 | 0 | 0 | 12486100 | 9980400 | 0 | 2.1 | 2687.0 | 5308.4 | 6488.1 | 37009.2 | 0 | 0 | 100/100 | 236.1 | 8.5 | - | 28.73 / 29.54 |

Run 1's profile:

| profile: the window, 10.0 s, 99803 messages, 9980822 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 189.2G | 1.896M | 18.96k | 66.20k | 18.30k | - | - | - |
| waiting for a CPU, ns | - | - | - | 33.77k | 3686 | - | - | - |
| allocations | 400.2k | 4.010 | 0.0401 | 4.010 | 0 | - | - | - |
| reallocations | 299.4k | 3.000 | 0.0300 | 3.000 | 0 | - | - | - |
| reallocations that moved their block | 226.4k | 2.268 | 0.0227 | 2.268 | 0 | - | - | - |
| frees | 354.0k | 3.547 | 0.0355 | 3.547 | 0 | - | - | - |
| bytes allocated | 103.4M | 1036 | 10.36 | 1036 | 0 | - | - | - |
| minor page faults | 5091 | 0.0510 | 0.000510 | 0.0510 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 1.000 | 0.0000100 | 0.000000100 | 0 | 0.000000100 | - | - | - |
| involuntary context switches | 15.29k | 0.153 | 0.00153 | 0.0115 | 0.00142 | - | - | - |
| cycles | - | - | - | 469.8k | 69.09k | - | - | - |
| instructions | - | - | - | 372.5k | 46.41k | - | - | - |
| cache references | - | - | - | 57.04 | 83.87 | - | - | - |
| cache misses | - | - | - | 9.329 | 0.0391 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.121 | 0.00129 | - | - | - |
| data TLB read misses | - | - | - | 0.0439 | 0.0238 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 37.15M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 37.15M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message
