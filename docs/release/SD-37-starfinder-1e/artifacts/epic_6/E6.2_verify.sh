#!/usr/bin/env bash
# E6.2 one-pass verify, one cargo process at a time (workflow-instruction.md §6 steps 5 and 7).
# Not-run skeleton first; each stage's result lands in results.txt as exit=<code>.
set -u
T=/home/ubuntu/workspace/worktrees/codex-sd37
A=$T/docs/release/SD-37-starfinder-1e/artifacts/epic_6
L=$A/E6.2_logs
SCRATCH=${SCRATCH:?}
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=/home/ubuntu/workspace/worktrees/cargo-target/sd37-codex-sd37
SD=$T/docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py
R=$L/results.txt
# RESUME=1 keeps results.txt and re-runs only the stages not yet recorded exit=0 (the first run
# stopped after pf_structural_diff: its `{ ...; exit $e; }` block exited the script, fixed to a
# subshell).
STAGES="plants sf_check pf_check pf_structural_diff sf_corpus_check verify_stage residue ingest_sf_tests e33_mutations seed_terms desktop_full pf_hash_pair desktop_clippy root_clippy root_full frontend_test frontend_typecheck"
if [ "${RESUME:-0}" != 1 ]; then : > $R; for s in $STAGES; do echo "$s not-run" >> $R; done; fi
done_ok() { grep -q "^$1 exit=0$" $R; }
set_result() { python3 - "$R" "$1" "$2" <<'PY'
import sys
p,k,v=sys.argv[1:]
rows=[l.split(' ',1) for l in open(p).read().splitlines()]
open(p,'w').write('\n'.join(f"{a} {v if a==k else b}" for a,b in rows)+'\n')
PY
}
cd $T
done_ok plants || { python3 $A/E6.2_plants.py > $L/plants.log 2>&1; set_result plants "exit=$?"; }
done_ok sf_check || { cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --system starfinder-1e --check > $L/sf_check.log 2>&1; set_result sf_check "exit=$?"; }
done_ok pf_check || { cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --check > $L/pf_check.log 2>&1; set_result pf_check "exit=$?"; }
rm -rf "$SCRATCH/pfdump"
done_ok pf_structural_diff || { ( cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --dump "$SCRATCH/pfdump" && python3 $SD "$SCRATCH/pfdump" --baseline data/sheet_rules; e=$?; echo "diff -rq:"; diff -rq "$SCRATCH/pfdump" data/sheet_rules; echo "diff_exit=$?"; exit $e ) > $L/pf_structural_diff.log 2>&1; set_result pf_structural_diff "exit=$?"; }
done_ok sf_corpus_check || { cargo run --locked -j 8 -q -p codex-ingest --bin sf_corpus -- --check > $L/sf_corpus_check.log 2>&1; set_result sf_corpus_check "exit=$?"; }
done_ok verify_stage || { bash scripts/verify.sh --only sf-sheet-rules-check > $L/verify_stage.log 2>&1; set_result verify_stage "exit=$?"; }
done_ok residue || { python3 scripts/pcgen_residue_gate.py --check --closure > $L/residue.log 2>&1; set_result residue "exit=$?"; }
done_ok ingest_sf_tests || { cargo test --locked -j 8 -p codex-ingest --test sf_license_registry --test sf_core_proof --test sf_mapping_table --test sf_formula_system --test sf_drone_companion -- --test-threads=8 > $L/ingest_sf_tests.log 2>&1; set_result ingest_sf_tests "exit=$?"; }
done_ok e33_mutations || { python3 docs/release/SD-37-starfinder-1e/artifacts/epic_3/token-mapping/sf_mapping_mutations.py > $L/e33_mutations.log 2>&1; set_result e33_mutations "exit=$?"; }
done_ok seed_terms || { python3 docs/release/SD-37-starfinder-1e/artifacts/epic_3/E3.4_seed_terms.py > $L/seed_terms.log 2>&1; set_result seed_terms "exit=$?"; }
cd $T/apps/desktop/src-tauri
done_ok desktop_full || { cargo test --locked -j 8 --no-fail-fast -- --test-threads=8 > $L/verify_desktop.log 2>&1; set_result desktop_full "exit=$?"; }
mkdir -p $L/pf_render
done_ok pf_hash_pair || { PF_SEED_RENDER_OUT=$L/pf_render cargo test --locked -j 8 --bins pf_seed_render_hash -- --test-threads=8 --nocapture > $L/pf_seed_render.log 2>&1; set_result pf_hash_pair "exit=$?"; }
done_ok desktop_clippy || { cargo clippy --locked --tests -j 8 -- -D warnings > $L/clippy_desktop.log 2>&1; set_result desktop_clippy "exit=$?"; }
cd $T
done_ok root_clippy || { bash scripts/verify.sh --only clippy > $L/clippy_root.log 2>&1; set_result root_clippy "exit=$?"; }
done_ok root_full || { cargo test --locked -j 8 --workspace --no-fail-fast -- --test-threads=8 > $L/verify_root.log 2>&1; set_result root_full "exit=$?"; }
cd $T/apps/desktop
done_ok frontend_test || { npm test > $L/frontend_test.log 2>&1; set_result frontend_test "exit=$?"; }
done_ok frontend_typecheck || { npm run typecheck > $L/frontend_typecheck.log 2>&1; set_result frontend_typecheck "exit=$?"; }
echo DONE >> $R
