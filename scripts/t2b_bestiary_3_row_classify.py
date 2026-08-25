#!/usr/bin/env python3
"""SD-32 card 11, shape T2b, lane t2b-w1-d -- row-content classification of
bestiary_3's 819 `race_trait_race_not_modelled` units.

`card11-t2b-census-census.md` sized bestiary_3 as "819 units, unregistered
book, needs full RACE_CORPUS_BOOKS onboarding" purely from book-level
cross-referencing (is the book in `RACE_CORPUS_BOOKS`? does its
`*_abilities_race.lst` file exist in the pinned oracle?). It did not run the
row-content classification it applied to the *registered*-book pile (the
147-header / 9-adopted-race / 562-other split in that memo's own §3) against
any of the 17 *unregistered* books, bestiary_3 included.

This script does that row-content classification for bestiary_3, re-deriving
directly from `docs/work-inventory.json` (no doc-quoted numbers) and
cross-referencing each unit's `type_facet` and `corpus_key` against:

1. The census memo's own established by-design-exclusion rule (§3 rule 2):
   `corpus_key` matching `^(Racial SLA|Unchained Evolution|Favored Class
   Bonus|Race Subtype) ~ ` is a category-header row, not open work.
2. The census memo's own established real-work rule (§3 rule 1):
   `corpus_key` starting `Adopted Race ~ ` is a real, closable
   selector-capture-gap row.
3. A NEW finding this script establishes: `type_facet`'s first dot-segment
   often does NOT match `MONSTER_ABILITY_TYPE_FACETS`
   (`src/bin/v06_work_inventory.rs`) even when the row is genuine monster
   special-ability content, because bestiary-style books use compound,
   race-specific TYPE first segments (`AghashRacialAbility`,
   `RaceAbility`, `BearLordRacialTrait`, `AdletSelection`, ...) rather than
   the bare `SpecialQuality`/`SpecialAttack`/`NaturalAttack`/`Universal
   Monster Rule` vocabulary that check matches. This script cross-references
   each remaining unit's KEY prefix (the text before " ~ ") against the
   book's own `b3_races.lst` CR:-bearing race names and `b3_templates.lst`
   template names (both re-read fresh from the pinned oracle, not
   hand-curated) to confirm the row belongs to a non-playable monster/
   template entity, not a player race.

Run: python3 scripts/t2b_bestiary_3_row_classify.py
"""
import json
import re

WORK_INVENTORY = "docs/work-inventory.json"
ORACLE_BOOK = (
    "docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/"
    "operator-supplied/pcgen/data/pathfinder/paizo/roleplaying_game/bestiary_3"
)


def cr_bearing_race_names():
    """First-field names from b3_races.lst whose row carries a CR: token --
    the corpus's own discriminator (`v06_work_inventory.rs`'s `refine_kind`:
    "A `*_races.lst` row carrying a `CR:` token is a monster")."""
    names = set()
    with open(f"{ORACLE_BOOK}/b3_races.lst", encoding="utf-8", errors="replace") as f:
        for line in f:
            if line.startswith("#") or not line.strip():
                continue
            fields = line.rstrip("\n").split("\t")
            first = fields[0].strip()
            if not first or first.startswith("SOURCELONG") or first.startswith("###"):
                continue
            if any("CR:" in field for field in fields):
                names.add(first)
    return names


def template_names():
    """First-field names from b3_templates.lst -- creature templates, never
    player races."""
    names = set()
    try:
        with open(f"{ORACLE_BOOK}/b3_templates.lst", encoding="utf-8", errors="replace") as f:
            for line in f:
                if line.startswith("#") or not line.strip():
                    continue
                fields = line.rstrip("\n").split("\t")
                first = fields[0].strip()
                if first and not first.startswith("SOURCELONG"):
                    names.add(first)
    except FileNotFoundError:
        pass
    return names


def load_units():
    d = json.load(open(WORK_INVENTORY))
    return [
        x for x in d["units"]
        if x.get("kind") == "race_trait"
        and x.get("evidence") == "race_trait_race_not_modelled"
        and x.get("book") == "bestiary_3"
    ]


def main():
    units = load_units()
    print(f"# bestiary_3 T2b units (race_trait_race_not_modelled): {len(units)}")

    monster_names = cr_bearing_race_names()
    tmpl_names = template_names()
    print(f"# CR:-bearing race names in b3_races.lst: {len(monster_names)}")
    print(f"# template names in b3_templates.lst: {len(tmpl_names)}")

    header_re = re.compile(r"^(Racial SLA|Unchained Evolution|Favored Class Bonus|Race Subtype) ~ ")

    buckets = {"category_header": [], "adopted_race": [], "monster_or_template_owned": [], "unresolved": []}
    for u in units:
        key = u.get("corpus_key", "")
        if key.startswith("Adopted Race ~ "):
            buckets["adopted_race"].append(u)
            continue
        if header_re.match(key):
            buckets["category_header"].append(u)
            continue
        prefix = key.split(" ~ ", 1)[0] if " ~ " in key else key
        if prefix in monster_names or prefix in tmpl_names:
            buckets["monster_or_template_owned"].append(u)
            continue
        buckets["unresolved"].append(u)

    for name, items in buckets.items():
        print(f"\n{name}: {len(items)}")
        for u in items[:10]:
            print("   ", u.get("corpus_key"), "|", u.get("type_facet"), "|", u.get("source_file"), u.get("source_line"))
        if len(items) > 10:
            print(f"    ... and {len(items) - 10} more")

    print(f"\nsum check: {sum(len(v) for v in buckets.values())} (expect {len(units)})")


if __name__ == "__main__":
    main()
