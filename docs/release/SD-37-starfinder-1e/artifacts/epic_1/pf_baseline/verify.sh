#!/bin/bash
set -u
export PATH="$HOME/.cargo/bin:$PATH"
S=/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/33627023-04e6-4757-9afa-d5c1463408f3/scratchpad/e1
cd /home/ubuntu/workspace/worktrees/codex-sd37
export CARGO_TARGET_DIR=/home/ubuntu/workspace/worktrees/cargo-target/sd37-codex-sd37
printf 'root-workspace: not-run\ndesktop: not-run\n' > $S/verify.results
cargo test --locked -j 8 --workspace --no-fail-fast -- --test-threads=8 > $S/verify.root.log 2>&1; r=$?
sed -i "s/^root-workspace: .*/root-workspace: exit=$r/" $S/verify.results
( cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8 ) > $S/verify.desktop.log 2>&1; d=$?
sed -i "s/^desktop: .*/desktop: exit=$d/" $S/verify.results
echo DONE >> $S/verify.results
