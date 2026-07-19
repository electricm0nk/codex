---
title: SD-19 — Acceptance and Verification
status: draft (operator review required)
date: 2026-07-14
companion_to: /home/ubuntu/workspace/SD-19-core-rules-spell-equipment-reachability-scope-draft.md
---

# SD-19 — Acceptance and Verification

SD-19 closes when every acceptance criterion in the scope doc's §2 satisfies its criterion AND every School and Equipment row in the SD-13 matrix reaches the highest tier achievable from corpus coverage alone.

## Closure gates (mandatory)

1. **Foundation slice shipped.** §1.0 of the scope doc (canonical Paizo-table store) has merged to `tranche/3` as the first atomic commit of SD-19's capability-shipping phase. This is the structural prerequisite for the main capability slice and for every §2 acceptance criterion. The slice's own `tests/sd19_table_store_foundation.rs` is green; `cargo test --locked` and `cargo clippy --locked --tests -- -D warnings` are green against the slice's tip with zero SD-18 regressions.

2. **Main capability slice shipped.** §1.1 of the scope doc (corpus-aware compute seam) has merged to `tranche/3` as the second atomic commit of SD-19's capability-shipping phase. The slice's own `tests/sd19_seam_shapes_correctness.rs` is green; `cargo test --locked` and `cargo clippy --locked --tests -- -D warnings` are green against the slice's tip with zero SD-18 regressions.

3. **Seam shapes correctness verified.** `tests/sd19_seam_shapes_correctness.rs` proves: (a) `compute_pilot_with_corpus` and the wrapper types (`CorpusPilotReceipt`, `CorpusDerivedSection`, `TableCellRef`, `SchoolCoverage`, `ResolvedEquipment`, `DerivedEquipmentStats`) exist with the documented signatures; (b) `equipment_id_resolve` returns `Some(&EquipmentRecord, Option<TableCellRef>)` for the documented fixture set and `None` for the documented unknown set; (c) `spell_id_resolve` returns `Some(&SpellRecord, Option<TableCellRef>)` for the documented fixture set and `None` for the documented unknown set; (d) `CharacterInput.spells_selected` round-trips through serde; (e) `MatrixSubjectType::School(Pf1SchoolId)` and `MatrixSubjectType::Equipment(EquipmentCategory)` round-trip through serde; (f) a sample end-to-end call (one spell in one school, one equipped item in one category) produces a `CorpusPilotReceipt` whose `corpus_derived` section is non-empty AND whose `base` field equals the same input run through `compute_pilot_base_chassis` directly. The (f) assertion is the proof that the seam is actually computing something corpus-derived in `corpus_derived` while not perturbing the unchanged chassis in `base`.

3. **Spell school cards landed** (9 cards). For each PF1 strict school (Abjuration, Conjuration, Divination, Enchantment, Evocation, Illusion, Necromancy, Transmutation, plus Universal), every spell in the school's slice is reachable from a chosen `CharacterInput.spells_selected` and consumable by the rules engine through the corpus-aware compute seam. Per school, the corresponding `MatrixSubjectType::School(Pf1SchoolId)` row reads `support_state=Supported` and `evidence_tier=Product-visible`. Each cycle's `CorpusDerivedSection.school_coverage[school]` carries a `TableCellRef` pointing at the school's row of the CRB spell list table, asserting the corpus record the cycle resolved lives at a specific canonical Paizo table cell. Grounding artifacts cite the commit SHA on `tranche/3` and the loop cycle's card id on `codex-tranche-3`. Spellbook engine / slot math / DCs remain deliberately out of scope per `decisions.md` §1.3 and `technical-design.md` §1.3; this gate grounds reachability, not effect execution.

4. **Equipment category cards landed** (4 cards). For each of the four `core_rulebook/cr_equip_*.lst` files (`arms_armor`, `general`, `magic_items`, `equipmods`), a representative sample of items is reachable from a chosen `CharacterInput.equipment_selections`, resolved via `equipment_id_resolve`, and produces corpus-derived stat contributions in the receipt's `equipped_items` list. Per category, the corresponding `MatrixSubjectType::Equipment(EquipmentCategory)` row reads `support_state=Supported` and `evidence_tier=Product-visible`. Each cycle's `ResolvedEquipment` carries a `TableCellRef` pointing at the item's row of the relevant CRB equipment table.

5. **Progress doc reflects every criterion as satisfied.** The shared `~/workspace/SD-18-core-rules-breadth-progress.md` shows every acceptance criterion in §2 (the SD-19 append-only `## SD-19 cycles` section) as `done` with row id, commit SHA, and card id. The pre-existing SD-18 §3.4/§3.5 rows are also updated to reflect the SD-19 landings.

6. **Kanban board populated.** `codex-tranche-3` shows the post-loop populated ledger, every SD-19 card `status=done`, with merge receipts and audit-grade context per the §Step 10 schema.

7. **`tranche/3 → develop` promotion PR opened.** Operator-driven, matching the existing promotion cadence (tranche-2-5 → develop, tranche-2-7 → develop, tranche-3 → develop). This PR covers both SD-18's lane and SD-19's lane since they share `tranche/3`.

## Verification at closure

The closure posture is reviewable entirely from four surfaces:

- The scope doc (`/home/ubuntu/workspace/SD-19-core-rules-spell-equipment-reachability-scope-draft.md`).
- The shared SD-18 progress doc (default path `/home/ubuntu/workspace/SD-18-core-rules-breadth-progress.md`, with SD-19 cycles appending to its `## SD-19 cycles` section).
- The technical-design doc (`/home/workspace/programs/codex/requirements/SD-19-corpus-aware-compute-seam/technical-design.md`) for seam/resolver signatures and the cycle surface.
- `codex-tranche-3` board (post-mortem records; SD-18 and SD-19 cards both live here).
- `git log --oneline tranche/3 -N` (the commit history, since SD-19 commits directly to `tranche/3` with no PRs or merges).

Operator's first action on return from a multi-day run: read `## Open blockers` in the progress doc. If empty, gates 1–7 above are the entire verification.

## What does *not* gate closure

- Loop's cycle log size (the loop may have run 13 cycles or 26 cycles if a category splits; the criterion is the criteria, not the volume).
- Number of self-heals applied during the run.
- Whether the pre-loop capability slice landed in 1 cycle or 5 cycles.
- Whether some cards landed as documentation-only versus full code-bearing (per the loop brief's eligibility check — a school or category may legitimately land as a documentation-only entry if the seam proves sufficient to ground the corpus-derived contribution in a recognition-only form).
- Spellbook engine / slot math / spell save DCs — explicitly out of scope per `decisions.md` §1.3; their absence is not a closure blocker.

## What additionally does NOT gate closure (because SD-18's loop still gates its own closure)

- SD-19 waits for SD-18's loop to complete (per operator directive 2026-07-14). No concurrent execution, no interleaving. The two loops run sequentially: SD-18 exhausts its lane (or is otherwise paused), then SD-19 begins. Operator's call on the exact moment of handoff.

## Cross-reference

- `decisions.md` §1.3 (what the seam does NOT do) — the closure posture intentionally grounds reachability, not effect execution.
- `technical-design.md` §6.6 (cycle hard stops) and §6.5 (self-healing posture).
- `~/workspace/SD-18-core-rules-breadth-progress.md` the dated cycle-2026-07-15T0300 (§3.4) and cycle-2026-07-15T0400 (§3.5) headers — the blocker entries that are now closed by the SD-19 capability slice + loop.