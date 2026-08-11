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

# --- Grand-epic (GE-NN) tag family. -----------------------------------------
# Operator directive 2026-08-11 ("references to SD and ge were to be replaced a
# few tranches ago with function based naming"): the GE-NN family is banned on
# exactly the same terms as SD-NN. Epic 1's hardened regex covered only SD.
run_case 'snake grand-epic tag (ge08_)'    1 src/gen.rs               'pub mod ge08_workbench;'
run_case 'screaming grand-epic tag (GE06_)' 1 src/gen.rs              'const GE06_BASE_ARMOR_CLASS: i16 = 16;'
run_case 'pascal grand-epic tag (Ge08)'    1 src/gen.rs               'pub struct Ge08AuthoringWorkbenchRequest;'
run_case 'hyphen grand-epic tag (ge08-)'   1 apps/desktop/src/a.ts    'const cls = "ge08-workbench-row";'

# --- Infix forms: the tag is not at the start of the identifier. -------------
# `_` is a word character, so `\b(sd|ge)[0-9]+` can never match `kind_is_sd17_b3`
# — both of the live examples below passed Epic 1's hardened gate clean.
run_case 'infix bundle tag (kind_is_sd17_b3)' 1 src/gen.rs            'pub fn kind_is_sd17_b3() -> bool { true }'
run_case 'infix grand-epic tag'            1 src/gen.rs               'fn build_ge08_workbench_snapshot() {}'
run_case 'infix seeded fixture accessor'   1 src/gen.rs               'pub fn seeded_sd13_e1_f1_current_truth() {}'

# --- PATH tags: file and directory names carry the same ban. ----------------
# Epic 1 established that the audit regex is identifier-shaped and catches NO
# path tags at all. A file named src/bin/sd27_gen_book_cache.rs is a bundle tag
# in exactly the sense the doctrine bans, and every one of these passed clean.
run_case 'path tag in file name'           1 src/bin/sd27_gen_book_cache.rs 'pub fn main() {}'
run_case 'path tag in directory name'      1 apps/desktop/src/sd16/update/fetch.ts 'export const x = 1;'
run_case 'path tag in grand-epic file'     1 apps/desktop/src-tauri/src/ge08_workbench.rs 'pub fn main() {}'

# --- Non-detection cases: each MUST stay clean (exit 0). --------------------
# Documented exclusion class (identifier-discipline doctrine, SD-25 1.1): a doc
# comment or string literal citing a REAL tests/... file by name is a
# test-traceability grounding, not an identifier carrying a bundle tag.
run_case 'tests/ citation in doc comment'  0 src/gen.rs               '/// Grounded by tests/sd13_fighter_level9_level10_progression.rs.'
run_case 'tests/ citation string literal'  0 src/gen.rs               'const T: &str = "tests/ge06_pilot_input_contract.rs";'
run_case 'tests/ path is not a path tag'   0 tests/sd13_progression.rs 'fn t() {}'

# --- Removals: deleting a tagged identifier is the CURE, not the disease. ----
# The script's own header says it "flags bundle identifiers newly introduced by
# the cycle". A rename sweep's diff necessarily carries the old tagged name on
# its `-` lines, and scanning the whole diff flagged exactly that — the gate
# failed the one cycle whose entire purpose was removing the tags.
remove_case() {
  local name="$1" expected="$2" file="$3" before="$4" after="$5"
  local dir rc
  dir="$(mktemp -d)"
  (
    cd "$dir" || exit 1
    git init -q -b base; git config user.email t@t.invalid; git config user.name t
    mkdir -p "$(dirname "$file")"
    printf 'pub fn ok() {}\n' > src/lib.rs
    printf '%s\n' "$before" > "$file"
    git add -A; git commit -qm base; git checkout -qb work
    printf '%s\n' "$after" > "$file"
    git add -A; git commit -qm work
  ) >/dev/null 2>&1
  ( cd "$dir" && BASE_BRANCH=base bash "$AUDIT" ) >/dev/null 2>&1
  rc=$?
  rm -rf "$dir"
  if [ "$rc" -eq "$expected" ]; then
    echo "  PASS  $name (exit $rc)"; PASSED=$((PASSED + 1))
  else
    echo "  FAIL  $name (expected exit $expected, got $rc)"; FAILED=$((FAILED + 1))
  fi
}
remove_case 'renaming a tagged fn away is clean' 0 src/gen.rs \
  'pub fn seeded_sd13_e1_f1_current_truth() {}' 'pub fn seeded_current_truth() {}'
remove_case 'renaming a tagged const away is clean' 0 src/gen.rs \
  'const GE06_BASE_ARMOR_CLASS: i16 = 16;' 'const BASE_ARMOR_CLASS: i16 = 16;'
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
