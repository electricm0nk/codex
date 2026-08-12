//! PF1 Alchemist formula spell list (deepening 2026-07-26, task #8:
//! Investigator spellcasting subsystem).
//!
//! Real PF1 rule text (`acg_abilities_class.lst`'s own Investigator
//! Alchemy DESC): "An investigator uses the alchemist formula list
//! (Pathfinder RPG Advanced Player's Guide 32) to determine the extracts
//! he can know" -- Investigator's `SPELLLIST:1|Alchemist` corpus token
//! confirms this is the same list, not merely a similar one. This module
//! is the shared list both Investigator's (this closure) and Alchemist's
//! own (task #4, not built here) spellcasting can consume.
//!
//! Source: every real record whose `CLASSES:` token names Alchemist in
//! any pipe-separated group, extracted via a direct parse of the raw
//! corpus files (not hand-transcribed) -- **147 total**. Two shapes exist
//! in the corpus:
//!
//! - 56 genuinely new Alchemist spells named directly on their own line.
//! - 91 `.MOD` records that graft Alchemist onto an existing Core
//!   Rulebook spell's `CLASSES:` token (e.g. `Cure Light Wounds.MOD`).
//!   The real spell name is the base name with `.MOD` stripped; the
//!   level here is read directly off that same `.MOD` line's own
//!   `CLASSES:` token -- no cross-file lookup into `cr_spells.lst` is
//!   needed for level purposes (only for school/description metadata,
//!   which this `(name, level)`-only shape doesn't carry, mirroring
//!   `CLERIC_SPELL_LIST`/`BARD_SPELL_LIST`'s own minimal shape).
//!
//! **Widened 2026-07-27 (task #32) from APG-only to all ingested books.**
//! Per-file: **121 from `apg_spells.lst` + 26 from `acg_spells.lst` =
//! 147**, all names distinct; `cr_spells.lst` names Alchemist zero times
//! (the class postdates the CRB). All 26 ACG additions are non-`.MOD` new
//! spells.
//!
//! The superseded rationale is worth recording, since this module argued
//! its own scope explicitly: it previously cited the rule text's own
//! "Advanced Player's Guide" citation plus a "single-book-source
//! discipline every other spell list already uses." That discipline was
//! real when written but was never a deliberate design -- it was an
//! artifact of each list having been generated from one file. PF1 does
//! not scope a class's spell list by sourcebook, and all eight lists now
//! span every ingested book. Ruling: team lead, 2026-07-27
//! (`risks-and-open-questions.md` item 53; this module's own open
//! question was what surfaced it).
//!
//! Level breakdown, re-derived: **28 / 36 / 29 / 23 / 15 / 16** across
//! extract levels 1-6, summing to 147. Per-file ceiling check:
//! `grep -c Alchemist` returns 0 / 122 / 26; the single APG line above
//! the parse is the `# Alchemist Spells` section-header comment, which
//! carries no `CLASSES:` token at all -- not a dropped record.
//!
//! **Corpus reachability: all 147 resolve** against this repo's own
//! ingested `data/corpus/` spell records (1,075 keys) -- unlike
//! `BLOODRAGER_SPELL_LIST`, this list has no unreachable entries.
//!
//! Regenerate by parsing the `CLASSES:` token in `apg_spells.lst` and
//! `acg_spells.lst` -- split the body on `|`, `rpartition` each group on
//! `=`, strip any trailing `[...]` gate from the level, then
//! membership-test the comma-separated name list -- and strip a trailing
//! `.MOD` from the record's own name column. Never substring-match
//! `Alchemist=`.

