---
title: SD-35 — Corpus Sheet Completion — Release Package
status: planning-ready
bundle_id: SD-35
slug: corpus-sheet-completion
scope: docs/release/SD-35-corpus-sheet-completion
artifact_type: release-index
canonical_branch: tranche/15
kanban_board: local-file ./kanban.md
target_version: 0.15.0
canonical_source: docs/release/SD-35-corpus-sheet-completion (this folder)
date: 2026-09-07
---

# SD-35 — Corpus Sheet Completion — Release Package

> ## WARNING — OPERATING METHOD — REQUIRED FOR THIS BUNDLE
>
> **This bundle is operated via a `Workflow`-tool script, invoked from a live session — NOT `/loop /batch` and NOT a one-shot task.** For how to **create** that script — phase structure, tiering, worked skeleton — see `workflow-instruction.md §2.4`. The full per-cycle procedure, orchestration mode, concurrency map, dual-audit gate, retro-event-logging discipline, and epic/bundle closure steps live in `workflow-instruction.md`'s body, authored from `../../governance/workflow-instruction-template.md`. The scope-draft ([`./scope-draft.md`](./scope-draft.md)) is the canonical handoff *what*; the workflow-instruction is the *how*.

This folder is the canonical surface for SD-35.

## 0. Preamble

The bundle's intent, scope, and acceptance-evidence obligations live in [`scope-draft.md`](./scope-draft.md). The per-cycle launch form, eligibility checks, and self-heal mechanics live in [`workflow-instruction.md`](./workflow-instruction.md). This README is the index.

## 1. Bundle snapshot

