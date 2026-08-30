//! SD-20 Epic 2 cycle 4 — Enchantment spellbook engine RED test
//! (`SD-20-rules-engine-completeness-scope-draft.md` §1.2,
//! `technical-design.md` §2.1).
//!
//! Fourth PF1 spell school landed per §1.2 Step 2's cycle order
//! (abjuration landed in `3147b28`, conjuration landed in `4f53724`,
//! divination landed in `a7568a5`; this cycle lands enchantment;
//! evocation, illusion, necromancy, transmutation, universal remain).
//! Mirrors `tests/sd20_spellbook_divination.rs` exactly, substituting a
//! real Enchantment spell for the real Divination spell.
//!
//! Per §1.2's per-cycle test description: a `CharacterInput` with at
//! least one spell of the landed school (Enchantment) produces a
//! non-empty `SpellbookCoverage`; the spell save DC is computed (not
//! hardcoded — asserted against the real PF1 formula, `10 + spell level +
//! ability modifier`, not a literal); bonus slots from a high ability
//! score are computed per `bonus_slots_from_ability`.
//!
//! Builds a minimal in-memory corpus via
//! `pcgen_import::lst_parser::spell::parse_lst_spell_row` (a single real
//! `cr_spells.lst`-shaped row for "Charm Person" — independently
//! confirmed against `rules_tables::crb::spell_list::SPELL_LIST`'s own
//! Enchantment entry before this test was written: school Enchantment,
//! level 1, per the loop instruction's Step 4 corpus-existence check)
//! rather than depending on `CORPUS_ROOT` or hand-copying the full corpus
//! file -- `spellbook::compute_spellbook_coverage` only needs one
//! resolvable Enchantment spell selection to exercise its full round trip.

use codex::pcgen_import::ir_converter::convert_spell_record;
use codex::pcgen_import::lst_parser::spell::parse_lst_spell_row;
use codex::rules_core::character_input::{
    AbilityScores, AcquisitionMode, CharacterClassLevel, CharacterInput, ChosenCharacterState,
    SpellSelection,
};
use codex::rules_core::rules_tables::crb::spell_list::Pf1SchoolId;
use codex::rules_core::source_content::{SourcePackageContent, SourceRef};
use codex::rules_core::spellbook::compute_spellbook_coverage;

fn corpus_with_charm_person() -> SourcePackageContent<'static> {
    let raw_line = "Charm Person\tSCHOOL:Enchantment\tCLASSES:Sorcerer=1,Wizard=1\t\
        DESCRIPTION:This charm makes a humanoid creature regard you as its trusted friend and ally.";
    let parsed = parse_lst_spell_row("sd20_spellbook_enchantment", 1, raw_line);
    let record = parsed
        .record
        .expect("the raw row above must parse into a real LstSpellRecord");
    assert_eq!(record.school.as_deref(), Some("Enchantment"));

    let source_ref = SourceRef {
        lst_file: "sd20_spellbook_enchantment".to_string(),
        line: 1,
    };
    let mut corpus = SourcePackageContent::empty("sd20_spellbook_enchantment", source_ref);
    // `convert_spell_record` borrows its input; leak the record so the
    // corpus (which the test keeps for its full lifetime) can hold a
    // `'static` borrow without a lifetime-juggling helper struct. Test-only,
    // bounded to a handful of records per test run.
    let record: &'static _ = Box::leak(Box::new(record));
    corpus.push(convert_spell_record(record));
    corpus
}

