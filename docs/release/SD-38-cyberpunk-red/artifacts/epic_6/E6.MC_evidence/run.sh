#!/usr/bin/env bash
# E6.MC runner: re-run E6.1-E6.4 acceptance commands on origin/tranche-18, then three plants.
# One cargo process at a time. Writes a not-run skeleton first; every row gets exit=.
set -u
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=/home/ubuntu/cargo-target/sd38-e6
export RETRO_ACTOR=sd38-e6.mc RUN_DESKTOP_AGENT=sd38-e6.mc
T=/home/ubuntu/workspace/repos/codex/.worktrees/sd38-e6
E=$T/docs/release/SD-38-cyberpunk-red/artifacts/epic_6/E6.MC_evidence
L=$E/logs
R=$E/results.txt
P=$E/progress.log
mkdir -p "$L"
STEPS="b01 b02 b03 b04 b05 b06 b07 b08 b09 m1_rust m1_ts m1_restored m2 m2_restored m3 m3_restored"
: > "$R"; for s in $STEPS; do echo "$s not-run" >> "$R"; done
: > "$P"
mark() { # step exit summary
  python3 -I - "$R" "$1" "$2" "$3" <<'PY'
import sys
path, step, rc, summ = sys.argv[1:]
lines = open(path).read().splitlines()
out = [f"{step} exit={rc} {summ}" if l.split(' ')[0] == step else l for l in lines]
open(path, 'w').write('\n'.join(out) + '\n')
PY
  echo "$(date -u +%FT%TZ) $1 exit=$2" >> "$P"
}
last() { grep -E "$2" "$1" | tail -1 | tr -s ' ' | cut -c1-300; }
ct() { (cd "$T/apps/desktop/src-tauri" && cargo test --locked -j 8 --bins cpr_catalog -- --test-threads=8) > "$L/$1.log" 2>&1; }

# --- Part 1: re-run ---
ct b01; rc=$?; mark b01 $rc "$(last $L/b01.log 'test result')"
n=$(awk '/cpr_catalog::/{n++} END{print n+0}' "$T/apps/desktop/src-tauri/src/main.rs"); mark b02 0 "cpr_catalog:: lines=$n (need >=2)"
(cd "$T/apps/desktop" && npm run typecheck) > "$L/b03.log" 2>&1; rc=$?; mark b03 $rc "typecheck"
(cd "$T/apps/desktop" && npm test) > "$L/b04.log" 2>&1; rc=$?; mark b04 $rc "$(last $L/b04.log 'test files')"
(cd "$T/apps/desktop" && npm run ui-smoke:doc) > "$L/b05.log" 2>&1; rc=$?
d=$(cd "$T" && git status --porcelain docs/testing/ui-smoke-inventory.md | awk 'END{print NR}')
mark b05 $rc "$(last $L/b05.log 'Wrote') inventory_drift_lines=$d"
(cd "$T/apps/desktop" && npx tsx src/cyberpunkCatalog/cyberpunkCatalog.test.ts) > "$L/b06.log" 2>&1; rc=$?; mark b06 $rc "$(last $L/b06.log 'tabs for|passed')"
(cd "$T/apps/desktop" && npx tsx src/cyberpunkCatalog/deck/cyberpunkDeck.test.ts) > "$L/b07.log" 2>&1; rc=$?; mark b07 $rc "$(last $L/b07.log 'passed')"
(cd "$T/apps/desktop" && npx tsx src/cyberpunkCatalog/gm/cyberpunkGm.test.ts) > "$L/b08.log" 2>&1; rc=$?; mark b08 $rc "$(last $L/b08.log 'records across')"
(cd "$T/apps/desktop" && npm run ui-smoke -- --only browse-cyberpunk-catalog,browse-cyberpunk-gm-reference,landing-select-pathfinder) > "$L/b09.log" 2>&1; rc=$?
mark b09 $rc "$(last $L/b09.log 'green, ') | $(last $L/b09.log 'UNTOUCHED|real store')"

# --- Part 2: plants ---
# m1: drop the last record of program.json -> E6.1 count test red (and E6.2 catalog test)
python3 -I - "$T/data/cyberpunk-red/corpus/program.json" <<'PY'
import json, sys
p = sys.argv[1]; d = json.load(open(p)); d.pop(); open(p, 'w').write(json.dumps(d, indent=2, ensure_ascii=False) + '\n')
PY
(cd "$T" && git diff --stat -- data/cyberpunk-red/corpus/program.json) > "$L/m1.diff"
ct m1_rust; rc=$?; mark m1_rust $rc "$(last $L/m1_rust.log 'test result') | $(last $L/m1_rust.log 'records vs MANIFEST|left:|right:|panicked')"
(cd "$T/apps/desktop" && npx tsx src/cyberpunkCatalog/cyberpunkCatalog.test.ts) > "$L/m1_ts.log" 2>&1; rc=$?; mark m1_ts $rc "$(last $L/m1_ts.log 'Error|Assert|expected|program')"
(cd "$T" && git checkout -- data/cyberpunk-red/corpus/program.json)
ct m1_restored; rc=$?; mark m1_restored $rc "$(last $L/m1_restored.log 'test result')"

# m2: Standard deck base slots 7 -> 8 in the engine -> E6.1 deck tests red
sed -i 's/            Self::Standard => 7,/            Self::Standard => 8,/' "$T/src/rules_core/cpr/netrunning.rs"
(cd "$T" && git diff -- src/rules_core/cpr/netrunning.rs) > "$L/m2.diff"
ct m2; rc=$?; mark m2 $rc "$(last $L/m2.log 'test result') | $(awk '/ FAILED$/' $L/m2.log | tr '\n' ' ' | cut -c1-400)"
(cd "$T" && git checkout -- src/rules_core/cpr/netrunning.rs)
ct m2_restored; rc=$?; mark m2_restored $rc "$(last $L/m2_restored.log 'test result')"

# m3: drop one demon record -> E6.4 GM page test red (MANIFEST count shown on the page)
python3 -I - "$T/data/cyberpunk-red/corpus/demon.json" <<'PY'
import json, sys
p = sys.argv[1]; d = json.load(open(p)); d.pop(); open(p, 'w').write(json.dumps(d, indent=2, ensure_ascii=False) + '\n')
PY
(cd "$T" && git diff --stat -- data/cyberpunk-red/corpus/demon.json) > "$L/m3.diff"
(cd "$T/apps/desktop" && npx tsx src/cyberpunkCatalog/gm/cyberpunkGm.test.ts) > "$L/m3.log" 2>&1; rc=$?; mark m3 $rc "$(last $L/m3.log 'Error|Assert|expected|demon')"
(cd "$T" && git checkout -- data/cyberpunk-red/corpus/demon.json)
(cd "$T/apps/desktop" && npx tsx src/cyberpunkCatalog/gm/cyberpunkGm.test.ts) > "$L/m3_restored.log" 2>&1; rc=$?; mark m3_restored $rc "$(last $L/m3_restored.log 'records across')"

(cd "$T" && git status --porcelain) > "$L/final_status.txt"
echo "$(date -u +%FT%TZ) ALL_DONE" >> "$P"
