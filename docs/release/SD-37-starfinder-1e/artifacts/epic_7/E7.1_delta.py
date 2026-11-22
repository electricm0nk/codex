#!/usr/bin/env python3
"""SD-37 E7.1 per-card structural classifier for the Starfinder package (the E4.MC ruling: SF keeps
per-card pins; the SD-36 structural_diff.py pins do not cover this card's classes).

E7.1's converter change (`always_held::convert_unconverted_globals`) writes Starfinder's global
`Default` as ONE always-held record carrying its variable bookkeeping. Every package file it may
move is classified; anything else is an `other` move and the script exits 1.

Classes (each with its predicate):
  global_default_record   the new file core/ability/default.json: one rule, always_held, print false,
                          no target, no grants
  declarer_default        a var table whose ONLY change is `declared_by`: base = the sorted set of its
                          contributors (E4.2's fold for a variable only the unconverted global DEFINEd),
                          new = ["core:ability:default"]
  declarer_default_added  a var table whose ONLY change is `declared_by` gaining core:ability:default
                          next to its own declarers (the global DEFINEs it too, `DEFINE:<var>|0`): the
                          variable now counts for every character, as in the oracle, where before it
                          counted only while one of its other declarers was held
  default_contribution    a var table whose contributions are the base's plus contributions from
                          core:ability:default only, and whose declared_by is the base's (or, E4.2's
                          contributor list, now default) plus core:ability:default
  declared_empty          a var table with no declarer and no contribution at base, now declared by
                          core:ability:default: 0 before and after
  new_var_default         a var table absent at base (no rule or contribution referenced it) and
                          declared by core:ability:default alone: it is referenced now only through
                          the global's own contributions (`SkillCap_Level`, `CS_Profession`)
  report                  _report.json (rules_written +1, var_tables +n new vars, nothing else),
                          _defects/unresolved-references.json (exactly the Default global line removed),
                          var_names.json (additions only)

Usage: E7.1_delta.py <new sheet_rules dir> <base sheet_rules dir>   (exit 0 iff other_moves == 0)
"""
import json
import os
import sys
from collections import Counter

DEFAULT = "core:ability:default"


def load(path):
    with open(path) as f:
        return json.load(f)


def files_under(root):
    out = set()
    for d, _, names in os.walk(root):
        for n in names:
            if n.endswith(".json"):
                out.add(os.path.relpath(os.path.join(d, n), root))
    return out


def key(c):
    return json.dumps(c, sort_keys=True)


def classify_var(new, base):
    if base is None:
        return "new_var_default" if sorted(new["declared_by"]) == [DEFAULT] else None
    if {k: v for k, v in new.items() if k not in ("declared_by", "contributions")} != {k: v for k, v in base.items() if k not in ("declared_by", "contributions")}:
        return None
    contributors = sorted({c["rule_id"] for c in base["contributions"]})
    base_decl = sorted(base["declared_by"])
    new_decl = sorted(new["declared_by"])
    if new["contributions"] == base["contributions"]:
        if new_decl == [DEFAULT] and base_decl == contributors and base_decl:
            return "declarer_default"
        if DEFAULT not in base_decl and new_decl == sorted(base_decl + [DEFAULT]) and base_decl:
            return "declarer_default_added"
        if not base_decl and not base["contributions"] and new_decl == [DEFAULT]:
            return "declared_empty"
        return None
    kept = [c for c in new["contributions"] if c["rule_id"] != DEFAULT]
    added = [c for c in new["contributions"] if c["rule_id"] == DEFAULT]
    if sorted(map(key, kept)) != sorted(map(key, base["contributions"])) or not added:
        return None
    rest = [d for d in new_decl if d != DEFAULT]
    if DEFAULT in new_decl and (rest == base_decl or (rest == [] and base_decl in ([], contributors))):
        return "default_contribution"
    return None


def main():
    new_root, base_root = sys.argv[1], sys.argv[2]
    counts = Counter()
    other = []
    new_files, base_files = files_under(new_root), files_under(base_root)
    for rel in sorted(new_files | base_files):
        n = os.path.join(new_root, rel)
        b = os.path.join(base_root, rel)
        if rel in new_files and rel in base_files and open(n, "rb").read() == open(b, "rb").read():
            continue
        if rel == os.path.join("core", "ability", "default.json") and rel not in base_files:
            rules = load(n)
            r = rules[0] if len(rules) == 1 else {}
            ok = r.get("id") == DEFAULT and r.get("always_held") is True and r.get("print") is False and "target" not in r and not r.get("grants")
            (counts.__setitem__("global_default_record", counts["global_default_record"] + 1) if ok else other.append((rel, "default record shape")))
            continue
        if rel.startswith("_vars" + os.sep) and rel in new_files:
            cls = classify_var(load(n), load(b) if rel in base_files else None)
            if cls:
                counts[cls] += 1
            else:
                other.append((rel, "var table move outside the classes"))
            continue
        if rel == "_report.json":
            nr, br = load(n), load(b)
            diff = {k for k in set(nr) | set(br) if nr.get(k) != br.get(k)}
            new_vars = counts["new_var_default"]  # filled before: _vars sorts after _report? guard below
            if diff <= {"rules_written", "var_tables"} and nr["rules_written"] == br["rules_written"] + 1:
                counts["report"] += 1
                report_var_delta = nr["var_tables"] - br["var_tables"]
            else:
                other.append((rel, f"report fields moved: {sorted(diff)}"))
                report_var_delta = None
            continue
        if rel == os.path.join("_defects", "unresolved-references.json"):
            nd, bd = load(n), load(b)
            removed = [x for x in bd if x not in nd]
            added = [x for x in nd if x not in bd]
            if not added and len(removed) == 1 and removed[0].startswith("INTERNAL|DEFAULT ("):
                counts["report"] += 1
            else:
                other.append((rel, f"defect lines moved: -{len(removed)} +{len(added)}"))
            continue
        other.append((rel, "file outside the classes"))
    print(" ".join(f"{k}: {v}" for k, v in sorted(counts.items())) + f" other_moves={len(other)}")
    for rel, why in other[:40]:
        print(f"OTHER {rel}: {why}")
    sys.exit(1 if other else 0)


if __name__ == "__main__":
    main()
