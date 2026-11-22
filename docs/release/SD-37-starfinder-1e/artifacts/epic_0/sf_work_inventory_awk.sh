#!/usr/bin/env bash
# SD-37 E0.3 implementation B: an independent awk re-derivation of the per-book dispositions.
# Prints TSV `book<TAB>disposition<TAB>count` (disposition `rows` = F-6 rows) and `EXCLUDED<TAB>name<TAB>rows`.
# Shares no code with sf_work_inventory.py (Python). Exit 1 on an unclassifiable file or off-pin oracle.
set -eu
export LC_ALL=C
PC_ROOT="${PCGEN_REPO_DIR:-$HOME/workspace/repos/pcgen}"
SF="${PCGEN_CORPUS_ROOT:-$PC_ROOT/data}/starfinder"
PIN=$(awk -F= '/^PCGEN_ORACLE_SHA=/{sub(/[ #].*/,"",$2); print $2}' scripts/pcgen-oracle-pin.env)
[ "$(git -C "$PC_ROOT" rev-parse HEAD)" = "$PIN" ] || { echo "OFF_PIN" >&2; exit 1; }
BOOKS="paizo/core paizo/armory paizo/character_operations_manual paizo/pact_worlds paizo/near_space paizo/alien_archive paizo/alien_archive_2 paizo/alien_archive_3"
T=$(mktemp -d); trap 'rm -rf "$T"' EXIT
for b in $BOOKS; do
  find "$SF/$b" -type f -name '*.lst' ! -path "$SF/paizo/core/_society/*" | sort | awk -v b="$b" '{print b "\t" $0}'
