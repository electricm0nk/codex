#!/usr/bin/env python3
"""scripts/supersession_register_gate.py -- SD31-D10-REGISTER-001, the
Supersession Register's own gate (`decisions.md` Decision 10 + its
2026-08-16 amendment, direction CORRECTED by Decision 13, 2026-08-17: for
an identical pair the FIRST print owns it, not the newest).

Decision 10 is the FIRST authorization in this package to shrink the
mandate denominator, and a standing rule a cycle may apply without a
per-entry operator signature (unlike the Structural Exclusion Register,
`decisions.md §3`) -- which makes THIS gate, not a signature, the only thing
protecting the number it touches. It must be able to fail, proven by
mutation test (`scripts/tests/test_supersession_register_gate.py`), before
it is trusted, per the standing "gates that could not fail" lesson
(`SD-30 state-goals-and-lessons.md §3.1`).

THE TWO REFUSALS
-----------------
1. **Material-difference refusal.** For every `objects[]` entry, re-derive
   each side's raw `.lst` row from the pinned oracle (never trust the
   register's own cached `raw_lines`) and confirm they are still IDENTICAL
   after stripping provenance/pricing fields (SOURCE*, COST, OUTPUTNAME,
   KEY, NAMEISPI) and normalizing `TYPE:` as an order-insensitive tag set --
   the exact bar `supersession_register_build.py` used to admit the entry
   in the first place. A drifted corpus, a hand-edited entry, or a future
   cycle appending an unverified pair all fail here.
2. **Variant-line refusal.** No `objects[]` entry may name
   `pathfinder_unchained` or `mythic_adventures` as either the surviving or
   a superseded book unless the entry itself carries a non-empty
   `reprint_proof` string. The default for both lines is VARIANT (a new
   object, never a reprint) -- silence is a refusal, not an allow.

Also checked (structural, cheap, no oracle needed):
  * every `surviving.source_date` is >= every one of its own
    `superseded[].source_date` (string-lexicographic `YYYY-MM` compare,
    which is date-order-correct for this corpus's SOURCEDATE format) --
    "newest wins" the wrong way round is refused;
  * no entry names `book == "core_essentials"` on either side (Decision 9:
    it is not a book and does not belong in this register).

Run: `python3 scripts/supersession_register_gate.py`
Wired as the `supersession-gate` stage in `scripts/verify.sh`, immediately
after `corpus-sweep` (same oracle dependency).
"""
from __future__ import annotations

import argparse
import json
import os
import sys

VARIANT_BOOKS = {"pathfinder_unchained", "mythic_adventures"}
IGNORE_FIELD_PREFIXES = ("SOURCE", "COST", "OUTPUTNAME", "KEY", "NAMEISPI")

DEFAULT_REGISTER = os.path.join(
    os.path.dirname(os.path.abspath(__file__)), "..",
    "docs", "release", "SD-31-corpus-closure-grind", "artifacts",
    "SUPERSESSION-REGISTER.json",
)


def fields_of(line: str | None) -> dict[str, set[str]] | None:
    if line is None:
        return None
    out: dict[str, set[str]] = {}
    for tok in line.split("\t"):
        if not tok.strip():
            continue
        if ":" in tok:
            k, _, v = tok.partition(":")
            k = k.strip()
            if any(k == p or k.startswith(p) for p in IGNORE_FIELD_PREFIXES):
                continue
            if k == "TYPE":
                out.setdefault(k, set()).update(v.split("."))
            else:
                out.setdefault(k, set()).add(v)
        else:
            out.setdefault("_bare", set()).add(tok.strip())
    return out


