# SD-15 Test-Program Operations, Regression, and Triage Acceptance and Verification

## Acceptance posture
This is a planning-ready documentary gate. The immediate proof burden is that the SD-15 packet defines tester-program operations honestly and concretely enough for later same-domain story minting without inventing clean-machine, external-test, or tranche-closure success.

## Documentation gate checks

### Gate A — Source STC shape exists
Acceptance:
- `README.md`, `technical-requirements.md`, `technical-design.md`, `acceptance-and-verification.md`, `risks-and-open-questions.md`, and `epic-breakdown.md` exist
- `references/upstream-dependency-contract.md` exists
- all named same-epic output artifacts exist under `artifacts/`

Verification:
- verify the file set exists under `programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/`

### Gate B — Triage and regression authority is concrete
Acceptance:
- the packet explicitly defines a bounded issue taxonomy
- the packet explicitly defines the regression/provenance fields required to reconstruct a claim
- the packet preserves unsupported and partial states as visible classes rather than generic bugs

Verification:
- confirm README `Authority and Scope`, `Acceptance Summary`, and `Out of Scope` preserve the bounded operator lane
- confirm `technical-requirements.md` sections 2 and 3 define taxonomy and evidence fields
- confirm `technical-design.md` names the triage classification and regression evidence surfaces separately

### Gate C — Install/use and clean-machine proof are concrete
Acceptance:
- the packet names a tranche-2 install/use matrix with an exact path
- the packet names a clean-machine validation report with an exact path
- the packet explicitly refuses authoring-machine proof as a substitute

Verification:
- confirm README frontmatter `expected_output_artifacts` matches the `Expected Output Artifacts` table
- confirm `artifacts/tranche-2-install-and-use-matrix.md` and `artifacts/tranche-2-clean-machine-validation-report.md` both exist and carry structured contract content
- confirm `technical-requirements.md` sections 4 and 5 define the proof burden explicitly

### Gate D — External testing is explicit and non-theatrical
Acceptance:
- the packet names both an external-test-cycle plan and an external-test-cycle report with exact paths
- the plan names cohort, missions, evidence, stop conditions, and cadence
- the report names actual execution/result fields and closure implications

Verification:
- confirm `artifacts/tranche-2-external-test-cycle-plan.md` and `artifacts/tranche-2-external-test-cycle-report.md` exist and are not placeholders
- confirm `technical-requirements.md` sections 6 and 7 define launch and reporting obligations separately

### Gate E — Status reconciliation is explicit before closure
Acceptance:
- the packet names repo/workspace/ledger status reconciliation as a first-class same-epic artifact
- the packet names exact surfaces to reconcile
- the packet defines drift classes and a closure-verdict rule

Verification:
- confirm `artifacts/tranche-2-project-status-truth-reconciliation-checklist.md` exists and names the three required status surfaces
- confirm README `Required Reads`, `In Scope`, and `Next Stage Rule` preserve the reconciliation burden
- confirm `technical-requirements.md` section 8 defines required status fields and drift handling

### Gate F — Adjacent authority surfaces remain separate
Acceptance:
- SD-15 does not counterfeit SD-11 tester-workbench authority
- SD-15 does not counterfeit SD-12 distribution/update authority
- SD-15 does not counterfeit SD-13 breadth/support authority
- SD-15 does not counterfeit SD-14 persistence/migration authority

Verification:
- confirm README `Authority and Scope`, `Required Reads`, and `Blockers / Forbidden Assumptions` preserve the SD-11 through SD-14 split
- confirm `references/upstream-dependency-contract.md` names what each adjacent surface does and does not authorize

### Gate G — Repo and program reality are grounded honestly
Acceptance:
- the packet grounds itself in the existing repo/workspace/ledger status surfaces
- the packet does not claim actual clean-machine, regression-suite, or external-test completion
- the packet does not claim tranche closure is already achieved

Verification:
- confirm README `Readiness`, `Closure State`, and `Target Runtime` preserve the currently documentary nature of the lane
- confirm `technical-design.md` `Context and constraints` and `Verification implications` refuse counterfeit closure

### Gate H — Epic decomposition is ready for successor routing
Acceptance:
- every major requirement family routes into at least one bounded epic
- the epic list preserves dependencies and anti-scope-creep boundaries
- no epic is itself an execution handoff

Verification:
- confirm `epic-breakdown.md` includes bounded epics for taxonomy/routing, regression evidence, install/use plus clean-machine proof, external testing, and status reconciliation
- confirm `epic-breakdown.md` ends with an explicit handoff boundary rule

## Future implementation and documentary proof obligations
A later SD-15 handoff is acceptable only when it names exact paths, exact write scope, exact inputs, and exact verification, and proves at least the relevant subset of these obligations:

### Triage-routing proof
- a bounded issue can be classified into the SD-15 taxonomy
- the classification preserves adjacent-authority provenance
- unsupported or partial states are not mislabeled as generic defects

### Regression evidence proof
- a defect or regression receipt carries enough build/platform/channel/workflow/support-state/persistence context to reconstruct the claim
- attachment and redaction posture is explicit

### Install/use and clean-machine proof
- a named build is acquired and exercised against the install/use matrix
- a clean-machine environment is identified explicitly
- step-level outcomes and evidence are captured in a validation report

### External cycle proof
- a real cohort and mission plan exist before execution
- the report records what testers actually did, not what operators hoped they would do
- stop conditions and unsupported-path outcomes remain visible

### Status-reconciliation proof
- repo/workspace/ledger surfaces are compared against current evidence
- any remaining drift is classified explicitly
- the closure verdict states why tranche-2 may or may not be claimed

## Anti-counterfeit rules
- issue creation alone does not prove triage operations exist
- planned regression fields alone do not prove a regression was reproduced or fixed
- an install/use matrix alone does not prove a clean machine succeeded
- an external test plan alone does not prove testers ran anything
- a README update alone does not prove status truth unless reconciliation evidence supports it
