# SD-15 Triage Class Dictionary

## Purpose
Define the bounded SD-15 operator taxonomy so incoming tester reports can be classified without collapsing UI defects, rules failures, unsupported paths, packaging trouble, persistence trouble, and status drift into one counterfeit backlog bucket.

## Classification posture
- assign one primary SD-15 class per report; record secondary adjacent-authority context separately when needed
- preserve the SD-11 intake schema exactly as-is; this dictionary governs downstream operator classification, not tester-facing issue UX
- enhancement requests remain SD-11 enhancement intake unless the evidence shows a current bounded defect, unsupported path, or status/documentation drift condition

## Outcome vocabulary
- `defect` — evidence supports a real in-scope failure inside the accepted bounded surface
- `unsupported` — the report hits an explicitly unsupported path, platform, support tier, or semantic boundary
- `partial` — the path exists but the adjacent authority already says some semantics remain incomplete or bounded
- `not-yet-verified` — the report may be real, but the evidence is still insufficient to prove defect versus unsupported versus drift
- `blocked` — triage cannot proceed truthfully because a required artifact, environment detail, reproduction step, or evidence handle is missing or unavailable
- `status-drift` — the main problem is contradiction between durable truth surfaces and evidence, not just a product behavior failure

## 1. UI or presentation defects
- Meaning: tester-visible layout, wording, navigation, diagnostics, explanation, or status-rendering behavior is wrong or misleading inside the bounded SD-11 workbench surface.
- Minimum evidence threshold:
  - observed behavior
  - expected behavior
  - build label/version and tester-facing channel/support label
  - platform / OS
  - current bounded workflow
  - reproduction steps or explicit reproduction-impossibility note
  - screenshot or visible rendering evidence when available
- Adjacent authority reference:
  - `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md`
  - `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/update-channel-and-promotion-mapping.md` when the defect concerns channel/support wording
- Visible treatment rules:
  - use `partial` or `unsupported` instead of a UI defect when the screen is truthfully exposing an SD-13, SD-14, or SD-12 bound
  - use `status-drift` when UI text contradicts accepted channel/support, breadth, persistence, or closure truth
  - use `not-yet-verified` when the report lacks visible evidence and cannot be distinguished from misunderstanding

## 2. Rules-engine defects
- Meaning: calculations, gating, derived statistics, validation, progression, or rules application are wrong within a bounded supported path.
- Minimum evidence threshold:
  - observed behavior
  - expected behavior
  - reproduction steps or impossibility note
  - build/channel/platform/workflow identity
  - affected class, race, level, choice, or progression step when relevant
  - diagnostics, explanation, or provenance references when present
- Adjacent authority reference:
  - `programs/codex/plans/spec-domains/SD-13-core-class-race-roster-and-level-10-progression-matrix.md` for bounded roster/progression truth
  - `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md` for the tester-visible workflow context
- Visible treatment rules:
  - use `unsupported` or `partial` when SD-13 already says the roster/progression path is not fully supported
  - use `not-yet-verified` when class/race/level specifics or reproduction detail are missing
  - use `blocked` when the operator cannot reconstruct the claim from the supplied inputs or missing save/evidence handles

## 3. Content or data defects
- Meaning: the loaded content, source values, option inventory, labels, mappings, or data-source behavior are wrong even if the runtime is otherwise functioning.
- Minimum evidence threshold:
  - observed incorrect content or data behavior
  - expected source truth
  - build/channel/platform/workflow identity
  - exact affected content identity when known
  - current data-source identity from SD-11 intake
  - screenshot, export, or diagnostics when available
- Adjacent authority reference:
  - `programs/codex/plans/spec-domains/SD-13-core-class-race-roster-and-level-10-progression-matrix.md` when the report touches bounded roster/progression coverage
  - `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/artifacts/tester-feedback-evidence-capture-matrix.md` for current data-source capture posture
- Visible treatment rules:
  - use `partial` or `unsupported` when the missing content is an accepted breadth limit rather than a broken supported path
  - use `not-yet-verified` when the report does not identify the affected content source or option precisely enough to test
  - use `status-drift` when durable status surfaces claim content support that the evidence contradicts

## 4. Unsupported semantics or known unsupported paths
- Meaning: the report proves the tester encountered a path outside accepted support truth, or inside a path that adjacent authority already marks partial, blocked, or not yet verified.
- Minimum evidence threshold:
  - exact attempted workflow or feature path
  - build/channel/platform identity
  - support warning, known bound, or adjacent authority citation showing the path is unsupported or partial when available
  - enough context to tell what the tester was trying to do
- Adjacent authority reference:
  - `programs/codex/plans/spec-domains/SD-13-core-class-race-roster-and-level-10-progression-matrix.md` for bounded breadth/progression support-state truth
  - `programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/README.md` for persistence and migration boundaries
  - `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md` for unsupported platform/channel/update paths
