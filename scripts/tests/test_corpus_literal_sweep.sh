#!/usr/bin/env bash
# ---------------------------------------------------------------------------
# test_corpus_literal_sweep.sh — detection self-test for the
# `corpus_literal_sweep` binary (src/bin/corpus_literal_sweep.rs).
#
# WHY THIS EXISTS
# ---------------
# The sweep is the instrument that gates every `static` unit's bar: a record
# whose whole token closure is literal magnitudes must byte-match the corpus
# literal it cites. An instrument like that is worth exactly as much as its
# proven ability to say NO. This repo has already shipped two gates that could
# not: a bundle-tag audit implementing 3 of its 4 patterns, and an open-handle
# check silently dead behind a SIGPIPE. Both emitted their success token with
# full confidence while checking nothing.
#
# So every case below plants a DELIBERATELY CORRUPTED record and asserts the
# sweep goes red for the intended reason — and, symmetrically, plants the
# faithful shapes that must NOT be flagged, because a gate that fails
# everything is as useless as one that passes everything.
#
# Case 11 is the one that matters most: an empty population must exit 2, not
# 0. A sweep that examined nothing proves nothing, and "clean" is the wrong
# word for it.
#
# Every case runs against throwaway roots under mktemp -- a synthetic repo
# root holding one JSON record and a synthetic PCGen corpus holding one .lst
# file. Nothing reads the real corpus or the real data/corpus tree.
#
# Usage: bash scripts/tests/test_corpus_literal_sweep.sh
# Exit 0 = all cases pass.
# ---------------------------------------------------------------------------
set -uo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
[ -f "$REPO/src/bin/corpus_literal_sweep.rs" ] || {
  echo "FATAL: corpus_literal_sweep.rs not found under $REPO/src/bin" >&2; exit 2; }

PASSED=0
FAILED=0
pass() { echo "  PASS  $1"; PASSED=$((PASSED + 1)); }
fail() { echo "  FAIL  $1"; FAILED=$((FAILED + 1)); echo "        --- output ---"; echo "${2:-}" | sed 's/^/        /'; }

BOOKS_REL="pathfinder/paizo/roleplaying_game"
BOOK="testbook"

# Build once; every case then runs the same binary. `cargo run` would rebuild
# nothing but still take the target-dir lock on each of the twelve calls.
BIN="$REPO/.corpus-literal-sweep-selftest-bin"
build_log=$(cd "$REPO" && cargo build --locked --quiet --bin corpus_literal_sweep 2>&1)
build_status=$?
if [ "$build_status" -ne 0 ]; then
  echo "FATAL: cargo build --bin corpus_literal_sweep failed (exit $build_status)" >&2
  echo "$build_log" >&2
  exit 2
fi
TARGET_DIR="${CARGO_TARGET_DIR:-$REPO/target}"
BIN="$TARGET_DIR/debug/corpus_literal_sweep"
[ -x "$BIN" ] || { echo "FATAL: built binary not at $BIN" >&2; exit 2; }

# fresh_case: makes $WS (synthetic repo root) and $CORPUS (synthetic corpus
# root) with one .lst file whose rows are the arguments, tab-joined as given.
fresh_case() {
  WS=$(mktemp -d)
  CORPUS=$(mktemp -d)
  mkdir -p "$WS/data/corpus/$BOOK/equipment/general"
  mkdir -p "$CORPUS/$BOOKS_REL/$BOOK"
  LST="$CORPUS/$BOOKS_REL/$BOOK/tb_equip.lst"
  : > "$LST"
}

drop_case() { rm -rf "$WS" "$CORPUS"; }

lst_sha() { sha256sum "$LST" | cut -d' ' -f1; }

