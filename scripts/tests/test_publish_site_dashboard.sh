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

# A stand-in for scripts/site/build_public_status.py. The script under test
# calls it on both paths (--check and a real run); the real one imports the
# observer package and reads the committed unit ledgers, neither of which
# exists in this hermetic fake repo. Same override pattern as FAKE_PRODUCER:
# this test covers publish-site-dashboard.sh's ORCHESTRATION, not the
# projection generator, which has its own suite in test_build_public_status.py.
FAKE_PUBLIC_STATUS="$WORKROOT/fake_build_public_status.py"
cat >"$FAKE_PUBLIC_STATUS" <<'PY_INNER'
"""Stand-in projection builder: honours --check, writes a marker otherwise."""
import os
import sys

out = os.path.join(os.getcwd(), "site", "status-data.json")
if "--check" in sys.argv:
    print("site/status-data.json is current")
else:
    os.makedirs(os.path.dirname(out), exist_ok=True)
    with open(out, "w") as handle:
        handle.write('{"stand_in": true}\n')
    print(f"wrote {out}")
sys.exit(0)
PY_INNER

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
    # Models the two real subtrees that drift with no commit involved:
    # `usage` (a live cache file OUTSIDE the repo) and `retrospective`
    # (docs/retro/events/*.jsonl, which every cycle appends to as normal
    # work). A --check comparison that does not drop these whole subtrees
    # reports STALE on every run regardless of FAKE_FIGURE -- the
    # wave-8-confirmed defect case 7 below guards against.
    "usage": {"session_used_pct": time.time_ns() % 100},
    "retrospective": {"total_events": time.time_ns() % 1000},
}
with open(args.out, "w") as f:
    json.dump(doc, f)
print(f"fake-producer: rendered {args.out}")
# AT-34-E6-001 wave-27: echoes PF1E_DASHBOARD_STRICT_TIMEOUT verbatim so the
# shell test below can prove --check sets it and a real (non---check) run
# does not -- the switch between "a timeout raises loudly" and "a timeout
# falls back to the stale cache", see pf1e_dashboard_producer.py's own
# `StateDumpTimeout` docstring for why the two paths must differ. Printed to
# stderr, not stdout: publish-site-dashboard.sh redirects the producer's
# stdout to /dev/null on both paths, so stdout can never reach this test's
# captured $OUT.
print(f"STRICT_TIMEOUT_ENV={os.environ.get('PF1E_DASHBOARD_STRICT_TIMEOUT', '<unset>')}", file=sys.stderr)
sys.exit(0)
PY

run() {
    OUT=$(cd "$FAKE_REPO" && PF1E_DASHBOARD_PRODUCER="$FAKE_PRODUCER" \
        PF1E_PUBLIC_STATUS_BUILDER="$FAKE_PUBLIC_STATUS" \
        ./scripts/publish-site-dashboard.sh "$@" 2>&1)
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

# --- 7. --check sets PF1E_DASHBOARD_STRICT_TIMEOUT=1 for the producer. -----
#         AT-34-E6-001 wave-27: --check is the ONLY caller allowed to turn a
#         state-dump timeout into a loud failure; this proves the plumbing
#         that opts it in, independent of the timeout-handling logic itself
#         (covered against the real producer's helpers by
#         test_pf1e_dashboard_producer.py's StateDumpTimeoutIsLoudUnderStrictModeTest).
FAKE_FIGURE=2 run --check
if echo "$OUT" | grep -q "STRICT_TIMEOUT_ENV=1"; then
    pass "--check sets PF1E_DASHBOARD_STRICT_TIMEOUT=1 for the producer"
else fail "--check sets PF1E_DASHBOARD_STRICT_TIMEOUT=1 for the producer" "$OUT (exit $ST)"; fi

# --- 8. A real (non---check) run leaves it unset -- the live-regen path ----
#         keeps its stale-cache-preferred fallback, so the public dashboard
#         never goes blank over one slow build.
FAKE_FIGURE=2 run
if echo "$OUT" | grep -q "STRICT_TIMEOUT_ENV=<unset>"; then
    pass "a real (non---check) run leaves PF1E_DASHBOARD_STRICT_TIMEOUT unset"
else fail "a real (non---check) run leaves PF1E_DASHBOARD_STRICT_TIMEOUT unset" "$OUT (exit $ST)"; fi

echo "---------------------------------------------------------------"
echo "passed: $PASSED  failed: $FAILED"
[ "$FAILED" -eq 0 ] || { echo "SELF-TEST FAILED."; exit 1; }
echo "SELF-TEST PASSED."
