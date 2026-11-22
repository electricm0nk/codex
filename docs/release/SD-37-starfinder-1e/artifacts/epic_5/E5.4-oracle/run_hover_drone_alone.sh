#!/usr/bin/env bash
# SD-37 E5.4: derive the hover drone from PCGen's own test drone (not copied into the repo) and run
# it alone through scripts/pcgen-run-character.sh. Usage (repo root, PCGEN_REPO_DIR set):
#   bash docs/release/SD-37-starfinder-1e/artifacts/epic_5/E5.4-oracle/run_hover_drone_alone.sh
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"; repo="$(git -C "$here" rev-parse --show-toplevel)"
: "${PCGEN_REPO_DIR:?set PCGEN_REPO_DIR (scripts/fetch-pcgen-oracle.sh)}"
work="$(mktemp -d)"; trap 'rm -rf "$work"' EXIT
sed -e 's/KEY:Drone Chassis ~ Stealth/KEY:Drone Chassis ~ Hover/' -e '/KEY:Drone Mod ~ Flight System/d' \
    "$PCGEN_REPO_DIR/code/testsuite/PCGfiles/sf_mechanic_drone.pcg" > "$work/hover_drone_alone.pcg"
bash "$repo/scripts/pcgen-run-character.sh" -c "$work/hover_drone_alone.pcg" -e "$here/abil.ftl" -o "$here/hover_drone_alone.oracle.txt" >/dev/null
cat "$here/hover_drone_alone.oracle.txt"
