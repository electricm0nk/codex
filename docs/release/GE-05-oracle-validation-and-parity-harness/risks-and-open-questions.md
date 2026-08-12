---
title: GE-05 Risks and Open Questions
stc_id: STC-CODEX-GE-05
artifact_type: risks-and-open-questions
status: draft
scope: programs/codex/requirements/GE-05-oracle-validation-and-parity-harness
source_stc: ./README.md
source_artifacts:
  - ./README.md
  - ./technical-requirements.md
  - ../GE-01-legacy-corpus-and-conversion-matrix/references/oracle-surface-inventory.md
  - ../../plans/spec-domains/GE-05-oracle-validation-and-parity-harness.md
---

# GE-05 Risks and Open Questions

## Primary risks

| ID | Risk | Consequence | Mitigation |
|---|---|---|---|
| R-05-001 | PCGen runtime output path remains ungrounded. | GE-05 cannot produce true oracle-checked evidence and may be limited to static/source hypotheses. | Make PCGen command/oracle discovery the first downstream implementation/discovery slice. |
| R-05-002 | GUI driving becomes the only usable oracle route. | Harness may become brittle, slow, and hard to run in CI. | Prefer CLI, validation, export, script, or test surfaces; require decision record if GUI driving is accepted. |
| R-05-003 | Normalization hides semantic disagreement. | False parity claims become possible. | Require explicit normalization rules, raw-value retention, and blocked/known-gap status for ambiguous transformations. |
| R-05-004 | PCGen behavior is known but undesirable. | Codex may preserve defects or confusing legacy semantics. | Route intentional divergence to `programs/codex/doctrine/decisions/` and record known-gap/intentional-divergence status. |
| R-05-005 | Fixture retention or redistribution is legally ambiguous. | Evidence artifacts may be unsafe to commit or share. | Require fixture-retention decision: direct storage, reduced facts, hashes/references, or generated-on-demand output. |
| R-05-006 | GE-03 or GE-04 outputs lack provenance, diagnostics, or explanation data. | Matching numbers may be unreviewable and lower-trust. | Treat diagnostics/provenance gaps as comparison failures or known gaps until upstream outputs improve. |
| R-05-007 | Parity harness scope expands into broad regression before the pilot proves the architecture. | The work becomes unfinishable and hides root-cause failures. | Keep the first fixture to PF1 Core Rulebook Human Fighter level 1; require expansion decision records. |
| R-05-008 | Current implementation checkout is on an unrelated GE-04 slice with untracked files. | A future GE-05 handoff could accidentally stack on unstable state. | Require every future code handoff to re-ground branch/worktree policy and allowed write scope before execution. |

## Open questions

### OQ-05-001 — Which PCGen command, validation task, export path, or scripting route can produce usable oracle output for the pilot?
Status: unresolved.  
Owner: future GE05-E1 oracle-discovery handoff.  
Notes: GE-01 records candidate and static oracle surfaces, but the runtime character-generation output path was not grounded.

### OQ-05-002 — Which outputs are comparable for the first Human Fighter case?
Status: partially grounded by merged GE05-E2-F1 schema, unresolved for governed case population.  
Owner: GE05-E2-F2 fixture-instance work, with GE-04/GE-06 inputs.  
Notes: The merged schema now carries candidate dimensions for loaded content summary, derived values, choice/prerequisite outcome, and exportable summary. The next truthful step is to turn that schema foothold into the first governed case instance without inventing pass states or final expected values.

### OQ-05-003 — How should old and new outputs be normalized before diffing?
Status: unresolved.  
Owner: GE05-E3 normalizer work.  
Notes: Formatting, ordering, labels, and absent fields may be normalized; semantic disagreement must remain visible.

### OQ-05-004 — What is the policy when PCGen behavior is known but undesirable?
Status: partially answered by policy, unresolved in concrete cases.  
Owner: `programs/codex/doctrine/decisions/` for intentional divergence decisions; GE-05 known-gap policy for report status.  
Notes: No individual undesirable behavior is accepted or rejected by this STC.

### OQ-05-005 — What known gaps are acceptable for pilot viability?
Status: unresolved.  
Owner: GE-05 report policy plus GE-06 pilot viability review.  
Notes: Known gaps may be acceptable only when scoped, visible, and non-fatal to the pilot's proof purpose.

### OQ-05-006 — What evidence is required to upgrade a claim from computed to oracle-checked?
Status: answered at class level, unresolved per behavior.  
Owner: GE-05 parity report format and future comparison artifacts.  
Notes: The class rule is reproducible old-vs-new comparison for exact scoped behavior. Each behavior still needs a concrete report.

### OQ-05-007 — Should PCGen-derived oracle output be committed, reduced, hashed, or generated on demand?
Status: partially answered for the current raw XML reference, unresolved for broader parity artifacts.  
Owner: future fixture-retention decision.  
Notes: The merged GE05-E2-F1 slice now preserves `local_generated_only` plus the raw-output SHA-256 and reduced-facts reference for the first pilot evidence surface. Broader retention policy for later parity artifacts still requires an explicit decision.

### OQ-05-008 — Where should runtime parity reports live in the implementation repo?
Status: unresolved.  
Owner: future code-authorizing GE-05 handoff.  
Notes: This source STC names report schema requirements but does not choose implementation file paths.

## Intentionally deferred
- final PCGen invocation command
- final Codex output command
- final report file extension and implementation storage path
- exact expected values for Human Fighter level 1
- exact normalizer implementation
- GUI automation strategy
- broad regression suite beyond the pilot
- release-governance handling for parity artifacts, which belongs to GE-09

## Forbidden assumptions
- Do not assume static PCGen source files prove runtime behavior.
- Do not assume GE-04 computed output is oracle-checked before GE-05 comparison evidence exists.
- Do not assume non-comparable outputs can be omitted from a report.
- Do not assume matching final numbers are sufficient without diagnostics and provenance.
- Do not assume PCGen behavior is desirable just because it is observable.
- Do not assume future code work may begin from this STC without a route-specific handoff.

## Review trigger
Reopen this risk file when any of the following occurs:

- a PCGen runtime/export/validation command is discovered
- the first Human Fighter old-system output is captured
- the first Codex new-system output is captured
- a normalizer or comparator implementation is proposed
- a parity report identifies a non-comparable or intentionally divergent behavior
- GE-06 consumes GE-05 evidence for the integrated pilot slice
