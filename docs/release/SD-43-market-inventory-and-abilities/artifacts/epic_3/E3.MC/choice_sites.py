"""E3.MC: E0.1's 1,329 class-granted choice sites (intake 22c), one row each, with the converted rules of the
granting record (matched by the record's oracle `source.path:line` against each rule's
`provenance.closure_rows`). The engine census `ability_budget_class_features_census_every_class_granted_choice_site_is_reachable_or_a_named_card`
reads `choice_sites.json`. Run from the repo root:
    python3 -I docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_3/E3.MC/choice_sites.py [--check]
"""
import collections, glob, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE, "..", "..", "epic_0"))
import abilities_census as census  # noqa: E402  E0.1's instrument: the same sites, the same grantor rule

OUT = os.path.join(HERE, "choice_sites.json")


def build() -> list:
    recs = census.load_corpus(".")
    sites = census.choice_sites_a(recs, census.class_names(recs), census.archetype_names(recs))
    by_path = {r["path"]: r for r in recs}
    rows_to_rules = collections.defaultdict(set)
    for f in glob.glob("data/sheet_rules/*/*/*.json"):
        try:
            rules = json.load(open(f, encoding="utf-8"))
        except Exception:
            continue
        for rule in rules if isinstance(rules, list) else []:
            for row in (rule.get("provenance") or {}).get("closure_rows") or []:
                rows_to_rules[row].add(rule["id"])
    out = []
    for path, mech, target, grantor in sorted(sites):
        src = json.load(open(path, encoding="utf-8")).get("source") or {}
        out.append({
            "path": path, "mechanism": mech, "target": target, "grantor": grantor,
            "owner": by_path[path]["key"].split(" ~ ", 1)[0].strip(),
            "rules": sorted(rows_to_rules.get(f"{src.get('path')}:{src.get('line')}", ())),
        })
    return out


def main() -> int:
    rows = build()
    text = json.dumps(rows, indent=0) + "\n"
    unmatched = sum(1 for r in rows if not r["rules"])
    print(f"choice_sites: {len(rows)} sites; {unmatched} with no converted rule")
    if "--check" in sys.argv:
        ok = os.path.isfile(OUT) and open(OUT, encoding="utf-8").read() == text
        print("choice_sites:", "OK" if ok else "STALE")
        return 0 if ok else 1
    open(OUT, "w", encoding="utf-8").write(text)
    return 0


if __name__ == "__main__":
    sys.exit(main())
