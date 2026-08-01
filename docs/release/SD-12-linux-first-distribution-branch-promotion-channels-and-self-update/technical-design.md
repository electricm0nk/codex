# SD-12 Linux-first Distribution Technical Design

## Purpose
This design operationalizes the SD-12 source STC by defining how branch promotion, GitHub publication, update manifests, rollback controls, and desktop update/status surfaces should cooperate without counterfeiting platform parity or release maturity.

## Design posture
- architecture style: `branch-governed GitHub-backed distribution control plane`
- promotion posture: `operator-branch-first, tester-channel-second`
- provenance posture: `strict`
- diagnostics posture: `strict`

## Context and constraints
- the current live repo proves only branch-gating and buildability, not release automation or packaging
- the desktop UI already exposes bounded channel/support/update language through SD-11 surfaces and must remain a consumer of release truth, not its source
- Linux must receive the strongest bounded packaging and update posture first
- macOS and Windows must be represented honestly even when their update and trust posture lags Linux
- later implementation must be able to withdraw, supersede, or roll back builds without inventing hidden operator folklore
- accepted repo truth now includes the Linux publication workflow, but still lacks any accepted runtime release-truth seam, any dedicated TypeScript desktop manifest/update consumer boundary, and any dedicated desktop recovery-consumer boundary; current SD-11 update/issue truth remains a largely hard-coded local model layered over unrelated GE-08 / pilot-shell seams

## Executable boundary truth as of 2026-06-29
- `.github/workflows/allow-only-develop-into-main.yml` and `.github/workflows/publish-tester-release.yml` are both durable accepted workflow surfaces on `origin/develop`; PR #32 merged the Linux tester publication workflow into accepted repo truth
- `apps/desktop/src/boundary/` still contains only `loadGe08AuthoringWorkbench.ts` and `loadPilotShellSnapshot.ts`; there is no dedicated `loadSd12ReleaseTruth.ts` or equivalent desktop release-truth boundary file yet
- `origin/develop` `apps/desktop/src-tauri/src/main.rs` contains no accepted `load_sd12_release_truth` / `ReleaseTruthSnapshot` seam at all; the earlier local candidate did not mature into durable repo truth, and no TypeScript boundary loader or update/apply client chain consumes an accepted replacement
- `apps/desktop/src/sd11/loadSd11TesterWorkbenchSurfaceRuntime.ts` still wires only GE-08 and pilot loaders, and `apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.ts` still composes from a local `createSd11WorkbenchStatus(context)` model plus GE-08/pilot data
- `apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts` still hard-codes `state: 'not-yet-supported'` and the stale operator promotion path `develop -> uat -> main`
- `apps/desktop/src/App.tsx` plus the SD-11 evidence/issue-payload surfaces still render from that local status/evidence truth rather than any accepted SD-12 release-truth snapshot, and no withdrawn-build warning, blocked-build explanation, preferred-replacement guidance, or downgrade/recovery UI boundary exists yet

These are not minor omissions. They mean the manifest/update lane and the rollback/recovery lane are still documentary contract, not executable repo truth.

## Proposed system shape
The SD-12 contract still models six cooperating surfaces, but durable accepted repo truth currently grounds only the operator-promotion surface plus the Linux publication workflow. The runtime release-truth seam, dedicated desktop consumer boundary, and rollback/recovery presentation surfaces remain incomplete:
1. an **operator promotion surface** that preserves the live control truth `develop -> main` and refuses to mint a candidate stage until repo/workflow governance makes it real
2. a **build-and-package surface** that produces per-platform artifacts plus build identity and provenance
3. a **GitHub publication surface** that publishes versioned release units, release assets, checksums, and provenance receipts
4. a **channel-manifest/update surface** that maps branch-backed promotion state to machine-readable tester eligibility
5. a **desktop consumer surface** that lets SD-11 read build/channel/support/update truth without becoming release authority
6. a **rollback and withdrawal surface** that can mark builds superseded, withdrawn, or recovery-preferred without silent failure

