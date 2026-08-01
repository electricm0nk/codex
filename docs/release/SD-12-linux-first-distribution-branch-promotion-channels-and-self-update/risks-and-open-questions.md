# Risks and Open Questions

## Active Blockers
- none for planning-ready SD-12 packet generation; several blockers remain before any code-authorizing handoff would be honest

## Open Questions
- Which exact Linux package set should ship first for the bounded tester program?
  - impact if unresolved: later implementation may overbuild multiple formats prematurely or underbuild a distribution path that is too fragile for testers
  - preferred resolver: a same-domain SD-12 execution slice that chooses the minimum truthful Linux artifact set and verification commands
- What is the minimum honest macOS posture for this tranche: manual download-only, partial packaging, or a narrower gated follow-on?
  - impact if unresolved: the desktop UI or release notes may counterfeit real macOS support
  - preferred resolver: a bounded packaging/promotion slice that records the exact second-class posture
- What exact Windows promise should exist beyond explicit third-class containment?
  - impact if unresolved: later work may accidentally imply Windows parity or block Linux-first progress unnecessarily
  - preferred resolver: a narrow containment/visibility slice, not a parity ambition detour
- Should the later updater implementation use Tauri’s built-in updater path, a custom GitHub-manifest reader, or another bounded client strategy?
  - impact if unresolved: later implementation could overfit to tooling folklore instead of the contract
  - preferred resolver: a dedicated updater-transport execution slice grounded against the manifest contract in this packet
- If tester distribution remains private or gated, what authentication friction is acceptable before the UX stops being honest?
  - impact if unresolved: a later private-release path may either hide login friction or overcomplicate tester onboarding
  - preferred resolver: a later release-distribution slice that makes the auth posture explicit
- What platform-specific trust threshold is required before automatic update may be enabled?
  - impact if unresolved: self-update claims may be made before signing/notarization/code-signing posture is good enough
  - preferred resolver: platform-specific integrity-gate decisions aligned to the provenance artifact and release automation design

## Risks Worth Preserving
- platform-support asymmetry may be erased by well-meaning release or UI copy, causing counterfeit parity claims
- GitHub-backed publication may be implemented without explicit manifest/checksum/provenance outputs, causing “download exists” to masquerade as updater readiness
- the current branch-protection workflow may be mistaken for a full release-control plane
- withdrawn or superseded builds may remain silently discoverable unless rollback/withdrawal states are encoded early
- SD-11 may drift into updater mechanics or SD-12 may drift into UI/product wording unless their boundaries remain explicit

## Forbidden Assumptions
- do not assume an exact Linux package format set without a later bounded decision slice
- do not assume macOS or Windows automatic update eligibility in this tranche
- do not assume GitHub release automation or updater code already exists in the repo
- do not assume private-distribution auth posture, notarization, or code-signing strategy
- do not assume feature branches are recognized tester channels

## Promotion Gate Notes
To move from planning-ready toward code-authorizing handoffs, the following must be resolved or explicitly grounded:
- exact first Linux artifact set and exact verification commands
- exact GitHub publication workflow/write scope
- exact update-manifest generation and client-consumption path
- exact rollback/withdrawal operator actions and user-visible recovery behavior
- exact integrity thresholds by platform
- exact repo/CI file scope for the first implementation slice

For each item above, classify it as one of:
- `recoverable-by-tools`
  - current repo branch-governance files and packaging surface inventory
  - current SD-11 update/status language and support-tier copy
- `requires-human`
  - final risk tolerance for private-distribution auth friction if multiple viable options remain
  - final support promise language for macOS and Windows if the bounded choices remain strategically live
- `hard-blocker`
  - any attempt to implement updater/release automation without exact repo paths, exact CI/write scope, exact verification commands, or explicit integrity thresholds for the affected platform
