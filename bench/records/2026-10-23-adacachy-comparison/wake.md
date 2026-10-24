# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 2.3 | 6.0 | 227.3 | 1507.3 | 5050.3 | 0 | 0 | 1/1 | 35.3 | 34.8 | - | 76.41 / 56.46 |
| best | wake | 1 × wait | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 2.3 | 6.0 | 227.3 | 1507.3 | 5050.3 | 0 | 0 | 1/1 | 35.3 | 34.8 | - | 76.41 / 56.46 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × wait | 354590 | 9988 | 0 | 0 | 4964260 | 4192958 | 0 | 5.7 | 15.7 | 688.1 | 2129.9 | 7594.7 | 0 | 0 | 14/14 | 42.1 | 35.2 | - | 60.72 / 47.48 |
| best | wake | 14 × wait | 354590 | 9988 | 0 | 0 | 4964260 | 4192958 | 0 | 5.7 | 15.7 | 688.1 | 2129.9 | 7594.7 | 0 | 0 | 14/14 | 42.1 | 35.2 | - | 60.72 / 47.48 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × wait | 354590 | 9989 | 0 | 0 | 4964260 | 4192958 | 0 | 12.9 | 352.3 | 1949.7 | 4522.0 | 10118.5 | 0 | 0 | 14/14 | 40.5 | 32.2 | - | 87.74 / 75.70 |
| best | wake | 14 × wait | 354590 | 9989 | 0 | 0 | 4964260 | 4192958 | 0 | 12.9 | 352.3 | 1949.7 | 4522.0 | 10118.5 | 0 | 0 | 14/14 | 40.5 | 32.2 | - | 87.74 / 75.70 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 3.2 | 802.8 | 1490.9 | 2752.5 | 9488.6 | 0 | 0 | 1/1 | 37.5 | 34.4 | - | 47.17 / 46.75 |
| best | wake | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 3.2 | 802.8 | 1490.9 | 2752.5 | 9488.6 | 0 | 0 | 1/1 | 37.5 | 34.4 | - | 47.17 / 46.75 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × wait | 3547543 | 99930 | 0 | 0 | 49665602 | 41975836 | 0 | 73.7 | 589.8 | 2392.1 | 4915.2 | 10713.7 | 0 | 0 | 14/14 | 87.8 | 32.5 | - | 71.70 / 75.60 |
| best | wake | 14 × wait | 3547543 | 99930 | 0 | 0 | 49665602 | 41975836 | 0 | 73.7 | 589.8 | 2392.1 | 4915.2 | 10713.7 | 0 | 0 | 14/14 | 87.8 | 32.5 | - | 71.70 / 75.60 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × wait | 3547543 | 99929 | 0 | 0 | 49665602 | 41975836 | 0 | 161.8 | 2719.7 | 9568.3 | 28573.7 | 42064.2 | 0 | 0 | 14/14 | 64.8 | 28.3 | - | 60.88 / 101.52 |
| best | wake | 14 × wait | 3547543 | 99929 | 0 | 0 | 49665602 | 41975836 | 0 | 161.8 | 2719.7 | 9568.3 | 28573.7 | 42064.2 | 0 | 0 | 14/14 | 64.8 | 28.3 | - | 60.88 / 101.52 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 0.1 | 215.0 | 1540.1 | 2916.4 | 5744.1 | 0 | 0 | 1/1 | 63.9 | 31.8 | - | 118.83 / 142.48 |
| best | wake | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 0.1 | 215.0 | 1540.1 | 2916.4 | 5744.1 | 0 | 0 | 1/1 | 63.9 | 31.8 | - | 118.83 / 142.48 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × busy | 354590 | 9989 | 0 | 0 | 4964260 | 4192958 | 0 | 9.2 | 1671.2 | 4063.2 | 6488.1 | 21894.5 | 0 | 0 | 14/14 | 363.9 | 22.4 | - | 75.41 / 103.86 |
| best | wake | 14 × busy | 354590 | 9989 | 0 | 0 | 4964260 | 4192958 | 0 | 9.2 | 1671.2 | 4063.2 | 6488.1 | 21894.5 | 0 | 0 | 14/14 | 363.9 | 22.4 | - | 75.41 / 103.86 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × busy | 354590 | 9988 | 0 | 0 | 4964260 | 4192958 | 0 | 0.2 | 319.5 | 1884.2 | 4161.5 | 15175.2 | 0 | 0 | 14/14 | 448.9 | 30.8 | - | 99.15 / 96.28 |
| best | wake | 14 × busy | 354590 | 9988 | 0 | 0 | 4964260 | 4192958 | 0 | 0.2 | 319.5 | 1884.2 | 4161.5 | 15175.2 | 0 | 0 | 14/14 | 448.9 | 30.8 | - | 99.15 / 96.28 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 0.1 | 0.1 | 75.8 | 950.3 | 2256.4 | 0 | 0 | 1/1 | 69.9 | 35.0 | - | 75.11 / 62.37 |
| best | wake | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 0.1 | 0.1 | 75.8 | 950.3 | 2256.4 | 0 | 0 | 1/1 | 69.9 | 35.0 | - | 75.11 / 62.37 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × busy | 3547543 | 99933 | 0 | 0 | 49665602 | 41975836 | 0 | 0.2 | 278.5 | 1572.9 | 3244.0 | 13574.3 | 0 | 0 | 14/14 | 477.0 | 31.9 | - | 66.83 / 72.73 |
| best | wake | 14 × busy | 3547543 | 99933 | 0 | 0 | 49665602 | 41975836 | 0 | 0.2 | 278.5 | 1572.9 | 3244.0 | 13574.3 | 0 | 0 | 14/14 | 477.0 | 31.9 | - | 66.83 / 72.73 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × busy | 3547543 | 99932 | 0 | 0 | 49665602 | 41975836 | 0 | 0.2 | 30.2 | 1081.3 | 2490.4 | 13009.8 | 0 | 0 | 14/14 | 496.8 | 33.2 | - | 79.03 / 73.47 |
| best | wake | 14 × busy | 3547543 | 99932 | 0 | 0 | 49665602 | 41975836 | 0 | 0.2 | 30.2 | 1081.3 | 2490.4 | 13009.8 | 0 | 0 | 14/14 | 496.8 | 33.2 | - | 79.03 / 73.47 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 2.2 | 4.2 | 170.0 | 1032.2 | 1740.6 | 0 | 0 | 1/1 | 35.7 | 35.2 | - | 50.53 / 43.14 |
| best | wake | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 2.2 | 4.2 | 170.0 | 1032.2 | 1740.6 | 0 | 0 | 1/1 | 35.7 | 35.2 | - | 50.53 / 43.14 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × wait | 354798 | 9994 | 0 | 0 | 4967172 | 4194162 | 0 | 4.8 | 8.1 | 27.6 | 598.0 | 4350.2 | 0 | 0 | 14/14 | 40.8 | 35.2 | - | 30.15 / 24.72 |
| best | wake | 14 × wait | 354798 | 9994 | 0 | 0 | 4967172 | 4194162 | 0 | 4.8 | 8.1 | 27.6 | 598.0 | 4350.2 | 0 | 0 | 14/14 | 40.8 | 35.2 | - | 30.15 / 24.72 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × wait | 354798 | 9994 | 0 | 0 | 4967172 | 4194162 | 0 | 4.8 | 9.0 | 372.7 | 1671.2 | 6482.9 | 0 | 0 | 14/14 | 40.8 | 35.2 | - | 25.82 / 22.60 |
| best | wake | 14 × wait | 354798 | 9994 | 0 | 0 | 4967172 | 4194162 | 0 | 4.8 | 9.0 | 372.7 | 1671.2 | 6482.9 | 0 | 0 | 14/14 | 40.8 | 35.2 | - | 25.82 / 22.60 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × wait | 3550038 | 99999 | 0 | 0 | 3550038 | 3000437 | 0 | 3.0 | 974.8 | 2850.8 | 7143.4 | 12013.4 | 0 | 0 | 1/1 | 35.1 | 32.5 | - | 55.93 / 71.00 |
| best | wake | 1 × wait | 3550038 | 99999 | 0 | 0 | 3550038 | 3000437 | 0 | 3.0 | 974.8 | 2850.8 | 7143.4 | 12013.4 | 0 | 0 | 1/1 | 35.1 | 32.5 | - | 55.93 / 71.00 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × wait | 3550038 | 100005 | 0 | 0 | 49700532 | 42006118 | 0 | 11.3 | 1703.9 | 12451.8 | 22806.5 | 29198.3 | 0 | 0 | 14/14 | 75.1 | 32.2 | - | 63.17 / 71.01 |
| best | wake | 14 × wait | 3550038 | 100005 | 0 | 0 | 49700532 | 42006118 | 0 | 11.3 | 1703.9 | 12451.8 | 22806.5 | 29198.3 | 0 | 0 | 14/14 | 75.1 | 32.2 | - | 63.17 / 71.01 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × wait | 3550026 | 100015 | 12 | 0 | 49700364 | 42006118 | 0 | 41.0 | 1540.1 | 9175.0 | 21495.8 | 29708.6 | 0 | 0 | 14/14 | 73.3 | 31.8 | - | 81.75 / 67.23 |
| best | wake | 14 × wait | 3550026 | 100015 | 12 | 0 | 49700364 | 42006118 | 0 | 41.0 | 1540.1 | 9175.0 | 21495.8 | 29708.6 | 0 | 0 | 14/14 | 73.3 | 31.8 | - | 81.75 / 67.23 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 0.1 | 0.1 | 0.8 | 34.8 | 1868.6 | 0 | 0 | 1/1 | 70.6 | 35.3 | - | 35.45 / 30.88 |
| best | wake | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 0.1 | 0.1 | 0.8 | 34.8 | 1868.6 | 0 | 0 | 1/1 | 70.6 | 35.3 | - | 35.45 / 30.88 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × busy | 354798 | 9994 | 0 | 0 | 4967172 | 4194162 | 0 | 0.2 | 0.2 | 475.1 | 1802.2 | 8629.0 | 0 | 0 | 14/14 | 521.0 | 34.8 | - | 60.69 / 51.45 |
| best | wake | 14 × busy | 354798 | 9994 | 0 | 0 | 4967172 | 4194162 | 0 | 0.2 | 0.2 | 475.1 | 1802.2 | 8629.0 | 0 | 0 | 14/14 | 521.0 | 34.8 | - | 60.69 / 51.45 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × busy | 354798 | 9994 | 0 | 0 | 4967172 | 4194162 | 0 | 33.8 | 1572.9 | 3899.4 | 7143.4 | 14585.8 | 0 | 0 | 14/14 | 363.2 | 24.4 | - | 80.16 / 84.65 |
| best | wake | 14 × busy | 354798 | 9994 | 0 | 0 | 4967172 | 4194162 | 0 | 33.8 | 1572.9 | 3899.4 | 7143.4 | 14585.8 | 0 | 0 | 14/14 | 363.2 | 24.4 | - | 80.16 / 84.65 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × busy | 3550038 | 99999 | 0 | 0 | 3550038 | 3000437 | 0 | 0.1 | 233.5 | 1916.9 | 4587.5 | 8669.8 | 0 | 0 | 1/1 | 64.8 | 32.3 | - | 65.83 / 70.28 |
| best | wake | 1 × busy | 3550038 | 99999 | 0 | 0 | 3550038 | 3000437 | 0 | 0.1 | 233.5 | 1916.9 | 4587.5 | 8669.8 | 0 | 0 | 1/1 | 64.8 | 32.3 | - | 65.83 / 70.28 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × busy | 3550038 | 99999 | 0 | 0 | 49700532 | 42006118 | 0 | 0.2 | 0.2 | 116.7 | 868.4 | 7598.4 | 0 | 0 | 14/14 | 525.7 | 35.1 | - | 48.34 / 42.50 |
| best | wake | 14 × busy | 3550038 | 99999 | 0 | 0 | 49700532 | 42006118 | 0 | 0.2 | 0.2 | 116.7 | 868.4 | 7598.4 | 0 | 0 | 14/14 | 525.7 | 35.1 | - | 48.34 / 42.50 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × busy | 3550038 | 99999 | 0 | 0 | 49700532 | 42006118 | 0 | 0.2 | 655.4 | 2424.8 | 4980.7 | 14611.7 | 0 | 0 | 14/14 | 444.3 | 29.5 | - | 125.94 / 94.40 |
| best | wake | 14 × busy | 3550038 | 99999 | 0 | 0 | 49700532 | 42006118 | 0 | 0.2 | 655.4 | 2424.8 | 4980.7 | 14611.7 | 0 | 0 | 14/14 | 444.3 | 29.5 | - | 125.94 / 94.40 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 2.3 | 5.0 | 454.7 | 1359.9 | 3833.1 | 0 | 0 | 1/1 | 35.8 | 35.3 | - | 31.42 / 25.34 |
| best | wake | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 2.3 | 5.0 | 454.7 | 1359.9 | 3833.1 | 0 | 0 | 1/1 | 35.8 | 35.3 | - | 31.42 / 25.34 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × wait | 356101 | 10031 | 0 | 0 | 4985414 | 4210766 | 0 | 5.3 | 13.1 | 442.4 | 1327.1 | 3769.8 | 0 | 0 | 14/14 | 41.6 | 35.1 | - | 20.88 / 23.75 |
| best | wake | 14 × wait | 356101 | 10031 | 0 | 0 | 4985414 | 4210766 | 0 | 5.3 | 13.1 | 442.4 | 1327.1 | 3769.8 | 0 | 0 | 14/14 | 41.6 | 35.1 | - | 20.88 / 23.75 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × wait | 356101 | 10031 | 0 | 0 | 4985414 | 4210766 | 0 | 15.4 | 1523.7 | 12976.1 | 36175.9 | 52704.4 | 0 | 0 | 14/14 | 37.4 | 29.3 | - | 60.97 / 73.84 |
| best | wake | 14 × wait | 356101 | 10031 | 0 | 0 | 4985414 | 4210766 | 0 | 15.4 | 1523.7 | 12976.1 | 36175.9 | 52704.4 | 0 | 0 | 14/14 | 37.4 | 29.3 | - | 60.97 / 73.84 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × wait | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 1.2 | 11.1 | 753.7 | 1392.6 | 2578.5 | 0 | 0 | 1/1 | 37.9 | 35.3 | - | 42.28 / 36.91 |
| best | wake | 1 × wait | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 1.2 | 11.1 | 753.7 | 1392.6 | 2578.5 | 0 | 0 | 1/1 | 37.9 | 35.3 | - | 42.28 / 36.91 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × wait | 3553987 | 100110 | 31 | 0 | 49755818 | 42036834 | 0 | 84.0 | 499.7 | 2719.7 | 17825.8 | 29815.1 | 0 | 0 | 14/14 | 89.7 | 34.5 | - | 51.94 / 49.03 |
| best | wake | 14 × wait | 3553987 | 100110 | 31 | 0 | 49755818 | 42036834 | 0 | 84.0 | 499.7 | 2719.7 | 17825.8 | 29815.1 | 0 | 0 | 14/14 | 89.7 | 34.5 | - | 51.94 / 49.03 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × wait | 3554018 | 100111 | 0 | 0 | 49756252 | 42036834 | 0 | 5.1 | 202.8 | 2490.4 | 6357.0 | 14227.1 | 0 | 0 | 14/14 | 80.0 | 34.8 | - | 47.23 / 47.11 |
| best | wake | 14 × wait | 3554018 | 100111 | 0 | 0 | 49756252 | 42036834 | 0 | 5.1 | 202.8 | 2490.4 | 6357.0 | 14227.1 | 0 | 0 | 14/14 | 80.0 | 34.8 | - | 47.23 / 47.11 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 0.1 | 0.1 | 471.0 | 2392.1 | 8322.9 | 0 | 0 | 1/1 | 70.1 | 35.0 | - | 21.21 / 26.89 |
| best | wake | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 0.1 | 0.1 | 471.0 | 2392.1 | 8322.9 | 0 | 0 | 1/1 | 70.1 | 35.0 | - | 21.21 / 26.89 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × busy | 356101 | 10031 | 0 | 0 | 4985414 | 4210766 | 0 | 0.2 | 0.2 | 1065.0 | 3276.8 | 15686.9 | 0 | 0 | 14/14 | 515.9 | 34.5 | - | 34.22 / 45.53 |
| best | wake | 14 × busy | 356101 | 10031 | 0 | 0 | 4985414 | 4210766 | 0 | 0.2 | 0.2 | 1065.0 | 3276.8 | 15686.9 | 0 | 0 | 14/14 | 515.9 | 34.5 | - | 34.22 / 45.53 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × busy | 356101 | 10031 | 0 | 0 | 4985414 | 4210766 | 0 | 0.2 | 475.1 | 2818.0 | 5832.7 | 16339.1 | 0 | 0 | 14/14 | 477.7 | 31.6 | - | 65.04 / 60.60 |
| best | wake | 14 × busy | 356101 | 10031 | 0 | 0 | 4985414 | 4210766 | 0 | 0.2 | 475.1 | 2818.0 | 5832.7 | 16339.1 | 0 | 0 | 14/14 | 477.7 | 31.6 | - | 65.04 / 60.60 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 1 × busy | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 0.1 | 1179.6 | 2949.1 | 5898.2 | 13290.3 | 0 | 0 | 1/1 | 48.5 | 23.1 | - | 82.49 / 78.65 |
| best | wake | 1 × busy | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 0.1 | 1179.6 | 2949.1 | 5898.2 | 13290.3 | 0 | 0 | 1/1 | 48.5 | 23.1 | - | 82.49 / 78.65 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × busy | 3554018 | 100112 | 0 | 0 | 49756252 | 42036834 | 0 | 0.2 | 8.2 | 1261.6 | 3145.7 | 9931.5 | 0 | 0 | 14/14 | 500.2 | 33.2 | - | 63.12 / 58.84 |
| best | wake | 14 × busy | 3554018 | 100112 | 0 | 0 | 49756252 | 42036834 | 0 | 0.2 | 8.2 | 1261.6 | 3145.7 | 9931.5 | 0 | 0 | 14/14 | 500.2 | 33.2 | - | 63.12 / 58.84 |
# quiver-bench fanout wake

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: wake (the kernel's wake-up alone, in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | wake | 14 × busy | 3554018 | 100103 | 0 | 0 | 49756252 | 42036834 | 0 | 0.1 | 0.2 | 1.0 | 999.4 | 4327.0 | 0 | 0 | 14/14 | 515.9 | 34.6 | - | 49.03 / 44.44 |
| best | wake | 14 × busy | 3554018 | 100103 | 0 | 0 | 49756252 | 42036834 | 0 | 0.1 | 0.2 | 1.0 | 999.4 | 4327.0 | 0 | 0 | 14/14 | 515.9 | 34.6 | - | 49.03 / 44.44 |
