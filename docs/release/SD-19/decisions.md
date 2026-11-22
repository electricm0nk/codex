---
title: SD-19 — Decision Record ("Why we did that")
status: draft (operator review required)
date: 2026-07-14
companion_to: /home/ubuntu/workspace/SD-19-core-rules-spell-equipment-reachability-scope-draft.md
---

# SD-19 — Decision Record

This file captures the deliberate choices made in the SD-19 planning conversation on 2026-07-14. Each item includes the decision, the alternative it displaced, and the reason the alternative was rejected. Future sessions that ask "why is SD-19 shaped this way" should find the answer here without re-litigating it.

SD-19 exists because the SD-18 loop's own investigation cycles (cycle-2026-07-15T0300 for §3.4 and cycle-2026-07-15T0400 for §3.5) discovered at first-hand code depth that the spell-school and equipment-category acceptance criteria are blocked on a shared structural gap: `pilot_compute.rs` has no corpus-aware compute path, `CharacterInput` has no spell-content or equipment-id selection mechanism that links to corpus identity, and the SD18-PRELOOP `ComposedCharacterInput` is built and immediately discarded at every call site. SD-19 closes that gap as a tranche-level capability slice, then runs a loop to ground the 13 acceptance criteria SD-18 could not.

## 1. SD-19 is a capability slice plus a coverage loop, not a coverage loop alone

**Decision:** SD-19 has two phases: (a) a single bounded capability slice — the corpus-aware compute seam, both resolvers, both `CharacterInput` selection surfaces, and the matrix carrier extensions — landed card-routed through tech-priest exactly the way SD-18's §1.1 pre-loop gate was; (b) an operator-driven claude-code loop that grounds all 9 §3.4 school cards and all 4 §3.5 equipment-category cards (13 cards total, one per cycle).

**Displaced alternative:** Loop-only — let the SD-18 loop's existing cycle surface eventually land §3.4/§3.5 by re-attempting each criterion per cycle.

**Reason:** The SD-18 loop's own investigation cycles proved this does not work. The eligibility check in the loop brief (`/home/ubuntu/workspace/SD-18-core-rules-breadth-loop-instruction.md` Step 1 condition 3) explicitly forbids cycles that "need a new subsystem." Spellbook engine and equipment-effect computation are exactly that. A loop-only path means the loop would cycle forever re-discovering and re-citing the same dated blocker entries, producing zero new landed criteria and burning operator review attention on no-op cycles. The capability-slice-then-loop pattern matches SD-18's own §1.1 pre-loop gate (the `ComposedCharacterInput` ship) — proven structural mechanism, not new doctrine.

## 2. Equipment-id ↔ corpus-record matching uses a documented resolver, not raw KEY: tokens

**Decision:** Define a single function `equipment_id_resolve(item_id: &str, corpus: &SourcePackageContent) -> Option<&EquipmentRecord>` in `src/rules_core/equipment_resolver.rs`. The resolver's rule: lookup is a `HashMap<String, EquipmentRecord>` keyed on a normalized form of `EquipmentRecord.name` (lowercase, spaces to underscores, strip parenthesized qualifiers like `(Base)` or `(Masterwork)`), with a secondary exact-match index on the corpus's `KEY:` token for collision resolution. The existing fixture-string namespace (`"item:longsword"`, `"item:chain_shirt"`) is preserved as the *public* `CharacterInput.item_id` surface; the resolver translates it to the canonical corpus record.

**Displaced alternatives:**

- *Raw KEY: token* — every `CharacterInput.item_id` carries the exact corpus `KEY:` string verbatim (e.g. `"Padded Armor (Base)"`). Zero new code, but collides with the existing fixture constants (`LONGSWORD_ITEM_ID = "item:longsword"` at `pilot_compute.rs:2283`), forcing every existing test fixture to change. Two identity namespaces inside the same compute path is worse than one bounded resolver.
- *Display-name match on `EquipmentRecord.name`* — same namespace problem plus cross-source collision risk the cycle already documented (`"Healing Potion"` appears in core_rulebook and Ultimate Magic).
- *Defer equipment entirely* — ship the spell seam only. Contradicts the operator's stated directive that both §3.4 and §3.5 are in-scope for this tranche.

