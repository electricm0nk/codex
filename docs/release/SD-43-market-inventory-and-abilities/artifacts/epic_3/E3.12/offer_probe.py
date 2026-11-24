#!/usr/bin/env python3
"""SD-43 E3.12 discovery probe (a PROXY, not the engine): for each of the 29 pools E3.12 gave corpus members, does
the converted package (data/sheet_rules) carry an offer on a rule granting into it, and how many rules does that offer
select (sheet_rule::offer_selects: same pool, every tag)? `ability_budget::pool_offer` lists a Class Features pool's
members from the first such offer; a pool with none lists nothing there. Feat-parent pools may instead be listed by
feat_budget (feat_list_of), which this probe does not model. Usage: python3 -I <this file> (from inside the repo)."""
import glob, json, os, re, subprocess
ROOT = subprocess.run(["git", "rev-parse", "--show-toplevel"], cwd=os.path.dirname(os.path.abspath(__file__)), capture_output=True, text=True).stdout.strip()
POOLS = ["Advanced Masked Performer Talent", "Autumn Witch Hex Choice", "Bloody Jake Terrain", "Cloaked Wolf Bonus Feat",
         "Death Slayer Bonus Feat", "Druid Bear Shaman Feat", "Druid Eagle Shaman Feat", "Druid Lion Shaman Feat",
         "Druid Serpent Shaman Feat", "Druid Wolf Shaman Feat", "Finesse Marksman Bonus Feat", "Hexenhammer Major Hex Choice",
         "Hexenhammer Minor Hex Choice", "Lantern Bearer Favored Enemy", "Lantern Bearer Favored Enemy Bonus",
         "Life Channeler Domain", "Mammoth Rider Born Survivor Feat", "Masked Performer Talent", "Psionic Fighter Starting Feat",
         "Sniper Marksman Bonus Feat", "Spring Witch Hex Choice", "Summer Witch Hex Choice", "Swashbuckler Weapon Proficiency",
         "Tempest Druid Domain Choice", "Undead Master Arcane Bond", "Vexing Daredevil Bonus Stare Feat",
         "Volley Marksman Bonus Feat", "Winter Witch Hex Choice", "Winter Witch Patron"]
slug = lambda s: "_".join(re.findall(r"[a-z0-9]+", s.lower()))
rules = []
for f in glob.glob(os.path.join(ROOT, "data/sheet_rules/*/**/*.json"), recursive=True):
    if "/_" in f[len(ROOT):]:
        continue
    d = json.load(open(f, encoding="utf-8"))
    rules += [r for r in d if isinstance(r, dict) and "id" in r] if isinstance(d, list) else []
none = []
for p in POOLS:
    grants = [r for r in rules if r.get("target") == {"Pool": slug(p)}]
    offer = next((r["offers"] for r in grants if r.get("offers")), None)
    frm = (offer or {}).get("from")
    n = None
    if isinstance(frm, dict) and "Rules" in frm:
        R = frm["Rules"]
        n = sum(1 for r in rules if "#" not in r["id"] and r.get("pool") == R["pool"]
                and all(any(t.lower() == o.lower() for o in r.get("tags", [])) for t in R["tags"]))
    if not n:
        none.append(p)
    print(f"{p} | grants {len(grants)} | offer {'none' if offer is None else json.dumps(frm)} | selects {n}")
print(f"{len(none)} of {len(POOLS)} pools: no converted offer selecting a member")