## Data flow
1. a candidate commit becomes eligible for tester distribution only when it reaches a governed promotion point (`develop` or `main`; a future candidate point such as `uat` is out of scope until it exists in repo/workflow truth) and the bounded release slice authorizes publication
2. the packaging layer builds the approved artifact set for each eligible platform and records build identity, checksums, and provenance
3. the publication layer emits a GitHub release unit containing platform assets, checksums, provenance/build receipt, and machine-readable update metadata
4. the channel-manifest surface maps that release unit into tester-facing channel eligibility (`alpha`, `beta`, `stable`) while retaining the underlying operator branch lineage for audit/provenance
5. the desktop app reads current build/channel/support/update truth from the release unit and manifest contract and exposes it through SD-11 surfaces
6. when a build is superseded or withdrawn, the rollback surface updates channel eligibility and recovery guidance so neither operators nor testers mistake stale builds for safe current ones

## Component boundaries
### Operator promotion surface
- responsibilities:
  - preserve the live `develop -> main` release control model and reserve any future candidate stage until it is grounded in repo/workflow governance
  - determine which branches may produce recognized tester-channel publication states
  - prevent feature branches from being treated as official tester channels
- inputs:
  - branch topology and merge policy
  - bounded release-slice decisions
- outputs:
  - promotion state
  - audit-friendly branch lineage
- must not own:
  - tester-facing wording by itself
  - silent platform-support upgrades

### Build-and-package surface
- responsibilities:
  - produce platform artifacts and build identity data
  - attach checksums and provenance receipts to every distributed build
- inputs:
  - approved source revision
  - platform-specific package recipe and trust posture
- outputs:
  - platform artifacts
  - checksums
  - provenance/build receipt
- must not own:
  - channel semantics
  - claims of automatic update before the manifest and integrity gates exist

### GitHub publication surface
- responsibilities:
  - store release units and release assets as the authoritative tester-distribution origin
  - distinguish prerelease/candidate/stable surfaces without inventing a second backend
- inputs:
  - packaged artifacts and metadata
  - promotion state
- outputs:
  - GitHub release/prerelease unit
  - immutable asset URLs/identifiers
  - manifest attachment or referenced metadata surface
- must not own:
  - hidden branch reinterpretation
  - unverifiable private artifact side channels

### Channel-manifest/update surface
- responsibilities:
  - describe current eligible release per channel/platform
  - tell a client whether an update is available, withdrawn, blocked, or manual-only
- inputs:
  - published release units
  - channel mapping policy
  - integrity/trust gates by platform
- outputs:
  - machine-readable channel manifests
  - update eligibility decisions
- current live repo status:
  - partially grounded on accepted repo truth: `.github/workflows/publish-tester-release.yml` is merged and emits Linux tester release assets, checksums, provenance, release notes, and `update-manifest-stub.json`, but no accepted runtime release-truth seam or dedicated desktop consumer boundary consumes that output yet
- must not own:
  - the UI wording layer
  - fake eligibility for unsupported platforms

### Desktop consumer surface
- responsibilities:
  - expose build/channel/support/update truth in product language through SD-11
  - include enough provenance/state for issue capture and user comprehension
- inputs:
  - current build identity
  - current platform/support posture
  - update-manifest state
- outputs:
  - tester-facing status/update read models
  - issue-capture metadata
- current live repo status:
  - absent as a dedicated TypeScript boundary; `origin/develop` has no accepted `load_sd12_release_truth` replacement in `src-tauri/src/main.rs`, and SD-11 still consumes only hard-coded status copy plus unrelated GE-08 / pilot-shell seams
- must not own:
  - publication or manifest authorship
  - direct branch mechanics as default UX

### Rollback and withdrawal surface
- responsibilities:
  - mark releases superseded, withdrawn, blocked, or recovery-preferred
  - preserve clear recovery instructions when a build must no longer be used
