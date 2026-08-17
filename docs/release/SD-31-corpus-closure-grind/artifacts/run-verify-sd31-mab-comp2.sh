#!/usr/bin/env bash
set -u
LOG=docs/release/SD-31-corpus-closure-grind/artifacts/SD31-E6-F9-004-verify.log
export RETRO_ACTOR=sd31-mab-comp2
export CARGO_TARGET_DIR=/home/ubuntu/cargo-targets/sd31-mab-comp2
./scripts/verify.sh > "$LOG" 2>&1
echo "VERIFY_EXIT=$?" >> "$LOG"
