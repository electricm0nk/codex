# SD-36 Epic F3 polish (F3p): receipt

Scope: the five F2/F3 merge-readiness polish items in `f2f3-polish.json` (P1-P5). P5 needed a
converter change, so this is a **converter step**: `data/sheet_rules/**` was regenerated under the
structural-diff protocol. P4's hit die is an **oracle** defect, so the data is left as it is and
the defect is registered as FS-23.

- Branch: `sd36/epic-f4-f5`, worktree `/home/ubuntu/workspace/worktrees/codex-epic-f4`.
- Baseline: tranche/16 HEAD `070c253e93`, extracted read-only with
  `git archive tranche/16 data/sheet_rules` from `/home/ubuntu/workspace/repos/codex` into scratch.
- Oracle pin: `7f818006e371188e5717fd18d74d18a420747fc6` (`$HOME/workspace/repos/pcgen/data`).
- Tests: `tests/sd36_f3_polish.rs` (new, 7 tests) and
  `multiclass_fold::tests::{every_slug_two_books_state_has_one_chassis_across_its_printings,
  printings_that_disagree_answer_nothing}` (new).

## RED -> GREEN

RED log: `f3p-red.log`. This was `cargo test --locked -j 8 --test sd36_f3_polish -- --test-threads=8
--nocapture` on the unchanged tree, and **6 of 6 failed**:

| Test | RED (before) | GREEN (after) |
|---|---|---|
| `p1_flurry_attack_bonus_in_a_mix_adds_the_other_classes_bab` | monk 4 + rogue 3 printed 2; expected 4 | 4; monk 4 + fighter 4 = 6; rogue 3 + monk 4 = 4; monk 4 alone stays 2 |
| `p1_scan_every_class_line_that_speaks_of_a_base_attack_bonus_is_classified` | 19 BAB-speaking lines found, 1 classified | 19 of 19 classified (§P1) |
| `p2_multiclass_save_explanation_lists_each_class_s_save_term` | `... across (class:barbarian 12: base attack bonus 12; class:fighter 1: base attack bonus 1)` | `class:barbarian 12: Fortitude 8; class:fighter 1: Fortitude 2.5` (Reflex/Will likewise) |
| `p4_hit_points_unknown_names_the_missing_chassis_record` | `class:monk: no converted class record states this class's hit points, ...` | names the missing chassis record |
| `p5_samurai_alone_prints_no_firearms_line` | samurai 1 held and printed `Exotic Weapon Proficiency ~ Firearms` | neither held nor printed, samurai 1 and magus 4 + samurai 2 |
| `p5_the_firearms_option_pick_holds_the_firearms_record_and_another_option_does_not` | the Katana pick held the firearms record | the Firearms pick holds it; the Katana pick does not |

P3 (`resolve_printings`) has no pre-change function to call, so its RED is a sabotage run instead
(`f3p-p3-sabotage.log`). `resolve_printings` was put back to the old `or_insert` behaviour ("the
first printing wins, whatever the others say"). `printings_that_disagree_answer_nothing` went red;
restored, it is green.

## P1: flurry BAB in a mix

**Rule.** Some class lines read the character's BAB with the class's own levels standing in for
that class's BAB. In a mix, such a line gets the BAB the other classes give added to it: the fold's
total BAB minus this class's own BAB (`multiclass_fold::LINES_READING_CHARACTER_BAB`,
`with_other_classes_bab`).

**Source.** CRB p.57, Flurry of Blows: the monk's BAB from monk levels equals her monk level, and
BAB from other classes adds.

**Hand-worked values.**
- Monk 4 + Rogue 3: 4 + 2 - 2 = **+4**.
- Monk 4 + Fighter 4: 4 + 4 - 2 = **+6**.

**Scan.** Population: 1,988 single-class builds, every census id (137) at every level from 1 to
its max level. They print 1,692 distinct class-line ids. 19 of them have a detail that speaks of a
base attack bonus, and each one is classified and pinned:

- **1 reads the character's BAB:** `class_chassis.monk.flurry_of_blows_attack_bonus`. The rule
  applies to it.
- **1 reads the character's BAB, but its class cannot join a mix:**
  `class_feature.pu.unchained_monk.flurry_attack_count`. Unchained Monk has no converted chassis,
  so the mix gate refuses it by name. The test asserts this for unchained monk 6 + fighter 2, and
  that no `multiclass.unchained_monk.*` line is copied.
- **17 read only their own class:**
  - 10 per-class BAB lines.
  - 3 companion BAB lines, from the companion's own HD.
  - The eidolon BAB, equal to summoner level.
  - Monk Maneuver Training, a delta on top of the counted BAB.
  - 2 lines whose words mention a BAB but whose value is not one (feral mutagen claw die; the
    unchained monk deferral marker).

