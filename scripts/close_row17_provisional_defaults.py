#!/usr/bin/env python3
"""Row 17's closing step (`kanban.md` row 17, `epic-7-shape-categorization-100`,
`decisions.md §27`/§27a/§27b) -- applies the genuinely-derived facet each
`monster_ability` record in `transcribe_monster_tables._MONSTER_ABILITY_FACET_OVERRIDES`
was individually re-derived to (see that dict's own docstring for the
per-record evidence) onto the SHIPPED corpus JSON.

**Why this exists, separately from `stamp_monster_ability_provisional_facets.py`.**
`cargo run --bin gen_book_cache -- <book>` is additive-only -- it never
overwrites a record already on disk (`gen_book_cache.rs`'s own "N already on
disk, left untouched" behavior). Re-running the transcribe/gen_book_cache
pipeline over a book whose `monster_ability` records already shipped under
`§27`'s provisional default therefore cannot correct them; this script is
the sanctioned finishing step that does, using
`scripts/shape_provisional_marker.py`'s `clear_provisional_default` (the
same module `§27`'s own contract names as the ONE place these fields are
written) and importing `_MONSTER_ABILITY_FACET_OVERRIDES` from
`transcribe_monster_tables.py` rather than re-deriving the resolution table
a second time (`decisions.md §17`/§26's duplication-drift warning).

**Scope, `close_corpus`.** Only `monster_ability` records whose
`data.corpus_key` matches an entry in `_MONSTER_ABILITY_FACET_OVERRIDES` are
touched. A provisional-default hit with NO matching entry is left exactly
as it is; this script never guesses a resolution for a record its override
table does not name.

**`close_class_feature_corpus` -- the row 17 residual (`occult_adventures`'
`Psychic ~ Phrenic Pool`).** A `class_feature` record's marker names a
different kind of provisional-ness than `monster_ability`'s: not "which
facet family", but "this formula needs a per-character choice input the
compute chassis didn't track yet." There is no facet field to reconcile --
once the compute-side mechanism is real and proven (a real
`chosen_psychic_discipline`-shaped input threaded into
`ground_psychic_class_features`, `src/rules_core/pilot_compute/mod.rs`,
proven per discipline by
`untabled_base_class_feature_roster_wiring_tests::
psychic_phrenic_pool_uses_the_real_ability_for_every_discipline`), the
marker is simply cleared: `_CLASS_FEATURE_PROVISIONAL_RESOLUTIONS` names
which `data.key` values are resolved and why, so a record this table does
not name is left untouched exactly like `close_corpus`'s override table
(same `§27`/`§1a` discipline: never guess a resolution for an unnamed
record). Generic by construction, not a one-record special case: any future
`class_feature` marked provisional for this SAME reason class (a
bloodline/mystery/domain-shaped per-character ability-score choice) closes
the same way once this table names it and the matching compute-side input
is proven -- no second copy of this mechanism is needed
(`decisions.md §17`/§26 duplication-drift discipline).

Usage::

    python3 scripts/close_row17_provisional_defaults.py [--corpus-root PATH] [--dry-run]

Idempotent: re-running after the fields are already cleared is a no-op for
those records (`clear_provisional_default` itself is idempotent).
"""
from __future__ import annotations

import argparse
import glob
import json
import os
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
from shape_provisional_marker import clear_provisional_default  # noqa: E402
from transcribe_monster_tables import _MONSTER_ABILITY_FACET_OVERRIDES  # noqa: E402

DEFAULT_CORPUS_ROOT = os.path.join(REPO_ROOT, "data", "corpus")

# `data.key` -> the resolution note (why the marker is now honestly
# clearable). See this module's own docstring: a `class_feature` provisional
# marker in this reason class closes by proving a real per-character choice
# input reaches the compute chassis, not by picking a different facet.
_CLASS_FEATURE_PROVISIONAL_RESOLUTIONS: dict[str, str] = {
    "Psychic ~ Phrenic Pool": (
        "chosen_psychic_discipline input (choice:psychic_discipline) wired into "
        "ground_psychic_class_features via psychic_discipline_pool_ability "
        "(src/rules_core/pilot_compute/mod.rs), proven per discipline"
    ),
}


