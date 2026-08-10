#!/usr/bin/env bash
# ---------------------------------------------------------------------------
# test_identifier_discipline_audit.sh — regression self-test for
# scripts/identifier-discipline-audit.sh.
#
# Why this exists: that script's own header records TWO live escapes of the
# gate (the misplaced `\b`, and the missing `:(glob)` magic), each of which let
# a real bundle tag pass clean. Neither escape had a test protecting it, so the
# gate's "OK_NO_BUNDLE_TAGS" token was an unverified assertion. SD-29 Epic 1
# (identifier-disclosure audit pass) requires the audit script to "return 0
# findings" — a claim only worth as much as the gate's proven detection power.
#
# Each case builds a throwaway git repo, plants exactly one pattern in shipping
# source, and asserts the audit's exit code. Nothing touches the real repo.
#
# Usage: bash scripts/tests/test_identifier_discipline_audit.sh
# Exit 0 = all cases pass.
# ---------------------------------------------------------------------------
set -uo pipefail

AUDIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)/scripts/identifier-discipline-audit.sh"
[ -f "$AUDIT" ] || { echo "FATAL: audit script not found at $AUDIT" >&2; exit 2; }

PASSED=0
FAILED=0

# Builds a scratch repo with a `base` branch, then applies $2 (a shell snippet
# run inside the repo) as the HEAD commit's change. Echoes the repo path.
make_repo() {
  local dir
  dir="$(mktemp -d)"
  (
    cd "$dir" || exit 1
    git init -q -b base
    git config user.email t@t.invalid
    git config user.name t
    mkdir -p src apps/desktop/src apps/desktop/src-tauri/src
    printf 'pub fn ok() {}\n' > src/lib.rs
    git add -A
    git commit -qm base
    git checkout -qb work
  ) >/dev/null 2>&1
  echo "$dir"
}

# run_case <name> <expected-exit> <file> <line-content>
run_case() {
  local name="$1" expected="$2" file="$3" content="$4"
  local dir rc
  dir="$(make_repo)"
  (
    cd "$dir" || exit 1
    mkdir -p "$(dirname "$file")"
    printf '%s\n' "$content" >> "$file"
    git add -A
    git commit -qm work
  ) >/dev/null 2>&1

  ( cd "$dir" && BASE_BRANCH=base bash "$AUDIT" ) >/dev/null 2>&1
  rc=$?
  rm -rf "$dir"

  if [ "$rc" -eq "$expected" ]; then
    echo "  PASS  $name (exit $rc)"
    PASSED=$((PASSED + 1))
  else
    echo "  FAIL  $name (expected exit $expected, got $rc)"
    FAILED=$((FAILED + 1))
  fi
}

echo "===== identifier-discipline-audit.sh — detection self-test ====="

# --- Detection cases: each MUST be caught (exit 1). --------------------------
# The four patterns SD-29 Epic 1 (epic-breakdown.md SD29-E1-F1) names by name.
run_case 'snake bundle tag (sd29_)'        1 src/gen.rs               'pub const SD_X: u8 = 0; // sd29_book_cache'
run_case 'screaming bundle tag (SD29_)'    1 src/gen.rs               'pub const SD29_BOOK_ID: &str = "b2";'
run_case 'pascal bundle tag (Sd29)'        1 src/gen.rs               'pub struct Sd29BookCache;'
run_case 'hyphen bundle tag (sd29-)'       1 apps/desktop/src/a.ts    'const cls = "sd29-monster-row";'
run_case 'kanban token (t_<hex8>)'         1 src/gen.rs               '// slice t_3cf90c2c'
# Escapes the script header records as having happened live.
run_case 'tag in top-level src/lib.rs'     1 src/lib.rs               'pub fn sd29_flag() {}'
run_case 'tag in tauri crate'              1 apps/desktop/src-tauri/src/m.rs 'pub const SD29_GATE: bool = true;'

# --- Non-detection cases: each MUST stay clean (exit 0). --------------------
# Doc-style bundle slugs are the normal way source comments cite this bundle's
# release package; flagging them would make the gate unusable.
run_case 'doc slug SD-29 in a comment'     0 src/gen.rs               '// See docs/release/SD-29-corpus-wide-catch-up-lanes/decisions.md'
run_case 'doc slug SD-29 possessive'       0 src/gen.rs               "// assigned to SD-29's bundle by epic-breakdown.md"
run_case 'tests are out of scope'          0 src/__tests__/x.rs       'pub fn sd29_fixture() {}'
# Epic-label citations in doc comments. 777 of these live in shipping source
# (re-derived 2026-08-10:
#   grep -rnE '\b[Ss][Dd][0-9]+-[A-Za-z0-9][A-Za-z0-9-]*\b' --include='*.rs' \
#     --include='*.ts' --include='*.tsx' src apps/desktop/src \
#     apps/desktop/src-tauri/src | wc -l  ->  777)
# so a hyphen pattern that does not exempt them turns the gate into noise the
# moment any lane writes a comment citing its own epic.
run_case 'epic citation SD28-E14-F1'       0 src/gen.rs               '/// SD28-E14-F1: closes the observation gap noted above.'
run_case 'epic citation SD29-E1'           0 src/gen.rs               '// The SD29-E1 identifier pass covers this module.'
run_case 'unrelated identifier'            0 src/gen.rs               'pub fn standard_cache() {}'

echo "---------------------------------------------------------------"
echo "passed: $PASSED  failed: $FAILED"
[ "$FAILED" -eq 0 ] || { echo "SELF-TEST FAILED."; exit 1; }
echo "SELF-TEST PASSED."
