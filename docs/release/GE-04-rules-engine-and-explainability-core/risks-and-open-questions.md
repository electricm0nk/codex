---
title: GE-04 Risks and Open Questions
stc_id: STC-CODEX-GE-04
artifact_type: risks-and-open-questions
status: draft
scope: programs/codex/requirements/GE-04-rules-engine-and-explainability-core
source_stc: ./README.md
---

# GE-04 Risks and Open Questions

## Primary risks

| ID | Risk | Consequence | Mitigation |
|---|---|---|---|
| R-04-001 | Black-box calculation | Codex returns plausible numbers without proving source contributions, recreating PCGen opacity. | Require explanation graph output and contribution traces for every tested derived value and failed prerequisite. |
| R-04-002 | Premature evaluator choice | Formula/prerequisite implementation is chosen before pilot semantic pressure is understood. | Preserve evaluator criteria in requirements and require a later spike/ADR before final selection. |
| R-04-003 | Evaluation-order folklore | Hidden dependency ordering bugs appear because effect activation, prerequisites, choices, and derived values are evaluated ad hoc. | Maintain an explicit pilot evaluation-order artifact and escalate unresolved order issues. |
| R-04-004 | Unsupported semantics become executable | Imported lossy/deferred behavior runs as though accepted, corrupting computation and parity claims. | Inherit GE-02/GE-03 diagnostics and block higher-tier claims when relevant diagnostics remain unresolved. |
| R-04-005 | Explanation graph too shallow | Outputs cannot be debugged, tested, or shown in UI because explanations only restate final values. | Require nodes/edges for inputs, effects, formulas, prerequisites, diagnostics, and provenance. |
| R-04-006 | Fixture invention | The STC fabricates final pilot character values before GE-06 fixture and GE-05 oracle evidence exist. | Keep fixture requirements concrete but value assertions deferred until fixture/oracle surfaces are grounded. |
| R-04-007 | Counterfeit code readiness | A coding harness receives this source STC as a broad implementation prompt. | Keep `planning-only` route, record unresolved execution facts, and require later bounded handoff with exact write scope and tests. |

## Open questions

### OQ-04-001 — What exact pilot derived values must GE-04 compute first?
Question: Which derived values are mandatory before GE-05 comparison is meaningful for the PF1 Human Fighter level 1 pilot?

Recommended answer if known: This STC names required categories, but the exact values and final assertions depend on GE-06 character fixture finalization and GE-05 oracle output discovery.

Owner if known: Future GE-04/GE-06 fixture planning.

Deferred owner: GE-04 pilot golden computation fixture and GE-06 pilot vertical-slice STC.

### OQ-04-002 — What evaluation order prevents hidden dependency bugs?
Question: What exact order should apply across static grants, formulas, prerequisites, choices, selected equipment, derived values, and explanation emission?

Recommended answer if known: The GE-04 evaluation-order artifact defines a pilot planning order, but final cycle, fixed-point, and invalidation behavior remain open until implementation pressure and tests ground them.

Owner if known: Future GE-04 rules-core implementation planning.

Deferred owner: GE04-E2 effect core and GE04-E3 formula/prerequisite evaluator epics.

### OQ-04-003 — How should stacking and modifier interactions be represented?
Question: How should stacking, modifier types, overlapping effects, conditional effects, and selected equipment be represented for the first pilot without overgeneralizing into full Pathfinder coverage?

Recommended answer if known: Represent only the pilot-required interactions initially and treat broader stacking doctrine as a future expansion decision. Do not silently apply generic “add everything” behavior.

Owner if known: Future rules-core design work.

Deferred owner: GE04-E2 effect core and GE04-E5 derived stat/diagnostic epics.

### OQ-04-004 — What expression-language semantics are mandatory?
Question: Which expression semantics are mandatory for pilot formulas and prerequisites before choosing an evaluator?

Recommended answer if known: GE-02 and GE-04 require deterministic, sandboxed, structured, dependency-capturing, diagnostic-rich expression evaluation. Final evaluator choice is deferred.

Owner if known: Future expression-language spike or ADR.

Deferred owner: GE04-E3 formula/prerequisite evaluator epic and possible `programs/codex/doctrine/decisions/` ADR.

### OQ-04-005 — What explanation graph granularity is sufficient?
Question: How much detail must the explanation graph preserve for users, tests, diagnostics, and future UI display?

Recommended answer if known: At minimum, every tested derived value and unavailable choice must link to contributing inputs, effects, formulas/prerequisites, diagnostics, and provenance. The exact serialization and UI projection remain deferred.

Owner if known: Future explanation graph implementation planning.

Deferred owner: GE04-E6 explanation graph builder and GE-07 UI presentation boundary.

### OQ-04-006 — How should diagnostics distinguish failure classes?
Question: How should rules diagnostics distinguish invalid content, invalid character choices, unsupported imported semantics, unresolved references, invalid expressions, circular dependencies, provenance gaps, and engine defects?

Recommended answer if known: The GE-04 diagnostic schema defines required classes and fields, but exact codes, severities, and CLI/test formatting remain implementation decisions.

Owner if known: Future diagnostics design work.

Deferred owner: GE04-E5 diagnostic emitter and GE04-E7 CLI/test entry-point epics.

### OQ-04-007 — What is the first code-authorizing GE-04 slice?
Question: Which bounded implementation slice should receive the first GE-04 execution handoff?

Recommended answer if known: GE04-E1-F1 — Character input record shape is selected and validated as the first code-authorizing GE-04 slice. The derived handoff is `execution-handoff.md`, and the coding run must branch from a clean, current `develop`.

Owner if known: Program-level planning owner.

Deferred owner: `artifacts/ge04-e1-f1-execution-readiness-closure-2026-06-20.md`, then a future GE-04 execution handoff if the blocker is resolved.

### OQ-04-008 — What branch-base policy governs GE04-E1-F1?
Question: What branch should a future GE04-E1-F1 feature branch start from and target?

Recommended answer if known: Resolved for GE04-E1-F1. The derived execution branch is `ge04-e1-f1-character-input-record-shape`, it must start from clean current `develop`, and its eventual PR targets `develop`. Promotion from `develop` to `main` remains a separate PR.

Owner if known: Todd Hintzmann.

Deferred owner: Future GE-04 execution-readiness update.

## Intentionally deferred
- final expression evaluator or language implementation
- final production schema for explanation graph serialization
- exact rules-engine Rust module layout
- full Pathfinder stacking and bonus-type doctrine
- full spellcasting engine or broad rules abstraction
- GE-05 oracle harness implementation
- GE-06 final character selections and exact expected values
- GE-07 UI presentation of explanations
- repo-local GE-04 implementation branch, write scope, and verification commands
- local checkout cleanup/update before deriving GE04-E1-F1 from `develop`

## Forbidden assumptions
- that GE-04 planning-ready status authorizes rules-engine code writes
- that a numerical derived value is correct without a contribution explanation
- that unsupported importer diagnostics can be ignored during execution
- that GE-02 expression criteria have already selected an evaluator
- that GE-05 parity is proven by GE-04 computation alone
- that UI display can substitute for headless rules tests
- that broad Pathfinder stacking rules are required before the pilot slice can progress
- that a GE-04 coding harness may branch from `main` or from a stale GE-03 feature branch instead of current `develop`

## Review trigger
Reopen this file when GE-02 changes model homes/expression/diagnostics/provenance/IR boundaries, GE-03 produces actual importer outputs, GE-05 grounds oracle output dimensions, GE-06 fixes the exact pilot character fixture, a spike/ADR selects evaluator or stacking policy, or a future GE-04 implementation handoff is proposed.
