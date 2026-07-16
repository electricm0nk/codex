//! SD-19 §2.5 equipment card: arms_armor.
//!
//! Proves a representative sample of real-corpus `cr_equip_arms_armor.lst`
//! records — Longsword (`KEY:Longsword (Base)`, line 165), Banded Mail
//! (`KEY:Banded Mail (Base)`, line 46), and Armor Spikes
//! (`KEY:Armor Spikes`, line 137) — each independently confirmed present
//! via `grep -n "KEY:<token>" cr_equip_arms_armor.lst` before this test
//! was written, per the loop instruction's Step 4 corpus-existence check —
//! is (a) resolvable via `equipment_id_resolve`, and (b) present in
//! `CorpusPilotReceipt.corpus_derived.equipped_items` after a call to
//! `compute_pilot_with_corpus`.
//!
//! Unlike the §2.4 spell-school cycles (which land every spell in the
//! school), §2.5 cycles land a representative sample per
//! `scope-draft.md` §2.5 / the loop instruction's Step 2 ("landing a
//! representative sample of items per round").
//!
//! Reads the real PCGen corpus directly (the per-cycle `CORPUS_ROOT`
//! pattern from `decisions.md` §6.6), skipping with a documented
//! `eprintln!` when `CORPUS_ROOT` is unset, matching the §2.4 cycles'
//! skip semantics.

use std::path::PathBuf;

use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::parse_equipment_file;
use codex::rules_core::character_input::{
    AbilityScores, ActiveState, CharacterClassLevel, CharacterInput, ChosenCharacterState,
    EquipmentSelection,
};
use codex::rules_core::equipment_resolver::equipment_id_resolve;
use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
use codex::rules_core::rules_tables::crb::equipment_tables::EquipmentCategory;
use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};
use codex::rules_core::support_state_matrix::{
    EvidenceTier, MatrixSubjectType, SupportState, seeded_sd13_e1_f1_current_truth,
};

/// The cycle's chosen representative sample: `(item_id, corpus KEY: token)`.
const REPRESENTATIVE_SAMPLE: &[(&str, &str)] = &[
    ("item:longsword", "Longsword (Base)"),
    ("item:banded_mail", "Banded Mail (Base)"),
    ("item:armor_spikes", "Armor Spikes"),
];

fn corpus_root() -> Option<PathBuf> {
    match std::env::var("CORPUS_ROOT") {
        Ok(value) => {
            let path = PathBuf::from(value);
            if path.is_dir() { Some(path) } else { None }
        }
        Err(_) => None,
    }
}

fn cr_equip_arms_armor_path(root: &std::path::Path) -> PathBuf {
    root.join("pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_arms_armor.lst")
}

