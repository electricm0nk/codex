//! Scratch probe (not part of the codex repo) — calls the real, public
//! `codex::rules_core::equipment_effects::compute_equipment_effects` engine
//! entry point against the real `data/corpus/ultimate_equipment/equipment/`
//! records to derive genuine "ours" computed ability-score-bonus values for
//! the 11 fixture-verified `equipment` units under AT-33-E5-001.
//!
//! Exists outside the codex repo tree deliberately: AT-33-E5-001's granted
//! write scope is `artifacts/epic-5-reverification/` plus Epic 2's harness
//! plus an append-only `THE-BOX.md`, not `src/`. This binary only READS the
//! codex crate as a path dependency; it writes nothing into the codex repo.

use codex::rules_core::character_input::{ActiveState, EquipmentSelection};
use codex::rules_core::corpus_loader::{load_equipment_corpus, BookCorpusRoot};
use codex::rules_core::equipment_effects::compute_equipment_effects;
use std::path::Path;

fn main() {
    let repo_root = Path::new("/home/ubuntu/workspace/repos/codex");
    let dir = repo_root.join("data/corpus/ultimate_equipment");
    let root = BookCorpusRoot { book_id: "ultimate_equipment", dir: &dir };
    let corpus = load_equipment_corpus(&[root]);

    let names = [
        "Belt of Mighty Hurling (Greater)",
        "Belt of Mighty Hurling (Lesser)",
        "Shifter's Headband (CHA) +2",
        "Shifter's Headband (CHA) +4",
        "Shifter's Headband (CHA) +6",
        "Shifter's Headband (INT) +2",
        "Shifter's Headband (INT) +4",
        "Shifter's Headband (INT) +6",
        "Shifter's Headband (WIS) +2",
        "Shifter's Headband (WIS) +4",
        "Shifter's Headband (WIS) +6",
    ];

    let mut out = Vec::new();
    for name in names {
        let selection = EquipmentSelection {
            item_id: name.to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        };
        let effects = compute_equipment_effects(&[selection], &corpus);
        let item = effects.per_item.first();
        let ability_bonus = item.and_then(|i| i.ability_bonus.as_ref());
        match ability_bonus {
            Some(ab) => {
                out.push(serde_json::json!({
                    "name": name,
                    "ability": ab.ability,
                    "bonus": ab.bonus,
                }));
            }
            None => {
                out.push(serde_json::json!({
                    "name": name,
                    "ability": serde_json::Value::Null,
                    "bonus": serde_json::Value::Null,
                    "note": "no ability_bonus resolved by compute_equipment_effects",
                }));
            }
        }
    }
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}
