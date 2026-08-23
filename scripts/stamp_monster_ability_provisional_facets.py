#!/usr/bin/env python3
"""Stamp `decisions.md §27`'s provisional-facet-default marker onto the
`monster_ability` JSON records `transcribe_monster_tables.py` shipped with a
defaulted `SpecialQuality` facet.

**Why this is a separate step from `transcribe_monster_tables.py`.** That
script writes `monster_data.rs` (a Rust source file `cargo run --bin
gen_book_cache -- <book>` then reads to emit the real `data/corpus/<book>/
monster_ability/*.json` records). The marker fields
(`data.shape_provisional_default`/`data.shape_provisional_reason`) belong on
those JSON records, and `workflow-instruction.md §6a`'s contract is explicit:
the ONLY sanctioned way to write them is `scripts/shape_provisional_marker.py
::stamp_provisional_default`, called on the JSON record itself -- never
written by hand, and never mirrored as a second implementation in Rust. So
this step runs AFTER `gen_book_cache` has produced the files, loads exactly
the records `transcribe_monster_tables.provisional_facet_units(book)` names,
and stamps each one via the sanctioned function.

Usage::

    python3 scripts/transcribe_monster_tables.py <book>
    cargo run --bin gen_book_cache -- <book>
    python3 scripts/stamp_monster_ability_provisional_facets.py <book>

Idempotent: `stamp_provisional_default` itself is (re-stamping with the same
reason changes nothing), and this script's own file-matching is a read +
conditional-write, safe to re-run.
"""
from __future__ import annotations

import glob
import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from shape_provisional_marker import stamp_provisional_default  # noqa: E402
from transcribe_monster_tables import (  # noqa: E402
    BOOKS,
    corpus_output_dir,
    provisional_facet_units,
)


def data_corpus_root() -> str:
    """`data/corpus/`, relative to the repo root this script is run from --
    deliberately NOT `PCGEN_CORPUS_ROOT`/`pcgen_corpus_root()` (that is the
    read-only oracle checkout `transcribe_monster_tables.py` reads `.lst`
    rows from; this script stamps the SHIPPED corpus this repo writes)."""
    return "data/corpus"


def stamp_book(book: str) -> list[tuple[str, str]]:
    """Stamp every `monster_ability` JSON record for `book` that
    `provisional_facet_units` names, matching by `data.corpus_key` (the
    original, un-slugified row identity every emitted record carries,
    exactly the join key `shape_ledger.py` already uses). Returns the
    `(corpus_key, reason)` pairs actually stamped -- fewer than
    `provisional_facet_units(book)`'s own count is a contract violation
    this function surfaces by omission, never by fabricating a match."""
    if book not in BOOKS:
        raise SystemExit(f"unknown book {book!r}; not in transcribe_monster_tables.BOOKS")
    reasons = provisional_facet_units(book)
    if not reasons:
        return []
    ability_dir = os.path.join(data_corpus_root(), corpus_output_dir(book), "monster_ability")
    remaining = dict(reasons)
    stamped: list[tuple[str, str]] = []
    for path in sorted(glob.glob(os.path.join(ability_dir, "*.json"))):
        with open(path, "r", encoding="utf-8") as handle:
            record = json.load(handle)
        corpus_key = (record.get("data") or {}).get("corpus_key")
        if corpus_key not in remaining:
            continue
        reason = remaining.pop(corpus_key)
        stamp_provisional_default(record, reason)
        with open(path, "w", encoding="utf-8") as handle:
            json.dump(record, handle, indent=2, sort_keys=True, ensure_ascii=False)
            handle.write("\n")
        stamped.append((corpus_key, reason))
    if remaining:
        raise SystemExit(
            f"{book}: {len(remaining)} provisional-facet corpus_key(s) named by "
            "provisional_facet_units() have no matching JSON record under "
            f"{ability_dir} -- run `cargo run --bin gen_book_cache -- {book}` first: "
            + ", ".join(sorted(remaining))
        )
    return stamped


def main() -> None:
    if len(sys.argv) != 2:
        raise SystemExit(f"usage: {sys.argv[0]} <book>")
    stamped = stamp_book(sys.argv[1])
    for corpus_key, reason in stamped:
        print(f"{sys.argv[1]}: stamped provisional default ({reason}): {corpus_key}")
    print(f"{sys.argv[1]}: {len(stamped)} record(s) stamped")


if __name__ == "__main__":
    main()
