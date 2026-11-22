#!/usr/bin/env bash
set -x
export PATH="$HOME/.cargo/bin:$PATH"
S=/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/33627023-04e6-4757-9afa-d5c1463408f3/scratchpad/e62
T=/home/ubuntu/workspace/worktrees/codex-sd37-e6.2-red
git -C /home/ubuntu/workspace/worktrees/codex-sd37 worktree add --detach $T 9b7888f9286d2e1988f3c1da1c03493cf2f63b76
cp /home/ubuntu/workspace/worktrees/codex-sd37/apps/desktop/scripts/ui-smoke/spec.json $T/apps/desktop/scripts/ui-smoke/spec.json
export CARGO_TARGET_DIR=/home/ubuntu/workspace/worktrees/cargo-target/sd37-codex-sd37-e6.2-red
cd $T/apps/desktop && npm ci > $S/red_npm_ci.log 2>&1; echo "npm_ci=$?"
mkdir -p $S/smoke_red
RUN_DESKTOP_AGENT=sd37-e6-2-red node scripts/ui-smoke/run.mjs --only create-starfinder-soldier --out $S/smoke_red > $S/smoke_red.log 2>&1; echo "smoke_exit=$?"
echo RED_DONE
