# SD-12 Linux-first Distribution Epic Breakdown

## Breakdown rule
This file decomposes the SD-12 source STC into implementation-facing epics and feature seeds without becoming an execution handoff.

## Epic SD12-E1 — Linux-first artifact topology and platform support matrix
**Objective:** Define and later implement the first bounded artifact set, packaging posture, and support-tier contract for Linux, macOS, and Windows.

**Derived from:**
- SD-12 README: Objective / In Scope / Acceptance Summary
- `technical-requirements.md`: Distribution platform/support matrix
- `artifacts/distribution-platform-support-and-channel-matrix.md`

**Depends on:**
- current GE-10 Linux-first proof and onboarding truth
- SD-11 platform/support wording

### Feature seed SD12-F1 — First Linux artifact set
**Outcome:** The bounded tester program has an explicit first-class Linux artifact set with named install/update/recovery roles.

**Acceptance signals:**
- the first Linux artifact set is selected explicitly
- verification commands and recovery paths are named

**Notes:**
- must not wait for cross-platform parity before Linux-first execution begins

### Feature seed SD12-F2 — macOS and Windows bounded posture
**Outcome:** macOS and Windows distribution promises are explicit, bounded, and non-parity-claiming.

**Acceptance signals:**
- macOS second-class posture is concrete
- Windows third-class containment remains explicit

**Notes:**
- later handoff must keep these platforms honest rather than aspirational

## Epic SD12-E2 — GitHub publication and promotion control plane
**Objective:** Implement the GitHub-backed release/publication surface that translates branch promotion truth into channel-specific distribution states.

**Derived from:**
- `artifacts/github-artifact-publication-and-promotion-contract.md`
- `artifacts/distribution-platform-support-and-channel-matrix.md`

**Depends on:**
- SD12-E1
- current branch-governance truth in `.github/workflows/allow-only-develop-into-main.yml`

### Feature seed SD12-F3 — Alpha/beta/stable publication states
**Outcome:** `develop` and `main` can map to governed publication states immediately, while `beta` remains reserved until a real candidate promotion surface exists.

**Acceptance signals:**
- live publication states are auditable against branch lineage
- no `beta`/candidate claim appears without repo/control-plane backing truth
- tester-facing channel semantics remain stable

**Notes:**
- feature branches must remain out of official tester-channel publication flow

### Feature seed SD12-F4 — Release unit asset bundle
**Outcome:** Every published tester build carries the required asset classes, checksums, provenance, and manifest reference.

**Acceptance signals:**
- no release unit ships without the full required metadata set
- asset naming and provenance are machine-linkable

**Notes:**
- later handoff must name exact repo/CI files and verification commands

## Epic SD12-E3 — Update manifest and desktop-consumer contract
**Objective:** Ground and later implement the machine-readable update contract that lets the desktop app determine eligibility, retrieval, and manual-only/unsupported posture honestly.

**Derived from:**
- `artifacts/self-update-transport-and-manifest-contract.md`
- `artifacts/provenance-integrity-and-update-eligibility.md`

**Depends on:**
- SD12-E2
- SD-11 current status/update surface

**Current executable-boundary truth:** accepted repo truth now includes `.github/workflows/publish-tester-release.yml` on `origin/develop`, but no accepted runtime release-truth seam exists in `apps/desktop/src-tauri/src/main.rs` or `apps/desktop/src/boundary/`, and no dedicated TypeScript desktop consumer boundary exists in the repo yet, so no E3 execution handoff may be minted until a later repair grounds that consumer seam honestly.

### Feature seed SD12-F5 — Manifest generation
**Outcome:** Publication emits a manifest with the required fields for channel, platform, version, provenance, assets, and eligibility state.

**Acceptance signals:**
- a client can determine update availability or manual-only posture from manifest data alone
- withdrawn/recovery states are machine-readable

**Notes:**
- later handoff must choose an implementation path without changing the external contract
- this seed stays blocked until publication owns a real manifest-emission seam in repo/workflow truth

### Feature seed SD12-F6 — Desktop update/status consumer
**Outcome:** SD-11 update/status surfaces can consume manifest truth without inventing packaging or branch logic locally.

**Acceptance signals:**
- current build/channel/support/update state remains visible
- failure/manual-only/unsupported states remain attributable

**Notes:**
- this slice must preserve SD-11 authority over tester-facing wording
- existing GE-08 / pilot-shell seams are read-only pattern references, not the updater boundary this slice must eventually name

## Epic SD12-E4 — Rollback, withdrawal, and downgrade recovery
**Objective:** Implement the operator and tester behavior for superseded, withdrawn, broken, or recovery-preferred builds.

**Derived from:**
- `artifacts/rollback-withdrawal-and-downgrade-policy.md`
- `artifacts/self-update-transport-and-manifest-contract.md`

**Depends on:**
- SD12-E2
- SD12-E3

**Current executable-boundary truth:** accepted repo truth now includes `.github/workflows/publish-tester-release.yml`, but that workflow stops at Linux release assets, checksums, provenance, release notes, and `update-manifest-stub.json`; no accepted withdrawn/replacement/recovery-state producer contract or dedicated desktop recovery-consumer boundary exists beyond the static SD-11 `not-yet-supported` status/evidence surface.

