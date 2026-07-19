# SD-22 — Content-Source Ingest (APG + ACG + Bestiary 1) + DM Toolkit — Operator-Driven Loop Instruction (Tranche-5 release)

> ## ⚠️  OPERATING METHOD — REQUIRED FOR THIS BUNDLE  ⚠️
> 
> **This bundle is operated via the `/loop 60m /goal` invocation model — NOT a one-shot task.**
> 
> After exiting plan mode, the coding harness (or operator) is **required** to launch this SD-22 run as:
> 
> ```bash
> /loop 60m /goal ./loop-instruction.md
> ```
> 
> The `/loop` form restarts the cycle on a 60-minute cadence; `/goal` is the load-bearing loop-instruction file whose body *this file* is. The supervisor manages the restart cadence; the loop runs to closure without operator intervention — every criterion `complete` or every criterion has a real blocker in `## Open blockers`. (`/batch` is deferred per operator directive 2026-07-18: it is re-added only when ≥2 book corpora exist on disk and the book lanes are genuinely parallel-eligible — see `decisions.md §5`.)
> 
> **Do not** attempt to execute this bundle's cycles as ad-hoc single-task invocations; the per-cycle procedure (file-touch partition, post-mortem card, progress-doc update, cycle log entry, criterion receipt SHA, `codex-tranche-5` mint) assumes the loop's self-restart cadence and the per-cycle atomicity rules. Ad-hoc execution will silently break the receipt-merge pattern, break the audit-trail comment chain, and break the respawn-guard pattern on `codex-tranche-5`.
> 
> **Pre-launch checklist (operator action only, before the loop's first launch):**
> 
> 1. Confirm `codex-tranche-5` kanban board is set as the SD-22 default (operator-pinned 2026-07-18; reused from the prior 2026-07-16 SD-21 launch that was repurposed).
> 2. Confirm `tranche/5` branch is pushed to origin.
> 3. Corpus source is real PCGen LST data (per `decisions.md §5`, corrected 2026-07-19) — already on disk at `/home/ubuntu/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/{advanced_players_guide,advanced_class_guide,bestiary}/`; a cloud sandbox that only clones `codex` adds `https://github.com/PCGen/pcgen` as a second git source to reach the same tree.
> 4. Run `git status --porcelain | wc -l` on `tranche/5` — must return `0` before loop launch.
> 
> Then launch with `/loop 60m /goal ./loop-instruction.md` and the bundle runs autonomously to closure.

---

title: SD-22 — Content-Source Ingest (APG + ACG + Bestiary 1) + DM Toolkit — Operator-Driven Loop Instruction (Tranche-5 release)
status: approved (operator review 2026-07-15; operator directives 2026-07-17 expanded scope to APG + ACG; operator clarification 2026-07-18: "ACG, APG are the two advanced guides"; branch + board pinned 2026-07-18 to tranche/5 / codex-tranche-5; bundle marked planning-ready; /loop 60m /goal launch form documented with prominent-early ⚠️ OPERATING METHOD callout per operator directive 2026-07-18; /batch deferred + corpus generation in-bundle per operator directives 2026-07-18, decisions.md §5)
date: 2026-07-15
canonical_branch: tranche/5 (operator directive 2026-07-18)
kanban_board: codex-tranche-5 (operator directive 2026-07-18)
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/decisions.md
mirror_of: /home/workspace/SD-22-content-source-ingest-and-dm-toolkit-scope-draft.md
---

This file is the body of the goal the `/loop 60m /goal ./loop-instruction.md` invocation runs. (One launch, run to closure. `/batch` is deferred per `decisions.md §5` — re-added only when ≥2 book corpora exist and the book lanes (Epic 3 APG + Epic 4 ACG + Epic 5 Bestiary 1) are genuinely parallel-eligible; Epic 1 (Identifier Cleanup) is single-stream by the dependency graph regardless.)

It is **self-sufficient**: no interactive prompts, no mid-loop questions to the operator, no shared state with anything other than the on-disk files named here. The loop runs it; the loop restarts every 60 minutes; the loop's self-restart cadence continues until every criterion `complete` or every criterion has a real blocker in `## Open blockers`. (SD-21's `loop-instruction.md` is the worked example; SD-22's `loop-instruction.md` mirrors it with Tranche-5 specifics.)

The progress doc `./progress.md` (created on first cycle by the loop) carries the cycle-log + status matrix (per `governance/spec-domain-lifecycle.md`'s plan A on the SD-status transcription surface).

## Required reading (every cycle)

A cycle is a unit of post-mortem, not a unit of delivery. Every cycle begins by reading these references to keep the loop's discipline honest:

1. **Scope-draft** at `./scope-draft.md` — canonical handoff; tells you *what* SD-22 ships.
2. **Progress doc** at `./progress.md` — loop's working memory; tells you which criteria are open, which are blocked, and the cycle history.
3. **Cycle matrix** in `programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/epic-breakdown.md` — the 31 acceptance criteria mapped to 9 epics; tells you which criterion belongs to which epic.
4. **Decision record** at `programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/decisions.md` — the 5-item decision record (§1 scope, §2 tranche/5 + codex-tranche-5, §3 deferred shape decisions, §4 Epic 9 — Closure Readiness added 2026-07-19, §5 corpus generation in-bundle + /batch deferred added 2026-07-18); tells you *why* the bundle is shaped this way.
5. **Sibling doctrines** at `governance/spec-domain-lifecycle.md`, `governance/identifier-discipline.md`, and `programs/codex/requirements/SD-21-campaign-manager-and-persistence/decisions.md §18` (the build-version scheme `<major>.<tranche-base>.<build>` amendment).

## Concurrency rules (read first, obey always)

These rules are structural. Two concurrent cycles that touch the same file are guaranteed to collide; the loser will be Tech-Priest (or the operator) having to reconcile.

### File-touch partition (the hard rule)

The SD-22 cycle surface is concentrated in these files:

| File | Purpose | Cycles that may touch it |
|---|---|---|
| `src/rules_core/rules_tables/apg/mod.rs` | NEW; APG book-level module (registration, index, `RuleSetId::Apg` variant). Epic 3's first cycle. | One cycle at a time (Epic 3's cycles). |
| `src/rules_core/rules_tables/apg/class_<class>.rs` | NEW; one per APG class. Epic 3's per-class cycles. | One cycle per file. |
| `src/rules_core/rules_tables/apg/spell_list.rs`, `apg/equipment_tables.rs` | NEW shared APG structured-data files. Edited by APG cycles. | One cycle at a time per file. |
| `src/rules_core/rules_tables/acg/mod.rs` | NEW; ACG book-level module. Epic 4's first cycle. | One cycle at a time (Epic 4's cycles). |
| `src/rules_core/rules_tables/acg/class_<class>.rs` | NEW; one per ACG class. Epic 4's per-class cycles. | One cycle per file. |
| `src/rules_core/rules_tables/acg/spell_list.rs`, `acg/equipment_tables.rs` | NEW shared ACG structured-data files. Edited by ACG cycles. | One cycle at a time per file. |
| `src/rules_core/rules_tables/beastiary1/mod.rs` | NEW; Bestiary 1 book-level module. Epic 5's first cycle. | One cycle at a time (Epic 5's cycles). |
| `src/rules_core/rules_tables/beastiary1/monster_<subset>.rs` | NEW; one per monster-block subset. Epic 5's per-monster-block cycles. | One cycle per file. |
| `src/rules_core/encounters.rs` | NEW; DM-toolkit encounter-difficulty computation. Epic 6's criterion 18 cycle. | One cycle at a time. |
| `src/rules_core/party_cr.rs` | NEW; DM-toolkit party-challenge-rating computation. Epic 6's criterion 19 cycle. | One cycle at a time. |
| `src/pcgen_import/lst_parser/class.rs`, `src/pcgen_import/lst_parser/spellcasting_class.rs` | EDIT (rare); a per-class Epic 3/4 cycle widens `MARTIAL_CLASS_NAMES` or `SPELLCASTING_CLASS_NAMES` by exactly one name, ONLY when that class's `.lst` record isn't yet recognized (per `decisions.md §5`, corrected 2026-07-19 — mirrors the SD-17 doc comments' own "owned by later B-slices" design). Not a general-purpose parser rewrite; adding a name plus a small real-corpus test per widening. | One cycle at a time; one class-name addition per cycle. |
| `tests/sd22_<book>_<class_or_subset>_resolves.rs` | Per-class or per-monster-block acceptance tests. | One cycle per file. |
| `tests/sd22_dm_toolkit_deterministic.rs` | DM-toolkit deterministic tests against canonical Paizo examples. Epic 6's criterion 20 cycle. | One cycle at a time. |
| `tests/sd22_dm_toolkit_happy_path_integration.rs` | DM-toolkit happy-path integration test consuming ingested content. Epic 6's criterion 21 cycle. | One cycle at a time. |
| `apps/desktop/package.json` | EDIT (version bump); Epic 8 criterion 27 cycle bumps `"version"` to `"0.5.<current_build>"`. | One cycle at a time. |
| `apps/desktop/src-tauri/tauri.conf.json` | EDIT (version bump); Epic 8 criterion 27 cycle bumps `"version"` to `"0.5.<current_build>"`. | One cycle at a time. |
| `apps/desktop/src-tauri/Cargo.toml` | EDIT (version bump); Epic 8 criterion 27 cycle bumps `version =` to `"0.5.<current_build>"`. `Cargo.lock` updates on next `cargo check`. | One cycle at a time. |
| `apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts` | EDIT (build-label format); Epic 8 criterion 28 cycle sets `BUILD_PREFIX = 'Codex'` and template `${BUILD_PREFIX} ${buildVersion}` (matches `<major>.<tranche>.<build>` triple from the version files). | One cycle at a time. |
| `apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts`, `apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`, `apps/desktop/src/testSupport/makeSurface.ts` | EDIT (test fixtures); Epic 8 criterion 28 cycle updates assertions/fixtures from `codex@0.0.0-test` to `Codex 0.5.<build>` shape. | One cycle at a time per file. |
| `docs/SD-22/release-closure-checklist.md` | NEW; Epic 8 criterion 29 cycle writes the four-step closure-process checklist using the `<major>.<tranche-base>.<build>` triple (per-position increment rules: build per-CI-build, tranche per-tranche-promotion, major per-main-publish). | One cycle at a time. |
| `programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/release-notes.md` | NEW; Epic 7 criterion 25 cycle generates release notes (New content, DM toolkit, Maintenance, Versioning sections). | One cycle. |
| Epic 7 sweep (closure PR, worktree cleanup, branch cleanup) | Epic 7 criterion 23 + 24 cycles run `gh pr create`, `git worktree remove --force`, `git branch -d`. Operates on integration-branch metadata, not on per-file content. | One cycle. |

The chassis and corpus-aware seam files (`pilot_compute.rs`, `pilot_compute_corpus.rs`, `support_state_matrix.rs`) stay untouched by SD-22. SD-22's Epic 1 is defensive cleanup only (Epic 1 doesn't open new directories). Epic 6 (DM Toolkit) reads from `src/rules_core/rules_tables/<book>/` after Epic 3+4+5 land.

