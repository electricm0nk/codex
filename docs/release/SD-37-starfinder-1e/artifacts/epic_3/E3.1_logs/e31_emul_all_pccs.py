import os,re,sys
D = os.environ['PCGEN_CORPUS_ROOT']  # the pinned oracle's data/ (scripts/fetch-pcgen-oracle.sh)
S = D + '/starfinder'
pccs=sorted(os.path.join(r,f) for r,_,fs in os.walk(S) for f in fs if f.endswith('.pcc'))
def res(src,t):
    t=t.split('|')[0].strip().replace('\\','/')
    t=t[1:] if t.startswith('@') else t
    if t.startswith('*/'): p=os.path.join(D,t[2:])
    elif t.startswith('/'): p=os.path.join(D,t.lstrip('/'))
    else: p=os.path.join(os.path.dirname(src),t)
    return os.path.normpath(p)
def walk(pcc,lst,seen,miss,odd):
    if pcc in seen: return
    seen.add(pcc)
    for i,l in enumerate(open(pcc,encoding='utf-8',errors='replace')):
        c=l.strip()
        if not c or c.startswith('#'): continue
        if c.startswith('PCC:'):
            walk(res(pcc,c[4:]),lst,seen,miss,odd); continue
        if ':' not in c: continue
        k,r=c.split(':',1); t=r.split('|')[0].strip()
        if '.lst' not in t.lower(): continue
        if '(' in t: odd.append((pcc,c))
        p=res(pcc,t)
        if os.path.isfile(p): lst.setdefault(p,k)
        else: miss.append((pcc,i+1,c))
allfiles=set(os.path.join(r,f) for r,_,fs in os.walk(S) for f in fs if f.endswith('.lst'))
tot={};miss=[];odd=[]
for p in pccs:
    l={}; walk(p,l,set(),miss,odd); tot.update(l)
    print(len(l), os.path.relpath(p,S))
print('union',len(tot),'under S',len([p for p in tot if p.startswith(S+'/')]),'all lst',len(allfiles))
print('outside S',[os.path.relpath(p,D) for p in tot if not p.startswith(S+'/')])
print('unreferenced',sorted(os.path.relpath(p,S) for p in allfiles-set(tot)))
print('missing',miss); print('odd',odd)
