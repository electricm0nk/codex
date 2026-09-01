"""
Self-test for the dashboard producer's doneness-verdict table
(`scripts/observer/pf1e_dashboard_producer.py`) -- launch-readiness
remediation Step 4D, blocker B6.

WHY THIS EXISTS
----------------
`doneness_verdict()` / `_doneness_verdict_uncapped()` raise `ValueError` on
any `(wiring_class, status)` pair they have no rule for -- by design, so a
new status word landing in the generator without a matching rule is a loud
crash, not a silent default. `compute_wiring_class_summary()` catches that
exception per-unit and records it in `doneness_unmapped` rather than letting
one novel unit kill the whole 5-minute cron tick -- but a table that quietly
grows an unmapped cell is still a real defect (blocker B6: the
`(ambiguous, literal-verified|fixture-verified)` cells were unmapped until
this remediation). This test grids over the FULL `WIRING_CLASS_VALUES x`
status-vocabulary cross product on a fabricated document -- not the real
corpus, which may not exercise every cell -- and asserts nothing lands in
`doneness_unmapped`.

Run: python3 -m unittest scripts/tests/test_pf1e_dashboard_producer.py
Wired as the `producer-selftest` stage in both `verify.sh` stage sets.

Prove-it-can-fail discipline (same as `test_fetch_pcgen_oracle.sh`): comment
out the `ambiguous` branch's `"literal-verified", "fixture-verified"` tuple
members in `_doneness_verdict_uncapped()` and re-run -- `test_full_grid_...`
and `test_ambiguous_literal_verified_is_held` both go red.
"""
from __future__ import annotations

import importlib.util
import json
import os
import pathlib
import tempfile
import unittest.mock
import unittest

# producer.py is always this test's sibling-of-a-sibling
# (scripts/observer/pf1e_dashboard_producer.py); resolved relative to
# __file__, the same convention the producer itself uses to find
# observer.py, rather than an absolute path that would break on a
# differently-rooted checkout.
_PRODUCER_PATH = (
    pathlib.Path(__file__).resolve().parent.parent / "observer" / "pf1e_dashboard_producer.py"
)
_spec = importlib.util.spec_from_file_location("pf1e_dashboard_producer", _PRODUCER_PATH)
producer = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(producer)

# Kept in sync BY HAND with `STATUS_VOCABULARY` in `src/bin/v06_work_inventory.rs`
# -- the Rust generator's status words are the one authoritative list; this
# test grids over its own copy rather than reaching across the Rust/Python
# boundary at test time, so a NEW status word landing in the generator
# without a matching `doneness_verdict()` rule is caught here as an
# unmapped cell in the grid below, not silently skipped because the test
# never knew the word existed.
STATUS_WORDS = (
    "grounded",
    "literal-verified",
    "fixture-verified",
    "ingested-magnitude",
    "text-complete",
    "deferred-with-reason",
    "engine-does-not-hold",
    "not-started",
    "unmeasurable",
)


def _fabricated_doc_path(tmpdir: str) -> str:
    """One unit per `(wiring_class, status)` cell -- 5 x 9 = 45 units.

    `kind="spell"`, deliberately: `spell` left `NO_GROUNDING_PROBE` (see the
    producer's own SD30-E0-F2 comment on `STATUS_LABEL`/`NO_GROUNDING_PROBE`,
    re-derived live 2026-08-14), so `doneness_verdict()`'s kind-cap never
    fires and cannot mask a raising cell behind a capped `held` -- this test
    wants to see the RAW `_doneness_verdict_uncapped()` table's coverage.
    """
    units = []
    for wc in producer.WIRING_CLASS_VALUES:
        for st in STATUS_WORDS:
            units.append({
                "id": f"fab:{wc}:{st}",
                "book": "core_rulebook",
                "kind": "spell",
                "wiring_class": wc,
                "status": st,
            })
    doc = {"generated_at": "2026-08-15T00:00:00Z", "units": units}
    doc_path = os.path.join(tmpdir, "fab-work-inventory.json")
    with open(doc_path, "w", encoding="utf-8") as f:
        json.dump(doc, f)
    return doc_path


