# F7a receipt: desktop sheet numbers (worklist F7-1, F7-8, F7-2)

Branch `sd36/epic-f7-sheet-visible`, cut from tranche/16 `7b240e8fb5` (stage-open commit
`809b95f769`). This step covers worklist items F7-1 (ability scores), F7-8 (Elowen's skill points)
and F7-2 (the Weapons tab's "Also proficient with" line). Logs for every RED and GREEN run named
here are in this directory (`f7a-*.log`).

## F7-1: the Abilities panel prints the engine's score

**Defect.** `CharacterSheet.tsx` `scoreFromModifier` printed `10 + 2 x modifier`. That prints every
odd score one too low.

**What the engine already served.** `load_saved_character` has carried `abilityScores` since v0.8
B-2 (`effective_ability_scores_dto`, which is `apply_human_ability_bonus` applied to the stored
scores). These are the same scores `compute_pilot_base_chassis` derives every modifier from. The
TypeScript wire type did not declare the field, so the sheet never read it. No DTO change was
needed at the engine boundary.

**One rule for every race.** A Human's stored score does not include the racial bonus, so the
engine adds +2 to the chosen ability. For every other race, the create form
(`composeCreateCharacterRequest.ts`) has already applied the fixed or chosen adjustment before the
score is stored, and the engine passes it through unchanged. In both cases the printed score is
`abilityScores[key]`. The engine does not model level-based ability increases, so none are printed.

**Change.** `abilityScoresModel.ts` `printedAbilityScore` prints the served score, or a dash when no
read has landed yet. It never reconstructs the score from the modifier. `AbilitiesPanel` reads
`engineRecords.abilityScores`, which is re-read on mount and after every mutation, like the other
engine records. `scoreFromModifier` is deleted.

| Character | Ability | Stored | Engine score | Printed before | Printed after |
|---|---|---|---|---|---|
| Elowen (Human Wizard 5, +2 Int) | Con | 13 | 13 | 12 | 13 |
| Elowen | Int | 16 | 16 + 2 Human = 18 | 18 | 18 |
| Aldric (Human Fighter 3, +2 Str) | Str | 17 | 17 + 2 Human = 19 | 18 | 19 |
| Aldric | Dex | 13 | 13 | 12 | 13 |
| Dwarf Fighter 1 (test build) | Con | 15 (13 + 2 Dwarf) | 15, modifier +2 | 14 | 15 |
| Dwarf | Wis | 13 | 13 | 12 | 13 |

**RED to GREEN.**
- `abilityScoresModel.test.ts`: RED because the module did not exist (`f7a-red-ts.log`). GREEN in
  `npm test`.
- `character_hub::starter_seed_tests::seeded_and_created_characters_load_the_engines_effective_scores`
  pins the served scores in the table above, for both seeds and for a Dwarf built through
  `create_character_at_root`. It passed from the start, because the engine side was already
  correct. The defect was entirely in what the sheet printed.

## F7-8: Elowen's skill points, and Aldric's

**Defects found.**
1. The sheet never read the persisted allocation. `CharacterSheet.tsx` seeded its state from the
   constant `DEFAULT_SKILL_ALLOCATION` (Climb, Intimidate, Swim at 1) on every open.
   `load_saved_character` already returned `skillAllocations` (v0.8 B-3), but the TypeScript type
   did not declare it.
2. The unallocated count used the 3.5 rule. `skillRankCost` charged 2 points for a cross-class rank.
   In PF1 (CRB Chapter 4, Acquiring Skills) one point buys one rank, class skill or not; a class
   skill instead adds +3. Elowen's reported 29 unallocated was 35 − (3 cross-class ranks × 2).
3. The engine refused any real allocation. `unmet_selected_skill_posture_conditions` and its corpus
   twin `compute_selected_skill_modifiers_from_corpus` raised the claim-blocking
   `skill.selected_modifier.unsupported` for any rank in a skill other than Climb, Intimidate or
   Swim. A seed with a single rank in Spellcraft failed with `did not compute (blocking:
   ["skill.selected_modifier.unsupported"])` (`f7a-green-attempt-blocked.log`).

**Changes.**
- Frontend. `loadSavedCharacterDetail.ts` declares `abilityScores?` and `skillAllocations?`. The
  Skills panel and the allocation dialog read the persisted allocation (`allocationFromPersisted`,
  re-read with the engine records) and write it back with `persistedFromAllocation`. An id with no
  panel row keeps its wire id, is counted as spent, and is written back unchanged.
  `skillPointsSpent` counts one point per rank. `DEFAULT_SKILL_ALLOCATION` and `skillRankCost` are
  deleted.
- Engine. Both twins stop refusing ranks in skills outside Climb, Intimidate and Swim. Those ranks
  feed none of the three totals the slice computes; each total reads only its own skill's ranks.
  The GE-06 posture on the three skills themselves (exactly rank 1 each, Chain Shirt equipped) is
  unchanged.
- Seeds. `StarterSeed.skill_ranks` goes through `pf1_adapter::apply_set_skill_allocations`, the same
  replace-wholesale input the "Manage skill allocation" dialog writes through
  `set_skill_allocations`. The saved file is not hand-edited.

**Hand-worked numbers.** Earned points are the class's ranks per level (read from the converted
package by `class_skill_ranks`: Wizard 2, Fighter 2), plus the Int modifier, plus 1 for the Human
Skilled trait, for each level. The maximum ranks in one skill equal the character level (CRB
Chapter 4).

| Seed | Earned | Allocation (ranks) | Spent | Unallocated |
|---|---|---|---|---|
| Elowen, Human Wizard 5, Int 18 (+4) | (2 + 4 + 1) × 5 = **35** | Climb 1, Intimidate 1, Swim 1 (GE-06 posture, cross-class); Spellcraft 5, Knowledge (arcana) 5, Knowledge (planes) 5, Knowledge (dungeoneering) 5, Knowledge (religion) 5, Fly 5, Linguistics 2 (Wizard class skills, per the engine's class-skill reader) | 3 + 32 = 35 | **0** (was 29) |
| Aldric, Human Fighter 3, Int 14 (+2) | (2 + 2 + 1) × 3 = **15** | Climb 1, Intimidate 1, Swim 1 (GE-06 posture, Fighter class skills); Ride 3, Survival 3, Knowledge (dungeoneering) 3, Knowledge (engineering) 3 | 3 + 12 = 15 | **0** (was 12) |

Elowen's Spellcraft is 5 ranks + 3 (class skill) + 4 (Int) = **+12**.

**RED to GREEN.**
- `elowen_loads_with_zero_unallocated_skill_points` and
  `aldric_loads_with_zero_unallocated_skill_points` were RED with spent `3` against earned `35` and
  `15` (`f7a-red-desktop-seeds.log`). They are GREEN in the full desktop run.
- `skillsModel.test.ts` (Elowen's persisted allocation leaves 0 unallocated and Spellcraft is +12; a
  cross-class rank costs 1 point; an unlisted id round-trips): RED on a missing export
  (`f7a-red-ts.log`), GREEN in `npm test`.

**Finding: the create path's fixed ranks (GE-06 posture), counted and not changed.**
`compose_character_input` places 1 rank each in Climb, Intimidate and Swim on every created
character, regardless of class. Measured over the 59-class roster at level 1
(`the_create_paths_fixed_ranks_measured_over_the_roster`):

- 22 of 59 classes hold all three as class skills.
- 28 of 59 hold at least one of the three as cross-class.
- 9 of 59 have no class-skill answer (Unknown).

The engine computes those three totals only at rank 1, so a player who moves any of them off rank 1
still gets a Blocked build. Mechanism: `require_selected_skill_rank`.

**Finding: 3.5 max-rank caps, not changed.** The allocation dialog (`maxClassSkillRanks` =
level + 3, `maxCrossClassSkillRanks` = half of that) and the engine's `allocate_skill_ranks` caps
(non-blocking diagnostics) still apply the 3.5 caps. PF1 caps every skill at the character level.
Neither seed exceeds the PF1 cap.

## F7-2: "Also proficient with" lists weapon records only

**One rule** (`class_facts_sheet_rules::WeaponRecordNames`). A granted weapon-proficiency name
prints only when it is the label of a converted equipment record that is tagged `Weapon` and carries
a proficiency category (`Simple`, `Martial` or `Exotic`; CRB Chapter 6 puts every weapon in one of
the three). A name spelled `Base (Qualifier)` is looked up by its record label `Qualifier Base`: for
example `Sword (Short)` is looked up as `Short Sword` and `Crossbow (Light)` as `Light Crossbow`.
This is the one spelling difference between the proficiency and equipment record families. The
converted equipment records do not carry their `PROFICIENCY` join, so the label is the only link
available without a converter step.

The rule applies to named weapons and weapon-set members, from both the static rows and the
converted-record reader. Conjunction selectors such as `Martial Ranged` are not weapon names and
are kept. A set whose members all drop is itself dropped, so its bare label never prints.

**Unarmed Strike.** The corpus does hold `Unarmed Strike` as a Weapon-tagged record, but it is
tagged `Special` with no proficiency category, exactly like `Flurry of Blows`. The one rule
therefore drops both. Every character is proficient with unarmed strikes; PCGen grants it through
the `Auto` weapon-proficiency type. It is not a class grant, so it does not belong in a class's
list.

**Measured over every roster class at levels 1 and 7** (118 rows: the F6a wire,
`stage-f6/f6a-class-facts-wire.json`, before and after):

| | Before (tranche/16 `7b240e8fb5`) | After |
|---|---|---|
| Names printed | 466 | 302 |
| Distinct names | 69 | 62 |
| Non-weapon names printed | 164, on 21 of 59 classes | 0 |

The 7 distinct names dropped are Flurry of Blows, Grapple, Mind Blade, Spells (Ray), Spells
(Touch), Splash Weapon and Unarmed Strike. The 21 classes whose line changed are adept, aegis,
antipaladin, aristocrat, commoner, cryptic, dread, expert, magus, marksman, monk, ninja, psion,
psychic_warrior, samurai, soulknife, tactician, unchained_monk, vitalist, warrior and wilder. No
other field of the wire moved (the tiers, groups, printed conditions, caster level, class skills
and hit die are unchanged for all 118 rows).

- **Monk 1**, 17 names: Club, Crossbow (Light), Crossbow (Heavy), Dagger, Handaxe, Javelin, Kama,
  Nunchaku, Quarterstaff, Sai, Shortspear, Sword (Short), Shuriken, Siangham, Sling, Spear and
  Sword (Temple). Flurry of Blows and Unarmed Strike are gone.
- **Magus 1** prints no named weapon beyond its Simple and Martial tiers. Its `Auto` set was
  entirely non-weapons, so the set is gone.

**RED to GREEN.**
- `class_facts_sheet_rules::tests::{monk_prints_only_weapon_records, magus_prints_no_pseudo_weapons,
  every_roster_class_prints_only_weapon_records}` were RED against the unchanged rule (3 failed,
  `f7a-red-engine-weapons.log`) and are GREEN in the root run. The roster test pins (302, 62).
- `classFactsModel.test.ts` `verifiesNoRosterClassPrintsAPseudoWeapon`: RED on the F6 wire
  ("Magus ... got Grapple, Spells (Ray), Spells (Touch), Splash Weapon, Unarmed Strike",
  `f7a-red-ts.log`), GREEN on the regenerated wire.
- The F6a wire artifact was rewritten with `CODEX_WRITE_F6A_WIRE=1` because it is the live pin of
  `list_class_facts`. Its diff is only the weapon names above.

## Other test updated

`crates/codex-ingest/tests/sd20_tabletop_readiness_integration.rs`
`tabletop_readiness_selected_skill_posture_deviation_no_longer_blocks_skill_cells` used an extra
Diplomacy rank to trip the posture diagnostic. That no longer trips it, so the test now uses 2
ranks in Climb, which still does. Its assertions are unchanged.

## Verification (commands and denominators)

All runs were in worktree `codex-epic-f7` with
`CARGO_TARGET_DIR=/home/ubuntu/workspace/worktrees/codex-epic-f7-target`, one cargo process at a
time.

| Command | Result | Log |
|---|---|---|
| `npx tsc --noEmit` (apps/desktop) | exit 0 | `f7a-desktop-npm.log` |
| `npm test` (apps/desktop) | 132 of 132 test files passed | `f7a-desktop-npm.log` |
| `cargo test --locked -j 8 --lib -- skill class_facts_sheet_rules --test-threads=8 --nocapture` | 155 passed, 0 failed (2,599 filtered out). This ran before the out-of-slice engine change; the workspace run below covers the library after it | `f7a-green-root-lib.log` |
| `cargo test --locked -j 8 --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8 --nocapture` (the whole desktop crate: the seeds, `character_hub`, `class_facts`, `pf1_adapter`) | 637 passed, 0 failed | `f7a-suite-desktop.log` |
| `cargo test --locked -j 8 --workspace --no-fail-fast -- --test-threads=8` | 461 test targets; 8,183 passed, 1 failed. The failure was `tests/ge06_pilot_selected_skill_modifiers.rs` `widened_selected_skill_allocation_blocks_skill_modifiers`, which pinned the refusal this step removes | `f7a-suite-workspace.log` |
| `cargo test --locked -j 8 -p codex --test ge06_pilot_selected_skill_modifiers -- --test-threads=8`, after rewriting that test as `an_allocation_outside_the_slice_leaves_the_three_totals_unchanged` (out-of-slice Stealth 1: no refusal; the three totals and their three explanations equal the unwidened fixture's) | 5 of 5 passed | `f7a-green-ge06-rerun.log` |

Every RED log listed above predates its fix.

`data/corpus/**`, `site/**` and `data/sheet_rules/**` are untouched; this is not a converter step.
The desktop app was not launched, so the ui-smoke harness did not run.
