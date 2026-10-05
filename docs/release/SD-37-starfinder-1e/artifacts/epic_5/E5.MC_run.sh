#!/usr/bin/env bash
# SD-37 E5.MC one-pass adversarial re-run of E5.1-E5.4's commands on origin/tranche/17 (repo root of
# the tranche/17 tree; SCRATCH outside the repo; E5.MC_chain.sh run first, its snapshots are reused).
# Not-run skeleton first; each stage log ends with exit=<code>. One cargo process at a time.
export PATH="$HOME/.cargo/bin:$PATH"
A=docs/release/SD-37-starfinder-1e/artifacts/epic_5
L=$A/E5.MC_logs
SCRATCH=${SCRATCH:?}
SD=docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py
P=data/starfinder-1e/sheet_rules
eval "$(scripts/fetch-pcgen-oracle.sh --check --quiet)"
STAGES="desktop_sf fixture_check e4mc_fixture_review ingest_unit input_unit ingest_drone ingest_mapping_unit e2_roundtrip m1_m4 root_sf sf_check pf_check sf_corpus_check verify_stage residue ingest_sf_tests seed_terms pf_dump pf_structural_diff pf_seed_render pf_registry_plant mc_plants e51_plants e52_plants e53_plants e54_plants clippy verify_root verify_desktop"
mkdir -p $L
for s in $STAGES; do echo "not-run" > $L/$s.log; done
run() { local s=$1; shift; { "$@"; echo "exit=$?"; } > $L/$s.log 2>&1; }
desk() { ( cd apps/desktop/src-tauri && "$@" ); }

# 1. Desktop: E5.1-E5.4 criterion tests + E4.MC's 160-of-160 SF hand values; the seed lines.
rm -rf "$SCRATCH/seedlines"; mkdir -p "$SCRATCH/seedlines"
{ desk env SF_SEED_LINES_OUT="$SCRATCH/seedlines" cargo test --locked -j 8 -- --test-threads=8 sf_sheet_print sf_adapter sf_drone_print race_books_load_through sf_seed_every_hand_value starfinder_1e_resolves; echo "exit=$?"; } > $L/desktop_sf.log 2>&1
mkdir -p $L/seed_lines && cp "$SCRATCH"/seedlines/*.txt $L/seed_lines/ 2>/dev/null
run fixture_check python3 $A/E5.1_seed_features.py . --check
run e4mc_fixture_review python3 docs/release/SD-37-starfinder-1e/artifacts/epic_4/E4.MC_fixture_review.py
# 2. E5.2 / E5.4 unit and package suites; E4.MC's root seed fixtures (sf_seed, sf_loadout under sf_).
run ingest_unit cargo test --locked -j 8 -p codex-ingest --lib sf_text_repair -- --test-threads=8
run input_unit cargo test --locked -j 8 --lib spell_selection_rule_id -- --test-threads=8
run ingest_drone cargo test --locked -j 8 -p codex-ingest --test sf_drone_companion -- --test-threads=8
run ingest_mapping_unit cargo test --locked -j 8 -p codex-ingest --lib sf_mapping -- --test-threads=8
run e2_roundtrip cargo test --locked -j 8 --lib sheet_rule -- --test-threads=8
run m1_m4 python3 docs/release/SD-37-starfinder-1e/artifacts/epic_3/token-mapping/sf_mapping_mutations.py
run root_sf cargo test --locked -j 8 --lib sf_ -- --test-threads=8
# 3. Package / converter gates.
run sf_check cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --system starfinder-1e --check
run pf_check cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --check
run sf_corpus_check cargo run --locked -j 8 -q -p codex-ingest --bin sf_corpus -- --check
run verify_stage bash scripts/verify.sh --only sf-sheet-rules-check
run residue python3 scripts/pcgen_residue_gate.py --check --closure
run ingest_sf_tests cargo test --locked -j 8 -p codex-ingest --test sf_license_registry --test sf_core_proof --test sf_mapping_table --test sf_formula_system -- --test-threads=8
run seed_terms python3 docs/release/SD-37-starfinder-1e/artifacts/epic_3/E3.4_seed_terms.py
rm -rf "$SCRATCH/pfdump"
run pf_dump cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --dump "$SCRATCH/pfdump"
{ python3 $SD "$SCRATCH/pfdump" --baseline data/sheet_rules; echo "exit=$?"; echo "diff -rq:"; diff -rq "$SCRATCH/pfdump" data/sheet_rules; echo "diff_exit=$?"; } > $L/pf_structural_diff.log 2>&1
# 4. PF hash pair (E1.4 harness) and E5.1's registry plant.
rm -rf "$SCRATCH/pfrender"; mkdir -p "$SCRATCH/pfrender"
{ desk env PF_SEED_RENDER_OUT="$SCRATCH/pfrender" cargo test --locked -j 8 --bins pf_seed_render_hash -- --nocapture --test-threads=8; echo "exit=$?"; cat "$SCRATCH/pfrender/sha256.txt"; } > $L/pf_seed_render.log 2>&1
RC=apps/desktop/src-tauri/src/race_catalog.rs
cp $RC "$SCRATCH/race_catalog.rs.orig"
{
  python3 - "$RC" <<'PY'
import sys
p = sys.argv[1]; s = open(p).read()
old = "BookRegistry::pathfinder_only(RACE_CORPUS_BOOKS);"
assert s.count(old) == 1
open(p, "w").write(s.replace(old, "BookRegistry::pathfinder_only(RACE_CORPUS_BOOKS.split_at(1).1); // planted"))
print("planted: core_rulebook dropped from RACE_CORPUS_BOOK_REGISTRY only")
PY
  rm -rf "$SCRATCH/pfrender_plant"; mkdir -p "$SCRATCH/pfrender_plant"
  desk env PF_SEED_RENDER_OUT="$SCRATCH/pfrender_plant" cargo test --locked -j 8 --bins pf_seed_render_hash -- --nocapture --test-threads=8; echo "render_exit=$?"
  cat "$SCRATCH/pfrender_plant/sha256.txt"
  cp "$SCRATCH/race_catalog.rs.orig" $RC
  cmp "$SCRATCH/race_catalog.rs.orig" $RC && echo RESTORED
  echo "exit=0"
} > $L/pf_registry_plant.log 2>&1
# 5. Plants: E5.MC's criterion plants, then each card's own plant script (delta-classifier plants
#    in E5.1/E5.2/E5.3's scripts compare against that card's base, now stale; their classifiers are
#    re-planted on their own link by E5.MC_chain.sh).
run mc_plants python3 $A/E5.MC_plants.py
run e51_plants python3 $A/E5.1_plants.py "$SCRATCH/chain/b9797f1392/$P" "$PCGEN_REPO_DIR/data"
run e52_plants python3 $A/E5.2_plants.py "$SCRATCH/chain/a94024c402/$P"
run e53_plants python3 $A/E5.3_plants.py "$SCRATCH/chain/e49a1669d1/$P" "$PCGEN_REPO_DIR/data"
run e54_plants env E54_BASE_SHEET_RULES="$SCRATCH/chain/4c50ff7752/$P" python3 $A/E5.4_plants.py --print
git status --porcelain > $L/status_after_plants.txt
# 6. Clippy, then the widest verify, once.
run clippy bash scripts/verify.sh --only clippy
run verify_root cargo test --locked -j 8 --workspace --no-fail-fast -- --test-threads=8
{ desk cargo test --locked -j 8 --no-fail-fast -- --test-threads=8; echo "exit=$?"; } > $L/verify_desktop.log 2>&1
echo RUN_DONE
