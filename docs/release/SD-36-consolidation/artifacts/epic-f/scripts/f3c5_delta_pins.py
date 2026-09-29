#!/usr/bin/env python3
"""Enumerate and pin the F3c5 package deltas (SD-36 Epic F3c5, converter step 5: Internal
natural-attack helper rows convert as NaturalAttack facts on the rule that grants them).

Writes `structural_diff_f3c5_deltas.json`, which `structural_diff.py` reads (`f3c5_check`):

  required_added_grants  every `grants` entry in the fresh package absent from the prior step's (F3c4)
                         package; each must be a (Gated)FactGrant of a NaturalAttack fact. A removed
                         grant, or an added grant of any other shape, is printed and nothing is written.
  added_rules            every rule id in the fresh package absent from tranche/16 AND from the prior
                         step's package, classified by `structural_diff.f3c5_classify_added`, pinned as
                         (id, principal, sha256 of the whole rule).
  field_deltas           every (rule id, field) other than grants/granted_by that moved against the
                         prior step's package, classified by `structural_diff.f3c5_classify_field`
                         against BOTH the prior package and tranche/16 (the diff runs against tranche/16),
                         pinned by sha256 of the new value.
  added_var_tables       every `_vars/` table the prior step's package did not have, by sha256; a table
                         both packages have that moved is printed and nothing is written.
  defect_rows            the row count of every `_defects/` file whose count moved, and of the
                         natural-attack-helper defect files (0 when absent).
A removed granted_by edge or a removed rule id is printed and nothing is written.

Usage (from the repo root):
    python3 f3c5_delta_pins.py <tranche/16 package dir> data/sheet_rules <out json> <F3c4 package dir>
"""
from __future__ import annotations

import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import structural_diff as sd  # noqa: E402

OWNER = "core_rulebook:class:dragon_disciple"
NAMED_DEFECTS = [
    "_defects/natural-attack-helper-carries-more.json",
    "_defects/natural-attack-helper-names-no-attack.json",
    "_defects/natural-attack-helper-pair-shared.json",
]


