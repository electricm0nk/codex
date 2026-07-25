# SD-24 Epic 4 — Per-Class Coverage Matrix

> Extended by criteria 4.1 (CRB), 4.2 (APG), 4.3 (ACG). Each criterion's cycle appends its own section below rather than rewriting the file. This satisfies `acceptance-and-verification.md` CG-03 ("Per-class coverage matrix (4.1–4.3) covers CRB + APG + ACG"). Per-class detail files use the `class_<name>_coverage.md` naming for CRB and `<book>_<name>_coverage.md` for APG/ACG, per `content-unit-inventory.md` §3.1's per-book cycle-artifact-path column.

## CRB (criterion 4.1) — complete (Fighter+Wizard scope; full 11-class breadth forwarded as a DISCOVERED item)

This cycle's granted file-touch scope (per `loop-instruction.md` §2.4's own partition table, which — unlike `epic-breakdown.md`'s criterion 4.1 header text naming all 11 CRB classes — names only `class_fighter.rs` / `class_wizard.rs` as Epic 4's owned files) was:

- `src/rules_core/rules_tables/crb/class_tables.rs` (generic BAB/save table, shared by all 11 CRB classes — read-only, no defect found)
- `src/rules_core/level_up/fighter.rs`
- `src/rules_core/level_up/wizard.rs`

Fighter and Wizard are also the two classes Epic 5's multiclass work is scoped to (per operator directive 2026-07-21, `decisions.md` §4), so this cycle's audit is the one that actually gates Epic 5's dependency ("Epic 5 is gated on Epic 4's coverage-matrix output"). The remaining 9 CRB classes (Cleric, Rogue, Sorcerer, Barbarian, Bard, Druid, Monk, Paladin, Ranger) each already have a landed `level_up/<class>.rs` module from SD-20 Epic 7, but auditing them was outside this cycle's granted file scope; forwarded as a `## DISCOVERED` item for a follow-on cycle if full-11-class breadth is later required (not gating Epic 5, since Epic 5 is Fighter+Wizard-only).

| Class | Book | `class_features_expected` | `class_features_wired` (before) | `class_features_wired` (after) | Gap(s) | Coverage artifact |
|---|---|---|---|---|---|---|
| Fighter | CRB | 10 | 10 | 10 | none | `./class_fighter_coverage.md` |
| Wizard | CRB | 12 | 7 | 12 | `class_spell.wizard.*` (5 explanation families) silently dropped from `LevelUpPlan` — **fixed this cycle** | `./class_wizard_coverage.md` |

