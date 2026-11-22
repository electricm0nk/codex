---
title: GE-08 Validation and Preview Workflow Requirements
stc_id: STC-CODEX-GE-08
artifact_type: generated-artifact
status: draft
scope: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts
source_stc: ../README.md
related:
  - ./ge08-e1-minimum-proof-object-selection-2026-06-22.md
  - ./initial-homebrew-acceptance-cases.md
  - ./package-file-lifecycle-requirements.md
  - ../technical-requirements.md
  - ../technical-design.md
  - ../../GE-02-canonical-rules-model-and-content-packages/artifacts/content-validation-and-diagnostics-specification.md
  - ../../GE-04-rules-engine-and-explainability-core/artifacts/diagnostic-schema.md
---

# Validation and Preview Workflow Requirements

## Core problem
GE08-E2 fixed the first proof package substrate as a deterministic YAML source bundle. That alone is insufficient. If validation, preview, and diagnostics remain vague, the later implementation lane will counterfeit success by loading malformed authored content, hiding unsupported semantics behind UI convenience, or presenting disconnected demo previews as proof.

The first GE-08 proof object therefore needs a refusal-first workflow: authored content may be saved while imperfect, but it may not be previewed, exported, or treated as evidence unless the validation and diagnostic contract says so explicitly.

## Decisive rule
For the first proof object, validation is not a courtesy pass before preview. It is the authority gate that decides whether preview, explanation, and portability claims are allowed at all.

Any implementation derived from this artifact MUST preserve all three truths at once:
- authored source remains the authority surface
- diagnostics remain machine-readable and claim-aware
- preview/explanation surfaces show real downstream behavior or an explicit refusal, never a quiet fallback

## Scope of this artifact
This workflow governs only the bounded GE08-E1 proof case:
- one authored `SourcePackage`
- one authored `Feat`
- one authored `Effect`
- optional one authored `Prerequisite`
- one bounded GE-06-derived Human bonus feat substitution
- one bounded armor-class contribution path

It does not authorize general formula editing, broad package composition, product-visible editor UX, plugin-default behavior, or public sharing policy.

## First-proof bridge contract
The preview bridge is allowed to consume only the normalized authored source plus the fixed GE08-E1 proof binding. It is not allowed to invent a looser preview surface, and it is not allowed to wait for GE-07 UI work before becoming precise.

### Required authored inputs
For the first proof object, the minimum conceptual input envelope is:

```yaml
case_id: pf1-crb-human-fighter-level1-homebrew-feat-proof
base_case_id: pf1-crb-human-fighter-level1
homebrew_package:
  package_id: pf1.homebrew.proof.guard-stance
  dependency_on: pf1.crb
  package_state: draft | valid | invalid | deferred
authored_records:
  feat_id: homebrew_guard_stance
  effect_id: <stable effect id>
  prerequisite_id: <optional stable prerequisite id>
proof_binding:
  slot: human_bonus_feat
  remove: dodge
  add: homebrew_guard_stance
preview_targets:
  - selected_feats_and_choice_slots
  - baseline_armor_class
  - diagnostics
  - provenance_or_source_refs
  - explanation_refs
  - oracle_dimension_status
```

The bridge MUST treat the following as required source authority for that envelope:
- `manifest.yaml` for package identity, dependency posture, and package state
- package-local feat/effect/prerequisite source files for authored semantics
- `metadata/provenance.yaml` for authored-source lineage
- `metadata/diagnostics.yaml` for current machine-readable diagnostics state
- the GE08-E1 proof binding that substitutes the Human bonus feat slot in the inherited GE-06 pilot case

The bridge MUST NOT:
- consume UI-only metadata as rules authority
- infer a different pilot case or slot substitution than the GE08-E1 closure fixed
- prepare a preview request from `draft`, `invalid`, or `deferred` content as though it were a valid proof candidate
- drop package-relative source references or stable IDs during normalization

## Workflow state model
The package lifecycle state defined in `package-file-lifecycle-requirements.md` remains authoritative. This artifact adds the validation/preview consequences for each state.

| Package state | Meaning | Validate | Preview | Explain | Export |
|---|---|---|---|---|---|
| `draft` | Saveable authored shell or work-in-progress source not yet validated to proof standard. | Yes | No | Only diagnostics-oriented blocked-path review | No |
| `valid` | Required structure, references, provenance, and proof-case semantics satisfy the first-proof contract with no claim-blocking diagnostics. | Yes | Yes | Yes | Yes |
| `invalid` | Structural or claim-bearing failures exist. | Yes | No | Only blocked-path explanation plus diagnostics | No |
| `deferred` | Package is structurally known but still carries accepted unsupported or intentionally deferred semantics. | Yes | No for proof claims | Diagnostics and deferral visibility only | No |

