- broker: apache/kafka:4.1.0@sha256:bff074a5d0051dbc0bbbcd25b045bb1fe84833ec0d3c7c965d1797dd289ec88f, CPUs 8-15
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 60.9 | 141.3 | 237.6 | 2752.5 | 15319.8 | 0 | 0 | 1/1 | 47.4 | 35.4 | 31.5 | 1.56 / 2.07 |
| best | kafka | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 60.9 | 141.3 | 237.6 | 2752.5 | 15319.8 | 0 | 0 | 1/1 | 47.4 | 35.4 | 31.5 | 1.56 / 2.07 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 299494 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 39.97G | 133.4k | 133.4k | 99.97k | 6640 | 16.71G | 54.86k | 54.86k |
| waiting for a CPU, ns | - | - | - | 4.433 | 13.22 | - | - | - |
| allocations | 7.066M | 23.59 | 23.59 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 7.066M | 23.59 | 23.59 | 1.000 | 4.702 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 1.687G | 5633 | 5633 | 457.0 | 24.00 | 7.185G | 23.59k | 23.59k |
| minor page faults | 0 | 0 | 0 | 0 | 0 | 27.43k | 0.0900 | 0.0900 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.563M | 5.218 | 5.218 | 0.000114 | 0.912 | - | - | - |
| involuntary context switches | 268.0 | 0.000895 | 0.000895 | 0.000521 | 0.0000568 | - | - | - |
| cycles | - | - | - | 578.4k | 14.87k | - | - | - |
| instructions | - | - | - | 1.072M | 12.32k | - | - | - |
| cache references | - | - | - | 61.53 | 157.6 | - | - | - |
| cache misses | - | - | - | 2.855 | 1.485 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0156 | 0.0270 | - | - | - |
| data TLB read misses | - | - | - | 0.201 | 0.733 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 184.3k | 0.605 | 0.605 |
| bytes written to storage | 0 | 0 | 0 | - | - | 12.63M | 41.46 | 41.46 |
| storage reads | - | - | - | - | - | 11.00 | 0.0000361 | 0.0000361 |
| storage writes | - | - | - | - | - | 1283 | 0.00421 | 0.00421 |
| garbage collections | - | - | - | - | - | 12.00 | 0.0000394 | 0.0000394 |
| collectors' pauses, ns | - | - | - | - | - | 30.04M | 98.61 | 98.61 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 14.81M | - | - | - | - | 1.243G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 14.37M | - | - | - | - | 1.244G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 181.3M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304612 messages, 304610 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3066191-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 2449473.5 | 5100273.7 | 8724152.3 | 11945377.8 | 12960239.2 | 0 | 0 | 100/100 | 174.7 | 35.4 | 116.2 | 5.40 / 6.55 |
| best | kafka | 22 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 2449473.5 | 5100273.7 | 8724152.3 | 11945377.8 | 12960239.2 | 0 | 0 | 100/100 | 174.7 | 35.4 | 116.2 | 5.40 / 6.55 |

Run 1's profile:

| profile: the window, 30.0 s, 299499 messages, 29334019 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 140.6G | 469.5k | 4794 | 99.88k | 1152 | 90.74G | 297.9k | 2960 |
| waiting for a CPU, ns | - | - | - | 37.31 | 43.17 | - | - | - |
| allocations | 89.95M | 300.3 | 3.066 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 89.93M | 300.3 | 3.066 | 1.000 | 2.291 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 25.00G | 83.47k | 852.2 | 457.0 | 24.00 | 34.18G | 112.2k | 1115 |
| minor page faults | 830.8k | 2.774 | 0.0283 | 0.0282 | 0.000000170 | 23.71k | 0.0778 | 0.000774 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 7.167M | 23.93 | 0.244 | 0.000147 | 0.0990 | - | - | - |
| involuntary context switches | 19.72k | 0.0658 | 0.000672 | 0.00298 | 0.000133 | - | - | - |
| cycles | - | - | - | 494.7k | 4765 | - | - | - |
| instructions | - | - | - | 902.9k | 5611 | - | - | - |
| cache references | - | - | - | 72.06 | 50.28 | - | - | - |
| cache misses | - | - | - | 7.799 | 7.160 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0815 | 0.111 | - | - | - |
| data TLB read misses | - | - | - | 1.805 | 1.732 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 4096 | 0.0134 | 0.000134 |
| bytes written to storage | 0 | 0 | 0 | - | - | 12.96M | 42.53 | 0.423 |
| storage reads | - | - | - | - | - | 1.000 | 0.00000328 | 0.000000033 |
| storage writes | - | - | - | - | - | 1218 | 0.00400 | 0.0000397 |
| garbage collections | - | - | - | - | - | 53.00 | 0.000174 | 0.00000173 |
| collectors' pauses, ns | - | - | - | - | - | 90.01M | 295.5 | 2.937 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 437.1M | - | - | - | - | 1.443G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 389.8M | - | - | - | - | 1.443G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 220.4M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304612 messages, 30649583 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3071719-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 104658 | 2947 | 249932 | 0 | 104658000 | 54594000 | 244903000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 289.3 | 35.4 | 139.3 | 9.29 / 10.30 |
| best | kafka | 22 × wait | 104658 | 2947 | 249932 | 0 | 104658000 | 54594000 | 244903000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 289.3 | 35.4 | 139.3 | 9.29 / 10.30 |

Run 1's profile:

| profile: the window, 30.0 s, 54530 messages, 3936458 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 158.2G | 2.902M | 40.20k | 548.4k | 8446 | 100.8G | 1.765M | 25.18k |
| waiting for a CPU, ns | - | - | - | 147.4 | 155.8 | - | - | - |
| allocations | 117.5M | 2155 | 29.86 | 1353 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 117.5M | 2155 | 29.85 | 1352 | 4.732 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 11.41G | 209.2k | 2898 | 34.23k | 24.00 | 40.08G | 701.9k | 10.01k |
| minor page faults | 5606 | 0.103 | 0.00142 | 0.0974 | 0 | 10.08k | 0.176 | 0.00252 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 7.526M | 138.0 | 1.912 | 0.00392 | 0.890 | - | - | - |
| involuntary context switches | 31.64k | 0.580 | 0.00804 | 0.0191 | 0.00171 | - | - | - |
| cycles | - | - | - | 2.986M | 24.60k | - | - | - |
| instructions | - | - | - | 6.753M | 14.00k | - | - | - |
| cache references | - | - | - | 145.0 | 574.5 | - | - | - |
| cache misses | - | - | - | 63.08 | 145.5 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 2.666 | 12.90 | - | - | - |
| data TLB read misses | - | - | - | 1.938 | 39.06 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 7.328M | 128.3 | 1.831 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 468.0 | 0.00820 | 0.000117 |
| garbage collections | - | - | - | - | - | 63.00 | 0.00110 | 0.0000157 |
| collectors' pauses, ns | - | - | - | - | - | 115.4M | 2020 | 28.83 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 581.6M | - | - | - | - | 1.492G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 581.6M | - | - | - | - | 1.491G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 235.8M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 57102 messages, 4001976 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3079165-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 12750684.2 | 14629732.4 | 15971909.6 | 16156424.0 | 16156424.0 | 0 | 0 | 1/1 | 57.6 | 35.4 | 37.7 | 2.74 / 2.66 |
| best | kafka | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 12750684.2 | 14629732.4 | 15971909.6 | 16156424.0 | 16156424.0 | 0 | 0 | 1/1 | 57.6 | 35.4 | 37.7 | 2.74 / 2.66 |

Run 1's profile:

| profile: the window, 30.0 s, 2998271 messages, 1982422 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 47.32G | 15.78k | 23.87k | 9979 | 526.8 | 30.61G | 10.04k | 15.44k |
| waiting for a CPU, ns | - | - | - | 0.287 | 3.187 | - | - | - |
| allocations | 27.91M | 9.308 | 14.08 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 27.91M | 9.307 | 14.08 | 1.000 | 2.001 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 7.798G | 2601 | 3934 | 457.0 | 24.00 | 23.70G | 7776 | 11.96k |
| minor page faults | 103.1k | 0.0344 | 0.0520 | 0.0243 | 0 | 6008 | 0.00197 | 0.00303 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.959M | 1.321 | 1.997 | 0.000319 | 0.0130 | - | - | - |
| involuntary context switches | 198.0 | 0.0000660 | 0.0000999 | 0.0000337 | 0.000000504 | - | - | - |
| cycles | - | - | - | 58.20k | 2886 | - | - | - |
| instructions | - | - | - | 103.8k | 3391 | - | - | - |
| cache references | - | - | - | 38.03 | 34.12 | - | - | - |
| cache misses | - | - | - | 2.613 | 1.368 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00272 | 0.0109 | - | - | - |
| data TLB read misses | - | - | - | 0.0939 | 0.370 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 20.48k | 0.00672 | 0.0103 |
| bytes written to storage | 0 | 0 | 0 | - | - | 38.94M | 12.77 | 19.64 |
| storage reads | - | - | - | - | - | 4.000 | 0.00000131 | 0.00000202 |
| storage writes | - | - | - | - | - | 4492 | 0.00147 | 0.00227 |
| garbage collections | - | - | - | - | - | 37.00 | 0.0000121 | 0.0000187 |
| collectors' pauses, ns | - | - | - | - | - | 54.74M | 17.96 | 27.61 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 31.72M | - | - | - | - | 2.584G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 29.62M | - | - | - | - | 2.593G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 246.1M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3048296 messages, 1982422 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3089736-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 3547543 | 99928 | 0 | 0 | 82196866 | 32302448 | 267524952 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 267524952 | 0/100 | 150.2 | 35.4 | 51.2 | 8.92 / 5.03 |
| best | kafka | 22 × wait | 3547543 | 99928 | 0 | 0 | 82196866 | 32302448 | 267524952 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 267524952 | 0/100 | 150.2 | 35.4 | 51.2 | 8.92 / 5.03 |

Run 1's profile:

