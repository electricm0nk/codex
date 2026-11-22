---
title: Cross-Platform Build Constraint Questions
artifact_type: constraint-ledger
stc_id: STC-CODEX-GE-07
source_stc: ../README.md
related_artifacts:
  - ./ge07-e6-platform-risk-receipt-2026-06-22.md
review_date: 2026-06-22
owner: Todd Hintzmann
scope: program
code_authority: false
---

# Cross-Platform Build Constraint Questions

## Objective
Record the packaging, signing, runtime, and ship-governance questions GE-07 must keep visible before desktop implementation claims become broad.

## Current verdict
The question set is no longer abstract.

`artifacts/ge07-e6-platform-risk-receipt-2026-06-22.md` now grounds the current stop condition:
- there is still no real shell slice to package
- the current Linux host is not yet a truthful Tauri packaging surface
- Windows and macOS both impose explicit tooling/signing lanes that do not yet exist here

This ledger therefore exists to separate:
1. questions that matter before the first real shell proof
2. questions that can remain deferred until later GE-09 release-governance work

## Platform posture summary
| Platform | Packaging path acknowledged now | Strong claim still forbidden |
|---|---|---|
| Linux | `.deb`, `.rpm`, and `AppImage` are plausible future outputs | claiming broad Linux compatibility or reproducible packaging without an older-baseline build receipt |
| Windows | NSIS and WiX/MSI are known output classes | claiming trusted Windows distribution without runner/tooling/signing proof |
| macOS | signed/notarized bundle path is known in doctrine | claiming macOS distribution without Apple-backed signing infrastructure |

## Questions that matter before the first real shell slice
### Q1. Linux baseline truth
Which Linux baseline will Codex treat as the first truthful packaging surface: Ubuntu 22.04, Debian 12, or a still narrower target?

Why it matters now:
- Tauri requires an old-enough base system that still provides WebKitGTK 4.1
- the current host is Ubuntu 24.04, which is usable for research but should not be assumed to be the canonical release builder

### Q2. First-proof packaging burden
Does the first real shell slice need to prove one Linux bundle output immediately, or may it remain an unbundled shell/runtime proof while packaging stays documentary for one more slice?

Why it matters now:
- this choice changes the size of the first shell handoff
- forcing packaging too early may contaminate scaffold or boundary work with release mechanics

### Q3. Windows proof posture
Should the first Windows packaging proof be:
- deferred entirely until a native Windows lane exists, or
- allowed as a documentary/cross-build experiment using NSIS from Linux with explicit caveats?

Why it matters now:
- WiX/MSI is Windows-only
- cross-building is possible but caveat-heavy and should not be confused with final release readiness

### Q4. Early shell scope multipliers
Will the first real shell slice require any of the following immediately:
- local database/storage
- bundled assets beyond the basic frontend
- tray support
- native dialogs/file pickers
- media playback

Why it matters now:
- each of these increases packaging and runtime burden, especially on Linux and Windows

## Questions that can remain deferred until GE-09
### Q5. Signing authority and custody
Where will Windows certificates, Apple certificates, notarization credentials, and updater signing keys live, and who owns them?

### Q6. Updater adoption
Will Codex use the Tauri updater at all for the first public release, and if so how will key rotation, release channels, and endpoint trust be governed?

### Q7. Public distribution posture
Will releases go through direct downloads, GitHub releases, store channels, enterprise/internal distribution, or a mixed posture?

### Q8. Packaging policy by channel
Will Codex ship all of `.deb`, `.rpm`, `AppImage`, NSIS, MSI, and macOS bundles, or choose a narrower first channel set?

## Decision rules
1. Do not let packaging questions silently expand a narrow shell slice into release engineering.
2. Do not let “Tauri is cross-platform” substitute for platform-specific builder, signer, runtime, or credential proof.
3. Do not let updater adoption happen casually; it creates key-custody obligations that outlive the first build.
4. Do not claim ship readiness until at least one real shell slice exists and each claimed platform has a platform-specific receipt.

## Completion rule
This artifact is satisfied when later GE-07 or GE-09 work can convert every relevant question into either:
- a bounded proof task with an exact platform receipt, or
- an explicit accepted deferment with a named owner and later governance lane.