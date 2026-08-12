//! Horror Adventures (`SOURCESHORT:HA`) — `companion`.
//!
//! # The second family this book contributes, and the first compiled one
//!
//! `RuleSetId::Ha` was added by SD-29's race-trait lane round 3, which ingested
//! this book's 43 `race_trait` records **off disk** from
//! `data/corpus/horror_adventures/race_trait/` — `decisions.md §24` rules out
//! the formula interpreter a compiled race-trait table would need. Its
//! `companion` rows are the opposite shape: verbatim creature and ability tokens
//! with no formula to interpret, so they compile, and this is the first table
//! this book has in the engine.
//!
//! # The whole book is one companion and its advancement
//!
//! ```text
//! python3 scripts/classify_companion_rows.py horror_adventures
//! book                              crea  abil  clas  named  prerace  prefix  ORPHAN
//! horror_adventures                    1     1     0      1        1       0       0
//! ```
//!
//! Both ownership shapes fire on the same pair — the creature row names the
//! advancement outright *and* the advancement's `PRERACE:` names the creature —
//! which is what makes this book the cheapest possible confirmation that the two
//! shapes agree when both are present. They are recorded once, not twice: the
//! chassis dedupes on the key.

mod companion_data;

pub use super::companion_chassis::{
    CompanionAbilityDelivery, CompanionAbilityFacet, CompanionAbilityRecord, CompanionRecord,
    NaturalAttack, Speed, StatAdjustment,
};

/// Every companion creature this book defines, in corpus row order.
pub const fn companions_static() -> &'static [CompanionRecord] {
    companion_data::COMPANIONS
}

/// Every companion ability record this book defines, in corpus row order.
pub const fn companion_abilities_static() -> &'static [CompanionAbilityRecord] {
    companion_data::COMPANION_ABILITIES
}

/// Every companion creature this book defines, in corpus row order.
pub fn companions() -> &'static [CompanionRecord] {
    companions_static()
}

/// Every companion ability record this book defines, in corpus row order.
pub fn companion_abilities() -> &'static [CompanionAbilityRecord] {
    companion_abilities_static()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// From `docs/work-inventory.json`'s own units for this book: 2 companion
    /// units, one creature and one ability.
    #[test]
    fn the_book_defines_one_companion_and_one_ability() {
        assert_eq!(companions().len(), 1);
        assert_eq!(companion_abilities().len(), 1);
    }

    /// Verbatim spot-check against `ha_races_companion.lst:3`, including the
    /// `Climb` speed — a mode a land-speed-only reader would have dropped.
    #[test]
    fn the_devolved_humanoid_matches_its_corpus_row() {
        let companion = &companions()[0];
        assert_eq!(companion.key, "Companion (Devolved Humanoid)");
        assert_eq!(companion.source_line, 3);
        assert_eq!(companion.size, Some("M"));
        assert_eq!(companion.monster_class, Some("Companion:2"));
        assert_eq!(companion.source_page, Some("p.50"));
        assert_eq!(companion.type_segments, &["Companion", "AnimalCompanion"]);
        assert_eq!(
            companion.speeds,
            &[
                Speed { mode: "Walk", feet: 30 },
                Speed { mode: "Climb", feet: 30 },
            ]
        );
    }

    /// Both ownership shapes fire on this pair, and the link is recorded once.
    /// A chassis that appended per shape would list the advancement twice on the
    /// creature and name the creature twice on the advancement.
    #[test]
    fn the_two_ownership_shapes_agree_and_are_recorded_once() {
        let companion = &companions()[0];
        assert_eq!(companion.ability_keys, &["Companion Advancement ~ Devolved Humanoid"]);
        let advancement = &companion_abilities()[0];
        assert_eq!(advancement.owners, &["Companion (Devolved Humanoid)"]);
    }
}