| profile: the window, 30.0 s, 2998279 messages, 28640154 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 72.05G | 24.03k | 2516 | 9970 | 554.4 | 31.47G | 10.32k | 1073 |
| waiting for a CPU, ns | - | - | - | 2.643 | 11.11 | - | - | - |
| allocations | 79.77M | 26.61 | 2.785 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 79.77M | 26.61 | 2.785 | 1.000 | 2.001 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 23.80G | 7936 | 830.8 | 457.0 | 24.00 | 21.88G | 7178 | 746.2 |
| minor page faults | 996.3k | 0.332 | 0.0348 | 0.0242 | 0 | 6312 | 0.00207 | 0.000215 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.987M | 1.330 | 0.139 | 0.000846 | 0.0113 | - | - | - |
| involuntary context switches | 1441 | 0.000481 | 0.0000503 | 0.0000770 | 0.0000170 | - | - | - |
| cycles | - | - | - | 64.92k | 3105 | - | - | - |
| instructions | - | - | - | 77.53k | 4397 | - | - | - |
| cache references | - | - | - | 48.14 | 31.47 | - | - | - |
| cache misses | - | - | - | 3.490 | 6.584 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00587 | 0.0760 | - | - | - |
| data TLB read misses | - | - | - | 0.290 | 0.312 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 54.11M | 17.75 | 1.845 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 6382 | 0.00209 | 0.000218 |
| garbage collections | - | - | - | - | - | 35.00 | 0.0000115 | 0.00000119 |
| collectors' pauses, ns | - | - | - | - | - | 59.73M | 19.59 | 2.037 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 434.4M | - | - | - | - | 3.728G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 366.7M | - | - | - | - | 3.737G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 365.0M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3048290 messages, 29320275 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3095127-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 106409 | 2983 | 3441134 | 0 | 106409000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 309.1 | 35.6 | 148.9 | 10.51 / 11.07 |
| best | kafka | 22 × wait | 106409 | 2983 | 3441134 | 0 | 106409000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 309.1 | 35.6 | 148.9 | 10.51 / 11.07 |

Run 1's profile:

| profile: the window, 30.0 s, 5365 messages, 5365121 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 157.0G | 29.27M | 29.27k | 5.568M | 6184 | 93.11G | 17.04M | 17.04k |
| waiting for a CPU, ns | - | - | - | 1642 | 116.9 | - | - | - |
| allocations | 124.9M | 23.28k | 23.28 | 15.09k | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 124.9M | 23.28k | 23.28 | 15.09k | 3.855 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 11.88G | 2.214M | 2214 | 377.6k | 24.00 | 37.11G | 6.794M | 6794 |
| minor page faults | 226.0 | 0.0421 | 0.0000421 | 0 | 0.000000186 | 6746 | 1.235 | 0.00123 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 7.007M | 1306 | 1.306 | 0.0199 | 0.608 | - | - | - |
| involuntary context switches | 24.97k | 4.655 | 0.00465 | 0.147 | 0.00101 | - | - | - |
| cycles | - | - | - | 29.15M | 18.13k | - | - | - |
| instructions | - | - | - | 68.02M | 10.95k | - | - | - |
| cache references | - | - | - | 1480 | 405.9 | - | - | - |
| cache misses | - | - | - | 689.2 | 108.5 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 29.42 | 10.24 | - | - | - |
| data TLB read misses | - | - | - | 18.56 | 26.60 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 7.721M | 1413 | 1.413 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 493.0 | 0.0902 | 0.0000902 |
| garbage collections | - | - | - | - | - | 59.00 | 0.0108 | 0.0000108 |
| collectors' pauses, ns | - | - | - | - | - | 130.3M | 23.85k | 23.85 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 570.1M | - | - | - | - | 3.786G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 570.1M | - | - | - | - | 3.785G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 384.4M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 5463 messages, 5462811 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3109473-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 69.6 | 159.7 | 360.4 | 3604.5 | 18695.6 | 0 | 0 | 1/1 | 82.7 | 35.4 | 21.8 | 3.86 / 3.83 |
| best | kafka | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 69.6 | 159.7 | 360.4 | 3604.5 | 18695.6 | 0 | 0 | 1/1 | 82.7 | 35.4 | 21.8 | 3.86 / 3.83 |

Run 1's profile:

| profile: the window, 30.0 s, 299500 messages, 299500 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 69.26G | 231.3k | 231.3k | 99.94k | 98.97k | 17.65G | 57.93k | 57.93k |
| waiting for a CPU, ns | - | - | - | 17.96 | 26.60 | - | - | - |
| allocations | 6.936M | 23.16 | 23.16 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.936M | 23.16 | 23.16 | 1.000 | 4.583 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 1.660G | 5543 | 5543 | 457.0 | 24.00 | 7.245G | 23.78k | 23.78k |
| minor page faults | 62.00 | 0.000207 | 0.000207 | 0 | 0 | 956.0 | 0.00314 | 0.00314 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.294M | 4.321 | 4.321 | 0.000157 | 0.405 | - | - | - |
| involuntary context switches | 2055 | 0.00686 | 0.00686 | 0.00197 | 0.00171 | - | - | - |
| cycles | - | - | - | 549.4k | 626.7k | - | - | - |
| instructions | - | - | - | 1.006M | 1.049M | - | - | - |
| cache references | - | - | - | 82.06 | 188.8 | - | - | - |
| cache misses | - | - | - | 6.855 | 7.780 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0545 | 0.0675 | - | - | - |
| data TLB read misses | - | - | - | 0.441 | 1.704 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 11.81M | 38.77 | 38.77 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 1210 | 0.00397 | 0.00397 |
| garbage collections | - | - | - | - | - | 12.00 | 0.0000394 | 0.0000394 |
| collectors' pauses, ns | - | - | - | - | - | 31.62M | 103.8 | 103.8 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 9.650M | - | - | - | - | 3.950G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 9.650M | - | - | - | - | 3.951G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 391.3M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304612 messages, 304612 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3121805-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 1627390.0 | 2785017.9 | 3489660.9 | 3858759.7 | 4346212.5 | 0 | 0 | 100/100 | 881.1 | 34.2 | 98.5 | 24.54 / 32.16 |
| best | kafka | 22 × busy | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 1627390.0 | 2785017.9 | 3489660.9 | 3858759.7 | 4346212.5 | 0 | 0 | 100/100 | 881.1 | 34.2 | 98.5 | 24.54 / 32.16 |

Run 1's profile:

| profile: the window, 30.0 s, 299477 messages, 29858370 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 676.9G | 2.260M | 22.67k | 96.22k | 19.52k | 78.00G | 256.1k | 2498 |
| waiting for a CPU, ns | - | - | - | 3544 | 2229 | - | - | - |
| allocations | 83.58M | 279.1 | 2.799 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 83.58M | 279.1 | 2.799 | 1.000 | 2.226 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 23.23G | 77.57k | 778.0 | 457.0 | 24.00 | 26.02G | 85.42k | 833.2 |
| minor page faults | 781.9k | 2.611 | 0.0262 | 0.0460 | 0 | 4370 | 0.0143 | 0.000140 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.785M | 12.64 | 0.127 | 0.0000301 | 0.0347 | - | - | - |
| involuntary context switches | 1.165M | 3.890 | 0.0390 | 0.257 | 0.0310 | - | - | - |
| cycles | - | - | - | 414.9k | 97.79k | - | - | - |
| instructions | - | - | - | 679.9k | 143.7k | - | - | - |
| cache references | - | - | - | 35.84 | 38.56 | - | - | - |
| cache misses | - | - | - | 6.874 | 5.607 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.236 | 0.0849 | - | - | - |
| data TLB read misses | - | - | - | 1.032 | 1.049 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 12.28M | 40.33 | 0.393 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 1114 | 0.00366 | 0.0000357 |
| garbage collections | - | - | - | - | - | 44.00 | 0.000144 | 0.00000141 |
| collectors' pauses, ns | - | - | - | - | - | 116.4M | 382.2 | 3.728 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 391.6M | - | - | - | - | 4.064G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 201.8M | - | - | - | - | 4.063G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 440.3M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304612 messages, 31230435 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3129770-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 354590 | 9988 | 0 | 0 | 330097167 | 277794490 | 21702510 | 31675383.8 | 36507222.0 | undelivered | undelivered | undelivered | 0 | 21702510 | 437/1000 | 1510.4 | 34.9 | 115.1 | 26.68 / 28.90 |
| best | kafka | 22 × busy | 354590 | 9988 | 0 | 0 | 330097167 | 277794490 | 21702510 | 31675383.8 | 36507222.0 | undelivered | undelivered | undelivered | 0 | 21702510 | 437/1000 | 1510.4 | 34.9 | 115.1 | 26.68 / 28.90 |

Run 1's profile:

| profile: the window, 30.0 s, 299516 messages, 43802933 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 677.2G | 2.261M | 15.46k | 98.66k | 12.68k | 82.41G | 270.5k | 1770 |
| waiting for a CPU, ns | - | - | - | 979.0 | 1968 | - | - | - |
| allocations | 154.7M | 516.5 | 3.532 | 133.5 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 154.2M | 515.0 | 3.521 | 132.5 | 2.179 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 34.25G | 114.4k | 781.9 | 3745 | 24.00 | 28.88G | 94.82k | 620.3 |
| minor page faults | 813.0k | 2.714 | 0.0186 | 0.0169 | 0 | 4267 | 0.0140 | 0.0000916 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 4.335M | 14.47 | 0.0990 | 0.000591 | 0.0213 | - | - | - |
| involuntary context switches | 1.180M | 3.940 | 0.0269 | 0.0234 | 0.0226 | - | - | - |
| cycles | - | - | - | 463.8k | 64.77k | - | - | - |
| instructions | - | - | - | 813.5k | 94.25k | - | - | - |
| cache references | - | - | - | 129.5 | 47.06 | - | - | - |
| cache misses | - | - | - | 37.63 | 15.28 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.338 | 0.661 | - | - | - |
| data TLB read misses | - | - | - | 1.043 | 1.919 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 10.38M | 34.09 | 0.223 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 913.0 | 0.00300 | 0.0000196 |
| garbage collections | - | - | - | - | - | 55.00 | 0.000181 | 0.00000118 |
| collectors' pauses, ns | - | - | - | - | - | 137.8M | 452.3 | 2.959 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 2.847G | - | - | - | - | 4.419G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 2.096G | - | - | - | - | 4.183G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 323.9M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304617 messages, 46563300 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3140788-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 90177.5 | 1275068.4 | 2248146.9 | 2382364.7 | 2400549.7 | 0 | 0 | 1/1 | 100.1 | 35.4 | 55.6 | 5.28 / 5.18 |
| best | kafka | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 90177.5 | 1275068.4 | 2248146.9 | 2382364.7 | 2400549.7 | 0 | 0 | 1/1 | 100.1 | 35.4 | 55.6 | 5.28 / 5.18 |

Run 1's profile:

