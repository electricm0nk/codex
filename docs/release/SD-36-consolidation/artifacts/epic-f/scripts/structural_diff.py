#!/usr/bin/env python3
"""Structural diff for the F1 converter change (epic-f-class-completion.md SS3.5, F1.6).

F1 adversarial finding 4: no structural-diff script existed on the branch, so the converter's
--check (which compares byte-for-byte and fails whole on the first difference) could not state
what actually moved between the on-disk baseline (data/sheet_rules/) and a fresh conversion --
only that something did. This script answers that question without ever writing to
data/sheet_rules itself: it reads the tracked baseline plus a SCRATCH directory a caller already
produced with

    cargo run --locked -j <n> -p codex-ingest --bin sheet_rule_convert -- --dump <scratch dir>

and reports:
  - the file-set delta (added / removed paths), split into rule files vs `_vars/*.json` vs other;
  - for every rule id present on BOTH sides, which top-level fields changed -- the diff contract
    (SS1 step 4 / SS3.5) allows only `granted_by` and `grants` to grow; anything else changing is
    named as an "unexpected field delta";
  - added `granted_by` EDGES, grouped by the TARGET rule's kind (the per-kind table SS3.1/F1.6
    ask the diff to reconcile against);
  - an explicit summary line for the record/rule/var_table counts so F1.6's acceptance command
    has one thing to read instead of re-deriving it by hand.

Usage:
    python3 structural_diff.py <scratch_dump_dir> [--baseline data/sheet_rules] [--max-examples N]

Exit code is 0 always (it is a REPORT, not a gate); pipe its "unexpected field deltas" line into
a gate command if F1.6 wants a hard failure.
"""
from __future__ import annotations

import argparse
import glob
import json
import os
import sys
from collections import Counter, defaultdict

ALLOWED_GROWING_FIELDS = {"granted_by", "grants"}


def load_tree(root: str) -> dict[str, object]:
    """Every rule file under `root` (`<book>/<kind>/<key>.json`) as `{relpath: [rule dict, ...]}`.
    `_vars/`, `_defects/`, `_report.json`, et. are walked too but kept as opaque byte-diff targets
    (they are census/side-channel files, not the rule-id-keyed package the diff contract governs).
    """
    rule_files: dict[str, list] = {}
    other_files: dict[str, bytes] = {}
    for dirpath, _dirnames, filenames in os.walk(root):
        for fn in filenames:
            if fn in ("GENERATED",):
                continue
            full = os.path.join(dirpath, fn)
            rel = os.path.relpath(full, root).replace(os.sep, "/")
            if rel.startswith("_vars/") or rel.startswith("_defects/") or rel in ("_refused.json", "_tokens.json", "_report.json"):
                with open(full, "rb") as fh:
                    other_files[rel] = fh.read()
                continue
            try:
                with open(full, "r", encoding="utf-8") as fh:
                    data = json.load(fh)
            except (OSError, json.JSONDecodeError):
                continue
            if isinstance(data, list):
                rule_files[rel] = data
    return {"rule_files": rule_files, "other_files": other_files}


def rules_by_id(rule_files: dict[str, list]) -> dict[str, dict]:
    out: dict[str, dict] = {}
    for rules in rule_files.values():
        for r in rules:
            if isinstance(r, dict) and "id" in r:
                out[r["id"]] = r
    return out


def kind_of(rule_id: str) -> str:
    parts = rule_id.split(":")
    return parts[1] if len(parts) >= 2 else "?"


