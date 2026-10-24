- broker: public.ecr.aws/docker/library/redis:8.2-alpine@sha256:b51665e66f00759be7c3152ad5ac3c66fb2f619c13ef62dea7cc1f9914524635, CPUs 8-15
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 17.9 | 47.6 | 103.4 | 219.1 | 1005.4 | 0 | 0 | 1/1 | 37.6 | 34.8 | 2.1 | 1.69 / 1.56 |
| best | redis | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 17.9 | 47.6 | 103.4 | 219.1 | 1005.4 | 0 | 0 | 1/1 | 37.6 | 34.8 | 2.1 | 1.69 / 1.56 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 299499 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 31.69G | 105.8k | 105.8k | 98.20k | 7605 | 1.834G | 6021 | 6021 |
| waiting for a CPU, ns | - | - | - | 5.870 | 8.408 | - | - | - |
| allocations | 3.898M | 13.01 | 13.01 | 7.982 | 5.032 | 952.6k | 3.127 | 3.127 |
| reallocations | 610.5k | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 610.5k | 2.038 | 2.038 | 2.000 | 0.0385 | - | - | - |
| frees | 3.898M | 13.01 | 13.01 | 7.982 | 5.032 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 853.4M | 2849 | 2849 | 2301 | 547.9 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 2.000 | 0.00000668 | 0.00000668 | 0.00000334 | 0.00000334 | 40.00 | 0.000131 | 0.000131 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 272.6k | 0.910 | 0.910 | 0.00000334 | 0.910 | - | - | - |
| involuntary context switches | 224.0 | 0.000748 | 0.000748 | 0.000664 | 0.0000835 | - | - | - |
| cycles | - | - | - | 365.0k | 15.74k | - | - | - |
| instructions | - | - | - | 679.8k | 20.03k | - | - | - |
| cache references | - | - | - | 1.890 | 18.37 | - | - | - |
| cache misses | - | - | - | 0.541 | 1.993 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0395 | 0.0288 | - | - | - |
| data TLB read misses | - | - | - | 0.0697 | 0.508 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.754M | - | - | - | - | 56.40M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.754M | - | - | - | - | 56.06M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.192M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304605 messages, 304605 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3064427-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3064427-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 573.4 | 876.5 | 1163.3 | 1359.9 | 2169.4 | 0 | 0 | 100/100 | 123.3 | 34.9 | 19.4 | 6.87 / 5.40 |
| best | redis | 22 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 573.4 | 876.5 | 1163.3 | 1359.9 | 2169.4 | 0 | 0 | 100/100 | 123.3 | 34.9 | 19.4 | 6.87 / 5.40 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 29949704 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 104.2G | 347.9k | 3479 | 98.48k | 2494 | 16.67G | 54.73k | 547.3 |
| waiting for a CPU, ns | - | - | - | 2.838 | 1.431 | - | - | - |
| allocations | 130.8M | 436.7 | 4.367 | 7.218 | 4.295 | 914.9k | 3.003 | 0.0300 |
| reallocations | 1.751M | 5.846 | 0.0585 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 1.751M | 5.846 | 0.0585 | 1.999 | 0.0385 | - | - | - |
| frees | 130.8M | 436.7 | 4.367 | 7.218 | 4.295 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 16.39G | 54.71k | 547.1 | 2276 | 524.3 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 0 | 0 | 0 | 0 | 0 | 133.0 | 0.000437 | 0.00000437 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 6.125M | 20.45 | 0.205 | 0 | 0.205 | - | - | - |
| involuntary context switches | 2569 | 0.00858 | 0.0000858 | 0.000411 | 0.0000817 | - | - | - |
| cycles | - | - | - | 525.4k | 8808 | - | - | - |
| instructions | - | - | - | 957.2k | 14.91k | - | - | - |
| cache references | - | - | - | 32.69 | 5.440 | - | - | - |
| cache misses | - | - | - | 3.920 | 0.317 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0224 | 0.00804 | - | - | - |
| data TLB read misses | - | - | - | 0.445 | 0.188 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 14.70M | - | - | - | - | 61.22M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 14.70M | - | - | - | - | 59.81M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 2.389M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304629 messages, 30463846 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3070678-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3070678-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 100701 | 2835 | 253889 | 0 | 85871000 | 43077000 | 256420000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 7560000 | 0/1000 | 97.2 | 0.5 | 32.1 | 10.21 / 9.29 |
| best | redis | 22 × wait | 100701 | 2835 | 253889 | 0 | 85871000 | 43077000 | 256420000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 7560000 | 0/1000 | 97.2 | 0.5 | 32.1 | 10.21 / 9.29 |

Run 1's profile:

| profile: the window, 30.0 s, 84366 messages, 71795892 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 80.90G | 958.9k | 1127 | 3626 | 1123 | 27.38G | 318.5k | 374.4 |
| waiting for a CPU, ns | - | - | - | 3.772 | 2.900 | - | - | - |
| allocations | 346.0M | 4101 | 4.819 | 7.017 | 4.811 | 2.905M | 33.79 | 0.0397 |
| reallocations | 3.417M | 40.50 | 0.0476 | 2.000 | 0.0452 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 3.408M | 40.39 | 0.0475 | 1.980 | 0.0451 | - | - | - |
| frees | 346.0M | 4101 | 4.819 | 7.017 | 4.811 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 43.94G | 520.8k | 612.0 | 2271 | 609.3 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 9785 | 0.116 | 0.000136 | 0.0000237 | 0.000136 | 1.170M | 13.61 | 0.0160 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 739.2k | 8.762 | 0.0103 | 0.0162 | 0.0103 | - | - | - |
| involuntary context switches | 2392 | 0.0284 | 0.0000333 | 0.0000830 | 0.0000332 | - | - | - |
| cycles | - | - | - | 24.86k | 7937 | - | - | - |
| instructions | - | - | - | 21.71k | 15.47k | - | - | - |
| cache references | - | - | - | 325.3 | 20.46 | - | - | - |
| cache misses | - | - | - | 277.2 | 10.42 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 26.43 | 0.297 | - | - | - |
| data TLB read misses | - | - | - | 11.39 | 0.536 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 109.1M | - | - | - | - | 105.6M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 109.1M | - | - | - | - | 93.31M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.58 s from a quarter second before the window to a quarter second after it, 85974 messages, 73143546 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3077238-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3077238-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 18.7 | 24.6 | 36.4 | 176.1 | 997.9 | 0 | 0 | 1/1 | 42.0 | 29.9 | 12.9 | 3.13 / 2.74 |
| best | redis | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 18.7 | 24.6 | 36.4 | 176.1 | 997.9 | 0 | 0 | 1/1 | 42.0 | 29.9 | 12.9 | 3.13 / 2.74 |

Run 1's profile:

| profile: the window, 30.0 s, 2998275 messages, 2998274 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 35.45G | 11.82k | 11.82k | 8419 | 3403 | 11.10G | 3641 | 3641 |
| waiting for a CPU, ns | - | - | - | 0.813 | 1.386 | - | - | - |
| allocations | 37.89M | 12.64 | 12.64 | 7.784 | 4.855 | 9.145M | 3.000 | 3.000 |
| reallocations | 6.112M | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 6.111M | 2.038 | 2.038 | 2.000 | 0.0385 | - | - | - |
| frees | 37.89M | 12.64 | 12.64 | 7.784 | 4.855 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 8.506G | 2837 | 2837 | 2295 | 542.3 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 0 | 0 | 0 | 0 | 0 | 43.00 | 0.0000141 | 0.0000141 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 2.241M | 0.747 | 0.747 | 0 | 0.747 | - | - | - |
| involuntary context switches | 476.0 | 0.000159 | 0.000159 | 0.000141 | 0.0000173 | - | - | - |
| cycles | - | - | - | 27.06k | 10.37k | - | - | - |
| instructions | - | - | - | 49.96k | 15.22k | - | - | - |
| cache references | - | - | - | 0.586 | 2.792 | - | - | - |
| cache misses | - | - | - | 0.0536 | 0.132 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00257 | 0.00115 | - | - | - |
| data TLB read misses | - | - | - | 0.0636 | 0.0791 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.889M | - | - | - | - | 57.86M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.889M | - | - | - | - | 56.59M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.248M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3048253 messages, 3048252 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3088832-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3088832-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 1007704 | 28333 | 2539839 | 0 | 85284400 | 43012300 | 256815100 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 7821000 | 0/100 | 99.7 | 2.5 | 31.2 | 11.92 / 8.92 |
| best | redis | 22 × wait | 1007704 | 28333 | 2539839 | 0 | 85284400 | 43012300 | 256815100 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 7821000 | 0/100 | 99.7 | 2.5 | 31.2 | 11.92 / 8.92 |

Run 1's profile:

| profile: the window, 30.0 s, 853320 messages, 72203939 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 84.41G | 98.92k | 1169 | 2389 | 1141 | 26.76G | 30.85k | 364.5 |
| waiting for a CPU, ns | - | - | - | 2.790 | 0.941 | - | - | - |
| allocations | 355.2M | 416.2 | 4.919 | 7.016 | 4.836 | 5.271M | 6.075 | 0.0718 |
| reallocations | 4.989M | 5.846 | 0.0691 | 2.000 | 0.0455 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 4.973M | 5.828 | 0.0689 | 1.981 | 0.0455 | - | - | - |
| frees | 355.2M | 416.2 | 4.919 | 7.016 | 4.836 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 46.16G | 54.09k | 639.3 | 2271 | 612.4 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 9277 | 0.0109 | 0.000128 | 0.00000234 | 0.000128 | 109.0 | 0.000126 | 0.00000148 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 944.6k | 1.107 | 0.0131 | 0.0154 | 0.0129 | - | - | - |
| involuntary context switches | 1574 | 0.00184 | 0.0000218 | 0.0000188 | 0.0000216 | - | - | - |
| cycles | - | - | - | 13.31k | 7366 | - | - | - |
| instructions | - | - | - | 22.65k | 14.92k | - | - | - |
| cache references | - | - | - | 135.5 | 3.064 | - | - | - |
| cache misses | - | - | - | 9.984 | 0.161 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.779 | 0.00578 | - | - | - |
| data TLB read misses | - | - | - | 3.533 | 0.117 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 57.73M | - | - | - | - | 61.96M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 57.73M | - | - | - | - | 59.87M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 5.075M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 867685 messages, 73421688 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3094235-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3094235-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 83570 | 2345 | 3463973 | 0 | 71240000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 85.8 | 0.5 | 31.4 | 14.05 / 10.51 |
| best | redis | 22 × wait | 83570 | 2345 | 3463973 | 0 | 71240000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 85.8 | 0.5 | 31.4 | 14.05 / 10.51 |

Run 1's profile:

| profile: the window, 30.0 s, 69931 messages, 59501000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 71.46G | 1.022M | 1201 | 4396 | 1196 | 26.75G | 375.2k | 441.1 |
| waiting for a CPU, ns | - | - | - | 7.414 | 1.660 | - | - | - |
| allocations | 286.7M | 4100 | 4.819 | 7.017 | 4.810 | 2.406M | 33.76 | 0.0397 |
| reallocations | 2.833M | 40.51 | 0.0476 | 2.000 | 0.0453 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.831M | 40.48 | 0.0476 | 1.982 | 0.0452 | - | - | - |
| frees | 286.7M | 4100 | 4.819 | 7.017 | 4.810 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 36.42G | 520.8k | 612.1 | 2271 | 609.4 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 8944 | 0.128 | 0.000150 | 0.000129 | 0.000150 | 983.1k | 13.79 | 0.0162 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.060M | 15.16 | 0.0178 | 0.0160 | 0.0178 | - | - | - |
| involuntary context switches | 1443 | 0.0206 | 0.0000243 | 0.000129 | 0.0000241 | - | - | - |
| cycles | - | - | - | 28.54k | 7852 | - | - | - |
| instructions | - | - | - | 22.97k | 15.53k | - | - | - |
| cache references | - | - | - | 355.3 | 22.86 | - | - | - |
| cache misses | - | - | - | 310.3 | 11.28 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 26.66 | 0.393 | - | - | - |
| data TLB read misses | - | - | - | 13.35 | 0.548 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 102.1M | - | - | - | - | 115.1M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 102.1M | - | - | - | - | 89.40M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.59 s from a quarter second before the window to a quarter second after it, 71287 messages, 60647175 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3104982-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3104982-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 14.6 | 20.5 | 67.6 | 215.0 | 345.8 | 0 | 0 | 1/1 | 70.0 | 34.8 | 2.2 | 4.56 / 3.86 |
| best | redis | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 14.6 | 20.5 | 67.6 | 215.0 | 345.8 | 0 | 0 | 1/1 | 70.0 | 34.8 | 2.2 | 4.56 / 3.86 |

Run 1's profile:

| profile: the window, 30.0 s, 299500 messages, 299500 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 59.12G | 197.4k | 197.4k | 98.12k | 99.28k | 1.893G | 6213 | 6213 |
| waiting for a CPU, ns | - | - | - | 5.850 | 3.968 | - | - | - |
| allocations | 3.905M | 13.04 | 13.04 | 7.981 | 5.058 | 947.6k | 3.111 | 3.111 |
| reallocations | 610.5k | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 610.5k | 2.038 | 2.038 | 2.000 | 0.0385 | - | - | - |
| frees | 3.905M | 13.04 | 13.04 | 7.981 | 5.058 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 853.6M | 2850 | 2850 | 2301 | 548.8 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 0 | 0 | 0 | 0 | 0 | 41.00 | 0.000135 | 0.000135 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 377.0 | 0.00126 | 0.00126 | 0.000681 | 0.000578 | - | - | - |
| cycles | - | - | - | 351.9k | 402.2k | - | - | - |
| instructions | - | - | - | 654.7k | 695.9k | - | - | - |
| cache references | - | - | - | 3.246 | 2.795 | - | - | - |
| cache misses | - | - | - | 1.319 | 0.746 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.138 | 0.0873 | - | - | - |
| data TLB read misses | - | - | - | 0.0667 | 0.00710 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.734M | - | - | - | - | 56.68M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.734M | - | - | - | - | 55.77M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.216M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304612 messages, 304612 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3120439-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3120439-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 729.1 | 1163.3 | 1867.8 | 2818.0 | 4615.4 | 0 | 0 | 100/100 | 805.6 | 34.1 | 21.6 | 24.55 / 24.54 |
| best | redis | 22 × busy | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 729.1 | 1163.3 | 1867.8 | 2818.0 | 4615.4 | 0 | 0 | 100/100 | 805.6 | 34.1 | 21.6 | 24.55 / 24.54 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 29949752 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 680.1G | 2.271M | 22.71k | 96.33k | 21.74k | 18.48G | 60.66k | 606.6 |
| waiting for a CPU, ns | - | - | - | 6.134 | 1.467 | - | - | - |
| allocations | 129.5M | 432.4 | 4.324 | 7.176 | 4.253 | 917.1k | 3.011 | 0.0301 |
| reallocations | 1.751M | 5.846 | 0.0585 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 1.750M | 5.844 | 0.0584 | 1.998 | 0.0385 | - | - | - |
| frees | 129.5M | 432.4 | 4.324 | 7.176 | 4.253 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 16.35G | 54.58k | 545.8 | 2276 | 523.0 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 7.000 | 0.0000234 | 0.000000234 | 0 | 0.000000234 | 276.0 | 0.000906 | 0.00000906 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 3247 | 0.0108 | 0.000108 | 0.000321 | 0.000105 | - | - | - |
| cycles | - | - | - | 173.0k | 54.17k | - | - | - |
| instructions | - | - | - | 308.8k | 85.56k | - | - | - |
| cache references | - | - | - | 1.952 | 0.746 | - | - | - |
| cache misses | - | - | - | 1.027 | 0.178 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.00147 | - | - | - |
| data TLB read misses | - | - | - | 0.00167 | 0.0127 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 13.43M | - | - | - | - | 61.91M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 13.43M | - | - | - | - | 59.51M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 2.594M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304626 messages, 30463584 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3127914-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3127914-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 76754 | 2161 | 277836 | 0 | 65254000 | 22620000 | 276877000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 4070000 | 0/1000 | 790.4 | 0.4 | 31.8 | 29.92 / 26.68 |
| best | redis | 22 × busy | 76754 | 2161 | 277836 | 0 | 65254000 | 22620000 | 276877000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 4070000 | 0/1000 | 790.4 | 0.4 | 31.8 | 29.92 / 26.68 |

Run 1's profile:

| profile: the window, 30.0 s, 63786 messages, 54076000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 658.1G | 10.32M | 12.17k | 3494 | 12.17k | 27.02G | 415.6k | 490.2 |
| waiting for a CPU, ns | - | - | - | 8.253 | 0.901 | - | - | - |
| allocations | 261.5M | 4100 | 4.836 | 7.017 | 4.828 | 2.233M | 34.34 | 0.0405 |
| reallocations | 2.585M | 40.52 | 0.0478 | 2.000 | 0.0454 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.582M | 40.48 | 0.0477 | 1.987 | 0.0454 | - | - | - |
| frees | 261.5M | 4100 | 4.836 | 7.017 | 4.828 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 33.22G | 520.8k | 614.4 | 2271 | 611.7 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 8840 | 0.139 | 0.000163 | 0 | 0.000163 | 921.3k | 14.17 | 0.0167 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1032 | 0.0162 | 0.0000191 | 0.0162 | 0 | - | - | - |
| involuntary context switches | 2956 | 0.0463 | 0.0000547 | 0 | 0.0000547 | - | - | - |
| cycles | - | - | - | 11.84k | 30.61k | - | - | - |
| instructions | - | - | - | 10.36k | 50.81k | - | - | - |
| cache references | - | - | - | 148.9 | 8.053 | - | - | - |
| cache misses | - | - | - | 129.9 | 4.373 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 18.38 | 0.0543 | - | - | - |
| data TLB read misses | - | - | - | 5.868 | 0.0241 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 101.7M | - | - | - | - | 116.4M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 101.7M | - | - | - | - | 113.1M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.58 s from a quarter second before the window to a quarter second after it, 65032 messages, 55132421 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3139935-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3139935-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 15.9 | 21.2 | 26.4 | 157.7 | 335.7 | 0 | 0 | 1/1 | 63.4 | 29.9 | 13.0 | 7.19 / 5.28 |
| best | redis | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 15.9 | 21.2 | 26.4 | 157.7 | 335.7 | 0 | 0 | 1/1 | 63.4 | 29.9 | 13.0 | 7.19 / 5.28 |

