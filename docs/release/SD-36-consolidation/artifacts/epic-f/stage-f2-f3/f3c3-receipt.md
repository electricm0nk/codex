# F3c3: PCGen SUBCLASS lines convert to class choices (SD-36 Epic F3, converter step 3)

This is the third step of the F2/F3 batch that regenerates `data/sheet_rules/**`. It follows the
same F1c structural-diff protocol and the same single exception as F3b2 and F3b2b. `data/corpus/**`
and `site/**` are untouched. The record count stays 49,450, and
`python3 scripts/pcgen_residue_gate.py --check --closure` passes.

It closes the one non-prestige class F3c2 left Blocked. F3c2 (`de01bf571a`, `f3c2-receipt.md` §1)
found that Psion's base class skills sit only on its discipline `SUBCLASS:` lines, which the
converter did not carry, and recorded it as `forward-scope-register.md` FS-16. It also re-checks
Dragon Disciple's weapon-proficiency blocker (§4).

**Code:**

- `crates/codex-ingest/src/pcgen_import/sheet_rule/subclass.rs` (new): the rule.
- `.../sheet_rule/mod.rs` (`run`): where the choice and its options join the package.
- `src/rules_core/class_seeds.rs`: Psion's canonical discipline.
- No reader changed. `class_skill_sheet_rules` already walks a class's canonical member picks, the
  rule F1c-5 D8 introduced.

**Baselines:**

- Every "before" figure is the F3c2 package and census (`de01bf571a`, `census-f3c2.json`).
- The structural-diff baseline is tranche/16 (`cc21cac195`).
- Both packages were extracted read-only with `git archive` into scratch.

## 1. The population: which classes declare SUBCLASS lines

Counted with `grep -c '^SUBCLASS:'` and `grep -c '^SUBCLASSLEVEL:'`, and each row attributed to the
`CLASS:` row it follows. The pinned oracle checkout is `~/workspace/repos/pcgen` at `7f818006e3`.

| scope | files | classes | `SUBCLASS:` rows | `SUBCLASSLEVEL:` rows |
|---|---:|---:|---:|---:|
| the whole Pathfinder tree (`data/pathfinder`) | 10 | 2 (Wizard, Psion) | 73 | 73 |
| the converter's pinned tree (`BOOKS_RELATIVE` + `EXTRA_BOOK_DIRS`) | 6 | 2 (Wizard, Psion) | **47** | 47 |

The 26 rows outside the converter's tree are in books the corpus does not ship. They are
`psionics_unleashed` (7), `psionics_expanded` (9), `elemental_masters_handbook` (9) and
`dragon_empires_primer` (1).

The 47 rows by class:

| class | rows | where |
|---|---:|---|
| Psion | 17 | `up_classes.lst:221-256`, after the `CLASS:Psion` continuation row `:220` |
| Wizard | 30 | 9 under the CRB class row (`cr_classes.lst:283-300`), plus `CLASS:Wizard.MOD` rows: APG 4 (`apg_classes.lst:366-373`), UM 2 (`um_classes.lst:93-96`), AG 7 (`ag_classes.lst:492-505`), ISM 7 (`ism_classes.lst:36-49`), UP 1 (`up_classes.lst:629`, Psychic Mage) |

**What the oracle says the tokens mean.** The citations are in the pinned checkout.

- **The sub-class line.** `docs/listfilepages/datafilestagpages/datafilesclasses.html`, under "The
  Sub-Class Line", describes a line beginning `SUBCLASS:<sub-class name>`, "placed between the
  Class Lines and the Sub-Class Level Lines". Its tag entry says it "defines a subclass or
  specialty choice".
- **The level line.** Under "The Sub-Class Level Line", the same page describes
  `SUBCLASSLEVEL:<level number>`, which "will define a Subclass level dependent ability for the
  SUBCLASS immediately above it".
- **`CSKILL`.** It is a global tag (`globalfilestagpages/globalfilesother.html`, `CSKILL:x|x`), so a
  sub-class line reads it the way a class line does.
- **A sub-class is a class object.** `pcgen/core/SubClass.java:31` declares
  `SubClass extends PCClass`.
- **The pick.** `pcgen/core/analysis/SubClassApplication.checkForSubClass` builds the offer.
  - It offers ONE pick from the class's `ListKey.SUB_CLASS` list, in list order, and drops any
    sub-class whose own prerequisites fail.
  - It puts the base class at index 0 only when `ALLOWBASECLASS` holds. Both classes state
    `ALLOWBASECLASS:NO` (`cr_classes.lst:277`, `up_classes.lst:216`), so a pick is mandatory.
  - No token marks a default pick.

