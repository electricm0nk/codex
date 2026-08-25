#!/usr/bin/env python3
"""SD-32 `decisions.md §24` — the Codex-generated neutral name for units
whose *own name* is Product Identity (`ability`/`deity`/`class_feature`;
population re-derived per `§17a`, do not trust a stale count without
re-running the ingest that consumes this module).

**Binding condition 1 (`§24b`-1): derived ONLY from non-PI coordinates.**
`(kind, book, source_file, source_line)` — the exact `(book, source_basename,
source_line)` join key `scripts/shape_ledger.py` already uses, plus `kind`
(`§24a`: "and optionally the record's kind"). **Never** from the PI name —
not transformed, not truncated, not hashed.

This is enforced structurally, not just by discipline: every public function
below takes `kind`/`book`/`source_file`/`source_line` and NO `name`, `key`,
free-text, or hash-of-name parameter. There is no argument through which a
PI string could enter the derivation, so "the generator's output is
unchanged when the PI name is replaced with a different string" holds by
construction. `scripts/tests/test_codex_neutral_name.py` proves it anyway,
black-box, by calling this module with two records that share coordinates
and differ only in a (never-passed) original name.

**Binding condition 6: determinism.** Pure string composition of the four
inputs — no randomness, no wall-clock, no hashing. `§24a` itself argues
against a hash ("a hash in particular is a fingerprint that confirms a
guess") in favour of a plainly-derived, self-describing name, so this
module deliberately does NOT hash anything, including the coordinates
themselves.
"""
from __future__ import annotations

import os
import re

# The literal marker every renamed record's `codex_generated_name` field
# carries `True` under, and the human-readable name prefix — both chosen so
# a reader (or a future generator) can recognise a Codex-minted identity on
# sight, per `§24b`-3 ("a field marks it... so no reader or player mistakes
# it for the printed name").
NAME_PREFIX = "Codex-Named Unit"


def _slug(value: str) -> str:
    """Lowercase, `[a-z0-9]+`-runs-joined-by-`_` normalisation. Applied only
    to non-PI coordinate strings (book names, filenames, kind names) — never
    to a record's display name or description."""
    slug = re.sub(r"[^a-z0-9]+", "_", value.lower()).strip("_")
    return slug or "x"


def neutral_coordinate_id(kind: str, book: str, source_file: str, source_line: int) -> str:
    """The stable, PI-free identifier for one unit: `kind_book_basename_line`.
    Two different units can never collide here unless they share the exact
    same `(book, source_file, source_line)` citation, which `v06_work_inventory`'s
    own enumeration already treats as the same object."""
    basename = os.path.basename(source_file)
    return f"{_slug(kind)}_{_slug(book)}_{_slug(basename)}_{int(source_line)}"


def neutral_name(kind: str, book: str, source_file: str, source_line: int) -> str:
    """The Codex-generated display name a renamed record's `data.name`
    carries. Self-describing as generated (contains `NAME_PREFIX`) so even a
    reader who never checks the `codex_generated_name` field sees it is not
    a printed name."""
    coord_id = neutral_coordinate_id(kind, book, source_file, source_line)
    return f"{NAME_PREFIX} ({coord_id})"


def neutral_key(kind: str, book: str, source_file: str, source_line: int) -> str:
    """Same identity, used wherever a record needs a `key` distinct in
    principle from `name` (mirrors the pre-existing generators' key/name
    split) — here the two are the same string, since both would otherwise
    just re-derive the identical coordinate id."""
    return neutral_name(kind, book, source_file, source_line)


def divergence_entry(kind: str, book: str, source_file: str, source_line: int, reason: str) -> dict:
    """`§22`'s divergence-recording, refined by `§24b`-4: record **that** a
    rename happened, its coordinates, and why — **never the original
    string**. This is the one shape every renamed-unit divergence log in
    this bundle must use, so a grep for a PI term across those logs can
    never find one (there is no field here it could be written into)."""
    return {
        "kind": kind,
        "book": book,
        "source_file": os.path.basename(source_file),
        "source_line": int(source_line),
        "codex_name": neutral_name(kind, book, source_file, source_line),
        "reason": reason,
    }
