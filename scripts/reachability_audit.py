#!/usr/bin/env python3
"""scripts/reachability_audit.py -- SD31-E0-F1, the standing reachability
audit (`decisions.md §4`).

Answers one question mechanically, for every unit on the board: *given
current engine capability, does a path to `done` exist?*

It imports the dashboard producer's own verdict function
(`scripts/observer/pf1e_dashboard_producer.py`) rather than reimplementing
its table -- a reimplemented table drifts from the thing it audits, which is
exactly what this audit exists to prevent.

THE GRID
--------
`_doneness_verdict_uncapped(wiring_class, status)` is evaluated over the
FULL `WIRING_CLASS_VALUES x status_vocabulary` cross product -- not just the
`(wiring_class, status)` pairs actually present on the board -- widened by
any wiring_class/status word actually observed on a real unit but absent
from the producer's/document's own declared vocabulary (a NOVEL WIRING_CLASS
word is exactly the failure mode this audit exists to catch; silently
excluding it from the grid would be the "reimplemented table that drifts"
hazard applied to the grid instead of the verdict function).

CORRECTED CLAIM (2026-08-15, Opus adversarial-review CONFIRMED finding
against an earlier revision of this paragraph and of commit `eadb263f7`'s
message). The detection envelope above is proven only for a NOVEL
WIRING_CLASS WORD, not for a novel STATUS word landing on a real `computed`
or `display` unit: both branches' catch-all `else` in
`_doneness_verdict_uncapped` (`computed`: `DONENESS_DONE if status ==
"grounded" else DONENESS_IN_PROGRESS`; `display`: `... else
DONENESS_IN_PROGRESS`) silently absorbs an unrecognised status word into
`in-progress` -- never raises `ValueError`, never reported as `unmapped`,
never moves the ceiling. Mutation-tested: forcing 3 real `computed` units to
a fabricated status leaves `unmapped_cells_with_units` empty and the
reported ceiling unchanged (`ok=True`, exit 0), where a fabricated
WIRING_CLASS word on 1 real unit correctly fails (`ok=False`, exit 1). The
producer's own `doneness_verdict()` docstring states the intended contract
covers both axes; this audit's status-axis coverage does not yet meet it.
Non-blocking (the miss cannot make a unit vanish from a rollup, only
misclassify it within `computed`/`display`'s existing branches) --
tracked as `OPEN-ISSUES.md` row 6, owner Epic 0, remedy: have `audit()`
additionally report cells whose `(wiring_class, status)` pair is absent from
the producer's/document's declared status vocabulary yet resolved by a
catch-all branch, flagged `default-absorbed` alongside `unmapped`.

Two kinds of dead-end cell are reported:

  * `unmapped`      -- `_doneness_verdict_uncapped()` raised `ValueError`.
                        Those units are absent from EVERY rollup the
                        producer or this audit computes. Any such cell with
                        on-board `unit_count > 0` fails the audit outright
                        (`ok=False`, non-zero exit) -- this is a real defect
                        in the doneness table, not a known, tracked gap.
  * `no-done-path`   -- the cell itself does not map to `done`, AND no
                        status at all maps that wiring_class to `done`
                        either (today: `ambiguous` alone -- Decision 4's
                        2,109-unit gap, owned by Epic 2). Reported and
                        counted against the reachable ceiling, but does NOT
                        by itself fail the audit: a known, epic-owned
                        capability gap is not an audit-table bug.

Usage:
    python3 scripts/reachability_audit.py [--inventory PATH] [--json-out PATH]

Exit status: 0 unless an unmapped cell carries on-board units.
"""
from __future__ import annotations

import argparse
import collections
import importlib.util
import json
import pathlib
import sys

_REPO_ROOT = pathlib.Path(__file__).resolve().parent.parent
_PRODUCER_PATH = _REPO_ROOT / "scripts" / "observer" / "pf1e_dashboard_producer.py"
DEFAULT_INVENTORY = _REPO_ROOT / "docs" / "work-inventory.json"


def _load_producer(producer_path: pathlib.Path = _PRODUCER_PATH):
    spec = importlib.util.spec_from_file_location("pf1e_dashboard_producer", producer_path)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


PRODUCER = _load_producer()


def load_inventory(path) -> dict:
    with open(path, encoding="utf-8") as f:
        return json.load(f)


def _cell(wiring_class: str, status: str) -> str:
    # Same "wc|status" join the producer's own cross_tab uses -- neither
    # axis carries a pipe (both closed vocabularies of bare words), so this
    # is unambiguous to split back apart if a caller ever needs to.
    return f"{wiring_class}|{status}"


