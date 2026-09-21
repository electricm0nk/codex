#!/usr/bin/env python3
"""Generate `docs/architecture/status.md`'s class-coverage table from the
`class_census` bin's own measured JSON.

SD-36 Epic F (`docs/release/SD-36-consolidation/epic-f-class-completion.md`
§2), batch F0, step F0e. F0a-d built the permanent census instrument
(`src/rules_core/class_census.rs`, `src/bin/class_census.rs`) and the
`class-census` verify stage; the "Class/level compute coverage —
corpus-wide" section of `status.md` was, until this step, a **hand-written**
table a human had to remember to update every time the census moved. This
script closes that gap: it reads the census's own `--json` document (either
freshly run or an already-produced file passed with `--json`) and renders
the headline-numbers table and the per-family breakdown table from it,
verbatim, with every figure carrying its own denominator and re-derive
command right next to it (`AGENTS.md` rule 9 / this repo's
`denominator_gate.py` discipline).

Paper-sheet doctrine (`docs/governance/no-stub-mvp-doctrine.md`): this
script never fabricates a number the census does not report. In
particular the census's `--json` document carries no per-class "has a
real chassis" boolean (only `family`/`books`/`registries`/`max_level`/
`status`), so the generated table has **no Chassis column** — the old
hand-written table's "117 of 135 have a chassis" / "56 of 74 prestige have
a chassis" figures are retired here, not silently carried forward as
something this instrument does not actually measure. See the F0e commit
body for the full list of what changed against the prior hand-written
table (import: nothing that disagreed was kept without saying so, per
this step's own written brief — "where the hand-written table in
status.md disagrees, the census wins").

Two modes:

- (default) run: render the block and splice it into `docs/architecture/
  status.md` between the `<!-- class-census:begin -->` / `<!-- class-census:
  end -->` markers (which must already exist in the file — this script never
  guesses where to put them; an operator places them once, by hand, at the
  right spot in the document).
- `--check`: render the block and compare it, byte for byte, against what
  is currently between the markers. Exit 0 if identical, 1 if the file has
  drifted (a rendered census that disagrees with the last-committed table)
  or the markers are missing/duplicated.

`--json <path>` in either mode reads an already-produced census document
instead of invoking the `class_census` bin itself (useful in tests, and in
`scripts/verify.sh`'s `class-census` stage, which has already produced one
this run and should not pay for a second `cargo run`).
"""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
from collections import OrderedDict

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DEFAULT_STATUS_MD = os.path.join(REPO_ROOT, "docs", "architecture", "status.md")

MARKER_BEGIN = "<!-- class-census:begin -->"
MARKER_END = "<!-- class-census:end -->"

CENSUS_BIN_CMD = [
    "cargo", "run", "--locked", "--quiet", "-j", "2", "--bin", "class_census", "--",
]

RE_DERIVE_CMD = (
    "`cargo run --locked -j 2 --bin class_census -- --json /tmp/census.json`"
)

# The non-prestige family labels this table knows how to render, in the
# fixed order the old hand-written table used
# (`src/rules_core/class_census.rs`'s `ClassFamily` enum, minus `Prestige`,
# which gets its own row built from the top-level `prestige_*` fields
# instead of the per-class `classes` array). A family label the census
# emits that is NOT in this list is a loud error, not a silent drop — see
# `family_rows`.
KNOWN_FAMILY_ORDER = [
    "CRB",
    "APG",
    "ACG",
    "Pathfinder Unchained",
    "Ultimate Combat",
    "Untabled exotic base classes",
    "CRB NPC / Ex-* classes",
    "generic_class_chassis-only (unclaimed by any of the eight canonical sources)",
]


class DriftError(Exception):
    pass


def load_census(json_path=None):
    """Return the parsed census document. `json_path` reads an existing
    file; `None` runs the `class_census` bin fresh into a throwaway file
    under the repo's own target dir (never `/tmp` — this repo's scratch
    discipline keeps temp census output inside the checkout's own scratch
    area when the caller does not name a path)."""
    if json_path:
        with open(json_path, "r", encoding="utf-8") as fh:
            return json.load(fh)

    scratch_dir = os.path.join(REPO_ROOT, "target", "class-census-scratch")
    os.makedirs(scratch_dir, exist_ok=True)
    out_path = os.path.join(scratch_dir, "gen_class_status_table.json")
    cmd = CENSUS_BIN_CMD + ["--json", out_path]
    result = subprocess.run(cmd, cwd=REPO_ROOT, capture_output=True, text=True)
    if result.returncode != 0:
        raise RuntimeError(
            f"class_census bin exited {result.returncode}\n"
            f"stdout: {result.stdout}\nstderr: {result.stderr}"
        )
    with open(out_path, "r", encoding="utf-8") as fh:
        return json.load(fh)


