# Epic 8 — Criterion 29 — `docs/SD-22/release-closure-checklist.md` established

- cycle_id: 2026-07-19T07:00:00Z
- criterion_section: §1.8 Epic 8 — Build Version Numbering (criterion 29)
- row_or_kind: version:closure_checklist
- branch_tip_before: e555f64
- rule_set_used: n/a (process documentation, not content-source ingest)

## Why this criterion, this cycle

Re-checked the Epic 3/4/5 corpus-generation blocker first: `corpus/` still
doesn't exist and no SRD mirror is reachable from this sandbox (per the
`## Open blockers` entry logged 2026-07-19), so re-attempting those epics
this cycle would just re-log the same fabrication-risk wall. Epic 6 remains
transitively blocked (needs ≥1 book ingested). Per Step 1's priority order,
picked the next eligible criterion: Epic 8's criterion 29
(`docs/SD-22/release-closure-checklist.md`), which
`loop-instruction.md`'s file-touch-partition table documents as independent
of Epics 1-6 and which SD-21's Epic 5 criterion 27 already established a
worked precedent for (`docs/SD-21/release-closure-checklist.md` +
`apps/desktop/src/sd21/releaseClosureChecklistDoc.test.ts`). Mirrored both
files, re-anchored to SD-22's `tranche/5` / `0.5.<build>` shape.

## Red-phase evidence

Added `apps/desktop/src/sd22/releaseClosureChecklistDoc.test.ts` (mirrors
SD-21's `sd21/releaseClosureChecklistDoc.test.ts` shape, re-anchored to
`docs/SD-22/release-closure-checklist.md` and the `feat(sd22): bump version
to` commit-message shape). Ran against the pre-cycle tree (`docs/SD-22/`
did not exist yet):

```
$ node_modules/.bin/tsx src/sd22/releaseClosureChecklistDoc.test.ts
Error: expected docs/SD-22/release-closure-checklist.md to exist at /home/user/codex/docs/SD-22/release-closure-checklist.md
    at verifiesChecklistDocCoversAllFourSteps (.../releaseClosureChecklistDoc.test.ts:20:11)
Exit code: 1
```

Failed for the intended reason (the doc did not exist yet), not a
setup/compile error.

## Green-phase evidence

Added `docs/SD-22/release-closure-checklist.md`, the four-step version-bump
process (repo-files-plus-workflow-stamp bump; build-label format check;
`cargo check` to refresh `Cargo.lock`; commit) — content mirrors SD-21's
doc verbatim except: `<tranche>` → `<tranche-base>` naming (matching
`decisions.md §2`'s terminology), the worked example bumped from `0.4.94`
to `0.5.95` (the value Epic 8 criteria 27/28 already landed on this
branch), and the commit-message shape changed from `feat(sd21):` to
`feat(sd22):`.

```
$ node_modules/.bin/tsx src/sd22/releaseClosureChecklistDoc.test.ts
Exit code: 0

$ npm test
...
48/48 test files passed.
```

```
$ cargo test --locked   (repo-root package `codex`)
...all suites green, 0 failures across the run (unaffected; this criterion is docs+JS-only)...

$ cargo clippy --locked --tests -- -D warnings
Finished `dev` profile [unoptimized + debuginfo] target(s) in 15.25s
(no warnings emitted; -D warnings would have failed the build on any)
```

No sibling regressions this cycle — the new test file and new doc are both
net-new, touching no existing file.

## Files touched

- `docs/SD-22/release-closure-checklist.md` (new; the four-step process doc)
- `apps/desktop/src/sd22/releaseClosureChecklistDoc.test.ts` (new; RED→GREEN
  test for this criterion)

## Note: the workflow stamp line is stale, and this cycle does not fix it

`.github/workflows/publish-tester-release.yml`'s "Stamp build version" step
still reads `VERSION="0.4.${GITHUB_RUN_NUMBER}"` — one tranche behind the
`0.5.95` that Epic 8 criteria 27/28 already landed in the three repo version
files. The new checklist doc's Step 1 documents that this line must be
bumped whenever the `<major>.<tranche-base>` prefix changes; it does not
retroactively bump it. `loop-instruction.md`'s Epic 8 file-touch partition
table doesn't list this workflow file among Epic 8's touched files, so
fixing the drift is out of scope for this cycle to avoid unbounded scope
creep. Flagging it here as a candidate self-heal item for Epic 9's
closure-readiness eval, since it's a real, mechanically-verifiable drift
(not a judgment call) once Epic 9 runs.

## What this criterion does NOT cover

Criterion 30 ("per-cycle tests pass at closure": full `cargo test --locked`
zero-regression + `cargo clippy` clean + the 31-criterion acceptance gate)
is a standing verification gate, not a one-shot artifact — it is
continuously re-verified by every cycle's own Step 5 `cargo test` +
`cargo clippy` run (including this one) and finally closed out by Epic 9's
criterion-31 eval. Not separately marked `complete` by this cycle.
