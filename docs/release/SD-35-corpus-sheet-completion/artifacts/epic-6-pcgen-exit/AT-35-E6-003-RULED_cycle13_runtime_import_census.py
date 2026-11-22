#!/usr/bin/env python3
"""AT-35-E6-003-RULED cycle 13 — the run-time ``pcgen_import`` census, re-derived.

Cycle 12's census is imported whole (which imports cycle 11's, … back to cycle 1's): the
``GROUPS`` table, ``classify``, and the two exclusions the gate itself applies (B14 comment
lines, B15 ``#[cfg(test)]`` regions) stay single-sourced. **The gate script and
``scripts/pcgen-residue-baseline.env`` are absent from this cycle's diff** — no path
exempted, no regex weakened, no rebaseline.

What cycle 13 moved
-------------------

The ``SourceContentPayload::Equipment`` variant **collapsed to the converted half alone**.
Cycle 10 put the settled ``CorpusEquipmentRecord`` beside the parser row and wrote the
transition down in the variant's own doc comment: the row stays "for the consumers that
have not moved yet, and it goes when the last of them reads a settled value instead".
Cycle 12 moved the last two value readers; cycle 13 moved the two that were left, neither
of which read a rules value at all:

* ``src/rules_core/equipment_resolver.rs`` — ``equipment_key_token`` is **deleted**. It
  read a parser row's ``KEY:`` token to answer the KEY-or-name identity rule, which has been
  settled as ``CorpusEquipmentRecord::identity`` since cycle 10. ``equipment_id_resolve``
  answers with the settled record now instead of the ingest row, and cycle 12's
  ``equipment_converted_resolve_with_cell`` — which became its exact duplicate once the row
  was gone — is **deleted** rather than kept as an alias.
* ``src/rules_core/corpus_loader.rs`` — the loader stopped **rebuilding** an ingest row. It
  read the record's own ingest token array and bonus-chain array
  (``ingest_record::token_pairs`` / ``bonus_chain_qualifiers`` / ``rebuild_bonus_token``),
  assembled an ``EquipmentRecord`` out of them and ran ``ir_converter`` over that row — five
  hits, all on the live side. That whole function moved verbatim to
  ``src/pcgen_import/corpus_equipment_json.rs``, where the field names, the traversal and the
  ``BONUS:`` re-spelling belong (``decisions.md`` §11).

Five closures and one relabel, booked separately
------------------------------------------------

This cycle does **not** claim six. ``corpus_loader`` still asks the converter one question —
*"what canonical record does this corpus JSON object stand for?"* — and that one call is a
**relabel**: the hit moved from the ``ir_converter`` group to this census's new
``corpus_json_boundary`` group, in the same file. The other five hits are closures: three
token-string reads and two parser-row type imports genuinely stopped being named by live
code, and do not reappear anywhere else under a live root. Six hits out, one hit back:
net ``19 → 14``.

**No file is cleared this cycle, and the census says so.** ``corpus_loader`` keeps its one
boundary call and ``equipment_resolver`` keeps its ``SourceContentPayload`` import, so
``live_files`` stays at 8 while ``live_hits`` falls by 5. A cycle that moves hits without
clearing a file must say which, and this one does.

Parity
------

The three whole-corpus parity proofs cycles 10, 11 and 12 built are **kept, unchanged, and
still compare against the ingest row** — they just get the pairing from the converter side
now (``corpus_equipment_json::every_live_corpus_equipment_pair``) rather than from the
canonical envelope, because the envelope no longer carries a row to compare against. Same
population, same records, same assertions. ``decisions.md`` §11 keeps the converter, the
parser and the oracle harness; this is exactly that.
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
    "at35_e6_003_ruled_cycle12_census",
    os.path.join(HERE, "AT-35-E6-003-RULED_cycle12_runtime_import_census.py"),
)
C12 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(C12)

CORPUS_JSON_BOUNDARY_WHY = (
    "THE INGEST BOUNDARY, and after cycle 13 it is ONE call. "
    "`rules_core::corpus_loader::load_equipment_corpus` reads an already-converted "
    "`data/corpus/<book>/equipment/**/*.json` record's `data` object and asks "
    "`pcgen_import::corpus_equipment_json::corpus_equipment_source_record` what canonical "
    "record that object stands for. BEFORE CYCLE 13 the live loader did that work itself: "
    "it read the record's own ingest token array and bonus-chain array through "
    "`ingest_record::token_pairs` / `bonus_chain_qualifiers` / `rebuild_bonus_token`, "
    "assembled an ingest-format `EquipmentRecord` out of them, and ran "
    "`ir_converter::convert_equipment_record` over that row -- FIVE hits. The rebuild moved "
    "verbatim to the converter side, where the field names, the traversal and the `BONUS:` "
    "re-spelling are the converter's own vocabulary (`decisions.md` §11). "
    "THIS ONE HIT IS BOOKED AS A RELABEL, NOT A CLOSURE: the other five are gone from live "
    "code entirely, this one changed which group names it. It clears when "
    "`data/corpus/` equipment JSON carries the settled fields itself -- an ingest-side "
    "generator cycle, the same shape `data/sheet_rules/` already has -- so the live loader "
    "deserializes settled values and never needs an ingest array rebuilt at all. "
    "Re-derive: `grep -rn 'corpus_equipment_json' --include=*.rs src/rules_core/`."
)

LST_PARSER_TYPES_WHY_CYCLE13 = (
    "CLEARED BY CYCLE 13. Both remaining imports are gone: `equipment_resolver` stopped "
    "naming `EquipmentRecord` when `equipment_key_token` was DELETED (the KEY-or-name "
    "identity rule it computed has been settled as `CorpusEquipmentRecord::identity` since "
    "cycle 10), and `corpus_loader` stopped naming it when the ingest-row rebuild moved to "
    "`pcgen_import::corpus_equipment_json`. The `SourceContentPayload::Equipment` variant "
    "carries the settled record ALONE, so no live signature hands a parser row out any "
    "more. Re-derive: "
    "`grep -rln 'lst_parser::' --include=*.rs src/rules_core/`."
)

INGEST_RECORD_TOKENS_WHY_CYCLE13 = (
    "Run-time token-string reads. CYCLE 13 CLOSED `corpus_loader`'s THREE: `token_pairs`, "
    "`bonus_chain_qualifiers` and `rebuild_bonus_token` are read on the converter side now, "
    "inside `corpus_equipment_json::equipment_record_from_json`, which is the same function "
    "body moved whole. TWO REMAIN, neither of them a corpus-equipment read: "
    "`race_resolver`'s `RaceCacheData`/`RaceTraitCacheData` payload shapes, which ride on "
    "the race-trait rule shape this epic's remaining piece produces, and "
    "`derived_evaluator_fixture_check.rs`, which stays MEASURED non-relocatable (cycle 3) "
    "-- three desktop catalogs import live rendering functions out of it and its token "
    "reads sit inside the private corpus sweeps those imports reach. Re-derive: "
    "`grep -rn 'ingest_record\\|ingest_payload' --include=*.rs src/rules_core/`."
)

IR_CONVERTER_WHY_CYCLE13 = (
    "CLEARED BY CYCLE 13. The one remaining live conversion call -- `corpus_loader`'s "
    "`convert_equipment_record` -- is made on the converter side now, from inside "
    "`corpus_equipment_json::corpus_equipment_source_record`. The live loader asks one "
    "question and names no converter entry point. The call itself did not vanish and this "
    "census does not pretend it did: it is booked in `corpus_json_boundary` above as a "
    "relabel. Re-derive: `grep -rn 'ir_converter::' --include=*.rs src/rules_core/`."
)

SOURCE_CONTENT_PAYLOAD_WHY_CYCLE13 = (
    "Payload types imported or re-exported by the live side. THREE HITS, unchanged by cycle "
    "13, and all three are `SourceContentPayload` itself: `equipment_resolver`, "
    "`source_content`'s re-export, and `spell_resolver`. The trim -- repointing an import "
    "at `rules_core::source_content`'s own re-export of the same enum -- is REFUSED NOW FOR "
    "THE SEVENTH CYCLE RUNNING on the same reasoning cycles 7 through 12 gave: it takes the "
    "gate down by one and changes nothing about what the module depends on. "
    "CYCLE 13 MOVED THE REAL THING INSTEAD: the `Equipment` variant no longer borrows a "
    "parser entry type, so equipment joins cycle 8's spell as a kind whose canonical "
    "payload names no `pcgen_import` type. FOUR of the enum's seven variants still borrow "
    "one (`Class`, `SpellcastingClass`, `Race`/`Ability`, `Metadata`), which is why the "
    "enum itself cannot move yet. Re-derive: `grep -rn "
    "'source_content_payload\\|equipment_bonus_reader' --include=*.rs src/rules_core/`."
)

_REWRITTEN = {
    "lst_parser_types": LST_PARSER_TYPES_WHY_CYCLE13,
    "ingest_record_tokens": INGEST_RECORD_TOKENS_WHY_CYCLE13,
    "ir_converter": IR_CONVERTER_WHY_CYCLE13,
    "source_content_payload": SOURCE_CONTENT_PAYLOAD_WHY_CYCLE13,
}

# The new group goes FIRST, so the boundary call is classified as itself rather than
# absorbed by a broader pattern. Nothing else about the table changes.
GROUPS = [("corpus_json_boundary", re.compile(r"corpus_equipment_json"),
           CORPUS_JSON_BOUNDARY_WHY)] + [
    (gid, rx, _REWRITTEN.get(gid, why)) for gid, rx, why in C12.GROUPS
]
FALLBACK = C12.FALLBACK

# Cycle 13 clears NO FILE. Both files it worked on keep one hit each. Stated explicitly so
# the receipt cannot round it up.
CLEARED_BY_CYCLE13 = ()

# The four hits cycle 13 genuinely closed, by file and symbol. A later cycle that
# reintroduces any of these on the live side fails this census.
CLOSED_BY_CYCLE13 = (
    ("src/rules_core/corpus_loader.rs", "ingest_record::token_pairs"),
    ("src/rules_core/corpus_loader.rs", "ingest_record::bonus_chain_qualifiers"),
    ("src/rules_core/corpus_loader.rs", "ingest_record::rebuild_bonus_token"),
    ("src/rules_core/corpus_loader.rs", "lst_parser::equipment::"),
    ("src/rules_core/equipment_resolver.rs", "lst_parser::equipment::EquipmentRecord"),
)

# The one hit cycle 13 relabelled, named so it can never be counted as a closure.
RELABELLED_BY_CYCLE13 = (
    ("src/rules_core/corpus_loader.rs", "ir_converter", "corpus_json_boundary"),
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

    # Cycle 12 asserted exactly one live `convert_equipment_record`. Cycle 13 moved it to
    # the converter side, so the live count is now ZERO and the assertion inverts.
    equip_conversions = [r for r in rows if "convert_equipment_record" in r["text"]]
    assert not equip_conversions, (
        f"a live path still calls the equipment converter directly: {equip_conversions}"
    )
    print("runtime_equipment_conversion_hits=0 (was 1, moved to the converter side)")

    # The four closures must actually be gone from live code, by file AND symbol.
    live_text = {}
    for _root, rel, abs_path in G._iter_live_source_files(ROOT):
        with open(abs_path, encoding="utf-8", errors="replace") as fh:
            lines = fh.read().splitlines()
        skip = set()
        for a, b in G.cfg_test_ranges(lines):
            skip.update(range(a, b + 1))
        live_text[rel] = "\n".join(
            ln for i, ln in enumerate(lines)
            if i not in skip and not ln.lstrip().startswith("//")
        )
    for rel, symbol in CLOSED_BY_CYCLE13:
        body = live_text.get(rel, "")
        assert symbol not in body, (
            f"cycle 13 closed `{symbol}` in {rel} and it is back in shipping code"
        )
    print(f"closed_by_cycle13={len(CLOSED_BY_CYCLE13)} (by file and symbol) "
          f"relabelled_by_cycle13={len(RELABELLED_BY_CYCLE13)} "
          f"cleared_files_by_cycle13={len(CLEARED_BY_CYCLE13)}")

    # Cycles 10's, 11's and 12's cleared files must STAY clear.
    hit_files = {r["file"] for r in rows}
    for label, cleared in (("cycle10", C12.C11.C10.CLEARED_BY_CYCLE10),
                           ("cycle11", C12.C11.CLEARED_BY_CYCLE11),
                           ("cycle12", C12.CLEARED_BY_CYCLE12)):
        still = [f for f in cleared if f in hit_files]
        assert not still, f"{label} cleared these files and they are hit again: {still}"
    print(f"cleared_by_cycle10={len(C12.C11.C10.CLEARED_BY_CYCLE10)} (still clear) "
          f"cleared_by_cycle11={len(C12.C11.CLEARED_BY_CYCLE11)} (still clear) "
          f"cleared_by_cycle12={len(C12.CLEARED_BY_CYCLE12)} (still clear)")

    # The payload variant must be the SINGLE-ARM converted one, and the enum must still
    # hold the four variants that genuinely borrow a parser entry type -- otherwise the
    # refusal in `source_content_payload` above is no longer true.
    payload_src = os.path.join(ROOT, "src/pcgen_import/source_content_payload.rs")
    with open(payload_src, encoding="utf-8") as fh:
        payload_text = fh.read()
    assert "Equipment(&'a CorpusEquipmentRecord)," in payload_text, (
        "the Equipment payload variant is not the single-arm converted one"
    )
    assert "Equipment(&'a EquipmentRecord" not in payload_text, (
        "the Equipment payload variant still borrows the ingest-format parser row"
    )
    borrowing = [v for v in ("Class(&'a ClassEntry)",
                             "SpellcastingClass(&'a SpellcastingClassEntry)",
                             "Race(&'a RaceDeclaration)",
                             "Ability(&'a AbilityDeclaration)",
                             "Metadata(&'a LstRecord)") if v in payload_text]
    assert len(borrowing) == 5, f"expected 5 parser-borrowing variants, found {borrowing}"
    print(f"equipment_payload=converted_only spell_payload=converted_only "
          f"parser_borrowing_variants={len(borrowing)}")

    # The live resolver's parser-row reader is deleted, and cycle 12's duplicate alias
    # went with it rather than being kept as a pass-through.
    resolver_src = os.path.join(ROOT, "src/rules_core/equipment_resolver.rs")
    with open(resolver_src, encoding="utf-8") as fh:
        resolver_text = fh.read()
    assert "pub fn equipment_key_token" not in resolver_text, (
        "the parser-row KEY reader is back; cycle 13 deleted it"
    )
    assert "pub fn equipment_converted_resolve_with_cell" not in resolver_text, (
        "cycle 12's with-cell alias became an exact duplicate of `equipment_id_resolve` "
        "once the row left the envelope, and cycle 13 deleted it rather than keeping a "
        "pass-through"
    )
    for needed in ("pub fn equipment_id_resolve", "pub fn equipment_converted_resolve"):
        assert needed in resolver_text, f"{needed} is missing from the live resolver"
    print("equipment_key_token=deleted equipment_converted_resolve_with_cell=deleted "
          "equipment_id_resolve=converted")

    # The moved reader must exist on the converter side and be the ONLY home of the three
    # ingest-array reads it took with it.
    moved_src = os.path.join(ROOT, "src/pcgen_import/corpus_equipment_json.rs")
    with open(moved_src, encoding="utf-8") as fh:
        moved_text = fh.read()
    for needed in ("pub fn corpus_equipment_source_record",
                   "pub fn equipment_record_from_json",
                   "ingest_record::token_pairs",
                   "ingest_record::bonus_chain_qualifiers",
                   "ingest_record::rebuild_bonus_token"):
        assert needed in moved_text, f"{needed} is missing from the converter-side reader"
    print("corpus_equipment_json=present ingest_array_reads_on_converter_side=3")

    out = os.path.join(HERE, "AT-35-E6-003-RULED_cycle13_runtime_import_census.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump({"total_hits": total, "total_files": len(files),
                   "by_root": dict(by_root), "by_group": sizes, "rows": rows,
                   "closed_by_cycle13": [list(x) for x in CLOSED_BY_CYCLE13],
                   "relabelled_by_cycle13": [list(x) for x in RELABELLED_BY_CYCLE13],
                   "cleared_files_by_cycle13": list(CLEARED_BY_CYCLE13)},
                  fh, indent=2)
    print(f"census written to {os.path.relpath(out, ROOT)}")


if __name__ == "__main__":
    main()
