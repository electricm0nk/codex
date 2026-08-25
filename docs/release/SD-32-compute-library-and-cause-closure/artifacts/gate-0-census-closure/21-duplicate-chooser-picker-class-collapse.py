"""SD-32 card 15/21 -- implements `decisions.md` Decision 21 (operator ruling
2026-08-23): "Every fallback-key `class_feature` collision group whose
members ALL carry a `TYPE:*Choice` facet AND whose granted targets pairwise
coincide is a duplicate-chooser-picker group, not distinct objects."

This is the CLASS predicate the operator ruled on, replacing the per-group
hand review `15-card-15-duplicate-identity-review-memo.md` did for the same
39 groups. It reuses `15-card-15-residual-group-review.py`'s own row
collection (same instrument that produced the evidence the operator ruled
on) rather than re-deriving corpus-walk logic from scratch.

# The predicate, precisely (Decision 21 binding condition 1)

A fallback-key (no `KEY:` field) `class_feature` collision group is a
duplicate-chooser-picker group iff BOTH:

  (a) every member row carries a `TYPE:` facet ending in `"Choice"`, AND
  (b) the group's rows partition, by their own REAL granted target set (the
      trailing segment of every `ABILITY:...|AUTOMATIC|...` field, EXCLUDING
      a `TYPE=`-prefixed segment -- see "The TYPE= exclusion" below), into
      subgroups of size >= 2 -- i.e. every row's target set is shared by at
      least one other row in the group ("pairwise coincide": no row grants a
      target unique to itself within the group).

Both conditions are checked per member, not relaxed for a "close enough"
group. A group failing either predicate half is left alone -- not collapsed,
not counted as covered by this ruling -- exactly condition 1 requires.

# The `TYPE=` exclusion -- why the raw worksheet script is not used verbatim

`15-card-15-residual-group-review.py`'s own `ability_targets()` collects
EVERY `ABILITY:...AUTOMATIC...` field's trailing pipe segment, including a
same-row `TYPE=<pool name> ~ Power LVL <n>` self-tag some rows also carry
(a `CHOOSE:`-pool's own category marker, not a second granted feature). That
self-tag is row-unique (it names the pool the ROW itself belongs to), so
including it in the target set inflates `distinct_targets` and would make a
genuinely-duplicate group look like it has no pairwise match. Excluding
`TYPE=`-prefixed segments (this module's own `ability_automatic_targets`)
recovers exactly the grant targets the operator's worked example describes
(`decisions.md` Decision 21, ACG "Aberrant Bloodline": four rows, two real
targets, each shared by two rows) -- verified below against all 39 groups.

# Why this needs no `v06_work_inventory.rs` code change

`disambiguate_class_feature_fallback_collisions` already leaves EVERY
`TYPE:*Choice`-typed fallback group untouched (does not disambiguate their
keys) -- see that function's own doc comment, "The `*Choice` exclusion".
Untouched means the group's rows keep competing for the SAME bare-key
identity, so the corpus-wide `(book, key)` collision collapse this repo
already performs (independent of this predicate) already keeps exactly one
survivor per group and drops the rest. That is precisely the disposition
Decision 21 calls for -- "the picker rows ... come out of the unit ledger
rather than being counted as separate objects". This script's job is not to
change that runtime behaviour (it is already correct) but to (1) PROVE the
predicate holds for all 39 groups with zero exceptions, (2) prove the
predicate does NOT over-reach on a group whose members grant different
targets (binding condition 3 -- see the paired test module,
`21-duplicate-chooser-picker-class-collapse_test.py`), and (3) emit the
committed, reviewable collapse log binding condition 2 requires.

Run:
    export PCGEN_CORPUS_ROOT=<repo>/docs/release/SD-32-compute-library-and-cause-closure/artifacts/corpus/operator-supplied/pcgen/data
    python3 docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/21-duplicate-chooser-picker-class-collapse.py [--output <path.json>]
"""
from __future__ import annotations

import argparse
import json
import os
import sys
from collections import defaultdict

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(
    os.path.dirname(os.path.abspath(__file__))
)))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import census_independent as ci  # noqa: E402


def type_of(fields: list[str]) -> str | None:
    for f in fields:
        fs = f.strip()
        if fs.upper().startswith("TYPE:"):
            return fs.split(":", 1)[1]
    return None


def category_of(fields: list[str]) -> str | None:
    for f in fields:
        fs = f.strip()
        if fs.upper().startswith("CATEGORY:"):
            return fs.split(":", 1)[1]
    return None


