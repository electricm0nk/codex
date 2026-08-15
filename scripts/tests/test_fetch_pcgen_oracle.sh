#!/usr/bin/env bash
# ---------------------------------------------------------------------------
# test_fetch_pcgen_oracle.sh — detection self-test for
# scripts/fetch-pcgen-oracle.sh, the pinned-PCGen-oracle bootstrap/verify
# script.
#
# WHY THIS EXISTS
# ----------------
# This script is the only place in the repo that resolves the PCGen oracle
# pin, so its ability to say NO (off-pin, dirty, absent, unproveable) is worth
# exactly as much as this self-test proves it is. This repo has already
# shipped gates that emitted their success token with full confidence while
# checking nothing (a bundle-tag audit implementing 3 of 4 patterns, an
# open-handle check silently dead behind a SIGPIPE) — same class of defect,
# same reason a self-test earns a `verify.sh` seat of its own
# (`oracle-pin-selftest`).
#
# Every case below runs against a synthetic local "upstream" — a bare git
# repo under mktemp with two commits, one file inside the pinned cone
# (data/pathfinder/...), one inside the OTHER pinned path
# (system/gameModes/Pathfinder/...) and one deliberately OUTSIDE the cone
# (data/other_publisher/...). Nothing here reads the real PCGen checkout or
# touches the network beyond the local filesystem.
#
# Usage: bash scripts/tests/test_fetch_pcgen_oracle.sh
# Exit 0 = all cases pass.
# ---------------------------------------------------------------------------
set -uo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$REPO/scripts/fetch-pcgen-oracle.sh"
[ -x "$SCRIPT" ] || { echo "FATAL: $SCRIPT not found or not executable" >&2; exit 2; }

PASSED=0
FAILED=0
pass() { echo "  PASS  $1"; PASSED=$((PASSED + 1)); }
fail() { echo "  FAIL  $1"; FAILED=$((FAILED + 1)); echo "        --- output ---"; echo "${2:-}" | sed 's/^/        /'; }

WORKROOT=$(mktemp -d)
cleanup() { rm -rf "$WORKROOT"; }
trap cleanup EXIT

# ---------------------------------------------------------------------------
# Build the synthetic upstream once: a bare repo with two commits.
#   commit 1 (the pin): data/pathfinder/paizo/roleplaying_game/core.lst,
#                        system/gameModes/Pathfinder/miscinfo.lst,
#                        data/other_publisher/should_not_appear.txt (out of cone)
#   commit 2 (off-pin):  core.lst appended to (a distinct SHA)
# ---------------------------------------------------------------------------
BARE="$WORKROOT/bare-upstream"
SEED="$WORKROOT/seed"
git init -q --bare "$BARE"
git init -q "$SEED"
(
    cd "$SEED"
    git config user.email test@example.com
    git config user.name "oracle-selftest"
    mkdir -p data/pathfinder/paizo/roleplaying_game system/gameModes/Pathfinder data/other_publisher
    echo "SOURCE:Core" > data/pathfinder/paizo/roleplaying_game/core.lst
    echo "MISC" > system/gameModes/Pathfinder/miscinfo.lst
    echo "outside the cone" > data/other_publisher/should_not_appear.txt
    git add -A
    git commit -q -m "commit1 (the pin)"
    echo "$(git rev-parse HEAD)" > "$WORKROOT/commit1.sha"

    echo "SOURCE:Core2" >> data/pathfinder/paizo/roleplaying_game/core.lst
    git commit -aqm "commit2 (off-pin)"
    echo "$(git rev-parse HEAD)" > "$WORKROOT/commit2.sha"

    git remote add origin "$BARE"
    git push -q origin HEAD:refs/heads/main
)
COMMIT1=$(cat "$WORKROOT/commit1.sha")
COMMIT2=$(cat "$WORKROOT/commit2.sha")

# Local transport still spawns upload-pack for a bare repo; both of these are
# required for `git fetch --depth 1 --filter=blob:none origin <sha>` to work
# against an arbitrary commit SHA rather than only an advertised ref tip.
git -C "$BARE" config uploadpack.allowFilter true
git -C "$BARE" config uploadpack.allowAnySHA1InWant true