done > "$T/files.tsv"
# shared awk prelude: suffix + kind tables, row predicate helpers
PRE='
function setup(   n,i,a,k) {
  n = split("abilities_class abilities_race abilities profs_weapon profs_armor profs_shield equip_gear equip_magic equipmods feats_spw spells_aa classes races feats skills spells equip templates deities languages _datacontrols _globalmodifiers abilitycategories biosettings companionmods _datatables _dynamic _variables _align _saves _sizes _stats kits", L, " ")
  split("abilities_class abilities_race abilities profs_weapon profs_armor profs_shield equip_gear equip_magic equipmods feats_spw spells_aa classes races feats skills spells equip templates deities languages", Kn, " ")
  split("ability ability ability proficiency proficiency proficiency equipment equipment equipment_modifier feat spell class race feat skill spell equipment template deity language", Kv, " ")
  for (i in Kn) KIND[Kn[i]] = Kv[i]
  # order by length desc is required: insertion sort on L
  for (i = 2; i <= n; i++) { k = L[i]; j = i - 1; while (j >= 1 && length(L[j]) < length(k)) { L[j+1] = L[j]; j-- } L[j+1] = k }
  NL = n
}
function suffix(path,   f, s, i) {
  f = path; sub(/.*\//, "", f); sub(/\.lst$/, "", f)
  for (i = 1; i <= NL; i++) { s = L[i]; if (length(f) > length(s) && substr(f, length(f) - length(s)) == "_" s) return s }
  return ""
}
function isrow(l) { if (l == "") return 0; if (l ~ /^[#\t \r]/) return 0; if (l ~ /^(SOURCELONG|SOURCESHORT|SOURCEWEB|SOURCEDATE)/) return 0; return 1 }
function baseof(f,   x) { x = f; sub(/^CATEGORY=[^|]*\|/, "", x); sub(/^CLASS:/, "", x); if (x ~ /\.MOD$/) x = substr(x, 1, length(x) - 4); return x }
'
awk -F'\t' "$PRE"'
BEGIN { setup() }
{ b=$1; p=$2; s=suffix(p); if (s=="") { print "UNCLASSIFIED " p > "/dev/stderr"; bad=1; next }
  if (!(s in KIND)) next; kind=KIND[s]
  while ((getline line < p) > 0) {
    if (!isrow(line)) continue
    n=split(line, F, "\t"); f1=F[1]
    if (f1 ~ /\.MOD$/ || f1 ~ /\.FORGET$/ || index(f1, ".COPY=") || index(f1, "CATEGORY=Internal")==1) continue
    if (kind=="class" && index(f1,"CLASS:")!=1) continue
    print kind "\t" baseof(f1)
  }
  close(p) }
END { exit bad }' "$T/files.tsv" | sort -u > "$T/declared.tsv" || exit 1
awk -F'\t' -v DECL="$T/declared.tsv" "$PRE"'
BEGIN { setup(); while ((getline d < DECL) > 0) { split(d, D, "\t"); DEC[D[1] "\t" D[2]] = 1 } }
{ b=$1; p=$2; s=suffix(p); BOOKS[b]=1; kind=(s in KIND)?KIND[s]:""
  bk=b; sub(/.*\//, "", bk)
  while ((getline line < p) > 0) {
    if (!isrow(line)) continue
    R[b]++
    n=split(line, F, "\t"); f1=F[1]
    if (kind=="") { C[b,"engine_config_file"]++; continue }
    internal=(index(f1,"CATEGORY=Internal")==1)
    for (i=2;i<=n;i++) if (F[i]=="CATEGORY:Internal") internal=1
    if (internal) { C[b,"internal_namespace"]++; continue }
    if (kind=="class" && index(f1,"CLASS:")!=1) { C[b,"class_level_line"]++; continue }
    if (f1 ~ /\.FORGET$/) { C[b,"forget_directive"]++; continue }
    disp=""
    if (f1 ~ /\.MOD$/) {
      nm=baseof(f1)
      if ((kind "\t" nm) in DEC) { C[b,"mod_record"]++; continue }
      disp="unit_mod_only_rescue"
    } else if (index(f1, ".COPY=")) {
      nm=f1; sub(/^CATEGORY=[^|]*\|/,"",nm); nm=substr(nm, index(nm,".COPY=")+6); disp="unit_copy"
    } else if (f1 ~ /^[A-Z][A-Z0-9_ ]*:/ && index(f1,"CLASS:")!=1) { C[b,"directive_line"]++; continue }
    else { nm=baseof(f1); disp="unit_declared" }
    key=nm
    for (i=2;i<=n;i++) if (index(F[i],"KEY:")==1) { key=substr(F[i],5); break }
    slug=tolower(key); gsub(/[^a-z0-9]+/,"_",slug); gsub(/^_+|_+$/,"",slug); if (slug=="") slug="unnamed"
    uid=bk ":" kind ":" slug
    if (uid in SEEN) { C[b, (kind=="class")?"class_continuation_row":"duplicate_key_row"]++; continue }
    SEEN[uid]=1; C[b,disp]++
  }
  close(p) }
END {
  nd=split("unit_declared unit_copy unit_mod_only_rescue mod_record forget_directive class_level_line class_continuation_row duplicate_key_row internal_namespace directive_line engine_config_file", DN, " ")
  for (b in BOOKS) { print b "\trows\t" R[b]+0; for (i=1;i<=nd;i++) print b "\t" DN[i] "\t" C[b,DN[i]]+0 }
}' "$T/files.tsv" | sort
# excluded rows by name
cnt() { find "$@" -type f -name '*.lst' -print0 | xargs -0 cat | awk '/^[^# \t\r]/ && !/^(SOURCELONG|SOURCESHORT|SOURCEWEB|SOURCEDATE)/{r++} END{print r+0}'; }
printf 'EXCLUDED\tcore _society (SFS guide core mods)\t%s\n' "$(cnt "$SF/paizo/core/_society")"
printf 'EXCLUDED\tstarfinder_society_rules (SSRGG)\t%s\n' "$(cnt "$SF/paizo/starfinder_society_rules")"
printf 'EXCLUDED\tlpj_design/infinite_space (LPJ9304)\t%s\n' "$(cnt "$SF/lpj_design/infinite_space")"
