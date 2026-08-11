//! Monster Codex (`SOURCESHORT:MC`) — `monster` + `monster_ability`.
//!
//! # This book is 2 monsters, and that is a corpus fact
//!
//! Monster Codex is not a bestiary: `docs/release/SD-29-corpus-wide-catch-up-lanes/loop-instruction.md`'s
//! corpus-shape notes record its weight as class features, feats, spells and
//! equipment, with **2** monster rows (`Seru`, `Bat (Sootwing)`) and **3**
//! monster-ability rows. Both counts are re-derived from
//! `docs/work-inventory.json`'s own units by
//! `scripts/transcribe_monster_tables.py`, not from a line count over
//! `mc_races.lst` (which counts rows the inventory's trap filters exclude).
//!
//! # Three token shapes Bonus Bestiary never carried
//!
//! Each was found by transcribing this book and each is now handled in the
//! shared transcriber rather than here:
//!
//! * **The ability link is the namespace, not the monster row.** None of the 3
//!   ability rows is named by a monster's `ABILITY:Special Ability` token; the
//!   owner is the first segment of the ability's own `KEY:`
//!   (`Seru ~ Poison` → `Seru`). Reading only Bonus Bestiary's shape would have
//!   produced 3 orphan abilities and a book whose abilities reach no screen.
//! * **Size is `FACT:BaseSize|S`, not `SIZE:S`.** These rows carry no `SIZE:`
//!   token at all, so the first transcription served an empty size for both
//!   monsters while the corpus plainly states one.
//! * **`NATURALATTACKS:` damage is not always dice.** Seru's `Venom` attack
//!   spells its damage field `Poison`. It is recorded as a named attack with no
//!   `damage_dice`, the same treatment this lane already gives the 13 Bonus
//!   Bestiary attacks the corpus never prices.
//!
//! # A fractional challenge rating, for the first time in the catalog
//!
//! `Bat (Sootwing)` carries `CR:1/2`. Every previously ingested monster row in
//! this repo carried an integer CR, so `monster_catalog`'s wire mapping parsed
//! the token with a bare `f32::from_str` and would have panicked on this row.
//! The catalog now reads the corpus's `a/b` spelling; the token stays verbatim
//! here, as [`MonsterStatBlock::challenge_rating`] requires.

mod monster_data;

pub use super::monster_chassis::{
    MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock,
    NaturalAttack, Speed,
};

/// Every monster stat block this book defines, in corpus row order.
pub const fn monsters_static() -> &'static [MonsterStatBlock] {
    monster_data::MONSTERS
}

/// Every monster-ability record this book defines, in corpus row order.
pub const fn monster_abilities_static() -> &'static [MonsterAbilityRecord] {
    monster_data::MONSTER_ABILITIES
}

/// Every monster stat block this book defines, in corpus row order.
pub fn monsters() -> &'static [MonsterStatBlock] {
    monsters_static()
}

/// Every monster-ability record this book defines, in corpus row order.
pub fn monster_abilities() -> &'static [MonsterAbilityRecord] {
    monster_abilities_static()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Both counts come from `docs/work-inventory.json`'s units for this book:
    /// `python3 -c "import json; d=json.load(open('docs/work-inventory.json'));
    /// print(sum(1 for u in d['units'] if u['book']=='monster_codex' and
    /// u['kind']=='monster'))"` -> 2, and the same for `monster_ability` -> 3.
    #[test]
    fn the_book_defines_two_monsters_and_three_abilities() {
        assert_eq!(monsters().len(), 2);
        assert_eq!(monster_abilities().len(), 3);
    }

    /// Verbatim spot-check against `mc_races.lst:6` and
    /// `mc_abilities_race.lst:71` — checkable against the named line rather
    /// than merely self-consistent. This row is also the fractional-CR case and
    /// the `FACT:BaseSize` case.
    #[test]
    fn the_sootwing_bat_matches_its_corpus_row() {
        let bat = monsters()
            .iter()
            .find(|m| m.key == "Bat (Sootwing)")
            .expect("Bat (Sootwing) is in this book");
        assert_eq!(bat.source_line, 6);
        // The first column is the display name; `KEY:` is the identity. They
        // differ on this row, which is why nothing here joins on the name.
        assert_eq!(bat.name, "Sootwing Bat");
        assert_eq!(bat.size, Some("T"));
        assert_eq!(bat.challenge_rating, Some("1/2"));
        assert_eq!(bat.monster_class, Some("Undead:2"));
        assert_eq!(bat.race_type, Some("Undead"));
        assert_eq!(bat.source_page, Some("p.88"));
        assert_eq!(
            bat.natural_attacks,
            &[NaturalAttack { name: "Bite", damage_dice: Some("1d3") }]
        );
        assert_eq!(bat.ability_keys, &["Bat (Sootwing) ~ Disease"]);

        let disease = monster_abilities()
            .iter()
            .find(|a| a.key == "Bat (Sootwing) ~ Disease")
            .expect("the namespaced key resolves");
        assert_eq!(disease.source_line, 71);
        assert_eq!(disease.name, "Disease");
        assert_eq!(disease.facet, MonsterAbilityFacet::SpecialAttack);
        assert_eq!(disease.delivery, Some(MonsterAbilityDelivery::Supernatural));
        assert_eq!(disease.owners, &["Bat (Sootwing)"]);
    }

    /// Seru's `Venom` attack is the row whose corpus damage field reads
    /// `Poison` rather than a die expression. It is a named attack with no
    /// dice, never an attack whose damage prints as the word "Poison".
    #[test]
    fn a_non_dice_damage_field_is_recorded_as_no_dice_not_as_text() {
        let seru = monsters().iter().find(|m| m.key == "Seru").expect("Seru is in this book");
        let venom = seru
            .natural_attacks
            .iter()
            .find(|a| a.name == "Venom")
            .expect("Seru's row names a Venom attack");
        assert_eq!(venom.damage_dice, None);
        let bite = seru
            .natural_attacks
            .iter()
            .find(|a| a.name == "Bite")
            .expect("Seru's row names a Bite attack");
        assert_eq!(bite.damage_dice, Some("1d6"));
    }
}
