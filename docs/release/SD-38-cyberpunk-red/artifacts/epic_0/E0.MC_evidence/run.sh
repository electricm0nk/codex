#!/bin/bash
# E0.MC runner: re-run E0.1-E0.6 acceptance commands on origin/tranche-18, then three planted mutations.
# usage: run.sh <tree> <outdir>
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=/home/ubuntu/cargo-target/sd38-main
export RUN_DESKTOP_AGENT=sd38-e0.mc
export RETRO_ACTOR=sd38-e0.mc
T=$1; O=$2; R=$O/results.txt
mkdir -p $O/logs $O/pf-head $O/sf-head $O/pf-m3
STEPS="a01_lib_game_system a02_trt a03_trt_planted a04_convert_cpr a05_build_root a06_build_desktop a07_license_py a08_license_awk a09_pi_residue a10_packaged_resources a11_no_baked a12_rsa_bins a13_cpr_adapter_bins a14_lib_saved_character a15_lib_cpr_store a16_npm a17_grep_cyberpunk a18_ui_smoke a19_ui_smoke_doc a20_pf_render a21_sf_writer a22_sf_named a23_lib_sf_seed a24_convert_pf a25_convert_sf m1_rsa_mutated m1_restored m2_packaged_mutated m2_restored m3_trt_mutated m3_pf_render_mutated m3_restored_trt"
for s in $STEPS; do echo "$s not-run"; done > $R
step() { name=$1; shift; echo "$name running $(date -u +%T)" >> $O/progress.log
  ( cd $T && "$@" ) > $O/logs/$name.log 2>&1; rc=$?
  sed -i "s/^$name .*/$name exit=$rc/" $R; echo "$name done rc=$rc $(date -u +%T)" >> $O/progress.log; }
clean() { test -z "$(git -C $T status --porcelain)" && echo CLEAN || { echo DIRTY; git -C $T status --porcelain; }; }

# ---- E0.1
step a01_lib_game_system cargo test --locked -j 8 --lib game_system -- --test-threads=8
step a02_trt bash scripts/verify.sh --only tauri-resources-tracked
sed -i 's|GameSystem::CyberpunkRed => None,|GameSystem::CyberpunkRed => Some("data/x/sheet_rules"),|' $T/src/rules_core/game_system.rs
git -C $T diff --stat > $O/logs/a03_plant.diff
step a03_trt_planted bash scripts/verify.sh --only tauri-resources-tracked
git -C $T checkout -- src/rules_core/game_system.rs; clean > $O/logs/a03_restore.txt
step a04_convert_cpr cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --system cyberpunk-red --check
step a05_build_root cargo build --locked -j 8
step a06_build_desktop bash -c 'cd apps/desktop/src-tauri && cargo build --locked -j 8'
# ---- E0.2
step a07_license_py python3 -c "import json;d=json.load(open('data/cyberpunk-red/corpus/LICENSE.json'));l=d['license_declaration'];assert l['open_game_content'] is False and l['policy']=='R. Talsorian Homebrew Content Policy' and l['operator_ruling']=='ship whole text, private single-user build, 2026-10-13' and d['operator_sign_off']['signed_off'] is True;print('LICENSE_OK')"
step a08_license_awk awk '/^## Cyberpunk RED/{f=1} f && /DEVIATION/{n++} END{print n+0}' docs/governance/license-matrix.md
step a09_pi_residue bash scripts/verify.sh --only pi-sweep --only pcgen-residue-gate
# ---- E0.3
step a10_packaged_resources bash -c 'cd apps/desktop/src-tauri && cargo test --locked -j 8 --test packaged_resources -- --test-threads=8'
step a11_no_baked cargo test --locked -j 8 --test no_baked_manifest_dir_in_production -- --test-threads=8
# ---- E0.4
step a12_rsa_bins bash -c 'cd apps/desktop/src-tauri && cargo test --locked -j 8 --bins rule_system_adapter -- --test-threads=8'
step a13_cpr_adapter_bins bash -c 'cd apps/desktop/src-tauri && cargo test --locked -j 8 --bins cpr_adapter -- --test-threads=8'
step a14_lib_saved_character cargo test --locked -j 8 --lib saved_character -- --test-threads=8
step a15_lib_cpr_store cargo test --locked -j 8 --lib cpr::store -- --test-threads=8
# ---- E0.5
step a16_npm bash -c 'cd apps/desktop && npm run typecheck && npm test'
step a17_grep_cyberpunk bash -c "grep -n \"'cyberpunk'\" apps/desktop/src/characterHub/LandingScreen.tsx | awk 'END{print NR}'"
step a18_ui_smoke bash -c 'cd apps/desktop && npm run ui-smoke -- --only landing-render,landing-ruleset-chips,load-character-render,select-cyberpunk-red,landing-select-pathfinder,open-seed-aldric'
step a19_ui_smoke_doc bash -c 'cd apps/desktop && npm run ui-smoke:doc && git status --porcelain'
# ---- E0.6 (head only; base hashes from E0.6's committed base record)
step a20_pf_render bash -c "cd apps/desktop/src-tauri && PF_SEED_RENDER_OUT=$O/pf-head cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture"
step a21_sf_writer bash -c "cd apps/desktop/src-tauri && SF_SEED_LINES_OUT=$O/sf-head cargo test --locked -j 8 --bins sf_seed_printed_features_equal_the_features_the_converted_records_grant -- --test-threads=8"
step a22_sf_named bash -c "cd apps/desktop/src-tauri && SF_SEED_LINES_OUT=$O/sf-named cargo test --locked -j 8 --bins sf_seed_lines_print_every_resolved_term_and_no_pcgen_residue -- --test-threads=8"
step a23_lib_sf_seed cargo test --locked -j 8 --lib sf_seed -- --test-threads=8
step a24_convert_pf cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --check
step a25_convert_sf cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --system starfinder-1e --check

