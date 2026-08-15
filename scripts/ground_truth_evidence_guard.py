#!/usr/bin/env python3
"""scripts/ground_truth_evidence_guard.py -- SD31-E2-F1-002's evidence-
provenance guard (OPEN-ISSUES.md row 3).

THE DEFECT THIS EXISTS TO CATCH: a hand-labelled ground-truth sample whose
`token_evidence` field is a canned string that quotes nothing from the
actual corpus record -- exactly what happened to 105 of the 150 units in
`SD31-E2-F1-ground-truth-sample-v1.json` before this cycle. A canned label
is a perfect function of "agrees with the engine," not of the record, and
silently corrupts every later accuracy claim measured against the sample.

THIS IS NOT A CLASSIFIER. It never computes, emits, or compares a
`wiring_class` verdict -- it only checks that each record's *evidence field*
is (a) present, (b) not byte-identical to another record's, and (c)
genuinely traceable to real corpus text. Decision 1(e) item 1 bars
classifier code from this epic before F2; this script stays on the correct
side of that line by construction (`WiringClass` never appears below).

THREE CHECKS, each independently able to fail the whole run:

1. ABSENT -- `token_evidence` missing, empty, or whitespace-only.
2. DUPLICATED -- `token_evidence` byte-identical to another record's (the
   exact shape of the original defect: one canned string shared by 105
   records).
3. UNGROUNDED -- no run of `MIN_QUOTE_LEN` (default 20) consecutive
   characters in `token_evidence` also appears, byte-for-byte, somewhere in
   the record's own corpus text -- its base row (located by a RECURSIVE
   search under the book directory, not the production single-level
   `dir.join(file)` join OPEN-ISSUES.md row 1 tracks, so this guard is not
   blind to the same bug it is meant to help catch) plus every `.MOD` row
   targeting the record's name or `corpus_key`, unioned with any paths the
   record's own `corpus_path_verified` field names (SD31-E2-F1-002 added
   this field precisely so a relabelled record's real read-path travels
   with it as evidence).

Corpus-path resolution is deliberately reimplemented here rather than
imported from `src/rules_core/wiring_class.rs` (there is no Python binding
for it): a guard that shared the production resolver's bug would be blind
to exactly the failure this program has already shipped once.

Usage:
    python3 scripts/ground_truth_evidence_guard.py [SAMPLE_JSON ...]
    (defaults to every docs/release/*/artifacts/*ground-truth-sample*.json
    file in the repo, so a future sample is covered without an edit here)

Exit 0 if every record in every sample file passes all three checks; exit 1
and print every violation otherwise.
"""
from __future__ import annotations

import argparse
import glob
import json
import os
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent

MIN_QUOTE_LEN = 20

DEFAULT_SAMPLE_GLOB = "docs/release/*/artifacts/*ground-truth-sample*.json"

# Reimplemented independently from src/rules_core/wiring_class.rs's
# `book_paths` construction (see module docstring: sharing the production
# resolver would share its bug). Rather than hardcode the same
# BOOKS_RELATIVE/EXTRA_BOOK_DIRS list (which would silently drift from the
# Rust source the day either changes), this walks the whole corpus root
# once and indexes every directory by its basename -- the same "book id is
# the directory basename" contract the production code documents.


def build_book_index(corpus_root: str) -> dict:
    index: dict[str, str] = {}
    if not corpus_root or not os.path.isdir(corpus_root):
        return index
    pathfinder_root = os.path.join(corpus_root, "pathfinder")
    walk_root = pathfinder_root if os.path.isdir(pathfinder_root) else corpus_root
    for dirpath, dirnames, _files in os.walk(walk_root):
        base = os.path.basename(dirpath)
        # First writer wins -- deterministic given os.walk's top-down,
        # sorted-dirnames traversal below; a genuine basename collision
        # across two unrelated corpus subtrees would be a corpus-shape
        # surprise worth a human look, not something this guard should
        # paper over by picking silently.
        dirnames.sort()
        index.setdefault(base, dirpath)
    return index


