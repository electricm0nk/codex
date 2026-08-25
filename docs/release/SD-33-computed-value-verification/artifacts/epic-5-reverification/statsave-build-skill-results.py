#!/usr/bin/env python3
"""sd33-r3-statsave: builds per-unit (ours, oracle, verdict) rows for this
lane's SKILL population from the real oracle exports
(`/tmp/statsave-oracle-out-skill/<slug>.txt`) and the real "ours" values
already computed (`e5_statsave_skill_ours`, `/tmp/skill_ours_output.json`).
"""
import json

manifest = json.load(open('/tmp/skill_manifest.json'))
manifest_by_id = {m['unit_id']: m for m in manifest}
ours = json.load(open('/tmp/skill_ours_output.json'))

rows = []
missing_export = []
for uid, ov in ours.items():
    slug = uid.split(':')[-1]
    m = manifest_by_id[uid]
    try:
        text = open(f'/tmp/statsave-oracle-out-skill/{slug}.txt').read()
    except FileNotFoundError:
        missing_export.append(uid)
        continue
    oracle_val = None
    for line in text.splitlines():
        if line.startswith('SKILL.MISC='):
            v = line.split('=', 1)[1].strip()
            try:
                oracle_val = int(v)
            except ValueError:
                oracle_val = None
            break
    if oracle_val is None:
        missing_export.append(uid)
        continue
    ours_val = ov['ours']
    verdict = 'agree' if ours_val == oracle_val else 'disagree'
    rows.append({
        'unit_id': uid,
        'ours': ours_val,
        'oracle': oracle_val,
        'verdict': verdict,
        'note': f'target_skill={m["target_skill"]} (first named of "{m["skill_field"]}"), declared_bonus={m["declared_bonus"]}, engine_raw_skill_field={ov["engine_raw_skill_field"]!r}',
    })

print('rows built:', len(rows), 'missing export (not yet run / failed):', len(missing_export))
if missing_export:
    print('MISSING:', missing_export)
from collections import Counter
print(Counter(r['verdict'] for r in rows))
json.dump({'rows': rows, 'missing_export': missing_export}, open('/tmp/skill_rows.json', 'w'), indent=1)
