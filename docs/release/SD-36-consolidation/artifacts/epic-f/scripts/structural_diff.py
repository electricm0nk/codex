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
  - `granted_by` diffed as a SET of edges, never by length: an ADDED edge is the only change
    SS3.5 allows; a REMOVED one gates the run (F1 re-check round 1, finding 2: the previous
    length-diff missed same-length replacements entirely, and a pass-through allow-list accepted
    any change including deletion);
  - `grants` diffed by TARGET SIGNATURE, not raw JSON equality: F1 itself ships two sanctioned
    in-place transformations of an existing grant's WRAPPER or REPRESENTATION with its target
    held fixed -- F1-2 wraps a bare `FactGrant` in a `GatedFactGrant` around the SAME fact when a
    PRE-gate applies, and F1-3 expands a bare `Proficiency::WeaponGroup`/`WeaponTag` tag into an
    equivalent `WeaponSet`/`WeaponAllOf` naming the SAME tag (case-insensitive) with a resolved
    member list. Neither changes what is granted, only how; a raw-JSON diff cannot tell either
    apart from a removal, and flagged both as violations on the real corpus (F1 re-check round 2
    fix). `grant_signature()` reduces a grant to `(fact kind, target)`, gating-wrapper-agnostic
    and tag-case-insensitive for `Proficiency`; a signature present on the baseline side and
    ABSENT (by count) on the fresh side is a genuine loss and gates the run -- fabricated content
    (a different target under the same or a different shape) still gates, since its signature
    differs from anything the baseline held;
  - added `granted_by` EDGES, grouped by the TARGET rule's kind (the per-kind table SS3.1/F1.6
    ask the diff to reconcile against);
  - an explicit summary line for the record/rule/var_table counts so F1.6's acceptance command
    has one thing to read instead of re-deriving it by hand -- and a GATE failure, not just a
    printed pair, when `records` or `converted` moves against the baseline `_report.json` (F1
    re-check round 1, finding 2: a moved count previously printed but was never flagged).

Usage:
    python3 structural_diff.py <scratch_dump_dir> [--baseline data/sheet_rules] [--max-examples N] [--report-only]

