#!/usr/bin/env python3
"""Card 11 T12 census — re-derive script.

T12 (`docs/release/SD-32-compute-library-and-cause-closure/decisions.md §13`):
"2,453 `class_feature`s belonging to classes the engine does not model."

This script:
  1. Confirms the total T12 population (evidence code
     `class_feature_of_unmodelled_corpus_class:<slug>`) from
     `docs/work-inventory.json`.
  2. Groups it by the *evidence-reported* class slug (what T12's own
     classifier currently attributes each unit to).
  3. For each unit, independently re-derives the TRUE owning class from the
     record's own `type_facet` string (the same "<Class> Class Feature(s)"
     marker `class_feature_owner_via_type_facet` in
     `src/bin/v06_work_inventory.rs` already extracts, reproduced here
     read-only) and checks it against the engine's actual modelled-class
     roster (the 34-class DISPATCHED list `epic-2-t2a-t12_cycle-1_cycle_
     receipt.md` audited this same run, reproduced verbatim — no new
     roster is invented here).
  4. Reports every unit whose independently-derived TRUE owner is a
     MODELLED class as a false positive (the evidence-reported slug is
     wrong; the unit does not belong in T12 at all — closing it is a
     classifier fix, not new class-modelling work).
  5. Reports every unit whose evidence-reported slug names a corpus
     "class" that is actually a PCGen monster racial-HD pseudo-class
     (`kind=class`, `type_facet=Monster`, `book=bestiary`/`bonus_bestiary`/
     etc.) as a second, structurally distinct false-positive class: no
     PCGen "class" by that name is ever playable, so no amount of
     class-modelling work could ever close it under that name — it is a
     census/classifier defect, not a content gap.

Nothing here writes to data/corpus, docs/work-inventory.json, or any
pinned count. Read-only.
"""
from __future__ import annotations

import json
import re
import sys
from collections import Counter, defaultdict

WI_PATH = "docs/work-inventory.json"

# The 34-class DISPATCHED/modelled roster, reproduced verbatim from THIS
# run's own `epic-2-t2a-t12_cycle-1_cycle_receipt.md` (itself re-derived,
# not merely cited) — the set `ClassId::ALL` + `ApgClassId::ALL` +
# `AcgClassId::ALL` + `UcClassId::ALL` + `PuClassId::ALL` resolve to in
# `src/bin/v06_work_inventory.rs::modelled_class_books()`.
DISPATCHED = [
    "Barbarian", "Bard", "Cleric", "Druid", "Fighter", "Monk", "Paladin",
    "Ranger", "Rogue", "Sorcerer", "Wizard", "Arcanist", "Bloodrager",
    "Brawler", "Hunter", "Investigator", "Shaman", "Skald", "Slayer",
    "Swashbuckler", "Warpriest", "Alchemist", "Cavalier", "Inquisitor",
    "Oracle", "Summoner", "Witch", "Gunslinger", "Ninja", "Samurai",
    "Unchained Barbarian", "Unchained Monk", "Unchained Rogue",
    "Unchained Summoner",
]
DISPATCHED_SORTED = sorted(DISPATCHED, key=len, reverse=True)


def slug(s: str) -> str:
    return s.lower().replace(" ", "_").replace("'", "")


def _segment_owner(segment: str, roster_sorted_desc: list[str]) -> str | None:
    """Does this single dot-delimited type_facet SEGMENT open with
    '<Class> Class Feature(s)' or '<Class>ClassFeatures'? Anchored at the
    segment's own start -- a substring-anywhere check is unsound here: e.g.
    'MagicWarriorClassFeatures' contains the literal text
    'WarriorClassFeature' (the 'c' ending "Magic" precedes the 'W'), which
    would wrongly credit "Warrior" if matched as a bare substring. Requiring
    the class name to be the segment's own prefix rules that out -- the
    segment starts with "Magic", never "Warrior", so only "Magic Warrior"
    (not itself a corpus class) or nothing can match it."""
    for cls in roster_sorted_desc:
        spaced = f"{cls} Class Feature"
        glued = f"{cls.replace(' ', '')}ClassFeature"
        if segment.startswith(spaced) or segment.startswith(glued):
            return cls
    return None


def true_owner_from_type_facet(type_facet: str | None) -> str | None:
    """Best-effort extraction of the '<Class> Class Feature(s)' or
    '<Class>ClassFeatures' marker a type_facet embeds, matched against the
    DISPATCHED roster only (mirrors
    `class_feature_owner_via_type_facet`'s own marker shape), segment-
    anchored (see `_segment_owner`)."""
    if not type_facet:
        return None
    for segment in type_facet.split("."):
        owner = _segment_owner(segment, DISPATCHED_SORTED)
        if owner is not None:
            return owner
    return None


