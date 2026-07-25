//! PF1 CRB Druid spell list — per-class spell-level overrides.
//!
//! Mirrors `cleric_spell_list.rs`'s own doc comment exactly, substituting
//! Druid: `crb::spell_list::SPELL_LIST`'s `level` field is the MINIMUM
//! spell level across every class named in the corpus's `CLASSES:` tag
//! for that record, not necessarily the Druid-specific level. This table
//! re-parses the same corpus record's raw `CLASSES:` tag directly
//! (`core_rulebook/cr_spells.lst`), isolating only the Druid-specific level
//! for each of the 169 real records that name Druid at all: 13 orisons
//! (0th level), 20 first-level, 26 second-level, 22 third-level, 18
//! fourth-level, 18 fifth-level, 18 sixth-level, 13 seventh-level, 11
//! eighth-level, 10 ninth-level -- matching the real PF1 Druid full
//! 9th-level caster ceiling (verified via a direct parse of the raw
//! `CLASSES:` token). Every `key` here is spot-checked against a real,
//! exact `crb::spell_list::SPELL_LIST` key -- this is a strict subset of
//! the 652 CRB spell records, never an invented name.
//!
//! Unlike Cleric (which always gets 2 domains and their bonus spell
//! slots), a Druid's Nature Bond choice is EITHER an animal companion OR a
//! domain -- when a domain is chosen, the same "+1 domain spell slot per
//! accessible spell level" applies, but that choice itself (and any
//! domain's spell-list contents) remains part of the separate, unproven
//! animal-companion/nature-bond burden
//! (`class_feature.druid.animal_companion.unsupported`), deliberately out
//! of scope for this general list.
//!
//! Regenerate by re-parsing `cr_spells.lst`'s `CLASSES:` tag for any
//! `|`-separated group whose name list contains "Druid", taking that
//! group's own level (not the record's collapsed minimum), if the corpus
//! changes.

/// (spell key, Druid-specific spell level 0-9). A real CRB Druid may only
/// prepare a spell that appears in this table (from the general list; a
/// domain spell slot's own contents, when Nature Bond chooses a domain,
/// are a separate, unproven burden), subject to the character's own
/// spell-level access ceiling for their druid level -- see
/// `druid_spell_level` for the lookup helper.
pub const DRUID_SPELL_LIST: &[(&str, u8)] = &[
    ("Air Walk", 4),
    ("Animal Growth", 5),
    ("Animal Messenger", 2),
    ("Animal Shapes", 8),
    ("Animal Trance", 2),
    ("Animate Plants", 7),
    ("Antilife Shell", 6),
    ("Antipathy", 9),
    ("Antiplant Shell", 4),
    ("Atonement", 5),
    ("Awaken", 5),
    ("Baleful Polymorph", 5),
    ("Barkskin", 2),
    ("Bear's Endurance", 2),
    ("Bear's Endurance (Mass)", 6),
    ("Blight", 4),
    ("Bull's Strength", 2),
    ("Bull's Strength (Mass)", 6),
    ("Call Lightning", 3),
    ("Call Lightning Storm", 5),
    ("Calm Animals", 1),
    ("Cat's Grace", 2),
    ("Cat's Grace (Mass)", 6),
    ("Changestaff", 7),
    ("Charm Animal", 1),
    ("Chill Metal", 2),
    ("Command Plants", 4),
    ("Commune with Nature", 4),
    ("Contagion", 3),
    ("Control Plants", 8),
    ("Control Water", 4),
    ("Control Weather", 7),
    ("Control Winds", 5),
    ("Create Water", 0),
    ("Creeping Doom", 7),
    ("Cure Critical Wounds", 5),
    ("Cure Critical Wounds (Mass)", 9),
    ("Cure Light Wounds", 1),
    ("Cure Light Wounds (Mass)", 6),
    ("Cure Moderate Wounds", 3),
    ("Cure Moderate Wounds (Mass)", 7),
    ("Cure Serious Wounds", 4),
    ("Cure Serious Wounds (Mass)", 8),
    ("Daylight", 3),
    ("Death Ward", 5),
    ("Delay Poison", 2),
    ("Detect Animals or Plants", 1),
    ("Detect Magic", 0),
    ("Detect Poison", 0),
    ("Detect Snares and Pits", 1),
    ("Diminish Plants", 3),
    ("Dispel Magic", 4),
    ("Dispel Magic (Greater)", 6),
    ("Dominate Animal", 3),
    ("Earthquake", 8),
    ("Elemental Swarm", 9),
    ("Endure Elements", 1),
    ("Entangle", 1),
    ("Faerie Fire", 1),
    ("Find the Path", 6),
    ("Finger of Death", 8),
    ("Fire Seeds", 6),
    ("Fire Storm", 7),
    ("Fire Trap", 2),
    ("Flame Blade", 2),
    ("Flame Strike", 4),
    ("Flaming Sphere", 2),
    ("Flare", 0),
    ("Fog Cloud", 2),
    ("Foresight", 9),
    ("Freedom of Movement", 4),
    ("Giant Vermin", 4),
    ("Goodberry", 1),
    ("Guidance", 0),
    ("Gust of Wind", 2),
    ("Hallow", 5),
    ("Heal", 7),
    ("Heat Metal", 2),
    ("Hide from Animals", 1),
    ("Hold Animal", 2),
    ("Ice Storm", 4),
    ("Insect Plague", 5),
    ("Ironwood", 6),
    ("Jump", 1),
    ("Know Direction", 0),
    ("Light", 0),
    ("Liveoak", 6),
    ("Longstrider", 1),
    ("Magic Fang", 1),
    ("Magic Fang (Greater)", 3),
    ("Magic Stone", 1),
    ("Meld into Stone", 3),
    ("Mending", 0),
    ("Move Earth", 6),
    ("Neutralize Poison", 3),
    ("Obscuring Mist", 1),
    ("Owl's Wisdom", 2),
    ("Owl's Wisdom (Mass)", 6),
    ("Pass without Trace", 1),
    ("Plant Growth", 3),
    ("Poison", 3),
    ("Produce Flame", 1),
    ("Protection from Energy", 3),
    ("Purify Food and Drink", 0),
    ("Quench", 3),
    ("Read Magic", 0),
    ("Reduce Animal", 2),
    ("Regenerate", 9),
    ("Reincarnate", 4),
    ("Remove Disease", 3),
    ("Repel Metal or Stone", 8),
    ("Repel Vermin", 4),
    ("Repel Wood", 6),
    ("Resist Energy", 2),
    ("Resistance", 0),
    ("Restoration (Lesser)", 2),
    ("Reverse Gravity", 8),
    ("Rusting Grasp", 4),
    ("Scrying", 4),
    ("Scrying (Greater)", 7),
    ("Shambler", 9),
    ("Shapechange", 9),
    ("Shillelagh", 1),
    ("Sleet Storm", 3),
    ("Snare", 3),
    ("Soften Earth and Stone", 2),
    ("Speak with Animals", 1),
    ("Speak with Plants", 3),
    ("Spellstaff", 6),
    ("Spider Climb", 2),
    ("Spike Growth", 3),
    ("Spike Stones", 4),
    ("Stabilize", 0),
    ("Stone Shape", 3),
    ("Stone Tell", 6),
    ("Stoneskin", 5),
    ("Storm of Vengeance", 9),
    ("Summon Nature's Ally I", 1),
    ("Summon Nature's Ally II", 2),
    ("Summon Nature's Ally III", 3),
    ("Summon Nature's Ally IV", 4),
    ("Summon Nature's Ally IX", 9),
    ("Summon Nature's Ally V", 5),
    ("Summon Nature's Ally VI", 6),
    ("Summon Nature's Ally VII", 7),
    ("Summon Nature's Ally VIII", 8),
    ("Summon Swarm", 2),
    ("Sunbeam", 7),
    ("Sunburst", 8),
    ("Sympathy", 9),
    ("Transmute Metal to Wood", 7),
    ("Transmute Mud to Rock", 5),
    ("Transmute Rock to Mud", 5),
    ("Transport via Plants", 6),
    ("Tree Shape", 2),
    ("Tree Stride", 5),
    ("True Seeing", 7),
    ("Unhallow", 5),
    ("Virtue", 0),
    ("Wall of Fire", 5),
    ("Wall of Stone", 6),
    ("Wall of Thorns", 5),
    ("Warp Wood", 2),
    ("Water Breathing", 3),
    ("Whirlwind", 8),
    ("Wind Walk", 7),
    ("Wind Wall", 3),
    ("Wood Shape", 2),
    ("Word of Recall", 8),
];

