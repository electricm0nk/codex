---
title: SD-18 — Core Rules Breadth
status: draft (operator review required)
date: 2026-07-12
operator: Todd Hintzmann
owner: god-emporer (architect), tech-priest (pre-loop gate slice), operator (loop execution)
parent: programs/codex/assumptions/tranche-3-starting-assumptions-2026-07-10.md
epic: corpus-driven breadth for codex rules engine; operator-driven loop-routed execution.
related_bundles:
  - programs/codex/requirements/SD-13-core-class-race-roster-and-level-10-progression-matrix/
  - programs/codex/requirements/SD-17-pcgen-corpus-include-graph-resolution/
  - programs/codex/requirements/tranche-2-7-pcgen-corpus-ingestion/
execution_mode: operator-driven claude-code loop on breadth (churn); pre-loop tech-priest card on the consumer-side composition gate.
canonical_handoff_doc: /home/ubuntu/workspace/SD-18-core-rules-breadth-scope-draft.md
objective: support + product-visible (per operator directive, 2026-07-12).
---

# SD-18 — Core Rules Breadth

## What this bundle is

SD-18 ships the full Pathfinder 1e Core Rulebook to the codex rules engine at **levels 1 through 20** across:

- All 7 core races (Dwarf, Elf, Gnome, Half-Elf, Half-Orc, Halfling, Human).
- All 11 core classes (Barbarian, Bard, Cleric, Druid, Fighter, Monk, Paladin, Ranger, Rogue, Sorcerer, Wizard).
- The 2 named SD-13 interaction rows (Human bonus feat / ability-bonus seam; non-Human race × class progression beyond pilot).
- All 9 PF1 strict schools (Abjuration, Conjuration, Divination, Enchantment, Evocation, Illusion, Necromancy, Transmutation, plus Universal).
- All 4 core-rulebook equipment categories (arms_armor, general, magic_items, equipmods).

End-user feature proof per acceptance criterion: the row or corpus element reaches `supported/Product-visible`. The objective is the *combination* — `SupportState::Supported` AND evidence tier `Product-visible`. State alone is counterfeit.

## Execution posture summary

- **Pre-loop gate** (§1.1 of the scope doc): a single consumer-side composition slice, card-routed to tech-priest, that bridges `CharacterInput` (chosen) with `SourcePackageContent` (corpus) into the input `pilot_compute.rs` evaluates. No loop iteration begins until this slice ships.
- **Loop-routed coverage** (§3 of the scope doc): 33 acceptance criteria, executed one per loop iteration. Each iteration:
  1. Reads the scope doc and progress doc.
  2. Creates a feature branch off `tranche/3` (per matured SD-13 model).
  3. Lands a bounded slice.
  4. Auto-merges to `tranche/3` (tranche branches don't need operator review).
  5. Self-heals merge conflicts inline if possible, otherwise writes to `## Open blockers` and exits `FAIL`.
  6. Deletes the feature branch from local + origin after successful merge.
  7. Mints a kanban card into `codex-tranche-3` as a post-mortem record (status=done).
  8. Updates the progress doc and exits.

## Why this bundle exists separately from `tranche-2-7-pcgen-corpus-ingestion/`

SD-17 (corpus ingestion) and SD-18 (rules engine consumes ingested corpus) are not the same scope. SD-17 closes when the corpus-side records (`SourcePackageContent`) exist and the converter projects into them. SD-18 starts there and *consumes* those records through the rules compute path. Two separate bundles so the doctrine doesn't conflate "the corpus parses" with "the rules engine reasons over the parsed corpus."

## Files in this bundle

- `README.md` (this file) — bundle overview, posture summary, navigation.
- `acceptance-and-verification.md` — closure gates per §7 of the scope doc.
- `epic-breakdown.md` — 34 acceptance criteria grouped by execution lane.
- `risks-and-open-questions.md` — self-healable vs. non-self-healable lists, §1.2 gap, pre-loop gate risk.
- `technical-design.md` — execution lane split, branch lifecycle, kanban card schema.
- `technical-requirements.md` — pre-loop prerequisites for the bundle to start.
- `decisions.md` — the 11-item decision record from this session ("why we did that").
- `artifacts/` — merge receipts, branch receipts, loop-handoff doc, loop seed prompt (operator-authored).
- `references/` — matured SD-13 model excerpt, scope doc pointer.

## Reading rule

When a future session opens SD-18:

1. Read `decisions.md` first — it tells you the *why* of the bundle's shape.
2. Read the scope doc at the canonical handoff path — it tells you the *what* and the *how*.
3. Open `technical-design.md` for the lane split and card schema details.
4. Open `epic-breakdown.md` to know how the 34 acceptance criteria map to cycles.
