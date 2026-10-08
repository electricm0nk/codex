#!/usr/bin/env bash
# E2.MC adversarial merge check: one serial script, one cargo process at a time.
set -u
T=/home/ubuntu/workspace/worktrees/codex-sd37
L=$T/docs/release/SD-37-starfinder-1e/artifacts/epic_2/E2.MC_logs
cd "$T" || exit 9
export PATH="$HOME/.cargo/bin:$PATH" RETRO_ACTOR=sd37-e2-mc
export CARGO_TARGET_DIR="/home/ubuntu/workspace/worktrees/cargo-target/sd37-$(basename "$(git rev-parse --show-toplevel)")"
mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"
R=$L/results.txt
STAGES="e21_sheet_rule e21_roundtrip e22_list e22_stage e22_regen_diff convert_check mA_vars_reformat mA_restore mB_serde_attr mB_roundtrip mB_stage mB_restore mC_type_variant mC_stage mC_restore mD_extra_file mD_restore mE_vartable_edit mE_restore post_restore_stage seeds root_sweep desktop_sweep"
: > "$R"; for s in $STAGES; do echo "$s not-run" >> "$R"; done
set_r() { sed -i "s/^$1 .*/$1 $2/" "$R"; }
clean() { git diff --quiet && [ -z "$(git status --porcelain -- src schemas data)" ]; }
ST=$L/stage-out; mkdir -p "$ST"
git rev-parse HEAD > $L/head.txt

cargo test --locked -j 8 --lib sheet_rule -- --test-threads=8 > $L/e21_sheet_rule.log 2>&1; set_r e21_sheet_rule $?
cargo test --locked -j 8 --lib sheet_rule::schema_variant_tests::every_pathfinder -- --test-threads=8 --nocapture > $L/e21_roundtrip.log 2>&1; set_r e21_roundtrip $?
bash scripts/verify.sh --list > $L/e22_list.log 2>&1; set_r e22_list "$(grep -c rules-schema-check $L/e22_list.log)"
VERIFY_LOG_DIR=$L/vlog-e22_stage bash scripts/verify.sh --only rules-schema-check > $L/e22_stage.log 2>&1; set_r e22_stage $?
rm -rf $L/regen; RULES_SCHEMA_OUT=$L/regen cargo test --locked -j 8 --lib sheet_rule::schema_publish_tests::published_schemas_match_the_serde_types -- --test-threads=8 > $L/e22_regen.log 2>&1
diff -r $L/regen schemas/rules > $L/e22_regen_diff.log 2>&1; set_r e22_regen_diff $?
cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --check > $L/convert_check.log 2>&1; set_r convert_check $?

# mA: a _vars/ table reformatted (E2.1 planted a rule file; this is the VarTable branch)
F=data/sheet_rules/_vars/v000aae4c54547015.json
python3 - "$F" > $L/mA.edit.log <<'PY'
import sys; p=sys.argv[1]; b=open(p).read(); i=b.index('":'); open(p,'w').write(b[:i+2]+' '+b[i+2:]); print("inserted one space after first key in",p)
PY
git diff --stat >> $L/mA.edit.log
cargo test --locked -j 8 --lib sheet_rule::schema_variant_tests::every_pathfinder -- --test-threads=8 --nocapture > $L/mA.log 2>&1; set_r mA_vars_reformat $?
git checkout -- "$F"; clean; set_r mA_restore $?

# mB: a serde attribute dropped on an existing field (SheetRule.also always serialises)
sed -i '65s/#\[serde(default, skip_serializing_if = "Vec::is_empty")\]/#[serde(default)]/' src/rules_core/sheet_rule.rs
git diff > $L/mB.edit.log
cargo test --locked -j 8 --lib sheet_rule::schema_variant_tests::every_pathfinder -- --test-threads=8 --nocapture > $L/mB.log 2>&1; set_r mB_roundtrip $?
VERIFY_LOG_DIR=$L/vlog-mB_stage bash scripts/verify.sh --only rules-schema-check > $L/mB_stage.log 2>&1; set_r mB_stage $?
git checkout -- src/rules_core/sheet_rule.rs; clean; set_r mB_restore $?

# mC: a type edit with no regeneration (a new BonusTarget variant)
sed -i 's|^    Resolve,$|    Resolve,\n    /// planted\n    PlantedProbe,|' src/rules_core/sheet_rule.rs
git diff > $L/mC.edit.log
VERIFY_LOG_DIR=$L/vlog-mC_stage bash scripts/verify.sh --only rules-schema-check > $L/mC_stage.log 2>&1; set_r mC_stage $?
git checkout -- src/rules_core/sheet_rule.rs; clean; set_r mC_restore $?

# mD: a stale extra published file
cp schemas/rules/var_table.schema.json schemas/rules/stale.schema.json; echo "added schemas/rules/stale.schema.json" > $L/mD.edit.log
VERIFY_LOG_DIR=$L/vlog-mD_stage bash scripts/verify.sh --only rules-schema-check > $L/mD_stage.log 2>&1; set_r mD_extra_file $?
rm -f schemas/rules/stale.schema.json; clean; set_r mD_restore $?

# mE: hand edit of var_table.schema.json (E2.2 edited sheet_rule.schema.json)
sed -i '0,/"declared_by"/s/"declared_by"/"declared_byX"/' schemas/rules/var_table.schema.json
git diff > $L/mE.edit.log
VERIFY_LOG_DIR=$L/vlog-mE_stage bash scripts/verify.sh --only rules-schema-check > $L/mE_stage.log 2>&1; set_r mE_vartable_edit $?
git checkout -- schemas/rules/var_table.schema.json; clean; set_r mE_restore $?

VERIFY_LOG_DIR=$L/vlog-post_restore_stage bash scripts/verify.sh --only rules-schema-check > $L/post_restore_stage.log 2>&1; set_r post_restore_stage $?

rm -rf $L/seed-render; mkdir -p $L/seed-render
( cd apps/desktop/src-tauri && PF_SEED_RENDER_OUT=$L/seed-render cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture ) > $L/seeds.log 2>&1; set_r seeds $?

df -h / > $L/df.txt; free -g >> $L/df.txt
cargo test --locked -j 8 --workspace --no-fail-fast -- --test-threads=8 > $L/root.sweep.log 2>&1; set_r root_sweep $?
( cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8 ) > $L/desktop.sweep.log 2>&1; set_r desktop_sweep $?
echo DONE >> "$R"
