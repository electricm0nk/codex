---
title: SD-34 — Book Completion — Release Package
status: planning
bundle_id: SD-34
slug: book-completion
scope: docs/release/SD-34-book-completion
artifact_type: release-index
canonical_branch: tranche/14
kanban_board: local-file ./kanban.md
target_version: 0.14.0
canonical_source: docs/release/SD-34-book-completion (this folder)
date: 2026-08-26
---

# SD-34 — Book Completion — Release Package

> ## WARNING — OPERATING METHOD — REQUIRED FOR THIS BUNDLE
>
> **This bundle is operated via a `Workflow`-tool script, invoked from a live session — NOT `/loop /batch` and NOT a one-shot task.** For how to **create** that script — phase structure, tiering, worked skeleton — see `workflow-instruction.md §2.4`. The full per-cycle procedure, orchestration mode, concurrency map, dual-audit gate, retro-event-logging discipline, and epic/bundle closure steps live in `workflow-instruction.md`'s body, authored from `../../governance/workflow-instruction-template.md`. The scope-draft ([`./scope-draft.md`](./scope-draft.md)) is the canonical handoff *what*; the workflow-instruction is the *how*.

This folder is the canonical surface for SD-34.

## 0. Preamble

The bundle's intent, scope, and acceptance-evidence obligations live in [`scope-draft.md`](./scope-draft.md). The per-cycle launch form, eligibility checks, and self-heal mechanics live in [`workflow-instruction.md`](./workflow-instruction.md). This README is the index.

## 1. Bundle snapshot

| Field | Value |
|---|---|
| Bundle ID | SD-34 |
| Slug | `book-completion` |
| Canonical branch | `tranche/14` — **not yet cut**. SD-33's closure PR #377 **merged 2026-08-27** (`ea2b3396f2`), so the cut is unblocked; it happens as `workflow-instruction.md §1` item 8 (`decisions.md §11`) |
| Kanban board | local-file `./kanban.md` (Hermes retired 2026-08-01) |
| Epics / criteria | 6 / 27 (26 kanban rows) |
| Target version | `0.14.0`, stamped at the `tranche/14` cut |
| Dispatch mechanism | `Workflow` tool, invoked from a live session, per `workflow-instruction.md §2` |
| Cadence | N/A — dispatch is a live `Workflow` session, not a timer loop |
| Closure gate | `tranche/14 → develop` PR; retrospective written + cited; worktree/branch sweep; release notes; architecture-docs refresh (§6) — full sequence in `workflow-instruction.md §11` |
| Launch state | **planning-ready, NOT launch-ready.** Audited 2026-08-27 with `stc-authoring`. Tier-1 item 1 (SD-33 PR merged) is satisfied; `workflow-instruction.md §1`'s outputs are not yet pasted; items 2, 3, 8 run at the cut. |

## 2. What this bundle is

SD-34 produces **one exhaustive, mechanically-derived statement of every step that remains for every one of the 49,438 units in the corpus** — and proves it by driving two books of opposite shape to zero remaining steps.

It is **not** a book-count bundle. The operator named the problem directly: *"I need to know what is left. everything I think we are done, you surface 3 more things. that stops with sd-34."* That is a closure-completeness problem, and the cure is a fail-closed partition where every unit lands in exactly one named bucket and `unclassified` is a hard error.

**SD-33 delivered full ingestion and the shape engines.** SD-34 uses them to establish what remains, and prices it. `decisions.md §1`, `§2`, `§2a`, `§2b` carry the reasoning and the two facts that reframed this package: ingestion is complete, and a shape engine computes a number without completing a record.

**Primary deliverable (S1):** the Completion Atlas — 49,438 units, ten buckets, `unclassified=0`.
**Proof (S2):** two books of opposite shape at zero remaining steps — the Core Rulebook (6,701 of 6,701, deep, every bucket) and Ultimate Campaign (265 of 265, shallow, effectively one bucket).
**Forward plan (S3):** the non-DONE units across the other 35 books, priced per bucket per book from rates measured in S2, with the one unbuilt table (`power`) costed individually.


## 3. Files in this folder

