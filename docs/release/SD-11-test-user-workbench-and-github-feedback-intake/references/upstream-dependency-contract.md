# Upstream Dependency Contract — SD-11

## Purpose
This contract records what the SD-11 source STC may rely on from accepted upstream strategic, requirements, and live-repo surfaces, and what those surfaces do **not** authorize.

## Upstream contract table
| Upstream surface | SD-11 may rely on | SD-11 must not infer |
|---|---|---|
| `programs/codex/plans/spec-domains/SD-11-test-user-workbench-and-github-feedback-intake.md` | the strategic problem statement, in-scope/out-of-scope boundary, required source-STC path, and the requirement to treat GitHub intake and upgrade UX as first-class surfaces | implementation authority, exact repo write scope, or exact transport mechanics |
| `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/README.md` | shell-boundary truth, anti-UI-rules-authority doctrine, and current desktop-shell planning posture | proof that a tester workbench already exists or permission to implement one without a later bounded handoff |
| `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/README.md` | the live existence of a bounded desktop workbench proof over real headless data and its anti-counterfeit-success posture | proof that the general tester workbench or character-builder flow is already complete |
| `programs/codex/requirements/GE-10-demo-proof-and-onboarding/README.md` | current-state truth, Linux-first verification posture, and the rule that the desktop surface is still bounded | public-release readiness, cross-platform parity, or end-user-product maturity |
| `repos/codex/README.md` | the current truthful posture, verified desktop build surfaces, and current scope limitations | code authority or permission to widen product claims |
| `repos/codex/apps/desktop/src/App.tsx` | the current visible desktop surface shape and the fact that it is currently a GE-08 Guard Stance authoring workbench | the final SD-11 tester-workbench IA, issue flow, or update surface |
| `repos/codex/apps/desktop/src/boundary/loadPilotShellSnapshot.ts` | the explicit placeholder/fallback rule for the current pilot-character seam | that a real bounded character workflow is already wired |
| `repos/codex/apps/desktop/src/boundary/loadGe08AuthoringWorkbench.ts` and `src-tauri/src/main.rs` | the shape of the current Tauri command boundary and its ability to carry real diagnostics/provenance/explanation context | that those exact payloads are sufficient for the final SD-11 tester workflow without adaptation |
| `programs/codex/doctrine/program-doctrine-and-scope-charter.md` and `quality-gate-policy.md` | headless-core-first, no-counterfeit-completion, and evidence-first doctrine | a reason to skip later exact handoff fields or verification commands |

## Downstream obligations
Any later SD-11 execution handoff must:
- name the exact repo write scope
- name the exact bounded workflow it is implementing
- preserve diagnostic/explanation visibility rather than hiding it for polish
- preserve GitHub issue payload structure as defined by this packet
- preserve operator-versus-tester update semantics rather than exposing raw branch mechanics by default

## What this packet still does not authorize
- no repo code changes by itself
- no GitHub credential posture by implication
- no updater transport by implication
- no SD-12/SD-13/SD-14/SD-15 pivot by implication
