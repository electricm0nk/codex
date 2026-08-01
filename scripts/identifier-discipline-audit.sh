#!/usr/bin/env bash
# ---------------------------------------------------------------------------
# identifier-discipline-audit.sh — the identifier-discipline half of the
# per-cycle dual-audit gate.
#
# NOT vendored: the identifier-discipline skill is inline-only and ships no
# script. This is authored from the form SD-26 actually ran and recorded as
# passing, at:
#   docs/release/SD-26-ingest-strategy-and-rule-system-plumbing/
#     loop-instruction.md §6 (the `OK_NO_BUNDLE_TAGS` grep)
# Its receipt at artifacts/epic_6/final-criterion-scan-cycle_receipt.md
# records the `OK_NO_BUNDLE_TAGS` result token this script reproduces.
#
# The audit is diff-scoped, not tree-scoped: it flags bundle identifiers
# newly introduced by the cycle, not pre-existing ones. Tests are excluded —
# a test may legitimately name the bundle slice it covers.
#
# Exit codes:
#   0 — clean (prints OK_NO_BUNDLE_TAGS)
#   1 — at least one violation (offending lines printed)
#   2 — pre-condition failure (BASE_BRANCH unresolvable)
#
# Usage:
#   bash scripts/identifier-discipline-audit.sh
#   BASE_BRANCH=develop bash scripts/identifier-discipline-audit.sh
#   bash scripts/identifier-discipline-audit.sh origin/develop
#
# Environment:
#   BASE_BRANCH — canonical base branch (default: origin/develop).
#
# Per-bundle exclusion: a release bundle's own docs may carry its own slug
# (e.g. `SD-27-`) as a naming convention. Only shipping source is scanned,
# so docs/release/** is out of scope by construction.
# Companion doctrine: skill `identifier-discipline`.
# ---------------------------------------------------------------------------
set -euo pipefail

BASE_BRANCH="${1:-${BASE_BRANCH:-origin/develop}}"

if ! git rev-parse --verify "$BASE_BRANCH" >/dev/null 2>&1; then
  echo "FATAL: BASE_BRANCH=$BASE_BRANCH is not resolvable." >&2
  exit 2
fi

echo "===== Identifier-discipline audit — bundle tags in shipping code ====="

# Patterns, per the identifier-discipline doctrine:
#   sd<N>_ / SD<N>_ / Sd<N>  — bundle-tagged source identifiers
#   t_<hex8+>                — kanban card tokens
#
# The trailing `\b` must sit at the END of the full identifier, not right after
# the mandatory underscore — `_` is a word character, so `\bsd[0-9]+_\b` can only
# match a bare `sd27_` immediately followed by a non-word char, which never
# happens for a real identifier like `sd27_gen_book_cache` (found live 2026-07-27:
# this exact bug let `sd27_bundle_flag`/`SD27_BOOK_ID` pass the gate clean).
# :(glob) magic is required on every pattern containing `**`. Without it, git's
# default pathspec mode treats `**` as requiring at least one intermediate
# directory, so `src/**/*.rs` silently never matches a top-level file like
# src/lib.rs — found live 2026-07-27: a bundle tag planted directly in the
# real src/lib.rs (which exists in this repo right now) passed this gate clean.
if git diff --unified=0 "${BASE_BRANCH}...HEAD" -- \
    ':(glob)apps/desktop/**/*.ts*' ':(glob)apps/desktop/src-tauri/**/*.rs' ':(glob)src/**/*.rs' \
    ':(exclude,glob)**/__tests__/**' ':(exclude,glob)**/*.test.*' \
    | grep -nE '\b(sd[0-9]+_[A-Za-z0-9_]+|SD[0-9]+_[A-Za-z0-9_]+|Sd[0-9]+[A-Za-z0-9_]*|t_[0-9a-f]{8,})\b'; then
  echo >&2
  echo "FAIL: bundle identifier(s) above leaked into shipping code." >&2
  echo "AUDIT FAILED. Cycle cannot mark complete." >&2
  exit 1
fi

echo 'OK_NO_BUNDLE_TAGS'
