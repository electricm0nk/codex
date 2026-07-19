---
title: SD-20 — Epic Breakdown
status: approved (operator review 2026-07-16; changes noted: §2 broadened to any class/any level, Q2 revised to class-selection trigger mechanic, Q3 revised to print-ready data; SD-20 launches on tranche/4 branch)
date: 2026-07-15
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-20-rules-engine-completeness/decisions.md
---

# SD-20 — Epic Breakdown

Maps the 15 acceptance criteria for SD-20 (per-character tabletop-readiness) into 8 epics inside the SD-20 bundle. Each epic has its own acceptance criteria; each epic lands via the same loop-routed-cycle pattern SD-19 used (one capability slice, then loop cycles that ground a representative sample per criterion).

## Execution lane split

```
Per-character epic capability slices (each an atomic direct commit, per
the no-branches convention from decisions.md §6):
- Epic 1: Boundary contract + wire-fixture parity tests       (gates 2, 3)
- Epic 2: Spellbook engine                                    (gate 4)
- Epic 3: Feat prerequisite engine                            (gate 5)
- Epic 4: Skill-rank allocation engine                       (gate 6)
- Epic 5: Equipment-effect engine                             (gate 7)
- Epic 6: Damage-total engine                                 (gate 8)
- Epic 7: Level Up grant model                                (gate 9)
- Epic 8: Tabletop-readiness integration closure              (gate 10)
```

Total: **15 acceptance criteria grouped into 8 epics + 1 boundary-contract gate + 1 promotion-gate.**

## Linear dependency (per decisions.md §7)

```
Epic 1 (boundary contract + parity tests)
  ├── Epic 2 (spellbook) — depends on SD-19 table store + boundary contract
  ├── Epic 3 (feat prereqs) — depends on SD-19 table store + boundary contract
  ├── Epic 4 (skill ranks) — depends on SD-19 table store + boundary contract
  ├── Epic 5 (equipment effects) — depends on SD-19 table store + boundary contract
  │     └── Epic 6 (damage total) — depends on epic 5 outputs + boundary contract
  ├── Epic 7 (Level Up grants) — depends on epics 2, 3, 4, 5, 6 outputs + boundary contract
  └── Epic 8 (integration closure) — depends on every other epic's outputs
```

Epics 2, 3, 4, and 5 are independent of each other and can run as concurrent loops if the operator chooses to host them that way. Three independent lanes: Epic 2 (spellbook), Epic 3 (feat prereqs), Epic 4 (skill ranks) + Epic 5 (equipment effects) as a paired cycle since Epic 5's outputs feed Epic 6. Epic 6 (damage) is sequentially after Epic 5 (equipment). Epic 7 (Level Up grants) integrates after Epics 2–6 close. Epic 8 (integration closure) is the integration milestone.

Campaign manager + Drive persistence + APG + ACG ingestion are NOT epics inside SD-20. They're promoted to their own top-level bundle SD-21 (per operator directive 2026-07-15; see `decisions.md` §1).

## Acceptance criteria (15, per epic)

### Epic 1 — Boundary contract + wire-fixture parity tests

1. **The `docs/SD-20/boundary-contract.md` artifact exists** and names every `CharacterInput` shape the engine accepts, every `PilotReceipt` field the engine returns, and every printed-sheet cell the GUI renders.
2. **The `tests/fixtures/wire/sd20/` directory contains at minimum eight golden JSON fixtures** — one for the boundary contract itself, one each for spellbook, feat prereqs, skill ranks, equipment effects, damage total, Level Up grants, and integration closure. Each fixture is a complete `CharacterInput` paired with the exact `PilotReceipt` the engine must produce.
3. **Both the engine and the GUI consume the same fixture files**; if either side diverges from the file, tests fail. The wire-fixture parity tests are the single dovetail mechanism between engine and GUI.

### Epic 2 — Spellbook engine

