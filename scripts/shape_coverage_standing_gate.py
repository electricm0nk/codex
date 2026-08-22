#!/usr/bin/env python3
"""SD-32 Gate 3 closure invariant: the standing gate wired into
`scripts/verify.sh` (AT-32-G3-001/002/003, card `gate-3-closure-invariant`).

Why this exists
----------------
Gate 1's shape ledger (`scripts/shape_ledger.py`) proved, once, that every
not-done unit on 2026-08-22's inventory classifies into one of the ten d20
compute-shape families or an honest extension (F0/F8). A one-time proof is
not a closure invariant: the census, the corpus, and the shape vocabulary
can all move after that proof was written, and nothing re-checked it. This
script is that re-check, made a real `scripts/verify.sh` stage so it runs
on every invocation rather than being a document someone has to remember to
re-open.

It does not duplicate `shape_ledger.py`'s classification rules -- it
**reuses** `build_corpus_index` / `build_ledger` / `classify_unit`
directly, the same "reuse, don't re-derive" discipline `shape_ledger.py`
itself uses for `coverage_ledger.py`'s `not_done_population`. What this
script adds on top:

1. **A live re-run**, not a frozen artifact: every invocation re-derives
   the ledger from the current `docs/work-inventory.json` and the current
   `data/corpus/` tree, so a regression in either is caught the next time
   `scripts/verify.sh` runs, not the next time someone remembers to check.
2. **The sum-the-piles check** (`AGENTS.md` "Concurrency and Measurement" /
   `workflow-instruction.md §9` standing lesson 5): the per-family rollup
   must add back to exactly the population considered. `unclassified_count`
   alone cannot catch a `build_ledger` regression that silently drops rows
   from the `families` rollup while leaving `unclassified_count` at 0 --
   `piles_reconcile` is the independent check for that failure shape.
3. **Fail-closed on an empty predicate** (AT-32-G3-002, Decision 1a): an
   inventory with zero not-done units is not a pass. Mirrors
   `shape_ledger.py`'s and `coverage_ledger.py`'s identical posture.
4. **The corpus SHA citation** (AT-32-G3-003): every report names the
   pinned oracle commit (`scripts/pcgen-oracle-pin.env`'s
   `PCGEN_ORACLE_SHA`) the inventory and corpus were re-derived against, so
   a reader can tell which oracle pin a given closure receipt is honest
   about.

`classify_unit()` in `shape_ledger.py` structurally never returns an
uncovered family -- it falls through to F0 (no formula content) or F8
(residual) rather than ever emitting `None` -- so on a real inventory
`unclassified_count` can never organically go non-zero today. That is Gate
1's own closed-vocabulary design, not a loophole in this gate: AT-32-G3-001
requires this gate to be *capable* of going red when an uncovered object
appears, and `scripts/tests/test_shape_coverage_standing_gate.py` proves
that capability with a FABRICATED uncovered row (the same "prove it can
fail before it is trusted" discipline `test_reachability_audit.py` uses),
since the real inventory cannot exercise that path by itself.

Usage
-----
    python3 scripts/shape_coverage_standing_gate.py \\
        --inventory docs/work-inventory.json --corpus-root data/corpus \\
        --output artifacts/gate-3-closure-invariant/<run>.run.json

    echo '{}' | python3 scripts/shape_coverage_standing_gate.py   # fails closed

If `--inventory` is omitted and stdin is not a tty, the inventory JSON is
read from stdin (the AT-32-G3-002 verification command's exact shape). If
omitted with no piped stdin, `docs/work-inventory.json` is read instead, so
a bare interactive invocation does not hang waiting on a terminal.

Exit status is 0 only when the population is non-empty, every row's family
is set (`unclassified_count == 0`), and the piles reconcile. 1 in every
other case, with a message on stderr -- the fail-closed posture
AT-32-G3-002 requires.
"""

from __future__ import annotations

import argparse
import json
import os
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DEFAULT_INVENTORY = os.path.join(REPO_ROOT, "docs", "work-inventory.json")
DEFAULT_CORPUS_ROOT = os.path.join(REPO_ROOT, "data", "corpus")
PIN_FILE = os.path.join(REPO_ROOT, "scripts", "pcgen-oracle-pin.env")

sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import shape_ledger as SL  # noqa: E402
import coverage_ledger as CL  # noqa: E402


