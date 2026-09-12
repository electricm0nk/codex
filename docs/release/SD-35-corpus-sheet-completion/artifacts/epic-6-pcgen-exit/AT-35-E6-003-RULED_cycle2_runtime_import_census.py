#!/usr/bin/env python3
"""AT-35-E6-003-RULED cycle 2 — the run-time ``pcgen_import`` census, re-derived.

Cycle 1's census (``AT-35-E6-003-RULED_cycle1_runtime_import_census.py``) is imported
whole rather than copied: its ``GROUPS`` table, its ``classify`` and its two exclusions
(B14 comment lines, B15 ``#[cfg(test)]`` regions) are the same ones the gate applies, and
a second copy of them would be a second thing to keep in step.

Two things differ, and both are corrections this cycle earned:

* ``provenance_prose`` is expected to be **0**. Cycle 2 cleared it — the five
  ``src/pcgen_import/sheet_rule/`` module paths written inside ``reach_gate.rs`` string
  literals now name the runnable converter (``cargo run --bin sheet_rule_convert``)
  instead, which is better provenance for a reader and carries no live-side module path.
* The ``renderer`` group's stated reason is **replaced**. Cycle 1 wrote "The mechanism is
  proved"; cycle 2 measured it over the whole corpus and it is not. The measured figure
  is carried here so no later reader picks up the disproved claim.

Re-derive:

    python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle2_runtime_import_census.py

Denominator for every figure: every source file (``.rs .ts .tsx .js .jsx .mjs .cjs``)
under the five live roots of ``technical-design.md`` §0, comment lines excluded (B14) and
``#[cfg(test)]`` regions excluded (B15). The total is asserted against
``scripts/pcgen_residue_gate.py``'s own ``hits_by_pattern["pcgen_import"]``, so the census
and the gate cannot disagree.
"""

import collections
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", ".."))
sys.path.insert(0, os.path.join(ROOT, "scripts"))

import pcgen_residue_gate as G  # noqa: E402

_spec = importlib.util.spec_from_file_location(
    "at35_e6_003_ruled_cycle1_census",
    os.path.join(HERE, "AT-35-E6-003-RULED_cycle1_runtime_import_census.py"),
)
C1 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(C1)

# The one reason cycle 2 rewrote, with the command that disproved the old one.
RENDERER_WHY_CYCLE2 = (
    "The PCGen description renderer called at run time on a corpus record's stored "
    "`description`. Cycle 1 recorded this group as 'the mechanism is proved'; cycle 2 "
    "MEASURED it and that is false. `class_feature_grant_consumer::tests::"
    "class_feature_prose_parity_census` renders every class_feature record both ways -- "
    "the live PCGen path over the stored `DESC:` text, and the converted rule through "
    "`resolved_prose::render_description` -- at every level 1..=20 under two ability "
    "probes, and the two disagree on 97,332 of 660,320 comparisons across 2,443 distinct "
    "record keys (both_some_differ=53,698 over 1,351 keys; converted-renders-where-live-"
    "did-not=28,720 over 718; live-renders-where-converted-does-not=14,914 over 374). The "
    "formula-only sibling disagrees on 11,978 of 660,320. The gap is CONVERTER-side: a "
    "converted rule carries Desc segments the stored description never had (a `.MOD` row "
    "merged from a second book), and for some records the converter settled a hole to "
    "plain text that the stored `DESC:` still states as a number. The converted candidate "
    "is written and exercised, in this module's own `#[cfg(test)]` region; it ships when "
    "the converter closes that gap, not before."
)

GROUPS = [
    (gid, rx, RENDERER_WHY_CYCLE2 if gid == "renderer" else why)
    for gid, rx, why in C1.GROUPS
]
FALLBACK = C1.FALLBACK


def classify(text):
    for gid, rx, why in GROUPS:
        if rx.search(text):
            return gid, why
    return FALLBACK


def main():
    rx = G._COMPILED["pcgen_import"]
    rows = []
    for live_root, rel, abs_path in G._iter_live_source_files(ROOT):
        with open(abs_path, encoding="utf-8", errors="replace") as fh:
            lines = fh.read().splitlines()
        skip = set()
        for a, b in G.cfg_test_ranges(lines):
            skip.update(range(a, b + 1))
        for i, line in enumerate(lines):
            if i in skip or line.lstrip().startswith("//"):
                continue
            n = len(rx.findall(line))
            if not n:
                continue
            gid, why = classify(line)
            rows.append(dict(root=live_root, file=rel, line=i + 1, hits=n,
                             group=gid, why=why, text=line.strip()[:200]))

    total = sum(r["hits"] for r in rows)
    assert total == G.scan(ROOT).hits_by_pattern["pcgen_import"], (
        "census disagrees with the gate it imports"
    )
    files = {r["file"] for r in rows}
    print(f"pcgen_import_hits={total} files={len(files)}")
    by_root = collections.Counter()
    for r in rows:
        by_root[r["root"]] += r["hits"]
    print("by_root=" + ", ".join(f"{k}={v}" for k, v in sorted(by_root.items())))
    print()

    by_group = collections.defaultdict(list)
    for r in rows:
        by_group[r["group"]].append(r)
    order = [g[0] for g in GROUPS] + [FALLBACK[0]]
    sizes = {}
    for gid in order:
        group = by_group.get(gid)
        sizes[gid] = sum(r["hits"] for r in group) if group else 0
        if not group:
            print(f"=== {gid}: hits=0 files=0 === CLEARED")
            print()
            continue
        gfiles = sorted({r["file"] for r in group})
        print(f"=== {gid}: hits={sizes[gid]} files={len(gfiles)} ===")
        print("  why it is still here: " + group[0]["why"])
        for r in sorted(group, key=lambda r: (r["file"], r["line"])):
            print(f"  {r['file']}:{r['line']}  {r['text']}")
        print()

    print("refused_tokens=" + ", ".join(
        f"{gid}={sizes[gid]}" for gid in order if sizes[gid]
    ))

    out = os.path.join(HERE, "AT-35-E6-003-RULED_cycle2_runtime_import_census.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump({"total_hits": total, "total_files": len(files),
                   "by_root": dict(by_root), "by_group": sizes, "rows": rows},
                  fh, indent=1)
    print(f"census written to {os.path.relpath(out, ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
