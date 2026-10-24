- broker: public.ecr.aws/docker/library/nats:2.12-alpine@sha256:b270f5e2428354c0335612694d7dd2fb588148e567a5757fdff325ef9c9332e6, CPUs 8-15
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 18.9 | 53.2 | 101.4 | 149.5 | 325.2 | 0 | 0 | 1/1 | 37.4 | 34.8 | 5.1 | 1.54 / 1.69 |
| best | nats | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 18.9 | 53.2 | 101.4 | 149.5 | 325.2 | 0 | 0 | 1/1 | 37.4 | 34.8 | 5.1 | 1.54 / 1.69 |

Run 1's profile:

| profile: the window, 30.0 s, 299500 messages, 299499 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 31.49G | 105.2k | 105.2k | 98.26k | 6892 | 4.353G | 14.29k | 14.29k |
| waiting for a CPU, ns | - | - | - | 7.668 | 5.900 | - | - | - |
| allocations | 899.0k | 3.002 | 3.002 | 2.000 | 1.002 | 922.1k | 3.027 | 3.027 |
| reallocations | 1.000 | 0.00000334 | 0.00000334 | 0.00000334 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.00000334 | 0.00000334 | 0 | - | - | - |
| frees | 899.0k | 3.002 | 3.002 | 2.000 | 1.002 | 942.8k | 3.095 | 3.095 |
| bytes allocated | 98.66M | 329.4 | 329.4 | 280.0 | 49.41 | 69.62M | 228.5 | 228.5 |
| minor page faults | 3.000 | 0.0000100 | 0.0000100 | 0.00000334 | 0 | 1599 | 0.00525 | 0.00525 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 264.3k | 0.882 | 0.882 | 0 | 0.882 | - | - | - |
| involuntary context switches | 292.0 | 0.000975 | 0.000975 | 0.000895 | 0.0000801 | - | - | - |
| cycles | - | - | - | 370.3k | 11.82k | - | - | - |
| instructions | - | - | - | 720.0k | 14.18k | - | - | - |
| cache references | - | - | - | 1.651 | 16.21 | - | - | - |
| cache misses | - | - | - | 0.195 | 0.995 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0136 | 0.0196 | - | - | - |
| data TLB read misses | - | - | - | 0.0367 | 0.315 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.0137 | 0.0137 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 23.00 | 0.0000755 | 0.0000755 |
| collectors' pauses, ns | - | - | - | - | - | 721.6k | 2.369 | 2.369 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.668M | - | - | - | - | 30.90M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.668M | - | - | - | - | 29.07M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 4.006M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304622 messages, 304623 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3063606-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3063606-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 141.3 | 249.9 | 663.6 | 1065.0 | 3195.3 | 0 | 0 | 100/100 | 217.1 | 34.6 | 139.4 | 3.08 / 6.87 |
| best | nats | 22 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 141.3 | 249.9 | 663.6 | 1065.0 | 3195.3 | 0 | 0 | 100/100 | 217.1 | 34.6 | 139.4 | 3.08 / 6.87 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 29949502 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 183.4G | 612.4k | 6124 | 97.74k | 5147 | 119.8G | 393.2k | 3932 |
| waiting for a CPU, ns | - | - | - | 31.53 | 23.45 | - | - | - |
| allocations | 30.73M | 102.6 | 1.026 | 2.000 | 1.006 | 49.36M | 162.0 | 1.620 |
| reallocations | 2.000 | 0.00000668 | 0.000000067 | 0.00000668 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | - | - | - |
| frees | 30.73M | 102.6 | 1.026 | 2.000 | 1.006 | 49.34M | 162.0 | 1.620 |
| bytes allocated | 3.739G | 12.48k | 124.8 | 280.0 | 122.0 | 1.569G | 5151 | 51.51 |
| minor page faults | 53.00 | 0.000177 | 0.00000177 | 0.00000334 | 0.00000164 | 45.54k | 0.149 | 0.00149 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 13.79M | 46.04 | 0.460 | 0 | 0.460 | - | - | - |
| involuntary context switches | 19.80k | 0.0661 | 0.000661 | 0.00262 | 0.000635 | - | - | - |
| cycles | - | - | - | 496.8k | 11.98k | - | - | - |
| instructions | - | - | - | 935.6k | 14.05k | - | - | - |
| cache references | - | - | - | 11.45 | 20.19 | - | - | - |
| cache misses | - | - | - | 1.874 | 0.857 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0327 | 0.0170 | - | - | - |
| data TLB read misses | - | - | - | 0.146 | 0.711 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.0137 | 0.000137 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 944.0 | 0.00310 | 0.0000310 |
| collectors' pauses, ns | - | - | - | - | - | refused: 944 collections in the span, past the 256 pauses Go keeps | refused | refused |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 22.30M | - | - | - | - | 40.56M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 22.30M | - | - | - | - | 40.07M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 5.546M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304622 messages, 30462218 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3069667-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3069667-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 111941 | 3107 | 242649 | 0 | 111941000 | 61877000 | 237620000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 245.1 | 2.3 | 213.2 | 5.85 / 10.21 |
| best | nats | 22 × wait | 111941 | 3107 | 242649 | 0 | 111941000 | 61877000 | 237620000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 245.1 | 2.3 | 213.2 | 5.85 / 10.21 |

Run 1's profile:

| profile: the window, 30.0 s, 84832 messages, 82849926 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 179.3G | 2.114M | 2164 | 586.9 | 2164 | 160.4G | 1.891M | 1904 |
| waiting for a CPU, ns | - | - | - | 0.580 | 4.810 | - | - | - |
| allocations | 84.13M | 991.7 | 1.015 | 2.017 | 1.013 | 79.18M | 933.4 | 0.940 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 84.13M | 991.7 | 1.015 | 2.017 | 1.013 | 79.17M | 933.2 | 0.940 |
| bytes allocated | 20.22G | 238.3k | 244.0 | 368.9 | 243.7 | 3.663G | 43.18k | 43.48 |
| minor page faults | 35.00 | 0.000413 | 0.000000422 | 0 | 0.000000386 | 17.08k | 0.201 | 0.000203 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 13.96M | 164.5 | 0.168 | 0.000212 | 0.168 | - | - | - |
| involuntary context switches | 11.55k | 0.136 | 0.000139 | 0.0000236 | 0.000139 | - | - | - |
| cycles | - | - | - | 3941 | 6667 | - | - | - |
| instructions | - | - | - | 8166 | 8899 | - | - | - |
| cache references | - | - | - | 17.43 | 37.68 | - | - | - |
| cache misses | - | - | - | 8.979 | 21.66 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.464 | 0.811 | - | - | - |
| data TLB read misses | - | - | - | 0.0772 | 1.516 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.0483 | 0.0000494 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 293.0 | 0.00345 | 0.00000348 |
| collectors' pauses, ns | - | - | - | - | - | refused: 293 collections in the span, past the 256 pauses Go keeps | refused | refused |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 120.3M | - | - | - | - | 113.4M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 120.3M | - | - | - | - | 112.0M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 23.14M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 84832 messages, 84252831 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3076392-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3076392-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 18.2 | 23.8 | 51.7 | 139.3 | 1109.8 | 0 | 0 | 1/1 | 40.8 | 30.4 | 22.5 | 3.96 / 3.13 |
| best | nats | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 18.2 | 23.8 | 51.7 | 139.3 | 1109.8 | 0 | 0 | 1/1 | 40.8 | 30.4 | 22.5 | 3.96 / 3.13 |

Run 1's profile:

| profile: the window, 30.0 s, 2998270 messages, 2998272 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 34.55G | 11.52k | 11.52k | 8563 | 2961 | 19.31G | 6334 | 6334 |
| waiting for a CPU, ns | - | - | - | 0.797 | 1.127 | - | - | - |
| allocations | 9.013M | 3.006 | 3.006 | 2.000 | 1.006 | 8.866M | 2.908 | 2.908 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 9.013M | 3.006 | 3.006 | 2.000 | 1.006 | 8.862M | 2.907 | 2.907 |
| bytes allocated | 1.202G | 400.8 | 400.8 | 280.0 | 120.8 | 1.008G | 330.6 | 330.6 |
| minor page faults | 3.000 | 0.00000100 | 0.00000100 | 0 | 0 | 17.73k | 0.00582 | 0.00582 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.669M | 0.557 | 0.557 | 0 | 0.557 | - | - | - |
| involuntary context switches | 514.0 | 0.000171 | 0.000171 | 0.000144 | 0.0000273 | - | - | - |
| cycles | - | - | - | 29.25k | 7685 | - | - | - |
| instructions | - | - | - | 56.75k | 10.41k | - | - | - |
| cache references | - | - | - | 0.500 | 2.536 | - | - | - |
| cache misses | - | - | - | 0.0494 | 0.215 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00272 | 0.00233 | - | - | - |
| data TLB read misses | - | - | - | 0.0443 | 0.0576 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.00137 | 0.00137 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 417.0 | 0.000137 | 0.000137 |
| collectors' pauses, ns | - | - | - | - | - | refused: 417 collections in the span, past the 256 pauses Go keeps | refused | refused |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.844M | - | - | - | - | 38.03M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.844M | - | - | - | - | 37.18M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 6.799M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3048311 messages, 3048311 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3087733-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3087733-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 887734 | 24806 | 2659809 | 0 | 88773400 | 38836300 | 260991100 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 235.5 | 0.8 | 185.1 | 7.93 / 11.92 |
| best | nats | 22 × wait | 887734 | 24806 | 2659809 | 0 | 88773400 | 38836300 | 260991100 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 235.5 | 0.8 | 185.1 | 7.93 / 11.92 |

Run 1's profile:

| profile: the window, 30.0 s, 732640 messages, 73480409 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 194.8G | 265.9k | 2651 | 568.1 | 2646 | 155.9G | 208.5k | 2087 |
| waiting for a CPU, ns | - | - | - | 0.219 | 6.945 | - | - | - |
| allocations | 75.87M | 103.6 | 1.033 | 2.016 | 1.012 | 78.22M | 104.6 | 1.047 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 75.87M | 103.6 | 1.033 | 2.016 | 1.012 | 78.20M | 104.6 | 1.047 |
| bytes allocated | 16.98G | 23.17k | 231.1 | 364.8 | 227.4 | 3.181G | 4255 | 42.58 |
| minor page faults | 100.0 | 0.000136 | 0.00000136 | 0 | 0.00000131 | 62.03k | 0.0830 | 0.000830 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 16.14M | 22.03 | 0.220 | 0.000194 | 0.220 | - | - | - |
| involuntary context switches | 13.80k | 0.0188 | 0.000188 | 0.0000177 | 0.000188 | - | - | - |
| cycles | - | - | - | 3712 | 7028 | - | - | - |
| instructions | - | - | - | 7622 | 9689 | - | - | - |
| cache references | - | - | - | 13.26 | 8.594 | - | - | - |
| cache misses | - | - | - | 5.859 | 0.503 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.231 | 0.0105 | - | - | - |
| data TLB read misses | - | - | - | 0.0593 | 0.242 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.00559 | 0.0000557 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 851.0 | 0.00114 | 0.0000114 |
| collectors' pauses, ns | - | - | - | - | - | refused: 851 collections in the span, past the 256 pauses Go keeps | refused | refused |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 23.53M | - | - | - | - | 47.35M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 23.53M | - | - | - | - | 46.42M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 9.224M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 747664 messages, 74712619 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3093042-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3093042-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 102823 | 2822 | 3444720 | 0 | 102823000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 245.6 | 0.4 | 218.5 | 9.02 / 14.05 |
| best | nats | 22 × wait | 102823 | 2822 | 3444720 | 0 | 102823000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 245.6 | 0.4 | 218.5 | 9.02 / 14.05 |

Run 1's profile:

| profile: the window, 30.0 s, 74688 messages, 74610611 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 177.4G | 2.375M | 2378 | 596.5 | 2377 | 161.0G | 2.155M | 2122 |
| waiting for a CPU, ns | - | - | - | 0.349 | 97.92 | - | - | - |
| allocations | 75.79M | 1015 | 1.016 | 2.016 | 1.014 | 66.76M | 893.8 | 0.880 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 75.79M | 1015 | 1.016 | 2.016 | 1.014 | 66.75M | 893.7 | 0.880 |
| bytes allocated | 18.77G | 251.3k | 251.5 | 363.5 | 251.1 | 3.293G | 44.09k | 43.40 |
| minor page faults | 8217 | 0.110 | 0.000110 | 0 | 0.000110 | 12.68k | 0.170 | 0.000167 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 11.04M | 147.8 | 0.148 | 0.000187 | 0.148 | - | - | - |
| involuntary context switches | 39.59k | 0.530 | 0.000531 | 0.0000268 | 0.000531 | - | - | - |
| cycles | - | - | - | 3643 | 7334 | - | - | - |
| instructions | - | - | - | 7582 | 8744 | - | - | - |
| cache references | - | - | - | 14.81 | 66.66 | - | - | - |
| cache misses | - | - | - | 8.447 | 33.16 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.311 | 1.690 | - | - | - |
| data TLB read misses | - | - | - | 0.0563 | 4.118 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.274 | 0.000274 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 228.0 | 0.00305 | 0.00000301 |
| collectors' pauses, ns | - | - | - | - | - | 17.02M | 227.9 | 0.224 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 134.0M | - | - | - | - | 120.0M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 122.7M | - | - | - | - | 117.0M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 27.99M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 74688 messages, 75861128 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3099790-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3099790-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 15.1 | 41.5 | 66.6 | 103.4 | 247.2 | 0 | 0 | 1/1 | 70.1 | 34.8 | 5.2 | 6.07 / 4.56 |
| best | nats | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 15.1 | 41.5 | 66.6 | 103.4 | 247.2 | 0 | 0 | 1/1 | 70.1 | 34.8 | 5.2 | 6.07 / 4.56 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 299499 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.19G | 197.6k | 197.6k | 98.30k | 99.32k | 4.479G | 14.71k | 14.71k |
| waiting for a CPU, ns | - | - | - | 2.166 | 9.718 | - | - | - |
| allocations | 898.8k | 3.001 | 3.001 | 2.000 | 1.001 | 944.3k | 3.100 | 3.100 |
| reallocations | 1.000 | 0.00000334 | 0.00000334 | 0.00000334 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 898.8k | 3.001 | 3.001 | 2.000 | 1.001 | 952.6k | 3.127 | 3.127 |
| bytes allocated | 96.04M | 320.7 | 320.7 | 280.0 | 40.65 | 71.50M | 234.7 | 234.7 |
| minor page faults | 4.000 | 0.0000134 | 0.0000134 | 0.00000334 | 0 | 1065 | 0.00350 | 0.00350 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 336.0 | 0.00112 | 0.00112 | 0.000401 | 0.000721 | - | - | - |
| cycles | - | - | - | 368.5k | 399.3k | - | - | - |
| instructions | - | - | - | 714.7k | 693.1k | - | - | - |
| cache references | - | - | - | 1.622 | 1.620 | - | - | - |
| cache misses | - | - | - | 0.288 | 0.284 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0258 | 0.0218 | - | - | - |
| data TLB read misses | - | - | - | 0.0313 | 0.000731 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.0137 | 0.0137 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 19.00 | 0.0000624 | 0.0000624 |
| collectors' pauses, ns | - | - | - | - | - | 456.0k | 1.497 | 1.497 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.705M | - | - | - | - | 40.12M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.705M | - | - | - | - | 39.37M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 11.07M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304614 messages, 304614 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3119499-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3119499-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 196.6 | 471.0 | 1376.3 | 3276.8 | 10471.5 | 0 | 0 | 100/100 | 782.8 | 34.1 | 104.9 | 17.33 / 24.55 |
| best | nats | 22 × busy | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 196.6 | 471.0 | 1376.3 | 3276.8 | 10471.5 | 0 | 0 | 100/100 | 782.8 | 34.1 | 104.9 | 17.33 / 24.55 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 29949444 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 660.9G | 2.207M | 22.07k | 96.13k | 21.11k | 89.05G | 292.3k | 2923 |
| waiting for a CPU, ns | - | - | - | 4.293 | 52.17 | - | - | - |
| allocations | 30.84M | 103.0 | 1.030 | 2.000 | 1.010 | 39.56M | 129.8 | 1.298 |
| reallocations | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.000000033 | 0.00000334 | 0 | - | - | - |
| frees | 30.84M | 103.0 | 1.030 | 2.000 | 1.010 | 39.62M | 130.1 | 1.301 |
| bytes allocated | 5.514G | 18.41k | 184.1 | 280.0 | 181.3 | 1.510G | 4957 | 49.57 |
| minor page faults | 68.00 | 0.000227 | 0.00000227 | 0.00000334 | 0.00000214 | 20.02k | 0.0657 | 0.000657 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.000 | 0.00000334 | 0.000000033 | 0 | 0 | - | - | - |
| involuntary context switches | 9382 | 0.0313 | 0.000313 | 0.000658 | 0.000307 | - | - | - |
| cycles | - | - | - | 181.7k | 55.11k | - | - | - |
| instructions | - | - | - | 330.2k | 83.83k | - | - | - |
| cache references | - | - | - | 2.319 | 1.577 | - | - | - |
| cache misses | - | - | - | 1.228 | 0.541 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.00547 | - | - | - |
| data TLB read misses | - | - | - | 0.000404 | 0.0162 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.0137 | 0.000137 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 337.0 | 0.00111 | 0.0000111 |
| collectors' pauses, ns | - | - | - | - | - | refused: 337 collections in the span, past the 256 pauses Go keeps | refused | refused |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 21.86M | - | - | - | - | 50.25M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 21.86M | - | - | - | - | 49.41M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 10.14M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304632 messages, 30462955 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3126534-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3126534-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 103659 | 2758 | 250931 | 0 | 103659000 | 53595000 | 245902000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 904.8 | 2.4 | 224.7 | 27.86 / 29.92 |
| best | nats | 22 × busy | 103659 | 2758 | 250931 | 0 | 103659000 | 53595000 | 245902000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 904.8 | 2.4 | 224.7 | 27.86 / 29.92 |

Run 1's profile:

| profile: the window, 30.0 s, 67728 messages, 71327586 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 621.4G | 9.174M | 8711 | 655.6 | 8711 | 157.5G | 2.148M | 2171 |
| waiting for a CPU, ns | - | - | - | 693.2 | 153.7 | - | - | - |
| allocations | 72.42M | 1069 | 1.015 | 2.016 | 1.013 | 68.50M | 934.1 | 0.944 |
| reallocations | 21.00 | 0.000310 | 0.000000294 | 0 | 0.000000294 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 21.00 | 0.000310 | 0.000000294 | 0 | 0.000000294 | - | - | - |
| frees | 72.42M | 1069 | 1.015 | 2.016 | 1.013 | 68.34M | 932.0 | 0.942 |
| bytes allocated | 17.44G | 257.4k | 244.5 | 360.9 | 244.1 | 3.214G | 43.83k | 44.29 |
| minor page faults | 19.75k | 0.292 | 0.000277 | 0 | 0.000277 | 21.60k | 0.295 | 0.000298 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 12.00 | 0.000177 | 0.000000168 | 0.000177 | 0 | - | - | - |
| involuntary context switches | 55.24k | 0.816 | 0.000775 | 0.000177 | 0.000774 | - | - | - |
| cycles | - | - | - | 1730 | 39.47k | - | - | - |
| instructions | - | - | - | 3536 | 58.16k | - | - | - |
| cache references | - | - | - | 8.345 | 25.56 | - | - | - |
| cache misses | - | - | - | 5.160 | 16.85 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00461 | 0.505 | - | - | - |
| data TLB read misses | - | - | - | 0.0176 | 0.703 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.302 | 0.000287 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 202.0 | 0.00275 | 0.00000278 |
| collectors' pauses, ns | - | - | - | - | - | 25.64M | 349.6 | 0.353 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 134.1M | - | - | - | - | 127.4M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 133.2M | - | - | - | - | 120.6M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 35.55M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 73328 messages, 72556324 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3137982-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3137982-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 14.6 | 18.9 | 39.4 | 90.1 | 527.1 | 0 | 0 | 1/1 | 64.1 | 30.4 | 22.5 | 10.41 / 7.19 |
| best | nats | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 14.6 | 18.9 | 39.4 | 90.1 | 527.1 | 0 | 0 | 1/1 | 64.1 | 30.4 | 22.5 | 10.41 / 7.19 |

