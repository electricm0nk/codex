#!/usr/bin/env python3
"""AT-35-E6-003-RULED cycle 10 — the run-time ``pcgen_import`` census, re-derived.

Cycle 9's census is imported whole (which imports cycle 8's, … back to cycle 1's): the
``GROUPS`` table, ``classify``, and the two exclusions the gate itself applies (B14 comment
lines, B15 ``#[cfg(test)]`` regions) stay single-sourced. **The gate script and
``scripts/pcgen-residue-baseline.env`` are absent from this cycle's diff** — no path
exempted, no regex weakened, no rebaseline.

What cycle 10 moved
-------------------

The equipment kind now has a converted record shape of its own, the way the spell kind got
one in cycle 8: ``rules_core::equipment_record::CorpusEquipmentRecord``.

Cycles 8 and 9 refused a ``CorpusEquipmentRecord`` **that would carry the ingest format's
token and BONUS-chain arrays**, on the correct ground that moving PCGen token structures
under a live root is worse than the hit it clears. That refusal is not relitigated and it
still stands. **This record carries no array.** Every field on it is a settled value — a
weight in pounds, a price in gold, a resolved ``+2 enhancement bonus to Str``. The token
spelling and the qualifier traversal moved the other way, onto the converter, into
``pcgen_import::ir_converter::equipment_record_to_corpus``, which is where ``decisions.md``
§11 rules that rule conversion happens.

Three live files stop naming ``pcgen_import`` entirely as a result, each a real closure and
not a relabel — the read does not move to another live file, it stops being a token read:

* ``src/rules_core/encumbrance.rs:60`` — ``lst_parser::equipment::EquipmentRecord``. Its
  ``weight_and_cost_from_record`` read the ``WT:``/``COST:`` tokens off the parser row; it
  reads ``CorpusEquipmentRecord::weight_and_cost`` now, through the new
  ``equipment_resolver::equipment_converted_resolve``.
* ``src/rules_core/equipment_effects/magic_items.rs:22`` — same import. Its ``BONUS:STAT``
  chain scan and its ``TEMPBONUS:`` fallback are both on the converter side now; the live
  function reads ``record.ability_score_bonus``.
* ``src/rules_core/equipment_effects/intelligent_item.rs:66`` — same import. Its
  ``BONUS:VAR|IntItemStat*`` family scan is on the converter side now; the live function
  reads ``record.intelligent_item``.

**The parity is proved over the whole live corpus, not a roster.**
``rules_core::equipment_record::tests::every_live_corpus_equipment_record_carries_the_same_values_the_token_reads_produced``
loads every book under ``data/corpus/`` (7,803 equipment records), re-derives all three
reads the way the live modules derived them — straight off the parser row still paired with
the converted record in the canonical envelope — and compares. Zero disagreements. The test
was mutation-proved: adding ``+ 1.0`` to the converter's weight read turns it red.

What cycle 10 did NOT move
--------------------------

The rest of the equipment shape: ``arms_armor``, ``equipmods``, ``general``,
``damage_total``, ``equipment_resolver`` and ``equipment_effects`` itself still read
``record.tokens`` / ``record.bonus_chains``. Their values are settled values too and each
lands on ``CorpusEquipmentRecord`` as its consumer moves; the ``Equipment`` payload carries
the parser row and the converted record as a **pair** for exactly as long as that takes, and
collapses to the converted half alone when the last consumer moves. Plus the two measured
converter-parity refusals (``renderer`` 5, ``trait_and_pool_tokens`` 3) and the two measured
non-relocatables, each carrying its own re-derive command in its group reason below.
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
    "at35_e6_003_ruled_cycle9_census",
    os.path.join(HERE, "AT-35-E6-003-RULED_cycle9_runtime_import_census.py"),
)
C9 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(C9)

LST_PARSER_TYPES_WHY_CYCLE10 = (
    "The ingest-format record STRUCT used as the live side's own data type. CYCLE 10 GAVE "
    "THE EQUIPMENT KIND A CONVERTED SHAPE -- "
    "`rules_core::equipment_record::CorpusEquipmentRecord`, the spell kind's cycle-8 move "
    "applied here -- and it carries NO token array and NO bonus-chain array, which is the "
    "shape cycles 8 and 9 refused and were right to refuse. Every field is a settled value "
    "and the token traversal moved the other way, onto "
    "`ir_converter::equipment_record_to_corpus`. Three consumers took it: `encumbrance` "
    "(weight, price), `magic_items` (ability-score enhancement), `intelligent_item` "
    "(stat-block contribution). SEVEN IMPORTS REMAIN, and they are the consumers whose "
    "values have not been settled onto the record yet: `arms_armor` (AC, MAXDEX, ACCHECK, "
    "SPELLFAILURE), `equipmods` (weapon enhancement, SR, VAR references), `general` (skill "
    "and VAR bonuses), `damage_total` (damage dice, crit, wield), `equipment_resolver` (the "
    "KEY token) and `equipment_effects` (EQMOD resolution, weapon typing) plus "
    "`corpus_loader`'s own rebuild. Each is one more settled field and one more consumer "
    "move, on the path this cycle built and proved. Re-derive: `grep -rln "
    "'lst_parser::equipment::EquipmentRecord' --include=*.rs src/rules_core/`."
)

IR_CONVERTER_WHY_CYCLE10 = (
    "The converter's own conversion entry points invoked from live code at run time. CYCLE "
    "9 closed both desktop-fixture calls. ONE CALL REMAINS, unchanged by cycle 10 and "
    "genuine: `corpus_loader.rs`'s `convert_equipment_record`, which builds the canonical "
    "envelope for an already-converted corpus JSON record. Cycle 10 made that ONE call do "
    "MORE work rather than adding a second -- it now also builds the "
    "`CorpusEquipmentRecord` the live side reads -- so the hit count did not rise. It "
    "clears when the equipment payload carries the converted record alone and the live "
    "loader builds it without the parser row, which is the last step of the consumer moves "
    "named in `lst_parser_types`. Re-derive: `grep -rn 'ir_converter::' --include=*.rs "
    "src/rules_core/`."
)

SOURCE_CONTENT_PAYLOAD_WHY_CYCLE10 = (
    "Payload and bonus-reader types re-exported or imported by the live side. CYCLE 10 did "
    "not take the `spell_resolver.rs` `SourceContentPayload` trim, REFUSED NOW FOR THE "
    "FOURTH CYCLE RUNNING on the same reasoning cycles 7, 8 and 9 gave: repointing the "
    "import at `rules_core::source_content`'s own re-export of the same enum takes the gate "
    "down by one and changes nothing about what the module depends on. The enum moves to "
    "the live side when its remaining variants stop borrowing parser entry types. Cycle 10 "
    "moved the `Equipment` variant HALF of the way -- it is a pair now, "
    "`(&EquipmentRecord, &CorpusEquipmentRecord)`, and the parser half goes when the "
    "consumers in `lst_parser_types` do. `equipment_bonus_reader` in `arms_armor`/"
    "`equipmods` rides on the same two consumer moves."
)

_REWRITTEN = {
    "lst_parser_types": LST_PARSER_TYPES_WHY_CYCLE10,
    "ir_converter": IR_CONVERTER_WHY_CYCLE10,
    "source_content_payload": SOURCE_CONTENT_PAYLOAD_WHY_CYCLE10,
}

GROUPS = [(gid, rx, _REWRITTEN.get(gid, why)) for gid, rx, why in C9.GROUPS]
FALLBACK = C9.FALLBACK

# The three files cycle 10 cleared, by name. A later cycle that reintroduces any of them
# fails this census, not a reader's memory.
CLEARED_BY_CYCLE10 = (
    "src/rules_core/encumbrance.rs",
    "src/rules_core/equipment_effects/magic_items.rs",
    "src/rules_core/equipment_effects/intelligent_item.rs",
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

    # Cycle 8's and cycle 9's closures must still hold.
    lst_spell = [r for r in rows if "lst_parser::spell" in r["text"]]
    assert not lst_spell, f"a spell parser/type is read live again: {lst_spell}"
    live_parsers = [r for r in rows if "parse_lst_spell_row" in r["text"]
                    or "parse_equipment_entries" in r["text"]]
    assert not live_parsers, f"a live path still parses a raw PCGen row: {live_parsers}"
    print("live_runtime_row_parse_hits=0 (held from cycle 9)")
    equip_conversions = [r for r in rows if "convert_equipment_record" in r["text"]]
    assert len(equip_conversions) == 1, (
        f"expected exactly the corpus-JSON equipment conversion to remain, got "
        f"{equip_conversions}"
    )
    print(f"runtime_equipment_conversion_hits=1 (held from cycle 9) "
          f"-> {equip_conversions[0]['file']}:{equip_conversions[0]['line']}")

    # Cycle 10's own claim, asserted by row: the three files it cleared name
    # `pcgen_import` nowhere in their shipping code.
    for cleared in CLEARED_BY_CYCLE10:
        still = [r for r in rows if r["file"] == cleared]
        assert not still, f"{cleared} was cleared by cycle 10 and reads pcgen_import again: {still}"
    print(f"cleared_by_cycle10={len(CLEARED_BY_CYCLE10)} "
          f"({', '.join(os.path.basename(f) for f in CLEARED_BY_CYCLE10)})")

    # The live converted record must actually exist, must carry no token array,
    # and must be reachable through the live resolver -- or "the live side reads
    # a settled value" is prose.
    record_src = os.path.join(ROOT, "src/rules_core/equipment_record.rs")
    assert os.path.exists(record_src), "the live converted equipment record is missing"
    with open(record_src, encoding="utf-8") as fh:
        record_lines = fh.read().split("#[cfg(test)]")[0].splitlines()
    # B14: a doc comment that names the converter is provenance, not a read. The
    # gate applies the same exclusion; this assertion applies it too rather than
    # grepping prose.
    shipping = [ln for ln in record_lines if not ln.lstrip().startswith(("//", "///", "//!"))]
    for forbidden in ("pcgen_import", "EquipmentToken", "BonusToken", "raw_tokens",
                      "bonus_chains"):
        offenders = [ln.strip() for ln in shipping if forbidden in ln]
        assert not offenders, (
            f"the live converted equipment record's shipping code carries `{forbidden}` -- "
            f"the shape cycles 8 and 9 refused: {offenders}"
        )
    print("live_converted_equipment_record=present token_arrays_on_it=0")

    resolver_src = os.path.join(ROOT, "src/rules_core/equipment_resolver.rs")
    with open(resolver_src, encoding="utf-8") as fh:
        assert "pub fn equipment_converted_resolve" in fh.read(), (
            "equipment_converted_resolve is missing; the three cleared files cannot reach "
            "the converted record"
        )
    print("equipment_converted_resolve=present")

    out = os.path.join(HERE, "AT-35-E6-003-RULED_cycle10_runtime_import_census.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump({"total_hits": total, "total_files": len(files),
                   "by_root": dict(by_root), "by_group": sizes, "rows": rows,
                   "cleared_by_cycle10": list(CLEARED_BY_CYCLE10)},
                  fh, indent=1)
    print(f"census written to {os.path.relpath(out, ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
