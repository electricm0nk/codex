#!/usr/bin/env bash
# SD-37 E6.MC one-pass adversarial re-run on origin/tranche/17 (+ the E6.MC increase-count fix).
# Not-run skeleton first; every stage log ends with exit=<code>; results.txt collects them.
# One cargo process at a time. SCRATCH = a dir outside the repo. Stages run in order:
#   frontend -> tauri resources -> HEAD desktop renders -> card plants -> MC plants -> PF plant ->
#   clippy -> root full -> desktop full -> BEFORE tree renders -> ui-smoke full spec -> smoke plant.
set -u
T=/home/ubuntu/workspace/worktrees/codex-sd37
A=$T/docs/release/SD-37-starfinder-1e/artifacts/epic_6
L=$A/E6.MC_logs
SCRATCH=${SCRATCH:?}
BASE=78f4f650ea            # E5.MC's last commit = the tree before E6.1 (cc38b39ce0)
BT=/home/ubuntu/workspace/worktrees/codex-sd37-e6mc-before
BTD=/home/ubuntu/workspace/worktrees/cargo-target/sd37-codex-sd37-e6mc-before
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=/home/ubuntu/workspace/worktrees/cargo-target/sd37-codex-sd37
STAGES="frontend_test frontend_typecheck frontend_build tauri_resources desktop_e6 pf_hash_after e62_plants e63_plants e64_plants e65_plants e65a_plants mc_plants pf_plant clippy root_full desktop_full before_tree pf_hash_before sf_before smoke_full smoke_plant"
mkdir -p $L
: > $L/results.txt
for s in $STAGES; do echo "not-run" > $L/$s.log; echo "$s not-run" >> $L/results.txt; done
mark() { sed -i "s/^$1 .*/$1 exit=$2/" $L/results.txt; }
run() { local s=$1; shift; ( "$@" ) > $L/$s.log 2>&1; local rc=$?; echo "exit=$rc" >> $L/$s.log; mark $s $rc; }

