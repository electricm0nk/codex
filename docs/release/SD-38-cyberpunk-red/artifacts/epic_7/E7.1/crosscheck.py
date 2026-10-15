#!/usr/bin/env python3
"""E7.1 second implementation: re-derive EVERY number on the three Cyberpunk RED starters' loaded
sheets from the book's formulas and the corpus records, with no engine or TSX code.

    python3 -I crosscheck.py <dump dir> <corpus dir>

<dump dir> is what `CPR_STARTER_DUMP_OUT=<dir> cargo test --locked -j 8 --bins
starter_seed_tests::restore_writes_the_three -- --test-threads=8` (apps/desktop/src-tauri) writes:
one directory per starter holding its saved files (`cpr_*.txt`, `envelope.txt`) and `loaded.json`,
the `load_saved_character` response the sheet renders. The script reads the BUILD from the request
side (template row / seeded rolls / typed allocation below, the saved skill levels, kit and
cyberware) and recomputes each `cpr.` row of `loaded.json`; a row it cannot classify is a failure.
Exit 1 on any mismatch, on a missing or unclassified row.
"""
import json
import math
import re
import sys

dump, corpus = sys.argv[1], sys.argv[2]
STATS = ["int", "ref", "dex", "tech", "cool", "will", "luck", "move", "body", "emp"]
LABELS = [s.upper() for s in STATS]


def records(kind):
    return json.load(open(f"{corpus}/{kind}.json", encoding="utf-8"))


def by_name(kind):
    return {r["name"]: r for r in records(kind)}


weapons, armor, cyberware, decks, programs, ammo = (
    by_name(k) for k in ["weapon", "armor", "cyberware", "cyberdeck", "program", "ammo"]
)
skills_catalog = records("skill")
range_rows = records("range_dv")
wounds = by_name("wound_state")
kits = {k["role"]: k for k in records("starting_kit")}
interface = next(r for r in records("role_ability") if r["role"] == "Netrunner")
medicine = next(r for r in records("role_ability") if r["role"] == "Medtech")


# --- the build side: STATs as the request gives them -------------------------------------------
M = (1 << 64) - 1


class Roller:
    """SplitMix64 over a 32-bit seed, faces by rejection (the creation roller's published recipe)."""

    def __init__(self, seed):
        self.state = seed & M

    def next(self):
        self.state = (self.state + 0x9E3779B97F4A7C15) & M
        z = self.state
        z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & M
        z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & M
        return z ^ (z >> 31)

    def roll(self, sides):
        zone = M - (M % sides)
        while True:
            draw = self.next()
            if draw < zone:
                return draw % sides + 1


def template(role):
    rows = {r["roll"]: [r[label] for label in LABELS] for r in records("streetrat_template") if r["role"] == role}
    return rows


solo_rows, medtech_rows = template("Solo"), template("Medtech")
roller = Roller(2045)
medtech_dice = [roller.roll(10) for _ in range(10)]
cp_points = next(r["stat_points"] for r in records("complete_package_points") if r["default"])
netrunner_stats = [7, 6, 6, 7, 6, 5, 6, 6, 6, 7]
assert sum(netrunner_stats) == cp_points == 62, (sum(netrunner_stats), cp_points)
BUILD_STATS = {
    "00000000-0000-0000-0000-000000000004": solo_rows[3],
    "00000000-0000-0000-0000-000000000005": netrunner_stats,
    "00000000-0000-0000-0000-000000000006": [medtech_rows[medtech_dice[i]][i] for i in range(10)],
}


def slug(text):
    return re.sub(r"_+", "_", re.sub(r"[^a-z0-9]+", "_", text.lower())).strip("_")


