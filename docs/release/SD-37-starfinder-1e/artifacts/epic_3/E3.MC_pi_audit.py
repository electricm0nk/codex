import json,re,sys,collections
from pathlib import Path
src=Path('src/rules_core/pi_screening.rs').read_text()
def arr(name):
    m=re.search(r'pub const '+name+r': &\[&str\] = &\[(.*?)\];',src,re.S)
    body=re.sub(r'//[^\n]*','',m.group(1))
    return re.findall(r'"([^"]*)"',body)
PF=arr('PI_BLACKLIST_TERMS'); SF=arr('SF_PI_TERMS'); T=PF+SF
print('terms pf',len(PF),'sf',len(SF))
def strings(v,key,out,skip):
    if isinstance(v,str): out.append((key,v))
    elif isinstance(v,list):
        for x in v: strings(x,key,out,skip)
    elif isinstance(v,dict):
        for k,x in v.items():
            if k in skip: continue
            strings(x,k,out,skip)
ruleid=re.compile(r'^[a-z0-9_#]+:[a-z0-9_#]+:[a-z0-9_#]+$')
def scan(root,skip,label):
    hits=[];n=0;files=0
    for p in sorted(Path(root).rglob('*.json')):
        if p.name.startswith('_') or p.name=='LICENSE.json' or '/_' in str(p.relative_to(root)): continue
        files+=1
        v=json.loads(p.read_text()); out=[]; strings(v,'',out,skip)
        for k,s in out:
            if ruleid.match(s): continue
            n+=1
            for t in T:
                if t in s: hits.append((str(p.relative_to(root)),k,t,s[:120])); break
    print(f'{label}: files={files} strings={n} hits={len(hits)}')
    for h in hits[:40]: print('  ',h)
    return hits
h1=scan('data/starfinder-1e/sheet_rules',{'provenance'},'package (exact-case substring, = classify_field_sf)')
h2=scan('data/starfinder-1e/corpus',{'source','provenance'},'corpus all fields except source')
# key-field leak: corpus records PI-REDACTED whose data.key still carries the term