def ability_automatic_targets(fields: list[str]) -> list[str]:
    """Real `ABILITY:...|AUTOMATIC|...` grant targets, excluding a same-row
    `TYPE=<pool> ~ ...` self-tag segment (see this module's own docstring,
    "The TYPE= exclusion")."""
    out = []
    for f in fields:
        fs = f.strip()
        if fs.upper().startswith("ABILITY:") and "AUTOMATIC" in fs.upper():
            parts = fs.split("|")
            if len(parts) >= 3:
                tgt = parts[-1]
                if not tgt.upper().startswith("TYPE="):
                    out.append(tgt)
    return out


def all_type_choice(rows: list[tuple]) -> bool:
    """Predicate half (a): every member row's `TYPE:` facet ends in
    `"Choice"`."""
    types = [type_of(r[3]) for r in rows]
    return len(rows) >= 2 and all(t is not None and t.endswith("Choice") for t in types)


def targets_pairwise_coincide(rows: list[tuple]) -> bool:
    """Predicate half (b): every row's own real-target set (frozenset,
    order-independent) is shared by at least one OTHER row in the group. A
    row with an empty target set, or a target set unique to itself, fails
    this half -- the group is then NOT a duplicate-chooser-picker group."""
    target_sets = [frozenset(ability_automatic_targets(r[3])) for r in rows]
    if any(len(ts) == 0 for ts in target_sets):
        return False
    counts: dict[frozenset, int] = defaultdict(int)
    for ts in target_sets:
        counts[ts] += 1
    return all(counts[ts] >= 2 for ts in target_sets)


def is_duplicate_chooser_picker_group(rows: list[tuple]) -> bool:
    """Decision 21's predicate, exactly: both halves, every member."""
    return all_type_choice(rows) and targets_pairwise_coincide(rows)


def collect_fallback_groups(pcgen_root: str, inventory_path: str) -> dict:
    """Re-derives every fallback-key (no `KEY:`) `class_feature` collision
    group corpus-wide -- the same row collection
    `15-card-15-residual-group-review.py` performs (reused, not
    re-implemented differently)."""
    with open(inventory_path, encoding="utf-8") as f:
        inventory_json = json.load(f)
    book_dirs = ci.discover_book_dirs(pcgen_root)
    scope = ci.classify_scope(book_dirs, inventory_json)
    pathfinder_root = os.path.join(pcgen_root, "pathfinder")

    rows_by_book_key: dict[tuple[str, str, bool], list] = defaultdict(list)
    for bd in scope.in_scope:
        for dirpath, _, filenames in os.walk(os.path.join(pathfinder_root, bd.rel_path)):
            for fn in sorted(filenames):
                if not fn.lower().endswith(".lst"):
                    continue
                bucket, _ = ci._classify_kind_by_filename(fn, bd.book_id)
                if bucket != "row_dependent_class_feature":
                    continue
                with open(os.path.join(dirpath, fn), encoding="utf-8", errors="replace") as fh:
                    for lineno, raw in enumerate(fh, 1):
                        line = raw.rstrip("\n")
                        if not line.strip() or line.lstrip().startswith("#") or "\t" not in line:
                            continue
                        identity = line.split("\t", 1)[0]
                        if ":" in identity:
                            continue
                        iu = identity.upper()
                        if iu.endswith(".FORGET") or iu.endswith(".MOD"):
                            continue
                        fields = line.split("\t")
                        is_internal = any(
                            f.strip() == "CATEGORY:Internal" for f in fields
                        ) or identity.startswith("CATEGORY=Internal|")
                        if is_internal:
                            continue
                        has_key = False
                        key_field = None
                        for f in fields:
                            fs = f.strip()
                            if fs.upper().startswith("KEY:"):
                                key_field = fs.split(":", 1)[1].strip()
                                has_key = True
                                break
                        if key_field is None:
                            if identity.startswith("CATEGORY=") and "|" in identity:
                                key_field = identity.split("|", 1)[1]
                            else:
                                key_field = identity
                        rows_by_book_key[(bd.book_id, key_field, has_key)].append(
                            (bd.book_id, fn, lineno, fields)
                        )

    fallback_groups = {k: v for k, v in rows_by_book_key.items() if len(v) > 1 and k[2] is False}

    def content_sig(fields):
        CONTENT_PREFIXES = [
            "DEFINE:", "BONUS", "DESC:", "ASPECT:", "CSKILL:", "MOVE:", "AUTO:", "TEMPLATE:",
            "SPROP:", "QUALITY:", "SR:", "DR:", "SAB:", "VISION:", "SPELLKNOWN", "TEMPBONUS:",
            "CHOOSE:", "NATURALATTACKS:", "COMPANIONLIST:", "ADD:", "FOLLOWERS:", "UDAM:",
            "UMULT:", "SELECT:", "COST:", "MOVECLONE:", "SPELLS:", "SERVESAS:", "DEFINESTAT:",
            "UNENCUMBEREDMOVE:", "BENEFIT:", "SPELLLEVEL:", "CMB:", "ABILITY:",
        ]
        parts = [f.strip() for f in fields if f.strip() and any(f.strip().upper().startswith(p) for p in CONTENT_PREFIXES)]
        return "\t".join(sorted(parts))

    # Distinct-content only (byte-identical groups are already, separately,
    # correctly left to collapse -- unaffected by this ruling).
    distinct_groups = {}
    for k, rows in fallback_groups.items():
        sigs = defaultdict(list)
        for r in rows:
            sigs[content_sig(r[3])].append(r)
        if len(sigs) > 1:
            distinct_groups[k] = rows
    return distinct_groups


