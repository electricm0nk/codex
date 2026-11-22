#!/usr/bin/env bash
# Self-blocking beta->stable (test -> main) promotion gate (SD16-E6-F1).
#
# Evaluates the six doctrine gates and either:
#   - prints every failed gate as a "FAIL: <gate>: <reason>" line plus
#     STATUS=blocked and exits non-zero, or
#   - prints the AV-BR-6 evidence PR body plus beta_release_url: and
#     provenance_url: (stdout, or --body-out <path>) plus STATUS=ready and
#     exits zero.
#
# This slice never calls `gh pr create`; a later wiring step consumes the body.
#
# usage: promote-beta-to-stable.sh \
#          --test-sha <sha> \
#          --test-version <semver> \
#          --beta-verification-evidence <url-or-path> \
#          --shell-consumed-beta-evidence <url-or-path> \
#          [--alpha-release-url <url>] \
#          [--body-out <path>]
#
# env overrides (used by the headless test): RELEASE_NOTES_PATH,
# TRANCHE_MANIFEST_PATH.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=scripts/release/_lib-gates.sh
source "$SCRIPT_DIR/_lib-gates.sh"

TEST_SHA=""
TEST_VERSION=""
BETA_VERIFICATION_EVIDENCE=""
SHELL_EVIDENCE=""
ALPHA_RELEASE_URL="not-recorded (pass --alpha-release-url to record the originating alpha release)"
BODY_OUT=""

while [ $# -gt 0 ]; do
  case "$1" in
    --test-sha) TEST_SHA="${2:-}" ;;
    --test-version) TEST_VERSION="${2:-}" ;;
    --beta-verification-evidence) BETA_VERIFICATION_EVIDENCE="${2:-}" ;;
    --shell-consumed-beta-evidence) SHELL_EVIDENCE="${2:-}" ;;
    --alpha-release-url) ALPHA_RELEASE_URL="${2:-}" ;;
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

# G_BETA_RELEASE_EXISTS -- a published (non-draft) beta release for the
# test candidate must exist.
BETA_TAG="beta/${TEST_VERSION}-${TEST_SHA}"
BETA_RELEASE_URL=""
if [ -z "$TEST_SHA" ] || [ -z "$TEST_VERSION" ]; then
  fail_gate G_BETA_RELEASE_EXISTS "--test-sha and --test-version are required"
elif ! BETA_RELEASE_URL="$(release_url_for_tag "$BETA_TAG")"; then
  fail_gate G_BETA_RELEASE_EXISTS "no published (non-draft) release for tag $BETA_TAG"
fi

# G_BETA_VERIFIED -- CI run URL or local log proving end-to-end beta
# verification (e.g. the SD16-E7 receipt).
if ! is_valid_evidence "$BETA_VERIFICATION_EVIDENCE"; then
  fail_gate G_BETA_VERIFIED "--beta-verification-evidence must be a non-empty URL or existing file path (got: '${BETA_VERIFICATION_EVIDENCE}')"
fi

# G_NO_STABLE_BLOCKERS -- only "- [stable-blocker]" lines block the stable
# promotion; beta-lane-only "- [blocker]" items pass this gate.
if [ -f "$NOTES_FILE" ] && known_issues_has_marker "$NOTES_FILE" '^- \[stable-blocker\]'; then
  fail_gate G_NO_STABLE_BLOCKERS "stable-blocking known issue found (- [stable-blocker] line in ## Known Issues)"
fi

# G_NOTES_VALID -- same release-notes validator as the alpha lane.
NOTES_PROBLEMS="$(validate_release_notes "$NOTES_FILE" || true)"
if [ -n "$NOTES_PROBLEMS" ]; then
  while IFS= read -r problem; do
    fail_gate G_NOTES_VALID "$problem"
  done <<< "$NOTES_PROBLEMS"
fi

# G_SHELL_CONSUMED_BETA -- receipt proving the shell fetched and installed a
# beta update.
if ! is_valid_evidence "$SHELL_EVIDENCE"; then
  fail_gate G_SHELL_CONSUMED_BETA "--shell-consumed-beta-evidence must be a non-empty URL or existing file path (got: '${SHELL_EVIDENCE}')"
fi

# G_PROVENANCE_VALID -- provenance.json downloaded from the beta release must
# parse as JSON with repo, workflow, run_id, source_commit.
PROVENANCE_URL=""
if [ -n "$BETA_RELEASE_URL" ]; then
  PROVENANCE_DIR="$(mktemp -d)"
  trap 'rm -rf "$PROVENANCE_DIR"' EXIT
  if ! gh release download "$BETA_TAG" --pattern provenance.json --dir "$PROVENANCE_DIR" 2> /dev/null; then
    fail_gate G_PROVENANCE_VALID "could not download provenance.json from release $BETA_TAG"
  elif ! jq -e 'has("repo") and has("workflow") and has("run_id") and has("source_commit")' \
    "$PROVENANCE_DIR/provenance.json" > /dev/null 2>&1; then
    fail_gate G_PROVENANCE_VALID "provenance.json from $BETA_TAG must be JSON containing repo, workflow, run_id, source_commit"
  else
    PROVENANCE_URL="$(sed 's|/releases/tag/|/releases/download/|' <<< "$BETA_RELEASE_URL")/provenance.json"
  fi
else
  fail_gate G_PROVENANCE_VALID "cannot verify provenance: beta release unavailable"
fi

report_gate_outcome

BODY="$(emit_pr_body alpha_release_url "$ALPHA_RELEASE_URL" "$BETA_VERIFICATION_EVIDENCE" "$NOTES_FILE")
shell_consumed_beta_evidence: ${SHELL_EVIDENCE}
beta_release_url: ${BETA_RELEASE_URL}
provenance_url: ${PROVENANCE_URL}"
deliver_body "$BODY" "$BODY_OUT"
