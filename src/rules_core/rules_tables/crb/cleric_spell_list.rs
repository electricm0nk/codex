//! PF1 CRB Cleric spell list — per-class spell-level overrides.
//!
//! Mirrors `ranger_spell_list.rs`/`paladin_spell_list.rs`/`sorcerer_spell_list.rs`'s
//! own doc comment exactly, substituting Cleric: `crb::spell_list::SPELL_LIST`'s
//! `level` field is the MINIMUM spell level across every class named in the
//! corpus's `CLASSES:` tag for that record, not necessarily the
//! Cleric-specific level. This table re-parses the same corpus record's raw
//! `CLASSES:` tag directly (`core_rulebook/cr_spells.lst`), isolating only
//! the Cleric-specific level for each of the 236 real records that name
//! Cleric at all: 12 orisons (0th level), 31 first-level, 32 second-level,
//! 34 third-level, 27 fourth-level, 28 fifth-level, 26 sixth-level, 18
//! seventh-level, 17 eighth-level, 11 ninth-level -- matching the real PF1
//! Cleric full 9th-level caster ceiling (verified via a direct parse of the
//! raw `CLASSES:` token). Every `key` here is spot-checked against a
//! real, exact `crb::spell_list::SPELL_LIST` key -- this is a strict
//! subset of the 652 CRB spell records, never an invented name.
//!
//! This list is the GENERAL Cleric spell list only (spells any cleric may
//! prepare regardless of domain). It deliberately does NOT include
//! domain-specific spells granted only through a chosen domain's domain
//! spell list (a separate corpus concept, `DOMAIN:<Domain>=<level>` tags)
//! -- domain selection and domain spell-list contents remain their own
//! unproven burden (`class_feature.cleric.domain_powers.unsupported`),
//! deliberately out of scope for this list.
//!
//! Regenerate by re-parsing `cr_spells.lst`'s `CLASSES:` tag for any
//! `|`-separated group whose name list contains "Cleric", taking that
//! group's own level (not the record's collapsed minimum), if the corpus
//! changes.

/// (spell key, Cleric-specific spell level 0-9). A real CRB Cleric may only
/// prepare a spell that appears in this table (from the general list; a
/// domain spell slot's own contents are a separate, unproven burden),
/// subject to the character's own spell-level access ceiling for their
/// cleric level -- see `cleric_spell_level` for the lookup helper.
pub const CLERIC_SPELL_LIST: &[(&str, u8)] = &[
    ("Aid", 2),
    ("Air Walk", 4),
    ("Align Weapon", 2),
    ("Animate Dead", 3),
    ("Animate Objects", 6),
    ("Antilife Shell", 6),
    ("Antimagic Field", 8),
    ("Astral Projection", 9),
    ("Atonement", 5),
    ("Augury", 2),
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
    ("Blindness/Deafness", 3),
    ("Break Enchantment", 5),
    ("Breath of Life", 5),
    ("Bull's Strength", 2),
    ("Bull's Strength (Mass)", 6),
    ("Calm Emotions", 2),
    ("Cause Fear", 1),
    ("Chaos Hammer", 4),
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
    ("Endure Elements", 1),
    ("Energy Drain", 9),
    ("Enthrall", 2),
    ("Entropic Shield", 1),
    ("Ethereal Jaunt", 7),
    ("Etherealness", 9),
    ("Find Traps", 2),
    ("Find the Path", 6),
    ("Fire Storm", 8),
    ("Flame Strike", 5),
    ("Forbiddance", 6),
    ("Freedom of Movement", 4),
    ("Gate", 9),
    ("Geas/Quest", 6),
    ("Gentle Repose", 2),
    ("Giant Vermin", 4),
    ("Glyph of Warding", 3),
    ("Glyph of Warding (Greater)", 6),
    ("Guidance", 0),
    ("Hallow", 5),
    ("Harm", 6),
    ("Heal", 6),
    ("Heal (Mass)", 9),
    ("Helping Hand", 3),
    ("Heroes' Feast", 6),
    ("Hide from Undead", 1),
    ("Hold Person", 2),
    ("Holy Aura", 8),
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
    ("Invisibility Purge", 3),
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
    ("Mark of Justice", 5),
    ("Meld into Stone", 3),
    ("Mending", 0),
    ("Miracle", 9),
    ("Neutralize Poison", 4),
    ("Obscure Object", 3),
    ("Obscuring Mist", 1),
    ("Order's Wrath", 4),
    ("Owl's Wisdom", 2),
    ("Owl's Wisdom (Mass)", 6),
    ("Planar Ally", 6),
    ("Planar Ally (Greater)", 8),
    ("Planar Ally (Lesser)", 4),
    ("Plane Shift", 5),
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
    ("Restoration", 4),
    ("Restoration (Greater)", 7),
    ("Restoration (Lesser)", 2),
    ("Resurrection", 7),
    ("Righteous Might", 5),
    ("Sanctuary", 1),
    ("Scrying", 5),
    ("Scrying (Greater)", 7),
    ("Searing Light", 3),
    ("Sending", 4),
    ("Shatter", 2),
    ("Shield Other", 2),
    ("Shield of Faith", 1),
    ("Shield of Law", 8),
    ("Silence", 2),
    ("Slay Living", 5),
    ("Soul Bind", 9),
    ("Sound Burst", 2),
    ("Speak with Dead", 3),
    ("Spell Immunity", 4),
    ("Spell Immunity (Greater)", 8),
    ("Spell Resistance", 5),
    ("Spiritual Weapon", 2),
    ("Stabilize", 0),
    ("Status", 2),
    ("Stone Shape", 3),
    ("Storm of Vengeance", 9),
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
    ("True Resurrection", 9),
    ("True Seeing", 5),
    ("Undeath to Death", 6),
    ("Undetectable Alignment", 2),
    ("Unhallow", 5),
    ("Unholy Aura", 8),
    ("Unholy Blight", 4),
    ("Virtue", 0),
    ("Wall of Stone", 5),
    ("Water Breathing", 3),
    ("Water Walk", 3),
    ("Wind Walk", 6),
    ("Wind Wall", 3),
    ("Word of Chaos", 7),
    ("Word of Recall", 6),
    ("Zone of Truth", 2),
];

/// Looks up a spell's Cleric-specific spell level (0-9). `None` means the
/// named spell is not on the real CRB general Cleric spell list at all --
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
    use crate::rules_core::rules_tables::crb::spell_list::SPELL_LIST;

    #[test]
    fn cleric_spell_list_has_the_real_corpus_verified_count() {
        assert_eq!(CLERIC_SPELL_LIST.len(), 236);
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

    #[test]
    fn every_cleric_spell_key_is_a_real_spell_list_entry() {
        for (key, _) in CLERIC_SPELL_LIST {
            assert!(
                SPELL_LIST.iter().any(|entry| entry.key == *key),
                "{key} is not a real SPELL_LIST key"
            );
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
        assert_eq!(count_at(0), 12);
        assert_eq!(count_at(1), 31);
        assert_eq!(count_at(2), 32);
        assert_eq!(count_at(3), 34);
        assert_eq!(count_at(4), 27);
        assert_eq!(count_at(5), 28);
        assert_eq!(count_at(6), 26);
        assert_eq!(count_at(7), 18);
        assert_eq!(count_at(8), 17);
        assert_eq!(count_at(9), 11);
    }
}
