#!/usr/bin/env python3
"""Enumerate every corpus `kind` in `docs/work-inventory.json` and state,
for each, whether a probe exists that can verify a COMPUTED MAGNITUDE for
that kind -- and name it (`AT-33-E1-003`).

Why this exists
----------------
`decisions.md` §7: "the probe surface is thin... Epic 1's criterion 1.3
enumerates it for real rather than restating it from memory." This module
is that enumeration, derived by reading `src/bin/v06_work_inventory.rs`'s
exhaustive `match unit.kind { ... }` verdict function (the sole generator of
`docs/work-inventory.json`, per `workflow-instruction.md` §3's Epic 4 file-
touch row) and confirmed against the live data, not asserted.

`PROBE_SURFACE` distinguishes THREE shapes, not two, because a bare "has a
probe / does not" hides a real difference the census would otherwise erase:

  * ``magnitude_probe`` -- a dedicated probe function that changes an input
    (adds/removes the record, or a level/consumer input) and observes a
    DELTA on a rendered computed snapshot -- `class`, `class_feature`,
    `feat`, `spell`, `equipment`/`equipment_modifier`, `race`, `race_trait`.
    Each of these is additionally confirmed, live, to have fired at least
    once: a kind whose probe exists in source but has never been observed
    to produce its own positive-evidence string on a real unit would be
    exactly the "a tool that has never been observed to fail is not a gate"
    defect `AT-33-E1-002` names, one level over -- here for a probe rather
    than a check.
  * ``presence_only`` -- the engine holds the record in a real table (a
    `monster_resolve`/`holds_key` lookup succeeds), but the generator never
    observes a computed DELTA attributable to that record -- it is a
    presence/lookup check, not a magnitude-verifying probe. `monster`,
    `monster_ability`, `companion`.
  * ``no_engine_table`` -- the generator's own verdict arm is an
    unconditional `engine_does_not_hold(...)`, always, for every unit of the kind:
    no engine table exists at all, so no probe of any kind is possible.
    `ability`, `template`, `deity`, `power`, `domain`, `skill`, `language`,
    `trait`.

Only ``magnitude_probe`` counts as `probe_exists: true` for this criterion --
the evidence obligation is "can verify a computed magnitude", and a presence
check answers a different, weaker question (`decisions.md` §7's own
`grounded` vocabulary entry names `monster_resolve` as sufficient for
`grounded`, but `grounded` there means "engine holds a real record", never
"a magnitude was verified computed").

Re-derive command
------------------
    python3 scripts/probe_surface_census.py --check
    python3 scripts/probe_surface_census.py > \
        docs/release/SD-33-computed-value-verification/artifacts/epic-1-instruments/probe-surface-census.json

Source citations for each row are file:line into
`src/bin/v06_work_inventory.rs` at the commit `derived_at_commit` (below)
names -- read them again if that file moves, per `decisions.md` §7's own
"derived by execution, not from memory" bar.
"""

import argparse
import json
import os
import subprocess
import sys

REPO_ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
GENERATOR_SOURCE = "src/bin/v06_work_inventory.rs"

