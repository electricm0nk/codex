"""Classify every core/ rule-file delta between the committed SF package (old) and a fresh dump
(new) by shape: a delta is 'additive' when every path that differs is either a list that only
GAINED elements (old list is a subsequence-as-multiset of new) or a key the new side added.
Anything else is printed in full."""
import json, sys, os, collections
old_root, new_root = sys.argv[1], sys.argv[2]
classes = collections.Counter(); files = 0; other = []
def walk(o, n, path, out):
    if o == n: return
    if isinstance(o, dict) and isinstance(n, dict):
        for k in set(o) | set(n):
            if k not in n: out.append(('removed_key', path + '/' + k))
            elif k not in o: out.append(('added_key:' + k, path + '/' + k))
            else: walk(o[k], n[k], path + '/' + k, out)
    elif isinstance(o, list) and isinstance(n, list):
        if len(o) == len(n):
            for i, (a, b) in enumerate(zip(o, n)): walk(a, b, f'{path}[{i}]', out)
        else:
            rest = [json.dumps(x, sort_keys=True) for x in n]
            ok = True
            for x in o:
                s = json.dumps(x, sort_keys=True)
                if s in rest: rest.remove(s)
                else: ok = False
            leaf = path.rsplit('/', 1)[-1].split('[')[0]
            out.append((('list_grew:' if ok else 'list_changed:') + leaf, path))
    else:
        out.append(('value_changed', path))
for dp, _, fs in os.walk(os.path.join(old_root, 'core')):
    for f in fs:
        p = os.path.join(dp, f); q = p.replace(old_root, new_root, 1)
        a, b = open(p).read(), open(q).read()
        if a == b: continue
        files += 1; out = []
        walk(json.loads(a), json.loads(b), '', out)
        for c, path in out:
            classes[c] += 1
            if not (c.startswith('list_grew:') or c == 'added_key:granted_by'):
                other.append((p.replace(old_root, ''), c, path))
new_only = sum(1 for dp, _, fs in os.walk(os.path.join(new_root, 'core')) for f in fs if not os.path.exists(os.path.join(dp, f).replace(new_root, old_root, 1)))
old_only = sum(1 for dp, _, fs in os.walk(os.path.join(old_root, 'core')) for f in fs if not os.path.exists(os.path.join(dp, f).replace(old_root, new_root, 1)))
print('core files differing', files, 'core files only in new', new_only, 'only in old', old_only)
for c, n in sorted(classes.items()): print(f'  {c}: {n}')
print('non-additive deltas', len(other))
for o in other: print('  ', o)
