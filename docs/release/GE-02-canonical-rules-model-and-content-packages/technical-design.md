---
title: GE-02 Technical Design
stc_id: STC-CODEX-GE-02
artifact_type: technical-design
status: accepted
scope: programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages
source_stc: ./README.md
---

# GE-02 Technical Design

## Purpose
This design operationalizes the GE-02 source STC without collapsing it into implementation code. It defines the canonical-model planning structure Codex should use to represent source packages, object identities, rules effects, prerequisites, formulas, choice sets, provenance, diagnostics, and the boundary between authored content and compiled runtime IR.

## Design posture
- architecture style: `semantic content model with explicit authoring -> validation -> compiled IR boundaries`
- migration posture: `pilot-first, no LST syntax cloning, provenance-preserving`
- package posture: `versioned source packages with stable IDs and explicit dependency/include semantics`
- diagnostics posture: `unresolved behavior remains visible; no silent downgrade`

## Context and constraints
- GE-01 is accepted as the upstream pilot-source truth surface for inventory, token taxonomy, conversion matrix, unsupported-token ledger, and oracle discovery.
- GE-02 owns the canonical target-model problem; GE-03 importer work must aim at this surface rather than defining its own target model.
- Codex is a new project. PCGen is heritage corpus and oracle substrate, not runtime architecture or canonical schema.
- This source STC grants no implementation authority in `/home/ubuntu/workspace/repos/codex`.
- The pilot boundary remains PF1 Core Rulebook Human Fighter level 1 unless a higher-order decision record changes it.

## Proposed system shape
Treat GE-02 as a model design composed of nine documentary entities:

1. **SourcePackage** — versioned package identity, source/book metadata, include/dependency graph, and validation state.
2. **StableId** — deterministic identity convention for objects and relations across import reruns and native authoring.
3. **GameObject** — typed semantic objects such as race, class, feat, skill, equipment, proficiency, ability score, and save.
4. **Relation / Selector** — typed links, categories, and selector semantics that replace overloaded legacy `TYPE` usage.
5. **Effect / Grant** — structured representation of automatic grants, numeric modifiers, trait grants/removals, class features, and equipment effects.
6. **Prerequisite** — structured predicate model for gates and eligibility checks.
7. **Formula / Value Expression** — representation for formulas and derived values, including unresolved/deferred expressions.
8. **ChoiceSet** — selectable/repeatable option model for feats, proficiencies, and future character choices.
9. **Provenance / Diagnostic / IR Boundary** — source lineage, validation evidence, unsupported behavior, and compiled runtime cache separation.

These entities are requirement homes. Later code may implement them differently, but it must preserve the semantic boundaries unless a later accepted decision surface supersedes them.

## Data flow
1. **Package authoring or import projection**
   - authoring files or GE-03 importer outputs produce package-level canonical source content
2. **Stable identity assignment**
   - package and object IDs are created or validated deterministically
3. **Semantic object construction**
   - race, class, feat, skill, equipment, ability score, save, proficiency, and related objects are represented as canonical model records
4. **Rule attachment**
   - effects, grants, prerequisites, formulas, and choice sets attach to objects as structured rule semantics
5. **Validation and diagnostics**
   - unresolved references, unsupported constructs, partial/lossy imports, and invalid formulas become explicit diagnostics
6. **Compiled runtime IR generation**
   - validated source content can later compile into an optimized runtime shape without becoming the only source of truth
7. **Engine and explanation consumption**
   - GE-04 consumes validated model/IR outputs; GE-05 can compare selected results against oracle surfaces

## Component boundaries

### SourcePackage
Responsibilities:
- represent package identity, game system, source/book, version/revision, dependency/include graph, and validation state
- preserve imported PCC lineage as package composition semantics

Inputs:
- GE-01 corpus inventory
- GE-01 conversion-matrix package/include rows
- future native-authored package manifests

Outputs:
- package records and dependency graph suitable for validation and compilation

Must not own:
- parser mechanics
- runtime rule execution

### StableId
Responsibilities:
- provide deterministic and reviewable identifiers for package and object records
- support imported PCGen names as aliases or provenance without making filesystem paths the canonical ID

Inputs:
- package identity
- object kind
- source names and aliases

Outputs:
- stable object IDs and relation target IDs

Must not own:
- display names or localization policy

### GameObject
Responsibilities:
- provide semantic homes for pilot object kinds
- separate object identity/fields from rules behavior attached through effects, prerequisites, formulas, and choices

Pilot object homes:
- source package
- race and race traits
- class and class features
- feat
- skill
- equipment
- proficiency
- ability score
- save

Must not own:
- raw LST token syntax as canonical data

