#!/usr/bin/env bash
# E4a.MC attempt 3 — adversarial re-run on origin/tranche/17 (repo root; SCRATCH outside the repo).
# Not-run skeleton first; every stage log ends with exit=<code>. Mutations are restored and the
# package sha256 set is compared before/after.
export PATH="$HOME/.cargo/bin:$PATH"
L=docs/release/SD-37-starfinder-1e/artifacts/epic_4a/E4a.MC_logs/attempt3
SCRATCH=${SCRATCH:?}
eval "$(scripts/fetch-pcgen-oracle.sh --check --quiet)"
MONSTER_BOOKS=$(python3 -c "import sys; sys.path.insert(0,'scripts'); import transcribe_monster_tables as t; print(' '.join(sorted(t.BOOKS)))")
COMPANION_BOOKS="advanced_race_guide advanced_players_guide bestiary bestiary_2 bestiary_3 bestiary_4 bestiary_5 bestiary_6 book_of_the_damned_volume_1 core_rulebook horror_adventures inner_sea_combat inner_sea_intrigue monster_codex ultimate_magic ultimate_wilderness"
STAGES="acceptance acceptance_py build package_sha_before proof_rerun proof_status gen_plants package_check golden_clean pf_clean mut_monster_golden mut_wizard_fort_pf mut_type_roundtrip package_sha_after verify_stages epic_tests python_tests converter_check residue desktop_full"
mkdir -p $L
for s in $STAGES; do echo "not-run" > $L/$s.log; done
run() { local s=$1; shift; { "$@"; echo "exit=$?"; } > $L/$s.log 2>&1; }
BIN="$CARGO_TARGET_DIR/debug"
pkgsha() { find data/rules_tables -type f -print0 | sort -z | xargs -0 sha256sum | sha256sum; find data/rules_tables -type f | awk 'END{print "files="NR}'; }

acceptance() {
  echo "== E4a.2"; for r in src crates apps/desktop/src-tauri tests; do grep -rlE 'rules_tables::' $r --include='*.rs' | awk '!/src\/rules_core\/rules_tables\//' | awk 'END{print NR}'; done
  echo "== E4a.3 module"; test ! -e src/rules_core/rules_tables || grep -rcF '.lst' src/rules_core/rules_tables | awk -F: '{s+=$2} END{print s+0}'; echo "module_rc=$?"
  echo "== E4a.3 package"; grep -rcF '.lst' data/rules_tables | awk -F: '{s+=$2} END{print s+0}'
  echo "== E4a.3 reported-only (rest of src/rules_core)"; grep -rcF '.lst' src/rules_core --exclude-dir=rules_tables | awk -F: '{s+=$2} END{print s+0}'
  echo "== E4a.4"; test ! -e src/rules_core/rules_tables || find src/rules_core/rules_tables -name '*.rs' | awk 'END{exit NR>0}'; echo "e4a4_rc=$?"
  echo "== E4a.4a"; grep -rnE '"src/rules_core/rules_tables/|f"src/rules_core/rules_tables/' crates/codex-ingest/src/bin scripts --include='*.rs' --include='*.py' | awk -F: '{print $1}' | sort -u | awk '!/scripts\/tests\//' | awk 'END{print NR}'
}
run acceptance acceptance
run acceptance_py python3 - <<'EOF'
import os, re
def walk(root, exts):
    for d, ds, fs in os.walk(root):
        ds[:] = [x for x in ds if x not in ('target', 'node_modules', '.git')]
        for f in fs:
            if f.endswith(exts): yield os.path.join(d, f)
def text(p):
    with open(p, encoding='utf-8', errors='replace') as fh: return fh.read()
print("E4a.2", [sum(1 for p in walk(r, ('.rs',)) if 'rules_tables::' in text(p) and not p.startswith('src/rules_core/rules_tables/')) for r in ('src', 'crates', 'apps/desktop/src-tauri', 'tests')])
print("E4a.3 module_exists", os.path.exists('src/rules_core/rules_tables'))
print("E4a.3 package_lst_lines", sum(1 for p in walk('data/rules_tables', ('',)) for line in text(p).splitlines() if '.lst' in line))
print("E4a.4 rs_left", sum(1 for _ in walk('src/rules_core/rules_tables', ('.rs',))) if os.path.exists('src/rules_core/rules_tables') else 0)
pat = re.compile(r'"src/rules_core/rules_tables/|f"src/rules_core/rules_tables/')
hits = sorted({p for r in ('crates/codex-ingest/src/bin', 'scripts') for p in walk(r, ('.rs', '.py')) if pat.search(text(p)) and not p.startswith('scripts/tests/')})
print("E4a.4a generators", len(hits), hits)
EOF