Run 1's profile:

| profile: the window, 30.0 s, 2998275 messages, 2998273 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 53.52G | 17.85k | 17.85k | 8416 | 9434 | 11.14G | 3653 | 3653 |
| waiting for a CPU, ns | - | - | - | 0.821 | 1.384 | - | - | - |
| allocations | 37.95M | 12.66 | 12.66 | 7.790 | 4.866 | 9.145M | 3.000 | 3.000 |
| reallocations | 6.112M | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 5.992M | 1.998 | 1.998 | 1.960 | 0.0385 | - | - | - |
| frees | 37.94M | 12.66 | 12.66 | 7.790 | 4.866 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 8.506G | 2837 | 2837 | 2294 | 542.6 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 0 | 0 | 0 | 0 | 0 | 43.00 | 0.0000141 | 0.0000141 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 807.0 | 0.000269 | 0.000269 | 0.000133 | 0.000136 | - | - | - |
| cycles | - | - | - | 27.16k | 36.27k | - | - | - |
| instructions | - | - | - | 48.70k | 63.65k | - | - | - |
| cache references | - | - | - | 0.513 | 0.385 | - | - | - |
| cache misses | - | - | - | 0.0534 | 0.0306 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00295 | 0.00194 | - | - | - |
| data TLB read misses | - | - | - | 0.0556 | 0.0110 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.750M | - | - | - | - | 58.24M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.750M | - | - | - | - | 55.84M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.246M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3048243 messages, 3048242 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3149775-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3149775-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 842847 | 23670 | 2704696 | 0 | 71947700 | 29311600 | 270515800 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 5036000 | 0/100 | 783.3 | 2.1 | 31.5 | 25.00 / 24.61 |
| best | redis | 22 × busy | 842847 | 23670 | 2704696 | 0 | 71947700 | 29311600 | 270515800 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 5036000 | 0/100 | 783.3 | 2.1 | 31.5 | 25.00 / 24.61 |

Run 1's profile:

| profile: the window, 30.0 s, 715816 messages, 61086600 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 659.3G | 921.1k | 10.79k | 2292 | 10.77k | 27.06G | 37.18k | 435.7 |
| waiting for a CPU, ns | - | - | - | 10.74 | 0.635 | - | - | - |
| allocations | 298.0M | 416.3 | 4.878 | 7.016 | 4.796 | 4.315M | 5.930 | 0.0695 |
| reallocations | 4.185M | 5.847 | 0.0685 | 2.000 | 0.0451 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 4.180M | 5.839 | 0.0684 | 1.992 | 0.0451 | - | - | - |
| frees | 298.0M | 416.3 | 4.878 | 7.016 | 4.796 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 38.73G | 54.11k | 634.0 | 2271 | 607.4 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 8738 | 0.0122 | 0.000143 | 0.00000978 | 0.000143 | 116.0 | 0.000159 | 0.00000187 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 11.46k | 0.0160 | 0.000188 | 0.0158 | 0.00000286 | - | - | - |
| involuntary context switches | 2786 | 0.00389 | 0.0000456 | 0.00000838 | 0.0000455 | - | - | - |
| cycles | - | - | - | 8280 | 35.78k | - | - | - |
| instructions | - | - | - | 10.24k | 58.81k | - | - | - |
| cache references | - | - | - | 65.06 | 7.196 | - | - | - |
| cache misses | - | - | - | 23.04 | 1.412 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 2.784 | 0.0183 | - | - | - |
| data TLB read misses | - | - | - | 5.966 | 0.527 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 50.75M | - | - | - | - | 52.69M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 50.75M | - | - | - | - | 49.93M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 3.024M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 727690 messages, 62099801 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3154116-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3154116-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 65105 | 1827 | 3482438 | 0 | 55245000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 783.0 | 0.4 | 32.6 | 35.64 / 34.56 |
| best | redis | 22 × busy | 65105 | 1827 | 3482438 | 0 | 55245000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 783.0 | 0.4 | 32.6 | 35.64 / 34.56 |

Run 1's profile:

| profile: the window, 30.0 s, 53469 messages, 45194358 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 652.0G | 12.19M | 14.43k | 3975 | 14.42k | 27.84G | 509.0k | 601.7 |
| waiting for a CPU, ns | - | - | - | 2712 | 50.48 | - | - | - |
| allocations | 219.0M | 4096 | 4.846 | 7.016 | 4.838 | 1.898M | 34.70 | 0.0410 |
| reallocations | 2.164M | 40.47 | 0.0479 | 2.000 | 0.0455 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.161M | 40.42 | 0.0478 | 1.983 | 0.0455 | - | - | - |
| frees | 219.0M | 4096 | 4.846 | 7.016 | 4.838 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 27.81G | 520.1k | 615.3 | 2271 | 612.6 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 6667 | 0.125 | 0.000148 | 0.000131 | 0.000147 | 774.6k | 14.16 | 0.0167 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 824.0 | 0.0154 | 0.0000182 | 0.0154 | 0 | - | - | - |
| involuntary context switches | 36.35k | 0.680 | 0.000804 | 0.00520 | 0.000798 | - | - | - |
| cycles | - | - | - | 23.42k | 40.47k | - | - | - |
| instructions | - | - | - | 19.66k | 65.69k | - | - | - |
| cache references | - | - | - | 274.5 | 10.70 | - | - | - |
| cache misses | - | - | - | 243.4 | 5.958 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 19.27 | 0.114 | - | - | - |
| data TLB read misses | - | - | - | 8.938 | 0.0539 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 85.17M | - | - | - | - | 77.23M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 85.17M | - | - | - | - | 73.90M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.58 s from a quarter second before the window to a quarter second after it, 54702 messages, 46272426 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3164657-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3164657-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 29.7 | 46.1 | 117.8 | 266.2 | 466.4 | 0 | 0 | 1/1 | 37.4 | 33.9 | 2.4 | 14.67 / 10.67 |
| best | redis | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 29.7 | 46.1 | 117.8 | 266.2 | 466.4 | 0 | 0 | 1/1 | 37.4 | 33.9 | 2.4 | 14.67 / 10.67 |

Run 1's profile:

| profile: the window, 30.0 s, 299581 messages, 299581 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 31.60G | 105.5k | 105.5k | 95.56k | 9905 | 2.035G | 6680 | 6680 |
| waiting for a CPU, ns | - | - | - | 10.05 | 5.660 | - | - | - |
| allocations | 3.895M | 13.00 | 13.00 | 7.975 | 5.028 | 913.8k | 3.000 | 3.000 |
| reallocations | 610.7k | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 610.7k | 2.038 | 2.038 | 2.000 | 0.0385 | - | - | - |
| frees | 3.895M | 13.00 | 13.00 | 7.975 | 5.028 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 853.5M | 2849 | 2849 | 2301 | 547.8 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 0 | 0 | 0 | 0 | 0 | 44.00 | 0.000144 | 0.000144 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 256.3k | 0.855 | 0.855 | 0 | 0.855 | - | - | - |
| involuntary context switches | 253.0 | 0.000845 | 0.000845 | 0.000748 | 0.0000968 | - | - | - |
| cycles | - | - | - | 199.4k | 11.16k | - | - | - |
| instructions | - | - | - | 348.3k | 10.21k | - | - | - |
| cache references | - | - | - | 13.24 | 15.51 | - | - | - |
| cache misses | - | - | - | 11.11 | 9.586 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0 | - | - | - |
| data TLB read misses | - | - | - | 0.000244 | 0.247 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.586M | - | - | - | - | 35.00M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.586M | - | - | - | - | 33.12M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.216M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304594 messages, 304593 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3176001-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3176001-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 1409.0 | 2293.8 | 3014.7 | 3538.9 | 4856.6 | 0 | 0 | 100/100 | 102.7 | 34.6 | 23.3 | 18.30 / 15.15 |
| best | redis | 22 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 1409.0 | 2293.8 | 3014.7 | 3538.9 | 4856.6 | 0 | 0 | 100/100 | 102.7 | 34.6 | 23.3 | 18.30 / 15.15 |

Run 1's profile:

| profile: the window, 30.0 s, 299584 messages, 29956640 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 86.16G | 287.6k | 2876 | 97.73k | 1899 | 20.07G | 65.89k | 658.8 |
| waiting for a CPU, ns | - | - | - | 21.88 | 2.264 | - | - | - |
| allocations | 127.3M | 424.9 | 4.250 | 7.102 | 4.179 | 917.9k | 3.013 | 0.0301 |
| reallocations | 1.751M | 5.846 | 0.0585 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 1.747M | 5.832 | 0.0583 | 1.987 | 0.0385 | - | - | - |
| frees | 127.3M | 424.9 | 4.250 | 7.102 | 4.179 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 16.28G | 54.33k | 543.4 | 2273 | 520.6 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 38.00 | 0.000127 | 0.00000127 | 0.0000134 | 0.00000113 | 133.0 | 0.000437 | 0.00000437 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 2.570M | 8.577 | 0.0858 | 0 | 0.0858 | - | - | - |
| involuntary context switches | 971.0 | 0.00324 | 0.0000324 | 0.00118 | 0.0000206 | - | - | - |
| cycles | - | - | - | 444.3k | 7539 | - | - | - |
| instructions | - | - | - | 811.1k | 12.92k | - | - | - |
| cache references | - | - | - | 73.50 | 23.34 | - | - | - |
| cache misses | - | - | - | 22.47 | 4.007 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.630 | 0.103 | - | - | - |
| data TLB read misses | - | - | - | 0.109 | 0.778 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 13.74M | - | - | - | - | 38.62M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 13.74M | - | - | - | - | 36.03M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 2.746M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 304645 messages, 30465937 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3183423-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3183423-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 89077 | 2507 | 265721 | 0 | 76447000 | 33326000 | 266257000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 5600000 | 0/1000 | 87.0 | 0.5 | 31.7 | 15.95 / 10.21 |
| best | redis | 22 × wait | 89077 | 2507 | 265721 | 0 | 76447000 | 33326000 | 266257000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 5600000 | 0/1000 | 87.0 | 0.5 | 31.7 | 15.95 / 10.21 |

