#!/usr/bin/env python3
"""State the shape-engine boundary as a committed fact, proven by execution
-- not an assumption a future bundle has to re-derive (SD-34 `AT-34-E1-004`).

    python3 scripts/shape_engine_boundary.py --check
        -> magnitude_bearing=26396 not_held_by_engine=8784 citation_ok=True
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
  - the promotion ladder's own source text, found by CONTENT ANCHOR inside
    `classify` (SD-35 `AT-35-E1-002`): the four condition lines must appear,
    consecutively and exactly once, in that function -- the line they
    resolve to is derived at check time, never pinned (the `file:line` pin
    this file carried before was re-derived by hand in eight SD-34 waves and
    was found already drifted at HEAD by wave 51 because nobody had asked
    the gate; `git log -p` on this file keeps that history).

Fails closed (non-zero exit, no artifact written) when either count cannot
be derived or the anchor no longer resolves -- a silently-stale "fact"
document is worse than no document.
"""

from __future__ import annotations

import argparse
import json
import os
import sys

from completion_atlas import resolve_content_anchor

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
INVENTORY_PATH = os.path.join(REPO_ROOT, "docs", "work-inventory.json")
ARTIFACT_PATH = os.path.join(
    REPO_ROOT, "docs", "release", "SD-34-book-completion", "artifacts", "epic-1-atlas",
    "shape-engine-boundary.md",
)
_ENGINE_SRC = "src/bin/v06_work_inventory.rs"

# The promotion ladder's four conditions, as a content anchor: the exact
# lines (indentation stripped), consecutive, exactly once inside `classify`.
# A sibling `if has_real_description` block earlier in `classify` shares the
# first three lines and differs on the fourth (`facts.explanation_ids
# .contains(...)`), so all four are load-bearing for uniqueness.
PROMOTION_LADDER_ANCHOR = {
    "file": _ENGINE_SRC,
    "context_fn": "classify",
    "anchor": [
        "if has_real_description",
        "&& is_display_wiring_class_for_promotion(wc_class)",
        "&& !universal_sheet_modifier",
        "&& facts.class_feature_pool_catalog_holds(&unit.source_book, &unit.key)",
    ],
}


class StaleCitationError(RuntimeError):
    """The promotion ladder no longer resolves to the content this
    instrument (and the package docs quoting it) assert it does."""


def _load_units() -> list:
    with open(INVENTORY_PATH, "r", encoding="utf-8") as fh:
        return json.load(fh)["units"]


def resolve_promotion_ladder(citation: dict = PROMOTION_LADDER_ANCHOR,
                             lines: "list[str] | None" = None) -> dict:
    """`resolve_content_anchor` over the ladder anchor; `lines` lets a test
    resolve against supplied text instead of the file on disk."""
    return resolve_content_anchor(citation, lines=lines, repo_root=REPO_ROOT)


def citation_failures(citation: dict = PROMOTION_LADDER_ANCHOR,
                      lines: "list[str] | None" = None) -> list:
    """Re-verify the promotion ladder by content anchor -- the four cited
    lines must still be present, consecutive, and unique inside `classify`,
    wherever a refactor has moved them (`risks-and-open-questions.md §10`)."""
    resolved = resolve_promotion_ladder(citation, lines)
    if resolved["ok"]:
        return []
    return [resolved["reason"]]


def magnitude_bearing(units: list) -> list:
    return [u for u in units if (u.get("magnitude_token_count") or 0) > 0]


def not_held_by_engine(units: list) -> list:
    """Of the magnitude-bearing population, the units the engine has not
    promoted past `engine-does-not-hold` -- it refused every rung, including the
    four-condition promotion ladder this document states as fact."""
    return [u for u in units if u.get("status") == "engine-does-not-hold"]


def build_report(units: list) -> dict:
    resolved = resolve_promotion_ladder()
    if not resolved["ok"]:
        raise StaleCitationError(
            "promotion-ladder citation no longer resolves at HEAD: " + resolved["reason"]
        )

    mag = magnitude_bearing(units)
    stuck = not_held_by_engine(mag)

    return {
        "magnitude_bearing": len(mag),
        "not_held_by_engine": len(stuck),
        "engine_source": _ENGINE_SRC,
        "promotion_ladder_context_fn": PROMOTION_LADDER_ANCHOR["context_fn"],
        # Derived by search at check time -- the last of the four cited lines.
        "promotion_ladder_anchor_line": resolved["end_line"],
        "promotion_ladder_source": "\n".join(resolved["source"]) + "\n",
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
gated by the engine's own promotion ladder -- the real authority, quoted below from the live
file by content anchor, not assumed:

```rust
{report['promotion_ladder_source']}```

(`{report['engine_source']}`, inside `fn {report['promotion_ladder_context_fn']}`, resolving to
line {report['promotion_ladder_anchor_line']} at the time of this run -- found by searching for
these exact four lines on every run of this instrument, so a refactor that moves them keeps this
citation green and a change to any of them fails it.)

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

**{report['not_held_by_engine'] * 100.0 / report['magnitude_bearing']:.1f}% of the shape engine's own feedstock is still stuck downstream of it**
({report['not_held_by_engine']} of {report['magnitude_bearing']} magnitude-bearing units; this
fraction moved from just over half, 13119/26396, at Epic 1's original AT-34-E1-004 cycle
to {report['not_held_by_engine']}/{report['magnitude_bearing']} here, as Epic 3's per-bucket
work and SD-35's corpus-wide conversion closed real units -- see `decisions.md §12` L10: a count
that drops from measurement work is closure, not a re-measurement artifact). This is exactly the
gap Epic 2's tables and Epics 3-4's per-bucket work close -- the engine already works; the
boundary is where its output goes next.

## Why this is a fact, not an assumption

Both counts above and the citation are re-derived by
`python3 scripts/shape_engine_boundary.py --check` on every invocation, against the live
`docs/work-inventory.json` and the live `{report['engine_source']}` -- never quoted from an
earlier document (`decisions.md §12` L2). The instrument fails closed (non-zero exit, no
artifact written) if the four quoted lines stop appearing, consecutively and exactly once,
inside `fn {report['promotion_ladder_context_fn']}` -- so a refactor cannot leave a stale
"fact" behind, and a change to the ladder's conditions cannot pass unnoticed
(`risks-and-open-questions.md §10`; SD-35 `AT-35-E1-002`).
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
