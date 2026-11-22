#!/usr/bin/env bash
# The publish channel is an explicit three-way case on the ref; any other ref fails rather than
# defaulting to alpha. Both publish-workflow steps that resolve the channel must use the script.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
SCRIPT="$ROOT/scripts/release/resolve-channel.sh"
WF="$ROOT/.github/workflows/publish-tester-release.yml"
fail() { echo "FAIL: $*" >&2; exit 1; }

expect() { # ref, expected stdout
  local out
  out="$(bash "$SCRIPT" "$1")" || fail "$1 must resolve, exited $?"
  [ "$out" = "$2" ] || fail "$1 resolved to: $out"
}
expect main    $'channel=stable\nprerelease=false'
expect test    $'channel=beta\nprerelease=true'
expect develop $'channel=alpha\nprerelease=true'

for ref in "" feature/x Main tests develop2 refs/heads/main; do
  if out="$(bash "$SCRIPT" "$ref" 2>&1)"; then fail "ref '$ref' must fail, got: $out"; fi
  grep -q "no publish channel" <<<"$out" || fail "ref '$ref' error must say why: $out"
done

# The workflow triggers on all three branches and resolves the channel through the script only.
TRIGGERS="$(awk '/^on:/{on=1} on&&/^ *- /{print $2} /^permissions:/{exit}' "$WF" | sort | tr '\n' ' ')"
[ "$TRIGGERS" = "develop main test " ] || fail "push triggers are: $TRIGGERS"
USES="$(awk '/resolve-channel\.sh/{n++} END{print n+0}' "$WF")"
[ "$USES" -ge 2 ] || fail "both channel steps must call resolve-channel.sh (found $USES)"
if awk '/CHANNEL="alpha"|channel=alpha/{found=1} END{exit !found}' "$WF"; then
  fail "the workflow still spells a default alpha channel itself"
fi
echo "PASS"
