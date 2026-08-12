# Codex Demo Step Cards

## GE10-E1 FLOW: Vanilla machine prerequisites and desktop shell build

### Description
Prepare the currently verified Linux developer machine, install the required system and toolchain dependencies, install the repo-local desktop dependencies, and complete the first desktop-shell build before any later demo step begins.

### DoD
- Linux desktop prerequisite packages are installed.
- Rust is installed or activated and `cargo --version` succeeds.
- Node.js and npm are installed and version checks succeed.
- `npm ci` completed under `apps/desktop`.
- `npm run typecheck`, `npm run build`, `npm run tauri:check`, and `npx tauri build --debug` all succeed.
- The debug desktop binary exists under `apps/desktop/src-tauri/target/debug/codex_desktop_shell_scaffold`.

### Prereqs
- A Linux desktop machine on the currently verified Ubuntu 24.04-style path.
- Network access for package/toolchain installation.
- Local checkout of `/home/ubuntu/workspace/repos/codex` or equivalent repo clone.

### Outputs
- A machine ready to proceed through later demo steps without stopping for new prerequisites.
- The first successful local desktop-shell build.
- A confirmed repo-local `npx tauri` path.

### Inputs
- `programs/codex/requirements/GE-10-demo-proof-and-onboarding/artifacts/vanilla-machine-demo-runbook.md`
- `repos/codex/apps/desktop/package.json`
- Tauri v2 Linux prerequisite documentation

### Next step
Proceed to `GE10-E2 FLOW: Headless proof-surface verification`.

## GE10-E2 FLOW: Headless proof-surface verification

### Description
Run the Rust proof harness and the bounded GE-06 and GE-08 proof slices so the operator sees what the current non-UI truth surfaces actually prove.

### DoD
- Root `cargo test` succeeds.
- Focused `cargo test ge06_` succeeds.
- Focused `cargo test ge08_` succeeds.
- The operator can explain what GE-06 and GE-08 prove and what they do not prove.

### Prereqs
- `GE10-E1` completed successfully.
- Rust toolchain available in the current shell.

### Outputs
- Verified headless proof receipts.
- Operator understanding of the bounded proof posture.

### Inputs
- `repos/codex/tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt`
- `repos/codex/tests/fixtures/ge08/guard-stance-package/manifest.yaml`
- `programs/codex/requirements/GE-10-demo-proof-and-onboarding/artifacts/current-project-state-summary.md`

### Next step
Proceed to `GE10-E3 FLOW: Desktop boundary verification and artifact build`.

## GE10-E3 FLOW: Desktop boundary verification and artifact build

### Description
Reconfirm the frontend and Tauri build surfaces and ensure the built desktop artifact is explicit before attempting GUI launch.

### DoD
- `npm run typecheck` succeeds.
- `npm run build` succeeds.
- `npm run tauri:check` succeeds.
- `npx tauri build --debug` succeeds.
- The operator can point to the built debug binary path.

### Prereqs
- `GE10-E1` completed successfully.
- Repo-local Node dependencies already installed.

### Outputs
- Verified frontend/Tauri build receipts.
- Built debug desktop binary ready for GUI launch.

### Inputs
- `repos/codex/apps/desktop/package.json`
- `repos/codex/README.md`
- `programs/codex/requirements/GE-10-demo-proof-and-onboarding/artifacts/vanilla-machine-demo-runbook.md`

### Next step
Proceed to `GE10-E4 FLOW: Interactive desktop workbench walkthrough`.

## GE10-E4 FLOW: Interactive desktop workbench walkthrough

### Description
Launch the current bounded desktop shell on a graphical Linux session and inspect the GE-08 Guard Stance workbench surface without overstating what the UI currently is.

### DoD
- The operator launches `npx tauri dev` or the built debug binary from a graphical Linux session.
- The app opens successfully.
- The operator observes the bounded GE-08 workbench behavior: package state, preview state, and structured snapshot.
- The operator states plainly that this is a bounded workbench/demo surface, not a general character builder.

### Prereqs
- `GE10-E3` completed successfully.
- A graphical Linux desktop session is available.

### Outputs
- Visible confirmation that the desktop shell launches on a GUI-capable machine.
- Shared understanding of the current product-visible surface and its limits.

### Inputs
- `repos/codex/apps/desktop/src/App.tsx`
- `repos/codex/apps/desktop/src-tauri/src/main.rs`
- `programs/codex/requirements/GE-10-demo-proof-and-onboarding/artifacts/current-project-state-summary.md`

### Next step
Proceed to `GE10-E5 FLOW: Current-state and onboarding review`.

## GE10-E5 FLOW: Current-state and onboarding review

### Description
Close the walkthrough by reviewing the explicit current-state summary and new-developer onboarding checklist so the demo ends with accurate orientation instead of inflated confidence.

### DoD
- The operator reads the current-state summary.
- The operator reads the onboarding checklist.
- The operator can explain the difference between current proof surfaces and unfinished product scope.
- Any drift noticed during the walkthrough is recorded for GE-10 upkeep.

### Prereqs
- At least `GE10-E2` completed.
- Preferably `GE10-E4` completed when a GUI session was available.

### Outputs
- Accurate shared narrative of the project’s current state.
- A usable onboarding handoff for incoming developers.

### Inputs
- `programs/codex/requirements/GE-10-demo-proof-and-onboarding/artifacts/current-project-state-summary.md`
- `programs/codex/requirements/GE-10-demo-proof-and-onboarding/artifacts/developer-onboarding-checklist.md`
- `repos/codex/README.md`

### Next step
If the repo has changed materially, route a GE-10 upkeep pass before the next demo run.
