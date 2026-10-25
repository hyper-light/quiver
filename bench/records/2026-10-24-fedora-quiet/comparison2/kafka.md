- broker: apache/kafka:4.1.0@sha256:bff074a5d0051dbc0bbbcd25b045bb1fe84833ec0d3c7c965d1797dd289ec88f, CPUs 8-15
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 66.6 | 149.5 | 256.0 | 1228.8 | 11515.6 | 0 | 0 | 1/1 | 47.2 | 35.4 | 31.2 | 1.98 / 2.61 |
| best | kafka | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 66.6 | 149.5 | 256.0 | 1228.8 | 11515.6 | 0 | 0 | 1/1 | 47.2 | 35.4 | 31.2 | 1.98 / 2.61 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 299494 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 39.79G | 132.9k | 132.9k | 99.96k | 6900 | 16.61G | 54.54k | 54.54k |
| waiting for a CPU, ns | - | - | - | 7.330 | 10.02 | - | - | - |
| allocations | 7.037M | 23.49 | 23.49 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 7.037M | 23.49 | 23.49 | 1.000 | 4.664 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 1.680G | 5611 | 5611 | 456.0 | 24.00 | 7.144G | 23.45k | 23.45k |
| minor page faults | 2.000 | 0.00000668 | 0.00000668 | 0 | 0 | 34.34k | 0.113 | 0.113 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.589M | 5.305 | 5.305 | 0.000120 | 0.935 | - | - | - |
| involuntary context switches | 419.0 | 0.00140 | 0.00140 | 0.000518 | 0.000234 | - | - | - |
| cycles | - | - | - | 359.1k | 15.76k | - | - | - |
| instructions | - | - | - | 660.8k | 12.52k | - | - | - |
| cache references | - | - | - | 42.83 | 167.9 | - | - | - |
| cache misses | - | - | - | 0.238 | 2.620 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0252 | 0.0392 | - | - | - |
| data TLB read misses | - | - | - | 0.0105 | 0.655 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 12.39M | 40.69 | 40.69 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 1268 | 0.00416 | 0.00416 |
| garbage collections | - | - | - | - | - | 12.00 | 0.0000394 | 0.0000394 |
| collectors' pauses, ns | - | - | - | - | - | 21.34M | 70.05 | 70.05 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 15.48M | - | - | - | - | 1.268G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 14.70M | - | - | - | - | 1.269G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 186.2M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304611 messages, 304610 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-290066-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 2952790.0 | 6777995.3 | 11408506.9 | 12750684.2 | 13762277.4 | 0 | 0 | 100/100 | 166.8 | 35.4 | 105.4 | 9.43 / 6.73 |
| best | kafka | 22 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 2952790.0 | 6777995.3 | 11408506.9 | 12750684.2 | 13762277.4 | 0 | 0 | 100/100 | 166.8 | 35.4 | 105.4 | 9.43 / 6.73 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 30343949 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 133.8G | 446.8k | 4410 | 99.80k | 1032 | 83.29G | 273.4k | 2720 |
| waiting for a CPU, ns | - | - | - | 29.39 | 27.19 | - | - | - |
| allocations | 89.06M | 297.4 | 2.935 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 89.09M | 297.5 | 2.936 | 1.000 | 2.253 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 25.17G | 84.04k | 829.5 | 456.0 | 24.00 | 30.50G | 100.1k | 995.9 |
| minor page faults | 1.048M | 3.499 | 0.0345 | 0.0302 | 0 | 65.65k | 0.216 | 0.00214 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 6.899M | 23.03 | 0.227 | 0.000104 | 0.0872 | - | - | - |
| involuntary context switches | 17.62k | 0.0588 | 0.000581 | 0.00150 | 0.000103 | - | - | - |
| cycles | - | - | - | 491.1k | 4365 | - | - | - |
| instructions | - | - | - | 903.4k | 5496 | - | - | - |
| cache references | - | - | - | 70.69 | 46.46 | - | - | - |
| cache misses | - | - | - | 7.232 | 6.721 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0910 | 0.0523 | - | - | - |
| data TLB read misses | - | - | - | 3.445 | 1.394 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 13.87M | 45.53 | 0.453 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 1246 | 0.00409 | 0.0000407 |
| garbage collections | - | - | - | - | - | 48.00 | 0.000158 | 0.00000157 |
| collectors' pauses, ns | - | - | - | - | - | 85.56M | 280.9 | 2.794 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 404.1M | - | - | - | - | 1.450G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 349.7M | - | - | - | - | 1.448G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 227.0M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304612 messages, 30624343 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-296631-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 104394 | 2939 | 250196 | 0 | 104394000 | 54330000 | 245167000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 376.9 | 35.5 | 213.8 | 8.69 / 10.78 |
| best | kafka | 22 × wait | 104394 | 2939 | 250196 | 0 | 104394000 | 54330000 | 245167000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 376.9 | 35.5 | 213.8 | 8.69 / 10.78 |

Run 1's profile:

| profile: the window, 30.0 s, 54265 messages, 3675847 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 154.9G | 2.855M | 42.14k | 550.7k | 8398 | 100.1G | 1.762M | 26.80k |
| waiting for a CPU, ns | - | - | - | 152.1 | 131.6 | - | - | - |
| allocations | 115.4M | 2127 | 31.39 | 1369 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 115.4M | 2126 | 31.38 | 1368 | 4.751 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 10.76G | 198.4k | 2929 | 33.27k | 24.00 | 36.43G | 640.9k | 9749 |
| minor page faults | 5603 | 0.103 | 0.00152 | 0.0996 | 0 | 6701 | 0.118 | 0.00179 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 7.139M | 131.6 | 1.942 | 0.00276 | 0.891 | - | - | - |
| involuntary context switches | 18.62k | 0.343 | 0.00506 | 0.0184 | 0.000879 | - | - | - |
| cycles | - | - | - | 2.975M | 24.82k | - | - | - |
| instructions | - | - | - | 6.605M | 14.20k | - | - | - |
| cache references | - | - | - | 167.5 | 587.7 | - | - | - |
| cache misses | - | - | - | 65.49 | 155.6 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 2.548 | 14.49 | - | - | - |
| data TLB read misses | - | - | - | 1.601 | 39.69 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 7.614M | 134.0 | 2.038 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 490.0 | 0.00862 | 0.000131 |
| garbage collections | - | - | - | - | - | 57.00 | 0.00100 | 0.0000153 |
| collectors' pauses, ns | - | - | - | - | - | 116.5M | 2050 | 31.19 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 587.6M | - | - | - | - | 1.497G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 587.6M | - | - | - | - | 1.497G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 243.4M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 56838 messages, 3736385 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-304314-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × wait | 3547543 | 99928 | 0 | 0 | 2800819 | 2301448 | 696826 | 35433480.2 | undelivered | undelivered | undelivered | undelivered | 0 | 696826 | 0/1 | 59.0 | 35.4 | 38.0 | 4.18 / 4.37 |
| best | kafka | 1 × wait | 3547543 | 99928 | 0 | 0 | 2800819 | 2301448 | 696826 | 35433480.2 | undelivered | undelivered | undelivered | undelivered | 0 | 696826 | 0/1 | 59.0 | 35.4 | 38.0 | 4.18 / 4.37 |

Run 1's profile:

| profile: the window, 30.0 s, 2998292 messages, 932928 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 47.23G | 15.75k | 50.62k | 9966 | 618.0 | 29.05G | 9529 | 30.03k |
| waiting for a CPU, ns | - | - | - | 0.926 | 8.176 | - | - | - |
| allocations | 23.83M | 7.949 | 25.55 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 23.84M | 7.951 | 25.55 | 1.000 | 2.001 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 6.707G | 2237 | 7190 | 456.0 | 24.00 | 21.17G | 6944 | 21.88k |
| minor page faults | 113.8k | 0.0379 | 0.122 | 0.0270 | 0 | 11.29k | 0.00370 | 0.0117 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.363M | 1.122 | 3.605 | 0.000457 | 0.0164 | - | - | - |
| involuntary context switches | 619.0 | 0.000206 | 0.000664 | 0.0000567 | 0.0000515 | - | - | - |
| cycles | - | - | - | 55.25k | 3510 | - | - | - |
| instructions | - | - | - | 96.96k | 4741 | - | - | - |
| cache references | - | - | - | 41.23 | 35.09 | - | - | - |
| cache misses | - | - | - | 4.149 | 1.661 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0139 | 0.0170 | - | - | - |
| data TLB read misses | - | - | - | 0.0747 | 0.263 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 11.21M | 3.678 | 11.59 |
| bytes written to storage | 0 | 0 | 0 | - | - | 59.36M | 19.47 | 61.37 |
| storage reads | - | - | - | - | - | 1405 | 0.000461 | 0.00145 |
| storage writes | - | - | - | - | - | 7107 | 0.00233 | 0.00735 |
| garbage collections | - | - | - | - | - | 33.00 | 0.0000108 | 0.0000341 |
| collectors' pauses, ns | - | - | - | - | - | 62.18M | 20.40 | 64.28 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 31.82M | - | - | - | - | 2.088G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 28.18M | - | - | - | - | 2.058G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 326.4M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3048286 messages, 967348 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-337455-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 3547543 | 99928 | 0 | 0 | 81762483 | 31858950 | 267968450 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 267968450 | 0/100 | 147.6 | 35.3 | 47.9 | 11.10 / 6.02 |
| best | kafka | 22 × wait | 3547543 | 99928 | 0 | 0 | 81762483 | 31858950 | 267968450 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 267968450 | 0/100 | 147.6 | 35.3 | 47.9 | 11.10 / 6.02 |

Run 1's profile:

