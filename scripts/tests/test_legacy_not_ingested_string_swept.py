#!/usr/bin/env python3
"""Regression test for AT-34-E1-005 (SD-34 Epic 1).

`not-ingested` asserted the opposite of what it meant: 26,002 of 26,002 of
its units carried a real `source_file`/`source_line`, and it already misled
once (this package's own first draft reported "52.7% not ingested" to the
operator). The status was renamed to `engine-does-not-hold` across
`src/bin/v06_work_inventory.rs`, `docs/work-inventory.json`, and every
consumer under `tests/`, `src/`, `apps/`, `scripts/`.

This test is the mechanical count sweep `acceptance-and-verification.md`
names as AT-34-E1-005's artifact: it fails CLOSED (nonzero exit, live count
printed) the moment either legacy spelling of the old name reappears in a
tracked file under one of those four directories, and proves it does so
for the intended reason via a plant/revert mutation -- not by assuming the
rename holds forever.
"""

from __future__ import annotations

import os
import re
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

SEARCH_DIRS = ("tests", "src", "apps", "scripts")

# The old spellings this cycle renamed away from. Matched with a leading
# word-boundary only (mirrors `workflow-instruction.md §6`'s own identifier
# grep: a trailing `\b` would still catch these -- both idioms end in a
# letter -- but is kept off here too, for the same "don't get clever, get
# consistent with the standing convention" reason).
_OLD_PATTERN = re.compile(r"\bnot[-_]ingested\b")

# This file's own path, and this module's own docstring/pattern above,
# necessarily mention the retired spelling -- excluded from the sweep it
# performs, exactly the way a lint rule excludes its own source.
_SELF_RELATIVE = os.path.join("scripts", "tests", "test_legacy_not_ingested_string_swept.py")

_TEXT_EXTENSIONS = (
    ".rs", ".py", ".ts", ".tsx", ".js", ".json", ".md", ".html", ".env",
)


def _iter_scanned_files():
    for top in SEARCH_DIRS:
        top_path = os.path.join(REPO_ROOT, top)
        if not os.path.isdir(top_path):
            continue
        for dirpath, dirnames, filenames in os.walk(top_path):
            dirnames[:] = [d for d in dirnames if d not in (".git", "__pycache__", "node_modules", "target")]
            for name in filenames:
                if not name.endswith(_TEXT_EXTENSIONS):
                    continue
                full = os.path.join(dirpath, name)
                rel = os.path.relpath(full, REPO_ROOT)
                if rel == _SELF_RELATIVE:
                    continue
                yield rel, full


def sweep() -> dict:
    """Return {relative_path: [line_numbers]} for every live hit of the
    retired `not-ingested`/`not_ingested` spelling under tests/src/apps/scripts.
    """
    hits: dict[str, list[int]] = {}
    for rel, full in _iter_scanned_files():
        try:
            with open(full, "r", encoding="utf-8", errors="ignore") as fh:
                lines = fh.readlines()
        except OSError:
            continue
        matched = [i + 1 for i, line in enumerate(lines) if _OLD_PATTERN.search(line)]
        if matched:
            hits[rel] = matched
    return hits


class LegacyStatusStringSweepTest(unittest.TestCase):
    def test_no_live_uses_remain_under_the_four_scanned_directories(self):
        hits = sweep()
        self.assertEqual(
            {}, hits,
            f"live `not-ingested`/`not_ingested` uses found (renamed to "
            f"`engine-does-not-hold` by AT-34-E1-005): {hits}",
        )

    def test_sweep_goes_red_on_a_planted_use_and_green_on_its_revert(self):
        """Prove-RED requirement: the sweep must fail for the intended
        reason (a real live use appearing), not merely report zero because
        it scans nothing."""
        planted_dir = os.path.join(REPO_ROOT, "scripts", "tests", "_at_34_e1_005_plant")
        planted_path = os.path.join(planted_dir, "planted.py")
        os.makedirs(planted_dir, exist_ok=True)
        try:
            with open(planted_path, "w", encoding="utf-8") as fh:
                fh.write("status = 'not-ingested'  # planted for RED proof\n")

            red_hits = sweep()
            planted_rel = os.path.relpath(planted_path, REPO_ROOT)
            self.assertIn(planted_rel, red_hits, "planted legacy string was not detected -- sweep is not RED")
        finally:
            os.remove(planted_path)
            os.rmdir(planted_dir)

        green_hits = sweep()
        self.assertNotIn(
            os.path.relpath(planted_path, REPO_ROOT), green_hits,
            "planted file removed but sweep still reports it -- stale state",
        )


if __name__ == "__main__":
    unittest.main()
