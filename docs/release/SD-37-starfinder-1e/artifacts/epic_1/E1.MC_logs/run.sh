#!/bin/bash
# SD-37 E1.MC adversarial merge check: one serial run on origin/tranche/17 (one cargo process at a time).
set -u
export PATH="$HOME/.cargo/bin:$PATH"
export RETRO_ACTOR=sd37-e1-mc
unset CODEX_REPO_ROOT
S=/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/33627023-04e6-4757-9afa-d5c1463408f3/scratchpad/e1mc
T=/home/ubuntu/workspace/worktrees/codex-sd37
B=/home/ubuntu/workspace/worktrees/codex-sd37-e1mc-before
R=$S/results.txt
STAGES="after-render mutA-render mutA-restored mutB-render mutB-restored post-restore-render before-render e11 e12-count e12-unregistered e13-check e13-system-check dump structural-diff mut1 mut2 residue desktop-sweep root-sweep"
: > $R; for s in $STAGES; do echo "$s: not-run" >> $R; done
set_r() { sed -i "s|^$1: .*|$1: $2|" $R; }
render() { # $1 = stage name (output dir + log under $S)
  ( cd apps/desktop/src-tauri && PF_SEED_RENDER_OUT=$S/$1 cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture ) > $S/$1.log 2>&1
}

cd $T
export CARGO_TARGET_DIR="/home/ubuntu/workspace/worktrees/cargo-target/sd37-$(basename "$(git rev-parse --show-toplevel)")"
mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"
echo "HEAD $(git rev-parse HEAD)" > $S/head.txt

# 1. after-tree render (merged tree)
rm -rf $S/after-render; render after-render; set_r after-render "exit=$?"

# 2. Mutation A: core_rulebook dropped from the PF race-corpus registry (runtime reader race_corpus())
F=apps/desktop/src-tauri/src/race_catalog.rs
python3 - "$F" <<'PY' > $S/mutA.edit.log 2>&1
import sys; p=sys.argv[1]; s=open(p).read()
old='BookRegistry::pathfinder_only(RACE_CORPUS_BOOKS);'
new='BookRegistry::pathfinder_only(RACE_CORPUS_BOOKS.split_at(1).1); // E1.MC planted: core_rulebook dropped'
assert s.count(old)==1, "mutation A site not found exactly once"
open(p,'w').write(s.replace(old,new)); print("mutA planted:", new)
PY
git diff -- $F >> $S/mutA.edit.log
rm -rf $S/mutA-render; render mutA-render; set_r mutA-render "exit=$?"
git checkout -- $F; git diff --quiet -- $F; set_r mutA-restored "diff-exit=$?"

# 3. Mutation B: core_rulebook + advanced_players_guide dropped from the engine class-family registry
F=src/rules_core/pilot_compute/generic_class_chassis.rs
python3 - "$F" <<'PY' > $S/mutB.edit.log 2>&1
import sys; p=sys.argv[1]; s=open(p).read()
old='BookRegistry::pathfinder_only(&CLASS_FAMILY_BOOKS);'
new='BookRegistry::pathfinder_only(CLASS_FAMILY_BOOKS.split_at(14).0); // E1.MC planted: CRB+APG dropped'
assert s.count(old)==1, "mutation B site not found exactly once"
open(p,'w').write(s.replace(old,new)); print("mutB planted:", new)
PY
git diff -- $F >> $S/mutB.edit.log
rm -rf $S/mutB-render; render mutB-render; set_r mutB-render "exit=$?"
git checkout -- $F; git diff --quiet -- $F; set_r mutB-restored "diff-exit=$?"

# 4. render again on the restored tree (must equal step 1)
rm -rf $S/post-restore-render; render post-restore-render; set_r post-restore-render "exit=$?"

# 5. before tree (209664dce2 = parent of the E1.1-E1.3 implementation commit), own target dir
git worktree add --detach $B 209664dce2 > $S/before-worktree.log 2>&1
cp apps/desktop/src-tauri/src/pf_seed_render_hash.rs $B/apps/desktop/src-tauri/src/
tail -7 apps/desktop/src-tauri/src/character_hub.rs > $S/mod-decl.txt
( cd $B && printf '\n' >> apps/desktop/src-tauri/src/character_hub.rs && cat $S/mod-decl.txt >> apps/desktop/src-tauri/src/character_hub.rs )
(
  cd $B
  export CARGO_TARGET_DIR="/home/ubuntu/workspace/worktrees/cargo-target/sd37-$(basename "$(git rev-parse --show-toplevel)")"
  mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"
  echo "before HEAD $(git rev-parse HEAD) target=$CARGO_TARGET_DIR" > $S/before-render.hdr
  rm -rf $S/before-render; render before-render
); set_r before-render "exit=$?"

