//! The Starfinder drone in the converted package (SD-37 E5.4, `epic-breakdown.md` E5.4: a
//! Mechanic 1 render prints the drone block).
//!
//! The oracle builds the drone from four places (pinned oracle, `starfinder/paizo/core`):
//!
//! - the mechanic's AI selection `Drone` (`scr_abilities.lst:1369`) raises the master's
//!   `DroneCompanionLVL` and offers a follower of role `Drone` (`COMPANIONLIST:Drone|Drone`);
//! - the companion modifier `FOLLOWER:DroneCompanionLVL=1 TYPE:Drone` (`scr_companionmods.lst:13`)
//!   applies to that follower: the level-1 special abilities (through the internal row
//!   `Drone Special Abilities`, :1398), the chassis / skill unit / feat / mod picks, and the
//!   drone's level variables from the master's (`MASTERVAR("DroneCompanionLVL")`);
//! - the drone race (`scr_races.lst:20`, `MONSTERCLASS:Drone:1`) and its class `Drone`
//!   (`scr_classes.lst:201`, base attack and saves over `DroneLVL`);
//! - each chassis (`scr_abilities.lst:1393-1395`) and its starting kit (`scr_kits.lst:6-13`,
//!   the chassis' base ability scores).
//!
//! These tests read the generated package (`data/starfinder-1e/sheet_rules`, kept fresh by
//! `sheet_rule_convert --system starfinder-1e --check`) as JSON, so they hold the converter's
//! output, not the engine's reading of it.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde_json::Value;

fn package_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/starfinder-1e/sheet_rules")
}

/// Every rule of the package file holding `id` (`<book>:<kind>:<slug>`).
fn rules_of(id: &str) -> Vec<Value> {
    let mut parts = id.splitn(3, ':');
    let (book, kind, slug) = (parts.next().unwrap(), parts.next().unwrap(), parts.next().unwrap());
    let path = package_dir().join(book).join(kind).join(format!("{slug}.json"));
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    serde_json::from_str::<Value>(&text).expect("package file parses").as_array().expect("an array of rules").clone()
}

fn principal(id: &str) -> Value {
    rules_of(id).into_iter().find(|r| r["id"] == id).unwrap_or_else(|| panic!("{id}: no principal rule"))
}

/// The ids of every principal rule under `<book>/<kind>/` whose id starts with `prefix`.
fn ids_with_prefix(book: &str, kind: &str, prefix: &str) -> BTreeSet<String> {
    let dir = package_dir().join(book).join(kind);
    let mut out = BTreeSet::new();
    for entry in std::fs::read_dir(&dir).unwrap_or_else(|e| panic!("{}: {e}", dir.display())) {
        let path = entry.expect("dir entry").path();
        let rules: Value = serde_json::from_str(&std::fs::read_to_string(&path).expect("reads")).expect("parses");
        for r in rules.as_array().expect("array") {
            let id = r["id"].as_str().expect("id");
            if !id.contains('#') && id.starts_with(prefix) {
                out.insert(id.to_string());
            }
        }
    }
    out
}

fn granted_by_rule(rule: &Value) -> Vec<String> {
    rule["granted_by"]
        .as_array()
        .map(|g| g.iter().filter_map(|g| g["by"]["Rule"].as_str().map(str::to_string)).collect())
        .unwrap_or_default()
}

const MODIFIER: &str = "core:companion_mod:drone";

/// The companion modifier converts as one rule, held through the drone race, under the oracle's
/// own gate: the master's `DroneCompanionLVL` is at least 1.
#[test]
fn the_drone_companion_modifier_is_held_through_the_drone_race_under_the_masters_level() {
    let rule = principal(MODIFIER);
    assert_eq!(granted_by_rule(&rule), vec!["core:race:drone".to_string()], "{rule:#}");
    let applies = rule["applies"].to_string();
    assert!(applies.contains("MasterVar") && applies.contains("Gte"), "gate on the master's DroneCompanionLVL: {applies}");
    let cites = rule["provenance"]["closure_rows"].to_string();
    assert!(cites.contains("scr_companionmods.lst:13"), "{cites}");
    // Its level variables come from the master (`MASTERVAR("DroneCompanionLVL")`).
    let all = serde_json::to_string(&rules_of(MODIFIER)).unwrap();
    assert!(all.contains("MasterVar"), "DroneMasterLVL reads the master's variable");
}

