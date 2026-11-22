#!/usr/bin/env python3
"""E0.4 independent reviewer: re-derive every SF seed hand value from the SRD constants.

The constants below were read by the reviewer from pages re-fetched on 2026-10-02
(E0.4-review-fetch-log.txt); the build inputs (ability scores, ranks, gear) are the seed-builds.md
choices, re-checked against the SRD's point-buy / skill-rank rules in the asserts below.
Nothing is read from seed-hand-values.md until compare() runs.

Usage (from the repo root):
  python3 docs/release/SD-37-starfinder-1e/artifacts/epic_0/E0.4_review_derive.py          # compare, print summary
  python3 docs/release/SD-37-starfinder-1e/artifacts/epic_0/E0.4_review_derive.py --table  # print the review table
Exit 1 if any value disagrees or a row is missing on either side.
"""
import re
import sys
from pathlib import Path

C = "https://www.aonsrd.com/"
SKILLS = {  # Skills.aspx?ItemName=All headings: ability, ACP, trained only
    "Acrobatics": ("Dex", True, False), "Athletics": ("Str", True, False),
    "Bluff": ("Cha", False, False), "Computers": ("Int", False, True),
    "Culture": ("Int", False, True), "Diplomacy": ("Cha", False, False),
    "Disguise": ("Cha", False, False), "Engineering": ("Int", False, True),
    "Intimidate": ("Cha", False, False), "Life Science": ("Int", False, True),
    "Medicine": ("Int", False, True), "Mysticism": ("Wis", False, True),
    "Perception": ("Wis", False, False), "Physical Science": ("Int", False, True),
    "Piloting": ("Dex", False, False), "Profession": ("Wis", False, True),
    "Sense Motive": ("Wis", False, False), "Sleight of Hand": ("Dex", True, True),
    "Stealth": ("Dex", True, False), "Survival": ("Wis", False, False),
}
# Classes.aspx?ItemName=<Class>: HP, SP, skill ranks/level, class skills, table rows (BAB, F, R, W)
CLASSES = {
    "Soldier": dict(hp=7, sp=7, ranks=4, key="Str",
                    cs={"Acrobatics", "Athletics", "Engineering", "Intimidate", "Medicine", "Piloting", "Profession", "Survival"},
                    table={3: (3, 3, 1, 3)}),
    "Mystic": dict(hp=6, sp=6, ranks=6, key="Wis",
                   cs={"Bluff", "Culture", "Diplomacy", "Disguise", "Intimidate", "Life Science", "Medicine", "Mysticism", "Perception", "Profession", "Sense Motive", "Survival"},
                   table={5: (3, 1, 1, 4)}, spd={5: (4, 2)}, known={5: (6, 4, 3)}),
    "Technomancer": dict(hp=5, sp=5, ranks=4, key="Int",
                         cs={"Computers", "Engineering", "Life Science", "Mysticism", "Physical Science", "Piloting", "Profession", "Sleight of Hand"},
                         table={5: (3, 1, 1, 4)}, spd={5: (4, 2)}, known={5: (6, 4, 3)}),
    "Envoy": dict(hp=6, sp=6, ranks=8, key="Cha",
                  cs={"Acrobatics", "Athletics", "Bluff", "Computers", "Culture", "Diplomacy", "Disguise", "Engineering", "Intimidate", "Medicine", "Perception", "Piloting", "Profession", "Sense Motive", "Sleight of Hand", "Stealth"},
                  table={3: (2, 1, 3, 3)}),
}
RACE_HP = {"Human": 4, "Lashunta": 4, "Android": 4, "Ysoki": 2}
# ArmorDisplay pages: EAC, KAC, max Dex, ACP
ARMOR = {"Defiance Series, Squad": (5, 8, 1, -4), "Lashunta Tempweave, Basic": (4, 4, 5, 0),
         "D-Suit I": (5, 6, 5, 0), "Carbon Skin, Graphite": (3, 4, 4, -1)}


def mod(score):  # Rules.aspx?ID=42 Table 2-1
    return score // 2 - 5


