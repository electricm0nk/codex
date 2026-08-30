//! SD-19 §2.5 equipment card: equipmods — full coverage (the last of the
//! 4 categories, closing the full-coverage §2.5 sweep per the operator's
//! amended loop instruction).
//!
//! Per the operator's amended loop instruction (2026-07-16, "i want to
//! make sure that we brought in ALL spells, ALL armor, ALL weapons, ALL
//! equipment, not just samples"), this proves **every independently
//! addressable** real-corpus `cr_equipmods.lst` record — not a
//! representative sample — is (a) resolvable via `equipment_id_resolve`,
//! (b) present in `CorpusPilotReceipt.corpus_derived.equipped_items`
//! after a call to `compute_pilot_with_corpus`, and (c) carries a
//! non-`None` `TableCellRef` grounding it to the CRB table store's
//! now-complete `equipment_data::equipmods::EQUIPMODS_TABLE`.
//!
//! **"Independently addressable" is 344 of the file's 658 raw lines, not
//! 658.** This cycle discovered (data-driven, not a guess — see the
//! discovery method below) that `cr_equipmods.lst` follows a systematic
//! PCGen convention: every one of 314 canonical modifiers (which has its
//! own real `KEY:` token) is paired with a hidden legacy alias row —
//! `<CanonicalName>.COPY=<ALLCAPS>`, no `KEY:` token of its own,
//! `VISIBLE:NO` — whose fallback identity (its name after the `.COPY=`
//! strip) is *identical* to the canonical record's real `KEY:` value
//! (e.g. `Material ~ Steel` the canonical KEY, and a separate
//! `Material ~ Steel.COPY=STEEL` row whose extracted name is also
//! `Material ~ Steel`). This is not a parser or resolver defect —
//! `VISIBLE:NO` is PCGen's own explicit "not independently selectable"
//! marker, confirmed for all 314 collisions with zero exceptions (see
//! this cycle's investigation). These hidden rows are legacy/technical
//! scaffolding, not real selectable equipment; the acceptance criterion's
//! "every item" means every item a character can actually select, which
//! is the 344 addressable identities. All 658 raw records are still
//! transcribed verbatim into `EQUIPMODS_TABLE` (nothing is silently
//! dropped from the table store) — this test's *reachability* claim is
//! scoped to the 344 that don't collide.
//!
//! Supersedes the prior representative-sample cycle (`c46c9b6`), which
//! *discovered* the plain-duplicate-name merge-collapse defect (two
//! unrelated "Cloth" rows) this full-coverage cycle depends on having
//! fixed (`22eeed9`). Also required the equipment resolver's
//! exact-name-match fix (added alongside this cycle).
//!
//! Reads the real PCGen corpus directly (the per-cycle `CORPUS_ROOT`
//! pattern from `decisions.md` §6.6), skipping with a documented
//! `eprintln!` when `CORPUS_ROOT` is unset.

use std::collections::HashMap;
use std::path::PathBuf;

use codex::pcgen_import::ir_converter::convert_equipment_record;
use codex::pcgen_import::lst_parser::equipment::parse_equipment_file;
use codex::rules_core::character_input::{
    AbilityScores, ActiveState, CharacterClassLevel, CharacterInput, ChosenCharacterState,
    EquipmentSelection,
};
use codex::rules_core::equipment_resolver::{equipment_id_resolve, equipment_key_token};
use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
use codex::rules_core::rules_tables::crb::equipment_tables::EquipmentCategory;
use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};
use codex::rules_core::support_state_matrix::{
    EvidenceTier, MatrixSubjectType, SupportState, seeded_current_truth,
};

fn corpus_root() -> Option<PathBuf> {
    match std::env::var("CORPUS_ROOT") {
        Ok(value) => {
            let path = PathBuf::from(value);
            if path.is_dir() { Some(path) } else { None }
        }
        Err(_) => None,
    }
}

fn cr_equipmods_path(root: &std::path::Path) -> PathBuf {
    root.join("pathfinder/paizo/roleplaying_game/core_rulebook/cr_equipmods.lst")
}

