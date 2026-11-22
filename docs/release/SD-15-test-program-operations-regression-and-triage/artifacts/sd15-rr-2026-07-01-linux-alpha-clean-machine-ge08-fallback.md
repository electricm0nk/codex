# SD15-RR-20260701-001 — Linux alpha clean-machine GE08 fallback receipt

```yaml
receipt_id: SD15-RR-20260701-001
intake_handle: clean-machine://codex-phase-2/t_ced36b6f/2026-07-01
created_at: 2026-07-01T05:50:51Z
last_updated_at: 2026-07-01T05:50:51Z
supersedes_receipt_id: null
evidence_owner: god-emporer
build_context:
  build_label_or_version: codex-desktop-shell-scaffold@0.0.0
  tester_channel_support_label: alpha / Linux first-class
  operator_provenance_handle: github-release://electricm0nk/codex/alpha-v0.0.0-c2cea5c6
  commit_or_build_identity: c2cea5c6baeb3ca34077b85331214c4b42a4809c
  publication_or_acquisition_handle: https://github.com/electricm0nk/codex/actions/runs/28463728483
  rollback_withdrawal_context: not-applicable
platform_context:
  platform_os: Ubuntu 24.04.4 LTS
  platform_architecture: x86_64
  package_install_context: deb package installed inside a disposable Ubuntu 24.04 container, then launched under dbus-run-session + xvfb-run
  environment_kind: clean-machine
  environment_identity_handle: docker://ubuntu@sha256:786a8b558f7be160c6c8c4a54f9a57274f3b4fb1491cf65146521ae77ff1dc54/sd15-clean-run-3
workflow_context:
  bounded_workflow_under_test: SD-11 tester workbench -> preferred GE08 workbench / Guard Stance proof package
  current_data_source_identity: explicit fallback over a pilot seam; GE07 pilot snapshot seam shown as Unknown/Unavailable because GE08 package load failed
  primary_sd15_class: content or data defect
  outcome_state: defect
  adjacent_authority_references:
    - /home/ubuntu/workspace/repos/codex/README.md#run-the-current-demo
    - /home/ubuntu/workspace/repos/codex/README.md#app-window-shows-package-root-does-not-exist-for-testsfixturesge08guard-stance-package
    - programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md
    - programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/triage-class-dictionary.md
  sd13_support_state_context: not-applicable to roster breadth; the failure occurs before Guard Stance package content can load into the preferred GE08 workbench
  sd14_persistence_migration_context: not-applicable
claim_statement:
  observed_behavior: The official Linux alpha deb launches on a fresh Ubuntu 24.04 container and opens an explicitly labeled SD-11 tester workbench fallback. The preferred GE08 authoring workbench does not load. The UI reports that the Guard Stance package root does not exist and resolves the path to /home/runner/work/codex/codex/tests/fixtures/ge08/guard-stance-package.
  expected_behavior: The bounded GE08 workbench should load the Guard Stance proof package and display package state, preview state, and a structured snapshot on the clean environment.
  claim_summary: Clean-machine install/use proof succeeded through launch and bounded fallback entry, but the governed alpha artifact still reproduces the missing Guard Stance package-root failure on the preferred GE08 workbench path.
  attempted_goal_or_mission: Acquire the governed Linux alpha artifact, install it on a named clean environment, launch it, and reach the bounded GE08 tester workbench path.
evidence:
  reproduction_status: reproduced
  reproduction_steps_or_impossibility_note: Downloaded the governed alpha deb plus provenance/checksum files, verified the deb checksum against checksums.sha256, installed the deb inside a disposable Ubuntu 24.04 container, launched /usr/bin/codex_desktop_shell_scaffold under dbus-run-session + xvfb-run, waited for the window to render, and captured the visible workbench state plus diagnostics.
  diagnostics_or_status_evidence:
    - artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/window.png
    - artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/xwininfo.txt
    - artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/app.stderr.log
    - artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/provenance.json
    - artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/checksums.sha256
  evidence_sufficiency_note: Current and complete for the named clean-machine Linux alpha path. The receipt proves governed artifact acquisition, deb installation, first launch, visible workbench entry, and the deeper GE08 package-root failure. It does not prove a fix or broader tranche closure.
  next_required_surface: Route a fix to the packaged GE08 package-root/load path, publish a superseding alpha artifact, then rerun the clean-machine validation report and install/use matrix row against that superseding build.
attachments:
  attachment_handles:
    - artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/window.png
    - artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/xwininfo.txt
    - artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/app.stderr.log
    - artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/app-process.txt
    - artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/os-release.txt
    - artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/dpkg-status.txt
    - artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/artifact.sha256.txt
    - artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/provenance.json
    - artifacts/evidence/2026-07-01-clean-machine-linux-alpha-c2cea5c6/checksums.sha256
  attachment_types:
    - screenshot
    - x11-window-tree
    - stderr-log
    - process-record
    - os-release
    - package-status
    - checksum
    - provenance-metadata
    - release-checksums
  redaction_posture: attached as-is
  redaction_reason: not-applicable
```

## Operator conclusion
- This is no longer a blocked clean-machine path. The governed Linux alpha deb was acquired, installed, launched, and observed on a named clean environment.
- The blocker cleared into a real defect receipt: the preferred GE08 workbench still falls back because the packaged build resolves the Guard Stance package root to a nonexistent path.
- The next honest move is a fix + superseding alpha publication + rerun, not continued claims that clean-machine proof is merely pending.
