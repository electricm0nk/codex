# F3c4b: ability-category pick rows convert as options of the choice that picks them (SD-36 Epic F3, converter step 4)

This is the fourth step of the F2/F3 batch that regenerates `data/sheet_rules/**`, under the same
F1c structural-diff protocol and the same single exception as F3b2, F3b2b and F3c3.

- `data/corpus/**` and `site/**` are untouched.
- The record count stays **49,450**. Pick rows become sheet rules, never inventory units.
- `python3 scripts/pcgen_residue_gate.py --check --closure` passes.

It closes the converter half of FS-17. F3c4 (`713bcfba74`, `f3c4-receipt.md`,
`f3c4-bloodline-sweep.md`) found every sorcerer bloodline line unreachable, 0 of 287. The variables
that switch the lines on are raised only by each bloodline's `CATEGORY:Sorcerer Bloodline` pick
row, and those rows sat outside the inventory (mechanism E, 65 unresolved-reference rows).

**Code:**

- `crates/codex-ingest/src/pcgen_import/sheet_rule/pool_option.rs` (new): the rule.
- `.../sheet_rule/mod.rs`:
  - `build_index` registers the pick rows' pairs before any record converts, and indexes a
    CATEGORY-less record under its row's declared category (§3).
  - `run` converts the options and folds their contributions and edges in.
- `.../sheet_rule/convert.rs` (the `ABILITY` arm): a grant of a `.MOD`-fragment object holds each
  fragment.
- `.../sheet_rule/ctx.rs`: the index fields.
- No engine code changed.

**Baselines:**

- The "before" package is F3c4's (`713bcfba74`, the same package as F3c3).
- The structural-diff baseline is tranche/16 (`cc21cac195`).
- Both were extracted read-only with `git archive` into scratch.

## 1. The oracle: what a pick row is

The citations are in the pinned oracle checkout (`~/workspace/repos/pcgen` at `7f818006e3`).

- **The pick.** `docs/listfilepages/globalfilestagpages/globalfileschoose.html:310`,
  `CHOOSE:ABILITYSELECTION|<category>|<criteria>`, "will produce a list of abilities ... matching the
  stipulated criteria".
  - A criterion is one of:
    - a name or key;
    - `TYPE=` (types joined by dots, all required);
    - `!TYPE=`;
    - a qualifier: `ANY` (the default), `ALL`, `QUALIFIED` or `PC`. A qualifier may be negated
      (`!PC`) and may wrap criteria (`QUALIFIED[TYPE=x]`).
  - "Criteria can be logically combined by using the comma (,), logical AND, and the pipe (|),
    logical OR. Logical ANDs are evaluated before logical ORs."
  - The code is `plugin/lsttokens/choose/AbilitySelectionToken.java`.
- **The pick applies the row.** `globalfilesother.html:290` gives the example
  `ABILITY:Special Ability|AUTOMATIC|%LIST <tab> CHOOSE:ABILITYSELECTION|Special Ability|TYPE=CuteBunnies`,
  which "would grant automatically, a Choice of type CuteBunnies". The picked row becomes an ability
  the character holds, so all of its tokens apply:
  - its `BONUS:VAR` raises its variables;
  - its `ABILITY:` hands out its targets;
  - its `PRE*` gates it.
- **The pool.** `pcgen/core/AbilityCategory.java`: the pool is every ability whose `CATEGORY:` is
  the category.
- **The worked row.** `Standard Bloodline` (`cr_abilities_class.lst:2329`) carries
  `CHOOSE:ABILITYSELECTION|Sorcerer Bloodline|!PC,QUALIFIED[TYPE=SorcererBloodlineChoice]` and
  `ABILITY:Sorcerer Bloodline|AUTOMATIC|%LIST`. The pick row `Draconic Bloodline` (`:2435`) has
  `CATEGORY:Sorcerer Bloodline` and `TYPE:SorcererBloodlineChoice`. It grants
  `Sorcerer Bloodline ~ Draconic` and raises `Sorcerer_Draconic_BloodlineClassSkill1` by
  `if(Sorcerer_CF_BloodlineClassSkill==0,1,0)`, plus the arcana, spells, powers and progression
  variables.

## 2. The rule (one mechanism, no per-pool case)

The rule is in `pool_option.rs`, and its module doc states it in full.

**A pick chooser** is an inventory record whose own closure carries BOTH of:

- `CHOOSE:[NUMCHOICES=n|]ABILITYSELECTION|<C>|<criteria>` (or `CHOOSE:ABILITY|<C>|...`);
- `ABILITY:<C>|<nature>|%LIST`.

The converter already writes such a choice as `offers: Rules { pool: slug(<C>) }`.

**A pick row** is an oracle ability row that meets all of these:

- it sits in the ability file family, outside `_pfs/`;
- it is a plain or `.COPY=` row, not an `ABILITYCATEGORY:` declaration;
- its category is `<C>`;
- a pick chooser's criteria select it;
- no inventory unit stands for it: no unit owns the row, and no unit answers its `(<C>, KEY)` or
  `(<C>, name)` pair.

Each pick row converts as ONE `pool_option` rule, `<book>:pool_option:<pool>_<name slug>`, through
the same `convert_record` every record goes through:

- `BONUS:VAR` becomes a `_vars/` contribution from the option;
- `ABILITY:` becomes a grant edge `Granter::Rule(<option>)`;
- `PRE*` becomes the option's gate.

It is granted by `Granter::Choice(<chooser>)` for every pick chooser that selects it. This is the
edge `subclass.rs` (F3c3) and `pool_pick.rs` (D8) already write, so the option is held exactly
when the character's recorded choice names it.

Its pairs are registered in the index before any record converts, and only where no unit answers
them. A record that names the row therefore resolves to the option. Two examples:

- Dragon Disciple's `Draconic Bloodline ~ Standard`:
  `ABILITY:Sorcerer Bloodline|AUTOMATIC|Draconic Bloodline` (`:2976`).
- The ACG blood-arcanist abilities' `!PREABILITY:1,CATEGORY=Sorcerer Bloodline,...`.

**Named, never guessed:**

- `pool-option-id-collision`, **1** row: `ma_abilities.lst:588/589`. Both rows are named
  `Relentless Healing` in `Path Dabbling`, one for Guardian and one for Hierophant. It is one key
  printed twice by the oracle, and the later row is named.
- `pool-option-unconverted`, **2** rows: two `iswg_feats.lst` Aldori feats whose names are product
  identity.
- `pool-option-criterion-unread`, `pool-option-chooser-unconverted`: **0** rows each.

### Enumeration: every converted choice pool

The denominators are pools and rows. The command is the `_defects/` files plus the package's
`pool_option` rules, with the pick-chooser test run over each choosing record's oracle rows.

| pool | converted choosers offering it | of them pick choosers | pick rows outside the inventory | converted as options | named, not converted |
|---|---:|---:|---:|---:|---:|
| `ability_focus` | 1 | 0 | 0 | 0 | 0 |
| `adoptive_parentage` | 1 | 1 | 0 | 0 | 0 |
| `aligned_class` | 1 | 0 | 0 | 0 | 0 |
| `arcanist_bloodline_development` | 1 | 1 | 3 | 3 | 0 |
| `blood_arcanist_bloodline` | 1 | 1 | 32 | 32 | 0 |
| `blood_of_dragons_bloodline` | 4 | 4 | 0 | 0 | 0 |
| `bloodrager_bloodline` | 2 | 2 | 10 | 10 | 0 |
| `class` | 7 | 0 | 0 | 0 | 0 |
| `class_skill` | 1 | 1 | 0 | 0 | 0 |
| `crossblooded_bloodline` | 1 | 1 | 12 | 12 | 0 |
| `crossblooded_rager_bloodline` | 1 | 1 | 11 | 11 | 0 |
| `draconic_mutagen_choice` | 1 | 1 | 0 | 0 | 0 |
| `dwarf_racial_trait` | 1 | 0 | 0 | 0 | 0 |
| `eldritch_heritage_bloodline` | 2 | 1 | 5 | 5 | 0 |
| `feat` | 34 | 6 | 14 | 12 | 2 |
| `gnome_racial_trait` | 1 | 0 | 0 | 0 | 0 |
| `heart_of_the_trait` | 1 | 1 | 5 | 5 | 0 |
| `internal` | 28 | 13 | 24 | 24 | 0 |
| `kobold_scale_color` | 2 | 2 | 0 | 0 | 0 |
| `mythic_weapon_training` | 1 | 1 | 0 | 0 | 0 |
| `normal` | 3 | 0 | 0 | 0 | 0 |
| `path_dabbling` | 1 | 1 | 128 | 127 | 1 |
| `raging_blood_feat_bloodline` | 1 | 1 | 1 | 1 | 0 |
| `sorcerer_bloodline` | 1 | 1 | 32 | 32 | 0 |
| `special_ability` | 152 | 6 | 0 | 0 | 0 |
| `subclass` | 2 | 0 | 0 | 0 | 0 |
| `virtual` | 3 | 0 | 0 | 0 | 0 |
| **total, 27 pools** | 255 | 46 | **277** | **274** | **3** |

