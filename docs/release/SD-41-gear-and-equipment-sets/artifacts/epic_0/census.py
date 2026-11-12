#!/usr/bin/env python3
"""SD-41 census of today's gear model. Run from the repo root: python3 -I docs/release/SD-41-gear-and-equipment-sets/artifacts/epic_0/census.py > .../census.txt
Every figure prints with the shell command that re-derives it (counts via awk, never grep -o)."""
import glob, json, re, subprocess, collections, sys, os

ROOT = os.getcwd()
def sh(cmd):
    out = subprocess.run(cmd, shell=True, capture_output=True, text=True, cwd=ROOT).stdout.strip()
    print(f"$ {cmd}\n{out}\n")
    return out

print(f"# SD-41 census — base {sh('git rev-parse --short HEAD')}")

print("## 1. Screen: EquipmentPanel, boundaries, market")
sh("wc -l apps/desktop/src/characterHub/EquipmentPanel.tsx apps/desktop/src/characterHub/MarketDialog.tsx apps/desktop/src/characterHub/CustomDialog.tsx apps/desktop/src/characterHub/customModel.ts apps/desktop/src/characterHub/encumbranceTabModel.ts apps/desktop/src/boundary/listHeldEquipment.ts apps/desktop/src/boundary/setEquipmentActiveState.ts | tail -8")
sh("awk '/^export const WORN_SLOTS/,/^\\];/' apps/desktop/src/characterHub/EquipmentPanel.tsx | grep -c \"slot: '\"")
sh("grep -n \"activeState: 'EquippedActive'\" apps/desktop/src/characterHub/CharacterSheet.tsx")
sh("grep -c 'draggable\\|onDrop\\|onDragStart' apps/desktop/src/characterHub/EquipmentPanel.tsx")
sh("grep -n 'export const SHEET_TABS' apps/desktop/src/characterHub/CharacterSheet.tsx")

print("## 2. Rust: selections, states, storage")
sh("grep -n 'pub enum ActiveState' -A6 src/rules_core/character_input.rs | grep -E 'EquippedActive|Absent|SelectedInactive' | awk 'END{print NR}'")
sh("grep -n 'const ENVELOPE_FILE\\|const CHARACTER_INPUT_FILE' src/saved_character/local_store.rs")
sh("grep -n 'equipment={}:{}' src/saved_character/local_store.rs")
sh("grep -n 'CURRENT_SAVED_CHARACTER_SCHEMA_VERSION' src/saved_character/mod.rs | head -1")
sh("grep -rnF 'EquipmentSelection {' src apps/desktop/src-tauri/src crates --include=*.rs | grep -v 'struct EquipmentSelection' | awk 'END{print NR}'")
sh("grep -n 'pub enum BodySlot' -A19 src/rules_core/body_slots.rs | grep -cE '^[0-9]+-    [A-Z]'")
sh("python3 -I -c \"import json;d=json.load(open('data/equipment_types.json'));print(len(d['slots']), [ (s['body_slot'],s['capacity']) for s in d['slots'] if s['capacity']!=1])\"")
sh("grep -nE '^pub (async )?fn (add_equipment_selection|attach_equipment_modifier|purchase_equipment|list_held_equipment|set_equipment_active_state|remove_equipment_selection)' apps/desktop/src-tauri/src/character_hub.rs")
sh("grep -n 'pub struct CarriedItemDto\\|pub struct EquipmentEffectsDto\\|pub struct EncumbranceDto\\|pub struct HeldEquipmentCopyDto' apps/desktop/src-tauri/src/character_hub.rs")
sh("wc -l apps/desktop/src-tauri/src/character_hub.rs apps/desktop/src-tauri/src/character_custom.rs src/rules_core/equipment_effects.rs src/rules_core/encumbrance.rs src/rules_core/body_slots.rs src/rules_core/damage_total.rs src/rules_core/equipment_record.rs apps/desktop/src-tauri/src/print/blocks/gear.rs | tail -9")

print("## 3. Engine rules today")
sh("grep -n 'ActiveState::EquippedActive' src/rules_core/equipment_effects.rs | head -3")
sh("grep -n 'ActiveState::Absent' src/rules_core/encumbrance.rs | head -2")
sh("grep -n 'ActiveState::EquippedActive' src/rules_core/damage_total.rs | head -2")
sh("grep -n 'weight\\|base_item' apps/desktop/src-tauri/src/character_custom.rs | awk 'END{print NR}'")
sh("grep -n 'pub slot: Option<BodySlot>\\|pub worn: bool\\|pub left_behind: bool' apps/desktop/src-tauri/src/character_custom.rs")

