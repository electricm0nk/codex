#!/usr/bin/env python3
"""Tests for `scripts/box_ledger.py` (SD-33 Epic 1, AT-33-E1-001).

Proves the load-bearing claim `THE-BOX.md` exists to make: that every unit
in `docs/work-inventory.json` belongs to **exactly one** named group in the
box's ledger table -- and that the tool actually detects the negative cases
(a unit in no group, a unit in two groups) rather than only ever reporting
success on inputs shaped to pass.

Uses small synthetic inventory/box fixtures, not the live 49,438-unit
corpus, so these tests stay fast and are not subject to corpus drift across
cycles (`test_coverage_ledger.py` sets the same precedent for this repo).
The live corpus is exercised separately, as acceptance evidence, by running
the committed CLI against the committed `docs/work-inventory.json` and
`THE-BOX.md` -- not inside this fast unit-test file.
"""

import json
import os
import subprocess
import sys
import tempfile
import unittest

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.join(REPO_ROOT, "scripts"))
import box_ledger as BL  # noqa: E402


def _unit(id_, status, **extra):
    u = {"id": id_, "status": status}
    u.update(extra)
    return u


def _inventory(units):
    return {"totals": {"units": len(units)}, "units": units}


def _box_doc(groups):
    """Build a minimal THE-BOX.md-shaped document: prose plus one fenced
    ```json ledger block, exactly what `box_ledger.load_box` must parse."""
    body = json.dumps({"groups": groups}, indent=2)
    return (
        "# THE-BOX\n\nSome prose a human reads.\n\n"
        "```json ledger\n" + body + "\n```\n\nMore prose.\n"
    )


GROUP_A = {
    "id": "alpha",
    "disposition": "held",
    "count": 2,
    "match": {"status": "alpha-status"},
    "command": "echo alpha",
}
GROUP_B = {
    "id": "beta",
    "disposition": "unverifiable",
    "count": 1,
    "match": {"status": "beta-status"},
    "command": "echo beta",
}

UNIT_A1 = _unit("book:kind:a1", "alpha-status")
UNIT_A2 = _unit("book:kind:a2", "alpha-status")
UNIT_B1 = _unit("book:kind:b1", "beta-status")


class TestLoadInventory(unittest.TestCase):
    def test_population_is_len_of_units_not_trusted_totals(self):
        # totals.units deliberately wrong -- load_inventory must not trust it.
        doc = {"totals": {"units": 999}, "units": [UNIT_A1, UNIT_A2, UNIT_B1]}
        with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as f:
            json.dump(doc, f)
            path = f.name
        try:
            units, population = BL.load_inventory(path)
            self.assertEqual(population, 3)
            self.assertEqual(len(units), 3)
        finally:
            os.unlink(path)


class TestLoadBox(unittest.TestCase):
    def test_parses_ledger_groups(self):
        text = _box_doc([GROUP_A, GROUP_B])
        with tempfile.NamedTemporaryFile("w", suffix=".md", delete=False) as f:
            f.write(text)
            path = f.name
        try:
            groups = BL.load_box(path)
            self.assertEqual([g["id"] for g in groups], ["alpha", "beta"])
        finally:
            os.unlink(path)

    def test_missing_ledger_block_raises(self):
        with tempfile.NamedTemporaryFile("w", suffix=".md", delete=False) as f:
            f.write("# THE-BOX\n\nNo fenced ledger block here.\n")
            path = f.name
        try:
            with self.assertRaises(ValueError):
                BL.load_box(path)
        finally:
            os.unlink(path)


class TestPartition(unittest.TestCase):
    def test_full_coverage_no_overlap(self):
        units = [UNIT_A1, UNIT_A2, UNIT_B1]
        groups = [GROUP_A, GROUP_B]
        result = BL.partition(units, groups)
        self.assertEqual(result.uncovered, [])
        self.assertEqual(result.overlap, [])
        self.assertEqual(result.population, 3)

    def test_detects_uncovered_unit(self):
        stray = _unit("book:kind:stray", "no-such-status")
        units = [UNIT_A1, UNIT_B1, stray]
        groups = [GROUP_A, GROUP_B]
        result = BL.partition(units, groups)
        self.assertEqual(result.uncovered, ["book:kind:stray"])
        self.assertEqual(result.overlap, [])

    def test_detects_overlapping_unit(self):
        # A group whose match is broader and collides with GROUP_A on UNIT_A1.
        broad_group = {
            "id": "broad",
            "disposition": "held",
            "count": 1,
            "match": {"status": "alpha-status"},
            "command": "echo broad",
        }
        units = [UNIT_A1]
        groups = [GROUP_A, broad_group]
        result = BL.partition(units, groups)
        self.assertEqual(result.overlap, ["book:kind:a1"])
        self.assertEqual(result.uncovered, [])


class TestCli(unittest.TestCase):
    def _write(self, dir_, inventory_units, groups):
        inv_path = os.path.join(dir_, "work-inventory.json")
        box_path = os.path.join(dir_, "THE-BOX.md")
        with open(inv_path, "w") as f:
            json.dump(_inventory(inventory_units), f)
        with open(box_path, "w") as f:
            f.write(_box_doc(groups))
        return inv_path, box_path

    def test_check_exits_zero_on_full_coverage(self):
        with tempfile.TemporaryDirectory() as d:
            inv_path, box_path = self._write(d, [UNIT_A1, UNIT_A2, UNIT_B1], [GROUP_A, GROUP_B])
            proc = subprocess.run(
                [sys.executable, os.path.join(REPO_ROOT, "scripts", "box_ledger.py"),
                 "--check", "--inventory", inv_path, "--box", box_path],
                capture_output=True, text=True,
            )
            self.assertEqual(proc.returncode, 0, proc.stdout + proc.stderr)
            self.assertIn("uncovered=0 overlap=0 population=3", proc.stdout)

    def test_check_exits_nonzero_on_uncovered(self):
        stray = _unit("book:kind:stray", "no-such-status")
        with tempfile.TemporaryDirectory() as d:
            inv_path, box_path = self._write(d, [UNIT_A1, UNIT_B1, stray], [GROUP_A, GROUP_B])
            proc = subprocess.run(
                [sys.executable, os.path.join(REPO_ROOT, "scripts", "box_ledger.py"),
                 "--check", "--inventory", inv_path, "--box", box_path],
                capture_output=True, text=True,
            )
            self.assertNotEqual(proc.returncode, 0, proc.stdout + proc.stderr)
            self.assertIn("uncovered=1", proc.stdout)

    def test_check_against_live_committed_files(self):
        """The real acceptance bar: the committed THE-BOX.md against the
        committed docs/work-inventory.json, population stated in the
        criterion (49,438), confirmed here by execution, not by memory."""
        inv_path = os.path.join(REPO_ROOT, "docs", "work-inventory.json")
        box_path = os.path.join(
            REPO_ROOT, "docs", "release",
            "SD-33-computed-value-verification", "THE-BOX.md",
        )
        if not (os.path.exists(inv_path) and os.path.exists(box_path)):
            self.skipTest("live THE-BOX.md not yet committed")
        proc = subprocess.run(
            [sys.executable, os.path.join(REPO_ROOT, "scripts", "box_ledger.py"),
             "--check", "--inventory", inv_path, "--box", box_path],
            capture_output=True, text=True,
        )
        self.assertEqual(proc.returncode, 0, proc.stdout + proc.stderr)
        self.assertIn("uncovered=0 overlap=0 population=49438", proc.stdout)


if __name__ == "__main__":
    unittest.main()