class DonenessVerdictGridTest(unittest.TestCase):
    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self.doc_path = _fabricated_doc_path(self._tmp.name)
        # A dedicated scratch cache path -- never the real
        # WIRING_CLASS_CACHE -- so this test can neither read a stale real
        # cache nor pollute it with a fabricated one.
        self.cache_path = os.path.join(self._tmp.name, "fab-wiring-cache.json")

    def test_full_grid_yields_no_unmapped_cells(self):
        """The whole 5 wiring_class x 9 status grid (45 cells) must map to a
        real doneness verdict -- `doneness_unmapped` must come back empty."""
        summary = producer.compute_wiring_class_summary(
            doc_path=self.doc_path, cache_path=self.cache_path
        )
        self.assertTrue(summary.get("available"), summary.get("note"))
        self.assertEqual(
            summary.get("doneness_unmapped", "MISSING (field absent)"),
            {},
            "the full WIRING_CLASS_VALUES x status_vocabulary grid must map every "
            f"cell to a verdict; unmapped: {summary.get('doneness_unmapped')}",
        )
        # Sanity: 45 units in, 45 units accounted for across done+unmapped.
        total_doneness = sum(summary.get("doneness", {}).values())
        total_unmapped = sum(summary.get("doneness_unmapped", {}).values())
        self.assertEqual(total_doneness + total_unmapped, len(producer.WIRING_CLASS_VALUES) * len(STATUS_WORDS))

    def test_ambiguous_literal_verified_is_held(self):
        self.assertEqual(
            producer.doneness_verdict("ambiguous", "literal-verified", "spell"),
            producer.DONENESS_HELD,
        )

    def test_ambiguous_fixture_verified_is_held(self):
        self.assertEqual(
            producer.doneness_verdict("ambiguous", "fixture-verified", "spell"),
            producer.DONENESS_HELD,
        )

    def test_ambiguous_unmeasurable_is_unmeasurable(self):
        """`AT-33-E6-001` (2026-08-25): the real, live cell -- 11 of 49,438
        `docs/work-inventory.json` units carry exactly this pair after
        `AT-33-E4-002` renamed the 318 genuinely-irreducible units'
        `status` from `unknown` to `unmeasurable` without updating this
        table's checked-first branch, which crashed `cargo test --locked
        --lib` (a real shell-out through `shape_ledger.py` ->
        `coverage_ledger.py` -> here) on every one of the 11. Checked first,
        ahead of the `ambiguous` branch's own status list, so the
        wiring-class classifier's separate failure never masks the honest
        "this instrument cannot classify this unit" verdict."""
        self.assertEqual(
            producer.doneness_verdict("ambiguous", "unmeasurable", "spell"),
            producer.DONENESS_UNMEASURABLE,
        )

    def test_unknown_is_still_accepted_as_unmeasurables_legacy_spelling(self):
        """`unknown` is `unmeasurable`'s pre-`AT-33-E4-002` name. No live
        unit carries it any more (`STATUS_VOCABULARY` in
        `src/bin/v06_work_inventory.rs` dropped it), but an older, already-
        generated `work-inventory.json` snapshot legitimately can -- this
        function's whole design is "never silently reinterpret a status
        word", so both spellings must keep resolving identically rather
        than the old one starting to raise the day the rename landed."""
        for wiring_class in producer.WIRING_CLASS_VALUES:
            self.assertEqual(
                producer.doneness_verdict(wiring_class, "unknown", "spell"),
                producer.doneness_verdict(wiring_class, "unmeasurable", "spell"),
                f"wiring_class={wiring_class!r}: 'unknown' and 'unmeasurable' must agree",
            )

    def test_not_ingested_is_still_accepted_as_engine_does_not_holds_legacy_spelling(self):
        """`not-ingested` is `engine-does-not-hold`'s pre-`AT-34-E1-005` name
        (`decisions.md §2b`). No live unit in `docs/work-inventory.json`
        carries it any more (`STATUS_VOCABULARY` in
        `src/bin/v06_work_inventory.rs` dropped it, and every occurrence
        there and in `docs/work-inventory.json` was renamed), but the
        committed `site/dashboard/units/*.json` shard cache is a separate,
        independently-staled artifact that legitimately still carries the
        old word until its own regeneration cycle -- this function's whole
        design is "never silently reinterpret a status word" (the same
        precedent `test_unknown_is_still_accepted_as_unmeasurables_legacy_
        spelling` establishes for `unknown`/`unmeasurable`), so both
        spellings must keep resolving identically rather than the old one
        starting to raise the day the rename landed. Found live: `python3
        scripts/site/build_public_status.py --check` raised `ValueError:
        doneness: unmapped 'static' + 'not-ingested'` reading the committed
        `site/dashboard/units/PF1e-units-ability.json` shard."""
        for wiring_class in producer.WIRING_CLASS_VALUES:
            self.assertEqual(
                producer.doneness_verdict(wiring_class, "not-ingested", "spell"),
                producer.doneness_verdict(wiring_class, "engine-does-not-hold", "spell"),
                f"wiring_class={wiring_class!r}: 'not-ingested' and 'engine-does-not-hold' must agree",
            )

    def test_static_literal_verified_is_done(self):
        """Control: the ORIGINAL done rung (SD-32 decisions.md §2) still
        works -- `static` unambiguously reaches `done` on the same status
        word `ambiguous` only reaches `held` on, above."""
        self.assertEqual(
            producer.doneness_verdict("static", "literal-verified", "spell"),
            producer.DONENESS_DONE,
        )

    def test_unmapped_cell_raises(self):
        """Negative case: a status word with genuinely no rule anywhere
        still raises -- this table must not have quietly become total."""
        with self.assertRaises(ValueError):
            producer.doneness_verdict("ambiguous", "bogus-status-word", "spell")


