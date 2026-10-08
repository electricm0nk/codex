#!/usr/bin/env bash
# Re-run the E3.3 oracle observations: every build below through the REAL PCGen engine
# (scripts/pcgen-run-character.sh, headless batch export) with the export sheet
# sf-mapping-oracle.txt.ftl, writing <build>.oracle.txt next to this script.
#
# Builds: the four SD-37 SF seeds (this directory's .pcg files, written by make_seed_pcg.py from
# artifacts/epic_0/seed-builds.md) and PCGen's own two SF test characters
# ($PCGEN_REPO_DIR/code/testsuite/PCGfiles/sf_soldier.pcg, sf_mechanic.pcg; not copied here).
#
# Usage (from the repo root): bash docs/release/SD-37-starfinder-1e/artifacts/epic_3/token-mapping/oracle-builds/run_oracle_builds.sh
# Requires PCGEN_REPO_DIR (the pinned checkout, scripts/pcgen-oracle-pin.env). ~40 s per build.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(git -C "$here" rev-parse --show-toplevel)"
: "${PCGEN_REPO_DIR:?set PCGEN_REPO_DIR to the pinned PCGen checkout (scripts/fetch-pcgen-oracle.sh)}"
pin="$(awk -F= '$1=="PCGEN_ORACLE_SHA"{print $2}' "$repo/scripts/pcgen-oracle-pin.env")"
have="$(git -C "$PCGEN_REPO_DIR" rev-parse HEAD)"
[ "$pin" = "$have" ] || { echo "oracle off-pin: $have != $pin" >&2; exit 1; }
builds=("$here"/sf_seed_*.pcg "$PCGEN_REPO_DIR/code/testsuite/PCGfiles/sf_soldier.pcg" "$PCGEN_REPO_DIR/code/testsuite/PCGfiles/sf_mechanic.pcg")
for f in "${builds[@]}"; do
  b="$(basename "$f" .pcg)"
  bash "$repo/scripts/pcgen-run-character.sh" -c "$f" -e "$here/sf-mapping-oracle.txt.ftl" -o "$here/$b.oracle.txt" >/dev/null
  echo "$b ok"
done
