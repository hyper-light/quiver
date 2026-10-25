| threads | rate | subs | floor p99/p99.9 µs | Quiver best p99/p99.9 µs | best competitor p99/p99.9 µs | lead p99 / p99.9 | ≥10× | over clean runs only | depends on a run not known clean |
|---|---|---|---|---|---|---|---|---|---|
| busy | 10000 | 1 | 0.2 / 2.1 | 1.1 / 3.9 (clean) | redis 63.5 / 258.0 (unknown) | 59.3× / 66.1× | yes | no clean runs | **yes** |
| busy | 10000 | 100 | 0.6 / 3.4 | 3.2 / 15.6 (overlapped) | nats 909.3 / 2162.7 (clean) | 281.4× / 138.5× | yes | no clean runs | **yes** |
| busy | 10000 | 1000 | 0.6 / 52.7 | 14.1 / 167.9 (overlapped) | nats undelivered / undelivered (-) | ∞ / ∞ | yes | no clean runs | **yes** |
| busy | 100000 | 1 | 0.3 / 2.2 | 0.8 / 6.7 (clean) | redis 26.4 / 157.7 (clean) | 31.4× / 23.5× | yes | 31.4× / 23.5× vs redis | no |
| busy | 100000 | 100 | 0.6 / 3.9 | 7.0 / 20.0 (overlapped) | nats undelivered / undelivered (-) | ∞ / ∞ | yes | ∞ / ∞ vs nats | no |
| busy | 100000 | 1000 | 0.7 / 1359.9 | 176.1 / 260.1 (unknown) | nats undelivered / undelivered (-) | ∞ / ∞ | yes | no clean runs | **yes** |
| wait | 10000 | 1 | 20.5 / 69.6 | 21.2 / 69.6 (clean) | nats 101.4 / 149.5 (clean) | 4.8× / 2.1× | no | 4.8× / 2.1× vs nats | no |
| wait | 10000 | 100 | 33.3 / 69.6 | 42.0 / 99.3 (clean) | nats 663.6 / 1065.0 (clean) | 15.8× / 10.7× | yes | 15.8× / 10.7× vs nats | no |
| wait | 10000 | 1000 | 33.3 / 67.6 | 49.2 / 141.3 (clean) | nats undelivered / undelivered (-) | ∞ / ∞ | yes | ∞ / ∞ vs nats | no |
| wait | 100000 | 1 | 5.6 / 60.9 | 6.9 / 53.8 (clean) | redis 36.4 / 176.1 (clean) | 5.3× / 3.3× | no | 5.3× / 3.3× vs redis | no |
| wait | 100000 | 100 | undelivered / undelivered | 75.8 / 123.9 (clean) | nats undelivered / undelivered (-) | ∞ / ∞ | yes | ∞ / ∞ vs nats | no |
| wait | 100000 | 1000 | undelivered / undelivered | 128.0 / 208.9 (overlapped) | nats undelivered / undelivered (-) | ∞ / ∞ | yes | no clean runs | **yes** |

40 of 180 runs overlapped no one else's work; 8 fell in minutes the busy log couldn't see; the rest overlapped work named below.
interleaving: 36 rounds of a load, each with every system once, back to back; the widest round took 0:03:58.500000.

Who overlapped which runs:
- busy: kiln CI ui checks (run 1138, fedora runner): runs 26-42 (17)
- busy: Mantle (kiln run r70): a release build of mantle at 8 jobs, then mantle disk probes, three at once at the end: runs 54-64 (11)
- busy: a kiln build in ~/.cache/kiln-gs (stable rustc): runs 55-94 (40)
- busy: rustfmt from a reach copy: runs 70-71 (2)
- busy: Focal builds and cold-stream bench pinned to CPU 4: runs 71-140 (70)
- busy: Mantle gate (gate-fw) and Amphora engine build: runs 74-94 (21)
- unknown: the snapshot didn't answer: runs 92-180 (53)
- busy: 309 /home/adalundhe/.rustup/toolchains/stable-x86_64-unknown-linux-gnu: runs 94-96 (3)
- busy: 631 /home/adalundhe/.rustup/toolchains/stable-x86_64-unknown-linux-gnu: runs 97-99 (3)
- busy: 580 /home/adalundhe/.rustup/toolchains/stable-x86_64-unknown-linux-gnu: runs 99-101 (3)
- busy: 261 /home/adalundhe/.cache/claude-scratch/amphora-t1-fedora-target/deb: runs 101-103 (3)
- busy: 1330 /home/adalundhe/.cache/claude-scratch/amphora-t1-fedora-target/de: runs 111-113 (3)
- busy: 380 /home/adalundhe/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu: runs 119-121 (3)
- busy: 201 [rust-lld] <defunct>: runs 121-124 (4)
- busy: 397 /home/adalundhe/.cache/claude-scratch/amphora-t1-fedora-target/deb: runs 125-127 (3)
- busy: 423 /home/adalundhe/.kiln/reach/adas-macbook-pro-2.local/gate-fw-8ee20: runs 127-129 (3)
- busy: 216 /home/adalundhe/.kiln/reach/adas-macbook-pro-2.local/gate-fw-8ee20: runs 129-131 (3)
- busy: 662 /home/adalundhe/.rustup/toolchains/stable-x86_64-unknown-linux-gnu: runs 133-136 (4)
- busy: 111 /usr/local/rustup/toolchains/1.98.0-x86_64-unknown-linux-gnu/bin/r: runs 141-143 (3)
- busy: 99.2 /home/adalundhe/.rustup/toolchains/stable-x86_64-unknown-linux-gn: runs 143-145 (3)
- busy: 98.1 /home/adalundhe/.kiln/reach/adas-macbook-pro-2.local/gate-fw-8ee2: runs 146-148 (3)
- busy: 315 /home/adalundhe/.rustup/toolchains/stable-x86_64-unknown-linux-gnu: runs 148-149 (2)
- busy: 696 /home/adalundhe/.rustup/toolchains/stable-x86_64-unknown-linux-gnu: runs 149-151 (3)
- busy: 101 /home/adalundhe/.rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu: runs 159-161 (3)
- busy: 393 /home/adalundhe/.cache/claude-scratch/amphora-t1-fedora-target/deb: runs 162-163 (2)
- busy: 105 /home/adalundhe/.cache/claude-scratch/amphora-t1-fedora-target/deb: runs 163-166 (4)
- busy: 461 /home/adalundhe/.cache/claude-scratch/amphora-t1-fedora-target/deb: runs 168-170 (3)
- busy: 101 /home/adalundhe/.cache/claude-scratch/amphora-t1-fedora-target/deb: runs 170-172 (3)
- busy: 133 /home/adalundhe/.cache/claude-scratch/amphora-t1-fedora-target/deb: runs 175-177 (3)
- busy: 150 /home/adalundhe/.cache/claude-scratch/amphora-t1-fedora-target/deb: runs 177-178 (2)
