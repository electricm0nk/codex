"""Rows E4.7 added to the settled bundles, and every other field that moved. Run from the repo root: python3 <this> <base-sha>."""
import collections, glob, json, subprocess, sys
base = sys.argv[1]
rows, others = collections.Counter(), []
for path in sorted(glob.glob('data/corpus/*/_settled/equipment.json')) + sorted(glob.glob('apps/desktop/src-tauri/resources/corpus_fixtures/_settled/equipment.json')):
    old = subprocess.run(['git', 'show', f'{base}:{path}'], capture_output=True, text=True)
    if old.returncode:
        continue
    a, b = json.loads(old.stdout), json.load(open(path))
    if a == b:
        continue
    assert set(a['records']) == set(b['records']), path
    for key, wrapped in a['records'].items():
        record, new = wrapped['record'], b['records'][key]['record']
        if record == new:
            continue
        for field in set(record) | set(new):
            if record.get(field) != new.get(field):
                if field == 'stated_bonuses':
                    for row in new[field]:
                        if row not in record[field]:
                            rows[row['target']] += 1
                else:
                    others.append((path.split('/')[2], new.get('identity', key), field, record.get(field), new.get(field)))
print(sorted(rows.items()), sum(rows.values()))
print('other fields:', others)