A UI or CLI MAY let an author save and reopen `draft` or `invalid` source. It MUST NOT present those states as preview-ready or export-ready.

## Required workflow stages

| Stage | Required work | Output | Refusal rule |
|---|---|---|---|
| 0. Load / normalize source | Read `manifest.yaml`, required section files, and package-local metadata from the deterministic source bundle. | In-memory authored-source graph plus initial diagnostics set. | Missing required files, unreadable YAML, or incompatible schema MUST emit diagnostics instead of inventing defaults. |
| 1. Structural package validation | Validate manifest shape, package identity, dependency declaration, required section presence, stable-ID uniqueness, and section-local file placement. | Package-level validation result and package-level diagnostics. | Any package-shape failure blocks preview and export. |
| 2. Object graph validation | Validate feat/effect/prerequisite record shapes, reference resolution, owner links, target families, and proof-binding alignment. | Object-level validation result and record-specific diagnostics. | Broken graph shape or unresolved references block preview and export. |
| 3. Proof-semantics posture validation | Confirm the package still represents the bounded GE08-E1 proof object rather than a widened rule-authoring case. | Claim-scope result naming whether the package is still within proof scope. | Unsupported widening, lossy fallback, or deferred semantics must block proof claims. |
| 4. Preview preparation | Translate validated authored source into a GE-02-consistent preview input without promoting compiled/runtime artifacts to source authority. | Prepared preview request or preparation diagnostics. | Preview preparation MUST refuse to run if claim-blocking diagnostics remain. |
| 5. Bounded preview execution | Run the real downstream preview path for the GE-06-derived case, producing selected-feat, armor-class, diagnostic, provenance, and explanation references. | Real preview output or blocking diagnostics. | Mock values, disconnected demo state, or silently substituted defaults are forbidden. |
| 6. Explanation / diagnostic review | Show how the authored feat and effect contributed to the bounded armor-class result or why the path was blocked. | Explanation refs plus final diagnostic set. | If explanation lineage or provenance cannot be shown for a claimed preview, the claim is blocked. |
| 7. Save / export decision | Persist authored source and diagnostics honestly; permit export only if package state and diagnostics posture allow it. | Saved package and, when allowed, export bundle. | Save may persist invalid drafts; export must refuse invalid, deferred, or preview-counterfeit states. |

## Validation stages in detail

### Stage 0 — Source load and normalization
Load MUST treat the directory-backed YAML bundle as the only authored-source authority. The workflow MUST:
- read `manifest.yaml` first
- load required section files under `objects/`, `rules/`, and `metadata/`
- parse authored YAML into a normalized internal source graph
- preserve package-local stable IDs exactly as authored
- record source file locations suitable for later diagnostics and provenance

Stage 0 MUST NOT:
- synthesize missing authored files silently
- infer package identity from local directory names alone
- use compiled/runtime cache material as fallback source
- auto-heal malformed authored source without recording a diagnostic

### Stage 1 — Structural package validation
Structural package validation MUST cover at least:
- manifest required fields and value shape
- `schema_version` presence and compatibility posture
- package dependency declaration, including the PF1 CRB dependency expected by the proof case
- required directory/section presence for feat, effect, provenance, and diagnostics data
- package-local stable-ID uniqueness across authored records
- deterministic file-placement expectations for the first package bundle
- package-level `validation_state` consistency with actual current diagnostics

Structural package validation answers: "Does this authored bundle exist in an honest package shape at all?"

### Stage 2 — Object graph validation
Object graph validation MUST cover at least:
- feat record kind and required field shape
- effect record kind, owning feat reference, and bounded modifier payload shape
- effect target family validity for armor-class contribution
- prerequisite structure validity if prerequisite parity is claimed
- reference resolution between feat, effect, prerequisite, manifest, and proof binding
- proof-case slot-substitution alignment to the Human bonus feat replacement posture
- prohibition on free-form prose being the only semantic carrier for prerequisite meaning

Object graph validation answers: "Do these authored records form a coherent proof-case graph?"

### Stage 3 — Proof-semantics posture validation
The first proof package is intentionally narrow. Validation MUST therefore confirm that the authored content remains inside the accepted proof boundary.

