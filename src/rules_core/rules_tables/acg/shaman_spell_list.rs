//! PF1 ACG Shaman spell list — one `(spell name, Shaman spell level)`
//! entry per real corpus record.
//!
//! Source: every record whose `CLASSES:` token names Shaman in any of its
//! comma-separated class groups, across the books this repo ingests.
//! **304 unique spells**, levels 0-9, split
//! **17 / 48 / 46 / 46 / 41 / 30 / 22 / 22 / 18 / 14**.
//!
//! Shaman is a fresh own-list ingest: unlike Investigator (which reuses
//! Alchemist's list via `SPELLLIST:1|Alchemist`) or Oracle (which shares
//! Cleric's), the class block carries **no `SPELLLIST:` reuse token at
//! all**, so this table has no existing list to delegate to.
//!
//! **All 304 come from `acg_spells.lst`.** `cr_spells.lst` and
//! `apg_spells.lst` name Shaman zero times — the class postdates both
//! books, and nothing back-grafts Shaman onto their records. Of the 304,
//! **267 are `.MOD` grafts** onto spells first printed in an earlier book
//! and only **37 are new ACG spells named on their own line**, which is
//! why the whole list lives in one file despite drawing most of its
//! contents from CRB/APG spells.
//!
//! Per-file ceiling check: `grep -c Shaman acg_spells.lst` returns 306
//! against a parse of 304. Both extra lines are `#`-commented and neither
//! is a dropped record: the `###Block: Shaman Spells` section header, and
//! `#Commune With Birds.MOD`. That second one is a deliberate corpus
//! exclusion worth naming — it is the ONLY record for that spell anywhere
//! in the three ingested books, it is commented out, and no
//! `commune_with_birds` record exists in this repo's own `data/corpus/`
//! either. So the spell is doubly absent rather than parse-dropped, and
//! including it would fabricate a spell this engine cannot resolve.
//!
//! **Corpus reachability: 283 of 304 (93%).** The 21 that do not resolve
//! against this repo's ingested `data/corpus/` spell records are listed
//! in `SHAMAN_SPELLS_NOT_INGESTED` below. They are carried here anyway,
//! for the same reason `BLOODRAGER_SPELL_LIST` carries its own 73: they
//! are genuinely on the real Shaman spell list, and dropping them would
//! silently shrink the class's list rather than surface the gap.
//!
//! Parsing discipline (see `apg::witch_spell_list`'s own header for the
//! full history of getting this wrong twice): split the `CLASSES:` body
//! on `|`, `rpartition` each group on `=`, strip any trailing `[...]`
//! gate from the level, then MEMBERSHIP-TEST the comma-separated name
//! list. Never substring-match `Shaman=` — 24 of these records name
//! Shaman mid-group and carry no such substring. Never let an `int()` on
//! the level throw a record away silently.