class ClassifierReclassifiedUnitsTest(unittest.TestCase):
    """SD-32 Epic 2 T8 (D13, `docs/release/SD-31-corpus-closure-grind/todo/defects.md`
    D13): `class_feature` units sitting in the `wiring_class='display'` +
    `status='grounded'` cross-tab cell are the classifier's own documented
    blind spot -- the determinator's single-row `no_magnitude_token`
    heuristic never considers that `status=='grounded'` is itself real,
    independent evidence a live consumer already computed something from the
    unit. `compute_wiring_class_summary()` now reclassifies exactly the
    units where that independent evidence exists (`evidence ==
    'explanation_id_observed_in_a_real_computation'`) from `display` to
    `computed`, BEFORE every corpus-wide rollup reads `wiring_class` --
    so `doneness_verdict('computed', 'grounded', kind)` (unmodified) fires
    DONE for them. A generic PREDICATE, not a hardcoded id list, so this is
    provably a by-CLASS fix (Decision 11 condition 1), and `doneness_verdict()`
    itself is untouched.

    Mutation-proof: six units, one of each shape that must NOT reclassify
    (wrong kind, wrong wiring_class, wrong status, missing the corroborating
    evidence, excluded book) sit beside two that must."""

    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self.cache_path = os.path.join(self._tmp.name, "fab-wiring-cache.json")

    def _doc(self, units):
        doc = {"generated_at": "2026-08-22T00:00:00Z", "units": units}
        doc_path = os.path.join(self._tmp.name, "fab-work-inventory.json")
        with open(doc_path, "w", encoding="utf-8") as f:
            json.dump(doc, f)
        return doc_path

    _EVIDENCE = "explanation_id_observed_in_a_real_computation"

    def test_only_evidence_corroborated_display_grounded_class_features_reclassify(self):
        units = [
            # Two genuine blind-spot units -- must reclassify to `computed`
            # and reach DONE.
            {"id": "core_rulebook:class_feature:monk_evasion", "book": "core_rulebook",
             "kind": "class_feature", "wiring_class": "display", "status": "grounded",
             "evidence": self._EVIDENCE},
            {"id": "core_rulebook:class_feature:rogue_evasion", "book": "core_rulebook",
             "kind": "class_feature", "wiring_class": "display", "status": "grounded",
             "evidence": self._EVIDENCE},
            # Wrong kind -- must NOT reclassify.
            {"id": "core_rulebook:spell:fab_spell", "book": "core_rulebook",
             "kind": "spell", "wiring_class": "display", "status": "grounded",
             "evidence": self._EVIDENCE},
            # Wrong wiring_class -- must NOT reclassify (already computed).
            {"id": "core_rulebook:class_feature:fab_computed", "book": "core_rulebook",
             "kind": "class_feature", "wiring_class": "computed", "status": "grounded",
             "evidence": self._EVIDENCE},
            # Wrong status -- must NOT reclassify.
            {"id": "core_rulebook:class_feature:fab_textcomplete", "book": "core_rulebook",
             "kind": "class_feature", "wiring_class": "display", "status": "text-complete",
             "evidence": self._EVIDENCE},
            # Missing the corroborating evidence -- must NOT reclassify, even
            # though kind/wiring_class/status all match. This is the guard
            # that keeps the predicate from silently widening past D13's own
            # scope.
            {"id": "core_rulebook:class_feature:fab_no_evidence", "book": "core_rulebook",
             "kind": "class_feature", "wiring_class": "display", "status": "grounded",
             "evidence": "some_other_evidence_string"},
            # A book living in EXCLUDED_BOOKS -- must NOT reclassify even
            # though it otherwise matches. `EXCLUDED_BOOKS` is empty by
            # default since 2026-08-24 (`decisions.md §27b`: no carve-outs
            # survive), so this case patches it in for the duration of this
            # test to prove the inline `book not in EXCLUDED_BOOKS` guard
            # still works for a FUTURE, genuinely admissible exclusion --
            # it does not assert any particular book stays excluded.
            {"id": "test_excluded_book:class_feature:fab_excluded", "book": "test_excluded_book",
             "kind": "class_feature", "wiring_class": "display", "status": "grounded",
             "evidence": self._EVIDENCE},
        ]
        with unittest.mock.patch.object(producer, "EXCLUDED_BOOKS", frozenset({"test_excluded_book"})):
            summary = producer.compute_wiring_class_summary(
                doc_path=self._doc(units), cache_path=self.cache_path
            )
        record = summary.get("classifier_reclassified_units")
        self.assertIsNotNone(record, "classifier_reclassified_units missing from the cache")
        self.assertEqual(record["reclassified_to"], "computed")
        self.assertEqual(record["count"], 2)
        self.assertEqual(
            sorted(record["units"]),
            ["core_rulebook:class_feature:monk_evasion",
             "core_rulebook:class_feature:rogue_evasion"],
        )
        # The reclassification must actually land in the rollups, not just
        # the audit-trail field: `display` drops by 2, `computed` gains 2,
        # and the two units now count toward `done` via the UNMODIFIED
        # computed+grounded rule.
        self.assertEqual(summary["corpus_wide"].get("display", 0), 4)  # spell + text-complete + no-evidence + excluded
        self.assertEqual(summary["corpus_wide"].get("computed", 0), 3)  # fab_computed + the 2 reclassified
        # DONE = the 2 reclassified (computed+grounded) + fab_computed
        # (already computed+grounded) + fab_textcomplete (display's own
        # text-complete->DONE rule, unrelated to this fix) = 4.
        self.assertEqual(summary["doneness"].get(producer.DONENESS_DONE, 0), 4)
        # fab_computed was already computed+grounded, plus the 2 reclassified = 3.
        self.assertEqual(summary["mechanically_confirmed_by_kind"].get("class_feature", 0), 3)

    def test_empty_case_is_a_real_zero_not_an_absent_field(self):
        """Decision 1a's anti-gaming doctrine: the empty case must fail
        closed -- an absent field reads identically to a broken run, a
        present field with count 0 reads as 'checked, none found'."""
        units = [{"id": "core_rulebook:spell:fab_spell", "book": "core_rulebook",
                   "kind": "spell", "wiring_class": "display", "status": "grounded"}]
        summary = producer.compute_wiring_class_summary(
            doc_path=self._doc(units), cache_path=self.cache_path
        )
        record = summary.get("classifier_reclassified_units")
        self.assertIsNotNone(record)
        self.assertEqual(record["count"], 0)
        self.assertEqual(record["units"], [])

    def test_doneness_verdict_itself_is_unchanged(self):
        """This fix reclassifies `wiring_class` at tally-time -- it must NOT
        touch `doneness_verdict()`'s own table. `display`+`grounded` still
        maps to `held`, exactly as before, for any unit this predicate does
        not reclassify."""
        self.assertEqual(
            producer.doneness_verdict("display", "grounded", "class_feature"),
            producer.DONENESS_HELD,
        )

    def test_reaches_work_inventory_panel(self):
        """The published panel (`work_inventory_panel()`) must carry the
        audit-trail field through from the cache -- this is the seam
        `build_pf1e_dashboard`/`main()` writes into the actual published
        JSON from, per `decisions.md §11` condition 2."""
        units = [
            {"id": "core_rulebook:class_feature:monk_evasion", "book": "core_rulebook",
             "kind": "class_feature", "wiring_class": "display", "status": "grounded",
             "evidence": self._EVIDENCE},
        ]
        summary = producer.compute_wiring_class_summary(
            doc_path=self._doc(units), cache_path=self.cache_path
        )
        inventory = {"totals": {"units": 1, "by_status": {}, "by_kind": {}}, "books": []}
        panel = producer.work_inventory_panel(inventory, wiring=summary)
        record = panel.get("classifier_reclassified_units")
        self.assertIsNotNone(record, "classifier_reclassified_units did not reach work_inventory_panel()")
        self.assertEqual(record["count"], 1)


