---
title: GE-06 Final Deterministic Pilot Input Contract
stc_id: STC-CODEX-GE-06
artifact_type: documentary-readiness-closure
status: accepted
scope: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts
source_stc: ../README.md
route_class: documentary-only
owner: Todd Hintzmann
authority_surface: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter
created: 2026-06-21
related:
  - ../../../plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md
  - ../../GE-05-oracle-validation-and-parity-harness/technical-design.md
  - ../../GE-05-oracle-validation-and-parity-harness/technical-requirements.md
  - ../../GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e1-f1-oracle-route-inventory-2026-06-20.md
  - /home/ubuntu/workspace/repos/codex/src/oracle_validation/golden_fixture.rs
  - /home/ubuntu/workspace/repos/codex/tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt
---

# GE-06 Final Deterministic Pilot Input Contract

## Core problem
GE-06 could not move from planning posture to bounded execution-readiness while the PF1 Human Fighter level 1 pilot still had unresolved Human ability bonus, feat-slot, skill-rank, equipment, active-state, and export-boundary choices.

## Verdict
The documentary pass is complete.

The first GE-06 pilot input contract is now deterministic. It still does **not** invent final computed values or parity results. Those belong to later GE-04/GE-05-backed runtime evidence. This artifact closes only the character-input ambiguity required before a code-authorizing handoff can be derived.

## Merge and upstream evidence posture
Observed post-merge state on 2026-06-21:

- `/home/ubuntu/workspace/repos/codex` `origin/develop` head: `a2c7e88`
- GE-05 schema commit `acf6ad4` is an ancestor of `origin/develop`
- merged GE-05 fixture schema exists at `/home/ubuntu/workspace/repos/codex/src/oracle_validation/golden_fixture.rs`
- merged GE-05 fixture instance exists at `/home/ubuntu/workspace/repos/codex/tests/fixtures/oracle_validation/pf1_human_fighter_level1_golden_fixture.txt`

The merged GE-05 fixture deliberately records these fields as provisional/unresolved:

```text
provisional=human_ability_bonus:+2 Strength
provisional=final_equipment_loadout:unresolved
provisional=skill_allocation:unresolved
provisional=additional_feat_slot_closure:unresolved
```

GE-06 may now close those inputs for its own pilot contract without mutating GE-05's historical fixture evidence.

## Final deterministic character identity

```yaml
case_id: pf1-crb-human-fighter-level1
case_version: 1
source_system: pathfinder-1e
source_package: core_rulebook
source_campaign: Core Rulebook
source_game_mode: Pathfinder_RPG
race: Human
class_levels:
  Fighter: 1
alignment: unspecified_for_first_slice
size: Medium
base_speed: 30
```

## Final ability-score contract

The pilot uses the existing charter vector as final post-racial ability scores:

```yaml
ability_scores:
  str: 16
  dex: 14
  con: 14
  int: 10
  wis: 12
  cha: 8
human_ability_bonus:
  target: str
  bonus: 2
```

Rationale:

- GE-05 already preserved `+2 Strength` as the only provisional ability assumption.
- Human source data grants one ability bonus pool choice.
- STR 16 satisfies the grounded `power_attack` prerequisite surface and supports the melee pilot's required output categories.

Grounding evidence:

- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/human_abilities_race.lst:18` — `Human ~ Ability Scores` grants `BONUS:ABILITYPOOL|Ability Bonus|1`.
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_feats.lst:134` — `Power Attack` requires `PRETOTALAB:1` and `PREVARGTEQ:PreStatScore_STR,13`.

## Final feat-slot contract

The level-1 Human Fighter has three selected feat slots for this pilot:

```yaml
feats:
  level_1_character_feat:
    feat: Power Attack
    key: power_attack
  human_bonus_feat:
    feat: Dodge
    key: dodge
  fighter_bonus_feat:
    feat: Weapon Focus
    key: weapon_focus
    selection: Longsword
```

Slot rationale:

- `Power Attack` remains the named charter feat and occupies the ordinary level-1 character feat slot.
- Human bonus feat is closed with `Dodge`, a Core Rulebook combat feat whose DEX prerequisite is satisfied by DEX 14 and whose effect touches an already-required output family: armor class.
- Fighter bonus feat is closed with `Weapon Focus (Longsword)`, a Core Rulebook combat feat that exercises the weapon-proficiency choice surface while remaining inside the representative Longsword equipment path.

Grounding evidence:

- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/human_races.lst:6` — Human has `STARTFEATS:1`.
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/human_abilities_race.lst:21` — Human `Bonus Feat` grants `BONUS:ABILITYPOOL|FEAT|1`.
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst:257` — Fighter receives `Fighter ~ Bonus Feats` at level 1.
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_feats.lst:53` — `Dodge` is `TYPE:Combat` and requires DEX 13 through the recorded prerequisite surface.
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_feats.lst:134` — `Power Attack` is `TYPE:Combat.AttackOption` and requires BAB 1 plus STR 13.
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_feats.lst:184` — `Weapon Focus` is `TYPE:Combat.WeaponFocus`, has `PRETOTALAB:1`, and uses `CHOOSE:WEAPONPROFICIENCY|PC`.
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst:139` — Fighter level 1 has full BAB progression sufficient for `PRETOTALAB:1`.

## Final skill-rank contract

The pilot's first deterministic skill allocation is:

```yaml
skill_ranks:
  Climb: 1
  Intimidate: 1
  Swim: 1
skill_rank_source:
  fighter_base_ranks: 2
  human_skilled_bonus_rank: 1
  intelligence_modifier: 0
  favored_class_skill_bonus: 0
```

Favored-class selection is closed separately:

```yaml
favored_class:
  class: Fighter
  bonus_selection: hit_point
  included_in_first_slice_required_outputs: false
```

Rationale:

- Fighter contributes two base skill ranks per level.
- Human Skilled contributes one additional skill rank at 1st level.
- INT 10 contributes no additional rank modifier.
- The favored-class bonus is fixed to hit point, not skill rank, to prevent silent expansion of the skill allocation beyond the three ranks above.
- Climb and Swim exercise STR-based armor-check skills, which are useful for testing armor penalties and equipment effects without adding non-Core or multi-scenario breadth.
- Intimidate exercises a class skill with CHA dependence and no armor-check complication.

Grounding evidence:

- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_classes.lst:141` — Fighter uses `STARTSKILLPTS:FighterSkillPoints` and `BONUS:VAR|FighterSkillPoints|2`.
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_essentials/races/human/human_abilities_race.lst:22` — Human Skilled grants `BONUS:SKILLPOINTS|NUMBER|1`.
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_skills.lst:10` — Climb is STR-based and armor-check affected.
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_skills.lst:42` — Intimidate is CHA-based.
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_skills.lst:102` — Swim is STR-based and armor-check affected.

## Final equipment and active-state contract

The pilot's first deterministic equipment loadout is:

```yaml
equipment:
  worn_armor:
    item: Chain Shirt
    key: Chain Shirt
    state: equipped_worn_active
  primary_weapon:
    item: Longsword
    key: Longsword
    state: equipped_primary_active
  shield:
    item: none
    state: absent
  additional_items:
    state: absent_for_first_slice
active_states:
  power_attack:
    selected: true
    active_for_baseline_outputs: false
  dodge:
    selected: true
    active_for_baseline_outputs: true
  weapon_focus_longsword:
    selected: true
    active_for_longsword_outputs: true
encumbrance:
  coin_weight_and_inventory_breadth: out_of_scope_for_first_slice
```

Rationale:

- Chain Shirt and Longsword are already representative GE-01/GE-06 anchors.
- No shield prevents the first slice from confusing one-handed weapon, shield AC, shield proficiency, and hand-occupancy semantics.
- Power Attack is selected because the charter requires it, but it is inactive for baseline comparison outputs. If a later parity dimension needs Power Attack active, that is a separate scenario/dimension, not implicit baseline state.

Grounding evidence:

- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst:40` — Chain Shirt base armor entry includes light armor type, AC bonus, armor-check penalty, max DEX, and spell failure properties.
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst:53` — Chain Shirt visible equipment copy.
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_class.lst:237-241` — Fighter gains simple/martial weapon proficiency, heavy/medium/light armor proficiency, and shield proficiency at level 1.
- `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/core_rulebook/cr_feats.lst:184` — Weapon Focus selects a weapon proficiency known to the PC.

## Required output boundary for the first bounded implementation handoff

A later code-authorizing handoff may target a headless fixture/input contract first. It must not jump directly to broad UI or broad parity.

The first bounded implementation artifact should be able to represent and validate this input contract and emit either evidence or explicit blockers for these output families:

```yaml
required_output_families:
  - character_identity
  - final_ability_scores
  - selected_feats_and_choice_slots
  - selected_skill_ranks
  - selected_equipment_and_active_state
  - base_attack_bonus
  - fortitude_save
  - reflex_save
  - will_save
  - baseline_longsword_melee_attack_bonus
  - baseline_armor_class
  - armor_check_penalty_effects_on_selected_skills
  - diagnostics
  - provenance_or_source_refs
  - explanation_refs
  - oracle_dimension_status
```

The first implementation handoff must explicitly say which of these are merely represented, which are computed, and which are oracle-checked.

## Export-summary boundary

The first GE-06 export/product boundary is one headless summary receipt, not a character sheet and not a UI screen.

Required receipt sections:

1. case identity and source package identity
2. final deterministic input selections from this artifact
3. selected computed outputs or explicit blockers
4. explanation/provenance references for each computed output
5. diagnostics and known gaps
6. GE-05 fixture/schema reference and oracle-dimension state
7. claim-tier table using Codex quality-gate terminology

No broad export-sheet compatibility is authorized by this artifact.

## Non-goals

This artifact does not authorize:

- broad Pathfinder support
- full PCGen export parity
- final numeric expected values without runtime evidence
- UI implementation
- changes to `/home/ubuntu/workspace/repos/pcgen`
- mutation of the merged GE-05 historical fixture to hide its provisional status
- any implementation branch that is not started from current `develop` or an explicitly recorded dependency branch

## Readiness closure

The following GE-06 documentary blockers are closed by this artifact:

| Prior blocker | Closure |
|---|---|
| Human `+2` ability score target | Closed as STR. |
| Additional Human/Fighter feat entitlement ambiguity | Closed as Power Attack, Dodge, Weapon Focus (Longsword). |
| Skill-rank allocation | Closed as Climb 1, Intimidate 1, Swim 1, with favored-class bonus assigned to HP rather than skill rank. |
| Equipment loadout | Closed as Chain Shirt worn, Longsword primary, no shield, no other first-slice inventory. |
| Power Attack active-state ambiguity | Closed as selected but inactive for baseline outputs. |
| Export-summary boundary | Closed as one headless summary receipt. |

The final state is:

```text
GE-06 documentary selection debt: CLOSED
GE-06 numeric expected values: NOT CLOSED, require runtime evidence
GE-06 broad UI readiness: NOT AUTHORIZED
GE-06 next permissible handoff: bounded headless implementation-readiness handoff derived from this contract
```

## Verification checklist for the next handoff author

Before generating any code-authorizing GE-06 handoff, verify:

- [ ] branch base is current `develop`, or any deviation is explicitly justified
- [ ] write scope is limited to exact Codex repo paths
- [ ] the handoff consumes this artifact by exact path
- [ ] the handoff consumes the merged GE-05 fixture schema by exact path
- [ ] no final expected numeric values are invented in the prompt
- [ ] all output claims are classified as represented, computed, oracle-checked, product-visible, blocked, or known-gap
- [ ] UI work is excluded unless a later GE-07/source-STC or explicit spike charter authorizes it

## Final rule

The lesser approach would continue researching because the old fixture said “unresolved.” That is now noise. The unresolved items were input choices, not unknowable facts. They are closed here. What remains unresolved is runtime proof, and runtime proof belongs to the next bounded execution slice.