/// The modifier's picks offer the oracle's members: 3 chassis, 6 skill units, 33 drone feats
/// (the `CATEGORY:Internal` rows of each pick's category, `scr_abilities.lst:1393-1449`;
/// counted by `E5.4_drone_members.py`), and the drone mods.
#[test]
fn the_drone_picks_offer_the_oracle_members() {
    let chassis = ids_with_prefix("core", "pool_option", "core:pool_option:drone_chassis_selection_");
    assert_eq!(
        chassis,
        ["combat", "hover", "stealth"].iter().map(|s| format!("core:pool_option:drone_chassis_selection_{s}")).collect(),
    );
    let skill_units = ids_with_prefix("core", "pool_option", "core:pool_option:drone_skill_unit_");
    assert_eq!(skill_units.len(), 6, "{skill_units:?}");
    let feats = ids_with_prefix("core", "pool_option", "core:pool_option:drone_feat_");
    assert_eq!(feats.len(), 33, "{feats:?}");
    // Every pick on the modifier offers a pool (no pick left offering nothing).
    let picks: Vec<Value> = rules_of(MODIFIER).into_iter().filter(|r| r["target"]["Pool"].is_string()).collect();
    assert_eq!(picks.len(), 4, "chassis, skill unit, feat, mod picks: {picks:#?}");
    for pick in &picks {
        assert!(pick["offers"].is_object(), "{} offers nothing", pick["id"]);
    }
}

/// The drone race holds its class (`MONSTERCLASS:Drone:1`), and prints no `Walk 0 ft.`: its
/// speed is the chassis' (`BONUS:VAR|Walk|30`).
#[test]
fn the_drone_race_holds_the_drone_class_and_prints_no_zero_speed() {
    let class = principal("core:class:drone");
    assert!(granted_by_rule(&class).contains(&"core:race:drone".to_string()), "{class:#}");
    let race = serde_json::to_string(&rules_of("core:race:drone")).unwrap();
    assert!(!race.contains("Walk 0 ft."), "{race}");
    assert!(!race.contains("\"Const\":0},{\"Text\":\" ft.\"}"), "{race}");
}

/// The special abilities reach the drone through the internal row `Drone Special Abilities`
/// (`ABILITY:Class Feature|AUTOMATIC|TYPE=Drone Special Ability LVL 1|PREVARGTEQ:DroneMasterLVL,1`).
#[test]
fn the_level_one_special_abilities_are_granted_by_the_modifier() {
    for id in ["core:ability:basic_mods", "core:ability:limited_ai", "core:ability:master_control", "core:ability:skill_unit"] {
        let rule = principal(id);
        let edge = rule["granted_by"]
            .as_array()
            .and_then(|g| g.iter().find(|g| g["by"]["Rule"] == MODIFIER))
            .unwrap_or_else(|| panic!("{id}: not granted by {MODIFIER}: {rule:#}"));
        assert!(edge["when"].to_string().contains("Gte"), "{id}: gated on DroneMasterLVL >= 1: {edge}");
    }
}

