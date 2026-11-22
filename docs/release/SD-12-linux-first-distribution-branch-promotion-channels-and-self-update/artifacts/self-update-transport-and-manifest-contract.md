# Self-Update Transport and Manifest Contract

## Purpose
Define the machine-readable update contract for SD-12 so later implementation can discover, classify, retrieve, and verify updates without inventing release truth locally.

## Core rule
For this tranche, “self-update” means a GitHub-backed artifact/update system with machine-readable metadata. It does not mean “tell the user to pull the repo” and it does not mean “downloads exist somewhere.”

## Manifest responsibilities
A channel/platform manifest must let a client determine:
- current channel and support tier
- whether an update exists for the current platform/build
- which asset is eligible for retrieval
- whether the update is automatic, manual-only, blocked, withdrawn, or unsupported
- what provenance and checksum material must be checked before install/apply
- whether a recovery or rollback target is preferred instead of the newest build

## Minimum manifest fields
The exact serialization remains open, but the contract must carry at least:
- `manifestVersion`
- `channel`
- `operatorPromotionPathReference`
- `platform`
- `supportTier`
- `releaseId`
- `version`
- `buildLabel`
- `commitOrProvenanceHandle`
- `publishedAt`
- `assets[]` with per-asset url/identifier, checksum, size, and install/update mode
- `updateEligibilityState` (`automatic`, `manual-only`, `unsupported`, `withdrawn`, `blocked`)
- `replacementReleaseId` or recovery target when applicable
- `notes` or equivalent human-readable context for failures/manual-only posture when needed

## Platform update posture
| Platform | Update posture |
|---|---|
| Linux | first-class target; may claim automatic update only after the Linux artifact set, manifest path, and integrity gate are all satisfied |
| macOS | must be classified explicitly as `manual-only`, `unsupported`, or later `automatic` only after macOS trust requirements are satisfied |
| Windows | remains `unsupported` or tightly bounded `manual-only` in this tranche unless a later explicit slice authorizes more |

## Client obligations
A future desktop consumer of this contract must:
- preserve current build/channel/support visibility
- show manual-only or unsupported posture explicitly
- refuse silent success when eligibility, retrieval, or verification fails
- preserve withdrawn/recovery state rather than quietly offering a bad build

## Decision boundary
The contract intentionally does **not** decide whether the later implementation uses Tauri’s updater path, a custom manifest reader, or another bounded client strategy. What is fixed here is the external truth surface the client must honor.

## Current executable truth
This contract does **not** assert that the repo already has a live manifest emitter or a dedicated desktop consumer seam. As of 2026-06-29, those surfaces are still absent from live repo truth. This artifact defines what a future producer and consumer must honor once they exist.

## Explicit refusals
- do not reduce update transport to branch names or human memory
- do not treat a release page with no machine-readable manifest as a finished self-update system
- do not treat this contract itself as proof that a manifest producer or desktop consumer boundary already exists in the repo
- do not claim automatic update for a platform whose trust threshold is not yet satisfied
- do not hide withdrawn or recovery-preferred state from the client
