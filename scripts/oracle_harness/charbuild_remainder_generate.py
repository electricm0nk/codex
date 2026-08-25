#!/usr/bin/env python3
"""SD-33 AT-33-E5-remainder-charbuild -- generates the .pcg fixtures for the
81-unit full-character-build remainder: one L20 .pcg per source class (13
classes, amortising many `class_feature` units per JVM start, per this
cycle's own next-cycle-plan mandate), one L1 .pcg per race (36 races).

Every ability score is fixed at 14 (modifier +2) uniformly across every
class build, so every ability-modifier-dependent formula (Master Hunter DC,
Master Strike DC, Wild Empathy, Monk AC Bonus, ...) uses the SAME, easily
re-derived modifier on both the engine side (this cycle's Rust probe) and
the oracle side (PCGen's own export) -- no per-class ability tuning, no
guesswork.
"""
import os
import sys

BASE = {'STR': 14, 'DEX': 14, 'CON': 14, 'INT': 14, 'WIS': 14, 'CHA': 14}

PCG_HEADER = """PCGVERSION:2.0

# System Information
{campaigns}VERSION:6.09.08.RC1
GAMEMODE:Pathfinder_RPG
CHARACTERTYPE:PC
PURCHASEPOINTS:N
AUTOSPELLS:Y

# Character Bio
CHARACTERNAME:{charname}
PLAYERNAME:sd-33-e5-remainder-charbuild

# Character Attributes
STAT:STR|SCORE:{STR}
STAT:DEX|SCORE:{DEX}
STAT:CON|SCORE:{CON}
STAT:INT|SCORE:{INT}
STAT:WIS|SCORE:{WIS}
STAT:CHA|SCORE:{CHA}
ALIGN:{align}
RACE:{race}

# Character Class(es)
CLASS:{pcgen_class}|LEVEL:{level}|SKILLPOOL:0
{extra_class_lines}
# Character Experience
EXPERIENCE:0
EXPERIENCETABLE:Medium
{extra_lines}"""

# Per-class campaign closures. The first 7 (single-book) closures are
# proven live this cycle (real `./gradlew`-free BatchExporter runs, real
# SA output, real magnitude match against this cycle's Rust probe). The
# other 6 (Bloodrager/Slayer/Investigator/Inquisitor/Ninja/Samurai) failed
# with `Attempt to fetch AbilityCategory: Class... but it does not exist`
# under a single-book closure -- confirmed live this cycle -- because their
# own sourcebook `.pcc` declares a HARD `PRECAMPAIGN:1,INCLUDES=...` chain
# (not an optional cross-book addition): `advanced_class_guide/
# _advanced_class_guide.pcc` requires Advanced Player's Guide + Ultimate
# Combat + Ultimate Equipment + Ultimate Magic; `ultimate_combat/*.pcc`
# requires Advanced Player's Guide (which itself requires only Core
# Rulebook). Each closure below is the real minimal chain read directly
# from those `.pcc` files this cycle, not a guessed superset.
ACG_CLOSURE = [
    "Core Rulebook", "Advanced Player's Guide", "Advanced Class Guide",
    "Ultimate Combat", "Ultimate Equipment", "Ultimate Magic",
]
APG_CLOSURE = ["Core Rulebook", "Advanced Player's Guide"]
UC_CLOSURE = ["Core Rulebook", "Advanced Player's Guide", "Ultimate Combat"]

