#!/usr/bin/env python3
"""Derives `spell_entries` for `tests/fixtures/rules_core/derived-evaluator-fixtures.json`.

SD31-E6-F2-006. Traced one held `spell` unit end to end
(`advanced_class_guide:spell:adhesive_blood`, `OPEN-ISSUES.md` row 119) and found the
dominant `derived`+held population's magnitude is a PF1 caster-level-LINEAR formula in
the corpus's `DURATION:` token -- `(CASTERLEVEL)` or `(CASTERLEVEL*N)` followed by a
literal unit ("minutes", "rounds", ...). 1046 of 1161 corpus-wide `DURATION`+`CASTERLEVEL`
tokens match this exact shape (re-derived at this cycle's tip); the other 115 carry a
`min(`/`max(`/additive-term/alternation shape this script (and the Rust evaluator it feeds)
both deliberately refuse rather than guess.

**Independence, matching `derived-evaluator-fixtures.json`'s own existing guarantee**
(see its `independence` field for the equipment/`BONUS:STAT` family this restates for
spell): this generator reads ONLY the raw, pinned upstream PCGen `.lst` bytes under
`$PCGEN_CORPUS_ROOT` (default `~/workspace/repos/pcgen/data`) -- never `data/corpus/`,
which is the engine's own ingest output and therefore the artifact
`derived_evaluator_fixture_check::run_spell_bar_check` (which reads `data/corpus/` at
evaluation time, mirroring the equipment seam) must stay independent of. It reads
`docs/work-inventory.json` only for unit identity and to find each candidate's
(book, source `.lst` path, line) triple -- never for the DURATION value itself, which is
re-read from the raw upstream file at that exact line and parsed by a standalone parser
never imported from -- and structurally different in implementation from --
`src/rules_core/derived_evaluator_fixture_check.rs`'s Rust parser.

Usage: python3 scripts/derive_spell_caster_level_duration_fixtures.py [--limit N]
Writes the `spell_entries` array to stdout as JSON (caller merges it into the fixture file).
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

# `SpellCatalogRow.book` short codes -> `docs/work-inventory.json`'s `book` field.
# Restates `spell_resolver::spell_catalog_rows()`'s own 8-book chain order --
# see that module's doc comment. Read-only cross-reference; this script does not
# import or execute any Rust.
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

# The exact shape this fixture family commits to: `(CASTERLEVEL)` or
# `(CASTERLEVEL*N)` at the START of the DURATION value, followed by a non-empty
# trailing unit that itself carries no second CASTERLEVEL occurrence. Anything
# else (min(/max(/additive terms/"Concentration, up to .../"/multiple
# occurrences) is refused, matching the Rust evaluator's own refusal shape.
SIMPLE_RE = re.compile(r"^\(CASTERLEVEL(?:\s*\*\s*(\d+))?\)\s*(.+)$")


def pcgen_corpus_root() -> str:
    root = os.environ.get("PCGEN_CORPUS_ROOT")
    if root:
        return root
    return os.path.join(os.path.expanduser("~"), "workspace", "repos", "pcgen", "data")


def parse_simple_caster_level_duration(raw: str) -> tuple[int, str] | None:
    """Standalone, independent re-implementation of the shape the Rust
    evaluator (`parse_caster_level_linear_duration`) commits to. Deliberately
    NOT shared code with that function -- see module docstring."""
    m = SIMPLE_RE.match(raw.strip())
    if not m:
        return None
    per_level = int(m.group(1)) if m.group(1) else 1
    unit = m.group(2).strip()
    if not unit or "CASTERLEVEL" in unit:
        return None
    return per_level, unit


def duration_field_from_raw_line(line: str) -> str | None:
    """Splits a raw PCGen `.lst` line on tabs and returns the `DURATION:`
    field's value verbatim, or None if the line carries no such field."""
    for field in line.split("\t"):
        if field.startswith("DURATION:"):
            return field[len("DURATION:"):]
    return None


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--limit", type=int, default=None)
    ap.add_argument(
        "--work-inventory",
        default=os.path.join(REPO_ROOT, "docs", "work-inventory.json"),
    )
    args = ap.parse_args()

    corpus_root = pcgen_corpus_root()
    with open(args.work_inventory) as f:
        inv = json.load(f)

    # Existence-only filter (never the source of `expected` -- see module
    # docstring): the Rust evaluator (`run_spell_bar_check`) resolves each
    # fixture against `data/corpus/<book>/spell/`'s own `raw_tokens`, and a
    # small pre-`.lst`-pipeline population (12 corpus-wide, sourced
    # `"web_second_source"`, e.g. `advanced_players_guide:spell:fester`)
    # carries no `raw_tokens` at all, so a fixture naming one would fail the
    # bar for a reason unrelated to this seam's own correctness. Filtered
    # here rather than silently included. Indexed once per book (a per-
    # candidate directory walk would be O(candidates x book size)).
    duration_key_index_cache: dict[str, set[str]] = {}

    def keys_with_duration_token(book: str) -> set[str]:
        if book in duration_key_index_cache:
            return duration_key_index_cache[book]
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
                    if t.get("key") == "DURATION":
                        found.add(key)
                        break
        duration_key_index_cache[book] = found
        return found

    candidates = [
        u
        for u in inv["units"]
        if u.get("kind") == "spell"
        and u.get("wiring_class") == "derived"
        and u.get("status") in ("ingested-magnitude", "grounded")
        and u.get("book") in WORK_INVENTORY_BOOK_TO_SHORT
    ]

    entries = []
    sha_cache: dict[str, str] = {}
    skipped_no_lst = 0
    skipped_no_duration_field = 0
    skipped_complex = 0
    skipped_unresolved_in_corpus_cache = 0

    for u in candidates:
        source_file = u.get("source_file")
        source_line = u.get("source_line")
        book = u["book"]
        if not source_file or not source_line:
            skipped_no_lst += 1
            continue
        # `source_file` in work-inventory.json is a bare filename
        # (e.g. "acg_spells.lst"); the real repo-relative path is
        # `pathfinder/paizo/roleplaying_game/<book>/<source_file>`, matching
        # every corpus record's own `source.path` shape re-derived elsewhere
        # in this package (see `docs/release/SD-31-corpus-closure-grind/
        # decisions.md` Decision 9's own `core_essentials` census for the
        # same convention).
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
        duration_value = duration_field_from_raw_line(raw_line)
        if duration_value is None:
            skipped_no_duration_field += 1
            continue
        parsed = parse_simple_caster_level_duration(duration_value)
        if parsed is None:
            skipped_complex += 1
            continue
        per_level, unit = parsed

        if u.get("corpus_key") not in keys_with_duration_token(book) and u.get(
            "name"
        ) not in keys_with_duration_token(book):
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
                "corpus_field": f"DURATION:{duration_value}",
                "expected": {"per_level": per_level, "unit": unit},
            }
        )
        if args.limit and len(entries) >= args.limit:
            break

    entries.sort(key=lambda e: e["unit_id"])
    print(json.dumps(entries, indent=2), file=sys.stdout)
    print(
        f"# {len(entries)} entries; candidates={len(candidates)}; "
        f"skipped_no_lst={skipped_no_lst} skipped_no_duration_field={skipped_no_duration_field} "
        f"skipped_complex={skipped_complex} "
        f"skipped_unresolved_in_corpus_cache={skipped_unresolved_in_corpus_cache}",
        file=sys.stderr,
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
