#!/usr/bin/env python3
"""Tests for `scripts/derive_derived_evaluator_fixtures.py`'s self-erasure fix.

WHY THIS FILE EXISTS (SD-31 wave 29 lane 1). `THE-BOX.md` wave 28 S3 item #1 /
`todo/defects.md` D7: run from the committed state, this generator destroyed
**2,109 fixture entries across 8 families** on its very first run -- not on a
second run, which is why the "twice-run-diff" test shape THE-BOX explicitly
names and rejects cannot catch it (the destruction happens going from the
COMMITTED state to run 1; run 1's output and run 2's output are then already
identical to each other, so a diff BETWEEN two runs is clean even though both
are wrong relative to what was committed).

Two independent causes, both fixed in `scripts/derive_derived_evaluator_
fixtures.py`:

1. `HELD_STATUSES` omitted `fixture-verified` (the done-rung stamp this exact
   fixture's own consumer, `tests/derived_evaluator_fixture_check.rs`, writes
   back onto a unit it verified) AND `literal-verified` (the sibling done-rung
   stamp `corpus_literal_sweep` writes) -- so a unit this generator had
   already proven, on ANY prior run, became permanently invisible to every
   run after that.
2. The write step rebuilt the whole fixture document from scratch every run
   and only ever preserved keys matching a hardcoded `"monster_"` prefix --
   silently dropping every OTHER sibling family (`spell_entries`,
   `companion_entries`, `class_feature_description_entries`, and more) that a
   DIFFERENT generator owns and had already written into the same file.

THE CHECK THIS FILE BUILDS. Not a twice-run-diff. A run-ONCE-against-the-
committed-baseline check: for every top-level `*_entries`-shaped family
present in the fixture BEFORE a run, assert `len(after) >= len(before)`.
`shrunk_families`/`family_entry_counts` (now importable from the generator
module itself, and called unconditionally by `main()` before every write) ARE
that check -- this file does not reimplement it, it exercises the real one.

PROVING THE CHECK CAN FAIL (the anti-gaming bar: a gate that cannot fail is
worse than no gate). `SandboxReproductionTests` below does not simulate the
historical bug's shape by hand -- it checks out the ACTUAL pre-fix script
text from the pinned buggy commit (`git show <sha>:scripts/derive_derived_
evaluator_fixtures.py`) and runs THAT code, unmodified, in an isolated
sandbox against the real, live `docs/work-inventory.json` and a private copy
of the real committed fixture (never the checked-out file itself). `Shrunk
FamiliesUnitTests` additionally proves `shrunk_families` itself is not
vacuous using the same real committed fixture as ground truth.
"""

import json
import os
import shutil
import subprocess
import sys
import tempfile
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
SCRIPT = os.path.join(REPO_ROOT, "scripts", "derive_derived_evaluator_fixtures.py")
FIXTURE = os.path.join(
    REPO_ROOT, "tests", "fixtures", "rules_core", "derived-evaluator-fixtures.json"
)
INVENTORY = os.path.join(REPO_ROOT, "docs", "work-inventory.json")

# The commit this generator's self-erasure bug was live at, measured by wave
# 28 and fixed by wave 29 lane 1 (`THE-BOX.md` S3 item #1, `todo/defects.md`
# D7). Pinned by SHA rather than `HEAD` so `SandboxReproductionTests` keeps
# reproducing the historical defect's exact shape even after this fix lands
# and HEAD moves past it.
BUGGY_COMMIT = "283850c13"

sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import derive_derived_evaluator_fixtures as gen  # noqa: E402


def _pcgen_root():
    root = os.environ.get("PCGEN_CORPUS_ROOT")
    if root:
        return root
    home = os.environ.get("HOME", "")
    return os.path.join(home, "workspace", "repos", "pcgen", "data")


def _oracle_available():
    return os.path.isdir(
        os.path.join(_pcgen_root(), "pathfinder", "paizo", "roleplaying_game")
    )


