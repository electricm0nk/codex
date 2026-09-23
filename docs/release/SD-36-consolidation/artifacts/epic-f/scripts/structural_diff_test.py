"""Pin for `structural_diff.py` becoming a gate (SD-36 Epic F1 re-check round 1, finding 2).

Reproduces, on synthetic fixtures (never the tracked `data/sheet_rules` package), the six
mutations the re-check ran by hand against a real fresh `--dump`:

    (a) a `value` field changed                          -> unexpected field delta
    (b) a rule removed entirely                           -> removed rule id
    (c) a `label` field renamed                            -> unexpected field delta
    (d) `_report.json` `records` count moved by one         -> moved count
    (e) a rule's sole `granted_by` edge deleted              -> removed granted_by edge
    (f) a rule's `grants` content replaced by a fabrication  -> removed grant

Before the fix, (a)/(c) were reported but did not gate (script always exited 0); (b) was
reported as a removed rule id but likewise never gated; (d) was printed as an unflagged pair;
(e) and (f) produced NO change in the report at all. This pin proves all six now gate (exit 1)
and are each named in the printed report.

Round 2 (finding 2) adds four more mutations `grant_signature` alone (wrapper-agnostic and
`WeaponSet`-content-blind in BOTH directions) could not see either, now closed by
`grant_covers`'s directional gate check and `WeaponSet` member-subset check:

    (g1) a `GatedFactGrant`'s gate deleted (-> bare `FactGrant`)        -> removed grant
    (g2) a `GatedFactGrant`'s `when` swapped for a different condition  -> removed grant
    (g3) a `WeaponSet`'s members replaced under the same label          -> removed grant
    (g4) a `WeaponSet`'s members emptied under the same label           -> removed grant

plus one non-mutation pin: a `WeaponSet` whose members merely GROW under the same label (a real
oracle-driven set growth, or F1-3's own bare-tag expansion) must not gate.

Round 3 (finding 1) pins the two shapes `missing_grant_signatures`'s greedy, order-dependent,
non-deduplicating match got wrong on the real corpus:

    (h1) the real `picaroon_weapon_proficiency` shape -- three baseline bare `WeaponGroup`
         grants (two byte-identical, one a case variant) folding, after F1-3's dedup, to the ONE
         fresh `WeaponSet` naming them all -- a real dedup, not a loss, must not gate.
    (h2) a reordered identical multiset -- baseline `[bare A, gated A(when X)]`, fresh
         `[gated A(when X), bare A]` (the same two grants, swapped) -- the greedy scan's first
         pick (bare A covers the lenient bare-covers-gated rule against fresh's gated A) used up
         the fresh grant the second baseline grant actually needed, reporting a spurious loss;
         the maximum bipartite match must not gate.

Run:
    python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff_test.py
"""
from __future__ import annotations

import io
import json
import os
import sys
import tempfile
import unittest
from contextlib import redirect_stdout

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import structural_diff  # noqa: E402


def write_tree(root: str, rules_by_file: dict[str, list[dict]], report: dict) -> None:
    for rel, rules in rules_by_file.items():
        path = os.path.join(root, rel)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf-8") as fh:
            json.dump(rules, fh)
    with open(os.path.join(root, "_report.json"), "w", encoding="utf-8") as fh:
        json.dump(report, fh)


BASE_REPORT = {"records": 49450, "converted": 40000, "refused": 9450, "rules_written": 71863, "var_tables": 100}


def base_rules() -> dict[str, list[dict]]:
    return {
        "core_rulebook/class_feature/arcane_bloodline_school_power.json": [
            {
                "id": "core_rulebook:class_feature:arcane_bloodline_school_power",
                "label": "Arcane Bloodline School Power",
                "value": {"Number": {"Const": 1}},
                "granted_by": [],
                "grants": [],
            }
        ],
        "core_rulebook/class_feature/wizard_weapon_and_armor_proficiency.json": [
            {
                "id": "core_rulebook:class_feature:wizard_weapon_and_armor_proficiency",
                "label": "Weapon and Armor Proficiency",
                "value": "Text",
                "granted_by": [],
                "grants": [],
            }
        ],
        "core_rulebook/class_feature/single_simple_weapon_proficiency.json": [
            {
                "id": "core_rulebook:class_feature:single_simple_weapon_proficiency",
                "label": "Simple Weapon Proficiency",
                "value": "Text",
                "granted_by": [],
                "grants": [],
            }
        ],
        "ultimate_combat/class_feature/samurai_proficiencies.json": [
            {
                "id": "ultimate_combat:class_feature:samurai_proficiencies",
                "label": "Samurai Proficiencies",
                "value": "Text",
                "granted_by": [{"by": {"Rule": "ultimate_combat:ability:samurai"}, "when": None}],
                "grants": [{"FactGrant": {"Proficiency": {"WeaponGroup": "Samurai"}}}],
            }
        ],
    }


