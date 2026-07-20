---
title: SD-19 — Epic Breakdown
status: draft (operator review required)
date: 2026-07-14
companion_to: /home/ubuntu/workspace/SD-19-core-rules-spell-equipment-reachability-scope-draft.md
---

# SD-19 — Epic Breakdown

Maps the 15 acceptance criteria from the scope doc's §1 (pre-loop) and §2 (loop) to execution lanes and cycle units.

## Execution lane split

```
Pre-loop capability slices (each an atomic direct commit to tranche/3,
atomic, no PR; per the no-branches convention from decisions.md §6)
├── §1.0 Foundation slice (atomic commit 1): canonical Paizo-table store  [1 criterion]
│         - src/rules_core/rules_tables/ module shell
│         - src/rules_core/rules_tables/crb/ populated with CRB cells
│           as structured data files (class tables, spell list header,
│           equipment stats — one file per table)
│         - pub enum RuleSetId { Crb, /* future: Um, Apg, ... */ }
│         - Test asserting CRB directory is parseable and RuleSetId::Crb
│           resolves. No seam, no resolvers, no wrapper types — just
│           data and type. Ships first; everything else depends on it.
└── §1.1 Main capability slice (atomic commit 2): corpus-aware compute  [1 criterion]
          seam. NEW module src/rules_core/pilot_compute_corpus.rs
          (compute_pilot_with_corpus + the wrapper types
          CorpusPilotReceipt, CorpusDerivedSection, SchoolCoverage,
          ResolvedEquipment, DerivedEquipmentStats).
          - compute_pilot_base_chassis imported from pilot_compute.rs
            (which itself stays untouched).
          - equipment_id_resolve function (NEW, src/rules_core/equipment_resolver.rs)
          - spell_id_resolve function (NEW, src/rules_core/spell_resolver.rs)
          - CharacterInput.spells_selected field (Vec<SpellSelection>)
          - MatrixSubjectType::School and ::Equipment variants
          - support_state_matrix.rs row shapes for both
          - tests/sd19_seam_shapes_correctness.rs (proof before loop runs)
          - tests/fixtures/rules_core/sd19_seam_crb_*.txt (13 hand-typed
            real-corpus fixtures with the sd19_seam_crb_ prefix)
          - RuleSetId threaded through both resolvers as explicit parameter

Loop-routed coverage (claude-code loop, one per cycle)
├── §2.4 Spell school cards (9 criteria: Abjuration, Conjuration,        [9 criteria]
│         Divination, Enchantment, Evocation, Illusion, Necromancy,
│         Transmutation, Universal)
└── §2.5 Equipment category cards (4 criteria: arms_armor, general,       [4 criteria]
          magic_items, equipmods)
```

Total: **16 acceptance criteria.** 2 pre-loop capability slices (foundation + main, atomic commits 1 and 2) + 13 loop-routed cycles (9 schools + 4 categories) + 1 seam-shapes-correctness gate that is part of the main capability slice but enumerated separately because it is the verification step, not the implementation step.

**Linear dependency**: the foundation slice (§1.0) must land before the main capability slice (§1.1) can begin. The main capability slice must land before the loop (§2.4/§2.5) can begin. SD-19 cannot run in parallel with SD-18's loop (per operator directive 2026-07-14; SD-19 waits for SD-18 to complete).

## Cycle ordering (operator-prioritized)

Per operator directive 2026-07-14, the loop's first cycles land the spell-school cards (the §3.4 side has the corpus-side half already proven by SD-17-B-4 and just needs the consumer-side half, which is the smaller of the two consumer-side changes). Equipment categories follow because the §3.5 cycle's investigation identified that equipment's selection surface (`equipment_selections`) already exists — only the resolver and the corpus-aware compute path are new — and the per-category representative-sample work is more mechanical than per-school spell-content grounding.

```
Cycle 1-9   §2.4 Spell schools × 9 (per strict school alphabet)
             1. Abjuration
             2. Conjuration
             3. Divination
             4. Enchantment
             5. Evocation
             6. Illusion
             7. Necromancy
             8. Transmutation
             9. Universal

Cycle 10-13 §2.5 Equipment categories × 4 (per corpus-natural category)
            10. arms_armor
            11. general
            12. magic_items
            13. equipmods
```

This is the *suggested* order. The loop's actual progression is data-dependent — see §4 of the scope doc for the loop's reading rules. The loop may reorder as needed based on per-cycle feasibility (e.g., if a school cycle's corpus slice is unexpectedly small, the loop may group multiple schools into one cycle's representative sample; if an equipment category's representative sample is unexpectedly large, the loop may split one category across two cycles and document the split in the progress doc).

## Cycle unit definition

A single loop cycle lands one acceptance criterion. Each cycle:

1. Picks one criterion from the progress doc's "open" list (§4 of scope doc).
2. Verifies the working tree is on `tranche/3` (per `decisions.md` §6; no feature branch created).
3. Lands the bounded work (code, tests, fixtures, handoff doc).
4. Commits directly to `tranche/3` (no PR; no auto-merge; no ephemeral branch).
5. Self-heals inline or exits `FAIL`.
6. Mints a kanban card on `codex-tranche-3` per the loop brief's §Step 10 schema.
7. Updates the shared progress doc (`~/workspace/SD-18-core-rules-breadth-progress.md`) by appending to its `## SD-19 cycles` section.
8. Exits.

A cycle is a *unit of post-mortem*, not a unit of delivered scope. One cycle, one criterion, one card. The cycle log in the progress doc plus the cards on the board let a 3-day-later operator reconstruct any specific cycle.

## What the breakdown does NOT specify

- Per-criterion implementation approach (the loop picks the smallest change that satisfies the criterion, within the seam shape established by the pre-loop capability slice).
- Per-criterion TDD structure (inherits from the matured SD-18 / SD-13 model's red-green-refactor pattern; see SD-18's `references/sd13-loop-model-excerpt.md`).
- Per-criterion timing (depends on corpus size, parser friction, and behavior complexity; the loop's self-healing handles friction; volume is incidental).
- The exact corpus-side representative sample per equipment category (the loop's per-cycle choice; documented in the cycle's progress doc entry and the per-cycle card body).

## Cross-reference

- `decisions.md` §1 (capability slice + loop pattern), §2 (equipment-id resolver), §3 (spell selection shape), §4 (compute seam as additive function), §5 (matrix carrier extension).
- `technical-design.md` (per-cycle mechanics, seam signature, resolver signatures, branch lifecycle command sequence).
- `risks-and-open-questions.md` (per-criterion risks and blockers; the two open override flags from `decisions.md` §2 and §3).
- `/home/ubuntu/workspace/SD-19-core-rules-spell-equipment-reachability-scope-draft.md` (canonical handoff doc; acceptance criteria with concrete corpus/code pointers).
- `/home/ubuntu/workspace/SD-19-core-rules-spell-equipment-reachability-loop-instruction.md` (loop body; the `/loop` invocation reads this).
- `~/workspace/SD-18-core-rules-breadth-progress.md` — under the headings "## cycle-2026-07-15T0300 | §3.4 spell-school reachability-chain investigation" and "## cycle-2026-07-15T0400 | §3.5 equipment-category reachability-chain investigation" (anchored headings, not line numbers).