fn wizard_input_with_charm_person_prepared(intelligence: i16) -> CharacterInput {
    CharacterInput {
        case_id: Some("sd20_spellbook_enchantment".to_string()),
        source_package_id: "sd20_spellbook_enchantment".to_string(),
        chosen: ChosenCharacterState {
            selected_traits: Vec::new(),
            race_id: "human".to_string(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:wizard".to_string(),
                level: 1,
            }],
            ability_scores: AbilityScores {
                strength: 10,
                dexterity: 10,
                constitution: 10,
                intelligence,
                wisdom: 10,
                charisma: 10,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            spells_selected: vec![SpellSelection {
                spell_id: "Charm Person".to_string(),
                source_class_id: "class:wizard".to_string(),
                acquisition_mode: AcquisitionMode::Prepared,
            }],
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

#[test]
fn enchantment_spell_selection_produces_non_empty_spellbook_coverage() {
    let corpus = corpus_with_charm_person();
    let input = wizard_input_with_charm_person_prepared(17); // +3 Intelligence modifier

    let coverage = compute_spellbook_coverage(&input, &corpus);

    assert_eq!(
        coverage.spells_prepared.len(),
        1,
        "expected exactly one prepared spell: {:?}",
        coverage.spells_prepared
    );
    let prepared = &coverage.spells_prepared[0];
    assert_eq!(prepared.effect.spell_id, "Charm Person");
    assert_eq!(prepared.effect.school, Pf1SchoolId::Enchantment);
    assert_eq!(prepared.effect.level, 1);
    assert!(
        !prepared.effect.effect_text.is_empty(),
        "effect text must be populated from the table store, not left empty"
    );
    assert_eq!(prepared.source_class_id, "class:wizard");
    assert_eq!(prepared.effect.table_cell.table, "spell_list");
    assert_eq!(prepared.effect.table_cell.row_key, "Charm Person");

    assert!(
        coverage.spells_known.is_empty(),
        "a Prepared-mode selection must not also land in spells_known"
    );
}

#[test]
fn spell_save_dc_is_computed_from_the_real_pf1_formula_not_hardcoded() {
    let corpus = corpus_with_charm_person();

    // 10 + spell level (1) + Intelligence modifier drives the DC; two
    // different Intelligence scores must produce two different DCs, which
    // a hardcoded constant could never do.
    let low_int =
        compute_spellbook_coverage(&wizard_input_with_charm_person_prepared(10), &corpus);
    let high_int =
        compute_spellbook_coverage(&wizard_input_with_charm_person_prepared(18), &corpus);

    let low_dc = *low_int
        .spell_save_dc
        .get("class:wizard")
        .expect("expected a spell_save_dc entry for class:wizard");
    let high_dc = *high_int
        .spell_save_dc
        .get("class:wizard")
        .expect("expected a spell_save_dc entry for class:wizard");

    // INT 10 -> modifier 0 -> DC 10 + 1 + 0 = 11.
    assert_eq!(low_dc, 11, "DC must follow 10 + spell level + ability modifier, not a hardcoded value");
    // INT 18 -> modifier +4 -> DC 10 + 1 + 4 = 15.
    assert_eq!(high_dc, 15, "DC must follow 10 + spell level + ability modifier, not a hardcoded value");
    assert_ne!(low_dc, high_dc, "DC must vary with ability score, proving it is computed, not hardcoded");
}

#[test]
fn bonus_slots_from_ability_are_computed_for_a_high_casting_ability() {
    let corpus = corpus_with_charm_person();
    // INT 18 -> modifier +4 -> PF1 Table 1-3 bonus slots: level 1 through
    // 4 each gain exactly one bonus slot (modifier >= level, and
    // (modifier - level) / 4 + 1 == 1 for level in 1..=4); no 5th-level+
    // bonus slot since modifier (4) < 5.
    let input = wizard_input_with_charm_person_prepared(18);
    let coverage = compute_spellbook_coverage(&input, &corpus);

    for level in 1..=4u8 {
        assert_eq!(
            coverage.bonus_slots_from_ability.get(&level).copied(),
            Some(1),
            "expected exactly one bonus slot at level {level} for a +4 ability modifier: {:?}",
            coverage.bonus_slots_from_ability
        );
    }
    assert!(
        !coverage.bonus_slots_from_ability.contains_key(&5),
        "a +4 ability modifier must not grant a 5th-level bonus slot: {:?}",
        coverage.bonus_slots_from_ability
    );
}

#[test]
fn a_low_ability_modifier_grants_no_bonus_slots() {
    let corpus = corpus_with_charm_person();
    let input = wizard_input_with_charm_person_prepared(10); // +0 modifier
    let coverage = compute_spellbook_coverage(&input, &corpus);

    assert!(
        coverage.bonus_slots_from_ability.is_empty(),
        "a +0 ability modifier must grant no bonus slots: {:?}",
        coverage.bonus_slots_from_ability
    );
}

#[test]
fn resolve_enchantment_spell_effect_returns_none_for_a_non_enchantment_spell() {
    // "Mage Armor" is a real SPELL_LIST record but Conjuration, not
    // Enchantment -- this school's contribution function must not claim a
    // spell that belongs to a different school.
    assert!(
        codex::rules_core::spellbook::enchantment::resolve_enchantment_spell_effect("Mage Armor")
            .is_none()
    );
}

#[test]
fn resolve_enchantment_spell_effect_returns_none_for_an_unknown_spell_id() {
    assert!(
        codex::rules_core::spellbook::enchantment::resolve_enchantment_spell_effect(
            "Not A Real Spell"
        )
        .is_none()
    );
}
