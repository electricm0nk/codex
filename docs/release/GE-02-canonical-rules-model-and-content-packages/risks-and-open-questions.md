---
title: GE-02 Risks and Open Questions
stc_id: STC-CODEX-GE-02
artifact_type: risks-and-open-questions
status: accepted
scope: programs/codex/requirements/GE-02-canonical-rules-model-and-content-packages
source_stc: ./README.md
---

# GE-02 Risks and Open Questions

## Purpose
Quarantine unresolved canonical-model decisions so they remain visible and do not get silently solved by a future importer, engine, or UI implementation.

## Blocking rules
Stop downstream implementation handoff derivation if any proposed slice requires one of these unresolved decisions and the handoff does not explicitly exclude or resolve it:
- stable ID syntax and namespace policy
- expression/prerequisite/formula technology
- effect stacking and resolution order
- selector/type taxonomy for overloaded legacy `TYPE` surfaces
- compiled IR authority and cache invalidation boundaries
- source-span downgrade policy for provenance

## Open questions carried from the GE-02 spec domain
1. What is the minimum object set required to represent the pilot without broad Pathfinder modeling?
   - Current answer: the minimum object homes are listed in `TR-02-006`, but exact production schema remains deferred.
2. What stable ID convention is durable across imported PCGen content and future native authored packages?
   - Current answer: requirements are defined in `TR-02-005`; exact syntax remains open.
3. Which expression-language qualities are mandatory before choosing CEL-like, Rhai-like, or another constrained expression system?
   - Current answer: structure, determinism, provenance, diagnostics, and sandboxing are mandatory; exact technology remains open.
4. What provenance fields are required for debugging imported content?
   - Current answer: minimum fields are defined in `TR-02-012`; final span precision remains open per implementation capability.
5. Where is the boundary between human-authored source files and compiled runtime IR?
   - Current answer: `TR-02-013` fixes the authority split; exact IR serialization remains open.
6. Which validation diagnostics must exist before authoring can be trusted?
   - Current answer: `TR-02-014` defines the diagnostic categories; exact CLI/API output remains downstream.

## GE-01 ledger-derived unresolveds
The following GE-01 ledger pressures must not disappear:

| Pressure | Severity | GE-02 owner |
|---|---:|---|
| `PREMULT / PREPROFWITH*` prerequisite chains | medium | prerequisite model |
| Fighter archetype-conditioned proficiency suppression | medium | effect/class-feature model |
| Formula-bearing `BONUS / DEFINE / VAR` expressions for Fighter progression | high | formula/value model |
| `CHOOSE + MULT` on Martial Weapon Proficiency | medium | choice-set model |
| Human racial-trait indirection through ability carrier rows | medium | race-trait/effect model |
| Fighter class-skill carrier through explicit skills and `TYPE` selectors | medium | skill relation / selector model |
| Human trait replacement flags and `PREFACT` gates | medium | race-trait composition and prerequisite model |
| Base-stat formula and derived variable surface | high | ability-score/formula model |
| Equipment-to-proficiency references for armor and shields | medium | equipment/proficiency relation model |
| `STARTSKILLPTS + FighterSkillPoints` variable chain | medium | class/character-creation formula model |

## Design risks

### Risk: LST clone by another name
If Codex preserves raw LST token tables as the canonical object model, it will inherit PCGen complexity without gaining a clean authoring substrate.

Mitigation:
- require semantic model homes
- preserve raw syntax only as provenance/source text or parser input
- reject implementation handoffs that model canonical content as raw-token bags

### Risk: formula and prerequisite semantics are flattened
Formula and prerequisite behavior is high-risk because pilot correctness depends on derived values and eligibility gates.

Mitigation:
- keep formulas/prerequisites structured even when unresolved
- require diagnostics for unevaluable forms
- defer engine evaluation to GE-04 without hiding the model obligation

### Risk: stable IDs overfit local paths
Imported content needs stable IDs that survive reruns and future native authoring. Absolute paths are evidence, not identity.

Mitigation:
- namespace IDs by package and object kind
- preserve PCGen paths in provenance/source maps
- record exact ID syntax as a deferred decision until implementation design

### Risk: source-package and runtime IR are conflated
If compiled runtime cache becomes the only representation, human authoring and review suffer.

Mitigation:
- preserve source package authority
- treat compiled IR as derived/cache/runtime material
- require validation evidence linking source to compiled output

### Risk: GE-03 importer defines its own target model
The importer may drift into canonical design because it needs target objects.

Mitigation:
- GE-03 must cite this GE-02 STC as a required dependency
- GE-03 must use `references/ge03-importer-dependency-contract.md` when deciding what it may consume from GE-02 artifacts
- conversion handlers must target GE-02 model homes or emit explicit unresolved diagnostics
- any materially new model home discovered during importer work must propagate back to GE-02 or a superseding decision surface

## Forbidden assumptions
- Do not assume all PCGen `TYPE` values are durable canonical categories.
- Do not assume every formula can be evaluated in the first engine milestone.
- Do not assume a prerequisite can be stored as free text and remain useful.
- Do not assume Human and Fighter carrier indirection can be flattened without losing explanation or provenance.
- Do not assume schema examples in `technical-design.md` are final production schemas.
- Do not assume GE-02 planning-ready means implementation-ready.
- Do not assume the GE-03 importer may treat GE-02 artifact examples as final schemas, evaluator choices, runtime behavior, or code-authorizing instructions.

## Recovery path
When a downstream worker encounters a missing canonical-model fact:
1. determine whether the fact is already covered by `technical-requirements.md`
2. if covered but underspecified, record the implementation-specific decision in a bounded handoff or ADR
3. if the missing fact concerns GE-03 importer planning, check `references/ge03-importer-dependency-contract.md` before deriving or patching a downstream handoff
4. if not covered, patch this GE-02 source STC or create a superseding decision surface before implementation proceeds
5. never let the importer or engine silently invent permanent model doctrine under local pressure
