//! APG Inquisitor spell list — one `(spell name, Inquisitor spell level)`
//! entry per real corpus record.
//!
//! Source: every record whose `CLASSES:` token names Inquisitor in any of
//! its comma-separated class groups, across the books this repo ingests:
//! `apg_spells.lst` (199 raw records) and `acg_spells.lst` (21 raw
//! records). `cr_spells.lst` names Inquisitor zero times (the class
//! postdates the CRB, the same shape as Witch/Alchemist). Unlike Hunter
//! (`ACG_HUNTER...` reuses Ranger's own list) and Oracle
//! (`SPELLLIST:2|Cleric|Oracle` reuses Cleric's), the real
//! `CLASS:Inquisitor` record in `apg_classes.lst` carries no `SPELLLIST:`
//! token at all — Inquisitor has its own, independently-tagged spell
//! list, so no existing list module can be reused here; this one had to
//! be built from a fresh corpus parse.
//!
//! **220 raw `CLASSES:Inquisitor` matches, 219 unique real spells** after
//! collapsing one genuine same-level duplicate (`Resounding Blow`, tagged
//! both as a full new record AND as a `.MOD` patch on itself, both at
//! level 5 — see `.MOD` note below). Levels 0-6, split
//! **15 / 38 / 43 / 44 / 35 / 24 / 20**, summing to 219.
//!
//! # Parsing `CLASSES:` correctly — the same two comma/bracket bugs
//! `witch_spell_list`/`alchemist_spell_list` document, plus a third
//!
//! **Bug 1 (comma-group membership, not `Inquisitor=` substring-match):**
//! a `CLASSES:` token is pipe-separated groups, each
//! `Name1,Name2,...=Level`; the level belongs to the WHOLE comma group.
//! `CLASSES:Cleric,Inquisitor,Sorcerer,Wizard=3` ("Blood Biography") and
//! `CLASSES:Alchemist,Inquisitor,Ranger,Witch=2` ("Perceive Cues") both
//! tag Inquisitor mid-group, so a `CLASSES:.*Inquisitor=` grep finds
//! neither. Always split on `|`, then split each group on `,`, then
//! membership-test `Inquisitor`.
//!
//! **Bug 2 (trailing bracketed prereq on the level):** a level may carry
//! a trailing `[PREVAREQ:...]` gate that a naive `int(level)` throws on,
//! silently discarding the whole record if the exception is swallowed.
//! No Inquisitor-tagged spell in the ingested books actually carries this
//! shape (checked directly), but the parse still strips any trailing
//! `[...]` defensively, matching the discipline every sibling list module
//! uses.
//!
//! **Bug 3 (`.MOD` records, this list's own dominant shape — 158 of 220
//! raw matches, ~72%):** most Inquisitor spells are not new records at
//! all; they are `.MOD` lines that graft `CLASSES:Inquisitor=N` onto an
//! existing Core Rulebook spell (e.g. `Bless.MOD`, `Cure Light
//! Wounds.MOD`), the same shape `alchemist_spell_list`'s own doc comment
//! documents (91 of its 147). The real spell name is the base name with
//! the trailing `.MOD` stripped; the level is read directly off that same
//! `.MOD` line's own `CLASSES:` token, no cross-file lookup needed. A
//! parser that keeps the raw `"Bless.MOD"` key would silently fail every
//! known-spell lookup against it, since nothing else in this codebase
//! ever spells a spell's name with a trailing `.MOD`.
//!
//! Regenerate by parsing `CLASSES:` in `apg_spells.lst`/`acg_spells.lst`
//! — split the body on `|`, `rsplit` each group on `=`, strip a trailing
//! `[...]` gate from the level, membership-test the comma-separated name
//! list against `Inquisitor`, and strip a trailing `.MOD` from the
//! record's own name column. Never substring-match `Inquisitor=`.

