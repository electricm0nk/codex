import json,os,collections,glob,re
ROOT=os.path.expanduser('~/workspace/repos/pcgen/data'); PF=ROOT+'/pathfinder'
rules={}; row2rule={}
for f in glob.glob('data/sheet_rules/*/*/*.json'):
    try: d=json.load(open(f))
    except: continue
    if not isinstance(d,list): continue
    for r in d:
        if isinstance(r,dict) and 'id' in r:
            rules[r['id']]=r
            if '#' in r['id']: continue
            for c in r.get('provenance',{}).get('closure_rows',[]): row2rule.setdefault(c,r['id'])
child={}; idx=collections.defaultdict(list)
for dp,dn,fn in os.walk(PF):
    for f in fn:
        if not f.endswith('.lst'): continue
        p=os.path.join(dp,f); rel=os.path.relpath(p,ROOT)
        for i,line in enumerate(open(p,encoding='utf-8',errors='replace').read().splitlines(),1):
            if not line or line[0]=='#': continue
            parts=[x for x in line.split('\t') if x]
            if not parts: continue
            head=parts[0]
            if head.startswith('ABILITYCATEGORY:'):
                nm=head.split(':',1)[1].strip().upper()
                par=[x.split(':',1)[1].strip().upper() for x in parts[1:] if x.startswith('CATEGORY:')]
                if par and par[0]!=nm: child[nm]=par[0]
                continue
            cat=[x.split(':',1)[1].strip().upper() for x in parts[1:] if x.startswith('CATEGORY:')]
            if not cat or '.MOD' in head: continue
            name=head.split('.COPY=',1)[1] if '.COPY=' in head else head
            key=[x.split(':',1)[1].strip().upper() for x in parts[1:] if x.startswith('KEY:')]
            for n in {name.strip().upper()}|set(key): idx[(cat[0],n)].append(f'{rel}:{i}')
d=json.load(open('data/sheet_rules/_defects/unresolved-references.json'))
edges=collections.defaultdict(set); bysrc=collections.defaultdict(list)
for row in d:
    src,ref=row.split(': ',1)
    cat,name=ref.split('|',1) if '|' in ref else ('',ref)
    C=cat.strip().upper(); N=name.strip().upper()
    base=N.rsplit(' (',1)[0] if N.endswith(')') and ' (' in N else None
    ischild=C in child; P=child.get(C,C)
    h=idx.get((P,N)) or (idx.get((P,base)) if base else None)
    tg=[row2rule[x] for x in (h or []) if x in row2rule]
    if ischild: m='A' if tg else ('B' if h else 'C')
    else: m='D' if tg else ('E' if h else 'F')
    bysrc[src].append((m,ref))
    if m=='A': edges[src].update(tg)
ch=collections.defaultdict(set)
for rid,r in rules.items():
    for g in r.get('granted_by') or []:
        by=g['by']
        if 'Rule' in by: ch[by['Rule']].add(rid)
        elif 'Class' in by: ch['CLASS:'+by['Class']['id']].add(rid)
for s,t in edges.items(): ch[s].update(t)
out=[]
for f in sorted(glob.glob('data/sheet_rules/*/class/*.json')):
    dd=json.load(open(f))
    if not any(r.get('target')=='BaseAttack' for r in dd) or 'Monster' in dd[0].get('tags',[]): continue
    slug=os.path.basename(f)[:-5]; rid=dd[0]['id']
    lv=set()
    def ids(e):
        if isinstance(e,dict):
            for k,v in e.items():
                if k=='ClassLevel' and isinstance(v,str): lv.add(v)
                else: ids(v)
        elif isinstance(e,list):
            for x in e: ids(x)
    ids(dd[0].get('value'))
    seen=set(); st=[rid]+['CLASS:'+x for x in lv|{slug}]
    while st:
        x=st.pop()
        if x in seen: continue
        seen.add(x); st.extend(ch.get(x,()))
    c=collections.Counter(); prof=False
    for x in seen:
        for m,ref in bysrc.get(x.split('#')[0],[]): 
            if m!='A': c[m]+=1
        r=rules.get(x)
        if r and any(isinstance(g,dict) and isinstance(g.get('FactGrant'),dict) and any(k in json.dumps(g['FactGrant'].get('Proficiency','')) for k in ('Weapon','Chosen')) for g in r.get('grants',[])): prof=True
    out.append((f.split('/')[2],slug,len(seen),prof,dict(c),'Prestige' in dd[0].get('tags',[])))
n=len(out); print('class records',n)
print('reach a weapon grant after A edges:',sum(1 for o in out if o[3]),'base:',sum(1 for o in out if o[3] and not o[5]),'prestige:',sum(1 for o in out if o[3] and o[5]))
print('closure size 1 (no outgoing edge even after A):',[o[0]+'/'+o[1] for o in out if o[2]<=1])
print('closures with D>0:',sum(1 for o in out if o[4].get('D')),' F>0:',sum(1 for o in out if o[4].get('F')),' E>0:',sum(1 for o in out if o[4].get('E')),' clean (no B..F):',sum(1 for o in out if not o[4]))
print('NO weapon grant after A edges:')
for o in out:
    if not o[3]: print('  ',o[0]+'/'+o[1],'size',o[2],o[4],'P' if o[5] else 'base')
json.dump(out,open(os.path.dirname(__file__)+'/closure_out.json','w'))
