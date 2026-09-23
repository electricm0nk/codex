#!/usr/bin/env python3
"""SD-36 Epic F1c render receipt classifier.

  f1c_render_classify.py <before_dir> <after_dir> <out.json>

Reads every `LINE|` row of each build dump written by f1c_render_population.py, keys it by
rule id, and diffs before -> after per build: added id, removed id, changed `printed`, changed
`condition` only, changed label. Also reports builds whose dump failed (EXIT=), a same-build
duplicate scan (identical (kind,label,printed,condition) under two ids), and the EXPL| rows whose
value differs. Grouped by (change, id, before, after) with the builds each group occurs in.
"""
import collections, json, os, re, sys

LINE = re.compile(r"^LINE\| id=(\S+) kind=(\S+) label=(.*) printed=(.*?) condition=(.*)$")


def load(path):
    lines, expl, failed = {}, {}, False
    for raw in open(path):
        raw = raw.rstrip("\n")
        if raw.startswith("EXIT="):
            failed = True
        m = LINE.match(raw)
        if m:
            i, kind, label, printed, cond = m.groups()
            lines[i] = (kind, label, printed, cond)
        elif raw.startswith("EXPL| "):
            m2 = re.match(r"^EXPL\| id=(\S+) value=(\S*)", raw)
            if m2:
                expl[m2.group(1)] = m2.group(2)
    return lines, expl, failed


def dups(lines):
    seen = collections.Counter(v for v in lines.values())
    return sum(n - 1 for n in seen.values() if n > 1)


def main(before_dir, after_dir, out):
    builds = sorted(set(os.listdir(before_dir)) | set(os.listdir(after_dir)))
    groups = collections.defaultdict(list)
    per_build, failed, expl_changes = {}, {"before": [], "after": []}, collections.defaultdict(list)
    dup_before = dup_after = 0
    for b in builds:
        name = b[:-4]
        bl, be, bf = load(os.path.join(before_dir, b)) if os.path.exists(os.path.join(before_dir, b)) else ({}, {}, True)
        al, ae, af = load(os.path.join(after_dir, b)) if os.path.exists(os.path.join(after_dir, b)) else ({}, {}, True)
        if bf:
            failed["before"].append(name)
        if af:
            failed["after"].append(name)
        dup_before += dups(bl)
        dup_after += dups(al)
        counts = collections.Counter()
        for i in sorted(set(bl) | set(al)):
            if i not in bl:
                key = ("added", i, "", f"{al[i][1]} printed={al[i][2]} condition={al[i][3]}")
            elif i not in al:
                key = ("removed", i, f"{bl[i][1]} printed={bl[i][2]} condition={bl[i][3]}", "")
            elif bl[i][2] != al[i][2]:
                key = ("changed_printed", i, f"{bl[i][2]} [{bl[i][3]}]", f"{al[i][2]} [{al[i][3]}]")
            elif bl[i][3] != al[i][3]:
                key = ("changed_condition", i, bl[i][3], al[i][3])
            elif bl[i][1] != al[i][1]:
                key = ("changed_label", i, bl[i][1], al[i][1])
            else:
                continue
            counts[key[0]] += 1
            groups[key].append(name)
        per_build[name] = dict(counts)
        for k in sorted(set(be) | set(ae)):
            if be.get(k) != ae.get(k):
                expl_changes[(k, be.get(k), ae.get(k))].append(name)
    totals = collections.Counter()
    rows = collections.Counter()
    for (kind, *_), bs in groups.items():
        totals[kind] += 1
        rows[kind] += len(bs)
    doc = {
        "population": len(builds),
        "failed_dumps": failed,
        "duplicate_extra_copies": {"before": dup_before, "after": dup_after},
        "distinct_changes_by_kind": dict(totals),
        "line_rows_by_kind": dict(rows),
        "per_build": per_build,
        "changes": [
            {"change": k[0], "id": k[1], "before": k[2], "after": k[3], "builds": len(v), "build_list": v}
            for k, v in sorted(groups.items())
        ],
        "expl_changes": [
            {"expl": k[0], "before": k[1], "after": k[2], "builds": len(v), "build_list": v}
            for k, v in sorted(expl_changes.items(), key=lambda kv: (kv[0][0], str(kv[0][1]), str(kv[0][2])))
        ],
    }
    json.dump(doc, open(out, "w"), indent=1)
    print(json.dumps({k: doc[k] for k in ("population", "duplicate_extra_copies", "distinct_changes_by_kind", "line_rows_by_kind")}))
    print("failed", {k: len(v) for k, v in failed.items()}, "expl_changes", len(doc["expl_changes"]))


if __name__ == "__main__":
    main(*sys.argv[1:4])