def _run_generator_in_sandbox(script_text):
    """Runs `script_text` (a full copy of the generator, buggy or fixed) in an
    isolated directory tree that mirrors just enough of the real repo layout
    for the script's own `os.path.dirname(__file__)/..`-computed `REPO`
    constant to resolve correctly -- WITHOUT ever touching the real committed
    fixture. `docs/work-inventory.json` is the real, live, on-disk inventory
    (read-only, symlinked in); the fixture is a private copy seeded from the
    real committed file. Returns `(returncode, stdout, stderr,
    resulting_fixture_dict_or_None)`.
    """
    with tempfile.TemporaryDirectory() as sandbox:
        os.makedirs(os.path.join(sandbox, "scripts"))
        os.makedirs(os.path.join(sandbox, "docs"))
        os.makedirs(os.path.join(sandbox, "tests", "fixtures", "rules_core"))
        script_path = os.path.join(
            sandbox, "scripts", "derive_derived_evaluator_fixtures.py"
        )
        with open(script_path, "w", encoding="utf-8") as fh:
            fh.write(script_text)
        os.symlink(INVENTORY, os.path.join(sandbox, "docs", "work-inventory.json"))
        sandbox_fixture = os.path.join(
            sandbox, "tests", "fixtures", "rules_core", "derived-evaluator-fixtures.json"
        )
        shutil.copyfile(FIXTURE, sandbox_fixture)
        env = dict(os.environ)
        env["PCGEN_CORPUS_ROOT"] = _pcgen_root()
        out = subprocess.run(
            [sys.executable, script_path],
            capture_output=True,
            text=True,
            cwd=sandbox,
            env=env,
        )
        result = None
        if os.path.exists(sandbox_fixture):
            with open(sandbox_fixture, encoding="utf-8") as fh:
                result = json.load(fh)
        return out.returncode, out.stdout, out.stderr, result


class ShrunkFamiliesUnitTests(unittest.TestCase):
    """Direct tests of `shrunk_families`/`family_entry_counts` -- the
    invariant `main()` now enforces, unconditionally, before every write."""

    def setUp(self):
        with open(FIXTURE, encoding="utf-8") as fh:
            self.committed = json.load(fh)

    def test_a_document_compared_to_itself_has_no_violations(self):
        self.assertEqual(gen.shrunk_families(self.committed, self.committed), {})

    def test_growth_is_never_a_violation(self):
        grown = dict(self.committed)
        grown["entries"] = list(self.committed["entries"]) + [
            {"unit_id": "synthetic:test:only", "expected": {}}
        ]
        self.assertEqual(gen.shrunk_families(self.committed, grown), {})

    def test_flags_the_real_historical_defect_shape(self):
        """Reproduces the EXACT output shape the pre-fix generator produced
        against the real committed fixture (measured directly, wave 29 lane
        1): `entries` rebuilt to empty, every sibling family EXCEPT the
        `monster_`-prefixed ones silently dropped. `shrunk_families` must
        flag every one of them, with the historical ~2,110-row magnitude."""
        buggy_after = {"entries": []}
        buggy_after.update(
            {k: v for k, v in self.committed.items() if k.startswith("monster_")}
        )
        violations = gen.shrunk_families(self.committed, buggy_after)

        expected_families = {
            "entries", "spell_entries", "spell_range_entries",
            "class_feature_entries", "class_feature_description_entries",
            "companion_entries", "companion_skill_entries",
            "companion_save_dc_entries",
        }
        self.assertEqual(set(violations), expected_families, violations)

        total_lost = sum(before - after for before, after in violations.values())
        self.assertGreaterEqual(
            total_lost, 2000,
            f"expected the historical ~2,110-row loss, got {total_lost}: {violations}",
        )
        # monster_* families, deliberately preserved even by the buggy
        # write-path (the allowlist's one working case), must NOT be flagged.
        self.assertNotIn("monster_entries", violations)
        self.assertNotIn("monster_sla_entries", violations)
        self.assertNotIn("monster_ability_entries", violations)
        self.assertNotIn("monster_ability_formula_entries", violations)