/// Looks up a spell's Druid-specific spell level (0-9). `None` means the
/// named spell is not on the real CRB general Druid spell list at all --
/// either it's not a real spell, it's a real spell no Druid can ever
/// prepare, or it's a domain-only spell (not on the general list).
pub fn druid_spell_level(spell_key: &str) -> Option<u8> {
    DRUID_SPELL_LIST
        .iter()
        .find(|(key, _)| *key == spell_key)
        .map(|(_, level)| *level)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::rules_tables::crb::spell_list::SPELL_LIST;

    #[test]
    fn druid_spell_list_has_the_real_corpus_verified_count() {
        assert_eq!(DRUID_SPELL_LIST.len(), 169);
    }

    #[test]
    fn every_druid_spell_level_is_within_the_real_druid_ceiling() {
        for (key, level) in DRUID_SPELL_LIST {
            assert!(
                (0..=9).contains(level),
                "{key} has out-of-range Druid spell level {level}"
            );
        }
    }

    #[test]
    fn every_druid_spell_key_is_a_real_spell_list_entry() {
        for (key, _) in DRUID_SPELL_LIST {
            assert!(
                SPELL_LIST.iter().any(|entry| entry.key == *key),
                "{key} is not a real SPELL_LIST key"
            );
        }
    }

    #[test]
    fn druid_spell_level_looks_up_known_values() {
        assert_eq!(druid_spell_level("Cure Light Wounds"), Some(1));
        assert_eq!(druid_spell_level("Produce Flame"), Some(1));
        assert_eq!(druid_spell_level("Magic Missile"), None);
    }

    #[test]
    fn level_distribution_matches_the_real_corpus_parse() {
        let count_at =
            |level: u8| DRUID_SPELL_LIST.iter().filter(|(_, l)| *l == level).count();
        assert_eq!(count_at(0), 13);
        assert_eq!(count_at(1), 20);
        assert_eq!(count_at(2), 26);
        assert_eq!(count_at(3), 22);
        assert_eq!(count_at(4), 18);
        assert_eq!(count_at(5), 18);
        assert_eq!(count_at(6), 18);
        assert_eq!(count_at(7), 13);
        assert_eq!(count_at(8), 11);
        assert_eq!(count_at(9), 10);
    }
}