run build cargo build --locked -j 8 -p codex-ingest --bin gen_feat_gap_tables --bin gen_equipment_gap_tables --bin ingest_spells --bin ingest_class_spell_levels_arg
run package_sha_before pkgsha

proof() {
  set -o pipefail
  echo "== gen_feat_gap_tables"; "$BIN/gen_feat_gap_tables" || return 1
  echo "== gen_equipment_gap_tables"; "$BIN/gen_equipment_gap_tables" || return 1
  echo "== ingest_spells"; "$BIN/ingest_spells" 2>&1 || return 1
  echo "== ingest_class_spell_levels_arg --verify --emit"; "$BIN/ingest_class_spell_levels_arg" --verify --emit || return 1
  for b in $MONSTER_BOOKS; do echo "== transcribe_monster_tables $b"; python3 scripts/transcribe_monster_tables.py "$b" 2>&1 || return 1; done
  for b in $COMPANION_BOOKS; do echo "== transcribe_companion_tables $b"; python3 scripts/transcribe_companion_tables.py "$b" 2>&1 || return 1; done
  echo "invocations: 4 bins + $(echo $MONSTER_BOOKS | awk '{print NF}') monster + $(echo $COMPANION_BOOKS | awk '{print NF}') companion"
}
run proof_rerun proof
run proof_status bash -c 'out=$(git status --porcelain --untracked-files=all data/rules_tables crates/codex-ingest/src/pcgen_import src/rules_core); echo "status_lines=$(printf "%s" "$out" | awk "NF{n++} END{print n+0}")"; printf "%s\n" "$out"; test -z "$out"'

# One plant per generator, each in a file E4a.4a's plants did not touch: the generator must
# restore the file byte-for-byte (git shows it clean again).
plant_one() { # <label> <file> <sed-expr> <restore cmd...>
  local label=$1 f=$2 expr=$3; shift 3
  local before; before=$(sha256sum "$f" | awk '{print $1}')
  sed -i "$expr" "$f"
  local planted; planted=$(sha256sum "$f" | awk '{print $1}')
  if [ "$before" = "$planted" ] || [ -z "$(git status --porcelain "$f")" ]; then echo "$label PLANT_DID_NOT_TAKE $f"; return 1; fi
  "$@" > "$SCRATCH/plant-$label.out" 2>&1 || { echo "$label generator failed"; tail -5 "$SCRATCH/plant-$label.out"; git checkout -- "$f"; return 1; }
  local after; after=$(sha256sum "$f" | awk '{print $1}')
  if [ -n "$(git status --porcelain "$f")" ]; then echo "$label NOT_RESTORED $f"; git checkout -- "$f"; return 1; fi
  echo "$label restored $f ${before:0:12} -> ${planted:0:12} -> ${after:0:12}"
}
gen_plants() {
  local ok=0
  plant_one G1-feat data/rules_tables/feat_gap_tables/ADVANCED_RACE_GUIDE_FEAT_GAP_ROWS.json '0,/"name": "/s//"name": "PLANTED /' "$BIN/gen_feat_gap_tables" || ok=1
  plant_one G2-equip data/rules_tables/equipment_gap_tables/ADVANCED_CLASS_GUIDE_GAP_ROWS.json '0,/"name": "/s//"name": "PLANTED /' "$BIN/gen_equipment_gap_tables" || ok=1
  plant_one G3-argcsl data/rules_tables/advanced_race_guide/class_spell_levels/ARG_CLASS_SPELL_LEVELS.json '0,/"Absorbing Inhalation"/s//"PLANTED Absorbing Inhalation"/' "$BIN/ingest_class_spell_levels_arg" --emit || ok=1
  plant_one G4-spells data/rules_tables/advanced_race_guide/spell_list/SPELL_LIST.json '0,/"key": "/s//"key": "PLANTED /' "$BIN/ingest_spells" || ok=1
  plant_one G5-monster data/rules_tables/bestiary_3/monster_data/MONSTERS.json '0,/"name": "/s//"name": "PLANTED /' python3 scripts/transcribe_monster_tables.py bestiary_3 || ok=1
  plant_one G6-companion data/rules_tables/bestiary_3/companion_data/COMPANIONS.json '0,/"name": "/s//"name": "PLANTED /' python3 scripts/transcribe_companion_tables.py bestiary_3 || ok=1
  return $ok
}
run gen_plants gen_plants
run package_check cargo run --locked -j 8 -q -p codex --bin rules_tables_package -- --check

