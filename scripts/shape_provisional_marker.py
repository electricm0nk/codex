#!/usr/bin/env python3
"""The §27 provisional-shape-default contract (`decisions.md §27`/§27a/§27b,
`kanban.md` row 17, `epic-7-shape-categorization-100`).

**What this exists to prevent.** `decisions.md §27` grants ingest a
provisional `SpecialQuality` default for a delivery-only `TYPE:` row (no
facet segment declared) so the unit stops being `no_record` and its shape
becomes measurable. The ruling's binding second clause: *"Every defaulted
unit must be MARKED as defaulted, distinguishably from a unit whose source
genuinely declares [the] value... A cycle applying this default must emit a
machine-countable marker."* `§1a`'s reason: an unmarked default is
indistinguishable from a real answer and would silently become one.

**The rule for who sets it.** THIS MODULE is the only sanctioned way to
apply a provisional shape default to a corpus record. Every ingest path
that applies `§27`'s default (or ANY other provisional/placeholder shape
assignment `§27a` widens the scope to cover) must call
`stamp_provisional_default` -- never write the raw field name directly.
That is what makes the marker "impossible to set silently": a record
carrying the marker's VALUE without going through this function is a
defect this module's own `audit_corpus_for_unmarked_defaults` (see
`scripts/audit_provisional_shape_defaults.py`) can catch, because the
marker and the value it accompanies are stamped in the same call, never
independently.

**Scope discipline.** This module reads and writes ONLY the marker fields
below. It does not decide WHEN a default should apply (that is `§27`'s own
per-record judgment call, made by the ingest cycle that owns the record's
kind) and it performs no corpus I/O of its own beyond
`scan_corpus_for_provisional_defaults`, a read-only walker used by the row
17 census (`scripts/row17_census.py`). No cycle should ever need a second
copy of this contract -- import it, per `AGENTS.md`'s duplication-drift
warning (`decisions.md §17`/`§26` name the exact failure shape a second
copy of a marker/screen produces).
"""
from __future__ import annotations

import glob
import json
import os

# The two fields this contract stamps, always together, under a record's
# `data` object (the same object `raw_tokens`/`key`/`name` live under).
# `PROVISIONAL_DEFAULT_FIELD` is the machine-countable marker `§27` demands;
# `PROVISIONAL_DEFAULT_REASON_FIELD` is required alongside it -- a marker
# with no stated reason is itself a defect (mirrors `decisions.md §19c`'s
# "a token added without a stated reason is a defect, not a shortcut").
PROVISIONAL_DEFAULT_FIELD = "shape_provisional_default"
PROVISIONAL_DEFAULT_REASON_FIELD = "shape_provisional_reason"


def stamp_provisional_default(record: dict, reason: str) -> dict:
    """The ONE sanctioned way to mark a corpus record's shape assignment as
    a provisional default rather than a source-declared value. Mutates and
    returns `record`. Raises `ValueError` if `reason` is empty -- a marker
    set with no reason is exactly the silent-default shape `§27`/`§1a`
    forbid.

    Idempotent: calling it twice on the same record with the same reason
    leaves the record unchanged (needed for `§24b`-6/determinism-style
    regen discipline -- a second ingest pass over an already-marked record
    must not create drift)."""
    if not reason or not reason.strip():
        raise ValueError(
            "stamp_provisional_default requires a non-empty reason -- "
            "decisions.md §27/§19c: a marker with no stated reason is a "
            "defect, not a shortcut"
        )
    data = record.setdefault("data", {})
    data[PROVISIONAL_DEFAULT_FIELD] = True
    data[PROVISIONAL_DEFAULT_REASON_FIELD] = reason.strip()
    return record


def clear_provisional_default(record: dict) -> dict:
    """The paired, sanctioned counterpart to `stamp_provisional_default` --
    used when `decisions.md §27a`/§27b's final categorization pass has
    determined a defaulted unit's shape IS genuinely correct (a real
    measurement, not a placeholder chosen among several readings). Removes
    both marker fields. Mutates and returns `record`.

    Idempotent: calling it on a record that never carried the marker is a
    no-op, not an error -- the row 17 categorization pass may re-run over
    an already-resolved record without needing to track what it already
    touched."""
    data = record.get("data")
    if not data:
        return record
    data.pop(PROVISIONAL_DEFAULT_FIELD, None)
    data.pop(PROVISIONAL_DEFAULT_REASON_FIELD, None)
    return record


def is_provisional_default(record: dict) -> bool:
    """Reads the marker back. Never raises on a record missing `data` or
    the field entirely -- absence means "not provisional", the correct
    default for the 99.99…% of records this contract does not touch."""
    data = record.get("data") or {}
    return bool(data.get(PROVISIONAL_DEFAULT_FIELD) is True)


def provisional_reason(record: dict) -> str | None:
    data = record.get("data") or {}
    return data.get(PROVISIONAL_DEFAULT_REASON_FIELD)


def scan_corpus_for_provisional_defaults(corpus_root: str, books: set[str] | None = None) -> list[dict]:
    """Read-only walk of `data/corpus/<book>/<kind>/*.json`, returning one
    entry per record carrying the marker: `{book, kind, id_or_key, reason,
    path}`. Never mutates anything. `books`, if given, restricts the walk
    (same convention as `shape_ledger.build_corpus_index`).

    A record found WITH the marker TRUE but a missing/empty reason is
    still reported -- with `reason: None` -- so a caller (the row 17
    census, or a future `--check` gate) can flag it as a contract
    violation rather than silently trusting a malformed marker."""
    hits: list[dict] = []
    if books is not None:
        search_roots = [(b, os.path.join(corpus_root, b)) for b in sorted(books)]
    else:
        search_roots = [(None, corpus_root)]
    for book_override, root in search_roots:
        if not os.path.isdir(root):
            continue
        for path in glob.glob(os.path.join(root, "**", "*.json"), recursive=True):
            if os.path.basename(path) == "LICENSE.json":
                continue
            try:
                with open(path, "r", encoding="utf-8") as fh:
                    rec = json.load(fh)
            except (OSError, json.JSONDecodeError):
                continue
            if not is_provisional_default(rec):
                continue
            rel = os.path.relpath(path, root)
            parts = rel.split(os.sep)
            if book_override is not None:
                # `root` IS this book's own directory, so `parts[0]` is the
                # kind directory directly.
                book = book_override
                kind = parts[0] if len(parts) >= 1 else None
            else:
                # `root` is the whole corpus; `parts[0]` is the book
                # directory, `parts[1]` the kind directory.
                book = parts[0] if len(parts) >= 1 else None
                kind = parts[1] if len(parts) >= 2 else None
            data = rec.get("data") or {}
            hits.append(
                {
                    "book": book,
                    "kind": kind,
                    "id_or_key": data.get("key") or data.get("id"),
                    "reason": provisional_reason(rec),
                    "path": path,
                }
            )
    return hits
