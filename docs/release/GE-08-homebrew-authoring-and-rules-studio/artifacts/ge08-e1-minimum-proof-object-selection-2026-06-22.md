---
title: GE08-E1 Minimum Proof Object Selection and Fixture Closure
stc_id: STC-CODEX-GE-08
artifact_type: documentary-readiness-closure
status: accepted
scope: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts
source_stc: ../README.md
route_class: documentary-only
owner: Todd Hintzmann
authority_surface: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio
created: 2026-06-22
related:
  - ../epic-breakdown.md
  - ../../GE-02-canonical-rules-model-and-content-packages/artifacts/canonical-model-specification.md
  - ../../GE-04-rules-engine-and-explainability-core/technical-requirements.md
  - ../../GE-04-rules-engine-and-explainability-core/artifacts/pilot-golden-computation-fixture-requirements.md
  - ../../GE-04-rules-engine-and-explainability-core/artifacts/explanation-graph-schema.md
  - ../../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md
  - ../../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/pilot-character-fixture-requirements.md
---

# GE08-E1 Minimum Proof Object Selection and Fixture Closure

## Core problem
GE-08 could not derive an honest first implementation lane while the "minimum homebrew proof object" remained abstract. Without an exact first case, later package-schema, validation, preview, and explanation work would either sprawl into a fake rules studio or quietly invent proof criteria mid-run.

## Verdict
The narrowest first authoring case is a package-local feat-like authored object that replaces the GE-06 Human bonus feat `Dodge` with a homebrew equivalent carrying one bounded armor-class modifier and no new formula language, no selector-heavy choice editing, and no product-visible editor claims.

This keeps the first proof surface inside already-grounded GE-02 model homes and already-required GE-04/GE-06 preview categories. It proves structured authoring mechanics, not broad rules-authoring breadth.

## Selected proof object

### Exact proof-object shape
The first GE-08 proof object is:
- one authored `SourcePackage` layered over the PF1 Core Rulebook package
- one authored `Feat` record selected into the already-closed GE-06 Human bonus feat slot
- one attached `Effect` record that contributes a bounded armor-class modifier
- optional simple prerequisite parity with `DEX >= 13`, but no new formula technology, no new choice-set authoring workflow, and no plugin behavior

The intended semantic posture is deliberately conservative:
- feat-like rather than trait-composition or class-feature composition
- single derived-value family (`armor_class`) rather than multi-output breadth
- one selected-slot substitution rather than broad character rebuild
- explanation/provenance-visible rather than UI-driven

### Why this is the narrowest truthful case
Rejected broader candidates:
- new trait/race-composition authoring would pull GE-02 `RaceTrait` replacement posture into the first proof and widen scope unnecessarily
- package-local equipment or weapon-rule edits would entangle the first proof with more equipment-state and attack-path variation than required
- formula-heavy or selector-heavy objects would force expression-language and choice-model breadth before the package/edit/validate/preview loop is even proven
- product-visible editor work would counterfeit GE-07 readiness

Why this case survives scrutiny:
- it uses a single authored object kind already grounded in GE-02 (`Feat`)
- it targets an output family already required by GE-04 and GE-06 (`armor_class`)
- it can be validated, previewed, explained, diffed, imported, and exported without invoking plugins
- it keeps GE-08 focused on structured authoring proof rather than on a broad rules-studio story

## GE-02 model-home binding
The proof object is only valid if it binds to the existing GE-02 canonical model homes instead of inventing GE-08-local structure.

| GE-02 home | Required use in this proof case | Why it matters |
|---|---|---|
| `SourcePackage` | Carry the authored homebrew package identity, dependency on the PF1 CRB package, and validation state. | GE-08 must prove package-local authoring, not free-floating rule blobs. |
| `StableId` | Give deterministic IDs to the authored package, feat object, and attached effect. | Diffability, re-editability, and import/export portability depend on stable identity. |
| `Feat` | Represent the authored proof object itself. | The first proof must stay within an already-grounded semantic object home. |
| `Effect` | Carry the bounded armor-class contribution. | The authored change must remain structured and explainable. |
| `Prerequisite` | Optional simple prerequisite parity only; if used, it must stay structured and bounded. | GE-08 may not smuggle a broad expression editor into the first proof. |
| `Diagnostic` | Surface duplicate IDs, invalid field shape, bad prerequisite structure, or blocked preview claims. | Validation failure must stay visible and claim-blocking. |
| `ProvenanceRecord` / `SourceMapEntry` | Record that the contribution came from the authored package object, not from core imported content. | Preview/explanation must show authored-source lineage honestly. |
| `CompiledRuntimeIR` boundary | Preserve authored package content as source authority while any preview path remains a derived boundary. | GE-08 must not let preview caches become the new source of truth. |

