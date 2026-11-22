# SD-11 Test-User Workbench Technical Design

## Purpose
This design operationalizes the SD-11 source STC by defining how the first tester-facing workbench should compose over the live Codex desktop/runtime seams without becoming rules authority or hiding bounded truth.

## Design posture
- architecture style: `hybrid desktop workbench over headless domain commands`
- migration posture: `partial-first`
- provenance posture: `mixed`
- diagnostics posture: `strict`

## Context and constraints
- the current live desktop app is a GE-08 authoring workbench over a real Tauri command boundary, not yet the SD-11 tester workbench
- the current pilot-character seam still exists only as a visibly bounded placeholder/fallback path
- GitHub intake and update behavior must be designed as product truth, not as repository convenience
- the workbench must preserve Linux-first truth without promising equal behavior on macOS or Windows

## Proposed system shape
The SD-11 tester workbench should be treated as five cooperating surfaces layered over the existing desktop shell:
1. a **bounded workbench frame** that states current build/channel/support truth and the current bounded workflow
2. a **domain snapshot/view-model layer** fed by real Tauri commands and later bounded character-builder commands
3. a **diagnostics and explanation visibility surface** that preserves reasons, blocked claims, and provenance/explanation context
4. a **feedback composer** that packages issue payloads for GitHub bug or enhancement submission
5. an **update/status surface** that maps current build/channel/support state to the operator promotion flow without surfacing raw git mechanics as the product

## Data flow
1. desktop shell loads the current bounded workbench snapshot from a real Tauri command or a visibly labeled placeholder/fallback surface when the truthful runtime seam is not yet implemented
2. the UI derives read models for visible tester surfaces, including explanation, diagnostics, support-tier, and workflow-boundary messaging
3. the tester triggers bug-report or enhancement-request actions from the current context, and the evidence packager composes a governed payload with auto-captured and user-supplied fields
4. the feedback transport submits the payload to GitHub or preserves a local draft/fallback path when submission cannot complete
5. the update/status surface reads current build/channel/support information and, later, update availability from a GitHub-backed artifact/update path aligned to the promotion model

## Component boundaries
### Workbench frame
- responsibilities:
  - show current bounded workflow identity
  - show build/channel/support metadata
  - host feedback and update actions
- inputs:
  - current bounded snapshot
  - support-tier metadata
  - current build/channel identity
- outputs:
  - visible tester context
  - launch points for feedback and update flows
- must not own:
  - rules computation
  - hidden support downgrades

### Snapshot/view-model adapter
- responsibilities:
  - adapt Tauri command payloads into the tester-facing read models
  - preserve blocked/unsupported state instead of flattening it away
- inputs:
  - Tauri command payloads from current and future bounded workbench commands
- outputs:
  - stable UI read models for the tester workbench
- must not own:
  - local recomputation of rules truth
  - silent repair of missing diagnostic or explanation context

### Diagnostics/explanation surface
- responsibilities:
  - show invalid-choice reasons, blocked claims, provenance, explanation references, and diagnostic context clearly enough for tester understanding and issue capture
- inputs:
  - snapshot diagnostics
  - explanation refs
  - provenance refs or equivalent bounded evidence
- outputs:
  - visible reasoning context
  - feedback-linked issue evidence
- must not own:
  - simplification that destroys triage value

### Feedback composer and evidence packager
- responsibilities:
  - package bug-report and enhancement-request payloads to governed contracts
  - preserve auto-captured fields, user-supplied fields, redactions, and attachment rules
- inputs:
  - current workbench context
  - tester-entered text
  - attachment metadata
- outputs:
  - GitHub issue payload or local draft/fallback package
- must not own:
  - alternate issue taxonomies beyond the governed contracts

### Update/status surface
- responsibilities:
  - display current build/channel/support truth
  - later surface update availability and update outcome honestly
- inputs:
  - current build metadata
  - branch/channel mapping metadata
  - support-tier posture
- outputs:
  - visible update/support state
- must not own:
  - raw git-branch terminology as tester-facing product language
  - unsupported platform promises

## Data and schema notes
- key entities:
  - bounded workbench snapshot
  - bug report payload
  - enhancement request payload
  - evidence-capture matrix row
  - update/channel mapping row
- provenance requirements:
  - feedback payloads must retain enough context to connect the issue to a concrete build/channel/workflow state
- diagnostic requirements:
  - blocked claims and diagnostic severity/classes must survive into the feedback surface
- lossiness/disposition model:
  - when the runtime seam cannot provide exact data, the UI must label the surface as placeholder/fallback rather than fabricating truth

## External dependencies and references
- `../GE-07-desktop-shell-and-modern-ux/README.md` — shell and UI-boundary obligations
- `../GE-08-homebrew-authoring-and-rules-studio/README.md` — current workbench proof surface and anti-counterfeit-success posture
- `../GE-10-demo-proof-and-onboarding/README.md` — truthful current-state/onboarding posture
- `/home/ubuntu/workspace/repos/codex/apps/desktop/src-tauri/src/main.rs` — current Tauri command boundary shape

## Design decisions already fixed
- the SD-11 workbench must consume real headless/domain truth rather than invent it locally
- GitHub issue intake is the authoritative feedback path for this tranche
- Linux-first support asymmetry must remain visible in the design
- live operator branch promotion truth remains `develop -> main`, and any future `beta`/candidate stage must gain a governed backing surface before documentation or product surfaces claim it

## Deferred design decisions
- exact workbench anchoring slice for the first bounded tester character workflow
- exact GitHub auth/storage/credential posture inside the desktop app
- exact offline/draft fallback behavior for issue submission failures
- exact updater transport and manifest path by platform
- final tester-facing wording for channel/build/support labels

## Failure modes and observability
- GitHub submission fails or is unavailable
- update check or update apply fails
- bounded workbench command returns placeholder/fallback data instead of real data
- diagnostics/explanation context is missing from the current snapshot
- platform/build mismatch causes unsupported update or unsupported launch path

Required observable signals:
- the workbench must visibly classify placeholder/fallback states
- issue submission failures must preserve a draft or copyable payload instead of disappearing silently
- update failures must remain visible and attributable to build/channel/platform state

## Verification implications
`acceptance-and-verification.md` must prove that the packet defines explicit contracts for the workbench surface, bug intake, enhancement intake, evidence capture, and update/channel mapping. Later execution handoffs must prove that implemented screens preserve those contracts without collapsing diagnostics or support-tier truth.

## Change constraints
- Do not assume a future repo implementation can infer missing issue fields or support tiers from vague prose; those contracts must stay explicit here.
- Do not collapse the operator promotion model into tester-facing branch UX.
- Do not treat the current GE-08 workbench as permission to skip the new tester-workbench boundary definition.
