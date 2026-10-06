#!/usr/bin/env bash
# E7.1 one-pass verify (repo root; SCRATCH outside the repo; CARD_BASE = the tranche/17 SHA the card
# started from). Not-run skeleton first; each stage log ends with exit=<code>. One cargo process at a
# time. Logs: artifacts/epic_7/E7.1_logs/.
export PATH="$HOME/.cargo/bin:$PATH"
A=docs/release/SD-37-starfinder-1e/artifacts/epic_7
L=$A/E7.1_logs
SCRATCH=${SCRATCH:?}
SD=docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py
eval "$(scripts/fetch-pcgen-oracle.sh --check --quiet)"
STAGES="verify_stages ingest_global_default ingest_sf_tests ingest_always_held m1_m4 sf_check pf_check sf_corpus_check verify_stage residue sf_structural_diff var_names pf_dump pf_structural_diff root_sf fixture_check desktop_sf parity_crosscheck pf_seed_render frontend_test frontend_typecheck frontend_build clippy verify_root verify_desktop plants"
mkdir -p $L
for s in $STAGES; do echo "not-run" > $L/$s.log; done
run() { local s=$1; shift; { "$@"; echo "exit=$?"; } > $L/$s.log 2>&1; }
desk() { ( cd apps/desktop/src-tauri && "$@" ); }
fe() { ( cd apps/desktop && "$@" ); }
run verify_stages bash scripts/verify.sh --only preflight-oracle --only token-coverage --only denominator-gate --only figure-provenance --only tauri-resources-tracked --only crate-wall --only pcgen-residue-gate --only rules-schema-check --only sheet-rules-check --only sf-sheet-rules-check
run ingest_global_default cargo test --locked -j 8 -p codex-ingest --test sf_global_default -- --test-threads=8
run ingest_sf_tests cargo test --locked -j 8 -p codex-ingest --test sf_license_registry --test sf_core_proof --test sf_mapping_table --test sf_formula_system --test sf_drone_companion -- --test-threads=8
run ingest_always_held cargo test --locked -j 8 -p codex-ingest --lib always_held -- --test-threads=8
run m1_m4 python3 docs/release/SD-37-starfinder-1e/artifacts/epic_3/token-mapping/sf_mapping_mutations.py
run sf_check cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --system starfinder-1e --check
run pf_check cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --check
run sf_corpus_check cargo run --locked -j 8 -q -p codex-ingest --bin sf_corpus -- --check
run verify_stage bash scripts/verify.sh --only sf-sheet-rules-check
run residue python3 scripts/pcgen_residue_gate.py --check --closure
rm -rf "$SCRATCH/e71base" "$SCRATCH/pfdump"; mkdir -p "$SCRATCH/e71base"
git archive "${CARD_BASE:?}" data/starfinder-1e | tar -x -C "$SCRATCH/e71base"
{ python3 $SD data/starfinder-1e/sheet_rules --baseline "$SCRATCH/e71base/data/starfinder-1e/sheet_rules"; echo "exit=$?"; python3 $A/E7.1_delta.py data/starfinder-1e/sheet_rules "$SCRATCH/e71base/data/starfinder-1e/sheet_rules"; echo "delta_exit=$?"; } > $L/sf_structural_diff.log 2>&1
run var_names python3 -c "import json,sys; a=json.load(open('data/starfinder-1e/var_names.json')); b=json.load(open('$SCRATCH/e71base/data/starfinder-1e/var_names.json')); bad=[k for k,v in b.items() if a.get(k)!=v]; new=sorted(set(a)-set(b)); print('kept', len(b)-len(bad), 'of', len(b), 'new', len(new), [a[k] for k in new]); sys.exit(1 if bad else 0)"
run pf_dump cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --dump "$SCRATCH/pfdump"
{ python3 $SD "$SCRATCH/pfdump" --baseline data/sheet_rules; echo "exit=$?"; echo "diff -rq:"; diff -rq "$SCRATCH/pfdump" data/sheet_rules; echo "diff_exit=$?"; } > $L/pf_structural_diff.log 2>&1
run root_sf cargo test --locked -j 8 --lib sf_ -- --test-threads=8
run fixture_check python3 docs/release/SD-37-starfinder-1e/artifacts/epic_5/E5.1_seed_features.py . --check
rm -rf "$SCRATCH/seedlines" "$SCRATCH/paritydump"; mkdir -p "$SCRATCH/seedlines"
{ desk env SF_SEED_LINES_OUT="$SCRATCH/seedlines" SF_PARITY_ENGINE_OUT="$SCRATCH/paritydump" cargo test --locked -j 8 -- --test-threads=8 --nocapture sf_oracle_parity sf_sheet_print sf_adapter sf_drone_print sf_seed_every_hand_value race_books_load_through starfinder_1e_resolves; echo "exit=$?"; } > $L/desktop_sf.log 2>&1
mkdir -p $L/seed_lines $L/parity_dump && cp "$SCRATCH"/seedlines/*.txt $L/seed_lines/ 2>/dev/null; cp "$SCRATCH"/paritydump/*.json $L/parity_dump/ 2>/dev/null
run parity_crosscheck python3 scripts/oracle_harness/sf_parity_check.py "$SCRATCH/paritydump"
rm -rf "$SCRATCH/pfrender"; mkdir -p "$SCRATCH/pfrender"
{ desk env PF_SEED_RENDER_OUT="$SCRATCH/pfrender" cargo test --locked -j 8 --bins pf_seed_render_hash -- --nocapture --test-threads=8; echo "exit=$?"; cat "$SCRATCH/pfrender/sha256.txt"; } > $L/pf_seed_render.log 2>&1
{ fe npm test; echo "exit=$?"; } > $L/frontend_test.log 2>&1
{ fe npm run typecheck; echo "exit=$?"; } > $L/frontend_typecheck.log 2>&1
{ fe npm run build; echo "exit=$?"; } > $L/frontend_build.log 2>&1
run clippy bash scripts/verify.sh --only clippy
run verify_root cargo test --locked -j 8 --workspace --no-fail-fast -- --test-threads=8
{ desk cargo test --locked -j 8 --no-fail-fast -- --test-threads=8; echo "exit=$?"; } > $L/verify_desktop.log 2>&1
{ E71_BASE_SHEET_RULES="$SCRATCH/e71base/data/starfinder-1e/sheet_rules" python3 $A/E7.1_plants.py; echo "exit=$?"; } > $L/plants.log 2>&1
echo RUN_DONE