def find_file_recursive(book_dir: str, filename: str) -> list[str]:
    matches = []
    for dirpath, _dirs, files in os.walk(book_dir):
        if filename in files:
            matches.append(os.path.join(dirpath, filename))
    return matches


def mod_base_name(before_mod: str) -> str:
    base = before_mod
    if base.startswith("CATEGORY=") and "|" in base:
        base = base.split("|", 1)[1]
    if base.startswith("CLASS:"):
        base = base[len("CLASS:") :]
    return base.strip()


def find_mod_rows(book_dir: str, target_names: set) -> list[str]:
    out = []
    for dirpath, _dirs, files in os.walk(book_dir):
        for fn in files:
            if not fn.endswith(".lst"):
                continue
            full = os.path.join(dirpath, fn)
            try:
                with open(full, "r", encoding="utf-8", errors="replace") as fh:
                    text = fh.read()
            except OSError:
                continue
            for raw in text.split("\n"):
                trimmed = raw.rstrip("\r")
                first = trimmed.strip()
                if not first or first.startswith("#"):
                    continue
                head = trimmed.split("\t", 1)[0].strip()
                if ".MOD" not in head:
                    continue
                before = head.split(".MOD", 1)[0]
                base = mod_base_name(before)
                if base in target_names:
                    out.append(trimmed)
    return out


def corpus_text_for_record(rec: dict, book_index: dict) -> str | None:
    """The concatenated raw text of everything this guard can find that
    plausibly belongs to `rec`'s corpus record: any `corpus_path_verified`
    paths the record itself names, plus a recursive base-row search by
    `source_file`/`source_line`, plus every `.MOD` row targeting `name` or
    `corpus_key`. Returns None only when NOTHING at all could be located
    (a genuinely missing book directory) -- an empty-but-located result
    still returns "" so the caller reports UNGROUNDED, not a false CLEAN.
    """
    book = rec.get("book")
    book_dir = book_index.get(book)
    if book_dir is None:
        return None

    chunks: list[str] = []

    verified_paths = rec.get("corpus_path_verified") or []
    for relpath in verified_paths:
        full = os.path.join(book_dir, relpath)
        try:
            with open(full, "r", encoding="utf-8", errors="replace") as fh:
                chunks.append(fh.read())
        except OSError:
            continue

    sf = rec.get("source_file")
    ln = rec.get("source_line")
    if sf and isinstance(ln, int) and ln > 0:
        for m in find_file_recursive(book_dir, sf):
            try:
                with open(m, "r", encoding="utf-8", errors="replace") as fh:
                    lines = fh.read().split("\n")
            except OSError:
                continue
            if 1 <= ln <= len(lines):
                chunks.append(lines[ln - 1])

    targets = set()
    if rec.get("name"):
        targets.add(rec["name"])
    if rec.get("corpus_key"):
        targets.add(rec["corpus_key"])
    if targets:
        chunks.extend(find_mod_rows(book_dir, targets))

    return "\n".join(chunks)


QUOTE_MARKER = "Quoted tokens (verbatim from the row(s) below): "
MIN_MARKER_QUOTE_LEN = 4


def evidence_grounded(evidence: str, corpus_text: str, min_len: int = MIN_QUOTE_LEN) -> bool:
    """Two paths, both requiring a genuine, corpus-specific quote -- never
    just an overlap with generic English:

    1. STRUCTURED path: evidence carrying the `QUOTE_MARKER` (every record
       SD31-E2-F1-002 relabelled) states its quotes explicitly, `" | "`-
       joined, after the marker. ALL of them must appear verbatim in
       `corpus_text` -- this is a stronger check than the sliding window
       below and tolerates PCGen's often-short field tokens (`CR:1`,
       `WT:2`) that a fixed 20-char window would wrongly reject.
    2. FREE-FORM path (every other record): a sliding window requires some
       run of `min_len` consecutive characters to occur verbatim in
       `corpus_text`. Both strings are short (a few KB at most), so the
       naive O(n*m) scan is fine at this scale.
    """
    if not corpus_text:
        return False

    marker_at = evidence.find(QUOTE_MARKER)
    if marker_at != -1:
        quoted = evidence[marker_at + len(QUOTE_MARKER) :]
        segments = [s.strip() for s in quoted.split(" | ") if s.strip()]
        if not segments:
            return False
        return all(
            len(seg) >= MIN_MARKER_QUOTE_LEN and seg in corpus_text for seg in segments
        )

    if len(evidence) < min_len:
        return False
    for i in range(len(evidence) - min_len + 1):
        window = evidence[i : i + min_len]
        if window in corpus_text:
            return True
    return False