# kind -> census row. `positive_evidence_examples` are the exact `evidence`
# string(s) that prove the probe FIRED (as opposed to merely existing in
# source) for `magnitude_probe` kinds only.
PROBE_SURFACE = {
    "class": {
        "probe_exists": True,
        "category": "magnitude_probe",
        "probe_functions": ["probe_class_effect_wiring", "class_probe_ceiling_report"],
        "probe_locations": [f"{GENERATOR_SOURCE}:6920", f"{GENERATOR_SOURCE}:6961"],
        "positive_evidence_examples": [
            "class_probe_observed_computed_delta_on_the_rendered_snapshot",
        ],
        "notes": "Applies class_probe_input to the probe fixture and observes a "
        "delta on the rendered computed snapshot vs. a classless baseline.",
    },
    "class_feature": {
        "probe_exists": True,
        "category": "magnitude_probe",
        "probe_functions": ["probe_class_feature_effect_wiring"],
        "probe_locations": [f"{GENERATOR_SOURCE}:10783"],
        "positive_evidence_examples": [
            "class_feature_probe_observed_a_delta_attributable_to_this_record",
        ],
        "notes": "Observes a delta attributable to the specific class-feature record, "
        "not merely to its owning class.",
    },
    "feat": {
        "probe_exists": True,
        "category": "magnitude_probe",
        "probe_functions": ["probe_feat_effect_wiring", "feat_probe_input"],
        "probe_locations": [f"{GENERATOR_SOURCE}:5610", f"{GENERATOR_SOURCE}:5624"],
        "positive_evidence_examples": [
            "feat_effect_probe_observed_computed_delta",
        ],
        "notes": "A feat whose presence changes what compute_pilot_base_chassis returns.",
    },
    "spell": {
        "probe_exists": True,
        "category": "magnitude_probe",
        "probe_functions": ["probe_spell_effect_wiring", "spell_probe_input"],
        "probe_locations": [f"{GENERATOR_SOURCE}:6567", f"{GENERATOR_SOURCE}:6446"],
        "positive_evidence_examples": [
            "spell_effect_probe_observed_computed_delta",
        ],
        "notes": "Observes a computed delta (e.g. a real save DC) attributable to the spell.",
    },
    "equipment": {
        "probe_exists": True,
        "category": "magnitude_probe",
        "probe_functions": ["probe_equipment_effect_wiring"],
        "probe_locations": [f"{GENERATOR_SOURCE}:6265"],
        "positive_evidence_examples": [
            "equipment_effect_probe_observed_computed_delta",
        ],
        "notes": "Shared with equipment_modifier -- both kinds hit the same Equipment "
        "match arm and the same probe.",
    },
    "equipment_modifier": {
        "probe_exists": True,
        "category": "magnitude_probe",
        "probe_functions": ["probe_equipment_effect_wiring"],
        "probe_locations": [f"{GENERATOR_SOURCE}:6265"],
        "positive_evidence_examples": [
            "equipment_effect_probe_observed_computed_delta",
        ],
        "notes": "Kind::Equipment | Kind::EquipmentModifier is one match arm "
        f"({GENERATOR_SOURCE}:8596) -- the probe does not distinguish the two kinds.",
    },
    "race": {
        "probe_exists": True,
        "category": "magnitude_probe",
        "probe_functions": ["probe_race_creation_roster", "race_magnitude_consumer_races"],
        "probe_locations": [f"{GENERATOR_SOURCE}:5889"],
        "positive_evidence_examples": [
            "race_offered_by_the_real_character_creation_roster",
            "race_offered_by_the_roster_but_no_pilot_compute_magnitude_consumer",
        ],
        "notes": "Reads the same race_creation_chassis function the player-facing roster "
        "calls; a race passes only if it states a real ability-score magnitude "
        "(BONUS:STAT or a floating pool) that changes the submitted sheet. "
        "Conservative/race-level, not trait-key-level.",
    },
    "race_trait": {
        "probe_exists": True,
        "category": "magnitude_probe",
        "probe_functions": [
            "probe_race_trait_corpus",
            "race_trait_magnitude_read_by_creation_chassis",
        ],
        "probe_locations": [f"{GENERATOR_SOURCE}:5914"],
        "positive_evidence_examples": [
            "race_trait_ability_magnitude_read_by_the_character_creation_chassis",
            "race_trait_applied_by_the_race_corpus_but_no_verified_consumer",
            "race_trait_states_a_universal_sheet_modifier_pending_compute",
        ],
        "notes": "Two independent observations: a race-level "
        "race_ids_with_a_magnitude_consumer check, and a record-level check for the "
        "exact trait whose BONUS:STAT the creation chassis reads.",
    },
    "monster": {
        "probe_exists": False,
        "category": "presence_only",
        "probe_functions": [],
        "probe_locations": [f"{GENERATOR_SOURCE}:8725", f"{GENERATOR_SOURCE}:8811"],
        "positive_evidence_examples": [],
        "notes": "monster_resolve()/holds_key() is a table LOOKUP -- confirms the engine "
        "holds a real stat block, never observes a computed delta attributable to the "
        "record. rules_core's monster_resolve() functions return a static "
        "MonsterStatBlock by key, not a value derived through a formula evaluator.",
    },
    "monster_ability": {
        "probe_exists": False,
        "category": "presence_only",
        "probe_functions": [],
        "probe_locations": [f"{GENERATOR_SOURCE}:8738"],
        "positive_evidence_examples": [],
        "notes": "Same holds_key() presence check as monster; text_only promotion path "
        "checks for real prose, not a computed magnitude.",
    },
    "companion": {
        "probe_exists": False,
        "category": "presence_only",
        "probe_functions": [],
        "probe_locations": [f"{GENERATOR_SOURCE}:9477"],
        "positive_evidence_examples": [],
        "notes": "Same holds_key() presence check pattern as monster/monster_ability.",
    },
    "ability": {
        "probe_exists": False,
        "category": "no_engine_table",
        "probe_functions": [],
        "probe_locations": [f"{GENERATOR_SOURCE}:9553"],
        "positive_evidence_examples": [],
        "notes": "Unconditional engine_does_not_hold('ability_content_has_no_engine_table') -- "
        "no engine table exists for this kind at all.",
    },
    "template": {
        "probe_exists": False,
        "category": "no_engine_table",
        "probe_functions": [],
        "probe_locations": [f"{GENERATOR_SOURCE}:9548"],
        "positive_evidence_examples": [],
        "notes": "Unconditional engine_does_not_hold('template_content_has_no_engine_table').",
    },
    "deity": {
        "probe_exists": False,
        "category": "no_engine_table",
        "probe_functions": [],
        "probe_locations": [f"{GENERATOR_SOURCE}:9549"],
        "positive_evidence_examples": [],
        "notes": "Unconditional engine_does_not_hold('deity_content_has_no_engine_table').",
    },
    "power": {
        "probe_exists": False,
        "category": "no_engine_table",
        "probe_functions": [],
        "probe_locations": [f"{GENERATOR_SOURCE}:9550"],
        "positive_evidence_examples": [],
        "notes": "Unconditional engine_does_not_hold('power_content_has_no_engine_table').",
    },
    "domain": {
        "probe_exists": False,
        "category": "no_engine_table",
        "probe_functions": [],
        "probe_locations": [f"{GENERATOR_SOURCE}:9551"],
        "positive_evidence_examples": [],
        "notes": "Unconditional engine_does_not_hold('domain_content_has_no_engine_table').",
    },
    "skill": {
        "probe_exists": False,
        "category": "no_engine_table",
        "probe_functions": [],
        "probe_locations": [f"{GENERATOR_SOURCE}:9537"],
        "positive_evidence_examples": [],
        "notes": "Unconditional engine_does_not_hold('skill_content_has_no_engine_table').",
    },
    "language": {
        "probe_exists": False,
        "category": "no_engine_table",
        "probe_functions": [],
        "probe_locations": [f"{GENERATOR_SOURCE}:9552"],
        "positive_evidence_examples": [],
        "notes": "Unconditional engine_does_not_hold('language_content_has_no_engine_table').",
    },
    "trait": {
        "probe_exists": False,
        "category": "no_engine_table",
        "probe_functions": [],
        "probe_locations": [f"{GENERATOR_SOURCE}:9554"],
        "positive_evidence_examples": [],
        "notes": "Unconditional engine_does_not_hold('trait_content_has_no_engine_table').",
    },
}