GOLDEN=every_catalog_table_view_and_lookup_matches_its_golden_digest
run golden_clean cargo test --locked -j 8 --lib -- --test-threads=8 $GOLDEN rules_data_package
pf() { ( cd apps/desktop/src-tauri && rm -rf "$SCRATCH/$1" && PF_SEED_RENDER_OUT="$SCRATCH/$1" cargo test --locked -j 8 --bins pf_seed_render_hash -- --nocapture --test-threads=8 ) ; local rc=$?; cat "$SCRATCH/$1/sha256.txt"; return $rc; }
run pf_clean pf pf-clean
mkdir -p $L/pf-clean && cp "$SCRATCH"/pf-clean/sha256.txt $L/pf-clean/

# Mutation M1: one monster row name in a table no class reads -> the golden digest test must FAIL.
M1=data/rules_tables/bestiary/monster_data/MONSTERS.json
mut_monster() {
  sha256sum $M1; sed -i '0,/"name": "/s//"name": "PLANTED /' $M1; sha256sum $M1
  cargo test --locked -j 8 --lib -- --test-threads=8 $GOLDEN; local rc=$?
  git checkout -- $M1; sha256sum $M1; echo "golden_rc_under_mutation=$rc"; test $rc -ne 0
}
run mut_monster_golden mut_monster
# Mutation M2: Wizard level 5 fort_save 1 -> 6 (a field neither earlier plant used) -> Elowen's hash must move.
M2=data/rules_tables/crb/class_tables/class_tables.json
mut_wizard() {
  sed -n 1645,1650p $M2
  python3 - "$M2" <<'EOF' || return 1
import sys
p = sys.argv[1]; lines = open(p).read().split('\n')
assert lines[1644].strip() == '"class_id": "Wizard",' and lines[1645].strip() == '"level": 5,' and lines[1647].strip() == '"fort_save": 1,', lines[1643:1648]
lines[1647] = lines[1647].replace('1', '6'); open(p, 'w').write('\n'.join(lines))
EOF
  sed -n 1648p $M2
  pf pf-mut-wizard-fort; local rc=$?
  git checkout -- $M2; echo "pf_rc_under_mutation=$rc"
  diff "$SCRATCH/pf-clean/sha256.txt" "$SCRATCH/pf-mut-wizard-fort/sha256.txt"; local d=$?
  for f in "$SCRATCH"/pf-clean/*; do b=$(basename "$f"); [ "$b" = sha256.txt ] && continue; diff "$f" "$SCRATCH/pf-mut-wizard-fort/$b" && echo "same: $b"; done
  echo "hash_diff_rc=$d"; test $d -ne 0
}
run mut_wizard_fort_pf mut_wizard
mkdir -p $L/pf-mut-wizard-fort && cp "$SCRATCH"/pf-mut-wizard-fort/sha256.txt $L/pf-mut-wizard-fort/
# Mutation M3: the same row's fort_save made a string -> the loader round trip must FAIL.
mut_type() {
  sed -i '1648s/"fort_save": 1,/"fort_save": "one",/' $M2; sed -n 1648p $M2
  cargo test --locked -j 8 --lib -- --test-threads=8 every_table_round_trips_through_the_loader; local rc=$?
  git checkout -- $M2; echo "roundtrip_rc_under_mutation=$rc"; test $rc -ne 0
}
run mut_type_roundtrip mut_type
run package_sha_after bash -c "$(declare -f pkgsha); pkgsha; git status --porcelain --untracked-files=all data/rules_tables | awk 'END{print \"status_lines=\"NR}'"

run verify_stages bash scripts/verify.sh -j 8 --only tauri-resources-tracked --only rules-schema-check --only pi-sweep --only clippy
run epic_tests cargo test --locked -j 8 -p codex-ingest --test table_generators_write_the_data_package -- --test-threads=8
run python_tests python3 -m unittest scripts/tests/test_transcribe_monster_tables.py scripts/tests/test_transcribers_write_the_data_package.py
run converter_check cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --check
run residue python3 scripts/pcgen_residue_gate.py --check --closure
# Desktop: the whole suite (codex-ingest is its dev-dependency, so E4a.4a's crate change reaches its
# test build), with the SF seed-line and PF render dumps on.
( cd apps/desktop/src-tauri && rm -rf "$SCRATCH/seedlines" "$SCRATCH/pf-desk" && SF_SEED_LINES_OUT="$SCRATCH/seedlines" PF_SEED_RENDER_OUT="$SCRATCH/pf-desk" cargo test --locked -j 8 --no-fail-fast -- --test-threads=8; echo "exit=$?" ) > $L/desktop_full.log 2>&1
mkdir -p $L/seed_lines && cp "$SCRATCH"/seedlines/*.txt $L/seed_lines/ 2>/dev/null; cp "$SCRATCH"/pf-desk/sha256.txt $L/seed_lines/pf-sha256.txt 2>/dev/null
echo RUN_DONE
