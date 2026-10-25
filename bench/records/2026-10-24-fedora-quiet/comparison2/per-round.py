#!/usr/bin/env python3
"""Each polling cell judged round by round (each round's runs of a load ran back to back, so its
systems' runs come from the same minutes), and the p99.9 lead against the least competitor p99.9.

  per-round.py RUNS.TSV [SYSTEMS_PER_ROUND]
"""
import csv, math, sys
rows = list(csv.DictReader(open(sys.argv[1]), delimiter="\t"))
n = int(sys.argv[2]) if len(sys.argv) > 2 else len({r["system"] for r in rows})
num = lambda s: math.inf if s in ("undelivered", "") else int(s)
comp = ("nats", "redis", "kafka")
cells = {}
for c in range(0, len(rows), n):
    chunk = rows[c:c + n]
    key = (chunk[0]["busy"], int(chunk[0]["rate"]), int(chunk[0]["subscribers"]))
    cells.setdefault(key, []).append({r["system"]: r for r in chunk})
fmt = lambda x: "∞" if x == math.inf else f"{x:.1f}×"
print("| threads | rate | subs | round 1 lead p99 / p99.9 | round 2 | round 3 | every round ≥10× | p99.9 lead against the least competitor p99.9 (best runs) |")
print("|---|---|---|---|---|---|---|---|")
for key in sorted(cells, key=lambda k: (k[0] != "busy", k[1], k[2])):
    if key[0] != "busy":
        continue
    leads, ok = [], True
    for rnd in cells[key]:
        q = rnd["quiver"]
        qp, qpp = num(q["p99_ns"]), num(q["p999_ns"])
        best = min((rnd[s] for s in comp), key=lambda r: (num(r["p99_ns"]), num(r["p999_ns"])))
        cp, cpp = num(best["p99_ns"]), num(best["p999_ns"])
        lp, lpp = cp / qp, cpp / qpp
        ok &= lp >= 10 and lpp >= 10
        leads.append(f"{fmt(lp)} / {fmt(lpp)} ({best['system']})")
    # Best of 3 by p99 per system, and the least p99.9 any competitor run reached.
    allq = [r["quiver"] for r in cells[key]]
    qb = min(allq, key=lambda r: (num(r["p99_ns"]), num(r["p999_ns"])))
    least999 = min(num(r[s]["p999_ns"]) for r in cells[key] for s in comp)
    lead999 = least999 / num(qb["p999_ns"])
    print(f"| busy | {key[1]} | {key[2]} | " + " | ".join(leads) + f" | {'yes' if ok else 'NO'} | {fmt(lead999)} |")
