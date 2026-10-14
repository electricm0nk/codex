#!/bin/bash
# E3.6 verification: clippy, sd24 audit, PF render hashes, SF seed lines, root + desktop full sweeps, npm gates, ui-smoke regression.
export PATH="$HOME/.cargo/bin:$PATH"
T=/home/ubuntu/workspace/repos/codex/.worktrees/sd38-e3
export CARGO_TARGET_DIR=/home/ubuntu/cargo-target/sd38-e3
export RETRO_ACTOR=sd38-e3.6
O=/home/ubuntu/cargo-target/sd38-e3/verify-e36
R=$O/results.txt
STEPS="clippy sd24_audit pf_render sf_lines_writer sf_lines_named root_full desktop_full npm_typecheck npm_test ui_smoke_doc ui_smoke"
mkdir -p $O/pf $O/sf $O/sf-named $O/logs
for s in $STEPS; do echo "$s not-run"; done > $R
step() { name=$1; shift; echo "$name running $(date -u +%T)" >> $O/progress.log
  ( "$@" ) > $O/logs/$name.log 2>&1; rc=$?
  sed -i "s/^$name .*/$name exit=$rc/" $R; echo "$name done rc=$rc $(date -u +%T)" >> $O/progress.log; }
cd $T
step clippy bash scripts/verify.sh --only clippy -j 8
step sd24_audit cargo test --locked -j 8 --test sd24_wired_integration_audit -- --test-threads=8
cd $T/apps/desktop/src-tauri
step pf_render env PF_SEED_RENDER_OUT=$O/pf cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture
step sf_lines_writer env SF_SEED_LINES_OUT=$O/sf cargo test --locked -j 8 --bins sf_seed_printed_features_equal_the_features_the_converted_records_grant -- --test-threads=8
step sf_lines_named env SF_SEED_LINES_OUT=$O/sf-named cargo test --locked -j 8 --bins sf_seed_lines_print_every_resolved_term_and_no_pcgen_residue -- --test-threads=8
cd $T
step root_full cargo test --locked -j 8 --no-fail-fast -- --test-threads=8
cd $T/apps/desktop/src-tauri
step desktop_full cargo test --locked -j 8 --no-fail-fast -- --test-threads=8
cd $T/apps/desktop
step npm_typecheck npm run typecheck
step npm_test npm test
step ui_smoke_doc npm run ui-smoke:doc
step ui_smoke env RUN_DESKTOP_AGENT=sd38-e3.6 node scripts/ui-smoke/run.mjs --only create-cyberpunk-streetrat-stats,landing-select-pathfinder
echo ALL_DONE >> $O/progress.log
