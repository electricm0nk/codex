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

print("## 8. The seven follow-ups (2026-11-12)")
sh("grep -n -i unarmed src/rules_core/damage_total.rs src/rules_core/pilot_compute/combat.rs")
sh("grep -n 'fn ground_unchained_monk_unarmed_strike_damage' src/rules_core/pilot_compute/class_monk.rs")
sh("grep -rl 'Improved Unarmed Strike' data/sheet_rules --include=*.json | awk 'END{print NR}'")
sh("python3 -I -c \"import json;d=json.load(open('data/corpus/core_rulebook/equipment/arms_armor/unarmed_strike.json'))['data'];print([t for t in d['raw_tokens'] if t['key'] in ('DAMAGE','CRITMULT','COST','WT')])\"")
tags=collections.Counter()
for f in glob.glob('data/sheet_rules/**/equipment/*.json', recursive=True):
    try: recs=json.load(open(f))
    except Exception: continue
    for r in (recs if isinstance(recs,list) else [recs]):
        for k in ('Projectile','Ammunition','Thrown','Bow','Crossbow','Sling','Firearm'):
            if k in set(r.get('tags',[])): tags[k]+=1
print("$ python3 -I <this script> §8 ammunition tags over data/sheet_rules/**/equipment/*.json:", dict(tags), "\n")
w=0
for f in glob.glob('data/corpus/**/equipment/**/*.json', recursive=True):
    try: x=json.load(open(f))['data']
    except Exception: continue
    toks={t['key']:t['value'] for t in x.get('raw_tokens',[]) if 'key' in t}
    if 'CONTAINS' in toks and 'Weapon' in toks.get('TYPE',''): w+=1
print("$ python3 -I <this script> §8 corpus weapon records with a CONTAINS ammunition family:", w, "\n")
sh("grep -n '\"Ammunition\"' apps/desktop/src-tauri/resources/print/looks/weapons.typ")
sh("grep -c -i 'ammunition\\|ammo' apps/desktop/src/characterHub/CharacterSheet.tsx")
sh("wc -l apps/desktop/src/characterHub/MarketDialog.tsx; grep -c \"value: '\" apps/desktop/src/characterHub/priceMode.ts; grep -n 'export type MarketCategory' apps/desktop/src/boundary/listMarket.ts")
sh("grep -n 'pub(crate) fn refund_copper\\|pub(crate) fn sell_held_copy_at_root' apps/desktop/src-tauri/src/character_hub.rs")
sh("grep -n 'Drop' apps/desktop/src/characterHub/CharacterSheet.tsx | head -6")
sh("python3 -I -c \"import json;rows=json.load(open('apps/desktop/scripts/ui-smoke/spec.json'))['rows'];print([r['id'] for r in rows if 'drop' in json.dumps(r).lower()])\"")
kinds=collections.Counter(); bybook=collections.defaultdict(collections.Counter); bycat=collections.defaultdict(collections.Counter); neg=[]
for f in glob.glob('data/corpus/**/equipment/**/*.json', recursive=True):
    try: d=json.load(open(f)); x=d.get('data',d)
    except Exception: continue
    c=x.get('cost_gp'); k='none' if c is None else ('zero' if c==0 else ('negative' if c<0 else 'positive'))
    kinds[k]+=1; bybook[k][f.split('/')[2]]+=1; bycat[k][x.get('category')]+=1
    if k=='negative': neg.append((x.get('name'),c))
print("$ python3 -I <this script> §8 price census over data/corpus/**/equipment/**/*.json (a proxy for the Market; E1.8 derives the Market's own count):", dict(kinds))
for k in ('none','zero','negative'): print("  ",k,"by book",dict(bybook[k].most_common(8)),"by category",dict(bycat[k]))
print("   negatives:", neg, "\n")
sh("python3 -I -c \"import json;d=json.load(open('data/corpus/core_rulebook/equipment/arms_armor/adamantine_breastplate.json'));x=d['data'];print(x.get('name'),x.get('cost_gp'),len(x.get('raw_tokens',[])))\"")
sh("python3 -I -c \"import json;d=json.load(open('data/corpus/ultimate_psionics/equipment/astral_armor.json'));x=d['data'];print([t for t in x['raw_tokens'] if t['key'] in ('COST','EQMOD')])\"")
n=0; costk=collections.Counter(); mat=0
for f in [f for f in glob.glob('data/corpus/**/*.json', recursive=True) if 'equipmod' in f]:
    try: d=json.load(open(f)); x=d.get('data',d)
    except Exception: continue
    n+=1; toks={t['key']:t['value'] for t in x.get('raw_tokens',[]) if 'key' in t}; c=toks.get('COST','')
    costk['formula' if re.search(r'[A-Z(]',c) else ('none' if c=='' else 'flat')]+=1
    if 'Material' in toks.get('TYPE',''): mat+=1
