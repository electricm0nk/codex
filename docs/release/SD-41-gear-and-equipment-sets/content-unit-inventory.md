---
canonical: true
owner: operator
bundle_id: SD-41
slug: gear-and-equipment-sets
canonical_branch: tranche-21
target_version: 0.21.0
date: 2026-11-12
status: planning-ready — every figure carries its command; measured on tranche-20 @ f7f85f3d24 on 2026-11-12 by artifacts/epic_0/census.py unless stated
---

# SD-41 Content Unit Inventory

A gear bundle counts states, slots, producers, containers, rows and tests rather than book content. Every
figure has its command and its denominator; `census.py` (run with `python3 -I` from the repo root) prints
them all with their commands into `artifacts/epic_0/census.txt`.

## §1 Version and package surfaces

| F | Figure | Value | Command |
|---|---|---|---|
| F-1 | Version surfaces C1 bumps to `0.21.` (the files SD-40's C1 bumped, less the pointer) | **14** | `git show --name-only --format= 3ceeb63b66 \| awk 'NF' \| grep -vc current-release.json` |
| F-2 | C1 commit file count (F-1 + `current-release.json`) | **15** | `git show --name-only --format= <sha> \| awk 'NF' \| wc -l` |
| F-3 | Cards in this bundle | **23** | `awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/' docs/release/SD-41-gear-and-equipment-sets/kanban.md \| awk 'END{print NR}'` |
| F-4 | Chassis files present | **16** (13 required + `technical-design.md`, `technical-requirements.md`, `kanban.md`) | `find docs/release/SD-41-gear-and-equipment-sets -maxdepth 2 -name '*.md' \| awk 'END{print NR}'` |

## §2 The gear model today

| F | Figure | Value | Command |
|---|---|---|---|
| F-5 | `EquipmentSelection` struct literals (producers a new field would touch; why NR-3 leaves it alone) | **97** | `grep -rnF 'EquipmentSelection {' src apps/desktop/src-tauri/src crates --include=*.rs \| grep -v 'struct EquipmentSelection' \| awk 'END{print NR}'` |
| F-6 | `ActiveState` variants; the DTO lines | **3**; `EquipmentEffectsDto` :262, `CarriedItemDto` :298, `EncumbranceDto` :319, `HeldEquipmentCopyDto` :4968 | `grep -n 'pub enum ActiveState' -A6 src/rules_core/character_input.rs`; `grep -n 'pub struct CarriedItemDto\|pub struct EquipmentEffectsDto\|pub struct EncumbranceDto\|pub struct HeldEquipmentCopyDto' apps/desktop/src-tauri/src/character_hub.rs` |
| F-7 | `BodySlot` variants; `WORN_SLOTS` rows; slot-table rows; slots with capacity ≠ 1 | **18**; **18**; **26**; Ring 2, none 2 | `grep -n 'pub enum BodySlot' -A19 src/rules_core/body_slots.rs \| grep -cE '^[0-9]+-    [A-Z]'`; `awk '/^export const WORN_SLOTS/,/^\];/' apps/desktop/src/characterHub/EquipmentPanel.tsx \| grep -c "slot: '"`; `python3 -I -c "import json;d=json.load(open('data/equipment_types.json'));print(len(d['slots']),[(s['body_slot'],s['capacity']) for s in d['slots'] if s['capacity']!=1])"` |
| F-8 | On-disk state line; the store's file names; the envelope schema version | `equipment={}:{}` at `local_store.rs:458`; `envelope.txt`, `authoritative_character_input.txt`; **2** | `grep -n 'equipment={}:{}' src/saved_character/local_store.rs`; `grep -n 'const ENVELOPE_FILE\|const CHARACTER_INPUT_FILE' src/saved_character/local_store.rs`; `grep -n 'CURRENT_SAVED_CHARACTER_SCHEMA_VERSION' src/saved_character/mod.rs \| head -1` |
| F-9 | Equipment Tauri commands today | **6** (`add_equipment_selection` :3618, `attach_equipment_modifier` :4186, `purchase_equipment` :4221, `list_held_equipment` :5094, `set_equipment_active_state` :5179, `remove_equipment_selection` :5634) | `grep -nE '^pub (async )?fn (add_equipment_selection\|attach_equipment_modifier\|purchase_equipment\|list_held_equipment\|set_equipment_active_state\|remove_equipment_selection)' apps/desktop/src-tauri/src/character_hub.rs` |
| F-10 | The engine's state reads: AC (`EquippedActive` only); weight (all but `Absent`); weapons (`EquippedActive` only) | `equipment_effects.rs:207`; `encumbrance.rs:408`; `damage_total.rs:820` | `grep -n 'ActiveState::EquippedActive' src/rules_core/equipment_effects.rs \| head -1`; `grep -n 'ActiveState::Absent' src/rules_core/encumbrance.rs \| head -1`; `grep -n 'ActiveState::EquippedActive' src/rules_core/damage_total.rs \| head -2` |
| F-11 | The market's requested state; drag handlers in the panel | `'EquippedActive'` at `CharacterSheet.tsx:3295`; **0** | `grep -n "activeState: 'EquippedActive'" apps/desktop/src/characterHub/CharacterSheet.tsx`; `grep -c 'draggable\|onDrop\|onDragStart' apps/desktop/src/characterHub/EquipmentPanel.tsx` |
| F-12 | Custom item fields: weight or base-item mentions | **0** (`slot` :108, `worn` :110, `left_behind` :113 exist) | `grep -n 'weight\|base_item' apps/desktop/src-tauri/src/character_custom.rs \| awk 'END{print NR}'` |
| F-13 | Lines: `character_hub.rs`, `character_custom.rs`, `equipment_effects.rs`, `encumbrance.rs`, `body_slots.rs`, `damage_total.rs`, `equipment_record.rs`, `print/blocks/gear.rs` | **21,905; 837; 865; 687; 906; 1,259; 224; 188** | `wc -l <the eight paths>` (`census.txt §2`) |

## §3 Containers in the data

| F | Figure | Value | Command |
|---|---|---|---|
| F-14 | Sheet-rule equipment records tagged `Container` (container-ness); with a `CONTAINS` token (none) | **232**; **0** (one non-equipment file matches the word) | `grep -rl '"Container"' data/sheet_rules --include=*.json \| grep '/equipment/' \| awk 'END{print NR}'`; `grep -rl 'CONTAINS' data/sheet_rules --include=*.json \| awk 'END{print NR}'` → 1 |
| F-15 | Corpus equipment files; records with a `CONTAINS` token; by capacity kind (weight / unlimited / reduction-or-other) | **3,806**; **138**; **70 / 43 / 25** | `python3 -I docs/release/SD-41-gear-and-equipment-sets/artifacts/epic_0/census.py \| grep 'records with a CONTAINS token'` |
| F-16 | Corpus equipment files whose text holds `CONTAINS` anywhere (the wider grep; the difference from F-15 is text, not tokens) | **210** | `grep -rl 'CONTAINS' data/corpus --include=*.json \| grep '/equipment/' \| awk 'END{print NR}'` |
| F-17 | Code reading a container or capacity today | **0** | `grep -rn 'CONTAINS\|is_container\|capacity_lbs' src/rules_core/equipment_record.rs src/rules_core/encumbrance.rs apps/desktop/src-tauri/src/character_hub.rs \| awk 'END{print NR}'` |
| F-18 | PCGen core-rulebook `CONTAINS:` lines (the oracle's own count, a cross-check of F-15's shape; pinned `7f818006e371188e5717fd18d74d18a420747fc6`) | general 39, arms & armor 12, magic items 13 | `set -a; . scripts/pcgen-oracle-pin.env; set +a; PD=${PCGEN_CORPUS_ROOT:-$HOME/workspace/repos/pcgen/data}; for f in cr_equip_general cr_equip_arms_armor cr_equip_magic_items; do printf '%s ' $f; grep -c 'CONTAINS:' "$PD/pathfinder/paizo/roleplaying_game/core_rulebook/$f.lst"; done` |

## §4 Print, smoke, tests

| F | Figure | Value | Command |
|---|---|---|---|
| F-19 | ui-smoke rows; equipment/market/custom rows; step ops (no drag) | **174**; **31**; click key type select scroll wait nativeFile | `python3 -I -c "import json,collections;rows=json.load(open('apps/desktop/scripts/ui-smoke/spec.json'))['rows'];print(len(rows), sum(1 for r in rows if any(k in r['id'] for k in ('equip','gear','market','inventory','custom-item'))), dict(collections.Counter(s['op'] for r in rows for s in r.get('steps',[]))))"` |
| F-20 | SD-40 open cards owning each gear file (the partition) | see `census.txt §7` (e.g. `character_hub.rs`: E2.27 E2.28 E2.29 E3.6; `encumbrance.rs`: none) | `awk -F'\|' -v f=<file> '$2 ~ /^ (E2\.2[3-9]\|E2\.MC3\|E2\.G\|E3\|E4\|E5)[0-9A-Za-z.]* *$/ && $0 ~ f {print $2}' docs/release/SD-40-pf-sheet-design/epic-breakdown.md` |
| F-21 | SD-40 cards complete / total at authoring | **32 / 56** | `awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/ && $5 ~ /^ complete *$/' docs/release/SD-40-pf-sheet-design/kanban.md \| awk 'END{print NR}'`; `awk -F'\|' '$2 ~ /^ (C\|E)[0-9]/' docs/release/SD-40-pf-sheet-design/kanban.md \| awk 'END{print NR}'` |
| F-22 | Rust files mentioning equip/carried/encumbrance under `src/rules_core` + desktop `tests/`; `print_pf_blocks_` test names | **153**; **40** | `grep -rlE 'equip\|carried\|encumbr' apps/desktop/src-tauri/tests src/rules_core --include=*.rs \| awk 'END{print NR}'`; `grep -c 'print_pf_blocks_' apps/desktop/src-tauri/src/print/blocks/print_pf_blocks.rs` |
| F-23 | The print gear block's functions and state words | `weapons` :72, `armor` :108, `inventory` :139; 6 state-word lines | `grep -n 'pub fn inventory\|pub fn weapons\|pub fn armor' apps/desktop/src-tauri/src/print/blocks/gear.rs`; `grep -c 'Left behind\|Carried\|Worn' apps/desktop/src-tauri/src/print/blocks/gear.rs` |
| F-24 | `.pcg` import fixtures the population tests cover | **(E1.2 states: N of N)**; SD-40's E1.MC counted 37 saved of 65 files | `crates/codex-ingest/src/pcg_import/fixtures_tests.rs::fixtures()`; the receipt quotes the test's own line |