def bonus_spells(score, lvl):  # Mystic/Technomancer Bonus Spells table: 12-13 → 1st; 14-15 → 1st,2nd; ...
    m = mod(score)
    return 1 if 1 <= lvl <= m else 0


SEEDS = {
    "SF-Soldier-3": dict(race="Human", cls="Soldier", lvl=3,
                         pb=dict(base={"Str": 13}, spent={"Str": 3, "Dex": 4, "Con": 2, "Int": 1}),
                         final=dict(Str=16, Dex=14, Con=12, Int=11, Wis=10, Cha=10), skilled=1,
                         ranks={"Athletics": 3, "Intimidate": 3, "Medicine": 3, "Piloting": 3, "Survival": 3},
                         armor="Defiance Series, Squad",
                         misc={"Athletics": [(1, "Mercenary theme knowledge")]}),
    "SF-Mystic-5": dict(race="Lashunta", cls="Mystic", lvl=5,
                        pb=dict(base={"Con": 8, "Int": 12, "Cha": 12, "Wis": 11}, spent={"Dex": 2, "Wis": 7, "Cha": 1}),
                        at1=dict(Str=10, Dex=12, Con=8, Int=12, Wis=18, Cha=13), inc=("Dex", "Int", "Wis", "Cha"),
                        final=dict(Str=10, Dex=14, Con=8, Int=14, Wis=19, Cha=15), skilled=0,
                        ranks={s: 5 for s in ["Bluff", "Culture", "Diplomacy", "Life Science", "Medicine", "Mysticism", "Perception", "Sense Motive"]},
                        armor="Lashunta Tempweave, Basic",
                        misc={"Diplomacy": [(2, "Lashunta Student")], "Medicine": [(2, "Lashunta Student")],
                              "Mysticism": [(1, "Priest theme knowledge")],
                              "Perception": [(2, "channel skill (Empath)")], "Sense Motive": [(2, "channel skill (Empath)")]},
                        connection_spells=(1, 1)),
    "SF-Technomancer-5": dict(race="Android", cls="Technomancer", lvl=5,
                              pb=dict(base={"Dex": 12, "Int": 13, "Cha": 8}, spent={"Dex": 2, "Con": 2, "Int": 5, "Wis": 1}),
                              at1=dict(Str=10, Dex=14, Con=12, Int=18, Wis=11, Cha=8), inc=("Dex", "Con", "Int", "Wis"),
                              final=dict(Str=10, Dex=16, Con=14, Int=19, Wis=13, Cha=8), skilled=0,
                              ranks={s: 5 for s in ["Computers", "Engineering", "Life Science", "Mysticism", "Physical Science", "Piloting", "Sleight of Hand", "Perception"]},
                              armor="D-Suit I",
                              misc={"Computers": [(1, "techlore")], "Mysticism": [(1, "techlore")],
                                    "Physical Science": [(1, "Scholar theme knowledge")],
                                    "Sense Motive": [(-2, "Android Flat Affect")]},
                              connection_spells=(0, 0)),
    "SF-Envoy-3": dict(race="Ysoki", cls="Envoy", lvl=3,
                       pb=dict(base={"Str": 8, "Dex": 12, "Int": 12, "Cha": 11}, spent={"Dex": 1, "Con": 2, "Cha": 7}),
                       final=dict(Str=8, Dex=13, Con=12, Int=12, Wis=10, Cha=18), skilled=0,
                       ranks={s: 3 for s in ["Bluff", "Computers", "Culture", "Diplomacy", "Engineering", "Intimidate", "Perception", "Sense Motive", "Stealth"]},
                       armor="Carbon Skin, Graphite",
                       misc={"Culture": [(1, "Icon theme knowledge")], "Engineering": [(2, "Ysoki Scrounger")],
                             "Stealth": [(2, "Ysoki Scrounger")], "Survival": [(2, "Ysoki Scrounger")]}),
}


