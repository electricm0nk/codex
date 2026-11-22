# Risks and Open Questions

## Active Blockers
- none for planning-ready SD-11 packet generation; several blockers remain before any code-authorizing handoff would be honest

## Open Questions
- What is the minimum truthful bounded character-building workflow for the first external tester surface?
  - impact if unresolved: later implementation may either overbuild against fake breadth or underbuild against a meaningless demo path
  - preferred resolver: a same-domain SD-11 execution slice grounded against GE-06/GE-07 truth and later SD-13 scope boundaries
- What GitHub auth/storage posture is acceptable for issue submission from the desktop app?
  - impact if unresolved: the bug/enhancement transport cannot be implemented honestly
  - preferred resolver: a later bounded handoff that explicitly names auth options, local secret handling, and operator constraints
- What is the correct failure-mode contract when issue submission cannot complete immediately?
  - impact if unresolved: testers may lose evidence or believe a report was filed when it was not
  - preferred resolver: a later bounded UX/transport slice that defines local draft persistence and copy/export fallback
- What is the exact update transport by platform?
  - impact if unresolved: updater implementation may counterfeit cross-platform parity or bundle impossible promises
  - preferred resolver: a later SD-12 authority surface or a bounded SD-11 slice that consumes SD-12 once accepted
- What final tester-facing label set should be used for channel/build/support wording?
  - impact if unresolved: the product may leak operator branch language or invent misleading stability claims
  - preferred resolver: a small wording/UX decision surface aligned to the fixed operator mapping

## Risks Worth Preserving
- platform-support asymmetry may be erased by well-meaning UI copy, causing counterfeit support claims
- feedback flows may drift into free-form text boxes that destroy triage value
- current GE-08 workbench evidence may be misread as proof that the general tester workbench already exists
- updater language may drift into branch mechanics or developer convenience rather than product truth
- attachment capture may leak data that should be redacted unless explicit rules are encoded early

## Forbidden Assumptions
- do not assume GitHub auth, token storage, or user-identity posture
- do not assume offline queueing, attachment upload, or screenshot transport behavior
- do not assume the bounded tester workflow should inherit all future class/race breadth from SD-13
- do not assume Windows updater parity with Linux
- do not assume current GE-08 workbench payloads are the final SD-11 tester-workbench payloads

## Promotion Gate Notes
To move from planning-ready toward code-authorizing handoffs, the following must be resolved or explicitly grounded:
- exact first bounded tester workflow identity and data source
- exact GitHub issue transport/auth posture
- exact issue-submission failure behavior and evidence-draft fallback
- exact updater transport, manifest, and failure-handling posture by platform
- exact repo file scope and verification commands for the first implementation slice

For each item above, classify it as one of:
- `recoverable-by-tools`
  - current desktop surface shape and existing Tauri boundary inventory
  - current build/channel/platform repo truth
- `requires-human`
  - final tester-facing wording for channels/support labels if the existing doctrine is still judged insufficient
  - acceptable credential posture for GitHub issue submission if several viable options remain
- `hard-blocker`
  - any later attempt to authorize implementation without exact repo paths, exact write scope, or exact verification commands