def main() -> int:
    with open(WI_PATH) as f:
        wi = json.load(f)
    units = wi["units"]

    class_records = {
        u["name"]: u
        for u in units
        if u.get("kind") == "class"
    }
    monster_pseudo_classes = {
        slug(u["name"])
        for u in units
        if u.get("kind") == "class" and u.get("type_facet") == "Monster"
    }

    t12 = [
        u for u in units
        if (u.get("evidence") or "").startswith(
            "class_feature_of_unmodelled_corpus_class"
        )
    ]
    print(f"T12 total (evidence-code count): {len(t12)}")

    by_slug: Counter[str] = Counter()
    fp_modelled_owner: list[dict] = []
    fp_monster_pseudo_class: list[dict] = []
    real_by_true_class: Counter[str] = Counter()
    real_unresolved_by_slug: Counter[str] = Counter()

    for u in t12:
        reported_slug = u["evidence"].split(":", 1)[1]
        by_slug[reported_slug] += 1

        if reported_slug in monster_pseudo_classes:
            fp_monster_pseudo_class.append(u)
            continue

        true_owner = true_owner_from_type_facet(u.get("type_facet"))
        if true_owner is not None:
            fp_modelled_owner.append({**u, "_true_owner": true_owner})
            continue

        real_by_true_class[reported_slug] += 1

    print(f"\nDistinct evidence-reported class slugs: {len(by_slug)}")
    print("\n--- False positive class A: evidence slug names a PCGen "
          "monster racial-HD pseudo-class, never a playable class "
          "(kind=class, type_facet=Monster) ---")
    fpA_by_slug = Counter(u["evidence"].split(":", 1)[1] for u in fp_monster_pseudo_class)
    for s, c in fpA_by_slug.most_common():
        print(f"  {c:4d}  {s}  (bestiary pseudo-class, not a PC/NPC class)")
    print(f"  TOTAL class-A false positives: {len(fp_monster_pseudo_class)}")

    print("\n--- False positive class B: type_facet's own '<Class> Class "
          "Feature' marker names an ALREADY-MODELLED class, contradicting "
          "the evidence-reported (unmodelled) slug ---")
    fpB_by_pair = Counter(
        (u["evidence"].split(":", 1)[1], u["_true_owner"]) for u in fp_modelled_owner
    )
    for (reported, true_owner), c in fpB_by_pair.most_common():
        print(f"  {c:4d}  reported={reported!r:22s} true_owner(modelled)={true_owner}")
    print(f"  TOTAL class-B false positives: {len(fp_modelled_owner)}")

    total_fp = len(fp_monster_pseudo_class) + len(fp_modelled_owner)
    print(f"\nTOTAL false positives (A + B, no double count -- A checked "
          f"first): {total_fp}")

    real_total = len(t12) - total_fp
    print(f"\nConfirmed real T12 population (evidence-code total minus "
          f"false positives): {real_total}")

    # Regroup the real (false-positive-excluded) population by TRUE owning
    # class -- not the evidence-reported slug, which several groups above
    # proved unreliable (e.g. "Magic Warrior"/"Crystal Warrior"/"Feral
    # Warrior"/"Warrior Path 1/2" all evidence-report as "warrior" but their
    # type_facet's own class marker names Magus / Aegis / Psychic Warrior).
    # This time the marker is matched against EVERY corpus-declared class
    # name (all `kind=class` records minus the Monster pseudo-classes), not
    # only the modelled 34, so an unmodelled owner is recovered too.
    all_corpus_classes = sorted(
        (u["name"] for u in units
         if u.get("kind") == "class" and u.get("type_facet") != "Monster"),
        key=len, reverse=True,
    )

    def true_owner_any(type_facet: str | None) -> str | None:
        if not type_facet:
            return None
        for segment in type_facet.split("."):
            owner = _segment_owner(segment, all_corpus_classes)
            if owner is not None:
                return owner
        return None

    real_by_true_class: Counter[str] = Counter()
    real_unresolved = 0
    for u in t12:
        reported_slug = u["evidence"].split(":", 1)[1]
        if reported_slug in monster_pseudo_classes:
            continue
        if true_owner_from_type_facet(u.get("type_facet")) is not None:
            continue
        owner = true_owner_any(u.get("type_facet"))
        if owner is not None:
            real_by_true_class[owner] += 1
        else:
            real_by_true_class[reported_slug.replace("_", " ").title()] += 1
            real_unresolved += 1

    print("\n--- Real T12 population, regrouped by TRUE owning class "
          "(type_facet-derived where available, evidence slug as fallback) ---")
    for s, c in real_by_true_class.most_common():
        cr = class_records.get(s)
        book = cr["book"] if cr else "?"
        print(f"  {c:4d}  {s:28s} book~{book}")
    print(f"\n  Distinct real unmodelled classes (work-dispatch units): "
          f"{len(real_by_true_class)}")
    print(f"  Units whose type_facet carried no class marker at all "
          f"(fell back to evidence slug, unresolved): {real_unresolved}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
