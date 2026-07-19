---
title: SD-22 — Content-Source Ingest (APG + ACG + Bestiary 1) + DM Toolkit + Closure Readiness — Progress
mirrors: /home/ubuntu/workspace/SD-22-content-source-ingest-and-dm-toolkit-scope-draft.md
created: 2026-07-19
snapshot_as_of: e555f64
---

# SD-22 — Progress

## SD-22 STATUS: LOOP RUNNING (cycle 1)

Loop launched 2026-07-19 per `decisions.md §5` amendments (corpus generation in-bundle,
`/batch` deferred). Running from a remote execution session — `hermes` CLI is not
available in this environment, so kanban card minting (Step 10) is recorded here as a
markdown note instead of a live board card; the operator should backfill cards on
`codex-tranche-5` from this log when next at a terminal with `hermes` available.

---

SD-22's own progress doc. Loop's claim protocol and per-cycle history live here under
`## SD-22 cycles`.

## Status matrix

| ID | Epic | row_or_kind | Description | Status | Commit |
|---|---|---|---|---|---|
| E1.1 | 1 — Identifier Cleanup | identifier:audit | `sd22_\|SD22_\|Sd22\|SD-22-[A-Z][0-9]` grep across `apps/desktop/`, `apps/desktop/src-tauri/`, `src/rules_core/` | **complete** (0 hits; defensive audit found nothing to clean) | n/a (verification-only) |
| E1.2 | 1 — Identifier Cleanup | identifier:regression_check | Per-rename tests pass | **complete (vacuous)** — no renames needed; baseline `cargo test --locked` green (14 tests, 0 failed) before Epic 3/4/5 work began | n/a |
| E2.3 | 2 — Operator Pre-Launch | prelaunch:board | `codex-tranche-5` kanban board set as SD-22 default | **complete** — `hermes kanban boards switch codex-tranche-5` ran locally 2026-07-19; persistent state file `~/.hermes/kanban/current` = `codex-tranche-5`; loop's per-invocation `hermes kanban --board codex-tranche-5` (per loop-instruction Step 10b) resolves to the same board. NB: session env `HERMES_KANBAN_BOARD=codex-tranche-4` was overriding the on-disk default until unset; not persisted in any shell init file. | n/a |
| E2.4 | 2 — Operator Pre-Launch | prelaunch:branch | `tranche/5` pushed to origin | **complete** — `git ls-remote origin tranche/5` = `233c426...` matches local HEAD | 233c426 |
| E2.5 | 2 — Operator Pre-Launch | prelaunch:no_inflight | No other `claude` processes touching `rules_tables/<book>/` | **complete** — `ps -eo pid,etime,stat,cmd \| grep claude` shows only this session's own process | n/a |
| E3.6-9 | 3 — APG ingest | ingest:apg_class | Alchemist (cycle 1 of 8) | **blocked** — see `## Open blockers` (no verifiable source for corpus generation) | none |
| E4.10-13 | 4 — ACG ingest | ingest:acg_class | Alchemist-ACG (cycle 1 of 10) | see cycle log | pending |
| E5.14-17 | 5 — Bestiary 1 ingest | ingest:beastiary1_subset | Subset 01 (CR 1: Goblin/Kobold/Orc/Skeleton/Zombie) | see cycle log | pending |
| E6.18-21 | 6 — DM Toolkit | dm:encounter, dm:party_cr | Not started (requires ≥1 book ingested) | open | — |
| E7.22-26 | 7 — Closure Epilogue | closure:* | Not started (fires last) | open | — |
| E8.27 | 8 — Build Version | version:patch_bump | Version fields set to `0.5.95` (`package.json`, `tauri.conf.json`, `Cargo.toml`) | **complete** — see `artifacts/epic_8/three_version_fields_cycle_receipt.md` | (this cycle's commit, see `## Cycle log`) |
| E8.28 | 8 — Build Version | version:build_label_format | `BUILD_PREFIX = 'Codex'` / `${BUILD_PREFIX} ${buildVersion}` format ships (inherited from SD-21 E5.26); this cycle re-anchored the format's own test fixtures from the pre-bump `Codex 0.4.94-test` literal to the current `Codex 0.5.95-test` | **complete** — see `artifacts/epic_8/build_label_format_cycle_receipt.md` | (this cycle's commit, see `## Cycle log`) |
| E8.29 | 8 — Build Version | version:closure_checklist | `docs/SD-22/release-closure-checklist.md` — four-step version-bump process, mirrors SD-21's E5.27 doc | **complete** — see `artifacts/epic_8/release_closure_checklist_cycle_receipt.md` | (this cycle's commit, see `## Cycle log`) |
| E8.30 | 8 — Build Version | version:* | Per-cycle tests pass at closure — standing verification gate (not a one-shot artifact), re-verified by every cycle's own `cargo test`/`cargo clippy` run; closed out by Epic 9's criterion-31 eval | open (standing gate; re-verified this cycle: `cargo test` all green, clippy clean) | — |
| E9.31 | 9 — Closure Readiness | closure_readiness:* | Not started (fires after Epic 8, before Epic 7) | open | — |

