#!/usr/bin/env python3
"""E6.6: inserts the Starfinder seed rows into apps/desktop/scripts/ui-smoke/spec.json (a text insert
before `browse-starfinder-catalog`, so the rest of the file stays byte-identical).

  python3 E6.6_rows.py            # insert build-/open- rows for the four SF seeds + the two PF open rows
  python3 E6.6_rows.py --print    # print the rows' expect lists only

Every number a row expects is parsed from the E0.4 / E4.5 hand-value tables
(artifacts/epic_0/seed-hand-values.md, artifacts/epic_4/E4.5-loadout-hand-values.md), never typed here.
Refuses to run when a row id already exists."""
import json, re, sys
ROOT = '/home/ubuntu/workspace/worktrees/codex-sd37'
SPEC = ROOT + '/apps/desktop/scripts/ui-smoke/spec.json'
HAND = ROOT + '/docs/release/SD-37-starfinder-1e/artifacts/epic_0/seed-hand-values.md'
LOAD = ROOT + '/docs/release/SD-37-starfinder-1e/artifacts/epic_4/E4.5-loadout-hand-values.md'

def c(t): return {'op': 'click', 'target': t}
def w(ms): return {'op': 'wait', 'ms': ms}
def sel(t, v, index=None):
    s = {'op': 'select', 'target': t, 'text': v}
    if index is not None: s['index'] = index
    return s

# ---------------------------------------------------------------- hand values
def hand_values():
    out = {}
    for path in (HAND, LOAD):
        for line in open(path):
            m = re.match(r'\| (SF-[A-Za-z]+-\d) \| ([^|]+?) \| ([^|]+?) \|', line)
            if m: out.setdefault(m.group(1), {})[m.group(2)] = m.group(3).replace('−', '-')
    return out

def title(slug): return ' '.join(x.capitalize() for x in slug.split())

def expectations(seed, hv):
    v = hv[seed]
    e = []
    for field, label in [('BAB', 'Base Attack Bonus'), ('Fort', 'Fortitude'), ('Ref', 'Reflex'), ('Will', 'Will'),
                         ('HP', 'Hit Points'), ('Stamina', 'Stamina Points'), ('Resolve', 'Resolve Points'),
                         ('EAC', 'Energy Armor Class (EAC)'), ('KAC', 'Kinetic Armor Class (KAC)'),
                         ('Starting credits', 'Starting credits'), ('Credits spent', 'Credits spent'),
                         ('Credits remaining', 'Credits remaining'), ('Bulk', 'Bulk carried'),
                         ('Bulk limit unencumbered', 'Unencumbered up to (bulk)'), ('Bulk limit overburdened', 'Overburdened above (bulk)')]:
        e.append(f'{label}\t{v[field]}')
    for field, val in v.items():
        if field.startswith('Skill: ') and not val.startswith('untrained'):
            e.append(f'{title(field[7:])}\t{val}')
    for lvl, name in (('1st', 1), ('2nd', 2)):
        if f'Spells per day: {lvl}' in v:
            e.append(f'Level {name}\t{v[f"Spells per day: {lvl}"]}\t{v[f"Spells known: {lvl}"]}\t')
    return e

# ---------------------------------------------------------------- steps
def create_steps(name, race, theme, cls, key, picks, points):
    st = [c('Starfinder 1e'), c('New\nCharacter'), c('sf-character-name'), {'op': 'type', 'text': name},
          sel('— choose a race —', race), sel('— choose a theme —', theme),
          sel('— choose a class —', cls), w(1000)]
    if key:
        st += [sel('— choose the key ability —', key), w(1000)]
    for t, v, *ix in picks:
        st += [sel(t, v, *ix), w(700)]
    for abbr, n in points:
        st += [c('+1 ' + abbr)] * n
    st += [w(1500), c('Create character'), w(3000), {'op': 'scroll', 'ticks': 40, 'direction': 'up'}, {'op': 'key', 'key': 'Shift'}]
    return st

