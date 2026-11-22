# `.pcg` character import

`pcg_import <file.pcg> --out <character.json> [--data-root <dir>]` (`crates/codex-ingest/src/bin/pcg_import.rs`)
converts a PCGen 6 character to the JSON the app's `import_character` reads. The app never parses a
`.pcg` (`decisions.md` §20 of SD-39); it runs this binary as a sidecar. `--data-root` is the directory
holding `data/`: a packaged app passes its resource directory.

## The file it writes

| Field | Read by |
|---|---|
| `displayLabel`, `characterInput` | `import_character`, as `ImportedCharacterFileDto` |
| `gameSystem` | the envelope's game system (`ImportedCharacterFileDto.gameSystem`): `starfinder-1e` for a `GAMEMODE:Starfinder` file, which computes through `sf_adapter::compute_sheet`; else Pathfinder |
| `bio` | `bio.json` (`CharacterBioDto`), by `import_character`, as an export's; `bio.notes` holds the Notes block (below) |
| `money.totalCopper` | `money.json`, by `import_character`, as an export's; nothing written at 0 |
| `custom.feats`, `custom.equipment`, `custom.spells`, `custom.grants` | `custom.json` (`CharacterCustomDto`); a line whose item Codex has no record of is a Create item in `custom.equipment` |
| `hitPoints` | `hit_points.json`: each level's `HITPOINTS`, when the file states every level's |
| `equipmentSets` | `equipment_sets.json` (`equipment_sets::save`): the imported `EQUIPSET` tree as one set, one stack per place a line is in, with its pieces |
| `report` | shown to the player: one line per line of the file, plus each row the file did not hold |

`import_pcg` (`character_hub.rs`) runs the sidecar and passes its JSON to `import_pcg_json`, which first
writes each line's material, masterwork and special-ability eqmods as a Buy Custom build on its selection
(`custom_equipment_quote`; a build it refuses keeps its `+n` and is a `### Custom parts not applied` line
in Notes), sets each Create item's worn, slot or left-behind state from the set, then saves through
`import_character_from_json` and writes `bio.json` and `equipment_sets.json` after it, so the Default
migration never runs on an import. Every import is a new character: a refused or failed import removes
the fresh character's directory.

**The Notes block.** Everything the import could not map is still imported and listed in `bio.notes`
under `## Imported from PCGen (<file>, <date>)`, one `###` heading per kind: an unknown key, a class or race
Codex lacks, an item with no Codex record (held as a Create item), a `QUANTITY:0` line (not held:
`<item>: 0 copies in the file`), an eqmod or `DATA` property kept as text, a line kept as a custom feat.

## How it ships

`apps/desktop/src-tauri/tauri.sidecar.json` names `binaries/pcg_import` as `bundle.externalBin`, and is passed
with `--config` only at bundling, because `tauri-build` fails on an absent `externalBin`. Each publish job in
`.github/workflows/publish-tester-release.yml` builds the binary and copies it to
`apps/desktop/src-tauri/binaries/pcg_import-<target triple>` first. An install holds it beside the app under
its bare name, which is where `import_pcg` looks; a dev run finds it there when the root workspace built into
the same `CARGO_TARGET_DIR`, and a missing one is a named error. `--data-root` is `codex_repo_root()`, the
directory the app itself reads `data/` from.

## What the engine makes it add

The app saves a character only when the rules engine computes it. These rows are in no PCGen file, and
each carries an `added` report line:

- The `power_attack` row. No item is added: a character computes with nothing worn or wielded.
- On a Fighter, `choice:fighter_bonus_feat` and the Weapon Focus it picks (`feat:weapon_focus`, unless the
  file holds it); a character with no Fighter level is given no Weapon Focus.
- On a Human, `choice:human_ability_bonus` for the floating +2, which the score is saved without.
- Each class's default picks from `class_seeds::canonical_seeds_for`. PCGen's own school, bloodline or
  domain is not read into them; a `DOMAIN` line is a custom feat.

A spell the engine's class list refuses (`class_spell.*`) would refuse the character, so it is a custom spell.

## What it does not import

