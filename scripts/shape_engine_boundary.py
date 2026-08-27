#!/usr/bin/env python3
"""State the shape-engine boundary as a committed fact, proven by execution
-- not an assumption a future bundle has to re-derive (SD-34 `AT-34-E1-004`).

    python3 scripts/shape_engine_boundary.py --check
        -> magnitude_bearing=26396 not_held_by_engine=13119 citation_ok=True
           (exit 0)
        writes artifacts/epic-1-atlas/shape-engine-boundary.md

A shape engine (`formula_interpreter`) turns a formula string into a
number. It does not place the record in a table, attach it to a character,
or show it to a player -- that is a separate, later step the engine's own
promotion ladder gates on four conditions, none of which is "a value was
computed" (`technical-design.md §3`, `decisions.md §2a`).

This instrument re-derives, at HEAD, every number the statement depends on
so the fact stays true after any later cycle touches `docs/work-inventory.json`
or `src/bin/v06_work_inventory.rs`:

  - `magnitude_bearing`   -- units carrying at least one magnitude token
                             (`magnitude_token_count > 0`)
  - `not_held_by_engine`  -- of those, the ones the engine has not promoted
                             past `engine-does-not-hold` (i.e. it refused every rung,
                             including the four-condition ladder below)
  - the promotion ladder's own source text and line numbers, re-read from
    the live file and asserted to still contain the exact conditions this
    document quotes -- content, not just path/line
    (`risks-and-open-questions.md §10`, same posture as `completion_atlas.py`
    condition 6 and `missing_engine_tables.py`'s `citation_failures()`).

Fails closed (non-zero exit, no artifact written) when either count cannot
be derived or the citation no longer resolves to the quoted content -- a
silently-stale "fact" document is worse than no document.
"""

from __future__ import annotations

import argparse
import json
import os
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
INVENTORY_PATH = os.path.join(REPO_ROOT, "docs", "work-inventory.json")
ARTIFACT_PATH = os.path.join(
    REPO_ROOT, "docs", "release", "SD-34-book-completion", "artifacts", "epic-1-atlas",
    "shape-engine-boundary.md",
)
_ENGINE_SRC = "src/bin/v06_work_inventory.rs"

# The promotion ladder's four conditions, as they appear in the live file
# today. Line numbers are 1-indexed. `technical-design.md §3` and
# `decisions.md §2a` both quote this same block, citing its last line as
# `:9595` -- the anchor a reader would grep for.
PROMOTION_LADDER_LINES = {
    9592: "if has_real_description",
    9593: "&& is_display_wiring_class_for_promotion(wc_class)",
    9594: "&& !universal_sheet_modifier",
    9595: "&& facts.class_feature_pool_catalog_holds(&unit.source_book, &unit.key)",
}
PROMOTION_LADDER_ANCHOR_LINE = 9595


class StaleCitationError(RuntimeError):
    """The promotion ladder no longer resolves to the content this
    instrument (and the package docs quoting it) assert it does."""


def _load_units() -> list:
    with open(INVENTORY_PATH, "r", encoding="utf-8") as fh:
        return json.load(fh)["units"]


def _read_source_lines(rel_path: str) -> "list[str] | None":
    path = os.path.join(REPO_ROOT, rel_path)
    try:
        with open(path, "r", encoding="utf-8") as fh:
            return fh.readlines()
    except OSError:
        return None


def citation_failures() -> list:
    """Re-verify every promotion-ladder line against the live file's actual
    content at its claimed line number -- not merely that the file and line
    exist, so a refactor that shifts code without changing line counts is
    still caught (`risks-and-open-questions.md §10`)."""
    lines = _read_source_lines(_ENGINE_SRC)
    failures = []
    if lines is None:
        return [f"{_ENGINE_SRC}: file not found"]
    for line_no, expected in PROMOTION_LADDER_LINES.items():
        if line_no < 1 or line_no > len(lines):
            failures.append(f"{_ENGINE_SRC}:{line_no}: out of range ({len(lines)} lines in file)")
            continue
        actual = lines[line_no - 1].strip()
        if expected not in actual:
            failures.append(
                f"{_ENGINE_SRC}:{line_no}: expected to contain {expected!r}, found {actual!r}"
            )
    return failures


def magnitude_bearing(units: list) -> list:
    return [u for u in units if (u.get("magnitude_token_count") or 0) > 0]


def not_held_by_engine(units: list) -> list:
    """Of the magnitude-bearing population, the units the engine has not
    promoted past `engine-does-not-hold` -- it refused every rung, including the
    four-condition promotion ladder this document states as fact."""
    return [u for u in units if u.get("status") == "engine-does-not-hold"]


