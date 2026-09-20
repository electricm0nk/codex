---
title: SD-36 — Consolidation — Release Package
status: in-progress — Epics B, A, E, C1 done; D1 (architecture docs) done; D2/D3 written; C2 and D4–D6 remaining
bundle_id: SD-36
slug: consolidation
scope: docs/release/SD-36-consolidation
artifact_type: release-index
canonical_branch: tranche/16
kanban_board: local-file ./kanban.md
target_version: 0.16.0
canonical_source: docs/release/SD-36-consolidation (this folder)
date: 2026-09-15
---

# SD-36 — Consolidation — Release Package

> ## WARNING — OPERATING METHOD — REQUIRED FOR THIS BUNDLE
>
> **This bundle is operated via a `Workflow`-tool script, invoked from a live session — NOT `/loop /batch` and NOT a one-shot task.** For how to **create** that script — phase structure, tiering, worked skeleton — see `workflow-instruction.md §2.4`. The full per-cycle procedure, orchestration mode, concurrency map, dual-audit gate, retro-event-logging discipline, and epic/bundle closure steps live in `workflow-instruction.md`'s body, authored from `../../governance/workflow-instruction-template.md`. The scope-draft ([`./scope-draft.md`](./scope-draft.md)) is the canonical handoff *what*; the workflow-instruction is the *how*.

This folder is the canonical surface for SD-36.

## 0. Preamble

The bundle's intent, scope, and acceptance-evidence obligations live in [`scope-draft.md`](./scope-draft.md). The per-cycle launch form, eligibility checks, and self-heal mechanics live in [`workflow-instruction.md`](./workflow-instruction.md). This README is the index.

## 1. Bundle snapshot

| Field | Value |
|---|---|
| Bundle ID | SD-36 |
| Slug | `consolidation` |
| Canonical branch | `tranche/16` — cut 2026-09-15 at origin/develop commit 50572eebad (SD-35's PR #390 merge); `decisions.md §2` |
| Kanban board | local-file `./kanban.md` |
| Epics / criteria | 4 / 1+ (B: dashboard freeze, A: crate wall, C: bloat cuts, D: closure) |
| Target version | `0.16.0`, stamped at cut in one commit (version-bump surfaces: `.github/workflows/publish-tester-release.yml`, `apps/desktop/package.json`, `apps/desktop/src-tauri/Cargo.toml` + `Cargo.lock`, `apps/desktop/src-tauri/tauri.conf.json`, 7 desktop test fixtures) |
| Dispatch mechanism | `Workflow` tool, invoked from a live session, per `workflow-instruction.md §2` |
| Cadence | N/A — dispatch is a live `Workflow` session, not a timer loop |
| Closure gate | `tranche/16 → develop` PR; retrospective written + cited; worktree/branch sweep; release notes; architecture-docs refresh (§6) — full sequence in `workflow-instruction.md §11` |

## 2. What this bundle is

SD-36 **consolidates and cleans the codebase after SD-35**: green CI, PCGen walled off, dashboard frozen, bloat cut, disk reclaimed. Four epic phases:

- **Epic B** — freeze the PF1e status page (100% complete), retire the producers (`v06_work_inventory`, `support_state_matrix`, `reach_gate`, dashboard cron jobs).
- **Epic A** — wall off PCGen (oracle + converter) in a dedicated `crates/codex-ingest` so the desktop can never link it. Close two gate blind spots.
- **Epic C** — cut bloat: split `pilot_compute/mod.rs` into ~36 submodules, consolidate path helpers, rewrite table-driven tests.
- **Epic D** — closure: architecture-docs refresh, retrospective, PR.

**Operator goal (2026-09-15):** CI green today; then a tight, clean codebase before UI work and before Starfinder; nothing of PCGen in live code, enforced by the build not by a script; disk reclaimed.

## 3. Status

- Package authored 2026-09-15 evening from the plan document.
- **Done:** Epic B (freeze status, retire producers — closed 2026-09-17, 46/46 `verify.sh` PASS);
  Epic E (SD-35 code-review correctness, 22 findings, folded in per operator ruling — closed
  2026-09-17); Epic A (PCGen crate wall — closed 2026-09-19); Epic C1 (source refactor — closed
  2026-09-20); Epic D1 (architecture-docs full-set rewrite, `decisions.md §10` — closed
  2026-09-20).
- **Written this pass (2026-09-20):** Epic D2 (`docs/retro/sd36-retrospective.md`) and D3
  (`release-notes.md`, re-derived), plus corrections to `kanban.md`/`progress.md` (Epic B's row
  was stale at `open`; Epic E had no row) and this status field.
- **Remaining:** Epic C2 (table-driven test rewrite — not started; its own acceptance command
  needs correcting first per the retrospective's Finding 1); Epic D4 (graphify), D5 (PR open and
  merged), D6 (worktree sweep). See `docs/retro/sd36-retrospective.md`'s "Epic status, as of this
  writing" table and `release-notes.md`'s "Known follow-ups" for the full, re-derivable picture.

## 4. Reading guide

- **Planning:** start with `scope-draft.md` for bundle intent and epic order.
- **Doing:** `workflow-instruction.md` is the per-cycle launch and dispatch procedure; `epic-breakdown.md` gives acceptance rows per epic.
- **Understanding:** `decisions.md` lists the 7 operator rulings (D1–D7) with their enforcement commands; `technical-design.md` and `technical-requirements.md` give the architectural rationale.
- **Verification:** `acceptance-and-verification.md` lists the end-to-end gates and per-criterion artifact map; `release-notes.md` records measured baselines before/after.
- **Trouble:** `risks-and-open-questions.md` lists self-healable vs. non-self-healable splits and any open override flags.
- **Done:** `progress.md`, `receipts.md`, `kanban.md` live as the bundle runs, one row per epic, status open → complete.

