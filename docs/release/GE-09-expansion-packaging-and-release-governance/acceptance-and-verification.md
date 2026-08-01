# GE-09 Acceptance and Verification

## Acceptance criteria

### A. Source STC integrity
- The GE-09 source STC bundle exists at `programs/codex/requirements/GE-09-expansion-packaging-and-release-governance/`.
- `README.md`, `technical-requirements.md`, `technical-design.md`, `acceptance-and-verification.md`, `risks-and-open-questions.md`, and `epic-breakdown.md` all exist and remain internally consistent.
- The bundle explicitly names its required same-epic documentary outputs rather than satisfying itself only with the control bundle.

### B. Evidence-driven expansion posture
- The bundle defines how expansion candidates are selected by grounded evidence, risk, and compatibility posture.
- The bundle defines a ranked candidate-band policy and review cadence rather than a vague encouragement to expand later.
- The bundle explicitly permits the top-ranked outcome to be “hold broadening” when the evidence ceiling is too low.
- The bundle names GE-01 conversion-matrix and unsupported-token-ledger truth as required inputs.

### C. Honest dependency and release posture
- The bundle explicitly preserves GE-06's current established verdict: `computed-but-not-oracle-checked`, not `pilot-viable`.
- The bundle explicitly preserves that GE06-E5-F2 / E5-F3 route the next mandatory proof burden to GE-05 parity ownership.
- The bundle explicitly consumes that GE-08 now has a planning-ready source STC while still refusing to treat that as contributor-workflow or implementation authority.
- The bundle does not assign counterfeit release authority.

### D. Compatibility and known-gap truth
- The bundle defines compatibility language scoped by evidence tier, package, or token family.
- The package policy names distinct package classes, version surfaces, downgrade rules, and migration obligations instead of using undifferentiated "compatible" language.
- The bundle requires visible known-gap and regression inputs before a stronger claim is made.
- The bundle preserves downgrade/block behavior when evidence weakens.
- The bundle preserves that authored-package migration and downgrade guarantees remain provisional while GE-08 is still planning-ready only.

### E. Contribution and packaging restraint
- Contribution intake remains provisional behind narrower future authoring truth and any required doctrine decisions.
- Cross-platform packaging remains a milestone consequence of proven behavior, not a substitute for that proof.

## Verification steps for this planning pass
1. Verify that every required control-bundle file exists.
2. Verify that every required output artifact exists under `artifacts/`.
3. Verify that `README.md` and `references/provisional-dependency-posture.md` state the current GE-06 and GE-08 posture accurately.
4. Verify that `technical-requirements.md` ties expansion and compatibility claims to evidence tiers, known-gap posture, and rerank triggers.
5. Verify that `artifacts/expansion-scope-selection-policy.md` lists the current hold/go gate, ranked candidate bands, and review cadence.
6. Verify that `artifacts/release-milestone-model.md` defines milestone classes without inventing final release authority.
7. Verify that `artifacts/contribution-intake-policy.md` keeps contribution posture provisional despite GE-08 now existing as a source STC.
8. Verify that no file in this STC claims public release readiness, broad package compatibility, or code authority.

## Required evidence for later operational promotion
A later GE-09 operational or implementation handoff may be considered only when all are true:
- GE-05 has produced parity evidence or explicit accepted-known-gap posture sufficient to raise or narrow the claim ceiling for the scoped package
- GE-06's propagated posture for the relevant scope is explicit and current
- GE-08 has at least a narrower readiness closure or accepted decision surface for the authoring/contribution path being invoked
- release-authority, package-signing, or public-distribution questions that materially affect behavior have a named decision surface
- the bounded runtime or operational surface to modify is explicit
- exact verification receipts are named

## Failure conditions
This STC fails acceptance if any of the following are true:
- it speaks about expansion without naming evidence inputs
- it speaks about compatibility without tiered claim language
- it speaks about release without explicit gate posture
- it speaks about contribution intake as if GE-08's existence alone settled the workflow
- it authorizes code, publication, or release operations directly

## Completion rule
GE-09 is complete for this pass when the planning bundle exists, its same-epic artifact contract exists, and the bundle preserves evidence-first governance while truthfully consuming the current GE-06 / GE-08 posture instead of pretending those surfaces are still missing or already solved.