def check_build(name, s):
    c = CLASSES[s["cls"]]
    assert sum(s["pb"]["spent"].values()) == 10, name  # Rules ID=42 step 4
    start = s.get("at1", s["final"])
    assert max(start.values()) <= 18, name
    if "inc" in s:  # Rules ID=57: +1 if >= 17 else +2, four scores
        assert len(s["inc"]) == 4
        for a in start:
            exp = start[a] + ((1 if start[a] >= 17 else 2) if a in s["inc"] else 0)
            assert exp == s["final"][a], (name, a)
    per_lvl = max(1, c["ranks"] + mod(s["final"]["Int"]) + s["skilled"])  # retroactive (ID=57)
    assert sum(s["ranks"].values()) == per_lvl * s["lvl"], (name, sum(s["ranks"].values()), per_lvl)
    assert max(s["ranks"].values()) <= s["lvl"], name
    assert mod(s["final"]["Con"]) != 0, name


def derive(name, s):
    c, L, f = CLASSES[s["cls"]], s["lvl"], s["final"]
    m = {a: mod(v) for a, v in f.items()}
    bab, fb, rb, wb = c["table"][L]
    eac, kac, maxdex, acp = ARMOR[s["armor"]]
    dex_ac = min(m["Dex"], maxdex)
    out = {"BAB": bab, "Fort": fb + m["Con"], "Ref": rb + m["Dex"], "Will": wb + m["Wis"],
           "HP": RACE_HP[s["race"]] + c["hp"] * L, "Stamina": max(0, c["sp"] + m["Con"]) * L,
           "Resolve": max(1, L // 2) + m[c["key"]], "EAC": 10 + eac + dex_ac, "KAC": 10 + kac + dex_ac}
    for sk, (ab, has_acp, trained_only) in SKILLS.items():
        r = s["ranks"].get(sk, 0)
        if trained_only and r == 0:
            out["Skill: " + sk] = None
            continue
        v = r + (3 if (r > 0 and sk in c["cs"]) else 0) + m[ab] + (acp if has_acp else 0)
        v += sum(b for b, _ in s["misc"].get(sk, []))
        out["Skill: " + sk] = v
    if "spd" in c:
        key = f[c["key"]]
        b1, b2 = c["spd"][L]
        out["Spells per day: 1st"] = b1 + bonus_spells(key, 1)
        out["Spells per day: 2nd"] = b2 + bonus_spells(key, 2)
        k0, k1, k2 = c["known"][L]
        cs1, cs2 = s["connection_spells"]
        out["Spells known: 0"], out["Spells known: 1st"], out["Spells known: 2nd"] = k0, k1 + cs1, k2 + cs2
    return out


def fmt(field, v):
    if v is None:
        return "untrained (trained only)"
    if field in ("HP", "Stamina", "Resolve", "EAC", "KAC") or field.startswith("Spells"):
        return str(v)
    return ("+" if v >= 0 else "−") + str(abs(v))


def transcribed(path):
    rows = {}
    for line in Path(path).read_text().splitlines():
        if re.match(r"^\| *SF-", line):
            cols = [x.strip() for x in line.split("|")]
            rows[(cols[1], cols[2])] = cols[3]
    return rows


def main():
    here = Path(__file__).resolve().parent
    mine = {}
    for name, s in SEEDS.items():
        check_build(name, s)
        for field, v in derive(name, s).items():
            mine[(name, field)] = fmt(field, v)
    theirs = transcribed(here / "seed-hand-values.md")
    keys = list(mine) + [k for k in theirs if k not in mine]
    bad = 0
    lines = []
    for i, k in enumerate(keys, 1):
        a, b = theirs.get(k, "MISSING"), mine.get(k, "MISSING")
        verdict = "agree" if a == b else "disagree"
        bad += verdict == "disagree"
        lines.append(f"| R{i} | {k[0]} | {k[1]} | {a} | {b} | {verdict} |")
    if "--table" in sys.argv:
        print("| # | Seed | Field | Transcriber | Reviewer | Verdict |")
        print("|---|---|---|---|---|---|")
        print("\n".join(lines))
    print(f"reviewed={len(keys)} transcribed={len(theirs)} derived={len(mine)} disagree={bad}", file=sys.stderr)
    sys.exit(1 if bad else 0)


if __name__ == "__main__":
    main()