/// Every `(spell name, Inquisitor spell level)` pair on the real APG/ACG
/// Inquisitor spell list, sorted by name.
pub const INQUISITOR_SPELL_LIST: &[(&str, u8)] = &[
    ("Acid Splash", 0),
    ("Adjustable Disguise", 3),
    ("Aid", 2),
    ("Alarm", 1),
    ("Align Weapon", 2),
    ("Align Weapon (Communal)", 3),
    ("Animal Purpose Training", 1),
    ("Arcane Sight", 3),
    ("Atonement", 5),
    ("Aura Sight", 4),
    ("Bane", 1),
    ("Banish Seeming", 3),
    ("Banishment", 5),
    ("Blade Barrier", 6),
    ("Blasphemy", 6),
    ("Bleed", 0),
    ("Bless", 1),
    ("Bless Water", 1),
    ("Blood Biography", 3),
    ("Bloodhound", 2),
    ("Brand", 0),
    ("Brand (Greater)", 4),
    ("Break Enchantment", 5),
    ("Bullet Ward", 2),
    ("Burst Bonds", 1),
    ("Calm Emotions", 2),
    ("Cast Out", 3),
    ("Castigate", 2),
    ("Castigate (Mass)", 5),
    ("Cause Fear", 1),
    ("Chaos Hammer", 4),
    ("Circle of Death", 6),
    ("Cleanse", 6),
    ("Command", 1),
    ("Command (Greater)", 5),
    ("Comprehend Languages", 1),
    ("Confess", 2),
    ("Consecrate", 2),
    ("Continual Flame", 3),
    ("Coordinated Effort", 3),
    ("Coward's Lament", 4),
    ("Create Water", 0),
    ("Cure Critical Wounds", 4),
    ("Cure Light Wounds", 1),
    ("Cure Light Wounds (Mass)", 5),
    ("Cure Moderate Wounds", 2),
    ("Cure Moderate Wounds (Mass)", 6),
    ("Cure Serious Wounds", 3),
    ("Curse Water", 1),
    ("Darkness", 2),
    ("Daylight", 3),
    ("Daze", 0),
    ("Death Knell", 2),
    ("Death Ward", 4),
    ("Deeper Darkness", 3),
    ("Delay Poison", 2),
    ("Denounce", 4),
    ("Desecrate", 2),
    ("Detect Chaos", 1),
    ("Detect Evil", 1),
    ("Detect Good", 1),
    ("Detect Law", 1),
    ("Detect Magic", 0),
    ("Detect Poison", 0),
    ("Detect Scrying", 4),
    ("Detect Thoughts", 2),
    ("Detect Undead", 1),
    ("Dictum", 6),
    ("Dimensional Anchor", 3),
    ("Discern Lies", 4),
    ("Disguise Self", 1),
    ("Dismissal", 4),
    ("Dispel Chaos", 5),
    ("Dispel Evil", 5),
    ("Dispel Good", 5),
    ("Dispel Law", 5),
    ("Dispel Magic", 3),
    ("Dispel Magic (Greater)", 6),
    ("Disrupt Undead", 0),
    ("Disrupting Weapon", 5),
    ("Divination", 4),
    ("Divine Favor", 1),
    ("Divine Power", 4),
    ("Doom", 1),
    ("Enchantment Foil", 4),
    ("Enthrall", 2),
    ("Expeditious Retreat", 1),
    ("Fear", 4),
    ("Fester", 3),
    ("Fester (Mass)", 6),
    ("Find Traps", 2),
    ("Find the Path", 6),
    ("Flame Strike", 5),
    ("Flames of the Faithful", 2),
    ("Focused Scrutiny", 2),
    ("Follow Aura", 2),
    ("Forbiddance", 6),
    ("Forced Repentance", 4),
    ("Freedom of Movement", 4),
    ("Geas (Lesser)", 4),
    ("Geas/Quest", 5),
    ("Ghostbane Dirge", 2),
    ("Ghostbane Dirge (Mass)", 5),
    ("Glyph of Warding", 3),
    ("Glyph of Warding (Greater)", 6),
    ("Guidance", 0),
    ("Hallow", 5),
    ("Halt Undead", 3),
    ("Harm", 6),
    ("Heal", 6),
    ("Heightened Awareness", 1),
    ("Heroes' Feast", 6),
    ("Heroism", 3),
    ("Hidden Speech", 3),
    ("Hide from Undead", 1),
    ("Hold Monster", 4),
    ("Hold Person", 2),
    ("Holy Ice Weapon", 2),
    ("Holy Smite", 4),
    ("Holy Word", 6),
    ("Honeyed Tongue", 2),
    ("Hunter's Eye", 3),
    ("Inflict Critical Wounds", 4),
    ("Inflict Light Wounds", 1),
    ("Inflict Light Wounds (Mass)", 5),
    ("Inflict Moderate Wounds", 2),
    ("Inflict Moderate Wounds (Mass)", 6),
    ("Inflict Serious Wounds", 3),
    ("Invisibility", 2),
    ("Invisibility (Greater)", 4),
    ("Invisibility Alarm", 1),
    ("Invisibility Purge", 3),
    ("Keen Edge", 3),
    ("Knock", 2),
    ("Legend Lore", 6),
    ("Light", 0),
    ("Locate Object", 3),
    ("Magic Circle against Chaos", 3),
    ("Magic Circle against Evil", 3),
    ("Magic Circle against Good", 3),
    ("Magic Circle against Law", 3),
    ("Magic Vestment", 3),
    ("Magic Weapon", 1),
    ("Magic Weapon (Greater)", 3),
    ("Mantle of Calm", 3),
    ("Mark of Justice", 5),
    ("Muffle Sound", 2),
    ("Neutralize Poison", 4),
    ("Nondetection", 3),
    ("Obscure Object", 3),
    ("Order's Wrath", 4),
    ("Perceive Cues", 2),
    ("Persistent Vigor", 4),
    ("Planeslayer's Call", 4),
    ("Prayer", 3),
    ("Protection from Chaos", 1),
    ("Protection from Energy", 3),
    ("Protection from Evil", 1),
    ("Protection from Good", 1),
    ("Protection from Law", 1),
    ("Read Magic", 0),
    ("Rebuke", 4),
    ("Refine Improvised Weapon", 1),
    ("Remove Curse", 3),
    ("Remove Disease", 3),
    ("Remove Fear", 1),
    ("Remove Paralysis", 2),
    ("Repulsion", 6),
    ("Resist Energy", 2),
    ("Resistance", 0),
    ("Resounding Blow", 5),
    ("Restoration", 4),
    ("Restoration (Lesser)", 2),
    ("Retribution", 3),
    ("Righteous Might", 5),
    ("Righteous Vigor", 3),
    ("Sacred Bond", 2),
    ("Sanctify Armor", 4),
    ("Sanctuary", 1),
    ("Searing Light", 3),
    ("See Invisibility", 2),
    ("Seek Thoughts", 3),
    ("Sending", 4),
    ("Shared Wrath", 4),
    ("Shield Other", 2),
    ("Shield of Faith", 1),
    ("Shield of Fortification", 1),
    ("Shield of Fortification (Greater)", 3),
    ("Sift", 0),
    ("Silence", 2),
    ("Sleepwalk", 4),
    ("Speak with Dead", 3),
    ("Spell Immunity", 4),
    ("Spell Resistance", 5),
    ("Spiritual Weapon", 2),
    ("Stabilize", 0),
    ("Stoneskin", 4),
    ("Stricken Heart", 2),
    ("Stunning Barrier", 1),
    ("Stunning Barrier (Greater)", 3),
    ("Telepathic Bond", 5),
    ("Tireless Pursuers", 4),
    ("Tireless Pursuit", 1),
    ("Tongues", 2),
    ("True Seeing", 5),
    ("True Strike", 1),
    ("Undeath to Death", 6),
    ("Undetectable Alignment", 2),
    ("Unhallow", 5),
    ("Unholy Blight", 4),
    ("Unholy Ice Weapon", 2),
    ("Unwilling Shield", 5),
    ("Virtue", 0),
    ("Ward the Faithful", 3),
    ("Weapon of Awe", 2),
    ("Whispering Wind", 2),
    ("Word of Chaos", 6),
    ("Wrath", 1),
    ("Zone of Truth", 2),
];

