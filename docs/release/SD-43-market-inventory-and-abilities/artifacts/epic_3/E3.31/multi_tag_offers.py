"""E3.31: every converted offer whose OptionSet::Rules carries 2+ tags, with its member counts
under all-of vs any-of (principal rules of the offered pool, TYPE matched without case).
Run from the repo root: python3 <this file>."""
import json, glob, collections
rules = []
for f in glob.glob('data/sheet_rules/*/*/*.json'):
    try:
        d = json.load(open(f))
    except Exception:
        continue
    if isinstance(d, list):
        rules += [r for r in d if isinstance(r, dict) and 'id' in r]
by_pool = collections.defaultdict(list)
for r in rules:
    if '#' not in r['id']:
        by_pool[r.get('pool')].append({t.lower() for t in r.get('tags', [])})
rows, lists = [], collections.Counter()
for r in rules:
    o = r.get('offers') or {}
    rs = (o.get('from') or {}).get('Rules') if isinstance(o.get('from'), dict) else None
    if not rs or len(rs['tags']) < 2:
        continue
    tags = [t.lower() for t in rs['tags']]
    members = by_pool.get(rs['pool'], [])
    all_of = sum(all(t in m for t in tags) for m in members)
    any_of = sum(any(t in m for t in tags) for m in members)
    key = (rs['pool'], tuple(rs['tags']))
    lists[key] += 1
    rows.append((r['id'], o['id'] == r['id'], rs['pool'], rs['tags'], all_of, any_of))
print(len(rows), 'offers with 2+ tags;', len(lists), 'distinct (pool, tags) lists')
for (pool, tags), n in sorted(lists.items()):
    tl = [t.lower() for t in tags]
    m = by_pool.get(pool, [])
    print(f"{n:3d} | {pool} | {' + '.join(tags)} | all-of {sum(all(t in x for t in tl) for x in m)} | any-of {sum(any(t in x for t in tl) for x in m)}")
print('--- offers')
for row in sorted(rows):
    print(' | '.join(map(str, row)))
