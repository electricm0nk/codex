#!/usr/bin/env bash
# ---------------------------------------------------------------------------
# test_publish_site_dashboard.sh — self-test for scripts/publish-site-
# dashboard.sh's `--check` mode, the freshness gate for the versioned public
# status feed under site/dashboard/.
#
# WHY THIS EXISTS
# ----------------
# `--check` renders the feed into a scratch location and compares it to the
# committed copy, ignoring stamps that move on every run by design. Its
# first version got that comparison wrong in a way that made it USELESS
# without being visibly broken: it rendered into a blank-slate scratch file
# instead of one seeded from the committed copy, so the real producer's
# owner-state merge and unit-shard cache always saw "nothing prior" and the
# check reported STALE on every run, including ones where nothing had
# changed. A `--check` stage that always fails is exactly as informative as
# one that always passes, and this repo has already shipped that shape of
# defect more than once (see `run_producer_selftest`'s and
# `run_oracle_pin_selftest`'s own doc comments in verify.sh). Proven here
# against a tiny FAKE producer (not the real ~4,000-line one) so the case is
# fast, deterministic, and does not need the real corpus:
#
#   1. An unseeded run over an untouched tree must NOT report stale (guards
#      the false-positive class the first draft shipped).
#   2. A genuinely stale committed copy MUST be caught (guards the opposite
#      failure — a check that always passes).
#   3. Owner-state that only the real producer would carry forward (modeled
#      here by a `sticky` field the fake producer echoes back from `--out`
#      if present) must survive a clean `--check` run unchanged.
#
# Usage: bash scripts/tests/test_publish_site_dashboard.sh
# Exit 0 = all cases pass.
# ---------------------------------------------------------------------------
set -uo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$REPO/scripts/publish-site-dashboard.sh"
[ -f "$SCRIPT" ] || { echo "FATAL: $SCRIPT not found" >&2; exit 2; }

PASSED=0
FAILED=0
pass() { echo "  PASS  $1"; PASSED=$((PASSED + 1)); }
fail() { echo "  FAIL  $1"; FAILED=$((FAILED + 1)); echo "        --- output ---"; echo "${2:-}" | sed 's/^/        /'; }

WORKROOT=$(mktemp -d)
cleanup() { rm -rf "$WORKROOT"; }
trap cleanup EXIT

# A fake "repo" -- just enough directory shape for the script's own path
# math (`$REPO_ROOT/site/dashboard/PF1e-dashboard.json`).
FAKE_REPO="$WORKROOT/fake-repo"
mkdir -p "$FAKE_REPO/scripts" "$FAKE_REPO/site/dashboard"
cp "$SCRIPT" "$FAKE_REPO/scripts/publish-site-dashboard.sh"
chmod +x "$FAKE_REPO/scripts/publish-site-dashboard.sh"

# The fake producer: writes a small JSON document to --out. Models the two
# real-producer behaviors the seeding fix depends on:
#   - `generated_at` always moves (every real run re-stamps it; scrubbed by
#     the comparison, so this alone must never cause a STALE report).
#   - `sticky` is read back from any PRE-EXISTING file at --out and carried
#     forward unchanged -- the same shape as the real producer's owner-state
#     merge (`_load_existing_owner_state`) and its shard-cache reuse
#     (`build_unit_shards`), both of which only see prior state when --out
#     is seeded before the producer runs.
FAKE_PRODUCER="$WORKROOT/fake_producer.py"
cat >"$FAKE_PRODUCER" <<'PY'
import argparse
import json
import os
import sys
import time

p = argparse.ArgumentParser()
p.add_argument("--out", required=True)
args = p.parse_args()

sticky = "DEFAULT-NO-PRIOR-STATE"
if os.path.exists(args.out):
    try:
        with open(args.out) as f:
            prior = json.load(f)
        if "sticky" in prior:
            sticky = prior["sticky"]
    except (OSError, json.JSONDecodeError):
        pass

doc = {
    "generated_at": f"TS-{time.time_ns()}",
    "generated_by": "fake_producer",
    "sticky": sticky,
    "figure": int(os.environ.get("FAKE_FIGURE", "42")),
}
with open(args.out, "w") as f:
    json.dump(doc, f)
print(f"fake-producer: rendered {args.out}")
sys.exit(0)
PY

run() {
    OUT=$(cd "$FAKE_REPO" && PF1E_DASHBOARD_PRODUCER="$FAKE_PRODUCER" ./scripts/publish-site-dashboard.sh "$@" 2>&1)
    ST=$?
}

COMMITTED="$FAKE_REPO/site/dashboard/PF1e-dashboard.json"

# --- 1. First real (non---check) run seeds the committed copy, sticky="A". -
FAKE_FIGURE=1 run
if [ "$ST" -eq 0 ] && [ -f "$COMMITTED" ]; then
    pass "a real run writes the committed copy"
else fail "a real run writes the committed copy" "$OUT (exit $ST)"; fi
python3 - "$COMMITTED" <<'PY'
import json, sys
d = json.load(open(sys.argv[1]))
d["sticky"] = "A"
json.dump(d, open(sys.argv[1], "w"))
PY

# --- 2. --check on an UNTOUCHED tree must report current, not stale. -------
#         This is the exact bug this cycle found and fixed: an unseeded
#         scratch render always lost `sticky` and always reported STALE here.
FAKE_FIGURE=1 run --check
if [ "$ST" -eq 0 ] && echo "$OUT" | grep -q "is current"; then
    pass "--check on an untouched tree reports current (seeded owner-state survives)"
else fail "--check on an untouched tree reports current (seeded owner-state survives)" "$OUT (exit $ST)"; fi

# --- 3. --check is stable across repeated runs (not a coin flip). ----------
FAKE_FIGURE=1 run --check
if [ "$ST" -eq 0 ] && echo "$OUT" | grep -q "is current"; then
    pass "--check is stable on a second consecutive run"
else fail "--check is stable on a second consecutive run" "$OUT (exit $ST)"; fi

# --- 4. A genuinely stale committed copy (figure moved) IS caught. ---------
FAKE_FIGURE=2 run --check
if [ "$ST" -eq 1 ] && echo "$OUT" | grep -q "is STALE"; then
    pass "--check catches a genuinely stale committed copy"
else fail "--check catches a genuinely stale committed copy" "$OUT (exit $ST)"; fi

# --- 5. Re-running the real regen brings it back to current. ---------------
FAKE_FIGURE=2 run
FAKE_FIGURE=2 run --check
if [ "$ST" -eq 0 ] && echo "$OUT" | grep -q "is current"; then
    pass "a real regen after a stale finding restores --check to current"
else fail "a real regen after a stale finding restores --check to current" "$OUT (exit $ST)"; fi

# --- 6. --check never mutates the committed copy on disk. ------------------
BEFORE=$(md5sum "$COMMITTED" | awk '{print $1}')
FAKE_FIGURE=2 run --check
AFTER=$(md5sum "$COMMITTED" | awk '{print $1}')
if [ "$BEFORE" = "$AFTER" ]; then
    pass "--check does not mutate the committed copy on disk"
else fail "--check does not mutate the committed copy on disk" "before=$BEFORE after=$AFTER"; fi

echo "---------------------------------------------------------------"
echo "passed: $PASSED  failed: $FAILED"
[ "$FAILED" -eq 0 ] || { echo "SELF-TEST FAILED."; exit 1; }
echo "SELF-TEST PASSED."
