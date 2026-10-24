# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 88409ddda5307abe166f310760bbe3bd51f27878
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 30 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 3547541 | 99934 | 2 | 0 | 3547541000 | 2998274000 | 0 | 10.8 | 36.9 | 122.9 | 202.8 | 3079.3 | 0 | 0 | 1000/1000 | 404.0 | 35.2 | - | 0.99 / 5.91 |
| 2 | quiver | 22 × wait | 3550038 | 100000 | 0 | 0 | 3550038000 | 3000437000 | 0 | 11.0 | 38.4 | 126.0 | 200.7 | 5529.3 | 0 | 0 | 1000/1000 | 405.9 | 35.3 | - | 5.91 / 9.54 |
| 3 | quiver | 22 × wait | 3554018 | 100111 | 0 | 0 | 3554018000 | 3002631000 | 0 | 10.1 | 29.7 | 118.8 | 188.4 | 4000.7 | 0 | 0 | 1000/1000 | 414.1 | 35.3 | - | 9.54 / 13.58 |
| best | quiver | 22 × wait | 3554018 | 100111 | 0 | 0 | 3554018000 | 3002631000 | 0 | 10.1 | 29.7 | 118.8 | 188.4 | 4000.7 | 0 | 0 | 1000/1000 | 414.1 | 35.3 | - | 9.54 / 13.58 |

Run 1's profile:

| profile: the window, 30.0 s, 2998274 messages, 2998273895 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 343.0G | 114.4k | 114.4 | 9936 | 104.5 | - | - | - |
| waiting for a CPU, ns | - | - | - | 1.506 | 0.254 | - | - | - |
| allocations | 6.026M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.026M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.448G | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 25.46M | 8.493 | 0.00849 | 0.0127 | 0.00848 | - | - | - |
| involuntary context switches | 9500 | 0.00317 | 0.00000317 | 0.000249 | 0.00000292 | - | - | - |
| cycles | - | - | - | 61.18k | 602.2 | - | - | - |
| instructions | - | - | - | 42.39k | 863.4 | - | - | - |
| cache references | - | - | - | 125.3 | 4.462 | - | - | - |
| cache misses | - | - | - | 11.63 | 0.00406 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.456 | 0.0000217 | - | - | - |
| data TLB read misses | - | - | - | 10.78 | 0.00472 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.71M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.71M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message

Run 2's profile:

| profile: the window, 30.0 s, 3000440 messages, 3000440313 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 345.6G | 115.2k | 115.2 | 9935 | 105.2 | - | - | - |
| waiting for a CPU, ns | - | - | - | 3.051 | 0.268 | - | - | - |
| allocations | 6.030M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.030M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.449G | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 24.66M | 8.218 | 0.00822 | 0.0109 | 0.00821 | - | - | - |
| involuntary context switches | 10.95k | 0.00365 | 0.00000365 | 0.000305 | 0.00000334 | - | - | - |
| cycles | - | - | - | 60.82k | 619.1 | - | - | - |
| instructions | - | - | - | 42.16k | 859.1 | - | - | - |
| cache references | - | - | - | 128.5 | 4.598 | - | - | - |
| cache misses | - | - | - | 11.64 | 0.00445 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.456 | 0.0000303 | - | - | - |
| data TLB read misses | - | - | - | 10.84 | 0.00424 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 43.98M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 43.98M | - | - | - | - | - | - | - |
| heap held at the end, bytes (the broker's runtime's) | - | - | - | - | - | - | - | - |

- L1 data cache read misses, the publisher's thread: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- L1 data cache read misses, the subscribers' threads: refused: cpu_atom's PMU has no such event (No such file or directory (os error 2))
- the node's accounted memory at its peak: 20.77M bytes, holding 65536 messages: 317.0 bytes a message

Run 3's profile:

| profile: the window, 30.0 s, 3002638 messages, 3002636487 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 350.4G | 116.7k | 116.7 | 9949 | 106.7 | - | - | - |
| waiting for a CPU, ns | - | - | - | 0.866 | 0.107 | - | - | - |
| allocations | 6.034M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 6.034M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| bytes allocated | 1.450G | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 17.00 | 0.00000566 | 0.000000006 | 0.00000566 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 25.40M | 8.459 | 0.00846 | 0.00757 | 0.00845 | - | - | - |
| involuntary context switches | 6319 | 0.00210 | 0.00000210 | 0.000132 | 0.00000197 | - | - | - |
| cycles | - | - | - | 60.75k | 629.1 | - | - | - |
| instructions | - | - | - | 42.59k | 864.7 | - | - | - |
| cache references | - | - | - | 122.7 | 4.638 | - | - | - |
| cache misses | - | - | - | 11.65 | 0.00267 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.463 | 0.0000185 | - | - | - |
| data TLB read misses | - | - | - | 8.727 | 0.00410 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 44.22M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 44.22M | - | - | - | - | - | - | - |
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
| process_minor_faults | message | 0.0001000 | 0 | yes |
| subscribers_llc_read_misses | delivery | 0.000168 | 0.0000217 | yes |
| subscribers_cpu_ns | delivery | 106.2 | 104.5 | yes |

Run 2's budgets:

| budget | per | budget | this run | met |
|---|---|---|---|---|
| subscribers_allocations | delivery | 0 | 0 | yes |
| subscribers_reallocations | delivery | 0 | 0 | yes |
| publisher_allocations | message | 2.010 | 2.010 | yes |
| publisher_reallocations | message | 0 | 0 | yes |
| process_minor_faults | message | 0.0001000 | 0 | yes |
| subscribers_llc_read_misses | delivery | 0.000168 | 0.0000303 | yes |
| subscribers_cpu_ns | delivery | 106.2 | 105.2 | yes |

Run 3's budgets:

| budget | per | budget | this run | met |
|---|---|---|---|---|
| subscribers_allocations | delivery | 0 | 0 | yes |
| subscribers_reallocations | delivery | 0 | 0 | yes |
| publisher_allocations | message | 2.010 | 2.010 | yes |
| publisher_reallocations | message | 0 | 0 | yes |
| process_minor_faults | message | 0.0001000 | 0.00000566 | yes |
| subscribers_llc_read_misses | delivery | 0.000168 | 0.0000185 | yes |
| subscribers_cpu_ns | delivery | 106.2 | 106.7 | yes |
