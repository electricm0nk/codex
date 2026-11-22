---
title: GE-09 Release Milestone Model
stc_id: STC-CODEX-GE-09
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-09-expansion-packaging-and-release-governance/artifacts
source_stc: ../README.md
related:
  - ../../../doctrine/quality-gate-policy.md
  - ../../../doctrine/decisions/README.md
  - ../../GE-05-oracle-validation-and-parity-harness/artifacts/known-gap-policy.md
  - ../../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/pilot-stack-viability-decision-criteria.md
  - ../../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e5-f1-viability-domain-confidence-decision-2026-06-22.md
  - ../../GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-post-e5-f2-decision-rack-2026-06-22.md
  - ../../GE-08-homebrew-authoring-and-rules-studio/README.md
---

# GE-09 Release Milestone Model

## Purpose
Define the milestone classes, receipt bundle, and authority checks that separate documentary progress, bounded previews, package-level compatibility language, and future releases.

Milestones are claim ceilings, not morale labels.
A green build, merged branch, or generated package is evidence.
None of those artifacts alone grants the right to speak as though Codex is released.

## Current non-negotiable posture
The current upstream evidence sharply limits what GE-09 may authorize today:
- GE-06 explicitly remains `computed-but-not-oracle-checked`, not `pilot-viable`.
- GE06 post-E5-F2 routing makes the next mandatory proof burden parity closure under GE-05 ownership, not broad release narration.
- GE-08 is planning-ready, not execution-ready, and does not yet authorize a settled contributor or authored-package lifecycle.
- GE-05 requires known gaps to stay visible; omitted comparison failures invalidate a release claim rather than merely weakening it.
- `programs/codex/doctrine/decisions/` exists as the doctrine surface for accepted release/packaging decisions, but no accepted release-authority record is cited by this artifact today.

Therefore the strongest truthful current GE-09 release class is:

```text
documentary governance state
```

A narrower internal proof receipt may exist for a named scope when evidence exists.
A public or broadly compatible release posture does not.

## Milestone classes

| Class | Allowed language ceiling | Minimum evidence gates | Required receipt bundle | Mandatory refusal conditions |
|---|---|---|---|---|
| Documentary governance state | "policy drafted", "decision surface defined", "scope ranked", "release not yet authorized" | Documentation gate only, plus links to the upstream evidence that sets the current claim ceiling | scope receipt, blocker receipt | Refuse if anyone tries to translate documentary completion into runnable-package, compatibility, or public-release language |
| Internal proof receipt | "the named scope computes", "the named comparison route exists", "for developer/operator consumption only" | Documentation gate plus the exact headless/import/rules evidence required by the named scope; if parity is absent it MUST remain absent in the language | scope receipt, gate receipt, known-gap receipt | Refuse if scope is unnamed, if diagnostics are hidden, or if the wording implies public compatibility or distribution |
| Controlled pilot preview | "bounded preview of the exact pilot slice", "operator-visible" or "Todd-authorized preview" | Internal proof evidence plus an explicit GE-06 posture for the slice; parity posture named honestly; UI truth gate only for the workflows actually shown | scope receipt, gate receipt, known-gap receipt, surface receipt, audience/limitation receipt | Refuse if the preview uses mocks, outruns the exact slice, hides a primary blocker, or speaks as though preview means released |
| Compatibility-scoped package preview | "preview-compatible for the exact named package/token family at the stated evidence tier" | Controlled preview-level honesty for the named scope plus expansion-gate inputs, compatibility-tier declaration, known-gap effect, and migration/downgrade posture; authored-package claims also require GE-08 readiness narrow enough for that path | scope receipt, gate receipt, known-gap receipt, compatibility receipt, migration receipt, provenance receipt when authoring is involved | Refuse if the exact package or token family is not named, if the claimed tier outruns the evidence, if downgrade behavior is undefined, or if authored-package workflow is still only planning prose |
| Release candidate | "candidate for release of the exact named scope" | The target scope has reached every quality gate required by its claim language, including parity or accepted intentional divergence where relevant, product-visible proof for surfaced workflows, and verified packaging on the named distribution surfaces | scope receipt, gate receipt, known-gap receipt, compatibility receipt, migration receipt, packaging receipt, authority receipt | Refuse if packaging/install/rollback proof is missing, if blockers are merely renamed as caveats, or if no accepted release-authority decision authorizes candidate publication |
| Supported release | Public release/support language for the exact named scope and nothing broader | Release-candidate evidence plus explicit accepted authority for publication/support, release notes that preserve known-gap truth, and regression/watch posture for the named scope | scope receipt, gate receipt, known-gap receipt, compatibility receipt, migration receipt, packaging receipt, authority receipt, release-notes receipt | Refuse if the claim extends beyond the exact scope, if new regressions are open without being surfaced, if authority is implicit, or if support language outruns parity truth |

## Standard receipt bundle
Every non-documentary milestone must assemble a durable receipt bundle. Later automation may collect these receipts, but automation does not get to waive them.

### 1. Scope receipt
Must name:
- exact package, token family, workflow, or pilot slice
- exact paths or artifacts being discussed
- exact claim language ceiling for that scope
- explicit statement of what is out of scope

If the scope sentence is vague, the milestone is invalid.