## Open blockers

### E3.6-9 (Epic 3, Alchemist, cycle 3) — corpus generation would require fabricating unverifiable game content

`corpus-source-inventory.md` §1.1 and `decisions.md §5` direct this cycle to
"generate `corpus/apg_alchemist.json` from PF1 OGL/SRD content" by having the
model recall and transcribe the APG Alchemist class table (bomb list,
discoveries, spell progression, etc.) from memory, with no in-repo source file
and no operator-supplied corpus. Before writing anything, this cycle tried to
ground that content against a real source:

- `WebFetch` to `aonprd.com` (Archives of Nethys) → **HTTP 403**
- `WebFetch` to `d20pfsrd.com` → **HTTP 403**

Neither OGL/SRD mirror is reachable from this sandbox, and no corpus or
reference file exists in-repo (`corpus/` doesn't exist; `docs/release/SD-22/artifacts/`
holds only its README). That leaves one path to close this criterion: transcribe
the Alchemist's bombs/discoveries/spell-list content purely from the model's own
training-data recall and commit it to `tranche/5` as if it were verified SRD
data.

This repo already has a documented precedent against exactly that move.
`src/rules_core/rules_tables/crb/class_tables.rs`'s header comment (SD-19)
explicitly scoped CRB's class tables down to BAB/save formulas and left out
named per-level features and spell-per-day cells for this reason, in its own
words: *"hand-transcribing exhaustive per-level feature text without a
verifiable in-repo source would be exactly the fabricated-data risk `AGENTS.md`
rules out."* `AGENTS.md`'s non-negotiable rules (`## Non-Negotiable Rules`,
esp. "No fake completion" and "Fix the source, not the symptom") apply
repo-wide and aren't something a bundle-local planning doc can waive for
itself — per `AGENTS.md`'s own "Role Boundaries": upstream planning artifacts
"define intent and constraints, not permission to improvise beyond the
bounded run."

`decisions.md §5` / `risks-and-open-questions.md` frame "missing corpus file"
as always self-healable by in-cycle generation from memory. That framing is
what's in tension with `AGENTS.md` here — a missing *file* is self-healable;
a missing *verifiable source* for detailed rules-text content is not the same
problem, and self-healing it by fabricating the content is the thing
`AGENTS.md` and the CRB precedent both rule out.

**Not self-healing this inline.** No commit lands this cycle. Recommend one of,
operator's call:
1. Supply a real corpus/reference file (e.g. a licensed text dump or a
   reachable SRD mirror) so the cycle has something verifiable to transcribe
   against, or