def check_sample(units: list, corpus_root: str, label: str) -> list:
    """Returns a list of violation strings; empty means clean."""
    violations = []
    book_index = build_book_index(corpus_root)
    seen_evidence: dict[str, str] = {}

    for rec in units:
        uid = rec.get("id", "<record with no id>")
        ev = rec.get("token_evidence")

        if ev is None or not str(ev).strip():
            violations.append(f"{label}: {uid}: token_evidence is absent/empty")
            continue
        ev = str(ev)

        if ev in seen_evidence:
            violations.append(
                f"{label}: {uid}: token_evidence byte-identical to "
                f"{seen_evidence[ev]}'s -- canned, not record-specific"
            )
        else:
            seen_evidence[ev] = uid

        corpus_text = corpus_text_for_record(rec, book_index)
        if corpus_text is None:
            violations.append(
                f"{label}: {uid}: book '{rec.get('book')}' has no known corpus "
                f"directory under {corpus_root!r} -- cannot verify evidence provenance"
            )
            continue
        if not evidence_grounded(ev, corpus_text):
            violations.append(
                f"{label}: {uid}: token_evidence has no {MIN_QUOTE_LEN}+ character "
                f"run that appears verbatim in its own corpus row/closure"
            )

    return violations


def default_corpus_root() -> str:
    return os.environ.get(
        "PCGEN_CORPUS_ROOT", os.path.expanduser("~/workspace/repos/pcgen/data")
    )


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument(
        "samples",
        nargs="*",
        help="ground-truth-sample JSON file(s); default: every "
        "docs/release/*/artifacts/*ground-truth-sample*.json in the repo",
    )
    parser.add_argument(
        "--corpus-root",
        default=None,
        help="override $PCGEN_CORPUS_ROOT (default: env var, else ~/workspace/repos/pcgen/data)",
    )
    args = parser.parse_args(argv)

    corpus_root = args.corpus_root or default_corpus_root()

    sample_paths = args.samples
    if not sample_paths:
        sample_paths = sorted(glob.glob(str(REPO_ROOT / DEFAULT_SAMPLE_GLOB)))

    if not sample_paths:
        print("ground-truth-evidence-guard: no ground-truth-sample JSON files found "
              f"(glob: {DEFAULT_SAMPLE_GLOB}) -- nothing to check")
        return 0

    all_violations = []
    total_units = 0
    for path in sample_paths:
        try:
            with open(path, "r", encoding="utf-8") as fh:
                units = json.load(fh)
        except (OSError, json.JSONDecodeError) as exc:
            all_violations.append(f"{path}: could not load ({exc})")
            continue
        if not isinstance(units, list):
            all_violations.append(f"{path}: expected a JSON list of records, got {type(units).__name__}")
            continue
        total_units += len(units)
        all_violations.extend(check_sample(units, corpus_root, os.path.relpath(path, REPO_ROOT)))

    print(f"ground-truth-evidence-guard: checked {total_units} unit(s) across {len(sample_paths)} file(s)")
    print(f"  corpus root: {corpus_root}")
    if all_violations:
        print(f"  FAIL: {len(all_violations)} violation(s):")
        for v in all_violations:
            print(f"    {v}")
        return 1

    print("  PASS: every record's token_evidence is present, unique, and grounded in its own corpus row.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