| profile: the window, 30.0 s, 2998266 messages, 31398698 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 74.27G | 24.77k | 2365 | 9964 | 508.4 | 30.26G | 9927 | 930.4 |
| waiting for a CPU, ns | - | - | - | 1.924 | 4.577 | - | - | - |
| allocations | 83.53M | 27.86 | 2.660 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 83.52M | 27.86 | 2.660 | 1.000 | 2.001 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 25.27G | 8427 | 804.7 | 456.0 | 24.00 | 19.57G | 6418 | 601.5 |
| minor page faults | 1.073M | 0.358 | 0.0342 | 0.0297 | 0 | 5179 | 0.00170 | 0.000159 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.561M | 1.188 | 0.113 | 0.000940 | 0.0112 | - | - | - |
| involuntary context switches | 1997 | 0.000666 | 0.0000636 | 0.000113 | 0.0000205 | - | - | - |
| cycles | - | - | - | 53.73k | 2940 | - | - | - |
| instructions | - | - | - | 92.78k | 4283 | - | - | - |
| cache references | - | - | - | 42.90 | 29.40 | - | - | - |
| cache misses | - | - | - | 3.361 | 5.146 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0121 | 0.0261 | - | - | - |
| data TLB read misses | - | - | - | 0.244 | 0.151 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 38.47M | 12.62 | 1.183 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 4594 | 0.00151 | 0.000141 |
| garbage collections | - | - | - | - | - | 35.00 | 0.0000115 | 0.00000108 |
| collectors' pauses, ns | - | - | - | - | - | 79.15M | 25.96 | 2.433 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 425.2M | - | - | - | - | 3.204G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 368.3M | - | - | - | - | 3.211G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 241.8M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3048290 messages, 32525161 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-369842-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 106798 | 2998 | 3440745 | 0 | 106798000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 316.5 | 35.5 | 144.3 | 11.20 / 11.99 |
| best | kafka | 22 × wait | 106798 | 2998 | 3440745 | 0 | 106798000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 316.5 | 35.5 | 144.3 | 11.20 / 11.99 |

Run 1's profile:

| profile: the window, 30.0 s, 5676 messages, 5673888 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 157.9G | 27.82M | 27.83k | 5.260M | 5826 | 91.75G | 15.91M | 15.91k |
| waiting for a CPU, ns | - | - | - | 2030 | 135.2 | - | - | - |
| allocations | 123.4M | 21.73k | 21.74 | 13.87k | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 123.4M | 21.74k | 21.74 | 13.87k | 3.759 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 11.95G | 2.105M | 2106 | 333.2k | 24.00 | 35.29G | 6.118M | 6119 |
| minor page faults | 392.0 | 0.0691 | 0.0000691 | 0 | 0.000000352 | 4287 | 0.743 | 0.000743 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 1.000 | 0.000173 | 0.000000173 |
| voluntary context switches | 7.015M | 1236 | 1.236 | 0.0393 | 0.571 | - | - | - |
| involuntary context switches | 34.58k | 6.092 | 0.00609 | 0.188 | 0.00120 | - | - | - |
| cycles | - | - | - | 27.71M | 17.48k | - | - | - |
| instructions | - | - | - | 63.56M | 10.68k | - | - | - |
| cache references | - | - | - | 1657 | 406.7 | - | - | - |
| cache misses | - | - | - | 670.8 | 102.9 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 29.67 | 9.768 | - | - | - |
| data TLB read misses | - | - | - | 14.27 | 25.64 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 4096 | 0.710 | 0.000710 |
| bytes written to storage | 0 | 0 | 0 | - | - | 6.308M | 1094 | 1.094 |
| storage reads | - | - | - | - | - | 1.000 | 0.000173 | 0.000000173 |
| storage writes | - | - | - | - | - | 424.0 | 0.0735 | 0.0000735 |
| garbage collections | - | - | - | - | - | 56.00 | 0.00971 | 0.00000971 |
| collectors' pauses, ns | - | - | - | - | - | 124.5M | 21.59k | 21.59 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 577.4M | - | - | - | - | 3.045G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 577.4M | - | - | - | - | 3.044G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 269.6M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 5768 messages, 5767077 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-429132-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 65.5 | 145.4 | 266.2 | 2392.1 | 16204.4 | 0 | 0 | 1/1 | 81.4 | 35.4 | 21.0 | 2.95 / 3.21 |
| best | kafka | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 65.5 | 145.4 | 266.2 | 2392.1 | 16204.4 | 0 | 0 | 1/1 | 81.4 | 35.4 | 21.0 | 2.95 / 3.21 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 299498 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 68.17G | 227.6k | 227.6k | 99.98k | 99.25k | 17.02G | 55.86k | 55.86k |
| waiting for a CPU, ns | - | - | - | 4.717 | 3.617 | - | - | - |
| allocations | 6.995M | 23.36 | 23.36 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.995M | 23.36 | 23.36 | 1.000 | 4.632 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 1.671G | 5581 | 5581 | 456.0 | 24.00 | 7.220G | 23.70k | 23.70k |
| minor page faults | 43.00 | 0.000144 | 0.000144 | 0 | 0 | 7361 | 0.0242 | 0.0242 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.364M | 4.555 | 4.555 | 0.000164 | 0.345 | - | - | - |
| involuntary context switches | 451.0 | 0.00151 | 0.00151 | 0.000447 | 0.000264 | - | - | - |
| cycles | - | - | - | 357.3k | 665.4k | - | - | - |
| instructions | - | - | - | 659.1k | 1.119M | - | - | - |
| cache references | - | - | - | 38.97 | 224.2 | - | - | - |
| cache misses | - | - | - | 0.207 | 6.262 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0197 | 0.0286 | - | - | - |
| data TLB read misses | - | - | - | 0.00164 | 2.413 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 10.84M | 35.59 | 35.59 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 1141 | 0.00375 | 0.00375 |
| garbage collections | - | - | - | - | - | 11.00 | 0.0000361 | 0.0000361 |
| collectors' pauses, ns | - | - | - | - | - | 29.33M | 96.30 | 96.30 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 9.716M | - | - | - | - | 3.098G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 9.716M | - | - | - | - | 3.098G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 278.2M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304612 messages, 304612 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-440842-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 1778384.9 | 3019898.9 | 3992977.4 | 4697620.5 | 5192619.8 | 0 | 0 | 100/100 | 865.0 | 34.2 | 91.1 | 24.86 / 34.32 |
| best | kafka | 22 × busy | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 1778384.9 | 3019898.9 | 3992977.4 | 4697620.5 | 5192619.8 | 0 | 0 | 100/100 | 865.0 | 34.2 | 91.1 | 24.86 / 34.32 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 32842216 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 679.1G | 2.268M | 20.68k | 96.42k | 17.85k | 74.71G | 245.3k | 2268 |
| waiting for a CPU, ns | - | - | - | 3319 | 1945 | - | - | - |
| allocations | 89.60M | 299.2 | 2.728 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 89.64M | 299.3 | 2.729 | 1.000 | 2.205 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 25.10G | 83.81k | 764.3 | 456.0 | 24.00 | 25.40G | 83.39k | 771.2 |
| minor page faults | 751.2k | 2.508 | 0.0229 | 0.0449 | 0 | 3028 | 0.00994 | 0.0000919 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.796M | 12.67 | 0.116 | 0.0000301 | 0.0278 | - | - | - |
| involuntary context switches | 1.181M | 3.944 | 0.0360 | 0.225 | 0.0287 | - | - | - |
| cycles | - | - | - | 412.9k | 90.08k | - | - | - |
| instructions | - | - | - | 674.4k | 131.5k | - | - | - |
| cache references | - | - | - | 29.57 | 36.35 | - | - | - |
| cache misses | - | - | - | 4.655 | 4.973 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.154 | 0.0380 | - | - | - |
| data TLB read misses | - | - | - | 0.752 | 0.753 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 9.552M | 31.36 | 0.290 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 1088 | 0.00357 | 0.0000330 |
| garbage collections | - | - | - | - | - | 40.00 | 0.000131 | 0.00000121 |
| collectors' pauses, ns | - | - | - | - | - | 96.89M | 318.1 | 2.942 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 398.3M | - | - | - | - | 3.225G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 331.3M | - | - | - | - | 3.218G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 329.3M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304612 messages, 32938394 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-445993-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 107926 | 3039 | 246664 | 0 | 107926000 | 57862000 | 241635000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 1242.1 | 35.0 | 125.0 | 26.21 / 95.62 |
| best | kafka | 22 × busy | 107926 | 3039 | 246664 | 0 | 107926000 | 57862000 | 241635000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 1242.1 | 35.0 | 125.0 | 26.21 / 95.62 |

Run 1's profile:

| profile: the window, 30.0 s, 57810 messages, 6964887 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 672.0G | 11.63M | 96.49k | 514.6k | 80.30k | 85.97G | 1.424M | 12.19k |
| waiting for a CPU, ns | - | - | - | 2502 | 13.22k | - | - | - |
| allocations | 91.49M | 1583 | 13.14 | 849.7 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 91.45M | 1582 | 13.13 | 848.7 | 3.219 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 11.05G | 191.1k | 1586 | 20.80k | 24.00 | 31.17G | 516.3k | 4420 |
| minor page faults | 6881 | 0.119 | 0.000988 | 0.0929 | 0 | 1220 | 0.0202 | 0.000173 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.903M | 67.52 | 0.560 | 0.00133 | 0.115 | - | - | - |
| involuntary context switches | 1.255M | 21.70 | 0.180 | 0.0492 | 0.152 | - | - | - |
| cycles | - | - | - | 1.010M | 399.3k | - | - | - |
| instructions | - | - | - | 2.282M | 578.3k | - | - | - |
| cache references | - | - | - | 42.89 | 196.7 | - | - | - |
| cache misses | - | - | - | 18.92 | 46.72 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0000173 | 3.846 | - | - | - |
| data TLB read misses | - | - | - | 0.135 | 11.52 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 7.385M | 122.3 | 1.047 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 497.0 | 0.00823 | 0.0000705 |
| garbage collections | - | - | - | - | - | 49.00 | 0.000812 | 0.00000695 |
| collectors' pauses, ns | - | - | - | - | - | 126.5M | 2095 | 17.93 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 584.3M | - | - | - | - | 3.220G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 584.3M | - | - | - | - | 3.218G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 354.2M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 60374 messages, 7052527 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-452865-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 69206.0 | 122683.4 | 140509.2 | 153092.1 | 168381.9 | 0 | 0 | 1/1 | 100.7 | 35.4 | 57.8 | 7.33 / 6.95 |
| best | kafka | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 69206.0 | 122683.4 | 140509.2 | 153092.1 | 168381.9 | 0 | 0 | 1/1 | 100.7 | 35.4 | 57.8 | 7.33 / 6.95 |

