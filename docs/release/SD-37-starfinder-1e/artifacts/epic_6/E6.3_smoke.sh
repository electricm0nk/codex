#!/usr/bin/env bash
# E6.3 smoke: MODE=red runs with CharacterHubPage.tsx at HEAD (pre-E6.3 routing), MODE=green with the change.
set -u
T=/home/ubuntu/workspace/worktrees/codex-sd37
S=/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/33627023-04e6-4757-9afa-d5c1463408f3/scratchpad/e63
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=/home/ubuntu/workspace/worktrees/cargo-target/sd37-codex-sd37
cd $T/apps/desktop
P=src/characterHub/CharacterHubPage.tsx
if [ "$MODE" = red ]; then cp $P $S/CharacterHubPage.e63.tsx; git show HEAD:apps/desktop/$P > $P; fi
RUN_DESKTOP_AGENT=sd37-e6-3-$MODE node scripts/ui-smoke/run.mjs --only create-starfinder-soldier,open-starfinder-sheet,landing-select-pathfinder --out $S/smoke_$MODE > $S/smoke_$MODE.log 2>&1
echo "exit=$?" >> $S/smoke_$MODE.log
if [ "$MODE" = red ]; then cp $S/CharacterHubPage.e63.tsx $P; fi
.claude/skills/run-desktop/driver.sh stop >/dev/null 2>&1 || true
echo DONE >> $S/smoke_$MODE.log
