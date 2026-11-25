"""PROXY: sheet rules whose `applies` gate names, as a `MissingRule`, a key whose slug equals the rule's own id slug
(e.g. Blue ~ Psionic Prodigy's PREMULT [PREABILITY ... Blue ~ Psionic Prodigy]). The engine reads `MissingRule` as
false, so a gate of that shape cannot be met by holding the rule itself. Run from the repo root:
python3 -I docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_3/E3.21/self_key_probe.py"""
import glob, json

def missing(o, acc):
    if isinstance(o, dict):
        if isinstance(o.get("MissingRule"), dict):
            acc.append(o["MissingRule"].get("name") or "")
        for v in o.values():
            missing(v, acc)
    elif isinstance(o, list):
        for v in o:
            missing(v, acc)

total = selfkey = 0
hits = []
for path in sorted(glob.glob("data/sheet_rules/*/*/*.json")):
    for rule in json.load(open(path)):
        acc = []
        missing(rule.get("applies"), acc)
        if not acc:
            continue
        total += 1
        own = rule["id"].split(":")[-1].split("#")[0]
        if any(n.lower().replace(" ~ ", "_").replace(" ", "_").replace("'", "") == own for n in acc):
            selfkey += 1
            hits.append(rule["id"])
print(f"rules with a MissingRule in applies: {total}; of them naming their own key: {selfkey}")
print("ultimate_psionics:monster_ability:blue_psionic_prodigy" in hits)