class StaleSchemaCacheIsRejectedTest(unittest.TestCase):
    """SD-32 T8 follow-up (`decisions.md §11` re-verification): the T8 fix
    added `classifier_reclassified_units` to `compute_wiring_class_summary()`'s
    return shape but did not bump `WIRING_SUMMARY_SCHEMA` off its pre-T8 value
    of 12. A cache written by the PRE-fix producer therefore carries schema
    12, is NEWER than the source doc, and passes the warm-cache equality
    check unchanged -- so the fix never fires against a warm cache, which is
    exactly the shape the real `WIRING_CLASS_CACHE` was found in on the tip
    of this bundle. `ClassifierReclassifiedUnitsTest` above always calls
    `compute_wiring_class_summary()` against a COLD cache (a path that has
    never been written), so it cannot see this: it built the fresh summary
    every time and never exercised the cache-hit branch at all.

    This test writes a pre-T8-SHAPED cache -- schema 12 (the pre-T8 value,
    hardcoded here rather than read off the live constant, precisely so this
    test keeps proving the historical defect even after the constant is
    bumped), no `classifier_reclassified_units` field, and `display`/`computed`
    counts that reflect the UNCLASSIFIED (pre-fix) tally -- gives it a newer
    mtime than the source doc, and asserts the cache is rejected: the
    reclassification must fire and the returned summary must NOT be the
    stale cached object.

    Prove-it-can-fail: revert `WIRING_SUMMARY_SCHEMA` to `12` and re-run --
    this test goes red because the stale cache (also schema 12) then passes
    the equality check and is served verbatim, exactly the live defect this
    test was written to catch."""

    _EVIDENCE = "explanation_id_observed_in_a_real_computation"
    _PRE_T8_SCHEMA = 12  # the value WIRING_SUMMARY_SCHEMA held before this fix

    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)

    def test_pre_t8_schema_cache_is_rejected_and_reclassification_fires(self):
        units = [
            {"id": "core_rulebook:class_feature:monk_evasion", "book": "core_rulebook",
             "kind": "class_feature", "wiring_class": "display", "status": "grounded",
             "evidence": self._EVIDENCE},
        ]
        doc_path = os.path.join(self._tmp.name, "work-inventory.json")
        with open(doc_path, "w", encoding="utf-8") as f:
            json.dump({"generated_at": "2026-08-22T00:00:00Z", "units": units}, f)

        cache_path = os.path.join(self._tmp.name, "wiring-class-summary.json")
        # A pre-T8-SHAPED cache: schema 12 (this test's own pinned constant,
        # not the live one), `available: True`, `source_document` matching
        # this doc_path (so the P0.2 source-document guard does not itself
        # cause the rejection -- this test isolates the schema/field gap
        # specifically), no `classifier_reclassified_units` field, and
        # `display`/`computed` counts that are the PRE-fix (unreclassified)
        # tally -- exactly what a producer run before the T8 fix landed
        # would have written.
        stale_cache = {
            "available": True,
            "schema": self._PRE_T8_SCHEMA,
            "source_document": producer.publishable_document_path(doc_path),
            "corpus_wide": {"display": 1, "computed": 0},
            "doneness": {producer.DONENESS_HELD: 1},
            # deliberately no "classifier_reclassified_units" key -- the
            # pre-T8 shape.
        }
        with open(cache_path, "w", encoding="utf-8") as f:
            json.dump(stale_cache, f)
        # Cache must be strictly newer than the source doc for the warm-cache
        # branch to even be considered.
        doc_mtime = os.path.getmtime(doc_path)
        os.utime(cache_path, (doc_mtime + 10, doc_mtime + 10))

        summary = producer.compute_wiring_class_summary(
            doc_path=doc_path, cache_path=cache_path
        )

        # The stale cache must be REJECTED, not served verbatim: if it were
        # served, `classifier_reclassified_units` would be absent (the exact
        # live symptom) and `corpus_wide.computed` would stay 0.
        record = summary.get("classifier_reclassified_units")
        self.assertIsNotNone(
            record,
            "classifier_reclassified_units missing -- a pre-T8-schema warm "
            "cache was served instead of being recomputed",
        )
        self.assertEqual(record["count"], 1)
        self.assertEqual(summary["corpus_wide"].get("computed", 0), 1)
        self.assertEqual(summary["corpus_wide"].get("display", 0), 0)
        self.assertEqual(summary["doneness"].get(producer.DONENESS_DONE, 0), 1)


