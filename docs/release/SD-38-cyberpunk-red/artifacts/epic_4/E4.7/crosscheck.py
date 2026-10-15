#!/usr/bin/env python3
"""E4.7 second implementation: re-derive each `expect` string of the three `sheet-cyberpunk-*` ui-smoke
rows from the book's formulas and the corpus records, with no engine or TSX code, and check each one
is on the captured live sheet text.

    python3 -I crosscheck.py <spec.json> <corpus dir> <row id>=<live sheet .txt> [...]

Inputs per row: the STAT table and the skill / cyberware / program lines read off the live sheet (the
saved character's own values: the Edgerunner STATs are rolled per run) and the corpus records. Every
expect is put in exactly one class:
  derived   -- recomputed here from a formula (cited) and compared;
  constant  -- a value the book fixes for a new character (cited), compared to the typed value;
  identity  -- a name with no number (the character name, the system line, a program, the deck).
Exit 1 on any mismatch, on an expect missing from the live text, or on an unclassified expect.
"""
import json
import math
import re
import sys

spec_path, corpus = sys.argv[1], sys.argv[2]
pairs = dict(a.split("=", 1) for a in sys.argv[3:])
spec = {r["id"]: r for r in json.load(open(spec_path, encoding="utf-8"))["rows"]}


def load(kind):
    return {r["name"]: r for r in json.load(open(f"{corpus}/{kind}.json", encoding="utf-8"))}


weapons, armor, cyberware, decks = load("weapon"), load("armor"), load("cyberware"), load("cyberdeck")
range_rows = json.load(open(f"{corpus}/range_dv.json", encoding="utf-8"))
DASH = "—"
bad = 0

