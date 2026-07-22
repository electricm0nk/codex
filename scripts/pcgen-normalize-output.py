#!/usr/bin/env python3
"""pcgen-normalize-output.py — GE05/SD-25 Epic 4 PCGen runner scaffolding.

Normalizes a real PCGen character export produced via the legacy
``code/testsuite/base-xml.ftl`` Freemarker route — the same headless-Gradle
route named by the ``legacy_route`` field of
``tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt``
and driven end-to-end by ``scripts/pcgen-run-character.sh`` (criterion 4.1) —
into the golden-fixture comparison shape consumed by:

* ``src/oracle_validation/golden_fixture.rs`` — the ``GoldenCaseFixture``
  case-identity / legacy-oracle-evidence metadata (``case_id``,
  ``source_package``, ``legacy_oracle.route``, ...).
* ``src/oracle_validation/selected_parity_dimensions.rs`` — the
  ``SelectedDimension`` carrier shape (``id`` / ``value_string`` /
  ``value_i16`` / ``source_package_id``) for the mandatory pilot dimensions:
  ``character.identity``, ``combat.baseline_melee_attack_bonus``,
  ``defense.baseline_armor_class``, ``defense.total_save.{fortitude,reflex,will}``,
  and ``skill.selected_modifier.{climb,intimidate,swim}``.

This script performs no parity judgement of its own — it is a pure
normalizer from raw PCGen XML to the typed comparison shape above. Parity
comparison against Codex's own computed output is out of scope here
(consistent with golden_fixture.rs's documented non-goals) and belongs to a
later GE-05 comparator slice / SD-25 criterion 4.4's end-to-end wiring.

Usage:
    pcgen-normalize-output.py <pcgen_xml_path> [-o OUTPUT_JSON]
        [--case-id CASE_ID] [--source-package-id PACKAGE_ID]
        [--legacy-route ROUTE_TEXT]

Exit codes:
    0 — input parsed as well-formed XML (normalization diagnostics, if any,
        are reported in the output's "diagnostics" list; missing individual
        dimensions do not by themselves fail the run).
    1 — input file could not be read.
    2 — input file is not well-formed XML (hard failure; nothing to normalize).
"""

from __future__ import annotations

import argparse
import json
import re
import sys
import xml.etree.ElementTree as ET
from dataclasses import dataclass, field
from pathlib import Path

DEFAULT_CASE_ID = "pf1-crb-human-fighter-level1"
DEFAULT_SOURCE_PACKAGE_ID = "pf1.core_rulebook"
DEFAULT_LEGACY_ROUTE = (
    "headless Gradle run batch export via code/testsuite/base-xml.ftl"
)

# Skill display names as PCGen's base-xml.ftl <skill><name> tag renders them,
# mapped to the selected_parity_dimensions.rs dimension id suffix.
_SKILL_NAME_TO_SUFFIX = {
    "climb": "climb",
    "intimidate": "intimidate",
    "swim": "swim",
}

_SIGNED_INT_RE = re.compile(r"[-+]?\d+")


@dataclass
class SelectedDimension:
    """Mirrors src/oracle_validation/selected_parity_dimensions.rs's
    SelectedDimension struct field-for-field."""

    id: str
    value_string: str | None
    value_i16: int | None
    source_package_id: str

    def to_json(self) -> dict:
        return {
            "id": self.id,
            "value_string": self.value_string,
            "value_i16": self.value_i16,
            "source_package_id": self.source_package_id,
        }


@dataclass
class NormalizationResult:
    case_id: str
    source_package_id: str
    legacy_route: str
    claim_tier_floor: str
    dimensions: list[SelectedDimension] = field(default_factory=list)
    diagnostics: list[str] = field(default_factory=list)

    def to_json(self) -> dict:
        return {
            "case_id": self.case_id,
            "source_package_id": self.source_package_id,
            "legacy_route": self.legacy_route,
            "claim_tier_floor": self.claim_tier_floor,
            "dimensions": [d.to_json() for d in self.dimensions],
            "diagnostics": self.diagnostics,
        }


def _text(el: ET.Element | None) -> str | None:
    if el is None or el.text is None:
        return None
    stripped = el.text.strip()
    return stripped or None


def _signed_int(raw: str | None) -> int | None:
    if raw is None:
        return None
    match = _SIGNED_INT_RE.search(raw)
    if match is None:
        return None
    return int(match.group(0))


def _find_saving_throw_total(root: ET.Element, short_name: str) -> int | None:
    for throw in root.findall("./saving_throws/saving_throw"):
        short = _text(throw.find("./name/short"))
        if short is not None and short.strip().lower() == short_name:
            return _signed_int(_text(throw.find("./total")))
    return None