/// Every `(spell name, Shaman spell level)` pair on the real ACG Shaman
/// spell list, sorted by name.
pub const SHAMAN_SPELL_LIST: &[(&str, u8)] = &[
    ("Adjustable Polymorph", 4),
    ("Aid", 2),
    ("Air Geyser", 4),
    ("Alter Self", 2),
    ("Anchored Step", 3),
    ("Animal Growth", 5),
    ("Animal Messenger", 2),
    ("Animal Purpose Training", 2),
    ("Animal Shapes", 8),
    ("Animate Dead", 3),
    ("Animate Dead (Lesser)", 2),
    ("Animate Plants", 7),
    ("Anti-Incorporeal Shell", 4),
    ("Antilife Shell", 6),
    ("Arcane Mark", 0),
    ("Augury", 2),
    ("Aura Sight", 3),
    ("Awaken", 6),
    ("Baleful Polymorph", 5),
    ("Ball Lightning", 4),
    ("Bane", 1),
    ("Banishment", 6),
    ("Barkskin", 2),
    ("Bear's Endurance", 2),
    ("Bear's Endurance (Mass)", 6),
    ("Beastspeak", 2),
    ("Bestow Curse", 3),
    ("Bleed", 0),
    ("Blend", 1),
    ("Bless", 1),
    ("Blight", 5),
    ("Blindness/Deafness", 3),
    ("Blood Mist", 8),
    ("Break Enchantment", 5),
    ("Breath of Life", 5),
    ("Bull's Strength", 2),
    ("Bull's Strength (Mass)", 6),
    ("Buoyancy", 2),
    ("Burning Gaze", 2),
    ("Burning Hands", 1),
    ("Call Lightning", 3),
    ("Call Lightning Storm", 5),
    ("Calm Animals", 1),
    ("Calm Emotions", 2),
    ("Cause Fear", 1),
    ("Charm Animal", 1),
    ("Charm Person", 1),
    ("Chill Touch", 1),
    ("Circle of Clarity", 8),
    ("Clairaudience/Clairvoyance", 3),
    ("Cloak of Dreams", 7),
    ("Command Plants", 4),
    ("Commune", 5),
    ("Commune with Nature", 5),
    ("Comprehend Languages", 1),
    ("Cone of Cold", 6),
    ("Control Water", 4),
    ("Control Weather", 7),
    ("Control Winds", 5),
    ("Create Food and Water", 3),
    ("Create Greater Undead", 8),
    ("Create Undead", 6),
    ("Create Water", 0),
    ("Creeping Doom", 7),
    ("Cure Critical Wounds", 4),
    ("Cure Critical Wounds (Mass)", 8),
    ("Cure Light Wounds", 1),
    ("Cure Light Wounds (Mass)", 5),
    ("Cure Moderate Wounds", 2),
    ("Cure Moderate Wounds (Mass)", 6),
    ("Cure Serious Wounds", 3),
    ("Cure Serious Wounds (Mass)", 7),
    ("Curse (Major)", 5),
    ("Curse of Burning Sleep", 4),
    ("Dancing Lantern", 1),
    ("Dancing Lights", 0),
    ("Darkness", 2),
    ("Daylight", 3),
    ("Daze", 0),
    ("Deep Slumber", 3),
    ("Deeper Darkness", 3),
    ("Delay Poison", 2),
    ("Destruction", 8),
    ("Detect Animals or Plants", 1),
    ("Detect Chaos", 1),
    ("Detect Evil", 1),
    ("Detect Good", 1),
    ("Detect Law", 1),
    ("Detect Magic", 0),
    ("Detect Poison", 0),
    ("Detect Scrying", 4),
    ("Detect Undead", 1),
    ("Discern Location", 8),
    ("Discern Next of Kin", 1),
    ("Dismissal", 4),
    ("Dispel Chaos", 5),
    ("Dispel Evil", 5),
    ("Dispel Good", 5),
    ("Dispel Law", 5),
    ("Dispel Magic", 3),
    ("Dispel Magic (Greater)", 6),
    ("Divination", 4),
    ("Divine Power", 4),
    ("Dominate Animal", 3),
    ("Dominate Person", 5),
    ("Doom", 1),
    ("Eagle Eye", 2),
    ("Eagle's Splendor", 2),
    ("Eagle's Splendor (Mass)", 6),
    ("Earth Glide", 4),
    ("Earthquake", 8),
    ("Elemental Swarm", 9),
    ("Endure Elements", 1),
    ("Energy Drain", 9),
    ("Entangle", 1),
    ("Enthrall", 2),
    ("Etherealness", 9),
    ("Fairy Ring Retreat", 7),
    ("False Life", 2),
    ("False Life (Greater)", 4),
    ("Familiar Melding", 4),
    ("Fear", 4),
    ("Feast on Fear", 5),
    ("Find the Path", 6),
    ("Fins to Feet", 3),
    ("Fire Seeds", 6),
    ("Fire Storm", 8),
    ("Flame Blade", 2),
    ("Flame Strike", 5),
    ("Flesh to Stone", 6),
    ("Fly", 3),
    ("Focused Scrutiny", 2),
    ("Fog Cloud", 2),
    ("Font of Spirit Magic", 3),
    ("Foresight", 9),
    ("Frostbite", 1),
    ("Gentle Breeze", 1),
    ("Gentle Repose", 2),
    ("Ghostbane Dirge", 2),
    ("Ghostbane Dirge (Mass)", 5),
    ("Giant Vermin", 4),
    ("Glide", 2),
    ("Goodberry", 1),
    ("Grove of Respite", 5),
    ("Guidance", 0),
    ("Guiding Star", 2),
    ("Harm", 7),
    ("Heal", 7),
    ("Heal (Mass)", 9),
    ("Heightened Awareness", 1),
    ("Hex Glyph", 3),
    ("Hex Glyph (Greater)", 5),
    ("Hex Vulnerability", 1),
    ("Hex Ward", 1),
    ("Hide from Animals", 1),
    ("Hold Person", 2),
    ("Horrid Wilting", 8),
    ("Hydraulic Push", 1),
    ("Ice Body", 7),
    ("Ice Storm", 4),
    ("Imbue With Elemental Might", 2),
    ("Imbue with Spell Ability", 4),
    ("Inflict Critical Wounds", 4),
    ("Inflict Critical Wounds (Mass)", 8),
    ("Inflict Light Wounds", 1),
    ("Inflict Light Wounds (Mass)", 5),
    ("Inflict Moderate Wounds", 2),
    ("Inflict Moderate Wounds (Mass)", 6),
    ("Inflict Serious Wounds", 3),
    ("Inflict Serious Wounds (Mass)", 7),
    ("Insect Plague", 5),
    ("Irresistible Dance", 8),
    ("Know Direction", 0),
    ("Levitate", 2),
    ("Life Pact", 2),
    ("Light", 0),
    ("Liveoak", 7),
    ("Magic Circle against Chaos", 3),
    ("Magic Circle against Evil", 3),
    ("Magic Circle against Good", 3),
    ("Magic Circle against Law", 3),
    ("Magic Stone", 1),
    ("Magic Vestment", 3),
    ("Magic Weapon", 1),
    ("Magic Weapon (Greater)", 4),
    ("Mantle of Calm", 3),
    ("Mending", 0),
    ("Mindlocked Messenger", 3),
    ("Monkey Fish", 1),
    ("Nauseating Trail", 3),
    ("Neutralize Poison", 4),
    ("Obscuring Mist", 1),
    ("Overland Flight", 5),
    ("Owl's Wisdom", 2),
    ("Owl's Wisdom (Mass)", 6),
    ("Pass without Trace", 1),
    ("Persistent Vigor", 4),
    ("Pierce Disguise", 3),
    ("Planar Ally", 6),
    ("Planar Ally (Greater)", 8),
    ("Planar Ally (Lesser)", 4),
    ("Plane Shift", 7),
    ("Poison", 4),
    ("Polar Midnight", 9),
    ("Polymorph Familiar", 3),
    ("Produce Flame", 1),
    ("Protection from Chaos", 1),
    ("Protection from Energy", 3),
    ("Protection from Evil", 1),
    ("Protection from Good", 1),
    ("Protection from Law", 1),
    ("Purify Food and Drink", 0),
    ("Rain of Frogs", 4),
    ("Raise Dead", 6),
    ("Read Magic", 0),
    ("Regenerate", 7),
    ("Reincarnate", 4),
    ("Remove Blindness/Deafness", 3),
    ("Remove Curse", 3),
    ("Remove Disease", 3),
    ("Remove Fear", 1),
    ("Remove Paralysis", 2),
    ("Repel Vermin", 4),
    ("Resist Energy", 2),
    ("Resistance", 0),
    ("Rest Eternal", 5),
    ("Restoration", 4),
    ("Restoration (Greater)", 7),
    ("Restoration (Lesser)", 2),
    ("Resurrection", 8),
    ("Ride the Waves", 4),
    ("Sands of Time", 4),
    ("Scare", 2),
    ("Scrying", 4),
    ("Scrying (Greater)", 7),
    ("Sending", 4),
    ("Sense Spirit Magic", 1),
    ("Shambler", 9),
    ("Shapechange", 9),
    ("Shield Companion", 2),
    ("Sickening Entanglement", 2),
    ("Slay Living", 6),
    ("Sleep", 1),
    ("Sleet Storm", 3),
    ("Slowing Mud", 4),
    ("Snake Staff", 6),
    ("Solid Fog", 4),
    ("Soul Bind", 9),
    ("Speak with Dead", 3),
    ("Speak with Haunt", 3),
    ("Spike Stones", 4),
    ("Spiritual Weapon", 2),
    ("Spit Venom", 4),
    ("Stabilize", 0),
    ("Stench of Prey", 3),
    ("Stinking Cloud", 3),
    ("Stone Shape", 3),
    ("Stone Shield", 1),
    ("Stone Tell", 7),
    ("Stone to Flesh", 6),
    ("Stoneskin", 5),
    ("Storm of Vengeance", 9),
    ("Stormbolts", 8),
    ("Stricken Heart", 3),
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
    ("Thorn Body", 4),
    ("Thorn Javelin", 1),
    ("Thorny Entanglement", 3),
    ("Tongues", 4),
    ("Touch of Fatigue", 0),
    ("Transport via Plants", 7),
    ("Tree Shape", 2),
    ("True Seeing", 5),
    ("Tsunami", 9),
    ("Virtue", 0),
    ("Vision", 7),
    ("Vortex", 7),
    ("Wail of the Banshee", 9),
    ("Wall of Fire", 5),
    ("Wall of Stone", 6),
    ("Wall of Thorns", 5),
    ("Wandering Star Motes", 4),
    ("Ward of the Season", 3),
    ("Warp Wood", 2),
    ("Water Breathing", 3),
    ("Water Walk", 3),
    ("Wave Shield", 1),
    ("Web Shelter", 2),
    ("Whirlwind", 8),
    ("Wind Walk", 7),
    ("Wind Wall", 3),
    ("Winds of Vengeance", 9),
    ("Wood Shape", 2),
];

