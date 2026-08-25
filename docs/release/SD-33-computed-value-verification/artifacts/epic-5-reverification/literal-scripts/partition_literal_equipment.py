#!/usr/bin/env python3
"""The authoritative, mutually-exclusive partition of the 5,170
literal-verified `equipment` units this cycle actually classified by real
execution over every real corpus file (SD-33 AT-33-E5-002 remediation).

Four groups, summing to 5,170 (`box_ledger.py`-style: uncovered=0,
overlap=0 by construction -- each unit is tested against the groups in a
fixed order and lands in exactly the first one that matches):

  1. stat_shape_examined      -- single-ability STAT|<ability>|n|... chain
                                  + Belt/Headband TYPE slot. Oracle-verified
                                  THIS cycle (41 units, see
                                  stat-shape-manifest.json /
                                  full41.oracle-results).
  2. no_bonus_chain           -- raw_bonus_chains is empty. Real corpus
                                  fact, established by reading every file:
                                  compute_equipment_effects has nothing to
                                  resolve for these (mundane gear whose
                                  only literal-verified magnitude is
                                  cost/weight, not a rules effect).
                                  Verdict: unverifiable, first-class,
                                  per-unit reason recorded.
  3. missing_corpus_file      -- the unit_id's corpus JSON was not found by
                                  a recursive `data/corpus/*/equipment/**/`
                                  search. A real anomaly, named, not
                                  silently folded anywhere.
  4. other_bonus_shape        -- has >=1 bonus chain but does not qualify
                                  for group 1 (multi-ability STAT, SKILL,
                                  COMBAT, VAR, WEAPON, SAVE, DC, SLOTS,
                                  SITUATION, WEAPONPROF, SPELLCAST*, MISC,
                                  HP, LOADMULT, MOVEMULT, MOVEADD,
                                  ABILITYPOOL, or a STAT chain in a
                                  non-Belt/Headband slot). Real magnitude
                                  probe exists (AT-33-E1-003) but not yet
                                  oracle-verified this cycle -- each needs
                                  its own template/token authoring, named
                                  per shape with a count.
"""
import json, glob, os, collections, sys

inv = json.load(open('docs/work-inventory.json'))
units = [u for u in inv['units'] if u.get('status') == 'literal-verified' and u.get('kind') == 'equipment']

book_index = {}
for path in glob.glob('data/corpus/*/equipment/**/*.json', recursive=True):
    book_index[(path.split('/')[2], path.split('/')[-1][:-5])] = path

# Real, execution-discovered book-id spelling mismatch: work-inventory.json
# spells this book "bestiary" (correct English spelling); the corpus
# directory on disk is "beastiary" (a pre-existing typo, unrelated to this
# cycle). 3 units were reported "missing" before this alias was added --
# re-derive with `find data/corpus -iname beastiary -o -iname bestiary`.
BOOK_ALIASES = {'bestiary': 'beastiary'}
for (book, key), path in list(book_index.items()):
    for canon, actual in BOOK_ALIASES.items():
        if book == actual:
            book_index.setdefault((canon, key), path)

stat_ids = set(json.load(open(sys.argv[1]))['unit_id'] for _ in [0]) if False else None
stat_manifest = json.load(open(sys.argv[1]))
stat_ids = set(x['unit_id'] for x in stat_manifest)

groups = collections.defaultdict(list)
other_shape_detail = collections.Counter()

for u in units:
    if u['id'] in stat_ids:
        groups['stat_shape_examined'].append(u['id'])
        continue
    parts = u['id'].split(':')
    path = book_index.get((parts[0], parts[2]))
    if path is None:
        groups['missing_corpus_file'].append(u['id'])
        continue
    d = json.load(open(path))['data']
    chains = d.get('raw_bonus_chains', [])
    if not chains:
        groups['no_bonus_chain'].append(u['id'])
        continue
    groups['other_bonus_shape'].append(u['id'])
    for c in chains:
        q = c['qualifiers']
        label = q[0]
        if label == 'STAT':
            label = 'STAT_multi_or_other_slot'
        other_shape_detail[label] += 1

total = sum(len(v) for v in groups.values())
print('literal-verified equipment population:', len(units))
print('partitioned total:', total, '(uncovered =', len(units) - total, ')')
for k in ('stat_shape_examined', 'no_bonus_chain', 'missing_corpus_file', 'other_bonus_shape'):
    print(f'  {len(groups[k]):6d}  {k}')
print()
print('other_bonus_shape by first-qualifier label (a unit can carry >1 chain, so this over-counts vs the group size):')
for k, v in other_shape_detail.most_common(30):
    print(f'  {v:6d}  {k}')

if len(sys.argv) > 2:
    with open(sys.argv[2], 'w') as f:
        json.dump({k: v for k, v in groups.items()}, f, indent=2)
    print('wrote', sys.argv[2])