Epic 3 and Epic 4 share the file-touch partition (per-book directory each). Epic 3 cycles serialize against Epic 4 cycles only if they touch the same file path; the directory partitioning makes that impossible in practice (`apg/` vs. `acg/`). Epic 5 cycles serialize against Epic 3+4 cycles only if they touch the same path; `beastiary1/` is disjoint. Epic 6 cycles serialize against Epic 3+4+5 cycles only if they touch the same path; `encounters.rs` and `party_cr.rs` are disjoint from the per-book directories.

Epic 5's file-touch set is the new per-monster-block subset files Epic 5 introduces. Epic 5 fires after Epic 3 + Epic 4 land (so the DM toolkit has data to consume in its integration test). Epic 5 is independent of Epics 1-3 in the cycle order.

Epic 8's file-touch set is the three version files plus `createSd11WorkbenchStatus.ts` plus three test-fixture files plus one new docs file. Epic 8 fires before Epic 9's evaluation cycle (Epic 8's version commit must be in Epic 7's closure PR's commit history). Epic 8 is independent of Epics 1-6.

Epic 9's file-touch set is small but pure-read-side: `docs/release/SD-22/artifacts/` (read-only survey), `./progress.md` (read the status matrix; log each self-heal cycle into the same doc), `./risks-and-open-questions.md` (write `## Open judgments deferred to next SD` entries), and `docs/release/SD-22/closure-readiness-report.md` (NEW; Epic 9's criterion-31 cycle writes the report describing the eval, the self-heal cycles run, and the dispatch decision). Epic 9 fires after Epic 8 and before Epic 7. Epic 9's self-heal cycles touch the cycle of their finding (e.g. if Epic 9 finds Epic 6's deterministic tests are green but the artifact `tests/sd22_dm_toolkit_deterministic.test-output.json` is missing, Epic 9's self-heal cycle emits the test-output JSON; if Epic 9 finds Epic 8's `apps/desktop/package.json` has the wrong version, Epic 9's self-heal cycle bumps it). Epic 9 does **not** modify source-code behavior beyond fixing the specific shortfall Epic 9 found.