Run 1's profile:

| profile: the window, 30.0 s, 74756 messages, 64013000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 72.67G | 972.1k | 1135 | 3841 | 1131 | 26.96G | 353.9k | 413.3 |
| waiting for a CPU, ns | - | - | - | 3.041 | 1.875 | - | - | - |
| allocations | 306.5M | 4100 | 4.788 | 7.017 | 4.780 | 2.488M | 32.67 | 0.0381 |
| reallocations | 3.028M | 40.50 | 0.0473 | 2.000 | 0.0450 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 3.025M | 40.46 | 0.0473 | 1.987 | 0.0449 | - | - | - |
| frees | 306.5M | 4100 | 4.788 | 7.017 | 4.780 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 38.93G | 520.7k | 608.1 | 2271 | 605.5 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 9189 | 0.123 | 0.000144 | 0.000107 | 0.000143 | 1.009M | 13.25 | 0.0155 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 943.9k | 12.63 | 0.0147 | 0.0158 | 0.0147 | - | - | - |
| involuntary context switches | 2020 | 0.0270 | 0.0000316 | 0.0000803 | 0.0000315 | - | - | - |
| cycles | - | - | - | 24.68k | 7618 | - | - | - |
| instructions | - | - | - | 21.23k | 15.24k | - | - | - |
| cache references | - | - | - | 302.3 | 18.32 | - | - | - |
| cache misses | - | - | - | 260.2 | 9.419 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 24.43 | 0.297 | - | - | - |
| data TLB read misses | - | - | - | 9.795 | 0.373 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 102.6M | - | - | - | - | 93.77M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 102.6M | - | - | - | - | 66.73M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.59 s from a quarter second before the window to a quarter second after it, 76167 messages, 65218028 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3202550-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3202550-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × wait | 3550038 | 99998 | 0 | 0 | 3549961 | 3000379 | 58 | 56.8 | 106.5 | 901.1 | 2129.9 | undelivered | 0 | 58 | 0/1 | 43.9 | 26.5 | 18.1 | 38.10 / 34.24 |
| best | redis | 1 × wait | 3550038 | 99998 | 0 | 0 | 3549961 | 3000379 | 58 | 56.8 | 106.5 | 901.1 | 2129.9 | undelivered | 0 | 58 | 0/1 | 43.9 | 26.5 | 18.1 | 38.10 / 34.24 |

Run 1's profile:

| profile: the window, 30.0 s, 3000443 messages, 3000385 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 36.85G | 12.28k | 12.28k | 7443 | 4838 | 15.80G | 5179 | 5179 |
| waiting for a CPU, ns | - | - | - | 12.53 | 717.6 | - | - | - |
| allocations | 35.90M | 11.97 | 11.97 | 7.471 | 4.494 | 9.153M | 3.000 | 3.000 |
| reallocations | 6.116M | 2.038 | 2.039 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 6.108M | 2.036 | 2.036 | 1.998 | 0.0383 | - | - | - |
| frees | 35.90M | 11.97 | 11.97 | 7.471 | 4.494 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 8.450G | 2816 | 2816 | 2285 | 531.4 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 87.00 | 0.0000290 | 0.0000290 | 0.0000170 | 0.0000120 | 40.00 | 0.0000131 | 0.0000131 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 891.6k | 0.297 | 0.297 | 0 | 0.297 | - | - | - |
| involuntary context switches | 5626 | 0.00188 | 0.00188 | 0.000426 | 0.00145 | - | - | - |
| cycles | - | - | - | 30.39k | 14.00k | - | - | - |
| instructions | - | - | - | 43.12k | 15.61k | - | - | - |
| cache references | - | - | - | 26.91 | 124.8 | - | - | - |
| cache misses | - | - | - | 11.12 | 14.81 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.293 | 0.369 | - | - | - |
| data TLB read misses | - | - | - | 0.119 | 2.733 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.549M | - | - | - | - | 23.88M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.549M | - | - | - | - | 23.02M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.252M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3050457 messages, 3050615 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3280064-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3280064-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 970557 | 27254 | 2579481 | 0 | 82993700 | 40242800 | 259800900 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 6830000 | 0/100 | 98.2 | 2.6 | 31.3 | 17.27 / 12.79 |
| best | redis | 22 × wait | 970557 | 27254 | 2579481 | 0 | 82993700 | 40242800 | 259800900 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 6830000 | 0/100 | 98.2 | 2.6 | 31.3 | 17.27 / 12.79 |

Run 1's profile:

| profile: the window, 30.0 s, 827376 messages, 70729695 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 83.70G | 101.2k | 1183 | 2530 | 1154 | 26.89G | 32.00k | 374.3 |
| waiting for a CPU, ns | - | - | - | 6.282 | 3.588 | - | - | - |
| allocations | 344.4M | 416.3 | 4.870 | 7.017 | 4.788 | 4.958M | 5.900 | 0.0690 |
| reallocations | 4.837M | 5.846 | 0.0684 | 2.000 | 0.0450 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 4.823M | 5.829 | 0.0682 | 1.983 | 0.0450 | - | - | - |
| frees | 344.4M | 416.3 | 4.870 | 7.017 | 4.788 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 44.76G | 54.10k | 632.8 | 2271 | 606.3 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 9187 | 0.0111 | 0.000130 | 0 | 0.000130 | 39.00 | 0.0000464 | 0.000000543 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 598.6k | 0.723 | 0.00846 | 0.0160 | 0.00828 | - | - | - |
| involuntary context switches | 2373 | 0.00287 | 0.0000336 | 0.0000532 | 0.0000329 | - | - | - |
| cycles | - | - | - | 14.95k | 7588 | - | - | - |
| instructions | - | - | - | 21.05k | 14.80k | - | - | - |
| cache references | - | - | - | 173.4 | 16.00 | - | - | - |
| cache misses | - | - | - | 50.98 | 1.008 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 4.026 | 0.0170 | - | - | - |
| data TLB read misses | - | - | - | 4.985 | 0.445 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 55.78M | - | - | - | - | 27.96M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 55.78M | - | - | - | - | 25.74M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 5.075M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 840302 messages, 71839155 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3291595-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3291595-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 79806 | 2236 | 3470232 | 0 | 67686000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 86.8 | 0.5 | 32.1 | 15.40 / 12.81 |
| best | redis | 22 × wait | 79806 | 2236 | 3470232 | 0 | 67686000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 86.8 | 0.5 | 32.1 | 15.40 / 12.81 |

Run 1's profile:

| profile: the window, 30.0 s, 66430 messages, 56210000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 72.36G | 1.089M | 1287 | 4638 | 1282 | 27.29G | 402.9k | 476.2 |
| waiting for a CPU, ns | - | - | - | 1028 | 285.5 | - | - | - |
| allocations | 272.3M | 4099 | 4.845 | 7.016 | 4.836 | 2.348M | 34.67 | 0.0410 |
| reallocations | 2.691M | 40.51 | 0.0479 | 2.000 | 0.0455 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.681M | 40.36 | 0.0477 | 1.985 | 0.0454 | - | - | - |
| frees | 272.3M | 4099 | 4.845 | 7.016 | 4.836 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 34.60G | 520.8k | 615.5 | 2271 | 612.8 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 31.15k | 0.469 | 0.000554 | 0.000120 | 0.000554 | 960.7k | 14.18 | 0.0168 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 837.8k | 12.61 | 0.0149 | 0.0154 | 0.0149 | - | - | - |
| involuntary context switches | 14.36k | 0.216 | 0.000256 | 0.000286 | 0.000255 | - | - | - |
| cycles | - | - | - | 28.51k | 8103 | - | - | - |
| instructions | - | - | - | 19.74k | 14.96k | - | - | - |
| cache references | - | - | - | 256.5 | 25.23 | - | - | - |
| cache misses | - | - | - | 225.3 | 12.04 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 22.19 | 0.448 | - | - | - |
| data TLB read misses | - | - | - | 11.34 | 0.727 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 108.8M | - | - | - | - | 77.14M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 108.8M | - | - | - | - | 57.26M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.59 s from a quarter second before the window to a quarter second after it, 67730 messages, 57309560 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3312123-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3312123-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 24.8 | 69.6 | 852.0 | 1949.7 | 5261.3 | 0 | 0 | 1/1 | 69.4 | 34.3 | 3.5 | 29.55 / 20.03 |
| best | redis | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 24.8 | 69.6 | 852.0 | 1949.7 | 5261.3 | 0 | 0 | 1/1 | 69.4 | 34.3 | 3.5 | 29.55 / 20.03 |

Run 1's profile:

| profile: the window, 30.0 s, 299587 messages, 299589 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 58.56G | 195.5k | 195.5k | 96.66k | 98.80k | 3.061G | 10.05k | 10.05k |
| waiting for a CPU, ns | - | - | - | 71.38 | 41.05 | - | - | - |
| allocations | 3.860M | 12.88 | 12.88 | 7.904 | 4.980 | 913.8k | 2.999 | 2.999 |
| reallocations | 610.7k | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 610.6k | 2.038 | 2.038 | 2.000 | 0.0384 | - | - | - |
| frees | 3.860M | 12.88 | 12.88 | 7.905 | 4.980 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 852.4M | 2845 | 2845 | 2299 | 546.3 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 14.00 | 0.0000467 | 0.0000467 | 0.0000467 | 0 | 40.00 | 0.000131 | 0.000131 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 760.0 | 0.00254 | 0.00254 | 0.00178 | 0.000758 | - | - | - |
| cycles | - | - | - | 455.6k | 546.3k | - | - | - |
| instructions | - | - | - | 807.1k | 912.6k | - | - | - |
| cache references | - | - | - | 150.2 | 146.3 | - | - | - |
| cache misses | - | - | - | 82.13 | 66.94 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 1.246 | 0.739 | - | - | - |
| data TLB read misses | - | - | - | 0.186 | 0.0816 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.717M | - | - | - | - | 21.48M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.717M | - | - | - | - | 20.01M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.216M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 304731 messages, 304731 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3361951-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3361951-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 354798 | 9994 | 0 | 0 | 35438600 | 29951200 | 7100 | 2162.7 | 4161.5 | 8126.5 | 60817.4 | undelivered | 0 | 7100 | 0/100 | 680.6 | 28.8 | 24.7 | 46.60 / 42.45 |
| best | redis | 22 × busy | 354798 | 9994 | 0 | 0 | 35438600 | 29951200 | 7100 | 2162.7 | 4161.5 | 8126.5 | 60817.4 | undelivered | 0 | 7100 | 0/100 | 680.6 | 28.8 | 24.7 | 46.60 / 42.45 |

Run 1's profile:

| profile: the window, 30.0 s, 299585 messages, 29951728 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 586.5G | 1.958M | 19.58k | 87.17k | 18.71k | 21.47G | 70.46k | 704.7 |
| waiting for a CPU, ns | - | - | - | 9184 | 3120 | - | - | - |
| allocations | 126.1M | 420.9 | 4.210 | 7.066 | 4.140 | 922.3k | 3.027 | 0.0303 |
| reallocations | 1.752M | 5.846 | 0.0585 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 1.735M | 5.792 | 0.0579 | 1.955 | 0.0384 | - | - | - |
| frees | 126.1M | 420.9 | 4.210 | 7.066 | 4.140 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 16.25G | 54.24k | 542.5 | 2272 | 519.8 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 3611 | 0.0121 | 0.000121 | 0.000304 | 0.000118 | 288.0 | 0.000945 | 0.00000945 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 51.20k | 0.171 | 0.00171 | 0.00729 | 0.00164 | - | - | - |
| cycles | - | - | - | 141.8k | 79.65k | - | - | - |
| instructions | - | - | - | 251.6k | 120.1k | - | - | - |
| cache references | - | - | - | 21.56 | 12.49 | - | - | - |
| cache misses | - | - | - | 8.486 | 3.403 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000237 | 0.0460 | - | - | - |
| data TLB read misses | - | - | - | 0.108 | 0.124 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 32.78M | - | - | - | - | 26.78M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 32.78M | - | - | - | - | 24.71M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 3.024M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 304661 messages, 30462233 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3405930-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3405930-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 73052 | 2057 | 281746 | 0 | 62992000 | 19651000 | 279932000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3250000 | 0/1000 | 743.9 | 0.3 | 32.5 | 34.03 / 32.75 |
| best | redis | 22 × busy | 73052 | 2057 | 281746 | 0 | 62992000 | 19651000 | 279932000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3250000 | 0/1000 | 743.9 | 0.3 | 32.5 | 34.03 / 32.75 |

Run 1's profile:

| profile: the window, 30.0 s, 60708 messages, 52233000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 612.6G | 10.09M | 11.73k | 3770 | 11.72k | 27.80G | 449.9k | 522.9 |
| waiting for a CPU, ns | - | - | - | 14.01k | 799.6 | - | - | - |
| allocations | 248.9M | 4100 | 4.765 | 7.017 | 4.757 | 1.992M | 32.24 | 0.0375 |
| reallocations | 2.459M | 40.50 | 0.0471 | 2.000 | 0.0448 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.429M | 40.01 | 0.0465 | 1.987 | 0.0442 | - | - | - |
| frees | 248.9M | 4100 | 4.765 | 7.017 | 4.757 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 31.70G | 522.2k | 607.0 | 2271 | 604.3 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 53.35k | 0.879 | 0.00102 | 0.000560 | 0.00102 | 804.1k | 13.01 | 0.0151 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1064 | 0.0175 | 0.0000204 | 0.0157 | 0.00000207 | - | - | - |
| involuntary context switches | 19.75k | 0.325 | 0.000378 | 0.000132 | 0.000378 | - | - | - |
| cycles | - | - | - | 22.07k | 33.51k | - | - | - |
| instructions | - | - | - | 18.94k | 54.81k | - | - | - |
| cache references | - | - | - | 259.2 | 10.39 | - | - | - |
| cache misses | - | - | - | 226.9 | 6.077 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 20.31 | 0.109 | - | - | - |
| data TLB read misses | - | - | - | 8.564 | 0.0997 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 249.6M | - | - | - | - | 72.83M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 247.4M | - | - | - | - | 52.69M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.63 s from a quarter second before the window to a quarter second after it, 61792 messages, 53168244 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3415290-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3415290-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × busy | 3550038 | 99998 | 0 | 0 | 3549926 | 3000386 | 51 | 18.4 | 24.8 | 46.1 | 1458.2 | undelivered | 0 | 51 | 0/1 | 61.4 | 29.3 | 14.0 | 13.18 / 10.11 |
| best | redis | 1 × busy | 3550038 | 99998 | 0 | 0 | 3549926 | 3000386 | 51 | 18.4 | 24.8 | 46.1 | 1458.2 | undelivered | 0 | 51 | 0/1 | 61.4 | 29.3 | 14.0 | 13.18 / 10.11 |

Run 1's profile:

| profile: the window, 30.0 s, 3000434 messages, 3000382 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 51.94G | 17.31k | 17.31k | 8323 | 8986 | 11.69G | 3831 | 3831 |
| waiting for a CPU, ns | - | - | - | 1.586 | 28.02 | - | - | - |
| allocations | 37.84M | 12.61 | 12.61 | 7.768 | 4.842 | 9.151M | 3.000 | 3.000 |
| reallocations | 6.116M | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 6.105M | 2.035 | 2.035 | 1.996 | 0.0384 | - | - | - |
| frees | 37.84M | 12.61 | 12.61 | 7.768 | 4.842 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 8.509G | 2836 | 2836 | 2294 | 542.1 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 68.00 | 0.0000227 | 0.0000227 | 0.00000767 | 0.0000150 | 48.00 | 0.0000157 | 0.0000157 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 1924 | 0.000641 | 0.000641 | 0.000156 | 0.000485 | - | - | - |
| cycles | - | - | - | 39.43k | 51.18k | - | - | - |
| instructions | - | - | - | 62.37k | 80.81k | - | - | - |
| cache references | - | - | - | 10.18 | 5.414 | - | - | - |
| cache misses | - | - | - | 4.893 | 1.121 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0232 | 0.0152 | - | - | - |
| data TLB read misses | - | - | - | 0.0613 | 0.116 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.414M | - | - | - | - | 22.05M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.414M | - | - | - | - | 19.98M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.244M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3050305 messages, 3050254 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3446153-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3446153-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 695715 | 19533 | 2854323 | 0 | 59163500 | 16636600 | 283407100 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2952000 | 0/100 | 743.8 | 1.9 | 31.9 | 30.24 / 30.02 |
| best | redis | 22 × busy | 695715 | 19533 | 2854323 | 0 | 59163500 | 16636600 | 283407100 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2952000 | 0/100 | 743.8 | 1.9 | 31.9 | 30.24 / 30.02 |

Run 1's profile:

| profile: the window, 30.0 s, 595217 messages, 50574334 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 645.4G | 1.084M | 12.76k | 2404 | 12.73k | 27.47G | 45.38k | 534.1 |
| waiting for a CPU, ns | - | - | - | 1414 | 219.1 | - | - | - |
| allocations | 247.9M | 416.4 | 4.901 | 7.017 | 4.818 | 3.638M | 6.011 | 0.0707 |
| reallocations | 3.480M | 5.847 | 0.0688 | 2.000 | 0.0453 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 3.447M | 5.790 | 0.0681 | 1.979 | 0.0449 | - | - | - |
| frees | 247.9M | 416.4 | 4.901 | 7.017 | 4.818 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 32.24G | 54.16k | 637.5 | 2271 | 610.7 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 8456 | 0.0142 | 0.000167 | 0 | 0.000167 | 59.00 | 0.0000975 | 0.00000115 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 10.08k | 0.0169 | 0.000199 | 0.0167 | 0.00000275 | - | - | - |
| involuntary context switches | 19.17k | 0.0322 | 0.000379 | 0.0000840 | 0.000378 | - | - | - |
| cycles | - | - | - | 14.85k | 57.42k | - | - | - |
| instructions | - | - | - | 20.15k | 90.05k | - | - | - |
| cache references | - | - | - | 141.2 | 2.743 | - | - | - |
| cache misses | - | - | - | 33.50 | 0.341 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 1.649 | 0.00757 | - | - | - |
| data TLB read misses | - | - | - | 5.908 | 0.0350 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 82.08M | - | - | - | - | 26.13M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 82.08M | - | - | - | - | 24.36M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 5.075M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 605294 messages, 51431283 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3478900-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3478900-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 75484 | 2117 | 3474554 | 0 | 65344000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 790.2 | 0.4 | 31.8 | 29.68 / 26.59 |
| best | redis | 22 × busy | 75484 | 2117 | 3474554 | 0 | 65344000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 790.2 | 0.4 | 31.8 | 29.68 / 26.59 |

