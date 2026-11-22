"""For every core rule whose gate changed from `Not Holds Fact X` to `Not Holds Rule(s)`, check
each named rule declares fact X (FactDeclare name X) in the new package."""
import json, os, sys
old, new = sys.argv[1], sys.argv[2]
idx = {}
for dp, _, fs in os.walk(new):
    for f in fs:
        if f.endswith('.json') and not f.startswith('_') and '/_' not in dp:
            try: v = json.load(open(os.path.join(dp, f)))
            except Exception: continue
            if isinstance(v, list):
                for r in v:
                    if isinstance(r, dict) and 'id' in r: idx[r['id']] = r
def facts(r): return {g['FactDeclare']['name'] for g in r.get('grants', []) if 'FactDeclare' in g}
def find(o, out, key):
    if isinstance(o, dict):
        for k, v in o.items():
            if k == key: out.append(v)
            find(v, out, key)
    elif isinstance(o, list):
        for x in o: find(x, out, key)
ok = bad = 0
for dp, _, fs in os.walk(os.path.join(old, 'core')):
    for f in fs:
        p = os.path.join(dp, f); q = p.replace(old, new, 1)
        a, b = json.load(open(p)), json.load(open(q))
        fa, fb = [], []
        find(a, fa, 'Fact'); find(b, fb, 'Fact')
        gone = [x['name'] for x in fa if x not in fb]
        if not gone: continue
        rules = []; find(b, rules, 'Rule')
        for name in gone:
            decl = [r for r in set(rules) if r in idx and name in facts(idx[r])]
            if decl: ok += 1; print('OK', p.replace(old, ''), name, '->', len(decl), 'declaring rules')
            else: bad += 1; print('BAD', p.replace(old, ''), name)
print(f'fact_gates_resolved={ok} unresolved={bad}')
