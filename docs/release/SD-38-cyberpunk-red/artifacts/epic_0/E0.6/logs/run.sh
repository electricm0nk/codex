#!/bin/bash
# usage: run.sh <label> <tree> <target-dir> <outdir>
export PATH="$HOME/.cargo/bin:$PATH"
L=$1; T=$2; export CARGO_TARGET_DIR=$3; O=$4
R=$O/results.txt
mkdir -p $O/pf $O/sf $O/sf-named $O/logs
for s in pf_render sf_lines_writer sf_lines_named root_lib_sf_seed convert_pf convert_sf; do echo "$L $s not-run" ; done > $R
step() { name=$1; shift; echo "$L $name running $(date -u +%T)" >> $O/progress.log
  ( "$@" ) > $O/logs/$name.log 2>&1; rc=$?
  sed -i "s/^$L $name .*/$L $name exit=$rc/" $R; echo "$L $name done rc=$rc $(date -u +%T)" >> $O/progress.log; }
cd $T/apps/desktop/src-tauri
step pf_render env PF_SEED_RENDER_OUT=$O/pf cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture
step sf_lines_writer env SF_SEED_LINES_OUT=$O/sf cargo test --locked -j 8 --bins sf_seed_printed_features_equal_the_features_the_converted_records_grant -- --test-threads=8
step sf_lines_named env SF_SEED_LINES_OUT=$O/sf-named cargo test --locked -j 8 --bins sf_seed_lines_print_every_resolved_term_and_no_pcgen_residue -- --test-threads=8
cd $T
step root_lib_sf_seed cargo test --locked -j 8 --lib sf_seed -- --test-threads=8
step convert_pf cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --check
step convert_sf cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --system starfinder-1e --check
echo "$L ALL_DONE" >> $O/progress.log
