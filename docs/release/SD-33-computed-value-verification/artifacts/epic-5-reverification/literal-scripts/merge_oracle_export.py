#!/usr/bin/env python3
"""Merge per-item raw BatchExporter `.txt` outputs into one slug-prefixed
KEY=VALUE export file, matching AT-33-E5-001/002 attempt 1's exact
convention (`<slug>.<TOKEN>=<value>` per line) so
`scripts/oracle_harness/oracle_export.py`'s flat parser reads it unchanged.
"""
import sys, glob, os

manifest_dir, out_dir, dest = sys.argv[1], sys.argv[2], sys.argv[3]
lines = []
for path in sorted(glob.glob(os.path.join(out_dir, '*.txt'))):
    slug = os.path.splitext(os.path.basename(path))[0]
    for raw in open(path):
        raw = raw.rstrip('\n')
        if not raw or '=' not in raw:
            continue
        key, _, val = raw.partition('=')
        lines.append(f"{slug}.{key}={val}")
with open(dest, 'w') as f:
    f.write('\n'.join(lines) + '\n')
print(f"merged {len(glob.glob(os.path.join(out_dir, '*.txt')))} files -> {dest} ({len(lines)} lines)")
