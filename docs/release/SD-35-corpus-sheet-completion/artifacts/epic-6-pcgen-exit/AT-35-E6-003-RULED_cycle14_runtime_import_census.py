#!/usr/bin/env python3
"""AT-35-E6-003-RULED cycle 14 — the run-time ``pcgen_import`` census, re-derived.

Cycle 13's census is imported whole (which imports cycle 12's, … back to cycle 1's): the
``GROUPS`` table, ``classify``, and the two exclusions the gate itself applies (B14 comment
lines, B15 ``#[cfg(test)]`` regions) stay single-sourced. **The gate script and
``scripts/pcgen-residue-baseline.env`` are absent from this cycle's diff** — no path
exempted, no regex weakened, no rebaseline.

What cycle 14 moved
-------------------

``src/rules_core/race_resolver.rs`` — the last live file with THREE converter imports — is
**cleared whole**. It used to hold

* ``use crate::pcgen_import::ingest_payload::{RaceCacheData, RaceTraitCacheData}``,
* ``use crate::pcgen_import::race_trait_tokens``,
* ``use crate::pcgen_import::bonus_chain_reader::{self, DeclaredBonuses}``

and then read **eleven** separate facts back out of the ingest token and bonus-chain arrays
at run time, once per accessor call: ``exclusion_guard_flags``, ``negated_fact_gates``,
``declares_preability_negated_guard``, ``automatic_ability_grants``,
``skinwalker_change_shape_kin``, ``positive_prefact_flag``, ``declared_walk_speed_ft``,
``declared_size``, ``declared_vision_segments``, ``adopted_race_pool_suffix`` and
``bonus_chain_reader::declared_bonuses``.

Every one of those readings still runs, in the same function, with the same body, on the
converter side — in the new ``src/pcgen_import/corpus_race_json.rs``, which settles a race
chassis and a racial trait ONCE at the ingest boundary. The live resolver holds
``rules_core::race_record::{CorpusRaceRecord, CorpusRaceTraitRecord}`` — settled values with
no token array and no bonus-chain array on either struct — and names no converter at all.

Three closures and one relabel, booked separately
-------------------------------------------------

This cycle does **not** claim four. The boundary call did not vanish: it moved into
``src/rules_core/corpus_loader.rs``, which already owned the live side's one ingest boundary
for equipment, and that file's hit count therefore RISES from 1 to 2. Three hits out of
``race_resolver``, one hit back in ``corpus_loader``: net ``14 → 12``, and **one file
cleared** (8 → 7), the first file this criterion has cleared since cycle 12.

Parity
------

``pcgen_import::corpus_race_json``'s two whole-corpus parity proofs re-derive every moved
reading the way the live module derived it — straight off the deserialized cache payload —
and compare field for field over **every** race and race_trait record under ``data/corpus/``.
The racial-trait prose oracle in
``tests/sd35_race_trait_prose_comes_from_the_converted_package.rs`` is KEPT and still reads
the ingest arrays; it reads them off disk itself now, in ``tests/``, which is exactly where
``decisions.md §11`` says an oracle belongs.
"""

import collections
import importlib.util
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", ".."))
sys.path.insert(0, os.path.join(ROOT, "scripts"))

import pcgen_residue_gate as G  # noqa: E402

_spec = importlib.util.spec_from_file_location(
    "at35_e6_003_ruled_cycle13_census",
    os.path.join(HERE, "AT-35-E6-003-RULED_cycle13_runtime_import_census.py"),
)
C13 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(C13)

