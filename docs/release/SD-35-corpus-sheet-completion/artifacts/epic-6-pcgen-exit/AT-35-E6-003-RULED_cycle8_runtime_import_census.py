#!/usr/bin/env python3
"""AT-35-E6-003-RULED cycle 8 — the run-time ``pcgen_import`` census, re-derived.

Cycle 7's census is imported whole (which imports cycle 6's, … back to cycle 1's): the
``GROUPS`` table, ``classify``, and the two exclusions the gate itself applies (B14 comment
lines, B15 ``#[cfg(test)]`` regions) stay single-sourced. **The gate script and
``scripts/pcgen-residue-baseline.env`` are absent from this cycle's diff** — no path
exempted, no regex weakened, no rebaseline.

What cycle 8 moved
------------------

Every census from cycle 1 on named the SAME blocker under three different group headings —
``lst_parser_types`` (*"the ingest-format record STRUCTS … used as the live side's own data
type … needs the live side to own a converted equipment/spell record shape, which does not
exist yet"*), ``ir_converter`` (*"the live side is RUNNING the conversion instead of reading
its output"*) and ``source_content_payload`` (*"shape-only dependencies … they move when the
converted shape above exists"*).

Cycle 8 builds that shape, for the spell kind, end to end:

* ``src/rules_core/spell_record.rs`` — new. ``CorpusSpellRecord``, the live side's own
  converted spell record. Field-for-field the content of a parsed spell row, minus the
  ingest format's own vocabulary: ``school`` is ``"Transmutation"``, ``casting_time`` is
  ``"1 standard action"``. The ``SCHOOL:``/``CASTTIME:`` column tags — the grammar — are
  stripped by the parser and appear nowhere on it.
* ``SourceContentPayload::Spell`` now borrows a ``&CorpusSpellRecord``, not the B-4 parser
  row. The conversion happens once, in the new converter-side
  ``ir_converter::spell_record_to_corpus`` — the ONLY reader of ``LstSpellRecord`` on this
  path — and the projection is proved TOTAL and LOSSLESS field by field
  (``tests/sd17_e_source_ir_shape.rs``, which is where the old ``std::ptr::eq`` zero-copy
  assertion stood; the replacement is a strictly stronger claim than pointer identity).
* ``SourceContentRecord::spell`` — new, LIVE. ``corpus_loader::load_spell_corpus`` reads
  ``data/corpus/<book>/spell/*.json``, which is ALREADY-CONVERTED corpus data, and builds
  the canonical envelope itself. It used to reconstruct the ingest-format parser struct from
  that JSON and hand it to the converter to be re-converted — the live side running the
  converter over the converter's own output.
* ``spell_resolver::spell_id_resolve`` returns ``&CorpusSpellRecord``. Nothing downstream of
  it (``spellbook``, ``pilot_compute_corpus``) names ``pcgen_import``; they read exactly two
  fields, ``name`` and ``school``. ``spell_resolver`` itself keeps one deliberate reference —
  see below.

**What that is worth, stated exactly** — two hits, both of them real:

* ``spell_resolver.rs:15`` — the ``LstSpellRecord`` type-ownership hit. **A closure.** The
  live side owns its spell record shape now.
* ``corpus_loader.rs:140`` — the run-time ``convert_spell_record`` call on the corpus-JSON
  path. **A closure.** The conversion is not deferred or hidden; it does not happen, because
  the data was already converted.
* ``spell_resolver.rs:16`` — ``SourceContentPayload`` imported from ``pcgen_import``.
  **DELIBERATELY NOT TAKEN.** Repointing it at ``rules_core::source_content``'s own re-export
  of the same enum would take the gate down by one and change nothing about what the module
  depends on; cycle 7 named that trim and refused it, and cycle 8 does not take it either.

What cycle 8 did NOT move, and why the equipment half is not the same job
-------------------------------------------------------------------------

The spell kind was tractable because its live consumers read two settled VALUES. The
equipment kind's nine live consumers read ``record.tokens`` and ``record.bonus_chains``
directly — the PCGen token arrays themselves. Giving the live side a
``CorpusEquipmentRecord`` carrying those arrays would move PCGen token structures UNDER a
live root, which is worse than the hit it would clear. The equipment half needs its
consumers to read converted ``SheetRule`` rows instead, which is this epic's remaining
piece and is not a relabelling of this one.
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
    "at35_e6_003_ruled_cycle7_census",
    os.path.join(HERE, "AT-35-E6-003-RULED_cycle7_runtime_import_census.py"),
)
C7 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(C7)

LST_PARSER_TYPES_WHY_CYCLE8 = (
    "The ingest-format record STRUCTS used as the live side's own data type. CYCLE 8 "
    "CLOSED THE SPELL HALF: `rules_core::spell_record::CorpusSpellRecord` is the live "
    "side's own converted spell record, `SourceContentPayload::Spell` borrows it instead "
    "of the B-4 parser row, and `spell_resolver` no longer names an ingest-format TYPE. "
    "The one-way map `ir_converter::spell_record_to_corpus` is the only reader of "
    "`LstSpellRecord` left on that path, and it is converter-side; its totality and "
    "losslessness are pinned field by field in `tests/sd17_e_source_ir_shape.rs`. WHAT IS "
    "LEFT IS THE EQUIPMENT HALF, and it is NOT the same job: the spell kind's live "
    "consumers read two settled VALUES (`name`, `school`), while `EquipmentRecord`'s nine "
    "live consumers read `record.tokens` and `record.bonus_chains` -- the PCGen token "
    "arrays themselves. A `CorpusEquipmentRecord` carrying those arrays would move PCGen "
    "token structures UNDER a live root, which is worse than the hit it clears. The "
    "equipment half clears when its consumers read converted `SheetRule` rows, which is "
    "this epic's remaining piece. `corpus_loader.rs:39`'s equipment import and its one "
    "remaining spell import (`parse_lst_spell_row`, for `load_lst_fixture_corpus`) are "
    "both that piece. Re-derive: `python3 docs/release/SD-35-corpus-sheet-completion/"
    "artifacts/epic-6-pcgen-exit/AT-35-E6-003-RULED_cycle8_runtime_import_census.py`."
)

IR_CONVERTER_WHY_CYCLE8 = (
    "The converter's own conversion entry points invoked from live code at run time. "
    "CYCLE 8 CLOSED `load_spell_corpus`'s call: `data/corpus/<book>/spell/*.json` is "
    "already-converted corpus data, so the loader now builds the canonical envelope itself "
    "through the new LIVE `SourceContentRecord::spell` and the conversion simply does not "
    "happen -- it was the live side running the converter over the converter's own output. "
    "THREE CALLS REMAIN, all of them genuine: `load_equipment_corpus` re-converts because "
    "the equipment half has no live record shape (see `lst_parser_types`), and "
    "`load_lst_fixture_corpus`'s two parse the desktop's bundled RAW `.lst` fixture lines "
    "at run time, which clears when that package is produced at build time and read as "
    "data."
)

SOURCE_CONTENT_PAYLOAD_WHY_CYCLE8 = (
    "Payload and bonus-reader types re-exported or imported by the live side. CYCLE 8 "
    "REFUSED the one trim available here, for the second cycle running: repointing "
    "`spell_resolver.rs`'s `SourceContentPayload` import at `rules_core::source_content`'s "
    "own re-export of the same enum would take the gate down by one and change nothing "
    "about what the module depends on. Cycle 7 named that trim and refused it; cycle 8 "
    "does not take it either, and the import carries a comment saying so. The enum moves "
    "to the live side when its remaining variants stop borrowing parser entry types -- the "
    "same equipment-shape work above."
)

_REWRITTEN = {
    "lst_parser_types": LST_PARSER_TYPES_WHY_CYCLE8,
    "ir_converter": IR_CONVERTER_WHY_CYCLE8,
    "source_content_payload": SOURCE_CONTENT_PAYLOAD_WHY_CYCLE8,
}

GROUPS = [(gid, rx, _REWRITTEN.get(gid, why)) for gid, rx, why in C7.GROUPS]
FALLBACK = C7.FALLBACK


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

    # The ingest-format spell record must be gone from the live side, or the
    # closure this cycle claims is not real. Nothing below is a regex over
    # prose: it is the census's own rows.
    lst_spell = [r for r in rows if "lst_parser::spell" in r["text"]
                 and "parse_lst_spell_row" not in r["text"]]
    assert not lst_spell, f"LstSpellRecord still read live: {lst_spell}"
    convert_on_corpus_json = [r for r in rows if "convert_spell_record(record)" in r["text"]]
    assert not convert_on_corpus_json, f"corpus-JSON spell path still converts: {convert_on_corpus_json}"
    print("live_LstSpellRecord_type_hits=0 (was 1 at cycle 7)")
    print("corpus_json_spell_conversion_hits=0 (was 1 at cycle 7)")

    out = os.path.join(HERE, "AT-35-E6-003-RULED_cycle8_runtime_import_census.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump({"total_hits": total, "total_files": len(files),
                   "by_root": dict(by_root), "by_group": sizes, "rows": rows},
                  fh, indent=1)
    print(f"census written to {os.path.relpath(out, ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
