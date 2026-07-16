//! SD-19 §2.5 equipment card: magic_items.
//!
//! Proves a representative sample of real-corpus `cr_equip_magic_items.lst`
//! records — Amulet of Natural Armor +1 (`KEY:Amulet of Natural Armor +1`,
//! line 20), Belt of Giant Strength +2 (`KEY:Belt of Giant Strength +2`),
//! and Ring of Protection +1 (`KEY:Ring of Protection +1`, line 396) —
//! each independently confirmed present via `grep -n "KEY:<token>"
//! cr_equip_magic_items.lst` before this test was written, per the loop
//! instruction's Step 4 corpus-existence check — is (a) resolvable via
//! `equipment_id_resolve`, and (b) present in
//! `CorpusPilotReceipt.corpus_derived.equipped_items` after a call to
//! `compute_pilot_with_corpus`.
//!
//! Unlike the §2.4 spell-school cycles (which land every spell in the
//! school), §2.5 cycles land a representative sample per
//! `scope-draft.md` §2.5 / the loop instruction's Step 2 ("landing a
//! representative sample of items per round").
//!
//! Sample items are addressed by their verbatim corpus `KEY:` token
//! (not the `"item:<name>"` fixture namespace) per `scope-draft.md` §2.5's
//! acceptance criterion, which names both forms as valid.
//!
//! **Known category-wide limitation, deliberately routed around, not
//! fixed, by this cycle:** a large share of `cr_equip_magic_items.lst`
//! (scrolls, wands, potions — the ~634/~351/~87 counts named in
//! `scope-draft.md` §2.5) use PCGen's `.COPY=` naming convention (e.g.
//! `Wand.COPY=Wand of Magic Missile`). `equipment.rs`'s
//! `extract_record_name` strips everything from `.COPY=` onward, so
//! `open_record`'s by-name merge (intended for `cr_equip_general.lst`'s
//! genuine same-name continuation rows) instead collapses every distinct
//! `.COPY=` item sharing a base word ("Wand", "Potion", "Scroll", ...)
//! into a single merged `EquipmentRecord` carrying many distinct `KEY:`
//! tokens. `equipment_id_resolve`'s `equipment_key_token` helper returns
//! only the first such token (by line number), so only the
//! alphabetically-first item under each merged name resolves correctly;
//! every other `.COPY=` item under that name is unreachable today. This
//! was discovered while drafting this cycle's sample (`Wand of Magic
//! Missile` failed to resolve for exactly this reason) and is a
//! parser-level defect in `src/pcgen_import/lst_parser/equipment.rs`
//! (SD-17's lane), not a resolver-normalization edge case this cycle's
//! file-touch scope (`equipment_resolver.rs`, not the parser) can fix.
//! The representative sample below is deliberately drawn from
//! non-`.COPY=` records (amulets, belts, rings — each independently
//! confirmed standalone: exactly one `KEY:` token per merged record) to
//! avoid this collision; see the progress doc's Open Blockers for the
//! full note routed to SD-17.
//!
//! Reads the real PCGen corpus directly (the per-cycle `CORPUS_ROOT`
//! pattern from `decisions.md` §6.6), skipping with a documented
//! `eprintln!` when `CORPUS_ROOT` is unset, matching the §2.4/§2.5
//! cycles' skip semantics.

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
/// `item_id` here is the verbatim `KEY:` token itself. All three are
/// non-`.COPY=` records (see module doc) so each is its own standalone
/// `EquipmentRecord` with exactly one `KEY:` token — no merge collision.
const REPRESENTATIVE_SAMPLE: &[(&str, &str)] = &[
    ("Amulet of Natural Armor +1", "Amulet of Natural Armor +1"),
    ("Belt of Giant Strength +2", "Belt of Giant Strength +2"),
    ("Ring of Protection +1", "Ring of Protection +1"),
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

fn cr_equip_magic_items_path(root: &std::path::Path) -> PathBuf {
    root.join("pathfinder/paizo/roleplaying_game/core_rulebook/cr_equip_magic_items.lst")
}

fn base_input() -> CharacterInput {
    CharacterInput {
        case_id: Some("sd19_equipment_magic_items".to_string()),
        source_package_id: "sd19_equipment_magic_items".to_string(),
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
    let cr_equip_magic_items = cr_equip_magic_items_path(&root);
    if !cr_equip_magic_items.is_file() {
        eprintln!(
            "canonical cr_equip_magic_items.lst not present at {}; skipping",
            cr_equip_magic_items.display()
        );
        return;
    }

    let parsed = parse_equipment_file(&cr_equip_magic_items)
        .expect("cr_equip_magic_items.lst must parse");

    for (_, key_token) in REPRESENTATIVE_SAMPLE {
        assert!(
            parsed.entries.iter().any(|record| {
                record
                    .tokens
                    .iter()
                    .any(|token| token.key == "KEY" && token.value == *key_token)
            }),
            "corpus-existence check: expected KEY:{key_token} among \
             cr_equip_magic_items.lst records"
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
    let cr_equip_magic_items = cr_equip_magic_items_path(&root);
    if !cr_equip_magic_items.is_file() {
        eprintln!(
            "canonical cr_equip_magic_items.lst not present at {}; skipping",
            cr_equip_magic_items.display()
        );
        return;
    }

    let parsed = parse_equipment_file(&cr_equip_magic_items)
        .expect("cr_equip_magic_items.lst must parse");
    assert!(
        !parsed.entries.is_empty(),
        "corpus-existence check must find magic_items records"
    );

    let source_ref = SourceRef {
        lst_file: cr_equip_magic_items.display().to_string(),
        line: 1,
    };
    let mut corpus = SourcePackageContent::empty("sd19_equipment_magic_items", source_ref);
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
        // The foundation slice's single magic_items bootstrap table entry
        // is keyed "Potion of Aid" (rules_tables::crb::equipment_tables::
        // EQUIPMENT_TABLES); none of this cycle's sample items match that
        // key (deliberately drawn from non-`.COPY=` records instead — see
        // module doc), so every sample item's table_cell stays None today.
        assert!(
            entry.table_cell.is_none(),
            "'{item_id}' is not the foundation slice's single magic_items bootstrap \
             entry (Potion of Aid); table_cell stays None until a future cycle widens \
             rules_tables::crb::equipment_tables"
        );
    }
}

#[test]
fn magic_items_matrix_row_reflects_the_grounded_reachability_proof() {
    let matrix = seeded_sd13_e1_f1_current_truth();
    let row = matrix
        .rows
        .iter()
        .find(|r| r.subject_type == MatrixSubjectType::Equipment(EquipmentCategory::MagicItems))
        .expect("expected an Equipment(MagicItems) row in the seeded matrix");

    assert_eq!(row.support_state, SupportState::Partial);
    assert_eq!(row.evidence_tier, EvidenceTier::Computed);
    assert!(
        row.grounding_ref.contains("sd19_equipment_magic_items"),
        "expected the row's grounding_ref to cite this cycle's proof test, got: {}",
        row.grounding_ref
    );
    assert!(
        !row.blocker_or_lossiness_note.is_empty(),
        "expected a non-empty blocker_or_lossiness_note on a Partial row"
    );
}
