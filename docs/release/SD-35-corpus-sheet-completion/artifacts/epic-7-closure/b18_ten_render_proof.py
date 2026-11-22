#!/usr/bin/env python3
"""SD-35 operator ruling B18 (`decisions.md §21`), step 3: the units the widened
`has_classifying_token` predicate admitted each reach a rendered sheet line.

Not "is in the inventory" -- RENDERS. For every unit id this run admitted, the
script reads `docs/work-inventory.json` for its status, finds the unit's own
converted rule in `data/sheet_rules/<book>/<kind>/<key>.json`, and prints the
sheet line that rule produces: the label plus its prose, in the final form the
Character Sheet prints (the sheet rule, `decisions.md §1` -- the rule's words,
no simulation). A unit that enumerates but renders nothing is reported as
NOT DONE, loudly, and the script exits non-zero.

    python3 docs/release/SD-35-corpus-sheet-completion/artifacts/epic-7-closure/b18_ten_render_proof.py

Writes `b18-render-proof.json` beside itself.
"""

from __future__ import annotations

import json
import pathlib
import sys

HERE = pathlib.Path(__file__).resolve().parent
REPO = HERE.parents[4]
INVENTORY = REPO / "docs/work-inventory.json"
RULES = REPO / "data/sheet_rules"
# The ids ruling B18's widened predicate admitted, re-derived (never pasted) by
# `widened_predicate_census.py`'s `admitted_unit_ids` row.
ADMITTED = HERE / "b18-admitted-units.json"

DONE_STATUSES = {
    "grounded",
    "text-complete",
    "sheet-complete",
    "oracle-agree",
    "oracle-unverifiable",
}


def sheet_lines(rule: dict) -> list[str]:
    """Every printable line this rule contributes, in sheet order."""
    if rule.get("print") is False:
        return []
    out: list[str] = []
    label = (rule.get("label") or "").strip()
    for family in rule.get("prose") or []:
        # A prose family gated `applies: "Never"` is the converter's
        # unresolved-formula variant: it is carried for provenance and a
        # resolved sibling prints in its place (`core_rulebook:spell:
        # magic_vestment` does exactly this, pre-B18). Counting it would let
        # this proof claim a sheet line the sheet never shows.
        if family.get("applies") == "Never":
            continue
        for piece in family.get("pieces") or []:
            if isinstance(piece, dict) and isinstance(piece.get("Text"), str):
                text = piece["Text"].strip()
                if text:
                    out.append(f"{label}: {text}" if label else text)
    value = rule.get("value")
    if isinstance(value, dict):
        # A resolved magnitude renders as its own final form ("DC 15", "1d8+2").
        for form in ("Number", "Dice", "Text"):
            if form in value:
                rendered = value[form]
                if isinstance(rendered, (int, float, str)) and str(rendered).strip():
                    out.append(f"{label}: {rendered}")
    return out


def find_rule(book: str, kind: str, unit_id: str) -> tuple[pathlib.Path | None, dict | None]:
    directory = RULES / book / kind
    if not directory.is_dir():
        return None, None
    for path in sorted(directory.rglob("*.json")):
        try:
            blob = json.loads(path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError):
            continue
        for rule in blob if isinstance(blob, list) else [blob]:
            if isinstance(rule, dict) and rule.get("id") == unit_id:
                return path, rule
    return None, None


def main() -> int:
    admitted = json.loads(ADMITTED.read_text(encoding="utf-8"))["ids"]
    units = {u["id"]: u for u in json.loads(INVENTORY.read_text(encoding="utf-8"))["units"]}

    rows = []
    failures = []
    for unit_id in admitted:
        unit = units.get(unit_id)
        if unit is None:
            failures.append(f"{unit_id}: not in docs/work-inventory.json")
            rows.append({"id": unit_id, "in_inventory": False, "renders": False})
            continue
        path, rule = find_rule(unit["book"], unit["kind"], unit_id)
        lines = sheet_lines(rule) if rule else []
        row = {
            "id": unit_id,
            "name": unit["name"],
            "book": unit["book"],
            "kind": unit["kind"],
            "source": f"{unit['source_file']}:{unit['source_line']}",
            "status": unit["status"],
            "evidence": unit["evidence"],
            "in_inventory": True,
            "rule_file": str(path.relative_to(REPO)) if path else None,
            "renders": bool(lines),
            "sheet_lines": lines,
        }
        rows.append(row)
        if not lines:
            failures.append(f"{unit_id}: enumerates but renders NOTHING -- not done")
        if unit["status"] not in DONE_STATUSES:
            failures.append(f"{unit_id}: status {unit['status']} is not a DONE status")

    report = {
        "ruling": "SD-35 operator ruling B18 (decisions.md §21)",
        "criterion": "AT-35-E7-000-POPULATION-FIX",
        "admitted": len(admitted),
        "render": sum(1 for r in rows if r["renders"]),
        "failures": failures,
        "units": rows,
    }
    (HERE / "b18-render-proof.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")

    for row in rows:
        head = row["sheet_lines"][0] if row["renders"] else "*** RENDERS NOTHING ***"
        print(f"{row['id']}\n    status={row.get('status')}  renders={row['renders']}\n    {head[:160]}")
    print(f"\nadmitted={len(admitted)} render={report['render']} failures={len(failures)}")
    for failure in failures:
        print(f"  FAIL {failure}")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