def audit(doc: dict, producer=PRODUCER) -> dict:
    """Run the reachability audit over `doc` (a work-inventory.json-shaped
    document). Returns a JSON-serializable result dict."""
    excluded = producer.EXCLUDED_BOOKS
    units = [u for u in (doc.get("units") or [])
             if (u.get("book") or "unknown") not in excluded]

    wiring_classes = set(producer.WIRING_CLASS_VALUES)
    status_vocab = set((doc.get("status_vocabulary") or {}).keys())

    cell_counts: collections.Counter = collections.Counter()
    cell_counts_by_kind: dict[str, collections.Counter] = collections.defaultdict(collections.Counter)
    kind_totals: collections.Counter = collections.Counter()

    for u in units:
        wc = u.get("wiring_class") or "ambiguous"
        st = u.get("status") or "unknown"
        kind = u.get("kind") or "unknown"
        # Widen the grid to any word actually observed on a real unit, even
        # if it is absent from WIRING_CLASS_VALUES / status_vocabulary -- a
        # NEW word landing in the corpus must be graded by this audit, not
        # silently skipped because it wasn't in either declared vocabulary.
        wiring_classes.add(wc)
        status_vocab.add(st)
        cell = _cell(wc, st)
        cell_counts[cell] += 1
        cell_counts_by_kind[kind][cell] += 1
        kind_totals[kind] += 1

    if not status_vocab:
        raise SystemExit(
            "reachability_audit: no status_vocabulary on the document and no "
            "units to observe statuses from -- cannot grid"
        )

    # Evaluate _doneness_verdict_uncapped over the FULL grid.
    grid: dict[str, dict] = {}
    for wc in sorted(wiring_classes):
        for st in sorted(status_vocab):
            cell = _cell(wc, st)
            try:
                verdict = producer._doneness_verdict_uncapped(wc, st)
            except ValueError as exc:
                grid[cell] = {
                    "wiring_class": wc, "status": st,
                    "verdict": None, "unmapped": True, "error": str(exc),
                }
            else:
                grid[cell] = {
                    "wiring_class": wc, "status": st,
                    "verdict": verdict, "unmapped": False, "error": None,
                }

    # Per-wiring_class: does ANY status reach `done`?
    wc_has_done_path: dict[str, bool] = {
        wc: any(
            grid[_cell(wc, st)]["verdict"] == producer.DONENESS_DONE
            for st in status_vocab
        )
        for wc in wiring_classes
    }

    dead_end_cells: list[dict] = []
    dead_end_unit_total = 0
    dead_end_by_kind: collections.Counter = collections.Counter()

    for cell in sorted(grid):
        info = grid[cell]
        wc = info["wiring_class"]
        if info["unmapped"]:
            reason = "unmapped"
            detail = info["error"]
        elif info["verdict"] != producer.DONENESS_DONE and not wc_has_done_path[wc]:
            reason = "no-done-path"
            detail = (f"wiring_class {wc!r} has no status in the current grid "
                      f"that reaches {producer.DONENESS_DONE!r}")
        else:
            continue  # this cell IS reachable -- either it's the done cell
                      # itself, or another status for the same wiring_class
                      # reaches done.

        n = cell_counts.get(cell, 0)
        by_kind = {k: c[cell] for k, c in cell_counts_by_kind.items() if c.get(cell)}
        dead_end_cells.append({
            "cell": cell,
            "wiring_class": wc,
            "status": info["status"],
            "reason": reason,
            "unit_count": n,
            "unit_count_by_kind": by_kind,
            "detail": detail,
        })
        dead_end_unit_total += n
        for k, c in by_kind.items():
            dead_end_by_kind[k] += c

    total_units = len(units)
    reachable_ceiling = (
        1.0 if total_units == 0
        else round(1 - dead_end_unit_total / total_units, 6)
    )
    reachable_ceiling_by_kind = {}
    for kind, total in kind_totals.items():
        de = dead_end_by_kind.get(kind, 0)
        reachable_ceiling_by_kind[kind] = (
            round(1 - de / total, 6) if total else None
        )

    unmapped_cells_with_units = [
        d for d in dead_end_cells if d["reason"] == "unmapped" and d["unit_count"] > 0
    ]

    return {
        "source_generated_at": doc.get("generated_at"),
        "excluded_books": sorted(excluded),
        "total_units": total_units,
        "grid_cells_evaluated": len(grid),
        "wiring_classes_evaluated": sorted(wiring_classes),
        "statuses_evaluated": sorted(status_vocab),
        "dead_end_cells": dead_end_cells,
        "dead_end_unit_total": dead_end_unit_total,
        "reachable_ceiling": reachable_ceiling,
        "reachable_ceiling_by_kind": reachable_ceiling_by_kind,
        "unmapped_cells_with_units": unmapped_cells_with_units,
        "ok": len(unmapped_cells_with_units) == 0,
    }


