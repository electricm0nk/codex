#!/usr/bin/env bash
# SD-37 E5.4: the drone oracle observation. A follower reads its master's variables
# (`MASTERVAR("DroneCompanionLVL")`), which PCGen resolves only when the master is loaded, so a
# single-character run (`scripts/pcgen-run-character.sh -c sf_mechanic_drone.pcg`) prints
# DroneLVL 0 and hp 0. This runs the REAL PCGen engine in its party batch-export mode
# (`pcgen.system.Main --party`, `BatchExporter.exportParty`) over PCGen's own two test characters,
# master first: `$PCGEN_REPO_DIR/code/testsuite/PCGfiles/sf_mechanic.pcg` (ysoki mechanic 20) and
# `sf_mechanic_drone.pcg` (its drone R2-D2, `MASTER:Yo-yo|...|FILE:sf_mechanic.pcg`). The export
# sheet sf-drone-party-oracle.txt prints the drone's (party index 1) and the master's (index 0) values
# on one line (the legacy party template joins lines), split here into key=value lines in
# sf_mechanic_drone.oracle.txt next to this script.
#
# Usage (from the repo root): bash docs/release/SD-37-starfinder-1e/artifacts/epic_3/token-mapping/oracle-builds/run_drone_party.sh
# Requires PCGEN_REPO_DIR (the pinned checkout, scripts/pcgen-oracle-pin.env). ~25 s.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(git -C "$here" rev-parse --show-toplevel)"
: "${PCGEN_REPO_DIR:?set PCGEN_REPO_DIR to the pinned PCGen checkout (scripts/fetch-pcgen-oracle.sh)}"
pin="$(awk -F= '$1=="PCGEN_ORACLE_SHA"{print $2}' "$repo/scripts/pcgen-oracle-pin.env")"
have="$(git -C "$PCGEN_REPO_DIR" rev-parse HEAD)"
[ "$pin" = "$have" ] || { echo "oracle off-pin: $have != $pin" >&2; exit 1; }
work="$(mktemp -d)"; mkdir -p "$work/settings"; trap 'rm -rf "$work"' EXIT
pcg="$PCGEN_REPO_DIR/code/testsuite/PCGfiles"
printf 'VERSION:6.09.00\n%s,%s,\n' "$pcg/sf_mechanic.pcg" "$pcg/sf_mechanic_drone.pcg" > "$work/drone.pcp"
out="$here/sf_mechanic_drone.oracle.txt"; rm -f "$out"
( cd "$PCGEN_REPO_DIR" && ./gradlew run --console=plain -q --args="--party \"$work/drone.pcp\" --exportsheet \"$here/sf-drone-party-oracle.txt\" --outputfile \"$work/raw.txt\" --settingsdir \"$work/settings\"" ) >/dev/null
[ -s "$work/raw.txt" ] || { echo "PCGen produced no output" >&2; exit 2; }
sed -e 's/^oracle: //' -e 's/ end$//' "$work/raw.txt" | tr ' ' '\n' | awk 'NF' > "$out"
grep -q '^hp=' "$out" || { echo "no hp= line in $out" >&2; exit 2; }
cat "$out"
