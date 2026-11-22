#!/usr/bin/env python3
"""SD-37 E4.2: classify the SF package delta of the three converter changes.

usage: E4.2_delta.py <new package dir> <baseline package dir>

Every differing file must fall in one of these classes, else it is printed and the exit is 1:

  hop_granted_by     a rule file whose only change is added `granted_by` edges
                     `{by: {Rule: <granter>}}` (CV1, the Internal selection hop: a race's
                     Race-category ability now grants its default racial traits)
  pool_option_added  a new `<book>/pool_option/<pool>_<name>.json` whose principal has pool
                     `internal`, at least one tag and no `granted_by` (CV3, an Internal-parented
                     ABILITYPOOL member)
  pool_offers_linked a rule file whose only change is an added `offers` of
                     `Rules {pool: internal}` on a rule that had none (CV3: the pick now offers
                     its members)
  var_declared       a `_vars/` table whose only change is `declared_by` going from [] to its
                     contributors' ids (CV2, the zero DEFINE on the unconverted global Default)
  var_contrib_added  a `_vars/` table whose change is contributions added from `pool_option`
                     rules (and, with CV2, the matching declared_by)
  var_table_added    a new `_vars/` table referenced by a pool option
  pool_option_ref    a rule file whose changed rules now name a pool option (a reference to an
                     Internal row that was `MissingRule`, a prose line gated on holding one), or
                     gain only `closure_complete` because such a reference stopped being a
                     defect (CV3's index registration)
  bookkeeping        `_report.json`, `_tokens.json`, `_refused.json`, `GENERATED`,
                     `var_names.json`, `_defects/*.json` (their counts are printed)

Prints one count per class (denominator: files differing between the two trees).
"""
import json, os, sys, filecmp
from collections import Counter

new, old = sys.argv[1], sys.argv[2]


def files(root):
    out = set()
    for d, _, ns in os.walk(root):
        for n in ns:
            out.add(os.path.relpath(os.path.join(d, n), root))
    return out


def load(root, rel):
    with open(os.path.join(root, rel)) as f:
        return json.load(f)


