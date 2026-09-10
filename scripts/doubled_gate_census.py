#!/usr/bin/env python3
"""Count converted feat records whose `applies` gate states its term list twice.

SD-35 `AT-35-E6-001` cycle 3's own re-derive command, kept as a script because the
figure it produces (963 of 1,830 at `da5e9f8d3c`) is quoted in that cycle's receipt
and in `progress.md`, and a figure ships with the command that produces it
(`AGENTS.md` rule 9).

Why the defect existed
----------------------
`sheet_rule_convert` conjoins a record's own gate onto every line it emits
(`src/pcgen_import/sheet_rule/convert.rs`: `Applies::all(vec![record_applies,
line.applies])`). For a record whose only line-level gate IS the record gate, the
conjunction stated each requirement twice. `A and A` is `A`, so no evaluator ever
disagreed -- but every consumer that REPORTS a gate printed each requirement twice,
and `feat_prereqs` counts terms. `Applies::all` now drops a term already present.

Usage
-----
    python3 scripts/doubled_gate_census.py [<git-rev>] [--kind feat]

With a revision, the converted records are read out of git at that revision, so the
pre-fix population can be re-derived without touching the working tree (never
`git stash` on a shared checkout). With no revision, the working tree is read and a
healthy tree prints `doubled=0`.
"""

import argparse
import glob
import json
import os
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))


def _doubled(gate):
    """True when the gate is an `All` whose term list is exactly itself, repeated."""
    if not isinstance(gate, dict) or "All" not in gate:
        return None  # not a gated record: outside the denominator
    terms = [json.dumps(t, sort_keys=True) for t in gate["All"]]
    half = len(terms) // 2
    return len(terms) % 2 == 0 and half > 0 and terms[:half] == terms[half:]


def _records_at_rev(rev, kind):
    listing = subprocess.run(
        ["git", "-C", ROOT, "ls-tree", "-r", "--name-only", rev, "data/sheet_rules"],
        capture_output=True,
        text=True,
        check=True,
    ).stdout.split()
    for path in listing:
        if f"/{kind}/" not in path:
            continue
        text = subprocess.run(
            ["git", "-C", ROOT, "show", f"{rev}:{path}"],
            capture_output=True,
            text=True,
            check=True,
        ).stdout
        yield from json.loads(text)


def _records_in_tree(kind):
    for path in sorted(glob.glob(os.path.join(ROOT, "data/sheet_rules", "*", kind, "*.json"))):
        with open(path, encoding="utf-8") as handle:
            yield from json.load(handle)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("rev", nargs="?", help="git revision to read; default: the working tree")
    parser.add_argument("--kind", default="feat", help="record kind directory (default: feat)")
    args = parser.parse_args(argv)

    records = _records_at_rev(args.rev, args.kind) if args.rev else _records_in_tree(args.kind)
    gated = doubled = 0
    for record in records:
        if "#" in record.get("id", ""):
            continue  # a `#`-suffixed sibling is not a principal record
        verdict = _doubled(record.get("applies"))
        if verdict is None:
            continue
        gated += 1
        doubled += bool(verdict)

    where = args.rev or "(working tree)"
    print(f"rev={where} kind={args.kind} gated={gated} doubled={doubled}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
