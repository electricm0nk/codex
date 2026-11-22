# SD-13 Core Class/Race Roster and Level-10 Progression Matrix Technical Requirements

## Purpose
This document defines the normative requirements for the first bounded breadth-expansion lane in Codex: the exact PF1 Core Rulebook core roster, the support-state taxonomy, the level-10 progression proof burdens, and the visibility rules that keep unsupported, partial, lossy, blocked, or unverified semantics from being mistaken for product support.

## Current-state grounding
- GE-06 proves only one accepted deterministic character path today: PF1 Core Rulebook Human Fighter level 1.
- `tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt` names exactly `race:human` plus `class:fighter:1` and no broader roster.
- `tests/ge06_pilot_total_saves.rs` explicitly claim-blocks `class:rogue:1` and `class:fighter:2` for the current bounded compute path.
- `tests/ge06_pilot_combat_baseline.rs` explicitly claim-blocks `class:fighter:2` for combat/defense totals under the current bounded compute path.
- The repo README still states plainly that Codex is not yet a general character builder or broad Pathfinder product.
- SD-11, SD-12, and SD-14 already establish adjacent tester, distribution, and persistence boundaries that SD-13 must not absorb.

## Requirement families

### 1. Boundary and scope requirements
- SD-13 MUST define breadth only for Pathfinder 1e Core Rulebook core races and core classes.
- SD-13 MUST remain a bounded breadth lane, not a generalized Pathfinder-support claim.
- SD-13 MUST remain single-class only through level 10 in this tranche.
- SD-13 MUST NOT imply multiclassing, prestige classes, archetypes, alternate racial traits, optional systems, or non-core books.
- SD-13 MUST preserve the distinction between:
  - GE-06 current pilot-proof truth
  - SD-11 tester-workbench and support-language truth
  - SD-12 distribution/update truth
  - SD-14 persistence/lifecycle truth
  - SD-13 breadth/progression truth

### 2. Exact roster identity requirements
The packet MUST name the exact core-race roster for this tranche:
1. Dwarf
2. Elf
3. Gnome
4. Half-Elf
5. Half-Orc
6. Halfling
7. Human

The packet MUST name the exact core-class roster for this tranche:
1. Barbarian
2. Bard
3. Cleric
4. Druid
5. Fighter
6. Monk
7. Paladin
8. Ranger
9. Rogue
10. Sorcerer
11. Wizard

The packet MUST treat any roster member outside those lists as out of scope unless a later authority surface expands the boundary.

### 3. Support-state taxonomy requirements
The packet MUST define a support-state taxonomy that keeps breadth claims falsifiable and visible.

At minimum, the taxonomy MUST include these states:
- `supported` — the named semantic dimension is proven at the required evidence floor and has no known missing semantics inside the bounded claim
- `partial` — some required semantics are proven, but one or more named required semantics remain incomplete and visible
- `lossy` — the path can be executed only by discarding, flattening, or approximating named semantics; `lossy` MUST NOT be counted as `supported`
- `blocked` — known missing semantics, explicit claim-blocking diagnostics, or known contradictory behavior prevent the claim
- `unverified` — the system has not yet produced direct evidence for the named dimension

The same-epic artifact `artifacts/core-roster-and-support-state-matrix.md` is authoritative for the matrix shape and seeded current-truth rows.

### 4. Matrix row and evidence requirements
The support-state matrix MUST record enough structured truth to avoid hand-wavy breadth claims.

Every matrix or ledger row MUST carry at least:
- row type or subject type
- subject identity (race, class, or named interaction seam)
- progression or semantic dimension under judgment
- support state
- highest evidence tier achieved
- grounding artifact or evidence reference
- known blockers or known lossiness when not `supported`
- next required uplift or owning future slice

The matrix MUST separate at least these row types:
- race-semantic rows
- class-progression rows
- named interaction rows where a race/class combination or cross-cutting rule seam matters materially

The matrix MUST NOT imply that all 7 x 11 race/class combinations are equally proven merely because every race and every class appears in the same document.

### 5. Breadth-claim gating requirements
A bounded breadth claim MUST require both:
- the relevant race-semantic row(s) to be in an acceptable state for the claimed surface
- the relevant class-progression row(s) to be in an acceptable state for the claimed surface

Where a specific interaction seam materially changes behavior, the claim MUST also require the named interaction row to be non-blocked and visible.

A class or race MUST NOT be reported as fully supported merely because:
- it parses
- it loads into a canonical model
- it appears in a picker or dropdown
- a shallow summary screen can name it

### 6. Race-semantic requirements
A race support claim MUST classify at least these semantic families when they affect the bounded character-builder surface:
- identity and ruleset provenance
- ability-score modifiers or bonuses
- size, speed, and movement-relevant baseline posture
- senses or visibility-affecting traits that matter to supported outputs
- racial bonus feats, skill modifiers, or derived-stat modifiers
- prerequisite, feat, or class-feature interactions triggered by the race
- other core racial traits that materially affect bounded level-10 support claims

