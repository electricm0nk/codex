# SD-23 — Character Mutation and Wired Integration

> **⚠️ OPERATING METHOD — REQUIRED FOR THIS BUNDLE**
>
> This bundle is operated via `/loop 60m /batch /goal programs/codex/requirements/SD-23-character-mutation-and-wired-integration/loop-instruction.md`, **not** via ad-hoc single-task invocations.
>
> `/loop 60m` is the cadence — every 60 minutes a new supervisor tick fires.
> `/batch` is the supervisor's concurrency primitive; the file-touch partition in the loop-instruction enforces one cycle per file at a time.
> `/goal <loop-instruction.md>` is the bundle's authoritative instruction file — the loop does not look anywhere else for cycle mechanics.
>
> A coding harness picking up this bundle after exiting plan mode reads this callout before reaching any other prose. Do not run cycles manually. Do not skip the loop.

## Bundle at a glance

- **Slug:** `SD-23-character-mutation-and-wired-integration`
- **Branch:** `tranche/5-1` (dash release from `tranche/5`, where SD-22 is active)
- **Board:** `codex-tranche-5` (reused after SD-22 closure PR lands)
- **Epics:** 7 / **Acceptance criteria:** 33 / **Closure gates:** 16
- **Tranche base:** 5 (per `<major>.<tranche-base>.<build>` scheme; same as `tranche/5`)
- **First concrete build value:** `0.5.<current_build_at_SD22_closure_merge>` — captured in `decisions.md` §"Build counter inheritance"
- **Doctrine under construction:** `../../governance/no-stub-mvp-doctrine.md` (active 2026-07-20)

## File map

| File | Purpose |
| --- | --- |
| `scope-draft.md` | Canonical scope of record for the bundle. Read first. |
| `loop-instruction.md` | Operational cycle mechanics — what the loop runs each tick. |
| `epic-breakdown.md` | 7 epics / 33 acceptance criteria / per-cycle story. |
| `decisions.md` | Decision log; build counter inheritance, doctrine refs, exception handling. |
| `risks-and-open-questions.md` | Latent risks (DB-tier convergence, orphan-reference, stat-field promotion) + open questions. |
| `acceptance-and-verification.md` | Test-surface contract, per-criterion artifact map, and the per-bundle closure-gate list. |
| `content-unit-inventory.md` | Per-content-unit N-tuple: rust module / test fixture / cycle artifact / CommandName-or-ComponentName. Mirrors SD-22's `corpus-source-inventory.md`. |
| `progress.md` | Cycle log — append-only, captures commit SHAs and kanban card ids per cycle. |
| `artifacts/` | Per-cycle receipt artifacts (Epic 3-7), README, and the closure-readiness-report. Mirrors SD-22's `artifacts/` convention. |
| `references/` | External reference docs cited by the package. Doctrine pointers, skill pointers, predecessor bundles, deferred research. |

## Out of scope (deferred to future bundles)

- **Database / storage-tiers convergence.** Documented at `programs/codex/research/storage-tiers-convergence-2026-07-20.md`. Operator's 2026-07-20 ruling: Option A (minimal file-store fix) for this bundle; structural convergence deferred.
- **Stat-field promotion for added equipment/spells.** SD-22's generated tables carry identity fields only; richer mechanical fields (weapon damage dice, AC bonus, spell range/duration/save) require a corpus → generated-table refactor outside this bundle's scope. Picker adds landing items without stat effects; future bundle promotes the mechanical fields.
- **Auto-granting spells/feats at level-up.** The `level_up_character` command takes the level but does not choose specific known spells or bonus feats. Per `level-up persistence brief 2026-07-20`: out of scope for SD-23.

## Cross-references

- `../../governance/no-stub-mvp-doctrine.md` — parent doctrine
- `../../governance/identifier-discipline.md` — sibling doctrine
- `../../governance/wired-integration-stubs-registry.md` — operator-granted stub exceptions
- `../../research/storage-tiers-convergence-2026-07-20.md` — deferred storage-tiers decision
- `../SD-21-campaign-manager-and-persistence/` — predecessor bundle, campaign manager / Drive persistence
- `../SD-22-content-source-ingest-and-dm-toolkit/` — active bundle on `tranche/5`
