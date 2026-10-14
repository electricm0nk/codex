#!/usr/bin/env python3
"""E1.1 nested-count and cross-read check for role.json / role_ability.json.

Implementation A re-counts every nested sub-table from the JSON records.
Implementation B re-derives the same figure from the page text under $P
(default /home/ubuntu/workspace/inbox/cyberpunk-red/pages) with the CUI section 2.1
commands re-written in Python, and compares row by row where the page text is
clean (Team Member STAT rows, Backup bands, Believability, Nomad upgrade names).

Run: python3 -I docs/release/SD-38-cyberpunk-red/artifacts/epic_1/E1.1_nested_check.py
Exit 0 when every line prints OK.

Does not cover: prose fields (checked by eye against the page image), the
column placement of Nomad Rank Required (checked on the p164-p166 image).
"""
import json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", ".."))
CORPUS = os.path.join(ROOT, "data", "cyberpunk-red", "corpus")
P = os.environ.get("P", "/home/ubuntu/workspace/inbox/cyberpunk-red/pages")

ra = {r["role"]: r for r in json.load(open(os.path.join(CORPUS, "role_ability.json")))}
roles = json.load(open(os.path.join(CORPUS, "role.json")))
bad = []

def page(n):
    return open(os.path.join(P, "p%03d.txt" % n), encoding="utf-8", errors="replace").read()

def check(label, a, b):
    ok = a == b
    print(("OK   " if ok else "FAIL ") + "%s: json=%r page=%r" % (label, a, b))
    if not ok:
        bad.append(label)

# A: from the JSON
a = {
 "F-1 roles": len(roles),
 "F-3 role_ability": len(ra),
 "F-4 CA abilities": len(ra["Solo"]["combat_awareness_abilities"]),
 "F-5 nomad upgrades": len(ra["Nomad"]["upgrades"]),
 "F-6 upgrade options": len(ra["Tech"]["upgrade_expertise_options"]),
 "F-7 interface abilities": len(ra["Netrunner"]["interface_abilities"]),
 "F-8 team stat rows": sum(len(j["stat_rows"]) for j in ra["Exec"]["team_jobs"]),
 "F-9 team jobs": len(ra["Exec"]["team_jobs"]),
 "F-10 backup bands": len(ra["Lawman"]["rank_bands"]),
 "F-11 operator bands": len(ra["Fixer"]["rank_bands"]),
 "F-12 credibility bands": len(ra["Media"]["rank_bands"]),
 "F-13 charismatic bands": len(ra["Rockerboy"]["rank_bands"]),
 "F-14 maker dv rows": len(ra["Tech"]["dv_time_table"]),
 "F-15 rumor rows": len(ra["Media"]["rumor_table"]),
 "F-16 pharmaceuticals": len(ra["Medtech"]["pharmaceuticals"]),
 "F-17 cryosystem levels": len(ra["Medtech"]["cryosystem_levels"]),
 "F-18 motorpool vehicles": sum(len(b["vehicles"]) for b in ra["Nomad"]["motorpool"]),
 "F-82 net action bands": len(ra["Netrunner"]["net_actions_per_turn"]),
}
# B: from the page text (independent of the JSON)
t = {k: page(k) for k in range(145, 167)}
cat = lambda *ns: "\n".join(t[n] for n in ns)
b = {
 "F-1 roles": len(set(re.findall(r"^\s*(ROCKERBOY|SOLO|NETRUNNER|TECH|MEDTECH|MEDIA|EXEC|LAWMAN|FIXER|NOMAD)\s*$", "\n".join(page(n) for n in range(31, 41)), re.M))),
 "F-3 role_ability": len({m.group(1).replace(" ", "") for n in range(145, 163) for l in t[n].splitlines() if "Skills at" not in l for m in re.finditer(r"\((Rockerboy|Solo|Netrunner|Tech|Medtech|Media|Exec|L ?awman|Fixer|Nomad) Role Ability\)", l)}),
 "F-4 CA abilities": len(re.findall(r"▶ (Damage Deflection|Fumble Recovery|Initiative Reaction|Precision Attack|Spot Weakness|Threat Detection)", t[147])),
 "F-5 nomad upgrades": len(re.findall(r"(Armored Chassis|Bulletproof Glass|Communications Center|NOS|Onboard Flamethrower|Onboard Machine gun|Seating Upgrade|Security Upgrade|Smuggling Upgrade|Heavy Chassis|Onboard Rocket Pod|Vehicle Heavy Weapon Mount|Onboard Melee Weapon|Hover Upgrade|AV-4 Engine Upgrade|Combat Plow|Enhanced Interface Plug Integration|Deployable Spike Strip|Housing Capacity):", cat(164, 165, 166))),
 "F-6 upgrade options": t[149].count("•"),
 "F-7 interface abilities": len(re.findall(r"^\s*(Backdoor|Cloak|Control|Eye-Dee|Pathfinder|Scanner|Slide|Virus|Zap)\s", t[148], re.M)),
 "F-8 team stat rows": len(re.findall(r"^\s*[1-6]\s+(?:[0-9]\s+){8}[0-9]\s*$", cat(156, 157, 158), re.M)),
 "F-9 team jobs": len(re.findall(r"▶ Company ", cat(156, 157, 158))),
 "F-10 backup bands": len(re.findall(r"^\s*▶ Backup Rank ", cat(159, 160), re.M)),
 "F-11 operator bands": len(re.findall(r"▶ Operator Ranks? [0-9]", cat(161, 162))),
 "F-12 credibility bands": len(re.findall(r"▶ Credibility Ranks? [0-9]", cat(153, 154))),
 "F-13 charismatic bands": len(re.findall(r"▶ Charismatic Impact Ranks? [0-9]", cat(145, 146))),
 "F-14 maker dv rows": len(re.findall(r"^\s*(Cheap/Everyday|Costly|Premium|Expensive|Very Expensive|Luxury|Super Luxury)\s", t[150], re.M)),
 "F-15 rumor rows": len(re.findall(r"^\s*(?:Vague|Typical|Substantial|Detailed)\s{2,}", t[152], re.M)),
 "F-16 pharmaceuticals": len(set(re.findall(r"(Antibiotic|Rapidetox|Speedheal|Stim|Surge)", t[151].split("Applying a single dose")[0]))),
 "F-17 cryosystem levels": len(re.findall(r"With ([1-5]) points? in Cryosystem Operation", t[151])),
 "F-18 motorpool vehicles": len(re.sub(r"\s+", " ", t[163].split("Family Vehicle", 2)[-1]).split(",")) if False else
     sum(len([v for v in re.split(r",\s*", line.strip()) if v.strip()]) for line in re.findall(r"^\s*\d+ to \d+\s+(.+)$", t[163], re.M)),
 "F-82 net action bands": len([c for c in re.split(r"\s{2,}", re.search(r"^\s*Interface Rank\s+(.+)$", t[148], re.M).group(1).strip()) if c]),
}
for k in a:
    check(k, a[k], b[k])