The package remains in proof scope only if:
- the authored object remains feat-like rather than widening into a different semantic object home
- the authored contribution remains a bounded armor-class modifier
- no new formula language, selector-heavy workflow, or plugin behavior is required to interpret the package honestly
- any prerequisite parity remains structured and conservative
- no UI-only convenience metadata is treated as semantics authority

If the package widens beyond that posture, the correct result is not a lenient preview. The correct result is a diagnostic-bearing refusal or a `deferred` state naming the unsupported expansion.

### Stage 4 — Preview preparation
Preview preparation is the boundary between authored source and derived preview inputs. It MUST:
- consume only authored source that passed validation with no claim-blocking diagnostics
- produce a preview request consistent with GE-02 source-versus-derived boundaries
- carry forward stable references needed for provenance and explanation
- keep diagnostic linkage intact so later preview/explanation failures still point back to authored source

Preview preparation MUST NOT:
- rewrite authored stable IDs
- silently drop unsupported records to make preview succeed
- reuse stale compiled artifacts as proof of current source validity

### Stage 5 — Bounded preview execution
Preview execution for this artifact means the real bounded GE-06-derived case, not a toy renderer. The preview path MUST be able to:
- load the inherited pilot case plus the authored package
- resolve the selected Human bonus feat slot to the authored feat stable ID
- compute the bounded armor-class path or emit blocking diagnostics
- return provenance or source references showing the authored package contribution
- preserve the distinction between successful preview, blocked preview, and unsupported preview

If the preview surface cannot distinguish those outcomes, it is not truthful enough for this proof.

### Stage 6 — Explanation and blocked-path review
A valid preview is incomplete unless explanation truth remains available. The workflow MUST expose:
- authored package ID
- authored feat stable ID
- authored effect stable ID
- relevant prerequisite reference if present
- the selected Human bonus feat slot as the input or selection surface
- the downstream derived-value family for armor class
- provenance/source references showing where the authored contribution came from
- blocking diagnostics when the preview or explanation path cannot continue

Blocked-path explanation is not optional. When preview is refused, the author still needs to see what was refused, why, and which claim was blocked.

## Required preview result contract
The first proof bridge MUST emit a headless result envelope that stays useful even when no GE-07 surface is present yet.

Minimum conceptual fields:

```yaml
case_id: pf1-crb-human-fighter-level1-homebrew-feat-proof
package_id: pf1.homebrew.proof.guard-stance
package_state: valid | invalid | deferred | draft
preview_status: success | blocked | unsupported
selected_slot_resolution:
  slot: human_bonus_feat
  removed: dodge
  added: homebrew_guard_stance
preview_outputs:
  selected_feats_and_choice_slots: <required>
  baseline_armor_class: <value or blocked marker>
  diagnostics: []
  provenance_or_source_refs: []
  explanation_refs: []
  oracle_dimension_status: <status>
blocked_claims: []
```

Output rules:
- `preview_status: success` is allowed only when the real bounded preview path ran and explanation/provenance obligations were satisfied.
- `preview_status: blocked` is required when claim-blocking diagnostics or explanation/provenance gaps prevent a truthful preview claim.
- `preview_status: unsupported` is required when the package stays structurally known but widens beyond the first-proof semantic posture.
- `selected_slot_resolution` MUST echo the exact Human bonus feat substitution so downstream consumers cannot mistake this for a generic feat preview.
- `baseline_armor_class` MUST either report the real bounded derived value or an explicit blocked marker; null-without-explanation is forbidden.
- `diagnostics`, `provenance_or_source_refs`, and `explanation_refs` are mandatory output families even when the preview is blocked.

## Explanation obligations bound to GE-04
GE-04 owns the explanation graph semantics. GE-08 owns the obligation to preserve enough bridge data that the first proof object can actually be explained.

For a successful first-proof preview, the minimum explanation graph surface MUST contain:
- one `character_input` node representing the selected Human bonus feat slot in the inherited pilot case
- one `source_package` node representing the authored homebrew package
- one `canonical_object` node representing the authored feat
- one `effect` node representing the authored armor-class modifier
- one `derived_value` node representing the bounded armor-class result
- one or more `provenance` nodes/refs showing authored-source lineage
- one `prerequisite` node when prerequisite parity is present

Minimum required relationships:
- the Human bonus feat slot MUST `select` the authored feat
- the authored feat MUST connect to the authored effect through the canonical object/effect relationship used by the GE-04 graph
- the authored effect MUST `contribute_to` or `modify` the armor-class `derived_value`
- authored feat/effect or derived-value nodes MUST be `sourced_from` the authored package/provenance lineage
- any blocking diagnostic MUST `diagnose` and/or `block` the affected preview or explanation claim explicitly

