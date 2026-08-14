---
canonical: true
owner: god-emporer
status: planning-ready (split from SD-30, operator ruling 2026-08-14)
date: 2026-08-14
---

# SD-32 Loop Instruction

**`SD-30-class-feature-archetype-bundle/loop-instruction.md` governs this package's cycles, with the
overrides below.** Do not fork a second copy of that file's cycle procedure — read it directly each
cycle. This file states only what differs.

## Overrides

1. **Card source:** this package's own `kanban.md` — `epic-1-race-chassis`, `epic-2-verdict-paths`,
   `epic-3-cloud-fanout`.
2. **No PI-gate dependency.** Unlike `SD-31-corpus-closure-grind`, this package's cycles do not write
   corpus content to `rules_tables/` — they build engine capability (chassis data model, classifier
   code). No cross-SD PI-gate check applies. If a cycle in this package finds itself about to write a
   `rules_tables/<book>/` record, it has drifted into SD-31's scope and should stop.
3. **DoD-8 verification is a closing step, not an afterthought, for every Epic 1 cycle** — a race
   chassis cycle is not `COMPLETE` until a real on-screen character sheet is captured (AT-32-001).
4. **Epic 2-F1 (the labelling gate) is a hard prerequisite for Epic 2-F2** — a cycle may not start
   classifier code before the hand-labelled sample is committed. A cycle that finds itself tempted to
   write classifier logic "to see what it finds" before the sample exists stops and reports instead.
5. **Handoff receipts** to `SD-31-corpus-closure-grind` are written explicitly per AT-32-004 — name the
   SD-31 card, not just "this unblocks ingest."
6. **Concurrency/hardware numbers:** same capture as
   `SD-31-corpus-closure-grind/loop-instruction.md` override 4 — 8 cores / 45GB / 968GB at 19% used,
   3-agent cap, 2026-08-14 stamp, re-derive if stale.

## What is not overridden

Everything else in SD-30's `loop-instruction.md` applies unchanged — the per-cycle procedure shape, the
disk reclamation step, the "generated, never hand-maintained" figure discipline, Workflow-tool dispatch
mode, stop-vs-press-on rules, the Opus-high orchestrator mandate.
