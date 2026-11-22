#!/usr/bin/env python3
"""E1.4 gear-lane checks. Run from the repo root:
    python3 -I docs/release/SD-38-cyberpunk-red/artifacts/epic_1/E1.4_checks.py
Reads the 15 gear kind files under data/cyberpunk-red/corpus/ and, for the source
comparisons, $PDF (default /home/ubuntu/workspace/inbox/cyberpunk-red/cpr-corebook-v122.pdf)
through `pdftotext` and $P (default .../pages) outside the tree. Prints PASS/FAIL per check
and exits 1 on any FAIL.
"""
import json, os, re, subprocess, sys, collections

C = "data/cyberpunk-red/corpus/"
PDF = os.environ.get("PDF", "/home/ubuntu/workspace/inbox/cyberpunk-red/cpr-corebook-v122.pdf")
P = os.environ.get("P", "/home/ubuntu/workspace/inbox/cyberpunk-red/pages")
KINDS = ("price_category weapon ammo attachment clip_chart weapon_quality armor gear fashion drug service "
         "lifestyle housing vehicle night_market_table").split()
D = {k: json.load(open(C + k + ".json", encoding="utf-8")) for k in KINDS}
fails = []

def check(name, ok, detail=""):
    print(("PASS " if ok else "FAIL ") + name + (" -- " + detail if detail else ""))
    if not ok:
        fails.append(name)

def pdf_page(n):
    return subprocess.run(["pdftotext", "-f", str(n), "-l", str(n), PDF, "-"], capture_output=True, text=True).stdout

def sq(s):
    s = s.replace("’", "'").replace("“", '"').replace("”", '"')
    return re.sub(r"\s+", "", s)

# ---- counts (manifest figures; gear is the SD-c correction 53 -> 54)
exp = {"price_category": 8, "weapon": 29, "ammo": 13, "attachment": 8, "clip_chart": 10, "weapon_quality": 3, "armor": 9,
       "gear": 54, "fashion": 10, "drug": 5, "service": 33, "lifestyle": 4, "housing": 11, "vehicle": 14, "night_market_table": 2}
for k, n in exp.items():
    check(f"count {k} = {n}", len(D[k]) == n, f"got {len(D[k])}")
check("weapon classes 4/11/14", dict(collections.Counter(r["weapon_class"] for r in D["weapon"])) == {"melee": 4, "ranged": 11, "exotic": 14})
check("armor sp-bearing 8", sum(1 for r in D["armor"] if r["sp"] is not None) == 8)
check("fashion price cells 90", sum(len(r["prices"]) for r in D["fashion"]) == 90)
check("vehicles 5 land / 4 sea / 5 air", dict(collections.Counter(r["vehicle_type"] for r in D["vehicle"])) == {"Land": 5, "Sea": 4, "Air": 5})
nm = D["night_market_table"]
check("night market 6 type rows, 20 d100 rows x 6 = 120 cells", len(nm[0]["rows"]) == 6 and len(nm[1]["rows"]) == 20 and sum(len(r) - 1 for r in nm[1]["rows"]) == 120)
check("gear in_starter_table 48, in_master 53", sum(r["in_starter_table"] for r in D["gear"]) == 48 and sum(r["in_master_gear_list"] for r in D["gear"]) == 53)

# ---- second implementation of the starter-row count (SD-c): awk-equivalent regexes over the layout pages
def count_rows(rx, files):
    n = 0
    for f in files:
        for line in open(os.path.join(P, f), encoding="utf-8"):
            n += len(re.findall(rx, line))
    return n
a = count_rows(r"[0-9]eb ?\(", ["p101.txt", "p102.txt"])
b = count_rows(r"[0-9]e?b ?\(", ["p101.txt", "p102.txt"])
check("CUI F-53 predicate [0-9]eb ?( over p101+p102 = 47 (misses Road Flare '10b (Cheap)')", a == 47, str(a))
check("typo-tolerant predicate [0-9]e?b ?( over p101+p102 = 48", b == 48, str(b))
check("p352 master rows = 53", count_rows(r"[0-9]e?b ?\(", ["p352.txt"]) == 53)

