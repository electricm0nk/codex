"""Self-test for `scripts/site/build_public_status.py` (SITE-PUBSTATUS-001,
2026-08-17): PI screening (Decision 12), the public three-bucket doneness
mapping, and the standing/denominator wiring (Decision 14).

Every fixture below builds a SCRATCH pcgen-shaped tree (never the real
pinned oracle -- these tests must pass on a machine with no oracle checkout
at all) and a scratch site/dashboard/units-shaped ledger, and exercises the
real production functions against them, same posture
`test_pi_redaction.py`/`test_provenance.py` already take.

Run: python3 -m unittest scripts.tests.test_build_public_status
"""
from __future__ import annotations

import importlib.util
import os
import pathlib
import shutil
import sys
import tempfile
import unittest

_OBSERVER_DIR = pathlib.Path(__file__).resolve().parent.parent / "observer"
sys.path.insert(0, str(_OBSERVER_DIR))

_MODULE_PATH = pathlib.Path(__file__).resolve().parent.parent / "site" / "build_public_status.py"
_spec = importlib.util.spec_from_file_location("build_public_status", _MODULE_PATH)
bps = importlib.util.module_from_spec(_spec)
sys.modules["build_public_status"] = bps
_spec.loader.exec_module(bps)

import pi_redaction  # noqa: E402
import provenance  # noqa: E402
import pi_substring_allowlist as pi_allowlist  # noqa: E402


class Scratch:
    """A throwaway PCGen-shaped oracle tree, same pattern as
    test_pi_redaction.py's own Scratch fixture."""

    def __init__(self, name: str):
        self.root = pathlib.Path(tempfile.gettempdir()) / f"codex_bps_test_{name}_{os.getpid()}"
        shutil.rmtree(self.root, ignore_errors=True)
        self.root.mkdir(parents=True)

    def write(self, rel: str, contents: str) -> str:
        path = self.root / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(contents, encoding="utf-8")
        return str(path)

    def cleanup(self):
        shutil.rmtree(self.root, ignore_errors=True)


def item(kind="feat", book="core_rulebook", name="Ordinary Feat", doneness_raw="done", type_facet=None):
    return {"kind": kind, "book": book, "name": name, "doneness_raw": doneness_raw, "type_facet": type_facet}


class DonenessBucketTests(unittest.TestCase):
    """Public bucket table: done->done, held/in-progress->partial,
    not-started/unmeasurable/deferred->not-started."""

    def test_every_internal_value_maps_to_one_of_three_public_buckets(self):
        import pf1e_dashboard_producer as producer

        expected = {
            producer.DONENESS_DONE: "done",
            producer.DONENESS_HELD: "partial",
            producer.DONENESS_IN_PROGRESS: "partial",
            producer.DONENESS_NOT_STARTED: "not-started",
            producer.DONENESS_UNMEASURABLE: "not-started",
            producer.DONENESS_DEFERRED: "not-started",
        }
        self.assertEqual(bps.DONENESS_TO_PUBLIC, expected)

    def test_attach_public_doneness_replaces_the_raw_verdict(self):
        it = item(doneness_raw="held")
        standing_by = {(bps.object_id(it), it["book"]): provenance.ORIGIN}
        bps.attach_standing_and_public_doneness([it], standing_by)
        self.assertEqual(it["doneness"], "partial")
        self.assertNotIn("doneness_raw", it)


class StandingTests(unittest.TestCase):
    """Standing reuses provenance.classify_unambiguous rather than
    reimplementing it."""

    def test_single_book_object_is_origin(self):
        items = [item(name="Solo Feat", book="core_rulebook")]
        standing = bps.compute_standing(items)
        self.assertEqual(standing[(bps.object_id(items[0]), "core_rulebook")], provenance.ORIGIN)

    def test_multi_book_object_is_unclassified_not_guessed(self):
        items = [
            item(name="Reprinted Feat", book="core_rulebook"),
            item(name="Reprinted Feat", book="advanced_players_guide"),
        ]
        standing = bps.compute_standing(items)
        for it in items:
            self.assertEqual(standing[(bps.object_id(it), it["book"])], provenance.UNCLASSIFIED)

    def test_core_essentials_is_packaging_artifact(self):
        items = [item(name="Ghost Feat", book="core_essentials")]
        standing = bps.compute_standing(items)
        self.assertEqual(
            standing[(bps.object_id(items[0]), "core_essentials")],
            provenance.PACKAGING_ARTIFACT,
        )

    def test_denominator_standings_come_from_provenance_module(self):
        # Never hand-listed: must be the exact tuple provenance.py itself
        # defines as "counts toward the denominator."
        self.assertIs(bps.DENOMINATOR_STANDINGS, provenance.DENOMINATOR_STATUSES)