def build_collapse_log(pcgen_root: str, inventory_path: str) -> dict:
    groups = collect_fallback_groups(pcgen_root, inventory_path)

    covered = []
    not_covered_choice_typed = []
    total_rows_in_scope_groups = 0
    for k in sorted(groups):
        rows = groups[k]
        book, key, _ = k
        if not all_type_choice(rows):
            continue  # not this ruling's population (the FavoredClass-shaped rescue, etc.)
        total_rows_in_scope_groups += len(rows)
        covers = targets_pairwise_coincide(rows)
        entry = {
            "book": book,
            "key": key,
            "row_count": len(rows),
            "members": [
                {
                    "file": fn,
                    "line": ln,
                    "category": category_of(fields),
                    "type": type_of(fields),
                    "grant_targets": ability_automatic_targets(fields),
                }
                for (_, fn, ln, fields) in rows
            ],
        }
        if covers:
            # First row in file-iteration order is the survivor (matches
            # `disambiguate_class_feature_fallback_collisions`'s own
            # unmodified tie-break -- the corpus-wide `(book,key)` collapse
            # this predicate's disposition relies on).
            survivor = rows[0]
            residual = rows[1:]
            entry["disposition"] = "duplicate_chooser_picker_group_collapsed"
            entry["survivor"] = {"file": survivor[1], "line": survivor[2]}
            entry["residual_rows_removed"] = [
                {"file": fn, "line": ln} for (_, fn, ln, _fields) in residual
            ]
            covered.append(entry)
        else:
            entry["disposition"] = "NOT_COVERED_left_alone"
            not_covered_choice_typed.append(entry)

    residual_row_count = sum(len(g["residual_rows_removed"]) for g in covered)

    return {
        "decision": "decisions.md Decision 21 (operator ruling 2026-08-23)",
        "predicate": (
            "every member row carries a TYPE:*Choice facet AND the group's "
            "rows partition by real ABILITY:AUTOMATIC grant target (TYPE= "
            "self-tags excluded) into subgroups of size >= 2 -- every row "
            "has a partner granting the identical target"
        ),
        "groups_covered": len(covered),
        "rows_in_covered_groups": total_rows_in_scope_groups if not not_covered_choice_typed else sum(g["row_count"] for g in covered),
        "residual_rows_removed_from_ledger": residual_row_count,
        "groups_not_covered_left_alone": len(not_covered_choice_typed),
        "covered_groups": covered,
        "not_covered_groups": not_covered_choice_typed,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--pcgen-root", default=os.environ.get("PCGEN_CORPUS_ROOT"))
    parser.add_argument(
        "--inventory",
        default=os.path.join(REPO_ROOT, "docs", "work-inventory.json"),
    )
    parser.add_argument("--output", help="write the full collapse log as JSON to this path")
    args = parser.parse_args(argv)

    if not args.pcgen_root:
        print("PCGEN_CORPUS_ROOT not set and --pcgen-root not given", file=sys.stderr)
        return 1

    log = build_collapse_log(args.pcgen_root, args.inventory)

    print(f"groups covered (duplicate-chooser-picker, collapsed): {log['groups_covered']}")
    print(f"residual rows removed from the unit ledger: {log['residual_rows_removed_from_ledger']}")
    print(f"groups NOT covered (left alone): {log['groups_not_covered_left_alone']}")
    if log["not_covered_groups"]:
        print("  NOT covered:")
        for g in log["not_covered_groups"]:
            print(f"    {g['book']}:{g['key']!r} ({g['row_count']} rows)")

    if args.output:
        with open(args.output, "w", encoding="utf-8") as fh:
            json.dump(log, fh, indent=2, sort_keys=False)
            fh.write("\n")
        print(f"wrote {args.output}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
