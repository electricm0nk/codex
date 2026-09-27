#!/usr/bin/env python3
"""SD-36 F6 merge-readiness B3: every bundled equipment/equipmods record carries exactly the raw
record's data.key/name/cost_gp + source.path/line, unaltered by the residue sanitizer.
Run from the repo root after `node scripts/gen-corpus-bundle.mjs`."""
import glob, json, os
n = same = ii = 0
changed = []
for f in sorted(glob.glob('data/corpus/*/equipment/equipmods/**/*.json', recursive=True)):
    if os.path.basename(f) == 'LICENSE.json':
        continue
    d = json.load(open(f))
    e = json.load(open(f.replace('data/corpus/', 'apps/desktop/src-tauri/resources/corpus_bundle/', 1)))
    n += 1
    want = {'data': {k: d['data'][k] for k in ('key', 'name', 'cost_gp') if k in d.get('data', {})},
            'source': {k: d['source'][k] for k in ('path', 'line') if k in d.get('source', {})}}
    if want == e:
        same += 1
    else:
        changed.append(f)
    ii += 'Intelligent Item' in d['data'].get('key', '')
print("SD-36 F6 merge-readiness B3 residue audit -- scripts/gen-corpus-bundle.mjs equipment/equipmods/ trim")
print("command: node scripts/gen-corpus-bundle.mjs && python3 " + __file__)
print(f"equipmods records bundled: {n}; bundled fields == raw data.key/name/cost_gp + source.path/line: {same} of {n}; altered by the residue sanitizer: {len(changed)}")
print(f"of which key contains 'Intelligent Item': {ii}")
for c in changed:
    print("ALTERED", c)
