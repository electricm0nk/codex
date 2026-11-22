---
title: GE-09 Package Compatibility and Migration Policy
stc_id: STC-CODEX-GE-09
artifact_type: generated-documentary-output
status: draft
scope: programs/codex/requirements/GE-09-expansion-packaging-and-release-governance/artifacts
source_stc: ../README.md
related:
  - ./expansion-scope-selection-policy.md
  - ./release-milestone-model.md
  - ../../../doctrine/quality-gate-policy.md
  - ../../GE-05-oracle-validation-and-parity-harness/artifacts/known-gap-policy.md
  - ../../GE-08-homebrew-authoring-and-rules-studio/README.md
  - ../../GE-08-homebrew-authoring-and-rules-studio/artifacts/package-file-lifecycle-requirements.md
---

# GE-09 Package Compatibility and Migration Policy

## Purpose
Define how Codex may speak about package compatibility, version evolution, downgrade behavior, and migration without spending authority the current pilot and authoring stack have not earned.

## Governing truths and current ceiling
This policy inherits four decisive current truths:
- GE09-E1 ranks `no scope-broadening launch` as the current top outcome while GE-06 remains `computed-but-not-oracle-checked` and GE-05 parity closure is still the next mandatory proof burden.
- The quality-gate policy already fixes the compatibility claim tiers: `Observed`, `Parsed`, `Converted`, `Computed`, `Oracle-checked`, and `Product-visible`.
- GE-05 already defines the known-gap truth model that must lower or block claims when comparison, computation, or support posture weakens.
- GE-08 now grounds the intended authored-package substrate, but only at a planning-ready documentary level. It does not yet authorize claims that authored packages can be created, loaded, migrated, downgraded, or exchanged successfully in a real implementation.

Therefore this policy is a ceiling-setting document, not a promise of operational package tooling.

## What “compatible” is allowed to mean
A compatibility statement is honest only when it names all of the following:
- exact package scope or token-family scope
- package class
- evidence-tier ceiling
- current version surface being discussed
- known-gap posture
- migration posture

If any of those fields are missing, the statement is too broad and must not use the word `compatible`.

## Package classes

| Package class | What the class means | Highest claim GE-09 may allow today |
|---|---|---|
| Imported legacy-package scope | A named PCGen-derived package, book slice, or token-family scope being parsed, converted, computed, or compared. | Only the exact evidence tier proven for the named scope. No broader "supports Pathfinder" language. |
| Codex-authored package scope | A native authored package defined by the GE-08 package-lifecycle posture. | Planning/documentary only until GE-08 yields narrower execution truth and implementation receipts exist. Version rules may be defined now, but runtime success may not be implied. |
| Intentionally divergent scope | A named package or rule surface where Codex chooses not to preserve PCGen behavior. | Only allowed when linked to an accepted doctrine decision record; divergence must never be hidden as normal compatibility. |
| Deprecated or downgraded scope | A scope previously described more strongly but later weakened by regressions, new known gaps, or a narrower decision. | The claim ceiling must be lowered immediately and visibly; silence is invalid. |

## Mandatory claim grammar
Every future compatibility surface — dashboard row, release note, migration note, package manifest note, or operator summary — MUST use this grammar:

```text
For <exact package / token family / bounded source scope>, Codex currently claims <evidence tier> compatibility for <named outputs or behaviors>, at <version surface>, with <known-gap posture>, and <migration posture>.
```

Examples of acceptable claim shape:
- `For PF1 Core Rulebook Human feat-import scope, Codex currently claims Converted compatibility for the named feat records at package_version 0.2.0, with visible known gaps, and no authored-package migration promise.`
- `For package pf1.homebrew.proof.guard-stance, Codex currently claims documentary-only schema compatibility for schema_version 1, with no runtime migration promise yet.`

Prohibited claim shape:
- `This package is compatible.`
- `Codex supports Pathfinder packages.`
- `Upgrade is seamless.`
- `Downgrade is safe.`

## Version surfaces
GE-09 must keep three different version surfaces separate.

### 1. `schema_version`
`schema_version` describes the authored package-file contract itself.

Rules:
- It changes only when the package-file contract changes.
- A `schema_version` change is a compatibility event even if the package's gameplay meaning did not change.
- If no accepted loader or translator exists across a schema boundary, the result is `migration-required`, not `compatible`.
- While GE-08 remains planning-ready only, `schema_version` semantics are documentary intent, not proof that a real loader already honors them.

### 2. `package_version`
`package_version` describes the content revision of a named package.

Provisional semantics:
- `major` change: breaks prior content expectations, removes or renames stable identities without a verified migration bridge, changes required dependencies incompatibly, or otherwise requires explicit migration guidance.
- `minor` change: adds or extends bounded content inside the same declared schema and claim ceiling without forcing migration for already-valid consumers.
- `patch` change: corrects defects, diagnostics, metadata, or narrowly scoped content details without widening the package contract.

