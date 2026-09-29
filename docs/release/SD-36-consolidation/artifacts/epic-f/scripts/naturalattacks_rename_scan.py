#!/usr/bin/env python3
"""Verify and enumerate the NATURALATTACKS suffix-fix rename class (SD-36 Epic F1, stage 5).

The converter's NATURALATTACKS match arm used to suffix each emitted `Line` with a
per-occurrence-local index (`format!("natural{i}")`, reset to 0 every time the token
occurs), instead of the record-global running line count every OTHER multi-emit arm uses.
That local index made two shapes collide under `data/sheet_rules`' old (buggy) suffixing:

  1. A record whose NATURALATTACKS token occurs more than once (e.g. two draconic heads,
     each a separate occurrence) minted the SAME id (`...#natural0`) for each occurrence's
     first attack -- a literal in-array JSON duplicate id, silently shadowed at LOAD time
     by `SheetRulePackage::insert_rule`'s BTreeMap overwrite (rule-gap-receipt.md's 306
     groups / 369 shadowed entries).
  2. EVERY record whose accumulator already held lines before the NATURALATTACKS arm ran
     (i.e. almost every record with any natural attack at all) got a suffix starting from
     0 instead of from the true running count -- so the fix renumbers far more ids than
     just the 306 colliding groups; it renumbers the WHOLE `#naturalN` family per record.

This script proves the fix is a PURE RENAME: for every rule file that differs only in its
`#natural<N>` id suffixes, the MULTISET of natural-attack rule content (every field except
`id`) is identical between the old (tracked) and new (freshly converted) arrays -- nothing
added, nothing dropped, nothing changed except which number a given attack's id carries.
Any file where that multiset differs is a genuine, unexplained content change and must
STOP the run, not be folded into this class.

Usage:
    python3 naturalattacks_rename_scan.py <baseline_dir> <fresh_dir> <out_json>

Exits 0 and writes <out_json> (the pinned {"old_ids": [...], "new_ids": [...], "_count": N,
"files_examined": M} used by structural_diff.py's enumerated-class allowance) only when
EVERY touched file passes the pure-rename check. Exits 1 and prints the offending file(s)
otherwise.
"""
from __future__ import annotations

import json
import os
import sys
from collections import Counter


def load_tree(root: str) -> dict[str, list]:
    out: dict[str, list] = {}
    for dirpath, _dn, filenames in os.walk(root):
        for fn in filenames:
            if not fn.endswith(".json"):
                continue
            full = os.path.join(dirpath, fn)
            rel = os.path.relpath(full, root).replace(os.sep, "/")
            if rel.startswith("_vars/") or rel.startswith("_defects/") or rel in ("_report.json",):
                continue
            try:
                with open(full, "r", encoding="utf-8") as fh:
                    data = json.load(fh)
            except (OSError, json.JSONDecodeError):
                continue
            if isinstance(data, list):
                out[rel] = data
    return out


def content_key(rule: dict) -> str:
    """Every field except `id`, stably serialized -- the content a natural-attack line
    carries, independent of which numeric suffix its id was assigned."""
    d = {k: v for k, v in rule.items() if k != "id"}
    return json.dumps(d, sort_keys=True)


def is_natural_id(rid: str) -> bool:
    if "#natural" not in rid:
        return False
    suffix = rid.rsplit("#natural", 1)[1]
    return suffix.isdigit()


def main() -> int:
    baseline_dir, fresh_dir, out_path = sys.argv[1], sys.argv[2], sys.argv[3]
    base = load_tree(baseline_dir)
    fresh = load_tree(fresh_dir)

    old_ids: list[str] = []
    new_ids: list[str] = []
    content_shift_ids: list[str] = []
    bad_files: list[tuple[str, str]] = []
    files_examined = 0

    for path in sorted(set(base) | set(fresh)):
        old_list = base.get(path, [])
        new_list = fresh.get(path, [])
        old_nat = [r for r in old_list if isinstance(r, dict) and is_natural_id(r.get("id", ""))]
        new_nat = [r for r in new_list if isinstance(r, dict) and is_natural_id(r.get("id", ""))]
        if not old_nat and not new_nat:
            continue
        old_id_set = {r["id"] for r in old_nat}
        new_id_set = {r["id"] for r in new_nat}
        old_by_id = {r["id"]: content_key(r) for r in old_nat}
        new_by_id = {r["id"]: content_key(r) for r in new_nat}
        shared_ids = old_id_set & new_id_set
        shifted_here = {rid for rid in shared_ids if old_by_id[rid] != new_by_id[rid]}
        if old_id_set == new_id_set and not shifted_here:
            # ids identical on both sides for this file, content identical too -- not part
            # of the rename class, normal diff logic handles it (nothing to explain).
            continue
        files_examined += 1
        old_content = Counter(content_key(r) for r in old_nat)
        new_content = Counter(content_key(r) for r in new_nat)
        if old_content != new_content:
            bad_files.append((path, f"content multiset differs: old={dict(old_content)} new={dict(new_content)}"))
            continue
        old_ids.extend(sorted(old_id_set - new_id_set))
        new_ids.extend(sorted(new_id_set - old_id_set))
        content_shift_ids.extend(sorted(shifted_here))

    if bad_files:
        print(f"FAIL: {len(bad_files)} file(s) are NOT a pure NATURALATTACKS rename:", file=sys.stderr)
        for path, reason in bad_files[:20]:
            print(f"  {path}: {reason}", file=sys.stderr)
        return 1

    old_ids = sorted(set(old_ids))
    new_ids = sorted(set(new_ids))
    content_shift_ids = sorted(set(content_shift_ids))
    payload = {
        "_purpose": "SD-36 Epic F1 stage 5: the NATURALATTACKS suffix fix's pure-rename id set. "
                    "Every id in old_ids is removed by the fix; every id in new_ids replaces it "
                    "1:1 in content (verified: per-file multiset of non-id fields is identical). "
                    "content_shift_ids are ids the OLD and NEW numbering both happen to assign "
                    "(same id, present on both sides) but to DIFFERENT attack content, because "
                    "the whole record's numbering shifted -- also content-preserved at the file "
                    "level, just not at that individual id. structural_diff.py excludes old_ids/"
                    "new_ids from removed_rule_ids/added_rule_ids gating, and excludes "
                    "content_shift_ids' label/value deltas from unexpected_field_deltas gating, "
                    "as one named, enumerated class -- never a blanket allowance.",
        "_command": "python3 naturalattacks_rename_scan.py <baseline_dir> <fresh_dir> <out_json>",
        "_files_examined": files_examined,
        "old_ids": old_ids,
        "new_ids": new_ids,
        "content_shift_ids": content_shift_ids,
        "_old_count": len(old_ids),
        "_new_count": len(new_ids),
        "_content_shift_count": len(content_shift_ids),
    }
    with open(out_path, "w", encoding="utf-8") as fh:
        json.dump(payload, fh, indent=2, sort_keys=True)
        fh.write("\n")
    print(
        f"PASS: {files_examined} files, {len(old_ids)} old ids -> {len(new_ids)} new ids "
        f"(net +{len(new_ids) - len(old_ids)}), {len(content_shift_ids)} same-id content-shift "
        f"ids, written to {out_path}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
