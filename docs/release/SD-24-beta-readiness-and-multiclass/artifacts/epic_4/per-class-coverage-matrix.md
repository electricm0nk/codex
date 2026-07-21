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

## ACG (criterion 4.3) — pending

Not yet audited. The next Epic-4 ACG cycle appends its section here.
