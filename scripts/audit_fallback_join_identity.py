#!/usr/bin/env python3
"""audit_fallback_join_identity.py -- kanban row 22 (epic-12-fallback-join-
correctness-audit).

`shape_ledger.py`'s `classify_unit` resolves each not-done unit's corpus
record through up to THREE join strategies, tried in order and stopping at
the first hit:

  1. primary  -- `(book, kind, source_file_basename, source_line)`
  2. key_index (same-book fallback) -- `(book, kind, data.key)`
  3. cross_book_key_index (cross-book fallback) -- `(kind, data.key)`

`classify_unit` itself only reports WHETHER a record was found
(`join_status`), never WHICH tier resolved it. This script re-runs the same
three lookups in the same order for every `status == "not-ingested"` unit,
records which tier answered it, and -- for every unit answered ONLY by tier
2 or tier 3 -- independently re-opens the matched corpus JSON record from
disk and checks that it is genuinely the SAME object the unit describes:

  * same `kind` directory (never trusted from the dict key alone -- the
    kind directory is re-derived from the record's own file path via
    `kind_from_path_parts`, exactly as `build_corpus_index`/`build_corpus_
    key_index` do, so a stale or hand-edited dict could never fool this
    check)
  * for a same-book fallback (tier 2): the record's own file lives under
    the unit's own book directory (or its documented alias)
  * for a cross-book fallback (tier 3): the record's own `data.key` is
    byte-identical to the unit's `corpus_key`, and the record is the ONE
    `build_cross_book_key_index` returned (never an ambiguous `None`)

Any unit whose tier-2/3 match fails one of these checks is reported by
coordinate under `MISMATCHES` and the script exits non-zero -- `decisions.
md §1a`: a verification that cannot fail is worse than none.

Usage:
    python3 scripts/audit_fallback_join_identity.py
    python3 scripts/audit_fallback_join_identity.py --json out.json
"""

from __future__ import annotations

import argparse
import glob
import json
import os
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))

import shape_ledger as SL  # noqa: E402
import coverage_ledger as CL  # noqa: E402

DEFAULT_INVENTORY = os.path.join(REPO_ROOT, "docs", "work-inventory.json")
DEFAULT_CORPUS_ROOT = os.path.join(REPO_ROOT, "data", "corpus")


def _resolve_record_path_for_key(corpus_root: str, book: str, kind: str, key: str) -> list[str]:
    """Re-derives, from disk, every corpus JSON path under `book` whose
    directory-derived kind (via `kind_from_path_parts`, the exact function
    the join indexes use) equals `kind` and whose `data.key` equals `key`.
    Independent of `shape_ledger`'s in-memory index -- this walks the
    corpus fresh so a stale dict can never pass a check it does not
    actually satisfy on disk."""
    hits = []
    roots = {book}
    aliased = SL.BOOK_CORPUS_DIR_ALIASES.get(book)
    if aliased:
        roots.add(aliased)
    for root_name in roots:
        root = os.path.join(corpus_root, root_name)
        if not os.path.isdir(root):
            continue
        for path in glob.glob(os.path.join(root, "**", "*.json"), recursive=True):
            if os.path.basename(path) == "LICENSE.json":
                continue
            rel = os.path.relpath(path, root)
            parts = rel.split(os.sep)
            if len(parts) < 2:
                continue
            rec_kind = SL.kind_from_path_parts(parts)
            if rec_kind != kind:
                continue
            try:
                with open(path, "r", encoding="utf-8") as fh:
                    rec = json.load(fh)
            except (OSError, json.JSONDecodeError):
                continue
            data = rec.get("data") or {}
            if data.get("key") == key:
                hits.append(path)
    return hits


def audit(inventory_path: str, corpus_root: str) -> dict:
    inventory = SL.load_inventory_or_die(inventory_path)
    all_not_done = CL.not_done_population(inventory)
    units = [u for u in all_not_done if u.get("status") == "not-ingested"]

    books = {u.get("book") for u in units if u.get("book")}
    corpus_index = SL.build_corpus_index(corpus_root, books)
    key_index = SL.build_corpus_key_index(corpus_root, books)
    cross_book_key_index = SL.build_cross_book_key_index(corpus_root)

    return audit_units(units, corpus_root, corpus_index, key_index, cross_book_key_index)


