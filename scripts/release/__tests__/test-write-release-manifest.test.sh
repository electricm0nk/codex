#!/usr/bin/env bash
# SD16-E4-F3b — bash self-test for scripts/release/{write,validate}_manifest.py
#
# Doctrine (per programs/codex/requirements/SD-16-feedback-loop-and-self-update-hardening/
# artifacts/SD16-E4-F2-execution-handoff-2026-07-03.md §TDD posture):
# - Auto-discovered only by direct invocation `bash <test>`.
# - NOT picked up by apps/desktop/scripts/run-tests.mjs (which is *.test.ts only).
# - Writes the test FIRST, runs it RED, then implementation, then GREEN, then refactor.
#
# Scope: this test asserts (a) the validator accepts a schema-conformant
# fixture, (b) the validator rejects a fixture with a malformed AppImage
# sha256, (c) the writer emits a payload that the validator accepts when
# fed a controlled environment.

set -euo pipefail

# Test file lives at scripts/release/__tests__/test-write-release-manifest.test.sh.
# Walk up three levels: __tests__ -> release -> scripts -> REPO_ROOT.
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
SCRIPT_DIR="${REPO_ROOT}/scripts/release"
FIXTURE_DIR="${SCRIPT_DIR}/__tests__/fixtures"
SCHEMA_PATH="${REPO_ROOT}/schemas/update/update-manifest.schema.json"

VALIDATOR="${SCRIPT_DIR}/validate_manifest.py"
WRITER="${SCRIPT_DIR}/write_release_manifest.py"

# Test 0: preflight — the two scripts and the two fixtures must exist after
# implementation lands. While the slice is RED (scripts not yet written),
# this assertion is what surfaces the missing-file error.
test_scripts_and_fixtures_exist() {
  if [ ! -f "${VALIDATOR}" ]; then
    echo "FAIL: missing ${VALIDATOR}" >&2
    exit 1
  fi
  if [ ! -f "${WRITER}" ]; then
    echo "FAIL: missing ${WRITER}" >&2
    exit 1
  fi
  if [ ! -f "${FIXTURE_DIR}/manifest-good.json" ]; then
    echo "FAIL: missing ${FIXTURE_DIR}/manifest-good.json" >&2
    exit 1
  fi
  if [ ! -f "${FIXTURE_DIR}/manifest-bad-hash.json" ]; then
    echo "FAIL: missing ${FIXTURE_DIR}/manifest-bad-hash.json" >&2
    exit 1
  fi
  if [ ! -f "${SCHEMA_PATH}" ]; then
    echo "FAIL: missing ${SCHEMA_PATH} (E3-F3a schema must be on develop before F3b can run)" >&2
    exit 1
  fi
}

# Test 1: the good fixture must validate against the schema.
test_good_fixture_validates() {
  python3 "${VALIDATOR}" --manifest "${FIXTURE_DIR}/manifest-good.json" --schema "${SCHEMA_PATH}"
}

# Test 2: a fixture with a malformed AppImage sha256 must be rejected.
# This is the AV-PUB-6 / AV-SCH-5 negative-case test: validator must fail closed.
test_bad_hash_fixture_rejected() {
  set +e
  python3 "${VALIDATOR}" \
    --manifest "${FIXTURE_DIR}/manifest-bad-hash.json" \
    --schema "${SCHEMA_PATH}"
  rc=$?
  set -e
  if [ "${rc}" -eq 0 ]; then
    echo "FAIL: validator accepted manifest-bad-hash.json (rc=0); expected non-zero" >&2
    exit 1
  fi
  echo "OK: bad-hash fixture rejected with rc=${rc}"
}

# Test 3: the writer must produce a payload that validates when run against
# a controlled environment. We stub the inputs the writer reads (channel,
# tag, version, sha256, size, run id, ref name, source commit, notes path).
test_writer_emits_valid_manifest() {
  local tmpdir
  tmpdir="$(mktemp -d)"
  trap "rm -rf '${tmpdir}'" EXIT

  # Stage a fake release-notes.md so the writer can compute the hash.
  mkdir -p "${tmpdir}/programs/codex/requirements/SD-16-feedback-loop-and-self-update-hardening"
  cat > "${tmpdir}/programs/codex/requirements/SD-16-feedback-loop-and-self-update-hardening/release-notes.md" <<'NOTES_EOF'
## Summary

- _placeholder._

## User-Visible Changes

- _placeholder._

## Defects Fixed

- _placeholder._

## Operational Notes

- _placeholder._

## Verification Evidence

- _placeholder._

## Known Issues

- _placeholder._

## Update Eligibility

- _placeholder._
NOTES_EOF

  # Stage a fake AppImage so the writer can compute its sha256/size.
  mkdir -p "${tmpdir}/release-staging"
  printf 'FAKE-APPIMAGE-CONTENT-FOR-TEST' > "${tmpdir}/release-staging/codex_0.0.0_amd64.AppImage"

  local out="${tmpdir}/out/update-manifest.json"
  mkdir -p "$(dirname "${out}")"

  python3 "${WRITER}" \
    --repo-root "${tmpdir}" \
    --channel alpha \
    --tag "alpha/v0.0.0-deadbeef" \
    --version "0.0.0" \
    --source-branch "develop" \
    --source-commit "0000000000000000000000000000000000000000" \
    --release-notes-path "programs/codex/requirements/SD-16-feedback-loop-and-self-update-hardening/release-notes.md" \
    --appimage-name "codex_0.0.0_amd64.AppImage" \
    --appimage-path "${tmpdir}/release-staging/codex_0.0.0_amd64.AppImage" \
    --appimage-url "https://github.com/electricm0nk/codex/releases/download/alpha-v0.0.0-deadbeef/codex_0.0.0_amd64.AppImage" \
    --workflow ".github/workflows/publish-tester-release.yml" \
    --run-id "1" \
    --run-attempt "1" \
    --release-notes-url "https://github.com/electricm0nk/codex/releases/tag/alpha-v0.0.0-deadbeef" \
    --min-supported-version "0.0.0" \
    --appimage-install "true" \
    --required-install-kind "appimage" \
    --output "${out}"

  if [ ! -f "${out}" ]; then
    echo "FAIL: writer did not produce ${out}" >&2
    exit 1
  fi

  # The writer's output must round-trip through the validator.
  python3 "${VALIDATOR}" --manifest "${out}" --schema "${SCHEMA_PATH}"
  echo "OK: writer output validated against schema"
}

main() {
  echo "SD16-E4-F3b: bash self-test starting"
  test_scripts_and_fixtures_exist
  test_good_fixture_validates
  test_bad_hash_fixture_rejected
  test_writer_emits_valid_manifest
  echo "SD16-E4-F3b: bash self-test PASS"
}

main "$@"