class WiringSummaryTopLevelKeysCanaryTest(unittest.TestCase):
    """General-defect hardening (SD-32 T8 follow-up, `decisions.md §11`
    condition 3): a field added to `compute_wiring_class_summary()`'s return
    dict without a `WIRING_SUMMARY_SCHEMA` bump is a recurring hazard, not a
    one-off -- this exact shape has now happened at least twice (schema
    11->12's own comment above the constant documents the prior incident,
    and the T8 fix this test file's `StaleSchemaCacheIsRejectedTest` covers
    is the second). The constant is only load-bearing if a human remembers
    to touch it; this canary makes forgetting loud instead of silent.

    Pins the exact top-level key set the function returns for a real
    (non-degenerate) summary, tied to the CURRENT `WIRING_SUMMARY_SCHEMA`
    value in the failure message. Any top-level key added, removed, or
    renamed on the `result` dict inside `compute_wiring_class_summary()`
    fails this test immediately -- forcing whoever touched the shape to
    consciously decide whether `WIRING_SUMMARY_SCHEMA` needs to move, rather
    than that decision silently defaulting to "no"."""

    _EXPECTED_TOP_LEVEL_KEYS = frozenset({
        "available", "schema", "generated_at", "source_document",
        "wiring_class_values", "corpus_wide", "by_book", "cross_tab",
        "cross_tab_by_book", "cross_tab_by_kind", "cross_tab_by_kind_by_book",
        "doneness_values", "doneness_meaning", "doneness", "doneness_by_book",
        "doneness_by_kind", "doneness_by_kind_by_book",
        "mechanically_confirmed_by_kind", "mechanically_confirmed_by_kind_by_book",
        "classifier_reclassified_units", "doneness_unmapped",
        "no_grounding_probe_kinds", "determinator_versions",
        # Present only when no unit in the source doc carries
        # `wiring_class_determinator_version` -- the fabricated doc this test
        # uses never does, so this key is always in the fixture's output.
        "determinator_version_note",
    })

    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)

    def test_top_level_keys_match_pinned_set_for_current_schema(self):
        doc_path = os.path.join(self._tmp.name, "work-inventory.json")
        with open(doc_path, "w", encoding="utf-8") as f:
            json.dump({"generated_at": "2026-08-22T00:00:00Z", "units": [
                {"id": "x", "book": "core_rulebook", "kind": "spell",
                 "wiring_class": "static", "status": "grounded"},
            ]}, f)
        cache_path = os.path.join(self._tmp.name, "wiring-class-summary.json")
        summary = producer.compute_wiring_class_summary(
            doc_path=doc_path, cache_path=cache_path
        )
        actual_keys = frozenset(summary.keys())
        self.assertEqual(
            actual_keys, self._EXPECTED_TOP_LEVEL_KEYS,
            "compute_wiring_class_summary()'s top-level key set changed "
            f"(schema is currently {producer.WIRING_SUMMARY_SCHEMA}) without "
            "updating this pinned set -- if the change was deliberate, update "
            "_EXPECTED_TOP_LEVEL_KEYS here AND bump WIRING_SUMMARY_SCHEMA in "
            "pf1e_dashboard_producer.py so every pre-change warm cache is "
            "correctly rejected (this is the exact T8 defect).",
        )


