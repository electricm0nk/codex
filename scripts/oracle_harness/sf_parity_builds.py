#!/usr/bin/env python3
"""SD-37 E7.1: the Starfinder oracle-parity roster as PCGen characters (.pcg).

The roster is the four SD-37 seeds, one level-1 build of each of the 10 Starfinder player classes
(the Mechanic 1 is E5.4's parity build, `docs/release/SD-37-starfinder-1e/artifacts/epic_5/E5.4-mechanic-1-build.md`),
and the Mechanic 1's drone. The engine side of every build is the `CharacterInput` the desktop
test `sf_oracle_parity` renders (`apps/desktop/src-tauri/src/sf_oracle_parity.rs`): the seeds are
`sf_adapter::tests::seeds()`, the Mechanic 1 is `sf_drone_print::tests::mechanic_1_input()`, the
other nine level-1 builds are written there. Each .pcg below holds exactly those picks, by the
oracle's own names, and nothing else, so the two sides render the same character. A drift between
the two is caught by the parity itself: the oracle exports the six ability scores, the class, the
level and every skill's ranks, and the test compares each against the engine build.

PCGen applies automatic ability adjustments itself (a race's fixed BONUS:STAT, the lashunta damaya
subrace, the theme's +1). Choice adjustments (the human's +2 to any one score) and the level-5
increases are folded into the base score, as E3.3's `make_seed_pcg.py` did, so the exported final
scores equal the engine build's saved final scores.

Usage: sf_parity_builds.py <out-dir>   (writes one <build>.pcg per roster row; prints each path)
"""
import sys
from pathlib import Path

CORE = "Starfinder RPG Core Supplements"
COM = "Starfinder Character Operations Manual"

STATS = ("STR", "DEX", "CON", "INT", "WIS", "CHA")


def theme(name, desc=""):
    return f"ABILITY:Theme Selection|TYPE:NORMAL|CATEGORY:Theme|KEY:{name}|TYPE:Theme.Theme Selection"


def feat(key, pool="FEAT", typ="General"):
    return f"ABILITY:{pool}|TYPE:NORMAL|CATEGORY:FEAT|KEY:{key}|TYPE:{typ}"


def soldier_key(ability):
    return [f"ABILITY:Soldier Key Ability|TYPE:NORMAL|CATEGORY:Internal|KEY:Soldier Key Ability ~ {ability}|TYPE:Soldier Key Ability"]


def biohacker_key(ability):
    return [f"ABILITY:Biohacker Key Ability|TYPE:NORMAL|CATEGORY:Internal|KEY:Biohacker Key Ability ~ {ability}|TYPE:Biohacker Key Ability"]


# (name, key, quantity, equip slot) -- slot "Armor" (worn), "Primary Hand" (wielded: the engine's
# `EquippedActive` weapon) or "Carried" (the engine's `SelectedInactive`).
def eq(name, key, slot="Carried", qty=1):
    return (name, key, qty, slot)


SECOND_SKIN = eq("Second Skin", "Second Skin", "Armor")
LASER_PISTOL = ("Laser pistol, azimuth", "Laser pistol (azimuth)")
BATON = ("Baton, tactical", "Baton (tactical)")