Run 1's profile:

| profile: the window, 30.0 s, 2998288 messages, 3004260 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 84.45G | 28.17k | 28.11k | 9970 | 9720 | 48.49G | 15.91k | 15.85k |
| waiting for a CPU, ns | - | - | - | 0.879 | 1.484 | - | - | - |
| allocations | 33.20M | 11.07 | 11.05 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 33.20M | 11.07 | 11.05 | 1.000 | 2.662 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 8.971G | 2992 | 2986 | 456.0 | 24.00 | 26.36G | 8649 | 8616 |
| minor page faults | 153.0k | 0.0510 | 0.0509 | 0.0329 | 0 | 1119 | 0.000367 | 0.000366 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 4.736M | 1.580 | 1.576 | 0.000426 | 0.117 | - | - | - |
| involuntary context switches | 1405 | 0.000469 | 0.000468 | 0.0000961 | 0.0000696 | - | - | - |
| cycles | - | - | - | 55.76k | 60.98k | - | - | - |
| instructions | - | - | - | 97.66k | 99.05k | - | - | - |
| cache references | - | - | - | 38.35 | 59.62 | - | - | - |
| cache misses | - | - | - | 2.373 | 0.978 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00598 | 0.00950 | - | - | - |
| data TLB read misses | - | - | - | 0.113 | 0.255 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 41.70M | 13.68 | 13.63 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 4945 | 0.00162 | 0.00162 |
| garbage collections | - | - | - | - | - | 48.00 | 0.0000157 | 0.0000157 |
| collectors' pauses, ns | - | - | - | - | - | 93.05M | 30.52 | 30.41 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 30.84M | - | - | - | - | 4.089G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 24.53M | - | - | - | - | 4.098G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 256.9M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3048281 messages, 3060109 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-463196-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 3547411 | 99924 | 132 | 0 | 123154656 | 72520832 | 227306568 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 227306568 | 14/100 | 1494.2 | 32.4 | 85.2 | 24.85 / 25.93 |
| best | kafka | 22 × busy | 3547411 | 99924 | 132 | 0 | 123154656 | 72520832 | 227306568 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 227306568 | 14/100 | 1494.2 | 32.4 | 85.2 | 24.85 / 25.93 |

Run 1's profile:

| profile: the window, 30.0 s, 2998200 messages, 83040497 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.1G | 229.5k | 8286 | 9154 | 7145 | 70.30G | 23.06k | 835.0 |
| waiting for a CPU, ns | - | - | - | 814.8 | 688.3 | - | - | - |
| allocations | 191.5M | 63.87 | 2.306 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 191.5M | 63.87 | 2.306 | 1.000 | 2.057 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 56.31G | 18.78k | 678.1 | 456.0 | 24.00 | 22.40G | 7350 | 266.1 |
| minor page faults | 814.9k | 0.272 | 0.00981 | 0.000309 | 0 | 2492 | 0.000818 | 0.0000296 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.386M | 1.129 | 0.0408 | 0.0000180 | 0.0114 | - | - | - |
| involuntary context switches | 1.332M | 0.444 | 0.0160 | 0.0406 | 0.0112 | - | - | - |
| cycles | - | - | - | 39.78k | 36.56k | - | - | - |
| instructions | - | - | - | 62.48k | 53.36k | - | - | - |
| cache references | - | - | - | 23.04 | 23.54 | - | - | - |
| cache misses | - | - | - | 1.441 | 1.366 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00871 | 0.0104 | - | - | - |
| data TLB read misses | - | - | - | 0.0647 | 0.146 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 22.00M | 7.218 | 0.261 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 2439 | 0.000800 | 0.0000290 |
| garbage collections | - | - | - | - | - | 36.00 | 0.0000118 | 0.000000428 |
| collectors' pauses, ns | - | - | - | - | - | 87.22M | 28.61 | 1.036 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 386.5M | - | - | - | - | 5.049G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 364.7M | - | - | - | - | 5.055G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 234.8M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3048289 messages, 84196164 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-475494-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 3547428 | 99925 | 115 | 0 | 638689992 | 140068777 | 2858205223 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2858205223 | 0/1000 | 1543.0 | 28.5 | 54.9 | 26.34 / 27.04 |
| best | kafka | 22 × busy | 3547428 | 99925 | 115 | 0 | 638689992 | 140068777 | 2858205223 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2858205223 | 0/1000 | 1543.0 | 28.5 | 54.9 | 26.34 / 27.04 |

Run 1's profile:

| profile: the window, 30.0 s, 2998263 messages, 306313122 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 705.4G | 235.3k | 2303 | 7899 | 1859 | 31.85G | 10.45k | 103.5 |
| waiting for a CPU, ns | - | - | - | 2063 | 278.4 | - | - | - |
| allocations | 625.1M | 208.5 | 2.041 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 625.2M | 208.5 | 2.041 | 1.000 | 2.001 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 193.2G | 64.45k | 630.8 | 456.0 | 24.00 | 9.367G | 3073 | 30.43 |
| minor page faults | 2.640M | 0.880 | 0.00862 | 0.000116 | 0 | 2662 | 0.000873 | 0.00000865 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.460M | 0.487 | 0.00477 | 0.000172 | 0.000723 | - | - | - |
| involuntary context switches | 868.7k | 0.290 | 0.00284 | 0.0737 | 0.000983 | - | - | - |
| cycles | - | - | - | 33.86k | 10.15k | - | - | - |
| instructions | - | - | - | 52.24k | 15.09k | - | - | - |
| cache references | - | - | - | 33.23 | 22.65 | - | - | - |
| cache misses | - | - | - | 6.751 | 8.860 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0769 | 0.0612 | - | - | - |
| data TLB read misses | - | - | - | 0.201 | 0.0503 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 39.46M | 12.94 | 0.128 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 4565 | 0.00150 | 0.0000148 |
| garbage collections | - | - | - | - | - | 15.00 | 0.00000492 | 0.000000049 |
| collectors' pauses, ns | - | - | - | - | - | 65.55M | 21.50 | 0.213 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 2.050G | - | - | - | - | 6.029G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 1.808G | - | - | - | - | 6.036G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 255.6M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3048360 messages, 307866715 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-492728-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 70.7 | 151.6 | 266.2 | 5177.3 | 17013.1 | 0 | 0 | 1/1 | 47.2 | 35.4 | 21.6 | 4.18 / 3.47 |
| best | kafka | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 70.7 | 151.6 | 266.2 | 5177.3 | 17013.1 | 0 | 0 | 1/1 | 47.2 | 35.4 | 21.6 | 4.18 / 3.47 |

Run 1's profile:

| profile: the window, 30.0 s, 299587 messages, 299587 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 39.93G | 133.3k | 133.3k | 99.94k | 7125 | 17.41G | 57.16k | 57.16k |
| waiting for a CPU, ns | - | - | - | 1.316 | 14.63 | - | - | - |
| allocations | 7.034M | 23.48 | 23.48 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 7.034M | 23.48 | 23.48 | 1.000 | 4.667 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 1.680G | 5607 | 5607 | 456.0 | 24.00 | 7.264G | 23.85k | 23.85k |
| minor page faults | 75.00 | 0.000250 | 0.000250 | 0 | 0 | 124.0 | 0.000407 | 0.000407 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.601M | 5.343 | 5.343 | 0.000157 | 0.948 | - | - | - |
| involuntary context switches | 292.0 | 0.000975 | 0.000975 | 0.000184 | 0.000204 | - | - | - |
| cycles | - | - | - | 355.6k | 15.85k | - | - | - |
| instructions | - | - | - | 653.8k | 12.70k | - | - | - |
| cache references | - | - | - | 44.67 | 167.3 | - | - | - |
| cache misses | - | - | - | 0.278 | 3.077 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0256 | 0.0467 | - | - | - |
| data TLB read misses | - | - | - | 0.00431 | 0.827 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 10.80M | 35.45 | 35.45 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 1135 | 0.00373 | 0.00373 |
| garbage collections | - | - | - | - | - | 11.00 | 0.0000361 | 0.0000361 |
| collectors' pauses, ns | - | - | - | - | - | 35.61M | 116.9 | 116.9 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 9.490M | - | - | - | - | 5.831G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 9.490M | - | - | - | - | 5.831G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 266.2M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304585 messages, 304582 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-502940-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 5234491.4 | 9797894.1 | 14495514.6 | 16642998.3 | 17484656.5 | 0 | 0 | 100/100 | 162.9 | 35.4 | 102.2 | 6.98 / 6.17 |
| best | kafka | 22 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 5234491.4 | 9797894.1 | 14495514.6 | 16642998.3 | 17484656.5 | 0 | 0 | 100/100 | 162.9 | 35.4 | 102.2 | 6.98 / 6.17 |

Run 1's profile:

| profile: the window, 30.0 s, 299584 messages, 28780457 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 127.7G | 426.2k | 4437 | 99.78k | 1045 | 81.87G | 268.8k | 2835 |
| waiting for a CPU, ns | - | - | - | 31.37 | 16.85 | - | - | - |
| allocations | 85.01M | 283.8 | 2.954 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 85.04M | 283.9 | 2.955 | 1.000 | 2.256 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 24.00G | 80.12k | 834.0 | 456.0 | 24.00 | 30.14G | 98.96k | 1044 |
| minor page faults | 296.9k | 0.991 | 0.0103 | 0.0237 | 0 | 13.64k | 0.0448 | 0.000472 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 6.812M | 22.74 | 0.237 | 0.0000567 | 0.0901 | - | - | - |
| involuntary context switches | 10.70k | 0.0357 | 0.000372 | 0.00246 | 0.0000693 | - | - | - |
| cycles | - | - | - | 492.6k | 4408 | - | - | - |
| instructions | - | - | - | 906.8k | 5471 | - | - | - |
| cache references | - | - | - | 61.75 | 45.72 | - | - | - |
| cache misses | - | - | - | 7.303 | 6.876 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0914 | 0.0540 | - | - | - |
| data TLB read misses | - | - | - | 0.686 | 0.887 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 13.07M | 42.91 | 0.453 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 1140 | 0.00374 | 0.0000395 |
| garbage collections | - | - | - | - | - | 64.00 | 0.000210 | 0.00000222 |
| collectors' pauses, ns | - | - | - | - | - | 111.1M | 364.8 | 3.848 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 483.8M | - | - | - | - | 5.945G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 479.1M | - | - | - | - | 5.946G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 193.4M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304586 messages, 28874679 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-510104-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 104350 | 2938 | 250448 | 0 | 104350000 | 54199000 | 245384000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 363.6 | 35.5 | 205.4 | 8.81 / 9.57 |
| best | kafka | 22 × wait | 104350 | 2938 | 250448 | 0 | 104350000 | 54199000 | 245384000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 363.6 | 35.5 | 205.4 | 8.81 / 9.57 |

Run 1's profile:

| profile: the window, 30.0 s, 54133 messages, 3670223 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 152.7G | 2.822M | 41.62k | 551.6k | 8505 | 98.60G | 1.742M | 26.42k |
| waiting for a CPU, ns | - | - | - | 281.3 | 156.5 | - | - | - |
| allocations | 108.5M | 2004 | 29.55 | 1258 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 108.4M | 2003 | 29.54 | 1257 | 4.695 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 10.46G | 193.3k | 2851 | 30.60k | 24.00 | 35.72G | 630.9k | 9570 |
| minor page faults | 5590 | 0.103 | 0.00152 | 0.0982 | 0 | 3065 | 0.0541 | 0.000821 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 7.042M | 130.1 | 1.919 | 0.00591 | 0.877 | - | - | - |
| involuntary context switches | 31.86k | 0.589 | 0.00868 | 0.0299 | 0.00166 | - | - | - |
| cycles | - | - | - | 2.934M | 25.03k | - | - | - |
| instructions | - | - | - | 6.519M | 14.09k | - | - | - |
| cache references | - | - | - | 167.3 | 606.1 | - | - | - |
| cache misses | - | - | - | 73.28 | 152.4 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 3.251 | 14.82 | - | - | - |
| data TLB read misses | - | - | - | 1.575 | 39.33 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 6.935M | 122.5 | 1.858 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 464.0 | 0.00820 | 0.000124 |
| garbage collections | - | - | - | - | - | 56.00 | 0.000989 | 0.0000150 |
| collectors' pauses, ns | - | - | - | - | - | 116.1M | 2051 | 31.11 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 579.3M | - | - | - | - | 5.916G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 579.3M | - | - | - | - | 5.912G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 210.4M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 56619 messages, 3732801 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-531580-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 13287555.1 | 19327352.8 | 21206401.0 | 21206401.0 | 21219308.0 | 0 | 0 | 1/1 | 56.9 | 35.4 | 36.2 | 2.58 / 2.08 |
| best | kafka | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 13287555.1 | 19327352.8 | 21206401.0 | 21206401.0 | 21219308.0 | 0 | 0 | 1/1 | 56.9 | 35.4 | 36.2 | 2.58 / 2.08 |

Run 1's profile:

| profile: the window, 30.0 s, 3000426 messages, 1164597 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 46.15G | 15.38k | 39.63k | 9975 | 561.1 | 29.02G | 9513 | 22.69k |
| waiting for a CPU, ns | - | - | - | 0.277 | 3.910 | - | - | - |
| allocations | 26.29M | 8.761 | 22.57 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 26.29M | 8.761 | 22.57 | 1.000 | 2.001 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 7.285G | 2428 | 6256 | 456.0 | 24.00 | 23.62G | 7742 | 18.47k |
| minor page faults | 90.59k | 0.0302 | 0.0778 | 0.0225 | 0 | 14.02k | 0.00459 | 0.0110 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.944M | 1.315 | 3.387 | 0.000190 | 0.0168 | - | - | - |
| involuntary context switches | 245.0 | 0.0000817 | 0.000210 | 0.0000347 | 0.00000773 | - | - | - |
| cycles | - | - | - | 59.16k | 3299 | - | - | - |
| instructions | - | - | - | 105.4k | 4064 | - | - | - |
| cache references | - | - | - | 43.98 | 35.99 | - | - | - |
| cache misses | - | - | - | 1.481 | 0.987 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00312 | 0.00584 | - | - | - |
| data TLB read misses | - | - | - | 0.0996 | 0.341 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 40.71M | 13.35 | 31.84 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 4542 | 0.00149 | 0.00355 |
| garbage collections | - | - | - | - | - | 37.00 | 0.0000121 | 0.0000289 |
| collectors' pauses, ns | - | - | - | - | - | 54.90M | 18.00 | 42.94 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 32.69M | - | - | - | - | 6.666G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 29.52M | - | - | - | - | 6.674G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 224.0M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3050399 messages, 1278636 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-551137-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 3550038 | 99998 | 0 | 0 | 81892157 | 31959237 | 268084463 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 268084463 | 0/100 | 146.5 | 35.4 | 48.3 | 9.84 / 6.08 |
| best | kafka | 22 × wait | 3550038 | 99998 | 0 | 0 | 81892157 | 31959237 | 268084463 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 268084463 | 0/100 | 146.5 | 35.4 | 48.3 | 9.84 / 6.08 |

Run 1's profile:

| profile: the window, 30.0 s, 3000452 messages, 27778257 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 69.94G | 23.31k | 2518 | 9965 | 531.6 | 29.88G | 9796 | 1005 |
| waiting for a CPU, ns | - | - | - | 0.632 | 6.319 | - | - | - |
| allocations | 77.57M | 25.85 | 2.792 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 77.57M | 25.85 | 2.792 | 1.000 | 2.001 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 23.30G | 7767 | 838.9 | 456.0 | 24.00 | 21.15G | 6932 | 711.0 |
| minor page faults | 885.7k | 0.295 | 0.0319 | 0.0264 | 0 | 435.0 | 0.000143 | 0.0000146 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.869M | 1.289 | 0.139 | 0.000867 | 0.0108 | - | - | - |
| involuntary context switches | 861.0 | 0.000287 | 0.0000310 | 0.0000540 | 0.0000123 | - | - | - |
| cycles | - | - | - | 56.26k | 2892 | - | - | - |
| instructions | - | - | - | 97.13k | 4354 | - | - | - |
| cache references | - | - | - | 41.23 | 30.42 | - | - | - |
| cache misses | - | - | - | 2.829 | 6.443 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00602 | 0.0319 | - | - | - |
| data TLB read misses | - | - | - | 0.347 | 0.208 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 40.66M | 13.33 | 1.367 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 4619 | 0.00151 | 0.000155 |
| garbage collections | - | - | - | - | - | 35.00 | 0.0000115 | 0.00000118 |
| collectors' pauses, ns | - | - | - | - | - | 55.31M | 18.13 | 1.860 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 432.2M | - | - | - | - | 7.711G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 397.9M | - | - | - | - | 7.720G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 243.1M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3050404 messages, 29740025 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-557113-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 107583 | 3016 | 3442455 | 0 | 107583000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 307.9 | 35.6 | 134.5 | 11.18 / 9.36 |
| best | kafka | 22 × wait | 107583 | 3016 | 3442455 | 0 | 107583000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 307.9 | 35.6 | 134.5 | 11.18 / 9.36 |

Run 1's profile:

| profile: the window, 30.0 s, 6036 messages, 6036371 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 159.2G | 26.38M | 26.38k | 4.949M | 5489 | 90.16G | 14.69M | 14.69k |
| waiting for a CPU, ns | - | - | - | 635.2 | 84.12 | - | - | - |
| allocations | 136.2M | 22.56k | 22.56 | 14.83k | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 136.2M | 22.56k | 22.56 | 14.83k | 3.719 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 12.68G | 2.102M | 2101 | 356.2k | 24.00 | 37.14G | 6.053M | 6053 |
| minor page faults | 135.0 | 0.0224 | 0.0000224 | 0 | 0 | 4777 | 0.779 | 0.000779 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 7.238M | 1199 | 1.199 | 0.0169 | 0.557 | - | - | - |
| involuntary context switches | 26.03k | 4.313 | 0.00431 | 0.0790 | 0.000901 | - | - | - |
| cycles | - | - | - | 26.90M | 16.80k | - | - | - |
| instructions | - | - | - | 61.97M | 10.60k | - | - | - |
| cache references | - | - | - | 1352 | 382.2 | - | - | - |
| cache misses | - | - | - | 519.3 | 95.27 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 21.20 | 8.917 | - | - | - |
| data TLB read misses | - | - | - | 9.477 | 24.26 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 7.455M | 1215 | 1.215 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 486.0 | 0.0792 | 0.0000792 |
| garbage collections | - | - | - | - | - | 58.00 | 0.00945 | 0.00000945 |
| collectors' pauses, ns | - | - | - | - | - | 114.7M | 18.69k | 18.69 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 576.4M | - | - | - | - | 7.756G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 576.4M | - | - | - | - | 7.748G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 263.2M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 6136 messages, 6135490 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-564293-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 65.5 | 143.4 | 260.1 | 2785.3 | 18645.6 | 0 | 0 | 1/1 | 81.3 | 35.4 | 21.0 | 3.36 / 3.24 |
| best | kafka | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 65.5 | 143.4 | 260.1 | 2785.3 | 18645.6 | 0 | 0 | 1/1 | 81.3 | 35.4 | 21.0 | 3.36 / 3.24 |