def parse_choices(path):
    out = {"skills": {}, "armor": [], "shield": None, "cyberware": [], "weapons": [], "ammo": [], "programs": [], "deck": None}
    for line in open(path, encoding="utf-8").read().splitlines():
        key, _, value = line.partition(" = ")
        if key == "role":
            out["role"] = value
        elif key == "role.rank":
            out["rank"] = int(value)
        elif key.startswith("skill."):
            out["skills"][key[len("skill."):]] = int(value)
        elif key.startswith("armor.") and key != "armor.shield":
            out["armor"].append((key[len("armor."):], value))
        elif key == "armor.shield":
            out["shield"] = value
        elif key == "cyberware":
            out["cyberware"].append(value.split(" @ ")[0])
        elif key == "weapon":
            out["weapons"].append(value)
        elif key == "cash":
            out["cash"] = int(value)
        elif key == "ammo":
            count, _, name = value.partition(" ")
            out["ammo"].append((name, int(count)))
        elif key == "deck":
            out["deck"] = value
        elif key == "deck.program":
            out["programs"].append(value)
    return out


def kit_cost(path):
    """Complete Package spend from the saved kit: every bought item at its printed price."""
    total = 0
    for line in open(path, encoding="utf-8").read().splitlines():
        _, _, rest = line.partition(" = ")
        section, quantity, name = (p.strip() for p in rest.split("|"))
        quantity = int(quantity)
        assert section == "bought", line
        if name in decks:
            total += decks[name]["cost_eb"] * quantity
        elif name in programs:
            total += programs[name]["cost_eb"] * quantity
        elif name in cyberware:
            total += cyberware[name]["cost_eb"] * quantity
        elif name in weapons:
            total += weapons[name]["cost_eb"] * quantity
        elif name.endswith(" Ammunition"):
            kind = name.split(" ")[0] + " Ammunition"  # "Basic H Pistol Ammunition" -> Basic Ammunition
            total += ammo[kind]["cost_eb"] * quantity // 10  # priced per 10 rounds (p. 345)
        elif m := re.fullmatch(r"(.+) (Body|Head) Armor \(SP\d+\)", name):
            total += armor[m.group(1)]["cost_eb"] * quantity
        else:
            raise SystemExit(f"unpriced kit item {name}")
    return total


