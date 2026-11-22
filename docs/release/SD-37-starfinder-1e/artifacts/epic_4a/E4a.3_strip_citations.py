#!/usr/bin/env python3
"""E4a.3 pass 1: drop the list-file extension from every citation under src/rules_core/rules_tables.

Only an extension that directly follows a name character is removed (b1_races.lst -> b1_races).
Anything else (a bare extension, a glob, a sentence about the extension) is reported, not edited,
and is fixed by hand. Neutral ids ("..._lst_129") do not contain the dotted literal and are untouched.
Usage: E4a.3_strip_citations.py [--apply]
"""
import os, re, sys
ROOT = 'src/rules_core/rules_tables'
PAT = re.compile(r'(?<=[A-Za-z0-9_])\.lst\b')
apply = '--apply' in sys.argv
changed_files = removed = 0
left = []
for dp, _, fs in os.walk(ROOT):
    for f in sorted(fs):
        p = os.path.join(dp, f)
        raw = open(p, 'rb').read()
        text = raw.decode('utf8')
        new, n = PAT.subn('', text)
        if n:
            removed += n
            changed_files += 1
            if apply:
                open(p, 'wb').write(new.encode('utf8'))
        for i, l in enumerate(new.split('\n')):
            if '.lst' in l:
                left.append((p, i + 1, l.strip()[:170]))
print(f'files_changed={changed_files} occurrences_removed={removed} lines_left={len(left)}')
for x in left: print(*x, sep=' | ')