Run 1's profile:

| profile: the window, 30.0 s, 2998266 messages, 2998270 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 54.16G | 18.06k | 18.06k | 8562 | 9501 | 19.31G | 6334 | 6334 |
| waiting for a CPU, ns | - | - | - | 0.891 | 0.753 | - | - | - |
| allocations | 9.010M | 3.005 | 3.005 | 2.000 | 1.005 | 8.881M | 2.913 | 2.913 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 9.010M | 3.005 | 3.005 | 2.000 | 1.005 | 8.884M | 2.914 | 2.914 |
| bytes allocated | 1.164G | 388.3 | 388.3 | 280.0 | 108.3 | 1.002G | 328.8 | 328.8 |
| minor page faults | 3.000 | 0.00000100 | 0.00000100 | 0 | 0 | 6218 | 0.00204 | 0.00204 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 684.0 | 0.000228 | 0.000228 | 0.000145 | 0.0000830 | - | - | - |
| cycles | - | - | - | 29.32k | 36.59k | - | - | - |
| instructions | - | - | - | 56.65k | 63.87k | - | - | - |
| cache references | - | - | - | 0.379 | 0.230 | - | - | - |
| cache misses | - | - | - | 0.0378 | 0.0326 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00211 | 0.00211 | - | - | - |
| data TLB read misses | - | - | - | 0.0346 | 0.00260 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.00137 | 0.00137 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 235.0 | 0.0000771 | 0.0000771 |
| collectors' pauses, ns | - | - | - | - | - | 8.681M | 2.848 | 2.848 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.513M | - | - | - | - | 42.10M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.513M | - | - | - | - | 40.69M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 8.544M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3048337 messages, 3048334 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3149030-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3149030-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 938430 | 26305 | 2609113 | 0 | 93843000 | 43905900 | 255921500 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 745.2 | 0.9 | 170.3 | 20.08 / 25.00 |
| best | nats | 22 × busy | 938430 | 26305 | 2609113 | 0 | 93843000 | 43905900 | 255921500 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 745.2 | 0.9 | 170.3 | 20.08 / 25.00 |

Run 1's profile:

| profile: the window, 30.0 s, 779968 messages, 78452836 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 619.5G | 794.3k | 7897 | 590.4 | 7891 | 142.5G | 178.2k | 1787 |
| waiting for a CPU, ns | - | - | - | 273.9 | 25.36 | - | - | - |
| allocations | 81.08M | 104.0 | 1.034 | 2.017 | 1.013 | 73.29M | 91.62 | 0.919 |
| reallocations | 13.00 | 0.0000167 | 0.000000166 | 0 | 0.000000166 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 13.00 | 0.0000167 | 0.000000166 | 0 | 0.000000166 | - | - | - |
| frees | 81.08M | 104.0 | 1.034 | 2.017 | 1.013 | 73.17M | 91.47 | 0.918 |
| bytes allocated | 19.46G | 24.95k | 248.0 | 366.4 | 244.4 | 3.543G | 4429 | 44.44 |
| minor page faults | 3566 | 0.00457 | 0.0000455 | 0 | 0.0000454 | 38.53k | 0.0482 | 0.000483 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 157.0 | 0.000201 | 0.00000200 | 0.000201 | 0 | - | - | - |
| involuntary context switches | 13.93k | 0.0179 | 0.000178 | 0.0000833 | 0.000177 | - | - | - |
| cycles | - | - | - | 2034 | 23.08k | - | - | - |
| instructions | - | - | - | 3537 | 35.64k | - | - | - |
| cache references | - | - | - | 6.436 | 1.366 | - | - | - |
| cache misses | - | - | - | 2.741 | 0.263 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.151 | 0.00275 | - | - | - |
| data TLB read misses | - | - | - | 0.0263 | 0.0241 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.00525 | 0.0000522 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 585.0 | 0.000731 | 0.00000734 |
| collectors' pauses, ns | - | - | - | - | - | refused: 585 collections in the span, past the 256 pauses Go keeps | refused | refused |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 39.35M | - | - | - | - | 58.41M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 39.35M | - | - | - | - | 53.23M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 15.40M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 799936 messages, 79738868 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3153330-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3153330-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 101095 | 2679 | 3446448 | 0 | 101095000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 895.6 | 0.4 | 226.1 | 28.26 / 35.64 |
| best | nats | 22 × busy | 101095 | 2679 | 3446448 | 0 | 101095000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 895.6 | 0.4 | 226.1 | 28.26 / 35.64 |

Run 1's profile:

| profile: the window, 30.0 s, 67728 messages, 69230756 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 613.6G | 9.060M | 8864 | 670.1 | 8863 | 158.0G | 2.161M | 2244 |
| waiting for a CPU, ns | - | - | - | 735.6 | 260.1 | - | - | - |
| allocations | 70.30M | 1038 | 1.015 | 2.016 | 1.013 | 65.90M | 901.5 | 0.936 |
| reallocations | 4.000 | 0.0000591 | 0.000000058 | 0 | 0.000000058 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 4.000 | 0.0000591 | 0.000000058 | 0 | 0.000000058 | - | - | - |
| frees | 70.30M | 1038 | 1.015 | 2.016 | 1.013 | 65.85M | 900.8 | 0.935 |
| bytes allocated | 17.01G | 251.1k | 245.7 | 364.5 | 245.3 | 3.109G | 42.54k | 44.16 |
| minor page faults | 26.41k | 0.390 | 0.000381 | 0 | 0.000382 | 17.99k | 0.246 | 0.000256 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 14.00 | 0.000207 | 0.000000202 | 0.000192 | 0.000000014 | - | - | - |
| involuntary context switches | 359.6k | 5.309 | 0.00519 | 0.000694 | 0.00520 | - | - | - |
| cycles | - | - | - | 1753 | 37.46k | - | - | - |
| instructions | - | - | - | 3535 | 53.99k | - | - | - |
| cache references | - | - | - | 8.392 | 39.46 | - | - | - |
| cache misses | - | - | - | 5.245 | 24.38 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00413 | 0.889 | - | - | - |
| data TLB read misses | - | - | - | 0.0247 | 1.384 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 53.25k | 0.786 | 0.000769 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 187.0 | 0.00256 | 0.00000266 |
| collectors' pauses, ns | - | - | - | - | - | 16.88M | 230.9 | 0.240 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 132.0M | - | - | - | - | 130.0M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 126.5M | - | - | - | - | 115.6M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 30.52M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 73104 messages, 70407740 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3162849-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3162849-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 26.4 | 58.9 | 119.8 | 174.1 | 1387.5 | 0 | 0 | 1/1 | 38.0 | 34.7 | 5.6 | 22.79 / 14.67 |
| best | nats | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 26.4 | 58.9 | 119.8 | 174.1 | 1387.5 | 0 | 0 | 1/1 | 38.0 | 34.7 | 5.6 | 22.79 / 14.67 |

Run 1's profile:

| profile: the window, 30.0 s, 299585 messages, 299586 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 32.15G | 107.3k | 107.3k | 97.98k | 9333 | 4.853G | 15.93k | 15.94k |
| waiting for a CPU, ns | - | - | - | 3.745 | 7.266 | - | - | - |
| allocations | 899.2k | 3.002 | 3.002 | 2.000 | 1.002 | 955.1k | 3.136 | 3.136 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 899.2k | 3.002 | 3.002 | 2.000 | 1.002 | 928.9k | 3.050 | 3.050 |
| bytes allocated | 99.18M | 331.0 | 331.0 | 280.0 | 51.05 | 73.54M | 241.4 | 241.4 |
| minor page faults | 5.000 | 0.0000167 | 0.0000167 | 0.00000334 | 0 | 1007 | 0.00331 | 0.00331 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 254.5k | 0.849 | 0.849 | 0 | 0.849 | - | - | - |
| involuntary context switches | 152.0 | 0.000507 | 0.000507 | 0.000421 | 0.0000868 | - | - | - |
| cycles | - | - | - | 345.1k | 16.07k | - | - | - |
| instructions | - | - | - | 668.1k | 18.74k | - | - | - |
| cache references | - | - | - | 5.244 | 45.57 | - | - | - |
| cache misses | - | - | - | 3.548 | 7.515 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.419 | 0.219 | - | - | - |
| data TLB read misses | - | - | - | 0.0503 | 0.613 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 69.63k | 0.232 | 0.232 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 13.00 | 0.0000427 | 0.0000427 |
| collectors' pauses, ns | - | - | - | - | - | 357.7k | 1.174 | 1.174 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.742M | - | - | - | - | 43.94M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.742M | - | - | - | - | 41.55M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 12.06M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304580 messages, 304577 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3174914-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3174914-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 194.6 | 340.0 | 909.3 | 1884.2 | 12017.0 | 0 | 0 | 100/100 | 191.8 | 34.1 | 101.5 | 10.63 / 18.30 |
| best | nats | 22 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 194.6 | 340.0 | 909.3 | 1884.2 | 12017.0 | 0 | 0 | 100/100 | 191.8 | 34.1 | 101.5 | 10.63 / 18.30 |

Run 1's profile:

| profile: the window, 30.0 s, 299583 messages, 29958084 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 172.0G | 574.3k | 5743 | 96.58k | 4777 | 91.70G | 301.1k | 3011 |
| waiting for a CPU, ns | - | - | - | 37.40 | 250.8 | - | - | - |
| allocations | 30.84M | 102.9 | 1.029 | 2.000 | 1.009 | 39.68M | 130.3 | 1.303 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 30.84M | 102.9 | 1.029 | 2.000 | 1.009 | 39.56M | 129.9 | 1.299 |
| bytes allocated | 5.374G | 17.94k | 179.4 | 280.0 | 176.6 | 1.462G | 4800 | 48.00 |
| minor page faults | 4.000 | 0.0000134 | 0.000000134 | 0 | 0 | 8423 | 0.0277 | 0.000277 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 9.884M | 32.99 | 0.330 | 0 | 0.330 | - | - | - |
| involuntary context switches | 71.69k | 0.239 | 0.00239 | 0.00236 | 0.00237 | - | - | - |
| cycles | - | - | - | 443.9k | 11.31k | - | - | - |
| instructions | - | - | - | 817.4k | 11.39k | - | - | - |
| cache references | - | - | - | 42.26 | 106.1 | - | - | - |
| cache misses | - | - | - | 16.50 | 16.65 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.528 | 0.318 | - | - | - |
| data TLB read misses | - | - | - | 0.209 | 5.820 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 36.86k | 0.123 | 0.00123 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 220.0 | 0.000722 | 0.00000722 |
| collectors' pauses, ns | - | - | - | - | - | 16.83M | 55.27 | 0.553 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 23.42M | - | - | - | - | 53.70M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 23.42M | - | - | - | - | 50.56M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 16.00M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.49 s from a quarter second before the window to a quarter second after it, 304544 messages, 30454611 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3181338-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3181338-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 167668 | 4522 | 187130 | 0 | 167668000 | 117517000 | 182066000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 408.2 | 7.7 | 351.4 | 8.89 / 15.95 |
| best | nats | 22 × wait | 167668 | 4522 | 187130 | 0 | 167668000 | 117517000 | 182066000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 408.2 | 7.7 | 351.4 | 8.89 / 15.95 |

