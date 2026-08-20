//! SD-31 — CRB's own 29 field-less-`CLASSES:` variant spell rows resolve
//! against their base spell's level, the same way SD-27's
//! `sd27_apg_delta_spell_rows_resolve_against_their_base.rs` already fixed
//! the cross-book shape of this exact defect.
//!
//! # The defect
//!
//! `crb::spell_list::SPELL_LIST` types `level` non-optionally (`u8`, never
//! `Option<u8>` — see that module's own doc comment), so every record needs
//! *some* value. 29 records are named `<base spell> (<qualifier> Only)` /
//! `<base spell> (<qualifier> Spell Only)` — narrow variants of a real,
//! higher-level spell (an elemental-only `Elemental Swarm`, an
//! energy-substitution `Burning Hands (Acid)`, a target-restricted
//! `Align Weapon (Chaos Only)`, and so on). Their own `cr_spells.lst` row
//! carries no `CLASSES:` token at all — PCGen's convention for a spell whose
//! class list and level are inherited from the base spell it names — so the
//! generator that built this table had nothing to read a level from and
//! defaulted to `0`. That default is silently indistinguishable from a real
//! 0th-level cantrip: `Planar Binding (Devils and Fiendish Creatures Only)`
//! (a 6th-level Sorcerer/Wizard spell) shipped reading `Level 0` on
//! `SpellCatalogScreen.tsx` and in the Character Sheet's Add Spell picker —
//! a wrong, player-facing value on a `done`-shaped record (`OPEN-ISSUES.md`
//! row 43, wave-3 integration; re-derived and root-caused independently this
//! cycle, wave 22).
//!
//! # Why the fix is a lookup, not a hand-typed table
//!
//! Every one of the 29 names strips to a base name (drop the trailing
//! parenthetical) that already has its own, independently-verified-correct
//! `SPELL_LIST` entry — `Align Weapon (Chaos Only)` -> `Align Weapon`,
//! `Elemental Body IV (Air Only)` -> `Elemental Body IV`, etc. Confirmed
//! against the pinned oracle (`scripts/pcgen-oracle-pin.env`,
//! `7f818006e371188e5717fd18d74d18a420747fc6`) for all 29: the base name's
//! own `cr_spells.lst` row DOES carry a real `CLASSES:` token, and every
//! variant's row shares the base's `SCHOOL:` exactly. This is the identical
//! shape SD-27 already fixed for APG's `.COPY=` rows that cross a book
//! boundary (`tests/sd27_apg_delta_spell_rows_resolve_against_their_base.rs`)
//! — this is the same defect *within* one book, which SD-27's own doc
//! comment named as future work ("cross-book spell variant resolution") and
//! never revisited for CRB's own in-book variants.
//!
//! `SPELL_LIST` is a flat `const` array and cannot call a lookup at compile
//! time, so the corrected `level` is a literal in the table, exactly as
//! SD-27's inherited fields are — this test is what keeps that literal
//! honest: it reads the base record at runtime and requires equality, so if
//! `crb::spell_list`'s base entry is ever corrected, the stale variant copy
//! fails here rather than quietly disagreeing with its own base spell.
use codex::rules_core::rules_tables::crb;

