#!/usr/bin/env bash
# SD-37 E7.1: run the Starfinder oracle-parity roster through the REAL PCGen engine.
#
# Writes scripts/oracle_harness/sf_parity/<build>.oracle.txt for every roster row of
# sf_parity_builds.py (the four seeds + one level-1 build of each of the 10 player classes), each a
# headless batch export (scripts/pcgen-run-character.sh) with the export sheet sf_parity.txt.ftl,
# and sf_mechanic_1_drone.oracle.txt for the Mechanic 1's drone: a follower reads its master's
# variables, so that one runs in PCGen's party batch-export mode (`pcgen.system.Main --party`) with
# the master first, exactly as E5.4's run_drone_party.sh, through the export sheet sf_parity_drone.txt.
# The desktop test `sf_oracle_parity` compares these files with the engine (apps/desktop/src-tauri).
#
# Usage (from the repo root): bash scripts/oracle_harness/sf_parity_run.sh [<build-stem> ...]
# Requires PCGEN_REPO_DIR (the pinned checkout; scripts/fetch-pcgen-oracle.sh). ~45 s per build.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
repo="$(git -C "$here" rev-parse --show-toplevel)"
: "${PCGEN_REPO_DIR:?set PCGEN_REPO_DIR to the pinned PCGen checkout (scripts/fetch-pcgen-oracle.sh)}"
pin="$(awk -F= '$1=="PCGEN_ORACLE_SHA"{print $2}' "$repo/scripts/pcgen-oracle-pin.env")"
have="$(git -C "$PCGEN_REPO_DIR" rev-parse HEAD)"
[ "$pin" = "$have" ] || { echo "oracle off-pin: $have != $pin" >&2; exit 1; }
out="$here/sf_parity"; mkdir -p "$out"
work="$(mktemp -d)"; trap 'rm -rf "$work"' EXIT
python3 "$here/sf_parity_builds.py" "$work/pcg" >/dev/null
only=("$@")
for pcg in "$work"/pcg/*.pcg; do
  b="$(basename "$pcg" .pcg)"
  [ "$b" = sf_mechanic_1_drone ] && continue
  if [ ${#only[@]} -gt 0 ] && [[ ! " ${only[*]} " == *" $b "* ]]; then continue; fi
  bash "$repo/scripts/pcgen-run-character.sh" -c "$pcg" -e "$here/sf_parity.txt.ftl" -o "$out/$b.oracle.txt" >"$work/$b.log" 2>&1 \
    || { echo "$b FAILED (PCGen exit $?)" >&2; tail -20 "$work/$b.log" >&2; exit 2; }
  grep -q '^hp=' "$out/$b.oracle.txt" || { echo "$b: no hp= line" >&2; exit 2; }
  echo "$b ok"
done
if [ ${#only[@]} -eq 0 ] || [[ " ${only[*]} " == *" sf_mechanic_1_drone "* ]]; then
  mkdir -p "$work/settings"
  printf 'VERSION:6.09.00\n%s,%s,\n' "$work/pcg/sf_mechanic_1.pcg" "$work/pcg/sf_mechanic_1_drone.pcg" > "$work/drone.pcp"
  ( cd "$PCGEN_REPO_DIR" && ./gradlew run --console=plain -q --args="--party \"$work/drone.pcp\" --exportsheet \"$here/sf_parity_drone.txt\" --outputfile \"$work/raw.txt\" --settingsdir \"$work/settings\"" ) >"$work/drone.log" 2>&1 \
    || { echo "drone party run FAILED" >&2; tail -20 "$work/drone.log" >&2; exit 2; }
  [ -s "$work/raw.txt" ] || { echo "PCGen produced no drone output" >&2; exit 2; }
  sed -e 's/^oracle: //' -e 's/ end$//' "$work/raw.txt" | tr ' ' '\n' | awk 'NF' > "$out/sf_mechanic_1_drone.oracle.txt"
  grep -q '^hp=' "$out/sf_mechanic_1_drone.oracle.txt" || { echo "no drone hp= line" >&2; exit 2; }
  echo "sf_mechanic_1_drone ok"
fi