def known_populations(doc: dict, producer=PRODUCER) -> dict:
    """Re-derive (not transcribe -- SD31-E0-F2 acceptance) the known,
    currently-tracked gap populations named in `epic-breakdown.md` Epic 0-F2
    / `decisions.md §4`. These are NOT all grid dead-ends -- `race`/
    `race_trait`'s not-done population spans every wiring_class (a chassis
    absence outside the wiring_class/status model entirely, see Epic 1), so
    the grid audit above cannot see it; it is reported here so the baseline
    run's numbers are self-contained rather than requiring a second,
    separate command to reproduce."""
    excluded = producer.EXCLUDED_BOOKS
    units = [u for u in (doc.get("units") or [])
             if (u.get("book") or "unknown") not in excluded]

    ambiguous = [u for u in units if (u.get("wiring_class") or "ambiguous") == "ambiguous"]
    unknown_status = [u for u in units if (u.get("status") or "unknown") == "unknown"]
    unknown_by_kind = collections.Counter(u.get("kind") or "unknown" for u in unknown_status)

    pop = {
        "ambiguous_wiring_class_units": len(ambiguous),
        "unmeasurable_unknown_status_units": len(unknown_status),
        "unmeasurable_unknown_status_by_kind": dict(unknown_by_kind),
    }
    for kind in ("race", "race_trait"):
        us = [u for u in units if u.get("kind") == kind]
        not_done = sum(
            1 for u in us
            if producer.doneness_verdict(
                u.get("wiring_class") or "ambiguous",
                u.get("status") or "unknown",
                kind,
            ) != producer.DONENESS_DONE
        )
        pop[f"{kind}_total"] = len(us)
        pop[f"{kind}_not_done"] = not_done
    return pop


def _print_report(result: dict, populations: dict) -> None:
    total = result["total_units"]
    reachable = total - result["dead_end_unit_total"]
    ceiling = result["reachable_ceiling"]
    print(f"reachability_audit: {total} units "
          f"(excl. {', '.join(result['excluded_books'])})")
    print(f"  grid: {len(result['wiring_classes_evaluated'])} wiring classes x "
          f"{len(result['statuses_evaluated'])} statuses = "
          f"{result['grid_cells_evaluated']} cells evaluated")
    pct = ceiling * 100 if ceiling is not None else float("nan")
    print(f"  REACHABLE CEILING: {pct:.2f}%  ({reachable} / {total})")
    print("  per-kind reachable ceiling:")
    for kind, v in sorted(result["reachable_ceiling_by_kind"].items()):
        vpct = v * 100 if v is not None else float("nan")
        print(f"    {kind:<20} {vpct:6.2f}%")
    print(f"  dead-end cells: {len(result['dead_end_cells'])}")
    for d in result["dead_end_cells"]:
        print(f"    [{d['reason']:<12}] {d['cell']:<40} units={d['unit_count']:<6} {d['detail']}")
    if result["unmapped_cells_with_units"]:
        print("  FAIL: unmapped cell(s) carry on-board units -- absent from "
              "every rollup:")
        for d in result["unmapped_cells_with_units"]:
            print(f"    {d['cell']}  units={d['unit_count']}")
    print("  known populations (re-derived, not transcribed):")
    for k, v in populations.items():
        print(f"    {k}: {v}")


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__,
                                      formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--inventory", default=str(DEFAULT_INVENTORY),
                         help="path to a work-inventory.json-shaped document")
    parser.add_argument("--json-out", help="also write the machine-readable result here")
    args = parser.parse_args(argv)

    doc = load_inventory(args.inventory)
    result = audit(doc)
    populations = known_populations(doc)
    result["known_populations"] = populations

    _print_report(result, populations)

    if args.json_out:
        with open(args.json_out, "w", encoding="utf-8") as f:
            json.dump(result, f, indent=2, sort_keys=True)
        print(f"  json written to {args.json_out}")

    return 0 if result["ok"] else 1


if __name__ == "__main__":
    sys.exit(main())
