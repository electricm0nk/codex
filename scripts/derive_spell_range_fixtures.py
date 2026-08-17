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


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--limit", type=int, default=None)
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

    candidates = [
        u
        for u in inv["units"]
        if u.get("kind") == "spell"
        and u.get("wiring_class") == "derived"
        and u.get("status") in ("ingested-magnitude", "grounded")
        and u.get("wiring_class_reason") == "range_keyword"
        and u.get("book") in WORK_INVENTORY_BOOK_TO_SHORT
    ]

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
