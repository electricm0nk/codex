#!/usr/bin/env bash
# ---------------------------------------------------------------------------
# test_reclaim_orphan_targets.sh — regression self-test for reclaim.sh's
# orphaned-CARGO_TARGET_DIR coverage (the `codex-target-*` convention).
#
# Why this exists: on 2026-08-10 reclaim.sh reported "would reclaim: 0
# item(s), 0.0B" while ~40G of orphaned cargo output sat in
# $HOME/workspace/codex-target-* and /tmp/codex-target-* — the exact
# directories the current operating discipline tells every dispatched agent
# to create. The script scanned only the scratchpad root and ~/.cache, so
# the safety net had a hole exactly where the new discipline generates
# garbage. 43 of ~60 recorded incidents in docs/retro/events/*.jsonl are
# disk-full/disk-pressure/preflight-disk.
#
# The safety property under test is the one that nearly went wrong live:
# codex-target-sd29-monster-r1 had a 2-hour-stale mtime and looked
# abandoned, but its agent was alive and between builds. Deleting a live
# dir costs a 30+ minute cold rebuild. reclaim.sh must skip a dir that is
# (a) young, (b) held open by any live process, or (c) claimed via a
# `.reclaim-claim` file naming a live PID — and must still reclaim a dir
# with none of those signals.
#
# Every case runs against throwaway roots via --workspace-root /
# --orphan-tmp-root; nothing touches the real workspace or /tmp.
#
# Usage: bash scripts/tests/test_reclaim_orphan_targets.sh
# Exit 0 = all cases pass.
# ---------------------------------------------------------------------------
set -uo pipefail

RECLAIM="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)/scripts/reclaim.sh"
[ -f "$RECLAIM" ] || { echo "FATAL: reclaim.sh not found at $RECLAIM" >&2; exit 2; }

PASSED=0
FAILED=0

pass() { echo "  PASS  $1"; PASSED=$((PASSED + 1)); }
fail() { echo "  FAIL  $1"; FAILED=$((FAILED + 1)); }

# Builds a cargo-shaped target dir (CACHEDIR.TAG + debug/, matching
# is_cargo_target_dir) named $2 under root $1; backdates it $3 hours.
make_target() {
  local root="$1" name="$2" hours="${3:-0}"
  local d="$root/$name"
  mkdir -p "$d/debug"
  printf 'Signature: 8a477f597d28d172789f06886806bc55\n' > "$d/CACHEDIR.TAG"
  printf 'x%.0s' {1..1024} > "$d/debug/payload"
  if [ "$hours" != 0 ]; then
    local stamp
    stamp=$(date -d "-$hours hours" +%Y%m%d%H%M 2>/dev/null)
    find "$d" -exec touch -t "$stamp" {} +
  fi
  echo "$d"
}

# run_reclaim <workspace-root> <tmp-root> [extra args...]
run_reclaim() {
  local ws="$1" tmp="$2"; shift 2
  RETRO_DISABLE=1 bash "$RECLAIM" --only cargo-target \
    --scratchpad-root /nonexistent-scratch --cache-root /nonexistent-cache \
    --workspace-root "$ws" --orphan-tmp-root "$tmp" "$@" 2>&1
}

fresh_roots() {
  WS=$(mktemp -d); TMPROOT=$(mktemp -d)
}

echo "===== reclaim.sh — orphaned codex-target-* self-test ====="

# --- 1. Orphan in the workspace root is found and reported (dry run). -------
fresh_roots
d=$(make_target "$WS" codex-target-dead-actor 24)
out=$(run_reclaim "$WS" "$TMPROOT")
if echo "$out" | grep -q "WOULD REMOVE  $d"; then pass "workspace orphan reported in dry run"; else fail "workspace orphan reported in dry run"; fi
if [ -d "$d" ]; then pass "dry run deleted nothing"; else fail "dry run deleted nothing"; fi

# --- 2. --apply removes it. --------------------------------------------------
out=$(run_reclaim "$WS" "$TMPROOT" --apply)
if [ ! -d "$d" ] && echo "$out" | grep -q "REMOVED  $d"; then pass "workspace orphan removed with --apply"; else fail "workspace orphan removed with --apply"; fi
rm -rf "$WS" "$TMPROOT"

# --- 3. Orphan in the tmp root is found and removed. -------------------------
fresh_roots
d=$(make_target "$TMPROOT" codex-target-tmp-orphan 24)
run_reclaim "$WS" "$TMPROOT" --apply >/dev/null
if [ ! -d "$d" ]; then pass "tmp-root orphan removed with --apply"; else fail "tmp-root orphan removed with --apply"; fi
rm -rf "$WS" "$TMPROOT"

# --- 4. A young dir survives (the monster-r1 case: 2h stale, agent alive). ---
fresh_roots
d=$(make_target "$WS" codex-target-live-young 2)
out=$(run_reclaim "$WS" "$TMPROOT" --apply)
if [ -d "$d" ] && echo "$out" | grep -q "too young"; then pass "young dir (2h) skipped even with --apply"; else fail "young dir (2h) skipped even with --apply"; fi
rm -rf "$WS" "$TMPROOT"

