#!/usr/bin/env bash
# E4.5 one-pass verify (repo root; SCRATCH outside the repo; CARD_BASE = the tranche/17 SHA the
# card started from). Not-run skeleton first; each stage log ends with exit=<code>.
export PATH="$HOME/.cargo/bin:$PATH"
L=docs/release/SD-37-starfinder-1e/artifacts/epic_4/E4.5_logs
A=docs/release/SD-37-starfinder-1e/artifacts/epic_4
SCRATCH=${SCRATCH:?}
SD=docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py
eval "$(scripts/fetch-pcgen-oracle.sh --check --quiet)"
STAGES="green unit_money_bulk sf_check pf_check sf_corpus_check verify_stage residue sf_structural_diff pf_dump pf_structural_diff ingest_sf_tests e33_seed_fixture seed_terms e41_plants e42_plants e43_plants e44_plants e45_plants class_module_check pf_seed_render clippy verify_root verify_desktop"
for s in $STAGES; do echo "not-run" > $L/$s.log; done
run() { local s=$1; shift; { "$@"; echo "exit=$?"; } > $L/$s.log 2>&1; }
run green cargo test --locked -j 8 --lib sf_seed -- --test-threads=8
run unit_money_bulk cargo test --locked -j 8 --lib -- --test-threads=8 money:: encumbrance::
run sf_check cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --system starfinder-1e --check
run pf_check cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --check
run sf_corpus_check cargo run --locked -j 8 -q -p codex-ingest --bin sf_corpus -- --check
run verify_stage bash scripts/verify.sh --only sf-sheet-rules-check
run residue python3 scripts/pcgen_residue_gate.py --check --closure
rm -rf "$SCRATCH/e45base" "$SCRATCH/pfdump"; mkdir -p "$SCRATCH/e45base"
{ git archive "${CARD_BASE:?}" data/starfinder-1e/sheet_rules | tar -x -C "$SCRATCH/e45base" && python3 $SD data/starfinder-1e/sheet_rules --baseline "$SCRATCH/e45base/data/starfinder-1e/sheet_rules"; echo "exit=$?"; python3 $A/E4.5_delta.py data/starfinder-1e/sheet_rules "$SCRATCH/e45base/data/starfinder-1e/sheet_rules" "$PCGEN_REPO_DIR/data"; echo "delta_exit=$?"; } > $L/sf_structural_diff.log 2>&1
run pf_dump cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --dump "$SCRATCH/pfdump"
{ python3 $SD "$SCRATCH/pfdump" --baseline data/sheet_rules; echo "exit=$?"; echo "diff -rq:"; diff -rq "$SCRATCH/pfdump" data/sheet_rules; echo "diff_exit=$?"; } > $L/pf_structural_diff.log 2>&1
run ingest_sf_tests cargo test --locked -j 8 -p codex-ingest --test sf_license_registry --test sf_core_proof --test sf_mapping_table --test sf_formula_system -- --test-threads=8
run e33_seed_fixture python3 docs/release/SD-37-starfinder-1e/artifacts/epic_3/token-mapping/sf_mapping_mutations.py
run seed_terms python3 docs/release/SD-37-starfinder-1e/artifacts/epic_3/E3.4_seed_terms.py
run e41_plants python3 $A/E4.1_plants.py
run e42_plants python3 $A/E4.2_plants.py
run e43_plants python3 $A/E4.3_plants.py
run e44_plants python3 $A/E4.4_plants.py
run e45_plants python3 $A/E4.5_plants.py
{ ls src/rules_core/pilot_compute | awk 'tolower($0) ~ /soldier|mystic|envoy|technomancer|operative|mechanic|solarian|biohacker|vanguard|witchwarper/' | awk 'END{print NR}'; ls src/rules_core/pilot_compute | python3 -c "import sys,re; print(sum(1 for l in sys.stdin if re.search('soldier|mystic|envoy|technomancer|operative|mechanic|solarian|biohacker|vanguard|witchwarper', l.lower())))"; echo "exit=0"; } > $L/class_module_check.log 2>&1
( cd apps/desktop/src-tauri && cargo test --locked -j 8 --bins pf_seed_render_hash -- --nocapture --test-threads=8; echo "exit=$?" ) > $L/pf_seed_render.log 2>&1
run clippy bash scripts/verify.sh --only clippy
run verify_root cargo test --locked -j 8 --workspace --no-fail-fast -- --test-threads=8
( cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8; echo "exit=$?" ) > $L/verify_desktop.log 2>&1
echo RUN_DONE
