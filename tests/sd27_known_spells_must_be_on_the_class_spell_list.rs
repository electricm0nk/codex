//! SD-27 — the **Known** acquisition mode gets its own correctness rule,
//! and it is deliberately NOT the Prepared rule.
//!
//! ## The defect
//!
//! `unmet_wizard_spellbook_conditions` validated the *prepared* set three
//! ways (every prepared spell recorded in the spellbook, at a spell level
//! the wizard's own level grants access to, within that level's slot
//! budget) and validated the *recorded* (`AcquisitionMode::Known`) set
//! exactly once: that it was non-empty. Nothing checked that a recorded
//! spell was a **wizard spell at all**.
//!
//! That gap is what let the desktop Add Spell picker persist any of the
//! catalog's 1185 records under `class:wizard`. Re-derived here rather than
//! asserted from memory (`every_catalog_row_off_the_wizard_list_is_refused`
//! below sweeps the live catalog): **543 of the 1185 records are on no
//! wizard list in any ingested book** — Cleric-only, Druid-only,
//! Bard-only, Alchemist-only spells a wizard can never scribe.
//!
//! Sorcerer already had exactly this check
//! (`unmet_sorcerer_known_spell_conditions`: *"known spell '{id}' is not on
//! the real PF1 sorcerer spell list"*). Wizard did not. The asymmetry was
//! the bug.
//!
//! ## The rule applied to Known, and why it is not the Prepared rule
//!
//! **Membership is gated; spell level is not.** PF1 CRB, *Spellbooks*: a
//! wizard's two free spells at each new level must be *"of spell levels he
//! can cast"*, but *"Spells Copied from Another's Spellbook or a Scroll"*
//! places **no character-level restriction** on what a wizard may add to
//! her spellbook — the cost is a Spellcraft check at DC 15 + spell level.
//! A spellbook is a *record*, not a set of castable options. So a Wizard 1
//! scribing Tsunami (APG, wizard level 9) is legal PF1 and is deliberately
//! **not** blocked here; what she cannot do is *prepare* or *cast* it, and
//! that half was already enforced (*"a prepared spell targets spell level
//! N, not yet accessible at wizard level L"*).
//!
//! `wizard_1_may_scribe_a_ninth_level_spell_but_never_prepare_it` pins both
//! halves at once, so a later cycle cannot "tidy" the Known path by cloning
//! the Prepared gate onto it.
//!
//! ## Standing guard (decisions.md §28)
//!
//! `pilot_compute.rs` changed, so the before/after is pinned per posture:
//! `the_populated_spellbook_posture_is_byte_identical_to_before` re-asserts
//! the exact SD-21 acceptance shape (3 recorded / 2 prepared / 4 and 3
//! slots) that this file's change must leave untouched.

use codex::rules_core::character_input::{AcquisitionMode, SpellSelection};
use codex::rules_core::pilot_compute::{
    ComputationExplanation, PilotBaseChassisComputation, compute_pilot_base_chassis,
};
use codex::rules_core::rules_tables::class_spell_levels;
use codex::rules_core::rules_tables::{
    acg, adventurers_guide, advanced_race_guide, apg, crb, inner_sea_faiths, inner_sea_gods,
    inner_sea_magic, inner_sea_temples, occult_adventures, ultimate_combat, ultimate_intrigue,
    ultimate_magic, ultimate_wilderness,
};
mod common;
use common::load;

const WIZARD_LEVEL_3_FIXTURE: &str =
    include_str!("fixtures/rules_core/pf1_human_wizard_level3_sd13_deterministic_input.txt");

/// The exact diagnostic id `unmet_wizard_spellbook_conditions`'s unmet list
/// is reported under (both diagnostics carry the same joined list; this is
/// the spellbook-specific one).
const SPELLBOOK_DIAGNOSTIC: &str = "class_spell.wizard.prepared_spellbook.unsupported";

fn wizard_spell(spell_id: &str, mode: AcquisitionMode) -> SpellSelection {
    SpellSelection {
        spell_id: spell_id.to_string(),
        source_class_id: "class:wizard".to_string(),
        acquisition_mode: mode,
    }
}