# write_record <json-file-basename> <line> <sha> <tokens-json>
write_record() {
  cat > "$WS/data/corpus/$BOOK/equipment/general/$1.json" <<JSON
{
  "population": "in_scope",
  "completeness": "full",
  "data": {
    "key": "Thing",
    "name": "Thing",
    "category": "general",
    "raw_tokens": $4
  },
  "source": {
    "kind": "lst_token",
    "path": "$BOOKS_REL/$BOOK/tb_equip.lst",
    "sha256": "$3",
    "line": $2,
    "record_key": "Thing"
  },
  "license": "OGL"
}
JSON
}

run_sweep() {
  "$BIN" --repo-root "$WS" --corpus-root "$CORPUS" 2>&1
}

echo "===== corpus_literal_sweep — detection self-test ====="

# --- 1. A faithful record is CLEAN, and says how much it examined. ----------
fresh_case
printf 'Thing\t\t\tCOST:50\t\t\tWT:3\n' > "$LST"
write_record thing 1 "$(lst_sha)" '[{"key":"COST","value":"50"},{"key":"WT","value":"3"}]'
out=$(run_sweep); st=$?
if [ "$st" -eq 0 ] && echo "$out" | grep -q "corpus-literal-sweep: CLEAN" \
   && echo "$out" | grep -q "1 records examined" && echo "$out" | grep -q "2 tokens compared"; then
  pass "faithful record is CLEAN and reports its population"
else fail "faithful record is CLEAN and reports its population" "$out (exit $st)"; fi
drop_case

# --- 2. A one-byte value drift is caught. -----------------------------------
# THE case this instrument exists for: the shipped COST is 500, the corpus
# says 50. Nothing else about the record changed.
fresh_case
printf 'Thing\t\t\tCOST:50\t\t\tWT:3\n' > "$LST"
write_record thing 1 "$(lst_sha)" '[{"key":"COST","value":"500"},{"key":"WT","value":"3"}]'
out=$(run_sweep); st=$?
if [ "$st" -eq 1 ] && echo "$out" | grep -q "MISMATCH.*token not byte-present in corpus token closure: COST:500"; then
  pass "one-byte magnitude drift is caught (COST 500 over corpus 50)"
else fail "one-byte magnitude drift is caught (COST 500 over corpus 50)" "$out (exit $st)"; fi
drop_case

# --- 3. A token the corpus row never carried is caught. ---------------------
fresh_case
printf 'Thing\t\t\tCOST:50\n' > "$LST"
write_record thing 1 "$(lst_sha)" '[{"key":"COST","value":"50"},{"key":"SPELLFAILURE","value":"35"}]'
out=$(run_sweep); st=$?
if [ "$st" -eq 1 ] && echo "$out" | grep -q "MISMATCH.*SPELLFAILURE:35"; then
  pass "a token absent from the corpus row is caught"
else fail "a token absent from the corpus row is caught" "$out (exit $st)"; fi
drop_case

# --- 4. Trailing whitespace is a mismatch, not a formatting difference. -----
fresh_case
printf 'Thing\t\t\tDAMAGE:1d8\n' > "$LST"
write_record thing 1 "$(lst_sha)" '[{"key":"DAMAGE","value":"1d8 "}]'
out=$(run_sweep); st=$?
if [ "$st" -eq 1 ] && echo "$out" | grep -q "MISMATCH.*DAMAGE:1d8"; then
  pass "byte-equality does not forgive trailing whitespace"
else fail "byte-equality does not forgive trailing whitespace" "$out (exit $st)"; fi
drop_case

# --- 5. A corpus file that changed under the record is caught. --------------
fresh_case
printf 'Thing\t\t\tCOST:50\n' > "$LST"
write_record thing 1 "$(lst_sha)" '[{"key":"COST","value":"50"}]'
# The tokens still match; only the file's digest moved. Without the digest
# check this record would pass while its provenance claim is false.
printf 'Thing\t\t\tCOST:50\t\t\tNOTE:added later\n' > "$LST"
out=$(run_sweep); st=$?
if [ "$st" -eq 1 ] && echo "$out" | grep -q "MISMATCH.*digest drift"; then
  pass "corpus file drifting under a still-matching record is caught"
