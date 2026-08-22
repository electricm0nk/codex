#!/usr/bin/env python3
"""scripts/site_dashboard_pi_gate.py -- Decision 12 (2026-08-17, operator
ruling) requirement #3: "A gate, proven able to fail. A verify.sh stage
must fail when the committed feed or any shard carries a declared-PI name.
Mutation-prove it by seeding one, in both the top-level feed and a shard."

WHAT THIS SCANS: every committed file under `site/dashboard/` that ends in
`.json`, walked as a decoded JSON document, every string leaf checked
against the pinned PCGen oracle's own full `NAMEISPI:YES` name index
(`scripts/observer/pi_redaction.py::build_declared_pi_name_index`) with
EXACT string-leaf equality (see `find_declared_pi_leaks`'s own docstring
for why exact-match, not a substring/word-boundary scan, is the right
default: an earlier word-boundary version flagged the ordinary, non-PI
"Shackles of Compliance" purely because "Shackles" occurs as one word
inside it). A SECOND, book-scoped pass
(`find_declared_pi_leaks_in_shard_rows`) additionally runs over every
`units/*.json` shard's own `fields`/`rows` schema against a PER-BOOK
declared-PI index (`build_declared_pi_name_book_index`), closing the
name-declared-PI-in-one-book-but-not-another gap the book-blind index
cannot see (SD31-W13-INTEGRATE-001 finding 2).

FIX-DASHBOARD-PI (2026-08-17): passes 1 and 2 above are both EXACT-match --
structurally blind to a declared-PI name EMBEDDED inside a longer string
(`"Helm of the Serpent King"` embeds the declared-PI record `"The Serpent
King"`; `"Varisian Pilgrim Domain"` embeds the declared-PI archetype
`"Varisian Pilgrim"`) -- exactly the gap this file's own "KNOWN RESIDUAL
GAP" note used to warn about, and exactly how 89 real leaks (`Bow of
Erastil`, `Witherfang`, `Legendsbane`, four `unit_index` category labels,
and more) shipped straight through an all-green run of this gate. THREE
more passes close it, all built from the SAME primitive
(`pi_redaction.find_declared_pi_word_matches`, word-boundary matching) and
the SAME shared, reviewed allow-list
(`scripts/site/pi_substring_allowlist.py` -- the identical module and file
`build_public_status.py`'s public status projection already uses, never a
second list):

  3. `find_book_roster_leaks` -- the top-level feed's `books[*].items.
     {equipment,feats,spells,monsters,races,prestige_classes}[]` (plain
     name strings, or a dict entry's own `name` field), book-scoped via
     each book's own `id`.
  4. `find_shard_word_boundary_leaks` -- every `units/*.json` shard's own
     `fields`/`rows`: `name` WORD-BOUNDARY instead of exact (book-scoped via
     each row's own `book` column), PLUS `type_facet` (if the shard carries
     it) GLOBAL plain SUBSTRING with no allow-list -- this raw compound
     TYPE-token field had NO screen of any kind before this fix.
  5. `find_category_label_leaks` -- `unit_index.kinds[*].categories[*]`/
     `school_categories[*]`'s own `label` (a BUILT-UP string aggregated
     across a whole kind, so no single book to scope against -- gated by
     `pi_substring_allowlist.is_allowlisted_for_any_book` instead). Reads
     both the top-level feed's embedded `unit_index` and the standalone
     `units/index.json` manifest, since both carry the same shape.

This is a SAFETY NET, not the primary defense: the primary defense is the
producer redacting at generation time (`pf1e_dashboard_producer.py`'s
`build_unit_shards`, `_parse_lst_first_field`, and `_PiScreen`, all wired
this cycle). This gate exists because a hand-edit, a reverted redaction, or
a future change to the producer that forgets to call the reader are all
real failure modes a generation-time fix alone cannot catch -- exactly
`declared_pi_shipping_audit.rs`'s own rationale for `data/corpus/`, applied
to this second surface.

Exit 0 and print `site-dashboard-pi-gate: CLEAN` when no declared-PI name is
found in any scanned file. Exit 1 and print every hit (file, JSON path,
name) otherwise.

Degraded-oracle posture: if the pinned checkout cannot be found at all, this
prints a loud warning and exits 1 -- a gate that cannot see the oracle
cannot prove anything clean, and "could not check" must never read as
"checked and clean" (Decision 12 requirement #2's whole point, applied to
the gate itself, not just the producer).

Run: python3 scripts/site_dashboard_pi_gate.py
Wired as the `site-dashboard-pi-gate` stage in `scripts/verify.sh`.
"""
from __future__ import annotations