CORPUS_JSON_BOUNDARY_WHY_CYCLE14 = (
    "THE INGEST BOUNDARY, and after cycle 14 it is TWO calls in ONE file -- equipment "
    "(cycle 13) and race/race_trait (this cycle). `rules_core::corpus_loader` reads an "
    "already-converted `data/corpus/<book>/<kind>/**/*.json` record's `data` object and "
    "asks `pcgen_import::corpus_equipment_json` / `pcgen_import::corpus_race_json` what "
    "canonical record that object stands for. "
    "THE SECOND HIT IS BOOKED AS A RELABEL, NOT A REGRESSION AND NOT A CLOSURE: "
    "`race_resolver` had THREE converter imports and now has none, and the boundary call "
    "it used to make from inside itself moved into the file that already owned the live "
    "side's one ingest boundary. Three hits out, one hit back, net -2, and one file "
    "cleared. "
    "BOTH clear the same way, and it is a DATA step rather than a code step: when "
    "`data/corpus/` JSON carries the settled fields itself -- an ingest-side generator "
    "cycle, the same shape `data/sheet_rules/` already has -- the live loader deserializes "
    "settled values with serde and needs no converter call at all. That clears this whole "
    "file. Re-derive: `grep -rn 'corpus_equipment_json\\|corpus_race_json' --include=*.rs "
    "src/rules_core/`."
)

INGEST_RECORD_TOKENS_WHY_CYCLE14 = (
    "Run-time token-string reads. CYCLE 13 CLOSED `corpus_loader`'s THREE; CYCLE 14 CLOSED "
    "`race_resolver`'s: the `RaceCacheData`/`RaceTraitCacheData` cache-payload shapes are "
    "deserialized on the converter side now, inside "
    "`corpus_race_json::corpus_race_source_record` / `corpus_race_trait_source_record`, and "
    "the live resolver holds `rules_core::race_record::{CorpusRaceRecord, "
    "CorpusRaceTraitRecord}` -- settled structs with NO token array and NO bonus-chain "
    "array, deliberately, so no future live module can re-open the ingest format behind the "
    "gate's back. ONE REMAINS: `derived_evaluator_fixture_check.rs`, MEASURED "
    "non-relocatable (cycle 3) -- three desktop catalogs import live rendering functions "
    "out of it and its token reads sit inside the private corpus sweeps those imports "
    "reach. Re-derive: `grep -rn 'ingest_record\\|ingest_payload' --include=*.rs "
    "src/rules_core/`."
)

TRAIT_AND_POOL_TOKENS_WHY_CYCLE14 = (
    "Per-row token readers for racial traits, feature pools and bonus chains. CYCLE 14 "
    "CLOSED `race_resolver`'s TWO -- `race_trait_tokens` and `bonus_chain_reader` -- by "
    "settling all eleven of its run-time readings at the ingest boundary; the reading "
    "functions themselves did not move, did not change, and are still the only place that "
    "grammar is spelled (`decisions.md` §11 KEEPS them, for Starfinder). "
    "`DeclaredBonuses`, `AbilityAdjustment` and `VarContribution` -- already settled "
    "ANSWERS rather than grammar, with no qualifier position and no chain keyword in any of "
    "them -- are DECLARED on the live side now (`rules_core::declared_bonuses`) and "
    "re-exported by `bonus_chain_reader`, which still fills them. "
    "ONE REMAINS: `class_feature_pool_catalog`'s `pool_member_tokens`, RE-MEASURED and "
    "STILL REFUSED on cycle 7's corrected number -- `ABILITY:` converts as a GRANT EDGE "
    "folded onto the TARGET rule's `granted_by` in a different file, and the probe "
    "disagrees on 1,870 of 18,043 (P1), 864 (P2), 89 (P3). Refused on the number, not on "
    "effort. Re-derive: `python3 docs/release/SD-35-corpus-sheet-completion/artifacts/"
    "epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle7_pool_guard_parity.py`."
)

_REWRITTEN = {
    "corpus_json_boundary": CORPUS_JSON_BOUNDARY_WHY_CYCLE14,
    "ingest_record_tokens": INGEST_RECORD_TOKENS_WHY_CYCLE14,
    "trait_and_pool_tokens": TRAIT_AND_POOL_TOKENS_WHY_CYCLE14,
}

# The boundary group's regex widens to name the new converter-side reader too. Nothing else
# about the table changes, and no group is dropped.
GROUPS = []
for gid, rx, why in C13.GROUPS:
    if gid == "corpus_json_boundary":
        rx = re.compile(r"corpus_equipment_json|corpus_race_json")
    GROUPS.append((gid, rx, _REWRITTEN.get(gid, why)))
