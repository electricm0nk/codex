#!/usr/bin/env python3
"""SD-32 T9-onboarding-cause-closure — the ONE shared implementation of
`scrub_name_pi_tokens`, extracted the way `scripts/codex_neutral_name.py`
was extracted, after two independently-maintained copies (`ingest_ability.py`,
`ingest_generic_kind.py`) drifted and shipped a real leak.

**The leak this fixes (found live, corpus-wide cross-check).**
`ingest_generic_kind.py`'s copy already added an alphanumeric-normalized
substring check for a record's OWN name/key being concatenated into another
token's value with no separator (`RedMantisAssassinLVL`), because PCGen's
`DEFINE`/`BONUS` tokens frequently build a variable identifier that way. The
`ingest_ability.py` copy this was forked from was **never updated** with that
fix, and 576 already-shipped `ability` records went through the unfixed copy.

**A second, more severe instance of the same defect class was found doing
the honest re-derivation this cycle was scoped to do (`decisions.md §17a`):
the 60-term `PI_BLACKLIST_TERMS` scan (`normalized_term_hit`) has the
IDENTICAL word-boundary blind spot for a BLACKLISTED term (not just the
record's own name) concatenated PascalCase-style into another token's value**
— e.g. a `TYPE:` token naming a choice-ability variable by concatenating a
deity's name directly onto a suffix with no separator. `normalized_term_hit`
requires `(?<![a-z0-9])term(?![a-z0-9])`, which is deliberately word-bounded
to avoid a short-blacklist-term-inside-an-ordinary-word false-positive
`ogl-pi-blacklist.md §4` already recorded — but that same boundary rule means a value like
`"<Deity>AspectChoice"` never matches, because the character immediately
after the deity's name is a letter, not a boundary. This was found live in
an ALREADY-SHIPPED, ALREADY-RENAMED `ability` record (a §24 `codex_named_unit_*`
file) whose `TYPE` token still carried the deity's name in plain text despite
the record's `NAME`/`DESC` having been correctly redacted — the exact shape
`decisions.md §24` exists to prevent (`§24b`-2: "the PI original appears
nowhere that ships"). This module fixes BOTH shapes with the SAME
alphanumeric-normalized substring technique, bounded to needles/terms of at
least 6 normalized characters so a short, generic string (an abbreviation, a
3-4-letter blacklist term already covered separately by the word-bounded
scan) cannot over-redact by coincidence.

**Both checks are additive, never a replacement.** The original
space-preserving substring check and the original word-bounded
`normalized_term_hit` scan still run first; the alphanumeric-normalized scan
only adds coverage for the concatenated-identifier shape neither of those two
can see.

Every ingest path that redacts a record's raw tokens for Product Identity
must import `scrub_name_pi_tokens` from HERE — never re-define it locally.
`decisions.md §17` names this exact duplication-drift shape as the failure
`§24b`'s screen must not repeat.
"""
from __future__ import annotations

import os
import re
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from sd32_t9_pi_review_feat_equipment import (  # noqa: E402
    PI_BLACKLIST_TERMS,
    normalized_term_hit,
)

# `src/rules_core/shape_b_v1.rs::REDACTED_PI_MARKER` / `PI_MARKER_REDACTED` —
# the same literal every generator in this program uses.
REDACTED_PI_MARKER = "[redacted PI]"
PI_MARKER_REDACTED = "redacted"

# The minimum normalized-character length a needle/term must reach before the
# alphanumeric-normalized (no-separator) check applies to it. Below this
# bound, a short generic word or abbreviation risks over-redacting on
# coincidence (`decisions.md §24b`'s own "Abb"/"RMA" test case). The
# word-bounded `normalized_term_hit` scan above still covers short blacklist
# terms (the 3-4 normalized-character entries in `PI_BLACKLIST_TERMS`) at
# their ordinary, separated occurrences.
_MIN_NORMALIZED_NEEDLE_LEN = 6


def _normalize(s: str) -> str:
    return re.sub(r"[^a-z0-9]", "", s.lower())


_NORM_BLACKLIST_TERMS: list[tuple[str, str]] = [
    (term, _normalize(term))
    for term in PI_BLACKLIST_TERMS
    if len(_normalize(term)) >= _MIN_NORMALIZED_NEEDLE_LEN
]


