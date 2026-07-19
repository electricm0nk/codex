# SD-21 — Campaign Manager + Drive + Multiclass Support + Identifier Cleanup + Update UI Bug + Build Versioning + Closure Epilogue — Operator-Driven Loop Instruction (Tranche-4-1 dash release)

> ## ⚠️  OPERATING METHOD — REQUIRED FOR THIS BUNDLE  ⚠️
> 
> **This bundle is operated via the `/loop 60m /batch /goal` invocation model — NOT a one-shot task.**
> 
> After exiting plan mode, the coding harness (or operator) is **required** to launch this SD-21 run as:
> 
> ```bash
> /loop 60m /batch /goal ./loop-instruction.md
> ```
> 
> The `/loop` form restarts the cycle on a 60-minute cadence; `/batch` enables concurrent streams for independent epic lanes (Epic 2 + Epic 3 in this SD); `/goal` is the load-bearing loop-instruction file whose body *this file* is. The supervisor manages the restart cadence; the loop runs to closure without operator intervention — every criterion `complete` or every criterion has a real blocker in `## Open blockers`.
> 
> **Do not** attempt to execute this bundle's cycles as ad-hoc single-task invocations; the per-cycle procedure (file-touch partition, post-mortem card, progress-doc update, cycle log entry, criterion receipt SHA, `codex-tranche-4-1` mint) assumes the loop's self-restart cadence and the per-cycle atomicity rules. Ad-hoc execution will silently break the receipt-merge pattern, break the audit-trail comment chain, and break the respawn-guard pattern on `codex-tranche-4-1`.
> 
> **Pre-launch checklist (operator action only, before the loop's first launch):**
> 
> 1. Confirm `codex-tranche-4-1` kanban board exists (operator creates after SD-20 closes).
> 2. Confirm `tranche/4-1` branch is pushed to origin.
> 3. Confirm Google OAuth credentials configured in `~/.hermes/profiles/god-emporer/.env` (Epic 2 / Drive adapter blocker if missing).
> 4. Run `git status --porcelain | wc -l` on `tranche/4-1` — must return `0` before loop launch.
> 
> Then launch with `/loop 60m /batch /goal ./loop-instruction.md` and the bundle runs autonomously to closure.

---
title: SD-21 — Campaign Manager + Drive + Multiclass Support + Identifier Cleanup + Update UI Bug + Build Versioning + Closure Epilogue — Operator-Driven Loop Instruction (Tranche-4-1 dash release)
status: approved (operator review 2026-07-16; changes noted: launches on tranche/4-1 branch, kanban board codex-tranche-4-1 (operator-created), /loop 60m /batch /goal launch form, pre-launch checklist added, Q1–Q5 PINNED in risks-and-open-questions.md; bundle marked approved with operator directives 2026-07-16; branch/board updated to tranche/4-1 / codex-tranche-4-1 per operator directive 2026-07-17)
date: 2026-07-15
canonical_branch: tranche/4-1 (operator directive 2026-07-17; slash-form dash release following SD-20's tranche/4; replaces the 2026-07-16 tranche/5 directive)
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-21-campaign-manager-and-persistence/decisions.md
mirror_of: /home/workspace/SD-21-campaign-manager-and-persistence-scope-draft.md
kanban_board: codex-tranche-4-1 (operator directive 2026-07-17; replaces the 2026-07-16 codex-tranche-5 directive; new board to separate SD-21 cycles from the chassis-lane codex-tranche-3 and the per-character-rules-engine-lane codex-tranche-4; operator creates the board after SD-20 completes; the loop's Step 10 mint uses `--board codex-tranche-4-1` explicitly so it works regardless of operator's default-board setting)
---

This file is the body of the goal the `/loop 60m /batch /goal ./loop-instruction.md` invocation runs. (One launch, run to closure. `/batch` enables concurrent streams for Epic 2 alone (APG/ACG ingest was moved to SD-22 per operator directive 2026-07-17), starting after Epic 2's capacity allows it; Epic 1 is single-stream by the dependency graph.)
It is **self-sufficient**: no interactive prompts, no mid-loop questions to the
operator, no shared state with anything other than the on-disk files
named here. The loop runs it; the loop restarts every 60 minutes; the loop
dies when the operator stops it (or, for `/batch`, when the supervisor's
streams all reach closure / block).

## Pre-launch verification findings & operator directives (2026-07-18)

Before this run's first launch, the operator had three read-only Explore agents verify
every bug report baked into this bundle against the live `tranche/4-1` tree (HEAD
`2715580`, post SD-20 + PRs #319/#320/#321). Several reports were written 2026-07-15
against an older tree and are now stale in their specifics even where the underlying
defect is still real. This section is the load-bearing correction; where it conflicts
with prose elsewhere in this file or in the scope-draft/epic-breakdown, **this section
wins**. Cite it, don't re-derive.

**Epic 1 (Code-Side Identifier Cleanup) — confirmed fully live, scope slightly wider.**
All named identifier leaks exist (~351 `Sd16`/`SD16` refs, ~19 `sd16-` data-testids, 41
`SD-16` doc comments, the full `SD16_UI_*` constant family). Correction: there are **7**
`sd*_*.rs` Tauri files, not 5 — criterion 1's rename sweep also covers
`apps/desktop/src-tauri/src/sd13_support_state_matrix.rs` and
`apps/desktop/src-tauri/src/sd19_corpus.rs` in addition to the 5 originally named.

**Epic 2 (Campaign Manager + Drive) — confirmed live, must build on landed work, OAuth
descoped.** PR #320 already landed a real local-disk campaign writer:
`apps/desktop/src-tauri/src/campaign_drive.rs` (`write_campaign_drive_artifacts`
command, registered in `main.rs`), `apps/desktop/src/boundary/writeCampaignDriveArtifacts.ts`
(note: this is the actual boundary filename — NOT `campaignDrive.ts` as this file's body
prose says elsewhere), and campaign UI under `apps/desktop/src/campaign/`
(`campaignModel.ts`, `CampaignManagerScreen.tsx`, etc.) that calls it. Epic 2's cycles
extend/reconcile with this landed surface — do not create a parallel-named duplicate.
Still genuinely absent and still Epic 2's job to add: `CampaignSnapshot`,
`CampaignBackend` trait, `src/rules_core/campaign.rs`, `src/rules_core/persistence/`.
**Operator directive: descope Google OAuth / Drive v3 API from SD-21.** The
`~/.hermes/profiles/god-emporer/.env` has no `GOOGLE_OAUTH_CLIENT_ID` /
`_CLIENT_SECRET` / `_REDIRECT_URI` configured, and rather than block Epic 2 on Google
Cloud Console setup, the `CampaignBackend`'s first (and for this bundle, only)
implementation targets a **local folder** the user points at a Google-Drive-for-Desktop-synced
directory — building on the already-landed local writer above. Criterion 7's "Drive
adapter" becomes a local-filesystem `CampaignBackend` impl; criterion 8's Tauri commands
(`drive_list_campaigns`, `drive_load_campaign`, `drive_save_campaign`, `drive_delete_campaign`)
operate on that local folder; `drive_authorize` and `drive_pick_folder`'s OAuth-specific
parts are OUT of scope for SD-21 (route to a future bundle, e.g. SD-22, if the operator
wants real Drive-API sync later). Gate 3 ("Drive OAuth flow operational") in
`acceptance-and-verification.md` is descoped accordingly. Conflict-detection (nonce),
the markdown file layout, and the Obsidian round-trip criteria (9, 10) are unaffected —
they still apply, just against the local backend instead of a Drive-API backend.

## Post-parallel-workflow closure scan & Epic 6b (2026-07-19)

A parallel Workflow run (`wf_3ca4a829-343`) landed real work across Epic 1/2/3/5/6/7 —
22 commits, all suites green. A subsequent operator-directed fresh closure-gate scan
against `acceptance-and-verification.md`'s 13 gates found:

- **Gate 8 (Epic 1) had 3 small leftover leaks** the parallel lanes missed because no
  lane's assigned scope covered the specific files (`character_hub.rs`'s
  `sd19_demo_spells_selected()` fn — Epic 2's lane was read-only there;
  `browserHandoff.ts`'s stale "SD-16 F4"/"AV-PAY-5" doc-comment; `update/transaction.rs`'s
  "SD-16-E7-F3b" header + two `t_<hex>` tokens — Epic 3's lane touched this file for
  functional work, not doc-comment cleanup). Fixed in commit `25e5050`; gate 8 now
  passes cleanly (verified via the full literal grep, not the relaxed mid-run check).
- **Gate 4 (markdown file format) as originally worded doesn't match what Epic 2 built**,
  and the operator chose to amend the gate rather than rework the already-shipped
  layout (commit `25e5050`'s sibling doc edit, `acceptance-and-verification.md` gate 4).
  Epic 2's cycles built on the already-landed `campaign_drive.rs` layout
  (`.config/<name>.json` for structured fields + per-asset `.md` files in
  `resources/adventure-log/maps/wiki/`) rather than the original per-field
  `campaign.md`/`party.md`/`members/<id>.md` YAML-frontmatter spec — reworking it now
  would mean discarding a real, tested, already-shipped writer for a layout no
  consumer needs. Gates 2/5/6's text was also updated to match (`CampaignStore` not
  `CampaignBackend` trait; actual test names, not the originally-named integration
  test files).
- **Gate 11 (Epic 6, Wizard reaches `Status: Computed`) does NOT pass, and the operator
  chose to scope real follow-on work rather than retarget the gate.** Epic 6's own
  landed acceptance test (`tests/sd21_wizard_chassis_computes.rs`) proves Wizard can
  never reach `Computed` as originally scoped — two permanent spell-engine diagnostics
  block it regardless of chassis completeness, plus two more Fighter-only gates
  (`compute_combat_baseline`, `compute_selected_skill_modifiers`) were found undiscovered
  by the original Epic 6 file-touch scope. **See `epic-breakdown.md`'s new "Epic 6b —
  Wizard full-completion" section for the three scoped, landable criteria (`E6b.1`
  combat-baseline/skill-modifier dispatch extension, `E6b.2` prepared spellbook,
  `E6b.3` Arcane School powers + opposed-school cost) and the SD-21 progress doc's
  matching `E6b.1`-`E6b.3` rows.** SD-21 does not close until Epic 6b's reproducer
  (Human Wizard 3, Evocation specialization, populated spellbook, daily prep selection
  → real `Status: Computed`, zero claim-blocking diagnostics) passes for real.
- **UPDATE (2026-07-19, later same day): Epic 6b landed and gate 11 is now genuinely
  met — verified independently, not just self-reported.** All three E6b criteria
  landed (`6ed19bd`, `de554ab`, `1c7ad89`); the capstone reproducer test
  (`tests/sd21_epic6b_full_completion_reproducer.rs`) was run directly and passes:
  Human Wizard 3 + Evocation + populated spellbook reaches `Status::Computed` with
  zero claim-blocking diagnostics. `acceptance-and-verification.md`'s gate 11 is
  updated accordingly. **Every closure gate now passes except gates 7/13 (the
  promotion PR itself) — Epic 4 is now genuinely ready to run in full-closure mode**,
  not just scan-only mode. Its own criterion 19 final-scan should find the SD-21
  progress matrix clean (every criterion `complete`, Epic 6b's rows included) and
  proceed to open the `tranche/4-1 → develop` promotion PR for real. (Note in passing:
  running Epic 6b's own local verification twice caught a real integration-level
  regression each time in `character_hub.rs`'s
  `claim_blocking_diagnostic_ids_match_the_catalogued_support_shape_per_class` test —
  fixed both times by an independent full-suite run at the merged HEAD, not by any
  individual lane's own tests. Epic 4's final scan should run the full suite fresh
  itself, not just trust the progress doc's `complete` markers.)

## Epic 2 engine-shape addendum & CampaignStore decision (2026-07-18, cycle 2 follow-up)

A deeper read-only pass over `campaign_drive.rs`, `campaignModel.ts`, and the existing
`src/saved_character/` persistence precedent (done between cycles 1 and 2 of this run)
refines the Epic 2 findings above. **This addendum overrides the `CampaignBackend`
trait language used elsewhere in this file for criterion E2.6 — read this section as
the authoritative shape for Epic 2's cycles.**

- **Operator directive: build a concrete `CampaignStore` struct, NOT a
  `CampaignBackend` trait.** No `*Backend` trait pattern exists anywhere in this
  codebase today — `SavedCharacterStore` (`src/saved_character/local_store.rs:25`) is a
  concrete zero-field struct with associated fns (`save`/`load`/`list_all`), not a
  trait object. Since OAuth/Drive-API is already descoped to local-disk-only (see
  above), there's no near-term second backend to justify trait-object indirection.
  Wherever this file's prose says `CampaignBackend` trait, read it as `CampaignStore`
  concrete struct instead. If a genuine second backend need arises later (e.g. a real
  Drive-API backend in a future bundle), *that* future cycle can introduce the trait
  retroactively — don't build the abstraction speculatively now.
- **Module location: `src/campaign/` (sibling to `src/saved_character/`), NOT
  `src/rules_core/`.** Campaigns aren't rules-computation, so they don't belong in the
  rules-core module tree the way the loop-instruction's file-touch partition table
  currently implies (`src/rules_core/campaign.rs`, `src/rules_core/persistence/`). The
  partition table's rows for those two paths should be read as pointing at
  `src/campaign/mod.rs` (types) and `src/campaign/local_store.rs` (the store impl)
  instead.
- **The load-bearing contract to preserve verbatim**:
  `WriteCampaignDriveArtifactsRequest { drive_folder_path, campaign_name,
  campaign_config_json: String (opaque JSON, unparsed today), assets: CampaignAssetsDto }`
  and the `write_campaign_drive_artifacts` command signature — this is what PR #320
  shipped and what `writeCampaignDriveArtifacts.ts` calls today. New types slot
  *underneath* this; `write_campaign_drive_artifacts_impl` becomes a thin adapter that
  deserializes `campaign_config_json` into the new typed `CampaignSnapshot` and
  delegates to `CampaignStore`, still returning `WriteCampaignDriveArtifactsResponse`
  unchanged.
- **`CampaignSnapshot` field shape**: mirror `campaignModel.ts`'s `Campaign` +
  `CampaignAssets` types 1:1 — `id, name, ruleSetId, ruleSetLabel, description,
  members: {email, invited}[], partyCharacterIds: Vec<String>, createdAt, updatedAt`,
  plus four asset lists (`resources/adventureLog/maps/wiki`) of `{title, body}`
  (dropping only the UI-local `id`/`updatedAt` bookkeeping on individual assets). Add a
  `schema_version: u16` field from day one, per the `SavedCharacterStore` precedent
  (`CURRENT_SAVED_CHARACTER_SCHEMA_VERSION`) — `campaign_config_json` has no version
  today, but the new typed struct should start versioned.
- **Party references reuse the existing character-hub id space** —
  `partyCharacterIds` are `SavedCharacterSummary`/`CharacterSummaryDto.character_id`
  values already used by `list_saved_characters`. Do not invent a new id type.
- **Persistence conventions to mirror from `SavedCharacterStore`**: `list_all`
  tolerant of a missing root directory (empty listing, not an error), per-entry read
  failures isolated into an `unreadable_entries` list rather than failing the whole
  listing, a flat `CampaignStoreError { message: String }` type (upgrade from today's
  bare `Result<_, String>` in `campaign_drive.rs`). JSON stays the wire format (matches
  what's already flowing from the frontend) — do not adopt the character-store's
  line-based fixture grammar.
- **Structural gap this closes**: `campaign_drive.rs` currently has zero dependency on
  the `codex` engine crate (pure `std::fs`/`serde` Tauri-app-local logic). The
  precedent to grow into is `character_hub.rs` (Tauri layer) wrapping the headless
  `codex::saved_character` crate — Epic 2 grows the same seam for campaigns.
- **Google Drive config** (`apps/desktop/src/settings/googleDrive.ts`) is a 48-line
  pure placeholder (two-field localStorage CRUD + an `isGoogleDriveConfigured()` gate,
  no OAuth/token code) — nothing here needs touching beyond the two-field shape the
  local-folder-picker settings screen already has.

**Epic 3 (Update UI bug remediation) — the bug is real, the prescribed mechanics are
stale.** Confirmed live: `deps.releaseNotes` is deliberately never assigned
(`controllerAdapter.ts:236-240`) so release notes never render, and `computeDecision`'s
success path deliberately short-circuits to `'unknown'`
(`controllerAdapter.ts:157-162`) so the eligibility card always reads "Unknown" — both
are real user-facing defects, just implemented as honest documented deferrals rather
than accidental breakage. Corrections to the fix mechanics: `apps/desktop/src/sd16/update/fetch.ts`
**already exists** (channel-index/manifest fetch) — criterion 12 extends it with a
release-notes-body fetch path, it does not create the file. `is_install_eligible` and
`perform_install` **already exist** as registered, not-wired Tauri command stubs at
`apps/desktop/src-tauri/src/update/transaction.rs:659` and `:680` (registered in
`main.rs:116-117`) — criterion 13 implements those stubs in place. **The
`install_eligibility_probe.rs` row in the file-touch partition below is void; there is
no such file, and none should be created** — the partition now points at
`apps/desktop/src-tauri/src/update/transaction.rs` instead. `loadSd16MountTimeState` is
already wired to a real Tauri command (`verify_relaunch_artifact`), just not yet to the
eligibility probe specifically.

**Epic 5 (Build Version Numbering) — version already moved, and the CI publish stamp
will clobber the bump unless updated too.** The three version files now read `0.1.0`
(changed by PR #320), not the `0.0.X` this file's prose assumes going in — same
mechanic, different starting value. Build-counter anchor: the latest cut release is
`alpha v0.0.93` (tagged from current HEAD `2715580`), so criterion 25's first concrete
value is **`0.4.94`**, not the `0.4.93` example elsewhere in this file. **New finding:**
`.github/workflows/publish-tester-release.yml`'s "Stamp build version" step (line 62)
overwrites `package.json` and `tauri.conf.json`'s version with
`VERSION="0.0.${GITHUB_RUN_NUMBER}"` at publish time — if criterion 25 only touches the
three repo files, every published/tester build still displays the old `0.0.<run>`
scheme, silently undoing the bump. **Operator directive: extend Epic 5's file-touch
partition to include this workflow file.** Criterion 25's cycle also updates that stamp
line to the new triple shape (e.g. `VERSION="0.4.${GITHUB_RUN_NUMBER}"`) —
`GITHUB_RUN_NUMBER` is already the monotonic, never-resets counter the new scheme wants
for the `<build>` position, so this is a one-line format change, not new plumbing.

**Epic 6 (Single-class coverage completion) — the core bug is fully live; two
prescribed details are wrong.** Confirmed: SD-20 never touched `pilot_compute.rs` (its
last commit affecting that file is Tranche-3's PR #318). `compute_pilot_base_chassis`
(`pilot_compute.rs:4568`) still gets BAB/saves solely from `compute_fighter_chassis`
(`:4575-4576`, fn at `:6364` — the only `compute_*_chassis` function that exists in the
codebase); any non-Fighter single class gets BAB 0 plus a claim-blocking
`class_chassis.unsupported` diagnostic, and `PilotReceipt.chassis` clones that output
verbatim (`contract.rs:352`). Criterion 25 is unchanged and fully live. Corrections to
criterion 26: **`supported_wizard_level` is already capped at 20, not 11** —
`MAX_SUPPORTED_WIZARD_LEVEL = 20` at `pilot_compute.rs:2751` (only the docstring at
`:13956-13966` is stale and still says 11), and that gate only covers a spell-baseline
*explanation*, not chassis BAB/saves. **`compute_wizard_chassis` does not exist yet** —
criterion 26 is "create this function," not "extend an existing partial
implementation from level-cap 11." Reuse note: per-class `BabProgression` and
`GoodSaves` data for all 11 classes already exist in
`src/rules_core/rules_tables/crb/class_tables.rs:73-83` (already consumed by
`src/rules_core/level_up/*.rs`) — the new per-class dispatch functions should consume
that table rather than re-deriving progressions.

**Epic 7 (Multiclass stacking) — fully live, one canonical-source citation is wrong.**
Confirmed: no multiclass logic exists anywhere in `src/rules_core/` — every
`supported_*_level` gate matches only single-element `class_levels`, SD-20's
`compute_level_up_grants` explicitly falls through to an empty default for multiclass
inputs with a documented deferral note (`level_up.rs:172-175,187-222`), and there is no
fractional-save-progression implementation anywhere. Correction: criterion 29 as written
says the canonical save formula lives at "`decideEligibility.class_save_bonus`" inside
`pilot_compute.rs` — **no such function exists in the Rust engine**; `decideEligibility`
is the TypeScript update-UI eligibility function (`apps/desktop/src/sd16/update/eligibility.ts`),
an unrelated file. The real canonical per-class save data for the fractional-progression
formula is `class_tables.rs`'s `GoodSaves`, referenced above.

## What this loop does

Ground SD-21 — campaign manager + Drive persistence + APG + ACG — toward
`CampaignManager Closure` for every acceptance criterion in
`./scope-draft.md`.
Working in bounded cycles against the integration branch `tranche/4-1` (per
operator directive 2026-07-17; SD-21's branch is `tranche/4-1`, NOT `tranche/3` or
`tranche/4`; slash-form dash release per the operator's call). Each cycle lands one
acceptance criterion.

The progress doc `./progress.md`
maintains a `## Status matrix` block at the top of the file (one row per
planned loop across the 32 acceptance criteria) so the operator can read the
cumulative state in seconds rather than scrolling through `## Cycle log`.
The matrix exists from the loop's first cycle (skeleton initialized from
the scope-draft's §1.1–§1.8 acceptance criteria) and is updated on every
subsequent cycle. Vocabulary: `pending` (open, unclaimed), `running`
(in-flight stream at the supervisor), `complete` (cycle landed with
green tests), `blocked` (real blocker in `## Open blockers`). SD-21 runs in parallel with SD-20 (per-character
tabletop-readiness); the two bundles share `tranche/4-1` (if SD-20 also moved
to `tranche/4-1`) or stay on separate branches (per operator's separate
launch-branch decision). Each bundle has its own kanban board: SD-21 is
`codex-tranche-4-1`; SD-20 is `codex-tranche-4`; SD-19 is `codex-tranche-3`.
**Each bundle has its own progress file** — SD-21's loop writes exclusively
to `./progress.md`; SD-20's
loop writes exclusively to `~/workspace/SD-20-rules-engine-completeness-progress.md`.

This file is **fully self-contained**. It does not read from, look up, or
inherit procedural mechanics from any other bundle's loop-instruction. The
cycle mechanics used here are captured below in full. If a future session
needs to recover SD-21's cycle mechanics from a clean checkout, this file
is sufficient on its own.

The loop uses the **matured** operator-loop model — the same procedural shape SD-13 established (1-cycle-at-a-time, single criterion per cycle, post-mortem kanban card, atomic direct commit to `tranche/4-1`, no ephemeral branches). No live inheritance from any other bundle's loop file; the procedural mechanics are internalized below:
- Linear commit-to-tranche/4-1 (no ephemeral feature branches; no PRs; no auto-merge).
- Per-cycle kanban card as post-mortem record (on `codex-tranche-4-1`).
- Per-cycle progress-doc entry appended to SD-21's own progress file `./progress.md`.

See `./decisions.md`
§6 for the rationale. SD-13's mature loop model (1-cycle-at-a-time, single criterion per cycle, post-mortem kanban card, atomic direct commit to `tranche/4-1`) is internalized in the §Per-cycle procedure below.
for the SD-13 patterns preserved unchanged.

## Required reading (every cycle)

### 1. Canonical handoff doc

```
cat /home/ubuntu/workspace/SD-21-campaign-manager-and-persistence-scope-draft.md
```

This is the canonical scope doc. The 30 acceptance criteria live here by section number (§1.1 Epic 1 = Code-Side Identifier Cleanup, §1.2 Epic 2 = Campaign manager + Drive persistence, §1.3 Epic 3 = Update UI bug remediation, §1.4 Epic 4 = Closure Epilogue, §1.5 Epic 5 = Build Version Numbering, §1.6 Epic 6 = Single-class coverage completion, §1.7 Epic 7 = Multiclass stacking, §2 promotion gate). Each criterion's acceptance criterion prose and concrete corpus/code pointers live here.

### 2. Progress doc (SD-21's own; loop's working memory)

```
cat /home/ubuntu/workspace/SD-21-campaign-manager-and-persistence-progress.md
```

This is SD-21's own progress doc — separate from SD-18's, SD-19's, and
SD-20's. Each bundle has its own progress file (per operator directive
2026-07-16: "each should use it's own progress file set"). Created on
first run if missing; frontmatter mirrors SD-18's progress doc shape
(`title`, `mirrors` pointing at the scope draft, `created`,
`snapshot_as_of`). Loop's claim protocol lives here under a single
`## SD-21 cycles` section. SD-21 is the only bundle writing to this
file. Each epic maintains `done` / `in-flight` / `open` status rows with
cycle-id, commit SHA, and card id.

The progress doc's `## Status matrix` block (one row per planned loop)
is the at-a-glance quick reference for the operator. On cycle 1 the loop
initializes the matrix skeleton from the scope-draft's §1.1–§1.8
acceptance criteria (every criterion listed as `pending`). On every
subsequent cycle the loop updates the matching row's `Started` (`—` or
ISO timestamp of the cycle that first claimed it), `Duration` (`—` or
the cycle's elapsed seconds, e.g. `~2700s`), and `Status` (`pending` →
`running` while the cycle is in flight → `complete` on green tests / merge
or `blocked` on real blocker per §Open blockers). Row identity is keyed
by the criterion identifier (e.g. `campaign_manager:character_create`,
`drive:snapshot_save`, `apg:barbarian_class_grant`). The matrix is
edited in place; the per-cycle log under `## Cycle log` continues to hold the full per-cycle evidence.

### 3. Required reading from SD-19 (the table-store pattern SD-21 extends)

```
grep -A 9 "Source-book subdirectories" /home/ubuntu/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/decisions.md
# and
grep -A 20 "Enumeration inside SD-20" /home/ubuntu/workspace/programs/codex/requirements/SD-21-campaign-manager-and-persistence/decisions.md
```

The source-book subdirectory pattern (APG → `rules_tables/apg/`, ACG → `rules_tables/acg/`) and the `RuleSetId` enum threading are the load-bearing authority surface for SD-21's ingestion epics. Do not re-derive; cite these sections when explaining eligibility, route around them when picking the next cycle, and update the SD-21 progress-doc section when they are no longer the binding constraint.

### 4. Live git state

```
cd /home/ubuntu/workspace/repos/codex
git fetch origin tranche/4-1
git log origin/tranche/4-1 --oneline -5
git worktree list --porcelain
```

(No `git ls-remote origin | grep -E 'loop/tranche3-cycle-'` check — SD-21 has no feature branches.)

### 5. In-flight detection

```
ps -eo pid,etime,stat,cmd | grep -iE 'claude' | grep -v grep
```

If any `claude` process is running with a prompt that names a specific SD-21 acceptance criterion, do NOT pick that criterion. Cycle exits with `CLAIM-EXISTS` status; loop restarts.

## Concurrency rules (read first, obey always)

These rules are structural. Two concurrent cycles that touch the same file are guaranteed to collide; the loser will be Tech-Priest (or the operator) having to reconcile.

### File-touch partition (the hard rule)

The SD-21 cycle surface is concentrated in these files:

| File | Purpose | Cycles that may touch it |
|---|---|---|
| `apps/desktop/src-tauri/src/sd*_*.rs` (7 files, per 2026-07-18 verification: `sd16_browser_handoff.rs`, `sd19_spell_catalog.rs`, `sd19_race_catalog.rs`, `sd19_equipment_catalog.rs`, `sd19_class_catalog.rs`, `sd13_support_state_matrix.rs`, `sd19_corpus.rs`) | EDIT (rename); Epic 1 criterion 1 cycle owns each Rust Tauri command rename + JS invoke-string + test assertion update. | One cycle at a time. |
| `apps/desktop/src/sd16/` (full TypeScript subtree, `feedback/` and `update/`) | EDIT; Epic 1 criteria 2-4 cycles sweep TS function and constant renames, `data-testid` rewrites, inline doc-comment rewrites. ~15 files in scope per criterion. | One cycle per file. |
| `tests/sd21_<criterion>.rs` (Epic 1 cases) | Per-cycle test file. | One cycle per file. |
| `src/campaign/mod.rs` | NEW; the `CampaignSnapshot` types (per 2026-07-18 cycle-2 addendum: sibling to `src/saved_character/`, not `src/rules_core/` — campaigns aren't rules-computation; supersedes the originally-planned `src/rules_core/campaign.rs` path). Touched only by Epic 2's cycles (definition) — other epics read but don't edit. | One cycle at a time (Epic 2's cycles). |
| `src/campaign/local_store.rs` | NEW; the concrete `CampaignStore` struct (per 2026-07-18 operator directive: a concrete struct mirroring `SavedCharacterStore`, NOT a `CampaignBackend` trait — no trait-object pattern exists in this codebase; supersedes the originally-planned `src/rules_core/persistence/mod.rs` trait path) plus its local-folder impl (OAuth/Drive-API descoped from SD-21; supersedes the originally-planned `persistence/drive.rs` Drive-API adapter — targets a local folder the user points at a Drive-for-Desktop-synced directory). Epic 2's cycles only. | One cycle at a time. |
| `apps/desktop/src-tauri/src/campaign_drive.rs` | EDIT (extend), NOT NEW — per 2026-07-18 verification, PR #320 already landed this file with a real `write_campaign_drive_artifacts` local-disk command. Epic 2's cycles extend it with the `CampaignSnapshot`/`CampaignBackend`-backed commands (`drive_list_campaigns`, `drive_load_campaign`, `drive_save_campaign`, `drive_delete_campaign` — operating on the local folder, no OAuth). | One cycle at a time. |
| `apps/desktop/src/boundary/writeCampaignDriveArtifacts.ts` | EDIT (extend), NOT NEW — per 2026-07-18 verification, this is the actual landed boundary filename (not `campaignDrive.ts`). Epic 2's cycles only. | One cycle at a time. |
| `tests/sd21_<criterion>.rs` (Epic 2 cases) | Per-cycle test file. | One cycle per file (its owning criterion). |
| `apps/desktop/src/sd16/update/fetch.ts` | EDIT (extend), NOT NEW — per 2026-07-18 verification this file already exists (channel-index/manifest fetch); Epic 3's criterion 12 cycle adds a release-notes-body fetch path. Epic 3's criterion 12 cycle only. | One cycle at a time. |
| `apps/desktop/src/sd16/update/controllerAdapter.ts` | EDIT; `runCheck` assignment of `deps.releaseNotes`, `computeDecision` success-path rewire, `loadSd16MountTimeState` real-probe wiring. Epic 3's criteria 12-14 cycles only (after Epic 1 has renamed its identifier leaks). | One cycle at a time per file. |
| `apps/desktop/src-tauri/src/update/transaction.rs` | EDIT (implement stubs), NOT a new `install_eligibility_probe.rs` file — per 2026-07-18 verification, `is_install_eligible` (`:659`) and `perform_install` (`:680`) already exist here as registered, not-wired Tauri command stubs (registered in `main.rs:116-117`). Epic 3's criterion 13 cycle implements these stubs in place. **The prior `install_eligibility_probe.rs` row is void — no such file exists and none should be created.** | One cycle at a time per file. |
| `apps/desktop/src/sd16/update/eligibility.ts` | UNTOUCHED by Epic 3 — `decideEligibility` is already complete. No cycle edits this file in Epic 3. | Read-only. |
| `apps/desktop/src/sd16/update/CheckPanel.tsx` | UNTOUCHED by Epic 3 — `renderReleaseNotes` is the load-bearing display surface that already renders body when `deps.releaseNotes` is non-null. No cycle edits this file in Epic 3. | Read-only. |
| `apps/desktop/src/sd16/update/updateModel.ts` | UNTOUCHED by Epic 3 — `Sd16ReleaseNotes` type is reused as-is. | Read-only. |
| `tests/sd21_<criterion>.rs` (Epic 3 cases) | Per-cycle test file. | One cycle per file (its owning criterion). |
| `apps/desktop/package.json` | EDIT (version bump); Epic 5 criterion 25 cycle bumps `"version"` to `"0.4.<current_build>"` (per 2026-07-18 verification, currently `"0.1.0"` and the anchor is `0.4.94`, not `0.4.93` — latest cut release is `alpha v0.0.93`); major stays `0` until first main-publish; tranche stays `4` because `tranche/4-1` is a dash release off Tranche 4; build is the next monotonic counter value after the last committed build on `tranche/4-1`. | One cycle at a time. |
| `apps/desktop/src-tauri/tauri.conf.json` | EDIT (version bump); Epic 5 criterion 25 cycle bumps `"version"` to `"0.4.<current_build>"` (currently `"0.1.0"` per 2026-07-18 verification). | One cycle at a time. |
| `apps/desktop/src-tauri/Cargo.toml` | EDIT (version bump); Epic 5 criterion 25 cycle bumps `version =` to `"0.4.<current_build>"` (currently `"0.1.0"` per 2026-07-18 verification). `Cargo.lock` updates on next `cargo check`. | One cycle at a time. |
| `.github/workflows/publish-tester-release.yml` | EDIT (stamp format); per 2026-07-18 operator directive, added to Epic 5's partition — line 62's `VERSION="0.0.${GITHUB_RUN_NUMBER}"` publish-time stamp is updated to the new triple shape (e.g. `VERSION="0.4.${GITHUB_RUN_NUMBER}"`) in the same cycle as criterion 25, or published/tester builds keep displaying the old scheme regardless of the repo-file bump. | One cycle at a time (Epic 5 criterion 25's cycle). |
| `apps/desktop/src/sd11/status/createSd11WorkbenchStatus.ts` | EDIT (build-label format); Epic 5 criterion 26 cycle sets `BUILD_PREFIX = 'Codex'` and template `${BUILD_PREFIX} ${buildVersion}` (matches `<major>.<tranche>.<build>` triple from the version files). | One cycle at a time. |
| `apps/desktop/src/sd11/loadSd11TesterWorkbenchSurface.test.ts`, `apps/desktop/src/sd11/status/createSd11WorkbenchStatus.test.ts`, `apps/desktop/src/testSupport/makeSurface.ts` | EDIT (test fixtures); Epic 5 criterion 26 cycle updates assertions/fixtures from `codex@0.0.0-test` to `Codex 0.4.<build>` shape. | One cycle at a time per file. |
| `docs/SD-21/release-closure-checklist.md` | NEW; Epic 5 criterion 27 cycle writes the four-step closure-process checklist using the `<major>.<tranche-base>.<build>` triple (per-position increment rules: build per-CI-build, tranche per-tranche-promotion, major per-main-publish). | One cycle at a time. |
| `src/rules_core/pilot_compute.rs` (Epic 6 surfaces) | EDIT; per 2026-07-18 verification `compute_pilot_base_chassis` (`:4568`) is confirmed still Fighter-only (calls only `compute_fighter_chassis` at `:6364`) — criterion 25 refactors it to per-class dispatch; criterion 26 **creates** `compute_wizard_chassis` (it does not exist yet — `supported_wizard_level` at `:13967` already gates 1-20, `MAX_SUPPORTED_WIZARD_LEVEL=20` at `:2751`, but only for a spell-baseline explanation, not chassis) and should consume the existing per-class `BabProgression`/`GoodSaves` data in `rules_tables/crb/class_tables.rs:73-83`; criterion 27 lands per-class foundation module shapes (one per class). | One cycle at a time; Epic 6 cycles serialize against each other on `pilot_compute.rs`. |
| `tests/sd21_<class>_chassis_computes.rs` (Epic 6 cases) | NEW; per-class acceptance tests exercising the resolved CharacterInput's post-compute state. | One cycle per file. |
| `src/rules_core/pilot_compute.rs` (Epic 7 surfaces) | EDIT; per 2026-07-18 verification confirmed no multiclass logic exists anywhere in `src/rules_core/`. Epic 7 criterion 28 cycle extends dispatch to length-2+ `class_levels` via `compute_multiclass_base_chassis`; criterion 29 lands the PF1 best-fractional-progression save formula (calls into `class_tables.rs`'s `GoodSaves` — NOT `decideEligibility.class_save_bonus`, which doesn't exist in the Rust engine; `decideEligibility` is the unrelated TypeScript update-UI eligibility function); criterion 30 reconciles per-class feature integration. | One cycle at a time; Epic 7 cycles serialize against each other and against Epic 6 on the same file. |
| `tests/sd21_multiclass_<X>_<Y>_chassis_computes.rs` (Epic 7 cases) | NEW; multiclass acceptance tests (Fighter 4 / Wizard 4 etc.). | One cycle per file. |
| `programs/codex/requirements/SD-21-campaign-manager-and-persistence/release-notes.md` | NEW; Epic 4 criterion 19 cycle generates release notes (New features, Bug fixes, Maintenance, Versioning sections). | One cycle. |
| Epic 4 sweep (closure PR, worktree cleanup, branch cleanup) | Epic 4 criterion 17 + 18 cycles run `gh pr create`, `git worktree remove --force`, `git branch -d`. Operates on integration-branch metadata, not on per-file content. | One cycle. |

The chassis and corpus-aware seam files (`pilot_compute_corpus.rs`, `support_state_matrix.rs`) stay untouched by SD-21 Epics 1-2-3-5. The Tauri `character_hub.rs` is read by SD-21's Epic 2 (for the persistence boundary reference) but not modified. Per the 2026-07-18 verification findings above, `apps/desktop/src-tauri/src/update/transaction.rs` (not a separate `install_eligibility_probe.rs`) already carries the `is_install_eligible` and `perform_install` command stubs under `apps/desktop/src-tauri/src/` per Epic 1's identifier discipline (no `sd*_` prefix on file names — that's a follow-on bundle).

Epic 3's file-touch set is disjoint from Epic 2's set (`apps/desktop/src/sd16/update/` vs. the Rust rules-core / Drive modules), and disjoint from Epic 1's set (Epic 1's identifier-leak files in `apps/desktop/src/sd16/` are renamed in the cleanup, not edited for behavior; Epic 3 lands behavior fixes on the post-Epic-1 file). Epic 3 cycles can therefore run after Epic 1's identifier renames have landed.

Epic 5's file-touch set is the three version files plus `createSd11WorkbenchStatus.ts` plus three test-fixture files plus one new docs file. Epic 5 fires before Epic 4's closure sweep (Epic 5's version commit must be in Epic 4's closure PR's commit history). Epic 5 is independent of Epics 1-3 (it doesn't touch the rules-core / Drive / sd16 / sd19 files).

Epic 6's file-touch set is metadata-only (closure PR, worktree, branch, release notes) and disjoint from every other epic's touched files. Epic 6 fires LAST and **serializes against all prior epics on the integration branch's commit history**: the closure PR must reflect all 30 criteria's commit evidence, so Epic 6's cycle waits for all preceding criteria to land before scanning. If Epic 6 is run while a prior criterion is still `pending`, the criterion-16 final-scan surfaces it as a `## Open blockers` entry and pauses for operator decision (no counterfeit completion).

### Per-cycle spawn budget (the default)

Default: **1 cycle at a time.** Reason: the file-touch partition collapses any parallel attempt into a serial one for Epic 1's shared `apps/desktop/src/sd16/` files (any two Epic 1 cycles racing on the same `apps/desktop/src/sd16/feedback/` file collide on it), Epic 2's `src/campaign/mod.rs` and `src/campaign/local_store.rs` modules (per the 2026-07-18 cycle-2 addendum's path correction), and Epic 4's `controllerAdapter.ts` (any two Epic 4 cycles at once collide on it), and for the new file Epic 5 introduces (`docs/SD-21/release-closure-checklist.md`'s sole cycle owns that path). Two cycles in parallel means two cycles racing on the same fixture file.

To run more than one cycle in parallel you must show that the second cycle touches a disjoint file set. That's possible only when one cycle is in Epic 1 (Rust Tauri command renames in `apps/desktop/src-tauri/src/sd*_*.rs`) and the other is in Epic 1 (TS renames in `apps/desktop/src/sd16/`), as long as the two file paths don't overlap; or when an Epic 4 cycle runs alongside an Epic 1/2/3 cycle, since Epic 4's `apps/desktop/src/sd16/update/` files are disjoint from the rules-core / Drive / `sd19_*.rs` files once Epic 1 has renamed them; or when Epic 5's three version files run alongside an Epic 1/2/3 cycle (the version files are disjoint from rules-core / Drive / sd16 work). Epic 3 and Epic 1 partially parallelize on disjoint files in `apps/desktop/src/sd16/` (one renames, one adds behavior); serial within Epic 3 itself on `controllerAdapter.ts`. Epic 4 does not parallelize against anything — its criterion-16 final-scan waits for all prior criteria to be in their final state. For code-bearing cycles, **1 cycle at a time is the rule** unless cycles are explicitly assigned to non-overlapping file paths.

## Per-cycle procedure (the steps, in order)

### Step 1 — Pick a criterion

From the SD-21 progress doc's `## SD-21 cycles` `open` list, pick the smallest unclaimed acceptance criterion. Priority order:

1. **Epic 1 cycles first** (Code-Side Identifier Cleanup). All subsequent work lands on clean identifiers; new readers the operator is onboarding after the next release see clean code on their first read.
2. **Epic 2 (Campaign Manager + Drive) and Epic 3 (Update UI bug) cycles next**, in parallel if the operator hosts two loop channels. Epic 2 lands the engine-side API; Epic 3 lands behavior fixes on the post-Epic-1 file.
3. **Epic 6 (Wizard) and Epic 5 (Build Version) and Epic 7 (Multiclass) cycles thereafter**. Epic 6 must precede Epic 7 (per the bug handoff's two-phase split).
4. **Epic 4 (Closure Epilogue) fires LAST** — its criterion-16 final-scan is the bundle's closure gate.

**Eligibility check.** A criterion is eligible when:

1. The criterion has not yet reached `done` per the progress doc.
2. No live `claude` process is working on that criterion (in-flight detection above).
3. The chosen class/book is **actually computable** from the existing engine surface — i.e. it is a new structured-data population in the appropriate `rules_tables/<book>/` directory, OR it is a `CampaignBackend` API extension, NOT a new subsystem. New subsystems are trunk-level decisions, not cycle decisions.

When several criteria tie on priority above, prefer the one that has not had a cycle attempted in the last 3 cycles (read the progress doc's `## SD-21 cycles` section to check). The loop's job is to advance the **frontier**, not to retry the same criterion forever.

### Step 2 — Pick the criterion's work-unit

- **Epic 1**: one identifier-class per cycle (e.g. one Rust Tauri command rename; one TS function/class rename batch; one `data-testid` sweep; one inline doc-comment sweep).
- **Epic 2**: one class of campaign-manager functionality per cycle (e.g. `CampaignSnapshot` round-trip, `load_campaign` correctness, `save_campaign` correctness, `create_campaign` correctness, `delete_campaign` correctness, Drive OAuth round-trip, markdown serialization).
- **Epic 3**: one Update-UI-bug-fix per cycle (e.g. release-notes fetch path, probe-and-eligibility, `computeDecision` rewiring, per-cycle tests).

### Step 3 — Verify the working tree is on tranche/4-1

```bash
cd /home/ubuntu/workspace/repos/codex
git fetch origin tranche/4-1
git checkout tranche/4-1
git pull origin tranche/4-1
git status --porcelain | wc -l   # expect 0; if non-zero, exit CLAIM-EXISTS
```

### Step 4 — Write the failing test first

Add `tests/sd21_<criterion>.rs`. Mirror the shape of the most recent sibling cycle's test file. The test must fail for the intended reason when run against `origin/tranche/4-1` as the base.

```bash
cargo test --locked --test sd21_<criterion> 2>&1 | tail -40
```

Capture the failing output. It is the RED evidence.

### Step 5 — Implement the smallest change that makes the test pass

For SD-21 cycles, the change is one of:

- **Epic 1 — Identifier Cleanup**: `apps/desktop/src-tauri/src/sd*_*.rs` rename + JS invoke-string + test assertion update. `apps/desktop/src/sd16/` TS function/class/data-testid/doc-comment sweep. Cross-cuts every file in those scopes.
- **Epic 2 — Campaign Manager**: extension to the `CampaignSnapshot` types in `src/campaign/mod.rs`. Add fields, fix structure, document cross-references.
- **Epic 2 — Campaign Manager**: extension to the `CampaignBackend` trait. Add methods, fix return types, document semantics.
- **Epic 2 — Campaign Manager**: extension to the local-folder `CampaignStore` impl in `src/campaign/local_store.rs` (concrete struct, not a trait, per 2026-07-18 operator directive; OAuth/Drive-API descoped per 2026-07-18 operator directive). Add folder handling, conflict resolution.
- **Epic 2 — Campaign Manager**: extend the already-landed Tauri commands in `apps/desktop/src-tauri/src/campaign_drive.rs` (real local-disk writer from PR #320) with the `CampaignSnapshot`/`CampaignBackend`-backed commands.
- **Epic 2 — Campaign Manager**: extend the already-landed GUI boundary in `apps/desktop/src/boundary/writeCampaignDriveArtifacts.ts` (not `campaignDrive.ts`).
- **Epic 3 — Update UI Bug**: extend the already-existing `apps/desktop/src/sd16/update/fetch.ts` (release-notes-body fetch path), `controllerAdapter.ts` (probe + `computeDecision` rewiring + `loadSd16MountTimeState` real-probe wiring), implement the already-existing-but-not-wired Tauri command stubs `is_install_eligible` and `perform_install` under `apps/desktop/src-tauri/src/update/transaction.rs` (not a separate `install_eligibility_probe.rs`).

For all paths, the change must be in the appropriate epic file. The forbidden write scopes are documented in `./risks-and-open-questions.md`.

Run:

```bash
cargo test --locked --test sd21_<criterion> 2>&1 | tail -40
cargo test --locked 2>&1 | tail -20
cargo clippy --locked --tests -- -D warnings 2>&1 | tail -20
```

All three must be green. Capture the output. It is the GREEN evidence.

### Step 6 — Commit, push directly to tranche/4-1

```bash
git add src/campaign/mod.rs \
        src/campaign/local_store.rs \
        apps/desktop/src-tauri/src/campaign_drive.rs \
        tests/sd21_<criterion>.rs \
        tests/fixtures/sd21/<file>
git -c user.name='Todd Hintzmann' \
    -c user.email='todd@hintzmann.net' \
    commit -m "feat(sd21): <criterion> (<row transition>)"
git push origin tranche/4-1
```

The commit lands directly on `tranche/4-1`. Capture the commit SHA — it is the durable receipt (recorded as `merge_receipt_sha` in the card body and progress doc, by analogy with SD-19's atomic-slice receipt pattern).

### Step 7 — Open the PR (NOT APPLICABLE to SD-21)

SD-21 has no PRs. Per `decisions.md` §6 (no-branches convention): every cycle commits directly to `tranche/4-1`. The `tranche/4-1 → develop` promotion PR is operator-driven only and happens once at SD-21 closure, not per cycle.

### Step 8 — Auto-merge to tranche/4-1 (NOT APPLICABLE to SD-21)

SD-21 has no auto-merge. The commit is already on `tranche/4-1` by
construction.
### Step 9 — Cleanup (NOT APPLICABLE to SD-21)

SD-21 has no ephemeral branch to clean up. The next cycle's Step 3 checkout handles any stale working-tree state.

### Step 10 — Mint the kanban card (post-mortem record)

```bash
hermes kanban --board codex-tranche-4-1 create \
  "SD21 <criterion> (<epic-section>) [cycle <cycle-id>]" \
  --assignee operator \
  --workspace scratch \
  --initial-status done \
  --created-by operator \
  --priority 3 \
  --body "<card body per schema below>"
```

Card body schema:

```
epic: SD-21
criterion_section: <scope doc section reference, e.g. "§1.1 Epic 1 — Campaign manager + Drive persistence">
row_or_kind: identifier:rust_tauri | identifier:ts_function_or_class | identifier:data_testid | identifier:doc_comment | campaign:snapshot | campaign:persistence | campaign:drive_oauth | campaign:conflict_log | update:release_notes_fetch | update:installed_state_probe | update:computeDecision_rewire | version:patch_bump | version:build_label_format | version:closure_checklist | class:compute_dispatch | class:compute_wizard_chassis | class:multiclass_dispatch | class:multiclass_save_stacking | class:multiclass_feature_integration
evidence_tier_before: <previous matrix row state>
evidence_tier_after: <new matrix row state after this commit>
merge_receipt_sha: <commit SHA on tranche/4-1>
cycle_id: <ISO-8601 timestamp>
cargo_test_summary: <test summary string>
clippy_signal: clean | dirty
cycle_timing_seconds: <N>
self_heals_applied: <list, empty if none>
next_required_uplift: <recommendation for next iteration>
corpus_existence_verified: yes — <corpus path> :: <KEY: used>
rule_set_used: Crb
```

### Step 11 — Update the progress doc

Edit `./progress.md` in place:

1. Update the `snapshot_as_of` line in the frontmatter to the current `tranche/4-1` HEAD short SHA. (SD-21's own snapshot; not shared with SD-18/SD-19/SD-20.)
2. Update the cycle's row in the `## Status matrix` block (see `## Required reading (every cycle) §2. Progress doc`). The matrix is the operator's quick reference; updated rows must reflect this cycle's outcome:
   - `Started`: ISO timestamp of the cycle that first claimed the criterion (or `—` if not yet started).
   - `Duration`: elapsed seconds for this cycle (e.g. `~2700s`), or `—` if not yet started.
   - `Status`: `complete` on green tests / merge landed; `blocked` on real blocker per §Open blockers; `pending` if the row's not yet been touched; `running` if the supervisor's stream is currently mid-cycle. A cycle that produced a landed commit on the criterion row updates its `Status` to `complete` and rolls the `commit SHA` into the row's receipt column.
   - On cycle 1 (the cycle that initially creates the progress doc), the loop also writes the matrix skeleton from the scope-draft's §1.1–§1.8 acceptance criteria — every criterion as a `pending` row with `Started` and `Duration` set to `—` and `Brief description` copied from the scope draft's per-criterion prose.
3. Append a new entry to the cycle log under `## Cycle log`:

```
### cycle-<cycle-id> | <criterion> | <commit sha> | <card id> | <evidence transition> | cargo test <N>/<N> green | clippy clean | <timing>
```

3. If the cycle did not produce a landed commit (test could not be made green, corpus record missing, in-flight process blocked the criterion, etc.), add an `## Open blockers` entry under SD-21's section with the specific reason so the next cycle routes around it.

Do NOT rewrite the doc from scratch. Edit in place so the diff is small and auditable.

### Step 12 — Exit the cycle

Print a final 7-line report and exit:

```
cycle: <cycle-id>
criterion touched: <criterion>
row_or_kind: <row_or_kind>
commit: <commit sha on tranche/4-1, or 'no commit: <reason>'>
card: <hermes kanban card id, or 'no card: <reason>'>
verify: cargo test <X>/<X> green; clippy clean
status: GREEN | FAIL | NO-OP | CLAIM-EXISTS
```

`/loop` restarts the cycle 60 minutes later. The next cycle re-reads the
progress doc and picks the next criterion.

## Self-healing posture

The loop self-heals wherever the failure is mechanically resolvable. The
operator returns from a multi-day run to a list of problems — not a
stopped loop.

### Self-healable conditions (resolve inline, exit GREEN)

| Condition | Detection | Self-heal |
|---|---|---|
| Working tree dirty at cycle start | `git status --porcelain \| wc -l` returns non-zero | Run `git stash` (if previous unfinished attempt) or `git checkout -- .` (stray edit noise); re-verify clean; retry |
| A Drive OAuth token refresh fails because Google revoked the refresh token | Engine surfaces "re-authorize required" diagnostic | GUI prompts user to re-authorize via the Drive authorize button; not blocking the loop |
| A markdown file on disk fails to parse (e.g. corrupt YAML frontmatter) | Per-file parse error on campaign load | Surface the file path and parse error in the load result; don't fail the whole load — the user can repair the file manually and re-load |
| A `RuleSetId` variant from a future book (e.g. `RuleSetId::Um`) appears during SD-22's content-source ingest work | Compile error or runtime match error | This is **SD-22's surface**, not SD-21's. SD-22 owns `RuleSetId::Apg`, `RuleSetId::Acg`, `RuleSetId::Um`, `RuleSetId::Bestiary1`, etc. SD-21 reads `RuleSetId::Crb` only. A cycle that hits an unknown `RuleSetId` variant surfaces a `## Open blockers` entry with the variant name; the operator records the operator-call to handle it under SD-22. |
| Two cycles both try to add new behavior to the same `apps/desktop/src/sd16/` file (e.g. both Epic 1 and Epic 3 cycles editing `controllerAdapter.ts` concurrently) | Merge conflict on the structured-data file | Resolve inline if mechanical (Epic 1 renames + Epic 3 adds behavior); escalate to operator if semantic |
| A cycle's RED test fails because the canonical `CharacterSummary` mapping is missing | `tests/sd21_character_summary_resolves.rs` fails for a class that hasn't been ingested yet | Route to Open Blockers; operator decides whether the cycle is the right time to extend the canonical mapping |
| Markdown file on disk has a stale `nonce` (from a Drive sync edge case) | `CampaignSnapshot.nonce != saved_nonce` on load | Engine surfaces "stale nonce, please save again"; doesn't trigger conflict log unless the *content* also differs |

### Non-self-healable conditions (write to `## Open blockers`, exit FAIL)

| Condition | Detection | Why not self-heal |
|---|---|---|
| The campaign-shape boundary contract (`CampaignSnapshot`) doesn't match the GUI's vibe-coded expectations from PR #316 | Engine produces a snapshot the GUI rejects (specific field naming or shape mismatch) | Boundary contract drift — the contract needs to be amended, the GUI needs to be patched, or both; cycle can't fix this alone |
| The Google Cloud Console project for codex doesn't have OAuth credentials configured (no client ID, secret, or redirect URI registered) | Engine surfaces "OAuth credentials not configured" on first campaign-create attempt | Operator-side fix (Google Cloud Console); bundle can't proceed |
| Two `claude` processes both touch `src/campaign/` or any per-epic module file | `ps -eo pid,etime,stat,cmd \| grep claude` shows multiple in-flight on the same file set | Structural: one-lane-at-a-time rule |
| A campaign on disk has `campaign.md` with frontmatter YAML that parses but typed values fail (e.g. `level: "four"` instead of `4`) | Per-field type error during load | Engine surfaces the file + the field with the bad value; cycle can't fix the user's data |
\| Cycles for class-extension work (e.g. Wizard, Cleric, Sorcerer) across Epic 6/Epic 7 land on the same `src/rules_core/pilot_compute.rs` region with conflicting save-stacking or feature-integration assumptions | Cycle's per-class test fails or the multiclass BAB/save test fails | Defer the second cycle until the first cycle's tests are green; surface as `## Open blockers` if the conflict indicates a real bug in Epic 6/Epic 7's class shape |
| Cargo test regresses on a row other than the one the cycle touched | Full suite regresses after a cycle's change | Sibling-preservation is a hard rule |
| Progress doc and live matrix disagree on a row's `evidence_tier` (not just stale snapshot) | Cycle's expected vs. actual differ | Manual operator reconciliation required |

## Hard stops (refuse, exit FAIL)

The cycle refuses to advance when any of the following is true. In every
case the cycle writes the reason to `## Open blockers` in the progress
doc and exits with `FAIL`.

- A slice branch has diverged from `tranche/4-1` in a way that needs a manual rebase.
- The progress doc and the live matrix disagree on a row's `evidence_tier` and the disagreement is not just a stale snapshot.
- `cargo test --tests` regresses on a row other than the one the cycle touched. Sibling-preservation is a hard rule.
- Two live `claude` processes are working on cycles that would both touch `src/campaign/` or any per-epic module file.
- **SD-21-specific:** A cycle's RED test depends on a class-shape (Wizard / Cleric / etc.) feature that isn't yet extended in `compute_<class>_chassis`.

## What "campaign-manager closure" actually means for SD-21

SD-21 closes when every closure gate in `acceptance-and-verification.md` passes AND Epic 4's `codex-tranche-4-1` board shows every criterion `complete`. Concretely:

1. **Epic 1 closed**: every `sd*_*` identifier in source has been renamed to a descriptive PascalCase / snake_case shape; `data-testid` attributes and inline doc-comments are clean.
2. **Epic 2 closed**: `CampaignSnapshot` types and `CampaignBackend` trait land; Drive adapter implements the trait; Tauri command surface exposes campaign manager to the GUI; conflict resolution works against the fixture-folder.
3. **Epic 3 closed**: Update-tab release-notes-and-eligibility bug is fixed; per-fetch and per-probe tests cover each outcome path.
4. **Epic 6 closed**: `compute_pilot_base_chassis` dispatches by class (not Fighter-only); `compute_wizard_chassis` extends from level-cap 11 to full 1-20 with PF1-correct BAB/saves/spellbook/features; per-class foundation shapes land.
5. **Epic 7 closed**: `compute_pilot_base_chassis` extends to length-2+ `class_levels`; saves use PF1's best-fractional-progression rule (NOT a naive sum); per-class feature integration reconciles without clobbering.

The campaign manager is **not** locked to character sheet feature completeness. A character that joins a campaign has chassis-only data during the SD-21 / SD-20 parallel window; that data auto-upgrades when SD-20 closes and the user's character sheet re-loads into the campaign (per `risks-and-open-questions.md` Flag A and Open Q4).

## How the loop will end

The `/loop` form exits when the operator stops it. There is no automatic
stopping condition. The loop keeps picking the next-best criterion until
every criterion is `done` (closure met) or every criterion has a real
blocker in `## Open blockers`.

The operator can stop the loop at any time; a stopped loop leaves the
progress doc in the state of the last completed cycle, with all open
claims expired, and the operator can resume by relaunching `/loop 60m
/batch /goal <this file>`.

## Operating posture (for the operator launching the loop)

1. **One launch command, run to closure.** Launch with `/loop 60m /batch /goal ./loop-instruction.md`. The loop runs to closure — every criterion `done` or every criterion has a real blocker in `## Open blockers` — and then exits. The operator does not need to inspect progress between cycles or between epics; the loop's own eligibility check + dependency graph + file-touch partition handle the sequencing automatically. The progress doc `./progress.md` is the durable record; the operator reads it on return (whether that's minutes or days later) and sees the final state.

2. **Why one launch, not three windows.** The dependency graph (`epic-breakdown.md`) is the sequencing mechanism:
   - **Epic 1 (Identifier Cleanup)** is the only eligible criterion at launch — every subsequent criterion touches source that Epic 1 has cleaned up. The loop's Step 1 eligibility check naturally serializes this.
   - **Epics 2 (Campaign Manager + Drive) and 3 (Update UI bug)** depend only on Epic 1's renames having landed. They have **disjoint code surfaces** (`src/rules_core/{campaign,persistence}/` vs. `apps/desktop/src/sd16/update/` after Epic 1 has renamed identifiers). The loop's Step 1 priority order (Epic 2 then Epic 3) + the file-touch partition together enable parallel progression without operator intervention.
   - **Epics 6 (Wizard) and 7 (Multiclass)** depend on Epic 1's renames having landed on Tranche-3 substrate (since they edit `src/rules_core/pilot_compute.rs` — same file Tranche-3 work touches).
   
   The loop's Step 1 picks the smallest unclaimed eligible criterion from the progress doc's `## SD-21 cycles` open list. Eligibility includes the dependency-graph gate (epic N's cycles only fire after epic N's prerequisites are `done`). The operator does NOT manually switch launch forms between epics — the loop's own logic handles each transition.

3. **What `/batch` actually does in Hermes.** Per the SD-13 loop-model excerpt (`programs/codex/requirements/SD-18-core-rules-breadth/references/sd13-loop-model-excerpt.md`), `/batch` is the form that lets a single shell invocation run multiple streams concurrently against the shared goal file, with the supervisor managing the 60-minute restart cadence across all streams. The two lanes for Epic 2 + Epic 3 run as two streams inside one `/loop /batch` invocation (when eligible), not as two separate shells.

4. **Default ceiling: 1 cycle at a time per file.** The file-touch partition collapses any parallel attempt for the shared `src/campaign/mod.rs` and `src/campaign/local_store.rs` modules (epic 2, per the 2026-07-18 cycle-2 addendum) and the per-book structured-data directories (epics 2 and 3 each have their own). Two cycles in parallel racing on the same file is a structural violation, not a recommendation.

5. **Watch the progress doc, not the loop output.** The cycle log is the durable truth. If the log shows three cycles in a row with no landed commit, the loop is stuck on a structural problem and the operator should investigate. If you're asleep or away from the terminal, the next time you read the progress doc you see the cumulative state — no operator-attention tax during the run.

6. **Post-mortem record is the kanban board.** Each cycle mints a card on `codex-tranche-4-1` (separate from the chassis-lane `codex-tranche-3` and the per-character-rules-engine-lane `codex-tranche-4`; per operator directive 2026-07-17, replacing the prior 2026-07-16 `codex-tranche-5` directive) with the §Step 10 schema. The loop's Step 10 kanban card mint command is **explicit**: `hermes kanban --board codex-tranche-4-1 create ...` (the `--board` flag is hard-coded, so it works regardless of the operator's `hermes kanban boards current` setting). A 3-day-later operator reads the board to reconstruct what happened.

7. **The 5-hour window applies here too.** A 60-minute cycle × 5 hours = up to 5 landed criteria per 5-hour window per stream. Realistic target: 3-5 criteria per window with 1 cycle each; during the concurrent window (Epic 2 + Epic 3 eligible in parallel after Epic 1 lands), the supervisor runs 2 streams in parallel and realizes 6-10 criteria per 5-hour window if both lanes are green.

8. **SD-20 runs in parallel.** SD-20's loop is on its own `tranche/4` branch (per SD-20's launch-branch decision; not operator-overridden in this bundle's run). Each bundle has its own kanban board: SD-21 is `codex-tranche-4-1`; SD-20 is `codex-tranche-4`; SD-19 is `codex-tranche-3`. Tranche-4-1 closes when SD-21's closure gate (which includes the multiclass support from Epic 6/7) lands.

9. **Force-push discipline on `tranche/4-1` is conservative.** A mid-cycle correction requires a `git reset --soft HEAD~1` + force-push. This is acceptable only when the previous commit was seconds old and no downstream observer has fetched. If the commit has been on `tranche/4-1` for any non-trivial time, escalate to operator before force-pushing.

10. **The `tranche/4-1` branch must exist on origin before the loop's Step 3 fetch succeeds.** Operator creates the branch once (`git push origin tranche/4-1` from the operator's side, after `tranche/4` is merged per operator directive 2026-07-17) and the loop's `git fetch origin tranche/4-1` then resolves cleanly. Until then, Step 3 falls through to the local-only checkout path with a benign "couldn't find remote ref" warning; cycles continue normally on the local branch.

11. **Pre-launch setup checklist (operator action, before first launch).**
    - [ ] `codex-tranche-4-1` kanban board created (board slug: `codex-tranche-4-1`; board display name: "Codex Tranche 4-1 (SD-21 dash release: campaign manager + Drive + Identifier Cleanup + Update UI bug + Wizard + Multiclass + governance epics)"; **operator creates this after SD-20 completes** per operator directive 2026-07-17).
    - [ ] `tranche/4-1` branch pushed to origin (operator runs `git push origin tranche/4-1` once after `tranche/4` is merged).
    - [ ] Operator's interactive `hermes kanban boards current` is set to `codex-tranche-4-1` for operator-driven inspection (note: the loop's Step 10 mint command has `--board codex-tranche-4-1` explicit, so it works regardless of the default-board setting; this step is only for operator inspection convenience).
    - [ ] Google OAuth credentials configured in `~/.hermes/profiles/god-emporer/.env`: `GOOGLE_OAUTH_CLIENT_ID`, `GOOGLE_OAUTH_CLIENT_SECRET`, `GOOGLE_OAUTH_REDIRECT_URI` (per `risks-and-open-questions.md` non-self-healable row 2). Without these, SD-21's Epic 2 cannot ship the Drive adapter.
    - [ ] `./progress.md` does not yet exist; the loop creates it on first run with frontmatter (`title`, `mirrors`, `created`, `snapshot_as_of`). No operator action needed.

12. **How the operator knows SD-21 is done.** The loop runs to closure per `## How the loop will end`: when the progress doc's `## SD-21 cycles` open list is exhausted (every criterion `done` or every criterion has a real blocker), the loop's last cycle prints a final 7-line report and exits. Operator wakes up, reads the progress doc, and sees the final state. No operator-attention tax during the run; no manual switchover between launch forms; the supervisor manages the streams.

13. **Resolving SD-19's `tranche/3` cycle that the launch is racing against.** The `codex-tranche-3` board currently holds 154 cards from the chassis lane (SD-18 + SD-19). The SD-21 loop does not touch any of those — its scope is `codex-tranche-4-1`. If the operator wants to inspect the chassis-lane cycle history, use `hermes kanban --board codex-tranche-3 list` directly; do not switch the default board to `codex-tranche-3` while SD-21 is running.

## Cross-reference

- `./scope-draft.md` — canonical handoff.
- `./decisions.md` — 21-item decision record (SD-21 §1–§21: 9 original decisions plus §10 cross-bundle auto-upgrade on SD-20 close, §11 SD-21 launch branch flip `tranche/5 → tranche/4-1` per operator directive 2026-07-17, §12 resolver cross-book fallback APG→CRB→ACG, §13 Status matrix in progress doc, §14 Q1–Q5 closure summary, §15 Epic 3 lifecycle routing under the spec-domain lifecycle doctrine, §16 identifier discipline + Epic 1 routing under the identifier-discipline doctrine, §17 closure epilogue as standard handoff per the operator directive 2026-07-17, §18 build version numbering `<major>.<tranche-base>.<build>` three-position scheme per the operator directive 2026-07-17, §19 multiclass + broader single-class support (Epic 6 + 7), §20 SD-21 bundle-sized-for-one-tranche posture, §21 operator-deferred shape decisions now closed).
- `./acceptance-and-verification.md` — closure gates.
- `./epic-breakdown.md` — 30 acceptance criteria grouped into 7 epics.
- `./risks-and-open-questions.md` — self-healable vs. non-self-healable split, override flags, open questions.
- `../SD-22/` — sibling bundle (APG + ACG + advanced guides + Bestiary 1 + DM toolkit; scope expanded 2026-07-17 per operator directive).
- `../SD-19/decisions.md` §9 (Source-book subdirectories pattern).
- `../SD-20/decisions.md` (parallel sibling bundle).
