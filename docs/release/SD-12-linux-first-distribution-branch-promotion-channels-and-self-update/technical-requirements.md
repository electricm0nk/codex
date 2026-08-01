# Technical Requirements

## Problem Statement
Codex has a real Linux-verified desktop build path and a truthful tester-facing status surface, but it still lacks an accepted authority surface for how tester builds are packaged, published, promoted, updated, rolled back, withdrawn, and proven trustworthy. SD-12 must define that distribution/update control plane so later implementation can be bounded instead of improvised.

## Current-State Facts
- the live repo currently exposes only one GitHub workflow under `.github/workflows/allow-only-develop-into-main.yml`, and that workflow governs PR source-branch rules for `main`; it does not publish release artifacts or updater metadata
- `repos/codex/apps/desktop/src-tauri/tauri.conf.json` currently sets `bundle.active` to `false`, which confirms that packaging outputs are not already enabled in the repo
- `repos/codex/apps/desktop/package.json` defines a Tauri desktop package at version `0.0.0` with build/typecheck scripts, but no release/publish/update scripts
- `repos/codex/README.md` explicitly says Linux onboarding/build was verified while release packaging remains unfinished
- SD-11 already fixes the tester-facing update/channel/support vocabulary: `alpha`, `beta`, and `stable` remain the channel labels, but after truth repair only `alpha`/`develop` and `stable`/`main` are backed by live repo promotion surfaces; `beta` remains reserved until a governed candidate surface exists, and current UI copy still says update checks are not wired yet
- the live desktop status surface and tests already preserve Linux first-class, macOS second-class, and Windows third-class language that SD-12 must not contradict
- the live repo exposes no dedicated manifest-emission workflow, no dedicated manifest/update boundary file under `apps/desktop/src/boundary/`, and no Tauri command for manifest fetch/check/apply; E3 execution truth therefore remains absent rather than merely unfinished
- the live repo also exposes no publication/manifests bridge that can emit withdrawn, superseded, blocked, or recovery-preferred release state and no dedicated desktop recovery-consumer boundary beyond the static SD-11 `not-yet-supported` status surfaced through `createSd11WorkbenchStatus.ts`, `loadSd11TesterWorkbenchSurface.ts`, and `App.tsx`; E4 execution truth therefore remains absent rather than merely unfinished

## Desired Behavior
- define, without falsely claiming present repo support, the first bounded tester-distribution contract as a GitHub-backed artifact/update system rather than a build-from-source ritual
- define Linux as the first-class packaging and self-update target, with explicit but lower-maturity postures for macOS and Windows
- define which release/publication surfaces are authoritative, which artifacts each promoted build must publish, and how branch promotions change tester-visible eligibility
- define a machine-readable update-manifest contract and the rules for when a client may claim update eligibility
- define withdrawn-build, rollback, downgrade, and recovery obligations so later updater work cannot counterfeit safety
- define the minimum provenance, checksum, and platform-specific trust gates required before self-update can be claimed honestly
- preserve the separation between operator branch truth and tester-facing channel language already established by SD-11

## Architecture Constraints
- the packet is planning-only and grants no implementation-code or CI-write authority
- distribution truth remains GitHub-backed for this tranche; do not invent a parallel artifact origin or updater backend here
- platform promises must remain asymmetric: Linux first-class, macOS second-class but real, Windows explicitly third-class
- branch promotion remains the operator control model; tester-facing channel language layers on top of it and does not replace it
- updater claims must be capability-based, not aspirational: no platform may claim automatic update unless its artifact type, manifest contract, integrity gate, and failure behavior are explicit
- repo-local release automation, updater libraries, and exact package formats remain downstream decisions unless this packet fixes them deliberately

## Interfaces / Contracts / Schemas
- **Distribution platform/support matrix** — the packet must define per-platform delivery mode, support tier, update posture, rollback posture, and channel eligibility rules
- **GitHub publication contract** — the packet must define the authoritative GitHub surfaces, required asset classes, naming/provenance burden, and promotion rules tied to branch truth
- **Update-manifest contract** — the packet must define the machine-readable fields required for update discovery, retrieval, integrity checks, and eligibility decisions, while stating explicitly that the contract is not proof of a live repo-owned manifest producer or consumer seam
- **Rollback/withdrawal contract** — the packet must define what happens when a build is superseded, withdrawn, or intentionally rolled back, including user-visible and operator-visible obligations
- **Provenance/integrity contract** — the packet must define the minimum build identity, checksum, provenance, and platform trust thresholds needed before self-update claims are allowed
- **SD-11 coupling contract** — the packet must define what SD-11 may show or auto-capture about build/channel/support/update state without taking ownership of artifact publication logic or pretending a dedicated consumer seam already exists

## Required Reads Carried into This Document
- `README.md` — authority, readiness, and packet scope
- `../../plans/spec-domains/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update.md` — strategic SD-12 boundary
- `../../plans/spec-domains/SD-11-test-user-workbench-and-github-feedback-intake.md` — adjacent tester-workbench boundary
- `../SD-11-test-user-workbench-and-github-feedback-intake/README.md` — accepted SD-11 planning surface
- `../SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md` — current channel/support mapping
- `/home/ubuntu/workspace/repos/codex/README.md` — current verified Linux-first onboarding/build truth and unfinished packaging statement
- `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml` — current branch-promotion governance surface
- `/home/ubuntu/workspace/repos/codex/apps/desktop/package.json` — current desktop package identity and build command surface
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json` — current inactive packaging surface
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx` and `src/sd11/**` — current tester-facing status/update language and support-tier truth