## 2. The rule (one mechanism, no per-class case)

The rule is in `subclass.rs`, and its module doc states it in full.

**Where a line belongs.** A `SUBCLASS:` row belongs to the class whose converted closure carries the
`CLASS:` row it follows. That row can be a base row, a continuation row or a `.MOD` row, which are
the same rows the class record already reads. There are 0 orphan rows
(`_defects/subclass-row-without-class.json` was not written).

For each such class the converter writes two things.

**The choice.** One choice sibling goes on the class record:

- id `<class id>#subclass`;
- `offers: Rules { pool: "subclass", tags: ["<Class> Subclass"] }`, count 1;
- `print: false`;
- gated by the class record's own gates, like every other line of the record (F1c-2's contract,
  `class_proficiency_line_gate::no_principal_carries_one_lines_condition`).

The choice is a sibling and not the principal's own `offers`, because a class principal can already
carry an offer: Cleric's and Druid's principals offer a language pick.

**The options.** One `subclass` rule goes out per line:

- id `<book>:subclass:<class>_<sub-class>`;
- granted by `Granter::Choice(<class id>#subclass)`;
- converted by the same `convert_record` every record goes through. The `SUBCLASS:` row is its base
  row, and each `SUBCLASSLEVEL:<n>` row is a level line gated at class level n.

So the shapes are the ones the converter already emits:

- `CSKILL` becomes `FactGrant(ClassSkill | ClassSkillGroup)`, the shape `class_skill_sheet_rules`
  reads.
- A `PRE*` token on the line becomes the option's gate. Ascendant Psion's `PRERACE:1,Elan` becomes
  `Holds Race elan`.
- A level line's `ABILITY` becomes a grant edge `Granter::Rule(<option>)`, gated at the level and
  by its own `PRE`.
- A level line's `BONUS:VAR` becomes a `_vars/` contribution.
- `BONUS:SPELLCAST` / `BONUS:ABILITYPOOL` become the option's own `#bonus<n>` lines.

**Tokens the option does not carry.** Each is a named row in
`_defects/subclass-token-unconverted.json`. That file has 117 rows, and 0 of them are degradations
inside `convert_record`:

| token | rows | what it is |
|---|---:|---|
| `COST` | 33 | the number of prohibited schools (not the equipment `COST` the metadata arm drops) |
| `CHOICE` | 21 | the specialty school (`CHOICE:SCHOOL\|Evocation`) |
| `SPELLLIST` | 18 | the discipline's or school's power/spell list |
| `PROHIBITCOST` | 17 | the cost of prohibiting this school |
| `PREVAREQ` | 15 | a stand-alone prerequisite on a `SUBCLASSLEVEL` row: `convert_record` does not read one on a level line, so it is named rather than silently dropped |
| `KNOWNSPELLSFROMSPECIALTY` | 13 | extra known specialty spells |

`SOURCEPAGE` and `SORTKEY` are filing metadata.

**Reprints.** A sub-class name that appears twice for one class, token-identical in two books that
each have a `SOURCEDATE:`, is one object printed twice. The standing supersession ruling applies
(`reprint.rs`, `decisions.md` §12): the newest printing is kept, and the older one becomes an
informational row in `_defects/subclass-superseded-reprint.json`. There are 7 such rows: the seven
sin schools of `ism_classes.lst:36-49` (`SOURCEDATE:2011-07`) are superseded by
`ag_classes.lst:492-505` (`2017-06`). Any other repeat keeps the first and names the rest in
`subclass-name-collision`, which has 0 rows.

**Converted:** 2 classes and 40 sub-classes.

| | count |
|---|---:|
| choice siblings (`core_rulebook:class:wizard#subclass`, `ultimate_psionics:class:psion#subclass`) | 2 |
| Psion options (Egoist first) | 17 |
| Wizard options (CRB 9 + APG 4 + UM 2 + AG 7 + UP 1; ISM 7 superseded; Abjurer first) | 23 |
| option line siblings (11 `BONUS:SPELLCAST`, 1 `BONUS:ABILITYPOOL`) | 12 |
| grant edges the options hand out (Wizard 27, Psion 26; 52 on `class_feature`, 1 on `feat`) | 53 |
| `_vars/` contributions added (9 tables, e.g. `Psychometabolism Discipline LVL`) | 16 |
| options carrying class-skill grants (all 17 Psion options; no Wizard line states `CSKILL`) | 17 |

## 3. Psion answers through its canonical discipline (62 → 63 of 63)

