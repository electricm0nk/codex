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

**Scope.** Only `monster_ability` records whose `data.corpus_key` matches an
entry in `_MONSTER_ABILITY_FACET_OVERRIDES` are touched. A provisional-
default hit with NO matching entry (e.g. `occult_adventures`'
`Psychic ~ Phrenic Pool`, a `class_feature` genuinely marked for a different
reason -- a per-character discipline choice, not a facet-classification
gap) is left exactly as it is; this script never guesses a resolution for a
record its override table does not name.

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
    print(f"{len(touched)} record(s) {'would be ' if args.dry_run else ''}resolved")
    return 0


if __name__ == "__main__":
    sys.exit(main())
