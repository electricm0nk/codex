#!/usr/bin/env python3
"""Generate .pcg fixtures for the equipment `other_bonus_shape` SKILL
sub-population (SD-33 remediation wave 2, equipment-other-bonus-shape
lane).

Reuses AT-33-E5-001/002's exact proven mechanism (Level-1 Human Fighter,
fixed base ability scores STR16 DEX14 CON14 INT10 WIS10 CHA8, one item
equipped per character) with one change: the item is equipped at the
`Carried` EquipSet location (the same location PCGen's own stock
`characters/Everything.pcg` uses for a Backpack -- a slotless `Goods.*`
item with no `LOCATION`/body-slot token, matching every SKILL-shape
item's own corpus record: no LOCATION token on the ones inspected this
cycle), not `Belt`/`Headband`.

Reads the item's granted skill bonus straight off PCGen's own
`SKILL.<name>.MISC` export token (`SkillToken.SKILL_MISC`, real PCGen
source: `modifier(aSkill,pc) - getStatMod(aSkill,pc)` -- total skill
modifier minus the ability-score component), confirmed empirically this
cycle against `Climber's Kit` (`+2` circumstance -> `SKILL.Climb.MISC=2`,
`SKILL.Climb.TOTAL=5` on a base STR16 fighter, `ABMOD=3`, `RANK=0`) --
this isolates the item's own contribution without needing a
skill-to-key-ability lookup table (a hand-rolled PF1 rules constant) or a
baseline-character diff.

**Real, execution-confirmed equip-location hazard, fixed this cycle:**
`EQUIPSET:Carried` (the location AT-33-E5-002's own `characters/
Everything.pcg` precedent used for a Backpack) does NOT trigger a
slotless item's `BONUS:` tokens -- confirmed empirically:
`EQUIPSET:Carried` on `Climber's Kit` produced `SKILL.Climb.MISC=0` (no
bonus applied); switching to `EQUIPSET:Equipped` (the correct location
for a `TYPE:Goods.*` item that carries no `LOCATION`/body-slot token at
all) produced the real `SKILL.Climb.MISC=2` matching the corpus record.
`Carried` genuinely means "possessed but inert" for `BONUS:` purposes;
`Equipped` is PCGen's actual "this item's tokens are live" location for a
slotless item, distinct from a body-slot name like `Belt`/`Headband`.

Usage: equipment-remainder-generate-skill-pcgs.py <census.json> <out_dir> <manifest_out.json>
"""
import json, os, sys

CAMPAIGN_NAME = {
    'core_rulebook': 'Core Rulebook',
    'ultimate_equipment': 'Ultimate Equipment',
    'ultimate_psionics': 'Ultimate Psionics',
    'inner_sea_gods': 'Inner Sea Gods',
    'advanced_race_guide': 'Advanced Race Guide',
    'advanced_players_guide': "Advanced Player's Guide",
    'book_of_the_damned_volume_2': 'Lords of Chaos - Book of the Damned, Volume 2',
    'advanced_class_guide': 'Advanced Class Guide',
    'ultimate_wilderness': 'Ultimate Wilderness',
    'occult_adventures': 'Occult Adventures',
}

PCG_TEMPLATE = """PCGVERSION:2.0

# System Information
CAMPAIGN:Core Rulebook
{extra_campaign}VERSION:6.09.08.RC1
GAMEMODE:Pathfinder_RPG
CHARACTERTYPE:PC
PURCHASEPOINTS:N
AUTOSPELLS:Y

# Character Bio
CHARACTERNAME:{charname}
PLAYERNAME:sd-33-e5-remainder

# Character Attributes
STAT:STR|SCORE:16
STAT:DEX|SCORE:14
STAT:CON|SCORE:14
STAT:INT|SCORE:10
STAT:WIS|SCORE:10
STAT:CHA|SCORE:8
ALIGN:LN
RACE:Human

# Character Class(es)
CLASS:Fighter|LEVEL:1|SKILLPOOL:0

# Character Experience
EXPERIENCE:0
EXPERIENCETABLE:Medium

# Character Equipment
MONEY:0
EQUIPNAME:{itemname}|OUTPUTORDER:1|COST:0|WT:0|QUANTITY:1.0
EQUIPSET:Default Set|ID:0.1|USETEMPMODS:Y
EQUIPSET:Equipped|ID:0.1.01|VALUE:{itemname}|QUANTITY:1.0|USETEMPMODS:Y
"""

FTL_TEMPLATE = """<#ftl encoding="UTF-8" strip_whitespace=true >
SKILL.NAME=${{pcstring('SKILL.{skill}.NAME')}}
SKILL.TOTAL=${{pcstring('SKILL.{skill}.TOTAL')}}
SKILL.MISC=${{pcstring('SKILL.{skill}.MISC')}}
"""


def slug(unit_id):
    return unit_id.split(':')[-1]


def main():
    census_path, out_dir, manifest_out = sys.argv[1], sys.argv[2], sys.argv[3]
    census = json.load(open(census_path))
    items = [x for x in census['items'] if ',' not in x['skill'] and x['skill'] != 'ALL']

    os.makedirs(out_dir, exist_ok=True)

    manifest = []
    for item in items:
        s = slug(item['unit_id'])
        book = item['book']
        extra = ''
        if book != 'core_rulebook':
            extra = f"CAMPAIGN:{CAMPAIGN_NAME[book]}\n"
        pcg_text = PCG_TEMPLATE.format(
            extra_campaign=extra,
            charname=f"E5 Rem {s}",
            itemname=item['key'],
        )
        pcg_path = os.path.join(out_dir, f"{s}.pcg")
        with open(pcg_path, 'w') as f:
            f.write(pcg_text)
        ftl_text = FTL_TEMPLATE.format(skill=item['skill'])
        ftl_path = os.path.join(out_dir, f"{s}.txt.ftl")
        with open(ftl_path, 'w') as f:
            f.write(ftl_text)
        manifest.append({
            'unit_id': item['unit_id'],
            'slug': s,
            'book': book,
            'key': item['key'],
            'pcg_path': pcg_path,
            'ftl_path': ftl_path,
            'skill': item['skill'],
            'expected_bonus': item['bonus'],
        })
    with open(manifest_out, 'w') as f:
        json.dump(manifest, f, indent=2)
    print(f"wrote {len(manifest)} .pcg + .ftl files to {out_dir}, manifest {manifest_out}")


if __name__ == '__main__':
    main()