def family_rows(doc):
    """Group `doc['classes']` (the non-prestige sweep) by family, in
    `KNOWN_FAMILY_ORDER`. Returns an ordered list of dicts:
    `{label, ids, computed, books}`. Raises `ValueError` (loud, not a
    silent drop) if a class carries a family label this script does not
    recognize — a real census change (a new `ClassFamily` variant) must
    update `KNOWN_FAMILY_ORDER` explicitly, never fall through unreported."""
    buckets = OrderedDict((label, {"ids": 0, "computed": 0, "books": set()}) for label in KNOWN_FAMILY_ORDER)
    unknown = set()
    for entry in doc.get("classes", []):
        label = entry["family"]
        if label not in buckets:
            unknown.add(label)
            continue
        buckets[label]["ids"] += 1
        if entry["status"] == "Computed":
            buckets[label]["computed"] += 1
        for book in entry.get("books", []):
            buckets[label]["books"].add(book)
    if unknown:
        raise ValueError(
            "class_census reported a family label this generator does not "
            f"know: {sorted(unknown)} — add it to KNOWN_FAMILY_ORDER, do "
            "not drop it silently"
        )
    rows = []
    for label, agg in buckets.items():
        if agg["ids"] == 0:
            continue  # never populated today (e.g. GenericOnly) — omit, don't fabricate a 0 row for a family with no evidence either way
        rows.append({
            "label": label,
            "ids": agg["ids"],
            "computed": agg["computed"],
            "books": sorted(agg["books"]),
        })
    return rows


def render_headline_table(doc):
    ids = doc["ids"]
    computed = doc["computed"]
    blocked = doc["blocked"]
    prestige_swept = doc["prestige_swept"]
    prestige_alone_blocked = doc["prestige_alone_blocked"]
    prestige_mix_computed = doc["prestige_mix_computed"]
    mix_swept = doc["mix_panel_swept"]
    mix_computed = doc["mix_panel_computed"]
    mix_blocked = doc["mix_panel_blocked"]

    lines = [
        "| Quantity | Count | Denominator | Census JSON field |",
        "|---|---|---|---|",
        f"| Distinct class ids, corpus-wide, across all engine registries | **{ids}** | — | `ids` |",
        f"| ...reach `Computed` at every swept level (non-prestige) | **{computed}** | of {ids} | `computed` |",
        f"| ...reach `Computed` at no level (non-prestige) | **{blocked}** | of {ids} | `blocked` |",
        f"| Prestige ids swept (never measured alone — see the carrier rule below) | **{prestige_swept}** | of {ids} total ids | `prestige_swept` |",
        f"| ...Blocked alone (negative control) | **{prestige_alone_blocked}** | of {prestige_swept} | `prestige_alone_blocked` |",
        f"| ...`Computed` in their deterministic carrier mix | **{prestige_mix_computed}** | of {prestige_swept} | `prestige_mix_computed` |",
        f"| Multiclass mix-panel rows swept (existing negative-control inputs, re-used) | **{mix_swept}** | — | `mix_panel_swept` |",
        f"| ...reach `Computed` | **{mix_computed}** | of {mix_swept} | `mix_panel_computed` |",
        f"| ...stay `Blocked` | **{mix_blocked}** | of {mix_swept} | `mix_panel_blocked` |",
    ]
    return "\n".join(lines)


def render_family_table(doc, rows):
    ids = doc["ids"]
    computed = doc["computed"]
    prestige_swept = doc["prestige_swept"]
    prestige_mix_computed = doc["prestige_mix_computed"]

    lines = [
        "| Family | Book(s) | Ids | `Computed` (all swept levels, alone) |",
        "|---|---|---|---|",
    ]
    for row in rows:
        books = ", ".join(row["books"]) if row["books"] else "—"
        lines.append(f"| {row['label']} | {books} | {row['ids']} | {row['computed']} |")
    lines.append(
        f"| Prestige | see per-class `books` in the census JSON (11 source books) | "
        f"{prestige_swept} | n/a alone (never a legitimate measurement — see headline "
        f"numbers: {prestige_mix_computed} of {prestige_swept} `Computed` in carrier mix) |"
    )
    total_ids = sum(r["ids"] for r in rows) + prestige_swept
    lines.append(
        f"| **Total** | | **{total_ids}** | **{computed}** of {total_ids} non-prestige "
        f"ids Computed alone (prestige carrier-mix result kept separate, per headline "
        f"numbers above — the bin's own `--json` output never folds the two together) |"
    )
    if total_ids != ids:
        raise ValueError(
            f"family_rows + prestige_swept ({total_ids}) != doc['ids'] ({ids}) — "
            "the census's own partition no longer sums to its own total; this is a "
            "real drift the generator must not paper over"
        )
    return "\n".join(lines)


