#!/usr/bin/env bash
# E6.4 smoke: MODE=red runs with LandingScreen.tsx + CharacterHubPage.tsx at HEAD (pre-E6.4: no
# Starfinder catalogs on the landing), MODE=green with the change. Isolated app-data root (run.mjs).
set -u
T=/home/ubuntu/workspace/worktrees/codex-sd37
S=/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/33627023-04e6-4757-9afa-d5c1463408f3/scratchpad/e64
mkdir -p $S
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=/home/ubuntu/workspace/worktrees/cargo-target/sd37-codex-sd37
cd $T/apps/desktop
FILES="src/characterHub/LandingScreen.tsx src/characterHub/CharacterHubPage.tsx"
if [ "$MODE" = red ]; then for f in $FILES; do cp $f $S/$(basename $f).e64; git show HEAD:apps/desktop/$f > $f; done; fi
RUN_DESKTOP_AGENT=sd37-e6-4-$MODE node scripts/ui-smoke/run.mjs --only browse-starfinder-catalog,landing-select-pathfinder --out $S/smoke_$MODE > $S/smoke_$MODE.log 2>&1
echo "exit=$?" >> $S/smoke_$MODE.log
if [ "$MODE" = red ]; then for f in $FILES; do cp $S/$(basename $f).e64 $f; done; sha256sum $FILES >> $S/smoke_$MODE.log; fi
.claude/skills/run-desktop/driver.sh stop >/dev/null 2>&1 || true
echo DONE >> $S/smoke_$MODE.log