def main() -> int:
    base_root, fresh_root, out, prior_root = sys.argv[1:5]
    base_tree, fresh_tree, prior_tree = sd.load_tree(base_root), sd.load_tree(fresh_root), sd.load_tree(prior_root)
    base = sd.rules_by_id(base_tree["rule_files"])
    fresh = sd.rules_by_id(fresh_tree["rule_files"])
    prior = sd.rules_by_id(prior_tree["rule_files"])
    unexplained: list[str] = []
    fields: dict[str, list] = {n: [] for n in sd.F3C5_CLASS_CAUSES if n != "f3c5_line_sibling"}
    grants: list = []
    for rid in sorted(set(prior) & set(fresh)):
        for field in sorted((set(prior[rid]) | set(fresh[rid])) - {"granted_by", "grants"}):
            if prior[rid].get(field) == fresh[rid].get(field):
                continue
            cls = sd.f3c5_classify_field(rid, field, prior[rid], fresh[rid], fresh)
            cls_base = sd.f3c5_classify_field(rid, field, base[rid], fresh[rid], fresh) if rid in base else cls
            if cls is None or cls_base != cls:
                unexplained.append(f"field moved since the prior step {rid}: {field} (vs prior {cls}, vs tranche/16 {cls_base})")
            else:
                fields[cls].append([rid, field, sd.f3b2_field_sha(fresh[rid].get(field))])
        removed, _ = sd.edge_diff(prior[rid].get("granted_by"), fresh[rid].get("granted_by"))
        for key in removed:
            unexplained.append(f"granted_by edge removed since the prior step {rid}: {key}")
        _, added_edges = sd.edge_diff(prior[rid].get("granted_by"), fresh[rid].get("granted_by"))
        for key in added_edges:
            unexplained.append(f"granted_by edge added since the prior step {rid}: {key}")
        g_removed, g_added = sd.edge_diff(prior[rid].get("grants"), fresh[rid].get("grants"))
        for key in g_removed:
            unexplained.append(f"grant removed since the prior step {rid}: {key}")
        for key in g_added:
            if sd._is_natural_attack_grant(json.loads(key)):
                grants.append([rid, key])
            else:
                unexplained.append(f"grant added since the prior step is not a NaturalAttack fact {rid}: {key}")
    for rid in sorted(set(prior) - set(fresh)):
        unexplained.append(f"rule id removed since the prior step: {rid}")
    added: list = []
    for rid in sorted(set(fresh) - set(base) - set(prior)):
        cls = sd.f3c5_classify_added(rid, fresh)
        if cls is None:
            unexplained.append(f"added rule id fits no F3c5 class: {rid}")
            continue
        added.append([rid, rid.split("#")[0], sd.f3b2_field_sha(fresh[rid])])
    tables: list = []
    for rel in sorted(p for p in fresh_tree["other_files"] if p.startswith("_vars/")):
        new_raw = fresh_tree["other_files"][rel]
        old_raw = prior_tree["other_files"].get(rel)
        if old_raw is None:
            tables.append([rel, sd.f3b2_field_sha(json.loads(new_raw))])
        elif json.loads(old_raw) != json.loads(new_raw):
            unexplained.append(f"_vars table moved since the prior step: {rel}")
    for rel in sorted(p for p in prior_tree["other_files"] if p.startswith("_vars/") and p not in fresh_tree["other_files"]):
        unexplained.append(f"_vars table removed since the prior step: {rel}")
    defect_rows = {}
    for rel in sorted(set(p for p in fresh_tree["other_files"] if p.startswith("_defects/")) | set(p for p in prior_tree["other_files"] if p.startswith("_defects/"))):
        new_rows = json.loads(fresh_tree["other_files"].get(rel, b"[]") or b"[]")
        old_rows = json.loads(prior_tree["other_files"].get(rel, b"[]") or b"[]")
        if len(new_rows) != len(old_rows):
            defect_rows[rel] = len(new_rows)
        if not set(json.dumps(r, sort_keys=True) for r in new_rows) <= set(json.dumps(r, sort_keys=True) for r in old_rows) and rel not in NAMED_DEFECTS:
            unexplained.append(f"_defects rows added since the prior step: {rel}")
    for rel in NAMED_DEFECTS:
        defect_rows.setdefault(rel, len(json.loads(fresh_tree["other_files"].get(rel, b"[]") or b"[]")))
    for line in unexplained[:50]:
        print(f"UNEXPLAINED {line}")
    if unexplained:
        print(f"{len(unexplained)} unexplained deltas; nothing written")
        return 1
    data = {
        "_purpose": "SD-36 Epic F3c5: the NaturalAttack fact grants an Internal natural-attack helper reference now converts to, the CONV-02 line siblings and principal field moves that follow on a record carrying one, the closure_complete attestation it earns (Dragon Disciple), the _vars/ tables a fact's gate first writes, and the _defects/ row counts, all against the F3c4 package; structural_diff.py gates on each (f3c5_check).",
        "_command": "python3 f3c5_delta_pins.py <tranche/16 package dir, git archive cc21cac195> data/sheet_rules structural_diff_f3c5_deltas.json <F3c4 package dir, git archive 5bdf879c24>",
        "owner": OWNER,
        "added_rules": {"f3c5_line_sibling": {"_count": len(added), "pins": added}},
        "field_deltas": {name: {"_count": len(p), "pins": p} for name, p in fields.items()},
        "required_added_grants": {"_count": len(grants), "pins": grants},
        "added_var_tables": {"_count": len(tables), "pins": tables},
        "defect_rows": defect_rows,
    }
    with open(out, "w", encoding="utf-8") as fh:
        json.dump(data, fh, indent=1)
        fh.write("\n")
    print(f"f3c5_line_sibling: {len(added)} pins")
    for name, p in fields.items():
        print(f"{name}: {len(p)} pins on {len({x[0] for x in p})} rule ids")
    print(f"required_added_grants: {len(grants)} on {len({g[0] for g in grants})} rule ids")
    print(f"added_var_tables: {len(tables)}")
    print(f"defect_rows: {defect_rows}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
