# quiver-bench soak quiver

- machine: AMD Ryzen 9 9950X3D2 16-Core Processor, 16 CPUs this process may use, Linux 7.2.9-2-cachyos, 130859532 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
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
| soak | quiver | 14 × wait | 360529149 | 99994 | 0 | 0 | 360529149000 | 359979959000 | 0 | 33.8 | 1409.0 | 19398.7 | 241172.5 | 9943588.1 | 0 | 0 | 1000/1000 | 20848.3 | 3351.2 | - | 39.13 / 61.07 |

frames lost: 0; doubled: 0; subscribers that kept up (no lag notice): 1000 of 1000

| profile: the window, 3600.0 s, 359979956 messages, 359978243156 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 20.81T | 57.80k | 57.80 | 9295 | 48.51 | - | - | - |
| waiting for a CPU, ns | - | - | - | 655.7 | 56.91 | - | - | - |
| allocations | 723.4M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 1.000 | 0.000000003 | 0.000000000 | 0.000000003 | 0 | - | - | - |
| reallocations that moved their block | 1.000 | 0.000000003 | 0.000000000 | 0.000000003 | 0 | - | - | - |
| frees | 722.9M | 2.008 | 0.00201 | 2.008 | 0 | - | - | - |
| bytes allocated | 173.9G | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 56.87k | 0.000158 | 0.000000158 | 0.000156 | 0.000000002 | - | - | - |
| major page faults | 479.0 | 0.00000133 | 0.000000001 | 0.000000983 | 0.000000000 | - | - | - |
| voluntary context switches | 880.1M | 2.445 | 0.00245 | 0.000189 | 0.00244 | - | - | - |
| involuntary context switches | 101.4M | 0.282 | 0.000282 | 0.00874 | 0.000273 | - | - | - |
| cycles | - | - | - | 39.86k | 201.6 | - | - | - |
| instructions | - | - | - | 36.82k | 381.2 | - | - | - |
| cache references | - | - | - | 263.1 | 4.111 | - | - | - |
| cache misses | - | - | - | 23.62 | 0.424 | - | - | - |
| L1 data cache read misses | - | - | - | 65.21 | 2.265 | - | - | - |
| last-level cache read misses | - | - | - | refused | refused | - | - | - |
| data TLB read misses | - | - | - | 1.595 | 0.0415 | - | - | - |
| bytes read from storage | 325.2M | 0.903 | 0.000903 | - | - | - | - | - |
| bytes written to storage | 245.8k | 0.000683 | 0.000000683 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 466.7M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 465.4M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- last-level cache read misses, the publisher's thread: refused: cpu's PMU has no such event (No such file or directory (os error 2))
- last-level cache read misses, the subscribers' threads: refused: cpu's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 316.1M bytes, holding 1000000 messages: 316.1 bytes a message
