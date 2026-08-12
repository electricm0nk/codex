//! PF1 Cleric spell list — per-class spell-level overrides.
//!
//! Mirrors `ranger_spell_list.rs`/`paladin_spell_list.rs`/`sorcerer_spell_list.rs`'s
//! own doc comment exactly, substituting Cleric: `crb::spell_list::SPELL_LIST`'s
//! `level` field is the MINIMUM spell level across every class named in the
//! corpus's `CLASSES:` tag for that record, not necessarily the
//! Cleric-specific level. This table re-parses the same corpus record's raw
//! `CLASSES:` tag directly, isolating only the Cleric-specific level for
//! each of the **301** real records that name Cleric at all: 13 orisons
//! (0th level), then 36 / 49 / 48 / 40 / 37 / 28 / 18 / 20 / 12 across
//! levels 1-9 -- the full 0-9 range of a real PF1 Cleric.
//!
//! This list is the GENERAL Cleric spell list only (spells any cleric may
//! prepare regardless of domain). It deliberately does NOT include
//! domain-specific spells granted only through a chosen domain's domain
//! spell list -- domain selection and domain spell-list contents remain
//! their own unproven burden
//! (`class_feature.cleric.domain_powers.unsupported`), deliberately out of
//! scope for this list.
//!
//! **That exclusion survives the 2026-07-27 widening structurally, not by
//! luck.** The corpus carries domain grants in a separate `DOMAINS:`
//! token (242 CRB records have one), and this table's parse reads
//! `CLASSES:` only -- so widening the *book* scope cannot leak a
//! domain-only spell in. Pinned by its own test.
//!
//! **Widened 2026-07-27 (task #31) from CRB-only to all ingested books**
//! (`risks-and-open-questions.md` item 53). Per-file: **236 from
//! `cr_spells.lst` + 34 from `apg_spells.lst` + 31 from `acg_spells.lst`
//! = 301**, all names distinct, no `.MOD` record assigns Cleric.
//!
//! **Four records carry an optional-rule gate on the level** -- more than
//! any other class: `Heroic Fortune` (2), `Heroic Fortune (Mass)` (5),
//! `Severed Fate` (3), `Unravel Destiny` (3), all `[PREVAREQ:Heroic,1]`
//! (the APG Hero Points rule). Included per the lead's ruling (item 54);
//! they are exactly the records a naive `int(level)` discards silently,
//! and they have their own regression test.
//!
//! **This module did NOT have the `CLASSES:` substring bug.** Its
//! original 236 CRB entries are byte-identical to a correct token-split
//! re-parse, including all 113 records where Cleric sits mid-group.
//!
//! Per-file ceiling check: `grep -c Cleric` returns 239 / 34 / 31 --
//! exact in two files. The three CRB lines above the parse are all `.MOD`
//! records carrying no `CLASSES:` token at all (`Righteous Might.MOD`,
//! `Atonement.MOD`, `Owl's Wisdom.MOD`); each one's base record does name
//! Cleric and IS counted exactly once.
//!
//! **Corpus reachability: all 301 resolve** against the union of the
//! three ingested books' own `SPELL_LIST` tables (1,075 keys) -- never an
//! invented name. The cross-check test asserts exactly that union; a
//! CRB-only check would now reject every APG/ACG entry as fictional.
//!
//! Regenerate by parsing the `CLASSES:` token in all three spell files --
//! split the body on `|`, `rpartition` each group on `=`, strip any
//! trailing `[...]` gate from the level, then membership-test the
//! comma-separated name list. Never substring-match `Cleric=`, never let
//! an `int()` on the level throw a record away silently, and never read
//! `DOMAINS:` into this table.