Epic 7's file-touch set is metadata-only (closure PR, worktree, branch, release notes) and disjoint from every other epic's touched files. Epic 7 fires LAST and serializes against all prior epics on the integration branch's commit history. Epic 7 is gated: its criterion-22 final-scan is blocked at the dispatcher until Epic 9's criterion-31 is `complete`.

### Per-cycle spawn budget (the default)

Default: **1 cycle at a time.** Reason: the file-touch partition collapses any parallel attempt into a serial one for shared `src/rules_core/rules_tables/<book>/` directories (Epic 3 + 4 + 5 each own their directory), Epic 6's `encounters.rs` and `party_cr.rs` modules, and Epic 8's three version files. Two cycles in parallel means two cycles racing on the same fixture file.

To run more than one cycle in parallel you must show that the second cycle touches a disjoint file set. That's possible when one cycle is in Epic 3 (APG directory) and the other is in Epic 4 (ACG directory), since the two directories are disjoint. Epic 3 and Epic 5 are also disjoint (Epic 3 writes `apg/`, Epic 5 writes `beastiary1/`). Epic 6 and Epic 8 are disjoint from Epic 3+4+5 (Epic 6 writes `encounters.rs` + `party_cr.rs`, Epic 8 writes version files). Epic 1 is single-stream (defensive cleanup runs first).