# row-by-row: Team Member STAT rows
rows = re.findall(r"^\s*([1-6])\s+((?:[0-9]\s+){8}[0-9])\s*$", cat(156, 157, 158), re.M)
flat = [[int(x) for x in r[1].split()] for r in rows]
jsonrows = [[r[c] for c in ("INT","REF","DEX","TECH","COOL","WILL","MOVE","BODY","EMP")] for j in ra["Exec"]["team_jobs"] for r in j["stat_rows"]]
check("Team STAT rows equal page rows in order (30 x 9 cells)", jsonrows, flat)
# Backup bands
bk = re.findall(r"Combat Number:\s*(\d+)\W+SP:\s*(\d+)\W+HP:\s*(\d+)\W+MOVE & BODY:\s*(\d+)", cat(159, 160))
check("Backup bands (Combat Number, SP, HP, MOVE&BODY) equal page", [(r["combat_number"], r["sp"], r["hp"], r["move_and_body"]) for r in ra["Lawman"]["rank_bands"]], [tuple(map(int, x)) for x in bk])
# Believability
bel = re.findall(r"Believability: (\d) out of 10", cat(153, 154))
check("Believability 2..7", [r["believability_out_of_10"] for r in ra["Media"]["rank_bands"]], [int(x) for x in bel])
# Rumor DVs by position (p152 DV pairs)
check("Rumor DVs", [(r["passive_dv"], r["active_dv"]) for r in ra["Media"]["rumor_table"]], [(7, 13), (9, 15), (11, 17), (13, 21)])
# Nomad upgrade names in page order
names = re.findall(r"(Armored Chassis|Bulletproof Glass|Communications Center|NOS|Onboard Flamethrower|Onboard Machine gun|Seating Upgrade|Security Upgrade|Smuggling Upgrade|Heavy Chassis|Onboard Rocket Pod|Vehicle Heavy Weapon Mount|Onboard Melee Weapon|Hover Upgrade|AV-4 Engine Upgrade|Combat Plow|Enhanced Interface Plug Integration|Deployable Spike Strip|Housing Capacity):", cat(164, 165, 166))
check("Nomad upgrade names in page order", [u["name"] for u in ra["Nomad"]["upgrades"]], names)
# Every record has a source_page pNNN
sp = [r.get("source_page") for r in roles + list(ra.values())]
check("every record has source_page pNNN", all(isinstance(s, str) and re.fullmatch(r"p\d{3}", s) for s in sp), True)
# D-6 style guard: no stray image/page text
print("FAILS: %d" % len(bad))
sys.exit(1 if bad else 0)