Run 1's profile:

| profile: the window, 30.0 s, 63160 messages, 54570000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 658.0G | 10.42M | 12.06k | 3466 | 12.05k | 27.07G | 420.4k | 486.6 |
| waiting for a CPU, ns | - | - | - | 0.0908 | 0.825 | - | - | - |
| allocations | 259.0M | 4100 | 4.746 | 7.017 | 4.738 | 2.179M | 33.84 | 0.0392 |
| reallocations | 2.557M | 40.49 | 0.0469 | 2.000 | 0.0445 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.556M | 40.46 | 0.0468 | 1.987 | 0.0445 | - | - | - |
| frees | 259.0M | 4100 | 4.746 | 7.017 | 4.738 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 32.87G | 520.5k | 602.4 | 2271 | 599.8 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 4602 | 0.0729 | 0.0000843 | 0.0000158 | 0.0000843 | 890.6k | 13.83 | 0.0160 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1031 | 0.0163 | 0.0000189 | 0.0163 | 0 | - | - | - |
| involuntary context switches | 3140 | 0.0497 | 0.0000575 | 0.0000158 | 0.0000575 | - | - | - |
| cycles | - | - | - | 11.66k | 29.85k | - | - | - |
| instructions | - | - | - | 10.32k | 49.67k | - | - | - |
| cache references | - | - | - | 144.0 | 8.391 | - | - | - |
| cache misses | - | - | - | 124.1 | 4.574 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 17.65 | 0.0519 | - | - | - |
| data TLB read misses | - | - | - | 5.814 | 0.0194 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 85.14M | - | - | - | - | 79.50M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 85.14M | - | - | - | - | 77.43M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.58 s from a quarter second before the window to a quarter second after it, 64380 messages, 55621071 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3503240-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3503240-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 31.0 | 92.2 | 483.3 | 1212.4 | 2272.0 | 0 | 0 | 1/1 | 38.4 | 34.2 | 3.2 | 23.50 / 18.06 |
| best | redis | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 31.0 | 92.2 | 483.3 | 1212.4 | 2272.0 | 0 | 0 | 1/1 | 38.4 | 34.2 | 3.2 | 23.50 / 18.06 |

Run 1's profile:

| profile: the window, 30.0 s, 300767 messages, 300768 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 32.46G | 107.9k | 107.9k | 96.39k | 11.54k | 2.711G | 8864 | 8864 |
| waiting for a CPU, ns | - | - | - | 16.51 | 2578 | - | - | - |
| allocations | 3.888M | 12.93 | 12.93 | 7.953 | 4.974 | 917.6k | 3.000 | 3.000 |
| reallocations | 613.1k | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 613.1k | 2.038 | 2.038 | 2.000 | 0.0385 | - | - | - |
| frees | 3.888M | 12.93 | 12.93 | 7.953 | 4.974 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 851.4M | 2831 | 2831 | 2285 | 546.1 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 1.000 | 0.00000332 | 0.00000332 | 0.00000332 | 0 | 40.00 | 0.000131 | 0.000131 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 234.8k | 0.781 | 0.781 | 0.00000332 | 0.781 | - | - | - |
| involuntary context switches | 834.0 | 0.00277 | 0.00277 | 0.00130 | 0.00147 | - | - | - |
| cycles | - | - | - | 479.7k | 26.75k | - | - | - |
| instructions | - | - | - | 870.8k | 20.93k | - | - | - |
| cache references | - | - | - | 153.4 | 548.0 | - | - | - |
| cache misses | - | - | - | 36.08 | 47.51 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.369 | 0.705 | - | - | - |
| data TLB read misses | - | - | - | 0.140 | 9.506 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.640M | - | - | - | - | 23.87M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.640M | - | - | - | - | 21.42M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.238M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305835 messages, 305835 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3546413-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3546413-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 1540.1 | 2621.4 | 3735.6 | 4522.0 | 6999.7 | 0 | 0 | 100/100 | 104.2 | 34.6 | 23.3 | 32.83 / 25.27 |
| best | redis | 22 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 1540.1 | 2621.4 | 3735.6 | 4522.0 | 6999.7 | 0 | 0 | 100/100 | 104.2 | 34.6 | 23.3 | 32.83 / 25.27 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 30076246 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 86.54G | 287.7k | 2877 | 97.29k | 1904 | 20.21G | 66.05k | 660.4 |
| waiting for a CPU, ns | - | - | - | 16.58 | 199.2 | - | - | - |
| allocations | 127.4M | 423.7 | 4.237 | 7.090 | 4.166 | 918.4k | 3.001 | 0.0300 |
| reallocations | 1.758M | 5.846 | 0.0585 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 1.752M | 5.825 | 0.0582 | 1.979 | 0.0385 | - | - | - |
| frees | 127.4M | 423.7 | 4.237 | 7.090 | 4.166 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 16.33G | 54.30k | 543.0 | 2273 | 520.3 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 245.0 | 0.000815 | 0.00000815 | 0.0000199 | 0.00000795 | 312.0 | 0.00102 | 0.0000102 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 2.271M | 7.552 | 0.0755 | 0 | 0.0755 | - | - | - |
| involuntary context switches | 6050 | 0.0201 | 0.000201 | 0.000362 | 0.000198 | - | - | - |
| cycles | - | - | - | 436.9k | 8464 | - | - | - |
| instructions | - | - | - | 789.6k | 13.12k | - | - | - |
| cache references | - | - | - | 106.0 | 38.42 | - | - | - |
| cache misses | - | - | - | 45.44 | 15.18 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 1.994 | 1.102 | - | - | - |
| data TLB read misses | - | - | - | 0.0941 | 0.945 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 14.69M | - | - | - | - | 23.46M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 14.69M | - | - | - | - | 22.52M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 3.004M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.52 s from a quarter second before the window to a quarter second after it, 306011 messages, 30604240 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3585765-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3585765-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 70515 | 1984 | 285586 | 0 | 60015000 | 17051000 | 283718000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3060000 | 0/1000 | 79.5 | 0.5 | 32.0 | 15.67 / 16.82 |
| best | redis | 22 × wait | 70515 | 1984 | 285586 | 0 | 60015000 | 17051000 | 283718000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3060000 | 0/1000 | 79.5 | 0.5 | 32.0 | 15.67 / 16.82 |

Run 1's profile:

| profile: the window, 30.0 s, 58575 messages, 49646917 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 65.35G | 1.116M | 1316 | 4613 | 1311 | 27.08G | 455.7k | 536.0 |
| waiting for a CPU, ns | - | - | - | 1863 | 1063 | - | - | - |
| allocations | 240.0M | 4097 | 4.834 | 7.017 | 4.826 | 2.040M | 34.32 | 0.0404 |
| reallocations | 2.370M | 40.46 | 0.0477 | 2.000 | 0.0454 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.299M | 39.25 | 0.0463 | 1.984 | 0.0440 | - | - | - |
| frees | 240.0M | 4097 | 4.834 | 7.017 | 4.826 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 30.47G | 520.1k | 613.6 | 2271 | 611.0 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 175.5k | 2.996 | 0.00353 | 0 | 0.00353 | 827.6k | 13.92 | 0.0164 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 751.8k | 12.84 | 0.0151 | 0.0163 | 0.0151 | - | - | - |
| involuntary context switches | 11.28k | 0.193 | 0.000227 | 0.00121 | 0.000226 | - | - | - |
| cycles | - | - | - | 28.21k | 7916 | - | - | - |
| instructions | - | - | - | 21.65k | 14.30k | - | - | - |
| cache references | - | - | - | 310.0 | 26.57 | - | - | - |
| cache misses | - | - | - | 269.0 | 15.53 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 23.38 | 0.818 | - | - | - |
| data TLB read misses | - | - | - | 12.11 | 0.622 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 93.55M | - | - | - | - | 76.66M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 90.19M | - | - | - | - | 52.15M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.68 s from a quarter second before the window to a quarter second after it, 59433 messages, 50524145 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3594805-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3594805-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × wait | 3554018 | 100110 | 0 | 0 | 3553976 | 3002589 | 42 | 26.9 | 45.1 | 68.6 | 303.1 | undelivered | 0 | 42 | 0/1 | 45.4 | 29.4 | 15.0 | 9.36 / 9.36 |
| best | redis | 1 × wait | 3554018 | 100110 | 0 | 0 | 3553976 | 3002589 | 42 | 26.9 | 45.1 | 68.6 | 303.1 | undelivered | 0 | 42 | 0/1 | 45.4 | 29.4 | 15.0 | 9.36 / 9.36 |

