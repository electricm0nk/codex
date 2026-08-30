#!/usr/bin/env python3
"""Enumerate bucket A -- "engine has no table for this kind" -- per kind,
with its book coverage, so no future bundle has to re-derive which books a
given table's absence blocks (SD-34 `AT-34-E1-003`).

    python3 scripts/missing_engine_tables.py --check
        -> population=449 kinds=2   (exit 0)
        writes artifacts/epic-1-atlas/missing-engine-tables.json

**Updated `AT-34-E2-004`:** building the seven `simple_kind_tables` and
wiring them into `classify()` for real (`AT-34-E2-001`/`AT-34-E2-004`) moved
`skill`/`template`/`deity`/`domain`/`language`/`ability`/`trait` off bucket A
for good -- none of those seven kinds' `classify()` arms can emit the
`has_no_engine_table` marker any more (a table miss now reports `absent_from`,
bucket B), so their `ENGINE_SURFACE_CITATIONS` entries are retired rather than
left pointing at a marker the source no longer contains anywhere. Only
`companion` (28 `bestiary`-book units whose REPORTED book itself has no
chassis registration at all -- a different shape from the
`core_rulebook`/`ultimate_campaign` misattribution this cycle fixed) and
`power` (421, all `ultimate_psionics`, Epic 5's to cost and build) remain.

Reads the same `docs/work-inventory.json` `completion_atlas.py` reads and
selects bucket A the same way it does (`status == engine-does-not-hold` and
`evidence` containing `has_no_engine_table`) -- not by re-deriving a second,
divergent definition of the bucket.

For each of the 2 kinds this population actually contains, records:

  - `count` -- units of this kind in bucket A
  - `by_book` -- per-book breakdown of that count
  - `engine_surface` -- the exact `engine_does_not_hold(...)` call site in
    `src/bin/v06_work_inventory.rs` a real table would replace (the same
    `file:line:must_contain` citation shape `completion_atlas.py` uses for
    its own bucket definitions -- verified against the live file, not
    assumed, and re-checked every run)
  - `zero_bucket_a_books` -- books whose ENTIRE bucket-A population is this
    one kind; building this kind's table alone would take that book's
    bucket A to zero. A book with two or more bucket-A kinds is not listed
    under either -- it needs both tables.

**A kind present in bucket A but absent from `ENGINE_SURFACE_CITATIONS` is
an `UnknownKindError`, not a silently-dropped unit** -- the same fail-closed
posture `completion_atlas.py` condition 6 established for the atlas itself
(`decisions.md §12` L1): a new kind reaching bucket A without an engine
surface citation must break the build, not disappear from the report.
"""

from __future__ import annotations

import argparse
import collections
import json
import os
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
INVENTORY_PATH = os.path.join(REPO_ROOT, "docs", "work-inventory.json")
ARTIFACT_PATH = os.path.join(
    REPO_ROOT, "docs", "release", "SD-34-book-completion", "artifacts", "epic-1-atlas",
    "missing-engine-tables.json",
)
_ENGINE_SRC = "src/bin/v06_work_inventory.rs"
_A_MARKER = "has_no_engine_table"

# The exact `engine_does_not_hold(...)` call site emitting each kind's bucket-A
# marker in `_ENGINE_SRC`, as of this cycle's re-verification. This is "the
# engine surface a table would attach to": the arm a real per-kind table
# lookup would replace.
ENGINE_SURFACE_CITATIONS = {
    # Re-derived by AT-34-E3-003's bucket-M skill widening cycle, whose own
    # edit shifted every citation below its insertion point.
    "companion": {"line": 11504, "must_contain": "companion_content_has_no_engine_table"},
    "power": {"line": 11604, "must_contain": "power_content_has_no_engine_table"},
}


class UnknownKindError(RuntimeError):
    """A unit landed in bucket A under a kind with no engine-surface citation."""


def _load_inventory(path: str = INVENTORY_PATH) -> dict:
    with open(path, "r", encoding="utf-8") as fh:
        return json.load(fh)


