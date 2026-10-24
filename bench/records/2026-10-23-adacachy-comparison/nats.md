- broker: public.ecr.aws/docker/library/nats:2.12-alpine@sha256:b270f5e2428354c0335612694d7dd2fb588148e567a5757fdff325ef9c9332e6, CPUs 4-7,20-23
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 39.4 | 190.5 | 1966.1 | 5308.4 | 11242.9 | 0 | 0 | 1/1 | 34.2 | 32.2 | 6.0 | 56.67 / 52.00 |
| best | nats | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 39.4 | 190.5 | 1966.1 | 5308.4 | 11242.9 | 0 | 0 | 1/1 | 34.2 | 32.2 | 6.0 | 56.67 / 52.00 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 2424.8 | 9044.0 | 25165.8 | 40894.5 | 59831.8 | 0 | 0 | 100/100 | 50.1 | 20.7 | 26.5 | 94.45 / 95.48 |
| best | nats | 14 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 2424.8 | 9044.0 | 25165.8 | 40894.5 | 59831.8 | 0 | 0 | 100/100 | 50.1 | 20.7 | 26.5 | 94.45 / 95.48 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × wait | 125389 | 3471 | 229201 | 0 | 125389000 | 75325000 | 224172000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 224172000 | 1000/1000 | 141.6 | 2.1 | 150.1 | 65.01 / 55.86 |
| best | nats | 14 × wait | 125389 | 3471 | 229201 | 0 | 125389000 | 75325000 | 224172000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 224172000 | 1000/1000 | 141.6 | 2.1 | 150.1 | 65.01 / 55.86 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 26.6 | 65.0 | 1097.7 | 5308.4 | 12381.4 | 0 | 0 | 1/1 | 36.8 | 28.4 | 24.7 | 46.67 / 41.60 |
| best | nats | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 26.6 | 65.0 | 1097.7 | 5308.4 | 12381.4 | 0 | 0 | 1/1 | 36.8 | 28.4 | 24.7 | 46.67 / 41.60 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × wait | 1256054 | 35298 | 2291489 | 0 | 125605400 | 75668300 | 224159100 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 224159100 | 100/100 | 63.5 | 0.8 | 57.9 | 76.57 / 80.27 |
| best | nats | 14 × wait | 1256054 | 35298 | 2291489 | 0 | 125605400 | 75668300 | 224159100 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 224159100 | 100/100 | 63.5 | 0.8 | 57.9 | 76.57 / 80.27 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × wait | 120650 | 3323 | 3426893 | 0 | 120650000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2998274000 | 1000/1000 | 121.5 | 0.3 | 135.0 | 100.03 / 92.75 |
| best | nats | 14 × wait | 120650 | 3323 | 3426893 | 0 | 120650000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2998274000 | 1000/1000 | 121.5 | 0.3 | 135.0 | 100.03 / 92.75 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 45.1 | 671.7 | 2523.1 | 5374.0 | 15188.5 | 0 | 0 | 1/1 | 65.1 | 32.3 | 5.3 | 106.85 / 92.95 |
| best | nats | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 45.1 | 671.7 | 2523.1 | 5374.0 | 15188.5 | 0 | 0 | 1/1 | 65.1 | 32.3 | 5.3 | 106.85 / 92.95 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × busy | 354582 | 9988 | 8 | 0 | 35458200 | 29949700 | 0 | 1490.9 | 9044.0 | 31195.1 | 44040.2 | 56834.5 | 0 | 0 | 100/100 | 400.0 | 31.0 | 28.6 | 104.91 / 113.39 |
| best | nats | 14 × busy | 354582 | 9988 | 8 | 0 | 35458200 | 29949700 | 0 | 1490.9 | 9044.0 | 31195.1 | 44040.2 | 56834.5 | 0 | 0 | 100/100 | 400.0 | 31.0 | 28.6 | 104.91 / 113.39 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × busy | 119046 | 3192 | 235544 | 0 | 119046000 | 68982000 | 230515000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 230515000 | 1000/1000 | 490.8 | 2.2 | 153.9 | 87.72 / 94.60 |
| best | nats | 14 × busy | 119046 | 3192 | 235544 | 0 | 119046000 | 68982000 | 230515000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 230515000 | 1000/1000 | 490.8 | 2.2 | 153.9 | 87.72 / 94.60 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 59.4 | 3145.7 | 14942.2 | 33554.4 | 50880.2 | 0 | 0 | 1/1 | 54.7 | 25.6 | 17.0 | 66.02 / 78.16 |
| best | nats | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 59.4 | 3145.7 | 14942.2 | 33554.4 | 50880.2 | 0 | 0 | 1/1 | 54.7 | 25.6 | 17.0 | 66.02 / 78.16 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × busy | 1900865 | 53318 | 1646678 | 0 | 190086500 | 140149400 | 159678000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 159678000 | 100/100 | 426.8 | 1.0 | 153.3 | 69.67 / 71.41 |
| best | nats | 14 × busy | 1900865 | 53318 | 1646678 | 0 | 190086500 | 140149400 | 159678000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 159678000 | 100/100 | 426.8 | 1.0 | 153.3 | 69.67 / 71.41 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × busy | 130153 | 3627 | 3417390 | 0 | 130153000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2998274000 | 1000/1000 | 478.2 | 0.4 | 139.8 | 76.32 / 80.35 |
| best | nats | 14 × busy | 130153 | 3627 | 3417390 | 0 | 130153000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2998274000 | 1000/1000 | 478.2 | 0.4 | 139.8 | 76.32 / 80.35 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 31.2 | 286.7 | 1278.0 | 3113.0 | 11365.1 | 0 | 0 | 1/1 | 36.0 | 34.3 | 5.1 | 62.13 / 54.86 |
| best | nats | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 31.2 | 286.7 | 1278.0 | 3113.0 | 11365.1 | 0 | 0 | 1/1 | 36.0 | 34.3 | 5.1 | 62.13 / 54.86 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 119.8 | 237.6 | 778.2 | 1671.2 | 3725.3 | 0 | 0 | 100/100 | 118.9 | 34.5 | 76.4 | 32.68 / 30.72 |
| best | nats | 14 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 119.8 | 237.6 | 778.2 | 1671.2 | 3725.3 | 0 | 0 | 100/100 | 118.9 | 34.5 | 76.4 | 32.68 / 30.72 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × wait | 137821 | 3833 | 216977 | 0 | 137821000 | 87670000 | 211913000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 211913000 | 1000/1000 | 181.3 | 2.6 | 175.3 | 51.31 / 43.50 |
| best | nats | 14 × wait | 137821 | 3833 | 216977 | 0 | 137821000 | 87670000 | 211913000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 211913000 | 1000/1000 | 181.3 | 2.6 | 175.3 | 51.31 / 43.50 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 38.4 | 1196.0 | 6488.1 | 16646.1 | 28131.4 | 0 | 0 | 1/1 | 34.1 | 28.0 | 19.2 | 37.33 / 41.87 |
| best | nats | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 38.4 | 1196.0 | 6488.1 | 16646.1 | 28131.4 | 0 | 0 | 1/1 | 34.1 | 28.0 | 19.2 | 37.33 / 41.87 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × wait | 936409 | 26270 | 2613629 | 0 | 93640900 | 43658000 | 256385700 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 256385700 | 100/100 | 89.6 | 0.7 | 96.2 | 63.70 / 72.03 |
| best | nats | 14 × wait | 936409 | 26270 | 2613629 | 0 | 93640900 | 43658000 | 256385700 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 256385700 | 100/100 | 89.6 | 0.7 | 96.2 | 63.70 / 72.03 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × wait | 87958 | 2410 | 3462080 | 0 | 87958000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3000437000 | 1000/1000 | 135.9 | 0.3 | 157.6 | 65.36 / 87.07 |
| best | nats | 14 × wait | 87958 | 2410 | 3462080 | 0 | 87958000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3000437000 | 1000/1000 | 135.9 | 0.3 | 157.6 | 65.36 / 87.07 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 23.8 | 37.9 | 376.8 | 1622.0 | 4031.6 | 0 | 0 | 1/1 | 69.3 | 34.3 | 5.5 | 46.84 / 36.42 |
| best | nats | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 23.8 | 37.9 | 376.8 | 1622.0 | 4031.6 | 0 | 0 | 1/1 | 69.3 | 34.3 | 5.5 | 46.84 / 36.42 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × busy | 354795 | 9994 | 3 | 0 | 35479500 | 29958300 | 0 | 3407.9 | 10485.8 | 22282.2 | 31195.1 | 47169.2 | 0 | 0 | 100/100 | 214.3 | 14.6 | 26.6 | 49.40 / 82.78 |
| best | nats | 14 × busy | 354795 | 9994 | 3 | 0 | 35479500 | 29958300 | 0 | 3407.9 | 10485.8 | 22282.2 | 31195.1 | 47169.2 | 0 | 0 | 100/100 | 214.3 | 14.6 | 26.6 | 49.40 / 82.78 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × busy | 88832 | 2323 | 265966 | 0 | 88832000 | 38681000 | 260902000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 260902000 | 1000/1000 | 470.6 | 1.9 | 115.0 | 48.17 / 55.96 |
| best | nats | 14 × busy | 88832 | 2323 | 265966 | 0 | 88832000 | 38681000 | 260902000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 260902000 | 1000/1000 | 470.6 | 1.9 | 115.0 | 48.17 / 55.96 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × busy | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 1081.3 | 4980.7 | 14024.7 | 133169.2 | 156664.0 | 0 | 0 | 1/1 | 39.3 | 19.1 | 13.9 | 65.84 / 68.34 |
| best | nats | 1 × busy | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 0 | 1081.3 | 4980.7 | 14024.7 | 133169.2 | 156664.0 | 0 | 0 | 1/1 | 39.3 | 19.1 | 13.9 | 65.84 / 68.34 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × busy | 1166802 | 32787 | 2383236 | 0 | 116680200 | 66697300 | 233346400 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 233346400 | 100/100 | 417.5 | 0.7 | 57.0 | 87.33 / 84.32 |
| best | nats | 14 × busy | 1166802 | 32787 | 2383236 | 0 | 116680200 | 66697300 | 233346400 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 233346400 | 100/100 | 417.5 | 0.7 | 57.0 | 87.33 / 84.32 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × busy | 136854 | 3789 | 3413184 | 0 | 136854000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3000437000 | 1000/1000 | 496.3 | 0.3 | 167.8 | 37.67 / 40.03 |
| best | nats | 14 × busy | 136854 | 3789 | 3413184 | 0 | 136854000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3000437000 | 1000/1000 | 496.3 | 0.3 | 167.8 | 37.67 / 40.03 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 31.2 | 50.7 | 487.4 | 1441.8 | 3050.9 | 0 | 0 | 1/1 | 36.4 | 34.3 | 6.2 | 94.40 / 61.73 |
| best | nats | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 31.2 | 50.7 | 487.4 | 1441.8 | 3050.9 | 0 | 0 | 1/1 | 36.4 | 34.3 | 6.2 | 94.40 / 61.73 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 155.6 | 835.6 | 4653.1 | 22282.2 | 74125.4 | 0 | 0 | 100/100 | 109.1 | 33.8 | 70.8 | 23.83 / 28.58 |
| best | nats | 14 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 155.6 | 835.6 | 4653.1 | 22282.2 | 74125.4 | 0 | 0 | 100/100 | 109.1 | 33.8 | 70.8 | 23.83 / 28.58 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × wait | 133046 | 3592 | 223055 | 0 | 133046000 | 82642000 | 218127000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 218127000 | 1000/1000 | 111.1 | 2.4 | 128.9 | 37.83 / 50.14 |
| best | nats | 14 × wait | 133046 | 3592 | 223055 | 0 | 133046000 | 82642000 | 218127000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 218127000 | 1000/1000 | 111.1 | 2.4 | 128.9 | 37.83 / 50.14 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × wait | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 49.2 | 548.9 | 2457.6 | 5636.1 | 14268.5 | 0 | 0 | 1/1 | 33.9 | 28.2 | 18.4 | 77.62 / 64.52 |
| best | nats | 1 × wait | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 49.2 | 548.9 | 2457.6 | 5636.1 | 14268.5 | 0 | 0 | 1/1 | 33.9 | 28.2 | 18.4 | 77.62 / 64.52 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × wait | 1298185 | 36369 | 2255833 | 0 | 129818500 | 79688900 | 220574200 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 220574200 | 100/100 | 133.2 | 1.1 | 139.5 | 36.97 / 39.28 |
| best | nats | 14 × wait | 1298185 | 36369 | 2255833 | 0 | 129818500 | 79688900 | 220574200 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 220574200 | 100/100 | 133.2 | 1.1 | 139.5 | 36.97 / 39.28 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × wait | 132248 | 3702 | 3421770 | 0 | 132248000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3002631000 | 1000/1000 | 182.4 | 0.3 | 179.4 | 42.05 / 55.42 |
| best | nats | 14 × wait | 132248 | 3702 | 3421770 | 0 | 132248000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3002631000 | 1000/1000 | 182.4 | 0.3 | 179.4 | 42.05 / 55.42 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 17.9 | 24.8 | 37.4 | 462.8 | 4144.4 | 0 | 0 | 1/1 | 69.6 | 34.5 | 4.2 | 44.34 / 29.15 |
| best | nats | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 17.9 | 24.8 | 37.4 | 462.8 | 4144.4 | 0 | 0 | 1/1 | 69.6 | 34.5 | 4.2 | 44.34 / 29.15 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 155.6 | 548.9 | 2293.8 | 4980.7 | 16873.3 | 0 | 0 | 100/100 | 495.6 | 33.6 | 65.4 | 22.54 / 30.92 |
| best | nats | 14 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 155.6 | 548.9 | 2293.8 | 4980.7 | 16873.3 | 0 | 0 | 100/100 | 495.6 | 33.6 | 65.4 | 22.54 / 30.92 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × busy | 147740 | 4140 | 208361 | 0 | 147740000 | 97336000 | 203433000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 203433000 | 1000/1000 | 531.7 | 3.4 | 185.7 | 37.13 / 39.19 |
| best | nats | 14 × busy | 147740 | 4140 | 208361 | 0 | 147740000 | 97336000 | 203433000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 203433000 | 1000/1000 | 531.7 | 3.4 | 185.7 | 37.13 / 39.19 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 1 × busy | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 119.8 | 2097.2 | 8781.8 | 17825.8 | 25337.0 | 0 | 0 | 1/1 | 57.4 | 27.2 | 13.7 | 73.76 / 74.66 |
| best | nats | 1 × busy | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 0 | 119.8 | 2097.2 | 8781.8 | 17825.8 | 25337.0 | 0 | 0 | 1/1 | 57.4 | 27.2 | 13.7 | 73.76 / 74.66 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × busy | 1311380 | 36781 | 2242638 | 0 | 131138000 | 81008400 | 219254700 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 219254700 | 100/100 | 307.9 | 0.8 | 66.3 | 75.83 / 80.87 |
| best | nats | 14 × busy | 1311380 | 36781 | 2242638 | 0 | 131138000 | 81008400 | 219254700 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 219254700 | 100/100 | 307.9 | 0.8 | 66.3 | 75.83 / 80.87 |
# quiver-bench fanout nats

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: nats (127.0.0.1:14222)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | nats | 14 × busy | 120440 | 3145 | 3433578 | 0 | 120440000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3002631000 | 1000/1000 | 483.5 | 0.3 | 163.0 | 45.40 / 48.50 |
| best | nats | 14 × busy | 120440 | 3145 | 3433578 | 0 | 120440000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3002631000 | 1000/1000 | 483.5 | 0.3 | 163.0 | 45.40 / 48.50 |