FALLBACK = C13.FALLBACK

# The one file cycle 14 clears whole.
CLEARED_BY_CYCLE14 = ("src/rules_core/race_resolver.rs",)

# The three hits cycle 14 genuinely closed, by file and symbol. A later cycle that
# reintroduces any of these on the live side fails this census.
CLOSED_BY_CYCLE14 = (
    ("src/rules_core/race_resolver.rs", "ingest_payload::{RaceCacheData, RaceTraitCacheData}"),
    ("src/rules_core/race_resolver.rs", "race_trait_tokens"),
    ("src/rules_core/race_resolver.rs", "bonus_chain_reader::{self, DeclaredBonuses}"),
)

# The one hit cycle 14 relabelled, named so it can never be counted as a closure: the
# boundary call moved from `race_resolver` into `corpus_loader`, which already had one.
RELABELLED_BY_CYCLE14 = (
    ("src/rules_core/corpus_loader.rs", "race_resolver's own boundary call", "corpus_json_boundary"),
)


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

    live_parsers = [r for r in rows if "parse_lst_spell_row" in r["text"]
                    or "parse_equipment_entries" in r["text"]]
    assert not live_parsers, f"a live path still parses a raw PCGen row: {live_parsers}"
    print("live_runtime_row_parse_hits=0 (held from cycle 10)")

    equip_conversions = [r for r in rows if "convert_equipment_record" in r["text"]]
    assert not equip_conversions, (
        f"a live path still calls the equipment converter directly: {equip_conversions}"
    )
    print("runtime_equipment_conversion_hits=0 (held from cycle 13)")

    hit_files = {r["file"] for r in rows}

    # THE CLOSURE ASSERTION for this cycle: the race resolver names no converter at all.
    for f in CLEARED_BY_CYCLE14:
        assert f not in hit_files, f"cycle 14 cleared {f} and it is hit again"
    print(f"closed_by_cycle14={len(CLOSED_BY_CYCLE14)} (by file and symbol) "
          f"relabelled_by_cycle14={len(RELABELLED_BY_CYCLE14)} "
          f"cleared_files_by_cycle14={len(CLEARED_BY_CYCLE14)}")

    # Every earlier cycle's cleared files must STAY clear.
    for label, cleared in (("cycle10", C13.C12.C11.C10.CLEARED_BY_CYCLE10),
                           ("cycle11", C13.C12.C11.CLEARED_BY_CYCLE11),
                           ("cycle12", C13.C12.CLEARED_BY_CYCLE12)):
        still = [f for f in cleared if f in hit_files]
        assert not still, f"{label} cleared these files and they are hit again: {still}"
    print(f"cleared_by_cycle10={len(C13.C12.C11.C10.CLEARED_BY_CYCLE10)} (still clear) "
          f"cleared_by_cycle11={len(C13.C12.C11.CLEARED_BY_CYCLE11)} (still clear) "
          f"cleared_by_cycle12={len(C13.C12.CLEARED_BY_CYCLE12)} (still clear)")

    # Cycle 13's payload facts must hold: the enum's Equipment/Spell variants stay converted
    # and the deleted readers stay deleted.
    payload_src = os.path.join(ROOT, "src/pcgen_import/source_content_payload.rs")
    with open(payload_src, encoding="utf-8") as fh:
        payload_text = fh.read()
    assert "Equipment(&'a CorpusEquipmentRecord)," in payload_text
    assert "Equipment(&'a EquipmentRecord" not in payload_text
    borrowing = [v for v in ("Class(&'a ClassEntry)",
                             "SpellcastingClass(&'a SpellcastingClassEntry)",
                             "Race(&'a RaceDeclaration)",
                             "Ability(&'a AbilityDeclaration)",
                             "Metadata(&'a LstRecord)") if v in payload_text]
    assert len(borrowing) == 5, f"expected 5 parser-borrowing variants, found {borrowing}"
    print(f"equipment_payload=converted_only spell_payload=converted_only "
          f"parser_borrowing_variants={len(borrowing)}")

    # THE MOVE ITSELF, asserted by symbol on both sides of the boundary.
    moved_src = os.path.join(ROOT, "src/pcgen_import/corpus_race_json.rs")
    with open(moved_src, encoding="utf-8") as fh:
        moved_text = fh.read()
    moved_readings = (
        "race_trait_tokens::exclusion_guard_flags",
        "race_trait_tokens::negated_fact_gates",
        "race_trait_tokens::declares_preability_negated_guard",
        "race_trait_tokens::automatic_ability_grants",
        "race_trait_tokens::skinwalker_change_shape_kin",
        "race_trait_tokens::positive_prefact_flag",
        "race_trait_tokens::declared_walk_speed_ft",
        "race_trait_tokens::declared_size",
        "race_trait_tokens::declared_vision_segments",
        "race_trait_tokens::adopted_race_pool_suffix",
        "race_trait_tokens::declared_template_bonus_languages",
        "bonus_chain_reader::declared_bonuses",
    )
    for needed in ("pub fn corpus_race_source_record",
                   "pub fn corpus_race_trait_source_record") + moved_readings:
        assert needed in moved_text, f"{needed} is missing from the converter-side reader"
    print(f"corpus_race_json=present token_readings_on_converter_side={len(moved_readings)}")

    # The settled records must carry NO ingest array, or the move is cosmetic.
    settled_src = os.path.join(ROOT, "src/rules_core/race_record.rs")
    with open(settled_src, encoding="utf-8") as fh:
        settled_text = fh.read()
    # CODE lines only -- ruling B14 (`decisions.md` §17): a doc comment naming the
    # converter-side reader that fills these structs is PROVENANCE, and the gate itself
    # stopped counting one. The same exclusion is applied here so this assertion measures
    # exactly what the gate measures.
    settled_code = "\n".join(
        ln for ln in settled_text.splitlines()
        if not ln.lstrip().startswith("//")
    )
    for banned in ("raw_tokens", "raw_bonus_chains", "pcgen_import"):
        assert banned not in settled_code, (
            f"the settled race records name {banned!r} in CODE; the move would be cosmetic"
        )
    for needed in ("pub struct CorpusRaceRecord", "pub struct CorpusRaceTraitRecord"):
        assert needed in settled_text, f"{needed} is missing"
    print("race_record=settled_only ingest_arrays_on_settled_records=0")

    # And the live resolver must name the converter nowhere at all -- the file-clearing claim,
    # asserted by text rather than inferred from the gate's file list.
    resolver_src = os.path.join(ROOT, "src/rules_core/race_resolver.rs")
    with open(resolver_src, encoding="utf-8") as fh:
        resolver_lines = fh.read().splitlines()
    cfg_skip = set()
    for a, b in G.cfg_test_ranges(resolver_lines):
        cfg_skip.update(range(a, b + 1))
    shipping_hits = [
        (i + 1, ln.strip())
        for i, ln in enumerate(resolver_lines)
        if i not in cfg_skip and not ln.lstrip().startswith("//") and "pcgen_import" in ln
    ]
    assert not shipping_hits, f"race_resolver still names the converter: {shipping_hits}"
    print("race_resolver_shipping_pcgen_import_hits=0 (was 3)")

    out = os.path.join(HERE, "AT-35-E6-003-RULED_cycle14_runtime_import_census.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump({"total_hits": total, "total_files": len(files),
                   "by_root": dict(by_root), "by_group": sizes, "rows": rows,
                   "closed_by_cycle14": [list(x) for x in CLOSED_BY_CYCLE14],
                   "relabelled_by_cycle14": [list(x) for x in RELABELLED_BY_CYCLE14],
                   "cleared_files_by_cycle14": list(CLEARED_BY_CYCLE14)},
                  fh, indent=2)
    print(f"census written to {os.path.relpath(out, ROOT)}")


if __name__ == "__main__":
    main()
