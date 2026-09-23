#!/usr/bin/env python3
"""SD-36 Epic F1c render receipt: how render_sheet now gates every D2 sibling.

  f1c_d2_line_gate_scan.py [<sheet_rules dir>]   (default: data/sheet_rules)

For each of the D2 siblings in structural_diff_f1c_deltas.json `d2_splits` (principal id ->
sibling id), applies render_sheet's `line_gate` (the sibling's `applies` minus the decided
terms of its principal's `applies`, kept only when the sibling carries the whole copy) and
buckets the resulting LINE gate:
  unconditional   Always / only Situational terms: prints whenever held, as tranche/16 printed
                  the principal line
  undecided_leaf  still contains a Holds leaf over a fact the character record does not carry
                  (alignment, deity, class skill, language, gender, age category): prints with
                  that condition in words, never dropped
  decided         only facts the character record carries: the line's own condition decides it
Also counts, over the same siblings, the leaves the ORIGINAL (unstripped) gate reads, which is
what the F1c merge-readiness check counted (class skill 103, alignment 29, deity 12, language 5).
"""
import collections, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
UNCARRIED = {"Alignment", "AlignmentMatchesDeity", "Deity", "DeityInPantheon", "DeityGrantsDomain",
             "DeityAlignment", "ClassSkill", "Language", "Gender", "AgeCategory"}


def rule_index(root):
    idx = {}
    for dp, _, fs in os.walk(root):
        if "/_" in dp or dp.endswith("_vars"):
            continue
        for f in fs:
            if f.endswith(".json") and not f.startswith("_"):
                try:
                    for r in json.load(open(os.path.join(dp, f))):
                        idx[r["id"]] = r
                except Exception:
                    pass
    return idx


def terms(a):
    if a == "Always":
        return []
    if isinstance(a, dict) and "All" in a:
        return a["All"]
    return [a]


def is_situational(t):
    return isinstance(t, dict) and "Situational" in t


def line_gate(principal, sibling):
    copied = [t for t in terms(principal["applies"]) if not is_situational(t)]
    own = terms(sibling["applies"])
    if not copied or not all(c in own for c in copied):
        return own
    return [t for t in own if t not in copied]


def leaves(a, out):
    if isinstance(a, dict):
        if "Holds" in a:
            what = a["Holds"]["what"]
            out.add(what if isinstance(what, str) else next(iter(what)))
        for v in a.values():
            leaves(v, out)
    elif isinstance(a, list):
        for v in a:
            leaves(v, out)
    return out


def main(root):
    splits = json.load(open(os.path.join(HERE, "structural_diff_f1c_deltas.json")))["d2_splits"]
    idx = rule_index(root)
    buckets, before, missing = collections.Counter(), collections.Counter(), 0
    for principal_id, sibling_id in splits.items():
        p, s = idx.get(principal_id), idx.get(sibling_id)
        if not p or not s:
            missing += 1
            continue
        for leaf in leaves(s["applies"], set()) & UNCARRIED:
            before[leaf] += 1
        gate = line_gate(p, s)
        if all(is_situational(t) for t in gate):
            buckets["unconditional"] += 1
        elif leaves(gate, set()) & UNCARRIED:
            buckets["undecided_leaf"] += 1
        else:
            buckets["decided"] += 1
    print(json.dumps({"d2_siblings": len(splits), "not_found": missing, "line_gate": dict(buckets),
                      "original_gate_uncarried_leaves": dict(before)}, indent=1))


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "data/sheet_rules")
