#!/usr/bin/env bash
# SD-42 E1.7 (decisions.md §9): PCGen's own export of each import-proof file, through the REAL PCGen
# engine at the pinned oracle. Writes exports/<sample>.txt for PCGen's 7 sample characters
# ($PCGEN_REPO_DIR/code/testsuite/PCGfiles); with --operator, the operator's file ($CODEX_SD42_PCG)
# to $HOME/sample_pcg/parity/<stem>.txt, never into the repo (decisions.md §10).
# The template is the game mode's totals template (sheet-totals.txt.ftl less its prose lines, or
# sf_parity.txt.ftl) followed by pcg-gear.txt.ftl. A companion file (sf_mechanic_drone) is exported
# alone; Codex refuses it by name (§21 Q-A), and the test reads only its class line.
# The test crates/codex-ingest/tests/pcg_import_parity.rs compares these with Codex's import.
# Usage (repo root): bash scripts/oracle_harness/pcg_import_parity/run.sh [--operator] [<sample> ...]
# ~45 s per file.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
harness="$(dirname "$here")"
repo="$(git -C "$here" rev-parse --show-toplevel)"
: "${PCGEN_REPO_DIR:=$HOME/workspace/repos/pcgen}"
export PCGEN_REPO_DIR
pin="$(awk -F= '$1=="PCGEN_ORACLE_SHA"{print $2}' "$repo/scripts/pcgen-oracle-pin.env")"
have="$(git -C "$PCGEN_REPO_DIR" rev-parse HEAD)"
[ "$pin" = "$have" ] || { echo "oracle off-pin: $have != $pin" >&2; exit 1; }
work="$(mktemp -d)"; trap 'rm -rf "$work"' EXIT
grep -v '\.DESC=' "$harness/sheet-totals.txt.ftl" | cat - "$here/pcg-gear.txt.ftl" > "$work/pf.ftl"
cat "$harness/sf_parity.txt.ftl" "$here/pcg-gear.txt.ftl" > "$work/sf.ftl"

export_one() { # <pcg> <out>
  local pcg="$1" out="$2" mode=pf
  grep -q '^GAMEMODE:Starfinder' "$pcg" && mode=sf
  bash "$repo/scripts/pcgen-run-character.sh" -c "$pcg" -e "$work/$mode.ftl" -o "$out" >"$work/log" 2>&1 \
    || { echo "$(basename "$pcg") FAILED" >&2; tail -20 "$work/log" >&2; exit 2; }
  grep -q '^pcg.classes=' "$out" || { echo "$out: no pcg.classes= line" >&2; exit 2; }
  echo "$(basename "$pcg") ok -> $out"
}

if [ "${1:-}" = --operator ]; then
  # $CODEX_SD42_PCG, else the one .pcg in $HOME/sample_pcg (no name is written here).
  pcg="${CODEX_SD42_PCG:-$(find "$HOME/sample_pcg" -maxdepth 1 -name '*.pcg' -print -quit 2>/dev/null)}"
  [ -n "$pcg" ] && [ -f "$pcg" ] || { echo "SKIPPED: operator .pcg absent at ${pcg:-$HOME/sample_pcg/*.pcg} (decisions.md §10)"; exit 0; }
  mkdir -p "$HOME/sample_pcg/parity"
  export_one "$pcg" "$HOME/sample_pcg/parity/$(basename "$pcg" .pcg).txt"
  exit 0
fi
samples=("$@")
[ ${#samples[@]} -gt 0 ] || samples=(pf_Cleric pf_Paladin pf_Rogue pf_goldielocks sf_mechanic sf_mechanic_drone sf_soldier)
for s in "${samples[@]}"; do
  export_one "$PCGEN_REPO_DIR/code/testsuite/PCGfiles/$s.pcg" "$here/exports/$s.txt"
done
