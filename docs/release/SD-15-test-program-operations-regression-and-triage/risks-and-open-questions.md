# SD-15 Test-Program Operations, Regression, and Triage Risks and Open Questions

## Purpose
This document quarantines unresolved SD-15 questions so the source STC stays concrete without pretending the tester-operations lane is fully settled already.

## Open questions

### 1. Triage taxonomy sharpness
- What is the minimum truthful taxonomy that keeps UI defects, rules defects, content/data defects, unsupported semantics, packaging failures, install/use failures, persistence failures, and status drift from collapsing into one pile without creating unusable bureaucracy?
- Which classes require separate GitHub labels or downstream fields, and which can remain operator-only classification state?

### 2. Regression evidence burden
- Which regression fields are mandatory for every issue versus only for classes that implicate distribution, support-state, or persistence behavior?
- What is the minimum acceptable evidence when a tester cannot reproduce reliably or cannot supply all diagnostic context?
- Which regression receipts must be refreshed after a fix versus carried forward as historical context only?

### 3. Clean-machine validation scope
- What exact machine or environment classes count as sufficiently clean for tranche-2 proof?
- How often must clean-machine proof be refreshed when builds, manifests, or support-state claims change?
- Which failures count as install/use defects versus packaging/distribution defects versus status/documentation drift?

### 4. External tester coordination
- What is the minimum truthful external tester cohort for this tranche?
- How much operator support is acceptable during the cycle before the result stops being representative of ordinary bounded testers?
- What communication or evidence-submission path is acceptable without drifting into support-ops sprawl?

### 5. Status reconciliation governance
- Which surface should be updated first when repo README, workspace README, and the execution ledger disagree?
- What drift duration is acceptable before a contradiction becomes a closure blocker?
- Which claims belong in the repo README versus the workspace README versus the execution ledger so the same fact is not rewritten three different ways?

### 6. Automation boundaries
- Which parts of SD-15 should remain documentary operator work permanently?
- Which later slices justify automation or in-product helpers without letting the automation become the authority instead of the evidence?

## Risks

### Risk A — Generic backlog collapse
If SD-15 does not fix a bounded taxonomy, the program will treat unsupported breadth, persistence failures, packaging failures, and UI defects as one homogeneous bug stream.

Mitigation direction:
- keep class definitions explicit and tied to adjacent authority surfaces

### Risk B — Counterfeit regression confidence
If regression evidence is under-specified, fixes will be reported as “seems fine now” without enough context to know what build, workflow, or support-state actually changed.

Mitigation direction:
- require receipt-grade provenance fields for every meaningful regression claim

### Risk C — Authoring-machine optimism
If clean-machine proof is not mandatory, the tranche will inherit the oldest lie in software delivery: “it worked on my machine.”

Mitigation direction:
- preserve clean-machine reports as named closure artifacts, not optional notes

### Risk D — External testing theater
If the program names external testing without exact missions, stop conditions, and result capture, the cycle will generate anecdotes rather than evidence.

Mitigation direction:
- require a plan artifact and a report artifact before any closure claim

### Risk E — Status drift across surfaces
If repo/workspace/ledger truth diverges, a later reader will be able to prove anything and therefore know nothing.

Mitigation direction:
- preserve a reconciliation checklist with blocking drift classes

## Forbidden shortcuts
- do not solve taxonomy ambiguity by calling everything a bug
- do not solve clean-machine ambiguity by downgrading the proof requirement
- do not solve external-testing uncertainty by replacing results with operator summary prose
- do not solve status drift by picking a favorite surface and calling the others stale by definition
