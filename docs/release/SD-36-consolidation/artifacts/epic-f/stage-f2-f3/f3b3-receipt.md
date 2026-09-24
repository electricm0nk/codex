# F3b3 -- engine consumes the F3b2 data; F3b findings closed (SD-36 Epic F3)

Engine-only step. `data/corpus/**`, `data/sheet_rules/**` and `site/**` are untouched (no converter
change); records 49,450; `python3 scripts/pcgen_residue_gate.py --check --closure` PASS
(shipped_scanned 69,720, hits 0). Before-figures are the F3b2b head `d739319bdc`.

Code:

- `src/rules_core/pilot_compute/class_skill_sheet_rules.rs` (new): a class's class skills read from its
  converted record.
- `src/rules_core/pilot_compute/feat_pillar_and_pool_aggregation.rs`: `selected_skill_class_skill`
  replaces the three hand-kept class lists; `pilot_compute.rs` path and `pilot_compute_corpus.rs` twin
  both refuse an Unknown by name.
- `src/rules_core/pilot_compute/multiclass_fold.rs`: `class_skill_points` (shared by the fold and the
  single-class line), `explain_single_class_skill_points`.
- `src/rules_core/pilot_compute/class_chassis_sheet_rules.rs`: `skill_ranks_per_level_from_package`.
- `src/rules_core/pilot_compute/class_shared_core.rs`: the single-class skill-points call.

## 1. Skill points

The multiclass fold already printed `multiclass.skill_points` from the converted ranks after F3b2
(`b1fdc45fca` re-pinned the four hand-worked totals). F3b3 finishes the item:

- **Single class.** A class alone now prints `class_chassis.skill_points`: the same per-class term the
  fold sums (`levels x max(1, ranks + Int)`, CRB p.30), from one function (`class_skill_points`), so a
  class alone and the same class in a mix print one number. The mix does not copy the per-class line
  (`is_character_level_total`). Racial and favored-class terms: the single-class path adds none to any
  skill-point total (Human's extra rank is the printed recognition line
  `race.human.trait_bundle.extra_skill_ranks`), so this line is the class term only, as the fold's is,
  and says so in its detail.
- **Where the ranks come from.** The chassis record's row, else the class principal's
  `Skill ranks per level` row even when the record is not chassis-bearing, else -- a class-selection
  class that declares no class line (`SheetRulePackage::base_class_of`) -- the base class it is taken on.
  One rule. A first cut that read only the chassis record left 5 census classes on
  `class_chassis.skill_points.unknown` although the data states the number: Monk (its CRB principal
  is degraded for BAB but states 4) and the four Pathfinder Unchained class selections (taken on Barbarian 4, Monk 4, Rogue 8, Summoner 2).
- **Unknown only where the row is missing.** Over every census id whose class alone reaches the line at
  level 1: **63 print the total, 0 Unknown** (population 63 of 137 ids; the 74 prestige ids never compute
  alone). Positive control: Eidolon (`STARTSKILLPTS:EidolonSkillPoints`, apg_classes.lst:211, no row in
  `_defects/skill-ranks-unresolved.json`) answers `class_chassis.skill_points.unknown` naming
  `class:eidolon`.

Hand-worked expectations asserted (oracle first, `f3b-hand-worked.md`, Int 10 so +0):

| figure | working | asserted | where |
|---|---|---:|---|
| Barbarian 12 / Fighter 1 | 12 x 4 + 1 x 2 | 50 | `tests/sd36_multiclass_any_class.rs` (4 mixes) |
| Fighter 6 / Arcane Archer 3 | 6 x 2 + 3 x 4 | 24 | same |
| Magus 4 / Samurai 2 | 4 x 2 + 2 x 4 | 16 | same |
| Wizard 5 / Loremaster 2 | 5 x 2 + 2 x 4 | 18 | same |
| Barbarian 12 alone / Fighter 1 alone | 48 / 2 (sum = the mix's 50) | 48 / 2 | `a_class_alone_prints_the_class_skill_points_term_the_fold_sums` |
| Wizard 5 / Rogue 3 alone | 5 x 2 (CRB p.77) / 3 x 8 (CRB p.67) | 10 / 24 | same |
| Monk 3, Int +1 (cr_classes.lst:151 `STARTSKILLPTS:4`, CRB p.56) | 3 x (4 + 1) | 15 | `multiclass_fold::tests::skill_ranks_come_from_the_record_or_the_base_class_it_is_taken_on` |
| Unchained Rogue 2, Int -1 | 2 x (8 - 1), read from `core_rulebook:class:rogue` | 14 | same |
| Fighter 4, Int -3 | at least 1 per level | 4 | same |

0 mismatches between the engine and the sheet.

## 2. Wizard 5 / Loremaster 2

**Computed**, and asserted so (`wizard5_loremaster2_computes_the_hand_worked_sheet`,
`blocked_only_by: None`): BAB 3, base saves 2/2/6, totals 4/4/7, HP 44, skill points 18. F3b2b attested
Loremaster's closure (`SecretLore` reads as the oracle's 0), which cleared the one blocker,
`combat.baseline_weapon_proficiency_unknown`. F3b3 changed nothing here.

