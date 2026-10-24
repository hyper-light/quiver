- broker: public.ecr.aws/docker/library/redis:8.2-alpine@sha256:b51665e66f00759be7c3152ad5ac3c66fb2f619c13ef62dea7cc1f9914524635, CPUs 4-7,20-23
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 43 | 39.9 | 159.7 | 1622.0 | 4390.9 | undelivered | 0 | 43 | 0/1 | 35.7 | 33.3 | 3.9 | 52.00 / 47.94 |
| best | redis | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 43 | 39.9 | 159.7 | 1622.0 | 4390.9 | undelivered | 0 | 43 | 0/1 | 35.7 | 33.3 | 3.9 | 52.00 / 47.94 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × wait | 354575 | 9988 | 15 | 0 | 35457500 | 29949700 | 26700 | 1228.8 | 3964.9 | 13631.5 | 62390.3 | undelivered | 0 | 26700 | 0/100 | 59.6 | 23.9 | 16.1 | 95.48 / 90.66 |
| best | redis | 14 × wait | 354575 | 9988 | 15 | 0 | 35457500 | 29949700 | 26700 | 1228.8 | 3964.9 | 13631.5 | 62390.3 | undelivered | 0 | 26700 | 0/100 | 59.6 | 23.9 | 16.1 | 95.48 / 90.66 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × wait | 93916 | 2644 | 260674 | 0 | 93916000 | 43852000 | 262235000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 262235000 | 0/1000 | 69.8 | 0.5 | 28.9 | 55.86 / 49.85 |
| best | redis | 14 × wait | 93916 | 2644 | 260674 | 0 | 93916000 | 43852000 | 262235000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 262235000 | 0/1000 | 69.8 | 0.5 | 28.9 | 55.86 / 49.85 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 9255 | 66.6 | 516.1 | 1736.7 | undelivered | undelivered | 0 | 9255 | 0/1 | 35.6 | 28.2 | 12.2 | 41.60 / 50.94 |
| best | redis | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 9255 | 66.6 | 516.1 | 1736.7 | undelivered | undelivered | 0 | 9255 | 0/1 | 35.6 | 28.2 | 12.2 | 41.60 / 50.94 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × wait | 1272560 | 35771 | 2274983 | 0 | 127256000 | 77318900 | 233325700 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 233325700 | 0/100 | 84.5 | 3.0 | 27.1 | 80.27 / 77.48 |
| best | redis | 14 × wait | 1272560 | 35771 | 2274983 | 0 | 127256000 | 77318900 | 233325700 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 233325700 | 0/100 | 84.5 | 3.0 | 27.1 | 80.27 / 77.48 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × wait | 76834 | 2156 | 3470709 | 0 | 76834000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2998274000 | 0/1000 | 65.1 | 0.5 | 29.8 | 92.75 / 77.51 |
| best | redis | 14 × wait | 76834 | 2156 | 3470709 | 0 | 76834000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2998274000 | 0/1000 | 65.1 | 0.5 | 29.8 | 92.75 / 77.51 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 34.8 | 260.1 | 1540.1 | 3080.2 | 6091.7 | 0 | 0 | 1/1 | 65.8 | 32.6 | 3.3 | 92.95 / 81.82 |
| best | redis | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 34.8 | 260.1 | 1540.1 | 3080.2 | 6091.7 | 0 | 0 | 1/1 | 65.8 | 32.6 | 3.3 | 92.95 / 81.82 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × busy | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 1097.7 | 5963.8 | 33554.4 | 81788.9 | 92946.9 | 0 | 0 | 0/100 | 445.7 | 25.8 | 15.9 | 113.39 / 106.47 |
| best | redis | 14 × busy | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 1097.7 | 5963.8 | 33554.4 | 81788.9 | 92946.9 | 0 | 0 | 0/100 | 445.7 | 25.8 | 15.9 | 113.39 / 106.47 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × busy | 99898 | 2810 | 254692 | 0 | 99898000 | 49834000 | 256783000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 256783000 | 0/1000 | 460.2 | 0.5 | 30.2 | 91.27 / 84.32 |
| best | redis | 14 × busy | 99898 | 2810 | 254692 | 0 | 99898000 | 49834000 | 256783000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 256783000 | 0/1000 | 460.2 | 0.5 | 30.2 | 91.27 / 84.32 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 9937 | 34.3 | 360.4 | 1949.7 | undelivered | undelivered | 0 | 9937 | 0/1 | 57.4 | 26.6 | 13.6 | 78.16 / 86.43 |
| best | redis | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 9937 | 34.3 | 360.4 | 1949.7 | undelivered | undelivered | 0 | 9937 | 0/1 | 57.4 | 26.6 | 13.6 | 78.16 / 86.43 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × busy | 1388964 | 39032 | 2158579 | 0 | 138896400 | 88959300 | 223500200 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 223500200 | 0/100 | 475.4 | 2.1 | 28.5 | 71.41 / 63.73 |
| best | redis | 14 × busy | 1388964 | 39032 | 2158579 | 0 | 138896400 | 88959300 | 223500200 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 223500200 | 0/100 | 475.4 | 2.1 | 28.5 | 71.41 / 63.73 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × busy | 96063 | 2689 | 3451480 | 0 | 96063000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2998274000 | 0/1000 | 452.2 | 0.5 | 28.8 | 80.35 / 76.01 |
| best | redis | 14 × busy | 96063 | 2689 | 3451480 | 0 | 96063000 | 0 | 2998274000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 2998274000 | 0/1000 | 452.2 | 0.5 | 28.8 | 80.35 / 76.01 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 35.3 | 270.3 | 1916.9 | 3768.3 | 8279.3 | 0 | 0 | 1/1 | 35.7 | 33.5 | 3.3 | 54.86 / 56.16 |
| best | redis | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 35.3 | 270.3 | 1916.9 | 3768.3 | 8279.3 | 0 | 0 | 1/1 | 35.7 | 33.5 | 3.3 | 54.86 / 56.16 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 548.9 | 827.4 | 1097.7 | 1622.0 | 6330.0 | 0 | 0 | 100/100 | 82.3 | 34.9 | 16.0 | 30.72 / 26.43 |
| best | redis | 14 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 548.9 | 827.4 | 1097.7 | 1622.0 | 6330.0 | 0 | 0 | 100/100 | 82.3 | 34.9 | 16.0 | 30.72 / 26.43 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × wait | 107085 | 3016 | 247713 | 0 | 107085000 | 56934000 | 251089000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 251089000 | 0/1000 | 71.0 | 0.5 | 29.6 | 43.50 / 33.61 |
| best | redis | 14 × wait | 107085 | 3016 | 247713 | 0 | 107085000 | 56934000 | 251089000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 251089000 | 0/1000 | 71.0 | 0.5 | 29.6 | 43.50 / 33.61 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 22752 | 155.6 | 1818.6 | 19398.7 | undelivered | undelivered | 0 | 22752 | 0/1 | 34.3 | 26.9 | 13.0 | 41.87 / 49.69 |
| best | redis | 1 × wait | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 22752 | 155.6 | 1818.6 | 19398.7 | undelivered | undelivered | 0 | 22752 | 0/1 | 34.3 | 26.9 | 13.0 | 41.87 / 49.69 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × wait | 945074 | 26543 | 2604964 | 0 | 94507400 | 44524500 | 261749200 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 261749200 | 0/100 | 75.5 | 4.1 | 25.5 | 72.03 / 71.61 |
| best | redis | 14 × wait | 945074 | 26543 | 2604964 | 0 | 94507400 | 44524500 | 261749200 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 261749200 | 0/100 | 75.5 | 4.1 | 25.5 | 72.03 / 71.61 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × wait | 53254 | 1495 | 3496784 | 0 | 53254000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3000437000 | 0/1000 | 51.3 | 0.5 | 27.3 | 87.07 / 74.37 |
| best | redis | 14 × wait | 53254 | 1495 | 3496784 | 0 | 53254000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3000437000 | 0/1000 | 51.3 | 0.5 | 27.3 | 87.07 / 74.37 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 26.4 | 43.5 | 729.1 | 1736.7 | 3965.3 | 0 | 0 | 1/1 | 68.7 | 34.0 | 3.6 | 36.42 / 35.00 |
| best | redis | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 26.4 | 43.5 | 729.1 | 1736.7 | 3965.3 | 0 | 0 | 1/1 | 68.7 | 34.0 | 3.6 | 36.42 / 35.00 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × busy | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 4600 | 811.0 | 2293.8 | 6946.8 | 19398.7 | undelivered | 0 | 4600 | 0/100 | 397.9 | 26.7 | 18.1 | 82.78 / 70.74 |
| best | redis | 14 × busy | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 4600 | 811.0 | 2293.8 | 6946.8 | 19398.7 | undelivered | 0 | 4600 | 0/100 | 397.9 | 26.7 | 18.1 | 82.78 / 70.74 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × busy | 70946 | 1997 | 283852 | 0 | 70946000 | 20795000 | 281788000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 281788000 | 0/1000 | 399.9 | 0.4 | 28.5 | 53.16 / 68.45 |
| best | redis | 14 × busy | 70946 | 1997 | 283852 | 0 | 70946000 | 20795000 | 281788000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 281788000 | 0/1000 | 399.9 | 0.4 | 28.5 | 53.16 / 68.45 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × busy | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 30126 | 35.3 | 1048.6 | 11665.4 | undelivered | undelivered | 0 | 30126 | 0/1 | 53.1 | 24.7 | 12.9 | 68.34 / 58.60 |
| best | redis | 1 × busy | 3550038 | 99998 | 0 | 0 | 3550038 | 3000437 | 30126 | 35.3 | 1048.6 | 11665.4 | undelivered | undelivered | 0 | 30126 | 0/1 | 53.1 | 24.7 | 12.9 | 68.34 / 58.60 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × busy | 1377108 | 38717 | 2172930 | 0 | 137710800 | 87727900 | 225282900 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 225282900 | 0/100 | 487.2 | 2.0 | 28.3 | 84.32 / 64.85 |
| best | redis | 14 × busy | 1377108 | 38717 | 2172930 | 0 | 137710800 | 87727900 | 225282900 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 225282900 | 0/100 | 487.2 | 2.0 | 28.3 | 84.32 / 64.85 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × busy | 79641 | 2228 | 3470397 | 0 | 79641000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3000437000 | 0/1000 | 461.2 | 0.4 | 29.4 | 40.03 / 43.39 |
| best | redis | 14 × busy | 79641 | 2228 | 3470397 | 0 | 79641000 | 0 | 3000437000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3000437000 | 0/1000 | 461.2 | 0.4 | 29.4 | 40.03 / 43.39 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 26.9 | 43.5 | 202.8 | 1040.4 | 3070.9 | 0 | 0 | 1/1 | 36.3 | 34.4 | 3.1 | 61.73 / 42.88 |
| best | redis | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 26.9 | 43.5 | 202.8 | 1040.4 | 3070.9 | 0 | 0 | 1/1 | 36.3 | 34.4 | 3.1 | 61.73 / 42.88 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 540.7 | 811.0 | 1097.7 | 1769.5 | 4803.2 | 0 | 0 | 100/100 | 78.7 | 34.9 | 16.2 | 28.58 / 24.75 |
| best | redis | 14 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 540.7 | 811.0 | 1097.7 | 1769.5 | 4803.2 | 0 | 0 | 100/100 | 78.7 | 34.9 | 16.2 | 28.58 / 24.75 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × wait | 117717 | 3315 | 238384 | 0 | 117717000 | 67313000 | 243846000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 243846000 | 0/1000 | 78.5 | 0.6 | 28.9 | 50.14 / 50.54 |
| best | redis | 14 × wait | 117717 | 3315 | 238384 | 0 | 117717000 | 67313000 | 243846000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 243846000 | 0/1000 | 78.5 | 0.6 | 28.9 | 50.14 / 50.54 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × wait | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 1135 | 36.4 | 161.8 | 1179.6 | 2293.8 | undelivered | 0 | 1135 | 0/1 | 38.1 | 27.9 | 15.7 | 64.52 / 53.39 |
| best | redis | 1 × wait | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 1135 | 36.4 | 161.8 | 1179.6 | 2293.8 | undelivered | 0 | 1135 | 0/1 | 38.1 | 27.9 | 15.7 | 64.52 / 53.39 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × wait | 1247844 | 34996 | 2306174 | 0 | 124784400 | 74654800 | 236333700 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 236333700 | 0/100 | 86.5 | 3.1 | 26.8 | 39.28 / 55.27 |
| best | redis | 14 × wait | 1247844 | 34996 | 2306174 | 0 | 124784400 | 74654800 | 236333700 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 236333700 | 0/100 | 86.5 | 3.1 | 26.8 | 39.28 / 55.27 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × wait | 104390 | 2932 | 3449628 | 0 | 104390000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3002631000 | 0/1000 | 70.3 | 0.5 | 29.0 | 55.42 / 40.54 |
| best | redis | 14 × wait | 104390 | 2932 | 3449628 | 0 | 104390000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3002631000 | 0/1000 | 70.3 | 0.5 | 29.0 | 55.42 / 40.54 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 18.4 | 26.4 | 67.6 | 1228.8 | 3694.7 | 0 | 0 | 1/1 | 69.6 | 34.5 | 2.4 | 29.15 / 23.42 |
| best | redis | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 18.4 | 26.4 | 67.6 | 1228.8 | 3694.7 | 0 | 0 | 1/1 | 69.6 | 34.5 | 2.4 | 29.15 / 23.42 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 540.7 | 868.4 | 1245.2 | 2064.4 | 8319.8 | 0 | 0 | 100/100 | 522.1 | 34.6 | 19.0 | 30.92 / 31.81 |
| best | redis | 14 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 540.7 | 868.4 | 1245.2 | 2064.4 | 8319.8 | 0 | 0 | 100/100 | 522.1 | 34.6 | 19.0 | 30.92 / 31.81 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × busy | 108299 | 3049 | 247802 | 0 | 108299000 | 57895000 | 251784000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 251784000 | 0/1000 | 459.5 | 0.5 | 28.9 | 39.19 / 45.57 |
| best | redis | 14 × busy | 108299 | 3049 | 247802 | 0 | 108299000 | 57895000 | 251784000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 251784000 | 0/1000 | 459.5 | 0.5 | 28.9 | 39.19 / 45.57 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 1 × busy | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 40591 | 51.7 | 1409.0 | undelivered | undelivered | undelivered | 0 | 40591 | 0/1 | 53.8 | 25.6 | 10.8 | 74.66 / 73.77 |
| best | redis | 1 × busy | 3554018 | 100110 | 0 | 0 | 3554018 | 3002631 | 40591 | 51.7 | 1409.0 | undelivered | undelivered | undelivered | 0 | 40591 | 0/1 | 53.8 | 25.6 | 10.8 | 74.66 / 73.77 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × busy | 1221586 | 34321 | 2332432 | 0 | 122158600 | 72029000 | 238448200 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 238448200 | 0/100 | 463.0 | 2.1 | 28.3 | 80.87 / 69.41 |
| best | redis | 14 × busy | 1221586 | 34321 | 2332432 | 0 | 122158600 | 72029000 | 238448200 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 238448200 | 0/100 | 463.0 | 2.1 | 28.3 | 80.87 / 69.41 |
# quiver-bench fanout redis

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: redis (127.0.0.1:16380)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | redis | 14 × busy | 107469 | 3016 | 3446549 | 0 | 107469000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3002631000 | 0/1000 | 498.2 | 0.4 | 29.5 | 48.50 / 39.71 |
| best | redis | 14 × busy | 107469 | 3016 | 3446549 | 0 | 107469000 | 0 | 3002631000 | undelivered | undelivered | undelivered | undelivered | undelivered | 0 | 3002631000 | 0/1000 | 498.2 | 0.4 | 29.5 | 48.50 / 39.71 |