fn explanation<'a>(
    computation: &'a PilotBaseChassisComputation,
    id: &str,
) -> &'a ComputationExplanation {
    computation
        .explanations
        .iter()
        .find(|e| e.id == id)
        .unwrap_or_else(|| panic!("expected explanation id '{id}'"))
}

/// The joined unmet-condition text for the spellbook diagnostic, or `None`
/// when the posture is met and the diagnostic never fired.
fn spellbook_block_reason(computation: &PilotBaseChassisComputation) -> Option<String> {
    computation
        .diagnostics
        .iter()
        .find(|d| d.id == SPELLBOOK_DIAGNOSTIC)
        .map(|d| d.message.clone())
}

/// Every record the desktop Add Spell picker serves, across all four
/// ingested books — the exact set `spell_catalog.rs::build_spell_catalog`
/// assembles.
fn full_desktop_spell_catalog() -> Vec<&'static str> {
    crb::spell_list::SPELL_LIST
        .iter()
        .map(|e| e.key)
        .chain(apg::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .chain(acg::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .chain(
            advanced_race_guide::spell_list::SPELL_LIST
                .iter()
                .map(|e| e.key),
        )
        .chain(ultimate_intrigue::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .chain(ultimate_magic::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .chain(occult_adventures::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .chain(ultimate_combat::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .chain(inner_sea_gods::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .chain(ultimate_wilderness::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .chain(adventurers_guide::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .chain(inner_sea_faiths::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .chain(inner_sea_magic::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .chain(inner_sea_temples::spell_list::SPELL_LIST.iter().map(|e| e.key))
        .collect()
}

// ---------------------------------------------------------------------
// The defect: an off-list spell recorded as Known
// ---------------------------------------------------------------------

/// `Cure Light Wounds` is a Cleric/Druid/Bard/Paladin/Ranger spell. No
/// wizard list in any ingested book carries it, so no wizard can scribe it
/// into a spellbook at any level, with any Intelligence, ever.
#[test]
fn a_recorded_spell_that_is_not_on_the_wizard_list_blocks_the_spellbook_posture() {
    assert!(
        class_spell_levels::class_spell_level("class:wizard", "Cure Light Wounds").is_none(),
        "premise: Cure Light Wounds must genuinely be off the wizard list"
    );

    let mut input = load(WIZARD_LEVEL_3_FIXTURE);
    input.chosen.spells_selected = vec![
        wizard_spell("evocation.1.magic_missile", AcquisitionMode::Known),
        wizard_spell("Cure Light Wounds", AcquisitionMode::Known),
        wizard_spell("evocation.1.magic_missile", AcquisitionMode::Prepared),
    ];

    let reason = spellbook_block_reason(&compute_pilot_base_chassis(&input)).unwrap_or_else(|| {
        panic!("a wizard whose spellbook records a non-wizard spell must be claim-blocked")
    });
    assert!(
        reason.contains("Cure Light Wounds") && reason.contains("wizard spell list"),
        "the block must name the offending spell and the rule it broke, got: {reason}"
    );
}

/// The sweep, so the rule cannot be satisfied by a hand-listed handful:
/// every one of the picker's records that is off the wizard list is
/// refused, and the count is re-derived here rather than quoted.
#[test]
fn every_catalog_row_off_the_wizard_list_is_refused() {
    let catalog = full_desktop_spell_catalog();
    let off_list: Vec<&str> = catalog
        .iter()
        .copied()
        .filter(|key| class_spell_levels::class_spell_level("class:wizard", key).is_none())
        .collect();

    assert_eq!(
        catalog.len(),
        2127,
        "the desktop Add Spell picker serves this many records"
    );
    assert_eq!(
        off_list.len(),
        1369 + 45 + 57 + 14,
        "this many of them are on no wizard list in any ingested book -- SD-31 wave-29 \
         (`lane5-book-onboard` lane) added `adventurers_guide`'s 45 spells, none of which are \
         on any wizard list in any ingested book (re-derived, not assumed); SD-32 Gate 0 \
         book-onboarding precondition (`gate-0-book-onboarding-precondition`, AT-32-G0-003) \
         added inner_sea_faiths/inner_sea_magic/inner_sea_temples' 57 spells (2 + 34 + 21), \
         none of which are on any wizard list in any ingested book either (re-derived); \
         `ea2a72dd64` (SD-32 `decisions.md §24` PI-name-blocked spell close, 2026-08-23) then \
         added 14 more Codex-generated-neutral-name spells across `inner_sea_gods` (4), \
         `adventurers_guide` (4), `inner_sea_faiths` (1) and `inner_sea_magic` (5) -- re-derived \
         per-book via `git show a50b7da04c:<path> | grep -c 'SpellListEntry {{'` against the \
         live file, none of which are on any wizard list either (this test's own off_list \
         filter, re-run, confirms all 14 land in off_list -- the +14/+14 delta matching exactly \
         is not assumed)"
    );

    // Sampling the whole 543 through `compute_pilot_base_chassis` would be
    // 543 full chassis computations; a deterministic spread across the
    // range proves the same rule without that cost.
    for key in off_list.iter().step_by(37) {
        let mut input = load(WIZARD_LEVEL_3_FIXTURE);
        input.chosen.spells_selected = vec![
            wizard_spell("evocation.1.magic_missile", AcquisitionMode::Known),
            wizard_spell(key, AcquisitionMode::Known),
            wizard_spell("evocation.1.magic_missile", AcquisitionMode::Prepared),
        ];
        let reason = spellbook_block_reason(&compute_pilot_base_chassis(&input))
            .unwrap_or_else(|| panic!("'{key}' is off the wizard list but was accepted as Known"));
        assert!(
            reason.contains(key),
            "the block for '{key}' must name it, got: {reason}"
        );
    }
}

// ---------------------------------------------------------------------
// The half that is legal, and stays legal
// ---------------------------------------------------------------------

/// PF1 places no character-level cap on what a spellbook may *contain*, so
/// a 9th-level wizard spell recorded as Known is accepted; the same spell
/// prepared is refused. Both halves pinned in one test so neither can be
/// changed without the other being reconsidered.
#[test]
fn wizard_1_may_scribe_a_ninth_level_spell_but_never_prepare_it() {
    assert_eq!(
        class_spell_levels::class_spell_level("class:wizard", "Tsunami"),
        Some(9),
        "premise: Tsunami is a wizard spell, at wizard level 9"
    );

    let mut scribed = load(WIZARD_LEVEL_3_FIXTURE);
    scribed.chosen.spells_selected = vec![
        wizard_spell("evocation.1.magic_missile", AcquisitionMode::Known),
        wizard_spell("Tsunami", AcquisitionMode::Known),
        wizard_spell("evocation.1.magic_missile", AcquisitionMode::Prepared),
    ];
    assert_eq!(
        spellbook_block_reason(&compute_pilot_base_chassis(&scribed)),
        None,
        "scribing a 9th-level wizard spell into a Wizard 3's spellbook is legal PF1 \
         (CRB, Spells Copied from Another's Spellbook or a Scroll — no level restriction)"
    );
    assert!(
        explanation(
            &compute_pilot_base_chassis(&scribed),
            "class_spell.wizard.spellbook_contents"
        )
        .detail
        .contains("Tsunami"),
        "and it must actually reach the grounded spellbook contents"
    );

    let mut prepared = scribed.clone();
    prepared
        .chosen
        .spells_selected
        .push(wizard_spell("Tsunami", AcquisitionMode::Prepared));
    let reason = spellbook_block_reason(&compute_pilot_base_chassis(&prepared))
        .expect("preparing a 9th-level spell at wizard level 3 must stay blocked");
    assert!(
        reason.contains("not yet accessible at wizard level 3"),
        "the Prepared rule is the level-access rule, unchanged: {reason}"
    );
}

/// The Known rule is membership, not spelling: a real wizard spell named
/// exactly as the catalog serves it is accepted at every level 0-9.
#[test]
fn a_real_wizard_spell_at_any_spell_level_is_accepted_as_known() {
    for (key, expected_level) in [
        ("Light", 0u8),
        ("Magic Missile", 1),
        ("Scorching Ray", 2),
        ("Fireball", 3),
        ("Tsunami", 9),
    ] {
        assert_eq!(
            class_spell_levels::class_spell_level("class:wizard", key),
            Some(expected_level),
            "premise for '{key}'"
        );
        let mut input = load(WIZARD_LEVEL_3_FIXTURE);
        input.chosen.spells_selected = vec![
            wizard_spell("evocation.1.magic_missile", AcquisitionMode::Known),
            wizard_spell(key, AcquisitionMode::Known),
            wizard_spell("evocation.1.magic_missile", AcquisitionMode::Prepared),
        ];
        assert_eq!(
            spellbook_block_reason(&compute_pilot_base_chassis(&input)),
            None,
            "'{key}' is a real wizard spell and must be scribable"
        );
    }
}

// ---------------------------------------------------------------------
// Standing guard (decisions.md §28): the before/after pin
// ---------------------------------------------------------------------

/// The SD-21 acceptance posture, re-asserted verbatim: this cycle's change
/// adds a refusal for spells that were never wizard spells, and must leave
/// every already-valid spellbook computing exactly what it did before.
///
/// The three synthetic `<school>.<level>.<name>` ids are the convention
/// `tests/sd21_wizard_prepared_spellbook.rs` owns; they resolve through
/// `resolve_prepared_spell_level`'s dotted-id branch, not through a class
/// list, so this also pins that the new membership check does not refuse
/// them.
#[test]
fn the_populated_spellbook_posture_is_byte_identical_to_before() {
    let mut input = load(WIZARD_LEVEL_3_FIXTURE);
    input.chosen.spells_selected = vec![
        wizard_spell("evocation.1.magic_missile", AcquisitionMode::Known),
        wizard_spell("evocation.1.burning_hands", AcquisitionMode::Known),
        wizard_spell("evocation.2.scorching_ray", AcquisitionMode::Known),
        wizard_spell("evocation.1.magic_missile", AcquisitionMode::Prepared),
        wizard_spell("evocation.2.scorching_ray", AcquisitionMode::Prepared),
    ];
    let computation = compute_pilot_base_chassis(&input);

    assert_eq!(spellbook_block_reason(&computation), None);
    assert_eq!(
        explanation(&computation, "class_spell.wizard.spellbook_contents").value,
        3
    );
    assert_eq!(
        explanation(&computation, "class_spell.wizard.daily_preparation").value,
        2
    );
    assert_eq!(
        explanation(
            &computation,
            "class_spell.wizard.total_spells_per_day.spell_level_1"
        )
        .value,
        4
    );
    assert_eq!(
        explanation(
            &computation,
            "class_spell.wizard.total_spells_per_day.spell_level_2"
        )
        .value,
        3
    );
}

/// Negative control for the guard above: the check is wizard-scoped, so a
/// spell recorded under a *different* source class is not swept into it.
#[test]
fn a_non_wizard_source_class_is_not_subject_to_the_wizard_membership_rule() {
    let mut input = load(WIZARD_LEVEL_3_FIXTURE);
    input.chosen.spells_selected = vec![
        wizard_spell("evocation.1.magic_missile", AcquisitionMode::Known),
        wizard_spell("evocation.1.magic_missile", AcquisitionMode::Prepared),
        SpellSelection {
            spell_id: "Cure Light Wounds".to_string(),
            source_class_id: "class:cleric".to_string(),
            acquisition_mode: AcquisitionMode::Known,
        },
    ];
    assert_eq!(
        spellbook_block_reason(&compute_pilot_base_chassis(&input)),
        None,
        "a cleric spell recorded under class:cleric is none of the wizard rule's business"
    );
}
