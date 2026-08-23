#!/usr/bin/env python3
"""Card 15 (`census-scope-closure`, `decisions.md` §12b) — per-row disposition
classifier for the `ability_category:*` buckets in
`artifacts/gate-0-census-closure/diff.json`'s `kind_unenumerable`.

WHY THIS EXISTS
----------------
`scripts/census_independent.py` counts every row in a bare (non-`_race`,
non-`_class`) `*abilities*.lst` file whose `CATEGORY:` tag is not `FEAT` into
`kind_unenumerable["ability_category:<tag>"]` (26 tags, 5,886 units at the
pinned oracle SHA). It does not say, for any one of those rows, whether the
row is a real distinct game object or a facet of something already counted
under a tracked kind or another `ability_category:*` row. This script
re-walks the same in-scope book set with the same row-identification rule
(re-using `census_independent`'s own `classify_scope` /
`_classify_kind_by_filename` / `_row_category_tag` / `_parse_lst_rows`, not a
reimplementation of them) and adds three things per row:

1. Which `ability_category:<tag>` bucket it is in (must reproduce
   `diff.json`'s own per-tag counts exactly -- this script's own first
   self-check).
2. Whether the row carries independent mechanical content of its own: a
   `DEFINE:`, `BONUS*:`, `DESC:`, `ASPECT:`, `CSKILL:`, `MOVE:`, `AUTO:`,
   `TEMPLATE:`, `SPROP:`, `QUALITY:`, `SR:`, `DR:`, `SAB:`, or `VISION:`
   token (`_has_content`). A row with none of these carries no game effect
   of its own -- it either grants something else (a gateway row) or exists
   purely as a chooser's pick-list entry (a plain pick-list row).
3. Whether the row's `KEY:` field exactly matches a `KEY:` field already
   counted under a tracked kind (`feat`, `class`, `spell`, `monster`,
   `monster_ability`, `equipment`, `equipment_modifier`, `companion`,
   `race`, `race_trait`) elsewhere in the in-scope corpus (`_key_collision`).
   A shared *identity string* is not proof of a shared thing
   (`decisions.md` "shared name" rule) -- PCGen's own bare-name-as-identity
   convention is scoped per file/kind (a plain-named Special Ability row and
   a plain-named Spell row of the same display text are two different
   objects in two different PCGen databases, not a namespace collision), so
   this check is **KEY:-field-only, never a fallback to bare identity** --
   a row with no explicit `KEY:` field can only be flagged `B-duplicate` if
   some OTHER field of it were used, and this script deliberately does not
   invent one.

USAGE
-----
    python3 15-card-15-ability-category-classify.py \
        --corpus-root <repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data \
        --inventory <repo>/docs/work-inventory.json \
        --output-jsonl <out>.jsonl --output-summary-md <out>.md

Exits non-zero if the per-bucket totals it derives do not match
`diff.json`'s own `kind_unenumerable` counts for every `ability_category:*`
key -- this script fails closed rather than silently drifting from the
census's own numbers (Decision 1a).
"""
from __future__ import annotations

import argparse
import collections
import json
import os
import re
import sys


def _load_census_module(repo_root: str):
    scripts_dir = os.path.join(repo_root, "scripts")
    sys.path.insert(0, scripts_dir)
    import census_independent as ci  # noqa: PLC0415

    return ci


CONTENT_RE = re.compile(
    r"DEFINE:|BONUS[A-Z]*:|DESC:|ASPECT:|CSKILL:|MOVE:|AUTO:|TEMPLATE:|SPROP:|QUALITY:|SR:|DR:|SAB:|VISION:"
)
GATEWAY_RE = re.compile(r"ABILITY:[^\t]+\|AUTOMATIC\|")


def _key_field(line: str):
    for f in line.split("\t"):
        f = f.strip()
        if f.upper().startswith("KEY:"):
            return f.split(":", 1)[1].strip()
    return None


