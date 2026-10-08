#!/usr/bin/env bash
# SD-37 E4.MC one-pass adversarial re-run (repo root of the tranche/17 tree; SCRATCH outside the repo).
# Not-run skeleton first; each stage log ends with exit=<code>. One cargo process at a time.
export PATH="$HOME/.cargo/bin:$PATH"
L=docs/release/SD-37-starfinder-1e/artifacts/epic_4/E4.MC_logs
A=docs/release/SD-37-starfinder-1e/artifacts/epic_4
SCRATCH=${SCRATCH:?}
SD=docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py
eval "$(scripts/fetch-pcgen-oracle.sh --check --quiet)"
mkdir -p "$L"
STAGES="root_sf_seed class_module_check pf_seed_render plant_con_stamina plant_pf_light_load e41_plants sf_check pf_check sf_corpus_check verify_stage residue sd36_sf_structural_diff pf_dump pf_structural_diff clippy verify_root verify_desktop"
for s in $STAGES; do echo "not-run" > "$L/$s.log"; done
run() { local s=$1; shift; { "$@"; echo "exit=$?"; } > "$L/$s.log" 2>&1; }
desk() { ( cd apps/desktop/src-tauri && "$@" ); }

# 1. The 4 SF seeds through the engine readers (E4.1-E4.5 acceptance).
run root_sf_seed cargo test --locked -j 8 --lib sf_seed -- --test-threads=8

# 2. E4.1's class-module check, two implementations.
{ ls src/rules_core/pilot_compute | awk 'tolower($0) ~ /soldier|mystic|envoy|technomancer|operative|mechanic|solarian|biohacker|vanguard|witchwarper/' | awk 'END{print NR}'
  ls src/rules_core/pilot_compute | python3 -c "import sys,re; print(sum(1 for l in sys.stdin if re.search('soldier|mystic|envoy|technomancer|operative|mechanic|solarian|biohacker|vanguard|witchwarper', l.lower())))"
  echo "exit=0"; } > "$L/class_module_check.log" 2>&1

# 3. PF hash pair through the E1.4 harness.
rm -rf "$SCRATCH/pfrender"; mkdir -p "$SCRATCH/pfrender"
{ desk env PF_SEED_RENDER_OUT="$SCRATCH/pfrender" cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture; echo "exit=$?"
  cat "$SCRATCH/pfrender/sha256.txt"; } > "$L/pf_seed_render.log" 2>&1
cp "$SCRATCH/pfrender/sha256.txt" "$L/pf_seed_sha256.txt"

# 4. Criterion plant: Con dropped from Stamina (sf_chassis.rs), root sf_seed AND the desktop
#    adapter's 160-row hand-value test must both go red; restored byte for byte.
ENG=src/rules_core/pilot_compute/sf_chassis.rs
cp "$ENG" "$SCRATCH/sf_chassis.rs.orig"
{ python3 - "$ENG" <<'PY'
import sys
p = sys.argv[1]; s = open(p).read()
old = 'label: "Constitution modifier × level".into(), value: con * level,'
assert s.count(old) == 1, "plant site not found"
open(p, "w").write(s.replace(old, 'label: "Constitution modifier × level".into(), value: 0,'))
print("PLANTED: Con x level dropped from Stamina")
PY
  cargo test --locked -j 8 --lib sf_seed -- --test-threads=8; echo "root_exit=$?"
  desk cargo test --locked -j 8 -- --test-threads=8 sf_seed_every_hand_value starfinder_1e_resolves; echo "desktop_exit=$?"
  cp "$SCRATCH/sf_chassis.rs.orig" "$ENG"; cmp "$SCRATCH/sf_chassis.rs.orig" "$ENG" && echo RESTORED
  echo "exit=0"; } > "$L/plant_con_stamina.log" 2>&1

# 5. Sensitivity plant on the PF side of encumbrance.rs (E4.5 edited the file): the PF light load
#    +1 lb; does the E1.4 harness see it? Restored byte for byte.
ENC=src/rules_core/encumbrance.rs
cp "$ENC" "$SCRATCH/encumbrance.rs.orig"
rm -rf "$SCRATCH/pfplant"; mkdir -p "$SCRATCH/pfplant"
{ python3 - "$ENC" <<'PY'
import sys
p = sys.argv[1]; s = open(p).read()
old = "light_max_lbs: (scaled / (denominator * 3)) as f64,"
assert s.count(old) == 1, "plant site not found"
open(p, "w").write(s.replace(old, "light_max_lbs: (scaled / (denominator * 3)) as f64 + 1.0,"))
print("PLANTED: PF light load +1 lb")
PY
  desk env PF_SEED_RENDER_OUT="$SCRATCH/pfplant" cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture; echo "render_exit=$?"
  cat "$SCRATCH/pfplant/sha256.txt"
  cp "$SCRATCH/encumbrance.rs.orig" "$ENC"; cmp "$SCRATCH/encumbrance.rs.orig" "$ENC" && echo RESTORED
  echo "exit=0"; } > "$L/plant_pf_light_load.log" 2>&1

# 6. M1-M4 (decisions.md §8) re-planted by E4.1's own script.
run e41_plants python3 "$A/E4.1_plants.py"

# 7. Converter / package gates.
run sf_check cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --system starfinder-1e --check
run pf_check cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --check
run sf_corpus_check cargo run --locked -j 8 -q -p codex-ingest --bin sf_corpus -- --check
run verify_stage bash scripts/verify.sh --only sf-sheet-rules-check
run residue python3 scripts/pcgen_residue_gate.py --check --closure

# 8. SD-36 structural diff over the whole epic (SF package at the E4 start 46f0acb5b4 vs now).
rm -rf "$SCRATCH/e4start"; mkdir -p "$SCRATCH/e4start"
{ git archive 46f0acb5b4 data/starfinder-1e/sheet_rules | tar -x -C "$SCRATCH/e4start" && python3 $SD data/starfinder-1e/sheet_rules --baseline "$SCRATCH/e4start/data/starfinder-1e/sheet_rules"; echo "exit=$?"; } > "$L/sd36_sf_structural_diff.log" 2>&1
rm -rf "$SCRATCH/pfdump"
run pf_dump cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --dump "$SCRATCH/pfdump"
{ python3 $SD "$SCRATCH/pfdump" --baseline data/sheet_rules; echo "exit=$?"; echo "diff -rq:"; diff -rq "$SCRATCH/pfdump" data/sheet_rules; echo "diff_exit=$?"; } > "$L/pf_structural_diff.log" 2>&1

# 9. Clippy stage (three crates), then the widest verify, once.
mkdir -p "$SCRATCH/clippy_after"
run clippy env VERIFY_LOG_DIR="$SCRATCH/clippy_after" bash scripts/verify.sh -j 8 --only clippy
mkdir -p "$L/clippy_after"; cp "$SCRATCH"/clippy_after/clippy-*.log "$L/clippy_after/" 2>/dev/null
run verify_root cargo test --locked -j 8 --workspace --no-fail-fast -- --test-threads=8
{ desk cargo test --locked -j 8 --no-fail-fast -- --test-threads=8; echo "exit=$?"; } > "$L/verify_desktop.log" 2>&1
echo RUN_DONE