class RollupDenominatorTests(unittest.TestCase):
    """denominator = origin + variant only; everything else is visibly
    excluded, not silently dropped."""

    def test_unclassified_and_packaging_artifact_are_excluded_from_pct(self):
        items = [
            {"kind": "feat", "book": "b", "name": "a", "doneness": "done", "standing": provenance.ORIGIN},
            {"kind": "feat", "book": "b", "name": "b", "doneness": "done", "standing": provenance.UNCLASSIFIED},
            {"kind": "feat", "book": "b", "name": "c", "doneness": "not-started", "standing": provenance.PACKAGING_ARTIFACT},
        ]
        roll = bps._rollup(items)
        self.assertEqual(roll["denominator"], 1)
        self.assertEqual(roll["done"], 1)
        self.assertEqual(roll["pct"], 100.0)
        self.assertEqual(roll["excluded_from_percentage"], 2)
        self.assertEqual(
            roll["standing_breakdown"],
            {provenance.ORIGIN: 1, provenance.UNCLASSIFIED: 1, provenance.PACKAGING_ARTIFACT: 1},
        )

    def test_variant_counts_toward_denominator(self):
        items = [
            {"kind": "feat", "book": "b", "name": "a", "doneness": "done", "standing": provenance.VARIANT},
        ]
        roll = bps._rollup(items)
        self.assertEqual(roll["denominator"], 1)
        self.assertEqual(roll["done"], 1)

    def test_empty_denominator_reports_zero_pct_not_a_crash(self):
        items = [
            {"kind": "feat", "book": "b", "name": "a", "doneness": "done", "standing": provenance.UNCLASSIFIED},
        ]
        roll = bps._rollup(items)
        self.assertEqual(roll["denominator"], 0)
        self.assertEqual(roll["pct"], 0.0)
        self.assertEqual(roll["excluded_from_percentage"], 1)


