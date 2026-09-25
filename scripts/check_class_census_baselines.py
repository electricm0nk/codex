#!/usr/bin/env python3
"""Baseline gate for `scripts/verify.sh`'s `class-census` stage.

SD-36 Epic F (`docs/release/SD-36-consolidation/epic-f-class-completion.md`
§2). §2 declares FOUR baselines and says the stage "fails if any count
falls below its baseline": `BASELINE_CENSUS_IDS`, `BASELINE_CENSUS_COMPUTED`,
`BASELINE_CENSUS_MIX_COMPUTED`, `BASELINE_CENSUS_PRESTIGE_ALONE_BLOCKED`.
Until the F0-check fix for finding 6, `scripts/verify.sh`'s inline python
only ever read `ids`/`computed` -- `BASELINE_CENSUS_PRESTIGE_ALONE_BLOCKED`
and `BASELINE_CENSUS_MIX_COMPUTED` were declared in
`scripts/verify-baselines.env` with their own provenance blocks (both
blocks even said so: "Not yet wired into `run_class_census` ... recorded
here as the measured floor a later step wires the stage's own baseline
check against") but had no consumer at all, so `prestige_alone_blocked`
dropping 74 -> 0 or `mix_panel_computed` dropping 185 -> 0 would not fail
the stage.

This script is the wiring: it reads a `class_census --json` document plus
all four baselines and fails loudly (exit 1, one `FAIL ...` line per count
below its floor) the moment any of the four drops, never only two of them.
Factored out of `scripts/verify.sh`'s own inline heredoc so the check logic
has real, fast unit-test coverage (`scripts/tests/
test_check_class_census_baselines.py`) that proves each of the four red
paths actually fires, rather than only the two `run_class_census` used to
exercise.

SD-36 Epic F3c (2026-09-24) adds a fifth floor,
`BASELINE_CENSUS_PRESTIGE_MIX_COMPUTED` (the census's `prestige_mix_computed`:
prestige classes whose deterministic carrier mix reaches Computed at every
prestige level).
"""

from __future__ import annotations

import argparse
import json
import sys


def check(doc: dict, baselines: dict) -> list[str]:
    """Return a list of `FAIL ...` lines, one per baseline the document
    falls below -- empty when every one of the four counts meets or
    exceeds its floor. Raises `KeyError` (loud, not a silent default) if
    the document is missing one of the four required integer fields."""
    required = ["ids", "computed", "prestige_alone_blocked", "mix_panel_computed", "prestige_mix_computed"]
    for field in required:
        if not isinstance(doc.get(field), int):
            raise KeyError(
                f"census document carries no integer `{field}` -- a real shape change, "
                "not something this gate silently tolerates"
            )

    failures: list[str] = []
    checks = [
        ("ids", "BASELINE_CENSUS_IDS"),
        ("computed", "BASELINE_CENSUS_COMPUTED"),
        ("prestige_alone_blocked", "BASELINE_CENSUS_PRESTIGE_ALONE_BLOCKED"),
        ("mix_panel_computed", "BASELINE_CENSUS_MIX_COMPUTED"),
        ("prestige_mix_computed", "BASELINE_CENSUS_PRESTIGE_MIX_COMPUTED"),
    ]
    for field, baseline_name in checks:
        actual = doc[field]
        expected = baselines[baseline_name]
        if actual < expected:
            failures.append(f"FAIL {field} {actual} below baseline {expected} ({baseline_name})")
    return failures


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("json_path", help="a class_census --json document")
    parser.add_argument("--baseline-ids", type=int, required=True)
    parser.add_argument("--baseline-computed", type=int, required=True)
    parser.add_argument("--baseline-prestige-alone-blocked", type=int, required=True)
    parser.add_argument("--baseline-mix-computed", type=int, required=True)
    parser.add_argument("--baseline-prestige-mix-computed", type=int, required=True)
    args = parser.parse_args(argv)

    try:
        with open(args.json_path, "r", encoding="utf-8") as fh:
            doc = json.load(fh)
    except Exception as exc:  # a shape/read failure must be loud, not swallowed
        print(f"FAIL unparseable census: {exc}")
        return 1

    baselines = {
        "BASELINE_CENSUS_IDS": args.baseline_ids,
        "BASELINE_CENSUS_COMPUTED": args.baseline_computed,
        "BASELINE_CENSUS_PRESTIGE_ALONE_BLOCKED": args.baseline_prestige_alone_blocked,
        "BASELINE_CENSUS_MIX_COMPUTED": args.baseline_mix_computed,
        "BASELINE_CENSUS_PRESTIGE_MIX_COMPUTED": args.baseline_prestige_mix_computed,
    }

    try:
        failures = check(doc, baselines)
    except KeyError as exc:
        print(f"FAIL {exc}")
        return 1

    if failures:
        for line in failures:
            print(line)
        return 1

    print(
        f"OK ids={doc['ids']} computed={doc['computed']} "
        f"prestige_alone_blocked={doc['prestige_alone_blocked']} "
        f"mix_panel_computed={doc['mix_panel_computed']} "
        f"prestige_mix_computed={doc['prestige_mix_computed']}"
    )
    print(f"ACTUAL BASELINE_CENSUS_IDS={doc['ids']}")
    print(f"ACTUAL BASELINE_CENSUS_COMPUTED={doc['computed']}")
    print(f"ACTUAL BASELINE_CENSUS_PRESTIGE_ALONE_BLOCKED={doc['prestige_alone_blocked']}")
    print(f"ACTUAL BASELINE_CENSUS_MIX_COMPUTED={doc['mix_panel_computed']}")
    print(f"ACTUAL BASELINE_CENSUS_PRESTIGE_MIX_COMPUTED={doc['prestige_mix_computed']}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