If later implementation chooses a more detailed serialized shape, it MAY do so. It MUST still preserve the semantic path: Human bonus feat slot -> authored feat -> authored effect -> armor-class derived value, plus provenance and any blocking diagnostics.

### Stage 7 — Save and export decision
Save and export are different authorities.

Save MUST:
- persist authored source in deterministic form
- rewrite structured diagnostics to reflect current truth
- update package `validation_state` honestly
- allow local persistence of `draft` and `invalid` states without pretending they are preview-ready

Export MUST:
- include authored source, provenance, and diagnostics
- exclude runtime-only or machine-local-only state
- refuse when claim-blocking diagnostics remain
- refuse when the package is `invalid` or `deferred`
- refuse when explanation/provenance obligations required by the proof claim are missing

## Diagnostic contract

### Required diagnostic classes for GE-08 authoring
GE-08 inherits GE-02 and GE-04 diagnostic doctrine. The first proof workflow MUST surface at least these classes when applicable.

| Diagnostic class | Typical authoring meaning in GE-08 | Claim impact |
|---|---|---|
| `invalid_package_shape` | Manifest/layout/required file structure is malformed or incomplete. | Blocks package validity, preview, and export. |
| `invalid_object_shape` | Feat/effect/prerequisite record violates required shape. | Blocks affected authored rule and preview claim. |
| `unresolved_reference` | Feat/effect/prerequisite/slot/dependency reference does not resolve. | Blocks affected preview and explanation path. |
| `invalid_expression` | Structured prerequisite or bounded formula cannot parse or validate safely. | Blocks affected prerequisite/value claim. |
| `unsupported_construct` | Authored content uses semantics the first proof workflow is not allowed to accept. | Blocks proof claim unless separately escalated and accepted outside this artifact. |
| `deferred_semantics` | Content shape is known but semantics are intentionally not yet claimable. | Blocks proof claim; may preserve source as `deferred`. |
| `lossy_conversion_risk` | Authoring/normalization would lose meaning if accepted as-is. | Blocks portability or proof claim for affected scope. |
| `provenance_gap` | Provenance/source lineage is too weak for review, debugging, or explanation. | Blocks explanation or proof claims that require lineage. |
| `invalid_content` | Downstream GE-04 preview sees the authored package as invalid canonical content. | Blocks computation claim for affected scope. |
| `engine_defect` | The preview/explanation machinery failed its own invariants. | Blocks all affected claims; must not be downgraded into content error. |
| `known_gap` | Accepted but still openly tracked limitation on the claimed surface. | Must travel with outputs and prevent overstated claims. |

A later implementation MAY add narrower subclasses. It MUST NOT collapse the classes above into one generic error bucket.

### Required diagnostic fields
Each diagnostic record for this workflow MUST be machine-readable enough for automation and concrete enough for human repair.

Minimum conceptual fields:

```yaml
id: <stable within run or deterministic when possible>
class: <diagnostic class>
severity: info | warning | error | blocker
message: <human-readable summary>
subject_ref: <package/object/effect/prerequisite/preview node>
source_ref: <package-relative file and source location when available>
claim_blocking: true | false
blocked_claims: []
expected: <optional expected condition>
actual: <optional observed state>
recovery_owner: <authoring | ge02 | ge04 | ge06 | engine>
recovery_hint: <next repair action>
related_diagnostics: []
```

The exact field names may later tighten, but the information content above is mandatory.

### Actionable failure rule
A failure is actionable only when the workflow can answer all of the following without reading implementation code:
- what object or package element failed
- where in authored source the failure came from
- which claim is blocked (`preview`, `explanation`, `export`, or broader proof claim)
- whether the problem belongs to authored content, an accepted known gap, or an engine defect
- what minimal repair or escalation path is expected next

A message like "validation failed" is not actionable. A message like "effect.homebrew.guard_stance.ac_bonus targets `attack_bonus`; proof object requires `armor_class`; preview blocked for case pf1-crb-human-fighter-level1-homebrew-feat-proof" is actionable.

