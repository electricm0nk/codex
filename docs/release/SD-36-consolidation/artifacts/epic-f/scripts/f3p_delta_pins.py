#!/usr/bin/env python3
"""Enumerate and pin the F3p package deltas (SD-36 Epic F3 polish P5, converter step: a
parameterised `PREABILITY` item `<Base> (<Option>)` whose base is a chooser converts as
`All[Holds(base), Chosen { choice: base, option: slug(<Option>) }]`, never the bare base --
`sheet_rule/prereq.rs::holdable_gate`).

Writes `structural_diff_f3p_deltas.json`, which `structural_diff.py` reads (`f3p_apply`):

  field_deltas  every (rule id, field) that moved against the baseline package, each of which must
                be EXACTLY the option gate: `structural_diff.f3p_normalize(new) == old` (drop every
                `Chosen{choice: X, option: Some}` term from an `All` list that also holds `Rule X`,
                collapse a one-term `All`). Pinned by sha256 of the new value, with the number of
                option terms it carries.
  var_tables    every `_vars/` table that moved, under the same rule, by sha256 of the parsed table.
Any other delta -- an added or removed file or rule id, a field or table that differs by more than
the option gate, a `_defects/` or `_report.json` count that moved -- is printed and nothing is
written.

Usage (from the repo root):
    python3 f3p_delta_pins.py <baseline package dir> <fresh package dir> <out json>
"""
from __future__ import annotations

import json
import os
import sys
from collections import Counter

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import structural_diff as sd  # noqa: E402


def main() -> int:
    base_root, fresh_root, out = sys.argv[1:4]
    base_tree, fresh_tree = sd.load_tree(base_root), sd.load_tree(fresh_root)
    unexplained: list[str] = []
    for kind in ("rule_files", "other_files"):
        for rel in sorted(set(base_tree[kind]) ^ set(fresh_tree[kind])):
            unexplained.append(f"file {'added' if rel in fresh_tree[kind] else 'removed'}: {rel}")
    base = sd.rules_by_id(base_tree["rule_files"])
    fresh = sd.rules_by_id(fresh_tree["rule_files"])
    for rid in sorted(set(base) ^ set(fresh)):
        unexplained.append(f"rule id {'added' if rid in fresh else 'removed'}: {rid}")
    fields: list = []
    by_field: Counter = Counter()
    by_chooser: Counter = Counter()
    for rid in sorted(set(base) & set(fresh)):
        for field in sorted(set(base[rid]) | set(fresh[rid])):
            old, new = base[rid].get(field), fresh[rid].get(field)
            if old == new:
                continue
            terms: list = []
            if sd.f3p_normalize(new, terms) != old or not terms:
                unexplained.append(f"field moved beyond the option gate: {rid}: {field}")
                continue
            fields.append([rid, field, sd.f3b2_field_sha(new), len(terms)])
            by_field[field] += len(terms)
            by_chooser.update(c or "(type-selector condition)" for c, _ in terms)
    tables: list = []
    for rel in sorted(set(base_tree["other_files"]) & set(fresh_tree["other_files"])):
        old_raw, new_raw = base_tree["other_files"][rel], fresh_tree["other_files"][rel]
        if old_raw == new_raw:
            continue
        if not rel.startswith("_vars/"):
            unexplained.append(f"side file moved: {rel}")
            continue
        old, new = json.loads(old_raw), json.loads(new_raw)
        terms = []
        if sd.f3p_normalize(new, terms) != old or not terms:
            unexplained.append(f"_vars table moved beyond the option gate: {rel}")
            continue
        tables.append([rel, sd.f3b2_field_sha(new), len(terms)])
        by_field["_vars"] += len(terms)
        by_chooser.update(c or "(type-selector condition)" for c, _ in terms)
    if unexplained:
        for line in unexplained:
            print(line)
        print(f"{len(unexplained)} unexplained -- nothing written")
        return 1
    total = sum(by_field.values())
    data = {
        "_purpose": "SD-36 Epic F3 polish P5: a parameterised PREABILITY item whose base is a chooser holds the base WITH that option chosen (All[Holds(base), Chosen{base, option}]), never the bare base; every (rule id, field) and _vars/ table that moved, each exactly that option gate; structural_diff.py gates on each (f3p_apply).",
        "_command": "python3 f3p_delta_pins.py <tranche/16 package dir, git archive 070c253e93> <fresh package dir> structural_diff_f3p_deltas.json",
        "owner": "ultimate_combat:class_feature:exotic_weapon_proficiency_firearms",
        "option_terms": {"_total": total, "by_field": dict(sorted(by_field.items())), "distinct_choosers": len(by_chooser), "by_chooser": dict(sorted(by_chooser.items()))},
        "field_deltas": {"_count": len(fields), "pins": fields},
        "var_tables": {"_count": len(tables), "pins": tables},
    }
    with open(out, "w", encoding="utf-8") as fh:
        json.dump(data, fh, indent=1, sort_keys=False)
        fh.write("\n")
    print(f"pinned {len(fields)} field deltas on {len({r for r, *_ in fields})} rules and {len(tables)} _vars tables; {total} option terms over {len(by_chooser)} choosers; by field {dict(by_field)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
