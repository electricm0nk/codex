#!/usr/bin/env python3
"""Ad hoc probe (t2b remeasure cycle, read-only): for the 'other' sub-bucket of
several large T2b books, how many rows' KEY prefix corresponds to ANY entry
(CR-bearing or not) in that book's own *_races*.lst file(s), vs. rows whose
KEY prefix never appears in a races file at all (a signal of further,
still-uncaught monster/template noise the refine_kind KEY-prefix fix's
same-book CR-bearing-name rule cannot reach because the name isn't in a
races file at all).

Not wired into any test; a measurement-cycle scratch tool, committed for
re-derivability per decisions.md §12c.
"""
import json
import os
import glob

ROOT = os.environ["PCGEN_CORPUS_ROOT"]

BOOK_DIRS = {
    "bestiary_2": "pathfinder/paizo/roleplaying_game/bestiary_2",
    "bestiary": "pathfinder/paizo/roleplaying_game/bestiary",
    "bestiary_3": "pathfinder/paizo/roleplaying_game/bestiary_3",
    "bestiary_4": "pathfinder/paizo/roleplaying_game/bestiary_4",
    "bestiary_5": "pathfinder/paizo/roleplaying_game/bestiary_5",
    "bestiary_6": "pathfinder/paizo/roleplaying_game/bestiary_6",
    "mythic_adventures": "pathfinder/paizo/roleplaying_game/mythic_adventures",
    "pathfinder_unchained": "pathfinder/paizo/roleplaying_game/pathfinder_unchained",
    "occult_adventures": "pathfinder/paizo/roleplaying_game/occult_adventures",
    "core_rulebook": "pathfinder/paizo/roleplaying_game/core_rulebook",
    "advanced_race_guide": "pathfinder/paizo/roleplaying_game/advanced_race_guide",
    "advanced_players_guide": "pathfinder/paizo/roleplaying_game/advanced_players_guide",
    "advanced_class_guide": "pathfinder/paizo/roleplaying_game/advanced_class_guide",
    "inner_sea_races": "pathfinder/campaign_setting/inner_sea_races",
    "inner_sea_world_guide": "pathfinder/campaign_setting/inner_sea_world_guide",
    "inner_sea_gods": "pathfinder/campaign_setting/inner_sea_gods",
    "inner_sea_bestiary": "pathfinder/campaign_setting/inner_sea_bestiary",
    "ultimate_wilderness": "pathfinder/roleplaying_game/ultimate_wilderness",
    "ultimate_psionics": "pathfinder/dreamscarred_press/ultimate_psionics",
    "ultimate_combat": "pathfinder/paizo/roleplaying_game/ultimate_combat",
    "ultimate_intrigue": "pathfinder/paizo/roleplaying_game/ultimate_intrigue",
    "ultimate_magic": "pathfinder/paizo/roleplaying_game/ultimate_magic",
    "monster_codex": "pathfinder/paizo/roleplaying_game/monster_codex",
    "horror_adventures": "pathfinder/paizo/roleplaying_game/horror_adventures",
    "book_of_the_damned_volume_1": "pathfinder/paizo/roleplaying_game/book_of_the_damned_volume_1",
    "book_of_the_damned_volume_2": "pathfinder/paizo/roleplaying_game/book_of_the_damned_volume_2",
}


def classify(ck):
    if ck.startswith("Adopted Race ~ "):
        return "adopted_race"
    for p in (
        "Racial SLA ~ ",
        "Unchained Evolution ~ ",
        "Favored Class Bonus ~ ",
        "Race Subtype ~ ",
    ):
        if ck.startswith(p):
            return "header"
    return "other"


def find_book_dir(book):
    """Fall back to a corpus-wide directory-name search when the hardcoded
    relative path is wrong -- more robust than guessing the exact publisher
    subtree layout for every one of the 26 books."""
    matches = []
    for dirpath, dirs, files in os.walk(ROOT):
        if os.path.basename(dirpath) == book:
            matches.append(dirpath)
    return matches


def main():
    d = json.load(open("docs/work-inventory.json"))
    u = [
        x
        for x in d["units"]
        if x.get("kind") == "race_trait"
        and x.get("evidence") == "race_trait_race_not_modelled"
    ]

    for book, reldir in BOOK_DIRS.items():
        bdir = os.path.join(ROOT, reldir)
        candidate_dirs = [bdir] if os.path.isdir(bdir) else find_book_dir(book)
        race_names = set()
        for bdir2 in candidate_dirs:
          for fn in glob.glob(os.path.join(bdir2, "*races*.lst")):
            with open(fn, encoding="utf-8", errors="replace") as fh:
                for line in fh:
                    if line.strip() and not line.startswith("#"):
                        name = line.split("\t")[0].strip()
                        if name:
                            race_names.add(name)
        others = [x for x in u if x["book"] == book and classify(x.get("corpus_key") or x.get("name") or "") == "other"]
        in_races_file = 0
        not_in = 0
        not_in_examples = []
        for x in others:
            ck = x.get("corpus_key") or x.get("name") or ""
            key = ck.split(" ~ ")[0] if " ~ " in ck else ck
            if key in race_names:
                in_races_file += 1
            else:
                not_in += 1
                if len(not_in_examples) < 8:
                    not_in_examples.append(key)
        print(
            f"{book:24s} other={len(others):4d}  key-in-races-file={in_races_file:4d}  "
            f"key-NOT-in-any-races-file={not_in:4d}  (races-file-names-found={len(race_names)})"
        )
        if not_in_examples:
            print("   not-in examples:", not_in_examples)


if __name__ == "__main__":
    main()