## Blocking conditions
The first proof workflow MUST treat the following as claim-blocking unless a higher-order authority explicitly changes the rule:
- missing or malformed `manifest.yaml`
- incompatible or missing `schema_version` posture
- missing PF1 CRB dependency declaration for the selected proof case
- duplicate, missing, or unstable authored stable IDs
- missing feat/effect records required by the proof package
- feat-to-effect or prerequisite references that do not resolve
- effect target not aligned to the bounded armor-class contribution family
- malformed prerequisite structure when prerequisite parity is claimed
- authored source requiring unsupported formula, selector, plugin, or widened semantic behavior
- missing provenance/source lineage required for explanation or review
- preview preparation attempted while claim-blocking diagnostics remain
- preview output returned without real downstream computation or without ability to distinguish refusal from success
- explanation path missing the authored feat/effect/provenance lineage needed to justify the claim
- any `engine_defect` on the claimed path

Warnings MAY exist, but warnings MUST NOT be used to hide a blocked proof claim.

## Negative-case expectations for the first proof object
The first proof package is incomplete unless later implementation can demonstrate these refusal cases explicitly.

| Case ID | Malformed or unsupported condition | Expected diagnostic class(es) | Minimum blocked claims |
|---|---|---|---|
| `GE08-NEG-01` | `manifest.yaml` missing required identity/dependency/proof-binding fields. | `invalid_package_shape` | `preview`, `explanation`, `export` |
| `GE08-NEG-02` | Duplicate or missing stable ID on package, feat, effect, or prerequisite record. | `invalid_package_shape`, `invalid_object_shape` | `preview`, `export` |
| `GE08-NEG-03` | Feat record references a missing effect or prerequisite. | `unresolved_reference` | `preview`, `explanation` |
| `GE08-NEG-04` | Effect target is not the bounded armor-class family required by the proof case. | `invalid_object_shape` or `unsupported_construct` | `preview`, `explanation`, proof claim |
| `GE08-NEG-05` | Prerequisite parity is attempted but represented only as prose or unparseable structure. | `invalid_expression` | `preview`, proof claim |
| `GE08-NEG-06` | Provenance metadata is missing for the authored feat or effect. | `provenance_gap` | `explanation`, `export`, proof claim |
| `GE08-NEG-07` | Preview preparation drops an invalid authored record and continues anyway. | `engine_defect` or equivalent refusal diagnostic | all affected claims |
| `GE08-NEG-08` | Preview returns a value but cannot attach authored feat/effect lineage into explanation/provenance output. | `provenance_gap` or `engine_defect` | `explanation`, proof claim |
| `GE08-NEG-09` | Package introduces widened formula/selector/plugin behavior outside the first proof posture. | `unsupported_construct` or `deferred_semantics` | `preview`, `export`, proof claim |
| `GE08-NEG-10` | Export is attempted while package state is `invalid` or `deferred`. | `invalid_package_shape`, `known_gap`, or state-aware export refusal diagnostic | `export` |

These negative cases are not optional QA extras. They are part of the proof contract. A system that can only demonstrate the happy path has not yet earned authoring trust.

## UI and tooling discipline
GE-08 MAY later gain richer UI surfaces through GE-07-backed work, but convenience surfaces MUST obey the same truth rules as headless flows.

Specifically:
- a diagnostics panel MUST show claim-blocking posture, not just severity color
- preview triggers MUST refuse to imply success when the current package state is `draft`, `invalid`, or `deferred`
- explanation views MUST carry blocked-path results instead of going blank
- authoring tools MUST not auto-convert unsupported semantics into silent omissions
- save/reload affordances MUST preserve machine-readable diagnostics rather than flattening them into prose-only status banners

## Acceptance consequence
A later implementation slice satisfies this artifact only if it can prove all of the following:
- the first proof package can move through load -> validate -> prepare -> preview -> explain when valid
- malformed or widened variants stop at the correct stage with machine-readable actionable diagnostics
- preview and explanation use real downstream rule behavior rather than disconnected mock state
- authored-source identity, provenance, and blocked-claim posture survive the entire workflow

## Documentary route consequence
This artifact is documentary-only. It does not authorize repo edits by itself.

It does, however, narrow the later execution surface materially. A future headless implementation handoff for GE08-E3 should be able to target, at minimum:
- a package validator for the deterministic YAML source bundle
- diagnostic record serialization aligned to the classes and fields above
- bounded proof-case validation fixtures covering both happy-path and negative-path cases
- preview-gating tests proving that invalid or deferred authored packages cannot counterfeit success

## Final rule
The lesser approach would have written "validate before preview" and left the dangerous parts implicit.

That is how counterfeit readiness is born. The first GE-08 proof object now has an explicit refusal-first workflow: validate honestly, preview only when earned, explain lineage when claimed, and fail with machine-readable diagnostics when the truth is anything less.