**Reason:** The resolver preserves the existing fixture idiom (`"item:longsword"`) and centralizes the normalization rule in one place. New cycles call `equipment_id_resolve`; the rule is one line in one file; the cross-source concern is handled by the secondary KEY: index. Future cycles never re-derive the matching rule.

## 3. Spell-content selection on `CharacterInput` is a top-level `Vec<SpellSelection>` with `acquisition_mode`

**Decision:** Extend `ChosenCharacterState` in `src/rules_core/character_input.rs` with a new field `pub spells_selected: Vec<SpellSelection>`. `SpellSelection` carries `spell_id: String` (the corpus `KEY:` token, normalized identically to equipment per §2), `source_class_id: ClassId`, and `acquisition_mode: AcquisitionMode` (an enum with variants `Known | Prepared | Granted`). Mirrors the existing `equipment_selections` shape exactly.

**Displaced alternatives:**

- *Class-scoped* — each `class_levels` entry gets `.spells_selected: Vec<SpellRef>`. Breaks the `ChosenCharacterState` shape asymmetry (equipment top-level, spells nested), forcing every consumer to special-case the nested path.
- *Prepared-only at first* — only `prepared_spells: Vec<(class_id, spell_id)>` lands in SD-19; spontaneous-caster math lands in a later SD. Smallest possible first cut but locks in a known refactor: a Wizard and a Sorcerer both "have" spells but only the Wizard's are prepared, so the type has to change the moment spontaneous math lands.
- *Defer the choice* — let SD-19's own pre-loop gate decide the surface. Adds a tranche-level decision into SD-19's own scope; contradicts the operator's stated deliverable.

**Reason:** The top-level `Vec<SpellSelection>` with `acquisition_mode` mirrors `equipment_selections` exactly. The codebase has already accepted the "top-level chosen-state list, each item carries an id + activation metadata" pattern (the `equipment_selections` field with `equipped_or_active` and `active_state`). Extending it is the smallest cognitive jump and the smallest blast radius for existing readers. `acquisition_mode` is the difference between a clean first cut and a future refactor — knowing whether a spell is Known/Prepared/Granted at the type level prevents a second `CharacterInput` change later when prepared-vs-spontaneous math actually lands.

## 4. The compute seam is `compute_pilot_with_corpus`, not a rewrite of `compute_pilot_base_chassis`

**Decision:** Add a new function `compute_pilot_with_corpus(input: &CharacterInput, corpus: &SourcePackageContent) -> PilotReceipt` in `src/rules_core/pilot_compute.rs` (line, file, and exact signature finalized in the §1.1 capability slice). This function calls `compute_pilot_base_chassis` internally and adds the corpus-derived contributions as additive deltas to its output. `compute_pilot_base_chassis` itself stays untouched — every landed SD-18 cycle's existing call sites and test fixtures keep working unchanged.

**Displaced alternative:** Modify `compute_pilot_base_chassis` to accept a corpus parameter directly.

**Reason:** Tranche-3 has 35 merged commits against `pilot_compute.rs`. Every one of them calls `compute_pilot_base_chassis` with the existing signature. Changing that signature would force every landed cycle's tests and fixtures to be revisited in the same slice. The additive-seam pattern — keep the existing function, add a new function that composes with it — is exactly the SD-18 §1.1 `ComposedCharacterInput` pattern (a wrapper type that carries both halves without modifying the inner type). Same doctrine, one capability slice.

## 5. `MatrixSubjectType` gains `School` and `Equipment` variants; `support_state_matrix.rs` gains row shapes for both

**Decision:** Extend `MatrixSubjectType` in `src/rules_core/support_state_matrix.rs` with two new variants: `School(Pf1SchoolId)` and `Equipment(EquipmentCategory)`. Add row-shape constructors and carrier fields mirroring the existing `Race` and `Class` row shapes. The progress doc already tracks §3.4/§3.5 status in its own table; after SD-19 the matrix carrier is the authoritative state, the progress doc mirrors it.

**Displaced alternative:** Keep §3.4/§3.5 status only in the progress doc; never model them as matrix rows.