class PiRedactionTests(unittest.TestCase):
    """Decision 12: withhold the name, keep the row."""

    def setUp(self):
        self.scratch = Scratch("pi_redact")
        self.addCleanup(self.scratch.cleanup)
        # A declared-PI feat in core_rulebook, and an ordinary non-PI feat
        # alongside it, same shape a real book directory would have.
        self.scratch.write(
            "pathfinder/paizo/roleplaying_game/core_rulebook/feats.lst",
            "Secret Feat\tNAMEISPI:YES\tTYPE:General\n"
            "Ordinary Feat\tNAMEISPI:NO\tTYPE:General\n",
        )
        self.name_to_books = pi_redaction.build_declared_pi_name_book_index(self.scratch.root)
        self.declared_names = pi_redaction.build_declared_pi_name_index(self.scratch.root)

    def test_declared_pi_name_is_replaced_row_survives(self):
        items = [
            item(name="Secret Feat", book="core_rulebook", doneness_raw="done"),
        ]
        standing_by = bps.compute_standing(items)
        bps.attach_standing_and_public_doneness(items, standing_by)
        bps.redact_for_display(items, self.name_to_books, self.declared_names)
        self.assertEqual(items[0]["name"], pi_redaction.REDACTED_PI_MARKER)
        # The row survives: doneness and standing are untouched by redaction.
        self.assertEqual(items[0]["doneness"], "done")
        self.assertEqual(items[0]["standing"], provenance.ORIGIN)

    def test_non_pi_name_is_left_alone(self):
        items = [item(name="Ordinary Feat", book="core_rulebook")]
        bps.redact_for_display(items, self.name_to_books, self.declared_names)
        self.assertEqual(items[0]["name"], "Ordinary Feat")

    def test_type_facet_carrying_a_declared_pi_name_is_fully_redacted(self):
        items = [item(name="Ordinary Feat", book="core_rulebook", type_facet="SecretFeatClassFeatures.SpecialQuality")]
        # "Secret Feat" as two-word declared name won't substring-match a
        # camelCase facet; use a single-token declared name instead to
        # exercise the real leak shape (compound identifier embedding).
        self.scratch.write(
            "pathfinder/paizo/roleplaying_game/core_rulebook/deities.lst",
            "Magaambya\tNAMEISPI:YES\tTYPE:Location\n",
        )
        name_to_books = pi_redaction.build_declared_pi_name_book_index(self.scratch.root)
        declared_names = pi_redaction.build_declared_pi_name_index(self.scratch.root)
        items[0]["type_facet"] = "MagaambyanInitiateClassFeatures.SpecialQuality.Supernatural"
        bps.redact_for_display(items, name_to_books, declared_names)
        self.assertEqual(items[0]["type_facet"], pi_redaction.REDACTED_PI_MARKER)
        # Name itself is untouched -- only the leaking field is withheld.
        self.assertEqual(items[0]["name"], "Ordinary Feat")

    def test_type_facet_with_no_declared_name_substring_is_untouched(self):
        items = [item(name="Ordinary Feat", type_facet="AegisClassFeatures.SpecialQuality.Supernatural")]
        bps.redact_for_display(items, self.name_to_books, self.declared_names)
        self.assertEqual(items[0]["type_facet"], "AegisClassFeatures.SpecialQuality.Supernatural")

    def test_name_carrying_a_same_book_declared_pi_name_is_redacted(self):
        # SD31-W13-INTEGRATE-001-VERIFY finding 1: a declared-PI proper
        # noun surfacing as an embedded substring of a published item name
        # (e.g. "Abadar's Truthtelling" carrying the declared-PI deity
        # name "Abadar") was invisible to the old exact-match-only `name`
        # check. Same-book word-boundary screening must catch it.
        self.scratch.write(
            "pathfinder/paizo/roleplaying_game/core_rulebook/deities.lst",
            "Abadar\tNAMEISPI:YES\tTYPE:Deity\n",
        )
        name_to_books = pi_redaction.build_declared_pi_name_book_index(self.scratch.root)
        declared_names = pi_redaction.build_declared_pi_name_index(self.scratch.root)
        items = [item(name="Abadar's Truthtelling", book="core_rulebook")]
        bps.redact_for_display(items, name_to_books, declared_names)
        self.assertEqual(items[0]["name"], pi_redaction.REDACTED_PI_MARKER)

    def test_name_word_match_from_an_unrelated_book_is_not_redacted(self):
        # The documented "Brigh"-in-"Brightness" false-positive class: a
        # declared-PI word ("Brigh") from a DIFFERENT book must not flag an
        # unrelated, ordinary name ("Brightness Seeker") merely because it
        # happens to start with the same letters -- WORD-BOUNDARY matching
        # (not book-scoping) is what keeps this from over-redacting: "Brigh"
        # is fused into "Brightness" (followed by "t"), never a candidate
        # regardless of which book declared it.
        self.scratch.write(
            "pathfinder/paizo/roleplaying_game/inner_sea_gods/deities.lst",
            "Brigh\tNAMEISPI:YES\tTYPE:Deity\n",
        )
        name_to_books = pi_redaction.build_declared_pi_name_book_index(self.scratch.root)
        declared_names = pi_redaction.build_declared_pi_name_index(self.scratch.root)
        items = [item(name="Brightness Seeker", book="core_rulebook")]
        bps.redact_for_display(items, name_to_books, declared_names)
        self.assertEqual(items[0]["name"], "Brightness Seeker")

    def test_name_carrying_a_CROSS_book_declared_pi_name_is_redacted(self):
        # SITE-PI-ALLOWLIST-001's own finding: "Death (Pharasma)", published
        # in advanced_players_guide, embeds the deity name "Pharasma" --
        # declared PI under a COMPLETELY DIFFERENT book directory
        # (inner_sea_gods in the real corpus). Book-scoped-only matching is
        # blind to this; the GLOBAL unambiguous pass must still catch it.
        self.scratch.write(
            "pathfinder/paizo/campaign_setting/inner_sea_gods/deities.lst",
            "Pharasma\tNAMEISPI:YES\tTYPE:Deity\n",
        )
        name_to_books = pi_redaction.build_declared_pi_name_book_index(self.scratch.root)
        declared_names = pi_redaction.build_declared_pi_name_index(self.scratch.root)
        items = [item(name="Death (Pharasma)", book="advanced_players_guide")]
        bps.redact_for_display(items, name_to_books, declared_names)
        self.assertEqual(items[0]["name"], pi_redaction.REDACTED_PI_MARKER)

    def test_name_declared_pi_in_own_book_but_globally_ambiguous_is_still_redacted(self):
        # SITE-PI-ALLOWLIST-001 mutation finding: the GLOBAL unambiguous
        # set (build_declared_pi_name_index) drops any name that ALSO
        # appears, non-PI, in an unrelated third-party/SRD book anywhere in
        # the wider oracle checkout -- e.g. the real "Baphomet" is declared
        # PI in inner_sea_gods but ALSO has a second, unrelated row
        # elsewhere under the scanned Paizo tree (a PFS-legality-override
        # row, matching the real corpus's actual "Baphomet" shape) that
        # carries no NAMEISPI token at all -- build_declared_pi_name_index's
        # pi_names-minus-non_pi_names subtraction (see its own "UNAMBIGUOUS
        # is load-bearing" docstring) then drops the bare name from the
        # GLOBAL set even though it is genuinely, unambiguously declared PI
        # in inner_sea_gods specifically. Using ONLY the global set would
        # silently un-redact a genuine same-book leak; the per-book source
        # (name_to_books) must still be checked.
        self.scratch.write(
            "pathfinder/paizo/campaign_setting/inner_sea_gods/deities.lst",
            "Baphomet\tNAMEISPI:YES\tTYPE:Deity\n",
        )
        self.scratch.write(
            "pathfinder/paizo/player_companion/faiths_of_corruption/_pfs/pfs_foc_deities.lst",
            "Baphomet\t!PRECHARACTERTYPE:1,PC\tTYPE:PFSNotLegal\n",  # no NAMEISPI token -- ambiguity source
        )
        name_to_books = pi_redaction.build_declared_pi_name_book_index(self.scratch.root)
        declared_names = pi_redaction.build_declared_pi_name_index(self.scratch.root)
        # Confirm the ambiguity actually landed as intended before relying on it.
        self.assertNotIn("Baphomet", declared_names)
        items = [item(name="Baphomet's Blessing", book="inner_sea_gods")]
        bps.redact_for_display(items, name_to_books, declared_names)
        self.assertEqual(items[0]["name"], pi_redaction.REDACTED_PI_MARKER)

    def test_allowlisted_name_and_book_is_published_despite_a_word_match(self):
        self.scratch.write(
            "pathfinder/paizo/campaign_setting/inner_sea_world_guide/regions.lst",
            "Shackles\tNAMEISPI:YES\tTYPE:Region\n",
        )
        name_to_books = pi_redaction.build_declared_pi_name_book_index(self.scratch.root)
        declared_names = pi_redaction.build_declared_pi_name_index(self.scratch.root)
        # "Dimensional Shackles" / core_rulebook is a real allow-list entry.
        items = [item(name="Dimensional Shackles", book="core_rulebook")]
        bps.redact_for_display(items, name_to_books, declared_names)
        self.assertEqual(items[0]["name"], "Dimensional Shackles")

    def test_allowlisted_name_in_an_UNLISTED_book_is_still_redacted(self):
        # The allow-list is keyed on (name, book) together, never on the
        # name alone -- publishing the exact same name string under a book
        # the review never covered must not silently inherit clearance.
        self.scratch.write(
            "pathfinder/paizo/campaign_setting/inner_sea_world_guide/regions.lst",
            "Shackles\tNAMEISPI:YES\tTYPE:Region\n",
        )
        name_to_books = pi_redaction.build_declared_pi_name_book_index(self.scratch.root)
        declared_names = pi_redaction.build_declared_pi_name_index(self.scratch.root)
        items = [item(name="Dimensional Shackles", book="ultimate_magic")]
        bps.redact_for_display(items, name_to_books, declared_names)
        self.assertEqual(items[0]["name"], pi_redaction.REDACTED_PI_MARKER)

    def test_a_brand_new_unlisted_word_match_is_redacted(self):
        # Mutation-proof (b)/(a) analogue at the producer layer: a name
        # never reviewed onto the allow-list is redacted, not published,
        # the moment it embeds a declared-PI word as a whole word.
        self.scratch.write(
            "pathfinder/paizo/campaign_setting/inner_sea_gods/deities.lst",
            "Iomedae\tNAMEISPI:YES\tTYPE:Deity\n",
        )
        name_to_books = pi_redaction.build_declared_pi_name_book_index(self.scratch.root)
        declared_names = pi_redaction.build_declared_pi_name_index(self.scratch.root)
        items = [item(name="Shrine of Iomedae Replica", book="core_rulebook")]
        bps.redact_for_display(items, name_to_books, declared_names)
        self.assertEqual(items[0]["name"], pi_redaction.REDACTED_PI_MARKER)

    def test_standing_uses_the_true_name_not_the_redacted_marker(self):
        # Two DIFFERENT declared-PI objects, each printed once, in
        # different books -- if provenance ran on the redacted marker
        # instead of the true name, both would collapse into one fake
        # "object" spanning two books and wrongly come back unclassified.
        self.scratch.write(
            "pathfinder/paizo/roleplaying_game/advanced_players_guide/feats.lst",
            "Other Secret Feat\tNAMEISPI:YES\tTYPE:General\n",
        )
        name_to_books = pi_redaction.build_declared_pi_name_book_index(self.scratch.root)
        declared_names = pi_redaction.build_declared_pi_name_index(self.scratch.root)
        items = [
            item(name="Secret Feat", book="core_rulebook"),
            item(name="Other Secret Feat", book="advanced_players_guide"),
        ]
        standing_by = bps.compute_standing(items)
        bps.attach_standing_and_public_doneness(items, standing_by)
        bps.redact_for_display(items, name_to_books, declared_names)
        for it in items:
            self.assertEqual(it["name"], pi_redaction.REDACTED_PI_MARKER)
            self.assertEqual(it["standing"], provenance.ORIGIN)


