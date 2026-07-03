#!/usr/bin/env bash
# tests/sd16-e5-f1/test_branch_promotion_guard.sh
#
# Unit tests for the SD-16 / Tranche 2.x release-channel branch governance
# policy (feature --> develop --> test --> main).
#
# This test exercises `verify_promotion_source` from
# `tools/ci/branch-promotion-guard.sh`. The same function is what the GitHub
# Actions workflows execute at PR time; if these tests pass and the workflow
# wiring is intact, the runtime enforcement matches the unit-tested policy.
#
# Run:
#   bash tests/sd16-e5-f1/test_branch_promotion_guard.sh

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
GUARD="$REPO_ROOT/tools/ci/branch-promotion-guard.sh"

if [ ! -f "$GUARD" ]; then
  echo "FAIL: guard script not found at $GUARD" >&2
  exit 1
fi

# shellcheck source=/dev/null
source "$GUARD"

PASSED=0
FAILED=0

assert_eq() {
  local label="$1"
  local expected="$2"
  local actual="$3"
  if [ "$expected" = "$actual" ]; then
    echo "PASS: $label"
    PASSED=$((PASSED + 1))
  else
    echo "FAIL: $label (expected=$expected actual=$actual)" >&2
    FAILED=$((FAILED + 1))
  fi
}

# assert_ok <label> <expected> <source> <head_repo> <base_repo>
assert_ok() {
  local label="$1"; shift
  if verify_promotion_source "$@"; then
    assert_eq "$label" "0" "0"
  else
    assert_eq "$label" "0" "1"
  fi
}

# assert_rejected <label> <expected> <source> <head_repo> <base_repo>
assert_rejected() {
  local label="$1"; shift
  if verify_promotion_source "$@"; then
    assert_eq "$label" "1" "0"
  else
    assert_eq "$label" "1" "1"
  fi
}

BASE_REPO="electricm0nk/codex"

# --- develop -> test (beta) -------------------------------------------------

assert_ok \
  "develop into test from same repo is allowed" \
  "develop" "develop" "$BASE_REPO" "$BASE_REPO"

assert_rejected \
  "feature/x into test from same repo is rejected" \
  "develop" "feature/x" "$BASE_REPO" "$BASE_REPO"

assert_rejected \
  "develop-into-test is rejected when head is a fork with same branch name" \
  "develop" "develop" "forker/codex" "$BASE_REPO"

assert_rejected \
  "main into test from same repo is rejected (wrong source)" \
  "develop" "main" "$BASE_REPO" "$BASE_REPO"

# --- test -> main (stable) --------------------------------------------------

assert_ok \
  "test into main from same repo is allowed" \
  "test" "test" "$BASE_REPO" "$BASE_REPO"

assert_rejected \
  "develop into main from same repo is rejected (now requires test)" \
  "test" "develop" "$BASE_REPO" "$BASE_REPO"

assert_rejected \
  "feature/x into main from same repo is rejected" \
  "test" "feature/x" "$BASE_REPO" "$BASE_REPO"

assert_rejected \
  "test-into-main is rejected when head is a fork with same branch name" \
  "test" "test" "forker/codex" "$BASE_REPO"

# --- direct script invocation (the CI step body) ---------------------------

assert_eq "direct invocation accepts good PR (develop into test)" \
  "0" "$(EXPECTED_SOURCE=develop SOURCE_BRANCH=develop HEAD_REPO="$BASE_REPO" BASE_REPO="$BASE_REPO" bash "$GUARD" >/dev/null 2>&1; echo $?)"

assert_eq "direct invocation rejects bad PR (develop into main)" \
  "1" "$(EXPECTED_SOURCE=test SOURCE_BRANCH=develop HEAD_REPO="$BASE_REPO" BASE_REPO="$BASE_REPO" bash "$GUARD" >/dev/null 2>&1; echo $?)"

assert_eq "direct invocation rejects fork (test into main from fork)" \
  "1" "$(EXPECTED_SOURCE=test SOURCE_BRANCH=test HEAD_REPO="forker/codex" BASE_REPO="$BASE_REPO" bash "$GUARD" >/dev/null 2>&1; echo $?)"

echo
echo "branch promotion guard tests: $PASSED passed, $FAILED failed"
exit "$FAILED"