Command: `cargo test --locked -j 8 --test sd36_f3_polish p1_scan -- --test-threads=8 --nocapture`
(output in `f3p-p1-bab-scan.log`).

## P2: save explanation terms

Each `class_chassis.base_save.<save>` detail in a mix now lists each class's own exact term for
that save. It used to list each class's BAB. The format is `class:<id> <level>: <Save> <value>`:
a whole number is printed bare, anything else to 3 decimals with trailing zeros trimmed. The values
are unchanged: the same `multiclass_member` saves are summed and floored once.

## P3: newest-printing chassis rows

**Denominator.** 136 conventional chassis-bearing class records over 133 slugs in every book. 3
slugs are stated by 2 books:
- `cyphermage`: adventurers_guide, inner_sea_magic.
- `hellknight`: adventurers_guide, inner_sea_world_guide.
- `red_mantis_assassin`: adventurers_guide, inner_sea_world_guide.

**What the converter's resolver says.** The resolver is `sheet_rule::reprint::newest_printing`:
one object is proved from the base row's non-`SOURCE*` tokens being identical, or from the rows
having prefix-ordered `DESC:`. Applied by hand to the three base class rows:
- **Hellknight:** the rows are identical (`iswg_classes.lst:23` = `ag_classes.lst:170`), so the
  newest printing is Adventurer's Guide (2017-06, over 2011-03).
- **Cyphermage:** not one object by the resolver's test. The rows differ in `NAMEISPI`, `FACT:Abb`
  and the save `PREVAREQ` tails (`ism_classes.lst:62` vs `ag_classes.lst:89`), and neither row
  states a `DESC:`.
- **Red Mantis Assassin:** not one object either. The ISWG row carries 4 `BONUS:VAR` and 2
  `DEFINE` caster-level tokens that the AG base row does not (`iswg_classes.lst:57` vs
  `ag_classes.lst:325`), and neither row states a `DESC:`.

**Runtime rule (`resolve_printings`).** The runtime package carries no publication order and no
one-object proof. So a slug that several books state is answered only when every printing states
the same chassis (`ClassChassis::same_chassis`: max level, hit die, skill ranks, save shapes, and
BAB and exact saves at every level). Printings that disagree answer nothing. This replaces
`or_insert`, where the alphabetically-first book won.

**Pin.** `every_slug_two_books_state_has_one_chassis_across_its_printings` names the 3 shared slugs
and asserts that each one's printings agree. It fails naming the slug if a printing ever differs.
It reads every book, so it also covers the book-precedence table in `generic_class_chassis`, which
answers these 3 prestige slugs first.

**Remainder, by mechanism.** When several printings agree, the book cited is the first in directory
order, `adventurers_guide` for all 3. For Hellknight that is also the resolver's newest printing.
To cite the resolver's newest printing in general, the converter would have to export its verdict,
because the runtime has no dates. No number depends on this.

## P4: monk hit die

**Oracle.** `cr_classes.lst:147` states `CLASS:Monk HD:10 ... SOURCEPAGE:p.56`. No
`CLASS:Monk.MOD` row in the pinned `pathfinder/` tree restates `HD:`; the rows checked were
`um_classes.lst:38`, `uc_classes.lst:47` and `pfs_cr_classes.lst:24/60`. So the **oracle itself
says 10**, while CRB p.56 says d8. That makes it an oracle defect, not an ingest defect. The data is
left as it is and the defect is registered as **FS-23** in `forward-scope-register.md`, on the same
rule as FS-15: it is closable only by a book-cited override, and that override must land before any
Monk chassis record makes Monk HP computable.

**Wording.** The Unknown message now says: `class:monk: no converted class chassis record (a class
record whose base attack bonus and save progressions converted, the record the fold reads a hit die
from), so its hit points are Unknown`.

## P5: option-gated feat grants (converter step)

**Rule** (`crates/codex-ingest/src/pcgen_import/sheet_rule/prereq.rs::holdable_gate`). A
`PREABILITY` item `<Base> (<Option>)` whose base resolves to a chooser converts to
`All[Holds(base), Chosen { choice: base, option: slug(<Option>) }]`, never to the bare base. A
chooser here is a record whose own rows carry a `CHOOSE:` other than `NOCHOICE`; its pick is keyed
by its own id, the same key its `Proficiency::Chosen` grant reads. An option written as a type
selector (`TYPE=Martial`, meaning any martial weapon) names no single option id. For those, the
option term is the printed condition `requires a <type> option chosen for <Base>`. The firearms
record now carries:
- `All[Holds(EWP), Chosen{EWP, "firearms"}]` for `uc_feats.lst:345`;
- `AtLeast{2, [All[Holds(EWP), Chosen{EWP, "firearms"}], Holds(gunsmithing)]}` for the PFS row.

