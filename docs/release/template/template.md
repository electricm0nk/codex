---
title: SD-NN — <Bundle Title> — Release Package
status: <planning | running | closed>
bundle_id: SD-NN
scope: docs/release/SD-NN
artifact_type: release-index
canonical_branch: <tranche/N>
kanban_board: <codex-tranche-N>
target_version: <major>.<tranche-base>.<build>
companion_to: ../../../../programs/codex/requirements/SD-NN-<slug>/decisions.md
mirror_of: ~/workspace/SD-NN-<slug>-{scope-draft,loop-instruction,progress}.md
date: <ISO-8601>
---

# SD-NN — <Bundle Title> — Release Package

> ## ⚠️  OPERATING METHOD — REQUIRED FOR THIS BUNDLE  ⚠️
>
> **This bundle is operated via the `/loop 60m /batch /goal` invocation model — NOT a one-shot task.** After exiting plan mode, the coding harness is **required** to launch SD-NN as:
>
> ```bash
> /loop 60m /batch /goal ~/workspace/SD-NN-<slug>-loop-instruction.md
> ```
>
> The full per-cycle procedure, file-touch partition, post-mortem card mint, and progress-doc update live in the loop-instruction file body. The scope-draft is the canonical handoff *what* — the loop-instruction is the *how*. See `loop-instruction.md`'s leading `⚠️ OPERATING METHOD` block for the verbatim launch instruction and pre-launch checklist.

This folder is the repo-local surface for SD-NN. The strategic/intake authority (`/home/workspace/programs/codex/requirements/SD-NN-<slug>/`) lives upstream in the operator's workspace. A cloud-harness cold clone of the repo reads this folder to find SD-NN's handoff without `~/workspace/` access.

## 0. Preamble

The bundle's intent, scope, and acceptance-evidence obligations live in `scope-draft.md` (this folder). The per-cycle launch form, eligibility checks, and self-heal mechanics live in `loop-instruction.md`. This README is the index.

## 1. Bundle snapshot

| Field | Value |
|---|---|
| Bundle ID | SD-NN |
| Slug | `<slug>` |
| Canonical branch | `<tranche/N>` (operator directive <date>) |
| Kanban board | `<codex-tranche-N>` (operator directive <date>) |
| Epics / criteria | `<N epics>` / `<N criteria>` |
| Target version | `<major>.<tranche-base>.<build>` |
| Loop launch form | `/loop 60m /batch /goal ~/workspace/SD-NN-<slug>-loop-instruction.md` |
| Cycle cadence | 60m restart; `/batch` for shared-file concurrency |
| Closure gate | `tranche/N → develop` PR; docs/epic-1-cleanup-delta; release-notes generation |

## 2. Files in this folder

| File | Job | Owner |
|---|---|---|
| `scope-draft.md` | Canonical handoff *what* — bundle intent, epics, criteria | operator (mirror of upstream `scope-draft.md`) |
| `loop-instruction.md` | Per-cycle launch *how* — eligibility, self-heal, post-mortem schema | operator (mirror of upstream `loop-instruction.md`) |
| `progress.md` | Live cycle-by-cycle progress + status matrix | loop (created on first cycle; not present in planning) |
| `decisions.md` | Repo-local ADRs (vs. upstream `programs/codex/doctrine/decisions/` which is program-level) | operator (mirror of upstream src-STC decision record) |
| `technical-requirements.md` | Pre-loop prerequisites + normative requirements | operator (mirror of upstream src-STC) |
| `epic-breakdown.md` | Acceptance criteria 1-N grouped across epics | operator (mirror of upstream src-STC) |
| `acceptance-and-verification.md` | Closure gates + verification commands | operator (mirror of upstream src-STC) |
| `risks-and-open-questions.md` | Self-healable vs. non-self-healable split; open override flags | operator (mirror of upstream src-STC) |
| `technical-design.md` | Architectural surface; engine/API shapes; cross-book resolution patterns | operator (mirror of upstream src-STC) |
| `artifacts/` | Per-cycle evidence: parity fixtures, receipt comments, TRACI snapshots | loop (populated per cycle) |

## 3. Authoritative pointers (operator workspace; not bundled here)

- **Upstream strategic/intake authority:** `/home/workspace/programs/codex/requirements/SD-NN-<slug>/` (operator's workspace; required reading).
- **Operator's editor-of-record loop-instruction:** `~/workspace/SD-NN-<slug>-loop-instruction.md` (mirror of `loop-instruction.md`; launch string).
- **Operator's editor-of-record scope-draft:** `~/workspace/SD-NN-<slug>-scope-draft.md` (mirror of `scope-draft.md`; canonical handoff).
- **Operator's live progress doc:** `~/workspace/SD-NN-<slug>-progress.md` (mirror of `progress.md`; the loop's claim protocol routes here).

## 4. Relationship to other release folders

- **Sibling bundles** — `../SD-MM/` for any other in-flight or historical bundle.
- **Tranche branch posture** — this bundle ships on `<tranche/N>`.

## 5. Build version target

For SD-NN, the release version triple is `<major>.<tranche-base>.<build>`:

- **`major`** — `0` until first publish to `main`; increments by `1` per main-publish.
- **`tranche-base`** — base digit of the active tranche branch.
- **`build`** — monotonic counter across all builds across all branches (never resets).

SD-NN's first concrete value is `<major>.<tranche-base>.<current_build_at_launch>`.
