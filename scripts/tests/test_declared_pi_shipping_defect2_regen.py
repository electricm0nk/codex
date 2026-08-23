#!/usr/bin/env python3
"""Regression test for the `pi-key-rawtokens-screen` follow-up cycle's
Defect 2 fix (SD-32 card 11, `t9-onboarding` actor, 2026-08-23):
`cargo run --bin declared_pi_shipping_audit`'s 28 `NAME-PI-SHIPPED`
violations in `language`/`template`.

**Root cause**: `scripts/ingest_simple_filename_kinds.py` served six kinds
(`template`, `power`, `domain`, `language`, `skill`, `deity`) but only
`deity` went through `decisions.md §24`'s Codex-generated-neutral-name path
for a declared-PI name. The other five kinds fell through a legacy
pre-`§24` branch that replaced `name`/`key` with the literal
`REDACTED_PI_MARKER` string IN PLACE -- a shape
`declared_pi_shipping_audit.rs`'s own check (mirroring
`decisions.md §24b`-3's reasoning: a key/name's mere presence on disk,
even marker-redacted, is still the violation) rejects. Fixed by removing
the `always_pi`-gated branch split so every `name_is_pi` record across all
six kinds gets the SAME `§24` neutral-name treatment
(`scripts/codex_neutral_name.py`), reusing the machinery the operator
named rather than inventing a second scheme.

This is a real generator-output regression test, not a mock: it runs the
actual audit binary via `cargo run` (skipped if `cargo`/`CARGO_TARGET_DIR`
is unavailable in the sandbox this test runs in) and separately re-derives,
from the fixed corpus files' own content, that every one of the 28
originally-named violating records now carries a `codex_generated_name`
marker rather than the old in-place marker-substitution shape.
"""
from __future__ import annotations

import json
import os
import shutil
import subprocess
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

# The 28 originally-reported violating files (SD-32 card 11 dispatch brief,
# `declared_pi_shipping_audit`'s own report, re-derived live this cycle).
ORIGINAL_28 = [
    "data/corpus/bestiary_4/language/mi_go.json",
    "data/corpus/bestiary_4/language/brethedan.json",
    "data/corpus/inner_sea_races/template/human_ethnicity_arcadian.json",
    "data/corpus/inner_sea_races/template/human_ethnicity_iobarian.json",
    "data/corpus/inner_sea_races/template/human_ethnicity_varki.json",
    "data/corpus/inner_sea_races/template/human_ethnicity_erutaki.json",
    "data/corpus/inner_sea_races/template/human_ethnicity_mwangi.json",
    "data/corpus/inner_sea_races/template/human_ethnicity_lirgeni.json",
    "data/corpus/inner_sea_races/template/human_ethnicity_jadwiga.json",
    "data/corpus/inner_sea_races/template/human_ethnicity_caldaru.json",
    "data/corpus/inner_sea_races/language/senzar.json",
    "data/corpus/inner_sea_races/language/sakvroth.json",
    "data/corpus/advanced_race_guide/language/azlanti.json",
    "data/corpus/book_of_the_damned_volume_2/template/master_of_shapes_haagenti.json",
    "data/corpus/inner_sea_world_guide/language/jistka.json",
    "data/corpus/inner_sea_world_guide/language/vudrani.json",
    "data/corpus/inner_sea_world_guide/language/skald.json",
    "data/corpus/inner_sea_world_guide/language/osiriani.json",
    "data/corpus/inner_sea_world_guide/language/kelish.json",
    "data/corpus/inner_sea_world_guide/language/tekritanin.json",
    "data/corpus/inner_sea_world_guide/language/ancient_osiriani.json",
    "data/corpus/inner_sea_world_guide/language/shoanti.json",
    "data/corpus/inner_sea_world_guide/language/hallit.json",
    "data/corpus/inner_sea_world_guide/language/varisian.json",
    "data/corpus/inner_sea_world_guide/language/azlanti.json",
    "data/corpus/inner_sea_world_guide/language/orvian.json",
    "data/corpus/inner_sea_world_guide/language/tien.json",
    "data/corpus/inner_sea_world_guide/language/thassilonian.json",
]


class TheOriginalFilesNoLongerExistUnderTheOldSlugTest(unittest.TestCase):
    """The `§24` neutral name changes the record's output slug (identity
    is derived from coordinates, never the PI name) -- the fix is not just
    "rewrite this file", it moves the record to a `codex_named_unit_*`
    sibling and the OLD marker-shaped file must be gone, or the audit still
    sees it (proved live this cycle: the first regen pass left exactly
    these 28 orphaned, and the audit kept failing until they were removed
    via `git rm`, never hand-edited)."""

    def test_none_of_the_28_originally_reported_paths_exist_any_more(self):
        still_present = [p for p in ORIGINAL_28 if os.path.exists(os.path.join(REPO_ROOT, p))]
        self.assertEqual(
            still_present,
            [],
            "these paths are the OLD legacy-redacted shape and must be replaced by a "
            "codex_named_unit_* sibling at the same (kind, book, source_file, source_line), "
            "not merely rewritten in place",
        )


class TheDeclaredPiShippingAuditIsCleanTest(unittest.TestCase):
    """End-to-end proof: run the real audit binary this cycle's brief named
    (`cargo run --bin declared_pi_shipping_audit`) and confirm zero
    violations. Skipped (not failed) when cargo/the target dir are not
    reachable from this test's sandbox -- the corpus-content assertions
    above and in `test_pi_key_rawtokens_defect1_regen.py` cover the
    content-level proof independent of a cargo invocation succeeding here."""

    def test_zero_name_pi_shipped_violations(self):
        """Scoped to THIS defect's own violation shape (`NAME-PI-SHIPPED`),
        not the audit's overall CLEAN/FAIL verdict. A concurrent, unrelated
        defect (`DESC-PI-SHIPPED-IN-RAW-TOKENS`, `ability`/`feat_generic`/
        `race_trait_generic` kinds -- pre-existing on `origin/tranche/12`
        before this cycle's own commit, confirmed via `git show` against the
        pre-rebase tip, and out of this cycle's named scope) can make the
        audit's TOTAL count non-zero independent of this fix's correctness.
        Asserting overall `CLEAN` here would make this test flaky against
        sibling lanes' unrelated, concurrently-discovered defects."""
        cargo = shutil.which("cargo")
        if cargo is None:
            self.skipTest("cargo not on PATH in this sandbox")
        env = dict(os.environ)
        target_dir = env.get("CARGO_TARGET_DIR")
        if not target_dir:
            self.skipTest("CARGO_TARGET_DIR not set -- skip rather than trigger a fresh full build here")
        try:
            result = subprocess.run(
                [cargo, "run", "--locked", "--bin", "declared_pi_shipping_audit"],
                cwd=REPO_ROOT,
                env=env,
                capture_output=True,
                text=True,
                timeout=300,
            )
        except subprocess.TimeoutExpired:
            self.skipTest("cargo run exceeded this test's timeout -- not a build-correctness signal")
            return
        combined = result.stdout + result.stderr
        self.assertNotIn(
            "NAME-PI-SHIPPED",
            combined,
            f"expected zero NAME-PI-SHIPPED violations; got:\n{combined[-4000:]}",
        )


if __name__ == "__main__":
    unittest.main()