**The pick path** (`src/rules_core/sheet_rule.rs`: `feat_sub_choices`, `HeldSeed::from_character`,
`CharacterFacts::with_linked_picks`). Without it, P5 would make every option gate unreachable for a
real character. The full root suite caught exactly that after the converter step: 2 tests went red,
`prestige_class_entry_gate_tests::arcane_archer_with_the_real_feats_qualifies` and
`prestige_class_entry_gate_wiring_tests::real_feats_surface_the_met_entry_gate_diagnostic`. The
character there records `"Weapon Focus (Longbow)"`, the catalog picker's shape, and nothing was
recorded under the Weapon Focus chooser.

The fix is ONE rule over the two shapes a feat's sub-choice takes:
- a selected feat `<Base> (<Option>)`, where the first parenthesis opens the option;
- a selected choice `feat:<base>:<kind>:<option>` (the GE-06 fixture's
  `feat:weapon_focus:weapon:longsword`).

Each such feat is seeded under its base slug. This moved out of `feat_prereqs::PrereqFacts::new`,
which was the only place that did it, so the sheet path now holds the feat too. Its option is then
recorded under the feat's converted chooser, the feat record whose `offers.id` is its own id.

Test: `p5_a_feat_recorded_with_its_option_holds_exactly_that_option_s_records`. A fighter 4 with
`Exotic Weapon Proficiency (Firearms)` holds the feat and the firearms record, with pick
`firearms`; with `(Katana)`, the feat is held and the firearms record is not. The RED for this
fix was a sabotage run (`f3p-pick-path-sabotage.log`): with the link disabled, this test and the
2 entry-gate tests fail, and with it restored all pass.

**Package delta** (`f3p_delta_pins.py`, `structural_diff_f3p_deltas.json`):
- **1,741 option terms**, on 465 (rule id, field) pairs across 465 rules and in 414 `_vars/` tables.
- By field: `_vars` 906, `applies` 426, `granted_by` 403, `prose` 4, `grants` 2.
- By kind: 1,711 `Chosen` option terms over 34 choosers, plus 30 type-selector conditions. The
  largest choosers are Ability Focus 550, Skill Focus 111, Weapon Focus 77, Slashing Grace 72, and
  Terrain Mastery 1-10 at 66 each.
- 0 deltas of any other shape.
- Records 49,450 -> 49,450; rules 73,363 -> 73,363; var tables 6,211 -> 6,211.

**Scan: grants conditioned on a chooser-bearing rule** (`scripts/f3p_chooser_grant_scan.py`,
`f3p-chooser-grant-scan.log`). A chooser-bearing rule is one whose `offers.id` is its own id: 1,597
of 73,363 rules.

| Population (denominator) | Conditioned on holding a chooser | Option-gated after F3p | Bare hold, no option term |
|---|---:|---:|---:|
| `granted_by` edges: 44,606 | 569 | 402 | 167 |
| record `applies` gates: 73,363 | 1,472 | 384 | 1,088 |
| `_vars/` contributions: 22,057 | 920 | 906 | 14 |

On the baseline, all 569 / 1,472 / 920 are bare. That means every one of the 402 + 384 + 906 was
satisfied by holding the feat with any option.

**Oracle proxy** (`scripts/f3p_option_gate_scan.py`, `f3p-option-gate-oracle-scan.log`; the whole
pinned `pathfinder/` tree, including books that were never ingested). There are 30,227
`PREABILITY` occurrences. Of these, 2,864 parameterised items are non-nested: 2,073 have a chooser
base, 200 have a declared base with no `CHOOSE`, and 591 have a base that no row declares.

