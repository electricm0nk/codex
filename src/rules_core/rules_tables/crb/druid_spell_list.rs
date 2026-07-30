//! PF1 Druid spell list — per-class spell-level overrides.
//!
//! Mirrors `cleric_spell_list.rs`'s own doc comment exactly, substituting
//! Druid: `crb::spell_list::SPELL_LIST`'s `level` field is the MINIMUM
//! spell level across every class named in the corpus's `CLASSES:` tag
//! for that record, not necessarily the Druid-specific level. This table
//! re-parses the same corpus record's raw `CLASSES:` tag directly,
//! isolating only the Druid-specific level for each of the **271** real
//! records that name Druid at all: 14 orisons (0th level), then 42 / 51 /
//! 39 / 33 / 25 / 22 / 16 / 15 / 14 across levels 1-9 -- the full 0-9
//! range of a real PF1 Druid.
//!
//! `Animal Growth` is the live example of why the per-class level
//! matters: `CLASSES:Ranger=4|Druid,Sorcerer,Wizard=5` collapses to a
//! minimum of 4 in `SPELL_LIST`, but a Druid casts it at **5**.
//!
//! **Widened 2026-07-27 (task #29) from CRB-only to all ingested books**
//! (`risks-and-open-questions.md` item 53). Per-file: **169 from
//! `cr_spells.lst` + 70 from `apg_spells.lst` + 32 from `acg_spells.lst`
//! = 271**, all names distinct, no `.MOD` record assigns Druid.
//!
//! Per-file ceiling check: `grep -c Druid` returns 170 / 70 / 32 -- exact
//! in two files. The single CRB line above the parse is `Atonement.MOD`,
//! which carries no `CLASSES:` token at all; its base record
//! (`CLASSES:Cleric,Druid=5`) does name Druid and IS counted.
//!
//! **This module did NOT have the `CLASSES:` substring bug.** Its
//! original 169 CRB entries are byte-identical to a correct token-split
//! re-parse of `cr_spells.lst`, including all 69 records where Druid sits
//! mid-group. The only defect was book scope. No bracketed-level record
//! names Druid.
//!
//! **Corpus reachability: all 271 resolve** against the union of the
//! three ingested books' own `SPELL_LIST` tables (1,075 keys) -- never an
//! invented name. The cross-check test asserts exactly that union; a
//! CRB-only check would now reject every APG/ACG entry as fictional.
//!
//! Regenerate by parsing the `CLASSES:` token in all three spell files --
//! split the body on `|`, `rpartition` each group on `=`, strip any
//! trailing `[...]` gate from the level, then membership-test the
//! comma-separated name list. Never substring-match `Druid=`.