# ---- rulings
air = next(r for r in D["weapon"] if r["name"] == "Air Pistol")
check("D-3 Air Pistol base_weapon Very Heavy Pistol (p096), p348 Medium Pistol kept in note", air["base_weapon"] == "Very Heavy Pistol" and air["base_weapon_p348"] == "Medium Pistol" and "D-3" in air["transcription_note"])
check("A-6 five attachments carry the note, three state 2 slots", sum("A-6" in r.get("transcription_note", "") for r in D["attachment"]) == 5 and sorted(r["name"] for r in D["attachment"] if r["slots"] == 2) == ["Grenade Launcher Underbarrel", "Shotgun Underbarrel", "Smartgun Link"])
sig = next(r for r in D["gear"] if r["name"].startswith("Linear Frame ∑"))
check("A-8 Linear Frame Sigma keeps both plug counts verbatim", "using two installations of Interface Plugs" in sig["text"] and "Requires 1 installations of Interface Plugs" in sig["text"] and "A-8" in sig["transcription_note"])
d10 = sorted((k, r["name"]) for k in ("gear", "fashion", "housing") for r in D[k] if "D-10" in r.get("transcription_note", ""))
check("D-10 three oddities flagged", d10 == [("fashion", "High Fashion"), ("gear", "Road Flare"), ("housing", "Luxury Penthouse")], str(d10))
pent = next(r for r in D["housing"] if r["name"] == "Luxury Penthouse")
hf = next(r for r in D["fashion"] if r["name"] == "High Fashion")
rf = next(r for r in D["gear"] if r["name"] == "Road Flare")
check("D-10 printed values kept", pent["cost_to_buy_text"] == "50,0000eb" and hf["prices"]["Jewelry"]["cost_text"] == "50,000eb (Luxury)" and rf["starter_cost_text"] == "10b (Cheap)")

# ---- every fashion price cell maps to the p386 scale except the one flagged cell
scale = {r["name"]: r["market_price_eb"] for r in D["price_category"]}
bad = [(r["name"], c) for r in D["fashion"] for c, v in r["prices"].items() if scale[v["price_category"]] != v["cost_eb"]]
check("fashion cells off the price scale = only High Fashion Jewelry", bad == [("High Fashion", "Jewelry")], str(bad))
bad2 = [(k, r["name"]) for k in ("weapon", "ammo", "attachment", "armor", "gear", "service") for r in D[k]
        if r.get("cost_eb") is not None and r.get("price_category") and scale[r["price_category"]] != r["cost_eb"]]
check("every cost_eb equals its price category's market price (weapon, ammo, attachment, armor, gear, service)", bad2 == [], str(bad2[:5]))

# ---- verbatim: sentences of the text fields are on the source pages (PDF reading order, whitespace squashed)
cache = {}
def page(pn):
    n = int(pn[1:])
    if n not in cache:
        cache[n] = sq(pdf_page(n))
    return cache[n]
FIELDS = ("text", "description", "summary_text", "margin_note", "what_it_entails", "section_text", "intro_text", "starter_description")
SKIP_KINDS_FIELDS = {("weapon", "text", "ranged"), ("price_category", "text", None), ("clip_chart", "text", None), ("fashion", "text", None), ("service", "text", None), ("vehicle", "text", None)}
checked = found = 0
missing = []
for k in KINDS:
    for r in D[k]:
        if (k, "text", r.get("weapon_class")) in SKIP_KINDS_FIELDS or (k, "text", None) in SKIP_KINDS_FIELDS:
            fields = [f for f in FIELDS if f != "text"]
        else:
            fields = list(FIELDS)
        pages = [r["source_page"]] + r.get("also_printed_on", []) + [r.get(x) for x in ("description_page", "starter_page", "table_page", "continues_on") if r.get(x)]
        pages += {"gear": ["p352", "p353", "p354", "p355", "p356"], "weapon": ["p348", "p349", "p350"], "drug": ["p359"], "housing": ["p378", "p379", "p380", "p381"], "armor": ["p352", "p097"], "lifestyle": ["p378"], "weapon_quality": ["p343"], "night_market_table": ["p339", "p340"]}.get(k, [])
        blob = "".join(page(p) for p in sorted(set(pages)))
        for f in fields:
            v = r.get(f)
            if not isinstance(v, str):
                continue
            for s in re.split(r"(?<=[.?!:])\s+|\n", v):
                s = s.strip()
                if len(s) < 25 or s.startswith("•") and len(s) < 25:
                    continue
                checked += 1
                s2 = sq(s.lstrip("• "))
                if s2 in blob or s2.replace("-", "") in blob.replace("-", ""):
                    found += 1
                else:
                    missing.append((k, r["name"], f, s[:70]))
print(f"verbatim: {found} of {checked} sentences found in the source pages' text; {len(missing)} not found")
for m in missing:
    print("  NOT FOUND", m)
check("verbatim sentence check: the only unfound sentence is the Microwaver text that straddles the p349/p350 page break (footer number between the halves)", [m[:3] for m in missing] == [("weapon", "Microwaver", "text")], f"{len(missing)} unfound")

print("RESULT", "ALL PASS" if not fails else f"{len(fails)} FAIL: {fails}")
sys.exit(1 if fails else 0)
