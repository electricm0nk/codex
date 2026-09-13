#!/usr/bin/env python3
"""AT-35-E6-003-RULED cycle 4 — the run-time ``pcgen_import`` census, re-derived.

Cycle 3's census is imported whole rather than copied, exactly as cycle 3 imported cycle
2's and cycle 2 imported cycle 1's: the ``GROUPS`` table, ``classify``, and the two
exclusions the gate itself applies (B14 comment lines, B15 ``#[cfg(test)]`` regions) stay
single-sourced. **The gate script and ``scripts/pcgen-residue-baseline.env`` are absent
from this cycle's diff** — no path exempted, no regex weakened, no rebaseline.

What cycle 4 moved: **``apps/desktop`` stops naming the converter at all**, which is the
literal sentence ``AT-35-E6-003``'s Evidence row asks for and which no cycle in this
family had met. Three files, three different mechanisms, and cycle 3's census was wrong
about all three — it recorded that the desktop's 7 hits "all need a converted equivalent
the package does not carry", and none of them did:

* ``feat_catalog.rs`` called ``pcgen_desc::leaked_pcgen_syntax`` to ask whether its own
  rendered catalog row still carried unrendered markup. That predicate reads **output
  prose**: it never opens a corpus record, never names a token key, and never needs the
  ingest grammar. It is the sheet rule (``decisions.md`` §1) stated as a check, so it
  belongs to the side that prints the line. Its body now lives at
  ``rules_core::pilot_compute::resolved_prose::leaked_markup`` and
  ``pcgen_desc::leaked_pcgen_syntax`` **delegates** to it — single-sourced, not copied,
  so the two sides cannot drift. Not one of its cases or its returned strings changed.
  The converter is allowed to depend on the live side's definition of a clean sheet line;
  the live side is not allowed to depend on the converter.
* ``race_trait_picker.rs`` imported ``pcgen_import::race_trait_tokens`` to ask three
  questions about a ``RaceTraitRecord`` it already held. ``race_resolver.rs`` owns that
  type and already imports that reader; the three answers are now
  ``RaceTraitRecord::exclusion_guard_flags`` / ``::negated_fact_gates`` /
  ``::declares_negated_ability_guard``. The picker's own doc comment already said it
  wanted the *relation*, not the token grammar.
* ``corpus_fixtures.rs`` ran the LST parser and the IR converter **inside the shipping
  desktop binary** on four bundled fixture records. ``rules_core::corpus_loader`` already
  does exactly that — same two parsers, same two converters, same ``Box::leak`` for the
  ``'static`` borrow — twice, for the two real corpus loaders. This is its third caller,
  ``load_lst_fixture_corpus``. The desktop keeps what is actually its concern: resolving
  a bundled resource path and reading files.

**What that does NOT do, stated plainly.** The third move does not stop the conversion
happening at run time; it stops it happening in two places. The two
``ir_converter::convert_*_record`` calls the desktop was making are now made once, in
``corpus_loader.rs``, beside the two identical calls already there — so ``src/rules_core``
reads ``19 / 37`` before and ``19 / 37`` after, and the whole ``44 → 37`` drop is the
desktop crate. That is reported here rather than netted away. The remaining 37 clear when
the converted equipment/spell record shape exists and the conversion is a build step, not
a load step.

Re-derive:

    python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle4_runtime_import_census.py

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
    "at35_e6_003_ruled_cycle3_census",
    os.path.join(HERE, "AT-35-E6-003-RULED_cycle3_runtime_import_census.py"),
)
C3 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(C3)

# The two reasons cycle 4 rewrote. Both record a MEASUREMENT this cycle made, so no later
# cycle spends itself re-discovering it.
RENDERER_WHY_CYCLE4 = (
    "The PCGen description renderer called at run time on a corpus record's stored "
    "`description`. Cycle 2 MEASURED the converted candidate against it: "
    "`class_feature_grant_consumer::tests::class_feature_prose_parity_census` renders "
    "every class_feature record both ways at every level 1..=20 under two ability "
    "probes, and the two disagree on 97,332 of 660,320 comparisons across 2,443 distinct "
    "record keys. The gap is CONVERTER-side (a converted rule carries Desc segments the "
    "stored description never had; for some records the converter settled a hole to "
    "plain text the stored `DESC:` still states as a number), so the converted candidate "
    "ships when the converter closes that gap, not before. CYCLE 4: the two "
    "`leaked_pcgen_syntax` calls in this group were NOT part of that gap and are gone -- "
    "that predicate reads rendered OUTPUT text, never a record, so it moved to the live "
    "side as `resolved_prose::leaked_markup` and `pcgen_desc::leaked_pcgen_syntax` now "
    "delegates to it. What is left in this group is the renderer itself: "
    "`PcgenDisplayValues`, `render_pcgen_desc_with_values`, `desc_token_arguments`."
)
IR_CONVERTER_WHY_CYCLE4 = (
    "The converter's own conversion entry points invoked from live code at run time -- "
    "`convert_equipment_record`, `convert_spell_record`. The live side is RUNNING the "
    "conversion instead of reading its output. Needs the converted artefact produced at "
    "build time and read as data. CYCLE 4: this group's count is unchanged at 4 and the "
    "reason is stated rather than netted -- the desktop crate's two calls "
    "(`corpus_fixtures.rs`) were removed and two new ones appear here, in "
    "`corpus_loader::load_lst_fixture_corpus`, because the fixture bundle's parse+convert "
    "moved to the module that already does it for the two real corpus loaders. The "
    "dependency left the shipping desktop binary; it did not leave the live side."
)

_REWRITTEN = {
    "renderer": RENDERER_WHY_CYCLE4,
    "ir_converter": IR_CONVERTER_WHY_CYCLE4,
}

GROUPS = [
    (gid, rx, _REWRITTEN.get(gid, why)) for gid, rx, why in C3.GROUPS
]
FALLBACK = C3.FALLBACK


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
    desktop = by_root.get("apps/desktop", 0)
    print(f"apps_desktop_hits={desktop}  "
          f"evidence_sentence_met={'YES' if desktop == 0 else 'NO'}")
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

    out = os.path.join(HERE, "AT-35-E6-003-RULED_cycle4_runtime_import_census.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump({"total_hits": total, "total_files": len(files),
                   "by_root": dict(by_root), "by_group": sizes, "rows": rows},
                  fh, indent=1)
    print(f"census written to {os.path.relpath(out, ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