/// (spell key, Druid-specific spell level 0-9). A real Druid may only
/// prepare a spell that appears in this table (from the general list; a
/// domain spell slot's own contents, when Nature Bond chooses a domain,
/// are a separate, unproven burden), subject to the character's own
/// spell-level access ceiling for their druid level -- see
/// `druid_spell_level` for the lookup helper.
pub const DRUID_SPELL_LIST: &[(&str, u8)] = &[
    ("Accelerate Poison", 2),
    ("Aggressive Thundercloud", 2),
    ("Aggressive Thundercloud (Greater)", 4),
    ("Air Geyser", 3),
    ("Air Step", 2),
    ("Air Walk", 4),
    ("Alter Winds", 1),
    ("Anchored Step", 3),
    ("Animal Growth", 5),
    ("Animal Messenger", 2),
    ("Animal Purpose Training", 2),
    ("Animal Shapes", 8),
    ("Animal Trance", 2),
    ("Animate Plants", 7),
    ("Ant Haul", 1),
    ("Antilife Shell", 6),
    ("Antipathy", 9),
    ("Antiplant Shell", 4),
    ("Aqueous Orb", 3),
    ("Aspect of the Bear", 2),
    ("Aspect of the Falcon", 1),
    ("Aspect of the Stag", 4),
    ("Aspect of the Wolf", 5),
    ("Atonement", 5),
    ("Awaken", 5),
    ("Baleful Polymorph", 5),
    ("Ball Lightning", 4),
    ("Barkskin", 2),
    ("Bear's Endurance", 2),
    ("Bear's Endurance (Mass)", 6),
    ("Beastspeak", 2),
    ("Blazing Rainbow", 6),
    ("Blessing of the Salamander", 5),
    ("Blight", 4),
    ("Bloody Claws", 4),
    ("Bristle", 1),
    ("Bull's Strength", 2),
    ("Bull's Strength (Mass)", 6),
    ("Burning Gaze", 2),
    ("Call Animal", 1),
    ("Call Lightning", 3),
    ("Call Lightning Storm", 5),
    ("Calm Animals", 1),
    ("Campfire Wall", 2),
    ("Cat's Grace", 2),
    ("Cat's Grace (Mass)", 6),
    ("Changestaff", 7),
    ("Charm Animal", 1),
    ("Chill Metal", 2),
    ("Clashing Rocks", 9),
    ("Climbing Beanstalk", 2),
    ("Cloak of Shade", 1),
    ("Cloak of Winds", 3),
    ("Command Plants", 4),
    ("Commune with Nature", 4),
    ("Companion Life Link", 2),
    ("Contagion", 3),
    ("Control Plants", 8),
    ("Control Water", 4),
    ("Control Weather", 7),
    ("Control Winds", 5),
    ("Create Treasure Map", 3),
    ("Create Water", 0),
    ("Creeping Doom", 7),
    ("Creeping Ice", 4),
    ("Cup of Dust", 3),
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
    ("Detect Aberration", 1),
    ("Detect Animals or Plants", 1),
    ("Detect Magic", 0),
    ("Detect Poison", 0),
    ("Detect Snares and Pits", 1),
    ("Diminish Plants", 3),
    ("Dispel Magic", 4),
    ("Dispel Magic (Greater)", 6),
    ("Dominate Animal", 3),
    ("Eagle Eye", 2),
    ("Earthquake", 8),
    ("Elemental Speech", 2),
    ("Elemental Swarm", 9),
    ("Endure Elements", 1),
    ("Entangle", 1),
    ("Euphoric Cloud", 2),
    ("Euphoric Tranquility", 8),
    ("Expeditious Excavation", 1),
    ("Faerie Fire", 1),
    ("Fairy Ring Retreat", 7),
    ("Feast of Ashes", 2),
    ("Feather Step", 1),
    ("Feather Step (Mass)", 3),
    ("Find the Path", 6),
    ("Finger of Death", 8),
    ("Fire Seeds", 6),
    ("Fire Snake", 5),
    ("Fire Storm", 7),
    ("Fire Trap", 2),
    ("Flame Blade", 2),
    ("Flame Strike", 4),
    ("Flaming Sphere", 2),
    ("Flaming Sphere (Greater)", 4),
    ("Flare", 0),
    ("Flare Burst", 1),
    ("Fog Cloud", 2),
    ("Foresight", 9),
    ("Freedom of Movement", 4),
    ("Gentle Breeze", 1),
    ("Geyser", 4),
    ("Giant Vermin", 4),
    ("Glide", 2),
    ("Goodberry", 1),
    ("Grove of Respite", 4),
    ("Guidance", 0),
    ("Gust of Wind", 2),
    ("Hallow", 5),
    ("Heal", 7),
    ("Heat Metal", 2),
    ("Heightened Awareness", 1),
    ("Hide Campsite", 3),
    ("Hide from Animals", 1),
    ("Hold Animal", 2),
    ("Hydraulic Push", 1),
    ("Hydraulic Torrent", 3),
    ("Ice Storm", 4),
    ("Insect Plague", 5),
    ("Ironwood", 6),
    ("Jump", 1),
    ("Keen Senses", 1),
    ("Know Direction", 0),
    ("Life Bubble", 4),
    ("Light", 0),
    ("Lily Pad Stride", 3),
    ("Liveoak", 6),
    ("Lockjaw", 2),
    ("Longstrider", 1),
    ("Longstrider (Greater)", 3),
    ("Magic Fang", 1),
    ("Magic Fang (Greater)", 3),
    ("Magic Stone", 1),
    ("Meld into Stone", 3),
    ("Mending", 0),
    ("Monkey Fish", 1),
    ("Moonstruck", 4),
    ("Move Earth", 6),
    ("Natural Rhythm", 2),
    ("Nature's Exile", 3),
    ("Nauseating Dart", 1),
    ("Nauseating Trail", 3),
    ("Negate Aroma", 1),
    ("Neutralize Poison", 3),
    ("Obscuring Mist", 1),
    ("Owl's Wisdom", 2),
    ("Owl's Wisdom (Mass)", 6),
    ("Pass without Trace", 1),
    ("Plant Growth", 3),
    ("Poison", 3),
    ("Pox Pustules", 2),
    ("Produce Flame", 1),
    ("Protection from Energy", 3),
    ("Purify Food and Drink", 0),
    ("Quench", 3),
    ("Rampart", 7),
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
    ("Rest Eternal", 5),
    ("Restoration (Lesser)", 2),
    ("Reverse Gravity", 8),
    ("River of Wind", 4),
    ("Rusting Grasp", 4),
    ("Scent Trail", 2),
    ("Scrying", 4),
    ("Scrying (Greater)", 7),
    ("Seamantle", 8),
    ("Shambler", 9),
    ("Shapechange", 9),
    ("Share Language", 2),
    ("Shifting Sand", 3),
    ("Shillelagh", 1),
    ("Sickening Entanglement", 2),
    ("Sirocco", 6),
    ("Sleet Storm", 3),
    ("Slipstream", 2),
    ("Slowing Mud", 4),
    ("Snake Staff", 5),
    ("Snare", 3),
    ("Soften Earth and Stone", 2),
    ("Spark", 0),
    ("Speak with Animals", 1),
    ("Speak with Plants", 3),
    ("Spellstaff", 6),
    ("Spider Climb", 2),
    ("Spike Growth", 3),
    ("Spike Stones", 4),
    ("Stabilize", 0),
    ("Stench of Prey", 3),
    ("Stone Call", 2),
    ("Stone Discus", 2),
    ("Stone Fist", 1),
    ("Stone Shape", 3),
    ("Stone Tell", 6),
    ("Stoneskin", 5),
    ("Storm of Vengeance", 9),
    ("Stormbolts", 8),
    ("Strong Jaw", 4),
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
    ("Swarm Skin", 6),
    ("Sympathy", 9),
    ("Thorn Body", 4),
    ("Thorn Javelin", 1),
    ("Thorny Entanglement", 3),
    ("Threefold Aspect", 5),
    ("Thunderstomp", 1),
    ("Thunderstomp (Greater)", 3),
    ("Touch of the Sea", 1),
    ("Transmute Metal to Wood", 7),
    ("Transmute Mud to Rock", 5),
    ("Transmute Rock to Mud", 5),
    ("Transport via Plants", 6),
    ("Tree Shape", 2),
    ("Tree Stride", 5),
    ("True Form", 4),
    ("True Seeing", 7),
    ("Tsunami", 9),
    ("Unhallow", 5),
    ("Virtue", 0),
    ("Vortex", 7),
    ("Wall of Fire", 5),
    ("Wall of Lava", 8),
    ("Wall of Stone", 6),
    ("Wall of Thorns", 5),
    ("Warp Wood", 2),
    ("Water Breathing", 3),
    ("Wave Shield", 1),
    ("Whip of Ants", 6),
    ("Whip of Centipedes", 5),
    ("Whip of Spiders", 2),
    ("Whirlwind", 8),
    ("Wind Walk", 7),
    ("Wind Wall", 3),
    ("Winds of Vengeance", 9),
    ("Wood Shape", 2),
    ("Word of Recall", 8),
    ("World Wave", 9),
];

