#!/bin/bash
set -u
export PATH="$HOME/.cargo/bin:$PATH"; unset CODEX_REPO_ROOT
S=/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/33627023-04e6-4757-9afa-d5c1463408f3/scratchpad/e1mc
cd /home/ubuntu/workspace/worktrees/codex-sd37
export CARGO_TARGET_DIR=/home/ubuntu/workspace/worktrees/cargo-target/sd37-codex-sd37
F=apps/desktop/src-tauri/src/race_catalog.rs
python3 - "$F" <<'PY' > $S/mutC.edit.log 2>&1
import sys; p=sys.argv[1]; s=open(p).read()
old='pub(crate) const RACE_CORPUS_BOOKS: &[&str] = &[\n    "core_rulebook",\n'
new='pub(crate) const RACE_CORPUS_BOOKS: &[&str] = &[\n'
assert s.count(old)==1, "mutation C site not found exactly once"
open(p,'w').write(s.replace(old,new)); print("mutC planted: core_rulebook removed from the PF entry of RACE_CORPUS_BOOK_REGISTRY (the RACE_CORPUS_BOOKS slice)")
PY
git diff -- $F >> $S/mutC.edit.log
rm -rf $S/mutC-render
( cd apps/desktop/src-tauri && PF_SEED_RENDER_OUT=$S/mutC-render cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture ) > $S/mutC-render.log 2>&1; echo "mutC-render exit=$?" > $S/mutC.results
git checkout -- $F; git diff --quiet -- $F; echo "mutC-restored diff-exit=$?" >> $S/mutC.results
rm -rf $S/post-mutC-render
( cd apps/desktop/src-tauri && PF_SEED_RENDER_OUT=$S/post-mutC-render cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture ) > $S/post-mutC-render.log 2>&1; echo "post-mutC-render exit=$?" >> $S/mutC.results
