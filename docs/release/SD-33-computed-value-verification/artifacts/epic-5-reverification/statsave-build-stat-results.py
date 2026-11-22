#!/usr/bin/env python3
"""sd33-r3-statsave: builds per-unit (ours, oracle, verdict) rows for this
lane's STAT_multi_or_other_slot population from the real oracle exports
already run (`/tmp/statsave-oracle-out/<slug>.txt`, `PCGEN_REPO_DIR`-driven
BatchExporter runs against `fixtures/e5-equip-stats.txt.ftl`) and the real
"ours" values already computed (`e5_literal_stat_ours`, unmodified binary,
`/tmp/stat_ours_output.json`).
"""
import json, sys

manifest = json.load(open('/tmp/stat_manifest.json'))
ours = json.load(open('/tmp/stat_ours_output.json'))

rows = []
missing_export = []
for item in manifest:
    uid = item['unit_id']
    slug = uid.split(':')[-1]
    idx = item['ability_index']
    ours_key, ours_val = ours[uid]
    try:
        text = open(f'/tmp/statsave-oracle-out/{slug}.txt').read()
    except FileNotFoundError:
        missing_export.append(uid)
        continue
    oracle_val = None
    for line in text.splitlines():
        if line.startswith(f'STAT.{idx}.SCORE='):
            oracle_val = int(line.split('=', 1)[1])
            break
    if oracle_val is None:
        missing_export.append(uid)
        continue
    verdict = 'agree' if ours_val == oracle_val else 'disagree'
    rows.append({
        'unit_id': uid,
        'ours': ours_val,
        'oracle': oracle_val,
        'verdict': verdict,
        'note': f'target_ability={item["target_ability"]} (first named of "{item["ability_field"]}"), declared_bonus={item["declared_bonus"]}',
    })

print('rows built:', len(rows), 'missing export:', len(missing_export))
if missing_export:
    print('MISSING:', missing_export)
from collections import Counter
print(Counter(r['verdict'] for r in rows))
json.dump(rows, open('/tmp/stat_rows.json', 'w'), indent=1)
