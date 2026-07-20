# Epic 8 — Criterion 28 — Build-label format fixtures re-anchored to `Codex 0.5.95`

- cycle_id: 2026-07-19T06:15:00Z
- criterion_section: §1.8 Epic 8 — Build Version Numbering (criterion 28)
- row_or_kind: version:build_label_format
- branch_tip_before: 4b79f5c
- rule_set_used: n/a (version metadata, not content-source ingest)

## Why this criterion, this cycle

Re-checked the Epic 3/4/5 corpus-generation blocker first: `corpus/` still
doesn't exist and neither OGL/SRD mirror is reachable from this sandbox, so
nothing has changed since the Alchemist cycle logged it — re-attempting
Epic 3/4/5 would just re-log the same blocker. Epic 6 remains transitively
blocked (needs ≥1 book ingested). Per Step 1's priority order, Epic 8 is
next: criterion 27 (version bump) is already `complete`; the same cycle's
receipt explicitly flagged criterion 28 as verified-but-not-closed — the
`BUILD_PREFIX = 'Codex'` / `${BUILD_PREFIX} ${buildVersion}` format already
ships (inherited from SD-21 E5.26), but the format's own test fixtures still
hard-coded the pre-bump literal `Codex 0.4.94-test` instead of the current
`0.5.95` triple. That's exactly `epic-breakdown.md` criterion 28's remaining
scope ("Test fixtures update to assert/fixture the new `Codex 0.5.<build>`
shape") and it's mechanical — no invented content, just re-syncing hard-coded
fixtures to a value already committed in criterion 27.

## Red-phase evidence

Added `apps/desktop/src/sd22/buildLabelFixtureFreshness.test.ts`, scanning
the three fixture files named in `loop-instruction.md`'s file-touch partition
(`sd11/loadSd11TesterWorkbenchSurface.test.ts`,
`sd11/status/createSd11WorkbenchStatus.test.ts`, `testSupport/makeSurface.ts`)
for the known pre-bump literal `Codex 0.4.94-test` and asserting each instead
carries `Codex <current package.json version>-test`. Ran against the
pre-edit fixtures (re-verified by stashing the fixture edits and re-running):

```
$ node_modules/.bin/tsx src/sd22/buildLabelFixtureFreshness.test.ts
Error: src/sd11/loadSd11TesterWorkbenchSurface.test.ts still carries the
pre-bump build-label fixture "Codex 0.4.94-test"
    at assert (src/testSupport/asserts.ts:16:11)
    at verifiesFixturesCarryCurrentTrancheBuildLabel (src/sd22/buildLabelFixtureFreshness.test.ts:36:5)
Exit code: 1
```

Failed for the intended reason (stale fixture literal, not a setup/compile
error). Note: an earlier draft of this test used a blanket
`/Codex \d+\.\d+\.\d+-test/` regex and false-positived on
`createSd11WorkbenchStatus.test.ts`'s unrelated `'Codex 0.0.0-test'` literal,
which is a deliberate arbitrary-input case for the formatter
(`verifiesLinuxAlphaStatusTruth`), not a "current build" fixture. Narrowed
the check to the specific known-stale literal before treating RED as valid.

## Green-phase evidence

Updated the three partitioned fixture files, replacing the pre-bump literal
`'Codex 0.4.94-test'` (and the bare version variant `'0.4.94-test'` in
`makeSurface.ts`) with `'Codex 0.5.95-test'` / `'0.5.95-test'`.

```
$ node_modules/.bin/tsx src/sd22/buildLabelFixtureFreshness.test.ts
Exit code: 0

$ npm test
...
47/47 test files passed.
```

One sibling regression surfaced and was fixed in the same commit:
`makeSurface.ts` is the documented single-source-of-truth fixture factory for
the SD-11 tester-workbench surface; four consumer test files independently
hard-code the same pre-bump build-label literal in their own assertions
(`sd11/feedback/bug/composeBugReport.test.ts`,
`sd11/feedback/enhancement/composeEnhancementRequest.test.ts`,
`sd11/feedback/evidence/captureFeedbackEvidence.test.ts`,
`sd15/buildSd15OperatorTriageDraft.test.ts`). These aren't in Epic 8's
file-touch partition, but they broke as a direct, mechanical consequence of
this cycle's fixture update (not a pre-existing failure — baseline `npm test`
before this cycle's edits was 46/46 green after `npm install` restored
`node_modules`, which was absent at cycle start). Per the sibling-preservation
rule and `AGENTS.md`'s "fix the source, not the symptom," updated their
hard-coded `Codex 0.4.94-test` literals to `Codex 0.5.95-test` in the same
commit rather than leave them red.

`cargo test --locked` and `cargo clippy --locked --tests -- -D warnings` were
re-run at repo root for full-suite regression safety even though this
criterion doesn't touch Rust source (Epic 8's file-touch set is disjoint from
`src/rules_core/rules_tables/<book>/`):

```
$ cargo test --locked
test result: ok. 136 passed; 0 failed; 0 ignored ... (plus per-integration-test-file suites, all ok)

$ cargo clippy --locked --tests -- -D warnings
Finished `dev` profile [unoptimized + debuginfo] target(s) in 17.96s
(no warnings emitted)
```

`node_modules` was absent at the start of this cycle (every `npm test` file
failed for an environment reason, not a code reason); ran `npm install`
first to restore it, confirming a clean 46/46 baseline before touching any
fixture.

## Files touched

- `apps/desktop/src/sd22/buildLabelFixtureFreshness.test.ts` (new; RED→GREEN test for this criterion)
- `apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts` (fixture re-anchor)
- `apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts` (fixture re-anchor)
- `apps/desktop/src/testSupport/makeSurface.ts` (fixture re-anchor, shared factory)
- `apps/desktop/src/sd11/feedback/bug/composeBugReport.test.ts` (sibling-regression fix)
- `apps/desktop/src/sd11/feedback/enhancement/composeEnhancementRequest.test.ts` (sibling-regression fix)
- `apps/desktop/src/sd11/feedback/evidence/captureFeedbackEvidence.test.ts` (sibling-regression fix)
- `apps/desktop/src/sd15/buildSd15OperatorTriageDraft.test.ts` (sibling-regression fix)

## Cycle metadata

- cycle_id: 2026-07-19T06:15:00Z
- duration: n/a (not recorded at cycle time; `cycle_timing_seconds: 0` in `receipts.md`)
- bundle_criterion: criterion-28
- corpus_input_path: n/a (version metadata, not content-source ingest)
- RuleSetId: n/a
- ingest_pipeline_version: n/a (Epic 8 is a mechanical fixture re-anchor, not an LST-ingest cycle)

## kanban

- card: no card: hermes unavailable from cloud sandbox (per `receipts.md`'s
  matching block, `cycle_id: 2026-07-19T06:15:00Z`).

## What this criterion does NOT cover

`docs/SD-22/release-closure-checklist.md` (criterion 29) is untouched —
that's a separate, not-yet-started criterion. `apps/desktop/src-tauri`'s
`cargo check` continues to fail in this sandbox on missing GTK system libs
(`gdk-3.0` via pkg-config), unrelated to this change — same pre-existing
environment limitation noted in the criterion-27 receipt.
