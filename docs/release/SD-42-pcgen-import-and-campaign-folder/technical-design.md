---
canonical: true
owner: operator
bundle_id: SD-42
slug: pcgen-import-and-campaign-folder
canonical_branch: tranche-22
target_version: 0.22.0
date: 2026-11-16
status: planning-ready — every existing path below was checked with ls/grep -n/sed -n on tranche-22 @ 1046daefc0; paths marked (new) are design choices the owning card confirms; §1's root causes are HYPOTHESES until E1.1 proves them
---

# SD-42 Technical Design

> **`decisions.md §20` wins where this file differs.**

## §0 Today (tranche-22 @ `1046daefc0`; line counts from `artifacts/epic_0/census.json` `lines`)

| Area | File | Lines | What it does today |
|---|---|---|---|
| Import converter | `crates/codex-ingest/src/pcg_import/map.rs` | 1,033 | `convert()`; `read_equipment_sets` (:236), `read_item` (:500), `custom_item` (:603), `equipment_modifiers` (:960), `quantity` (:920), `finish` (:629) |
| | `catalog.rs` | 158 | `Catalog::load(data_root)`: classes from the census, feats from `all_feat_tables()`, equipment from `equipment_catalog_rows()` (keys first, names second), spells and skills from `data/sheet_rules/*/`, races from `data/corpus` |
| | `wire.rs` | 172 | `ImportFile { displayLabel, characterInput, bio, money, custom, report }`; `EquipmentWire { itemId, activeState, appliedModifiers }` (4 fields incl. the doc) |
| | `parse.rs` | 136 | the `KEY:value\|KEY:value` line format with `[...]` nesting |
| | `fixtures_tests.rs` | 422 | 13 repo fixtures + ≥ 42 oracle `.pcg`; asserts `sf_soldier` **cannot** compute (:386) |
| Import, desktop | `apps/desktop/src-tauri/src/character_hub.rs` | 23,869 | `import_pcg` (:8015) → sidecar → `import_pcg_json` (:7973) → `import_character_from_json` (:7792); writes bio, custom, money; **no** `equipment_sets.json` |
| Campaign, root | `src/campaign/mod.rs`, `local_store.rs` | 234, 721 | `CampaignSnapshot` (members as emails, `assets`, `party_character_ids`); `CampaignStore::save_with_conflict_detection` (:194, nonce sidecar) |
| | `src/saved_character/campaign_mirror.rs` | 273 | SD-41 §29: copies the character folder to `<campaign>/characters/<slug>-<id>/` after every writer |
| Campaign, desktop | `apps/desktop/src-tauri/src/campaign_drive.rs` | 446 | 5 `drive_*` commands over `CampaignStore`, writing `.config/<name>.json` under a "Drive folder" path the UI passes per request |
| | `apps/desktop/src/campaign/campaignModel.ts` | 232 | **localStorage** is the truth (`codex.campaigns`, `codex.campaign.assets.*`); the folder is a write-through mirror |
| | `CampaignManagerScreen.tsx` | 180 | `:61` shows `campaign.members.length` (invited emails only → "0 members") |
| | `CampaignSheet.tsx` | 423 | four asset tabs over plain textarea elements |
| | `CreateCampaignScreen.tsx`, `EditCampaignScreen.tsx` | 190, 346 | member emails; party from saved characters |
| | `settings/GoogleDrivePanel.tsx`, `googleDrive.ts`, `campaignManagerAccessGate.ts` | 192, 49, 27 | the "Drive folder" gate (a local path; no OAuth exists) |
| | `characterHub/CampaignMembershipDialog.tsx`, `campaignMembershipModel.ts` | 179, 102 | SD-41 E3.5: the sheet's membership dialog over `drive_list_campaigns` |
| DM toolkit | `apps/desktop/src-tauri/src/encounter_rating.rs`, `src/dmToolkit/encounterBuilderModel.ts` | 579, 57 | party = saved character ids or typed levels |
| Editor | `apps/desktop/package.json` | — | no Markdown or CodeMirror dependency (`grep -cE 'codemirror\|marked\|markdown' apps/desktop/package.json` → 0) |

## §1 Import defects — root-cause hypotheses (HYPOTHESES until E1.1's diagnosis run; file:line as of `1046daefc0`)

