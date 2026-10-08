#!/usr/bin/env python3
"""SD-37 E6.2: classify the SF package delta of the selection-hop ability-pool change.

usage: E6.2_delta.py <new package dir> <baseline package dir>

Allowed classes (anything else is printed and the exit is 1):
  hop_pool_sibling  a rule file whose only change is ADDED rules with id `<principal>#pool_<slug>`,
                    target {"Pool": <slug>}, value Number(Const n), no offers/grants/granted_by;
                    every pre-existing rule byte-identical (no id, label or value moved)
  report_count      _report.json whose only change is rules_written moving by the number of
                    hop_pool_sibling rules added
Prints the counts (denominator: files differing between the two trees).
"""
import json, os, sys, filecmp

new, old = sys.argv[1], sys.argv[2]

def files(root):
    out = set()
    for d, _, ns in os.walk(root):
        for n in ns:
            out.add(os.path.relpath(os.path.join(d, n), root))
    return out

a, b = files(new), files(old)
differ = sorted(p for p in a | b if p not in a or p not in b or not filecmp.cmp(os.path.join(new, p), os.path.join(old, p), shallow=False))
classes, bad, added = {}, [], []
for p in differ:
    if p == '_report.json':
        continue
    if p not in a or p not in b:
        bad.append((p, 'file added or removed')); continue
    n = {r['id']: r for r in json.load(open(os.path.join(new, p)))}
    o = {r['id']: r for r in json.load(open(os.path.join(old, p)))}
    if any(n.get(i) != r for i, r in o.items()):
        bad.append((p, 'a pre-existing rule changed')); continue
    extra = [n[i] for i in n if i not in o]
    ok = extra and all(
        '#pool_' in r['id'] and isinstance(r.get('target'), dict) and 'Pool' in r['target']
        and r['id'].endswith('#pool_' + r['target']['Pool'])
        and 'Const' in r['value'].get('Number', {}) and not r.get('offers') and not r.get('grants') and not r.get('granted_by')
        for r in extra)
    if not ok:
        bad.append((p, 'added rules are not hop pool siblings')); continue
    classes['hop_pool_sibling'] = classes.get('hop_pool_sibling', 0) + 1
    added += [r['id'] for r in extra]
if '_report.json' in differ:
    rn, ro = json.load(open(os.path.join(new, '_report.json'))), json.load(open(os.path.join(old, '_report.json')))
    moved = {k for k in set(rn) | set(ro) if rn.get(k) != ro.get(k)}
    if moved == {'rules_written'} and rn['rules_written'] - ro['rules_written'] == len(added):
        classes['report_count'] = 1
    else:
        bad.append(('_report.json', f'moved {sorted(moved)}'))
print(f'differing files: {len(differ)}')
for k, v in sorted(classes.items()):
    print(f'  {k}: {v}')
print(f'added rule ids: {added}')
for p, why in bad:
    print(f'UNCLASSIFIED {p}: {why}')
print('verdict=' + ('PASS' if not bad else 'FAIL'))
sys.exit(1 if bad else 0)