Run 1's profile:

| profile: the window, 30.0 s, 3002632 messages, 3002590 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 38.54G | 12.83k | 12.83k | 8272 | 4562 | 12.98G | 4253 | 4253 |
| waiting for a CPU, ns | - | - | - | 1.074 | 5.246 | - | - | - |
| allocations | 37.40M | 12.46 | 12.46 | 7.703 | 4.753 | 9.158M | 3.000 | 3.000 |
| reallocations | 6.121M | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 6.120M | 2.038 | 2.038 | 2.000 | 0.0384 | - | - | - |
| frees | 37.40M | 12.46 | 12.46 | 7.703 | 4.753 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 8.502G | 2831 | 2832 | 2292 | 539.0 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 34.00 | 0.0000113 | 0.0000113 | 0.000000666 | 0.0000107 | 49.00 | 0.0000161 | 0.0000161 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.483M | 0.494 | 0.494 | 0.000000666 | 0.494 | - | - | - |
| involuntary context switches | 889.0 | 0.000296 | 0.000296 | 0.000162 | 0.000135 | - | - | - |
| cycles | - | - | - | 23.03k | 13.05k | - | - | - |
| instructions | - | - | - | 42.20k | 18.90k | - | - | - |
| cache references | - | - | - | 0.949 | 7.871 | - | - | - |
| cache misses | - | - | - | 0.345 | 1.234 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0256 | 0.0209 | - | - | - |
| data TLB read misses | - | - | - | 0.0479 | 0.104 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 7.434M | - | - | - | - | 21.21M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 7.434M | - | - | - | - | 19.19M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.252M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3052724 messages, 3052684 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3613785-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3613785-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 635625 | 17831 | 2918393 | 0 | 54011300 | 11351800 | 288911300 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2081100 | 0/100 | 78.0 | 2.7 | 30.9 | 33.35 / 33.73 |
| best | redis | 22 × wait | 635625 | 17831 | 2918393 | 0 | 54011300 | 11351800 | 288911300 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2081100 | 0/100 | 78.0 | 2.7 | 30.9 | 33.35 / 33.73 |

Run 1's profile:

| profile: the window, 30.0 s, 516384 messages, 43809087 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 64.46G | 124.8k | 1471 | 4174 | 1422 | 26.50G | 50.17k | 591.3 |
| waiting for a CPU, ns | - | - | - | 3215 | 1432 | - | - | - |
| allocations | 214.9M | 416.2 | 4.906 | 7.016 | 4.824 | 3.208M | 6.075 | 0.0716 |
| reallocations | 3.019M | 5.846 | 0.0689 | 2.000 | 0.0453 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.981M | 5.773 | 0.0680 | 1.992 | 0.0446 | - | - | - |
| frees | 214.9M | 416.2 | 4.906 | 7.016 | 4.824 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 28.02G | 54.27k | 639.7 | 2270 | 612.9 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 12.21k | 0.0236 | 0.000279 | 0.0000465 | 0.000278 | 39.00 | 0.0000738 | 0.000000870 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 411.8k | 0.797 | 0.00940 | 0.0153 | 0.00922 | - | - | - |
| involuntary context switches | 31.87k | 0.0617 | 0.000727 | 0.00285 | 0.000694 | - | - | - |
| cycles | - | - | - | 24.36k | 8208 | - | - | - |
| instructions | - | - | - | 20.89k | 14.41k | - | - | - |
| cache references | - | - | - | 264.9 | 33.70 | - | - | - |
| cache misses | - | - | - | 209.9 | 15.40 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 20.39 | 0.723 | - | - | - |
| data TLB read misses | - | - | - | 10.18 | 0.645 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 65.35M | - | - | - | - | 24.31M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 65.35M | - | - | - | - | 22.58M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 5.075M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 528137 messages, 44813543 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3652489-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3652489-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × wait | 76561 | 2145 | 3477457 | 0 | 65141000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 83.3 | 0.5 | 31.4 | 19.16 / 14.88 |
| best | redis | 22 × wait | 76561 | 2145 | 3477457 | 0 | 65141000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 83.3 | 0.5 | 31.4 | 19.16 / 14.88 |

Run 1's profile:

| profile: the window, 30.0 s, 64297 messages, 54548547 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 69.04G | 1.074M | 1266 | 4386 | 1261 | 26.70G | 408.0k | 480.7 |
| waiting for a CPU, ns | - | - | - | 2.859 | 5.023 | - | - | - |
| allocations | 263.6M | 4100 | 4.832 | 7.017 | 4.824 | 2.235M | 34.16 | 0.0402 |
| reallocations | 2.604M | 40.50 | 0.0477 | 2.000 | 0.0454 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.602M | 40.47 | 0.0477 | 1.987 | 0.0454 | - | - | - |
| frees | 263.6M | 4099 | 4.832 | 7.017 | 4.824 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 33.48G | 520.8k | 613.8 | 2271 | 611.2 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 9876 | 0.154 | 0.000181 | 0.0000778 | 0.000180 | 911.1k | 13.92 | 0.0164 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.014M | 15.77 | 0.0186 | 0.0163 | 0.0186 | - | - | - |
| involuntary context switches | 1310 | 0.0204 | 0.0000240 | 0.0000156 | 0.0000240 | - | - | - |
| cycles | - | - | - | 12.63k | 7848 | - | - | - |
| instructions | - | - | - | 10.33k | 15.60k | - | - | - |
| cache references | - | - | - | 127.9 | 22.16 | - | - | - |
| cache misses | - | - | - | 115.8 | 11.32 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.000715 | 0.396 | - | - | - |
| data TLB read misses | - | - | - | 5.575 | 0.532 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 105.0M | - | - | - | - | 76.37M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 105.0M | - | - | - | - | 58.24M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.60 s from a quarter second before the window to a quarter second after it, 65431 messages, 55531275 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3674070-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3674070-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 17.9 | 32.3 | 63.5 | 258.0 | 964.6 | 0 | 0 | 1/1 | 69.6 | 34.5 | 2.6 | 17.93 / 12.89 |
| best | redis | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 17.9 | 32.3 | 63.5 | 258.0 | 964.6 | 0 | 0 | 1/1 | 69.6 | 34.5 | 2.6 | 17.93 / 12.89 |

Run 1's profile:

| profile: the window, 30.0 s, 300767 messages, 300768 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 58.83G | 195.6k | 195.6k | 97.16k | 98.43k | 2.276G | 7441 | 7441 |
| waiting for a CPU, ns | - | - | - | 10.49 | 5.108 | - | - | - |
| allocations | 3.920M | 13.03 | 13.03 | 7.978 | 5.054 | 954.0k | 3.119 | 3.119 |
| reallocations | 613.1k | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 613.1k | 2.038 | 2.038 | 2.000 | 0.0385 | - | - | - |
| frees | 3.920M | 13.03 | 13.03 | 7.978 | 5.054 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 857.2M | 2850 | 2850 | 2301 | 548.7 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 0 | 0 | 0 | 0 | 0 | 39.00 | 0.000128 | 0.000128 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 251.0 | 0.000835 | 0.000835 | 0.000409 | 0.000426 | - | - | - |
| cycles | - | - | - | 493.7k | 589.4k | - | - | - |
| instructions | - | - | - | 891.6k | 1.012M | - | - | - |
| cache references | - | - | - | 23.18 | 16.64 | - | - | - |
| cache misses | - | - | - | 13.30 | 4.429 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.496 | 0.205 | - | - | - |
| data TLB read misses | - | - | - | 0.0751 | 0.00879 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.730M | - | - | - | - | 17.74M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.730M | - | - | - | - | 16.43M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.216M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305838 messages, 305838 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3696315-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3696315-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 696.3 | 1097.7 | 1572.9 | 3047.4 | 8773.2 | 0 | 0 | 100/100 | 804.1 | 34.5 | 21.3 | 17.29 / 20.55 |
| best | redis | 22 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 696.3 | 1097.7 | 1572.9 | 3047.4 | 8773.2 | 0 | 0 | 100/100 | 804.1 | 34.5 | 21.3 | 17.29 / 20.55 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 30076672 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 678.8G | 2.257M | 22.57k | 96.98k | 21.60k | 18.33G | 59.91k | 599.1 |
| waiting for a CPU, ns | - | - | - | 16.86 | 50.89 | - | - | - |
| allocations | 130.2M | 433.0 | 4.330 | 7.183 | 4.259 | 919.8k | 3.007 | 0.0301 |
| reallocations | 1.758M | 5.846 | 0.0585 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 1.758M | 5.844 | 0.0584 | 1.998 | 0.0385 | - | - | - |
| frees | 130.2M | 433.0 | 4.330 | 7.183 | 4.259 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 16.42G | 54.60k | 546.0 | 2276 | 523.2 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 217.0 | 0.000721 | 0.00000721 | 0.0000465 | 0.00000675 | 604.0 | 0.00197 | 0.0000197 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 12.00 | 0.0000399 | 0.000000399 | 0 | 0.000000399 | - | - | - |
| involuntary context switches | 4227 | 0.0141 | 0.000141 | 0.000359 | 0.000137 | - | - | - |
| cycles | - | - | - | 266.1k | 62.71k | - | - | - |
| instructions | - | - | - | 392.8k | 98.73k | - | - | - |
| cache references | - | - | - | 3.230 | 6.487 | - | - | - |
| cache misses | - | - | - | 0.912 | 1.076 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0585 | 0.0124 | - | - | - |
| data TLB read misses | - | - | - | 0.130 | 0.155 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 14.41M | - | - | - | - | 23.31M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 14.41M | - | - | - | - | 21.94M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 2.602M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.51 s from a quarter second before the window to a quarter second after it, 305919 messages, 30591682 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3704750-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3704750-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 69178 | 1947 | 286923 | 0 | 58838000 | 15934000 | 284835000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2840000 | 0/1000 | 741.9 | 0.4 | 32.8 | 37.04 / 35.44 |
| best | redis | 22 × busy | 69178 | 1947 | 286923 | 0 | 58838000 | 15934000 | 284835000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2840000 | 0/1000 | 741.9 | 0.4 | 32.8 | 37.04 / 35.44 |

