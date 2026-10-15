#!/usr/bin/env python3
"""E5.MC: the five ui-smoke rows for E5.2-E5.5 (+ the skipped-Level refusal) as a JSON list on stdout."""
import json


def build(name):
    """Build the Streetrat Solo of improve-cyberpunk-skill (template row 1) named `name`, then load it from the store."""
    return [
        {"op": "click", "target": "Cyberpunk Red"},
        {"op": "click", "target": "New\nCharacter"},
        {"op": "wait", "ms": 1500},
        {"op": "click", "target": "cpr-character-name"},
        {"op": "type", "text": name},
        {"op": "select", "target": "— choose a Role —", "text": "Solo"},
        {"op": "wait", "ms": 1500},
        {"op": "click", "target": "Streetrat"},
        {"op": "select", "target": "Roll the row on 1d10", "text": "1"},
        {"op": "wait", "ms": 1500},
        {"op": "click", "target": "Create character"},
        {"op": "wait", "ms": 3000},
        {"op": "scroll", "ticks": 40, "direction": "up"},
        {"op": "key", "key": "Shift"},
        {"op": "click", "target": "Back"},
    ] + reload(name)


def reload(name):
    return [
        {"op": "click", "target": "Load\nCharacter"},
        {"op": "wait", "ms": 1500},
        {"op": "click", "target": name},
        {"op": "click", "target": "Load"},
        {"op": "wait", "ms": 3000},
    ]


def spend_and_reload(name):
    return [
        {"op": "click", "target": "Spend IP"},
        {"op": "wait", "ms": 3000},
        {"op": "click", "target": "Close"},
        {"op": "wait", "ms": 1500},
        {"op": "click", "target": name},
        {"op": "click", "target": "Load"},
        {"op": "wait", "ms": 3000},
    ]


def open_improve():
    return [{"op": "click", "target": "Improve"}, {"op": "wait", "ms": 2000}]


def typed(target, text):
    return [{"op": "click", "target": target}, {"op": "type", "text": text}, {"op": "wait", "ms": 1000}]


BASE_FORBID = ["Character not created", "This character did not compute", "Race", "Strength"]
SOLO = "Streetrat Solo of improve-cyberpunk-skill (template row 1: COOL 8, EMP 5, Humanity 36 of Maximum 46 after its kit's 14 Humanity Loss, Solo Rank 4)"

rows = []

n = "CPR MC Role Switch Solo"
rows.append({
    "id": "improve-cyberpunk-role-switch",
    "screen": "sheet",
    "steps": build(n) + open_improve() + typed("IP entered", "60")
    + [{"op": "select", "target": "— change Role to —", "text": "Nomad"}, {"op": "wait", "ms": 1000},
       {"op": "click", "target": "Add Role switch"}, {"op": "wait", "ms": 2000}]
    + spend_and_reload(n) + open_improve()
    + [{"op": "select", "target": "— change Role to —", "text": "Fixer"}, {"op": "wait", "ms": 1000},
       {"op": "click", "target": "Add Role switch"}, {"op": "wait", "ms": 2000}],
    "marker": n,
    "expect": [n, "Cyberpunk RED", "Solo\nRank\t4", "Nomad\nRank\t1", "Improvement Points (current / total)\t0 / 60",
               "cannot change Roles: the latest Role, Nomad, is at Rank 1; it must reach Rank 4 first (p. 143)"],
    "forbid": BASE_FORBID + ["Improvement Points (current / total)\t0 / 0", "Fixer\nRank"],
    "notes": f"SD-38 E5.MC for E5.2 (epic-breakdown.md E5.2: 'A Role switch buys rank 1 of a new Role (A-1) only when the current Role is at rank >= 4; both Roles' rank-derived values show'). Builds the {SOLO}, loads it, Improve, enters 60 IP, changes Role to Nomad (A-1: the Role Ability Rank table's Rank 1 price, 60 IP), Spend IP, Close and Load again from the store: the sheet shows both Roles (Solo Rank 4, Nomad Rank 1) and IP 0 / 60. Then Improve again and asks to change Role to Fixer: the engine refuses while the latest Role, Nomad, is below Rank 4 (p. 143), so the plan cannot be spent. A switch that did not save is red (IP 0 / 0 forbidden); a switch gate that let Rank 1 through is red (the refusal is expected). Before E5.MC the row does not exist ('--only <id>: no such row id in spec.json').",
})

