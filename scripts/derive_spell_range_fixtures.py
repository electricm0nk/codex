#!/usr/bin/env python3
"""Derives `spell_range_entries` for `tests/fixtures/rules_core/derived-evaluator-fixtures.json`.

SD31-E6-F2-008. `SD31-E6-F10-002`'s own re-derivation of the `derived`+held `spell`
population (`OPEN-ISSUES.md` row 178) found `range_keyword` is 206 of the ~590 held
units (35%) -- the largest single un-built lever left in the `spell` lane -- and named
its shape: PF1's `RANGE:Close`/`Medium`/`Long` keywords are each a STANDARD, universal,
caster-level-linear formula, defined once by the PCGen game mode itself
(`system/gameModes/Pathfinder/miscinfo.lst`'s `SPELLRANGE:` rows -- part of this repo's
own oracle pin, `PCGEN_ORACLE_SPARSE_PATHS` already covers `system/gameModes/Pathfinder`),
not a per-spell literal. This script independently re-reads that SAME ruleset file (never
imports or calls the Rust `spell_range_formula` it feeds) to transcribe the three formulas,
then walks every candidate spell's own upstream `.lst` line to confirm its `RANGE:` token
names one of the three keywords verbatim.

**Independence, matching `derive_spell_caster_level_duration_fixtures.py`'s own guarantee**:
this generator reads ONLY the raw, pinned upstream PCGen bytes under `$PCGEN_CORPUS_ROOT`
(the game-mode file) and `$PCGEN_REPO_DIR`/`system/gameModes/Pathfinder/miscinfo.lst` (the
ruleset formulas) -- never `data/corpus/`, which the Rust evaluator
(`derived_evaluator_fixture_check::run_spell_range_bar_check`) reads at evaluation time and
must therefore stay independent of. It reads `docs/work-inventory.json` only for unit
identity and each candidate's (book, source `.lst` path, line) triple.

Usage: python3 scripts/derive_spell_range_fixtures.py [--limit N]
Writes the `spell_range_entries` array to stdout as JSON (caller merges it into the
fixture file).
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

WORK_INVENTORY_BOOK_TO_SHORT = {
    "core_rulebook": "CRB",
    "advanced_players_guide": "APG",
    "advanced_class_guide": "ACG",
    "advanced_race_guide": "ARG",
    "ultimate_intrigue": "UI",
    "ultimate_magic": "UM",
    "occult_adventures": "OA",
    "ultimate_combat": "UC",
    # SD-31 wave 20: widened 8 -> 10. Both books already carry a real
    # `data/corpus/<book>/spell/` cache (ISG since `SD31-E6-F10-001`; UW
    # since SD-31 wave-19's `ultimate_wilderness` lane +
    # `SD31-W19-INTEGRATE-001`'s row-324 follow-up, which fixed the
    # identical gap on the Rust read side -- `SPELL_CORPUS_BOOK_DIRS` /
    # `spell_book_corpus_dir_for_short_code` in
    # `derived_evaluator_fixture_check.rs`) but this generator's own book
    # dict was never widened to match, so no `derived`+held unit in either
    # book could ever become a fixture candidate regardless of data
    # completeness. See `scripts/tests/test_derive_spell_range_fixtures.py`'s
    # `test_inner_sea_gods_and_ultimate_wilderness_are_candidates`.
    "inner_sea_gods": "ISG",
    "ultimate_wilderness": "UW",
}

# The three keywords the ruleset states a caster-level-linear SPELLRANGE
# formula for. Anything else (`Personal`, `Touch`, a literal distance,
# `See text`, ...) is refused rather than guessed -- matching the Rust
# evaluator's own refusal shape.
KNOWN_KEYWORDS = ("Close", "Medium", "Long")

SPELLRANGE_LINE_RE = re.compile(
    r"^SPELLRANGE:(CLOSE|MEDIUM|LONG)\|floor\(CASTERLEVEL/(\d+)\)\*(\d+)\+(\d+)$"
    r"|^SPELLRANGE:(CLOSE|MEDIUM|LONG)\|\(CASTERLEVEL\*(\d+)\)\+(\d+)$"
)


def pcgen_corpus_root() -> str:
    root = os.environ.get("PCGEN_CORPUS_ROOT")
    if root:
        return root
    return os.path.join(os.path.expanduser("~"), "workspace", "repos", "pcgen", "data")


def pcgen_repo_dir() -> str:
    root = os.environ.get("PCGEN_REPO_DIR")
    if root:
        return root
    return os.path.join(os.path.expanduser("~"), "workspace", "repos", "pcgen")


def load_spellrange_formulas() -> dict[str, tuple[int, int, int]]:
    """Independently re-reads `system/gameModes/Pathfinder/miscinfo.lst`'s
    `SPELLRANGE:` rows and returns keyword -> (base_ft, rate_ft, per_levels).
    A standalone regex, never imports the Rust module this feeds."""
    path = os.path.join(
        pcgen_repo_dir(), "system", "gameModes", "Pathfinder", "miscinfo.lst"
    )
    with open(path, encoding="utf-8", errors="replace") as f:
        lines = f.read().splitlines()
    out: dict[str, tuple[int, int, int]] = {}
    for line in lines:
        line = line.strip()
        if not line.startswith("SPELLRANGE:"):
            continue
        m = SPELLRANGE_LINE_RE.match(line)
        if not m:
            continue
        if m.group(1):
            keyword, per_levels_s, rate_s, base_s = m.group(1), m.group(2), m.group(3), m.group(4)
            per_levels = int(per_levels_s)
        else:
            keyword, rate_s, base_s = m.group(5), m.group(6), m.group(7)
            per_levels = 1
        out[keyword.title()] = (int(base_s), int(rate_s), per_levels)
    return out


def range_field_from_raw_line(line: str) -> str | None:
    """Splits a raw PCGen `.lst` line on tabs and returns the `RANGE:`
    field's value verbatim, or None if the line carries no such field."""
    for field in line.split("\t"):
        if field.startswith("RANGE:"):
            return field[len("RANGE:"):]
    return None