fn base_input() -> CharacterInput {
    CharacterInput {
        case_id: Some("sd19_equipment_equipmods".to_string()),
        source_package_id: "sd19_equipment_equipmods".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
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
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

/// True iff this record is a hidden PCGen legacy alias row: no `KEY:`
/// token of its own, and explicitly marked `VISIBLE:NO`. Confirmed by
/// this cycle's investigation to account for all 314 identity collisions
/// in `cr_equipmods.lst`, with zero unexplained exceptions.
fn is_hidden_legacy_alias(record: &codex::pcgen_import::lst_parser::equipment::EquipmentRecord) -> bool {
    equipment_key_token(record).is_none()
        && record
            .tokens
            .iter()
            .any(|t| t.key == "VISIBLE" && t.value == "NO")
}

#[test]
fn every_addressable_real_corpus_item_resolves_reaches_equipped_items_and_grounds_through_table_cell()
{
    let Some(root) = corpus_root() else {
        eprintln!(
            "CORPUS_ROOT not set or not a directory; skipping (set \
             CORPUS_ROOT=$HOME/workspace/repos/pcgen/data to enable)"
        );
        return;
    };
    let cr_equipmods = cr_equipmods_path(&root);
    if !cr_equipmods.is_file() {
        eprintln!(
            "canonical cr_equipmods.lst not present at {}; skipping",
            cr_equipmods.display()
        );
        return;
    }

    let parsed = parse_equipment_file(&cr_equipmods).expect("cr_equipmods.lst must parse");
    assert!(
        !parsed.entries.is_empty(),
        "corpus-existence check must find equipmods records"
    );
    assert_eq!(
        parsed.entries.len(),
        658,
        "expected 658 distinct equipmods records post-merge-fix; if this \
         changes the corpus or the parser changed — regenerate \
         equipment_data::equipmods"
    );

    let hidden_alias_count = parsed.entries.iter().filter(|r| is_hidden_legacy_alias(r)).count();
    assert_eq!(
        hidden_alias_count, 314,
        "expected exactly 314 hidden legacy alias rows (no KEY:, VISIBLE:NO); if this \
         changed, the addressable-item count and the reachability claim below must be \
         re-derived, not assumed"
    );

    let source_ref = SourceRef {
        lst_file: cr_equipmods.display().to_string(),
        line: 1,
    };
    let mut corpus = SourcePackageContent::empty("sd19_equipment_equipmods", source_ref);
    for record in &parsed.entries {
        corpus.push(convert_equipment_record(record));
    }

    // Only independently-addressable records (not hidden legacy aliases)
    // are expected to resolve to themselves and reach equipped_items.
    let addressable: Vec<_> = parsed
        .entries
        .iter()
        .filter(|r| !is_hidden_legacy_alias(r))
        .collect();
    assert_eq!(addressable.len(), 344);

    // Every addressable identity must be unique (no remaining collisions
    // once hidden aliases are excluded).
    let mut seen: HashMap<&str, &str> = HashMap::new();
    for record in &addressable {
        let identity = equipment_key_token(record).unwrap_or(&record.name);
        if let Some(prior_name) = seen.insert(identity, &record.name) {
            panic!(
                "unexpected remaining identity collision for '{identity}': '{prior_name}' vs '{}'",
                record.name
            );
        }
    }

    let mut input = base_input();
    for record in &addressable {
        let identity = equipment_key_token(record).unwrap_or(&record.name);
        let resolved = equipment_id_resolve(identity, RuleSetId::Crb, &corpus);
        let (resolved_record, table_cell) = resolved.unwrap_or_else(|| {
            panic!("expected equipment_id_resolve to resolve '{identity}'")
        });
        assert_eq!(&resolved_record.name, &record.name);
        assert!(
            table_cell.is_some(),
            "expected '{identity}' to ground through the full CRB table store"
        );
        input.chosen.equipment_selections.push(EquipmentSelection {
            item_id: identity.to_string(),
            equipped_or_active: true,
            active_state: ActiveState::EquippedActive,
            applied_modifiers: Vec::new(),
        });
    }

    let receipt = compute_pilot_with_corpus(&input, &corpus);
    assert_eq!(
        receipt.corpus_derived.equipped_items.len(),
        addressable.len(),
        "expected every one of the {} addressable equipmods records to resolve into its \
         own equipped_items entry",
        addressable.len()
    );
    for item in &receipt.corpus_derived.equipped_items {
        assert!(
            item.table_cell.is_some(),
            "expected equipped_items entry for '{}' to carry a table_cell",
            item.item_id
        );
        assert!(!item.equipment_record_name.is_empty());
    }
}

#[test]
fn equipmods_matrix_row_reflects_full_coverage() {
    let matrix = seeded_current_truth();
    let row = matrix
        .rows
        .iter()
        .find(|r| r.subject_type == MatrixSubjectType::Equipment(EquipmentCategory::Equipmods))
        .expect("expected an Equipment(Equipmods) row in the seeded matrix");

    assert_eq!(row.support_state, SupportState::Supported);
    assert_eq!(row.evidence_tier, EvidenceTier::ProductVisible);
    assert!(
        row.grounding_ref.contains("sd19_equipment_equipmods"),
        "expected the row's grounding_ref to cite this cycle's proof test, got: {}",
        row.grounding_ref
    );
    assert!(
        row.blocker_or_lossiness_note.contains("344"),
        "expected the row's note to describe full addressable coverage (344/658, \
         314 hidden legacy aliases excluded), got: {}",
        row.blocker_or_lossiness_note
    );
}
