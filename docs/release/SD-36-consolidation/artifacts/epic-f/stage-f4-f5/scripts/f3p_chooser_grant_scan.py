#!/usr/bin/env python3
"""SD-36 F3 polish P5 scan over the converted package: every GRANT (`granted_by` edge) whose `when`
is conditioned on holding a chooser-bearing rule (a rule whose `offers.id` is its own id), split by
whether that holding now carries the option gate (`Chosen{choice: X, option}` beside `Holds(X)`),
or is a bare hold (no option term: a `PREABILITY` item naming the rule with no option, or a hold another token writes). The same
split is printed for record `applies` gates and `_vars/` contributions.

Usage (repo root): python3 f3p_chooser_grant_scan.py [data/sheet_rules]
"""
import json, os, sys
from collections import Counter

root = sys.argv[1] if len(sys.argv) > 1 else "data/sheet_rules"
rules = {}
for d, _, fs in os.walk(root):
    if "/_" in d.replace(root, "", 1) or d.endswith("_vars") or d.endswith("_defects"):
        continue
    for f in fs:
        if f.endswith(".json") and not f.startswith("_"):
            try:
                data = json.load(open(os.path.join(d, f)))
            except Exception:
                continue
            if isinstance(data, list):
                for r in data:
                    if isinstance(r, dict) and "id" in r:
                        rules[r["id"]] = r
choosers = {rid for rid, r in rules.items() if isinstance(r.get("offers"), dict) and r["offers"].get("id") == rid}


def holds_of(node, out_bare, out_opt, parent_all=None):
    """Record every Holds(Rule chooser) leaf: optioned when a sibling Chosen{choice: X, option} sits in the same All."""
    if isinstance(node, list):
        for v in node:
            holds_of(v, out_bare, out_opt, parent_all)
        return
    if not isinstance(node, dict):
        return
    if set(node) == {"All"} and isinstance(node["All"], list):
        opts = {t["Chosen"]["choice"] for t in node["All"] if isinstance(t, dict) and set(t) == {"Chosen"} and isinstance(t["Chosen"], dict) and t["Chosen"].get("option") is not None}
        for t in node["All"]:
            holds_of(t, out_bare, out_opt, opts)
        return
    if set(node) == {"Holds"} and isinstance(node["Holds"], dict):
        w = node["Holds"].get("what")
        if isinstance(w, dict) and set(w) == {"Rule"} and w["Rule"] in choosers:
            (out_opt if parent_all and w["Rule"] in parent_all else out_bare).append(w["Rule"])
        return
    for v in node.values():
        holds_of(v, out_bare, out_opt, None)


def scan(items):
    edges = opt_edges = bare_edges = 0
    for when in items:
        b, o = [], []
        holds_of(when, b, o)
        if b or o:
            edges += 1
            opt_edges += bool(o)
            bare_edges += bool(b) and not o
    return edges, opt_edges, bare_edges


grants = [e.get("when") for r in rules.values() for e in (r.get("granted_by") or []) if isinstance(e, dict)]
applies = [r.get("applies") for r in rules.values()]
contribs = []
for f in os.listdir(os.path.join(root, "_vars")):
    t = json.load(open(os.path.join(root, "_vars", f)))
    contribs += [c.get("when") for c in t.get("contributions", [])]
print(f"rules {len(rules)}; chooser-bearing rules (offers.id == own id) {len(choosers)}")
for name, items in (("granted_by edges", grants), ("record applies gates", applies), ("_vars contributions", contribs)):
    n, o, b = scan(items)
    print(f"{name}: {len(items)} total; {n} conditioned on holding a chooser-bearing rule -- {o} option-gated, {b} bare hold (no option term)")