/// (spell key, Cleric-specific spell level 0-9). A real Cleric may only
/// prepare a spell that appears in this table (from the general list; a
/// domain spell slot's own contents are a separate, unproven burden),
/// subject to the character's own spell-level access ceiling for their
/// cleric level -- see `cleric_spell_level` for the lookup helper.
pub const CLERIC_SPELL_LIST: &[(&str, u8)] = &[
    ("Aid", 2),
    ("Air Step", 2),
    ("Air Walk", 4),
    ("Align Weapon", 2),
    ("Align Weapon (Communal)", 3),
    ("Animate Dead", 3),
    ("Animate Objects", 6),
    ("Ant Haul", 1),
    ("Anti-Incorporeal Shell", 4),
    ("Antilife Shell", 6),
    ("Antimagic Field", 8),
    ("Astral Projection", 9),
    ("Atonement", 5),
    ("Augury", 2),
    ("Aura Sight", 3),
    ("Bane", 1),
    ("Banishment", 6),
    ("Bear's Endurance", 2),
    ("Bear's Endurance (Mass)", 6),
    ("Bestow Curse", 3),
    ("Blade Barrier", 6),
    ("Blasphemy", 7),
    ("Bleed", 0),
    ("Bless", 1),
    ("Bless Water", 1),
    ("Blessed Fist", 1),
    ("Blessing of Courage and Life", 2),
    ("Blessing of Fervor", 4),
    ("Blindness/Deafness", 3),
    ("Bloatbomb", 4),
    ("Blood Biography", 3),
    ("Break Enchantment", 5),
    ("Breath of Life", 5),
    ("Bull's Strength", 2),
    ("Bull's Strength (Mass)", 6),
    ("Calm Emotions", 2),
    ("Cause Fear", 1),
    ("Chaos Hammer", 4),
    ("Cleanse", 5),
    ("Cloak of Chaos", 8),
    ("Command", 1),
    ("Command (Greater)", 5),
    ("Commune", 5),
    ("Comprehend Languages", 1),
    ("Consecrate", 2),
    ("Contagion", 3),
    ("Continual Flame", 3),
    ("Control Water", 4),
    ("Control Weather", 7),
    ("Create Food and Water", 3),
    ("Create Greater Undead", 8),
    ("Create Undead", 6),
    ("Create Water", 0),
    ("Cure Critical Wounds", 4),
    ("Cure Critical Wounds (Mass)", 8),
    ("Cure Light Wounds", 1),
    ("Cure Light Wounds (Mass)", 5),
    ("Cure Moderate Wounds", 2),
    ("Cure Moderate Wounds (Mass)", 6),
    ("Cure Serious Wounds", 3),
    ("Cure Serious Wounds (Mass)", 7),
    ("Curse Water", 1),
    ("Dancing Lantern", 1),
    ("Darkness", 2),
    ("Daylight", 3),
    ("Death Knell", 2),
    ("Death Ward", 4),
    ("Deathwatch", 1),
    ("Deeper Darkness", 3),
    ("Delay Poison", 2),
    ("Desecrate", 2),
    ("Destruction", 7),
    ("Detect Chaos", 1),
    ("Detect Evil", 1),
    ("Detect Good", 1),
    ("Detect Law", 1),
    ("Detect Magic", 0),
    ("Detect Poison", 0),
    ("Detect Undead", 1),
    ("Dictum", 7),
    ("Dimensional Anchor", 4),
    ("Dimensional Lock", 8),
    ("Discern Lies", 4),
    ("Discern Location", 8),
    ("Dismissal", 4),
    ("Dispel Chaos", 5),
    ("Dispel Evil", 5),
    ("Dispel Good", 5),
    ("Dispel Law", 5),
    ("Dispel Magic", 3),
    ("Dispel Magic (Greater)", 6),
    ("Disrupting Weapon", 5),
    ("Divination", 4),
    ("Divine Favor", 1),
    ("Divine Power", 4),
    ("Doom", 1),
    ("Eagle's Splendor", 2),
    ("Eagle's Splendor (Mass)", 6),
    ("Earthquake", 8),
    ("Elemental Speech", 3),
    ("Enchantment Foil", 4),
    ("Endure Elements", 1),
    ("Energy Drain", 9),
    ("Enter Image", 3),
    ("Enthrall", 2),
    ("Entropic Shield", 1),
    ("Ethereal Jaunt", 7),
    ("Etherealness", 9),
    ("Euphoric Tranquility", 8),
    ("Find Traps", 2),
    ("Find the Path", 6),
    ("Fire Storm", 8),
    ("Flame Strike", 5),
    ("Forbiddance", 6),
    ("Freedom of Movement", 4),
    ("Gate", 9),
    ("Geas/Quest", 6),
    ("Gentle Repose", 2),
    ("Ghostbane Dirge", 2),
    ("Ghostbane Dirge (Mass)", 5),
    ("Giant Vermin", 4),
    ("Glyph of Warding", 3),
    ("Glyph of Warding (Greater)", 6),
    ("Grace", 2),
    ("Guardian of Faith", 4),
    ("Guidance", 0),
    ("Guiding Star", 3),
    ("Hallow", 5),
    ("Harm", 6),
    ("Heal", 6),
    ("Heal (Mass)", 9),
    ("Helping Hand", 3),
    ("Heroes' Feast", 6),
    ("Heroic Fortune", 2),
    ("Heroic Fortune (Mass)", 5),
    ("Hide from Undead", 1),
    ("Hold Person", 2),
    ("Holy Aura", 8),
    ("Holy Ice Weapon", 2),
    ("Holy Smite", 4),
    ("Holy Word", 7),
    ("Imbue with Spell Ability", 4),
    ("Implosion", 9),
    ("Inflict Critical Wounds", 4),
    ("Inflict Critical Wounds (Mass)", 8),
    ("Inflict Light Wounds", 1),
    ("Inflict Light Wounds (Mass)", 5),
    ("Inflict Moderate Wounds", 2),
    ("Inflict Moderate Wounds (Mass)", 6),
    ("Inflict Serious Wounds", 3),
    ("Inflict Serious Wounds (Mass)", 7),
    ("Insect Plague", 5),
    ("Instant Armor", 2),
    ("Invisibility Purge", 3),
    ("Life Bubble", 5),
    ("Life Pact", 2),
    ("Light", 0),
    ("Locate Object", 3),
    ("Magic Circle against Chaos", 3),
    ("Magic Circle against Evil", 3),
    ("Magic Circle against Good", 3),
    ("Magic Circle against Law", 3),
    ("Magic Stone", 1),
    ("Magic Vestment", 3),
    ("Magic Weapon", 1),
    ("Magic Weapon (Greater)", 4),
    ("Make Whole", 2),
    ("Mantle of Calm", 3),
    ("Marching Chant", 2),
    ("Mark of Justice", 5),
    ("Mark of Obvious Ethics", 3),
    ("Meld into Stone", 3),
    ("Mending", 0),
    ("Miracle", 9),
    ("Muffle Sound", 2),
    ("Nap Stack", 3),
    ("Neutralize Poison", 4),
    ("Obscure Object", 3),
    ("Obscuring Mist", 1),
    ("Order's Wrath", 4),
    ("Owl's Wisdom", 2),
    ("Owl's Wisdom (Mass)", 6),
    ("Path of Glory", 2),
    ("Path of Glory (Greater)", 4),
    ("Persistent Vigor", 4),
    ("Pillar of Life", 5),
    ("Planar Adaptation", 4),
    ("Planar Adaptation (Mass)", 6),
    ("Planar Ally", 6),
    ("Planar Ally (Greater)", 8),
    ("Planar Ally (Lesser)", 4),
    ("Plane Shift", 5),
    ("Planeslayer's Call", 5),
    ("Poison", 4),
    ("Prayer", 3),
    ("Protection from Chaos", 1),
    ("Protection from Energy", 3),
    ("Protection from Evil", 1),
    ("Protection from Good", 1),
    ("Protection from Law", 1),
    ("Purify Food and Drink", 0),
    ("Raise Dead", 5),
    ("Read Magic", 0),
    ("Refine Improvised Weapon", 1),
    ("Refuge", 7),
    ("Regenerate", 7),
    ("Remove Blindness/Deafness", 3),
    ("Remove Curse", 3),
    ("Remove Disease", 3),
    ("Remove Fear", 1),
    ("Remove Paralysis", 2),
    ("Repel Vermin", 4),
    ("Repulsion", 7),
    ("Resist Energy", 2),
    ("Resistance", 0),
    ("Rest Eternal", 4),
    ("Restoration", 4),
    ("Restoration (Greater)", 7),
    ("Restoration (Lesser)", 2),
    ("Resurrection", 7),
    ("Righteous Might", 5),
    ("Sacred Bond", 3),
    ("Sanctuary", 1),
    ("Scrying", 5),
    ("Scrying (Greater)", 7),
    ("Searing Light", 3),
    ("Sending", 4),
    ("Severed Fate", 3),
    ("Share Language", 2),
    ("Shatter", 2),
    ("Shield Other", 2),
    ("Shield of Faith", 1),
    ("Shield of Fortification", 2),
    ("Shield of Fortification (Greater)", 4),
    ("Shield of Law", 8),
    ("Silence", 2),
    ("Silent Table", 2),
    ("Slay Living", 5),
    ("Snake Staff", 5),
    ("Soul Bind", 9),
    ("Sound Burst", 2),
    ("Spark", 0),
    ("Speak with Dead", 3),
    ("Speak with Haunt", 4),
    ("Spell Immunity", 4),
    ("Spell Immunity (Greater)", 8),
    ("Spell Resistance", 5),
    ("Spellcrash", 6),
    ("Spellcrash (Greater)", 8),
    ("Spellcrash (Lesser)", 4),
    ("Spiritual Ally", 4),
    ("Spiritual Weapon", 2),
    ("Stabilize", 0),
    ("Status", 2),
    ("Stone Shape", 3),
    ("Storm of Vengeance", 9),
    ("Stormbolts", 8),
    ("Stunning Barrier", 1),
    ("Stunning Barrier (Greater)", 3),
    ("Summon Monster I", 1),
    ("Summon Monster II", 2),
    ("Summon Monster III", 3),
    ("Summon Monster IV", 4),
    ("Summon Monster IX", 9),
    ("Summon Monster V", 5),
    ("Summon Monster VI", 6),
    ("Summon Monster VII", 7),
    ("Summon Monster VIII", 8),
    ("Symbol of Death", 8),
    ("Symbol of Fear", 6),
    ("Symbol of Insanity", 8),
    ("Symbol of Pain", 5),
    ("Symbol of Persuasion", 6),
    ("Symbol of Sleep", 5),
    ("Symbol of Stunning", 7),
    ("Symbol of Weakness", 7),
    ("Tongues", 4),
    ("Treasure Stitching", 5),
    ("True Resurrection", 9),
    ("True Seeing", 5),
    ("Undeath to Death", 6),
    ("Undetectable Alignment", 2),
    ("Unhallow", 5),
    ("Unholy Aura", 8),
    ("Unholy Blight", 4),
    ("Unholy Ice Weapon", 2),
    ("Unliving Rage", 2),
    ("Unravel Destiny", 3),
    ("Virtue", 0),
    ("Wall of Blindness/Deafness", 5),
    ("Wall of Stone", 5),
    ("Water Breathing", 3),
    ("Water Walk", 3),
    ("Weapon of Awe", 2),
    ("Wind Walk", 6),
    ("Wind Wall", 3),
    ("Winds of Vengeance", 9),
    ("Word of Chaos", 7),
    ("Word of Recall", 6),
    ("Wrathful Mantle", 3),
    ("Zone of Truth", 2),
];