# A pin file pointing at the synthetic upstream, read via PCGEN_ORACLE_PIN_FILE.
PIN="$WORKROOT/pin.env"
cat > "$PIN" <<EOF
PCGEN_ORACLE_REPO=file://$BARE
PCGEN_ORACLE_SHA=$COMMIT1
PCGEN_ORACLE_SPARSE_PATHS="data/pathfinder system/gameModes/Pathfinder"
EOF

run() {
    # run <dest> <extra args...> -- runs the script with PCGEN_ORACLE_PIN_FILE
    # set to the synthetic pin, captures combined stdout+stderr, sets $OUT/$ST.
    OUT=$(PCGEN_ORACLE_PIN_FILE="$PIN" "$SCRIPT" "$@" 2>&1)
    ST=$?
}

fresh_dest() {
    local d="$WORKROOT/$1"
    printf '%s' "$d"
}

# --- 1. --check on an absent DEST -> exit 1, names the fetch script. -------
D1=$(fresh_dest d1-absent)
run --dest "$D1" --check
if [ "$ST" -eq 1 ] && echo "$OUT" | grep -q "fetch-pcgen-oracle.sh"; then
  pass "--check on an absent DEST exits 1 and names the fetch script"
else fail "--check on an absent DEST exits 1 and names the fetch script" "$OUT (exit $ST)"; fi

# --- 2. Fresh fetch -> exit 0, OK token, export line, HEAD==pin, cone -------
#        present, out-of-cone file absent.
D2=$(fresh_dest d2-fresh)
run --dest "$D2"
head2=$(git -C "$D2" rev-parse HEAD 2>/dev/null || echo "")
if [ "$ST" -eq 0 ] \
   && echo "$OUT" | grep -q "^pcgen-oracle: OK" \
   && echo "$OUT" | grep -q "^export PCGEN_CORPUS_ROOT=$D2/data$" \
   && echo "$OUT" | grep -q "^export PCGEN_REPO_DIR=$D2$" \
   && [ "$head2" = "$COMMIT1" ] \
   && [ -f "$D2/data/pathfinder/paizo/roleplaying_game/core.lst" ] \
   && [ -f "$D2/system/gameModes/Pathfinder/miscinfo.lst" ] \
   && [ ! -e "$D2/data/other_publisher/should_not_appear.txt" ]; then
  pass "fresh fetch: exit 0, OK token, exports, HEAD at pin, cone present, out-of-cone absent"
else
  fail "fresh fetch: exit 0, OK token, exports, HEAD at pin, cone present, out-of-cone absent" \
    "$OUT (exit $ST, HEAD=$head2)"
fi

# --- 3. Re-run on the same DEST -> no-op, exit 0 (idempotent). -------------
run --dest "$D2"
if [ "$ST" -eq 0 ] && echo "$OUT" | grep -q "^pcgen-oracle: OK"; then
  pass "re-run on an already-on-pin DEST is idempotent (exit 0)"
else fail "re-run on an already-on-pin DEST is idempotent (exit 0)" "$OUT (exit $ST)"; fi

# --- 4. Checkout moved to commit2 -> --check exit 1, both SHAs named. ------
D4=$(fresh_dest d4-offpin)
run --dest "$D4"
[ "$ST" -eq 0 ] || { fail "case 4 setup: bootstrap D4" "$OUT"; }
# Advance D4 to commit2 directly (simulating drift), bypassing the script.
git -C "$D4" fetch -q --depth 1 origin "$COMMIT2" >/dev/null 2>&1
git -C "$D4" checkout -q --detach FETCH_HEAD >/dev/null 2>&1
run --dest "$D4" --check
if [ "$ST" -eq 1 ] && echo "$OUT" | grep -q "$COMMIT1" && echo "$OUT" | grep -q "$COMMIT2"; then
  pass "off-pin checkout: --check exits 1 and names both SHAs"
else fail "off-pin checkout: --check exits 1 and names both SHAs" "$OUT (exit $ST)"; fi

