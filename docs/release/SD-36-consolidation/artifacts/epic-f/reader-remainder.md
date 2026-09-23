# SD-36 Epic F1 -- proficiency-reader remainder

The named remainder of `every_census_class_has_a_known_proficiency_answer`
(`src/rules_core/rules_tables/crb/weapon_tables.rs`). That test walks every class in
`codex::rules_core::class_census::census()` (135) that has no static `CLASS_WEAPON_PROFICIENCIES`
row, asks the converted-record reader (`class_proficiency_sheet_rules::class_weapon_proficiency_view`)
at every level `1..=max_level`, and passes only when the set of classes answering Unknown at any
level equals the `| class:` rows of the table below -- no silent tolerance in either direction. A
class that gains an answer must leave this table in the same commit; a class that loses one fails
the test by name.

Command: `cargo test --locked -j 8 --lib every_census_class_has_a_known_proficiency_answer -- --nocapture`
(prints the population line and every Unknown class with the reader's reason). Measured 2026-09-22
on sd36/epic-f1c after F1c-1 (grant-by-type selectors convert; package regenerated with
`sheet_rule_convert -- --write`, `--check` exit 0).

## Measured

- Census classes: **135** (61 non-prestige + 74 prestige). With a static row: **42** (all non-prestige).
  Walked by the reader: **93** (19 non-prestige + 74 prestige).
- Known at every level: **33 of 93** (18 of the 19 non-prestige; 15 of the 74 prestige).
- Unknown: **60 of 93** -- **1 non-prestige** (commoner: the only one of the 61 non-prestige census
  classes not Computed, `census-f1-reader.json`) and **59 prestige** (prestige classes are Blocked
  alone until F2 regardless -- census `prestige_alone_blocked` = 74 of 74).
- F1c-1 (2026-09-22) closed mechanism A: `ABILITY:<category>|AUTOMATIC|TYPE=<tag>` now converts to a
  grant edge onto every converted record of that category carrying the tag (the oracle's own
  `Weapon Prof ~ Auto` / `~ Simple` / `~ Martial` Internal abilities for the weapon case), so
  antipaladin, magus, holy_vindicator, low_templar and sentinel (65 -> 60 Unknown) answer Known.
  `_defects/grant-by-type.json`: 613 -> 24 rows, 0 of them `TYPE=WeaponProf*` (was 47).
- F1c-2 (2026-09-23, defect D2: a line's condition gates only its own line) did not move this
  table: the Unknown set is the same 60 of 93. What it moved is the 42 static-row classes the
  reader is checked against (`every_static_row_equals_the_reader_answer`): fighter, medium,
  mesmerist, psychic and spiritualist read Unknown because one line's condition (fighter's
  level-20 Weapon Mastery pool, the four occult classes' `<Class>_CF_Knacks` spell-cast line) had
  been hoisted onto the whole class-feature record; all five now reproduce their static row
  exactly, **32 of 42** static rows reproduced at level 1 (was 27).
- Commoner joined 2026-09-22 (reader batch blocker 2): it was Known with an incomplete closure --
  its one-simple-weapon pick is unseen, so every simple weapon read Known(false) and a Club took a
  wrong -4 on a sheet marked Computed. A Known view that carries an unresolved weapon pick now
  counts as Unknown here (mechanism F).
- Every mechanism B-E reason is the same reader verdict: "the converted closure of `<class>` at level 1 grants no
  weapon proficiency, and the package carries no closure-complete attestation that none is owed".
  The mechanism column says WHY the closure is empty, classified from
  offline evidence (test-side, never read by live code): the oracle class lines
  (`~/workspace/repos/pcgen/data/pathfinder/**/*class*.lst`, `ABILITY:`/`AUTO:WEAPONPROF` tokens
  naming a weapon proficiency, PRE-gates excluded), the converted package's `granted_by` edges, and
  `data/sheet_rules/_defects/grant-by-type.json` restricted to the converted closure (live code never
  reads `_defects/`).

## Mechanisms

| code | mechanism | classes | detail | closes in |
|---|---|---|---|---|
| A | grant-by-type not converted | 0 | CLOSED by F1c-1: the proficiency grant was a PCGen `TYPE=WeaponProf...` grant-by-type (`_defects/grant-by-type.json`); it now converts to grant edges onto the oracle's tagged `Weapon Prof ~ *` records (5 classes left this table) | converter: grant-by-type (spec §3.1(3)) -- done |
| B | proficiency record not converted | 1 | the oracle class line grants a named proficiency ability that has no converted record | converter: convert the record + regenerate |
| C | proficiency record not linked to the class | 2 | the proficiency record IS converted but carries no `granted_by` edge to this class (the oracle grants it from the class line) | converter: class-line link repair (F1 option A residue) + regenerate |
| D | proficiency record converted without its grant | 2 | the class's `... ~ Weapon and Armor Proficiency` record is converted and linked, but carries no `Proficiency` fact | converter: convert the record's weapon grant + regenerate |
| E | no weapon grant anywhere; attestation missing | 54 | neither the oracle class lines nor any rule in the converted closure names a weapon-proficiency grant (the class adds none), so the true answer is empty -- but spec §3.4 allows `Known(empty)` only with the per-class `closure_complete` attestation, which the converter does not write yet, so the reader answers Unknown rather than fabricate 'proficient with nothing' | converter: write `closure_complete` (spec §3.4) + regenerate |
| F | proficiency pick into an unlinked pool | 1 | the class walk holds a player's pick (`target: Pool(p)`, count > 0) whose pool has no converted member rule, on a record whose own name or pool names a proficiency (`class_proficiency_sheet_rules::unresolved_weapon_pick`). The reader cannot see what the pick covers, so every weapon its counted grants do not cover is Unknown (reader batch blocker 2) -- never Known(false). Measured 2026-09-22: 1,090 of the 1,092 pools a converted rule picks into have no member rule | converter: carry the `ABILITYCATEGORY` `TYPE` link from a pool to its member records + regenerate; the pick is then read from the character's choices |

Every mechanism closes in the converter + a package regeneration (A closed that way in F1c-1). None is a per-class special case
in live code, and none is closed by a new Rust row (rules_tables stay Rust until Starfinder; the
42 static rows stay).

## Remainder (the test reads the `| class:` rows)

| class | family | mechanism | evidence |
|---|---|---|---|
| class:aldori_swordlord | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:arcane_trickster | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:argent_dramaturge | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:aspis_agent | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:battle_herald | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:bellflower_tiller | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:body_snatcher | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:cerebremancer | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:commoner | base | F: proficiency pick into an unlinked pool | core_rulebook:class_feature:weapon_and_armor_proficiency_commoner picks 1 from pool `simple_weapon_proficiency_choice`; no converted rule is a member of it (the member, core_rulebook:class_feature:single_simple_weapon_proficiency, carries tag `SingleSimpleWeaponProficiency`; oracle `cr_abilitycategories.lst:136` `ABILITYCATEGORY:Simple Weapon Proficiency Choice ... TYPE:SingleSimpleWeaponProficiency` is the link the converter does not carry). The view is Known for `all_automatic_proficiencies`' four names and Unknown for every other weapon |
| class:cyphermage | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:dark_tempest | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:death_slayer | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:diabolist | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:divine_scion | prestige | D: proficiency record converted without its grant | oracle `Divine Scion ~ Weapon and Armor Proficiency`: converted as inner_sea_magic:class_feature:divine_scion_weapon_and_armor_proficiency, linked, but carries no proficiency grant |
| class:dragon_disciple | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:duelist | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:eldritch_knight | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:enchanting_courtesan | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:exalted | prestige | D: proficiency record converted without its grant | oracle `Exalted ~ Weapon and Armor Proficiency`: converted as inner_sea_gods:ability:exalted_weapon_and_armor_proficiency, linked, but carries no proficiency grant |
| class:golden_legionnaire | prestige | C: proficiency record not linked to the class | oracle `Weapon Prof ~ Auto`: converted as core_rulebook:class_feature:weapon_prof_auto but not linked to the class; oracle `Weapon Prof ~ Martial`: converted as core_rulebook:class_feature:weapon_prof_martial but not linked to the class; oracle `Weapon Prof ~ Simple`: converted as core_rulebook:class_feature:weapon_prof_simple but not linked to the class |
| class:gray_corsair | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:harrower | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:hellknight | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:hellknight_signifer | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:horizon_walker | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:lantern_bearer | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:lion_blade | prestige | B: proficiency record not converted | oracle `Lion Blade ~ Weapon and Armor Proficiency`: no converted record |
| class:loremaster | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:magaambyan_arcanist | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:mammoth_rider | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:master_chymist | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:master_spy | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:metaforge | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:metamind | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:metamorph | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:mystic_archer | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:mystic_theurge | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:nature_warden | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:pathfinder_chronicler | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:pathfinder_delver | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:pathfinder_savant | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:psicrystal_imprinter | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:psion_uncarnate | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:psychic_fist | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:pyrokineticist | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:rage_prophet | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:red_mantis_assassin | prestige | C: proficiency record not linked to the class | oracle `RMA Weapon Proficiencies`: converted as inner_sea_world_guide:class_feature:rma_weapon_proficiencies but not linked to the class; oracle class line `ABILITY:...\|TYPE=WeaponProfMartial` (grant-by-type, not converted) |
| class:rivethun_emissary | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:sanguine_angel | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:soul_archer | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:stalwart_defender | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:steel_falcon | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:storm_kindler | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:student_of_war | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:telekinetic_weaponmaster | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:thrallherd | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:twilight_talon | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:ulfen_guard | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:war_mind | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
| class:westcrown_devil | prestige | E: no weapon grant anywhere; attestation missing | oracle class lines: no weapon-proficiency grant; converted closure: no dropped `WeaponProf` grant-by-type |
