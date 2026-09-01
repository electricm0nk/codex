#!/usr/bin/env python3
"""Regression test for AT-34-E6-001 gate lane B (SD-34 wave 23).

`site/dashboard/units/*.json` is a committed cache of per-kind unit rows
(`build_unit_shards()` in `scripts/observer/pf1e_dashboard_producer.py`),
consumed directly by `scripts/site/build_public_status.py` (see that
script's own module docstring: it deliberately reads these shards, not
`docs/work-inventory.json`, to avoid a heavier dependency chain).

AT-34-E1-005 (SD-34 Epic 1) renamed the misleading `not-ingested` status
word to `engine-does-not-hold` across `src/bin/v06_work_inventory.rs`,
`docs/work-inventory.json`, and every consumer under `tests/`, `src/`,
`apps/`, `scripts/` -- but that criterion's own directory list did not
include `site/`, so this committed cache kept the old spelling (19 files,
36 live occurrences, found live 2026-09-01 when
`python3 scripts/site/build_public_status.py --check` raised `ValueError:
doneness: unmapped 'static' + 'not-ingested'` reading
`site/dashboard/units/PF1e-units-ability.json`).

The fix is a plain string rename in the committed cache files (`sed
's/not-ingested/engine-does-not-hold/g'`, the exact command AT-34-E1-005's
own receipt used for `docs/work-inventory.json`), each validated as
well-formed JSON before and after -- NOT a run of the dashboard producer
(`scripts/publish-site-dashboard.sh`, no `--check`), which is out of this
lane's authority (it can silently drop other stamps; see
`docs/release/SD-34-book-completion/fable-review.md` §7's hazard note).

This test is the mechanical regression guard for that rename, the same
shape as `scripts/tests/test_legacy_not_ingested_string_swept.py`
(AT-34-E1-005's own sweep) but scoped to the one directory that sweep's
criterion excluded.

Run: python3 -m unittest scripts.tests.test_site_dashboard_units_status_vocabulary_current
"""
from __future__ import annotations

import glob
import json
import os
import re
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
UNITS_DIR = os.path.join(REPO_ROOT, "site", "dashboard", "units")

# Same pattern AT-34-E1-005's own sweep uses -- word-boundary only, no
# trailing `\b`, matching the standing convention.
_OLD_PATTERN = re.compile(r"\bnot[-_]ingested\b")


def _shard_files():
    return sorted(glob.glob(os.path.join(UNITS_DIR, "*.json")))


class SiteDashboardUnitsStatusVocabularyTest(unittest.TestCase):
    def test_no_legacy_status_word_in_committed_shards(self):
        if not os.path.isdir(UNITS_DIR):
            self.skipTest(f"{UNITS_DIR} not present")
        hits = {}
        for path in _shard_files():
            with open(path, "r", encoding="utf-8") as fh:
                text = fh.read()
            if _OLD_PATTERN.search(text):
                hits[os.path.relpath(path, REPO_ROOT)] = len(_OLD_PATTERN.findall(text))
        self.assertEqual(
            {}, hits,
            f"legacy `not-ingested` status word found in committed unit shards "
            f"(renamed to `engine-does-not-hold` everywhere else by AT-34-E1-005): {hits}",
        )

    def test_shards_are_well_formed_json(self):
        files = _shard_files()
        if not files:
            self.skipTest(f"no shard files under {UNITS_DIR}")
        for path in files:
            with open(path, "r", encoding="utf-8") as fh:
                try:
                    json.load(fh)
                except json.JSONDecodeError as exc:
                    self.fail(f"{os.path.relpath(path, REPO_ROOT)} is not valid JSON: {exc}")

    def test_build_public_status_check_does_not_raise_on_the_committed_shards(self):
        """The actual live proof: `classify_all()` must not raise reading
        whatever is currently committed under `site/dashboard/units/`. This
        is what caught the original defect (`ValueError: doneness: unmapped
        'static' + 'not-ingested'`) -- a string sweep alone would not prove
        the *consumer* is unblocked, only that the string is gone."""
        import importlib.util
        import pathlib

        build_public_status_path = (
            pathlib.Path(REPO_ROOT) / "scripts" / "site" / "build_public_status.py"
        )
        spec = importlib.util.spec_from_file_location("build_public_status", build_public_status_path)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        try:
            module.build()
        except ValueError as exc:
            self.fail(f"build_public_status.build() raised on the committed shards: {exc}")


if __name__ == "__main__":
    unittest.main()
