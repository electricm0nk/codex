#!/usr/bin/env python3
"""Generic, kind-parameterized ingest of the small-tail `no_record`
populations into `data/corpus/<book>/<kind>_generic/*.json` -- a directory
SIBLING to `<kind>/`, deliberately not inside it.

**Name-PI handling (`decisions.md §24`).** A unit whose name is declared
`NAMEISPI:YES` or whose name/key hits the blacklist is NOT skipped -- it is
ingested under a Codex-generated neutral name derived ONLY from
`(kind, book, source_file, source_line)`, exactly the scheme
`scripts/ingest_ability.py` (the `§24` reference implementation for
`ability`) already proved: `scripts/codex_neutral_name.py::neutral_name`/
`neutral_key`/`divergence_entry`, plus the identical `scrub_name_pi_tokens`
sweep for the record's OTHER fields (`§24b`-2: a `KEY:` token can restate
the row's own PI name even when the NAME column doesn't). This module
reuses that reference implementation's functions verbatim rather than
re-deriving them, per `decisions.md §17`'s "search for an existing path
before building one".

SD-32 `decisions.md §20`: `no_record` means an object was never ingested, so
its shape cannot be measured. `decisions.md §17`: enumeration and shape
analysis are GENERIC passes, not per-object work -- do not build a fifth
per-kind, per-book transcriber when the fourth (`scripts/ingest_ability.py`,
then `scripts/ingest_race_trait_generic.py`) already proved the shape safe.
This module is that same shape, parameterized by `--kind` instead of forked
per kind, for `race` (59), `monster` (28) and `class` (21) -- SD-32's
"small tails" (`docs/release/SD-32-compute-library-and-cause-closure/
decisions.md §20`'s per-kind table).

**Why a sibling directory, not `<kind>/` itself.** `race/`, `monster/` and
`class/` already hold semantically-resolved, engine-reachable records built
by the kind's own curated Rust ingesters (`ingest_races.rs`,
`transcribe_monster_tables.py` + `gen_book_cache.rs::gen_monster_book`,
`untabled_base_class_chassis.rs`/`ingest_pu_classes.rs`). At least one
existing consumer (`src/bin/enrich_monster_raw_tokens.rs`) globs
`data/corpus/*/monster/*.json` directly and would misinterpret this script's
flatter, unenriched shape if it landed there. `scripts/shape_ledger.py::
build_corpus_index` walks `data/corpus/<book>/**/*.json` with no
subdirectory-name filter, so a sibling `<kind>_generic/` directory is exactly
as measurable for Gate-1 purposes while touching zero existing consumers of
the curated shape -- the identical "Gate-1 measurability and player-
reachability are different claims" ruling `ingest_race_trait_generic.py`
already established for `race_trait`.

**Nothing is computed.** Every emitted `raw_tokens` entry is a verbatim
substring of the cited row (skip the identity column, split each remaining
tab field on its first `:`) -- identical shape to `ingest_ability.py::
row_tokens` / `ingest_race_trait_generic.py::row_tokens`, so
`corpus_literal_sweep` verifies it the same way.

**Product Identity, description-only** (`decisions.md §15/§19`): a
`DESCISPI:YES` declaration or a description-blacklist hit redacts only the
`DESC` field -- the record still ships under its real name. **Product
Identity, name-level** (`decisions.md §24`, see above): the record still
ships, under a Codex-generated neutral name, named in the run's own JSON
report under `renamed_records` (coordinates and reason only, never the
original name -- `§24b`-4).

**`BOOK_CORPUS_DIR_ALIASES`** (`scripts/shape_ledger.py`'s reader-side
alias) is honoured on the WRITE side too, via the identical
`CORPUS_WRITE_DIR_ALIASES` table `ingest_ability.py`/
`ingest_race_trait_generic.py` already carry -- `bestiary` writes under the
physical `beastiary` directory.

Run: `python3 scripts/ingest_generic_kind.py --kind race --ledger <ledger.json> [--dry-run] [--out <report.json>]`
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
from codex_neutral_name import (  # noqa: E402
    divergence_entry,
    neutral_key,
    neutral_name,
)
from pi_scrub import (  # noqa: E402
    PI_MARKER_REDACTED,
    REDACTED_PI_MARKER,
    blacklist_term_hit_including_concatenated,
    scrub_name_pi_tokens,
)

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
INVENTORY_PATH = os.path.join(REPO_ROOT, "docs/work-inventory.json")
CORE_ESSENTIALS_BASENAME = "core_essentials"

# Mirrors `ingest_ability.py::CORPUS_WRITE_DIR_ALIASES` /
# `ingest_race_trait_generic.py::CORPUS_WRITE_DIR_ALIASES` /
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
    Mirrors `ingest_ability.py::row_tokens` / `ingest_race_trait_generic.py::
    row_tokens` / `cache_gen::class_feature::row_tokens` exactly."""
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


