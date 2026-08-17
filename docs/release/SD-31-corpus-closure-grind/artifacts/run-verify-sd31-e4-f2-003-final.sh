#!/usr/bin/env bash
set -u
export CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd31-pool-consumers
export RETRO_ACTOR=sd31-pool-consumers
cd /home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_091c1ff2-4bf-1
LOG=docs/release/SD-31-corpus-closure-grind/artifacts/SD31-E4-F2-003-verify-final.log
./scripts/verify.sh > "$LOG" 2>&1
echo "VERIFY_EXIT=$?" >> "$LOG"