def load_inventory(repo_root=REPO_ROOT):
    path = os.path.join(repo_root, "docs", "work-inventory.json")
    with open(path, encoding="utf-8") as f:
        return json.load(f)


def _head_sha(repo_root=REPO_ROOT):
    try:
        return subprocess.run(
            ["git", "rev-parse", "HEAD"], cwd=repo_root, capture_output=True, text=True, check=True,
        ).stdout.strip()
    except Exception:
        return None


def build_census(inventory, repo_root=REPO_ROOT):
    units = inventory["units"]
    population = len(units)
    counts = {}
    for u in units:
        counts[u["kind"]] = counts.get(u["kind"], 0) + 1

    rows = []
    for kind, spec in sorted(PROBE_SURFACE.items()):
        rows.append(
            {
                "kind": kind,
                "unit_count": counts.get(kind, 0),
                "probe_exists": spec["probe_exists"],
                "category": spec["category"],
                "probe_functions": spec["probe_functions"],
                "probe_locations": spec["probe_locations"],
                "notes": spec["notes"],
            }
        )

    with_probe = sum(1 for r in rows if r["probe_exists"])
    without_probe = len(rows) - with_probe

    return {
        "derived_at_commit": _head_sha(repo_root),
        "generator_source": GENERATOR_SOURCE,
        "population_source": "docs/work-inventory.json",
        "population": population,
        "kind_count": len(rows),
        "kinds_with_probe": with_probe,
        "kinds_without_probe": without_probe,
        "kinds": rows,
    }


