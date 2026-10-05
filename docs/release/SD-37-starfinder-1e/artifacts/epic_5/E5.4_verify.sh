#!/usr/bin/env bash
# E5.4 one-pass verify (repo root; SCRATCH outside the repo; CARD_BASE = the tranche/17 SHA the
# card started from (attempt 2: 4c50ff7752; E5.4_delta.py is attempt 1's classifier, E5.4_r2_delta.py
# attempt 2's); RUN = log sub-directory, e.g. `committed` or `with-patch`). Not-run skeleton
# first; each stage log ends with exit=<code>.
export PATH="$HOME/.cargo/bin:$PATH"
A=docs/release/SD-37-starfinder-1e/artifacts/epic_5
L=$A/E5.4_logs/${RUN:?}
SCRATCH=${SCRATCH:?}
SD=docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py
eval "$(scripts/fetch-pcgen-oracle.sh --check --quiet)"
STAGES="ingest_drone ingest_mapping_unit e2_roundtrip m1_m4 sf_check pf_check sf_corpus_check verify_stage residue sf_structural_diff var_names pf_dump pf_structural_diff ingest_sf_tests root_sf_seed fixture_check desktop_sf pf_seed_render clippy verify_root verify_desktop"
mkdir -p $L
for s in $STAGES; do echo "not-run" > $L/$s.log; done
run() { local s=$1; shift; { "$@"; echo "exit=$?"; } > $L/$s.log 2>&1; }
run ingest_drone cargo test --locked -j 8 -p codex-ingest --test sf_drone_companion -- --test-threads=8
run ingest_mapping_unit cargo test --locked -j 8 -p codex-ingest --lib sf_mapping -- --test-threads=8
run e2_roundtrip cargo test --locked -j 8 --lib sheet_rule -- --test-threads=8
run m1_m4 python3 docs/release/SD-37-starfinder-1e/artifacts/epic_3/token-mapping/sf_mapping_mutations.py
run sf_check cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --system starfinder-1e --check
run pf_check cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --check
run sf_corpus_check cargo run --locked -j 8 -q -p codex-ingest --bin sf_corpus -- --check
run verify_stage bash scripts/verify.sh --only sf-sheet-rules-check
run residue python3 scripts/pcgen_residue_gate.py --check --closure
rm -rf "$SCRATCH/e54base" "$SCRATCH/pfdump"; mkdir -p "$SCRATCH/e54base"
git archive "${CARD_BASE:?}" data/starfinder-1e | tar -x -C "$SCRATCH/e54base"
{ python3 $SD data/starfinder-1e/sheet_rules --baseline "$SCRATCH/e54base/data/starfinder-1e/sheet_rules"; echo "exit=$?"; python3 $A/E5.4_delta.py data/starfinder-1e/sheet_rules "$SCRATCH/e54base/data/starfinder-1e/sheet_rules"; echo "delta_exit=$?"; python3 $A/E5.4_r2_delta.py data/starfinder-1e/sheet_rules "$SCRATCH/e54base/data/starfinder-1e/sheet_rules"; echo "r2_delta_exit=$?"; } > $L/sf_structural_diff.log 2>&1
run var_names python3 -c "import json,sys; a=json.load(open('data/starfinder-1e/var_names.json')); b=json.load(open('$SCRATCH/e54base/data/starfinder-1e/var_names.json')); bad=[k for k,v in b.items() if a.get(k)!=v]; new=sorted(set(a)-set(b)); print('kept', len(b)-len(bad), 'of', len(b), 'new', len(new), [a[k] for k in new]); sys.exit(1 if bad else 0)"
run pf_dump cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --dump "$SCRATCH/pfdump"
{ python3 $SD "$SCRATCH/pfdump" --baseline data/sheet_rules; echo "exit=$?"; echo "diff -rq:"; diff -rq "$SCRATCH/pfdump" data/sheet_rules; echo "diff_exit=$?"; } > $L/pf_structural_diff.log 2>&1
run ingest_sf_tests cargo test --locked -j 8 -p codex-ingest --test sf_license_registry --test sf_core_proof --test sf_mapping_table --test sf_formula_system -- --test-threads=8
run root_sf_seed cargo test --locked -j 8 --lib sf_ -- --test-threads=8
run fixture_check python3 $A/E5.1_seed_features.py . --check
rm -rf "$SCRATCH/seedlines"; mkdir -p "$SCRATCH/seedlines"
( cd apps/desktop/src-tauri && SF_SEED_LINES_OUT="$SCRATCH/seedlines" cargo test --locked -j 8 -- --test-threads=8 sf_sheet_print sf_adapter sf_drone_print; echo "exit=$?" ) > $L/desktop_sf.log 2>&1
mkdir -p $L/seed_lines && cp "$SCRATCH"/seedlines/*.txt $L/seed_lines/ 2>/dev/null
( cd apps/desktop/src-tauri && PF_SEED_RENDER_OUT="$SCRATCH/pfrender" cargo test --locked -j 8 --bins pf_seed_render_hash -- --nocapture --test-threads=8; echo "exit=$?" ) > $L/pf_seed_render.log 2>&1
run clippy bash scripts/verify.sh --only clippy
run verify_root cargo test --locked -j 8 --workspace --no-fail-fast -- --test-threads=8
( cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8; echo "exit=$?" ) > $L/verify_desktop.log 2>&1
echo RUN_DONE
