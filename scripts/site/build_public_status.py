#!/usr/bin/env python3
"""Generate the public, end-user-safe Pathfinder build-status feed.

Reads the per-kind unit ledgers under site/dashboard/units/*.json — flat
(name, book, status, wiring_class, type_facet) rows, one file per content
kind — and writes:

  - site/status-data.json           overall + per-book summary
  - site/status-data/<book_id>.json per-book detail: per-kind rollups and
                                     the full item list with each item's
                                     public doneness bucket and standing

This deliberately does NOT read site/dashboard/PF1e-dashboard.json. That
file is a pre-aggregated summary computed by a separate pipeline (engine
state-dump binaries, a doneness cache, etc.) that depends on more of the
operator's machine than a plain checkout has, and it has degraded to
all-zero aggregates twice in one evening for exactly that reason. The unit
ledgers are the first-party source those aggregates are themselves derived
from — recomputing straight from them here has no such dependency.

This is a projection, not a trim: it only ever *copies specific fields* it
knows about (name, book id, a curated display-name/label map, a doneness
verdict, a provenance standing) into a new document. It never passes
through free-text fields from elsewhere in the dashboard tree, so agent
snippets, decisions, session prose, worktree paths, and Claude usage/quota
data structurally cannot leak into the output even if that tree's shape
changes.

THREE SAFETY/CORRECTNESS RULES THIS SCRIPT ENFORCES (2026-08-17, operator
ruling — see SD-31-corpus-closure-grind/decisions.md §12 and §14):

1. PI SCREENING (Decision 12, "withhold the name, keep the row"). Every
   `name` and every `type_facet` is checked against the pinned PCGen
   oracle's own declared-PI state (`scripts/observer/pi_redaction.py` — the
   AUTHORITY is the record's own `NAMEISPI:`/`DESCISPI:` declaration, never
   a hand-maintained blacklist) and replaced with
   `pi_redaction.REDACTED_PI_MARKER` when it is declared PI. The row
   itself, its counts, its standing and its doneness are unaffected — only
   the display string is withheld.

2. PUBLIC DONENESS BUCKETS. The internal six-value doneness verdict
   (`pf1e_dashboard_producer.doneness_verdict`) is collapsed to the three
   public buckets `done` / `partial` / `not-started` via
   `DONENESS_TO_PUBLIC` below (see that mapping's own comment for the
   per-value rationale).

3. STANDING AND THE DENOMINATOR (Decision 14). Every item also carries a
   `standing` — one of `scripts/observer/provenance.py`'s nine canonical
   statuses (or its `unclassified` bookkeeping value), reused rather than
   reimplemented. `denominator = origin + variant`
   (`provenance.DENOMINATOR_STATUSES`) — only items with a qualifying
   standing count toward `done`/`denominator`/`pct` at every rollup level.
   Every other standing is still SHOWN (the item's own `standing` field,
   and each rollup's `standing_breakdown` + `excluded_from_percentage`) —
   never silently dropped from view, only from the percentage math.

Usage:
    python3 scripts/site/build_public_status.py
    python3 scripts/site/build_public_status.py --check   # exit 1 if stale
"""
import json
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
UNITS_DIR = REPO_ROOT / "site" / "dashboard" / "units"
OUTPUT = REPO_ROOT / "site" / "status-data.json"
BOOK_DETAIL_DIR = REPO_ROOT / "site" / "status-data"

# The (wiring_class, status, kind) -> doneness classification is genuinely
# intricate (see pf1e_dashboard_producer.doneness_verdict's own docstring —
# several rounds of documented QA corrections). Import and reuse it rather
# than re-deriving a simplified version here, so this script's numbers can
# never quietly disagree with the engineering-side doneness definition.
sys.path.insert(0, str(REPO_ROOT / "scripts" / "observer"))
# This module's own directory -- inserted explicitly (not relied on as
# sys.path[0]) because test_build_public_status.py loads this file via
# importlib.util.spec_from_file_location, which does NOT put the module's
# own directory on sys.path the way running it as a script does.
sys.path.insert(0, str(REPO_ROOT / "scripts" / "site"))
import pf1e_dashboard_producer as producer  # noqa: E402
import pi_redaction  # noqa: E402
import provenance  # noqa: E402
import pi_substring_allowlist  # noqa: E402