print("## 4. Containers in the data")
files = glob.glob('data/corpus/**/equipment/*.json', recursive=True)
n = 0; kinds = collections.Counter(); names = []
for f in files:
    try: d = json.load(open(f))
    except Exception: continue
    data = d.get('data', d)
    for t in data.get('raw_tokens') or []:
        if t.get('key') == 'CONTAINS':
            n += 1; cap = t['value'].split('|')[0]
            kinds['weight_lb' if re.match(r'^[0-9.]+$', cap) else ('UNLIM' if cap.startswith('UNLIM') else 'reduction_or_other')] += 1
            names.append(data.get('name')); break
print(f"$ python3 -I <this script> §4: data/corpus equipment files {len(files)}; records with a CONTAINS token {n}; by capacity kind {dict(kinds)}")
print("  examples:", names[:12], "\n")
sh("grep -rl 'CONTAINS' data/corpus --include=*.json | grep '/equipment/' | awk 'END{print NR}'")
sh("grep -rl '\"Container\"' data/sheet_rules --include=*.json | grep '/equipment/' | awk 'END{print NR}'")
sh("grep -rl 'CONTAINS' data/sheet_rules --include=*.json | awk 'END{print NR}'")
sh("grep -rn 'CONTAINS\\|is_container\\|capacity_lbs' src/rules_core/equipment_record.rs src/rules_core/encumbrance.rs apps/desktop/src-tauri/src/character_hub.rs | awk 'END{print NR}'")

print("## 5. Print and smoke")
sh("grep -n 'pub fn inventory\\|pub fn weapons\\|pub fn armor' apps/desktop/src-tauri/src/print/blocks/gear.rs")
sh("grep -c 'Left behind\\|Carried\\|Worn' apps/desktop/src-tauri/src/print/blocks/gear.rs")
sh("python3 -I -c \"import json,collections;rows=json.load(open('apps/desktop/scripts/ui-smoke/spec.json'))['rows'];print(len(rows), sum(1 for r in rows if any(k in r['id'] for k in ('equip','gear','market','inventory','custom-item'))), dict(collections.Counter(s['op'] for r in rows for s in r.get('steps',[]))))\"")
sh("grep -rlE 'equip|carried|encumbr' apps/desktop/src-tauri/tests src/rules_core --include=*.rs | awk 'END{print NR}'")
sh("grep -c 'print_pf_blocks_' apps/desktop/src-tauri/src/print/blocks/print_pf_blocks.rs")

print("## 6. PCGen (functional reference only; pinned oracle)")
sh("grep PCGEN_ORACLE_SHA= scripts/pcgen-oracle-pin.env")
sh("set -a; . scripts/pcgen-oracle-pin.env; set +a; PD=${PCGEN_CORPUS_ROOT:-$HOME/workspace/repos/pcgen/data}; for f in cr_equip_general cr_equip_arms_armor cr_equip_magic_items; do printf '%s ' $f; grep -c 'CONTAINS:' \"$PD/pathfinder/paizo/roleplaying_game/core_rulebook/$f.lst\"; done")
sh("set -a; . scripts/pcgen-oracle-pin.env; set +a; PD=${PCGEN_CORPUS_ROOT:-$HOME/workspace/repos/pcgen/data}; ls \"$PD/../docs/tabpages/inventory\"")

print("## 7. SD-40 overlap (files SD-40's open cards own)")
EB = 'docs/release/SD-40-pf-sheet-design/epic-breakdown.md'
for f in ['character_hub.rs','CharacterSheet.tsx','main.rs','spec.json','gear.rs','pf1e.rs','model.rs','typst.rs','blocks.typ','print_golden.json','packaged_resources.rs','loadSavedCharacterDetail.ts','encumbrance.rs','equipment_effects.rs','damage_total.rs','character_custom.rs','EquipmentPanel.tsx','MarketDialog.tsx','CustomDialog.tsx','equipment_record.rs','local_store.rs']:
    out = subprocess.run(f"awk -F'|' -v f='{f}' '$2 ~ /^ (E2\\.2[3-9]|E2\\.MC3|E2\\.G|E3|E4|E5)[0-9A-Za-z.]* *$/ && $0 ~ f {{gsub(/ /,\"\",$2); printf \"%s \", $2}}' {EB}", shell=True, capture_output=True, text=True, cwd=ROOT).stdout.strip()
    print(f"  {f:28s} {out or '(none)'}")
print("  command: awk -F'|' -v f=<file> '$2 ~ /^ (E2\\.2[3-9]|E2\\.MC3|E2\\.G|E3|E4|E5)[0-9A-Za-z.]* *$/ && $0 ~ f {print $2}' " + EB + "\n")
sh("awk -F'|' '$2 ~ /^ (C|E)[0-9]/ && $5 ~ /^ complete *$/' docs/release/SD-40-pf-sheet-design/kanban.md | awk 'END{print NR}'; awk -F'|' '$2 ~ /^ (C|E)[0-9]/' docs/release/SD-40-pf-sheet-design/kanban.md | awk 'END{print NR}'")
