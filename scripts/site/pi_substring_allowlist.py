#!/usr/bin/env python3
"""scripts/site/pi_substring_allowlist.py -- the REVIEWED, per-entry
allow-list of published item names that embed a declared-PI proper noun as
a freestanding word (`pi_redaction.find_declared_pi_word_matches`) but are
judged, on a case-by-case reviewed basis, mundane rather than a genuine
disclosure of Product Identity.

BACKGROUND. `build_public_status.py`'s `name` screen now catches every
freestanding word-boundary embed of a declared-PI name, GLOBALLY (not
book-scoped -- see `pi_redaction.find_declared_pi_word_matches`'s own
docstring for why book-scoping alone missed cross-book deity references
like `"Death (Pharasma)"`). That widening is deliberately over-inclusive:
`"Shackles"` is both a declared-PI Golarion region AND an ordinary English
word for manacles, and an item plainly using the ordinary meaning
(`"Shackles of Compliance"`) should not vanish from a public feed just
because a homonym exists. This file is where that judgment call is made
explicit, one name at a time, instead of silently baked into the screening
code -- SITE-PI-ALLOWLIST-001, 2026-08-17 operator ruling (option C):
redact the one genuine leak found this cycle (`"Death (Pharasma)"`, via the
generalised mechanism above, not hard-coded here), keep the other 11
published, but ONLY behind this reviewed list.

THIS LIST MUST STAY SHORT. Every entry here is a place a real leak could
hide behind "we looked at that once" -- the operator's own words. Before
adding an entry:

  1. Read the actual corpus row(s) for the name (description, source
     page, mechanical effect) -- not just the name in isolation. A name
     that merely CONTAINS an ordinary English homonym of a declared-PI
     word (manacles/"Shackles", underground/"Darklands") is a good
     candidate. A name whose own description is substantively ABOUT the
     declared-PI thing (a deity's clergy, a nation's specific lore) is
     not -- redact it instead, the way `"Death (Pharasma)"` was.
  2. Write a reason a reviewer with no other context can check against
     that row. `test_pi_substring_allowlist_has_reasons` (in
     `scripts/tests/test_build_public_status.py`) fails the build if any
     entry's reason is blank -- this is enforced, not just asked for.
  3. Re-read every EXISTING entry in this file whenever you add a new
     one. A list that only ever grows and is never re-checked is exactly
     the hiding place the operator flagged.

WHY KEYED ON THE FULL NAME, NOT THE TERM. An entry clears the EXACT string
`"Dimensional Shackles"`, never the bare word `"Shackles"` -- allow-listing
the term itself would silently wave through a genuinely PI-shaped future
name like `"Shackles of the Runelord"` the moment it is published, with
no review at all. `books` further narrows an entry to the printing(s) it
was actually reviewed for: the same name string appearing in a book NOT
listed here is not covered and is redacted like any other unreviewed hit.
"""
from __future__ import annotations