REDACTED_PI_MARKER = pi_redaction.REDACTED_PI_MARKER

# --- Public doneness buckets (operator ruling 2026-08-17) -----------------
#
# "Anything that 'counts' should either be done, partial, or not started."
# Derived from the producer's own six-value doneness_verdict() output, never
# a re-implementation of its (wiring_class, status, kind) table:
#
#   done          -> done      the unchanged best-evidence bucket
#   held          -> partial   real, partial progress recorded by the ladder
#   in-progress   -> partial   same — real, partial progress
#   not-started   -> not-started
#   unmeasurable  -> not-started   JUDGEMENT CALL: we cannot yet measure
#                    these units at all, so claiming ANY of "done" or
#                    "partial" would overstate what is known. Under-claiming
#                    is the correct direction here, not the flattering one.
#   deferred      -> not-started   explicitly deferred work is not done and
#                    not in progress; "not started" is the honest bucket.
DONENESS_DONE = "done"
DONENESS_PARTIAL = "partial"
DONENESS_NOT_STARTED = "not-started"
PUBLIC_DONENESS_VALUES = (DONENESS_DONE, DONENESS_PARTIAL, DONENESS_NOT_STARTED)

DONENESS_TO_PUBLIC = {
    producer.DONENESS_DONE: DONENESS_DONE,
    producer.DONENESS_HELD: DONENESS_PARTIAL,
    producer.DONENESS_IN_PROGRESS: DONENESS_PARTIAL,
    producer.DONENESS_NOT_STARTED: DONENESS_NOT_STARTED,
    producer.DONENESS_UNMEASURABLE: DONENESS_NOT_STARTED,
    producer.DONENESS_DEFERRED: DONENESS_NOT_STARTED,
}
# Fail loud if the producer ever adds a seventh doneness value this mapping
# does not know about — silently defaulting an unmapped verdict would be
# exactly the kind of confidently-wrong figure this package keeps finding.
_unmapped = set(producer.DONENESS_VALUES) - set(DONENESS_TO_PUBLIC)
if _unmapped:
    raise KeyError(
        f"producer.DONENESS_VALUES has value(s) {sorted(_unmapped)} with no "
        "entry in DONENESS_TO_PUBLIC — add one before regenerating."
    )

# --- Standing / provenance (Decision 14) -----------------------------------
#
# `core_essentials` is the one already-decided packaging-artifact book
# (decisions.md §9's census) -- every (object, book) pair naming it is
# `packaging-artifact` unconditionally, per provenance.classify_unambiguous.
PACKAGING_ARTIFACT_BOOKS = frozenset({"core_essentials"})

# Only these standings count toward the published denominator. Reused
# directly from provenance.py (never hand-listed) so a future change to the
# canonical set is inherited automatically rather than silently missed here.
DENOMINATOR_STANDINGS = provenance.DENOMINATOR_STATUSES

# Curated display labels for the unit "kind" facets. Fail-loud: an
# unrecognized kind raises instead of silently omitting it.
KIND_LABELS = {
    "ability": "Abilities",
    "class": "Classes",
    "class_feature": "Class Features",
    "companion": "Companions",
    "deity": "Deities",
    "domain": "Domains",
    "equipment": "Equipment",
    "equipment_modifier": "Equipment Modifiers",
    "feat": "Feats",
    "language": "Languages",
    "monster": "Monsters",
    "monster_ability": "Monster Abilities",
    "power": "Powers",
    "race": "Races",
    "race_trait": "Race Traits",
    "skill": "Skills",
    "spell": "Spells",
    "template": "Templates",
    "trait": "Traits",
}