import glob
import importlib.util
import json
import os
import pathlib
import sys

_REPO_ROOT = pathlib.Path(__file__).resolve().parent.parent
_PI_REDACTION = _REPO_ROOT / "scripts" / "observer" / "pi_redaction.py"
_spec = importlib.util.spec_from_file_location("pi_redaction", _PI_REDACTION)
pi_redaction = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(pi_redaction)

_PI_ALLOWLIST = _REPO_ROOT / "scripts" / "site" / "pi_substring_allowlist.py"
_spec2 = importlib.util.spec_from_file_location("pi_substring_allowlist", _PI_ALLOWLIST)
pi_substring_allowlist = importlib.util.module_from_spec(_spec2)
_spec2.loader.exec_module(pi_substring_allowlist)

DASHBOARD_DIR = str(_REPO_ROOT / "site" / "dashboard")


def scanned_files(dashboard_dir: str) -> list[str]:
    """Every committed `.json` file under `site/dashboard/`, sorted for a
    deterministic report. Includes both the top-level feed and any shard
    files under `units/` -- Decision 12 flags both surfaces, and nothing
    here special-cases either path."""
    return sorted(glob.glob(os.path.join(dashboard_dir, "**", "*.json"), recursive=True))


def find_book_roster_leaks(doc, declared_by_length, book_declared, allowlist_index) -> list[tuple[str, str]]:
    """WORD-BOUNDARY leak scan for the top-level feed's own
    `books[*].items.{equipment,feats,spells,monsters,races,
    prestige_classes}[]` shape (`pf1e_dashboard_producer.py`'s
    `_book_item_roster`/`_load_beastiary_monsters`). A no-op on any document
    that is not shaped this way. Each entry is either a plain name string,
    or a dict carrying its own `name` field (monster/prestige-class rows).
    Book-scoped via the enclosing `books[*].id` -- mirrors
    `build_public_status.py::redact_for_display`'s own book-scoped union
    global technique, applied to this feed's different document shape."""
    hits: list[tuple[str, str]] = []
    if not isinstance(doc, dict):
        return hits
    books = doc.get("books")
    if not isinstance(books, list):
        return hits
    for bi, book in enumerate(books):
        if not isinstance(book, dict):
            continue
        book_id = book.get("id")
        items = book.get("items")
        if not isinstance(book_id, str) or not isinstance(items, dict):
            continue
        own_book_by_length = book_declared.get(book_id, ())
        for kind, entries in items.items():
            if not isinstance(entries, list):
                continue
            for ei, entry in enumerate(entries):
                if isinstance(entry, str):
                    name, path = entry, f"$.books[{bi}].items.{kind}[{ei}]"
                elif isinstance(entry, dict) and isinstance(entry.get("name"), str):
                    name, path = entry["name"], f"$.books[{bi}].items.{kind}[{ei}].name"
                else:
                    continue
                if name == pi_redaction.REDACTED_PI_MARKER:
                    continue
                matches = set(pi_redaction.find_declared_pi_word_matches(name, own_book_by_length, case_insensitive=True))
                matches.update(pi_redaction.find_declared_pi_word_matches(name, declared_by_length, case_insensitive=True))
                if matches and not pi_substring_allowlist.is_allowlisted(name, book_id, allowlist_index):
                    hits.append((
                        path,
                        f"{name!r} carries declared-PI word(s) {sorted(matches)!r} in book {book_id!r}, "
                        "not on the reviewed allow-list for this (name, book)",
                    ))
    return hits


