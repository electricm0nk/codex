import json,glob,os,re
S=os.path.expanduser('~/workspace/repos/pcgen/data')
rows={}  # (category,key) -> fields (abilities)
def load(path):
    for line in open(path,encoding='utf-8',errors='replace'):
        if line.startswith('#') or not line.strip(): continue
        f=[x for x in line.rstrip('\n').split('\t') if x]
        yield f
abil={}
for p in glob.glob(S+'/starfinder/paizo/**/*abilities*.lst',recursive=True):
    for f in load(p):
        name=f[0]
        if '.MOD' in name or '.COPY' in name: continue
        cat=next((x[9:] for x in f if x.startswith('CATEGORY:')),'')
        key=next((x[4:] for x in f if x.startswith('KEY:')),name)
        abil.setdefault((cat.upper(),key.upper()),f)
races={}
res={}
for f in sorted(glob.glob('data/starfinder-1e/corpus/*/race/*.json')):
    r=json.load(open(f))
    src=r['source']; p=S+'/'+src['path']
    lines=open(p,encoding='utf-8',errors='replace').read().split('\n')
    fields=[x for x in lines[src['line']-1].split('\t') if x]
    vals=[]
    for x in fields:
        if x.startswith('ABILITY:Race|'):
            for a in x.split('|')[2:]:
                if a.startswith('PRE') or a.startswith('!PRE'): continue
                af=abil.get(('RACE',a.upper()))
                if not af: vals.append(('noRaceAbility',a)); continue
                for y in af:
                    if y.startswith('ABILITY:Internal|'):
                        for s in y.split('|')[2:]:
                            if s.startswith('PRE') or s.startswith('!PRE'): continue
                            sf=abil.get(('INTERNAL',s.upper()))
                            if sf:
                                for z in sf:
                                    if z.startswith('BONUS:VAR|RaceHP|'): vals.append(('ok',z.split('|')[2]))
        if x.startswith('BONUS:VAR|RaceHP|'): vals.append(('direct',x))
    res[r['unit_id']]=vals
from collections import Counter
c=Counter(tuple(k for k,_ in v) for v in res.values())
print(c)
for u,v in res.items():
    if [k for k,_ in v]!=['ok']: print(u,v)
print(Counter(v[0][1] for v in res.values() if v))