else fail "corpus file drifting under a still-matching record is caught" "$out (exit $st)"; fi
drop_case

# --- 6. source.line past end of file is caught. -----------------------------
fresh_case
printf 'Thing\t\t\tCOST:50\n' > "$LST"
write_record thing 99 "$(lst_sha)" '[{"key":"COST","value":"50"}]'
out=$(run_sweep); st=$?
if [ "$st" -eq 1 ] && echo "$out" | grep -q "MISMATCH.*record claims line 99"; then
  pass "a source.line past end of file is caught"
else fail "a source.line past end of file is caught" "$out (exit $st)"; fi
drop_case

# --- 7. A corpus file the record names but that does not exist is caught. ---
fresh_case
printf 'Thing\t\t\tCOST:50\n' > "$LST"
write_record thing 1 "$(lst_sha)" '[{"key":"COST","value":"50"}]'
rm -f "$LST"
out=$(run_sweep); st=$?
if [ "$st" -eq 1 ] && echo "$out" | grep -q "MISMATCH.*corpus file missing"; then
  pass "a cited corpus file that does not exist is caught"
else fail "a cited corpus file that does not exist is caught" "$out (exit $st)"; fi
drop_case

# --- 8. A token carried by a .MOD row in the same book is NOT flagged. ------
# The 25-record correction: the base row alone is the wrong comparand.
fresh_case
printf 'Thing\t\t\tCOST:50\n' > "$LST"
printf 'CATEGORY=Special Ability|Thing.MOD\tSR:13\n' >> "$LST"
write_record thing 1 "$(lst_sha)" '[{"key":"COST","value":"50"},{"key":"SR","value":"13"}]'
out=$(run_sweep); st=$?
if [ "$st" -eq 0 ] && echo "$out" | grep -q "CLEAN"; then
  pass "a .MOD-carried token in the same book is faithful, not drift"
else fail "a .MOD-carried token in the same book is faithful, not drift" "$out (exit $st)"; fi
drop_case

# --- 9. A declared synthesized token found in the book corpus is CLEAN. -----
fresh_case
printf 'Thing\t\t\tCOST:50\n' > "$LST"
printf 'Other.MOD\tABILITY:Aasimar Racial Trait|AUTOMATIC|Thing|PREVAREQ:X,0\n' >> "$LST"
write_record thing 1 "$(lst_sha)" '[{"key":"COST","value":"50"},{"key":"GLOBALVAR:ABILITY","value":"Aasimar Racial Trait|AUTOMATIC|Thing|PREVAREQ:X,0"}]'
out=$(run_sweep); st=$?
if [ "$st" -eq 0 ] && echo "$out" | grep -q "1 synthesized"; then
  pass "a synthesized token present verbatim in the book corpus is faithful"
else fail "a synthesized token present verbatim in the book corpus is faithful" "$out (exit $st)"; fi
drop_case

# --- 10. A synthesized token NOT in the book corpus is caught. --------------
fresh_case
printf 'Thing\t\t\tCOST:50\n' > "$LST"
write_record thing 1 "$(lst_sha)" '[{"key":"COST","value":"50"},{"key":"GLOBALVAR:ABILITY","value":"Invented|AUTOMATIC|Thing|PREVAREQ:X,0"}]'
out=$(run_sweep); st=$?
if [ "$st" -eq 1 ] && echo "$out" | grep -q "MISMATCH.*synthesized token not byte-present"; then
  pass "a synthesized token absent from the book corpus is caught"
else fail "a synthesized token absent from the book corpus is caught" "$out (exit $st)"; fi
drop_case

# --- 11. An undeclared namespaced key does not inherit the exemption. -------
fresh_case
printf 'Thing\t\t\tCOST:50\n' > "$LST"
write_record thing 1 "$(lst_sha)" '[{"key":"COST","value":"50"},{"key":"INVENTED:ABILITY","value":"anything"}]'
out=$(run_sweep); st=$?
if [ "$st" -eq 1 ] && echo "$out" | grep -q "MISMATCH.*not a declared synthesized key: INVENTED:ABILITY"; then
  pass "a new namespaced key must be declared, never silently exempt"