def find_shard_word_boundary_leaks(doc, declared_by_length, book_declared, allowlist_index) -> list[tuple[str, str]]:
    """WORD-BOUNDARY counterpart to `pi_redaction.
    find_declared_pi_leaks_in_shard_rows` (exact-match) for the SAME
    `units/*.json` shard `{"fields": [...], "rows": [...]}` shape -- a
    no-op on any document not shaped this way. `name`: book-scoped
    WORD-BOUNDARY, same technique as `find_book_roster_leaks` above.
    `type_facet` (if present): GLOBAL plain SUBSTRING, no allow-list --
    mirrors `pf1e_dashboard_producer.py`'s own `type_facet` pass in
    `build_unit_shards`, which mirrors `build_public_status.py`'s
    established convention for this same compound-identifier field shape."""
    hits: list[tuple[str, str]] = []
    fields = doc.get("fields") if isinstance(doc, dict) else None
    rows = doc.get("rows") if isinstance(doc, dict) else None
    if not isinstance(fields, list) or not isinstance(rows, list):
        return hits
    if "name" not in fields or "book" not in fields:
        return hits
    name_idx = fields.index("name")
    book_idx = fields.index("book")
    type_facet_idx = fields.index("type_facet") if "type_facet" in fields else None
    for i, row in enumerate(rows):
        if not isinstance(row, list) or len(row) <= max(name_idx, book_idx):
            continue
        name = row[name_idx]
        book = row[book_idx]
        if isinstance(name, str) and name != pi_redaction.REDACTED_PI_MARKER:
            own_book_by_length = book_declared.get(book, ()) if isinstance(book, str) else ()
            matches = set(pi_redaction.find_declared_pi_word_matches(name, own_book_by_length, case_insensitive=True))
            matches.update(pi_redaction.find_declared_pi_word_matches(name, declared_by_length, case_insensitive=True))
            allowed = isinstance(book, str) and pi_substring_allowlist.is_allowlisted(name, book, allowlist_index)
            if matches and not allowed:
                hits.append((
                    f"$.rows[{i}][{name_idx}]",
                    f"{name!r} carries declared-PI word(s) {sorted(matches)!r} in book {book!r}, "
                    "not on the reviewed allow-list for this (name, book)",
                ))
        if type_facet_idx is not None and type_facet_idx < len(row):
            tf = row[type_facet_idx]
            if (isinstance(tf, str) and tf != pi_redaction.REDACTED_PI_MARKER
                    and pi_redaction.value_carries_declared_pi_substring(tf, declared_by_length)):
                hits.append((
                    f"$.rows[{i}][{type_facet_idx}]",
                    f"{tf!r} carries a declared-PI name as a substring",
                ))
    return hits


def _unit_index_kinds(doc):
    """Locate the `kinds` dict either at the document root (the standalone
    `units/index.json` manifest IS the index) or under `unit_index` (the
    top-level `PF1e-dashboard.json` feed embeds the same index verbatim as
    `data["unit_index"]`). Returns `None` if neither shape matches."""
    if not isinstance(doc, dict):
        return None
    kinds = doc.get("kinds")
    if isinstance(kinds, dict):
        return kinds
    unit_index = doc.get("unit_index")
    if isinstance(unit_index, dict) and isinstance(unit_index.get("kinds"), dict):
        return unit_index["kinds"]
    return None


def find_category_label_leaks(doc, declared_by_length, allowlist_index) -> list[tuple[str, str]]:
    """WORD-BOUNDARY leak scan over `unit_index.kinds[*].categories[*]`/
    `school_categories[*]`'s own `label` (`pf1e_dashboard_producer.py`'s
    `build_unit_shards`, `_screen_category_label`) -- a BUILT-UP string
    (a TYPE token's first segment, translated for display) aggregated
    across every book in a whole kind, so there is no single book to scope
    against: gated by `pi_substring_allowlist.is_allowlisted_for_any_book`
    instead of the per-book `is_allowlisted` every other pass in this file
    uses. This is exactly the shape this file's own former "KNOWN RESIDUAL
    GAP" note named (`"Varisian Pilgrim Domain"`, `"Ulfen Guard Class
    Feature"`, `"Tattooed Sorcerer Varisian Tattoo"`, `"Pathfinders Past
    Focus"`)."""
    hits: list[tuple[str, str]] = []
    kinds = _unit_index_kinds(doc)
    if not kinds:
        return hits
    for kind, kind_entry in kinds.items():
        if not isinstance(kind_entry, dict):
            continue
        for group_field in ("categories", "school_categories"):
            group = kind_entry.get(group_field)
            if not isinstance(group, dict):
                continue
            for gkey, bucket in group.items():
                if not isinstance(bucket, dict):
                    continue
                label = bucket.get("label")
                if not isinstance(label, str) or label == pi_redaction.REDACTED_PI_MARKER:
                    continue
                matches = pi_redaction.find_declared_pi_word_matches(label, declared_by_length, case_insensitive=True)
                if matches and not pi_substring_allowlist.is_allowlisted_for_any_book(label, allowlist_index):
                    hits.append((
                        f"$.kinds.{kind}.{group_field}.{gkey}.label",
                        f"{label!r} carries declared-PI word(s) {sorted(matches)!r}, not on the "
                        "reviewed allow-list (any book) for this label",
                    ))
    return hits


