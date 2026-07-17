//! PF1 CRB race trait tables — one row per named racial trait dimension per
//! race.
//!
//! Values and corpus citations are transcribed verbatim from the already-
//! grounded per-race explanation seams in `pilot_compute.rs`
//! (`explain_human_pilot_race_seam`, `explain_human_trait_bundle`,
//! `explain_dwarf_race_seam`, `explain_elf_race_seam`,
//! `explain_gnome_race_seam`, `explain_half_elf_race_seam`,
//! `explain_half_orc_race_seam`, `explain_halfling_race_seam`) — not
//! re-derived from memory. Coverage is bounded to exactly the dimensions
//! those seams already ground (ability modifiers/bonus, size, speed, senses,
//! and each race's named special traits); no additional racial trait family
//! is fabricated for this table.
//!
//! The two ability-bonus dimensions that are player choices rather than
//! fixed values (Human's ability-bonus target and bonus feat; Half-Elf's and
//! Half-Orc's ability-bonus target) carry `value: 0` and a detail noting the
//! choice explicitly, mirroring how `pilot_compute.rs` treats them as
//! recognition-only records rather than fabricating a specific selection.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RaceId {
    Human,
    Dwarf,
    Elf,
    Gnome,
    HalfElf,
    HalfOrc,
    Halfling,
}

impl RaceId {
    pub const ALL: &'static [RaceId] = &[
        RaceId::Human,
        RaceId::Dwarf,
        RaceId::Elf,
        RaceId::Gnome,
        RaceId::HalfElf,
        RaceId::HalfOrc,
        RaceId::Halfling,
    ];
}

pub struct RaceTraitEntry {
    pub race_id: RaceId,
    pub trait_name: &'static str,
    pub value: i16,
    pub detail: &'static str,
}

