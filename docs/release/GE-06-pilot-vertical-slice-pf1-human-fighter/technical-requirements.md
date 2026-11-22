---
title: GE-06 Technical Requirements
stc_id: STC-CODEX-GE-06
artifact_type: technical-requirements
status: draft
scope: programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter
source_stc: ./README.md
source_artifacts:
  - ../../plans/spec-domains/GE-06-pilot-vertical-slice-pf1-human-fighter.md
  - ../../plans/pilot-slices/pf1-crb-human-fighter-level1-charter.md
  - ../GE-01-legacy-corpus-and-conversion-matrix/README.md
  - ../GE-01-legacy-corpus-and-conversion-matrix/artifacts/pilot-token-taxonomy.csv
  - ../GE-01-legacy-corpus-and-conversion-matrix/artifacts/conversion-matrix.csv
  - ../GE-02-canonical-rules-model-and-content-packages/README.md
  - ../GE-02-canonical-rules-model-and-content-packages/artifacts/canonical-model-specification.md
  - ../GE-03-pcgen-import-pipeline-and-provenance/technical-requirements.md
  - ../GE-04-rules-engine-and-explainability-core/technical-requirements.md
  - ../GE-04-rules-engine-and-explainability-core/artifacts/pilot-golden-computation-fixture-requirements.md
  - ../GE-05-oracle-validation-and-parity-harness/technical-requirements.md
  - ../../plans/spec-domains/GE-07-desktop-shell-and-modern-ux.md
  - ../../doctrine/quality-gate-policy.md
---

# GE-06 Technical Requirements

## Objective
Define the normative requirements for the Codex integrated pilot vertical slice: one bounded PF1 Core Rulebook Human Fighter level 1 path that can be imported, represented canonically, computed, explained, compared against legacy PCGen for selected outputs, surfaced through a minimal UI, and judged viable or not without counterfeit completion.

## Normative language
- **MUST** means required for GE-06 completion.
- **SHOULD** means expected unless a later decision surface records a justified deviation.
- **MUST NOT** means prohibited for this STC.

## TR-06-001 — Integrated vertical-slice posture
Codex MUST treat GE-06 as all of the following:
- an integrated proof contract spanning import, canonical content, computation, explanation, selected parity comparison, diagnostics, and minimal UI truth
- the program's first end-to-end product-slice gate for the PF1 Human Fighter level 1 pilot
- a narrow architectural test that can expose fatal flaws before broader expansion
- a planning boundary that decomposes later work into bounded execution slices rather than a single broad implementation command

Codex MUST NOT treat GE-06 as:
- broad Pathfinder support
- permission to collapse GE-01 through GE-05 ownership boundaries into one vague “integration” bucket
- a substitute for the GE-07 UI source STC
- proof that parity, viability, or product truth already exists before evidence is generated

## TR-06-002 — Upstream dependency truth
The GE-06 source STC MUST state these dependencies explicitly:
- GE-01 owns grounded pilot corpus discovery, token-family taxonomy, conversion-matrix posture, and unsupported-token visibility
- GE-02 owns canonical model homes, object relationships, diagnostic/provenance records, and the authoring-versus-runtime boundary
- GE-03 owns importer, structured parse, provenance, conversion-report, and unsupported-token diagnostic obligations
- GE-04 owns deterministic character computation, explanation, diagnostic emission, and pilot golden-computation fixture posture
- GE-05 owns oracle comparison, normalization, actionable diff, known-gap, and compatibility claim-tier posture
- GE-07 owns desktop shell and broader UX architecture once it exists as a source STC
- the quality-gate policy owns the evidence tiers that govern `Converted`, `Computed`, `Oracle-checked`, and `Product-visible` claims

GE-06 MUST consume those boundaries rather than redefining them locally.

## TR-06-003 — Upstream readiness and evidence gate
Before any future GE-06 implementation slice may claim it is execution-ready, the slice MUST ground which upstream proof surfaces it depends on.

At minimum, a bounded GE-06 implementation slice MUST identify:
- the exact GE-03 import surface or blocker it depends on
- the exact GE-04 computation/explanation surface or blocker it depends on
- the exact GE-05 comparison surface or blocker it depends on when any oracle or compatibility claim is in scope
- whether the slice is headless-only or product-visible
- whether a GE-07 source STC or an explicit non-production spike charter is required for the UI portion

GE-06 MUST NOT assume that because GE-03 through GE-05 source STCs exist, their runtime outputs already exist.

## TR-06-004 — Pilot character fixture requirements
GE-06 MUST define the integrated pilot character fixture as a grounded contract rather than a hand-waved example.

