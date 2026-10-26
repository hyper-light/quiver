#!/usr/bin/env bash
# AC-T2.2.5's sweep (kiln's CP-T2.2 step 6, docs/design/persisted-sim.md): seeds 1 to 10,000 of the
# persisted class's simulation in a release build, twice, in two partitions of the seeds over
# processes (pass A: eight of 1,250; pass B: nine, shifted by 625), so each seed runs after other
# seeds in another process and its digest must be alike in both. Each process at nice 10, its whole
# output in OUT/pass-<P>-<first>.log ending in an EXIT line; the passes one after the other (a seed's
# run is a folder under the test's directory, the same in both); then bench/sim-collect.py OUT.
#   bench/sim-sweep.sh OUT [A|B|AB]
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."
OUT=$1
PASSES=${2:-AB}
mkdir -p "$OUT"
cargo test --release --locked -p quiver --features sim --test sim --no-run > "$OUT/build.log" 2>&1
BIN=$(grep -o 'Executable tests/sim.rs ([^)]*)' "$OUT/build.log" | tail -1 | sed 's/.*(\(.*\))/\1/')
# A copy: a later build can't replace what a pass runs.
cp "$BIN" "$OUT/sim"
git rev-parse HEAD > "$OUT/commit"
for P in $(printf '%s' "$PASSES" | fold -w1); do
  if [ "$P" = A ]; then
    starts=(1 1251 2501 3751 5001 6251 7501 8751)
    ends=(1251 2501 3751 5001 6251 7501 8751 10001)
  else
    starts=(1 626 1876 3126 4376 5626 6876 8126 9376)
    ends=(626 1876 3126 4376 5626 6876 8126 9376 10001)
  fi
  for i in "${!starts[@]}"; do
    a=${starts[$i]}
    b=${ends[$i]}
    (
      status=0
      QUIVER_SIM_SEEDS="$a..$b" nice -n 10 "$OUT/sim" --nocapture --test-threads 1 \
        faults_over_many_seeds > "$OUT/pass-$P-$a.log" 2>&1 || status=$?
      echo "EXIT $status" >> "$OUT/pass-$P-$a.log"
    ) &
  done
  wait
done
python3 bench/sim-collect.py "$OUT"
