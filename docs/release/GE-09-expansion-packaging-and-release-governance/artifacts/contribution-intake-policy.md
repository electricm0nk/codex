---
title: GE-09 Contribution Intake Policy
stc_id: STC-CODEX-GE-09
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-09-expansion-packaging-and-release-governance/artifacts
source_stc: ../README.md
related:
  - ../../plans/spec-domains/GE-08-homebrew-authoring-and-rules-studio.md
  - ../../GE-08-homebrew-authoring-and-rules-studio/README.md
  - ../../GE-08-homebrew-authoring-and-rules-studio/artifacts/package-file-lifecycle-requirements.md
  - ../../GE-08-homebrew-authoring-and-rules-studio/artifacts/validation-and-preview-workflow-requirements.md
  - ./package-compatibility-and-migration-policy.md
---

# GE-09 Contribution Intake Policy

## Purpose
Define when Codex may accept package contributions, what evidence and provenance gates must be satisfied before a submission is even reviewable, and which contribution lanes remain closed until GE-08 earns narrower authoring authority.

## Governing truths and current ceiling
This policy inherits five decisive truths:
- GE-08 now exists as a planning-ready source STC, so GE-09 must stop acting as if package authoring posture is missing.
- GE-08's package lifecycle artifact fixes the intended first-proof substrate as a deterministic directory-backed YAML source bundle with explicit `manifest.yaml`, package-local stable IDs, `metadata/provenance.yaml`, and `metadata/diagnostics.yaml`.
- GE-08's validation workflow artifact fixes a refusal-first state model: `draft`, `valid`, `invalid`, and `deferred` are distinct package states with different consequences for preview, explanation, and export.
- GE09-E2 already separated `schema_version`, `package_version`, and stable-ID continuity, so contribution intake may not collapse versioning truth into a vague "looks fine" review.
- None of those truths yet authorize a settled collaborator workflow, public contribution portal, or broad community package exchange.

Therefore contribution governance is no longer ungrounded, but it remains deliberately narrow.

## Intake classes and current availability
| Intake class | Who this means | Current posture | Why |
|---|---|---|---|
| Operator-owned proof package | Todd or an explicitly assigned internal operator working inside a named bounded GE-08 / GE-09 implementation or readiness slice. | Conditionally reviewable. | This is not open contribution intake; it is execution-scoped package work tied to a named proof lane and review owner. |
| Trusted collaborator package | A specifically invited collaborator submitting a package for Codex evaluation outside Todd-only operator work. | Closed for normal intake. | GE-08 has not yet produced a narrower accepted authoring execution posture plus intake authority for collaborator workflow. |
| Broad community package | Any generally available or public package submission path. | Refused. | GE-08, release authority, trust/distribution, and review-operations posture are not mature enough for honest public intake. |

## Current allowed path
The only currently admissible package-intake path is the narrowest one: an operator-owned proof package produced inside a named bounded work lane with explicit review ownership.

That means all of the following must already be true:
1. the package belongs to an active GE-08 or successor slice whose scope is explicitly bounded
2. the package is being submitted by the named operator for that slice rather than by an ungoverned contributor pool
3. the submission requests bounded internal review only, not public compatibility, distribution, or marketplace posture
4. the package stays within the currently authorized proof semantics instead of widening into arbitrary authoring, plugin-default behavior, or broader package classes

Anything broader remains closed until GE-08 and later doctrine decisions narrow the authority surface.

## Mandatory intake bundle
Any package submission that asks to enter review MUST provide an inspectable source bundle, not screenshots, prose, or opaque runtime state.

Minimum required bundle:
- deterministic directory-backed source package shaped by the GE-08 lifecycle contract
- `manifest.yaml` with at least `schema_version`, `package_id`, `package_version`, dependency declaration, and current `validation_state`
- package-local stable IDs for every contributed record
- `metadata/provenance.yaml` naming authored-source lineage without requiring absolute local paths
- `metadata/diagnostics.yaml` carrying machine-readable current diagnostics state
- explicit statement of package class and bounded scope being requested for review
- explicit request posture: diagnostic review only, bounded internal proof review, or future compatibility-language review
- named intake owner and named review owner

If the submission lacks that bundle, the correct result is refusal, not manual heroics.

## Validation-state gate
GE-08's validation posture is already specific enough to govern intake consequences.