The fixture requirements MUST preserve the following grounded facts from the pilot charter:
- case identity: `pf1-crb-human-fighter-level1`
- race: Human
- class: Fighter
- level: 1
- initial ability-score vector: STR 16, DEX 14, CON 14, INT 10, WIS 12, CHA 8
- at least one explicitly named feat path: `power_attack`
- required output categories including base attack bonus, Fortitude/Reflex/Will saves, melee attack bonus, armor class, skill ranks, and equipment effects

The fixture requirements MUST also preserve unresolved selection debt explicitly when not yet grounded, including exact skill allocation, exact equipment loadout, any additional feat or choice entitlement implied by Human/Fighter rules surfaces, and the exact export-summary boundary.

GE-06 MUST NOT fabricate final expected values; exact values belong to later GE-04/GE-05-backed evidence.

## TR-06-005 — Required token-family boundary
GE-06 MUST define a required token-family list for the integrated slice using grounded GE-01 taxonomy and matrix inputs.

The hard-gate set MUST include, at minimum, pilot-critical families such as:
- PCC include directives
- object declarations for CLASS, RACE, SKILL, and EQUIPMENT
- ABILITY, AUTO, BONUS, PRE* prerequisites, CSKILL, KEYSTAT, STARTSKILLPTS, STATMOD/MODIFY, STARTFEATS, ABILITYPOOL, CHOOSE, PROFICIENCY references, and PREVARGTEQ/PREVAREQ/PREFACT gates

The supporting-but-still-visible set SHOULD include, when touched by the slice:
- WEAPONPROF / ARMORPROF / SHIELDPROF catalogs
- TYPE facets
- DEFINE / VAR support expressions
- FACT / MOVE / TEMPLATE / RACETYPE / LEGS / HANDS metadata
- MULT and related repeatability semantics

GE-06 MUST classify each token family as hard gate, supporting dependency, or deferred risk. It MUST NOT flatten the entire slice into a generic “import works” claim.

## TR-06-006 — Required canonical-object boundary
GE-06 MUST define the minimum canonical model homes and support records the integrated slice depends on.

At minimum, the required canonical-object list MUST include:
- `SourcePackage`
- `StableId`
- `Race` and `RaceTrait`
- `Class` and `ClassFeature`
- `Feat`
- `Skill`
- `Equipment`
- `Proficiency`
- `AbilityScore`
- `Save`
- `Effect`
- `Prerequisite`
- `Formula`
- `ChoiceSet`
- `Selector`
- `Diagnostic`
- `ProvenanceRecord` / `SourceMapEntry`
- the `CompiledRuntimeIR` boundary as a derived-runtime obligation rather than authored content authority

GE-06 MUST name which of these are mandatory for the first integrated proof and which remain supporting dependencies. It MUST NOT invent new canonical homes where GE-02 has already grounded them.

## TR-06-007 — End-to-end proof path requirements
GE-06 MUST define one end-to-end proof path for the pilot slice.

That path MUST include, at minimum:
- source-package identity and pilot input boundary
- import/validation prerequisites and diagnostic posture
- canonical object availability requirements
- character-input resolution requirements
- deterministic computation and explanation emission requirements
- selected oracle-comparison boundary and known-gap posture
- minimal UI projection boundary
- stack-viability decision output

Every stage MUST produce either evidence or an explicit blocker. No stage may hide failure behind a generic “integration incomplete” statement.

## TR-06-008 — Headless-first integration gate
Before any GE-06 slice may claim UI truth, the same pilot path MUST be runnable headlessly.

At minimum, future implementation handoffs MUST preserve the ability to:
- load the pilot package or equivalent fixture inputs without desktop UI
- compute pilot outputs without desktop UI
- emit explanations and diagnostics without desktop UI
- emit selected oracle-comparison inputs or blockers without desktop UI
- fail loudly when expected values, explanations, diagnostics, or required receipts are absent

UI-only demonstration is insufficient for GE-06.

## TR-06-009 — Explanation, provenance, and diagnostic visibility
GE-06 MUST preserve explanation and diagnostic visibility across the integrated path.

At minimum, the integrated slice MUST be able to surface:
- why each tested derived value has its observed value
- why each tested unavailable choice or failed prerequisite is unavailable
- which imported source rows/tokens contributed when imported content drives behavior
- importer, engine, normalization, and UI-facing diagnostics relevant to the slice
- known-gap and blocked-comparison status when parity cannot yet be claimed

A numerical answer or visible UI row without explanation provenance is incomplete.

## TR-06-010 — Selected oracle-comparison boundary
GE-06 MUST consume GE-05 parity doctrine without broadening it.

