#!/usr/bin/env bash
# E5.3 one-pass verify (repo root; SCRATCH outside the repo; CARD_BASE = the tranche/17 SHA the
# card started from). Not-run skeleton first; each stage log ends with exit=<code>.
export PATH="$HOME/.cargo/bin:$PATH"
L=docs/release/SD-37-starfinder-1e/artifacts/epic_5/E5.3_logs
A=docs/release/SD-37-starfinder-1e/artifacts/epic_5
SCRATCH=${SCRATCH:?}
SD=docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py
eval "$(scripts/fetch-pcgen-oracle.sh --check --quiet)"
STAGES="green ingest_unit fixture_check sf_check pf_check sf_corpus_check verify_stage residue sf_structural_diff pf_dump pf_structural_diff ingest_sf_tests root_sf_seed seed_terms plants pf_seed_render clippy verify_root verify_desktop"
mkdir -p $L
for s in $STAGES; do echo "not-run" > $L/$s.log; done
run() { local s=$1; shift; { "$@"; echo "exit=$?"; } > $L/$s.log 2>&1; }
rm -rf "$SCRATCH/seedlines"; mkdir -p "$SCRATCH/seedlines"
( cd apps/desktop/src-tauri && SF_SEED_LINES_OUT="$SCRATCH/seedlines" cargo test --locked -j 8 -- --test-threads=8 sf_sheet_print sf_adapter; echo "exit=$?" ) > $L/green.log 2>&1
mkdir -p $L/seed_lines && cp "$SCRATCH"/seedlines/*.txt $L/seed_lines/ 2>/dev/null
run ingest_unit cargo test --locked -j 8 -p codex-ingest --lib sf_modifier_stat -- --test-threads=8
run fixture_check python3 $A/E5.1_seed_features.py . --check
run sf_check cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --system starfinder-1e --check
run pf_check cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --check
run sf_corpus_check cargo run --locked -j 8 -q -p codex-ingest --bin sf_corpus -- --check
run verify_stage bash scripts/verify.sh --only sf-sheet-rules-check
run residue python3 scripts/pcgen_residue_gate.py --check --closure
rm -rf "$SCRATCH/e53base" "$SCRATCH/pfdump"; mkdir -p "$SCRATCH/e53base"
git archive "${CARD_BASE:?}" data/starfinder-1e/sheet_rules | tar -x -C "$SCRATCH/e53base"
{ python3 $SD data/starfinder-1e/sheet_rules --baseline "$SCRATCH/e53base/data/starfinder-1e/sheet_rules"; echo "exit=$?"; python3 $A/E5.3_delta.py data/starfinder-1e/sheet_rules "$SCRATCH/e53base/data/starfinder-1e/sheet_rules" "$PCGEN_CORPUS_ROOT"; echo "delta_exit=$?"; } > $L/sf_structural_diff.log 2>&1
run pf_dump cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --dump "$SCRATCH/pfdump"
{ python3 $SD "$SCRATCH/pfdump" --baseline data/sheet_rules; echo "exit=$?"; echo "diff -rq:"; diff -rq "$SCRATCH/pfdump" data/sheet_rules; echo "diff_exit=$?"; } > $L/pf_structural_diff.log 2>&1
run ingest_sf_tests cargo test --locked -j 8 -p codex-ingest --test sf_license_registry --test sf_core_proof --test sf_mapping_table --test sf_formula_system -- --test-threads=8
run root_sf_seed cargo test --locked -j 8 --lib sf_seed -- --test-threads=8
run seed_terms python3 docs/release/SD-37-starfinder-1e/artifacts/epic_3/E3.4_seed_terms.py
run plants python3 $A/E5.3_plants.py "$SCRATCH/e53base/data/starfinder-1e/sheet_rules" "$PCGEN_CORPUS_ROOT"
( cd apps/desktop/src-tauri && PF_SEED_RENDER_OUT="$SCRATCH/pfrender" cargo test --locked -j 8 --bins pf_seed_render_hash -- --nocapture --test-threads=8; echo "exit=$?" ) > $L/pf_seed_render.log 2>&1
run clippy bash scripts/verify.sh --only clippy
run verify_root cargo test --locked -j 8 --workspace --no-fail-fast -- --test-threads=8
( cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8; echo "exit=$?" ) > $L/verify_desktop.log 2>&1
echo RUN_DONE