@unittest.skipUnless(
    _oracle_available(),
    "PCGen oracle checkout not found at $PCGEN_CORPUS_ROOT "
    "(default $HOME/workspace/repos/pcgen/data); see scripts/fetch-pcgen-oracle.sh",
)
class SandboxReproductionTests(unittest.TestCase):
    """Runs the REAL generator code -- both the historical buggy commit and
    today's fixed script -- end to end in an isolated sandbox, against the
    real live inventory and a private copy of the real committed fixture.
    Proves the check can fail (on the real pre-fix code) and that today's
    code does not trigger it."""

    def test_the_buggy_commit_reproducibly_self_erases_and_the_check_catches_it(self):
        buggy_text = subprocess.run(
            ["git", "show", f"{BUGGY_COMMIT}:scripts/derive_derived_evaluator_fixtures.py"],
            capture_output=True, text=True, cwd=REPO_ROOT, check=True,
        ).stdout
        self.assertIn(
            'HELD_STATUSES = ("ingested-magnitude", "grounded", "text-complete")',
            buggy_text,
            "the pinned commit no longer carries the exact buggy line this test "
            "means to reproduce -- BUGGY_COMMIT needs re-pinning",
        )

        rc, out, err, after = _run_generator_in_sandbox(buggy_text)
        self.assertEqual(rc, 0, f"buggy script itself should exit 0 -- that IS the danger:\n{err}")
        self.assertIsNotNone(after, f"buggy script wrote nothing:\n{out}\n{err}")

        with open(FIXTURE, encoding="utf-8") as fh:
            before = json.load(fh)
        violations = gen.shrunk_families(before, after)

        self.assertGreaterEqual(
            len(violations), 8,
            f"expected the historical 8-family shrink from the real buggy "
            f"commit, got {len(violations)}: {violations}",
        )
        total_lost = sum(b - a for b, a in violations.values())
        self.assertGreaterEqual(total_lost, 2000, violations)

    def test_the_fixed_generator_never_shrinks_any_family(self):
        with open(SCRIPT, encoding="utf-8") as fh:
            fixed_text = fh.read()

        rc, out, err, after = _run_generator_in_sandbox(fixed_text)
        self.assertEqual(rc, 0, f"fixed script should exit 0:\n{out}\n{err}")
        self.assertIsNotNone(after, f"fixed script wrote nothing:\n{out}\n{err}")

        with open(FIXTURE, encoding="utf-8") as fh:
            before = json.load(fh)
        violations = gen.shrunk_families(before, after)
        self.assertEqual(violations, {}, f"stdout:\n{out}\nviolations: {violations}")

        # And: the real committed file on disk must be untouched by any of
        # this -- every mutation above happened inside the sandbox only.
        with open(FIXTURE, encoding="utf-8") as fh:
            after_test = json.load(fh)
        self.assertEqual(before, after_test, "this test must never mutate the real committed fixture")


class HeldStatusesRegressionTests(unittest.TestCase):
    """SD31-W29-INTEGRATE (adversarial-review CONFIRMED finding): the
    `shrunk_families` gate is completely BLIND to a `HELD_STATUSES`
    regression, because the carry-forward merge in `main()` silently
    backfills a fully-empty fresh-derivation with the previously-committed
    rows -- `entries` never shrinks even though the generator has stopped
    doing its job. Reverting `HELD_STATUSES` to its pre-fix 3-status tuple
    (with the rest of today's fix -- preserve-by-exclusion, carry-forward
    merge -- intact) leaves every OTHER test in this file green and the
    written fixture byte-identical to committed. This class proves the
    dedicated guards added for exactly that gap actually catch it: a
    module-level `assert` fires before `main()` even runs."""

    def test_reverting_fixture_verified_out_of_held_statuses_is_caught(self):
        with open(SCRIPT, encoding="utf-8") as fh:
            fixed_text = fh.read()
        needle = (
            'HELD_STATUSES = (\n'
            '    "ingested-magnitude", "grounded", "text-complete", "fixture-verified",\n'
            ')'
        )
        self.assertIn(
            needle, fixed_text,
            "generator's HELD_STATUSES tuple text has moved -- update this "
            "mutation test's needle to match",
        )
        mutated_text = fixed_text.replace(
            needle,
            'HELD_STATUSES = (\n    "ingested-magnitude", "grounded", "text-complete",\n)',
        )
        self.assertNotEqual(mutated_text, fixed_text)

        with open(FIXTURE, encoding="utf-8") as fh:
            before = json.load(fh)

        rc, out, err, after = _run_generator_in_sandbox(mutated_text)
        self.assertNotEqual(
            rc, 0,
            f"a HELD_STATUSES regression must be refused, not silently "
            f"written:\nstdout:\n{out}",
        )
        self.assertIn(
            "fixture-verified must stay in HELD_STATUSES", err,
            f"expected the module-level assert to fire; got:\nstdout:\n{out}\nstderr:\n{err}",
        )
        # The sandbox fixture was seeded from the real committed one before
        # the script ran and errored out before writing -- it must still
        # equal exactly what was seeded, proving nothing was written.
        self.assertEqual(after, before)

    def test_a_generator_that_somehow_bypassed_the_module_assert_is_still_caught_in_main(self):
        """Belt-and-suspenders: even if a future edit removed the
        module-level `assert` but the underlying live selection still
        produced zero fresh `entries`, `main()`'s own
        `existing_by_id and not fresh_by_id` guard must independently
        refuse the write. Proven by neutralising ONLY the module-level
        assert (turning it into a no-op) while keeping the 3-status tuple,
        so `held_derived` still resolves to nothing derivable."""
        with open(SCRIPT, encoding="utf-8") as fh:
            fixed_text = fh.read()
        needle = (
            'HELD_STATUSES = (\n'
            '    "ingested-magnitude", "grounded", "text-complete", "fixture-verified",\n'
            ')\n'
            'assert "fixture-verified" in HELD_STATUSES, (\n'
            '    "fixture-verified must stay in HELD_STATUSES -- its absence is silently "\n'
            '    "masked by the carry-forward merge in main() and caught by nothing else "\n'
            '    "(todo/defects.md D7, todo/sweeps.md S6)"\n'
            ')'
        )
        self.assertIn(needle, fixed_text, "text has moved -- update this test's needle")
        mutated_text = fixed_text.replace(
            needle,
            'HELD_STATUSES = (\n    "ingested-magnitude", "grounded", "text-complete",\n)',
        )
        self.assertNotEqual(mutated_text, fixed_text)

        with open(FIXTURE, encoding="utf-8") as fh:
            before = json.load(fh)

        rc, out, err, after = _run_generator_in_sandbox(mutated_text)
        self.assertNotEqual(rc, 0, f"main()'s own guard must also refuse:\nstdout:\n{out}")
        self.assertIn(
            "FATAL: this run derived ZERO fresh", out + err, f"stdout:\n{out}\nstderr:\n{err}"
        )
        self.assertEqual(after, before)