class FileFinder:
    """Resolves book -> {basename: absolute path}, cached per book, over a
    corpus root. Injectable so the self-test can point at a hermetic fake
    tree instead of the real pinned oracle."""

    BOOK_DIRS = {
        "advanced_class_guide": "pathfinder/paizo/roleplaying_game/advanced_class_guide",
        "advanced_players_guide": "pathfinder/paizo/roleplaying_game/advanced_players_guide",
        "advanced_race_guide": "pathfinder/paizo/roleplaying_game/advanced_race_guide",
        "adventurers_guide": "pathfinder/paizo/roleplaying_game/adventurers_guide",
        "beginner_box": "pathfinder/paizo/roleplaying_game/beginner_box",
        "bestiary": "pathfinder/paizo/roleplaying_game/bestiary",
        "bestiary_2": "pathfinder/paizo/roleplaying_game/bestiary_2",
        "bestiary_3": "pathfinder/paizo/roleplaying_game/bestiary_3",
        "bestiary_4": "pathfinder/paizo/roleplaying_game/bestiary_4",
        "bestiary_5": "pathfinder/paizo/roleplaying_game/bestiary_5",
        "bestiary_6": "pathfinder/paizo/roleplaying_game/bestiary_6",
        "bonus_bestiary": "pathfinder/paizo/roleplaying_game/bonus_bestiary",
        "book_of_the_damned_volume_1": "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_1",
        "book_of_the_damned_volume_2": "pathfinder/paizo/campaign_setting/book_of_the_damned_volume_2",
        "core_essentials": "pathfinder/paizo/roleplaying_game/core_essentials",
        "core_rulebook": "pathfinder/paizo/roleplaying_game/core_rulebook",
        "horror_adventures": "pathfinder/paizo/roleplaying_game/horror_adventures",
        "inner_sea_bestiary": "pathfinder/paizo/campaign_setting/inner_sea_bestiary",
        "inner_sea_combat": "pathfinder/paizo/campaign_setting/inner_sea_combat",
        "inner_sea_faiths": "pathfinder/paizo/campaign_setting/inner_sea_faiths",
        "inner_sea_gods": "pathfinder/paizo/campaign_setting/inner_sea_gods",
        "inner_sea_intrigue": "pathfinder/paizo/campaign_setting/inner_sea_intrigue",
        "inner_sea_magic": "pathfinder/paizo/campaign_setting/inner_sea_magic",
        "inner_sea_races": "pathfinder/paizo/campaign_setting/inner_sea_races",
        "inner_sea_taverns": "pathfinder/paizo/campaign_setting/inner_sea_taverns",
        "inner_sea_temples": "pathfinder/paizo/campaign_setting/inner_sea_temples",
        "inner_sea_world_guide": "pathfinder/paizo/campaign_setting/inner_sea_world_guide",
        "monster_codex": "pathfinder/paizo/roleplaying_game/monster_codex",
        "mythic_adventures": "pathfinder/paizo/roleplaying_game/mythic_adventures",
        "occult_adventures": "pathfinder/paizo/roleplaying_game/occult_adventures",
        "pathfinder_unchained": "pathfinder/paizo/roleplaying_game/pathfinder_unchained",
        "ultimate_campaign": "pathfinder/paizo/roleplaying_game/ultimate_campaign",
        "ultimate_combat": "pathfinder/paizo/roleplaying_game/ultimate_combat",
        "ultimate_equipment": "pathfinder/paizo/roleplaying_game/ultimate_equipment",
        "ultimate_intrigue": "pathfinder/paizo/roleplaying_game/ultimate_intrigue",
        "ultimate_magic": "pathfinder/paizo/roleplaying_game/ultimate_magic",
        "ultimate_psionics": "pathfinder/dreamscarred_press/ultimate_psionics",
        "ultimate_wilderness": "pathfinder/paizo/roleplaying_game/ultimate_wilderness",
    }

    def __init__(self, root: str, book_dirs: dict[str, str] | None = None):
        self.root = root
        self.book_dirs = book_dirs if book_dirs is not None else self.BOOK_DIRS
        self._cache: dict[str, dict[str, str]] = {}

    def raw_line(self, book: str, source_file: str, source_line: int) -> str | None:
        if book not in self._cache:
            idx: dict[str, str] = {}
            rel = self.book_dirs.get(book)
            d = os.path.join(self.root, rel) if rel else None
            if d and os.path.isdir(d):
                for dirpath, _dirs, filenames in os.walk(d):
                    for fn in filenames:
                        if fn.endswith(".lst"):
                            idx.setdefault(fn, os.path.join(dirpath, fn))
            self._cache[book] = idx
        path = self._cache[book].get(source_file)
        if not path:
            return None
        with open(path, errors="replace") as fh:
            lines = fh.readlines()
        if source_line < 1 or source_line > len(lines):
            return None
        return lines[source_line - 1].rstrip("\n")


