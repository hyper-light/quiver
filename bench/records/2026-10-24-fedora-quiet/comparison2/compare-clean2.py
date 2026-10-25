#!/usr/bin/env python3
"""The second grid's cells judged as the first's were (compare-clean.py): best of 3 over every run,
and over the runs no one else's work overlapped (Ada, 2026-10-24: accept overlaps, mark the cells
they touched, rerun those whose verdict depends on them); the interleaving's check; and Quiver's lead
split at the floor over TCP (the deployment's part, the floor over Quiver; the broker's, the
competitor over the floor).

  compare-clean2.py RUNS.TSV BUSY.LOG [OVERLAPS.TSV] [DAY]

BUSY.LOG is the window's own watcher (quiet-window4.sh): "== HH:MM:SS load a b c runs N", then a
line "cpu cgroup command" for each process over 60% of a CPU that was neither the window's nor the
comparison's containers', then "unit name" lines. The runs counts place each run in time, each at
its widest: a snapshot near it marks it, whatever moment within those bounds it really ran.
OVERLAPS.TSV adds intervals ("HH:MM:SS<TAB>HH:MM:SS<TAB>what") of work its owners reported, which the
snapshots may have missed between them or under their 60% filter.
"""
import csv
import math
import sys
from collections import defaultdict
from datetime import datetime, timedelta

runs_path, busy_path = sys.argv[1:3]
overlaps_path = sys.argv[3] if len(sys.argv) > 3 and sys.argv[3].endswith(".tsv") else None
DAY = sys.argv[-1] if len(sys.argv) > 3 and not sys.argv[-1].endswith(".tsv") else "2026-10-24"
t = lambda hms: datetime.fromisoformat(f"{DAY}T{hms}")
num = lambda s: math.inf if s in ("undelivered", "") else int(s)

rows = list(csv.DictReader(open(runs_path), delimiter="\t"))

snaps = []  # (time, runs done, [foreign processes])
for line in open(busy_path):
    line = line.rstrip("\n")
    if line.startswith("== "):
        parts = line.split()
        at = t(parts[1])
        done = int(parts[parts.index("runs") + 1]) if "runs" in parts else -1
        snaps.append([at, max(done, 0), []])
    elif snaps and line and line.split(maxsplit=1)[0].replace(".", "", 1).isdigit():
        # "cpu cgroup command": a process's line (the watcher's own complaints and its unit lines aren't).
        snaps[-1][2].append(line)

# Each run's widest span: from the last snapshot that hadn't counted the run before it (the earliest
# it could have begun) to the first that counted it (the latest it could have ended).
def span(i):
    begun = max((at for at, done, _ in snaps if done <= i - 1), default=snaps[0][0]) if i > 0 else max(at for at, done, _ in snaps if done == 0)
    ended = min((at for at, done, _ in snaps if done >= i + 1), default=snaps[-1][0])
    return begun, ended


spans = [span(i) for i in range(len(rows))]
marks = [(at - timedelta(seconds=35), at + timedelta(seconds=35), procs[0][:110]) for at, _, procs in snaps if procs]
if overlaps_path:
    for r in csv.reader(open(overlaps_path), delimiter="\t"):
        if r and not r[0].startswith("#"):
            marks.append((t(r[0]), t(r[1]), r[2]))


def touched(span):
    s, e = span
    return [w for (a, b, w) in marks if a < e and b > s]


cells = defaultdict(lambda: defaultdict(list))
for i, r in enumerate(rows):
    key = (r["busy"], int(r["rate"]), int(r["subscribers"]))
    cells[key][r["system"]].append({"p99": num(r["p99_ns"]), "p999": num(r["p999_ns"]), "i": i, "marks": touched(spans[i]),
                                    "undelivered": int(r["undelivered"]), "measured": int(r["measured"])})

us = lambda ns: "undelivered" if ns == math.inf else f"{ns / 1000:.1f}"
ratio = lambda a, b: "-" if b == math.inf else ("∞" if a == math.inf else f"{a / b:.1f}×")
competitors = ("nats", "redis", "kafka")


