"""SD-36 F3c4: what gates every line a converted sorcerer bloodline record grants.

Run from the repo root: python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/f3c4_bloodline_gates.py
For each `<book>:class_feature:sorcerer_bloodline_<x>` principal (bloodline feat picks excluded), every
rule whose `granted_by` names it: the grant's `when` gate, the vars it reads, and which rules contribute to
those vars (`data/sheet_rules/_vars/`). Prints the Draconic rows in full and a histogram for all records.
"""
import collections
import glob
import json

rules = {}
for f in glob.glob('data/sheet_rules/*/*/*.json'):
    try:
        d = json.load(open(f))
    except Exception:
        continue
    if isinstance(d, list):
        for r in d:
            if isinstance(r, dict) and 'id' in r:
                rules[r['id']] = r
vars_ = {}
for f in glob.glob('data/sheet_rules/_vars/*.json'):
    d = json.load(open(f))
    vars_[d['var']] = d
recs = sorted(i for i in rules if ':class_feature:sorcerer_bloodline_' in i and 'feat_' not in i.split(':')[-1] and '#' not in i)


def vars_in(x, acc):
    if isinstance(x, dict):
        for k, v in x.items():
            if k == 'Var' and isinstance(v, str):
                acc.add(v)
            else:
                vars_in(v, acc)
    elif isinstance(x, list):
        for v in x:
            vars_in(v, acc)


tot_edges = 0
var_gated = 0
contrib_hist = collections.Counter()
for rid in recs:
    for gid, g in rules.items():
        for gb in g.get('granted_by', []):
            if gb['by'].get('Rule') == rid:
                tot_edges += 1
                vs = set()
                vars_in(gb['when'], vs)
                if vs:
                    var_gated += 1
                for v in vs:
                    cs = vars_[v]['contributions'] if v in vars_ else []
                    contribs = sorted(set(c['rule_id'] for c in cs))
                    contrib_hist[tuple(c.split(':')[0] for c in contribs) or ('none',)] += 1
                if rid.endswith('draconic'):
                    print('DRACONIC', gid, json.dumps(gb['when'])[:120],
                          [(v, vars_.get(v, {}).get('label'), sorted(set(c['rule_id'] for c in vars_.get(v, {}).get('contributions', [])))) for v in vs])
print('RECORDS', len(recs), 'EDGES', tot_edges, 'VAR_GATED', var_gated)
for k, v in contrib_hist.most_common():
    print('CONTRIB', k, v)