/// Looks up a spell's Druid-specific spell level (0-9). `None` means the
/// named spell is not on the real general Druid spell list at all --
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
    use crate::rules_core::rules_tables::acg::spell_list as acg_spell_list;
    use crate::rules_core::rules_tables::apg::spell_list as apg_spell_list;
    use crate::rules_core::rules_tables::crb::spell_list::SPELL_LIST;

    #[test]
    fn druid_spell_list_has_the_real_corpus_verified_count() {
        assert_eq!(DRUID_SPELL_LIST.len(), 271);
    }

    /// Guards the book-scope widening (task #29). One anchor per ingested
    /// book, each naming Druid mid-group so its raw line carries no
    /// `Druid=` substring at all:
    /// `Animal Growth` is `CLASSES:Ranger=4|Druid,Sorcerer,Wizard=5` (CRB
    /// -- also a live example of why the per-class level matters, since
    /// the record's collapsed minimum is Ranger's 4, not Druid's 5),
    /// `Alter Winds` is `CLASSES:Druid,Sorcerer,Wizard=1` (APG),
    /// `Air Geyser` is
    /// `CLASSES:Bloodrager,Druid,Magus,Sorcerer,Witch,Wizard=3|Shaman=4` (ACG).
    #[test]
    fn spells_tagged_mid_list_in_their_classes_group_are_present() {
        assert_eq!(druid_spell_level("Animal Growth"), Some(5));
        assert_eq!(druid_spell_level("Alter Winds"), Some(1));
        assert_eq!(druid_spell_level("Air Geyser"), Some(3));
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

    /// Druid's list spans all three ingested books, so a CRB-only
    /// cross-check would reject every APG/ACG entry as fictional. Checks
    /// the union instead -- still a real "never an invented name"
    /// guarantee, just scoped to everything this repo actually ingests.
    #[test]
    fn every_druid_spell_key_is_a_real_spell_list_entry() {
        for (key, _) in DRUID_SPELL_LIST {
            let known = SPELL_LIST.iter().any(|entry| entry.key == *key)
                || apg_spell_list::SPELL_LIST.iter().any(|entry| entry.key == *key)
                || acg_spell_list::SPELL_LIST.iter().any(|entry| entry.key == *key);
            assert!(known, "{key} is not a real spell key in any ingested book");
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
        assert_eq!(count_at(0), 14);
        assert_eq!(count_at(1), 42);
        assert_eq!(count_at(2), 51);
        assert_eq!(count_at(3), 39);
        assert_eq!(count_at(4), 33);
        assert_eq!(count_at(5), 25);
        assert_eq!(count_at(6), 22);
        assert_eq!(count_at(7), 16);
        assert_eq!(count_at(8), 15);
        assert_eq!(count_at(9), 14);
    }
}
