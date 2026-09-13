#!/usr/bin/env python3
"""AT-35-E6-003-RULED cycle 6 — the run-time ``pcgen_import`` census, re-derived.

Cycle 5's census is imported whole (which imports cycle 4's, which imports cycle 3's, which
imports cycle 2's, which imports cycle 1's): the ``GROUPS`` table, ``classify``, and the two
exclusions the gate itself applies (B14 comment lines, B15 ``#[cfg(test)]`` regions) stay
single-sourced. **The gate script and ``scripts/pcgen-residue-baseline.env`` are absent from
this cycle's diff** — no path exempted, no regex weakened, no rebaseline.

What cycle 6 moved
------------------

One hit, one whole file: ``src/rules_core/rules_tables/simple_kind_tables.rs``. The
seven-kind engine table carried a ``raw_token_count`` field, filled by
``pcgen_import::ingest_record::token_count`` — the length of the corpus record's own PCGen
``raw_tokens`` array, read inside the crate that prints a character sheet, and printed by
``--epic2-table-transcript`` as ``ingest_tokens=N``. It is now ``converted_rule_count``,
filled through cycle 5's closure-row join: how many rules ``sheet_rule_convert`` wrote from
**this record's own source row**. The transcript prints ``converted_rules=N``, or
``not-converted`` where the package holds nothing — never ``0``, which would read as a
converted record with no rules.

Pinned by a corpus-wide test over all seven Epic-2 kinds
(``every_seven_kind_record_resolves_to_its_own_converted_rules``) that recomputes the join
in-test as its oracle and asserts **totality**: 8,486 of 8,486 records resolve to
``Some(n >= 1)``. Mutation-proved red — joining on ``source_line + 1`` makes it fail naming
real records.

What cycle 6 MEASURED and refused, and why that is the cycle's other product
---------------------------------------------------------------------------

``trait_and_pool_tokens`` has carried the same reason since cycle 1: that
``class_feature_pool_catalog``'s four ``pool_member_tokens`` guards "read a record's
DESC/effect token array rather than one classified fact" and so do not ride on the join.
Cycle 5's own lesson is that such a sentence is an assertion about a dependency and has to be
checked against what the caller needs. This cycle checked it, over the whole live
``class_feature`` corpus, rather than carrying it forward a seventh time:
``AT-35-E6-003-RULED_cycle6_pool_guard_parity.py``.

The join is fine — **18,043 of 18,074** live ``class_feature`` records resolve. The converted
rule is not:

* ``P1 has_no_engine_effect_token``: **6,494 of 18,043 disagree.**
* ``P2 is_archetype_locked``: 919 of 18,043 disagree.
* ``P3 carries_more_than_one_desc_segment``: 89 of 18,043 disagree.

P1 is the guard that decides the swap and the mechanism behind it is exact, not vague:
PCGen's ``ABILITY:`` token maps to ``MapsTo::Applies`` — a *prerequisite* — in
``src/pcgen_import/sheet_rule/table.rs``, and ``BONUS:VAR|...`` lands in
``data/sheet_rules/_vars/`` rather than on the rule. A record that hands out two abilities
automatically therefore converts to ``grants: null, target: None, value: "Text"``, which is
byte-identical, on the converted side, to a record that really is prose only. **The swap is
refused on the number**, the disposition cycle 2 reached for the renderer group, and the
remedy is converter-side: ``ABILITY``'s mapping row, and a ``BONUS:VAR`` reference on the
rule that declares it.
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
    "at35_e6_003_ruled_cycle5_census",
    os.path.join(HERE, "AT-35-E6-003-RULED_cycle5_runtime_import_census.py"),
)
C5 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(C5)

INGEST_RECORD_WHY_CYCLE6 = (
    "Run-time token-string reads: `token_pairs`, `bonus_chain_qualifiers`, "
    "`rebuild_bonus_token`, and the `RaceCacheData` / `RaceTraitCacheData` payload shapes. "
    "CYCLE 6: `token_count` leaves this group -- "
    "`rules_tables/simple_kind_tables.rs` carried the length of a corpus record's PCGen "
    "`raw_tokens` array as `raw_token_count` and printed it as `ingest_tokens=N`; it now "
    "carries `converted_rule_count`, how many rules the converter wrote from that record's "
    "own source row, through cycle 5's closure-row join, total over 8,486 of 8,486 "
    "seven-kind corpus records. What is left is NOT blocked on the join: `corpus_loader`'s "
    "three calls REBUILD an `EquipmentRecord`'s token and BONUS arrays out of the corpus "
    "JSON, and `race_resolver`'s cache shapes carry `raw_tokens`/`raw_bonus_chains` as "
    "fields. Both want a converted equipment/race-trait rule shape to read instead, which "
    "is the same one piece of work `lst_parser_types` names. "
    "`derived_evaluator_fixture_check.rs` stays measured non-relocatable (cycle 3): three "
    "desktop catalogs import live rendering functions out of it, and its three token reads "
    "sit inside the private corpus sweeps those imports reach "
    "(`load_spell_durations`, `load_spell_ranges`, `load_class_feature_bonus_vars`). "
    "Re-derive: `grep -rn 'rules_core::derived_evaluator_fixture_check' --include=*.rs apps/`."
)
TRAIT_AND_POOL_WHY_CYCLE6 = (
    "Per-row token readers for racial traits, feature pools and bonus chains. CYCLE 6 "
    "MEASURED the `class_feature_pool_catalog` hit rather than restating cycle 1's reason "
    "for it a seventh time, and REFUSES it on the number. The closure-row join resolves "
    "18,043 of 18,074 live `class_feature` corpus records, so the join is not the blocker; "
    "the converted rule is. Over that population the converted probe for "
    "`has_no_engine_effect_token` disagrees with the ingest read on 6,494 records "
    "(`is_archetype_locked` 919, `carries_more_than_one_desc_segment` 89) -- because "
    "PCGen's `ABILITY:` token maps to `MapsTo::Applies`, a PREREQUISITE, and `BONUS:VAR` "
    "lands in `data/sheet_rules/_vars/` rather than on the rule, so a record that grants "
    "two abilities automatically converts to `grants: null, target: None, value: \"Text\"` "
    "-- indistinguishable from a genuinely prose-only record. The remedy is converter-side. "
    "`race_resolver`/`skinwalker_change_shape` read `raw_tokens`/`raw_bonus_chains` off a "
    "cache payload whose converted equivalent is the race-trait rule shape this epic's "
    "remaining piece produces. Re-derive: "
    "`python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-6-pcgen-exit/"
    "AT-35-E6-003-RULED_cycle6_pool_guard_parity.py`."
)

_REWRITTEN = {
    "ingest_record_tokens": INGEST_RECORD_WHY_CYCLE6,
    "trait_and_pool_tokens": TRAIT_AND_POOL_WHY_CYCLE6,
}

GROUPS = [(gid, rx, _REWRITTEN.get(gid, why)) for gid, rx, why in C5.GROUPS]
FALLBACK = C5.FALLBACK


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

    # The census's own total must equal the gate's, or one of them is lying.
    gate_total = G.scan(ROOT).hits_by_pattern["pcgen_import"]
    assert total == gate_total, f"census {total} != gate {gate_total}"
    print(f"gate_agreement=OK ({total} == {gate_total})")

    out = os.path.join(HERE, "AT-35-E6-003-RULED_cycle6_runtime_import_census.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump({"total_hits": total, "total_files": len(files),
                   "by_root": dict(by_root), "by_group": sizes, "rows": rows},
                  fh, indent=1)
    print(f"census written to {os.path.relpath(out, ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
