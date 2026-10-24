- broker: apache/kafka:4.1.0@sha256:bff074a5d0051dbc0bbbcd25b045bb1fe84833ec0d3c7c965d1797dd289ec88f, CPUs 8-15
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 10 over 10 threads, which wait when there's nothing to take; window 10 s after 2 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 10 × wait | 124861 | 9988 | 0 | 0 | 1248610 | 998040 | 0 | 311.3 | 478150.7 | 569339.1 | 569339.1 | 569339.1 | 0 | 0 | 10/10 | 43.2 | 12.5 | 50.7 | 15.63 / 16.39 |
| best | kafka | 10 × wait | 124861 | 9988 | 0 | 0 | 1248610 | 998040 | 0 | 311.3 | 478150.7 | 569339.1 | 569339.1 | 569339.1 | 0 | 0 | 10/10 | 43.2 | 12.5 | 50.7 | 15.63 / 16.39 |

Run 1's profile:

| profile: the window, 10.0 s, 99804 messages, 1006194 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 37.14G | 372.1k | 36.91k | 99.85k | 6457 | 36.82G | 350.9k | 31.33k |
| waiting for a CPU, ns | - | - | - | 18.81 | 62.65 | - | - | - |
| allocations | 11.09M | 111.2 | 11.03 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 11.10M | 111.2 | 11.03 | 1.000 | 4.337 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 2.497G | 25.02k | 2482 | 456.0 | 24.00 | 10.50G | 100.1k | 8939 |
| minor page faults | 1713 | 0.0172 | 0.00170 | 0.00522 | 0 | 332.2k | 3.166 | 0.283 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.964M | 19.68 | 1.952 | 0.0000802 | 0.781 | - | - | - |
| involuntary context switches | 1545 | 0.0155 | 0.00154 | 0.00175 | 0.000263 | - | - | - |
| cycles | - | - | - | 470.0k | 16.29k | - | - | - |
| instructions | - | - | - | 855.6k | 12.92k | - | - | - |
| cache references | - | - | - | 79.68 | 176.1 | - | - | - |
| cache misses | - | - | - | 6.190 | 10.24 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.172 | 0.251 | - | - | - |
| data TLB read misses | - | - | - | 0.225 | 8.222 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 245.8k | 2.342 | 0.209 |
| bytes written to storage | 0 | 0 | 0 | - | - | 1.810M | 17.26 | 1.541 |
| storage reads | - | - | - | - | - | 14.00 | 0.000133 | 0.0000119 |
| storage writes | - | - | - | - | - | 170.0 | 0.00162 | 0.000145 |
| garbage collections | - | - | - | - | - | 17.00 | 0.000162 | 0.0000145 |
| collectors' pauses, ns | - | - | - | - | - | 49.22M | 469.2 | 41.89 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 31.30M | - | - | - | - | 1.215G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 28.71M | - | - | - | - | 1.215G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 183.4M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 10.50 s from a quarter second before the window to a quarter second after it, 104916 messages, 1175006 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/profile-record-1911/comparison/profiles/kafka-591956-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 10 s after 2 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 124861 | 9988 | 0 | 0 | 12486100 | 9980400 | 0 | 2717909.0 | 4831838.2 | 5570035.7 | 6039797.8 | 6313554.4 | 0 | 0 | 100/100 | 67.3 | 12.5 | 42.3 | 16.30 / 17.34 |
| best | kafka | 22 × wait | 124861 | 9988 | 0 | 0 | 12486100 | 9980400 | 0 | 2717909.0 | 4831838.2 | 5570035.7 | 6039797.8 | 6313554.4 | 0 | 0 | 100/100 | 67.3 | 12.5 | 42.3 | 16.30 / 17.34 |

Run 1's profile:

| profile: the window, 10.0 s, 99804 messages, 8374501 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 45.10G | 451.9k | 5385 | 99.81k | 1255 | 27.29G | 260.1k | 3234 |
| waiting for a CPU, ns | - | - | - | 51.33 | 98.54 | - | - | - |
| allocations | 25.37M | 254.2 | 3.030 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 25.36M | 254.1 | 3.028 | 1.000 | 2.283 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 7.031G | 70.45k | 839.6 | 456.0 | 24.00 | 10.21G | 97.34k | 1210 |
| minor page faults | 281.8k | 2.824 | 0.0337 | 0.0516 | 0 | 38.02k | 0.362 | 0.00450 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.912M | 19.15 | 0.228 | 0.0000301 | 0.0961 | - | - | - |
| involuntary context switches | 5020 | 0.0503 | 0.000599 | 0.00229 | 0.000135 | - | - | - |
| cycles | - | - | - | 462.3k | 5019 | - | - | - |
| instructions | - | - | - | 847.7k | 5836 | - | - | - |
| cache references | - | - | - | 55.99 | 61.09 | - | - | - |
| cache misses | - | - | - | 8.495 | 9.382 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.205 | 0.165 | - | - | - |
| data TLB read misses | - | - | - | 0.855 | 3.016 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 1.831M | 17.45 | 0.217 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 172.0 | 0.00164 | 0.0000204 |
| garbage collections | - | - | - | - | - | 16.00 | 0.000153 | 0.00000190 |
| collectors' pauses, ns | - | - | - | - | - | 44.27M | 422.0 | 5.245 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 468.6M | - | - | - | - | 1.257G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 180.6M | - | - | - | - | 1.251G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 197.5M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 10.50 s from a quarter second before the window to a quarter second after it, 104916 messages, 8440553 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/profile-record-1911/comparison/profiles/kafka-594423-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 10 over 10 threads, which poll without waiting; window 10 s after 2 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 10 × busy | 124861 | 9988 | 0 | 0 | 1248610 | 998040 | 0 | 278.5 | 2031.6 | 65011.7 | 76546.0 | 80089.9 | 0 | 0 | 10/10 | 160.7 | 12.2 | 32.5 | 25.62 / 27.34 |
| best | kafka | 10 × busy | 124861 | 9988 | 0 | 0 | 1248610 | 998040 | 0 | 278.5 | 2031.6 | 65011.7 | 76546.0 | 80089.9 | 0 | 0 | 10/10 | 160.7 | 12.2 | 32.5 | 25.62 / 27.34 |