| Package state | Intake consequence | Allowed claim |
|---|---|---|
| `draft` | May be shared only for diagnostic help inside a bounded internal lane. It is not accepted as review-complete intake. | No preview, export, compatibility, or release claim. |
| `valid` | Eligible for bounded internal review if the rest of this policy is satisfied. | At most the exact bounded internal-proof posture requested; no broader public claim. |
| `invalid` | Refused for intake until the submission is repaired and diagnostics are visible. | No proof, preview, export, compatibility, or release claim. |
| `deferred` | May be logged as known deferred debt or research material, but not accepted as a proof-bearing contribution. | Explicit deferral only; no compatibility or release claim. |

The decisive rule is simple: non-`valid` packages may be preserved for work-in-progress or diagnostic purposes, but they may not enter Codex as accepted proof-bearing package contributions.

## Provenance and identity gates
A reviewable submission MUST preserve identity and source truth.

Required gates:
- package identity must be explicit and durable through `package_id`
- record stable IDs must remain visible and continuous rather than being regenerated opportunistically
- dependency posture must be explicit in the manifest instead of hidden in local workspace assumptions
- provenance must show where authored records came from and which source files own them
- diagnostics must remain machine-readable and linked to the authored package rather than pasted as ad hoc prose
- if the package claims pilot-facing preview or explanation relevance, the bounded proof binding or equivalent scoped target must be named explicitly

A submission that cannot explain what it is, where it came from, and which package records it changes is not intake-ready.

## Scope and semantics gates
Contribution intake is also blocked when the package widens beyond the currently earned semantic posture.

Refuse intake when any of these are true:
- the package requires arbitrary scripting or opaque plugin behavior as the default interpretation path
- the package widens beyond the currently bounded proof or readiness slice without an accepted successor authority surface
- prerequisite or rules semantics are carried only in free-form prose where GE-08 requires structured data
- the package asks reviewers to ignore missing provenance, unresolved diagnostics, or unstable IDs "for now"
- the submission tries to smuggle release, registry, or distribution promises through package review

## Compatibility-language gate
Package intake and package compatibility are separate authorities.

Even a reviewable submission may not claim more than GE09-E2 allows.

Therefore any future submission or review note that uses compatibility language MUST name:
- exact package scope
- package class
- evidence-tier ceiling
- version surface being discussed
- known-gap posture
- migration posture

If that grammar is absent, the contribution may still be reviewed as a bounded source package, but it must not be described as "compatible."

## Review ownership rules
No package enters review without named responsibility.

Minimum required review posture:
- one named intake owner responsible for the submission bundle
- one named review owner responsible for acceptance/refusal
- one named governing lane, artifact, or decision surface that explains why this package is being reviewed now
- explicit disposition outcome: accepted for bounded internal proof, refused, or logged as deferred/diagnostic-only

Unowned queue growth is not governance. It is entropy.

## Refusal conditions
Refuse package intake immediately when any of the following is true:
1. the contributor class is trusted-collaborator or broad-community and no narrower accepted GE-08 execution posture plus intake authority decision has opened that lane
2. the submission is not an inspectable deterministic source bundle
3. `manifest.yaml`, provenance, diagnostics, stable IDs, dependency posture, or review ownership is missing
4. package state is `invalid`, `draft`, or `deferred` and the request is anything stronger than diagnostic-only sharing
5. the package depends on arbitrary scripting, opaque plugins, or hidden runtime cache state
6. the package widens beyond the bounded proof scope or asks for public compatibility/distribution posture
7. the submission uses vague compatibility or migration language instead of the GE09-E2 grammar

## Future-open conditions
Trusted collaborator intake may be reconsidered only when all are true:
- GE-08 has produced a narrower accepted authoring execution posture rather than only planning-ready documentary truth
- the validation/diagnostic substrate is real enough to reject malformed or widened packages without operator guesswork
- intake ownership and review operations are assigned explicitly
- any doctrine decisions needed for trust, distribution, or release posture are named
- the allowed package classes and claim ceilings for collaborator submissions are explicit

Broad community intake remains closed beyond that point until public-distribution, trust, and release-authority surfaces exist.

## Explicit non-authorizations
This policy does not authorize:
- a public package registry or package marketplace
- package-signing or trust-network mechanics
- general collaborator upload rights
- treating GE-08 planning artifacts as proof that contributor tooling already works
- treating local saveability as proof of preview, migration, export, or compatibility success
- broad Pathfinder package-sharing claims

## Completion rule
This policy is complete for GE09-E5 only if a future session can answer, without improvising:
1. who may submit a package right now
2. what bundle, provenance, and diagnostics surfaces must exist before review starts
3. how `draft`, `valid`, `invalid`, and `deferred` change intake consequences
4. which contributor lanes remain closed and why
5. what exact truths must change before collaborator or community intake becomes honest
