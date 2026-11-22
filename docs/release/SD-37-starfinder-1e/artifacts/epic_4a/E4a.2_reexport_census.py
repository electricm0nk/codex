#!/usr/bin/env python3
"""Census of what the package-backed catalog still takes from the compiled `rules_tables` module
(SD-37 E4a.2): every `pub use rt::<path>::<Name>;` line of src/rules_core/rules_catalog, by the
kind of item it names (struct/enum/fn/const/...), found by parsing the compiled module's source.
Run from the repo root:  python3 docs/release/SD-37-starfinder-1e/artifacts/epic_4a/E4a.2_reexport_census.py
"""
import collections, os, re, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), 'E4a.2_gen_catalog'))
import analyze as A

cnt = collections.Counter()
for d, _, fs in os.walk('src/rules_core/rules_catalog'):
    for f in fs:
        if not f.endswith('.rs') or f.endswith('_tests.rs'):
            continue
        for line in open(os.path.join(d, f)):
            m = re.match(r'\s*pub(?:\(crate\))? use rt::([A-Za-z0-9_:]+);', line)
            if not m:
                continue
            path = m.group(1).split('::')
            name, chain = path[-1], tuple(path[:-1])
            kind = 'submodule-or-reexport'
            for it in A.items(chain):
                if it['name'] == name and it['kw'] not in ('use', 'impl', 'mod'):
                    kind = it['kw']
                    break
            cnt[kind] += 1
print('total', sum(cnt.values()), dict(sorted(cnt.items())))