def validate_entry(entry: dict, finder: FileFinder | None) -> list[str]:
    """Returns a list of violation strings; empty means the entry passes."""
    violations: list[str] = []
    surviving = entry.get("surviving", {})
    superseded = entry.get("superseded", [])
    all_sides = [surviving] + list(superseded)

    # -- degenerate corpus_key: a bare integer is a PCGen level-number
    # continuation row, never an object identity (the `companion` "1"
    # defect: two DIFFERENT class-continuation rows that happen to share a
    # level number are not the same object, and the material-difference
    # check below cannot tell them apart when their shared key IS the
    # thing being compared).
    corpus_key = entry.get("corpus_key")
    if isinstance(corpus_key, str) and corpus_key.strip().isdigit():
        violations.append(
            f"{entry.get('kind')}:{corpus_key}: corpus_key is a bare integer -- "
            f"a level-number continuation row, not an object identity; refused"
        )

    # -- core_essentials never belongs here (Decision 9) -------------------
    for side in all_sides:
        if side.get("book") == "core_essentials":
            violations.append(
                f"{entry.get('kind')}:{entry.get('corpus_key')}: "
                f"names book=core_essentials ({side.get('id')}) -- Decision 9: "
                f"core_essentials is not a book and does not belong in this register"
            )

    # -- REFUSAL 2: variant-line guard --------------------------------------
    reprint_proof = (entry.get("reprint_proof") or "").strip()
    for side in all_sides:
        if side.get("book") in VARIANT_BOOKS and not reprint_proof:
            violations.append(
                f"{entry.get('kind')}:{entry.get('corpus_key')}: names variant-line "
                f"book '{side.get('book')}' ({side.get('id')}) with no `reprint_proof` "
                f"-- default for pathfinder_unchained/mythic_adventures is VARIANT, "
                f"never a reprint, without record-level proof"
            )

    # -- SOURCEDATE ordering: FIRST print owns it (Decision 13, 2026-08-17) -
    # For an IDENTICAL pair the FIRST printing is `surviving`; a later
    # printing is `superseded`. (Decision 10's original "newest wins"
    # direction was corrected by Decision 13 -- see decisions.md §13.)
    sdate = surviving.get("source_date")
    for s in superseded:
        if sdate is not None and s.get("source_date") is not None and s["source_date"] < sdate:
            violations.append(
                f"{entry.get('kind')}:{entry.get('corpus_key')}: surviving "
                f"{surviving.get('book')} ({sdate}) is NEWER than superseded "
                f"{s.get('book')} ({s['source_date']}) -- Decision 13: for an "
                f"identical pair the FIRST print owns it, the later printing is superseded"
            )

    # -- REFUSAL 1: material-difference guard (re-derived from the oracle) -
    #
    # No fallback to the register's own cached `raw_lines`. A side with no
    # `source_file`/`source_line`, or whose citation the oracle cannot
    # resolve, is a HARD violation -- never a silent pass. (Prior shape:
    # falling back to the cached `raw_lines` made this branch entirely
    # dead code, since no shipped entry ever carried `source_file`/
    # `source_line` on its sides -- proven by mutation test: a fabricated
    # entry with no evidence at all, or with its raw_lines swapped for
    # nonsense, passed clean. See `scripts/tests/test_supersession_register_gate.py`.)
    if finder is not None:
        rederived: dict[str, str | None] = {}
        for side in all_sides:
            book = side.get("book")
            sf = side.get("source_file")
            sl = side.get("source_line")
            if sf is None or sl is None:
                violations.append(
                    f"{entry.get('kind')}:{entry.get('corpus_key')}: {book} "
                    f"({side.get('id')}) carries no source_file/source_line to "
                    f"re-derive from the oracle -- an entry with no re-derivable "
                    f"evidence is refused, never trusted on its cached raw_lines alone"
                )
                rederived[book] = None
                continue
            line = finder.raw_line(book, sf, sl)
            if line is None:
                violations.append(
                    f"{entry.get('kind')}:{entry.get('corpus_key')}: {book} "
                    f"({side.get('id')}) citation {sf}:{sl} could not be re-derived "
                    f"from the pinned oracle (unknown book, missing file, or line out "
                    f"of range)"
                )
            rederived[book] = line
        parsed = {b: fields_of(line) for b, line in rederived.items()}
        base_book = surviving.get("book")
        base = parsed.get(base_book)
        for side in all_sides:
            b = side.get("book")
            if b == base_book:
                continue
            if parsed.get(b) != base:
                violations.append(
                    f"{entry.get('kind')}:{entry.get('corpus_key')}: {b} ({side.get('id')}) "
                    f"and {base_book} ({surviving.get('id')}) do NOT carry identical "
                    f"mechanical fields once re-derived from the oracle -- material "
                    f"difference, not a duplicate"
                )

    return violations