/// The Inquisitor spell level for `spell_key`, or `None` when the spell
/// is not on the Inquisitor list at all.
pub fn inquisitor_spell_level(spell_key: &str) -> Option<u8> {
    INQUISITOR_SPELL_LIST
        .iter()
        .find(|(key, _)| *key == spell_key)
        .map(|(_, level)| *level)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_list_matches_the_verified_corpus_extraction() {
        assert_eq!(INQUISITOR_SPELL_LIST.len(), 219, "219 real Inquisitor spell records");
        let expected = [15, 38, 43, 44, 35, 24, 20];
        for (level, want) in expected.iter().enumerate() {
            let count = INQUISITOR_SPELL_LIST
                .iter()
                .filter(|(_, l)| usize::from(*l) == level)
                .count();
            assert_eq!(count, *want, "spell level {level} count");
        }
    }

    /// Regression guard for the mid-group `CLASSES:` parsing bug: each of
    /// these tags Inquisitor mid-group (not last), so a
    /// `CLASSES:.*Inquisitor=` grep would find neither.
    #[test]
    fn spells_tagged_mid_list_in_their_classes_group_are_present() {
        for (name, level) in [("Blood Biography", 3), ("Perceive Cues", 2), ("Bloodhound", 2)] {
            assert_eq!(
                inquisitor_spell_level(name),
                Some(level),
                "{name} is tagged Inquisitor mid-group and must not be dropped"
            );
        }
    }

    /// Regression guard for the `.MOD` bug: the vast majority of this
    /// list is `.MOD` records grafting Inquisitor onto an existing CRB
    /// spell. The stripped base name must resolve; the raw `.MOD`-suffixed
    /// key must NOT (nothing else in this codebase ever looks up a spell
    /// by a `.MOD`-suffixed name).
    #[test]
    fn mod_records_resolve_under_their_stripped_base_name_only() {
        assert_eq!(inquisitor_spell_level("Bless"), Some(1));
        assert_eq!(inquisitor_spell_level("Cure Light Wounds"), Some(1));
        assert_eq!(inquisitor_spell_level("Bless.MOD"), None);
        assert_eq!(inquisitor_spell_level("Cure Light Wounds.MOD"), None);
    }

    /// Inquisitor is a 0-6 caster (verified against `apg_classes.lst`'s
    /// own `CAST:0,5,5,5,5,5,5` level-20 row, seven columns).
    #[test]
    fn inquisitor_tops_out_at_sixth_level_spells() {
        for (name, level) in INQUISITOR_SPELL_LIST {
            assert!(*level <= 6, "{name} at level {level}: 6 is the ceiling");
        }
        assert!(INQUISITOR_SPELL_LIST.iter().any(|(_, l)| *l == 0));
        assert!(INQUISITOR_SPELL_LIST.iter().any(|(_, l)| *l == 6));
    }

    #[test]
    fn unknown_spell_resolves_to_none() {
        assert_eq!(inquisitor_spell_level("Definitely Not A Real Spell"), None);
    }
}