Run 1's profile:

| profile: the window, 30.0 s, 117514 messages, 76825631 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 179.7G | 1.529M | 2339 | 23.63k | 2303 | 159.9G | 1.335M | 2051 |
| waiting for a CPU, ns | - | - | - | 5.663 | 221.5 | - | - | - |
| allocations | 78.09M | 664.5 | 1.016 | 1.994 | 1.013 | 71.91M | 600.3 | 0.922 |
| reallocations | 7.000 | 0.0000596 | 0.000000091 | 0.0000596 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 1.000 | 0.00000851 | 0.000000013 | 0.00000851 | 0 | - | - | - |
| frees | 78.09M | 664.5 | 1.016 | 1.976 | 1.013 | 71.81M | 599.4 | 0.921 |
| bytes allocated | 18.76G | 159.6k | 244.1 | 342.7 | 243.6 | 3.434G | 28.66k | 44.03 |
| minor page faults | 11.93k | 0.101 | 0.000155 | 0.00235 | 0.000152 | 11.58k | 0.0967 | 0.000149 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 12.33M | 104.9 | 0.160 | 0.000128 | 0.160 | - | - | - |
| involuntary context switches | 34.30k | 0.292 | 0.000447 | 0.000408 | 0.000446 | - | - | - |
| cycles | - | - | - | 115.3k | 6983 | - | - | - |
| instructions | - | - | - | 217.8k | 8854 | - | - | - |
| cache references | - | - | - | 24.52 | 46.86 | - | - | - |
| cache misses | - | - | - | 12.47 | 25.62 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.714 | 1.229 | - | - | - |
| data TLB read misses | - | - | - | 0.103 | 2.458 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 36.86k | 0.314 | 0.000480 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 212.0 | 0.00177 | 0.00000272 |
| collectors' pauses, ns | - | - | - | - | - | 18.09M | 151.0 | 0.232 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 138.8M | - | - | - | - | 144.6M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 138.8M | - | - | - | - | 142.8M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 36.49M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.52 s from a quarter second before the window to a quarter second after it, 119793 messages, 77982449 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3196573-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3196573-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 44.5 | 93.2 | 1130.5 | 4259.8 | 8966.3 | 0 | 0 | 1/1 | 41.5 | 26.9 | 30.8 | 42.22 / 38.10 |
| best | nats | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 44.5 | 93.2 | 1130.5 | 4259.8 | 8966.3 | 0 | 0 | 1/1 | 41.5 | 26.9 | 30.8 | 42.22 / 38.10 |

Run 1's profile:

| profile: the window, 30.0 s, 3000439 messages, 3000436 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 34.53G | 11.51k | 11.51k | 7490 | 4018 | 26.55G | 8705 | 8705 |
| waiting for a CPU, ns | - | - | - | 8.882 | 617.9 | - | - | - |
| allocations | 9.034M | 3.011 | 3.011 | 2.000 | 1.011 | 7.939M | 2.603 | 2.603 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 9.034M | 3.011 | 3.011 | 2.000 | 1.011 | 7.939M | 2.602 | 2.602 |
| bytes allocated | 1.440G | 479.8 | 479.8 | 280.0 | 199.8 | 1.262G | 413.8 | 413.8 |
| minor page faults | 98.00 | 0.0000327 | 0.0000327 | 0 | 0.0000317 | 2388 | 0.000783 | 0.000783 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 893.5k | 0.298 | 0.298 | 0 | 0.298 | - | - | - |
| involuntary context switches | 2150 | 0.000717 | 0.000717 | 0.000219 | 0.000498 | - | - | - |
| cycles | - | - | - | 30.51k | 9896 | - | - | - |
| instructions | - | - | - | 48.49k | 9992 | - | - | - |
| cache references | - | - | - | 17.55 | 50.63 | - | - | - |
| cache misses | - | - | - | 8.340 | 12.28 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.206 | 0.549 | - | - | - |
| data TLB read misses | - | - | - | 0.0580 | 0.975 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.00137 | 0.00137 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 199.0 | 0.0000652 | 0.0000652 |
| collectors' pauses, ns | - | - | - | - | - | 14.31M | 4.692 | 4.692 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.373M | - | - | - | - | 47.39M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.373M | - | - | - | - | 45.11M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 11.84M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3050559 messages, 3050558 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3278678-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3278678-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 858040 | 23988 | 2691998 | 0 | 85804000 | 35821100 | 264222600 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 255.1 | 0.9 | 188.1 | 15.56 / 17.27 |
| best | nats | 22 × wait | 858040 | 23988 | 2691998 | 0 | 85804000 | 35821100 | 264222600 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 255.1 | 0.9 | 188.1 | 15.56 / 17.27 |

Run 1's profile:

| profile: the window, 30.0 s, 703712 messages, 70455477 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 209.9G | 298.3k | 2980 | 591.5 | 2974 | 157.4G | 218.8k | 2196 |
| waiting for a CPU, ns | - | - | - | 1.525 | 32.01 | - | - | - |
| allocations | 72.74M | 103.4 | 1.032 | 2.016 | 1.012 | 75.84M | 105.4 | 1.058 |
| reallocations | 1.000 | 0.00000142 | 0.000000014 | 0.00000142 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 1.000 | 0.00000142 | 0.000000014 | 0.00000142 | 0 | - | - | - |
| frees | 72.74M | 103.4 | 1.032 | 2.016 | 1.012 | 75.84M | 105.4 | 1.058 |
| bytes allocated | 16.16G | 22.96k | 229.3 | 364.3 | 225.7 | 3.048G | 4237 | 42.54 |
| minor page faults | 44.00 | 0.0000625 | 0.000000625 | 0.0000426 | 0.000000128 | 13.19k | 0.0183 | 0.000184 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 14.76M | 20.97 | 0.209 | 0.000190 | 0.209 | - | - | - |
| involuntary context switches | 34.80k | 0.0494 | 0.000494 | 0.0000384 | 0.000493 | - | - | - |
| cycles | - | - | - | 3690 | 7591 | - | - | - |
| instructions | - | - | - | 7684 | 10.13k | - | - | - |
| cache references | - | - | - | 14.08 | 24.85 | - | - | - |
| cache misses | - | - | - | 6.318 | 0.934 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.230 | 0.0130 | - | - | - |
| data TLB read misses | - | - | - | 0.0537 | 1.096 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.00582 | 0.0000581 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 389.0 | 0.000541 | 0.00000543 |
| collectors' pauses, ns | - | - | - | - | - | refused: 389 collections in the span, past the 256 pauses Go keeps | refused | refused |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 25.08M | - | - | - | - | 58.18M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 25.08M | - | - | - | - | 55.61M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 17.24M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 719408 messages, 71656678 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3290698-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3290698-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 106019 | 2809 | 3444019 | 0 | 106019000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 266.7 | 0.4 | 230.5 | 11.38 / 15.40 |
| best | nats | 22 × wait | 106019 | 2809 | 3444019 | 0 | 106019000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 266.7 | 0.4 | 230.5 | 11.38 / 15.40 |

Run 1's profile:

| profile: the window, 30.0 s, 72656 messages, 74401803 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 187.4G | 2.579M | 2519 | 646.3 | 2518 | 164.0G | 2.258M | 2169 |
| waiting for a CPU, ns | - | - | - | 0.0928 | 24.84 | - | - | - |
| allocations | 75.57M | 1040 | 1.016 | 2.016 | 1.014 | 69.09M | 950.9 | 0.914 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 75.57M | 1040 | 1.016 | 2.016 | 1.014 | 69.22M | 952.7 | 0.915 |
| bytes allocated | 18.59G | 255.8k | 249.8 | 364.7 | 249.5 | 3.297G | 45.38k | 43.60 |
| minor page faults | 725.0 | 0.00998 | 0.00000974 | 0 | 0.00000969 | 9210 | 0.127 | 0.000122 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 11.31M | 155.6 | 0.152 | 0.000193 | 0.152 | - | - | - |
| involuntary context switches | 20.00k | 0.275 | 0.000269 | 0.0000138 | 0.000269 | - | - | - |
| cycles | - | - | - | 4147 | 7579 | - | - | - |
| instructions | - | - | - | 8583 | 9188 | - | - | - |
| cache references | - | - | - | 15.45 | 59.55 | - | - | - |
| cache misses | - | - | - | 8.782 | 31.49 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.440 | 1.454 | - | - | - |
| data TLB read misses | - | - | - | 0.0755 | 3.557 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.0564 | 0.0000551 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 193.0 | 0.00266 | 0.00000255 |
| collectors' pauses, ns | - | - | - | - | - | 13.77M | 189.5 | 0.182 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 122.5M | - | - | - | - | 125.0M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 122.5M | - | - | - | - | 125.7M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 33.10M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 72656 messages, 75624422 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3309788-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3309788-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 30.0 | 61.4 | 198.7 | 614.4 | 2159.3 | 0 | 0 | 1/1 | 69.4 | 34.3 | 8.0 | 36.14 / 29.55 |
| best | nats | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 30.0 | 61.4 | 198.7 | 614.4 | 2159.3 | 0 | 0 | 1/1 | 69.4 | 34.3 | 8.0 | 36.14 / 29.55 |

Run 1's profile:

| profile: the window, 30.0 s, 299585 messages, 299583 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 58.58G | 195.6k | 195.6k | 96.79k | 98.76k | 6.793G | 22.30k | 22.30k |
| waiting for a CPU, ns | - | - | - | 25.52 | 27.88 | - | - | - |
| allocations | 899.4k | 3.002 | 3.002 | 2.000 | 1.002 | 962.5k | 3.160 | 3.160 |
| reallocations | 1.000 | 0.00000334 | 0.00000334 | 0.00000334 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.00000334 | 0.00000334 | 0 | - | - | - |
| frees | 899.4k | 3.002 | 3.002 | 2.000 | 1.002 | 978.7k | 3.213 | 3.213 |
| bytes allocated | 101.2M | 337.8 | 337.8 | 280.0 | 57.84 | 86.87M | 285.2 | 285.2 |
| minor page faults | 5.000 | 0.0000167 | 0.0000167 | 0.00000334 | 0 | 617.0 | 0.00203 | 0.00203 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 606.0 | 0.00202 | 0.00202 | 0.00110 | 0.000921 | - | - | - |
| cycles | - | - | - | 483.8k | 539.2k | - | - | - |
| instructions | - | - | - | 907.2k | 914.9k | - | - | - |
| cache references | - | - | - | 69.80 | 100.2 | - | - | - |
| cache misses | - | - | - | 25.81 | 37.84 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.412 | 0.272 | - | - | - |
| data TLB read misses | - | - | - | 0.0601 | 0.0609 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.0137 | 0.0137 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 12.00 | 0.0000394 | 0.0000394 |
| collectors' pauses, ns | - | - | - | - | - | 664.2k | 2.181 | 2.181 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.730M | - | - | - | - | 47.19M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.730M | - | - | - | - | 48.07M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 16.42M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304583 messages, 304586 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3360691-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3360691-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 354785 | 9994 | 13 | 0 | 35478500 | 29958300 | 0 | 2719.7 | 6422.5 | 18612.2 | 122683.4 | 432092.5 | 0 | 0 | 100/100 | 554.6 | 20.2 | 39.0 | 45.44 / 46.60 |
| best | nats | 22 × busy | 354785 | 9994 | 13 | 0 | 35478500 | 29958300 | 0 | 2719.7 | 6422.5 | 18612.2 | 122683.4 | 432092.5 | 0 | 0 | 100/100 | 554.6 | 20.2 | 39.0 | 45.44 / 46.60 |

Run 1's profile:

| profile: the window, 30.0 s, 299594 messages, 29946152 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 475.3G | 1.587M | 15.87k | 56.51k | 15.31k | 33.10G | 108.7k | 1087 |
| waiting for a CPU, ns | - | - | - | 40.97k | 6394 | - | - | - |
| allocations | 31.05M | 103.6 | 1.037 | 2.000 | 1.017 | 14.28M | 46.88 | 0.469 |
| reallocations | 2.000 | 0.00000668 | 0.000000067 | 0.00000334 | 0.000000033 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 1.000 | 0.00000334 | 0.000000033 | 0 | 0.000000033 | - | - | - |
| frees | 31.05M | 103.6 | 1.037 | 2.000 | 1.017 | 14.34M | 47.09 | 0.471 |
| bytes allocated | 8.937G | 29.83k | 298.4 | 280.3 | 295.6 | 1.544G | 5069 | 50.69 |
| minor page faults | 7807 | 0.0261 | 0.000261 | 0.0000501 | 0.000260 | 14.51k | 0.0476 | 0.000476 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 100.6k | 0.336 | 0.00336 | 0.0162 | 0.00320 | - | - | - |
| cycles | - | - | - | 99.46k | 51.81k | - | - | - |
| instructions | - | - | - | 175.4k | 79.03k | - | - | - |
| cache references | - | - | - | 16.04 | 13.24 | - | - | - |
| cache misses | - | - | - | 8.892 | 5.163 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0000134 | 0.0942 | - | - | - |
| data TLB read misses | - | - | - | 0.234 | 0.191 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 20.48k | 0.0684 | 0.000684 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 150.0 | 0.000492 | 0.00000492 |
| collectors' pauses, ns | - | - | - | - | - | 96.24M | 316.0 | 3.160 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 65.95M | - | - | - | - | 64.42M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 65.79M | - | - | - | - | 61.94M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 21.91M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304586 messages, 30459095 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3405010-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3405010-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 98382 | 2625 | 256416 | 0 | 98382000 | 48231000 | 251352000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 882.5 | 2.3 | 210.1 | 33.02 / 34.03 |
| best | nats | 22 × busy | 98382 | 2625 | 256416 | 0 | 98382000 | 48231000 | 251352000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 882.5 | 2.3 | 210.1 | 33.02 / 34.03 |

Run 1's profile:

| profile: the window, 30.0 s, 69744 messages, 67660738 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 607.3G | 8.707M | 8975 | 668.4 | 8975 | 147.0G | 2.108M | 2137 |
| waiting for a CPU, ns | - | - | - | 657.6 | 378.9 | - | - | - |
| allocations | 68.73M | 985.4 | 1.016 | 2.016 | 1.014 | 61.99M | 888.8 | 0.901 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 68.72M | 985.4 | 1.016 | 2.016 | 1.014 | 61.83M | 886.5 | 0.899 |
| bytes allocated | 16.80G | 240.9k | 248.3 | 363.3 | 247.9 | 3.039G | 43.57k | 44.16 |
| minor page faults | 36.95k | 0.530 | 0.000546 | 0 | 0.000546 | 11.18k | 0.160 | 0.000163 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 14.00 | 0.000201 | 0.000000207 | 0.000186 | 0.000000015 | - | - | - |
| involuntary context switches | 43.22k | 0.620 | 0.000639 | 0.000186 | 0.000639 | - | - | - |
| cycles | - | - | - | 1731 | 34.34k | - | - | - |
| instructions | - | - | - | 3535 | 50.42k | - | - | - |
| cache references | - | - | - | 8.681 | 22.71 | - | - | - |
| cache misses | - | - | - | 4.995 | 15.07 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00300 | 0.538 | - | - | - |
| data TLB read misses | - | - | - | 0.0174 | 0.608 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 36.86k | 0.529 | 0.000545 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 153.0 | 0.00219 | 0.00000222 |
| collectors' pauses, ns | - | - | - | - | - | 30.99M | 444.3 | 0.450 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 140.9M | - | - | - | - | 136.9M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 131.9M | - | - | - | - | 130.3M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 36.86M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 69744 messages, 68800159 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3413209-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3413209-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × busy | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 15.4 | 21.0 | 46.1 | 111.6 | 875.7 | 0 | 0 | 1/1 | 64.0 | 30.2 | 23.3 | 20.01 / 13.18 |
| best | nats | 1 × busy | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 15.4 | 21.0 | 46.1 | 111.6 | 875.7 | 0 | 0 | 1/1 | 64.0 | 30.2 | 23.3 | 20.01 / 13.18 |

Run 1's profile:

| profile: the window, 30.0 s, 3000436 messages, 3000435 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 54.02G | 18.00k | 18.00k | 8512 | 9490 | 20.06G | 6575 | 6575 |
| waiting for a CPU, ns | - | - | - | 1.108 | 0.804 | - | - | - |
| allocations | 9.018M | 3.005 | 3.005 | 2.000 | 1.005 | 8.861M | 2.904 | 2.904 |
| reallocations | 1.000 | 0.000000333 | 0.000000333 | 0.000000333 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 9.018M | 3.005 | 3.005 | 2.000 | 1.005 | 8.887M | 2.912 | 2.912 |
| bytes allocated | 1.178G | 392.8 | 392.8 | 280.0 | 112.8 | 1.030G | 337.6 | 337.6 |
| minor page faults | 6.000 | 0.00000200 | 0.00000200 | 0.000001000 | 0 | 1690 | 0.000554 | 0.000554 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.000 | 0.000000333 | 0.000000333 | 0.000000333 | 0 | - | - | - |
| involuntary context switches | 731.0 | 0.000244 | 0.000244 | 0.000162 | 0.0000813 | - | - | - |
| cycles | - | - | - | 42.17k | 56.89k | - | - | - |
| instructions | - | - | - | 73.82k | 95.13k | - | - | - |
| cache references | - | - | - | 4.794 | 4.033 | - | - | - |
| cache misses | - | - | - | 1.713 | 1.552 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00996 | 0.00920 | - | - | - |
| data TLB read misses | - | - | - | 0.0817 | 0.0322 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.00137 | 0.00137 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 130.0 | 0.0000426 | 0.0000426 |
| collectors' pauses, ns | - | - | - | - | - | 5.345M | 1.752 | 1.752 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.816M | - | - | - | - | 50.54M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.816M | - | - | - | - | 47.96M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 17.40M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 3051542 messages, 3051545 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3442966-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3442966-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 946904 | 26272 | 2603134 | 0 | 94670739 | 44707500 | 255336200 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 88/100 | 577.3 | 0.9 | 154.0 | 13.73 / 30.24 |
| best | nats | 22 × busy | 946904 | 26272 | 2603134 | 0 | 94670739 | 44707500 | 255336200 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 88/100 | 577.3 | 0.9 | 154.0 | 13.73 / 30.24 |

Run 1's profile:

| profile: the window, 30.0 s, 790576 messages, 79135021 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 509.1G | 644.0k | 6433 | 636.7 | 6427 | 131.9G | 162.8k | 1636 |
| waiting for a CPU, ns | - | - | - | 1236 | 1323 | - | - | - |
| allocations | 82.09M | 103.8 | 1.037 | 2.017 | 1.017 | 66.02M | 81.45 | 0.819 |
| reallocations | 1.000 | 0.00000126 | 0.000000013 | 0 | 0.000000013 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 1.000 | 0.00000126 | 0.000000013 | 0 | 0.000000013 | - | - | - |
| frees | 82.05M | 103.8 | 1.037 | 2.017 | 1.017 | 72.16M | 89.02 | 0.895 |
| bytes allocated | 21.93G | 27.74k | 277.2 | 365.9 | 273.5 | 3.542G | 4370 | 43.93 |
| minor page faults | 53.63k | 0.0678 | 0.000678 | 0 | 0.000678 | 48.19k | 0.0595 | 0.000598 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 170.0 | 0.000215 | 0.00000215 | 0.000197 | 0.000000177 | - | - | - |
| involuntary context switches | 100.8k | 0.128 | 0.00127 | 0.000250 | 0.00127 | - | - | - |
| cycles | - | - | - | 3866 | 28.97k | - | - | - |
| instructions | - | - | - | 7182 | 42.70k | - | - | - |
| cache references | - | - | - | 14.94 | 11.37 | - | - | - |
| cache misses | - | - | - | 7.187 | 3.362 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.178 | 0.0578 | - | - | - |
| data TLB read misses | - | - | - | 0.0545 | 0.163 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 53.25k | 0.0674 | 0.000673 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 103.0 | 0.000127 | 0.00000128 |
| collectors' pauses, ns | - | - | - | - | - | 41.86M | 51.64 | 0.519 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 2.495G | - | - | - | - | 908.7M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 939.5M | - | - | - | - | 136.6M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 67.75M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 810544 messages, 80623642 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3459040-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3459040-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 98480 | 2712 | 3451558 | 0 | 98480000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 878.9 | 0.4 | 220.9 | 28.14 / 29.68 |
| best | nats | 22 × busy | 98480 | 2712 | 3451558 | 0 | 98480000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 878.9 | 0.4 | 220.9 | 28.14 / 29.68 |

Run 1's profile:

| profile: the window, 30.0 s, 70864 messages, 70889203 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 630.3G | 8.894M | 8891 | 631.7 | 8890 | 161.6G | 2.281M | 2242 |
| waiting for a CPU, ns | - | - | - | 295.5 | 45.34 | - | - | - |
| allocations | 71.95M | 1015 | 1.015 | 2.016 | 1.013 | 70.51M | 995.0 | 0.978 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 71.95M | 1015 | 1.015 | 2.016 | 1.013 | 70.54M | 995.4 | 0.979 |
| bytes allocated | 16.81G | 237.2k | 237.1 | 362.7 | 236.8 | 3.119G | 44.01k | 43.27 |
| minor page faults | 6108 | 0.0862 | 0.0000862 | 0 | 0.0000861 | 13.81k | 0.195 | 0.000192 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 13.00 | 0.000183 | 0.000000183 | 0.000183 | 0 | - | - | - |
| involuntary context switches | 13.13k | 0.185 | 0.000185 | 0.000141 | 0.000185 | - | - | - |
| cycles | - | - | - | 2062 | 28.70k | - | - | - |
| instructions | - | - | - | 3532 | 44.15k | - | - | - |
| cache references | - | - | - | 6.204 | 15.18 | - | - | - |
| cache misses | - | - | - | 3.171 | 10.92 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.193 | 0.140 | - | - | - |
| data TLB read misses | - | - | - | 0.0349 | 0.206 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.0578 | 0.0000578 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 169.0 | 0.00238 | 0.00000234 |
| collectors' pauses, ns | - | - | - | - | - | 13.23M | 186.7 | 0.184 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 131.1M | - | - | - | - | 150.9M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 131.1M | - | - | - | - | 143.9M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 35.03M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 70864 messages, 72082471 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3501796-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3501796-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 42.0 | 79.9 | 1065.0 | 2883.6 | 4928.4 | 0 | 0 | 1/1 | 37.9 | 34.1 | 7.9 | 31.74 / 23.50 |
| best | nats | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 42.0 | 79.9 | 1065.0 | 2883.6 | 4928.4 | 0 | 0 | 1/1 | 37.9 | 34.1 | 7.9 | 31.74 / 23.50 |

Run 1's profile:

| profile: the window, 30.0 s, 300767 messages, 300768 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 32.00G | 106.4k | 106.4k | 95.91k | 10.48k | 6.785G | 22.18k | 22.18k |
| waiting for a CPU, ns | - | - | - | 27.03 | 1499 | - | - | - |
| allocations | 903.2k | 3.003 | 3.003 | 2.000 | 1.003 | 977.8k | 3.196 | 3.196 |
| reallocations | 3.000 | 0.00000997 | 0.00000997 | 0.00000997 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 1.000 | 0.00000332 | 0.00000332 | 0.00000332 | 0 | - | - | - |
| frees | 903.2k | 3.003 | 3.003 | 2.000 | 1.003 | 962.6k | 3.147 | 3.147 |
| bytes allocated | 105.7M | 351.3 | 351.3 | 280.0 | 71.25 | 82.86M | 270.9 | 270.9 |
| minor page faults | 10.00 | 0.0000332 | 0.0000332 | 0.0000166 | 0 | 728.0 | 0.00238 | 0.00238 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 239.4k | 0.796 | 0.796 | 0 | 0.796 | - | - | - |
| involuntary context switches | 753.0 | 0.00250 | 0.00250 | 0.000775 | 0.00173 | - | - | - |
| cycles | - | - | - | 477.3k | 22.01k | - | - | - |
| instructions | - | - | - | 889.9k | 15.60k | - | - | - |
| cache references | - | - | - | 77.52 | 316.1 | - | - | - |
| cache misses | - | - | - | 37.44 | 89.73 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 1.173 | 2.467 | - | - | - |
| data TLB read misses | - | - | - | 0.0834 | 7.980 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.0136 | 0.0136 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 9.000 | 0.0000294 | 0.0000294 |
| collectors' pauses, ns | - | - | - | - | - | 691.6k | 2.261 | 2.261 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.758M | - | - | - | - | 74.59M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.758M | - | - | - | - | 75.67M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 20.00M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305908 messages, 305906 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3520202-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3520202-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 202.8 | 356.4 | 868.4 | 2162.7 | 11983.0 | 0 | 0 | 100/100 | 211.2 | 33.9 | 115.5 | 35.45 / 32.83 |
| best | nats | 22 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 202.8 | 356.4 | 868.4 | 2162.7 | 11983.0 | 0 | 0 | 100/100 | 211.2 | 33.9 | 115.5 | 35.45 / 32.83 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 30076866 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 183.9G | 611.3k | 6113 | 95.19k | 5161 | 102.0G | 333.5k | 3335 |
| waiting for a CPU, ns | - | - | - | 21.52 | 1459 | - | - | - |
| allocations | 30.96M | 102.9 | 1.029 | 2.000 | 1.009 | 40.81M | 133.4 | 1.334 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 30.96M | 102.9 | 1.029 | 2.000 | 1.009 | 40.73M | 133.2 | 1.332 |
| bytes allocated | 5.448G | 18.11k | 181.1 | 280.0 | 178.3 | 1.456G | 4760 | 47.60 |
| minor page faults | 36.00 | 0.000120 | 0.00000120 | 0 | 0.00000110 | 4779 | 0.0156 | 0.000156 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 9.429M | 31.35 | 0.314 | 0 | 0.314 | - | - | - |
| involuntary context switches | 231.4k | 0.770 | 0.00770 | 0.000961 | 0.00769 | - | - | - |
| cycles | - | - | - | 419.0k | 11.94k | - | - | - |
| instructions | - | - | - | 759.7k | 10.87k | - | - | - |
| cache references | - | - | - | 94.87 | 168.2 | - | - | - |
| cache misses | - | - | - | 23.83 | 17.42 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.392 | 0.231 | - | - | - |
| data TLB read misses | - | - | - | 0.544 | 9.326 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 4096 | 0.0136 | 0.000136 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 139.0 | 0.000454 | 0.00000454 |
| collectors' pauses, ns | - | - | - | - | - | 11.89M | 38.87 | 0.389 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 22.67M | - | - | - | - | 82.08M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 22.67M | - | - | - | - | 79.10M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 22.79M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305867 messages, 30586889 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3583529-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3583529-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 100877 | 2806 | 255224 | 0 | 100877000 | 50473000 | 250296000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 260.8 | 2.2 | 217.7 | 10.31 / 15.67 |
| best | nats | 22 × wait | 100877 | 2806 | 255224 | 0 | 100877000 | 50473000 | 250296000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 260.8 | 2.2 | 217.7 | 10.31 / 15.67 |

Run 1's profile:

| profile: the window, 30.0 s, 74224 messages, 73923331 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 190.7G | 2.570M | 2580 | 652.7 | 2580 | 163.7G | 2.205M | 2178 |
| waiting for a CPU, ns | - | - | - | 0 | 51.61 | - | - | - |
| allocations | 75.09M | 1012 | 1.016 | 2.017 | 1.014 | 66.61M | 897.5 | 0.887 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 75.09M | 1012 | 1.016 | 2.017 | 1.014 | 66.74M | 899.2 | 0.888 |
| bytes allocated | 18.42G | 248.2k | 249.2 | 366.4 | 248.9 | 3.211G | 43.26k | 42.73 |
| minor page faults | 347.0 | 0.00468 | 0.00000469 | 0 | 0.00000465 | 3931 | 0.0530 | 0.0000523 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 10.54M | 142.1 | 0.143 | 0.000202 | 0.143 | - | - | - |
| involuntary context switches | 31.76k | 0.428 | 0.000430 | 0 | 0.000430 | - | - | - |
| cycles | - | - | - | 1787 | 7976 | - | - | - |
| instructions | - | - | - | 3536 | 9223 | - | - | - |
| cache references | - | - | - | 8.173 | 74.85 | - | - | - |
| cache misses | - | - | - | 4.573 | 36.95 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00278 | 1.746 | - | - | - |
| data TLB read misses | - | - | - | 0.0188 | 4.756 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 36.86k | 0.497 | 0.000499 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 159.0 | 0.00214 | 0.00000212 |
| collectors' pauses, ns | - | - | - | - | - | 11.64M | 156.9 | 0.155 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 122.0M | - | - | - | - | 150.5M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 122.0M | - | - | - | - | 148.2M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 28.69M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.48 s from a quarter second before the window to a quarter second after it, 74224 messages, 75138819 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3593940-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3593940-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × wait | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 21.5 | 31.0 | 63.5 | 190.5 | 2208.3 | 0 | 0 | 1/1 | 42.4 | 30.0 | 24.0 | 12.40 / 9.36 |
| best | nats | 1 × wait | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 21.5 | 31.0 | 63.5 | 190.5 | 2208.3 | 0 | 0 | 1/1 | 42.4 | 30.0 | 24.0 | 12.40 / 9.36 |

Run 1's profile:

