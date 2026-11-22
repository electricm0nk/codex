#!/usr/bin/env python3
"""sd33-r3-statsave: .pcg + per-unit .ftl generator for this lane's 42-unit
SKILL population (both the 17 single-skill and 25 multi-skill/comma-list
units -- for a multi-skill item, this lane verifies the FIRST named skill
only, mirroring the STAT lane's own first-named-target decision: PF1's real
rule applies the SAME bonus to every named skill independently, so this
checks the mechanism once per unit).

Reuses `equipment-remainder-generate-skill-pcgs.py`'s exact proven
mechanism (Level-1 Human Fighter, EQUIPSET:Equipped -- the equip-location
fix that cycle found and fixed -- SKILL.<name>.MISC export token,
SkillToken.SKILL_MISC) unmodified; only the item list (this lane's own 42
units, not that lane's 90) and the always-first-named-skill targeting
(instead of a pre-filtered single-skill-only census) are new.
"""
import json, os, sys

CAMPAIGN_NAME = {
    'core_rulebook': 'Core Rulebook',
    'ultimate_equipment': 'Ultimate Equipment',
    'ultimate_psionics': 'Ultimate Psionics',
    'inner_sea_gods': 'Inner Sea Gods',
    'inner_sea_races': 'Inner Sea Races',
    'advanced_race_guide': 'Advanced Race Guide',
    'advanced_players_guide': "Advanced Player's Guide",
    'book_of_the_damned_volume_2': 'Lords of Chaos - Book of the Damned, Volume 2',
    'advanced_class_guide': 'Advanced Class Guide',
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
PLAYERNAME:sd-33-r3-statsave

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
    """Book-prefixed, not bare item-name -- a bare-slug version of this
    function collided real filenames for three genuine cross-book reprints
    this lane's own population contains (`ring_of_maniacal_devices`,
    `cloak_of_the_diplomat`, `ring_of_the_sophisticate`, each present in
    both `advanced_players_guide`/`advanced_race_guide` and
    `ultimate_equipment`), silently making one book's oracle run stand in
    for the other's. Caught this cycle (both books' declared bonus/skill
    happened to be identical, so the mis-attributed result was still
    numerically correct, confirmed by an independent per-book re-run), then
    fixed at the source rather than left as a footnote -- see this cycle's
    receipt, instrument-correction bucket."""
    book, _, item = unit_id.partition(':equipment:')
    return f"{book}__{item}"


def main():
    manifest_path, out_dir = sys.argv[1], sys.argv[2]
    manifest = json.load(open(manifest_path))
    os.makedirs(out_dir, exist_ok=True)
    slugs_seen = set()
    for item in manifest:
        s = slug(item['unit_id'])
        assert s not in slugs_seen, f"slug collision: {s}"
        slugs_seen.add(s)
        book = item['book']
        extra = f"CAMPAIGN:{CAMPAIGN_NAME[book]}\n" if book != 'core_rulebook' else ''
        pcg_text = PCG_TEMPLATE.format(
            extra_campaign=extra,
            charname=f"R3 Skill {s}",
            itemname=item['key'],
        )
        with open(os.path.join(out_dir, f"{s}.pcg"), 'w') as f:
            f.write(pcg_text)
        ftl_text = FTL_TEMPLATE.format(skill=item['target_skill'])
        with open(os.path.join(out_dir, f"{s}.txt.ftl"), 'w') as f:
            f.write(ftl_text)
    print(f"wrote {len(manifest)} .pcg + .ftl files to {out_dir}")


if __name__ == '__main__':
    main()
