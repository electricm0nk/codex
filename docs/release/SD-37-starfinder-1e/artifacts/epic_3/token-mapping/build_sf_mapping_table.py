#!/usr/bin/env python3
"""Author of sf-mapping-table.v1.json (SD-37 E3.3): the hand-written rows below, with every oracle
observation value read from the committed oracle-builds/<build>.oracle.txt (never retyped).

Usage: build_sf_mapping_table.py <path to sf-mapping-table.v1.json>
then:  sf_reused_pf_fields.py --write   (fills reused_pf_fields / remapped_pf_fields, left empty here)"""
import json
import sys
from pathlib import Path

T = Path(sys.argv[1])
OB = T.parent / "oracle-builds"
BUILDS = ["sf_seed_soldier_3", "sf_seed_mystic_5", "sf_seed_technomancer_5", "sf_seed_envoy_3", "sf_soldier", "sf_mechanic"]


def run(b):
    return dict(l.split("=", 1) for l in (OB / f"{b}.oracle.txt").read_text().splitlines() if "=" in l)


def obs(*keys):
    return [{"build": b, "key": k, "value": run(b)[k].strip()} for b in BUILDS for k in keys]


CORE = "paizo/core/"
rows = [
    {
        "id": "hit_points",
        "sheet_field": "Hit Points",
        "pf_reading": "PF reads BONUS:HP|CURRENTMAX as a bonus to the one hit-point total and HD as a die rolled per level; "
                      "it has no RaceHP and reads BONUS:HP|ALTHP as a second hit-point figure. Read that way, a "
                      "reader that adds ALTHP into HP, or swaps the two pools, prints per-level-plausible numbers with "
                      "Con and race HP in the wrong total.",
        "terms": [
            {"id": "class_hp_coefficient", "token": "BONUS:HP|CURRENTMAX", "file": CORE + "scr_classes.lst",
             "record": "CLASS:{class}", "shape": "class_level_coefficient",
             "note": "Soldier 6, Envoy 5, Mystic 5, Mechanic 5, Technomancer 4 per level: one less than the SRD class Hit Points, the HD die makes up the 1"},
            {"id": "hit_die", "token": "HD:", "file": CORE + "scr_classes.lst", "record": "CLASS:{class}",
             "shape": "hit_die_per_level", "note": "HD:1 on every player class: a d1, one point per level (the .pcg stores HITPOINTS:1 per level)"},
            {"id": "race_hp", "token": "BONUS:HP|CURRENTMAX|RaceHP", "file": CORE + "scr_abilities.lst",
             "record": "Default", "shape": "race_hp_var",
             "note": "RaceHP is set by BONUS:VAR|RaceHP|<n> on '<race> Race Selection ~ Default' (Human/Lashunta/Android 4, Ysoki 2)"},
        ],
        "srd": [
            {"url": "https://www.aonsrd.com/Rules.aspx?ID=49", "section": "Calculating Hit Points: racial Hit Points at 1st level + class Hit Points each level"},
            {"url": "https://www.aonsrd.com/Classes.aspx?ItemName=Soldier", "section": "Hit Points: 7"},
            {"url": "https://www.aonsrd.com/Races.aspx?ItemName=Human", "section": "Hit Points 4"},
        ],
        "oracle": obs("hp", "var.race_hp"),
    },
    {
        "id": "stamina",
        "sheet_field": "Stamina Points",
        "pf_reading": "PF has no Stamina; BONUS:HP|ALTHP read through the PF mapping is an alternate hit-point "
                      "figure, and Con-to-hit-points (PF's per-level Con bonus) would put CON*TL into Hit Points.",
        "terms": [
            {"id": "class_sp_coefficient", "token": "BONUS:HP|ALTHP", "file": CORE + "scr_classes.lst",
             "record": "CLASS:{class}", "shape": "class_level_coefficient",
             "note": "Soldier 7, Envoy 6, Mystic 6, Mechanic 6, Technomancer 5: equal to the SRD class Stamina Points"},
            {"id": "con_mod_per_level", "token": "BONUS:HP|ALTHP|CON*TL", "file": CORE + "scr__stats.lst",
             "record": "Constitution", "shape": "con_mod_times_level"},
            {"id": "toughness", "token": "BONUS:HP|ALTHP|TL", "file": CORE + "scr_feats.lst",
             "record": "Toughness", "shape": "feat_total_level"},
        ],
        "srd": [
            {"url": "https://www.aonsrd.com/Rules.aspx?ID=49", "section": "Calculating Stamina Points: class Stamina Points + Constitution modifier each level"},
            {"url": "https://www.aonsrd.com/Classes.aspx?ItemName=Soldier", "section": "Stamina Points: 7"},
            {"url": "https://www.aonsrd.com/FeatDisplay.aspx?ItemName=Toughness", "section": "Benefit: 1 Stamina Point for every character level"},
        ],
        "oracle": obs("althp"),
    },
    {
        "id": "eac",
        "sheet_field": "Energy Armor Class",
        "pf_reading": "PF reads every BONUS:COMBAT|AC as one Armor Class and prints AC.Total, which here sums EAC_Armor "
                      "and KAC_Armor: the six runs print AC.Total 24/20/24/18/49/28 against EAC 16/16/18/14/30/21.",
        "terms": [
            {"id": "ac_base", "token": "BONUS:COMBAT|AC|10|TYPE=Base", "file": CORE + "scr__stats.lst",
             "record": "Dexterity", "shape": "ac_base"},
            {"id": "eac_armor", "token": "BONUS:COMBAT|AC|TYPE=EAC_Armor", "file": CORE + "scr_equip.lst",
             "record": "KEY:{armor}", "shape": "armor_typed_bonus"},
            {"id": "dex_capped", "token": "BONUS:COMBAT|AC|min(ACAbilityStat, min(MXDXEN,MODEQUIPMAXDEX))|TYPE=Ability|PREVAREQ:ACStatNotDex,0",
             "file": CORE + "scr__stats.lst", "record": "Dexterity", "shape": "dex_capped_by_max_dex",
             "armor_file": CORE + "scr_equip.lst"},
        ],
        "srd": [
            {"url": "https://www.aonsrd.com/Rules.aspx?ID=102", "section": "Armor Class: EAC = 10 + armor's EAC bonus + Dex modifier"},
            {"url": "https://www.aonsrd.com/Rules.aspx?ID=87", "section": "Maximum Dex Bonus"},
        ],
        "oracle": obs("ac.eac", "ac.total"),
    },
    {
        "id": "kac",
        "sheet_field": "Kinetic Armor Class",
        "pf_reading": "As eac: the PF reading prints AC.Total (EAC_Armor + KAC_Armor + Dex + 10), not KAC.",
        "terms": [
            {"id": "ac_base", "token": "BONUS:COMBAT|AC|10|TYPE=Base", "file": CORE + "scr__stats.lst",
             "record": "Dexterity", "shape": "ac_base"},
            {"id": "kac_armor", "token": "BONUS:COMBAT|AC|TYPE=KAC_Armor", "file": CORE + "scr_equip.lst",
             "record": "KEY:{armor}", "shape": "armor_typed_bonus"},
            {"id": "dex_capped", "token": "BONUS:COMBAT|AC|min(ACAbilityStat, min(MXDXEN,MODEQUIPMAXDEX))|TYPE=Ability|PREVAREQ:ACStatNotDex,0",
             "file": CORE + "scr__stats.lst", "record": "Dexterity", "shape": "dex_capped_by_max_dex",
             "armor_file": CORE + "scr_equip.lst"},
        ],
        "srd": [
            {"url": "https://www.aonsrd.com/Rules.aspx?ID=102", "section": "Armor Class: KAC = 10 + armor's KAC bonus + Dex modifier"},
            {"url": "https://www.aonsrd.com/Rules.aspx?ID=87", "section": "Maximum Dex Bonus"},
        ],
        "oracle": obs("ac.kac", "ac.total"),
    },
    {
        "id": "key_ability",
        "sheet_field": "Key Ability Score",
        "pf_reading": "PF has no key ability; FACT is a free-text fact, so a PF reader prints the string ('Str or Dex') "
                      "and cannot resolve the choice forms the Resolve total depends on.",
        "terms": [
            {"id": "key_ability_fact", "token": "FACT:KeyAbilityScore", "file": CORE + "scr_classes.lst",
             "record": "CLASS:{class}", "shape": "key_ability_fact",
             "note": "fixed (CHA, WIS, INT, DEX, CON) or choice ('Str or Dex' Soldier; 'INT or WIS' Biohacker, COM); the choice comes from the build ('Soldier Key Ability ~ STR' in the .pcg)"},
        ],
        "srd": [
            {"url": "https://www.aonsrd.com/Classes.aspx?ItemName=Soldier", "section": "Key Ability Score - Str|Dex"},
            {"url": "https://www.aonsrd.com/Classes.aspx?ItemName=Mystic", "section": "Key Ability Score - Wis"},
        ],
        "oracle": obs("var.key_ability_score", "var.key_ability_bonus"),
    },
    {
        "id": "resolve",
        "sheet_field": "Resolve Points",
        "pf_reading": "PF has no Resolve; the PF mapping reads BONUS:VAR|Resolve as an opaque variable contribution "
                      "with no sheet line.",
        "terms": [
            {"id": "level_and_key", "token": "BONUS:VAR|Resolve|max(1,Resolve_PCLvl+KeyAbilityBonus)", "file": CORE + "scr_abilities.lst",
             "record": "Default", "shape": "resolve_level_and_key",
             "note": "with BONUS:VAR|Resolve_PCLvl|max(1,EffectiveLVL/2): max(1, max(1, level/2) + key modifier). The SRD has no outer max(1, …); the two differ only when the key modifier makes the sum < 1"},
            {"id": "extra_resolve", "token": "BONUS:VAR|Resolve|2", "file": CORE + "scr_feats.lst",
             "record": "Extra Resolve", "shape": "feat_flat_bonus"},
        ],
        "srd": [
            {"url": "https://www.aonsrd.com/Rules.aspx?ID=50", "section": "Calculating Resolve Points: half character level (minimum 1) + key ability modifier"},
            {"url": "https://www.aonsrd.com/FeatDisplay.aspx?ItemName=Extra%20Resolve", "section": "Benefit: 2 additional Resolve Points"},
        ],
        "oracle": obs("var.resolve"),
    },
]