### Feature seed SD12-F7 — Withdrawal and supersedence state transitions
**Outcome:** Release units and manifests can mark builds active, superseded, withdrawn, or blocked with visible recovery metadata.

**Acceptance signals:**
- withdrawn builds stop presenting as normal eligible updates
- superseded builds can name their preferred replacement

**Notes:**
- deletion is not a substitute for withdrawal state
- this seed stays blocked until publication owns a real recovery-state bridge and E3 grounds the manifest/update seam that carries those states honestly

### Feature seed SD12-F8 — Recovery and downgrade guidance
**Outcome:** When an update fails or a build is withdrawn, testers receive a governed recovery path rather than folklore.

**Acceptance signals:**
- recovery artifact/instruction source is explicit
- rollback/downgrade behavior does not pretend to be silent success

**Notes:**
- later handoff must name exact user-visible and operator-visible proof surfaces
- the current SD-11 status card and App shell are read-only truth references, not the dedicated recovery-consumer boundary this slice must eventually name

## Epic SD12-E5 — Provenance, integrity, and update-eligibility gates
**Objective:** Implement the required build identity, checksum/provenance outputs, and platform-specific trust gates that decide whether automatic update may be claimed.

**Derived from:**
- `artifacts/provenance-integrity-and-update-eligibility.md`
- `technical-requirements.md`: Provenance, integrity, and update eligibility

**Depends on:**
- SD12-E2
- SD12-E3

### Feature seed SD12-F9 — Build identity and checksum/provenance emission
**Outcome:** Every distributed tester build publishes the required build identity, checksum, and provenance receipt.

**Acceptance signals:**
- build provenance is machine-linkable to a specific release unit and source revision
- issue capture and diagnostics can cite build identity unambiguously

**Notes:**
- no build should become channel-eligible without these materials

### Feature seed SD12-F10 — Platform trust thresholds
**Outcome:** Automatic update is enabled only on platforms whose trust gates are explicitly satisfied.

**Acceptance signals:**
- Linux trust threshold is concrete
- macOS and Windows truth stays honest when their thresholds are not yet met

**Notes:**
- later handoff must not broaden platform claims merely because the manifest schema exists

## Epic SD12-E6 — SD-11 and SD-12 truth synchronization
**Objective:** Keep tester-facing update/channel/support surfaces synchronized with the artifact/update control plane without blurring authority boundaries.

**Derived from:**
- `artifacts/distribution-platform-support-and-channel-matrix.md`
- `artifacts/self-update-transport-and-manifest-contract.md`
- SD-11 update-channel artifact and README

**Depends on:**
- SD12-E3
- SD12-E5

**Current executable-boundary truth:** accepted repo truth now includes the publication workflow in `.github/workflows/publish-tester-release.yml`, but no accepted runtime release-truth seam exists in `apps/desktop/src-tauri/src/main.rs` or `apps/desktop/src/boundary/`, and SD-11 status/evidence surfaces still do not consume release truth: `apps/desktop/src/boundary/` has no dedicated release-truth loader, `loadSd11TesterWorkbenchSurfaceRuntime.ts` wires only GE-08 / pilot seams, `createSd11WorkbenchStatus.ts` still hard-codes `not-yet-supported` plus `develop -> uat -> main`, and the feedback payload composers still fork local truth.

### Feature seed SD12-F11 — Status-surface sync
**Outcome:** The desktop UI can present channel/support/update truth that is generated from the accepted SD-12 control plane.

**Acceptance signals:**
- channel/support wording stays consistent across issue payloads and update surfaces
- operator branch lineage remains auditable but not dominant in the UI

**Notes:**
- later handoff must preserve existing SD-11 copy constraints unless Todd explicitly changes them

### Feature seed SD12-F12 — Issue-payload provenance coupling
**Outcome:** Feedback payloads can capture build/channel/support/provenance handles from the release control plane without inventing them locally.

**Acceptance signals:**
- bug and enhancement flows can cite concrete build identity and channel state
- withdrawn/superseded builds remain classifiable in issue evidence

**Notes:**
- this is coupling, not ownership transfer; SD-11 still owns issue-flow UX contracts

## Initial sequencing
1. SD12-E1 — Linux-first artifact topology and platform support matrix
2. SD12-E2 — GitHub publication and promotion control plane
3. SD12-E5 — Provenance, integrity, and update-eligibility gates
4. SD12-E3 — Update manifest and desktop-consumer contract
5. SD12-E4 — Rollback, withdrawal, and downgrade recovery
6. SD12-E6 — SD-11 and SD-12 truth synchronization

## Handoff boundary
No coding harness should act directly from this file. Each later execution slice must receive a dedicated handoff that names:
- exact repo paths
- exact allowed write scope
- exact required reads
- exact verification commands
- exact non-goals
- exact release-surface authority and rollback behavior

Any derived handoff file must also receive its own artifact card on the board.

No E3-derived handoff is truthful while the repo still lacks a dedicated desktop consumer boundary that truthfully consumes durable accepted publication/release-truth surfaces.

## Completion gate
- [ ] every requirement is routed to at least one epic
- [ ] every epic has a bounded objective
- [ ] no epic silently changes program doctrine
- [ ] unresolved decisions remain in `risks-and-open-questions.md`, not hidden here
