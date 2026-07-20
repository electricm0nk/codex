---
title: SD-20 — Rules Engine Completeness — Release Package (CLOSED)
status: closed (Epic 1 closed cycle 4, 2026-07-17; Epic 2-7 wiring project closed cycle 7; promoted `tranche/4 → develop`)
bundle_id: SD-20
slug: rules-engine-completeness
canonical_branch: tranche/4
kanban_board: codex-tranche-4 (archived)
target_version: 0.4.<last_build>
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-20-rules-engine-completeness/decisions.md
mirror_of: ~/workspace/SD-20-rules-engine-completeness-{scope-draft,loop-instruction,progress}.md
date: 2026-07-08 (planning); 2026-07-17 (closure)
epics: 7
criteria: 30
boundary_contract: ../../SD-20/boundary-contract.md
---

# SD-20 — Rules Engine Completeness — Release Package (CLOSED)

This folder is the repo-local surface for the **closed** SD-20 bundle. SD-20 shipped the per-character rules-engine surface, the closure of Epic 2-7's wiring project. Boundary contract at [`boundary-contract.md`](./boundary-contract.md) (Epic 1 cycle 4) is the canonical API every post-SD-20 epic adds onto.

> **Relocated 2026-07-20** (operator directive, repo-wide docs-structure consolidation): the boundary contract used to live at `docs/SD-20/boundary-contract.md`, kept outside `docs/release/` on the reasoning that "code-shaped artifacts... let codebase cross-references resolve without redirect." That reasoning didn't hold up under a full consolidation pass — nothing about referencing `docs/release/SD-20/boundary-contract.md` from Rust doc comments is actually harder than referencing the old path. It now lives in this folder like every other SD-20 doc.

## 1. Bundle snapshot

| Field | Value |
|---|---|
| Bundle ID | SD-20 |
| Slug | `rules-engine-completeness` |
| Canonical branch | `tranche/4` |
| Kanban board | `codex-tranche-4` (archived) |
| Final version | `0.4.<last_build>` |
| Closure date | 2026-07-17 (Epic 1 cycle 4 + Epic 2-7 cycle 7; PR landed) |

## 2. Files in this folder

9 mirror files: `scope-draft.md`, `loop-instruction.md`, `progress.md`, `decisions.md`, `technical-requirements.md`, `epic-breakdown.md`, `acceptance-and-verification.md`, `risks-and-open-questions.md`, `technical-design.md`.

## 3. Authoritative pointers (operator workspace; not bundled here)

- **Upstream strategic/intake authority:** `/home/ubuntu/workspace/programs/codex/requirements/SD-20-rules-engine-completeness/`.
- **Boundary contract:** [`boundary-contract.md`](./boundary-contract.md) (relocated 2026-07-20 into this folder; previously code-adjacent at `docs/SD-20/`).

## 4. Relationship to other release folders

- **Downstream CLOSED bundle:** [`../SD-21/`](../SD-21/) (per-character rules-engine consumption runs on top of SD-20's boundary contract).
- **Downstream PLANNING bundle:** [`../SD-22/`](../SD-22/) (SD-22's content-source ingest feeds the engine SD-20 closed).
- **Upstream bundles:** [`../SD-19/`](../SD-19/) (corpus-source pattern SD-20's data lookup uses), [`../SD-18/`](../SD-18/) (chassis grounding).
