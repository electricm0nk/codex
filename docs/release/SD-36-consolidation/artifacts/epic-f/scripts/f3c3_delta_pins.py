#!/usr/bin/env python3
"""Enumerate and pin the F3c3 package deltas (SD-36 Epic F3c3, converter step 3: PCGen SUBCLASS lines).

The F3c3 converter change (sd36/epic-f2-f3) ADDS rules and grows existing ones; it moves no existing
field. This script checks that and writes `structural_diff_f3c3_deltas.json`, which
`structural_diff.py` reads:

  added_rules            every rule id in the fresh package and absent from tranche/16 that no
                         earlier step names, classified by `structural_diff.f3c3_classify_added`
                         (`f3c3_subclass_choice` / `f3c3_subclass_option`), pinned as (id, owning
                         class principal, sha256 of the whole rule). An added id that fits no class
                         is printed and nothing is written.
  required_added_edges   every `granted_by` edge in the fresh package absent from the PRIOR step's
                         package (F3c2, `de01bf571a`) -- the options' grant edges.
  var_contributions      every `_vars/` contribution in the fresh package absent from the prior
                         step's package, as (file, canonical JSON). A contribution the prior step
                         had and the fresh package lost is printed and nothing is written.
  defect_rows            the row counts of the two new `_defects/` files.

It also requires that no existing rule's fields moved against the prior step (every field of every
rule id both packages hold is equal, `granted_by` aside, which may only grow).

Usage (from the repo root):
    python3 f3c3_delta_pins.py <tranche/16 package dir> data/sheet_rules <out json> <F3c2 package dir>
"""
from __future__ import annotations

import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import structural_diff as sd  # noqa: E402

DEFECT_FILES = ["_defects/subclass-token-unconverted.json", "_defects/subclass-superseded-reprint.json"]
OWNER = "ultimate_psionics:class:psion"


def owner_of(rid: str, rule: dict) -> str:
    if rid.endswith("#subclass"):
        return rid.split("#")[0]
    for g in rule.get("granted_by") or []:
        ch = (g.get("by") or {}).get("Choice")
        if isinstance(ch, str) and ch.endswith("#subclass"):
            return ch.split("#")[0]
    return ""


def main() -> int:
    base_root, fresh_root, out, prior_root = sys.argv[1:5]
    base_tree, fresh_tree, prior_tree = sd.load_tree(base_root), sd.load_tree(fresh_root), sd.load_tree(prior_root)
    base = sd.rules_by_id(base_tree["rule_files"])
    fresh = sd.rules_by_id(fresh_tree["rule_files"])
    prior = sd.rules_by_id(prior_tree["rule_files"])
    unexplained: list[str] = []
    # No existing rule's fields moved against the prior step.
    for rid in sorted(set(prior) & set(fresh)):
        for field in sorted((set(prior[rid]) | set(fresh[rid])) - {"granted_by"}):
            if prior[rid].get(field) != fresh[rid].get(field):
                unexplained.append(f"field moved since the prior step {rid}: {field}")
        removed, _ = sd.edge_diff(prior[rid].get("granted_by"), fresh[rid].get("granted_by"))
        if removed:
            unexplained.append(f"granted_by edge removed since the prior step {rid}: {removed}")
    for rid in sorted(set(prior) - set(fresh)):
        unexplained.append(f"rule id removed since the prior step: {rid}")
    # Added rule ids against tranche/16 that the prior step did not already carry.
    added: dict[str, list] = {name: [] for name in sd.F3C3_CLASS_CAUSES}
    for rid in sorted(set(fresh) - set(base) - set(prior)):
        cls = sd.f3c3_classify_added(rid, fresh[rid])
        own = owner_of(rid, fresh[rid]) if cls == "f3c3_subclass_choice" else ""
        if cls == "f3c3_subclass_option":
            principal = fresh.get(rid.split("#")[0], {})
            own = owner_of(rid.split("#")[0], principal)
        if cls is None or not own:
            unexplained.append(f"added rule id fits no F3c3 class: {rid}")
            continue
        added[cls].append([rid, own, sd.f3c3_rule_sha(fresh[rid])])
    required_edges = []
    for rid in sorted(set(prior) & set(fresh)):
        _, new = sd.edge_diff(prior[rid].get("granted_by"), fresh[rid].get("granted_by"))
        required_edges.extend([rid, key] for key in new)
    contribs = []
    for rel in sorted(p for p in fresh_tree["other_files"] if p.startswith("_vars/")):
        new_t = json.loads(fresh_tree["other_files"][rel])
        old_t = json.loads(prior_tree["other_files"].get(rel, b"{}") or b"{}")
        old_keys = {json.dumps(c, sort_keys=True) for c in old_t.get("contributions", [])}
        new_keys = {json.dumps(c, sort_keys=True) for c in new_t.get("contributions", [])}
        for k in sorted(old_keys - new_keys):
            unexplained.append(f"contribution removed since the prior step in {rel}: {k}")
        contribs.extend([rel, k] for k in sorted(new_keys - old_keys))
        for field in sorted((set(old_t) | set(new_t)) - {"contributions"}):
            if old_t and old_t.get(field) != new_t.get(field):
                unexplained.append(f"_vars field moved since the prior step {rel}: {field}")
    defect_rows = {}
    for rel in DEFECT_FILES:
        rows = json.loads(fresh_tree["other_files"].get(rel, b"[]") or b"[]")
        defect_rows[rel] = len(rows)
    for line in unexplained:
        print(f"UNEXPLAINED {line}")
    if unexplained:
        print(f"{len(unexplained)} unexplained deltas; nothing written")
        return 1
    data = {
        "_purpose": "SD-36 Epic F3c3: the rules the SUBCLASS conversion adds (pinned by content sha256), the granted_by edges and _vars/ contributions it adds against the F3c2 package, and the row counts of its two _defects/ files; structural_diff.py gates on each (f3c3_check).",
        "_command": "python3 f3c3_delta_pins.py <tranche/16 package dir, git archive cc21cac195> data/sheet_rules structural_diff_f3c3_deltas.json <F3c2 package dir, git archive de01bf571a>",
        "owner": OWNER,
        "added_rules": {name: {"_count": len(pins), "pins": pins} for name, pins in added.items()},
        "required_added_edges": {"_count": len(required_edges), "pins": required_edges},
        "var_contributions": {"_count": len(contribs), "pins": contribs},
        "defect_rows": defect_rows,
    }
    with open(out, "w", encoding="utf-8") as fh:
        json.dump(data, fh, indent=1)
        fh.write("\n")
    for name, pins in added.items():
        print(f"{name}: {len(pins)} pins")
    print(f"required_added_edges: {len(required_edges)} edges on {len({e[0] for e in required_edges})} rule ids")
    print(f"var_contributions: {len(contribs)} on {len({c[0] for c in contribs})} tables")
    print(f"defect_rows: {defect_rows}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