table = {
    "schema": "sf-mapping-table.v1",
    "system": "starfinder-1e",
    "oracle_sha": "7f818006e371188e5717fd18d74d18a420747fc6",
    "decision": "docs/release/SD-37-starfinder-1e/decisions.md §8",
    "reading_rule": "Each row is one sheet field; its terms are the oracle tokens that feed it, summed. A term names "
                    "its token, the .lst (under the corpus starfinder/ directory) and record that carry it, and a "
                    "shape the evaluator (codex-ingest sheet_rule::sf_mapping) knows. A shape the evaluator does not "
                    "know, a token missing from its carrier, or an unresolved choice is refused by name. Every "
                    "oracle observation is a value a real PCGen run printed for a named build "
                    "(oracle-builds/<build>.oracle.txt, re-run by oracle-builds/run_oracle_builds.sh).",
    "oracle_builds": {
        "sf_seed_soldier_3": "oracle-builds/sf_seed_soldier_3.pcg (SD-37 seed SF-Soldier-3, artifacts/epic_0/seed-builds.md §1)",
        "sf_seed_mystic_5": "oracle-builds/sf_seed_mystic_5.pcg (SF-Mystic-5, §2)",
        "sf_seed_technomancer_5": "oracle-builds/sf_seed_technomancer_5.pcg (SF-Technomancer-5, §3)",
        "sf_seed_envoy_3": "oracle-builds/sf_seed_envoy_3.pcg (SF-Envoy-3, §4)",
        "sf_soldier": "$PCGEN_REPO_DIR/code/testsuite/PCGfiles/sf_soldier.pcg (PCGen's own test character: android soldier 10)",
        "sf_mechanic": "$PCGEN_REPO_DIR/code/testsuite/PCGfiles/sf_mechanic.pcg (PCGen's own test character: ysoki mechanic 20, Toughness)",
    },
    "rows": rows,
    "refusals": [
        {"field": "BONUS:HP|ALTHP|DroneMasterLVL (Energy Shield, drone)", "why": "drone/companion sheets are not built (no seed reaches a drone); refused by name until a drone build has an oracle row"},
        {"field": "BONUS:HP|CURRENTMAX on CLASS:Drone", "why": "as above"},
        {"field": "BONUS:HP|CURRENTMAX|1 (+1 Hit Point ability)", "why": "no seed or PCGen test character holds it; it feeds hit_points by its token but has no oracle observation, so it is named here, not mapped"},
        {"field": "HD:<n> with n != 1", "why": "no Starfinder player class has one; a rolled die is refused by the evaluator"},
    ],
    "reused_pf_fields": [],
    "remapped_pf_fields": [],
}
T.parent.mkdir(parents=True, exist_ok=True)
T.write_text(json.dumps(table, indent=2, ensure_ascii=False) + "\n")
print(T)
