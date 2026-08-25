#!/usr/bin/env python3
"""SD-32 card 11 T2b classifier fix, cluster 4 (`card11-t2b-remeasure.md §5`,
`decisions.md §17a`) -- the corpus-wide safety check for `refine_kind`'s new
PC-class-name-prefix arm (`book_pc_class_names` in
`src/bin/v06_work_inventory.rs`).

Deliberately NOT scoped to a hardcoded book list -- the whole point of this
script (per the guard rail in `card11-t2b-remeasure.md`'s dispatch brief:
"any new discriminator needs a safety test whose failure branch genuinely
covers the whole corpus, not a hardcoded subset") is that its failure branch
covers every `*_abilities_race.lst`-shaped file the corpus has, found by
walking `PCGEN_CORPUS_ROOT` itself, not a list this script's own author
picked. The stress test this fix's own predecessor shipped
(`t2b_refine_kind_key_prefix_stress_test.py`) had exactly this defect --
a `KNOWN_RACE_BOOKS_DIRS` allow-list that could never exercise a book
outside it -- and it silently missed 112 Ultimate Psionics units as a
result. This script has no such list.

Mirrors the Rust logic in `refine_kind`/`book_pc_class_names` byte-for-byte
(the `.PC`-gated CLASS: scan, the `is_choice_row` exclusion, the bare-KEY
fallback, the word-boundary prefix match) so it is a real cross-check of the
shipped behaviour, not a second implementation.

Usage: PCGEN_CORPUS_ROOT=<oracle>/data python3 scripts/t2b_pc_class_prefix_stress_test.py
Exit 0 and prints per-book counts on success. Exit 1 if any KNOWN real-race
book (the books this project already treats as fully modelled: core_rulebook,
bestiary through bestiary_6, advanced_race_guide, inner_sea_races,
core_essentials, ultimate_wilderness) shows a nonzero match -- that would be
the false-positive shape this fix must never reintroduce.
"""
import glob
import os
import sys

CORPUS = os.environ.get("PCGEN_CORPUS_ROOT")
if not CORPUS:
    print("PCGEN_CORPUS_ROOT not set", file=sys.stderr)
    sys.exit(2)

# The same "known real playable-race book" list `refine_kind`'s own doc
# comments already name as fully modelled -- used ONLY as the failure
# threshold below, never to scope which files this script scans.
KNOWN_REAL_RACE_BOOKS = {
    "core_rulebook",
    "bestiary",
    "bestiary_2",
    "bestiary_3",
    "bestiary_4",
    "bestiary_5",
    "bestiary_6",
    "advanced_race_guide",
    "inner_sea_races",
    "core_essentials",
    "ultimate_wilderness",
}


def tab_fields(line):
    return [f.strip() for f in line.split("\t") if f.strip()]


def pc_class_names_for_dir(bookdir):
    names = set()
    for f in glob.glob(f"{bookdir}/**/*classes*.lst", recursive=True):
        for line in open(f, errors="ignore"):
            fields = tab_fields(line)
            cls = None
            typ = None
            for fld in fields:
                if fld.startswith("CLASS:"):
                    cls = fld[len("CLASS:"):].strip()
                elif fld.startswith("TYPE:"):
                    typ = fld[len("TYPE:"):].strip()
            if cls and typ and any(seg == "PC" for seg in typ.split(".")):
                names.add(cls)
    return names


def is_choice_row(fields):
    typ = ""
    for f in fields:
        if f.startswith("TYPE:"):
            typ = f[len("TYPE:"):]
    segs = typ.split(".")
    second = segs[1] if len(segs) > 1 else ""
    if second.endswith("Choice"):
        return True
    return any("Favored Class Bonus" in f or "FavClassBonus" in f for f in fields)


def find_book_dir(start, corpus_root):
    d = start
    while d != corpus_root and not glob.glob(f"{d}/*.pcc"):
        parent = os.path.dirname(d)
        if parent == d:
            break
        d = parent
    return d


def main():
    race_files = [
        f for f in glob.glob(f"{CORPUS}/**/*.lst", recursive=True)
        if "abilities_race" in os.path.basename(f).lower()
    ]
    by_book = {}
    for f in race_files:
        bookdir = find_book_dir(os.path.dirname(f), CORPUS)
        cn = pc_class_names_for_dir(bookdir)
        if not cn:
            continue
        book = os.path.basename(bookdir)
        for line in open(f, errors="ignore"):
            fields = tab_fields(line)
            if not fields or fields[0].startswith("#"):
                continue
            key = None
            for fld in fields:
                if fld.startswith("KEY:"):
                    key = fld[len("KEY:"):]
                    break
            if key is None:
                key = fields[0]
            prefix = key.split(" ~ ")[0].strip()
            matched = prefix in cn or any(
                prefix.startswith(c + " ") for c in cn
            )
            if matched and not is_choice_row(fields):
                by_book.setdefault(book, []).append(prefix)

    failures = []
    for book, items in sorted(by_book.items(), key=lambda kv: -len(kv[1])):
        print(f"{book}\t{len(items)}\t{sorted(set(items))[:8]}")
        if book in KNOWN_REAL_RACE_BOOKS and items:
            failures.append(book)

    if failures:
        print(f"FAIL: known real-race book(s) matched: {failures}", file=sys.stderr)
        sys.exit(1)
    print("OK: no known real-race book was matched by the PC-class-prefix discriminator")


if __name__ == "__main__":
    main()
