//! PF1 Advanced Player's Guide Alchemist formula spell list (deepening
//! 2026-07-26, task #8: Investigator spellcasting subsystem).
//!
//! Real PF1 rule text (`acg_abilities_class.lst`'s own Investigator
//! Alchemy DESC): "An investigator uses the alchemist formula list
//! (Pathfinder RPG Advanced Player's Guide 32) to determine the extracts
//! he can know" -- Investigator's `SPELLLIST:1|Alchemist` corpus token
//! confirms this is the same list, not merely a similar one. This module
//! is the shared list both Investigator's (this closure) and Alchemist's
//! own (task #4, not built here) spellcasting can consume.
//!
//! Source: every real record in `apg_spells.lst` whose `CLASSES:` token
//! names Alchemist in any pipe-separated group, extracted via a direct
//! parse of the raw corpus file (not hand-transcribed) -- **121 total**.
//! Two shapes exist in the corpus:
//!
//! - 30 genuinely new Alchemist-only spells named directly on their own
//!   line (Absorbing Touch, Alchemical Allocation, Amplify Elixir, Ant
//!   Haul, Bloodhound, Bomber's Eye, Crafter's Fortune, Delayed
//!   Consumption, Detonate, Draconic Reservoir, Dragon's Breath,
//!   Elemental Aura, Elemental Touch, Elude Time, Fire Breath, Fluid
//!   Form, Heroic Fortune, Keen Senses, Negate Aroma, Perceive Cues,
//!   Planar Adaptation, Resurgent Transformation, Seek Thoughts, Stone
//!   Fist, Thorn Body, Touch of the Sea, Transmute Potion to Poison,
//!   Twin Form, Universal Formula, Vomit Swarm).
//! - 91 `.MOD` records that graft `Alchemist=N` onto an existing Core
//!   Rulebook spell's `CLASSES:` token (e.g. `Cure Light Wounds.MOD`).
//!   The real spell name is the base name with `.MOD` stripped; the
//!   level here is read directly off that same `.MOD` line's own
//!   `CLASSES:` token -- no cross-file lookup into `cr_spells.lst` is
//!   needed for level purposes (only for school/description metadata,
//!   which this `(name, level)`-only shape doesn't carry, mirroring
//!   `CLERIC_SPELL_LIST`/`BARD_SPELL_LIST`'s own minimal shape).
//!
//! Deliberately scoped to `apg_spells.lst` alone, not the wider PCGen
//! corpus (Ultimate Magic/Combat/Wilderness/Intrigue/Horror Adventures
//! also tag `Alchemist=N` on further spells) -- the real rule text's own
//! citation of "Advanced Player's Guide" as the canonical source matches
//! the same single-book-source discipline every other spell list in this
//! codebase already uses (Wizard/Cleric/Bard/Druid/Ranger are all
//! CRB-only, not aggregated across every later splatbook that later
//! added more spells to their lists).
//!
//! **Open scope question, deliberately NOT decided here.** That
//! single-book claim was true when written but is no longer: as of task
//! #23 `WITCH_SPELL_LIST` spans two books (250 APG + 74 ACG). Parsing
//! `acg_spells.lst` for Alchemist yields a further **26** records, all
//! non-`.MOD` new ACG spells, from a book this repo does ingest -- the
//! same shape as Witch's 74. So either Witch over-includes or the other
//! seven lists under-include, and the answer changes CRB lists too
//! (Cleric/Druid/Bard/etc. also gain APG/ACG additions). That is a
//! repo-wide policy call for the swarm lead, not a parsing bug, so this
//! list stays APG-only pending that ruling. See
//! `docs/release/v0.6/investigator-alchemist-spell-list-scoping.md` for
//! the original extraction record; its level breakdown predates the
//! correction below and is superseded by the figures here.
//!
//! Level breakdown, re-derived: **20 / 30 / 23 / 18 / 15 / 15** across
//! extract levels 1-6, summing to 121. Per-file ceiling check:
//! `grep -c Alchemist apg_spells.lst` is 122, so 121 is plausible and
//! not over-counted.
//!
//! **Corpus reachability: all 121 resolve** against this repo's own
//! ingested `data/corpus/` spell records (1,075 keys) -- unlike
//! `BLOODRAGER_SPELL_LIST`, this list has no unreachable entries.
//!
//! Regenerate by parsing `apg_spells.lst`'s `CLASSES:` token -- split the
//! body on `|`, `rpartition` each group on `=`, then membership-test the
//! comma-separated name list -- and strip a trailing `.MOD` from the
//! record's own name column. Never substring-match `Alchemist=`.

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
    ("Aid", 2),
    ("Air Walk", 4),
    ("Alchemical Allocation", 2),
    ("Alter Self", 2),
    ("Amplify Elixir", 3),
    ("Analyze Dweomer", 6),
    ("Ant Haul", 1),
    ("Arcane Eye", 4),
    ("Arcane Sight", 3),
    ("Barkskin", 2),
    ("Bear's Endurance", 2),
    ("Beast Shape I", 3),
    ("Beast Shape II", 4),
    ("Beast Shape III", 5),
    ("Beast Shape IV", 6),
    ("Bloodhound", 3),
    ("Blur", 2),
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
    ("Endure Elements", 1),
    ("Enlarge Person", 1),
    ("Expeditious Retreat", 1),
    ("Eyebite", 6),
    ("False Life", 2),
    ("Fire Breath", 2),
    ("Fire Shield", 4),
    ("Fluid Form", 4),
    ("Fly", 3),
    ("Form of the Dragon I", 6),
    ("Fox's Cunning", 2),
    ("Freedom of Movement", 4),
    ("Gaseous Form", 3),
    ("Giant Form I", 6),
    ("Haste", 3),
    ("Heal", 6),
    ("Heroic Fortune", 2),
    ("Heroism", 3),
    ("Identify", 1),
    ("Invisibility", 2),
    ("Invisibility (Greater)", 4),
    ("Jump", 1),
    ("Keen Senses", 1),
    ("Levitate", 2),
    ("Magic Jar", 5),
    ("Mislead", 6),
    ("Negate Aroma", 1),
    ("Neutralize Poison", 4),
    ("Nightmare", 5),
    ("Nondetection", 3),
    ("Overland Flight", 5),
    ("Owl's Wisdom", 2),
    ("Perceive Cues", 2),
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
    fn contains_exactly_121_records_matching_the_raw_corpus_token_count() {
        assert_eq!(ALCHEMIST_SPELL_LIST.len(), 121);
    }

    #[test]
    fn level_breakdown_matches_the_raw_corpus_exactly() {
        let mut counts = [0u32; 7];
        for (_, level) in ALCHEMIST_SPELL_LIST {
            counts[*level as usize] += 1;
        }
        assert_eq!(counts, [0, 20, 30, 23, 18, 15, 15]);
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

    #[test]
    fn an_unrelated_or_nonexistent_spell_resolves_to_none() {
        assert_eq!(alchemist_spell_level("Fireball"), None);
        assert_eq!(alchemist_spell_level("Not A Real Spell"), None);
    }
}