class BuildUnitShardsPiRedactionTest(unittest.TestCase):
    """Decision 12 (2026-08-17): `build_unit_shards` must withhold a unit's
    NAME when its own (book, source_file, source_line) row declares
    `NAMEISPI:YES` in the pinned oracle -- and must keep the row (count,
    status, wiring_class all unaffected) rather than dropping it. Mutation-
    proof: a declared-PI unit and a clean unit sit side by side; only the
    declared one is redacted."""

    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        # Fake pinned-oracle checkout: one book, one LST file, one
        # NAMEISPI:YES row and one clean row.
        oracle_root = os.path.join(self._tmp.name, "oracle")
        book_dir = os.path.join(
            oracle_root, "pathfinder", "paizo", "roleplaying_game", "ultimate_equipment"
        )
        os.makedirs(book_dir)
        with open(os.path.join(book_dir, "ue_equip.lst"), "w", encoding="utf-8") as f:
            f.write("Sturdy Rope\tCOST:1\tWT:5\n")
            f.write("Otyugh Hide\tNAMEISPI:YES\tCOST:1415\n")

        doc = {
            "generated_at": "2026-08-17T00:00:00Z",
            "units": [
                {
                    "id": "ultimate_equipment:equipment:sturdy_rope",
                    "book": "ultimate_equipment",
                    "kind": "equipment",
                    "name": "Sturdy Rope",
                    "source_file": "ue_equip.lst",
                    "source_line": 1,
                    "status": "grounded",
                    "wiring_class": "static",
                },
                {
                    "id": "ultimate_equipment:equipment:otyugh_hide",
                    "book": "ultimate_equipment",
                    "kind": "equipment",
                    "name": "Otyugh Hide",
                    "source_file": "ue_equip.lst",
                    "source_line": 2,
                    "status": "grounded",
                    "wiring_class": "static",
                },
            ],
        }
        self.doc_path = os.path.join(self._tmp.name, "fab-work-inventory.json")
        with open(self.doc_path, "w", encoding="utf-8") as f:
            json.dump(doc, f)
        self.shard_dir = os.path.join(self._tmp.name, "shards")

        self._env_patch = unittest.mock.patch.dict(
            os.environ, {"PCGEN_CORPUS_ROOT": oracle_root}
        )
        self._env_patch.start()
        self.addCleanup(self._env_patch.stop)

    def _rows_by_name(self, kind_shard_path):
        with open(kind_shard_path, encoding="utf-8") as f:
            shard = json.load(f)
        idx = shard["fields"].index("name")
        return [row[idx] for row in shard["rows"]]

    def test_declared_pi_name_is_withheld_but_the_row_survives(self):
        index = producer.build_unit_shards(doc_path=self.doc_path, shard_dir=self.shard_dir)
        self.assertTrue(index.get("available"), index.get("note"))
        self.assertTrue(index.get("pi_oracle_available"), "the fake oracle checkout must be found")
        self.assertEqual(index.get("pi_redacted_names"), 1)

        equipment = index["kinds"]["equipment"]
        # The row count is UNCHANGED -- both units still counted.
        self.assertEqual(equipment["units"], 2)

        shard_path = os.path.join(self.shard_dir, equipment["shard"])
        names = self._rows_by_name(shard_path)
        self.assertIn("Sturdy Rope", names, "the clean unit's real name must ship")
        self.assertNotIn("Otyugh Hide", names, "the declared-PI name must never ship")
        self.assertIn(
            producer.pi_redaction.REDACTED_PI_MARKER, names,
            "the declared-PI row must still be present, with its name withheld",
        )


