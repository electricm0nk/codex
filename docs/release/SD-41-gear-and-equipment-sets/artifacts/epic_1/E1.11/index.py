"""index.py <cache-dir> <out.json>: every section (heading -> first Price) and table row (name -> price cell) of every cached page."""
import re, sys, json, html, hashlib, os
cache, out = sys.argv[1:3]
log = [l.rstrip('\n').split('\t') for l in open(os.path.join(cache, 'fetch-log.tsv'))]
def text(h):
    h = re.sub(r'(?s)<script.*?</script>|<style.*?</style>', '', h)
    return re.sub(r'\s+', ' ', html.unescape(re.sub(r'<[^>]+>', ' ', h))).strip()
def slug(s):
    s = s.lower().replace("'", "").replace('’', '')
    return re.sub(r'[^a-z0-9]+', '-', s).strip('-')
PRICE = re.compile(r'\b(?:Price|Market Price)\s*:?\s*(.{1,200}?)(?:;|\sDESCRIPTION\b|\s(?:Slot|Weight|CL|ML|Aura|Cost)\b|$)')
entries = []
for u, code, at in log:
    if code != '200': continue
    f = os.path.join(cache, hashlib.md5(u.encode()).hexdigest() + '.html')
    h = open(f, errors='replace').read()
    m = re.search(r'(?s)<article.*?</article>', h) or re.search(r'(?s)<div[^>]*class="[^"]*article-content.*', h)
    body = m.group(0) if m else h
    parts = re.split(r'(?s)(<h[1-6](?![^>]*widget-title)[^>]*>.*?</h[1-6]>)', body)
    heading = None
    for p in parts:
        if re.match(r'(?s)<h[1-6]', p):
            heading = text(p); continue
        if heading is None: continue
        t = text(p)
        pm = PRICE.search(t)
        entries.append({'url': u, 'fetched_at': at, 'kind': 'section', 'name': heading, 'slug': slug(heading),
                        'price': pm.group(1).strip() if pm else None, 'snippet': t[:300]})
    for tbl in re.findall(r'(?s)<table.*?</table>', body):
        rows = [[text(c) for c in re.findall(r'(?s)<t[hd][^>]*>(.*?)</t[hd]>', r)] for r in re.findall(r'(?s)<tr.*?</tr>', tbl)]
        pc = None
        for r in rows:
            for i, c in enumerate(r):
                if re.fullmatch(r'(?i)(market )?(price|cost)\d?', c.strip()): pc = i
            if pc is not None and r and pc < len(r) and not re.fullmatch(r'(?i)(market )?(price|cost)\d?', r[pc].strip()):
                entries.append({'url': u, 'fetched_at': at, 'kind': 'table', 'name': r[0], 'slug': slug(r[0]),
                                'price': r[pc], 'snippet': ' | '.join(r)[:300]})
json.dump(entries, open(out, 'w'))
print(len(entries), sum(1 for e in entries if e['price']))
