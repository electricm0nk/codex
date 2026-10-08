#!/usr/bin/env bash
# E6.3 one-pass verify, one cargo process at a time (workflow-instruction.md §6 step 7; decisions.md §21 rule 2).
# Not-run skeleton first; each stage's result lands in results.txt as exit=<code>.
set -u
T=/home/ubuntu/workspace/worktrees/codex-sd37
A=$T/docs/release/SD-37-starfinder-1e/artifacts/epic_6
L=$A/E6.3_logs
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=/home/ubuntu/workspace/worktrees/cargo-target/sd37-codex-sd37
R=$L/results.txt
STAGES="desktop_full pf_hash_pair desktop_clippy frontend_test frontend_typecheck"
: > $R; for s in $STAGES; do echo "$s not-run" >> $R; done
set_result() { python3 - "$R" "$1" "$2" <<'PY'
import sys
p,k,v=sys.argv[1:]
rows=[l.split(' ',1) for l in open(p).read().splitlines()]
open(p,'w').write('\n'.join(f"{a} {v if a==k else b}" for a,b in rows)+'\n')
PY
}
cd $T/apps/desktop/src-tauri
cargo test --locked -j 8 --no-fail-fast -- --test-threads=8 > $L/verify_desktop.log 2>&1; set_result desktop_full "exit=$?"
mkdir -p $L/pf_render
PF_SEED_RENDER_OUT=$L/pf_render cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture > $L/pf_seed_render.log 2>&1; set_result pf_hash_pair "exit=$?"
cargo clippy --locked --tests -j 8 -- -D warnings > $L/clippy_desktop.log 2>&1; set_result desktop_clippy "exit=$?"
cd $T/apps/desktop
npm test > $L/frontend_test.log 2>&1; set_result frontend_test "exit=$?"
npm run typecheck > $L/frontend_typecheck.log 2>&1; set_result frontend_typecheck "exit=$?"
echo DONE >> $R
