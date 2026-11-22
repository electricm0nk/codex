#!/usr/bin/env bash
# Bash test for the promotion-gate scripts (SD16-E6-F1).
#
# Self-executing: run via `bash scripts/release/test-promotion-gates.test.sh`.
# Exits non-zero on the first failed assertion. Not discovered by
# apps/desktop/scripts/run-tests.mjs (which only globs src/**/*.test.ts).
#
# gh is never hit for real: a stub gh is placed first on PATH, driven by
# FAKE_GH_* env vars, and every invocation is appended to FAKE_GH_LOG so the
# suite can prove no script ever called `gh pr create`.
set -u

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ALPHA="$REPO_ROOT/scripts/release/promote-alpha-to-beta.sh"
BETA="$REPO_ROOT/scripts/release/promote-beta-to-stable.sh"
LIB="$REPO_ROOT/scripts/release/_lib-gates.sh"

for required in "$ALPHA" "$BETA" "$LIB"; do
  if [ ! -f "$required" ]; then
    echo "FAIL: $required not found"
    exit 1
  fi
done

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

# --- assertion helpers -------------------------------------------------------

assert_contains() { # <needle> <haystack> <label>
  case "$2" in
    *"$1"*) echo "PASS: $3" ;;
    *)
      echo "FAIL: $3: output does not contain '$1'"
      echo "--- output was ---"
      echo "$2"
      exit 1
      ;;
  esac
}

assert_not_contains() { # <needle> <haystack> <label>
  case "$2" in
    *"$1"*)
      echo "FAIL: $3: output unexpectedly contains '$1'"
      echo "--- output was ---"
      echo "$2"
      exit 1
      ;;
    *) echo "PASS: $3" ;;
  esac
}

assert_rc() { # <expected: zero|nonzero> <actual-rc> <label>
  if [ "$1" = "zero" ] && [ "$2" -ne 0 ]; then
    echo "FAIL: $3: expected exit 0, got $2"
    exit 1
  fi
  if [ "$1" = "nonzero" ] && [ "$2" -eq 0 ]; then
    echo "FAIL: $3: expected non-zero exit, got 0"
    exit 1
  fi
  echo "PASS: $3"
}

run() { # <cmd...> -> sets OUT and RC
  OUT="$("$@" 2>&1)"
  RC=$?
}

# --- gh stub -----------------------------------------------------------------

mkdir -p "$TMP/bin"
cat > "$TMP/bin/gh" << 'STUB'
#!/usr/bin/env bash
echo "$*" >> "${FAKE_GH_LOG:-/dev/null}"
case "${1:-} ${2:-}" in
  "release view")
    if [ "${FAKE_GH_VIEW_FAIL:-0}" = "1" ]; then
      echo "release not found" >&2
      exit 1
    fi
    printf '%s\n' "${FAKE_GH_VIEW_JSON:?}"
    ;;
  "release download")
    if [ "${FAKE_GH_DOWNLOAD_FAIL:-0}" = "1" ]; then
      echo "no assets match the pattern" >&2
      exit 1
    fi
    dir="."
    prev=""
    for arg in "$@"; do
      if [ "$prev" = "--dir" ]; then dir="$arg"; fi
      prev="$arg"
    done
    cp "${FAKE_GH_PROVENANCE_SRC:?}" "$dir/provenance.json"
    ;;
  *)
    echo "stub gh: unexpected invocation: $*" >&2
    exit 64
    ;;
esac
STUB
chmod +x "$TMP/bin/gh"
export PATH="$TMP/bin:$PATH"
export FAKE_GH_LOG="$TMP/gh-calls.log"
: > "$FAKE_GH_LOG"

# --- fixtures ----------------------------------------------------------------

write_notes() { # <path> <known-issues-body...>
  local path="$1"
  shift
  {
    cat << 'HEAD'
# Release Notes: SD-16 Fixture

## Summary

Fixture summary body.

## User-Visible Changes

- A visible change.

## Defects Fixed

- A fixed defect.

## Operational Notes

- An operational note.

## Verification Evidence

- Fixture evidence line.

## Known Issues

HEAD
    printf '%s\n' "$@"
    cat << 'TAIL'

## Update Eligibility

- Fixture eligibility statement.
TAIL
  } > "$path"
}

GOOD_NOTES="$TMP/notes-good.md"
write_notes "$GOOD_NOTES" "- Minor cosmetic flicker in the updater panel."

NOTES_MISSING_SECTION="$TMP/notes-missing-section.md"
write_notes "$NOTES_MISSING_SECTION" "- Minor issue."
sed -i '/^## Update Eligibility$/,$d' "$NOTES_MISSING_SECTION"

