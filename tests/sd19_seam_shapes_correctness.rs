//! SD-19 main capability slice: seam-shapes-correctness proof.
//!
//! This is the RED/GREEN test for atomic commit 2 (capability slice) per
//! `SD-19-core-rules-spell-equipment-reachability-scope-draft.md` §1.1 and
//! `technical-design.md` §5.2. It proves, before any loop cycle runs:
//!
//! (a) the seam function and wrapper types exist with the documented shapes;
//! (b) the resolvers return `Some` for the 13 fixture identities and `None`
//!     for an unknown identity;
//! (c) `CharacterInput.spells_selected` round-trips through the existing
//!     fixture-text parser (this repo has no serde anywhere in `rules_core` —
//!     see the review note atop `technical-design.md` §3);
//! (d) the new `MatrixSubjectType` variants construct and compare correctly
//!     (same reason: no serde to round-trip through);
//! (e) an end-to-end call over all 13 real-corpus fixtures produces a
//!     `CorpusPilotReceipt` whose `corpus_derived` is non-empty AND whose
//!     `base` equals `compute_pilot_base_chassis` run on the same input
//!     directly.
//!
//! The 13 fixtures are read directly from `tests/fixtures/rules_core/` — no
//! external `PCGEN_CORPUS_ROOT` checkout is required, since the fixture text
//! is itself the real corpus content, copied verbatim and checked in.

use codex::pcgen_import::ir_converter::{convert_equipment_record, convert_spell_record};
use codex::pcgen_import::lst_parser::equipment::parse_equipment_entries;
use codex::pcgen_import::lst_parser::spell::parse_lst_spell_row;
use codex::rules_core::character_input::{
    AbilityScores, AcquisitionMode, CharacterClassLevel, CharacterInput, ChosenCharacterState,
    SpellSelection,
};
use codex::rules_core::equipment_resolver::equipment_id_resolve;
use codex::rules_core::pilot_compute::compute_pilot_base_chassis;
use codex::rules_core::pilot_compute_corpus::compute_pilot_with_corpus;
use codex::rules_core::rules_tables::RuleSetId;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};
use codex::rules_core::spell_resolver::spell_id_resolve;
use codex::rules_core::support_state_matrix::MatrixSubjectType;
use codex::rules_core::rules_tables::crb::equipment_tables::EquipmentCategory;
use codex::rules_core::rules_tables::crb::spell_list::Pf1SchoolId;

const SPELL_FIXTURES: &[(&str, &str)] = &[
    ("abjuration", "Alarm"),
    ("conjuration", "Acid Arrow"),
    ("divination", "Analyze Dweomer"),
    ("enchantment", "Aid"),
    ("evocation", "Blade Barrier"),
    ("illusion", "Blur"),
    ("necromancy", "Animate Dead"),
    ("transmutation", "Air Walk"),
    ("universal", "Arcane Mark"),
];

const EQUIPMENT_FIXTURES: &[(&str, &str)] = &[
    ("arms_armor", "Longsword (Base)"),
    ("general", "Backpack"),
    ("magic_items", "Potion of Aid"),
    ("equipmods", "Material ~ Cloth"),
];

fn fixture_text(name: &str) -> &'static str {
    match name {
        "abjuration" => include_str!("fixtures/rules_core/sd19_seam_crb_spell_abjuration.txt"),
        "conjuration" => include_str!("fixtures/rules_core/sd19_seam_crb_spell_conjuration.txt"),
        "divination" => include_str!("fixtures/rules_core/sd19_seam_crb_spell_divination.txt"),
        "enchantment" => include_str!("fixtures/rules_core/sd19_seam_crb_spell_enchantment.txt"),
        "evocation" => include_str!("fixtures/rules_core/sd19_seam_crb_spell_evocation.txt"),
        "illusion" => include_str!("fixtures/rules_core/sd19_seam_crb_spell_illusion.txt"),
        "necromancy" => include_str!("fixtures/rules_core/sd19_seam_crb_spell_necromancy.txt"),
        "transmutation" => {
            include_str!("fixtures/rules_core/sd19_seam_crb_spell_transmutation.txt")
        }
        "universal" => include_str!("fixtures/rules_core/sd19_seam_crb_spell_universal.txt"),
        "arms_armor" => include_str!("fixtures/rules_core/sd19_seam_crb_equip_arms_armor.txt"),
        "general" => include_str!("fixtures/rules_core/sd19_seam_crb_equip_general.txt"),
        "magic_items" => include_str!("fixtures/rules_core/sd19_seam_crb_equip_magic_items.txt"),
        "equipmods" => include_str!("fixtures/rules_core/sd19_seam_crb_equip_equipmods.txt"),
        other => panic!("no fixture registered for '{other}'"),
    }
}