Run 1's profile:

| profile: the window, 30.0 s, 299586 messages, 299584 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 68.09G | 227.3k | 227.3k | 99.96k | 99.18k | 16.95G | 55.65k | 55.65k |
| waiting for a CPU, ns | - | - | - | 2.602 | 2.743 | - | - | - |
| allocations | 7.001M | 23.37 | 23.37 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 7.001M | 23.37 | 23.37 | 1.000 | 4.628 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 1.673G | 5584 | 5585 | 456.0 | 24.00 | 7.220G | 23.70k | 23.70k |
| minor page faults | 46.00 | 0.000154 | 0.000154 | 0 | 0 | 306.0 | 0.00100 | 0.00100 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.355M | 4.523 | 4.523 | 0.000120 | 0.368 | - | - | - |
| involuntary context switches | 355.0 | 0.00118 | 0.00118 | 0.000344 | 0.000204 | - | - | - |
| cycles | - | - | - | 358.6k | 657.7k | - | - | - |
| instructions | - | - | - | 663.0k | 1.104M | - | - | - |
| cache references | - | - | - | 39.43 | 274.9 | - | - | - |
| cache misses | - | - | - | 0.138 | 17.46 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0120 | 0.0169 | - | - | - |
| data TLB read misses | - | - | - | 0.00276 | 3.343 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 13.52M | 44.38 | 44.38 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 1312 | 0.00431 | 0.00431 |
| garbage collections | - | - | - | - | - | 15.00 | 0.0000492 | 0.0000492 |
| collectors' pauses, ns | - | - | - | - | - | 30.96M | 101.6 | 101.6 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 9.978M | - | - | - | - | 7.642G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 9.978M | - | - | - | - | 7.643G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 226.4M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304585 messages, 304585 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-574832-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 1526726.7 | 2818572.3 | 4093640.7 | 4966055.9 | 5338813.4 | 0 | 0 | 100/100 | 848.6 | 34.2 | 88.9 | 24.57 / 39.68 |
| best | kafka | 22 × busy | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 1526726.7 | 2818572.3 | 4093640.7 | 4966055.9 | 5338813.4 | 0 | 0 | 100/100 | 848.6 | 34.2 | 88.9 | 24.57 / 39.68 |

Run 1's profile:

| profile: the window, 30.0 s, 299585 messages, 33027085 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 679.2G | 2.267M | 20.57k | 96.32k | 17.89k | 69.26G | 227.4k | 2088 |
| waiting for a CPU, ns | - | - | - | 3450 | 1804 | - | - | - |
| allocations | 88.81M | 296.4 | 2.689 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 88.85M | 296.6 | 2.690 | 1.000 | 2.194 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 25.03G | 83.56k | 758.0 | 456.0 | 24.00 | 24.23G | 79.56k | 730.6 |
| minor page faults | 815.6k | 2.722 | 0.0247 | 0.0396 | 0 | 342.0 | 0.00112 | 0.0000103 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.612M | 12.06 | 0.109 | 0.0000300 | 0.0255 | - | - | - |
| involuntary context switches | 1.147M | 3.829 | 0.0347 | 0.258 | 0.0274 | - | - | - |
| cycles | - | - | - | 419.2k | 91.75k | - | - | - |
| instructions | - | - | - | 681.9k | 134.3k | - | - | - |
| cache references | - | - | - | 26.09 | 37.03 | - | - | - |
| cache misses | - | - | - | 3.137 | 4.596 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0516 | 0.0359 | - | - | - |
| data TLB read misses | - | - | - | 0.583 | 0.820 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 11.58M | 38.00 | 0.349 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 1121 | 0.00368 | 0.0000338 |
| garbage collections | - | - | - | - | - | 48.00 | 0.000158 | 0.00000145 |
| collectors' pauses, ns | - | - | - | - | - | 92.68M | 304.3 | 2.794 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 388.4M | - | - | - | - | 7.747G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 263.9M | - | - | - | - | 7.747G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 200.5M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304590 messages, 33170573 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-579716-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 354798 | 9994 | 0 | 0 | 333072425 | 280297994 | 19285006 | 30870077.4 | 35970351.1 | undelivered | undelivered | undelivered | 0 | 19285006 | 494/1000 | 1504.8 | 34.7 | 106.7 | 26.43 / 59.18 |
| best | kafka | 22 × busy | 354798 | 9994 | 0 | 0 | 333072425 | 280297994 | 19285006 | 30870077.4 | 35970351.1 | undelivered | undelivered | undelivered | 0 | 19285006 | 494/1000 | 1504.8 | 34.7 | 106.7 | 26.43 / 59.18 |

Run 1's profile:

| profile: the window, 30.0 s, 299591 messages, 45867444 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 671.0G | 2.240M | 14.63k | 98.05k | 12.05k | 74.62G | 245.0k | 1547 |
| waiting for a CPU, ns | - | - | - | 1591 | 1960 | - | - | - |
| allocations | 157.6M | 526.2 | 3.437 | 136.2 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 157.7M | 526.3 | 3.438 | 135.2 | 2.162 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 34.03G | 113.6k | 742.0 | 3677 | 24.00 | 27.29G | 89.61k | 565.7 |
| minor page faults | 749.2k | 2.501 | 0.0163 | 0.0171 | 0.000000022 | 1560 | 0.00512 | 0.0000323 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 4.108M | 13.71 | 0.0896 | 0.000504 | 0.0195 | - | - | - |
| involuntary context switches | 1.124M | 3.752 | 0.0245 | 0.0586 | 0.0204 | - | - | - |
| cycles | - | - | - | 461.6k | 61.06k | - | - | - |
| instructions | - | - | - | 819.5k | 88.60k | - | - | - |
| cache references | - | - | - | 209.0 | 44.66 | - | - | - |
| cache misses | - | - | - | 57.21 | 13.97 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.388 | 0.558 | - | - | - |
| data TLB read misses | - | - | - | 1.033 | 1.629 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 6.517M | 21.40 | 0.135 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 430.0 | 0.00141 | 0.00000891 |
| garbage collections | - | - | - | - | - | 47.00 | 0.000154 | 0.000000974 |
| collectors' pauses, ns | - | - | - | - | - | 109.6M | 359.7 | 2.271 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 2.921G | - | - | - | - | 8.097G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 2.071G | - | - | - | - | 7.620G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 238.9M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304566 messages, 48247302 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-586788-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × busy | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 2348810.2 | 11140071.4 | 13421772.8 | 14003613.8 | 14003613.8 | 0 | 0 | 1/1 | 111.3 | 35.4 | 42.0 | 8.42 / 6.53 |
| best | kafka | 1 × busy | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 2348810.2 | 11140071.4 | 13421772.8 | 14003613.8 | 14003613.8 | 0 | 0 | 1/1 | 111.3 | 35.4 | 42.0 | 8.42 / 6.53 |

Run 1's profile:

| profile: the window, 30.0 s, 3000439 messages, 2394076 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 81.13G | 27.04k | 33.89k | 9961 | 12.35k | 34.12G | 11.18k | 14.02k |
| waiting for a CPU, ns | - | - | - | 0.787 | 1.401 | - | - | - |
| allocations | 27.28M | 9.091 | 11.39 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 27.28M | 9.092 | 11.39 | 1.000 | 2.177 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 7.695G | 2565 | 3214 | 456.0 | 24.00 | 21.50G | 7048 | 8837 |
| minor page faults | 112.7k | 0.0376 | 0.0471 | 0.0268 | 0 | 689.0 | 0.000226 | 0.000283 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.357M | 1.119 | 1.402 | 0.00102 | 0.0631 | - | - | - |
| involuntary context switches | 721.0 | 0.000240 | 0.000301 | 0.0000680 | 0.0000915 | - | - | - |
| cycles | - | - | - | 54.06k | 75.54k | - | - | - |
| instructions | - | - | - | 95.38k | 127.2k | - | - | - |
| cache references | - | - | - | 37.57 | 50.61 | - | - | - |
| cache misses | - | - | - | 3.281 | 1.424 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0156 | 0.0170 | - | - | - |
| data TLB read misses | - | - | - | 0.109 | 0.108 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 47.88M | 15.70 | 19.68 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 5555 | 0.00182 | 0.00228 |
| garbage collections | - | - | - | - | - | 48.00 | 0.0000157 | 0.0000197 |
| collectors' pauses, ns | - | - | - | - | - | 92.59M | 30.35 | 38.06 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 32.19M | - | - | - | - | 7.190G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 26.10M | - | - | - | - | 7.197G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 199.0M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3050389 messages, 2432711 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-603621-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 3550038 | 99998 | 0 | 0 | 104819848 | 54491095 | 245552605 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 245552605 | 6/100 | 1503.1 | 32.3 | 68.6 | 23.89 / 26.22 |
| best | kafka | 22 × busy | 3550038 | 99998 | 0 | 0 | 104819848 | 54491095 | 245552605 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 245552605 | 6/100 | 1503.1 | 32.3 | 68.6 | 23.89 / 26.22 |

Run 1's profile:

| profile: the window, 30.0 s, 3000423 messages, 61810904 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 694.5G | 231.5k | 11.24k | 9096 | 9975 | 55.86G | 18.31k | 897.1 |
| waiting for a CPU, ns | - | - | - | 866.6 | 589.4 | - | - | - |
| allocations | 145.1M | 48.37 | 2.348 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 145.1M | 48.37 | 2.348 | 1.000 | 2.056 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 42.86G | 14.28k | 693.4 | 456.0 | 24.00 | 18.44G | 6044 | 296.0 |
| minor page faults | 692.3k | 0.231 | 0.0112 | 0.000686 | 0 | 290.0 | 0.0000951 | 0.00000466 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 2.725M | 0.908 | 0.0441 | 0.0000153 | 0.0108 | - | - | - |
| involuntary context switches | 1.190M | 0.397 | 0.0193 | 0.0434 | 0.0121 | - | - | - |
| cycles | - | - | - | 40.10k | 51.63k | - | - | - |
| instructions | - | - | - | 62.87k | 75.81k | - | - | - |
| cache references | - | - | - | 24.68 | 24.77 | - | - | - |
| cache misses | - | - | - | 1.551 | 1.620 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0102 | 0.0109 | - | - | - |
| data TLB read misses | - | - | - | 0.0832 | 0.106 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 39.59M | 12.98 | 0.636 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 4502 | 0.00148 | 0.0000723 |
| garbage collections | - | - | - | - | - | 39.00 | 0.0000128 | 0.000000626 |
| collectors' pauses, ns | - | - | - | - | - | 89.41M | 29.31 | 1.436 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 412.3M | - | - | - | - | 7.949G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 407.8M | - | - | - | - | 7.956G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 211.6M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3050403 messages, 62272489 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-609355-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 3549976 | 99996 | 62 | 0 | 1293247055 | 793594862 | 2206842138 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2206842138 | 0/1000 | 1525.7 | 23.6 | 90.7 | 27.34 / 39.29 |
| best | kafka | 22 × busy | 3549976 | 99996 | 62 | 0 | 1293247055 | 793594862 | 2206842138 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2206842138 | 0/1000 | 1525.7 | 23.6 | 90.7 | 27.34 / 39.29 |

Run 1's profile:

| profile: the window, 30.0 s, 3000462 messages, 582962037 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 693.8G | 231.2k | 1190 | 6457 | 801.0 | 47.82G | 15.68k | 81.27 |
| waiting for a CPU, ns | - | - | - | 3486 | 307.4 | - | - | - |
| allocations | 1.177G | 392.1 | 2.018 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 1.176G | 392.1 | 2.018 | 1.000 | 2.001 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 360.4G | 120.1k | 618.1 | 456.0 | 24.00 | 6.653G | 2181 | 11.31 |
| minor page faults | 4.219M | 1.406 | 0.00724 | 0.000258 | 0.000000007 | 876.0 | 0.000287 | 0.00000149 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.444M | 0.481 | 0.00248 | 0.000387 | 0.000428 | - | - | - |
| involuntary context switches | 686.6k | 0.229 | 0.00118 | 0.0589 | 0.000558 | - | - | - |
| cycles | - | - | - | 26.03k | 4470 | - | - | - |
| instructions | - | - | - | 39.35k | 6748 | - | - | - |
| cache references | - | - | - | 31.94 | 19.17 | - | - | - |
| cache misses | - | - | - | 11.79 | 11.58 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.225 | 0.126 | - | - | - |
| data TLB read misses | - | - | - | 0.382 | 0.0434 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 46.60M | 15.28 | 0.0792 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 5584 | 0.00183 | 0.00000949 |
| garbage collections | - | - | - | - | - | 14.00 | 0.00000459 | 0.000000024 |
| collectors' pauses, ns | - | - | - | - | - | 86.71M | 28.43 | 0.147 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 2.025G | - | - | - | - | 8.011G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 1.959G | - | - | - | - | 8.018G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 209.6M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3050417 messages, 588354119 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-617658-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 70.7 | 153.6 | 274.4 | 2687.0 | 11513.1 | 0 | 0 | 1/1 | 47.8 | 35.4 | 21.4 | 8.43 / 5.14 |
| best | kafka | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 70.7 | 153.6 | 274.4 | 2687.0 | 11513.1 | 0 | 0 | 1/1 | 47.8 | 35.4 | 21.4 | 8.43 / 5.14 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 300768 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 40.25G | 133.8k | 133.8k | 99.54k | 6886 | 17.45G | 57.05k | 57.05k |
| waiting for a CPU, ns | - | - | - | 5.722 | 6.262 | - | - | - |
| allocations | 7.038M | 23.40 | 23.40 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 7.038M | 23.40 | 23.40 | 1.000 | 4.650 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 1.681G | 5590 | 5590 | 456.0 | 24.00 | 7.262G | 23.75k | 23.75k |
| minor page faults | 36.00 | 0.000120 | 0.000120 | 0 | 0 | 470.0 | 0.00154 | 0.00154 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.573M | 5.229 | 5.229 | 0.000206 | 0.942 | - | - | - |
| involuntary context switches | 377.0 | 0.00125 | 0.00125 | 0.000485 | 0.000156 | - | - | - |
| cycles | - | - | - | 578.4k | 15.43k | - | - | - |
| instructions | - | - | - | 1.069M | 12.74k | - | - | - |
| cache references | - | - | - | 66.17 | 169.2 | - | - | - |
| cache misses | - | - | - | 1.205 | 2.826 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0173 | 0.0394 | - | - | - |
| data TLB read misses | - | - | - | 0.0973 | 1.077 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 10.99M | 35.93 | 35.93 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 1146 | 0.00375 | 0.00375 |
| garbage collections | - | - | - | - | - | 16.00 | 0.0000523 | 0.0000523 |
| collectors' pauses, ns | - | - | - | - | - | 38.38M | 125.5 | 125.5 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 9.732M | - | - | - | - | 8.133G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 9.732M | - | - | - | - | 8.133G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 204.2M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305833 messages, 305831 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-629187-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 5771362.3 | 10603200.5 | 14629732.4 | 16106127.4 | 17027049.5 | 0 | 0 | 100/100 | 166.0 | 35.4 | 101.1 | 7.05 / 5.67 |
| best | kafka | 22 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 5771362.3 | 10603200.5 | 14629732.4 | 16106127.4 | 17027049.5 | 0 | 0 | 100/100 | 166.0 | 35.4 | 101.1 | 7.05 / 5.67 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 28709858 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 128.7G | 427.8k | 4482 | 99.41k | 1056 | 80.21G | 262.3k | 2736 |
| waiting for a CPU, ns | - | - | - | 20.14 | 24.46 | - | - | - |
| allocations | 84.59M | 281.3 | 2.946 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 84.64M | 281.4 | 2.948 | 1.000 | 2.257 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 23.88G | 79.40k | 831.8 | 456.0 | 24.00 | 30.00G | 98.11k | 1023 |
| minor page faults | 394.4k | 1.311 | 0.0137 | 0.0364 | 0 | 2939 | 0.00961 | 0.000100 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 6.606M | 21.96 | 0.230 | 0.0000698 | 0.0898 | - | - | - |
| involuntary context switches | 12.87k | 0.0428 | 0.000448 | 0.00150 | 0.0000812 | - | - | - |
| cycles | - | - | - | 489.8k | 4460 | - | - | - |
| instructions | - | - | - | 895.2k | 5519 | - | - | - |
| cache references | - | - | - | 59.69 | 46.51 | - | - | - |
| cache misses | - | - | - | 6.400 | 7.599 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0923 | 0.0639 | - | - | - |
| data TLB read misses | - | - | - | 0.760 | 1.075 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 13.10M | 42.83 | 0.447 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 1122 | 0.00367 | 0.0000383 |
| garbage collections | - | - | - | - | - | 65.00 | 0.000213 | 0.00000222 |
| collectors' pauses, ns | - | - | - | - | - | 106.5M | 348.2 | 3.632 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 461.0M | - | - | - | - | 8.251G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 435.4M | - | - | - | - | 8.251G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 196.4M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305833 messages, 29318501 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-634162-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 104295 | 2936 | 251806 | 0 | 104295000 | 53891000 | 246878000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 372.1 | 35.5 | 196.7 | 8.77 / 9.16 |
| best | kafka | 22 × wait | 104295 | 2936 | 251806 | 0 | 104295000 | 53891000 | 246878000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 372.1 | 35.5 | 196.7 | 8.77 / 9.16 |

Run 1's profile:

| profile: the window, 30.0 s, 53829 messages, 3611670 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 150.2G | 2.791M | 41.59k | 554.8k | 8430 | 91.54G | 1.623M | 24.92k |
| waiting for a CPU, ns | - | - | - | 174.4 | 173.5 | - | - | - |
| allocations | 105.5M | 1960 | 29.21 | 1253 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 105.5M | 1959 | 29.20 | 1252 | 4.561 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 9.991G | 185.6k | 2766 | 30.47k | 24.00 | 33.15G | 587.7k | 9022 |
| minor page faults | 5606 | 0.104 | 0.00155 | 0.0997 | 0 | 103.0 | 0.00183 | 0.0000280 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 6.552M | 121.7 | 1.814 | 0.00286 | 0.835 | - | - | - |
| involuntary context switches | 23.59k | 0.438 | 0.00653 | 0.0122 | 0.00121 | - | - | - |
| cycles | - | - | - | 2.879M | 24.10k | - | - | - |
| instructions | - | - | - | 6.392M | 13.72k | - | - | - |
| cache references | - | - | - | 152.1 | 565.7 | - | - | - |
| cache misses | - | - | - | 71.15 | 156.1 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 2.745 | 15.26 | - | - | - |
| data TLB read misses | - | - | - | 0.904 | 36.65 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 7.459M | 132.2 | 2.030 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 483.0 | 0.00856 | 0.000131 |
| garbage collections | - | - | - | - | - | 66.00 | 0.00117 | 0.0000180 |
| collectors' pauses, ns | - | - | - | - | - | 153.7M | 2725 | 41.83 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 578.8M | - | - | - | - | 8.274G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 578.8M | - | - | - | - | 8.272G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 194.1M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 56403 messages, 3673853 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-642515-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × wait | 3554018 | 100110 | 0 | 0 | 2852838 | 2351542 | 651089 | 32212254.7 | undelivered | undelivered | undelivered | undelivered | 0 | 651089 | 0/1 | 56.9 | 35.4 | 35.7 | 3.88 / 3.35 |
| best | kafka | 1 × wait | 3554018 | 100110 | 0 | 0 | 2852838 | 2351542 | 651089 | 32212254.7 | undelivered | undelivered | undelivered | undelivered | 0 | 651089 | 0/1 | 56.9 | 35.4 | 35.7 | 3.88 / 3.35 |

