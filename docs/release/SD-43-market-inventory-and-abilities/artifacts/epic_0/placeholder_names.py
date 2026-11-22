#!/usr/bin/env python3
"""SD-43 E0.3 -- placeholder-name scan, every record kind and every surface (read-only).

Surfaces: corpus records (data.name; every other name/label/OUTPUTNAME field inside the record),
corpus `_settled` copies, `data/rules_tables`, and the desktop `corpus_bundle` copy.
Shapes: PRIMARY = the decisions.md s22 pattern `[\\[\\]%]|\\bNAME\\b`; WIDE adds `{...}` and `%N`
(`%` is already PRIMARY, so WIDE only adds braces). Every surface's count is printed, zero or not.

    python3 -I <this file>          # write placeholder-names.json beside it, print the counts
    python3 -I <this file> --check  # recompute; exit 0 only if 111/3,805 and 201/51,474 reproduce
                                    # and the written JSON equals the recomputation
Exit: 0 ok; 1 a check failed; 2 an input (script output / corpus) is missing.
Does not cover: names wrong without a placeholder shape.
"""
import collections, glob, json, os, re, subprocess, sys

PRIMARY = re.compile(r'[\[\]%]|\bNAME\b')
WIDE = re.compile(r'[\[\]%]|\bNAME\b|\{[^{}]*\}')
NAME_KEYS = {'name', 'label', 'outputname', 'display_name', 'identity'}
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = subprocess.check_output(['git', '-C', HERE, 'rev-parse', '--show-toplevel'], text=True).strip()
OUT = os.path.join(HERE, 'placeholder-names.json')
EXPECT = {'equipment': (111, 3805), 'all_kinds': (201, 51474)}


def fields(o, path=''):
    """Yield (field path, string) for every name-like string; OUTPUTNAME tokens are {key,value} pairs."""
    if isinstance(o, dict):
        if str(o.get('key', '')).upper() == 'OUTPUTNAME' and isinstance(o.get('value'), str):
            yield path + '.OUTPUTNAME', o['value']
        for k, v in o.items():
            if isinstance(v, str) and k.lower() in NAME_KEYS:
                yield f'{path}.{k}', v
            else:
                yield from fields(v, f'{path}.{k}')
    elif isinstance(o, list):
        for v in o:
            yield from fields(v, path + '[]')


def load(f):
    with open(f) as fh:
        return json.load(fh)


def rel(f):
    return os.path.relpath(f, ROOT)


def scan():
    hits, counts = [], collections.OrderedDict()

    def add(surface, field, total, f, name, key, book):
        c = counts.setdefault(f'{surface}|{field}', {'surface': surface, 'field': field, 'scanned': 0, 'primary': 0, 'wide': 0})
        c['scanned'] += 1
        if WIDE.search(name):
            c['wide'] += 1
            p = bool(PRIMARY.search(name))
            c['primary'] += p
            hits.append({'surface': surface, 'record': rel(f), 'field': field, 'raw_name': name, 'key': key, 'book': book, 'primary': p})

    # corpus: one record per file, kind = 4th path part; _settled/_parity are not records
    # [records, primary hits on data.name]; `equipment` = equipment/*.json only (the s22 glob), `equipment_all_depths` adds equipment/<sub>/*.json
    corpus = {'all_kinds': [0, 0], 'equipment': [0, 0], 'equipment_all_depths': [0, 0]}
    kinds = collections.Counter()
    for f in sorted(glob.glob(os.path.join(ROOT, 'data/corpus/*/*/**/*.json'), recursive=True)):
        p = rel(f).split('/')
        book, kind = p[2], p[3]
        if kind in ('_settled', '_parity'):
            continue
        d = load(f)
        corpus['all_kinds'][0] += 1
        kinds[kind] += 1
        top = kind == 'equipment' and len(p) == 5
        corpus['equipment'][0] += top
        corpus['equipment_all_depths'][0] += kind == 'equipment'
        data = d.get('data') if isinstance(d, dict) else None
        key = (data or {}).get('key') if isinstance(data, dict) else None
        if isinstance(data, dict) and isinstance(data.get('name'), str):
            add('corpus:' + kind, 'data.name', 0, f, data['name'], key, book)
            if PRIMARY.search(data['name']):
                corpus['all_kinds'][1] += 1
                corpus['equipment'][1] += top
                corpus['equipment_all_depths'][1] += kind == 'equipment'
        # every other name-like field in the record (label, OUTPUTNAME, nested name/identity)
        for fp, s in fields(d):
            if fp != '.data.name':
                add('corpus-other:' + kind, fp, 0, f, s, key, book)
    # _settled, rules_tables, desktop bundle: all name-like fields in every JSON file
    for surface, pat in (('_settled', 'data/corpus/*/_settled/*.json'),
                         ('rules_tables', 'data/rules_tables/**/*.json'),
                         ('corpus_bundle', 'apps/desktop/src-tauri/resources/corpus_bundle/**/*.json')):
        counts.setdefault(f'{surface}|files', {'surface': surface, 'field': 'files', 'scanned': 0, 'primary': 0, 'wide': 0})
        for f in sorted(glob.glob(os.path.join(ROOT, pat), recursive=True)):
            counts[f'{surface}|files']['scanned'] += 1
            book = rel(f).split('/')[2] if surface == '_settled' else ''
            for fp, s in fields(load(f)):
                add(surface, fp.split('.')[-1], 0, f, s, fp, book)
    return {'corpus': {k: {'records': v[0], 'primary_hits_data_name': v[1]} for k, v in corpus.items()},
            'kinds': dict(sorted(kinds.items())), 'counts': list(counts.values()), 'hits': hits}


def main():
    res = scan()
    if not res["corpus"]["all_kinds"]["records"]:
        print('corpus missing', file=sys.stderr); return 2
    for c in res['counts']:
        print(f"{c['surface']:28} {c['field']:28} scanned={c['scanned']:6} primary={c['primary']:4} wide={c['wide']:4}")
    ok = True
    for k in ('equipment', 'equipment_all_depths', 'all_kinds'):
        h, t = EXPECT.get(k, (None, None))
        got = res['corpus'][k]
        line = f"{k}: {got['primary_hits_data_name']} of {got['records']:,}" + (f" (want {h} of {t:,})" if h is not None else ' (printed, no expectation)')
        good = h is None or (got['primary_hits_data_name'], got['records']) == (h, t)
        ok &= good
        print(('OK ' if good else 'FAIL ') + line)
    if '--check' in sys.argv:
        if not os.path.exists(OUT) or load(OUT) != json.loads(json.dumps(res)):
            print('FAIL written JSON missing or differs from recomputation'); ok = False
        return 0 if ok else 1
    with open(OUT, 'w') as fh:
        json.dump(res, fh, indent=1, sort_keys=True); fh.write('\n')
    return 0 if ok else 1


if __name__ == '__main__':
    sys.exit(main())