# Statuses whose evidence a `fixture-verified` stamp supersedes. Restates
# `v06_work_inventory::apply_done_rung_stamps`'s own `matches!` arm, minus
# `text-complete` (a description-only record states no magnitude for this
# family's `RANGE:` formula to be the verification OF).
#
# `fixture-verified` is in the list for IDEMPOTENCE, and it is load-bearing:
# it is not a base status the classifier can produce, it is the STAMP this
# very fixture family causes `apply_done_rung_stamps` to write. A generator
# that excluded it would drop every already-covered unit out of its own
# candidate set on the next run and silently shrink the fixture -- exactly
# the stamp-loss hazard `v06_work_inventory`'s `--allow-stamp-loss` guard
# exists to refuse. Re-deriving over a stamped unit re-states the same
# independently-read upstream `RANGE:` token, so the entry reproduces
# byte-for-byte rather than disappearing. (SD31-W15; the same latent defect
# is fixed in the sibling DURATION generator in the same commit.)
STAMPABLE_STATUSES = ("ingested-magnitude", "grounded", "fixture-verified")


def is_candidate(unit: dict) -> bool:
    """Whether `unit` is eligible for a `spell_range_entries` fixture row.

    Deliberately does NOT consult `wiring_class_reason`. That field is
    `src/rules_core/wiring_class.rs::classify()`'s tie-break -- the
    LEXICOGRAPHICALLY SMALLEST `derived:` signal on the row
    (`sigs.iter().filter(|s| s.starts_with("derived:")).min()`) -- so a unit
    carrying both `derived:prose_expr` and `derived:range_keyword` always
    reports `prose_expr` (`p` < `r`) however plainly its own upstream row
    reads `RANGE:Close`. Selecting on it excluded 151 `derived`+held spell
    units in the eight ingested books whose upstream `RANGE:` token names one
    of the three keywords verbatim, on an alphabetical accident and nothing
    else (SD31-W15; the sibling DURATION generator never filtered on it,
    which is why the two families' coverage diverged).

    What DOES gate: the facts that decide whether a `fixture-verified` stamp
    could apply at all (`kind`/`wiring_class`/`status`, per
    `v06_work_inventory::apply_done_rung_stamps`) and whether the book has an
    ingest for `run_spell_range_bar_check` to evaluate against. The record's
    own `RANGE:` token is then read from the pinned upstream bytes by
    `upstream_range_value` below -- the real, per-record filter.
    """
    return (
        unit.get("kind") == "spell"
        and unit.get("wiring_class") == "derived"
        and unit.get("status") in STAMPABLE_STATUSES
        and unit.get("book") in WORK_INVENTORY_BOOK_TO_SHORT
    )


