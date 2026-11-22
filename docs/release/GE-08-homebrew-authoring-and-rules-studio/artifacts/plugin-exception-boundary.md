---
title: GE-08 Plugin Exception Boundary
stc_id: STC-CODEX-GE-08
artifact_type: generated-artifact
status: draft
scope: programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts
source_stc: ../README.md
related:
  - ../technical-requirements.md
  - ../risks-and-open-questions.md
  - ../../../doctrine/quality-gate-policy.md
  - ../../../doctrine/decisions/ADR-0001-plugin-exception-path.md
---

# Plugin Exception Boundary

## Purpose
Define the boundary between ordinary GE-08 structured authoring and the narrow future cases that might justify plugin treatment.

## Decision summary
- Ordinary customization stays inside structured package authoring, constrained expressions, validation, preview, explanation, and portable lifecycle surfaces.
- No currently known GE-08 first-proof or routine homebrew case justifies plugin runtime treatment.
- A suspected exception routes first to research and doctrine review, not straight to implementation.
- Plugins remain exceptional even if later allowed. They are never the default answer to missing ergonomics, incomplete model work, or pressure to move faster than the evidence allows.

## Ordinary structured path: what stays inside GE-08
The following needs remain ordinary structured-authoring work, not plugin work:

| Need shape | Correct route | Why it is not a plugin exception |
|---|---|---|
| Package manifest, identity, versioning, provenance, diffability, import/export, and lifecycle rules | GE-08 package/lifecycle requirements | These are core authored-package responsibilities. Hiding them behind plugins would destroy portability and reviewability. |
| Bounded authored objects like the GE08-E1 feat-like proof object | GE-08 ordinary authoring slices | The first proof case was explicitly selected to be satisfiable without plugins. |
| Structured effects, prerequisites, formulas, and choice records that can be represented as data or constrained expressions | GE-08 + GE-02 model/expression work | The ordinary path is required to cover safe structured semantics before any plugin discussion begins. |
| Unsupported or ambiguous semantics that can be recorded as explicit diagnostics, deferred records, or modeling debt | GE-02/GE-08 diagnostic and debt surfaces | Honest refusal is preferable to silently widening into executable extensions. |
| Missing canonical object homes, fields, or relationships for otherwise declarative rules content | GE-02 source-STC/model evolution | A missing model home is a canonical-model gap, not a plugin license. |
| Preview, explanation, and validation gaps | GE-04/GE-08 bounded downstream work | Engine-facing truth surfaces must stay headless, inspectable, and evidence-backed. |
| Product-visible authoring ergonomics, editor flow, shell layout, and diagnostics presentation | GE-07 + GE-08 product-surface work | UI/editor inconvenience is not proof that runtime plugins are needed. |

## Non-justifications
A plugin exception is not justified merely because:
- the structured path is still immature or inconvenient
- a rule is annoying to model cleanly the first time
- the team wants to outrun GE-02 or GE-04 by embedding behavior directly
- GE-07 editor decisions are still unsettled
- a broad extension story feels architecturally elegant
- unsupported semantics exist but can still be surfaced honestly as diagnostics or deferred debt
- a spike or prototype would be easier if arbitrary code were allowed

## Qualifying threshold for a genuine exception
Plugin treatment may be considered only if every statement below is true:
1. The exact authoring need is concrete, bounded, and important enough to justify non-ordinary treatment.
2. The need cannot be represented safely through GE-02 canonical content, GE-08 structured fields, or the constrained-expression posture without breaking determinism, inspectability, or portability.
3. A narrower fix was evaluated first and rejected with evidence:
   - GE-02 model extension
   - GE-08 structured-surface refinement
   - deferred/unsupported diagnostic path
   - narrower preview or explanation bridge work
4. The capability cannot be postponed as explicit unsupported debt because the intended scope genuinely requires execution now.
5. The proposed exception can still preserve a minimum contract for:
   - deterministic or otherwise explicitly bounded behavior
   - provenance and source identity
   - machine-readable diagnostics
   - reviewable failure states
   - isolation from the ordinary portable package path
6. The exception is documented as a separate research and doctrine decision surface before any runtime implementation starts.

If any item above fails, the answer is not "plugin later." The answer is "keep the work in the structured path, or keep it explicitly unsupported for now."

## Candidate exception classes
No class below is authorized today. These are the only kinds of future cases that could even qualify for review:

| Candidate class | Why it might qualify | Why it is still exceptional |
|---|---|---|
| Truly unmodelable runtime semantics | A later slice may prove that a required rule behavior cannot be expressed through accepted canonical structures or constrained expressions without collapsing safety/explainability. | This is an architecture failure or boundary case, not ordinary homebrew. It requires evidence and doctrine review first. |
| Narrow host-capability bridge | A future bounded capability may require a host-side operation that cannot live inside a portable authored package while still needing a governed integration path. | This breaks the ordinary package-portability posture and therefore must stay outside routine authoring. |
| Research-only extension spike | A bounded spike may be needed to prove whether a suspected model gap is real before the program broadens scope. | Research is not runtime authorization. The spike exists only to answer the boundary question. |

## Minimum contract if a future exception is reviewed
Any future plugin candidate must come with a review packet that names:
- the exact user need and why it matters to the approved scope
- the exact ordinary-path options considered and why they failed
- the proposed extension boundary and what remains outside it
- the deterministic/sandbox/provenance/diagnostic contract
- the effect on portability, package exchange, and reviewability
- the verification plan and expected receipts
- the rollback or refusal path if the exception proves too broad

## Governance route
When structured authoring appears insufficient, use this route:
1. Record the gap in GE-08 or GE-02 as a concrete requirement/model problem, not as an immediate implementation request.
2. Prove that the case is outside the ordinary path with explicit evidence tied to the exact rule behavior.
3. Produce a research artifact or bounded spike showing why narrower data-first approaches fail.
4. Open or update a doctrine decision record under `programs/codex/doctrine/decisions/`.
5. Only after that decision surface is accepted may a separate bounded execution handoff be created.

Current doctrine proposal: `../../../doctrine/decisions/ADR-0001-plugin-exception-path.md`.

## Current status for GE-08
For the current GE-08 planning boundary:
- plugin runtime implementation is not authorized
- the first feat-like proof object must remain plugin-free
- unsupported semantics should become explicit diagnostics, deferred records, or narrower model work before any plugin path is discussed
- future plugin conversation belongs to research and ADR review, not to the ordinary authoring backlog by default