**Audit method:** for each class, `class_features_expected` counts the named class-feature pillars `pilot_compute.rs` itself grounds (as `ComputationExplanation` records, excluding the generic BAB/save columns already covered by `class_tables()`) for that class. `class_features_wired` counts how many of those pillars the class's `level_up/<class>.rs` module's explanation filter actually surfaces into the `LevelUpPlan.automatic_features` list. A pillar `pilot_compute.rs` does not ground at all for a class (e.g. Wizard's Arcane Bond) is excluded from `class_features_expected` entirely — it is a `pilot_compute.rs`-level scope boundary, not a `level_up` wiring gap.

**Remediation input (criterion 4.4):** Wizard's `class_spell.wizard.*` dropped-from-`LevelUpPlan` gap — fix: add `class_spell.wizard.` prefix to `is_wizard_pillar` filter in `level_up/wizard.rs` — P1, **remediated this cycle**, no carry-forward.

**Epic 5 dependency status:** Epic 5 (Multiclass, Fighter+Wizard only) may proceed — both classes' `LevelUpPlan` composers are free of known wiring gaps within this audit's scope.

## APG (criterion 4.2) — complete

**Audited:** 2026-07-21. **Source of truth:** live code (`src/rules_core/rules_tables/apg/mod.rs`'s `coverage_report()`, exercised by `tests/sd24_apg_class_coverage_audit.rs`) plus the real PCGen corpus checkout at `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/advanced_players_guide/` (commit `7f818006e371188e5717fd18d74d18a420747fc6`, 2026-06-17).

**Roster:** Alchemist, Cavalier, Inquisitor, Oracle, Summoner, Witch — the six real APG classes (Gunslinger and Magus are not real APG content in the PCGen corpus; see `rules_tables::apg` module doc comment, corrected 2026-07-19 in SD-22).

| Class | Book | Chassis rows wired / expected | Named class features wired / expected | `pilot_compute.rs` integrated | `level_up::<class>` module exists | Gap priority |
|---|---|---|---|---|---|---|
| Alchemist | APG | 20 / 20 | 0 / 24 | No | No | P1 |
| Cavalier | APG | 20 / 20 | 0 / 16 | No | No | P1 |
| Inquisitor | APG | 20 / 20 | 0 / 19 | No | No | P1 |
| Oracle | APG | 20 / 20 | 0 / 19 | No | No | P1 |
| Summoner | APG | 20 / 20 | 0 / 17 | No | No | P1 |
| Witch | APG | 20 / 20 | 0 / 7 | No | No | P1 |

**Findings:**

1. **Chassis (BAB + saves): fully wired and independently verified correct for every class.** All six classes' `class_table()` outputs were cross-checked against `apg_classes.lst`'s real `BONUS:COMBAT|BASEAB` and `BONUS:SAVE` tokens during this audit (base attack bonus progression and good/poor save assignment) — no defects found, unlike the Cleric/Druid `good_saves.fortitude` bug found and fixed during a prior CRB cycle (`level_up/fighter.rs`'s doc comment references that history). This chassis coverage was already SD-22 Epic 3 work; this cycle's contribution is turning the audit claim into executable, regression-tested code (`ApgClassId::ALL`, `class_coverage`, `coverage_report`) rather than leaving it as an unverified doc-comment assertion.
2. **Named class features (Bombs, Discoveries, Mutagen, Cavalier's Order/Challenge/Banner, Inquisitor's Judgment/Monster Lore/Bane, Oracle's Mystery/Revelations/Curse, Summoner's Eidolon/Summon Monster/Shield Ally, Witch's Hex/Patron): zero wired for any class.** This was already true and documented per-class (see e.g. `class_alchemist.rs`'s own doc comment, SD-22) — this cycle's contribution is (a) a reproducible corpus count of the expected feature-slot surface per class (`KEY:<Class> ~ ...` records in `apg_abilities_class.lst`; methodology and exact grep documented in `ApgClassCoverage`'s doc comment in `rules_tables::apg::mod`), and (b) a canary test (`zero_named_class_features_are_wired_for_any_apg_class_yet`) that fails loudly the moment any class's count changes, so this line item can't silently go stale.
3. **`pilot_compute.rs`'s live chassis dispatch (`compute_class_chassis`, the function the character-hub pilot flow actually calls) does not recognize any APG class.** Confirmed both by inspection (the function's `match` arm only recognizes `FIGHTER_CLASS_ID`/`WIZARD_CLASS_ID`) and empirically: `tests/sd24_apg_class_coverage_audit.rs`'s `apg_classes_trip_the_honest_class_chassis_unsupported_diagnostic` test drives a real, minimal `CharacterInput` for each of the six classes through `compute_pilot_base_chassis` and confirms it returns the honest, claim-blocking `class_chassis.unsupported` diagnostic with `base_attack_bonus: 0` — never a fabricated chassis number. This is the same diagnostic every non-Fighter/Wizard **CRB** class also trips today (per `tests/sd21_wizard_chassis_computes.rs`'s doc comment: only Fighter and Wizard have a `compute_<class>_chassis`), so APG classes are not a special case — the live pilot compute seam is presently Fighter/Wizard-only across every book.
4. **No `level_up::<class>` module exists for any APG class.** `src/rules_core/level_up/` (the SD-20 Epic 7 per-level automatic-feature-grant model) contains only the 11 CRB classes (`barbarian.rs` … `wizard.rs`). No APG (or ACG) class has one.

**Conclusion feeding criterion 4.5:** APG classes are *not* fully wired (chassis-only; zero named features; zero live-compute integration; zero level-up grant modules). This is the exact condition `loop-instruction.md §4.2`'s hard-stop row anticipates ("Epic 4 finds APG/ACG classes are *not* fully wired → Multiclass Epic 5 scope is restricted to Fighter + Wizard only … defer APG/ACG-class multiclass to a follow-on bundle"). No blocker is raised here — this is the expected, already-decided outcome; criterion 4.5 records the deferral formally.