# One row per build. stats = the .pcg base scores (final minus PCGen's automatic adjustments).
# skills = (oracle skill name, ranks). spells = (oracle spell name, class, spell level).
ROSTER = [
    # ---- the four seeds (artifacts/epic_0/seed-builds.md; engine: sf_adapter::tests::seeds()) ----
    dict(build="SF-Soldier-3", race="Human", size="Medium", cls="Soldier", level=3,
         stats=(15, 14, 12, 11, 10, 10),  # final 16 14 12 11 10 10: +1 Str Mercenary (auto), +2 Str human (folded)
         abilities=[theme("Mercenary"), *soldier_key("STR"),
                    feat("Weapon Focus", typ="Combat.WeaponFocus"), feat("Quick Draw", typ="Combat"),
                    feat("Deadly Aim", typ="Combat"), feat("Coordinated Shot", typ="Combat")],
         skills=[("Athletics", 3), ("Intimidate", 3), ("Medicine", 3), ("Piloting", 3), ("Survival", 3)],
         equipment=[eq("Defiance Series, Squad", "Defiance Series (Squad)", "Armor"),
                    eq(*("Laser rifle, azimuth", "Laser rifle (azimuth)"), "Primary Hand"),
                    eq(*BATON), eq("Battery", "Battery", qty=2), eq("Serum of Healing Mk 1", "Serum of Healing Mk 1", qty=2)]),
    dict(build="SF-Mystic-5", race="Lashunta", size="Medium", cls="Mystic", level=5,
         stats=(10, 14, 10, 12, 18, 13),  # final 10 14 8 14 19 15: lashunta +2 Cha, damaya +2 Int -2 Con, priest +1 Wis; level-5 increases folded
         abilities=[theme("Priest"),
                    "ABILITY:Lashunta Subrace Selection|TYPE:NORMAL|CATEGORY:Racial Trait|KEY:Lashunta Subrace ~ Damaya|TYPE:Lashunta Subrace Selection",
                    "ABILITY:+2 Racial Bonus to Skill|TYPE:NORMAL|CATEGORY:Racial Trait|KEY:+2 Racial Bonus to Skill|APPLIEDTO:Display ~ Diplomacy|TYPE:+2 Racial Bonus to Skill",
                    "ABILITY:+2 Racial Bonus to Skill|TYPE:NORMAL|CATEGORY:Racial Trait|KEY:+2 Racial Bonus to Skill|APPLIEDTO:Display ~ Medicine|TYPE:+2 Racial Bonus to Skill",
                    "ABILITY:Connection Selection|TYPE:NORMAL|CATEGORY:Class Feature|KEY:Empath|TYPE:Connection.Connection Selection",
                    feat("Spell Penetration"), feat("Spell Focus"), feat("Quick Draw", typ="Combat")],
         skills=[("Bluff", 5), ("Culture", 5), ("Diplomacy", 5), ("Life Science", 5), ("Medicine", 5),
                 ("Mysticism", 5), ("Perception", 5), ("Sense Motive", 5)],
         equipment=[eq("Lashunta tempweave, basic", "Lashunta tempweave (basic)", "Armor"),
                    eq(*LASER_PISTOL), eq(*BATON), eq("Battery", "Battery"), eq("Medkit (Basic)", "Medkit (Basic)"),
                    eq("Serum of Healing Mk 1", "Serum of Healing Mk 1", qty=2)],
         spells=[(s, "Mystic", 0) for s in ("Detect Affliction", "Detect Magic", "Ghost Sound", "Grave Words", "Stabilize", "Telepathic Message")]
         + [(s, "Mystic", 1) for s in ("Charm Person", "Command", "Mystic Cure (Level 1)", "Share Language")]
         + [(s, "Mystic", 2) for s in ("Hold Person", "Remove Condition", "Status")]),
    dict(build="SF-Technomancer-5", race="Android", size="Medium", cls="Technomancer", level=5,
         stats=(10, 14, 14, 16, 13, 10),  # final 10 16 14 19 13 8: android +2 Dex +2 Int -2 Cha, scholar +1 Int; level-5 increases folded
         abilities=[theme("Scholar"),
                    "ABILITY:Scholar Theme Chosen Skill|TYPE:NORMAL|CATEGORY:Internal|KEY:Physical Science|TYPE:Scholar Theme Chosen Skill",
                    feat("Spell Penetration"), feat("Spell Focus"), feat("Mobility", typ="Combat"), feat("Quick Draw", typ="Combat")],
         skills=[("Computers", 5), ("Engineering", 5), ("Life Science", 5), ("Mysticism", 5), ("Physical Science", 5),
                 ("Piloting", 5), ("Sleight of Hand", 5), ("Perception", 5)],
         equipment=[eq("D-suit I", "D-suit I", "Armor"), eq(*LASER_PISTOL), eq("Battery", "Battery", qty=2),
                    eq("Medkit (Basic)", "Medkit (Basic)"), eq("Serum of Healing Mk 1", "Serum of Healing Mk 1", qty=2)],
         spells=[(s, "Technomancer", 0) for s in ("Dancing Lights", "Detect Magic", "Energy Ray", "Mending", "Token Spell", "Transfer Charge")]
         + [(s, "Technomancer", 1) for s in ("Detect Tech", "Magic Missile", "Overheat", "Supercharge Weapon")]
         + [(s, "Technomancer", 2) for s in ("Invisibility", "Knock", "Mirror Image")]),
    dict(build="SF-Envoy-3", race="Ysoki", size="Small", cls="Envoy", level=3,
         stats=(10, 11, 12, 10, 10, 17),  # final 8 13 12 12 10 18: ysoki +2 Dex +2 Int -2 Str, icon +1 Cha
         abilities=[theme("Icon"), feat("Mobility", typ="Combat"), feat("Quick Draw", typ="Combat")],
         skills=[("Bluff", 3), ("Computers", 3), ("Culture", 3), ("Diplomacy", 3), ("Engineering", 3),
                 ("Intimidate", 3), ("Perception", 3), ("Sense Motive", 3), ("Stealth", 3)],
         equipment=[eq("Carbon skin, graphite", "Carbon skin (graphite)", "Armor"),
                    eq("Semi-auto pistol, tactical", "Semi-auto pistol (tactical)"), eq(*BATON),
                    eq("Serum of Healing Mk 1", "Serum of Healing Mk 1", qty=2)]),
    # ---- level 1 of each player class (Core Rulebook + Character Operations Manual) ----
    dict(build="SF-Mechanic-1", race="Human", size="Medium", cls="Mechanic", level=1,
         stats=(10, 14, 12, 18, 10, 8),  # final 11 14 12 18 10 8: mercenary +1 Str, human +2 Int folded
         abilities=[theme("Mercenary"),
                    "ABILITY:Mechanic AI Selection|TYPE:NORMAL|CATEGORY:Class Feature|KEY:Mechanic Artificial Intelligence ~ Drone|TYPE:Class Feature.Mechanic Class Feature.Mechanic AI Selection"],
         skills=[("Computers", 1), ("Engineering", 1), ("Physical Science", 1), ("Piloting", 1), ("Perception", 1)],
         equipment=[]),
    dict(build="SF-Envoy-1", race="Human", size="Medium", cls="Envoy", level=1,
         stats=(10, 12, 12, 11, 10, 17),  # final 10 12 12 11 10 18: xenoseeker +1 Cha
         abilities=[theme("Xenoseeker")],
         skills=[("Bluff", 1), ("Diplomacy", 1), ("Sense Motive", 1), ("Culture", 1)],
         equipment=[SECOND_SKIN, eq(*LASER_PISTOL, "Primary Hand"), eq(*BATON)]),
    dict(build="SF-Mystic-1", race="Human", size="Medium", cls="Mystic", level=1,
         stats=(10, 12, 12, 10, 17, 11),  # final 10 12 12 10 18 11: priest +1 Wis
         abilities=[theme("Priest")],
         skills=[("Mysticism", 1), ("Medicine", 1), ("Perception", 1)],
         equipment=[SECOND_SKIN, eq(*LASER_PISTOL, "Primary Hand"), eq(*BATON)],
         spells=[("Detect Magic", "Mystic", 0), ("Stabilize", "Mystic", 0), ("Mystic Cure (Level 1)", "Mystic", 1)]),
    dict(build="SF-Operative-1", race="Human", size="Medium", cls="Operative", level=1,
         stats=(10, 17, 12, 12, 11, 10),  # final 10 18 12 12 11 10: ace pilot +1 Dex
         abilities=[theme("Ace Pilot")],
         skills=[("Acrobatics", 1), ("Stealth", 1), ("Piloting", 1), ("Perception", 1)],
         equipment=[SECOND_SKIN, eq(*LASER_PISTOL, "Primary Hand"), eq(*BATON)]),
    dict(build="SF-Solarian-1", race="Human", size="Medium", cls="Solarian", level=1,
         stats=(12, 11, 11, 10, 10, 18),  # final 12 11 12 10 10 18: bounty hunter +1 Con
         abilities=[theme("Bounty Hunter")],
         skills=[("Athletics", 1), ("Mysticism", 1), ("Survival", 1)],
         equipment=[SECOND_SKIN, eq(*LASER_PISTOL, "Primary Hand"), eq(*BATON)]),
    dict(build="SF-Soldier-1", race="Human", size="Medium", cls="Soldier", level=1,
         stats=(12, 17, 12, 10, 11, 10),  # final 12 18 12 10 11 10: outlaw +1 Dex; key ability Dex
         abilities=[theme("Outlaw"), *soldier_key("DEX")],
         skills=[("Athletics", 1), ("Piloting", 1), ("Sleight of Hand", 1)],
         equipment=[SECOND_SKIN, eq(*LASER_PISTOL, "Primary Hand"), eq(*BATON)]),
    dict(build="SF-Technomancer-1", race="Human", size="Medium", cls="Technomancer", level=1,
         stats=(10, 12, 11, 18, 11, 10),  # final 10 12 12 18 11 10: spacefarer +1 Con
         abilities=[theme("Spacefarer")],
         skills=[("Computers", 1), ("Engineering", 1), ("Physical Science", 1)],
         equipment=[SECOND_SKIN, eq(*LASER_PISTOL, "Primary Hand"), eq(*BATON)],
         spells=[("Detect Magic", "Technomancer", 0), ("Energy Ray", "Technomancer", 0), ("Magic Missile", "Technomancer", 1)]),
    dict(build="SF-Biohacker-1", race="Human", size="Medium", cls="Biohacker", level=1,
         stats=(10, 13, 11, 18, 10, 10),  # final 10 14 11 18 10 10: ace pilot +1 Dex; key ability Int
         abilities=[theme("Ace Pilot"), *biohacker_key("INT")],
         skills=[("Medicine", 1), ("Life Science", 1), ("Computers", 1)],
         equipment=[SECOND_SKIN, eq(*LASER_PISTOL, "Primary Hand"), eq(*BATON)],
         campaigns=[CORE, COM]),
    dict(build="SF-Vanguard-1", race="Human", size="Medium", cls="Vanguard", level=1,
         stats=(12, 13, 17, 10, 10, 10),  # final 12 13 18 10 10 10: bounty hunter +1 Con
         abilities=[theme("Bounty Hunter")],
         skills=[("Athletics", 1), ("Acrobatics", 1), ("Intimidate", 1)],
         equipment=[SECOND_SKIN, eq(*LASER_PISTOL, "Primary Hand"), eq(*BATON)],
         campaigns=[CORE, COM]),
    dict(build="SF-Witchwarper-1", race="Human", size="Medium", cls="Witchwarper", level=1,
         stats=(10, 14, 11, 10, 10, 17),  # final 10 14 11 10 10 18: icon +1 Cha
         abilities=[theme("Icon")],
         skills=[("Mysticism", 1), ("Bluff", 1), ("Physical Science", 1)],
         equipment=[SECOND_SKIN, eq(*LASER_PISTOL, "Primary Hand"), eq(*BATON)],
         spells=[("Hazard", "Witchwarper", 0), ("Puncture Veil", "Witchwarper", 1)],
         campaigns=[CORE, COM]),
]