/// `(variant key, base key)` for every CRB record whose own `cr_spells.lst`
/// row carries no `CLASSES:` token and whose name is `<base> (<qualifier>)`.
/// Re-derived against the pinned oracle this cycle (all 29 `level: 0` table
/// rows were audited; 28 genuinely have a `CLASSES:` token of their own —
/// real cantrips, e.g. `Acid Splash`, `Detect Magic` — and stay untouched).
const CRB_WITHIN_BOOK_VARIANT_ROWS: &[(&str, &str)] = &[
    ("Elemental Swarm (Air Spell Only)", "Elemental Swarm"),
    ("Elemental Swarm (Earth Spell Only)", "Elemental Swarm"),
    ("Elemental Swarm (Fire Spell Only)", "Elemental Swarm"),
    ("Elemental Swarm (Water Spell Only)", "Elemental Swarm"),
    ("Planar Binding (Devils and Fiendish Creatures Only)", "Planar Binding"),
    ("Summon Monster IX (Chaos Spell Only)", "Summon Monster IX"),
    ("Summon Monster IX (Evil Spell Only)", "Summon Monster IX"),
    ("Summon Monster IX (Good Spell Only)", "Summon Monster IX"),
    ("Summon Monster IX (Law Spell Only)", "Summon Monster IX"),
    ("Summon Monster V (1d3 Shadows)", "Summon Monster V"),
    ("Summon Monster VIII (Elementals Only)", "Summon Monster VIII"),
    ("Summon Nature's Ally IV (Animals Only)", "Summon Nature's Ally IV"),
    ("Summon Nature's Ally VIII (Animals Only)", "Summon Nature's Ally VIII"),
    ("Burning Hands (Acid)", "Burning Hands"),
    ("Burning Hands (Cold)", "Burning Hands"),
    ("Burning Hands (Electricity)", "Burning Hands"),
    ("Scorching Ray (Acid)", "Scorching Ray"),
    ("Scorching Ray (Cold)", "Scorching Ray"),
    ("Scorching Ray (Electricity)", "Scorching Ray"),
    ("Blindness/Deafness (Blindness Only)", "Blindness/Deafness"),
    ("Align Weapon (Chaos Only)", "Align Weapon"),
    ("Align Weapon (Evil Only)", "Align Weapon"),
    ("Align Weapon (Good Only)", "Align Weapon"),
    ("Align Weapon (Law Only)", "Align Weapon"),
    ("Beast Shape III (Animals Only)", "Beast Shape III"),
    ("Elemental Body IV (Air Only)", "Elemental Body IV"),
    ("Elemental Body IV (Earth Only)", "Elemental Body IV"),
    ("Elemental Body IV (Fire Only)", "Elemental Body IV"),
    ("Elemental Body IV (Water Only)", "Elemental Body IV"),
];

fn crb_entry(key: &str) -> &'static crb::spell_list::SpellListEntry {
    crb::spell_list::SPELL_LIST
        .iter()
        .find(|entry| entry.key == key)
        .unwrap_or_else(|| panic!("CRB record {key:?} must exist"))
}

#[test]
fn every_within_book_variant_row_inherits_its_base_spells_level_and_school() {
    for (variant, base) in CRB_WITHIN_BOOK_VARIANT_ROWS {
        let copy = crb_entry(variant);
        let source = crb_entry(base);
        assert_eq!(
            format!("{:?}", copy.school),
            format!("{:?}", source.school),
            "{variant} must inherit {base}'s school"
        );
        assert_eq!(
            copy.level, source.level,
            "{variant} must inherit {base}'s level -- its own corpus row carries no CLASSES: \
             token, so it cannot state a level any other way, and 0 is a real cantrip level, \
             not a safe default"
        );
        // The base's own level must never itself be the fallback default --
        // that would make this test pass by both sides being wrong.
        assert_ne!(
            source.level, 0,
            "{base} itself must be a real, resolved (non-cantrip) level for this pairing to \
             mean anything"
        );
    }
    assert_eq!(CRB_WITHIN_BOOK_VARIANT_ROWS.len(), 29);
}

/// The regression pin: before this cycle, all 29 of the above read `level:
/// 0` verbatim in the committed table (row 43's exact defect). This does not
/// re-run the oracle cross-reference (that is a one-time audit, recorded in
/// this file's own doc comment and this cycle's `progress.md` receipt); it
/// pins that the fix landed and stays landed.
#[test]
fn no_within_book_variant_row_is_left_reading_the_stale_zero_default() {
    for (variant, _base) in CRB_WITHIN_BOOK_VARIANT_ROWS {
        assert_ne!(
            crb_entry(variant).level,
            0,
            "{variant} regressed to the stale zero-default level"
        );
    }
}
