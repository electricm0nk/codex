"""Lists the self_key_probe.py hits still left (rule id | pool | MissingRule name | closure row). Run from the repo root:
python3 -I docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_3/E3.23/remaining_self_key_hits.py"""
import glob, json
def missing(o, acc):
    if isinstance(o, dict):
        if isinstance(o.get("MissingRule"), dict): acc.append(o["MissingRule"])
        for v in o.values(): missing(v, acc)
    elif isinstance(o, list):
        for v in o: missing(v, acc)
for path in sorted(glob.glob("data/sheet_rules/*/*/*.json")):
    for rule in json.load(open(path)):
        acc=[]; missing(rule.get("applies"), acc)
        own = rule["id"].split(":")[-1].split("#")[0]
        for m in acc:
            n=m.get("name") or ""
            if n.lower().replace(" ~ ", "_").replace(" ", "_").replace("'", "") == own:
                print(rule["id"], "|", m["pool"], "|", n, "|", rule["provenance"]["closure_rows"][:1])
