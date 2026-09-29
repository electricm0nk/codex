#!/usr/bin/env python3
"""SD-36 F3c5 proxy measurement (FS-22, mechanism M): records whose first closure row is a .MOD row
and whose object's base plain row sits in no record's closure. A Python walk of the pinned tree
(~/workspace/repos/pcgen/data) and data/sheet_rules; a proxy, not the converter's index.
Run from the repo root: python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/f3c5_mod_base_rows.py
[<out.json>] (writes the hit list only when a path is given).
"""
import os,json,glob,collections,re
root=os.path.expanduser('~/workspace/repos/pcgen/data')
pkg='data/sheet_rules'
cache={}
def line(p):
    path,l=p.rsplit(':',1)
    if path not in cache: cache[path]=open(os.path.join(root,path),encoding='utf-8',errors='replace').read().split('\n')
    return cache[path][int(l)-1]
def ident(raw):
    f=[x for x in raw.rstrip('\r').split('\t') if x.strip()]
    name=f[0].strip(); toks=dict()
    for t in f[1:]:
        if ':' in t: k,v=t.split(':',1); toks.setdefault(k.strip(),v.strip())
    cat=''; base=name
    if base.startswith('CATEGORY=') and '|' in base: cat,base=base[9:].split('|',1)
    mod=base.endswith('.MOD')
    if mod: base=base[:-4]
    if not mod: cat=toks.get('CATEGORY',cat); base=toks.get('KEY',base)
    return cat.upper(),base.upper(),mod
# plain rows index over pathfinder
plain=collections.defaultdict(list)
for dp,dn,fn in os.walk(os.path.join(root,'pathfinder')):
    if '_pfs' in dp: continue
    for f in fn:
        if not f.endswith('.lst'): continue
        p=os.path.join(dp,f); rel=os.path.relpath(p,root)
        for i,l in enumerate(open(p,encoding='utf-8',errors='replace').read().split('\n'),1):
            if not l.strip() or l.lstrip().startswith('#') or '\t' not in l and ':' not in l: continue
            try: c,k,m=ident(l)
            except Exception: continue
            if not m and '.COPY=' not in l.split('\t')[0]: plain[(c,k)].append(f'{rel}:{i}')
owned=set(); recs=[]
for f in glob.glob(pkg+'/*/*/*.json'):
    for r in json.load(open(f)):
        if '#' in r['id']: continue
        cr=r.get('provenance',{}).get('closure_rows',[])
        owned.update(cr); recs.append((r['id'],cr))
hits=[]
for rid,cr in recs:
    if not cr: continue
    try: c,k,m=ident(line(cr[0]))
    except Exception: continue
    if not m: continue
    bases=[b for b in plain.get((c,k),[])]
    missing=[b for b in bases if b not in cr and b not in owned]
    if bases and missing: hits.append((rid,cr[0],missing))
print('records anchored at a .MOD row whose object base row is in no record closure:',len(hits))
for h in hits[:15]: print(' ',h)
import sys
if len(sys.argv) > 1:
    json.dump(hits, open(sys.argv[1], 'w'), indent=1)
