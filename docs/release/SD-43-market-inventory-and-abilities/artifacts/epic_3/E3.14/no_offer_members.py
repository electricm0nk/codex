#!/usr/bin/env python3
"""SD-43 E3.14: for E3.4's class-feature pools with no offer (pool_shapes.py's list), the members the
converter's filter (data/sheet_rules/_pool_members.json) selects. A second implementation beside the
Rust census. Usage: python3 -I <this> <repo root>."""
import json, glob, re, sys, collections
root = sys.argv[1]
rules = {}
for f in glob.glob(root + '/data/sheet_rules/*/*/*.json'):
    try: d = json.load(open(f))
    except Exception: continue
    if isinstance(d, list):
        for r in d:
            if isinstance(r, dict) and 'id' in r: rules[r['id']] = r
byp = collections.defaultdict(list)
for r in rules.values():
    t = r.get('target')
    if isinstance(t, dict) and 'Pool' in t: byp[t['Pool']].append(r)
def featish(p, rs):
    w = p.split('_')
    if p == 'feat' or (('feat' in w or 'feats' in w) and 'companion' not in w): return True
    return any((r.get('offers') or {}).get('from', {}).get('Rules', {}).get('pool') == 'feat' for r in rs)
slug = lambda s: re.sub(r'[^a-z0-9]+', '_', s.lower()).strip('_')
c = json.load(open(root + '/docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_0/abilities-census.json'))
P = c['class_feature_pools']['pools']
cls = [n for n, v in P.items() if 'class' in v['granted_by']]
rest = [n for n in cls if slug(n) in byp and not featish(slug(n), byp[slug(n)])]
nooffer = sorted(n for n in rest if not any(r.get('offers') for r in byp[slug(n)]))
table = json.load(open(root + '/data/sheet_rules/_pool_members.json'))
def members(f):
    tags = {t.lower() for t in f['any_tags']}
    return [r['id'] for r in rules.values() if '#' not in r['id'] and r.get('label') and (r['id'] in f['rules'] or (r.get('pool') == f['member_pool'] and any(t.lower() in tags for t in r.get('tags') or [])))]
counts = {n: len(members(table[slug(n)])) if slug(n) in table else 0 for n in nooffer}
print('no-offer pools', len(nooffer), 'with a filter', sum(slug(n) in table for n in nooffer), 'listing >= 1 member', sum(v > 0 for v in counts.values()))
for n in nooffer: print(f'{counts[n]:4d}  {n}')
