---
title: SD-18 — Acceptance and Verification
status: draft (operator review required)
date: 2026-07-12
companion_to: /home/ubuntu/workspace/SD-18-core-rules-breadth-scope-draft.md
---

# SD-18 — Acceptance and Verification

SD-18 closes when every acceptance criterion in the scope doc's §3 satisfies its criterion AND every row of the SD-13 matrix reaches the highest tier achievable from corpus coverage.

## Closure gates (mandatory)

1. **Pre-loop gate shipped.** §1.1 of the scope doc (consumer-side composition) has merged to `tranche/3`. This is the binary prerequisite — no §3 acceptance criterion is verifiable until §1.1 lands.

2. **Race rows at `supported/Product-visible`** (7 rows). For each of the 7 core races (Dwarf, Elf, Gnome, Half-Elf, Half-Orc, Halfling, Human), the SD-13 matrix row reads `support_state=Supported` and `evidence_tier=Product-visible`. Per row, an end-user-visible character build must demonstrate the race at L1, L10, and L20 against its exemplar class. Grounding artifacts cite the merge SHA on `tranche/3` and the loop cycle's card id on `codex-tranche-3`.

3. **Class rows at `supported/Product-visible`** (11 rows). For each of the 11 core classes (Barbarian, Bard, Cleric, Druid, Fighter, Monk, Paladin, Ranger, Rogue, Sorcerer, Wizard), the SD-13 matrix row reads `support_state=Supported` and `evidence_tier=Product-visible`. Per class, an end-user-visible character build demonstrates L1, L10, L20 against its exemplar race with at least one leveled class feature or spell that has measurable effect on derived stats.

4. **Interaction rows advanced** (2 rows). For each of the 2 SD-13 interaction rows (Human bonus feat / ability-bonus seam; non-Human race × class progression beyond pilot), the row's matrix state advances one or more tiers per the SD-13 quality gate vocabulary. `partial/Computed` is accepted only with explicit grounding artifacts.

5. **Spell school cards landed** (9 cards). For each PF1 strict school (Abjuration, Conjuration, Divination, Enchantment, Evocation, Illusion, Necromancy, Transmutation, plus Universal), every spell in the school's slice parses via SD17-B-4, lands in `SourcePackageContent`, is reachable from a chosen `CharacterInput` via class spell list, and is consumable by the rules engine. End-user-visible character at L10 of a casting class demonstrates the school.

6. **Equipment category cards landed** (4 cards). For each of the four `core_rulebook/cr_equip_*.lst` files (`arms_armor`, `general`, `magic_items`, `equipmods`), a representative sample of items parses via SD17-B-5, lands in `SourcePackageContent`, is selectable from `CharacterInput`, and affects derived stats when equipped. End-user-visible character demonstrates the category.

7. **Progress doc reflects every criterion as satisfied.** The progress doc structured per scope doc §4.4 shows every acceptance criterion in §3 as `done` with row id, branch, merge SHA, and card id.

8. **Kanban board populated.** `codex-tranche-3` shows the post-loop populated ledger, every card `status=done`, with merge receipts and audit-grade context per the §4.3 schema.

9. **`tranche/3 → develop` promotion PR opened.** Operator-driven, per the existing promotion cadence (matches the tranche-2-5 → develop and tranche-2-7 → develop patterns).

## Verification at closure

The closure posture is reviewable entirely from four surfaces:

- The scope doc (`/home/ubuntu/workspace/SD-18-core-rules-breadth-scope-draft.md`).
- The progress doc (parallel to scope doc, defaults to `/home/ubuntu/workspace/SD-18-core-rules-breadth-progress.md`).
- The technical-design doc (`/home/ubuntu/workspace/programs/codex/requirements/SD-18-core-rules-breadth/technical-design.md`) for parser/code paths and seam function inventory.
- `codex-tranche-3` board (post-mortem records).
- `git log --oneline tranche/3 -N` (the merge history).

No additional reconstruction needed. Operator's first action on return from a multi-day run: read `## Open blockers` in the progress doc. If empty, §1 + §3-§7 above are the entire verification.

## What does *not* gate closure

- Loop's cycle log size (the loop may have run 5 cycles or 100; the criterion is the criteria, not the volume).
- Number of self-heals applied during the run.
- Whether the pre-loop card landed in 1 cycle or 5 cycles.
- Whether some cards landed as documentation-only versus full code-bearing.
