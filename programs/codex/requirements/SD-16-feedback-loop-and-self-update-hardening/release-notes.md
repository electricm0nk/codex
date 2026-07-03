# Release Notes: SD-16 Feedback Loop and Self-Update Hardening

## Summary

Tranche 2.5 makes the Codex shell scaffold the real operational path for defect submission and self-update. The shell will be able to file a defect through a real GitHub browser handoff, consume governed release-channel indexes, display tranche release notes, install an AppImage update through a staged transaction, verify relaunch, and prove that the originally reported defect is fixed in the updated build.

## User-Visible Changes

- Add shell defect submission flow that opens a prefilled GitHub issue form.
- Add alpha/beta/stable update channel selector.
- Add Check action that fetches the release-lane-generated channel index and displays release notes/update eligibility.
- Add Install action for eligible Linux AppImage builds.
- Add compact updater diagnostics showing installed state, last check, pending/rollback state, and update storage status.
- Show explicit disabled reasons for local/dev/non-AppImage/non-writable/ineligible update contexts.

## Defects Fixed

- Shell defect logging is no longer a side-channel/manual process after this tranche is complete.
- Shell updates are no longer manual download/install or mock status experiences after this tranche is complete.
- Prior release/channel ambiguity is repaired by introducing `develop=alpha`, `test=beta`, `main=stable`, protected promotion PRs, and channel indexes.
- Prior release-note ambiguity is repaired by using canonical tranche release notes generated at tranche inception and amended by PR review.

## Operational Notes

- Linux AppImage is the first-class update target for Tranche 2.5.
- `.deb` may remain a secondary install artifact but is not the first self-update path.
- The `update-index` branch is a protected update-control surface; only the release lane writes channel index files in the normal path.
- Signing is deferred, but schema fields reserve future signing support.
- Todd retains approval/merge authority for promotion PRs unless explicitly delegated.

## Verification Evidence

Final release evidence must include:

- GitHub issue URL
- PR URL
- merge commit
- GitHub Release URL
- update-index commit
- manifest URL
- checksum verification result
- shell Check evidence
- shell Install/relaunch verification evidence
- post-update installed-state
- defect-fixed verification

This section must be amended by each Spec Domain PR when execution produces concrete evidence.

## Known Issues

- Cryptographic signing is not implemented in Tranche 2.5; signature fields are reserved for later hardening.
- Browser issue handoff does not prove issue submission inside the shell; acceptance verifies the real GitHub issue exists externally.
- macOS and Windows updater mechanics are out of scope for this tranche.
- Repair/reinstall mode is deferred unless separately scoped.

## Update Eligibility

- Eligible install target: Linux AppImage official governed builds.
- Local/dev builds may Check for release notes but cannot Install.
- Install requires:
  - selected channel index validates
  - update manifest validates
  - AppImage checksum validates
  - current managed executable path matches installed-state
  - managed path is writable or stageable
  - selected release is newer/eligible under parsed channel/version/artifact policy
- Final alpha acceptance requires the shell to consume an alpha release produced from `develop` and update through the shell.