No new GE-02 semantic object home is authorized by this selection. If a later GE-08 slice needs a different home to make the first case work, the correct move is upstream GE-02 review, not local improvisation.

## GE-06-derived fixture closure
GE-08 does not rewrite the canonical GE-06 baseline fixture. It defines a bounded homebrew variant that inherits the closed GE-06 pilot and applies one explicit authored delta.

### Base case
- inherited case: `pf1-crb-human-fighter-level1`
- inherited authority: `../../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e1-f1-final-deterministic-pilot-input-contract-2026-06-21.md`

### Homebrew proof variant
```yaml
case_id: pf1-crb-human-fighter-level1-homebrew-feat-proof
inherits_from: pf1-crb-human-fighter-level1
homebrew_package:
  id: pf1.homebrew.proof.guard-stance
  dependency_on: pf1.crb
slot_substitution:
  slot: human_bonus_feat
  remove: dodge
  add: homebrew_guard_stance
unchanged_from_base_case:
  - race
  - class_levels
  - ability_scores
  - level_1_character_feat (Power Attack)
  - fighter_bonus_feat (Weapon Focus [Longsword])
  - skill_ranks
  - equipment
  - active_states
preview_target_family:
  - selected_feats_and_choice_slots
  - baseline_armor_class
  - diagnostics
  - provenance_or_source_refs
  - explanation_refs
  - oracle_dimension_status
```

### Closure rule
This GE-08 variant is closed enough for downstream readiness work because:
- the slot mutation is exact and singular
- the base GE-06 pilot remains unchanged everywhere else
- the preview target family is already grounded by GE-04/GE-06 authority surfaces
- the homebrew package proves authored-source contribution without forcing broader fixture redesign

Any later attempt to widen the first homebrew case beyond this one-slot substitution is scope-bearing and must justify itself explicitly.

## GE-04 / GE-06 preview obligations
A later implementation-facing GE-08 handoff must preserve the following proof duties.

### Validation duties
The authored package must produce structural validation over at least:
- package manifest identity and dependency reference
- stable-ID uniqueness
- authored feat object-kind and field-shape validity
- effect target validity for the bounded armor-class modifier
- prerequisite structure validity if a prerequisite is present
- provenance completeness for the authored object and effect

### Preview duties
Headless preview must be able to:
- load the inherited GE-06 pilot variant plus the authored package
- resolve the selected Human bonus feat to the authored feat stable ID
- compute the bounded armor-class preview path or emit blocking diagnostics
- preserve the GE-02 authored-source versus derived-preview boundary

### Explanation duties
The explanation path must be able to show, at minimum:
- the selected Human bonus feat slot as character input / chosen state
- the authored feat as a `canonical_object`
- the authored modifier as an `effect`
- contribution into the `derived_value` for armor class
- attached provenance showing the effect came from the authored package
- blocking diagnostics when the authored object is invalid or unsupported

### Claim-tier rule
This first proof case may claim only that:
- structured homebrew authoring can create or edit one bounded feat-like rule without LST
- validation can accept or refuse the authored package honestly
- headless preview/explanation can expose the authored contribution or its failure visibly

It may not claim:
- broad editor readiness
- general formula authoring
- broad choice-set authoring
- plugin sufficiency or plugin necessity
- end-user product usability
- broad Pathfinder homebrew coverage

## Negative-case obligations
The first proof case is incomplete unless it also carries one malformed authored variant that proves the system refuses counterfeit success.

Minimum malformed cases:
- duplicate or missing `StableId`
- invalid effect target for the armor-class modifier
- malformed prerequisite structure if prerequisite parity is attempted
- missing authored provenance metadata required by the package contract

Those cases must block preview claims and surface diagnostics rather than quietly falling back.

## Downstream routing consequence
This closure does not authorize code by itself. It only fixes the first truthful target for later bounded GE-08 slices.

The immediate downstream implications are:
- GE08-E2 should ground package-schema and lifecycle work for exactly this feat-like object plus one authored package dependency edge
- GE08-E3 should ground validation and diagnostics around this object's structural and negative cases
- GE08-E4 should ground preview/explanation over this object's armor-class contribution path in a headless lane
- GE08-E5 remains out of scope until GE-07-backed product-visible routing is separately grounded

## Final rule
The lesser approach would say "build a homebrew editor" and call that direction. That is noise.

The decisive move is smaller: prove one package-local feat-like authored object, bound to one already-closed GE-06 pilot variant, with real GE-02 identities and real GE-04/GE-06 preview obligations. Everything broader is downstream of that proof or irrelevant to it.