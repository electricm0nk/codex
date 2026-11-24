#!/usr/bin/env python3
"""SD-43 E3.9 figures: the Background and Misc categories the engine reads, and what the converted package
holds for them. Run from the repo root: `python3 -I docs/release/SD-43-market-inventory-and-abilities/artifacts/epic_3/E3.9/figures.py`.

Predicates (the engine's, `src/rules_core/ability_budget.rs`):
- a category's *grant rules* are package rules whose target is `Pool(<category pool>)`;
- a category's *members* are package rules (no `#` sibling line, non-empty label) whose `pool` is the category's
  `member_pool` and that carry one of its `member_tags` (case-insensitive), or any record of it when it has none;
  counted once per (category, label), as the options list them.
"""
import collections
import glob
import json

cats = json.load(open("data/ability_categories.json"))["categories"]
grants = collections.defaultdict(list)
by_pool = collections.defaultdict(list)
for f in glob.glob("data/sheet_rules/*/*/*.json"):
    try:
        doc = json.load(open(f))
    except Exception:
        continue
    for r in doc if isinstance(doc, list) else []:
        if not isinstance(r, dict):
            continue
        t = r.get("target")
        if isinstance(t, dict) and "Pool" in t:
            grants[t["Pool"]].append(r)
        if "#" not in r["id"] and r.get("label"):
            by_pool[r.get("pool") or ""].append(r)

for tab in ("background", "misc"):
    cs = [c for c in cats if c["subtab"] == tab]
    members = {}
    for c in cs:
        tags = {t.lower() for t in c["member_tags"]}
        labels = {r["label"] for r in by_pool[c["member_pool"]] if not tags or tags & {t.lower() for t in r.get("tags") or []}}
        members[c["pool"]] = len(labels)
    print(f"{tab}: categories {len(cs)}; editable pool {sum(c['editable_pool'] for c in cs)}; "
          f"with package grant rules {sum(1 for c in cs if grants[c['pool']])}; "
          f"with an always-held grant {sum(1 for c in cs if any(r.get('always_held') for r in grants[c['pool']]))}; "
          f"with members {sum(1 for n in members.values() if n)}; member options {sum(members.values())}; "
          f"no grant rule and not editable {sorted(c['pool'] for c in cs if not grants[c['pool']] and not c['editable_pool'])}; "
          f"no members {sorted(p for p, n in members.items() if not n)}")
