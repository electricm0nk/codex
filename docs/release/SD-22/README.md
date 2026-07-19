---
title: SD-22 — Content-Source Ingest (APG + ACG + Bestiary 1) + DM Toolkit + Closure Readiness — Release Package
status: planning (operator directive 2026-07-18; bundle approved scope; pre-launch checklist pending; Epic 9 — Closure Readiness added 2026-07-19)
bundle_id: SD-22
slug: content-source-ingest-and-dm-toolkit
canonical_branch: tranche/5 (operator directive 2026-07-18; branch + board pinned; replaces dead-state codex-tranche-5 board repurposed from the 2026-07-16 SD-21 launch)
kanban_board: codex-tranche-5 (operator directive 2026-07-18)
target_version: 0.5.<current_build> (per operator's 2026-07-17 <major>.<tranche-base>.<build> amendment applied symmetrically to SD-22)
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/decisions.md
mirror_of: ~/workspace/SD-22-content-source-ingest-and-dm-toolkit-{scope-draft,loop-instruction}.md
date: 2026-07-15 (scope); 2026-07-17 (scope expansion: APG + ACG); 2026-07-18 (ACG/APG clarification + tranche/5 + codex-tranche-5 launch pin); 2026-07-19 (Epic 9 — Closure Readiness added)
epics: 9
criteria: 31
---

# SD-22 — Content-Source Ingest (APG + ACG + Bestiary 1) + DM Toolkit + Closure Readiness — Release Package

> ## ⚠️  OPERATING METHOD — REQUIRED FOR THIS BUNDLE  ⚠️
>
> **This bundle is operated via the `/loop 60m /goal` invocation model — NOT a one-shot task.** After exiting plan mode, the coding harness is **required** to launch SD-22 as:
>
> ```bash
> /loop 60m /goal ~/workspace/SD-22-content-source-ingest-and-dm-toolkit-loop-instruction.md
> ```
>
> `/batch` is deferred per operator directive 2026-07-18 — re-added only when ≥2 book corpora exist and the book lanes are genuinely parallel (see [`./decisions.md`](./decisions.md) §5).
>
> The full per-cycle procedure, file-touch partition, post-mortem card mint, and progress-doc update live in [`./loop-instruction.md`](./loop-instruction.md). The scope-draft ([`./scope-draft.md`](./scope-draft.md)) is the canonical handoff *what* — the loop-instruction is the *how*. See loop-instruction's leading `⚠️ OPERATING METHOD` block for the verbatim launch instruction and pre-launch checklist.

This folder is the repo-local surface for SD-22. SD-22 is the **planning-stage** ACTIVE bundle (operator directive 2026-07-18; pre-launch checklist pending; Epic 9 — Closure Readiness added 2026-07-19). It launches on `tranche/5` against the board `codex-tranche-5`. The progress doc [`./progress.md`](./progress.md) is forward-referenced — it's created on the loop's first cycle.

## 0. Preamble

The bundle's intent, scope, and acceptance-evidence obligations live in [`./scope-draft.md`](./scope-draft.md). The per-cycle launch form, eligibility checks, and self-heal mechanics live in [`./loop-instruction.md`](./loop-instruction.md).

## 1. Bundle snapshot

| Field | Value |
|---|---|
| Bundle ID | SD-22 |
| Slug | `content-source-ingest-and-dm-toolkit` |
| Canonical branch | `tranche/5` (operator directive 2026-07-18) |
| Kanban board | `codex-tranche-5` (operator directive 2026-07-18) |
| Epics / criteria | 9 epics / 31 criteria |
| Target version | `0.5.<current_build>` |
| Loop launch form | `/loop 60m /goal ~/workspace/SD-22-content-source-ingest-and-dm-toolkit-loop-instruction.md` |
| Cycle cadence | 60m restart; `/batch` deferred per `decisions.md` §5 |
| Closure gate | `tranche/5 → develop` PR; identifier-cleanup-delta; release-notes generation; Epic 9 must complete before Epic 7 fires |

## 2. Files in this folder

| File | Job | Owner |
|---|---|---|
| `scope-draft.md` | Canonical handoff *what* — bundle intent, 9 epics, 31 criteria | operator (mirror of upstream `scope-draft.md`) |
| `loop-instruction.md` | Per-cycle launch *how* — eligibility, self-heal, post-mortem schema | operator (mirror of upstream `loop-instruction.md`) |
| `progress.md` | Live cycle-by-cycle progress + status matrix | loop (created on first cycle; forward-referenced) |
| `receipts.md` | **Durable per-cycle receipt ledger — repo-resident fallback when kanban DB is unreachable (cloud runs)** | loop (appended every cycle; see `loop-instruction.md` Step 10a) |
| `decisions.md` | Repo-local ADRs (mirror of upstream src-STC decision record) | operator (mirror of upstream src-STC; §1/§2/§3 + §4 Epic 9 — Closure Readiness) |
| `technical-requirements.md` | Pre-loop prerequisites | operator (mirror of upstream src-STC) |
| `epic-breakdown.md` | Acceptance criteria 1-31 grouped across 9 epics (Epic 9 added 2026-07-19) | operator (mirror of upstream src-STC) |
| `acceptance-and-verification.md` | Closure gates 1-17 (gate 17 added 2026-07-19 for criterion-31) | operator (mirror of upstream src-STC) |
| `risks-and-open-questions.md` | Self-healable vs. non-self-healable split; "Open judgments deferred to next SD" parking lot | operator (mirror of upstream src-STC; new section 2026-07-19) |
| `technical-design.md` | Architectural surface; engine/API shapes; cross-book resolution patterns | operator (mirror of upstream src-STC) |
| `artifacts/` | Per-cycle evidence: parity fixtures, receipt comments, TRACI snapshots | loop (populated per cycle) |

## 3. Authoritative pointers (operator workspace; not bundled here)

- **Upstream strategic/intake authority:** `/home/ubuntu/workspace/programs/codex/requirements/SD-22-content-source-ingest-and-dm-toolkit/`.
- **Operator's editor-of-record scope-draft:** `~/workspace/SD-22-content-source-ingest-and-dm-toolkit-scope-draft.md`.
- **Operator's editor-of-record loop-instruction:** `~/workspace/SD-22-content-source-ingest-and-dm-toolkit-loop-instruction.md`.
- **Operator's live progress doc:** `~/workspace/SD-22-content-source-ingest-and-dm-toolkit-progress.md` (loop's claim-protocol target).