# Curated display names for the sourcebooks shown publicly. Deliberately a
# fixed allow-list, not "every book id seen in the ledger": the ledger also
# carries future-state books (not yet started, not meant to be shown as a
# stalled 0%) and possibly a stray/renamed id, and a fixed list makes both
# cases a loud KeyError instead of a silent guess. To add a book: add it
# here first, then regenerate.
BOOK_TITLES = {
    "core_rulebook": "Core Rulebook",
    "advanced_players_guide": "Advanced Player's Guide",
    "advanced_class_guide": "Advanced Class Guide",
    "ultimate_psionics": "Ultimate Psionics",
    "ultimate_combat": "Ultimate Combat",
    "ultimate_wilderness": "Ultimate Wilderness",
    "ultimate_magic": "Ultimate Magic",
    "occult_adventures": "Occult Adventures",
    "bestiary": "Bestiary",
    "ultimate_equipment": "Ultimate Equipment",
    "advanced_race_guide": "Advanced Race Guide",
    "bestiary_4": "Bestiary 4",
    "bestiary_3": "Bestiary 3",
    "bestiary_2": "Bestiary 2",
    "ultimate_intrigue": "Ultimate Intrigue",
    "horror_adventures": "Horror Adventures",
    "pathfinder_unchained": "Pathfinder Unchained",
    "inner_sea_gods": "Inner Sea Gods",
    "inner_sea_world_guide": "Inner Sea World Guide",
    "inner_sea_combat": "Inner Sea Combat",
    "inner_sea_races": "Inner Sea Races",
    "inner_sea_intrigue": "Inner Sea Intrigue",
    "book_of_the_damned_volume_2": "Book of the Damned, Vol. 2",
    "bestiary_5": "Bestiary 5",
    "inner_sea_bestiary": "Inner Sea Bestiary",
    "monster_codex": "Monster Codex",
    "book_of_the_damned_volume_1": "Book of the Damned, Vol. 1",
    "bestiary_6": "Bestiary 6",
    "bonus_bestiary": "Bonus Bestiary",
    "ultimate_campaign": "Ultimate Campaign",
}

# A packaging artifact is not a sourcebook, so it must never be published as
# one. `core_essentials` was listed in BOOK_TITLES and therefore rendered as a
# book on the public status page -- with a 0% headline and 128 excluded units
# -- even though decisions.md §9 had already ruled it is not a real Pathfinder
# book. Removing the entry fixes today's page; this assertion is what stops it,
# or any future packaging artifact, from being added back by hand.
_shown_artifacts = PACKAGING_ARTIFACT_BOOKS & set(BOOK_TITLES)
if _shown_artifacts:
    raise ValueError(
        "packaging-artifact books must never be shown as sourcebooks; "
        f"remove from BOOK_TITLES: {sorted(_shown_artifacts)}"
    )


def load_units_by_kind(units_dir: Path = UNITS_DIR):
    """Read every site/dashboard/units/PF1e-units-<kind>.json ledger.

    Returns {kind: [{name, book, status, wiring_class, type_facet}, ...]}.
    """
    by_kind = {}
    for path in sorted(units_dir.glob("PF1e-units-*.json")):
        doc = json.loads(path.read_text())
        kind = doc["kind"]
        fields = doc["fields"]
        if kind not in KIND_LABELS:
            raise KeyError(
                f"unit kind {kind!r} ({path.name}) has no curated label in "
                "KIND_LABELS — add one before regenerating."
            )
        by_kind[kind] = [dict(zip(fields, row)) for row in doc["rows"]]
    return by_kind


def classify_all(units_by_kind):
    """Attach a raw (internal, six-value) doneness verdict to every row,
    across every book (including ones not in BOOK_TITLES) — needed so the
    overall headline matches the "everything in scope" definition rather
    than only the shown books.

    Returns a flat list of {kind, book, name, doneness_raw, type_facet}.
    Names and type_facets here are the TRUE, unredacted values — provenance
    identity (object_id) and PI screening both need the real name, so
    redaction happens in a later, separate pass (see redact_for_display).
    """
    out = []
    for kind, rows in units_by_kind.items():
        for row in rows:
            doneness_raw = producer.doneness_verdict(row["wiring_class"], row["status"], kind)
            out.append({
                "kind": kind,
                "book": row["book"],
                "name": row["name"],
                "doneness_raw": doneness_raw,
                "type_facet": row.get("type_facet") or None,
            })
    return out