def main(argv=None) -> int:
    p = argparse.ArgumentParser()
    p.add_argument("--repo-root", default=os.getcwd())
    p.add_argument("--corpus-root", required=True)
    p.add_argument("--inventory", required=True)
    p.add_argument("--diff-json", required=True, help="gate-0 diff.json, for the self-check")
    p.add_argument("--output-jsonl", required=True)
    p.add_argument("--output-summary-md", required=True)
    args = p.parse_args(argv)

    ci = _load_census_module(args.repo_root)

    with open(args.inventory, "r", encoding="utf-8") as fh:
        inventory = json.load(fh)

    pathfinder_root = os.path.join(args.corpus_root, "pathfinder")
    scope = ci.classify_scope(ci.discover_book_dirs(args.corpus_root), inventory)

    # Pass 1: every tracked-kind KEY (or bare identity when no KEY: field),
    # exactly as census_independent.py would bucket it -- used only for the
    # cross-kind collision check, never to change how ability_category rows
    # are themselves counted.
    tracked_keys: dict[str, set[str]] = collections.defaultdict(set)
    for bd in scope.in_scope:
        book_dir = os.path.join(pathfinder_root, bd.rel_path)
        for dirpath, _dirnames, filenames in os.walk(book_dir):
            for fn in sorted(filenames):
                if not fn.lower().endswith(".lst"):
                    continue
                full = os.path.join(dirpath, fn)
                bucket, key = ci._classify_kind_by_filename(fn, bd.book_id)
                if bucket == "row_dependent":
                    for identity, raw in ci._parse_lst_rows(full):
                        cat = ci._row_category_tag(raw)
                        if cat and cat.upper() == "FEAT":
                            ident_upper = identity.upper()
                            if ident_upper.endswith((".FORGET", ".MOD")):
                                continue
                            kf = _key_field(raw)
                            if kf:
                                tracked_keys["feat"].add(kf)
                    continue
                if bucket != "kind":
                    continue
                for identity, raw in ci._parse_lst_rows(full):
                    ident_upper = identity.upper()
                    if ident_upper.endswith((".FORGET", ".MOD")):
                        continue
                    kf = _key_field(raw)
                    if kf:
                        tracked_keys[key].add(kf)

    # Pass 2: every ability_category row, with its disposition signals.
    rows = []
    bucket_counts: collections.Counter = collections.Counter()
    for bd in scope.in_scope:
        book_dir = os.path.join(pathfinder_root, bd.rel_path)
        for dirpath, _dirnames, filenames in os.walk(book_dir):
            for fn in sorted(filenames):
                if not fn.lower().endswith(".lst"):
                    continue
                full = os.path.join(dirpath, fn)
                bucket, _key = ci._classify_kind_by_filename(fn, bd.book_id)
                if bucket != "row_dependent":
                    continue
                for identity, raw_line in ci._parse_lst_rows(full):
                    cat = ci._row_category_tag(raw_line)
                    if cat and cat.upper() == "FEAT":
                        continue
                    ident_upper = identity.upper()
                    if ident_upper.endswith((".FORGET", ".MOD")):
                        continue
                    tag = cat or "UNKNOWN"
                    bucket_key = f"ability_category:{tag}"
                    bucket_counts[bucket_key] += 1

                    has_content = bool(CONTENT_RE.search(raw_line))
                    has_gateway = bool(GATEWAY_RE.search(raw_line))
                    key = _key_field(raw_line)
                    collided_kind = None
                    if key:
                        # KEY:-field-only join -- never falls back to bare
                        # identity (see module docstring point 3).
                        for kind, keyset in tracked_keys.items():
                            if key in keyset:
                                collided_kind = kind
                                break

                    if collided_kind:
                        disposition = "B-duplicate"
                    elif has_content:
                        disposition = "A"
                    elif has_gateway:
                        disposition = "B-gateway"
                    else:
                        disposition = "B-picklist"

                    rows.append(
                        {
                            "bucket": bucket_key,
                            "book_id": bd.book_id,
                            "file": os.path.relpath(full, pathfinder_root),
                            "identity": identity,
                            "key": key,
                            "has_content": has_content,
                            "has_gateway": has_gateway,
                            "collided_kind": collided_kind,
                            "disposition": disposition,
                            "line": raw_line,
                        }
                    )

    with open(args.output_jsonl, "w", encoding="utf-8") as fh:
        for r in rows:
            fh.write(json.dumps(r) + "\n")

    # Self-check against diff.json's own kind_unenumerable counts.
    with open(args.diff_json, "r", encoding="utf-8") as fh:
        diff = json.load(fh)
    diff_ac = {
        k: v for k, v in diff["kind_unenumerable"].items() if k.startswith("ability_category:")
    }
    mismatches = []
    for k, v in diff_ac.items():
        if bucket_counts.get(k) != v:
            mismatches.append((k, v, bucket_counts.get(k)))
    for k, v in bucket_counts.items():
        if k not in diff_ac:
            mismatches.append((k, None, v))

    by_bucket_disp: dict[str, collections.Counter] = collections.defaultdict(collections.Counter)
    for r in rows:
        by_bucket_disp[r["bucket"]][r["disposition"]] += 1

    with open(args.output_summary_md, "w", encoding="utf-8") as fh:
        fh.write("# Card 15 ability_category per-row disposition summary\n\n")
        fh.write(
            "Self-check against `diff.json`'s own `kind_unenumerable` "
            f"ability_category counts: {'MATCH' if not mismatches else 'MISMATCH ' + str(mismatches)}\n\n"
        )
        fh.write("| bucket | total | A (real, distinct) | B-gateway (facet/wrapper) | B-picklist (bare chooser value) | B-duplicate (exact KEY match on a tracked kind) |\n")
        fh.write("|---|---:|---:|---:|---:|---:|\n")
        tot = collections.Counter()
        for k in sorted(by_bucket_disp, key=lambda k: -sum(by_bucket_disp[k].values())):
            d = by_bucket_disp[k]
            total = sum(d.values())
            fh.write(
                f"| `{k}` | {total} | {d['A']} | {d['B-gateway']} | {d['B-picklist']} | {d['B-duplicate']} |\n"
            )
            tot.update(d)
        fh.write(
            f"| **TOTAL** | **{sum(tot.values())}** | **{tot['A']}** | **{tot['B-gateway']}** | **{tot['B-picklist']}** | **{tot['B-duplicate']}** |\n"
        )

    print(f"rows: {len(rows)}")
    print(f"self-check: {'MATCH' if not mismatches else 'MISMATCH ' + str(mismatches)}")
    return 1 if mismatches else 0


if __name__ == "__main__":
    raise SystemExit(main())