BOOKKEEPING = {"_report.json", "_tokens.json", "_refused.json", "GENERATED", "var_names.json"}
classes, bad = Counter(), []
granters, offers_pools, defect_delta = Counter(), Counter(), {}
fn, fo = files(new), files(old)
differing = 0
for rel in sorted(fn | fo):
    if rel in fn and rel in fo and filecmp.cmp(os.path.join(new, rel), os.path.join(old, rel), shallow=False):
        continue
    differing += 1
    if rel in BOOKKEEPING:
        classes["bookkeeping"] += 1
        continue
    if rel.startswith("_defects" + os.sep):
        a = load(new, rel) if rel in fn else []
        b = load(old, rel) if rel in fo else []
        defect_delta[rel] = (len(b), len(a))
        classes["bookkeeping"] += 1
        continue
    if rel.startswith("_vars" + os.sep):
        if rel not in fo:
            classes["var_table_added"] += 1
            continue
        if rel not in fn:
            bad.append(f"var table removed: {rel}")
            continue
        a, b = load(new, rel), load(old, rel)
        a_rest = {k: v for k, v in a.items() if k not in ("declared_by", "contributions", "provenance")}
        b_rest = {k: v for k, v in b.items() if k not in ("declared_by", "contributions", "provenance")}
        # A row a pool option now owns leaves the outside-corpus provenance; nothing is added.
        if not set(a["provenance"].get("outside_corpus_rows", [])) <= set(b["provenance"].get("outside_corpus_rows", [])):
            bad.append(f"var provenance grew: {rel}")
            continue
        old_c, new_c = b["contributions"], a["contributions"]
        added_c = [c for c in new_c if c not in old_c]
        if a_rest != b_rest or any(c not in new_c for c in old_c):
            bad.append(f"var moved: {rel}")
            continue
        contributors = sorted({c["rule_id"] for c in new_c})
        if added_c:
            if not all(":pool_option:" in c["rule_id"] for c in added_c):
                bad.append(f"var contribution from a non-pool-option rule: {rel}")
                continue
            if a["declared_by"] != b["declared_by"] and not (b["declared_by"] == [] and a["declared_by"] == contributors):
                bad.append(f"var declared_by moved: {rel}")
                continue
            classes["var_contrib_added"] += 1
            continue
        if b["declared_by"] == [] and a["declared_by"] == contributors and contributors:
            classes["var_declared"] += 1
            continue
        bad.append(f"var moved: {rel}")
        continue
    if rel not in fo:
        if os.sep + "pool_option" + os.sep not in rel:
            bad.append(f"new non-pool-option file: {rel}")
            continue
        rules = load(new, rel)
        p = rules[0]
        if p.get("pool") != "internal" or not p.get("tags") or p.get("granted_by"):
            bad.append(f"pool option shape: {rel}")
            continue
        classes["pool_option_added"] += 1
        continue
    if rel not in fn:
        bad.append(f"file removed: {rel}")
        continue
    a, b = load(new, rel), load(old, rel)
    if [r["id"] for r in a] != [r["id"] for r in b]:
        bad.append(f"rule ids moved: {rel}")
        continue
    kind = None
    for ra, rb in zip(a, b):
        if ra == rb:
            continue
        # CV3 consequence: the rule now names a pool option, or only gained `closure_complete`.
        only_closure = {k: v for k, v in ra.items() if k != "closure_complete"} == {k: v for k, v in rb.items() if k != "closure_complete"}
        if only_closure or (":pool_option:" in json.dumps(ra) and ":pool_option:" not in json.dumps(rb)
                            and {k: v for k, v in ra.items() if k in ("id", "label", "target", "bonus_type", "tags", "pool", "provenance")}
                            == {k: v for k, v in rb.items() if k in ("id", "label", "target", "bonus_type", "tags", "pool", "provenance")}):
            kind = "pool_option_ref" if kind in (None, "pool_option_ref") else kind
            continue
        rest_a = {k: v for k, v in ra.items() if k not in ("granted_by", "offers")}
        rest_b = {k: v for k, v in rb.items() if k not in ("granted_by", "offers")}
        if rest_a != rest_b:
            kind = "bad"
            break
        ga, gb = ra.get("granted_by") or [], rb.get("granted_by") or []
        if ga != gb:
            extra = [g for g in ga if g not in gb]
            if any(g not in ga for g in gb) or not extra or not all("Rule" in g["by"] for g in extra):
                kind = "bad"
                break
            for g in extra:
                granters[g["by"]["Rule"]] += 1
            kind = "hop_granted_by" if kind in (None, "hop_granted_by", "pool_option_ref") else "bad"
        if ra.get("offers") != rb.get("offers"):
            o = ra.get("offers") or {}
            if rb.get("offers") is not None or o.get("from", {}).get("Rules", {}).get("pool") != "internal":
                kind = "bad"
                break
            offers_pools[o["id"]] += 1
            kind = "pool_offers_linked" if kind in (None, "pool_offers_linked", "pool_option_ref") else "bad"
    if kind in (None, "bad"):
        bad.append(f"moved: {rel}")
        continue
    classes[kind] += 1

print(f"differing_files={differing} " + " ".join(f"{k}={v}" for k, v in sorted(classes.items())) + f" other_moves={len(bad)}")
print(f"hop granters={len(granters)} hop edges={sum(granters.values())}")
for g, n in sorted(granters.items()):
    print(f"  granter {g}: {n}")
print(f"picks now offering an Internal pool: {len(offers_pools)}")
for o in sorted(offers_pools):
    print(f"  offers {o}")
for rel, (b, a) in sorted(defect_delta.items()):
    print(f"  defects {rel}: {b} -> {a}")
for x in bad:
    print("  BAD", x)
sys.exit(1 if bad else 0)
