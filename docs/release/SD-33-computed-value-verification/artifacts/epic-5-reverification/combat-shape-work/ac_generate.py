#!/usr/bin/env python3
"""SD-33 Epic 5 combat/weapon lane (AT-33-E5-002 remainder, COMBAT|AC
sub-population): generates one .pcg per AC-shape unit (Level-1 Human
Fighter, base ability scores STR16/DEX14/CON14/INT10/WIS10/CHA8, one item
equipped at EQUIPSET:Equipped -- the exact fixture pattern
AT-33-E5-002/AT-33-E5-remainder-equipment already proved for a slotless
item), plus one baseline .pcg with nothing equipped. All units share ONE
static .ftl export template (ac-check.txt.ftl, AC.Total only) -- unlike
the SKILL-shape lane, the export token name (AC.Total) does not vary per
unit, so no per-unit .ftl is needed.

Oracle comparable value = AC.Total(item equipped) - AC.Total(baseline) --
a baseline-diff, not a per-bonus-type AC.<TYPE> token lookup, deliberately:
this population's own real corpus records carry TYPE=Deflection/
NaturalArmor/NaturalArmorEnhancement/Luck/Insight/Circumstance/Armor/
Shield, AND at least one real corpus record with a BARE, non-`TYPE=`-
prefixed bonus-type qualifier (`BONUS:COMBAT|AC|4|NaturalArmor`, no
`TYPE=` prefix -- confirmed against real PCGen parser source that this is
never registered as a bonus type at all). A baseline-diff on AC.Total is
robust to every one of those shapes uniformly, including the grammar
quirk, without needing to guess or verify each literal PCGen AC-type
name string separately.

Usage: ac_generate.py <census.json> <population.json> <out_dir> <manifest_out.json>
"""
import json, os, sys

CAMPAIGN_NAME = {
    'core_rulebook': 'Core Rulebook',
    'advanced_class_guide': 'Advanced Class Guide',
    'advanced_players_guide': "Advanced Player's Guide",
    'advanced_race_guide': 'Advanced Race Guide',
    'inner_sea_gods': 'Inner Sea Gods',
    'inner_sea_races': 'Inner Sea Races',
    'inner_sea_world_guide': 'Inner Sea World Guide',
    'ultimate_combat': 'Ultimate Combat',
    'ultimate_equipment': 'Ultimate Equipment',
    'ultimate_intrigue': 'Ultimate Intrigue',
    'ultimate_magic': 'Ultimate Magic',
    'ultimate_psionics': 'Ultimate Psionics',
}

