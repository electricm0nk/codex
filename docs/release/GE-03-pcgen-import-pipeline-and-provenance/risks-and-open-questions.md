---
title: GE-03 Risks and Open Questions
stc_id: STC-CODEX-GE-03
artifact_type: risks-and-open-questions
status: draft
scope: programs/codex/requirements/GE-03-pcgen-import-pipeline-and-provenance
source_stc: ./README.md
---

# GE-03 Risks and Open Questions

## Primary risks

| ID | Risk | Consequence | Mitigation |
|---|---|---|---|
| R-03-001 | Target-model overreach | The importer treats GE-02 planning artifacts as final production schema or invents model details beyond accepted GE-02 boundaries, causing LST-clone drift or arbitrary model invention. | Cite accepted GE-02 artifacts for canonical model homes and keep remaining final-schema, expression/evaluator, and runtime-engine gaps visible in requirements, design, and later handoffs. |
| R-03-002 | Unsupported-token silence | Import output appears healthier than it is because unresolved or lossy behavior disappears during parsing or translation. | Require explicit unsupported-token diagnostics, conversion-report caveats, and linkage back to GE-01 control-plane posture. |
| R-03-003 | Provenance collapse | Later debugging, parity review, and explainability become impossible because source lineage is not preserved through parse and conversion stages. | Require source-map obligations before implementation begins and name downgrade behavior explicitly. |
| R-03-004 | Registry/handler blur | Responsibility for legacy semantics becomes unreviewable because token routing, handler logic, and validation obligations are smeared together. | Keep token-registry and conversion-handler boundaries separate in the source STC and later handoffs. |
| R-03-005 | Counterfeit readiness | A coding harness receives this source STC as if it were already a code-authorizing implementation brief. | Keep readiness below execution-ready, block code-authorizing handoff derivation, and require bounded implementation-slice facts later. |
| R-03-006 | Oracle folklore | Teams assume future parity checks will be possible without proving which legacy surfaces are actually automatable or trustworthy. | Keep oracle-backed validation as a planned downstream capability and preserve uncertainty explicitly until grounded. |

## Open questions

### OQ-03-001 — What structured representation is sufficient for the first importer slice?
Question:
Should the first parser milestone preserve a token stream, AST, typed AST, semantic events, or a layered combination?

Recommended answer if known:
Not yet fixed here. GE-03 should require structure and provenance, not prematurely choose a representation class without review.

Owner if known:
Future parser design work.

Deferred owner:
GE-03 downstream PCC/LST parser epics.

### OQ-03-002 — Which pilot syntax edge cases are mandatory in the first importer slice?
Question:
Which PCC and LST syntax edge cases appear in the pilot corpus strongly enough that the first bounded parser epics must support them instead of deferring them?

Recommended answer if known:
Only partially grounded through GE-01 inventory posture. The GE-03 source STC should preserve the question rather than fabricating a complete edge-case list.

Owner if known:
Future parser implementation planning.

Deferred owner:
GE-03 downstream parser epics informed by GE-01 inventory work.

### OQ-03-003 — Where is the registry/handler boundary for high-risk constructs?
Question:
What belongs in generic token-registry metadata versus individual conversion handlers for prerequisites, formulas, bonuses, and choices?

Recommended answer if known:
The boundary must remain explicit, but the exact split is not yet settled.

Owner if known:
Future handler-architecture planning.

Deferred owner:
GE-03 token-registry and token-handler epics.

### OQ-03-004 — What is the minimum acceptable source precision for the first importer milestone?
Question:
Is file-level provenance enough for the first slice, or must the first real parser preserve line numbers and token spans immediately?

Recommended answer if known:
GE-03 requires an explicit downgrade path if the strongest precision is unavailable, but the minimum acceptable threshold is still open.

Owner if known:
Future provenance / parser design work.

Deferred owner:
GE-03 provenance and parser epics.

### OQ-03-005 — What is the minimum useful conversion-report schema?
Question:
Which fields are mandatory in the first auditable conversion report so that coverage claims cannot outrun the evidence?

Recommended answer if known:
The report must at least expose exact/partial/unsupported/ignored posture and validation evidence references, but the final schema is still open.

Owner if known:
Future diagnostics/reporting design work.

Deferred owner:
GE-03 report CLI and diagnostics epic.

### OQ-03-006 — Which GE-02 decisions remain insufficient for implementation handoff?
Question:
Which importer-facing sections can now rely on accepted GE-02 artifacts, and which sections must remain explicitly deferred because GE-02 planning readiness does not settle final schema, evaluator, runtime, branch/worktree, write scope, or verification commands?

Recommended answer if known:
GE-03 can now rely on GE-02 artifacts for canonical model homes, content package layout posture, provenance/source-map obligations, and validation/diagnostic classes. It must still defer final production schemas, expression/evaluator implementation, runtime engine behavior, exact code write scope, and future verification commands.

Owner if known:
Program-level planning owner.

Deferred owner:
Later GE-03 implementation-slice handoff and GE-02/GE-04 runtime decision surfaces.

### OQ-03-007 — What is the GE03-E1-F1 handoff gate state?
Question:
What gate state applies after selecting GE03-E1-F1 as the first candidate implementation slice?

Recommended answer if known:
`artifacts/ge03-e1-f1-execution-readiness-closure-2026-06-19.md` selects GE03-E1-F1 and resolves the policy/environment gates used by `execution-handoff.md`: branch `ge03-e1-f1-pcc-entry-parser` from `main`, exact allowed write scope, Rust/Codex runtime substrate, verification commands, and minimum source-path-plus-line provenance threshold. The closure remains non-code-authorizing; code authority begins only in `execution-handoff.md`.

Owner if known:
Program-level planning owner.

Deferred owner:
Future GE03-E1-F1 execution handoff.

## Intentionally deferred
- final production schema details beyond accepted GE-02 pilot model homes
- exact formula and prerequisite expression-language/evaluator choice
- final CLI syntax for conversion reports
- full parity-harness automation design
- broad Pathfinder or multi-system coverage
- repo-local implementation layout inside `/home/ubuntu/workspace/repos/codex`
- executing the actual GE03-E1-F1 `execution-handoff.md` and reporting implementation evidence

## Forbidden assumptions
- that GE-02 planning readiness authorizes GE-03 to invent final production schemas or runtime semantics
- that unsupported legacy behavior can be ignored until coding begins
- that parser success alone proves semantic conversion success
- that provenance precision may be silently weakened without being reported
- that this source STC alone authorizes repository scaffolding or importer implementation
- that the execution-readiness closure itself authorizes code before a derived `execution-handoff.md` exists

## Review trigger
Reopen this file when any of the following occurs:

- a later GE-02 decision record resolves canonical target-model details beyond the accepted planning artifacts
- GE-01 inventory or taxonomy work grounds new pilot-critical syntax families
- a parser spike proves or disproves the expected source-span precision
- a report schema or unsupported-diagnostic shape becomes concrete enough to promote into requirements
- a future GE-03 implementation handoff is being proposed
