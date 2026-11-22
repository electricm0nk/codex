#!/usr/bin/env python3
"""Full-population census of the literal-verified `equipment` kind's
STAT-ability-enhancement shape (SD-33 AT-33-E5-002 remediation).

A unit qualifies if it carries AT LEAST ONE `STAT|<single ability>|<n>|...`
bonus chain and its `TYPE` token names a `Belt` or `Headband` slot -- this is
exactly the shape `AT-33-E5-001`/`AT-33-E5-002` attempt 1 proved end-to-end
(the ability-score check reads `STAT.<i>.SCORE`, which is correct regardless
of whatever OTHER unrelated bonus chains (VAR combat-maneuver bonuses,
ABILITYPOOL skill choices, ...) the same record also carries -- those are a
different check, out of scope here, and `compute_magic_items_effect`
(`src/rules_core/equipment_effects/magic_items.rs`) already ignores them by
construction (`find_map` over the first STAT chain)).

Multi-ability STAT chains (`STAT|DEX,CON|...`) are excluded -- our own
`compute_magic_items_effect` stores the raw multi-ability string verbatim in
`AbilityScoreBonus.ability` rather than splitting it, so there is no single
`STAT.<i>.SCORE` token to compare against; that is a separate, harder shape,
named but not attempted this cycle.
"""
import json, glob, os, sys

inv = json.load(open('docs/work-inventory.json'))
units = [u for u in inv['units'] if u.get('status') == 'literal-verified' and u.get('kind') == 'equipment']

book_index = {}
for path in glob.glob('data/corpus/*/equipment/**/*.json', recursive=True):
    book_index[(path.split('/')[2], path.split('/')[-1][:-5])] = path

# See partition_literal_equipment.py's identical alias for why: work-inventory
# spells this book "bestiary"; the corpus directory on disk is "beastiary".
BOOK_ALIASES = {'bestiary': 'beastiary'}
for (book, key), path in list(book_index.items()):
    for canon, actual in BOOK_ALIASES.items():
        if book == actual:
            book_index.setdefault((canon, key), path)

result = []
multi_ability_ids = []
missing = []
for u in units:
    parts = u['id'].split(':')
    path = book_index.get((parts[0], parts[2]))
    if path is None:
        missing.append(u['id'])
        continue
    d = json.load(open(path))['data']
    chains = d.get('raw_bonus_chains', [])
    stat_chains = [c['qualifiers'] for c in chains if c['qualifiers'][0] == 'STAT']
    if not stat_chains:
        continue
    q = stat_chains[0]
    ability = q[1]
    if ',' in ability:
        multi_ability_ids.append(u['id'])
        continue
    type_val = ''
    for t in d.get('raw_tokens', []):
        if t['key'] == 'TYPE' and isinstance(t.get('value'), str):
            type_val = t['value']
            break
    slot = None
    if 'Belt' in type_val:
        slot = 'Belt'
    elif 'Headband' in type_val:
        slot = 'Headband'
    if slot is None:
        continue
    bonus = int(q[2])
    result.append({
        'unit_id': u['id'],
        'book': parts[0],
        'key': d['key'],
        'ability': ability,
        'bonus': bonus,
        'slot': slot,
        'corpus_path': path,
    })

print('literal-verified equipment total:', len(units))
print('missing corpus file:', len(missing), missing)
print('single-ability STAT + Belt/Headband (this shape):', len(result))
print('multi-ability STAT excluded (different shape, named not attempted):', len(multi_ability_ids))

if len(sys.argv) > 1:
    with open(sys.argv[1], 'w') as f:
        json.dump({'items': result, 'multi_ability_excluded': multi_ability_ids, 'missing_corpus_file': missing}, f, indent=2)
    print('wrote', sys.argv[1])