def validate_register(register: dict, finder: FileFinder | None) -> dict:
    violations: list[str] = []
    for entry in register.get("objects", []):
        violations.extend(validate_entry(entry, finder))

    proposed_removed = register.get("denominator", {}).get("count_removed")
    actual_removed = sum(len(e.get("superseded", [])) for e in register.get("objects", []))
    if proposed_removed is not None and proposed_removed != actual_removed:
        violations.append(
            f"denominator.count_removed ({proposed_removed}) does not match the "
            f"register's own objects[].superseded tally ({actual_removed})"
        )

    return {"ok": len(violations) == 0, "violations": violations,
            "objects_checked": len(register.get("objects", []))}


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__,
                                      formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--register", default=DEFAULT_REGISTER)
    parser.add_argument(
        "--corpus-root",
        default=os.environ.get("PCGEN_CORPUS_ROOT", os.path.expanduser("~/workspace/repos/pcgen/data")),
    )
    parser.add_argument("--no-corpus-root", action="store_true",
                         help="skip the oracle-backed material-difference re-derivation (structural checks only)")
    parser.add_argument("--json-out")
    args = parser.parse_args(argv)

    if not os.path.isfile(args.register):
        print(f"FAIL: register not found at {args.register}")
        return 1

    register = json.load(open(args.register))
    if args.no_corpus_root:
        finder = None
        print("WARNING: --no-corpus-root -- skipping the material-difference "
              "re-derivation (structural checks only)")
    elif not os.path.isdir(args.corpus_root):
        print(f"FAIL: corpus root not found at {args.corpus_root} "
              f"(bootstrap via scripts/fetch-pcgen-oracle.sh, or pass --no-corpus-root "
              f"to run structural checks only)")
        return 1
    else:
        finder = FileFinder(args.corpus_root)

    result = validate_register(register, finder)

    print(f"supersession_register_gate: {result['objects_checked']} objects checked")
    if result["violations"]:
        print(f"  FAIL: {len(result['violations'])} violation(s):")
        for v in result["violations"]:
            print(f"    - {v}")
    else:
        print("  OK: every entry proves same-object field equality and clears both guards")

    if args.json_out:
        json.dump(result, open(args.json_out, "w"), indent=2)

    return 0 if result["ok"] else 1


if __name__ == "__main__":
    sys.exit(main())