def diff_rule(old: dict, new: dict) -> list[str]:
    """Field names that differ between the two rule dicts (shallow -- SS3.5 wants a NAMED field,
    not a value diff; a deep JSON diff is a separate, noisier tool)."""
    keys = set(old.keys()) | set(new.keys())
    return sorted(k for k in keys if old.get(k) != new.get(k))


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("scratch_dir", help="a directory `sheet_rule_convert --dump <dir>` wrote (never data/sheet_rules)")
    ap.add_argument("--baseline", default="data/sheet_rules", help="the tracked on-disk package (default: data/sheet_rules)")
    ap.add_argument("--max-examples", type=int, default=10)
    args = ap.parse_args()

    baseline_root = os.path.abspath(args.baseline)
    scratch_root = os.path.abspath(args.scratch_dir)
    if baseline_root == scratch_root:
        print("refusing: scratch_dir must not be the tracked baseline itself", file=sys.stderr)
        return 2

    base = load_tree(baseline_root)
    fresh = load_tree(scratch_root)

    base_paths = set(base["rule_files"]) | set(base["other_files"])
    fresh_paths = set(fresh["rule_files"]) | set(fresh["other_files"])
    added_paths = sorted(fresh_paths - base_paths)
    removed_paths = sorted(base_paths - fresh_paths)

    added_vars = [p for p in added_paths if p.startswith("_vars/")]
    added_defects = [p for p in added_paths if p.startswith("_defects/")]
    added_rule_files = [p for p in added_paths if p in fresh["rule_files"]]
    added_other = [p for p in added_paths if p not in added_vars and p not in added_defects and p not in added_rule_files]
    removed_vars = [p for p in removed_paths if p.startswith("_vars/")]
    removed_defects = [p for p in removed_paths if p.startswith("_defects/")]
    removed_rule_files = [p for p in removed_paths if p in base["rule_files"]]
    removed_other = [p for p in removed_paths if p not in removed_vars and p not in removed_defects and p not in removed_rule_files]

    base_rules = rules_by_id(base["rule_files"])
    fresh_rules = rules_by_id(fresh["rule_files"])
    base_ids = set(base_rules)
    fresh_ids = set(fresh_rules)
    added_rule_ids = sorted(fresh_ids - base_ids)
    removed_rule_ids = sorted(base_ids - fresh_ids)

    unexpected_field_deltas: list[tuple[str, str]] = []
    added_edges_by_target_kind: Counter[str] = Counter()
    added_edges_total = 0
    for rid in sorted(base_ids & fresh_ids):
        old, new = base_rules[rid], fresh_rules[rid]
        changed = diff_rule(old, new)
        for field in changed:
            if field not in ALLOWED_GROWING_FIELDS:
                unexpected_field_deltas.append((rid, field))
        old_edges = len(old.get("granted_by") or [])
        new_edges = len(new.get("granted_by") or [])
        if new_edges > old_edges:
            added_edges_by_target_kind[kind_of(rid)] += new_edges - old_edges
            added_edges_total += new_edges - old_edges

    base_report = json.loads(base["other_files"].get("_report.json", b"{}") or b"{}")
    fresh_report = json.loads(fresh["other_files"].get("_report.json", b"{}") or b"{}")

    print("== file set ==")
    print(f"  rule files:  +{len(added_rule_files)} -{len(removed_rule_files)}  (+{len(added_rule_ids)} -{len(removed_rule_ids)} rule ids)")
    print(f"  _vars/:      +{len(added_vars)} -{len(removed_vars)}")
    print(f"  _defects/:   +{len(added_defects)} -{len(removed_defects)}")
    print(f"  other:       +{len(added_other)} -{len(removed_other)}  {added_other + removed_other}")
    print()
    print("== counts (baseline -> fresh) ==")
    for key in ("records", "converted", "refused", "rules_written", "var_tables"):
        print(f"  {key}: {base_report.get(key)} -> {fresh_report.get(key)}")
    print()
    print("== added granted_by edges by TARGET kind ==")
    for kind, n in sorted(added_edges_by_target_kind.items(), key=lambda kv: (-kv[1], kv[0])):
        print(f"  {kind}: {n}")
    print(f"  TOTAL: {added_edges_total}")
    print()
    print(f"unexpected field deltas: {len(unexpected_field_deltas)}")
    for rid, field in unexpected_field_deltas[: args.max_examples]:
        print(f"  {rid}: {field}")
    if len(unexpected_field_deltas) > args.max_examples:
        print(f"  ... and {len(unexpected_field_deltas) - args.max_examples} more")
    if added_rule_ids:
        print(f"new rule ids: {len(added_rule_ids)} {added_rule_ids[: args.max_examples]}")
    if removed_rule_ids:
        print(f"removed rule ids: {len(removed_rule_ids)} {removed_rule_ids[: args.max_examples]}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