# --- 5. --force on case 4's DEST -> HEAD back at pin, exit 0. --------------
run --dest "$D4" --force
head4=$(git -C "$D4" rev-parse HEAD 2>/dev/null || echo "")
if [ "$ST" -eq 0 ] && [ "$head4" = "$COMMIT1" ]; then
  pass "--force moves an off-pin checkout back to the pin"
else fail "--force moves an off-pin checkout back to the pin" "$OUT (exit $ST, HEAD=$head4)"; fi

# --- 6. Dirty tracked .lst in the cone -> --check exit 1 naming the file; --
#        --force refuses too.
D6=$(fresh_dest d6-dirty)
run --dest "$D6"
[ "$ST" -eq 0 ] || { fail "case 6 setup: bootstrap D6" "$OUT"; }
echo "LOCALLY EDITED" >> "$D6/data/pathfinder/paizo/roleplaying_game/core.lst"
run --dest "$D6" --check
if [ "$ST" -eq 1 ] && echo "$OUT" | grep -q "core.lst"; then
  pass "dirty tracked cone file: --check exits 1 naming the file"
else fail "dirty tracked cone file: --check exits 1 naming the file" "$OUT (exit $ST)"; fi
run --dest "$D6" --force
if [ "$ST" -eq 1 ] && echo "$OUT" | grep -q "core.lst"; then
  pass "dirty tracked cone file: --force refuses too, naming the file"
else fail "dirty tracked cone file: --force refuses too, naming the file" "$OUT (exit $ST)"; fi

# --- 7. DEST is a plain directory (no .git) -> exit 1. ----------------------
D7=$(fresh_dest d7-plain)
mkdir -p "$D7"
echo "not a git repo" > "$D7/marker.txt"
run --dest "$D7" --check
if [ "$ST" -eq 1 ]; then
  pass "DEST exists but is not a git checkout -> exit 1"
else fail "DEST exists but is not a git checkout -> exit 1" "$OUT (exit $ST)"; fi

# --- 8. Pin file lacking PCGEN_ORACLE_SHA -> exit 2. ------------------------
BADPIN="$WORKROOT/badpin.env"
echo "PCGEN_ORACLE_REPO=file://$BARE" > "$BADPIN"
D8=$(fresh_dest d8-badpin)
OUT=$(PCGEN_ORACLE_PIN_FILE="$BADPIN" "$SCRIPT" --dest "$D8" --check 2>&1); ST=$?
if [ "$ST" -eq 2 ]; then
  pass "pin file missing PCGEN_ORACLE_SHA -> exit 2"
else fail "pin file missing PCGEN_ORACLE_SHA -> exit 2" "$OUT (exit $ST)"; fi

# --- 9. DEST is a FULL clone (plain `git clone`) at the pin -> --check OK. -
#        The operator's own real arrangement (~/workspace/repos/pcgen is a
#        full clone) must pass exactly this way.
D9="$WORKROOT/d9-full"
git clone -q "$BARE" "$D9" >/dev/null 2>&1
git -C "$D9" checkout -q --detach "$COMMIT1" >/dev/null 2>&1
run --dest "$D9" --check
if [ "$ST" -eq 0 ] && echo "$OUT" | grep -q "^pcgen-oracle: OK"; then
  pass "a full (non-sparse) clone at the pin passes --check"
else fail "a full (non-sparse) clone at the pin passes --check" "$OUT (exit $ST)"; fi

# --- 10. --check exits 0 only WITH the OK token present. --------------------
#         Guards the same "exit 0 without the success token" class the
#         corpus-sweep and pi-sweep verify.sh stages already guard.
if [ "$ST" -eq 0 ] && printf '%s\n' "$OUT" | grep -qx "pcgen-oracle: OK $COMMIT1 $D9"; then
  pass "successful --check emits the exact pcgen-oracle: OK token"
else fail "successful --check emits the exact pcgen-oracle: OK token" "$OUT (exit $ST)"; fi

echo "---------------------------------------------------------------"
echo "passed: $PASSED  failed: $FAILED"
[ "$FAILED" -eq 0 ] || { echo "SELF-TEST FAILED."; exit 1; }
echo "SELF-TEST PASSED."
