#!/usr/bin/env python3
"""Corpus-wide screen for Product Identity leaking through `data.key` or
`data.raw_tokens` on a record whose bare `data.name` is clean.

**Why this exists (`decisions.md §17` gap-close).** The original per-kind
ingest screens (and `declared_pi_shipping_audit`'s per-row cross-check) only
ever compared a record's `NAME`/`DESC` fields against the PI-blacklist and
the row's own `NAMEISPI:`/`DESCISPI:` declaration. A record can pass that
screen — bare name clean, no `NAMEISPI:YES`/`DESCISPI:YES` — and still ship
a published campaign-setting deity or proper-noun name inside `data.key` or a non-`DESC`
`data.raw_tokens` VALUE, because nothing ever looked there. Two already-
shipped `ability` records proved this live and are fixed by this cycle's
`scripts/ingest_ability.py` change:
`data/corpus/inner_sea_gods/ability/adept.json` and
`data/corpus/inner_sea_magic/ability/diplomatic_student.json`.

**Two separate questions, kept apart on purpose (never conflate them):**

1. **Confirmed leak** — a hit against the operator-**SIGNED-OFF** 60-term
   `PI_BLACKLIST_TERMS` list (`docs/governance/ogl-pi-blacklist.md`,
   `decisions.md §19`), using the same word-boundary, case-fold, OCR-
   normalized scan (`normalized_term_hit`) every other T9-era PI tool uses.
   This is a real, actionable defect.
2. **Unratified-vocabulary candidate** — a capitalized, proper-noun-SHAPED
   token that is NOT on the 60-term list. This is a HEURISTIC over a
   vocabulary nobody has approved (see `CANDIDATE_STOPWORDS` below for its
   documented limits). **This script never redacts or otherwise acts on a
   candidate hit** — it only counts and samples them, for an operator to
   decide whether the blacklist vocabulary should expand
   (`ogl-pi-blacklist.md`'s standing DRAFT-era caution — stop and ask the
   operator rather than guess — binds candidates exactly as hard as it ever
   did, sign-off of the 60-term list notwithstanding).

Run (from repo root, `PCGEN_CORPUS_ROOT` unused — this script reads only the
already-shipped `data/corpus/**` tree, never the oracle):

    python3 scripts/pi_key_rawtokens_audit.py [--kind KIND ...] [--json-out report.json]

Prints a summary to stdout; `--json-out` additionally writes the full
machine-readable report (per-kind, per-book confirmed counts, and a bounded
candidate sample) for citing in a receipt.
"""
from __future__ import annotations

import argparse
import json
import os
import re
import sys
from collections import defaultdict

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from sd32_t9_pi_review_feat_equipment import (  # noqa: E402
    PI_BLACKLIST_TERMS,
    normalized_term_hit,
)

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CORPUS_ROOT = os.path.join(REPO_ROOT, "data/corpus")
REDACTED_PI_MARKER = "[redacted PI]"

# Heuristic-only stoplist for the CANDIDATE (unratified-vocabulary) pass.
# This is deliberately small and does not claim completeness -- it exists
# only to keep the loudest false-positive class (PCGen's own ALLCAPS-ish
# mixed-case field vocabulary and ordinary sentence-initial capitalized
# English words) from drowning every real candidate. A word surviving this
# stoplist is NOT thereby proven to be a proper noun; a word filtered by it
# is NOT thereby proven clean. See the module docstring's point 2.
CANDIDATE_STOPWORDS = {
    "the", "this", "that", "with", "from", "when", "your", "each", "with",
    "while", "which", "these", "those", "their", "there", "where", "what",
    "special", "ability", "abilities", "class", "level", "levels", "spell",
    "spells", "feat", "feats", "bonus", "bonuses", "damage", "check",
    "checks", "save", "saves", "skill", "skills", "effect", "effects",
    "attack", "attacks", "action", "actions", "creature", "creatures",
    "target", "targets", "point", "points", "round", "rounds", "gain",
    "gains", "must", "will", "can", "cannot", "you", "if", "and", "or",
    "not", "as", "per", "of", "in", "on", "at", "to", "a", "an", "is",
    "are", "be", "may", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine", "ten", "yes", "no", "type", "types",
    "category", "cost", "supernatural", "extraordinary", "spell-like",
    "immediate", "standard", "swift", "free", "minute", "minutes", "hour",
    "hours", "day", "days", "half", "full", "additional", "instead",
    "another", "any", "all", "none", "same", "other", "such", "certain",
    "prerequisite", "prerequisites", "benefit", "normal", "special",
    "source", "book", "page", "chapter", "table", "true", "false",
}