If a race semantic family is outside the bounded visible product slice, the packet MUST state that explicitly rather than silently omitting it.

### 7. Class-progression requirements
A class support claim MUST classify at least these progression families through level 10 when they matter to the class:
- class identity and level progression posture
- base attack bonus progression where applicable
- saving-throw progression
- hit-die and derived durability obligations where they are surfaced by the supported slice
- mandatory class-feature progression through level 10
- class-granted feat, talent, rage-power, domain, bloodline, style, school, or similar choice surfaces where the class requires them
- prerequisite and invalid-choice blocking behavior
- skill-rank and class-skill effects where the class or level progression requires them
- explanation/diagnostic visibility for claimed supported outputs
- spellcasting progression where the class requires it

The same-epic artifact `artifacts/level-10-progression-validation-contract.md` is authoritative for the concrete proof burden table.

### 8. Spellcasting-specific requirements
The packet MUST explicitly distinguish spellcasting-heavy or spellcasting-hybrid classes from non-caster classes.

For Bard, Cleric, Druid, Paladin, Ranger, Sorcerer, and Wizard, a truthful support claim MUST classify at least:
- spell list/source lineage posture
- spells known versus prepared posture where applicable
- slot/per-day progression where applicable
- class-specific spellcasting choices such as domains, schools, bloodlines, or equivalent bounded core surfaces where the class requires them
- spellcasting-dependent class features that materially affect the level-10 claim

A spellcasting class MUST NOT be counted as `supported` if the packet can only prove class selection or non-spell chassis behavior.

### 9. Level-10 progression requirements
A level-10 claim MUST mean more than successful load at some lower level.

For every class, the packet MUST define a level-10 proof burden that names:
- which levels or milestone levels matter for that class
- which class features or feature families must be represented or classified by those levels
- which progression dimensions must be executable versus which may remain explicitly documentary or deferred
- what visible support-state consequence applies when a required milestone remains unsupported, lossy, or unverified

The packet MUST allow partial truth. A class may be `partial`, `lossy`, `blocked`, or `unverified` for level-10 support rather than being forced into a fake binary.

### 10. Prerequisite, feat, equipment, skill, and derived-stat requirements
The packet MUST define cross-cutting progression dimensions that later execution slices must not ignore:
- feat and prerequisite gating
- class-granted choices and bonus-choice gating
- selected-skill and class-skill pressure where level progression depends on it
- race/class interactions that change feat or prerequisite availability
- derived combat or defense values where the class claim depends on them
- explanation and diagnostic posture for blocked or unsupported choices

These cross-cutting dimensions MUST remain separate from the simpler “can select the class/race” question.

### 11. Unsupported, partial, lossy, blocked, and unverified visibility requirements
The packet MUST define a ledger contract for visible debt.

At minimum, the ledger MUST preserve:
- the subject and semantic family
- why the current state is partial, lossy, blocked, or unverified
- what evidence currently grounds that classification
- what next slice or action would upgrade the state
- whether the condition must remain visible to testers, operators, or both

Unsupported-depth debt MUST NOT live only in chat memory, private operator knowledge, or ad hoc UI text.

The same-epic artifact `artifacts/unsupported-partial-lossy-and-unverified-semantics-ledger.md` is authoritative for the ledger shape and seeded current-state entries.

### 12. Adjacent-lane boundary requirements
The packet MUST explicitly preserve these seams:
- SD-11 owns tester-facing workbench structure, issue flows, and support wording
- SD-12 owns branch/channel/distribution/update truth
- SD-14 owns saved-state continuity and upgrade-survival truth
- SD-13 owns breadth and progression truth only

Later implementation MAY feed SD-11, SD-12, or SD-14 with SD-13 outputs, but SD-13 MUST NOT rewrite those lanes by implication.

The same-epic artifact `artifacts/adjacent-lane-boundary-and-breadth-claim-contract.md` is authoritative for this seam map.

### 13. Verification and proof obligations for later execution lanes
A future execution handoff derived from this packet MUST name exact commands and fixtures, but it may not weaken these proof classes:
- roster matrix seeding and current-state evidence proof
- race-semantic classification proof
- class-level progression proof through the relevant level-10 milestones
- spellcasting-specific proof for spell-bearing classes
- prerequisite/feat/skill/derived-stat blocking proof
- visible unsupported/lossy/deferred debt proof
- breadth-claim wording or evidence-refresh proof where testers or operators consume the matrix

### 14. Non-goals
This packet does not authorize:
- repo implementation code
- non-core Pathfinder breadth
- multiclassing or prestige-class semantics
- UI polish standing in for breadth truth
- distribution/update or persistence behavior by implication
