"""
Tests for scripts/pcgen-normalize-output.py.

Doctrine (v0.6 alpha swarm, self-directed backend scan, 2026-07-24):

  - `src/oracle_validation/selected_parity_dimensions.rs` and this script must
    agree on the same dimension-id set, or the comparator (`comparator.rs`)
    will report every dimension one side has and the other doesn't as a
    `MissingFromCodex`/`MissingFromPcgen` mismatch rather than a real
    comparison.
  - `combat.base_attack_bonus` (the raw class-table BAB, distinct from
    `combat.baseline_melee_attack_bonus`'s Strength/feat-inclusive total) is
    real, exported by PCGen's own `base-xml.ftl` template as
    `/character/attack/melee/bab`, and was never extracted before this fix --
    confirmed empirically by running the real PCGen pipeline against the
    Fighter pilot fixture (`<bab>+1</bab>`, matching Codex's own computed
    `base_attack_bonus` of +1 for the same build) before writing this test.

Invoked via subprocess against a small hand-built XML fixture (mirroring the
real `base-xml.ftl` shape) rather than importing the script as a module --
its filename has a hyphen, so it isn't import-friendly, and every other
python script test in this repo (`scripts/tranche/tests/
test_validate_tranche_notes.py`) already uses the same subprocess-CLI
pattern.
"""

from __future__ import annotations

import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
NORMALIZER = REPO_ROOT / "scripts" / "pcgen-normalize-output.py"

# A minimal but real-shaped fragment of base-xml.ftl's actual output
# structure (fields this script already parses, plus the one this test adds
# coverage for) -- not the full real export, but every element read by
# `normalize()` is present with the same nesting and tag names as the real
# template.
SAMPLE_XML = """<?xml version="1.0" encoding="UTF-8"?>
<character>
    <basics>
        <name>Test Fighter</name>
    </basics>
    <attack>
        <melee>
            <total>+5</total>
            <bab>+1</bab>
            <base_attack_bonus>+1</base_attack_bonus>
        </melee>
    </attack>
    <armor_class>
        <total>16</total>
    </armor_class>
    <saving_throws>
        <saving_throw><name><short>fort</short></name><total>2</total></saving_throw>
        <saving_throw><name><short>ref</short></name><total>0</total></saving_throw>
        <saving_throw><name><short>will</short></name><total>0</total></saving_throw>
    </saving_throws>
    <skills>
        <skill><name>Climb</name><skill_mod>5</skill_mod></skill>
        <skill><name>Intimidate</name><skill_mod>-1</skill_mod></skill>
        <skill><name>Swim</name><skill_mod>5</skill_mod></skill>
    </skills>
    <weight_allowance>
        <light>76</light>
        <medium>153</medium>
        <heavy>230</heavy>
    </weight_allowance>
    <equipment>
        <total><weight>39</weight></total>
    </equipment>
    <hit_points>
        <points>12</points>
        <current/>
    </hit_points>
    <misc>
        <funds><fund>5 gp</fund></funds>
    </misc>
</character>
"""


def _run_normalizer(xml_path: Path) -> dict:
    result = subprocess.run(
        [sys.executable, str(NORMALIZER), str(xml_path)],
        capture_output=True,
        text=True,
    )
    assert result.returncode == 0, f"normalizer failed: {result.stderr}"
    return json.loads(result.stdout)


class BaseAttackBonusDimensionTest(unittest.TestCase):
    def test_extracts_the_raw_base_attack_bonus_distinct_from_the_melee_total(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            xml_path = Path(tmp) / "fighter.xml"
            xml_path.write_text(SAMPLE_XML, encoding="utf-8")

            output = _run_normalizer(xml_path)

        dims = {d["id"]: d for d in output["dimensions"]}

        self.assertIn(
            "combat.base_attack_bonus",
            dims,
            f"expected a combat.base_attack_bonus dimension, got: {sorted(dims)}",
        )
        self.assertEqual(dims["combat.base_attack_bonus"]["value_i16"], 1)

        # Distinct from the pre-existing melee-total dimension -- this test
        # would pass vacuously if the new field were wired to the same
        # <total> element instead of <bab>.
        self.assertEqual(dims["combat.baseline_melee_attack_bonus"]["value_i16"], 5)

    def test_missing_bab_element_reports_a_diagnostic_not_a_fabricated_zero(self) -> None:
        xml_without_bab = SAMPLE_XML.replace("<bab>+1</bab>", "")
        with tempfile.TemporaryDirectory() as tmp:
            xml_path = Path(tmp) / "fighter-no-bab.xml"
            xml_path.write_text(xml_without_bab, encoding="utf-8")

            output = _run_normalizer(xml_path)

        dims = {d["id"]: d for d in output["dimensions"]}
        self.assertNotIn("combat.base_attack_bonus", dims)
        self.assertTrue(
            any("combat.base_attack_bonus" in diagnostic for diagnostic in output["diagnostics"]),
            f"expected a missing-field diagnostic, got: {output['diagnostics']}",
        )


if __name__ == "__main__":
    unittest.main()
