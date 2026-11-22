#!/usr/bin/env python3
"""AT-35-E6-003-RULED cycle 11 — the run-time ``pcgen_import`` census, re-derived.

Cycle 10's census is imported whole (which imports cycle 9's, … back to cycle 1's): the
``GROUPS`` table, ``classify``, and the two exclusions the gate itself applies (B14 comment
lines, B15 ``#[cfg(test)]`` regions) stay single-sourced. **The gate script and
``scripts/pcgen-residue-baseline.env`` are absent from this cycle's diff** — no path
exempted, no regex weakened, no rebaseline.

What cycle 11 moved
-------------------

Cycle 10 built the path and named the sequence: the equipment half is not one indivisible
piece, it is a sequence of consumer moves onto a record that carries **no token array**.
Cycle 11 walked three of them, the three cycle 10 named first:

* ``src/rules_core/equipment_effects/arms_armor.rs`` — the armour/shield stat contribution
  (AC, max Dex, arcane spell failure, armour check penalty) and the contribution a
  referenced modifier makes to a host item's AC. Both are settled on the record now
  (``stat_effect``, ``armor_class_chain_bonus``); the ``ACCHECK:``/``MAXDEX:``/
  ``SPELLFAILURE:`` token reads, the ``BONUS:COMBAT|AC`` chain scan with its
  circumstance-type exclusion, the ``TEMPBONUS:`` fallback and the ``BONUS:EQMARMOR|…``
  family scan are all on the converter side.
* ``src/rules_core/equipment_effects/general.rs`` — the circumstance bonus to one named
  skill (with PF1's automatic swim-speed ``+8`` folded in) and the flat bonuses to named
  rules variables. Settled as ``skill_check_bonus`` and ``var_bonuses``; the
  ``BONUS:SKILL``/``BONUS:VAR`` chain scans, the ``TEMPBONUS:`` single-skill fallback with
  its three wildcard exclusions, and the ``MOVE:`` swim-speed read are on the converter
  side.
* ``src/rules_core/equipment_effects/equipmods.rs`` — the weapon to-hit/damage enhancement
  (with its natural-attack and proficiency scopes, and the one real family whose magnitude
  names a sibling variable on the same record) and the flat Spell Resistance grant. Settled
  as ``weapon_enhancement`` and ``spell_resistance_bonus``.

All three are **closures, not relabels**: each file's token reading is deleted from the live
side, not moved to another live file. The fourth thing that moved — the ``EQMOD:``
attachment grammar — is why: ``CorpusEquipmentRecord::eqmod_references`` is a **list of item
identities**, not a token, and ``equipment_effects::eqmod_referenced_converted_records``
resolves that list exactly the way the live side resolved the token's own segments.

**The parity is proved over the whole live corpus, not a roster.**
``rules_core::equipment_record::tests::every_live_corpus_equipment_record_carries_the_same_armour_skill_and_weapon_values_the_token_reads_produced``
loads every book under ``data/corpus/`` (7,803 equipment records), re-derives all nine moved
reads the way the live modules derived them — straight off the parser row still paired with
the converted record in the canonical envelope — and compares field for field. Zero
disagreements. The test was mutation-proved: adding ``+ 1`` to the converter's Spell
Resistance read turns it red on 16 of 7,803 records, and the mutation was reverted and
re-verified green.

What cycle 11 did NOT move
--------------------------

``damage_total`` (damage dice, crit, wield), ``equipment_resolver`` (the key token),
``equipment_effects`` itself (weapon typing and the parser-row half of the EQMOD pass) and
``corpus_loader``'s own rebuild. Those four are the rest of the equipment sequence, on the
same built path. Plus the two measured converter-parity refusals (``renderer`` 5,
``trait_and_pool_tokens`` 3) and the two measured non-relocatables, each carrying its own
re-derive command in its group reason below.
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
    "at35_e6_003_ruled_cycle10_census",
    os.path.join(HERE, "AT-35-E6-003-RULED_cycle10_runtime_import_census.py"),
)
C10 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(C10)

LST_PARSER_TYPES_WHY_CYCLE11 = (
    "The ingest-format record STRUCT used as the live side's own data type. CYCLE 10 built "
    "the path -- `rules_core::equipment_record::CorpusEquipmentRecord`, carrying NO token "
    "array and NO bonus-chain array -- and named the remaining consumers as a SEQUENCE "
    "rather than one indivisible piece. CYCLE 11 WALKED THREE OF THEM: `arms_armor` (AC, "
    "MAXDEX, ACCHECK, SPELLFAILURE, and the referenced-modifier AC contribution), `general` "
    "(skill bonus and named-variable rows) and `equipmods` (weapon enhancement, Spell "
    "Resistance). FOUR IMPORTS REMAIN, and they are the consumers whose values have not "
    "been settled onto the record yet: `damage_total` (damage dice, crit, wield), "
    "`equipment_resolver` (the KEY token), `equipment_effects` (weapon typing and the "
    "parser-row half of the EQMOD pass) and `corpus_loader`'s own rebuild. Each is one more "
    "settled field and one more consumer move, on the path cycle 10 built and cycles 10 and "
    "11 have now proved corpus-wide twice. Re-derive: `grep -rln "
    "'lst_parser::equipment::EquipmentRecord' --include=*.rs src/rules_core/`."
)

SOURCE_CONTENT_PAYLOAD_WHY_CYCLE11 = (
    "Payload and bonus-reader types re-exported or imported by the live side. CYCLE 11 DID "
    "NOT take the `spell_resolver.rs` `SourceContentPayload` trim, REFUSED NOW FOR THE "
    "FIFTH CYCLE RUNNING on the same reasoning cycles 7, 8, 9 and 10 gave: repointing the "
    "import at `rules_core::source_content`'s own re-export of the same enum takes the gate "
    "down by one and changes nothing about what the module depends on. The enum moves to "
    "the live side when its remaining variants stop borrowing parser entry types. CYCLE 11 "
    "CLEARED BOTH `equipment_bonus_reader` IMPORTS (`arms_armor`, `equipmods`): the two "
    "rules questions they asked it -- is this AC chain a circumstance bonus, does this roll "
    "chain carry the enhancement type -- are asked at ingest now, and the converter is "
    "still the only module that holds the answer's vocabulary. THREE HITS REMAIN, all "
    "`SourceContentPayload` itself: `equipment_resolver`, `source_content`'s re-export, and "
    "`spell_resolver`. Re-derive: `grep -rn 'source_content_payload\\|equipment_bonus_reader' "
    "--include=*.rs src/rules_core/`."
)

_REWRITTEN = {
    "lst_parser_types": LST_PARSER_TYPES_WHY_CYCLE11,
    "source_content_payload": SOURCE_CONTENT_PAYLOAD_WHY_CYCLE11,
}

GROUPS = [(gid, rx, _REWRITTEN.get(gid, why)) for gid, rx, why in C10.GROUPS]
FALLBACK = C10.FALLBACK

# The three files cycle 11 cleared, by name, on top of cycle 10's three. A later cycle that
# reintroduces any of them fails this census, not a reader's memory.
CLEARED_BY_CYCLE11 = (
    "src/rules_core/equipment_effects/arms_armor.rs",
    "src/rules_core/equipment_effects/general.rs",
    "src/rules_core/equipment_effects/equipmods.rs",
)

# The settled fields cycle 11 added to the live converted record. Each is asserted present
# below, so "the live side reads a settled value" is a check, not a sentence.
SETTLED_FIELDS_ADDED_BY_CYCLE11 = (
    "stat_effect",
    "armor_class_chain_bonus",
    "skill_check_bonus",
    "var_bonuses",
    "weapon_enhancement",
    "spell_resistance_bonus",
    "eqmod_references",
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
    print(f"runtime_equipment_conversion_hits=1 (held from cycle 10) "
          f"-> {equip_conversions[0]['file']}:{equip_conversions[0]['line']}")

    # Cycle 10's claim stays asserted -- a cycle may not quietly undo a predecessor's
    # closure -- and cycle 11's is asserted the same way.
    for cleared in C10.CLEARED_BY_CYCLE10:
        still = [r for r in rows if r["file"] == cleared]
        assert not still, f"{cleared} was cleared by cycle 10 and reads pcgen_import again: {still}"
    print(f"cleared_by_cycle10={len(C10.CLEARED_BY_CYCLE10)} (still clear)")
    for cleared in CLEARED_BY_CYCLE11:
        still = [r for r in rows if r["file"] == cleared]
        assert not still, f"{cleared} was cleared by cycle 11 and still reads pcgen_import: {still}"
    print(f"cleared_by_cycle11={len(CLEARED_BY_CYCLE11)} "
          f"({', '.join(os.path.basename(f) for f in CLEARED_BY_CYCLE11)})")

    # The live converted record must actually exist, must still carry no token array, and
    # must now carry this cycle's settled fields -- or "the live side reads a settled value"
    # is prose.
    record_src = os.path.join(ROOT, "src/rules_core/equipment_record.rs")
    assert os.path.exists(record_src), "the live converted equipment record is missing"
    with open(record_src, encoding="utf-8") as fh:
        record_text = fh.read()
    record_lines = record_text.split("#[cfg(test)]")[0].splitlines()
    # B14: a doc comment that names the converter is provenance, not a read. The gate
    # applies the same exclusion; this assertion applies it too rather than grepping prose.
    shipping = [ln for ln in record_lines if not ln.lstrip().startswith(("//", "///", "//!"))]
    for forbidden in ("pcgen_import", "EquipmentToken", "BonusToken", "raw_tokens",
                      "bonus_chains"):
        offenders = [ln.strip() for ln in shipping if forbidden in ln]
        assert not offenders, (
            f"the live converted equipment record's shipping code carries `{forbidden}` -- "
            f"the shape cycles 8 and 9 refused: {offenders}"
        )
    missing = [f for f in SETTLED_FIELDS_ADDED_BY_CYCLE11
               if not any(f"pub {f}:" in ln for ln in shipping)]
    assert not missing, f"cycle 11's settled fields are missing from the record: {missing}"
    print(f"live_converted_equipment_record=present token_arrays_on_it=0 "
          f"settled_fields_added_by_cycle11={len(SETTLED_FIELDS_ADDED_BY_CYCLE11)}")

    # The list of attached-modifier identities must be a list of IDENTITIES, resolved by
    # the live resolver -- not the attachment token handed across the boundary.
    effects_src = os.path.join(ROOT, "src/rules_core/equipment_effects.rs")
    with open(effects_src, encoding="utf-8") as fh:
        effects_text = fh.read()
    assert "pub fn eqmod_referenced_converted_records" in effects_text, (
        "the converted-record sibling of eqmod_referenced_records is missing; the three "
        "cleared files cannot resolve an attached modifier"
    )
    print("eqmod_referenced_converted_records=present")

    resolver_src = os.path.join(ROOT, "src/rules_core/equipment_resolver.rs")
    with open(resolver_src, encoding="utf-8") as fh:
        resolver_text = fh.read()
    for needed in ("pub fn equipment_converted_resolve", "pub fn equipment_pair_resolve"):
        assert needed in resolver_text, f"{needed} is missing from the live resolver"
    print("equipment_converted_resolve=present equipment_pair_resolve=present")

    out = os.path.join(HERE, "AT-35-E6-003-RULED_cycle11_runtime_import_census.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump({"total_hits": total, "total_files": len(files),
                   "by_root": dict(by_root), "by_group": sizes, "rows": rows,
                   "cleared_by_cycle10": list(C10.CLEARED_BY_CYCLE10),
                   "cleared_by_cycle11": list(CLEARED_BY_CYCLE11),
                   "settled_fields_added_by_cycle11":
                       list(SETTLED_FIELDS_ADDED_BY_CYCLE11)},
                  fh, indent=1)
    print(f"census written to {os.path.relpath(out, ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
