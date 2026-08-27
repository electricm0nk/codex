#!/usr/bin/env python3
"""Generic ingest of `Kind::Companion`'s enumerated-but-engine-does-not-hold units into
`data/corpus/<book>/companion/*.json` (SD-32 `decisions.md §20`).

**Why this is a separate lever from `companion_chassis.rs`/
`transcribe_companion_tables.py`.** That pipeline solves a harder problem
than `scripts/shape_ledger.py` asks: it resolves each ability row's OWNING
creature (so the render layer can attach the ability to the right chassis),
and it deliberately DROPS every row it cannot prove an owner for -- orphan
ability rows, `.COPY=`/`.MOD` delta rows, `*_classes_companion.lst` class
rows, and `PRECAMPAIGN`-gated rows -- from its emitted table, which is the
correct anti-gaming posture for reachability (`decisions.md §1a`: never
fabricate an ownership claim the corpus doesn't state).

`shape_ledger.py`'s `no_record` join asks a narrower, prior question: does a
corpus record exist at all for this `(book, source_file, source_line)`
citation, so its formula shape can be measured? That does not require
resolving ownership -- `decisions.md §20`: "an un-ingested object's shape
cannot be measured... Gate 1's DoD is that every unit's shape is measured."
This script answers that question directly: literal, verbatim transcription
of every not-yet-ingested `companion` unit's own cited row, independent of
whether its owning creature is known. It makes NO reachability or rendering
claim -- an emitted record's `owners` field is always `[]`, and this script
does not touch `companion_chassis.rs`, `gen_book_cache.rs`, or any render
path. Reachability is a separate, later question this cycle does not answer.

Confirmed by direct measurement (not assumed): of the 769 `companion`
`no_record` units this cycle found, 768 are exactly the union of
`scripts/classify_companion_rows.py`'s own `orphans`/`deltas`/`classes`/
`gated` categories, re-derived per book against the pinned oracle. The
769th (`bestiary:companion:pseudodragon_tail`) is a rendering-side gap
(`engine_book` resolution), not a missing corpus record, and this script's
generic per-unit ingest covers it identically -- it does not special-case
any of these categories, it transcribes every `status: engine-does-not-hold`
`companion` unit's own row.

**Generic, not per-book** (`decisions.md §17`): resolves each unit's own
`(book, source_file)` citation against the pinned oracle by directory
basename + filename search, exactly `scripts/ingest_ability.py`'s method
(imported nothing from it only because that script has no importable
surface -- the method, not the code, is reused). No book is named in this
file's own source.

**Nothing is computed.** Every emitted `raw_tokens` entry is a verbatim
substring of the cited row (skip the identity column, split each remaining
tab field on its first `:`). `corpus_literal_sweep` independently re-derives
the same tokens from the same citation to confirm the copy byte for byte.
A `.COPY=`/`.MOD` delta row's own tokens are transcribed exactly as the row
states them -- this is NOT a merged/resolved record, and the emitted JSON's
`data.origin` field carries the unit's own `copy`/`mod_only`/`declared`
value so a later reader is never misled into treating a delta row's tokens
as the base record's full definition.

**Product Identity — the exact `decisions.md §19a`/`§19c`-approved companion
screen, reused rather than re-derived a third time.** Imports
`scripts/sd32_t9_pi_exposure_audit.py::classify_row` (the exact declared-tag
+ 60-term blacklist scan) and `scripts/sd32_t9_pi_review_companion_
monsterability.py::normalized_scan`/`classify_uncertain_content` (the
operator-approved normalized re-scan and per-record content classifier that
resolved companion's 443-unit uncertain bucket). Disposition, identical to
that review's own chain:

1. `classify_row` blocked (`NAMEISPI:YES`/`DESCISPI:YES` declared, or an
   exact blacklist-term hit) -> **skip**, name it, per §15/§18/§19's
   standing rule.
2. `normalized_scan` hit (case-fold + OCR-fold catches what the exact scan
   missed) -> **skip**, name it.
3. Free-text tag present, no hit above, `classify_uncertain_content` returns
   `still_undecidable` (capitalized token outside the allowlist, or a
   lowercase creature-species-shaped reference) -> **skip**, name it. This is
   a real disposition, not a defect: forcing it to `clear` to produce a
   tidier number is exactly what `decisions.md §18`'s standing constraints
   forbid.
4. Otherwise (no free-text tag at all, or `clear`) -> **transcribe**.

Never transcribe, never silently skip: every skip is named in the run
report's `pi_skipped_records`.

Run: `python3 scripts/ingest_companion.py [--dry-run] [--out <report.json>]`
`PCGEN_CORPUS_ROOT` must point at a pinned PCGen `data/` checkout.
"""
from __future__ import annotations