bad = 0
summary = []
for character_id, stats in BUILD_STATS.items():
    folder = f"{dump}/{character_id}"
    loaded = json.load(open(f"{folder}/loaded.json", encoding="utf-8"))
    rows = {e["id"]: e["value"] for e in loaded["explanations"]}
    choices = parse_choices(f"{folder}/cpr_sheet_choices.txt")
    s = dict(zip(STATS, stats))
    want = {}

    # STATs (p. 72-79) and derived (p. 79-80)
    for stat in STATS:
        want[f"cpr.stat.{stat}"] = s[stat]
    hl = sum(cyberware[n]["humanity_loss_preset"] for n in choices["cyberware"])
    hp = 10 + 5 * math.ceil((s["body"] + s["will"]) / 2)
    humanity = 10 * s["emp"] - hl
    emp_now = max(0, humanity // 10)
    want.update({
        "cpr.derived.hit_points": hp,
        "cpr.derived.seriously_wounded": math.ceil(hp / 2),
        "cpr.derived.death_save": s["body"],
        "cpr.derived.humanity_max": 10 * s["emp"],
        "cpr.derived.humanity_current": humanity,
        "cpr.derived.emp_current": emp_now,
        "cpr.derived.move": s["move"],
        "cpr.derived.luck_pool": s["luck"],
    })

    # Armour (p. 184, p. 351): highest SP per location, the most punishing penalty once.
    worn = choices["armor"]
    penalty = min([armor[name]["armor_penalty_ref_dex_move"] for _, name in worn] + [0])
    for location in ["head", "body"]:
        here = [armor[name] for loc, name in worn if loc == location]
        want[f"cpr.armor.sp_{location}"] = max([a["sp"] for a in here] + [0])
        want[f"cpr.armor.penalty_{location}"] = min([a["armor_penalty_ref_dex_move"] for a in here] + [0])
    want["cpr.armor.penalty"] = penalty
    current = dict(s)
    for stat in ["ref", "dex", "move"]:
        current[stat] = max(0, s[stat] + penalty)
        want[f"cpr.armor.{stat}"] = current[stat]
    current["emp"] = emp_now
    if choices["shield"]:
        want["cpr.armor.shield_hp"] = armor[choices["shield"]]["hp"]
        want["cpr.armor.shield_penalty"] = armor[choices["shield"]]["armor_penalty_ref_dex_move"]

    # Cyberware (p. 111-118, p. 230, p. 359-368)
    pieces = choices["cyberware"]
    want["cpr.cyberware.humanity_loss"] = hl
    for i, name in enumerate(pieces):
        want[f"cpr.cyberware.piece.{i}.humanity_loss"] = cyberware[name]["humanity_loss_preset"]
    with_loss = sum(1 for n in pieces if cyberware[n]["humanity_loss_preset"] > 0)
    want["cpr.cyberware.humanity_cap"] = 10 * s["emp"] - 2 * with_loss  # no borgware on any seed
    want["cpr.cyberware.body"] = s["body"]  # no Grafted Muscle and Bone Lace / Linear Frame
    want["cpr.cyberware.sp_body"] = want["cpr.cyberware.sp_head"] = 0  # no Skin Weave / Subdermal Armor
    want["cpr.cyberware.initiative_bonus"] = 0  # no Kerenzikov
    pools = {"fashion": "fashionware", "internal": "internal", "external": "external"}
    for pool, row in pools.items():
        capacity = next(c["pool_capacity"] for c in cyberware.values() if c["slot_pool"] == pool and c["pool_capacity"])
        want[f"cpr.cyberware.slots.{row}.capacity"] = capacity
        want[f"cpr.cyberware.slots.{row}.used"] = sum(cyberware[n]["slots_used"] for n in pieces if cyberware[n]["slot_pool"] == pool)
    if "Neural Link" in pieces:
        want["cpr.cyberware.slots.neural_link.capacity"] = cyberware["Neural Link"]["slots_provided"]
        want["cpr.cyberware.slots.neural_link.used"] = sum(cyberware[n]["slots_used"] for n in pieces if cyberware[n]["slot_pool"] == "neural_link")
    if "Cybereye" in pieces:
        want["cpr.cyberware.slots.cybereye_1.capacity"] = cyberware["Cybereye"]["slots_provided"]
        want["cpr.cyberware.slots.cybereye_1.used"] = sum(cyberware[n]["slots_used"] for n in pieces if cyberware[n]["slot_pool"] == "cybereye")

    # Wound track (p. 186-187)
    want.update({
        "cpr.wound.lightly_wounded": hp,
        "cpr.wound.seriously_wounded": math.ceil(hp / 2),
        "cpr.wound.mortally_wounded": 1,
        "cpr.wound.lightly_wounded_dv": wounds["Lightly Wounded"]["stabilization_dv"],
        "cpr.wound.seriously_wounded_dv": wounds["Seriously Wounded"]["stabilization_dv"],
        "cpr.wound.mortally_wounded_dv": wounds["Mortally Wounded"]["stabilization_dv"],
        "cpr.wound.death_save": s["body"],
        "cpr.wound.base_death_save_penalty": 0,
    })

    # Skills (p. 80, p. 128): every skill at its saved level (0 if none); base = current STAT + level.
    held = choices["skills"]
    for skill in skills_catalog:
        stat = skill["stat"].lower()
        specialties = [k for k in held if k.startswith(skill["name"] + " (")]
        keys = specialties or [skill["name"]]
        for key in keys:
            level = held.get(key, 0)
            rest = slug(skill["name"]) + ("." + slug(key[len(skill["name"]) + 2:-1]) if key != skill["name"] else "")
            want[f"cpr.skill_level.{rest}"] = level
            want[f"cpr.skill.{rest}"] = current[stat] + level
    skill_base = {name: want[f"cpr.skill.{slug(name)}"] for name in ["Handgun", "Shoulder Arms", "Melee Weapon", "Autofire"]}

    # Role (p. 142-151)
    role, rank = choices["role"], choices["rank"]
    want[f"cpr.role.{role.lower()}.rank"] = rank
    if role == "Solo":
        want["cpr.role.solo.combat_awareness_points"] = rank  # p. 147: the pool equals the Rank
    if role == "Netrunner":
        actions = next(t["net_actions"] for t in interface["net_actions_per_turn"] if t["rank_min"] <= rank <= t["rank_max"])
        want["cpr.role.netrunner.net_actions"] = want["cpr.netrunning.net_actions"] = actions
    if role == "Medtech":
        want["cpr.role.medtech.medicine_specialty_points"] = rank * medicine["points_per_rank"]
    if choices["deck"]:
        want["cpr.netrunning.deck_slots_available"] = decks[choices["deck"]]["slots"]
        want["cpr.netrunning.deck_slots_used"] = sum(programs[p]["slots"] for p in choices["programs"])

    # Reputation (p. 193-194), Improvement Points (p. 410), cash and ammunition as saved.
    want["cpr.reputation.level"] = 0
    want["cpr.reputation.facedown"] = s["cool"] + 0
    want["cpr.ip.current"] = want["cpr.ip.total"] = 0
    kit_file = f"{folder}/cpr_kit.txt"
    if role == "Netrunner":
        want["cpr.cash"] = 2550 - kit_cost(kit_file)  # p. 105: 2,550eb, the purchases at printed prices
    else:
        want["cpr.cash"] = kits[role]["cash_eb"]
    for i, (_, count) in enumerate(choices["ammo"]):
        want[f"cpr.ammo.{i}"] = count

    # Weapons (p. 93-95, p. 172-174, p. 341-342) and range bands (p. 174)
    for band_i, band in enumerate(range_rows[0]["bands"]):
        want[f"cpr.range_band.{band_i}.max"] = band["max_m_yds"]
    for i, name in enumerate(choices["weapons"]):
        w = weapons[name]
        want[f"cpr.weapon.{i}.rof"] = w["rof"]
        if isinstance(w["hands"], int):
            want[f"cpr.weapon.{i}.hands"] = w["hands"]
        if w["standard_magazine"] is not None:
            want[f"cpr.weapon.{i}.magazine"] = w["standard_magazine"]
        skill = w["skill"] or ("Melee Weapon" if w["weapon_class"] == "melee" else None)
        want[f"cpr.weapon.{i}.skill_base"] = skill_base[skill]
        for mode, prefix in [("single_shot", "range_dv"), ("autofire", "autofire_dv")]:
            row = next((r for r in range_rows if r["mode"] == mode and name in r["weapon_types"]), None)
            if row is None:
                continue
            for b, band in enumerate(row["bands"]):
                if band["dv"] is not None:
                    want[f"cpr.weapon.{i}.{prefix}.{b}"] = band["dv"]

    missing = sorted(set(want) - set(rows))
    unclassified = sorted(set(rows) - set(want))
    wrong = sorted(k for k in set(want) & set(rows) if want[k] != rows[k])
    for k in missing:
        print(f"{character_id} MISSING {k} (want {want[k]})")
    for k in unclassified:
        print(f"{character_id} UNCLASSIFIED {k} = {rows[k]}")
    for k in wrong:
        print(f"{character_id} WRONG {k}: sheet {rows[k]} != derived {want[k]}")
    bad += len(missing) + len(unclassified) + len(wrong)
    summary.append(f"{loaded['summary']['displayLabel']}: {len(rows)} numeric rows, {len(set(want) & set(rows)) - len(wrong)} re-derived equal")

print(f"medtech seed 2045 rolls: {medtech_dice}")
for line in summary:
    print(line)
print("CROSSCHECK_OK" if bad == 0 else f"CROSSCHECK_FAILED {bad}")
sys.exit(1 if bad else 0)