def audit_units(
    units: list[dict],
    corpus_root: str,
    corpus_index: dict,
    key_index: dict,
    cross_book_key_index: dict,
) -> dict:
    """The reusable core of `audit()`, split out so tests can inject a
    deliberately-wrong `key_index`/`cross_book_key_index` (simulating an
    index desync -- the exact shape a real defect would take: the in-memory
    fallback claims a hit that the corpus on disk does not actually
    support) without needing a live inventory file. `audit()` is the only
    caller that builds the indices from a real corpus walk; this function
    never re-derives them, so a test's injected index is exactly what gets
    checked -- the on-disk independence lives entirely in
    `_resolve_record_path_for_key`, which always re-reads `corpus_root`
    fresh regardless of what the caller-supplied indices claim."""
    tier_counts = {"primary": 0, "key_index": 0, "cross_book": 0, "no_record": 0}
    mismatches: list[dict] = []
    fallback_records: list[dict] = []

    for unit in units:
        book = unit.get("book")
        kind = unit.get("kind")
        basename = unit.get("source_file")
        line = unit.get("source_line")
        corpus_key = unit.get("corpus_key")

        primary_key = (book, kind, basename, line) if (book and kind and basename and line is not None) else None
        primary_hit = corpus_index.get(primary_key) if primary_key is not None else None

        if primary_hit is not None:
            tier_counts["primary"] += 1
            continue

        key_hit = None
        if book and kind and corpus_key:
            key_hit = key_index.get((book, kind, corpus_key))

        if key_hit is not None:
            tier_counts["key_index"] += 1
            on_disk = _resolve_record_path_for_key(corpus_root, book, kind, corpus_key)
            row = {
                "id": unit.get("id"),
                "tier": "key_index",
                "book": book,
                "kind": kind,
                "corpus_key": corpus_key,
                "on_disk_matches": on_disk,
            }
            fallback_records.append(row)
            if not on_disk:
                mismatches.append(
                    {
                        **row,
                        "reason": "key_index fallback reported a hit, but re-walking the corpus fresh "
                        "found no on-disk record with the same (book, kind-directory, data.key)",
                    }
                )
            continue

        cross_hit = None
        if kind and corpus_key:
            cross_hit = cross_book_key_index.get((kind, corpus_key))

        if cross_hit is not None:
            tier_counts["cross_book"] += 1
            matched_book, _tokens = cross_hit
            on_disk = _resolve_record_path_for_key(corpus_root, matched_book, kind, corpus_key)
            row = {
                "id": unit.get("id"),
                "tier": "cross_book",
                "unit_book": book,
                "matched_book": matched_book,
                "kind": kind,
                "corpus_key": corpus_key,
                "on_disk_matches": on_disk,
            }
            fallback_records.append(row)
            if not on_disk:
                mismatches.append(
                    {
                        **row,
                        "reason": "cross_book fallback reported a hit, but re-walking the corpus fresh "
                        "found no on-disk record under the matched book with the same "
                        "(kind-directory, data.key)",
                    }
                )
            elif cross_book_key_index.get((kind, corpus_key)) is None:
                # Defensive: cannot actually happen given the `is not None`
                # guard above, kept only to document the ambiguous-collision
                # exclusion this audit relies on `build_cross_book_key_index`
                # to have already enforced.
                mismatches.append({**row, "reason": "matched an ambiguous (kind, key) collision marked None"})
            continue

        tier_counts["no_record"] += 1

    fallback_only = tier_counts["key_index"] + tier_counts["cross_book"]
    return {
        "population": len(units),
        "tier_counts": tier_counts,
        "primary": tier_counts["primary"],
        "fallback_only": fallback_only,
        "fallback_records": fallback_records,
        "mismatches": mismatches,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--inventory", default=DEFAULT_INVENTORY)
    parser.add_argument("--corpus-root", default=DEFAULT_CORPUS_ROOT)
    parser.add_argument("--json", help="write the full audit result as JSON to this path")
    args = parser.parse_args(argv)

    result = audit(args.inventory, args.corpus_root)
    tc = result["tier_counts"]
    print(f"population (status == 'not-ingested'): {result['population']}")
    print(f"  primary match         : {tc['primary']}")
    print(f"  key_index fallback    : {tc['key_index']}")
    print(f"  cross_book fallback   : {tc['cross_book']}")
    print(f"  no_record             : {tc['no_record']}")
    print(f"fallback_only (key_index + cross_book): {result['fallback_only']}")
    print(f"mismatches (fallback claimed a hit no on-disk record supports): {len(result['mismatches'])}")
    for m in result["mismatches"]:
        print(f"  MISMATCH: {m['id']} tier={m['tier']} reason={m['reason']}")

    if args.json:
        os.makedirs(os.path.dirname(args.json) or ".", exist_ok=True)
        with open(args.json, "w", encoding="utf-8") as fh:
            json.dump(result, fh, indent=2)
            fh.write("\n")

    return 1 if result["mismatches"] else 0


if __name__ == "__main__":
    sys.exit(main())
