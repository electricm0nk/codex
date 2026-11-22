#!/usr/bin/env python3
"""CLI entry point for the oracle comparison harness (`AT-33-E2-003`).

Usage:
    python3 scripts/oracle_harness/run.py \\
        --oracle-export <path to a BatchExporter KEY=VALUE export> \\
        --ours <path to a JSON file: {"unit_id": ["ORACLE_KEY", ours_value], ...}> \\
        --output <path to write oracle-results.json>

Writes `{"results": [...]}` in the exact shape
`scripts/box_ledger.py::load_oracle_results` reads (`AT-33-E1-002` condition
3) -- this is the tool `AT-33-E5-001`/`002` run against the real
fixture-verified and literal-verified populations, and whose output
`box_ledger.py --check` reads from
`docs/release/SD-33-computed-value-verification/artifacts/epic-2-oracle-harness/oracle-results.json`
by default.
"""

from __future__ import annotations

import argparse
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from oracle_harness import compare as OC  # noqa: E402
from oracle_harness import oracle_export as OE  # noqa: E402


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--oracle-export", required=True, help="path to a BatchExporter KEY=VALUE export file")
    parser.add_argument(
        "--ours", required=True,
        help='path to a JSON file: {"unit_id": ["ORACLE_EXPORT_KEY", ours_value], ...}',
    )
    parser.add_argument("--output", required=True, help="path to write the oracle-results.json to")
    args = parser.parse_args(argv)

    oracle_parsed = OE.load_oracle_export(args.oracle_export)
    with open(args.ours, "r", encoding="utf-8") as f:
        ours_raw = json.load(f)
    ours = {unit_id: (entry[0], entry[1]) for unit_id, entry in ours_raw.items()}

    records = OC.run_comparison(ours, oracle_parsed)

    counts = {"agree": 0, "disagree": 0, "unverifiable": 0}
    for r in records:
        counts[r["verdict"]] += 1

    with open(args.output, "w", encoding="utf-8") as f:
        json.dump({"results": records}, f, indent=2)
        f.write("\n")

    print(
        f"oracle_harness: {len(records)} units compared -- "
        f"agree={counts['agree']} disagree={counts['disagree']} "
        f"unverifiable={counts['unverifiable']} -> {args.output}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
