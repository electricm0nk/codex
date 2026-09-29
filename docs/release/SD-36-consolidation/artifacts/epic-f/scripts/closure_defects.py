#!/usr/bin/env python3
"""Closure defects of a class, re-derived from the committed package (SD-36 Epic F1c).

The same walk as `crates/codex-ingest/src/pcgen_import/sheet_rule/attest.rs`: every record reachable
from the class principal through `Granter::Class { id: <class> }` edges (any level) and
`Granter::Rule` edges (transitively), plus the base class's closure for a `TakenOnClass` class.
Gates are ignored. For each record reached it lists the `_defects/<kind>.json` rows attributed to
it, for the attestation's closure-defect kinds. `grants-to-unconverted-targets` is keyed by the
target in the package, not the granter, so it cannot be attributed here; a class whose principal
reads `closure_complete: false` with no row listed below is printed as `unattributed`.

Read-only. Usage (from the repo root):
    python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/closure_defects.py [--json] <class slug>...
"""
from __future__ import annotations

import glob
import json
import sys
from collections import defaultdict

ROOT = "data/sheet_rules"
KINDS = ["unresolved-references", "ambiguous-parent-category-target", "grant-by-type", "undefined-variables", "unrecognized-proficiency-tag"]


def record_of(rule_id: str) -> str:
    return rule_id.split("#")[0]


def load():
    rule_edges: dict[str, set[str]] = defaultdict(set)
    class_edges: dict[str, set[str]] = defaultdict(set)
    principals: dict[str, list[dict]] = defaultdict(list)
    for path in glob.glob(f"{ROOT}/*/*/*.json"):
        rel = path[len(ROOT) + 1:]
        if rel.startswith("_"):
            continue
        rules = json.load(open(path))
        if not isinstance(rules, list) or not rules:
            continue
        for rule in rules:
            target = record_of(rule["id"])
            for g in rule.get("granted_by") or []:
                by = g["by"]
                if isinstance(by, dict) and "Rule" in by:
                    rule_edges[record_of(by["Rule"])].add(target)
                elif isinstance(by, dict) and "Class" in by:
                    class_edges[by["Class"]["id"]].add(target)
        if rel.split("/")[1] == "class":
            principals[rules[0]["id"].rsplit(":", 1)[1]].append(rules[0])
    rows: dict[str, list[tuple[str, str]]] = defaultdict(list)
    for kind in KINDS:
        try:
            data = json.load(open(f"{ROOT}/_defects/{kind}.json"))
        except FileNotFoundError:
            continue
        for row in data:
            rid, _, detail = row.partition(": ")
            rows[record_of(rid)].append((kind, detail))
    return rule_edges, class_edges, principals, rows


def closure_defects(slug, rule_edges, class_edges, principals, rows):
    stack: list[str] = []
    classes = [slug]
    for p in principals.get(slug, []):
        for e in p.get("grants") or []:
            if isinstance(e, dict) and "TakenOnClass" in e:
                classes.append(e["TakenOnClass"])
    for c in classes:
        stack.extend(p["id"] for p in principals.get(c, []))
        stack.extend(class_edges.get(c, ()))
    seen: set[str] = set()
    found = []
    while stack:
        rec = stack.pop()
        if rec in seen:
            continue
        seen.add(rec)
        for kind, detail in rows.get(rec, []):
            found.append({"record": rec, "kind": kind, "detail": detail})
        stack.extend(rule_edges.get(rec, ()))
    attested = [bool(p.get("closure_complete")) for p in principals.get(slug, [])]
    return {"class": slug, "closure_records": len(seen), "closure_complete": attested, "defects": sorted(found, key=lambda d: (d["kind"], d["record"], d["detail"]))}


def main(argv: list[str]) -> int:
    as_json = "--json" in argv
    slugs = [a for a in argv if not a.startswith("--")]
    data = load()
    out = [closure_defects(s, *data) for s in slugs]
    if as_json:
        print(json.dumps(out, indent=1))
        return 0
    for c in out:
        by_kind: dict[str, int] = defaultdict(int)
        for d in c["defects"]:
            by_kind[d["kind"]] += 1
        summary = ", ".join(f"{k} {n}" for k, n in sorted(by_kind.items())) or "unattributed"
        print(f"{c['class']}: closure records {c['closure_records']}; closure_complete {c['closure_complete']}; {summary}")
        for d in c["defects"]:
            print(f"  {d['kind']}: {d['record']}: {d['detail']}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
