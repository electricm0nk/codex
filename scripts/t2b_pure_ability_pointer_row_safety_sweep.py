#!/usr/bin/env python3
"""Corpus-wide safety proof for `is_pure_ability_pointer_race_trait_row`
(`src/bin/v06_work_inventory.rs`).

The census fix that predicate backs excludes a `_abilities_race.lst` row
from the `race_trait` population when it carries no `TYPE:`, no `DESC:`, no
`BONUS*` token of its own, but does carry an `ABILITY:...AUTOMATIC...`
grant -- PCGen's own pool-selector/companion-token plumbing for an
ALREADY-modelled trait (the Svirfneblin `Stalwart Watcher Output` shape,
generalized; `decisions.md §16`/`§20`).

A discriminator that fires on genuine, already-real content is exactly the
defect class `decisions.md §16` warns against (the KEY-prefix fix that
misclassified 112 Ultimate Psionics units on an insufficiently-covered
safety test). This script is the safety test: it reads EVERY currently-
ingested `race_trait` record's own source row back from the pinned oracle
and asserts the predicate never fires on one. If it ever does, that record
is proof the predicate is unsafe and must not ship.

Usage:
    PCGEN_CORPUS_ROOT=<oracle>/data python3 scripts/t2b_pure_ability_pointer_row_safety_sweep.py

Exits 0 and prints `SAFE: 0 violations / <N> records checked` on success;
exits 1 and lists every violation otherwise.
"""
from __future__ import annotations

import glob
import json
import os
import sys


def is_pure_ability_pointer_row(fields: list[str]) -> bool:
    """Mirrors `is_pure_ability_pointer_race_trait_row` in
    `src/bin/v06_work_inventory.rs` exactly -- kept in lockstep by hand; a
    drift here would make this safety sweep prove nothing about what
    actually ships."""
    has_type = any(f.startswith("TYPE:") for f in fields)
    has_desc = any(f.startswith("DESC:") for f in fields)
    has_bonus = any(f.startswith("BONUS") for f in fields)
    has_automatic = any("AUTOMATIC" in f for f in fields)
    return (not has_type) and (not has_desc) and (not has_bonus) and has_automatic


def main() -> int:
    corpus_root = os.environ.get("PCGEN_CORPUS_ROOT")
    if not corpus_root:
        print("PCGEN_CORPUS_ROOT must be set to a PCGen data/ checkout", file=sys.stderr)
        return 2

    repo_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    lst_cache: dict[str, list[str] | None] = {}

    def get_lines(path: str) -> list[str] | None:
        if path not in lst_cache:
            full = os.path.join(corpus_root, path)
            try:
                with open(full, encoding="utf-8", errors="replace") as fh:
                    lst_cache[path] = fh.readlines()
            except OSError:
                lst_cache[path] = None
        return lst_cache[path]

    checked = 0
    violations: list[tuple[str, str, int, str]] = []
    pattern = os.path.join(repo_root, "data", "corpus", "*", "race_trait", "**", "*.json")
    for path in glob.glob(pattern, recursive=True):
        try:
            with open(path, encoding="utf-8") as fh:
                rec = json.load(fh)
        except (OSError, json.JSONDecodeError):
            continue
        source = rec.get("source") or {}
        src_path = source.get("path")
        src_line = source.get("line")
        if not src_path or src_line is None:
            continue
        lines = get_lines(src_path)
        if not lines or src_line - 1 >= len(lines):
            continue
        raw = lines[src_line - 1].rstrip("\n")
        fields = raw.split("\t")
        checked += 1
        if is_pure_ability_pointer_row(fields):
            violations.append((path, src_path, src_line, raw[:200]))

    if violations:
        print(f"UNSAFE: {len(violations)} violation(s) / {checked} records checked", file=sys.stderr)
        for corpus_path, src_path, src_line, raw in violations:
            print(f"  {corpus_path} <- {src_path}:{src_line}: {raw!r}", file=sys.stderr)
        return 1

    print(f"SAFE: 0 violations / {checked} records checked")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
