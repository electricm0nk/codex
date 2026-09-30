#!/usr/bin/env bash
# Finalize must tolerate absent optional artifacts (*.msi, *.dmg). Without
# nullglob, bash passes the literal '*.msi' to `gh release create`, which
# exits 1 with "no matches found" (run 36733199801).
set -euo pipefail
WF="$(cd "$(dirname "$0")/../../.." && pwd)/.github/workflows/publish-tester-release.yml"
fail() { echo "FAIL: $*" >&2; exit 1; }

# Take the release step body: from the GLOB=( line back to `set -euo pipefail`.
BODY="$(awk '/GLOB=\( \*\.deb/ {print prev_block; exit} /set -euo pipefail/ {blk=""} {blk=blk"\n"$0; prev_block=blk}' "$WF")"
[ -n "$BODY" ] || fail "could not locate the GLOB step"
grep -q 'shopt -s nullglob' <<<"$BODY" || fail "release step must enable nullglob before GLOB=(...)"

# Behavioural check: with nullglob, absent patterns expand to nothing.
D="$(mktemp -d)"; trap 'rm -rf "$D"' EXIT
touch "$D/a.AppImage" "$D/b.exe"
OUT="$(cd "$D" && bash -c 'shopt -s nullglob; G=( *.deb *.AppImage *.msi *.exe *.dmg ); printf "%s\n" "${G[@]}"')"
[ "$OUT" = $'a.AppImage\nb.exe' ] || fail "nullglob expansion wrong: $OUT"
echo "PASS"
