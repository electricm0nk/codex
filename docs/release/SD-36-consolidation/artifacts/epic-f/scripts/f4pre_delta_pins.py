#!/usr/bin/env python3
"""Enumerate and pin the F4pre package deltas (SD-36 Epic F4pre, converter step: a selection pool
the oracle fills becomes a converted choice -- `sheet_rule/pool_link.rs::link_pool_choices`).

Every delta must be an `offers` field ADDED to a rule that had none, and must be exactly the rule:
  - a `BONUS:ABILITYPOOL` pick (`target: {"Pool": p}`): `{"id": <rule id>, "count": <the rule's
    value Number>, "from": {"Rules": {"pool": <parent>, "tags": [...], "requires": "Always"}}}`;
  - a `BONUS:DOMAIN|NUMBER` line (`target: {"Other": "domains"}`): `{"id": <rule id>, "count":
    <value Number>, "from": "Domains"}`.
Pinned by sha256 of the new `offers` value (`structural_diff.f3b2_field_sha`), with its kind.
Any other delta -- a file or rule id added or removed, any other field, a side file -- is printed
and nothing is written.

Usage (from the repo root):
    python3 f4pre_delta_pins.py <baseline package dir> <fresh package dir> <out json>
The baseline is the package BEFORE F4pre (sd36/epic-f4-f5 at the F3p commit 6db56623e8); the
structural diff itself still runs against tranche/16 and undoes these pins first (`f4pre_apply`).
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
        for rel in sorted(set(base_tree[kind]) & set(fresh_tree[kind])):
            if kind == "other_files" and base_tree[kind][rel] != fresh_tree[kind][rel]:
                unexplained.append(f"side file moved: {rel}")
    base = sd.rules_by_id(base_tree["rule_files"])
    fresh = sd.rules_by_id(fresh_tree["rule_files"])
    for rid in sorted(set(base) ^ set(fresh)):
        unexplained.append(f"rule id {'added' if rid in fresh else 'removed'}: {rid}")
    pins: list = []
    by_kind: Counter = Counter()
    by_pool: Counter = Counter()
    for rid in sorted(set(base) & set(fresh)):
        for field in sorted(set(base[rid]) | set(fresh[rid])):
            old, new = base[rid].get(field), fresh[rid].get(field)
            if old == new:
                continue
            kind = sd.f4pre_offer_kind(fresh[rid]) if field == "offers" and old is None else None
            if kind is None:
                unexplained.append(f"field moved beyond an F4pre offer: {rid}: {field}")
                continue
            pins.append([rid, sd.f3b2_field_sha(new), kind])
            by_kind[kind] += 1
            if kind == "ability_pool":
                by_pool[new["from"]["Rules"]["pool"]] += 1
    if unexplained:
        for line in unexplained[:200]:
            print(line)
        print(f"{len(unexplained)} unexplained -- nothing written")
        return 1
    data = {
        "_purpose": "SD-36 Epic F4pre (FS-21): a selection pool the oracle fills (BONUS:ABILITYPOOL into a TYPE-filtered child category with a converted member; BONUS:DOMAIN|NUMBER) becomes a converted choice -- an `offers` field added to the pick, exactly the pick's own count over the category's members / the domains; structural_diff.py gates on each (f4pre_apply).",
        "_command": "python3 f4pre_delta_pins.py <package at 6db56623e8, git archive> <fresh package dir> structural_diff_f4pre_deltas.json",
        "owner": "core_rulebook:class:cleric#bonus1",
        "by_kind": dict(sorted(by_kind.items())),
        "ability_pools_by_parent": dict(sorted(by_pool.items())),
        "offers": {"_count": len(pins), "pins": pins},
    }
    with open(out, "w", encoding="utf-8") as fh:
        json.dump(data, fh, indent=1)
        fh.write("\n")
    print(f"pinned {len(pins)} offers: {dict(by_kind)}; ability pools by parent {dict(by_pool)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
