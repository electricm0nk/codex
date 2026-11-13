"""resolve.py <match.json> <index.json> <resolved.json>: every indexed page section or table row
(index.py) whose heading slug matches one of a row's slugs, with the price text the page states."""
import json, re, sys
match, index, out = sys.argv[1:4]
rows = json.load(open(match)); idx = json.load(open(index))
def slug(s):
    s = s.lower().replace("'", "").replace('’', '')
    return re.sub(r'[^a-z0-9]+', '-', s).strip('-')
by = {}
for e in idx:
    keys = {e['slug'], slug(re.sub(r'\s*\([^)]*\)|\*|†', '', e['name']))}
    if ',' in e['name']:
        a, b = e['name'].split(',', 1); keys.add(slug(b + ' ' + a))
    for k in keys: by.setdefault(k, []).append(e)
res = []
for r in rows:
    hits = []
    for s in r['slugs']:
        for e in by.get(s, []):
            if e not in hits: hits.append(e)
    res.append({**r, 'hits': [{k: e[k] for k in ('url', 'kind', 'name', 'price', 'fetched_at')} for e in hits]})
json.dump(res, open(out, 'w'), indent=1)
n = sum(1 for x in res if any(h['price'] for h in x['hits']))
print(len(res), 'with a priced hit', n)
