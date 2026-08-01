#!/usr/bin/env bash
# ---------------------------------------------------------------------------
# VENDORED 2026-07-27 from the wired-integration-discipline skill:
#   <hermes>/profiles/god-emporer/skills/devops/wired-integration-discipline/
#     scripts/audit-wired-integration.sh
#
# Vendored so the SD-27 bundle's cited verification command
# (`bash scripts/wired-integration-audit.sh`) resolves repo-locally instead of
# depending on a machine-local skills tree. Upstream remains the doctrine
# owner; re-sync from it rather than editing this copy in place for anything
# other than the fix below.
#
# NOT verbatim: fixed 2026-07-27 (independent multi-agent review) two pathspec
# defects present in the original upstream script, both confirmed live against
# this repo's actual files:
#   1. `**` requires `:(glob)` magic. Without it, git's default pathspec mode
#      treats `**` as needing at least one intermediate directory, so
#      `src/**/*.rs` never matched top-level files like `src/lib.rs` or
#      `apps/desktop/src-tauri/build.rs` (both real files in this repo).
#      Verified: a `STUB placeholder not yet implemented` string planted
#      directly in `src/lib.rs` passed all four checks clean.
#   2. `{ts,tsx,jsx,rs}` brace-alternation is not supported by git pathspec
#      matching AT ALL, glob magic or not — it silently matches zero files.
#      Check 3's sole positive pathspec used this form, so Check 3 was a
#      structural no-op for every file, always, regardless of `:(glob)`.
#      Fixed by expanding each brace list into separate literal patterns.
#   3. `:(glob)!pattern` (glob magic + shorthand negation) is also silently
#      wrong — it does not exclude. The correct combined form is
#      `:(exclude,glob)pattern`.
# Companion doctrine: docs/governance/no-stub-mvp-doctrine.md
# ---------------------------------------------------------------------------
# audit-wired-integration.sh — runs the four-check Wired Integration audit.
# Bundles under the wired-integration-discipline skill should `cp` this into
# their loop-instruction or invoke it directly per cycle. Exit codes:
#   0 — all four checks pass (clean)
#   1 — at least one check failed
#   2 — pre-condition failure (e.g. no git history, BASE_BRANCH unreadable)
#
# Usage:
#   BASE_BRANCH=develop ./scripts/audit-wired-integration.sh
#   ./scripts/audit-wired-integration.sh origin/develop
#
# Environment:
#   BASE_BRANCH — the canonical base branch (default: origin/develop).
#   AUDIT_EXCLUDE_FILES — colon-separated list of paths to exclude from
#     the diff (used by epics whose scope is in-flight cleanup).
#
# The audit output goes to stdout. Bundles should capture stdout into the
# kanban card's comments stream per kanban-claude-code-execution-receipt.

set -euo pipefail

BASE_BRANCH="${1:-${BASE_BRANCH:-origin/develop}}"

if ! git rev-parse --verify "$BASE_BRANCH" >/dev/null 2>&1; then
  echo "FATAL: BASE_BRANCH=$BASE_BRANCH is not resolvable." >&2
  exit 2
fi

EXCLUDE_ARGS=(
  ':(exclude,glob)**/__tests__/**'
  ':(exclude,glob)**/*.test.ts'
  ':(exclude,glob)**/*.test.tsx'
  ':(exclude,glob)**/*.test.rs'
)

FAIL=0

echo "===== Check 1 — forbidden tokens in shipping code ====="
if git diff --unified=0 "${BASE_BRANCH}...HEAD" \
    -- ':(glob)apps/desktop/**/*.ts*' ':(glob)apps/desktop/src-tauri/**/*.rs' ':(glob)src/**/*.rs' \
    "${EXCLUDE_ARGS[@]}" \
    | grep -nE '\b(STUB|MOCK|placeholder|not yet implemented|todo|fixme|hack)\b'; then
  echo "FAIL: forbidden token(s) above." >&2
  FAIL=1
else
  echo 'OK_NO_TOKENS'
fi

echo
echo "===== Check 2 — empty event handlers on user-facing affordances ====="
if git diff --unified=0 "${BASE_BRANCH}...HEAD" \
    -- ':(glob)apps/desktop/**/*.tsx' ':(glob)apps/desktop/**/*.jsx' \
    | grep -nE 'onClick=\{\s*\(\)\s*=>\s*\{\s*\}\s*\}|onClick=\{undefined'; then
  echo "FAIL: no-op handler(s) above." >&2
  FAIL=1
else
  echo 'OK_NO_NOOP_HANDLERS'
fi

echo
echo "===== Check 3 — mock libraries leaking into shipping code ====="
if git diff --unified=0 "${BASE_BRANCH}...HEAD" \
    -- ':(glob)apps/desktop/**/*.ts' ':(glob)apps/desktop/**/*.tsx' \
       ':(glob)apps/desktop/**/*.jsx' ':(glob)apps/desktop/**/*.rs' \
    "${EXCLUDE_ARGS[@]}" \
    | grep -nE 'mockResolvedValue|mockReturnValue\(|vi\.mock\(|__mocks__'; then
  echo "FAIL: mock leak(s) above." >&2
  FAIL=1
else
  echo 'OK_NO_MOCK_LEAKS'
fi

echo
echo "===== Check 4 — \"Would …\" return strings (canonical stub pattern) ====="
if git diff --unified=0 "${BASE_BRANCH}...HEAD" \
    -- ':(glob)apps/desktop/**/*.ts' ':(glob)apps/desktop/**/*.tsx' ':(glob)src/**/*.rs' \
    | grep -nE '"Would [^"]*"'; then
  echo "FAIL: stub-shaped return string(s) above." >&2
  FAIL=1
else
  echo 'OK_NO_WOULD_STRINGS'
fi

if [[ "$FAIL" -ne 0 ]]; then
  echo
  echo "AUDIT FAILED. Cycle cannot mark complete. Return to cycle-backlog with this output as the failure reason." >&2
  exit 1
fi

echo
echo "AUDIT PASSED. All four checks clean."
