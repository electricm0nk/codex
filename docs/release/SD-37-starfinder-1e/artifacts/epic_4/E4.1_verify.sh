#!/usr/bin/env bash
# E4.1 one-pass verify (repo root; SCRATCH outside the repo). Not-run skeleton first; each stage
# log ends with exit=<code>.
export PATH="$HOME/.cargo/bin:$PATH"
L=docs/release/SD-37-starfinder-1e/artifacts/epic_4/E4.1_logs
SCRATCH=${SCRATCH:?}
SD=docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py
STAGES="sf_check pf_check sf_corpus_check verify_stage residue sf_structural_diff pf_dump pf_structural_diff ingest_sf_tests e33_seed_fixture seed_terms class_module_check pf_seed_render verify_root verify_desktop"
for s in $STAGES; do echo "not-run" > $L/$s.log; done
run() { local s=$1; shift; { "$@"; echo "exit=$?"; } > $L/$s.log 2>&1; }
run sf_check cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --system starfinder-1e --check
run pf_check cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --check
run sf_corpus_check cargo run --locked -j 8 -q -p codex-ingest --bin sf_corpus -- --check
run verify_stage bash scripts/verify.sh --only sf-sheet-rules-check
run residue python3 scripts/pcgen_residue_gate.py --check --closure
rm -rf "$SCRATCH/e41base" "$SCRATCH/pfdump"; mkdir -p "$SCRATCH/e41base"
{ git archive "${CARD_BASE:?}" data/starfinder-1e/sheet_rules | tar -x -C "$SCRATCH/e41base" && python3 $SD data/starfinder-1e/sheet_rules --baseline "$SCRATCH/e41base/data/starfinder-1e/sheet_rules"; echo "exit=$?"; python3 docs/release/SD-37-starfinder-1e/artifacts/epic_4/E4.1_race_hp_delta.py data/starfinder-1e/sheet_rules "$SCRATCH/e41base/data/starfinder-1e/sheet_rules"; echo "delta_exit=$?"; } > $L/sf_structural_diff.log 2>&1
run pf_dump cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --dump "$SCRATCH/pfdump"
{ python3 $SD "$SCRATCH/pfdump" --baseline data/sheet_rules; echo "exit=$?"; echo "diff -rq:"; diff -rq "$SCRATCH/pfdump" data/sheet_rules; echo "diff_exit=$?"; } > $L/pf_structural_diff.log 2>&1
run ingest_sf_tests cargo test --locked -j 8 -p codex-ingest --test sf_license_registry --test sf_core_proof --test sf_mapping_table --test sf_formula_system -- --test-threads=8
run e33_seed_fixture python3 docs/release/SD-37-starfinder-1e/artifacts/epic_3/token-mapping/sf_mapping_mutations.py
run seed_terms python3 docs/release/SD-37-starfinder-1e/artifacts/epic_3/E3.4_seed_terms.py
{ ls src/rules_core/pilot_compute | awk 'tolower($0) ~ /soldier|mystic|envoy|technomancer|operative|mechanic|solarian|biohacker|vanguard|witchwarper/' | awk 'END{print NR}'; ls src/rules_core/pilot_compute | python3 -c "import sys,re; print(sum(1 for l in sys.stdin if re.search('soldier|mystic|envoy|technomancer|operative|mechanic|solarian|biohacker|vanguard|witchwarper', l.lower())))"; echo "exit=0"; } > $L/class_module_check.log 2>&1
( cd apps/desktop/src-tauri && cargo test --locked -j 8 --bins pf_seed_render_hash -- --nocapture --test-threads=8; echo "exit=$?" ) > $L/pf_seed_render.log 2>&1
run verify_root cargo test --locked -j 8 --workspace --no-fail-fast -- --test-threads=8
( cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8; echo "exit=$?" ) > $L/verify_desktop.log 2>&1
echo RUN_DONE