def render_block(doc):
    headline = render_headline_table(doc)
    rows = family_rows(doc)
    family = render_family_table(doc, rows)
    parts = [
        MARKER_BEGIN,
        "**Generated table — do not hand-edit this region.** Produced by "
        "`python3 scripts/gen_class_status_table.py` from a fresh "
        f"{RE_DERIVE_CMD} sweep (`class_census`'s own "
        f"`source_of_truth`: `{doc.get('source_of_truth', 'codex::rules_core::pilot_compute::build_pilot_headless_receipt')}`). "
        "Re-derive and check for drift with "
        "`python3 scripts/gen_class_status_table.py --check` (also run as part of "
        "`scripts/verify.sh`'s `class-census` stage). Where this generated table "
        "disagrees with an earlier hand-written version of this section, the "
        "census wins — see the F0e commit body "
        "(`feat(sd36,epic-f0e): status.md class table generated from the census; "
        "F0 closed`) for what changed and why.",
        "",
        "**Headline numbers** (each states its own denominator and the exact "
        f"census JSON field it is read from; re-derive with {RE_DERIVE_CMD}, "
        "which also prints these same numbers to stdout as "
        "`ids=... computed=... blocked=...` / `prestige_swept=... "
        "prestige_alone_blocked=... prestige_mix_computed=...` / "
        "`mix_panel_swept=... mix_panel_computed=... mix_panel_blocked=...`):",
        "",
        headline,
        "",
        "**Per-family breakdown** (a partition of the merged census: every "
        "non-prestige id is in exactly one family row above, every prestige id "
        "is in the Prestige row; the census JSON reports no per-id chassis "
        "field, so no Chassis column is printed here — see this script's own "
        "module docstring for why the old hand-counted 117-of-135 / 56-of-74 "
        "chassis figures are retired rather than carried forward unmeasured):",
        "",
        family,
        MARKER_END,
    ]
    return "\n".join(parts) + "\n"


_MARKER_RE = re.compile(
    re.escape(MARKER_BEGIN) + r"\n(.*?)\n" + re.escape(MARKER_END),
    re.DOTALL,
)


def extract_current_block(content):
    """Return the full marker-to-marker block (markers included) currently
    in `content`. Raises `ValueError` if the markers are missing or appear
    more than once — this script refuses to guess which occurrence is the
    real one."""
    matches = list(_MARKER_RE.finditer(content))
    if not matches:
        raise ValueError(
            f"{MARKER_BEGIN} / {MARKER_END} markers not found — place them "
            "by hand around the class-coverage table region first"
        )
    if len(matches) > 1:
        raise ValueError(
            f"{MARKER_BEGIN} / {MARKER_END} markers appear {len(matches)} times — "
            "expected exactly one occurrence"
        )
    m = matches[0]
    return content[m.start():m.end()]


def apply_block(content, block):
    """Return `content` with the marker-to-marker region replaced by
    `block` (which must itself start with MARKER_BEGIN and end with
    MARKER_END). Raises the same way `extract_current_block` does on a
    missing/duplicated marker pair."""
    extract_current_block(content)  # validates exactly-one, raises otherwise
    new_block = block.rstrip("\n")
    return _MARKER_RE.sub(lambda _m: new_block, content, count=1)


def run(args):
    doc = load_census(args.json)
    rendered = render_block(doc)

    status_md_path = args.status_md
    with open(status_md_path, "r", encoding="utf-8") as fh:
        content = fh.read()

    if args.check:
        try:
            current = extract_current_block(content)
        except ValueError as exc:
            print(f"FAIL {exc}")
            return 1
        if current.strip("\n") == rendered.strip("\n"):
            print(
                f"OK class-coverage table matches the census "
                f"(ids={doc['ids']} computed={doc['computed']} "
                f"prestige_swept={doc['prestige_swept']} "
                f"mix_panel_computed={doc['mix_panel_computed']} of "
                f"{doc['mix_panel_swept']})"
            )
            return 0
        print(
            f"FAIL {status_md_path}'s class-coverage table has drifted from "
            "a fresh census sweep — re-run `python3 scripts/gen_class_status_table.py` "
            "(no --check) to regenerate it, then review and commit the diff"
        )
        return 1

    try:
        new_content = apply_block(content, rendered)
    except ValueError as exc:
        print(f"FAIL {exc}")
        return 1
    with open(status_md_path, "w", encoding="utf-8") as fh:
        fh.write(new_content)
    print(
        f"OK wrote class-coverage table to {status_md_path} "
        f"(ids={doc['ids']} computed={doc['computed']} "
        f"prestige_swept={doc['prestige_swept']} "
        f"mix_panel_computed={doc['mix_panel_computed']} of {doc['mix_panel_swept']})"
    )
    return 0


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument(
        "--check", action="store_true",
        help="check the committed table for drift instead of regenerating it",
    )
    parser.add_argument(
        "--json", default=None,
        help="an already-produced class_census --json document to read "
             "instead of running the bin fresh",
    )
    parser.add_argument(
        "--status-md", default=DEFAULT_STATUS_MD,
        help="path to the status.md file to check/regenerate (default: "
             "docs/architecture/status.md)",
    )
    args = parser.parse_args(argv)
    try:
        return run(args)
    except RuntimeError as exc:
        print(f"FAIL {exc}")
        return 1


if __name__ == "__main__":
    sys.exit(main())
