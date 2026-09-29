#!/usr/bin/env python3
"""Replicates rules_core::corpus_loader::load_sheet_rules's walk over
data/sheet_rules/, but *without* deduping by rule id, so we can measure
exactly where the 71,862 (converter's rules_written) -> 71,493 (loaded
package.rules.len()) gap comes from.

Mirrors the Rust walk precisely:
  - top-level entries under data/sheet_rules/ that are directories
  - "_vars" is skipped here (it's a separate VarTable collection, not rules)
  - any other top-level dir starting with "_" is skipped (book.starts_with('_'))
  - every subdirectory of a book dir is a "kind" dir; ALL kind dirs are
    walked (load_sheet_rules's `keep` closure is `|_, _| true`)
  - find_json_files: recursive walk of the kind dir, skipping any directory
    named "_parity" and any file named "LICENSE.json", collecting *.json
  - files are parsed as `Vec<SheetRule>` (a JSON array); each element has an
    "id" field (the RuleId)
  - insertion order = sorted file path order, then array order within a file
  - package.rules is a BTreeMap<RuleId, SheetRule> -- last insert for a given
    id wins, all earlier ones for that id vanish with zero diagnostic
"""
import json
import os
import sys
from collections import defaultdict

ROOT = sys.argv[1] if len(sys.argv) > 1 else "data/sheet_rules"


def find_json_files(kind_dir):
    out = []
    stack = [kind_dir]
    while stack:
        cur = stack.pop()
        try:
            entries = list(os.scandir(cur))
        except OSError:
            continue
        for e in entries:
            if e.is_dir(follow_symlinks=False):
                if e.name == "_parity":
                    continue
                stack.append(e.path)
            else:
                if e.name == "LICENSE.json":
                    continue
                if e.name.endswith(".json"):
                    out.append(e.path)
    out.sort()
    return out


def main():
    books = sorted(
        e.path for e in os.scandir(ROOT) if e.is_dir(follow_symlinks=False)
    )
    rule_paths = []
    for book_dir in books:
        book = os.path.basename(book_dir)
        if book == "_vars":
            continue
        if book.startswith("_"):
            continue
        kinds = sorted(
            e.path for e in os.scandir(book_dir) if e.is_dir(follow_symlinks=False)
        )
        for kind_dir in kinds:
            rule_paths.extend(find_json_files(kind_dir))

    total_entries = 0
    parse_failures = []
    # id -> list of (file, index_in_file) that wrote it, in insertion order
    id_writes = defaultdict(list)
    # id -> last rule dict written (what the real BTreeMap ends up holding)
    id_last = {}
    file_rule_counts = {}

    for path in rule_paths:
        try:
            with open(path, "r", encoding="utf-8") as fh:
                text = fh.read()
            rules = json.loads(text)
        except Exception as exc:  # noqa: BLE001
            parse_failures.append((path, str(exc)))
            continue
        if not isinstance(rules, list):
            parse_failures.append((path, f"not a JSON array: {type(rules)}"))
            continue
        file_rule_counts[path] = len(rules)
        for idx, rule in enumerate(rules):
            total_entries += 1
            rid = rule.get("id") if isinstance(rule, dict) else None
            if rid is None:
                parse_failures.append((path, f"entry {idx} has no id"))
                continue
            id_writes[rid].append((path, idx))
            id_last[rid] = rule

    unique_ids = len(id_last)
    dup_ids = {rid: writes for rid, writes in id_writes.items() if len(writes) > 1}
    shadowed_entries = sum(len(w) - 1 for w in dup_ids.values())

    print(f"rule_files_found         = {len(rule_paths)}")
    print(f"total_array_entries      = {total_entries}")
    print(f"parse_failures           = {len(parse_failures)}")
    print(f"unique_rule_ids_after_dedup = {unique_ids}")
    print(f"duplicate_id_groups      = {len(dup_ids)}")
    print(f"shadowed_entries (lost)  = {shadowed_entries}")
    print(f"accounted (unique + shadowed) = {unique_ids + shadowed_entries}  (should == total_array_entries)")

    if parse_failures:
        print("\n--- parse failures (first 20) ---")
        for p, msg in parse_failures[:20]:
            print(p, msg)

    # Dump the full duplicate-id detail for downstream analysis.
    with open(sys.argv[2] if len(sys.argv) > 2 else "dup_ids.json", "w") as out:
        detail = []
        for rid, writes in sorted(dup_ids.items()):
            book, kind, slug = (rid.split(":", 2) + ["", ""])[:3]
            detail.append({
                "id": rid,
                "book": book,
                "kind": kind,
                "writes": [{"file": f, "index": i} for f, i in writes],
                "kept_file": writes[-1][0],
                "subject": id_last[rid].get("subject"),
                "label": id_last[rid].get("label"),
                "print": id_last[rid].get("print"),
            })
        json.dump(detail, out, indent=2)

    print(f"\nduplicate id detail written to {sys.argv[2] if len(sys.argv) > 2 else 'dup_ids.json'}")


if __name__ == "__main__":
    main()