## Per-cycle procedure (the steps, in order)

### Step 1 — Pick a criterion

From the SD-22 progress doc's `## SD-22 cycles` `open` list, pick the smallest unclaimed eligible acceptance criterion. Priority order:

1. **Epic 1 cycles first** (Code-Side Identifier Cleanup). All subsequent work lands on clean identifiers.
2. **Epic 2 cycles next** (Operator Pre-Launch). The board-exists / branch-pushed / clean-state verifications gate the loop's first cycle.
3. **Epic 3 (APG) and Epic 4 (ACG) and Epic 5 (Bestiary 1) cycles thereafter**, in parallel if the operator hosts three loop channels. Each book has its own disjoint directory; concurrent cycles on disjoint books serialize naturally.
4. **Epic 6 (DM Toolkit) cycles after Epic 3+4+5** — at least one book must be ingested before DM-toolkit cycles have data to consume.
5. **Epic 8 (Build Version Numbering) fires before Epic 9** — same shape as SD-21 Epic 5 → SD-21 Epic 4.
6. **Epic 9 (Closure Readiness) fires before Epic 7** — Epic 9's cycle picker evaluates every criterion 1-30 + Epic 8's outputs, runs self-heal cycles until 30/30 are clean, then dispatches Epic 7. Self-heal is open-ended (per operator directive 2026-07-19); judgment-call shortfalls are deferred to `risks-and-open-questions.md` §"Open judgments deferred to next SD" rather than remediated in-bundle.
7. **Epic 7 (Closure Epilogue) fires LAST** — its criterion-22 final-scan is the bundle's closure gate. Epic 7 is dispatched only after Epic 9's criterion-31 is `complete`.

### Step 2 — Pick the criterion's work-unit

- **Epic 1**: one identifier-class per cycle (e.g. one Rust Tauri command-name audit sweep; one TS function/class rename batch; one `data-testid` sweep; one inline doc-comment sweep).
- **Epic 3**: one APG class per cycle (e.g. Alchemist, then Cavalier, then Gunslinger, etc.). Each cycle lands one class table plus its spell/equipment integration.
- **Epic 4**: one ACG class per cycle (e.g. Alchemist, then Arcanist, then Bloodrager, etc.).
- **Epic 5**: one monster-block subset per cycle (operator-pinned ordering; default: alphabetical by name within CR band).
- **Epic 6**: one DM-toolkit function per cycle (e.g. `Encounter::new` first, `party_challenge_rating` second, deterministic tests third, happy-path integration fourth).

### Step 3 — Verify the working tree is on `tranche/5`

```bash
cd /home/ubuntu/workspace/repos/codex
git fetch origin tranche/5
git checkout tranche/5
git pull origin tranche/5
git status --porcelain | wc -l   # expect 0; if non-zero, exit CLAIM-EXISTS
```

### Step 4 — Red phase: write the failing test first

**Operator-pinned 2026-07-19 red-green TDD mandate: Steps 4 and 5 are mandatory. A cycle that ships test code without running RED, or persists GREEN before RED existed, is a Bucket-B / Bucket-C shortfall (Epic 9 evaluates it as a self-heal trigger; the cycle is re-run with the red→green transition preserved in the cycle artifact).**

Read first (RED-phase reading set):
1. `corpus-source-inventory.md` — the row for this cycle's content unit. The row gives `rust_module_path`, `test_fixture_path`, `cycle_artifact_path`, and `RuleSetId`.
2. The corresponding section of `epic-breakdown.md` — the criterion's exact wording and the prior criterion's cycle-receipt pattern.
3. `risks-and-open-questions.md` — self-healable vs. non-self-healable rows for this epic.

Then:

1. Add `tests/sd22_<criterion>.rs` (or the cycle's class/monster/test-fixture-path per `corpus-source-inventory.md`). Mirror the shape of the most recent sibling cycle's test file.
2. Confirm the test fails for the **intended reason** when run against `origin/tranche/5` as the base. A test that fails for an unrelated reason (compile error in the production code under test, missing dependencies, etc.) is a Bucket-B shortfall — fix the test setup, don't carry the cycle forward.
3. **Persist the RED output** in the cycle artifact (`docs/release/SD-22/artifacts/<cycle_artifact_path>`) under the "Red-phase evidence" section. The handoff cannot reference RED that doesn't exist; Epic 9's evaluator reads the artifact to confirm RED→GREEN existed.

```bash
cargo test --locked --test sd22_<criterion> 2>&1 | tail -40
```

### Step 5 — Green phase: implement the smallest change that makes the test pass

For SD-22 cycles, the change is one of:

- **Epic 1 — Identifier Cleanup**: source-code identifier audit + renames. Per the identifier-discipline doctrine. RED is `grep` finding the dirty identifier; GREEN is the rename + tests passing.
- **Epic 3 — APG content-source ingest**: **first, parse the real record from `apg_classes.lst` / `apg_abilities_class.lst` / `apg_equip_*.lst` (per `decisions.md §5`) using the existing `src/pcgen_import/lst_parser/*` functions**, per the `corpus-source-inventory.md` §1.1 row's routing columns (the row's *Content shape* prose is illustrative only — see that file's corrective banner; the `.lst` record is the source of truth); then new file `src/rules_core/rules_tables/apg/<class>.rs` with the transcribed structured data, citing the source `.lst` file + record key in a doc comment (mirroring `rules_tables/crb/class_tables.rs`), or edit `apg/spell_list.rs` / `apg/equipment_tables.rs` per §1.2. Add `RuleSetId::Apg` if not yet added. **RED phase must reference §1.3 cross-book invariants** so the test asserts `Some(...)` for `RuleSetId::Apg` and `None` for the other variants.
- **Epic 4 — ACG content-source ingest**: symmetric to APG with `acg/` directory and `RuleSetId::Acg` — **parse `acg_classes.lst` / `acg_abilities_class.lst` / `acg_equip*.lst` first, per §2.1**. RED phase references §2.3 cross-book invariants.
- **Epic 5 — Bestiary 1 content-source ingest**: **parse `b1_races.lst` / `b1_abilities_race.lst` first, per §3.1**; then new files in `beastiary1/` per monster-block subset. RED phase references §3.1 and §3.2 invariants.
- **Epic 6 — DM Toolkit**: extension to encounter-difficulty and party-CR modules. RED phase references §4.1's five deterministic test cases. Happy-path integration consumes ingested Epic 3+4+5 output.
- **Epic 7 — Closure Epilogue**: GREEN-only; the criterion is "PR is opened, release notes are generated, closure is closed." No cycle fixture; the cycle artifact is the closure PR + the release notes.
- **Epic 8 — Build Version Numbering**: GREEN-only; the version fields are simple mutations with a small test fixture asserting the build-label format. Cycle artifact: version file diff + build-label test output.
- **Epic 9 — Closure Readiness**: GREEN-only; the criterion is the artifact-evidence survey output. Cycle artifact: `closure-readiness-report.md` per `corpus-source-inventory.md` §6 contract.

For all paths, the change must be in the appropriate epic file. The forbidden write scopes are documented in `programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/risks-and-open-questions.md`.

Run:

```bash
cargo test --locked --test sd22_<criterion> 2>&1 | tail -40
cargo test --locked 2>&1 | tail -20
cargo clippy --locked --tests -- -D warnings 2>&1 | tail -20
```

All three must be green. Capture the output. **It is the GREEN evidence.** Persist the GREEN output in the same cycle artifact under the "Green-phase evidence" section, alongside the RED output captured in Step 4.

Refactoring is permitted only after GREEN. A cycle that refactors before GREEN is a Bucket-B shortfall (the cycle artifact must show RED → GREEN in that order; refactor moves are post-GREEN with cargo test --locked + clippy held green throughout).

**For Epic 3/4/5/6 cycles, Step 5 is incomplete until the cycle reads `./ingest.md`.** The `ingest.md` file is the canonical process doctrine for content-source ingest — every per-class (APG/ACG) cycle, every per-monster-block-subset (Bestiary 1) cycle, and every DM-Toolkit (Epic 6) cycle binds to its pipeline (corrected 2026-07-19 to match the proven pipeline: hand-transcribe BAB/save chassis directly from the real `.lst` record, widen the relevant `pcgen_import` parser allowlist by one class name when needed — no corpus-loader abstraction, no default stub-swap). The cycle's production code is implemented per `ingest.md` §2.2, the cycle artifact is formatted per `ingest.md` §2.4, and `artifacts/corpus/operator-supplied/` is a fallback-only slot (`ingest.md` §5) for the rare case the real public PCGen corpus doesn't cover a book — not part of the default pipeline.

### Step 6 — Commit, push directly to `tranche/5`

```bash
git add src/rules_core/rules_tables/<book>/<file>.rs \
        tests/sd22_<criterion>.rs \
        src/rules_core/encounters.rs \
        src/rules_core/party_cr.rs \
        apps/desktop/package.json \
        apps/desktop/src-tauri/tauri.conf.json \
        apps/desktop/src-tauri/Cargo.toml \
        apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts
git -c user.name='Todd Hintzmann' \
    -c user.email='todd@hintzmann.net' \
    commit -m "feat(sd22): <criterion> (<row transition>)"
git push origin tranche/5
```

The commit lands directly on `tranche/5`. Capture the commit SHA — it is the durable receipt.

### Step 7 — Open the PR (NOT APPLICABLE to SD-22)

SD-22 has no PRs. Per `decisions.md §6` (no-branches convention): every cycle commits directly to `tranche/5`. The `tranche/5 → develop` promotion PR is operator-driven only and happens once at SD-22 closure, not per cycle.

### Step 8 — Auto-merge to `tranche/5` (NOT APPLICABLE to SD-22)

SD-22 has no auto-merge. The commit is already on `tranche/5` by construction.

### Step 9 — Cleanup (NOT APPLICABLE to SD-22)

SD-22 has no ephemeral branch to clean up. The next cycle's Step 3 checkout handles any stale working-tree state.

### Step 10a — Write the repo-resident receipt (ALWAYS; durability backbone)

The repo-resident receipt file `docs/release/SD-22/receipts.md` is the **always-write**
surface. Cloud-run cycles may not reach the kanban DB; the receipt file captures the
cycle even when Step 10b fails.

1. Render one receipt block following the schema at the top of `receipts.md`.
2. Append it under that file's `## Cycle log` heading.
3. Commit the receipts update alongside the production change (single commit; do not
   split the receipt append into its own commit).

```bash
# Append block, then commit together with the production change in Step 6.
```

If `receipts.md` is unreachable for any reason, **that is a Bucket-B (post-mortem
failure) shortfall**, not a hard stop. The cycle continues, but writes a `## Open
blockers` entry on `progress.md` describing the receipts-file failure with the cycle
artifact path.

### Step 10b — Mint the kanban card (best-effort; primary post-mortem record)

When `hermes kanban` is reachable, mint the card. When it is not (cloud-run, board
locked, DB unreachable, etc.), record `kanban_card: "no card: <reason>"` in the
receipt block from Step 10a and continue; the receipts file becomes the primary
post-mortem surface for that cycle.

```bash
hermes kanban --board codex-tranche-5 create \
  "SD22 <criterion> (<epic-section>) [cycle <cycle-id>]" \
  --assignee operator \
  --workspace scratch \
  --initial-status done \
  --created-by operator \
  --priority 3 \
  --body "<card body per schema below>"
```

Card body schema:

```
epic: SD-22
criterion_section: <scope doc section reference, e.g. "§1.3 Epic 3 — APG content-source ingest">
row_or_kind: ingest:apg_class | ingest:acg_class | ingest:beastiary1_subset | dm:encounter | dm:party_cr | identifier:rust_tauri | identifier:ts_function_or_class | version:patch_bump | version:build_label_format | version:closure_checklist | closure_readiness:eval | closure_readiness:self_heal | closure_readiness:dispatch
evidence_tier_before: <previous matrix row state>
evidence_tier_after: <new matrix row state after this commit>
merge_receipt_sha: <commit SHA on tranche/5>
cycle_id: <ISO-8601 timestamp>
cargo_test_summary: <test summary string>
clippy_signal: clean | dirty
cycle_timing_seconds: <N>
self_heals_applied: <list, empty if none>
next_required_uplift: <recommendation for next iteration>
corpus_input_path: <path to the real .lst source record, e.g. pathfinder/paizo/roleplaying_game/advanced_players_guide/apg_classes.lst:CLASS:Alchemist (per decisions.md §5)>
rule_set_used: Apg | Acg | Bestiary1
```

### Step 11 — Update the progress doc

Edit `./progress.md` in place:

1. Update the `snapshot_as_of` line in the frontmatter to the current `tranche/5` HEAD short SHA.
2. Update the cycle's row in the `## Status matrix` block.
3. Append a new entry to the cycle log under `## Cycle log`.
4. If the cycle did not produce a landed commit, add an `## Open blockers` entry.

Do NOT rewrite the doc from scratch. Edit in place so the diff is small and auditable.

### Step 12 — Exit the cycle

Print a final 7-line report and exit:

```
cycle: <cycle-id>
criterion touched: <criterion>
row_or_kind: <row_or_kind>
commit: <commit sha on tranche/5, or 'no commit: <reason>'>
card: <hermes kanban card id, or 'no card: <reason>'>
verify: cargo test <X>/<X> green; clippy clean
status: GREEN | FAIL | NO-OP | CLAIM-EXISTS
```

`/loop` restarts the cycle 60 minutes later. The next cycle re-reads the progress doc and picks the next criterion.

## Self-healing posture

The loop self-heals wherever the failure is mechanically resolvable. The operator returns from a multi-day run to a list of problems — not a stopped loop.

### Self-healable conditions (resolve inline, exit GREEN)

(Same shape as SD-21's self-healable table; see `risks-and-open-questions.md` for the canonical list.)

### Non-self-healable conditions (write to `## Open blockers`, exit FAIL)

(Same shape as SD-21's non-self-healable table; see `risks-and-open-questions.md` for the canonical list.)

## Hard stops (refuse, exit FAIL)

The cycle refuses to advance when any of the following is true. In every case the cycle writes the reason to `## Open blockers` in the progress doc and exits with `FAIL`.

- A slice branch has diverged from `tranche/5` in a way that needs a manual rebase.
- The progress doc and the live matrix disagree on a row's `evidence_tier` and the disagreement is not just a stale snapshot.
- `cargo test --tests` regresses on a row other than the one the cycle touched. Sibling-preservation is a hard rule.
- Two live `claude` processes are working on cycles that would both touch `src/rules_core/rules_tables/<book>/` or any per-epic module file.
- **SD-22-specific:** A cycle's `.lst` lookup is genuinely unreachable — neither the local sibling repo (`/home/ubuntu/workspace/repos/pcgen/data/...`) nor a cloned `https://github.com/PCGen/pcgen` mirror resolves in that session's environment, or the specific record isn't present in the resolved tree. Per `decisions.md §5` (corrected 2026-07-19), real LST data already exists for APG/ACG/Bestiary 1 — this hard stop should be rare; if it fires repeatedly, that's a signal the session's environment setup (not the corpus) is the actual problem.

## What "content-source ingest closure" actually means for SD-22

SD-22 closes when every closure gate in `acceptance-and-verification.md` passes AND Epic 7's `codex-tranche-5` board shows every criterion `complete` (criteria 1-30 + Epic 9's criterion-31). Concretely:

1. **Epic 1 closed**: every `sd*_*` identifier in source has been audited + cleaned up.
2. **Epic 2 closed**: board-exists / branch-pushed / clean-state verifications all passed.
3. **Epic 3 closed**: every APG class table ships in `src/rules_core/rules_tables/apg/`; `RuleSetId::Apg` variant ships; cross-book resolution works against CRB.
4. **Epic 4 closed**: same for ACG.
5. **Epic 5 closed**: same for Bestiary 1.
6. **Epic 6 closed**: `Encounter::new` and `party_challenge_rating` ship with deterministic tests covering the canonical Paizo examples; happy-path integration test consumes ingested content.
7. **Epic 8 closed**: build-version scheme ships at `0.5.<current_build>`; build-label format reads "Codex 0.5.<build>"; four-step closure-process checklist is committed at `docs/SD-22/release-closure-checklist.md`.
8. **Epic 9 closed**: criterion-31 has artifact evidence in `docs/release/SD-22/artifacts/` (the closure-readiness-report.md, the artifact-evidence survey output, the self-heal-cycle log, and the open-judgments log) AND the report records a clean 30/30 evaluation AND Epic 7's kanban card has been transitioned from `pending` to `ready` (i.e. Epic 7 has been dispatched).
9. **Epic 7 closed**: closure PR `tranche/5 → develop` is merged; worktrees + stale branches cleaned up; release notes generated; tranche-position version increment lands.

The DM toolkit is **not** locked to character sheet feature completeness. APG/ACG/Bestiary 1 ingest ships class/monster data; downstream character-compute rules are SD-20's lane.

## How the loop will end

The `/loop` form exits when the operator stops it. There is no automatic stopping condition. The loop keeps picking the next-best criterion until every criterion is `done` (closure met) or every criterion has a real blocker in `## Open blockers`.

The operator can stop the loop at any time; a stopped loop leaves the progress doc in the state of the last completed cycle, with all open claims expired, and the operator can resume by relaunching `/loop 60m /goal <this file>`.

## Operating posture (for the operator launching the loop)

1. **One launch command, run to closure.** Launch with `/loop 60m /goal ./loop-instruction.md`. The loop runs to closure — every criterion `done` or every criterion has a real blocker in `## Open blockers` — and then exits. (`/batch` is added only when ≥2 book lanes are corpus-ready and genuinely parallel — operator directive 2026-07-18, `decisions.md §5`.)

2. **Why one launch, not three windows.** The dependency graph (`epic-breakdown.md`) is the sequencing mechanism:
   - **Epic 1 (Identifier Cleanup)** is the only eligible criterion at launch — every subsequent criterion touches source that Epic 1 has cleaned up.
   - **Epic 2 (Operator Pre-Launch)** verifies the launch infrastructure before any cycle runs.
   - **Epics 3 (APG) and 4 (ACG) and 5 (Bestiary 1)** depend only on Epic 1's renames having landed. They have **disjoint book directories** (`rules_tables/apg/`, `rules_tables/acg/`, `rules_tables/beastiary1/`).
   - **Epic 6 (DM Toolkit)** depends on at least one book ingested (Epic 3/4/5 outputs).
   - **Epics 8 (Build Version), 9 (Closure Readiness), and 7 (Closure Epilogue)** depend on Epics 1-6 all complete. Specifically: Epic 9's eval cycle runs after Epic 8's version commit lands; Epic 9 dispatches Epic 7 only when 30/30 criterion-evidence survey is clean; Epic 7 opens the `tranche/5 → develop` PR after Epic 9 dispatches.

3. **What `/batch` would add, and why it's deferred.** Per the SD-13 loop-model excerpt, `/batch` is the form that lets a single shell invocation run multiple streams concurrently against the shared goal file — the three book lanes (Epic 3 APG + Epic 4 ACG + Epic 5 Bestiary 1) as streams inside one invocation, not three separate shells. Per operator directive 2026-07-18 (`decisions.md §5`), SD-22 launches **without** `/batch`: at launch zero book corpora exist and Epic 1/2 are single-stream by the dependency graph. Re-add `/batch` only when ≥2 book corpora exist on disk and the book lanes are genuinely parallel-eligible under the file-touch partition.

4. **Default ceiling: 1 cycle at a time per file.** Per the file-touch partition above.

5. **Watch the progress doc, not the loop output.** If the log shows three cycles in a row with no landed commit, the loop is stuck on a structural problem.

6. **Post-mortem record is the kanban board.** Each cycle mints a card on `codex-tranche-5`.

7. **The 5-hour window applies here too.** A 60-minute cycle × 5 hours = up to 5 landed criteria per 5-hour window per stream.

8. **SD-21 + SD-20 run independently.** Their loops are on their own branches (`tranche/4-1` and `tranche/4` respectively). SD-22 doesn't depend on them; SD-21's Epic 2 (Campaign Manager + Drive) *consumes* SD-22's Epic 6 (DM Toolkit) once both bundles ship.

9. **Force-push discipline on `tranche/5` is conservative.** A mid-cycle correction requires `git reset --soft HEAD~1` + force-push. Only when previous commit is seconds old.

10. **The `tranche/5` branch must exist on origin before the loop's Step 3 fetch succeeds.** Operator creates it once.

11. **Pre-launch setup checklist (operator action, before first launch).**
    - [ ] `codex-tranche-5` kanban board set as the SD-22 default (operator-pinned 2026-07-18; reused from dead-state).
    - [ ] `tranche/5` branch pushed to origin.
    - [ ] Corpus source is real PCGen LST data per `decisions.md §5` (corrected 2026-07-19) — already on disk locally; cloud sessions add `https://github.com/PCGen/pcgen` as a second git source.
    - [ ] Operator's interactive `hermes kanban boards current` is set to `codex-tranche-5` for operator-driven inspection.
    - [ ] `./progress.md` does not yet exist; the loop creates it on first run.

12. **How the operator knows SD-22 is done.** The loop runs to closure; operator reads the progress doc and sees the final state.

13. **Resolving SD-21's `codex-tranche-4-1` cycles.** SD-21's board holds SD-21's cycles; SD-22 doesn't touch them. If the operator wants to inspect SD-21's cycle history, use `hermes kanban --board codex-tranche-4-1 list` directly.

## Cross-reference

- `./scope-draft.md` — canonical handoff; carries the prominent-early `/loop /goal` OPERATING METHOD callout (`/batch` deferred per `decisions.md §5`).
- `./decisions.md` — 5-item decision record (§1 scope, §2 tranche/5 + codex-tranche-5, §3 deferred shape decisions, §4 Epic 9 — Closure Readiness added 2026-07-19, §5 corpus generation in-bundle + /batch deferred added 2026-07-18).
- `./acceptance-and-verification.md` — 16 closure gates.
- `./epic-breakdown.md` — 31 acceptance criteria grouped into 9 epics (Epic 9 — Closure Readiness added 2026-07-19).
- `./risks-and-open-questions.md` — self-healable vs. non-self-healable split, override flags (Flag A through Flag D), open questions (Q1 through Q5).
- `../SD-19/decisions.md` §9 (source-book subdirectories pattern).
- `../SD-20/decisions.md` (parallel sibling bundle; per-character rules-engine).
- `../SD-21/decisions.md` §18 (the operator's 2026-07-17 `<major>.<tranche-base>.<build>` build-version amendment; SD-22 Epic 8 mirrors).
