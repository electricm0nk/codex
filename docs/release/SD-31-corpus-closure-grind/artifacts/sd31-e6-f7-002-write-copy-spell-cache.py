#!/usr/bin/env python3
"""SD31-E6-F7-002: hand-write the 13 `.COPY=` racial-SLA spell cache records
(decisions.md §15). Deliberately NOT produced via gen_core_rulebook_cache.rs /
gen_book_cache.rs -- running either wholesale against the current checked-in
data/corpus/ this cycle revealed both generators are stale relative to the
committed schema (they silently drop the `raw_tokens` field and 4700+ other
files diverge), which is exactly the "regenerate data/corpus/ wholesale"
hazard the wave brief forbids. This script writes ONLY these 13 new files,
each schema-identical (field order, `raw_tokens` shape) to its sibling
records, sourced one hop from the pinned oracle: description/level/school
inherit from the named parent (already encoded identically in
rules_tables::crb::spell_list::SPELL_LIST / advanced_race_guide::spell_list),
`raw_tokens`/`source` reflect this record's OWN corpus row.
"""
import json
import os

CRB_SHA = "6da07d57030565fec332991219192133f06d699a960d9d2cb9a5da9e9282bb31"
ARG_SHA = "ed4570f19fdea4c1392b542e2ecfa11fb091094e17c1de9d34bac7830f834f58"
INGESTED_AT = "2026-08-17T00:00:00Z"

REPO = "/home/ubuntu/workspace/repos/codex/.claude/worktrees/wf_c9995ce5-db0-6"

ANIMATE_OBJECTS_DESC = "You imbue inanimate objects with mobility and a semblance of life. Each such animated object then immediately attacks whomever or whatever you initially designate. An animated object can be of any nonmagical material. You may animate one Small or smaller object or a corresponding number of larger objects as follows: A Medium object counts as two Small or smaller objects, a Large object as four, a Huge object as eight, a Gargantuan object as 16, and a Colossal object as 32. You can change the designated target or targets as a move action, as if directing an active spell. This spell cannot affect objects carried or worn by a creature. Animate objects can be made permanent with a permanency spell."
CHARM_ANIMAL_DESC = "This spell functions like charm person, except that it affects a creature of the animal type."
DISGUISE_SELF_DESC = "You make yourself-including clothing, armor, weapons, and equipment-look different. You can seem 1 foot shorter or taller, thin, fat, or in between. You cannot change your creature type [although you can appear as another subtype]. Otherwise, the extent of the apparent change is up to you. You could add or obscure a minor feature or look like an entirely different person or gender. The spell does not provide the abilities or mannerisms of the chosen form, nor does it alter the perceived tactile [touch] or audible [sound] properties of you or your equipment. If you use this spell to create a disguise, you get a +10 bonus on the Disguise check. A creature that interacts with the glamer gets a Will save to recognize it as an illusion."
NONDETECTION_DESC = "The warded creature or object becomes difficult to detect by divination spells such as clairaudience/clairvoyance, locate object, and detect spells. Nondetection also prevents location by such magic items as crystal balls. If a divination is attempted against the warded creature or item, the caster of the divination must succeed on a caster level check [1d20 + caster level] against a DC of 11 + the caster level of the spellcaster who cast nondetection. If you cast nondetection on yourself or on an item currently in your possession, the DC is 15 + your caster level. If cast on a creature, nondetection wards the creature's gear as well as the creature itself."
PLANE_SHIFT_DESC = "You move yourself or some other creature to another plane of existence or alternate dimension. If several willing persons link hands in a circle, as many as eight can be affected by the plane shift at the same time. Precise accuracy as to a particular arrival location on the intended plane is nigh impossible. From the Material Plane, you can reach any other plane, though you appear 5 to 500 miles [5d%] from your intended destination. Plane shift transports creatures instantaneously and then ends. The creatures need to find other means if they are to travel back [including casting plane shift again]."
SPEAK_WITH_ANIMALS_DESC = "You can ask questions of and receive answers from animals, but the spell doesn't make them any more friendly than normal. Wary and cunning animals are likely to be terse and evasive, while the more stupid ones make inane comments. If an animal is friendly toward you, it may do some favor or service for you."
SUMMON_MONSTER_III_DESC = "This spell functions like summon monster I, except that you can summon one creature from the 3rd-level list, 1d3 creatures of the same kind from the 2nd-level list, or 1d4+1 creatures of the same kind from the 1st-level list."
SUMMON_NATURES_ALLY_I_DESC = "This spell summons to your side a natural creature [typically an animal, fey, magical beast, outsider with the elemental subtype, or a giant]. The summoned ally appears where you designate and acts immediately, on your turn. It attacks your opponents to the best of its ability. If you can communicate with the creature, you can direct it not to attack, to attack particular enemies, or to perform other actions as you command. A summoned monster cannot summon or otherwise conjure another creature, nor can it use any teleportation or planar travel abilities. Creatures cannot be summoned into an environment that cannot support them. Creatures summoned using this spell cannot use spells or spell-like abilities that duplicate spells that have expensive material components [such as wish]. The spell conjures one of the creatures from the 1st Level list on Table 10-6. You choose which kind of creature to summon, and you can change that choice each time you cast the spell. All the creatures on the table are neutral unless otherwise noted. When you use a summoning spell to summon a creature with an alignment or elemental subtype, it is a spell of that type. All creatures summoned with this spell without alignment subtypes have an alignment that matches yours, regardless of their usual alignment. Summoning these creatures makes the summoning spell's type match your alignment. [Chart not included]"
FINS_TO_FEET_DESC = "You transform the target's fins, flippers, or tail into legs and feet, allowing it to walk on land. The target loses its swim speed but gains a base speed appropriate for a humanoid of its size (speed 30 if a Medium or larger creature, speed 20 if Small). If the creature is immersed in water for 1 round, the transformation reverts, allowing it to swim normally. One round after leaving the water, the transformation occurs again, allowing it to walk. This spell only works on merfolk, tritons, seals, fish, and other creatures whose bodies or limbs are used mainly for swimming and are not suitable for walking. It does not give the target the ability to breathe air."