class BuildUnitShardsWordBoundaryAndTypeFacetTest(unittest.TestCase):
    """FIX-DASHBOARD-PI (2026-08-17): `build_unit_shards` must ALSO catch a
    declared-PI name EMBEDDED in a `name` field with no `NAMEISPI:YES` row
    of its own (word-boundary, via `_PiScreen`) and a declared-PI name
    embedded in the raw `type_facet` compound identifier (plain substring,
    no allow-list) -- neither existed before this fix. Mutation-proof: a
    clean row and a leaking row of each kind sit side by side; only the
    leaking ones are redacted."""

    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        oracle_root = os.path.join(self._tmp.name, "oracle")
        book_dir = os.path.join(
            oracle_root, "pathfinder", "paizo", "roleplaying_game", "ultimate_equipment"
        )
        os.makedirs(book_dir)
        with open(os.path.join(book_dir, "ue_equip.lst"), "w", encoding="utf-8") as f:
            # "Rendmoor" is declared PI on its OWN row -- the embedding unit
            # below carries no such declaration on ITS row, so only the
            # word-boundary/substring layer can catch it.
            f.write("Rendmoor\tNAMEISPI:YES\tCOST:1\n")

        doc = {
            "generated_at": "2026-08-17T00:00:00Z",
            "units": [
                {
                    "id": "ultimate_equipment:equipment:sturdy_rope",
                    "book": "ultimate_equipment",
                    "kind": "equipment",
                    "name": "Sturdy Rope",
                    "type_facet": "Goods.Adventuring Gear",
                    "source_file": "ue_equip.lst",
                    "source_line": 99,
                    "status": "grounded",
                    "wiring_class": "static",
                },
                {
                    "id": "ultimate_equipment:equipment:blade_of_rendmoor",
                    "book": "ultimate_equipment",
                    "kind": "equipment",
                    "name": "Blade of Rendmoor",
                    "type_facet": "Weapon.Martial.Sword",
                    "source_file": "ue_equip.lst",
                    "source_line": 98,
                    "status": "grounded",
                    "wiring_class": "static",
                },
                {
                    "id": "ultimate_equipment:equipment:rendmoor_helm",
                    "book": "ultimate_equipment",
                    "kind": "equipment",
                    "name": "Ordinary Helm",
                    "type_facet": "ClassFeatures.Rendmoor Guard.SpecialQuality",
                    "source_file": "ue_equip.lst",
                    "source_line": 97,
                    "status": "grounded",
                    "wiring_class": "static",
                },
            ],
        }
        self.doc_path = os.path.join(self._tmp.name, "fab-work-inventory.json")
        with open(self.doc_path, "w", encoding="utf-8") as f:
            json.dump(doc, f)
        self.shard_dir = os.path.join(self._tmp.name, "shards")

        self._env_patch = unittest.mock.patch.dict(os.environ, {"PCGEN_CORPUS_ROOT": oracle_root})
        self._env_patch.start()
        self.addCleanup(self._env_patch.stop)

    def _rows(self, kind_shard_path):
        with open(kind_shard_path, encoding="utf-8") as f:
            return json.load(f)

    def test_word_boundary_embed_and_type_facet_substring_are_both_redacted(self):
        index = producer.build_unit_shards(doc_path=self.doc_path, shard_dir=self.shard_dir)
        self.assertTrue(index.get("available"), index.get("note"))
        equipment = index["kinds"]["equipment"]
        self.assertEqual(equipment["units"], 3, "no row is ever dropped for a name/type_facet hit")

        shard = self._rows(os.path.join(self.shard_dir, equipment["shard"]))
        name_idx = shard["fields"].index("name")
        tf_idx = shard["fields"].index("type_facet")
        # `build_unit_shards` preserves unit order within a kind, so pairing
        # the fabricated units (which carry a unique `source_line`) with the
        # shard's own rows positionally is a stable lookup.
        rows_by_line = {u["source_line"]: row for u, row in zip(doc_units_in_order(self.doc_path), shard["rows"])}

        clean = rows_by_line[99]
        self.assertEqual(clean[name_idx], "Sturdy Rope")
        self.assertEqual(clean[tf_idx], "Goods.Adventuring Gear", "an ordinary type_facet must ship unredacted")

        self.assertEqual(rows_by_line[98][name_idx], producer.pi_redaction.REDACTED_PI_MARKER,
                          "\"Blade of Rendmoor\" embeds the declared-PI name \"Rendmoor\" as a word")

        self.assertEqual(rows_by_line[97][name_idx], "Ordinary Helm",
                          "the name field itself carries no leak")
        self.assertEqual(rows_by_line[97][tf_idx], producer.pi_redaction.REDACTED_PI_MARKER,
                          "type_facet embeds \"Rendmoor\" as a substring and must be withheld")


def doc_units_in_order(doc_path):
    with open(doc_path, encoding="utf-8") as f:
        return json.load(f)["units"]


