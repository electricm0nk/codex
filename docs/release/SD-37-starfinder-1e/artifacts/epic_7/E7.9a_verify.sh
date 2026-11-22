#!/usr/bin/env bash
# SD-37 E7.9a: one widest-scope verify pass over the merge of origin/develop (fd68740f60) into
# tranche/17. Not-run skeleton first; every stage log ends with exit=<code>; results.txt collects
# them. One cargo process at a time. SCRATCH = a dir outside the repo.
set -u
T=/home/ubuntu/workspace/worktrees/codex-sd37
L=$T/docs/release/SD-37-starfinder-1e/artifacts/epic_7/E7.9a_logs
SCRATCH=${SCRATCH:?}
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR=/home/ubuntu/workspace/worktrees/cargo-target/sd37-codex-sd37
STAGES="frontend_test frontend_typecheck release_pytest root_full desktop_full pf_hash sf_seed_root sf_seed_desktop verify_sh store_before smoke_sf store_after"
mkdir -p "$L" "$SCRATCH"
: PY=${PY:-python3}  # release_pytest needs CI's deps (release-tooling-tests.yml: jsonschema==4.21.1 pytest)
# RESUME=1 keeps every stage already at exit=0 in results.txt and runs only the rest (E7.9a: the
# first pass was killed by a reboot after three stages).
done_ok() { [ "${RESUME:-0}" = 1 ] && grep -qx "$1 exit=0" "$L/results.txt" 2>/dev/null; }
if [ "${RESUME:-0}" = 1 ] && [ -f "$L/results.txt" ]; then
  sed -i '/^RUN_DONE$/d' "$L/results.txt"
  for s in $STAGES; do done_ok "$s" || { echo "not-run" > "$L/$s.log"; sed -i "s/^$s .*/$s not-run/" "$L/results.txt"; }; done
else
  : > "$L/results.txt"
  for s in $STAGES; do echo "not-run" > "$L/$s.log"; echo "$s not-run" >> "$L/results.txt"; done
fi
mark() { sed -i "s/^$1 .*/$1 exit=$2/" "$L/results.txt"; }
run() { local s=$1; shift; if done_ok "$s"; then return 0; fi; ( "$@" ) > "$L/$s.log" 2>&1; local rc=$?; echo "exit=$rc" >> "$L/$s.log"; mark "$s" "$rc"; }

cd "$T"
run frontend_test bash -c "cd apps/desktop && npm test"
run frontend_typecheck bash -c "cd apps/desktop && npm run typecheck"
run release_pytest "$PY" -m pytest -q tools/release
run root_full cargo test --locked -j 8 --workspace --no-fail-fast -- --test-threads=8
run desktop_full bash -c "cd apps/desktop/src-tauri && cargo test --locked -j 8 --no-fail-fast -- --test-threads=8"
rm -rf "$SCRATCH/pf"; mkdir -p "$SCRATCH/pf"
run pf_hash bash -c "cd apps/desktop/src-tauri && PF_SEED_RENDER_OUT=$SCRATCH/pf cargo test --locked -j 8 --bins pf_seed_render_hash -- --nocapture --test-threads=8 && cat $SCRATCH/pf/sha256.txt"
mkdir -p "$L/pf" && cp "$SCRATCH"/pf/* "$L/pf/" 2>/dev/null
run sf_seed_root cargo test --locked -j 8 --lib sf_seed -- --test-threads=8
rm -rf "$SCRATCH/sf_lines"; mkdir -p "$SCRATCH/sf_lines"
run sf_seed_desktop bash -c "cd apps/desktop/src-tauri && SF_SEED_LINES_OUT=$SCRATCH/sf_lines cargo test --locked -j 8 --bin codex-desktop -- --test-threads=8 sf_ starfinder"
mkdir -p "$L/sf_seed_lines" && cp "$SCRATCH"/sf_lines/*.txt "$L/sf_seed_lines/" 2>/dev/null
run verify_sh bash scripts/verify.sh -j 8
store() {
  local d="$HOME/.local/share/io.electricm0nk.codex/characters"
  echo "entries=$(find "$d" -mindepth 1 -maxdepth 1 2>/dev/null | awk 'END{print NR}')"
  echo "tree_sha256=$(find "$d" -type f -print0 2>/dev/null | sort -z | xargs -0 sha256sum | sha256sum | cut -d' ' -f1)"
}
run store_before store
SF_ROWS=create-starfinder-soldier,open-starfinder-sheet,level-up-starfinder-soldier-preview,level-up-starfinder-soldier-accept,choices-starfinder-soldier-loadout,build-starfinder-soldier,open-starfinder-soldier,build-starfinder-mystic,open-starfinder-mystic,build-starfinder-technomancer,open-starfinder-technomancer,build-starfinder-envoy,open-starfinder-envoy,browse-starfinder-catalog,landing-select-pathfinder,open-seed-aldric,open-seed-elowen
run smoke_sf bash -c "cd apps/desktop && RUN_DESKTOP_AGENT=sd37-e7-9a-smoke node scripts/ui-smoke/run.mjs --only $SF_ROWS --out $SCRATCH/smoke_sf"
( cd apps/desktop && .claude/skills/run-desktop/driver.sh stop >/dev/null 2>&1 || true )
mkdir -p "$L/smoke_sf" && cp "$SCRATCH"/smoke_sf/results.json "$L/smoke_sf/" 2>/dev/null
run store_after store
echo RUN_DONE >> "$L/results.txt"