class PiSubstringAllowlistTests(unittest.TestCase):
    """SITE-PI-ALLOWLIST-001 requirement #4: the allow-list must stay a
    reviewed, checkable list, not a silent hiding place."""

    def test_the_real_allowlist_loads_and_every_entry_has_a_reason(self):
        index = pi_allowlist.build_allowlist_index()
        self.assertGreater(len(index), 0)
        for name, entry in index.items():
            with self.subTest(name=name):
                self.assertTrue(entry["reason"].strip(), f"{name!r} has a blank reason")
                self.assertTrue(entry["books"], f"{name!r} has no books")

    def test_the_real_allowlist_stays_short(self):
        # Not a hard ceiling -- a documented, deliberate exception -- but a
        # loud tripwire: this list growing past a small, hand-reviewable
        # size unnoticed is exactly the "hiding place" risk the operator
        # flagged. Bump this only alongside a real re-read of every entry.
        self.assertLessEqual(
            len(pi_allowlist.ALLOWLIST), 20,
            "pi_substring_allowlist.ALLOWLIST has grown past 20 entries -- "
            "re-read every entry (see the file's own module docstring) "
            "before raising this ceiling, not just the new one",
        )

    def test_an_entry_missing_a_reason_fails_to_load(self):
        original = pi_allowlist.ALLOWLIST
        try:
            pi_allowlist.ALLOWLIST = [
                {"name": "Test Name", "term": "Test", "books": ["core_rulebook"], "reason": "   "},
            ]
            with self.assertRaises(ValueError):
                pi_allowlist.build_allowlist_index()
        finally:
            pi_allowlist.ALLOWLIST = original

    def test_an_entry_missing_books_fails_to_load(self):
        original = pi_allowlist.ALLOWLIST
        try:
            pi_allowlist.ALLOWLIST = [
                {"name": "Test Name", "term": "Test", "books": [], "reason": "A real reason."},
            ]
            with self.assertRaises(ValueError):
                pi_allowlist.build_allowlist_index()
        finally:
            pi_allowlist.ALLOWLIST = original

    def test_a_duplicate_name_fails_to_load(self):
        original = pi_allowlist.ALLOWLIST
        try:
            pi_allowlist.ALLOWLIST = [
                {"name": "Test Name", "term": "Test", "books": ["a"], "reason": "First."},
                {"name": "Test Name", "term": "Test", "books": ["b"], "reason": "Second."},
            ]
            with self.assertRaises(ValueError):
                pi_allowlist.build_allowlist_index()
        finally:
            pi_allowlist.ALLOWLIST = original

    def test_is_allowlisted_requires_both_name_and_book(self):
        index = {"Widget": {"name": "Widget", "term": "Wid", "books": ["core_rulebook"], "reason": "x"}}
        self.assertTrue(pi_allowlist.is_allowlisted("Widget", "core_rulebook", index))
        self.assertFalse(pi_allowlist.is_allowlisted("Widget", "ultimate_magic", index))
        self.assertFalse(pi_allowlist.is_allowlisted("Other Widget", "core_rulebook", index))


