#!/usr/bin/env python3
"""sd33-r3-statsave: .pcg generator for this lane's 39-unit STAT_multi_or_
other_slot population (re-derived, see receipt). Reuses AT-33-E5-001/002's
proven Level-1 Human Fighter / fixed base scores (STR16 DEX14 CON14 INT10
WIS10 CHA8) mechanism and the equipment-remainder SKILL lane's proven
`EQUIPSET:Equipped` fix (Carried does not activate BONUS: tokens for a
slotless item -- confirmed empirically that cycle). One item equipped per
character; oracle side reads STAT.<ability_index>.SCORE (total score with
the item's bonus applied), exactly `e5_literal_stat_ours`'s own established
convention (base score + engine's parsed `ability_bonus.bonus`, unmodified
binary, reused as-is).

For a multi-ability item (`BONUS:STAT|STR,DEX,CON|n|...`), this lane
verifies the FIRST named ability only -- PF1's real rule applies the SAME
bonus to every named ability independently, so this checks the mechanism
once per unit rather than once per named ability (a documented scope
decision, not a gap -- see receipt).

Usage: statsave-generate-stat-pcgs.py <manifest.json> <out_dir>
"""
import json, os, sys

BASE = {'STR': 16, 'DEX': 14, 'CON': 14, 'INT': 10, 'WIS': 10, 'CHA': 8}
CAMPAIGN_NAME = {
    'core_rulebook': 'Core Rulebook',
    'ultimate_equipment': 'Ultimate Equipment',
    'advanced_players_guide': "Advanced Player's Guide",
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


def slug(unit_id):
    return unit_id.split(':')[-1]


def main():
    manifest_path, out_dir = sys.argv[1], sys.argv[2]
    manifest = json.load(open(manifest_path))
    os.makedirs(out_dir, exist_ok=True)
    # Cross-book reprints can share a bare item-name slug (this lane's own
    # SKILL sibling census found 3 -- see statsave-generate-skill-pcgs.py's
    # `slug()` docstring); no collision exists in this STAT-shape 39-unit
    # population (checked this cycle), but a bare-slug collision here would
    # silently overwrite one book's .pcg with another's, so fail loudly
    # instead of trusting that absence to hold on a future re-run.
    slugs_seen = set()
    for item in manifest:
        s = slug(item['unit_id'])
        assert s not in slugs_seen, f"slug collision: {s}"
        slugs_seen.add(s)
        book = item['book']
        extra = f"CAMPAIGN:{CAMPAIGN_NAME[book]}\n" if book != 'core_rulebook' else ''
        pcg_text = PCG_TEMPLATE.format(
            extra_campaign=extra,
            charname=f"R3 Stat {s}",
            itemname=item['key'],
        )
        with open(os.path.join(out_dir, f"{s}.pcg"), 'w') as f:
            f.write(pcg_text)
    print(f"wrote {len(manifest)} .pcg files to {out_dir}")


if __name__ == '__main__':
    main()