**Reason:** Per the §3.5 investigation cycle's own finding (line ~5664): "neither spell schools nor equipment categories are modeled as matrix rows; both are tracked only in this progress doc's own §3.4/§3.5 sections, outside the `support_state_matrix.rs` carrier. This is not itself a new blocker ... but it confirms a landed §3.5 cycle would not touch `support_state_matrix.rs` even if eligible." Modeling them as matrix rows is the closure-gate condition (`supported/Product-visible` requires a matrix row transition per the SD-13 vocabulary); without the variant, the §3.4/§3.5 cards can never reach the same closure posture as the §3.1/§3.2 cards.

## 6. Loop pattern is linear commit-to-tranche/3, no ephemeral feature branches

**Decision:** SD-19 lives on the same `tranche/3` branch as SD-18, uses the same `~/workspace/SD-18-core-rules-breadth-progress.md` (SD-19 cycles append to a new §SD-19 section in the same doc), and mints post-mortem cards to the same `codex-tranche-3` board. Each cycle commits directly to `tranche/3` — no ephemeral feature branch, no auto-merge, no PR. The capability slice commits directly to `tranche/3` as a single commit (no PR) per operator directive 2026-07-14.

**Displaced alternatives:**

- *Inherit SD-18's ephemeral-branch + auto-merge posture* — operator reports recent claude-cli friction with automatic PR merges, and the file-touch partition already collapses parallel attempts to serial execution, so feature branches add merge-step overhead with zero concurrency benefit.
- *Use a separate `tranche/4` branch for SD-19* — operator directive: SD-19 is part of tranche-3, not a new tranche. Shared branch, shared progress doc, shared board.
- *Re-author the loop pattern from scratch for SD-19* — unnecessary; SD-18's cycle cadence, eligibility check, hard stops, self-healing posture, and post-mortem card schema all transfer cleanly to a no-branches context with only the branch-lifecycle sections changing.

**Reason:** Linear execution removes the merge-step friction without removing any safety property the loop actually depended on. The file-touch partition (one cycle at a time on `pilot_compute.rs` / `support_state_matrix.rs`) was the real concurrency control; ephemeral branches were an artifact of the as-written SD-13 prompt's parallel-cycle assumption, which never held for the codex codebase. The SD-19 cycle log entries gain a `merge_sha` field (the direct-commit SHA on `tranche/3`) instead of the SD-18 `feature_branch` + `merge_receipt_sha` pair. The capability slice's PR template is replaced by a commit message that names the slice in its subject line and references this decisions file in its body.

**Override window:** If claude-cli's PR-merge friction is later resolved, this decision can be revisited. The doctrine bundle's technical-design.md and loop brief name the linear-commit pattern as the SD-19 posture and would need patching to revert.

## 7. Both §3.4 (9 schools) and §3.5 (4 categories) are in-scope

**Decision:** SD-19 grounds all 13 §3.4 + §3.5 acceptance criteria via the loop. The capability slice lands both resolvers and both `CharacterInput` selection surfaces in one bounded ship; the loop then runs 13 ordinary cycles, one per acceptance criterion, exactly mirroring the SD-18 §3.2 widening cadence.

**Displaced alternatives:**

- *Seam only, no card landings* — ship the seam and hand off all 13 cards to a future SD-N. Contradicts the operator's stated deliverable that both sections be grounded.
- *Spells-only first, equipment later* — split SD-19 into SD-19 (spells) and SD-20 (equipment). Smaller first deliverable but contradicts the framing that both deficiencies are being fixed together.
- *Seam design only, no implementation* — SD-19 is a research bundle. Most conservative but produces no shipped capability; the SD-18 loop's blocker entries stay open.

**Reason:** Per operator directive 2026-07-14: "the structural deficiency ... needs to be rectified since I identified it clearly as a deliverable for this tranche." Both §3.4 and §3.5 are tranche deliverables the operator named. The capability slice is bounded; the loop work is ordinary; no architectural reason to split.

## 8. Scope doc and loop brief live at workspace root, mirroring SD-18

**Decision:** Two workspace-root docs anchor SD-19, matching the SD-18 pattern (`decisions.md` §10):
- `/home/ubuntu/workspace/SD-19-core-rules-spell-equipment-reachability-scope-draft.md` (canonical handoff, the 15 acceptance criteria live here)
- `/home/ubuntu/workspace/SD-19-core-rules-spell-equipment-reachability-loop-instruction.md` (loop body, the `/loop` invocation reads this)