def object_id(item) -> str:
    """The best identity key available at this ledger layer: kind-qualified
    name. The unit ledger carries no finer-grained corpus_key (no
    source_line, no PCC-level identifier), so this is the most precise
    grouping classify_unambiguous can be given here — it is still correct
    to keep `kind` in the key (`corpus_identifier_scope_collisions`: "a
    shared name never implies a shared thing") even though it cannot
    disambiguate two genuinely different objects of the SAME kind that
    happen to share a bare name across books; those fall out as
    `unclassified`, the honest, conservative default classify_unambiguous
    already uses for anything it cannot prove is a single printing."""
    return f"{item['kind']}::{item['name']}"


def compute_standing(all_items):
    """{(object_id, book): status} for every item, via
    provenance.classify_unambiguous — reused, not reimplemented. Runs over
    ALL items (not just BOOK_TITLES-scoped ones) so an object's cross-book
    footprint is judged against its full appearance, not a filtered view."""
    pairs = [(object_id(it), it["book"], it["kind"]) for it in all_items]
    records = provenance.classify_unambiguous(pairs, PACKAGING_ARTIFACT_BOOKS)
    return {(r.object_id, r.book): r.status for r in records}


def attach_standing_and_public_doneness(all_items, standing_by_pair):
    """Mutates each item in place: adds `standing`, and replaces
    `doneness_raw` with the public three-bucket `doneness`."""
    for it in all_items:
        it["standing"] = standing_by_pair[(object_id(it), it["book"])]
        it["doneness"] = DONENESS_TO_PUBLIC[it.pop("doneness_raw")]


# type_facet substring rationale (kept here, not just in pi_redaction.py,
# because the false-positive trade-off is specific to THIS field's shape):
#
# WHY PLAIN SUBSTRING, WHEN `name` BELOW USES WORD-BOUNDARY:
# `type_facet` is not a natural-language object name; it is a compound,
# PCGen-TYPE-derived identifier
# (`MagaambyanInitiateClassFeatures.SpecialQuality.Supernatural`) with no
# real word boundaries to check — a proper noun surfaces there embedded in
# an adjectival/compound form (`"Magaambyan"` fused onto
# `"InitiateClassFeatures"` with no delimiter at all) that a boundary-aware
# match would miss entirely. PROVEN against the live corpus this cycle
# (SITE-PUBSTATUS-001): a substring sweep over every distinct type_facet in
# the shipped ledger found exactly 30 genuine hits (Magaambya, Talmandor,
# Rivethun, Qadira/Qadiran, Varisia, Signifer, Jezelda, Ulfen, Galt,
# Keleshite, Nex/Nexian, Hermea — all real Golarion proper nouns) and ZERO
# coincidental short-name collisions, so the false-positive risk that
# motivates boundary-checking for `name` does not carry over to this
# different-shaped field. Scoped GLOBALLY (every declared-PI name anywhere
# in the oracle) because a type_facet is a shared PCGen TYPE token, not a
# book-scoped object identity — the same token can legitimately surface
# under any book.
#
# `name` IS natural-language prose, so it uses
# `pi_redaction.find_declared_pi_word_matches` — a GLOBAL (not book-scoped)
# freestanding-WORD match, gated by the reviewed
# `pi_substring_allowlist` (SITE-PI-ALLOWLIST-001, 2026-08-17 operator
# ruling). See that function's own docstring for the full history: an
# earlier GLOBAL PLAIN-substring scan over `name` found 78 hits, 52 of them
# same-word fusions (`"Brigh"`-in-`"Brightness"`) rather than genuine
# embeds; a BOOK-SCOPED substring scan fixed those 52 but then missed
# `"Death (Pharasma)"` — a genuine, word-bounded, but CROSS-BOOK deity
# embed. Word-boundary matching (this pass) clears the 52 without any
# book-scoping and without missing Pharasma, leaving exactly the reviewed,
# allow-listed 11 mundane whole-word embeds (`"Shackles of Compliance"`
# etc.) as the only thing standing between "found a hit" and "published
# anyway" — and those 11 are now an explicit, reasoned, tested list rather
# than an implicit gap in the screen.


