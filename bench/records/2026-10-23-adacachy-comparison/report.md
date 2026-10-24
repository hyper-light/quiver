| subscribers' threads | rate /s | subscribers | the wake-up's floor, p99 / p99.9 µs | Quiver | NATS | Redis | Kafka | the best competitor | Quiver's lead, p99 / p99.9 | an order of magnitude |
|---|---|---|---|---|---|---|---|---|---|---|
| busy | 10000 | 1 | 0.8 / 34.8 | 1.0 / 5.4 | 37.4 / 462.8 | 67.6 / 1228.8 | 1622.0 / 6881.3 | nats | 37.4× / 85.1× | yes |
| busy | 10000 | 100 | 475.1 / 1802.2 | 1.9 / 8.4 | 2293.8 / 4980.7 | 1245.2 / 2064.4 | 6643777.5 / 7851737.1 | redis | 659.9× / 244.4× | yes |
| busy | 10000 | 1000 | 1884.2 / 4161.5 | 1572.9 / 3506.2 | undelivered / undelivered | undelivered / undelivered | undelivered / undelivered | nats | ∞ (the competitor's undelivered) / ∞ (the competitor's undelivered) | yes |
| busy | 100000 | 1 | 75.8 / 950.3 | 479.2 / 1589.2 | 8781.8 / 17825.8 | 1949.7 / undelivered | undelivered / undelivered | redis | 4.1× / ∞ (the competitor's undelivered) | no |
| busy | 100000 | 100 | 116.7 / 868.4 | 4.7 / 262.1 | undelivered / undelivered | undelivered / undelivered | undelivered / undelivered | nats | ∞ (the competitor's undelivered) / ∞ (the competitor's undelivered) | yes |
| busy | 100000 | 1000 | 1.0 / 999.4 | 66.6 / 655.4 | undelivered / undelivered | undelivered / undelivered | undelivered / undelivered | nats | ∞ (the competitor's undelivered) / ∞ (the competitor's undelivered) | yes |
| wait | 10000 | 1 | 170.0 / 1032.2 | 67.6 / 1065.0 | 487.4 / 1441.8 | 202.8 / 1040.4 | 4030.5 / 11272.2 | redis | 3.0× / 1.0× | no |
| wait | 10000 | 100 | 27.6 / 598.0 | 73.7 / 1007.6 | 778.2 / 1671.2 | 1097.7 / 1622.0 | 8120172.5 / 8992587.8 | nats | 10.6× / 1.7× | no |
| wait | 10000 | 1000 | 372.7 / 1671.2 | 1392.6 / 3080.2 | undelivered / undelivered | undelivered / undelivered | undelivered / undelivered | nats | ∞ (the competitor's undelivered) / ∞ (the competitor's undelivered) | yes |
| wait | 100000 | 1 | 753.7 / 1392.6 | 454.7 / 1261.6 | 1097.7 / 5308.4 | 1179.6 / 2293.8 | 27648852.0 / 27797983.9 | nats | 2.4× / 4.2× | no |
| wait | 100000 | 100 | 2392.1 / 4915.2 | 892.9 / 3113.0 | undelivered / undelivered | undelivered / undelivered | undelivered / undelivered | nats | ∞ (the competitor's undelivered) / ∞ (the competitor's undelivered) | yes |
| wait | 100000 | 1000 | 2490.4 / 6357.0 | 1818.6 / 5898.2 | undelivered / undelivered | undelivered / undelivered | undelivered / undelivered | nats | ∞ (the competitor's undelivered) / ∞ (the competitor's undelivered) | yes |

| subscribers' threads | rate /s | subscribers | offered deliveries /s | Quiver | NATS | Redis | Kafka | Quiver's lead |
|---|---|---|---|---|---|---|---|---|
| busy | 10000 | 1 | 0.01M | 0.01M | 0.01M | 0.01M | 0.01M | 1.0× |
| busy | 10000 | 100 | 1.00M | 1.00M | 1.00M | 1.00M | 1.00M | 1.0× |
| busy | 10000 | 1000 | 10.00M | 9.98M | 2.30M | 1.66M | 8.79M | 1.1× |
| busy | 100000 | 1 | 0.10M | 0.10M | 0.10M | 0.10M | 0.05M | 1.0× |
| busy | 100000 | 100 | 10.00M | 10.01M | 4.67M | 2.97M | 1.54M | 2.1× |
| busy | 100000 | 1000 | 100.00M | 100.01M | 0.00M | 0.00M | 1.29M | 77.4× |
| wait | 10000 | 1 | 0.01M | 0.01M | 0.01M | 0.01M | 0.01M | 1.0× |
| wait | 10000 | 100 | 1.00M | 1.00M | 1.00M | 1.00M | 1.00M | 1.0× |
| wait | 10000 | 1000 | 10.00M | 9.99M | 2.51M | 1.46M | 1.86M | 4.0× |
| wait | 100000 | 1 | 0.10M | 0.10M | 0.10M | 0.10M | 0.10M | 1.0× |
| wait | 100000 | 100 | 10.00M | 10.01M | 2.52M | 2.58M | 0.71M | 3.9× |
| wait | 100000 | 1000 | 100.00M | 100.09M | 0.00M | 0.00M | 1.46M | 68.7× |