# (unit-group name, PCGen CLASS:, campaign(s), level, extra .pcg lines list)
CLASS_BUILDS = [
    ("bloodrager", "Bloodrager", ACG_CLOSURE, 20, []),
    ("slayer", "Slayer", ACG_CLOSURE, 20,
     ["ABILITY:Slayer Talent|TYPE:NORMAL|CATEGORY:Slayer Talent|KEY:Slayer Talent ~ Foil Scrutiny"]),
    ("paladin", "Paladin", ["Core Rulebook"], 20, []),
    ("barbarian", "Barbarian", ["Core Rulebook"], 20,
     ["ABILITY:Rage Power|TYPE:NORMAL|CATEGORY:Rage Power|KEY:Rage Power ~ Superstition"]),
    ("ranger", "Ranger", ["Core Rulebook"], 20, []),
    ("rogue", "Rogue", ["Core Rulebook"], 20,
     ["ABILITY:Rogue Talent|TYPE:NORMAL|CATEGORY:Rogue Talent|KEY:Rogue Talent ~ Resiliency"]),
    ("ninja", "Ninja", UC_CLOSURE, 20, []),
    ("samurai", "Samurai", UC_CLOSURE, 20, []),
    ("investigator", "Investigator", ACG_CLOSURE, 20, []),
    ("inquisitor", "Inquisitor", APG_CLOSURE, 20, []),
    ("druid", "Druid", ["Core Rulebook"], 20, []),
    ("monk", "Monk", ["Core Rulebook"], 20, []),
    # Unchained Rogue is not a separate CLASS: in PCGen's own data -- it is
    # an alternate-class-features ABILITY (CATEGORY:Class, KEY:"Rogue ~
    # Unchained Class") selected on top of the base Rogue class
    # (`pu_abilities_class.lst:116`, confirmed by direct grep this cycle).
    ("unchained_rogue", "Rogue", ["Core Rulebook", "Pathfinder Unchained"], 20,
     ["ABILITY:Class|TYPE:NORMAL|CATEGORY:Class|KEY:Rogue ~ Unchained Class"]),
]

RACE_CAMPAIGNS = {
    'core_rulebook': ['Core Rulebook'],
    'advanced_race_guide': ['Core Rulebook', 'Advanced Race Guide'],
    'bestiary_4': ['Core Rulebook', 'Bestiary 4'],
    'bestiary_5': ['Core Rulebook', 'Bestiary 5'],
    'bestiary_6': ['Core Rulebook', 'Bestiary 6'],
}


def write_pcg(path, charname, campaigns, race, pcgen_class, level, extra_lines):
    camp = ''.join(f"CAMPAIGN:{c}\n" for c in campaigns)
    text = PCG_HEADER.format(
        campaigns=camp,
        charname=charname,
        align='LN',
        race=race,
        pcgen_class=pcgen_class,
        level=level,
        extra_class_lines='',
        extra_lines='\n'.join(extra_lines) + ('\n' if extra_lines else ''),
        **BASE,
    )
    with open(path, 'w') as f:
        f.write(text)


def main():
    out_dir = sys.argv[1]
    races_arg = sys.argv[2] if len(sys.argv) > 2 else None
    os.makedirs(out_dir, exist_ok=True)
    manifest = []

    for group, pcgen_class, campaigns, level, extra in CLASS_BUILDS:
        pcg_path = os.path.join(out_dir, f"class-{group}.pcg")
        write_pcg(pcg_path, f"E5 Charbuild {group}", campaigns, "Human", pcgen_class, level, extra)
        manifest.append({'kind': 'class', 'group': group, 'pcg_path': pcg_path})

    if races_arg:
        import json
        races = json.load(open(races_arg))
        for unit_id, race_key in races:
            book = unit_id.split(':')[0]
            campaigns = RACE_CAMPAIGNS.get(book, ['Core Rulebook'])
            slug = unit_id.split(':')[-1]
            pcg_path = os.path.join(out_dir, f"race-{slug}.pcg")
            write_pcg(pcg_path, f"E5 Charbuild race {slug}", campaigns, race_key, "Fighter", 1, [])
            manifest.append({'kind': 'race', 'unit_id': unit_id, 'race_key': race_key, 'pcg_path': pcg_path})

    manifest_path = os.path.join(out_dir, 'manifest.json')
    import json as _json
    with open(manifest_path, 'w') as f:
        _json.dump(manifest, f, indent=2)
    print(f"wrote {len(manifest)} .pcg files to {out_dir}, manifest {manifest_path}")


if __name__ == '__main__':
    main()
