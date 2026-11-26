"""E3.25: every `"Rule":"<id>"` reference a changed rule file lost vs a base commit -- did the same <id> gain a
reference in another changed file (moved: the reference now resolves to a different target) or vanish?
Run from the repo root: python3 -I docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_3/E3.25/lost_refs_moved.py <base>"""
import collections, re, subprocess, sys
BASE = sys.argv[1] if len(sys.argv) > 1 else "HEAD"
rx = re.compile(r'"Rule":"([^"]+)"')
files = [l for l in subprocess.run(["git", "diff", "--name-only", BASE, "--", "data/sheet_rules"], capture_output=True, text=True).stdout.splitlines() if not l.split("/")[2].startswith("_")]
lost, gained = [], collections.defaultdict(set)
for f in files:
    o = collections.Counter(rx.findall(subprocess.run(["git", "show", f"{BASE}:{f}"], capture_output=True, text=True).stdout))
    n = collections.Counter(rx.findall(open(f).read()))
    lost += [(f, k) for k in (o - n)]
    for k in n - o:
        gained[k].add(f)
moved = [(f, k) for f, k in lost if gained[k] - {f}]
vanished = [(f, k) for f, k in lost if not gained[k] - {f}]
print(f"changed rule files {len(files)}; distinct lost (file, id) {len(lost)}; moved {len(moved)}; vanished {len(vanished)}")
for f, k in vanished:
    print("vanished", f, k)
