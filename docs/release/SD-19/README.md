---
title: SD-19 — Core Rules: Spell / Equipment / Reachability — Release Package (CLOSED)
status: closed (closed on `tranche/3` with sibling SD-18; corpus-aware compute seam landed)
bundle_id: SD-19
slug: core-rules-spell-equipment-reachability (workspace slug); src-STC slug: corpus-aware-compute-seam
canonical_branch: tranche/3
kanban_board: codex-tranche-3 (archived)
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/decisions.md
mirror_of: ~/workspace/SD-19-core-rules-spell-equipment-reachability-{scope-draft,loop-instruction,progress}.md
date: 2026-07 (closure window)
epics: 7
criteria: 30
---

# SD-19 — Core Rules: Spell / Equipment / Reachability — Release Package (CLOSED)

This folder is the repo-local full-mirror surface for the closed SD-19 bundle. SD-19 shipped the corpus-aware compute seam (`src/rules_core/corpus/`) — the unified interface that loads book data behind a single seam — and the source-book sibling-directory convention that post-SD-19 bundles (SD-22's APG + ACG + Bestiary 1 ingest) inherit.

## 1. Bundle snapshot

| Field | Value |
|---|---|
| Bundle ID | SD-19 |
| Workspace slug | `core-rules-spell-equipment-reachability` |
| src-STC slug | `corpus-aware-compute-seam` |
| Canonical branch | `tranche/3` |
| Kanban board | `codex-tranche-3` (archived) |
| Closure | Tranche-3 chassis substrate (paired with SD-18) |

## 2. Files in this folder

9 mirror files: `scope-draft.md`, `loop-instruction.md`, `progress.md`, `decisions.md`, `technical-requirements.md`, `epic-breakdown.md`, `acceptance-and-verification.md`, `risks-and-open-questions.md`, `technical-design.md`.

## 3. Authoritative pointers (operator workspace; not bundled here)

- **Upstream strategic/intake authority:** `/home/ubuntu/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/`.

## 4. Relationship to other release folders

- **Paired CLOSED bundle:** [`../SD-18/`](../SD-18/) (Tranche-3 chassis together with SD-19).
- **Downstream CLOSED bundle:** [`../SD-20/`](../SD-20/) (consumes the corpus-aware seam).
- **Downstream CLOSED bundle:** [`../SD-21/`](../SD-21/) (Epic 2 consumes `RuleSetId::Crb` content SD-19 set).
- **Downstream PLANNING bundle:** [`../SD-22/`](../SD-22/) (Epic 3+4+5 expand `RuleSetId::*` to APG / ACG / Bestiary 1, the first multi-`RuleSetId` content surface).
