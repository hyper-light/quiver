| subscribers' threads | rate | subscribers | the floor | the floor over TCP | Quiver | the best competitor | Quiver's lead | an order of magnitude | over clean runs alone | rests on runs not known clean |
|---|---|---|---|---|---|---|---|---|---|---|
| busy | 10000 | 1 | 0.4 / 2.4 (clean) | 17.7 / 19.7 (overlapped) | 0.9 / 2.8 (clean) | redis 62.5 / 196.6 (overlapped) | 67.4× / 70.6× | yes | 69.0× / 71.4× vs redis | no |
| busy | 10000 | 100 | 0.5 / 2.7 (clean) | 180.2 / 3473.4 (clean) | 2.5 / 13.1 (overlapped) | nats 811.0 / 1376.3 (clean) | 320.9× / 105.4× | yes | 298.3× / 100.5× vs nats | no |
| busy | 10000 | 1000 | 0.5 / 3.1 (overlapped) | 27000.8 / 36700.2 (clean) | 13.4 / 157.7 (clean) | nats undelivered / undelivered (clean) | ∞ / ∞ | yes | ∞ / ∞ vs nats | no |
| busy | 100000 | 1 | 0.2 / 2.3 (overlapped) | 23.0 / 27.9 (overlapped) | 0.8 / 6.8 (overlapped) | redis 33.3 / 170.0 (clean) | 39.3× / 24.8× | yes | 22.9× / 23.1× vs redis | no |
| busy | 100000 | 100 | 0.6 / 2.6 (overlapped) | 1212.4 / 21495.8 (overlapped) | 6.0 / 15.6 (overlapped) | nats undelivered / undelivered (clean) | ∞ / ∞ | yes | ∞ / ∞ vs nats | no |
| busy | 100000 | 1000 | 0.6 / 3.5 (overlapped) | 55050.2 / 106954.8 (overlapped) | 139.3 / 221.2 (overlapped) | nats undelivered / undelivered (overlapped) | ∞ / ∞ | yes | no clean runs | **yes** |
| wait | 10000 | 1 | 21.2 / 86.0 (overlapped) | 54.8 / 101.4 (clean) | 21.5 / 73.7 (clean) | redis 107.5 / 229.4 (clean) | 5.0× / 3.1× | no | 5.0× / 3.1× vs redis | no |
| wait | 10000 | 100 | 32.5 / 45.1 (clean) | 170.0 / 409.6 (clean) | 39.4 / 78.8 (clean) | nats 622.6 / 1179.6 (overlapped) | 15.8× / 15.0× | yes | 18.7× / 16.0× vs nats | no |
| wait | 10000 | 1000 | 32.8 / 52.2 (clean) | 2293.8 / 15990.8 (clean) | 48.6 / 137.2 (clean) | nats undelivered / undelivered (clean) | ∞ / ∞ | yes | ∞ / ∞ vs nats | no |
| wait | 100000 | 1 | 5.3 / 56.3 (clean) | 26.6 / 65.5 (overlapped) | 6.7 / 55.8 (clean) | redis 41.5 / 184.3 (clean) | 6.2× / 3.3× | no | 6.2× / 3.3× vs redis | no |
| wait | 100000 | 100 | 562036.7 / 587202.6 (clean) | 274.4 / 450.6 (clean) | 77.8 / 131.1 (clean) | nats undelivered / undelivered (clean) | ∞ / ∞ | yes | ∞ / ∞ vs nats | no |
| wait | 100000 | 1000 | 406847.5 / 415236.1 (clean) | 12976.1 / 15859.7 (clean) | 122.9 / 192.5 (clean) | nats undelivered / undelivered (overlapped) | ∞ / ∞ | yes | ∞ / ∞ vs nats | no |

The lead split at the floor over TCP (best of 3 over every run):

| subscribers' threads | rate | subscribers | the deployment's part: the floor over TCP over Quiver | the broker's part: the best competitor over the floor over TCP |
|---|---|---|---|---|
| busy | 10000 | 1 | 19.1× / 7.1× | redis 3.5× / 10.0× |
| busy | 10000 | 100 | 71.3× / 266.1× | nats 4.5× / 0.4× |
| busy | 10000 | 1000 | 2009.1× / 232.7× | nats ∞ / ∞ |
| busy | 100000 | 1 | 27.2× / 4.1× | redis 1.4× / 6.1× |
| busy | 100000 | 100 | 201.6× / 1376.6× | nats ∞ / ∞ |
| busy | 100000 | 1000 | 395.3× / 483.6× | nats ∞ / ∞ |
| wait | 10000 | 1 | 2.5× / 1.4× | redis 2.0× / 2.3× |
| wait | 10000 | 100 | 4.3× / 5.2× | nats 3.7× / 2.9× |
| wait | 10000 | 1000 | 47.2× / 116.5× | nats ∞ / ∞ |
| wait | 100000 | 1 | 4.0× / 1.2× | redis 1.6× / 2.8× |
| wait | 100000 | 100 | 3.5× / 3.4× | nats ∞ / ∞ |
| wait | 100000 | 1000 | 105.6× / 82.4× | nats ∞ / ∞ |

