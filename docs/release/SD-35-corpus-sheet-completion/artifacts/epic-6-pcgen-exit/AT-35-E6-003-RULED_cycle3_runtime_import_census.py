#!/usr/bin/env python3
"""AT-35-E6-003-RULED cycle 3 — the run-time ``pcgen_import`` census, re-derived.

Cycle 2's census (``AT-35-E6-003-RULED_cycle2_runtime_import_census.py``) is imported
whole rather than copied, exactly as cycle 2 imported cycle 1's: the ``GROUPS`` table,
``classify``, and the two exclusions the gate itself applies (B14 comment lines, B15
``#[cfg(test)]`` regions) stay single-sourced.

What cycle 3 moved, and it is one mechanism plus one ownership correction — **no
rename, no exclusion, no weakened regex, and the gate script itself is untouched**:

* ``src/rules_core/composed_input.rs`` lost **all eight** of its reads. The module's own
  doc comment claimed the composer "does no parsing, no include resolution, no IR
  conversion of its own". That was true of ``compose`` and **false of the file**: the PCC
  convenience loader that shared it resolved an include graph, ran all six B-family LST
  parsers and ran the IR converter — at run time, from a live root. Those eight reads
  counted against the live side under ``decisions.md §19``/B16 and they counted
  correctly. ``load_composed_core_rulebook``, ``project_corpus_from_owned``,
  ``drain_into``, ``ComposedCoreRulebookOwnedInputs``, ``ComposedCoreRulebookLoadResult``
  and ``ComposedInputDiagnostic::include_resolution_failure`` now live at
  ``src/pcgen_import/pcc_package_loader.rs`` — same functions, same order, same
  diagnostics, same borrow discipline, KEPT per ``decisions.md §11``. Its only consumer
  is ``tests/sd18_preloop_consumer_compose.rs``, which imports the new path.
* ``src/rules_core/rules_tables/crb/json_cache.rs`` lost its one read. Its
  ``CorpusRecord::rename`` field was typed
  ``Option<cache_gen::equipment_gap::RenameInfo>`` — a live reader naming a converter
  module to describe the shape of **its own on-disk JSON**. ``cache_gen`` already keeps
  three separate local copies of that two-string shape (``equipment_gap``,
  ``class_feature``, ``spell_lane_dump``) under the no-shared-types-file convention
  ``equipment_gap.rs``'s own doc comment establishes; this is the fourth, owned by the
  side that reads it. The wire shape is byte-identical.

**What cycle 3 deliberately did NOT do, because checking refuted the plan.**
``src/rules_core/derived_evaluator_fixture_check.rs`` looked like the same relocation as
``AT-35-E6-001``'s move of ``race_trait_formula_bar_check`` to ``src/oracle_validation/``:
a bar-check harness reading a committed fixture, sitting in a live root. It is not. Three
desktop catalogs import real rendering functions out of it at run time
(``monster_catalog.rs`` → ``spell_like_ability_caster_level``/``_save_dc``,
``spell_catalog.rs`` → ``all_spell_caster_level_durations``/``_ranges`` and the two
formatters, ``companion_catalog.rs`` → six ``parse_``/``format_companion_*``), and the
private walks those reach (``load_spell_durations``, ``load_spell_ranges``,
``load_class_feature_bonus_vars``) are what call ``ingest_record::first_token_value`` /
``token_values``. Moving the file would have moved the import line off the live roots and
left the live reads exactly where they are — the gate would read one lower and the
shipping binary would be unchanged. That is the blind-spot shape ``decisions.md §19`` was
ruled to end, so the hit stays counted and is named below instead.

Re-derive:

    python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle3_runtime_import_census.py

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
    "at35_e6_003_ruled_cycle2_census",
    os.path.join(HERE, "AT-35-E6-003-RULED_cycle2_runtime_import_census.py"),
)
C2 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(C2)

# The one reason cycle 3 rewrote: the `ingest_record_tokens` group now carries the
# measured refutation of the relocation that looked available, so no later cycle spends
# itself re-discovering it.
INGEST_RECORD_WHY_CYCLE3 = (
    "Run-time token-string reads: `token_pairs`, `bonus_chain_qualifiers`, "
    "`rebuild_bonus_token`, `token_count`, `first_token_value`/`token_values`, and the "
    "`RaceCacheData` / `RaceTraitCacheData` payload shapes. These are the literal ingest "
    "tokens the sheet rule forbids, reached through a function call so no token-syntax "
    "pattern fires. Needs the converted package to carry the same facts keyed by `VarId` "
    "rather than by source token name. CYCLE 3, MEASURED: "
    "`derived_evaluator_fixture_check.rs` is NOT relocatable to the tool side the way "
    "`AT-35-E6-001` relocated `race_trait_formula_bar_check` -- three desktop catalogs "
    "(`monster_catalog.rs`, `spell_catalog.rs`, `companion_catalog.rs`) import live "
    "rendering functions out of it, and those reach the private corpus walks that make "
    "the `ingest_record` calls. Re-derive: "
    "`grep -rn 'rules_core::derived_evaluator_fixture_check' --include=*.rs apps/`."
)

GROUPS = [
    (gid, rx, INGEST_RECORD_WHY_CYCLE3 if gid == "ingest_record_tokens" else why)
    for gid, rx, why in C2.GROUPS
]
FALLBACK = C2.FALLBACK


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

    out = os.path.join(HERE, "AT-35-E6-003-RULED_cycle3_runtime_import_census.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump({"total_hits": total, "total_files": len(files),
                   "by_root": dict(by_root), "by_group": sizes, "rows": rows},
                  fh, indent=1)
    print(f"census written to {os.path.relpath(out, ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