CRB_RECORDS = [
    # (key, level, school, line, parent, extra_raw_tokens, description)
    ("Animate Objects (Small or Smaller)", 6, "Transmutation", 1467, "Animate Objects", [], ANIMATE_OBJECTS_DESC),
    ("Charm Animal (aquatic animals only)", 1, "Enchantment", 1468, "Charm Animal", [], CHARM_ANIMAL_DESC),
    ("Disguise Self (humanoid only)", 1, "Illusion", 1469, "Disguise Self", [], DISGUISE_SELF_DESC),
    ("Nondetection (self only)", 3, "Abjuration", 1470, "Nondetection", [], NONDETECTION_DESC),
    ("Plane Shift (self only/to Shadow or Material Plane)", 5, "Conjuration", 1471, "Plane Shift",
     [("OUTPUTNAME", "Plane Shift (self only, to the Plane of Shadow or the Material Plane only)")], PLANE_SHIFT_DESC),
    ("Plane Shift (to Shadow or Material Plane)", 5, "Conjuration", 1472, "Plane Shift",
     [("OUTPUTNAME", "Plane Shift (to the Plane of Shadow or the Material Plane only)")], PLANE_SHIFT_DESC),
    ("Speak with Animals (aquatic animals only)", 1, "Divination", 1473, "Speak with Animals", [], SPEAK_WITH_ANIMALS_DESC),
    ("Speak with Animals (birds or other flying animals only)", 1, "Divination", 1474, "Speak with Animals", [], SPEAK_WITH_ANIMALS_DESC),
    ("Speak with Animals (pigs and boars only)", 1, "Divination", 1475, "Speak with Animals", [], SPEAK_WITH_ANIMALS_DESC),
    ("Speak with Animals (rodents only)", 1, "Divination", 1476, "Speak with Animals", [], SPEAK_WITH_ANIMALS_DESC),
    ("Summon Monster III (lantern archon only)", 3, "Conjuration", 1477, "Summon Monster III", [], SUMMON_MONSTER_III_DESC),
    ("Summon Nature's Ally I (dolphins only)", 1, "Conjuration", 1478, "Summon Nature's Ally I", [], SUMMON_NATURES_ALLY_I_DESC),
]

ARG_RECORDS = [
    ("Fins to Feet (self only)", 3, "Transmutation", 230, "Fins to Feet", [], FINS_TO_FEET_DESC),
]


def slugify(name: str) -> str:
    out = []
    for ch in name.lower():
        if ch.isalnum():
            out.append(ch)
        elif ch in " -/'":
            out.append("_")
    slug = "".join(out)
    while "__" in slug:
        slug = slug.replace("__", "_")
    return slug.strip("_")


def write_record(path, key, level, school, line, parent, extra_tokens, description, corpus_path, sha, book_dir, level_bucketed):
    raw_tokens = list(extra_tokens) + [("CLASSES", ".CLEARALL")]
    record = {
        "population": "in_scope",
        "completeness": "full",
        "ingested_at": INGESTED_AT,
        "data": {
            "description": description,
            "key": key,
            "level": level,
            "raw_tokens": [{"key": k, "value": v} for k, v in raw_tokens],
            "school": school,
        },
        "source": {
            "kind": "lst_token",
            "path": corpus_path,
            "sha256": sha,
            "line": line,
            "record_key": f"{parent}.COPY={key}",
        },
        "wiring_class": "computed",
        "wiring_class_signals": ["computed:pre_guard", "derived:bonus", "derived:spells"],
        "license": "OGL",
        "pi_field": None,
        "pi_marker": None,
    }
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        json.dump(record, f, indent=2, sort_keys=True)
        f.write("\n")
    print("wrote", path)


def main():
    crb_dir = os.path.join(REPO, "data/corpus/core_rulebook/spell")
    for key, level, school, line, parent, extra, desc in CRB_RECORDS:
        slug = slugify(key)
        path = os.path.join(crb_dir, f"level_{level}", f"{slug}.json")
        write_record(path, key, level, school, line, parent, extra, desc,
                     "pathfinder/paizo/roleplaying_game/core_rulebook/cr_spells.lst", CRB_SHA, crb_dir, True)

    arg_dir = os.path.join(REPO, "data/corpus/advanced_race_guide/spell")
    for key, level, school, line, parent, extra, desc in ARG_RECORDS:
        slug = slugify(key)
        path = os.path.join(arg_dir, f"{slug}.json")
        write_record(path, key, level, school, line, parent, extra, desc,
                     "pathfinder/paizo/roleplaying_game/advanced_race_guide/arg_spells.lst", ARG_SHA, arg_dir, False)


if __name__ == "__main__":
    main()