def blacklist_term_hit_including_concatenated(value: str) -> str | None:
    """`normalized_term_hit(value)` (word-bounded, OCR-normalized), OR — if
    that finds nothing — an alphanumeric-normalized (no-separator) substring
    match against `PI_BLACKLIST_TERMS`, bounded to `_MIN_NORMALIZED_NEEDLE_LEN`
    normalized characters.

    This is the SAME check-4 the module docstring above describes, exposed as
    its own function so every caller that scans a token value against the
    blacklist — not only `scrub_name_pi_tokens`'s renamed-record branch — gets
    the concatenated-term coverage. `normalized_term_hit` alone cannot see a
    blacklisted term concatenated PascalCase-style into another token's value
    (e.g. a `TYPE:` token naming a variable by joining a blacklisted term
    directly onto a suffix with no separator) because its word-boundary
    requirement fails the moment the term is immediately followed by another
    letter — found live in an already-shipped record whose `TYPE:` token
    carried exactly this shape, un-redacted, on an otherwise-clean (non-
    renamed) record.

    Returns the matched term, or `None`."""
    if not value:
        return None
    hit = normalized_term_hit(value)
    if hit:
        return hit
    norm_value = _normalize(value)
    if not norm_value:
        return None
    for term, canon_term in _NORM_BLACKLIST_TERMS:
        if canon_term in norm_value:
            return term
    return None


def scrub_name_pi_tokens(
    tokens: list[dict[str, str]], name: str, key: str
) -> tuple[list[dict[str, str]], bool]:
    """`decisions.md §24b`-2: "The PI original appears nowhere that ships."

    A record whose NAME is PI can carry that same name again inside another
    token's VALUE (most concretely a `KEY:` field that restates the row's own
    full original key verbatim, or a `~`-delimited segment of `key`, since a
    `'<Concept> ~ <Deity>'`-shaped key embeds the PI term as one segment, not
    the whole string). `name`/`key` are used ONLY to build the redaction
    needle set here — they are never written into the returned record.

    Returns `(scrubbed_tokens, any_redacted)`. Never mutates the input.

    Four independent checks run per token value, any one of which redacts it:

    1. The word-bounded, OCR-normalized 60-term blacklist scan
       (`normalized_term_hit`) — catches an ordinary, separated occurrence of
       a blacklisted deity/place name.
    2. A space-preserving case-insensitive substring check against the
       record's own `name`/`key` (and `~`-split / whitespace-split segments
       of `key`) — catches an ordinary, separated occurrence of the record's
       own PI name.
    3. **Identifier-form check for the record's own identity.** PCGen's own
       `DEFINE`/`BONUS` tokens frequently concatenate the record's name into
       a variable identifier with no separator at all (`RedMantisAssassinLVL`,
       `WestcrownDevilLVL`) — a space-preserving substring check never matches
       a value with no spaces. Every needle from check 2 is therefore ALSO
       checked in a fully alphanumeric-normalized form (`[^a-z0-9]` stripped)
       against the same normalization of the value, bounded to needles of at
       least `_MIN_NORMALIZED_NEEDLE_LEN` normalized characters.
    4. **Identifier-form check for the 60-term blacklist.** The identical
       concatenation shape, but for a BLACKLISTED term rather than the
       record's own identity — e.g. a `TYPE:` token naming a choice-ability
       variable `"<Deity>AspectChoice"` with no separator. `normalized_term_hit`
       (check 1) cannot see this: its word-boundary requirement fails the
       moment the term is directly followed by another letter. Bounded the
       same way as check 3, using `_NORM_BLACKLIST_TERMS`.
    """
    needles: set[str] = set()
    norm_needles: set[str] = set()

    def add_needle(s: str) -> None:
        s = s.strip()
        if not s:
            return
        needles.add(s.lower())
        normalized = _normalize(s)
        if len(normalized) >= _MIN_NORMALIZED_NEEDLE_LEN:
            norm_needles.add(normalized)

    for s in (name, key):
        add_needle(s)
    if key:
        for segment in re.split(r"\s*~\s*", key):
            add_needle(segment)
            for word in re.split(r"[\s()]+", segment):
                add_needle(word)

    scrubbed = []
    any_redacted = False
    for t in tokens:
        value = t["value"]
        if not value:
            scrubbed.append(dict(t))
            continue

        norm_value = _normalize(value)
        value_lower = value.lower()

        blacklist_hit = blacklist_term_hit_including_concatenated(value)  # checks 1+4
        identity_hit = any(needle in value_lower for needle in needles)  # check 2
        identity_hit = identity_hit or (  # check 3
            bool(norm_value) and any(n in norm_value for n in norm_needles)
        )

        if blacklist_hit or identity_hit:
            scrubbed.append({"key": t["key"], "value": REDACTED_PI_MARKER})
            any_redacted = True
        else:
            scrubbed.append(dict(t))
    return scrubbed, any_redacted