**The seed.** The canonical default is the first sub-class in oracle order, because the oracle marks
no default (§1). For Psion that is Egoist (`up_classes.lst:221`). It is seeded through
`class_seeds::canonical_seeds_for("psion")` like every other pick: `PSION_SUBCLASS_CHOICE_ID` →
`PSION_CANONICAL_DISCIPLINE`.

`class_skill_sheet_rules` walks it with the rule it already has. `canonical_member_picks` accepts a
pick when its chooser offers `Rules` under its own id and the member is granted by that choice. It
then holds the option, and through the option's edge it holds `Psychometabolism Class Skills`.

**Egoist's class skills** (UP p.49 and p.52):

- from the line itself: Autohypnosis, Craft, Knowledge, Profession and Spellcraft (`:221` `CSKILL`);
- from `Psychometabolism Class Skills`: Acrobatics and Heal (`up_abilities_class.lst:409`, granted at
  level 1 by `:222`).

The edge carries `!PREABILITY:1,CATEGORY=Archetype,TYPE.PsionDisciplineClassSkills`, so an
archetype that replaces discipline skills turns it off.

**Wizard is not seeded, by design.** Its first line is Abjurer. The existing Path-A school pick from
`pf1_adapter.rs` (`choice:wizard_school_specialization` → `school:evocation`) is in another id space,
and seeding Abjurer next to it would contradict it. Seeding Evoker would depart from the one rule.
The Wizard needs no sub-class pick for any sheet total, because none of its lines states `CSKILL`.
Its choice converts and is tested (§5), and it waits for the picker work that unifies the two id
spaces.

**Hand-worked first** (oracle first). The fixture is the shared census fixture: Human, Str 18 (+4),
Cha 8 (-1), 1 rank each, chain shirt ACP -2 (CRB p.150). None of Climb, Intimidate or Swim is an
Egoist class skill, so none gets +3 (CRB p.87).

| line | working | asserted, levels 1 and 20 |
|---|---|---:|
| Psion Climb | 1 + 4 - 2 | 3 |
| Psion Intimidate | 1 - 1 | 0 |
| Psion Swim | 1 + 4 - 2 | 3 |

### RED to GREEN (`f3c3-red.log`)

| test | RED | GREEN |
|---|---|---|
| `sheet_rule_convert_gate::a_subclass_choice_converts_with_its_class_skill_grants` | panicked: "psion carries a subclass choice" (skeleton `convert_subclasses` returned nothing) | ok |
| `sheet_rule_convert_gate::a_wizard_school_subclass_converts_through_the_same_rule` (the second SUBCLASS-bearing class: 23 options, Abjurer first, sin schools from AG not ISM, Evoker → `evocation_school` edge) | panicked: `out.choosers ... wizard#subclass` | ok |
| `sd36_class_skill_from_record::psion_class_skills_answer_through_its_canonical_discipline` | `psion: Unknown { "the converted closure of psion at level 1 reaches no class-skill grant ..." }` | ok: Known, `granted_by` has `psion_egoist`; Climb 3, Intimidate 0, Swim 3 at levels 1 and 20; no `class_skill_unknown` |

The first GREEN run of the converter test found a bug in the test itself: it read `options[0]`,
which is a Wizard option in class-record order. It now looks the Egoist option up by id.

### Double-grant scan

`sd36_class_skill_from_record::a_seeded_subclass_pick_is_scanned_for_double_grants_against_the_bespoke_join`
checks whether a class's bespoke chassis module already grounds a record that a sub-class pick now
holds, joined through `sheet_line_join::rule_for_explanation`. Over all 137 census ids at levels 1
and 20:

- **Classes seeding a sub-class pick:** 1 (Psion).
- **Rules held only because of the pick:** 10, which is 5 at each level. They are the option and
  what it grants.
- **Those the bespoke module also joins to:** 0.
- **What a pick would meet, for each class carrying a converted choice** (Psion and Wizard): the
  options grant 53 records directly, and the bespoke module joins to 0 of them. The Wizard's
  bespoke school lines do not join to the converted `*_school` records, so seeding a Wizard pick
  later would not double-print.

So no dedup was needed: 0 of 0.

## 4. Dragon Disciple: `combat.baseline_weapon_proficiency_unknown` stays, named

`python3 docs/release/SD-36-consolidation/artifacts/epic-f/scripts/closure_defects.py dragon_disciple`
reports "closure records 17; closure_complete [False]; unresolved-references 1". The one row is
`core_rulebook:class_feature:dragon_disciple_dragon_bite: Internal|Bite`.

