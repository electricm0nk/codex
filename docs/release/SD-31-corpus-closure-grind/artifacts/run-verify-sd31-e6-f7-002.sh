#!/usr/bin/env bash
set -u
export RETRO_ACTOR=sd31-ingest6
export CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd31-ingest6
cd /home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_c9995ce5-db0-6
LOG=/home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_c9995ce5-db0-6/docs/release/SD-31-corpus-closure-grind/artifacts/SD31-E6-F7-002-verify.log
./scripts/verify.sh > "$LOG" 2>&1
echo "VERIFY_EXIT=$?" >> "$LOG"