cd $T
# 1. Frontend (E6.1-E6.5a frontend tests incl. E6.3's no-hand-kept-table test), types, real build.
run frontend_test bash -c "cd apps/desktop && npm test"
run frontend_typecheck bash -c "cd apps/desktop && npm run typecheck"
run frontend_build bash -c "cd apps/desktop && npm run build"
# 2. E6.1: the SF root is bundled.
run tauri_resources bash scripts/verify.sh --only tauri-resources-tracked
# 3. HEAD desktop: every E6 Rust test + E4.6's 160-of-160 + E5 print tests, writing the SF seed lines.
rm -rf $SCRATCH/after_lines; mkdir -p $SCRATCH/after_lines
run desktop_e6 bash -c "cd apps/desktop/src-tauri && SF_SEED_LINES_OUT=$SCRATCH/after_lines cargo test --locked -j 8 --bin codex-desktop -- --test-threads=8 sf_ starfinder"
mkdir -p $L/seed_lines_after && cp $SCRATCH/after_lines/*.txt $L/seed_lines_after/ 2>/dev/null
rm -rf $SCRATCH/pf_after; mkdir -p $SCRATCH/pf_after
run pf_hash_after bash -c "cd apps/desktop/src-tauri && PF_SEED_RENDER_OUT=$SCRATCH/pf_after cargo test --locked -j 8 --bins pf_seed_render_hash -- --nocapture --test-threads=8 && cat $SCRATCH/pf_after/sha256.txt"
# 4. The cards' own plants, re-run on this tree.
run e62_plants python3 $A/E6.2_plants.py
run e63_plants python3 $A/E6.3_plants.py
run e64_plants python3 $A/E6.4_plants.py
run e65_plants python3 $A/E6.5_plants.py
run e65a_plants python3 $A/E6.5a_plants.py
# 5. E6.MC's own plants.
run mc_plants python3 $A/E6.MC_plants.py
# 6. PF plant: the hash pair must move (core_rulebook dropped from the race registry; E5.MC's shape).
RC=apps/desktop/src-tauri/src/race_catalog.rs
cp $RC $SCRATCH/race_catalog.rs.orig
pf_plant() {
  python3 - "$RC" <<'PY'
import sys
p = sys.argv[1]; s = open(p).read()
old = "BookRegistry::pathfinder_only(RACE_CORPUS_BOOKS);"
assert s.count(old) == 1
open(p, "w").write(s.replace(old, "BookRegistry::pathfinder_only(RACE_CORPUS_BOOKS.split_at(1).1);"))
print("planted: core_rulebook dropped from RACE_CORPUS_BOOK_REGISTRY")
PY
  rm -rf $SCRATCH/pf_plant; mkdir -p $SCRATCH/pf_plant
  ( cd apps/desktop/src-tauri && PF_SEED_RENDER_OUT=$SCRATCH/pf_plant cargo test --locked -j 8 --bins pf_seed_render_hash -- --nocapture --test-threads=8 ); echo "render_exit=$?"
  cp $SCRATCH/race_catalog.rs.orig $RC; cmp $SCRATCH/race_catalog.rs.orig $RC && echo RESTORED
  echo "planted:"; cat $SCRATCH/pf_plant/sha256.txt; echo "baseline:"; cat $SCRATCH/pf_after/sha256.txt
  ! cmp -s $SCRATCH/pf_plant/sha256.txt $SCRATCH/pf_after/sha256.txt
}
run pf_plant pf_plant
# 7. Widest scope, once: clippy, root workspace, desktop workspace.
run clippy bash scripts/verify.sh --only clippy
run root_full cargo test --locked -j 8 --workspace --no-fail-fast -- --test-threads=8
run desktop_full bash -c "cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8"
# 8. The BEFORE tree (pre-E6.1), its own target dir; the same renders.
before_tree() {
  git -C $T worktree add --detach $BT $BASE && mkdir -p $BTD && git -C $BT rev-parse HEAD
}
run before_tree before_tree
rm -rf $SCRATCH/pf_before $SCRATCH/before_lines; mkdir -p $SCRATCH/pf_before $SCRATCH/before_lines
run pf_hash_before bash -c "cd $BT/apps/desktop/src-tauri && CARGO_TARGET_DIR=$BTD PF_SEED_RENDER_OUT=$SCRATCH/pf_before cargo test --locked -j 8 --bins pf_seed_render_hash -- --nocapture --test-threads=8 && cat $SCRATCH/pf_before/sha256.txt"
run sf_before bash -c "cd $BT/apps/desktop/src-tauri && CARGO_TARGET_DIR=$BTD SF_SEED_LINES_OUT=$SCRATCH/before_lines cargo test --locked -j 8 --bin codex-desktop -- --test-threads=8 sf_ starfinder"
mkdir -p $L/seed_lines_before && cp $SCRATCH/before_lines/*.txt $L/seed_lines_before/ 2>/dev/null
# 9. The real app: the full ui-smoke spec (every E6 row, the six seeds, every Pathfinder row), isolated.
run smoke_full bash -c "cd apps/desktop && RUN_DESKTOP_AGENT=sd37-e6-mc-full node scripts/ui-smoke/run.mjs --out $SCRATCH/smoke_full"
( cd apps/desktop && .claude/skills/run-desktop/driver.sh stop >/dev/null 2>&1 || true )
# 10. Smoke plant: the adapter serves KAC as EAC; the real app must show it and the soldier row go red.
AD=apps/desktop/src-tauri/src/sf_adapter.rs
cp $AD $SCRATCH/sf_adapter.rs.orig
smoke_plant() {
  python3 - "$AD" <<'PY'
import sys
p = sys.argv[1]; s = open(p).read()
old = '("sf.eac", &self.defense.eac),'
assert s.count(old) == 1
open(p, "w").write(s.replace(old, '("sf.eac", &self.defense.kac),'))
print("planted: sf.eac row serves the KAC total")
PY
  ( cd apps/desktop && RUN_DESKTOP_AGENT=sd37-e6-mc-plant node scripts/ui-smoke/run.mjs --only open-starfinder-soldier --out $SCRATCH/smoke_plant ); local rc=$?
  ( cd apps/desktop && .claude/skills/run-desktop/driver.sh stop >/dev/null 2>&1 || true )
  cp $SCRATCH/sf_adapter.rs.orig $AD; cmp $SCRATCH/sf_adapter.rs.orig $AD && echo RESTORED
  echo "smoke_exit=$rc"; [ $rc -ne 0 ]
}
run smoke_plant smoke_plant
git -C $T status --porcelain > $L/status_after_run.txt
echo RUN_DONE >> $L/results.txt