In short: 27 converted choice pools, 19 of them offered by at least one pick chooser (46
choosers), and **12 pools** with at least one pick row outside the inventory. Of those **277 rows**,
274 convert (plus 16 line siblings, 290 rules) and 3 are named.

The other 7 pick-chooser pools have none: every row they select is already an inventory unit.
The UM crossblooded and eldritch-heritage rows, for example, were units, while their ARG/OA/MC
`support/*` twins were not. The 12 `feat` options are PCGen's hidden parameterised variants, such
as `Power Attack (Flurry)` and `Reach Spell +1`, which `Add a Feat (ignoring restrictions)`
(`ANY`) selects.

## 3. The two resolution gaps the sweep exposed (same batch, same protocol)

The first regeneration made **119 of 287** lines reachable. The class skill, arcana, bonus spells
and the powers header were reached, but no power was.

Every power's gate reads `Sorcerer_<X>_BloodlineProgressionLVL`. The pick row raises it by
`BloodlineProgressionLVL`, and that in turn is raised by `SorcererLVL` on `Bloodline Tracker`
(`cr_abilities_class.lst:1704-1707`). The sorcerer's `Sorcerer ~ Standard Bloodline` grants the
tracker with `ABILITY:Internal|AUTOMATIC|Bloodline Tracker`, and that reference was unresolved.

**Why it was unresolved (mechanism D):**

- The tracker's corpus record sits at the `.MOD` row
  `CATEGORY=Internal|Bloodline Tracker.MOD` (`:1705`). Its shipped tokens state no `CATEGORY:`, so
  it was indexed under the pair `("", "BLOODLINE TRACKER")`.
- The object has one base row (`:1704`, not a unit) and `.MOD` rows in **8 books**. The inventory
  filed each book's `.MOD` rows as a separate record.

**Two generic fixes, both resolution-only (no field of any existing rule moves):**

1. **A CATEGORY-less record is found under the category its own source row declares.** This is
   F3b2's declared-key rule applied to the category. It applies only where no record answers the
   pair.
   - Population: **235** corpus records have no shipped `CATEGORY:` but a category in their source
     row.
   - **43** unresolved references named such a record before the fix, all `Internal|...`. After
     it: **0**.
2. **One object filed as several `.MOD`-row records.** When every claimant of a declared pair comes
   from a `.MOD` row, the pair names ONE object, and PCGen applies every `.MOD` row to the object
   its key names. An `ABILITY` grant of it therefore holds each fragment
   (`CorpusIndex::mod_fragments`).
   - 7 pairs are affected: `Bloodline Tracker` (8 books), `Bloodrager ~ Bloodline Tracker` (2),
     `CMB` (2), `Archetype Display` (4), `Combat Trick Tracker` (2), `Default` (18) and
     `Vishkanya ~ Toxic ~ Vishkanya Venom` (3).
   - Any other shared pair is named as `row-declared-category-shared`, **1** row
     (`Special Ability|Repletion`, two unrelated monster abilities).

**A consequence for 62 class principals:** they are newly attested `closure_complete: true`. Their
one closure defect had been `core_rulebook:class_feature:cmb_output: Internal|CMB`, which now
resolves to the two `CMB` fragments. `closure_defects.py` on the F3c4 package gives, for example,
"expert: closure records 7; unresolved-references 1 ... Internal|CMB"; on this package it gives
"closure records 9; closure_complete [True]". The census does not move (§6).

## 4. Counts (before = F3c4 package; commands in `f3c4b-verify.log`)

| measure | before | after |
|---|---:|---:|
| inventory records / converted / refused | 49,450 / 49,450 / 0 | 49,450 / 49,450 / 0 |
| `rules_written` | 73,070 | **73,360** (+290 `pool_option` rules: 274 options, 16 line siblings) |
| `var_tables` | 6,205 | 6,209 |
| `_defects/unresolved-references.json` rows | 7,119 | **6,932** |