Run 1's profile:

| profile: the window, 10.0 s, 99806 messages, 1003679 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 125.4G | 1.256M | 124.9k | 99.02k | 96.44k | 26.82G | 255.6k | 25.54k |
| waiting for a CPU, ns | - | - | - | 830.7 | 965.2 | - | - | - |
| allocations | 9.715M | 97.34 | 9.680 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 9.717M | 97.36 | 9.681 | 1.000 | 3.971 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 2.215G | 22.19k | 2207 | 456.0 | 24.00 | 9.125G | 86.98k | 8691 |
| minor page faults | 91.00 | 0.000912 | 0.0000907 | 0.000130 | 0 | 1383 | 0.0132 | 0.00132 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.400M | 14.03 | 1.395 | 0.0000301 | 0.375 | - | - | - |
| involuntary context switches | 26.17k | 0.262 | 0.0261 | 0.0474 | 0.0136 | - | - | - |
| cycles | - | - | - | 179.3k | 497.9k | - | - | - |
| instructions | - | - | - | 319.5k | 779.0k | - | - | - |
| cache references | - | - | - | 26.06 | 164.0 | - | - | - |
| cache misses | - | - | - | 1.508 | 6.613 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0 | 0.200 | - | - | - |
| data TLB read misses | - | - | - | 0.00748 | 2.062 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 1.499M | 14.29 | 1.428 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 156.0 | 0.00149 | 0.000149 |
| garbage collections | - | - | - | - | - | 15.00 | 0.000143 | 0.0000143 |
| collectors' pauses, ns | - | - | - | - | - | 39.67M | 378.1 | 37.78 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 18.19M | - | - | - | - | 1.326G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 18.19M | - | - | - | - | 1.327G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 186.4M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 10.50 s from a quarter second before the window to a quarter second after it, 104915 messages, 1049957 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/profile-record-1911/comparison/profiles/kafka-600272-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 20e1c949603100cffb697bd0740a2bd89a78b35b
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 10 s after 2 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 124861 | 9988 | 0 | 0 | 12486100 | 9980400 | 0 | 2348810.2 | 3590324.2 | 4160749.6 | 4563402.8 | 4729079.9 | 0 | 0 | 100/100 | 310.6 | 11.8 | 34.5 | 32.86 / 37.29 |
| best | kafka | 22 × busy | 124861 | 9988 | 0 | 0 | 12486100 | 9980400 | 0 | 2348810.2 | 3590324.2 | 4160749.6 | 4563402.8 | 4729079.9 | 0 | 0 | 100/100 | 310.6 | 11.8 | 34.5 | 32.86 / 37.29 |

Run 1's profile:

| profile: the window, 10.0 s, 99804 messages, 9006569 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 189.2G | 1.896M | 21.01k | 94.06k | 17.54k | 25.03G | 238.6k | 2757 |
| waiting for a CPU, ns | - | - | - | 5691 | 6475 | - | - | - |
| allocations | 25.46M | 255.1 | 2.827 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 25.45M | 255.0 | 2.826 | 1.000 | 2.231 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 7.044G | 70.58k | 782.1 | 456.0 | 24.00 | 8.771G | 83.59k | 966.0 |
| minor page faults | 251.0k | 2.515 | 0.0279 | 0.0483 | 0.00000266 | 3171 | 0.0302 | 0.000349 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.120M | 11.22 | 0.124 | 0.0000401 | 0.0303 | - | - | - |
| involuntary context switches | 475.4k | 4.763 | 0.0528 | 0.223 | 0.0425 | - | - | - |
| cycles | - | - | - | 163.8k | 87.71k | - | - | - |
| instructions | - | - | - | 287.3k | 126.5k | - | - | - |
| cache references | - | - | - | 13.53 | 31.51 | - | - | - |
| cache misses | - | - | - | 2.898 | 3.584 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0000100 | 0.0545 | - | - | - |
| data TLB read misses | - | - | - | 0.343 | 0.973 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 1.425M | 13.58 | 0.157 |
| bytes written to storage | 0 | 0 | 0 | - | - | 17.30M | 164.9 | 1.906 |
| storage reads | - | - | - | - | - | 87.00 | 0.000829 | 0.00000958 |
| storage writes | - | - | - | - | - | 1117 | 0.0106 | 0.000123 |
| garbage collections | - | - | - | - | - | 13.00 | 0.000124 | 0.00000143 |
| collectors' pauses, ns | - | - | - | - | - | 44.94M | 428.3 | 4.949 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 352.6M | - | - | - | - | 1.375G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 242.7M | - | - | - | - | 1.370G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 200.1M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 10.50 s from a quarter second before the window to a quarter second after it, 104927 messages, 9079610 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/profile-record-1911/comparison/profiles/kafka-603503-0-kafka.jfr
