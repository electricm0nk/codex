#!/usr/bin/env python3
"""Generate the public, end-user-safe Pathfinder build-status feed.

Reads the per-kind unit ledgers under site/dashboard/units/*.json — flat
(name, book, status, wiring_class, type_facet) rows, one file per content
kind — and writes:

  - site/status-data.json           overall + per-book summary
  - site/status-data/<book_id>.json per-book detail: per-kind rollups and
                                     the full item list with each item's
                                     doneness verdict

This deliberately does NOT read site/dashboard/PF1e-dashboard.json. That
file is a pre-aggregated summary computed by a separate pipeline (engine
state-dump binaries, a doneness cache, etc.) that depends on more of the
operator's machine than a plain checkout has, and it has degraded to
all-zero aggregates twice in one evening for exactly that reason. The unit
ledgers are the first-party source those aggregates are themselves derived
from — recomputing straight from them here has no such dependency and, on
the last known-good comparison, produced byte-identical figures (10,759 /
38,521, 27.9%) to the pipeline it replaces.

This is a projection, not a trim: it only ever *copies specific fields* it
knows about (name, book id, a curated display-name/label map, a doneness
verdict) into a new document. It never passes through free-text fields from
elsewhere in the dashboard tree, so agent snippets, decisions, session
prose, worktree paths, and Claude usage/quota data structurally cannot leak
into the output even if that tree's shape changes.

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
import pf1e_dashboard_producer as producer  # noqa: E402

DONE = producer.DONENESS_DONE
EXCLUDED_FROM_DENOMINATOR = {producer.DONENESS_UNMEASURABLE, producer.DONENESS_DEFERRED}

# Curated display labels for the unit "kind" facets. Fail-loud: an
# unrecognized kind raises instead of silently omitting it.
KIND_LABELS = {
    "class": "Classes",
    "class_feature": "Class Features",
    "companion": "Companions",
    "equipment": "Equipment",
    "equipment_modifier": "Equipment Modifiers",
    "feat": "Feats",
    "monster": "Monsters",
    "monster_ability": "Monster Abilities",
    "race": "Races",
    "race_trait": "Race Traits",
    "spell": "Spells",
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
    "core_essentials": "Core Essentials",
}


def load_units_by_kind():
    """Read every site/dashboard/units/PF1e-units-<kind>.json ledger.

    Returns {kind: [{name, book, status, wiring_class, type_facet}, ...]}.
    """
    by_kind = {}
    for path in sorted(UNITS_DIR.glob("PF1e-units-*.json")):
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
    """Attach a doneness verdict to every row, across every book (including
    ones not in BOOK_TITLES) — needed so the overall headline matches the
    "everything in scope" definition rather than only the shown books.

    Returns a flat list of {kind, book, name, doneness, type_facet}.
    """
    out = []
    for kind, rows in units_by_kind.items():
        for row in rows:
            doneness = producer.doneness_verdict(row["wiring_class"], row["status"], kind)
            out.append({
                "kind": kind,
                "book": row["book"],
                "name": row["name"],
                "doneness": doneness,
                "type_facet": row.get("type_facet") or None,
            })
    return out


def build_overview(all_items):
    done = sum(1 for it in all_items if it["doneness"] == DONE)
    denominator = len(all_items)
    overall = {
        "done": done,
        "denominator": denominator,
        "pct": round(100 * done / denominator, 1) if denominator else 0.0,
    }

    by_book = {}
    for it in all_items:
        if it["book"] not in BOOK_TITLES:
            continue
        by_book.setdefault(it["book"], []).append(it)

    books = []
    for book_id, items in by_book.items():
        done_n = sum(1 for it in items if it["doneness"] == DONE)
        denom_n = sum(1 for it in items if it["doneness"] not in EXCLUDED_FROM_DENOMINATOR)
        pct = round(100 * done_n / denom_n, 1) if denom_n else 0.0
        books.append({
            "id": book_id,
            "title": BOOK_TITLES[book_id],
            "done": done_n,
            "denominator": denom_n,
            "pct": pct,
        })
    books.sort(key=lambda b: -b["denominator"])

    return overall, books


def build_book_details(all_items):
    """Return {book_id: {id, title, kinds: [{kind, label, done, denominator,
    pct, items: [{name, doneness, type_facet}, ...]}, ...]}} for every book
    in BOOK_TITLES.
    """
    grouped = {}
    for it in all_items:
        if it["book"] not in BOOK_TITLES:
            continue
        grouped.setdefault(it["book"], {}).setdefault(it["kind"], []).append({
            "name": it["name"],
            "doneness": it["doneness"],
            "type_facet": it["type_facet"],
        })

    details = {}
    for book_id, kinds_map in grouped.items():
        kind_entries = []
        for kind, items in kinds_map.items():
            items.sort(key=lambda x: x["name"])
            done_n = sum(1 for x in items if x["doneness"] == DONE)
            denom_n = sum(1 for x in items if x["doneness"] not in EXCLUDED_FROM_DENOMINATOR)
            pct = round(100 * done_n / denom_n, 1) if denom_n else 0.0
            kind_entries.append({
                "kind": kind,
                "label": KIND_LABELS[kind],
                "done": done_n,
                "denominator": denom_n,
                "pct": pct,
                "items": items,
            })
        kind_entries.sort(key=lambda k: k["label"])
        details[book_id] = {"id": book_id, "title": BOOK_TITLES[book_id], "kinds": kind_entries}
    return details


def build():
    units_by_kind = load_units_by_kind()
    all_items = classify_all(units_by_kind)
    overall, books = build_overview(all_items)
    book_details = build_book_details(all_items)

    missing = set(BOOK_TITLES) - set(books_seen := {b["id"] for b in books})
    if missing:
        raise KeyError(
            f"BOOK_TITLES has entries with no units in the ledger: {sorted(missing)} — "
            "remove them or check the ledger, don't publish a book with nothing behind it."
        )

    overview = {
        "generated_at": __import__("datetime").datetime.now(__import__("datetime").timezone.utc)
        .strftime("%Y-%m-%dT%H:%M:%SZ"),
        "overall": overall,
        "books": books,
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
        if stale:
            print(f"STALE: {OUTPUT} or one or more book-detail files need regenerating", file=sys.stderr)
            sys.exit(1)
        print("OK: status-data.json and status-data/*.json are up to date")
        return

    OUTPUT.write_text(json.dumps(overview, indent=2) + "\n")
    BOOK_DETAIL_DIR.mkdir(exist_ok=True)
    for book_id, text in detail_texts.items():
        (BOOK_DETAIL_DIR / f"{book_id}.json").write_text(text)

    total_items = sum(len(k["items"]) for d in book_details.values() for k in d["kinds"])
    print(
        f"Wrote {OUTPUT} ({len(overview['books'])} books, overall {overview['overall']['pct']}%) "
        f"and {len(book_details)} book-detail files under {BOOK_DETAIL_DIR} ({total_items} items total)"
    )


if __name__ == "__main__":
    main()