**Displaced alternative:** Single canonical doc under `programs/codex/requirements/SD-19-corpus-aware-compute-seam/`.

**Reason:** SD-18 set the pattern: workspace root holds operator-facing working docs; `programs/codex/requirements/` holds doctrine. Two audiences, two locations. SD-19 inherits.

## 8. Bundle 1 (canonical Paizo-table store) folds into SD-19

**Decision:** The canonical Paizo-table store, originally scoped as a stand-alone "bundle 1" tranche-4 prerequisite, is folded into SD-19 as a new capability-slice deliverable. SD-19 ships the table store, the `RuleSetId` enum, and the seam that consumes the table store, all as part of one bundle.

**Reason:** Per operator directive 2026-07-14: The canonical Paizo-table store is the structural prerequisite everything in tranche-4 depends on (per the tranche-4 dependency analysis), and folding it into SD-19's scope rather than spinning up a separate bundle-1 STC keeps the doctrine count manageable while preserving the dependency-ordering that prevents future subsystems from synthesizing fake data or hard-coding the table twice.

The foundation slice ships first as a tiny stand-alone atomic commit (table store + `RuleSetId` only); the main capability slice follows as a separate atomic commit with the existing six SD-19 deliverables plus the table-store consumption and `RuleSetId`-threaded resolvers. Two slices, two atomic commits, both into `tranche/3`. The dependency between them is linear, not parallel.

## 9. CRB-only scope with `RuleSetId` shape ready for future rule books

**Decision:** SD-19 ships the canonical table store under `src/rules_core/rules_tables/crb/` (one source-book subdirectory, one rule set) populated with the CRB cells as structured data files. A `pub enum RuleSetId { Crb, /* future: Um, Apg, ... */ }` ships with the foundation slice; CRB is the only variant populated today, other variants ship with their own STC sub-bundles when future rule books land.

**Displaced alternatives:**

- *Flat table store with book-typed by row* — one big structured file where every row carries `source_book: RuleSetId`. Cross-book queries get noisy fast; PF1 Wizard spell list and PF1 Wizard-from-UM spell list share the school partition but have different records.
- *CRB-only without `RuleSetId`* — would require a refactor when the next book lands to add the type.

**Reason:** Per operator directive 2026-07-14: "we are doing core rules now, but will be doing more rule books as soon as this weekend... Shape A it is." Source-book subdirectories under one module is the shape that makes the next book's landing zone obvious without committing to it now. `RuleSetId` is in the type system from day one; the 13 cycles pass `RuleSetId::Crb` explicitly; future books get sibling directories (`um/`, `apg/`, etc.) and their own enum variants.

**Naming convention shift:** the 13 slice-ship fixtures shift from `sd19_seam_*` to `sd19_seam_crb_*` to leave room for next-book fixtures. The 13 per-cycle fixtures retain their cycle-specific naming (`sd19_<criterion>_*.txt`).

## Cross-reference

- `README.md` — bundle overview and posture summary.
- `acceptance-and-verification.md` — closure gates.
- `epic-breakdown.md` — 15 acceptance criteria grouped by execution lane (1 pre-loop + 9 schools + 4 categories + 1 seam-shapes-correctness check).
- `risks-and-open-questions.md` — self-healable vs. non-self-healable, the §3.5 cycle's open item-id convention history, the SD-18 §1.1 pattern this bundle inherits.
- `technical-design.md` — seam signature, resolver signatures, `CharacterInput` extensions, `MatrixSubjectType` extension, branch lifecycle, card schema.
- `technical-requirements.md` — pre-loop prerequisites (SD-18 §1.1 ship is required; both §3.4/§3.5 cycles' dated blocker entries are required reading).
- `~/workspace/SD-19-core-rules-spell-equipment-reachability-scope-draft.md` — canonical handoff.
- `~/workspace/SD-19-core-rules-spell-equipment-reachability-loop-instruction.md` — loop body.
- `~/workspace/SD-18-core-rules-breadth-progress.md` — under the headings "## cycle-2026-07-15T0300 | §3.4 spell-school reachability-chain investigation" and "## cycle-2026-07-15T0400 | §3.5 equipment-category reachability-chain investigation" (anchored headings, not line numbers).