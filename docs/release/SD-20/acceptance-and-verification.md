---
title: SD-20 — Acceptance and Verification (Closure Gates)
status: approved (operator review 2026-07-16; changes noted: §2 broadened to any class/any level, Q2 revised to class-selection trigger mechanic, Q3 revised to print-ready data; SD-20 launches on tranche/4 branch)
date: 2026-07-15
companion_to: /home/ubuntu/workspace/programs/codex/requirements/SD-20-rules-engine-completeness/decisions.md
---

# SD-20 — Acceptance and Verification

SD-20 closes when every closure gate below is met AND a `tranche/4 → develop` promotion PR has been merged. Per operator directive 2026-07-16: SD-20 launches on `tranche/4`; the `tranche/3 → develop` promotion PR is the chassis-lane promotion, not the per-character-rules-engine-lane promotion. Each gate is independently verifiable; the verification command is the contract.

## Closure gates (mandatory)

1. **Foundation closed**: Tranche-3's three bundles (SD-18 chassis, SD-19 corpus-aware seam + canonical Paizo-table store, SD-19 §3.4/§3.5 acceptance criteria) all show `done` in the shared progress doc `~/workspace/SD-18-core-rules-breadth-progress.md`.

2. **Boundary contract lands**: A `boundary-contract.md` artifact exists in `docs/SD-20/` (or equivalent) and names every `CharacterInput` shape, every `PilotReceipt` field, and every printed-sheet cell. Epic-1's capability slice landed as a single atomic commit per SD-19 §1 pattern; `cargo test --locked` is green with zero SD-18/SD-19 regressions.

3. **Wire-fixture parity tests land**: At least one golden JSON fixture per epic (8 fixtures minimum, one for boundary contract, one each for spellbook / feat prereqs / skill ranks / equipment effects / damage total / Level Up grants / integration closure). Each fixture is a complete `CharacterInput` paired with the exact `PilotReceipt` the engine must produce. Both the engine's RED tests and the GUI's render tests read the same fixtures.

4. **Spellbook engine grounded (epic 2)**: The 9 strict-school PF1 spell schools each route from `CharacterInput.spells_selected` through the engine to a non-empty `SpellbookCoverage` in the `PilotReceipt`. Spell save DCs are computed per the engine (not hardcoded in fixtures). `class_spell.<class>.<burden>.unsupported` diagnostics no longer fire claim-blocking for any of the 9 strict schools in CRB scope.

5. **Feat prerequisite engine grounded (epic 3)**: A user-selected feat that satisfies all prerequisites produces a non-empty `FeatEffects` in the `PilotReceipt`; a feat that fails a prerequisite produces a non-empty `PrerequisiteEvaluation.failing_prerequisites`. The feat catalog covers every feat in CRB's feat tables.

6. **Skill-rank allocation engine grounded (epic 4)**: A user-allocated skill distribution produces `SkillTotals` whose per-skill totals match what the chassis + user-allocated + cross-class-penalty rules would yield. PF1's max-rank cap (character-level + 3 for class skills, character-level / 2 rounded-up for cross-class) is enforced; cap violations produce diagnostics not fabricated totals.

7. **Equipment-effect engine grounded (epic 5)**: A `CharacterInput` with a full equipment loadout produces a populated `EquipmentEffects` whose per-item stats come from `src/rules_core/rules_tables/crb/equipment_tables.rs` (the foundation slice). Every CRB equipment category (`arms_armor`, `general`, `magic_items`, `equipmods`) reaches the engine end-to-end. SD-19's bounded baseline (AC, attack bonus, max dex, spell failure) is a strict subset of what epic 5 produces — epic 5 extends the baseline to every field on every Paizo equipment entry.

8. **Damage-total engine grounded (epic 6)**: A weapon attack produces a `DamageRoll` whose base dice come from the weapon's equipment entry and whose damage modifier sums STR mod + weapon enhancement + relevant feat effects (read from epic 3's outputs). Critical threat range and critical multiplier come from the weapon entry. PF1's critical rules (×2 default, ×3 for keen, etc.) are enforced.

