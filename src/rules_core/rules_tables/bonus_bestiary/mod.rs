//! Bonus Bestiary (`SOURCESHORT:BB`) — the merged monster / monster-ability
//! chassis, and the first book in this repo to ingest `monster_ability` at all.
//!
//! # Why the two kinds live in one module
//!
//! `docs/release/corpus-work-channels.md §9.2` is the operator ruling this
//! module implements: a monster is playable, so `monster` is a chassis kind and
//! `monster_ability` is the features kind attached to it — the same shape
//! `race` + `race_trait` already have, not two independent kinds that happen to
//! sort next to each other. The link is carried on the chassis
//! ([`MonsterStatBlock::ability_keys`]) because that is where the corpus itself
//! carries it: the monster's own `ABILITY:Special Ability|AUTOMATIC|<key>|…`
//! token names its abilities, and the ability row names no owner.
//!
//! # Identity is the `KEY:` token
//!
//! 6 of this book's 17 ability rows carry a namespaced `KEY:`
//! (`Caryatid Column ~ Immunity to Magic`) whose display name is the bare leaf.
//! Joining on the display name would merge `Immunity to Magic` with any other
//! book's rule of the same name, which is precisely the `key-differs-from-name`
//! finding `v06_corpus_trap_report -- bonus_bestiary` reports for this book.
//! Every lookup here is keyed on `key`.
//!
//! # What is deliberately absent, and why that is a corpus fact
//!
//! * **AC / HP / saves** — not tokens on the monster row. PCGen computes them
//!   at runtime from `MONSTERCLASS:` hit dice and ability-score modifiers.
//!   `monster_class` carries the real token instead of a fabricated total.
//! * **Damage dice for 13 of the book's 14 natural attacks** — those attacks
//!   are named by a bare `ABILITY:Internal|AUTOMATIC|<name>` cross-reference
//!   that resolves to no dice anywhere in this book, so
//!   [`NaturalAttack::damage_dice`] is `None` for them and the catalog prints
//!   the attack's name alone. Bestiary 1's ingest closed the same gap by
//!   grounding dice from published text under `SD-26 decisions.md §11.5`; this
//!   pilot does not, because inventing a value the corpus does not carry is the
//!   one thing this bundle's loop instruction forbids outright. The gap is
//!   recorded as such rather than filled.
//! * **Abilities defined in another book** — a monster's
//!   `ABILITY:Special Ability` list also names universal monster rules (`Grab`,
//!   `Scent`, `Pounce`, …) that Bonus Bestiary references but does not define.
//!   Those references are kept verbatim in
//!   [`MonsterStatBlock::external_ability_refs`] so the count of what this book
//!   *defines* (17) never silently absorbs what it merely *cites*.

mod monster_data;

pub use super::monster_chassis::{
    MonsterAbilityDelivery, MonsterAbilityFacet, MonsterAbilityRecord, MonsterStatBlock,
    NaturalAttack, Speed,
};

/// Every monster stat block this book defines, in corpus row order.
///
/// `const` so `monster_chassis::MONSTER_BOOKS` can name it in a `const` item;
/// [`monsters`] is the same value for callers that want a plain function.
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

/// The stat block with this corpus key, if this book defines one.
pub fn monster_resolve(key: &str) -> Option<&'static MonsterStatBlock> {
    monsters().iter().find(|m| m.key == key)
}

/// The ability record with this corpus key, if this book defines one.
pub fn monster_ability_resolve(key: &str) -> Option<&'static MonsterAbilityRecord> {
    monster_abilities().iter().find(|a| a.key == key)
}

