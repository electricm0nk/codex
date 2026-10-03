#!/usr/bin/env python3
"""Write one minimal PCGen .pcg per SD-37 SF seed (E3.3 oracle observation builds).

The .pcg carries only what the HP / Stamina / EAC / KAC / key-ability fields depend on: race (and
the Lashunta subrace pick), class and level (one HITPOINTS:1 row per level -- HD:1), the six base
ability scores, the soldier's key-ability pick and the worn armour. Bases are the seed-builds.md
final scores minus the bonuses PCGen applies automatically (race BONUS:STAT and the Damaya subrace);
the theme and the level-5 increases are not picked, so their points are folded into the base. The
exported <abilities> block is checked against seed-builds.md's final scores by the caller.

Usage: make_seed_pcg.py <out-dir>
"""
import sys
from pathlib import Path

SEEDS = [
    # name, race, size, class, level, bases STR DEX CON INT WIS CHA, extra ability lines, armour (name, key)
    ("sf_seed_soldier_3", "Human", "Medium", "Soldier", 3, (16, 14, 12, 11, 10, 10),
     ["ABILITY:Soldier Key Ability|TYPE:NORMAL|CATEGORY:Internal|KEY:Soldier Key Ability ~ STR|TYPE:Soldier Key Ability",
      "USERPOOL:Soldier Key Ability|POOLPOINTS:0.0"],
     ("Defiance Series, Squad", "Defiance Series (Squad)")),
    ("sf_seed_mystic_5", "Lashunta", "Medium", "Mystic", 5, (10, 14, 10, 12, 19, 13),
     ["ABILITY:Lashunta Subrace Selection|TYPE:NORMAL|CATEGORY:Racial Trait|KEY:Lashunta Subrace ~ Damaya|TYPE:Lashunta Subrace Selection"],
     ("Lashunta tempweave, basic", "Lashunta tempweave (basic)")),
    ("sf_seed_technomancer_5", "Android", "Medium", "Technomancer", 5, (10, 14, 14, 17, 13, 10),
     [],
     ("D-suit I", "D-suit I")),
    ("sf_seed_envoy_3", "Ysoki", "Small", "Envoy", 3, (10, 11, 12, 10, 10, 18),
     [],
     ("Carbon skin, graphite", "Carbon skin (graphite)")),
]

HEADER = """PCGVERSION:2.0

# System Information
CAMPAIGN:Starfinder RPG Core Supplements
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
"""


def render(name, race, size, cls, level, bases, extra, armour):
    stats = "\n".join(f"STAT:{s}|SCORE:{v}" for s, v in zip(("STR", "DEX", "CON", "INT", "WIS", "CHA"), bases))
    levels = "\n".join(
        f"CLASSABILITIESLEVEL:{cls}={n}|HITPOINTS:1|SKILLSGAINED:0|SKILLSREMAINING:0" for n in range(1, level + 1)
    )
    arm_name, arm_key = armour
    return f"""{HEADER}
# Character Bio
CHARACTERNAME:{name}
TABNAME:{name}
PLAYERNAME:SD-37 E3.3

# Character Attributes
{stats}
ALIGN:NG
RACE:{race}

# Character Class(es)
CLASS:{cls}|LEVEL:{level}|SKILLPOOL:0|SPELLBASE:None|CANCASTPERDAY:
{levels}

# Character Templates
TEMPLATESAPPLIED:[NAME:{size}]
TEMPLATESAPPLIED:[NAME:Class ~ {cls}]
TEMPLATESAPPLIED:[NAME:First Level Base Class|APPLIEDTO:{cls}]
TEMPLATESAPPLIED:[NAME:{cls}]

# Character Region
REGION:None

# Character Feats
FEATPOOL:0.0

# Character Abilities
{chr(10).join(extra)}

# Character Equipment
MONEY:0.00
EQUIPNAME:{arm_name}|OUTPUTORDER:1|COST:0|WT:1.0|QUANTITY:1.0|NOTE:|CUSTOMIZATION:[BASEITEM:{arm_key}|DATA:NAME={arm_name}$KEY={arm_key}]
EQUIPSET:Default Set|ID:0.1|USETEMPMODS:Y
EQUIPSET:Armor|ID:0.1.01|VALUE:{arm_name}|QUANTITY:1.0|USETEMPMODS:Y
CALCEQUIPSET:0.1

# Kits
KIT:Playable Race Defaults

# Age Set Selections
AGESET:1:0:0:0:0:0:0:0:0:0
"""


def main():
    out = Path(sys.argv[1])
    out.mkdir(parents=True, exist_ok=True)
    for seed in SEEDS:
        (out / f"{seed[0]}.pcg").write_text(render(*seed))
        print(out / f"{seed[0]}.pcg")


if __name__ == "__main__":
    main()