import hashlib
import json
import os
import re
import subprocess
import sys
from collections import defaultdict

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import sd32_t9_pi_exposure_audit as pi_audit  # noqa: E402
from sd32_t9_pi_review_companion_monsterability import (  # noqa: E402
    classify_uncertain_content,
    extract_free_text,
    normalized_scan,
)

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
INVENTORY_PATH = os.path.join(REPO_ROOT, "docs/work-inventory.json")
REDACTED_PI_MARKER = "[redacted PI]"  # src/rules_core/shape_b_v1.rs::REDACTED_PI_MARKER
PI_MARKER_REDACTED = "redacted"  # src/rules_core/shape_b_v1.rs::PI_MARKER_REDACTED
CORE_ESSENTIALS_BASENAME = "core_essentials"

# `scripts/shape_ledger.py::BOOK_CORPUS_DIR_ALIASES` -- the corpus-record
# writer must agree with the READER's directory choice
# (SD-32 wave-1 lesson: 30 records landed under a literal `bestiary/` and
# were invisible to the join until diffed and caught).
CORPUS_WRITE_DIR_ALIASES: dict[str, str] = {
    "bestiary": "beastiary",
}


def corpus_root() -> str:
    return os.environ.get("PCGEN_CORPUS_ROOT", os.path.expanduser("~/workspace/repos/pcgen/data"))


def build_dir_index(root: str) -> dict[str, list[str]]:
    index: dict[str, list[str]] = defaultdict(list)
    for dirpath, _dirnames, _filenames in os.walk(root):
        index[os.path.basename(dirpath)].append(dirpath)
    return index


def find_file_under(root_dir: str, filename: str) -> list[str]:
    hits = []
    for dirpath, _dirnames, filenames in os.walk(root_dir):
        if filename in filenames:
            hits.append(os.path.join(dirpath, filename))
    return sorted(hits)


def resolve_file(dir_index: dict[str, list[str]], root: str, book: str, filename: str) -> list[str]:
    book_dirs = dir_index.get(book, [])
    if len(book_dirs) == 1:
        hits = find_file_under(book_dirs[0], filename)
        if hits:
            return hits
    elif len(book_dirs) > 1:
        return []  # ambiguous book directory -- caller reports, never guesses
    ce_dirs = dir_index.get(CORE_ESSENTIALS_BASENAME, [])
    if len(ce_dirs) == 1 and ce_dirs[0] not in book_dirs:
        return find_file_under(ce_dirs[0], filename)
    return []


def read_row(path: str, line_no: int) -> str:
    """Byte-verbatim (soft hyphen kept, never substituted) so
    `corpus_literal_sweep`'s independent re-derivation matches -- same rule
    `ingest_ability.py::read_row` documents and for the same reason."""
    with open(path, encoding="utf-8", errors="replace") as fh:
        lines = fh.read().split("\n")
    if line_no < 1 or line_no > len(lines):
        return ""
    return lines[line_no - 1]


def row_tokens(line: str) -> list[dict[str, str]]:
    fields = [f.strip() for f in line.split("\t") if f.strip()]
    fields = fields[1:]
    tokens = []
    for field in fields:
        if ":" in field:
            key, value = field.split(":", 1)
        else:
            key, value = field, ""
        tokens.append({"key": key, "value": value})
    return tokens


def desc_value(tokens: list[dict[str, str]]) -> str | None:
    for t in tokens:
        if t["key"] == "DESC":
            return t["value"]
    return None


def slugify(name: str, used: set[str]) -> str:
    base = re.sub(r"[^a-z0-9]+", "_", name.lower()).strip("_") or "unnamed"
    slug = base
    n = 2
    while slug in used:
        slug = f"{base}_{n}"
        n += 1
    used.add(slug)
    return slug


def sha256_file(path: str) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as fh:
        h.update(fh.read())
    return h.hexdigest()