**Per-class detail:** `./apg_alchemist_coverage.md`, `./apg_cavalier_coverage.md`, `./apg_inquisitor_coverage.md`, `./apg_oracle_coverage.md`, `./apg_summoner_coverage.md`, `./apg_witch_coverage.md` (per `content-unit-inventory.md` §3.1's cycle-artifact-path column).

## ACG (criterion 4.3) — complete

**Audited:** 2026-07-21. **Source of truth:** live code (`src/rules_core/rules_tables/acg/mod.rs`'s `coverage_report()`, exercised by `tests/sd24_acg_class_coverage_audit.rs`) plus the real PCGen corpus checkout at `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/advanced_class_guide/` (commit `7f818006e371188e5717fd18d74d18a420747fc6`, 2026-06-17).

**Roster:** Arcanist, Bloodrager, Brawler, Hunter, Investigator, Shaman, Skald, Slayer, Swashbuckler, Warpriest — the ten real ACG classes (the full, corrected roster; see `rules_tables::acg` module doc comment, corrected 2026-07-19 in SD-22). Criterion 4.3's own header text in `epic-breakdown.md` lists "Alchemist-side" as one of the ten and omits "Slayer" — both stale: there is no real `CLASS:Alchemist` record in `acg_classes.lst` (Alchemist is APG-only content), and Slayer does have a real record. This audit covers the real roster, not the header's stale list.

| Class | Book | Chassis rows wired / expected | Named class features wired / expected | `pilot_compute.rs` integrated | `level_up::<class>` module exists | Gap priority |
|---|---|---|---|---|---|---|
| Arcanist | ACG | 20 / 20 | 0 / 9 | No | No | P1 |
| Bloodrager | ACG | 20 / 20 | 1 / 19 | Yes | No | P1 |
| Brawler | ACG | 20 / 20 | 1 / 14 | Yes | No | P1 |
| Hunter | ACG | 20 / 20 | 1 / 21 | Yes | No | P1 |
| Investigator | ACG | 20 / 20 | 0 / 95 | No | No | P1 |
| Shaman | ACG | 20 / 20 | 0 / 10 | No | No | P1 |
| Skald | ACG | 20 / 20 | 1 / 20 | Yes | No | P1 |
| Slayer | ACG | 20 / 20 | 0 / 15 | No | No | P1 |
| Swashbuckler | ACG | 20 / 20 | 0 / 29 | No | No | P1 |
| Warpriest | ACG | 20 / 20 | 0 / 18 | No | No | P1 |

**Findings:**

1. **Chassis (BAB + saves): fully wired and independently verified correct for every class.** All ten classes' `class_table()` outputs were cross-checked against `acg_classes.lst`'s real `BONUS:COMBAT|BASEAB` and `BONUS:SAVE` tokens during this audit — no defects found. This chassis coverage was already SD-22 Epic 4 work; this cycle's contribution is turning the audit claim into executable, regression-tested code (`AcgClassId::ALL`, `class_coverage`, `coverage_report`), mirroring `rules_tables::apg`'s own SD-24 criterion-4.2 additions exactly.
2. **Named class features (Arcane Reservoir/Exploit, Bloodline, Martial Flexibility, Hunter's Trick, Studied Combat/Strike, Spirit, Raging Song, Sneak Attack/Slayer Talents, Panache/Deeds, Blessings/Fervor, ...): zero wired for any class.** This was already true and documented per-class (see e.g. `class_arcanist.rs`'s own doc comment, SD-22) — this cycle's contribution is (a) a reproducible corpus count of the expected feature-slot surface per class (`KEY:<Class> ~ ...` records in `acg_abilities_class.lst`; methodology and exact grep documented in `AcgClassCoverage`'s doc comment in `rules_tables::acg::mod`), and (b) a canary test (`zero_named_class_features_are_wired_for_any_acg_class_yet`) that fails loudly the moment any class's count changes.
3. **`pilot_compute.rs`'s live chassis dispatch (`compute_class_chassis`) does not recognize any ACG class.** Confirmed both by inspection and empirically: `tests/sd24_acg_class_coverage_audit.rs`'s `acg_classes_trip_the_honest_class_chassis_unsupported_diagnostic` test drives a real, minimal `CharacterInput` for each of the ten classes through `compute_pilot_base_chassis` and confirms it returns the honest, claim-blocking `class_chassis.unsupported` diagnostic with `base_attack_bonus: 0` — never a fabricated chassis number. Same behavior as every non-Fighter/Wizard CRB class and every APG class — the live pilot compute seam is presently Fighter/Wizard-only across every book.
4. **No `level_up::<class>` module exists for any ACG class.** `src/rules_core/level_up/` (the SD-20 Epic 7 per-level automatic-feature-grant model) contains only the 11 CRB classes.

**Conclusion feeding criterion 4.5:** ACG classes are *not* fully wired (chassis-only; zero named features; zero live-compute integration; zero level-up grant modules) — the identical shape criterion 4.2 found for APG. This is the exact condition `loop-instruction.md §4.2`'s hard-stop row anticipates ("Epic 4 finds APG/ACG classes are *not* fully wired → Multiclass Epic 5 scope is restricted to Fighter + Wizard only … defer APG/ACG-class multiclass to a follow-on bundle"). No blocker is raised here — this is the expected, already-decided outcome; criterion 4.5 records the deferral formally, jointly for APG and ACG.

**Update (v0.6 alpha swarm, risks item 8, first APG/ACG class-specific closure, 2026-07-25):** this audit's own findings 2 and 3 no longer hold unconditionally for Skald specifically — its Skald row above is updated accordingly (1 named feature wired, `pilot_compute_integrated: Yes`). Skald's Inspired Rage is now genuinely wired and its own chassis-integration gate (`is_supported_skald_single_class`) recognizes single-class Skald in `has_supported_class_chassis`. Skald still does not reach `Computed` (spellcasting and every other named feature remain deferred) and is still not multiclass-eligible (`AcgClassId::from_class_id_str` remains deliberately unregistered with `multiclass_class_level_supported`), so criterion 4.5's Epic 5 multiclass-deferral conclusion is unaffected. Every other ACG class's row is unchanged from the original 2026-07-21 audit. See `./acg_skald_coverage.md`'s own update note for the full record.

**Update (v0.6 alpha swarm, risks item 8, second APG/ACG class-specific closure, 2026-07-25):** the same update applies to Bloodrager -- its row above is updated accordingly (1 named feature wired, `pilot_compute_integrated: Yes`). Bloodrager's Bloodrage is now genuinely wired via `is_supported_bloodrager_single_class`, mirroring Skald's own gate-widening pattern exactly. Bloodrager still does not reach `Computed` (spellcasting and every other named feature remain deferred) and is still not multiclass-eligible, so criterion 4.5's conclusion remains unaffected. Every other ACG class's row (including Skald's, already updated above) is unchanged. See `./acg_bloodrager_coverage.md`'s own update note for the full record.

**Update (v0.6 alpha swarm, risks item 8, third APG/ACG class-specific closure, 2026-07-25):** the same update applies to Brawler -- its row above is updated accordingly (1 named feature wired, `pilot_compute_integrated: Yes`). Brawler's AC Bonus is now genuinely wired via `is_supported_brawler_single_class`, and is structurally simpler than Skald's/Bloodrager's own Rage-shaped mechanics: a pure function of level and class ownership, with no `class_ability_activations` entry needed at all (the "not wearing Medium/Heavy armor" precondition is provably vacuous in this codebase). Brawler still does not reach `Computed` (its remaining named-feature bucket stays deferred via a new `other_features_deferred` diagnostic, not `spellcasting_deferred` since Brawler is a pure martial class with no spellcasting at all) and is still not multiclass-eligible, so criterion 4.5's conclusion remains unaffected. Every other ACG class's row (including Skald's/Bloodrager's, already updated above) is unchanged. See `./acg_brawler_coverage.md`'s own update note for the full record.

**Update (v0.6 alpha swarm, risks item 8, fourth APG/ACG class-specific closure, 2026-07-25):** the same update applies to Hunter -- its row above is updated accordingly (1 named feature wired, `pilot_compute_integrated: Yes`). Hunter's 1st-level Animal Companion is now genuinely wired via `is_supported_hunter_single_class`, reusing the exact Wolf companion stat-block math Druid's own closure already verified (the corpus confirms this is the same mechanic: "the hunter's effective druid level is equal to her hunter level"). Unlike Druid's own choice-gated Nature Bond, Hunter's companion needs no `selected_choices` entry at all -- every Hunter gets one automatically, unconditional on class ownership and level alone. Hunter still does not reach `Computed` (spellcasting and every other named feature remain deferred) and is still not multiclass-eligible, so criterion 4.5's conclusion remains unaffected. Every other ACG class's row (including Skald's/Bloodrager's/Brawler's, already updated above) is unchanged. See `./acg_hunter_coverage.md`'s own update note for the full record.

**Per-class detail:** `./acg_arcanist_coverage.md`, `./acg_bloodrager_coverage.md`, `./acg_brawler_coverage.md`, `./acg_hunter_coverage.md`, `./acg_investigator_coverage.md`, `./acg_shaman_coverage.md`, `./acg_skald_coverage.md`, `./acg_slayer_coverage.md`, `./acg_swashbuckler_coverage.md`, `./acg_warpriest_coverage.md` (per `content-unit-inventory.md` §3.1's cycle-artifact-path column).
