#!/usr/bin/env python3
"""SD-32 card 11, shape T2b -- census and per-row proof for the "Adoptive
Parentage" / "Adopted Race" selector population `decisions.md §16` item 2
names (wave-1 receipts: `epic-2-t2b-bestiary2_cycle-1_cycle_receipt.md`,
`epic-2-t2b-bestiary6_cycle-1_cycle_receipt.md`, `t2b-bestiary_3-measurement-
receipt.md`, `epic-2-t2b-w1b-bestiary_5_cycle_receipt.md`,
`epic-2-t2b-w1-c_cycle-1_cycle_receipt.md`).

Two things this script re-derives that no prior receipt re-derived together:

1. **The 21-unit population**, by class, from `docs/work-inventory.json`
   directly (not from a book/count table transcribed by hand). This is
   exactly the union of `corpus_key` starting `"Adopted Race ~ "` (14 units:
   bestiary_2 7, bestiary_3 5, bestiary_5 1, bestiary_6 1) and the
   `advanced_race_guide` book's 7 bare-race-name units at
   `arg_abilities_race.lst:291-297` (`###Block: Adoptive Parentage
   Options`).

2. **A corpus-wide (not same-file) proof of real-vs-empty content behind
   each row's CHOOSE pool.** `epic-2-t2b-bestiary6_cycle-1_cycle_receipt.md`
   and `epic-2-t2b-bestiary2_cycle-1_cycle_receipt.md` both concluded all 8
   `Adopted Race ~ <X>` rows in those two books are "the identical
   browse-only-stub shape" as Rougarou, by grepping each row's OWN file
   only. **That finding is corrected here**: grepping the WHOLE pinned
   oracle for `"<Race> Race Trait"` (the exact CHOOSE-pool TYPE target,
   distinct from the "<Race> Racial Trait"/"<Race> Racial Default" vocabulary
   standard chassis traits use) finds real content elsewhere in the oracle
   for every one of the 13 CHOOSE-selector rows except Rougarou -- 1 file
   (itself) for Rougarou, 2+ files for all 13 others. Rougarou remains the
   one proven-empty row; the other 13 are proven NOT empty, just
   not-yet-ingestable without a new "PF1e Trait" content kind this project
   has never modelled (escalated, not fabricated -- `decisions.md §1a`/§3`).

   The other shape -- ARG's 7 flat `ABILITY:...AUTOMATIC|<Race> ~ Weapon
   Familiarity|<Race> ~ Languages` grants -- resolves to content already
   ingested in THIS project's own corpus (not the wider oracle), so those 7
   are closed by `src/bin/ingest_race_traits.rs`'s new Adoptive Parentage
   branch and `codex::rules_core::race_resolver::adoptive_parentage_options`.

Run: python3 scripts/t2b_adoptive_parentage_census.py
Requires PCGEN_CORPUS_ROOT (the repo-local pinned oracle's data/ root) for
the corpus-wide proof step; the work-inventory census step runs without it.
"""
from __future__ import annotations

import json
import os
import subprocess
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
WORK_INVENTORY = REPO_ROOT / "docs" / "work-inventory.json"

# The exact 5 books `decisions.md §16` item 2 names for THIS cycle's scope.
# `Adopted Race ~ <X>` selector rows exist in several other books too (11 more
# in `bestiary`, `bestiary_4`, `core_rulebook`, `inner_sea_world_guide`,
# `ultimate_wilderness` -- 44 corpus-wide) but those are out of this cycle's
# granted scope; scoping the census to these 5 keeps this script's count
# matching the dispatch brief's 21, not the corpus-wide 58.
IN_SCOPE_BOOKS = {"bestiary_2", "advanced_race_guide", "bestiary_3", "bestiary_5", "bestiary_6"}
ADOPTED_RACE_PREFIX = "Adopted Race ~ "
ARG_ADOPTIVE_PARENTAGE_RACES = {"Drow", "Dwarf", "Elf", "Gnome", "Halfling", "Orc", "Grippli"}