for row_id, live_path in pairs.items():
    text = open(live_path, encoding="utf-8").read()
    lines = text.split("\n")
    stats, current = {}, {}
    for line in lines:
        m = re.fullmatch(r"(INT|REF|DEX|TECH|COOL|WILL|LUCK|MOVE|BODY|EMP)\t(\d+)\t(\d+|" + DASH + ")", line)
        if m and m.group(1) not in stats:
            stats[m.group(1)] = int(m.group(2))
            current[m.group(1)] = int(m.group(2)) if m.group(3) == DASH else int(m.group(3))
    skills = {}
    for line in lines:
        m = re.fullmatch(r"(.+) \((INT|REF|DEX|TECH|COOL|WILL|LUCK|MOVE|BODY|EMP)\)\t(\d+)\t(\d+)", line)
        if m:
            skills[m.group(1)] = (m.group(2), int(m.group(3)), int(m.group(4)))
    # Cyberware block: name<TAB>HL lines between CYBERWARE and the Humanity Loss total.
    block = lines[lines.index("CYBERWARE") + 1:]
    installed = []
    for line in block:
        if line.startswith("Humanity Loss\t"):
            break
        name, _, hl = line.partition("\t")
        if name in cyberware:
            installed.append((name, int(hl)))
    hl_total = sum(cyberware[n]["humanity_loss_preset"] for n, _ in installed)  # preset loss at creation (CUI F-66)
    max_h = 10 * stats["EMP"]  # p. 79-80: Humanity = EMP x 10
    humanity = max_h - hl_total
    pieces = sum(1 for n, _ in installed if cyberware[n]["humanity_loss_preset"] > 0)
    hp = 10 + 5 * math.ceil((stats["BODY"] + stats["WILL"]) / 2)  # p. 79
    programs = sum(1 for line in lines if line.startswith("Program\t"))

    def base(skill_name):
        stat, level, _ = skills[skill_name]
        return current[stat] + level  # STAT + Skill (p. 128); current STAT (D-6 EMP, armour penalty 0 here)

    derived = {}
    for skill_name, (stat, level, _) in skills.items():
        derived[f"{skill_name} ({stat})\t{level}\t{base(skill_name)}"] = "STAT + level (p. 128)"
    derived[f"EMP\t{stats['EMP']}\t{humanity // 10}"] = "current EMP = tens place of Humanity (p. 80-81, D-6)"
    derived[f"Humanity\t{humanity} / {max_h}"] = "Humanity = 10 x EMP - preset Humanity Loss (p. 79-80)"
    derived[f"Humanity {humanity} = EMP {stats['EMP']} x 10 - {hl_total}"] = "same, printed line"
    derived[f"Humanity Loss\t{hl_total}"] = "sum of the installed pieces' preset Humanity Loss (CUI F-66, p. 359-368)"
    derived[f"Maximum Humanity\t{max_h - 2 * pieces}"] = "10 x EMP - 2 per installed piece with loss > 0 (p. 230 therapy table)"
    for n, _ in installed:
        derived[f"{n}\t{cyberware[n]['humanity_loss_preset']}"] = "corpus humanity_loss_preset"
    fashion = sum(cyberware[n]["slots_used"] for n, _ in installed if cyberware[n]["slot_pool"] == "fashion")
    derived[f"Fashionware slots\t{fashion} / 7"] = "fashion pool capacity 7 (corpus pool_capacity)"
    derived[f"Internal Body slots\t0 / 7"] = "no internal-body piece installed; capacity 7 (corpus pool_capacity)"
    if "Neural Link" in dict(installed):
        used = sum(cyberware[n]["slots_used"] for n, _ in installed if cyberware[n]["slot_pool"] == "neural_link")
        derived[f"Neural Link slots\t{used} / {cyberware['Neural Link']['slots_provided']}"] = "Neural Link slots_provided (p. 360)"
    derived[f"Lightly Wounded\t{hp}\t10"] = "HP = 10 + 5 x ceil((BODY + WILL) / 2) (p. 79); Stabilize DV 10 (p. 186)"
    derived[f"Seriously Wounded\t{math.ceil(hp / 2)}\t13"] = "ceil(HP / 2) (p. 186); DV 13 (p. 186)"
    derived[f"Facedown (COOL + Reputation)\t{stats['COOL']}"] = "COOL + Reputation 0 (p. 194)"
    derived[f"Facedown roll\t{stats['COOL']} + 1d10"] = "same, dice literal"
    derived[f"Deck slots (used / available)\t{programs} / {decks['Cyberdeck (Standard Quality)']['slots']}"] = "programs on the deck / corpus cyberdeck slots"
    for w in weapons.values():
        if w["skill"] and w["skill"] in skills:
            conceal = "Yes" if w["can_be_concealed"] else "No"
            derived[f"{w['name']}\t{w['damage']}\t{w['rof']}\t{w['hands']}\t{conceal}\t{w['standard_magazine']}\t{base(w['skill'])}"] = "corpus weapon record (p. 342) + its skill base"
        if w["skill"] in skills and "Autofire" in (w["alt_fire_modes"] or "") and "Autofire" in skills:
            derived[f"Autofire Skill Base {base('Autofire')}"] = "REF + Autofire level"
    for r in range_rows:
        cells = [str(b["dv"]) if b["dv"] is not None else "N/A" for b in r["bands"]]
        cells = (cells + [DASH] * 8)[:8] if r["mode"] == "autofire" else cells
        for wt in r["weapon_types"]:
            label = f"{wt} (Autofire)" if r["mode"] == "autofire" else wt
            derived["\t".join([label] + cells)] = f"corpus range_dv '{r['name']}' (p. 173, D-1)"
    derived["Body\t11\t0"] = derived["Head\t11\t0"] = "Light Armorjack SP 11, penalty 0 (corpus armor, p. 351)"
    if armor["Light Armorjack"]["sp"] != 11 or armor["Light Armorjack"]["armor_penalty_ref_dex_move"] != 0:
        raise SystemExit("corpus Light Armorjack is not SP 11 / penalty 0")
    constants = {
        "Rank\t4": "Role Ability Rank 4 at creation (p. 143)",
        "Combat Awareness Points\t4": "points = Combat Awareness Rank (p. 146)", "Combat Awareness points 4": "same, printed",
        "Combat Awareness Rank 4": "rank 4", "Haggle Bonus\t4": "Haggle = Operator Rank (p. 159)",
        "Haggle bonus +4": "same, printed", "Operator Rank 4": "rank 4", "Interface Rank 4": "rank 4",
        "Net Actions\t3": "Interface Rank 4 to 6 -> 3 NET Actions (p. 147 table)", "NET Actions\t3": "same, deck block",
        "NET Actions per Turn 3": "same, printed", "Reputation Level\t0": "new character, no Reputation event",
        "Improvement Points (current / total)\t0 / 0": "new character, no IP awarded",
        "Mortally Wounded\t1\t15": "below 1 HP; DV 15 (p. 186)",
        "Base Death Save Penalty\t0\t" + DASH: "no Critical Injury recorded (p. 188)",
        "Head\t0\t0": "no head armour bought", "Armor Penalty (REF, DEX, MOVE)\t" + DASH + "\t0": "Light Armorjack penalty 0",
        "armor SP does not apply": "brain damage ignores armour (D-2)",
    }
    row = spec[row_id]
    counts = {"derived": 0, "constant": 0, "identity": 0}
    for e in row["expect"]:
        if e not in text:
            print(f"  {row_id}: MISSING on live sheet: {e!r}")
            bad += 1
        if e in derived:
            counts["derived"] += 1
        elif e in constants:
            counts["constant"] += 1
        elif e in (row["marker"], "Cyberpunk RED") or e.startswith("Program\t") or e.startswith("Cyberdeck\tCyberdeck"):
            counts["identity"] += 1
        else:
            # a skill/humanity/weapon-shaped line the formulas give a different value for
            print(f"  {row_id}: NOT RE-DERIVED: {e!r}")
            bad += 1
    print(f"{row_id}: {len(row['expect'])} expects = {counts['derived']} derived + {counts['constant']} constant + {counts['identity']} identity; on live sheet {sum(e in text for e in row['expect'])}/{len(row['expect'])}")

print("CROSSCHECK", "FAIL" if bad else "PASS", f"({bad} problems)")
sys.exit(1 if bad else 0)