def close_corpus(corpus_root: str, dry_run: bool = False) -> list[dict]:
    """Walks every `monster_ability/*.json` record under `corpus_root`,
    resolving any whose `corpus_key` is named in
    `_MONSTER_ABILITY_FACET_OVERRIDES`. Returns one entry per record
    touched: `{path, corpus_key, old_facet, new_facet, was_provisional}`.
    A record named by the override table but NOT carrying the provisional
    marker (already resolved by a prior run, or never defaulted in the
    first place) still has its `facet` field reconciled to the override --
    idempotent either way."""
    touched: list[dict] = []
    for path in sorted(glob.glob(os.path.join(corpus_root, "*", "monster_ability", "*.json"))):
        with open(path, "r", encoding="utf-8") as handle:
            record = json.load(handle)
        data = record.get("data") or {}
        corpus_key = data.get("corpus_key")
        if corpus_key not in _MONSTER_ABILITY_FACET_OVERRIDES:
            continue
        real_facet = _MONSTER_ABILITY_FACET_OVERRIDES[corpus_key]
        old_facet = data.get("facet")
        was_provisional = bool(data.get("shape_provisional_default") is True)
        if old_facet == real_facet and not was_provisional:
            continue  # already resolved, nothing to do
        data["facet"] = real_facet
        clear_provisional_default(record)
        touched.append(
            {
                "path": path,
                "corpus_key": corpus_key,
                "old_facet": old_facet,
                "new_facet": real_facet,
                "was_provisional": was_provisional,
            }
        )
        if not dry_run:
            with open(path, "w", encoding="utf-8") as handle:
                json.dump(record, handle, indent=2, sort_keys=True, ensure_ascii=False)
                handle.write("\n")
    return touched


def close_class_feature_corpus(corpus_root: str, dry_run: bool = False) -> list[dict]:
    """Walks every `class_feature/**/*.json` record under `corpus_root`,
    clearing the provisional marker on any record whose `data.key` is named
    in `_CLASS_FEATURE_PROVISIONAL_RESOLUTIONS` AND currently carries the
    marker. Returns one entry per record touched:
    `{path, key, resolution, was_provisional}`. A record not named by the
    table, or named but not currently provisional, is left untouched and
    not reported -- idempotent, mirrors `close_corpus`'s own contract."""
    touched: list[dict] = []
    for path in sorted(glob.glob(os.path.join(corpus_root, "*", "class_feature", "**", "*.json"), recursive=True)):
        with open(path, "r", encoding="utf-8") as handle:
            record = json.load(handle)
        data = record.get("data") or {}
        key = data.get("key")
        if key not in _CLASS_FEATURE_PROVISIONAL_RESOLUTIONS:
            continue
        was_provisional = bool(data.get("shape_provisional_default") is True)
        if not was_provisional:
            continue  # already resolved, nothing to do
        clear_provisional_default(record)
        touched.append(
            {
                "path": path,
                "key": key,
                "resolution": _CLASS_FEATURE_PROVISIONAL_RESOLUTIONS[key],
                "was_provisional": was_provisional,
            }
        )
        if not dry_run:
            with open(path, "w", encoding="utf-8") as handle:
                json.dump(record, handle, indent=2, sort_keys=True, ensure_ascii=False)
                handle.write("\n")
    return touched


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--corpus-root", default=DEFAULT_CORPUS_ROOT)
    parser.add_argument("--dry-run", action="store_true", help="report what would change, write nothing")
    args = parser.parse_args(argv)

    touched = close_corpus(args.corpus_root, dry_run=args.dry_run)
    for entry in touched:
        reclass = " (RECLASSIFIED)" if entry["old_facet"] != entry["new_facet"] else ""
        print(
            f"{entry['corpus_key']}: {entry['old_facet']} -> {entry['new_facet']}{reclass}, "
            f"provisional marker cleared={entry['was_provisional']}"
        )
    print(f"{len(touched)} monster_ability record(s) {'would be ' if args.dry_run else ''}resolved")

    cf_touched = close_class_feature_corpus(args.corpus_root, dry_run=args.dry_run)
    for entry in cf_touched:
        print(f"{entry['key']}: provisional marker cleared -- {entry['resolution']}")
    print(f"{len(cf_touched)} class_feature record(s) {'would be ' if args.dry_run else ''}resolved")
    return 0


if __name__ == "__main__":
    sys.exit(main())
