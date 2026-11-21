#!/usr/bin/env python3 -I
"""SD-42 census: every figure the package quotes, with the command that produces it.

Run from the repo root: `python3 -I docs/release/SD-42-pcgen-import-and-campaign-folder/artifacts/epic_0/census.py`.
The operator's local `.pcg` is read from `$CODEX_SD42_PCG` (default `/home/ubuntu/sd42-input/Aonghus Macleod.pcg`)
and the PCGen samples from `$PCGEN_REPO_DIR/code/testsuite/PCGfiles` (default `$HOME/workspace/repos/pcgen`);
either absent → that section prints SKIPPED with the reason. Only counts are printed, never a line of the file.
"""
import json, os, pathlib, re, subprocess, sys

ROOT = pathlib.Path.cwd()
SAMPLES = ["pf_Cleric", "pf_Paladin", "pf_Rogue", "pf_goldielocks", "sf_mechanic", "sf_mechanic_drone", "sf_soldier"]

def sh(cmd):
    return subprocess.run(cmd, shell=True, capture_output=True, text=True, cwd=ROOT).stdout.strip()

def pcg_counts(path):
    text = path.read_text(encoding="utf-8", errors="replace").splitlines()
    equip = [l for l in text if l.startswith("EQUIPNAME:")]
    sets = [l for l in text if l.startswith("EQUIPSET:")]
    def qty(line):
        m = re.search(r"\|QUANTITY:([0-9.]+)", line)
        return float(m.group(1)) if m else 1.0
    depth = {}
    for l in sets:
        m = re.search(r"\|ID:([0-9.]+)", l)
        if m:
            d = m.group(1).count(".") + 1
            depth[d] = depth.get(d, 0) + 1
    data_keys = {}
    for l in equip:
        for m in re.finditer(r"DATA:([^\]]+)", l):
            for field in m.group(1).split("$"):
                k = field.split("=", 1)[0]
                data_keys[k] = data_keys.get(k, 0) + 1
    gm = next((l.split(":", 1)[1] for l in text if l.startswith("GAMEMODE:")), "")
    classes = [l.split("|")[0].split(":", 1)[1] for l in text if l.startswith("CLASS:")]
    return {
        "EQUIPNAME": len(equip),
        "EQUIPNAME_with_CUSTOMIZATION": sum(1 for l in equip if "CUSTOMIZATION" in l),
        "EQUIPNAME_quantity_gt_1": sum(1 for l in equip if qty(l) > 1),
        "EQUIPNAME_quantity_0": sum(1 for l in equip if qty(l) == 0),
        "EQUIPNAME_max_quantity": max([qty(l) for l in equip] or [0]),
        "EQUIPSET": len(sets),
        "EQUIPSET_depth_by_id_components": dict(sorted(depth.items())),
        "DATA_field_keys": dict(sorted(data_keys.items())),
        "GAMEMODE": gm,
        "CLASS": classes,
    }

out = {}
# §1 the operator's file (local only)
op = pathlib.Path(os.environ.get("CODEX_SD42_PCG", "/home/ubuntu/sd42-input/Aonghus Macleod.pcg"))
out["operator_file"] = pcg_counts(op) if op.is_file() else f"SKIPPED: {op} absent (operator file is local only, decisions.md §10)"
# §2 the PCGen samples at the pinned oracle
repo = pathlib.Path(os.environ.get("PCGEN_REPO_DIR") or (pathlib.Path.home() / "workspace/repos/pcgen"))
pcgfiles = repo / "code/testsuite/PCGfiles"
if pcgfiles.is_dir():
    out["oracle_sha"] = sh(f"git -C {repo} rev-parse HEAD")
    out["oracle_pin"] = sh("awk -F= '/^PCGEN_ORACLE_SHA=/{print $2}' scripts/pcgen-oracle-pin.env")
    out["samples"] = {name: pcg_counts(pcgfiles / f"{name}.pcg") for name in SAMPLES}
else:
    out["samples"] = f"SKIPPED: {pcgfiles} absent (run scripts/fetch-pcgen-oracle.sh)"