NOTES_WITH_BLOCKER="$TMP/notes-blocker.md"
write_notes "$NOTES_WITH_BLOCKER" "- [Blocker] Crash on first update check."

NOTES_WITH_STABLE_BLOCKER="$TMP/notes-stable-blocker.md"
write_notes "$NOTES_WITH_STABLE_BLOCKER" "- [stable-blocker] Data loss on relaunch."

# Beta-only blocker: must block alpha->beta promotion, but NOT beta->stable.
NOTES_WITH_BETA_ONLY_BLOCKER="$TMP/notes-beta-only-blocker.md"
write_notes "$NOTES_WITH_BETA_ONLY_BLOCKER" "- [blocker] Beta-lane-only issue."

GOOD_MANIFEST="$TMP/tranche-manifest.json"
cat > "$GOOD_MANIFEST" << 'EOF'
{
  "tranche_id": "codex-tranche-2-5",
  "release_notes_path": "docs/release/SD-16/release-notes.md"
}
EOF

BAD_MANIFEST="$TMP/tranche-manifest-bad.json"
printf '{"tranche_id": "wrong-tranche"}\n' > "$BAD_MANIFEST"

EVIDENCE_FILE="$TMP/shell-consumed-evidence.txt"
printf 'https://github.com/electricm0nk/codex/actions/runs/999 receipt\n' > "$EVIDENCE_FILE"

GOOD_PROVENANCE="$TMP/provenance-good.json"
cat > "$GOOD_PROVENANCE" << 'EOF'
{
  "repo": "electricm0nk/codex",
  "workflow": "publish-tester-release",
  "run_id": "12345",
  "source_commit": "def5678"
}
EOF

BAD_PROVENANCE="$TMP/provenance-bad.json"
printf '{"repo": "electricm0nk/codex", "workflow": "publish-tester-release"}\n' > "$BAD_PROVENANCE"

ALPHA_VIEW_JSON='{"tagName":"alpha/0.5.0-abc1234","isDraft":false,"url":"https://github.com/electricm0nk/codex/releases/tag/alpha%2F0.5.0-abc1234"}'
BETA_VIEW_JSON='{"tagName":"beta/0.5.0-def5678","isDraft":false,"url":"https://github.com/electricm0nk/codex/releases/tag/beta%2F0.5.0-def5678"}'

run_alpha() { # <notes> <manifest> <evidence> [extra args...]
  local notes="$1" manifest="$2" evidence="$3"
  shift 3
  RELEASE_NOTES_PATH="$notes" TRANCHE_MANIFEST_PATH="$manifest" \
    run bash "$ALPHA" \
    --develop-sha abc1234 \
    --develop-version 0.5.0 \
    --shell-consumed-alpha-evidence "$evidence" \
    "$@"
}

run_beta() { # <notes> <beta-evidence> <shell-evidence> [extra args...]
  local notes="$1" beta_evidence="$2" shell_evidence="$3"
  shift 3
  RELEASE_NOTES_PATH="$notes" TRANCHE_MANIFEST_PATH="$GOOD_MANIFEST" \
    run bash "$BETA" \
    --test-sha def5678 \
    --test-version 0.5.0 \
    --beta-verification-evidence "$beta_evidence" \
    --shell-consumed-beta-evidence "$shell_evidence" \
    "$@"
}

export FAKE_GH_VIEW_JSON="$ALPHA_VIEW_JSON"
export FAKE_GH_PROVENANCE_SRC="$GOOD_PROVENANCE"

# --- alpha gate failures (each gate individually) -----------------------------

echo "== alpha: G_ALPHA_RELEASE_EXISTS fails =="
FAKE_GH_VIEW_FAIL=1 run_alpha "$GOOD_NOTES" "$GOOD_MANIFEST" "$EVIDENCE_FILE"
assert_rc nonzero "$RC" "alpha blocked exit when alpha release missing"
assert_contains "FAIL: G_ALPHA_RELEASE_EXISTS" "$OUT" "alpha FAIL line names G_ALPHA_RELEASE_EXISTS"
assert_contains "STATUS=blocked" "$OUT" "alpha prints STATUS=blocked when release missing"
assert_not_contains "STATUS=ready" "$OUT" "alpha does not print STATUS=ready when release missing"

echo "== alpha: G_NOTES_VALIDATED fails =="
run_alpha "$NOTES_MISSING_SECTION" "$GOOD_MANIFEST" "$EVIDENCE_FILE"
assert_rc nonzero "$RC" "alpha blocked exit when notes invalid"
assert_contains "FAIL: G_NOTES_VALIDATED" "$OUT" "alpha FAIL line names G_NOTES_VALIDATED"
assert_contains "STATUS=blocked" "$OUT" "alpha prints STATUS=blocked when notes invalid"