- Visible treatment rules:
  - keep `unsupported`, `partial`, or `not-yet-verified` visible; do not relabel these as ordinary defects for convenience
  - if the adjacent authority is not yet strong enough to prove unsupported status, hold the report as `not-yet-verified` rather than auto-dismissing it
  - route to `status-drift` only when durable surfaces overclaim support beyond the accepted bound

## 5. Packaging or distribution defects
- Meaning: the governed build artifact, publication surface, promotion state, update source, rollback state, or acquisition path is wrong before ordinary install/use proof can even start.
- Minimum evidence threshold:
  - tester-visible build label or expected build identity
  - tester-facing channel/support label
  - platform and package/install context
  - acquisition path, artifact URL, or publication handle when available
  - observed failure or missing-artifact evidence
  - rollback/withdrawal context when relevant
- Adjacent authority reference:
  - `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md`
  - `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/artifacts/rollback-withdrawal-and-downgrade-policy.md`
- Visible treatment rules:
  - use `unsupported` when the report is on an explicitly out-of-scope platform or support tier
  - use `blocked` when the artifact cannot be obtained or its provenance cannot be established yet
  - use `status-drift` when durable surfaces claim an artifact or channel state that the governed publication evidence does not support

## 6. Install/use defects
- Meaning: a governed build can be acquired, but installation, first launch, workbench entry, or bounded workflow use fails on a named environment.
- Minimum evidence threshold:
  - build identity and tester-facing channel/support label
  - platform and package/install context
  - exact install/use step that failed
  - observed versus expected result
  - logs, screenshots, or status evidence when available
  - explicit note whether the run was a clean-machine or authoring-machine attempt when known
- Adjacent authority reference:
  - `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md` for build/channel/platform/update truth
  - `programs/codex/requirements/SD-11-test-user-workbench-and-github-feedback-intake/README.md` for the bounded workbench entry surface
- Visible treatment rules:
  - use `blocked` when a missing prerequisite or environment condition stops the proof path before the product behavior can be judged
  - use `unsupported` for out-of-scope platform/tier paths
  - reclassify to persistence, rules, or content only if install/use succeeds and the failure clearly belongs deeper in the workflow

## 7. Persistence, migration, or saved-state continuity defects
- Meaning: save/load/reopen/revise/migrate/upgrade-safe behavior, or the diagnostics around those operations, fails or contradicts the accepted saved-state contract.
- Minimum evidence threshold:
  - exact continuity step under test
  - build/channel/platform identity
  - observed versus expected behavior
  - reproduction or migration path
  - save artifact, log, or attachment handle when available
  - redaction posture for save files or logs when attachments exist
- Adjacent authority reference:
  - `programs/codex/requirements/SD-14-character-lifecycle-persistence-and-upgrade-safe-revision/README.md`
  - `programs/codex/requirements/SD-12-linux-first-distribution-branch-promotion-channels-and-self-update/README.md` when the report crosses an update/rollback boundary
- Visible treatment rules:
  - keep `blocked`, `partial`, or `not-yet-verified` visible when the save artifact, compatibility vector, or migration context is missing
  - do not let update success stand in for persistence success
  - use `status-drift` when durable status surfaces claim upgrade-safe continuity without named receipts or accepted saved-state proof

## 8. Status or documentation drift
- Meaning: durable truth surfaces make materially incompatible claims about support, install/use reality, channel/update posture, breadth, persistence, or tranche-closure status.
- Minimum evidence threshold:
  - at least two conflicting durable surfaces, or one durable claim contradicted by a named receipt/evidence bundle
  - exact claim family in conflict
  - the current evidence basis that shows the contradiction
- Adjacent authority reference:
  - `programs/codex/requirements/SD-15-test-program-operations-regression-and-triage/artifacts/tranche-2-project-status-truth-reconciliation-checklist.md`
  - the relevant adjacent authority surface being contradicted: SD-11, SD-12, SD-13, or SD-14
- Visible treatment rules:
  - keep `status-drift` as the primary outcome when the contradiction itself changes operator decisions
  - add `blocked` only when missing evidence prevents a reconciliation verdict
  - do not downgrade material drift to an ordinary UI or wording bug when the real issue is control-plane truth divergence

## Cross-class guardrails
- do not classify a report as `unsupported` without naming the adjacent authority that establishes the bound
- do not classify a report as `defect` when the evidence only proves contradiction between surfaces; that is `status-drift`
- do not classify a persistence-facing failure under packaging or install/use once the build is already running and the failure is about saved-state continuity
- do not collapse third-class-platform pain into generic bugs when SD-12 explicitly bounds that platform posture
