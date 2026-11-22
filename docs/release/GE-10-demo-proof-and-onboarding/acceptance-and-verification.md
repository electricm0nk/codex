---
title: GE-10 Acceptance and Verification
stc_id: STC-CODEX-GE-10
artifact_type: acceptance-and-verification
status: draft
scope: programs/codex/requirements/GE-10-demo-proof-and-onboarding
source_stc: ./README.md
---

# GE-10 Acceptance and Verification

## Acceptance criteria

| ID | Requirement | Acceptance rule | Verification |
|---|---|---|---|
| GE10-AV-01 | Repo-root truth surface | `repos/codex/README.md` contains current state, getting started, verification/demo commands, and known limitations. | Read the README and confirm the sections exist and reference real repo paths/commands. |
| GE10-AV-02 | Front-loaded prerequisite closure | Step 1 of the canonical runbook installs prerequisites and completes the first desktop-shell build before later steps require additional tooling. | Read `artifacts/vanilla-machine-demo-runbook.md` and confirm Step 1 covers prerequisite install, `npm ci`, `npm run typecheck`, `npm run build`, `npm run tauri:check`, and `npx tauri build --debug`. |
| GE10-AV-03 | Headless proof coverage | The demo path includes the current bounded Rust proof surfaces. | Confirm the runbook and README cite `cargo test`, `cargo test ge06_`, and `cargo test ge08_` or equivalent bounded proof instructions. |
| GE10-AV-04 | Desktop boundary coverage | The demo path includes the current TypeScript/Tauri verification surfaces and GUI-launch distinction. | Confirm the runbook and README cite `npm run typecheck`, `npm run build`, `npm run tauri:check`, `npx tauri build --debug`, and a graphical-session launch step. |
| GE10-AV-05 | Governed card set | A demo-step card artifact exists with one card per step using the required card body grammar. | Read `artifacts/demo-step-cards.md` and confirm each card uses Description, DoD, Prereqs, Outputs, Inputs, and Next step. |
| GE10-AV-06 | Honest current-state narrative | A current-state summary exists and explicitly distinguishes bounded proof surfaces from unfinished product scope. | Read `artifacts/current-project-state-summary.md` and confirm it does not claim general end-user-product readiness. |
| GE10-AV-07 | New-developer intake | An onboarding checklist exists with reads, commands, and anti-confusion rules. | Read `artifacts/developer-onboarding-checklist.md` and confirm it includes minimum reads, commands, and boundary rules. |

## Live verification commands used for this pass

These commands were run against the live repo during the GE-10 grounding pass.

### Core Rust surface

```bash
cd /home/ubuntu/workspace/repos/codex
export PATH="$HOME/.cargo/bin:$PATH"
cargo test
```

Observed result:
- command exited successfully
- GE-03, GE-05, GE-06, and GE-08 proof tests were listed and executed successfully

### Desktop TypeScript/Tauri surface

```bash
cd /home/ubuntu/workspace/repos/codex/apps/desktop
npm run typecheck
npm run build
npm run tauri:check
npx tauri build --debug
```

Observed result:
- all commands exited successfully
- production frontend bundle was built
- Tauri compile/build path succeeded

### GUI launch boundary check

Observed truth from the verification pass:
- the built desktop binary exists
- launching it from the current headless shell failed because GTK could not initialize without a graphical session

This is a platform/session boundary, not proof that the build is invalid.

## Verification obligations for future refreshes

When GE-10 is updated later, rerun at least:

```bash
cd /home/ubuntu/workspace/repos/codex
export PATH="$HOME/.cargo/bin:$PATH"
cargo test

cd /home/ubuntu/workspace/repos/codex/apps/desktop
npm run typecheck
npm run build
npm run tauri:check
npx tauri build --debug
```

If any command fails, downgrade the current-state narrative before editing the README upward.