def stem(build):
    return build.lower().replace("-", "_")


def render(b):
    campaigns = "|".join(f"CAMPAIGN:{c}" if i else c for i, c in enumerate(b.get("campaigns", [CORE])))
    stats = "\n".join(f"STAT:{s}|SCORE:{v}" for s, v in zip(STATS, b["stats"]))
    cls, level = b["cls"], b["level"]
    levels = "\n".join(
        f"CLASSABILITIESLEVEL:{cls}={n}|HITPOINTS:1|SKILLSGAINED:0|SKILLSREMAINING:0" for n in range(1, level + 1)
    )
    skills = "\n".join(
        f"SKILL:{name}|CLASSBOUGHT:[CLASS:{cls}|RANKS:{float(r)}|COST:1|CLASSSKILL:Y]" for name, r in b["skills"]
    )
    equip, sets = [], ["EQUIPSET:Default Set|ID:0.1|USETEMPMODS:Y"]
    for i, (name, key, qty, slot) in enumerate(b["equipment"], start=1):
        equip.append(
            f"EQUIPNAME:{name}|OUTPUTORDER:{i}|QUANTITY:{float(qty)}|NOTE:|CUSTOMIZATION:[BASEITEM:{key}|DATA:NAME={name}$KEY={key}]"
        )
        sets.append(f"EQUIPSET:{slot}|ID:0.1.{i:02d}|VALUE:{name}|QUANTITY:{float(qty)}|USETEMPMODS:Y")
    spells = "\n".join(
        f"SPELLNAME:{s}|TIMES:1|CLASS:{c}|BOOK:Known Spells|SPELLLEVEL:{lvl}|SOURCE:[TYPE:CLASS|NAME:{c}]"
        for s, c, lvl in b.get("spells", [])
    )
    return f"""PCGVERSION:2.0

# System Information
CAMPAIGN:{campaigns}
VERSION:6.09.07
ROLLMETHOD:0|EXPRESSION:0
PURCHASEPOINTS:N
CHARACTERTYPE:PC
PREVIEWSHEET:Standard.htm.ftl
POOLPOINTS:0
POOLPOINTSAVAIL:-1
GAMEMODE:Starfinder
AUTOSPELLS:Y
USEHIGHERKNOWN:N
USEHIGHERPREPPED:N
LOADCOMPANIONS:N
USETEMPMODS:Y
SKILLSOUTPUTORDER:0
SKILLFILTER:2
IGNORECOST:N
ALLOWDEBT:N
AUTORESIZEGEAR:Y

# Character Bio
CHARACTERNAME:{stem(b["build"])}
TABNAME:{stem(b["build"])}
PLAYERNAME:SD-37 E7.1

# Character Attributes
{stats}
ALIGN:NG
RACE:{b["race"]}

# Character Class(es)
CLASS:{cls}|LEVEL:{level}|SKILLPOOL:0|SPELLBASE:None|CANCASTPERDAY:
{levels}

# Character Templates
TEMPLATESAPPLIED:[NAME:{b["size"]}]
TEMPLATESAPPLIED:[NAME:Class ~ {cls}]
TEMPLATESAPPLIED:[NAME:First Level Base Class|APPLIEDTO:{cls}]
TEMPLATESAPPLIED:[NAME:{cls}]

# Character Region
REGION:None

# Character Skills
{skills}

# Character Feats
FEATPOOL:0.0

# Character Abilities
{chr(10).join(b["abilities"])}

# Character Equipment
MONEY:0.00
{chr(10).join(equip)}
{chr(10).join(sets)}
CALCEQUIPSET:0.1

# Kits
KIT:Playable Race Defaults

# Spells
{spells}

# Age Set Selections
AGESET:1:0:0:0:0:0:0:0:0:0
"""


