# quiver-bench soak quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: 08d3dc97048cdebf949535706daafc0603b19775
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 1000000 messages
- subscribers: 1000 over 14 threads, which wait when there's nothing to take; window 3600 s after 5 s of warm-up; seed 0x5eed

| minute | published | deliveries are counted at the end |
|---|---|---|
| 1 | 5999538 | |
| 2 | 11997775 | |
| 3 | 17999269 | |
| 4 | 23999145 | |
| 5 | 30000182 | |
| 6 | 36000002 | |
| 7 | 41998599 | |
| 8 | 47997111 | |
| 9 | 53996910 | |
| 10 | 59997956 | |
| 11 | 65997101 | |
| 12 | 71999189 | |
| 13 | 77996348 | |
| 14 | 83994038 | |
| 15 | 89991832 | |
| 16 | 95989555 | |
| 17 | 101990587 | |
| 18 | 107990879 | |
| 19 | 113989379 | |
| 20 | 119991312 | |
| 21 | 125991037 | |
| 22 | 131986706 | |
| 23 | 137986460 | |
| 24 | 143987246 | |
| 25 | 149989351 | |
| 26 | 155989380 | |
| 27 | 161988712 | |
| 28 | 167987519 | |
| 29 | 173988970 | |
| 30 | 179989833 | |
| 31 | 185990874 | |
| 32 | 191988538 | |
| 33 | 197988077 | |
| 34 | 203987432 | |
| 35 | 209986014 | |
| 36 | 215983618 | |
| 37 | 221988891 | |
| 38 | 227991865 | |
| 39 | 233992780 | |
| 40 | 239990605 | |
| 41 | 245989973 | |
| 42 | 251990366 | |
| 43 | 257991882 | |
| 44 | 263994621 | |
| 45 | 269996240 | |
| 46 | 275992444 | |
| 47 | 281994642 | |
| 48 | 287992950 | |
| 49 | 293996285 | |
| 50 | 299994485 | |
| 51 | 305991256 | |
| 52 | 311991289 | |
| 53 | 317989380 | |
| 54 | 323983293 | |
| 55 | 329981719 | |
| 56 | 335981604 | |
| 57 | 341987055 | |
| 58 | 347984907 | |
| 59 | 353983270 | |
| 60 | 359978023 | |

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| soak | quiver | 14 × wait | 360529149 | 99994 | 0 | 0 | 360529149000 | 359979959000 | 0 | 17.2 | 389.1 | 8650.8 | 75497.5 | 547103.9 | 0 | 0 | 1000/1000 | 23476.1 | 3437.7 | - | 35.85 / 61.57 |

frames lost: 0; doubled: 0; subscribers that kept up (no lag notice): 1000 of 1000

| profile: the window, 3600.0 s, 359979966 messages, 359979966813 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 23.44T | 65.11k | 65.11 | 9535 | 55.57 | - | - | - |
| waiting for a CPU, ns | - | - | - | 370.9 | 31.79 | - | - | - |
| allocations | 2.571G | 7.142 | 0.00714 | 4.010 | 0.00313 | - | - | - |
| reallocations | 1.080G | 3.000 | 0.00300 | 3.000 | 0 | - | - | - |
| reallocations that moved their block | 901.2M | 2.504 | 0.00250 | 2.504 | 0 | - | - | - |
| frees | 2.571G | 7.141 | 0.00714 | 4.008 | 0.00313 | - | - | - |
| bytes allocated | 437.9G | 1216 | 1.216 | 1041 | 0.175 | - | - | - |
| minor page faults | 55.94k | 0.000155 | 0.000000155 | 0.000155 | 0.000000000 | - | - | - |
| major page faults | 477.0 | 0.00000133 | 0.000000001 | 0.00000118 | 0.000000000 | - | - | - |
| voluntary context switches | 1.432G | 3.978 | 0.00398 | 0.00504 | 0.00397 | - | - | - |
| involuntary context switches | 80.57M | 0.224 | 0.000224 | 0.00681 | 0.000217 | - | - | - |
| cycles | - | - | - | 40.89k | 216.3 | - | - | - |
| instructions | - | - | - | 39.78k | 392.6 | - | - | - |
| cache references | - | - | - | 351.8 | 4.172 | - | - | - |
| cache misses | - | - | - | 28.77 | 0.729 | - | - | - |
| L1 data cache read misses | - | - | - | 74.17 | 2.034 | - | - | - |
| last-level cache read misses | - | - | - | refused | refused | - | - | - |
| data TLB read misses | - | - | - | 2.155 | 0.0237 | - | - | - |
| bytes read from storage | 194.0M | 0.539 | 0.000539 | - | - | - | - | - |
| bytes written to storage | 245.8k | 0.000683 | 0.000000683 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 459.9M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 459.7M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- last-level cache read misses, the publisher's thread: refused: cpu's PMU has no such event (No such file or directory (os error 2))
- last-level cache read misses, the subscribers' threads: refused: cpu's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 316.1M bytes, holding 1000000 messages: 316.1 bytes a message