def level_steps(ranks_by_level, increase):
    """ranks_by_level: {new_level: [(skill label, clicks)]}; increase: ability names at 5th."""
    st = []
    for lvl in sorted(ranks_by_level):
        st += [c('Level up'), w(2500)]
        if lvl == 5:
            for a in increase:
                st += [c('Increase ' + a), w(400)]
        for skill, n in ranks_by_level[lvl]:
            for _ in range(n):
                st += [c('+1 rank ' + skill), w(250)]
        st += [w(1500), c('Accept level-up'), w(3500)]
    return st

def choice_steps(feats, spells, gear):
    st = [c('Feats, spells and gear'), w(3000)]
    for f in feats:
        st += [sel('— add a feat —', f'core:feat:{f}'), w(1500)]
    for cls, lvl, slug in spells:
        st += [sel(f'— add a {cls} level {lvl} spell —', f'core:spell:{slug}'), w(1200)]
    for slug, how in gear:
        st += [sel('— choose equipment —', f'core:equipment:{slug}'), w(500), c('Add equipped' if how == 'e' else 'Add carried'), w(2000)]
    st += [c('Save choices'), w(3500)]
    return st

def ranks(plan):
    """plan: [(skill label, total ranks)] -> {level: [(skill, clicks)]} the way the level-up allows
    them (ranks <= level; a level owes the ranks of every level not yet taken, so L2 adds two)."""
    top = max(t for _, t in plan)
    by = {}
    for lvl in range(2, top + 1):
        by[lvl] = []
        for skill, total in plan:
            have = min(total, lvl - 1)
            want = min(total, lvl)
            if want - have > 0 or lvl == 2:
                by[lvl].append((skill, want - (0 if lvl == 2 else have)))
    return {lvl: [(s, n) for s, n in by[lvl] if n > 0] for lvl in by}

