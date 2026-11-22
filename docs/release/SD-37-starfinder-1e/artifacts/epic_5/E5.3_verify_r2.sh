#!/usr/bin/env bash
# E5.3 attempt-2 one-pass verify (decisions.md §20 change; no converter change in this round).
# Run from the repo root with SCRATCH outside the repo. Not-run skeleton first; each log ends exit=<code>.
export PATH="$HOME/.cargo/bin:$PATH"
L=docs/release/SD-37-starfinder-1e/artifacts/epic_5/E5.3_logs/r2
SCRATCH=${SCRATCH:?}
STAGES="green root_sf_loadout residue pf_seed_render clippy verify_root verify_desktop"
mkdir -p $L
for s in $STAGES; do echo "not-run" > $L/$s.log; done
run() { local s=$1; shift; { "$@"; echo "exit=$?"; } > $L/$s.log 2>&1; }
rm -rf "$SCRATCH/seedlines"; mkdir -p "$SCRATCH/seedlines"
( cd apps/desktop/src-tauri && SF_SEED_LINES_OUT="$SCRATCH/seedlines" cargo test --locked -j 8 -- --test-threads=8 sf_sheet_print sf_adapter; echo "exit=$?" ) > $L/green.log 2>&1
mkdir -p $L/seed_lines && cp "$SCRATCH"/seedlines/*.txt $L/seed_lines/ 2>/dev/null
run root_sf_loadout cargo test --locked -j 8 --lib sf_ -- --test-threads=8
run residue python3 scripts/pcgen_residue_gate.py --check --closure
( cd apps/desktop/src-tauri && PF_SEED_RENDER_OUT="$SCRATCH/pfrender" cargo test --locked -j 8 --bins pf_seed_render_hash -- --nocapture --test-threads=8; echo "exit=$?" ) > $L/pf_seed_render.log 2>&1
run clippy bash scripts/verify.sh --only clippy
run verify_root cargo test --locked -j 8 --workspace --no-fail-fast -- --test-threads=8
( cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8; echo "exit=$?" ) > $L/verify_desktop.log 2>&1
echo RUN_DONE