## Subsystem Notes
### Distribution platform/support matrix
- responsibilities:
  - define the truthful distribution posture for Linux, macOS, and Windows
  - bind platform support tiers to allowed artifact classes and update claims
  - preserve the existing SD-11 operator-to-tester mapping rather than rewriting it
- relevant files:
  - `artifacts/distribution-platform-support-and-channel-matrix.md`
  - `artifacts/self-update-transport-and-manifest-contract.md`
- known risks:
  - pressure to over-promise macOS or Windows parity
  - pressure to let platform tiers drift out of sync between release surfaces and the desktop UI

### GitHub artifact publication and promotion
- responsibilities:
  - define the authoritative GitHub release/prerelease surfaces
  - define required assets, publication states, and promotion rules
  - define what branch truth controls publication eligibility
- relevant files:
  - `artifacts/github-artifact-publication-and-promotion-contract.md`
  - `artifacts/distribution-platform-support-and-channel-matrix.md`
- known risks:
  - confusing feature-branch build output with accepted tester channels
  - burying promotion truth in prose instead of explicit tables and states

### Self-update transport and manifest
- responsibilities:
  - define how a client discovers an update, identifies the correct asset, checks eligibility, and verifies integrity
  - define the minimal manifest schema later implementation must honor
- current executable truth:
  - no live repo workflow or boundary file currently owns manifest emission or manifest consumption for SD-12
- relevant files:
  - `artifacts/self-update-transport-and-manifest-contract.md`
  - `artifacts/provenance-integrity-and-update-eligibility.md`
- known risks:
  - claiming updater support before manifest and trust gates exist
  - coupling the public contract to one specific library too early
  - mistaking the documentary manifest contract for proof that an executable producer/consumer path already exists

### Rollback, withdrawal, and downgrade
- responsibilities:
  - define what happens when a build is superseded, withdrawn, or needs rollback
  - preserve recovery paths that do not depend on silent success
- current executable truth:
  - no live publication/manifests bridge or dedicated desktop recovery-consumer boundary owns rollback/withdrawal state today
- relevant files:
  - `artifacts/rollback-withdrawal-and-downgrade-policy.md`
  - `artifacts/github-artifact-publication-and-promotion-contract.md`
- known risks:
  - treating rollback as an operator-only concern with no tester-visible consequence
  - allowing withdrawn builds to remain silently eligible for update or issue reporting without clear status
  - allowing the static SD-11 `not-yet-supported` update posture or unrelated GE-08 `blocked` diagnostics to masquerade as the dedicated recovery-consumer boundary

### Provenance, integrity, and update eligibility
- responsibilities:
  - define the minimum build-identity and checksum/provenance burden for every distributed tester build
  - define which extra trust gates block automatic update on each platform
- relevant files:
  - `artifacts/provenance-integrity-and-update-eligibility.md`
- known risks:
  - substituting “download exists” for trustworthy update eligibility
  - assuming one platform’s integrity posture authorizes another’s

### SD-11 coupling surface
- responsibilities:
  - define what build/channel/support/update truth the desktop UI may expose and auto-capture
  - keep SD-11 as the tester-facing UX authority while SD-12 owns the underlying planned artifact/update system
- relevant files:
  - `artifacts/distribution-platform-support-and-channel-matrix.md`
  - `artifacts/self-update-transport-and-manifest-contract.md`
  - `../SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md`
- known risks:
  - allowing SD-11 to infer release mechanics from UI copy alone
  - allowing SD-12 to overwrite issue-intake or workbench authority that belongs to SD-11
  - allowing existing GE-08 / pilot-shell seams or hard-coded status copy to masquerade as the dedicated desktop consumer boundary for manifest/update truth

## Non-Goals
- implementing GitHub release automation, updater code, or package builds during this planning pass
- selecting the final Linux package set, updater library, or platform signing stack unless later decisions ground them explicitly
- public-release, marketplace, app-store, or enterprise deployment posture
- flattening feature branches into recognized tester channels
- expanding SD-11, SD-13, or SD-14 scope under the cover of packaging work

## Decision Boundaries
- Decisions already made:
  - Linux is first-class, macOS is second-class, Windows is third-class for this tranche
  - GitHub is the authoritative distribution/update origin for this lane
  - the live operator promotion path is `develop -> main`, and any future candidate-stage label such as `beta` must stay unclaimed until a governed backing surface exists
  - tester-facing channel language remains layered over branch truth rather than replacing it
  - current desktop update language must remain honest about “not yet wired” until SD-12-backed execution lands
  - current desktop recovery behavior must remain honest about its absence until a dedicated recovery-consumer boundary exists
- Decisions still open:
  - the exact Linux package set for the first bounded tester program
  - whether macOS initially receives manual download-only posture or a narrower package path
  - the exact Windows distribution promise, if any, beyond explicit third-class containment
  - the exact updater implementation technology and manifest hosting shape within the GitHub-backed contract
  - the exact signing/notarization thresholds required before each platform may claim automatic update
- Decisions forbidden at this stage:
  - granting repo or CI write authority from this packet alone
  - claiming equal packaging/update maturity across all desktop platforms
  - reinterpreting feature-branch artifacts as official tester-channel releases
  - treating checksums/provenance as optional for distributed tester builds