echo "== alpha: G_MANIFEST_VALID fails =="
run_alpha "$GOOD_NOTES" "$BAD_MANIFEST" "$EVIDENCE_FILE"
assert_rc nonzero "$RC" "alpha blocked exit when manifest invalid"
assert_contains "FAIL: G_MANIFEST_VALID" "$OUT" "alpha FAIL line names G_MANIFEST_VALID"

echo "== alpha: G_NO_RELEASE_BLOCKERS fails =="
run_alpha "$NOTES_WITH_BLOCKER" "$GOOD_MANIFEST" "$EVIDENCE_FILE"
assert_rc nonzero "$RC" "alpha blocked exit when a [Blocker] known issue exists"
assert_contains "FAIL: G_NO_RELEASE_BLOCKERS" "$OUT" "alpha FAIL line names G_NO_RELEASE_BLOCKERS (case-insensitive match)"

echo "== alpha: G_SHELL_CONSUMED_ALPHA fails =="
run_alpha "$GOOD_NOTES" "$GOOD_MANIFEST" ""
assert_rc nonzero "$RC" "alpha blocked exit when shell-consumed evidence empty"
assert_contains "FAIL: G_SHELL_CONSUMED_ALPHA" "$OUT" "alpha FAIL line names G_SHELL_CONSUMED_ALPHA"

# --- alpha all gates pass ------------------------------------------------------

echo "== alpha: all gates pass =="
run_alpha "$GOOD_NOTES" "$GOOD_MANIFEST" "$EVIDENCE_FILE"
assert_rc zero "$RC" "alpha exits zero when all gates pass"
assert_contains "STATUS=ready" "$OUT" "alpha prints STATUS=ready"
assert_not_contains "FAIL: " "$OUT" "alpha prints no FAIL lines on success"
assert_contains "tranche_id: codex-tranche-2-5" "$OUT" "alpha body has tranche_id (AV-BR-6)"
assert_contains "release_notes_path: docs/release/SD-16/release-notes.md" "$OUT" "alpha body has release_notes_path (AV-BR-6)"
assert_contains "alpha_release_url: https://github.com/electricm0nk/codex/releases/tag/alpha%2F0.5.0-abc1234" "$OUT" "alpha body has alpha_release_url (AV-BR-6)"
assert_contains "verification_evidence: $EVIDENCE_FILE" "$OUT" "alpha body has verification_evidence (AV-BR-6)"
assert_contains "known_issues:" "$OUT" "alpha body has known_issues (AV-BR-6)"
assert_contains "- Minor cosmetic flicker in the updater panel." "$OUT" "alpha body carries the Known Issues bullet list"

echo "== alpha: manifest missing -> WARN but still ready =="
run_alpha "$GOOD_NOTES" "$TMP/no-such-manifest.json" "$EVIDENCE_FILE"
assert_rc zero "$RC" "alpha exits zero when manifest not yet shipped (E1-F1 pending)"
assert_contains "WARN:" "$OUT" "alpha warns loudly when manifest missing"
assert_contains "STATUS=ready" "$OUT" "alpha still ready when manifest missing"

echo "== alpha: --body-out writes the body to a file =="
BODY_OUT_FILE="$TMP/alpha-body.md"
run_alpha "$GOOD_NOTES" "$GOOD_MANIFEST" "$EVIDENCE_FILE" --body-out "$BODY_OUT_FILE"
assert_rc zero "$RC" "alpha --body-out exits zero"
BODY_CONTENT="$(cat "$BODY_OUT_FILE" 2>/dev/null || echo MISSING)"
assert_contains "tranche_id: codex-tranche-2-5" "$BODY_CONTENT" "alpha --body-out file contains the PR body"

# --- beta gate failures (each gate individually) -------------------------------

export FAKE_GH_VIEW_JSON="$BETA_VIEW_JSON"

echo "== beta: G_BETA_RELEASE_EXISTS fails =="
FAKE_GH_VIEW_FAIL=1 run_beta "$GOOD_NOTES" "https://ci.example/runs/1" "$EVIDENCE_FILE"
assert_rc nonzero "$RC" "beta blocked exit when beta release missing"
assert_contains "FAIL: G_BETA_RELEASE_EXISTS" "$OUT" "beta FAIL line names G_BETA_RELEASE_EXISTS"
assert_contains "STATUS=blocked" "$OUT" "beta prints STATUS=blocked when release missing"