def redact_for_display(all_items, name_to_books, declared_names):
    """Mutates each item in place: replaces `name` and/or `type_facet` with
    REDACTED_PI_MARKER when either carries a declared-PI name. Called
    AFTER standing has been computed (object_id/provenance identity must
    use the true name — see classify_all's docstring) so redaction is a
    pure display-layer concern, never a classification one.

    THREE independent checks combine, matching site_dashboard_pi_gate.py's
    own multi-pass posture:
      - `name`, EXACT match against `declared_names` (global unambiguous
        safety net, in case a ledger `book` id and an oracle directory
        basename ever disagree);
      - `name`, WORD-BOUNDARY match against declared-PI names FROM THE
        ITEM'S OWN BOOK (`name_to_books`, via `build_book_declared_name_lists`
        -- NOT filtered for global ambiguity) UNION the GLOBAL unambiguous
        declared-PI set (`declared_names`), minus anything covered by the
        reviewed `pi_substring_allowlist` for this exact (name, book).
        BOTH sources are needed, not just one: the book-scoped source alone
        misses a genuine cross-book embed like `"Death (Pharasma)"`
        (Pharasma is declared PI under `inner_sea_gods`, not
        `advanced_players_guide`); the global-unambiguous source alone
        misses a genuine SAME-book embed of a name that ALSO has a second,
        unrelated row elsewhere in the wider Paizo-scoped oracle tree that
        carries no `NAMEISPI:YES` token at all, making the bare name
        globally ambiguous per `build_declared_pi_name_index`'s own
        conservative "unambiguous everywhere" rule (`"Baphomet's Blessing"`
        in `inner_sea_gods`: Baphomet is genuinely declared PI there, but a
        PFS-legality-override row for the same deity in a DIFFERENT book
        carries no NAMEISPI token of its own). Mutation-tested
        (SITE-PI-ALLOWLIST-001): using the global set alone silently
        un-redacted 12 previously-caught genuine leaks;
      - `type_facet`, plain SUBSTRING match against the GLOBAL declared-PI
        name set (see the module-level comment above for why this field
        stays plain-substring, not word-boundary)."""
    declared_by_length = sorted(declared_names, key=len, reverse=True)
    # `build_book_declared_name_lists` already sorts each book's own list
    # longest-first (see its own docstring) -- reused directly, no
    # per-item re-sort.
    book_declared = pi_redaction.build_book_declared_name_lists(name_to_books)
    allowlist_index = pi_substring_allowlist.build_allowlist_index()
    facet_cache: dict[str, bool] = {}
    name_cache: dict[tuple[str, str], list[str]] = {}

    for it in all_items:
        name = it["name"]
        book = it["book"]
        if name.strip() in declared_names:
            it["name"] = REDACTED_PI_MARKER
        else:
            key = (book, name)
            if key not in name_cache:
                matches = set(pi_redaction.find_declared_pi_word_matches(name, book_declared.get(book, ())))
                matches.update(pi_redaction.find_declared_pi_word_matches(name, declared_by_length))
                name_cache[key] = sorted(matches)
            if name_cache[key] and not pi_substring_allowlist.is_allowlisted(name, book, allowlist_index):
                it["name"] = REDACTED_PI_MARKER

        tf = it["type_facet"]
        if tf:
            if tf not in facet_cache:
                facet_cache[tf] = pi_redaction.value_carries_declared_pi_substring(tf, declared_by_length)
            if facet_cache[tf]:
                it["type_facet"] = REDACTED_PI_MARKER


def _standing_breakdown(items):
    breakdown = {}
    for it in items:
        breakdown[it["standing"]] = breakdown.get(it["standing"], 0) + 1
    return breakdown


def _rollup(items):
    """{done, partial, not_started, denominator, pct, excluded_from_percentage,
    standing_breakdown} for a list of already-classified, already-redacted
    items. `denominator` (and therefore `pct`) counts ONLY items whose
    `standing` is in DENOMINATOR_STANDINGS (origin/errata-source/variant,
    Decision 14 §5) — every other standing is still counted in
    `standing_breakdown` and `excluded_from_percentage`, never silently
    dropped."""
    counted = [it for it in items if it["standing"] in DENOMINATOR_STANDINGS]
    denominator = len(counted)
    done_n = sum(1 for it in counted if it["doneness"] == DONENESS_DONE)
    partial_n = sum(1 for it in counted if it["doneness"] == DONENESS_PARTIAL)
    not_started_n = sum(1 for it in counted if it["doneness"] == DONENESS_NOT_STARTED)
    pct = round(100 * done_n / denominator, 1) if denominator else 0.0
    return {
        "done": done_n,
        "partial": partial_n,
        "not_started": not_started_n,
        "denominator": denominator,
        "pct": pct,
        "excluded_from_percentage": len(items) - denominator,
        "standing_breakdown": _standing_breakdown(items),
    }


