#!/usr/bin/env python3
"""E5.4 independent walk: the drone block's records for the Mechanic 1 parity build
(E5.4-mechanic-1-build.md), derived from the package JSON alone (no engine code).

Held = the drone race, then every rule granted by a held rule (Granter Rule/Choice) whose grant
gate and own gate hold at drone level 1, plus every chosen option of a held chooser.
Variables at level 1 (oracle scr_companionmods.lst:13): master DroneCompanionLVL = 1 ->
DroneMasterLVL = DroneLVL = 1; DroneMasterTotalLVL = master TL = 1; counters 0. A gate this
reader cannot decide is printed and excluded. Per-weapon rows (pool `weapon`) and non-printing
rules are dropped, as the print path drops them. Run from the repo root.
"""
import json, os, sys
PKG = 'data/starfinder-1e/sheet_rules'
names = json.load(open('data/starfinder-1e/var_names.json'))
VAL = {'DRONECOMPANIONLVL': 1, 'DRONEMASTERLVL': 1, 'DRONELVL': 1, 'DRONEMASTERTOTALLVL': 1, 'TL': 1}
CHOICES = {
    'core:companion_mod:drone': 'core:pool_option:drone_chassis_selection_hover',
    'core:companion_mod:drone#bonus1': 'core:pool_option:drone_skill_unit_perception',
    'core:companion_mod:drone#bonus2': 'core:pool_option:drone_feat_iron_will',
    'core:companion_mod:drone#bonus3': 'core:ability:drone_mod_camera',
    'core:ability:drone': 'core:ability:initial_drone_proficiency_small_arms',
}
rules = {}
for root, _, files in os.walk(PKG):
    if '/_' in root: continue
    for f in files:
        if f.endswith('.json'):
            d = json.load(open(os.path.join(root, f)))
            if not isinstance(d, list): continue
            for r in d:
                rules[r['id']] = r
undecided = []
def num(e):
    if 'Const' in e: return e['Const']
    for k in ('Var', 'MasterVar'):
        if k in e: return VAL.get(names.get(e[k], '?'), 0)
    if e == 'Level' or e == {'Level': None}: return 0
    raise ValueError(e)
def holds(a):
    if a == 'Always': return True
    if 'All' in a: return all(holds(x) for x in a['All'])
    if 'AtLeast' in a: return sum(holds(x) for x in a['AtLeast']['of']) >= a['AtLeast']['n']
    if 'Holds' in a and 'Rule' in a['Holds']['what']: return a['Holds']['what']['Rule'] in held  # E5.4 r2: an automatic grant's waiver
    if 'Compare' in a:
        c = a['Compare']; l, r = num(c['lhs']), num(c['rhs'])
        return {'Gte': l >= r, 'Gt': l > r, 'Lt': l < r, 'Lte': l <= r, 'Eq': l == r, 'Ne': l != r}[c['op']]
    raise ValueError(a)
def ok(a, who):
    try: return holds(a)
    except (ValueError, KeyError):
        undecided.append((who, json.dumps(a)[:120])); return False
held = {'core:race:drone'}
changed = True
while changed:
    changed = False
    for rid, r in rules.items():
        if rid in held or '#' in rid: continue
        for g in r.get('granted_by') or []:
            by = g['by'].get('Rule') or g['by'].get('Choice')
            if by in held and ('Rule' in g['by'] or CHOICES.get(by) == rid) and ok(g['when'], rid) and ok(r['applies'], rid):
                held.add(rid); changed = True; break
    for chooser, opt in CHOICES.items():
        if chooser.split('#')[0] in held and opt not in held and opt in rules and ok(rules[opt]['applies'], opt):
            held.add(opt); changed = True
printed = sorted(h for h in held if rules[h].get('print', True) and rules[h].get('pool') != 'weapon')
for h in printed: print(h)
print(f'TOTAL {len(printed)}', file=sys.stderr)
for u in sorted(set(undecided)): print('UNDECIDED', *u, file=sys.stderr)