# --- 5. A claim file naming a live PID protects an old dir. ------------------
fresh_roots
d=$(make_target "$WS" codex-target-claimed-live 24)
sleep 300 &
CLAIM_PID=$!
echo "$CLAIM_PID" > "$d/.reclaim-claim"
touch -t "$(date -d '-24 hours' +%Y%m%d%H%M)" "$d/.reclaim-claim" "$d"
out=$(run_reclaim "$WS" "$TMPROOT" --apply)
if [ -d "$d" ] && echo "$out" | grep -q "claimed by live pid"; then pass "old dir with live-PID claim skipped"; else fail "old dir with live-PID claim skipped"; fi
kill "$CLAIM_PID" 2>/dev/null; wait "$CLAIM_PID" 2>/dev/null
rm -rf "$WS" "$TMPROOT"

# --- 6. A claim file naming a dead PID does NOT protect it. ------------------
fresh_roots
d=$(make_target "$WS" codex-target-claimed-dead 24)
sleep 0.1 &
DEAD_PID=$!
wait "$DEAD_PID" 2>/dev/null
echo "$DEAD_PID" > "$d/.reclaim-claim"
touch -t "$(date -d '-24 hours' +%Y%m%d%H%M)" "$d/.reclaim-claim" "$d"
run_reclaim "$WS" "$TMPROOT" --apply >/dev/null
if [ ! -d "$d" ]; then pass "old dir with dead-PID claim removed"; else fail "old dir with dead-PID claim removed"; fi
rm -rf "$WS" "$TMPROOT"

# --- 7. An open file handle anywhere under the dir protects it. --------------
fresh_roots
d=$(make_target "$WS" codex-target-held-open 24)
tail -f "$d/debug/payload" >/dev/null 2>&1 &
HOLDER_PID=$!
sleep 0.3   # let tail actually open the file
touch -t "$(date -d '-24 hours' +%Y%m%d%H%M)" "$d" "$d/debug" "$d/debug/payload" "$d/CACHEDIR.TAG"
out=$(run_reclaim "$WS" "$TMPROOT" --apply)
if [ -d "$d" ] && echo "$out" | grep -q "open file handle"; then pass "old dir with open file handle skipped"; else fail "old dir with open file handle skipped"; fi
kill "$HOLDER_PID" 2>/dev/null; wait "$HOLDER_PID" 2>/dev/null
rm -rf "$WS" "$TMPROOT"

# --- 8. Only codex-target-* names are considered in these roots. -------------
fresh_roots
d=$(make_target "$WS" some-other-project-target 24)
run_reclaim "$WS" "$TMPROOT" --apply >/dev/null
if [ -d "$d" ]; then pass "non-codex-target-* name in workspace root untouched"; else fail "non-codex-target-* name in workspace root untouched"; fi
rm -rf "$WS" "$TMPROOT"

# --- 9. A codex-target-* dir that is not cargo-shaped is untouched. ----------
fresh_roots
d="$WS/codex-target-not-cargo"
mkdir -p "$d"
printf 'z\n' > "$d/notes.txt"
touch -t "$(date -d '-24 hours' +%Y%m%d%H%M)" "$d" "$d/notes.txt"
run_reclaim "$WS" "$TMPROOT" --apply >/dev/null
if [ -d "$d" ]; then pass "non-cargo-shaped codex-target-* dir untouched"; else fail "non-cargo-shaped codex-target-* dir untouched"; fi
rm -rf "$WS" "$TMPROOT"

# --- 10. A codex-target-* dir with real build output but NO CACHEDIR.TAG ----
# is still found and reported. This is the 2026-08-13 defect: five real,
# 8-50G codex-target-* dirs (debug/release output present, CACHEDIR.TAG
# absent — cargo does not write it on every run) were silently invisible.
# consider_cargo_target_dir()'s `! is_cargo_target_dir "$real"` branch used
# to `return` with no output line at all for exactly this shape, so this
# case failed red against the pre-fix code (dir survived --apply, no
# disposition line of any kind — not even a SKIP).
fresh_roots
d="$WS/codex-target-no-tag"
mkdir -p "$d/debug"
printf 'x%.0s' {1..1024} > "$d/debug/payload"
touch -t "$(date -d '-24 hours' +%Y%m%d%H%M)" "$d" "$d/debug" "$d/debug/payload"
out=$(run_reclaim "$WS" "$TMPROOT" --apply)
if [ ! -d "$d" ] && echo "$out" | grep -q "REMOVED  $d"; then pass "cargo-shaped dir with no CACHEDIR.TAG still found and removed"; else fail "cargo-shaped dir with no CACHEDIR.TAG still found and removed"; fi
rm -rf "$WS" "$TMPROOT"

# --- 11. Every codex-target-* candidate is enumerated in the summary, ------
# including the ones that get rejected as not-a-cargo-target — the
# "considered N" line makes silent invisibility structurally visible.
fresh_roots
make_target "$WS" codex-target-alpha 24 >/dev/null
mkdir -p "$WS/codex-target-empty-junk"
touch -t "$(date -d '-24 hours' +%Y%m%d%H%M)" "$WS/codex-target-empty-junk"
out=$(run_reclaim "$WS" "$TMPROOT")
if echo "$out" | grep -q "considered 2 codex-target-\* candidate(s)"; then pass "considered-count reports all name-matched candidates"; else fail "considered-count reports all name-matched candidates"; fi
if echo "$out" | grep -q "not a cargo target dir"; then pass "non-cargo candidate gets an explicit disposition line"; else fail "non-cargo candidate gets an explicit disposition line"; fi
rm -rf "$WS" "$TMPROOT"

echo "---------------------------------------------------------------"
echo "passed: $PASSED  failed: $FAILED"
[ "$FAILED" -eq 0 ] || { echo "SELF-TEST FAILED."; exit 1; }
echo "SELF-TEST PASSED."