### 2. Gate receipt
Must record the status of each applicable gate from `quality-gate-policy.md`:
- documentation
- headless behavior
- import fidelity
- rules correctness
- oracle parity
- UI truth
- expansion

The receipt must say `pass`, `blocked`, or `not applicable for this class` per gate, with linked evidence.
Silence is refusal.

### 3. Known-gap receipt
Must link the GE-05 known-gap surface and state:
- every gap affecting the scoped claim
- whether the gap blocks, narrows, or merely annotates the claim
- who owns the gap
- what review trigger changes the milestone posture

An omitted known gap is a counterfeit pass.

### 4. Surface receipt
Required whenever a user-facing or operator-facing workflow is shown.
Must prove the surfaced workflow runs on real outputs, explanations, and diagnostics rather than mock state or hardcoded examples.

### 5. Compatibility receipt
Required whenever the language includes `compatible`, `preview-compatible`, `supported`, or any similar promise.
Must name:
- the exact compatibility tier being claimed
- the exact outputs proven at that tier
- the comparison artifact or accepted reason the comparison ceiling stops lower
- any intentionally divergent behavior and the decision record that authorizes it

### 6. Migration receipt
Required whenever a package, version, or release can supersede another one.
Must name upgrade, downgrade, and rollback behavior for the exact scoped artifact.
If rollback is undefined, the milestone cannot rise above preview.

### 7. Provenance receipt
Required for authored packages, collaborator submissions, or any future contribution intake.
Must name who produced the package, what provenance is visible, what validation path applies, and what review owner accepted the intake.
GE-08 planning-ready posture alone cannot satisfy this receipt.

### 8. Packaging receipt
Required for release-candidate and supported-release classes.
Must name:
- build artifact identities
- target platforms
- installation path
- uninstall or rollback path
- any signing/checksum/trust surface actually in effect

A package that exists but cannot be installed, verified, or backed out is not release-ready.

### 9. Authority receipt
Required whenever language moves beyond internal proof.
Must identify:
- the accepted decision record(s) that define release authority for this class
- the human or delegated role allowed to cut the release
- the approval event or checkpoint
- any expiry, override, or revocation rule

Merged code is not an authority receipt.
A built package is not an authority receipt.
A chat statement is not an authority receipt.

### 10. Release-notes receipt
Required for supported release.
Must preserve scope, known gaps, compatibility ceiling, migration expectations, and any blocked or intentionally divergent behavior still visible at release time.

## Authority model
Release authority is a separate control plane from implementation progress.

### Evidence owners
These surfaces produce evidence but do not grant shipping rights:
- GE-01 establishes conversion and unsupported-surface truth.
- GE-05 establishes parity and known-gap truth.
- GE-06 establishes the current integrated pilot claim ceiling.
- GE-08 establishes authoring and contribution-boundary truth.
- future packaging automation may establish artifact reproducibility.

### Release authority owner
Until an accepted doctrine decision says otherwise, Todd remains the only honest release authority for Codex milestone promotion beyond documentary or internal-proof classes.

This artifact does not delegate that authority.
It defines the conditions a future accepted decision would have to satisfy before delegation becomes truthful.

### Required future decision surfaces
No future class above internal proof should be promoted without accepted doctrine records covering, at minimum:
1. who may authorize controlled previews, release candidates, and supported releases
2. what scope each authority may cover
3. what evidence receipts are mandatory before that authority may act
4. what signing, trust, or publication surfaces are actually in force
5. how intentional divergence is disclosed when compatibility language is still used
6. what rollback or withdrawal authority exists if later evidence invalidates a release claim

The existing `doctrine/decisions/README.md` trigger surface is the correct place for those records.

## Refusal rules
A milestone MUST be refused when any of the following is true:
- the claim scope is broader than the evidence scope
- the language tier is higher than the proven compatibility tier
- a known gap that affects the claim is missing from the receipt bundle
- UI-visible claims front mock state instead of real outputs
- authored-package or contribution claims rely only on GE-08 planning posture
- installation, downgrade, or rollback behavior is unknown for a class that implies distribution
- authority is implied from implementation momentum rather than cited from accepted doctrine
- a later artifact tries to convert this model itself into shipping authority

## Current truthful ceiling for GE-09
Given today's evidence, future sessions should interpret Codex release posture this way:

| Scope today | Highest honest class now | Why |
|---|---|---|
| GE-09 policy documents | Documentary governance state | These artifacts define rules; they do not prove runnable compatibility or release readiness |
| Named internal proof surfaces backed by real tests/receipts | Internal proof receipt | Some bounded scopes may compute or compare honestly without authorizing public claims |
| Public pilot release, package preview, or supported distribution | Not authorized | GE-06 remains below parity closure, GE-08 remains planning-only, and no accepted release-authority doctrine is cited here |

## No counterfeit shipping rights
The following artifacts are necessary in future release work, but none is sufficient on its own:
- merge receipts
- green tests
- generated packages
- UI demos
- parity reports
- planning completion of GE-09 itself

Shipping rights begin only when evidence receipts and authority receipts both exist for the same exact scope.

## Completion rule
This model is complete for GE09-E3 only if a future session can answer five questions without improvising:
1. What milestone class is being claimed?
2. What exact evidence receipts are mandatory for that class?
3. Who has authority to promote that class?
4. What conditions force refusal instead of optimistic narration?
5. Why do merged code or generated packages still fail to count as release authority by themselves?