Run 1's profile:

| profile: the window, 30.0 s, 3002642 messages, 1015415 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 46.46G | 15.47k | 45.75k | 9963 | 608.0 | 29.12G | 9539 | 28.20k |
| waiting for a CPU, ns | - | - | - | 0.421 | 5.480 | - | - | - |
| allocations | 24.96M | 8.313 | 24.58 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 24.95M | 8.310 | 24.57 | 1.000 | 2.001 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 6.970G | 2321 | 6864 | 456.0 | 24.00 | 22.32G | 7313 | 21.62k |
| minor page faults | 105.4k | 0.0351 | 0.104 | 0.0249 | 0 | 406.0 | 0.000133 | 0.000393 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.653M | 1.217 | 3.597 | 0.000376 | 0.0164 | - | - | - |
| involuntary context switches | 321.0 | 0.000107 | 0.000316 | 0.0000393 | 0.0000236 | - | - | - |
| cycles | - | - | - | 58.81k | 3442 | - | - | - |
| instructions | - | - | - | 105.0k | 4130 | - | - | - |
| cache references | - | - | - | 42.72 | 40.37 | - | - | - |
| cache misses | - | - | - | 1.926 | 1.459 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00749 | 0.0203 | - | - | - |
| data TLB read misses | - | - | - | 0.162 | 0.324 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 1.573M | 0.515 | 1.523 |
| bytes written to storage | 0 | 0 | 0 | - | - | 77.11M | 25.26 | 74.68 |
| storage reads | - | - | - | - | - | 96.00 | 0.0000314 | 0.0000930 |
| storage writes | - | - | - | - | - | 6780 | 0.00222 | 0.00657 |
| garbage collections | - | - | - | - | - | 36.00 | 0.0000118 | 0.0000349 |
| collectors' pauses, ns | - | - | - | - | - | 58.70M | 19.23 | 56.85 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 31.40M | - | - | - | - | 8.590G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 26.01M | - | - | - | - | 8.588G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 208.1M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3052584 messages, 1032616 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-657585-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 3554018 | 100110 | 0 | 0 | 110220280 | 60094538 | 240168562 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 240168562 | 0/100 | 169.3 | 35.4 | 49.0 | 9.65 / 5.26 |
| best | kafka | 22 × wait | 3554018 | 100110 | 0 | 0 | 110220280 | 60094538 | 240168562 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 240168562 | 0/100 | 169.3 | 35.4 | 49.0 | 9.65 / 5.26 |

Run 1's profile:

| profile: the window, 30.0 s, 3002643 messages, 38562576 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 81.03G | 26.99k | 2101 | 9955 | 518.3 | 30.99G | 10.15k | 717.3 |
| waiting for a CPU, ns | - | - | - | 1.062 | 3.914 | - | - | - |
| allocations | 98.92M | 32.94 | 2.565 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 98.91M | 32.94 | 2.565 | 1.000 | 2.001 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 29.99G | 9986 | 777.6 | 456.0 | 24.00 | 20.85G | 6832 | 482.7 |
| minor page faults | 1.060M | 0.353 | 0.0275 | 0.0267 | 0 | 429.0 | 0.000141 | 0.00000993 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 4.062M | 1.353 | 0.105 | 0.000836 | 0.0118 | - | - | - |
| involuntary context switches | 1314 | 0.000438 | 0.0000341 | 0.0000676 | 0.0000125 | - | - | - |
| cycles | - | - | - | 56.02k | 3073 | - | - | - |
| instructions | - | - | - | 97.79k | 4347 | - | - | - |
| cache references | - | - | - | 39.27 | 30.54 | - | - | - |
| cache misses | - | - | - | 2.657 | 1.912 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00549 | 0.0113 | - | - | - |
| data TLB read misses | - | - | - | 0.271 | 0.156 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 30.64M | 10.04 | 0.709 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 3478 | 0.00114 | 0.0000805 |
| garbage collections | - | - | - | - | - | 45.00 | 0.0000147 | 0.00000104 |
| collectors' pauses, ns | - | - | - | - | - | 68.82M | 22.54 | 1.593 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 409.3M | - | - | - | - | 8.590G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 391.6M | - | - | - | - | 8.589G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 191.0M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3052582 messages, 43204795 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-670100-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 106968 | 3001 | 3447050 | 0 | 106968000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 314.2 | 35.6 | 146.8 | 9.25 / 9.25 |
| best | kafka | 22 × wait | 106968 | 3001 | 3447050 | 0 | 106968000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 314.2 | 35.6 | 146.8 | 9.25 / 9.25 |

Run 1's profile:

| profile: the window, 30.0 s, 5678 messages, 5678710 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 157.6G | 27.76M | 27.76k | 5.266M | 5722 | 94.05G | 16.28M | 16.28k |
| waiting for a CPU, ns | - | - | - | 1031 | 87.32 | - | - | - |
| allocations | 135.9M | 23.93k | 23.93 | 15.92k | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 135.9M | 23.93k | 23.93 | 15.92k | 3.801 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 12.39G | 2.182M | 2182 | 382.5k | 24.00 | 36.88G | 6.383M | 6381 |
| minor page faults | 335.0 | 0.0590 | 0.0000590 | 0 | 0 | 150.0 | 0.0260 | 0.0000260 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 7.186M | 1266 | 1.265 | 0.0259 | 0.585 | - | - | - |
| involuntary context switches | 24.44k | 4.305 | 0.00430 | 0.119 | 0.000872 | - | - | - |
| cycles | - | - | - | 29.01M | 17.64k | - | - | - |
| instructions | - | - | - | 66.62M | 10.94k | - | - | - |
| cache references | - | - | - | 1480 | 400.0 | - | - | - |
| cache misses | - | - | - | 582.0 | 99.70 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 25.87 | 9.349 | - | - | - |
| data TLB read misses | - | - | - | 13.84 | 25.91 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 6.164M | 1067 | 1.067 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 402.0 | 0.0696 | 0.0000696 |
| garbage collections | - | - | - | - | - | 74.00 | 0.0128 | 0.0000128 |
| collectors' pauses, ns | - | - | - | - | - | 132.4M | 22.91k | 22.91 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 577.5M | - | - | - | - | 8.525G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 577.5M | - | - | - | - | 8.523G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 193.6M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 5777 messages, 5778915 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-681124-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 63.5 | 143.4 | 256.0 | 2752.5 | 14376.7 | 0 | 0 | 1/1 | 80.8 | 35.4 | 20.6 | 4.79 / 4.14 |
| best | kafka | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 63.5 | 143.4 | 256.0 | 2752.5 | 14376.7 | 0 | 0 | 1/1 | 80.8 | 35.4 | 20.6 | 4.79 / 4.14 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 300766 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 67.66G | 224.9k | 224.9k | 99.56k | 98.91k | 16.93G | 55.35k | 55.35k |
| waiting for a CPU, ns | - | - | - | 4.148 | 4.391 | - | - | - |
| allocations | 7.049M | 23.44 | 23.44 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 7.049M | 23.44 | 23.44 | 1.000 | 4.635 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 1.684G | 5599 | 5599 | 456.0 | 24.00 | 7.262G | 23.75k | 23.75k |
| minor page faults | 44.00 | 0.000146 | 0.000146 | 0 | 0 | 206.0 | 0.000674 | 0.000674 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.373M | 4.564 | 4.564 | 0.000110 | 0.300 | - | - | - |
| involuntary context switches | 450.0 | 0.00150 | 0.00150 | 0.000545 | 0.000436 | - | - | - |
| cycles | - | - | - | 581.2k | 663.1k | - | - | - |
| instructions | - | - | - | 1.063M | 1.114M | - | - | - |
| cache references | - | - | - | 107.8 | 222.7 | - | - | - |
| cache misses | - | - | - | 3.491 | 6.808 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0118 | 0.0181 | - | - | - |
| data TLB read misses | - | - | - | 1.469 | 2.357 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 12.01M | 39.28 | 39.28 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 1232 | 0.00403 | 0.00403 |
| garbage collections | - | - | - | - | - | 11.00 | 0.0000360 | 0.0000360 |
| collectors' pauses, ns | - | - | - | - | - | 29.35M | 95.97 | 95.97 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 9.568M | - | - | - | - | 8.207G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 9.568M | - | - | - | - | 8.207G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 204.7M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305836 messages, 305835 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-693543-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 1627390.0 | 2785017.9 | 3657433.1 | 4429185.0 | 5915345.2 | 0 | 0 | 100/100 | 841.7 | 34.1 | 92.1 | 22.26 / 33.61 |
| best | kafka | 22 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 1627390.0 | 2785017.9 | 3657433.1 | 4429185.0 | 5915345.2 | 0 | 0 | 100/100 | 841.7 | 34.1 | 92.1 | 22.26 / 33.61 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 33581917 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 677.3G | 2.252M | 20.17k | 95.72k | 17.49k | 71.48G | 233.7k | 2112 |
| waiting for a CPU, ns | - | - | - | 3449 | 1872 | - | - | - |
| allocations | 89.84M | 298.7 | 2.675 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 89.84M | 298.7 | 2.675 | 1.000 | 2.189 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 25.16G | 83.65k | 749.2 | 456.0 | 24.00 | 24.01G | 78.50k | 709.5 |
| minor page faults | 822.4k | 2.734 | 0.0245 | 0.0168 | 0 | 3325 | 0.0109 | 0.0000983 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.629M | 12.07 | 0.108 | 0.000366 | 0.0259 | - | - | - |
| involuntary context switches | 1.153M | 3.835 | 0.0343 | 0.225 | 0.0274 | - | - | - |
| cycles | - | - | - | 406.1k | 88.29k | - | - | - |
| instructions | - | - | - | 662.6k | 129.1k | - | - | - |
| cache references | - | - | - | 37.99 | 35.87 | - | - | - |
| cache misses | - | - | - | 5.896 | 5.401 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.119 | 0.0444 | - | - | - |
| data TLB read misses | - | - | - | 0.925 | 0.751 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 12.06M | 39.44 | 0.356 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 1149 | 0.00376 | 0.0000340 |
| garbage collections | - | - | - | - | - | 42.00 | 0.000137 | 0.00000124 |
| collectors' pauses, ns | - | - | - | - | - | 83.77M | 273.9 | 2.475 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 414.6M | - | - | - | - | 8.319G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 269.2M | - | - | - | - | 8.318G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 200.9M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305840 messages, 33840227 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-698439-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 108541 | 3056 | 247560 | 0 | 108541000 | 58137000 | 242632000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 1171.0 | 35.1 | 109.8 | 26.56 / 94.85 |
| best | kafka | 22 × busy | 108541 | 3056 | 247560 | 0 | 108541000 | 58137000 | 242632000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 1171.0 | 35.1 | 109.8 | 26.56 / 94.85 |