# Each entry: the exact published `name` string, the declared-PI term it
# embeds (documentation only -- matching is by `name`+`books`, never by
# `term` alone), the book id(s) this review covers, and a one-line reason
# a reviewer can check against the corpus row itself.
ALLOWLIST: list[dict] = [
    {
        "name": "Shackles of Durance Vile",
        "term": "Shackles",
        "books": ["advanced_race_guide", "ultimate_equipment"],
        "reason": (
            "Wondrous-item manacles (SPROP: dominate-person effect on a "
            "bound prisoner) -- \"shackles\" is the ordinary English word "
            "for restraints; unrelated to the Shackles pirate-isle region."
        ),
    },
    {
        "name": "Dimensional Shackles",
        "term": "Shackles",
        "books": ["core_rulebook"],
        "reason": (
            "Wondrous-item wrist restraints (dimensional-anchor binding "
            "effect) -- same ordinary \"manacles\" meaning as Shackles of "
            "Durance Vile, not the region."
        ),
    },
    {
        "name": "Shackles of Compliance",
        "term": "Shackles",
        "books": ["ultimate_equipment"],
        "reason": (
            "Another ordinary restraint-type wondrous item (Skull & "
            "Shackles adventure-path gear); \"shackles\" used literally, "
            "not as the region name."
        ),
    },
    {
        "name": "Leashed Shackles",
        "term": "Shackles",
        "books": ["ultimate_magic"],
        "reason": (
            "A conjuration/force spell that manifests literal restraining "
            "shackles on the target; ordinary meaning, not the region."
        ),
    },
    {
        "name": "Darklands Goggles",
        "term": "Darklands",
        "books": ["ultimate_equipment"],
        "reason": (
            "Eyewear granting darkvision plus Perception/Survival bonuses "
            "\"Underground\" -- \"Darklands\" used descriptively for a dark, "
            "underground environment, not narrative content about the "
            "region itself."
        ),
    },
    {
        "name": "Darklands Stalker",
        "term": "Darklands",
        "books": ["advanced_race_guide"],
        "reason": (
            "A drow racial trait for moving through difficult terrain "
            "\"while underground\" -- same descriptive-environment usage "
            "as Darklands Goggles, not region-specific lore."
        ),
    },
    {
        "name": "Ulfen Guard",
        "term": "Ulfen",
        "books": ["inner_sea_combat"],
        "reason": (
            "Prestige-class title for guards of Ulfen descent -- an "
            "ethnicity-of-origin naming convention (cf. real-world \"Swiss "
            "Guard\"), the class mechanics are not themselves PI narrative "
            "content."
        ),
    },
    {
        "name": "Lastwall Banner (Harchist)",
        "term": "Lastwall",
        "books": ["inner_sea_combat"],
        "reason": (
            "Wondrous banner item named for a Lastwall military order; "
            "place-of-origin naming on ordinary equipment, same pattern as "
            "the other two Lastwall Banner variants."
        ),
    },
    {
        "name": "Lastwall Banner (Hordeline)",
        "term": "Lastwall",
        "books": ["inner_sea_combat"],
        "reason": "Same Lastwall Banner item family; see the Harchist entry's reason.",
    },
    {
        "name": "Lastwall Banner (Sunwall)",
        "term": "Lastwall",
        "books": ["inner_sea_combat"],
        "reason": "Same Lastwall Banner item family; see the Harchist entry's reason.",
    },
]


def build_allowlist_index() -> dict[str, dict]:
    """`{name: entry}`, validated. Raises `ValueError` (fail loud, at
    import/build time, never a silent pass-through) if any entry is
    missing its name/books/reason, carries a blank reason, or duplicates
    another entry's name -- this is the enforcement half of "the list
    must stay short and every entry must be checkable," not just the
    documentation half above."""
    index: dict[str, dict] = {}
    for entry in ALLOWLIST:
        name = entry.get("name")
        if not isinstance(name, str) or not name.strip():
            raise ValueError(f"pi_substring_allowlist entry has no valid 'name': {entry!r}")
        if name in index:
            raise ValueError(f"pi_substring_allowlist has a duplicate entry for name {name!r}")
        books = entry.get("books")
        if not isinstance(books, list) or not books or not all(isinstance(b, str) and b for b in books):
            raise ValueError(f"pi_substring_allowlist entry {name!r} needs a non-empty 'books' list")
        reason = entry.get("reason")
        if not isinstance(reason, str) or not reason.strip():
            raise ValueError(
                f"pi_substring_allowlist entry {name!r} has no reason -- every entry must carry "
                "a one-line reason a reviewer can check; see this file's module docstring"
            )
        index[name] = entry
    return index


def is_allowlisted(name: str, book: str, index: dict[str, dict] | None = None) -> bool:
    """True if `(name, book)` exactly matches a reviewed allow-list entry.
    Matching requires BOTH the full name string and the book id -- see the
    module docstring for why this is deliberately not keyed on the
    embedded term alone."""
    idx = index if index is not None else build_allowlist_index()
    entry = idx.get(name)
    if not entry:
        return False
    return book in entry["books"]
