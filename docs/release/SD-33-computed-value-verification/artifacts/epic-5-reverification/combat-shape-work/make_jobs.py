#!/usr/bin/env python3
import json, os, sys

manifest_path, work_dir, jobs_out = sys.argv[1], sys.argv[2], sys.argv[3]
work_dir = os.path.abspath(work_dir)
m = json.load(open(manifest_path))
ftl = os.path.join(work_dir, 'ac-check.txt.ftl')
out_dir = os.path.join(work_dir, 'ac-oracle-txt')
lines = []
for item in m['items']:
    slug = item['slug']
    pcg = os.path.abspath(item['pcg_path'])
    out = os.path.join(out_dir, f'{slug}.txt')
    lines.append(f'{pcg}\t{ftl}\t{out}')
for b in m['baselines']:
    slug = b['slug']
    pcg = os.path.abspath(b['pcg_path'])
    out = os.path.join(out_dir, f'{slug}.txt')
    lines.append(f'{pcg}\t{ftl}\t{out}')
with open(jobs_out, 'w') as f:
    f.write('\n'.join(lines) + '\n')
print(f'wrote {len(lines)} jobs to {jobs_out}')