The integrated slice MUST define:
- which pilot outputs are mandatory comparison targets for viability
- which outputs may remain known gaps without invalidating the entire slice
- which outputs are out of current scope
- what evidence is required before a selected output may be called `Oracle-checked`

GE-06 MUST NOT treat static PCGen files, screenshots, or plausible-looking new-system values as parity evidence.

## TR-06-011 — Minimal UI truth contract
GE-06 MUST define a minimal UI contract that consumes real domain outputs.

At minimum, the first acceptable UI surface MUST be able to:
- load and display the pilot character path from real importer/engine/parity outputs or explicit blockers
- show derived-value explanation affordances
- keep importer and validation diagnostics visible rather than hiding them
- make invalid or unavailable choices inspectable
- stay within the pilot charter boundary

UI code MUST NOT own rules semantics. Final shell architecture remains a GE-07 concern.

## TR-06-012 — Failure taxonomy and ownership
GE-06 MUST classify integrated failures into one primary category:
- model flaw
- importer flaw
- engine flaw
- oracle gap
- UI gap

The failure report SHOULD also record contributing owners when a failure crosses boundaries, but it MUST still identify one primary owner so follow-up work narrows rather than diffuses.

GE-06 MUST NOT allow “integration issue” to be the terminal diagnosis.

## TR-06-013 — Pilot-stack viability decision criteria
GE-06 MUST produce an explicit viability-decision artifact.

The viability criteria MUST tie decisions to Codex quality-gate tiers. At minimum, a pilot may be called viable only when:
- selected import surfaces are at least `Converted` with provenance and diagnostics
- selected rules outputs are at least `Computed` with explanations
- selected comparison dimensions are `Oracle-checked` or explicitly recorded as known gaps with stated consequence
- selected UI surfaces are `Product-visible` over real domain behavior
- remaining unresolved items do not erase the pilot's ability to prove or falsify the architecture

The criteria MUST also define fatal-flaw triggers, narrowing triggers, and upstream-expansion triggers.

## TR-06-014 — Charter update and scope-control rule
GE-06 MUST preserve the pilot charter as the scope boundary.

A later session MUST update the charter and consider an ADR when the integrated slice adds or changes:
- new books or non-Core-Rulebook content
- new classes or broader feat/equipment breadth beyond the minimum pilot needs
- UI surfaces beyond the minimum truth contract
- broader export parity than one bounded summary path
- new architectural constraints that change the ownership split between GE-06 and another spec domain

GE-06 MUST NOT hide scope expansion inside fixture “clarification.”

## TR-06-015 — Produced artifacts
GE-06 MUST produce a source-STC bundle containing:
- `README.md`
- `technical-requirements.md`
- `technical-design.md`
- `acceptance-and-verification.md`
- `risks-and-open-questions.md`
- `epic-breakdown.md`

GE-06 MUST also produce concrete same-epic documentary outputs containing:
- `references/upstream-dependency-contract.md`
- `artifacts/pilot-charter-alignment.md`
- `artifacts/pilot-character-fixture-requirements.md`
- `artifacts/required-token-family-list-requirements.md`
- `artifacts/required-canonical-object-list-requirements.md`
- `artifacts/pilot-stack-viability-decision-criteria.md`

This package MUST live under `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/`.

## TR-06-016 — Downstream routing rule
GE-06 MUST route later implementation into bounded downstream epics rather than a single integrated handoff.

At minimum, downstream decomposition MUST include:
- pilot character fixture closure and governed-input alignment
- headless integrated import/compute/proof-path work
- parity-dimension and failure-routing integration
- minimal UI truth slice
- viability-review and upstream-delta review work

## TR-06-017 — Authority surface boundary
During GE-06 source-STC generation, writable scope MUST remain limited to the Codex documentation authority surface.

The work MUST NOT:
- modify `/home/ubuntu/workspace/repos/pcgen`
- write implementation code into `/home/ubuntu/workspace/repos/codex`
- substitute repo scaffolding or screenshots for requirement truth
- create a code-authorizing handoff from this package-generation pass

## Success definition
GE-06 succeeds when Codex has an integrated pilot-planning surface strong enough to say:
- what the first end-to-end pilot case is and which selections remain unresolved
- which token families and canonical model homes are mandatory for that case
- what the headless proof path must emit before UI truth claims are allowed
- which selected outputs require oracle comparison and how failures are categorized
- what counts as pilot viability versus fatal architectural failure
- which later implementation slices exist and which upstream truths they depend on

If those answers still require invention, GE-06 is not complete.
