#!/usr/bin/env bash
# E3.5 one-pass widest-scope verify (repo root; SCRATCH = a scratch dir outside the repo).
# Writes a not-run skeleton first; each stage overwrites its own log and ends with exit=<code>.
export PATH="$HOME/.cargo/bin:$PATH"
L=docs/release/SD-37-starfinder-1e/artifacts/epic_3/E3.5_logs
SCRATCH=${SCRATCH:?}
STAGES="sf_gates pf_dump pf_structural_diff formula_census e33_mutations clippy verify_root verify_desktop"
for s in $STAGES; do echo "not-run" > $L/v.$s.log; done
bash docs/release/SD-37-starfinder-1e/artifacts/epic_3/E3.5_gates.sh > $L/v.sf_gates.log 2>&1; echo "exit=$?" >> $L/v.sf_gates.log
rm -rf "$SCRATCH/pfdump"
{ cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --dump "$SCRATCH/pfdump"; echo "exit=$?"; } > $L/v.pf_dump.log 2>&1
{ python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py "$SCRATCH/pfdump" --baseline data/sheet_rules; echo "exit=$?"; echo "diff -rq:"; diff -rq "$SCRATCH/pfdump" data/sheet_rules; } > $L/v.pf_structural_diff.log 2>&1
{ cargo run --locked -j 8 -q -p codex-ingest --bin sf_formula_census -- --check; echo "exit=$?"; } > $L/v.formula_census.log 2>&1
{ python3 docs/release/SD-37-starfinder-1e/artifacts/epic_3/token-mapping/sf_mapping_mutations.py; echo "exit=$?"; } > $L/v.e33_mutations.log 2>&1
( cd crates/codex-ingest && cargo clippy --locked --tests -j 8 -- -D warnings; echo "exit=$?" ) > $L/v.clippy.log 2>&1
{ cargo test --locked -j 8 --workspace --no-fail-fast -- --test-threads=8; echo "exit=$?"; } > $L/v.verify_root.log 2>&1
( cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8; echo "exit=$?" ) > $L/v.verify_desktop.log 2>&1
echo VERIFY_DONE