n = "CPR MC Cyberware Solo"
rows.append({
    "id": "improve-cyberpunk-cyberware",
    "screen": "sheet",
    "steps": build(n) + open_improve()
    + [{"op": "select", "target": "— install cyberware —", "text": "Gills"}, {"op": "wait", "ms": 1000}]
    + typed("Humanity Loss rolled", "12")
    + [{"op": "click", "target": "Add cyberware"}, {"op": "wait", "ms": 2000}]
    + spend_and_reload(n) + open_improve()
    + [{"op": "select", "target": "— install cyberware —", "text": "Gills"}, {"op": "wait", "ms": 1000}]
    + typed("Humanity Loss rolled", "13")
    + [{"op": "click", "target": "Add cyberware"}, {"op": "wait", "ms": 2000}],
    "marker": n,
    "expect": [n, "Cyberpunk RED", "Gills\t12", "Humanity\t24 / 50", "EMP\t5\t2", "Maximum Humanity\t44", "Internal Body slots\t1 / 7",
               "Humanity Loss 13", "2d6 (2-12)"],
    "forbid": BASE_FORBID + ["Humanity\t36 / 50", "EMP\t5\t3"],
    "notes": f"SD-38 E5.MC for E5.3 (epic-breakdown.md E5.3: 'A cyberware install after creation takes the rolled or entered Humanity Loss within the dice range (A-3), enforces slots (D-11), and recomputes Humanity and EMP'). Builds the {SOLO}, loads it, Improve, installs Gills (Internal Body, 2d6, p. 115) with an entered Humanity Loss of 12 (the top of 2d6), Spend IP, Close and Load again from the store: Gills 12 on the cyberware list, Humanity 36 - 12 = 24, EMP 5 -> 2 (the tens place of Humanity, p. 80-81), Maximum Humanity 46 - 2 = 44 (D-5), Internal Body slots 1 / 7. Then Improve again with a second Gills at Humanity Loss 13: the engine refuses it as outside 2d6 (2-12) (A-3). The pre-install Humanity / EMP rows are forbidden, so an install that did not save is red. Before E5.MC the row does not exist.",
})

n = "CPR MC Therapy Solo"
rows.append({
    "id": "improve-cyberpunk-therapy",
    "screen": "sheet",
    "steps": build(n) + open_improve()
    + [{"op": "select", "target": "— therapy —", "text": "Standard Humanity Loss"}, {"op": "wait", "ms": 1000}]
    + typed("Humanity regained rolled", "12")
    + [{"op": "click", "target": "Add therapy"}, {"op": "wait", "ms": 2000}]
    + spend_and_reload(n) + open_improve()
    + [{"op": "select", "target": "— therapy —", "text": "Standard Humanity Loss"}, {"op": "wait", "ms": 1000}]
    + typed("Humanity regained rolled", "13")
    + [{"op": "click", "target": "Add therapy"}, {"op": "wait", "ms": 2000}],
    "marker": n,
    "expect": [n, "Cyberpunk RED", "Humanity\t46 / 50", "EMP\t5\t4", "Maximum Humanity\t46", "Humanity regained 13 is outside its dice"],
    "forbid": BASE_FORBID + ["Humanity\t36 / 50", "Humanity\t48 / 50", "EMP\t5\t3"],
    "notes": f"SD-38 E5.MC for E5.4 (epic-breakdown.md E5.4: 'Therapy (Standard 2d6, Extreme 4d6, rolled or entered) restores Humanity up to the D-5 cap'). Builds the {SOLO}, loads it, Improve, Standard Humanity Loss therapy (2d6, p. 230) with 12 entered, Spend IP, Close and Load again from the store: Humanity 36 + 12 = 48 stops at the D-5 Maximum Humanity 46 (regains 10), EMP 3 -> 4. Then Improve again with a Standard therapy of 13: refused as outside its dice (2-12). An uncapped regain (48) and an unsaved one (36) are forbidden. Before E5.MC the row does not exist.",
})

