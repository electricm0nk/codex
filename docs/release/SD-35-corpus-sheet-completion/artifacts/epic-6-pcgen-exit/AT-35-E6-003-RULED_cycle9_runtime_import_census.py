#!/usr/bin/env python3
"""AT-35-E6-003-RULED cycle 9 — the run-time ``pcgen_import`` census, re-derived.

Cycle 8's census is imported whole (which imports cycle 7's, … back to cycle 1's): the
``GROUPS`` table, ``classify``, and the two exclusions the gate itself applies (B14 comment
lines, B15 ``#[cfg(test)]`` regions) stay single-sourced. **The gate script and
``scripts/pcgen-residue-baseline.env`` are absent from this cycle's diff** — no path
exempted, no regex weakened, no rebaseline.

What cycle 9 moved
------------------

Every census from cycle 1 on named the clearing condition for the desktop fixture package in
the same words, and cycle 8's ``ir_converter`` reason states it verbatim: *"`load_lst_fixture
_corpus`'s two parse the desktop's bundled RAW `.lst` fixture lines at run time, which clears
when that package is produced at build time and read as data."*

It is produced at build time now.

* ``src/bin/gen_desktop_fixture_corpus.rs`` — new, converter-side. Reads the four ``.txt``
  fixtures, parses them with the LST parsers, and writes the converted records into
  ``apps/desktop/src-tauri/resources/corpus_fixtures/{spell,equipment}/*.json`` — the same
  ``BookCorpusRoot`` layout, and for equipment the same Shape B v1 ``data.raw_tokens`` /
  ``data.raw_bonus_chains`` form, that the real ``data/corpus/<book>/`` records carry.
  ``--check`` regenerates in memory and fails on byte drift.
* ``rules_core::corpus_loader::load_lst_fixture_corpus`` and ``LstFixtureLine`` are **gone**,
  replaced by ``load_book_corpus`` — a live two-kind loader that composes the two loaders
  that already existed and parses nothing.
* The desktop's ``corpus_fixtures.rs`` hands ``load_book_corpus`` a ``BookCorpusRoot`` and
  asserts the record count it committed to. It reads converted data; it does not convert.

**What that is worth, stated exactly** — three hits, all of them real closures, and NOT a
deferral wearing a closure's clothes: the conversion does not move elsewhere at run time, it
stops happening at run time.

* ``corpus_loader.rs:50`` — ``use …lst_parser::spell::parse_lst_spell_row``. The last live
  import of a PCGen row parser in the crate. **Gone.**
* ``corpus_loader.rs:515`` — ``ir_converter::convert_spell_record`` on the fixture path.
  **Gone.**
* ``corpus_loader.rs:528`` — ``ir_converter::convert_equipment_record`` on the fixture path.
  **Gone.**

The ``.txt`` fixtures stay on disk and stay bundled. They are the generator's INPUT, and
``decisions.md`` §11 keeps the converter, the parsers and their inputs — what it forbids is
the live side running them. Nothing was renamed to duck a regex and no path was exempted.

What cycle 9 did NOT move
-------------------------

The remaining 29 hits are the equipment-shape piece (``lst_parser_types`` 10,
``ir_converter`` 1, ``source_content_payload`` 5, three of ``ingest_record_tokens``), the
measured converter-parity refusals (``renderer`` 5, ``trait_and_pool_tokens`` 3) and the two
measured non-relocatables. Every one carries its own re-derive command in its group reason
below. The equipment half still needs its nine live consumers to read converted ``SheetRule``
rows rather than ``EquipmentRecord.tokens``; giving the live side a ``CorpusEquipmentRecord``
carrying those arrays would move PCGen token structures UNDER a live root, which is worse
than the hit it clears — cycle 8 established that and cycle 9 does not relitigate it.
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
    "at35_e6_003_ruled_cycle8_census",
    os.path.join(HERE, "AT-35-E6-003-RULED_cycle8_runtime_import_census.py"),
)
C8 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(C8)

LST_PARSER_TYPES_WHY_CYCLE9 = (
    "The ingest-format record STRUCTS used as the live side's own data type. CYCLE 9 "
    "CLOSED the last live PARSER import: `parse_lst_spell_row` was here for "
    "`load_lst_fixture_corpus`, which parsed the desktop's bundled raw `.lst` rows at run "
    "time. That package is produced at build time now by "
    "`src/bin/gen_desktop_fixture_corpus.rs` and read as data through the new live "
    "`corpus_loader::load_book_corpus`, so no live path in the crate parses a PCGen row. "
    "WHAT IS LEFT IS THE EQUIPMENT TYPE, ten imports of `EquipmentRecord`, and it is the "
    "same job cycle 8 named: the spell kind's live consumers read two settled VALUES "
    "(`name`, `school`), while `EquipmentRecord`'s nine live consumers read `record.tokens` "
    "and `record.bonus_chains` -- the PCGen token arrays themselves. A "
    "`CorpusEquipmentRecord` carrying those arrays would move PCGen token structures UNDER "
    "a live root, which is worse than the hit it clears. The equipment half clears when its "
    "consumers read converted `SheetRule` rows. Re-derive: `grep -rln "
    "'lst_parser::equipment::EquipmentRecord' --include=*.rs src/rules_core/`."
)

IR_CONVERTER_WHY_CYCLE9 = (
    "The converter's own conversion entry points invoked from live code at run time. CYCLE "
    "9 CLOSED BOTH FIXTURE CALLS: `load_lst_fixture_corpus` is gone, and with it the "
    "run-time `convert_spell_record` and `convert_equipment_record` on the desktop's "
    "bundled fixture path. The records ship already converted -- "
    "`apps/desktop/src-tauri/resources/corpus_fixtures/{spell,equipment}/*.json`, written "
    "by `src/bin/gen_desktop_fixture_corpus.rs`, verified byte-stable by its `--check`. "
    "ONE CALL REMAINS and it is genuine: `load_equipment_corpus` re-converts because the "
    "equipment half has no live record shape (see `lst_parser_types`). Re-derive: `cargo "
    "run --locked --bin gen_desktop_fixture_corpus -- --check`."
)

INGEST_RECORD_TOKENS_WHY_CYCLE9 = (
    C8._REWRITTEN.get("ingest_record_tokens")
    or next(why for gid, _rx, why in C8.GROUPS if gid == "ingest_record_tokens")
)

_REWRITTEN = {
    "lst_parser_types": LST_PARSER_TYPES_WHY_CYCLE9,
    "ir_converter": IR_CONVERTER_WHY_CYCLE9,
}

GROUPS = [(gid, rx, _REWRITTEN.get(gid, why)) for gid, rx, why in C8.GROUPS]
FALLBACK = C8.FALLBACK


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

    # Cycle 8's two closures must still hold — a later cycle must not quietly
    # reintroduce what an earlier one cleared.
    lst_spell = [r for r in rows if "lst_parser::spell" in r["text"]]
    assert not lst_spell, f"a spell parser/type is read live again: {lst_spell}"
    print("live_lst_parser_spell_hits=0 (was 1 at cycle 8: parse_lst_spell_row)")

    # Cycle 9's own claim, asserted by row, not by prose: no live code runs the
    # converter on the desktop's bundled fixture package any more, and nothing
    # in the crate parses a raw PCGen row at run time.
    fixture_conversion = [r for r in rows if "convert_spell_record" in r["text"]]
    assert not fixture_conversion, f"fixture spell conversion still live: {fixture_conversion}"
    live_parsers = [r for r in rows if "parse_lst_spell_row" in r["text"]
                    or "parse_equipment_entries" in r["text"]]
    assert not live_parsers, f"a live path still parses a raw PCGen row: {live_parsers}"
    print("live_runtime_row_parse_hits=0 (was 2 at cycle 8)")
    equip_conversions = [r for r in rows if "convert_equipment_record" in r["text"]]
    assert len(equip_conversions) == 1, (
        f"expected exactly the corpus-JSON equipment conversion to remain, got "
        f"{equip_conversions}"
    )
    print(f"runtime_equipment_conversion_hits=1 (was 2 at cycle 8) "
          f"-> {equip_conversions[0]['file']}:{equip_conversions[0]['line']}")

    # The generated package must actually exist and be the shape the live loader
    # reads, or the "produced at build time and read as data" claim is prose.
    fixtures = os.path.join(ROOT, "apps/desktop/src-tauri/resources/corpus_fixtures")
    shipped = sorted(
        os.path.relpath(os.path.join(d, f), fixtures)
        for d in (os.path.join(fixtures, "spell"), os.path.join(fixtures, "equipment"))
        for f in os.listdir(d)
        if f.endswith(".json")
    )
    assert len(shipped) == 4, f"expected 4 converted fixture records, found {shipped}"
    for rel in shipped:
        with open(os.path.join(fixtures, rel), encoding="utf-8") as fh:
            doc = json.load(fh)
        assert "key" in doc.get("data", {}), f"{rel} carries no data.key"
    print(f"shipped_converted_fixture_records={len(shipped)} ({', '.join(shipped)})")

    out = os.path.join(HERE, "AT-35-E6-003-RULED_cycle9_runtime_import_census.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump({"total_hits": total, "total_files": len(files),
                   "by_root": dict(by_root), "by_group": sizes, "rows": rows},
                  fh, indent=1)
    print(f"census written to {os.path.relpath(out, ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