Exit code: 1 when any of removed rule ids, unexpected field deltas, removed `granted_by` edges,
removed `grants`, or a moved `records`/`converted` count is non-zero -- the gate F1.6 reads. Pass
--report-only to keep the old report-and-exit-0 behavior for exploration.
"""
from __future__ import annotations

import argparse
import glob
import json
import os
import sys
from collections import Counter, defaultdict

ALLOWED_GROWING_FIELDS = {"granted_by", "grants"}


def edge_diff(old_list: object, new_list: object) -> tuple[list[str], list[str]]:
    """Diff two `granted_by`/`grants` lists as SETS of edges (each edge serialized to a stable
    JSON key), never by length or shallow equality. Returns `(removed, added)` -- both sorted --
    where `removed` is the gate-relevant half: an edge/grant present on the baseline side and
    absent on the fresh side, whether it was dropped outright or its content was replaced by a
    different value (a replacement is a removal of the old key plus an addition of the new one,
    and the removal half is what SS3.5 never allows)."""
    old_keys = {json.dumps(e, sort_keys=True) for e in (old_list or [])}
    new_keys = {json.dumps(e, sort_keys=True) for e in (new_list or [])}
    return sorted(old_keys - new_keys), sorted(new_keys - old_keys)


def _fact_of(effect: object) -> object:
    """The `Fact` a `grants` entry (an `Effect`) carries, unwrapping `GatedFactGrant` -- the
    gating wrapper F1-2 adds is not part of what was granted, only when."""
    if not isinstance(effect, dict):
        return None
    if "FactGrant" in effect:
        return effect["FactGrant"]
    gated = effect.get("GatedFactGrant")
    if isinstance(gated, dict):
        return gated.get("fact")
    return None


def grant_signature(effect: object) -> tuple:
    """A `grants` entry reduced to `(fact kind, target)` -- gating-wrapper-agnostic (F1-2:
    `FactGrant` vs `GatedFactGrant` around the same fact is the SAME signature) and, for a
    `Proficiency` fact, tag-case-insensitive with `WeaponGroup`/`WeaponTag`/`WeaponAllOf`/
    `WeaponSet` all folded to the same `("prof_tag", <lowercased tag text>)` shape (F1-3: a bare
    tag expanding to a `WeaponSet`/`WeaponAllOf` naming the identical tag is the SAME signature).
    Anything this function does not recognize (a non-`Proficiency` fact, a `Weapon`/`ArmorGroup`/
    `ShieldGroup`/`Chosen`/`DeityFavoredWeapon` proficiency, or a non-fact `Effect` such as
    `FactDeclare`) falls back to an exact serialized match -- unrecognized shapes get no
    transformation leniency, only the two named above do."""
    fact = _fact_of(effect)
    if not isinstance(fact, dict):
        return ("raw", json.dumps(effect, sort_keys=True))
    prof = fact.get("Proficiency") if isinstance(fact, dict) else None
    if isinstance(prof, dict):
        if "WeaponGroup" in prof and isinstance(prof["WeaponGroup"], str):
            return ("prof_tag", prof["WeaponGroup"].lower())
        if "WeaponTag" in prof and isinstance(prof["WeaponTag"], str):
            return ("prof_tag", prof["WeaponTag"].lower())
        if "WeaponAllOf" in prof and isinstance(prof["WeaponAllOf"], list):
            return ("prof_tag", ".".join(str(t) for t in prof["WeaponAllOf"]).lower())
        if "WeaponSet" in prof and isinstance(prof["WeaponSet"], dict):
            return ("prof_tag", str(prof["WeaponSet"].get("label", "")).lower())
        return ("prof_other", json.dumps(prof, sort_keys=True))
    return ("fact", json.dumps(fact, sort_keys=True))


def missing_grant_signatures(old_list: object, new_list: object) -> list[tuple[tuple, int]]:
    """Every grant SIGNATURE present on the baseline side more times than on the fresh side, as
    `(signature, missing_count)` -- a multiset (`Counter`) comparison, not a set one, so a grant
    dropped from a rule that still carries an unrelated grant of the matching signature is still
    named (F1 re-check round 1, finding 2's own gate contract: a genuine loss must never hide
    behind an untouched sibling grant)."""
    old_sigs = Counter(grant_signature(e) for e in (old_list or []))
    new_sigs = Counter(grant_signature(e) for e in (new_list or []))
    missing = old_sigs - new_sigs
    return sorted(missing.items())


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
    """Field names OTHER than the two allowed-to-grow fields that differ between the two rule
    dicts (shallow -- SS3.5 wants a NAMED field, not a value diff; a deep JSON diff is a separate,
    noisier tool). `granted_by`/`grants` are excluded here -- they get their own set-based
    `edge_diff` treatment below, since a length-preserving replacement inside either field would
    otherwise pass this shallow-inequality check silently only when nothing else changed, or be
    lumped in as an "unexpected field delta" when something else did, neither of which names the
    edge/grant that was actually removed."""
    keys = (set(old.keys()) | set(new.keys())) - ALLOWED_GROWING_FIELDS
    return sorted(k for k in keys if old.get(k) != new.get(k))


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("scratch_dir", help="a directory `sheet_rule_convert --dump <dir>` wrote (never data/sheet_rules)")
    ap.add_argument("--baseline", default="data/sheet_rules", help="the tracked on-disk package (default: data/sheet_rules)")
    ap.add_argument("--max-examples", type=int, default=10)
    ap.add_argument(
        "--report-only",
        action="store_true",
        help="keep the pre-gate behavior: always print the report and exit 0, for exploration",
    )
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
    removed_granted_by: list[tuple[str, str]] = []
    removed_grants: list[tuple[str, tuple, int]] = []
    added_grants_total = 0
    for rid in sorted(base_ids & fresh_ids):
        old, new = base_rules[rid], fresh_rules[rid]
        for field in diff_rule(old, new):
            unexpected_field_deltas.append((rid, field))

        edges_removed, edges_added = edge_diff(old.get("granted_by"), new.get("granted_by"))
        for e in edges_removed:
            removed_granted_by.append((rid, e))
        if edges_added:
            added_edges_by_target_kind[kind_of(rid)] += len(edges_added)
            added_edges_total += len(edges_added)

        for sig, missing_n in missing_grant_signatures(old.get("grants"), new.get("grants")):
            removed_grants.append((rid, sig, missing_n))
        _, grants_added_raw = edge_diff(old.get("grants"), new.get("grants"))
        added_grants_total += len(grants_added_raw)

    base_report = json.loads(base["other_files"].get("_report.json", b"{}") or b"{}")
    fresh_report = json.loads(fresh["other_files"].get("_report.json", b"{}") or b"{}")
    moved_counts = [
        (key, base_report.get(key), fresh_report.get(key))
        for key in ("records", "converted")
        if base_report.get(key) != fresh_report.get(key)
    ]

    print("== file set ==")
    print(f"  rule files:  +{len(added_rule_files)} -{len(removed_rule_files)}  (+{len(added_rule_ids)} -{len(removed_rule_ids)} rule ids)")
    print(f"  _vars/:      +{len(added_vars)} -{len(removed_vars)}")
    print(f"  _defects/:   +{len(added_defects)} -{len(removed_defects)}")
    print(f"  other:       +{len(added_other)} -{len(removed_other)}  {added_other + removed_other}")
    print()
    print("== counts (baseline -> fresh) ==")
    for key in ("records", "converted", "refused", "rules_written", "var_tables"):
        marker = "  *** MOVED ***" if key in ("records", "converted") and base_report.get(key) != fresh_report.get(key) else ""
        print(f"  {key}: {base_report.get(key)} -> {fresh_report.get(key)}{marker}")
    print()
    print("== added granted_by edges by TARGET kind ==")
    for kind, n in sorted(added_edges_by_target_kind.items(), key=lambda kv: (-kv[1], kv[0])):
        print(f"  {kind}: {n}")
    print(f"  TOTAL: {added_edges_total}")
    print(f"  added grants (total): {added_grants_total}")
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
    print()
    print(f"removed granted_by edges: {len(removed_granted_by)}  (never allowed -- SS3.5 permits growth only)")
    for rid, e in removed_granted_by[: args.max_examples]:
        print(f"  {rid}: {e}")
    if len(removed_granted_by) > args.max_examples:
        print(f"  ... and {len(removed_granted_by) - args.max_examples} more")
    print(f"removed grants: {len(removed_grants)}  (a grant SIGNATURE lost with no replacement of the same target -- gating-wrapper and WeaponGroup/WeaponTag/WeaponAllOf/WeaponSet-tag-case changes are NOT counted here, only a genuine loss or a fabricated replacement)")
    for rid, sig, n in removed_grants[: args.max_examples]:
        print(f"  {rid}: {sig} x{n}")
    if len(removed_grants) > args.max_examples:
        print(f"  ... and {len(removed_grants) - args.max_examples} more")

    failures: list[str] = []
    if removed_rule_ids:
        failures.append(f"removed rule ids: {len(removed_rule_ids)}")
    if unexpected_field_deltas:
        failures.append(f"unexpected field deltas: {len(unexpected_field_deltas)}")
    if removed_granted_by:
        failures.append(f"removed granted_by edges: {len(removed_granted_by)}")
    if removed_grants:
        failures.append(f"removed grants: {len(removed_grants)}")
    for key, old_v, new_v in moved_counts:
        failures.append(f"{key} moved: {old_v} -> {new_v}")

    print()
    if failures:
        print(f"verdict=FAIL ({'; '.join(failures)})")
    else:
        print("verdict=PASS")

    if args.report_only:
        return 0
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