**Mechanism N** (`f3b2-converter-receipt.md` §2): the target is declared in the pinned tree, at
`core_essentials/ce_abilities_race.lst:249` (`Bite`, `CATEGORY:Internal`, a natural-attack helper).
No inventory unit stands for it, though: `docs/work-inventory.json` has no `Internal` `Bite` unit.
So the converter has no rule to point the edge at.

This is not a converter defect. Closing it needs a corpus/inventory record, and the corpus is frozen
at 49,450. The attestation therefore stays false, and the reader cannot answer the known-empty set
that CRB p.380 states ("no proficiency with any weapon or armor").

Even if it closed, the sorcerer 5 / Dragon Disciple mix would stay Blocked. Its second line,
`class_feature.sorcerer.arcane_bond_and_bloodline_progression.unsupported`, is the Sorcerer seam,
which grounds only the Arcane bloodline.

Census row: unchanged vs F3c2, Blocked at levels 1-10 on the same two ids.

## 5. Regeneration protocol (F1c)

| gate | command | result |
|---|---|---|
| write | `cargo run --locked --quiet -j 8 -p codex-ingest --bin sheet_rule_convert -- --write` | exit 0; records 49,450, converted 49,450, refused 0; rules_written 73,016 → **73,070** (+54); var_tables 6,205 |
| freshness | `... -- --check` | exit 0 |
| pins | `python3 .../scripts/f3c3_delta_pins.py <tranche/16>/data/sheet_rules data/sheet_rules .../scripts/structural_diff_f3c3_deltas.json <F3c2>/data/sheet_rules` | 0 unexplained; no existing field moved vs F3c2 and no edge or contribution removed. Pinned: `f3c3_subclass_choice` 2, `f3c3_subclass_option` 52 (whole-rule sha256), 53 required added edges, 16 `_vars/` contributions, defect rows {token-unconverted 117, superseded-reprint 7} |
| structural diff | `python3 .../scripts/structural_diff.py data/sheet_rules --baseline <tranche/16>/data/sheet_rules` | `verdict=PASS`, exit 0 (`f3c3-structural-diff.txt`): unexpected field deltas 0, removed edges 0, removed grants 0, added rule ids 54 (0 with no named cause), F3b2/F3b2b pins unchanged |
| planted mutations | `python3 .../scripts/f3c3_planted_mutations.py <scratch> <repo>` | **8 of 8 FAIL, both controls PASS** (`f3c3-planted-mutations.txt`) |
| diff self-test | `python3 .../scripts/structural_diff_test.py` | 33 of 33 (+1: a dropped pinned sub-class rule, and an unpinned one, gate) |
| residue | `python3 scripts/pcgen_residue_gate.py --check --closure` | `verdict=PASS`, shipped_scanned 69,762, hits 0 |
| frozen | `python3 scripts/site/check_frozen_status.py --check` | OK, frozen at 100% (49,450 units) |
| bundle | `node scripts/gen-corpus-bundle.mjs` | `files_copied=14029`; tree unchanged |

**Why a new diff guard.** The structural diff forbids only removals and field changes against
tranche/16. An added rule id with no named cause was printed and not gated, so a dropped, changed or
extra sub-class rule would have passed. `structural_diff.py` now reads
`structural_diff_f3c3_deltas.json` (`f3c3_check`).

What it gates:

- a pinned rule that is missing (while its owning class principal is a converted record);
- a pinned rule whose sha256 moved;
- a `subclass` / `#subclass` rule nobody pinned;
- a pinned edge or contribution that is missing;
- a defect-file row count that moved.

The mutations:

1. Egoist loses its Spellcraft class-skill grant (sha moved).
2. Psion's choice is dropped from the class record.
3. The Egoist → `Psychometabolism Class Skills` edge is dropped.
4. Egoist's `Psychometabolism Discipline LVL` contribution is dropped.
5. One `subclass-token-unconverted` row is dropped.
6. An unpinned sub-class rule is planted.
7. Evoker's tag is moved off the Wizard choice.
8. Every Psion sub-class file is deleted (19 failures).

**One suite finding during protocol.**
`class_proficiency_line_gate::no_principal_carries_one_lines_condition` failed on the first write.
The choice sibling was `applies: Always`, and the class principals carry `ClassLevel <= 20`. The
choice now takes the class record's gates. Both choices were rewritten, and pins and diff were
re-run on the final package.

## 6. Census (`cargo run --locked -j 8 --bin class_census -- --json <scratch>/census-f3c3.json`, copied to `../census-f3c3.json`)