| File | Job | Owner |
|---|---|---|
| `scope-draft.md` | Canonical handoff *what* — intent, measured baseline, epics, success | operator |
| `workflow-instruction.md` | Per-cycle launch *how* — eligibility, dispatch, self-heal, receipt schema | operator |
| `epic-breakdown.md` | The 27 acceptance criteria across 6 epics | operator |
| `decisions.md` | Bundle-specific ADRs | operator |
| `kanban.md` | One row per criterion; the board | loop (updated per cycle) |
| `progress.md` | Live cycle-by-cycle record + status | loop (created on first cycle) |
| `technical-requirements.md` | Pre-loop prerequisites + normative requirements | operator |
| `technical-design.md` | Instruments, engine surfaces, the done-ledger shape | operator |
| `acceptance-and-verification.md` | Closure gates + verification commands + per-criterion artifact map | operator |
| `risks-and-open-questions.md` | Self-healable vs non-self-healable; open flags | operator |
| `content-unit-inventory.md` | The measured baseline and every re-derive command behind it | operator |
| `forward-scope-register.md` | Successor work depending on this package's output | operator |
| `receipts.md` | Closure-pipeline receipts (architecture-truth-up, graphify, PR) | loop |
| `release-notes.md` | Written at closure | loop |
| `artifacts/` | Per-epic evidence and cycle receipts | loop |
| `artifacts/README.md` | Cycle-artifacts index | operator |
| `references/` | Doctrine, skill, sibling-bundle pointers | operator |
| `references/README.md` | Reference index | operator |

## 4. In-repo cross-references

- **Predecessor** — `../SD-33-computed-value-verification/`. Its shape engines (`formula_interpreter`), oracle harness, `box_ledger.py`, and denominator gate are SD-34's starting instruments.
- **Doctrine mirrors** — `../../doctrine-external/identifier-discipline.md`, `../../governance/no-stub-mvp-doctrine.md`, `../../governance/blocker-closure-doctrine.md`, `../../governance/deferral-revisit-doctrine.md`.
- **Architecture docs** — `../../architecture/` (the closure epilogue §6 obligation re-verifies every touched topic).
- **Conduct surface** — `../../../AGENTS.md`, `../../../CLAUDE.md`.

## 5. Build version target

`0.14.0`:

- **major** — `0` until first publish to `main`.
- **tranche-base** — `14`, the base digit of `tranche/14`.
- **build** — `0` at the cut.

The tranche digit moves on a **new `tranche/N` branch cut**, never on a bundle's own closure (`decisions.md §11`). Root `Cargo.toml` stays pinned at `0.1.0` and is not the version source of truth.

## 6. Architecture-docs, graphify, and PR closure obligation

Identical in shape to every prior bundle; the canonical procedure is `../template/template.md §6`, and its procedural half (retrospective + sweep, which land **before** the PR) is `workflow-instruction.md §11`. Do not duplicate either half into the other file.

**Step 1 is a hard gate and a filed blocker does not satisfy it.** If anything is short, the closure epilogue stops: no retrospective, no sweep, **no PR**.

## 7. Required canonical files

Per `../template/template.md §7`, all present in this folder: `README.md`, `scope-draft.md`, `workflow-instruction.md`, `progress.md`, `epic-breakdown.md`, `decisions.md`, `risks-and-open-questions.md`, `acceptance-and-verification.md`, `content-unit-inventory.md`, `artifacts/`, `artifacts/README.md`, `references/`, `references/README.md` — plus `technical-requirements.md`, `technical-design.md`, `kanban.md`, `forward-scope-register.md`, `receipts.md`, `release-notes.md` per current sibling convention.

## 8. Provenance of every number in this package

Every population figure was measured on 2026-08-27 against `origin/develop` at `ea2b3396f2` — the merge commit of SD-33's closure PR #377 — from `docs/work-inventory.json`. `content-unit-inventory.md` carries the re-derive command for each. An earlier draft measured at the parent of SD-33's final fold; the fold moved four figures, and every number was re-run (`content-unit-inventory.md §0`).

**They are not provisional.** The load-bearing one — that ingestion is complete — was proved directly: 26,002 of 26,002 units (100%) whose status field reads `not-ingested` carry a real `source_file` and `source_line`, and every one of that field's evidence strings is engine-side. This package's first draft reported "52.7% not ingested" in error; `decisions.md §2b` records the correction and AT-34-E1-005 renames the field.


## 9. Launch gates

`workflow-instruction.md §1`, items 1–12. **Unrun.** Tier-1 item 1 — SD-33's closure PR merged to `develop` — is satisfied (#377, `ea2b3396f2`); items 2, 3, and 8 pass at the `tranche/14` cut; item 12 (the denominator gate pointed at this package) is new.