# Words already ratified -- a candidate hit that IS one of these belongs in
# the CONFIRMED bucket, not the candidate one; excluded here so the two
# buckets never double-count the same token.
_BLACKLIST_LOWER = {t.lower() for t in PI_BLACKLIST_TERMS}

_CAPWORD_RE = re.compile(r"\b[A-Z][a-z]{3,}\b")


def candidate_terms(text: str) -> set[str]:
    """Capitalized, proper-noun-shaped words in `text` that are NOT already
    on the ratified 60-term list and NOT in the stoplist. See module
    docstring point 2 -- this is a heuristic, not a legal determination."""
    if not text:
        return set()
    found = set()
    for m in _CAPWORD_RE.finditer(text):
        word = m.group(0)
        low = word.lower()
        if low in CANDIDATE_STOPWORDS or low in _BLACKLIST_LOWER:
            continue
        found.add(word)
    return found


def iter_corpus_records(kinds: set[str] | None = None):
    """Yields `(path, kind, book, record_dict)` for every JSON file under
    `data/corpus/<book>/<kind>/*.json`."""
    for book in sorted(os.listdir(CORPUS_ROOT)):
        book_dir = os.path.join(CORPUS_ROOT, book)
        if not os.path.isdir(book_dir):
            continue
        for kind in sorted(os.listdir(book_dir)):
            if kinds is not None and kind not in kinds:
                continue
            kind_dir = os.path.join(book_dir, kind)
            if not os.path.isdir(kind_dir):
                continue
            for fname in sorted(os.listdir(kind_dir)):
                if not fname.endswith(".json"):
                    continue
                path = os.path.join(kind_dir, fname)
                try:
                    with open(path, encoding="utf-8") as fh:
                        record = json.load(fh)
                except (OSError, json.JSONDecodeError):
                    continue
                yield path, kind, book, record


def name_already_flagged(name: str) -> bool:
    """`True` when `name` is either a fresh blacklist hit or already the
    redaction marker -- either way, this record's name is NOT the "clean
    name hiding a leak elsewhere" shape this screen exists to find. See
    `screen_record`'s caller (`main`) for the `§17a`-caught false-positive
    this guards against: a record whose name an EARLIER screen already
    redacted must not be re-reported as a fresh confirmed leak just
    because the literal marker string contains no blacklist term."""
    if not name:
        return False
    return name == REDACTED_PI_MARKER or bool(normalized_term_hit(name))