/// (spell key, Alchemist-specific extract level 1-6). A real Investigator
/// (or, in a future closure, Alchemist) may only know/prepare an extract
/// that appears in this table, subject to the character's own extract-
/// level access ceiling for their class level -- see
/// `alchemist_spell_level` for the lookup helper.
///
/// **Corrected 2026-07-27 (task #24): this list shipped with 104 entries
/// and was short by 17.** It was generated with an `Alchemist=` substring
/// match, which only hits when Alchemist is the LAST name in its comma
/// group -- the level belongs to the whole group, so
/// `CLASSES:Alchemist,Witch=2` contains no `Alchemist=` substring at all.
/// All 104 shipped entries were real; the 17 missing ones were silently
/// absent, not flagged. Regenerated with a token-split-and-membership
/// parse. Same bug class as the Witch/Bloodrager fix in `0ca6fd89`.
pub const ALCHEMIST_SPELL_LIST: &[(&str, u8)] = &[
    ("Absorbing Touch", 3),
    ("Adhesive Blood", 2),
    ("Adhesive Spittle", 1),
    ("Adjustable Disguise", 3),
    ("Adjustable Polymorph", 4),
    ("Aid", 2),
    ("Air Step", 2),
    ("Air Walk", 4),
    ("Alchemical Allocation", 2),
    ("Alter Self", 2),
    ("Amplify Elixir", 3),
    ("Analyze Dweomer", 6),
    ("Anchored Step", 3),
    ("Ant Haul", 1),
    ("Arcane Eye", 4),
    ("Arcane Sight", 3),
    ("Aura Sight", 3),
    ("Barkskin", 2),
    ("Bear's Endurance", 2),
    ("Beast Shape I", 3),
    ("Beast Shape II", 4),
    ("Beast Shape III", 5),
    ("Beast Shape IV", 6),
    ("Blood Armor", 2),
    ("Blood Sentinel", 3),
    ("Bloodhound", 3),
    ("Blur", 2),
    ("Blurred Movement", 1),
    ("Body Capacitance", 1),
    ("Bomber's Eye", 1),
    ("Bull's Strength", 2),
    ("Cat's Grace", 2),
    ("Comprehend Languages", 1),
    ("Contact Other Plane", 5),
    ("Crafter's Fortune", 1),
    ("Cure Critical Wounds", 4),
    ("Cure Light Wounds", 1),
    ("Cure Moderate Wounds", 2),
    ("Cure Serious Wounds", 3),
    ("Darkvision", 2),
    ("Death Ward", 4),
    ("Delay Poison", 2),
    ("Delayed Consumption", 5),
    ("Detect Secret Doors", 1),
    ("Detect Thoughts", 2),
    ("Detect Undead", 1),
    ("Detonate", 4),
    ("Disable Construct", 3),
    ("Discern Lies", 4),
    ("Disguise Self", 1),
    ("Displacement", 3),
    ("Draconic Reservoir", 3),
    ("Dragon's Breath", 4),
    ("Dream", 5),
    ("Eagle's Splendor", 2),
    ("Elemental Aura", 3),
    ("Elemental Body I", 4),
    ("Elemental Body II", 5),
    ("Elemental Body III", 6),
    ("Elemental Touch", 2),
    ("Elude Time", 5),
    ("Enchantment Foil", 4),
    ("Endure Elements", 1),
    ("Enlarge Person", 1),
    ("Expeditious Retreat", 1),
    ("Extreme Flexibility", 2),
    ("Eyebite", 6),
    ("Eyes of the Void", 4),
    ("False Life", 2),
    ("Fire Breath", 2),
    ("Fire Shield", 4),
    ("Fluid Form", 4),
    ("Fly", 3),
    ("Focused Scrutiny", 2),
    ("Form of the Dragon I", 6),
    ("Fox's Cunning", 2),
    ("Freedom of Movement", 4),
    ("Gaseous Form", 3),
    ("Giant Form I", 6),
    ("Haste", 3),
    ("Heal", 6),
    ("Heightened Awareness", 1),
    ("Heroic Fortune", 2),
    ("Heroism", 3),
    ("Identify", 1),
    ("Investigative Mind", 2),
    ("Invisibility", 2),
    ("Invisibility (Greater)", 4),
    ("Invisibility Alarm", 1),
    ("Jump", 1),
    ("Keen Senses", 1),
    ("Levitate", 2),
    ("Long Arm", 1),
    ("Magic Jar", 5),
    ("Mislead", 6),
    ("Monkey Fish", 1),
    ("Nauseating Trail", 3),
    ("Negate Aroma", 1),
    ("Neutralize Poison", 4),
    ("Nightmare", 5),
    ("Nondetection", 3),
    ("Overland Flight", 5),
    ("Owl's Wisdom", 2),
    ("Perceive Cues", 2),
    ("Persistent Vigor", 4),
    ("Phantom Blood", 1),
    ("Planar Adaptation", 5),
    ("Plant Shape I", 5),
    ("Plant Shape II", 6),
    ("Polymorph", 5),
    ("Protection from Arrows", 2),
    ("Protection from Energy", 3),
    ("Rage", 3),
    ("Reduce Person", 1),
    ("Remove Blindness/Deafness", 3),
    ("Remove Curse", 3),
    ("Remove Disease", 3),
    ("Resist Energy", 2),
    ("Restoration", 4),
    ("Restoration (Lesser)", 2),
    ("Resurgent Transformation", 5),
    ("See Invisibility", 2),
    ("Seek Thoughts", 3),
    ("Sending", 5),
    ("Shadow Walk", 6),
    ("Shield", 1),
    ("Sonic Form", 6),
    ("Spell Immunity", 4),
    ("Spell Resistance", 5),
    ("Spider Climb", 2),
    ("Statue", 6),
    ("Stone Fist", 1),
    ("Stoneskin", 4),
    ("Thorn Body", 3),
    ("Tongues", 3),
    ("Touch of the Sea", 1),
    ("Transformation", 6),
    ("Transmute Potion to Poison", 2),
    ("True Seeing", 6),
    ("True Strike", 1),
    ("Twin Form", 6),
    ("Unbearable Brightness", 4),
    ("Undetectable Alignment", 2),
    ("Universal Formula", 4),
    ("Vomit Swarm", 2),
    ("Water Breathing", 3),
    ("Wind Walk", 6),
];

