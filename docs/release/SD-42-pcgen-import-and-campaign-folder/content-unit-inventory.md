---
canonical: true
owner: operator
bundle_id: SD-42
slug: pcgen-import-and-campaign-folder
canonical_branch: tranche-22
target_version: 0.22.0
date: 2026-11-16
status: planning-ready — every figure carries its command; measured on tranche-22 @ 1046daefc0 on 2026-11-16 by artifacts/epic_0/census.py (output artifacts/epic_0/census.json) unless stated
---

# SD-42 Content Unit Inventory

An import-and-folder bundle counts file shapes, lines, commands and rows rather than book content. Every
figure below is in `artifacts/epic_0/census.json` under the key named, produced by
`python3 -I docs/release/SD-42-pcgen-import-and-campaign-folder/artifacts/epic_0/census.py` from the repo root
(the operator's file read from `$CODEX_SD42_PCG`, the samples from `$PCGEN_REPO_DIR`; counts only, never a
line of the file). Denominators are stated with each figure.

## §1 The proof set (`census.json` `operator_file`, `samples`; oracle `7f818006e371188e5717fd18d74d18a420747fc6` = the pin)

| ID | Figure | Value | Key / command |
|---|---|---|---|
| F-1 | `EQUIPNAME` lines in the operator's file | 89 | `operator_file.EQUIPNAME` (= `awk '/^EQUIPNAME:/{n++} END{print n+0}' "$CODEX_SD42_PCG"`) |
| F-2 | …of which customised (`CUSTOMIZATION`) | 38 of 89 | `operator_file.EQUIPNAME_with_CUSTOMIZATION` |
| F-3 | …with `QUANTITY` > 1 (stacks) | 21 of 89 | `operator_file.EQUIPNAME_quantity_gt_1` |
| F-4 | …with `QUANTITY` 0 | 1 of 89 | `operator_file.EQUIPNAME_quantity_0` (SD-d) |
| F-5 | the largest stack in the operator's file | 84 | `operator_file.EQUIPNAME_max_quantity` |
| F-6 | `EQUIPSET` lines; depth by id components (2 / 3 / 4 / 5) | 87; 1 / 33 / 17 / 36 | `operator_file.EQUIPSET`, `EQUIPSET_depth_by_id_components` |
| F-7 | `DATA` field kinds in the operator's file | `NAME` 36, `SPROP` 17, `KEY` 13, `EQMOD` 11, `ALTEQMOD` 1 | `operator_file.DATA_field_keys` |
| F-8 | `EQUIPNAME` lines per sample (pf_Cleric / pf_Paladin / pf_Rogue / pf_goldielocks / sf_mechanic / sf_mechanic_drone / sf_soldier) | 20 / 29 / 41 / 4 / 22 / 0 / 4 | `samples.<name>.EQUIPNAME` |
| F-9 | customised lines per sample | 5 / 12 / 14 / 4 / 6 / 0 / 4 | `samples.<name>.EQUIPNAME_with_CUSTOMIZATION` |
| F-10 | `DATA` kinds the samples add beyond F-7 | `SIZE` (pf_Rogue 10, sf_mechanic 2), `COSTMOD` (pf_Paladin 1, pf_Rogue 1) | `samples.*.DATA_field_keys` |
| F-11 | the largest stack in any sample | 100,050 (sf_mechanic) | `samples.sf_mechanic.EQUIPNAME_max_quantity` |
| F-12 | game modes | 5 × `Pathfinder_RPG` (incl. the operator's), 3 × `Starfinder` | `*.GAMEMODE` |
| F-13 | classes in the proof set | Swashbuckler+Gunslinger; Cleric; Paladin; Rogue; Sorcerer/Cleric (Arcane)+Dragon; Mechanic; Drone; Soldier | `*.CLASS` |

## §2 The importer today (`census.json` `importer`, `lines`)

| ID | Figure | Value | Key / command |
|---|---|---|---|
| F-14 | lines in `map.rs` / `catalog.rs` / `wire.rs` / `parse.rs` / `fixtures_tests.rs` | 1,033 / 158 / 172 / 136 / 422 | `lines.<path>` |
| F-15 | `MAX_COPIES` | 99 (`map.rs:156`) | `importer.MAX_COPIES` |
| F-16 | the per-copy emit lines; base-first lookup; eqmod filter; custom item; quantity; game-mode warning; the longsword literal; the SF refusal test | `map.rs:534-535`; `:510`; `:960`; `:603`; `:920`; `:635`; `:139`; `fixtures_tests.rs:386` | `importer.*_line` |
| F-17 | repo `.pcg` fixtures (`scripts/oracle_harness/eqm-fixtures`) | 13 | `importer.repo_eqm_fixtures` |
| F-18 | `EquipmentWire` fields today | 3 (`itemId`, `activeState`, `appliedModifiers`; the awk counts the doc line too → 4) | `importer.wire_EquipmentWire_fields` |

## §3 The campaign code today (`census.json` `campaign`, `lines`)

| ID | Figure | Value | Key / command |
|---|---|---|---|
| F-19 | `campaign_drive.rs` lines / `#[tauri::command]`s / registration lines in `main.rs` | 446 / 5 / 7 | `lines`, `campaign.campaign_drive_tauri_commands`, `campaign.drive_commands_registered` |
| F-20 | **Lane disjointness:** files the `import` lane names ∩ files the `campaign` or `editor` lane names | 0 | `comm -12 <(sort import.txt) <(sort campaign-editor.txt)` over the FILES YOU OWN of E1.x vs E2.x/E3.x in `epic-breakdown.md` (the lead derives the two lists before launch: `awk -F'\|' '$2 ~ /^ E1\./ {print $4}' epic-breakdown.md` and the E2/E3 twin) |
| F-21 | **SD-41 partition:** Phase-1 files ∩ files SD-41's open cards own | 0 | `git diff --name-only <C1 sha>..HEAD -- apps/desktop/src-tauri/src/character_hub.rs apps/desktop/src/characterHub/BuyCustomDialog.tsx docs/architecture scripts/verify-baselines.env \| awk 'END{print NR}'` (G-3); SD-41 open cards: `sd41_open_cards` (7 of 56) |
| F-22 | `CampaignManagerScreen.tsx` line showing `members.length` | 61 | `campaign.members_length_line` |
| F-23 | TS files naming the Drive path | 13 | `campaign.ts_files_naming_drive` (`grep -rliE 'googleDrive\|driveFolderPath\|drive_list_campaigns\|drive_save_campaign\|write_campaign_drive' apps/desktop/src --include=*.ts --include=*.tsx \| awk 'END{print NR}'`) |
| F-24 | campaign ui-smoke rows | 8 of 203 | `campaign.campaign_smoke_rows`; ids in `campaign_smoke_row_ids` |
| F-25 | `campaign_mirror.rs` lines naming `characters` as the target dir | 7 lines (`:80,190,191,212,215,245,261`) | `campaign.mirror_target_literal` |
| F-26 | Markdown / CodeMirror dependencies in `package.json` | 0 | `campaign.markdown_dependency_in_package_json` |
| F-27 | the Market's stack merge on buy (SD-41 E2.5) | `character_hub.rs:4181` | `campaign.market_buy_merges_into_stack_line` |
| F-28 | Tauri capabilities today | `core:default`, `dialog:default` (2) | `campaign.tauri_capabilities` |
| F-29 | lines: `character_hub.rs` / `main.rs` / `campaign/mod.rs` / `local_store.rs` / `campaign_mirror.rs` / `equipment_sets.rs` / `CampaignSheet.tsx` / `encounter_rating.rs` | 23,869 / 435 / 234 / 721 / 273 / 519 / 423 / 579 | `lines.<path>` |

## §4 Version, branch, board

| ID | Figure | Value | Key / command |
|---|---|---|---|
| F-30 | version surfaces SD-41's C1 bumped (excluding docs) | 14 | `version_surfaces_sd41_c1` (`git show --name-only --format= c817ad502f \| awk 'NF' \| grep -v '^docs/' \| awk 'END{print NR}'`) |
| F-31 | base | `tranche-22` @ `1046daefc0` = `merge-base origin/tranche-21` | `base.*` |
| F-32 | cards | 41 (2 + 9 + 14 + 4 + 7 + 5; 37 at authoring + E1.8 + E2.12, E2.13, E4.6 — `decisions.md §26`) | `awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/' kanban.md \| awk 'END{print NR}'` |
| F-33 | editor packages and licences (npm registry, 2026-11-16) | 11, all MIT; versions in `decisions.md §18` | `for p in @codemirror/state @codemirror/view @codemirror/language @codemirror/commands @codemirror/autocomplete @codemirror/search @codemirror/lang-markdown @lezer/markdown @lezer/highlight @lezer/common marked; do npm view $p version license; done` |

## §5 What the census does not count

The engine's verdict per file (E1.1 produces it); which eqmods Codex's option list holds (E1.3 derives it from
`list_custom_equipment_options`' table); the number of parity fields per build (E1.7's output); the PCGen slot
table's length (E1.2 reads `equipmentslots.lst` at the oracle).
