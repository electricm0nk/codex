#!/usr/bin/env bash
# Shared gate helpers for the SD16-E6-F1 promotion scripts.
# Sourced by promote-alpha-to-beta.sh and promote-beta-to-stable.sh.
#
# Requires: bash, gh, jq, grep, awk, sed. No third-party dependencies.

# Canonical repo-relative release-notes path. The PR body always records this
# doctrine path (AV-BR-6); RELEASE_NOTES_PATH only overrides which file the
# validator reads (used by the headless test fixtures).
RELEASE_NOTES_REPO_PATH="programs/codex/requirements/SD-16-feedback-loop-and-self-update-hardening/release-notes.md"
TRANCHE_ID="codex-tranche-2-5"
DEFAULT_TRANCHE_MANIFEST_PATH="programs/codex/requirements/SD-16-feedback-loop-and-self-update-hardening/tranche-manifest.json"

# Required release-notes section headers, in doctrine order.
REQUIRED_NOTE_SECTIONS=(
  "## Summary"
  "## User-Visible Changes"
  "## Defects Fixed"
  "## Operational Notes"
  "## Verification Evidence"
  "## Known Issues"
  "## Update Eligibility"
)

GATE_FAILURES=()

fail_gate() { # <gate-name> <reason>
  GATE_FAILURES+=("FAIL: $1: $2")
}

warn() {
  echo "WARN: $*" >&2
}

# Print all recorded failures (one per line) plus STATUS=blocked and exit 1.
# No-op when every gate passed.
report_gate_outcome() {
  if [ "${#GATE_FAILURES[@]}" -gt 0 ]; then
    printf '%s\n' "${GATE_FAILURES[@]}"
    echo "STATUS=blocked"
    if [ -n "${GITHUB_OUTPUT:-}" ]; then
      echo "status=blocked" >> "$GITHUB_OUTPUT"
    fi
    exit 1
  fi
}

# Print the body of one "## <name>" section: the lines strictly between the
# exact header line and the next "## " header (or EOF).
extract_section() { # <file> <exact-header-line>
  awk -v header="$2" '
    $0 == header { in_section = 1; next }
    /^## / { in_section = 0 }
    in_section { print }
  ' "$1"
}

# Validate the tranche release-notes file: every required section present, in
# order, non-empty; no `channel:` field anywhere. Prints one reason per
# problem; returns non-zero if any problem was found.
validate_release_notes() { # <file>
  local file="$1" problems=0 header line prev_line=0 body
  if [ ! -f "$file" ]; then
    echo "release notes not found at $file"
    return 1
  fi
  for header in "${REQUIRED_NOTE_SECTIONS[@]}"; do
    line="$(grep -nxF "$header" "$file" | head -1 | cut -d: -f1 || true)"
    if [ -z "$line" ]; then
      echo "missing section: $header"
      problems=$((problems + 1))
      continue
    fi
    if [ "$line" -le "$prev_line" ]; then
      echo "section out of order: $header"
      problems=$((problems + 1))
    fi
    prev_line="$line"
    body="$(extract_section "$file" "$header" | tr -d '[:space:]')"
    if [ -z "$body" ]; then
      echo "empty section: $header"
      problems=$((problems + 1))
    fi
  done
  if grep -qiE '^[[:space:]]*channel:' "$file"; then
    echo "forbidden 'channel:' field present (channel is derived from the promotion lane, not the notes)"
    problems=$((problems + 1))
  fi
  [ "$problems" -eq 0 ]
}

# Return 0 if the Known Issues section has a line matching the marker regex
# (case-insensitive), e.g. '^- \[blocker\]' or '^- \[stable-blocker\]'.
known_issues_has_marker() { # <file> <marker-regex>
  extract_section "$1" "## Known Issues" | grep -qiE "$2"
}

# Look up a published release by tag and print its URL.
# Returns non-zero if the release is missing or still a draft.
release_url_for_tag() { # <tag>
  local json
  json="$(gh release view "$1" --json tagName,isDraft,url)" || return 1
  [ "$(jq -r '.isDraft' <<< "$json")" = "false" ] || return 1
  jq -r '.url' <<< "$json"
}

# Evidence must be a non-empty http(s) URL or an existing file path.
is_valid_evidence() { # <url-or-path>
  local evidence="$1"
  [ -n "$evidence" ] || return 1
  case "$evidence" in
    http://* | https://*) return 0 ;;
  esac
  [ -f "$evidence" ]
}

# Emit the AV-BR-6 evidence body. Callers append lane-specific extra keys
# (beta_release_url:, provenance_url:) after this block.
emit_pr_body() { # <release-url-key> <release-url> <verification-evidence> <notes-file>
  cat << EOF
tranche_id: ${TRANCHE_ID}
release_notes_path: ${RELEASE_NOTES_REPO_PATH}
$1: $2
verification_evidence: $3
known_issues:
$(extract_section "$4" "## Known Issues" | grep -E '^-' || echo "- none listed")
EOF
}

# Print the body to stdout, or to the --body-out path when one was given,
# then declare STATUS=ready.
deliver_body() { # <body> <body-out-path-or-empty>
  if [ -n "$2" ]; then
    printf '%s\n' "$1" > "$2"
  else
    printf '%s\n' "$1"
  fi
  echo "STATUS=ready"
  if [ -n "${GITHUB_OUTPUT:-}" ]; then
    echo "status=ready" >> "$GITHUB_OUTPUT"
  fi
}