# Full transitive PRECAMPAIGN closure per book, read directly from each
# book's own real .pcc file this cycle (never guessed -- see receipt):
# advanced_players_guide.pcc / _advanced_class_guide.pcc /
# advanced_race_guide.pcc / _inner_sea_gods.pcc / _inner_sea_races.pcc /
# inner_sea_world_guide.pcc / _ultimate_combat.pcc / ultimate_equipment.pcc /
# _ultimate_intrigue.pcc / _ultimate_magic.pcc / ultimate_psionics.pcc /
# bestiary.pcc, each under `$PCGEN_REPO_DIR/data/pathfinder/...`. Every
# closure includes 'core_rulebook' first, which the template's own
# unconditional `CAMPAIGN:Core Rulebook` line already covers.
CAMPAIGN_CLOSURE = {
    'core_rulebook': [],
    'advanced_players_guide': ["Advanced Player's Guide"],
    'advanced_class_guide': [
        "Advanced Player's Guide", 'Ultimate Combat', 'Ultimate Equipment', 'Ultimate Magic', 'Advanced Class Guide',
    ],
    'advanced_race_guide': ["Advanced Player's Guide", 'Ultimate Combat', 'Ultimate Magic', 'Advanced Race Guide'],
    'inner_sea_gods': [
        "Advanced Player's Guide", 'Ultimate Magic', 'Inner Sea World Guide', 'Bestiary',
        'Ultimate Equipment', 'Ultimate Combat', 'Inner Sea Gods',
    ],
    'inner_sea_races': [
        "Advanced Player's Guide", 'Advanced Race Guide', 'Inner Sea World Guide',
        'Ultimate Combat', 'Ultimate Equipment', 'Ultimate Magic', 'Inner Sea Races',
    ],
    'inner_sea_world_guide': ["Advanced Player's Guide", 'Inner Sea World Guide'],
    'ultimate_combat': ["Advanced Player's Guide", 'Ultimate Combat'],
    'ultimate_equipment': ["Advanced Player's Guide", 'Ultimate Equipment'],
    'ultimate_intrigue': [
        "Advanced Player's Guide", 'Ultimate Magic', 'Ultimate Combat', 'Ultimate Equipment',
        'Advanced Class Guide', 'Ultimate Intrigue',
    ],
    'ultimate_magic': ["Advanced Player's Guide", 'Ultimate Magic'],
    'ultimate_psionics': ['Ultimate Psionics'],
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
PLAYERNAME:sd-33-r3-combat

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
{equip_block}"""

EQUIP_BLOCK = """EQUIPNAME:{itemname}|OUTPUTORDER:1|COST:0|WT:0|QUANTITY:1.0
EQUIPSET:Default Set|ID:0.1|USETEMPMODS:Y
EQUIPSET:Equipped|ID:0.1.01|VALUE:{itemname}|QUANTITY:1.0|USETEMPMODS:Y
"""

BASELINE_EQUIP_BLOCK = "EQUIPSET:Default Set|ID:0.1|USETEMPMODS:Y\n"


def slug(unit_id):
    return unit_id.split(':')[-1]


def main():
    census_path, pop_path, out_dir, manifest_out = sys.argv[1], sys.argv[2], sys.argv[3], sys.argv[4]
    census = json.load(open(census_path))
    pop = json.load(open(pop_path))

    os.makedirs(out_dir, exist_ok=True)

    # AC-shape units: carry a literal-valued (first) COMBAT|AC chain.
    ac_units = []
    for uid in pop['COMBAT']:
        rec = census[uid]['record']['data']
        chains = [c for c in rec.get('raw_bonus_chains', [])
                  if c['qualifiers'][0] == 'COMBAT' and c['qualifiers'][1] == 'AC']
        if not chains:
            continue
        value_str = chains[0]['qualifiers'][2]
        try:
            value = int(value_str)
        except ValueError:
            # Formula-valued (e.g. "2+Global_LuckBonus") -- real, distinct
            # shape, not attempted this cycle (see receipt).
            continue
        ac_units.append({'unit_id': uid, 'book': uid.split(':')[0], 'key': rec['key'], 'expected_bonus': value})

    manifest = []
    for item in ac_units:
        s = slug(item['unit_id'])
        book = item['book']
        extra = ''.join(f'CAMPAIGN:{name}\n' for name in CAMPAIGN_CLOSURE[book])
        pcg_text = PCG_TEMPLATE.format(
            extra_campaign=extra,
            charname=f"E5 R3 {s}",
            equip_block=EQUIP_BLOCK.format(itemname=item['key']),
        )
        pcg_path = os.path.join(out_dir, f"{s}.pcg")
        with open(pcg_path, 'w') as f:
            f.write(pcg_text)
        manifest.append({
            'unit_id': item['unit_id'],
            'slug': s,
            'book': book,
            'key': item['key'],
            'pcg_path': pcg_path,
            'expected_bonus': item['expected_bonus'],
        })

    # One baseline .pcg per book actually needed (a book's own CAMPAIGN
    # load can shift the level-1 Human Fighter's baseline AC by 0 -- no
    # class/race feature here reads CAMPAIGN -- but keeping one baseline
    # per book used avoids relying on that assumption silently; still far
    # cheaper than one PER ITEM).
    books_used = sorted(set(item['book'] for item in manifest))
    baseline_manifest = []
    for book in books_used:
        extra = ''.join(f'CAMPAIGN:{name}\n' for name in CAMPAIGN_CLOSURE[book])
        pcg_text = PCG_TEMPLATE.format(
            extra_campaign=extra,
            charname=f"E5 R3 baseline {book}",
            equip_block=BASELINE_EQUIP_BLOCK,
        )
        pcg_path = os.path.join(out_dir, f"baseline_{book}.pcg")
        with open(pcg_path, 'w') as f:
            f.write(pcg_text)
        baseline_manifest.append({'book': book, 'pcg_path': pcg_path, 'slug': f'baseline_{book}'})

    with open(manifest_out, 'w') as f:
        json.dump({'items': manifest, 'baselines': baseline_manifest}, f, indent=2)
    print(f"wrote {len(manifest)} item .pcg + {len(baseline_manifest)} baseline .pcg to {out_dir}, manifest {manifest_out}")
    print(f"({len(pop['COMBAT']) - len(ac_units)} COMBAT units excluded: no literal AC chain or formula-valued)")


if __name__ == '__main__':
    main()