def build_overview(all_items):
    overall = _rollup(all_items)

    by_book = {}
    for it in all_items:
        if it["book"] not in BOOK_TITLES:
            continue
        by_book.setdefault(it["book"], []).append(it)

    books = []
    for book_id, items in by_book.items():
        roll = _rollup(items)
        books.append({"id": book_id, "title": BOOK_TITLES[book_id], **roll})
    books.sort(key=lambda b: -b["denominator"])

    return overall, books


def add_display_names(items):
    """Adds a `display_name` field to every item in `items`, in place.

    Decision 17 / `OPERATOR-RULINGS-2026-08-19.md` Ruling §17: the
    drill-down shows a bare `name` that is not unique in ~2,325
    `(book, kind, name)` groups corpus-wide (an 11.07% excess) — 0 of
    those groups share a `corpus_key`, 0 share a `source_file`+
    `source_line`. Every row is a distinct printed thing (e.g.
    `advanced_class_guide` `class_feature` "Bloodline Powers" appears 11
    times because eleven different bloodlines each print their own), so
    the fix is a disambiguator, never a merge — this function never drops
    or combines an item, only labels it.

    `items` must already be the per-(book, kind) list `build_book_details`
    builds (same `name` value = same drill-down collision the operator
    saw), and must already be past `redact_for_display`: this function
    reads only `name` and `type_facet`, both already PI-screened by that
    point, and introduces no new field or lookup that could reach past
    that screen — a `REDACTED_PI_MARKER` name/type_facet disambiguates
    exactly like any other string, revealing nothing beyond what was
    already public.

    Unique names are left as `display_name == name`. A collision is first
    disambiguated with the item's own `type_facet` ("Name (facet)"); if
    that still collides (identical name AND type_facet, or no type_facet
    at all), a stable positional suffix ("Name (facet) #2" / "Name #2") is
    appended so the page never shows two rows with nothing to tell them
    apart.
    """
    by_name = {}
    for it in items:
        by_name.setdefault(it["name"], []).append(it)
    for name, group in by_name.items():
        if len(group) == 1:
            group[0]["display_name"] = name
            continue
        seen = set()
        for idx, it in enumerate(group, start=1):
            tf = it.get("type_facet")
            label = f"{name} ({tf})" if tf else name
            if label in seen:
                label = f"{label} #{idx}"
            seen.add(label)
            it["display_name"] = label


def build_book_details(all_items):
    """Return {book_id: {id, title, kinds: [{kind, label, ...rollup,
    items: [{name, display_name, doneness, type_facet, standing}, ...]}, ...
    ]}} for every book in BOOK_TITLES. `display_name` (see
    `add_display_names`) is what the drill-down UI shows; `name` is kept
    unredacted-of-purpose (it is already PI-screened by this point) for
    any consumer that wants the plain field."""
    grouped = {}
    for it in all_items:
        if it["book"] not in BOOK_TITLES:
            continue
        grouped.setdefault(it["book"], {}).setdefault(it["kind"], []).append({
            "name": it["name"],
            "doneness": it["doneness"],
            "type_facet": it["type_facet"],
            "standing": it["standing"],
        })

    details = {}
    for book_id, kinds_map in grouped.items():
        kind_entries = []
        for kind, items in kinds_map.items():
            add_display_names(items)
            items.sort(key=lambda x: x["name"])
            roll = _rollup(items)
            kind_entries.append({
                "kind": kind,
                "label": KIND_LABELS[kind],
                **roll,
                "items": items,
            })
        kind_entries.sort(key=lambda k: k["label"])
        details[book_id] = {"id": book_id, "title": BOOK_TITLES[book_id], "kinds": kind_entries}
    return details


