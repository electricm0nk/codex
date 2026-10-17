//! Cyberpunk RED catalog and deck-calculator commands (SD-38 E6.1).
//!
//! - [`list_cyberpunk_catalog`] returns every record of one MANIFEST kind from the Cyberpunk RED
//!   corpus root (`data/cyberpunk-red/corpus`, packaged-root aware through
//!   [`codex::rules_core::game_system::runtime_repo_root`]) and refuses a kind the MANIFEST does
//!   not list.
//! - [`compute_cyberpunk_deck`] looks a deck, its Programs and its Hardware up in the corpus and
//!   answers with the engine's slot rows (`codex::rules_core::cpr::netrunning`), refusing a loadout
//!   over the deck's slots (A-7).
//!
//! Nothing here counts or sums a slot: the numbers are the engine's `cpr.netrunning.*` rows.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use codex::rules_core::cpr::netrunning::{
    compute_deck, net_actions_row, CprDeckExtras, CprDeckItem, CprDeckQuality,
};
use codex::rules_core::game_system::{runtime_repo_root, GameSystem};

/// One MANIFEST kind's file and its pinned record count.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ManifestKind {
    file: String,
    expected: usize,
    print_only: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprCatalogResponse {
    pub kind: String,
    /// The corpus root the records were read from (repo-relative).
    pub source: String,
    /// The MANIFEST's pinned record count for the kind.
    pub expected: usize,
    /// A `print_only` kind is a GM reference page, not a player-facing catalog.
    pub print_only: bool,
    pub records: Vec<Value>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprDeckExtrasDto {
    #[serde(default)]
    pub bodyweight_suit: bool,
    #[serde(default)]
    pub cyberarm_deck: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprDeckRowDto {
    pub id: String,
    pub value: i16,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CprDeckResponse {
    pub deck: String,
    pub slots_available: u8,
    pub slots_used: u8,
    pub bodyweight_suit_hardware: Option<String>,
    /// NET Actions per Turn row, present when an Interface rank was asked for.
    pub net_actions: Option<CprDeckRowDto>,
    pub rows: Vec<CprDeckRowDto>,
}

fn corpus_root() -> PathBuf {
    GameSystem::CyberpunkRed.package_roots(&runtime_repo_root()).corpus
}

fn read_records(root: &Path, file: &str) -> Result<Vec<Value>, String> {
    let path = root.join(file);
    let text = std::fs::read_to_string(&path).map_err(|e| format!("cpr_catalog.read_failed: {}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("cpr_catalog.parse_failed: {}: {e}", path.display()))
}

fn manifest_kind(root: &Path, kind: &str) -> Result<ManifestKind, String> {
    let path = root.join("MANIFEST.json");
    let text = std::fs::read_to_string(&path).map_err(|e| format!("cpr_catalog.read_failed: {}: {e}", path.display()))?;
    let manifest: Value =
        serde_json::from_str(&text).map_err(|e| format!("cpr_catalog.parse_failed: {}: {e}", path.display()))?;
    let kinds = manifest["kinds"].as_array().ok_or("cpr_catalog.manifest_malformed: no kinds array")?;
    let entry = kinds
        .iter()
        .find(|k| k["kind"].as_str() == Some(kind))
        .ok_or_else(|| format!("cpr_catalog.unknown_kind: unknown Cyberpunk RED catalog kind '{kind}'"))?;
    Ok(ManifestKind {
        file: entry["file"].as_str().ok_or("cpr_catalog.manifest_malformed: kind without file")?.to_owned(),
        expected: entry["expected"].as_u64().ok_or("cpr_catalog.manifest_malformed: kind without expected")? as usize,
        print_only: entry["print_only"].as_bool().unwrap_or(false),
    })
}

/// Every record of `kind` from the corpus at `root`; a kind the MANIFEST does not list is refused.
fn catalog_at(root: &Path, kind: &str) -> Result<CprCatalogResponse, String> {
    let entry = manifest_kind(root, kind)?;
    Ok(CprCatalogResponse {
        kind: kind.to_owned(),
        source: GameSystem::CyberpunkRed.corpus_relative().to_owned(),
        expected: entry.expected,
        print_only: entry.print_only,
        records: read_records(root, &entry.file)?,
    })
}

fn row_dto(row: &codex::rules_core::pilot_compute::ComputationExplanation) -> CprDeckRowDto {
    CprDeckRowDto { id: row.id.clone(), value: row.value, detail: row.detail.clone() }
}

fn record_named<'a>(records: &'a [Value], kind: &str, name: &str) -> Result<&'a Value, String> {
    records
        .iter()
        .find(|r| r["name"].as_str() == Some(name))
        .ok_or_else(|| format!("cpr_catalog.unknown_item: no {kind} named '{name}' in the Cyberpunk RED corpus"))
}

/// The deck calculator over the corpus at `root`: names resolve to corpus records, slots and the
/// refusal come from the engine.
fn deck_at(
    root: &Path,
    deck: &str,
    programs: &[String],
    hardware: &[String],
    extras: &CprDeckExtrasDto,
    interface_rank: Option<u8>,
) -> Result<CprDeckResponse, String> {
    let quality = CprDeckQuality::from_name(deck)
        .ok_or_else(|| format!("cpr_catalog.unknown_item: no Cyberdeck named '{deck}'"))?;
    let deck_record = record_named(&read_records(root, "cyberdeck.json")?, "Cyberdeck", deck)?.clone();
    if deck_record["slots"].as_u64() != Some(u64::from(quality.base_slots())) {
        return Err(format!("cpr_catalog.corpus_mismatch: corpus slots for '{deck}' differ from the engine's"));
    }
    let program_records = read_records(root, "program.json")?;
    let hardware_records = read_records(root, "net_hardware.json")?;

    let mut items = Vec::new();
    for name in programs {
        let record = record_named(&program_records, "Program", name)?;
        items.push(if record["program_class"].as_str() == Some("Black ICE") {
            CprDeckItem::black_ice(name)
        } else {
            CprDeckItem::program(name)
        });
    }
    for name in hardware {
        let record = record_named(&hardware_records, "Hardware", name)?;
        let slots = record["slots"].as_u64().ok_or_else(|| {
            format!("cpr_catalog.not_hardware: '{name}' is an option, not Hardware; set it in the deck extras")
        })?;
        items.push(CprDeckItem::hardware(name, u8::try_from(slots).unwrap_or(u8::MAX)));
    }

    let load = compute_deck(
        quality,
        CprDeckExtras { bodyweight_suit: extras.bodyweight_suit, cyberarm_deck: extras.cyberarm_deck },
        &items,
    )
    .map_err(|e| e.to_string())?;
    let net_actions = match interface_rank {
        Some(rank) => Some(row_dto(&net_actions_row(rank).map_err(|e| e.to_string())?)),
        None => None,
    };
    Ok(CprDeckResponse {
        deck: deck.to_owned(),
        slots_available: load.slots_available,
        slots_used: load.slots_used,
        bodyweight_suit_hardware: load.bodyweight_suit_hardware,
        net_actions,
        rows: load.rows.iter().map(row_dto).collect(),
    })
}

/// One Cyberpunk RED catalog: every record of a MANIFEST kind.
#[tauri::command]
pub fn list_cyberpunk_catalog(kind: String) -> Result<CprCatalogResponse, String> {
    catalog_at(&corpus_root(), &kind)
}

/// The Netrunner deck calculator: slots used and available, or the engine's over-capacity refusal.
#[tauri::command]
#[allow(clippy::needless_pass_by_value)]
pub fn compute_cyberpunk_deck(
    deck: String,
    programs: Vec<String>,
    hardware: Vec<String>,
    extras: CprDeckExtrasDto,
    interface_rank: Option<u8>,
) -> Result<CprDeckResponse, String> {
    deck_at(&corpus_root(), &deck, &programs, &hardware, &extras, interface_rank)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..").join(GameSystem::CyberpunkRed.corpus_relative())
    }

    /// Second implementation: the MANIFEST read as plain JSON, files counted as arrays.
    fn manifest_pairs() -> Vec<(String, usize, bool, usize)> {
        let manifest: Value =
            serde_json::from_str(&std::fs::read_to_string(root().join("MANIFEST.json")).unwrap()).unwrap();
        manifest["kinds"]
            .as_array()
            .unwrap()
            .iter()
            .map(|k| {
                let file = k["file"].as_str().unwrap();
                let rows: Vec<Value> =
                    serde_json::from_str(&std::fs::read_to_string(root().join(file)).unwrap()).unwrap();
                (
                    k["kind"].as_str().unwrap().to_owned(),
                    k["expected"].as_u64().unwrap() as usize,
                    k["print_only"].as_bool().unwrap(),
                    rows.len(),
                )
            })
            .collect()
    }

    #[test]
    fn every_manifest_kind_lists_its_expected_count() {
        let pairs = manifest_pairs();
        assert_eq!(pairs.len(), 53);
        for (kind, expected, print_only, on_disk) in pairs {
            let response = catalog_at(&root(), &kind).unwrap_or_else(|e| panic!("{kind}: {e}"));
            assert_eq!(response.records.len(), expected, "{kind} records vs MANIFEST expected");
            assert_eq!(on_disk, expected, "{kind} file rows vs MANIFEST expected");
            assert_eq!(response.expected, expected);
            assert_eq!(response.print_only, print_only, "{kind} print_only");
            assert!(response.records.iter().all(|r| r["kind"] == kind.as_str()), "{kind} record kind tag");
        }
    }

    #[test]
    fn an_unknown_kind_is_refused() {
        let err = catalog_at(&root(), "not_a_kind").unwrap_err();
        assert!(err.contains("unknown Cyberpunk RED catalog kind"), "{err}");
        assert!(catalog_at(&root(), "../MANIFEST").is_err());
        assert!(catalog_at(&root(), "").is_err());
    }

    #[test]
    fn the_command_reads_the_same_corpus_root() {
        let response = list_cyberpunk_catalog("program".to_owned()).unwrap();
        assert_eq!(response.records.len(), 27);
        assert_eq!(response.source, "data/cyberpunk-red/corpus");
    }

    fn names(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| (*s).to_owned()).collect()
    }

    const STANDARD: &str = "Cyberdeck (Standard Quality)";
    const P157: [&str; 6] = ["Sword", "Sword", "Killer", "Worm", "Worm", "Armor"];

    #[test]
    fn the_p157_sample_deck_is_seven_of_seven() {
        let r = deck_at(&root(), STANDARD, &names(&P157), &[], &CprDeckExtrasDto::default(), None).unwrap();
        assert_eq!((r.slots_used, r.slots_available), (7, 7));
        let used = r.rows.iter().find(|x| x.id == "cpr.netrunning.deck_slots_used").unwrap();
        assert_eq!(used.value, 7);
    }

    #[test]
    fn an_eighth_slot_is_refused() {
        let err = deck_at(&root(), STANDARD, &names(&P157), &names(&["Range Upgrade"]), &CprDeckExtrasDto::default(), None)
            .unwrap_err();
        assert!(err.contains("needs 8 slots"), "{err}");
    }

    #[test]
    fn extras_add_slots_and_hardware_slots_come_from_the_corpus() {
        let extras = CprDeckExtrasDto { bodyweight_suit: true, cyberarm_deck: true };
        let r = deck_at(&root(), STANDARD, &names(&P157), &names(&["Range Upgrade"]), &extras, Some(7)).unwrap();
        assert_eq!((r.slots_used, r.slots_available), (8, 9));
        assert_eq!(r.bodyweight_suit_hardware.as_deref(), Some("Range Upgrade"));
        // A 2-slot Hardware cannot use the suit's slot: 7 + 2 = 9 general slots needed, 8 exist.
        assert!(deck_at(&root(), STANDARD, &names(&P157), &names(&["KRASH Barrier"]), &extras, None).is_err());
        assert_eq!(r.net_actions.unwrap().value, 4);
    }

    #[test]
    fn unknown_deck_program_hardware_and_rank_are_refused() {
        let none = CprDeckExtrasDto::default();
        assert!(deck_at(&root(), "Cyberdeck", &[], &[], &none, None).is_err());
        assert!(deck_at(&root(), STANDARD, &names(&["Nope"]), &[], &none, None).unwrap_err().contains("Nope"));
        assert!(deck_at(&root(), STANDARD, &[], &names(&["Bodyweight Suit"]), &none, None).is_err());
        assert!(deck_at(&root(), STANDARD, &[], &[], &none, Some(11)).is_err());
    }

    #[test]
    fn each_deck_quality_name_is_a_corpus_cyberdeck_record() {
        let decks = catalog_at(&root(), "cyberdeck").unwrap();
        for quality in CprDeckQuality::ALL {
            let record = decks.records.iter().find(|r| r["name"] == quality.name()).expect("deck record");
            assert_eq!(record["slots"], u64::from(quality.base_slots()));
        }
    }
}
