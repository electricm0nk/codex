# F7c receipt: class-skill lists hold only skills; requirement skill labels; canonical picks marked; magus finding

Stage F7, step F7c (worklist items F7-6, F7-7, F7-3 and F7-4). Branch `sd36/epic-f7-sheet-visible`,
on F7b `f317b310ca`. `data/sheet_rules/**`, `data/corpus/**` and `site/**` were not touched. This is
not a converter step. Every fix is in the live engine or the desktop. Logs are in this directory
(`f7c-*`). The ui-smoke evidence is in `artifacts/ui-smoke/f7/f7c/`.

## (a) F7-6: a class-skill list holds only skills

**Defect.** The Samurai's Mount record (`ultimate_combat:class_feature:samurai_mount`, from
`uc_abilities_class.lst:188`) grants `ClassSkill("samurai_mount")`. The class-skill reader listed
it as a class skill, and `list_class_facts` served it to the desktop.

**One rule** (`class_skill_sheet_rules::class_skill_view_with`). A class skill is a converted skill
record. A `ClassSkill` id that names no `skill` record in the package is not a skill. It is left
out of `skills` and named in the new `ClassSkillView::not_skills`.

**Package-wide measurement** (a Python walk of every `grants` entry in `data/sheet_rules`,
denominator 148 converted skill slugs). 3 `ClassSkill` ids name no skill record, on 2 records:

- `samurai_mount` (Samurai's Mount).
- `clear_diplomacy` and `clear_handle_animal` (ACG `spirit_warden_unnatural_mien`, an archetype
  feature). No class walk reaches this feature, because the reader answers the unarchetyped class.

**Scans.**

| Scan | Denominator | Result | Command / log |
|---|---|---|---|
| census classes at levels 1 and 20 | 274 (137 census ids x 2); 256 Known answers | classes with a dropped non-skill id: **1** (`samurai`: `samurai_mount`); served ids that are not skills: **0** | `cargo test --locked -j 8 --lib class_skill -- --test-threads=8 --nocapture` (`no_census_class_skill_list_names_a_non_skill`), `f7c-green-class-skill.log` |
| roster classes at levels 1, 7 and 20 | 177 (59 roster classes x 3); 150 Known (the 27 Unknown are the 9 ACG classes x 3, F7-10) | roster classes with a dropped non-skill id: **1 of 59** (`samurai`); served ids that are not skills: **0** | desktop `class_facts::tests::roster_class_skills_hold_only_skills_and_canonical_picks_are_marked`, `f7c-suite-desktop.log` |

The committed wire `stage-f6/f6a-class-facts-wire.json` loses its two `samurai_mount` entries
(Samurai at levels 1 and 7). It was regenerated with `CODEX_WRITE_F6A_WIRE=1`.

## (b) F7-7: a prestige requirement prints the skill's label

**Defect.** `level_up_option_filter::describe_expr` printed a skill term through `pretty(id)`:
`knowledge nobility ranks at least 3`.

**One rule** (`level_up_option_filter::skill_label`). A skill term prints the converted skill
record's label, and this applies to every skill term the requirement printer words: `SkillRanks`,
`SkillTotal` and `Holdable::ClassSkill`.

- The package label carries the source key as a trailing parenthetical: `Climb (Climb)`,
  `Knowledge (Nobility) (Knowledge (Nobility))`, `Craft (Tattoos) (Craft (Alchemy))`.
- When that parenthetical slugs to a converted skill, it is the key. The label is then the text
  before it: `Climb`, `Knowledge (Nobility)`, `Craft (Tattoos)`.
- Any other parenthetical is the name's own qualifier and stays: `Perception (Dim Light)`,
  `Lore (Dagon)`.
- The record's own capitalization is printed. The book writes `Knowledge (nobility)`, but a
  lowercasing rule would also lowercase proper-noun qualifiers (`Lore (Dagon)`), so none is applied.
- An id with no skill record prints `<words> (no skill record)`. The scan below counts 0 of these
  in the prestige gates.

The same printer words the Level Up dialog's entry requirements, the catalog's condition sentences
and the feat prerequisite lines, so all three now print the label.

**RED to GREEN.**
- `a_skill_term_prints_the_skill_label` and `no_prestige_requirement_line_prints_a_skill_slug` were
  RED: they did not compile, because `skill_label` did not exist (`f7c-red-root-lib.log`,
  `E0425`/`E0432`).
- Both are GREEN: `cargo test --locked -j 8 --lib -- --test-threads=8 level_up_option_filter
  class_proficiency_sheet_rules class_facts_sheet_rules sheet_rule_catalog` gives 65 passed
  (`f7c-green-filters.log`). `cargo test --locked -j 8 --lib prestige -- --test-threads=8
  --nocapture` gives 141 passed (`f7c-green-prestige.log`).

**Scan** (`class_census::tests::no_prestige_requirement_line_prints_a_skill_slug`). It covers
**74** prestige gates and 286 printed requirement lines. 67 of those lines carry 180 skill terms.
Terms that print a slug: **0**. Example: Aldori Swordlord prints `at least 4 of: Acrobatics ranks
at least 3, Intimidate ranks at least 5, Knowledge (Nobility) ranks at least 3, Sense Motive ranks
at least 3`.

The committed Level Up wire `stage-f4-f5/f4c-level-up-fighter6-wire.json` changes on exactly those
**67** lines. It was regenerated with `CODEX_WRITE_F4C_WIRE=1`.

## (c) F7-3: a Path-A canonical pick prints with the one `(default pick)` marker

**Defect.** Expert's ten class skills are the Path-A canonical picks recorded by the F3c2 ruling
(CRB p.450: "any ten", the player's choice). They printed as plain class skills. The Weapons
reader's seeded-pick line printed raw ids:
`Summoner Class Selection (advanced_players_guide:…) picks Standard Class (…) -- the Path-A canonical default`.

**One marker.** `class_seeds::DEFAULT_PICK_MARKER` = `"default pick"`, defined beside the one seed
table `canonical_seeds_for`. The desktop's `skillsModel.DEFAULT_PICK_MARKER` is the same string,
pinned by `skillsModel.test.ts`.

**One rule, keyed on the pick being a canonical seed** (`class_skill_view_with`). The reader walks
the class twice:

- once with the canonical seeds applied, which is the answer;
- once with only the player's own linked picks.

A class skill, or a whole family, is a **default pick** when either of these holds:

- every rule granting it is held only in the seeded walk;
- an Expert-style chooser pick (`add_canonical_class_skill_picks`) adds it, and no rule in the
  unseeded walk grants it outright.

These are served as `defaultPicks` and `defaultPickGroups` on `list_class_facts`. The Skills panel
and the allocation dialog print `(default pick)` beside a class skill that holds only as a default
pick in every held class that grants it (`ClassSkillLookup.isDefaultPick`). If another held class
grants the skill outright, as a Fighter does Climb in an Expert/Fighter mix, it prints plain. The
Weapons reader's seeded-pick line now reads `<chooser label>: <member label> (default pick)`.

**Count** (roster, 59 classes x levels 1, 7 and 20; `f7c-suite-desktop.log`):

| Surface | Roster classes printing a default pick | Per class (max over levels) |
|---|---|---|
| Skills panel / allocation dialog | **3 of 59** | Expert 10 skills (its whole list); Summoner 9 (6 skills + Craft, Knowledge and Profession, all through the canonical Standard class selection); Psion 7 (4 skills + 3 families, through the canonical Egoist discipline) |
| Weapons tab | **0 of 59** | the only roster class whose reader seeds a weapon pick is the Summoner, and it answers from its static `CLASS_WEAPON_PROFICIENCIES` row, which prints no pick line; the reader's own line carries the marker (`a_seeded_weapon_pick_prints_labels_and_the_default_pick_marker`) |

**Remainder, named by mechanism.** Of the 26 distinct seed choice ids in `canonical_seeds_for`, 22
are legacy `choice:*` ids: Sorcerer bloodline, Cleric domain, Wizard school and others. No converted
chooser carries them, so the class-level readers (`class_skill_view`, `class_weapon_proficiency_view`,
`canonical_member_picks`) never apply them. The class-facts surfaces therefore print nothing that
depends on those seeds, marked or unmarked. For example, the Arcane bloodline's class skill is not
part of the Sorcerer's class-level answer. The 4 converted-rule seed ids are all applied, and each
line they produce is marked: Summoner class selection, Expert class skills, Psion discipline, and
the Commoner weapon. The Commoner's weapon is not applied at class level; it prints as "one weapon
of the player's choice (39 options)".

The saved character's sheet lines (`sheet_lines_for`) read the character's recorded
`selected_choices`. A recorded choice does not say whether it was the create path's seed or the
player's pick, because `SelectedChoice` carries no provenance field. Those lines print as the
character's record.

**RED to GREEN.**
- `expert_picks_are_default_picks_and_a_fixed_list_has_none`,
  `psion_discipline_class_skills_are_default_picks`, `samurai_class_skills_hold_only_skills`,
  `no_census_class_skill_list_names_a_non_skill` and
  `a_seeded_weapon_pick_prints_labels_and_the_default_pick_marker` were RED on compile, because the
  fields and marker did not exist (`f7c-red-root-lib.log`). They are GREEN: 47 passed
  (`f7c-green-class-skill.log`) and 65 passed (`f7c-green-filters.log`).
- `skillsModel.test.ts` `verifiesCanonicalClassSkillPicksAreDefaultPicks` was RED against the
  pre-F7c wire with `Error: Acrobatics is an Expert default pick` (`f7c-red-ts.log`). It is GREEN
  in `npm test` (`f7c-desktop-npm.log`).
- `crates/codex-ingest/tests/class_weapon_proficiency_via_converter.rs`
  `summoner_reads_simple_through_the_standard_class_pick` now asserts the marker and no raw id. It
  passes: 6 passed, 1 ignored (`f7c-ingest-weapon.log`).

## (d) F7-4: Magus Knowledge (religion): oracle not contradicted; no FS row

**What the corpus states** (`artifacts/epic-f/scripts/f7c_magus_scan.py`, read-only over
`data/corpus/**`, output in `f7c-magus-scan.txt`). The scan walks 51,474 corpus records and finds:

- **1,036** Magus records (`data.class == "Magus"` or key/name `Magus`). 393 of them carry prose.
- No record anywhere has prose that states a magus class-skill list: **0** hits for "magus" together
  with "class skill".
- The Ultimate Magic class record (`ultimate_magic/class/magus.json`, `um_classes.lst:8`) has no
  description.
- **4** CSKILL statements, all on class-skill records:
  - the base row `um_abilities_class.lst:68`: Climb, Craft, Fly, Intimidate, Knowledge (Arcana),
    Knowledge (Dungeoneering), Knowledge (Planes), Profession, Ride, Spellcraft, Swim, Use Magic
    Device;
  - three archetype rewrites of it:
    - Spell Dancer (`arg_abilities_class.lst:121`) swaps Intimidate and Ride for Acrobatics and
      Perform (Dance).
    - Magic Warrior, printed in both `ag_abilities_class.lst:885` and `isi_abilities_class.lst:183`,
      swaps Knowledge (dungeoneering) and (planes) for (history) and (nature).
- Magus records naming Knowledge (religion) in any field: **0**.

**Closure.** Every statement the corpus carries omits Knowledge (religion): the base row and all
three rewrites. Each rewrite keeps the rest of the base list, which is consistent with a base list
that does not contain it. The corpus holds no book prose for the Magus class skills, so an FS-23
style oracle-defect row would have no source text to cite. Writing one from memory would be a
fabricated row (`decisions.md` §14.1). The item is closed as **oracle correct: no corpus text
contradicts it**. The sheet keeps printing the oracle row. No FS row was added and no data was
changed.

## Verification

| Scope | Command | Result |
|---|---|---|
| root `--lib class_skill` | `cargo test --locked -j 8 --lib class_skill -- --test-threads=8 --nocapture` | 47 passed, 0 failed (`f7c-green-class-skill.log`) |
| root `--lib prestige` | `cargo test --locked -j 8 --lib prestige -- --test-threads=8 --nocapture` | 141 passed, 0 failed (`f7c-green-prestige.log`) |
| root printer, readers, catalog | `cargo test --locked -j 8 --lib -- --test-threads=8 level_up_option_filter class_proficiency_sheet_rules class_facts_sheet_rules sheet_rule_catalog` | 65 passed, 0 failed (`f7c-green-filters.log`) |
| codex-ingest weapon reader | `cargo test --locked -j 8 -p codex-ingest --test class_weapon_proficiency_via_converter -- --test-threads=8` | 6 passed, 1 ignored (`f7c-ingest-weapon.log`) |
| desktop crate | `cargo test --locked -j 8 --manifest-path apps/desktop/src-tauri/Cargo.toml -- --test-threads=8 --nocapture` | 639 passed, 0 failed (`f7c-suite-desktop.log`) |
| frontend | `npm run typecheck && npm test` (apps/desktop) | tsc exit 0; 132/132 test files passed (`f7c-desktop-npm.log`) |
| clippy | `cargo clippy --locked -j 8 --lib --tests -p codex -- -D warnings`, then the desktop manifest `--tests` | ROOT_EXIT=0, DESKTOP_EXIT=0 (`f7c-clippy.log`) |
| ui-smoke, isolated root | `RUN_DESKTOP_AGENT=f7c-smoke node scripts/ui-smoke/run.mjs --only sheet-f7c-expert-skills-default-picks,sheet-f7c-samurai-skills-only-skills,level-up-f7c-prestige-skill-labels,sheet-f6a-samurai-weapons,sheet-action-skill-allocation-dialog,sheet-action-level-up-dialog,level-up-fighter6-blockers-and-labels --out docs/release/SD-36-consolidation/artifacts/ui-smoke/f7/f7c` | **7/7 green** (3 new F7c rows plus 4 existing Skills panel and Level Up rows); 7 created, 7 deleted, 0 leftover; the isolated root was removed (`f7c-smoke.log`, `ui-smoke/f7/f7c/results.json`) |
| real store | `ls ~/.local/share/io.electricm0nk.codex/characters \| wc -l`, before and after the smoke run | 1 and 1 (unchanged) |

New ui-smoke rows (`apps/desktop/scripts/ui-smoke/spec.json`; `docs/testing/ui-smoke-inventory.md`
re-rendered, 86 rows):

- `sheet-f7c-expert-skills-default-picks`: Human Expert 1. The Skills panel prints
  `(default pick)` (screenshot: all ten marked, Swim and Intimidate plain).
- `sheet-f7c-samurai-skills-only-skills`: Samurai 1. There is no `samurai_mount` and no
  `(default pick)`.
- `level-up-f7c-prestige-skill-labels`: Human Fighter 6, Level Up, Aldori Swordlord. It prints
  `Knowledge (Nobility) ranks at least 3` and `Sense Motive ranks at least 3`, and neither
  `knowledge nobility` nor `sense motive ranks` appears.

## Files

- Engine:
  - `src/rules_core/class_seeds.rs` (`DEFAULT_PICK_MARKER`)
  - `src/rules_core/pilot_compute/class_skill_sheet_rules.rs` (`not_skills`, `default_picks`,
    `default_pick_groups`)
  - `src/rules_core/pilot_compute/class_proficiency_sheet_rules.rs` (seeded-pick line)
  - `src/rules_core/level_up_option_filter.rs` (`skill_label`, `skill_words`)
  - `src/rules_core/class_census.rs` (scan test)
- Desktop:
  - `apps/desktop/src-tauri/src/class_facts.rs` (wire fields and roster scan)
  - `apps/desktop/src/boundary/listClassFacts.ts`
  - `apps/desktop/src/characterHub/skillsModel.ts` (`isDefaultPick`)
  - `CharacterSheet.tsx`
  - `SkillAllocationDialog.tsx`
  - `skillsModel.test.ts`
- Tests: `crates/codex-ingest/tests/class_weapon_proficiency_via_converter.rs`.
- Regenerated wires: `stage-f6/f6a-class-facts-wire.json`, `stage-f4-f5/f4c-level-up-fighter6-wire.json`.