SEEDS = {
 'soldier': dict(seed='SF-Soldier-3', name='SF Seed Soldier', race='core:race:human', theme='core:ability:mercenary', cls='core:class:soldier', key='STR',
    picks=[('— choose: +2 Racial Stat Bonus —', 'STR')],
    points=[('Str', 3), ('Dex', 4), ('Con', 2), ('Int', 1)],
    plan=[('Athletics', 3), ('Intimidate', 3), ('Medicine', 3), ('Piloting', 3), ('Survival', 3)], increase=[],
    feats=['weapon_focus', 'quick_draw', 'deadly_aim', 'coordinated_shot'], spells=[],
    gear=[('defiance_series_squad', 'e'), ('laser_rifle_azimuth', 'e'), ('baton_tactical', 'c'), ('battery', 'c'), ('battery', 'c'), ('serum_of_healing_mk_1', 'c'), ('serum_of_healing_mk_1', 'c')]),
 'mystic': dict(seed='SF-Mystic-5', name='SF Seed Mystic', race='core:race:lashunta', theme='core:ability:priest', cls='core:class:mystic', key=None,
    picks=[('DIMORPHIC —', 'core:ability:lashunta_subrace_damaya'), ('Connection —', 'core:ability:empath'),
           ('Racial Bonus to Skill —', 'diplomacy', 0), ('Racial Bonus to Skill —', 'medicine', 1)],
    points=[('Dex', 2), ('Wis', 7), ('Cha', 1)],
    plan=[('Bluff', 5), ('Culture', 5), ('Diplomacy', 5), ('Life Science', 5), ('Medicine', 5), ('Mysticism', 5), ('Perception', 5), ('Sense Motive', 5)],
    increase=['Dexterity', 'Intelligence', 'Wisdom', 'Charisma'],
    feats=['spell_penetration', 'spell_focus', 'quick_draw'],
    spells=[('Mystic', 0, s) for s in ['detect_affliction', 'detect_magic', 'ghost_sound', 'grave_words', 'stabilize', 'telepathic_message']]
         + [('Mystic', 1, s) for s in ['charm_person', 'command', 'mystic_cure_level_1', 'share_language', 'detect_thoughts']]
         + [('Mystic', 2, s) for s in ['hold_person', 'remove_condition', 'status', 'zone_of_truth']],
    gear=[('lashunta_tempweave_basic', 'e'), ('laser_pistol_azimuth', 'e'), ('baton_tactical', 'c'), ('battery', 'c'), ('medkit_basic', 'c'), ('serum_of_healing_mk_1', 'c'), ('serum_of_healing_mk_1', 'c')]),
 'technomancer': dict(seed='SF-Technomancer-5', name='SF Seed Technomancer', race='core:race:android', theme='core:ability:scholar', cls='core:class:technomancer', key=None,
    picks=[('THEME KNOWLEDGE (scholar theme chosen skill picks) —', 'core:pool_option:scholar_theme_chosen_skill_physical_science')],
    points=[('Dex', 2), ('Con', 2), ('Int', 5), ('Wis', 1)],
    plan=[('Computers', 5), ('Engineering', 5), ('Life Science', 5), ('Mysticism', 5), ('Physical Science', 5), ('Piloting', 5), ('Sleight of Hand', 5), ('Perception', 5)],
    increase=['Dexterity', 'Constitution', 'Intelligence', 'Wisdom'],
    feats=['spell_penetration', 'spell_focus', 'mobility', 'quick_draw'],
    spells=[('Technomancer', 0, s) for s in ['dancing_lights', 'detect_magic', 'energy_ray', 'mending', 'token_spell', 'transfer_charge']]
         + [('Technomancer', 1, s) for s in ['detect_tech', 'magic_missile', 'overheat', 'supercharge_weapon']]
         + [('Technomancer', 2, s) for s in ['invisibility', 'knock', 'mirror_image']],
    gear=[('d_suit_i', 'e'), ('laser_pistol_azimuth', 'e'), ('battery', 'c'), ('battery', 'c'), ('medkit_basic', 'c'), ('serum_of_healing_mk_1', 'c'), ('serum_of_healing_mk_1', 'c')]),
 'envoy': dict(seed='SF-Envoy-3', name='SF Seed Envoy', race='core:race:ysoki', theme='core:ability:icon', cls='core:class:envoy', key=None,
    picks=[('Envoy Improvisation —', 'core:ability:envoy_improvisation_inspiring_boost')],
    points=[('Dex', 1), ('Con', 2), ('Cha', 7)],
    plan=[('Bluff', 3), ('Computers', 3), ('Culture', 3), ('Diplomacy', 3), ('Engineering', 3), ('Intimidate', 3), ('Perception', 3), ('Sense Motive', 3), ('Stealth', 3)],
    increase=[], feats=['mobility', 'quick_draw'], spells=[],
    gear=[('carbon_skin_graphite', 'e'), ('semi_auto_pistol_tactical', 'e'), ('baton_tactical', 'c'), ('serum_of_healing_mk_1', 'c'), ('serum_of_healing_mk_1', 'c')]),
}

def build_row(k, hv):
    d = SEEDS[k]
    st = create_steps(d['name'], d['race'], d['theme'], d['cls'], d['key'], d['picks'], d['points'])
    st += [c('Back'), c('Load\nCharacter'), c(d['name']), c('Load'), w(2000)]
    st += level_steps(ranks(d['plan']), d['increase'])
    st += choice_steps(d['feats'], d['spells'], d['gear'])
    v = hv[d['seed']]
    return {'id': f'build-starfinder-{k}', 'screen': 'sheet', 'steps': st, 'marker': 'Saved feats, spells and gear:',
            'expect': [f"Energy Armor Class (EAC) {v['EAC']} · Kinetic Armor Class (KAC) {v['KAC']} · Credits remaining {v['Credits remaining']} · Bulk carried {v['Bulk']}"],
            'forbid': ['This character did not compute', 'Failed to', 'over_budget', '☰ Menu'],
            'notes': f"SD-37 E6.6: {d['seed']} built end to end through the real app, every step a click or a select: the creation flow (race, theme, class, point buy {', '.join(f'{n} {a}' for a, n in d['points'])}), load, Level up to level {max(t for _, t in d['plan'])} with the skill ranks of seed-builds.md and the 5th-level increase where the seed has one, then 'Feats, spells and gear' (feats, spells known, gear) and 'Save choices'. The sheet prints the engine's totals; EAC, KAC, credits remaining and bulk are the hand values of seed-hand-values.md / E4.5-loadout-hand-values.md. Used as the setup of open-starfinder-{k}; landing-select-pathfinder restores the Pathfinder selection."}

