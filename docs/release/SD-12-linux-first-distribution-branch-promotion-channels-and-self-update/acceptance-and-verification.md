# Acceptance and Verification

## Acceptance Criteria
- [ ] the SD-12 control bundle exists with `README.md`, `technical-requirements.md`, `technical-design.md`, `acceptance-and-verification.md`, `risks-and-open-questions.md`, and `epic-breakdown.md`
- [ ] the packet defines Linux-first distribution/update truth as a concrete GitHub-backed artifact/update system rather than a build-from-source ritual or vague future promise
- [ ] the packet preserves explicit platform asymmetry: Linux first-class, macOS second-class but real, Windows third-class
- [ ] the packet preserves live `develop -> main` operator truth while keeping tester-facing channel language separate and treating any future `beta` stage as reserved until it has a governed backing surface
- [ ] the packet defines explicit publication, manifest, rollback/withdrawal, and integrity/provenance contracts
- [ ] the packet explicitly records that accepted repo truth now includes the merged Linux tester publication workflow on `origin/develop`, but no accepted runtime release-truth surface or dedicated desktop consumer boundary exists yet, so E3 and E6 executable desktop-synchronization work remain blocked until those seams are grounded or higher-order authority revises the claim again
- [ ] the packet explicitly records that accepted publication truth currently stops at release assets, checksums, provenance, release notes, and `update-manifest-stub.json`, so no accepted recovery-state producer contract or desktop recovery-consumer boundary exists yet and E4 executable recovery work remains blocked until that seam is grounded or higher-order authority revises the claim again
- [ ] the packet remains planning-only and does not counterfeit repo or CI code authority

## Artifact Completeness Gate
- [ ] every `status: required` artifact in **Expected Output Artifacts** has an exact destination path
- [ ] every `status: required` artifact has a concrete completion rule
- [ ] final verification proves each required artifact exists or was updated as required
- [ ] any intentionally deferred artifact is called out explicitly in `README.md` and `risks-and-open-questions.md`

## Verification Requirements

### Planning-surface truth check
- confirm the packet reflects the live repo truth that `.github/workflows/allow-only-develop-into-main.yml` governs branch source rules and that `.github/workflows/publish-tester-release.yml` is merged accepted repo truth on `origin/develop` via PR #32
- confirm the packet reflects the live repo truth that `apps/desktop/src-tauri/tauri.conf.json` still has `bundle.active: false`
- confirm the packet reflects the live repo truth that `origin/develop` `apps/desktop/src-tauri/src/main.rs` carries no accepted `load_sd12_release_truth` / `ReleaseTruthSnapshot` seam, that no dedicated TypeScript manifest/update boundary exists beyond `apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts` and `apps/desktop/src/boundary/loadPilotShellSnapshot.ts`, and that the hard-coded `not-yet-supported` SD-11 status surface still stands in for real release-truth consumption
- confirm the packet reflects the live repo truth that accepted publication currently stops at Linux release assets, checksums, provenance, release notes, and `update-manifest-stub.json` rather than a full withdrawn/replacement/recovery state bridge, and that `App.tsx` still exposes only the static SD-11 status/evidence surface rather than recovery guidance
- confirm `repos/codex/README.md` still frames Linux onboarding/build as verified while release packaging remains unfinished
- confirm Linux/macOS/Windows support asymmetry remains explicit in the packet
- confirm GitHub is the only distribution/update origin named by SD-12

### Minimal Change Rule
- make only documentary/control-plane changes required to create and register the SD-12 source STC
- do not change repo implementation code in `repos/codex/**`
- do not create an execution handoff during this planning pass

### Final Verification
- verify the new packet directory exists under `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/`
- verify all required control-bundle files exist
- verify all required same-epic documentary artifacts exist
- verify `artifacts/sd12-e3-b1-manifest-producer-and-updater-boundary-truth.md` exists and names the exact blocker/unblock condition for the later E3 handoff lane
- verify `artifacts/sd12-e4-b1-rollback-withdrawal-and-downgrade-boundary-truth-2026-06-29.md` exists and names the exact blocker/unblock condition for the later E4 handoff lane
- verify `artifacts/sd12-e6-b1-status-and-issue-payload-coupling-truth.md` exists and names the exact blocker/unblock condition for the later E6 handoff lane
- verify `programs/codex/requirements/README.md` now indexes SD-12 as a live source STC rather than a merely planned one
- verify the SD-12 strategic spec-domain no longer claims that no source STC exists
- verify the packet names the already-linked next move as `SD-12 FLOW: Mint bounded execution stories from the SD-12 epic breakdown`

## Evidence Expectations
- report actual file paths written and actual control-plane files patched
- if any referenced upstream truth cannot be grounded from the live repo or current accepted docs, say so explicitly
- do not substitute intention for proof

## Exit Conditions
The source STC may be treated as execution-story-ready when the packet and same-epic documentary artifacts exist, the control plane is updated, and `epic-breakdown.md` is concrete enough to mint bounded same-domain stories without guesswork. A later coding handoff remains forbidden until a future bounded slice grounds exact repo paths, exact verification commands, exact write scope, exact release-surface authority, and — for E3/E6 specifically — a dedicated desktop consumer boundary that truthfully consumes the live publication/release-truth surfaces, and — for E4 specifically — a dedicated desktop recovery-consumer boundary that truthfully consumes recovery-facing publication state.