# §3 the importer and the campaign code today
out["lines"] = {f: int(sh(f"awk 'END{{print NR}}' {f}")) for f in [
    "crates/codex-ingest/src/pcg_import/map.rs", "crates/codex-ingest/src/pcg_import/catalog.rs",
    "crates/codex-ingest/src/pcg_import/wire.rs", "crates/codex-ingest/src/pcg_import/parse.rs",
    "crates/codex-ingest/src/pcg_import/fixtures_tests.rs",
    "apps/desktop/src-tauri/src/character_hub.rs", "apps/desktop/src-tauri/src/campaign_drive.rs",
    "apps/desktop/src-tauri/src/main.rs", "src/campaign/mod.rs", "src/campaign/local_store.rs",
    "src/saved_character/campaign_mirror.rs", "src/saved_character/equipment_sets.rs",
    "apps/desktop/src/campaign/CampaignManagerScreen.tsx", "apps/desktop/src/campaign/CampaignSheet.tsx",
    "apps/desktop/src/campaign/CreateCampaignScreen.tsx", "apps/desktop/src/campaign/EditCampaignScreen.tsx",
    "apps/desktop/src/campaign/campaignModel.ts", "apps/desktop/src/campaign/campaignManagerAccessGate.ts",
    "apps/desktop/src/settings/GoogleDrivePanel.tsx", "apps/desktop/src/settings/googleDrive.ts",
    "apps/desktop/src/characterHub/CampaignMembershipDialog.tsx", "apps/desktop/src/characterHub/campaignMembershipModel.ts",
    "apps/desktop/src/boundary/listDriveCampaigns.ts", "apps/desktop/src/boundary/writeCampaignLocalFolderArtifacts.ts",
    "apps/desktop/src/boundary/setCharacterCampaigns.ts", "apps/desktop/src/characterHub/gearScreenModel.ts",
    "apps/desktop/src/characterHub/GearScreen.tsx", "apps/desktop/src/characterHub/LoadCharacterScreen.tsx",
    "apps/desktop/src/boundary/importPcg.ts", "apps/desktop/src/dmToolkit/encounterBuilderModel.ts",
    "apps/desktop/src-tauri/src/encounter_rating.rs",
]}
out["importer"] = {
    "MAX_COPIES": sh("awk -F'= ' '/^const MAX_COPIES/{print $2}' crates/codex-ingest/src/pcg_import/map.rs"),
    "per_copy_row_lines": sh("grep -n 'self.equipment.extend((0..' crates/codex-ingest/src/pcg_import/map.rs | cut -d: -f1 | paste -sd,"),
    "base_item_first_lookup_line": sh("grep -n 'let item_key = \\[base_item.as_deref()' crates/codex-ingest/src/pcg_import/map.rs | cut -d: -f1"),
    "enhancement_only_eqmods_line": sh("grep -n '^fn equipment_modifiers' crates/codex-ingest/src/pcg_import/map.rs | cut -d: -f1"),
    "custom_item_fn_line": sh("grep -n 'fn custom_item' crates/codex-ingest/src/pcg_import/map.rs | cut -d: -f1"),
    "quantity_fn_line": sh("grep -n '^fn quantity' crates/codex-ingest/src/pcg_import/map.rs | cut -d: -f1"),
    "game_mode_warning_line": sh("grep -n 'game_mode != \"Pathfinder_RPG\"' crates/codex-ingest/src/pcg_import/map.rs | cut -d: -f1"),
    "fighter_longsword_literal_line": sh("grep -n 'FIGHTER_BONUS_SELECTION: &str' crates/codex-ingest/src/pcg_import/map.rs | cut -d: -f1"),
    "sf_soldier_refusal_test_line": sh("grep -n 'no Pathfinder engine row can compute a Starfinder soldier' crates/codex-ingest/src/pcg_import/fixtures_tests.rs | cut -d: -f1"),
    "repo_eqm_fixtures": int(sh("ls scripts/oracle_harness/eqm-fixtures/*.pcg | awk 'END{print NR}'")),
    "wire_EquipmentWire_fields": sh("awk '/pub struct EquipmentWire/,/^}/' crates/codex-ingest/src/pcg_import/wire.rs | grep -c 'pub '"),
}
out["campaign"] = {
    "members_length_line": sh("grep -n 'campaign.members.length}' apps/desktop/src/campaign/CampaignManagerScreen.tsx | head -1 | cut -d: -f1"),
    "drive_commands_registered": sh("grep -c 'drive_\\|write_campaign_drive_artifacts' apps/desktop/src-tauri/src/main.rs"),
    "campaign_drive_tauri_commands": sh("grep -c '#\\[tauri::command\\]' apps/desktop/src-tauri/src/campaign_drive.rs"),
    "campaign_smoke_rows": sh("python3 -I -c \"import json;r=json.load(open('apps/desktop/scripts/ui-smoke/spec.json'))['rows'];print(sum(1 for x in r if 'campaign' in x['id']),'of',len(r))\""),
    "campaign_smoke_row_ids": sh("python3 -I -c \"import json;r=json.load(open('apps/desktop/scripts/ui-smoke/spec.json'))['rows'];print(','.join(x['id'] for x in r if 'campaign' in x['id']))\""),
    "ts_files_naming_drive": int(sh("grep -rliE 'googleDrive|driveFolderPath|drive_list_campaigns|drive_save_campaign|write_campaign_drive' apps/desktop/src --include=*.ts --include=*.tsx | awk 'END{print NR}'")),
    "mirror_target_literal": sh("grep -n 'join(\"characters\")' src/saved_character/campaign_mirror.rs | cut -d: -f1"),
    "markdown_dependency_in_package_json": sh("grep -cE 'codemirror|marked|markdown' apps/desktop/package.json"),
    "market_buy_merges_into_stack_line": sh("grep -n 'quantity = Some(row.quantity.saturating_add(pieces))' apps/desktop/src-tauri/src/character_hub.rs | cut -d: -f1"),
    "tauri_capabilities": sh("python3 -I -c \"import json;print(json.load(open('apps/desktop/src-tauri/capabilities/default.json'))['permissions'])\""),
}
out["version_surfaces_sd41_c1"] = int(sh("git show --name-only --format= c817ad502f | awk 'NF' | grep -v '^docs/' | awk 'END{print NR}'"))
out["sd41_open_cards"] = sh("awk -F'|' '$2 ~ /^ (C|E)[0-9]/ && $5 !~ /^ complete *$/ {gsub(/ /,\"\",$2); printf \"%s \", $2}' docs/release/SD-41-gear-and-equipment-sets/kanban.md").split()
out["sd41_cards_total"] = int(sh("awk -F'|' '$2 ~ /^ (C|E)[0-9]/' docs/release/SD-41-gear-and-equipment-sets/kanban.md | awk 'END{print NR}'"))
out["base"] = {"branch": sh("git rev-parse --abbrev-ref HEAD"), "head": sh("git rev-parse --short=10 HEAD"),
               "merge_base_tranche_21": sh("git merge-base HEAD origin/tranche-21 | cut -c1-10")}
print(json.dumps(out, indent=2, default=str))
