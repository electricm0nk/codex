#!/usr/bin/env python3
"""AT-35-E6-003-RULED cycle 17 — the run-time ``pcgen_import`` census, re-derived.

Cycle 16's census is imported whole (which imports cycle 15's, … back to cycle 1's): the
``GROUPS`` table, ``classify``, ``_code_lines`` and the two exclusions the gate itself applies
(B14 comment lines, B15 ``#[cfg(test)]`` regions) stay single-sourced. **No regex is widened and
no regex is narrowed**, and ``scripts/pcgen_residue_gate.py`` /
``scripts/pcgen-residue-baseline.env`` are absent from this cycle's diff — no path exempted, no
rebaseline.

What cycle 17 moved
-------------------

The ``ingest_record_tokens`` group — one counted hit, ``src/rules_core/
derived_evaluator_fixture_check.rs``'s ``use crate::pcgen_import::ingest_record;`` — is
**cleared whole**, and with it the file. Behind that single counted line sat **three** shipping
converter calls (cycle 16's own correction, asserted there by file and symbol). Cycle 16 read
them as one relocation problem. They were **two different problems**, and the census below books
them separately because the fix for each is different:

* ``load_spell_durations`` / ``load_spell_ranges`` (``ingest_record::first_token_value`` at what
  were lines 668 and 1011) were **production reads**. ``apps/desktop/src-tauri/src/
  spell_catalog.rs``'s ``duration_for()`` / ``range_for()`` are backed by
  ``all_spell_caster_level_durations`` / ``all_spell_caster_level_ranges``, so a reader browsing
  the spell catalog made the live side walk ``data/corpus/`` and read raw ``DURATION:`` /
  ``RANGE:`` tokens. That is the thing ``decisions.md`` §11 forbids, not a gate's bookkeeping.
  **Settled at ingest**, the cycle-15/16 shape: ``src/pcgen_import/spell_formula_settle.rs``
  performs the identical walk once at authoring time and
  ``data/converted/record_vars.json`` gains ``spell_formulas``; the live side reads a settled
  ``CasterLevelLinearFormula`` / ``SpellRangeFormula`` and never sees a token.

* ``load_class_feature_bonus_vars`` (``ingest_record::token_values`` at what was line 1380) had
  **no production consumer at all** — it is the ``kind=class_feature`` half of the bar check.
  **Moved, whole and unchanged, with its report and its six mutation proofs**, to
  ``src/oracle_validation/class_feature_scaling_bar_check.rs``, exactly as ``AT-35-E6-001`` moved
  the ``kind=race_trait`` FORMULA half to ``oracle_validation::race_trait_formula_bar_check``;
  ``run_bar_check`` folds the report the same way, so the gate's reach is identical. This is the
  move cycle 16 named as the only admissible one for this file ("it leaves ``src/rules_core/``
  with its bar-check reports or not at all") — **not** a wrapper behind a name the gate does not
  match, which is the shape ruling B16 exists to punish.

The claim this cycle CORRECTS
-----------------------------

Cycle 16's receipt filed this hit under ``ingest_record_tokens`` as "a relocation cycle, not a
settling one". Two of its three calls were a settling problem, and they were on a **shipping
desktop path**. Re-derive the consumer:
``grep -n 'all_spell_caster_level' apps/desktop/src-tauri/src/spell_catalog.rs``.
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
    "at35_e6_003_ruled_cycle16_census",
    os.path.join(HERE, "AT-35-E6-003-RULED_cycle16_runtime_import_census.py"),
)
C16 = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(C16)
C15 = C16.C15

INGEST_RECORD_WHY_CYCLE17 = (
    "CLEARED BY CYCLE 17. One counted hit -- `derived_evaluator_fixture_check.rs`'s `use "
    "crate::pcgen_import::ingest_record;` -- carrying THREE shipping calls behind it, split "
    "two ways. (1) The `DURATION:`/`RANGE:` walks were PRODUCTION: "
    "`apps/desktop/src-tauri/src/spell_catalog.rs`'s `duration_for()`/`range_for()` are served "
    "by `all_spell_caster_level_durations`/`_ranges`, so browsing the spell catalog made the "
    "live side read raw ingest tokens out of `data/corpus/` on every process start. Settled at "
    "ingest by `pcgen_import::spell_formula_settle` into `record_vars.json`'s `spell_formulas`, "
    "with a whole-corpus test that re-reads every record in every book and compares the settled "
    "tables key for key and value for value, refusals included. (2) The `BONUS:VAR|` walk had no "
    "production consumer: it is the `kind=class_feature` bar-check half, and it MOVED whole -- "
    "functions, control flow, failure strings and six mutation proofs -- to "
    "`oracle_validation::class_feature_scaling_bar_check`, the same address and the same reason "
    "as `AT-35-E6-001`'s `race_trait_formula_bar_check`. Re-derive: "
    "`grep -rn 'pcgen_import' src/rules_core/derived_evaluator_fixture_check.rs` (expect no "
    "output)."
)

_REWRITTEN = {
    "ingest_record_tokens": INGEST_RECORD_WHY_CYCLE17,
}

# Every regex is cycle 16's, untouched. No group is dropped; the ingest-record group simply
# matches nothing under a live root any more.
GROUPS = [(gid, rx, _REWRITTEN.get(gid, why)) for gid, rx, why in C16.GROUPS]
FALLBACK = C16.FALLBACK

# The one file cycle 17 clears whole.
CLEARED_BY_CYCLE17 = ("src/rules_core/derived_evaluator_fixture_check.rs",)

# The hit cycle 17 genuinely closed, by file and symbol, plus the three shipping calls behind it.
CLOSED_BY_CYCLE17 = (
    ("src/rules_core/derived_evaluator_fixture_check.rs", "ingest_record"),
)
SETTLED_BY_CYCLE17 = (
    ("src/rules_core/derived_evaluator_fixture_check.rs", "ingest_record::first_token_value"),
)
MOVED_BY_CYCLE17 = (
    ("src/rules_core/derived_evaluator_fixture_check.rs", "ingest_record::token_values",
     "src/oracle_validation/class_feature_scaling_bar_check.rs"),
)

# Booked explicitly: this cycle relabels nothing on the live side. The bar-check half moved to a
# NON-live root, which is a closure under `decisions.md` §11's own boundary (the converter, the
# parser and the oracle harness are KEPT, beside each other), not a live-to-live relabel.
RELABELLED_BY_CYCLE17 = ()

LIVE_SEAM_MODULE = "src/rules_core/derived_evaluator_fixture_check.rs"
CONVERTER_SETTLING_MODULE = "src/pcgen_import/spell_formula_settle.rs"
ORACLE_BAR_CHECK_MODULE = "src/oracle_validation/class_feature_scaling_bar_check.rs"
CONVERTED_ARTIFACT = "data/converted/record_vars.json"
DESKTOP_CONSUMER = "apps/desktop/src-tauri/src/spell_catalog.rs"


def classify(text):
    for gid, rx, why in GROUPS:
        if rx.search(text):
            return gid, why
    return FALLBACK


def _read(rel):
    with open(os.path.join(ROOT, rel), encoding="utf-8") as fh:
        return fh.read()


def _shipping(rel):
    return "\n".join(t for _, t in C15._code_lines(os.path.join(ROOT, rel)))


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
    for f in CLEARED_BY_CYCLE17:
        assert f not in hit_files, f"cycle 17 cleared {f} and it is hit again"
    for f in C16.CLEARED_BY_CYCLE16:
        assert f not in hit_files, f"cycle 16 cleared {f} and it is hit again"
    for f in C15.CLEARED_BY_CYCLE15:
        assert f not in hit_files, f"cycle 15 cleared {f} and it is hit again"
    print(f"closed_by_cycle17={len(CLOSED_BY_CYCLE17)} (by file and symbol) "
          f"settled_calls={len(SETTLED_BY_CYCLE17)} moved_calls={len(MOVED_BY_CYCLE17)} "
          f"relabelled_by_cycle17={len(RELABELLED_BY_CYCLE17)} "
          f"cleared_files_by_cycle17={len(CLEARED_BY_CYCLE17)}")
    print("cleared_by_cycle10=3 (still clear) cleared_by_cycle11=3 (still clear) "
          "cleared_by_cycle12=2 (still clear) cleared_by_cycle14=1 (still clear) "
          "cleared_by_cycle15=1 (still clear) cleared_by_cycle16=1 (still clear)")

    # THE CLOSURE ASSERTION for this cycle: the seam module names the converter nowhere in
    # shipping code -- asserted by SYMBOL, not inferred from the gate's file list, because the
    # gate counted one `use` line for three calls and that is exactly how cycle 15's figure went
    # wrong (ruling B16).
    seam_shipping = _shipping(LIVE_SEAM_MODULE)
    for symbol in ("pcgen_import", "ingest_record", "first_token_value", "token_values",
                   "raw_tokens"):
        assert symbol not in seam_shipping, (
            f"{LIVE_SEAM_MODULE} still names {symbol} in shipping code"
        )
    print("derived_evaluator_fixture_check_shipping_pcgen_import_hits=0 (was 1 counted, "
          "3 calls)")

    # The settling must be on the converter side, must be the same walk, and must apply the same
    # two live parsers -- a settling that invented its own reading would be a second engine.
    settling = _read(CONVERTER_SETTLING_MODULE)
    for needed in ("ingest_record::first_token_value", "parse_caster_level_linear_duration",
                   "spell_range_formula", "SPELL_CORPUS_BOOK_DIRS"):
        assert needed in settling, f"{needed} is missing from the converter-side settling"
    print("spell_formula_settle=present reused_live_parsers=2")

    # The moved bar check must be at its oracle address, whole, and folded by `run_bar_check` --
    # a half that moved and stopped being reachable would be a silently narrowed gate.
    oracle = _read(ORACLE_BAR_CHECK_MODULE)
    for needed in ("fn load_class_feature_bonus_vars", "fn find_level_var_alias",
                   "pub fn run_class_feature_bar_check", "ingest_record::token_values"):
        assert needed in oracle, f"{needed} did not arrive at {ORACLE_BAR_CHECK_MODULE}"
    mutation_proofs = oracle.count("run_class_feature_bar_check(&scratch.root)") + \
        oracle.count("run_class_feature_bar_check(&root)")
    assert mutation_proofs >= 6, (
        f"the six mutation proofs must have moved with the check, found {mutation_proofs}"
    )
    assert "class_feature_scaling_bar_check::run_class_feature_bar_check" in \
        _read(LIVE_SEAM_MODULE), "run_bar_check no longer folds the moved report"
    print(f"class_feature_bar_check=oracle_side folded_by_run_bar_check=YES "
          f"mutation_proofs_moved={mutation_proofs}")

    # The artifact itself: a real settled population, and the two tables partitioned from their
    # refusals (a refusal that became a formula would print a fabricated duration on a sheet).
    with open(os.path.join(ROOT, CONVERTED_ARTIFACT), encoding="utf-8") as fh:
        pkg = json.load(fh)
    spell = pkg.get("spell_formulas", {})
    durations = spell.get("durations", {})
    ranges = spell.get("ranges", {})
    duration_refused = set(spell.get("duration_refused", []))
    range_refused = set(spell.get("range_refused", []))
    assert len(durations) >= 1_000, f"the settled duration population collapsed: {len(durations)}"
    assert len(ranges) >= 800, f"the settled range population collapsed: {len(ranges)}"
    assert not (set(durations) & duration_refused), "a duration is both settled and refused"
    assert not (set(ranges) & range_refused), "a range is both settled and refused"
    books = {k.split("|", 1)[0] for k in list(durations) + list(ranges)}
    assert len(books) >= 8, f"the settled tables must span the ingested books, got {books}"
    print(f"settled_spell_durations={len(durations)} refused={len(duration_refused)} "
          f"settled_spell_ranges={len(ranges)} refused={len(range_refused)} "
          f"books={len(books)}")

    # And the production consumer that forced this to be a settling rather than a move.
    desktop_src = _read(DESKTOP_CONSUMER)
    for needed in ("all_spell_caster_level_durations", "all_spell_caster_level_ranges"):
        assert needed in desktop_src, (
            f"{DESKTOP_CONSUMER} no longer consumes {needed} -- restate this census's own "
            f"account of why the settling was required rather than leaving it stale"
        )
    assert "pcgen_import" not in _shipping(DESKTOP_CONSUMER), (
        f"{DESKTOP_CONSUMER} names the converter in shipping code"
    )
    print("desktop_spell_catalog_consumes_settled_tables=YES "
          "desktop_shipping_pcgen_import_hits=0")

    out = os.path.join(HERE, "AT-35-E6-003-RULED_cycle17_runtime_import_census.json")
    with open(out, "w", encoding="utf-8") as fh:
        json.dump({"total_hits": total, "total_files": len(files),
                   "by_root": dict(by_root), "by_group": sizes, "rows": rows,
                   "closed_by_cycle17": [list(x) for x in CLOSED_BY_CYCLE17],
                   "settled_by_cycle17": [list(x) for x in SETTLED_BY_CYCLE17],
                   "moved_by_cycle17": [list(x) for x in MOVED_BY_CYCLE17],
                   "relabelled_by_cycle17": [list(x) for x in RELABELLED_BY_CYCLE17],
                   "cleared_files_by_cycle17": list(CLEARED_BY_CYCLE17),
                   "settled_spell_durations": len(durations),
                   "settled_spell_duration_refused": len(duration_refused),
                   "settled_spell_ranges": len(ranges),
                   "settled_spell_range_refused": len(range_refused)},
                  fh, indent=2)
    print(f"census written to {os.path.relpath(out, ROOT)}")


if __name__ == "__main__":
    main()
