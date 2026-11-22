---
title: SD-18 — Artifacts README
status: placeholder
date: 2026-07-12
purpose: Index of artifacts produced by SD-18 over its lifecycle.
---

# SD-18 — Artifacts

This directory holds artifacts produced during SD-18's lifecycle. Each artifact has its own naming convention and lives under its own subdirectory or filename.

## Expected artifacts (operator-provided or loop-produced)

These artifacts are anticipated. They land here as SD-18 progresses.

### §1.1 pre-loop merge receipt

**Filename:** `consumer-side-composition-merge-receipt-<cycle-id-or-date>.md`

**Provenance:** Tech-priest's §1.1 PR to `tranche/3`.

**Contents:** Standard merge receipt naming the slice ID, base SHA, merge SHA, PR number, and the cycle the merge resolved.

### Loop instruction document

**Path:** `/home/ubuntu/workspace/SD-18-core-rules-breadth-loop-instruction.md`

**Provenance:** Authored 2026-07-12 alongside the bundle, modeled on the matured SD-13 operator-loop pattern. Self-sufficient — the loop reads only this file plus the scope doc, progress doc, and live git state.

### Per-cycle merge receipts

**Filename:** `cycle-<cycle-id>-merge-receipt.md` (one per cycle that lands)

**Provenance:** Each loop cycle that lands work.

**Contents:** Cycle-id, branch, merge SHA, criterion covered, evidence tier transition, self-heals applied, cycle timing.

### Loop seed prompt

**Filename:** `loop-seed-prompt.md` (operator-authored, parallel to the as-written SD-13 prompt)

**Provenance:** Operator, derived from the loop instruction body.

**Contents:** The actual loop-prompt body. Inherits from `references/sd13-loop-model-excerpt.md`.

## What's NOT in this directory

- Working drafts. The canonical scope doc lives at `/home/ubuntu/workspace/SD-18-core-rules-breadth-scope-draft.md`. Working drafts do not belong in the artifacts bundle.
- Per-cycle handoff docs. Per-criterion handoff docs (the per-burden equivalent of SD-13's `sd13-e5-*` handoffs) live under `programs/codex/requirements/SD-18-core-rules-breadth/artifacts/` ONLY when they're merge receipts; otherwise they live next to their test fixture in `tests/fixtures/` or the per-criterion directory.
- Kanban card contents. Cards live on `codex-tranche-3`. Their bodies are accessible via `hermes kanban show`. They are NOT mirrored as files here.