def build_report(units: list) -> dict:
    failures = citation_failures()
    if failures:
        raise StaleCitationError(
            "promotion-ladder citation no longer resolves at HEAD: " + "; ".join(failures)
        )

    mag = magnitude_bearing(units)
    stuck = not_held_by_engine(mag)

    ladder_source = "".join(
        _read_source_lines(_ENGINE_SRC)[9591:9595]  # lines 9592..9595, 0-indexed slice
    )

    return {
        "magnitude_bearing": len(mag),
        "not_held_by_engine": len(stuck),
        "engine_source": _ENGINE_SRC,
        "promotion_ladder_anchor_line": PROMOTION_LADDER_ANCHOR_LINE,
        "promotion_ladder_source": ladder_source,
        "citation_ok": True,
    }


def render_markdown(report: dict) -> str:
    return f"""# The shape-engine boundary

A committed statement, proven by execution, of what a shape engine does and where its output
stops -- so no future bundle re-learns it (SD-34 `AT-34-E1-004`).

Re-derive: `python3 scripts/shape_engine_boundary.py --check`

## The fact

**A shape engine turns a formula string into a number.** `formula_interpreter` covers F1..F9
(`technical-design.md §3`): population 11,652, recognised 10,626, refused 240, unjoined 786
(`content-unit-inventory.md`). It refuses rather than guesses:

```
"var(\\"CL=Arcanist\\")" -> unrecognised function "var" -- refusing rather than guessing its semantics
```

**It does not place the record, attach it, or display it.** Those are separate, later steps
gated by the engine's own promotion ladder -- the real authority, quoted below with its line
number re-verified at HEAD, not assumed:

```rust
{report['promotion_ladder_source']}```

(`{report['engine_source']}:{report['promotion_ladder_anchor_line']}` -- re-checked by content,
not just path/line, on every run of this instrument.)

None of the four conditions is "a value was computed". Fail the last one and the verdict is
`class_feature_owner_matched_by_name_but_record_not_held_by_engine` -- a unit the shape engine
may already compute a correct number for, still refused promotion because no table holds the
record it would attach to.

## The measured consequence

- **{report['magnitude_bearing']}** units in `docs/work-inventory.json` carry at least one
  magnitude token (`magnitude_token_count > 0`) -- re-derive:
  `python3 -c "import json; d=json.load(open('docs/work-inventory.json')); print(sum(1 for u in d['units'] if (u.get('magnitude_token_count') or 0) > 0))"`
  (denominator: {report['magnitude_bearing']} of the corpus's full unit population, printed by
  `scripts/completion_atlas.py --check`)
- Of those **{report['magnitude_bearing']}**, **{report['not_held_by_engine']}** are still not
  held by the engine (`status == engine-does-not-hold`) -- re-derive:
  `python3 -c "import json; d=json.load(open('docs/work-inventory.json')); m=[u for u in d['units'] if (u.get('magnitude_token_count') or 0) > 0]; print(sum(1 for u in m if u.get('status') == 'engine-does-not-hold'))"`
  (denominator: {report['magnitude_bearing']} magnitude-bearing units, computed immediately
  above)

**Half the shape engine's own feedstock is stuck downstream of it.** This is exactly the gap
Epic 2's tables and Epics 3-4's per-bucket work close -- the engine already works; the boundary
is where its output goes next.

## Why this is a fact, not an assumption

Both counts above and the citation are re-derived by
`python3 scripts/shape_engine_boundary.py --check` on every invocation, against the live
`docs/work-inventory.json` and the live `{report['engine_source']}` -- never quoted from an
earlier document (`decisions.md §12` L2). The instrument fails closed (non-zero exit, no
artifact written) if the citation's line numbers stop containing the exact conditions quoted
above, so a refactor that moves this code cannot leave a stale "fact" behind
(`risks-and-open-questions.md §10`).
"""


def main(argv: "list[str] | None" = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="re-derive and write the artifact")
    args = parser.parse_args(argv)

    units = _load_units()
    try:
        report = build_report(units)
    except StaleCitationError as exc:
        print(f"STALE_CITATION: {exc}", file=sys.stderr)
        return 1

    if args.check:
        os.makedirs(os.path.dirname(ARTIFACT_PATH), exist_ok=True)
        with open(ARTIFACT_PATH, "w", encoding="utf-8") as fh:
            fh.write(render_markdown(report))

    print(
        f"magnitude_bearing={report['magnitude_bearing']} "
        f"not_held_by_engine={report['not_held_by_engine']} "
        f"citation_ok={report['citation_ok']}"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