/// The abilities a monster holds, resolved through its own `ability_keys`.
pub fn abilities_of(monster: &MonsterStatBlock) -> Vec<&'static MonsterAbilityRecord> {
    monster
        .ability_keys
        .iter()
        .filter_map(|key| monster_ability_resolve(key))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Both counts are the ones re-derived from the corpus this cycle:
    /// `awk -F'\t' '!/^#/ && !/^SOURCELONG/ && NF>0' bb_races.lst | wc -l` -> 14
    /// and the same over `bb_abilities_race.lst` -> 17.
    #[test]
    fn the_book_defines_fourteen_monsters_and_seventeen_abilities() {
        assert_eq!(monsters().len(), 14);
        assert_eq!(monster_abilities().len(), 17);
    }

    #[test]
    fn every_key_is_unique_across_both_kinds() {
        let mut monster_keys: Vec<_> = monsters().iter().map(|m| m.key).collect();
        monster_keys.sort_unstable();
        let before = monster_keys.len();
        monster_keys.dedup();
        assert_eq!(monster_keys.len(), before, "monster keys must be unique");

        let mut ability_keys: Vec<_> = monster_abilities().iter().map(|a| a.key).collect();
        ability_keys.sort_unstable();
        let before = ability_keys.len();
        ability_keys.dedup();
        assert_eq!(ability_keys.len(), before, "ability keys must be unique");
    }

    /// The trap this book's trap report names: 6 rows whose `KEY:` differs from
    /// the display name. A lookup keyed on the name would find the wrong record
    /// (or none), so the identity has to be the key.
    #[test]
    fn namespaced_ability_keys_resolve_by_key_and_not_by_display_name() {
        let namespaced: Vec<_> = monster_abilities()
            .iter()
            .filter(|a| a.key != a.name)
            .collect();
        assert_eq!(namespaced.len(), 6, "bb_abilities_race.lst carries 6 `KEY:` tokens");

        let immunity = monster_ability_resolve("Caryatid Column ~ Immunity to Magic")
            .expect("the namespaced key resolves");
        assert_eq!(immunity.name, "Immunity to Magic");
        assert!(
            monster_ability_resolve("Immunity to Magic").is_none(),
            "the bare leaf is not an identity in this book"
        );
    }

    /// Every ability key a monster row names is either defined here or recorded
    /// as an external reference — the two lists together are the row's whole
    /// `ABILITY:Special Ability` token, with nothing dropped.
    #[test]
    fn every_ability_key_a_monster_names_resolves() {
        for monster in monsters() {
            for key in monster.ability_keys {
                assert!(
                    monster_ability_resolve(key).is_some(),
                    "{} names ability {key:?}, which this book does not define — it belongs in \
                     external_ability_refs",
                    monster.name
                );
            }
            for key in monster.external_ability_refs {
                assert!(
                    monster_ability_resolve(key).is_none(),
                    "{} lists {key:?} as external, but this book defines it",
                    monster.name
                );
            }
        }
    }

    /// Every ability is owned by at least one monster in this book. An orphan
    /// would mean the chassis link was transcribed wrong in one direction.
    #[test]
    fn every_ability_is_owned_by_a_monster_in_this_book() {
        for ability in monster_abilities() {
            assert!(
                !ability.owners.is_empty(),
                "{} ({}) is owned by no monster row",
                ability.name,
                ability.key
            );
            for owner in ability.owners {
                let monster = monster_resolve(owner).expect("an owner is a monster in this book");
                assert!(monster.ability_keys.contains(&ability.key));
            }
        }
    }

    /// Verbatim spot-checks against `bb_abilities_race.lst:6` and
    /// `bb_races.lst:6` — the transcription is checkable against the named
    /// line, not merely self-consistent.
    #[test]
    fn allip_and_its_babble_ability_match_their_corpus_rows() {
        let allip = monster_resolve("Allip").expect("Allip is in this book");
        assert_eq!(allip.source_line, 6);
        assert_eq!(allip.size, Some("M"));
        assert_eq!(allip.speeds, &[Speed { mode: "Fly", feet: 30 }]);
        assert_eq!(allip.race_type, Some("Undead"));
        assert_eq!(allip.race_subtype, Some("Incorporeal"));
        assert_eq!(allip.challenge_rating, Some("3"));
        assert_eq!(allip.monster_class, Some("Undead:4"));
        assert_eq!(allip.source_page, Some("p.4"));
        assert_eq!(
            allip.natural_attacks,
            &[NaturalAttack { name: "Incorporeal touch", damage_dice: Some("0") }]
        );

        let babble = monster_ability_resolve("Babble").expect("Babble is in this book");
        assert_eq!(babble.source_line, 6);
        assert_eq!(babble.facet, MonsterAbilityFacet::SpecialAttack);
        assert_eq!(babble.delivery, Some(MonsterAbilityDelivery::Supernatural));
        assert_eq!(babble.traits, &["Aura"]);
        assert_eq!(babble.description_variables, &["BabbleDC"]);
        assert!(babble
            .description
            .expect("Babble carries DESC text")
            .starts_with("An allip constantly mutters and whines to itself"));
        assert_eq!(babble.owners, &["Allip"]);
    }

    /// The one row in this book with no `DESC:` at all. Recorded as `None` so a
    /// consumer can say "the corpus carries no text" rather than printing an
    /// empty string that looks like a rendering bug.
    #[test]
    fn the_single_description_less_ability_is_recorded_as_such() {
        let without: Vec<_> = monster_abilities()
            .iter()
            .filter(|a| a.description.is_none())
            .map(|a| a.key)
            .collect();
        assert_eq!(without, vec!["Magic Circle against Evil"]);
    }

    /// 13 of the 14 natural attacks this book names carry no die expression
    /// anywhere in the corpus. The table says so rather than inventing one.
    ///
    /// The denominator was first written as 15 and this test caught it. The
    /// re-derived anatomy: 11 of the 14 rows carry an
    /// `ABILITY:Internal|AUTOMATIC|` list naming **13** attacks between them,
    /// plus Allip's single `NATURALATTACKS:` token = **14**. `Caryatid Column`
    /// and `Nixie` name no natural attack at all.
    #[test]
    fn natural_attacks_without_corpus_dice_are_none_not_a_placeholder() {
        let all: Vec<_> = monsters().iter().flat_map(|m| m.natural_attacks.iter()).collect();
        assert_eq!(all.len(), 14);
        assert_eq!(all.iter().filter(|a| a.damage_dice.is_some()).count(), 1);
        for attack in &all {
            assert!(!attack.name.trim().is_empty());
            if let Some(dice) = attack.damage_dice {
                assert!(!dice.trim().is_empty());
            }
        }
    }

    #[test]
    fn abilities_of_resolves_the_chassis_link() {
        let caryatid = monster_resolve("Caryatid Column").expect("in this book");
        let names: Vec<_> = abilities_of(caryatid).iter().map(|a| a.name).collect();
        assert_eq!(names, vec!["Immunity to Magic", "Shatter Weapons", "Statue"]);
    }
}