**Unresolved references by mechanism** (`artifacts/epic-f/scripts/unres2.py`, run per row over
each package):

- **Cleared: 217.**
  - Mechanism E: **149 of 3,565**, all of them pick rows or `.MOD` fragments now converted:
    - all 65 `Sorcerer Bloodline|<X>` rows;
    - 65 `Internal|...`: the `Bloodline Tracker` / `Bloodrager ~ Bloodline Tracker` / `CMB`
      fragments, and the `Internal` pick rows of Magical Talent, (Greater/Mythic) Elemental Focus
      and the eidolon elemental subtype;
    - 14 `Crossblooded Bloodline`;
    - 3 `FEAT` (`Power Attack (...)` and `Reach Spell +<n>` variants);
    - 1 `Bloodrager Bloodline`;
    - 1 `Crossblooded Rager Bloodline`.
  - Mechanism D: **40** `Special Ability|...`, CATEGORY-less records now found under their row's
    category (`Special Ability|Camel ~ Spit`).
  - Mechanism F: **28** `Special Ability|...`, the same fix. Their targets' rows were not cited in
    any closure, so unres2 classed them F (`Special Ability|Cacodaemon ~ Change Shape`).
- **New: 30.** These are references the options themselves state to rows outside the inventory:
  E 28 (for example `Internal|Aberrant Bloodrager Bloodline ~ Feat Tracker`, and 20
  `Eldritch Heritage Bloodline|...`) and F 2.
- **Mechanism totals** (unres2): E 3,565 → **3,444**, D 2,686 → 2,646, F 805 → 779, B 63 → 63.

**Sorcerer bloodline lines** (`f3c4b-bloodline-sweep.md`): **269 of 287** are reachable through the
pick at sorcerer 20, up from 0 of 287. 32 of 32 bloodline records now have their pick option.

## 5. What stays open, by mechanism

**Engine link (FS-17, the engine half). 31 of 32 bloodlines stay Blocked single-class.** No engine
code changed in this step:

- The Sorcerer module still refuses every non-Arcane bloodline by name.
- The character's Path-A `bloodline:<x>` pick is not linked to its `pool_option`.
- The class-skill reader answers per class from canonical seeds, not per character. Aquatic's Swim
  would lose its +3 (`f3c4-receipt.md` §3).
- The SD-32 generic pool-group magnitude pass is still level-ungated (`f3c4-receipt.md` §6).

Linking the pick is the next step. The converter half is done: each option holds its record and
lines at the levels the book states (`tests/sd36_bloodline_pick_option.rs`).

**The 18 unreachable lines (FS-19).** The Imperious and Kobold pick rows are race-gated.
`PREFACT:1,TEMPLATES,IsHuman=true` (`arg_abilities_class.lst:13`) converts as
`Holds core_rulebook:template:ishuman`. Only `template:race_human` grants that template, and in the
package three race traits grant `race_human`, never the race record itself. The oracle's
`TEMPLATE:Human` (`core_essentials/races/human/human_races.lst:6`) does not reach the package's
race. The Kobold row has the same shape.

**The 2 lines held late (FS-18).** Draconic and Abyssal Claws (`cr_abilities_class.lst:2444`)
convert to one line, their claw-size bonus gated `Power1LVL >= 7`. CONV-02 keeps a one-line
record's line as its principal (`principal_carries_more_than_its_line` does not count the record's
prose), so the record is held from sorcerer 7 where CRB p.75 grants it at 1.

**Dragon Disciple `Internal|Bite` (mechanism N): stays named.** The converter emits `NATURALATTACKS`
as `#natural<n>` Dice lines on the record that carries the token. That is not a named, holdable
natural-attack fact, so there is no fact to point `Internal|<attack>` at.

- The target `Bite` (`ce_abilities_race.lst:249`, `CATEGORY:Internal`,
  `TYPE:NaturalAttack.NaturalAttackPrimary.Primary`) is a variable/weapon-bonus helper. It carries
  no `NATURALATTACKS` of its own and no inventory unit stands for it.
- Across the package, **686 of 1,829** `Internal|...` unresolved references target a
  NaturalAttack-typed Internal helper row (Bite 417, Claw 178, Tail Slap 38, ...). Only 38 of them
  target a row that carries `NATURALATTACKS`.