### Relation / Selector
Responsibilities:
- represent type/category/group semantics and selector inputs without blindly preserving legacy `TYPE` as final taxonomy
- support class-skill selectors, proficiency groups, equipment categories, and future rule filters

Must not own:
- arbitrary string-bucket behavior without validation

### Effect / Grant
Responsibilities:
- represent automatic grants, class-feature carriers, trait grants/removals, numeric modifiers, equipment modifiers, and conditionally suppressed behavior
- retain source owner, target, trigger/context, and provenance

Inputs:
- GE-01 rows for `ABILITY`, `AUTO`, `BONUS`, `CSKILL`, trait carrier, and equipment effects

Outputs:
- structured rule effects consumable by validation and later GE-04 engine work

Must not own:
- final stacking/evaluation semantics beyond explicit requirements and open questions

### Prerequisite
Responsibilities:
- preserve predicate structure for feats, proficiencies, archetype suppression, and eligibility checks
- produce diagnostics for unsupported or unevaluated prerequisite forms

Must not own:
- expression-language selection by itself

### Formula / Value Expression
Responsibilities:
- preserve formula and variable structure for BAB, saves, ability scores, skill points, equipment values, and derived statistics
- allow unresolved/deferred formulas to remain visible while preserving source text and target value intent

Must not own:
- full runtime evaluator implementation

### ChoiceSet
Responsibilities:
- represent option sources, allowed targets/selectors, cardinality, repeatability, and selected-value provenance
- preserve `CHOOSE + MULT` debt without flattening it to a boolean

Must not own:
- UI selection workflow

### Provenance / Diagnostic / IR Boundary
Responsibilities:
- preserve source lineage from package/source to target object/effect/field
- record unsupported, lossy, partial, deferred, or intentionally ignored behavior
- define the boundary between authoring source and compiled runtime IR

Must not own:
- final CLI/report UX or oracle harness execution

## Documentary schema skeletons
These sketches are not final production schemas. They are requirement skeletons that prove the model homes are coherent.

```yaml
source_package:
  id: pf1.crb
  game_system: pathfinder_1e
  title: Core Rulebook
  version: unresolved
  dependencies: []
  imported_from:
    pcgen_pcc: /pathfinder/paizo/roleplaying_game/core_rulebook/core_rulebook.pcc
  validation_state: pending
```

```yaml
race:
  id: pf1.crb.race.human
  package: pf1.crb
  display_name: Human
  traits:
    - ref: pf1.crb.race_trait.human.default_traits
  effects: []
  provenance:
    source_file: human_races.lst
    source_entry: Human
```

```yaml
class:
  id: pf1.crb.class.fighter
  package: pf1.crb
  display_name: Fighter
  levels:
    1:
      effects:
        - ref: pf1.crb.effect.fighter.level1.bab
        - ref: pf1.crb.effect.fighter.level1.saves
      grants:
        - ref: pf1.crb.class_feature.fighter.proficiencies
  provenance:
    source_file: cr_classes.lst
    source_entry: Fighter
```

```yaml
effect:
  id: pf1.crb.effect.fighter.weapon_proficiency_grant
  owner: pf1.crb.class_feature.fighter.proficiencies
  kind: grant
  target_kind: proficiency
  target_selector: martial_weapons
  condition: standard_fighter_unless_archetype_suppressed
  provenance:
    source_file: cr_abilities_class.lst
    source_span: lines 236-242
  diagnostics: []
```

```yaml
prerequisite:
  id: pf1.crb.prereq.martial_weapon_proficiency
  structure: unresolved_structured_predicate
  source_text: preserve legacy predicate until converted
  provenance:
    source_file: cr_feats.lst
    source_span: line 120 or related feat predicate lines
  diagnostic_policy: emit_unsupported_if_not_evaluable
```

```yaml
formula:
  id: pf1.crb.formula.fighter.bab.level_progression
  target: base_attack_bonus
  expression: preserve source formula until converted
  variables:
    - fighter_level
  provenance:
    source_file: cr_classes.lst
    source_span: lines 139-141
  execution_state: deferred_to_ge04
```

```yaml
choice_set:
  id: pf1.crb.choice.martial_weapon_proficiency
  owner: pf1.crb.feat.martial_weapon_proficiency
  repeatable: true
  selector: martial_weapon_proficiency_catalog
  cardinality: one_per_selection
  provenance:
    source_file: cr_feats.lst
    source_span: line 120
```

```yaml
source_map_entry:
  source_package: pf1.crb
  source_file: cr_classes.lst
  source_span: lines 139-143
  legacy_construct: CLASS Fighter / BONUS / DEFINE
  canonical_target: pf1.crb.class.fighter
  disposition: deferred
  lossiness: unknown-risk
```

