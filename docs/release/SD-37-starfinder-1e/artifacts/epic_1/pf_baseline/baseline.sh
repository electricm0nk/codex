#!/bin/bash
# Pre-change baseline for SD-37 E1.1-E1.3 (untouched tree ab7e0b8139).
set -u
export PATH="$HOME/.cargo/bin:$PATH"
T=/home/ubuntu/workspace/worktrees/codex-sd37
S=/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/33627023-04e6-4757-9afa-d5c1463408f3/scratchpad/e1
PHASE=${1:-before}
cd $T
export CARGO_TARGET_DIR="/home/ubuntu/workspace/worktrees/cargo-target/sd37-$(basename "$(git rev-parse --show-toplevel)")"
mkdir -p "$CARGO_TARGET_DIR" && echo $$ > "$CARGO_TARGET_DIR/.reclaim-claim"
echo "== HEAD $(git rev-parse HEAD) phase=$PHASE"
echo "== E1.3 check (PF default)"; cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --check > $S/$PHASE.check.log 2>&1; echo "exit=$?"; tail -3 $S/$PHASE.check.log
echo "== E1.3 check --system pathfinder-1e"; cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --system pathfinder-1e --check > $S/$PHASE.check_system.log 2>&1; echo "exit=$?"; tail -3 $S/$PHASE.check_system.log
echo "== dump"; rm -rf $S/$PHASE.dump; cargo run --locked -j 8 -p codex-ingest --bin sheet_rule_convert -- --dump $S/$PHASE.dump > $S/$PHASE.dump.log 2>&1; echo "exit=$?"; tail -3 $S/$PHASE.dump.log
echo "== structural diff"; python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py $S/$PHASE.dump > $S/$PHASE.structural_diff.log 2>&1; echo "exit=$?"; tail -15 $S/$PHASE.structural_diff.log
echo "== residue gate"; python3 scripts/pcgen_residue_gate.py --check --closure > $S/$PHASE.residue.log 2>&1; echo "exit=$?"; tail -3 $S/$PHASE.residue.log
echo "== E1.1 lib test filter"; cargo test --locked -j 8 --lib game_system_root -- --test-threads=8 > $S/$PHASE.e11.log 2>&1; echo "exit=$?"; grep -E 'test result|running [0-9]+ test' $S/$PHASE.e11.log | tail -3
/tmp/claude-1000/-home-ubuntu-workspace-repos-codex/33627023-04e6-4757-9afa-d5c1463408f3/scratchpad/e1/probe.sh $PHASE
echo "== git status after"; git status --porcelain
echo DONE
