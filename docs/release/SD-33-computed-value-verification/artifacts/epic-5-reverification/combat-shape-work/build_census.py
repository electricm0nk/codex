#!/usr/bin/env python3
"""Full corpus-record census for the combat/weapon lane's 125-unit
population (92 COMBAT + 18 WEAPON + 15 WEAPONPROF=*). Reads every whole
record (raw_bonus_chains AND every other field), not a filtered view."""
import json, glob

pop = json.load(open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/combat-shape-work/population.json'))
all_ids = pop['COMBAT'] + pop['WEAPON'] + pop['WEAPONPROF']
assert len(all_ids) == 125, len(all_ids)

book_index = {}
for path in glob.glob('data/corpus/*/equipment/**/*.json', recursive=True):
    book_index[(path.split('/')[2], path.split('/')[-1][:-5])] = path
BOOK_ALIASES = {'bestiary': 'beastiary'}
for (book, key), path in list(book_index.items()):
    for canon, actual in BOOK_ALIASES.items():
        if book == actual:
            book_index.setdefault((canon, key), path)

out = {}
missing = []
for uid in all_ids:
    parts = uid.split(':')
    book, key = parts[0], parts[2]
    path = book_index.get((book, key))
    if not path:
        missing.append(uid)
        continue
    rec = json.load(open(path))
    out[uid] = {'path': path, 'record': rec}

print('resolved', len(out), 'missing', len(missing))
if missing:
    print('MISSING:', missing)

json.dump(out, open('docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/combat-shape-work/census-full-records.json', 'w'), indent=2)