## 3. The four prestige classes Blocked on `multiclass.save_shape.unrecognized`: 0 of 4 classified

Each record's three save `Expr`s (converted, `data/sheet_rules/<book>/class/<slug>.json`) against the
PF1 save forms (CRB p.30 base good `L/2+2`, poor `L/3`; CRB p.374+ prestige good `(L+1)/2`, poor
`(L+1)/3`, +5 / +3 at 10th). The oracle's source formula is quoted from the corpus record's
provenance line.

| class | oracle line (book, page) | Fort | Ref | Will | oracle's own fractional-save declaration |
|---|---|---|---|---|---|
| Exalted | isg_classes.lst:25 (Inner Sea Gods p.200) | `CL+1/3` | `CL+1/3` | `CL+1/2` | Poor / Poor / Good |
| Sentinel | isg_classes.lst:48 (Inner Sea Gods p.202) | `CL+1/2` | `CL+1/3` | `CL+1/3` | Poor / Poor / Poor |
| Ulfen Guard | isc_classes.lst:29 (Inner Sea Combat p.34) | `CL+3/2` | `CL+1/3` | `CL+3/2` | Good / Poor / Good |
| Mammoth Rider | ag_classes.lst:260 (Adventurer's Guide p.128) | `(CL+2)/2` | `CL/3` (Poor, recognized) | `CL/3` (Poor, recognized) | Good / Poor / Poor |

**Finding: the converted `Expr` is a faithful conversion of a defective oracle formula. It is not a
shape the classifier misses.** PCGen's formula parser divides before it adds, so `classlevel+1/3` is
`level + 1/3`. The oracle itself therefore prints Exalted's Fortitude as its class level: +10 at 10th,
twice PF1's highest 10-level save. The engine's row reproduces it (`row_at(10)`: Exalted 10/10/10,
Sentinel 10/10/10, Ulfen Guard 11/10/11). Mammoth Rider's `(classlevel+2)/2` gives +2 at 2nd and +6 at
10th, where the prestige good table gives +1 and +5, and the same row declares `ClassSaveGood_Fortitude`.
The oracle contradicts its own formula there.

No classifier extension is right:

- The brief's candidates do not apply. None of the 10 slots is the +1-at-1st prestige form, a 5-level
  table, or a flattened constant table.
- Reading the author's intent into `CL+1/3` as `(CL+1)/3` would be a guess.
- The oracle's second statement, the `ClassSave{Good,Poor}_<Save>` variables under
  `UseFractionalSave`, is not carried in the converted records (0 matches in the four files or in
  `_vars/`). It also disagrees with the formula's apparent intent for Sentinel: `CL+1/2` Fortitude
  reads as intended-good, while the declaration says Poor.

The page tables themselves are not in any repo source. The book and page are cited from provenance.
The comparison here is structural and does not rest on a recalled table. The three `CL+x/y` classes
exceed every PF1 save form at 10th (base good +7). Mammoth Rider's Fortitude differs from both good
forms at every even level (+2 at 2nd against base good +3 and prestige good +1).

The four stay Blocked. The defect is pinned by
`multiclass_fold::tests::the_four_unrecognized_prestige_saves_are_oracle_formula_defects_not_a_missed_shape`.
That test asserts the 10 Unrecognized slots, the two recognized Poor slots, each value at 10th, and
that `multiclass_member` refuses each at level 3 with `multiclass.save_shape.unrecognized`. The
mechanism: **oracle-data operator-precedence defect in the class line's `BONUS:SAVE` formula**. Its
close is a data correction with a book-table oracle, outside an engine batch.

## 4. The class-skill +3: read from the record

**Root cause.** `selected_skill_{climb,intimidate,swim}_is_class_skill` were three hand-kept class
lists covering 13 classes. Barbarian was never added. Its converted record states all three:
`core_rulebook:class_feature:class_skills_barbarian`, cr_abilities_class.lst:2831,
`CSKILL:Acrobatics|Climb|TYPE=Craft|Handle Animal|Intimidate|Knowledge (Nature)|Perception|Ride|Survival|Swim`.

**Fix (one rule).**

- `class_skill_sheet_rules::class_skill_view(slug, level)` seeds the held set with the class alone, the
  same walk the weapon-proficiency reader makes. It collects every `Fact::ClassSkill` /
  `Fact::ClassSkillGroup` that a held rule grants, with a gated grant counted only when class-decidable
  and true.
- The selected-skill check is PF1's union (CRB p.87) over those answers.
- An empty walk is Unknown, never "no class skills".
- A class the reader cannot answer uses a cited oracle row only if it has one: the 9 ACG classes whose
  `ABILITY:Class|AUTOMATIC|<Class>` grant is an unresolved reference (`_defects/unresolved-references.json`
  `<book>:class:<slug>: Class|<Class>`). Those rows are the old list's own values, checked against
  acg_abilities_class.lst:65/891/1169/1241/1384/1720/1786/1951/2133.
  `every_fallback_row_is_a_class_the_reader_cannot_answer` fails the day the converter closes the edge.
- Otherwise the check refuses by name, on both the headless and the corpus path:
  `skill.selected_modifier.class_skill_unknown`.

**RED -> GREEN** (`tests/sd36_class_skill_from_record.rs`; RED log `f3b3-red.log`):

| test | RED (before the fix) | GREEN |
|---|---|---|
| `barbarian_gets_the_class_skill_bonus_its_record_grants` (hand-worked: Str 18, 1 rank, +3, chain shirt -2 (CRB p.150); Cha 8) | Climb 3 (expected 6) | Climb 6 / Intimidate 3 / Swim 6 |
| `every_census_class_prints_the_class_skill_bonus_its_record_grants` | **79 of 156** (class, skill) lines disagree with the record, across **35 of 52** classes; every one was "record grants, sheet withholds"; 0 over-grants | 0 of 156 |

The scan's denominator: census ids 137. The record is Known at level 1 for 126. Of those, 52 classes
print the three selected-skill lines at level 1, which is 156 lines.

The 35 classes that lacked the +3 on at least one line:

aegis, antipaladin, aristocrat, barbarian, bard, commoner, cryptic, dread, druid, ex_antipaladin,
ex_barbarian, ex_inquisitor, gunslinger, kineticist, magus, marksman, medium, mesmerist, ninja,
psychic, psychic_warrior, ranger, samurai, shifter, sorcerer, soulknife, spiritualist, tactician,
unchained_barbarian, unchained_monk, unchained_rogue, vigilante, vitalist, warrior, wilder.

**Reader coverage over the census** (137 ids, level 1): Known 126, Unknown 11. The 11 Unknown, by
mechanism:

- **Unresolved `Class|<Class>` edge (converter), 9:** arcanist, brawler, hunter, investigator, shaman,
  skald, slayer, swashbuckler and warpriest. The cited fallback rows answer these.
- **The player's own choice, which the record does not resolve, 2:**
  - Expert chooses ten class skills (CRB p.450). `expert_class_skills` offers `Skills: all` and has no
    granting edge.
  - Psion's class skills are on the discipline `SUBCLASS` lines (`CSKILL:` at up_classes.lst:221+),
    which are not converted.
  - Both are now refused by name. Before F3b3 they printed without a +3.

**Consequence, recorded as a correction.** The census non-prestige Computed count is **61 of 63** (was
63): Expert and Psion are Blocked at every level on `skill.selected_modifier.class_skill_unknown` alone.
This is a refusal of a number the record cannot support, not a regression of a supported one.

- Logged: `scripts/retro.py correction` (`docs/retro/events/sd36-f3b3-executor.jsonl`).
- Pins moved with it:
  - `class_census::tests::census_id_set_matches_the_published_partition`: 63 -> 61.
  - `untabled_class_chassis_gate_tests::every_untabled_class_outside_the_named_reader_remainder_reaches_computed`:
    names `CLASS_SKILL_REMAINDER = [expert, psion]` and asserts that blocker alone.
- Baseline floor `BASELINE_CENSUS_COMPUTED=61` still holds
  (`scripts/check_class_census_baselines.py` OK).
- Closing them needs a recorded class-skill choice to reach the reader, the F4 seeding work.

Pins that encoded the defect, re-worked by hand (class_barbarian.rs `raging_climber_and_swimmer_tests`):

- unraged Barbarian 5 Climb/Swim 3 -> **6**;
- raging 10 -> **13** (1 + Str 22 (+6) + 3 - 2 + Raging Climber 5);
- inactive rage Climb 3 -> **6**.

**The same shape, named and not changed here.** `skill_allocation::class_skill_set` /
`is_class_skill_for` (the allocated-ranks path and the v06 inventory classifier) still hold a hand-kept
3-class list (Fighter, Rogue, Wizard) of 137 census ids. Routing it through the reader moves the
work-inventory classifier's counts, which is a separate measured change.

## 5. Desktop dips blocked on their own choice line (left for F4)

A Fighter who dips a chooser class via `apply_level_up` carries that class's own "no choice made"
blocker into the mix (F3b, `f3b-multiclass-fold.md` "Desktop consequence"). The dip branch
(`apps/desktop/src-tauri/src/pf1_adapter.rs` `apply_level_up`) seeds only Wizard, Arcanist and Monk. It
does not seed Cavalier (order), Inquisitor (domain/inquisition) or Oracle (mystery/curse), among others.
Those stay Blocked on the class's choice line until F4 makes seeding single-source.

## 6. Census (`cargo run --locked -j 8 --bin class_census -- --json <scratch>/census-f3b3.json`; committed as `artifacts/epic-f/census-f3b3.json`)

| figure | F3b2b | F3b3 |
|---|---:|---:|
| ids | 137 | 137 |
| computed (non-prestige) | 63 of 63 | **61 of 63** (expert, psion: `skill.selected_modifier.class_skill_unknown`) |
| prestige alone Blocked | 74 of 74 | 74 of 74 |
| prestige carrier mix Computed | 59 of 74 | **59 of 74** (no movement) |
| prestige carrier mix Blocked | 4 | 4 (exalted, mammoth_rider, sentinel, ulfen_guard: §3) |
| prestige carrier mix Unknown (no nameable carrier) | 11 | 11 |
| mix panel Computed | 185 of 185 | 185 of 185 |

Prestige-mix movement: none. No carrier mix involves Expert or Psion, and every carrier's class skills
were already Yes or No. `docs/architecture/status.md` class table regenerated
(`python3 scripts/gen_class_status_table.py --json <census>`; `--check` OK). It had been stale at 56 of
74 since F3b2. `docs/architecture/rules-engine.md` multiclass section updated.

## 7. Verify

| command | result |
|---|---|
| `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` (root) | every integration target green. The lib had 5 failures, the defect pins and the census pin above, then re-run: 2,719 passed, 0 failed |
| `cargo test --locked -j 8 --lib -- --test-threads=8` | 2,719 passed, 0 failed, 6 ignored (then +1: the §3 save-shape test, 1 of 1; the named filters below run on the final tree, 2,726 lib tests) |
| `cargo test --locked -j 8 --no-fail-fast --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8` | 615 of 615 |
| `--test sd36_multiclass_any_class` | 11 of 11 |
| `--test sd36_class_skill_from_record` | 2 of 2 |
| `--lib multiclass` / `--lib class_chassis_sheet_rules` / `--lib save_shape` / `--lib class_skill_sheet_rules` | green (`f3b3-verify.log`) |
| `cargo clippy --locked --tests -j 8 -- -D warnings` | clean |
| `python3 scripts/pcgen_residue_gate.py --check --closure` | PASS |
| `python3 scripts/check_class_census_baselines.py` (env floors) | OK |