/// Strips the fixture's leading `# Fixture for ...` comment line.
fn record_line(fixture: &str) -> &str {
    fixture
        .lines()
        .find(|line| !line.trim_start().starts_with('#') && !line.trim().is_empty())
        .expect("fixture must contain a non-comment record line")
}

fn build_corpus() -> SourcePackageContent<'static> {
    let source_ref = SourceRef {
        lst_file: "sd19_seam_crb_fixtures".to_string(),
        line: 1,
    };
    let mut corpus = SourcePackageContent::empty("sd19_seam_shapes_correctness", source_ref);

    for (school, _) in SPELL_FIXTURES {
        let line = record_line(fixture_text(school));
        let parsed = parse_lst_spell_row("sd19_seam_crb_fixtures", 1, line);
        let record = parsed
            .record
            .unwrap_or_else(|| panic!("spell fixture '{school}' failed to parse: {line}"));
        // Leak: fixtures are a small, fixed, test-only set; leaking to get a
        // `'static` borrow for the record is simpler and just as correct as
        // a full arena for a 13-entry, process-lifetime test corpus.
        let record: &'static codex::pcgen_import::lst_parser::spell::LstSpellRecord =
            Box::leak(Box::new(record));
        corpus.push(convert_spell_record(record));
    }

    for (category, _) in EQUIPMENT_FIXTURES {
        let line = record_line(fixture_text(category));
        let result = parse_equipment_entries("sd19_seam_crb_fixtures", line);
        assert!(
            !result.entries.is_empty(),
            "equipment fixture '{category}' failed to parse: {line}"
        );
        for entry in result.entries {
            let entry: &'static codex::pcgen_import::lst_parser::equipment::EquipmentRecord =
                Box::leak(Box::new(entry));
            corpus.push(convert_equipment_record(entry));
        }
    }

    corpus
}

