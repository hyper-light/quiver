# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × wait | 354584 | 9988 | 6 | 0 | 354584 | 299497 | 0 | 3.2 | 30.5 | 1622.0 | 5308.4 | 11220.9 | 0 | 0 | 1/1 | 33.8 | 33.1 | - | 56.46 / 56.67 |
| best | quiver | 1 × wait | 354584 | 9988 | 6 | 0 | 354584 | 299497 | 0 | 3.2 | 30.5 | 1622.0 | 5308.4 | 11220.9 | 0 | 0 | 1/1 | 33.8 | 33.1 | - | 56.46 / 56.67 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 30.5 | 622.6 | 2080.8 | 4259.8 | 10968.3 | 0 | 0 | 100/100 | 45.1 | 32.2 | - | 47.48 / 94.45 |
| best | quiver | 14 × wait | 354590 | 9988 | 0 | 0 | 35459000 | 29949700 | 0 | 30.5 | 622.6 | 2080.8 | 4259.8 | 10968.3 | 0 | 0 | 100/100 | 45.1 | 32.2 | - | 47.48 / 94.45 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × wait | 354590 | 9987 | 0 | 0 | 354590000 | 299497000 | 0 | 28.4 | 729.1 | 3244.0 | 6357.0 | 14419.5 | 0 | 0 | 1000/1000 | 59.0 | 32.4 | - | 75.70 / 65.36 |
| best | quiver | 14 × wait | 354590 | 9987 | 0 | 0 | 354590000 | 299497000 | 0 | 28.4 | 729.1 | 3244.0 | 6357.0 | 14419.5 | 0 | 0 | 1000/1000 | 59.0 | 32.4 | - | 75.70 / 65.36 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 5.7 | 950.3 | 2457.6 | 5636.1 | 13100.8 | 0 | 0 | 1/1 | 36.1 | 32.9 | - | 46.75 / 46.67 |
| best | quiver | 1 × wait | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 5.7 | 950.3 | 2457.6 | 5636.1 | 13100.8 | 0 | 0 | 1/1 | 36.1 | 32.9 | - | 46.75 / 46.67 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × wait | 3547540 | 99916 | 3 | 0 | 354754000 | 299827400 | 0 | 28.7 | 335.9 | 1490.9 | 3440.6 | 18190.5 | 0 | 0 | 100/100 | 85.6 | 31.2 | - | 75.60 / 77.06 |
| best | quiver | 14 × wait | 3547540 | 99916 | 3 | 0 | 354754000 | 299827400 | 0 | 28.7 | 335.9 | 1490.9 | 3440.6 | 18190.5 | 0 | 0 | 100/100 | 85.6 | 31.2 | - | 75.60 / 77.06 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × wait | 3547543 | 99934 | 0 | 0 | 3547543000 | 2998274000 | 0 | 141.3 | 5505.0 | 63963.1 | 201326.6 | 395209.8 | 0 | 0 | 1000/1000 | 159.1 | 27.6 | - | 101.52 / 100.03 |
| best | quiver | 14 × wait | 3547543 | 99934 | 0 | 0 | 3547543000 | 2998274000 | 0 | 141.3 | 5505.0 | 63963.1 | 201326.6 | 395209.8 | 0 | 0 | 1000/1000 | 159.1 | 27.6 | - | 101.52 / 100.03 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 0.7 | 1.9 | 827.4 | 1720.3 | 5356.7 | 0 | 0 | 1/1 | 67.7 | 33.8 | - | 142.48 / 106.85 |
| best | quiver | 1 × busy | 354590 | 9988 | 0 | 0 | 354590 | 299497 | 0 | 0.7 | 1.9 | 827.4 | 1720.3 | 5356.7 | 0 | 0 | 1/1 | 67.7 | 33.8 | - | 142.48 / 106.85 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × busy | 354590 | 9989 | 0 | 0 | 35459000 | 29949700 | 0 | 1.0 | 1228.8 | 3047.4 | 5374.0 | 15477.0 | 0 | 0 | 100/100 | 415.7 | 27.1 | - | 103.86 / 104.91 |
| best | quiver | 14 × busy | 354590 | 9989 | 0 | 0 | 35459000 | 29949700 | 0 | 1.0 | 1228.8 | 3047.4 | 5374.0 | 15477.0 | 0 | 0 | 100/100 | 415.7 | 27.1 | - | 103.86 / 104.91 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × busy | 354590 | 9987 | 0 | 0 | 354590000 | 299497000 | 0 | 2.6 | 233.5 | 1572.9 | 3506.2 | 12016.0 | 0 | 0 | 1000/1000 | 474.9 | 31.5 | - | 96.28 / 87.72 |
| best | quiver | 14 × busy | 354590 | 9987 | 0 | 0 | 354590000 | 299497000 | 0 | 2.6 | 233.5 | 1572.9 | 3506.2 | 12016.0 | 0 | 0 | 1000/1000 | 474.9 | 31.5 | - | 96.28 / 87.72 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 0.4 | 0.9 | 1146.9 | 2523.1 | 4557.3 | 0 | 0 | 1/1 | 68.3 | 34.2 | - | 62.37 / 66.02 |
| best | quiver | 1 × busy | 3547543 | 99928 | 0 | 0 | 3547543 | 2998274 | 0 | 0.4 | 0.9 | 1146.9 | 2523.1 | 4557.3 | 0 | 0 | 1/1 | 68.3 | 34.2 | - | 62.37 / 66.02 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × busy | 3547479 | 99929 | 64 | 0 | 354747900 | 299827400 | 0 | 0.8 | 194.6 | 1507.3 | 3244.0 | 16248.9 | 0 | 0 | 100/100 | 484.7 | 32.5 | - | 72.73 / 69.67 |
| best | quiver | 14 × busy | 3547479 | 99929 | 64 | 0 | 354747900 | 299827400 | 0 | 0.8 | 194.6 | 1507.3 | 3244.0 | 16248.9 | 0 | 0 | 100/100 | 484.7 | 32.5 | - | 72.73 / 69.67 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × busy | 3547543 | 99930 | 0 | 0 | 3547543000 | 2998274000 | 0 | 3.1 | 454.7 | 2326.5 | 4915.2 | 26361.3 | 0 | 0 | 1000/1000 | 477.1 | 31.7 | - | 73.47 / 76.32 |
| best | quiver | 14 × busy | 3547543 | 99930 | 0 | 0 | 3547543000 | 2998274000 | 0 | 3.1 | 454.7 | 2326.5 | 4915.2 | 26361.3 | 0 | 0 | 1000/1000 | 477.1 | 31.7 | - | 73.47 / 76.32 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 4.6 | 290.8 | 1278.0 | 2293.8 | 6953.4 | 0 | 0 | 1/1 | 35.7 | 34.7 | - | 74.50 / 62.13 |
| best | quiver | 1 × wait | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 4.6 | 290.8 | 1278.0 | 2293.8 | 6953.4 | 0 | 0 | 1/1 | 35.7 | 34.7 | - | 74.50 / 62.13 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 11.4 | 21.5 | 73.7 | 1007.6 | 7219.4 | 0 | 0 | 100/100 | 50.3 | 35.3 | - | 43.14 / 32.68 |
| best | quiver | 14 × wait | 354798 | 9994 | 0 | 0 | 35479800 | 29958300 | 0 | 11.4 | 21.5 | 73.7 | 1007.6 | 7219.4 | 0 | 0 | 100/100 | 50.3 | 35.3 | - | 43.14 / 32.68 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × wait | 354798 | 9994 | 0 | 0 | 354798000 | 299583000 | 0 | 15.5 | 52.7 | 1392.6 | 3080.2 | 10065.0 | 0 | 0 | 1000/1000 | 63.6 | 34.9 | - | 24.72 / 51.31 |
| best | quiver | 14 × wait | 354798 | 9994 | 0 | 0 | 354798000 | 299583000 | 0 | 15.5 | 52.7 | 1392.6 | 3080.2 | 10065.0 | 0 | 0 | 1000/1000 | 63.6 | 34.9 | - | 24.72 / 51.31 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × wait | 3550038 | 99999 | 0 | 0 | 3550038 | 3000437 | 0 | 2.9 | 622.6 | 1458.2 | 2621.4 | 4870.3 | 0 | 0 | 1/1 | 37.5 | 34.4 | - | 22.60 / 37.33 |
| best | quiver | 1 × wait | 3550038 | 99999 | 0 | 0 | 3550038 | 3000437 | 0 | 2.9 | 622.6 | 1458.2 | 2621.4 | 4870.3 | 0 | 0 | 1/1 | 37.5 | 34.4 | - | 22.60 / 37.33 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × wait | 3550038 | 99987 | 0 | 0 | 355003800 | 300043700 | 0 | 13.6 | 327.7 | 1687.6 | 3768.3 | 16048.8 | 0 | 0 | 100/100 | 103.3 | 33.6 | - | 71.00 / 63.70 |
| best | quiver | 14 × wait | 3550038 | 99987 | 0 | 0 | 355003800 | 300043700 | 0 | 13.6 | 327.7 | 1687.6 | 3768.3 | 16048.8 | 0 | 0 | 100/100 | 103.3 | 33.6 | - | 71.00 / 63.70 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × wait | 3550038 | 99990 | 0 | 0 | 3550038000 | 3000437000 | 0 | 15.9 | 520.2 | 2588.7 | 6357.0 | 35668.0 | 0 | 0 | 1000/1000 | 201.8 | 34.3 | - | 71.01 / 65.36 |
| best | quiver | 14 × wait | 3550038 | 99990 | 0 | 0 | 3550038000 | 3000437000 | 0 | 15.9 | 520.2 | 2588.7 | 6357.0 | 35668.0 | 0 | 0 | 1000/1000 | 201.8 | 34.3 | - | 71.01 / 65.36 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 0.3 | 0.6 | 2.4 | 1065.0 | 8030.6 | 0 | 0 | 1/1 | 70.5 | 35.3 | - | 67.23 / 46.84 |
| best | quiver | 1 × busy | 354798 | 9994 | 0 | 0 | 354798 | 299583 | 0 | 0.3 | 0.6 | 2.4 | 1065.0 | 8030.6 | 0 | 0 | 1/1 | 70.5 | 35.3 | - | 67.23 / 46.84 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × busy | 354790 | 9994 | 8 | 0 | 35479000 | 29958300 | 0 | 0.9 | 1343.5 | 4849.7 | 9175.0 | 30293.5 | 0 | 0 | 100/100 | 442.2 | 28.4 | - | 30.88 / 49.40 |
| best | quiver | 14 × busy | 354790 | 9994 | 8 | 0 | 35479000 | 29958300 | 0 | 0.9 | 1343.5 | 4849.7 | 9175.0 | 30293.5 | 0 | 0 | 100/100 | 442.2 | 28.4 | - | 30.88 / 49.40 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × busy | 354798 | 9994 | 0 | 0 | 354798000 | 299583000 | 0 | 2.6 | 622.6 | 3276.8 | 7536.6 | 20670.1 | 0 | 0 | 1000/1000 | 477.4 | 31.9 | - | 51.45 / 48.17 |
| best | quiver | 14 × busy | 354798 | 9994 | 0 | 0 | 354798000 | 299583000 | 0 | 2.6 | 622.6 | 3276.8 | 7536.6 | 20670.1 | 0 | 0 | 1000/1000 | 477.4 | 31.9 | - | 51.45 / 48.17 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × busy | 3550038 | 99989 | 0 | 0 | 3550038 | 3000437 | 0 | 0.5 | 0.8 | 479.2 | 1589.2 | 4192.6 | 0 | 0 | 1/1 | 67.5 | 33.7 | - | 84.65 / 68.44 |
| best | quiver | 1 × busy | 3550038 | 99989 | 0 | 0 | 3550038 | 3000437 | 0 | 0.5 | 0.8 | 479.2 | 1589.2 | 4192.6 | 0 | 0 | 1/1 | 67.5 | 33.7 | - | 84.65 / 68.44 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × busy | 3550038 | 100004 | 0 | 0 | 355003800 | 300043700 | 0 | 1.1 | 1359.9 | 3506.2 | 6553.6 | 20841.4 | 0 | 0 | 100/100 | 395.7 | 26.0 | - | 73.78 / 87.33 |
| best | quiver | 14 × busy | 3550038 | 100004 | 0 | 0 | 355003800 | 300043700 | 0 | 1.1 | 1359.9 | 3506.2 | 6553.6 | 20841.4 | 0 | 0 | 100/100 | 395.7 | 26.0 | - | 73.78 / 87.33 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eee

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × busy | 3550038 | 99999 | 0 | 0 | 3550038000 | 3000437000 | 0 | 2.5 | 5.8 | 66.6 | 655.4 | 4743.6 | 0 | 0 | 1000/1000 | 528.5 | 35.3 | - | 42.50 / 37.67 |
| best | quiver | 14 × busy | 3550038 | 99999 | 0 | 0 | 3550038000 | 3000437000 | 0 | 2.5 | 5.8 | 66.6 | 655.4 | 4743.6 | 0 | 0 | 1000/1000 | 528.5 | 35.3 | - | 42.50 / 37.67 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 2.4 | 4.0 | 67.6 | 1065.0 | 3412.9 | 0 | 0 | 1/1 | 35.8 | 35.4 | - | 25.34 / 23.83 |
| best | quiver | 1 × wait | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 2.4 | 4.0 | 67.6 | 1065.0 | 3412.9 | 0 | 0 | 1/1 | 35.8 | 35.4 | - | 25.34 / 23.83 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 25.6 | 184.3 | 1081.3 | 2129.9 | 6814.3 | 0 | 0 | 100/100 | 49.5 | 34.1 | - | 23.75 / 37.83 |
| best | quiver | 14 × wait | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 25.6 | 184.3 | 1081.3 | 2129.9 | 6814.3 | 0 | 0 | 100/100 | 49.5 | 34.1 | - | 23.75 / 37.83 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × wait | 356094 | 10031 | 7 | 0 | 356094000 | 300769000 | 0 | 45.1 | 364.5 | 1671.2 | 3571.7 | 12933.5 | 0 | 0 | 1000/1000 | 56.8 | 31.6 | - | 73.84 / 77.62 |
| best | quiver | 14 × wait | 356094 | 10031 | 7 | 0 | 356094000 | 300769000 | 0 | 45.1 | 364.5 | 1671.2 | 3571.7 | 12933.5 | 0 | 0 | 1000/1000 | 56.8 | 31.6 | - | 73.84 / 77.62 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × wait | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 1.6 | 4.6 | 454.7 | 1261.6 | 3691.9 | 0 | 0 | 1/1 | 38.2 | 35.3 | - | 36.91 / 36.97 |
| best | quiver | 1 × wait | 3554018 | 100111 | 0 | 0 | 3554018 | 3002631 | 0 | 1.6 | 4.6 | 454.7 | 1261.6 | 3691.9 | 0 | 0 | 1/1 | 38.2 | 35.3 | - | 36.91 / 36.97 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × wait | 3554018 | 100112 | 0 | 0 | 355401800 | 300263100 | 0 | 8.8 | 23.6 | 892.9 | 3113.0 | 16835.1 | 0 | 0 | 100/100 | 120.4 | 34.4 | - | 49.03 / 42.05 |
| best | quiver | 14 × wait | 3554018 | 100112 | 0 | 0 | 355401800 | 300263100 | 0 | 8.8 | 23.6 | 892.9 | 3113.0 | 16835.1 | 0 | 0 | 100/100 | 120.4 | 34.4 | - | 49.03 / 42.05 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × wait | 3554018 | 100111 | 0 | 0 | 3554018000 | 3002631000 | 0 | 10.1 | 63.5 | 1818.6 | 5898.2 | 28923.7 | 0 | 0 | 1000/1000 | 223.0 | 34.8 | - | 47.11 / 44.34 |
| best | quiver | 14 × wait | 3554018 | 100111 | 0 | 0 | 3554018000 | 3002631000 | 0 | 10.1 | 63.5 | 1818.6 | 5898.2 | 28923.7 | 0 | 0 | 1000/1000 | 223.0 | 34.8 | - | 47.11 / 44.34 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 0.3 | 0.4 | 1.0 | 5.4 | 1121.9 | 0 | 0 | 1/1 | 70.8 | 35.4 | - | 26.89 / 22.54 |
| best | quiver | 1 × busy | 356101 | 10031 | 0 | 0 | 356101 | 300769 | 0 | 0.3 | 0.4 | 1.0 | 5.4 | 1121.9 | 0 | 0 | 1/1 | 70.8 | 35.4 | - | 26.89 / 22.54 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 0.9 | 1.1 | 1.9 | 8.4 | 1728.5 | 0 | 0 | 100/100 | 531.1 | 35.4 | - | 45.53 / 37.13 |
| best | quiver | 14 × busy | 356101 | 10031 | 0 | 0 | 35610100 | 30076900 | 0 | 0.9 | 1.1 | 1.9 | 8.4 | 1728.5 | 0 | 0 | 100/100 | 531.1 | 35.4 | - | 45.53 / 37.13 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × busy | 356101 | 10031 | 0 | 0 | 356101000 | 300769000 | 0 | 3.3 | 1769.5 | 5111.8 | 9699.3 | 52674.5 | 0 | 0 | 1000/1000 | 394.1 | 26.2 | - | 60.60 / 73.76 |
| best | quiver | 14 × busy | 356101 | 10031 | 0 | 0 | 356101000 | 300769000 | 0 | 3.3 | 1769.5 | 5111.8 | 9699.3 | 52674.5 | 0 | 0 | 1000/1000 | 394.1 | 26.2 | - | 60.60 / 73.76 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1 over 1 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 1 × busy | 3553987 | 100106 | 31 | 0 | 3553987 | 3002631 | 0 | 20.0 | 1998.8 | 5111.8 | 8192.0 | 11716.0 | 0 | 0 | 1/1 | 48.3 | 23.0 | - | 78.65 / 75.83 |
| best | quiver | 1 × busy | 3553987 | 100106 | 31 | 0 | 3553987 | 3002631 | 0 | 20.0 | 1998.8 | 5111.8 | 8192.0 | 11716.0 | 0 | 0 | 1/1 | 48.3 | 23.0 | - | 78.65 / 75.83 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 100 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × busy | 3554018 | 100111 | 0 | 0 | 355401800 | 300263100 | 0 | 0.8 | 1.1 | 4.7 | 262.1 | 4121.3 | 0 | 0 | 100/100 | 530.2 | 35.3 | - | 58.84 / 45.40 |
| best | quiver | 14 × busy | 3554018 | 100111 | 0 | 0 | 355401800 | 300263100 | 0 | 0.8 | 1.1 | 4.7 | 262.1 | 4121.3 | 0 | 0 | 100/100 | 530.2 | 35.3 | - | 58.84 / 45.40 |
# quiver-bench fanout quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: d8e5d54360b055bf212e6fc68b8c76040b085239
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 14 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eef

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 14 × busy | 3554018 | 100100 | 0 | 0 | 3554018000 | 3002631000 | 0 | 2.5 | 6.0 | 92.2 | 1359.9 | 9587.6 | 0 | 0 | 1000/1000 | 528.5 | 35.2 | - | 44.44 / 38.44 |
| best | quiver | 14 × busy | 3554018 | 100100 | 0 | 0 | 3554018000 | 3002631000 | 0 | 2.5 | 6.0 | 92.2 | 1359.9 | 9587.6 | 0 | 0 | 1000/1000 | 528.5 | 35.2 | - | 44.44 / 38.44 |