def _find_skill_modifier(root: ET.Element, skill_key: str) -> int | None:
    for skill in root.findall("./skills/skill"):
        name = _text(skill.find("./name"))
        if name is None:
            continue
        # PCGen renders e.g. "Climb" or "Climb (Str)"; match on the leading
        # bare skill name, case-insensitively.
        bare_name = name.split("(", 1)[0].strip().lower()
        if bare_name == skill_key:
            return _signed_int(_text(skill.find("./skill_mod")))
    return None


def normalize(
    root: ET.Element,
    *,
    case_id: str,
    source_package_id: str,
    legacy_route: str,
) -> NormalizationResult:
    result = NormalizationResult(
        case_id=case_id,
        source_package_id=source_package_id,
        legacy_route=legacy_route,
        claim_tier_floor="computed",
    )

    def emit_string(dimension_id: str, value: str | None, missing_message: str) -> None:
        if value is None:
            result.diagnostics.append(missing_message)
            return
        result.dimensions.append(
            SelectedDimension(
                id=dimension_id,
                value_string=value,
                value_i16=None,
                source_package_id=source_package_id,
            )
        )

    def emit_i16(dimension_id: str, value: int | None, missing_message: str) -> None:
        if value is None:
            result.diagnostics.append(missing_message)
            return
        result.dimensions.append(
            SelectedDimension(
                id=dimension_id,
                value_string=None,
                value_i16=value,
                source_package_id=source_package_id,
            )
        )

    emit_string(
        "character.identity",
        _text(root.find("./basics/name")),
        "missing PCGen output field for dimension 'character.identity' (expected /character/basics/name)",
    )

    emit_i16(
        "combat.baseline_melee_attack_bonus",
        _signed_int(_text(root.find("./attack/melee/total"))),
        "missing PCGen output field for dimension 'combat.baseline_melee_attack_bonus' "
        "(expected /character/attack/melee/total)",
    )

    emit_i16(
        "defense.baseline_armor_class",
        _signed_int(_text(root.find("./armor_class/total"))),
        "missing PCGen output field for dimension 'defense.baseline_armor_class' "
        "(expected /character/armor_class/total)",
    )

    for short_name, dimension_suffix in (
        ("fort", "fortitude"),
        ("ref", "reflex"),
        ("will", "will"),
    ):
        emit_i16(
            f"defense.total_save.{dimension_suffix}",
            _find_saving_throw_total(root, short_name),
            f"missing PCGen output field for dimension 'defense.total_save.{dimension_suffix}' "
            f"(expected /character/saving_throws/saving_throw[name/short='{short_name}']/total)",
        )

    for skill_key, dimension_suffix in _SKILL_NAME_TO_SUFFIX.items():
        emit_i16(
            f"skill.selected_modifier.{dimension_suffix}",
            _find_skill_modifier(root, skill_key),
            f"missing PCGen output field for dimension 'skill.selected_modifier.{dimension_suffix}' "
            f"(expected /character/skills/skill[name~='{skill_key}']/skill_mod)",
        )

    return result


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "pcgen_xml_path",
        help="Path to a real PCGen character XML export "
        "(base-xml.ftl legacy-route output; see 4.1's pcgen-run-character.sh)",
    )
    parser.add_argument(
        "-o",
        "--output",
        help="Write normalized JSON here instead of stdout",
    )
    parser.add_argument(
        "--case-id",
        default=DEFAULT_CASE_ID,
        help=f"Golden-case fixture case_id to stamp onto the output (default: {DEFAULT_CASE_ID})",
    )
    parser.add_argument(
        "--source-package-id",
        default=DEFAULT_SOURCE_PACKAGE_ID,
        help=f"Source package id to stamp onto each dimension (default: {DEFAULT_SOURCE_PACKAGE_ID})",
    )
    parser.add_argument(
        "--legacy-route",
        default=DEFAULT_LEGACY_ROUTE,
        help="Legacy-oracle route description to record on the output",
    )
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)

    input_path = Path(args.pcgen_xml_path)
    try:
        raw = input_path.read_bytes()
    except OSError as exc:
        print(f"pcgen-normalize-output.py: cannot read '{input_path}': {exc}", file=sys.stderr)
        return 1

    try:
        root = ET.fromstring(raw)
    except ET.ParseError as exc:
        print(
            f"pcgen-normalize-output.py: '{input_path}' is not well-formed XML: {exc}",
            file=sys.stderr,
        )
        return 2

    result = normalize(
        root,
        case_id=args.case_id,
        source_package_id=args.source_package_id,
        legacy_route=args.legacy_route,
    )

    output_text = json.dumps(result.to_json(), indent=2, sort_keys=False)
    if args.output:
        Path(args.output).write_text(output_text + "\n", encoding="utf-8")
    else:
        print(output_text)

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
