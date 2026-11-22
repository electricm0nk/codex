# F3b2 -- converter step: skill ranks per level; closure attestation (SD-36 Epic F3)

The one step of the F2/F3 batch that regenerates `data/sheet_rules/**`, under the F1c
structural-diff protocol. `data/corpus/**` and `site/**` are untouched. It takes the two items F3b
(`f3b-multiclass-fold.md`) declined because they need the converter.

Code: `crates/codex-ingest/src/pcgen_import/sheet_rule/convert.rs` (`STARTSKILLPTS` arm,
`start_skill_points`) and `.../sheet_rule/mod.rs` (`build_index`: placeholder-keyed records,
`declared_row_key`). No engine code changed. Tests that pinned the old data were updated:
`class_chassis_sheet_rules.rs` tests, `tests/sd36_multiclass_any_class.rs`, and
`feat_prereqs.rs` (eligible count). Baseline for every "before" figure: tranche/16 at `cc21cac195`,
extracted read-only with `git archive` into scratch. Its `data/sheet_rules` is byte-identical to
this branch's before this step.

## 1. Skill ranks per level

**The token.** `STARTSKILLPTS:x` is a class-line tag: "how many skill points a character gains per
level ... Formulas can be used as well". Sources in the pinned oracle checkout (`7f818006e3`):
`docs/listfilepages/datafilestagpages/datafilesclasses.html` and
`code/src/java/plugin/lsttokens/pcclass/StartskillptsToken.java`. The converter already read it
from the class's own continuation row, which is in the closure (`cr_classes.lst:239`
`CLASS:Rogue STARTSKILLPTS:8`), and dropped it as metadata.

**The rule.** The converter now writes it as the principal's `StatBlock "Skill ranks per level"`
prose row. That is the same row shape `HD:` gives `"Hit die"`, so `ClassChassis::skill_ranks_per_level`
reads it with its existing parser. A later statement replaces an earlier one, following PCGen's order
of base row, then `.MOD` rows. There are 0 `.MOD` rows with the token in the tree. The value is
written as one number in two cases:

- a literal, such as `STARTSKILLPTS:4`;
- a bare variable that the record's own closure `DEFINE`s with a literal and raises only by
  unconditional literal `BONUS:VAR` rows outside level lines. The one case is Fighter:
  `STARTSKILLPTS:FighterSkillPoints`, `DEFINE:FighterSkillPoints|0` and
  `BONUS:VAR|FighterSkillPoints|2`, all on `cr_classes.lst:141`, give 2.

Anything else writes no row. It is named in the new `_defects/skill-ranks-unresolved.json`, never
given a guessed count.

**Counts** (`python3` walk of `data/sheet_rules/*/class/*.json` principals):

| population | with the row | without |
|---|---:|---:|
| census registry chassis records (`registry_chassis_records`, the census ids with a converted chassis) | **135 of 135** | 0 |
| chassis-bearing class records (`records()` over `prestige_scan_books`) | 171 of 177 | 6 (formula, below) |
| class files | 172 of 189 | 17 |

The companion kind also gains the row on 4 records (black_blade, imp_companion, shadow_companion,
vermin_companion), because their `CLASS:` rows state `STARTSKILLPTS`. That is the same rule applied
unchanged. Those records do not print (`print: false`).

The 17 class files with no row, by mechanism:

- **Formula, not a number (6, in `_defects/skill-ranks-unresolved.json`; none is a census class).**
  - 5 Bestiary creature-type classes use `0+BaseClassSkillPts` (`ce_classes_race.lst`): construct,
    ooze, plant, undead, vermin.
  - Eidolon uses `EidolonSkillPoints` (`apg_classes.lst:211`), a variable its own closure does not
    define.
  - The 7th defect row is `core_rulebook:companion:companion`.