class ParseLstFirstFieldPiRedactionTest(unittest.TestCase):
    """`_parse_lst_first_field` feeds `_book_item_roster` (equipment/feats/
    spells rosters) AND `_prestige_class_roadmap`'s `ag_variants` --
    Decision 12's row-149 exposure. A declared-PI row's name must be
    withheld from the returned list while the row still occupies its own
    slot (dedup keys on the real name, not the redacted marker)."""

    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self.path = os.path.join(self._tmp.name, "fab_classes.lst")
        with open(self.path, "w", encoding="utf-8") as f:
            f.write("Ordinary Fighter\tHD:10\n")
            f.write("Aldori Swordlord\tNAMEISPI:YES\tHD:10\n")

    def test_declared_pi_row_is_redacted_but_still_present(self):
        names = producer._parse_lst_first_field(self.path)
        self.assertIn("Ordinary Fighter", names)
        self.assertNotIn("Aldori Swordlord", names)
        self.assertIn(producer.pi_redaction.REDACTED_PI_MARKER, names)
        self.assertEqual(len(names), 2, "the declared-PI row must not be dropped")


class PublishableDocumentPathTests(unittest.TestCase):
    """`unit_index.source_document` must be CHECKOUT-INDEPENDENT.

    WHY (SD31-W15-INTEGRATE-001, wave-15 adversarial finding, confirmed twice
    by two independent reviewers): the published feeds
    (`site/dashboard/PF1e-dashboard.json`, `site/dashboard/units/index.json`)
    recorded the ABSOLUTE filesystem path of whichever checkout published
    them. Two consequences, both real and both observed this wave:

      1. `verify.sh`'s `site-dashboard-check` compares the committed feed
         against a freshly generated one after a scrub that strips only
         timestamps. An absolute path is the ONE remaining leaf that differs
         between checkouts, so the stage reported STALE for every worktree
         other than the one that published -- a gate failing for a reason
         entirely unrelated to what it guards, which is how a gate gets
         baselined away (the mirror image of Decision 1(a)).
      2. A developer's home directory plus an ephemeral worktree id was
         committed into `site/`, the directory published to Cloudflare Pages.

    The fix is to record the document's path relative to the enclosing git
    checkout. Deliberately NOT relative to `DEFAULT_REPO_ROOT`: that is an
    env-var default pointing at the shared checkout, so it would still be
    wrong from a worktree. The enclosing checkout is found by walking up for
    `.git` (a directory in a normal clone, a FILE in a linked worktree -- both
    must work, and a worktree is precisely the case that broke).
    """

    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self.root = pathlib.Path(self._tmp.name)

    def _doc(self, marker_is_file: bool) -> str:
        (self.root / "docs").mkdir(parents=True, exist_ok=True)
        doc = self.root / "docs" / "work-inventory.json"
        doc.write_text("{}", encoding="utf-8")
        if marker_is_file:
            (self.root / ".git").write_text("gitdir: /somewhere/else\n", encoding="utf-8")
        else:
            (self.root / ".git").mkdir()
        return str(doc)

    def test_path_inside_a_normal_clone_is_repo_relative(self):
        self.assertEqual(
            producer.publishable_document_path(self._doc(marker_is_file=False)),
            "docs/work-inventory.json",
        )

    def test_path_inside_a_linked_worktree_is_repo_relative(self):
        """A linked worktree's `.git` is a FILE, not a directory. This is the
        case that produced the finding: all six wave-15 lanes ran in linked
        worktrees."""
        self.assertEqual(
            producer.publishable_document_path(self._doc(marker_is_file=True)),
            "docs/work-inventory.json",
        )

    def test_two_different_checkouts_yield_the_SAME_published_value(self):
        """The whole point: the value must not encode which checkout ran."""
        a = pathlib.Path(self._tmp.name) / "checkout-a"
        b = pathlib.Path(self._tmp.name) / "checkout-b"
        vals = []
        for root in (a, b):
            (root / "docs").mkdir(parents=True)
            (root / ".git").mkdir()
            doc = root / "docs" / "work-inventory.json"
            doc.write_text("{}", encoding="utf-8")
            vals.append(producer.publishable_document_path(str(doc)))
        self.assertEqual(vals[0], vals[1])
        self.assertEqual(vals[0], "docs/work-inventory.json")

    def test_a_document_outside_any_checkout_keeps_its_absolute_path(self):
        """Degrade VISIBLY, not silently: a doc that is not in a checkout has
        no repo-relative name, and inventing a bare basename would make two
        genuinely different documents compare equal."""
        loose = self.root / "loose.json"
        loose.write_text("{}", encoding="utf-8")
        self.assertEqual(
            producer.publishable_document_path(str(loose)), os.path.realpath(str(loose))
        )

    def test_no_published_index_field_carries_an_absolute_path(self):
        """End-to-end over the real emitter, not just the helper."""
        doc = self._doc(marker_is_file=False)
        with open(doc, "w", encoding="utf-8") as f:
            json.dump({"generated_at": "2026-08-19T00:00:00Z", "units": []}, f)
        shard_dir = self.root / "shards"
        index = producer.build_unit_shards(doc_path=doc, shard_dir=str(shard_dir))
        self.assertEqual(index.get("source_document"), "docs/work-inventory.json")
        self.assertNotIn(str(self.root), json.dumps(index))


if __name__ == "__main__":
    unittest.main()