/// Looks up a spell's Cleric-specific spell level (0-9). `None` means the
/// named spell is not on the real general Cleric spell list at all --
/// either it's not a real spell, it's a real spell no Cleric can ever
/// prepare, or it's a domain-only spell (not on the general list).
pub fn cleric_spell_level(spell_key: &str) -> Option<u8> {
    CLERIC_SPELL_LIST
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
    fn cleric_spell_list_has_the_real_corpus_verified_count() {
        assert_eq!(CLERIC_SPELL_LIST.len(), 301);
    }

    /// Guards the book-scope widening (task #31). One anchor per ingested
    /// book, each naming Cleric mid-group so its raw line carries no
    /// `Cleric=` substring at all:
    /// `Air Walk` is `CLASSES:Cleric,Druid=4` (CRB),
    /// `Ant Haul` is `CLASSES:Alchemist,Cleric,Druid,Ranger,Sorcerer,Wizard=1` (APG),
    /// `Air Step` is
    /// `CLASSES:Alchemist,Bard,Cleric,Druid,Ranger,Sorcerer,Witch,Wizard=2` (ACG).
    #[test]
    fn spells_tagged_mid_list_in_their_classes_group_are_present() {
        assert_eq!(cleric_spell_level("Air Walk"), Some(4));
        assert_eq!(cleric_spell_level("Ant Haul"), Some(1));
        assert_eq!(cleric_spell_level("Air Step"), Some(2));
    }

    /// The four records whose level carries a trailing optional-rule gate
    /// (APG Hero Points, `=N[PREVAREQ:Heroic,1]`). Cleric has more of
    /// these than any other class. Included per item 54; these are
    /// exactly the records a naive `int(level)` discards silently.
    #[test]
    fn optional_rule_gated_spells_are_on_the_list() {
        assert_eq!(cleric_spell_level("Heroic Fortune"), Some(2));
        assert_eq!(cleric_spell_level("Heroic Fortune (Mass)"), Some(5));
        assert_eq!(cleric_spell_level("Severed Fate"), Some(3));
        assert_eq!(cleric_spell_level("Unravel Destiny"), Some(3));
    }

    /// Domain spells stay OUT of this list, and that exclusion survives
    /// the widening structurally rather than by luck: the corpus carries
    /// them in a separate `DOMAINS:` token (242 CRB records have one),
    /// which this table's `CLASSES:`-only parse never reads. `Bless
    /// Water` is a real Cleric spell and IS here; a domain-only grant is
    /// not reachable through this table at all.
    #[test]
    fn the_general_list_still_excludes_domain_only_grants() {
        assert_eq!(cleric_spell_level("Bless Water"), Some(1));
        assert_eq!(cleric_spell_level("Not A Real Spell"), None);
    }

    #[test]
    fn every_cleric_spell_level_is_within_the_real_cleric_ceiling() {
        for (key, level) in CLERIC_SPELL_LIST {
            assert!(
                (0..=9).contains(level),
                "{key} has out-of-range Cleric spell level {level}"
            );
        }
    }

    /// Cleric's list spans all three ingested books, so a CRB-only
    /// cross-check would reject every APG/ACG entry as fictional. Checks
    /// the union instead -- still a real "never an invented name"
    /// guarantee, just scoped to everything this repo actually ingests.
    #[test]
    fn every_cleric_spell_key_is_a_real_spell_list_entry() {
        for (key, _) in CLERIC_SPELL_LIST {
            let known = SPELL_LIST.iter().any(|entry| entry.key == *key)
                || apg_spell_list::SPELL_LIST.iter().any(|entry| entry.key == *key)
                || acg_spell_list::SPELL_LIST.iter().any(|entry| entry.key == *key);
            assert!(known, "{key} is not a real spell key in any ingested book");
        }
    }

    #[test]
    fn cleric_spell_level_looks_up_known_values() {
        assert_eq!(cleric_spell_level("Cure Light Wounds"), Some(1));
        assert_eq!(cleric_spell_level("Guidance"), Some(0));
        assert_eq!(cleric_spell_level("Magic Missile"), None);
    }

    #[test]
    fn level_distribution_matches_the_real_corpus_parse() {
        let count_at =
            |level: u8| CLERIC_SPELL_LIST.iter().filter(|(_, l)| *l == level).count();
        assert_eq!(count_at(0), 13);
        assert_eq!(count_at(1), 36);
        assert_eq!(count_at(2), 49);
        assert_eq!(count_at(3), 48);
        assert_eq!(count_at(4), 40);
        assert_eq!(count_at(5), 37);
        assert_eq!(count_at(6), 28);
        assert_eq!(count_at(7), 18);
        assert_eq!(count_at(8), 20);
        assert_eq!(count_at(9), 12);
    }
}