def ingested_at_now() -> str:
    out = subprocess.run(["date", "-u", "+%Y-%m-%dT%H:%M:%SZ"], capture_output=True, text=True, check=True)
    return out.stdout.strip()


def load_units() -> list[dict]:
    with open(INVENTORY_PATH, encoding="utf-8") as fh:
        doc = json.load(fh)
    units = doc["units"] if isinstance(doc, dict) and "units" in doc else doc
    # `status == "engine-does-not-hold"` only -- companion, unlike wave 1's `ability`,
    # already has ~900 records shipped through `companion_chassis.rs`. This
    # script must never touch those; re-writing an already-`grounded`/
    # `fixture-verified` unit's file would destroy a real verification stamp.
    return [u for u in units if u.get("kind") == "companion" and u.get("status") == "engine-does-not-hold"]


def existing_slugs_by_book(root_repo: str, units: list[dict]) -> dict[str, set[str]]:
    """Seed each book's `used` slug set from files ALREADY on disk (the
    engine-emitted `companion_chassis` records), not just this run's own
    output, so a coincidental name collision never overwrites an existing
    record."""
    out: dict[str, set[str]] = defaultdict(set)
    books = {u["book"] for u in units}
    for book in books:
        write_dir_book = CORPUS_WRITE_DIR_ALIASES.get(book, book)
        d = os.path.join(root_repo, "data/corpus", write_dir_book, "companion")
        if os.path.isdir(d):
            for fn in os.listdir(d):
                if fn.endswith(".json"):
                    out[book].add(fn[: -len(".json")])
    return out


def existing_citations_by_book(root_repo: str, books: set[str]) -> dict[str, set[tuple[str, int]]]:
    """Index every already-written `data/corpus/<book>/companion/*.json`
    record's own `(source.path, source.line)` citation, per book.

    **Why this exists (SD-32 T9-onboarding-cause-closure, 2026-08-23).**
    `docs/work-inventory.json`'s `status` field is not updated by this
    script writing a record -- it flips to something other than
    `"engine-does-not-hold"` only when `v06_work_inventory` is rebuilt and re-run.
    A second pass over the same stale inventory (e.g. after a PI-allowlist
    widening) would otherwise re-process units a prior pass already wrote;
    `slugify()`'s collision-avoidance means it would allocate a NEW
    suffixed slug rather than overwrite, producing a duplicate record for
    the identical PCGen citation. This index lets `main()` recognize that
    case and skip it before a slug is ever allocated."""
    out: dict[str, set[tuple[str, int]]] = defaultdict(set)
    for book in books:
        write_dir_book = CORPUS_WRITE_DIR_ALIASES.get(book, book)
        d = os.path.join(root_repo, "data/corpus", write_dir_book, "companion")
        if not os.path.isdir(d):
            continue
        for fn in os.listdir(d):
            if not fn.endswith(".json"):
                continue
            try:
                with open(os.path.join(d, fn), encoding="utf-8") as fh:
                    rec = json.load(fh)
            except (OSError, ValueError):
                continue
            src = rec.get("source") or {}
            path = src.get("path")
            line = src.get("line")
            if path is not None and line is not None:
                out[book].add((path, line))
    return out


