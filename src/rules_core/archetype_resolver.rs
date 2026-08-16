//! SD-28-E30/C4.8 archetype-swap resolver.
//!
//! The generic primitive `archetype_claims_slot` unblocks the archetype
//! wiring the operator authorized (`decisions.md §59`): given a selected
//! archetype, does it replace a specific base-class feature slot? This is
//! the first thing every one of the seven `§59` vacuity comments needs
//! before it can be closed for real -- a character who has genuinely
//! selected the named archetype must make the previously-vacuous
//! deduction/flag real.
//!
//! **Aggregation, not a new parallel list.** Every tier-1 book's own
//! `archetype_tables::archetype_swap_tables()` (`§51`'s 403-record
//! catalog) is chained here into one combined lookup surface, the same
//! `equipment_resolver::equipment_catalog_rows()` shape this program
//! already proved for equipment (`§55`) -- not a hand-maintained
//! duplicate that can drift the way `equipment_keys` once did.
//!
//! **Selection input shape, new by necessity (no prior archetype-selection
//! mechanism existed anywhere in this codebase before this cycle --
//! confirmed by grep, zero hits for any "choice:archetype"-shaped id).**
//! An archetype selection is recorded as a `SelectedChoice` with
//! `choice_set_id: ARCHETYPE_CHOICE_ID` and `selection_id` equal to the
//! archetype's own real corpus `KEY:` (e.g. `"Alchemist Archetype ~
//! Plague Bringer"`), not an invented slug -- the table's own `key` field
//! is already a stable, unambiguous identifier, so reusing it directly
//! avoids inventing a second naming scheme the table and the input would
//! then have to agree on separately.

use crate::rules_core::character_input::CharacterInput;
use crate::rules_core::rules_tables::archetype_swap::ArchetypeSwapEntry;
use crate::rules_core::rules_tables::{
    acg, advanced_race_guide as arg, apg, ultimate_combat as uc, ultimate_magic as um,
    ultimate_psionics as upsi, ultimate_wilderness as uw,
};

/// The choice-set id a character's archetype selections are recorded
/// under. One character can carry multiple selections (one per class in
/// a multiclass build, or -- per corpus convention -- a class that allows
/// more than one archetype to be combined); `archetype_claims_slot` does
/// not assume at most one.
pub const ARCHETYPE_CHOICE_ID: &str = "choice:archetype";

/// Every tier-1 book's archetype-swap catalog, chained into one lookup
/// surface. `§51`'s own seven landed tables, in the same book order their
/// own tier-1 closure receipts landed in.
pub fn archetype_catalog_entries() -> &'static [ArchetypeSwapEntry] {
    static ENTRIES: std::sync::OnceLock<Vec<ArchetypeSwapEntry>> = std::sync::OnceLock::new();
    ENTRIES.get_or_init(|| {
        let mut all = Vec::new();
        all.extend_from_slice(upsi::archetype_tables::archetype_swap_tables());
        all.extend_from_slice(acg::archetype_tables::archetype_swap_tables());
        all.extend_from_slice(apg::archetype_tables::archetype_swap_tables());
        all.extend_from_slice(um::archetype_tables::archetype_swap_tables());
        all.extend_from_slice(uc::archetype_tables::archetype_swap_tables());
        all.extend_from_slice(arg::archetype_tables::archetype_swap_tables());
        all.extend_from_slice(uw::archetype_tables::archetype_swap_tables());
        all
    })
}

/// Resolves one archetype's own catalog entry by its real corpus `KEY:`.
pub fn archetype_resolve(key: &str) -> Option<&'static ArchetypeSwapEntry> {
    archetype_catalog_entries().iter().find(|entry| entry.key == key)
}

