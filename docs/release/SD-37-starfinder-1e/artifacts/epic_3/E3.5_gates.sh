#!/usr/bin/env bash
# E3.5 acceptance gates (run from the repo root). Each block writes its own log and its exit code.
export PATH="$HOME/.cargo/bin:$PATH"
L=docs/release/SD-37-starfinder-1e/artifacts/epic_3/E3.5_logs
for f in green.check_sf.log green.sf_corpus_check.log residue_gate.log green.sf_core_proof.log; do echo "not-run" > $L/$f; done
{ cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --system starfinder-1e --check; echo "exit=$?"; } > $L/green.check_sf.log 2>&1
{ cargo run --locked -j 8 -q -p codex-ingest --bin sf_corpus -- --check; echo "exit=$?"; } > $L/green.sf_corpus_check.log 2>&1
{ python3 scripts/pcgen_residue_gate.py --check --closure; echo "exit=$?"; } > $L/residue_gate.log 2>&1
{ cargo test --locked -j 8 -p codex-ingest --test sf_core_proof -- --test-threads=8; echo "exit=$?"; } > $L/green.sf_core_proof.log 2>&1
echo GATES_DONE