def open_row(k, hv):
    d = SEEDS[k]
    return {'id': f'open-starfinder-{k}', 'screen': 'sheet', 'setup': [f'build-starfinder-{k}'],
            'steps': [c('Open'), c(d['name']), c('Load'), w(2500)],
            'marker': 'Kinetic Armor Class (KAC)', 'expect': [f"{d['name']}"] + expectations(d['seed'], hv),
            'forbid': ['This character did not compute', 'CMB', 'CMD', 'Flat-Footed', '☰ Menu'],
            'notes': f"SD-37 E6.6: {d['seed']} opened from the saved store. The setup builds it through the real app (build-starfinder-{k}); this row clicks the sheet's Open (back to the Load Character list), selects '{d['name']}' and loads it, so every figure is read back from the saved character, not from the build's own screen. `expect` is every sheet total that E0.4/E4.5 hand-values for the seed, in the sheet's own label and sign format (a trained-only skill with no ranks has no hand value and no expectation), parsed from the hand-value tables by artifacts/epic_6/E6.6_rows.py. landing-select-pathfinder restores the Pathfinder selection."}

def pf_rows():
    return [
     {'id': 'open-seed-aldric', 'screen': 'sheet', 'steps': [c('Load\nCharacter'), c('Aldric Ironhand'), c('Load'), w(3000)],
      'marker': '☰ Menu', 'expect': ['Aldric Ironhand', 'Fighter 3', '☰ Menu'],
      'notes': 'SD-37 E6.6: the first Pathfinder starter seed, Aldric Ironhand (Human Fighter 3), opened from the isolated store (the app seeds it on first launch; load-seed-list-both lists it). With open-seed-elowen and the four open-starfinder-* rows this is the six seeds of E6.6.'},
     {'id': 'open-seed-elowen', 'screen': 'sheet', 'steps': [c('Load\nCharacter'), c('Elowen Ashgrave'), c('Load'), w(3000)],
      'marker': '☰ Menu', 'expect': ['Elowen Ashgrave', 'Wizard 5', '☰ Menu'],
      'notes': 'SD-37 E6.6: the second Pathfinder starter seed, Elowen Ashgrave (Human Wizard 5), opened from the isolated store. load-seed-wizard-fireball (the last row of the spec) additionally opens her Spells tab.'},
    ]

def main():
    hv = hand_values()
    rows = []
    for k in SEEDS:
        rows += [build_row(k, hv), open_row(k, hv)]
    pf = pf_rows()
    if '--print' in sys.argv:
        for r in rows + pf: print(r['id'], len(r['steps']), len(r.get('expect', [])))
        print(json.dumps(open_row('mystic', hv)['expect'])); return
    raw = open(SPEC).read()
    for r in rows + pf:
        assert f'"id": "{r["id"]}"' not in raw, r['id']
    def fmt(rs): return ',\n'.join('    ' + json.dumps(r, indent=2, ensure_ascii=False).replace('\n', '\n    ') for r in rs)
    anchor = '    {\n      "id": "browse-starfinder-catalog"'
    assert raw.count(anchor) == 1
    raw = raw.replace(anchor, fmt(rows) + ',\n' + anchor)
    # The Pathfinder open rows go last: they need the landing's Pathfinder selection landing-select-pathfinder restores.
    tail = '\n    }\n  ]\n}\n'
    assert raw.endswith(tail)
    raw = raw[:-len(tail)] + '\n    },\n' + fmt(pf) + '\n  ]\n}\n'
    open(SPEC, 'w').write(raw)
main()