/// **The generic primitive.** True when the character has selected a real
/// archetype (for `subject`, e.g. `"Alchemist"`) whose own `replaces` list
/// names `slot_id` -- i.e. the named base-class feature slot has genuinely
/// been swapped out by an archetype this character actually holds, not
/// merely one that exists in the catalog.
///
/// Three things this deliberately does NOT do, matching this program's own
/// no-fabrication discipline:
/// - Does not infer a slot claim from the archetype's `grants` list --
///   `replaces` and `grants` are not paired 1:1 (`archetype_swap.rs`'s own
///   doc comment), so only `replaces` answers "was this slot taken away."
/// - Does not validate the selection against the archetype's own
///   `prerequisites` -- that is `feat_prereqs`' job elsewhere in this
///   codebase, the same separation of concerns `archetype_swap.rs`'s own
///   doc comment on `SpellFocusFact` already establishes for a different
///   chooser (ambiguity vs. prerequisite validation are different jobs).
/// - Does not assume at most one archetype selection per subject -- every
///   matching selection is checked, so a corpus-legal archetype
///   combination that touches the same slot from two directions is not
///   silently collapsed to "the first one wins."
pub fn archetype_claims_slot(input: &CharacterInput, subject: &str, slot_id: &str) -> bool {
    archetype_claiming_slot(input, subject, slot_id).is_some()
}

/// Same check as `archetype_claims_slot`, but returns the claiming
/// archetype's own display name (`archetype_name`) rather than a bare
/// bool -- for building an explanation that names which archetype is
/// responsible, rather than only stating that some unnamed one is.
/// `None` when no selected archetype claims the slot. When more than one
/// selected archetype claims the same slot, returns the first match in
/// `selected_choices` order -- a corpus-legal edge case (see this
/// module's own doc comment), not expected in practice for a single
/// class's own single-archetype-at-a-time convention.
pub fn archetype_claiming_slot(
    input: &CharacterInput,
    subject: &str,
    slot_id: &str,
) -> Option<&'static str> {
    archetype_claiming_slot_entry(input, subject, slot_id).map(|entry| entry.archetype_name)
}