| profile: the window, 30.0 s, 2998278 messages, 3000903 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 83.32G | 27.79k | 27.77k | 9968 | 9774 | 45.56G | 14.94k | 14.95k |
| waiting for a CPU, ns | - | - | - | 0.634 | 1.365 | - | - | - |
| allocations | 33.34M | 11.12 | 11.11 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 33.35M | 11.12 | 11.11 | 1.000 | 2.542 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 9.036G | 3014 | 3011 | 457.0 | 24.00 | 27.12G | 8896 | 8897 |
| minor page faults | 131.3k | 0.0438 | 0.0438 | 0.0298 | 0 | 629.0 | 0.000206 | 0.000206 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 4.707M | 1.570 | 1.568 | 0.000515 | 0.0975 | - | - | - |
| involuntary context switches | 744.0 | 0.000248 | 0.000248 | 0.0000610 | 0.0000663 | - | - | - |
| cycles | - | - | - | 56.31k | 62.74k | - | - | - |
| instructions | - | - | - | 98.56k | 101.8k | - | - | - |
| cache references | - | - | - | 39.09 | 60.33 | - | - | - |
| cache misses | - | - | - | 2.083 | 0.616 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00639 | 0.0112 | - | - | - |
| data TLB read misses | - | - | - | 0.111 | 0.172 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 39.40M | 12.93 | 12.93 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 4502 | 0.00148 | 0.00148 |
| garbage collections | - | - | - | - | - | 43.00 | 0.0000141 | 0.0000141 |
| collectors' pauses, ns | - | - | - | - | - | 78.69M | 25.82 | 25.82 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 30.36M | - | - | - | - | 5.221G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 25.73M | - | - | - | - | 5.227G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 351.6M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3048286 messages, 3048032 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3150605-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 3547543 | 99928 | 0 | 0 | 135608364 | 85222182 | 214605218 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 214605218 | 9/100 | 1462.4 | 31.7 | 87.4 | 24.61 / 31.57 |
| best | kafka | 22 × busy | 3547543 | 99928 | 0 | 0 | 135608364 | 85222182 | 214605218 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 214605218 | 9/100 | 1462.4 | 31.7 | 87.4 | 24.61 / 31.57 |

Run 1's profile:

| profile: the window, 30.0 s, 2998368 messages, 67206650 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 694.3G | 231.6k | 10.33k | 8968 | 9070 | 70.08G | 22.99k | 1035 |
| waiting for a CPU, ns | - | - | - | 1002 | 628.1 | - | - | - |
| allocations | 158.3M | 52.81 | 2.356 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 158.3M | 52.81 | 2.356 | 1.000 | 2.065 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 46.52G | 15.51k | 692.1 | 457.0 | 24.00 | 21.50G | 7052 | 317.4 |
| minor page faults | 524.2k | 0.175 | 0.00780 | 0.000561 | 0 | 8574 | 0.00281 | 0.000127 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.106M | 1.036 | 0.0462 | 0.0000223 | 0.0128 | - | - | - |
| involuntary context switches | 1.203M | 0.401 | 0.0179 | 0.0585 | 0.0120 | - | - | - |
| cycles | - | - | - | 37.77k | 46.32k | - | - | - |
| instructions | - | - | - | 58.52k | 68.25k | - | - | - |
| cache references | - | - | - | 18.85 | 24.77 | - | - | - |
| cache misses | - | - | - | 3.120 | 1.937 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0271 | 0.0252 | - | - | - |
| data TLB read misses | - | - | - | 0.0606 | 0.167 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 16.38k | 0.00537 | 0.000242 |
| bytes written to storage | 0 | 0 | 0 | - | - | 31.28M | 10.26 | 0.462 |
| storage reads | - | - | - | - | - | 1.000 | 0.000000328 | 0.000000015 |
| storage writes | - | - | - | - | - | 3473 | 0.00114 | 0.0000513 |
| garbage collections | - | - | - | - | - | 33.00 | 0.0000108 | 0.000000487 |
| collectors' pauses, ns | - | - | - | - | - | 96.14M | 31.54 | 1.419 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 426.0M | - | - | - | - | 5.271G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 413.1M | - | - | - | - | 5.277G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 360.5M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3048223 messages, 67731822 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3154873-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 3547543 | 99928 | 0 | 0 | 631155646 | 132222519 | 2866051481 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2866051481 | 0/1000 | 1517.2 | 26.5 | 61.9 | 34.56 / 39.28 |
| best | kafka | 22 × busy | 3547543 | 99928 | 0 | 0 | 631155646 | 132222519 | 2866051481 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2866051481 | 0/1000 | 1517.2 | 26.5 | 61.9 | 34.56 / 39.28 |

Run 1's profile:

| profile: the window, 30.0 s, 2998352 messages, 303743960 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 676.2G | 225.5k | 2226 | 7357 | 1783 | 34.94G | 11.46k | 113.9 |
| waiting for a CPU, ns | - | - | - | 2583 | 370.0 | - | - | - |
| allocations | 618.3M | 206.2 | 2.036 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 618.2M | 206.2 | 2.035 | 1.000 | 2.001 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 191.1G | 63.74k | 629.2 | 457.0 | 24.00 | 7.066G | 2318 | 23.04 |
| minor page faults | 2.454M | 0.818 | 0.00808 | 0.00176 | 0 | 681.0 | 0.000223 | 0.00000222 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.266M | 0.422 | 0.00417 | 0.000151 | 0.000585 | - | - | - |
| involuntary context switches | 868.3k | 0.290 | 0.00286 | 0.0917 | 0.00104 | - | - | - |
| cycles | - | - | - | 29.03k | 9296 | - | - | - |
| instructions | - | - | - | 45.19k | 13.66k | - | - | - |
| cache references | - | - | - | 25.01 | 21.63 | - | - | - |
| cache misses | - | - | - | 8.762 | 10.21 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.103 | 0.137 | - | - | - |
| data TLB read misses | - | - | - | 0.175 | 0.139 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 770.0k | 0.253 | 0.00251 |
| bytes written to storage | 0 | 0 | 0 | - | - | 45.76M | 15.01 | 0.149 |
| storage reads | - | - | - | - | - | 47.00 | 0.0000154 | 0.000000153 |
| storage writes | - | - | - | - | - | 5516 | 0.00181 | 0.0000180 |
| garbage collections | - | - | - | - | - | 11.00 | 0.00000361 | 0.000000036 |
| collectors' pauses, ns | - | - | - | - | - | 79.66M | 26.13 | 0.260 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 1.886G | - | - | - | - | 5.137G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 1.823G | - | - | - | - | 5.140G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 406.8M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3048417 messages, 306679663 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3166595-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 77.8 | 157.7 | 290.8 | 3342.3 | 16163.1 | 0 | 0 | 1/1 | 50.0 | 35.4 | 23.2 | 10.67 / 8.45 |
| best | kafka | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 77.8 | 157.7 | 290.8 | 3342.3 | 16163.1 | 0 | 0 | 1/1 | 50.0 | 35.4 | 23.2 | 10.67 / 8.45 |

Run 1's profile:

| profile: the window, 30.0 s, 299587 messages, 299588 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 42.07G | 140.4k | 140.4k | 99.83k | 6594 | 18.79G | 61.70k | 61.69k |
| waiting for a CPU, ns | - | - | - | 4.933 | 9.069 | - | - | - |
| allocations | 6.961M | 23.24 | 23.24 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.961M | 23.24 | 23.24 | 1.000 | 4.620 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 1.666G | 5560 | 5560 | 457.0 | 24.00 | 7.274G | 23.88k | 23.88k |
| minor page faults | 40.00 | 0.000134 | 0.000134 | 0 | 0 | 264.0 | 0.000867 | 0.000867 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.392M | 4.646 | 4.646 | 0.0000868 | 0.873 | - | - | - |
| involuntary context switches | 380.0 | 0.00127 | 0.00127 | 0.000587 | 0.0000701 | - | - | - |
| cycles | - | - | - | 526.6k | 15.57k | - | - | - |
| instructions | - | - | - | 965.2k | 12.67k | - | - | - |
| cache references | - | - | - | 154.1 | 143.9 | - | - | - |
| cache misses | - | - | - | 28.66 | 14.70 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.171 | 0.428 | - | - | - |
| data TLB read misses | - | - | - | 0.143 | 1.577 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 12.21M | 40.07 | 40.07 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 1248 | 0.00410 | 0.00410 |
| garbage collections | - | - | - | - | - | 12.00 | 0.0000394 | 0.0000394 |
| collectors' pauses, ns | - | - | - | - | - | 40.10M | 131.6 | 131.6 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 9.839M | - | - | - | - | 5.243G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 9.839M | - | - | - | - | 5.244G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 419.5M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304590 messages, 304591 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3177310-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 5100273.7 | 8992587.8 | 13019119.6 | 14361296.9 | 15047383.4 | 0 | 0 | 100/100 | 144.2 | 35.4 | 84.6 | 15.15 / 13.14 |
| best | kafka | 22 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 5100273.7 | 8992587.8 | 13019119.6 | 14361296.9 | 15047383.4 | 0 | 0 | 100/100 | 144.2 | 35.4 | 84.6 | 15.15 / 13.14 |

Run 1's profile:

| profile: the window, 30.0 s, 299584 messages, 29804410 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 114.6G | 382.7k | 3846 | 99.71k | 948.5 | 66.36G | 217.9k | 2221 |
| waiting for a CPU, ns | - | - | - | 35.61 | 28.38 | - | - | - |
| allocations | 78.51M | 262.1 | 2.634 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 78.56M | 262.2 | 2.636 | 1.000 | 2.170 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 22.46G | 74.97k | 753.6 | 457.0 | 24.00 | 21.40G | 70.24k | 716.2 |
| minor page faults | 565.7k | 1.888 | 0.0190 | 0.0579 | 0 | 3001 | 0.00985 | 0.000100 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 4.317M | 14.41 | 0.145 | 0.0000834 | 0.0608 | - | - | - |
| involuntary context switches | 7094 | 0.0237 | 0.000238 | 0.00240 | 0.0000571 | - | - | - |
| cycles | - | - | - | 447.4k | 4043 | - | - | - |
| instructions | - | - | - | 824.0k | 5138 | - | - | - |
| cache references | - | - | - | 50.07 | 42.14 | - | - | - |
| cache misses | - | - | - | 9.365 | 7.958 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.261 | 0.168 | - | - | - |
| data TLB read misses | - | - | - | 0.477 | 1.246 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 11.77M | 38.64 | 0.394 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 1125 | 0.00369 | 0.0000377 |
| garbage collections | - | - | - | - | - | 41.00 | 0.000135 | 0.00000137 |
| collectors' pauses, ns | - | - | - | - | - | 145.0M | 476.0 | 4.853 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 471.5M | - | - | - | - | 5.293G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 397.7M | - | - | - | - | 5.293G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 232.3M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304585 messages, 29873090 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3184500-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 103906 | 2926 | 250892 | 0 | 103906000 | 53755000 | 245828000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 375.4 | 35.4 | 198.2 | 10.21 / 66.64 |
| best | kafka | 22 × wait | 103906 | 2926 | 250892 | 0 | 103906000 | 53755000 | 245828000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 375.4 | 35.4 | 198.2 | 10.21 / 66.64 |

