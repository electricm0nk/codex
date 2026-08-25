#!/usr/bin/env python3
"""Full re-derivation of the 448 other_bonus_shape units' real bonus-chain
shapes -- every chain's first qualifier, counted per unit (a unit counted
once per distinct first-qualifier label it carries, matching the prior
wave's own counting convention so the two numbers are comparable)."""
import json, glob
from collections import Counter

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
label_counts = Counter()
unit_label_sets = {}
for uid in other_ids:
    u = units_by_id[uid]
    parts = uid.split(':')
    path = book_index.get((parts[0], parts[2]))
    d = json.load(open(path))['data']
    chains = d.get('raw_bonus_chains', [])
    labels = set()
    for c in chains:
        q = c['qualifiers']
        label = q[0]
        if label == 'STAT':
            label = 'STAT_multi_or_other_slot'
        labels.add(label)
    unit_label_sets[uid] = labels
    for l in labels:
        label_counts[l] += 1

print('other_bonus_shape population:', len(other_ids))
print('by first-qualifier label (unit counted once per distinct label it carries):')
for k, v in label_counts.most_common(40):
    print(f'  {v:6d}  {k}')

with open('/tmp/e5rem/full448-labels.json', 'w') as f:
    json.dump({k: v for k, v in unit_label_sets.items()} and {uid: sorted(labels) for uid, labels in unit_label_sets.items()}, f, indent=2)