echo "== beta: G_BETA_VERIFIED fails =="
run_beta "$GOOD_NOTES" "$TMP/does-not-exist.log" "$EVIDENCE_FILE"
assert_rc nonzero "$RC" "beta blocked exit when verification evidence is not a URL or existing file"
assert_contains "FAIL: G_BETA_VERIFIED" "$OUT" "beta FAIL line names G_BETA_VERIFIED"

echo "== beta: G_NO_STABLE_BLOCKERS fails =="
run_beta "$NOTES_WITH_STABLE_BLOCKER" "https://ci.example/runs/1" "$EVIDENCE_FILE"
assert_rc nonzero "$RC" "beta blocked exit when a [stable-blocker] known issue exists"
assert_contains "FAIL: G_NO_STABLE_BLOCKERS" "$OUT" "beta FAIL line names G_NO_STABLE_BLOCKERS"

echo "== beta: G_NOTES_VALID fails =="
run_beta "$NOTES_MISSING_SECTION" "https://ci.example/runs/1" "$EVIDENCE_FILE"
assert_rc nonzero "$RC" "beta blocked exit when notes invalid"
assert_contains "FAIL: G_NOTES_VALID" "$OUT" "beta FAIL line names G_NOTES_VALID"

echo "== beta: G_SHELL_CONSUMED_BETA fails =="
run_beta "$GOOD_NOTES" "https://ci.example/runs/1" ""
assert_rc nonzero "$RC" "beta blocked exit when shell-consumed evidence empty"
assert_contains "FAIL: G_SHELL_CONSUMED_BETA" "$OUT" "beta FAIL line names G_SHELL_CONSUMED_BETA"

echo "== beta: G_PROVENANCE_VALID fails (download fails) =="
FAKE_GH_DOWNLOAD_FAIL=1 run_beta "$GOOD_NOTES" "https://ci.example/runs/1" "$EVIDENCE_FILE"
assert_rc nonzero "$RC" "beta blocked exit when provenance.json cannot be downloaded"
assert_contains "FAIL: G_PROVENANCE_VALID" "$OUT" "beta FAIL line names G_PROVENANCE_VALID (download)"

echo "== beta: G_PROVENANCE_VALID fails (missing keys) =="
FAKE_GH_PROVENANCE_SRC="$BAD_PROVENANCE" run_beta "$GOOD_NOTES" "https://ci.example/runs/1" "$EVIDENCE_FILE"
assert_rc nonzero "$RC" "beta blocked exit when provenance.json misses required keys"
assert_contains "FAIL: G_PROVENANCE_VALID" "$OUT" "beta FAIL line names G_PROVENANCE_VALID (keys)"

# --- beta all gates pass --------------------------------------------------------

echo "== beta: all gates pass (beta-only [blocker] must not block stable) =="
run_beta "$NOTES_WITH_BETA_ONLY_BLOCKER" "https://ci.example/runs/1" "$EVIDENCE_FILE"
assert_rc zero "$RC" "beta exits zero when all gates pass"
assert_contains "STATUS=ready" "$OUT" "beta prints STATUS=ready"
assert_not_contains "FAIL: " "$OUT" "beta prints no FAIL lines on success"
assert_not_contains "G_NO_STABLE_BLOCKERS" "$OUT" "beta-only [blocker] does not trip G_NO_STABLE_BLOCKERS"
assert_contains "tranche_id: codex-tranche-2-5" "$OUT" "beta body has tranche_id (AV-BR-6)"
assert_contains "release_notes_path: docs/release/SD-16/release-notes.md" "$OUT" "beta body has release_notes_path (AV-BR-6)"
assert_contains "alpha_release_url:" "$OUT" "beta body has alpha_release_url key (AV-BR-6)"
assert_contains "verification_evidence: https://ci.example/runs/1" "$OUT" "beta body has verification_evidence (AV-BR-6)"
assert_contains "known_issues:" "$OUT" "beta body has known_issues (AV-BR-6)"
assert_contains "beta_release_url: https://github.com/electricm0nk/codex/releases/tag/beta%2F0.5.0-def5678" "$OUT" "beta body has beta_release_url"
assert_contains "provenance_url:" "$OUT" "beta body has provenance_url"

# --- the cardinal rule: neither script ever calls gh pr create ------------------

echo "== no script ever invoked gh pr create =="
GH_CALLS="$(cat "$FAKE_GH_LOG")"
assert_not_contains "pr create" "$GH_CALLS" "no gh pr create call recorded across the entire suite"

echo
echo "ALL PROMOTION-GATE TESTS PASSED"
exit 0
