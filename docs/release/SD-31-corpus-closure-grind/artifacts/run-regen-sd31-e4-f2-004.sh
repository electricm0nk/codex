#!/bin/bash
set -e
cd /home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_c9995ce5-db0-5
export CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd31-cf-pools
export CORPUS_LITERAL_SWEEP_REPORT=/tmp/claude-1000/-home-ubuntu-workspace-repos-codex-docs-release-SD-31-corpus-closure-grind/442534fa-fe61-4970-bcb4-076e3980c180/scratchpad/sd31-cf-pools/sweep.json
export DERIVED_FIXTURE_CHECK_REPORT=/tmp/claude-1000/-home-ubuntu-workspace-repos-codex-docs-release-SD-31-corpus-closure-grind/442534fa-fe61-4970-bcb4-076e3980c180/scratchpad/sd31-cf-pools/fixture.json
cargo run --locked --bin v06_work_inventory -- --allow-stamp-loss