print("$ python3 -I <this script> §8 equipmod records:", n, "COST kinds", dict(costk), "Material-typed", mat, "\n")
sh("grep -rl '\"Resizable\"' data/sheet_rules --include=*.json | grep '/equipment/' | awk 'END{print NR}'")
b=0;q=0
for f in glob.glob('data/corpus/**/equipment/**/*.json', recursive=True):
    try: x=json.load(open(f))['data']
    except Exception: continue
    toks={t['key']:t['value'] for t in x.get('raw_tokens',[]) if 'key' in t}
    if re.search(r'\((\d+)\)', x.get('name') or '') or 'BASEQTY' in toks: b+=1; q+= 'BASEQTY' in toks
print("$ python3 -I <this script> §8 bundle-shaped records:", b, "with BASEQTY", q, "\n")
sh("grep -n 'record.slot.is_some() || !record.bonuses.is_empty()' apps/desktop/src-tauri/src/character_hub.rs")
sh("grep -rn 'compute_encumbrance' src apps/desktop/src-tauri/src crates --include=*.rs | awk 'END{print NR}'")
sh("awk -F'|' '$2 ~ /^ (E2\\.2[3-9]|E2\\.MC3|E2\\.G|E3|E4|E5)[0-9A-Za-z.]* *$/ && $4 ~ /data\\/|codex-ingest|rules_catalog/ {print $2}' docs/release/SD-40-pf-sheet-design/epic-breakdown.md | awk 'END{print NR}'")

print("## 9. The campaign item (2026-11-12)")
sh("grep -n 'fn campaign_dir_for' apps/desktop/src-tauri/src/campaign_drive.rs; grep -n 'pub fn save' src/campaign/local_store.rs | head -2")
sh("grep -rn 'driveFolderPath: string' apps/desktop/src/settings/googleDrive.ts; grep -rn 'drive_folder_path' apps/desktop/src-tauri/src/*.rs | grep -v 'test\\|campaign_drive.rs' | awk 'END{print NR}'")
sh("grep -rn 'partyCharacterIds:' apps/desktop/src/campaign/EditCampaignScreen.tsx")
sh("grep -n \"setMode('campaign-list')\" apps/desktop/src/characterHub/CharacterHubPage.tsx; grep -n 'onLoad: (campaignId' apps/desktop/src/campaign/CampaignManagerScreen.tsx")
sh("grep -n '>Campaign<' apps/desktop/src/characterHub/CharacterSheet.tsx")
sh("grep -rn 'SavedCharacterStore::save(' apps/desktop/src-tauri/src src --include=*.rs | grep -v test | awk 'END{print NR}'")
sh("for f in custom.json hit_points.json portrait money.json bio.json; do printf '%s ' $f; grep -rn \"$f\" apps/desktop/src-tauri/src/*.rs | grep -c 'write\\|atomic\\|save'; done")
sh("grep -n 'no Drive folder configured' apps/desktop/src-tauri/src/campaign_drive.rs")
sh("python3 -I -c \"import json;rows=json.load(open('apps/desktop/scripts/ui-smoke/spec.json'))['rows'];print(len([r for r in rows if 'campaign' in r['id']]))\"")
sh("awk -F'|' '$2 ~ /^ (E2\\.2[3-9]|E2\\.MC3|E2\\.G|E3|E4|E5)[0-9A-Za-z.]* *$/ && $4 ~ /campaign|local_store|CharacterHubPage|LandingScreen/ {print $2}' docs/release/SD-40-pf-sheet-design/epic-breakdown.md | awk 'END{print NR}'")
