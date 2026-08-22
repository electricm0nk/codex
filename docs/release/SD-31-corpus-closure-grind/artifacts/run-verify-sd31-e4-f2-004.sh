#!/bin/bash
export RETRO_ACTOR=sd31-cf-pools
export CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd31-cf-pools
cd /home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_c9995ce5-db0-5
LOG=docs/release/SD-31-corpus-closure-grind/artifacts/SD31-E4-F2-004-verify.log
./scripts/verify.sh > "$LOG" 2>&1
echo "VERIFY_EXIT=$?" >> "$LOG"
