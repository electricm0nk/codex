"""Second implementation: rules whose `applies` holds `Rule(<own base id>)` (a resolved self reference) at REV, over
the sheet-rule files the E3.23 work commit changed (every other file is byte-identical at both revisions). Run from the
repo root: python3 -I docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_3/E3.23/self_rule_count.py WORK REV"""
import json, subprocess, sys
work, rev = sys.argv[1], sys.argv[2]
files = subprocess.run(["git", "diff", "--name-only", f"{work}~1", work, "--", "data/sheet_rules"], capture_output=True, text=True).stdout.split()
def held(o, acc):
    if isinstance(o, dict):
        w = o.get("Holds", {}).get("what") if isinstance(o.get("Holds"), dict) else None
        if isinstance(w, dict) and isinstance(w.get("Rule"), str):
            acc.append(w["Rule"])
        for v in o.values(): held(v, acc)
    elif isinstance(o, list):
        for v in o: held(v, acc)
n = total = 0
for f in files:
    if f.count("/") != 4 or "/_" in f or not f.endswith(".json"):
        continue
    for rule in json.loads(subprocess.run(["git", "show", f"{rev}:{f}"], capture_output=True, text=True).stdout):
        total += 1
        acc = []; held(rule.get("applies"), acc)
        if rule["id"].split("#")[0] in acc:
            n += 1
print(f"{rev}: rules {total}; holding their own id in applies: {n}")
