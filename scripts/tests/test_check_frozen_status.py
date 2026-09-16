"""Self-test for `scripts/site/check_frozen_status.py` (SD-36 Epic B, D3/D5).

RED before the D5 regen: the committed site/status-data.json still reads
95.0% of 37,892 under the retired SD-31 denominator rule, so `check()`
returns violations and this fails. GREEN after: the regenerated snapshot
reads 100% of 49,450 under the inventory's own DONE vocabulary and every
per-book detail file reconciles.

Run: python3 -m unittest -v scripts/tests/test_check_frozen_status.py
(matches the runner every other scripts/tests/*.py self-test in this repo
uses — pytest is not installed in this environment).
"""
import importlib.util
import json
import pathlib
import unittest

_REPO_ROOT = pathlib.Path(__file__).resolve().parent.parent.parent
_MODULE_PATH = _REPO_ROOT / "scripts" / "site" / "check_frozen_status.py"
_spec = importlib.util.spec_from_file_location("check_frozen_status", _MODULE_PATH)
cfs = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(cfs)


class FrozenStatusCheckTests(unittest.TestCase):
    def test_committed_snapshot_is_frozen_at_100_percent(self):
        violations = cfs.check()
        self.assertEqual(violations, [], "\n".join(violations))

    def test_frozen_denominator_matches_the_work_inventory_total_at_freeze_time(self):
        inventory = json.loads((_REPO_ROOT / "docs" / "work-inventory.json").read_text())
        self.assertEqual(cfs.FROZEN_DENOMINATOR, inventory["totals"]["units"])

    def test_missing_status_data_file_is_a_violation_not_a_crash(self):
        violations = cfs.check(
            status_data_path=_REPO_ROOT / "does" / "not" / "exist.json",
            book_dir=_REPO_ROOT / "site" / "status-data",
        )
        self.assertEqual(len(violations), 1)
        self.assertIn("does not exist", violations[0])

    def test_missing_book_detail_file_is_reported(self):
        scratch = json.loads((_REPO_ROOT / "site" / "status-data.json").read_text())
        tmp = pathlib.Path(pathlib.Path(__file__).parent / "_scratch_status_data.json")
        try:
            scratch["books"] = scratch["books"] + [
                {"id": "nonexistent_book", "title": "Nonexistent", "done": 0, "partial": 0,
                 "not_started": 0, "denominator": 0, "pct": 0.0,
                 "excluded_from_percentage": 0, "standing_breakdown": {}}
            ]
            tmp.write_text(json.dumps(scratch))
            violations = cfs.check(status_data_path=tmp, book_dir=_REPO_ROOT / "site" / "status-data")
            self.assertTrue(any("nonexistent_book" in v for v in violations))
        finally:
            tmp.unlink(missing_ok=True)


if __name__ == "__main__":
    unittest.main()
