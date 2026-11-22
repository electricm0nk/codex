#!/usr/bin/env bash
# PreToolUse hook (matcher: Bash).
#
# Enforces two of AGENTS.md's shared-tree rules as a hard control instead of prose:
#   - "Never git stash in a shared tree." (git stash bit us 3x — see memory)
#   - Never `git add -A` / `git add --all` / `git add .` (explicit paths only).
#
# Reads the PreToolUse hook JSON on stdin, inspects .tool_input.command, and
# denies (via hookSpecificOutput.permissionDecision) any command whose git-stash
# invocation is not `git stash list` / `git stash show`, or whose git-add
# invocation is a blanket `-A` / `--all` / `.`. Everything else is allowed.
#
# The check applies per semicolon/&&/||/pipe/newline-separated segment, and
# strips a leading `git -C <path>` before matching, so `cd x && git stash` and
# `git -C /x add --all` are caught too.

set -euo pipefail

input="$(cat)"

command="$(printf '%s' "$input" | python3 -c '
import json, sys
try:
    data = json.load(sys.stdin)
except Exception:
    print("")
    sys.exit(0)
print(data.get("tool_input", {}).get("command", "") or "")
')"

if [ -z "$command" ]; then
    exit 0
fi

set +e
result="$(python3 - "$command" <<'PYEOF'
import re
import sys

command = sys.argv[1]

# Split on common shell chainers and newlines. Not a full shell parser, but
# enough to catch the chained forms this control is scoped to.
segments = re.split(r'&&|\|\||;|\n|\|', command)

STASH_ALLOWED = re.compile(r'^git\s+stash\s+(list|show)\b')
STASH_ANY = re.compile(r'^git\s+stash\b')
ADD_DASH_A = re.compile(r'^git\s+add\s+(-A\b|--all\b)')
ADD_DOT = re.compile(r'^git\s+add\s+\.\s*$')
GIT_DASH_C = re.compile(r'^git\s+-C\s+\S+\s+')

reason = None
for seg in segments:
    s = seg.strip()
    if not s:
        continue
    # Strip a leading `git -C <path>` so `git -C /x add --all` normalizes to
    # `git add --all` for matching purposes.
    normalized = GIT_DASH_C.sub('git ', s)

    if STASH_ANY.match(normalized) and not STASH_ALLOWED.match(normalized):
        reason = (
            "git stash is banned in this shared tree (it is tree-wide and takes "
            "everyone's uncommitted work). Allowed: `git stash list`, `git stash show`. "
            "To read a HEAD baseline use `git show HEAD:<file>` into a temp path, or a "
            "separate worktree. Blocked command segment: " + s
        )
        break
    if ADD_DASH_A.match(normalized) or ADD_DOT.match(normalized):
        reason = (
            "git add -A / --all / . is banned in this shared tree. Stage explicit "
            "paths only. Blocked command segment: " + s
        )
        break

if reason:
    print(reason)
    sys.exit(1)
sys.exit(0)
PYEOF
)"
status=$?
set -e

if [ "$status" -ne 0 ]; then
    escaped_reason="$(printf '%s' "$result" | python3 -c 'import json, sys; print(json.dumps(sys.stdin.read()))')"
    printf '{"hookSpecificOutput": {"hookEventName": "PreToolUse", "permissionDecision": "deny", "permissionDecisionReason": %s}}\n' "$escaped_reason"
    printf '%s\n' "$result" >&2
    exit 0
fi

exit 0
