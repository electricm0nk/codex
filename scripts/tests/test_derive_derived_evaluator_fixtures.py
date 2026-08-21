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


if __name__ == "__main__":
    unittest.main()