def upstream_lst_path(corpus_root: str, unit: dict) -> str | None:
    """The pinned upstream `.lst` this unit's provenance cites, or None."""
    source_file = unit.get("source_file")
    if not source_file:
        return None
    book = unit.get("book")
    path = os.path.join(
        corpus_root, "pathfinder", "paizo", "roleplaying_game", str(book), source_file
    )
    return path if os.path.isfile(path) else None


def upstream_range_value(corpus_root: str, unit: dict) -> str | None:
    """This unit's own `RANGE:` field value, read verbatim from the pinned
    upstream bytes at the exact `(source_file, source_line)` its provenance
    cites. Never from `data/corpus/`, which is what the Rust evaluator reads
    and must stay independent of."""
    path = upstream_lst_path(corpus_root, unit)
    if path is None:
        return None
    source_line = unit.get("source_line")
    if not source_line:
        return None
    with open(path, "r", encoding="utf-8", errors="replace") as f:
        lines = f.read().split("\n")
    line_no = int(source_line)
    if line_no < 1 or line_no > len(lines):
        return None
    value = range_field_from_raw_line(lines[line_no - 1])
    return None if value is None else value.strip()


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--limit", type=int, default=None)
    ap.add_argument(
        "--write",
        action="store_true",
        help=(
            "merge the derived entries into tests/fixtures/rules_core/"
            "derived-evaluator-fixtures.json's `spell_range_entries` in place, "
            "instead of printing them for a caller to paste. The fixture is a "
            "GENERATED artifact; hand-merging it is how a figure stops being "
            "re-derivable (SD-30 loop-instruction.md, \"generated, never "
            "hand-maintained\")."
        ),
    )
    ap.add_argument(
        "--work-inventory",
        default=os.path.join(REPO_ROOT, "docs", "work-inventory.json"),
    )
    args = ap.parse_args()

    formulas = load_spellrange_formulas()
    for kw in KNOWN_KEYWORDS:
        if kw not in formulas:
            print(f"# FATAL: ruleset file carries no SPELLRANGE row for {kw!r}", file=sys.stderr)
            return 1

    corpus_root = pcgen_corpus_root()
    with open(args.work_inventory) as f:
        inv = json.load(f)

    range_key_index_cache: dict[str, set[str]] = {}

    def keys_with_range_token(book: str) -> set[str]:
        if book in range_key_index_cache:
            return range_key_index_cache[book]
        found: set[str] = set()
        spell_dir = os.path.join(REPO_ROOT, "data", "corpus", book, "spell")
        for root, _dirs, files in os.walk(spell_dir):
            for fn in files:
                if not fn.endswith(".json"):
                    continue
                try:
                    with open(os.path.join(root, fn)) as jf:
                        rec = json.load(jf)
                except Exception:
                    continue
                key = rec.get("data", {}).get("key")
                if not key:
                    continue
                for t in rec.get("data", {}).get("raw_tokens", []):
                    if t.get("key") == "RANGE":
                        found.add(key)
                        break
        range_key_index_cache[book] = found
        return found

    candidates = [u for u in inv["units"] if is_candidate(u)]

    entries = []
    sha_cache: dict[str, str] = {}
    skipped_no_lst = 0
    skipped_no_range_field = 0
    skipped_not_known_keyword = 0
    skipped_unresolved_in_corpus_cache = 0

    for u in candidates:
        source_file = u.get("source_file")
        source_line = u.get("source_line")
        book = u["book"]
        if not source_file or not source_line:
            skipped_no_lst += 1
            continue
        rel_path = f"pathfinder/paizo/roleplaying_game/{book}/{source_file}"
        full_path = os.path.join(corpus_root, rel_path)
        if not os.path.isfile(full_path):
            skipped_no_lst += 1
            continue
        with open(full_path, "r", encoding="utf-8", errors="replace") as f:
            lines = f.read().split("\n")
        line_no = int(source_line)
        if line_no < 1 or line_no > len(lines):
            skipped_no_lst += 1
            continue
        raw_line = lines[line_no - 1]
        range_value = range_field_from_raw_line(raw_line)
        if range_value is None:
            skipped_no_range_field += 1
            continue
        keyword = range_value.strip()
        if keyword not in KNOWN_KEYWORDS:
            skipped_not_known_keyword += 1
            continue
        base_ft, rate_ft, per_levels = formulas[keyword]

        if u.get("corpus_key") not in keys_with_range_token(book) and u.get(
            "name"
        ) not in keys_with_range_token(book):
            skipped_unresolved_in_corpus_cache += 1
            continue

        if full_path not in sha_cache:
            with open(full_path, "rb") as f:
                sha_cache[full_path] = hashlib.sha256(f.read()).hexdigest()

        entries.append(
            {
                "unit_id": u["id"],
                "book": book,
                "record_key": u.get("corpus_key") or u.get("name"),
                "upstream_lst": rel_path,
                "upstream_lst_sha256": sha_cache[full_path],
                "upstream_line": line_no,
                "corpus_field": f"RANGE:{range_value}",
                "expected": {"base_ft": base_ft, "rate_ft": rate_ft, "per_levels": per_levels},
            }
        )
        if args.limit and len(entries) >= args.limit:
            break

    entries.sort(key=lambda e: e["unit_id"])
    if args.write:
        fixture_path = os.path.join(
            REPO_ROOT, "tests", "fixtures", "rules_core", "derived-evaluator-fixtures.json"
        )
        with open(fixture_path) as f:
            doc = json.load(f)
        previous = {e["unit_id"] for e in doc.get("spell_range_entries", [])}
        dropped = sorted(previous - {e["unit_id"] for e in entries})
        if dropped:
            # A generated artifact may GROW freely; it may never silently
            # shrink. Same posture as `v06_work_inventory`'s stamp-loss guard:
            # refuse and name the losses rather than write them away.
            print(
                f"# FATAL: this run would drop {len(dropped)} already-covered "
                f"unit(s) from spell_range_entries, first 5: {dropped[:5]}",
                file=sys.stderr,
            )
            return 1
        doc["spell_range_entries"] = entries
        with open(fixture_path, "w") as f:
            f.write(json.dumps(doc, indent=2) + "\n")
        print(f"# wrote {len(entries)} spell_range_entries to {fixture_path}", file=sys.stderr)
    else:
        print(json.dumps(entries, indent=2), file=sys.stdout)
    print(
        f"# {len(entries)} entries; candidates={len(candidates)}; "
        f"skipped_no_lst={skipped_no_lst} skipped_no_range_field={skipped_no_range_field} "
        f"skipped_not_known_keyword={skipped_not_known_keyword} "
        f"skipped_unresolved_in_corpus_cache={skipped_unresolved_in_corpus_cache}",
        file=sys.stderr,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
