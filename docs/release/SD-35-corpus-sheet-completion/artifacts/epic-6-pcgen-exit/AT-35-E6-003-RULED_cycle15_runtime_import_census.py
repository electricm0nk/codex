#!/usr/bin/env python3
"""AT-35-E6-003-RULED cycle 15 — the run-time ``pcgen_import`` census, re-derived.

Cycle 14's census is imported whole (which imports cycle 13's, … back to cycle 1's): the
``GROUPS`` table, ``classify``, and the two exclusions the gate itself applies (B14 comment
lines, B15 ``#[cfg(test)]`` regions) stay single-sourced. **The gate script and
``scripts/pcgen-residue-baseline.env`` are absent from this cycle's diff** — no path
exempted, no regex weakened, no rebaseline.

What cycle 15 moved
-------------------

``src/rules_core/corpus_loader.rs`` — the live side's one ingest boundary, and the only file
left whose ``pcgen_import`` hits were **calls** rather than payload imports — is **cleared
whole**. It used to hold

* ``crate::pcgen_import::corpus_equipment_json::corpus_equipment_source_record(data)`` (the
  call, cycle 13), and
* ``use crate::pcgen_import::corpus_race_json`` (the import behind the two race boundary
  functions cycle 14 parked here).

Both cycles booked those honestly as the ``corpus_json_boundary`` group and both named the
clearing condition in the same words: *"it clears when ``data/corpus/`` carries the settled
fields itself — an ingest-side generator cycle, the same shape ``data/sheet_rules/`` already
has, after which the live loader deserializes settled values with serde."*

That is exactly what this cycle did, and nothing else:

* ``src/bin/gen_settled_corpus.rs`` runs **the same three calls**, unchanged, at authoring
  time and writes ``<book>/_settled/{equipment,race,race_trait}.json``;
* ``src/rules_core/settled_corpus.rs`` reads one with serde and names no converter;
* ``corpus_loader`` and ``race_resolver`` ask serde instead of the converter.

No corpus record file was rewritten — the bundles sit **beside** the kind directories, so
the ingested records, their license blocks and their ``pi_*`` stamps are untouched.

Two closures, no relabel
------------------------

Net ``12 → 10``, one file cleared (7 → 6), and this cycle books **no** relabel: the boundary
call did not move to another live file, it left the live side entirely for a build-time
producer. ``pcgen_live_files`` falls for the second cycle running.
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
    "at35_e6_003_ruled_cycle14_census",
    os.path.join(HERE, "AT-35-E6-003-RULED_cycle14_runtime_import_census.py"),
)
C14 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(C14)

CORPUS_JSON_BOUNDARY_WHY_CYCLE15 = (
    "CLEARED BY CYCLE 15. This group held the live side's last TWO converter CALLS -- "
    "`corpus_equipment_json::corpus_equipment_source_record` (cycle 13) and "
    "`corpus_race_json` (cycle 14) -- both in `rules_core::corpus_loader`, asking the "
    "converter at run time what settled record a corpus `data` object stands for. Cycles 13 "
    "and 14 both named the clearing condition as a DATA step, and cycle 15 performed it: "
    "`src/bin/gen_settled_corpus.rs` makes the SAME three calls, unchanged, at authoring "
    "time and writes `data/corpus/<book>/_settled/{equipment,race,race_trait}.json`; "
    "`rules_core::settled_corpus` reads one with serde; `corpus_loader` and `race_resolver` "
    "hold settled records and name nothing. NO corpus record file was rewritten -- the "
    "bundles sit BESIDE the kind directories, so license blocks and `pi_*` stamps are "
    "untouched. Re-derive: `grep -rn 'corpus_equipment_json\\|corpus_race_json' "
    "--include=*.rs src/rules_core/` (expect no output)."
)

_REWRITTEN = {"corpus_json_boundary": CORPUS_JSON_BOUNDARY_WHY_CYCLE15}

# The boundary group's regex is UNCHANGED from cycle 14 -- nothing renamed to duck it; it
# simply matches nothing under a live root any more. No group is dropped.
GROUPS = [
    (gid, rx, _REWRITTEN.get(gid, why))
    for gid, rx, why in C14.GROUPS
]
FALLBACK = C14.FALLBACK

# The one file cycle 15 clears whole.
CLEARED_BY_CYCLE15 = ("src/rules_core/corpus_loader.rs",)

# The two hits cycle 15 genuinely closed, by file and symbol. A later cycle that reintroduces
# either on the live side fails this census.
CLOSED_BY_CYCLE15 = (
    ("src/rules_core/corpus_loader.rs", "corpus_equipment_json::corpus_equipment_source_record"),
    ("src/rules_core/corpus_loader.rs", "corpus_race_json"),
)

# Booked explicitly: this cycle relabels nothing. The call left the live side; it did not
# move to another live file.
RELABELLED_BY_CYCLE15 = ()

# The settled-bundle kinds the producer writes and the live side reads.
SETTLED_KINDS = ("equipment", "race", "race_trait")


def classify(text):
    for gid, rx, why in GROUPS:
        if rx.search(text):
            return gid, why
    return FALLBACK


def _code_lines(path):
    """The file's shipping lines: comments and ``#[cfg(test)]`` regions removed exactly the
    way the gate removes them (rulings B14 and B15)."""
    with open(path, encoding="utf-8", errors="replace") as fh:
        lines = fh.read().splitlines()
    skip = set()
    for a, b in G.cfg_test_ranges(lines):
        skip.update(range(a, b + 1))
    return [
        (i + 1, ln)
        for i, ln in enumerate(lines)
        if i not in skip and not ln.lstrip().startswith("//")
    ]


def main():
    rx = G._COMPILED["pcgen_import"]
    rows = []
    for live_root, rel, abs_path in G._iter_live_source_files(ROOT):
        for lineno, line in _code_lines(abs_path):
            n = len(rx.findall(line))
            if not n:
                continue
            gid, why = classify(line)
            rows.append(dict(root=live_root, file=rel, line=lineno, hits=n,
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
            if gid == "corpus_json_boundary":
                print("  " + CORPUS_JSON_BOUNDARY_WHY_CYCLE15)
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

    # THE CLOSURE ASSERTION for this cycle: the live loader names no converter at all.
    for f in CLEARED_BY_CYCLE15:
        assert f not in hit_files, f"cycle 15 cleared {f} and it is hit again"
    print(f"closed_by_cycle15={len(CLOSED_BY_CYCLE15)} (by file and symbol) "
          f"relabelled_by_cycle15={len(RELABELLED_BY_CYCLE15)} "
          f"cleared_files_by_cycle15={len(CLEARED_BY_CYCLE15)}")

    # Every earlier cycle's cleared files must STAY clear.
    for label, cleared in (("cycle10", C14.C13.C12.C11.C10.CLEARED_BY_CYCLE10),
                           ("cycle11", C14.C13.C12.C11.CLEARED_BY_CYCLE11),
                           ("cycle12", C14.C13.C12.CLEARED_BY_CYCLE12),
                           ("cycle14", C14.CLEARED_BY_CYCLE14)):
        still = [f for f in cleared if f in hit_files]
        assert not still, f"{label} cleared these files and they are hit again: {still}"
    print("cleared_by_cycle10=3 (still clear) cleared_by_cycle11=3 (still clear) "
          "cleared_by_cycle12=2 (still clear) cleared_by_cycle14=1 (still clear)")

    # The two boundary CALL sites must be gone by text, not merely absent from the gate's
    # file list -- the same by-symbol standard cycles 13 and 14 held themselves to.
    loader_src = os.path.join(ROOT, "src/rules_core/corpus_loader.rs")
    loader_code = "\n".join(ln for _, ln in _code_lines(loader_src))
    for banned in ("pcgen_import", "corpus_equipment_json", "corpus_race_json",
                   "raw_tokens", "raw_bonus_chains"):
        assert banned not in loader_code, (
            f"corpus_loader still names {banned!r} in shipping code"
        )
    print("corpus_loader_shipping_pcgen_import_hits=0 (was 2)")

    # The live reader must be pure serde: it may name no converter module anywhere at all,
    # comments included, or the boundary has simply been renamed.
    reader_src = os.path.join(ROOT, "src/rules_core/settled_corpus.rs")
    with open(reader_src, encoding="utf-8") as fh:
        reader_text = fh.read()
    reader_code = "\n".join(ln for _, ln in _code_lines(reader_src))
    assert "pcgen_import" not in reader_code, "the live settled-bundle reader names the converter"
    for needed in ("pub fn read_settled_bundle", "pub fn bundle_key", "pub const SETTLED_DIR"):
        assert needed in reader_text, f"{needed} is missing from the live reader"
    print("settled_corpus=live_serde_only converter_names_in_reader=0")

    # And the producer must make the SAME three calls the live side used to make, so this is
    # a move of WHEN the reading happens, not a second implementation of it.
    producer_src = os.path.join(ROOT, "src/pcgen_import/corpus_settled_bundle.rs")
    with open(producer_src, encoding="utf-8") as fh:
        producer_text = fh.read()
    moved_calls = (
        "corpus_equipment_source_record",
        "corpus_race_source_record",
        "corpus_race_trait_source_record",
    )
    for needed in moved_calls:
        assert needed in producer_text, f"{needed} is missing from the converter-side producer"
    bin_src = os.path.join(ROOT, "src/bin/gen_settled_corpus.rs")
    assert os.path.isfile(bin_src), "the authoring-time producer binary is missing"
    print(f"corpus_settled_bundle=present moved_boundary_calls_on_converter_side="
          f"{len(moved_calls)} producer_bin=present")

    # The bundles themselves: every book that states a kind carries a bundle for it, and the
    # population is the real corpus, not a sample.
    corpus_root = os.path.join(ROOT, "data/corpus")
    books = sorted(
        d for d in os.listdir(corpus_root)
        if os.path.isdir(os.path.join(corpus_root, d)) and not d.startswith("_")
    )
    bundles = 0
    bundle_records = 0
    missing = []
    for book in books:
        for kind in SETTLED_KINDS:
            kind_dir = os.path.join(corpus_root, book, kind)
            if not os.path.isdir(kind_dir):
                continue
            path = os.path.join(corpus_root, book, "_settled", f"{kind}.json")
            if not os.path.isfile(path):
                missing.append(os.path.relpath(path, ROOT))
                continue
            with open(path, encoding="utf-8") as fh:
                payload = json.load(fh)
            assert payload["kind"] == kind, f"{path} declares kind {payload['kind']!r}"
            bundles += 1
            bundle_records += len(payload["records"])
    assert not missing, f"books state a kind with no settled bundle: {missing}"
    assert bundle_records >= 8000, f"the settled population is thousands of records, got {bundle_records}"
    print(f"books={len(books)} settled_bundles={bundles} settled_records={bundle_records} "
          f"missing_bundles=0")

    # A bundle inside a kind directory would be walked as a corpus record. None is.
    stray = [
        os.path.join(dirpath, name)
        for book in books
        for kind in SETTLED_KINDS
        for dirpath, _dirs, filenames in os.walk(os.path.join(corpus_root, book, kind))
        for name in filenames
        if name in {f"{k}.json" for k in SETTLED_KINDS} and os.path.basename(dirpath) == "_settled"
    ]
    assert not stray, f"a settled bundle sits inside a kind directory: {stray}"
    print("bundles_inside_a_kind_dir=0")

    out = os.path.join(HERE, "AT-35-E6-003-RULED_cycle15_runtime_import_census.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump({"total_hits": total, "total_files": len(files),
                   "by_root": dict(by_root), "by_group": sizes, "rows": rows,
                   "closed_by_cycle15": [list(x) for x in CLOSED_BY_CYCLE15],
                   "relabelled_by_cycle15": [list(x) for x in RELABELLED_BY_CYCLE15],
                   "cleared_files_by_cycle15": list(CLEARED_BY_CYCLE15),
                   "settled_bundles": bundles, "settled_records": bundle_records},
                  fh, indent=2)
    print(f"census written to {os.path.relpath(out, ROOT)}")


if __name__ == "__main__":
    main()