n = "CPR MC Reputation Solo"
rows.append({
    "id": "improve-cyberpunk-reputation",
    "screen": "sheet",
    "steps": build(n) + open_improve()
    + typed("Reputation Level", "4")
    + typed("Reputation event", "Took down a cyberpsycho")
    + [{"op": "click", "target": "Add Reputation event"}, {"op": "wait", "ms": 2000}]
    + spend_and_reload(n) + open_improve()
    + typed("Reputation Level", "3")
    + typed("Reputation event", "Paid a debt on time")
    + [{"op": "click", "target": "Add Reputation event"}, {"op": "wait", "ms": 1500}]
    + typed("Reputation Level", "6")
    + [{"op": "select", "target": "deedcowardice", "text": "cowardice"}, {"op": "wait", "ms": 500}]
    + typed("Reputation event", "Ran out on the crew")
    + [{"op": "click", "target": "Add Reputation event"}, {"op": "wait", "ms": 2000}]
    + spend_and_reload(n),
    "marker": n,
    "expect": [n, "Cyberpunk RED", "Reputation Level\t6", "Facedown (COOL + Reputation)\t2", "Facedown roll\t2 + 1d10", "Level 6 cowardice: Ran out on the crew (defining)"],
    "forbid": BASE_FORBID + ["Reputation Level\t0", "Reputation Level\t3", "Facedown (COOL + Reputation)\t14"],
    "notes": f"SD-38 E5.MC for E5.5 (epic-breakdown.md E5.5: 'A reputation change sets the level and event; a new positive deed replaces the old only if higher; a cowardice event makes Facedown use a negative Rep'). Builds the {SOLO}, loads it, Improve, a Level 4 deed, Spend IP, Close and Load again; Improve again with a Level 3 deed (lower: the Level 4 deed stays defining, p. 193) and a Level 6 cowardice (a negative event replaces a positive one, p. 193), Spend IP, Close and Load again from the store: Reputation Level 6, Facedown COOL 8 + (-6) = 2 (p. 194: cowardice makes the Level negative), Facedown roll 2 + 1d10. A Level 3 that replaced the 4, a positive 6 (Facedown 14) and an unsaved Reputation (0) are forbidden. Before E5.MC the row does not exist.",
})

n = "CPR MC Reputation Deed Solo"
rows.append({
    "id": "improve-cyberpunk-reputation-lower-deed",
    "screen": "sheet",
    "steps": build(n) + open_improve()
    + typed("Reputation Level", "4")
    + typed("Reputation event", "Took down a cyberpsycho")
    + [{"op": "click", "target": "Add Reputation event"}, {"op": "wait", "ms": 2000}]
    + spend_and_reload(n) + open_improve()
    + typed("Reputation Level", "3")
    + typed("Reputation event", "Paid a debt on time")
    + [{"op": "click", "target": "Add Reputation event"}, {"op": "wait", "ms": 2000}]
    + spend_and_reload(n),
    "marker": n,
    "expect": [n, "Cyberpunk RED", "Reputation Level\t4", "Facedown (COOL + Reputation)\t12", "Facedown roll\t12 + 1d10",
               "Level 4 deed: Took down a cyberpsycho (defining)", "Level 3 deed: Paid a debt on time"],
    "forbid": BASE_FORBID + ["Reputation Level\t3", "Paid a debt on time (defining)", "Facedown (COOL + Reputation)\t11"],
    "notes": f"SD-38 E5.MC for E5.5 (epic-breakdown.md E5.5: '... a new positive deed replaces the old only if higher ...'). Builds the {SOLO}, loads it, Improve, a Level 4 deed, Spend IP, Close and Load again; Improve again with a Level 3 deed, Spend IP, Close and Load again from the store: both events are saved, the Level 4 deed stays the defining one (p. 193: a new deed replaces the old only if its Level is higher), Reputation Level 4, Facedown COOL 8 + 4 = 12. A lower deed that replaced the higher (Level 3, Facedown 11, the Level 3 event marked defining) is red. Before E5.MC the row does not exist.",
})

n = "CPR MC Refusals Solo"
rows.append({
    "id": "improve-cyberpunk-skip-refused",
    "screen": "sheet",
    "steps": build(n) + open_improve() + typed("IP entered", "500")
    + [{"op": "select", "target": "— choose a skill —", "text": "Handgun"}, {"op": "wait", "ms": 1000}]
    + typed("Skill Level to buy", "8")
    + [{"op": "click", "target": "Add skill level"}, {"op": "wait", "ms": 2000}],
    "marker": n,
    "expect": [n, "Cyberpunk RED", "Handgun (REF)\t6\t13", "Improvement Points (current / total)\t0 / 0",
               "Handgun: Typical Skill Improvement: Level 6 to 8 skips a Level; buy Level 7 first (p. 411: you can't skip Levels/Ranks)"],
    "forbid": BASE_FORBID + ["Handgun (REF)\t8"],
    "notes": f"SD-38 E5.MC for E5.1's refusal path through the app (epic-breakdown.md E5.1: '... refuses skipped levels and overspend'). Builds the {SOLO} (Handgun Level 6), loads it, Improve, enters 500 IP and asks for Handgun Level 8: the engine refuses the skipped Level (p. 411) and the plan cannot be spent; the sheet behind the dialog still shows Handgun 6 / 13 and IP 0 / 0. The overspend refusal through the app is exercised by E5.MC's cost plant: with every IP price x 1.5, Handgun Level 7 costs 210 > 200 IP, the spend is refused and improve-cyberpunk-skill goes red (Handgun stays 6 / 13, IP 0 / 0). Before E5.MC the row does not exist.",
})

print(json.dumps(rows, ensure_ascii=False, indent=1))