| # | Symptom (operator) | Hypothesis | Evidence to prove or refute (E1.1) |
|---|---|---|---|
| H1 | "Weapons came from the default character (longsword)" | `read_item` resolves the key **base item first** (`map.rs:510`: `[base_item, DATA.KEY, name]`), so the operator's `Mourn` (`BASEITEM:Longsword (Base)`) imports as a plain **Longsword**; its `NAME`, `_DAMAGE\|2d4`, `SPROP` and `Masterwork` eqmod are dropped (`equipment_modifiers`, `map.rs:960-983`, keeps `PLUSnW/A/S` only and warns about the rest). The Weapons tab then shows "Longsword", which reads as the default character's | the sidecar's report for the operator's file names `Mourn` → `equipmentSelections <longsword key>`; the warnings list its eqmods |
| H2 | "Armor: some came, not all" | (a) `read_equipment_sets` (`map.rs:236-262`) reads worn/carried only: a worn item's **slot** is never carried, so SD-41's `default_from` (`equipment_sets.rs:267`) infers a slot from the record and two body-slot garments (`Swashbuckler's Shirt`, `Spider-silk Bodysuit`, `Quick Runner's Shirt`) collide → `slot_conflict` → one stays active; (b) a base the catalog lacks → text-only custom record (`custom_item`, `map.rs:603`), never a worn copy | the per-item table: for each worn line, the location Codex gave it and the engine's diagnostics; the count of lines that became custom records |
| H3 | "Rations, trail listed many times" | one `EquipmentWire` **per copy** (`map.rs:534-535`), capped at `MAX_COPIES = 99` (`:156`); no `quantity` on the wire; `import_pcg_json` writes no `equipment_sets.json`, so each copy is its own stack of one bundle | `equipmentSelections` holds 84 rations rows for the operator's file (`QUANTITY:84.0`, `census.json`) |
| H4 | an item the character does not own appears | `quantity()` (`map.rs:920-924`) rounds `QUANTITY:0.0` **up to 1** (the operator's file has one such line, `census.json` `EQUIPNAME_quantity_0`) | the report shows the 0-quantity line as "1 carried" |
| H5 | an item maps in the sidecar and is missing in the app (or the reverse) | two catalogs: the sidecar's `Catalog::load` over `equipment_catalog_rows()` (`catalog.rs:74-79`) vs the app's `corpus_fixture_bundle()` the engine resolves against (`character_hub.rs:221`); a key one holds and the other lacks is dropped by the engine | the diff of the sidecar's mapped keys against the app's held list after import |
| H6 | a literal "longsword" in the importer | `FIGHTER_BONUS_SELECTION = "feat:weapon_focus:weapon:longsword"` (`map.rs:139`) is the Fighter bonus-feat row the importer adds; not the operator's case (Swashbuckler / Gunslinger) but a default worth naming | the Added rows of the report for a Fighter sample |
| H7 | a Starfinder file fails | `finish` (`map.rs:635`) only warns on `GAMEMODE:Starfinder`; classes resolve against the PF census → `unmapped_classes` → `Blocked` (`fixtures_tests.rs:380-386` asserts exactly this) | the report of `sf_soldier` |

E1.1 runs the built sidecar on the 8 files and writes, per file, a per-line table (key, outcome, Codex id,
worn/carried/slot, quantity) and the engine's verdict; the operator's file's table stays local
(`decisions.md §10`); the receipt replaces each "hypothesis" above with "proved" or "refuted" and the count.

## §2 The import after SD-42 (E1.2–E1.7, E4.1, E4.2)

```
.pcg ──pcg_import (sidecar)──▶ ImportFile JSON ──import_pcg_json──▶ character root
  EQUIPNAME×N + EQUIPSET tree     equipmentSelections (1 row / line)      envelope.txt (engine input)
  CUSTOMIZATION[DATA]             equipmentSets { active, copies[] }   ─▶ equipment_sets.json (locations, quantity, name, size, overrides, properties)
  base not in catalog             custom.equipment (Create items)      ─▶ custom.json
  unmapped lines                  bio.notes "## Imported from PCGen"  ─▶ bio.json
  GAMEMODE                        gameSystem pf1 | sf1                  ─▶ envelope game_system
```

- **`read_equipment_sets`** builds a tree: `id → (name, value, quantity, parent)` for the `CALCEQUIPSET` set;
  `location_of(line)`: the entry whose `VALUE` folds to the line's name/key/base — depth 3 with a slot word →
  `Slot` (the PCGen slot table), `Carried`/`Not Carried` → those, else `Equipped`; depth ≥ 4 → `Inside { container:
  key of the parent entry's line }`. A line no entry names → `Carried` (as today's "whatever no set names is carried").
- **`read_item`** emits one `EquipmentWire` and one `CopyWire { key, location, quantity, name, size, damage_override,
  weight_override_lbs, cost_override_copper, properties, eqmods }`; the Buy Custom build (`decisions.md §6`) is
  carried as `eqmods: Vec<String>` (the option keys) for E4.1 to price and weigh through
  `purchase_custom_equipment_at_root`'s pure core (the card names the function it extracts, `quote` today).
- **Keys:** `CopyKey { itemId, bonus, ordinal }` as `equipment_sets.rs` defines; the ordinal is the n-th line of
  that item id in file order, so the set file and the selections agree by construction (the theorem test of
  SD-41 E1.2 re-run over the new fixtures: `project(sets, copies) == active_states`).
- **E4.1** (`import_pcg_json`): after `import_character_from_json` saves, write `equipment_sets.json` from the
  block through `equipment_sets::save`, then re-run the engine so the first load projects the imported set (no
  `default_from` on an import — a test asserts the file exists before the first load and the held list has
  one row per line with its quantity).
- **E4.2:** `CopyFacts.damage_override` read in `resolve_weapon_damage_breakdown_with_carried` (SD-41 E1.4's
  entry point) — the row's die; `weight_override_lbs` beside `quantity` in `compute_encumbrance`'s stack weight
  and `container_loads`; `cost_override_copper` in the sell path; `properties` on `PlacedEquipmentCopyDto` and
  `WeaponDamageDto` → the gear screen's row, the Weapons tab, the printed Inventory and Weapons blocks.

## §3 Starfinder (E1.6)

`convert` → `Importer::new(..., mode)`; `Catalog::load_sf(data_root)` reads classes, themes, races, feats,
skills and equipment from `data/starfinder-1e/**` (the directory `sf_adapter.rs` loads its `SheetRulePackage`
from; the card names the loader it reuses); `SOURCE_PACKAGE_ID` becomes a per-mode constant; the wire gains
`gameSystem`; `import_character_from_json` sets `envelope.game_system` from it and computes through
`rule_system_adapter.rs`'s existing routing (`:185`). SF gear uses the same `equipmentSets` block (SF has no
body slots in the app's PF sense; worn → `Equipped`).

## §4 Parity (E1.7)

`scripts/oracle_harness/pcg_import_parity/run.sh <file.pcg> <out dir>` runs `scripts/pcgen-run-character.sh`
with `sheet-totals.txt.ftl` (PF) or `sf_parity.txt.ftl` (SF) plus a new `pcg-gear.txt.ftl` *(new)* listing every
gear line with quantity and location; `compare.py` reads Codex's snapshot JSON (the test writes it from the
engine) and the export; `crates/codex-ingest/tests/pcg_import_parity.rs` is the gate: every field equal or a row
of `explained.tsv` (build, field, pcgen, codex, why, decision), stale rows fail. The operator's file runs
through the same gate from its local path, output under `/home/ubuntu/sd42-input/parity/`.

## §5 The campaign folder (E2.1–E2.11)

```
campaign_folder.rs (desktop commands)          src/campaign/ (root crate, pure)
  list_known_campaigns / add_known_folder        folder.rs   (new) create_layout, open, campaign.json via CampaignStore::save_with_conflict_detection
  create_campaign_folder / open_campaign_folder  frontmatter.rs (new) tolerant YAML-subset read; write only what changed
  list_campaign_tree (help screen, live)         adventure_log.rs (new) list + sort by session; new entry
  read_campaign_file / write_campaign_file       maps.rs (new) pins in frontmatter; image bytes
  stat_campaign_file (mtime, size)               wiki.rs (new) folders, pages, [[link]] index, backlinks, search
  list_adventure_log / new_adventure_log_entry   characters.rs (new) list backups; master.json; import as stand-in / move master
  list_maps / add_map / set_map_pins
  list_wiki / new_wiki_page / wiki_backlinks / search_campaign
  list_campaign_characters / import_campaign_character
  save_attachment (drag-in image bytes → Attachments/)
  migrate_campaign
```

All file IO is in Rust (no fs plugin; the capability set gains only `opener:default`). Paths from the UI are
always **relative to the campaign folder** and the Rust side refuses `..` and absolute paths (a test). Every
write is temp + rename; SD-r retries once on a rename collision.

**UI:** `CampaignManagerScreen.tsx` (known folders, New, Open, Help, Migrate when needed) →
`CampaignSheet.tsx` (header; left: `Characters/` list; tabs Party Resources / Adventure Log / Maps / Wiki /
Help) → each tab a component under `apps/desktop/src/campaign/tabs/` *(new)* using the editor (§6).
`CharacterHubPage.tsx`'s `campaign-*` modes stay; `campaign-sheet` takes a folder path instead of an id. The
sheet's `CampaignMembershipDialog` lists known folders (E4.3).

## §6 The editor (E3.1–E3.3)

`apps/desktop/src/editor/MarkdownEditor.tsx` *(new)* mounts an `EditorView` (`@codemirror/view`) with
`markdown()` from `@codemirror/lang-markdown`, `history`, `defaultKeymap`, `search`, a `livePreview`
`ViewPlugin` (`livePreview.ts`), a `wikilinkCompletion` source (`wikilinks.ts`), the toolbar (`Toolbar.tsx`,
plain buttons dispatching transactions), `autosave.ts` (debounce, mtime check through `stat_campaign_file`),
`imageDrop.ts` (reads the dropped `File`, posts bytes to `save_attachment`, inserts the link) and
`readingView.ts` (`marked` with `html` tokens escaped). `roundTrip.ts` keeps `{ eol, trailingNewline, bom }`
from load and re-applies them on save; its test is the byte-stability proof over every `.md` the folder
template and the fixtures hold. Props: `{ path, campaignDir, onSaved }`; no global state.

## §7 Tests the bundle adds (names are the acceptance filters)

| Card | Test (crate / file) |
|---|---|
| E1.1 | `pcg_import_diagnosis_*` (`crates/codex-ingest/tests/pcg_import_diagnosis.rs`): writes the per-file tables; asserts the 7 samples convert |
| E1.2 | `pcg_import_stacks_*`: one copy per line; `QUANTITY` carried; locations incl. `Inside` nested; `QUANTITY:0` noted; 100,050 round-trips |
| E1.3 | `pcg_import_custom_*`: `NAME` → `name`; eqmod keys; `_DAMAGE`; `SPROP` → properties; `SIZE`; `COST`/`WT` overrides |
| E1.4 | `pcg_import_unmapped_*`: a base the catalog lacks → Create item at its location + Notes line |
| E1.6 | `pcg_import_sf_*`: `sf_soldier`, `sf_mechanic` compute as SF; the drone per Q-A; `fixtures_tests.rs:380` flipped |
| E1.7 | `pcg_import_parity` (gate), `explained.tsv` |
| E1.5 | `pcg_import_notes_*`: the Notes block lists every unmapped line, once |
| E2.1 | `campaign_folder_*` (root): layout, `campaign.json` schema 2, relative-path refusal |
| E2.2 | `campaign_members_*`: creator first, count ≥ 1, typed list round-trips |
| E2.3 | `campaign_known_folders_*`; `grep -c drive_ main.rs` → 0; TS `rg googleDrive` → 0 |
| E2.4 | `campaign_migrate_*`: old localStorage shape and `.config/<name>.json` → files; backup written; idempotent |
| E2.5 | `campaignHelpModel.test.ts`: every required sentence present (Drive example, any tool, Syncthing, Dropbox, Obsidian link, "everything is shared"); live tree |
| E2.6 | `party_resources_*`: starter file bytes; untouched by any Codex read |
| E2.7 | `adventure_log_*`: new entry frontmatter; sort by `session` (date and range); tolerant parse (SD-o) |
| E2.8 | `maps_*`: pins round-trip in frontmatter as fractions; image bytes; add map copies the file |
| E2.9 | `wiki_*`: starters; `[[Page]]`/`[[Page\|alias]]`/`[[Page#h]]` resolution; backlinks; search title-then-text |
| E2.10 | `characters_backup_*`: `master.json`; counter mismatch → no overwrite + warning; stand-in never writes; move master; `characters/` → `Characters/` rename |
| E2.11 | `rate_encounter_from_campaign_*`: a backup rated by its envelope |
| E3.1 | `roundTrip.test.ts` (sha256 equal over every fixture), `livePreview.test.ts` (decoration set per construct) |
| E3.2 | `wikilinks.test.ts` (completion source; click resolution) |
| E3.3 | `toolbar.test.ts`, `autosave.test.ts` (mtime change → banner), `imageDrop.test.ts` |
| E4.1 | desktop `import_writes_equipment_sets_*`: one held row per line with quantity and location; a new root every import |
| E4.2 | `copy_overrides_*` (root + desktop): damage die printed; weight and cost overrides in the sums; properties on the DTO and the print |
| E4.3 | `mirror_two_master_*` (desktop): `after_write` honours stand-in and the counter; membership dialog over folders |
| E4.4 | the smoke rows (`npm run ui-smoke -- --only <row>`) |
| E4.5 | `pcg_import_operator_file` (skips loudly), the local run's receipt |
