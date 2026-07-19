# Epic 8 — Criterion 27 — Three version fields bumped to `0.5.95`

- cycle_id: 2026-07-19T05:02:04Z
- criterion_section: §1.8 Epic 8 — Build Version Numbering (criterion 27)
- row_or_kind: version:patch_bump
- branch_tip_before: 05a9ced
- rule_set_used: n/a (version metadata, not content-source ingest)

## Why this criterion, this cycle

Epic 3's Alchemist cycle (`## Open blockers` in `progress.md`) is a real,
still-open blocker: generating `corpus/apg_alchemist.json` requires either a
reachable SRD mirror (both `aonprd.com` and `d20pfsrd.com` 403 from this
sandbox) or transcribing named class-feature content from the model's own
training-data recall, which `AGENTS.md`'s "No fake completion" rule and the
`crb/class_tables.rs` precedent both rule out. Epic 4 (ACG) and Epic 5
(Bestiary 1) hit the identical wall on their first cycles for the same
reason — same "generate from PF1 OGL/SRD content in-cycle" instruction, same
absence of a verifiable in-repo or reachable source. Nothing has changed
since that blocker was logged (`decisions.md §5` still frames "missing
corpus file" as always self-healable by memory-recall, which is the exact
tension `AGENTS.md`'s role-boundaries section calls out — a bundle-local
planning doc doesn't get to waive a repo-root conduct rule for itself).
Re-attempting Epic 3/4/5 this cycle would just re-log the same blocker, so
per Step 1's priority order this cycle picked the next eligible, *not*
transitively blocked criterion: Epic 8 criterion 27. Epic 6 needs ≥1 book
ingested (blocked transitively); Epic 8 is explicitly documented as
independent of Epics 1-6 in `loop-instruction.md`'s file-touch-partition
section, and criterion 27 is a mechanical, fully-specified version bump with
no fabrication risk — the target value is derivable, not invented: SD-21's
last committed build on this line was `0.4.94` (commit `6ea6bfd`,
`apps/desktop/src/sd21/buildVersionTriple.test.ts`), so the next monotonic
build per `decisions.md §2` ("build is the next monotonic counter value
after the last committed build on `tranche/5`") is `95`, and tranche moves
from `4` to `5` per the same section. `0.5.95` follows mechanically from
those two facts, not from operator judgment.

## Red-phase evidence

Added `apps/desktop/src/sd22/buildVersionTriple.test.ts` (mirrors SD-21's
`sd21/buildVersionTriple.test.ts` shape, re-anchored to tranche 5). Ran
against the pre-bump tree (`package.json`/`tauri.conf.json`/`Cargo.toml`
still at `0.4.94`):

```
$ node_modules/.bin/tsx src/sd22/buildVersionTriple.test.ts
Error: version "0.4.94" must move to major=0, tranche=5 on tranche/5
    at assert (src/testSupport/asserts.ts:16:11)
    at verifiesAllThreeVersionFilesAgreeAndFollowTripleShape (src/sd22/buildVersionTriple.test.ts:44:3)
Exit code: 1
```

Failed for the intended reason (version triple hadn't moved to the `0.5.`
tranche-5 anchor yet), not a setup/compile error.

## Green-phase evidence

Bumped `"version"` in `apps/desktop/package.json` and
`apps/desktop/src-tauri/tauri.conf.json`, and `version =` in
`apps/desktop/src-tauri/Cargo.toml`, from `0.4.94` to `0.5.95`. Re-ran
`npm install` so `package-lock.json`'s embedded version field stays in sync
(it had already drifted to `0.1.0` pre-cycle; now matches `package.json` at
`0.5.95`).

```
$ node_modules/.bin/tsx src/sd22/buildVersionTriple.test.ts
Exit code: 0

$ npm test
...
46/46 test files passed.
```

One sibling regression surfaced and was fixed in the same commit: SD-21's
own `apps/desktop/src/sd21/buildVersionTriple.test.ts` hard-codes
`pkg.startsWith('0.4.')` ("tranche stays 4 ... until promoted"). `tranche/5`
*is* that promotion, so the assertion (inherited onto this branch via the
`aea478c` merge) was stale, not a real regression from this change. Updated
its anchor + comment to `0.5.` with a note explaining the promotion — the
sibling-preservation rule requires not leaving a known-broken test on the
branch, and `AGENTS.md`'s "fix the source, not the symptom" rules out
leaving it red for a later cycle to clean up. This file is outside Epic 8's
file-touch partition in `loop-instruction.md`, but the partition table
predates this cross-branch-inheritance case; the fix is a one-line anchor
change plus comment, not a rewrite.

`cargo check` on `apps/desktop/src-tauri` fails in this sandbox
(`gdk-3.0` / GTK system libs not installed — pre-existing environment
limitation, unrelated to this change) but got far enough to resolve and
rewrite `Cargo.lock`'s `codex-desktop` package entry to `0.5.95` before
failing at the native-linking stage.

```
$ cargo test --locked   (repo-root package `codex`, independent of apps/desktop/src-tauri)
test result: ok. 136 passed; 0 failed; 0 ignored ... (plus per-integration-test-file suites, all ok)
...all suites green, 0 failures across the run...

$ cargo clippy --locked --tests -- -D warnings
Finished `dev` profile [unoptimized + debuginfo] target(s) in 15.13s
(no warnings emitted; -D warnings would have failed the build on any)
```

## Files touched

- `apps/desktop/package.json` (version bump)
- `apps/desktop/package-lock.json` (re-synced by `npm install`)
- `apps/desktop/src-tauri/tauri.conf.json` (version bump)
- `apps/desktop/src-tauri/Cargo.toml` (version bump)
- `apps/desktop/src-tauri/Cargo.lock` (auto-updated by `cargo check`'s dependency resolution)
- `apps/desktop/src/sd22/buildVersionTriple.test.ts` (new; RED→GREEN test for this criterion)
- `apps/desktop/src/sd21/buildVersionTriple.test.ts` (sibling-regression fix; anchor `0.4.` → `0.5.`)

## What this criterion does NOT cover

Criterion 28 (build-label format) is not touched this cycle beyond
verification: `createSd11WorkbenchStatus.ts` already has `BUILD_PREFIX =
'Codex'` and the `${BUILD_PREFIX} ${buildVersion}` template from SD-21's
E5.26 (commit `5980037`) — that criterion's shape already ships on this
branch. A future cycle should confirm criterion 28's specific test-fixture
assertions (if any remain) and mark it `complete` explicitly rather than
this cycle silently assuming it.
