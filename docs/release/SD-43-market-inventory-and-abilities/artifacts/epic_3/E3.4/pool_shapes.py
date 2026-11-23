#!/usr/bin/env python3
"""SD-43 E3.4: class-owned pools (E0.1 census) by shape in the converted package. Usage: python3 -I <this> <repo root>."""
import json,glob,re,sys,collections
root=sys.argv[1]
rules={}
for f in glob.glob(root+'/data/sheet_rules/*/*/*.json'):
    try: d=json.load(open(f))
    except Exception: continue
    if isinstance(d,list):
        for r in d:
            if isinstance(r,dict) and 'id' in r: rules[r['id']]=r
byp=collections.defaultdict(list)
for r in rules.values():
    t=r.get('target')
    if isinstance(t,dict) and 'Pool' in t: byp[t['Pool']].append(r)
def featish(p, rs):
    w=p.split('_')
    if p=='feat' or (('feat' in w or 'feats' in w) and 'companion' not in w): return True
    return any((r.get('offers') or {}).get('from',{}).get('Rules',{}).get('pool')=='feat' for r in rs)
slug=lambda s: re.sub(r'[^a-z0-9]+','_',s.lower()).strip('_')
c=json.load(open(root+'/docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_0/abilities-census.json'))
P=c['class_feature_pools']['pools']
cls=[n for n,v in P.items() if 'class' in v['granted_by']]
nopool=[n for n in cls if slug(n) not in byp]
feat=[n for n in cls if slug(n) in byp and featish(slug(n),byp[slug(n)])]
rest=[n for n in cls if slug(n) in byp and n not in feat]
nooffer=[n for n in rest if not any(r.get('offers') for r in byp[slug(n)])]
print('class-owned',len(cls),'no Pool target',len(nopool),'feat pools',len(feat),'class-feature pools',len(rest),'of which no offer',len(nooffer))
print(sorted(nooffer))
