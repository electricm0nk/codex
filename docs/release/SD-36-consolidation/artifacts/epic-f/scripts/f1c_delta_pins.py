#!/usr/bin/env python3
"""Enumerate and verify the F1c package delta classes (SD-36 Epic F1c, defects D1-D6).

The F1c converter batch (commits 1e6b2db9ee, 5979ef4668, e61473e9c9 on sd36/epic-f1c) changes
existing RECORD FIELDS and adds rule ids against the tranche/16 package. `structural_diff.py`
gates every field delta and names every added id; this script classifies each one into exactly
one named mechanism, VERIFIES the mechanism's own shape on the real package, and writes the pinned
lists `structural_diff.py` reads (`structural_diff_f1c_deltas.json`). A delta that fits no class,
or a class member that fails its shape check, is printed and the script exits 1 without writing:
it is a genuine, unexplained change and must stop the run, never be folded into a class.

Classes (field deltas are pinned as exact (rule id, field) pairs; added ids as exact ids):

  d2_line_split        a record whose FIRST line carried its own condition gets a Text principal
                       carrying only the record's gates; the old line becomes a new `#<suffix>`
                       sibling that keeps its condition. Check: the fresh principal's value is
                       Text; the new sibling's value/target/bonus_type/applies equal the OLD
                       principal's; the `also` and `prose` multisets are conserved
                       (old == fresh principal + sibling). Nothing is dropped, only moved.
  d4_closure_complete  `closure_complete: true` on a class principal whose closure is attested
                       defect-free (sheet_rule/attest.rs). Check: absent before, `true` after.
  d4_pi_reclosure      the 21 product-identity class records whose closure was keyed on the
                       codex-named placeholder are now keyed on the class's own base row, so
                       their continuation rows/level lines/.MOD rows convert. Check: every
                       provenance delta grows closure_rows ONLY (old rows a subset, nothing else).
  d3_unchained_class   the 4 Pathfinder Unchained class principals (new ids, whole new files).
  d6_weapon_choice     `offers` of a CHOOSE:WEAPONPROFICIENCY pick resolved to oracle weapon names,
                       or linked to the one member of a child ABILITYCATEGORY pool.
  d7_always_held       (F1c-4) `always_held: true` on the principal of a record every character
                       holds unconditionally (sheet_rule/always_held.rs). Check: a principal id,
                       absent before, `true` after.
  f1c3_preability_bracket  PREABILITY `[<key>]` items: an unresolved `MissingRule "[...]"`
                       alternative (never holdable) is removed or becomes a HeldCount exclusion.
                       Check: the old field text holds a `"name": "[` MissingRule and the fresh
                       one does not.

Usage:
    python3 f1c_delta_pins.py <baseline sheet_rules dir> <fresh sheet_rules dir> <out_json>
"""
from __future__ import annotations

import json
import os
import sys
from collections import Counter, defaultdict

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import structural_diff as sd  # noqa: E402

IGNORED = {"granted_by", "grants"}
LINE_FIELDS = ("value", "target", "bonus_type", "applies")
MULTISET_FIELDS = ("also", "prose")


def base_of(rid: str) -> str:
    return rid.split("#")[0]


# The shape checks live in structural_diff.py, which re-runs them on every pinned pair.
split_conserves = sd.d2_split_conserves
has_bracket_missing_rule = sd.has_bracket_missing_rule
d6_shape = sd.d6_shape