/// The 21 entries of `SHAMAN_SPELL_LIST` that have no matching record in
/// this repo's ingested `data/corpus/` spell data.
///
/// Kept as an explicit, asserted list rather than a bare count so the gap
/// is auditable: if corpus ingestion later widens, this list shrinks and
/// its test fails loudly rather than the discrepancy going unnoticed. A
/// Shaman selecting one of these surfaces through the existing
/// unresolved-selection idiom instead of being silently dropped from the
/// class's list.
pub const SHAMAN_SPELLS_NOT_INGESTED: &[&str] = &[
    "Animate Dead (Lesser)",
    "Blend",
    "Blood Mist",
    "Circle of Clarity",
    "Curse (Major)",
    "Earth Glide",
    "False Life (Greater)",
    "Familiar Melding",
    "Fins to Feet",
    "Frostbite",
    "Hex Ward",
    "Ice Body",
    "Imbue With Elemental Might",
    "Polar Midnight",
    "Rain of Frogs",
    "Ride the Waves",
    "Sands of Time",
    "Spit Venom",
    "Stone Shield",
    "Ward of the Season",
    "Web Shelter",
];

/// Looks up a spell's Shaman-specific spell level (0-9). `None` means the
/// named spell is not on the real Shaman spell list at all -- either it's
/// not a real spell, or it's a real spell no Shaman can ever prepare.
pub fn shaman_spell_level(spell_key: &str) -> Option<u8> {
    SHAMAN_SPELL_LIST
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
    fn the_list_matches_the_verified_corpus_extraction() {
        assert_eq!(SHAMAN_SPELL_LIST.len(), 304, "304 real Shaman spell records");
        let expected = [17, 48, 46, 46, 41, 30, 22, 22, 18, 14];
        for (level, want) in expected.iter().enumerate() {
            let count = SHAMAN_SPELL_LIST
                .iter()
                .filter(|(_, l)| usize::from(*l) == level)
                .count();
            assert_eq!(count, *want, "spell level {level} count");
        }
    }

    #[test]
    fn every_shaman_spell_level_is_within_the_real_shaman_ceiling() {
        for (key, level) in SHAMAN_SPELL_LIST {
            assert!(
                (0..=9).contains(level),
                "{key} has out-of-range Shaman spell level {level}"
            );
        }
    }

    #[test]
    fn no_duplicate_spell_names() {
        let mut names: Vec<&str> = SHAMAN_SPELL_LIST.iter().map(|(name, _)| *name).collect();
        let original_len = names.len();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), original_len, "expected zero duplicate spell names");
    }

    /// Guards the `CLASSES:` mid-group parsing hazard. Both of these name
    /// Shaman in a comma group where Shaman is NOT last, so the substring
    /// `Shaman=` never appears on their raw lines:
    /// `Adjustable Polymorph` is
    /// `CLASSES:Alchemist,Bard,Magus,Shaman,Sorcerer,Witch,Wizard=4` and
    /// `Aura Sight` is
    /// `CLASSES:Alchemist,Cleric,Shaman,Sorcerer,Witch,Wizard=3|Inquisitor=4`
    /// -- note the latter also proves the per-class level is isolated
    /// correctly, since Inquisitor's own group says 4, not 3.
    #[test]
    fn spells_tagged_mid_list_in_their_classes_group_are_present() {
        assert_eq!(shaman_spell_level("Adjustable Polymorph"), Some(4));
        assert_eq!(shaman_spell_level("Aura Sight"), Some(3));
    }

    /// `Commune With Birds` must NOT be here. Its only record anywhere in
    /// the three ingested books is `#Commune With Birds.MOD` -- commented
    /// out -- and no corresponding `data/corpus/` spell record exists
    /// either. Including it would fabricate a spell this engine cannot
    /// resolve.
    #[test]
    fn a_corpus_commented_out_grant_is_not_on_the_list() {
        assert_eq!(shaman_spell_level("Commune With Birds"), None);
    }

    #[test]
    fn an_unrelated_or_nonexistent_spell_resolves_to_none() {
        assert_eq!(shaman_spell_level("Magic Missile"), None);
        assert_eq!(shaman_spell_level("Not A Real Spell"), None);
    }

    /// Every name here must be a real spell key in some ingested book --
    /// never an invented name -- EXCEPT the explicitly-listed
    /// not-yet-ingested entries, which are real Shaman spells whose base
    /// records this repo does not carry.
    #[test]
    fn every_shaman_spell_key_is_real_or_explicitly_listed_as_engine_does_not_hold() {
        for (key, _) in SHAMAN_SPELL_LIST {
            let known = SPELL_LIST.iter().any(|entry| entry.key == *key)
                || apg_spell_list::SPELL_LIST.iter().any(|entry| entry.key == *key)
                || acg_spell_list::SPELL_LIST.iter().any(|entry| entry.key == *key);
            if !known {
                assert!(
                    SHAMAN_SPELLS_NOT_INGESTED.contains(key),
                    "{key} resolves nowhere and is not declared in SHAMAN_SPELLS_NOT_INGESTED"
                );
            }
        }
    }

    /// The converse direction: every declared engine-does-not-hold entry must
    /// actually be on the list and actually be unresolvable. This is what
    /// makes the gap shrink loudly if corpus ingestion later widens.
    #[test]
    fn the_engine_does_not_hold_list_is_accurate_in_both_directions() {
        assert_eq!(SHAMAN_SPELLS_NOT_INGESTED.len(), 21);
        for key in SHAMAN_SPELLS_NOT_INGESTED {
            assert!(
                shaman_spell_level(key).is_some(),
                "{key} is declared engine-does-not-hold but is not on the Shaman list at all"
            );
            let resolves = SPELL_LIST.iter().any(|entry| entry.key == *key)
                || apg_spell_list::SPELL_LIST.iter().any(|entry| entry.key == *key)
                || acg_spell_list::SPELL_LIST.iter().any(|entry| entry.key == *key);
            assert!(
                !resolves,
                "{key} is declared engine-does-not-hold but now resolves -- corpus ingestion widened, \
                 shrink SHAMAN_SPELLS_NOT_INGESTED"
            );
        }
    }
}
