import os as _os
OUT_DIR = _os.path.dirname(_os.path.abspath(__file__))
import os, re, sys, types
sys.modules['pf1e_dashboard_producer'] = types.ModuleType('stub')
import importlib.util
spec = importlib.util.spec_from_file_location("build", "docs/release/SD-31-corpus-closure-grind/artifacts/supersession_register_build.py")
build = importlib.util.module_from_spec(spec)
spec.loader.exec_module(build)

ROOT = "/home/ubuntu/workspace/repos/pcgen/data"

key_pattern = re.compile(r'KEY:([A-Za-z0-9\'\- ]+ ~ [A-Za-z0-9\'\-/&,\. ]+)')
all_keys = set()
for dirpath, dirs, files in os.walk(os.path.join(ROOT, "pathfinder/paizo/roleplaying_game/core_essentials/races")):
    for fn in files:
        if fn.endswith('.lst'):
            with open(os.path.join(dirpath, fn), errors='replace') as fh:
                for line in fh:
                    for m in key_pattern.finditer(line):
                        all_keys.add(m.group(1).strip())

IGNORE_PREFIXES = ("SOURCE", "TYPE")
findings = []
for book, reldir in build.BOOK_DIRS.items():
    if book == "core_essentials":
        continue
    bookdir = os.path.join(ROOT, reldir)
    if not os.path.isdir(bookdir):
        continue
    for dirpath, dirs, files in os.walk(bookdir):
        for fn in files:
            if not fn.endswith('.lst'):
                continue
            path = os.path.join(dirpath, fn)
            with open(path, errors='replace') as fh:
                for lineno, line in enumerate(fh, 1):
                    if '.MOD' not in line:
                        continue
                    m = re.match(r'^(?:CATEGORY=Special Ability\|)?([^\t]+)\.MOD\t(.*)$', line.rstrip('\n'))
                    if not m:
                        continue
                    name = m.group(1).strip()
                    if name not in all_keys:
                        continue
                    rest = m.group(2)
                    tags = [t for t in rest.split('\t') if t.strip()]
                    non_source_tags = [t for t in tags if not any(t.startswith(p) for p in IGNORE_PREFIXES)]
                    if non_source_tags:
                        findings.append((book, os.path.relpath(path, ROOT), lineno, name, non_source_tags))

print("in-mandate-scope (excl. core_essentials) .MOD rows on race-trait KEYs with non-SOURCE/TYPE tags:", len(findings))
for f in findings:
    print(f)
