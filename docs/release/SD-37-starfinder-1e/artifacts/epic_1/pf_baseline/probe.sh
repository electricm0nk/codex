#!/bin/bash
set -u
export PATH="$HOME/.cargo/bin:$PATH"
T=/home/ubuntu/workspace/worktrees/codex-sd37
S=/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/33627023-04e6-4757-9afa-d5c1463408f3/scratchpad/e1
PHASE=$1
cd $T
export CARGO_TARGET_DIR="/home/ubuntu/workspace/worktrees/cargo-target/sd37-codex-sd37"
echo "== HEAD $(git rev-parse HEAD) phase=$PHASE"
cp apps/desktop/src-tauri/src/character_hub.rs $S/character_hub.rs.preprobe
cat $S/probe.rs >> apps/desktop/src-tauri/src/character_hub.rs
for run in a b; do
  rm -rf $S/$PHASE.render.$run
  ( cd apps/desktop/src-tauri && E1_PROBE_OUT=$S/$PHASE.render.$run cargo test --locked -j 8 --bins e1_pf_render_probe_dump -- --test-threads=8 ) > $S/$PHASE.probe.$run.log 2>&1; echo "run $run exit=$?"
  grep -E 'test result' $S/$PHASE.probe.$run.log | tail -1
  (cd $S/$PHASE.render.$run && sha256sum *.json)
done
cp $S/character_hub.rs.preprobe apps/desktop/src-tauri/src/character_hub.rs
git status --porcelain
echo DONE
