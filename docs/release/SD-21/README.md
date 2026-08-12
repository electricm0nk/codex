---
title: SD-21 — Campaign Manager + Drive + Multiclass Support + Identifier Cleanup + Update UI Bug + Build Versioning + Closure Epilogue — Release Package
status: closed (operator directive 2026-07-17; bundle CLOSED on tranche/4-1; progress doc snapshot as of 9206ad0)
bundle_id: SD-21
slug: campaign-manager-and-persistence
canonical_branch: tranche/4-1 (operator directive 2026-07-17; slash-form dash release following SD-20's tranche/4)
kanban_board: codex-tranche-4-1 (operator directive 2026-07-17)
target_version: 0.4.<current_build> (per operator's 2026-07-17 <major>.<tranche-base>.<build> amendment)
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-21-campaign-manager-and-persistence/decisions.md
mirror_of: ~/workspace/SD-21-campaign-manager-and-persistence-{scope-draft,loop-instruction,progress}.md
date: 2026-07-15 (scope); 2026-07-17 (slash-form tranche/4-1 launch pin)
epics: 7
criteria: 30
---

# SD-21 — Campaign Manager + Drive + Multiclass + Identifier Cleanup + Update UI Bug + Build Versioning + Closure Epilogue — Release Package (CLOSED)

> ## ⚠️  OPERATING METHOD — REQUIRED FOR THIS BUNDLE  ⚠️
>
> **This bundle is operated via the `/loop 60m /batch /goal` invocation model — NOT a one-shot task.** After exiting plan mode, the coding harness is **required** to launch SD-21 as:
>
> ```bash
> /loop 60m /batch /goal ~/workspace/SD-21-campaign-manager-and-persistence-loop-instruction.md
> ```
>
> The full per-cycle procedure, file-touch partition, post-mortem card mint, and progress-doc update live in [`./loop-instruction.md`](./loop-instruction.md). The scope-draft ([`./scope-draft.md`](./scope-draft.md)) is the canonical handoff *what*.

This folder is the repo-local surface for SD-21. **SD-21 is CLOSED.** Bundle ran to completion on `tranche/4-1` against `codex-tranche-4-1`. The progress doc [`./progress.md`](./progress.md) is the per-cycle record (snapshot as of `9206ad0`).

## 1. Bundle snapshot

| Field | Value |
|---|---|
| Bundle ID | SD-21 |
| Slug | `campaign-manager-and-persistence` |
| Canonical branch | `tranche/4-1` |
| Kanban board | `codex-tranche-4-1` |
| Epics / criteria | 7 epics / 30 criteria |
| Final version | `0.4.<last_build>` |

## 2. Files in this folder

| File | Job |
|---|---|
| `scope-draft.md` | Canonical handoff *what* (mirror) |
| `loop-instruction.md` | Per-cycle launch *how* (mirror) |
| `progress.md` | Live cycle-by-cycle progress + status matrix (mirror; snapshot as of `9206ad0`) |
| `decisions.md` | Repo-local ADRs (mirror; ~21-item record) |
| `technical-requirements.md`, `epic-breakdown.md`, `acceptance-and-verification.md`, `risks-and-open-questions.md`, `technical-design.md` | Mirror of upstream src-STC pieces |

## 3. Authoritative pointers (operator workspace; not bundled here)

- **Upstream strategic/intake authority:** `/home/ubuntu/workspace/programs/codex/requirements/SD-21-campaign-manager-and-persistence/`.
- **Operator's editor-of-record scope-draft:** `~/workspace/SD-21-campaign-manager-and-persistence-scope-draft.md`.
- **Operator's editor-of-record loop-instruction:** `~/workspace/SD-21-campaign-manager-and-persistence-loop-instruction.md`.
- **Operator's live progress doc:** `~/workspace/SD-21-campaign-manager-and-persistence-progress.md`.

## 4. Relationship to other release folders

- **Downstream PLANNING bundle:** [`../SD-22/`](../SD-22/) — SD-22 ships on `tranche/5`; SD-22's Epic 6 (DM Toolkit) provides the party-CR math SD-21's Epic 2 (Campaign Manager + Drive) consumes.
- **Upstream CLOSED bundles:** [`../SD-20/`](../SD-20/) (per-character rules engine; trunk closure 2026-07-17), [`../SD-19/`](../SD-19/) (corpus-aware compute seam), [`../SD-18/`](../SD-18/) (core-rules breadth).
