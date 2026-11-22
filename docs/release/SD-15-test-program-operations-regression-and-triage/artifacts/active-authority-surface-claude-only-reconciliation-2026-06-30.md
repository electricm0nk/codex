# Active authority-surface Claude-only reconciliation

## Purpose
Record the active Codex SD authority surfaces reviewed for legacy launch-authority drift and state whether each surface was normalized, already correct, or left as legacy record only.

## Decision rule used in this slice
- `normalized-active` = an allowed active SD authority surface was edited to remove stale `equivalent frontier coding harness` wording and/or stale `awaiting-launch-review` state while preserving Claude-only and block-on-non-Claude doctrine.
- `already-correct` = an allowed active SD authority surface already matched the front-loaded Claude-only doctrine and needed no edit.
- `legacy-record-only` = a discovered legacy hit exists outside the allowed write scope and was recorded but not rewritten in this slice.
- `follow-on-required` = a discovered surface would still need later reconciliation after this slice because it is active authority but not writable here.

## Reviewed active SD authority surfaces
| Surface | Classification | Action taken | Notes |
|---|---|---|---|
| `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/sd11-update-action-execution-handoff.md` | `normalized-active` | Replaced legacy `Claude Code or equivalent frontier coding harness` wording with `Claude Code only` in frontmatter and `Run in`, and added explicit block-on-non-Claude wording. | Historical PR #34 truth remains preserved. |
| `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e2-r2-github-publication-and-promotion-handoff-2026-06-29.md` | `normalized-active` | Replaced legacy equivalent-harness wording with `Claude Code only` in frontmatter and `Run in`, and added explicit block-on-non-Claude wording. | Historical merged PR #32 truth remains preserved. |
| `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e4-r2-rollback-withdrawal-and-downgrade-handoff-2026-06-29.md` | `normalized-active` | Replaced legacy equivalent-harness wording with `Claude Code only once unblocked` in frontmatter and `Run in`, and made the blocked-state instruction explicit: if Claude cannot be launched truthfully once unblocked, the lane stays blocked. | Blocked prebuild posture remains unchanged. |
| `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/sd12-e5-r2-provenance-integrity-and-update-eligibility-handoff-2026-06-29.md` | `normalized-active` | Replaced legacy equivalent-harness wording with `Claude Code only once unblocked` in frontmatter and `Run in`, and made the blocked-state instruction explicit: if Claude cannot be launched truthfully once unblocked, the lane stays blocked. | Blocked prebuild posture remains unchanged. |
| `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e1-r2-matrix-schema-and-seeded-current-state-execution-handoff-2026-06-30.md` | `normalized-active` | Promoted stale `status: awaiting-launch-review` to `status: ready-for-claude-launch` and tightened `run_in` / `Run in` to `Claude Code only`. | The downstream CODE lane already exists, so launch-review wording was stale under the front-loaded approval doctrine. |
| `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e2-r2-core-race-semantic-execution-handoff-2026-06-30.md` | `already-correct` | No edit. | This surface already carried `status: ready-for-claude-launch` plus `Claude Code only` wording and explicit no-substitute doctrine. |
| `programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/artifacts/sd13-e6-r2-unsupported-depth-diagnostics-and-tester-visible-reporting-execution-handoff-2026-06-30.md` | `normalized-active` | Promoted stale `status: awaiting-launch-review` to `status: ready-for-claude-launch` and tightened `run_in` / `Run in` to `Claude Code only`. | The downstream CODE lane already exists, so launch-review wording was stale under the front-loaded approval doctrine. |

## Verification on allowed active SD surfaces
Searches over each of the seven allowed active SD surfaces returned zero remaining hits for either of the legacy drift markers below:
- `Claude Code or equivalent frontier coding harness`
- `Claude Code or an equivalent frontier coding harness`
- `awaiting-launch-review`

Interpretation:
- the allowed active SD corpus no longer claims an equivalent frontier harness where Claude-only doctrine is required
- the allowed active SD corpus no longer leaves `awaiting-launch-review` on the reviewed slices that are already governed by front-loaded approval

## Remaining legacy hits discovered outside allowed write scope
Broad search over `programs/codex/requirements/` after authoring this report returns 21 paths total:
- this report itself, because it records the marker phrases for auditability
- 20 actual out-of-scope legacy-hit files

None of the 20 out-of-scope files were rewritten here because the task explicitly limited write authority to the seven active SD handoff surfaces above plus this report, and none of those remaining hits were proven active authority for current SD lanes.

### Classification: `legacy-record-only`
All 20 remaining out-of-scope hits below are GE-era historical execution or prebuild artifacts outside the allowed write scope for this slice.

- `programs/codex/requirements/GE-05-oracle-validation-and-parity-harness/artifacts/ge05-e2-f2-execution-handoff-2026-06-24.md`
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f1a-execution-handoff-2026-06-21.md`
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2a-execution-handoff-2026-06-21.md`
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2b-execution-handoff-2026-06-21.md`
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2c-execution-handoff-2026-06-21.md`
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f2d-execution-handoff-2026-06-21.md`
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e2-f3-execution-handoff-2026-06-21.md`
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f1-execution-handoff-2026-06-22.md`
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f1-prebuild-handoff-2026-06-21.md`
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f2-execution-handoff-2026-06-22.md`
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e3-f2-prebuild-handoff-2026-06-21.md`
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f1-execution-handoff-2026-06-22.md`
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f2-prebuild-handoff-2026-06-22.md`
- `programs/codex/requirements/GE-06-pilot-vertical-slice-pf1-human-fighter/artifacts/ge06-e4-f3-prebuild-handoff-2026-06-22.md`
- `programs/codex/requirements/GE-07-desktop-shell-and-modern-ux/artifacts/ge07-e1-execution-handoff-2026-06-22.md`
- `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e3-f1-execution-handoff-2026-06-27.md`
- `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e3-f1-prebuild-handoff-2026-06-27.md`
- `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e4-f1-execution-handoff-2026-06-27.md`
- `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e4-f1-prebuild-handoff-2026-06-27.md`
- `programs/codex/requirements/GE-08-homebrew-authoring-and-rules-studio/artifacts/ge08-e5-f1-execution-handoff-2026-06-27.md`

Reason left unchanged:
- they are outside the allowed write scope for this slice
- they were discovered as historical GE-era record surfaces, not as proven active authority for the current SD lanes under review
- rewriting them here would violate the bounded-scope rule and risk silently converting archival history into active doctrine work

## Follow-on-required assessment
No `follow-on-required` item was created in this slice.

Reason:
- every remaining legacy hit discovered by the verification search lives in out-of-scope GE-era record surfaces rather than in the reviewed active SD authority corpus
- the allowed active SD corpus is now normalized or already correct for the legacy markers named in this task

## Final verdict
The active SD authority corpus named in this card is reconciled to the Claude-only, front-loaded approval doctrine within the allowed write scope. Remaining legacy hits are recorded explicitly as out-of-scope historical GE-era surfaces rather than silently left undisclosed.
