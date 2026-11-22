#!/usr/bin/env bash
# E6.1 widest-scope verify, one cargo process at a time (workflow-instruction.md §6 step 7).
set -u
T=/home/ubuntu/workspace/worktrees/codex-sd37
L=$T/docs/release/SD-37-starfinder-1e/artifacts/epic_6/E6.1_logs
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=/home/ubuntu/workspace/worktrees/cargo-target/sd37-codex-sd37
R=$L/results.txt
printf 'desktop-full not-run\npf-hash-pair not-run\ndesktop-clippy not-run\nroot-full not-run\n' > $R
set_result() { python3 - "$R" "$1" "$2" <<'PY'
import sys
p,k,v=sys.argv[1:]
rows=[l.split(' ',1) for l in open(p).read().splitlines()]
open(p,'w').write('\n'.join(f"{a} {v if a==k else b}" for a,b in rows)+'\n')
PY
}
cd $T/apps/desktop/src-tauri
cargo test --locked -j 8 --no-fail-fast -- --test-threads=8 > $L/verify_desktop.log 2>&1; set_result desktop-full "exit=$?"
mkdir -p $L/pf_render
PF_SEED_RENDER_OUT=$L/pf_render cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture > $L/pf_seed_render.log 2>&1; set_result pf-hash-pair "exit=$?"
cargo clippy --locked --tests -j 8 -- -D warnings > $L/clippy_desktop.log 2>&1; set_result desktop-clippy "exit=$?"
cd $T
cargo test --locked -j 8 --no-fail-fast -- --test-threads=8 > $L/verify_root.log 2>&1; set_result root-full "exit=$?"
echo DONE >> $R
