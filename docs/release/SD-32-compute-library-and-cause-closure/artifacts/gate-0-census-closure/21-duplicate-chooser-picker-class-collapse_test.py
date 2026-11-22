"""SD-32 Decision 21 binding condition 3: "A test proves the predicate
cannot over-reach -- specifically that a group whose members grant
*different* targets is left alone. Prove it goes red by loosening the
predicate to adjacency (the rule Decision 17 rejected), then revert."

Run:
    python3 docs/release/SD-32-compute-library-and-cause-closure/artifacts/gate-0-census-closure/21-duplicate-chooser-picker-class-collapse_test.py
"""
from __future__ import annotations

import os
import sys
import unittest

import importlib.util

# `21-...-collapse.py` is not a valid Python identifier module name (starts
# with a digit, has hyphens) -- load it explicitly by path.

_SPEC = importlib.util.spec_from_file_location(
    "card15_decision21_collapse",
    os.path.join(os.path.dirname(os.path.abspath(__file__)), "21-duplicate-chooser-picker-class-collapse.py"),
)
collapse = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(collapse)


def row(fn: str, line: int, type_facet: str, targets: list[str], category: str = "Some Category") -> tuple:
    """Builds one synthetic `(book, fn, line, fields)` row tuple in the same
    shape `collect_fallback_groups` produces, with a fabricated `TYPE:` and
    however many `ABILITY:...|AUTOMATIC|...` fields are needed to produce
    the given real grant `targets` (this helper never emits a `TYPE=`
    self-tag field -- that exclusion is proven separately by the real-corpus
    run, `21-...-collapse.py`'s own `main()`, against the ACG "Aberrant
    Bloodline" worked example)."""
    fields = ["SomeIdentity", f"CATEGORY:{category}", f"TYPE:{type_facet}"]
    for t in targets:
        fields.append(f"ABILITY:Class Feature|AUTOMATIC|{t}")
    return ("some_book", fn, line, fields)


class DecisionTwentyOnePredicateTests(unittest.TestCase):
    def test_real_worked_example_shape_is_covered(self) -> None:
        """The ACG "Aberrant Bloodline" shape (4 rows, 2 targets in pairs):
        covered."""
        rows = [
            row("f.lst", 1, "SorcererBloodlineChoice", ["Sorcerer Bloodline ~ X"]),
            row("f.lst", 2, "BloodragerBloodlineChoice", ["Bloodrager Bloodline ~ X"]),
            row("f.lst", 3, "SorcererBloodlineChoice", ["Sorcerer Bloodline ~ X"]),
            row("f.lst", 4, "BloodragerBloodlineChoice", ["Bloodrager Bloodline ~ X"]),
        ]
        self.assertTrue(collapse.is_duplicate_chooser_picker_group(rows))

    def test_five_row_single_target_shape_is_covered(self) -> None:
        """The 5-row ARG/monster_codex/occult_adventures shape: all rows
        converge on ONE target -- covered."""
        rows = [row("f.lst", i, "SorcererBloodlineChoice", ["Sorcerer Bloodline ~ X"]) for i in range(1, 6)]
        self.assertTrue(collapse.is_duplicate_chooser_picker_group(rows))

    def test_differing_targets_group_is_NOT_covered_left_alone(self) -> None:
        """THE OVER-REACH PROOF (binding condition 3). Two rows, both
        genuinely `TYPE:*Choice`-typed, same book/key collision -- but each
        grants a DIFFERENT target (no partner). The real Decision 21
        predicate must NOT flag this as a duplicate-chooser-picker group:
        it is exactly the shape a distinct-object pair with a shared
        display name would produce, and Decision 17's own text is why this
        program refuses to sweep it in on adjacency/type-shape alone."""
        rows = [
            row("f.lst", 1, "SorcererBloodlineChoice", ["Sorcerer Bloodline ~ Alpha"]),
            row("f.lst", 2, "SorcererBloodlineChoice", ["Sorcerer Bloodline ~ Beta"]),
        ]
        self.assertFalse(
            collapse.is_duplicate_chooser_picker_group(rows),
            "predicate over-reached: a group with two DIFFERENT grant targets "
            "was flagged as a duplicate-chooser-picker group",
        )

    def test_loosening_to_adjacency_ignoring_targets_WOULD_over_reach(self) -> None:
        """Demonstrates the exact failure Decision 17 rejected and Decision
        21 binding condition 3 requires proving against: an "adjacency"
        predicate that checks only `TYPE:*Choice`-for-all (same book/key
        collision, same TYPE family) and ignores grant targets entirely.
        Run against the SAME differing-targets fixture as the test above,
        this weaker predicate WRONGLY says "duplicate-chooser-picker group".

        This is the mutation-proof binding condition 3 asks for: replacing
        `is_duplicate_chooser_picker_group`'s real body with just
        `all_type_choice(rows)` (the "loosen to adjacency" mutation) makes
        `test_differing_targets_group_is_NOT_covered_left_alone` FAIL --
        confirmed by hand for this cycle's own receipt (run, observed RED,
        reverted; the loosened variant is never committed as production
        code) -- and this test asserts the loosened variant's own answer
        directly, as a permanent, standing proof that stays red if anyone
        reintroduces the adjacency shortcut in place of
        `targets_pairwise_coincide`.
        """
        rows = [
            row("f.lst", 1, "SorcererBloodlineChoice", ["Sorcerer Bloodline ~ Alpha"]),
            row("f.lst", 2, "SorcererBloodlineChoice", ["Sorcerer Bloodline ~ Beta"]),
        ]
        adjacency_only_verdict = collapse.all_type_choice(rows)  # the rejected heuristic
        real_predicate_verdict = collapse.is_duplicate_chooser_picker_group(rows)

        self.assertTrue(
            adjacency_only_verdict,
            "fixture setup error: expected the adjacency-only heuristic to "
            "misfire True on this differing-targets pair",
        )
        self.assertFalse(real_predicate_verdict)
        self.assertNotEqual(
            adjacency_only_verdict,
            real_predicate_verdict,
            "the real predicate must disagree with the adjacency-only "
            "heuristic on a differing-targets group -- if they ever agree "
            "here, the real predicate has regressed to the rejected rule",
        )

    def test_all_39_real_groups_covered_zero_exceptions(self) -> None:
        """Re-derives the real corpus groups (requires PCGEN_CORPUS_ROOT;
        skipped if not set) and asserts the exact 39/74 figures this cycle's
        receipt reports, with the TYPE= exclusion applied."""
        pcgen_root = os.environ.get("PCGEN_CORPUS_ROOT")
        if not pcgen_root:
            self.skipTest("PCGEN_CORPUS_ROOT not set")
        repo_root = collapse.REPO_ROOT
        inventory_path = os.path.join(repo_root, "docs", "work-inventory.json")
        log = collapse.build_collapse_log(pcgen_root, inventory_path)
        self.assertEqual(log["groups_covered"], 39)
        self.assertEqual(log["residual_rows_removed_from_ledger"], 74)
        self.assertEqual(log["groups_not_covered_left_alone"], 0)


if __name__ == "__main__":
    unittest.main()