- Resolving them needs a holdable natural-attack record per helper. That is not a pick row, so it
  is out of this rule and named as FS-20.
- Dragon Disciple's mix therefore keeps both of its blockers, and prestige mixes stay **67 of 74**.

## 6. Census (`cargo run --locked -j 8 --bin class_census -- --json <scratch>/census-f3c4b.json`)

```
ids=137 computed=63 blocked=0
prestige_swept=74 prestige_alone_blocked=74 prestige_mix_computed=67 prestige_mix_unknown=0
mix_panel_swept=185 mix_panel_computed=185 mix_panel_blocked=0
```

A per-key diff against `census-f3c3.json` finds only `generated_at` changed: 0 of 322 rows move.
`scripts/check_class_census_baselines.py` with the env floors (135/63/74/185/67) is OK. No
baseline moves, and `BASELINE_CENSUS_PRESTIGE_MIX_COMPUTED` stays 67.

## 7. RED to GREEN (`f3c4b-red.log`)

| test | RED (package on disk before `--write`) | GREEN |
|---|---|---|
| `sheet_rule_convert_gate::a_sorcerer_bloodline_pick_row_converts_as_an_option_of_the_bloodline_choice` | panicked: `core_rulebook/pool_option/sorcerer_bloodline_draconic_bloodline.json is in the package` | ok. Checks: the option is granted by `Choice(sorcerer_standard_bloodline_selection)` and by `Rule(draconic_bloodline_standard)`; it grants the record; its `BloodlineClassSkill1` contribution is present; `:2435` is no longer `outside_corpus_rows`; Aquatic's option grants its record; 0 `Sorcerer Bloodline\|` unresolved |
| `sheet_rule_convert_gate::a_category_less_record_is_found_under_the_category_its_row_declares` | panicked (the tracker had no `granted_by` from `sorcerer_standard_bloodline`: `[]`) | ok |
| `tests/sd36_bloodline_pick_option.rs::a_draconic_pick_reaches_its_lines_at_the_levels_the_book_states` | F3c4 measured 0 of 287 on the same probe | ok. Hand-worked from CRB p.75: Perception, arcana, bonus spells and the powers header at 1; Dragon Resistances from 3, Breath Weapon from 9, Wings from 15, Power of Wyrms from 20 (asserted absent below each); Claws asserted from 7 only (§5, FS-18) |
| `tests/sd36_bloodline_pick_option.rs::a_race_gated_pick_is_not_held_by_another_race` | -- | ok (a human's Kobold pick holds nothing) |
| `pool_option::tests::criteria_read_the_oracle_grammar` | -- | ok (AND before OR, `Q[inner]`, `!TYPE`, default `ANY`, unread named) |

## 8. Regeneration protocol (F1c)

| gate | command | result |
|---|---|---|
| write | `cargo run --locked --quiet -j 8 -p codex-ingest --bin sheet_rule_convert -- --write` | exit 0; records 49,450, refused 0; rules_written 73,070 → **73,360** |
| freshness | `... -- --check` | exit 0: `records=49450 converted=49450 refused=0 rules=73360 var_tables=6209 verdict=PASS` |
| pins | `python3 .../scripts/f3c4b_delta_pins.py <tranche/16> data/sheet_rules .../structural_diff_f3c4b_deltas.json <F3c4>` | 0 unexplained against the F3c4 package; details below the table |
| structural diff | `python3 .../scripts/structural_diff.py data/sheet_rules --baseline <tranche/16>/data/sheet_rules` | `verdict=PASS`, exit 0 (`f3c4b-structural-diff.txt`): unexpected field deltas 0, removed edges 0, removed grants 0, added rule ids 344 (0 with no named cause) |
| planted mutations | `python3 .../scripts/f3c4b_planted_mutations.py <scratch> <repo> <tranche/16 package>` | **11 of 11 FAIL, both controls PASS** (`f3c4b-planted-mutations.txt`) |
| diff self-test | `python3 .../scripts/structural_diff_test.py` | 34 of 34 (+1: a dropped or unpinned `pool_option`, an unpinned gate-term delta, an unreplaced fallback edge) |
| residue | `python3 scripts/pcgen_residue_gate.py --check --closure` | `verdict=PASS`, shipped_scanned 70,043, hits 0 |
| frozen | `python3 scripts/site/check_frozen_status.py --check` | OK, frozen at 100% (49,450 units) |
| bundle | `node scripts/gen-corpus-bundle.mjs` | `files_copied=14029`; tree unchanged |

**Pinned by `f3c4b_delta_pins.py`:**

- 290 `pool_option` rules by whole-rule sha256;
- 40 `f3c4b_gate_term_resolves` field deltas;
- 62 `f3c4b_closure_complete` field deltas;
- 6 replaced edges;
- 480 added `granted_by` edges on 356 rule ids;
- 1,189 `_vars/` contributions on 662 tables;
- 4 `_defects/` row counts.

**What the new diff guard checks** (`structural_diff.f3c4b_check`, with a module comment):

- **Added rules.** A pinned option that is missing, whose sha moved, or that nobody pinned.
- **Field deltas.** Each pinned (rule, field) delta's shape and value. There are two classes:
  - `f3c4b_gate_term_resolves`: old == new, except that `MissingRule` terms are now `Rule`.
  - `f3c4b_closure_complete`: a class principal newly attested.

  A pinned delta that is withdrawn also fails. The first run of M10 showed that a reverted
  attestation was invisible to a diff against tranche/16, and the guard was added.
- **Replaced edges.** The 6 removed edges are on `core_rulebook:feat:power_attack`. They are the
  fallback edges its parameterised references (`Power Attack (Light)`, `(Flurry)`, ...) had landed
  on through `resolve_holdable_rule`'s `" ("` split. Each is allowed only while the `pool_option`
  the reference now names exactly carries the same granter and `when`.
- **Growth.** Pinned edges, pinned contributions and defect-row counts.

**`_vars/` field moves against F3c4, all checked by the pins script:**

- 653 `outside_corpus_rows` shrink: the pick rows are now owned.
- 5 `declared_by` grow.
- 79 contributions changed only by `MissingRule` → `Rule` in their `when`.

**The planted mutations:**

1. The Draconic option loses its Choice edge.
2. The option → record edge is dropped.
3. Its `BloodlineClassSkill1` contribution is dropped.
4. A `pool-option-unconverted` row is dropped.
5. An unpinned option is planted.
6. The option's pool is changed.
7. The Bloodline Tracker edge is dropped.
8. Power Attack (Flurry)'s replacement edges are dropped.
9. A pinned gate delta takes another value.
10. A pinned attestation is withdrawn.
11. Every core_rulebook option is deleted.

## 9. Moved pins (fixture protocol)

The pin move is logged with `scripts/retro.py correction` in
`docs/retro/events/sd36-f3c4b-executor.jsonl`:

- `sheet_rule::evaluate_tests::every_kind_in_the_package_evaluates_to_a_well_formed_line`: package
  kinds 20 → **21**. `pool_option` is new: 290 rules, 56 numbers, 234 words.

**Docs:**

- `docs/architecture/rules-engine.md`: an ability-category pick paragraph.
- `forward-scope-register.md`:
  - FS-17: converter half CLOSED, engine half open.
  - FS-18: CONV-02 single-line principal.
  - FS-19: race template gate.
  - FS-20: natural-attack helper references, mechanism N.

## 10. Verify (`f3c4b-verify.log`, final tree)

| command | result |
|---|---|
| `cargo test --locked -j 8 -p codex-ingest --no-fail-fast -- --test-threads=8` | 167 result lines, **1,760 passed**, 0 failed, 43 ignored. That is +3: the two gate tests and the criteria unit test. |
| `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` (root) | 289 result lines, 6,355 passed, 1 failed (the kinds pin, §9), 27 ignored |
| `cargo test --locked -j 8 --lib -- --test-threads=8`, after the pin move | 2,723 passed, 0 failed |
| `cargo test --locked -j 8 --test sd36_bloodline_pick_option -- --test-threads=8` | 2 passed |
| `cargo test --locked -j 8 --no-fail-fast --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8` | 615 passed, 0 failed |
| `cargo clippy --locked -j 8 -p codex-ingest --tests -- -D warnings` | clean |
| `cargo clippy --locked --tests -j 8 -- -D warnings` (root) | clean |
| class census + baseline check | unchanged, OK (§6) |
| residue / frozen / bundle / structural diff | PASS / OK / unchanged / PASS |

`git diff --quiet -- data/corpus site` is clean. Only `data/sheet_rules/**` moved under `data/`,
plus the generated `scripts/oracle_harness/var_names.json` (+3 names).
