# SD-24 — Beta Readiness + Multiclass Stacking + Equipment Completeness

> **⚠️ OPERATING METHOD — REQUIRED FOR THIS BUNDLE ⚠️**
>
> This bundle is operated via `/loop 1m /batch /goal ./loop-instruction.md`, **not** via ad-hoc single-task invocations.
>
> `/loop 1m` is the cadence — every 1 minute a new supervisor tick fires.
> `/batch` is the supervisor's concurrency primitive; the file-touch partition in the loop-instruction enforces one cycle per file at a time.
> `/goal <loop-instruction.md>` is the bundle's authoritative instruction file — the loop does not look anywhere else for cycle mechanics.
>
> Per operator directive 2026-07-21: **Claude Code handles looping and dispatching**. Kanban is the durable receipt layer only — cards are minted on `codex-tranche-5` only with a done receipt; kanban does NOT dispatch work.
>
> A coding harness picking up this bundle after exiting plan mode reads this callout before reaching any other prose. Do not run cycles manually. Do not skip the loop.

## Bundle at a glance

- **Slug:** `SD-24-beta-readiness-and-multiclass`
- **Branch:** `tranche/5-2` (operator directive 2026-07-21)
- **Board:** `codex-tranche-5` (reused; same operational board as SD-22)
- **Epics:** 8 / **Acceptance criteria:** 35
- **Multiclass scope:** Fighter + Wizard only, advancing each to level 10
- **Equipment scope:** strict 100% field coverage for full PF1 core rules + APG + ACG + Bestiary 1
- **Tranche base:** 5 (per `<major>.<tranche-base>.<build>` scheme; first concrete value `0.5.<build>`)
- **Loop launch form:** `/loop 1m /batch /goal ./loop-instruction.md`
- **Cycle dispatch model:** deterministic-seeded-then-dynamic (per `decisions.md §2`)
- **Cycle cadence:** 1m tick; 35-criterion deterministic seed + dynamic `## DISCOVERED` entries
- **Publish mode:** move-not-copy (workspace copy deleted on publish commit per operator directive 2026-07-21)
- **Doctrine under construction:** `~/.hermes/profiles/god-emporer/skills/devops/wired-integration-discipline/` and `~/.hermes/profiles/god-emporer/skills/devops/identifier-discipline/` (sibling doctrines, dual-audit gate per cycle; first bundled both into the dual-audit in SD-23)

## File map

| File | Purpose |
| --- | --- |
| `scope-draft.md` | Canonical scope of record. Read first. |
| `loop-instruction.md` | Operational cycle mechanics. |
| `epic-breakdown.md` | 8 epics / 35 acceptance criteria / per-cycle stories. |
| `decisions.md` | Decision log; deterministic-then-dynamic dispatcher, multiclass scope, equipment scope, publish mode, kanban-as-receipt. |
| `risks-and-open-questions.md` | Self-healable vs. non-self-healable split + override flags + deferrals. |
| `acceptance-and-verification.md` | Closure gates + verification commands + per-criterion artifact map. |
| `content-unit-inventory.md` | Per-content-unit N-tuple: rust module / test fixture / cycle artifact / source canonical. |
| `technical-design.md` | Architectural surface; engine/API shapes; multiclass dispatch; Tauri command-surface extension; equipment-corpus delivery. |
| `technical-requirements.md` | Pre-loop prerequisites + normative requirements + out-of-scope deferrals. |
| `progress.md` | Cycle log + `## TODO` + `## DONE` + `## DISCOVERED` + `## Status matrix` + `## Open blockers`. |
| `release-notes.md` | Generated at Epic 8's criterion 8.3 (placeholder). |
| `artifacts/` | Per-cycle receipt artifacts (Epic 1-8), plus dynamic artifacts (audit outputs, coverage matrices, content-completion logs). |
| `references/` | Doctrine pointers, skill pointers, sibling bundle pointers. |

## Cross-references

- `../../governance/no-stub-mvp-doctrine.md` — wired-integration parent doctrine
- `../../governance/identifier-discipline.md` — identifier-discipline sibling doctrine
- `../../governance/wired-integration-stubs-registry.md` — operator-granted stub exceptions
- `../SD-22/` — predecessor bundle, content-source ingest + DM toolkit
- `../SD-23-character-mutation-and-wired-integration/` — active bundle on `tranche/5-1` (SD-24's Tier-1 launch-gate dependency)
- `../SD-21-campaign-manager-and-persistence/decisions.md §18` — operator's 2026-07-17 `<major>.<tranche-base>.<build>` build-version amendment