class StructuralDiffGateTest(unittest.TestCase):
    def run_diff(self, base: dict[str, list[dict]], fresh: dict[str, list[dict]], fresh_report: dict | None = None) -> tuple[int, str]:
        with tempfile.TemporaryDirectory() as baseline_dir, tempfile.TemporaryDirectory() as scratch_dir:
            write_tree(baseline_dir, base, BASE_REPORT)
            write_tree(scratch_dir, fresh, fresh_report if fresh_report is not None else BASE_REPORT)
            argv = sys.argv
            sys.argv = ["structural_diff.py", scratch_dir, "--baseline", baseline_dir]
            buf = io.StringIO()
            try:
                with redirect_stdout(buf):
                    code = structural_diff.main()
            finally:
                sys.argv = argv
            return code, buf.getvalue()

    def test_unchanged_tree_passes(self):
        rules = base_rules()
        code, out = self.run_diff(rules, rules)
        self.assertEqual(code, 0, out)
        self.assertIn("verdict=PASS", out)

    def test_a_value_field_change_gates(self):
        base = base_rules()
        fresh = base_rules()
        fresh["core_rulebook/class_feature/arcane_bloodline_school_power.json"][0]["value"] = {"Number": {"Const": 7}}
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("unexpected field deltas: 1", out)
        self.assertIn("arcane_bloodline_school_power: value", out)

    def test_b_removed_rule_gates(self):
        base = base_rules()
        fresh = base_rules()
        del fresh["core_rulebook/class_feature/wizard_weapon_and_armor_proficiency.json"]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("removed rule ids: 1", out)
        self.assertIn("wizard_weapon_and_armor_proficiency", out)

    def test_c_renamed_field_gates(self):
        base = base_rules()
        fresh = base_rules()
        rec = fresh["core_rulebook/class_feature/single_simple_weapon_proficiency.json"][0]
        rec["labell"] = rec.pop("label")
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("unexpected field deltas:", out)
        self.assertNotIn("unexpected field deltas: 0", out)

    def test_d_moved_record_count_gates(self):
        rules = base_rules()
        fresh_report = dict(BASE_REPORT)
        fresh_report["records"] = BASE_REPORT["records"] - 1
        code, out = self.run_diff(rules, rules, fresh_report=fresh_report)
        self.assertEqual(code, 1, out)
        self.assertIn("records moved: 49450 -> 49449", out)

    def test_e_deleted_granted_by_edge_gates(self):
        base = base_rules()
        fresh = base_rules()
        fresh["ultimate_combat/class_feature/samurai_proficiencies.json"][0]["granted_by"] = []
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("removed granted_by edges: 1", out)
        self.assertIn("samurai_proficiencies", out)

    def test_f_fabricated_grant_gates(self):
        base = base_rules()
        fresh = base_rules()
        fresh["ultimate_combat/class_feature/samurai_proficiencies.json"][0]["grants"] = [
            {"FactGrant": {"Proficiency": {"WeaponGroup": "TOTALLY FABRICATED"}}}
        ]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("removed grants: 1", out)
        self.assertIn("samurai_proficiencies", out)

    def test_f1_2_gating_wrap_of_the_same_fact_does_not_gate(self):
        """F1-2 wraps a bare `FactGrant` in a `GatedFactGrant` around the identical fact when a
        PRE-gate applies -- a real, sanctioned transformation this branch ships (not a removal).
        Reproduces the false positive the round-1 fix's raw-JSON `edge_diff` produced against the
        real corpus (`brawler_weapon_and_armor_proficiency`) before `grant_signature` replaced it."""
        base = base_rules()
        fresh = base_rules()
        rec = fresh["ultimate_combat/class_feature/samurai_proficiencies.json"][0]
        rec["grants"] = [
            {
                "GatedFactGrant": {
                    "fact": {"Proficiency": {"WeaponGroup": "Samurai"}},
                    "when": {"Compare": {"lhs": {"Var": "v1"}, "op": "Eq", "rhs": {"Const": 0}}},
                }
            }
        ]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 0, out)
        self.assertIn("removed grants: 0", out)

    def test_f1_3_bare_tag_to_weapon_set_upgrade_does_not_gate(self):
        """F1-3 expands a bare `WeaponGroup`/`WeaponTag` into an equivalent `WeaponSet` naming the
        SAME tag (case-insensitive) with a resolved member list -- a real, sanctioned
        transformation (not a removal). Reproduces the false positive the round-1 fix produced
        against the real corpus (`musketeer_weapon_proficiency`'s `OnehandedFirearm`) before
        `grant_signature` replaced the raw-JSON edge diff."""
        base = base_rules()
        base["advanced_class_guide/class_feature/musketeer_weapon_proficiency.json"] = [
            {
                "id": "advanced_class_guide:class_feature:musketeer_weapon_proficiency",
                "label": "Musketeer Weapon Proficiency",
                "value": "Text",
                "granted_by": [],
                "grants": [{"FactGrant": {"Proficiency": {"WeaponGroup": "OnehandedFirearm"}}}],
            }
        ]
        fresh = base_rules()
        fresh["advanced_class_guide/class_feature/musketeer_weapon_proficiency.json"] = [
            {
                "id": "advanced_class_guide:class_feature:musketeer_weapon_proficiency",
                "label": "Musketeer Weapon Proficiency",
                "value": "Text",
                "granted_by": [],
                "grants": [
                    {
                        "FactGrant": {
                            "Proficiency": {
                                "WeaponSet": {"label": "OnehandedFirearm", "members": ["Pistol", "Musket"]}
                            }
                        }
                    }
                ],
            }
        ]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 0, out)
        self.assertIn("removed grants: 0", out)

    def test_g1_an_ungated_conditional_proficiency_gates(self):
        """SD-36 Epic F1 re-check round 2, finding 2 (mutation g1): the baseline holds a
        `GatedFactGrant` (a conditional proficiency); the fresh side holds the SAME fact as a
        bare `FactGrant` -- the gate deleted, so the proficiency is now unconditional. Before the
        fix `grant_signature` unwrapped `GatedFactGrant` in BOTH directions, so this produced
        `removed grants: 0`, verdict=PASS."""
        base = base_rules()
        rec = base["ultimate_combat/class_feature/samurai_proficiencies.json"][0]
        rec["grants"] = [
            {
                "GatedFactGrant": {
                    "fact": {"Proficiency": {"WeaponGroup": "Samurai"}},
                    "when": {"Compare": {"lhs": {"Var": "v1"}, "op": "Eq", "rhs": {"Const": 0}}},
                }
            }
        ]
        fresh = base_rules()
        fresh["ultimate_combat/class_feature/samurai_proficiencies.json"][0]["grants"] = [
            {"FactGrant": {"Proficiency": {"WeaponGroup": "Samurai"}}}
        ]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("removed grants: 1", out)
        self.assertIn("samurai_proficiencies", out)

    def test_g2_a_gate_swapped_for_a_different_condition_gates(self):
        """Mutation g2: both sides carry a `GatedFactGrant` around the SAME fact, but the `when`
        changed to a different (here, always-true) condition -- still a removal of the baseline's
        own condition, not a wrap. Before the fix, gating was unwrapped entirely, so the two
        `when` values were never compared."""
        base = base_rules()
        rec = base["ultimate_combat/class_feature/samurai_proficiencies.json"][0]
        rec["grants"] = [
            {
                "GatedFactGrant": {
                    "fact": {"Proficiency": {"WeaponGroup": "Samurai"}},
                    "when": {"Compare": {"lhs": {"Var": "v1"}, "op": "Eq", "rhs": {"Const": 0}}},
                }
            }
        ]
        fresh = base_rules()
        fresh["ultimate_combat/class_feature/samurai_proficiencies.json"][0]["grants"] = [
            {"GatedFactGrant": {"fact": {"Proficiency": {"WeaponGroup": "Samurai"}}, "when": {"Const": True}}}
        ]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("removed grants: 1", out)

    def test_g3_a_weapon_sets_members_replaced_under_the_same_label_gates(self):
        """Mutation g3: both sides carry a `WeaponSet` under the identical folded label, but the
        fresh side's member list is a totally different set of weapons -- content-destroying
        fabrication `grant_signature` alone (label-only) could not see, since it discarded
        `members` entirely."""
        base = base_rules()
        rec = base["ultimate_combat/class_feature/samurai_proficiencies.json"][0]
        rec["grants"] = [
            {"FactGrant": {"Proficiency": {"WeaponSet": {"label": "Samurai", "members": ["Katana", "Naginata", "Wakizashi"]}}}}
        ]
        fresh = base_rules()
        fresh["ultimate_combat/class_feature/samurai_proficiencies.json"][0]["grants"] = [
            {"FactGrant": {"Proficiency": {"WeaponSet": {"label": "Samurai", "members": ["Holy Avenger", "Vorpal Sword of Doom"]}}}}
        ]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("removed grants: 1", out)

    def test_g4_a_weapon_sets_members_emptied_under_the_same_label_gates(self):
        """Mutation g4: same shape as g3, but the fresh side's member list is wiped to empty --
        the emptied-set sibling of a replaced one, same blind spot."""
        base = base_rules()
        rec = base["ultimate_combat/class_feature/samurai_proficiencies.json"][0]
        rec["grants"] = [
            {"FactGrant": {"Proficiency": {"WeaponSet": {"label": "Samurai", "members": ["Katana", "Naginata", "Wakizashi"]}}}}
        ]
        fresh = base_rules()
        fresh["ultimate_combat/class_feature/samurai_proficiencies.json"][0]["grants"] = [
            {"FactGrant": {"Proficiency": {"WeaponSet": {"label": "Samurai", "members": []}}}}
        ]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("removed grants: 1", out)

    def test_h1_the_picaroon_shape_an_exact_duplicate_collapse_does_not_gate(self):
        """SD-36 Epic F1 re-check round 3, finding 1: the real
        `advanced_class_guide:class_feature:picaroon_weapon_proficiency` shape -- the baseline
        holds THREE bare `WeaponGroup` grants of one folded signature (two byte-identical
        `OnehandedFirearm`, one a case variant `OneHandedFirearm`), and F1-3's
        `dedup_weapon_set_grants` folds them, on the fresh side, to the ONE `WeaponSet` grant
        that names them all. Before the fix, `missing_grant_signatures`'s greedy one-fresh-per-
        one-baseline match let the single fresh grant cover only the FIRST baseline grant,
        reporting the other two as `removed grants: 2` -- a real dedup miscounted as a loss."""
        base = base_rules()
        base["advanced_class_guide/class_feature/picaroon_weapon_proficiency.json"] = [
            {
                "id": "advanced_class_guide:class_feature:picaroon_weapon_proficiency",
                "label": "Picaroon Weapon Proficiency",
                "value": "Text",
                "granted_by": [],
                "grants": [
                    {"FactGrant": {"Proficiency": {"WeaponGroup": "OnehandedFirearm"}}},
                    {"FactGrant": {"Proficiency": {"WeaponGroup": "OnehandedFirearm"}}},
                    {"FactGrant": {"Proficiency": {"WeaponGroup": "OneHandedFirearm"}}},
                ],
            }
        ]
        fresh = base_rules()
        fresh["advanced_class_guide/class_feature/picaroon_weapon_proficiency.json"] = [
            {
                "id": "advanced_class_guide:class_feature:picaroon_weapon_proficiency",
                "label": "Picaroon Weapon Proficiency",
                "value": "Text",
                "granted_by": [],
                "grants": [
                    {
                        "FactGrant": {
                            "Proficiency": {
                                "WeaponSet": {"label": "OnehandedFirearm", "members": ["Pistol", "Musket", "Blunderbuss"]}
                            }
                        }
                    }
                ],
            }
        ]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 0, out)
        self.assertIn("removed grants: 0", out)

    def test_h2_a_reordered_identical_multiset_does_not_gate(self):
        """SD-36 Epic F1 re-check round 3, finding 1 (latent hazard): the baseline holds a bare
        grant and a gated grant of the SAME signature; the fresh side holds the identical two
        grants, merely reordered. Before the fix, the greedy scan matched the bare baseline
        grant against the fresh GATED one first (a bare baseline grant covers any fresh gate),
        leaving no fresh grant left for the baseline's own gated grant -- a spurious
        `removed grants: 1` for an unchanged multiset. The maximum bipartite match finds the
        exact pairing regardless of order."""
        base = base_rules()
        rec = base["ultimate_combat/class_feature/samurai_proficiencies.json"][0]
        rec["grants"] = [
            {"FactGrant": {"Proficiency": {"WeaponGroup": "Samurai"}}},
            {
                "GatedFactGrant": {
                    "fact": {"Proficiency": {"WeaponGroup": "Samurai"}},
                    "when": {"Compare": {"lhs": {"Var": "v1"}, "op": "Eq", "rhs": {"Const": 0}}},
                }
            },
        ]
        fresh = base_rules()
        fresh["ultimate_combat/class_feature/samurai_proficiencies.json"][0]["grants"] = [
            {
                "GatedFactGrant": {
                    "fact": {"Proficiency": {"WeaponGroup": "Samurai"}},
                    "when": {"Compare": {"lhs": {"Var": "v1"}, "op": "Eq", "rhs": {"Const": 0}}},
                }
            },
            {"FactGrant": {"Proficiency": {"WeaponGroup": "Samurai"}}},
        ]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 0, out)
        self.assertIn("removed grants: 0", out)

    def test_h3_a_genuine_drop_does_not_hide_behind_an_untouched_duplicate_sibling(self):
        """The duplicate-collapse in h1 must not weaken round 1's own contract: the baseline
        holds TWO grants of one signature that are NOT duplicates of each other (a bare grant
        and a genuinely gated one) -- `_grants_are_duplicates` requires the same gate, so these
        never collapse. The fresh side drops the gated one and keeps only the bare one; that
        drop must still gate, not hide behind the untouched bare sibling."""
        base = base_rules()
        rec = base["ultimate_combat/class_feature/samurai_proficiencies.json"][0]
        rec["grants"] = [
            {"FactGrant": {"Proficiency": {"WeaponGroup": "Samurai"}}},
            {
                "GatedFactGrant": {
                    "fact": {"Proficiency": {"WeaponGroup": "Samurai"}},
                    "when": {"Compare": {"lhs": {"Var": "v1"}, "op": "Eq", "rhs": {"Const": 0}}},
                }
            },
        ]
        fresh = base_rules()
        fresh["ultimate_combat/class_feature/samurai_proficiencies.json"][0]["grants"] = [
            {"FactGrant": {"Proficiency": {"WeaponGroup": "Samurai"}}},
        ]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("removed grants: 1", out)

    def test_a_weapon_sets_members_growing_under_the_same_label_does_not_gate(self):
        """The non-mutation sibling of g3/g4: the baseline's member list is a SUBSET of the
        fresh one (a real oracle-driven set growth, or F1-3's bare-tag expansion where the
        baseline carries no member list at all) -- never a removal, must not gate."""
        base = base_rules()
        rec = base["ultimate_combat/class_feature/samurai_proficiencies.json"][0]
        rec["grants"] = [
            {"FactGrant": {"Proficiency": {"WeaponSet": {"label": "Samurai", "members": ["Katana", "Naginata"]}}}}
        ]
        fresh = base_rules()
        fresh["ultimate_combat/class_feature/samurai_proficiencies.json"][0]["grants"] = [
            {"FactGrant": {"Proficiency": {"WeaponSet": {"label": "Samurai", "members": ["Katana", "Naginata", "Wakizashi"]}}}}
        ]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 0, out)
        self.assertIn("removed grants: 0", out)

    def test_added_edge_and_grant_are_allowed_and_do_not_gate(self):
        base = base_rules()
        fresh = base_rules()
        rec = fresh["ultimate_combat/class_feature/samurai_proficiencies.json"][0]
        rec["granted_by"] = list(rec["granted_by"]) + [{"by": {"Rule": "ultimate_combat:ability:other_source"}, "when": None}]
        rec["grants"] = list(rec["grants"]) + [{"FactGrant": {"Proficiency": {"WeaponGroup": "Extra"}}}]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 0, out)
        self.assertIn("verdict=PASS", out)

    def test_a_provenance_delta_on_the_pinned_current_class_fix_record_does_not_gate(self):
        """SD-36 Epic F1 re-check round 3, finding 1 (ORCHESTRATOR RULING): a `provenance` delta
        on a record from `structural_diff_expected_provenance_deltas.json`'s pinned list -- the
        `current_class` closure fix's `closure_rows` growth -- is an expected, named, counted
        delta class, not a field-delta failure. `ultimate_psionics:class:psion` is on that
        list."""
        rid = "ultimate_psionics:class:psion"
        self.assertIn(rid, structural_diff.EXPECTED_PROVENANCE_DELTA_RECORDS, "fixture assumption: this id must be on the pinned list")
        base = base_rules()
        base["ultimate_psionics/class/psion.json"] = [
            {"id": rid, "label": "Psion", "value": "Text", "granted_by": [], "grants": [], "provenance": {"closure_rows": ["a:1"]}}
        ]
        fresh = base_rules()
        fresh["ultimate_psionics/class/psion.json"] = [
            {"id": rid, "label": "Psion", "value": "Text", "granted_by": [], "grants": [], "provenance": {"closure_rows": ["a:1", "a:2"]}}
        ]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 0, out)
        self.assertIn("unexpected field deltas: 0", out)
        self.assertIn("provenance deltas on the pinned current_class-fix record list: 1", out)

    def test_a_provenance_delta_on_an_unpinned_record_still_gates(self):
        """The pinned list is an EXACT enumeration, never a blanket allowance for the field: the
        identical `provenance` mutation on a record NOT on the pinned list still fails."""
        rid = "core_rulebook:class_feature:arcane_bloodline_school_power"
        self.assertNotIn(rid, structural_diff.EXPECTED_PROVENANCE_DELTA_RECORDS, "fixture assumption: this id must NOT be on the pinned list")
        base = base_rules()
        base["core_rulebook/class_feature/arcane_bloodline_school_power.json"][0]["provenance"] = {"closure_rows": ["a:1"]}
        fresh = base_rules()
        fresh["core_rulebook/class_feature/arcane_bloodline_school_power.json"][0]["provenance"] = {"closure_rows": ["a:1", "a:2"]}
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("unexpected field deltas: 1", out)
        self.assertIn("arcane_bloodline_school_power: provenance", out)

    def test_a_pinned_records_oracle_pin_change_still_gates(self):
        """SD-36 Epic F1 polish backlog item 2: the pinned-record provenance allowance is
        `structural_diff_expected_provenance_deltas.json`'s own narrower contract -- it grows
        ONLY `closure_rows`, no other field. A pinned record whose `oracle_pin` also changed
        (alongside a legitimate `closure_rows` growth) must still gate, since the allowance's
        differing-subkeys set is then `{closure_rows, oracle_pin}`, not exactly `{closure_rows}`."""
        rid = "ultimate_psionics:class:psion"
        self.assertIn(rid, structural_diff.EXPECTED_PROVENANCE_DELTA_RECORDS, "fixture assumption: this id must be on the pinned list")
        base = base_rules()
        base["ultimate_psionics/class/psion.json"] = [
            {
                "id": rid,
                "label": "Psion",
                "value": "Text",
                "granted_by": [],
                "grants": [],
                "provenance": {"closure_rows": ["a:1"], "oracle_pin": "psion_v1.lst"},
            }
        ]
        fresh = base_rules()
        fresh["ultimate_psionics/class/psion.json"] = [
            {
                "id": rid,
                "label": "Psion",
                "value": "Text",
                "granted_by": [],
                "grants": [],
                "provenance": {"closure_rows": ["a:1", "a:2"], "oracle_pin": "psion_v2.lst"},
            }
        ]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("unexpected field deltas: 1", out)
        self.assertIn("psion: provenance", out)

    def test_a_pinned_records_closure_rows_shrinking_still_gates(self):
        """SD-36 Epic F1 polish backlog item 2: the allowance requires the baseline `closure_rows`
        to be a SUBSET of the fresh one (growth-only), never merely a different value. A pinned
        record whose `closure_rows` list SHRINKS -- a real row dropped, not a fold-fix addition --
        must still gate."""
        rid = "ultimate_psionics:class:psion"
        self.assertIn(rid, structural_diff.EXPECTED_PROVENANCE_DELTA_RECORDS, "fixture assumption: this id must be on the pinned list")
        base = base_rules()
        base["ultimate_psionics/class/psion.json"] = [
            {
                "id": rid,
                "label": "Psion",
                "value": "Text",
                "granted_by": [],
                "grants": [],
                "provenance": {"closure_rows": ["a:1", "a:2"]},
            }
        ]
        fresh = base_rules()
        fresh["ultimate_psionics/class/psion.json"] = [
            {"id": rid, "label": "Psion", "value": "Text", "granted_by": [], "grants": [], "provenance": {"closure_rows": ["a:1"]}}
        ]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("unexpected field deltas: 1", out)
        self.assertIn("psion: provenance", out)

    def test_cross_shape_baseline_pair_is_not_a_duplicate(self):
        """SD-36 Epic F1 polish backlog item 3: `grant_signature` folds `WeaponGroup`/
        `WeaponTag`/`WeaponAllOf`/`WeaponSet` of the same tag text to one `("prof_tag", ...)`
        value for GROUPING and `grant_covers` purposes -- but two BASELINE grants of DIFFERENT
        shapes naming the same tag (`WeaponGroup("Samurai")` and `WeaponTag("Samurai")`) are not
        the same fact restated twice and must not collapse via `_grants_are_duplicates`. Dropping
        one of them on the fresh side must still gate as a genuine loss, not hide as a dedup."""
        base = base_rules()
        rec = base["ultimate_combat/class_feature/samurai_proficiencies.json"][0]
        rec["grants"] = [
            {"FactGrant": {"Proficiency": {"WeaponGroup": "Samurai"}}},
            {"FactGrant": {"Proficiency": {"WeaponTag": "Samurai"}}},
        ]
        fresh = base_rules()
        fresh["ultimate_combat/class_feature/samurai_proficiencies.json"][0]["grants"] = [
            {"FactGrant": {"Proficiency": {"WeaponGroup": "Samurai"}}},
        ]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("removed grants: 1", out)

    def test_a_pinned_bonus_var_split_delta_does_not_gate(self):
        """SD-36 Epic F1 stage 6 (merge-readiness blocker 2): a (rule id, field) pair pinned in
        `structural_diff_bonus_var_split_record_deltas.json` -- the comma-split `BONUS:VAR` index
        fix resolving a previously-flattened field to its real multi-contribution `Var` -- is an
        expected, named, counted delta class, not a field-delta failure."""
        rid, field = "core_rulebook:race_trait:racial_sla_levitate", "applies"
        self.assertIn((rid, field), structural_diff.EXPECTED_BONUS_VAR_SPLIT_FIELD_DELTAS, "fixture assumption: this pair must be on the pinned list")
        base = base_rules()
        base["core_rulebook/race_trait/racial_sla_levitate.json"] = [
            {"id": rid, "label": "Levitate", "value": "Text", "applies": "Always", "granted_by": [], "grants": []}
        ]
        fresh = base_rules()
        fresh["core_rulebook/race_trait/racial_sla_levitate.json"] = [
            {
                "id": rid,
                "label": "Levitate",
                "value": "Text",
                "applies": {"Compare": {"lhs": {"Var": "vf3886f08a793ad87"}, "op": "Eq", "rhs": {"Const": 0}}},
                "granted_by": [],
                "grants": [],
            }
        ]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 0, out)
        self.assertIn("unexpected field deltas: 0", out)
        self.assertIn("bonus_var_index comma-split fix (stage5 §1a): 1", out)

    def test_an_unpinned_field_delta_on_the_same_record_still_gates(self):
        """The pinned list is an EXACT (rule id, field) enumeration, never a blanket allowance for
        the record: a DIFFERENT field changing on the same pinned record (`label`, which is not
        among `racial_sla_levitate`'s pinned `applies`/`also`/`value` triple) still fails."""
        rid = "core_rulebook:race_trait:racial_sla_levitate"
        self.assertNotIn((rid, "label"), structural_diff.EXPECTED_BONUS_VAR_SPLIT_FIELD_DELTAS, "fixture assumption: this pair must NOT be on the pinned list")
        base = base_rules()
        base["core_rulebook/race_trait/racial_sla_levitate.json"] = [
            {"id": rid, "label": "Levitate", "value": {"Number": {"Const": 1}}, "applies": "Always", "granted_by": [], "grants": []}
        ]
        fresh = base_rules()
        fresh["core_rulebook/race_trait/racial_sla_levitate.json"] = [
            {
                "id": rid,
                "label": "Levitate (SLA)",
                "value": {"Number": {"Const": 1}},
                "applies": "Always",
                "granted_by": [],
                "grants": [],
            }
        ]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("unexpected field deltas: 1", out)
        self.assertIn("racial_sla_levitate: label", out)

    def _f1c_split_fixture(self, sibling_applies):
        rid = "advanced_class_guide:class_feature:arcanist_archetype_elemental_master"
        rel = "advanced_class_guide/class_feature/arcanist_archetype_elemental_master.json"
        self.assertEqual(structural_diff.F1C_FIELD_DELTA_CLASS.get((rid, "value")), "d2_line_split", "fixture assumption: pinned D2 pair")
        self.assertEqual(structural_diff.F1C_D2_SPLITS.get(rid), rid + "#bonus0", "fixture assumption: pinned D2 sibling")
        gate = {"Compare": {"lhs": {"Var": "v1"}, "op": "Gte", "rhs": {"Const": 1}}}
        base = base_rules()
        base[rel] = [{"id": rid, "label": "Elemental Master", "value": {"Number": {"Const": 1}}, "target": {"Pool": "elemental_master_type"}, "applies": gate, "granted_by": [], "grants": []}]
        fresh = base_rules()
        fresh[rel] = [
            {"id": rid, "label": "Elemental Master", "value": "Text", "applies": gate, "granted_by": [], "grants": []},
            {"id": rid + "#bonus0", "label": "Elemental Master (picks)", "value": {"Number": {"Const": 1}}, "target": {"Pool": "elemental_master_type"}, "applies": sibling_applies, "granted_by": [], "grants": []},
        ]
        return base, fresh

    def test_f1c_a_pinned_d2_line_split_does_not_gate(self):
        """SD-36 Epic F1c (D2): a pinned principal whose line moved, condition intact, to its pinned
        `#<suffix>` sibling is the named d2_line_split class, not a field-delta failure."""
        gate = {"Compare": {"lhs": {"Var": "v1"}, "op": "Gte", "rhs": {"Const": 1}}}
        base, fresh = self._f1c_split_fixture(gate)
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 0, out)
        self.assertIn("unexpected field deltas: 0", out)
        self.assertIn("F1c d2_line_split: 2 field deltas on 1 records, 1 added rule ids", out)

    def test_f1c_a_d2_split_that_drops_the_line_condition_gates(self):
        """The pin is not a blanket allowance: the same pinned pairs gate when the sibling does not
        carry the old line's condition (the moved line lost its gate -- the defect D2 must never
        introduce)."""
        base, fresh = self._f1c_split_fixture("Always")
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("unexpected field deltas: 2", out)

    def test_f1c_a_pinned_closure_complete_false_gates(self):
        """A pinned d4_closure_complete pair passes only as absent -> true."""
        rid, rel = "advanced_class_guide:class:ex_warpriest", "advanced_class_guide/class/ex_warpriest.json"
        self.assertEqual(structural_diff.F1C_FIELD_DELTA_CLASS.get((rid, "closure_complete")), "d4_closure_complete")
        base = base_rules()
        base[rel] = [{"id": rid, "label": "Ex Warpriest", "value": "Text", "granted_by": [], "grants": []}]
        fresh = base_rules()
        fresh[rel] = [{"id": rid, "label": "Ex Warpriest", "value": "Text", "closure_complete": True, "granted_by": [], "grants": []}]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 0, out)
        fresh[rel][0]["closure_complete"] = False
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("ex_warpriest: closure_complete", out)

    def test_f1c4_a_pinned_always_held_passes_only_absent_to_true(self):
        """A pinned d7_always_held pair passes only as absent -> true on the principal."""
        rid, rel = "advanced_players_guide:class_feature:default", "advanced_players_guide/class_feature/default.json"
        self.assertEqual(structural_diff.F1C_FIELD_DELTA_CLASS.get((rid, "always_held")), "d7_always_held")
        base = base_rules()
        base[rel] = [{"id": rid, "label": "Default", "value": "Text", "granted_by": [], "grants": []}]
        fresh = base_rules()
        fresh[rel] = [{"id": rid, "label": "Default", "value": "Text", "always_held": True, "granted_by": [], "grants": []}]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 0, out)
        fresh[rel][0]["always_held"] = False
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("default: always_held", out)

    def test_f1c5_a_pinned_pool_pick_passes_only_its_shape(self):
        """A pinned d8_pool_pick pair passes only as absent -> the record's own pick counted by one
        variable; a planted Const count, a foreign choice id, or the same offer on an unpinned
        record FAILS."""
        rid, rel = "advanced_players_guide:class_feature:summoner", "advanced_players_guide/class_feature/summoner.json"
        self.assertEqual(structural_diff.F1C_FIELD_DELTA_CLASS.get((rid, "offers")), "d8_pool_pick")
        pick = {"id": rid, "count": {"Var": "v5064279ed9539fc5"},
                "from": {"Rules": {"pool": "class", "tags": ["Summoner Class Selection"], "requires": "Always"}}}
        base = base_rules()
        base[rel] = [{"id": rid, "label": "Summoner", "value": "Text", "granted_by": [], "grants": []}]
        fresh = base_rules()
        fresh[rel] = [{"id": rid, "label": "Summoner", "value": "Text", "offers": pick, "granted_by": [], "grants": []}]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 0, out)
        for planted in ({**pick, "count": {"Const": 1}}, {**pick, "id": "advanced_players_guide:class_feature:other"}):
            fresh[rel][0]["offers"] = planted
            code, out = self.run_diff(base, fresh)
            self.assertEqual(code, 1, out)
            self.assertIn("summoner: offers", out)
        other_rid, other_rel = "core_rulebook:class_feature:unpinned_fixture", "core_rulebook/class_feature/unpinned_fixture.json"
        base = base_rules()
        base[other_rel] = [{"id": other_rid, "label": "X", "value": "Text", "granted_by": [], "grants": []}]
        fresh = base_rules()
        fresh[other_rel] = [{"id": other_rid, "label": "X", "value": "Text", "offers": {**pick, "id": other_rid}, "granted_by": [], "grants": []}]
        code, out = self.run_diff(base, fresh)
        self.assertEqual(code, 1, out)
        self.assertIn("unpinned_fixture: offers", out)

    def test_report_only_flag_keeps_exit_zero(self):
        base = base_rules()
        fresh = base_rules()
        fresh["ultimate_combat/class_feature/samurai_proficiencies.json"][0]["granted_by"] = []
        with tempfile.TemporaryDirectory() as baseline_dir, tempfile.TemporaryDirectory() as scratch_dir:
            write_tree(baseline_dir, base, BASE_REPORT)
            write_tree(scratch_dir, fresh, BASE_REPORT)
            argv = sys.argv
            sys.argv = ["structural_diff.py", scratch_dir, "--baseline", baseline_dir, "--report-only"]
            buf = io.StringIO()
            try:
                with redirect_stdout(buf):
                    code = structural_diff.main()
            finally:
                sys.argv = argv
            self.assertEqual(code, 0, buf.getvalue())
            self.assertIn("verdict=FAIL", buf.getvalue())


if __name__ == "__main__":
    unittest.main()
