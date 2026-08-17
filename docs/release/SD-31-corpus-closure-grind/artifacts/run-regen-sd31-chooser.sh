#!/usr/bin/env bash
set -u
export CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd31-chooser-regen
cd /home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_800ec009-61e-1
SCRATCH=/tmp/claude-1000/-home-ubuntu-workspace-repos-codex-docs-release-SD-31-corpus-closure-grind/442534fa-fe61-4970-bcb4-076e3980c180/scratchpad/sd31-chooser
echo "== corpus_literal_sweep =="
cargo run --locked --bin corpus_literal_sweep -- --json-out "$SCRATCH/sweep-sd31-chooser.json"
echo "SWEEP_EXIT=$?"
echo "== derived_evaluator_fixture_check =="
cargo run --locked --bin derived_evaluator_fixture_check -- --json-out "$SCRATCH/fixture-sd31-chooser.json"
echo "FIXTURE_EXIT=$?"
echo "== v06_work_inventory (guarded) =="
CORPUS_LITERAL_SWEEP_REPORT="$SCRATCH/sweep-sd31-chooser.json" \
DERIVED_FIXTURE_CHECK_REPORT="$SCRATCH/fixture-sd31-chooser.json" \
  cargo run --locked --bin v06_work_inventory
echo "REGEN_EXIT=$?"
echo "DONE"
