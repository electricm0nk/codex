---
title: GE08-E4-F1 PR-Created Receipt — Headless Preview and Explanation Bridge for First Proof Package
artifact_type: pr-created-receipt
stc_id: STC-CODEX-GE-08
slice_id: GE08-E4-F1
workflow_route: gate-receipt
status: pr-open
review_date: 2026-06-27
owner: Todd Hintzmann
scope: program
code_authority: false
related_artifacts:
  - ./ge08-e4-f1-branch-ready-receipt-2026-06-27.md
  - ../execution-handoff.md
---

# GE08-E4-F1 PR-Created Receipt

## Verdict
GE08-E4-F1 is no longer only branch-ready. It now has a real GitHub review surface and an explicit human merge boundary.

## Pull request evidence
- PR: `#20`
- Title: `feat: add GE08 preview bridge`
- URL: https://github.com/electricm0nk/codex/pull/20
- State: `OPEN`
- Draft: `false`
- Mergeability: `MERGEABLE`
- Review decision: none yet
- Base branch: `develop`
- Head branch: `ge08-e4-f1-preview-and-explanation-bridge`

## Commit set in PR
1. `c6c18d2c085dc22cd456977442aa47abc6c267cf` — `feat: capture GE08 E2 package lifecycle substrate`
2. `6de72fcd2ba2bacc79bc4ae4f0a8bf163c875606` — `feat: add GE08 validation diagnostics fixtures and tests`
3. `9c883dc1ee3e8a7de35ecea9cf84bd6c9611cb1f` — `feat: add GE08 preview bridge`

## Verification anchored to the PR head
A detached worktree was created at `origin/ge08-e4-f1-preview-and-explanation-bridge@9c883dc1ee3e8a7de35ecea9cf84bd6c9611cb1f` and the bounded GE08 test set was re-run there.

Commands executed:
```bash
cargo test ge08_package_file_lifecycle -- --nocapture
cargo test ge08_validation_and_diagnostics -- --nocapture
cargo test ge08_preview_bridge -- --nocapture
```

Observed results:
- lifecycle lane: green
- validation/diagnostics lane: green
- preview bridge lane: green (`3 passed`)

## Custody truth
This lane must still be classified carefully.

- Claude was genuinely launched for GE08-E4-F1.
- The surviving Claude result ended with `error_max_turns` rather than a clean durable success receipt.
- Hermes then verified, committed, pushed, and opened the PR.

Final custody classification for the implemented branch remains:

```text
mixed-custody reconciled
```

It is truthful to say the branch is real, verified, pushed, and reviewable. It is not truthful to casually describe the final result as pure Claude execution.

## Human merge boundary
Todd retains merge authority. The next honest operator surface is therefore a visible merge gate over PR `#20`, not another hidden automation step.
