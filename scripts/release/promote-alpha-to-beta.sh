#!/usr/bin/env bash
# Self-blocking alpha->beta (develop -> test) promotion gate (SD16-E6-F1).
#
# Evaluates the five doctrine gates and either:
#   - prints every failed gate as a "FAIL: <gate>: <reason>" line plus
#     STATUS=blocked and exits non-zero, or
#   - prints the AV-BR-6 evidence PR body (stdout, or --body-out <path>) plus
#     STATUS=ready and exits zero.
#
# This slice never calls `gh pr create`; a later wiring step consumes the body.
#
# usage: promote-alpha-to-beta.sh \
#          --develop-sha <sha> \
#          --develop-version <semver> \
#          --shell-consumed-alpha-evidence <url-or-path> \
#          [--body-out <path>]
#
# env overrides (used by the headless test): RELEASE_NOTES_PATH,
# TRANCHE_MANIFEST_PATH.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/release/_lib-gates.sh
source "$SCRIPT_DIR/_lib-gates.sh"

DEVELOP_SHA=""
DEVELOP_VERSION=""
SHELL_EVIDENCE=""
BODY_OUT=""

while [ $# -gt 0 ]; do
  case "$1" in
    --develop-sha) DEVELOP_SHA="${2:-}" ;;
    --develop-version) DEVELOP_VERSION="${2:-}" ;;
    --shell-consumed-alpha-evidence) SHELL_EVIDENCE="${2:-}" ;;
    --body-out) BODY_OUT="${2:-}" ;;
    *)
      echo "FAIL: usage: unknown argument: $1"
      echo "STATUS=blocked"
      exit 2
      ;;
  esac
  shift $(($# > 1 ? 2 : 1))
done

NOTES_FILE="${RELEASE_NOTES_PATH:-$RELEASE_NOTES_REPO_PATH}"
MANIFEST_FILE="${TRANCHE_MANIFEST_PATH:-$DEFAULT_TRANCHE_MANIFEST_PATH}"

# G_ALPHA_RELEASE_EXISTS -- a published (non-draft) alpha release for the
# develop candidate must exist.
ALPHA_TAG="alpha/${DEVELOP_VERSION}-${DEVELOP_SHA}"
ALPHA_RELEASE_URL=""
if [ -z "$DEVELOP_SHA" ] || [ -z "$DEVELOP_VERSION" ]; then
  fail_gate G_ALPHA_RELEASE_EXISTS "--develop-sha and --develop-version are required"
elif ! ALPHA_RELEASE_URL="$(release_url_for_tag "$ALPHA_TAG")"; then
  fail_gate G_ALPHA_RELEASE_EXISTS "no published (non-draft) release for tag $ALPHA_TAG"
fi

# G_NOTES_VALIDATED -- tranche release notes parse with every doctrine
# section present, ordered, non-empty, and no channel: field.
NOTES_PROBLEMS="$(validate_release_notes "$NOTES_FILE" || true)"
if [ -n "$NOTES_PROBLEMS" ]; then
  while IFS= read -r problem; do
    fail_gate G_NOTES_VALIDATED "$problem"
  done <<< "$NOTES_PROBLEMS"
fi

# G_MANIFEST_VALID -- notes must exist and the tranche manifest, when it
# exists, must parse with the right tranche_id and a release_notes_path.
# A missing manifest is E1-F1's pending deliverable: warn loudly, proceed.
if [ ! -f "$NOTES_FILE" ]; then
  fail_gate G_MANIFEST_VALID "release notes not found at $NOTES_FILE"
fi
if [ ! -f "$MANIFEST_FILE" ]; then
  warn "tranche manifest not found at $MANIFEST_FILE (E1-F1 deliverable pending); proceeding without it"
elif ! jq -e --arg tranche "$TRANCHE_ID" \
  '.tranche_id == $tranche and (.release_notes_path | type == "string" and length > 0)' \
  "$MANIFEST_FILE" > /dev/null 2>&1; then
  fail_gate G_MANIFEST_VALID "manifest at $MANIFEST_FILE must be JSON with tranche_id == \"$TRANCHE_ID\" and release_notes_path present"
fi

# G_NO_RELEASE_BLOCKERS -- no "- [blocker]" line in Known Issues.
if [ -f "$NOTES_FILE" ] && known_issues_has_marker "$NOTES_FILE" '^- \[blocker\]'; then
  fail_gate G_NO_RELEASE_BLOCKERS "release-blocking known issue found (- [blocker] line in ## Known Issues)"
fi

# G_SHELL_CONSUMED_ALPHA -- receipt proving the shell fetched and installed
# an alpha update (E2/E6/E7 lane).
if ! is_valid_evidence "$SHELL_EVIDENCE"; then
  fail_gate G_SHELL_CONSUMED_ALPHA "--shell-consumed-alpha-evidence must be a non-empty URL or existing file path (got: '${SHELL_EVIDENCE}')"
fi

report_gate_outcome

BODY="$(emit_pr_body alpha_release_url "$ALPHA_RELEASE_URL" "$SHELL_EVIDENCE" "$NOTES_FILE")"
deliver_body "$BODY" "$BODY_OUT"