def main() -> int:
    corpus_root = pi_redaction.pcgen_corpus_root()
    if not os.path.isdir(corpus_root):
        print(
            f"site-dashboard-pi-gate: FAIL — pinned PCGen oracle not found at "
            f"{corpus_root!r}; a gate that cannot read the oracle cannot prove "
            f"the feed clean (run scripts/fetch-pcgen-oracle.sh)",
            file=sys.stderr,
        )
        return 1

    declared_names = pi_redaction.build_declared_pi_name_index(corpus_root)
    if not declared_names:
        # A sweep that found zero declared names anywhere in a 6000+-file
        # oracle checkout is itself a red flag (same "asserts nothing"
        # posture pi-sweep/declared-pi-audit both guard against) -- more
        # likely a broken sparse checkout than a genuinely PI-free oracle.
        print(
            "site-dashboard-pi-gate: FAIL — the pinned oracle sweep found zero "
            "NAMEISPI:YES declarations anywhere; this has never been true of "
            "the real checkout and most likely means the sparse checkout is "
            "broken or empty, not that the corpus is PI-free",
            file=sys.stderr,
        )
        return 1

    patterns = pi_redaction.compile_name_patterns(declared_names)
    name_to_books = pi_redaction.build_declared_pi_name_book_index(corpus_root)
    declared_by_length = sorted(declared_names, key=len, reverse=True)
    book_declared = pi_redaction.build_book_declared_name_lists(name_to_books)
    allowlist_index = pi_substring_allowlist.build_allowlist_index()

    files = scanned_files(DASHBOARD_DIR)
    if not files:
        print(
            "site-dashboard-pi-gate: CLEAN — no site/dashboard/*.json files "
            "are committed yet (nothing to scan)"
        )
        return 0

    all_hits: list[tuple[str, str, str]] = []
    for path in files:
        try:
            with open(path, encoding="utf-8") as f:
                doc = json.load(f)
        except (OSError, json.JSONDecodeError) as exc:
            print(f"site-dashboard-pi-gate: FAIL — could not read/parse {path}: {exc}", file=sys.stderr)
            return 1
        rel = os.path.relpath(path, str(_REPO_ROOT))
        for json_path, name in pi_redaction.find_declared_pi_leaks(doc, patterns):
            all_hits.append((rel, json_path, f"carries declared-PI name {name!r}"))
        for json_path, name in pi_redaction.find_declared_pi_leaks_in_shard_rows(doc, name_to_books):
            all_hits.append((rel, json_path, f"carries declared-PI name {name}"))
        # FIX-DASHBOARD-PI (2026-08-17): the three WORD-BOUNDARY passes --
        # see the module docstring for what each one covers and why exact
        # matching alone missed 89 real leaks. Each already returns a full,
        # self-explaining detail string (not just a bare name), so it is
        # appended as-is rather than re-wrapped.
        for json_path, detail in find_book_roster_leaks(doc, declared_by_length, book_declared, allowlist_index):
            all_hits.append((rel, json_path, detail))
        for json_path, detail in find_shard_word_boundary_leaks(doc, declared_by_length, book_declared, allowlist_index):
            all_hits.append((rel, json_path, detail))
        for json_path, detail in find_category_label_leaks(doc, declared_by_length, allowlist_index):
            all_hits.append((rel, json_path, detail))

    if all_hits:
        print(
            f"site-dashboard-pi-gate: FAIL — {len(all_hits)} declared-PI leak(s) "
            f"found across {len(files)} scanned file(s):",
            file=sys.stderr,
        )
        for rel, json_path, detail in all_hits:
            print(f"  {rel}:{json_path} {detail}", file=sys.stderr)
        return 1

    print(
        f"site-dashboard-pi-gate: CLEAN — {len(files)} file(s) scanned against "
        f"{len(declared_names)} declared-PI name(s), zero leaked"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