- **The oracle row lacks the token (11).** Checked against every row in each record's
  `provenance.closure_rows`:
  - `bestiary:class:sorcerer_cleric_arcane`, `occult_adventures:class:psychic_detective`,
    `ultimate_intrigue:class:vcabalist` and `:vwarlock`;
  - `ultimate_psionics:class:gifted_blade`, `:gifted_blade_marksman_power_list` and
    `:unlocked_talent`;
  - the 4 Pathfinder Unchained class-selection principals (`pu_abilities_class.lst:114-117`). These
    are `CATEGORY:CLASS` abilities, and the base class's own row carries the token.
  - None of the 11 has a chassis. `record()` is `None` for all of them.

**Hand-worked values**, from the CRB class tables. They were written into the test before GREEN:
Fighter 2 (p.55), Rogue 8 (p.67), Wizard 2 (p.77), Bard 6 (p.35), Ranger 6 (p.64), Warrior 2
(p.449), Loremaster 4 (p.385) and Mystic Theurge 2 (p.387). The measured distribution over the
172 class rows is 2 x65, 4 x61, 6 x33, 0 x9 and 8 x4.

**Consequence: multiclass skill points now print.** In F3b this figure was Unknown for every class.
Every expected total below comes from `f3b-hand-worked.md`, which was written before F3b first ran.
The Int modifier is 0, and the human and favored-class extras are not the class's.
`tests/sd36_multiclass_any_class.rs` now asserts `multiclass.skill_points`:

| mix | skill points |
|---|---:|
| Barbarian 12 / Fighter 1 | **50** |
| Fighter 6 / Arcane Archer 3 | **24** |
| Magus 4 / Samurai 2 | **16** |
| Wizard 5 / Loremaster 2 | **18** |

No mix raises `class_chassis.skill_points.unknown` any more. The result was green in the full-suite
run (§5).

## 2. Closure attestation: the 45 defect rows

Before: `python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/closure_defects.py` over the
17 Unknown prestige classes listed 45 rows (38 `unresolved-references`, 7 `undefined-variables`).
Each row was re-traced against the pinned tree:

| mechanism | rows | classes | converter defect? | outcome |
|---|---:|---|---|---|
| **P: placeholder-keyed target.** The target converts, but its corpus JSON ships a codex-named placeholder key (`Codex-Named Unit (class_feature_adventurers_guide_ag_abilities_class_lst_9)`), and the index held it only under that key. The reference names the KEY its oracle row declares (`Aldori Swordlord ~ Adaptive Tactics`, `ag_abilities_class.lst:9`). | 33 | aldori_swordlord 10, magaambyan_arcanist 11, sanguine_angel 4, harrower 2, hellknight 2, steel_falcon 2, bellflower_tiller 1, hellknight_signifer 1 | yes | **fixed** |
| T: twin printing + category parent dropped (`Cyphermage Class Feature\|Cyphermage ~ Cypher Lore`) | 1 | cyphermage | partly; see declined | stays |
| U: target declared nowhere in the pinned tree (`Hunter's Bond ~ Companion`) | 1 | diabolist | no (oracle data) | stays |
| N: target declared but not an inventory unit (`Internal\|Bite`, `ce_abilities_race.lst:249`) | 1 | dragon_disciple | no (corpus: frozen 49,450) | stays |
| O: nested parameter (`skill focus (knowledge (religion))`) | 1 | exalted | see declined | stays |
| K: a key no record declares (`FEAT\|Spirit Beacon`; only `Spirit Beacon (Fey/Undead/Outsiders)` exist, `ag_feats.lst:60-62`; PCGen's own `PREABILITY` test compares the exact key, `PrerequisiteUtilities.passesAbilityTest`) | 1 | rivethun_emissary | no (oracle data) | stays |
| H: variable no row of the pinned tree declares: `CasterLevel_Highest`, `SecretLore` (declared only under `data/3e`), `MetaforgedLVL`, `MysticTheurgeLVL`, `PaDTrueSeeingLvl`, and `IsProfane` / `IsSacred` (declared only on the commented-out row `isg_abilities.lst:85`) | 7 | exalted 2, hellknight_signifer, loremaster, metaforge, mystic_theurge, pathfinder_delver | no. PCGen reads such a name as 0 (`PlayerCharacter.getVariable`: no `VariableKey`, bonuses excluded), and the converter correctly records it | stays |

**The mechanism P fix.** It is one rule and names no class. `build_index` now also indexes a
placeholder-keyed record under the KEY its own oracle row declares (`closure::row_identity`: the
`KEY:` token, else the name field). This happens after every corpus key has been indexed, and only
where no corpus key already answers the `(category, key)` pair. Two placeholder records that declare
one pair are marked ambiguous to each other.

The ordering was measured, not assumed. The first pass indexed inline, and the structural diff
failed on 42 removed `granted_by` edges. A reprint's real key, for example
`inner_sea_combat:feat:unblinking_flame_feint`, lost its target to the `adventurers_guide`
placeholder twin, which came first alphabetically. Indexing after the corpus keys removed all 42.

This is the repair F1c-3 made for a product-identity class's own name (`index.classes`), applied to
every placeholder-keyed record.

**Package effect** (structural diff, §4):

| figure | before | after |
|---|---:|---:|
| `_defects/unresolved-references.json` | 7,503 | 7,120 |
| class principals attested `closure_complete` | 56 of 189 | 64 of 189 |

- 383 reference rows resolved and 0 added. 0 of the 383 have a parenthesised name, so none relies on
  the single-level parameter split.
- 73 rule fields go from `MissingRule` to `Rule`, with nothing else in the field moving.
- 310 `granted_by` edges were added: 257 on `ability`, 53 on `class_feature`.
- 8 more class principals are attested `closure_complete`. The 7 classes are aldori_swordlord,
  bellflower_tiller, harrower, hellknight (both printings), magaambyan_arcanist, sanguine_angel and
  steel_falcon.
- `_vars/`: 1 added. `v6628da798516c76e` "Deific Obedience Active" is now referenced through a
  resolved rule. 1 changed: in `v7cc63dfeb52a80c1`, one contribution's gate goes from `MissingRule`
  to `Rule`.

**Reader remainder** (`cargo test --locked -j 8 --lib every_census_class_has_a_known_proficiency_answer -- --test-threads=8 --nocapture`):

`census classes: 137; with a static row: 42; walked by the reader: 95; Known at every level: 85; Unknown: 10`

- **Prestige Unknown: 17 -> 10 of 74.** Non-prestige Unknown stays 0 of 21 walked.
- The 7 that left each read Known with no weapon grant in their converted closure, and the pinned
  oracle's class rows grant none.
- The remainder by mechanism:
  - G-T, 1: cyphermage.
  - G-U, 1: diabolist.
  - G-N, 1: dragon_disciple.
  - G-O + H, 1: exalted.
  - G-K, 1: rivethun_emissary.
  - H, 5: hellknight_signifer, loremaster, metaforge, mystic_theurge, pathfinder_delver.
- `../reader-remainder.md` was regenerated with 10 `| class:` rows, and the test passes.

**Loremaster is still Unknown.** The brief expected it to flip. Its only closure defect is
`SecretLore` on `loremaster_secret_lore` (`cr_abilities_class.lst:3017`, read by `ASPECT`). No
Pathfinder row declares that variable; the only `DEFINE:SecretLore` is in
`data/3e/.../srd_abilities_prestige.lst:59`. This is not a converter defect, so its attestation stays
false, and the reader cannot answer Known(empty). The same holds for Mystic Theurge, Metaforge,
Pathfinder Delver and Hellknight Signifer. Wizard 5 / Loremaster 2 therefore still has its one
blocker, `combat.baseline_weapon_proficiency_unknown`.

**Census** (`cargo run --locked -j 8 --bin class_census -- --json docs/release/SD-36-consolidation/artifacts/epic-f/census-f3b2.json`):

| figure | F3b | F3b2 |
|---|---:|---:|
| ids | 137 | 137 |
| computed (non-prestige) | 63 of 63 | 63 of 63 (0 status changes) |
| blocked | 0 | 0 |
| prestige alone Blocked | 74 of 74 | 74 of 74 |
| prestige carrier mix Computed | 56 of 74 | **57 of 74** (magaambyan_arcanist: Blocked -> Computed) |
| prestige carrier mix Unknown | 11 | 11 |
| mix panel Computed | 185 of 185 | 185 of 185 |

The 6 still Blocked are unchanged:

- 4 on `multiclass.save_shape.unrecognized`.
- 2 on `combat.baseline_weapon_proficiency_unknown`: Cyphermage and Mystic Theurge. The mechanisms
  are G-T and H above.

`BASELINE_CENSUS_PRESTIGE_MIX_COMPUTED` is not added; the brief leaves it to F3c.

**Consequence: feat eligibility.** `feat_prereqs::a_starting_fighter_keeps_a_real_catalog...` went
from 539 to 534 eligible. This was attributed by diffing the eligible set on the tranche/16 package
and on the F3b2 package, with the same test binary. Five Inner Sea Races human-ethnicity feats now
check their ethnicity. Before, the reference was an unholdable `MissingRule`, so the clause was
reported and not checked. The level-1 fixture holds no ethnicity, so all five are now denied with a
stated reason:

- Friendly Rivalry: "requires Taldan";
- Loyal to the Death: Tian;
- Pursuit of Glory: Ulfen;
- Ruthless Opportunist: Chelaxian;
- Scion of the Lost Empire: Chelaxian or Taldan.

Every other record's verdict is unchanged.

## 3. RED -> GREEN

| stage | command | result | log |
|---|---|---|---|
| RED | `cargo test --locked -j 8 -p codex-ingest --test sheet_rule_convert_gate -- --test-threads=8 a_class_principal_states a_unit_converted_from a_nested_parameterised` | 0 of 3 pass. Rogue skill row `None`, expected `Some("8")`; Aldori Swordlord's `unresolved-references` non-empty; Exalted's `skill focus` row present | `f3b2-red.log` |
| diagnosis | read-only probe of the index | the RED test's category hypothesis was wrong: the category was already `Special Ability`. The target's corpus key is the placeholder. The test was renamed `a_placeholder_keyed_record_is_found_by_the_key_its_row_declares` and now also asserts that the reprint keeps its target. | -- |
| GREEN | same file, whole: `cargo test --locked -j 8 -p codex-ingest --test sheet_rule_convert_gate -- --test-threads=8` | 41 of 41; after the rename, the renamed test and the skill-ranks test: 2 of 2 | `f3b2-verify.log` |

The third RED test (nested parameter) was dropped with its change, as described in "Declined".

## 4. Regeneration protocol (F1c)

| gate | command | result |
|---|---|---|
| write | `cargo run --locked --quiet -j 8 -p codex-ingest --bin sheet_rule_convert -- --write` | exit 0; records 49,450, converted 49,450, refused 0, rules_written 73,016 (unchanged) |
| freshness | `... -- --check` | exit 0 |
| structural diff | `python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/structural_diff.py data/sheet_rules --baseline <tranche/16 git archive>/data/sheet_rules` | `verdict=PASS`, exit 0 (`f3b2-structural-diff.txt`) |
| pins | `python3 .../scripts/f3b2_delta_pins.py <baseline> data/sheet_rules .../scripts/structural_diff_f3b2_deltas.json` | 0 unexplained deltas. Pinned: `f3b2_skill_ranks` 176 (the number), `f3b2_closure_complete` 8 (`true`), `f3b2_placeholder_key_resolved` 73 (the fresh field's sha256) |
| planted mutations | 6 single-file mutations of a copy of the fresh package, each diffed | 6 of 6 FAIL, control PASS (`f3b2-planted-mutations.txt`) |
| residue | `python3 scripts/pcgen_residue_gate.py --check --closure` | `verdict=PASS` (shipped_scanned 69,719, hits 0) |
| frozen | `python3 scripts/site/check_frozen_status.py --check` | OK, frozen at 100% (49,450 units) |
| bundle | `node scripts/gen-corpus-bundle.mjs` | `files_copied=14029`; tree unchanged |
| clippy | `cargo clippy --locked -j 8 -p codex-ingest --tests -- -D warnings`; `cargo clippy --locked --tests -j 8 -- -D warnings` | clean |

The six planted mutations:

1. Rogue skill ranks 8 -> 9.
2. `closure_complete` planted on unpinned loremaster.
3. A resolved `Rule` target swapped.
4. A skill row planted on unpinned eidolon.
5. Rogue's `Hit die` row dropped alongside its pinned skill row.
6. The records count moved from 49,450 to 49,449.

`structural_diff.py` checks the F3b2 pins before the F1c pins. Nine product-identity class principals
F1c pinned for `prose` (`d4_pi_reclosure`) are now held to F3b2's stricter pinned-value check. With
the old order, a planted value change on them would have passed. `structural_diff_test.py`: 31 of 31.

## 5. Verify (`f3b2-verify.log`)

- Full root suite, `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8`: 288 test binaries,
  with 1 failure: the feat eligible count, attributed and re-pinned in §2. After the re-pin,
  `cargo test --locked -j 8 --lib feat_prereqs -- --test-threads=8` gives 25 of 25.
- `every_census_class_has_a_known_proficiency_answer` passes with 10 remainder rows.
- The `class_chassis_sheet_rules` tests pass:
  - `skill_ranks_per_level_is_read_for_every_chassis_bearing_class` replaces the F0 "honest absence"
    pin. It checks 171 of 177 and names the 6 formula records.
  - `a_class_missing_skill_ranks_reports_skill_points_unknown` now exercises Unknown on the
    synthetic chassis and asserts 0 of 135 registry records Unknown.
- `cargo test --locked -j 8 --test sd36_multiclass_any_class -- --test-threads=8`: 9 of 9, with
  skill points asserted at 50 / 24 / 16 / 18.
- Desktop, `cargo test --locked -j 8 --no-fail-fast --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8`:
  615 of 615.

## Declined (named, with the mechanism)

- **Nested-parameter split (exalted, mechanism O).** A balanced split was implemented and measured,
  then reverted. `Holdable` carries no option, so `Skill Focus (Knowledge (Religion))` would be held
  as `Skill Focus` with any option. The measured side effect was on `_vars/v3ecc4923bf829539`
  ("Dwarven Waraxe Exotic Use"), which would hold for any Exotic Weapon Proficiency. That is a wrong
  computed number, and the doctrine ranks it below a refusal. Closes in: an option-carrying
  `Holdable` (schema). Exalted also carries 2 H rows, so it would stay Unknown regardless.
- **Cyphermage's category parent (mechanism T).**
  - `closure.rs` drops a child category's parent when its two declarations disagree only on `TYPE`
    (`ism_abilitycategories.lst:56`, `ag_abilitycategories.lst:7`; both declare `Special Ability`).
  - Repairing that alone turns the row into `ambiguous-parent-category-target`, still a closure
    defect. The reason is that two units, `adventurers_guide` and `inner_sea_magic`, declare
    `Cyphermage ~ Cypher Lore`.
  - Closes in: a ruling on which printing a same-book reference names. That is a converter policy
    that was not taken here.
- U, N, K and H rows: not converter defects (table above). They stay defects, and their classes stay
  Unknown, each named in `../reader-remainder.md`.