def main() -> int:
    dry_run = "--dry-run" in sys.argv
    out_path = None
    if "--out" in sys.argv:
        out_path = sys.argv[sys.argv.index("--out") + 1]

    root = corpus_root()
    if not os.path.isdir(root):
        print(f"PCGEN_CORPUS_ROOT ({root}) is not a directory", file=sys.stderr)
        return 1

    units = load_units()
    dir_index = build_dir_index(root)

    report = {
        "population": len(units),
        "written": 0,
        "pi_skipped": 0,
        "skipped_existing_already_ingested": 0,
        "unresolved": [],
        "written_by_book": defaultdict(int),
        "pi_skipped_records": [],
        "pi_skipped_by_bucket": defaultdict(int),
    }

    file_cache: dict[tuple[str, str], list[str]] = {}
    used_by_book = existing_slugs_by_book(REPO_ROOT, units)
    books = {u["book"] for u in units}
    citations_by_book = existing_citations_by_book(REPO_ROOT, books)
    ingested_at = ingested_at_now()

    for unit in units:
        book = unit["book"]
        source_file = unit["source_file"]
        line = unit["source_line"]
        key = unit.get("corpus_key") or unit.get("key") or unit["name"]
        name = unit["name"]
        origin = unit.get("origin")

        cache_key = (book, source_file)
        if cache_key not in file_cache:
            file_cache[cache_key] = resolve_file(dir_index, root, book, source_file)
        hits = file_cache[cache_key]
        if len(hits) != 1:
            report["unresolved"].append(
                {"book": book, "source_file": source_file, "hits": len(hits), "key": key}
            )
            continue
        path = hits[0]

        # Idempotency guard (SD-32 T9-onboarding-cause-closure, 2026-08-23):
        # `docs/work-inventory.json`'s `status` field does not flip when this
        # script writes a record, so a re-run over the same stale inventory
        # must recognize a unit a prior pass already ingested by its own
        # (source path, source line) citation -- never re-slug and duplicate it.
        rel_path_for_citation = os.path.relpath(path, root)
        if (rel_path_for_citation, line) in citations_by_book.get(book, ()):
            report["skipped_existing_already_ingested"] += 1
            continue

        raw_line = read_row(path, line)
        tokens = row_tokens(raw_line)

        # --- PI screen: decisions.md §19a/§19c-approved companion chain ---
        exact_bucket, exact_reason = pi_audit.classify_row(raw_line)
        free_text = extract_free_text(raw_line)
        norm_hit = normalized_scan(free_text)

        if exact_bucket == "blocked":
            final_bucket, final_reason = "blocked", exact_reason
        elif norm_hit is not None:
            final_bucket = "blocked"
            final_reason = f'normalized-scan hit (case-fold + OCR-fold): "{norm_hit}"'
        elif exact_bucket == "uncertain":
            final_bucket, final_reason = classify_uncertain_content(free_text)
        else:
            final_bucket, final_reason = "clear", exact_reason

        if final_bucket != "clear":
            report["pi_skipped"] += 1
            report["pi_skipped_by_bucket"][final_bucket] += 1
            report["pi_skipped_records"].append(
                f"{book}:{source_file}:{line} '{name}' (key: '{key}') -- {final_bucket}: {final_reason}"
            )
            continue

        description = desc_value(tokens)
        has_formula_token = any(t["key"] == "DEFINE" or t["key"].startswith("BONUS") for t in tokens)
        wiring_class = "static" if has_formula_token else "display"
        wiring_signals = (
            ["static:has_magnitude_token"] if has_formula_token else ["display:no_magnitude_token"]
        )

        rel_path = os.path.relpath(path, root)
        sha256 = sha256_file(path)
        used = used_by_book[book]
        slug = slugify(name, used)

        record = {
            "population": "in_scope",
            "completeness": "full" if description else "chassis_only",
            "ingested_at": ingested_at,
            "data": {
                "key": key,
                "name": name,
                "description": description,
                "raw_tokens": tokens,
                # Not a merged/resolved record for `.COPY=`/`.MOD` origin --
                # this row's own tokens only, verbatim. `owners: []` states
                # plainly that this script makes no reachability claim;
                # `companion_chassis.rs` (a separate mechanism) is what
                # would resolve ownership for rendering.
                "origin": origin,
                "owners": [],
            },
            "source": {
                "kind": "lst_token",
                "path": rel_path,
                "sha256": sha256,
                "line": line,
                "record_key": key,
            },
            "wiring_class": wiring_class,
            "wiring_class_signals": wiring_signals,
            "license": "OGL",
            "pi_field": None,
            "pi_marker": None,
        }

        write_dir_book = CORPUS_WRITE_DIR_ALIASES.get(book, book)
        out_dir = os.path.join(REPO_ROOT, "data/corpus", write_dir_book, "companion")
        if not dry_run:
            os.makedirs(out_dir, exist_ok=True)
            with open(os.path.join(out_dir, f"{slug}.json"), "w", encoding="utf-8") as fh:
                json.dump(record, fh, indent=2, ensure_ascii=False)
                fh.write("\n")
        report["written"] += 1
        report["written_by_book"][book] += 1

    report["written_by_book"] = dict(sorted(report["written_by_book"].items()))
    report["pi_skipped_by_bucket"] = dict(sorted(report["pi_skipped_by_bucket"].items()))
    text = json.dumps(report, indent=2)
    print(text)
    if out_path:
        with open(out_path, "w", encoding="utf-8") as fh:
            fh.write(text)
    return 0


if __name__ == "__main__":
    sys.exit(main())
