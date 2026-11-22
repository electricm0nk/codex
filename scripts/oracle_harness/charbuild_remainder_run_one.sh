#!/usr/bin/env bash
# SD-33 AT-33-E5-remainder-charbuild -- runs one BatchExporter invocation
# against the pinned, installed PCGen distribution directly (no gradle
# daemon), so N parallel invocations never contend on one shared gradle
# daemon across concurrent sibling lanes on the same checkout. Proven
# byte-identical to `./gradlew run`'s own output for the same inputs
# this cycle (see the cycle receipt).
#
# Usage: charbuild_remainder_run_one.sh <pcg-path> <ftl-path> <out-txt-path> <settings-dir>
set -euo pipefail
PCG="$1"; FTL="$2"; OUT="$3"; SETTINGS="$4"
PCGEN_REPO_DIR="${PCGEN_REPO_DIR:-$HOME/workspace/repos/pcgen}"
mkdir -p "$SETTINGS"
cd "$PCGEN_REPO_DIR/build/install/pcgen"
exec java --module-path "$PCGEN_REPO_DIR/mods/lib" \
  --add-modules javafx.base,javafx.controls,javafx.fxml,javafx.graphics,javafx.swing,javafx.web,javafx.media \
  -cp "lib/*" pcgen.system.Main \
  -c "$PCG" -E "$FTL" -o "$OUT" -s "$SETTINGS"
