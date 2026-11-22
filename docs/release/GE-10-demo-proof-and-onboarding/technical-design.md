---
title: GE-10 Technical Design
stc_id: STC-CODEX-GE-10
artifact_type: technical-design
status: draft
scope: programs/codex/requirements/GE-10-demo-proof-and-onboarding
source_stc: ./README.md
---

# GE-10 Technical Design

## Design goal

Turn the current Codex proof surfaces into one coherent operator experience without collapsing governance, runtime proof, and onboarding into the same undifferentiated prose blob.

## Design posture

GE-10 uses a **layered documentary design**:

1. **Repo README** — first-contact surface for any developer entering `repos/codex`
2. **Demo runbook** — the authoritative walkthrough with exact sequence and commands
3. **Current-state summary** — honest status classification for what is real today
4. **Onboarding checklist** — minimal intake path for developers joining the project
5. **Demo-step cards** — governed board-ready per-step units that can be tracked or replayed independently

This keeps each document narrow enough to stay readable while still making the whole onboarding path coherent.

## Surface responsibilities

### 1. Repo README
Use the repo README for:
- a concise project identity statement
- current state at a glance
- verified getting-started instructions for the current platform
- build/test/demo entrypoint commands
- known limitations

Do **not** use it as the only location for every nuance. The README should point at the current truth, not absorb the entire governance corpus.

### 2. Demo runbook
Use the runbook for:
- exact step ordering
- exact commands
- expected intermediate outcomes
- demo narration
- headless-versus-GUI distinctions
- platform assumptions

The runbook is the canonical “do this in order” surface.

### 3. Current-state summary
Use the current-state summary for:
- capability classification
- proof-surface inventory
- verified-versus-unverified distinctions
- known limits and confidence boundaries

This isolates status truth from procedural instructions.

### 4. Onboarding checklist
Use the onboarding checklist for:
- minimum reads
- minimum commands
- boundary and doctrine reminders
- anti-confusion rules for new contributors

This avoids forcing every developer to re-derive repo behavior from the full program lattice.

### 5. Demo-step cards
Use the demo-step cards for:
- board-ready decomposition
- one bounded step per card
- explicit handoff between steps
- repeatable progress tracking

## Demo sequence design

The demo sequence is intentionally front-loaded:

### Step 1
Install prerequisites and complete the first desktop-shell build.

Why first:
- it removes later pauses for missing dependencies
- it validates the environment before the operator invests attention elsewhere
- it proves the desktop boundary is buildable before later steps narrate the workbench

### Step 2
Run the headless proof surfaces.

Why second:
- the Rust core is the truth surface beneath the UI
- it proves the bounded rules and preview substrate without depending on a graphical session

### Step 3
Reconfirm the desktop boundary surface and build artifacts.

Why third:
- it keeps the operator aware that the desktop shell is real, but still bounded
- it ensures the built artifact exists before the GUI launch step

### Step 4
Launch the desktop shell in a graphical session and inspect the bounded GE-08 workbench behavior.

Why fourth:
- GUI launch is the first step that depends on a desktop session
- this avoids confusing buildability with launchability

### Step 5
Review current state and onboarding rules.

Why fifth:
- by this point the operator has seen the proof surfaces directly
- the current-state narrative becomes anchored in experience rather than rhetoric

## Maintenance rule

When the repo advances materially, refresh GE-10 from the live proof surfaces outward:

1. rerun verification commands
2. update the current-state summary
3. update the demo runbook if the steps changed
4. update the README summary if the high-level story changed
5. update cards only if the step boundaries changed

## Data flow

```text
live repo verification
  -> current-project-state-summary.md
  -> vanilla-machine-demo-runbook.md
  -> repos/codex/README.md
  -> demo-step-cards.md
  -> developer-onboarding-checklist.md
```

The current-state summary is the anti-drift anchor. The README is the first-contact summary. The runbook is the execution surface. The cards are the board surface.
