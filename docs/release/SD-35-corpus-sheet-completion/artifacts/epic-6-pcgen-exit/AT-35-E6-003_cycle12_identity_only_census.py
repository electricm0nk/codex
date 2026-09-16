#!/usr/bin/env python3
"""The identity-only census behind AT-35-E6-003 cycle 12's receipt.

Joins every corpus record under the twelve reference-library kind directories to its
converted rule on its own `source.path:line`, then applies the same five content tests
`rules_core::sheet_rule_catalog::catalog_field_summary` applies -- the rule's own fields,
the stat-block prose families, the grant edge read forwards (`granted_by`), the grant edge
read backwards (`granted_from`), and the labelled variable tables -- and reports the records
for which all five say nothing.

Run from the repo root:
    python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003_cycle12_identity_only_census.py
"""
import collections
import glob
import json
import os

KINDS = [
    "ability", "class_generic", "deity", "domain", "feat_generic", "language",
    "monster_generic", "power", "race_generic", "skill", "template", "trait_generic",
]


def load_rules():
    rules = {}
    for path in glob.glob("data/sheet_rules/*/*/*.json"):
        if "/_" in path:
            continue
        try:
            arr = json.load(open(path))
        except (OSError, ValueError):
            continue
        if isinstance(arr, list):
            for rule in arr:
                rules[rule["id"]] = rule
    return rules


def main():
    rules = load_rules()
    by_row = {}
    for rid, rule in rules.items():
        rows = rule.get("provenance", {}).get("closure_rows") or []
        if rows:
            by_row.setdefault(rows[0], []).append(rid)

    # grant edge read backwards: who hands this rule out
    hands_out = set()
    for rule in rules.values():
        for grant in rule.get("granted_by", []):
            by = grant.get("by")
            if isinstance(by, dict) and "Rule" in by:
                hands_out.add(by["Rule"])

    # labelled variable tables
    var_users = set()
    for path in glob.glob("data/sheet_rules/_vars/*.json"):
        table = json.load(open(path))
        if not (table.get("label") or "").strip():
            continue
        for c in table.get("contributions", []):
            var_users.add(c["rule_id"])
        var_users.update(table.get("declared_by", []))

    def says_nothing(rid):
        r = rules[rid]
        if r.get("prose") or r.get("tags") or r.get("target") or r.get("grants"):
            return False
        if r.get("offers") or r.get("also") or r.get("granted_by"):
            return False
        if r.get("applies") != "Always" or r.get("value") != "Text":
            return False
        return rid not in hands_out and rid not in var_users

    total = 0
    bare_rows = {}
    for book in sorted(os.listdir("data/corpus")):
        for kind in KINDS:
            d = os.path.join("data/corpus", book, kind)
            if not os.path.isdir(d):
                continue
            for root, _, files in os.walk(d):
                for name in files:
                    if not name.endswith(".json"):
                        continue
                    path = os.path.join(root, name)
                    try:
                        doc = json.load(open(path))
                    except (OSError, ValueError):
                        continue
                    if "key" not in (doc.get("data") or {}):
                        continue
                    total += 1
                    src = doc.get("source") or {}
                    row = "%s:%s" % (src.get("path"), src.get("line"))
                    ids = by_row.get(row)
                    if ids and all(says_nothing(i) for i in ids):
                        bare_rows[path] = (doc, ids)

    hidden = collections.Counter()
    for doc, _ in bare_rows.values():
        hidden[any(
            t.get("key") == "VISIBLE" and t.get("value", "").strip().upper() == "NO"
            for t in doc["data"].get("raw_tokens", [])
        )] += 1

    bare_rule_ids = {i for _, ids in bare_rows.values() for i in ids}
    print("reference_library_records=%d" % total)
    print("identity_only_records=%d" % len(bare_rows))
    print("identity_only_rules=%d" % len(bare_rule_ids))
    print("of_which_source_marks_not_visible=%d" % hidden[True])


if __name__ == "__main__":
    main()
