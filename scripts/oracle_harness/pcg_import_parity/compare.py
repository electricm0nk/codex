#!/usr/bin/env python3
"""SD-42 E1.7: the import parity ledger's working view (decisions.md §9).

The gate is crates/codex-ingest/tests/pcg_import_parity.rs. Run it with PCG_PARITY_OUT=<dir> and it
writes one <build>.json per build: every compared field as [field, pcgen, codex]. This script reads
those and the ledger and prints, per build, `fields compared / equal / differ / explained`, then
every difference no ledger row explains as a TSV row (build, field, pcgen, codex) for the author to
fix or to explain (why, decision), and every ledger row that explains nothing (stale).

    PCG_PARITY_OUT=/tmp/pcgp cargo test --locked --test pcg_import_parity   # in crates/codex-ingest
    python3 scripts/oracle_harness/pcg_import_parity/compare.py /tmp/pcgp [--ledger <explained.tsv>]

Exit 1 when anything is unexplained or stale. The matching rule is the test's: a ledger field ending
`*` covers every field with that prefix; a `*` value matches any value.
"""
import argparse
import json
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent


def covers(row, build, field, pcgen, codex):
    want_field = row["field"]
    field_ok = field.startswith(want_field[:-1]) if want_field.endswith("*") else field == want_field
    return (row["build"] == build and field_ok
            and row["pcgen"] in ("*", pcgen) and row["codex"] in ("*", codex))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("dump")
    ap.add_argument("--ledger", default=str(HERE / "explained.tsv"))
    args = ap.parse_args()
    lines = pathlib.Path(args.ledger).read_text().splitlines()[1:]
    rows = [dict(zip(("build", "field", "pcgen", "codex", "why", "decision"), l.split("\t")))
            for l in lines if l.strip() and not l.startswith("#")]
    used = [False] * len(rows)
    bad = 0
    for path in sorted(pathlib.Path(args.dump).glob("*.json")):
        build = path.stem
        compared = json.loads(path.read_text())
        diffs = [c for c in compared if c[1] != c[2]]
        explained = 0
        for field, pcgen, codex in diffs:
            hit = next((i for i, r in enumerate(rows) if covers(r, build, field, pcgen, codex)), None)
            if hit is None:
                print(f"{build}\t{field}\t{pcgen}\t{codex}")
                bad += 1
            else:
                used[hit] = True
                explained += 1
        print(f"# {build}: fields compared {len(compared)}, equal {len(compared) - len(diffs)}, "
              f"differ {len(diffs)}, explained {explained}", file=sys.stderr)
    for row, u in zip(rows, used):
        if not u:
            print(f"# stale: {row}", file=sys.stderr)
            bad += 1
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())