def best(runs):
    """compare.sh's best: the least p99, a quantile that came beating one that didn't."""
    return min(runs, key=lambda x: (x["p99"], x["p999"])) if runs else None


def verdict(by, pick):
    q = best(pick(by.get("quiver", [])))
    comp = [(s, best(pick(by.get(s, [])))) for s in competitors]
    comp = [(s, b) for s, b in comp if b]
    if not q or not comp:
        return None
    cs, cb = min(comp, key=lambda sb: (sb[1]["p99"], sb[1]["p999"]))
    lead = (cb["p99"] / q["p99"], cb["p999"] / q["p999"])
    return q, cs, cb, lead[0] >= 10 and lead[1] >= 10


state = lambda x: "clean" if not x["marks"] else "overlapped"
order = lambda k: (k[0] != "busy", k[1], k[2])
print("| subscribers' threads | rate | subscribers | the floor | the floor over TCP | Quiver | the best competitor | Quiver's lead | an order of magnitude | over clean runs alone | rests on runs not known clean |")
print("|---|---|---|---|---|---|---|---|---|---|---|")
for key in sorted(cells, key=order):
    by = cells[key]
    v = verdict(by, lambda xs: xs)
    if not v:
        continue
    q, cs, cb, ok = v
    vc = verdict(by, lambda xs: [x for x in xs if not x["marks"]])
    if vc:
        qc, csc, cbc, okc = vc
        clean_txt = f"{ratio(cbc['p99'], qc['p99'])} / {ratio(cbc['p999'], qc['p999'])} vs {csc}"
        depends = "no" if okc == ok else "**yes**"
    else:
        clean_txt, depends = "no clean runs", "**yes**"
    pair = lambda b: f"{us(b['p99'])} / {us(b['p999'])} ({state(b)})" if b else "-"
    print(f"| {key[0]} | {key[1]} | {key[2]} | {pair(best(by.get('wake', [])))} | {pair(best(by.get('tcp', [])))} | {pair(q)} | {cs} {pair(cb)} | "
          f"{ratio(cb['p99'], q['p99'])} / {ratio(cb['p999'], q['p999'])} | {'yes' if ok else 'no'} | {clean_txt} | {depends} |")

print("\nThe lead split at the floor over TCP (best of 3 over every run):\n")
print("| subscribers' threads | rate | subscribers | the deployment's part: the floor over TCP over Quiver | the broker's part: the best competitor over the floor over TCP |")
print("|---|---|---|---|---|")
for key in sorted(cells, key=order):
    by = cells[key]
    q, tcp = best(by.get("quiver", [])), best(by.get("tcp", []))
    v = verdict(by, lambda xs: xs)
    if not (q and tcp and v):
        continue
    _, cs, cb, _ = v
    print(f"| {key[0]} | {key[1]} | {key[2]} | {ratio(tcp['p99'], q['p99'])} / {ratio(tcp['p999'], q['p999'])} | "
          f"{cs} {ratio(cb['p99'], tcp['p99'])} / {ratio(cb['p999'], tcp['p999'])} |")

n_clean = sum(1 for i in range(len(rows)) if not touched(spans[i]))
print(f"\n{n_clean} of {len(rows)} runs overlapped no one else's work.")
systems = sorted({r["system"] for r in rows})
n = len(systems)
bad = [c for c in range(0, len(rows), n)
       if len({(r["busy"], r["rate"], r["subscribers"]) for r in rows[c:c + n]}) != 1 or sorted(r["system"] for r in rows[c:c + n]) != systems]
widest = max((spans[min(c + n, len(rows)) - 1][1] - spans[c][0] for c in range(0, len(rows), n)), default=timedelta(0))
print(f"interleaving: {len(rows) // n} rounds of a load over {n} systems, {'each' if not bad else 'NOT each'} with every system once, back to back; the widest round took {widest}.")
seen = defaultdict(list)
for i in range(len(rows)):
    for w in touched(spans[i]):
        seen[w].append(i + 1)
if seen:
    print("\nWho overlapped which runs:")
    for w, idx in sorted(seen.items(), key=lambda kv: kv[1][0]):
        print(f"- {w}: runs {idx[0]}-{idx[-1]} ({len(idx)})")
