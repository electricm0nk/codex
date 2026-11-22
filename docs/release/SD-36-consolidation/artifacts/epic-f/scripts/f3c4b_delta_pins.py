#!/usr/bin/env python3
"""Enumerate and pin the F3c4b package deltas (SD-36 Epic F3c4b, converter step 4: ability-category
pick rows as choice options; CATEGORY-less records found under their row's declared category).

Writes `structural_diff_f3c4b_deltas.json`, which `structural_diff.py` reads (`f3c4b_check`):

  added_rules            every rule id in the fresh package absent from tranche/16 AND from the prior
                         step's package, classified by `structural_diff.f3c4b_classify_added`
                         (`f3c4b_pool_option`), pinned as (id, first choosing record, sha256 of the
                         whole rule). An added id that fits no class is printed and nothing is written.
  field_deltas           every (rule id, field) that moved against the PRIOR step's package, classified
                         by `structural_diff.f3c4b_classify_field`, pinned by sha256 of the new value.
                         A moved field that fits no class is printed and nothing is written.
  replaced_edges         every granted_by edge the prior step had and the fresh package lost: allowed
                         only in `structural_diff.f3c4b_replacement_holds`'s shape (a parameterised
                         reference's fallback edge, now on the pool_option it names exactly).
  required_added_edges   every granted_by edge in the fresh package absent from the prior step's.
  var_contributions      every `_vars/` contribution in the fresh package absent from the prior step's.
                         A lost contribution is allowed only when a fresh one from the same rule equals
                         it modulo MissingRule -> Rule; any other `_vars/` field move must be an
                         `outside_corpus_rows` shrink or a `declared_by` growth.
  defect_rows            the row counts of the new `_defects/` files and of unresolved-references.

Usage (from the repo root):
    python3 f3c4b_delta_pins.py <tranche/16 package dir> data/sheet_rules <out json> <F3c4 package dir>
"""
from __future__ import annotations

import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import structural_diff as sd  # noqa: E402

DEFECT_FILES = [
    "_defects/pool-option-id-collision.json",
    "_defects/pool-option-unconverted.json",
    "_defects/row-declared-category-shared.json",
    "_defects/unresolved-references.json",
]
OWNER = "core_rulebook:class_feature:sorcerer_standard_bloodline_selection"