Run 1's profile:

| profile: the window, 30.0 s, 53698 messages, 3215404 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 153.6G | 2.861M | 47.78k | 555.2k | 9931 | 94.12G | 1.675M | 28.74k |
| waiting for a CPU, ns | - | - | - | 463.6 | 6227 | - | - | - |
| allocations | 84.51M | 1574 | 26.28 | 927.5 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 84.47M | 1573 | 26.27 | 926.5 | 4.621 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 8.859G | 165.0k | 2755 | 23.60k | 24.00 | 31.34G | 557.9k | 9570 |
| minor page faults | 5818 | 0.108 | 0.00181 | 0.0957 | 0.00000124 | 4745 | 0.0845 | 0.00145 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 5.792M | 107.9 | 1.801 | 0.00140 | 0.844 | - | - | - |
| involuntary context switches | 166.9k | 3.109 | 0.0519 | 0.00685 | 0.00438 | - | - | - |
| cycles | - | - | - | 2.716M | 27.08k | - | - | - |
| instructions | - | - | - | 5.391M | 12.23k | - | - | - |
| cache references | - | - | - | 489.5 | 646.9 | - | - | - |
| cache misses | - | - | - | 149.4 | 173.2 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 6.302 | 17.02 | - | - | - |
| data TLB read misses | - | - | - | 2.922 | 38.90 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 7.504M | 133.6 | 2.291 |
| bytes written to storage | 0 | 0 | 0 | - | - | 83.10M | 1479 | 25.37 |
| storage reads | - | - | - | - | - | 458.0 | 0.00815 | 0.000140 |
| storage writes | - | - | - | - | - | 5152 | 0.0917 | 0.00157 |
| garbage collections | - | - | - | - | - | 49.00 | 0.000872 | 0.0000150 |
| collectors' pauses, ns | - | - | - | - | - | 150.7M | 2682 | 46.01 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 573.3M | - | - | - | - | 5.320G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 573.3M | - | - | - | - | 5.318G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 248.3M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 56179 messages, 3275106 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3204566-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 22548578.3 | 24964497.4 | 26843545.6 | 27007013.7 | 27007013.7 | 0 | 0 | 1/1 | 61.3 | 34.9 | 39.2 | 34.24 / 27.87 |
| best | kafka | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 22548578.3 | 24964497.4 | 26843545.6 | 27007013.7 | 27007013.7 | 0 | 0 | 1/1 | 61.3 | 34.9 | 39.2 | 34.24 / 27.87 |

Run 1's profile:

| profile: the window, 30.0 s, 3000446 messages, 1044538 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 49.18G | 16.39k | 47.09k | 9818 | 607.0 | 30.93G | 10.14k | 29.61k |
| waiting for a CPU, ns | - | - | - | 135.2 | 34.71 | - | - | - |
| allocations | 20.94M | 6.977 | 20.04 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 20.95M | 6.981 | 20.05 | 1.000 | 2.001 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 6.073G | 2024 | 5815 | 457.0 | 24.00 | 17.23G | 5648 | 16.49k |
| minor page faults | 147.6k | 0.0492 | 0.141 | 0.0321 | 0 | 16.93k | 0.00555 | 0.0162 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 2.551M | 0.850 | 2.442 | 0.000353 | 0.0158 | - | - | - |
| involuntary context switches | 63.20k | 0.0211 | 0.0605 | 0.0179 | 0.000245 | - | - | - |
| cycles | - | - | - | 48.04k | 3270 | - | - | - |
| instructions | - | - | - | 84.24k | 4392 | - | - | - |
| cache references | - | - | - | 41.08 | 30.03 | - | - | - |
| cache misses | - | - | - | 4.581 | 4.361 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0448 | 0.178 | - | - | - |
| data TLB read misses | - | - | - | 0.0942 | 0.313 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 26.61M | 8.724 | 25.48 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 3066 | 0.00101 | 0.00294 |
| garbage collections | - | - | - | - | - | 37.00 | 0.0000121 | 0.0000354 |
| collectors' pauses, ns | - | - | - | - | - | 93.92M | 30.79 | 89.92 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 32.12M | - | - | - | - | 4.776G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 22.47M | - | - | - | - | 4.779G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 214.3M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3050396 messages, 1044538 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3281142-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 3550038 | 99998 | 0 | 0 | 77893088 | 28184685 | 271859015 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 271859015 | 0/100 | 136.8 | 35.3 | 50.7 | 12.79 / 6.81 |
| best | kafka | 22 × wait | 3550038 | 99998 | 0 | 0 | 77893088 | 28184685 | 271859015 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 271859015 | 0/100 | 136.8 | 35.3 | 50.7 | 12.79 / 6.81 |

Run 1's profile:

| profile: the window, 30.0 s, 3000442 messages, 27693905 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 71.47G | 23.82k | 2581 | 9949 | 535.7 | 31.55G | 10.34k | 1139 |
| waiting for a CPU, ns | - | - | - | 5.921 | 5.835 | - | - | - |
| allocations | 76.94M | 25.64 | 2.778 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 76.93M | 25.64 | 2.778 | 1.000 | 2.001 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 23.18G | 7725 | 837.0 | 457.0 | 24.00 | 20.70G | 6786 | 747.3 |
| minor page faults | 733.3k | 0.244 | 0.0265 | 0.0272 | 0 | 712.0 | 0.000233 | 0.0000257 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.702M | 1.234 | 0.134 | 0.000405 | 0.0115 | - | - | - |
| involuntary context switches | 2102 | 0.000701 | 0.0000759 | 0.000271 | 0.0000202 | - | - | - |
| cycles | - | - | - | 52.34k | 3154 | - | - | - |
| instructions | - | - | - | 92.22k | 4542 | - | - | - |
| cache references | - | - | - | 42.91 | 29.92 | - | - | - |
| cache misses | - | - | - | 3.340 | 6.429 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00785 | 0.0677 | - | - | - |
| data TLB read misses | - | - | - | 0.195 | 0.269 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 39.73M | 13.02 | 1.434 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 4582 | 0.00150 | 0.000165 |
| garbage collections | - | - | - | - | - | 48.00 | 0.0000157 | 0.00000173 |
| collectors' pauses, ns | - | - | - | - | - | 87.92M | 28.82 | 3.174 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 467.3M | - | - | - | - | 5.091G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 431.0M | - | - | - | - | 5.099G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 196.1M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3050394 messages, 27700430 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3297204-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 107571 | 3013 | 3442467 | 0 | 107571000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 282.9 | 34.0 | 125.8 | 12.81 / 87.00 |
| best | kafka | 22 × wait | 107571 | 3013 | 3442467 | 0 | 107571000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 282.9 | 34.0 | 125.8 | 12.81 / 87.00 |

Run 1's profile:

| profile: the window, 30.0 s, 6457 messages, 6454891 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 145.3G | 22.51M | 22.51k | 4.383M | 4612 | 83.85G | 12.85M | 12.84k |
| waiting for a CPU, ns | - | - | - | 243.1k | 12.48k | - | - | - |
| allocations | 92.23M | 14.28k | 14.29 | 8281 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 92.24M | 14.29k | 14.29 | 8280 | 3.191 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 10.37G | 1.606M | 1607 | 207.4k | 24.00 | 28.41G | 4.352M | 4351 |
| minor page faults | 20.74k | 3.212 | 0.00321 | 0 | 0.000000465 | 7824 | 1.199 | 0.00120 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 5.153M | 798.1 | 0.798 | 0.00836 | 0.356 | - | - | - |
| involuntary context switches | 268.4k | 41.56 | 0.0416 | 2.882 | 0.00232 | - | - | - |
| cycles | - | - | - | 21.36M | 13.00k | - | - | - |
| instructions | - | - | - | 47.27M | 7802 | - | - | - |
| cache references | - | - | - | 2428 | 294.4 | - | - | - |
| cache misses | - | - | - | 762.0 | 75.56 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 38.51 | 6.896 | - | - | - |
| data TLB read misses | - | - | - | 21.49 | 17.19 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 10.50M | 1609 | 1.609 |
| bytes written to storage | 0 | 0 | 0 | - | - | 3.830M | 586.7 | 0.587 |
| storage reads | - | - | - | - | - | 641.0 | 0.0982 | 0.0000982 |
| storage writes | - | - | - | - | - | 389.0 | 0.0596 | 0.0000596 |
| garbage collections | - | - | - | - | - | 60.00 | 0.00919 | 0.00000919 |
| collectors' pauses, ns | - | - | - | - | - | 313.4M | 48.01k | 48.00 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 642.1M | - | - | - | - | 5.082G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 607.4M | - | - | - | - | 5.078G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 183.9M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 6528 messages, 6528678 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3317426-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 120.8 | 225.3 | 6553.6 | 22282.2 | 25984.7 | 0 | 0 | 1/1 | 85.6 | 35.2 | 29.4 | 20.03 / 17.17 |
| best | kafka | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 120.8 | 225.3 | 6553.6 | 22282.2 | 25984.7 | 0 | 0 | 1/1 | 85.6 | 35.2 | 29.4 | 20.03 / 17.17 |

Run 1's profile:

| profile: the window, 30.0 s, 299587 messages, 299583 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 71.82G | 239.7k | 239.7k | 99.21k | 97.81k | 24.88G | 81.68k | 81.68k |
| waiting for a CPU, ns | - | - | - | 615.0 | 185.9 | - | - | - |
| allocations | 6.651M | 22.20 | 22.20 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.651M | 22.20 | 22.20 | 1.000 | 4.379 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 1.602G | 5347 | 5348 | 457.0 | 24.00 | 6.810G | 22.36k | 22.36k |
| minor page faults | 83.00 | 0.000277 | 0.000277 | 0.00000334 | 0 | 215.0 | 0.000706 | 0.000706 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.247M | 4.164 | 4.164 | 0.000113 | 0.540 | - | - | - |
| involuntary context switches | 19.80k | 0.0661 | 0.0661 | 0.0361 | 0.00478 | - | - | - |
| cycles | - | - | - | 461.0k | 516.2k | - | - | - |
| instructions | - | - | - | 846.3k | 852.2k | - | - | - |
| cache references | - | - | - | 92.09 | 157.9 | - | - | - |
| cache misses | - | - | - | 15.53 | 15.63 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.282 | 0.517 | - | - | - |
| data TLB read misses | - | - | - | 0.0512 | 0.727 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 11.35M | 37.25 | 37.25 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 1167 | 0.00383 | 0.00383 |
| garbage collections | - | - | - | - | - | 14.00 | 0.0000460 | 0.0000460 |
| collectors' pauses, ns | - | - | - | - | - | 62.82M | 206.3 | 206.3 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 9.794M | - | - | - | - | 4.383G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 9.794M | - | - | - | - | 4.383G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 195.6M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304587 messages, 304584 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3366847-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 5502926.8 | 8120172.5 | 9663676.4 | 10334765.1 | 11485412.7 | 0 | 0 | 100/100 | 719.8 | 30.3 | 53.1 | 42.45 / 42.24 |
| best | kafka | 22 × busy | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 5502926.8 | 8120172.5 | 9663676.4 | 10334765.1 | 11485412.7 | 0 | 0 | 100/100 | 719.8 | 30.3 | 53.1 | 42.45 / 42.24 |

Run 1's profile:

| profile: the window, 30.0 s, 299604 messages, 29631600 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 521.2G | 1.740M | 17.59k | 85.71k | 15.33k | 42.09G | 138.2k | 1397 |
| waiting for a CPU, ns | - | - | - | 13.90k | 6765 | - | - | - |
| allocations | 71.10M | 237.3 | 2.399 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 71.02M | 237.1 | 2.397 | 1.000 | 2.106 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 20.80G | 69.42k | 701.9 | 457.0 | 24.00 | 12.64G | 41.49k | 419.3 |
| minor page faults | 855.9k | 2.857 | 0.0289 | 0.0784 | 0 | 2566 | 0.00842 | 0.0000851 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.652M | 5.515 | 0.0558 | 0.0000200 | 0.00655 | - | - | - |
| involuntary context switches | 1.206M | 4.026 | 0.0407 | 0.371 | 0.0330 | - | - | - |
| cycles | - | - | - | 334.2k | 71.14k | - | - | - |
| instructions | - | - | - | 536.5k | 105.2k | - | - | - |
| cache references | - | - | - | 25.64 | 32.02 | - | - | - |
| cache misses | - | - | - | 8.802 | 8.463 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.269 | 0.146 | - | - | - |
| data TLB read misses | - | - | - | 0.692 | 0.598 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 11.26M | 36.97 | 0.374 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 1070 | 0.00351 | 0.0000355 |
| garbage collections | - | - | - | - | - | 28.00 | 0.0000919 | 0.000000929 |
| collectors' pauses, ns | - | - | - | - | - | 217.2M | 713.2 | 7.209 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 414.4M | - | - | - | - | 4.482G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 367.8M | - | - | - | - | 4.478G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 217.4M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 304585 messages, 30135650 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3406863-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 107969 | 3040 | 246829 | 0 | 107969000 | 57818000 | 241765000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 1148.6 | 34.4 | 121.1 | 32.75 / 44.12 |
| best | kafka | 22 × busy | 107969 | 3040 | 246829 | 0 | 107969000 | 57818000 | 241765000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 1148.6 | 34.4 | 121.1 | 32.75 / 44.12 |

Run 1's profile:

| profile: the window, 30.0 s, 57725 messages, 7013890 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 590.5G | 10.23M | 84.19k | 503.1k | 68.03k | 81.83G | 1.359M | 11.52k |
| waiting for a CPU, ns | - | - | - | 14.51k | 24.90k | - | - | - |
| allocations | 83.00M | 1438 | 11.83 | 757.9 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 82.96M | 1437 | 11.83 | 756.9 | 3.074 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 10.35G | 179.2k | 1475 | 19.36k | 24.00 | 28.04G | 465.5k | 3947 |
| minor page faults | 7293 | 0.126 | 0.00104 | 0.0912 | 0 | 38.00 | 0.000631 | 0.00000535 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.328M | 57.65 | 0.474 | 0.00120 | 0.0912 | - | - | - |
| involuntary context switches | 1.439M | 24.92 | 0.205 | 0.163 | 0.180 | - | - | - |
| cycles | - | - | - | 2.298M | 317.5k | - | - | - |
| instructions | - | - | - | 4.129M | 457.3k | - | - | - |
| cache references | - | - | - | 135.8 | 179.8 | - | - | - |
| cache misses | - | - | - | 48.88 | 49.14 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 2.346 | 3.841 | - | - | - |
| data TLB read misses | - | - | - | 1.429 | 10.51 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 7.385M | 122.6 | 1.040 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 491.0 | 0.00815 | 0.0000691 |
| garbage collections | - | - | - | - | - | 56.00 | 0.000930 | 0.00000788 |
| collectors' pauses, ns | - | - | - | - | - | 241.7M | 4013 | 34.03 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 577.2M | - | - | - | - | 4.502G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 577.2M | - | - | - | - | 4.498G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 194.5M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 60238 messages, 7104018 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3422782-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × busy | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 13153337.3 | 17716740.1 | 18522046.5 | 18607769.2 | 18607769.2 | 0 | 0 | 1/1 | 110.3 | 35.4 | 40.8 | 10.11 / 6.39 |
| best | kafka | 1 × busy | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 13153337.3 | 17716740.1 | 18522046.5 | 18607769.2 | 18607769.2 | 0 | 0 | 1/1 | 110.3 | 35.4 | 40.8 | 10.11 / 6.39 |

Run 1's profile:

| profile: the window, 30.0 s, 3000444 messages, 1566324 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 77.14G | 25.71k | 49.25k | 9964 | 19.06k | 31.82G | 10.43k | 19.95k |
| waiting for a CPU, ns | - | - | - | 1.138 | 2.501 | - | - | - |
| allocations | 26.64M | 8.878 | 17.01 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 26.63M | 8.877 | 17.00 | 1.000 | 2.068 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 7.428G | 2475 | 4742 | 457.0 | 24.00 | 23.11G | 7577 | 14.49k |
| minor page faults | 107.0k | 0.0357 | 0.0683 | 0.0267 | 0 | 7026 | 0.00230 | 0.00441 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.781M | 1.260 | 2.414 | 0.000285 | 0.0215 | - | - | - |
| involuntary context switches | 747.0 | 0.000249 | 0.000477 | 0.0000833 | 0.000147 | - | - | - |
| cycles | - | - | - | 55.95k | 120.9k | - | - | - |
| instructions | - | - | - | 98.82k | 203.2k | - | - | - |
| cache references | - | - | - | 39.48 | 49.43 | - | - | - |
| cache misses | - | - | - | 1.602 | 0.724 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00422 | 0.0105 | - | - | - |
| data TLB read misses | - | - | - | 0.106 | 0.138 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 38.90M | 12.75 | 24.39 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 4500 | 0.00148 | 0.00282 |
| garbage collections | - | - | - | - | - | 45.00 | 0.0000148 | 0.0000282 |
| collectors' pauses, ns | - | - | - | - | - | 71.84M | 23.55 | 45.05 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 30.55M | - | - | - | - | 5.563G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 29.90M | - | - | - | - | 5.571G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 213.9M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3050396 messages, 1594807 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3449437-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 3550038 | 99998 | 0 | 0 | 115932324 | 65949424 | 234094276 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 234094276 | 0/100 | 1431.8 | 26.1 | 34.7 | 30.02 / 34.75 |
| best | kafka | 22 × busy | 3550038 | 99998 | 0 | 0 | 115932324 | 65949424 | 234094276 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 234094276 | 0/100 | 1431.8 | 26.1 | 34.7 | 30.02 / 34.75 |

Run 1's profile:

| profile: the window, 30.0 s, 3000564 messages, 59169887 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 640.3G | 213.4k | 10.82k | 7452 | 9975 | 23.08G | 7565 | 386.4 |
| waiting for a CPU, ns | - | - | - | 2513 | 1130 | - | - | - |
| allocations | 129.1M | 43.04 | 2.182 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 129.1M | 43.03 | 2.182 | 1.000 | 2.006 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 39.97G | 13.32k | 675.5 | 457.0 | 24.00 | 7.085G | 2323 | 118.6 |
| minor page faults | 543.7k | 0.181 | 0.00919 | 0.0207 | 0 | 453.0 | 0.000149 | 0.00000758 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.109M | 0.370 | 0.0188 | 0.0000160 | 0.00110 | - | - | - |
| involuntary context switches | 974.3k | 0.325 | 0.0165 | 0.142 | 0.00433 | - | - | - |
| cycles | - | - | - | 26.24k | 48.26k | - | - | - |
| instructions | - | - | - | 40.83k | 72.03k | - | - | - |
| cache references | - | - | - | 13.09 | 17.49 | - | - | - |
| cache misses | - | - | - | 3.637 | 3.247 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0306 | 0.0391 | - | - | - |
| data TLB read misses | - | - | - | 0.0580 | 0.0927 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 37.31M | 12.23 | 0.625 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 4503 | 0.00148 | 0.0000754 |
| garbage collections | - | - | - | - | - | 16.00 | 0.00000525 | 0.000000268 |
| collectors' pauses, ns | - | - | - | - | - | 129.2M | 42.34 | 2.163 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 470.3M | - | - | - | - | 6.542G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 457.3M | - | - | - | - | 6.548G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 216.1M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3050376 messages, 59723260 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3491181-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 3550038 | 99998 | 0 | 0 | 621030438 | 122092194 | 2878344806 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2878344806 | 0/1000 | 1563.4 | 28.5 | 56.1 | 26.59 / 33.74 |
| best | kafka | 22 × busy | 3550038 | 99998 | 0 | 0 | 621030438 | 122092194 | 2878344806 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2878344806 | 0/1000 | 1563.4 | 28.5 | 56.1 | 26.59 / 33.74 |

Run 1's profile:

| profile: the window, 30.0 s, 3000357 messages, 294577078 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 707.1G | 235.7k | 2400 | 7951 | 1956 | 31.76G | 10.41k | 106.1 |
| waiting for a CPU, ns | - | - | - | 2008 | 267.9 | - | - | - |
| allocations | 601.8M | 200.6 | 2.043 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 601.9M | 200.6 | 2.043 | 1.000 | 2.001 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 186.0G | 61.99k | 631.4 | 457.0 | 24.00 | 9.605G | 3148 | 32.10 |
| minor page faults | 1.479M | 0.493 | 0.00502 | 0.00000700 | 0 | 317.0 | 0.000104 | 0.00000106 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.455M | 0.485 | 0.00494 | 0.000120 | 0.000759 | - | - | - |
| involuntary context switches | 851.5k | 0.284 | 0.00289 | 0.0747 | 0.000981 | - | - | - |
| cycles | - | - | - | 34.33k | 10.81k | - | - | - |
| instructions | - | - | - | 52.80k | 15.90k | - | - | - |
| cache references | - | - | - | 33.80 | 22.19 | - | - | - |
| cache misses | - | - | - | 6.416 | 9.244 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.108 | 0.105 | - | - | - |
| data TLB read misses | - | - | - | 0.203 | 0.128 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 30.02M | 9.840 | 0.100 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 3474 | 0.00114 | 0.0000116 |
| garbage collections | - | - | - | - | - | 20.00 | 0.00000656 | 0.000000067 |
| collectors' pauses, ns | - | - | - | - | - | 75.26M | 24.67 | 0.252 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 2.050G | - | - | - | - | 7.543G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 2.038G | - | - | - | - | 7.552G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 203.7M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3050873 messages, 299214789 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3504746-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 180.2 | 4259.8 | 322961.4 | 343932.9 | 350256.9 | 0 | 0 | 1/1 | 50.4 | 32.8 | 30.0 | 17.09 / 28.71 |
| best | kafka | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 180.2 | 4259.8 | 322961.4 | 343932.9 | 350256.9 | 0 | 0 | 1/1 | 50.4 | 32.8 | 30.0 | 17.09 / 28.71 |

Run 1's profile:

| profile: the window, 30.0 s, 300767 messages, 297473 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 42.12G | 140.0k | 141.6k | 91.22k | 6604 | 25.68G | 83.97k | 84.85k |
| waiting for a CPU, ns | - | - | - | 8121 | 2898 | - | - | - |
| allocations | 6.060M | 20.15 | 20.37 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.052M | 20.12 | 20.35 | 1.000 | 3.885 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 1.480G | 4920 | 4975 | 457.0 | 24.00 | 6.050G | 19.78k | 19.99k |
| minor page faults | 1645 | 0.00547 | 0.00553 | 0.00123 | 0 | 538.0 | 0.00176 | 0.00178 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 31.00 | 0.000101 | 0.000102 |
| voluntary context switches | 1.269M | 4.220 | 4.266 | 0.000123 | 0.772 | - | - | - |
| involuntary context switches | 248.3k | 0.826 | 0.835 | 0.550 | 0.000965 | - | - | - |
| cycles | - | - | - | 422.1k | 16.46k | - | - | - |
| instructions | - | - | - | 747.8k | 9813 | - | - | - |
| cache references | - | - | - | 118.8 | 129.3 | - | - | - |
| cache misses | - | - | - | 38.26 | 32.15 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 1.519 | 3.433 | - | - | - |
| data TLB read misses | - | - | - | 0.172 | 2.718 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 2.724M | 8.906 | 9.000 |
| bytes written to storage | 0 | 0 | 0 | - | - | 7.426M | 24.28 | 24.54 |
| storage reads | - | - | - | - | - | 97.00 | 0.000317 | 0.000320 |
| storage writes | - | - | - | - | - | 762.0 | 0.00249 | 0.00252 |
| garbage collections | - | - | - | - | - | 13.00 | 0.0000425 | 0.0000430 |
| collectors' pauses, ns | - | - | - | - | - | 131.8M | 430.9 | 435.5 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 15.36M | - | - | - | - | 1.988G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 15.36M | - | - | - | - | 1.989G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 202.3M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305836 messages, 302657 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3551516-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 4093640.7 | 7583301.6 | 11945377.8 | 14629732.4 | 15615051.9 | 0 | 0 | 100/100 | 150.4 | 34.9 | 86.1 | 25.27 / 18.77 |
| best | kafka | 22 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 4093640.7 | 7583301.6 | 11945377.8 | 14629732.4 | 15615051.9 | 0 | 0 | 100/100 | 150.4 | 34.9 | 86.1 | 25.27 / 18.77 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 29885763 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 118.4G | 393.6k | 3961 | 97.69k | 972.8 | 66.88G | 218.7k | 2230 |
| waiting for a CPU, ns | - | - | - | 1620 | 277.6 | - | - | - |
| allocations | 77.59M | 258.0 | 2.596 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 77.62M | 258.1 | 2.597 | 1.000 | 2.152 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 22.66G | 75.35k | 758.3 | 457.0 | 24.00 | 19.82G | 64.80k | 660.9 |
| minor page faults | 889.8k | 2.958 | 0.0298 | 0.0348 | 0 | 112.0 | 0.000366 | 0.00000374 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 3.000 | 0.00000981 | 0.000000100 |
| voluntary context switches | 4.044M | 13.45 | 0.135 | 0.0000499 | 0.0557 | - | - | - |
| involuntary context switches | 94.82k | 0.315 | 0.00317 | 0.0818 | 0.000226 | - | - | - |
| cycles | - | - | - | 436.2k | 4300 | - | - | - |
| instructions | - | - | - | 787.6k | 5186 | - | - | - |
| cache references | - | - | - | 92.83 | 42.25 | - | - | - |
| cache misses | - | - | - | 18.13 | 13.70 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.735 | 0.541 | - | - | - |
| data TLB read misses | - | - | - | 0.666 | 1.397 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 12.29k | 0.0402 | 0.000410 |
| bytes written to storage | 0 | 0 | 0 | - | - | 11.27M | 36.86 | 0.376 |
| storage reads | - | - | - | - | - | 3.000 | 0.00000981 | 0.000000100 |
| storage writes | - | - | - | - | - | 1079 | 0.00353 | 0.0000360 |
| garbage collections | - | - | - | - | - | 43.00 | 0.000141 | 0.00000143 |
| collectors' pauses, ns | - | - | - | - | - | 171.0M | 559.2 | 5.704 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 425.9M | - | - | - | - | 2.026G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 372.1M | - | - | - | - | 2.027G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 193.1M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305834 messages, 29985832 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3587893-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 104108 | 2932 | 251993 | 0 | 104108000 | 53704000 | 247065000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 364.7 | 34.3 | 190.9 | 16.82 / 22.29 |
| best | kafka | 22 × wait | 104108 | 2932 | 251993 | 0 | 104108000 | 53704000 | 247065000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 364.7 | 34.3 | 190.9 | 16.82 / 22.29 |

Run 1's profile:

| profile: the window, 30.0 s, 53648 messages, 3485096 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 149.4G | 2.786M | 42.88k | 552.1k | 8801 | 91.05G | 1.620M | 25.77k |
| waiting for a CPU, ns | - | - | - | 4762 | 2965 | - | - | - |
| allocations | 92.34M | 1721 | 26.50 | 1086 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 92.30M | 1720 | 26.48 | 1085 | 4.327 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 9.000G | 167.8k | 2582 | 27.55k | 24.00 | 30.01G | 533.8k | 8492 |
| minor page faults | 5885 | 0.110 | 0.00169 | 0.0993 | 0 | 646.0 | 0.0115 | 0.000183 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 5.689M | 106.0 | 1.632 | 0.00300 | 0.760 | - | - | - |
| involuntary context switches | 85.74k | 1.598 | 0.0246 | 0.0963 | 0.00247 | - | - | - |
| cycles | - | - | - | 2.651M | 23.52k | - | - | - |
| instructions | - | - | - | 5.914M | 11.99k | - | - | - |
| cache references | - | - | - | 388.1 | 550.7 | - | - | - |
| cache misses | - | - | - | 111.7 | 160.1 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 3.665 | 15.00 | - | - | - |
| data TLB read misses | - | - | - | 2.159 | 34.60 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 7.533M | 134.0 | 2.132 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 492.0 | 0.00875 | 0.000139 |
| garbage collections | - | - | - | - | - | 64.00 | 0.00114 | 0.0000181 |
| collectors' pauses, ns | - | - | - | - | - | 228.2M | 4059 | 64.57 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 585.2M | - | - | - | - | 1.861G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 585.2M | - | - | - | - | 1.861G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 193.5M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 56219 messages, 3533755 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3595812-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × wait | 3554018 | 100110 | 0 | 0 | 2871414 | 2370118 | 632513 | 27917287.4 | undelivered | undelivered | undelivered | undelivered | 0 | 632513 | 0/1 | 60.5 | 35.3 | 38.3 | 9.36 / 7.41 |
| best | kafka | 1 × wait | 3554018 | 100110 | 0 | 0 | 2871414 | 2370118 | 632513 | 27917287.4 | undelivered | undelivered | undelivered | undelivered | 0 | 632513 | 0/1 | 60.5 | 35.3 | 38.3 | 9.36 / 7.41 |

Run 1's profile:

