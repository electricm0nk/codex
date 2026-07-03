#!/usr/bin/env bash
# tools/ci/branch-promotion-guard.sh
#
# Sourceable policy + runnable guard for the SD-16 / Tranche 2.x
# release-channel branch governance:
#
#   feature/*  -->  develop  (alpha)
#   develop    -->  test     (beta)
#   test       -->  main     (stable)
#
# Behavior:
#   - When sourced: defines `verify_promotion_source` so tests can exercise it.
#   - When run directly with env vars (SOURCE_BRANCH, HEAD_REPO, BASE_REPO,
#     EXPECTED_SOURCE): acts as the GitHub Actions step body and exits non-zero
#     when the policy is violated.
#
# Both the GitHub Actions workflows and the unit tests MUST exercise the same
# `verify_promotion_source` function. Drift between this file and the
# workflows fails the test suite.

set -euo pipefail

# verify_promotion_source <expected_source_branch> <source_branch> <head_repo> <base_repo>
#
# Emits a human-readable error on stderr and returns non-zero when the PR
# must NOT be merged. Returns zero when the PR satisfies the policy.
verify_promotion_source() {
  local expected="$1"
  local source_branch="$2"
  local head_repo="$3"
  local base_repo="$4"

  if [ "$head_repo" != "$base_repo" ]; then
    echo "Pull requests into the target branch must come from this repository's ${expected} branch." >&2
    echo "Got head repo: ${head_repo}" >&2
    return 1
  fi

  if [ "$source_branch" != "$expected" ]; then
    echo "Pull requests into the target branch must come from ${expected}. Got: ${source_branch}" >&2
    return 1
  fi

  return 0
}

# When invoked directly (not sourced), act as the GitHub Actions step body.
if [ "${BASH_SOURCE[0]}" = "${0}" ]; then
  : "${EXPECTED_SOURCE:?EXPECTED_SOURCE must be set (e.g. develop or test)}"
  : "${SOURCE_BRANCH:?SOURCE_BRANCH must be set from github.event.pull_request.head.ref}"
  : "${HEAD_REPO:?HEAD_REPO must be set from github.event.pull_request.head.repo.full_name}"
  : "${BASE_REPO:?BASE_REPO must be set from github.repository}"

  verify_promotion_source "$EXPECTED_SOURCE" "$SOURCE_BRANCH" "$HEAD_REPO" "$BASE_REPO"
fi