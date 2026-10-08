#!/usr/bin/env bash
# E3.MC adversarial merge check: one pass over origin/tranche/17 (repo root; SCRATCH outside repo).
# Not-run skeleton first; every stage writes its own log ending with exit=<code>.
export PATH="$HOME/.cargo/bin:$PATH"
A=docs/release/SD-37-starfinder-1e/artifacts/epic_3
L=$A/E3.MC_logs
SCRATCH=${SCRATCH:?}
SD=docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py
STAGES="sf_check sf_corpus_check verify_stage residue sf_tests sf_dump sf_structural_diff pf_dump pf_structural_diff formula_census e33_mutations converter_mutations e34_plants pi_audit seed_terms core_delta_rederive pf_seed_render verify_root verify_desktop"
for s in $STAGES; do echo "not-run" > $L/$s.log; done
run() { local s=$1; shift; { "$@"; echo "exit=$?"; } > $L/$s.log 2>&1; }
run sf_check cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --system starfinder-1e --check
run sf_corpus_check cargo run --locked -j 8 -q -p codex-ingest --bin sf_corpus -- --check
run verify_stage bash scripts/verify.sh --only sf-sheet-rules-check
run residue python3 scripts/pcgen_residue_gate.py --check --closure
run sf_tests cargo test --locked -j 8 -p codex-ingest --test sf_license_registry --test sf_core_proof --test sf_mapping_table --test sf_formula_system -- --test-threads=8
rm -rf "$SCRATCH/sfdump" "$SCRATCH/pfdump"
run sf_dump cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --system starfinder-1e --dump "$SCRATCH/sfdump"
{ python3 $SD "$SCRATCH/sfdump" --baseline data/starfinder-1e/sheet_rules; echo "exit=$?"; echo "diff -rq:"; diff -rq "$SCRATCH/sfdump" data/starfinder-1e/sheet_rules; echo "diff_exit=$?"; } > $L/sf_structural_diff.log 2>&1
run pf_dump cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --dump "$SCRATCH/pfdump"
{ python3 $SD "$SCRATCH/pfdump" --baseline data/sheet_rules; echo "exit=$?"; echo "diff -rq:"; diff -rq "$SCRATCH/pfdump" data/sheet_rules; echo "diff_exit=$?"; cmp "$SCRATCH/pfdump/var_names.json" scripts/oracle_harness/var_names.json && echo var_names_cmp_equal; } > $L/pf_structural_diff.log 2>&1
run formula_census cargo run --locked -j 8 -q -p codex-ingest --bin sf_formula_census -- --check
run e33_mutations python3 $A/token-mapping/sf_mapping_mutations.py
run converter_mutations python3 $A/E3.MC_converter_mutations.py
run e34_plants bash $A/E3.4_plants.sh
run pi_audit python3 $A/E3.MC_pi_audit.py
run seed_terms python3 $A/E3.4_seed_terms.py
rm -rf "$SCRATCH/e34pkg"; mkdir -p "$SCRATCH/e34pkg"
{ git archive a12dec77d3 data/starfinder-1e/sheet_rules | tar -x -C "$SCRATCH/e34pkg" && python3 $A/E3.5_core_delta_classes.py "$SCRATCH/e34pkg/data/starfinder-1e/sheet_rules" data/starfinder-1e/sheet_rules; echo "exit=$?"; } > $L/core_delta_rederive.log 2>&1
( cd apps/desktop/src-tauri && cargo test --locked -j 8 --bins pf_seed_render_hash -- --nocapture --test-threads=8; echo "exit=$?" ) > $L/pf_seed_render.log 2>&1
run verify_root cargo test --locked -j 8 --workspace --no-fail-fast -- --test-threads=8
( cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8; echo "exit=$?" ) > $L/verify_desktop.log 2>&1
echo RUN_DONE
