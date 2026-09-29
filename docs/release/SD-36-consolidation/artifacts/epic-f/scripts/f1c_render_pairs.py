#!/usr/bin/env python3
"""SD-36 Epic F1c render receipt: pair D2 relocations and bucket every LINE delta by mechanism.

  f1c_render_pairs.py <before_dir> <after_dir> <out.json>

Per build, per rule id (LINE| rows as parsed by f1c_render_classify.py):
  d2_relocated   principal printed X on tranche/16 and prints '' on F1c, and a NEW sibling
                 <principal>#... prints the same X under the principal's label (or that label plus a
                 "(target)" suffix the split adds) on F1c (the sibling row is paired too)
  d2_folded      principal printed X and prints '' on F1c; another rule id prints the identical
                 (kind, label, printed, condition) line on F1c (render_sheet's duplicate fold)
  d2_lost        principal printed X and prints '' on F1c with no same-value new sibling
  added_undecided  a new line whose condition is an undecided-fact leaf in words
  added_other    any other new line
  changed / removed / changed_condition   as f1c_render_classify.py
"""
import collections, json, os, re, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from f1c_render_classify import load  # noqa: E402

UNDECIDED = re.compile(r"requires alignment|as a class skill|deity|language|requires a \S+ character|alignment matching")


def main(before_dir, after_dir, out):
    rows = collections.defaultdict(list)
    for b in sorted(os.listdir(before_dir)):
        name = b[:-4]
        bl, _, _ = load(os.path.join(before_dir, b))
        al, _, _ = load(os.path.join(after_dir, b))
        added = {i for i in al if i not in bl}
        paired = set()
        for i in sorted(set(bl) & set(al)):
            if bl[i][2] and not al[i][2]:
                sib = next((s for s in sorted(added) if s.startswith(i + "#") and s not in paired and al[s][2] == bl[i][2] and (al[s][1] == bl[i][1] or al[s][1].startswith(bl[i][1] + " ("))), None)
                if sib:
                    paired.add(sib)
                    rows["d2_relocated"].append((name, i, sib, bl[i][2], al[sib][3]))
                elif any(v == bl[i] for k, v in al.items() if k != i):
                    other = next(k for k, v in al.items() if k != i and v == bl[i])
                    rows["d2_folded"].append((name, i, other, bl[i][2], bl[i][3]))
                else:
                    rows["d2_lost"].append((name, i, bl[i][2], al[i][3]))
            elif bl[i][2] != al[i][2]:
                rows["changed"].append((name, i, bl[i][2], al[i][2]))
            elif bl[i][3] != al[i][3]:
                rows["changed_condition"].append((name, i, bl[i][3], al[i][3]))
        for s in sorted(added - paired):
            key = "added_undecided" if UNDECIDED.search(al[s][3]) else "added_other"
            rows[key].append((name, s, al[s][1], al[s][2], al[s][3]))
        for i in sorted(set(bl) - set(al)):
            rows["removed"].append((name, i, bl[i][1], bl[i][2]))
    summary = {k: len(v) for k, v in rows.items()}
    reloc_cond = collections.Counter("undecided" if UNDECIDED.search(r[4]) else ("none" if r[4] == "-" else "other") for r in rows["d2_relocated"])
    summary["d2_relocated_by_condition"] = dict(reloc_cond)
    json.dump({"summary": summary, "rows": rows}, open(out, "w"), indent=1)
    print(json.dumps(summary))


if __name__ == "__main__":
    main(*sys.argv[1:4])