```yaml
diagnostic:
  id: diag.unresolved_formula.fighter_bab
  severity: blocking_for_engine_execution
  linked_source_map: pf1.crb.class.fighter#cr_classes.lst:139-141
  message: Formula model captured, evaluator semantics deferred to GE-04.
```

## GE-01 input mapping
The high-pressure inputs from GE-01 map into GE-02 as follows:

| GE-01 pressure | GE-02 model home |
|---|---|
| PCC include directives | `SourcePackage` manifest and dependency graph |
| `RACE` Human entry | `Race` plus `RaceTrait` composition |
| `CLASS` Fighter entry | `Class`, `ClassFeature`, `Formula`, and `Effect` homes |
| `SKILL` entries and `CSKILL` carriers | `Skill`, `Relation/Selector`, and `Effect` homes |
| `EQUIPMENT`, `WEAPONPROF`, `ARMORPROF`, `SHIELDPROF` | `Equipment`, `Proficiency`, selectors, and effects |
| `ABILITY`, `AUTO`, `BONUS` | `Effect/Grant` model |
| `PRE*`, `PREMULT`, `PREPROFWITH*` | `Prerequisite` model |
| `DEFINE`, `VAR`, formula-bearing `BONUS` | `Formula / Value Expression` model |
| `CHOOSE + MULT` | `ChoiceSet` model |
| source paths, lines, spans, matrix dispositions | `ProvenanceRecord` / `SourceMapEntry` and diagnostics |

## External dependencies and references
- `programs/codex/plans/spec-domains/GE-02-canonical-rules-model-and-content-packages.md` — strategic source artifact
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/README.md` — accepted upstream closure state
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv` — source-to-target pressure map
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/artifacts/unsupported-token-ledger.csv` — unresolved modeling debt
- `programs/codex/requirements/GE-01-legacy-corpus-and-conversion-matrix/references/oracle-surface-inventory.md` — source truth and future comparison surfaces
- `programs/codex/plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md` — first proof target
- `programs/codex/research/codex-reference-architecture-2026-06-17.md` — package/model/provenance architecture context

## Design decisions already fixed
- GE-02 is the canonical target-model authority for the pilot boundary.
- The canonical model must not mirror LST syntax directly.
- Content packages and object IDs must be stable, source-aware, and version-aware.
- Effects, prerequisites, formulas, and choices are first-class structures, not prose notes.
- Imported content must preserve source lineage and diagnostic posture.
- Authored source content and compiled runtime IR are separate authority surfaces.

## Deferred design decisions
- exact stable ID syntax and namespace delimiter
- exact authoring file format and schema language
- exact expression/prerequisite/formula technology
- exact stacking and effect resolution rules
- exact selector/type taxonomy for overloaded legacy `TYPE` surfaces
- exact compiled IR serialization and cache invalidation strategy
- exact production validation CLI/API shape

## Failure modes and observability
- **Failure mode:** Codex clones LST token syntax into a new file format.
  - **Required signal:** each target concept has semantic object/effect/prerequisite/formula/choice homes instead of raw-token fields as the model.
- **Failure mode:** source lineage collapses during canonical projection.
  - **Required signal:** every imported object/effect/diagnostic can point back to source package, file, entry, and span where available.
- **Failure mode:** formulas and prerequisites become prose-only placeholders.
  - **Required signal:** unresolved formulas/prerequisites remain structured records with diagnostics and GE-04/GE-03 ownership.
- **Failure mode:** source package and runtime IR are conflated.
  - **Required signal:** authoring source and compiled IR have different roles and authority in requirements.
- **Failure mode:** GE-02 becomes code authority.
  - **Required signal:** no implementation handoff exists until branch/worktree/write-scope and bounded slice are explicit.

## Verification implications
`acceptance-and-verification.md` must prove that this design yields:
- a complete GE-02 source-STC control bundle
- a concrete generated documentary output set under `artifacts/`
- explicit model homes for all pilot-critical GE-01 target concepts in `artifacts/canonical-model-specification.md`
- explicit GE-01 input usage
- a conceptual Human Fighter pilot representation without LST syntax cloning in `artifacts/pilot-object-examples.yaml`
- visible unresolved debt for high-risk formulas, prerequisites, choices, and trait/proficiency indirection
- no counterfeit code authority

## Change constraints
- Do not resolve deferred expression/schema/IR decisions casually inside prose.
- Do not broaden scope beyond the pilot without an accepted decision record.
- Do not collapse requirements into design or design into implementation handoff.
- Do not claim parity, import success, or engine readiness from planning artifacts.
- Do not modify PCGen or Codex implementation code from this source STC.
