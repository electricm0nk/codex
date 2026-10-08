#!/bin/bash
# SD-37 E1.4 serial gate run: before-tree render, structural diff + residue gate (after tree), desktop sweep.
set -u
export PATH="$HOME/.cargo/bin:$PATH"
unset CODEX_REPO_ROOT
S=/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/33627023-04e6-4757-9afa-d5c1463408f3/scratchpad/e14
T=/home/ubuntu/workspace/worktrees/codex-sd37
B=/home/ubuntu/workspace/worktrees/codex-sd37-e1.4-before
R=$S/results.txt
printf 'before-render: not-run\ndump: not-run\nstructural-diff: not-run\nmut1: not-run\nmut2: not-run\nresidue: not-run\ndesktop-sweep: not-run\n' > $R
set_r() { sed -i "s|^$1: .*|$1: $2|" $R; }
# 1. before tree render (own CARGO_TARGET_DIR, by basename)
cd $B
export CARGO_TARGET_DIR="/home/ubuntu/workspace/worktrees/cargo-target/sd37-$(basename "$(git rev-parse --show-toplevel)")"
mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"
echo "before HEAD $(git rev-parse HEAD) target=$CARGO_TARGET_DIR" > $S/before.render.log
( cd apps/desktop/src-tauri && PF_SEED_RENDER_OUT=$S/before.render cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture ) >> $S/before.render.log 2>&1; set_r before-render "exit=$?"
# 2. after tree: converter dump + structural diff + planted mutations + residue gate
cd $T
export CARGO_TARGET_DIR=/home/ubuntu/workspace/worktrees/cargo-target/sd37-codex-sd37
echo "after HEAD $(git rev-parse HEAD)" > $S/dump.log
rm -rf $S/after.dump
cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --dump $S/after.dump >> $S/dump.log 2>&1; set_r dump "exit=$?"
python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py $S/after.dump > $S/structural_diff.log 2>&1; set_r structural-diff "exit=$?"
rm -rf $S/mut1 $S/mut2; cp -r $S/after.dump $S/mut1; cp -r $S/after.dump $S/mut2
rm -f $S/mut1/core_rulebook/feat/acrobatic.json
python3 - "$S/mut2/core_rulebook/feat/acrobatic.json" <<'PY' > $S/mut2.edit.log 2>&1
import json,sys,re
p=sys.argv[1]; s=open(p).read()
# bump the first numeric Const value by one (planted rule delta)
m=re.search(r'("Const"\s*:\s*)(-?\d+)', s)
assert m, "no Const in acrobatic.json"
s2=s[:m.start(2)]+str(int(m.group(2))+1)+s[m.end(2):]
open(p,'w').write(s2); print("mutated Const", m.group(2), "->", int(m.group(2))+1)
PY
python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py $S/mut1 > $S/mut1.log 2>&1; set_r mut1 "exit=$?"
python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py $S/mut2 > $S/mut2.log 2>&1; set_r mut2 "exit=$?"
python3 scripts/pcgen_residue_gate.py --check --closure > $S/residue.log 2>&1; set_r residue "exit=$?"
# 3. desktop crate full sweep (the only workspace this change reaches)
( cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8 ) > $S/desktop.sweep.log 2>&1; set_r desktop-sweep "exit=$?"
echo DONE >> $R
