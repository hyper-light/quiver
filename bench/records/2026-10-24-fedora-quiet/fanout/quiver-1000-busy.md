# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which poll without waiting; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 3547543 | 99929 | 0 | 0 | 3547543000 | 2998274000 | 0 | 2.4 | 6.2 | 133.1 | 210.9 | 4554.4 | 0 | 0 | 1000/1000 | 814.0 | 35.4 | - | 1.00 / 10.73 |
| 2 | quiver | 22 × busy | 3550038 | 100000 | 0 | 0 | 3550038000 | 3000437000 | 0 | 2.9 | 8.1 | 137.2 | 217.1 | 38081.3 | 0 | 0 | 1000/1000 | 814.1 | 35.4 | - | 10.73 / 16.68 |
| 3 | quiver | 22 × busy | 3554018 | 100112 | 0 | 0 | 3554018000 | 3002631000 | 0 | 2.8 | 9.6 | 174.1 | 9437.2 | 37295.1 | 0 | 0 | 1000/1000 | 813.9 | 35.3 | - | 16.68 / 19.83 |
| best | quiver | 22 × busy | 3547543 | 99929 | 0 | 0 | 3547543000 | 2998274000 | 0 | 2.4 | 6.2 | 133.1 | 210.9 | 4554.4 | 0 | 0 | 1000/1000 | 814.0 | 35.4 | - | 1.00 / 10.73 |

Run 1's profile:

| profile: the window, 30.0 s, 2998278 messages, 2998278000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.3G | 229.6k | 229.6 | 9975 | 219.6 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.387 | 0.00735 | - | - | - |
| allocations | 6.026M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.026M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.448G | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 16.00 | 0.00000534 | 0.000000005 | 0.00000534 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 1879 | 0.000627 | 0.000000627 | 0.0000357 | 0.000000591 | - | - | - |
| cycles | - | - | - | 44.04k | 835.9 | - | - | - |
| instructions | - | - | - | 24.90k | 1966 | - | - | - |
| cache references | - | - | - | 24.10 | 2.533 | - | - | - |
| cache misses | - | - | - | 4.666 | 0.000362 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.104 | 0.00000185 | - | - | - |
| data TLB read misses | - | - | - | 0.0317 | 0.00176 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.66M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.66M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message

Run 2's profile:

| profile: the window, 30.0 s, 3000525 messages, 3000525000 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 688.0G | 229.3k | 229.3 | 9970 | 219.3 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.0983 | 0.0298 | - | - | - |
| allocations | 6.030M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.030M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.449G | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 2246 | 0.000749 | 0.000000749 | 0.0000137 | 0.000000735 | - | - | - |
| cycles | - | - | - | 33.60k | 785.5 | - | - | - |
| instructions | - | - | - | 32.67k | 1778 | - | - | - |
| cache references | - | - | - | 22.07 | 2.372 | - | - | - |
| cache misses | - | - | - | 4.657 | 0.00313 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.00328 | 0.00000838 | - | - | - |
| data TLB read misses | - | - | - | 0.0354 | 0.00171 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 43.97M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 43.97M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message

Run 3's profile:

| profile: the window, 30.0 s, 3002592 messages, 3002592225 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 687.7G | 229.0k | 229.0 | 9937 | 219.1 | - | - | - |
| waiting for a CPU, ns | - | - | - | 21.66 | 0.0720 | - | - | - |
| allocations | 6.034M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.034M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.450G | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 0 | 0 | 0 | 0 | 0 | - | - | - |
| involuntary context switches | 2428 | 0.000809 | 0.000000809 | 0.000106 | 0.000000703 | - | - | - |
| cycles | - | - | - | 39.08k | 762.9 | - | - | - |
| instructions | - | - | - | 22.55k | 1725 | - | - | - |
| cache references | - | - | - | 27.67 | 2.287 | - | - | - |
| cache misses | - | - | - | 4.926 | 0.000316 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.111 | 0.00000357 | - | - | - |
| data TLB read misses | - | - | - | 0.0264 | 0.00136 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 44.20M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 44.20M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message

Run 1's budgets:

| budget | per | budget | this run | met |
|---|---|---|---|---|
| subscribers_allocations | delivery | 0 | 0 | yes |
| subscribers_reallocations | delivery | 0 | 0 | yes |
| publisher_allocations | message | 2.010 | 2.010 | yes |
| publisher_reallocations | message | 0 | 0 | yes |
| process_minor_faults | message | 0.0001000 | 0.00000534 | yes |
| subscribers_llc_read_misses | delivery | 0.0000413 | 0.00000185 | yes |

Run 2's budgets:

| budget | per | budget | this run | met |
|---|---|---|---|---|
| subscribers_allocations | delivery | 0 | 0 | yes |
| subscribers_reallocations | delivery | 0 | 0 | yes |
| publisher_allocations | message | 2.010 | 2.010 | yes |
| publisher_reallocations | message | 0 | 0 | yes |
| process_minor_faults | message | 0.0001000 | 0 | yes |
| subscribers_llc_read_misses | delivery | 0.0000413 | 0.00000838 | yes |

Run 3's budgets:

| budget | per | budget | this run | met |
|---|---|---|---|---|
| subscribers_allocations | delivery | 0 | 0 | yes |
| subscribers_reallocations | delivery | 0 | 0 | yes |
| publisher_allocations | message | 2.010 | 2.010 | yes |
| publisher_reallocations | message | 0 | 0 | yes |
| process_minor_faults | message | 0.0001000 | 0 | yes |
| subscribers_llc_read_misses | delivery | 0.0000413 | 0.00000357 | yes |