# ---- Mutation 1: route cyberpunk-red back to StubAdapter
sed -i 's|id if id == codex::rules_core::game_system::GameSystem::CyberpunkRed.id() => {|id if id == codex::rules_core::game_system::GameSystem::CyberpunkRed.id() \&\& false => {|' $T/apps/desktop/src-tauri/src/rule_system_adapter.rs
git -C $T diff > $O/logs/m1.diff
step m1_rsa_mutated bash -c 'cd apps/desktop/src-tauri && cargo test --locked -j 8 --bins rule_system_adapter -- --test-threads=8'
git -C $T checkout -- apps/desktop/src-tauri/src/rule_system_adapter.rs; clean > $O/logs/m1_restore.txt
step m1_restored bash -c 'cd apps/desktop/src-tauri && cargo test --locked -j 8 --bins rule_system_adapter -- --test-threads=8'
# ---- Mutation 2: remove the CPR bundle.resources entry
sed -i '\|"../../../data/cyberpunk-red/corpus/": "data/cyberpunk-red/corpus/",|d' $T/apps/desktop/src-tauri/tauri.conf.json
git -C $T diff > $O/logs/m2.diff
step m2_packaged_mutated bash -c 'cd apps/desktop/src-tauri && cargo test --locked -j 8 --test packaged_resources -- --test-threads=8'
git -C $T checkout -- apps/desktop/src-tauri/tauri.conf.json; clean > $O/logs/m2_restore.txt
step m2_restored bash -c 'cd apps/desktop/src-tauri && cargo test --locked -j 8 --test packaged_resources -- --test-threads=8'
# ---- Mutation 3: change PF's sheet_rules_relative string
sed -i 's|GameSystem::Pathfinder1e => Some("data/sheet_rules"),|GameSystem::Pathfinder1e => Some("data/sheet_rules_moved"),|' $T/src/rules_core/game_system.rs
git -C $T diff > $O/logs/m3.diff
step m3_trt_mutated bash scripts/verify.sh --only tauri-resources-tracked
step m3_pf_render_mutated bash -c "cd apps/desktop/src-tauri && PF_SEED_RENDER_OUT=$O/pf-m3 cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture"
git -C $T checkout -- src/rules_core/game_system.rs; clean > $O/logs/m3_restore.txt
step m3_restored_trt bash scripts/verify.sh --only tauri-resources-tracked
echo ALL_DONE >> $O/progress.log