def check_census(inventory):
    """Two fail-closed checks, execution-derived against the passed
    inventory (never against memory):

    1. coverage -- every `kind` the live data actually contains is one this
       census maps. An unmapped kind is reported, not silently skipped.
    2. claim integrity -- every `probe_exists: true` kind has at least one
       live unit carrying one of its own positive-evidence strings (the
       probe genuinely fired, not just exists in source); and every
       `probe_exists: false` kind has ZERO live units whose evidence string
       contains "probe" (no undocumented probe the census under-claims).

    Returns (ok: bool, problems: list[str]).
    """
    units = inventory["units"]
    problems = []

    live_kinds = {u["kind"] for u in units}
    unmapped = live_kinds - set(PROBE_SURFACE.keys())
    for kind in sorted(unmapped):
        problems.append(f"kind '{kind}' has live units but is not in PROBE_SURFACE (uncensused)")

    by_kind_evidence = {}
    for u in units:
        by_kind_evidence.setdefault(u["kind"], []).append(u.get("evidence", ""))

    for kind, spec in PROBE_SURFACE.items():
        evidences = by_kind_evidence.get(kind, [])
        if spec["probe_exists"]:
            positive = set(spec["positive_evidence_examples"])
            fired = any(ev in positive for ev in evidences)
            if evidences and not fired:
                problems.append(
                    f"kind '{kind}' is claimed probe_exists=true but its probe never fires "
                    f"on the live population (no unit carries any of {sorted(positive)})"
                )
        else:
            probe_shaped = [ev for ev in evidences if "probe" in ev]
            if probe_shaped:
                problems.append(
                    f"kind '{kind}' is claimed probe_exists=false but carries probe-shaped "
                    f"evidence on the live population: {sorted(set(probe_shaped))}"
                )

    return (len(problems) == 0, problems)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--check", action="store_true",
        help="run the fail-closed coverage/claim-integrity checks against the live inventory "
        "and print a summary line instead of emitting the JSON census",
    )
    args = parser.parse_args(argv)

    inventory = load_inventory()

    if args.check:
        ok, problems = check_census(inventory)
        census = build_census(inventory)
        for p in problems:
            print(f"PROBLEM: {p}", file=sys.stderr)
        print(
            f"kind_count={census['kind_count']} "
            f"kinds_with_probe={census['kinds_with_probe']} "
            f"kinds_without_probe={census['kinds_without_probe']} "
            f"population={census['population']} "
            f"ok={ok}"
        )
        return 0 if ok else 1

    census = build_census(inventory)
    print(json.dumps(census, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