| Field | Value |
|---|---|
| Bundle ID | SD-35 |
| Slug | `corpus-sheet-completion` |
| Canonical branch | `tranche/15` — **cut and pushed 2026-09-07** at `4c6c57eb9f` on `fe5ae6cd4a` (SD-34's PR #383 merge); `decisions.md §10` |
| Kanban board | local-file `./kanban.md` (Hermes retired 2026-08-01) |
| Epics / criteria | 7 / 30 (29 kanban rows) |
| Target version | `0.15.0`, stamped at the cut in `4c6c57eb9f` (both version files confirmed) |
| Dispatch mechanism | `Workflow` tool, invoked from a live session, per `workflow-instruction.md §2` |
| Cadence | N/A — dispatch is a live `Workflow` session, not a timer loop |
| Closure gate | `tranche/15 → develop` PR; retrospective written + cited; worktree/branch sweep; release notes; architecture-docs refresh (§6) — full sequence in `workflow-instruction.md §11` |
| Launch state | Authored 2026-09-07 morning with `stc-authoring` while SD-34 was at wave 51; **launch-readiness audit the same evening at the `tranche/15` cut** re-based the premise, fixed two script/citation defects and three path/scope mismatches, added AT-35-E1-006 (SD-34's closure folded — `decisions.md §12`), and re-ran `workflow-instruction.md §1` with outputs pasted. **planning-ready** as of 2026-09-08 00:10Z: every §1 item green at the cut (590 targets / 8,656 passed / 0 failed; desktop 573/0; sweep CLEAN; atlas `unclassified=0`). Launch = `/model opus`, then the §2.4 script with `args.laneModel='fable'`. |

## 2. What this bundle is

SD-35 drives **every remaining content unit in the corpus — 23,315 of 49,438, across all 37
books — to the state where the character sheet shows what the player would write on paper**:
a final number, a dice expression, or the rule's words.

The operator named the finish line (`decisions.md §1`): *"This isn't a video game, it's a
character sheet generator. [...] We should print out a final number. Not a number, plus another
number, plus another number. Do the math. But sometimes a number is that very addition. A sword
that does d8+2 damage - you just write that down."*

And the boundary (`decisions.md §11`): *"Using it to convert and test is fine. But when we are
done, there should be nothing left of pcgen."* PCGen is a converter input and a test oracle.
**Not one line of it on the live side at closure** — including the 78 files that read it today.

And the process (`decisions.md §2`, `§3`): one mechanism per cycle, corpus-wide, **500 units
minimum, enforced by a script with a nonzero exit**; one build per cycle; the full gate once
per epic.

**SD-34 proved the map** (the Completion Atlas, 49,438 units in ten fail-closed buckets) and
measured that cost scales with token vocabulary — 189 token types, and 28 of them cover 90% of 181,274 instances —
not with unit count (`../SD-34-book-completion/fable-review.md §1.b`).
**SD-35 builds two small things** — an ingest-time converter that writes our own rule records
from PCGen tokens, and a forty-line live evaluator for our own expression form
(`technical-design.md §1`–`§2`) — runs the converter corpus-wide until the atlas reads
`DONE=49438 of 49438`, then **removes the old run-time PCGen path from the live side** with the
oracle proving parity before and after (Epic 6).

**Primary deliverable (S1):** every unit DONE under the sheet rule, `unclassified=0`.
**Proof (S2):** one on-screen test per kind, 19 kinds.
**Boundary (S3):** `pcgen_residue_gate.py --check --closure` → zero; our data files carry no
PCGen; oracle parity agrees before and after the exit.
**Process (S4):** no cycle under the floor; one build per cycle; the residue count never rose;
build time lower at closure than at launch, measured.

## 3. Files in this folder

| File | Job | Owner |
|---|---|---|
| `scope-draft.md` | Canonical handoff *what* — the operator's rulings, measured baseline, epics, success | operator |
| `workflow-instruction.md` | Per-cycle launch *how* — the batch-floor gate, one-build cadence, dispatch, receipt schema | operator |
| `epic-breakdown.md` | The 30 acceptance criteria across 7 epics | operator |
| `decisions.md` | Bundle-specific ADRs — §1 the sheet rule, §2 the batch floor, §3 the cadence, §4 the ratio, §11 no PCGen in live code, §12 SD-34's closure folded | operator |
| `kanban.md` | One row per criterion; the board | loop (updated per cycle) |
| `progress.md` | Live cycle-by-cycle record + status | loop (created on first cycle) |
| `technical-requirements.md` | Pre-loop prerequisites + normative requirements N1–N18 | operator |
| `technical-design.md` | The converter/live boundary, the converter, the evaluator, the `sheet-complete` status, the gates, the PCGen exit | operator |
| `acceptance-and-verification.md` | Closure gates + verification commands + per-criterion artifact map | operator |
| `risks-and-open-questions.md` | Self-healable vs non-self-healable; R1–R7; open questions | operator |
| `content-unit-inventory.md` | The measured baseline and every re-derive command behind it | operator |
| `forward-scope-register.md` | Successor work depending on this package's output | operator |
| `receipts.md` | Closure-pipeline receipts (architecture-truth-up, graphify, PR) | loop |
| `release-notes.md` | Written at closure | loop |
| `artifacts/` | Per-epic evidence and cycle receipts | loop |
| `artifacts/README.md` | Cycle-artifacts index | operator |
| `references/` | Doctrine, skill, sibling-bundle pointers | operator |
| `references/README.md` | Reference index | operator |

## 4. In-repo cross-references

- **Predecessor** — `../SD-34-book-completion/`. Its Completion Atlas (`scripts/completion_atlas.py`),
  eight engine tables, fable review, and TOKEN-MODEL are SD-35's starting instruments and method.
- **Doctrine mirrors** — `../../doctrine-external/identifier-discipline.md`, `../../governance/no-stub-mvp-doctrine.md`, `../../governance/blocker-closure-doctrine.md`, `../../governance/deferral-revisit-doctrine.md`.
- **Architecture docs** — `../../architecture/` (the closure epilogue §6 obligation re-verifies every touched topic: `overview.md` (the boundary), `rules-engine.md`, `rules-data-tables.md`, `corpus-ingest.md`, `desktop-app.md`, `status.md`, `testing.md`).
- **Conduct surface** — `../../../AGENTS.md`, `../../../CLAUDE.md`.

## 5. Build version target

`0.15.0`:

- **major** — `0` until first publish to `main`.
- **tranche-base** — `15`, the base digit of `tranche/15`.
- **build** — `0` at the cut.

The tranche digit moves on a **new `tranche/N` branch cut**, never on a bundle's own closure
(`decisions.md §10`). Root `Cargo.toml` stays pinned at `0.1.0` and is not the version source
of truth; `apps/desktop/package.json` and `apps/desktop/src-tauri/tauri.conf.json` are.

## 6. Architecture-docs, graphify, and PR closure obligation

Identical in shape to every prior bundle; the canonical procedure is `../template/template.md §6`,
and its procedural half (retrospective + sweep, which land **before** the PR) is
`workflow-instruction.md §11`. Do not duplicate either half into the other file.

**Step 1 is a hard gate and a filed blocker does not satisfy it.** If anything is short, the
closure epilogue stops: no retrospective, no sweep, **no PR**.

## 7. Required canonical files

Per `../template/template.md §7`, all present in this folder: `README.md`, `scope-draft.md`,
`workflow-instruction.md`, `progress.md`, `epic-breakdown.md`, `decisions.md`,
`risks-and-open-questions.md`, `acceptance-and-verification.md`, `content-unit-inventory.md`,
`artifacts/`, `artifacts/README.md`, `references/`, `references/README.md` — plus
`technical-requirements.md`, `technical-design.md`, `kanban.md`, `forward-scope-register.md`,
`release-notes.md` per the sibling-package convention.

## 8. Provenance of every number in this package

Every population figure was measured on 2026-09-07 against `tranche/14` HEAD `5f6b18f4e3` from
`docs/work-inventory.json` (`generated_at: 2026-09-07T13:06:14Z`), cross-checked against
`artifacts/epic-1-atlas/completion-atlas.json`, **and re-measured at the `tranche/15` cut
`4c6c57eb9f` the same evening — identical** (the inventory file did not change between the two).
`content-unit-inventory.md` carries the re-derive command for each.

The live-side PCGen residue figure (78 files) is a coarse grep at the same HEAD; AT-35-E1-005's
first run records the exact baseline.

**They are the cut's figures.** Epic 2's first corpus-wide conversion is the next thing that
moves them (`content-unit-inventory.md §0`). Three denominators are in play and
every figure names its own (`decisions.md §8`): 49,438 (corpus), 23,315 (non-DONE, 37 books),
22,369 (the fable review's non-DONE across 35 non-vehicle books, an older snapshot quoted only
when citing that review).

## 9. Launch gates

`workflow-instruction.md §1`, items 1–12, **run at the `tranche/15` cut `4c6c57eb9f` on
2026-09-07 with outputs pasted.** Tier-1 items 1–3 are satisfied (PR #383 merged, branch cut,
`0.15.0` stamped). Item 4 records that SD-34's own epilogue never ran and is folded
(`decisions.md §12`, AT-35-E1-006). Item 10 records the cold build time AT-35-E1-003 measures
against and the inherited failing-suite baseline.
