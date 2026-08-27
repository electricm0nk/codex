#!/usr/bin/env bash
set -uo pipefail
export PCGEN_REPO_DIR=/tmp/pcgen-oracle-sd33-r3-combat
JOBS="$1"
SETTINGS_BASE="$2"
PARALLEL="$3"

run_one() {
  local line="$1"
  IFS=$'\t' read -r pcg ftl out <<< "$line"
  local slug
  slug=$(basename "$pcg" .pcg)
  bash /home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_1f33bc36-e5f-2/scripts/oracle_harness/charbuild_remainder_run_one.sh \
    "$pcg" "$ftl" "$out" "${SETTINGS_BASE}/${slug}" >"${out}.log" 2>&1
  echo "EXIT $? $slug"
}
export -f run_one
export SETTINGS_BASE

cat "$JOBS" | xargs -d '\n' -P "$PARALLEL" -I{} bash -c 'run_one "$@"' _ {}