def load_no_record_ids(ledger_path: str, kind: str) -> set[str]:
    """The set of unit ids `scripts/shape_ledger.py` reports `join_status ==
    "no_record"` for the given kind, read from a ledger already generated
    against `docs/work-inventory.json`. The live join, not the possibly-stale
    `status` field, is the ground truth for "needs a corpus record"."""
    with open(ledger_path, encoding="utf-8") as fh:
        ledger = json.load(fh)
    rows = ledger["rows"] if isinstance(ledger, dict) and "rows" in ledger else ledger
    return {
        r["id"]
        for r in rows
        if r.get("kind") == kind and r.get("join_status") == "no_record"
    }


def load_units(no_record_ids: set[str], kind: str) -> list[dict]:
    with open(INVENTORY_PATH, encoding="utf-8") as fh:
        doc = json.load(fh)
    units = doc["units"] if isinstance(doc, dict) and "units" in doc else doc
    return [u for u in units if u.get("kind") == kind and u.get("id") in no_record_ids]


def main() -> int:
    dry_run = "--dry-run" in sys.argv
    out_path = None
    if "--out" in sys.argv:
        out_path = sys.argv[sys.argv.index("--out") + 1]
    if "--kind" not in sys.argv:
        print("--kind <race|monster|class|...> is required", file=sys.stderr)
        return 1
    kind = sys.argv[sys.argv.index("--kind") + 1]

    ledger_path = None
    if "--ledger" in sys.argv:
        ledger_path = sys.argv[sys.argv.index("--ledger") + 1]
    if ledger_path is None:
        print("--ledger <shape_ledger_output.json> is required", file=sys.stderr)
        return 1

    root = corpus_root()
    if not os.path.isdir(root):
        print(f"PCGEN_CORPUS_ROOT ({root}) is not a directory", file=sys.stderr)
        return 1

    no_record_ids = load_no_record_ids(ledger_path, kind)
    units = load_units(no_record_ids, kind)
    dir_index = build_dir_index(root)

    report = {
        "kind": kind,
        "population": len(units),
        "written": 0,
        "name_pi_renamed": 0,
        "unresolved": [],
        "written_by_book": defaultdict(int),
        # `decisions.md §24b`-4: divergence entries carry coordinates and the
        # reason, never the original PI string -- see
        # `scripts/codex_neutral_name.py::divergence_entry`.
        "renamed_records": [],
    }

    file_cache: dict[tuple[str, str], list[str]] = {}
    used_by_book: dict[str, set[str]] = defaultdict(set)
    seeded_books: set[str] = set()
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
        name_is_pi = name_declared or bool(name_hit)

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

        # Seed this book's `used` slug set from whatever `<kind>_generic/`
        # already holds on disk, not only from slugs assigned earlier in
        # THIS run. Without this, two distinct units that happen to
        # slugify to the same string (found live this cycle: a
        # `class_feature` and a `race_trait` unit sharing the identical
        # `corpus_key` text at different source lines) silently overwrite
        # each other across separate script invocations instead of
        # colliding into a `_2` suffix the way same-run collisions already
        # do.
        write_dir_book_for_seed = CORPUS_WRITE_DIR_ALIASES.get(book, book)
        if book not in seeded_books:
            existing_dir = os.path.join(REPO_ROOT, "data/corpus", write_dir_book_for_seed, f"{kind}_generic")
            if os.path.isdir(existing_dir):
                for fname in os.listdir(existing_dir):
                    if fname.endswith(".json"):
                        used_by_book[book].add(fname[: -len(".json")])
            seeded_books.add(book)
        used = used_by_book[book]

        # `decisions.md §17` gap-close (found live, `pi-key-rawtokens-screen`
        # follow-up cycle, 2026-08-23, applied here to `<kind>_generic`): a
        # record whose bare `name`/`description` are clean can still carry a
        # blacklisted term inside a DIFFERENT raw-token value. Before this
        # fix, `scrub_name_pi_tokens` below only ran when `name_is_pi` -- a
        # clean-name record's `raw_tokens` shipped entirely unscanned. Scans
        # EVERY token value with `blacklist_term_hit_including_concatenated`
        # (the shared `pi_scrub` module's own blacklist-only check, the same
        # word-bounded 60-term scan `scrub_name_pi_tokens` uses plus its
        # concatenated-identifier coverage), independent of whether the name
        # triggered a rename below.
        blacklist_extra_redacted = False
        _scrubbed_for_blacklist: list[dict[str, str]] = []
        for _t in tokens:
            _value = _t["value"]
            if pi_redacted and _t["key"] == "DESC" and _value == REDACTED_PI_MARKER:
                _scrubbed_for_blacklist.append(dict(_t))
                continue
            if _value and blacklist_term_hit_including_concatenated(_value):
                _scrubbed_for_blacklist.append({"key": _t["key"], "value": REDACTED_PI_MARKER})
                blacklist_extra_redacted = True
            else:
                _scrubbed_for_blacklist.append(dict(_t))
        tokens = _scrubbed_for_blacklist

        fields_redacted: list[str] = []
        if pi_redacted:
            fields_redacted.append("description")
        if blacklist_extra_redacted:
            fields_redacted.append("raw_tokens")

        if name_is_pi:
            # `decisions.md §24` -- ingest under a Codex-generated neutral
            # name derived ONLY from (kind, book, source_file, source_line).
            # Reuses `scripts/ingest_ability.py`'s own §24 reference
            # implementation via `scripts/codex_neutral_name.py`.
            codex_name = neutral_name(kind, book, source_file, line)
            codex_key = neutral_key(kind, book, source_file, line)
            scrubbed_tokens, extra_redacted = scrub_name_pi_tokens(tokens, name, key)
            record_name = codex_name
            record_key = codex_key
            record_tokens = scrubbed_tokens
            slug = slugify(codex_name, used)
            report["name_pi_renamed"] += 1
            report["renamed_records"].append(
                divergence_entry(kind, book, source_file, line, reason="name_pi_blocked")
            )
            codex_generated_name = True
            rename_info = {
                "reason": "name_pi_blocked",
                "coordinate": f"{book}:{os.path.basename(source_file)}:{line}",
            }
            fields_redacted.append("name")
            if extra_redacted and "raw_tokens" not in fields_redacted:
                fields_redacted.append("raw_tokens")
        else:
            record_name = name
            record_key = key
            record_tokens = tokens
            slug = slugify(key, used)
            codex_generated_name = False
            rename_info = None

        license_value = "PI-REDACTED" if fields_redacted else "OGL"

        record = {
            "population": "in_scope",
            "completeness": "full" if stored_description else "chassis_only",
            "ingested_at": ingested_at,
            "data": {
                "key": record_key,
                "name": record_name,
                "description": stored_description,
                "raw_tokens": record_tokens,
            },
            "source": {
                "kind": "lst_token",
                "path": rel_path,
                "sha256": sha256,
                "line": line,
                "record_key": record_key,
            },
            "wiring_class": wiring_class,
            "wiring_class_signals": wiring_signals,
            "license": license_value,
            "pi_field": ",".join(fields_redacted) if fields_redacted else None,
            "pi_marker": PI_MARKER_REDACTED if fields_redacted else None,
            # `decisions.md §24b`-3: "A field marks it as carrying a
            # Codex-generated name, so no reader or player mistakes it for
            # the printed name." `§24b`-4: the divergence record stops at
            # the coordinate -- never the original string.
            "codex_generated_name": codex_generated_name,
            "rename": rename_info,
        }

        write_dir_book = CORPUS_WRITE_DIR_ALIASES.get(book, book)
        out_dir = os.path.join(REPO_ROOT, "data/corpus", write_dir_book, f"{kind}_generic")
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
