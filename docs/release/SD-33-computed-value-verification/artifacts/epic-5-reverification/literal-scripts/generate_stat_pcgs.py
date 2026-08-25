#!/usr/bin/env python3
"""Generate .pcg fixtures for the 20 NEW single-ability STAT/Belt-Headband
literal-verified equipment units (SD-33 AT-33-E5-002 remediation) --
the 21 already examined by attempt 1 are kept unchanged, not regenerated.

Reuses AT-33-E5-001/002's exact proven mechanism unmodified: same base
ability scores (STR16 DEX14 CON14 INT10 WIS10 CHA8), same Level-1 Human
Fighter template, same EQUIPSET Belt/Headband slot convention, same
e5-equip-stats.txt.ftl output template (unmodified). Only the item list is
new.
"""
import json, os, sys

BASE = {'STR': 16, 'DEX': 14, 'CON': 14, 'INT': 10, 'WIS': 10, 'CHA': 8}
CAMPAIGN_NAME = {
    'core_rulebook': 'Core Rulebook',
    'ultimate_equipment': 'Ultimate Equipment',
    'inner_sea_gods': 'Inner Sea Gods',
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
PLAYERNAME:sd-33-e5-literal

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
EQUIPSET:{slot}|ID:0.1.01|VALUE:{itemname}|QUANTITY:1.0|USETEMPMODS:Y
"""


def slug(unit_id):
    return unit_id.split(':')[-1]


def main():
    census_path, out_dir, manifest_out = sys.argv[1], sys.argv[2], sys.argv[3]
    census = json.load(open(census_path))
    known21 = {
        'anaconda_s_coils', 'belt_of_the_weasel', 'belt_of_thunderous_charging',
        'cord_of_stubborn_resolve', 'elemental_earth_belt', 'gorgon_belt',
        'headband_of_aerial_agility_cha_2', 'headband_of_aerial_agility_cha_4',
        'headband_of_aerial_agility_cha_6', 'headband_of_aerial_agility_int_2',
        'headband_of_aerial_agility_int_4', 'headband_of_aerial_agility_int_6',
        'headband_of_aerial_agility_wis_2', 'headband_of_aerial_agility_wis_4',
        'headband_of_aerial_agility_wis_6', 'headband_of_ponderous_recollection',
        'headband_of_unshakeable_resolve', 'minotaur_belt', 'monkey_belt',
        'plague_rat_belt', 'shadowform_belt',
    }
    new_items = [x for x in census['items'] if slug(x['unit_id']) not in known21]

    os.makedirs(out_dir, exist_ok=True)
    manifest = []
    for item in new_items:
        s = slug(item['unit_id'])
        book = item['book']
        extra = ''
        if book != 'core_rulebook':
            extra = f"CAMPAIGN:{CAMPAIGN_NAME[book]}\n"
        expected_total = BASE[item['ability']] + item['bonus']
        pcg_text = PCG_TEMPLATE.format(
            extra_campaign=extra,
            charname=f"E5 Lit2 {s}",
            itemname=item['key'],
            slot=item['slot'],
        )
        pcg_path = os.path.join(out_dir, f"{s}.pcg")
        with open(pcg_path, 'w') as f:
            f.write(pcg_text)
        manifest.append({
            'unit_id': item['unit_id'],
            'slug': s,
            'pcg_path': pcg_path,
            'ability': item['ability'],
            'bonus': item['bonus'],
            'expected_total': expected_total,
            'ability_index': ['STR', 'DEX', 'CON', 'INT', 'WIS', 'CHA'].index(item['ability']),
        })
    with open(manifest_out, 'w') as f:
        json.dump(manifest, f, indent=2)
    print(f"wrote {len(manifest)} .pcg files to {out_dir}, manifest {manifest_out}")


if __name__ == '__main__':
    main()
