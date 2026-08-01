---
title: GE-02 Epic Breakdown
stc_id: STC-CODEX-GE-02
artifact_type: epic-breakdown
status: accepted
scope: programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages
source_stc: ./README.md
---

# GE-02 Epic Breakdown

## Purpose
Break the GE-02 source STC into downstream epics and feature seeds without authorizing code directly from this planning bundle.

## Routing rule
Each downstream epic below still requires a route-appropriate handoff before implementation. A future coding handoff must name exact repo paths, branch/worktree policy, allowed write scope, required reads, non-goals, tests, and verification commands.

## GE02-E1 — Source package manifest and dependency graph
Objective: Define and later implement package identity, package metadata, dependency/include semantics, versioning, and validation state.

Inputs:
- GE-01 corpus inventory package/include rows
- GE-01 conversion matrix row for PCC include directives
- GE-02 `TR-02-004`

Expected downstream outputs:
- package manifest schema decision or implementation handoff
- source-package validation rules
- package dependency graph representation

Non-goals:
- parser implementation
- public package registry

## GE02-E2 — Stable ID convention and object namespace
Objective: Define deterministic object IDs that support imported PCGen content and future native-authored packages.

Inputs:
- GE-02 `TR-02-005`
- pilot object homes from `TR-02-006`

Expected downstream outputs:
- stable ID convention decision surface
- implementation handoff for ID helpers/validators when code-authorized

Non-goals:
- broad localization/display-name system

## GE02-E3 — Pilot object schemas
Objective: Define pilot canonical object schemas for race, race traits, class, class features, feat, skill, equipment, proficiency, ability score, and saves.

Inputs:
- GE-01 taxonomy and conversion matrix rows for `RACE`, `CLASS`, `SKILL`, `EQUIPMENT`, `WEAPONPROF / ARMORPROF / SHIELDPROF`, ability scores, and saves
- GE-02 `TR-02-006` and `TR-02-015`

Expected downstream outputs:
- schema examples promoted from documentary skeletons into accepted schema artifacts or implementation handoffs
- fixture cases for Human, Fighter, representative skills, and representative equipment

Non-goals:
- full Pathfinder object coverage
- final engine evaluation

## GE02-E4 — Effect and grant model
Objective: Define structured grants and effects for proficiencies, class features, race traits, numeric modifiers, and equipment effects.

Inputs:
- GE-01 rows for `ABILITY`, `AUTO`, `BONUS`, `CSKILL`, trait carriers, proficiency grants, and equipment modifiers
- GE-01 ledger entries on Fighter proficiency suppression, Human trait replacement, and class-skill carriers
- GE-02 `TR-02-007`

Expected downstream outputs:
- effect/grant schema or implementation handoff
- diagnostics for unresolved condition/suppression semantics

Non-goals:
- final stacking and resolution order unless a handoff explicitly targets it

## GE02-E5 — Prerequisite, formula, and value-expression model
Objective: Define structured representation for prerequisite predicates and formulas without selecting a final expression technology prematurely.

Inputs:
- GE-01 ledger entries for `PREMULT`, `PREPROFWITH*`, formula-bearing `BONUS / DEFINE / VAR`, base-stat formulas, and skill-point variable chains
- GE-02 `TR-02-008` and `TR-02-009`

Expected downstream outputs:
- expression-model decision criteria
- prerequisite/formula schema or parser/validation handoff
- explicit diagnostics for unevaluable or deferred expressions

Non-goals:
- full runtime evaluator implementation; that routes into GE-04

## GE02-E6 — Choice-set, selector, and type taxonomy model
Objective: Define choice sets, repeatability/cardinality, type selectors, and semantic category boundaries.

Inputs:
- GE-01 ledger entry for `CHOOSE + MULT`
- GE-01 taxonomy/matrix rows involving `TYPE`, proficiency groups, class-skill type selectors, and equipment categories
- GE-02 `TR-02-010` and `TR-02-011`

Expected downstream outputs:
- choice-set schema or implementation handoff
- selector/type taxonomy decision surface
- diagnostics for unresolved or unsupported selectors

Non-goals:
- full character-builder UI workflow

## GE02-E7 — Provenance, diagnostics, and source-map model
Objective: Define lineage and diagnostic records that tie canonical objects/effects back to legacy source packages, files, entries, spans, support dispositions, and validation outcomes.

Inputs:
- GE-01 conversion matrix provenance requirements
- GE-01 unsupported-token ledger
- GE-01 oracle surface inventory
- GE-02 `TR-02-012` and `TR-02-014`

Expected downstream outputs:
- source-map/provenance schema or implementation handoff
- diagnostic schema and severity policy
- validation/reporting handoff links into GE-03 and GE-05

Non-goals:
- oracle runner implementation

## GE02-E8 — Authoring format and compiled IR boundary
Objective: Define the boundary between source package authoring files and derived compiled runtime IR/cache.

Inputs:
- GE-02 `TR-02-013`
- reference architecture guidance
- downstream GE-04 engine needs when available

Expected downstream outputs:
- decision record or implementation handoff for authoring file format
- decision record or implementation handoff for compiled IR boundary
- validation-to-IR proof obligations

Non-goals:
- complete runtime engine implementation

## Dependency order
Recommended ordering:
1. GE02-E1 package manifest
2. GE02-E2 stable IDs
3. GE02-E3 pilot object schemas
4. GE02-E4 effect/grant model
5. GE02-E5 prerequisite/formula model
6. GE02-E6 choice/selector model
7. GE02-E7 provenance/diagnostics/source-map model
8. GE02-E8 authoring/IR boundary

This order is guidance, not authorization for one giant coding run.

## Downstream readiness notes
- GE-03 has been re-audited now that GE-02 exists as a source STC instead of only a spec-domain dependency; use `references/ge03-importer-dependency-contract.md` as the GE-02-side contract for any further importer planning.
- GE-04 cannot honestly implement engine execution until GE-02 effect/formula/prerequisite model decisions are bounded.
- GE-05 cannot honestly claim oracle comparison until source-map and diagnostic outputs exist.
- GE-06 should not integrate the pilot until GE-02/GE-03/GE-04 produce real evidence.