9. **Level Up grant model grounded (epic 7)**: Advancing a `CharacterInput` from level N to level N+1 produces a `LevelUpPlan` whose `automatic_features` and `pick_from_lists` match the published CRB table at level N+1 for the character's class(es). The plan cites each grant's source via `TableCellRef` (the foundation slice's table-cell reference). When the user picks selections (free feats, spells known, ASI allocation, etc.) and provides them in the next `CharacterInput`, the engine produces the updated receipt without re-fabricating any feature already auto-granted.

10. **Tabletop-readiness integration closure (epic 8 — the load-bearing gate)**: Any of the 11 core classes (Barbarian, Bard, Cleric, Druid, Fighter, Monk, Paladin, Ranger, Rogue, Sorcerer, Wizard) at any level 1-20 with its **canonical first-build state** — feats = the class-appropriate first-feat pick from epic 3 (e.g. "any Combat feat" for Fighter, "Shield Focus" or similar for a caster), skill ranks = class skill allocation from epic 4 (e.g. 2 + Int mod ranks for Fighter, class-skill + cross-class mix for casters), equipped = class-appropriate starting equipment from epic 5 (e.g. longsword + chain shirt for Fighter, dagger + quarterstaff for Sorcerer), prepared/known = class-appropriate spell selections from epic 2 (none for non-spellcasters; the class's starting cantrips + 1st-level spells known/prepared for casters), at the chosen level — produces a `PilotReceipt` whose every displayed sheet cell matches the table cells referenced by `TableCellRef`s. The integration test fixture set covers one canonical character per core class at level 1 (11 fixtures) plus a smaller sample of higher-level characters that exercise multi-level mechanics (one Fighter at level 4/8/12/16/20 to ground feat-pick-at-level cadence; one Wizard at level 5/10 to ground spell-pick cadence; one Cleric at level 20 to ground capstone; one multiclass character e.g. Fighter 2 / Wizard 1 at total level 3 to ground per-class-level grant layering). Each fixture's `expected_output` matches the values printed by Pathbuilder 2e for the same character. The cycle is real: a user can hit "Print Sheet" in the GUI and take the output to a real PF1 table. Per operator directive 2026-07-16: "decisions §2 says a level-1 fighter. in reality, it should be any class, any level."

11. **`tranche/4 → develop` promotion PR opened**: Operator opens the promotion PR per the existing cadence. This is operator-driven; the bundle ships the GitHub-side artifacts (branch, merge-friendly commit history, sweep of any audit-trail comments per codex-tranche-2-5 respawn-guard pattern). Per operator directive 2026-07-16: the promotion branch is `tranche/4 → develop`, not `tranche/3 → develop`.

## Verification at closure

The closure posture is reviewable entirely from these surfaces:

- `~/workspace/SD-18-core-rules-breadth-progress.md` (shared progress doc) — SD-18 §X (chassis done), SD-19 §Y (corpus reachability done), SD-20 §Z (per-epic done with commit SHAs and card IDs).
- `./decisions.md` — the decision record; future sessions reconstruct the bundle shape from this alone.
- The boundary contract artifact at `docs/SD-20/boundary-contract.md`.
- The wire-fixture parity test fixtures at `tests/fixtures/wire/sd20/`.
- **git log --oneline tranche/4 -N** — the merge history into the integration branch.
- `codex-tranche-4` board — post-loop populated ledger, every SD-20 epic-card `status=done`, with merge receipts and audit-grade context per codex-tranche-2-5's respawn-guard pattern (audit comment at merge time, receipt comment naming the disk-truth).

The operator's first action on return from a multi-day run: read the `## SD-20 cycles` section of the shared progress doc; if empty, gates 1–9 above are the entire verification.

## What does *not* gate closure

- Loop's cycle log size (10 cycles or 100; criterion is the criterion, not volume).
- Number of self-heals applied during the run (zero or many; self-heals are the normal operating mode).
- Whether some epic-cards land as documentation-only versus full code-bearing (per the eligibility check — a school or category may legitimately land as a doc-only entry if the engine proves sufficient to ground the corpus-derived contribution in a recognition-only form).
- Spell *effects* beyond the engine's spellbook output — i.e. spell descriptions rendered as prose paragraphs rather than computed dice expressions. (Spell effect *text* is the canonical Paizo table cell; the engine renders it; computed dice within the prose is documented in epic 2's seam signature.)
- Equipment *effects* beyond the bounded baseline that SD-19 closed on (SD-19 closed at AC / attack / max dex / spell failure; epic 5 extends to every field on every Paizo equipment entry, but those extensions are part of SD-20's tabletop-readiness, not a separate gate from the bounded-baseline contract).

## Cross-reference

- `decisions.md` — the 9-item decision record (Tabletop-readiness posture, per-character scope, SD-21 promotion, etc.).
- `epic-breakdown.md` — 15 acceptance criteria grouped into 8 epics.
- `risks-and-open-questions.md` — self-healable vs. non-self-healable split + open override flags.
- `technical-design.md` — per-epic seam signatures, boundary contract shape, wire-fixture parity test format.
- `technical-requirements.md` — pre-loop prerequisites for SD-20.
- `./scope-draft.md` — canonical handoff.
- `./loop-instruction.md` — loop body.
- `~/workspace/programs/codex/requirements/SD-18-core-rules-breadth/` — chassis grounding.
- `../SD-19/` — corpus-aware seam + canonical Paizo-table store.
- `../SD-21/` (sibling bundle, parallelizable) — campaign manager + Drive persistence + APG + ACG ingestion.
