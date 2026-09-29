import json,os,collections,glob
ROOT=os.path.expanduser('~/workspace/repos/pcgen/data')
PF=ROOT+'/pathfinder'
rows=set(); 
for f in glob.glob('data/sheet_rules/*/*/*.json'):
    try: d=json.load(open(f))
    except: continue
    if not isinstance(d,list): continue
    for r in d:
        if isinstance(r,dict):
            for c in r.get('provenance',{}).get('closure_rows',[]): rows.add(c)
print('closure rows',len(rows))
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
            conv=f'{rel}:{i}' in rows
            for n in {name.strip().upper()}|set(key): idx[(cat[0],n)].append(conv)
d=json.load(open('data/sheet_rules/_defects/unresolved-references.json'))
mech=collections.Counter(); ex=collections.defaultdict(list); prof=collections.Counter(); m1kind=collections.Counter(); m1cls=set()
import re
for row in d:
    src,ref=row.split(': ',1)
    cat,name=ref.split('|',1) if '|' in ref else ('',ref)
    C=cat.strip().upper(); N=name.strip().upper()
    base=N.rsplit(' (',1)[0] if N.endswith(')') and ' (' in N else None
    def look(c):
        h=idx.get((c,N)) or (idx.get((c,base)) if base else None)
        if not h: return None
        return any(h)
    if C in child:
        r=look(child[C])
        m={True:'A child category; target IS a converted record under the parent -> fixed by parent map',False:'B child category; target exists in oracle but is NOT a converted unit',None:'C child category; target not found under parent'}[r]
    else:
        r=look(C)
        m={True:'D plain category; target IS a converted record (resolver misses it for another cause)',False:'E plain category; target exists in oracle but is NOT a converted unit (book/file not ingested)',None:'F plain category; target found nowhere (dangling, bracketed, or odd syntax)'}[r]
    mech[m]+=1
    if re.search('roficien|Weapon Prof|Armor Prof|Shield Prof',row): prof[m[0]]+=1
    if len(ex[m])<3: ex[m].append(row[:140])
    if m[0]=='A':
        k=src.split(':'); m1kind[k[1]]+=1
        if k[1]=='class': m1cls.add(src)
for m,c in sorted(mech.items()):
    print(c,m)
    for e in ex[m]: print('    ',e)
print('proficiency rows by mechanism',dict(prof))
print('A by source kind',m1kind.most_common()); print('distinct class sources in A',len(m1cls))
import random
random.seed(1)
D=[];PF_=[]
for row in d:
    src,ref=row.split(': ',1)
    cat,name=ref.split('|',1) if '|' in ref else ('',ref)
    C=cat.strip().upper(); N=name.strip().upper()
    base=N.rsplit(' (',1)[0] if N.endswith(')') and ' (' in N else None
    if C in child: continue
    h=idx.get((C,N)) or (idx.get((C,base)) if base else None)
    if h and any(h): D.append((row,len(h),sum(h)))
    if re.search('roficien|Weapon Prof|Armor Prof|Shield Prof',row) and not(h and any(h)) : PF_.append(row)
print('D categories',collections.Counter(r[0].split(': ',1)[1].split('|')[0] for r in D).most_common(8))
print('D where oracle has >1 definition of the key',sum(1 for r in D if r[1]>1),'of',len(D))
print('D where some definitions unconverted',sum(1 for r in D if r[2]<r[1]))
for r in random.sample(D,12): print('   ',r[0][:130],r[1],r[2])
print('prof F rows:'); 
for r in PF_[:17]: print('   ',r[:150])
