#!/usr/bin/env bash
# promotion-gates.yml finds the latest alpha/beta GitHub release by tag. The publish workflow names
# releases `<channel>-v<version>-<sha>` (the slash form `<channel>/v...` is only the manifest tag), so
# every jq filter in the gate must select that form. Runs the workflow's own filters.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../../.." && pwd)"
WF="$ROOT/.github/workflows/promotion-gates.yml"
fail() { echo "FAIL: $*" >&2; exit 1; }
D="$(mktemp -d)"; trap 'rm -rf "$D"' EXIT

cat > "$D/releases.json" <<'JSON'
[
  {"tag_name":"beta-v0.22.161-cccccccc","published_at":"2026-11-23T00:00:00Z","html_url":"https://x/beta-new",
   "assets":[{"name":"provenance.json","url":"https://x/beta-new/provenance"}]},
  {"tag_name":"alpha-v0.22.160-bbbbbbbb","published_at":"2026-11-22T00:00:00Z","html_url":"https://x/alpha-new","assets":[]},
  {"tag_name":"beta-v0.22.150-dddddddd","published_at":"2026-11-01T00:00:00Z","html_url":"https://x/beta-old","assets":[]},
  {"tag_name":"alpha-v0.22.140-eeeeeeee","published_at":"2026-10-01T00:00:00Z","html_url":"https://x/alpha-old","assets":[]}
]
JSON

FILTERS="$(awk -F"jq -r '" 'NF>1 && /tag_name/ { sub(/'"'"' < releases\.json.*/, "", $2); print $2 }' "$WF")"
N="$(awk 'END{print NR}' <<<"$FILTERS")"
[ "$N" -eq 5 ] || fail "expected 5 tag filters in promotion-gates.yml, found $N"

while IFS= read -r f; do
  got="$(jq -r "$f" < "$D/releases.json")"
  case "$f" in
    *'"alpha'*html_url*) want="https://x/alpha-new" ;;
    *'"alpha'*tag_name\ //\ empty*) want="alpha-v0.22.160-bbbbbbbb" ;;
    *'"beta'*provenance.json*) want="https://x/beta-new/provenance" ;;
    *'"beta'*html_url*) want="https://x/beta-new" ;;
    *'"beta'*tag_name\ //\ empty*) want="beta-v0.22.161-cccccccc" ;;
    *) fail "unrecognised filter: $f" ;;
  esac
  [ "$got" = "$want" ] || fail "filter found '$got', want '$want': $f"
done <<<"$FILTERS"
echo "PASS"