def build():
    units_by_kind = load_units_by_kind()
    all_items = classify_all(units_by_kind)

    standing_by_pair = compute_standing(all_items)
    attach_standing_and_public_doneness(all_items, standing_by_pair)

    corpus_root = pi_redaction.pcgen_corpus_root()
    name_to_books = pi_redaction.build_declared_pi_name_book_index(corpus_root)
    declared_names = pi_redaction.build_declared_pi_name_index(corpus_root)
    redact_for_display(all_items, name_to_books, declared_names)

    overall, books = build_overview(all_items)
    book_details = build_book_details(all_items)

    missing = set(BOOK_TITLES) - {b["id"] for b in books}
    if missing:
        raise KeyError(
            f"BOOK_TITLES has entries with no units in the ledger: {sorted(missing)} — "
            "remove them or check the ledger, don't publish a book with nothing behind it."
        )

    overview = {
        "generated_at": __import__("datetime").datetime.now(__import__("datetime").timezone.utc)
        .strftime("%Y-%m-%dT%H:%M:%SZ"),
        "doneness_values": list(PUBLIC_DONENESS_VALUES),
        "overall": overall,
        "books": books,
    }

    # Defense-in-depth blanket sweep (site_dashboard_pi_gate.py's own
    # "safety net" posture): the two call-site-specific redactions above
    # (name, type_facet) are the primary defense; this final exact-match
    # pass over the WHOLE assembled document catches any name-shaped leak
    # through a surface neither pass individually targeted (a curated
    # label, a future field) before anything is written or compared.
    overview = pi_redaction.redact_declared_pi_names(overview, declared_names)
    book_details = {
        book_id: pi_redaction.redact_declared_pi_names(detail, declared_names)
        for book_id, detail in book_details.items()
    }

    return overview, book_details


def main():
    check_only = "--check" in sys.argv
    overview, book_details = build()

    # generated_at always differs run to run; compare everything else.
    def without_timestamp(doc):
        return {k: v for k, v in doc.items() if k != "generated_at"}

    detail_texts = {
        book_id: json.dumps(detail, indent=2) + "\n"
        for book_id, detail in book_details.items()
    }

    def orphan_detail_files(texts):
        """Committed per-book files for books this run no longer publishes."""
        if not BOOK_DETAIL_DIR.is_dir():
            return []
        return sorted(
            path
            for path in BOOK_DETAIL_DIR.glob("*.json")
            if path.stem not in texts
        )

    if check_only:
        stale = False
        if OUTPUT.exists():
            current = json.loads(OUTPUT.read_text())
            if without_timestamp(current) != without_timestamp(overview):
                stale = True
        else:
            stale = True
        for book_id, text in detail_texts.items():
            path = BOOK_DETAIL_DIR / f"{book_id}.json"
            if not path.exists() or path.read_text() != text:
                stale = True
                print(f"STALE: {path}", file=sys.stderr)
        for orphan in orphan_detail_files(detail_texts):
            stale = True
            print(f"STALE: {orphan} is no longer a published book", file=sys.stderr)
        if stale:
            print(f"STALE: {OUTPUT} or one or more book-detail files need regenerating", file=sys.stderr)
            sys.exit(1)
        print("OK: status-data.json and status-data/*.json are up to date")
        return

    OUTPUT.write_text(json.dumps(overview, indent=2) + "\n")
    BOOK_DETAIL_DIR.mkdir(exist_ok=True)
    for book_id, text in detail_texts.items():
        (BOOK_DETAIL_DIR / f"{book_id}.json").write_text(text)
    for orphan in orphan_detail_files(detail_texts):
        # Writing is not enough: dropping a book from BOOK_TITLES left its old
        # detail file on disk, and site/ IS the deployed artifact, so the file
        # stayed publicly fetchable after the book stopped being published.
        orphan.unlink()
        print(f"removed orphaned detail file {orphan}")

    total_items = sum(len(k["items"]) for d in book_details.values() for k in d["kinds"])
    print(
        f"Wrote {OUTPUT} ({len(overview['books'])} books, overall {overview['overall']['pct']}%) "
        f"and {len(book_details)} book-detail files under {BOOK_DETAIL_DIR} ({total_items} items total)"
    )


if __name__ == "__main__":
    main()
