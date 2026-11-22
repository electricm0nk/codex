---
title: GE-10 Risks and Open Questions
stc_id: STC-CODEX-GE-10
artifact_type: risks-and-open-questions
status: draft
scope: programs/codex/requirements/GE-10-demo-proof-and-onboarding
source_stc: ./README.md
---

# GE-10 Risks and Open Questions

## Active risks

### R1 — Cross-platform drift risk
GE-10 currently documents only the platform that was verified during this pass: Linux desktop with Ubuntu 24.04-style Tauri prerequisites.

Risk:
- future readers may misread project intent (Linux/Windows/macOS target architecture) as live onboarding proof for all platforms

Mitigation:
- keep README and current-state summary explicit that only the Linux path was verified in this pass

### R2 — GUI-session ambiguity risk
The desktop shell can build in a headless environment but cannot launch there successfully.

Risk:
- an operator running the demo over SSH or in a headless CI shell may think the app is broken when the real limitation is session type

Mitigation:
- keep the runbook and README explicit that GUI launch requires a graphical desktop session

### R3 — README staleness risk
The repo README was previously too thin. It can become thin again if updates happen only in the program docs.

Mitigation:
- treat the README as a maintained onboarding surface and refresh it whenever proof commands or current-state claims change materially

### R4 — Documentary success masking product immaturity
A polished onboarding path can create false confidence that the product itself is broadly ready.

Mitigation:
- keep the current-state summary and README explicit that Codex remains a developer proof harness plus bounded desktop workbench surface

## Open questions

### Q1 — When should bootstrap become automated?
At some point the prerequisite/build step may deserve a scripted bootstrap or smoke-check command. GE-10 does not authorize that work yet.

### Q2 — When should non-Linux onboarding be documented?
Only after live proof exists for the target platform.

### Q3 — Should the demo remain tied to the GE-08 Guard Stance workbench?
For now that is the most truthful product-visible surface. Revisit only when a broader interactive path is real.

### Q4 — How should current-state refresh be governed?
GE-10 currently relies on manual refresh after live verification. Later governance may require an explicit recurring audit or completion gate.

## Forbidden assumptions

- “Tauri target architecture includes Windows/macOS” does not equal “Windows/macOS onboarding is verified.”
- “Desktop shell builds” does not equal “product is broadly usable.”
- “README is improved” does not equal “GE-10 no longer needs maintenance.”
