#!/usr/bin/env python3
"""SD-37 E4.4: classify the SF package delta of the three converter changes.

usage: E4.4_delta.py <new package dir> <baseline package dir>

Every differing file must fall in one of these classes, else it is printed and the exit is 1:

  class_spell_progression  a class file whose every baseline rule is unchanged and whose only
                           additions are `#spells_per_day_<n>` / `#spells_known_<n>` rules after
                           them, each targeting that class's `SpellCell` / `SpellsKnown`
                           (CV1, the `CAST:` / `KNOWN:` level lines lowered)
  connection_spell_hop     a rule file whose baseline rules are unchanged (the principal may gain
                           only `closure_complete`, its unresolved Internal reference no longer a
                           defect) and whose only additions are `#spell_known_*` rules after them,
                           each a `SpellsKnown` row worth 1 (CV2, the connection-spell hop)
  cl_is_character_level    a file of one of the records whose only degradation was
                           `FORMULA:CL-no-owner` in the baseline `_tokens.json` (CV3; printed
                           rule by rule, old -> new)
  var_table_added          a new `_vars/` table referenced only by a `#spell_known_*` gate
  bookkeeping              `_report.json`, `_tokens.json`, `_refused.json`, `GENERATED`,
                           `var_names.json`, `_defects/*.json` (their counts are printed)

Prints one count per class (denominator: files differing between the two trees).
"""
import filecmp
import json
import os
import sys
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
cl_records = {e["id"] for e in load(old, "_tokens.json")["entries"] if set((e.get("degradations") or {})) == {"FORMULA:CL-no-owner"}}
hop_vars = set()
for d, _, ns in os.walk(new):
    for n in ns:
        if n.endswith(".json") and not d.endswith("_vars") and "_defects" not in d:
            p = os.path.join(d, n)
            try:
                rules = json.load(open(p))
            except Exception:
                continue
            if isinstance(rules, list):
                for r in rules:
                    if isinstance(r, dict) and "#spell_known_" in r.get("id", ""):
                        s = json.dumps(r.get("applies"))
                        hop_vars.update(x.split('"')[0] for x in s.split('{"Var": "')[1:])

classes, bad, detail = Counter(), [], []
defect_delta = {}
fn, fo = files(new), files(old)
differing = 0
added_rules = Counter()
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
        if rel not in fo and os.path.basename(rel)[:-5] in hop_vars:
            classes["var_table_added"] += 1
            detail.append(f"var table added {rel}: {load(new, rel).get('label')}")
        else:
            bad.append(f"var moved: {rel}")
        continue
    if rel not in fo or rel not in fn:
        bad.append(f"file added or removed: {rel}")
        continue
    a, b = load(new, rel), load(old, rel)
    if a and a[0]["id"] in cl_records:
        classes["cl_is_character_level"] += 1
        for ra in a:
            rb = next((x for x in b if x["id"] == ra["id"]), None)
            detail.append(f"{rel} {ra['id']}: {json.dumps(rb.get('target') if rb else None)} {json.dumps(rb.get('value') if rb else None)[:80]} -> {json.dumps(ra.get('target'))} {json.dumps(ra.get('value'))[:160]}")
        continue
    if [r["id"] for r in a[: len(b)]] != [r["id"] for r in b]:
        bad.append(f"baseline rule ids moved: {rel}")
        continue
    moved = False
    for i, (ra, rb) in enumerate(zip(a, b)):
        if ra == rb:
            continue
        only_closure = i == 0 and {k: v for k, v in ra.items() if k != "closure_complete"} == {k: v for k, v in rb.items() if k != "closure_complete"}
        if not only_closure:
            moved = True
    extra = a[len(b):]
    if moved or not extra:
        bad.append(f"moved: {rel}")
        continue
    if "/class/" in "/" + rel.replace(os.sep, "/") and all(
        ("#spells_per_day_" in r["id"] and "SpellCell" in (r.get("target") or {}))
        or ("#spells_known_" in r["id"] and "SpellsKnown" in (r.get("target") or {}))
        for r in extra
    ):
        classes["class_spell_progression"] += 1
        added_rules["class_spell_progression"] += len(extra)
        continue
    if all("#spell_known_" in r["id"] and "SpellsKnown" in (r.get("target") or {}) and r["value"] == {"Number": {"Const": 1}} for r in extra):
        classes["connection_spell_hop"] += 1
        added_rules["connection_spell_hop"] += len(extra)
        continue
    bad.append(f"unclassified additions: {rel}")

print(f"differing_files={differing} " + " ".join(f"{k}={v}" for k, v in sorted(classes.items())) + f" other_moves={len(bad)}")
print("rules added: " + " ".join(f"{k}={v}" for k, v in sorted(added_rules.items())))
print(f"baseline records whose only degradation was FORMULA:CL-no-owner: {len(cl_records)} {sorted(cl_records)}")
for x in detail:
    print("  " + x)
for rel, (b, a) in sorted(defect_delta.items()):
    print(f"  defects {rel}: {b} -> {a}")
for x in bad:
    print("  BAD", x)
sys.exit(1 if bad else 0)
