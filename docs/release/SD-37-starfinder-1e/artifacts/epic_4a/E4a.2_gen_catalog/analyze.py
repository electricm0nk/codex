import sys,os,re,json,collections
sys.path.insert(0,os.path.dirname(__file__))
import rtparse
R=rtparse.R
def registry():
    t=open('src/rules_core/rules_data_package.rs').read()
    reg=t[t.index('rules_tables_registry! {\n    statics'):]
    reg=reg[:reg.index('\n}\n')]
    statics=re.findall(r'"([^"]+)" =>',reg[:reg.index('built {')])
    built=re.findall(r'"([^"]+)" =>',reg[reg.index('built {'):])
    vt=t[t.index('pub const VIEWS'):]
    vt=vt[:vt.index('];')]
    views=re.findall(r'\("([^"]+)",',vt)
    return statics,built,views
def all_files():
    out={}
    for d,ds,fs in os.walk(R):
        for f in fs:
            if f.endswith('.rs'):
                p=os.path.join(d,f)
                rel=os.path.relpath(p,R)[:-3]
                parts=rel.split('/')
                if parts[-1]=='mod': parts=parts[:-1]
                out[tuple(parts)]=p
    return out
FILES=all_files()
_cache={}
def items(chain):
    if chain not in _cache:
        _cache[chain]=rtparse.top_items(FILES[chain]) if chain in FILES else []
    return _cache[chain]
def ids_to_pairs(ids):
    s=set()
    for i in ids:
        parts=i.split('/')
        s.add((tuple(parts[:-1]),parts[-1]))
    return s
if __name__=='__main__':
    st,bu,vi=registry()
    print(len(st),len(bu),len(vi))

def is_type_name(n):
    return bool(re.match(r'^[A-Z][a-z0-9]',n)) 
def flagged_reads():
    st,bu,vi=registry()
    regpairs=ids_to_pairs(st)|ids_to_pairs(bu)|ids_to_pairs(vi)
    tok=re.compile(r'[A-Za-z_][A-Za-z0-9_]*')
    names={n for _,n in regpairs if not is_type_name(n)}
    reads={}
    allitems={c:items(c) for c in FILES}
    changed=True
    while changed:
        changed=False
        for c,its in allitems.items():
            for it in its:
                if it['kw'] in('fn','const fn') and it['name'] and (c,it['name']) not in regpairs and (c,it['name']) not in reads:
                    ts=set(tok.findall(it['code']))-{it['name']}
                    hit=ts&names
                    if hit:
                        reads[(c,it['name'])]=sorted(hit)[:4]; names.add(it['name']); changed=True
    return regpairs,reads