class OwnDocumentFieldsDesyncTests(unittest.TestCase):
    """SD31-W29-INTEGRATE (adversarial-review CONFIRMED finding, MEDIUM):
    the write step used to hand-maintain a separate `OWN_KEYS` set that
    had to be kept in sync BY EYE with the literal keys in the
    `document = {...}` dict -- proven able to desync by mutation (removing
    one key from `OWN_KEYS` left a stale `preserved` value silently
    overriding a freshly-derived one, with zero test catching it).
    `own_document_fields()` is now the SINGLE source both the exclusion
    set and the write step build from, making that desync structurally
    impossible. These tests prove the invariant holds, not merely that it
    was intended to."""

    def test_own_document_fields_keys_are_stable_regardless_of_entries_content(self):
        empty = set(gen.own_document_fields([]))
        nonempty = set(gen.own_document_fields([{"unit_id": "x"}]))
        self.assertEqual(
            empty, nonempty,
            "the KEY SET must not depend on the entries payload -- OWN_KEYS is "
            "derived by calling this with an empty list specifically so it never "
            "needs a real `entries` value to compute the exclusion set",
        )
        self.assertIn("entries", empty)
        self.assertIn("schema", empty)

    def test_a_stale_preserved_value_for_an_own_key_can_never_win(self):
        """The real-world manifestation of the desync this class guards
        against: a pre-existing fixture on disk carries a STALE value for
        one of this generator's own keys (e.g. `derivation` text from a
        previous version of the docstring). Because `own_document_fields()`
        is now the only source of truth for what counts as \"own\", that
        stale value can never leak through `preserved` and override the
        freshly-computed one -- proven by seeding the sandbox with a
        deliberately wrong `derivation` string and confirming the written
        output carries today's real one, not the stale one."""
        with open(SCRIPT, encoding="utf-8") as fh:
            fixed_text = fh.read()
        with open(FIXTURE, encoding="utf-8") as fh:
            real_committed = json.load(fh)

        with tempfile.TemporaryDirectory() as sandbox:
            os.makedirs(os.path.join(sandbox, "scripts"))
            os.makedirs(os.path.join(sandbox, "docs"))
            os.makedirs(os.path.join(sandbox, "tests", "fixtures", "rules_core"))
            script_path = os.path.join(sandbox, "scripts", "derive_derived_evaluator_fixtures.py")
            with open(script_path, "w", encoding="utf-8") as fh:
                fh.write(fixed_text)
            os.symlink(INVENTORY, os.path.join(sandbox, "docs", "work-inventory.json"))
            sandbox_fixture = os.path.join(
                sandbox, "tests", "fixtures", "rules_core", "derived-evaluator-fixtures.json"
            )
            # Seed with the real committed document, but with a deliberately
            # STALE value for one of this generator's own keys.
            stale = dict(real_committed)
            stale["derivation"] = "STALE VALUE FROM A PRIOR VERSION -- must not survive a run"
            with open(sandbox_fixture, "w", encoding="utf-8") as fh:
                json.dump(stale, fh)

            env = dict(os.environ)
            env["PCGEN_CORPUS_ROOT"] = _pcgen_root()
            result = subprocess.run(
                [sys.executable, script_path], capture_output=True, text=True,
                cwd=sandbox, env=env,
            )
            self.assertEqual(result.returncode, 0, result.stderr)
            with open(sandbox_fixture, encoding="utf-8") as fh:
                written = json.load(fh)

        self.assertNotEqual(
            written["derivation"], stale["derivation"],
            "a stale value for an OWN key leaked through `preserved` and "
            "overrode the freshly-computed one -- the desync this class "
            "exists to prevent is back",
        )
        self.assertEqual(written["derivation"], gen.own_document_fields([])["derivation"])


if __name__ == "__main__":
    unittest.main()
