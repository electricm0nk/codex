"""SD-40 E2.G round-1 replan census (2026-11-11). Run from the repo root:
    python3 -I docs/release/SD-40-pf-sheet-design/artifacts/epic_2/replan-r1/census.py
Reads the converted rule package (data/sheet_rules/<book>/<kind>/*.json) and counts, per bonus
target, the rules whose `applies` gate holds a Situational clause (printed with its condition, never
folded into a total), and the prose entries per `Aspect` family (PCGen ASPECT: lines such as
SaveBonus, the words a race trait like Dwarf Hardy carries). Read-only."""
import collections, glob, json

sit = collections.Counter(); sit_kind = collections.Counter(); first = {}
aspects = collections.Counter(); files = 0; rules = 0
for path in sorted(glob.glob('data/sheet_rules/*/*/*.json')):
    try:
        rows = json.load(open(path))
    except (ValueError, UnicodeDecodeError):
        continue
    files += 1
    for rule in rows:
        rules += 1
        for prose in rule.get('prose') or []:
            family = prose.get('family')
            if isinstance(family, dict) and 'Aspect' in family:
                aspects[family['Aspect']] += 1
        target = rule.get('target')
        if target is None or '"Situational"' not in json.dumps(rule.get('applies')):
            continue
        key = target if isinstance(target, str) else next(iter(target))
        if key == 'Save':
            key = 'Save:' + str(target['Save'])
        sit[key] += 1
        sit_kind[(key, rule['id'].split(':')[1])] += 1
        first.setdefault(key, rule['id'])
print(f'files {files} rules {rules}')
print('situational rules by target (target count first-example):')
for key in ['Hp', 'Initiative', 'Ac', 'Save:Fortitude', 'Save:Reflex', 'Save:Will', 'Cmd', 'Cmb', 'Speed']:
    print(f'  {key} {sit.get(key, 0)} {first.get(key, "-")}')
print('aspect prose entries (save/combat/AC families):')
for key in ['SaveBonus', 'AllSaveBonus', 'Save Bonus', 'WillSaveBonus', 'RefSaveBonus', 'CombatBonus', 'ModifyAC']:
    print(f'  {key} {aspects.get(key, 0)}')