106 of 216 runs overlapped no one else's work.
interleaving: 36 rounds of a load over 6 systems, each with every system once, back to back; the widest round took 0:06:02.

Who overlapped which runs:
- 164.8 /user.slice/user-1000.slice/user@1000.service/app.slice/w24-baseline.service /home/adalundhe/.cache/w24/: runs 24-27 (4)
- 99.5 /user.slice/user-1000.slice/user@1000.service/app.slice/w24-baseline.service git -c user.name=Test -c use: runs 32-35 (4)
- 99.7 /user.slice/user-1000.slice/user@1000.service/app.slice/w24-baseline.service git -c user.name=Test -c use: runs 33-36 (4)
- 97.1 /user.slice/user-1000.slice/user@1000.service/app.slice/w24-baseline.service /usr/libexec/git-core/git un: runs 67-71 (5)
- 172.7 /user.slice/user-1000.slice/user@1000.service/app.slice/w24-baseline.service /home/adalundhe/.cache/w24/: runs 89-91 (3)
- W24, Build Caching's capped cargo builds (nice 19, -j 4, 17 builds), to the grid's end: runs 124-216 (93)
- 98.7 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p593002-i42512409.scope /home/adalundhe/.rust: runs 125-128 (4)
- 98.9 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p593002-i42512409.scope /home/adalundhe/.rust: runs 127-130 (4)
- 84.2 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p602843-i42522154.scope /home/adalundhe/.rust: runs 128-131 (4)
- 99.7 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p602843-i42522154.scope /home/adalundhe/.rust: runs 130-133 (4)
- 97.8 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p614207-i42533484.scope /home/adalundhe/.rust: runs 139-142 (4)
- 97.9 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p616641-i42535918.scope /home/adalundhe/.rust: runs 141-143 (3)
- 76.0 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p616641-i42535918.scope /home/adalundhe/.rust: runs 142-144 (3)
- 99.1 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p624625-i42543902.scope /home/adalundhe/.rust: runs 143-146 (4)
- 322.7 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p624625-i42543902.scope /home/adalundhe/.rus: runs 144-148 (5)
- 99.8 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p636701-i42555977.scope git -c user.name=Test: runs 154-157 (4)
- 128.3 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p639066-i42558342.scope /home/adalundhe/.rus: runs 156-159 (4)
- 79.5 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p641980-i42561256.scope /home/adalundhe/.rust: runs 157-160 (4)
- 63.0 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p653537-i42572808.scope /home/adalundhe/.rust: runs 161-164 (4)
- 97.5 /user.slice/user-1000.slice/user@1000.service/app.slice/w24-m0-gate.service /home/adalundhe/.rustup/toolc: runs 166-169 (4)
- 99.8 /user.slice/user-1000.slice/user@1000.service/app.slice/w24-m0-gate.service git -c user.name=Test -c user: runs 167-170 (4)
- 99.5 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p673324-i42592584.scope /home/adalundhe/.rust: runs 173-176 (4)
- 99.4 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p687902-i42607084.scope /home/adalundhe/.cach: runs 178-182 (5)
- 98.7 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p696319-i42615496.scope /home/adalundhe/.rust: runs 187-190 (4)
- 99.4 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p699377-i42618490.scope /home/adalundhe/.rust: runs 189-192 (4)
- 99.5 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p699377-i42618490.scope /home/adalundhe/.rust: runs 190-194 (5)
- 99.5 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p702818-i42621921.scope /home/adalundhe/.rust: runs 194-196 (3)
- 96.5 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p702818-i42621921.scope /home/adalundhe/.rust: runs 195-198 (4)
- 227.5 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p702818-i42621921.scope /home/adalundhe/.rus: runs 196-199 (4)
- 98.1 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p713440-i42632542.scope /home/adalundhe/.rust: runs 198-201 (4)
- 99.5 /user.slice/user-1000.slice/user@1000.service/app.slice/run-p715770-i42634872.scope /home/adalundhe/.rust: runs 199-203 (5)