# 6. E1.1 / E1.2 / E1.3 acceptance commands on the merged tree
cargo test --locked -j 8 --lib game_system_root -- --test-threads=8 > $S/e11.log 2>&1; set_r e11 "exit=$?"
grep -rnE 'const [A-Z_]*BOOKS[A-Z_]*\s*:' src apps/desktop/src-tauri/src crates/codex-ingest/src --include='*.rs' | awk 'END{print NR}' > $S/e12.count.log 2>&1; set_r e12-count "out=$(cat $S/e12.count.log)"
python3 - > $S/e12.count.py.log 2>&1 <<'PY'
import os,re
pat=re.compile(r'const [A-Z_]*BOOKS[A-Z_]*\s*:'); n=0; files=set()
for root in ["src","apps/desktop/src-tauri/src","crates/codex-ingest/src"]:
    for d,_,fs in os.walk(root):
        for f in fs:
            if f.endswith(".rs"):
                p=os.path.join(d,f)
                for line in open(p,errors="replace"):
                    if pat.search(line): n+=1; files.add(p)
print(n, len(files))
PY
( for f in $(grep -rlE 'const [A-Z_]*BOOKS[A-Z_]*\s*:' src apps/desktop/src-tauri/src crates/codex-ingest/src --include='*.rs'); do grep -q 'GameSystem' "$f" || echo "UNREGISTERED $f"; done ) > $S/e12.unregistered.log 2>&1
set_r e12-unregistered "lines=$(awk 'END{print NR}' $S/e12.unregistered.log)"
cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --check > $S/e13.check.log 2>&1; set_r e13-check "exit=$?"
cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --system pathfinder-1e --check > $S/e13.system.log 2>&1; set_r e13-system-check "exit=$?"

# 7. E1.4 structural diff + independent planted mutations + residue gate
rm -rf $S/after.dump
cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --dump $S/after.dump > $S/dump.log 2>&1; set_r dump "exit=$?"
SD=docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py
python3 $SD $S/after.dump > $S/structural_diff.log 2>&1; set_r structural-diff "exit=$?"
rm -rf $S/mut1 $S/mut2; cp -r $S/after.dump $S/mut1; cp -r $S/after.dump $S/mut2
# m1: delete a class record (not the feat file E1.4 used)
M1=$(ls $S/mut1/core_rulebook/class/*.json 2>/dev/null | sort | head -1); echo "m1 deleted: $M1" > $S/mut1.edit.log; rm -f "$M1"
# m2: bump the LAST integer Const in a core_rulebook class record (not E1.4's first-Const-in-acrobatic)
python3 - "$S/mut2" <<'PY' >> $S/mut2.edit.log 2>&1
import sys,os,re,glob
fs=sorted(glob.glob(os.path.join(sys.argv[1],"core_rulebook","class","*.json")))
for p in fs:
    s=open(p).read(); ms=list(re.finditer(r'("Const"\s*:\s*)(-?\d+)', s))
    if ms:
        m=ms[-1]; s2=s[:m.start(2)]+str(int(m.group(2))+1)+s[m.end(2):]
        open(p,'w').write(s2); print("m2 mutated", p, m.group(2), "->", int(m.group(2))+1); break
else: print("m2: no Const found"); sys.exit(3)
PY
python3 $SD $S/mut1 > $S/mut1.log 2>&1; set_r mut1 "exit=$?"
python3 $SD $S/mut2 > $S/mut2.log 2>&1; set_r mut2 "exit=$?"
python3 scripts/pcgen_residue_gate.py --check --closure > $S/residue.log 2>&1; set_r residue "exit=$?"

# 8. widest scope: desktop crate, then root workspace (serial)
( cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8 ) > $S/desktop.sweep.log 2>&1; set_r desktop-sweep "exit=$?"
cargo test --locked -j 8 --workspace --no-fail-fast -- --test-threads=8 > $S/root.sweep.log 2>&1; set_r root-sweep "exit=$?"
echo DONE >> $R
