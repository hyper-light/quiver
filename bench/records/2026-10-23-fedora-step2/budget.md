# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 1790a7ce024120bdf1a08a11dae36b4ced4cd868
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which poll without waiting; window 60 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × busy | 6549345 | 99992 | 0 | 0 | 6549345000 | 5999684000 | 0 | 3.2 | 15.2 | 1146.9 | 45613.1 | 516211.7 | 0 | 0 | 1000/1000 | 1481.9 | 64.7 | - | 16.19 / 23.96 |
| best | quiver | 22 × busy | 6549345 | 99992 | 0 | 0 | 6549345000 | 5999684000 | 0 | 3.2 | 15.2 | 1146.9 | 45613.1 | 516211.7 | 0 | 0 | 1000/1000 | 1481.9 | 64.7 | - | 16.19 / 23.96 |

Run 1's profile:

| profile: the window, 60.0 s, 5999684 messages, 5999683990 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 1.363T | 227.3k | 227.3 | 9924 | 217.3 | - | - | - |
| waiting for a CPU, ns | - | - | - | 44.28 | 1.942 | - | - | - |
| allocations | 12.06M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 12.06M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| bytes allocated | 2.898G | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 35.00 | 0.00000583 | 0.000000006 | 0.00000100 | 0.000000005 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 11.00 | 0.00000183 | 0.000000002 | 0.000000333 | 0.000000002 | - | - | - |
| involuntary context switches | 20.54k | 0.00342 | 0.00000342 | 0.000141 | 0.00000328 | - | - | - |
| cycles | - | - | - | 67.39k | 1080 | - | - | - |
| instructions | - | - | - | 52.83k | 2413 | - | - | - |
| cache references | - | - | - | 50.05 | 3.595 | - | - | - |
| cache misses | - | - | - | 11.85 | 0.00882 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.275 | 0.0000413 | - | - | - |
| data TLB read misses | - | - | - | 0.110 | 0.00394 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.19M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.19M | - | - | - | - | - | - | - |
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
| process_minor_faults | message | 0.0001000 | 0.00000583 | yes |
# quiver-bench fanout quiver

- machine: Intel(R) Core(TM) i9-14900HX, 24 CPUs this process may use, Linux 6.19.10-300.fc44.x86_64, 65555324 kB
- commit: 1790a7ce024120bdf1a08a11dae36b4ced4cd868
- system: quiver (in this process)
- offered: 100000 messages/s, Poisson, 256 B payloads, one topic keeping 65536 messages
- subscribers: 1000 over 22 threads, which wait when there's nothing to take; window 60 s after 5 s of warm-up; seed 0x5eed

| run | system | subscribers' threads | published | /s achieved | never offered | publish errors | deliveries | measured | undelivered | p50 µs | p90 µs | p99 µs | p99.9 µs | max µs | lag notices | lost silently | kept up | CPU s | publisher's CPU s | broker's CPU s | load before / after |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | quiver | 22 × wait | 6549344 | 99990 | 1 | 0 | 6549344000 | 5999684000 | 0 | 13.6 | 55.8 | 161.8 | 344.1 | 5723.1 | 0 | 0 | 1000/1000 | 756.7 | 65.0 | - | 23.96 / 18.69 |
| best | quiver | 22 × wait | 6549344 | 99990 | 1 | 0 | 6549344000 | 5999684000 | 0 | 13.6 | 55.8 | 161.8 | 344.1 | 5723.1 | 0 | 0 | 1000/1000 | 756.7 | 65.0 | - | 23.96 / 18.69 |

Run 1's profile:

| profile: the window, 60.0 s, 5999686 messages, 5999685761 deliveries | this process | per message | per delivery | the publisher's thread, per message | the subscribers' threads, per delivery | the broker | per message | per delivery |
|---|---|---|---|---|---|---|---|---|
| CPU, ns | 696.5G | 116.1k | 116.1 | 9924 | 106.2 | - | - | - |
| waiting for a CPU, ns | - | - | - | 8.530 | 2.056 | - | - | - |
| allocations | 12.06M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| reallocations | 0 | 0 | 0 | 0 | 0 | - | - | - |
| reallocations that moved their block | 0 | 0 | 0 | 0 | 0 | - | - | - |
| frees | 12.06M | 2.010 | 0.00201 | 2.010 | 0 | - | - | - |
| bytes allocated | 2.898G | 483.0 | 0.483 | 483.0 | 0 | - | - | - |
| minor page faults | 35.00 | 0.00000583 | 0.000000006 | 0.00000583 | 0 | - | - | - |
| major page faults | 0 | 0 | 0 | 0 | 0 | - | - | - |
| voluntary context switches | 42.72M | 7.120 | 0.00712 | 0.00472 | 0.00712 | - | - | - |
| involuntary context switches | 130.8k | 0.0218 | 0.0000218 | 0.000680 | 0.0000211 | - | - | - |
| cycles | - | - | - | 53.76k | 567.6 | - | - | - |
| instructions | - | - | - | 37.26k | 835.0 | - | - | - |
| cache references | - | - | - | 119.0 | 3.803 | - | - | - |
| cache misses | - | - | - | 13.93 | 0.0102 | - | - | - |
| L1 data cache read misses | - | - | - | refused | refused | - | - | - |
| last-level cache read misses | - | - | - | 0.594 | 0.000168 | - | - | - |
| data TLB read misses | - | - | - | 9.727 | 0.0104 | - | - | - |
| bytes read from storage | 0 | 0 | 0 | - | - | - | - | - |
| bytes written to storage | 0 | 0 | 0 | - | - | - | - | - |
| storage reads | - | - | - | - | - | - | - | - |
| storage writes | - | - | - | - | - | - | - | - |
| garbage collections | - | - | - | - | - | - | - | - |
| collectors' pauses, ns | - | - | - | - | - | - | - | - |
| memory at its peak in the window, bytes (resident; the broker's cgroup's) | 41.87M | - | - | - | - | - | - | - |
| memory at the window's end, bytes (resident; the broker's cgroup's) | 41.87M | - | - | - | - | - | - | - |
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
| process_minor_faults | message | 0.0001000 | 0.00000583 | yes |
| subscribers_llc_read_misses | delivery | 0.000865 | 0.000168 | yes |
| subscribers_cpu_ns | delivery | 97.73 | 106.2 | yes |
