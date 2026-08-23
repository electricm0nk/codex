#!/usr/bin/env python3
"""Generic ingest of `Kind::RaceTrait`'s remaining `no_record` units into
`data/corpus/<book>/race_trait_generic/*.json` -- a directory SIBLING to
`race_trait/`, deliberately not inside it.

**Why a sibling directory, not `race_trait/` itself.** `race_trait/` already
holds the semantically-resolved, engine-reachable records the Rust ingesters
build (a `race_key`/`category`/`type_tokens` schema, nested one directory
per race for several books). At least one existing test
(`tests/v06_work_inventory.rs::
arg_race_file_carries_favored_class_bonus_and_choice_suboption_rows_not_traits`)
walks `data/corpus/advanced_race_guide/race_trait/` assuming every entry is a
race-named subdirectory of that richer shape, and asserts a `category` field
this generic transcriber's flatter schema does not carry. `scripts/
shape_ledger.py::build_corpus_index` walks `data/corpus/<book>/**/*.json`
with no subdirectory-name filter, so a sibling directory is exactly as
measurable for Gate-1 purposes while touching zero existing consumers of the
curated `race_trait/` shape -- consistent with this bundle's own "Gate-1
measurability and player-reachability are different claims" ruling (this
package's dispatch brief, "Lessons wave 1 paid for" item 4): these records
are measurable, not (yet) engine-reachable through the race picker.

SD-32 `decisions.md §20`: `no_record` means an object was never ingested, so
its shape cannot be measured. `race_trait` carries the largest remaining
`no_record` population of any kind (1,883 at `857eb85d0`,
`python3 scripts/shape_ledger.py --inventory docs/work-inventory.json`, count
`join_status == "no_record"` and `kind == "race_trait"`). `race_trait`'s own
Rust ingesters (`ingest_race_traits.rs`, `ingest_apg_race_traits.rs`,
`ingest_races.rs`) build semantically-resolved, engine-reachable records
scoped to a specific playable race chassis -- real, valuable work, but
necessarily per-race and per-book. This script does the SAME thing
`scripts/ingest_ability.py` (`decisions.md §17`'s wave-1 precedent, 4,248
units) already proved safe for `ability`: a generic, book-agnostic,
kind-filtered VERBATIM transcription that makes a row's shape measurable
without asserting it resolves through the race picker. Gate-1 measurability
and player-reachability are different claims (this bundle's own dispatch
brief, "Lessons wave 1 paid for" item 4) -- this script advances only the
first.

**Scope, precisely.** Only units whose `(book, source_file, source_line)`
join against the pinned oracle corpus currently returns `no_record`
(re-derived fresh via `scripts/shape_ledger.py`, not read off the possibly-
stale `status` field in `docs/work-inventory.json` -- `status` and the live
corpus join have drifted before, see `epic-2-t2b-pure-ability-pointer-row-
fix_cycle-1_cycle_receipt.md` finding 5/6). A unit already carrying a real
corpus record (from the Rust ingesters, or a prior generic pass) is never
touched, so this script can be re-run safely after any of those land more
units.

**Generic, not per-book** (`decisions.md §17`): resolution mirrors
`ingest_ability.py::resolve_file` exactly -- a directory whose basename
equals the unit's `book` field, falling back to `core_essentials`. No book
is named in this file's own source.

**Nothing is computed.** Every emitted `raw_tokens` entry is a verbatim
substring of the cited row (skip the identity column, split each remaining
tab field on its first `:`) -- identical shape to `ingest_ability.py::
row_tokens`, so `corpus_literal_sweep` verifies it the same way.

**Product Identity** (`decisions.md §15/§19`): identical two-screen
discipline to `ingest_ability.py` -- a `NAMEISPI:YES` declaration or a
name-blacklist hit (checked against both `name` and the full `key`) skips
the whole record and is named in the run's own JSON report under
`pi_skipped_records`, never silently dropped. A `DESCISPI:YES` declaration
or a description-blacklist hit redacts only the `DESC` field.

**`BOOK_CORPUS_DIR_ALIASES`** (`scripts/shape_ledger.py`'s reader-side
alias) is honoured on the WRITE side too, via the identical
`CORPUS_WRITE_DIR_ALIASES` table `ingest_ability.py` already carries --
`bestiary` writes under the physical `beastiary` directory, mirroring the
30-unit near-miss that table's own comment documents.

Run: `python3 scripts/ingest_race_trait_generic.py [--dry-run] [--out <report.json>]`
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
from sd32_t9_pi_review_feat_equipment import (  # noqa: E402
    PI_BLACKLIST_TERMS,
    extract_free_text,
    normalized_term_hit,
)

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
INVENTORY_PATH = os.path.join(REPO_ROOT, "docs/work-inventory.json")
REDACTED_PI_MARKER = "[redacted PI]"  # src/rules_core/shape_b_v1.rs::REDACTED_PI_MARKER
PI_MARKER_REDACTED = "redacted"  # src/rules_core/shape_b_v1.rs::PI_MARKER_REDACTED
CORE_ESSENTIALS_BASENAME = "core_essentials"

# Mirrors `ingest_ability.py::CORPUS_WRITE_DIR_ALIASES` /
# `scripts/shape_ledger.py::BOOK_CORPUS_DIR_ALIASES` exactly -- the writer
# must agree with the reader's directory choice.
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
    with open(path, encoding="utf-8", errors="replace") as fh:
        lines = fh.read().split("\n")
    if line_no < 1 or line_no > len(lines):
        return ""
    return lines[line_no - 1]


def row_tokens(line: str) -> list[dict[str, str]]:
    """Skip the identity column, split each remaining field on its first `:`.
    Mirrors `ingest_ability.py::row_tokens` / `cache_gen::class_feature::
    row_tokens` exactly."""
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


def declared_pi(tokens: list[dict[str, str]]) -> tuple[bool, bool]:
    name_declared = False
    desc_declared = False
    for t in tokens:
        if t["value"].strip().upper() != "YES":
            continue
        if t["key"].upper() == "NAMEISPI":
            name_declared = True
        elif t["key"].upper() == "DESCISPI":
            desc_declared = True
    return name_declared, desc_declared


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


def load_no_record_ids(ledger_path: str) -> set[str]:
    """The set of unit ids `scripts/shape_ledger.py` reports `join_status ==
    "no_record"` for kind `race_trait`, read from a ledger already generated
    against `docs/work-inventory.json`. The live join, not the possibly-stale
    `status` field, is the ground truth for "needs a corpus record"."""
    with open(ledger_path, encoding="utf-8") as fh:
        ledger = json.load(fh)
    rows = ledger["rows"] if isinstance(ledger, dict) and "rows" in ledger else ledger
    return {
        r["id"]
        for r in rows
        if r.get("kind") == "race_trait" and r.get("join_status") == "no_record"
    }


def load_units(no_record_ids: set[str]) -> list[dict]:
    with open(INVENTORY_PATH, encoding="utf-8") as fh:
        doc = json.load(fh)
    units = doc["units"] if isinstance(doc, dict) and "units" in doc else doc
    return [u for u in units if u.get("kind") == "race_trait" and u.get("id") in no_record_ids]


def main() -> int:
    dry_run = "--dry-run" in sys.argv
    out_path = None
    if "--out" in sys.argv:
        out_path = sys.argv[sys.argv.index("--out") + 1]
    ledger_path = os.path.join(REPO_ROOT, "docs/work-inventory.json")
    if "--ledger" in sys.argv:
        ledger_path = sys.argv[sys.argv.index("--ledger") + 1]
    else:
        ledger_path = None

    root = corpus_root()
    if not os.path.isdir(root):
        print(f"PCGEN_CORPUS_ROOT ({root}) is not a directory", file=sys.stderr)
        return 1

    if ledger_path is None:
        print("--ledger <shape_ledger_output.json> is required", file=sys.stderr)
        return 1

    no_record_ids = load_no_record_ids(ledger_path)
    units = load_units(no_record_ids)
    dir_index = build_dir_index(root)

    report = {
        "population": len(units),
        "written": 0,
        "name_pi_skipped": 0,
        "unresolved": [],
        "written_by_book": defaultdict(int),
        "pi_skipped_records": [],
    }

    file_cache: dict[tuple[str, str], list[str]] = {}
    used_by_book: dict[str, set[str]] = defaultdict(set)
    ingested_at = ingested_at_now()

    for unit in units:
        book = unit["book"]
        source_file = unit["source_file"]
        line = unit["source_line"]
        key = unit.get("corpus_key") or unit.get("key") or unit["name"]
        name = unit["name"]

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

        raw_line = read_row(path, line)
        tokens = row_tokens(raw_line)
        name_declared, desc_declared = declared_pi(tokens)
        name_hit = normalized_term_hit(name) or normalized_term_hit(key)
        if name_declared or name_hit:
            report["name_pi_skipped"] += 1
            report["pi_skipped_records"].append(
                f"{book}:{source_file}:{line} '{name}' (key: '{key}')"
                + (" (NAMEISPI:YES)" if name_declared else f" (term: {name_hit})")
            )
            continue

        description = desc_value(tokens)
        free_text = extract_free_text(raw_line)
        desc_hit = normalized_term_hit(free_text) if free_text else None
        pi_redacted = desc_declared or bool(desc_hit)
        stored_description = description
        if pi_redacted and description is not None:
            stored_description = REDACTED_PI_MARKER
            for t in tokens:
                if t["key"] == "DESC":
                    t["value"] = REDACTED_PI_MARKER

        has_formula_token = any(t["key"] == "DEFINE" or t["key"].startswith("BONUS") for t in tokens)
        wiring_class = "static" if has_formula_token else "display"
        wiring_signals = (
            ["static:has_magnitude_token"] if has_formula_token else ["display:no_magnitude_token"]
        )

        rel_path = os.path.relpath(path, root)
        sha256 = sha256_file(path)
        used = used_by_book[book]
        slug = slugify(key, used)

        record = {
            "population": "in_scope",
            "completeness": "full" if stored_description else "chassis_only",
            "ingested_at": ingested_at,
            "data": {
                "key": key,
                "name": name,
                "description": stored_description,
                "raw_tokens": tokens,
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
            "license": "PI-REDACTED" if pi_redacted else "OGL",
            "pi_field": "description" if pi_redacted else None,
            "pi_marker": PI_MARKER_REDACTED if pi_redacted else None,
        }

        write_dir_book = CORPUS_WRITE_DIR_ALIASES.get(book, book)
        out_dir = os.path.join(REPO_ROOT, "data/corpus", write_dir_book, "race_trait_generic")
        if not dry_run:
            os.makedirs(out_dir, exist_ok=True)
            with open(os.path.join(out_dir, f"{slug}.json"), "w", encoding="utf-8") as fh:
                json.dump(record, fh, indent=2, ensure_ascii=False)
                fh.write("\n")
        report["written"] += 1
        report["written_by_book"][book] += 1

    report["written_by_book"] = dict(sorted(report["written_by_book"].items()))
    report["term_list_size"] = len(PI_BLACKLIST_TERMS)
    text = json.dumps(report, indent=2)
    print(text)
    if out_path:
        with open(out_path, "w", encoding="utf-8") as fh:
            fh.write(text)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