| profile: the window, 30.0 s, 3002626 messages, 905097 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 48.98G | 16.31k | 54.12k | 9950 | 702.6 | 30.43G | 9970 | 33.63k |
| waiting for a CPU, ns | - | - | - | 1.949 | 4.113 | - | - | - |
| allocations | 22.72M | 7.568 | 25.11 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 22.72M | 7.567 | 25.10 | 1.000 | 2.001 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 6.455G | 2150 | 7131 | 457.0 | 24.00 | 19.87G | 6509 | 21.95k |
| minor page faults | 127.2k | 0.0424 | 0.141 | 0.0306 | 0 | 440.0 | 0.000144 | 0.000486 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 1.000 | 0.000000328 | 0.00000110 |
| voluntary context switches | 3.053M | 1.017 | 3.373 | 0.000430 | 0.0212 | - | - | - |
| involuntary context switches | 633.0 | 0.000211 | 0.000699 | 0.0000633 | 0.0000265 | - | - | - |
| cycles | - | - | - | 51.11k | 3725 | - | - | - |
| instructions | - | - | - | 90.69k | 4681 | - | - | - |
| cache references | - | - | - | 40.04 | 40.30 | - | - | - |
| cache misses | - | - | - | 2.644 | 1.316 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00812 | 0.00946 | - | - | - |
| data TLB read misses | - | - | - | 0.0731 | 0.361 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 4096 | 0.00134 | 0.00453 |
| bytes written to storage | 0 | 0 | 0 | - | - | 29.74M | 9.742 | 32.85 |
| storage reads | - | - | - | - | - | 1.000 | 0.000000328 | 0.00000110 |
| storage writes | - | - | - | - | - | 3361 | 0.00110 | 0.00371 |
| garbage collections | - | - | - | - | - | 35.00 | 0.0000115 | 0.0000387 |
| collectors' pauses, ns | - | - | - | - | - | 68.61M | 22.48 | 75.80 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 30.89M | - | - | - | - | 2.916G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 25.01M | - | - | - | - | 2.925G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 206.5M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3052585 messages, 905097 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3619159-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 3554018 | 100110 | 0 | 0 | 69426618 | 19344247 | 280918853 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 280918853 | 0/100 | 135.1 | 35.2 | 50.8 | 33.73 / 16.51 |
| best | kafka | 22 × wait | 3554018 | 100110 | 0 | 0 | 69426618 | 19344247 | 280918853 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 280918853 | 0/100 | 135.1 | 35.2 | 50.8 | 33.73 / 16.51 |

Run 1's profile:

| profile: the window, 30.0 s, 3395599 messages, 29923700 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 76.47G | 22.52k | 2555 | 8765 | 533.6 | 32.08G | 9378 | 1071 |
| waiting for a CPU, ns | - | - | - | 37.63 | 54.27 | - | - | - |
| allocations | 79.07M | 23.28 | 2.642 | 2.204 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 79.16M | 23.31 | 2.645 | 1.204 | 2.003 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 23.80G | 7010 | 795.4 | 462.1 | 24.00 | 16.21G | 4737 | 541.0 |
| minor page faults | 1.009M | 0.297 | 0.0337 | 0 | 0 | 522.0 | 0.000153 | 0.0000174 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 83.00 | 0.0000243 | 0.00000277 |
| voluntary context switches | 2.777M | 0.818 | 0.0928 | 0.00110 | 0.0113 | - | - | - |
| involuntary context switches | 18.63k | 0.00549 | 0.000623 | 0.00227 | 0.0000663 | - | - | - |
| cycles | - | - | - | 42.03k | 3052 | - | - | - |
| instructions | - | - | - | 73.93k | 4329 | - | - | - |
| cache references | - | - | - | 40.01 | 27.90 | - | - | - |
| cache misses | - | - | - | 7.330 | 6.237 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0375 | 0.132 | - | - | - |
| data TLB read misses | - | - | - | 0.168 | 0.265 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 327.7k | 0.0958 | 0.0109 |
| bytes written to storage | 0 | 0 | 0 | - | - | 39.74M | 11.61 | 1.326 |
| storage reads | - | - | - | - | - | 80.00 | 0.0000234 | 0.00000267 |
| storage writes | - | - | - | - | - | 4824 | 0.00141 | 0.000161 |
| garbage collections | - | - | - | - | - | 42.00 | 0.0000123 | 0.00000140 |
| collectors' pauses, ns | - | - | - | - | - | 118.9M | 34.74 | 3.967 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 447.0M | - | - | - | - | 3.443G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 418.7M | - | - | - | - | 3.448G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 193.0M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3421086 messages, 29959500 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3664795-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × wait | 106612 | 2986 | 3447406 | 0 | 106612000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 327.8 | 35.6 | 154.0 | 14.88 / 19.44 |
| best | kafka | 22 × wait | 106612 | 2986 | 3447406 | 0 | 106612000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 1000/1000 | 327.8 | 35.6 | 154.0 | 14.88 / 19.44 |

Run 1's profile:

| profile: the window, 30.0 s, 5340 messages, 5342289 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 159.8G | 29.93M | 29.92k | 5.590M | 6383 | 92.94G | 17.09M | 17.10k |
| waiting for a CPU, ns | - | - | - | 1294 | 669.0 | - | - | - |
| allocations | 107.2M | 20.07k | 20.07 | 12.41k | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 107.2M | 20.08k | 20.07 | 12.40k | 3.696 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 10.91G | 2.044M | 2043 | 310.6k | 24.00 | 33.24G | 6.113M | 6115 |
| minor page faults | 437.0 | 0.0818 | 0.0000818 | 0.000375 | 0.00000468 | 34.00 | 0.00625 | 0.00000625 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 3.000 | 0.000552 | 0.000000552 |
| voluntary context switches | 6.329M | 1185 | 1.185 | 0.0223 | 0.556 | - | - | - |
| involuntary context switches | 62.07k | 11.62 | 0.0116 | 0.0573 | 0.00173 | - | - | - |
| cycles | - | - | - | 27.91M | 17.98k | - | - | - |
| instructions | - | - | - | 63.85M | 9931 | - | - | - |
| cache references | - | - | - | 3161 | 408.6 | - | - | - |
| cache misses | - | - | - | 928.0 | 111.4 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 34.55 | 10.50 | - | - | - |
| data TLB read misses | - | - | - | 50.27 | 25.91 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 12.29k | 2.260 | 0.00226 |
| bytes written to storage | 0 | 0 | 0 | - | - | 6.873M | 1264 | 1.264 |
| storage reads | - | - | - | - | - | 3.000 | 0.000552 | 0.000000552 |
| storage writes | - | - | - | - | - | 445.0 | 0.0818 | 0.0000819 |
| garbage collections | - | - | - | - | - | 69.00 | 0.0127 | 0.0000127 |
| collectors' pauses, ns | - | - | - | - | - | 195.6M | 35.96k | 35.97 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 595.9M | - | - | - | - | 3.483G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 595.9M | - | - | - | - | 3.480G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 193.7M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 5438 messages, 5435933 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3675505-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 73.7 | 147.5 | 299.0 | 2195.5 | 7029.4 | 0 | 0 | 1/1 | 84.0 | 35.4 | 21.7 | 12.89 / 9.24 |
| best | kafka | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 73.7 | 147.5 | 299.0 | 2195.5 | 7029.4 | 0 | 0 | 1/1 | 84.0 | 35.4 | 21.7 | 12.89 / 9.24 |

Run 1's profile:

| profile: the window, 30.0 s, 300768 messages, 300769 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 70.38G | 234.0k | 234.0k | 99.50k | 98.15k | 17.60G | 57.54k | 57.54k |
| waiting for a CPU, ns | - | - | - | 5.421 | 21.32 | - | - | - |
| allocations | 6.947M | 23.10 | 23.10 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.947M | 23.10 | 23.10 | 1.000 | 4.563 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 1.664G | 5531 | 5531 | 457.0 | 24.00 | 7.237G | 23.66k | 23.66k |
| minor page faults | 21.00 | 0.0000698 | 0.0000698 | 0 | 0 | 2655 | 0.00868 | 0.00868 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 3.000 | 0.00000981 | 0.00000981 |
| voluntary context switches | 1.246M | 4.144 | 4.144 | 0.000126 | 0.572 | - | - | - |
| involuntary context switches | 654.0 | 0.00217 | 0.00217 | 0.000572 | 0.000658 | - | - | - |
| cycles | - | - | - | 537.2k | 606.9k | - | - | - |
| instructions | - | - | - | 991.2k | 1.021M | - | - | - |
| cache references | - | - | - | 66.30 | 161.5 | - | - | - |
| cache misses | - | - | - | 5.489 | 6.264 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0875 | 0.0928 | - | - | - |
| data TLB read misses | - | - | - | 0.265 | 0.801 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 348.2k | 1.138 | 1.138 |
| bytes written to storage | 0 | 0 | 0 | - | - | 11.43M | 37.38 | 37.38 |
| storage reads | - | - | - | - | - | 19.00 | 0.0000621 | 0.0000621 |
| storage writes | - | - | - | - | - | 1178 | 0.00385 | 0.00385 |
| garbage collections | - | - | - | - | - | 15.00 | 0.0000490 | 0.0000490 |
| collectors' pauses, ns | - | - | - | - | - | 35.00M | 114.4 | 114.4 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 9.609M | - | - | - | - | 3.624G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 9.609M | - | - | - | - | 3.625G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 206.7M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305834 messages, 305836 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3697015-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 1375731.7 | 2483028.0 | 3355443.2 | 4294967.3 | 5137382.5 | 0 | 0 | 100/100 | 860.6 | 34.1 | 105.8 | 20.55 / 30.15 |
| best | kafka | 22 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 1375731.7 | 2483028.0 | 3355443.2 | 4294967.3 | 5137382.5 | 0 | 0 | 100/100 | 860.6 | 34.1 | 105.8 | 20.55 / 30.15 |

Run 1's profile:

| profile: the window, 30.0 s, 300766 messages, 32621366 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 675.4G | 2.245M | 20.70k | 95.43k | 17.68k | 83.39G | 272.7k | 2547 |
| waiting for a CPU, ns | - | - | - | 3932 | 2213 | - | - | - |
| allocations | 91.12M | 302.9 | 2.793 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 91.14M | 303.0 | 2.794 | 1.000 | 2.224 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 25.39G | 84.40k | 778.2 | 457.0 | 24.00 | 28.12G | 91.94k | 858.9 |
| minor page faults | 814.6k | 2.708 | 0.0250 | 0.0371 | 0.000000399 | 1125 | 0.00368 | 0.0000344 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 4.057M | 13.49 | 0.124 | 0.0000133 | 0.0320 | - | - | - |
| involuntary context switches | 1.290M | 4.289 | 0.0395 | 0.290 | 0.0315 | - | - | - |
| cycles | - | - | - | 395.7k | 88.56k | - | - | - |
| instructions | - | - | - | 637.8k | 129.6k | - | - | - |
| cache references | - | - | - | 83.41 | 39.09 | - | - | - |
| cache misses | - | - | - | 22.18 | 5.307 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.974 | 0.0883 | - | - | - |
| data TLB read misses | - | - | - | 1.935 | 1.074 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 12.00M | 39.24 | 0.367 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 1100 | 0.00360 | 0.0000336 |
| garbage collections | - | - | - | - | - | 62.00 | 0.000203 | 0.00000189 |
| collectors' pauses, ns | - | - | - | - | - | 134.4M | 439.6 | 4.106 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 395.6M | - | - | - | - | 3.720G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 193.9M | - | - | - | - | 3.717G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 190.8M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 305834 messages, 32738742 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3707164-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 104248 | 2935 | 251853 | 0 | 104248000 | 53844000 | 246925000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 1541.9 | 33.3 | 176.6 | 35.44 / 39.91 |
| best | kafka | 22 × busy | 104248 | 2935 | 251853 | 0 | 104248000 | 53844000 | 246925000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 0 | 0/1000 | 1541.9 | 33.3 | 176.6 | 35.44 / 39.91 |