def main() -> int:
    if len(sys.argv) != 4:
        print(__doc__, file=sys.stderr)
        return 2
    base_dir, fresh_dir, out = sys.argv[1:]
    br = sd.rules_by_id(sd.load_tree(base_dir)["rule_files"])
    fr = sd.rules_by_id(sd.load_tree(fresh_dir)["rule_files"])
    added = sorted(set(fr) - set(br))
    deltas = {}
    for rid in sorted(set(br) & set(fr)):
        fields = sorted(k for k in (br[rid].keys() | fr[rid].keys()) - IGNORED if br[rid].get(k) != fr[rid].get(k))
        if fields:
            deltas[rid] = fields

    pi = sorted(r for r, fs in deltas.items() if "provenance" in fs and "#" not in r and r.split(":")[1] == "class")
    pi_set = set(pi)
    new_sibs = defaultdict(list)
    for a in added:
        if "#" in a:
            new_sibs[base_of(a)].append(a)
    splits: dict[str, str] = {}
    for rec, sibs in new_sibs.items():
        if rec in br and rec in fr:
            for s in sibs:
                if split_conserves(br[rec], fr[rec], fr[s]):
                    splits[rec] = s
                    break

    classes: dict[str, dict] = {k: {"field_deltas": [], "new_rule_ids": []} for k in (
        "d2_line_split", "d4_closure_complete", "d4_pi_reclosure", "d3_unchained_class", "d6_weapon_choice", "f1c3_preability_bracket", "d7_always_held")}
    problems: list[str] = []
    for rid, fields in deltas.items():
        for field in fields:
            o, n = br[rid].get(field), fr[rid].get(field)
            if field == "always_held":
                if o is None and n is True and "#" not in rid:
                    classes["d7_always_held"]["field_deltas"].append([rid, field])
                else:
                    problems.append(f"{rid}: always_held {o!r} -> {n!r}")
            elif field == "closure_complete":
                if o is None and n is True and rid.split(":")[1] == "class" and "#" not in rid:
                    classes["d4_closure_complete"]["field_deltas"].append([rid, field])
                else:
                    problems.append(f"{rid}: closure_complete {o!r} -> {n!r}")
            elif base_of(rid) in pi_set:
                if field == "provenance" and not sd._provenance_delta_is_closure_rows_growth_only(o, n):
                    problems.append(f"{rid}: provenance delta is not closure_rows growth only")
                else:
                    classes["d4_pi_reclosure"]["field_deltas"].append([rid, field])
            elif has_bracket_missing_rule(o) and not has_bracket_missing_rule(n):
                classes["f1c3_preability_bracket"]["field_deltas"].append([rid, field])
            elif rid in splits and field in LINE_FIELDS + MULTISET_FIELDS:
                classes["d2_line_split"]["field_deltas"].append([rid, field])
            elif field == "offers" and fields == ["offers"] and d6_shape(o, n):
                classes["d6_weapon_choice"]["field_deltas"].append([rid, field])
            else:
                problems.append(f"{rid}: {field} (no class)")
    for a in added:
        if "#" not in a:
            if a.startswith("pathfinder_unchained:class:") and any(e.get("TakenOnClass") for e in fr[a].get("grants") or [] if isinstance(e, dict)):
                classes["d3_unchained_class"]["new_rule_ids"].append(a)
            else:
                problems.append(f"{a}: added principal (no class)")
        elif splits.get(base_of(a)) == a:
            classes["d2_line_split"]["new_rule_ids"].append(a)
        elif base_of(a) in pi_set:
            classes["d4_pi_reclosure"]["new_rule_ids"].append(a)
        else:
            problems.append(f"{a}: added sibling (no class)")

    if problems:
        print(f"UNCLASSIFIED: {len(problems)}")
        for p in problems:
            print("  " + p)
        return 1
    doc = {
        "_purpose": "SD-36 Epic F1c: the package delta classes of the F1c converter batch vs the tranche/16 package, each verified by f1c_delta_pins.py (see its docstring for each class's mechanism and shape check). structural_diff.py accepts a field delta only on an exact pinned (rule id, field) pair of a class, re-running that class's shape check; an added id's cause is its class. Never a blanket allowance.",
        "_command": "python3 f1c_delta_pins.py <tranche/16 worktree>/data/sheet_rules data/sheet_rules structural_diff_f1c_deltas.json",
        "d2_splits": dict(sorted(splits.items())),
        "classes": {},
    }
    for name, c in classes.items():
        doc["classes"][name] = {
            "_field_delta_count": len(c["field_deltas"]),
            "_record_count": len({r for r, _ in c["field_deltas"]}),
            "_new_rule_id_count": len(c["new_rule_ids"]),
            "field_deltas": c["field_deltas"],
            "new_rule_ids": c["new_rule_ids"],
        }
        if name == "d6_weapon_choice":
            doc["classes"][name]["_by_shape"] = dict(Counter(d6_shape(br[r].get("offers"), fr[r].get("offers")) for r, _ in c["field_deltas"]))
        print(f"{name}: {len(c['field_deltas'])} field deltas on {len({r for r, _ in c['field_deltas']})} records, {len(c['new_rule_ids'])} new ids; e.g. {sorted({r for r, _ in c['field_deltas']} | set(c['new_rule_ids']))[:3]}")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump(doc, fh, indent=1, sort_keys=False)
        fh.write("\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