def read_oracle_sha(pin_file: str = PIN_FILE) -> str | None:
    """Reads `PCGEN_ORACLE_SHA` out of `scripts/pcgen-oracle-pin.env`
    without sourcing the file as shell (this is a Python script, not bash).
    Returns None if the file or the key is missing -- callers must treat a
    None the same as an unknown/unresolved citation, never as a passing
    stand-in for a real SHA."""
    try:
        with open(pin_file, "r", encoding="utf-8") as fh:
            for line in fh:
                line = line.strip()
                if line.startswith("PCGEN_ORACLE_SHA="):
                    return line.split("=", 1)[1].strip() or None
    except OSError:
        return None
    return None


def _load_inventory(inventory_path: str | None) -> dict:
    """Loads the inventory JSON. Raises OSError/json.JSONDecodeError on any
    failure -- callers must treat that as the fail-closed 'no coverage'
    case, never as an implicit zero-unit pass.

    Resolution order: an explicit --inventory path always wins; with none
    given, piped stdin (non-tty) is read (the literal AT-32-G3-002
    verification command shape); with neither, the live
    docs/work-inventory.json is read."""
    if inventory_path is not None:
        with open(inventory_path, "r", encoding="utf-8") as fh:
            return json.load(fh)
    if not sys.stdin.isatty():
        raw = sys.stdin.read()
        return json.loads(raw)
    with open(DEFAULT_INVENTORY, "r", encoding="utf-8") as fh:
        return json.load(fh)


def run_gate(inventory, corpus_root: str) -> tuple[int, dict]:
    """Runs the standing gate against an already-loaded inventory dict (or
    a path -- see below) and a corpus root. Returns (exit_status, report).

    `inventory` may be a dict (already parsed -- the shape the test suite
    and any Python caller uses) or a string path (loaded here) or None
    (falls back to _load_inventory's stdin/default resolution) -- this
    keeps `main()`'s CLI plumbing and the test suite's direct calls sharing
    one code path rather than two independently-maintained ones."""
    if not isinstance(inventory, dict):
        try:
            inventory = _load_inventory(inventory)
        except (OSError, json.JSONDecodeError) as exc:
            return 1, {
                "error": f"no coverage: cannot read/parse inventory at {inventory!r}: {exc}"
            }

    units = CL.not_done_population(inventory)
    if not units:
        return 1, {
            "error": (
                "no coverage: inventory has zero not-done units; an empty "
                "population cannot manufacture a passing gate (fail-closed "
                "posture, AT-32-G3-002)"
            )
        }

    books = {u.get("book") for u in units if u.get("book")}
    corpus_index = SL.build_corpus_index(corpus_root, books)
    ledger = SL.build_ledger(units, corpus_index)

    # Sum-the-piles: the per-family rollup must add back to exactly the
    # population considered -- an independent check from unclassified_count,
    # catching a build_ledger regression that silently drops rows from the
    # families dict while leaving unclassified_count reading 0.
    family_total = sum(info["count"] for info in ledger["families"].values())
    piles_reconcile = family_total == ledger["population"]
    unclassified_count = ledger["unclassified_count"]

    ok = piles_reconcile and unclassified_count == 0

    report = {
        "population": ledger["population"],
        "unclassified_count": unclassified_count,
        "family_total": family_total,
        "piles_reconcile": piles_reconcile,
        "families": {fid: info["count"] for fid, info in ledger["families"].items()},
        "corpus_sha": read_oracle_sha(),
    }
    return (0 if ok else 1), report


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument(
        "--inventory",
        default=None,
        help="path to docs/work-inventory.json; reads JSON from stdin if omitted and piped, else the live inventory",
    )
    parser.add_argument("--corpus-root", default=DEFAULT_CORPUS_ROOT, help="path to data/corpus")
    parser.add_argument("--output", help="write the full report as JSON to this path")
    args = parser.parse_args(argv)

    status, report = run_gate(args.inventory, args.corpus_root)

    if "error" in report:
        print(report["error"], file=sys.stderr)
        return status

    print(f"population (not-done units considered): {report['population']}")
    print(f"unclassified: {report['unclassified_count']}")
    print(
        f"piles reconcile: {report['piles_reconcile']} "
        f"({report['family_total']} families-total == {report['population']} population)"
    )
    print(f"corpus SHA: {report.get('corpus_sha') or 'unknown'}")
    print()
    print("family rollup:")
    for fid, count in sorted(report["families"].items()):
        print(f"  {fid:<4} {count:>7}")

    if status != 0:
        print(
            "shape-coverage-standing-gate: FAIL — an object appears that no "
            "shape covers, or the per-family piles do not reconcile to the "
            "population",
            file=sys.stderr,
        )

    if args.output:
        os.makedirs(os.path.dirname(args.output) or ".", exist_ok=True)
        with open(args.output, "w", encoding="utf-8") as fh:
            json.dump(report, fh, indent=2)
            fh.write("\n")

    return status


if __name__ == "__main__":
    sys.exit(main())