class DisplayNameDisambiguationTests(unittest.TestCase):
    """Decision 17 / `OPERATOR-RULINGS-2026-08-19.md` Ruling §17's drill-down
    display defect: the page shows a bare `name` that is not unique across
    ~4,266 units corpus-wide (0 share a `corpus_key`, 0 share a
    `source_file`+`source_line` -- every one is a distinct printed row, so
    the fix is a disambiguator, never a merge). `add_display_names` is the
    per-(book, kind) item-list pass that gives every item a `display_name`
    that IS unique within its own list, built only from fields already
    through PI redaction (`name`, `type_facet`) by the time
    `build_book_details` calls it -- see that function's own docstring for
    why no new field/lookup is introduced that could bypass
    `redact_for_display`."""

    def _bare(self, name, type_facet=None):
        return {"name": name, "doneness": "not-started", "type_facet": type_facet, "standing": "origin"}

    def test_a_unique_name_gets_an_unchanged_display_name(self):
        items = [self._bare("Iron Will")]
        bps.add_display_names(items)
        self.assertEqual(items[0]["display_name"], "Iron Will")

    def test_colliding_names_are_disambiguated_by_type_facet(self):
        # The operator's own worked example: core_rulebook class_feature
        # "Aberrant Bloodline" listed twice, once as the class feature and
        # once as the SorcererBloodlineChoice picker option.
        items = [
            self._bare("Aberrant Bloodline", type_facet="Class Feature.Sorcerer Bloodline"),
            self._bare("Aberrant Bloodline", type_facet="SorcererBloodlineChoice"),
        ]
        bps.add_display_names(items)
        labels = sorted(it["display_name"] for it in items)
        self.assertEqual(len(set(labels)), 2, "colliding rows must get distinct display_name values")
        self.assertTrue(all(label.startswith("Aberrant Bloodline") for label in labels))
        self.assertIn("Class Feature.Sorcerer Bloodline", labels[0] + labels[1])
        self.assertIn("SorcererBloodlineChoice", labels[0] + labels[1])

    def test_eleven_bloodline_powers_rows_stay_eleven_distinct_labels(self):
        # advanced_class_guide's real shape: 11 different bloodlines each
        # print their own "Bloodline Powers" row. Collapsing them would
        # destroy real content (Ruling §17) -- disambiguation must produce
        # 11 DISTINCT labels, not merge them into one.
        items = [
            self._bare("Bloodline Powers", type_facet=f"Bloodline{i}Choice") for i in range(11)
        ]
        bps.add_display_names(items)
        labels = {it["display_name"] for it in items}
        self.assertEqual(len(labels), 11)

    def test_collision_with_no_type_facet_falls_back_to_a_positional_suffix(self):
        items = [self._bare("Mystery Row"), self._bare("Mystery Row")]
        bps.add_display_names(items)
        labels = {it["display_name"] for it in items}
        self.assertEqual(len(labels), 2, "must still disambiguate when type_facet is absent")
        self.assertTrue(all(label.startswith("Mystery Row") for label in labels))

    def test_collision_with_identical_type_facet_still_disambiguates(self):
        items = [
            self._bare("Twin Row", type_facet="SameChoice"),
            self._bare("Twin Row", type_facet="SameChoice"),
        ]
        bps.add_display_names(items)
        labels = {it["display_name"] for it in items}
        self.assertEqual(len(labels), 2)

    def test_build_book_details_wires_display_name_through_for_a_real_collision(self):
        # The end-to-end wiring proof: build_book_details is the actual
        # call site the drill-down JSON comes from. Feeding it two
        # already-classified/redacted items that collide on `name`
        # (the operator's own "Aberrant Bloodline" shape) must produce
        # `display_name` values that differ, inside the REAL output
        # structure the site reads (`kinds[].items[]`), not just via a
        # direct call to `add_display_names`.
        all_items = [
            {
                "kind": "class_feature", "book": "core_rulebook",
                "name": "Aberrant Bloodline", "doneness": "not-started",
                "type_facet": "Class Feature.Sorcerer Bloodline", "standing": "origin",
            },
            {
                "kind": "class_feature", "book": "core_rulebook",
                "name": "Aberrant Bloodline", "doneness": "not-started",
                "type_facet": "SorcererBloodlineChoice", "standing": "origin",
            },
        ]
        details = bps.build_book_details(all_items)
        kind_entry = details["core_rulebook"]["kinds"][0]
        self.assertEqual(kind_entry["kind"], "class_feature")
        display_names = sorted(it["display_name"] for it in kind_entry["items"])
        self.assertEqual(len(set(display_names)), 2, "the wired pipeline must disambiguate too")
        self.assertTrue(all(n.startswith("Aberrant Bloodline") for n in display_names))

    def test_display_name_never_exposes_more_than_already_redacted_fields(self):
        # Both name AND type_facet are already REDACTED_PI_MARKER by the
        # time this runs (redact_for_display ran first) -- disambiguation
        # must not reach past those two fields for anything else, and a
        # positional suffix built from redacted values leaks nothing new.
        marker = pi_redaction.REDACTED_PI_MARKER
        items = [self._bare(marker, type_facet=marker), self._bare(marker, type_facet=marker)]
        bps.add_display_names(items)
        for it in items:
            self.assertIn(marker, it["display_name"])
            # every non-marker character must come from the disambiguation
            # scaffolding itself (parens, digits, '#'), never a leaked field
            leftover = it["display_name"].replace(marker, "")
            self.assertTrue(all(ch in " ()#0123456789" for ch in leftover), it["display_name"])


