#!/usr/bin/env bash
# E4.1 verify: clippy, sd24 audit, PF render hashes, SF seed lines, desktop full sweep.
export PATH="$HOME/.cargo/bin:$PATH" CARGO_TARGET_DIR=/home/ubuntu/cargo-target/sd38-e4 RETRO_ACTOR=sd38-e4.1
T=/home/ubuntu/workspace/repos/codex/.worktrees/sd38-e4
V=/home/ubuntu/cargo-target/sd38-e4/verify
R=$V/results.txt
printf '%s not-run\n' clippy sd24_audit pf_render sf_lines desktop_full > $R
mark() { sed -i "s/^$1 .*/$1 $2/" $R; }
step() { name=$1; shift; mark $name running; echo "== $name $(date -Is)" >> $V/progress.log
  if ( "$@" ) > $V/$name.log 2>&1; then mark $name pass; else mark $name "FAIL(exit $?)"; fi; }
step clippy bash -c "cd $T && bash scripts/verify.sh --only clippy -j 8"
step sd24_audit bash -c "cd $T && cargo test --locked -j 8 --test sd24_wired_integration_audit -- --test-threads=8"
step pf_render bash -c "cd $T/apps/desktop/src-tauri && rm -rf $V/pf && PF_SEED_RENDER_OUT=$V/pf cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8"
step sf_lines bash -c "cd $T/apps/desktop/src-tauri && rm -rf $V/sf && SF_SEED_LINES_OUT=$V/sf cargo test --locked -j 8 --bins sf_seed_printed_features_equal_the_features_the_converted_records_grant -- --test-threads=8"
step desktop_full bash -c "cd $T/apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8"
echo "== done $(date -Is)" >> $V/progress.log