Run 1's profile:

| profile: the window, 30.0 s, 58054 messages, 7578820 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 673.2G | 11.60M | 88.83k | 512.0k | 74.31k | 82.81G | 1.365M | 10.81k |
| waiting for a CPU, ns | - | - | - | 2613 | 11.68k | - | - | - |
| allocations | 89.98M | 1550 | 11.87 | 818.7 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 89.93M | 1549 | 11.87 | 817.7 | 3.075 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 11.15G | 192.0k | 1471 | 20.06k | 24.00 | 29.35G | 483.8k | 3831 |
| minor page faults | 8104 | 0.140 | 0.00107 | 0.0939 | 0 | 2863 | 0.0472 | 0.000374 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.691M | 63.58 | 0.487 | 0.00115 | 0.0918 | - | - | - |
| involuntary context switches | 1.265M | 21.80 | 0.167 | 0.0646 | 0.142 | - | - | - |
| cycles | - | - | - | 2.466M | 360.6k | - | - | - |
| instructions | - | - | - | 4.365M | 521.5k | - | - | - |
| cache references | - | - | - | 1127 | 185.0 | - | - | - |
| cache misses | - | - | - | 539.0 | 44.32 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 12.85 | 3.907 | - | - | - |
| data TLB read misses | - | - | - | 16.47 | 10.41 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 7.594M | 125.2 | 0.991 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 508.0 | 0.00838 | 0.0000663 |
| garbage collections | - | - | - | - | - | 46.00 | 0.000758 | 0.00000601 |
| collectors' pauses, ns | - | - | - | - | - | 132.6M | 2186 | 17.31 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 586.5M | - | - | - | - | 8.343G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 586.5M | - | - | - | - | 8.341G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 214.3M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 60652 messages, 7659964 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-704293-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × busy | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 101711.9 | 897581.1 | 1711276.0 | 1962934.3 | 1988311.8 | 0 | 0 | 1/1 | 99.1 | 35.4 | 51.0 | 11.83 / 9.23 |
| best | kafka | 1 × busy | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 101711.9 | 897581.1 | 1711276.0 | 1962934.3 | 1988311.8 | 0 | 0 | 1/1 | 99.1 | 35.4 | 51.0 | 11.83 / 9.23 |

Run 1's profile:

| profile: the window, 30.0 s, 3002641 messages, 2990078 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 82.54G | 27.49k | 27.60k | 9948 | 9801 | 41.77G | 13.68k | 13.71k |
| waiting for a CPU, ns | - | - | - | 0.644 | 1.284 | - | - | - |
| allocations | 30.65M | 10.21 | 10.25 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 30.64M | 10.20 | 10.25 | 1.000 | 2.402 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 8.493G | 2829 | 2840 | 456.0 | 24.00 | 23.76G | 7785 | 7800 |
| minor page faults | 151.5k | 0.0505 | 0.0507 | 0.0330 | 0 | 421.0 | 0.000138 | 0.000138 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 4.099M | 1.365 | 1.371 | 0.000314 | 0.0945 | - | - | - |
| involuntary context switches | 821.0 | 0.000273 | 0.000275 | 0.0000679 | 0.0000625 | - | - | - |
| cycles | - | - | - | 54.04k | 60.20k | - | - | - |
| instructions | - | - | - | 95.61k | 98.96k | - | - | - |
| cache references | - | - | - | 38.86 | 53.43 | - | - | - |
| cache misses | - | - | - | 2.129 | 0.835 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00902 | 0.0117 | - | - | - |
| data TLB read misses | - | - | - | 0.109 | 0.151 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 40.90M | 13.40 | 13.42 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 4610 | 0.00151 | 0.00151 |
| garbage collections | - | - | - | - | - | 63.00 | 0.0000206 | 0.0000207 |
| collectors' pauses, ns | - | - | - | - | - | 88.40M | 28.96 | 29.01 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 32.00M | - | - | - | - | 8.058G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 31.30M | - | - | - | - | 8.067G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 213.5M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3052584 messages, 3046861 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-716616-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 3554001 | 100110 | 17 | 0 | 94769944 | 44540196 | 255722904 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 255722904 | 2/100 | 1509.9 | 31.2 | 37.0 | 21.62 / 25.05 |
| best | kafka | 22 × busy | 3554001 | 100110 | 17 | 0 | 94769944 | 44540196 | 255722904 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 255722904 | 2/100 | 1509.9 | 31.2 | 37.0 | 21.62 / 25.05 |

Run 1's profile:

| profile: the window, 30.0 s, 3002642 messages, 53652413 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 701.5G | 233.6k | 13.08k | 8785 | 11.97k | 27.61G | 9046 | 509.8 |
| waiting for a CPU, ns | - | - | - | 1171 | 240.8 | - | - | - |
| allocations | 122.5M | 40.80 | 2.284 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 122.5M | 40.80 | 2.283 | 1.000 | 2.030 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 36.94G | 12.30k | 688.5 | 456.0 | 24.00 | 11.65G | 3818 | 215.2 |
| minor page faults | 451.5k | 0.150 | 0.00841 | 0.00136 | 0 | 224.0 | 0.0000734 | 0.00000414 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.665M | 0.554 | 0.0310 | 0.0000140 | 0.00666 | - | - | - |
| involuntary context switches | 634.7k | 0.211 | 0.0118 | 0.0571 | 0.00403 | - | - | - |
| cycles | - | - | - | 38.98k | 63.77k | - | - | - |
| instructions | - | - | - | 61.19k | 94.06k | - | - | - |
| cache references | - | - | - | 22.11 | 23.75 | - | - | - |
| cache misses | - | - | - | 1.536 | 1.701 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0111 | 0.00969 | - | - | - |
| data TLB read misses | - | - | - | 0.0687 | 0.0594 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 38.99M | 12.77 | 0.720 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 4500 | 0.00147 | 0.0000831 |
| garbage collections | - | - | - | - | - | 24.00 | 0.00000786 | 0.000000443 |
| collectors' pauses, ns | - | - | - | - | - | 56.99M | 18.67 | 1.052 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 446.7M | - | - | - | - | 8.590G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 434.0M | - | - | - | - | 8.590G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 195.8M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3052582 messages, 54163437 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-721182-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: c69920cc6ae470c96defc41a2f1c4bcda1c8c550
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 3554018 | 100110 | 0 | 0 | 638054567 | 137187120 | 2865443880 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2865443880 | 0/1000 | 1548.7 | 28.2 | 53.6 | 25.85 / 29.93 |
| best | kafka | 22 × busy | 3554018 | 100110 | 0 | 0 | 638054567 | 137187120 | 2865443880 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2865443880 | 0/1000 | 1548.7 | 28.2 | 53.6 | 25.85 / 29.93 |

Run 1's profile:

| profile: the window, 30.0 s, 3002642 messages, 305585868 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 705.5G | 234.9k | 2309 | 7851 | 1875 | 31.48G | 10.31k | 100.6 |
| waiting for a CPU, ns | - | - | - | 2069 | 265.9 | - | - | - |
| allocations | 622.8M | 207.4 | 2.038 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 623.4M | 207.6 | 2.040 | 1.000 | 2.001 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 191.3G | 63.71k | 626.0 | 456.0 | 24.00 | 8.867G | 2905 | 28.35 |
| minor page faults | 2.580M | 0.859 | 0.00844 | 0.000577 | 0 | 257.0 | 0.0000842 | 0.000000822 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.415M | 0.471 | 0.00463 | 0.000157 | 0.000724 | - | - | - |
| involuntary context switches | 845.0k | 0.281 | 0.00277 | 0.0757 | 0.000941 | - | - | - |
| cycles | - | - | - | 33.57k | 10.27k | - | - | - |
| instructions | - | - | - | 51.78k | 15.23k | - | - | - |
| cache references | - | - | - | 33.37 | 21.56 | - | - | - |
| cache misses | - | - | - | 5.384 | 8.270 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0547 | 0.0666 | - | - | - |
| data TLB read misses | - | - | - | 0.194 | 0.0472 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 3.146M | 1.031 | 0.0101 |
| bytes written to storage | 0 | 0 | 0 | - | - | 34.27M | 11.23 | 0.110 |
| storage reads | - | - | - | - | - | 389.0 | 0.000127 | 0.00000124 |
| storage writes | - | - | - | - | - | 4005 | 0.00131 | 0.0000128 |
| garbage collections | - | - | - | - | - | 20.00 | 0.00000655 | 0.000000064 |
| collectors' pauses, ns | - | - | - | - | - | 77.05M | 25.25 | 0.246 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 2.122G | - | - | - | - | 8.590G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 1.928G | - | - | - | - | 8.584G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 220.1M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3052105 messages, 312785206 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet2-1024-1428/comparison/profiles/kafka-727533-0-kafka.jfr