# The Mechanic 1's drone (E5.4-mechanic-1-build.md): hover chassis (its kit's base scores, PCGen's
# `STARTPACK:Drone ~ Hover`; a construct has no Constitution, 10 as PCGen's own test drone), skill unit
# Perception, feat Iron Will, mod Camera, initial proficiency small arms. A follower reads its master's
# variables, so it renders only in party mode with the master loaded first (`sf_parity_run.sh`).
DRONE_FILE = "sf_mechanic_1_drone.pcg"
MASTER = "SF-Mechanic-1"
DRONE = """PCGVERSION:2.0
CAMPAIGN:Starfinder RPG Core Supplements
VERSION:6.09.07
ROLLMETHOD:0|EXPRESSION:0
PURCHASEPOINTS:N
CHARACTERTYPE:PC
POOLPOINTS:0
POOLPOINTSAVAIL:-1
GAMEMODE:Starfinder
AUTOSPELLS:Y
LOADCOMPANIONS:N
USETEMPMODS:Y
SKILLSOUTPUTORDER:0
SKILLFILTER:2
IGNORECOST:N
ALLOWDEBT:N
AUTORESIZEGEAR:Y
CHARACTERNAME:sf_mechanic_1_drone
TABNAME:sf_mechanic_1_drone
PLAYERNAME:SD-37 E7.1
STAT:STR|SCORE:6
STAT:DEX|SCORE:16
STAT:CON|SCORE:10
STAT:INT|SCORE:6
STAT:WIS|SCORE:8
STAT:CHA|SCORE:6
ALIGN:NG
RACE:Drone
CLASS:Drone|LEVEL:1|SKILLPOOL:-1|SPELLBASE:None|CANCASTPERDAY:
CLASSABILITIESLEVEL:Drone=1|HITPOINTS:1|SKILLSGAINED:0|SKILLSREMAINING:0
TEMPLATESAPPLIED:[NAME:SIZE ~ Tiny]
TEMPLATESAPPLIED:[NAME:Tiny]
REGION:None
FEATPOOL:0.0
ABILITY:Drone Chassis Selection|TYPE:NORMAL|CATEGORY:Internal|KEY:Drone Chassis ~ Hover|TYPE:Drone Chassis Selection
ABILITY:DRONE Skill Unit|TYPE:NORMAL|CATEGORY:Internal|KEY:DRONE Skill Unit ~ Perception|TYPE:Drone Skill Unit
ABILITY:Drone Feat|TYPE:NORMAL|CATEGORY:Internal|KEY:Drone Feat ~ Iron Will|TYPE:Drone Feat
ABILITY:Drone Mod|TYPE:NORMAL|CATEGORY:Class Feature|KEY:Drone Mod ~ Camera|TYPE:Drone Mod.Class Feature.Extraordinary
ABILITY:Initial Drone Proficiency|TYPE:NORMAL|CATEGORY:Class Feature|KEY:Initial Drone Proficiency ~ Small Arms|TYPE:Initial Drone Proficiency
MONEY:0
EQUIPSET:Default Set|ID:0.1|USETEMPMODS:Y
CALCEQUIPSET:0.1
KIT:Drone ~ Hover
MASTER:sf_mechanic_1|TYPE:Drone|HITDICE:0|FILE:sf_mechanic_1.pcg|ADJUSTMENT:0
AGESET:0:0:0:0:0:0:0:0:0:0
"""


def main():
    out = Path(sys.argv[1])
    out.mkdir(parents=True, exist_ok=True)
    for b in ROSTER:
        path = out / f"{stem(b['build'])}.pcg"
        text = render(b)
        if b["build"] == MASTER:
            text = text.replace("# Kits\n", f"FOLLOWER:sf_mechanic_1_drone|TYPE:Drone|RACE:DRONE|HITDICE:0|FILE:{DRONE_FILE}\n\n# Kits\n")
        path.write_text(text)
        print(path)
    (out / DRONE_FILE).write_text(DRONE)
    print(out / DRONE_FILE)


if __name__ == "__main__":
    main()
