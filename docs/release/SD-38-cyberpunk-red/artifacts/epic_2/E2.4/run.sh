#!/bin/bash
export PATH="$HOME/.cargo/bin:$PATH"
T=/home/ubuntu/workspace/repos/codex/.worktrees/sd38-a
export CARGO_TARGET_DIR=/home/ubuntu/cargo-target/sd38-a
O=/home/ubuntu/cargo-target/sd38-a/verify-e24
R=$O/results.txt
STEPS="sd24_audit pf_render sf_lines_writer sf_lines_named root_lib_sf_seed convert_pf convert_sf root_full desktop_full"
for s in $STEPS; do echo "$s not-run"; done > $R
mkdir -p $O/pf $O/sf $O/sf-named $O/logs
step() { name=$1; shift; echo "$name running $(date -u +%T)" >> $O/progress.log
  ( "$@" ) > $O/logs/$name.log 2>&1; rc=$?
  sed -i "s/^$name .*/$name exit=$rc/" $R; echo "$name done rc=$rc $(date -u +%T)" >> $O/progress.log; }
cd $T
step sd24_audit cargo test --locked -j 8 --test sd24_wired_integration_audit -- --test-threads=8
cd $T/apps/desktop/src-tauri
step pf_render env PF_SEED_RENDER_OUT=$O/pf cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture
step sf_lines_writer env SF_SEED_LINES_OUT=$O/sf cargo test --locked -j 8 --bins sf_seed_printed_features_equal_the_features_the_converted_records_grant -- --test-threads=8
step sf_lines_named env SF_SEED_LINES_OUT=$O/sf-named cargo test --locked -j 8 --bins sf_seed_lines_print_every_resolved_term_and_no_pcgen_residue -- --test-threads=8
cd $T
step root_lib_sf_seed cargo test --locked -j 8 --lib sf_seed -- --test-threads=8
step convert_pf cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --check
step convert_sf cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --system starfinder-1e --check
step root_full cargo test --locked -j 8 --no-fail-fast -- --test-threads=8
cd $T/apps/desktop/src-tauri
step desktop_full cargo test --locked -j 8 --no-fail-fast -- --test-threads=8
echo ALL_DONE >> $O/progress.log