- A file with no `PCGVERSION` or `CHARACTERNAME` line (PCGen 5 and older) is refused by name, exit 1.
- A class or race Codex has no record of is a custom feat and is named in `report.unmappedClasses` or
  `report.unmappedRace`. The file still converts; `import_character` then returns the engine's refusal.
  PCGen's test characters, its Modern files and monster classes are this case.
- An eqmod Buy Custom has no option for is a `properties` line on the copy, a report warning and a
  Notes line; the item still imports with its other properties.
- A companion file (a `MASTER` line, such as `sf_mechanic_drone.pcg`) is refused by name: import its master.
- An alternate racial trait or character trait is a custom feat.

## Key mapping

The table is `MAPPING` in `crates/codex-ingest/src/pcg_import/map.rs`, printed here word for word; the test
`pcg_import_fixtures_the_mapping_doc_lists_every_key` fails when the two differ. A key not in the table
becomes a custom feat and is named in `report.unknownKeys`. `GAMEMODE` is reported, and also picks the
catalog: `Starfinder` reads Starfinder records (`Catalog::for_game_mode`).

| `.pcg` key | Outcome | Goes to |
|---|---|---|
| `CLASS` | mapped | classLevels |
| `RACE` | mapped | raceId |
| `STAT` | mapped | abilityScores, plus the race's adjustments |
| `ABILITY` | mapped | CATEGORY:FEAT to selectedFeats + a target choice; a Favored Class Bonus taken as hit points or skill ranks to custom.grants and skillAllocations; other categories to a custom feat |
| `FEAT` | mapped | selectedFeats + a target choice (the older form of ABILITY) |
| `SKILL` | mapped | skillAllocations, ranks summed over CLASSBOUGHT |
| `SPELLNAME` | mapped | spellsSelected |
| `EQUIPNAME` | mapped | one equipmentSelections row and one equipmentSets stack per place the line is in, its QUANTITY the stack's pieces; BASEITEM first, PLUSnW/A/S to an applied +n modifier; the custom name and every DATA property per CUSTOM_PROPERTIES |
| `EQUIPSET` | mapped | equipmentSets: a slot word to a Slot (PCGEN_SLOTS), Equipped, Carried, Not Carried, or inside the entry above it; activeState is the set's projection |
| `CALCEQUIPSET` | mapped | which EQUIPSET is the set imported, and its name |
| `MONEY` | mapped | money.totalCopper, gp x 100 |
| `LANGUAGE` | mapped | the Languages custom feat (the Pathfinder sheet has no language list to fill) |
| `ALIGN` | mapped | bio.alignment |
| `DEITY` | mapped | bio.deity |
| `CHARACTERNAME` | mapped | displayLabel |
| `PLAYERNAME` | mapped | bio.playerName |
| `GENDER` | mapped | bio.sex |
| `AGE` | mapped | bio.age |
| `HEIGHT` | mapped | bio.height |
| `WEIGHT` | mapped | bio.weight |
| `HAIRCOLOR` | mapped | bio.hair |
| `EYECOLOR` | mapped | bio.eyes |
| `CHARACTERBIO` | custom | Biography |
| `CHARACTERDESC` | custom | Biography |
| `CHARACTERCOMP` | custom | Biography |
| `CHARACTERASSET` | custom | Biography |
| `CHARACTERMAGIC` | custom | Biography |
| `CHARACTERDMNOTES` | custom | Biography |
| `TABNAME` | custom | Biography |
| `EXPERIENCE` | custom | Biography |
| `HANDED` | custom | Biography |
| `SKINCOLOR` | custom | Biography |
| `HAIRSTYLE` | custom | Biography |
| `LOCATION` | custom | Biography |
| `CITY` | custom | Biography |
| `BIRTHDAY` | custom | Biography |
| `BIRTHPLACE` | custom | Biography |
| `PERSONALITYTRAIT1` | custom | Biography |
| `PERSONALITYTRAIT2` | custom | Biography |
| `SPEECHPATTERN` | custom | Biography |
| `PHOBIAS` | custom | Biography |
| `INTERESTS` | custom | Biography |
| `CATCHPHRASE` | custom | Biography |
| `REGION` | custom | Biography |
| `NOTE` | custom | a custom feat named by the note |
| `TEMPLATESAPPLIED` | custom | a custom feat per template |
| `TEMPLATE` | custom | a custom feat per template |
| `DOMAIN` | custom | a custom feat per domain (the cleric's engine pick stays its default) |
| `KIT` | custom | a custom feat |
| `MASTER` | custom | a custom feat |
| `FOLLOWER` | custom | a custom feat |
| `CHRONICLEENTRY` | custom | a custom feat |
| `TEMPBONUS` | custom | a custom feat |
| `CLASSABILITIESLEVEL` | mapped | hitPoints: each level's HITPOINTS, when the file states every level's; skill points per level are derived |
| `WEAPONPROF` | derived | proficiencies |
| `FEATPOOL` | derived | feat budget |
| `USERPOOL` | derived | ability pools |
| `FAVOREDCLASS` | derived | favored class |
| `AGESET` | derived | age category |
| `SPELLBOOK` | derived | the spell lists |
| `SPELLLIST` | derived | the spell lists |
| `PCGVERSION` | setting | PCGen's own setup; reported only |
| `VERSION` | setting | PCGen's own setup; reported only |
| `CAMPAIGN` | setting | PCGen's own setup; reported only |
| `CAMPAIGNS` | setting | PCGen's own setup; reported only |
| `GAMEMODE` | setting | PCGen's own setup; reported only |
| `CHARACTERTYPE` | setting | PCGen's own setup; reported only |
| `ROLLMETHOD` | setting | PCGen's own setup; reported only |
| `PURCHASEPOINTS` | setting | PCGen's own setup; reported only |
| `POOLPOINTS` | setting | PCGen's own setup; reported only |
| `POOLPOINTSAVAIL` | setting | PCGen's own setup; reported only |
| `PREVIEWSHEET` | setting | PCGen's own setup; reported only |
| `TABLABEL` | setting | PCGen's own setup; reported only |
| `AUTOSPELLS` | setting | PCGen's own setup; reported only |
| `USEHIGHERKNOWN` | setting | PCGen's own setup; reported only |
| `USEHIGHERPREPPED` | setting | PCGen's own setup; reported only |
| `LOADCOMPANIONS` | setting | PCGen's own setup; reported only |
| `USETEMPMODS` | setting | PCGen's own setup; reported only |
| `SKILLSOUTPUTORDER` | setting | PCGen's own setup; reported only |
| `SKILLFILTER` | setting | PCGen's own setup; reported only |
| `AUTOSORTSKILLS` | setting | PCGen's own setup; reported only |
| `IGNORECOST` | setting | PCGen's own setup; reported only |
| `ALLOWDEBT` | setting | PCGen's own setup; reported only |
| `AUTORESIZEGEAR` | setting | PCGen's own setup; reported only |
| `PORTRAIT` | setting | PCGen's own setup; reported only |
| `PORTRAITTHUMBNAILRECT` | setting | PCGen's own setup; reported only |
| `SUPPRESSBIOFIELDS` | setting | PCGen's own setup; reported only |
| `EXPERIENCETABLE` | setting | PCGen's own setup; reported only |

## Item properties

A line's customisation (`decisions.md §6` of SD-42) is `CUSTOM_PROPERTIES` in the same file, printed here
word for word under the same test. `PCGEN_SLOTS` there maps every location word of PCGen's Pathfinder
`equipmentslots.lst` to a body slot; a word in no row is `Equipped`.

| `EQUIPNAME` property | Goes to |
|---|---|
| `NAME` | CopyFacts.name, when it is not the record's own name |
| `EQMOD PLUSnW/A/S` | applied_modifiers, the copy's +n |
| `EQMOD material, masterwork, special ability` | eqmods: the Buy Custom option key (equipment_sets::eqmod_option_key) |
| `EQMOD / ALTEQMOD _DAMAGE\|<die>` | CopyFacts.damage_override, the die as written |
| `SIZE` | CopyFacts.size (Medium is no size) |
| `SPROP` | CopyFacts.properties, one line per '; '-separated part |
| `COST (line)` | CopyFacts.cost_override_copper, when it differs from the record's price; COSTMOD is already in it |
| `WT (line)` | CopyFacts.weight_override_lbs, when it differs from the record's weight |
| `KEY` | a lookup candidate for the record |
