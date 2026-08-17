#!/usr/bin/env python3
"""scripts/site_public_status_pi_gate.py -- Decision 12 (2026-08-17, operator
ruling) requirement #3, applied to the public status projection: "A gate,
proven able to fail. A verify.sh stage must fail when the committed feed or
any shard carries a declared-PI name."

`scripts/site_dashboard_pi_gate.py` already covers `site/dashboard/**`
(the internal feed and its unit shards). This is that same gate SHAPE
applied to the SEPARATE public surface `scripts/site/build_public_status.py`
generates: `site/status-data.json` (overview + per-book roll-up) and
`site/status-data/<book_id>.json` (per-book item detail). Cloudflare Pages
deploys `site/**` on every push to `main` with no build step, so these
committed files ARE the published artifact -- this gate is the last thing
standing between a leaked name and a live page.

WHAT THIS SCANS, in TWO passes:

  1. Every committed `.json` file directly under `site/status-data/` plus
     the top-level `site/status-data.json`, walked as a decoded JSON
     document, every string leaf checked against the pinned PCGen oracle's
     own full `NAMEISPI:YES` name index (`scripts/observer/pi_redaction.py
     ::build_declared_pi_name_index`) with EXACT string-leaf equality --
     same exact-match rationale as `site_dashboard_pi_gate.py` (a
     word-boundary/substring scan over ordinary object names
     false-positives on real non-PI names that merely contain a
     declared-PI word, e.g. "Shackles of Compliance").

  2. Per-book detail file (`site/status-data/<book_id>.json`), every
     `kinds[*].items[*].name` checked with a WORD-BOUNDARY scan
     (`pi_redaction.find_declared_pi_word_matches`) against declared-PI
     names FROM THIS FILE'S OWN BOOK union the GLOBAL unambiguous
     declared-PI set, MINUS whatever the reviewed
     `scripts/site/pi_substring_allowlist.py` covers for this exact
     (name, book) pair, and every `kinds[*].items[*].type_facet` checked
     with a plain SUBSTRING scan against the GLOBAL declared-PI name set.
     This is SD31-W13-INTEGRATE-001-VERIFY finding 2 closed (pass 1 alone
     is blind to any name declared PI in a published book but not globally
     unambiguous) AND SITE-PI-ALLOWLIST-001's own finding closed (a
     book-scoped substring scan alone missed the cross-book
     `"Death (Pharasma)"` embed -- see `pi_redaction.
     find_declared_pi_word_matches`'s docstring for the full history).
     Pass 2 mirrors `build_public_status.py`'s own `redact_for_display`
     field-for-field (via the SAME SHARED `pi_redaction` helpers and the
     SAME allow-list module, so the two can never independently drift into
     checking different things) rather than reimplementing its logic here.

This is a SAFETY NET, not the primary defense: the primary defense is
`build_public_status.py`'s own `redact_for_display` (book-scoped name
substring check, plus a `type_facet` substring screen -- see that
function's own docstring for why `type_facet` needs a different,
globally-scoped substring check). This gate exists because a hand-edit, a
reverted redaction, or a future change to the builder that forgets to call
the redaction path are all real failure modes a generation-time fix alone
cannot catch.

Exit 0 and print `site-public-status-pi-gate: CLEAN` when no declared-PI
name is found in any scanned file. Exit 1 and print every hit (file,
JSON path, name) otherwise.

Degraded-oracle posture: if the pinned checkout cannot be found at all, this
prints a loud warning and exits 1 -- a gate that cannot see the oracle
cannot prove anything clean, and "could not check" must never read as
"checked and clean."

Run: python3 scripts/site_public_status_pi_gate.py
Wired as the `site-public-status-pi-gate` stage in `scripts/verify.sh`.
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

STATUS_DATA_TOP = _REPO_ROOT / "site" / "status-data.json"
STATUS_DATA_DIR = _REPO_ROOT / "site" / "status-data"


def scanned_files() -> list[str]:
    """The top-level overview feed plus every committed per-book detail
    file under `site/status-data/`, sorted for a deterministic report."""
    files = []
    if STATUS_DATA_TOP.exists():
        files.append(str(STATUS_DATA_TOP))
    files.extend(sorted(glob.glob(str(STATUS_DATA_DIR / "*.json"))))
    return sorted(files)


def find_status_item_pi_leaks(
    doc, declared_by_length: list[str], book_declared: dict, allowlist_index: dict
) -> list[tuple[str, str]]:
    """Per-book-detail-file leak scan for `build_public_status.py`'s own
    `{"id": <book_id>, "title": ..., "kinds": [{"items": [{"name": ...,
    "type_facet": ...}, ...]}, ...]}` shape. A no-op (returns `[]`) on any
    document not shaped this way -- `find_declared_pi_leaks`'s global,
    book-blind exact-match scan (pass 1, in `main`) remains the net for
    everything else, including the top-level overview feed.

    `name`: WORD-BOUNDARY match (`pi_redaction.find_declared_pi_word_matches`)
    against declared-PI names FROM THIS FILE'S OWN BOOK (`book_declared`)
    UNION the GLOBAL unambiguous declared-PI set (`declared_by_length`),
    with a hit suppressed only when `(name, doc["id"])` is covered by the
    reviewed `pi_substring_allowlist`. See `build_public_status.py`'s
    `redact_for_display` docstring for why BOTH sources are required (the
    book-scoped source alone misses a cross-book embed like
    `"Death (Pharasma)"`; the global-unambiguous source alone misses a
    same-book embed of a name that ALSO has an unrelated row elsewhere in
    the wider Paizo-scoped oracle tree with no NAMEISPI token of its own,
    e.g. `"Baphomet's Blessing"`).

    `type_facet`: plain substring-checked against `declared_by_length`.
    All via the SAME SHARED `pi_redaction` helpers and the SAME allow-list
    module `redact_for_display` itself uses, so this can never drift from
    what the producer checks."""
    hits: list[tuple[str, str]] = []
    if not isinstance(doc, dict):
        return hits
    book_id = doc.get("id")
    kinds = doc.get("kinds")
    if not isinstance(book_id, str) or not isinstance(kinds, list):
        return hits
    own_book_by_length = book_declared.get(book_id, ())
    for ki, kind in enumerate(kinds):
        if not isinstance(kind, dict):
            continue
        items = kind.get("items")
        if not isinstance(items, list):
            continue
        for ii, item in enumerate(items):
            if not isinstance(item, dict):
                continue
            name = item.get("name")
            if isinstance(name, str) and name != pi_redaction.REDACTED_PI_MARKER:
                matches = set(pi_redaction.find_declared_pi_word_matches(name, own_book_by_length))
                matches.update(pi_redaction.find_declared_pi_word_matches(name, declared_by_length))
                if matches and not pi_substring_allowlist.is_allowlisted(name, book_id, allowlist_index):
                    hits.append((
                        f"$.kinds[{ki}].items[{ii}].name",
                        f"{name!r} carries declared-PI word(s) {sorted(matches)!r} in book {book_id!r}, "
                        "not on the reviewed allow-list for this (name, book)",
                    ))
            tf = item.get("type_facet")
            if (
                isinstance(tf, str)
                and tf != pi_redaction.REDACTED_PI_MARKER
                and pi_redaction.value_carries_declared_pi_substring(tf, declared_by_length)
            ):
                hits.append((
                    f"$.kinds[{ki}].items[{ii}].type_facet",
                    f"{tf!r} carries a declared-PI name",
                ))
    return hits


def main() -> int:
    corpus_root = pi_redaction.pcgen_corpus_root()
    if not os.path.isdir(corpus_root):
        print(
            f"site-public-status-pi-gate: FAIL — pinned PCGen oracle not found at "
            f"{corpus_root!r}; a gate that cannot read the oracle cannot prove "
            f"the feed clean (run scripts/fetch-pcgen-oracle.sh)",
            file=sys.stderr,
        )
        return 1

    declared_names = pi_redaction.build_declared_pi_name_index(corpus_root)
    if not declared_names:
        print(
            "site-public-status-pi-gate: FAIL — the pinned oracle sweep found zero "
            "NAMEISPI:YES declarations anywhere; this has never been true of "
            "the real checkout and most likely means the sparse checkout is "
            "broken or empty, not that the corpus is PI-free",
            file=sys.stderr,
        )
        return 1

    declared_by_length = sorted(declared_names, key=len, reverse=True)
    name_to_books = pi_redaction.build_declared_pi_name_book_index(corpus_root)
    book_declared = pi_redaction.build_book_declared_name_lists(name_to_books)
    allowlist_index = pi_substring_allowlist.build_allowlist_index()

    files = scanned_files()
    if not files:
        print(
            "site-public-status-pi-gate: CLEAN — no site/status-data.json or "
            "site/status-data/*.json files are committed yet (nothing to scan)"
        )
        return 0

    all_hits: list[tuple[str, str, str]] = []
    for path in files:
        try:
            with open(path, encoding="utf-8") as f:
                doc = json.load(f)
        except (OSError, json.JSONDecodeError) as exc:
            print(f"site-public-status-pi-gate: FAIL — could not read/parse {path}: {exc}", file=sys.stderr)
            return 1
        rel = os.path.relpath(path, str(_REPO_ROOT))
        for json_path, name in pi_redaction.find_declared_pi_leaks(doc, declared_names):
            all_hits.append((rel, json_path, name))
        for json_path, name in find_status_item_pi_leaks(doc, declared_by_length, book_declared, allowlist_index):
            all_hits.append((rel, json_path, name))

    if all_hits:
        print(
            f"site-public-status-pi-gate: FAIL — {len(all_hits)} declared-PI name(s) "
            f"found across {len(files)} scanned file(s):",
            file=sys.stderr,
        )
        for rel, json_path, name in all_hits:
            print(f"  {rel}:{json_path} carries declared-PI name {name!r}", file=sys.stderr)
        return 1

    print(
        f"site-public-status-pi-gate: CLEAN — {len(files)} file(s) scanned against "
        f"{len(declared_names)} declared-PI name(s), zero leaked"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
