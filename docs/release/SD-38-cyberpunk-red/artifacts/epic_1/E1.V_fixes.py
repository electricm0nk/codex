#!/usr/bin/env python3
"""E1.V corpus fixes: every disagreement the page-image check found, applied once.

Usage (from the repo root):
  python3 -I docs/release/SD-38-cyberpunk-red/artifacts/epic_1/E1.V_fixes.py          # apply (idempotent)
  python3 -I docs/release/SD-38-cyberpunk-red/artifacts/epic_1/E1.V_fixes.py --check  # exit 1 if any fix is unapplied

Each fix names the record, the page image that decides it and the crop box
(150 dpi px unless marked 300 dpi). Substring fixes edit the raw JSON text so
the files' formatting is untouched; the two whole-field replacements re-dump
net_architecture.json with its own formatting (indent 2, UTF-8, trailing newline).
"""
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", ".."))
CORPUS = os.path.join(ROOT, "data", "cyberpunk-red", "corpus")

# (file, old, new, expected occurrences, record ids, evidence)
SUBSTRING_FIXES = [
    ("armor.json", "Kevlar ®", "Kevlar®", 8,
     "armor[1,2,4,5] description+text",
     "p351 300 dpi crop x1300-2200,y2390-2620: 'Kevlar®' printed with no space; text-layer bbox gap 0.2-0.4 pt on all 7 p351 occurrences"),
    ("armor.json", "Metalgear ®", "Metalgear®", 2,
     "armor[7] description+text",
     "p351 text-layer bbox: '®' starts 0.1-0.4 pt after 'Metalgear' (3 occurrences), same as Kevlar® above"),
    ("cyberpsychosis_band.json", "Do your best to role-play accordingly.",
     "Do your best to roleplay accordingly.", 1,
     "cyberpsychosis_band[1] text",
     "p233 x655-1125,y625-725: the hyphen is the line-end break; the text layer prints 'roleplay' and the book's text layer has 52 'roleplay', 0 'role-play'"),
    ("encounter.json", "use Automated Turret on pg. 214 )", "use Automated Turret on pg. 214)", 1,
     "encounter[27] text (Roaming Netrunner)",
     "p422 x130-600,y1130-1515: printed 'PG. 214)' with no space"),
    ("encounter.json", "Kevlar® . ", "Kevlar®. ", 2,
     "encounter[0] text (Local Law), encounter[15] text (Nomad Truck)",
     "p418 300 dpi crop x1250-2350,y880-990 'Kevlar®. If'; p420 x655-1125,y165-380 'Kevlar®. Gangers'"),
    ("skill.json", "(see sidebar on Page 140 ).", "(see sidebar on Page 140).", 7,
     "skill[51,52,53,55,58,64,65] text",
     "p141 x360-1125,y440-1200: printed 'PAGE 140).' with no space (3 of 7 on this page); the text layer of p141-p143 prints 'Page 140).' for all 7"),
    ("gear.json", "• .Scan the Data Pool", "• Scan the Data Pool", 1,
     "gear[0] text (Agent)",
     "p353 300 dpi crop x230-1130,y1290-1380: bullet then 'Scan', no printed full stop"),
]