def screen_record(record: dict) -> dict:
    """Screen one already-shipped record's `data.key`/`data.raw_tokens`
    against both the confirmed (60-term) and candidate (heuristic)
    vocabularies, GIVEN that `data.name` is clean (callers filter first).

    Returns `{"confirmed_terms": set[str], "candidate_terms": set[str]}`.
    A raw_tokens value that is already `[redacted PI]` is skipped -- it is
    not a leak, it is evidence a prior screen already caught it."""
    data = record.get("data") or {}
    key = data.get("key") or ""
    raw_tokens = data.get("raw_tokens") or []

    confirmed: set[str] = set()
    candidates: set[str] = set()

    key_hit = normalized_term_hit(key) if key else None
    if key_hit:
        confirmed.add(key_hit)
    candidates |= candidate_terms(key)

    for t in raw_tokens:
        value = t.get("value") if isinstance(t, dict) else None
        if not value or value == REDACTED_PI_MARKER:
            continue
        hit = normalized_term_hit(value)
        if hit:
            confirmed.add(hit)
        candidates |= candidate_terms(value)

    return {"confirmed_terms": confirmed, "candidate_terms": candidates}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--kind", action="append", default=None, help="restrict to this kind (repeatable)")
    parser.add_argument("--json-out", default=None)
    parser.add_argument("--max-samples", type=int, default=30)
    args = parser.parse_args()

    kinds = set(args.kind) if args.kind else None

    scanned = 0
    name_pi_skipped = 0
    confirmed_by_kind: dict[str, int] = defaultdict(int)
    confirmed_by_book: dict[str, int] = defaultdict(int)
    confirmed_samples: list[dict] = []
    candidate_by_kind: dict[str, int] = defaultdict(int)
    candidate_by_book: dict[str, int] = defaultdict(int)
    candidate_term_counts: dict[str, int] = defaultdict(int)
    candidate_samples: list[dict] = []

    for path, kind, book, record in iter_corpus_records(kinds):
        scanned += 1
        data = record.get("data") or {}
        name = data.get("name") or ""
        if name_already_flagged(name):
            # Name itself already flags this record on the existing
            # name/description screen -- out of THIS screen's job (it
            # exists to find leaks a clean NAME would otherwise hide).
            name_pi_skipped += 1
            continue

        result = screen_record(record)
        rel = os.path.relpath(path, REPO_ROOT)

        if result["confirmed_terms"]:
            confirmed_by_kind[kind] += 1
            confirmed_by_book[book] += 1
            if len(confirmed_samples) < args.max_samples:
                confirmed_samples.append(
                    {"path": rel, "kind": kind, "book": book, "terms": sorted(result["confirmed_terms"])}
                )
        elif result["candidate_terms"]:
            # Candidate bucket is mutually exclusive with confirmed for a
            # single record's REPORTING here (a record already confirmed is
            # not also double-counted as merely a candidate), though the
            # same record's OTHER unratified terms are still worth noting
            # in the sample.
            candidate_by_kind[kind] += 1
            candidate_by_book[book] += 1
            for term in result["candidate_terms"]:
                candidate_term_counts[term] += 1
            if len(candidate_samples) < args.max_samples:
                candidate_samples.append(
                    {"path": rel, "kind": kind, "book": book, "terms": sorted(result["candidate_terms"])}
                )

    report = {
        "scanned_records": scanned,
        "name_already_pi_skipped": name_pi_skipped,
        "term_list_size": len(PI_BLACKLIST_TERMS),
        "confirmed": {
            "total_records": sum(confirmed_by_kind.values()),
            "by_kind": dict(sorted(confirmed_by_kind.items())),
            "by_book": dict(sorted(confirmed_by_book.items())),
            "samples": confirmed_samples,
        },
        "candidate_unratified_vocabulary": {
            "total_records": sum(candidate_by_kind.values()),
            "by_kind": dict(sorted(candidate_by_kind.items())),
            "by_book": dict(sorted(candidate_by_book.items())),
            "top_terms": dict(sorted(candidate_term_counts.items(), key=lambda kv: -kv[1])[:60]),
            "samples": candidate_samples,
        },
    }

    text = json.dumps(report, indent=2)
    print(
        f"scanned={scanned} name_already_pi_skipped={name_pi_skipped} "
        f"confirmed_records={report['confirmed']['total_records']} "
        f"candidate_records={report['candidate_unratified_vocabulary']['total_records']}"
    )
    if args.json_out:
        with open(args.json_out, "w", encoding="utf-8") as fh:
            fh.write(text)
    else:
        print(text)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
