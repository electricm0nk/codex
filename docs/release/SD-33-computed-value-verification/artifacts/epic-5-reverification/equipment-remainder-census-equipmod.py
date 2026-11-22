#!/usr/bin/env python3
"""Real, whole-record classification of the 46 literal-verified
`equipment_modifier` units (SD-33 remediation wave 2, equipment lane).
Reads each corpus record's raw_bonus_chains (and raw_tokens, for a defect
check) directly -- not filtered to any one token family."""
import json, glob

inv = json.load(open('docs/work-inventory.json'))
units = [u for u in inv['units'] if u.get('status') == 'literal-verified' and u.get('kind') == 'equipment_modifier']
print('equipment_modifier population:', len(units))

book_index = {}
for path in glob.glob('data/corpus/*/equipment/**/*.json', recursive=True):
    book_index[(path.split('/')[2], path.split('/')[-1][:-5])] = path

result = []
missing = []
for u in units:
    parts = u['id'].split(':')
    book, key = parts[0], parts[2]
    path = book_index.get((book, key))
    if path is None:
        missing.append(u['id'])
        continue
    d = json.load(open(path))['data']
    chains = d.get('raw_bonus_chains', [])
    result.append({
        'unit_id': u['id'],
        'book': book,
        'corpus_key': d.get('key'),
        'corpus_path': path,
        'chains': [c['qualifiers'] for c in chains],
        'has_chain': bool(chains),
    })

print('missing corpus file:', len(missing), missing)
no_chain = [x for x in result if not x['has_chain']]
has_chain = [x for x in result if x['has_chain']]
print('no bonus chain at all (genuinely no computable magnitude):', len(no_chain))
print('has >=1 bonus chain (real magnitude source):', len(has_chain))
for x in has_chain:
    print(' ', x['unit_id'], x['chains'])

with open('/tmp/e5rem/equipmod-census.json', 'w') as f:
    json.dump({'items': result, 'missing_corpus_file': missing}, f, indent=2)