class LoadUnitsByKindTests(unittest.TestCase):
    def setUp(self):
        self.scratch = Scratch("units_dir")
        self.addCleanup(self.scratch.cleanup)

    def test_unknown_kind_raises_loud(self):
        import json

        path = self.scratch.write(
            "PF1e-units-mystery.json",
            json.dumps({"kind": "mystery", "fields": ["name", "book", "status", "wiring_class"], "rows": []}),
        )
        with self.assertRaises(KeyError):
            bps.load_units_by_kind(self.scratch.root)


class LiveKindCoverageTests(unittest.TestCase):
    """SITE-PUBSTATUS-002: the sibling pf1e-dashboard-producer lane
    regenerated site/dashboard/units/*.json with 8 newly-classified kinds
    the committed shard index had never carried (ability, deity, domain,
    language, power, skill, template, trait) -- 4,337 `ability` rows alone
    -- and build_public_status.py's own load_units_by_kind fail-loud check
    (KIND_LABELS is a curated allow-list, deliberately not "every kind
    seen") crashed on the very first of them before writing anything.

    This test reads the REAL, checked-in site/dashboard/units/ ledger
    (not a scratch fixture -- the defect is specifically that the curated
    label map fell behind the real, committed kind set) and proves every
    kind it contains has a KIND_LABELS entry, so load_units_by_kind can
    actually load the whole live ledger without raising."""

    UNITS_DIR = pathlib.Path(__file__).resolve().parent.parent.parent / "site" / "dashboard" / "units"

    def test_every_live_unit_kind_has_a_curated_label(self):
        live_kinds = set()
        for path in sorted(self.UNITS_DIR.glob("PF1e-units-*.json")):
            import json

            live_kinds.add(json.loads(path.read_text())["kind"])
        self.assertTrue(live_kinds, "no PF1e-units-*.json ledgers found -- check UNITS_DIR")
        missing = sorted(live_kinds - set(bps.KIND_LABELS))
        self.assertEqual(
            missing, [],
            f"KIND_LABELS is missing curated label(s) for live kind(s) {missing} -- "
            "load_units_by_kind will raise KeyError on the real ledger",
        )

    def test_load_units_by_kind_succeeds_against_the_real_live_ledger(self):
        # The actual crash site: not just a set-difference check, but the
        # real function running over the real, committed directory.
        by_kind = bps.load_units_by_kind(self.UNITS_DIR)
        self.assertIn("ability", by_kind)
        self.assertGreater(len(by_kind["ability"]), 0)


if __name__ == "__main__":
    unittest.main()
