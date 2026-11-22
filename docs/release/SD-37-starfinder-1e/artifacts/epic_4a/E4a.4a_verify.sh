#!/usr/bin/env bash
# E4a.4a one-pass verify (repo root; SCRATCH outside the repo; CARD_BASE = the tranche/17 SHA the
# card started from). Not-run skeleton first; each stage log ends with exit=<code>.
export PATH="$HOME/.cargo/bin:$PATH"
L=docs/release/SD-37-starfinder-1e/artifacts/epic_4a/E4a.4a_logs
SCRATCH=${SCRATCH:?}
SD=docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py
eval "$(scripts/fetch-pcgen-oracle.sh --check --quiet)"
MONSTER_BOOKS=$(python3 -c "import sys; sys.path.insert(0,'scripts'); import transcribe_monster_tables as t; print(' '.join(sorted(t.BOOKS)))")
COMPANION_BOOKS="advanced_race_guide advanced_players_guide bestiary bestiary_2 bestiary_3 bestiary_4 bestiary_5 bestiary_6 book_of_the_damned_volume_1 core_rulebook horror_adventures inner_sea_combat inner_sea_intrigue monster_codex ultimate_magic ultimate_wilderness"
STAGES="build proof_rerun proof_status package_check acceptance plants golden pf_seed_render pf_check residue pf_dump pf_structural_diff structural_plant python_tests pi_sweep clippy verify_root"
mkdir -p $L
for s in $STAGES; do echo "not-run" > $L/$s.log; done
run() { local s=$1; shift; { "$@"; echo "exit=$?"; } > $L/$s.log 2>&1; }
BIN="$CARGO_TARGET_DIR/debug"

run build cargo build --locked -j 8 -p codex-ingest --bin gen_feat_gap_tables --bin gen_equipment_gap_tables --bin ingest_spells --bin ingest_class_spell_levels_arg

proof() {
  set -o pipefail
  echo "== gen_feat_gap_tables"; "$BIN/gen_feat_gap_tables" || return 1
  echo "== gen_equipment_gap_tables"; "$BIN/gen_equipment_gap_tables" || return 1
  echo "== ingest_spells"; "$BIN/ingest_spells" 2>&1 || return 1
  echo "== ingest_class_spell_levels_arg --verify --emit"; "$BIN/ingest_class_spell_levels_arg" --verify --emit || return 1
  for b in $MONSTER_BOOKS; do echo "== transcribe_monster_tables $b"; python3 scripts/transcribe_monster_tables.py "$b" 2>&1 || return 1; done
  for b in $COMPANION_BOOKS; do echo "== transcribe_companion_tables $b"; python3 scripts/transcribe_companion_tables.py "$b" 2>&1 || return 1; done
}
run proof_rerun proof
run proof_status bash -c 'out=$(git status --porcelain data/rules_tables crates/codex-ingest/src/pcgen_import); echo "status_lines=$(printf "%s" "$out" | awk "NF{n++} END{print n+0}")"; printf "%s\n" "$out"; test -z "$out"'
run package_check cargo run --locked -j 8 -q -p codex --bin rules_tables_package -- --check
run acceptance bash -c "grep -rnE '\"src/rules_core/rules_tables/|f\"src/rules_core/rules_tables/' crates/codex-ingest/src/bin scripts --include='*.rs' --include='*.py' | awk -F: '{print \$1}' | sort -u | awk '!/scripts\/tests\//' | awk 'END{print NR; exit NR>0}'"

plants() {
  # P1: a hand edit to a Rust generator's table is undone by its re-run.
  local f=data/rules_tables/feat_gap_tables/CORE_RULEBOOK_FEAT_GAP_ROWS.json
  sha256sum "$f"; sed -i '0,/"name": "/s//"name": "PLANTED /' "$f"; sha256sum "$f"
  git status --porcelain "$f"; test -n "$(git status --porcelain "$f")" || { echo "P1 plant did not take"; return 1; }
  "$BIN/gen_feat_gap_tables" > /dev/null || return 1
  sha256sum "$f"; test -z "$(git status --porcelain "$f")" || { echo "P1 not restored"; return 1; }
  echo "P1 restored by gen_feat_gap_tables"
  # P2: the same for a Python transcriber's table.
  local g=data/rules_tables/bestiary_2/monster_data/MONSTERS.json
  sha256sum "$g"; sed -i '0,/"name": "/s//"name": "PLANTED /' "$g"; sha256sum "$g"
  test -n "$(git status --porcelain "$g")" || { echo "P2 plant did not take"; return 1; }
  python3 scripts/transcribe_monster_tables.py bestiary_2 > /dev/null 2>&1 || return 1
  sha256sum "$g"; test -z "$(git status --porcelain "$g")" || { echo "P2 not restored"; return 1; }
  echo "P2 restored by transcribe_monster_tables bestiary_2"
}
run plants plants
run golden cargo test --locked -j 8 --lib every_catalog_table_view_and_lookup_matches_its_golden_digest -- --test-threads=8
( cd apps/desktop/src-tauri && PF_SEED_RENDER_OUT="$SCRATCH/pfrender" cargo test --locked -j 8 --bins pf_seed_render_hash -- --nocapture --test-threads=8; echo "exit=$?" ) > $L/pf_seed_render.log 2>&1
run pf_check cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --check
run residue python3 scripts/pcgen_residue_gate.py --check --closure
rm -rf "$SCRATCH/pfdump" "$SCRATCH/pfdump-planted"
run pf_dump cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --dump "$SCRATCH/pfdump"
{ python3 $SD "$SCRATCH/pfdump" --baseline data/sheet_rules; echo "exit=$?"; echo "diff -rq:"; diff -rq "$SCRATCH/pfdump" data/sheet_rules; echo "diff_exit=$?"; } > $L/pf_structural_diff.log 2>&1
structural_plant() {
  # The structural diff must fail on a planted removal (one rule file deleted from a copy).
  cp -r "$SCRATCH/pfdump" "$SCRATCH/pfdump-planted"
  local victim; victim=$(find "$SCRATCH/pfdump-planted" -name '*.json' ! -path '*/_vars/*' ! -name '_report.json' | sort | head -1)
  echo "planted: removed ${victim#$SCRATCH/pfdump-planted/}"; rm "$victim"
  python3 $SD "$SCRATCH/pfdump-planted" --baseline data/sheet_rules > "$SCRATCH/planted.out" 2>&1; local rc=$?
  tail -5 "$SCRATCH/planted.out"; echo "planted_diff_exit=$rc"; test "$rc" -ne 0
}
run structural_plant structural_plant
run python_tests python3 -m unittest scripts/tests/test_transcribe_monster_tables.py scripts/tests/test_transcribers_write_the_data_package.py
run pi_sweep bash scripts/verify.sh --only pi-sweep
run clippy bash scripts/verify.sh --only clippy
run verify_root cargo test --locked -j 8 --workspace --no-fail-fast -- --test-threads=8
echo RUN_DONE
