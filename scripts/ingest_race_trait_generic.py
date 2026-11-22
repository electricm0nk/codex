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
from pi_scrub import (  # noqa: E402
    blacklist_term_hit_including_concatenated,
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


def find_owned_race_trait_files(book_filter: str | None) -> list[str]:
    """Every `data/corpus/<book>/race_trait_generic/*.json` file that does
    **not** carry a `codex_generated_name` key -- the structural ownership
    marker `scripts/ingest_generic_kind.py` (and only that script) stamps on
    every record it writes, including when invoked with `--kind race_trait`
    against a `NAMEISPI:YES`/name-blacklisted unit this script itself always
    skips outright (this script has no rename path of its own -- see
    `remediate` below). `race_trait_generic/` is a directory SHARED between
    the two scripts (one of the two "dormant shared-directory pairs" named
    by `t9-generic-ingest-remediation-mode_cycle-1_cycle_receipt.md`); a
    blanket rewrite of everything in it would touch the sibling script's
    records.

    Verified empirically, not merely assumed sound (`decisions.md §17a`):
    `python3 scripts/pi_key_rawtokens_audit.py --kind race_trait_generic`
    (2026-08-23) scans 1884 files corpus-wide; exactly 6 carry
    `codex_generated_name` (`ingest_generic_kind.py`'s own `--kind
    race_trait` output). Every one of the other 1878 was checked field-by-
    field against this script's own exact write schema (`population`,
    `completeness`, `ingested_at`, `data{key,name,description,raw_tokens}`,
    `source{kind,path,sha256,line,record_key}`, `wiring_class`,
    `wiring_class_signals`, `license`, `pi_field`, `pi_marker` -- no extra
    key) with zero mismatches, so "absent `codex_generated_name`" identifies
    exactly this script's own population here, not a default that could
    silently include a third writer's records."""
    out: list[str] = []
    corpus_root_dir = os.path.join(REPO_ROOT, "data/corpus")
    for dirpath, _dirnames, filenames in os.walk(corpus_root_dir):
        if os.path.basename(dirpath) != "race_trait_generic":
            continue
        book_dir = os.path.basename(os.path.dirname(dirpath))
        if book_filter and book_dir != book_filter:
            continue
        for fn in sorted(filenames):
            if not fn.endswith(".json"):
                continue
            path = os.path.join(dirpath, fn)
            try:
                with open(path, encoding="utf-8") as fh:
                    rec = json.load(fh)
            except (OSError, json.JSONDecodeError):
                continue
            if "codex_generated_name" in rec:
                continue  # ingest_generic_kind.py's own record -- never touched
            out.append(path)
    return out


def remediate(root: str, book_filter: str | None, dry_run: bool, out_path: str | None) -> int:
    """`decisions.md §17`'s gap-close for the structural defect
    `t9-generic-ingest-remediation-mode_cycle-1_cycle_receipt.md` names for
    this script: the ordinary writer above is `no_record`-ledger-gated and
    can therefore never re-touch a record it already shipped, even when the
    CURRENT scrub pipeline would now catch a leak in it. `--remediate`
    bypasses the ledger entirely: it walks every SELF-OWNED
    (`find_owned_race_trait_files`) record already on disk, re-reads its own
    pinned-oracle citation (`source.path` + `source.line` -- the coordinate
    the ORIGINAL ingest used, never re-resolved by name), and re-derives the
    record from scratch with the CURRENT redaction pipeline -- the same
    `declared_pi`/`normalized_term_hit` checks the ordinary writer uses,
    PLUS a scan of every `raw_tokens` VALUE (not only `DESC` and the row's
    free text) against `scripts/pi_scrub.py::
    blacklist_term_hit_including_concatenated` (imported, never re-defined
    -- `decisions.md §17`), mirroring the same gap-close
    `ingest_generic_kind.py`'s own `--remediate` already closed for its
    kinds. A record is rewritten only if its content (everything except
    `ingested_at`) actually changed.

    This script's ordinary writer SKIPS a name-PI unit outright at ingest
    time -- it has no Codex-generated-neutral-name rename path the way
    `ingest_generic_kind.py` does. If re-derivation finds a previously-clean
    shipped record's name/key NOW hits the blacklist (e.g. a term added
    since the original ingest), `remediate` does not invent an unapproved
    rename scheme for it, and it does not delete the record (that would
    move `no_record`, which this mode must never do). It is reported under
    `name_pi_newly_detected` by coordinate for an operator ruling, and left
    untouched -- `decisions.md §15`'s stop-the-cycle-on-that-record
    discipline, not a silent skip."""
    paths = find_owned_race_trait_files(book_filter)
    ingested_at = ingested_at_now()

    report = {
        "mode": "remediate",
        "book_filter": book_filter,
        "scanned": len(paths),
        "changed": 0,
        "unchanged": 0,
        "unresolved": [],
        "changed_paths": [],
        "name_pi_newly_detected": [],
    }

    for path in paths:
        with open(path, encoding="utf-8") as fh:
            rec = json.load(fh)

        src_rel = rec["source"]["path"]
        src_line = rec["source"]["line"]
        src_path = os.path.join(root, src_rel)
        book_dir = os.path.basename(os.path.dirname(os.path.dirname(path)))
        # Reverse the write-side alias so the book NAME (not the physical
        # directory) is what's used in reporting -- the alias is a
        # directory-naming detail, not part of a unit's coordinate.
        book = next((b for b, d in CORPUS_WRITE_DIR_ALIASES.items() if d == book_dir), book_dir)

        if not os.path.isfile(src_path):
            report["unresolved"].append(path)
            continue

        raw_line = read_row(src_path, src_line)
        tokens = row_tokens(raw_line)
        name_declared, desc_declared = declared_pi(tokens)

        name = rec["data"]["name"]
        key = rec["data"]["key"]
        name_hit = normalized_term_hit(name) or normalized_term_hit(key)
        if name_declared or name_hit:
            report["name_pi_newly_detected"].append(
                f"{book}:{os.path.basename(src_rel)}:{src_line}"
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

        # `decisions.md §17` gap-close, mirroring `ingest_generic_kind.py`'s
        # own `--remediate` widening: a record whose bare name/description
        # are clean can still carry a blacklisted term inside a DIFFERENT
        # raw-token value. Scan EVERY token's value, not only `DESC`.
        blacklist_extra_redacted = False
        scrubbed = []
        for t in tokens:
            value = t["value"]
            if pi_redacted and t["key"] == "DESC" and value == REDACTED_PI_MARKER:
                scrubbed.append(dict(t))
                continue
            if value and blacklist_term_hit_including_concatenated(value):
                scrubbed.append({"key": t["key"], "value": REDACTED_PI_MARKER})
                blacklist_extra_redacted = True
            else:
                scrubbed.append(dict(t))
        tokens = scrubbed

        fields_redacted: list[str] = []
        if pi_redacted:
            fields_redacted.append("description")
        if blacklist_extra_redacted:
            fields_redacted.append("raw_tokens")

        new_record = {
            "population": rec.get("population", "in_scope"),
            "completeness": "full" if stored_description else "chassis_only",
            "ingested_at": rec.get("ingested_at"),
            "data": {
                "key": key,
                "name": name,
                "description": stored_description,
                "raw_tokens": tokens,
            },
            "source": rec["source"],
            "wiring_class": wiring_class,
            "wiring_class_signals": wiring_signals,
            "license": "PI-REDACTED" if fields_redacted else "OGL",
            "pi_field": ",".join(fields_redacted) if fields_redacted else None,
            "pi_marker": PI_MARKER_REDACTED if fields_redacted else None,
        }

        old_compare = {k: v for k, v in rec.items() if k != "ingested_at"}
        new_compare = {k: v for k, v in new_record.items() if k != "ingested_at"}
        if old_compare == new_compare:
            report["unchanged"] += 1
            continue

        new_record["ingested_at"] = ingested_at
        if not dry_run:
            with open(path, "w", encoding="utf-8") as fh:
                json.dump(new_record, fh, indent=2, ensure_ascii=False)
                fh.write("\n")
        report["changed"] += 1
        report["changed_paths"].append(path)

    text = json.dumps(report, indent=2)
    print(text)
    if out_path:
        with open(out_path, "w", encoding="utf-8") as fh:
            fh.write(text)
    return 0


def main() -> int:
    dry_run = "--dry-run" in sys.argv
    out_path = None
    if "--out" in sys.argv:
        out_path = sys.argv[sys.argv.index("--out") + 1]

    root = corpus_root()
    if not os.path.isdir(root):
        print(f"PCGEN_CORPUS_ROOT ({root}) is not a directory", file=sys.stderr)
        return 1

    if "--remediate" in sys.argv:
        # `decisions.md §17` structural gap-close: the ordinary writer below
        # is `no_record`-ledger-gated and can never re-touch a record it
        # already shipped. `--remediate` never needs (and never consults)
        # `no_record` status -- see `remediate` above.
        book_filter = None
        if "--book" in sys.argv:
            book_filter = sys.argv[sys.argv.index("--book") + 1]
        return remediate(root, book_filter, dry_run, out_path)

    ledger_path = os.path.join(REPO_ROOT, "docs/work-inventory.json")
    if "--ledger" in sys.argv:
        ledger_path = sys.argv[sys.argv.index("--ledger") + 1]
    else:
        ledger_path = None

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