4. **Spell save DCs are computed by the engine from class level + casting stat modifier + school specialization, not hardcoded**. The engine's `compute_spellbook_coverage` returns `spell_save_dc: BTreeMap<ClassId, u8>` for every class that appears in the character's `class_summary`.
5. **Spell *effects* (text, target, duration, save throw, spell resistance) come from the CRB table cell** the spell lives in, not from a fabricated or invented description. The receipt carries `TableCellRef` provenance for every spell `PreparedSpell` and `KnownSpell`.
6. **Bonus slots from high ability** (Int for Wizard/Sorcerer, Wis for Cleric/Druid/Bard, Cha for Sorcerer separately if Unearthed Arcana applies) are computed per the engine's `bonus_slots_from_ability` field, with the spell-level-vs-modifier math per the spellbook engine epic's seam signature.

### Epic 3 — Feat prerequisite engine

7. **Every feat in CRB's feat tables** is in the engine's feat catalog (`src/rules_core/rules_tables/crb/feats/...` or equivalent). Feat eligibility is computed per `evaluate_feat_prerequisites`; failure paths are reported in `PrerequisiteEvaluation.failing_prerequisites` with the specific prerequisite that failed.
8. **Chosen feats apply their effects to the receipt** via `compute_feat_effects`. The receipt carries the per-feat contribution so the GUI renders the post-feat sheet correctly. Epic 3 reads from epic 5's equipment outputs (for weapon focus / specialization entries) and from epic 2's spellbook outputs (for spell-related feats); the dependency is one-way (epic 3 consumes), no cycle.

### Epic 4 — Skill-rank allocation engine

9. **Per-level skill rank totals respect PF1's max-rank cap**: class skills max at character level + 3, cross-class skills max at (character level + 1) / 2 rounded up. Cap violations produce diagnostics, not fabricated totals. The receipt's `SkillTotals.cross_class_penalty_applied: true` records that the cross-class penalty was correctly applied.
10. **Class skill bonuses** (Ranger's +2 to specific skills, Bard's kit bonuses, etc.) flow from epic 5's class-feature tables (where SD-19 reads them) into the receipt's `SkillTotal.class_skill_bonus` field.

### Epic 5 — Equipment-effect engine

11. **Every CRB equipment category** (`arms_armor`, `general`, `magic_items`, `equipmods`) reaches the engine end-to-end via `compute_equipment_effects`. The receipt's `EquipmentEffects.per_item` carries every `ResolvedEquipment` the character equipped; every field on each `DerivedEquipmentStats` (the four bounded-baseline fields from SD-19 plus the additional fields the CRB table cell defines) is populated for that item if non-default.
12. **Aggregate equipment effects** (`armor_class_delta`, `attack_bonus_delta`, `max_dex_cap`, `spell_failure_chance`) are computed from the per-item stats. Magic weapons with enhancement bonuses contribute to `attack_bonus_delta`; armor contributes to `armor_class_delta` and `max_dex_cap` and `spell_failure_chance`.

### Epic 6 — Damage-total engine

