#!/usr/bin/env python3
"""The comparison's cells judged twice: best of 3 over every run, and over the runs no one else's work
overlapped (Ada, 2026-10-24: accept overlaps, mark the cells they touched, rerun those whose verdict
depends on them). And the interleaving's check: each round of a load runs every system back to back.

  compare-clean.py RUNS.TSV PROGRESS.TSV BUSY.LOG OVERLAPS.TSV

PROGRESS.TSV: time and runs done (the window's log), to place each run in time.
BUSY.LOG: per-minute snapshots ("== HH:MM:SS", then processes over 60% CPU not the comparison's, then
  the 1-minute load and the window's state); a snapshot with neither load nor state didn't answer, and
  the minute around it is unknown, not clean.
OVERLAPS.TSV: extra intervals, "HH:MM:SS<TAB>HH:MM:SS<TAB>what", from before the busy log began.
"""
import csv
import math
import sys
from collections import defaultdict
from datetime import datetime, timedelta

runs_path, progress_path, busy_path, overlaps_path = sys.argv[1:5]
DAY = "2026-10-24"
t = lambda hms: datetime.fromisoformat(f"{DAY}T{hms}")
num = lambda s: math.inf if s in ("undelivered", "") else int(s)

rows = list(csv.DictReader(open(runs_path), delimiter="\t"))
prog = sorted((datetime.fromisoformat(r[0]), int(r[1])) for r in csv.reader(open(progress_path), delimiter="\t") if r and r[0] != "time")


def when_done(i):
    """When run i (0-based) ended: interpolated between the progress readings around it."""
    n = i + 1
    for (t0, n0), (t1, n1) in zip(prog, prog[1:]):
        if n0 == n:
            return t0
        if n0 < n <= n1:
            return t0 + (t1 - t0) * ((n - n0) / (n1 - n0))
    return prog[-1][0]


spans = [(when_done(i - 1) if i > 0 else prog[0][0], when_done(i)) for i in range(len(rows))]

# Each snapshot marks the 80 s around it busy (a foreign process), unknown (no answer) or clean.
marks = []
snap = None


def close(s):
    if s is None:
        return
    at, procs, answered = s
    if procs:
        marks.append((at - timedelta(seconds=40), at + timedelta(seconds=40), "busy", procs[0][:70]))
    elif not answered:
        marks.append((at - timedelta(seconds=40), at + timedelta(seconds=40), "unknown", "the snapshot didn't answer"))


busy_from = None
for line in open(busy_path):
    line = line.rstrip("\n")
    s = line.strip()
    if line.startswith("== "):
        close(snap)
        snap = (t(line[3:]), [], False)
        busy_from = busy_from or snap[0]
    elif not s or snap is None:
        continue
    elif s in ("active", "inactive", "failed", "activating") or s.replace(".", "", 1).isdigit():
        snap = (snap[0], snap[1], True)
    else:
        snap[1].append(s)
close(snap)
for r in csv.reader(open(overlaps_path), delimiter="\t"):
    if r and not r[0].startswith("#"):
        marks.append((t(r[0]), t(r[1]), "busy", r[2]))
# Before the busy log began, only OVERLAPS.TSV speaks: a run there not in it counts clean.


def touched(span):
    s, e = span
    return [(k, w) for (a, b, k, w) in marks if a < e and b > s]


cells = defaultdict(lambda: defaultdict(list))
for i, r in enumerate(rows):
    key = (r["busy"], int(r["rate"]), int(r["subscribers"]))
    cells[key][r["system"]].append({"p99": num(r["p99_ns"]), "p999": num(r["p999_ns"]), "i": i, "marks": touched(spans[i]),
                                    "undelivered": int(r["undelivered"]), "measured": int(r["measured"])})

us = lambda ns: "undelivered" if ns == math.inf else f"{ns / 1000:.1f}"
ratio = lambda a, b: "∞" if a == math.inf and b != math.inf else ("-" if b == math.inf else f"{a / b:.1f}×")
competitors = ("nats", "redis", "kafka")


