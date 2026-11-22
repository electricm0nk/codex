#!/bin/bash
# E2.MC runner: re-run E2.1-E2.10 acceptance commands on origin/tranche-18, G-1 PF/SF, then planted mutations.
# usage: run.sh <tree> <outdir>
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=/home/ubuntu/cargo-target/sd38-main
export RETRO_ACTOR=sd38-e2.mc
T=$1; O=$2; R=$O/results.txt
mkdir -p $O/logs $O/pf-head $O/sf-head $O/sf-named
STEPS="b01_stats b02_skills b03_armor b04_critical b05_cyberware b06_roles b07_improvement b08_reputation b09_netrunning b10_sheet b11_cpr_adapter_bins b12_lib_cpr_all b13_pf_render b14_sf_writer b15_sf_named b16_lib_sf_seed b17_convert_pf b18_convert_sf m1_hp_floor m2_drop_d11 m3_armor_sum m4_media_emp_string m5a_cyberarm_plus1_off m5b_suit_plus1_off m5c_net_actions_plus1 r1_restored_lib_cpr r2_restored_cpr_adapter"
for s in $STEPS; do echo "$s not-run"; done > $R
step() { name=$1; shift; echo "$name running $(date -u +%T)" >> $O/progress.log
  ( cd $T && "$@" ) > $O/logs/$name.log 2>&1; rc=$?
  sed -i "s/^$name .*/$name exit=$rc/" $R; echo "$name done rc=$rc $(date -u +%T)" >> $O/progress.log; }
restored() { git -C $T diff --quiet && echo RESTORED_CLEAN || { echo DIRTY; git -C $T diff --stat; }; }
plant() { # name file sed-expr
  name=$1; f=$2; expr=$3
  sed -i "$expr" $T/$f
  git -C $T diff > $O/logs/$name.diff
  if git -C $T diff --quiet; then echo "$name PLANT_NOT_APPLIED" >> $O/progress.log; sed -i "s/^$name .*/$name plant-not-applied/" $R; return; fi
  step $name cargo test --locked -j 8 --lib cpr:: -- --test-threads=8
  git -C $T checkout -- $f; restored > $O/logs/$name.restore.txt
}
lib() { cargo test --locked -j 8 --lib "$1" -- --test-threads=8; }
# ---- E2.1-E2.10 acceptance commands
step b01_stats lib cpr::stats
step b02_skills lib cpr::skills
step b03_armor lib cpr::armor
step b04_critical lib cpr::critical
step b05_cyberware lib cpr::cyberware
step b06_roles lib cpr::roles
step b07_improvement lib cpr::improvement
step b08_reputation lib cpr::reputation
step b09_netrunning lib cpr::netrunning
step b10_sheet lib cpr::sheet
step b11_cpr_adapter_bins bash -c 'cd apps/desktop/src-tauri && cargo test --locked -j 8 --bins cpr_adapter -- --test-threads=8'
step b12_lib_cpr_all lib cpr::
# ---- G-1 (head vs E0.6's committed C1 base records)
step b13_pf_render bash -c "cd apps/desktop/src-tauri && PF_SEED_RENDER_OUT=$O/pf-head cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture"
step b14_sf_writer bash -c "cd apps/desktop/src-tauri && SF_SEED_LINES_OUT=$O/sf-head cargo test --locked -j 8 --bins sf_seed_printed_features_equal_the_features_the_converted_records_grant -- --test-threads=8"
step b15_sf_named bash -c "cd apps/desktop/src-tauri && SF_SEED_LINES_OUT=$O/sf-named cargo test --locked -j 8 --bins sf_seed_lines_print_every_resolved_term_and_no_pcgen_residue -- --test-threads=8"
step b16_lib_sf_seed lib sf_seed
step b17_convert_pf cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --check
step b18_convert_sf cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --system starfinder-1e --check
# ---- Plants (each restored with git checkout before the next)
plant m1_hp_floor src/rules_core/cpr/stats.rs '17s|(u16::from(body) + u16::from(will)).div_ceil(2);|(u16::from(body) + u16::from(will)) / 2;|'
plant m2_drop_d11 src/rules_core/cpr/cyberware.rs 's|                if requested > cap {|                if false \&\& requested > cap {|'
plant m3_armor_sum src/rules_core/cpr/armor.rs 's|        \*slot = (\*slot).max(sp);|        *slot = (*slot).saturating_add(sp);|'
plant m4_media_emp_string src/rules_core/cpr/cyberware.rs 's|    humanity_after_loss(base_emp, humanity_loss).div_euclid(10).max(0)|    (i16::from(base_emp) - 2 * (humanity_loss / 10)).max(0)|'
plant m5a_cyberarm_plus1_off src/rules_core/cpr/netrunning.rs 's|    let cyberarm = u8::from(extras.cyberarm_deck);|    let cyberarm = 0u8;|'
plant m5b_suit_plus1_off src/rules_core/cpr/netrunning.rs 's|    let suit = u8::from(extras.bodyweight_suit);|    let suit = 0u8;|'
plant m5c_net_actions_plus1 src/rules_core/cpr/netrunning.rs 's|        10 => Ok(5),|        10 => Ok(6),|'
# ---- Restored tree green again
step r1_restored_lib_cpr lib cpr::
step r2_restored_cpr_adapter bash -c 'cd apps/desktop/src-tauri && cargo test --locked -j 8 --bins cpr_adapter -- --test-threads=8'
echo ALL_DONE >> $O/progress.log