2. Narrow Epic 3/4/5's acceptance shape to formula-derivable data only
   (BAB/saves/simple numeric progressions), mirroring the CRB precedent, and
   drop the named-item/named-feature resolution requirements from
   `corpus-source-inventory.md` §1.1/§1.3, or
3. Explicitly re-affirm (outside this bundle's own self-referential docs) that
   memory-recalled OGL content is acceptable here, accepting the fabrication
   risk knowingly.

Logged as a real `## Open blockers` entry per the loop-instruction's hard-stop
clause (unresolvable source ambiguity), rather than force a cycle forward.
E1.1, E1.2, E2.3, E2.4, E2.5 remain **complete** (see cycle log above) — this
blocker is scoped to Epic 3 onward (and, by the same content shape, Epic 4 and
Epic 5, which will hit the identical wall on their first cycles).

## Cycle log

### cycle-2026-07-19T00:00:00Z | Epic 1 + Epic 2 pre-flight | n/a (verification-only) | no card (hermes unavailable; logged here) | open → **complete** (E1.1, E1.2, E2.4, E2.5); E2.3 → **blocked (environment)**

Ran the Epic 1 identifier-audit grep gate scoped to SD-22-specific patterns
(`sd22_|SD22_|Sd22|SD-22-[A-Z][0-9]`) across `apps/desktop/`, `apps/desktop/src-tauri/`,
`src/rules_core/` — zero hits. (The broader `sd[0-9]+_` pattern in the criterion's
verification command also matches pre-existing `sd19_*`/`sd13_*`/`sd16_*` identifiers
from already-shipped, unrelated spec domains — those are out of Epic 1's scope per
`epic-breakdown.md`'s own scope-doctrine note and AGENTS.md's no-scope-expansion rule;
not touched.) Ran baseline `cargo test --locked` — 14 tests passed, 0 failed, confirming
a clean starting tree before Epic 3/4/5 cycles begin. Verified `tranche/5` is pushed to
origin (E2.4) and no other `claude` processes are in-flight (E2.5). E2.3 (kanban board)
requires operator-local `hermes`, unavailable here — recorded as a blocker, non-gating.

### cycle-2026-07-19T03:50:00Z | Epic 2 follow-up: E2.3 + receipts-doctrine amendment | n/a (operator-local + doctrine) | no card (operator-local action; amendment commits land as `1df00d0` and `3c9fa6a`) | E2.3 → **complete**; no other row touched

Operator ran `hermes kanban boards switch codex-tranche-5` from a local terminal with
`hermes` available — the persistent state file `~/.hermes/kanban/current` now reads
`codex-tranche-5`. The loop's per-invocation `hermes kanban --board codex-tranche-5`
calls (loop-instruction Step 10b) will resolve to the same board. One snag: the
session's `HERMES_KANBAN_BOARD=codex-tranche-4` env var was masking the on-disk default
in `hermes kanban boards current` output; the env var is not in any shell init file,
so it is session-scoped only and will not survive into the next launched loop session.
Loop launch will need either `unset HERMES_KANBAN_BOARD` first, or to rely on the
explicit `--board codex-tranche-5` flag (which is what Step 10b does already, so the
loop is correct as written).

Between cycles, the operator landed a doctrine amendment on top of cloud cycle 1:
- `1df00d0 feat(sd22): repo-resident receipts.md + Step 10a/10b split` — adds
  `docs/release/SD-22/receipts.md` (durability backbone for cloud cycles) and splits
  Step 10 into 10a (always-write the repo-resident receipt) and 10b (best-effort
  kanban card mint). Cycle-receipt schema lives at the top of `receipts.md`.
- The amendment post-dates the cloud cycle that wrote `progress.md`, so the cycle
  log here does not retroactively reference Step 10a. Future cycles will.

No Epic 3/4/5 cycles have started yet (correct per dependency graph: Epic 1 vacuous
done, Epic 2 fully done as of this entry). Loop is ready for the first ingest cycle
on next restart.

### cycle-2026-07-19T04:00:00Z | Epic 3, Alchemist (cycle 1 of 8) | ingest:apg_class | no card (blocked, no commit) | open → **blocked**

Attempted Step 4's RED-phase reading for the Alchemist cycle. `corpus-source-inventory.md`
§1.1 and `decisions.md §5` call for generating `corpus/apg_alchemist.json` from
"PF1 OGL/SRD content" in-cycle since no corpus file exists yet. Tried to ground
that against a real source before writing anything: `WebFetch` to `aonprd.com`
and to `d20pfsrd.com` both returned HTTP 403 (unreachable from this sandbox).
With no in-repo corpus/reference file either, the only way to produce the
content this criterion wants (bomb list, discoveries, spell progression, named
class features) is to transcribe it from the model's own training-data recall
and commit it as if it were verified SRD data — which is the exact fabrication
risk `src/rules_core/rules_tables/crb/class_tables.rs`'s own SD-19 doc comment
says it deliberately avoided, citing `AGENTS.md`. Did not write `corpus/`,
`src/rules_core/rules_tables/apg/`, or any `tests/sd22_apg_*` files this cycle.
No commit landed. Full detail and recommended paths forward in `## Open
blockers` above. `cargo test --locked` was not re-run since no production code
changed this cycle (last known-green baseline: 14/14, recorded in the E1+E2
pre-flight cycle above).

### cycle-2026-07-19T05:02:04Z | Epic 8, criterion 27 (three version fields) | version:patch_bump | no card (hermes unavailable; logged here + `receipts.md`) | open → **complete**

Re-checked the Epic 3 Alchemist blocker before picking a criterion: nothing has
changed (`decisions.md §5` still frames "missing corpus file" as always
self-healable by memory-recall from PF1 OGL/SRD content, which is still in
tension with `AGENTS.md`'s "No fake completion" rule and the
`crb/class_tables.rs` precedent). Epic 4 and Epic 5 would hit the identical
wall on their first cycles (same corpus-generation instruction, same absence
of a verifiable source), so did not re-attempt Epic 3/4/5 this cycle. Epic 6
needs ≥1 book ingested (blocked transitively). Per Step 1's priority order,
picked the next eligible, non-transitively-blocked criterion: Epic 8's
criterion 27, which `loop-instruction.md`'s file-touch-partition section
documents as independent of Epics 1-6, and which is a mechanical version bump
with a derivable (not invented) target value.

RED: added `apps/desktop/src/sd22/buildVersionTriple.test.ts` (mirrors SD-21's
`sd21/buildVersionTriple.test.ts`), asserting the version triple starts with
`0.5.`; failed against the pre-bump `0.4.94` tree for the intended reason.
GREEN: bumped `apps/desktop/package.json`, `apps/desktop/src-tauri/tauri.conf.json`,
and `apps/desktop/src-tauri/Cargo.toml` from `0.4.94` to `0.5.95` (major=0 until
first main-publish; tranche=5 for `tranche/5`; build=95, the next monotonic
counter after SD-21's last committed build of 94 on this line per
`decisions.md §2`). Re-ran `npm install` to re-sync `package-lock.json`'s
embedded version (it had already drifted to a stale `0.1.0` pre-cycle).

One sibling regression surfaced and was fixed in the same commit:
`apps/desktop/src/sd21/buildVersionTriple.test.ts` (inherited onto `tranche/5`
via the `aea478c` merge) hard-codes an assertion that the tranche stays at 4
"until promoted" — `tranche/5` *is* that promotion, so the assertion was stale,
not a real regression from this change. Updated its anchor from `0.4.` to
`0.5.` with an explanatory comment rather than leave a known-broken sibling
test on the branch (sibling-preservation + AGENTS.md's "fix the source, not
the symptom").

Verification: `npm test` 46/46 JS test files green (including the new
`sd22/buildVersionTriple.test.ts` and the fixed `sd21/buildVersionTriple.test.ts`).
`cargo test --locked` at repo root (independent Cargo package from
`apps/desktop/src-tauri`) — all suites green, 0 failures. `cargo clippy --locked
--tests -- -D warnings` clean. `cargo check` on `apps/desktop/src-tauri` itself
fails in this sandbox on missing GTK system libs (`gdk-3.0` via pkg-config) —
pre-existing environment limitation unrelated to this change; it got far enough
to resolve and rewrite `Cargo.lock`'s `codex-desktop` entry to `0.5.95` before
failing at the native-linking stage.

Full RED/GREEN evidence, file list, and reasoning:
`artifacts/epic_8/three_version_fields_cycle_receipt.md`. Receipt block appended
to `receipts.md`. Criterion 28 (build-label format) was NOT touched or marked
complete this cycle — `createSd11WorkbenchStatus.ts` already carries the
`BUILD_PREFIX = 'Codex'` / `${BUILD_PREFIX} ${buildVersion}` shape from SD-21's
E5.26, but a future cycle should explicitly verify and close it rather than
this cycle assuming it.

### cycle-2026-07-19T06:15:00Z | Epic 8, criterion 28 (build-label format fixtures) | version:build_label_format | no card (hermes unavailable; logged here + `receipts.md`) | open → **complete**

Re-checked the Epic 3/4/5 corpus-generation blocker first: `corpus/` still
doesn't exist and no SRD mirror is reachable from this sandbox, so nothing
has changed and re-attempting those epics would just re-log the same
blocker. Epic 6 remains transitively blocked (needs ≥1 book ingested). Per
Step 1's priority order, picked Epic 8's remaining open item: criterion 28,
which the prior cycle's receipt explicitly flagged as verified-but-not-closed
(the `Codex ${buildVersion}` format already ships via SD-21 E5.26, but its
own test fixtures still hard-coded the pre-bump `Codex 0.4.94-test` literal).

`node_modules` was missing at cycle start (all 46 JS test files failed for
an environment reason); ran `npm install` to restore it, confirming a clean
46/46 baseline before touching anything.

RED: added `apps/desktop/src/sd22/buildLabelFixtureFreshness.test.ts`,
scanning the three fixture files named in `loop-instruction.md`'s file-touch
partition for the pre-bump literal and asserting each carries
`Codex <current package.json version>-test` instead. Ran against the
pre-edit fixtures (re-verified via `git stash`) — failed for the intended
reason: `"...loadSd11TesterWorkbenchSurface.test.ts still carries the
pre-bump build-label fixture \"Codex 0.4.94-test\""`. (An earlier draft used
a blanket regex that false-positived on an unrelated arbitrary-input fixture,
`'Codex 0.0.0-test'`, used by `createSd11WorkbenchStatus.test.ts`'s
`verifiesLinuxAlphaStatusTruth` case; narrowed to the specific known-stale
literal before trusting RED.)

GREEN: re-anchored `sd11/loadSd11TesterWorkbenchSurface.test.ts`,
`sd11/status/createSd11WorkbenchStatus.test.ts`, and `testSupport/makeSurface.ts`
from `Codex 0.4.94-test` to `Codex 0.5.95-test`. One sibling regression
surfaced from `makeSurface.ts` being the shared fixture factory: four
consumer test files (`sd11/feedback/bug/composeBugReport.test.ts`,
`sd11/feedback/enhancement/composeEnhancementRequest.test.ts`,
`sd11/feedback/evidence/captureFeedbackEvidence.test.ts`,
`sd15/buildSd15OperatorTriageDraft.test.ts`) independently hard-coded the
same stale literal in their own assertions and broke as a direct,
mechanical consequence of this cycle's edit — fixed in the same commit per
sibling-preservation + AGENTS.md's "fix the source, not the symptom," even
though they're outside Epic 8's file-touch partition.

Verification: `npm test` 47/47 green. `cargo test --locked` at repo root —
all suites green, 0 failures (unaffected; this criterion is JS-only).
`cargo clippy --locked --tests -- -D warnings` clean.

Full RED/GREEN evidence, file list, and reasoning:
`artifacts/epic_8/build_label_format_cycle_receipt.md`. Receipt block
appended to `receipts.md`. Next-eligible: Epic 8 criterion 29
(`docs/SD-22/release-closure-checklist.md`) — untouched this cycle.

### cycle-2026-07-19T07:00:00Z | Epic 8, criterion 29 (release closure checklist doc) | version:closure_checklist | no card (hermes unavailable; logged here + `receipts.md`) | open → **complete**

Re-checked the Epic 3/4/5 corpus-generation blocker first: `corpus/` still
doesn't exist and no SRD mirror is reachable from this sandbox — nothing
has changed since the blocker was logged, so re-attempting those epics
would just re-log the same fabrication-risk wall. Epic 6 remains
transitively blocked (needs ≥1 book ingested). Per Step 1's priority
order, picked Epic 8's remaining open item: criterion 29.

`node_modules` was missing at cycle start; ran `npm install` to restore it.

RED: added `apps/desktop/src/sd22/releaseClosureChecklistDoc.test.ts`
(mirrors SD-21's `sd21/releaseClosureChecklistDoc.test.ts`), asserting
`docs/SD-22/release-closure-checklist.md` exists and names all four steps
(three version files, workflow stamp, build-label check, `cargo check`,
the `feat(sd22): bump version to` commit shape, the
`<major>.<tranche-base>.<build>` triple). Failed for the intended reason:
the doc didn't exist yet.

GREEN: added `docs/SD-22/release-closure-checklist.md`, mirroring SD-21's
doc content with `<tranche>` renamed to `<tranche-base>` (matching
`decisions.md §2`'s terminology), the worked example updated to `0.5.95`
(this branch's current version, landed by criteria 27/28), and the
commit-message shape changed to `feat(sd22):`.

Verification: `npm test` 48/48 green. `cargo test --locked` at repo root —
all suites green, 0 failures (unaffected; this criterion is docs+JS-only).
`cargo clippy --locked --tests -- -D warnings` clean.

One note, not fixed this cycle: `.github/workflows/publish-tester-release.yml`'s
stamp line still reads `VERSION="0.4.${GITHUB_RUN_NUMBER}"` — one tranche
behind the `0.5.95` already in the three repo version files. Not in Epic
8's file-touch-partition scope; flagged in the cycle artifact as a
candidate Epic 9 self-heal item (mechanically verifiable drift, not a
judgment call).

Criterion 30 ("per-cycle tests pass at closure") is a standing
verification gate re-verified by every cycle's own `cargo test`/`cargo
clippy` run (including this one), not a one-shot artifact — left `open`
in the status matrix pending Epic 9's criterion-31 eval closing it out.

Full RED/GREEN evidence, file list, and reasoning:
`artifacts/epic_8/release_closure_checklist_cycle_receipt.md`. Receipt
block appended to `receipts.md`. All of Epic 8's file-touch-partition-scoped
criteria (27, 28, 29) are now complete. Next-eligible: Epic 3/4/5 remain
blocked; Epic 6 transitively blocked; Epic 9 (criterion 31) is now
eligible per Step 1's priority order (fires after Epic 8's criterion-30 is
`complete` per `epic-breakdown.md` line 179 — criterion 30 is the standing
gate discussed above, satisfied by this cycle's own green run, so Epic 9
could reasonably start next cycle) but Epic 9 fires "after Epic 8 lands,"
and Epic 8's own criteria 27-29 (the three file-touch-partition-scoped
ones) are now all `complete` — a future cycle should make the explicit
call on whether Epic 9 is now unblocked or whether criterion 30 needs its
own discrete landing first.