fn base_input() -> CharacterInput {
    CharacterInput {
        case_id: Some("sd19_equipment_arms_armor".to_string()),
        source_package_id: "sd19_equipment_arms_armor".to_string(),
        chosen: ChosenCharacterState {
            race_id: "human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "fighter".to_string(),
                level: 1,
            }],
            ability_scores: AbilityScores {
                strength: 10,
                dexterity: 10,
                constitution: 10,
                intelligence: 10,
                wisdom: 10,
                charisma: 10,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            spells_selected: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

#[test]
fn representative_sample_key_tokens_match_real_corpus() {
    let Some(root) = corpus_root() else {
        eprintln!(
            "CORPUS_ROOT not set or not a directory; skipping (set \
             CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data to enable)"
        );
        return;
    };
    let cr_equip_arms_armor = cr_equip_arms_armor_path(&root);
    if !cr_equip_arms_armor.is_file() {
        eprintln!(
            "canonical cr_equip_arms_armor.lst not present at {}; skipping",
            cr_equip_arms_armor.display()
        );
        return;
    }

    let parsed = parse_equipment_file(&cr_equip_arms_armor)
        .expect("cr_equip_arms_armor.lst must parse");

    for (_, key_token) in REPRESENTATIVE_SAMPLE {
        assert!(
            parsed.entries.iter().any(|record| {
                record
                    .tokens
                    .iter()
                    .any(|token| token.key == "KEY" && token.value == *key_token)
            }),
            "corpus-existence check: expected KEY:{key_token} among \
             cr_equip_arms_armor.lst records"
        );
    }
}

#[test]
fn every_sample_item_resolves_and_reaches_equipped_items() {
    let Some(root) = corpus_root() else {
        eprintln!(
            "CORPUS_ROOT not set or not a directory; skipping (set \
             CORPUS_ROOT=/home/ubuntu/workspace/repos/pcgen/data to enable)"
        );
        return;
    };
    let cr_equip_arms_armor = cr_equip_arms_armor_path(&root);
    if !cr_equip_arms_armor.is_file() {
        eprintln!(
            "canonical cr_equip_arms_armor.lst not present at {}; skipping",
            cr_equip_arms_armor.display()
        );
        return;
    }

    let parsed = parse_equipment_file(&cr_equip_arms_armor)
        .expect("cr_equip_arms_armor.lst must parse");
    assert!(
        !parsed.entries.is_empty(),
        "corpus-existence check must find arms_armor records"
    );

    let source_ref = SourceRef {
        lst_file: cr_equip_arms_armor.display().to_string(),
        line: 1,
    };
    let mut corpus = SourcePackageContent::empty("sd19_equipment_arms_armor", source_ref);
    for record in &parsed.entries {
        corpus.push(convert_equipment_record(record));
    }

    // (a) every sample item resolves via equipment_id_resolve.
    for (item_id, key_token) in REPRESENTATIVE_SAMPLE {
        let resolved = equipment_id_resolve(item_id, RuleSetId::Crb, &corpus);
        assert!(
            resolved.is_some(),
            "expected equipment_id_resolve to resolve sample item '{item_id}' (KEY:{key_token})"
        );
    }

    // (b) every sample item is present in corpus_derived.equipped_items
    // after a call to compute_pilot_with_corpus, with its
    // equipment_record_name / equipment_record_key populated.
    let mut input = base_input();
    for (item_id, _) in REPRESENTATIVE_SAMPLE {
        input.chosen.equipment_selections.push(EquipmentSelection {
            item_id: item_id.to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
        });
    }

    let receipt = compute_pilot_with_corpus(&input, &corpus);
    assert_eq!(
        receipt.corpus_derived.equipped_items.len(),
        REPRESENTATIVE_SAMPLE.len(),
        "expected every sample item to resolve into its own equipped_items entry"
    );

    for (item_id, key_token) in REPRESENTATIVE_SAMPLE {
        let entry = receipt
            .corpus_derived
            .equipped_items
            .iter()
            .find(|item| &item.item_id == item_id)
            .unwrap_or_else(|| panic!("expected an equipped_items entry for '{item_id}'"));
        assert_eq!(&entry.equipment_record_key, key_token);
        assert!(
            !entry.equipment_record_name.is_empty(),
            "expected a non-empty equipment_record_name for '{item_id}'"
        );
        if *item_id == "item:longsword" {
            // The foundation slice's single arms_armor bootstrap table
            // entry is keyed "Longsword (Base)" (rules_tables::crb::
            // equipment_tables::EQUIPMENT_TABLES); only this sample item
            // grounds through a table cell today.
            assert!(
                entry.table_cell.is_some(),
                "expected 'item:longsword' to resolve through the foundation slice's \
                 bootstrap table cell"
            );
        } else {
            assert!(
                entry.table_cell.is_none(),
                "arms_armor sample items beyond the foundation slice's single bootstrap \
                 entry (Longsword) are not yet in the table store; table_cell stays None \
                 until a future cycle widens rules_tables::crb::equipment_tables"
            );
        }
    }
}

#[test]
fn arms_armor_matrix_row_reflects_the_grounded_reachability_proof() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let row = matrix
        .rows
        .iter()
        .find(|r| r.subject_type == MatrixSubjectType::Equipment(EquipmentCategory::ArmsArmor))
        .expect("expected an Equipment(ArmsArmor) row in the seeded matrix");

    assert_eq!(row.support_state, SupportState::Partial);
    assert_eq!(row.evidence_tier, EvidenceTier::Computed);
    assert!(
        row.grounding_ref.contains("sd19_equipment_arms_armor"),
        "expected the row's grounding_ref to cite this cycle's proof test, got: {}",
        row.grounding_ref
    );
    assert!(
        !row.blocker_or_lossiness_note.is_empty(),
        "expected a non-empty blocker_or_lossiness_note on a Partial row"
    );
}