| profile: the window, 30.0 s, 3002626 messages, 3002622 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 35.83G | 11.93k | 11.93k | 8440 | 3493 | 20.59G | 6745 | 6745 |
| waiting for a CPU, ns | - | - | - | 2.071 | 2.991 | - | - | - |
| allocations | 9.028M | 3.007 | 3.007 | 2.000 | 1.007 | 8.859M | 2.902 | 2.902 |
| reallocations | 2.000 | 0.000000666 | 0.000000666 | 0.000000666 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 1.000 | 0.000000333 | 0.000000333 | 0.000000333 | 0 | - | - | - |
| frees | 9.028M | 3.007 | 3.007 | 2.000 | 1.007 | 8.860M | 2.903 | 2.903 |
| bytes allocated | 1.244G | 414.2 | 414.2 | 280.0 | 134.2 | 1.055G | 345.6 | 345.6 |
| minor page faults | 32.00 | 0.0000107 | 0.0000107 | 0.00000833 | 0.000000999 | 785.0 | 0.000257 | 0.000257 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.387M | 0.462 | 0.462 | 0 | 0.462 | - | - | - |
| involuntary context switches | 690.0 | 0.000230 | 0.000230 | 0.000163 | 0.0000673 | - | - | - |
| cycles | - | - | - | 41.06k | 8648 | - | - | - |
| instructions | - | - | - | 71.85k | 11.62k | - | - | - |
| cache references | - | - | - | 5.588 | 4.984 | - | - | - |
| cache misses | - | - | - | 1.244 | 0.904 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0198 | 0.0176 | - | - | - |
| data TLB read misses | - | - | - | 0.0614 | 0.0691 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 36.86k | 0.0123 | 0.0123 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 106.0 | 0.0000347 | 0.0000347 |
| collectors' pauses, ns | - | - | - | - | - | 4.015M | 1.315 | 1.315 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.803M | - | - | - | - | 75.41M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.803M | - | - | - | - | 73.61M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 15.62M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3052705 messages, 3052705 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3611398-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3611398-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 910831 | 25480 | 2643187 | 0 | 91083100 | 40953500 | 259309600 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 142.3 | 0.9 | 112.8 | 8.74 / 33.35 |
| best | nats | 22 × wait | 910831 | 25480 | 2643187 | 0 | 91083100 | 40953500 | 259309600 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 142.3 | 0.9 | 112.8 | 8.74 / 33.35 |

Run 1's profile:

| profile: the window, 30.0 s, 745872 messages, 74490614 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 101.4G | 136.0k | 1362 | 689.5 | 1355 | 84.94G | 112.3k | 1119 |
| waiting for a CPU, ns | - | - | - | 513.3 | 3521 | - | - | - |
| allocations | 77.28M | 103.6 | 1.038 | 2.016 | 1.017 | 39.99M | 52.88 | 0.527 |
| reallocations | 12.00 | 0.0000161 | 0.000000161 | 0 | 0.000000161 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 12.00 | 0.0000161 | 0.000000161 | 0 | 0.000000161 | - | - | - |
| frees | 77.28M | 103.6 | 1.038 | 2.016 | 1.017 | 40.14M | 53.08 | 0.529 |
| bytes allocated | 22.34G | 29.95k | 299.9 | 365.0 | 296.3 | 3.521G | 4656 | 46.38 |
| minor page faults | 17.52k | 0.0235 | 0.000235 | 0 | 0.000235 | 37.19k | 0.0492 | 0.000490 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.482M | 4.668 | 0.0467 | 0.000194 | 0.0467 | - | - | - |
| involuntary context switches | 301.2k | 0.404 | 0.00404 | 0.00173 | 0.00403 | - | - | - |
| cycles | - | - | - | 4252 | 4799 | - | - | - |
| instructions | - | - | - | 7524 | 6749 | - | - | - |
| cache references | - | - | - | 17.09 | 36.70 | - | - | - |
| cache misses | - | - | - | 10.07 | 12.53 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.236 | 0.362 | - | - | - |
| data TLB read misses | - | - | - | 0.0591 | 1.297 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 53.25k | 0.0714 | 0.000715 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 251.0 | 0.000332 | 0.00000331 |
| collectors' pauses, ns | - | - | - | - | - | 183.9M | 243.1 | 2.422 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 78.18M | - | - | - | - | 110.6M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 78.18M | - | - | - | - | 93.68M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 28.48M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 756192 messages, 75912286 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3635112-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3635112-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × wait | 100397 | 2800 | 3453621 | 0 | 100397000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 255.9 | 0.4 | 216.6 | 15.66 / 19.16 |
| best | nats | 22 × wait | 100397 | 2800 | 3453621 | 0 | 100397000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 255.9 | 0.4 | 216.6 | 15.66 / 19.16 |

Run 1's profile:

| profile: the window, 30.0 s, 72656 messages, 73228102 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 187.8G | 2.585M | 2565 | 613.9 | 2564 | 162.5G | 2.237M | 2182 |
| waiting for a CPU, ns | - | - | - | 5.639 | 89.92 | - | - | - |
| allocations | 74.40M | 1024 | 1.016 | 2.016 | 1.014 | 64.15M | 882.9 | 0.861 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 74.40M | 1024 | 1.016 | 2.016 | 1.014 | 64.18M | 883.4 | 0.862 |
| bytes allocated | 18.60G | 256.0k | 254.0 | 364.4 | 253.6 | 3.177G | 43.73k | 42.65 |
| minor page faults | 637.0 | 0.00877 | 0.00000870 | 0 | 0.00000866 | 3654 | 0.0503 | 0.0000490 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 10.22M | 140.7 | 0.140 | 0.000193 | 0.140 | - | - | - |
| involuntary context switches | 61.42k | 0.845 | 0.000839 | 0.000110 | 0.000839 | - | - | - |
| cycles | - | - | - | 3681 | 7908 | - | - | - |
| instructions | - | - | - | 7338 | 8683 | - | - | - |
| cache references | - | - | - | 15.96 | 80.82 | - | - | - |
| cache misses | - | - | - | 8.648 | 38.11 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.256 | 2.096 | - | - | - |
| data TLB read misses | - | - | - | 0.0485 | 5.068 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 36.86k | 0.507 | 0.000503 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 148.0 | 0.00204 | 0.00000199 |
| collectors' pauses, ns | - | - | - | - | - | 11.38M | 156.7 | 0.153 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 122.2M | - | - | - | - | 157.8M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 122.2M | - | - | - | - | 156.3M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 35.14M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 72656 messages, 74497009 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3673056-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3673056-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 29.2 | 44.0 | 65.5 | 91.1 | 1905.2 | 0 | 0 | 1/1 | 69.7 | 34.5 | 8.2 | 23.89 / 17.93 |
| best | nats | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 29.2 | 44.0 | 65.5 | 91.1 | 1905.2 | 0 | 0 | 1/1 | 69.7 | 34.5 | 8.2 | 23.89 / 17.93 |

Run 1's profile:

| profile: the window, 30.0 s, 300766 messages, 300766 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 58.81G | 195.5k | 195.5k | 96.91k | 98.62k | 6.939G | 22.69k | 22.69k |
| waiting for a CPU, ns | - | - | - | 31.80 | 25.18 | - | - | - |
| allocations | 902.8k | 3.002 | 3.002 | 2.000 | 1.002 | 1.003M | 3.280 | 3.280 |
| reallocations | 1.000 | 0.00000332 | 0.00000332 | 0.00000332 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 1.000 | 0.00000332 | 0.00000332 | 0.00000332 | 0 | - | - | - |
| frees | 902.8k | 3.002 | 3.002 | 2.000 | 1.002 | 969.3k | 3.169 | 3.169 |
| bytes allocated | 99.01M | 329.2 | 329.2 | 280.0 | 49.20 | 85.16M | 278.4 | 278.4 |
| minor page faults | 5.000 | 0.0000166 | 0.0000166 | 0.00000332 | 0 | 584.0 | 0.00191 | 0.00191 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 871.0 | 0.00290 | 0.00290 | 0.00158 | 0.00132 | - | - | - |
| cycles | - | - | - | 493.5k | 553.2k | - | - | - |
| instructions | - | - | - | 899.3k | 949.9k | - | - | - |
| cache references | - | - | - | 25.46 | 23.42 | - | - | - |
| cache misses | - | - | - | 12.26 | 9.514 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.654 | 0.432 | - | - | - |
| data TLB read misses | - | - | - | 0.0316 | 0.0320 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 36.86k | 0.123 | 0.123 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 8.000 | 0.0000262 | 0.0000262 |
| collectors' pauses, ns | - | - | - | - | - | 323.3k | 1.057 | 1.057 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.787M | - | - | - | - | 73.58M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.787M | - | - | - | - | 74.29M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 21.74M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305843 messages, 305842 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3695457-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3695457-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 170.0 | 364.5 | 909.3 | 2162.7 | 30446.1 | 0 | 0 | 100/100 | 780.9 | 34.0 | 112.1 | 5.27 / 17.29 |
| best | nats | 22 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 170.0 | 364.5 | 909.3 | 2162.7 | 30446.1 | 0 | 0 | 100/100 | 780.9 | 34.0 | 112.1 | 5.27 / 17.29 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 30075911 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 661.0G | 2.198M | 21.98k | 95.77k | 21.02k | 96.77G | 316.4k | 3164 |
| waiting for a CPU, ns | - | - | - | 91.35 | 14.52 | - | - | - |
| allocations | 30.93M | 102.8 | 1.028 | 2.000 | 1.008 | 42.71M | 139.6 | 1.396 |
| reallocations | 2.000 | 0.00000665 | 0.000000066 | 0 | 0.000000066 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 2.000 | 0.00000665 | 0.000000066 | 0 | 0.000000066 | - | - | - |
| frees | 30.93M | 102.8 | 1.028 | 2.000 | 1.008 | 42.73M | 139.7 | 1.397 |
| bytes allocated | 4.930G | 16.39k | 163.9 | 280.0 | 161.1 | 1.493G | 4881 | 48.81 |
| minor page faults | 166.0 | 0.000552 | 0.00000552 | 0.00000665 | 0.00000529 | 2394 | 0.00783 | 0.0000783 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 9181 | 0.0305 | 0.000305 | 0.00112 | 0.000294 | - | - | - |
| cycles | - | - | - | 186.4k | 51.62k | - | - | - |
| instructions | - | - | - | 342.3k | 78.66k | - | - | - |
| cache references | - | - | - | 0.988 | 1.189 | - | - | - |
| cache misses | - | - | - | 0.591 | 0.259 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.00347 | - | - | - |
| data TLB read misses | - | - | - | 0.00938 | 0.120 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 36.86k | 0.123 | 0.00123 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 132.0 | 0.000432 | 0.00000432 |
| collectors' pauses, ns | - | - | - | - | - | 13.96M | 45.64 | 0.456 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 23.58M | - | - | - | - | 84.62M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 23.58M | - | - | - | - | 82.96M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 19.31M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305906 messages, 30590701 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3702761-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3702761-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 98978 | 2776 | 257123 | 0 | 98978000 | 48574000 | 252195000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 864.4 | 2.5 | 218.5 | 26.61 / 37.04 |
| best | nats | 22 × busy | 98978 | 2776 | 257123 | 0 | 98978000 | 48574000 | 252195000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 864.4 | 2.5 | 218.5 | 26.61 / 37.04 |

