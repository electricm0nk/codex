# Rollback, Withdrawal, and Downgrade Policy

## Purpose
Define the required state transitions and recovery behavior when a distributed tester build is superseded, withdrawn, broken, or intentionally rolled back.

## State model
| State | Meaning | Tester consequence | Operator obligation |
|---|---|---|---|
| `active` | current eligible build for its channel/platform | normal availability/update behavior | keep manifest/release metadata current |
| `superseded` | replaced by a preferred newer build | build may remain known, but the replacement should be preferred | record replacement target explicitly |
| `withdrawn` | build should no longer be used | client must surface warning/recovery path instead of normal update state | record reason and replacement/recovery direction |
| `blocked` | build is known but must not be offered automatically | client must explain why automatic update is unavailable | fix or reclassify before re-enabling |

## Required behavior
- a withdrawn build must not remain silently eligible for normal update flow
- a superseded build must be able to name its preferred replacement
- if the newest build is bad, a recovery-preferred prior build may be named explicitly rather than leaving testers to guess
- rollback and downgrade must preserve provenance and support-tier truth; a recovery target is still a governed build, not a mystery file

## Recovery requirements
Every channel/platform recovery path must define:
- how a tester learns the current build is withdrawn or blocked
- where the approved replacement or recovery artifact lives
- whether the recovery path is automatic, manual-assisted, or manual-only
- what build/provenance identity the tester should expect after recovery

## Manual-only posture
Manual recovery is acceptable in this tranche when automatic rollback is not yet implemented, but manual-only behavior must still be explicit and must still point at governed artifacts and release records.

## Current executable truth
As of 2026-06-29, this policy does **not** assert that the repo already has a live publication/manifests bridge for withdrawn, superseded, blocked, or recovery-preferred state, and it does **not** assert that the desktop app already has a dedicated recovery-consumer boundary. Those seams remain absent from live repo truth; this policy defines what future execution must honor once they exist.

## Explicit refusals
- do not treat deletion or disappearance as rollback policy
- do not ask testers to infer whether a build is safe from release timing or rumors
- do not let rollback behavior bypass checksum/provenance expectations
- do not pretend downgrade is impossible if recovery actually depends on it