def _load_units() -> list:
    return _load_inventory()["units"]


def _is_bucket_a(unit: dict) -> bool:
    return unit.get("status") == "engine-does-not-hold" and _A_MARKER in (unit.get("evidence") or "")


def _read_source_line(rel_path: str, line_no: int) -> "str | None":
    path = os.path.join(REPO_ROOT, rel_path)
    try:
        with open(path, "r", encoding="utf-8") as fh:
            lines = fh.readlines()
    except OSError:
        return None
    if line_no < 1 or line_no > len(lines):
        return None
    return lines[line_no - 1]


def citation_failures() -> list:
    """Bucket-A kinds whose citation no longer resolves or whose cited
    line's content no longer contains the marker it claims to (content, not
    just path/line -- `risks-and-open-questions.md §10`, same shape as
    `completion_atlas.py` condition 6)."""
    failures = []
    for kind, cite in ENGINE_SURFACE_CITATIONS.items():
        content = _read_source_line(_ENGINE_SRC, cite["line"])
        if content is None or cite["must_contain"] not in content:
            failures.append(f"{kind}: {_ENGINE_SRC}:{cite['line']} does not contain {cite['must_contain']!r}")
    return failures


def build_report(units: list) -> dict:
    a_units = [u for u in units if _is_bucket_a(u)]

    by_kind_book: "dict[str, collections.Counter]" = collections.defaultdict(collections.Counter)
    book_a_total: "collections.Counter" = collections.Counter()
    for u in a_units:
        kind = u.get("kind")
        book = u.get("book")
        by_kind_book[kind][book] += 1
        book_a_total[book] += 1

    unknown = sorted(set(by_kind_book) - set(ENGINE_SURFACE_CITATIONS))
    if unknown:
        raise UnknownKindError(
            f"bucket A contains kind(s) with no engine-surface citation: {unknown} "
            "-- add an ENGINE_SURFACE_CITATIONS entry before this report can be trusted"
        )

    kinds_out = {}
    for kind, book_counts in by_kind_book.items():
        zero_books = sorted(b for b, c in book_counts.items() if c == book_a_total[b])
        cite = ENGINE_SURFACE_CITATIONS[kind]
        kinds_out[kind] = {
            "count": sum(book_counts.values()),
            "by_book": dict(book_counts.most_common()),
            "engine_surface": {
                "file": _ENGINE_SRC,
                "line": cite["line"],
                "must_contain": cite["must_contain"],
            },
            "zero_bucket_a_books": zero_books,
        }

    return {
        "population": len(a_units),
        "kinds": kinds_out,
    }


def cmd_check(args) -> int:
    units = _load_units()
    try:
        report = build_report(units)
    except UnknownKindError as exc:
        print(f"UNKNOWN_KIND: {exc}")
        return 1

    cite_failures = citation_failures()
    print(f"population={report['population']} kinds={len(report['kinds'])}")
    for kind in sorted(report["kinds"]):
        k = report["kinds"][kind]
        print(f"  {kind}: count={k['count']} books={len(k['by_book'])} zero_bucket_a_books={len(k['zero_bucket_a_books'])}")
    print(f"citation_failures={len(cite_failures)}")
    for f in cite_failures:
        print(f"  citation_failure: {f}")

    artifact = dict(report)
    artifact["re_derive_command"] = "python3 scripts/missing_engine_tables.py --check"
    artifact["source"] = "docs/work-inventory.json (status == engine-does-not-hold, evidence contains 'has_no_engine_table')"

    os.makedirs(os.path.dirname(ARTIFACT_PATH), exist_ok=True)
    with open(ARTIFACT_PATH, "w", encoding="utf-8") as fh:
        json.dump(artifact, fh, indent=2, sort_keys=True)
        fh.write("\n")

    return 0 if not cite_failures else 1


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args(argv)
    if args.check:
        return cmd_check(args)
    parser.print_help()
    return 2


if __name__ == "__main__":
    sys.exit(main())