- inputs:
  - release state changes
  - manifest state transitions
  - support guidance
- outputs:
  - withdrawn/recovery status
  - recovery path metadata
- current live repo status:
  - not yet durably grounded as a recovery bridge; accepted repo truth publishes Linux release assets plus `update-manifest-stub.json`, but no accepted withdrawn/replacement/recovery-state producer contract or dedicated desktop recovery-consumer boundary exists
- must not own:
  - silent deletion with no recorded replacement or warning
  - static SD-11 `not-yet-supported` status or unrelated GE-08 diagnostics masquerading as governed recovery truth

## Data and schema notes
- key entities:
  - release unit
  - channel manifest
  - platform artifact record
  - provenance/build receipt
  - rollback/withdrawal record
- minimum manifest fields:
  - manifest version
  - channel label
  - operator source branch / promotion path reference
  - platform label and support tier
  - release identifier and version/build label
  - commit/provenance handle
  - publication timestamp
  - asset list with checksum and install/update mode
  - update eligibility state
  - withdrawn/superseded/recovery metadata when applicable
- lossiness/disposition model:
  - when a platform lacks the required trust threshold, the manifest must report manual-only or unsupported posture rather than claiming auto-update
  - when a build is withdrawn, the manifest must preserve the reason and replacement/recovery direction rather than simply disappearing

## External dependencies and references
- `../SD-11-test-user-workbench-and-github-feedback-intake/README.md` — tester-facing workbench authority
- `../SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md` — current operator-to-tester mapping
- `/home/ubuntu/workspace/repos/codex/.github/workflows/allow-only-develop-into-main.yml` — existing branch-governance evidence
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/tauri.conf.json` — current inactive packaging surface
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src/App.tsx` and `src/sd11/**` — current desktop consumer surface

## Design decisions already fixed
- GitHub is the authoritative distribution/update origin for this tranche
- Linux-first packaging and update posture are the first-class target
- current live operator branch promotion truth remains `develop -> main`; any future `beta`/candidate stage must gain a governed backing surface before the docs or product claim it
- tester-facing channel wording remains layered over branch truth rather than replacing it
- every distributed build must carry explicit identity, provenance, and integrity material before update claims are honest

## Deferred design decisions
- exact package formats per platform
- exact updater implementation technology and hosting layout inside the GitHub-backed contract
- exact signing/notarization thresholds and automation strategy by platform
- exact authentication posture if distribution remains private or gated for testers
- exact downgrade UI/UX wording when recovery or rollback is required

## Failure modes and observability
- a build publishes artifacts but not manifest/checksum/provenance material
- a channel manifest points to withdrawn or incomplete assets
- a desktop client cannot determine whether a build is manual-only, unsupported, or update-eligible
- a rollback/withdrawal occurs without a visible replacement or recovery path
- a platform is shown as update-capable despite missing its integrity gate

Required observable signals:
- current build/channel/platform/support state must remain visible
- update failures and manual-only states must remain attributable
- withdrawn builds must be classifiable and linked to a recovery path
- branch lineage must remain auditable even when the UI shows only tester-friendly channel language

## Verification implications
`acceptance-and-verification.md` must prove that the packet defines explicit contracts for platform support/channel mapping, GitHub publication, update manifests, rollback/withdrawal behavior, and provenance/integrity thresholds. It must also prove the packet does not counterfeit a live manifest producer, desktop consumer boundary, recovery-state publication/manifests bridge, or desktop recovery-consumer boundary while those seams remain absent. Later execution handoffs must prove implemented packaging, release automation, updater behavior, and recovery guidance preserve those contracts without inventing parity or hiding failure states.

## Change constraints
- do not let a later handoff conflate operator branch truth with tester-facing wording
- do not let a later handoff infer release authority from feature-branch builds
- do not allow repo-local packaging or updater code to skip explicit manifest, checksum, or provenance contracts
- do not treat macOS or Windows lag as a reason to weaken Linux-first truth or to fake symmetric maturity