Run 1's profile:

| profile: the window, 30.0 s, 68176 messages, 69939589 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 617.4G | 9.056M | 8827 | 657.0 | 8827 | 155.1G | 2.275M | 2182 |
| waiting for a CPU, ns | - | - | - | 689.2 | 201.4 | - | - | - |
| allocations | 71.02M | 1042 | 1.015 | 2.016 | 1.014 | 64.81M | 950.7 | 0.912 |
| reallocations | 39.00 | 0.000572 | 0.000000558 | 0 | 0.000000558 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 39.00 | 0.000572 | 0.000000558 | 0 | 0.000000558 | - | - | - |
| frees | 71.02M | 1042 | 1.015 | 2.016 | 1.013 | 64.95M | 952.8 | 0.914 |
| bytes allocated | 17.16G | 251.8k | 245.4 | 364.3 | 245.0 | 3.128G | 45.88k | 44.02 |
| minor page faults | 25.76k | 0.378 | 0.000368 | 0 | 0.000368 | 17.39k | 0.255 | 0.000245 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 121.0 | 0.00177 | 0.00000173 | 0.000191 | 0.00000154 | - | - | - |
| involuntary context switches | 24.33k | 0.357 | 0.000348 | 0.000191 | 0.000348 | - | - | - |
| cycles | - | - | - | 1737 | 34.38k | - | - | - |
| instructions | - | - | - | 3536 | 50.97k | - | - | - |
| cache references | - | - | - | 8.124 | 23.53 | - | - | - |
| cache misses | - | - | - | 4.874 | 15.85 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00268 | 0.567 | - | - | - |
| data TLB read misses | - | - | - | 0.0263 | 0.604 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 36.86k | 0.541 | 0.000527 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 143.0 | 0.00210 | 0.00000201 |
| collectors' pauses, ns | - | - | - | - | - | 19.80M | 290.4 | 0.279 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 161.3M | - | - | - | - | 160.3M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 161.3M | - | - | - | - | 153.1M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 48.89M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 68176 messages, 71060453 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3713003-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3713003-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × busy | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 15.6 | 34.3 | 655.4 | 2457.6 | 7856.2 | 0 | 0 | 1/1 | 63.5 | 29.8 | 23.2 | 29.77 / 18.12 |
| best | nats | 1 × busy | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 15.6 | 34.3 | 655.4 | 2457.6 | 7856.2 | 0 | 0 | 1/1 | 63.5 | 29.8 | 23.2 | 29.77 / 18.12 |

Run 1's profile:

| profile: the window, 30.0 s, 3002627 messages, 3002630 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 53.58G | 17.84k | 17.84k | 8390 | 9455 | 19.69G | 6450 | 6450 |
| waiting for a CPU, ns | - | - | - | 8.941 | 5.331 | - | - | - |
| allocations | 9.027M | 3.006 | 3.006 | 2.000 | 1.006 | 8.658M | 2.836 | 2.836 |
| reallocations | 4.000 | 0.00000133 | 0.00000133 | 0.00000133 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 2.000 | 0.000000666 | 0.000000666 | 0.000000666 | 0 | - | - | - |
| frees | 9.027M | 3.006 | 3.006 | 2.000 | 1.006 | 8.635M | 2.829 | 2.829 |
| bytes allocated | 1.220G | 406.4 | 406.4 | 280.1 | 126.4 | 1.073G | 351.4 | 351.4 |
| minor page faults | 131.0 | 0.0000436 | 0.0000436 | 0.0000147 | 0.0000273 | 1168 | 0.000383 | 0.000383 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 884.0 | 0.000294 | 0.000294 | 0.000203 | 0.0000913 | - | - | - |
| cycles | - | - | - | 38.64k | 55.29k | - | - | - |
| instructions | - | - | - | 66.38k | 93.40k | - | - | - |
| cache references | - | - | - | 10.75 | 11.24 | - | - | - |
| cache misses | - | - | - | 6.184 | 5.328 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0514 | 0.00385 | - | - | - |
| data TLB read misses | - | - | - | 0.0462 | 0.00470 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 53.25k | 0.0177 | 0.0177 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 106.0 | 0.0000347 | 0.0000347 |
| collectors' pauses, ns | - | - | - | - | - | 6.446M | 2.112 | 2.112 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.283M | - | - | - | - | 75.75M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.283M | - | - | - | - | 73.43M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 21.50M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3052474 messages, 3052472 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3752796-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3752796-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 984750 | 27521 | 2569268 | 0 | 98475000 | 48345400 | 251917700 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 751.2 | 1.1 | 164.5 | 10.88 / 20.75 |
| best | nats | 22 × busy | 984750 | 27521 | 2569268 | 0 | 98475000 | 48345400 | 251917700 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 100/100 | 751.2 | 1.1 | 164.5 | 10.88 / 20.75 |

Run 1's profile:

| profile: the window, 30.0 s, 807984 messages, 80523940 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 617.4G | 764.1k | 7667 | 615.8 | 7661 | 138.1G | 168.7k | 1686 |
| waiting for a CPU, ns | - | - | - | 501.2 | 72.76 | - | - | - |
| allocations | 83.28M | 103.1 | 1.034 | 2.016 | 1.014 | 71.24M | 86.99 | 0.869 |
| reallocations | 1.000 | 0.00000124 | 0.000000012 | 0 | 0.000000012 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 1.000 | 0.00000124 | 0.000000012 | 0 | 0.000000012 | - | - | - |
| frees | 83.28M | 103.1 | 1.034 | 2.016 | 1.014 | 71.39M | 87.17 | 0.871 |
| bytes allocated | 20.54G | 25.42k | 255.0 | 363.2 | 251.4 | 3.600G | 4396 | 43.93 |
| minor page faults | 2660 | 0.00329 | 0.0000330 | 0 | 0.0000330 | 23.39k | 0.0286 | 0.000285 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 152.0 | 0.000188 | 0.00000189 | 0.000187 | 0 | - | - | - |
| involuntary context switches | 32.80k | 0.0406 | 0.000407 | 0.000145 | 0.000406 | - | - | - |
| cycles | - | - | - | 1708 | 20.49k | - | - | - |
| instructions | - | - | - | 3537 | 31.19k | - | - | - |
| cache references | - | - | - | 6.315 | 2.967 | - | - | - |
| cache misses | - | - | - | 3.133 | 0.376 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00213 | 0.00382 | - | - | - |
| data TLB read misses | - | - | - | 0.00984 | 0.0585 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 36.86k | 0.0456 | 0.000458 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 288.0 | 0.000352 | 0.00000351 |
| collectors' pauses, ns | - | - | - | - | - | refused: 288 collections in the span, past the 256 pauses Go keeps | refused | refused |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 48.87M | - | - | - | - | 93.19M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 48.87M | - | - | - | - | 92.71M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 25.17M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.52 s from a quarter second before the window to a quarter second after it, 818992 messages, 81939212 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3789319-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3789319-0-nats-allocs.pb.gz
# quiver-bench fanout nats

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 22 × busy | 98417 | 2730 | 3455601 | 0 | 98417000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 903.0 | 0.4 | 201.9 | 34.97 / 34.78 |
| best | nats | 22 × busy | 98417 | 2730 | 3455601 | 0 | 98417000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 903.0 | 0.4 | 201.9 | 34.97 / 34.78 |

Run 1's profile:

| profile: the window, 30.0 s, 65920 messages, 65606016 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 605.4G | 9.184M | 9228 | 662.0 | 9227 | 133.2G | 2.020M | 1999 |
| waiting for a CPU, ns | - | - | - | 764.9 | 452.6 | - | - | - |
| allocations | 66.67M | 1011 | 1.016 | 2.016 | 1.014 | 53.74M | 815.2 | 0.807 |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: Go grows a block by allocating another | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 66.67M | 1011 | 1.016 | 2.016 | 1.014 | 53.49M | 811.5 | 0.803 |
| bytes allocated | 16.89G | 256.2k | 257.5 | 362.0 | 257.1 | 2.969G | 45.03k | 44.55 |
| minor page faults | 37.74k | 0.573 | 0.000575 | 0 | 0.000575 | 10.30k | 0.156 | 0.000155 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 37.00 | 0.000561 | 0.000000564 | 0.000182 | 0.000000366 | - | - | - |
| involuntary context switches | 25.44k | 0.386 | 0.000388 | 0.000182 | 0.000388 | - | - | - |
| cycles | - | - | - | 1746 | 38.65k | - | - | - |
| instructions | - | - | - | 3533 | 56.10k | - | - | - |
| cache references | - | - | - | 8.300 | 24.92 | - | - | - |
| cache misses | - | - | - | 4.995 | 16.64 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00306 | 0.656 | - | - | - |
| data TLB read misses | - | - | - | 0.0257 | 0.652 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 36.86k | 0.559 | 0.000562 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | 121.0 | 0.00184 | 0.00000182 |
| collectors' pauses, ns | - | - | - | - | - | 42.17M | 639.7 | 0.633 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 139.2M | - | - | - | - | 167.4M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 130.7M | - | - | - | - | 158.2M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 47.28M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 65920 messages, 66630003 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3826272-0-nats-heap.pb.gz
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/nats-3826272-0-nats-allocs.pb.gz