/// Looks up a spell's Alchemist-specific extract level (1-6). `None`
/// means the named spell is not on the real Alchemist formula list at
/// all -- either it's not a real spell, or it's a real spell no
/// Alchemist/Investigator can ever know.
pub fn alchemist_spell_level(spell_key: &str) -> Option<u8> {
    ALCHEMIST_SPELL_LIST
        .iter()
        .find(|(key, _)| *key == spell_key)
        .map(|(_, level)| *level)
}

#[cfg(test)]
mod tests {
    use super::{alchemist_spell_level, ALCHEMIST_SPELL_LIST};

    #[test]
    fn contains_exactly_147_records_matching_the_raw_corpus_token_count() {
        assert_eq!(ALCHEMIST_SPELL_LIST.len(), 147);
    }

    #[test]
    fn level_breakdown_matches_the_raw_corpus_exactly() {
        let mut counts = [0u32; 7];
        for (_, level) in ALCHEMIST_SPELL_LIST {
            counts[*level as usize] += 1;
        }
        assert_eq!(counts, [0, 28, 36, 29, 23, 15, 16]);
    }

    #[test]
    fn no_duplicate_spell_names() {
        let mut names: Vec<&str> = ALCHEMIST_SPELL_LIST.iter().map(|(name, _)| *name).collect();
        let original_len = names.len();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), original_len, "expected zero duplicate spell names");
    }

    #[test]
    fn a_genuinely_new_alchemist_only_spell_resolves_its_real_level() {
        assert_eq!(alchemist_spell_level("Bomber's Eye"), Some(1));
        assert_eq!(alchemist_spell_level("Absorbing Touch"), Some(3));
    }

    #[test]
    fn a_mod_grafted_core_rulebook_spell_resolves_with_the_mod_suffix_stripped() {
        assert_eq!(alchemist_spell_level("Cure Light Wounds"), Some(1));
        assert_eq!(alchemist_spell_level("Haste"), Some(3));
        assert_eq!(alchemist_spell_level("Restoration"), Some(4));
        // The raw corpus key (with .MOD still attached) must NOT resolve --
        // proves the lookup is keyed on the real spell name, not the raw
        // corpus record name.
        assert_eq!(alchemist_spell_level("Cure Light Wounds.MOD"), None);
    }

    /// Regression guard for the `CLASSES:` substring-matching bug that
    /// shipped this list 17 records short. Each of these three names
    /// Alchemist in a comma group where Alchemist is NOT last, so the
    /// raw line contains no `Alchemist=` substring at all:
    /// `Ant Haul` is `CLASSES:Alchemist,Cleric,Druid,Ranger,Sorcerer,Wizard=1`,
    /// `Vomit Swarm` is `CLASSES:Alchemist,Witch=2`, and
    /// `Stone Fist` is `CLASSES:Alchemist,Druid,Sorcerer,Wizard=1`.
    #[test]
    fn spells_tagged_mid_list_in_their_classes_group_are_present() {
        assert_eq!(alchemist_spell_level("Ant Haul"), Some(1));
        assert_eq!(alchemist_spell_level("Vomit Swarm"), Some(2));
        assert_eq!(alchemist_spell_level("Stone Fist"), Some(1));
    }

    /// Guards the ACG half added by the book-scope widening (task #32).
    /// All three name Alchemist FIRST in their comma group, so their raw
    /// lines carry no `Alchemist=` substring at all:
    /// `Adhesive Blood` and `Blood Armor` are
    /// `CLASSES:Alchemist,Bloodrager,Sorcerer,Witch,Wizard=2`, and
    /// `Long Arm` is
    /// `CLASSES:Alchemist,Bloodrager,Magus,Sorcerer,Witch,Wizard=1`.
    #[test]
    fn acg_book_extracts_are_present() {
        assert_eq!(alchemist_spell_level("Adhesive Blood"), Some(2));
        assert_eq!(alchemist_spell_level("Blood Armor"), Some(2));
        assert_eq!(alchemist_spell_level("Long Arm"), Some(1));
    }

    #[test]
    fn an_unrelated_or_nonexistent_spell_resolves_to_none() {
        assert_eq!(alchemist_spell_level("Fireball"), None);
        assert_eq!(alchemist_spell_level("Not A Real Spell"), None);
    }
}
