#!/usr/bin/env bash
# E4.MC re-run of the E4 cargo acceptance commands + PF/SF seed records (one cargo process at a time).
export PATH="$HOME/.cargo/bin:$PATH" CARGO_TARGET_DIR=/home/ubuntu/cargo-target/sd38-e4 RETRO_ACTOR=sd38-e4.mc
T=/home/ubuntu/workspace/repos/codex/.worktrees/sd38-e4
V=/home/ubuntu/cargo-target/sd38-e4/verify-mc
mkdir -p $V; R=$V/results.txt
printf '%s not-run\n' cpr_adapter cpr_sheet_print pf_render sf_lines > $R
mark() { sed -i "s/^$1 .*/$1 $2/" $R; }
step() { name=$1; shift; mark $name running; echo "== $name $(date -Is)" >> $V/progress.log
  if ( "$@" ) > $V/$name.log 2>&1; then mark $name pass; else mark $name "FAIL(exit $?)"; fi; }
step cpr_adapter bash -c "cd $T/apps/desktop/src-tauri && cargo test --locked -j 8 --bins cpr_adapter -- --test-threads=8"
step cpr_sheet_print bash -c "cd $T/apps/desktop/src-tauri && rm -rf $V/lines && mkdir -p $V/lines && CPR_SHEET_LINES_OUT=$V/lines cargo test --locked -j 8 --bins cpr_sheet_print -- --test-threads=8"
step pf_render bash -c "cd $T/apps/desktop/src-tauri && rm -rf $V/pf && PF_SEED_RENDER_OUT=$V/pf cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8"
step sf_lines bash -c "cd $T/apps/desktop/src-tauri && rm -rf $V/sf && mkdir -p $V/sf && SF_SEED_LINES_OUT=$V/sf cargo test --locked -j 8 --bins sf_seed_printed_features_equal_the_features_the_converted_records_grant -- --test-threads=8"
echo "== done $(date -Is)" >> $V/progress.log