def main() -> int:
    base_root, fresh_root, out, prior_root = sys.argv[1:5]
    base_tree, fresh_tree, prior_tree = sd.load_tree(base_root), sd.load_tree(fresh_root), sd.load_tree(prior_root)
    base = sd.rules_by_id(base_tree["rule_files"])
    fresh = sd.rules_by_id(fresh_tree["rule_files"])
    prior = sd.rules_by_id(prior_tree["rule_files"])
    unexplained: list[str] = []
    fields: dict[str, list] = {n: [] for n in sd.F3C4B_CLASS_CAUSES if n != "f3c4b_pool_option"}
    replaced: list = []
    for rid in sorted(set(prior) & set(fresh)):
        for field in sorted((set(prior[rid]) | set(fresh[rid])) - {"granted_by"}):
            if prior[rid].get(field) == fresh[rid].get(field):
                continue
            cls = sd.f3c4b_classify_field(rid, field, prior[rid], fresh[rid])
            if cls is None:
                unexplained.append(f"field moved since the prior step {rid}: {field}")
            else:
                fields[cls].append([rid, field, sd.f3b2_field_sha(fresh[rid].get(field))])
        removed, _ = sd.edge_diff(prior[rid].get("granted_by"), fresh[rid].get("granted_by"))
        for key in removed:
            if sd.f3c4b_replacement_holds(rid, key, fresh):
                replaced.append([rid, key])
            else:
                unexplained.append(f"granted_by edge removed since the prior step {rid}: {key}")
    for rid in sorted(set(prior) - set(fresh)):
        unexplained.append(f"rule id removed since the prior step: {rid}")
    added: list = []
    for rid in sorted(set(fresh) - set(base) - set(prior)):
        cls = sd.f3c4b_classify_added(rid, fresh[rid])
        principal = fresh.get(rid.split("#")[0], {})
        owner = next((g["by"]["Choice"] for g in principal.get("granted_by") or [] if isinstance((g.get("by") or {}).get("Choice"), str)), "")
        if cls is None or not owner:
            unexplained.append(f"added rule id fits no F3c4b class: {rid}")
            continue
        added.append([rid, owner, sd.f3b2_field_sha(fresh[rid])])
    required_edges = []
    for rid in sorted(set(prior) & set(fresh)):
        _, new = sd.edge_diff(prior[rid].get("granted_by"), fresh[rid].get("granted_by"))
        required_edges.extend([rid, key] for key in new)
    contribs = []
    for rel in sorted(p for p in fresh_tree["other_files"] if p.startswith("_vars/")):
        new_t = json.loads(fresh_tree["other_files"][rel])
        old_t = json.loads(prior_tree["other_files"].get(rel, b"{}") or b"{}")
        old_l, new_l = old_t.get("contributions", []), new_t.get("contributions", [])
        new_keys = {json.dumps(c, sort_keys=True) for c in new_l}
        old_keys = {json.dumps(c, sort_keys=True) for c in old_l}
        for c in old_l:
            if json.dumps(c, sort_keys=True) in new_keys:
                continue
            if not any(n.get("rule_id") == c.get("rule_id") and sd.f3c4b_missing_to_rule(c, n) for n in new_l):
                unexplained.append(f"contribution removed since the prior step in {rel}: {json.dumps(c, sort_keys=True)}")
        contribs.extend([rel, k] for k in sorted(new_keys - old_keys))
        if old_t:
            op = (old_t.get("provenance") or {}).get("outside_corpus_rows", [])
            np_ = (new_t.get("provenance") or {}).get("outside_corpus_rows", [])
            if not set(np_) <= set(op):
                unexplained.append(f"_vars outside_corpus_rows grew in {rel}")
            if not set(old_t.get("declared_by", [])) <= set(new_t.get("declared_by", [])):
                unexplained.append(f"_vars declared_by lost a record in {rel}")
            for field in sorted((set(old_t) | set(new_t)) - {"contributions", "provenance", "declared_by"}):
                if old_t.get(field) != new_t.get(field):
                    unexplained.append(f"_vars field moved since the prior step {rel}: {field}")
    defect_rows = {}
    for rel in DEFECT_FILES:
        rows = json.loads(fresh_tree["other_files"].get(rel, b"[]") or b"[]")
        defect_rows[rel] = len(rows)
    for line in unexplained[:50]:
        print(f"UNEXPLAINED {line}")
    if unexplained:
        print(f"{len(unexplained)} unexplained deltas; nothing written")
        return 1
    data = {
        "_purpose": "SD-36 Epic F3c4b: the pool_option rules the pick-row conversion adds (pinned by content sha256), the field deltas it moves (MissingRule -> Rule gate terms; closure_complete attestations), the parameterised self-edges it replaces, the granted_by edges and _vars/ contributions it adds against the F3c4 package, and its _defects/ row counts; structural_diff.py gates on each (f3c4b_check).",
        "_command": "python3 f3c4b_delta_pins.py <tranche/16 package dir, git archive cc21cac195> data/sheet_rules structural_diff_f3c4b_deltas.json <F3c4 package dir, git archive 713bcfba74>",
        "owner": OWNER,
        "added_rules": {"f3c4b_pool_option": {"_count": len(added), "pins": added}},
        "field_deltas": {name: {"_count": len(p), "pins": p} for name, p in fields.items()},
        "replaced_edges": {"_count": len(replaced), "pins": replaced},
        "required_added_edges": {"_count": len(required_edges), "pins": required_edges},
        "var_contributions": {"_count": len(contribs), "pins": contribs},
        "defect_rows": defect_rows,
    }
    with open(out, "w", encoding="utf-8") as fh:
        json.dump(data, fh, indent=1)
        fh.write("\n")
    print(f"f3c4b_pool_option: {len(added)} pins")
    for name, p in fields.items():
        print(f"{name}: {len(p)} pins on {len({x[0] for x in p})} rule ids")
    print(f"replaced_edges: {len(replaced)}")
    print(f"required_added_edges: {len(required_edges)} edges on {len({e[0] for e in required_edges})} rule ids")
    print(f"var_contributions: {len(contribs)} on {len({c[0] for c in contribs})} tables")
    print(f"defect_rows: {defect_rows}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
