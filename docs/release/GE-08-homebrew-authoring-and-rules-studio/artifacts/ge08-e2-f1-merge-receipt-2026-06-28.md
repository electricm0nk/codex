---
title: GE08-E2-F1 Merge Receipt
artifact_type: implementation-merge-receipt
stc_id: STC-CODEX-GE-08
source_stc: ../README.md
source_handoff: ./ge08-e2-f1-branch-ready-receipt-2026-06-27.md
selected_slice: GE08-E2-F1 — Headless package lifecycle for first proof package
workflow_route: coding
status: merged
merge_date: 2026-06-28
owner: Todd Hintzmann
scope: program
code_authority: false
direct_pull_request: none
transitive_landing_pull_request: https://github.com/electricm0nk/codex/pull/20
related_artifacts:
  - ./ge08-e2-f1-branch-ready-receipt-2026-06-27.md
---

# GE08-E2-F1 Merge Receipt

## Verdict
GE08-E2-F1 is now present on `develop`, but it did **not** land through its own standalone PR.

The exact package-lifecycle commit `c6c18d2c085dc22cd456977442aa47abc6c267cf` was carried transitively inside merged PR `#20` (`ge08-e4-f1-preview-and-explanation-bridge`). This receipt exists to preserve that truth and to retire the false impression that E2 merely remained branch-ready forever.

## Verified repository state
Observed from the live repo after `git fetch origin --prune` and ancestry inspection:

```text
repo: /home/ubuntu/workspace/repos/codex
current origin/develop: 43314de3f7e60b8a6f758adfcb013f2abe4f197b
branch-ready commit: c6c18d2c085dc22cd456977442aa47abc6c267cf
first containing merge commit on develop: 4155c48c6b4248d398de50004b0525ad2d8eb01a
merge: Merge pull request #20 from electricm0nk/ge08-e4-f1-preview-and-explanation-bridge
direct PR for ge08-e2-f1-package-lifecycle: none
landing path: transitive via PR #20 commit set
remote feature branch on origin: deleted during governance repair after merge truth was captured
```

## Landed files

```text
src/homebrew_authoring/mod.rs
src/homebrew_authoring/package_manifest.rs
src/homebrew_authoring/package_store.rs
tests/ge08_package_file_lifecycle.rs
tests/fixtures/ge08/guard-stance-package/manifest.yaml
tests/fixtures/ge08/guard-stance-package/metadata/diagnostics.yaml
tests/fixtures/ge08/guard-stance-package/metadata/provenance.yaml
tests/fixtures/ge08/guard-stance-package/objects/feats/feat.homebrew.guard_stance.yaml
tests/fixtures/ge08/guard-stance-package/rules/effects/effect.homebrew.guard_stance.ac_bonus.yaml
tests/fixtures/ge08/guard-stance-package/rules/prerequisites/prerequisite.homebrew.guard_stance.dex13.yaml
```

Diff footprint observed from `cc4e1a55caad07af83768a036d4b0f5fffbf99c9..c6c18d2c085dc22cd456977442aa47abc6c267cf`:

```text
10 files changed, 1443 insertions(+)
```

## Verified behavior
Executed on current `develop`:

```bash
cargo test ge08_package_file_lifecycle -- --nocapture
```

Observed result: `ge08_package_file_lifecycle_create_save_load_diff_and_export_gate` passed on current `develop`.

## Governance truth
This was a doctrine miss.

- E2 was pushed to `origin` as a durable feature branch.
- No standalone PR or explicit no-PR exception was created for that branch.
- E3 and later stacked lanes treated E2 as acceptable substrate anyway.
- The E2 commit ultimately landed only because PR `#20` carried it transitively.

Under the repaired doctrine, that branch should have minted an explicit PR/review/merge surface before later stacked successors treated it as durable upstream truth.

## Next truthful move
Preserve this receipt as the historical landing record for E2, keep PR `#20` annotated with the transitive-landing explanation, and require future origin branches to have either an explicit PR surface or an explicit documented no-PR exception before downstream lanes stack on them.