/// Mirrors the exact per-race explanation-record set each race's own seam in
/// `pilot_compute.rs` grounds as of the SD-19 Full-matrix-closure sweep
/// (2026-07-16): Human 6, Dwarf 9, Elf 7, Gnome 8, Half-Elf 6, Half-Orc 5,
/// Halfling 8 (49 total).
const RACE_TRAITS: &[RaceTraitEntry] = &[
    // ----- Human (6) -----
    RaceTraitEntry {
        race_id: RaceId::Human,
        trait_name: "Ability Bonus",
        value: 0,
        detail: "Player-chosen +2 to any one ability score (cr_races.lst race:human, choice:human_ability_bonus). Player choice, not a fixed pair.",
    },
    RaceTraitEntry {
        race_id: RaceId::Human,
        trait_name: "Bonus Feat",
        value: 0,
        detail: "One bonus feat at 1st level, selected from any feat whose prerequisites are met (cr_races.lst race:human, choice:human_bonus_feat). Player choice.",
    },
    RaceTraitEntry {
        race_id: RaceId::Human,
        trait_name: "Size",
        value: 0,
        detail: "Medium size (cr_races.lst race:human SIZE:MEDIUM).",
    },
    RaceTraitEntry {
        race_id: RaceId::Human,
        trait_name: "Speed",
        value: 30,
        detail: "30 ft base land speed (cr_races.lst race:human GAIT:WALK|30).",
    },
    RaceTraitEntry {
        race_id: RaceId::Human,
        trait_name: "Senses",
        value: 0,
        detail: "No special senses (cr_races.lst race:human carries no SENSE tag).",
    },
    RaceTraitEntry {
        race_id: RaceId::Human,
        trait_name: "Extra Skill Ranks",
        value: 1,
        detail: "4 extra skill points at 1st level, +1 extra skill rank per level thereafter (cr_races.lst race:human BONUS:SKILL).",
    },
    // ----- Dwarf (9) -----
    RaceTraitEntry {
        race_id: RaceId::Dwarf,
        trait_name: "Ability Modifiers",
        value: 0,
        detail: "+2 Constitution / -2 Charisma (cr_races.lst race:dwarf STAT:CON|+2, STAT:CHA|-2).",
    },
    RaceTraitEntry {
        race_id: RaceId::Dwarf,
        trait_name: "Size",
        value: 0,
        detail: "Medium size (cr_races.lst race:dwarf SIZE:MEDIUM).",
    },
    RaceTraitEntry {
        race_id: RaceId::Dwarf,
        trait_name: "Speed",
        value: 20,
        detail: "20 ft base land speed, never reduced by armor or encumbrance (cr_races.lst race:dwarf GAIT:WALK|20).",
    },
    RaceTraitEntry {
        race_id: RaceId::Dwarf,
        trait_name: "Senses",
        value: 60,
        detail: "Darkvision 60 ft (cr_races.lst race:dwarf SENSE:Darkvision (60 ft)).",
    },
    RaceTraitEntry {
        race_id: RaceId::Dwarf,
        trait_name: "Stonecunning",
        value: 2,
        detail: "+2 Perception to potentially notice unusual stonework, such as traps and hidden doors (dwarf_abilities_race.lst).",
    },
    RaceTraitEntry {
        race_id: RaceId::Dwarf,
        trait_name: "Greed",
        value: 2,
        detail: "+2 Appraise checks to determine the price of nonmagical goods containing precious metals or gemstones (dwarf_abilities_race.lst).",
    },
    RaceTraitEntry {
        race_id: RaceId::Dwarf,
        trait_name: "Hardy",
        value: 2,
        detail: "+2 saving throws against poison, and +2 against spells and spell-like abilities (dwarf_abilities_race.lst).",
    },
    RaceTraitEntry {
        race_id: RaceId::Dwarf,
        trait_name: "Stability",
        value: 4,
        detail: "+4 Combat Maneuver Defense when resisting a bull rush or trip attempt while standing on the ground (dwarf_abilities_race.lst).",
    },
    RaceTraitEntry {
        race_id: RaceId::Dwarf,
        trait_name: "Defensive Training",
        value: 4,
        detail: "+4 dodge bonus to Armor Class against monsters of the giant subtype (dwarf_abilities_race.lst).",
    },
    // ----- Elf (7) -----
    RaceTraitEntry {
        race_id: RaceId::Elf,
        trait_name: "Ability Modifiers",
        value: 0,
        detail: "+2 Dexterity / -2 Constitution (cr_races.lst race:elf STAT:DEX|+2, STAT:CON|-2). The alternate +2 Intelligence Elf variant is out of scope.",
    },
    RaceTraitEntry {
        race_id: RaceId::Elf,
        trait_name: "Size",
        value: 0,
        detail: "Medium size (cr_races.lst race:elf SIZE:MEDIUM).",
    },
    RaceTraitEntry {
        race_id: RaceId::Elf,
        trait_name: "Speed",
        value: 30,
        detail: "30 ft base land speed (cr_races.lst race:elf GAIT:WALK|30).",
    },
    RaceTraitEntry {
        race_id: RaceId::Elf,
        trait_name: "Senses",
        value: 0,
        detail: "Low-Light Vision (cr_races.lst race:elf SENSE:Low-Light Vision).",
    },
    RaceTraitEntry {
        race_id: RaceId::Elf,
        trait_name: "Keen Senses",
        value: 2,
        detail: "+2 Perception skill checks (elf_abilities_race.lst).",
    },
    RaceTraitEntry {
        race_id: RaceId::Elf,
        trait_name: "Elven Immunities",
        value: 2,
        detail: "Immune to magic sleep effects; +2 saving throws against enchantment spells and effects (elf_abilities_race.lst).",
    },
    RaceTraitEntry {
        race_id: RaceId::Elf,
        trait_name: "Elven Magic",
        value: 2,
        detail: "+2 caster level checks to overcome spell resistance; +2 Spellcraft checks to identify magic item properties (elf_abilities_race.lst).",
    },
    // ----- Gnome (8) -----
    RaceTraitEntry {
        race_id: RaceId::Gnome,
        trait_name: "Ability Modifiers",
        value: 0,
        detail: "+2 Constitution / -2 Strength (cr_races.lst race:gnome STAT:CON|+2, STAT:STR|-2).",
    },
    RaceTraitEntry {
        race_id: RaceId::Gnome,
        trait_name: "Size",
        value: 0,
        detail: "Small size (cr_races.lst race:gnome SIZE:SMALL).",
    },
    RaceTraitEntry {
        race_id: RaceId::Gnome,
        trait_name: "Speed",
        value: 20,
        detail: "20 ft base land speed (cr_races.lst race:gnome GAIT:WALK|20).",
    },
    RaceTraitEntry {
        race_id: RaceId::Gnome,
        trait_name: "Senses",
        value: 0,
        detail: "Low-Light Vision (cr_races.lst race:gnome SENSE:Low-Light Vision).",
    },
    RaceTraitEntry {
        race_id: RaceId::Gnome,
        trait_name: "Keen Senses",
        value: 2,
        detail: "+2 Perception skill checks (gnome_abilities_race.lst).",
    },
    RaceTraitEntry {
        race_id: RaceId::Gnome,
        trait_name: "Illusion Resistance",
        value: 2,
        detail: "+2 saving throws against illusion spells and effects (gnome_abilities_race.lst).",
    },
    RaceTraitEntry {
        race_id: RaceId::Gnome,
        trait_name: "Defensive Training",
        value: 4,
        detail: "+4 dodge bonus to Armor Class against monsters of the giant subtype (gnome_abilities_race.lst).",
    },
    RaceTraitEntry {
        race_id: RaceId::Gnome,
        trait_name: "Hatred",
        value: 1,
        detail: "+1 attack rolls against humanoid creatures of the reptilian and goblinoid subtypes (gnome_abilities_race.lst).",
    },
    // ----- Half-Elf (6) -----
    RaceTraitEntry {
        race_id: RaceId::HalfElf,
        trait_name: "Ability Bonus",
        value: 0,
        detail: "Player-chosen +2 to any one ability score (choice:half_elf_ability_bonus). Player choice, like Human.",
    },
    RaceTraitEntry {
        race_id: RaceId::HalfElf,
        trait_name: "Size",
        value: 0,
        detail: "Medium size (cr_races.lst race:half-elf SIZE:MEDIUM).",
    },
    RaceTraitEntry {
        race_id: RaceId::HalfElf,
        trait_name: "Speed",
        value: 30,
        detail: "30 ft base land speed (cr_races.lst race:half-elf GAIT:WALK|30).",
    },
    RaceTraitEntry {
        race_id: RaceId::HalfElf,
        trait_name: "Senses",
        value: 0,
        detail: "Low-Light Vision (cr_races.lst race:half-elf SENSE:Low-Light Vision).",
    },
    RaceTraitEntry {
        race_id: RaceId::HalfElf,
        trait_name: "Keen Senses",
        value: 2,
        detail: "+2 Perception skill checks (halfelf_abilities_race.lst).",
    },
    RaceTraitEntry {
        race_id: RaceId::HalfElf,
        trait_name: "Elven Immunities",
        value: 2,
        detail: "Immune to magic sleep effects; +2 saving throws against enchantment spells and effects (halfelf_abilities_race.lst).",
    },
    // ----- Half-Orc (5) -----
    RaceTraitEntry {
        race_id: RaceId::HalfOrc,
        trait_name: "Ability Bonus",
        value: 0,
        detail: "Player-chosen +2 to any one ability score (choice:half_orc_ability_bonus). Player choice, like Half-Elf.",
    },
    RaceTraitEntry {
        race_id: RaceId::HalfOrc,
        trait_name: "Size",
        value: 0,
        detail: "Medium size (cr_races.lst race:half-orc SIZE:MEDIUM).",
    },
    RaceTraitEntry {
        race_id: RaceId::HalfOrc,
        trait_name: "Speed",
        value: 30,
        detail: "30 ft base land speed (cr_races.lst race:half-orc GAIT:WALK|30).",
    },
    RaceTraitEntry {
        race_id: RaceId::HalfOrc,
        trait_name: "Senses",
        value: 60,
        detail: "Darkvision 60 ft (cr_races.lst race:half-orc SENSE:Darkvision (60 ft)).",
    },
    RaceTraitEntry {
        race_id: RaceId::HalfOrc,
        trait_name: "Intimidating",
        value: 2,
        detail: "+2 Intimidate skill checks (halforc_abilities_race.lst).",
    },
    // ----- Halfling (8) -----
    RaceTraitEntry {
        race_id: RaceId::Halfling,
        trait_name: "Ability Modifiers",
        value: 0,
        detail: "+2 Dexterity / -2 Strength (cr_races.lst race:halfling STAT:DEX|+2, STAT:STR|-2).",
    },
    RaceTraitEntry {
        race_id: RaceId::Halfling,
        trait_name: "Size",
        value: 0,
        detail: "Small size (cr_races.lst race:halfling SIZE:SMALL).",
    },
    RaceTraitEntry {
        race_id: RaceId::Halfling,
        trait_name: "Speed",
        value: 20,
        detail: "20 ft base land speed (cr_races.lst race:halfling GAIT:WALK|20).",
    },
    RaceTraitEntry {
        race_id: RaceId::Halfling,
        trait_name: "Senses",
        value: 0,
        detail: "No special senses (cr_races.lst race:halfling carries no SENSE tag).",
    },
    RaceTraitEntry {
        race_id: RaceId::Halfling,
        trait_name: "Keen Senses",
        value: 2,
        detail: "+2 Perception skill checks (halfling_abilities_race.lst).",
    },
    RaceTraitEntry {
        race_id: RaceId::Halfling,
        trait_name: "Sure-Footed",
        value: 2,
        detail: "+2 Acrobatics and Climb skill checks (halfling_abilities_race.lst).",
    },
    RaceTraitEntry {
        race_id: RaceId::Halfling,
        trait_name: "Fearless",
        value: 2,
        detail: "+2 saving throws against fear (halfling_abilities_race.lst).",
    },
    RaceTraitEntry {
        race_id: RaceId::Halfling,
        trait_name: "Halfling Luck",
        value: 1,
        detail: "+1 on all saving throws; stacks with Fearless per the corpus DESC text (halfling_abilities_race.lst).",
    },
];

pub fn race_traits() -> &'static [RaceTraitEntry] {
    RACE_TRAITS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_race_has_its_grounded_trait_count() {
        let count = |race: RaceId| race_traits().iter().filter(|t| t.race_id == race).count();
        assert_eq!(count(RaceId::Human), 6);
        assert_eq!(count(RaceId::Dwarf), 9);
        assert_eq!(count(RaceId::Elf), 7);
        assert_eq!(count(RaceId::Gnome), 8);
        assert_eq!(count(RaceId::HalfElf), 6);
        assert_eq!(count(RaceId::HalfOrc), 5);
        assert_eq!(count(RaceId::Halfling), 8);
        assert_eq!(race_traits().len(), 49);
    }

    #[test]
    fn every_entry_has_a_non_empty_name_and_detail() {
        for entry in race_traits() {
            assert!(!entry.trait_name.is_empty());
            assert!(!entry.detail.is_empty());
        }
    }
}
