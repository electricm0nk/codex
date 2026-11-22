#!/usr/bin/env python3
"""SD-36 F1c-4 (D7): which gates the always-held base state reaches.

Reads the converted package (`data/sheet_rules`) and reports, over the whole package:

  * the always-held records (principals carrying `always_held: true`);
  * the SEEDED variables: every variable an always-held record declares AND sets with an
    unconditional (`when: Always`) contribution -- the base state `Evaluator::var` folds for
    every character since D7;
  * the ONLY-SEEDED variables: seeded variables no other record contributes to at all. Before D7
    each of these read 0 for every character, whatever it held;
  * every rule whose gate (`applies`, or a `granted_by[].when`) reads an only-seeded variable,
    as rule ids, record ids (`#suffix` stripped) and class records (`<book>:class:<slug>`).

A gate counted here READS a variable whose value moved from "always 0" to its base value; whether
the gate opens depends on its comparison, which the Rust tests pin (class_proficiency_sheet_rules.rs
`summoner_standard_summoner_gate_opens_from_the_always_held_base_state`).

Usage: python3 d7_always_held_scan.py [<sheet_rules dir>] [--json]
"""
from __future__ import annotations

import json
import sys
from pathlib import Path


def walk_vars(node, out: set[str]) -> None:
    if isinstance(node, dict):
        for k, v in node.items():
            if k == "Var" and isinstance(v, str):
                out.add(v)
            else:
                walk_vars(v, out)
    elif isinstance(node, list):
        for v in node:
            walk_vars(v, out)


def main() -> int:
    args = [a for a in sys.argv[1:] if not a.startswith("--")]
    root = Path(args[0] if args else "data/sheet_rules")
    rules: dict[str, dict] = {}
    for p in sorted(root.glob("*/*/*.json")):
        if p.parts[-3].startswith("_"):
            continue
        for r in json.loads(p.read_text()):
            rules[r["id"]] = r
    held = sorted(i for i, r in rules.items() if r.get("always_held") and "#" not in i)
    held_set = set(held)
    base = lambda rid: rid.split("#")[0]
    seeded, only_seeded = {}, {}
    for p in sorted((root / "_vars").glob("*.json")):
        t = json.loads(p.read_text())
        if not any(base(d) in held_set for d in t["declared_by"]):
            continue
        uncond = [c for c in t["contributions"] if c["when"] == "Always" and base(c["rule_id"]) in held_set]
        if not uncond:
            continue
        seeded[t["var"]] = t["label"]
        if all(base(c["rule_id"]) in held_set for c in t["contributions"]):
            only_seeded[t["var"]] = t["label"]
    gated = []
    for rid, r in rules.items():
        vs: set[str] = set()
        walk_vars(r.get("applies"), vs)
        for g in r.get("granted_by", []):
            walk_vars(g.get("when"), vs)
        hit = sorted(vs & only_seeded.keys())
        if hit:
            gated.append((rid, [only_seeded[v] for v in hit]))
    records = sorted({base(r) for r, _ in gated})
    classes = sorted({r for r in records if r.split(":")[1] == "class"})
    report = {
        "denominators": {"rules": len(rules), "records": len({base(r) for r in rules}), "var_tables": len(list((root / "_vars").glob("*.json")))},
        "always_held_records": held,
        "seeded_vars": dict(sorted(seeded.items(), key=lambda kv: kv[1])),
        "only_seeded_vars": dict(sorted(only_seeded.items(), key=lambda kv: kv[1])),
        "gated_rules": len(gated),
        "gated_records": records,
        "gated_class_records": classes,
    }
    if "--json" in sys.argv:
        print(json.dumps(report, indent=1))
        return 0
    d = report["denominators"]
    print(f"always-held records: {len(held)} of {d['records']} records")
    print(f"seeded variables: {len(seeded)} of {d['var_tables']} variable tables; only-seeded: {len(only_seeded)}")
    for v, label in report["only_seeded_vars"].items():
        print(f"  {label} ({v})")
    print(f"rules whose gate reads an only-seeded variable: {len(gated)} of {d['rules']} rules")
    print(f"  records: {len(records)} of {d['records']}; class records: {len(classes)}")
    for r in records:
        print(f"  {r}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
