#!/usr/bin/env bash
# E6.1: app build on a clean checkout of cc38b39ce0 (own worktree, own CARGO_TARGET_DIR).
set -u
C=/home/ubuntu/workspace/worktrees/codex-sd37-e6.1-clean
L=/home/ubuntu/workspace/worktrees/codex-sd37/docs/release/SD-37-starfinder-1e/artifacts/epic_6/E6.1_logs
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=/home/ubuntu/workspace/worktrees/cargo-target/sd37-codex-sd37-e6.1-clean
R=$L/clean_build_results.txt
printf 'npm-ci not-run\nfrontend-build not-run\ndesktop-cargo-build not-run\n' > $R
cd $C/apps/desktop
npm ci --prefer-offline --no-audit --no-fund > $L/clean_npm_ci.log 2>&1; sed -i "s/^npm-ci .*/npm-ci exit=$?/" $R
npm run build > $L/clean_frontend_build.log 2>&1; sed -i "s/^frontend-build .*/frontend-build exit=$?/" $R
cd src-tauri
cargo build --locked -j 8 > $L/clean_desktop_build.log 2>&1; sed -i "s/^desktop-cargo-build .*/desktop-cargo-build exit=$?/" $R
echo DONE >> $R