**Remainders, by mechanism.**
1. **A chooser granted with a fixed option records no pick.** This is a different path from a
   feat the character picks (above). Samurai's `ABILITY:FEAT|AUTOMATIC|
   Exotic Weapon Proficiency (Katana)` holds the feat with nothing recorded under its chooser, so
   an option gate that reads that chooser answers Exclude for it. That is the right answer for
   Firearms, but it would also be Exclude for a Katana-gated line. Proxy count: 328 such grants in
   the pinned tree.
2. **A nested option stays unresolved.** An example is `Exotic Weapon Proficiency (Waraxe
   (Dwarven))`, and these remain FS-14 (`MissingRule`). Dwarven Waraxe Exotic Use
   (`_vars/v3ecc4923bf829539`) is one of them, and it is unchanged.
3. **A non-`TYPE=` option naming a group** can never match a specific pick. The case found is
   Loremaster's `Skill Focus (Knowledge)`, which prints as unmet and is never enforced.

**Census effect.** The census is `cargo run --locked -j 8 --bin class_census -- --json`, and the
output is `census-f3p.json`. It is unchanged against `census-f3c5.json` apart from the prestige
entry-requirement text, which now names the option; for example Arcane Archer now reads "requires
Weapon Focus and requires longbow chosen for core rulebook:feat:weapon focus". The counts:
- ids 137;
- non-prestige Computed 63 of 63;
- prestige alone Blocked 74 of 74;
- prestige carrier mixes Computed 68 of 74;
- mix panel 185 of 185.

## Converter gates

| Gate | Command | Result |
|---|---|---|
| write | `cargo run --locked --quiet -j 8 -p codex-ingest --bin sheet_rule_convert -- --write` | exit 0; records 49,450, refused 0; rules 73,363 (unchanged) |
| pins | `python3 .../scripts/f3p_delta_pins.py <tranche/16 pkg> data/sheet_rules .../structural_diff_f3p_deltas.json` | 0 unexplained; 465 field deltas + 414 tables pinned |
| structural diff | `python3 .../scripts/structural_diff.py data/sheet_rules --baseline <tranche/16 pkg>` | `verdict=PASS`, exit 0 (`f3p-structural-diff.txt`): F3p 465 of 465 + 414 of 414, 1,741 of 1,741 terms; unexpected field deltas 0; removed edges 0; removed grants 0 |
| planted mutations | `python3 .../scripts/f3p_planted_mutations.py <scratch> <repo> <tranche/16 pkg>` | **8 of 8 FAIL, both controls PASS** (`f3p-planted-mutations.txt`) |
| diff self-test | `python3 .../scripts/structural_diff_test.py` | 36 of 36 (+1: F3p normalizer, a withdrawn pinned gate, an unpinned gate) |
| freshness | `cargo run --locked --quiet -j 8 -p codex-ingest --bin sheet_rule_convert -- --check` | exit 0 |
| residue | `python3 scripts/pcgen_residue_gate.py --check --closure` | `verdict=PASS`, shipped_scanned 70,045, hits 0 |
| frozen | `python3 scripts/site/check_frozen_status.py --check` | OK, frozen at 100% (49,450 units) |
| bundle | `node scripts/gen-corpus-bundle.mjs` | `files_copied=14029`; tree unchanged |

The planted mutations were (labels as in the log):
- M1: the firearms gate is withdrawn back to a bare hold.
- M2: the option is changed to katana.
- M3: the option term names another chooser.
- M4: a second option term is added.
- M5: an unpinned option gate is put on Power Attack.
- M6: a pinned `_vars` table loses its option term.
- M7: the firearms record's EWP edge is dropped.
- M8: a type-selector condition (Low Templar) is replaced by an option id that no pick carries.

## Verify

Logs: `f3p-verify.log` (after the converter step), `f3p-full-root.log`, `f3p-full-rest.log`,
`f3p-final.log` (after the pick path).

| What | Command | Result |
|---|---|---|
| polish tests | `cargo test --locked -j 8 --test sd36_f3_polish -- --test-threads=8` | 7 passed |
| sd36 test files | the same command for `sd36_multiclass_any_class`, `sd36_class_skill_from_record`, `sd36_bloodline_pick_option` and `sd36_sorcerer_bloodline_record` | 12 / 5 / 2 / 11 passed |
| lib multiclass | `cargo test --locked -j 8 --lib multiclass -- --test-threads=8` | 39 passed |
| full root suite (after the pick path) | `cargo test --locked -j 8 --no-fail-fast -- --test-threads=8` | exit 0; 292 test targets, 0 failed (lib 2,725 passed, 6 ignored) |
| ingest, all targets | `cargo test --locked -j 8 -p codex-ingest --no-fail-fast -- --test-threads=8` | exit 0 |
| desktop, all targets | `cargo test --locked -j 8 --manifest-path apps/desktop/src-tauri/Cargo.toml --no-fail-fast -- --test-threads=8` | exit 0 |
| clippy root / ingest / desktop | `cargo clippy --locked --tests -j 8 [-p codex-ingest \| --manifest-path apps/desktop/src-tauri/Cargo.toml] -- -D warnings` | exit 0 / 0 / 0 |
| census | `cargo run --locked -j 8 --bin class_census -- --json census-f3p.json` | 63/63, 74/74, 68/74, 185/185; unchanged against `census-f3c5.json` apart from the 14 prestige ids' entry-requirement text |

The first wider run, after the converter step and before the pick path (`f3p-wider.log`: root lib plus 8 affected integration targets), had 2 failures: the
two entry-gate tests named in §P5. That run is how the pick-path gap was found. The first clippy
run after the pick path failed on one `nonminimal_bool` in `with_linked_picks`; it was fixed and
clippy re-run clean (`f3p-final.log`).