else fail "a new namespaced key must be declared, never silently exempt" "$out (exit $st)"; fi
drop_case

# --- 12. AN EMPTY POPULATION IS EXIT 2, NEVER A CLEAN PASS. -----------------
# The false-green case. Records exist but none is in the sweep's population,
# so the sweep compared zero tokens. That is a broken sweep, not a clean tree.
fresh_case
printf 'Thing\t\t\tCOST:50\n' > "$LST"
cat > "$WS/data/corpus/$BOOK/equipment/general/thing.json" <<JSON
{"source":{"kind":"web_second_source","path":"x","line":1},"data":{"key":"Thing"}}
JSON
out=$(run_sweep); st=$?
if [ "$st" -eq 2 ] && echo "$out" | grep -q "0 records examined" \
   && echo "$out" | grep -q "proves nothing" && ! echo "$out" | grep -q "CLEAN"; then
  pass "an empty population exits 2 and never prints CLEAN"
else fail "an empty population exits 2 and never prints CLEAN" "$out (exit $st)"; fi
drop_case

# --- 13. A missing corpus root is exit 2, never a clean pass. ---------------
fresh_case
printf 'Thing\t\t\tCOST:50\n' > "$LST"
write_record thing 1 "$(lst_sha)" '[{"key":"COST","value":"50"}]'
out=$("$BIN" --repo-root "$WS" --corpus-root /nonexistent-corpus-root 2>&1); st=$?
if [ "$st" -eq 2 ] && ! echo "$out" | grep -q "CLEAN"; then
  pass "an absent corpus root exits 2 and never prints CLEAN"
else fail "an absent corpus root exits 2 and never prints CLEAN" "$out (exit $st)"; fi
drop_case

# --- 14. A record with NO tokens still has its digest claim checked. --------
# The provenance population is wider than the token population on purpose: a
# spell/monster/companion record carries no raw_tokens but still cites a file
# and a digest, and 5387 of the tree's 8903 claims are of that shape. Scoping
# the digest check to the token population would leave them unverified while
# the stage reported full coverage.
fresh_case
printf 'Thing\t\t\tCOST:50\n' > "$LST"
write_record thing 1 "$(lst_sha)" '[{"key":"COST","value":"50"}]'
cat > "$WS/data/corpus/$BOOK/equipment/general/tokenless.json" <<JSON
{"source":{"kind":"web_second_source","path":"$BOOKS_REL/$BOOK/tb_equip.lst",
 "sha256":"0000000000000000000000000000000000000000000000000000000000000000"},
 "data":{"key":"Tokenless"}}
JSON
out=$(run_sweep); st=$?
if [ "$st" -eq 1 ] && echo "$out" | grep -q "MISMATCH.*tokenless.json.*digest drift"; then
  pass "a record outside the token population still has its digest checked"
else fail "a record outside the token population still has its digest checked" "$out (exit $st)"; fi
drop_case

# --- 15. Malformed JSON is exit 2, never skipped. ---------------------------
fresh_case
printf 'Thing\t\t\tCOST:50\n' > "$LST"
printf '{not json' > "$WS/data/corpus/$BOOK/equipment/general/broken.json"
out=$(run_sweep); st=$?
if [ "$st" -eq 2 ] && echo "$out" | grep -q "invalid JSON"; then
  pass "a malformed shipped record is a hard failure, not a silent skip"
else fail "a malformed shipped record is a hard failure, not a silent skip" "$out (exit $st)"; fi
drop_case

echo "---------------------------------------------------------------"
echo "passed: $PASSED  failed: $FAILED"
[ "$FAILED" -eq 0 ] || { echo "SELF-TEST FAILED."; exit 1; }
echo "SELF-TEST PASSED."