13. **Weapon damage rolls** come from the weapon's CRB damage entry (the foundation slice's `equipment_tables.rs`), not from a fabricated or hardcoded damage expression. The receipt's `DamageRoll.base_dice` cites the weapon's KEY via the `TableCellRef` provenance. Damage modifier sums STR + weapon enhancement + relevant feat effects (read from epic 3's outputs).
14. **Critical hits** are computed correctly: ×2 default, ×3 for keen or other sources, ×4 for some specific weapons per CRB. Critical damage is the base-dice rolled again plus the modifier (no added modifier on the additional dice).

### Epic 7 — Level Up grant model

15. **Advancing a `CharacterInput` from level N to level N+1 produces a `LevelUpPlan` whose `automatic_features` and `pick_from_lists` match the CRB table cell at level N+1 for the character's class(es)**. The plan cites each grant's source via `TableCellRef`. Multiclass characters get per-class grants layered correctly (Fighter 2 / Wizard 1 at total level 2 gets Fighter-class feature, not Wizard's per-class feature). User picks (free feats, spells known, ASI allocation) become part of the next `CharacterInput`; the next receipt reflects them without re-fabricating already-granted features.

### Epic 8 — Tabletop-readiness integration closure

The full pipeline end-to-end for the canonical tabletop scenario (any of the 11 core classes at any level 1-20 per the broadened acceptance criterion — fixtures per `acceptance-and-verification.md` gate 10) is covered by the wire-fixture parity tests in the integration fixture set. Epic 8 is not a new epic; it's the closure criterion that every other epic's output composes correctly. The 15 acceptance criteria above are the closure-gate inputs; gate 10 in `acceptance-and-verification.md` is the closure-gate output.

## Cycle ordering (operator-prioritized)

The operator can prioritize per the dependency graph. Default ordering:
1. Epic 1 — Boundary contract + parity tests
2. Epic 2 — Spellbook engine (3 + 4 + 5 lanes can run concurrent here)
3. Epic 3 — Feat prerequisite engine (concurrent with 2)
4. Epic 4 — Skill-rank allocation (concurrent with 2, 3)
5. Epic 5 — Equipment effects (concurrent with 4)
6. Epic 6 — Damage total (sequential after 5)
7. Epic 7 — Level Up grants (after 2–6)
8. Epic 8 — Integration closure (after every other)

If the operator hosts three concurrent loops: Epic 2 on lane A, Epic 3 on lane B, Epics 4 + 5 (paired) on lane C. Epic 6 waits for lane C. Epic 7 waits for all three lanes. Epic 8 waits for everything.

## Cycle unit definition

A single loop cycle within an epic lands one acceptance criterion (or one representative sample for that criterion). Each cycle:

1. Picks one acceptance criterion from the epic's open list.
2. Verifies the working tree is on `tranche/4` (no feature branches; per the SD-19 no-branches convention).
3. Reads the wire-fixture parity test for that criterion (or for the criterion's representative sample).
4. Implements the smallest change that makes the receipt match the fixture.
5. Runs `cargo test --locked` (zero regressions) and `cargo clippy --locked --tests -- -D warnings` (clean).
6. Commits directly to `tranche/4` with a `feat(sd20): <criterion> (<row transition>)` message — no PR, no ephemeral branch (per the no-branches convention; per operator directive 2026-07-16: SD-20 launches on `tranche/4`, not `tranche/3`).
7. Mints a kanban card on `codex-tranche-4` as a post-mortem record (`status=done`, with merge receipt, audit-trail comment per the codex-tranche-2-5 respawn-guard pattern).
8. Updates the shared progress doc's `## SD-20 cycles` section (cycle-id, commit SHA, card id, evidence transition).
9. Exits.

A cycle is a *unit of post-mortem*, not a unit of delivered scope. One cycle, one criterion, one card, one commit. The cycle log in the progress doc plus the cards on the board let a 3-day-later operator reconstruct any specific cycle.

## What the breakdown does not specify

- Per-criterion implementation approach — the loop picks the smallest change that satisfies the criterion.
- Per-criterion TDD structure (mirrors SD-18 / SD-19's red-green-refactor pattern).
- Per-criterion timing — depends on corpus size, parser friction, behavior complexity; the loop's self-healing handles friction.
- Whether the wire-fixture parity test for an epic lands as part of the epic's capability slice or as part of the integration closure — the operator may override this per `risks-and-open-questions.md` Flag A.

## Cross-reference

- `decisions.md` — the 9-item decision record (epic split, ordering, dependency reasoning).
- `acceptance-and-verification.md` — closure gates including tabletop-readiness.
- `risks-and-open-questions.md` — self-healable vs. non-self-healable split + open override flags (Flag A through Flag D).
- `technical-design.md` — per-epic seam signatures, boundary contract shape, wire-fixture parity test format.
- `technical-requirements.md` — pre-loop prerequisites.
- `./scope-draft.md` — canonical handoff.
- `./loop-instruction.md` — loop body.