Guardrail:
- Version labels alone do not prove upgrade safety. If the migration surface is absent, the package must be described as `migration-undefined` even when a semantic version exists.

### 3. Stable IDs and identity continuity
Stable IDs are not version numbers, but they are part of compatibility truth.

Rules:
- Package ID continuity and record stable-ID continuity must survive normal edit/save/load/export cycles.
- Title renames, file moves, or serialization cleanup must not be misrepresented as new package identity.
- If stable IDs are intentionally replaced or remapped, that is a migration event and must be documented explicitly.
- A claim of backward compatibility is invalid if identity continuity changed invisibly.

## Downgrade rules
Downgrade behavior must distinguish claim downgrades from artifact rollbacks.

### A. Claim-tier downgrade
A claim-tier downgrade happens when evidence weakens.

Triggers include:
- a new GE-05 known gap blocks prior comparison
- a regression invalidates a previously proven output
- a package dependency changes the effective behavior ceiling
- a doctrine decision narrows what counts as acceptable compatibility

Required actions:
1. lower the public/documented evidence-tier ceiling immediately
2. record the blocking gap or regression explicitly
3. mark the affected scope as downgraded in the dashboard/release surface
4. stop using any stronger prior claim language for that scope

### B. Artifact/version rollback
A rollback or downgrade across package artifacts is honest only when all are named:
- target from-version and to-version
- whether `schema_version` changes
- whether stable IDs remain continuous, are remapped, or break
- whether data loss, manual repair, or feature removal is expected
- verification evidence or the explicit reason verification is not yet available

If those facts are missing, the only honest label is `rollback behavior undefined`.

## Migration obligations
No package evolution may be called compatible across versions unless a migration surface exists for the named scope.

The minimum migration surface for any release, preview, or handoff that claims cross-version compatibility MUST name:
- exact package ID or bounded source-package scope
- package class
- from-version and to-version
- affected object kinds or token families
- `schema_version` posture
- stable-ID continuity posture
- dependency changes
- automated migration steps, if any
- manual operator steps, if any
- known lossiness, divergence, or data-loss risk
- rollback/downgrade stance
- verification artifact or explicit deferred-proof reason

If this surface does not exist, the package may still be versioned, but it must be described as one of:
- `migration-undefined`
- `migration-required`
- `not downgrade-safe`

## Imported legacy-package rules
Imported legacy-package compatibility is not the same thing as authored-package compatibility.

Rules:
- Imported legacy-package claims must name the exact source package, book slice, or token-family scope.
- `Parsed`, `Converted`, `Computed`, or `Oracle-checked` claims apply only to the exact outputs actually evidenced for that scope.
- A successful import or comparison for one pilot-bounded slice does not authorize compatibility claims for adjacent packages, even when they share a source book.
- A one-way import path from PCGen source into canonical Codex objects does not imply round-trip export compatibility back into legacy formats.

## Codex-authored package rules
GE-08 now provides the intended authored-package shape: deterministic YAML source bundle, explicit manifest, provenance, diagnostics, stable IDs, and distinct `schema_version` / `package_version` surfaces.

But the current posture remains planning-only.

Therefore:
- GE-09 may define the compatibility contract for authored packages now.
- GE-09 may not claim that authored package migration, downgrade, load, save, export, or cross-workspace exchange already works in practice.
- No authored-package compatibility promise may exceed the narrowest verified GE-08 execution proof that exists at the time of the claim.
- Arbitrary scripting, opaque plugins, or hidden runtime caches may not be treated as compatibility-preserving authoring surfaces.

## Intentional divergence and deprecation
If Codex deliberately stops preserving a legacy behavior or package convention, the change must be classified explicitly.

Required rule:
- intentional divergence requires a doctrine decision record
- deprecation requires a visible sunset or downgrade note for the affected scope
- neither divergence nor deprecation may be hidden inside a version bump or release note adjective

## Review triggers
Re-evaluate this policy when any of the following becomes true:
- GE-05 publishes stronger parity evidence or a new blocking known gap for a named package scope
- GE-06 changes the propagated claim ceiling for the pilot or adjacent scope
- GE-08 produces a narrower execution/readiness closure for authored package lifecycle behavior
- a doctrine decision sets release authority, package signing, trust-network, or intentional-divergence policy
- a narrower GE-09 expansion update names a new package family or source-scope band for active proof

## Explicit non-authorizations
This policy does not authorize:
- broad package compatibility claims beyond the exact named scope
- seamless upgrade or downgrade promises without a migration surface and evidence
- treating GE-08's planning-ready package lifecycle document as runtime proof
- public registry, marketplace, package-signing, or trust-network promises
- translation of version numbers alone into release readiness

## Completion rule
This policy is complete for GE09-E2 only if a future session can answer, without improvising:
1. What exact scope is being called compatible?
2. Which package class and evidence tier justify that statement?
3. Which version surface changed?
4. What happens on migration or downgrade?
5. What remains explicitly unproven because GE-05 parity and GE-08 execution truth are not there yet?
