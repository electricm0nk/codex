#!/usr/bin/env bash
# E4.3 one-pass verify (repo root). Not-run skeleton first; each stage log ends with exit=<code>.
# No converter change in E4.3, so no structural diff / regeneration stage.
export PATH="$HOME/.cargo/bin:$PATH"
L=docs/release/SD-37-starfinder-1e/artifacts/epic_4/E4.3_logs
STAGES="green sf_check class_module_check pf_seed_render verify_root verify_desktop"
for s in $STAGES; do echo "not-run" > $L/$s.log; done
run() { local s=$1; shift; { "$@"; echo "exit=$?"; } > $L/$s.log 2>&1; }
run green cargo test --locked -j 8 --lib sf_seed -- --test-threads=8
run sf_check cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --system starfinder-1e --check
{ ls src/rules_core/pilot_compute | awk 'tolower($0) ~ /soldier|mystic|envoy|technomancer|operative|mechanic|solarian|biohacker|vanguard|witchwarper/' | awk 'END{print NR}'; echo "exit=0"; } > $L/class_module_check.log 2>&1
( cd apps/desktop/src-tauri && cargo test --locked -j 8 --bins pf_seed_render_hash -- --nocapture --test-threads=8; echo "exit=$?" ) > $L/pf_seed_render.log 2>&1
run verify_root cargo test --locked -j 8 --workspace --no-fail-fast -- --test-threads=8
( cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8; echo "exit=$?" ) > $L/verify_desktop.log 2>&1
echo RUN_DONE
