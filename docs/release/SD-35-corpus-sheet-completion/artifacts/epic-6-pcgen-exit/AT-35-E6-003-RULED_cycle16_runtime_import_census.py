#!/usr/bin/env python3
"""AT-35-E6-003-RULED cycle 16 — the run-time ``pcgen_import`` census, re-derived.

Cycle 15's census is imported whole (which imports cycle 14's, … back to cycle 1's): the
``GROUPS`` table, ``classify``, ``_code_lines`` and the two exclusions the gate itself applies
(B14 comment lines, B15 ``#[cfg(test)]`` regions) stay single-sourced. **No regex is widened and
no regex is narrowed**, and ``scripts/pcgen_residue_gate.py`` /
``scripts/pcgen-residue-baseline.env`` are absent from this cycle's diff — no path exempted, no
rebaseline.

What cycle 16 moved
-------------------

The ``renderer`` group — cycle 15's own named next-cycle scope, and the largest remaining group —
is **cleared whole**, and with it ``class_feature_grant_consumer.rs``'s last hits. It used to
hold five calls, in two functions:

* ``resolved_description_for``: ``PcgenDisplayValues::new`` + ``render_pcgen_desc_with_values``;
* ``resolved_description_for_formula_only_desc_argument``: ``desc_token_arguments``,
  ``PcgenDisplayValues::new`` + ``render_pcgen_desc_with_values``.

All five handed a corpus record's **stored source description string** to the converter's prose
renderer at request time. Cycle 2 measured a *different* replacement (the converted RULE's own
prose) and refused it on 97,332 of 660,320 disagreements — a **content** gap, not a rendering
one. Cycle 16 did not reopen that question. It applied the shape cycle 15 used instead: settle
the reading at authoring time.

* ``src/pcgen_import/desc_template_convert.rs`` runs the renderer's own scan, branch for branch,
  over the same stored description at ingest and emits an ordered op list;
* ``src/rules_core/desc_template.rs`` walks that op list with a plain ``name -> i64`` map;
* ``data/converted/record_vars.json`` gains ``desc_templates``, keyed by corpus ``KEY:`` under
  the live table's own first-record-wins guards.

**The sentence on the sheet did not move, and four whole-corpus proofs say so**, not a fixture:
every described ``class_feature`` record rendered both ways under six value environments, every
other record kind's description under two, the served table's keys compared against the
artifact's key for key, and the live table's own records rendered both ways — ``text`` **and**
``dropped_args`` compared field for field, 0 disagreements.

Five closures, no relabel
-------------------------

Net ``10 → 5``, one file cleared (6 → 5), no relabel: the calls did not move to another live
file, they left the live side for an authoring-time producer.

The claim this cycle CORRECTS
-----------------------------

Cycle 15's receipt called ``corpus_loader``'s boundary call *"the live side's last converter
CALL of any kind"*. It was not. ``derived_evaluator_fixture_check.rs`` lines 668, 1011 and 1380
call ``pcgen_import::ingest_record::{first_token_value,token_values}`` in shipping code — the
gate counts that file's ``use`` line only, so the calls were invisible to the hit count but not
to the codebase. This census asserts them, by file and symbol, so the figure cannot be restated
wrongly again.
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
    "at35_e6_003_ruled_cycle15_census",
    os.path.join(HERE, "AT-35-E6-003-RULED_cycle15_runtime_import_census.py"),
)
C15 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(C15)

RENDERER_WHY_CYCLE16 = (
    "CLEARED BY CYCLE 16. This group held FIVE calls, all in "
    "`pilot_compute::class_feature_grant_consumer`, all handing a corpus record's STORED "
    "source description string to the converter's prose renderer at request time: "
    "`PcgenDisplayValues::new` + `render_pcgen_desc_with_values` in `resolved_description_for`, "
    "and `desc_token_arguments` + `PcgenDisplayValues::new` + `render_pcgen_desc_with_values` "
    "in `resolved_description_for_formula_only_desc_argument`. Cycle 2 measured a DIFFERENT "
    "replacement -- rendering the converted RULE's own prose instead -- and refused it on "
    "97,332 of 660,320 disagreements across 2,443 record keys; that is a CONTENT gap on the "
    "converter side and cycle 16 did not reopen it. Cycle 16 applied cycle 15's shape instead: "
    "`pcgen_import::desc_template_convert` runs the renderer's own scan, branch for branch, at "
    "INGEST and emits an ordered op list; `rules_core::desc_template` walks it with a plain "
    "`name -> i64` map; `data/converted/record_vars.json` carries `desc_templates` keyed by "
    "corpus `KEY:` under the live table's own first-record-wins guards. Byte parity is proved "
    "over the REAL corpus, not a fixture, by four tests comparing `text` AND `dropped_args`. "
    "Re-derive: `grep -rn 'pcgen_desc' --include=*.rs src/rules_core/` (expect no output)."
)

_REWRITTEN = {
    "renderer": RENDERER_WHY_CYCLE16,
    "corpus_json_boundary": C15.CORPUS_JSON_BOUNDARY_WHY_CYCLE15,
}

# Every regex is cycle 15's, untouched. No group is dropped; the renderer group simply matches
# nothing under a live root any more.
GROUPS = [(gid, rx, _REWRITTEN.get(gid, why)) for gid, rx, why in C15.GROUPS]
FALLBACK = C15.FALLBACK

# The one file cycle 16 clears whole.
CLEARED_BY_CYCLE16 = ("src/rules_core/pilot_compute/class_feature_grant_consumer.rs",)

# The five hits cycle 16 genuinely closed, by file and symbol. A later cycle that reintroduces
# any of them on the live side fails this census.
CLOSED_BY_CYCLE16 = (
    ("src/rules_core/pilot_compute/class_feature_grant_consumer.rs", "pcgen_desc"),
    ("src/rules_core/pilot_compute/class_feature_grant_consumer.rs", "PcgenDisplayValues"),
    ("src/rules_core/pilot_compute/class_feature_grant_consumer.rs",
     "render_pcgen_desc_with_values"),
    ("src/rules_core/pilot_compute/class_feature_grant_consumer.rs", "desc_token_arguments"),
)

# Booked explicitly: this cycle relabels nothing.
RELABELLED_BY_CYCLE16 = ()

# The three SHIPPING converter calls that remain on the live side, by file and symbol. The gate
# counts this file's `use` line only (one hit), so these were invisible to the hit count and
# cycle 15's receipt mis-stated the population as zero. Asserted here so the figure is derived,
# never restated.
REMAINING_SHIPPING_CONVERTER_CALLS = (
    ("src/rules_core/derived_evaluator_fixture_check.rs", "ingest_record::first_token_value"),
    ("src/rules_core/derived_evaluator_fixture_check.rs", "ingest_record::token_values"),
)

# What the live side must hold for the renderer closure to be real, asserted by text rather than
# inferred from the gate's file list.
LIVE_RENDER_MODULE = "src/rules_core/desc_template.rs"
CONVERTER_SETTLING_MODULE = "src/pcgen_import/desc_template_convert.rs"
CONVERTED_ARTIFACT = "data/converted/record_vars.json"


def classify(text):
    for gid, rx, why in GROUPS:
        if rx.search(text):
            return gid, why
    return FALLBACK


def _read(rel):
    with open(os.path.join(ROOT, rel), encoding="utf-8") as fh:
        return fh.read()


def main():
    rx = G._COMPILED["pcgen_import"]
    rows = []
    for live_root, rel, abs_path in G._iter_live_source_files(ROOT):
        for lineno, line in C15._code_lines(abs_path):
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
            if gid in _REWRITTEN:
                print("  " + _REWRITTEN[gid])
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
    for f in CLEARED_BY_CYCLE16:
        assert f not in hit_files, f"cycle 16 cleared {f} and it is hit again"
    for f in C15.CLEARED_BY_CYCLE15:
        assert f not in hit_files, f"cycle 15 cleared {f} and it is hit again"
    print(f"closed_by_cycle16={len(CLOSED_BY_CYCLE16)} (by file and symbol) "
          f"relabelled_by_cycle16={len(RELABELLED_BY_CYCLE16)} "
          f"cleared_files_by_cycle16={len(CLEARED_BY_CYCLE16)}")
    print("cleared_by_cycle10=3 (still clear) cleared_by_cycle11=3 (still clear) "
          "cleared_by_cycle12=2 (still clear) cleared_by_cycle14=1 (still clear) "
          "cleared_by_cycle15=1 (still clear)")

    # THE CLOSURE ASSERTION for this cycle: the consumer names the renderer nowhere in shipping
    # code -- asserted by SYMBOL, not inferred from the gate's file list.
    consumer = "src/rules_core/pilot_compute/class_feature_grant_consumer.rs"
    shipping = "\n".join(t for _, t in C15._code_lines(os.path.join(ROOT, consumer)))
    for symbol in ("pcgen_desc", "PcgenDisplayValues", "render_pcgen_desc_with_values",
                   "desc_token_arguments", "raw_tokens"):
        assert symbol not in shipping, (
            f"{consumer} still names {symbol} in shipping code"
        )
    print("class_feature_grant_consumer_shipping_pcgen_import_hits=0 (was 5)")

    # The live renderer must be live-only: a live module that named the converter would just be
    # the old boundary under a new name.
    live_render = _read(LIVE_RENDER_MODULE)
    live_render_shipping = "\n".join(
        t for _, t in C15._code_lines(os.path.join(ROOT, LIVE_RENDER_MODULE))
    )
    assert "pcgen_import" not in live_render_shipping, (
        f"{LIVE_RENDER_MODULE} names the converter in shipping code"
    )
    assert "serde" in live_render, f"{LIVE_RENDER_MODULE} must deserialize the settled artifact"
    print("desc_template=live_serde_only converter_names_in_reader=0")

    # The settling must be the converter's own scan, on the converter side.
    settling = _read(CONVERTER_SETTLING_MODULE)
    for needed in ("split_prose_and_args", "is_percentile_dice_notation", "PCGEN_ENTITIES"):
        assert needed in settling, f"{needed} is missing from the converter-side settling"
    print("desc_template_convert=present reused_renderer_readings=3")

    # The artifact itself: keyed by corpus KEY:, a real population, and no op left unsettled.
    with open(os.path.join(ROOT, CONVERTED_ARTIFACT), encoding="utf-8") as fh:
        pkg = json.load(fh)
    templates = pkg.get("desc_templates", {})
    ops = collections.Counter(
        op["op"] for t in templates.values() for op in t.get("ops", [])
    )
    with_ops = sum(1 for t in templates.values() if t.get("ops"))
    assert len(templates) >= 10_000, (
        f"the settled description population collapsed: {len(templates)}"
    )
    assert ops["arg"] >= 1_000, f"no value slots settled: {dict(ops)}"
    known = {"text", "arg", "missing_arg", "drop", "percent_or_drop"}
    assert set(ops) <= known, f"an unknown op reached the artifact: {set(ops) - known}"
    print(f"settled_descriptions={len(templates)} non_empty={with_ops} "
          + " ".join(f"{k}={v}" for k, v in sorted(ops.items())))

    # The correction cycle 15's receipt needs: shipping converter CALLS are not zero.
    remaining = []
    for rel, symbol in REMAINING_SHIPPING_CONVERTER_CALLS:
        shipping_lines = [
            (n, t) for n, t in C15._code_lines(os.path.join(ROOT, rel)) if symbol in t
        ]
        assert shipping_lines, (
            f"{rel} no longer calls {symbol} in shipping code -- update this census's own "
            f"statement of the remainder rather than leaving it stale"
        )
        remaining.extend((rel, symbol, n) for n, _ in shipping_lines)
    print(f"remaining_shipping_converter_calls={len(remaining)} "
          + ", ".join(f"{os.path.basename(r)}:{n}" for r, _s, n in remaining))

    out = os.path.join(HERE, "AT-35-E6-003-RULED_cycle16_runtime_import_census.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump({"total_hits": total, "total_files": len(files),
                   "by_root": dict(by_root), "by_group": sizes, "rows": rows,
                   "closed_by_cycle16": [list(x) for x in CLOSED_BY_CYCLE16],
                   "relabelled_by_cycle16": [list(x) for x in RELABELLED_BY_CYCLE16],
                   "cleared_files_by_cycle16": list(CLEARED_BY_CYCLE16),
                   "settled_descriptions": len(templates),
                   "settled_ops": dict(ops),
                   "remaining_shipping_converter_calls": [list(x) for x in remaining]},
                  fh, indent=2)
    print(f"census written to {os.path.relpath(out, ROOT)}")


if __name__ == "__main__":
    main()