def best(runs):
    """compare.sh's best: of the runs that delivered everything, the least p99; none did: undelivered."""
    delivered = [x for x in runs if x["undelivered"] == 0 and x["measured"] > 0]
    if delivered:
        return min(delivered, key=lambda x: x["p99"])
    return {"p99": math.inf, "p999": math.inf, "i": None, "marks": []} if runs else None


def verdict(by, pick):
    q = best(pick(by.get("quiver", [])))
    comp = [(s, best(pick(by.get(s, [])))) for s in competitors]
    comp = [(s, b) for s, b in comp if b]
    if not q or not comp:
        return None
    cs, cb = min(comp, key=lambda sb: (sb[1]["p99"], sb[1]["p999"]))
    lead = (cb["p99"] / q["p99"] if q["p99"] else math.inf, cb["p999"] / q["p999"] if q["p999"] else math.inf)
    return q, cs, cb, lead, lead[0] >= 10 and lead[1] >= 10


state = lambda x: "clean" if not x["marks"] else ("unknown" if all(k == "unknown" for k, _ in x["marks"]) else "overlapped")
print("| threads | rate | subs | floor p99/p99.9 µs | Quiver best p99/p99.9 µs | best competitor p99/p99.9 µs | lead p99 / p99.9 | ≥10× | over clean runs only | depends on a run not known clean |")
print("|---|---|---|---|---|---|---|---|---|---|")
for key in sorted(cells, key=lambda k: (k[0] != "busy", k[1], k[2])):
    by = cells[key]
    v = verdict(by, lambda xs: xs)
    if not v:
        continue
    q, cs, cb, lead, ok = v
    floor = best(by.get("wake", []))
    vc = verdict(by, lambda xs: [x for x in xs if not x["marks"]])
    if vc:
        qc, csc, cbc, leadc, okc = vc
        clean_txt = f"{ratio(cbc['p99'], qc['p99'])} / {ratio(cbc['p999'], qc['p999'])} vs {csc}"
        depends = "no" if okc == ok else "**yes**"
    else:
        clean_txt, depends = "no clean runs", "**yes**"
    m = lambda x: state(x) if x.get("i") is not None else "-"
    ftxt = f"{us(floor['p99'])} / {us(floor['p999'])}" if floor else "-"
    print(f"| {key[0]} | {key[1]} | {key[2]} | {ftxt} | {us(q['p99'])} / {us(q['p999'])} ({m(q)}) | {cs} {us(cb['p99'])} / {us(cb['p999'])} ({m(cb)}) | "
          f"{ratio(cb['p99'], q['p99'])} / {ratio(cb['p999'], q['p999'])} | {'yes' if ok else 'no'} | {clean_txt} | {depends} |")

print()
n_clean = sum(1 for i in range(len(rows)) if not touched(spans[i]))
n_unknown = sum(1 for i in range(len(rows)) if touched(spans[i]) and all(k == "unknown" for k, _ in touched(spans[i])))
print(f"{n_clean} of {len(rows)} runs overlapped no one else's work; {n_unknown} fell in minutes the busy log couldn't see; the rest overlapped work named below.")

# The interleaving: every 5 consecutive rows one load's round, each system once, back to back.
systems = sorted({r["system"] for r in rows})
n = len(systems)
bad = []
for c in range(0, len(rows), n):
    chunk = rows[c:c + n]
    keys = {(r["busy"], r["rate"], r["subscribers"]) for r in chunk}
    if len(keys) != 1 or sorted(r["system"] for r in chunk) != systems:
        bad.append(c)
widest = max(spans[c + n - 1][1] - spans[c][0] for c in range(0, len(rows), n))
print(f"interleaving: {len(rows) // n} rounds of a load, {'each' if not bad else 'NOT each'} with every system once, back to back; the widest round took {widest}.")
if bad:
    print("  rounds out of shape at rows", bad)

print("\nWho overlapped which runs:")
seen = defaultdict(list)
for i in range(len(rows)):
    for k, w in touched(spans[i]):
        seen[(k, w)].append(i + 1)
for (k, w), idx in sorted(seen.items(), key=lambda kv: kv[1][0]):
    print(f"- {k}: {w}: runs {idx[0]}-{idx[-1]} ({len(idx)})")