/// Same primitive as `archetype_claiming_slot`, but returns the claiming
/// archetype's own full catalog entry rather than only its display name --
/// for a caller that needs to read a SPECIFIC named grant off the
/// superseding archetype (e.g. its own "~ Weapon and Armor Proficiency"
/// sub-feature text), not just the fact that supersession happened.
/// `archetype_claiming_slot` is now a thin wrapper over this (SD31-E4-F1-001,
/// 2026-08-16, Slayer's Weapon and Armor Proficiency supersession -- the
/// first caller that needed the full entry, not just the name).
pub fn archetype_claiming_slot_entry(
    input: &CharacterInput,
    subject: &str,
    slot_id: &str,
) -> Option<&'static ArchetypeSwapEntry> {
    input
        .chosen
        .selected_choices
        .iter()
        .filter(|choice| choice.choice_set_id == ARCHETYPE_CHOICE_ID)
        .filter_map(|choice| archetype_resolve(&choice.selection_id))
        .find(|entry| {
            entry.subject == subject
                && entry.replaces.map(|r| r.contains(&slot_id)).unwrap_or(false)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::character_input::SelectedChoice;

    /// Plague Bringer (ARG) is the real, catalog-confirmed test case: its
    /// own `replaces` list names all six Alchemist Poison Resistance/
    /// Immunity slots plus Mutagen. Confirms the primitive reads real
    /// corpus data, not a fixture.
    #[test]
    fn plague_bringer_claims_every_poison_resistance_slot() {
        let entry = archetype_resolve("Alchemist Archetype ~ Plague Bringer")
            .expect("Plague Bringer must resolve from the real ARG archetype table");
        assert_eq!(entry.subject, "Alchemist");
        for slot in [
            "AlchemistMutagen",
            "AlchemistPoisonResistance",
            "AlchemistPoisonResistance2",
            "AlchemistPoisonResistance4",
            "AlchemistPoisonResistance6",
            "AlchemistPoisonImmunity",
        ] {
            assert!(
                entry.replaces.unwrap().contains(&slot),
                "Plague Bringer's own catalog row must name {slot} as replaced"
            );
        }
    }

    fn input_with_archetype(key: &str) -> CharacterInput {
        let mut input = crate::rules_core::character_input::load_character_input_fixture(
            include_str!(
                "../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
            ),
        )
        .character_input
        .expect("fixture must load");
        input.chosen.selected_choices.push(SelectedChoice {
            choice_set_id: ARCHETYPE_CHOICE_ID.to_owned(),
            selection_id: key.to_owned(),
        });
        input
    }

    /// The primitive's own positive case: a character who has genuinely
    /// selected Plague Bringer claims the Poison Resistance slot.
    #[test]
    fn a_character_with_plague_bringer_selected_claims_the_poison_resistance_slot() {
        let input = input_with_archetype("Alchemist Archetype ~ Plague Bringer");
        assert!(archetype_claims_slot(&input, "Alchemist", "AlchemistPoisonResistance"));
        assert!(archetype_claims_slot(&input, "Alchemist", "AlchemistPoisonImmunity"));
    }

    /// The negative case, proven not assumed: a character with NO
    /// archetype selected must not claim any slot -- the base-class
    /// feature stays vacuous-claim-free.
    #[test]
    fn a_character_with_no_archetype_selected_claims_nothing() {
        let input = crate::rules_core::character_input::load_character_input_fixture(
            include_str!(
                "../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
            ),
        )
        .character_input
        .expect("fixture must load");
        assert!(!archetype_claims_slot(&input, "Alchemist", "AlchemistPoisonResistance"));
    }

    /// The second negative case: a character who selected a DIFFERENT
    /// archetype (one that does not touch Poison Resistance at all) must
    /// not claim that slot either -- proves the check is scoped to the
    /// selected archetype's own real `replaces` list, not "any archetype
    /// selected at all."
    #[test]
    fn a_different_alchemist_archetype_does_not_claim_an_unrelated_slot() {
        // Bramble Brewer (ARG) replaces only AlchemistDiscovery.
        let input = input_with_archetype("Alchemist Archetype ~ Bramble Brewer");
        assert!(!archetype_claims_slot(&input, "Alchemist", "AlchemistPoisonResistance"));
        assert!(archetype_claims_slot(&input, "Alchemist", "AlchemistDiscovery"));
    }

    /// Wrong-subject guard: an archetype selection recorded for one class
    /// must never claim another class's identically-named-feeling slot.
    #[test]
    fn a_wrong_subject_check_never_claims_the_slot() {
        let input = input_with_archetype("Alchemist Archetype ~ Plague Bringer");
        assert!(!archetype_claims_slot(&input, "Barbarian", "AlchemistPoisonResistance"));
    }

    /// `archetype_claiming_slot_entry` (SD31-E4-F1-001): the whole catalog
    /// entry comes back, not just the name, so a caller can read a specific
    /// named grant off it. Bounty Hunter's real ACG row is the test case --
    /// added by this same cycle, so this also proves the new Slayer
    /// archetype rows are reachable through this primitive end to end.
    #[test]
    fn claiming_slot_entry_returns_the_whole_catalog_row_with_its_grants() {
        let input = input_with_archetype("Slayer Archetype ~ Bounty Hunter");
        let entry = archetype_claiming_slot_entry(&input, "Slayer", "WeaponProficiencies")
            .expect("Bounty Hunter must claim the WeaponProficiencies slot");
        assert_eq!(entry.archetype_name, "Bounty Hunter");
        let grant = entry
            .grants
            .iter()
            .find(|g| g.grants_feature_key == "Bounty Hunter ~ Weapon and Armor Proficiency")
            .expect("Bounty Hunter must grant its own Weapon and Armor Proficiency sub-feature");
        assert!(grant.description.unwrap().contains("aklys"));
    }

    /// No selection, no entry -- the base case for the same new primitive.
    #[test]
    fn claiming_slot_entry_is_none_with_no_archetype_selected() {
        let input = crate::rules_core::character_input::load_character_input_fixture(
            include_str!(
                "../../tests/fixtures/rules_core/pf1_human_fighter_level1_ge06_deterministic_input.txt"
            ),
        )
        .character_input
        .expect("fixture must load");
        assert!(archetype_claiming_slot_entry(&input, "Slayer", "WeaponProficiencies").is_none());
    }
}
