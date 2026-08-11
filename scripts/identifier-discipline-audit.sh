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
#   sd<N>-                   — bundle-tagged hyphenated names (CSS classes,
#                              data-testids, string keys). Named by SD-29
#                              epic-breakdown.md SD29-E1-F1 alongside the other
#                              three, but absent from this regex until
#                              2026-08-10; the gate passed `"sd29-monster-row"`
#                              clean. NOT the same as the doc slug `SD-29-`
#                              (hyphen BEFORE the digits), which source comments
#                              legitimately cite and this pattern cannot match.
#                              The segment after the hyphen must start with a
#                              LOWERCASE letter: this repo's doc comments cite
#                              epic labels (`SD13-E3`, `SD28-E14-F1`) 777 times
#                              in shipping source (re-derived 2026-08-10), and
#                              those are citations, not identifiers. Uppercase
#                              after the hyphen ⇒ citation ⇒ not flagged.
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
# Both tag families are banned (operator directive 2026-08-11, landed by SD-29
# card `epic-1b-naming-sweep`): the SD-NN release-bundle family AND the GE-NN
# grand-epic family. Epic 1's hardened regex covered only SD, so the whole
# `Ge08*` type family, `ge08_workbench`, and `GE06_BASE_ARMOR_CLASS_...` passed
# it clean.
#
# `(^|[^A-Za-z0-9_])(<word>_)*` is the INFIX prefix: `_` is a word character, so
# a leading `\b` can never match the tag in `kind_is_sd17_b3`,
# `build_ge08_workbench_snapshot`, or `seeded_sd13_e1_f1_current_truth` — all
# three are live in this repo and all three passed Epic 1's gate clean.
TAG_INFIX='(^|[^A-Za-z0-9_])([A-Za-z0-9]+_)*'
TAG_BODY='(sd[0-9]+_[A-Za-z0-9_]+|SD[0-9]+_[A-Za-z0-9_]+|Sd[0-9]+[A-Za-z0-9_]*|ge[0-9]+_[A-Za-z0-9_]+|GE[0-9]+_[A-Za-z0-9_]+|Ge[0-9]+[A-Za-z0-9_]*|[SsGg][DdEe][0-9]+-[a-z][A-Za-z0-9-]*|t_[0-9a-f]{8,})'
TAG_RE="${TAG_INFIX}${TAG_BODY}\\b"

SHIPPING_PATHSPEC=(
  ':(glob)apps/desktop/**/*.ts*' ':(glob)apps/desktop/src-tauri/**/*.rs' ':(glob)src/**/*.rs'
  ':(exclude,glob)**/__tests__/**' ':(exclude,glob)**/*.test.*'
)

# Documented exclusion class (identifier-discipline doctrine, SD-25 1.1): a doc
# comment or string literal citing a REAL `tests/...` file by name is
# test-traceability grounding, not an identifier carrying a bundle tag. Only the
# identifier itself is a violation. Strip such citations before matching — this
# repo carries hundreds of them (`src/rules_core/support_state_matrix.rs` alone
# holds 319 tag-shaped hits, nearly all of this class), and a gate that flags
# them is a gate every cycle learns to ignore.
strip_test_citations() { sed -E 's#\btests/[A-Za-z0-9_./-]*##g'; }

# ADDED lines only. This script's own header says it flags identifiers "newly
# introduced by the cycle"; a `-` line is a tag being REMOVED, which is the cure,
# not the disease. Scanning the whole diff made the gate fail the one cycle whose
# entire purpose was deleting tags (SD-29 `epic-1b-naming-sweep`, 2026-08-11) —
# every rename it landed reappeared as a violation on its own `-` lines. `+++`
# file headers are dropped here; path tags get their own check below.
added_lines_only() { grep -E '^\+' | grep -vE '^\+\+\+'; }

if git diff --unified=0 "${BASE_BRANCH}...HEAD" -- "${SHIPPING_PATHSPEC[@]}" \
    | added_lines_only \
    | strip_test_citations \
    | grep -nE "$TAG_RE"; then
  echo >&2
  echo "FAIL: bundle identifier(s) above leaked into shipping code." >&2
  echo "AUDIT FAILED. Cycle cannot mark complete." >&2
  exit 1
fi

# ---------------------------------------------------------------------------
# PATH tags — file and directory names.
#
# Epic 1 hardened the identifier regex but the regex is identifier-shaped and
# caught NO path tags at all: `src/bin/sd27_gen_book_cache.rs`,
# `apps/desktop/src/sd16/update/`, and `apps/desktop/src-tauri/src/ge08_workbench.rs`
# all passed clean. The doctrine's headline covers the artifact, not only the
# symbol: a FILE or DIRECTORY named for its release bundle is the same
# violation. Only paths ADDED or RENAMED by the diff are scanned (`--diff-filter=AR`),
# so a cycle is never blocked by a pre-existing path it did not touch.
# ---------------------------------------------------------------------------
TAGGED_PATHS="$(
  git diff --name-only --diff-filter=AR "${BASE_BRANCH}...HEAD" -- "${SHIPPING_PATHSPEC[@]}" \
    | grep -E "(^|/)([A-Za-z0-9]+_)*(sd|SD|Sd|ge|GE|Ge)[0-9]+([_-][A-Za-z0-9_.-]*)?(/|\$|\.)" || true
)"
if [ -n "$TAGGED_PATHS" ]; then
  printf '%s\n' "$TAGGED_PATHS"
  echo >&2
  echo "FAIL: bundle-tagged file/directory name(s) above added by this diff." >&2
  echo "Name files and directories for WHAT they do, not which bundle shipped them." >&2
  echo "AUDIT FAILED. Cycle cannot mark complete." >&2
  exit 1
fi

echo 'OK_NO_BUNDLE_TAGS'
