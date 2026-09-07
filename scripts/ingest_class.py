#!/usr/bin/env python3
"""SD-32 `decisions.md §20` — generic ingest for `Kind::Class`'s
enumerated-but-engine-does-not-hold units (`docs/work-inventory.json`, 157 units at
`857eb85d0`, all `join_status: no_record`).

**Why `class` is not `ingest_simple_filename_kinds.py`'s sixth kind.** That
script's `TARGET_KINDS` covers five kinds whose corpus rows share one shape:
a flat single-line record whose leading tab-delimited field IS the bare
record identity (`corpus_key`). `class` units instead cite a `CLASS:<Name>`
line — the identity column carries a `CLASS:` prefix the other five kinds
never do, and (`cr_classes.lst:429/431/433` for Loremaster, confirmed
against the pinned oracle) a single class is frequently declared across
*several* physical lines sharing the same `CLASS:<Name>` identity (HD/TYPE
on one line, `PREABILITY` on another, `STARTSKILLPTS`/`CSKILL` on a third —
a `.MOD`-continuation shape, `decisions.md §12b`). `v06_work_inventory.rs`
already resolves this to exactly one `class` unit per class name citing one
representative line; this script transcribes only that cited line, the same
"one row, verbatim, no computed merge" discipline `gen_core_rulebook_cache.rs`'s
own `class_source()` already applies to the 11 base classes — never
reconstructing a class's other lines into one synthetic record.

**Generic, not per-book** (`decisions.md §17`): a unit's own
`(book, source_file, source_line)` citation is resolved against the pinned
corpus by directory-name + basename search (via `census_independent.py`'s
own `discover_book_dirs`/`classify_scope`, reused not re-derived, and a
`pcc_includes` fallback for shared-library files). No book is named in this
script's own source.

**Nothing is computed.** `raw_tokens` is the cited row's tab-delimited
`KEY:VALUE` fields, verbatim (skip the identity column, split each
remaining field on its first `:`). `corpus_literal_sweep` independently
re-derives the same tokens from the same citation to confirm byte-for-byte.

**`source.path` carries the leading `pathfinder/` segment** — SD-32 wave 1's
own recorded defect (a sibling generator computed `source.path` relative to
`$PCGEN_CORPUS_ROOT/pathfinder` instead of `$PCGEN_CORPUS_ROOT`, dropping
that segment corpus-wide and breaking `corpus_literal_sweep`). This script's
`out_path` for `sha256_of`/citation resolution is computed relative to
`args.pcgen_root` itself, so every emitted `source.path` starts
`pathfinder/...`, matching every Rust generator's convention
(`gen_core_rulebook_cache.rs`'s own `CorpusRecord.source.path` values).

**Product Identity** (`decisions.md §15/§19`, standing rule, in force for
every shape): the row's own `NAMEISPI:YES`/`DESCISPI:YES` declaration OR a
word-boundary, OCR-normalized blacklist hit (`sd32_t9_pi_review_feat_
equipment.py`'s `normalized_term_hit` — imported, not re-typed) against the
class's bare name. A name hit skips the whole record (never transcribed,
name cannot be redacted); PCGen class rows carry no free-text `DESC:`/
`BENEFIT:` field in this corpus (`class_tables.rs`'s own hand-authored
tables confirm — class fiction lives in the corebook, not the LST), so no
description-redaction path applies here.

Usage:
    python3 scripts/ingest_class.py \\
        --inventory docs/work-inventory.json \\
        --pcgen-root "$PCGEN_CORPUS_ROOT" \\
        --out-root data/corpus \\
        [--dry-run]
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import sys
from collections import Counter, defaultdict
from datetime import datetime, timezone

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import census_independent as ci  # noqa: E402
from sd32_t9_pi_review_feat_equipment import (  # noqa: E402
    normalized_term_hit,
)
from shape_ledger import BOOK_CORPUS_DIR_ALIASES  # noqa: E402

REDACTED_PI_MARKER = "[redacted PI]"
TARGET_KIND = "class"


def slugify(name: str) -> str:
    s = re.sub(r"[^a-z0-9]+", "_", name.strip().lower())
    return s.strip("_") or "unnamed"


def parse_row(raw_line: str) -> list[dict]:
    """Tab-delimited `KEY:VALUE` fields -> raw_tokens, skipping the leading
    `CLASS:<Name>` identity column itself (that identity becomes `data.key`/
    `data.name`, not a raw_tokens entry — mirrors `ingest_simple_filename_
    kinds.py::parse_row`'s treatment of its own leading field, generalized
    to skip exactly one field rather than assuming the whole field is the
    identity)."""
    fields = [f.strip() for f in raw_line.split("\t") if f.strip()]
    tokens = []
    for field in fields[1:]:
        if ":" not in field:
            continue
        key, _, value = field.partition(":")
        key = key.strip()
        if not key:
            continue
        tokens.append({"key": key, "value": value})
    return tokens


def declared_pi(raw_tokens: list[dict]) -> tuple[bool, bool]:
    name_pi = any(
        t["key"].upper() == "NAMEISPI" and t["value"].strip().upper() == "YES"
        for t in raw_tokens
    )
    desc_pi = any(
        t["key"].upper() == "DESCISPI" and t["value"].strip().upper() == "YES"
        for t in raw_tokens
    )
    return name_pi, desc_pi


def build_book_index(pcgen_root: str, inv: dict) -> tuple[dict[str, str], dict[str, list[str]]]:
    book_dirs = ci.discover_book_dirs(pcgen_root)
    scope = ci.classify_scope(book_dirs, inv)
    pathfinder_root = os.path.join(pcgen_root, "pathfinder")
    paths = {bd.book_id: os.path.join(pathfinder_root, bd.rel_path) for bd in scope.in_scope}
    includes = {b["id"]: b.get("pcc_includes") or [] for b in inv["books"]}
    return paths, includes


def find_file(book: str, book_paths: dict, includes: dict, basename: str, cache: dict) -> str | None:
    def index_for(bdir: str) -> dict:
        if bdir not in cache:
            idx = {}
            for dirpath, _dn, filenames in os.walk(bdir):
                for fn in filenames:
                    idx.setdefault(fn, os.path.join(dirpath, fn))
            cache[bdir] = idx
        return cache[bdir]

    own_dir = book_paths.get(book)
    if own_dir is not None:
        hit = index_for(own_dir).get(basename)
        if hit is not None:
            return hit
    for dep in includes.get(book, []):
        dep_dir = book_paths.get(dep)
        if dep_dir is None:
            continue
        hit = index_for(dep_dir).get(basename)
        if hit is not None:
            return hit
    return None


def sha256_of(path: str) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as fh:
        h.update(fh.read())
    return h.hexdigest()


def read_line(path: str, line_no: int) -> str | None:
    with open(path, "r", encoding="utf-8", errors="replace") as fh:
        for i, raw in enumerate(fh, start=1):
            if i == line_no:
                return raw.rstrip("\n").replace("­", "-")
    return None


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--inventory", default="docs/work-inventory.json")
    ap.add_argument("--pcgen-root", required=True)
    ap.add_argument("--out-root", default="data/corpus")
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args(argv)

    with open(args.inventory, "r", encoding="utf-8") as fh:
        inv = json.load(fh)

    book_paths, book_includes = build_book_index(args.pcgen_root, inv)
    file_cache: dict = {}
    slug_seen: dict[tuple[str, str], set] = defaultdict(set)

    stats = Counter()
    citation_mismatches = []
    written = []
    skipped_existing = []
    pi_skipped = []

    for unit in inv["units"]:
        if unit.get("kind") != TARGET_KIND:
            continue
        book = unit["book"]
        basename = unit["source_file"]
        line_no = unit["source_line"]
        corpus_key = unit["corpus_key"]
        name = unit["name"]
        stats["seen"] += 1

        # `decisions.md §20`'s wave-1 lesson: `shape_ledger.py`'s READER
        # honours `BOOK_CORPUS_DIR_ALIASES` (currently only `bestiary` ->
        # `beastiary`) when a `--kind`-restricted call passes `books`, so a
        # record physically written under the unaliased directory name is
        # invisible to the join and stays `no_record` forever. Every writer
        # must honour the same alias for the directory it writes into.
        out_dir = os.path.join(args.out_root, BOOK_CORPUS_DIR_ALIASES.get(book, book), TARGET_KIND)
        slug = slugify(corpus_key)
        out_path = os.path.join(out_dir, f"{slug}.json")
        if os.path.exists(out_path):
            skipped_existing.append(out_path)
            stats["skipped_existing"] += 1
            continue

        if book not in book_paths:
            stats["no_book_dir"] += 1
            continue
        file_path = find_file(book, book_paths, book_includes, basename, file_cache)
        if file_path is None:
            stats["no_file"] += 1
            continue
        raw_line = read_line(file_path, line_no)
        if raw_line is None:
            stats["no_line"] += 1
            continue

        identity_field = raw_line.split("\t", 1)[0].strip()
        # `CLASS:<Name>` -- strip the tag, the same way every other reader
        # in this codebase treats an identity column's own leading tag
        # (`ingest_simple_filename_kinds.py`'s equivalent check assumes the
        # whole leading field IS the bare name; class rows carry one tag
        # more).
        _, _, identity = identity_field.partition(":")
        identity = identity.strip()
        if identity != corpus_key:
            citation_mismatches.append(
                {"book": book, "file": basename, "line": line_no, "expected": corpus_key, "found": identity}
            )
            stats["citation_mismatch"] += 1
            continue

        raw_tokens = parse_row(raw_line)
        name_pi, _desc_pi = declared_pi(raw_tokens)
        term_hit_name = normalized_term_hit(name)
        if name_pi or term_hit_name:
            pi_skipped.append({"book": book, "unit_id": unit["id"], "name": name, "reason": "name_pi" if name_pi else f"blacklist:{term_hit_name}"})
            stats["pi_skipped"] += 1
            continue

        rec = {
            "population": "in_scope",
            "completeness": "chassis_only",
            "ingested_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
            "data": {
                "key": corpus_key,
                "name": name,
                "raw_tokens": raw_tokens,
            },
            "source": {
                "kind": "lst_token",
                "path": os.path.relpath(file_path, args.pcgen_root),
                "sha256": sha256_of(file_path),
                "line": line_no,
                "record_key": corpus_key,
            },
            "wiring_class": "display",
            "wiring_class_signals": ["display:sd32_class_ingest"],
            "license": "OGL",
            "pi_field": None,
            "pi_marker": None,
        }

        seen = slug_seen[(book, TARGET_KIND)]
        if slug in seen:
            slug = f"{slug}_{line_no}"
            out_path = os.path.join(out_dir, f"{slug}.json")
        seen.add(slug)

        if not args.dry_run:
            os.makedirs(out_dir, exist_ok=True)
            with open(out_path, "w", encoding="utf-8") as fh:
                json.dump(rec, fh, indent=2, ensure_ascii=False)
                fh.write("\n")
        written.append(out_path)
        stats["written"] += 1

    summary = {
        "stats": dict(stats),
        "citation_mismatches": citation_mismatches,
        "pi_skipped": pi_skipped,
        "written_count": len(written),
        "skipped_existing_count": len(skipped_existing),
    }
    print(json.dumps(summary, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
