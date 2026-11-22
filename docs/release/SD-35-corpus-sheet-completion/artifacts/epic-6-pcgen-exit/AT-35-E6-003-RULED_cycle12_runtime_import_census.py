#!/usr/bin/env python3
"""AT-35-E6-003-RULED cycle 12 — the run-time ``pcgen_import`` census, re-derived.

Cycle 11's census is imported whole (which imports cycle 10's, … back to cycle 1's): the
``GROUPS`` table, ``classify``, and the two exclusions the gate itself applies (B14 comment
lines, B15 ``#[cfg(test)]`` regions) stay single-sourced. **The gate script and
``scripts/pcgen-residue-baseline.env`` are absent from this cycle's diff** — no path
exempted, no regex weakened, no rebaseline.

What cycle 12 moved
-------------------

The two remaining ``lst_parser_types`` consumers that are *readers* rather than *builders*:

* ``src/rules_core/damage_total.rs`` — the whole weapon half. The base damage die
  (``DAMAGE:``), the stand-in item's identity (``BASEITEM:``), the wield category
  (``WIELD:``), the critical threat range (``CRITRANGE:``, a stated *width* the sheet prints
  as inclusive bounds), the critical multiplier (``CRITMULT:``) and the damage-size step a
  referenced modifier contributes (``BONUS:EQMWEAPON|DAMAGESIZE``) are settled on the record
  now. So is the record key every one of the six work-units reports: it is
  ``CorpusEquipmentRecord::identity``, the same KEY-or-name rule ``equipment_key_token``
  applied. The one thing that stayed live is the ``BASEITEM:`` *chase* — a corpus
  resolution, which the converter cannot do because it has no corpus.
* ``src/rules_core/equipment_effects.rs`` — the weapon typing and the parser-row half of the
  EQMOD pass. ``is_natural_attack_weapon`` and ``is_weapon_record`` are two settled booleans
  plus a presence flag (``is_natural_attack``, ``is_shield``, ``states_base_damage``);
  ``resolve_eqm_weightdiv_effect``'s divisor is ``weight_divisor``; and the parser-row
  ``eqmod_referenced_records`` is **deleted**, its last two callers having moved to
  ``eqmod_referenced_converted_records``.

Both are **closures, not relabels**: each file's token reading is deleted from the live
side, not moved to another live file. The census asserts it by file below.

**The parity is proved over the whole live corpus, not a roster.**
``rules_core::equipment_record::tests::every_live_corpus_equipment_record_carries_the_same_weapon_values_the_token_reads_produced``
loads every book under ``data/corpus/`` (7,803 equipment records), re-derives all ten moved
reads the way the live modules derived them — straight off the parser row still paired with
the converted record in the canonical envelope — and compares field for field, including
the ``f64``-then-narrow the weight divisor's dividend now goes through. Zero disagreements.
The test was mutation-proved: adding ``+ 1`` to the converter's critical-multiplier read
turns it red on 522 of 7,803 records, and the mutation was reverted and re-verified green.

What cycle 12 did NOT move
--------------------------

``equipment_resolver``'s and ``corpus_loader``'s own ``EquipmentRecord`` imports. They are
not readers of a rules value at all — they are the two halves of the *envelope*:
``corpus_loader`` REBUILDS a parser row out of the corpus JSON, and ``equipment_resolver``
names the type only in the signatures that hand that row out. They clear together, in one
move, when the ``Equipment`` payload collapses to the converted half alone — which is also
when ``ir_converter`` (1) and the ``equipment_resolver`` half of
``source_content_payload`` clear. Splitting that into two cycles would book a relabel.

Plus the two measured converter-parity refusals (``renderer`` 5, ``trait_and_pool_tokens``
3) and the two measured non-relocatables, each carrying its own re-derive command in its
group reason below.
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
    "at35_e6_003_ruled_cycle11_census",
    os.path.join(HERE, "AT-35-E6-003-RULED_cycle11_runtime_import_census.py"),
)
C11 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(C11)

LST_PARSER_TYPES_WHY_CYCLE12 = (
    "The ingest-format record STRUCT used as the live side's own data type. CYCLE 10 built "
    "the path -- `rules_core::equipment_record::CorpusEquipmentRecord`, carrying NO token "
    "array and NO bonus-chain array -- and named the remaining consumers as a SEQUENCE. "
    "CYCLE 11 walked three (`arms_armor`, `general`, `equipmods`). CYCLE 12 WALKED THE LAST "
    "TWO READERS: `damage_total` (base damage die, BASEITEM stand-in identity, WIELD, "
    "CRITRANGE, CRITMULT, the EQMWEAPON|DAMAGESIZE step, and the record key every work-unit "
    "reports) and `equipment_effects` (weapon typing, the EQM|WEIGHTDIV divisor, and the "
    "DELETION of the parser-row `eqmod_referenced_records`). TWO IMPORTS REMAIN and NEITHER "
    "IS A READER OF A RULES VALUE: `corpus_loader` REBUILDS a parser row out of the corpus "
    "JSON, and `equipment_resolver` names the type only in the signatures that hand that "
    "row out. They are one move, not two -- the `Equipment` payload collapsing to the "
    "converted half alone -- and that same move clears `ir_converter` (1) and the "
    "`equipment_resolver` half of `source_content_payload`. Taking either alone would book "
    "a relabel: the type would simply be named in the other file instead. Re-derive: "
    "`grep -rln 'lst_parser::equipment::EquipmentRecord' --include=*.rs src/rules_core/`."
)

SOURCE_CONTENT_PAYLOAD_WHY_CYCLE12 = (
    "Payload and bonus-reader types re-exported or imported by the live side. THREE HITS, "
    "unchanged by cycle 12, and all three are `SourceContentPayload` itself: "
    "`equipment_resolver`, `source_content`'s re-export, and `spell_resolver`. The "
    "`spell_resolver.rs` trim -- repointing its import at `rules_core::source_content`'s "
    "own re-export of the same enum -- is REFUSED NOW FOR THE SIXTH CYCLE RUNNING on the "
    "same reasoning cycles 7 through 11 gave: it takes the gate down by one and changes "
    "nothing about what the module depends on. The enum moves to the live side when its "
    "remaining variants stop borrowing parser entry types, which is the same payload "
    "collapse `lst_parser_types` names. Re-derive: `grep -rn "
    "'source_content_payload\\|equipment_bonus_reader' --include=*.rs src/rules_core/`."
)

_REWRITTEN = {
    "lst_parser_types": LST_PARSER_TYPES_WHY_CYCLE12,
    "source_content_payload": SOURCE_CONTENT_PAYLOAD_WHY_CYCLE12,
}

GROUPS = [(gid, rx, _REWRITTEN.get(gid, why)) for gid, rx, why in C11.GROUPS]
FALLBACK = C11.FALLBACK

# The two files cycle 12 cleared, by name, on top of cycles 10's and 11's three each. A
# later cycle that reintroduces any of them fails this census, not a reader's memory.
CLEARED_BY_CYCLE12 = (
    "src/rules_core/damage_total.rs",
    "src/rules_core/equipment_effects.rs",
)

# The settled fields cycle 12 added to the live converted record. Each is asserted present
# below, so "the live side reads a settled value" is a check, not a sentence.
SETTLED_FIELDS_ADDED_BY_CYCLE12 = (
    "base_damage_dice",
    "states_base_damage",
    "base_item",
    "wield_category",
    "critical_threat_range",
    "critical_multiplier",
    "damage_size_steps",
    "weight_divisor",
    "is_natural_attack",
    "is_shield",
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
    assert len(equip_conversions) == 1, (
        f"expected exactly the corpus-JSON equipment conversion to remain, got "
        f"{equip_conversions}"
    )
    print(f"runtime_equipment_conversion_hits=1 (held from cycle 10) -> "
          f"{equip_conversions[0]['file']}:{equip_conversions[0]['line']}")

    # Cycles 10's and 11's cleared files must STAY clear, and cycle 12's must be clear.
    hit_files = {r["file"] for r in rows}
    for label, cleared in (("cycle10", C11.C10.CLEARED_BY_CYCLE10),
                           ("cycle11", C11.CLEARED_BY_CYCLE11),
                           ("cycle12", CLEARED_BY_CYCLE12)):
        still = [f for f in cleared if f in hit_files]
        assert not still, f"{label} cleared these files and they are hit again: {still}"
    print(f"cleared_by_cycle10={len(C11.C10.CLEARED_BY_CYCLE10)} (still clear) "
          f"cleared_by_cycle11={len(C11.CLEARED_BY_CYCLE11)} (still clear) "
          f"cleared_by_cycle12={len(CLEARED_BY_CYCLE12)} "
          f"({', '.join(os.path.basename(f) for f in CLEARED_BY_CYCLE12)})")

    # The live converted record must still carry NO token array and NO bonus-chain array --
    # the shape cycles 8 and 9 refused -- and must carry cycle 12's settled fields.
    record_src = os.path.join(ROOT, "src/rules_core/equipment_record.rs")
    with open(record_src, encoding="utf-8") as fh:
        record_text = fh.read()
    record_lines = record_text.split("#[cfg(test)]")[0].splitlines()
    shipping = [ln for ln in record_lines if not ln.lstrip().startswith(("//", "///", "//!"))]
    for forbidden in ("pcgen_import", "EquipmentToken", "BonusToken", "raw_tokens",
                      "bonus_chains"):
        offenders = [ln.strip() for ln in shipping if forbidden in ln]
        assert not offenders, (
            f"the live converted equipment record's shipping code carries `{forbidden}` -- "
            f"the shape cycles 8 and 9 refused: {offenders}"
        )
    missing = [f for f in SETTLED_FIELDS_ADDED_BY_CYCLE12
               if not any(f"pub {f}:" in ln for ln in shipping)]
    assert not missing, f"cycle 12's settled fields are missing from the record: {missing}"
    print(f"live_converted_equipment_record=present token_arrays_on_it=0 "
          f"settled_fields_added_by_cycle12={len(SETTLED_FIELDS_ADDED_BY_CYCLE12)}")

    # The parser-row EQMOD resolver is GONE, and the converted one is the only one left.
    effects_src = os.path.join(ROOT, "src/rules_core/equipment_effects.rs")
    with open(effects_src, encoding="utf-8") as fh:
        effects_text = fh.read()
    assert "pub fn eqmod_referenced_converted_records" in effects_text, (
        "the converted-record EQMOD resolver is missing"
    )
    assert "pub fn eqmod_referenced_records" not in effects_text, (
        "the parser-row EQMOD resolver is back; cycle 12 deleted it"
    )
    print("eqmod_referenced_converted_records=present eqmod_referenced_records=deleted")

    # The resolver's converted-with-cell answer is what replaced cycle 11's pair resolve.
    resolver_src = os.path.join(ROOT, "src/rules_core/equipment_resolver.rs")
    with open(resolver_src, encoding="utf-8") as fh:
        resolver_text = fh.read()
    for needed in ("pub fn equipment_converted_resolve",
                   "pub fn equipment_converted_resolve_with_cell"):
        assert needed in resolver_text, f"{needed} is missing from the live resolver"
    assert "pub fn equipment_pair_resolve" not in resolver_text, (
        "cycle 11's pair resolve is back; its only caller stopped needing the parser row"
    )
    print("equipment_converted_resolve=present equipment_converted_resolve_with_cell=present "
          "equipment_pair_resolve=deleted")

    out = os.path.join(HERE, "AT-35-E6-003-RULED_cycle12_runtime_import_census.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump({"total_hits": total, "total_files": len(files),
                   "by_root": dict(by_root), "by_group": sizes, "rows": rows,
                   "cleared_by_cycle10": list(C11.C10.CLEARED_BY_CYCLE10),
                   "cleared_by_cycle11": list(C11.CLEARED_BY_CYCLE11),
                   "cleared_by_cycle12": list(CLEARED_BY_CYCLE12),
                   "settled_fields_added_by_cycle12":
                       list(SETTLED_FIELDS_ADDED_BY_CYCLE12)},
                  fh, indent=1)
    print(f"census written to {os.path.relpath(out, ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
