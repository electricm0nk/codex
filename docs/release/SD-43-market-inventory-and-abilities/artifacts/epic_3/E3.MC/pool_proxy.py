"""E3.MC second implementation (a PROXY over the raw `data/sheet_rules` JSON, not the engine): per mechanism, how many of
`choice_sites.json`'s sites name a candidate Class Features pool at all (the first gate of the engine census). Run
from the repo root:  python3 -I docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_3/E3.MC/pool_proxy.py"""
import collections, glob, json, os, re

HERE = os.path.dirname(os.path.abspath(__file__))
BASE = {"special_ability", "internal", "feat"}


def slug(name):
    return "_".join(re.findall(r"[a-z0-9]+", name.lower()))


rules = {}
for f in glob.glob("data/sheet_rules/*/*/*.json"):
    try:
        rr = json.load(open(f, encoding="utf-8"))
    except Exception:
        continue
    for r in rr if isinstance(rr, list) else []:
        rules[r["id"]] = r
pool_of_target = lambda r: (r.get("target") or {}).get("Pool") if isinstance(r.get("target"), dict) else None
granted = {pool_of_target(r) for r in rules.values()} - {None}
member_pools = {r.get("pool") for r in rules.values()}


def selection_pool(r):
    frm = ((r.get("offers") or {}).get("from") or {})
    sel = frm.get("Rules") if isinstance(frm, dict) else None
    if not sel or r.get("target") is not None or r.get("subject") != "Character" or "#" in r["id"]:
        return None
    if r["id"].split(":")[1] != "class_feature" or sel.get("tags") or sel["pool"] in BASE:
        return None
    return sel["pool"] if sel["pool"] in member_pools and sel["pool"] not in granted else None


counts = collections.Counter()
for s in json.load(open(os.path.join(HERE, "choice_sites.json"), encoding="utf-8")):
    rs = [rules[i] for i in s["rules"] if i in rules]
    m = s["mechanism"]
    if m == "abilitypool":
        has = any(pool_of_target(r) == slug(s["target"]) for r in rs)
    elif m == "domain_count":
        has = any(pool_of_target(r) and ("domain" in pool_of_target(r) or "Domains" == ((r.get("offers") or {}).get("from"))) for r in rs)
    else:
        has = any(selection_pool(r) for r in rs)
    counts[(m, "pool" if has else "no pool")] += 1
for k in sorted(counts):
    print(f"{k[0]} | {k[1]} | {counts[k]}")
print("total", sum(counts.values()))