Run 1's profile:

| profile: the window, 30.0 s, 56979 messages, 48318886 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 614.8G | 10.79M | 12.72k | 3832 | 12.72k | 27.86G | 480.2k | 565.6 |
| waiting for a CPU, ns | - | - | - | 7488 | 812.2 | - | - | - |
| allocations | 233.7M | 4101 | 4.836 | 7.018 | 4.828 | 1.996M | 34.39 | 0.0405 |
| reallocations | 2.307M | 40.49 | 0.0478 | 2.000 | 0.0454 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.281M | 40.03 | 0.0472 | 1.990 | 0.0449 | - | - | - |
| frees | 233.7M | 4101 | 4.836 | 7.018 | 4.828 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 29.72G | 521.5k | 615.0 | 2271 | 612.3 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 42.61k | 0.748 | 0.000882 | 0 | 0.000882 | 802.9k | 13.84 | 0.0163 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1013 | 0.0178 | 0.0000210 | 0.0170 | 0.000000869 | - | - | - |
| involuntary context switches | 31.53k | 0.553 | 0.000653 | 0.000333 | 0.000652 | - | - | - |
| cycles | - | - | - | 12.31k | 46.96k | - | - | - |
| instructions | - | - | - | 10.37k | 75.31k | - | - | - |
| cache references | - | - | - | 155.6 | 23.01 | - | - | - |
| cache misses | - | - | - | 140.3 | 11.90 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 19.01 | 0.237 | - | - | - |
| data TLB read misses | - | - | - | 5.828 | 0.283 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 217.0M | - | - | - | - | 75.22M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 217.0M | - | - | - | - | 52.38M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.68 s from a quarter second before the window to a quarter second after it, 58031 messages, 49264780 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3716068-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3716068-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × busy | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 15.9 | 21.2 | 26.6 | 157.7 | 391.5 | 0 | 0 | 1/1 | 63.3 | 29.8 | 13.0 | 18.12 / 11.42 |
| best | redis | 1 × busy | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 15.9 | 21.2 | 26.6 | 157.7 | 391.5 | 0 | 0 | 1/1 | 63.3 | 29.8 | 13.0 | 18.12 / 11.42 |

Run 1's profile:

| profile: the window, 30.0 s, 3002622 messages, 3002622 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 53.46G | 17.80k | 17.80k | 8394 | 9411 | 11.13G | 3647 | 3647 |
| waiting for a CPU, ns | - | - | - | 1.200 | 1.614 | - | - | - |
| allocations | 38.00M | 12.65 | 12.65 | 7.789 | 4.865 | 9.158M | 3.000 | 3.000 |
| reallocations | 6.121M | 2.038 | 2.038 | 2.000 | 0.0385 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 6.120M | 2.038 | 2.038 | 2.000 | 0.0385 | - | - | - |
| frees | 38.00M | 12.65 | 12.65 | 7.789 | 4.865 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 8.520G | 2838 | 2838 | 2295 | 542.6 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 0 | 0 | 0 | 0 | 0 | 47.00 | 0.0000154 | 0.0000154 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 873.0 | 0.000291 | 0.000291 | 0.000151 | 0.000140 | - | - | - |
| cycles | - | - | - | 26.88k | 36.27k | - | - | - |
| instructions | - | - | - | 49.50k | 63.67k | - | - | - |
| cache references | - | - | - | 0.532 | 0.361 | - | - | - |
| cache misses | - | - | - | 0.0508 | 0.0296 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00268 | 0.00178 | - | - | - |
| data TLB read misses | - | - | - | 0.0522 | 0.00699 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 6.881M | - | - | - | - | 18.10M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 6.881M | - | - | - | - | 17.00M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 1.252M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3052576 messages, 3052575 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3755068-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3755068-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 736922 | 20694 | 2817096 | 0 | 62377100 | 19933600 | 280329500 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3629000 | 0/100 | 731.4 | 2.1 | 31.5 | 20.75 / 38.65 |
| best | redis | 22 × busy | 736922 | 20694 | 2817096 | 0 | 62377100 | 19933600 | 280329500 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3629000 | 0/100 | 731.4 | 2.1 | 31.5 | 20.75 / 38.65 |

Run 1's profile:

| profile: the window, 30.0 s, 631157 messages, 53386700 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 609.1G | 965.1k | 11.41k | 2708 | 11.38k | 27.06G | 42.23k | 499.2 |
| waiting for a CPU, ns | - | - | - | 1108 | 935.5 | - | - | - |
| allocations | 262.7M | 416.3 | 4.922 | 7.016 | 4.839 | 3.899M | 6.084 | 0.0719 |
| reallocations | 3.692M | 5.849 | 0.0691 | 2.000 | 0.0455 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 3.614M | 5.727 | 0.0677 | 1.988 | 0.0442 | - | - | - |
| frees | 262.7M | 416.3 | 4.922 | 7.016 | 4.839 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 34.45G | 54.59k | 645.4 | 2271 | 618.5 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 122.8k | 0.195 | 0.00230 | 0 | 0.00230 | 5476 | 0.00854 | 0.000101 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 9910 | 0.0157 | 0.000186 | 0.0154 | 0.00000330 | - | - | - |
| involuntary context switches | 23.29k | 0.0369 | 0.000436 | 0.000290 | 0.000433 | - | - | - |
| cycles | - | - | - | 16.67k | 38.47k | - | - | - |
| instructions | - | - | - | 21.93k | 60.65k | - | - | - |
| cache references | - | - | - | 102.1 | 16.67 | - | - | - |
| cache misses | - | - | - | 69.36 | 4.856 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 8.151 | 1.009 | - | - | - |
| data TLB read misses | - | - | - | 4.252 | 0.388 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 412.0M | - | - | - | - | 119.4M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 369.0M | - | - | - | - | 38.54M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 5.075M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 640906 messages, 54216061 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3790390-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3790390-0-redis-malloc-stats.txt
# quiver-bench fanout redis

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 22 × busy | 66036 | 1852 | 3487982 | 0 | 55990000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 789.4 | 0.4 | 32.7 | 34.78 / 31.66 |
| best | redis | 22 × busy | 66036 | 1852 | 3487982 | 0 | 55990000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 789.4 | 0.4 | 32.7 | 34.78 / 31.66 |

Run 1's profile:

| profile: the window, 30.0 s, 53560 messages, 45346280 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 653.3G | 12.20M | 14.41k | 3952 | 14.40k | 27.86G | 509.6k | 601.6 |
| waiting for a CPU, ns | - | - | - | 2560 | 42.76 | - | - | - |
| allocations | 219.7M | 4102 | 4.845 | 7.016 | 4.836 | 1.896M | 34.69 | 0.0409 |
| reallocations | 2.171M | 40.53 | 0.0479 | 2.000 | 0.0455 | n/a: jemalloc counts requests alone | n/a | n/a |
| reallocations that moved their block | 2.168M | 40.48 | 0.0478 | 1.985 | 0.0455 | - | - | - |
| frees | 219.7M | 4102 | 4.845 | 7.016 | 4.836 | n/a: jemalloc counts requests alone | n/a | n/a |
| bytes allocated | 27.89G | 520.8k | 615.1 | 2271 | 612.4 | n/a: jemalloc counts requests alone | n/a | n/a |
| minor page faults | 6597 | 0.123 | 0.000145 | 0.0000934 | 0.000145 | 771.1k | 14.10 | 0.0167 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 824.0 | 0.0154 | 0.0000182 | 0.0154 | 0 | - | - | - |
| involuntary context switches | 5325 | 0.0994 | 0.000117 | 0.000131 | 0.000117 | - | - | - |
| cycles | - | - | - | 23.51k | 38.94k | - | - | - |
| instructions | - | - | - | 20.53k | 62.42k | - | - | - |
| cache references | - | - | - | 271.6 | 9.775 | - | - | - |
| cache misses | - | - | - | 245.6 | 5.613 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 18.85 | 0.0989 | - | - | - |
| data TLB read misses | - | - | - | 6.817 | 0.0505 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 0 | 0 | 0 |
| garbage collections | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| collectors' pauses, ns | - | - | - | - | - | n/a: Redis frees as it goes: no collector | n/a | n/a |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 86.40M | - | - | - | - | 79.21M | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 86.40M | - | - | - | - | 51.12M | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 39.63M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.68 s from a quarter second before the window to a quarter second after it, 54666 messages, 46309143 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3827135-0-redis-memory-stats.txt
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/redis-3827135-0-redis-malloc-stats.txt
