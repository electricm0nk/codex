#!/usr/bin/env python3
"""Census of the SKILL-shape sub-population within the `other_bonus_shape`
448-unit equipment group (SD-33 remediation wave 2, equipment-other-bonus-
shape lane). A unit qualifies if compute_general_effect's own selection
rule matches: the FIRST bonus chain whose qualifiers[0] == 'SKILL' (real
corpus fact, read the same way src/rules_core/equipment_effects/general.rs
reads it -- not a re-derived approximation)."""
import json, glob, sys

inv = json.load(open('docs/work-inventory.json'))
part = json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/literal-stat-shape/equipment-partition.json'))
other_ids = set(part['other_bonus_shape'])

book_index = {}
for path in glob.glob('data/corpus/*/equipment/**/*.json', recursive=True):
    book_index[(path.split('/')[2], path.split('/')[-1][:-5])] = path
BOOK_ALIASES = {'bestiary': 'beastiary'}
for (book, key), path in list(book_index.items()):
    for canon, actual in BOOK_ALIASES.items():
        if book == actual:
            book_index.setdefault((canon, key), path)

units_by_id = {u['id']: u for u in inv['units']}
units = [units_by_id[i] for i in other_ids if i in units_by_id]
assert len(units) == len(other_ids), (len(units), len(other_ids))

result = []
missing = []
for u in units:
    parts = u['id'].split(':')
    path = book_index.get((parts[0], parts[2]))
    if path is None:
        missing.append(u['id'])
        continue
    d = json.load(open(path))['data']
    chains = d.get('raw_bonus_chains', [])
    skill_chain = None
    for c in chains:
        q = c['qualifiers']
        if q[0] == 'SKILL' and len(q) >= 3:
            skill_chain = q
            break
    if skill_chain is None:
        continue
    try:
        bonus = int(skill_chain[2])
    except ValueError:
        continue
    result.append({
        'unit_id': u['id'],
        'book': parts[0],
        'key': d['key'],
        'skill': skill_chain[1],
        'bonus': bonus,
        'corpus_path': path,
    })

print('other_bonus_shape population:', len(units))
print('missing corpus file:', len(missing))
print('SKILL-shape (this census):', len(result))
if len(sys.argv) > 1:
    with open(sys.argv[1], 'w') as f:
        json.dump({'items': result, 'missing_corpus_file': missing}, f, indent=2)
    print('wrote', sys.argv[1])
