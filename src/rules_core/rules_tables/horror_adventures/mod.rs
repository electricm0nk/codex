//! Horror Adventures (`SOURCESHORT:HA`) — `companion` + `monster` +
//! `monster_ability`.
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
mod monster_data;
/// SD-32 card 11 (T9 onboarding, `decisions.md §19` sign-off): this book's
/// third family, ingested by the shared config-driven `ingest_spells.rs`
/// pass (`decisions.md §17`) rather than a dedicated per-book binary.
pub mod spell_list;

pub use super::monster_chassis::{
    MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock,
};

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

    /// The whole REAL remainder of SD-29's monster lane, pinned.
    ///
    /// From `docs/work-inventory.json`'s own units for this book:
    /// `python3 scripts/classify_monster_ability_rows.py horror_adventures` ->
    /// `horror_adventures  3  71  0  6  65  0  0`, i.e. 3 monster rows and 6 of
    /// 71 ability rows owned by one of them. The other 65 are orphans and are
    /// pinned by line in `monster_data`'s header rather than shipped as records
    /// no screen can reach.
    #[test]
    fn the_book_defines_three_monsters_and_six_owned_abilities() {
        assert_eq!(monsters().len(), 3);
        // 6 owned + 56 owner-less (`decisions.md §20`, no_record-to-zero
        // wave 2 follow-on) = 62. The 65 orphans this book's own doc comment
        // names above did not all ship 1:1 as owner-less records: 9 of them
        // are ALSO unparseable multi-DESC: rows, excluded by the same
        // pre-existing screen that already applies to owned rows (see
        // `scripts/transcribe_monster_tables.py horror_adventures`'s own
        // stderr). The owner-less count is pinned separately below
        // (`every_owner_less_ability_is_a_named_and_pinned_non_reach`).
        // 6/62 -> 6/71 (`decisions.md §27b` round 9, +9 total, all
        // owner-less): the 9 previously-unparseable multi-DESC: rows this
        // comment names above now resolve via `parse_desc`'s new
        // generalised sixth branch -- `owned` is UNCHANGED, all 9 land in
        // the owner-less pin below.
        let owned = monster_abilities()
            .iter()
            .filter(|a| !a.owners.is_empty())
            .count();
        assert_eq!(owned, 6);
        assert_eq!(monster_abilities().len(), 71);
    }

    /// The `ABILITY:Internal|AUTOMATIC|` bundle token, read for its ATTACK
    /// segments on a monster row that carries them.
    ///
    /// `ha_races.lst:4` states Hive Queen's attacks in two places: a
    /// `NATURALATTACKS:Claw,...,*2,1d10` token, and
    /// `ABILITY:Internal|AUTOMATIC|Race Traits ~ Hive Queen|Bite|Tail Slap`,
    /// whose trailing segments are two further attacks the corpus prices
    /// nowhere. A reader of `NATURALATTACKS:` alone serves a hive queen with one
    /// attack when the corpus states three. The two undiced attacks are recorded
    /// as named attacks with no `damage_dice`, never as attacks whose damage
    /// prints as an empty string.
    #[test]
    fn the_hive_queen_carries_the_two_attacks_only_its_bundle_token_states() {
        let queen = monsters()
            .iter()
            .find(|m| m.key == "Hive Queen")
            .expect("Hive Queen is in this book");
        assert_eq!(queen.source_line, 4);
        assert_eq!(queen.size, Some("H"));
        assert_eq!(queen.challenge_rating, Some("10"));
        assert_eq!(queen.monster_class, Some("Aberration:15"));
        assert_eq!(queen.race_subtype, Some("Hive"));
        assert_eq!(queen.source_page, Some("p.236"));
        let names: Vec<&str> = queen.natural_attacks.iter().map(|a| a.name).collect();
        assert_eq!(names, vec!["Claw", "Bite", "Tail Slap"]);
        assert_eq!(queen.natural_attacks[0].damage_dice, Some("1d10"));
        assert_eq!(queen.natural_attacks[1].damage_dice, None);
        assert_eq!(queen.natural_attacks[2].damage_dice, None);
    }

    /// Every OWNED ability row this book ships reaches a monster row of this
    /// book, and every monster row's `ability_keys` resolves.
    ///
    /// **Superseded `decisions.md §20` for the owner-less half.** This used
    /// to also forbid an empty `owners` list; that forbids what the 56
    /// owner-less rows below are now correctly allowed to be — an
    /// un-ingested row's shape cannot be measured, so they SHIP with
    /// `owners: &[]` for shape measurement rather than being dropped.
    /// Reachability is not claimed for them; each is pinned by exact key in
    /// `reach_gate.rs::UNREACHED_RECORD_FINDINGS`.
    #[test]
    fn every_shipped_ability_is_owned_by_a_shipped_monster() {
        for ability in monster_abilities() {
            for owner in ability.owners {
                assert!(
                    monsters().iter().any(|m| m.key == *owner),
                    "{} names owner {owner}, which this book does not ship",
                    ability.key
                );
            }
        }
        for monster in monsters() {
            for key in monster.ability_keys {
                assert!(
                    monster_abilities().iter().any(|a| a.key == *key),
                    "{} names ability {key}, which this book does not ship",
                    monster.key
                );
            }
            assert!(
                monster.external_ability_refs.is_empty(),
                "{} names an ability outside this book",
                monster.key
            );
        }
    }

    /// **Superseded `decisions.md §20` (no_record-to-zero wave 2 follow-on).**
    /// The 56 rows no monster row of this book claims now SHIP with
    /// `owners: &[]`, and this test pins the EXACT set of records that carry
    /// one — a silent new arrival OR a silent disappearance both fail here,
    /// by name. `list_monster_catalog` never walks these directly (only a
    /// monster's own `ability_keys`), so shipping them does not surface a
    /// stub; each key is pinned separately, by name, in `reach_gate.rs::
    /// UNREACHED_RECORD_FINDINGS` under
    /// `("horror_adventures", "monster_abilities")` as a proven non-reach,
    /// not a silent claim of reachability.
    #[test]
    fn every_owner_less_ability_is_a_named_and_pinned_non_reach() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut unowned: Vec<&str> = monster_abilities()
            .iter()
            .filter(|a| a.owners.is_empty())
            .map(|a| a.key)
            .collect();
        unowned.sort_unstable();

        // 56 -> 65 (`decisions.md §27b` round 9, +9): the 9 previously-
        // unparseable multi-DESC: rows close, all owner-less, see the test
        // above.
        assert_eq!(
            unowned.len(),
            65,
            "the number of owner-less (unreachable-by-design) monster_ability records \
             changed — re-derive this pin from a real \
             `scripts/transcribe_monster_tables.py horror_adventures` run, and update the \
             matching `reach_gate.rs::UNREACHED_RECORD_FINDINGS` entry to the same key set"
        );

        let mut hasher = DefaultHasher::new();
        unowned.hash(&mut hasher);
        let digest = hasher.finish();
        assert_eq!(
            digest, 0x941a_2132_e655_2505,
            "the owner-less key SET changed (same count, different members) — re-derive and \
             update `reach_gate.rs::UNREACHED_RECORD_FINDINGS` to match exactly. \
             0x4db7998b_4652eb60 -> 0x941a2132_e6552505 (`decisions.md §27b` round 9): the \
             set gains 9 new members (the 9 previously-unparseable multi-DESC: rows), \
             re-derived live from this test's own failing run, never guessed, per \
             `decisions.md §17a`."
        );
    }

    /// The two `DESC:` formula shapes, both present in this book's six rows.
    /// `Hive Warrior ~ Acid Spit` carries TWO `%n` slots against two variables;
    /// `Hive Queen ~ Egg Layer` carries none. The variables stay verbatim —
    /// `decisions.md §24` rules out the formula interpreter that would resolve
    /// them, so the record carries the corpus's own expression.
    #[test]
    fn description_variables_are_carried_verbatim_and_positionally() {
        let spit = monster_abilities()
            .iter()
            .find(|a| a.key == "Hive Warrior ~ Acid Spit")
            .expect("the namespaced key resolves");
        assert_eq!(spit.source_line, 285);
        assert_eq!(spit.facet, MonsterAbilityFacet::SpecialAttack);
        assert_eq!(spit.delivery, Some(MonsterAbilityDelivery::Extraordinary));
        assert_eq!(spit.description_variables, &["HD", "10+HD/2+DEX"]);

        let eggs = monster_abilities()
            .iter()
            .find(|a| a.key == "Hive Queen ~ Egg Layer")
            .expect("the namespaced key resolves");
        assert_eq!(eggs.facet, MonsterAbilityFacet::SpecialQuality);
        assert!(eggs.description_variables.is_empty());
    }
}
