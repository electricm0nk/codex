"""SD-36 F7c (d): what the corpus states about the Magus's class skills.

Walks every data/corpus/**/*.json record (read-only). A record is a Magus record when its
`data.class` is "Magus" or its `data.key`/`data.name` is "Magus". Reports: how many Magus
records exist, how many carry prose (a non-empty `data.description`), every CSKILL token on a
Magus record, and every record anywhere whose prose states a magus class-skill list
("magus" and "class skill" in the same description).
"""
import glob, json, re, sys

def texts(o):
    if isinstance(o, str):
        yield o
    elif isinstance(o, dict):
        for v in o.values():
            yield from texts(v)
    elif isinstance(o, list):
        for v in o:
            yield from texts(v)

records = magus = magus_prose = 0
cskill, prose_hits, religion_on_magus = [], [], []
for path in sorted(glob.glob('data/corpus/**/*.json', recursive=True)):
    try:
        doc = json.load(open(path))
    except Exception:
        continue
    for rec in (doc if isinstance(doc, list) else [doc]):
        if not isinstance(rec, dict) or not isinstance(rec.get('data'), dict):
            continue
        records += 1
        data = rec['data']
        desc = data.get('description') or ''
        is_magus = data.get('class') == 'Magus' or data.get('key') == 'Magus' or data.get('name') == 'Magus'
        if is_magus:
            magus += 1
            if desc.strip():
                magus_prose += 1
            for tok in data.get('raw_tokens') or []:
                if tok.get('key') == 'CSKILL':
                    cskill.append((path, rec.get('source', {}).get('path'), rec.get('source', {}).get('line'), tok['value']))
            if any(re.search(r'(?i)knowledge \(religion\)', t) for t in texts(data)):
                religion_on_magus.append(path)
        if desc and re.search(r'(?i)\bmagus\b', desc) and re.search(r'(?i)class skill', desc):
            prose_hits.append((path, desc[:240]))

print(f'corpus records walked: {records}')
print(f'Magus records (data.class == "Magus" or key/name "Magus"): {magus}; with prose (non-empty description): {magus_prose}')
print(f'CSKILL tokens on Magus records: {len(cskill)}')
for p, src, line, v in cskill:
    print(f'  {p} ({src}:{line}): {v}')
print(f'Magus records whose any field names Knowledge (religion): {len(religion_on_magus)}')
for p in religion_on_magus:
    print(f'  {p}')
print(f'records anywhere whose prose names "magus" and "class skill": {len(prose_hits)}')
for p, d in prose_hits:
    print(f'  {p}: {d!r}')
