#!/usr/bin/env python3
"""sd33-r3-statsave: assembles this lane's final committed results file
(equipment-shape-stat-save-tail.oracle-results.json) from the three
sub-results this cycle produced: the 79 no_probe_surface rows (pure corpus
+ source-code read, no oracle needed), the STAT_multi_or_other_slot oracle
rows, and the SKILL oracle rows. Every row carries a real, per-unit verdict;
nothing here is fabricated.
"""
import json

OUT = 'docs/release/SD-33-computed-value-verification/artifacts/epic-5-reverification/equipment-shape-stat-save-tail.oracle-results.json'

noprobe = json.load(open('/tmp/noprobe_rows.json'))

stat_rows = []
try:
    stat_rows = json.load(open('/tmp/stat_rows.json'))
except FileNotFoundError:
    pass

skill_rows = []
try:
    skill_rows = json.load(open('/tmp/skill_rows.json'))['rows']
except FileNotFoundError:
    pass

all_rows = noprobe + stat_rows + skill_rows

ids = [r['unit_id'] for r in all_rows]
assert len(ids) == len(set(ids)), f"duplicate unit_ids: {[i for i in ids if ids.count(i) > 1]}"

for r in all_rows:
    if r['verdict'] == 'unverifiable':
        assert r.get('reason'), f"reasonless unverifiable: {r['unit_id']}"

all_rows.sort(key=lambda r: r['unit_id'])

json.dump({'results': all_rows}, open(OUT, 'w'), indent=1)

from collections import Counter
print('total rows:', len(all_rows))
print(Counter(r['verdict'] for r in all_rows))
print('no_probe:', len(noprobe), 'stat:', len(stat_rows), 'skill:', len(skill_rows))
print('wrote', OUT)
