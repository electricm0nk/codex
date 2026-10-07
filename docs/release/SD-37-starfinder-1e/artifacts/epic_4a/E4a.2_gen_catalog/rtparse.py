import os,re,json,sys
R='src/rules_core/rules_tables'
def strip_noncode(t):
    # blank out comments, strings, chars while preserving newlines/positions
    out=[];i=0;n=len(t)
    while i<n:
        c=t[i]
        if t.startswith('//',i):
            j=t.find('\n',i); j=n if j<0 else j
            out.append(' '*(j-i)); i=j
        elif t.startswith('/*',i):
            d=1;j=i+2
            while j<n and d:
                if t.startswith('/*',j): d+=1;j+=2
                elif t.startswith('*/',j): d-=1;j+=2
                else: j+=1
            seg=t[i:j]; out.append(re.sub(r'[^\n]',' ',seg)); i=j
        elif c=='"' or (c=='r' and re.match(r'r#*"',t[i:i+10])):
            if c=='r':
                m=re.match(r'r(#*)"',t[i:]); h=m.group(1); j=i+len(m.group(0)); end='"'+h
                k=t.find(end,j); k=n if k<0 else k+len(end)
            else:
                j=i+1
                while j<n and t[j]!='"':
                    j+=2 if t[j]=='\\' else 1
                k=j+1
            seg=t[i:k]; out.append(re.sub(r'[^\n]',' ',seg)); i=k
        elif c=="'":
            m=re.match(r"'(\\.[^']*|[^\\'])'",t[i:i+12])
            if m: out.append(' '*len(m.group(0))); i+=len(m.group(0))
            else: out.append(c); i+=1
        else:
            out.append(c); i+=1
    return ''.join(out)
ITEM=re.compile(r'(?P<attrs>(?:\s*#\[[^\]]*\]\s*)*)\s*(?P<vis>pub(?:\([^)]*\))?\s+)?(?P<kw>const\s+fn|unsafe\s+fn|async\s+fn|fn|struct|enum|union|type|trait|const|static|mod|use|impl|macro_rules!)\s*(?P<mut>mut\s+)?(?P<name>[A-Za-z_][A-Za-z0-9_]*)?')
def top_items(path):
    raw=open(path,errors='replace').read()
    code=strip_noncode(raw)
    items=[]
    depth=0;i=0;n=len(code)
    line_start=0
    # iterate statements at depth 0
    pos=0
    while pos<n:
        # skip whitespace
        m=re.compile(r'\s*').match(code,pos); pos=m.end()
        if pos>=n: break
        # attributes
        start=pos
        attrs=''
        while code.startswith('#',pos):
            # parse #[...] or #![...]
            j=pos+1
            if code[j]=='!': j+=1
            if code[j]!='[': break
            d=0
            while j<n:
                if code[j]=='[': d+=1
                elif code[j]==']':
                    d-=1
                    if d==0: j+=1;break
                j+=1
            attrs+=raw[pos:j]; pos=j
            pos=re.compile(r'\s*').match(code,pos).end()
        m=ITEM.match(code,pos)
        # find end of item: scan to ';' or matching '}' at depth0
        j=pos; d=0; endpos=None
        paren=0
        while j<n:
            c=code[j]
            if c in '([': paren+=1
            elif c in ')]': paren-=1
            elif c=='{':
                d+=1
            elif c=='}':
                d-=1
                if d==0 and paren==0:
                    j+=1
                    # item like `struct X {}` ends; for fn/impl/mod ends. for const with block? continue to ';' if const/static/use/type
                    break
            elif c==';' and d==0 and paren==0:
                j+=1;break
            j+=1
        endpos=j
        # for const/static whose initializer contains braces, we need to continue to ';'
        if m and m.group('kw') in ('const','static','type','use') and not code[pos:endpos].rstrip().endswith(';'):
            k=code.find(';',endpos)
            endpos=n if k<0 else k+1
        text=raw[pos:endpos]
        if m:
            vis=(m.group('vis') or '').strip()
            items.append(dict(kw=re.sub(r'\s+',' ',m.group('kw')),name=m.group('name'),vis=vis,attrs=attrs,line=raw.count('\n',0,pos)+1,text=text,code=code[pos:endpos]))
        pos=max(endpos,pos+1)
    return items
def modfile(chain):
    a=R+'/'+'/'.join(chain) if chain else R
    for c in ((a+'.rs',a+'/mod.rs') if chain else (R+'/mod.rs',)):
        if os.path.exists(c): return c
if __name__=='__main__':
    its=top_items(R+'/crb/class_tables.rs')
    for it in its: print(it['kw'],it['name'],it['vis'],it['line'],bool(it['attrs'].strip()))