```
ids=137 computed=63 blocked=0
prestige_swept=74 prestige_alone_blocked=74 prestige_mix_computed=67 prestige_mix_unknown=0
mix_panel_swept=185 mix_panel_computed=185 mix_panel_blocked=0
```

**Per-row diff against `census-f3c2.json`.** It covers 322 rows: 63 non-prestige, 74 prestige and
185 mix panel. 1 of 322 changed:

| row | F3c2 | F3c3 |
|---|---|---|
| `class:psion` | Blocked at levels 1-20 (`skill.selected_modifier.class_skill_unknown`) | **Computed** at levels 1-20 |

Every other row keeps its status, levels and blocker ids, Dragon Disciple included (§4).

`BASELINE_CENSUS_COMPUTED` goes 62 → **63**, with a dated reason in `scripts/verify-baselines.env`.
`BASELINE_CENSUS_PRESTIGE_MIX_COMPUTED` stays 67.

## 7. Moved pins (fixture protocol)

Each is logged with `scripts/retro.py correction` in `docs/retro/events/sd36-f3c3-executor.jsonl`:

- `class_census::tests::census_id_set_matches_the_published_partition`: 62 → **63 of 63**.
- `untabled_base_class_features::CLASS_SKILL_REMAINDER`: `[psion]` → `[]`. Citation:
  `up_classes.lst:221-256` is now carried.
- `tests/sd36_class_skill_from_record.rs`: the F3c2 test
  `expert_prints_its_canonical_class_skill_picks_and_psion_stays_refused_by_name` pinned Psion
  refused. It is now `expert_prints_its_canonical_class_skill_picks`, and Psion's pin moved to the
  new GREEN test in §3.
- `scripts/verify-baselines.env` `BASELINE_CENSUS_COMPUTED`: 62 → 63.

**Docs:**

- `docs/architecture/status.md`: the class table was regenerated with
  `python3 scripts/gen_class_status_table.py --json .../census-f3c3.json`, `--check` OK. The Dragon
  Disciple sentence now names mechanism N.
- `docs/architecture/rules-engine.md`: the class-skill paragraph now covers the sub-class choice.
- `forward-scope-register.md`: FS-16 CLOSED.

## 8. Verify (`f3c3-verify.log`, every command on the final tree)

| command | result |
|---|---|
| `cargo test --locked -j 8 -p codex-ingest --no-fail-fast -- --test-threads=8` | 167 result lines, 1,757 passed, 0 failed, 43 ignored. Includes `package_on_disk_is_fresh_and_clean`, the two §3 converter tests and `no_principal_carries_one_lines_condition`. |
| `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` (root) | 289 result lines, 6,355 passed, 1 failed, 27 ignored |
| the 1 failure | `every_kind_in_the_package_evaluates_to_a_well_formed_line` pinned 19 package kinds. F3c3 adds a 20th, `subclass` (52 rules: 12 numbers, 40 words). The pin moved 19 → 20 (retro correction logged). |
| `cargo test --locked -j 8 --lib -- --test-threads=8`, after the pin move | 2,723 passed, 0 failed, 6 ignored |
| class-skill tests (in the root run) | `psion_class_skills_answer_through_its_canonical_discipline`, `expert_prints_its_canonical_class_skill_picks`, the double-grant scan, `every_census_class_prints_the_class_skill_bonus_its_record_grants`: ok |
| reader, remainder and census tests (in the root run) | `every_census_class_has_a_known_proficiency_answer`, `every_untabled_class_outside_the_named_reader_remainder_reaches_computed`, `census_id_set_matches_the_published_partition`: ok |
| `cargo test --locked -j 8 --no-fail-fast --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8` | 615 passed, 0 failed |
| `cargo clippy --locked -j 8 -p codex-ingest --tests -- -D warnings` | clean |
| `cargo clippy --locked --tests -j 8 -- -D warnings` (root) | clean |
| `cargo run --locked --quiet -j 8 -p codex-ingest --bin sheet_rule_convert -- --check` (final package) | exit 0: `records=49450 converted=49450 refused=0 rules=73070 var_tables=6205 verdict=PASS` |
| `python3 scripts/check_class_census_baselines.py .../census-f3c3.json` with the env floors | OK: ids 137, computed 63, alone-blocked 74, mix 185, prestige mix 67 |
| `python3 scripts/pcgen_residue_gate.py --check --closure` | PASS (shipped_scanned 69,762, hits 0) |
| `python3 scripts/site/check_frozen_status.py --check` | OK, 49,450 units |

`git diff --quiet -- data/corpus site` is clean. Only `data/sheet_rules/**` moved under `data/`.
