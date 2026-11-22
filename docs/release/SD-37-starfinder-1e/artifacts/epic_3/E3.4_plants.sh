#!/usr/bin/env bash
# SD-37 E3.4 planted mutations: each plant must turn its gate red (exit != 0), and the restored
# tree must turn it green again. Run from the repo root with the pinned oracle in
# $PCGEN_CORPUS_ROOT. Prints one line per plant: "<plant> -> FAIL (exit n)" or "-> PASS".
# Every plant is restored from a backup copy before the next starts; the script ends by
# re-running every gate on the restored tree.
set -u
cd "$(git rev-parse --show-toplevel)"
SCRATCH="${SCRATCH:?set SCRATCH to a scratch directory outside the repo}"
mkdir -p "$SCRATCH"
TABLE=docs/release/SD-37-starfinder-1e/artifacts/epic_3/token-mapping/sf-mapping-table.v1.json
PKG=data/starfinder-1e/sheet_rules
CORPUS=data/starfinder-1e/corpus
SD=docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py
tree_sum() { find "$TABLE" "$PKG" "$CORPUS" -type f -print0 | sort -z | xargs -0 sha256sum | sha256sum | cut -c1-16; }
BEFORE=$(tree_sum); echo "inputs sha256 before: $BEFORE"
convert_check() { cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --system starfinder-1e --check >"$SCRATCH/last.log" 2>&1; }
corpus_check() { cargo run --locked -j 8 -q -p codex-ingest --bin sf_corpus -- --check >"$SCRATCH/last.log" 2>&1; }
report() { local name=$1 status=$2; if [[ $status -ne 0 ]]; then echo "$name -> FAIL (exit $status)"; else echo "$name -> PASS"; fi; }

# P1 = M1 on the converter: swap ALTHP and CURRENTMAX in the table the converter reads.
cp "$TABLE" "$SCRATCH/table.bak"
python3 - "$TABLE" <<'PY'
import sys; p=sys.argv[1]; s=open(p).read()
s=s.replace("BONUS:HP|ALTHP","@@A@@").replace("BONUS:HP|CURRENTMAX","BONUS:HP|ALTHP").replace("@@A@@","BONUS:HP|CURRENTMAX")
open(p,"w").write(s)
PY
convert_check; report "P1 table M1 (swap ALTHP<->CURRENTMAX) / sheet_rule_convert --system starfinder-1e --check" $?
grep -m3 -E 'stale on disk' "$SCRATCH/last.log" | sed 's/^/    /'
cp "$SCRATCH/table.bak" "$TABLE"

# P2: a hand edit to one package file (Second Skin's KAC 2 -> 3).
F="$PKG/core/equipment/second_skin.json"; cp "$F" "$SCRATCH/pkg.bak"
python3 - "$F" <<'PY'
import sys,json; p=sys.argv[1]; d=json.load(open(p))
for r in d:
    if r.get("target")=="Kac": r["value"]={"Number":{"Const":3}}
open(p,"w").write(json.dumps(d,indent=2)+"\n")
PY
convert_check; report "P2 package hand edit (second_skin KAC 2->3) / --check" $?
bash scripts/verify.sh --only sf-sheet-rules-check >"$SCRATCH/stage.log" 2>&1; report "P2 package hand edit / verify.sh --only sf-sheet-rules-check" $?
cp "$SCRATCH/pkg.bak" "$F"

# P3: a corpus record un-renamed (a product-identity name written back in clear).
C=$(grep -l '"codex_generated_name": true' -r "$CORPUS/core/deity" | head -1); cp "$C" "$SCRATCH/corpus.bak"
python3 - "$C" <<'PY'
import sys,json; p=sys.argv[1]; d=json.load(open(p)); d["data"]["name"]="Abadar"; d["license"]="OGL"; d["pi_field"]=None
open(p,"w").write(json.dumps(d,indent=2)+"\n")
PY
corpus_check; report "P3 corpus PI name written back in clear ($C) / sf_corpus --check" $?
cp "$SCRATCH/corpus.bak" "$C"

# P4: a corpus record deleted.
C2="$CORPUS/core/feat/toughness.json"; cp "$C2" "$SCRATCH/corpus2.bak"; rm "$C2"
corpus_check; report "P4 corpus record deleted (core/feat/toughness) / sf_corpus --check" $?
cp "$SCRATCH/corpus2.bak" "$C2"

# P5: structural diff of a fresh SF dump against the committed SF package, then two plants on
# copies of the dump.
rm -rf "$SCRATCH/sfdump" "$SCRATCH/sfdump_m1" "$SCRATCH/sfdump_m2"
cargo run --locked -j 8 -q -p codex-ingest --bin sheet_rule_convert -- --system starfinder-1e --dump "$SCRATCH/sfdump" >"$SCRATCH/last.log" 2>&1
python3 "$SD" "$SCRATCH/sfdump" --baseline "$PKG" >"$SCRATCH/sd.log" 2>&1; report "P5a structural_diff fresh SF dump vs committed package" $?
grep -E 'verdict=' "$SCRATCH/sd.log" | tail -1 | sed 's/^/    /'
cp -r "$SCRATCH/sfdump" "$SCRATCH/sfdump_m1"; rm "$SCRATCH/sfdump_m1/core/class/soldier.json"
python3 "$SD" "$SCRATCH/sfdump_m1" --baseline "$PKG" >"$SCRATCH/sd1.log" 2>&1; report "P5b structural_diff with core/class/soldier.json deleted" $?
grep -E 'verdict=' "$SCRATCH/sd1.log" | tail -1 | sed 's/^/    /'
cp -r "$SCRATCH/sfdump" "$SCRATCH/sfdump_m2"
python3 - "$SCRATCH/sfdump_m2/core/class/soldier.json" <<'PY'
import sys,json; p=sys.argv[1]; d=json.load(open(p))
for r in d:
    if r.get("target")=="Stamina": r["target"]="Hp"
open(p,"w").write(json.dumps(d,indent=2)+"\n")
PY
python3 "$SD" "$SCRATCH/sfdump_m2" --baseline "$PKG" >"$SCRATCH/sd2.log" 2>&1; report "P5c structural_diff with Soldier's Stamina line retargeted to Hp (the PF reading)" $?
grep -E 'verdict=' "$SCRATCH/sd2.log" | tail -1 | sed 's/^/    /'

# Restored tree: every gate green again.
AFTER=$(tree_sum); echo "inputs sha256 after:  $AFTER"; [[ "$AFTER" == "$BEFORE" ]] && echo "restored: table, package and corpus byte-identical to before" || echo "RESTORE MISMATCH"
convert_check; report "restored / sheet_rule_convert --system starfinder-1e --check" $?
corpus_check; report "restored / sf_corpus --check" $?