def load_population() -> list[dict]:
    data = json.loads(WORK_INVENTORY.read_text())
    units = data["units"]
    out = []
    for u in units:
        if u.get("kind") != "race_trait":
            continue
        book = u.get("book")
        if book not in IN_SCOPE_BOOKS:
            continue
        key = str(u.get("corpus_key", ""))
        if key.startswith(ADOPTED_RACE_PREFIX):
            out.append(
                {
                    "shape": "adopted_race_choose_selector",
                    "book": book,
                    "corpus_key": key,
                    "adopted_race": key[len(ADOPTED_RACE_PREFIX):],
                    "source_file": u.get("source_file"),
                    "source_line": u.get("source_line"),
                }
            )
        elif book == "advanced_race_guide" and key in ARG_ADOPTIVE_PARENTAGE_RACES:
            out.append(
                {
                    "shape": "arg_flat_grant",
                    "book": book,
                    "corpus_key": key,
                    "adopted_race": key,
                    "source_file": u.get("source_file"),
                    "source_line": u.get("source_line"),
                }
            )
    out.sort(key=lambda r: (r["book"], r["adopted_race"]))
    return out


def oracle_root() -> Path | None:
    root = os.environ.get("PCGEN_CORPUS_ROOT")
    if not root:
        return None
    p = Path(root)
    return p if p.is_dir() else None


def corpus_wide_pool_file_count(root: Path, race: str) -> int:
    """Files across the WHOLE pinned oracle whose text contains
    `"<Race> Race Trait"` verbatim -- the CHOOSE pool's exact TYPE target
    (`TYPE=<Race> Race Trait`), distinct from `<Race> Racial Trait`/`<Race>
    Racial Default` (the standard chassis vocabulary). Includes the row's
    own file (which always matches, since the row states its own pool's
    placeholder there too), so a count of exactly 1 means "nowhere else in
    the whole oracle" -- proven empty. A count of 2+ means real content
    exists somewhere else in the oracle, even if this project has not
    ingested it.
    """
    needle = f"{race} Race Trait"
    proc = subprocess.run(
        ["grep", "-rl", "-F", needle, str(root)],
        capture_output=True,
        text=True,
        check=False,
    )
    files = [line for line in proc.stdout.splitlines() if line.strip()]
    return len(files)


def main() -> int:
    population = load_population()
    by_shape: dict[str, list[dict]] = {}
    for row in population:
        by_shape.setdefault(row["shape"], []).append(row)

    print(f"# T2b Adoptive Parentage census -- {WORK_INVENTORY.relative_to(REPO_ROOT)}")
    print(f"total population: {len(population)}  (expect 21)")
    print()
    by_book: dict[str, int] = {}
    for row in population:
        by_book[row["book"]] = by_book.get(row["book"], 0) + 1
    for book in sorted(by_book):
        print(f"  {book}: {by_book[book]}")
    print()
    print(f"shape 'adopted_race_choose_selector' (CHOOSE:ABILITYSELECTION pool): "
          f"{len(by_shape.get('adopted_race_choose_selector', []))}  (expect 14)")
    print(f"shape 'arg_flat_grant' (ABILITY:...AUTOMATIC flat grant): "
          f"{len(by_shape.get('arg_flat_grant', []))}  (expect 7)")
    print()

    root = oracle_root()
    if root is None:
        print("PCGEN_CORPUS_ROOT not set or not a directory -- skipping the corpus-wide "
              "real-vs-empty proof. Population census above still stands.")
        return 0

    print(f"# Corpus-wide real-vs-empty proof, oracle root {root}")
    print(f"# command per race: grep -rl -F '<Race> Race Trait' <oracle root> | wc -l")
    print()
    proven_empty = []
    proven_real = []
    for row in by_shape.get("adopted_race_choose_selector", []):
        race = row["adopted_race"]
        n = corpus_wide_pool_file_count(root, race)
        verdict = "PROVEN EMPTY (only its own file)" if n <= 1 else f"REAL CONTENT ({n} files corpus-wide)"
        print(f"  {row['book']:<20} {race:<12} {n} file(s) -- {verdict}")
        (proven_empty if n <= 1 else proven_real).append(row)

    print()
    print(f"proven empty (n<=1 file corpus-wide): {len(proven_empty)}")
    for row in proven_empty:
        print(f"  - {row['book']}: {row['corpus_key']}")
    print(f"proven real content exists (n>=2 files corpus-wide), not yet ingestable "
          f"without a new PF1e Trait content kind: {len(proven_real)}")
    for row in proven_real:
        print(f"  - {row['book']}: {row['corpus_key']}")

    print()
    print(f"closed this cycle (arg_flat_grant, real content already in THIS project's "
          f"own corpus): {len(by_shape.get('arg_flat_grant', []))}")
    for row in by_shape.get("arg_flat_grant", []):
        print(f"  - {row['book']}: {row['corpus_key']}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
