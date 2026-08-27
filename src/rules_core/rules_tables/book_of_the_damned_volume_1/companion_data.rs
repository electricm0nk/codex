//! book_of_the_damned_volume_1 companion tables, transcribed verbatim from the book's own
//! PCGen `.lst` rows.
//!
//! GENERATED FILE -- do not hand-edit. Regenerate with
//! `python3 scripts/transcribe_companion_tables.py book_of_the_damned_volume_1`, whose unit set is
//! `docs/work-inventory.json`'s own units for this book rather than a raw
//! line count over the `.lst`.
//!
//! Sources, with the file AND line each record was read from carried per row:
//!   * `botd1_races_companion.lst` -- 1 companion creature rows
//!   * `botd1_abilities_companion.lst` -- 1 companion ability rows
//!
//! NOT transcribed -- ability rows no creature row of this book owns, so
//! nothing could ever reach them on screen. Dropped rather than emitted
//! unreachable (`decisions.md §50`, adopted from the monster lane; §56.1).
//! These rows keep their `engine-does-not-hold` status in
//! `docs/work-inventory.json`, which is where the shortfall is counted; they
//! are NOT a `reach_gate` `OPEN_FINDINGS` entry, because that list is keyed by
//! FAMILY and this book's `companions` family does reach a player:
//!   * `Imp Companion Trick ~ Alternate Form (Boar)`
//!   * `Imp Companion Trick ~ Alternate Form (Rat)`
//!   * `Imp Companion Trick ~ Alternate Form (Raven)`
//!   * `Imp Companion Trick ~ Alternate Form (Young Giant Spider)`
//!   * `Imp Companion Trick ~ Spell-like Ability 1/day (Curse Water)`
//!   * `Imp Companion Trick ~ Spell-like Ability 1/day (Floating Disk)`
//!   * `Imp Companion Trick ~ Spell-like Ability 1/day (Grease)`
//!   * `Imp Companion Trick ~ Spell-like Ability 1/day (Hold Portal)`
//!   * `Imp Companion Trick ~ Spell-like Ability 1/day (Identify)`
//!   * `Imp Companion Trick ~ Spell-like Ability 1/day (Silent Image)`
//!   * `Imp Companion Trick ~ Spell-like Ability 1/day (Unseen Servant)`
//!   * `Imp Companion Trick ~ Spell-like Ability 1/day (Ventriloquism)`
//!   * `Imp Companion Trick ~ Spell-like Ability At-will (Bleed)`
//!   * `Imp Companion Trick ~ Spell-like Ability At-will (Deathwatch)`
//!   * `Imp Companion Trick ~ Spell-like Ability At-will (Detect Evil)`
//!   * `Imp Companion Trick ~ Spell-like Ability At-will (Detect Law)`
//!   * `Imp Companion Trick ~ Spell-like Ability At-will (Doom)`
//!   * `Imp Companion Trick ~ Spell-like Ability At-will (Ghost Sound)`
//!   * `Imp Companion Trick ~ Spell-like Ability At-will (Mage Hand)`
//!   * `Imp Companion Trick ~ Spell-like Ability At-will (Message)`
//!   * `Imp Companion Trick ~ Spell-like Ability At-will (Open/Close)`
//!   * `Imp Companion Trick ~ Spell-like Ability At-will (Prestidigitation)`
//!   * `Imp Companion Trick ~ Telepathy`
//!   * `Imp Companion ~ Bonus Tricks`
//!   * `Imp Companion ~ Link`
//!   * `Imp Companion ~ Share Spells`
//!   * `Imp Companion ~ Starting Shape Change`
//!
//! NOT transcribed -- `*_classes_companion.lst` CLASS rows (`decisions.md
//! §65.1`). A PCGen monster class is the hit-dice progression a creature
//! row's `MONSTERCLASS:` token names -- it states no `SIZE:`, no `MOVE:` and
//! no natural attacks, so every field this chassis models transcribes empty.
//! Modelling it is a new record type (a level progression table), not a wider
//! predicate on this one. Left honestly `engine-does-not-hold`; the creature rows that
//! name them ship, and carry the token verbatim:
//!   * `1`
//!   * `Imp Companion`

use crate::rules_core::rules_tables::companion_chassis::{CompanionAbilityDelivery, CompanionAbilityFacet, CompanionAbilityRecord, CompanionRecord, NaturalAttack, Speed, StatAdjustment};

/// Every book_of_the_damned_volume_1 companion creature (1 rows).
pub(super) static COMPANIONS: &[CompanionRecord] = &[
    CompanionRecord {
        key: "Companion (Imp)",
        name: "Companion (Imp)",
        size: Some("T"),
        speeds: &[Speed { mode: "Walk", feet: 20 }, Speed { mode: "Fly", feet: 50 }],
        reach_feet: Some(0),
        race_type: Some("Outsider"),
        race_subtype: Some("Devil|Lawful|Evil"),
        monster_class: Some("Imp Companion:2"),
        type_segments: &[],
        natural_attacks: &[NaturalAttack { name: "Sting", damage_dice: Some("1d4") }],
        natural_attack_damage_bonuses: &[],
        skill_ability_diff_bonuses: &[],
        stat_adjustments: &[StatAdjustment { ability: "DEX", amount: 7 }, StatAdjustment { ability: "INT", amount: 3 }, StatAdjustment { ability: "WIS", amount: 2 }, StatAdjustment { ability: "CHA", amount: 4 }],
        natural_armor: Some(1),
        source_page: Some("p.78"),
        ability_keys: &["Imp Companion ~ Poison"],
        external_ability_refs: &["Detect Good ~ Constant", "Detect Magic ~ Constant", "Flight Maneuverability"],
        source_file: "botd1_races_companion.lst",
        source_line: 6,
    },
];

/// Every book_of_the_damned_volume_1 companion ability record (1 rows).
pub(super) static COMPANION_ABILITIES: &[CompanionAbilityRecord] = &[
    CompanionAbilityRecord {
        key: "Imp Companion ~ Poison",
        name: "Poison",
        facet: Some(CompanionAbilityFacet::SpecialAttack),
        delivery: Some(CompanionAbilityDelivery::Extraordinary),
        type_segments: &["SpecialAttack", "Extraordinary"],
        description: Some("Sting - injury; save Fort DC %1; frequency 1/round for 5 minutes; effect 1d2 Dex; cure 1 save."),
        description_variables: &["ImpCompPoisonDC"],
        description_variants: &[],
        stat_adjustments: &[],
        source_page: Some("p.78"),
        owners: &["Companion (Imp)"],
        source_file: "botd1_abilities_companion.lst",
        source_line: 8,
    },
];