## 4. Relationship to other release folders

- **Sibling CLOSED bundles:** [`../SD-20/`](../SD-20/) (per-character rules engine; trunk closure 2026-07-17), [`../SD-19/`](../SD-19/) (corpus-aware compute seam), [`../SD-18/`](../SD-18/) (core-rules breadth).
- **Upstream ACTIVE bundle (closed):** [`../SD-21/`](../SD-21/) — SD-21 closed on `tranche/4-1` per operator directive; sister tranche per the doc-doctrine hierarchy. SD-21's Epic 2 (Campaign Manager + Drive) consumed the party-CR math SD-22's Epic 6 (DM Toolkit) provides.

## 5. Build version target

SD-22's release version triple is `<major>.<tranche-base>.<build>` (operator's 2026-07-17 amendment replacing the prior `0.0.X` patch-only scheme):

- **`major`** — `0` until first publish to `main`; increments by `1` per main-publish.
- **`tranche-base`** — base digit of the active tranche branch. SD-22's `tranche/5` carries `5`.
- **`build`** — monotonic counter across all builds across all branches (never resets); increments per merge.

SD-22's first concrete value is **`0.5.<current_build>`** at cycle launch. Epic 8's cycles touch the version fields; Epic 9 (Closure Readiness) dispatches Epic 7 to open the `tranche/5 → develop` PR after all 30 of criteria 1-30 are artifact-verified. Epic 7's PR bumps the tranche position on promotion (`0.5.<last_build>` → `0.6.0` for the next tranche).