NA_3_TEXT = " ".join([
    "A NET Architecture will set you back considerably. The more floors you want, the more expensive the price per floor. Additionally, the length of the NET Architecture determines if the physical server that houses it is portable (the size of a large backpack) or not, and determines the maximum amount of Control Nodes (sold separately) you can install in it. Choose wisely, because you can't increase the number of floors in a NET Architecture after purchase. NET Architectures are only available for purchase at a Night Market that includes Personal Electronics, run by a Fixer of Rank 4 or higher.",
    "Once you've got a NET Architecture, you'll want to fill it up. Passwords, Control Nodes, and Files all fill up a floor in your new baby and are all priced based on how difficult you want to make it for a Netrunner to crack them open. NET Architectures are only available for purchase at a Night Market that includes Personal Electronics, run by a Fixer of Rank 4 or higher.",
    "Now that you've got something to protect, you can fill your NET Architecture with Black ICE. To do so , you pay the cost of each Program you want to add individually, and place them as you desire in open floors in your NET Architecture. Black ICE for NET Architecture use is priced exactly the same as Black ICE for Cyberdecks. If you want to add 2 Black ICE to a single floor, you pay double the cost of each Program you put in that floor. If you want to add 3 Black ICE to a single floor, you pay triple the cost of each Program you put in that floor. Demons are special and cannot be placed in doubles or triples. You can only have one Demon per six floors in your Architecture. See the table below for their prices. All Programs designed for NET Architectures are only available for purchase at a Night Market that includes Personal Electronics, run by a Fixer of Rank 4 or higher.",
    "To add the finishing touches to your home defense solution, you can connect each of your Control Nodes to a Defense. If you would rather connect your Control Nodes to something else, talk to your GM. You'll need a method by which your Defenses can discern friend from foe. You may also want a Demon to help manage your Control Nodes, unless you plan on doing so manually.",
    "Defenses are priced individually, based on the difficulty to counter them using the Electronics/Security Tech Skill. These are only available for purchase at a Night Market that includes Personal Electronics, run by a Fixer of Rank 4 or higher (For a list of Defenses see pg. 212).",
])
NA_4_TEXT = " ".join([
    "Type: The name of the Defense. Description: A description of the Defense. Some will offer a number of different options for the GM to choose from when building the Defense. The DV for countering the Defense with Electronics/Security Tech (and the amount of time it takes) is also located here. Default Trigger: The most common method of activating the Defense. GMs should feel free to modify it to their own needs.",
    "Data: Additional notes can be found here. This can include MOVE for Active Defenses, Combat Number for Emplaced Defenses, range of operation, HP, the Perception Check to spot a Defense for Environmental Defenses, and any information about ROF.",
])
# (index, expected name, new text, evidence)
FIELD_FIXES = [
    (3, "Home Security 2045: Buying a NET Architecture", NA_3_TEXT,
     "corpus text was a condensed paraphrase with one unprinted merged sentence; replaced by the printed paragraphs "
     "p218 x130-900,y415-600 + y995-1115 and p219 x150-620,y215-645 + x655-1125,y215-575 (decisions.md section 11: text is verbatim)"),
    (4, "NET Architecture Defenses", NA_4_TEXT,
     "corpus text lower-cased the printed capitals after each label ('Type: the name'); p213 x655-1125,y1200-1510 prints "
     "'Type: The name', 'Description: A description', 'Default Trigger: The most'; the Data line is the p214 text layer (not image-checked)"),
]


def main(argv):
    check = "--check" in argv
    pending = []
    for fname, old, new, n, rec, _ev in SUBSTRING_FIXES:
        p = os.path.join(CORPUS, fname)
        s = open(p, encoding="utf-8").read()
        c_old, c_new = s.count(old), s.count(new)
        if c_old == 0:
            print(f"OK {fname}: {rec} applied ({c_new} present)")
            continue
        if c_old != n:
            print(f"FAIL {fname}: {old!r} occurs {c_old}, expected {n}")
            return 1
        pending.append(f"{fname}: {rec}")
        if not check:
            open(p, "w", encoding="utf-8").write(s.replace(old, new))
            print(f"FIXED {fname}: {rec} ({n} occurrences)")
    p = os.path.join(CORPUS, "net_architecture.json")
    recs = json.load(open(p, encoding="utf-8"))
    changed = False
    for i, name, text, _ev in FIELD_FIXES:
        if recs[i]["name"] != name:
            print(f"FAIL net_architecture[{i}]: name {recs[i]['name']!r}, expected {name!r}")
            return 1
        if recs[i]["text"] == text:
            print(f"OK net_architecture[{i}]: text applied")
            continue
        pending.append(f"net_architecture[{i}] text")
        recs[i]["text"] = text
        changed = True
        print(("PENDING" if check else "FIXED") + f" net_architecture[{i}] text")
    if changed and not check:
        open(p, "w", encoding="utf-8").write(json.dumps(recs, indent=2, ensure_ascii=False) + "\n")
    if check:
        print("fixes: %s" % ("all applied" if not pending else f"{len(pending)} pending"))
        return 1 if pending else 0
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