Run 1's profile:

| profile: the window, 30.0 s, 53761 messages, 3665608 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 607.9G | 11.31M | 165.8k | 535.4k | 136.4k | 82.55G | 1.465M | 22.20k |
| waiting for a CPU, ns | - | - | - | 20.22k | 41.51k | - | - | - |
| allocations | 77.66M | 1445 | 21.19 | 843.1 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 77.62M | 1444 | 21.17 | 842.1 | 4.038 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 8.356G | 155.4k | 2279 | 21.48k | 24.00 | 27.87G | 494.6k | 7493 |
| minor page faults | 6362 | 0.118 | 0.00174 | 0.0964 | 0 | 105.0 | 0.00186 | 0.0000282 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 3.290M | 61.19 | 0.897 | 0.00106 | 0.167 | - | - | - |
| involuntary context switches | 1.523M | 28.33 | 0.415 | 0.278 | 0.370 | - | - | - |
| cycles | - | - | - | 2.405M | 636.6k | - | - | - |
| instructions | - | - | - | 4.389M | 921.2k | - | - | - |
| cache references | - | - | - | 112.1 | 308.1 | - | - | - |
| cache misses | - | - | - | 42.60 | 83.50 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 1.896 | 6.783 | - | - | - |
| data TLB read misses | - | - | - | 1.625 | 19.49 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 5.779M | 102.6 | 1.554 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 397.0 | 0.00705 | 0.000107 |
| garbage collections | - | - | - | - | - | 57.00 | 0.00101 | 0.0000153 |
| collectors' pauses, ns | - | - | - | - | - | 230.9M | 4098 | 62.08 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 584.4M | - | - | - | - | 3.681G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 584.4M | - | - | - | - | 3.680G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 194.2M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 56337 messages, 3718827 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3729747-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × busy | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 897581.1 | 6174015.5 | 8053063.7 | 8254390.3 | 8276726.9 | 0 | 0 | 1/1 | 102.7 | 34.8 | 43.5 | 11.42 / 16.89 |
| best | kafka | 1 × busy | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 897581.1 | 6174015.5 | 8053063.7 | 8254390.3 | 8276726.9 | 0 | 0 | 1/1 | 102.7 | 34.8 | 43.5 | 11.42 / 16.89 |

Run 1's profile:

| profile: the window, 30.0 s, 3002619 messages, 2923902 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 79.10G | 26.34k | 27.05k | 9778 | 10.05k | 33.45G | 10.96k | 11.33k |
| waiting for a CPU, ns | - | - | - | 170.2 | 31.24 | - | - | - |
| allocations | 25.46M | 8.481 | 8.709 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 25.46M | 8.479 | 8.707 | 1.000 | 2.229 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 7.339G | 2444 | 2510 | 457.0 | 24.00 | 17.86G | 5852 | 6052 |
| minor page faults | 197.5k | 0.0658 | 0.0675 | 0.0411 | 0 | 466.0 | 0.000153 | 0.000158 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 2.770M | 0.922 | 0.947 | 0.000280 | 0.0591 | - | - | - |
| involuntary context switches | 67.48k | 0.0225 | 0.0231 | 0.0167 | 0.000350 | - | - | - |
| cycles | - | - | - | 49.20k | 57.43k | - | - | - |
| instructions | - | - | - | 86.97k | 95.63k | - | - | - |
| cache references | - | - | - | 34.27 | 31.31 | - | - | - |
| cache misses | - | - | - | 4.396 | 2.435 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0185 | 0.0203 | - | - | - |
| data TLB read misses | - | - | - | 0.105 | 0.141 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 0 | 0 | 0 |
| bytes written to storage | 0 | 0 | 0 | - | - | 40.97M | 13.42 | 13.88 |
| storage reads | - | - | - | - | - | 0 | 0 | 0 |
| storage writes | - | - | - | - | - | 4928 | 0.00161 | 0.00167 |
| garbage collections | - | - | - | - | - | 43.00 | 0.0000141 | 0.0000146 |
| collectors' pauses, ns | - | - | - | - | - | 101.7M | 33.32 | 34.45 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 66.04M | - | - | - | - | 4.463G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 27.83M | - | - | - | - | 4.465G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 223.6M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3052566 messages, 2951754 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3755866-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 3553954 | 100109 | 64 | 0 | 155265449 | 105085822 | 195177278 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 195177278 | 1/100 | 1411.2 | 28.8 | 49.2 | 35.79 / 47.15 |
| best | kafka | 22 × busy | 3553954 | 100109 | 64 | 0 | 155265449 | 105085822 | 195177278 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 195177278 | 1/100 | 1411.2 | 28.8 | 49.2 | 35.79 / 47.15 |

Run 1's profile:

| profile: the window, 30.0 s, 3002695 messages, 50211325 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.4G | 229.2k | 13.71k | 8140 | 12.58k | 34.79G | 11.40k | 677.1 |
| waiting for a CPU, ns | - | - | - | 1815 | 481.6 | - | - | - |
| allocations | 114.6M | 38.17 | 2.282 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 114.5M | 38.15 | 2.281 | 1.000 | 2.017 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 34.81G | 11.59k | 693.3 | 457.0 | 24.00 | 10.92G | 3578 | 212.6 |
| minor page faults | 580.4k | 0.193 | 0.0116 | 0.00217 | 0.00000155 | 225.0 | 0.0000737 | 0.00000438 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| voluntary context switches | 1.671M | 0.557 | 0.0333 | 0.0000979 | 0.00378 | - | - | - |
| involuntary context switches | 921.5k | 0.307 | 0.0184 | 0.104 | 0.00432 | - | - | - |
| cycles | - | - | - | 32.55k | 62.72k | - | - | - |
| instructions | - | - | - | 50.77k | 92.87k | - | - | - |
| cache references | - | - | - | 24.34 | 22.09 | - | - | - |
| cache misses | - | - | - | 4.167 | 3.146 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0366 | 0.0463 | - | - | - |
| data TLB read misses | - | - | - | 0.202 | 0.165 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 1.196M | 0.392 | 0.0233 |
| bytes written to storage | 0 | 0 | 0 | - | - | 90.08M | 29.51 | 1.753 |
| storage reads | - | - | - | - | - | 73.00 | 0.0000239 | 0.00000142 |
| storage writes | - | - | - | - | - | 8145 | 0.00267 | 0.000159 |
| garbage collections | - | - | - | - | - | 28.00 | 0.00000917 | 0.000000545 |
| collectors' pauses, ns | - | - | - | - | - | 125.1M | 40.97 | 2.434 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 450.1M | - | - | - | - | 4.237G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 440.1M | - | - | - | - | 4.237G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 200.9M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3052665 messages, 51386265 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3792541-0-kafka.jfr
# quiver-bench fanout kafka

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 22 × busy | 3554018 | 100110 | 0 | 0 | 625110658 | 124054565 | 2878576435 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2878576435 | 0/1000 | 1445.4 | 25.0 | 64.9 | 29.53 / 37.63 |
| best | kafka | 22 × busy | 3554018 | 100110 | 0 | 0 | 625110658 | 124054565 | 2878576435 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2878576435 | 0/1000 | 1445.4 | 25.0 | 64.9 | 29.53 / 37.63 |

Run 1's profile:

| profile: the window, 30.0 s, 3003092 messages, 295955060 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 675.4G | 224.9k | 2282 | 6891 | 1892 | 33.40G | 10.94k | 110.7 |
| waiting for a CPU, ns | - | - | - | 3061 | 321.2 | - | - | - |
| allocations | 603.2M | 200.8 | 2.038 | 2.000 | 1.000 | n/a: the JVM counts bytes, not objects (its allocation samples are in the recording) | n/a | n/a |
| reallocations | 0 | 0 | 0 | 0 | 0 | n/a: the JVM counts bytes, not objects | n/a | n/a |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 603.1M | 200.8 | 2.038 | 1.000 | 2.001 | n/a: a collector frees, uncounted | n/a | n/a |
| bytes allocated | 184.3G | 61.38k | 622.8 | 457.0 | 24.00 | 7.608G | 2492 | 25.22 |
| minor page faults | 2.531M | 0.843 | 0.00855 | 0.00105 | 0.000000003 | 4043 | 0.00132 | 0.0000134 |
| major page faults | 0 | 0 | 0 | 0 | 0 | 14.00 | 0.00000459 | 0.000000046 |
| voluntary context switches | 1.378M | 0.459 | 0.00466 | 0.0000423 | 0.000544 | - | - | - |
| involuntary context switches | 982.2k | 0.327 | 0.00332 | 0.139 | 0.00129 | - | - | - |
| cycles | - | - | - | 25.92k | 9911 | - | - | - |
| instructions | - | - | - | 39.47k | 14.65k | - | - | - |
| cache references | - | - | - | 26.23 | 21.57 | - | - | - |
| cache misses | - | - | - | 8.091 | 9.458 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.0972 | 0.106 | - | - | - |
| data TLB read misses | - | - | - | 0.139 | 0.131 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | 5.026M | 1.646 | 0.0167 |
| bytes written to storage | 0 | 0 | 0 | - | - | 54.56M | 17.87 | 0.181 |
| storage reads | - | - | - | - | - | 217.0 | 0.0000711 | 0.000000719 |
| storage writes | - | - | - | - | - | 5566 | 0.00182 | 0.0000185 |
| garbage collections | - | - | - | - | - | 16.00 | 0.00000524 | 0.000000053 |
| collectors' pauses, ns | - | - | - | - | - | 91.19M | 29.87 | 0.302 |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 1.948G | - | - | - | - | 4.522G | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 1.707G | - | - | - | - | 4.465G | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | 219.7M | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the broker's span: 30.50 s from a quarter second before the window to a quarter second after it, 3052583 messages, 301664278 deliveries
- saved: /home/adalundhe/.cache/quiver-bench/quiet-1024-0915/comparison/profiles/kafka-3828356-0-kafka.jfr
