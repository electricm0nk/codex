#!/usr/bin/env python3
"""Census the literal-verified equipment population by magnitude shape.
SD-33 AT-33-E5-002 remediation. Reads docs/work-inventory.json + the real
corpus records; classifies by raw_bonus_chains shape and TYPE slot so the
generator knows which items the proven single-ability STAT/Belt/Headband
mechanism (AT-33-E5-001/002 attempt 1) covers and which need a different
shape.
"""
import json, os, collections, sys, glob

inv = json.load(open('docs/work-inventory.json'))
units = [u for u in inv['units'] if u.get('status') == 'literal-verified' and u.get('kind') == 'equipment']

# Recursive index, once: data/corpus/<book>/equipment/**/<key>.json --
# equipment corpus is nested by category (arms_armor/equipmods/magic_items/
# general/...), a shallow single-level glob undercounts (workflow-instruction
# §4's "known hazard").
book_index = {}
for path in glob.glob('data/corpus/*/equipment/**/*.json', recursive=True):
    book = path.split('/')[2]
    key = os.path.splitext(os.path.basename(path))[0]
    book_index[(book, key)] = path

shape = collections.Counter()
missing_file = 0
qual_first_token = collections.Counter()
single_stat_ids = []

for u in units:
    parts = u['id'].split(':')
    path = book_index.get((parts[0], parts[2]))
    if path is None:
        missing_file += 1
        continue
    d = json.load(open(path))['data']
    chains = d.get('raw_bonus_chains', [])
    type_val = ''
    for t in d.get('raw_tokens', []):
        if t['key'] == 'TYPE' and isinstance(t.get('value'), str):
            type_val = t['value']
            break
    if not chains:
        shape['no_bonus_chain'] += 1
        continue
    if len(chains) == 1:
        q = chains[0]['qualifiers']
        qual_first_token[q[0]] += 1
        if q[0] == 'STAT':
            ability = q[1]
            is_single_ability = ',' not in ability
            has_belt = 'Belt' in type_val
            has_headband = 'Headband' in type_val
            if is_single_ability and (has_belt or has_headband) and len(q) >= 4 and 'Enhancement' in q[3]:
                shape['single_stat_belt_headband_enh'] += 1
                single_stat_ids.append(u['id'])
            elif is_single_ability:
                shape['single_stat_other_slot'] += 1
            else:
                shape['multi_stat'] += 1
        else:
            shape[f'single_chain_{q[0]}'] += 1
    else:
        shape[f'multi_chain_{len(chains)}'] += 1

print('literal-verified equipment total:', len(units))
print('missing_file:', missing_file)
print('total classified:', sum(shape.values()))
for k, v in shape.most_common(40):
    print(f'{v:6d}  {k}')
print()
print('qualifier first-token distribution (single-chain items):')
for k, v in qual_first_token.most_common(30):
    print(f'{v:6d}  {k}')

if len(sys.argv) > 1 and sys.argv[1] == '--dump-ids':
    out = sys.argv[2]
    with open(out, 'w') as f:
        json.dump(single_stat_ids, f)
    print('dumped', len(single_stat_ids), 'ids to', out)
    print('re-derive: python3', sys.argv[0])
