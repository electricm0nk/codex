#!/usr/bin/env bash
# Full-gate runner for SD31-W15-MONSTER-ABILITY-001.
# Own CARGO_TARGET_DIR (never /tmp), exit code captured in the same shell
# statement that ran the gate, never through a pipe (SD-30 loop-instruction
# cycle mechanics 4/4a).
set -u
cd /home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_0628906e-65b-2
export CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd31-w15-monster-ability
export PCGEN_CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data
export RETRO_ACTOR=sd31-w15-monster-ability
LOG=docs/release/SD-31-corpus-closure-grind/artifacts/SD31-W15-MONSTER-ABILITY-001-verify.log
./scripts/verify.sh > "$LOG" 2>&1
echo "VERIFY_EXIT=$?" >> "$LOG"
