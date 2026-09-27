#!/usr/bin/env python3
"""Enumerate and pin the F6c package deltas (SD-36 Epic F6c, converter step).

Every delta must be one of:
  - `class_printing`: on a class principal (`<book>:class:<slug>`, no `#`), `provenance` differs
    ONLY by an added `printing` key: {source_date?, printings, newest?}, `printings` the sorted ids
    of every class principal with that slug (2+ books), this rule among them;
  - `class_granted_domain_count`: on a class sibling (`<book>:class:<slug>#...`), `offers` REMOVED
    (it was the F4pre domain choice) and `print` true -> false, target `Other "domains"`;
  - `class_granted_domain_record`: on a domain principal every `granted_by` edge of which is the
    automatic grant (`Class`) of a class whose count above is withheld, `print` true -> false only.
Pinned by sha256 of the new `printing` value (`structural_diff.f3b2_field_sha`). Anything else --
a file or rule id added or removed, any other field, a side file -- is printed and nothing is
written.

Usage (from the repo root):
    python3 f6c_delta_pins.py <baseline package dir> <fresh package dir> <out json>
The baseline is the package at tranche/16 HEAD (git archive), the structural diff's own baseline.
"""
from __future__ import annotations

import json
import os
import sys
from collections import Counter

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import structural_diff as sd  # noqa: E402

OWNER = "adventurers_guide:class:hellknight"


def is_printing_delta(rid: str, old: dict, new: dict, fresh: dict) -> bool:
    op, np_ = dict(old.get("provenance", {})), dict(new.get("provenance", {}))
    printing = np_.pop("printing", None)
    if "printing" in op or op != np_ or not isinstance(printing, dict):
        return False
    if ":class:" not in rid or "#" in rid:
        return False
    printings = printing.get("printings")
    if not isinstance(printings, list) or len(printings) < 2 or printings != sorted(printings) or rid not in printings:
        return False
    slug = rid.split(":class:", 1)[1]
    if any(p.split(":class:", 1)[-1] != slug or p not in fresh for p in printings):
        return False
    newest = printing.get("newest")
    return newest is None or newest in printings


def is_withheld_count(old: dict, new: dict) -> bool:
    return (
        old.get("print") is True
        and new.get("print") is False
        and sd.f4pre_offer_kind(old) == "domain_count"
        and "offers" not in new
        and {k: v for k, v in old.items() if k not in ("offers", "print")} == {k: v for k, v in new.items() if k not in ("offers", "print")}
    )


def is_unprinted_domain(rid: str, old: dict, new: dict, withheld_classes: set) -> bool:
    if ":domain:" not in rid or "#" in rid or old.get("print") is not True or new.get("print") is not False:
        return False
    if {k: v for k, v in old.items() if k != "print"} != {k: v for k, v in new.items() if k != "print"}:
        return False
    edges = new.get("granted_by") or []
    return bool(edges) and all(isinstance(e.get("by"), dict) and "Class" in e["by"] and e["by"]["Class"].get("id") in withheld_classes for e in edges)


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
    printings: list = []
    withheld: list = []
    unprinted: list = []
    verdicts: Counter = Counter()
    withheld_classes = {
        rid.split(":class:", 1)[1].split("#", 1)[0]
        for rid in set(base) & set(fresh)
        if ":class:" in rid and "#" in rid and is_withheld_count(base[rid], fresh[rid])
    }
    for rid in sorted(set(base) & set(fresh)):
        fields = sorted(f for f in set(base[rid]) | set(fresh[rid]) if base[rid].get(f) != fresh[rid].get(f))
        if not fields:
            continue
        if fields == ["provenance"] and is_printing_delta(rid, base[rid], fresh[rid], fresh):
            p = fresh[rid]["provenance"]["printing"]
            printings.append([rid, sd.f3b2_field_sha(p)])
            verdicts["proved one object" if p.get("newest") else "not proved"] += 1
            continue
        if set(fields) <= {"offers", "print"} and is_withheld_count(base[rid], fresh[rid]):
            withheld.append(rid)
            continue
        if fields == ["print"] and is_unprinted_domain(rid, base[rid], fresh[rid], withheld_classes):
            unprinted.append(rid)
            continue
        unexplained.append(f"field moved beyond an F6c delta: {rid}: {fields}")
    if unexplained:
        for line in unexplained[:200]:
            print(line)
        print(f"{len(unexplained)} unexplained -- nothing written")
        return 1
    data = {
        "_purpose": "SD-36 Epic F6c (converter step): class principals several books state carry provenance.printing (reprint::stamp_class_printings); a class's BONUS:DOMAIN|NUMBER count its own DOMAIN: grants fill keeps no offers and prints no line (pool_link::withhold_class_granted_domain_counts), and the domain records only that class grants print no line; structural_diff.py undoes each pin first (f6c_apply).",
        "_command": "python3 f6c_delta_pins.py <tranche/16 package, git archive> <fresh package dir> structural_diff_f6c_deltas.json",
        "owner": OWNER,
        "class_printing": {"_count": len(printings), "by_verdict": dict(sorted(verdicts.items())), "pins": printings},
        "class_granted_domain_count": {"_count": len(withheld), "pins": withheld},
        "class_granted_domain_record": {"_count": len(unprinted), "pins": unprinted},
    }
    with open(out, "w", encoding="utf-8") as fh:
        json.dump(data, fh, indent=1)
        fh.write("\n")
    print(f"pinned {len(printings)} class printings {dict(verdicts)}; {len(withheld)} withheld domain counts {withheld}; {len(unprinted)} unprinted domain records")
    return 0


if __name__ == "__main__":
    sys.exit(main())
