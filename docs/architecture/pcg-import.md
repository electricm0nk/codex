# `.pcg` character import

`pcg_import <file.pcg> --out <character.json> [--data-root <dir>]` (`crates/codex-ingest/src/bin/pcg_import.rs`)
converts a PCGen 6 character to the JSON the app's `import_character` reads. The app never parses a
`.pcg` (`decisions.md` §20 of SD-39); it runs this binary as a sidecar. `--data-root` is the directory
holding `data/`: a packaged app passes its resource directory.

## The file it writes

| Field | Read by |
|---|---|
| `displayLabel`, `characterInput` | `import_character`, as `ImportedCharacterFileDto` |
| `bio` | `bio.json` (`CharacterBioDto`), by `import_character`, as an export's |
| `money.totalCopper` | `money.json`, by `import_character`, as an export's; nothing written at 0 |
| `custom.feats`, `custom.equipment`, `custom.spells` | `custom.json` (`CharacterCustomDto`) |
| `report` | shown to the player: one line per line of the file, plus each row the file did not hold |

`import_character_from_json` reads `custom`, `bio` and `money` as an export's. It has no field for `report`,
so the sidecar's caller, `import_pcg` (`character_hub.rs`), returns it with the outcome, and writes the bio
again after the save, an empty one included. A refused or failed import removes the fresh character's directory.

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
- `feat:weapon_focus`, unless the file holds Weapon Focus; `choice:fighter_bonus_feat` on any Fighter.
- Each class's default picks from `class_seeds::canonical_seeds_for`. PCGen's own school, bloodline or
  domain is not read into them; a `DOMAIN` line is a custom feat.

A spell the engine's class list refuses (`class_spell.*`) would refuse the character, so it is a custom spell.

## What it does not import

- A file with no `PCGVERSION` or `CHARACTERNAME` line (PCGen 5 and older) is refused by name, exit 1.
- A class or race Codex has no record of is a custom feat and is named in `report.unmappedClasses` or
  `report.unmappedRace`. The file still converts; `import_character` then returns the engine's refusal.
  PCGen's test characters, its Modern and Starfinder files, and monster classes are this case.
- `+1` to `+5` eqmods (`PLUSnW`, `PLUSnA`, `PLUSnS`) become applied modifiers. Every other eqmod (materials,
  special abilities) is a report warning (`decisions.md` R14).
- Stacks list one selection per copy, at most 99. An alternate racial trait or character trait is a custom feat.

## Key mapping

The table is `MAPPING` in `crates/codex-ingest/src/pcg_import/map.rs`; a test holds this list to it. A key
not in the table becomes a custom feat and is named in `report.unknownKeys`.

| `.pcg` key | Outcome | Goes to |
|---|---|---|
| `CLASS` | mapped | classLevels |
| `RACE` | mapped | raceId |
| `STAT` | mapped | abilityScores, plus the race's adjustments |
| `ABILITY` | mapped | CATEGORY:FEAT to selectedFeats + a target choice; other categories to a custom feat |
| `FEAT` | mapped | selectedFeats + a target choice (the older form of ABILITY) |
| `SKILL` | mapped | skillAllocations, ranks summed over CLASSBOUGHT |
| `SPELLNAME` | mapped | spellsSelected |
| `EQUIPNAME` | mapped | equipmentSelections, BASEITEM first, PLUSnW/A/S to an applied +n modifier |
| `EQUIPSET` | mapped | activeState: a location at the set's top level is worn; Carried, Not Carried and anything inside a container are carried |
| `CALCEQUIPSET` | mapped | which EQUIPSET is the worn one |
| `MONEY` | mapped | money.totalCopper, gp x 100 |
| `LANGUAGE` | mapped | the Languages custom feat (the Pathfinder sheet has no language list to fill) |
| `ALIGN` | mapped | bio.alignment |
| `DEITY` | mapped, `None` derived as no deity | bio.deity |
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
| `CLASSABILITIESLEVEL` | derived | hit points and skill points per level |
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
