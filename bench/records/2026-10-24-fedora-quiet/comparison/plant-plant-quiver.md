# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 10000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 16 over 4 threads, which poll without waiting; window 3 s after 1 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 4 × busy | 45029 | 10005 | 0 | 0 | 720464 | 480528 | 0 | 0.6 | 0.9 | 1.9 | 6.8 | 72.6 | 0 | 0 | 16/16 | 22.5 | 4.5 | - | 2.11 / 2.35 |
| best | quiver | 4 × busy | 45029 | 10005 | 0 | 0 | 720464 | 480528 | 0 | 0.6 | 0.9 | 1.9 | 6.8 | 72.6 | 0 | 0 | 16/16 | 22.5 | 4.5 | - | 2.11 / 2.35 |

Run 1's profile:

| profile: the window, 3.0 s, 30030 messages, 480480 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 14.98G | 498.7k | 31.17k | 99.75k | 24.94k | - | - | - |
| waiting for a CPU, ns | - | - | - | 6.633 | 1.685 | - | - | - |
| allocations | 540.8k | 18.01 | 1.126 | 2.010 | 1.000 | - | - | - |
| reallocations | 2.000 | 0.0000666 | 0.00000416 | 0.0000666 | 0 | - | - | - |
| reallocations that moved their block | 2.000 | 0.0000666 | 0.00000416 | 0.0000666 | 0 | - | - | - |
| frees | 510.5k | 17.00 | 1.062 | 1.000 | 1.000 | - | - | - |
| bytes allocated | 18.35M | 611.0 | 38.19 | 483.0 | 8.000 | - | - | - |
| minor page faults | 3337 | 0.111 | 0.00695 | 0.111 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 81.00 | 0.00270 | 0.000169 | 0.000799 | 0.000119 | - | - | - |
| cycles | - | - | - | 507.3k | 120.7k | - | - | - |
| instructions | - | - | - | 292.6k | 267.3k | - | - | - |
| cache references | - | - | - | 24.04 | 432.1 | - | - | - |
| cache misses | - | - | - | 0.0808 | 0.00435 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00509 | 0.000285 | - | - | - |
| data TLB read misses | - | - | - | 0.0811 | 0.0191 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 24.33M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 24.33M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 12.65M bytes, holding 40044 messages: 316.0 bytes a message
