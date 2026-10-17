#!/usr/bin/env bash
# E5.MC: re-run E5.1-E5.5 (cargo + the app) and the two plants, one cargo process at a time.
#   bash run.sh <scratch dir>
# Results: <scratch>/results.txt (every step starts "not-run"); logs per step in <scratch>.
# A plant edits src/rules_core/cpr/improvement.rs, runs, and is reverted with `git checkout --`;
# the script refuses to start on a dirty src/ and checks src/ is clean again after each plant.
set -u
export PATH="$HOME/.cargo/bin:$PATH" CARGO_TARGET_DIR=/home/ubuntu/cargo-target/sd38-main RETRO_ACTOR=sd38-e5.mc RUN_DESKTOP_AGENT=sd38-e5.mc
T=/home/ubuntu/workspace/repos/codex/.worktrees/sd38-main
V=$1
F=src/rules_core/cpr/improvement.rs
ROWS=improve-cyberpunk-skill,improve-cyberpunk-role-switch,improve-cyberpunk-cyberware,improve-cyberpunk-therapy,improve-cyberpunk-reputation,improve-cyberpunk-reputation-lower-deed,improve-cyberpunk-skip-refused
mkdir -p "$V"; R=$V/results.txt
STEPS="cpr_improve pf_seed sf_seed ui_green p1_cargo p1_ui p2a_cargo p2b_cargo p2b_ui src_clean"
for s in $STEPS; do echo "$s not-run"; done > "$R"
mark() { sed -i "s/^$1 .*/$1 $2/" "$R"; }
step() { name=$1; shift; mark "$name" running; echo "== $name $(date -Is)" >> "$V/progress.log"
  ( "$@" ) > "$V/$name.log" 2>&1; rc=$?; mark "$name" "exit=$rc"; echo "== $name exit=$rc $(date -Is)" >> "$V/progress.log"; }
plant() { python3 - "$T/$F" "$1" "$2" <<'EOF'
import sys
path, old, new = sys.argv[1], sys.argv[2], sys.argv[3]
text = open(path).read()
assert text.count(old) == 1, f"plant site found {text.count(old)} times"
open(path, "w").write(text.replace(old, new))
EOF
}
revert() { git -C "$T" checkout -- "$F"; test -z "$(git -C "$T" status --porcelain -- src)"; }
cargo_improve() { cd "$T/apps/desktop/src-tauri" && cargo test --locked -j 8 --bins cpr_improve -- --test-threads=8; }
ui() { cd "$T/apps/desktop" && node scripts/ui-smoke/run.mjs --only "$1" --out "$2"; }

test -z "$(git -C "$T" status --porcelain -- src)" || { echo "src/ dirty before the run"; exit 2; }

step cpr_improve cargo_improve
step pf_seed bash -c "cd $T/apps/desktop/src-tauri && rm -rf $V/pf && PF_SEED_RENDER_OUT=$V/pf cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8"
step sf_seed bash -c "cd $T/apps/desktop/src-tauri && rm -rf $V/sf && mkdir -p $V/sf && SF_SEED_LINES_OUT=$V/sf cargo test --locked -j 8 --bins sf_seed_printed_features_equal_the_features_the_converted_records_grant -- --test-threads=8"
step ui_green ui "$ROWS" "$V/ui-green"

# P1: every IP price x 1.5 (the card's "cost x 10 -> 15")
plant 'Ok(self.costs[&table][usize::from(level - 1)])' 'Ok(self.costs[&table][usize::from(level - 1)] * 15 / 10)'
step p1_cargo cargo_improve
step p1_ui ui improve-cyberpunk-skill "$V/ui-p1"
revert || { mark src_clean "FAIL(after p1)"; exit 3; }

# P2a: a switch at Rank 3 accepted (the gate moved one Rank down)
plant 'if rank < SWITCH_RANK {' 'if rank < SWITCH_RANK - 1 {'
step p2a_cargo cargo_improve
revert || { mark src_clean "FAIL(after p2a)"; exit 3; }

# P2b: the switch gate removed (any Rank accepted)
plant 'if rank < SWITCH_RANK {' 'if rank < SWITCH_RANK && false {'
step p2b_cargo cargo_improve
step p2b_ui ui improve-cyberpunk-role-switch "$V/ui-p2b"
revert || { mark src_clean "FAIL(after p2b)"; exit 3; }
mark src_clean "exit=0"
echo "== done $(date -Is)" >> "$V/progress.log"
