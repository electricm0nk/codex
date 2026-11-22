---
title: SD-24 — Beta Readiness + Multiclass Stacking + Equipment Completeness — Scope Draft (Tranche-5-2 Bundle)
status: planning-ready (operator directives 2026-07-21: scope tightened to 8 epics / 35 criteria, Fighter+Wizard only multiclass scope per "not every combination", full PF1 core rules + APG + ACG + Bestiary 1 corpus, strict equipment-fields coverage, deterministic-seeded-then-dynamic loop dispatcher with /loop 1m /batch and Claude-Code-driven cycle mechanics, move-not-copy publish to docs/release/SD-24-.../ on tranche/5-2)
date: 2026-07-21
canonical_branch: tranche/5-2 (dash from tranche/5; SD-22 closed on tranche/5 → develop; SD-23 active on tranche/5-1 → develop; SD-24 sequence number per the canonical branch sequence)
kanban_board: codex-tranche-5 (reused after SD-22 closure PR landed; same as SD-22's operational board; SD-24 mint uses --board codex-tranche-5 explicitly so cycle minting works regardless of operator's default-board setting)
companion_to: ./decisions.md
mirror_of: ./decisions.md
loop_launch_form: "/loop 1m /batch /goal ./loop-instruction.md"
cycle_dispatch_model: deterministic-seeded-then-dynamic (per operator directive 2026-07-21; seed walks the 35-criterion deterministic list, ## DISCOVERED entries prioritize-bump into the dispatcher queue on operator call)
publish_mode: move-not-copy (workspace copy → repo on tranche/5-2 at first launch; workspace copy deleted on commit per operator directive 2026-07-21 — "we plan in workspace, we execute in the repo. We do not have more than one copy")
---

# SD-24 — Beta Readiness + Multiclass Stacking + Equipment Completeness (Tranche-5-2 Bundle)

> ## ⚠️  OPERATING METHOD — REQUIRED FOR THIS BUNDLE  ⚠️
>
> **This bundle is operated via the `/loop 1m /batch /goal ./loop-instruction.md` invocation model — NOT a one-shot task.** After exiting plan mode, the coding harness (or operator) is **required** to launch SD-24 as:
>
> ```bash
> cd docs/release/SD-24-beta-readiness-and-multiclass
> /loop 1m /batch /goal ./loop-instruction.md
> ```
>
> `/loop 1m` is the cadence — every 1 minute a new supervisor tick fires (operator-pinned 2026-07-21; replaces the prior 60m default to match the operator's "no perceptible timer" requirement on the 20-hour dense-run-to-5am run).
> `/batch` is the supervisor's concurrency primitive; the file-touch partition in the loop-instruction enforces one cycle per file at a time.
> `/goal <loop-instruction.md>` is the bundle's authoritative instruction file — the loop does not look anywhere else for cycle mechanics.
>
> Per operator directive 2026-07-21: **Claude Code handles looping and dispatching**. Kanban is the durable receipt layer only — cards are minted on `codex-tranche-5` only with a done receipt; kanban does NOT dispatch work (per the SD-22 cloud-launch retrospective's "kanban attempted to dispatch work and needed to be stopped" failure mode, which SD-24's architecture is explicitly designed to prevent).
>
> A coding harness picking up this bundle after exiting plan mode reads this callout before reaching any other prose. Do not run cycles manually. Do not skip the loop.
>
> **Pre-launch checklist (operator action only, before the loop's first launch):**
>
> 1. Confirm `codex-tranche-5` kanban board exists. Run `hermes kanban list-boards` and confirm `codex-tranche-5` is reachable.
> 2. Confirm `tranche/5-2` branch is pushed to origin. Per operator directive 2026-07-21, another agent creates `tranche/5-2`; operator verifies.
> 3. Run `git status --porcelain | wc -l` on `tranche/5-2` — must return `0` before loop launch.
> 4. Doctrines loaded: `wired-integration-discipline` and `identifier-discipline` skills are in the loop's skill list.
> 5. Build counter captured in `decisions.md` §3 from develop's `Cargo.toml` workspace version (per the `<major>.<tranche-base>.<build>` scheme; SD-24's first concrete value lands as `0.5.<current_build>` since tranche/5-2 carries tranche-base=5, same as SD-22).
>
> Then launch with `/loop 1m /batch /goal ./loop-instruction.md` and the bundle runs autonomously to closure.

## 0. Preamble

This is the canonical handoff for SD-24. The `/loop 1m /batch /goal ./loop-instruction.md` invocation reads this file plus its sibling doctrine files and runs to closure.

Working in bounded cycles against the integration branch `tranche/5-2` (per operator directive 2026-07-21). Each cycle lands one acceptance criterion or one discovered-work item.

The progress doc `./progress.md` (created on first cycle by the loop) carries the cycle-log + status matrix + the `## TODO` / `## DONE` / `## DISCOVERED` sections per the deterministic-seeded-then-dynamic dispatcher model (per `decisions.md` §6).

## 1. SD-24 — 35 criteria across 8 epics on `tranche/5-2`

SD-24 ships eight epics, each with its own capability slice. Code-Side Identifier Cleanup (Epic 1) fires FIRST on shared files; Closure Epilogue (Epic 8) fires LAST, only after Epic 7 (Closure Readiness) has dispatched it. Per operator directives 2026-07-21, SD-24 owns (a) **beta-readiness remediation** of unwired functions, unimplemented logic, and stubbed affordances across the whole codebase, (b) **per-class coverage audit + remediation** for APG/ACG ingests, verifying SD-22's class wiring was real and complete, (c) **multiclass stacking real and full** scoped to Fighter + Wizard to 10 (per operator's "not every combination" directive), (d) **equipment/armor/spells content completion** with strict 100% field coverage (cost + weight + full description) for the full PF1 core rules + APG + ACG + Bestiary 1 corpus, (e) **Tauri command-surface repair** for iterative character mutation (load/append/recompute/re-save, addressing the SD-23-handed-off `apps/desktop/src-tauri/src/.../characterHub.ts` loadout hardcoding), and (f) **unwired user-facing workflow cleanup** (Add Weapon/Armor/Spells onClick, picker modal flows).

### 1.1 Epic 1 — Code-Side Identifier Cleanup (governance base requirement; fires FIRST)

Under the identifier-discipline doctrine (`/home/ubuntu/workspace/repos/codex/governance/identifier-discipline.md` and skill `identifier-discipline`), source-code identifiers must describe what the artifact does, not which release or spec domain it came from. Epic 1 audits source for `sd22_*` / `Sd22` / `SD-24-Ex...` / `t_<hex>` leaks, removes them, and seeds the working tree for subsequent epics. Post-SD-23 cleanup, scope is defensive — most identifier leaks were caught in SD-21 Epic 1 + SD-22 Epic 1; SD-24 Epic 1 handles any straggler leaks from SD-22 / SD-23 cycles plus defensive cleanup of any `sd24_*` patterns the loop introduces. Detailed acceptance criteria 1-2 in `./epic-breakdown.md §Epic 1`.

### 1.2 Epic 2 — Operator Pre-Launch (board-exists, branch-pushed, OAuth-credentials-pinned)

SD-24's gating epic. Four pre-flight checks the operator runs once before the loop's first cycle: (1) `codex-tranche-5` kanban board reachable; (2) `tranche/5-2` branch pushed to origin; (3) SD-23 closure PR merged to develop (per the canonical `tranche/5-1 → develop` PR per duracon 2026-07-21 09:24:59; this is the launch-gate dependency the loop cannot self-heal); (4) clean working tree on `tranche/5-2`. Detailed acceptance criteria 3-5 in `./epic-breakdown.md §Epic 2`.

### 1.3 Epic 3 — Wired-Integration Audit + Remediation (the operator's "loose ends" sweep)

Per operator directives 2026-07-21: even though SD-23 eliminated stubs, the operator "expects more defects related to unwired functions, unimplemented logic, and stubbed out things." Epic 3 runs the four-check audit (from skill `wired-integration-discipline`) read-only across the entire codebase, generates a remediation backlog, then remediates. The audit covers (a) forbidden tokens in shipping code, (b) empty event handlers on user-facing affordances, (c) mock-library leaks from test files into shipping modules, (d) "Would …" return strings. Output: `artifacts/epic_3/wired-integration-audit.md` (the audit findings) + receiver cycles that fix each finding. All new entries to the Stubs Registry (only when the operator grants an exception to a discovered stub) land at `/home/ubuntu/workspace/repos/codex/governance/wired-integration-stubs-registry.md`. Detailed acceptance criteria 6-9 in `./epic-breakdown.md §Epic 3`.

### 1.4 Epic 4 — Per-Class Coverage Audit + Plan

Per operator's skepticism on SD-22 throughput: "it should have taken longer than it did unless you've managed major throughput improvements in our ingest process." Epic 4 audits per-class feature-table coverage for every class in CRB (Fighter + Wizard + the rest), APG (Alchemist, Cavalier, Inquisitor, Oracle, Summoner, Witch), and ACG (Alchemist-side, Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Swashbuckler, Warpriest). For each class: count `class_features_wired` vs `class_features_expected` from canonical PF1 sources. Output: `artifacts/epic_4/per-class-coverage-matrix.md` (matrix) + remediation plan. Detailed acceptance criteria 10-14 in `./epic-breakdown.md §Epic 4`.

### 1.5 Epic 5 — Multiclass Stacking Real and Full (Fighter + Wizard to 10)

Per operator directives 2026-07-21 ("we should only test multiclass on fully wired classes. … I suggest using fighter + wizard to mix combat and spells. Walk through advancing each of them up to level 10"): multiclass real-and-full scoped to Fighter + Wizard only, advancing both characters up to level 10. SD-21 Epic 7 shipped Fighter-only compute (per duracon 2026-07-18 20:17:52; the `src/rules_core/pilot_compute.rs:4568` dispatch refactor was scoped to Fighter); SD-24 Epic 5 extends the dispatch to support Fighter + Wizard multiclass with PF1's correct best-fractional-progression save-rule and BAB stacking. Test surface: 30 characters worth of advancement cycles (level 1 → 10 for each class, plus the split-class combination), deterministic tests against canonical PF1 examples, integration test consuming ingested content. Multiclass on APG/ACG classes is **explicitly out of scope** for SD-24 — gated on Epic 4's coverage report. Detailed acceptance criteria 15-19 in `./epic-breakdown.md §Epic 5`.

### 1.6 Epic 6 — Equipment/Armor/Spells Content Completion (strict 100% field coverage)

Per operator directives 2026-07-21 ("I also suspect that a lot of information is missing from equipment, armor and spells. last i looked there was a lot of missing information like cost, weight, etc. Full descriptions need to be present as well for all of these things … we should only test multiclass on fully wired classes … full PF1 core rules + APG + ACG + Bestiary 1, all content, no stubs, nothing left unwired"): the *strict* equipment-corpus scope. Every equipment item, armor piece, weapon, and spell in the beta-tier corpus (full PF1 core rules + APG + ACG + Bestiary 1) must carry cost (where applicable, e.g. equipment that has a gp cost), weight (where applicable, e.g. physical items), and a full description (every spell: full text per the SRD/PRD; every item: full description per the source). The cycle produces `artifacts/epic_6/equipment-coverage-matrix.md` (audit) → `artifacts/epic_6/content-completion-log.md` (per-cycle remediation log). Closure requires 100% field coverage against the beta-tier corpus definition (no P0 missing fields; remediation backlog of P1/P2 fields must be empty before closure). Detailed acceptance criteria 20-24 in `./epic-breakdown.md §Epic 6`.

### 1.7 Epic 7 — Unwired User-Facing Workflows + Tauri Command-Surface Repair

Two intertwined work streams in this epic:

1. **Tauri command-surface repair.** Per duracon 2026-07-18 18:20:41 + 2026-07-20 09:24:59: `create_character` / `list_saved_characters` / `load_saved_character` only compose-and-save once; they cannot load an existing character, append, recompute, or re-save. SD-24 Epic 7 adds the missing command surface: `appendToCharacter`, `recomputeCharacter`, `reSaveCharacter`, ensuring iterative character mutation works end-to-end. The current single-write-only flow's loadout hardcoding (weapon list, armor list, spell list — all defaulted instead of read from real corpus data) is fixed in this work.

2. **Unwired user-facing workflows.** Add Weapon / Add Armor / Add Spell onClick handlers wired to real corpus data (per the four-check audit). Picker modals flow through real corpus queries, not fixture-only arrays. Real Tauri commands on the mutation path.

Output: `artifacts/epic_7/tauri-command-surface.md` (command-surface audit + per-command receipts). Detailed acceptance criteria 25-28 in `./epic-breakdown.md §Epic 7`.

### 1.8 Epic 8 — Closure Epilogue (final scan + PR + worktree cleanup + release notes + version increment; fires LAST)

The standard part-of-handoff doctrine per spec-domain-lifecycle for every spec-domain closure. Epic 8 fires LAST in SD-24's cycle-ordering. Its cycle scans every prior criterion (1-30) for `complete` or `## Open blockers` status; opens the `tranche/5-2 → develop` closure PR via `gh pr create`; cleans up worktrees and stale branches; generates release notes under `./release-notes.md`; runs the closure test suite as the final gate; and runs the architecture-truth-up + graphify-update closure-pipeline sub-steps from the template's §6 architecture-docs closure obligation. The version-increment logic carries the SD-22 / SD-23 build-version scheme: `<major>.<tranche-base>.<build>` where `major=0` until first main-publish, `tranche-base=5` (tranche/5-2 carries base 5), `build=<monotonic-counter never-resets>`. Detailed acceptance criteria 29-30 in `./epic-breakdown.md §Epic 8`.

## 2. Cycle dispatch model — deterministic-seeded-then-dynamic

Per operator directive 2026-07-21: "we should … design this to be fairly open ended this time so that the loop is free to keep driving it forward as much as possible … Each loop should pick up and run as soon as the previous finishes … If new things are discovered that need to be done during a loop's run, it needs to document them in the loops to-do/progress documents."

The cycle picker walks the deterministic epic/criterion list during cycles 1-N (until `## TODO` has been meaningfully seeded); as soon as a cycle discovers work outside the deterministic list (e.g. Epic 4 finds APG Wizard-class `class_features_wired` count is below threshold; Epic 6 finds 12 armor pieces missing `weight`; Epic 7 finds `appendToCharacter` needs three additional commands), the discovery lands in `## DISCOVERED` with a priority bump relative to the in-flight criterion. The next cycle reads `## TODO` + `## DISCOVERED` together and dispatches the highest-priority unclaimed item. The seed list is the rules-of-engagement; the dispatcher queue is the live state.

Claude Code owns the looping and dispatching (per operator directive 2026-07-21). The loop-instruction file is the operator-edited boot-and-maintain manual; `/loop 1m /batch /goal ./loop-instruction.md` is the operational driver; kanban is receipt-only.

## 3. Bundle at a glance

- **Slug:** `SD-24-beta-readiness-and-multiclass`
- **Branch:** `tranche/5-2` (operator directive 2026-07-21; another agent creates the branch)
- **Board:** `codex-tranche-5` (reused; same operational board as SD-22; SD-24 explicit per the same `--board` flag in cycle mints)
- **Epics:** 8 / **Acceptance criteria:** 35 / **Closure gates:** 17 (per `./acceptance-and-verification.md`)
- **Tranche base:** 5 (per `<major>.<tranche-base>.<build>` scheme; same as `tranche/5`; SD-24's first concrete value is `0.5.<current_build_at_launch>`)
- **Multiclass test scope:** Fighter + Wizard only, advancing to level 10. APG/ACG-class multiclass is out of scope.
- **Equipment corpus scope:** strict. Full PF1 core rules + APG + ACG + Bestiary 1, all content, no stubs.
- **Loop launch form:** `/loop 1m /batch /goal ./loop-instruction.md`
- **Cycle dispatch model:** deterministic-seeded-then-dynamic
- **Cycle cadence:** 1m tick; 35-criterion deterministic seed + dynamic `## DISCOVERED` entries
- **Publish mode:** move-not-copy from the workspace-side planning surface to `docs/release/SD-24-beta-readiness-and-multiclass/` on `tranche/5-2`; the workspace-side copy is deleted on the publish commit (this signal is the "released to DEV" notification). Per operator directive 2026-07-21 ("we plan in workspace, we execute in the repo. We do not have more than one copy").

## 4. Files in this folder

| File | Purpose | Owner |
|---|---|---|
| `README.md` | Bundle index + bundle-at-a-glance + cross-references | operator (loop creates on first cycle from template) |
| `scope-draft.md` | This file. Canonical handoff *what* — bundle intent, epics, criteria, cycle dispatch model | operator |
| `loop-instruction.md` | Operational cycle mechanics — what the loop runs each tick | operator |
| `progress.md` | Live cycle-by-cycle progress + `## TODO` / `## DONE` / `## DISCOVERED` dispatcher state | loop (created on first cycle) |
| `decisions.md` | Bundle-specific ADRs (deterministic-then-dynamic dispatcher, multiclass scope, equipment scope, build counter inheritance, strict-vs-tolerant selection, etc.) | operator |
| `risks-and-open-questions.md` | Self-healable vs. non-self-healable split; open override flags; deferred judgments | operator |
| `acceptance-and-verification.md` | Closure gates + verification commands + per-criterion artifact map | operator |
| `content-unit-inventory.md` | Per-content-unit N-tuple (rust module / test fixture / cycle artifact / CommandName-or-ComponentName) | operator |
| `epic-breakdown.md` | 8 epics / 35 acceptance criteria / per-cycle story | operator |
| `technical-design.md` | Architectural surface; engine/API shapes; Tauri command-surface repair scope; equipment-corpus delivery pattern | operator |
| `technical-requirements.md` | Pre-loop prerequisites + normative requirements | operator |
| `release-notes.md` | Generated at Epic 8 (placeholder; loop populates) | loop |
| `artifacts/` | Per-cycle evidence: cycle receipts, audit outputs, coverage matrices, content-completion logs | loop (populated per cycle) |
| `artifacts/README.md` | Cycle-artifacts index (Epic-N subdirectories + closure-readiness-report) | operator-authored at package-construction time |
| `references/` | Doctrine pointers, skill pointers, sibling bundle pointers | operator |
| `references/README.md` | Doctrine / skill / sibling-bundle reference index | operator |

## 5. Cross-references

- `../../governance/no-stub-mvp-doctrine.md` — REPO-LOCAL CANONICAL wired-integration parent doctrine
- `../../governance/identifier-discipline.md` — REPO-LOCAL CANONICAL identifier-discipline doctrine
- `../../governance/wired-integration-stubs-registry.md` — REPO-LOCAL CANONICAL stubs registry (operator-granted exceptions)
- `../../governance/spec-domain-lifecycle.md` — REPO-LOCAL CANONICAL spec-domain lifecycle routing
- `../../architecture/` — repo-local architecture docs (closed by Epic 8's architecture-truth-up sub-step)
- `../SD-22/` — predecessor bundle (content-source ingest + DM toolkit; its classes feed SD-24 Epic 5 multiclass and Epic 4 coverage audit)
- `../SD-23-character-mutation-and-wired-integration/` — active bundle on tranche/5-1 (SD-24's Tier-1 Epic 7 launch-gate dependency: SD-23 closure PR merged to develop)
- `../SD-21-campaign-manager-and-persistence/decisions.md §18` — the operator's `<major>.<tranche-base>.<build>` build-version amendment

## 6. Hard-stop conditions

- A slice branch has diverged from `tranche/5-2` in a way that needs a manual rebase.
- The progress doc and the live matrix disagree on a row's `evidence_tier` and the disagreement is not just a stale snapshot.
- `cargo test --tests` regresses on a row other than the one the cycle touched. Sibling-preservation is a hard rule.
- Two live `claude` processes are working on cycles that would both touch the same file set under the file-touch partition.
- SD-23 closure PR has not yet merged to develop. SD-24's Tier-1 launch-gate dependency.
- The `## DISCOVERED` queue grows past 10 entries; operator override to clear or prioritize is required before the cycle continues.
