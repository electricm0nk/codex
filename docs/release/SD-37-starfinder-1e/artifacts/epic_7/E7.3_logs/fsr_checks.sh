#!/usr/bin/env bash
# E7.3 FSR revisit checks. Run from the repo root. Each section prints its command's output.
set -u
B=20bf84a3b2   # tranche/17 cut point (decisions.md §2)
echo "## DEF-1 (decisions.md §17 fenced command, verbatim)"
bash -c '
eval "$(scripts/fetch-pcgen-oracle.sh --check --quiet)" || { echo ORACLE_UNAVAILABLE; exit 2; }
test -f "$PCGEN_REPO_DIR/data/starfinder/paizo/core/_starfinder_core_rulebook.pcc" || { echo ORACLE_SF_MISSING; exit 2; }
test -d "$PCGEN_REPO_DIR/data/starfinder/paizo/core/starship" && echo "STARSHIP_DIR present"
awk '"'"'/^(ABILITY|EQUIPMENT|RACE|KIT):[^\t]*starship/{print FILENAME": "FNR": "$0}'"'"' "$PCGEN_REPO_DIR"/data/starfinder/*/*/*.pcc
'; echo "DEF-1 exit=$?"
eval "$(scripts/fetch-pcgen-oracle.sh --check --quiet)"
echo "oracle HEAD $(git -C "$PCGEN_REPO_DIR" rev-parse HEAD)"
echo "-- DEF-1 matcher sanity (same awk, '#' allowed: the commented-out add-on lines it must skip):"
awk '/^#?(ABILITY|EQUIPMENT|RACE|KIT):[^\t]*starship/{print FILENAME": "FNR": "$0}' "$PCGEN_REPO_DIR"/data/starfinder/*/*/*.pcc | awk -F'/data/starfinder/' '{print $2}'

echo; echo "## FSR-C1 open SD-34 fable-review P1 rows"
for f in apps/desktop/src-tauri/src/character_hub.rs src/saved_character/local_store.rs src/bin/v06_work_inventory.rs src/rules_core/pilot_compute/mod.rs scripts/transcribe_companion_tables.py apps/desktop/src-tauri/src/reach_gate.rs; do
  printf '%s exists=%s sd37_numstat=%s\n' "$f" "$(test -e "$f" && echo y || echo n)" "$(git diff --numstat $B HEAD -- "$f" | awk '{print "+"$1"/-"$2}')"
done
awk -F'|' '/^\| (desktop-P1|PC8-|PC4-1|R12-01)/{print $2"|"$3}' docs/release/SD-36-consolidation/receipts/epic-e_receipt.md
git log --format='%h %s' --diff-filter=D -1 -- src/bin/v06_work_inventory.rs
git log --format='%h %s' --diff-filter=D -1 -- apps/desktop/src-tauri/src/reach_gate.rs

echo; echo "## FSR-C2 prestige_mix_computed (E7.2 verify log on 2443fb2ac4)"
awk '/PASS  class-census/' docs/release/SD-37-starfinder-1e/artifacts/epic_7/E7.2_logs/verify-full.out

echo; echo "## FSR-C3 FS-27 pin ran in E7.2 desktop; FS-28 inputs unchanged since $B"
awk '/level_up_options_name_the_engines_mix_refusal_before_accept \.\.\./' docs/release/SD-37-starfinder-1e/artifacts/epic_7/E7.2_logs/desktop.log
echo "data/sheet_rules diff lines since $B: $(git diff $B HEAD -- data/sheet_rules | awk 'END{print NR}')"
echo "fact_words/weapon_words body lines changed since $B: $(git diff -U0 $B HEAD -- src/rules_core/sheet_rule_catalog.rs | awk '/^@@/{print}' )"

echo; echo "## FSR-C4 SF (kind, slug) collisions within Starfinder"
python3 - <<'PY'
import json,collections,glob
d=json.load(open('docs/work-inventory.starfinder-1e.json'))
c=collections.defaultdict(set)
for u in d['units']:
    c[(u['kind'],u['id'].split(':',2)[2])].add(u['book'])
coll=sorted(k for k,v in c.items() if len(v)>1)
print('A inventory: units',len(d['units']),'distinct (kind,slug)',len(c),'held in 2+ books',len(coll))
g=collections.defaultdict(dict)
for f in glob.glob('data/starfinder-1e/sheet_rules/*/*/*.json'):
    x=json.load(open(f))
    if not x: continue
    r=(x if isinstance(x,list) else [x])[0]
    p=r.get('id','').split(':')
    if len(p)!=3: continue
    g[(p[1],p[2])][p[0]]=json.dumps(r.get('value'),sort_keys=True)
c2=sorted(k for k,v in g.items() if len(v)>1)
diff=sorted(k for k,v in g.items() if len(v)>1 and len(set(v.values()))>1)
print('B sheet_rules: (kind,slug)',len(g),'held in 2+ books',len(c2),'differing value',len(diff),diff)
print('A == B:',coll==c2)
for k in coll: print('  ',k,sorted(c[k]))
PY

echo; echo "## FSR-C5 E7.1 oracle-runner change shape since $B"
git diff --name-status $B HEAD -- scripts/oracle_harness scripts/pcgen-run-character.sh | awk '{s[$1]++} END{for(k in s) print k, s[k]}'

echo; echo "## FSR-C6 SF ACP signature"
awk '/pub fn armor_check_penalty/' src/rules_core/pilot_compute/sf_defense.rs

echo; echo "## FSR-C7 residue gate diff since $B (lines)"
git diff $B HEAD -- scripts/pcgen_residue_gate.py | awk 'END{print NR}'