/// Each chassis prints its starting kit's base ability scores (`STARTPACK:Drone ~ Hover`,
/// `STAT:STR=6|DEX=16|INT=6|WIS=8|CHA=6`).
#[test]
fn each_chassis_prints_its_kits_base_ability_scores() {
    for (chassis, scores) in [
        ("combat", "Str 14, Dex 12, Int 6, Wis 10, Cha 6"),
        ("hover", "Str 6, Dex 16, Int 6, Wis 8, Cha 6"),
        ("stealth", "Str 12, Dex 14, Int 6, Wis 10, Cha 6"),
    ] {
        let rules = serde_json::to_string(&rules_of(&format!("core:pool_option:drone_chassis_selection_{chassis}"))).unwrap();
        assert!(rules.contains("Base ability scores") && rules.contains(scores), "{chassis}: {rules}");
    }
}

/// A Starfinder `ABILITY:<cat>|AUTOMATIC|<key>` grant holds its target whatever the target's own
/// prerequisites say (`decisions.md §21(c)`). The SRD hover chassis lists "flight system (x2,
/// included in its speed)" among its initial mods, "a part of the chassis itself"
/// (https://www.aonsrd.com/DroneChassis.aspx?ItemName=All, Hover Drone, Core Rulebook p. 75); the
/// pinned oracle agrees: a hover drone whose master is not loaded (DroneMasterTotalLVL 0, character
/// level 1, so `PREMULT:1,[PREPCLEVEL:MIN=11],[PREVARGTEQ:DroneMasterTotalLVL,11]` fails) still
/// counts `DroneModFlightSystemTaken` = 2 (`artifacts/epic_5/E5.4-oracle/hover_drone_alone.oracle.txt`).
/// So Flight System's gate is "the hover chassis is held, or its own prerequisite": a pick into the
/// Drone Mod pool still needs level 11.
#[test]
fn the_hover_chassis_grant_satisfies_flight_systems_own_prerequisite() {
    let rule = principal("core:ability:drone_mod_flight_system");
    let applies = &rule["applies"];
    assert_eq!(applies["AtLeast"]["n"], 1, "{applies:#}");
    let of = applies["AtLeast"]["of"].as_array().unwrap_or_else(|| panic!("{applies:#}"));
    assert!(
        of.iter().any(|a| a["Holds"]["what"]["Rule"] == "core:pool_option:drone_chassis_selection_hover"),
        "the chassis' automatic grant waives the prerequisite: {applies:#}"
    );
    let own = serde_json::to_string(of).unwrap();
    assert!(own.contains("\"Level\"") && own.contains("{\"Const\":11}"), "the mod's own level-11 prerequisite is kept for a pick: {own}");
}

/// The drone's Hit Points (`decisions.md §21(b)`): the drone class's
/// `BONUS:HP|CURRENTMAX|(10*DroneLVL)+if(DroneLVL>=18,10,0)+...` feeds hit points (SRD drone base
/// statistics: 10 per level, 190 / 210 / 230 at 18-20, https://www.aonsrd.com/Classes.aspx?ItemName=Drone;
/// PCGen party run `oracle-builds/sf_mechanic_drone.oracle.txt`: DroneLVL 10, hp 100), and its
/// `BONUS:HP|CURRENTMAX|-1` cancels the one `HD:1` drone class level, so it prints no line.
#[test]
fn the_drone_class_prints_its_hit_points_and_no_hit_die_offset() {
    let rules = rules_of("core:class:drone");
    let hp: Vec<&Value> = rules.iter().filter(|r| r["target"] == "Hp").collect();
    assert_eq!(hp.len(), 1, "one hit-point line: {:#}", serde_json::to_value(&hp).unwrap());
    let value = hp[0]["value"].to_string();
    // `10*DroneLVL` plus one step of 10 at each of 18, 19 and 20 (`if(DroneLVL>=n,10,0)` lowers
    // to `min(1, max(0, DroneLVL - n + 1)) * 10`).
    for part in ["{\"Mul\":[{\"Const\":10},{\"Var\":", "{\"Const\":-18}", "{\"Const\":-19}", "{\"Const\":-20}"] {
        assert!(value.contains(part), "{part} in {value}");
    }
    assert!(!serde_json::to_string(&rules).unwrap().contains("{\"Const\":-1}"), "the -1 offset prints no line");
}