fn base_input() -> CharacterInput {
    CharacterInput {
        case_id: Some("sd19_seam_shapes_correctness".to_string()),
        source_package_id: "sd19_seam_shapes_correctness".to_string(),
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

// --- (b) resolvers return Some for fixtures, None for unknowns -------------

#[test]
fn spell_id_resolve_returns_some_for_every_fixture_and_none_for_unknown() {
    let corpus = build_corpus();
    for (_, spell_name) in SPELL_FIXTURES {
        let resolved = spell_id_resolve(spell_name, RuleSetId::Crb, &corpus);
        assert!(
            resolved.is_some(),
            "expected spell_id_resolve to resolve fixture spell '{spell_name}'"
        );
    }
    assert!(
        spell_id_resolve("Not A Real Spell", RuleSetId::Crb, &corpus).is_none(),
        "expected spell_id_resolve to return None for an unknown spell id"
    );
}

#[test]
fn equipment_id_resolve_returns_some_for_every_fixture_and_none_for_unknown() {
    let corpus = build_corpus();
    for (_, item_key) in EQUIPMENT_FIXTURES {
        let resolved = equipment_id_resolve(item_key, RuleSetId::Crb, &corpus);
        assert!(
            resolved.is_some(),
            "expected equipment_id_resolve to resolve fixture item '{item_key}'"
        );
    }
    assert!(
        equipment_id_resolve("item:not_a_real_item", RuleSetId::Crb, &corpus).is_none(),
        "expected equipment_id_resolve to return None for an unknown item id"
    );
}

// --- (c) CharacterInput.spells_selected round-trips through fixture text --

#[test]
fn spells_selected_round_trips_through_fixture_text() {
    let text = "\
source_package_id=sd19_seam_shapes_correctness\n\
race_id=human\n\
class_level=wizard:1\n\
ability=strength:10\n\
ability=dexterity:10\n\
ability=constitution:10\n\
ability=intelligence:10\n\
ability=wisdom:10\n\
ability=charisma:10\n\
spell=Alarm:wizard:known\n\
spell=Blur:wizard:prepared\n\
";
    let result = codex::rules_core::character_input::load_character_input_fixture(text);
    assert!(
        result.diagnostics.is_empty(),
        "unexpected diagnostics: {:?}",
        result.diagnostics
    );
    let input = result.character_input.expect("valid character input");
    assert_eq!(
        input.chosen.spells_selected,
        vec![
            SpellSelection {
                spell_id: "Alarm".to_string(),
                source_class_id: "wizard".to_string(),
                acquisition_mode: AcquisitionMode::Known,
            },
            SpellSelection {
                spell_id: "Blur".to_string(),
                source_class_id: "wizard".to_string(),
                acquisition_mode: AcquisitionMode::Prepared,
            },
        ]
    );
}

#[test]
fn omitting_spell_lines_leaves_spells_selected_empty_for_backward_compatibility() {
    let text = "\
source_package_id=sd19_seam_shapes_correctness\n\
race_id=human\n\
class_level=fighter:1\n\
ability=strength:10\n\
ability=dexterity:10\n\
ability=constitution:10\n\
ability=intelligence:10\n\
ability=wisdom:10\n\
ability=charisma:10\n\
";
    let result = codex::rules_core::character_input::load_character_input_fixture(text);
    assert!(result.diagnostics.is_empty());
    let input = result.character_input.expect("valid character input");
    assert!(input.chosen.spells_selected.is_empty());
}

// --- (d) MatrixSubjectType new variants construct and compare correctly ---

#[test]
fn matrix_subject_type_school_and_equipment_variants_construct_and_compare() {
    let a = MatrixSubjectType::School(Pf1SchoolId::Abjuration);
    let b = MatrixSubjectType::School(Pf1SchoolId::Abjuration);
    let c = MatrixSubjectType::School(Pf1SchoolId::Conjuration);
    assert_eq!(a, b);
    assert_ne!(a, c);

    let d = MatrixSubjectType::Equipment(EquipmentCategory::ArmsArmor);
    let e = MatrixSubjectType::Equipment(EquipmentCategory::ArmsArmor);
    let f = MatrixSubjectType::Equipment(EquipmentCategory::General);
    assert_eq!(d, e);
    assert_ne!(d, f);

    assert_ne!(MatrixSubjectType::Class, MatrixSubjectType::Race);
}

// --- (e) end-to-end: 13 fixtures -> non-empty corpus_derived, base equality

#[test]
fn end_to_end_seam_call_over_all_13_fixtures_is_non_empty_and_preserves_base_chassis() {
    let corpus = build_corpus();
    let mut input = base_input();

    for (_, spell_name) in SPELL_FIXTURES {
        input.chosen.spells_selected.push(SpellSelection {
            spell_id: spell_name.to_string(),
            source_class_id: "fighter".to_string(),
            acquisition_mode: AcquisitionMode::Granted,
        });
    }
    for (_, item_key) in EQUIPMENT_FIXTURES {
        input
            .chosen
            .equipment_selections
            .push(codex::rules_core::character_input::EquipmentSelection {
                item_id: item_key.to_string(),
                equipped_or_active: true,
                active_state: codex::rules_core::character_input::ActiveState::EquippedActive,
                applied_modifiers: Vec::new(),
            });
    }

    let receipt = compute_pilot_with_corpus(&input, &corpus);

    assert!(
        !receipt.corpus_derived.school_coverage.is_empty(),
        "expected at least one resolved school in corpus_derived.school_coverage"
    );
    assert!(
        !receipt.corpus_derived.equipped_items.is_empty(),
        "expected at least one resolved item in corpus_derived.equipped_items"
    );
    assert_eq!(
        receipt.corpus_derived.school_coverage.len(),
        SPELL_FIXTURES.len(),
        "expected every fixture school to resolve into its own school_coverage entry"
    );
    assert_eq!(
        receipt.corpus_derived.equipped_items.len(),
        EQUIPMENT_FIXTURES.len(),
        "expected every fixture item to resolve into its own equipped_items entry"
    );
    for coverage in receipt.corpus_derived.school_coverage.values() {
        assert!(
            coverage.table_cell.is_some(),
            "expected {:?}'s table_cell to resolve through the foundation slice's table store",
            coverage.school
        );
    }
    for item in &receipt.corpus_derived.equipped_items {
        assert!(
            item.table_cell.is_some(),
            "expected {}'s table_cell to resolve through the foundation slice's table store",
            item.item_id
        );
    }

    let base_direct = compute_pilot_base_chassis(&input);
    assert_eq!(
        receipt.base, base_direct,
        "compute_pilot_with_corpus's base chassis must equal a direct \
         compute_pilot_base_chassis call on the same input"
    );
}
