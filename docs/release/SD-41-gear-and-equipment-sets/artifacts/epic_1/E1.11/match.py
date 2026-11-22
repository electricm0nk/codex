"""match.py <census.json> <urls.txt> <match.json>: candidate d20pfsrd pages per unpriced Market row.
urls.txt = every <loc> of https://www.d20pfsrd.com/wp-sitemap.xml's page sitemaps (45,254 URLs on 2026-11-13).
A candidate is a page whose last path segment is a slug of the row's key, catalog name or a reordering
of them ("Dagger (Silver/Masterwork)" -> silver-dagger-masterwork), or that slug plus a short suffix."""
import json, re, sys, collections
census, urls_path, out = sys.argv[1:4]
d = json.load(open(census))
rows = d['none'] + d['zero'] + d['negative']
urls = [u.strip() for u in open(urls_path) if u.strip()]
by_slug = collections.defaultdict(list)
for u in urls:
    seg = u.rstrip('/').rsplit('/', 1)[-1]
    by_slug[seg].append(u)
def slug(s):
    s = s.lower().replace("'", "").replace('’', '')
    return re.sub(r'[^a-z0-9]+', '-', s).strip('-')
def names(r):
    out = []
    for n in (r['key'], r['catalog_name'], r['name']):
        out.append(n)
        base = re.sub(r'\s*\([^)]*\)', '', n).strip()
        out.append(base)
        m = re.match(r'(.*?)\s*\(([^)]*)\)$', n)
        if m:
            out.append(f"{m.group(2)} {m.group(1)}")
            out.append(f"{m.group(1)} {m.group(2)}")
        if ',' in n:
            a, b = n.split(',', 1); out.append(f"{b} {a}"); out.append(a)
        out.append(re.sub(r'^the\s+', '', n, flags=re.I))
        m = re.match(r'(.*?)\s*\(([^/)]*)/([^)]*)\)$', n)
        if m: out.append(f"{m.group(2)} {m.group(1)} {m.group(3)}")
        out.append(re.sub(r'\s*\+\d+', '', base))
        w = base.split()
        if len(w) == 2: out.append(f"{w[1]} {w[0]}")
    seen = []
    for n in out:
        for s in (slug(n), slug(n.replace("'", ' ').replace('’', ' '))):
            if s and s not in seen: seen.append(s)
    return seen
PREF = ['/magic-items/', '/equipment/', '/alternative-rule-systems/', '/bestiary/']
res = []
for r in rows:
    cands = []
    for s in names(r):
        for u in by_slug.get(s, []):
            if u not in cands: cands.append(u)
        if s.count('-') >= 1:
            for seg, us in by_slug.items():
                if seg.startswith(s + '-') and len(seg) - len(s) <= 16:
                    for u in us:
                        if u not in cands and any(p in u for p in PREF + ['/psionic', '/mythic']): cands.append(u)
    cands.sort(key=lambda u: next((i for i, p in enumerate(PREF) if p in u), 9))
    res.append({'key': r['key'], 'name': r['catalog_name'], 'category': r['market_category'], 'book': r['book'],
                'cause': r['cause'], 'cost_gp': r['cost_gp'], 'slugs': names(r), 'candidates': cands})
json.dump(res, open(out, 'w'), indent=1)
print(len(res), sum(1 for x in res if x['candidates']))
