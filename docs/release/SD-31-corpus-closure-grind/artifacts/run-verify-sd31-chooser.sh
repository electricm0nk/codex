#!/usr/bin/env bash
set -u
export CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd31-chooser
export RETRO_ACTOR=sd31-chooser
cd /home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_800ec009-61e-1
LOG=docs/release/SD-31-corpus-closure-grind/artifacts/SD31-E4-F2-001-verify.log
./scripts/verify.sh > "$LOG" 2>&1
echo "VERIFY_EXIT=$?" >> "$LOG"
