#!/usr/bin/env bash
# The channel-index step must not need a tranche manifest for the branch being published: the repo
# holds only one (promotion_branch: develop), so beta (test) and stable (main) must still get their feed.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
WF="$ROOT/.github/workflows/publish-tester-release.yml"
fail() { echo "FAIL: $*" >&2; exit 1; }

# The real lookup lines, straight from the workflow (from TRANCHE_ID="" up to the PUBLISHED_AT lookup).
SNIPPET="$(awk '/^          TRANCHE_ID=""/{on=1} /PUBLISHED_AT="\$\(gh release view/{on=0} on' "$WF" | sed 's/^          //')"
[ -n "$SNIPPET" ] || fail "could not extract the tranche lookup from the workflow"

for ref in develop test main; do
  out="$(cd "$ROOT" && GITHUB_REF_NAME="$ref" bash -c "set -euo pipefail; $SNIPPET; echo \"TRANCHE_ID=\$TRANCHE_ID\"" 2>&1)" \
    || fail "ref '$ref' must not fail the channel-index step: $out"
  grep -q '^TRANCHE_ID=.\+' <<<"$out" || fail "ref '$ref' left an empty tranche label: $out"
done
echo "PASS test-channel-index-tranche-lookup"
