- broker: apache/kafka:4.1.0@sha256:bff074a5d0051dbc0bbbcd25b045bb1fe84833ec0d3c7c965d1797dd289ec88f, CPUs 4-7,20-23
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 266338.3 | 1929379.8 | 2516582.4 | 2717909.0 | 2721366.0 | 0 | 0 | 1/1 | 36.6 | 31.6 | 20.4 | 47.94 / 60.72 |
| best | kafka | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 266338.3 | 1929379.8 | 2516582.4 | 2717909.0 | 2721366.0 | 0 | 0 | 1/1 | 36.6 | 31.6 | 20.4 | 47.94 / 60.72 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 3992977.4 | 6241124.4 | 8120172.5 | 8992587.8 | 9489939.8 | 0 | 0 | 100/100 | 63.2 | 21.6 | 31.1 | 90.66 / 87.74 |
| best | kafka | 14 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 3992977.4 | 6241124.4 | 8120172.5 | 8992587.8 | 9489939.8 | 0 | 0 | 100/100 | 63.2 | 21.6 | 31.1 | 90.66 / 87.74 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × wait | 105764 | 2978 | 248826 | 0 | 105764000 | 55700000 | 243797000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 243797000 | 1000/1000 | 230.8 | 35.3 | 138.2 | 49.85 / 47.17 |
| best | kafka | 14 × wait | 105764 | 2978 | 248826 | 0 | 105764000 | 55700000 | 243797000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 243797000 | 1000/1000 | 230.8 | 35.3 | 138.2 | 49.85 / 47.17 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × wait | 3547497 | 99921 | 46 | 0 | 2508651 | 2009280 | 988994 | 26306674.7 | undelivered | undelivered | undelivered | undelivered | 0 | 988994 | 0/1 | 30.4 | 25.6 | 9.3 | 50.94 / 71.70 |
| best | kafka | 1 × wait | 3547497 | 99921 | 46 | 0 | 2508651 | 2009280 | 988994 | 26306674.7 | undelivered | undelivered | undelivered | undelivered | 0 | 988994 | 0/1 | 30.4 | 25.6 | 9.3 | 50.94 / 71.70 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × wait | 3547543 | 99928 | 0 | 0 | 71165309 | 21228209 | 278599191 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 278599191 | 0/100 | 73.6 | 32.6 | 16.9 | 77.48 / 60.88 |
| best | kafka | 14 × wait | 3547543 | 99928 | 0 | 0 | 71165309 | 21228209 | 278599191 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 278599191 | 0/100 | 73.6 | 32.6 | 16.9 | 77.48 / 60.88 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × wait | 3547543 | 99928 | 0 | 0 | 538555629 | 43729881 | 2954544119 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2954544119 | 0/1000 | 407.5 | 25.3 | 34.0 | 77.51 / 118.83 |
| best | kafka | 14 × wait | 3547543 | 99928 | 0 | 0 | 538555629 | 43729881 | 2954544119 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2954544119 | 0/1000 | 407.5 | 25.3 | 34.0 | 77.51 / 118.83 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 696.3 | 5898.2 | 603979.8 | 713031.7 | 716963.2 | 0 | 0 | 1/1 | 73.0 | 33.0 | 15.9 | 81.82 / 75.41 |
| best | kafka | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 696.3 | 5898.2 | 603979.8 | 713031.7 | 716963.2 | 0 | 0 | 1/1 | 73.0 | 33.0 | 15.9 | 81.82 / 75.41 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × busy | 354588 | 9988 | 2 | 0 | 35458800 | 29949700 | 0 | 3556769.8 | 6509559.8 | 8724152.3 | 9797894.1 | 10233898.7 | 0 | 0 | 100/100 | 507.0 | 29.2 | 39.3 | 106.47 / 99.15 |
| best | kafka | 14 × busy | 354588 | 9988 | 2 | 0 | 35458800 | 29949700 | 0 | 3556769.8 | 6509559.8 | 8724152.3 | 9797894.1 | 10233898.7 | 0 | 0 | 100/100 | 507.0 | 29.2 | 39.3 | 106.47 / 99.15 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × busy | 354590 | 9988 | 0 | 0 | 315104186 | 263569016 | 35927984 | 33285996.5 | undelivered | undelivered | undelivered | undelivered | 0 | 35927984 | 286/1000 | 855.8 | 29.1 | 87.5 | 84.32 / 75.11 |
| best | kafka | 14 × busy | 354590 | 9988 | 0 | 0 | 315104186 | 263569016 | 35927984 | 33285996.5 | undelivered | undelivered | undelivered | undelivered | 0 | 35927984 | 286/1000 | 855.8 | 29.1 | 87.5 | 84.32 / 75.11 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × busy | 3547497 | 99927 | 46 | 0 | 1863704 | 1364333 | 1633941 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 1633941 | 0/1 | 103.4 | 32.6 | 17.4 | 86.43 / 66.83 |
| best | kafka | 1 × busy | 3547497 | 99927 | 46 | 0 | 1863704 | 1364333 | 1633941 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 1633941 | 0/1 | 103.4 | 32.6 | 17.4 | 86.43 / 66.83 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × busy | 3547543 | 99928 | 0 | 0 | 96277048 | 46339948 | 253487452 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 253487452 | 0/100 | 871.0 | 28.3 | 16.5 | 63.73 / 79.03 |
| best | kafka | 14 × busy | 3547543 | 99928 | 0 | 0 | 96277048 | 46339948 | 253487452 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 253487452 | 0/100 | 871.0 | 28.3 | 16.5 | 63.73 / 79.03 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × busy | 3547543 | 99928 | 0 | 0 | 517747320 | 38770018 | 2959503982 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2959503982 | 0/1000 | 797.5 | 24.7 | 49.5 | 76.01 / 74.50 |
| best | kafka | 14 × busy | 3547543 | 99928 | 0 | 0 | 517747320 | 38770018 | 2959503982 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2959503982 | 0/1000 | 797.5 | 24.7 | 49.5 | 76.01 / 74.50 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 170.0 | 61866.0 | 503316.5 | 587202.6 | 597437.6 | 0 | 0 | 1/1 | 44.4 | 34.7 | 18.9 | 56.16 / 50.53 |
| best | kafka | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 170.0 | 61866.0 | 503316.5 | 587202.6 | 597437.6 | 0 | 0 | 1/1 | 44.4 | 34.7 | 18.9 | 56.16 / 50.53 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 4697620.5 | 10200547.3 | 15166603.3 | 17716740.1 | 18904108.5 | 0 | 0 | 100/100 | 103.9 | 35.3 | 71.3 | 26.43 / 30.15 |
| best | kafka | 14 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 4697620.5 | 10200547.3 | 15166603.3 | 17716740.1 | 18904108.5 | 0 | 0 | 100/100 | 103.9 | 35.3 | 71.3 | 26.43 / 30.15 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × wait | 106084 | 2987 | 248714 | 0 | 106084000 | 55933000 | 243650000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 243650000 | 1000/1000 | 205.3 | 35.3 | 127.8 | 33.61 / 25.82 |
| best | kafka | 14 × wait | 106084 | 2987 | 248714 | 0 | 106084000 | 55933000 | 243650000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 243650000 | 1000/1000 | 205.3 | 35.3 | 127.8 | 33.61 / 25.82 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 22548578.3 | 26038239.2 | 27648852.0 | 27797983.9 | 27797983.9 | 0 | 0 | 1/1 | 44.2 | 32.0 | 17.8 | 49.69 / 55.93 |
| best | kafka | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 22548578.3 | 26038239.2 | 27648852.0 | 27797983.9 | 27797983.9 | 0 | 0 | 1/1 | 44.2 | 32.0 | 17.8 | 49.69 / 55.93 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × wait | 3550038 | 99998 | 0 | 0 | 105135284 | 55152384 | 244891316 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 244891316 | 0/100 | 111.2 | 33.4 | 29.3 | 71.61 / 63.17 |
| best | kafka | 14 × wait | 3550038 | 99998 | 0 | 0 | 105135284 | 55152384 | 244891316 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 244891316 | 0/100 | 111.2 | 33.4 | 29.3 | 71.61 / 63.17 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × wait | 3550038 | 99998 | 0 | 0 | 541017102 | 45833472 | 2954603528 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2954603528 | 0/1000 | 446.1 | 30.3 | 48.1 | 74.37 / 79.64 |
| best | kafka | 14 × wait | 3550038 | 99998 | 0 | 0 | 541017102 | 45833472 | 2954603528 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2954603528 | 0/1000 | 446.1 | 30.3 | 48.1 | 74.37 / 79.64 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 120.8 | 1392.6 | 11796.5 | 72351.7 | 78874.5 | 0 | 0 | 1/1 | 79.5 | 34.7 | 24.1 | 35.00 / 34.61 |
| best | kafka | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 120.8 | 1392.6 | 11796.5 | 72351.7 | 78874.5 | 0 | 0 | 1/1 | 79.5 | 34.7 | 24.1 | 35.00 / 34.61 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × busy | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 2785017.9 | 5100273.7 | 6643777.5 | 7851737.1 | 8806698.5 | 0 | 0 | 100/100 | 531.1 | 30.3 | 50.5 | 70.74 / 60.69 |
| best | kafka | 14 × busy | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 2785017.9 | 5100273.7 | 6643777.5 | 7851737.1 | 8806698.5 | 0 | 0 | 100/100 | 531.1 | 30.3 | 50.5 | 70.74 / 60.69 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × busy | 103690 | 2919 | 251108 | 0 | 103692000 | 53539000 | 246044000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 246044000 | 0/1000 | 861.5 | 23.7 | 102.1 | 68.45 / 80.16 |
| best | kafka | 14 × busy | 103690 | 2919 | 251108 | 0 | 103692000 | 53539000 | 246044000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 246044000 | 0/1000 | 861.5 | 23.7 | 102.1 | 68.45 / 80.16 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × busy | 3550038 | 99998 | 0 | 0 | 3150378 | 2650549 | 349888 | 14629732.4 | undelivered | undelivered | undelivered | undelivered | 0 | 349888 | 0/1 | 98.9 | 31.0 | 13.9 | 58.60 / 65.83 |
| best | kafka | 1 × busy | 3550038 | 99998 | 0 | 0 | 3150378 | 2650549 | 349888 | 14629732.4 | undelivered | undelivered | undelivered | undelivered | 0 | 349888 | 0/1 | 98.9 | 31.0 | 13.9 | 58.60 / 65.83 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × busy | 3550038 | 99998 | 0 | 0 | 88086445 | 38103545 | 261940155 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 261940155 | 0/100 | 913.2 | 28.3 | 31.0 | 64.85 / 48.34 |
| best | kafka | 14 × busy | 3550038 | 99998 | 0 | 0 | 88086445 | 38103545 | 261940155 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 261940155 | 0/100 | 913.2 | 28.3 | 31.0 | 64.85 / 48.34 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × busy | 3549976 | 99996 | 62 | 0 | 550816836 | 55943676 | 2944493324 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2944493324 | 0/1000 | 721.8 | 17.9 | 37.8 | 43.39 / 125.94 |
| best | kafka | 14 × busy | 3549976 | 99996 | 62 | 0 | 550816836 | 55943676 | 2944493324 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2944493324 | 0/1000 | 721.8 | 17.9 | 37.8 | 43.39 / 125.94 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 82.9 | 202.8 | 4030.5 | 11272.2 | 19909.7 | 0 | 0 | 1/1 | 44.9 | 35.4 | 22.7 | 42.88 / 31.42 |
| best | kafka | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 82.9 | 202.8 | 4030.5 | 11272.2 | 19909.7 | 0 | 0 | 1/1 | 44.9 | 35.4 | 22.7 | 42.88 / 31.42 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 4898947.1 | 8858370.0 | 12348031.0 | 16106127.4 | 17131856.2 | 0 | 0 | 100/100 | 107.5 | 35.4 | 79.5 | 24.75 / 20.88 |
| best | kafka | 14 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 4898947.1 | 8858370.0 | 12348031.0 | 16106127.4 | 17131856.2 | 0 | 0 | 100/100 | 107.5 | 35.4 | 79.5 | 24.75 / 20.88 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × wait | 356101 | 10031 | 0 | 0 | 334638547 | 282420243 | 18348757 | 29796335.6 | 34359738.4 | undelivered | undelivered | undelivered | 0 | 18348757 | 357/1000 | 264.5 | 32.8 | 68.9 | 50.54 / 60.97 |
| best | kafka | 14 × wait | 356101 | 10031 | 0 | 0 | 334638547 | 282420243 | 18348757 | 29796335.6 | 34359738.4 | undelivered | undelivered | undelivered | 0 | 18348757 | 357/1000 | 264.5 | 32.8 | 68.9 | 50.54 / 60.97 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × wait | 3554018 | 100110 | 0 | 0 | 1544955 | 1043659 | 1958972 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 1958972 | 0/1 | 50.3 | 34.9 | 29.3 | 53.39 / 42.28 |
| best | kafka | 1 × wait | 3554018 | 100110 | 0 | 0 | 1544955 | 1043659 | 1958972 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 1958972 | 0/1 | 50.3 | 34.9 | 29.3 | 53.39 / 42.28 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × wait | 3554018 | 100110 | 0 | 0 | 58979523 | 8864250 | 291398850 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 291398850 | 0/100 | 76.3 | 30.7 | 29.4 | 57.09 / 51.94 |
| best | kafka | 14 × wait | 3554018 | 100110 | 0 | 0 | 58979523 | 8864250 | 291398850 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 291398850 | 0/100 | 76.3 | 30.7 | 29.4 | 57.09 / 51.94 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × wait | 106724 | 2998 | 3447294 | 0 | 106724000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3002631000 | 1000/1000 | 197.2 | 35.4 | 116.1 | 40.54 / 47.23 |
| best | kafka | 14 × wait | 106724 | 2998 | 3447294 | 0 | 106724000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3002631000 | 1000/1000 | 197.2 | 35.4 | 116.1 | 40.54 / 47.23 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 76.8 | 153.6 | 1622.0 | 6881.3 | 13258.8 | 0 | 0 | 1/1 | 79.5 | 35.4 | 20.2 | 23.42 / 21.21 |
| best | kafka | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 76.8 | 153.6 | 1622.0 | 6881.3 | 13258.8 | 0 | 0 | 1/1 | 79.5 | 35.4 | 20.2 | 23.42 / 21.21 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 1644167.2 | 4630511.6 | 7314866.2 | 8992587.8 | 9780191.5 | 0 | 0 | 100/100 | 604.6 | 34.2 | 77.7 | 31.81 / 34.22 |
| best | kafka | 14 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 1644167.2 | 4630511.6 | 7314866.2 | 8992587.8 | 9780191.5 | 0 | 0 | 100/100 | 604.6 | 34.2 | 77.7 | 31.81 / 34.22 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × busy | 106057 | 2987 | 250044 | 0 | 106057000 | 55653000 | 245116000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 245116000 | 1000/1000 | 811.8 | 31.2 | 117.0 | 45.57 / 65.04 |
| best | kafka | 14 × busy | 106057 | 2987 | 250044 | 0 | 106057000 | 55653000 | 245116000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 245116000 | 1000/1000 | 811.8 | 31.2 | 117.0 | 45.57 / 65.04 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 1 × busy | 3554018 | 100110 | 0 | 0 | 1332788 | 831492 | 2171139 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2171139 | 0/1 | 78.7 | 26.0 | 12.6 | 73.77 / 82.49 |
| best | kafka | 1 × busy | 3554018 | 100110 | 0 | 0 | 1332788 | 831492 | 2171139 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2171139 | 0/1 | 78.7 | 26.0 | 12.6 | 73.77 / 82.49 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × busy | 3554014 | 100110 | 4 | 0 | 85250689 | 35121089 | 265142011 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 265142011 | 0/100 | 888.9 | 28.5 | 24.6 | 69.41 / 63.12 |
| best | kafka | 14 × busy | 3554014 | 100110 | 4 | 0 | 85250689 | 35121089 | 265142011 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 265142011 | 0/100 | 888.9 | 28.5 | 24.6 | 69.41 / 63.12 |
# quiver-bench fanout kafka

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: kafka (127.0.0.1:19092)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | kafka | 14 × busy | 3554018 | 100110 | 0 | 0 | 465908048 | 7137540 | 2995493460 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2995493460 | 0/1000 | 925.3 | 26.0 | 51.1 | 39.71 / 49.03 |
| best | kafka | 14 × busy | 3554018 | 100110 | 0 | 0 | 465908048 | 7137540 | 2995493460 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2995493460 | 0/1000 | 925.3 | 26.0 | 51.1 | 39.71 / 49.03 |
