//! The `PRE`-family prerequisite tokens the feat catalog records carry,
//! relocated off the live side — SD-35 `AT-35-E6-003-SWEEP` cycle 3,
//! enforcing `decisions.md` §11 (nothing on the live side reads a PCGen
//! token).
//!
//! # Why these moved, and why they were not deleted
//!
//! `rules_tables::feats_all::FeatCatalogRecord` and the five book-local feat
//! entry types (`ultimate_combat`, `ultimate_magic`, `ultimate_psionics`,
//! `ultimate_wilderness`, `ultimate_intrigue`) each carried a
//! `prerequisites: Option<&'static [&'static str]>` field holding every
//! top-level `PRE`-family token of the corpus row, verbatim, plus the three
//! `ARG_`/`PU_`/`UCA_FEAT_PREREQUISITES` backfill tables that supplied the
//! same field for the books whose own ingest never gathered it.
//!
//! **The live prerequisite evaluator never read any of it.**
//! `rules_core::feat_prereqs::evaluate_catalog_feat_prerequisites` reads the
//! CONVERTED [`crate::rules_core::sheet_rule::Applies`] gate out of
//! `data/sheet_rules/` and never a token string — it takes the record only
//! for its `key`. The two readers of the field are both converter modules,
//! `cache_gen::feat_gap` and `cache_gen::hand_authored_feat_dump`, which
//! write the tokens into the ingest cache. Re-derive the absence of any
//! other reader:
//!
//! ```text
//! grep -rn "\.prerequisites" src apps/desktop/src-tauri/src --include=*.rs \
//!   | grep -v prerequisites_added | grep -v failing_prerequisites
//! ```
//!
//! So this is a **move, not a removal** (`decisions.md` §11: the converter
//! side is KEPT and reused for Starfinder). Both consumers read the same
//! tokens through [`hand_authored_feat_prereq_tokens`] and
//! [`feat_gap_prereq_tokens`], and three independent checks say the values
//! did not change in transit:
//!
//! * the gap half **regenerates byte-identically off the pinned corpus** —
//!   `cargo run --bin gen_feat_gap_tables` writes
//!   `feat_gap_prereq_tokens.rs` and `rules_tables::feat_gap_tables.rs` in
//!   one pass, and all 601 relocated rows came back unchanged;
//! * `tests/sd27_feat_prerequisite_enforcement.rs::
//!   the_gathered_arg_and_pu_prerequisites_match_the_live_corpus` re-derives
//!   ARG's 187 and PU's 17 records straight from `arg_feats.lst` /
//!   `pu_feats.lst` and compares them to [`joined_catalog_tokens`];
//! * `tests::the_per_book_coverage_survived_the_move_unchanged` re-asserts
//!   every per-book count the removed field's own test asserted.
//!
//! What is **not** claimed: the ingest cache under `data/corpus/` was not
//! regenerated here and so is not byte-proven — regenerating it rewrites
//! license and PI fields, which is its own cycle's work.
//!
//! # How a row is addressed
//!
//! By `(rule_set, index)` — the record's own position in the live table it
//! was relocated from — and **not** by `(rule_set, key)`, which would
//! collide: CRB carries two distinct `"Combat Expertise"` records with
//! different token sets (`crb/feat_data/combat.rs`, the plain feat and the
//! Dirty Trickster variant). Each row also carries the key it was taken
//! from, and both lookups assert it against the caller's record, so a table
//! edit that shifts an index fails loudly instead of silently pairing a
//! record with another record's prerequisites.
//! `tests::every_row_still_names_the_record_it_was_taken_from` walks both
//! tables end to end and re-checks every pairing.
//!
//! A record is absent from these tables when its corpus row carried no
//! `PRE`-family token at all — the same `None`-when-absent rule the removed
//! field documented. `Some(&[])` never occurred and is not representable
//! here.

use std::collections::HashMap;
use std::sync::OnceLock;

use crate::rules_core::rules_tables::RuleSetId;

/// One relocated record's tokens: `(rule_set, index in that book's table,
/// the record's `key`, the tokens in corpus source order)`.
pub type FeatPrereqRow = (RuleSetId, usize, &'static str, &'static [&'static str]);

/// The tokens the hand-authored feat tables carried, addressed by each
/// record's index in `feats_all::hand_authored_feat_tables()`'s book slice.
///
/// 1429 row(s) across 11 book(s).
pub static HAND_AUTHORED_FEAT_PREREQ_TOKENS: &[FeatPrereqRow] = &[
    // Core Rulebook — 130 of 185 record(s) in the hand-authored table carry at least one `PRE`-family token.
    (RuleSetId::Crb, 1, "Acrobatic Steps", &["PREABILITY:1,CATEGORY=FEAT,Nimble Moves", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,15],[PREVARGTEQ:FeatDexRequirement,15]"]),
    (RuleSetId::Crb, 3, "Alignment Channel", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Energy"]),
    (RuleSetId::Crb, 6, "Augment Summoning", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Conjuration)"]),
    (RuleSetId::Crb, 8, "Command Undead", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Negative Energy"]),
    (RuleSetId::Crb, 11, "Diehard", &["PREABILITY:1,CATEGORY=FEAT,Endurance"]),
    (RuleSetId::Crb, 12, "Elemental Channel", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Energy"]),
    (RuleSetId::Crb, 15, "Extra Channel", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Positive Energy,TYPE.Channel Negative Energy"]),
    (RuleSetId::Crb, 16, "Extra Ki", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Ki Pool"]),
    (RuleSetId::Crb, 17, "Extra Lay On Hands", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Lay on Hands"]),
    (RuleSetId::Crb, 18, "Extra Mercy", &["PREABILITY:2,CATEGORY=Special Ability,TYPE.Lay on Hands,TYPE.Mercy"]),
    (RuleSetId::Crb, 19, "Extra Performance", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Bardic Performance"]),
    (RuleSetId::Crb, 20, "Extra Rage", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Rage"]),
    (RuleSetId::Crb, 23, "Greater Spell Focus", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus"]),
    (RuleSetId::Crb, 24, "Greater Spell Penetration", &["PREABILITY:1,CATEGORY=FEAT,Spell Penetration"]),
    (RuleSetId::Crb, 25, "Improved Channel", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Positive Energy,TYPE.Channel Negative Energy"]),
    (RuleSetId::Crb, 28, "Improved Great Fortitude", &["PREABILITY:1,CATEGORY=FEAT,Great Fortitude"]),
    (RuleSetId::Crb, 29, "Improved Iron Will", &["PREABILITY:1,CATEGORY=FEAT,Iron Will"]),
    (RuleSetId::Crb, 30, "Improved Lightning Reflexes", &["PREABILITY:1,CATEGORY=FEAT,Lightning Reflexes"]),
    (RuleSetId::Crb, 32, "Leadership", &["PRELEVEL:MIN=7"]),
    (RuleSetId::Crb, 35, "Master Craftsman", &["PRESKILL:1,TYPE.Craft=5,TYPE.Profession=5"]),
    (RuleSetId::Crb, 36, "Natural Spell", &["PREMULT:1,[PREVARGTEQ:WildShapeProgression,1],[PREVARGTEQ:ShamanWildShapeProgression,1]", "PREVARGTEQ:PreStatScore_WIS,13"]),
    (RuleSetId::Crb, 37, "Nimble Moves", &["PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]"]),
    (RuleSetId::Crb, 40, "Selective Channeling", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Positive Energy,TYPE.Channel Negative Energy", "PREVARGTEQ:PreStatScore_CHA,13"]),
    (RuleSetId::Crb, 45, "Spell Mastery", &["PREVARGTEQ:SpellMasteryQualify,1"]),
    (RuleSetId::Crb, 49, "Turn Undead", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Positive Energy"]),
    (RuleSetId::Crb, 51, "Arcane Armor Mastery", &["PREABILITY:1,CATEGORY=FEAT,Arcane Armor Training", "PREMULT:1,[PRECLASS:1,SPELLCASTER=7],[PREVARGTEQ:CasterLevel_Highest,7]", "PREMULT:1,[PREPROFWITHARMOR:1,TYPE.Medium],[PREABILITY:1,CATEGORY=FEAT,Armor Proficiency (Medium)]"]),
    (RuleSetId::Crb, 52, "Arcane Armor Training", &["PREMULT:1,[PRECLASS:1,SPELLCASTER=3],[PREVARGTEQ:CasterLevel_Highest,3]", "PREMULT:1,[PREPROFWITHARMOR:1,TYPE.Light],[PREABILITY:1,CATEGORY=FEAT,Armor Proficiency (Light)]"]),
    (RuleSetId::Crb, 53, "Arcane Strike", &["PREMULT:1,[PREMULT:1,[PRECLASS:1,SPELLCASTER.Arcane=1],[PREVARGTEQ:Caster_Level_Highest__Arcane,1]],[PREABILITY:1,CATEGORY=Special Ability,TYPE.SpellLike]"]),
    (RuleSetId::Crb, 54, "Armor Proficiency (Heavy)", &["PREMULT:1,[PREPROFWITHARMOR:1,TYPE.Medium],[PREABILITY:1,CATEGORY=FEAT,Armor Proficiency (Medium)]"]),
    (RuleSetId::Crb, 56, "Armor Proficiency (Medium)", &["PREMULT:1,[PREPROFWITHARMOR:1,TYPE.Light],[PREABILITY:1,CATEGORY=FEAT,Armor Proficiency (Light)]"]),
    (RuleSetId::Crb, 57, "Bleeding Critical", &["PREABILITY:1,CATEGORY=FEAT,Critical Focus", "PREMULT:1,[PRETOTALAB:11],[PREVARGTEQ:CriticalFocusQualify,11]"]),
    (RuleSetId::Crb, 59, "Blinding Critical", &["PREABILITY:1,CATEGORY=FEAT,Critical Focus", "PREMULT:1,[PRETOTALAB:15],[PREVARGTEQ:CriticalFocusQualify,15]"]),
    (RuleSetId::Crb, 61, "Channel Smite", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Energy"]),
    (RuleSetId::Crb, 62, "Cleave", &["PREABILITY:1,CATEGORY=FEAT,Power Attack", "PRETOTALAB:1", "PREVARGTEQ:PreStatScore_STR,13"]),
    (RuleSetId::Crb, 63, "Combat Expertise", &["PREMULT:1,[PREVARGTEQ:PreStatScore_INT,13],[PREVARGTEQ:CombatFeatIntRequirement,13]"]),
    (RuleSetId::Crb, 64, "Combat Expertise", &["PREMULT:1,[PREVARGTEQ:PreStatScore_INT,13],[PREVARGTEQ:CombatFeatIntRequirement,13],[PREVARGTEQ:DirtyTricksterIntQualify,1]"]),
    (RuleSetId::Crb, 66, "Critical Focus", &["PREMULT:1,[PRETOTALAB:9],[PREVARGTEQ:CriticalFocusQualify,9]"]),
    (RuleSetId::Crb, 67, "Critical Mastery", &["PREABILITY:1,CATEGORY=FEAT,Critical Focus", "PREABILITY:2,CATEGORY=FEAT,TYPE.Critical", "PREVARGTEQ:CriticalMasteryQualify,1"]),
    (RuleSetId::Crb, 68, "Dazzling Display", &["PREABILITY:1,CATEGORY=FEAT,Weapon Focus"]),
    (RuleSetId::Crb, 69, "Deadly Aim", &["PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRETOTALAB:1"]),
    (RuleSetId::Crb, 70, "Deadly Stroke", &["PREABILITY:4,CATEGORY=FEAT,Dazzling Display,Greater Weapon Focus,Shatter Defenses,Weapon Focus", "PRETOTALAB:11"]),
    (RuleSetId::Crb, 71, "Deafening Critical", &["PREABILITY:1,CATEGORY=FEAT,Critical Focus", "PREMULT:1,[PRETOTALAB:13],[PREVARGTEQ:CriticalFocusQualify,13]"]),
    (RuleSetId::Crb, 73, "Deflect Arrows", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]"]),
    (RuleSetId::Crb, 74, "Disruptive", &["PREVARGTEQ:DisruptiveQualify,1"]),
    (RuleSetId::Crb, 75, "Dodge", &["PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]"]),
    (RuleSetId::Crb, 76, "Double Slice", &["PREABILITY:1,CATEGORY=FEAT,Two-Weapon Fighting", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,15],[PREVARGTEQ:FeatDexRequirement,15]"]),
    (RuleSetId::Crb, 77, "Exhausting Critical", &["PREABILITY:2,CATEGORY=FEAT,Critical Focus,Tiring Critical", "PREMULT:1,[PRETOTALAB:15],[PREVARGTEQ:CriticalFocusQualify,15]"]),
    (RuleSetId::Crb, 78, "Exotic Weapon Proficiency", &["PRETOTALAB:1"]),
    (RuleSetId::Crb, 79, "Far Shot", &["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot"]),
    (RuleSetId::Crb, 80, "Gorgon's Fist", &["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Scorpion Style", "PRETOTALAB:6"]),
    (RuleSetId::Crb, 81, "Great Cleave", &["PREABILITY:2,CATEGORY=FEAT,Cleave,Power Attack", "PRETOTALAB:4", "PREVARGTEQ:PreStatScore_STR,13"]),
    (RuleSetId::Crb, 82, "Greater Bull Rush", &["PREABILITY:2,CATEGORY=FEAT,Improved Bull Rush,Power Attack", "PRETOTALAB:6", "PREVARGTEQ:PreStatScore_STR,13"]),
    (RuleSetId::Crb, 83, "Greater Disarm", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Disarm", "PREMULT:1,[PREVARGTEQ:PreStatScore_INT,13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:6"]),
    (RuleSetId::Crb, 84, "Greater Feint", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Feint", "PREMULT:1,[PREVARGTEQ:PreStatScore_INT,13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:6"]),
    (RuleSetId::Crb, 85, "Greater Grapple", &["PREABILITY:2,CATEGORY=FEAT,Improved Grapple,Improved Unarmed Strike", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRETOTALAB:6"]),
    (RuleSetId::Crb, 86, "Greater Overrun", &["PREABILITY:2,CATEGORY=FEAT,Improved Overrun,Power Attack", "PRETOTALAB:6", "PREVARGTEQ:PreStatScore_STR,13"]),
    (RuleSetId::Crb, 87, "Greater Penetrating Strike", &["PREABILITY:2,CATEGORY=FEAT,Penetrating Strike,Weapon Focus", "PREVARGTEQ:GreatPenetratingStrikeQualify,1"]),
    (RuleSetId::Crb, 88, "Greater Shield Focus", &["PREABILITY:1,CATEGORY=FEAT,Shield Focus", "PREMULT:1,[PREPROFWITHSHIELD:3,TYPE.Buckler,TYPE.Light,TYPE.Heavy],[PREABILITY:1,CATEGORY=FEAT,Shield Proficiency]", "PREVARGTEQ:GreatShieldFocusQualify,1"]),
    (RuleSetId::Crb, 89, "Greater Sunder", &["PREABILITY:2,CATEGORY=FEAT,Improved Sunder,Power Attack", "PRETOTALAB:6", "PREVARGTEQ:PreStatScore_STR,13"]),
    (RuleSetId::Crb, 90, "Greater Trip", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Trip", "PREMULT:1,[PREVARGTEQ:PreStatScore_INT,13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:6"]),
    (RuleSetId::Crb, 91, "Greater Two-Weapon Fighting", &["PREABILITY:2,CATEGORY=FEAT,Improved Two-Weapon Fighting,Two-Weapon Fighting", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,19],[PREVARGTEQ:FeatDexRequirement,19]", "PRETOTALAB:11"]),
    (RuleSetId::Crb, 92, "Greater Vital Strike", &["PREABILITY:2,CATEGORY=FEAT,Improved Vital Strike,Vital Strike", "PRETOTALAB:16"]),
    (RuleSetId::Crb, 93, "Greater Weapon Focus", &["PREABILITY:1,CATEGORY=FEAT,Weapon Focus", "PRETOTALAB:1", "PREVARGTEQ:GreatWeapFocusQualify,1"]),
    (RuleSetId::Crb, 94, "Greater Weapon Specialization", &["PREABILITY:3,CATEGORY=FEAT,Greater Weapon Focus,Weapon Focus,Weapon Specialization", "PREVARGTEQ:GreatWeapSpecQualify,1"]),
    (RuleSetId::Crb, 95, "Improved Bull Rush", &["PREABILITY:1,CATEGORY=FEAT,Power Attack", "PRETOTALAB:1", "PREVARGTEQ:PreStatScore_STR,13"]),
    (RuleSetId::Crb, 96, "Improved Critical", &["PRETOTALAB:8"]),
    (RuleSetId::Crb, 97, "Improved Disarm", &["PREABILITY:1,CATEGORY=FEAT,Combat Expertise", "PREMULT:1,[PREVARGTEQ:PreStatScore_INT,13],[PREVARGTEQ:CombatFeatIntRequirement,13]"]),
    (RuleSetId::Crb, 98, "Improved Feint", &["PREABILITY:1,CATEGORY=FEAT,Combat Expertise", "PREMULT:1,[PREVARGTEQ:PreStatScore_INT,13],[PREVARGTEQ:CombatFeatIntRequirement,13]"]),
    (RuleSetId::Crb, 99, "Improved Grapple", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]"]),
    (RuleSetId::Crb, 101, "Improved Overrun", &["PREABILITY:1,CATEGORY=FEAT,Power Attack", "PRETOTALAB:1", "PREVARGTEQ:PreStatScore_STR,13"]),
    (RuleSetId::Crb, 102, "Improved Precise Shot", &["PREABILITY:2,CATEGORY=FEAT,Point-Blank Shot,Precise Shot", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,19],[PREVARGTEQ:FeatDexRequirement,19]", "PRETOTALAB:11"]),
    (RuleSetId::Crb, 103, "Improved Shield Bash", &["PREMULT:1,[PREPROFWITHSHIELD:3,TYPE.Buckler,TYPE.Light,TYPE.Heavy],[PREABILITY:1,CATEGORY=FEAT,Shield Proficiency]"]),
    (RuleSetId::Crb, 104, "Improved Sunder", &["PREABILITY:1,CATEGORY=FEAT,Power Attack", "PRETOTALAB:1", "PREVARGTEQ:PreStatScore_STR,13"]),
    (RuleSetId::Crb, 105, "Improved Trip", &["PREABILITY:1,CATEGORY=FEAT,Combat Expertise", "PREMULT:1,[PREVARGTEQ:PreStatScore_INT,13],[PREVARGTEQ:CombatFeatIntRequirement,13]"]),
    (RuleSetId::Crb, 106, "Improved Two-Weapon Fighting", &["PREABILITY:1,CATEGORY=FEAT,Two-Weapon Fighting", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,17],[PREVARGTEQ:FeatDexRequirement,17]", "PRETOTALAB:6"]),
    (RuleSetId::Crb, 108, "Improved Vital Strike", &["PREABILITY:1,CATEGORY=FEAT,Vital Strike", "PRETOTALAB:11"]),
    (RuleSetId::Crb, 109, "Improvised Weapon Mastery", &["PREABILITY:1,CATEGORY=FEAT,Catch Off-Guard,Throw Anything", "PRETOTALAB:8"]),
    (RuleSetId::Crb, 111, "Lightning Stance", &["PREABILITY:2,CATEGORY=FEAT,Dodge,Wind Stance", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,17],[PREVARGTEQ:FeatDexRequirement,17]", "PRETOTALAB:11"]),
    (RuleSetId::Crb, 112, "Lunge", &["PRETOTALAB:6"]),
    (RuleSetId::Crb, 113, "Manyshot", &["PREABILITY:2,CATEGORY=FEAT,Point-Blank Shot,Rapid Shot", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,17],[PREVARGTEQ:FeatDexRequirement,17]", "PRETOTALAB:6"]),
    (RuleSetId::Crb, 114, "Martial Weapon Proficiency", &["!PREABILITY:1,CATEGORY=FEAT,Martial Weapon Proficiency Output"]),
    (RuleSetId::Crb, 115, "Medusa's Wrath", &["PREABILITY:3,CATEGORY=FEAT,Improved Unarmed Strike,Gorgon's Fist,Scorpion Style", "PRETOTALAB:11"]),
    (RuleSetId::Crb, 116, "Mobility", &["PREABILITY:1,CATEGORY=FEAT,Dodge", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]"]),
    (RuleSetId::Crb, 117, "Mounted Archery", &["PREABILITY:1,CATEGORY=FEAT,Mounted Combat", "PRESKILL:1,Ride=1"]),
    (RuleSetId::Crb, 118, "Mounted Combat", &["PRESKILL:1,Ride=1"]),
    (RuleSetId::Crb, 119, "Penetrating Strike", &["PREABILITY:1,CATEGORY=FEAT,Weapon Focus", "PRETOTALAB:1", "PREVARGTEQ:PenetratingStrikeQualify,1"]),
    (RuleSetId::Crb, 120, "Pinpoint Targeting", &["PREABILITY:3,CATEGORY=FEAT,Improved Precise Shot,Point-Blank Shot,Precise Shot", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,19],[PREVARGTEQ:FeatDexRequirement,19]", "PRETOTALAB:16"]),
    (RuleSetId::Crb, 122, "Power Attack", &["PRETOTALAB:1", "PREVARGTEQ:PreStatScore_STR,13"]),
    (RuleSetId::Crb, 123, "Precise Shot", &["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot"]),
    (RuleSetId::Crb, 124, "Quick Draw", &["PRETOTALAB:1"]),
    (RuleSetId::Crb, 125, "Rapid Reload", &["PRETEXT:Weapon Proficiency (crossbow type chosen).", "PREWEAPONPROF:1,TYPE.Crossbow"]),
    (RuleSetId::Crb, 126, "Rapid Shot", &["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]"]),
    (RuleSetId::Crb, 127, "Ride-By Attack", &["PREABILITY:1,CATEGORY=FEAT,Mounted Combat", "PRESKILL:1,Ride=1"]),
    (RuleSetId::Crb, 128, "Scorpion Style", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike"]),
    (RuleSetId::Crb, 129, "Shatter Defenses", &["PREABILITY:2,CATEGORY=FEAT,Weapon Focus,Dazzling Display", "PRETOTALAB:6"]),
    (RuleSetId::Crb, 130, "Shield Focus", &["PREMULT:1,[PREPROFWITHSHIELD:3,TYPE.Buckler,TYPE.Light,TYPE.Heavy],[PREABILITY:1,CATEGORY=FEAT,Shield Proficiency]", "PRETOTALAB:1"]),
    (RuleSetId::Crb, 131, "Shield Master", &["PREABILITY:3,CATEGORY=FEAT,Improved Shield Bash,Shield Slam,Two-Weapon Fighting", "PREMULT:1,[PREPROFWITHSHIELD:3,TYPE.Buckler,TYPE.Light,TYPE.Heavy],[PREABILITY:1,CATEGORY=FEAT,Shield Proficiency]", "PRETOTALAB:11"]),
    (RuleSetId::Crb, 133, "Shield Slam", &["PREABILITY:2,CATEGORY=FEAT,Improved Shield Bash,Two-Weapon Fighting", "PREMULT:1,[PREPROFWITHSHIELD:3,TYPE.Buckler,TYPE.Light,TYPE.Heavy],[PREABILITY:1,CATEGORY=FEAT,Shield Proficiency]", "PRETOTALAB:6"]),
    (RuleSetId::Crb, 134, "Shot on the Run", &["PREABILITY:3,CATEGORY=FEAT,Dodge,Mobility,Point-Blank Shot", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRETOTALAB:4"]),
    (RuleSetId::Crb, 135, "Sickening Critical", &["PREABILITY:1,CATEGORY=FEAT,Critical Focus", "PREMULT:1,[PRETOTALAB:11],[PREVARGTEQ:CriticalFocusQualify,11]"]),
    (RuleSetId::Crb, 136, "Snatch Arrows", &["PREABILITY:2,CATEGORY=FEAT,Deflect Arrows,Improved Unarmed Strike", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,15],[PREVARGTEQ:FeatDexRequirement,15]"]),
    (RuleSetId::Crb, 137, "Spellbreaker", &["PREABILITY:1,CATEGORY=FEAT,Disruptive", "PREVARGTEQ:SpellBreakerQualify,1"]),
    (RuleSetId::Crb, 138, "Spirited Charge", &["PREABILITY:2,CATEGORY=FEAT,Mounted Combat,Ride-By Attack", "PRESKILL:1,Ride=1"]),
    (RuleSetId::Crb, 139, "Spring Attack", &["PREABILITY:2,CATEGORY=FEAT,Dodge,Mobility", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRETOTALAB:4"]),
    (RuleSetId::Crb, 140, "Staggering Critical", &["PREABILITY:1,CATEGORY=FEAT,Critical Focus", "PREMULT:1,[PRETOTALAB:13],[PREVARGTEQ:CriticalFocusQualify,13]"]),
    (RuleSetId::Crb, 141, "Stand Still", &["PREABILITY:1,CATEGORY=FEAT,Combat Reflexes"]),
    (RuleSetId::Crb, 142, "Step Up", &["PRETOTALAB:1"]),
    (RuleSetId::Crb, 143, "Strike Back", &["PRETOTALAB:11"]),
    (RuleSetId::Crb, 144, "Stunning Critical", &["PREABILITY:2,CATEGORY=FEAT,Critical Focus,Staggering Critical", "PREMULT:1,[PRETOTALAB:17],[PREVARGTEQ:CriticalFocusQualify,17]"]),
    (RuleSetId::Crb, 145, "Stunning Fist", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PREMULT:2,[PREVARGTEQ:PreStatScore_WIS,13],[PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]]", "PRETOTALAB:8"]),
    (RuleSetId::Crb, 147, "Tiring Critical", &["PREABILITY:1,CATEGORY=FEAT,Critical Focus", "PREMULT:1,[PRETOTALAB:13],[PREVARGTEQ:CriticalFocusQualify,13]"]),
    (RuleSetId::Crb, 148, "Tower Shield Proficiency", &["PREMULT:1,[PREPROFWITHSHIELD:3,TYPE.Buckler,TYPE.Light,TYPE.Heavy],[PREABILITY:1,CATEGORY=FEAT,Shield Proficiency]"]),
    (RuleSetId::Crb, 149, "Trample", &["PREABILITY:1,CATEGORY=FEAT,Mounted Combat", "PRESKILL:1,Ride=1"]),
    (RuleSetId::Crb, 150, "Two-Weapon Defense", &["PREABILITY:1,CATEGORY=FEAT,Two-Weapon Fighting", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,15],[PREVARGTEQ:FeatDexRequirement,15]"]),
    (RuleSetId::Crb, 151, "Two-Weapon Fighting", &["PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,15],[PREVARGTEQ:FeatDexRequirement,15]"]),
    (RuleSetId::Crb, 152, "Two-Weapon Rend", &["PREABILITY:3,CATEGORY=FEAT,Double Slice,Improved Two-Weapon Fighting,Two-Weapon Fighting", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,17],[PREVARGTEQ:FeatDexRequirement,17]", "PRETOTALAB:11"]),
    (RuleSetId::Crb, 153, "Unseat", &["PREABILITY:3,CATEGORY=FEAT,Mounted Combat,Power Attack,Improved Bull Rush", "PRESKILL:1,Ride=1", "PRETOTALAB:1", "PREVARGTEQ:PreStatScore_STR,13"]),
    (RuleSetId::Crb, 154, "Vital Strike", &["PRETOTALAB:6"]),
    (RuleSetId::Crb, 156, "Weapon Focus", &["PRETOTALAB:1"]),
    (RuleSetId::Crb, 157, "Weapon Specialization", &["PREABILITY:1,CATEGORY=FEAT,Weapon Focus", "PREVARGTEQ:WeapSpecQualify,1"]),
    (RuleSetId::Crb, 158, "Whirlwind Attack", &["PREABILITY:4,CATEGORY=FEAT,Combat Expertise,Dodge,Mobility,Spring Attack", "PREMULT:1,[PREVARGTEQ:PreStatScore_INT,13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRETOTALAB:4"]),
    (RuleSetId::Crb, 159, "Wind Stance", &["PREABILITY:1,CATEGORY=FEAT,Dodge", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,15],[PREVARGTEQ:FeatDexRequirement,15]", "PRETOTALAB:6"]),
    (RuleSetId::Crb, 160, "Brew Potion", &["PREMULT:1,[PRECLASS:1,SPELLCASTER=3],[PREVARGTEQ:CasterLevel_Highest,3]"]),
    (RuleSetId::Crb, 161, "Craft Magic Arms and Armor", &["PREMULT:1,[PREMULT:1,[PRECLASS:1,SPELLCASTER=5],[PREVARGTEQ:CasterLevel_Highest,5]],[PREVARGTEQ:MasterCraftsmanRanks,5]"]),
    (RuleSetId::Crb, 162, "Craft Rod", &["PREMULT:1,[PRECLASS:1,SPELLCASTER=9],[PREVARGTEQ:CasterLevel_Highest,9]"]),
    (RuleSetId::Crb, 163, "Craft Staff", &["PREMULT:1,[PRECLASS:1,SPELLCASTER=11],[PREVARGTEQ:CasterLevel_Highest,11]"]),
    (RuleSetId::Crb, 164, "Craft Wand", &["PREMULT:1,[PRECLASS:1,SPELLCASTER=5],[PREVARGTEQ:CasterLevel_Highest,5]"]),
    (RuleSetId::Crb, 165, "Craft Wondrous Item", &["PREMULT:1,[PREMULT:1,[PRECLASS:1,SPELLCASTER=3],[PREVARGTEQ:CasterLevel_Highest,3]],[PREVARGTEQ:MasterCraftsmanRanks,3]"]),
    (RuleSetId::Crb, 166, "Forge Ring", &["PREMULT:1,[PRECLASS:1,SPELLCASTER=7],[PREVARGTEQ:CasterLevel_Highest,7]"]),
    (RuleSetId::Crb, 167, "Scribe Scroll", &["PREMULT:1,[PRECLASS:1,SPELLCASTER=1],[PREVARGTEQ:CasterLevel_Highest,1]"]),
    // Advanced Player’s Guide — 143 of 172 record(s) in the hand-authored table carry at least one `PRE`-family token.
    (RuleSetId::Apg, 1, "Arcane Blast", &["PREMULT:1,[PRECLASS:1,SPELLCASTER.Arcane=10],[PREVARGTEQ:Caster_Level_Highest__Arcane,10]"]),
    (RuleSetId::Apg, 2, "Arcane Shield", &["PREMULT:1,[PRECLASS:1,SPELLCASTER.Arcane=10],[PREVARGTEQ:Caster_Level_Highest__Arcane,10]"]),
    (RuleSetId::Apg, 3, "Arcane Talent", &["PRERACE:1,RACESUBTYPE=Elf,RACESUBTYPE=Gnome", "PRESTAT:1,CHA=10"]),
    (RuleSetId::Apg, 4, "Aspect of the Beast", &["PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Ranger Combat Style ~ Natural Weapon],[PREABILITY:1,CATEGORY=Special Ability,Wild Shape],[PREVARGT:Lycanthrope,0]", "PRETEXT:wild shape class feature, see Special."]),
    (RuleSetId::Apg, 5, "Breadth of Experience", &["PRERACE:1,RACESUBTYPE=Dwarf,RACESUBTYPE=Elf,RACESUBTYPE=Gnome", "PRETEXT:100+ years old."]),
    (RuleSetId::Apg, 6, "Childlike", &["PRERACE:1,RACESUBTYPE=Halfling", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Apg, 7, "Cloud Step", &["PREABILITY:1,CATEGORY=FEAT,Spider Step", "PRECLASS:1,Monk=12"]),
    (RuleSetId::Apg, 8, "Cooperative Crafting", &["PREABILITY:1,CATEGORY=FEAT,TYPE.ItemCreation", "PRESKILL:1,TYPE.Craft=1"]),
    (RuleSetId::Apg, 10, "Deep Drinker", &["PREABILITY:1,CATEGORY=Special Ability,Drunken Master ~ Drunken Ki", "PRECLASS:1,Monk=11", "PRESTAT:1,CON=13"]),
    (RuleSetId::Apg, 11, "Deepsight", &["PREVISION:1,Darkvision=60"]),
    (RuleSetId::Apg, 12, "Diviner's Delving", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Divination)"]),
    (RuleSetId::Apg, 13, "Eagle Eyes", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.KeenSenses", "PRESTAT:1,WIS=13"]),
    (RuleSetId::Apg, 14, "Eclectic", &["PRERACE:1,RACESUBTYPE=Human"]),
    (RuleSetId::Apg, 16, "Expanded Arcana", &["PREMULT:1,[PRECLASS:1,SPELLCASTER=1],[PREVARGTEQ:CasterLevel_Highest,1]", "PRESPELLCAST:MEMORIZE=N"]),
    (RuleSetId::Apg, 17, "Extra Bombs", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Bomb"]),
    (RuleSetId::Apg, 18, "Extra Discovery", &["PREABILITY:1,CATEGORY=Special Ability,Alchemist ~ Discovery"]),
    (RuleSetId::Apg, 19, "Extra Hex", &["PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,TYPE.WitchHex],[PREVARGTEQ:WitchMinorHexQualify,1],[PREVARGTEQ:WitchHexAbilityLVL,1]"]),
    (RuleSetId::Apg, 20, "Extra Rage Power", &["PREABILITY:1,CATEGORY=Special Ability,Barbarian ~ Rage Powers"]),
    (RuleSetId::Apg, 21, "Extra Revelation", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Oracle's Mystery"]),
    (RuleSetId::Apg, 22, "Extra Rogue Talent", &["PREVARGTEQ:RogueTalentLVL,1"]),
    (RuleSetId::Apg, 23, "Fast Drinker", &["PREABILITY:1,CATEGORY=Special Ability,Drunken Master ~ Drunken Ki", "PRESTAT:1,CON=18"]),
    (RuleSetId::Apg, 24, "Fast Healer", &["PREABILITY:2,CATEGORY=FEAT,Diehard,Endurance", "PRESTAT:1,CON=13"]),
    (RuleSetId::Apg, 25, "Favored Defense", &["PREABILITY:1,CATEGORY=Special Ability,Basic Favored Enemy"]),
    (RuleSetId::Apg, 26, "Fight On", &["PRERACE:1,RACESUBTYPE=Dwarf,RACESUBTYPE=Orc", "PRESTAT:1,CON=13"]),
    (RuleSetId::Apg, 27, "Gnome Trickster", &["PREABILITY:1,CATEGORY=Special Ability,Gnome ~ Gnome Magic", "PRERACE:1,RACESUBTYPE=Gnome", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Apg, 28, "Go Unnoticed", &["PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRESIZELTEQ:S"]),
    (RuleSetId::Apg, 29, "Greater Elemental Focus", &["PREABILITY:1,CATEGORY=FEAT,Elemental Focus"]),
    (RuleSetId::Apg, 30, "Groundling", &["PREABILITY:1,CATEGORY=Special Ability,Gnome ~ Gnome Magic", "PRERACE:1,RACESUBTYPE=Gnome", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Apg, 31, "Heroic Defiance", &["PREABILITY:2,CATEGORY=FEAT,Diehard,Endurance", "PRETOTALAB:8"]),
    (RuleSetId::Apg, 32, "Heroic Recovery", &["PREABILITY:2,CATEGORY=FEAT,Diehard,Endurance", "PRECHECKBASE:1,Fortitude=4"]),
    (RuleSetId::Apg, 33, "Improved Share Spells", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Companion,TYPE.Special Mount,TYPE.Familiar,TYPE.Eidolon", "PRESKILL:1,Spellcraft=10"]),
    (RuleSetId::Apg, 34, "Improved Stonecunning", &["PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Stonecunning", "PRERACE:1,RACESUBTYPE=Dwarf", "PRESTAT:1,WIS=13"]),
    (RuleSetId::Apg, 35, "Ironguts", &["PRERACE:1,RACESUBTYPE=Dwarf,RACESUBTYPE=Orc", "PRESTAT:1,CON=13"]),
    (RuleSetId::Apg, 36, "Ironhide", &["PRERACE:1,RACESUBTYPE=Dwarf,RACESUBTYPE=Orc", "PRESTAT:1,CON=13"]),
    (RuleSetId::Apg, 37, "Keen Scent", &["PRERACE:1,RACESUBTYPE=Orc", "PRESTAT:1,WIS=13"]),
    (RuleSetId::Apg, 38, "Leaf Singer", &["PREABILITY:1,CATEGORY=Special Ability,Bard ~ Bardic Performance", "PRERACE:1,RACESUBTYPE=Elf", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Apg, 39, "Light Step", &["PREABILITY:2,CATEGORY=FEAT,Acrobatic Steps,Nimble Moves", "PRERACE:1,RACESUBTYPE=Elf"]),
    (RuleSetId::Apg, 40, "Lingering Performance", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Bardic Performance"]),
    (RuleSetId::Apg, 41, "Lucky Halfling", &["PRERACE:1,RACESUBTYPE=Halfling"]),
    (RuleSetId::Apg, 42, "Major Spell Expertise", &["PREABILITY:1,CATEGORY=FEAT,Minor Spell Expertise", "PRESPELLTYPE:1,Arcane=9,Divine=9"]),
    (RuleSetId::Apg, 43, "Master Alchemist", &["PRESKILL:1,Craft (Alchemy)=5"]),
    (RuleSetId::Apg, 44, "Minor Spell Expertise", &["PRESPELLTYPE:1,Arcane=4,Divine=4"]),
    (RuleSetId::Apg, 45, "Parry Spell", &["PREABILITY:1,CATEGORY=FEAT,Improved Counterspell", "PRESKILL:1,Spellcraft=15"]),
    (RuleSetId::Apg, 46, "Pass for Human", &["PREMULT:1,[PRERACE:1,Half-Elf,Half-Orc],[PREMULT:2,[PRERACE:1,RACESUBTYPE=Halfling],[PREABILITY:1,CATEGORY=FEAT,Childlike]]"]),
    (RuleSetId::Apg, 47, "Racial Heritage", &["PRERACE:1,RACESUBTYPE=Human"]),
    (RuleSetId::Apg, 48, "Raging Vitality", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Rage", "PRESTAT:1,CON=15"]),
    (RuleSetId::Apg, 49, "Razortusk", &["PRERACE:1,Half-orc"]),
    (RuleSetId::Apg, 50, "Shared Insight", &["PRERACE:1,Half-Elf", "PRESTAT:1,WIS=13"]),
    (RuleSetId::Apg, 51, "Sharp Senses", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.KeenSenses"]),
    (RuleSetId::Apg, 52, "Smell Fear", &["PREABILITY:1,CATEGORY=FEAT,Keen Scent", "PRERACE:1,RACESUBTYPE=Orc"]),
    (RuleSetId::Apg, 53, "Sociable", &["PRERACE:1,Half-Elf", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Apg, 54, "Spell Perfection", &["PREABILITY:3,CHECKMULT,CATEGORY=FEAT,TYPE.Metamagic", "PRESKILL:1,Spellcraft=15"]),
    (RuleSetId::Apg, 55, "Spider Step", &["PRECLASS:1,Monk=6", "PRESKILL:2,Acrobatics=6,Climb=6"]),
    (RuleSetId::Apg, 56, "Steel Soul", &["PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Hardy", "PRERACE:1,RACESUBTYPE=Dwarf"]),
    (RuleSetId::Apg, 57, "Stone-Faced", &["PRERACE:1,RACESUBTYPE=Dwarf"]),
    (RuleSetId::Apg, 58, "Stone Sense", &["PREABILITY:1,CATEGORY=FEAT,Improved Stonecunning", "PRESKILL:1,Perception=10"]),
    (RuleSetId::Apg, 59, "Stone Singer", &["PREABILITY:1,CATEGORY=Special Ability,Bard ~ Bardic Performance", "PRERACE:1,RACESUBTYPE=Dwarf", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Apg, 60, "Summoner's Call", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Eidolon"]),
    (RuleSetId::Apg, 61, "Taunt", &["PRESIZELTEQ:S", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Apg, 62, "Tenacious Transmutation", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Transmutation)"]),
    (RuleSetId::Apg, 63, "Verminheart", &["PREABILITY:1,CATEGORY=Special Ability,Wild Empathy,TYPE.WildEmpathy"]),
    (RuleSetId::Apg, 64, "War Singer", &["PREABILITY:1,CATEGORY=Special Ability,Bard ~ Bardic Performance", "PRERACE:1,RACESUBTYPE=Orc", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Apg, 65, "Well-Prepared", &["PRERACE:1,RACESUBTYPE=Halfling"]),
    (RuleSetId::Apg, 66, "Blood of Heroes", &["PREABILITY:1,CATEGORY=FEAT,Hero's Fortune", "PREVAREQ:Heroic,1"]),
    (RuleSetId::Apg, 67, "Hero's Fortune", &["PREVAREQ:Heroic,1"]),
    (RuleSetId::Apg, 68, "Luck of Heroes", &["PREABILITY:1,CATEGORY=FEAT,Hero's Fortune", "PREVAREQ:Heroic,1"]),
    (RuleSetId::Apg, 69, "Bashing Finish", &["PREABILITY:3,CATEGORY=FEAT,Improved Shield Bash,Shield Master,Two-Weapon Fighting", "PRETOTALAB:11"]),
    (RuleSetId::Apg, 70, "Bloody Assault", &["PREABILITY:1,CATEGORY=FEAT,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:6"]),
    (RuleSetId::Apg, 71, "Bodyguard", &["PREABILITY:1,CATEGORY=FEAT,Combat Reflexes"]),
    (RuleSetId::Apg, 72, "Bull Rush Strike", &["PREABILITY:2,CATEGORY=FEAT,Improved Bull Rush,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:9"]),
    (RuleSetId::Apg, 73, "Charge Through", &["PREABILITY:2,CATEGORY=FEAT,Improved Overrun,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:1"]),
    (RuleSetId::Apg, 74, "Cockatrice Strike", &["PREABILITY:3,CATEGORY=FEAT,Improved Unarmed Strike,Gorgon's Fist,Medusa's Wrath", "PRETOTALAB:14"]),
    (RuleSetId::Apg, 75, "Combat Patrol", &["PREABILITY:2,CATEGORY=FEAT,Combat Reflexes,Mobility", "PRETOTALAB:5"]),
    (RuleSetId::Apg, 78, "Covering Defense", &["PREABILITY:1,CATEGORY=FEAT,Shield Focus", "PRETOTALAB:6"]),
    (RuleSetId::Apg, 79, "Crippling Critical", &["PREABILITY:1,CATEGORY=FEAT,Critical Focus", "PRETOTALAB:13"]),
    (RuleSetId::Apg, 80, "Crossbow Mastery", &["PREABILITY:3,CATEGORY=FEAT,Point-Blank Shot,Rapid Reload,Rapid Shot", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,15],[PREVARGTEQ:FeatDexRequirement,15]"]),
    (RuleSetId::Apg, 81, "Dastardly Finish", &["PREVARGTEQ:SneakAttackDice,5"]),
    (RuleSetId::Apg, 82, "Dazing Assault", &["PREABILITY:1,CATEGORY=FEAT,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:11"]),
    (RuleSetId::Apg, 83, "Disarming Strike", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Disarm", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:9"]),
    (RuleSetId::Apg, 84, "Disrupting Shot", &["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot", "PRECLASS:1,Fighter=6", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]"]),
    (RuleSetId::Apg, 85, "Dreadful Carnage", &["PREABILITY:2,CATEGORY=FEAT,Power Attack,Furious Focus", "PRESTAT:1,STR=15", "PRETOTALAB:11"]),
    (RuleSetId::Apg, 86, "Eldritch Claws", &["PRESTAT:1,STR=15", "PRETOTALAB:6", "PREWEAPONPROF:1,TYPE.Natural"]),
    (RuleSetId::Apg, 87, "Elemental Fist", &["PREMULT:1,[PREVAREQ:ElementalFistGranted,1],[PREABILITY:1,CATEGORY=FEAT,Dragon Ferocity],[PREMULT:3,[PRETOTALAB:8],[PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike],[PRESTAT:2,CON=13,WIS=13]]"]),
    (RuleSetId::Apg, 90, "Elven Accuracy", &["PRERACE:1,RACESUBTYPE=Elf"]),
    (RuleSetId::Apg, 91, "Enforcer", &["PRESKILL:1,Intimidate=1"]),
    (RuleSetId::Apg, 92, "Focused Shot", &["PREABILITY:2,CATEGORY=FEAT,Point-Blank Shot,Precise Shot", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]"]),
    (RuleSetId::Apg, 93, "Following Step", &["PREABILITY:1,CATEGORY=FEAT,Step Up", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]"]),
    (RuleSetId::Apg, 94, "Furious Focus", &["PREABILITY:1,CATEGORY=FEAT,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:1"]),
    (RuleSetId::Apg, 95, "Gang Up", &["PREABILITY:1,CATEGORY=FEAT,Combat Expertise", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]"]),
    (RuleSetId::Apg, 96, "Greater Blind-Fight", &["PREABILITY:1,CATEGORY=FEAT,Improved Blind-Fight", "PRESKILL:1,Perception=15"]),
    (RuleSetId::Apg, 97, "Greater Dirty Trick", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Dirty Trick", "PREMULT:1,[PREVARGTEQ:PreStatScore_INT,13],[PREVARGTEQ:CombatFeatIntRequirement,13],[PREVARGTEQ:DirtyTricksterIntQualify,1]", "PRETOTALAB:6"]),
    (RuleSetId::Apg, 98, "Greater Drag", &["PREABILITY:1,CATEGORY=FEAT,Improved Drag,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:6"]),
    (RuleSetId::Apg, 99, "Greater Reposition", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Reposition", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:6"]),
    (RuleSetId::Apg, 100, "Greater Shield Specialization", &["PREABILITY:2,CATEGORY=FEAT,Greater Shield Focus,Shield Focus", "PRECLASS:1,Fighter=12"]),
    (RuleSetId::Apg, 101, "Greater Steal", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Steal", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:6"]),
    (RuleSetId::Apg, 102, "Improved Blind-Fight", &["PREABILITY:1,CATEGORY=FEAT,Blind-Fight", "PRESKILL:1,Perception=10"]),
    (RuleSetId::Apg, 103, "Improved Dirty Trick", &["PREABILITY:1,CATEGORY=FEAT,Combat Expertise", "PREMULT:1,[PREVARGTEQ:PreStatScore_INT,13],[PREVARGTEQ:CombatFeatIntRequirement,13],[PREVARGTEQ:DirtyTricksterIntQualify,1]"]),
    (RuleSetId::Apg, 104, "Improved Drag", &["PREABILITY:1,CATEGORY=FEAT,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:1"]),
    (RuleSetId::Apg, 105, "Improved Ki Throw", &["PREABILITY:2,CATEGORY=FEAT,Improved Bull Rush,Ki Throw"]),
    (RuleSetId::Apg, 106, "Improved Reposition", &["PREABILITY:1,CATEGORY=FEAT,Combat Expertise", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]"]),
    (RuleSetId::Apg, 107, "Improved Second Chance", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Second Chance", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:11"]),
    (RuleSetId::Apg, 108, "Improved Sidestep", &["PREABILITY:3,CATEGORY=FEAT,Dodge,Mobility,Sidestep", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,15],[PREVARGTEQ:FeatDexRequirement,15]"]),
    (RuleSetId::Apg, 109, "Improved Steal", &["PREABILITY:1,CATEGORY=FEAT,Combat Expertise", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]"]),
    (RuleSetId::Apg, 110, "In Harm's Way", &["PREABILITY:1,CATEGORY=FEAT,Bodyguard"]),
    (RuleSetId::Apg, 111, "Ki Throw", &["PREABILITY:2,CATEGORY=FEAT,Improved Trip,Improved Unarmed Strike"]),
    (RuleSetId::Apg, 113, "Low Profile", &["PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRESIZELTEQ:S"]),
    (RuleSetId::Apg, 114, "Missile Shield", &["PREABILITY:1,CATEGORY=FEAT,Shield Focus", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]"]),
    (RuleSetId::Apg, 115, "Mounted Shield", &["PREABILITY:2,CATEGORY=FEAT,Mounted Combat,Shield Focus"]),
    (RuleSetId::Apg, 116, "Mounted Skirmisher", &["PREABILITY:2,CATEGORY=FEAT,Mounted Combat,Trick Riding", "PRESKILL:1,Ride=14"]),
    (RuleSetId::Apg, 117, "Outflank", &["PRETOTALAB:4"]),
    (RuleSetId::Apg, 119, "Parting Shot", &["PREABILITY:4,CATEGORY=FEAT,Dodge,Mobility,Point-Blank Shot,Shot on the Run", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRETOTALAB:6"]),
    (RuleSetId::Apg, 120, "Perfect Strike", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRESTAT:1,WIS=13", "PRETOTALAB:8"]),
    (RuleSetId::Apg, 121, "Point Blank Master", &["PREABILITY:1,CATEGORY=FEAT,Weapon Specialization (TYPE=Ranged)", "PRETEXT:Weapon Specialization with selected ranged weapon."]),
    (RuleSetId::Apg, 122, "Practiced Tactician", &["PREABILITY:1,CATEGORY=Special Ability,Cavalier ~ Tactician"]),
    (RuleSetId::Apg, 123, "Precise Strike", &["PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRETOTALAB:1"]),
    (RuleSetId::Apg, 124, "Punishing Kick", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRESTAT:2,CON=13,WIS=13", "PRETOTALAB:8"]),
    (RuleSetId::Apg, 125, "Pushing Assault", &["PREABILITY:1,CATEGORY=FEAT,Power Attack", "PRESTAT:1,STR=15", "PRETOTALAB:1"]),
    (RuleSetId::Apg, 126, "Ray Shield", &["PREABILITY:2,CATEGORY=FEAT,Missile Shield,Spellbreaker", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,15],[PREVARGTEQ:FeatDexRequirement,15]"]),
    (RuleSetId::Apg, 127, "Rending Claws", &["PREMULT:2,[PREWEAPONPROF:1,Claw,Claws],[PREWEAPONPROF:1,TYPE=Natural]", "PRESTAT:1,STR=13", "PRETOTALAB:6"]),
    (RuleSetId::Apg, 128, "Repositioning Strike", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Reposition", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:9"]),
    (RuleSetId::Apg, 129, "Saving Shield", &["PREABILITY:1,CATEGORY=FEAT,Shield Proficiency"]),
    (RuleSetId::Apg, 130, "Second Chance", &["PREABILITY:1,CATEGORY=FEAT,Combat Expertise", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:6"]),
    (RuleSetId::Apg, 131, "Shadow Strike", &["PRETOTALAB:1"]),
    (RuleSetId::Apg, 132, "Shield of Swings", &["PREABILITY:1,CATEGORY=FEAT,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:1"]),
    (RuleSetId::Apg, 133, "Shield Specialization", &["PREABILITY:2,CATEGORY=FEAT,Shield Proficiency,Shield Focus", "PRECLASS:1,Fighter=4"]),
    (RuleSetId::Apg, 134, "Shield Wall", &["PREMULT:1,[PREPROFWITHSHIELD:3,TYPE.Buckler,TYPE.Light,TYPE.Heavy],[PREABILITY:1,CATEGORY=FEAT,Shield Proficiency]"]),
    (RuleSetId::Apg, 135, "Sidestep", &["PREABILITY:2,CATEGORY=FEAT,Dodge,Mobility", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]"]),
    (RuleSetId::Apg, 136, "Smash", &["PREABILITY:1,CATEGORY=FEAT,Power Attack", "PRERACE:1,Half-Orc"]),
    (RuleSetId::Apg, 137, "Stabbing Shot", &["PREABILITY:1,CATEGORY=FEAT,Rapid Shot", "PRERACE:1,RACESUBTYPE=Elf"]),
    (RuleSetId::Apg, 138, "Step Up and Strike", &["PREABILITY:2,CATEGORY=FEAT,Following Step,Step Up", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRETOTALAB:6"]),
    (RuleSetId::Apg, 139, "Stunning Assault", &["PREABILITY:1,CATEGORY=FEAT,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:16"]),
    (RuleSetId::Apg, 140, "Sundering Strike", &["PREABILITY:1,CATEGORY=FEAT,Improved Sunder,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:9"]),
    (RuleSetId::Apg, 142, "Swift Aid", &["PREABILITY:1,CATEGORY=FEAT,Combat Expertise", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:6"]),
    (RuleSetId::Apg, 143, "Team Up", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Gang Up", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:6"]),
    (RuleSetId::Apg, 144, "Teleport Tactician", &["PREABILITY:3,CATEGORY=FEAT,Combat Reflexes,Disruptive,Spellbreaker"]),
    (RuleSetId::Apg, 145, "Touch of Serenity", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRESTAT:1,WIS=18", "PRETOTALAB:8"]),
    (RuleSetId::Apg, 146, "Trick Riding", &["PREABILITY:1,CATEGORY=FEAT,Mounted Combat", "PRESKILL:1,Ride=9"]),
    (RuleSetId::Apg, 147, "Tripping Strike", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Trip", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:9"]),
    (RuleSetId::Apg, 148, "Under and Over", &["PREABILITY:1,CATEGORY=FEAT,Agile Maneuvers", "PRESIZELTEQ:S"]),
    (RuleSetId::Apg, 149, "Underfoot", &["PREABILITY:2,CATEGORY=FEAT,Dodge,Mobility", "PRESIZELTEQ:S"]),
    (RuleSetId::Apg, 164, "Preferred Spell", &["PREABILITY:1,CATEGORY=FEAT,Heighten Spell", "PRESKILL:1,Spellcraft=5"]),
    (RuleSetId::Apg, 166, "Selective Spell", &["PRESKILL:1,Spellcraft=10"]),
    (RuleSetId::Apg, 169, "Allied Spellcaster", &["PREMULT:1,[PRECLASS:1,SPELLCASTER=1],[PREVARGTEQ:CasterLevel_Highest,1]"]),
    // Advanced Class Guide — 125 of 129 record(s) in the hand-authored table carry at least one `PRE`-family token.
    (RuleSetId::Acg, 0, "Aberrant Tumor", &["PREABILITY:1,CATEGORY=Special Ability,Bloodrager Bloodline ~ Aberrant,Arcanist Bloodline ~ Aberrant,Sorcerer Bloodline ~ Aberrant,Crossbloodline ~ Aberrant,TYPE.AberrantBloodline"]),
    (RuleSetId::Acg, 1, "Amateur Investigator", &["!PREABILITY:1,CATEGORY=Special Ability,Investigator ~ Inspiration", "PRESKILL:1,TYPE.Knowledge=1", "PRESTAT:1,INT=13"]),
    (RuleSetId::Acg, 2, "Animal Soul", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Companion,TYPE.Mount,TYPE.Special Mount"]),
    (RuleSetId::Acg, 3, "Believer's Boon", &["!PREABILITY:2,CHECKMULT,CATEGORY=FEAT,Believer's Boon", "PRESTAT:1,WIS=13"]),
    (RuleSetId::Acg, 4, "Believer's Hands", &["PREABILITY:1,CATEGORY=FEAT,Believer's Boon", "PREALIGN:LG", "PREDEITYALIGN:LG,LN,NG", "PRESTAT:1,WIS=13"]),
    (RuleSetId::Acg, 5, "Blasting Charge", &["PREABILITY:1,CATEGORY=Special Ability,Bloodrager ~ Bloodline", "PRETOTALAB:7", "PREVARGTEQ:BloodlineCasterLVL,2"]),
    (RuleSetId::Acg, 6, "Blessed Striker", &["PRECLASS:1,SPELLCASTER.Divine=1", "PRETOTALAB:11"]),
    (RuleSetId::Acg, 7, "Bookish Rogue", &["PREABILITY:1,CATEGORY=Special Ability,Rogue Talent ~ Minor Magic"]),
    (RuleSetId::Acg, 8, "Channeled Blessing", &["PREABILITY:1,CATEGORY=Special Ability,Warpriest ~ Blessings", "PREABILITY:1,CATEGORY=Special Ability,TYPE.ChannelEnergy"]),
    (RuleSetId::Acg, 9, "Disable Dweomer", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Trapfinding", "PRESKILL:2,Disable Device=5,Use Magic Device=5"]),
    (RuleSetId::Acg, 10, "Divine Protection", &["PRESKILL:1,Knowledge (Religion)=5", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Acg, 11, "Energy Channel", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.ChannelEnergy", "PREMULT:1,[PREDOMAIN:1,Air,Earth,Fire,Water],[PREABILITY:1,CATEGORY=Special Ability,Blessings ~ Air Blessing,Blessings ~ Earth Blessing,Blessings ~ Fire Blessing,Blessings ~ Water Blessing]"]),
    (RuleSetId::Acg, 12, "Esoteric Linguistics", &["PREABILITY:1,CATEGORY=FEAT,Skill Focus (Linguistics)"]),
    (RuleSetId::Acg, 13, "Evolved Companion", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Companion", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Acg, 14, "Evolved Summoned Monster", &["PREABILITY:2,CATEGORY=FEAT,Augment Summoning,Spell Focus (Conjuration)", "PREMULT:1,[PRESPELL:1,Summon Monster I],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Summon Monster SLA]", "PRETEXT:Augmented Summoning, Spell Focus (conjuration), ability to cast summon monster I."]),
    (RuleSetId::Acg, 15, "Expanded Preparation", &["!PREABILITY:3,CHECKMULT,CATEGORY=FEAT,Expanded Preparation", "PRECLASS:1,Arcanist=1"]),
    (RuleSetId::Acg, 16, "Extended Animal Focus", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Focus"]),
    (RuleSetId::Acg, 17, "Extra Arcanist Exploit", &["PREABILITY:1,CATEGORY=Special Ability,Arcanist ~ Arcanist Exploits"]),
    (RuleSetId::Acg, 18, "Extra Inspiration", &["PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Amateur Investigator],[PREABILITY:1,CATEGORY=Special Ability,Investigator ~ Inspiration]", "PREMULT:1,[!PREABILITY:1,CATEGORY=FEAT,Extra Inspiration],[PREABILITY:1,CATEGORY=Special Ability,Investigator ~ Inspiration]"]),
    (RuleSetId::Acg, 19, "Extra Investigator Talent", &["PREABILITY:1,CATEGORY=Special Ability,Investigator ~ Investigator Talents"]),
    (RuleSetId::Acg, 20, "Extra Martial Flexibility", &["PREABILITY:1,CATEGORY=Special Ability,Brawler ~ Martial Flexibility"]),
    (RuleSetId::Acg, 21, "Extra Reservoir", &["PREABILITY:1,CATEGORY=Special Ability,Arcanist ~ Arcane Reservoir,TYPE.Arcane Reservoir"]),
    (RuleSetId::Acg, 22, "Extra Slayer Talent", &["PREABILITY:1,CATEGORY=Special Ability,Slayer ~ Slayer Talents"]),
    (RuleSetId::Acg, 23, "Favored Enemy Spellcasting", &["PRECLASS:1,SPELLCASTER=1"]),
    (RuleSetId::Acg, 24, "Flexible Hex", &["PREABILITY:1,CATEGORY=Special Ability,Shaman ~ Wandering Hex"]),
    (RuleSetId::Acg, 25, "Flexible Wizardry", &["PREABILITY:1,CATEGORY=FEAT,Spell Mastery", "PREVARGTEQ:FeatQualifier_WizardLVL,1"]),
    (RuleSetId::Acg, 26, "Focused Inspiration", &["PREABILITY:2,CATEGORY=Special Ability,Investigator ~ Inspiration,Investigator ~ Keen Recollection"]),
    (RuleSetId::Acg, 27, "Force Dash", &["PRECLASS:1,SPELLCASTER.Arcane=2", "PRESPELLDESCRIPTOR:1,Force=0", "PRETOTALAB:4"]),
    (RuleSetId::Acg, 28, "Formula Recollection", &["PREABILITY:1,CATEGORY=Special Ability,Investigator ~ Keen Recollection", "PRESKILL:1,Spellcraft=5"]),
    (RuleSetId::Acg, 29, "Greater Dirge of Doom", &["PREABILITY:1,CATEGORY=FEAT,Improved Dirge of Doom", "PREABILITY:1,CATEGORY=Special Ability,Skald ~ Dirge of Doom,Bardic Performance ~ Dirge of Doom,TYPE.DirgeOfDoom"]),
    (RuleSetId::Acg, 30, "Greater Skald's Vigor", &["PREABILITY:1,CATEGORY=Special Ability,TYPE=SkaldRagingSong", "PREABILITY:1,CATEGORY=FEAT,Skald's Vigor", "PRESKILL:1,Perform (Sing)=10"]),
    (RuleSetId::Acg, 31, "Improved Dirge of Doom", &["PREABILITY:1,CATEGORY=Special Ability,Skald ~ Dirge of Doom,Bardic Performance ~ Dirge of Doom,TYPE.DirgeOfDoom"]),
    (RuleSetId::Acg, 32, "Improved Flexible Wizardry", &["PREABILITY:2,CATEGORY=FEAT,Flexible Wizardry,Spell Mastery", "PREVARGTEQ:FeatQualifier_WizardLVL,8"]),
    (RuleSetId::Acg, 33, "Improved Studied Combatant", &["PREMULT:5,[PRESTAT:1,INT=13],[PRETOTALAB:8],[PRESKILL:1,TYPE.Knowledge=1],[PREABILITY:2,CATEGORY=FEAT,Amateur Investigator,Studied Combatant],[!PREABILITY:1,CATEGORY=Special Ability,Investigator ~ Inspiration]"]),
    (RuleSetId::Acg, 34, "Insightful Delivery", &["PREABILITY:1,CATEGORY=Special Ability,Investigator ~ Poison Lore", "PREVARGTEQ:InvestigatorStudiedStrikeDice,4"]),
    (RuleSetId::Acg, 35, "Inspired Alchemy", &["PREABILITY:1,CATEGORY=Special Ability,Investigator ~ Inspiration", "PREABILITY:1,CATEGORY=Special Ability,Investigator ~ Alchemy,TYPE.Alchemy", "PRECLASS:1,Investigator=4"]),
    (RuleSetId::Acg, 36, "Inspired by Fear", &["PREABILITY:1,CATEGORY=Special Ability,Skald ~ Dirge of Doom,Bardic Performance ~ Dirge of Doom"]),
    (RuleSetId::Acg, 37, "Inspired Strike", &["PREABILITY:1,CATEGORY=Special Ability,Investigator ~ Studied Combat"]),
    (RuleSetId::Acg, 38, "Intimidating Performance", &["PREABILITY:1,CATEGORY=Special Ability,Skald ~ Raging Song,Bard ~ Bardic Performance", "PRECLASS:1,Skald=7,Bard=7"]),
    (RuleSetId::Acg, 39, "Lay of the Land", &["PREABILITY:2,CATEGORY=Special Ability,Hunter ~ Animal Focus,Hunter ~ Wild Empathy", "!PREABILITY:4,CHECKMULT,CATEGORY=FEAT,Lay of the Land"]),
    (RuleSetId::Acg, 40, "Lunging Spell Touch", &["PRESKILL:1,Spellcraft=5"]),
    (RuleSetId::Acg, 41, "Manifested Blood", &["PRESTAT:1,CHA=13", "PRETEXT:Draconic or elemental bloodline"]),
    (RuleSetId::Acg, 42, "Nature Magic", &["PRESKILL:1,Knowledge (Nature)=1"]),
    (RuleSetId::Acg, 43, "Orator", &["PREABILITY:1,CATEGORY=FEAT,Skill Focus (Linguistics)"]),
    (RuleSetId::Acg, 44, "Quicken Blessing", &["PREABILITY:1,CATEGORY=Special Ability,Warpriest ~ Blessings", "PREVARGTEQ:WarpriestLVL,WarpriestMajorBlessingGrantedLVL"]),
    (RuleSetId::Acg, 45, "Raging Blood", &["PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Eldritch Heritage],[PREABILITY:1,CATEGORY=Special Ability,Sorcerer ~ Standard Bloodline]"]),
    (RuleSetId::Acg, 46, "Ranged Study", &["PREMULT:2,[PREABILITY:1,CATEGORY=FEAT,Weapon Focus],[PREABILITY:1,CATEGORY=Special Ability,Investigator ~ Studied Combat]"]),
    (RuleSetId::Acg, 47, "Reactive Healing", &["PREABILITY:1,CATEGORY=FEAT,Quick Channel,Quicken Spell", "PREABILITY:1,CATEGORY=Special Ability,TYPE.ChannelEnergy,TYPE.Channel Energy,TYPE.Lay on Hands,TYPE.LayOnHands"]),
    (RuleSetId::Acg, 48, "Resilient Armor", &["PREABILITY:1,CATEGORY=Special Ability,Warpriest ~ Sacred Armor,Paladin ~ Divine Bond"]),
    (RuleSetId::Acg, 49, "Skilled Rager", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Rage"]),
    (RuleSetId::Acg, 50, "Slow Faller", &["PREMULT:1,[PRESKILL:1,Acrobatics=5],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Slow Fall]"]),
    (RuleSetId::Acg, 51, "Spirit Talker", &["PREABILITY:1,CATEGORY=Special Ability,Shaman ~ Hex,Witch ~ Hex", "PRECLASS:1,Shaman=6,Witch=6"]),
    (RuleSetId::Acg, 52, "Spirit's Gift", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Companion,TYPE.Familiar"]),
    (RuleSetId::Acg, 53, "Spiritual Guardian", &["PREABILITY:1,CATEGORY=Special Ability,Shaman ~ Spirit Magic", "PRESPELL:1,Spiritual Weapon,Spiritual Ally"]),
    (RuleSetId::Acg, 54, "Spontaneous Nature's Ally", &["PREABILITY:1,CATEGORY=Special Ability,Warpriest ~ Spontaneous Casting,Cleric ~ Spontaneous Casting", "PREDOMAIN:1,Animal,Plant", "PRESKILL:1,Knowledge (Nature)=5"]),
    (RuleSetId::Acg, 55, "Stalker's Focus", &["!PREABILITY:2,CATEGORY=Special Ability,Hunter ~ Animal Focus,Sacred Huntsmaster ~ Animal Focus", "PRESKILL:2,Knowledge (Nature)=3,Survival=3"]),
    (RuleSetId::Acg, 57, "Studied Combatant", &["PREMULT:5,[PRESTAT:1,INT=13],[PREABILITY:1,CATEGORY=FEAT,Amateur Investigator],[PRETOTALAB:6],[PRESKILL:1,TYPE.Knowledge=1],[!PREABILITY:1,CATEGORY=Special Ability,Investigator ~ Inspiration]"]),
    (RuleSetId::Acg, 58, "Surprise Maneuver", &["PREABILITY:1,CATEGORY=FEAT,Combat Expertise", "PREMULT:1,[PREVARGTEQ:SneakAttackDice,3],[PREVARGTEQ:InvestigatorStudiedStrikeDice,3]"]),
    (RuleSetId::Acg, 59, "Talented Magician", &["PREABILITY:2,CATEGORY=Special Ability,Rogue Talent ~ Major Magic,Rogue Talent ~ Minor Magic"]),
    (RuleSetId::Acg, 60, "Unfettered Familiar", &["PRECLASS:1,SPELLCASTER=5"]),
    (RuleSetId::Acg, 61, "War Blessing", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.OracleMystery,TYPE.Domains,TYPE.CF_Domain"]),
    (RuleSetId::Acg, 62, "Amateur Swashbuckler", &["!PREABILITY:1,CATEGORY=Special Ability,TYPE.Panache"]),
    (RuleSetId::Acg, 63, "Anticipate Dodge", &["PREABILITY:2,CATEGORY=FEAT,Dodge,Mobility", "PREMULT:1,[PRETOTALAB:7],[PREVARGTEQ:MonkFeatQualify,4]"]),
    (RuleSetId::Acg, 64, "Barroom Brawler", &["PRETOTALAB:4"]),
    (RuleSetId::Acg, 65, "Battle Cry", &["PREMULT:1,[PRETOTALAB:5],[PRESKILL:1,Perform (Oratory)=5],[PRESKILL:1,Perform (Sing)=5],[PRESKILL:1,Perform (Act)=5]", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Acg, 66, "Befuddling Strike", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRESTAT:1,WIS=13", "PRETOTALAB:8"]),
    (RuleSetId::Acg, 67, "Blooded Arcane Strike", &["PREABILITY:1,CATEGORY=Special Ability,Bloodrager ~ Bloodrage", "PREABILITY:1,CATEGORY=FEAT,Arcane Strike", "PRECLASS:1,SPELLCASTER.Arcane=1"]),
    (RuleSetId::Acg, 68, "Canny Tumble", &["PREABILITY:2,CATEGORY=FEAT,Dodge,Mobility", "PRESKILL:1,Acrobatics=5"]),
    (RuleSetId::Acg, 69, "Channeling Force", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.ChannelEnergy", "PRESPELLDESCRIPTOR:1,Force=0"]),
    (RuleSetId::Acg, 70, "Coordinated Shot", &["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot"]),
    (RuleSetId::Acg, 71, "Counter Reflexes", &["PREABILITY:3,CATEGORY=FEAT,Dodge,Mobility,Anticipate Dodge", "PREMULT:1,[PRETOTALAB:9],[PREVARGTEQ:MonkFeatQualify,6]"]),
    (RuleSetId::Acg, 72, "Counterpunch", &["PREMULT:2,[PREABILITY:3,CATEGORY=FEAT,Combat Reflexes,Weapon Focus (Unarmed Strike),Improved Unarmed Strike],[PREMULT:1,[PRETOTALAB:16],[PRECLASS:1,Brawler=12]]", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,18],[PREVARGTEQ:FeatDexRequirement,18]"]),
    (RuleSetId::Acg, 73, "Dazing Fist", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRESTAT:1,WIS=13", "PRETOTALAB:4"]),
    (RuleSetId::Acg, 74, "Disheartening Display", &["PREMULT:2,[PREABILITY:2,CATEGORY=FEAT,Weapon Focus,Dazzling Display],[PRETOTALAB:6]"]),
    (RuleSetId::Acg, 76, "Draining Strike", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRESTAT:1,WIS=13", "PRETOTALAB:8"]),
    (RuleSetId::Acg, 77, "Dual Enhancement", &["PREABILITY:1,CATEGORY=FEAT,Two-Weapon Fighting", "PREABILITY:1,CATEGORY=Special Ability,Warpriest ~ Sacred Weapon,Divine Bond ~ Celestial Spirit"]),
    (RuleSetId::Acg, 78, "Dueling Cape Deed", &["PREABILITY:1,CATEGORY=FEAT,Dodge", "PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Amateur Swashbuckler],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Panache]", "PRESKILL:1,Sleight of Hand=1"]),
    (RuleSetId::Acg, 79, "Extreme Prejudice", &["PREABILITY:1,CATEGORY=FEAT,Seething Hatred", "PREABILITY:1,CATEGORY=Special Ability,Slayer ~ Studied Target", "PREVARGTEQ:SneakAttackDice,3"]),
    (RuleSetId::Acg, 80, "Faerie's Strike", &["PREABILITY:1,CATEGORY=FEAT,Vital Strike", "PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Nature Magic],[PRECLASS:2,SPELLCASTER=1,Ranger=4],[PRECLASS:2,SPELLCASTER=1,Druid=1],[PRECLASS:2,SPELLCASTER=1,Hunter=1]", "PRESKILL:1,Knowledge (Nature)=5"]),
    (RuleSetId::Acg, 81, "Grabbing Drag", &["PREABILITY:2,CATEGORY=FEAT,Grabbing Style,Improved Grapple", "PREMULT:1,[PRETOTALAB:8],[PREVARGTEQ:MonkFeatQualify,4],[PRECLASS:1,Monk=4,Brawler=4]"]),
    (RuleSetId::Acg, 82, "Grabbing Master", &["PREABILITY:3,CATEGORY=FEAT,Grabbing Drag,Grabbing Style,Improved Grapple", "PREMULT:1,[PRETOTALAB:12],[PREVARGTEQ:MonkFeatQualify,8],[PRECLASS:1,Monk=8,Brawler=8]"]),
    (RuleSetId::Acg, 83, "Grabbing Style", &["PREABILITY:1,CATEGORY=FEAT,Improved Grapple", "PREMULT:1,[PRETOTALAB:6],[PREABILITY:1,CATEGORY=Special Ability,Brawler ~ Brawler's Flurry],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Flurry of Blows]"]),
    (RuleSetId::Acg, 84, "Grasping Strike", &["PREABILITY:1,CATEGORY=FEAT,Vital Strike", "PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Nature Magic],[PRECLASS:2,SPELLCASTER=1,Ranger=4],[PRECLASS:2,SPELLCASTER=1,Druid=1],[PRECLASS:2,SPELLCASTER=1,Hunter=1]", "PRESKILL:1,Knowledge (Nature)=5"]),
    (RuleSetId::Acg, 85, "Greater Weapon of the Chosen", &["PREABILITY:3,CATEGORY=FEAT,Improved Weapon of the Chosen,Weapon of the Chosen,Weapon Focus"]),
    (RuleSetId::Acg, 86, "Gruesome Slaughter", &["PREABILITY:2,CATEGORY=FEAT,Killing Flourish,Intimidating Prowess", "PRECLASS:1,Slayer=11", "PRESKILL:1,Intimidate=11"]),
    (RuleSetId::Acg, 87, "Improved Awesome Blow", &["PREABILITY:1,CATEGORY=FEAT,Power Attack", "PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Brawler ~ Awesome Blow,TYPE.AwesomeBlow],[PREABILITY:1,CATEGORY=FEAT,Awesome Blow]", "PRESTAT:1,STR=13"]),
    (RuleSetId::Acg, 88, "Improved Swap Places", &["PREABILITY:1,CATEGORY=FEAT,Swap Places"]),
    (RuleSetId::Acg, 89, "Improved Weapon of the Chosen", &["PREABILITY:2,CATEGORY=FEAT,Weapon of the Chosen,Weapon Focus"]),
    (RuleSetId::Acg, 91, "Jabbing Dancer", &["PREABILITY:4,CATEGORY=FEAT,Dodge,Improved Unarmed Strike,Jabbing Style,Mobility", "PREMULT:1,[PRETOTALAB:9],[PREVARGTEQ:MonkFeatQualify,5],[PRECLASS:1,Monk=5,Brawler=5]"]),
    (RuleSetId::Acg, 92, "Jabbing Master", &["PREABILITY:6,CATEGORY=FEAT,Dodge,Improved Unarmed Strike,Jabbing Dancer,Jabbing Style,Mobility,Power Attack", "PREMULT:1,[PRETOTALAB:12],[PREVARGTEQ:MonkFeatQualify,8],[PRECLASS:1,Monk=8,Brawler=8]"]),
    (RuleSetId::Acg, 93, "Jabbing Style", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:6],[PREABILITY:1,CATEGORY=Special Ability,Brawler ~ Brawler's Flurry],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Flurry of Blows]"]),
    (RuleSetId::Acg, 94, "Kick Up", &["PREABILITY:1,CATEGORY=FEAT,Acrobatic", "PRECLASS:1,Slayer=1,Swashbuckler=1", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRESKILL:1,Acrobatics=1"]),
    (RuleSetId::Acg, 95, "Killing Flourish", &["PREABILITY:1,CATEGORY=FEAT,Intimidating Prowess", "PRECLASS:1,Slayer=4", "PRESKILL:1,Intimidate=4"]),
    (RuleSetId::Acg, 96, "Merciless Butchery", &["PREABILITY:1,CATEGORY=FEAT,Dastardly Finish", "PREABILITY:1,CATEGORY=Special Ability,Slayer ~ Studied Target", "PREVARGTEQ:SneakAttackDice,5"]),
    (RuleSetId::Acg, 97, "Paralyzing Strike", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRESTAT:1,WIS=13", "PRETOTALAB:14"]),
    (RuleSetId::Acg, 98, "Pummeling Bully", &["PREABILITY:4,CATEGORY=FEAT,Improved Reposition,Improved Trip,Improved Unarmed Strike,Pummeling Style", "PREMULT:1,[PRETOTALAB:9],[PREVARGTEQ:MonkFeatQualify,5],[PRECLASS:1,Monk=5,Brawler=5]"]),
    (RuleSetId::Acg, 99, "Pummeling Charge", &["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Pummeling Style", "PREMULT:1,[PRETOTALAB:12],[PREVARGTEQ:MonkFeatQualify,8],[PRECLASS:1,Monk=8,Brawler=8]"]),
    (RuleSetId::Acg, 100, "Pummeling Style", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:6],[PREABILITY:1,CATEGORY=Special Ability,Brawler ~ Brawler's Flurry,TYPE.Flurry of Blows]"]),
    (RuleSetId::Acg, 101, "Rage Casting", &["PREABILITY:1,CATEGORY=Special Ability,Bloodrager ~ Blood Casting", "PRECLASS:1,SPELLCASTER.Arcane=1"]),
    (RuleSetId::Acg, 102, "Raging Absorption", &["PREABILITY:1,CATEGORY=Special Ability,Bloodrager ~ Bloodrage", "PRECLASS:1,SPELLCASTER.Arcane=2"]),
    (RuleSetId::Acg, 103, "Raging Concentration", &["PREABILITY:1,CATEGORY=Special Ability,Bloodrager ~ Blood Casting", "PRECLASS:1,SPELLCASTER.Arcane=1"]),
    (RuleSetId::Acg, 104, "Reckless Rage", &["PREABILITY:1,CATEGORY=FEAT,Power Attack", "PREABILITY:1,CATEGORY=Special Ability,TYPE=Rage,TYPE=SkaldRagingSong"]),
    (RuleSetId::Acg, 105, "Recovered Rage", &["PREABILITY:1,CATEGORY=Special Ability,TYPE=Rage,TYPE=SkaldRagingSong"]),
    (RuleSetId::Acg, 106, "Riving Strike", &["PREABILITY:1,CATEGORY=FEAT,Arcane Strike", "PRECLASS:1,SPELLCASTER.Arcane=1"]),
    (RuleSetId::Acg, 107, "Seething Hatred", &["PREABILITY:1,CATEGORY=Special Ability,Slayer ~ Studied Target"]),
    (RuleSetId::Acg, 108, "Seize Advantage", &["PREABILITY:1,CATEGORY=Special Ability,Swashbuckler ~ Opportune Parry and Riposte"]),
    (RuleSetId::Acg, 109, "Silent Kill", &["PREABILITY:1,CATEGORY=Special Ability,Slayer Talent ~ Assassinate,Ninja Trick ~ Assassinate", "PRESKILL:1,Stealth=12"]),
    (RuleSetId::Acg, 110, "Skald's Vigor", &["PREABILITY:1,CATEGORY=Special Ability,TYPE=SkaldRagingSong"]),
    (RuleSetId::Acg, 111, "Slashing Grace", &["PREVAREQ:HasWeaponFinesseFeat,1", "PREABILITY:1,CATEGORY=FEAT,Weapon Focus", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]"]),
    (RuleSetId::Acg, 112, "Slayer's Feint", &["PREABILITY:1,CATEGORY=FEAT,Combat Expertise", "PREMULT:1,[PRECLASS:1,Slayer=1],[PREABILITY:1,CATEGORY=FEAT,Acrobatic]", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,15],[PREVARGTEQ:FeatDexRequirement,15]", "PRESKILL:1,Acrobatics=1"]),
    (RuleSetId::Acg, 113, "Staggering Fist", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRESTAT:1,WIS=13", "PRETOTALAB:2"]),
    (RuleSetId::Acg, 114, "Stouthearted", &["PREABILITY:1,CATEGORY=Special Ability,TYPE=SkaldRagingSong"]),
    (RuleSetId::Acg, 115, "Twinned Feint", &["PRESTAT:1,CHA=13"]),
    (RuleSetId::Acg, 117, "Undersized Mount", &["PRESKILL:1,Ride=1"]),
    (RuleSetId::Acg, 118, "Weapon of the Chosen", &["PREABILITY:1,CATEGORY=FEAT,Weapon Focus", "PRECLASS:1,SPELLCASTER.Divine=1", "PRETEXT:Must have Weapon Focus with one of the deity's favored weapons."]),
    (RuleSetId::Acg, 119, "Winter's Strike", &["PREABILITY:1,CATEGORY=FEAT,Vital Strike", "PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Nature Magic],[PRECLASS:2,SPELLCASTER=1,Ranger=4],[PRECLASS:2,SPELLCASTER=1,Druid=1],[PRECLASS:2,SPELLCASTER=1,Hunter=1]", "PRESKILL:1,Knowledge (Nature)=5"]),
    (RuleSetId::Acg, 120, "Wounded Paw Gambit", &["PREABILITY:1,CATEGORY=FEAT,Broken Wing Gambit", "PRESKILL:1,Bluff=5"]),
    (RuleSetId::Acg, 121, "Improved Duck and Cover", &["PREABILITY:1,CATEGORY=FEAT,Duck and Cover"]),
    (RuleSetId::Acg, 122, "Improved Spell Sharing", &["PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Companion,TYPE.Eidolon,TYPE.Familiar,TYPE.Special Mount],[PREVARGT:MasterLevel,0]"]),
    (RuleSetId::Acg, 123, "Pack Flanking", &["PREABILITY:1,CATEGORY=FEAT,Combat Expertise", "PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Companion", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]"]),
    (RuleSetId::Acg, 124, "Share Healing", &["PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Companion,TYPE.Eidolon,TYPE.Familiar,TYPE.Special Mount,TYPE.Mount],[PREVARGT:MasterLevel,0]"]),
    (RuleSetId::Acg, 125, "Confounding Tumble Deed", &["PREABILITY:1,CATEGORY=FEAT,Canny Tumble", "PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Amateur Swashbuckler],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Panache]", "PRESKILL:1,Acrobatics=7"]),
    (RuleSetId::Acg, 126, "Disarming Threat Deed", &["PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Amateur Swashbuckler],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Panache]", "PRESKILL:2,Diplomacy=2,Intimidate=2"]),
    (RuleSetId::Acg, 127, "Extra Panache", &["PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Amateur Swashbuckler],[PREABILITY:1,CATEGORY=Special Ability,Swashbuckler ~ Panache,TYPE.Panache]", "PREMULT:1,[!PREABILITY:1,CATEGORY=FEAT,Extra Panache],[PREABILITY:1,CATEGORY=Special Ability,Swashbuckler ~ Panache,TYPE.Panache]"]),
    (RuleSetId::Acg, 128, "Pommel Strike Deed", &["PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Amateur Swashbuckler],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Panache]", "PRETOTALAB:3"]),
    // Advanced Race Guide — 187 of 187 record(s) in the hand-authored table carry at least one `PRE`-family token.
    (RuleSetId::Arg, 0, "Adaptive Fortune", &["PREABILITY:1,CATEGORY=Special Ability,Halfling ~ Adaptable Luck", "PREABILITY:1,CATEGORY=FEAT,Fortunate One", "PRELEVEL:MIN=10", "PREFACT:1,TEMPLATES,IsHalfling=true"]),
    (RuleSetId::Arg, 1, "Agile Tongue", &["PREFACT:1,TEMPLATES,IsGrippli=true"]),
    (RuleSetId::Arg, 2, "Airy Step", &["PREFACT:1,TEMPLATES,IsSylph=true"]),
    (RuleSetId::Arg, 3, "Angel Wings", &["PREABILITY:1,CATEGORY=FEAT,Angelic Blood", "PREPCLEVEL:MIN=10", "PREFACT:1,TEMPLATES,IsAasimar=true"]),
    (RuleSetId::Arg, 4, "Angelic Blood", &["PREFACT:1,TEMPLATES,IsAasimar=true", "PRESTAT:1,CON=13"]),
    (RuleSetId::Arg, 5, "Angelic Flesh", &["PREABILITY:1,CATEGORY=FEAT,Angelic Blood", "PREFACT:1,TEMPLATES,IsAasimar=true"]),
    (RuleSetId::Arg, 6, "Aquatic Ancestry", &["PREFACT:1,TEMPLATES,IsUndine=true"]),
    (RuleSetId::Arg, 7, "Armor of the Pit", &["PREFACT:1,TEMPLATES,IsTiefling=true"]),
    (RuleSetId::Arg, 8, "Attuned to the Wild", &["PREFACT:1,TEMPLATES,IsElf=true"]),
    (RuleSetId::Arg, 9, "Beast Rider", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Companion,TYPE.Special Mount", "PRELEVEL:MIN=7", "PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true"]),
    (RuleSetId::Arg, 10, "Bestow Luck", &["PREABILITY:2,CATEGORY=FEAT,Defiant Luck,Inexplicable Luck", "PREFACT:1,TEMPLATES,IsHuman=true"]),
    (RuleSetId::Arg, 11, "Black Cat", &["PREFACT:1,TEMPLATES,IsCatfolk=true"]),
    (RuleSetId::Arg, 12, "Blood Drinker", &["PREFACT:1,TEMPLATES,IsDhampir=true"]),
    (RuleSetId::Arg, 13, "Blood Feaster", &["PREABILITY:1,CATEGORY=FEAT,Blood Drinker", "PREFACT:1,TEMPLATES,IsDhampir=true", "PRETOTALAB:6"]),
    (RuleSetId::Arg, 14, "Blood Salvage", &["PREABILITY:1,CATEGORY=FEAT,Blood Drinker", "PREFACT:1,TEMPLATES,IsDhampir=true"]),
    (RuleSetId::Arg, 15, "Blood Vengeance", &["!PREALIGN:LG,LN,LE", "PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true"]),
    (RuleSetId::Arg, 16, "Born Alone", &["PREFACT:1,TEMPLATES,IsOrc=true"]),
    (RuleSetId::Arg, 17, "Brewmaster", &["PREFACT:1,TEMPLATES,IsDwarf=true", "PRESKILL:2,Craft (Alchemy)=1,Profession (Brewer)=1"]),
    (RuleSetId::Arg, 18, "Burn! Burn! Burn!", &["PREFACT:1,TEMPLATES,IsGoblin=true", "PRESKILL:1,Disable Device=1"]),
    (RuleSetId::Arg, 19, "Burrowing Teeth", &["PREABILITY:2,CATEGORY=FEAT,Sharpclaw,Tunnel Rat", "PREFACT:1,TEMPLATES,IsRatfolk=true"]),
    (RuleSetId::Arg, 20, "Carrion Feeder", &["PREFACT:1,TEMPLATES,IsTengu=true"]),
    (RuleSetId::Arg, 21, "Casual Illusionist", &["PREABILITY:1,CATEGORY=Special Ability,Gnome ~ Gnome Magic", "PREFACT:1,TEMPLATES,IsGnome=true"]),
    (RuleSetId::Arg, 22, "Catfolk Exemplar", &["PREFACT:1,TEMPLATES,IsCatfolk=true"]),
    (RuleSetId::Arg, 23, "Celestial Servant", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Companion,TYPE.Special Mount,TYPE.Familiar", "PREFACT:1,TEMPLATES,IsAasimar=true"]),
    (RuleSetId::Arg, 24, "Channel Force", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Energy", "PREMULT:1,[PREVARGTEQ:OracleChannelDice,2],[PREVARGTEQ:ClericChannelPositiveEnergyDice,2],[PREVARGTEQ:PaladinChannelDice,2],[PREVARGTEQ:ShamanChannelDice,2]", "PREFACT:1,TEMPLATES,IsAasimar=true", "PRETEXT:channel energy 2d6"]),
    (RuleSetId::Arg, 25, "Cloud Gazer", &["PREFACT:1,TEMPLATES,IsSylph=true"]),
    (RuleSetId::Arg, 26, "Courageous Resolve", &["PREABILITY:2,CATEGORY=Special Ability,Halfling ~ Craven,Halfling ~ Fearless", "PREFACT:1,TEMPLATES,IsHalfling=true"]),
    (RuleSetId::Arg, 27, "Dark Sight", &["PREABILITY:1,CATEGORY=FEAT,Gloom Sight", "PREFACT:1,TEMPLATES,IsFetchling=true"]),
    (RuleSetId::Arg, 28, "Dauntless Destiny", &["PREABILITY:1,CATEGORY=FEAT,Fearless Curiosity", "PREFACT:1,TEMPLATES,IsHuman=true", "PRESKILL:1,Intimidate=10", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Arg, 29, "Deafening Explosion", &["PREABILITY:1,CATEGORY=Special Ability,Bomb", "PREFACT:1,TEMPLATES,IsHobgoblin=true"]),
    (RuleSetId::Arg, 30, "Defiant Luck", &["PREFACT:1,TEMPLATES,IsHuman=true"]),
    (RuleSetId::Arg, 31, "Discerning Eye", &["PREABILITY:1,CATEGORY=Special Ability,Half-Elf ~ Keen Senses", "PREFACT:1,TEMPLATES,IsElf=true,IsHalfElf=true"]),
    (RuleSetId::Arg, 32, "Diverse Palate", &["PREABILITY:1,CATEGORY=FEAT,Blood Drinker", "PREFACT:1,TEMPLATES,IsDhampir=true"]),
    (RuleSetId::Arg, 33, "Draconic Aspect", &["PREFACT:1,TEMPLATES,IsKobold=true"]),
    (RuleSetId::Arg, 34, "Draconic Breath", &["PREABILITY:1,CATEGORY=FEAT,Draconic Aspect", "PREFACT:1,TEMPLATES,IsKobold=true"]),
    (RuleSetId::Arg, 35, "Draconic Glide", &["PREABILITY:1,CATEGORY=FEAT,Draconic Aspect", "PREFACT:1,TEMPLATES,IsKobold=true"]),
    (RuleSetId::Arg, 36, "Draconic Paragon", &["PREABILITY:2,CATEGORY=FEAT,Draconic Breath,Draconic Glide", "PREABILITY:1,CATEGORY=FEAT,Draconic Aspect", "PRELEVEL:MIN=10", "PREFACT:1,TEMPLATES,IsKobold=true"]),
    (RuleSetId::Arg, 37, "Drow Nobility", &["PREABILITY:1,CATEGORY=Special Ability,Drow ~ Spell-Like Abilities", "PREFACT:1,TEMPLATES,IsDrow=True"]),
    (RuleSetId::Arg, 38, "Drow ~ Spider Step", &["PREFACT:1,TEMPLATES,IsDrow=True", "PRELEVEL:MIN=3"]),
    (RuleSetId::Arg, 39, "Dwarf Blooded", &["PREFACT:1,TEMPLATES,IsOread=true"]),
    (RuleSetId::Arg, 40, "Echoes of Stone", &["PREFACT:1,TEMPLATES,IsOread=true"]),
    (RuleSetId::Arg, 41, "Elemental Jaunt", &["PRELEVEL:MIN=15", "PREFACT:1,TEMPLATES,IsIfrit=true,IsOread=true,IsSylph=true,IsUndine=true"]),
    (RuleSetId::Arg, 42, "Elven Spirit", &["!PREABILITY:1,CATEGORY=FEAT,Human Spirit", "PREPCLEVEL:MAX=1", "PREFACT:1,TEMPLATES,IsHalfElf=true"]),
    (RuleSetId::Arg, 43, "Exile's Path", &["PREFACT:1,TEMPLATES,IsHalfElf=true"]),
    (RuleSetId::Arg, 44, "Expanded Fiendish Resistance (Acid)", &["!PREABILITY:1,CATEGORY=Special Ability,Resistance to Acid", "PREFACT:1,TEMPLATES,IsTiefling=true"]),
    (RuleSetId::Arg, 45, "Expanded Fiendish Resistance (Cold)", &["!PREABILITY:1,CATEGORY=Special Ability,Resistance to Cold", "PREFACT:1,TEMPLATES,IsTiefling=true"]),
    (RuleSetId::Arg, 46, "Expanded Fiendish Resistance (Electricity)", &["!PREABILITY:1,CATEGORY=Special Ability,Resistance to Electricity", "PREFACT:1,TEMPLATES,IsTiefling=true"]),
    (RuleSetId::Arg, 47, "Expanded Fiendish Resistance (Fire)", &["!PREABILITY:1,CATEGORY=Special Ability,Resistance to Fire", "PREFACT:1,TEMPLATES,IsTiefling=true"]),
    (RuleSetId::Arg, 48, "Expanded Resistance", &["PREABILITY:1,CATEGORY=Special Ability,Gnome ~ Illusion Resistance", "PREFACT:1,TEMPLATES,IsGnome=true"]),
    (RuleSetId::Arg, 49, "Extra Elemental Assault", &["PREFACT:1,TEMPLATES,IsSuli=true"]),
    (RuleSetId::Arg, 50, "Fast Learner", &["PREFACT:1,TEMPLATES,IsHuman=true", "PRESTAT:1,INT=13"]),
    (RuleSetId::Arg, 51, "Fearless Curiosity", &["PREFACT:1,TEMPLATES,IsHuman=true", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Arg, 52, "Feline Grace", &["PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PREFACT:1,TEMPLATES,IsCatfolk=true"]),
    (RuleSetId::Arg, 53, "Ferocious Action", &["PREABILITY:1,CATEGORY=Special Ability,Orc ~ Ferocity", "PREFACT:1,TEMPLATES,IsOrc=true"]),
    (RuleSetId::Arg, 54, "Ferocious Resolve", &["PREABILITY:1,CATEGORY=Special Ability,Half-Orc ~ Orc Ferocity", "PREFACT:1,TEMPLATES,IsHalfOrc=true", "PRESTAT:1,CON=13"]),
    (RuleSetId::Arg, 55, "Ferocious Summons", &["PREABILITY:2,CATEGORY=FEAT,Augment Summoning,Spell Focus (Conjuration)", "PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true"]),
    (RuleSetId::Arg, 56, "Fiend Sight", &["PREFACT:1,TEMPLATES,IsTiefling=true", "PREVARLT:FiendSightTier,2", "PREVISION:1,Darkvision=60"]),
    (RuleSetId::Arg, 57, "Fire Tamer", &["PREFACT:1,TEMPLATES,IsGoblin=true"]),
    (RuleSetId::Arg, 58, "Firesight", &["PREFACT:1,TEMPLATES,IsIfrit=true"]),
    (RuleSetId::Arg, 59, "Flame Heart", &["PREABILITY:1,CATEGORY=FEAT,Fire Tamer", "PRELEVEL:MIN=5", "PREFACT:1,TEMPLATES,IsGoblin=true"]),
    (RuleSetId::Arg, 60, "Foment the Blood", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Energy", "PREFACT:1,TEMPLATES,IsOrc=true"]),
    (RuleSetId::Arg, 61, "Fortunate One", &["PREABILITY:1,CATEGORY=Special Ability,Halfling ~ Adaptable Luck", "PREFACT:1,TEMPLATES,IsHalfling=true"]),
    (RuleSetId::Arg, 62, "Giant Steps", &["PREABILITY:1,CATEGORY=Special Ability,Duergar ~ Slow and Steady", "PREFACT:1,TEMPLATES,IsDuergar=true"]),
    (RuleSetId::Arg, 63, "Gloom Sight", &["PREFACT:1,TEMPLATES,IsFetchling=true"]),
    (RuleSetId::Arg, 64, "Gore Fiend", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Rage", "PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true"]),
    (RuleSetId::Arg, 65, "Grasping Tail", &["PREFACT:1,TEMPLATES,IsTiefling=true"]),
    (RuleSetId::Arg, 66, "Greater Channel Force", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Energy", "PREABILITY:2,CATEGORY=FEAT,Channel Force,Improved Channel Force", "PREMULT:1,[PREVARGTEQ:OracleChannelDice,6],[PREVARGTEQ:ClericChannelPositiveEnergyDice,6],[PREVARGTEQ:PaladinChannelDice,6],[PREVARGTEQ:ShamanChannelDice,6]", "PREFACT:1,TEMPLATES,IsAasimar=true", "PRETEXT:channel energy 6d6"]),
    (RuleSetId::Arg, 67, "Greater Drow Nobility", &["PREABILITY:1,CATEGORY=Special Ability,Drow ~ Spell-Like Abilities", "PREABILITY:2,CATEGORY=FEAT,Drow Nobility,Improved Drow Nobility", "PREFACT:1,TEMPLATES,IsDrow=True", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Arg, 68, "Guardian of the Wild", &["PREABILITY:1,CATEGORY=FEAT,Attuned to the Wild", "PREFACT:1,TEMPLATES,IsElf=true"]),
    (RuleSetId::Arg, 69, "Half-Drow Paragon", &["PREABILITY:2,CATEGORY=Special Ability,Half-Elf ~ Drow Blooded,Half-Elf ~ Drow Magic", "PREFACT:1,TEMPLATES,IsHalfElf=true"]),
    (RuleSetId::Arg, 70, "Heavenly Radiance", &["PREABILITY:1,CATEGORY=Special Ability,Aasimar ~ Spell-Like Ability", "PREFACT:1,TEMPLATES,IsAasimar=true"]),
    (RuleSetId::Arg, 71, "Heroic Will", &["PREABILITY:1,CATEGORY=FEAT,Iron Will", "PRECHECKBASE:1,Will=4", "PREFACT:1,TEMPLATES,IsHuman=true"]),
    (RuleSetId::Arg, 72, "Hobgoblin Discipline", &["PREFACT:1,TEMPLATES,IsHobgoblin=true", "PRETOTALAB:1"]),
    (RuleSetId::Arg, 73, "Human Spirit", &["PREPCLEVEL:MAX=1", "PREFACT:1,TEMPLATES,IsHalfElf=true"]),
    (RuleSetId::Arg, 74, "Huntmaster", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Companion,TYPE.Special Mount", "!PREABILITY:1,CATEGORY=FEAT,Huntmaster", "PREFACT:1,TEMPLATES,IsHuman=true", "PRESKILL:1,Handle Animal=1"]),
    (RuleSetId::Arg, 75, "Hydraulic Maneuver", &["PREABILITY:1,CATEGORY=Special Ability,Undine ~ Spell-Like Ability", "PREFACT:1,TEMPLATES,IsUndine=true"]),
    (RuleSetId::Arg, 76, "Improved Channel Force", &["PREABILITY:1,CATEGORY=FEAT,Channel Force", "PREMULT:1,[PREVARGTEQ:OracleChannelDice,4],[PREVARGTEQ:ClericChannelPositiveEnergyDice,4],[PREVARGTEQ:PaladinChannelDice,4],[PREVARGTEQ:ShamanChannelDice,4]", "PREFACT:1,TEMPLATES,IsAasimar=true", "PRETEXT:channel energy 4d6"]),
    (RuleSetId::Arg, 77, "Improved Dark Sight", &["PREABILITY:2,CATEGORY=FEAT,Dark Sight,Gloom Sight", "PREFACT:1,TEMPLATES,IsFetchling=true"]),
    (RuleSetId::Arg, 78, "Improved Drow Nobility", &["PREABILITY:1,CATEGORY=Special Ability,Drow ~ Spell-Like Abilities", "PREABILITY:1,CATEGORY=FEAT,Drow Nobility", "PREFACT:1,TEMPLATES,IsDrow=True", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Arg, 79, "Improved Improvisation", &["PREABILITY:2,CATEGORY=FEAT,Fast Learner,Improvisation", "PREFACT:1,TEMPLATES,IsHuman=true", "PRESTAT:1,INT=13"]),
    (RuleSetId::Arg, 80, "Improved Umbral Scion", &["PREABILITY:1,CATEGORY=Special Ability,Drow ~ Spell-Like Abilities", "PREABILITY:4,CATEGORY=FEAT,Drow Nobility,Greater Drow Nobility,Improved Drow Nobility,Umbral Scion", "PREFACT:1,TEMPLATES,IsDrow=True", "PRESTAT:2,CHA=13,WIS=13"]),
    (RuleSetId::Arg, 81, "Improvisation", &["PREABILITY:1,CATEGORY=FEAT,Fast Learner", "PREFACT:1,TEMPLATES,IsHuman=true", "PRESTAT:1,INT=13"]),
    (RuleSetId::Arg, 82, "Incremental Elemental Assault", &["PREFACT:1,TEMPLATES,IsSuli=true"]),
    (RuleSetId::Arg, 83, "Inexplicable Luck", &["PREABILITY:1,CATEGORY=FEAT,Defiant Luck", "PREFACT:1,TEMPLATES,IsHuman=true"]),
    (RuleSetId::Arg, 84, "Inner Breath", &["PRELEVEL:MIN=11", "PREFACT:1,TEMPLATES,IsSylph=true"]),
    (RuleSetId::Arg, 85, "Intimidating Confidence", &["PREABILITY:1,CATEGORY=FEAT,Fearless Curiosity", "PREFACT:1,TEMPLATES,IsHuman=true", "PRESKILL:1,Intimidate=5", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Arg, 86, "Ledge Walker", &["PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Mountaineer,Dwarf ~ Stability", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PREFACT:1,TEMPLATES,IsDwarf=true"]),
    (RuleSetId::Arg, 87, "Life's Blood", &["PREFACT:1,TEMPLATES,IsSamsaran=true"]),
    (RuleSetId::Arg, 88, "Lingering Invisibility", &["PREFACT:1,TEMPLATES,IsDuergar=true"]),
    (RuleSetId::Arg, 89, "Long-Nose Form", &["PRELEVEL:MIN=3", "PREFACT:1,TEMPLATES,IsTengu=true"]),
    (RuleSetId::Arg, 90, "Lucky Healer", &["PREABILITY:1,CATEGORY=Special Ability,Halfling ~ Adaptable Luck", "PREFACT:1,TEMPLATES,IsHalfling=true"]),
    (RuleSetId::Arg, 91, "Mage of the Wild", &["PREABILITY:1,CATEGORY=FEAT,Attuned to the Wild", "PREFACT:1,TEMPLATES,IsElf=true"]),
    (RuleSetId::Arg, 92, "Magical Tail", &["PREFACT:1,TEMPLATES,IsKitsune=true", "PREVARLT:KitsuneTails,8"]),
    (RuleSetId::Arg, 93, "Metallic Wings", &["PREABILITY:3,CATEGORY=FEAT,Angel Wings,Angelic Blood,Angelic Flesh", "PRELEVEL:MIN=11", "PREFACT:1,TEMPLATES,IsAasimar=true"]),
    (RuleSetId::Arg, 94, "Mother's Gift", &["PREFACT:1,TEMPLATES,IsChangeling=true"]),
    (RuleSetId::Arg, 95, "Multitalented Mastery", &["PREABILITY:1,CATEGORY=Special Ability,Half-Elf ~ Multitalented", "PRELEVEL:MIN=5", "PREFACT:1,TEMPLATES,IsHalfElf=true"]),
    (RuleSetId::Arg, 96, "Murmurs of Earth", &["PREABILITY:1,CATEGORY=FEAT,Echoes of Stone", "PRELEVEL:MIN=9", "PREFACT:1,TEMPLATES,IsOread=true"]),
    (RuleSetId::Arg, 97, "Natural Charmer", &["PREFACT:1,TEMPLATES,IsDhampir=true", "PRESTAT:1,CHA=17"]),
    (RuleSetId::Arg, 98, "Neither Elf nor Human", &["PREABILITY:2,CATEGORY=FEAT,Exile's Path,Seen and Unseen", "PRELEVEL:MIN=11", "PREFACT:1,TEMPLATES,IsHalfElf=true"]),
    (RuleSetId::Arg, 99, "Noble Spell Resistance", &["PREABILITY:1,CATEGORY=FEAT,Greater Drow Nobility", "PREFACT:1,TEMPLATES,IsDrow=True", "PRELEVEL:MIN=13", "PRESTAT:2,CHA=13,WIS=13"]),
    (RuleSetId::Arg, 100, "Oread Burrower", &["PREABILITY:1,CATEGORY=FEAT,Stony Step", "PRELEVEL:MIN=9", "PREFACT:1,TEMPLATES,IsOread=true"]),
    (RuleSetId::Arg, 101, "Oread Earth Glider", &["PREABILITY:2,CATEGORY=FEAT,Oread Burrower,Stony Step", "PRELEVEL:MIN=13", "PREFACT:1,TEMPLATES,IsOread=true"]),
    (RuleSetId::Arg, 102, "Realistic Likeness", &["PREFACT:1,TEMPLATES,IsKitsune=true"]),
    (RuleSetId::Arg, 103, "Resilient Brute", &["PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true"]),
    (RuleSetId::Arg, 104, "Resolute Rager", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Rage", "PREFACT:1,TEMPLATES,IsOrc=true"]),
    (RuleSetId::Arg, 105, "Scavenger's Eye", &["PREFACT:1,TEMPLATES,IsTengu=true"]),
    (RuleSetId::Arg, 106, "Seen and Unseen", &["PREABILITY:1,CATEGORY=FEAT,Exile's Path", "PRELEVEL:MIN=5", "PREFACT:1,TEMPLATES,IsHalfElf=true"]),
    (RuleSetId::Arg, 107, "Shadow Caster", &["PREFACT:1,TEMPLATES,IsDrow=True", "PREMULT:1,[PRECLASS:1,SPELLCASTER=1],[PREVARGTEQ:CasterLevel_Highest,1]"]),
    (RuleSetId::Arg, 108, "Shadow Ghost", &["PREABILITY:1,CATEGORY=Special Ability,Fetchling ~ Spell-Like Abilities", "PREPCLEVEL:MIN=9", "PREFACT:1,TEMPLATES,IsFetchling=true"]),
    (RuleSetId::Arg, 109, "Shadow Walker", &["PREABILITY:1,CATEGORY=Special Ability,Fetchling ~ Spell-Like Abilities", "PREPCLEVEL:MIN=9", "PREFACT:1,TEMPLATES,IsFetchling=true"]),
    (RuleSetId::Arg, 110, "Shadowy Dash", &["PREFACT:1,TEMPLATES,IsWayang=true"]),
    (RuleSetId::Arg, 111, "Shared Manipulation", &["PREFACT:1,TEMPLATES,IsHalfElf=true", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Arg, 112, "Sleep Venom", &["PREFACT:1,TEMPLATES,IsVishkanya=true"]),
    (RuleSetId::Arg, 113, "Spider Summoner", &["PREFACT:1,TEMPLATES,IsDrow=True", "PRESPELL:1,Summon Monster I,Summon Monster II,Summon Monster III,Summon Monster IV,Summon Monster V,Summon Monster VI,Summon Monster VII,Summon Monster VIII,Summon Monster IX,Summon Nature's Ally I,Summon Nature's Ally II,Summon Nature's Ally III,Summon Nature's Ally IV,Summon Nature's Ally V,Summon Nature's Ally VI,Summon Nature's Ally VII,Summon Nature's Ally VIII,Summon Nature's Ally IX"]),
    (RuleSetId::Arg, 114, "Spirit of the Wild", &["PREABILITY:2,CATEGORY=FEAT,Attuned to the Wild,Guardian of the Wild", "PREFACT:1,TEMPLATES,IsElf=true"]),
    (RuleSetId::Arg, 115, "Steam Caster", &["PREFACT:1,TEMPLATES,IsUndine=true"]),
    (RuleSetId::Arg, 116, "Stoic Pose", &["PREFACT:1,TEMPLATES,IsSvirfneblin=true"]),
    (RuleSetId::Arg, 117, "Stony Step", &["PREFACT:1,TEMPLATES,IsOread=true"]),
    (RuleSetId::Arg, 118, "Stretched Wings", &["PREABILITY:1,CATEGORY=Special Ability,Strix ~ Wing-Clipped", "PREABILITY:1,CATEGORY=FEAT,Skill Focus (Fly)", "PREFACT:1,TEMPLATES,IsStrix=true", "PRESTAT:1,STR=13"]),
    (RuleSetId::Arg, 119, "Sure and Fleet", &["PREABILITY:1,CATEGORY=Special Ability,Halfling ~ Fleet Of Foot", "PREFACT:1,TEMPLATES,IsHalfling=true"]),
    (RuleSetId::Arg, 120, "Surge of Success", &["PREFACT:1,TEMPLATES,IsHuman=true"]),
    (RuleSetId::Arg, 121, "Tenacious Survivor", &["PREABILITY:2,CATEGORY=FEAT,Diehard,Endurance", "PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true", "PRESTAT:1,CON=13"]),
    (RuleSetId::Arg, 122, "Tengu Raven Form", &["PREABILITY:1,CATEGORY=FEAT,Tengu Wings", "PRELEVEL:MIN=7", "PREFACT:1,TEMPLATES,IsTengu=true"]),
    (RuleSetId::Arg, 123, "Tengu Wings", &["PRELEVEL:MIN=5", "PREFACT:1,TEMPLATES,IsTengu=true"]),
    (RuleSetId::Arg, 124, "Thrill of the Kill", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Rage", "PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true"]),
    (RuleSetId::Arg, 125, "Toxic Recovery", &["PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Hardy", "PREFACT:1,TEMPLATES,IsDwarf=true"]),
    (RuleSetId::Arg, 126, "Trap Wrecker", &["PREABILITY:1,CATEGORY=FEAT,Power Attack", "PREFACT:1,TEMPLATES,IsOrc=true", "PRESKILL:1,Disable Device=1"]),
    (RuleSetId::Arg, 127, "Triton Portal", &["PREABILITY:1,CATEGORY=Special Ability,Undine ~ Spell-Like Ability", "PRELEVEL:MIN=5", "PREFACT:1,TEMPLATES,IsUndine=true"]),
    (RuleSetId::Arg, 128, "Tunnel Rat", &["PREABILITY:1,CATEGORY=Special Ability,Ratfolk ~ Swarming", "PREFACT:1,TEMPLATES,IsRatfolk=true"]),
    (RuleSetId::Arg, 129, "Umbral Scion", &["PREABILITY:1,CATEGORY=Special Ability,Drow ~ Spell-Like Abilities", "PREABILITY:3,CATEGORY=FEAT,Drow Nobility,Greater Drow Nobility,Improved Drow Nobility", "PREFACT:1,TEMPLATES,IsDrow=True", "PRESTAT:2,CHA=13,WIS=13"]),
    (RuleSetId::Arg, 130, "Water Skinned", &["PREFACT:1,TEMPLATES,IsUndine=true"]),
    (RuleSetId::Arg, 131, "Wings of Air", &["PREABILITY:1,CATEGORY=FEAT,Airy Step", "PRELEVEL:MIN=9", "PREFACT:1,TEMPLATES,IsSylph=true"]),
    (RuleSetId::Arg, 132, "Blazing Aura", &["PREABILITY:2,CATEGORY=FEAT,Inner Flame,Scorching Weapons", "PRELEVEL:MIN=13", "PREFACT:1,TEMPLATES,IsIfrit=true"]),
    (RuleSetId::Arg, 133, "Blistering Feint", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Feint", "PREFACT:1,TEMPLATES,IsIfrit=true"]),
    (RuleSetId::Arg, 134, "Blood Beak", &["PREABILITY:1,CATEGORY=Special Ability,Tengu ~ Natural Weapon", "PREFACT:1,TEMPLATES,IsTengu=true", "PRETOTALAB:5"]),
    (RuleSetId::Arg, 135, "Blundering Defense", &["PREABILITY:1,CATEGORY=FEAT,Cautious Fighter", "PREFACT:1,TEMPLATES,IsHalfling=true"]),
    (RuleSetId::Arg, 136, "Bullying Blow", &["PREFACT:1,TEMPLATES,IsOrc=true", "PRESKILL:1,Intimidate=1"]),
    (RuleSetId::Arg, 137, "Cautious Fighter", &["PREFACT:1,TEMPLATES,IsHalfling=true"]),
    (RuleSetId::Arg, 138, "Claw Pounce", &["PREABILITY:1,CATEGORY=FEAT,Nimble Striker", "PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Catfolk ~ Cat's Claws],[PREABILITY:1,CATEGORY=FEAT,Aspect of the Beast (Claws of the Beast)]", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PREFACT:1,TEMPLATES,IsCatfolk=true", "PRESTAT:1,STR=13", "PRETOTALAB:10"]),
    (RuleSetId::Arg, 139, "Cleave Through", &["PREABILITY:2,CATEGORY=FEAT,Power Attack,Cleave", "PREFACT:1,TEMPLATES,IsDwarf=true", "PRESTAT:1,STR=13", "PRETOTALAB:11"]),
    (RuleSetId::Arg, 140, "Cloven Helm", &["PREABILITY:2,CATEGORY=FEAT,Dented Helm,Hard Headed", "PREFACT:1,TEMPLATES,IsDwarf=true", "PRETOTALAB:11"]),
    (RuleSetId::Arg, 141, "Critical Versatility", &["PREFACT:1,TEMPLATES,IsHuman=true", "PREVARGTEQ:FighterWeaponQualifyLVL,11"]),
    (RuleSetId::Arg, 142, "Demoralizing Lash", &["PREFACT:1,TEMPLATES,IsHobgoblin=true", "PRESKILL:1,Intimidate=1", "PRETOTALAB:1"]),
    (RuleSetId::Arg, 143, "Dented Helm", &["PREABILITY:1,CATEGORY=FEAT,Hard Headed", "PREFACT:1,TEMPLATES,IsDwarf=true", "PRETOTALAB:6"]),
    (RuleSetId::Arg, 144, "Desperate Swing", &["PREABILITY:1,CATEGORY=FEAT,Cautious Fighter", "PREFACT:1,TEMPLATES,IsHalfling=true", "PRETOTALAB:1"]),
    (RuleSetId::Arg, 145, "Destroyer's Blessing", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Rage", "PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true"]),
    (RuleSetId::Arg, 146, "Elven Battle Training", &["PREFACT:1,TEMPLATES,IsElf=true", "PRETOTALAB:1"]),
    (RuleSetId::Arg, 147, "Ferocious Tenacity", &["PREABILITY:2,CATEGORY=Special Ability,Half-Orc ~ Orc Ferocity,TYPE.Rage", "PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true"]),
    (RuleSetId::Arg, 148, "Fire Hand", &["PREFACT:1,TEMPLATES,IsGoblin=true"]),
    (RuleSetId::Arg, 149, "Giant Killer", &["PREABILITY:5,CATEGORY=FEAT,Cleave,Goblin Cleaver,Orc Hewer,Power Attack,Strike Back", "PREFACT:1,TEMPLATES,IsDwarf=true", "PRESTAT:1,STR=13", "PRETOTALAB:11"]),
    (RuleSetId::Arg, 150, "Gloom Strike", &["PREABILITY:1,CATEGORY=FEAT,Blind-Fight", "PREFACT:1,TEMPLATES,IsFetchling=true"]),
    (RuleSetId::Arg, 151, "Gnome Weapon Focus", &["PREFACT:1,TEMPLATES,IsGnome=true", "PRETOTALAB:1", "PREWEAPONPROF:1,TYPE.Martial"]),
    (RuleSetId::Arg, 152, "Goblin Cleaver", &["PREABILITY:2,CATEGORY=FEAT,Cleave,Power Attack", "PREFACT:1,TEMPLATES,IsDwarf=true", "PRESTAT:1,STR=13"]),
    (RuleSetId::Arg, 153, "Goblin Gunslinger", &["PREFACT:1,TEMPLATES,IsGoblin=true"]),
    (RuleSetId::Arg, 154, "Great Hatred", &["PREABILITY:1,CATEGORY=Special Ability,Gnome ~ Hatred", "PREFACT:1,TEMPLATES,IsGnome=true"]),
    (RuleSetId::Arg, 155, "Grudge Fighter", &["PREFACT:1,TEMPLATES,IsOrc=true"]),
    (RuleSetId::Arg, 156, "Hard-Headed", &["PREFACT:1,TEMPLATES,IsDwarf=true", "PRETOTALAB:1"]),
    (RuleSetId::Arg, 157, "Improved Low Blow", &["PREABILITY:1,CATEGORY=Special Ability,Halfling ~ Low-Blow", "PREFACT:1,TEMPLATES,IsHalfling=true", "PRETOTALAB:4"]),
    (RuleSetId::Arg, 158, "Improved Surprise Follow-Through", &["PREABILITY:4,CATEGORY=FEAT,Cleave,Great Cleave,Power Attack,Surprise Follow-Through", "PRESTAT:1,STR=13", "PRETOTALAB:8"]),
    (RuleSetId::Arg, 159, "Inner Flame", &["PREABILITY:1,CATEGORY=FEAT,Scorching Weapons", "PRELEVEL:MIN=7", "PREFACT:1,TEMPLATES,IsIfrit=true"]),
    (RuleSetId::Arg, 160, "Kobold Ambusher", &["PREFACT:1,TEMPLATES,IsKobold=true", "PRESKILL:1,Stealth=4"]),
    (RuleSetId::Arg, 161, "Kobold Sniper", &["PREFACT:1,TEMPLATES,IsKobold=true", "PRESKILL:1,Stealth=1"]),
    (RuleSetId::Arg, 162, "Lucky Strike", &["PREABILITY:1,CATEGORY=Special Ability,Halfling ~ Adaptable Luck", "PREFACT:1,TEMPLATES,IsHalfling=true", "PRETOTALAB:5"]),
    (RuleSetId::Arg, 163, "Martial Mastery", &["PREABILITY:1,CATEGORY=FEAT,Martial Versatility", "PREFACT:1,TEMPLATES,IsHuman=true", "PREVARGTEQ:FighterWeaponQualifyLVL,16"]),
    (RuleSetId::Arg, 164, "Martial Versatility", &["PREFACT:1,TEMPLATES,IsHuman=true", "PREVARGTEQ:FighterWeaponQualifyLVL,4"]),
    (RuleSetId::Arg, 165, "Nimble Striker", &["PREABILITY:1,CATEGORY=Special Ability,Catfolk ~ Sprinter", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PREFACT:1,TEMPLATES,IsCatfolk=true", "PRETOTALAB:1"]),
    (RuleSetId::Arg, 166, "Orc Hewer", &["PREABILITY:3,CATEGORY=FEAT,Cleave,Goblin Cleaver,Power Attack", "PREFACT:1,TEMPLATES,IsDwarf=true", "PRESTAT:1,STR=13"]),
    (RuleSetId::Arg, 167, "Orc Weapon Expertise", &["PREFACT:1,TEMPLATES,IsOrc=true", "PRETOTALAB:1"]),
    (RuleSetId::Arg, 168, "Reverse-Feint", &["PREABILITY:1,CATEGORY=FEAT,Toughness", "PREFACT:1,TEMPLATES,IsOrc=true", "PRETOTALAB:1"]),
    (RuleSetId::Arg, 169, "Risky Striker", &["PREFACT:1,TEMPLATES,IsHalfling=true", "PRETOTALAB:1"]),
    (RuleSetId::Arg, 170, "Scorching Weapons", &["PREFACT:1,TEMPLATES,IsIfrit=true"]),
    (RuleSetId::Arg, 171, "Sea Hunter", &["PREABILITY:1,CATEGORY=FEAT,Combat Expertise", "PREFACT:1,TEMPLATES,IsMerfolk=true"]),
    (RuleSetId::Arg, 172, "Sharpclaw", &["PREFACT:1,TEMPLATES,IsRatfolk=true"]),
    (RuleSetId::Arg, 173, "Shatterspell", &["PREABILITY:2,CATEGORY=FEAT,Disruptive,Spellbreaker", "PREFACT:1,TEMPLATES,IsDwarf=true", "PREVARGTEQ:FighterWeaponQualifyLVL,10"]),
    (RuleSetId::Arg, 174, "Spit Venom", &["PREFACT:1,TEMPLATES,IsNagaji=true"]),
    (RuleSetId::Arg, 175, "Surprise Follow-Through", &["PREABILITY:2,CATEGORY=FEAT,Cleave,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:1"]),
    (RuleSetId::Arg, 176, "Surprise Strike", &["PREABILITY:2,CATEGORY=FEAT,Cautious Fighter,Desperate Swing", "PREFACT:1,TEMPLATES,IsHalfling=true", "PRETOTALAB:6"]),
    (RuleSetId::Arg, 177, "Sympathetic Rage", &["!PREALIGN:LG,LN,LE", "PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true"]),
    (RuleSetId::Arg, 178, "Tail Terror", &["PREFACT:1,TEMPLATES,IsKobold=true", "PRETOTALAB:1"]),
    (RuleSetId::Arg, 179, "Tangle Feet", &["PREABILITY:3,CATEGORY=FEAT,Dodge,Mobility,Underfoot", "PREFACT:1,TEMPLATES,IsGoblin=true", "PRESIZELTEQ:S"]),
    (RuleSetId::Arg, 180, "Taskmaster", &["PREABILITY:1,CATEGORY=FEAT,Demoralizing Lash", "PREFACT:1,TEMPLATES,IsHobgoblin=true", "PRESKILL:1,Intimidate=5"]),
    (RuleSetId::Arg, 181, "Tree Hanger", &["PREFACT:1,TEMPLATES,IsVanara=true", "PRESKILL:1,Acrobatics=1"]),
    (RuleSetId::Arg, 182, "Uncanny Defense", &["PREABILITY:1,CATEGORY=FEAT,Cautious Fighter", "PREFACT:1,TEMPLATES,IsHalfling=true", "PRETOTALAB:3"]),
    (RuleSetId::Arg, 183, "Vast Hatred", &["PREABILITY:1,CATEGORY=Special Ability,Gnome ~ Hatred", "PREFACT:1,TEMPLATES,IsGnome=true"]),
    (RuleSetId::Arg, 184, "Focusing Blow", &["PREABILITY:1,CATEGORY=FEAT,Hobgoblin Discipline", "PREFACT:1,TEMPLATES,IsHobgoblin=true"]),
    (RuleSetId::Arg, 185, "Greater Brand", &["PREABILITY:1,CATEGORY=Archetype,Inquisitor Archetype ~ Kinslayer", "PREFACT:1,TEMPLATES,IsDhampir=true"]),
    (RuleSetId::Arg, 186, "Horde Charge", &["PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true", "PRETOTALAB:1"]),
    // Pathfinder Unchained — 14 of 17 record(s) in the hand-authored table carry at least one `PRE`-family token.
    (RuleSetId::Pu, 0, "Champion of Anarchy", &["PREABILITY:1,CATEGORY=FEAT,TYPE.Alignment,Champion of Anarchy", "PREALIGN:CN", "PREHD:MIN=10", "PRETEXT:10 Hit Dice, chaotic neutral alignment."]),
    (RuleSetId::Pu, 1, "Champion of Balance", &["PREABILITY:1,CATEGORY=FEAT,TYPE.Alignment,Champion of Balance", "PREALIGN:TN", "PREHD:MIN=10", "PRETEXT:10 Hit Dice, neutral alignment."]),
    (RuleSetId::Pu, 2, "Champion of Destruction", &["PREABILITY:1,CATEGORY=FEAT,TYPE.Alignment,Champion of Destruction", "PREALIGN:CE", "PREHD:MIN=10", "PRETEXT:10 Hit Dice, chaotic evil alignment."]),
    (RuleSetId::Pu, 3, "Champion of Freedom", &["PREABILITY:1,CATEGORY=FEAT,TYPE.Alignment,Champion of Freedom", "PREALIGN:CG", "PREHD:MIN=10", "PRETEXT:10 Hit Dice, chaotic good alignment."]),
    (RuleSetId::Pu, 4, "Champion of Grace", &["PREABILITY:1,CATEGORY=FEAT,TYPE.Alignment,Champion of Grace", "PREALIGN:NG", "PREHD:MIN=10", "PRETEXT:10 Hit Dice, neutral good alignment."]),
    (RuleSetId::Pu, 5, "Champion of Malevolence", &["PREABILITY:1,CATEGORY=FEAT,TYPE.Alignment,Champion of Malevolence", "PREALIGN:NE", "PREHD:MIN=10", "PRETEXT:10 Hit Dice, neutral evil alignment."]),
    (RuleSetId::Pu, 6, "Champion of Righteousness", &["PREABILITY:1,CATEGORY=FEAT,TYPE.Alignment,Champion of Righteousness", "PREALIGN:LG", "PREHD:MIN=10", "PRETEXT:10 Hit Dice, lawful good alignment."]),
    (RuleSetId::Pu, 7, "Champion of Tranquility", &["PREABILITY:1,CATEGORY=FEAT,TYPE.Alignment,Champion of Tranquility", "PREALIGN:LN", "PREHD:MIN=10", "PRETEXT:10 Hit Dice, lawful neutral alignment."]),
    (RuleSetId::Pu, 8, "Champion of Tyranny", &["PREABILITY:1,CATEGORY=FEAT,TYPE.Alignment,Champion of Tyranny", "PREALIGN:LE", "PREHD:MIN=10", "PRETEXT:10 Hit Dice, lawful evil alignment."]),
    (RuleSetId::Pu, 9, "Combat Stamina", &["PRETEXT:Prerequisite: Base attack bonus +1.", "PRETOTALAB:1"]),
    (RuleSetId::Pu, 10, "Extra Stamina", &["PREABILITY:1,CATEGORY=FEAT,Combat Stamina", "!PREABILITY:3,CATEGORY=FEAT,Extra Stamina", "PRETEXT:Prerequisites: Combat Stamina, base attack bonus +5.", "PRETOTALAB:5"]),
    (RuleSetId::Pu, 11, "Push the Limits", &["PREABILITY:1,CATEGORY=FEAT,Combat Stamina", "PRESTAT:1,CON=13", "PRETEXT:Prerequisites: Con 13, Combat Stamina, base attack bonus +1.", "PRETOTALAB:1"]),
    (RuleSetId::Pu, 15, "Extra Unchained Rogue Talent", &["PREABILITY:1,CATEGORY=CLASS,Rogue ~ Unchained Class", "PREVARGTEQ:RogueTalentLVL,1"]),
    (RuleSetId::Pu, 16, "Signature Skill", &["PRESKILL:1,TYPE.Base=5", "PRETEXT:Prerequisite: 5 ranks in the chosen skill.", "PREVAREQ:CannotUseSignatureSkill,0"]),
    // Ultimate Campaign — 23 of 23 record(s) in the hand-authored table carry at least one `PRE`-family token.
    (RuleSetId::Uca, 0, "Accursed", &["PRETEXT:Prerequisite:You must carry a curse that can be lifted only by a quest or similar great undertaking, or have the Cursed Birth background."]),
    (RuleSetId::Uca, 1, "Arisen", &["PRETEXT:Prerequisite:You must have been slain and brought back from the dead, or have the Left to Die or Cursed Birth background."]),
    (RuleSetId::Uca, 2, "Battlefield Healer", &["PRETEXT:Prerequisite:You must successfully cast a conjuration (healing) spell on an ally after being hit by an attack of opportunity, or have the Battle, Chaplain, or Healed background."]),
    (RuleSetId::Uca, 3, "Champion", &["PRETEXT:Prerequisite:You must have defeated a single challenging foe without any aid from another, or have the Champion of a God, Champion of the People, Competition Champion, or Gladiator background."]),
    (RuleSetId::Uca, 4, "Damned", &["PRETEXT:Prerequisite:You must have had friendly contact with an evil-aligned outsider that would qualify as a challenging foe, have a fiend-related sorcerous bloodline such as abyssal or infernal, have direct fiendish ancestry (such as being a tiefling or half-fiend), or have the Fiend Raised or The Fiend background."]),
    (RuleSetId::Uca, 5, "Deny the Reaper", &["PRETEXT:Prerequisite:You must have witnessed the death of a close companion in battle-a death that could have been prevented, such as from bleeding, failure to stabilize, or ongoing poison damage-or have the Death in the Family or The War background."]),
    (RuleSetId::Uca, 6, "Eldritch Researcher", &["PRETEXT:Prerequisite:You must have created a new spell, or have The Way Things Work background."]),
    (RuleSetId::Uca, 7, "Fearless Zeal", &["PRETEXT:Prerequisite:You must be ordained as a sacred (or profane) champion of your faith by a high-ranking member of its clergy, or have the Devoted, Faith-Bringer, or Moral Debt background. Such an honor goes above and beyond the normal oaths required of a cleric or paladin."]),
    (RuleSetId::Uca, 8, "Feral Heart", &["PRETEXT:Prerequisite:You must have reverted to savage behavior through a traumatic event or extended period in the wilderness, or have the Raised by Beasts background."]),
    (RuleSetId::Uca, 9, "Foeslayer", &["PRETEXT:Prerequisite:You must have been defeated and robbed of at least half your possessions by a particular group of humanoids or monstrous humanoids, or have the An Eye for an Eye, Hated Foe, Raiders, or Vengeance background. You may choose a specific race, such as duergar, or a broader group, such as goblinoids. At the GM's option, you may instead choose residents of a particular country, settlement, or tribe."]),
    (RuleSetId::Uca, 10, "Forgotten Past", &["PRETEXT:Prerequisite:You must have suffered permanent memory loss or have the Reincarnated background."]),
    (RuleSetId::Uca, 11, "Glimpse Beyond", &["PRETEXT:Prerequisite:You must have faced an undead, evil outsider, or aberration with a CR greater than your level +4, or have the Raised Among the Dead or The Dead One background."]),
    (RuleSetId::Uca, 12, "Innocent Blood", &["PRETEXT:Prerequisite:You must slay at least 50 intelligent noncombatants for either your own personal gain or for no cause at all, or have the Bloodthirsty, First Kill, or The Kill background."]),
    (RuleSetId::Uca, 13, "Liberator", &["PRETEXT:Prerequisite:You must have been enslaved for at least 6 months, or have the Imprisoned or Kidnapped background."]),
    (RuleSetId::Uca, 14, "Lost Legacy", &["PRETEXT:Prerequisite:Your family must have claim to an inherited title or position that no longer belongs to them, or have the Dishonored Family background. You can take this feat even if you have no knowledge of this lost family title."]),
    (RuleSetId::Uca, 15, "Magnum Opus", &["PRETEXT:Prerequisite:You must either have sold five or more self-created works of art worth a total of at least 5,000 gp, have performed at least five performances for audiences of 50 or more while achieving a great performance result or better on your Perform check, or have the Virtuoso background."]),
    (RuleSetId::Uca, 16, "Shamed", &["PRETEXT:Prerequisite:You must have been publicly embarrassed, or must have the Bastard Born background. If the embarrassment didn't cause significant harm to your personal honor or social standing, it does not qualify for the feat prerequisites. The humiliation doesn't need to have been unjustified."]),
    (RuleSetId::Uca, 17, "Stronghold", &["PRETEXT:Prerequisites:You must have the Leadership feat and must lead at least 10 combat-capable followers (such as fighters or rangers)."]),
    (RuleSetId::Uca, 18, "Thief of Legend", &["PRETEXT:Prerequisites:You must have stolen at least 1,000 gp worth of treasure without being caught and kept mementos of these thefts worth at least 500 gp, or have the Greed background."]),
    (RuleSetId::Uca, 19, "Town Tamer", &["PRETEXT:Prerequisites:You must have 5 ranks in Intimidate and a personal motivation to clean up a particular town (such as an old friend calling in a favor, or seeking a place to settle down), or you must have the Bounty Hunter or Champion of the People background."]),
    (RuleSetId::Uca, 20, "True Love", &["PRETEXT:Prerequisite:You must have found love with a person you can't be with, have a current lover, or have the Current Lover, For Love, or The Lover background. Possible complications include distance, your love being with another, your feelings being unrequited, or your relationship being forbidden."]),
    (RuleSetId::Uca, 21, "Unforgotten", &["PRETEXT:Prerequisite:You must have a close relative, spouse, or other person dear to your heart who never returned from a journey, was captured, or otherwise vanished with little trace, or you have the Major Disaster background."]),
    (RuleSetId::Uca, 22, "Vengeance", &["PRETEXT:Prerequisite:You must have a close family member or other loved one slain by a specific challenging foe or that foe's minions, or have the Raiders or Vengeance background."]),
    // Ultimate Intrigue — 98 of 104 record(s) in the hand-authored table carry at least one `PRE`-family token.
    (RuleSetId::Ui, 0, "Acrobatic Spellcaster", &["PREABILITY:2,CATEGORY=FEAT,Combat Casting,Skill Focus (Acrobatics)"]),
    (RuleSetId::Ui, 1, "Agent of Fear", &["PREABILITY:1,CATEGORY=FEAT,Persuasive", "PREABILITY:1,CATEGORY=Special Ability,Vigilante ~ Frightening Appearance"]),
    (RuleSetId::Ui, 2, "Betrayal Sense", &["PREABILITY:1,CATEGORY=Special Ability,Trap Sense", "PRECLASS:1,Rogue=3", "PREVARGTEQ:PreStatScore_WIS,13"]),
    (RuleSetId::Ui, 3, "Blustering Bluff", &["PRESKILL:2,Bluff=1,Intimidate=1", "PREVARGTEQ:PreStatScore_CHA,13"]),
    (RuleSetId::Ui, 4, "Brilliant Planner", &["PRELEVEL:MIN=5", "PREVARGTEQ:PreStatScore_INT,13"]),
    (RuleSetId::Ui, 5, "Brilliant Spell Preparation", &["PRESPELLCAST:MEMORIZE=Y", "PRESPELLTYPE:1,ANY=3", "PREVARGTEQ:PreStatScore_INT,13"]),
    (RuleSetId::Ui, 6, "But a Scratch", &["PRESKILL:1,Bluff=4", "PREVARGTEQ:PreStatScore_CHA,13"]),
    (RuleSetId::Ui, 7, "Call Truce", &["PREABILITY:1,CATEGORY=FEAT,Persuasive", "PRESKILL:1,Diplomacy=5", "PREVARGTEQ:PreStatScore_CHA,15"]),
    (RuleSetId::Ui, 8, "Careful Flyer", &["PREABILITY:1,CATEGORY=FEAT,Acrobatic", "PRESKILL:1,Fly=5"]),
    (RuleSetId::Ui, 9, "Careful Sneak", &["PRESKILL:1,Stealth=3", "PREVARGTEQ:PreStatScore_DEX,13"]),
    (RuleSetId::Ui, 10, "Cartogramancer", &["PRESKILL:1,Knowledge (geography)=10", "PRESPELL:1,Teleport (Greater)"]),
    (RuleSetId::Ui, 11, "Cat and Mouse", &["PREABILITY:1,CATEGORY=Special Ability,Swashbuckler ~ Opportune Parry and Riposte", "PRESKILL:2,Bluff=5,Sense Motive=5"]),
    (RuleSetId::Ui, 12, "Cat's Fall", &["PRESKILL:1,Acrobatics=4", "PREVARGTEQ:PreStatScore_DEX,13"]),
    (RuleSetId::Ui, 13, "Circuitous Shot", &["PREABILITY:4,CATEGORY=FEAT,Blind-Fight,Improved Precise Shot,Point-Blank Shot,Precise Shot", "PRETOTALAB:11", "PREVARGTEQ:PreStatScore_DEX,19"]),
    (RuleSetId::Ui, 14, "City Sprinter", &["PREABILITY:1,CATEGORY=FEAT,Street Smarts"]),
    (RuleSetId::Ui, 15, "Clambering Escape", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Reposition", "PREABILITY:1,CATEGORY=Special Ability,Evasion", "PREVARGTEQ:PreStatScore_INT,13"]),
    (RuleSetId::Ui, 16, "Conceal Spell", &["PREABILITY:1,CATEGORY=FEAT,Deceitful", "PRESKILL:3,Bluff=1,Disguise=1,Sleight of Hand=1"]),
    (RuleSetId::Ui, 17, "Confabulist", &["PRESKILL:2,Bluff=9,Sense Motive=9"]),
    (RuleSetId::Ui, 18, "Cooperative Disabling", &["PREMULT:1,[PREVARGTEQ:TrapfindingLVL,1],[PREABILITY:1,CATEGORY=Special Ability,TYPE=.Trapfinding]", "PRESKILL:1,Disable Device=1"]),
    (RuleSetId::Ui, 19, "Criminal Reputation", &["PRESKILL:2,Diplomacy=5,Intimidate=5"]),
    (RuleSetId::Ui, 20, "Cunning Intuition", &["PREABILITY:5,CATEGORY=FEAT,Alertness,Improved Initiative,Lightning Reflexes,Quick Draw,Ready for Anything", "PREMULT:1,[PRETOTALAB:13],[PRECLASS:1,Rogue=13]", "PRESKILL:1,Sense Motive=13"]),
    (RuleSetId::Ui, 21, "Cutting Humiliation", &["PREABILITY:1,CATEGORY=FEAT,Persuasive", "PRESKILL:1,Intimidate=5"]),
    (RuleSetId::Ui, 22, "Darkness Trick", &["PRESKILL:1,Use Magic Device=5", "PRESPELL:1,Darkness"]),
    (RuleSetId::Ui, 23, "Deft Catcher", &["PREABILITY:1,CATEGORY=FEAT,Skill Focus (Sleight of Hand)"]),
    (RuleSetId::Ui, 24, "Drunkard's Recovery", &["PREVARGTEQ:PreStatScore_CON,13"]),
    (RuleSetId::Ui, 25, "Enrage Opponent", &["PREABILITY:1,CATEGORY=FEAT,Persuasive", "PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Amateur Swashbuckler],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Panache]", "PREVARGTEQ:PreStatScore_CHA,13"]),
    (RuleSetId::Ui, 26, "Entreating Critical", &["PREABILITY:1,CATEGORY=FEAT,Call Truce,Critical Focus,Persuasive", "PRESKILL:1,Diplomacy=5", "PRETOTALAB:11", "PREVARGTEQ:PreStatScore_CHA,15"]),
    (RuleSetId::Ui, 27, "Expeditious Sleuth", &["PREABILITY:1,CATEGORY=Special Ability,Investigator ~ Inspiration,Skald ~ Lore Master,Bard ~ Lore Master", "PRESKILL:1,Perception=3"]),
    (RuleSetId::Ui, 28, "Exquisite Sneak", &["PREABILITY:1,CATEGORY=FEAT,Careful Sneak", "PRESKILL:1,Stealth=6", "PREVARGTEQ:PreStatScore_DEX,15"]),
    (RuleSetId::Ui, 29, "Extra Contingency", &["PRELEVEL:MIN=19"]),
    (RuleSetId::Ui, 30, "Eye for Ingredients", &["PREABILITY:1,CATEGORY=FEAT,Eschew Materials", "PRESKILL:2,Appraise=6,Spellcraft=6"]),
    (RuleSetId::Ui, 31, "Feign Curse", &["PREABILITY:1,CATEGORY=FEAT,Deceitful", "PRESKILL:2,Bluff=5,Spellcraft=1"]),
    (RuleSetId::Ui, 32, "Fencing Grace", &["PREABILITY:1,CATEGORY=FEAT,Weapon Focus (rapier)", "PREVAREQ:HasWeaponFinesseFeat,1", "PREVARGTEQ:PreStatScore_DEX,13"]),
    (RuleSetId::Ui, 33, "Fey Spell Lore", &["PREABILITY:1,CATEGORY=Special Ability,Hunter ~ Orisons,Druid ~ Orisons", "PRESKILL:1,Spellcraft=1", "PREVARGTEQ:PreStatScore_CHA,13"]),
    (RuleSetId::Ui, 34, "Fey Spell Versatility", &["PRESKILL:1,Spellcraft=1", "PRESPELLCAST:TYPE=Divine", "PREVARGTEQ:PreStatScore_CHA,13"]),
    (RuleSetId::Ui, 36, "Fool Magic", &["PREABILITY:1,CATEGORY=FEAT,Deceitful", "PRESKILL:2,Disguise=1,Use Magic Device=1"]),
    (RuleSetId::Ui, 37, "Fox Insight", &["PREABILITY:1,CATEGORY=FEAT,Fox Style", "PREVARGTEQ:PreStatScore_INT,13"]),
    (RuleSetId::Ui, 38, "Fox Style", &["PREVARGTEQ:PreStatScore_INT,13"]),
    (RuleSetId::Ui, 39, "Fox Trickery", &["PREABILITY:4,CATEGORY=FEAT,Combat Expertise,Fox Insight,Fox Style,Improved Dirty Trick", "PREMULT:1,[PREVARGTEQ:PreStatScore_INT,13],[PREVARGTEQ:CombatFeatIntRequirement,13],[PREVARGTEQ:DirtyTricksterIntQualify,1]"]),
    (RuleSetId::Ui, 40, "Graceful Steal", &["PREABILITY:2,CATEGORY=FEAT,Agile Maneuvers,Improved Steal", "PRESKILL:1,Sleight of Hand=3", "PREVARGTEQ:PreStatScore_DEX,13"]),
    (RuleSetId::Ui, 41, "Improved Bravery", &["PREABILITY:1,CATEGORY=Special Ability,Fighter ~ Bravery", "PREVARGTEQ:PreStatScore_CHA,13"]),
    (RuleSetId::Ui, 42, "Improved Conceal Spell", &["PREABILITY:2,CATEGORY=FEAT,Conceal Spell,Deceitful", "PRESKILL:3,Bluff=5,Disguise=5,Sleight of Hand=10", "PRESPELLTYPE:1,ANY=3"]),
    (RuleSetId::Ui, 43, "Improved Sabotaging Sunder", &["PREABILITY:3,CATEGORY=FEAT,Improved Sunder,Power Attack,Sabotaging Sunder", "PRESKILL:1,Disable Device=9", "PREVARGTEQ:PreStatScore_STR,13"]),
    (RuleSetId::Ui, 44, "Incite Paranoia", &["PREABILITY:4,CATEGORY=FEAT,Combat Expertise,Deceitful,Greater Feint,Improved Feint", "PRETOTALAB:6", "PREVARGTEQ:PreStatScore_INT,13"]),
    (RuleSetId::Ui, 45, "Insightful Advice", &["PRESKILL:1,Perform (oratory)=3"]),
    (RuleSetId::Ui, 46, "Inspiring Bravery", &["PREABILITY:1,CATEGORY=Special Ability,Fighter ~ Bravery", "PREVARGTEQ:PreStatScore_CHA,13"]),
    (RuleSetId::Ui, 47, "Inspiring Mentor", &["PREABILITY:1,CATEGORY=Special Ability,Bardic Performance ~ Inspire Competence", "PREVARGTEQ:PreStatScore_CHA,13"]),
    (RuleSetId::Ui, 48, "Intoxicating Flattery", &["PREABILITY:1,CATEGORY=FEAT,Deceitful", "PRESKILL:1,Bluff=5"]),
    (RuleSetId::Ui, 49, "Ironclad Logic", &["PRESKILL:1,Diplomacy=3", "PREVARGTEQ:PreStatScore_INT,19"]),
    (RuleSetId::Ui, 50, "Lightning Draw", &["PREABILITY:1,CATEGORY=FEAT,Quick Draw", "PREABILITY:1,CATEGORY=Special Ability,Swashbuckler ~ Swashbuckler Initiative", "PRECLASS:1,Swashbuckler=7"]),
    (RuleSetId::Ui, 51, "Manipulative Agility", &["PRESKILL:2,Bluff=1,Sleight of Hand=1"]),
    (RuleSetId::Ui, 52, "Martial Dominance", &["PRESKILL:1,Intimidate=1", "PRETOTALAB:5"]),
    (RuleSetId::Ui, 53, "Measure Foe", &["PREABILITY:1,CATEGORY=FEAT,Street Smarts", "PRETOTALAB:1"]),
    (RuleSetId::Ui, 54, "Misdirection Attack", &["PREABILITY:4,CATEGORY=FEAT,Combat Expertise,Deceitful,Misdirection Redirection,Misdirection Tactics", "PRESKILL:1,Bluff=10", "PREVARGTEQ:PreStatScore_INT,13"]),
    (RuleSetId::Ui, 55, "Misdirection Redirection", &["PREABILITY:3,CATEGORY=FEAT,Combat Expertise,Misdirection Tactics,Deceitful", "PRESKILL:1,Bluff=10", "PREVARGTEQ:PreStatScore_INT,13"]),
    (RuleSetId::Ui, 56, "Misdirection Tactics", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Deceitful", "PRESKILL:1,Bluff=4", "PREVARGTEQ:PreStatScore_INT,13"]),
    (RuleSetId::Ui, 57, "My Blade Is Yours", &["PREABILITY:1,CATEGORY=FEAT,Combat Expertise", "PRESKILL:1,Sense Motive=3", "PREVARGTEQ:PreStatScore_INT,13"]),
    (RuleSetId::Ui, 58, "Nerve-Racking Negotiator", &["PREABILITY:1,CATEGORY=FEAT,Persuasive"]),
    (RuleSetId::Ui, 59, "Notorious Vigilante", &["PREABILITY:2,CATEGORY=FEAT,Dazzling Display,Weapon Focus", "PREABILITY:1,CATEGORY=Special Ability,Social Talent ~ Great Renown", "!PREALIGN:LG,NG,CG"]),
    (RuleSetId::Ui, 60, "Omnipresent Mentor", &["PREABILITY:1,CATEGORY=FEAT,Inspiring Mentor", "PREABILITY:1,CATEGORY=Special Ability,Bardic Performance ~ Inspire Competence", "PREVARGTEQ:PreStatScore_CHA,17"]),
    (RuleSetId::Ui, 61, "Ostentatious Rager", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Rage", "PRESKILL:1,Intimidate=5"]),
    (RuleSetId::Ui, 62, "Owl Dive", &["PREABILITY:3,CATEGORY=FEAT,Owl Style,Owl Swoop,Skill Focus (Stealth)", "PREMULT:1,[PRETOTALAB:7],[PRECLASS:1,Monk=5]", "PRESKILL:3,Acrobatics=1,Fly=1,Stealth=1", "PREVARGTEQ:PreStatScore_DEX,17"]),
    (RuleSetId::Ui, 63, "Owl Style", &["PREABILITY:1,CATEGORY=FEAT,Skill Focus (Stealth)", "PRESKILL:1,Stealth=1", "PREVARGTEQ:PreStatScore_DEX,13"]),
    (RuleSetId::Ui, 64, "Owl Swoop", &["PREABILITY:2,CATEGORY=FEAT,Owl Style,Skill Focus (Stealth)", "PRESKILL:2,Acrobatics=1,Stealth=1", "PREVARGTEQ:PreStatScore_DEX,15"]),
    (RuleSetId::Ui, 65, "Persuasive Bribery", &["PREABILITY:1,CATEGORY=FEAT,Persuasive"]),
    (RuleSetId::Ui, 66, "Piercing Grapple", &["PREABILITY:3,CATEGORY=FEAT,Improved Grapple,Improved Unarmed Strike,Quick Draw", "PRESKILL:1,Intimidate=7", "PREVARGTEQ:PreStatScore_DEX,13"]),
    (RuleSetId::Ui, 67, "Planar Wanderer", &["PREABILITY:1,CATEGORY=FEAT,Cartogramancer", "PRESKILL:2,Knowledge (geography)=10,Knowledge (planes)=10", "PRESPELL:2,Teleport (Greater),Plane Shift"]),
    (RuleSetId::Ui, 68, "Play to the Crowd", &["PREABILITY:1,CATEGORY=FEAT,Ironclad Logic", "PRESKILL:2,Diplomacy=5,Sense Motive=5", "PREVARGTEQ:PreStatScore_INT,19"]),
    (RuleSetId::Ui, 69, "Quick Favor", &["PREABILITY:1,CATEGORY=FEAT,Persuasive"]),
    (RuleSetId::Ui, 70, "Quick Study", &["PREABILITY:1,CATEGORY=Special Ability,Fighter ~ Bravery", "PRECLASS:1,Fighter=10", "PREVARGTEQ:PreStatScore_INT,13", "PREVARGTEQ:Bravery,3"]),
    (RuleSetId::Ui, 71, "Quiet Death", &["PRECLASS:1,Rogue=10", "PRESKILL:1,Stealth=10", "PREVARGTEQ:PreStatScore_DEX,19"]),
    (RuleSetId::Ui, 72, "Ranged Disable", &["PREABILITY:3,CATEGORY=FEAT,Far Shot,Point-Blank Shot,Weapon Focus", "PRESKILL:1,Disable Device=9"]),
    (RuleSetId::Ui, 73, "Ranged Feint", &["PRESKILL:1,Bluff=3", "PRETOTALAB:2"]),
    (RuleSetId::Ui, 75, "Ready for Anything", &["PREABILITY:4,CATEGORY=FEAT,Alertness,Improved Initiative,Lightning Reflexes,Quick Draw", "PREMULT:1,[PRETOTALAB:6],[PREABILITY:1,CATEGORY=Special Ability,Uncanny Dodge]"]),
    (RuleSetId::Ui, 76, "Sabotage Magic Item", &["PREABILITY:1,CATEGORY=FEAT,Magical Aptitude", "PRESKILL:2,Disable Device=5,Use Magic Device=5"]),
    (RuleSetId::Ui, 77, "Sabotage Specialist", &["PREABILITY:1,CATEGORY=FEAT,Deft Hands", "PRESKILL:1,Disable Device=5"]),
    (RuleSetId::Ui, 78, "Sabotaging Sunder", &["PREABILITY:2,CATEGORY=FEAT,Improved Sunder,Power Attack", "PRESKILL:1,Disable Device=7", "PREVARGTEQ:PreStatScore_STR,13"]),
    (RuleSetId::Ui, 79, "Sense Assumptions", &["PRESKILL:1,Sense Motive=3"]),
    (RuleSetId::Ui, 80, "Sense Relationships", &["PREABILITY:1,CATEGORY=FEAT,Street Smarts"]),
    (RuleSetId::Ui, 81, "Shadows of Fear", &["PREMULT:1,[PREVARGTEQ:SneakAttackDice,2],[PREVARGTEQ:HiddenStrikeDiceCount,2]"]),
    (RuleSetId::Ui, 82, "Sliding Dash", &["PREMULT:1,[PRESKILL:1,Acrobatics=10],[PREABILITY:1,CATEGORY=Special Ability,Duelist ~ Acrobatic Charge]", "PRESKILL:1,Bluff=3", "PREVARGTEQ:PreStatScore_DEX,15"]),
    (RuleSetId::Ui, 83, "Social Bravery", &["PREABILITY:1,CATEGORY=Special Ability,Fighter ~ Bravery", "PREVARGTEQ:PreStatScore_CHA,13"]),
    (RuleSetId::Ui, 84, "Starry Grace", &["PREABILITY:1,CATEGORY=FEAT,Weapon Focus (starknife)", "PREVAREQ:HasWeaponFinesseFeat,1", "PREVARGTEQ:PreStatScore_DEX,13"]),
    (RuleSetId::Ui, 85, "Startling Getaway", &["PREABILITY:1,CATEGORY=Special Ability,Vigilante ~ Startling Appearance"]),
    (RuleSetId::Ui, 86, "Street Carnage", &["PREABILITY:5,CATEGORY=FEAT,Improved Bull Rush,Improved Unarmed Strike,Power Attack,Street Style,Street Sweep", "PREMULT:1,[PRETOTALAB:8],[PRECLASS:1,Monk=7]", "PREVARGTEQ:PreStatScore_STR,15"]),
    (RuleSetId::Ui, 88, "Street Style", &["PREABILITY:3,CATEGORY=FEAT,Improved Bull Rush,Improved Unarmed Strike,Power Attack", "PREMULT:1,[PRETOTALAB:4],[PRECLASS:1,Monk=3]", "PREVARGTEQ:PreStatScore_STR,15"]),
    (RuleSetId::Ui, 89, "Street Sweep", &["PREABILITY:4,CATEGORY=FEAT,Improved Bull Rush,Improved Unarmed Strike,Power Attack,Street Style", "PREMULT:1,[PRETOTALAB:6],[PRECLASS:1,Monk=5]", "PREVARGTEQ:PreStatScore_STR,15"]),
    (RuleSetId::Ui, 90, "Structural Strike", &["PREABILITY:1,CATEGORY=Special Ability,Swashbuckler ~ Precise Strike,Duelist ~ Precise Strike", "PRESKILL:1,Knowledge (engineering)=5"]),
    (RuleSetId::Ui, 92, "Stylized Spell", &["PRESKILL:2,Bluff=5,Spellcraft=5"]),
    (RuleSetId::Ui, 93, "Subtle Enchantments", &["PREABILITY:2,CATEGORY=FEAT,Deceitful,Spell Focus (enchantment)"]),
    (RuleSetId::Ui, 94, "Superior Scryer", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus (divination)", "PRESPELLSCHOOLSUB:1,Scrying=1"]),
    (RuleSetId::Ui, 95, "Swipe and Stash", &["PRESKILL:1,Sleight of Hand=5"]),
    (RuleSetId::Ui, 96, "Telepathy Tap", &["PREMULT:1,[PRESPELL:1,Detect Thoughts,Telepathy],[PREABILITY:1,CATEGORY=Special Ability,Telepathy]", "PRESKILL:1,Sense Motive=10"]),
    (RuleSetId::Ui, 98, "Threatening Negotiator", &["PREABILITY:3,CATEGORY=FEAT,Nerve-Racking Negotiator,Persuasive,Skill Focus (Intimidate)"]),
    (RuleSetId::Ui, 100, "True Deception", &["PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Ninja Trick ~ Master Disguise],[PREABILITY:1,CATEGORY=Special Ability,Rogue Talent ~ Master of Disguise]", "PREMULT:1,[PRECLASS:1,Rogue=10],[PRECLASS:1,Rogue=10]", "PRESKILL:1,Disguise=17", "PREVARGTEQ:PreStatScore_CHA,17"]),
    (RuleSetId::Ui, 101, "Unimpeachable Honor", &["PREABILITY:1,CATEGORY=FEAT,Iron Will"]),
    (RuleSetId::Ui, 102, "Walking Sleight", &["PREABILITY:1,CATEGORY=FEAT,Deft Hands", "PRESKILL:1,Sleight of Hand=5"]),
    (RuleSetId::Ui, 103, "Willing Accomplice", &["PRESKILL:2,Bluff=3,Sense Motive=1", "PREVARGTEQ:PreStatScore_CHA,13"]),
    // Ultimate Wilderness — 127 of 135 record(s) in the hand-authored table carry at least one `PRE`-family token.
    (RuleSetId::Uw, 0, "Ambush Awareness", &["PREABILITY:1,CATEGORY=FEAT,Alertness"]),
    (RuleSetId::Uw, 1, "Animal Call", &["PRESKILL:2,Bluff=1,Knowledge (Nature)=1"]),
    (RuleSetId::Uw, 2, "Animal Disguise", &["PRESKILL:2,Disguise=6,Knowledge (Nature)=6"]),
    (RuleSetId::Uw, 3, "Animal Ferocity", &["PREABILITY:1,CATEGORY=Special Ability,Half-Orc ~ Orc Ferocity,Orc ~ Ferocity,Ferocity,Builder Racial Trait ~ Ferocity,Builder Racial Trait ~ Orc Ferocity", "PRETOTALAB:3"]),
    (RuleSetId::Uw, 4, "Aquatic Combatant", &["PRESKILL:1,Swim=1"]),
    (RuleSetId::Uw, 6, "Arctic Adaptation", &["PREABILITY:1,CATEGORY=Special Ability,Favored Terrain ~ Cold"]),
    (RuleSetId::Uw, 7, "Beast Hunter", &["PRESKILL:1,Knowledge (Nature)=1,Survival=1", "PRETOTALAB:1"]),
    (RuleSetId::Uw, 8, "Beastmaster Ire", &["PREABILITY:3,CATEGORY=FEAT,Alertness,Beastmaster Salvation,Beastmaster Style", "PRESKILL:2,Handle Animal=9,Sense Motive=5", "PREVARGTEQ:PreStatScore_CHA,13"]),
    (RuleSetId::Uw, 9, "Beastmaster Salvation", &["PREABILITY:2,CATEGORY=FEAT,Alertness,Beastmaster Style", "PRESKILL:2,Handle Animal=5,Sense Motive=5", "PREVARGTEQ:PreStatScore_CHA,13"]),
    (RuleSetId::Uw, 10, "Beastmaster Style", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Companion", "PRESKILL:1,Handle Animal=1", "PREVARGTEQ:PreStatScore_CHA,13"]),
    (RuleSetId::Uw, 11, "Boon Companion", &["PREMULT:1,[PREVARGT:AnimalCompanionLVL,0],[PREVARGT:FamiliarLVL,0],[PREVARGT:SpecialMountLVL,0],[PREVARGT:CavalierMountLVL,0],[PREVARGT:HuntmasterCompanionLVL,0],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Companion,TYPE.Familiar,TYPE.Mount,TYPE.Special Mount,TYPE.Huntmaster Companion]"]),
    (RuleSetId::Uw, 12, "Branch Pounce", &["PRESKILL:2,Climb=3,Stealth=3"]),
    (RuleSetId::Uw, 13, "Bristling Bull Rush", &["PREABILITY:2,CATEGORY=FEAT,Improved Bull Rush,Power Attack", "PRETOTALAB:3", "PREVARGTEQ:PreStatScore_STR,13"]),
    (RuleSetId::Uw, 14, "Bristling Drag", &["PREABILITY:2,CATEGORY=FEAT,Improved Drag,Power Attack", "PRETOTALAB:3", "PREVARGTEQ:PreStatScore_STR,13"]),
    (RuleSetId::Uw, 15, "Camouflaged Trap", &["PRESKILL:2,Craft (Traps)=4,Survival=4"]),
    (RuleSetId::Uw, 16, "Clinging Climber", &["PRESKILL:1,Climb=3", "PREVARGTEQ:PreStatScore_STR,13"]),
    (RuleSetId::Uw, 17, "Command Animals", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Positive Energy,TYPE.Channel Negative Energy", "PREDOMAIN:1,Animal"]),
    (RuleSetId::Uw, 18, "Command Plants", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Positive Energy,TYPE.Channel Negative Energy", "PREDOMAIN:1,Plant"]),
    (RuleSetId::Uw, 19, "Cover Tracks", &["PRESKILL:1,Survival=3"]),
    (RuleSetId::Uw, 20, "Crashing Wave Buffet", &["PREABILITY:4,CATEGORY=FEAT,Crashing Wave Style,Improved Drag,Improved Reposition,Improved Unarmed Strike", "PRETOTALAB:6", "PREVARGTEQ:PreStatScore_WIS,15"]),
    (RuleSetId::Uw, 21, "Crashing Wave Fist", &["PREABILITY:5,CATEGORY=FEAT,Crashing Wave Buffet,Crashing Wave Style,Improved Drag,Improved Reposition,Improved Unarmed Strike", "PRETOTALAB:9", "PREVARGTEQ:PreStatScore_WIS,15"]),
    (RuleSetId::Uw, 22, "Crashing Wave Style", &["PREABILITY:3,CATEGORY=FEAT,Improved Drag,Improved Reposition,Improved Unarmed Strike", "PREVARGTEQ:PreStatScore_WIS,13"]),
    (RuleSetId::Uw, 23, "Cultivate Magic Plants", &["PREABILITY:2,CATEGORY=FEAT,Brew Potion,Craft Wondrous Item", "PRESKILL:1,Knowledge (Nature)=1"]),
    (RuleSetId::Uw, 24, "Deadly Trap", &["PRESKILL:2,Craft (Traps)=8,Survival=8"]),
    (RuleSetId::Uw, 25, "Deep Diver", &["PREABILITY:1,CATEGORY=FEAT,Endurance"]),
    (RuleSetId::Uw, 26, "Desert Dweller", &["PREABILITY:1,CATEGORY=Special Ability,Favored Terrain ~ Desert"]),
    (RuleSetId::Uw, 27, "Eagle-Eyed", &["PRESKILL:1,Perception=3"]),
    (RuleSetId::Uw, 28, "Earth Magic", &["PREABILITY:1,CATEGORY=Special Ability,Basic Favored Terrain", "PRESPELLTYPE:1,ANY=1"]),
    (RuleSetId::Uw, 30, "Energized Wild Shape", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape", "PRETOTALAB:6", "PREVARGTEQ:PreStatScore_WIS,19"]),
    (RuleSetId::Uw, 31, "Enhanced Gnome Magic", &["PREABILITY:1,CATEGORY=Special Ability,Gnome ~ Gnome Magic", "PREFACT:1,TEMPLATES,IsGnome=true", "PRESKILL:1,Knowledge (Nature)=3", "PREVARGTEQ:PreStatScore_CHA,13"]),
    (RuleSetId::Uw, 33, "Expert Cartographer", &["PRESKILL:1,Craft (Maps)=3"]),
    (RuleSetId::Uw, 34, "Expert Explorer", &["PRESKILL:1,Knowledge (Nature)=5,Survival=5"]),
    (RuleSetId::Uw, 35, "Expert Salvager", &["PRESKILL:2,TYPE.Craft=2,Spellcraft=2"]),
    (RuleSetId::Uw, 36, "Extended Aspects", &["PREABILITY:1,CATEGORY=Special Ability,Shifter ~ Shifter Aspect"]),
    (RuleSetId::Uw, 37, "False Trail", &["PRESKILL:1,Survival=3"]),
    (RuleSetId::Uw, 38, "Fey Insight", &["PRESKILL:1,Knowledge (Nature)=2,Knowledge (Planes)=2", "PREVARGTEQ:PreStatScore_WIS,13"]),
    (RuleSetId::Uw, 39, "Fey Performance", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Bardic Performance"]),
    (RuleSetId::Uw, 40, "Fey-Guarded", &["PRESKILL:1,Knowledge (Nature)=3,Knowledge (Planes)=3"]),
    (RuleSetId::Uw, 41, "Flinging Charge", &["PREABILITY:1,CATEGORY=FEAT,Quick Draw", "PRETOTALAB:6"]),
    (RuleSetId::Uw, 42, "Foebane Magic", &["PREABILITY:1,CATEGORY=Special Ability,Basic Favored Enemy", "PRESPELLTYPE:1,ANY=1"]),
    (RuleSetId::Uw, 43, "Forester", &["PREABILITY:1,CATEGORY=Special Ability,Favored Terrain ~ Forest"]),
    (RuleSetId::Uw, 44, "Frightful Shape", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape", "PRETOTALAB:9", "PREVARGTEQ:PreStatScore_WIS,19"]),
    (RuleSetId::Uw, 45, "Greater Beast Hunter", &["PREABILITY:2,CATEGORY=FEAT,Beast Hunter,Improved Beast Hunter", "PRESKILL:1,Knowledge (Nature)=6,Survival=6", "PRETOTALAB:6"]),
    (RuleSetId::Uw, 46, "Greater Hunter's Bond", &["PRECLASS:1,Ranger=12", "PREMULT:2,[PREABILITY:1,CATEGORY=Special Ability,Hunter's Bond ~ Companion],[PREABILITY:1,CATEGORY=FEAT,Improved Hunter's Bond]"]),
    (RuleSetId::Uw, 47, "Greater Spring Attack", &["PREABILITY:6,CATEGORY=FEAT,Acrobatic Steps,Dodge,Improved Spring Attack,Mobility,Nimble Moves,Spring Attack", "PRETOTALAB:16", "PREVARGTEQ:PreStatScore_DEX,17"]),
    (RuleSetId::Uw, 48, "Greater Wilding Strike", &["PREABILITY:4,CATEGORY=FEAT,Improved Unarmed Strike,Improved Wilding Strike,Wilding,Wilding Strike", "PRETOTALAB:16", "PREVARGTEQ:PreStatScore_STR,18"]),
    (RuleSetId::Uw, 50, "Harder They Fall", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Power Attack", "PREVARGTEQ:PreStatScore_STR,13", "PREVARGTEQ:PreStatScore_INT,13"]),
    (RuleSetId::Uw, 51, "Hide Worker", &["PRESKILL:2,Craft (Armor)=3,Knowledge (Nature)=3"]),
    (RuleSetId::Uw, 52, "Ice Climber", &["PREABILITY:1,CATEGORY=FEAT,Arctic Adaptation,Mountaineer", "PRESKILL:1,Climb=2"]),
    (RuleSetId::Uw, 53, "Improved Beast Hunter", &["PREABILITY:1,CATEGORY=FEAT,Beast Hunter", "PRESKILL:1,Knowledge (Nature)=3,Survival=3", "PRETOTALAB:3"]),
    (RuleSetId::Uw, 54, "Improved Hunter's Bond", &["PREABILITY:1,CATEGORY=Special Ability,Hunter's Bond ~ Companion", "PRECLASS:1,Ranger=9"]),
    (RuleSetId::Uw, 55, "Improved Natural Poison Harvester", &["PREABILITY:1,CATEGORY=FEAT,Natural Poison Harvester", "PRESKILL:2,Craft (Alchemy)=9,Survival=9"]),
    (RuleSetId::Uw, 56, "Improved Spring Attack", &["PREABILITY:4,CATEGORY=FEAT,Dodge,Mobility,Nimble Moves,Spring Attack", "PRETOTALAB:9", "PREVARGTEQ:PreStatScore_DEX,15"]),
    (RuleSetId::Uw, 57, "Improved Wilding Strike", &["PREABILITY:3,CATEGORY=FEAT,Improved Unarmed Strike,Wilding,Wilding Strike", "PRETOTALAB:11", "PREVARGTEQ:PreStatScore_STR,16"]),
    (RuleSetId::Uw, 58, "Indomitable Mountain Avalanche", &["PREABILITY:4,CATEGORY=FEAT,Combat Reflexes,Improved Unarmed Strike,Indomitable Mountain Peak,Indomitable Mountain Style", "PRETOTALAB:9", "PREVARGTEQ:PreStatScore_WIS,15"]),
    (RuleSetId::Uw, 59, "Indomitable Mountain Peak", &["PREABILITY:3,CATEGORY=FEAT,Combat Reflexes,Improved Unarmed Strike,Indomitable Mountain Style", "PRETOTALAB:6", "PREVARGTEQ:PreStatScore_WIS,15"]),
    (RuleSetId::Uw, 60, "Indomitable Mountain Style", &["PREABILITY:2,CATEGORY=FEAT,Combat Reflexes,Improved Unarmed Strike", "PRETOTALAB:4", "PREVARGTEQ:PreStatScore_WIS,13"]),
    (RuleSetId::Uw, 61, "Intimidate Animals", &["PREMULT:2,[PREABILITY:1,CATEGORY=Special Ability,Wild Empathy,TYPE.WildEmpathy],[PREABILITY:1,CATEGORY=FEAT,Greater Wild Empathy]", "PRESKILL:2,Intimidate=5,Knowledge (Nature)=5"]),
    (RuleSetId::Uw, 62, "Jaguar Pounce", &["PRETOTALAB:4"]),
    (RuleSetId::Uw, 63, "Jungle Survivalist", &["PREABILITY:1,CATEGORY=Special Ability,Favored Terrain ~ Jungle"]),
    (RuleSetId::Uw, 64, "Live Off the Land", &["PREABILITY:1,CATEGORY=Special Ability,Basic Favored Terrain"]),
    (RuleSetId::Uw, 66, "Mountaineer", &["PREABILITY:1,CATEGORY=Special Ability,Favored Terrain ~ Mountains"]),
    (RuleSetId::Uw, 67, "Mutated Shape", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape", "PRETOTALAB:6", "PREVARGTEQ:PreStatScore_WIS,19"]),
    (RuleSetId::Uw, 68, "Natural Poison Antitoxin", &["PREABILITY:1,CATEGORY=FEAT,Natural Poison Harvester", "PRESKILL:2,Craft (Alchemy)=8,Survival=8"]),
    (RuleSetId::Uw, 69, "Natural Poison Harvester", &["PRESKILL:2,Craft (Alchemy)=6,Survival=6"]),
    (RuleSetId::Uw, 70, "Nature's Freedom", &["PREABILITY:1,CATEGORY=FEAT,Catch Off-Guard", "PRESKILL:1,Knowledge (Nature)=2,Survival=2", "PRETOTALAB:2"]),
    (RuleSetId::Uw, 71, "Nature's Weapons", &["PREABILITY:1,CATEGORY=FEAT,Catch Off-Guard", "PRESKILL:1,Knowledge (Nature)=2,Survival=2", "PRETOTALAB:2"]),
    (RuleSetId::Uw, 72, "Night Sky Hex", &["PREMULT:2,[PREABILITY:1,CATEGORY=Special Ability,Witch ~ Hex],[PREABILITY:1,CATEGORY=Special Ability,Witch Patron ~ Moon,Witch Patron ~ Stars,Witch Patron ~ Winter]"]),
    (RuleSetId::Uw, 73, "One Eye Open", &["PREABILITY:1,CATEGORY=FEAT,Alertness", "PREVARGTEQ:PreStatScore_WIS,13"]),
    (RuleSetId::Uw, 74, "One with the Land", &["PREABILITY:1,CATEGORY=Special Ability,Basic Favored Terrain"]),
    (RuleSetId::Uw, 75, "Out of the Sun", &["PRESKILL:2,Bluff=3,Stealth=3"]),
    (RuleSetId::Uw, 76, "Plains Nomad", &["PREABILITY:1,CATEGORY=Special Ability,Favored Terrain ~ Plains"]),
    (RuleSetId::Uw, 78, "Rubble Skirmisher", &["PREABILITY:1,CATEGORY=FEAT,Nimble Moves", "PRETOTALAB:2", "PREVARGTEQ:PreStatScore_DEX,13"]),
    (RuleSetId::Uw, 79, "Scion of the Land", &["PRESKILL:1,Survival=1"]),
    (RuleSetId::Uw, 80, "Shifter's Edge", &["PREMULT:2,[PREVAREQ:HasWeaponFinesseFeat,1],[PREABILITY:1,CATEGORY=Special Ability,Shifter ~ Shifter Claws]", "PREVARGTEQ:PreStatScore_DEX,13"]),
    (RuleSetId::Uw, 81, "Shifter's Rush", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape", "PRECLASS:1,Shifter=4"]),
    (RuleSetId::Uw, 82, "Storm Survivor", &["PRESKILL:2,Knowledge (Nature)=2,Survival=2"]),
    (RuleSetId::Uw, 83, "Swamper", &["PREABILITY:1,CATEGORY=Special Ability,Favored Terrain ~ Swamp"]),
    (RuleSetId::Uw, 84, "Thrill of the Hunt", &["PREMULT:1,[PRETOTALAB:4],[PREABILITY:1,CATEGORY=Special Ability,Shifter ~ Track,Ranger ~ Track]", "PRESKILL:1,Survival=1"]),
    (RuleSetId::Uw, 85, "Torrid Tolerance", &["PREABILITY:1,CATEGORY=FEAT,Desert Dweller,Jungle Survivalist"]),
    (RuleSetId::Uw, 86, "Totemic Disciple", &["PREABILITY:2,CATEGORY=FEAT,Athletic,Totemic Initiate", "!PREALIGN:LG,LN,LE", "PRETOTALAB:9", "PREVARGTEQ:PreStatScore_STR,15", "PREVARGTEQ:PreStatScore_DEX,13"]),
    (RuleSetId::Uw, 87, "Totemic Initiate", &["PREABILITY:1,CATEGORY=FEAT,Athletic", "!PREALIGN:LG,LN,LE", "PRETOTALAB:5", "PREVARGTEQ:PreStatScore_STR,13", "PREVARGTEQ:PreStatScore_DEX,13"]),
    (RuleSetId::Uw, 88, "Totemic Master", &["PREABILITY:3,CATEGORY=FEAT,Athletic,Totemic Disciple,Totemic Initiate", "!PREALIGN:LG,LN,LE", "PRETOTALAB:13", "PREVARGTEQ:PreStatScore_STR,17", "PREVARGTEQ:PreStatScore_DEX,13", "PREVARGTEQ:PreStatScore_WIS,13"]),
    (RuleSetId::Uw, 90, "Tribal Hunter", &["PREABILITY:1,CATEGORY=FEAT,Animal Affinity"]),
    (RuleSetId::Uw, 91, "Verdant Spell", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Enchantment)", "PRESKILL:1,Knowledge (Nature)=6"]),
    (RuleSetId::Uw, 92, "Vigilant Charger", &["PREABILITY:1,CATEGORY=FEAT,Combat Reflexes", "PREVARGTEQ:PreStatScore_STR,13", "PREVARGTEQ:PreStatScore_DEX,13"]),
    (RuleSetId::Uw, 93, "Voice of Beasts", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape"]),
    (RuleSetId::Uw, 95, "Wild Growth Channel", &["PREABILITY:1,CATEGORY=Special Ability,Channel Positive Energy", "PREDOMAIN:1,Plant"]),
    (RuleSetId::Uw, 96, "Wild Growth Hex", &["PREMULT:2,[PREABILITY:1,CATEGORY=Special Ability,Witch ~ Hex],[PREABILITY:1,CATEGORY=Special Ability,Witch Patron ~ Summer,Witch Patron ~ Thorns,Witch Patron ~ Woodlands]"]),
    (RuleSetId::Uw, 97, "Wild Vigor", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape"]),
    (RuleSetId::Uw, 98, "Wilding", &["PREALIGN:NG,NE,TN,CN,LN", "PRELEVEL:MAX=1"]),
    (RuleSetId::Uw, 99, "Wilding Mind", &["PREABILITY:1,CATEGORY=FEAT,Wilding", "PREVARGTEQ:PreStatScore_WIS,13"]),
    (RuleSetId::Uw, 100, "Wilding Senses", &["PREABILITY:1,CATEGORY=FEAT,Wilding", "PREVARGTEQ:PreStatScore_WIS,13"]),
    (RuleSetId::Uw, 101, "Wilding Stride", &["PREABILITY:1,CATEGORY=FEAT,Wilding", "PREVARGTEQ:PreStatScore_WIS,13"]),
    (RuleSetId::Uw, 102, "Wilding Strike", &["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Wilding", "PREVARGTEQ:PreStatScore_STR,13"]),
    (RuleSetId::Uw, 103, "Witchbreaker", &["PREABILITY:1,CATEGORY=FEAT,Iron Will"]),
    (RuleSetId::Uw, 104, "Wolf Rider", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Special Mount,TYPE.Mount", "PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Undersized Mount],[PREMULT:1,[PREVARGTEQ:SpecialMountLVL,7],[PREVARGTEQ:CavalierMountLVL,7],[PREVARGTEQ:SamuraiMountLVL,7]]", "PRESKILL:1,Knowledge (Nature)=1"]),
    (RuleSetId::Uw, 105, "Wolf Savage", &["PREABILITY:3,CATEGORY=FEAT,Improved Unarmed Strike,Wolf Style,Wolf Trip", "PRESKILL:1,Knowledge (Nature)=9", "PREVARGTEQ:PreStatScore_WIS,17"]),
    (RuleSetId::Uw, 106, "Wolf Style", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRESKILL:1,Knowledge (Nature)=3", "PREVARGTEQ:PreStatScore_WIS,13"]),
    (RuleSetId::Uw, 107, "Wolf Trip", &["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Wolf Style", "PRESKILL:1,Knowledge (Nature)=6", "PREVARGTEQ:PreStatScore_WIS,15"]),
    (RuleSetId::Uw, 108, "Wood Crafter", &["PRESKILL:2,Craft (Armor)=3,Knowledge (Nature)=3"]),
    (RuleSetId::Uw, 109, "Woodland Wraith", &["PREABILITY:1,CATEGORY=FEAT,Dodge", "PRETOTALAB:6", "PREVARGTEQ:PreStatScore_DEX,15"]),
    (RuleSetId::Uw, 110, "Advanced Gathlain Magic", &["PRELEVEL:MIN=3", "PREFACT:1,TEMPLATES,IsGathlain=true", "PREVARGTEQ:PreStatScore_CHA,13"]),
    (RuleSetId::Uw, 111, "Greater Gathlain Magic", &["PREABILITY:1,CATEGORY=FEAT,Advanced Gathlain Magic", "PRELEVEL:MIN=9", "PREFACT:1,TEMPLATES,IsGathlain=true", "PREVARGTEQ:PreStatScore_CHA,15"]),
    (RuleSetId::Uw, 112, "Green Tongue", &["PRELEVEL:MIN=5", "PREFACT:1,TEMPLATES,IsGathlain=true", "PREVARGTEQ:PreStatScore_CHA,13"]),
    (RuleSetId::Uw, 113, "Seasoned Flier", &["PREMOVE:1,Fly=1", "PREFACT:1,TEMPLATES,IsGathlain=true", "PRESKILL:1,Fly=5", "PREVARGTEQ:PreStatScore_DEX,15"]),
    (RuleSetId::Uw, 114, "Superior Gathlain Magic", &["PREABILITY:2,CATEGORY=FEAT,Advanced Gathlain Magic,Greater Gathlain Magic", "PRELEVEL:MIN=15", "PREFACT:1,TEMPLATES,IsGathlain=true", "PREVARGTEQ:PreStatScore_CHA,15"]),
    (RuleSetId::Uw, 115, "Symbiotic Resilience", &["PREFACT:1,TEMPLATES,IsGathlain=true", "PREVARGTEQ:PreStatScore_CON,13"]),
    (RuleSetId::Uw, 116, "Wandering Mind", &["PRELEVEL:MIN=9", "PREFACT:1,TEMPLATES,IsGathlain=true"]),
    (RuleSetId::Uw, 117, "Delectable Feint", &["PREABILITY:1,CATEGORY=Special Ability,Ghoran ~ Delicious", "PREFACT:1,TEMPLATES,IsGhoran=true", "PREVARGTEQ:PreStatScore_CON,15"]),
    (RuleSetId::Uw, 118, "Inner Light", &["PREFACT:1,TEMPLATES,IsGhoran=true", "PREVARGTEQ:PreStatScore_WIS,13"]),
    (RuleSetId::Uw, 119, "Sproutling", &["PREABILITY:1,CATEGORY=Special Ability,Ghoran ~ Seed,Ghoran ~ Ghorus Seed", "PREFACT:1,TEMPLATES,IsGhoran=true", "PREVARGTEQ:PreStatScore_CHA,15"]),
    (RuleSetId::Uw, 120, "Climbing Vine", &["PRERACE:1,Vine Leshy", "PREVARGTEQ:PreStatScore_STR,13"]),
    (RuleSetId::Uw, 121, "Kudzu Grappler", &["PREABILITY:3,CATEGORY=FEAT,Improved Grapple,Improved Unarmed Strike,Greater Grapple", "PRERACE:1,Vine Leshy", "PRETOTALAB:6", "PREVARGTEQ:PreStatScore_DEX,13"]),
    (RuleSetId::Uw, 122, "Photosynthetic Healing", &["PREABILITY:1,CATEGORY=Special Ability,Vine Leshy ~ Change Shape,Leshy ~ Change Shape", "PRERACE:1,RACESUBTYPE=Leshy", "PREVARGTEQ:PreStatScore_CON,15"]),
    (RuleSetId::Uw, 123, "Reactive Reversion", &["PREABILITY:1,CATEGORY=Special Ability,Vine Leshy ~ Change Shape,Leshy ~ Change Shape", "PRERACE:1,RACESUBTYPE=Leshy", "PREVARGTEQ:PreStatScore_DEX,13"]),
    (RuleSetId::Uw, 124, "Devotion against the Unnatural", &["PREABILITY:1,CATEGORY=Special Ability,Companion ~ Devotion", "PRECLASS:1,Companion=1"]),
    (RuleSetId::Uw, 125, "Disruptive Companion", &["PRECLASS:1,Companion=1"]),
    (RuleSetId::Uw, 126, "Feral Grace", &["PREVAREQ:HasWeaponFinesseFeat,1", "PRECLASS:1,Companion=1", "PRETOTALAB:6"]),
    (RuleSetId::Uw, 127, "Ferocious Beast", &["!PREABILITY:1,CATEGORY=FEAT,Intimidating Prowess", "PRECLASS:1,Companion=1", "PRESKILL:1,Intimidate=1"]),
    (RuleSetId::Uw, 128, "Ferocious Feint", &["PRECLASS:1,Companion=1", "PRESKILL:1,Bluff=1"]),
    (RuleSetId::Uw, 129, "Greater Tenacious Hunter", &["PREABILITY:2,CATEGORY=FEAT,Combat Reflexes,Tenacious Hunter", "PRECLASS:1,Companion=1", "PRESKILL:2,Perception=3,Stealth=3"]),
    (RuleSetId::Uw, 130, "Improved Intercept Blow", &["PREABILITY:1,CATEGORY=FEAT,Intercept Blow", "PRECLASS:1,Companion=1", "PRETOTALAB:6"]),
    (RuleSetId::Uw, 131, "Intercept Blow", &["PRECLASS:1,Companion=1", "PRETOTALAB:1"]),
    (RuleSetId::Uw, 132, "Reflexive Interception", &["PREABILITY:1,CATEGORY=FEAT,Intercept Blow", "PREABILITY:1,CATEGORY=Special Ability,Evasion", "PRECLASS:1,Companion=1", "PRETOTALAB:3"]),
    (RuleSetId::Uw, 133, "Share Feature", &["PRECLASS:1,Companion=1"]),
    (RuleSetId::Uw, 134, "Tenacious Hunter", &["PRECLASS:1,Companion=1", "PRESKILL:2,Perception=3,Stealth=3"]),
    // Ultimate Combat — 247 of 261 record(s) in the hand-authored table carry at least one `PRE`-family token.
    (RuleSetId::Uc, 0, "Adder Strike", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Poison Use,Poison Use,Alchemist ~ Poison Use,Rogue ~ Poison Use", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRESKILL:1,Craft (Alchemy)=1"]),
    (RuleSetId::Uc, 1, "Adept Champion", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Smite Evil", "PRETOTALAB:5"]),
    (RuleSetId::Uc, 2, "Amateur Gunslinger", &["!PREABILITY:1,CATEGORY=Special Ability,Gunslinger ~ Grit"]),
    (RuleSetId::Uc, 3, "Back to Back", &["PRESKILL:1,Perception=3"]),
    (RuleSetId::Uc, 4, "Branded for Retribution", &["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Bane,Inquisitor ~ Greater Bane"]),
    (RuleSetId::Uc, 5, "Betrayer", &["PREABILITY:2,CATEGORY=FEAT,Quick Draw,Persuasive", "PRETOTALAB:3"]),
    (RuleSetId::Uc, 6, "Binding Throw", &["PREABILITY:4,CATEGORY=FEAT,Improved Grapple,Improved Trip,Improved Unarmed Strike,Ki Throw"]),
    (RuleSetId::Uc, 8, "Boar Ferocity", &["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Boar Style", "PRESKILL:1,Intimidate=6"]),
    (RuleSetId::Uc, 9, "Boar Shred", &["PREABILITY:3,CATEGORY=FEAT,Improved Unarmed Strike,Boar Ferocity,Boar Style", "PRESKILL:1,Intimidate=9"]),
    (RuleSetId::Uc, 10, "Boar Style", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRESKILL:1,Intimidate=3"]),
    (RuleSetId::Uc, 11, "Body Shield", &["PREABILITY:2,CATEGORY=FEAT,Improved Grapple,Improved Unarmed Strike", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 13, "Bonebreaker", &["PREABILITY:4,CATEGORY=FEAT,Improved Grapple,Improved Unarmed Strike,Jawbreaker,Stunning Fist", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRESKILL:1,HEAL=9"]),
    (RuleSetId::Uc, 14, "Break Guard", &["PREABILITY:3,CATEGORY=FEAT,Combat Expertise,Improved Disarm,Two-Weapon Fighting", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,15],[PREVARGTEQ:FeatDexRequirement,15]"]),
    (RuleSetId::Uc, 15, "Broken Wing Gambit", &["PRESKILL:1,Bluff=5"]),
    (RuleSetId::Uc, 16, "Cartwheel Dodge", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Improved Evasion", "PRESKILL:1,Acrobatics=12"]),
    (RuleSetId::Uc, 17, "Cavalry Formation", &["PREABILITY:1,CATEGORY=FEAT,Mounted Combat"]),
    (RuleSetId::Uc, 18, "Channeling Scourge", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Energy", "PRECLASS:1,Inquisitor=1"]),
    (RuleSetId::Uc, 19, "Jawbreaker", &["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Stunning Fist", "PRESKILL:1,Heal=6"]),
    (RuleSetId::Uc, 20, "Arc Slinger", &["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot", "PREWEAPONPROF:1,Sling,Sling Staff (Halfling)"]),
    (RuleSetId::Uc, 21, "Channeled Revival", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Positive Energy", "PREMULT:1,[PREVARGTEQ:OracleChannelDice,6],[PREVARGTEQ:ClericChannelPositiveEnergyDice,6],[PREVARGTEQ:PaladinChannelDice,6],[PREVARGTEQ:ClassChannelPositiveEnergyDice,6]", "PREMULT:1,[PREVARGTEQ:OracleChannelDieSize,6],[PREVARGTEQ:ClericChannelPositiveEnergyDieSize,6],[PREVARGTEQ:PaladinChannelDieSize,6],[PREVARGTEQ:ClassChannelPositiveEnergyDieSize,6]"]),
    (RuleSetId::Uc, 22, "Charging Hurler", &["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot"]),
    (RuleSetId::Uc, 23, "Chokehold", &["PREABILITY:2,CATEGORY=FEAT,Improved Grapple,Improved Unarmed Strike", "PREMULT:1,[PREVARGTEQ:MonkFeatQualify,5],[PRETOTALAB:6]"]),
    (RuleSetId::Uc, 24, "Cleaving Finish", &["PREABILITY:2,CATEGORY=FEAT,Cleave,Power Attack", "PRESTAT:1,STR=13"]),
    (RuleSetId::Uc, 25, "Close-Quarters Thrower", &["PREABILITY:2,CATEGORY=FEAT,Dodge,Weapon Focus (TYPE=Thrown)", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]"]),
    (RuleSetId::Uc, 26, "Clustered Shots", &["PREABILITY:2,CATEGORY=FEAT,Point-Blank Shot,Precise Shot", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 27, "Combat Medic", &["PRESKILL:1,Heal=5"]),
    (RuleSetId::Uc, 28, "Combat Style Master", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PREABILITY:2,CATEGORY=FEAT,TYPE=Style", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:MonkFeatQualify,5]"]),
    (RuleSetId::Uc, 29, "Contingent Channeling", &["PREABILITY:1,CATEGORY=Special Ability,True Healer ~ Merciful Healer", "PREABILITY:1,CATEGORY=FEAT,Selective Channeling"]),
    (RuleSetId::Uc, 30, "Coordinated Charge", &["PREABILITY:2,CATEGORY=FEAT,TYPE=Teamwork", "PRETOTALAB:10"]),
    (RuleSetId::Uc, 31, "Crushing Blow", &["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Stunning Fist"]),
    (RuleSetId::Uc, 32, "Deadly Finish", &["PRETOTALAB:11"]),
    (RuleSetId::Uc, 34, "Death or Glory", &["PREABILITY:1,CATEGORY=FEAT,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 35, "Deathless Initiate", &["PREABILITY:2,CATEGORY=FEAT,Diehard,Endurance", "PREFACT:1,TEMPLATES,IsOrc=true", "PRESTAT:2,STR=13,CON=13", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 36, "Deathless Master", &["PREABILITY:4,CATEGORY=FEAT,Deathless Initiate,Diehard,Endurance,Ironhide", "PREFACT:1,TEMPLATES,IsOrc=true", "PRESTAT:2,STR=13,CON=15", "PRETOTALAB:9"]),
    (RuleSetId::Uc, 37, "Deathless Zealot", &["PREABILITY:4,CATEGORY=FEAT,Deathless Master,Deathless Initiate,Diehard,Endurance,Ironhide", "PREFACT:1,TEMPLATES,IsOrc=true", "PRESTAT:2,STR=13,CON=17", "PRETOTALAB:12"]),
    (RuleSetId::Uc, 38, "Deceptive Exchange", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Feint", "PRESTAT:1,INT=13"]),
    (RuleSetId::Uc, 39, "Defensive Weapon Training", &["PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:5"]),
    (RuleSetId::Uc, 40, "Deft Shootist Deed", &["PREABILITY:2,CATEGORY=FEAT,Dodge,Mobility", "PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Gunslinger ~ Grit],[PREABILITY:1,CATEGORY=FEAT,Amateur Gunslinger]"]),
    (RuleSetId::Uc, 41, "Destructive Dispel", &["PREMULT:1,[PRECLASS:1,SPELLCASTER=11],[PREVARGTEQ:CasterLevel_Highest,11]", "PRESPELL:1,Dispel Magic,Dispel Magic (Greater)"]),
    (RuleSetId::Uc, 42, "Devastating Strike", &["PREABILITY:1,CATEGORY=FEAT,Vital Strike", "PRETOTALAB:9"]),
    (RuleSetId::Uc, 43, "Dimensional Agility", &["PREMULT:1,[PRESPELL:1,Dimension Door],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Abundant Step]"]),
    (RuleSetId::Uc, 44, "Dimensional Assault", &["PREABILITY:1,CATEGORY=FEAT,Dimensional Agility", "PREMULT:1,[PRESPELL:1,Dimension Door],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Abundant Step]"]),
    (RuleSetId::Uc, 45, "Dimensional Dervish", &["PREABILITY:2,CATEGORY=FEAT,Dimensional Agility,Dimensional Assault", "PREMULT:1,[PRESPELL:1,Dimension Door],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Abundant Step]", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 46, "Dimensional Maneuvers", &["PREABILITY:3,CATEGORY=FEAT,Dimensional Agility,Dimensional Assault,Dimensional Dervish", "PREMULT:1,[PRESPELL:1,Dimension Door],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Abundant Step]", "PRETOTALAB:9"]),
    (RuleSetId::Uc, 47, "Dimensional Savant", &["PREABILITY:3,CATEGORY=FEAT,Dimensional Agility,Dimensional Assault,Dimensional Dervish", "PREMULT:1,[PRESPELL:1,Dimension Door],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Abundant Step]", "PRETOTALAB:9"]),
    (RuleSetId::Uc, 48, "Discordant Voice", &["PREABILITY:1,CATEGORY=Special Ability,Bard ~ Bardic Performance", "PRESKILL:1,Perform (Oratory)=10,Perform (Sing)=10"]),
    (RuleSetId::Uc, 49, "Disengaging Feint", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Feint", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]"]),
    (RuleSetId::Uc, 50, "Disengaging Flourish", &["PREABILITY:3,CATEGORY=FEAT,Combat Expertise,Disengaging Feint,Improved Feint", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]"]),
    (RuleSetId::Uc, 51, "Disengaging Shot", &["PREABILITY:5,CATEGORY=FEAT,Combat Expertise,Disengaging Feint,Dodge,Improved Feint,Mobility", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]"]),
    (RuleSetId::Uc, 52, "Disorienting Maneuver", &["PREABILITY:1,CATEGORY=FEAT,Dodge", "PRESKILL:1,Acrobatics=5"]),
    (RuleSetId::Uc, 53, "Dispel Synergy", &["PRESKILL:1,Spellcraft=5"]),
    (RuleSetId::Uc, 54, "Dispelling Critical", &["PREABILITY:1,CATEGORY=FEAT,Arcane Strike", "PRESPELL:1,Dispel Magic", "PRETOTALAB:11"]),
    (RuleSetId::Uc, 55, "Disposable Weapon", &["PRETOTALAB:1", "PREWEAPONPROF:1,TYPE.Fragile"]),
    (RuleSetId::Uc, 56, "Disruptive Recall", &["PREABILITY:1,CATEGORY=Special Ability,Spell Recall ~ Magus", "PRESKILL:1,Spellcraft=5"]),
    (RuleSetId::Uc, 57, "Distance Thrower", &["PRESTAT:1,STR=13"]),
    (RuleSetId::Uc, 58, "Djinni Spin", &["PREABILITY:4,CATEGORY=FEAT,Djinni Style,Djinni Spirit,Elemental Fist,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:13],[PREVARGTEQ:MonkFeatQualify,11]", "PRESTAT:2,CON=15,WIS=17"]),
    (RuleSetId::Uc, 59, "Djinni Spirit", &["PREABILITY:3,CATEGORY=FEAT,Djinni Style,Elemental Fist,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:11],[PREVARGTEQ:MonkFeatQualify,9]", "PRESTAT:2,CON=15,WIS=15"]),
    (RuleSetId::Uc, 60, "Djinni Style", &["PREABILITY:2,CATEGORY=FEAT,Elemental Fist,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:9],[PREVARGTEQ:MonkFeatQualify,5]", "PRESTAT:2,CON=13,WIS=15"]),
    (RuleSetId::Uc, 61, "Domain Strike", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Domains", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike"]),
    (RuleSetId::Uc, 62, "Double Bane", &["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Bane", "PREABILITY:1,CATEGORY=FEAT,Two-Weapon Fighting"]),
    (RuleSetId::Uc, 63, "Drag Down", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Trip", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]"]),
    (RuleSetId::Uc, 64, "Dragon Ferocity", &["PREABILITY:3,CATEGORY=FEAT,Improved Unarmed Strike,Dragon Style,Stunning Fist", "PRESKILL:1,Acrobatics=5", "PRESTAT:1,STR=15"]),
    (RuleSetId::Uc, 65, "Dragon Roar", &["PREABILITY:3,CATEGORY=FEAT,Improved Unarmed Strike,Dragon Style,Stunning Fist", "PRESKILL:1,Acrobatics=8", "PRESTAT:1,STR=15"]),
    (RuleSetId::Uc, 66, "Dragon Style", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRESKILL:1,Acrobatics=3", "PRESTAT:1,STR=15"]),
    (RuleSetId::Uc, 67, "Dramatic Display", &["PREABILITY:1,CATEGORY=FEAT,Dazzling Display"]),
    (RuleSetId::Uc, 68, "Earth Child Binder", &["PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Defensive Training,Gnome ~ Defensive Training", "PREABILITY:6,CATEGORY=FEAT,Earth Child Style,Earth Child Topple,Greater Trip,Improved Trip,Improved Unarmed Strike,Stunning Fist", "PREFACT:1,TEMPLATES,IsDwarf=true,IsGnome=True", "PRESKILL:1,Acrobatics=9", "PRESTAT:1,WIS=13"]),
    (RuleSetId::Uc, 69, "Earth Child Style", &["PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Defensive Training,Gnome ~ Defensive Training", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PREFACT:1,TEMPLATES,IsDwarf=true,IsGnome=True", "PRESKILL:1,Acrobatics=3", "PRESTAT:1,WIS=13"]),
    (RuleSetId::Uc, 70, "Earth Child Topple", &["PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Defensive Training,Gnome ~ Defensive Training", "PREABILITY:3,CATEGORY=FEAT,Improved Unarmed Strike,Earth Child Style,Improved Trip", "PREFACT:1,TEMPLATES,IsDwarf=true,IsGnome=True", "PRESKILL:1,Acrobatics=6", "PRESTAT:1,WIS=13"]),
    (RuleSetId::Uc, 71, "Efreeti Stance", &["PREABILITY:3,CATEGORY=FEAT,Efreeti Style,Elemental Fist,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:11],[PREVARGTEQ:MonkFeatQualify,9]", "PRESTAT:2,CON=15,WIS=15"]),
    (RuleSetId::Uc, 72, "Efreeti Style", &["PREABILITY:2,CATEGORY=FEAT,Elemental Fist,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:9],[PREVARGTEQ:MonkFeatQualify,5]", "PRESTAT:2,CON=13,WIS=15"]),
    (RuleSetId::Uc, 73, "Efreeti Touch", &["PREABILITY:4,CATEGORY=FEAT,Efreeti Style,Efreeti Stance,Elemental Fist,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:13],[PREVARGTEQ:MonkFeatQualify,11]", "PRESTAT:2,CON=15,WIS=17"]),
    (RuleSetId::Uc, 74, "Elusive Redirection", &["PREABILITY:1,CATEGORY=Special Ability,Elusive Target ~ Flowing Monk,Flowing TYPE.Elusive Target", "PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Flowing Monk", "PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Unarmed Strike", "PREVARGTEQ:MonkFeatQualify,12"]),
    (RuleSetId::Uc, 75, "Enfilading Fire", &["PREABILITY:2,CATEGORY=FEAT,Point-Blank Shot,Precise Shot", "PREABILITY:1,CATEGORY=FEAT,TYPE=Teamwork"]),
    (RuleSetId::Uc, 77, "Expert Driver", &["PREABILITY:1,CATEGORY=FEAT,Skilled Driver"]),
    (RuleSetId::Uc, 78, "Extra Bane", &["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Bane"]),
    (RuleSetId::Uc, 79, "Extra Grit", &["PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Gunslinger ~ Grit],[PREABILITY:1,CATEGORY=FEAT,Amateur Gunslinger]"]),
    (RuleSetId::Uc, 80, "False Opening", &["PREABILITY:2,CATEGORY=FEAT,Dodge,Weapon Focus (TYPE=Ranged)", "PREABILITY:1,CATEGORY=FEAT,Close-Quarters Thrower,Point Blank Master", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]"]),
    (RuleSetId::Uc, 81, "Feint Partner", &["PRESKILL:1,Bluff=1"]),
    (RuleSetId::Uc, 82, "Felling Escape", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Trip", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]"]),
    (RuleSetId::Uc, 83, "Felling Smash", &["PREABILITY:3,CATEGORY=FEAT,Combat Expertise,Improved Trip,Power Attack", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRESTAT:1,STR=13", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 84, "Feral Combat Training", &["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Weapon Focus (TYPE=Natural)"]),
    (RuleSetId::Uc, 85, "Field Repair", &["PRESKILL:1,Craft=4"]),
    (RuleSetId::Uc, 86, "Final Embrace", &["PREMULT:1,[PRERACE:1,Naga,Serpentfolk],[PREABILITY:1,CATEGORY=Special Ability,Constrict,TYPE.Constrict]", "PRESTAT:2,STR=13,INT=3", "PRETOTALAB:3"]),
    (RuleSetId::Uc, 87, "Final Embrace Horror", &["PREABILITY:1,CATEGORY=FEAT,Final Embrace", "PREMULT:1,[PRERACE:1,Naga,Serpentfolk],[PREABILITY:1,CATEGORY=Special Ability,Constrict]", "PRESTAT:2,STR=15,INT=3", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 88, "Final Embrace Master", &["PREABILITY:2,CATEGORY=FEAT,Final Embrace,Final Embrace Horror", "PREMULT:1,[PRERACE:1,Naga,Serpentfolk],[PREABILITY:1,CATEGORY=Special Ability,Constrict]", "PRESTAT:2,STR=17,INT=3", "PRETOTALAB:9"]),
    (RuleSetId::Uc, 90, "Fortified Armor Training", &["PREMULT:1,[PREPROFWITHSHIELD:1,TYPE.Tower,TYPE.Light,TYPE.Heavy],[PREPROFWITHARMOR:1,TYPE.Light,TYPE.Medium,TYPE.Heavy]"]),
    (RuleSetId::Uc, 91, "Furious Finish", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Rage", "PREABILITY:1,CATEGORY=FEAT,Vital Strike", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 92, "Gory Finish", &["PREABILITY:2,CATEGORY=FEAT,Dazzling Display,Weapon Focus"]),
    (RuleSetId::Uc, 93, "Greater Channel Smite", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Energy", "PREABILITY:1,CATEGORY=FEAT,Channel Smite", "PRETOTALAB:8"]),
    (RuleSetId::Uc, 94, "Greater Rending Fury", &["PREABILITY:1,CATEGORY=Special Ability,Rend", "PREABILITY:2,CATEGORY=FEAT,Improved Rending Fury,Rending Fury", "PRETOTALAB:12"]),
    (RuleSetId::Uc, 95, "Greater Snap Shot", &["PREABILITY:4,CATEGORY=FEAT,Improved Snap Shot,Point-Blank Shot,Rapid Shot,Snap Shot", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,17],[PREVARGTEQ:FeatDexRequirement,17]", "PRETOTALAB:12"]),
    (RuleSetId::Uc, 96, "Guided Hand", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Energy", "PREABILITY:1,CATEGORY=FEAT,Channel Smite", "PREWEAPONPROF:1,DEITYWEAPON"]),
    (RuleSetId::Uc, 98, "Hammer the Gap", &["PRETOTALAB:6"]),
    (RuleSetId::Uc, 99, "Harmonic Sage", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Bardic Performance", "PRESKILL:1,Knowledge (engineering)=5"]),
    (RuleSetId::Uc, 100, "Haunted Gnome", &["PREABILITY:1,CATEGORY=Special Ability,Gnome ~ Gnome Magic", "PRESKILL:1,Knowledge (arcana)=1", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Uc, 101, "Haunted Gnome Assault", &["PREABILITY:1,CATEGORY=Special Ability,Gnome ~ Gnome Magic", "PREABILITY:1,CATEGORY=FEAT,Haunted Gnome", "PRESKILL:1,Knowledge (arcana)=3", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Uc, 102, "Haunted Gnome Shroud", &["PREABILITY:1,CATEGORY=Special Ability,Gnome ~ Gnome Magic", "PREABILITY:2,CATEGORY=FEAT,Haunted Gnome,Haunted Gnome Assault", "PRESKILL:1,Knowledge (arcana)=6", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Uc, 103, "Hero's Display", &["PREABILITY:2,CATEGORY=FEAT,Dazzling Display,Weapon Focus"]),
    (RuleSetId::Uc, 104, "Hex Strike", &["PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,TYPE.WitchHex],[PREVARGTEQ:WitchMinorHexQualify,1],[PREVARGTEQ:WitchHexAbilityLVL,1]", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike"]),
    (RuleSetId::Uc, 105, "Horse Master", &["PREABILITY:1,CATEGORY=Special Ability,Cavalier ~ Expert Trainer", "PRESKILL:1,Ride=6"]),
    (RuleSetId::Uc, 106, "Impact Critical Shot", &["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRETOTALAB:9"]),
    (RuleSetId::Uc, 107, "Impaling Critical", &["PREABILITY:1,CATEGORY=FEAT,Critical Focus", "PREABILITY:1,CATEGORY=FEAT,Weapon Specialization (TYPE=MeleePiercing)", "PRETOTALAB:11"]),
    (RuleSetId::Uc, 108, "Improved Back to Back", &["PREABILITY:1,CATEGORY=FEAT,Back to Back", "PRESKILL:1,Perception=5"]),
    (RuleSetId::Uc, 109, "Improved Charging Hurler", &["PREABILITY:2,CATEGORY=FEAT,Charging Hurler,Point-Blank Shot"]),
    (RuleSetId::Uc, 110, "Improved Cleaving Finish", &["PREABILITY:4,CATEGORY=FEAT,Cleave,Cleaving Finish,Great Cleave,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 111, "Improved Devastating Strike", &["PREABILITY:2,CATEGORY=FEAT,Devastating Strike,Vital Strike", "PRETOTALAB:13"]),
    (RuleSetId::Uc, 112, "Improved Feint Partner", &["PREABILITY:2,CATEGORY=FEAT,Combat Reflexes,Feint Partner", "PRESKILL:1,Bluff=1", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 113, "Improved Impaling Critical", &["PREABILITY:3,CATEGORY=FEAT,Impaling Critical,Critical Focus,Weapon Specialization (TYPE=MeleePiercing)", "PRETOTALAB:13"]),
    (RuleSetId::Uc, 114, "Improved Rending Fury", &["PREABILITY:1,CATEGORY=Special Ability,Rend", "PREABILITY:1,CATEGORY=FEAT,Rending Fury", "PRETOTALAB:9"]),
    (RuleSetId::Uc, 115, "Improved Snap Shot", &["PREABILITY:4,CATEGORY=FEAT,Point-Blank Shot,Rapid Shot,Snap Shot,Weapon Focus", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,15],[PREVARGTEQ:FeatDexRequirement,15]", "PRETOTALAB:9"]),
    (RuleSetId::Uc, 116, "Improved Stalwart", &["PREABILITY:3,CATEGORY=FEAT,Diehard,Endurance,Stalwart", "PRETOTALAB:11"]),
    (RuleSetId::Uc, 117, "Improved Two-Weapon Feint", &["PREABILITY:4,CATEGORY=FEAT,Combat Expertise,Improved Two-Weapon Fighting,Two-Weapon Fighting,Two-Weapon Feint", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,17],[PREVARGTEQ:FeatDexRequirement,17]", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 118, "Instant Judgment", &["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Second Judgment"]),
    (RuleSetId::Uc, 119, "Intimidating Bane", &["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Bane", "PREABILITY:2,CATEGORY=FEAT,Dazzling Display,Weapon Focus", "PRELEVEL:MIN=8"]),
    (RuleSetId::Uc, 120, "Janni Rush", &["PREABILITY:3,CATEGORY=FEAT,Improved Unarmed Strike,Janni Style,Janni Tempest", "PRESKILL:2,Acrobatics=8,Perform (dance)=8"]),
    (RuleSetId::Uc, 121, "Janni Style", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRESKILL:2,Acrobatics=3,Perform (dance)=3"]),
    (RuleSetId::Uc, 122, "Janni Tempest", &["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Janni Style", "PRESKILL:2,Acrobatics=5,Perform (dance)=5"]),
    (RuleSetId::Uc, 123, "Kirin Path", &["PREABILITY:3,CATEGORY=FEAT,Kirin Strike,Kirin Style,Improved Unarmed Strike", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRESKILL:1,Knowledge (arcana)=12", "PRESKILL:1,Knowledge (dungeoneering)=5,Knowledge (local)=5,Knowledge (nature)=5,Knowledge (planes)=5,Knowledge (religion)=5"]),
    (RuleSetId::Uc, 124, "Kirin Strike", &["PREABILITY:2,CATEGORY=FEAT,Kirin Style,Improved Unarmed Strike", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRESKILL:1,Knowledge (arcana)=9", "PRESKILL:1,Knowledge (dungeoneering)=3,Knowledge (local)=3,Knowledge (nature)=3,Knowledge (planes)=3,Knowledge (religion)=3"]),
    (RuleSetId::Uc, 125, "Kirin Style", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRESKILL:1,Knowledge (arcana)=6", "PRESKILL:1,Knowledge (dungeoneering)=1,Knowledge (local)=1,Knowledge (nature)=1,Knowledge (planes)=1,Knowledge (religion)=1"]),
    (RuleSetId::Uc, 126, "Knockout Artist", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Sneak Attack", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike"]),
    (RuleSetId::Uc, 127, "Landing Roll", &["PREABILITY:2,CATEGORY=FEAT,Dodge,Mobility", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]"]),
    (RuleSetId::Uc, 128, "Leaping Shot Deed", &["PREABILITY:2,CATEGORY=FEAT,Dodge,Mobility", "PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Gunslinger ~ Grit],[PREABILITY:1,CATEGORY=FEAT,Amateur Gunslinger]", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRETOTALAB:4"]),
    (RuleSetId::Uc, 129, "Mantis Style", &["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Stunning Fist", "PRESKILL:1,Heal=3"]),
    (RuleSetId::Uc, 130, "Mantis Torment", &["PREABILITY:4,CATEGORY=FEAT,Improved Unarmed Strike,Mantis Style,Mantis Wisdom,Stunning Fist", "PRESKILL:1,Heal=9"]),
    (RuleSetId::Uc, 131, "Mantis Wisdom", &["PREABILITY:3,CATEGORY=FEAT,Improved Unarmed Strike,Mantis Style,Stunning Fist", "PRESKILL:1,Heal=6"]),
    (RuleSetId::Uc, 132, "Marid Coldsnap", &["PREABILITY:4,CATEGORY=FEAT,Elemental Fist,Marid Spirit,Marid Style,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:13],[PREVARGTEQ:MonkFeatQualify,11]", "PRESTAT:2,CON=15,WIS=17"]),
    (RuleSetId::Uc, 133, "Marid Spirit", &["PREABILITY:3,CATEGORY=FEAT,Elemental Fist,Marid Style,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:11],[PREVARGTEQ:MonkFeatQualify,9]", "PRESTAT:2,CON=15,WIS=15"]),
    (RuleSetId::Uc, 134, "Marid Style", &["PREABILITY:2,CATEGORY=FEAT,Elemental Fist,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:9],[PREVARGTEQ:MonkFeatQualify,5]", "PRESTAT:2,CON=13,WIS=15"]),
    (RuleSetId::Uc, 135, "Master Combat Performer", &["PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Performing Combatant],[PREABILITY:3,CATEGORY=FEAT,TYPE=Performance]", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 136, "Master Siege Engineer", &["PREABILITY:1,CATEGORY=FEAT,Siege Weapon Engineer", "PRESKILL:1,Knowledge (engineering)=10"]),
    (RuleSetId::Uc, 137, "Masterful Display", &["PREABILITY:1,CATEGORY=FEAT,Dazzling Display", "PREABILITY:2,CATEGORY=FEAT,TYPE=Performance"]),
    (RuleSetId::Uc, 138, "Maximized Spellstrike", &["PREABILITY:1,CATEGORY=Special Ability,Maximized Magic ~ Magus Arcana", "PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Samurai ~ Weapon Expertise],[PREABILITY:1,CATEGORY=FEAT,Quick Draw]"]),
    (RuleSetId::Uc, 139, "Menacing Bane", &["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Bane"]),
    (RuleSetId::Uc, 140, "Merciful Bane", &["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Bane"]),
    (RuleSetId::Uc, 141, "Mocking Dance", &["PRESKILL:1,Acrobatics=4,Perform (dance)=4"]),
    (RuleSetId::Uc, 142, "Monastic Legacy", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Still Mind", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike"]),
    (RuleSetId::Uc, 143, "Monkey Moves", &["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Monkey Style", "PRESKILL:2,Acrobatics=8,Climb=8", "PRESTAT:1,WIS=13"]),
    (RuleSetId::Uc, 144, "Monkey Shine", &["PREABILITY:4,CATEGORY=FEAT,Improved Unarmed Strike,Monkey Moves,Monkey Style,Stunning Fist", "PRESKILL:2,Acrobatics=11,Climb=11", "PRESTAT:1,WIS=13"]),
    (RuleSetId::Uc, 145, "Monkey Style", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRESKILL:2,Acrobatics=5,Climb=5", "PRESTAT:1,WIS=13"]),
    (RuleSetId::Uc, 146, "Murderer's Circle", &["PREABILITY:1,CATEGORY=FEAT,Dodge", "PRESKILL:1,Acrobatics=4"]),
    (RuleSetId::Uc, 147, "Neckbreaker", &["PREABILITY:6,CATEGORY=FEAT,Bonebreaker,Greater Grapple,Improved Grapple,Improved Unarmed Strike,Jawbreaker,Stunning Fist", "PRESKILL:1,Heal=12"]),
    (RuleSetId::Uc, 148, "Net Adept", &["PRETOTALAB:1", "PREWEAPONPROF:1,Net"]),
    (RuleSetId::Uc, 149, "Net Maneuvering", &["PREABILITY:1,CATEGORY=FEAT,Net Adept", "PRETOTALAB:3", "PREWEAPONPROF:1,Net"]),
    (RuleSetId::Uc, 150, "Net Trickery", &["PREABILITY:2,CATEGORY=FEAT,Net Adept,Net Maneuvering", "PRETOTALAB:6", "PREWEAPONPROF:1,Net"]),
    (RuleSetId::Uc, 151, "Net and Trident", &["PREABILITY:2,CATEGORY=FEAT,Net Adept,Two-Weapon Fighting", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,15],[PREVARGTEQ:FeatDexRequirement,15]", "PREWEAPONPROF:1,Net"]),
    (RuleSetId::Uc, 152, "Nightmare Fist", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PREMULT:1,[PRESPELL:1,Darkness,Deeper Darkness,Hungry Darkness],[PREABILITY:1,CATEGORY=Special Ability,Shadow Bloodline ~ Enveloping Darkness]", "PRESKILL:1,Intimidate=1"]),
    (RuleSetId::Uc, 153, "Nightmare Striker", &["PREABILITY:4,CATEGORY=FEAT,Improved Unarmed Strike,Nightmare Fist,Nightmare Weaver,Stunning Fist", "PRESKILL:1,Heal=5", "PRESPELL:1,Faerie Fire"]),
    (RuleSetId::Uc, 154, "Nightmare Weaver", &["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Nightmare Fist", "PRESKILL:1,Intimidate=2", "PRESPELL:1,Darkness"]),
    (RuleSetId::Uc, 155, "No Name", &["PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Gunslinger ~ Grit],[PREABILITY:1,CATEGORY=FEAT,Amateur Gunslinger]", "PRESKILL:1,Bluff=4"]),
    (RuleSetId::Uc, 157, "Pack Attack", &["PRETOTALAB:1"]),
    (RuleSetId::Uc, 158, "Panther Claw", &["PREABILITY:3,CATEGORY=FEAT,Combat Reflexes,Improved Unarmed Strike,Panther Style", "PRESTAT:1,WIS=15"]),
    (RuleSetId::Uc, 159, "Panther Parry", &["PREABILITY:4,CATEGORY=FEAT,Combat Reflexes,Improved Unarmed Strike,Panther Claw,Panther Style", "PRESTAT:1,WIS=15"]),
    (RuleSetId::Uc, 160, "Panther Style", &["PREABILITY:2,CATEGORY=FEAT,Combat Reflexes,Improved Unarmed Strike", "PRESTAT:1,WIS=13"]),
    (RuleSetId::Uc, 161, "Passing Trick", &["PREABILITY:4,CATEGORY=FEAT,Combat Expertise,Dodge,Improved Feint,Mobility", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRESIZELTEQ:S"]),
    (RuleSetId::Uc, 163, "Performing Combatant", &["PREABILITY:1,CATEGORY=FEAT,TYPE=Performance"]),
    (RuleSetId::Uc, 164, "Pin Down", &["PREABILITY:1,CATEGORY=FEAT,Combat Reflexes", "PREVARGTEQ:FighterWeaponQualifyLVL,11"]),
    (RuleSetId::Uc, 165, "Pinning Knockout", &["PREMULT:1,[PREMULT:3,[PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]],[PREABILITY:3,CATEGORY=FEAT,Greater Grapple,Improved Grapple,Improved Unarmed Strike],[PRETOTALAB:9]],[PREVARGTEQ:MonkFeatQualify,9]"]),
    (RuleSetId::Uc, 166, "Pinning Rend", &["PREMULT:1,[PREMULT:3,[PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]],[PREABILITY:3,CATEGORY=FEAT,Greater Grapple,Improved Grapple,Improved Unarmed Strike],[PRETOTALAB:9]],[PREVARGTEQ:MonkFeatQualify,9]"]),
    (RuleSetId::Uc, 167, "Pinpoint Poisoner", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Poison Use,Poison Use", "PREABILITY:2,CATEGORY=FEAT,Adder Strike,Improved Unarmed Strike", "PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Two-Weapon Fighting],[PREABILITY:1,CATEGORY=Special Ability,Flurry of Blows]", "PRESKILL:1,Craft (alchemy)=6"]),
    (RuleSetId::Uc, 168, "Planar Wild Shape", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape", "PRESKILL:1,Knowledge (planes)=5"]),
    (RuleSetId::Uc, 169, "Prone Shooter", &["PRETOTALAB:1"]),
    (RuleSetId::Uc, 171, "Quick Bull Rush", &["PREABILITY:2,CATEGORY=FEAT,Improved Bull Rush,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 172, "Quick Dirty Trick", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Dirty Trick", "PREMULT:1,[PREVARGTEQ:PreStatScore_INT,13],[PREVARGTEQ:CombatFeatIntRequirement,13],[PREVARGTEQ:DirtyTricksterIntQualify,1]", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 173, "Quick Drag", &["PREABILITY:2,CATEGORY=FEAT,Improved Drag,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 174, "Quick Reposition", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Reposition", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 175, "Quick Steal", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Steal", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 176, "Raging Brutality", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Rage", "PREABILITY:1,CATEGORY=FEAT,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:12"]),
    (RuleSetId::Uc, 177, "Raging Deathblow", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Greater Rage"]),
    (RuleSetId::Uc, 178, "Raging Hurler", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Rage", "PREABILITY:1,CATEGORY=FEAT,Throw Anything"]),
    (RuleSetId::Uc, 179, "Raging Throw", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Rage", "PREABILITY:2,CATEGORY=FEAT,Improved Bull Rush,Power Attack", "PRESTAT:2,STR=13,CON=13", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 180, "Rapid Grappler", &["PREMULT:1,[PREMULT:3,[PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]],[PREABILITY:3,CATEGORY=FEAT,Greater Grapple,Improved Grapple,Improved Unarmed Strike],[PRETOTALAB:9]],[PREVARGTEQ:MonkFeatQualify,9]"]),
    (RuleSetId::Uc, 181, "Rebounding Leap", &["PREABILITY:1,CATEGORY=Special Ability,Dragoon ~ Leaping Lance", "PRESKILL:2,Acrobatics=5,Ride=11"]),
    (RuleSetId::Uc, 182, "Rebuffing Reduction", &["PREABILITY:2,CATEGORY=FEAT,Improved Bull Rush,Power Attack", "PREDR:1,ANY=1", "PRESTAT:1,STR=13", "PRETOTALAB:1"]),
    (RuleSetId::Uc, 183, "Rending Fury", &["PREABILITY:1,CATEGORY=Special Ability,Rend", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 184, "Revelation Strike", &["PREABILITY:1,CATEGORY=Special Ability,Oracle's Mystery", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike"]),
    (RuleSetId::Uc, 185, "Rhetorical Flourish", &["PREABILITY:1,CATEGORY=FEAT,Persuasive", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Uc, 186, "Ricochet Shot Deed", &["PREABILITY:1,CATEGORY=FEAT,Blind-Fight", "PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Gunslinger ~ Grit],[PREABILITY:1,CATEGORY=FEAT,Amateur Gunslinger]"]),
    (RuleSetId::Uc, 187, "Righteous Healing", &["PREABILITY:1,CATEGORY=Special Ability,TYPE=InquisitorJudgment"]),
    (RuleSetId::Uc, 188, "Sap Adept", &["PREVARGTEQ:SneakAttackDice,1"]),
    (RuleSetId::Uc, 189, "Sap Master", &["PREVARGTEQ:SneakAttackDice,3"]),
    (RuleSetId::Uc, 190, "Savage Display", &["PREABILITY:1,CATEGORY=FEAT,Dazzling Display"]),
    (RuleSetId::Uc, 191, "School Strike", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.ArcaneSchool", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike"]),
    (RuleSetId::Uc, 192, "Sea Legs", &["PRESKILL:1,Profession (sailor)=5"]),
    (RuleSetId::Uc, 193, "Secret Stash Deed", &["PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Gunslinger ~ Grit],[PREABILITY:1,CATEGORY=FEAT,Amateur Gunslinger]", "PRESKILL:1,Sleight of Hand=1"]),
    (RuleSetId::Uc, 194, "Seize the Moment", &["PREABILITY:2,CATEGORY=FEAT,Combat Reflexes,Improved Critical"]),
    (RuleSetId::Uc, 195, "Shaitan Earthblast", &["PREMULT:1,[PREMULT:3,[PRESTAT:2,CON=15,WIS=17],[PREABILITY:4,CATEGORY=FEAT,Elemental Fist,Improved Unarmed Strike,Shaitan Skin,Shaitan Style],[PREMULT:1,[PRETOTALAB:13],[PREVARGTEQ:MonkFeatQualify,11]]]"]),
    (RuleSetId::Uc, 196, "Shaitan Skin", &["PREMULT:1,[PREMULT:3,[PRESTAT:2,CON=15,WIS=15],[PREABILITY:3,CATEGORY=FEAT,Elemental Fist,Improved Unarmed Strike,Shaitan Style],[PREMULT:1,[PRETOTALAB:11],[PREVARGTEQ:MonkFeatQualify,9]]]"]),
    (RuleSetId::Uc, 197, "Shaitan Style", &["PREABILITY:2,CATEGORY=FEAT,Elemental Fist,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:9],[PREVARGTEQ:MonkFeatQualify,5]", "PRESTAT:2,CON=13,WIS=15"]),
    (RuleSetId::Uc, 199, "Shapeshifter Foil", &["PREMULT:1,[PRESPELLSCHOOLSUB:1,Polymorph=1],[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ Shapeshifter],[PREABILITY:1,CATEGORY=Special Ability,Punitive Transformation ~ Waves Mystery]", "PRESKILL:1,Knowledge (arcana)=5,Knowledge (nature)=5"]),
    (RuleSetId::Uc, 200, "Shapeshifting Hunter", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.FavoredEnemy,TYPE.Favored Enemy", "PREABILITY:1,CATEGORY=Special Ability,Wild Shape"]),
    (RuleSetId::Uc, 201, "Signature Deed", &["PREABILITY:1,CATEGORY=Special Ability,Gunslinger ~ Grit,TYPE.GritShared", "PRECLASS:1,Gunslinger=11,Swashbuckler=11"]),
    (RuleSetId::Uc, 203, "Wave Strike", &["PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Samurai ~ Weapon Expertise],[PREABILITY:1,CATEGORY=FEAT,Quick Draw]", "PRESKILL:1,Bluff=1"]),
    (RuleSetId::Uc, 204, "Whip Mastery", &["PREABILITY:1,CATEGORY=FEAT,Weapon Focus (whip)", "PRETOTALAB:2"]),
    (RuleSetId::Uc, 205, "Improved Whip Mastery", &["PREABILITY:2,CATEGORY=FEAT,Weapon Focus (whip),Whip Mastery", "PRETOTALAB:5"]),
    (RuleSetId::Uc, 206, "Greater Whip Mastery", &["PREABILITY:3,CATEGORY=FEAT,Improved Whip Mastery,Weapon Focus (whip),Whip Mastery", "PRETOTALAB:8"]),
    (RuleSetId::Uc, 207, "Crane Style", &["PREABILITY:2,CATEGORY=FEAT,Dodge,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:2],[PREVARGTEQ:MonkFeatQualify,1]"]),
    (RuleSetId::Uc, 208, "Crane Riposte", &["PREABILITY:4,CATEGORY=FEAT,Crane Style,Crane Wing,Dodge,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:8],[PREVARGTEQ:MonkFeatQualify,7]"]),
    (RuleSetId::Uc, 209, "Crane Wing", &["PREABILITY:3,CATEGORY=FEAT,Crane Style,Dodge,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:5],[PREVARGTEQ:MonkFeatQualify,5]"]),
    (RuleSetId::Uc, 210, "Crusader's Fist", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Lay on Hands,Touch of Corruption ~ Antipaladin", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 211, "Crusader's Flurry", &["PREABILITY:2,CATEGORY=Special Ability,TYPE.Channel Energy,TYPE.Flurry of Blows", "PREABILITY:1,CATEGORY=FEAT,Weapon Focus", "PREWEAPONPROF:1,DEITYWEAPON"]),
    (RuleSetId::Uc, 212, "Dispelling Fist", &["PREMULT:1,[PRECLASS:1,SPELLCASTER=7],[PREVARGTEQ:CasterLevel_Highest,7]", "PRESPELL:1,Dispel Magic", "PRETOTALAB:11"]),
    (RuleSetId::Uc, 213, "Moonlight Stalker", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.RacialVision,TYPE.Racial Vision", "PREABILITY:1,CATEGORY=Special Ability,Darkvision,Low-Light Vision", "PREABILITY:2,CATEGORY=FEAT,Blind-Fight,Combat Expertise", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRESKILL:1,Bluff=3"]),
    (RuleSetId::Uc, 214, "Moonlight Stalker Feint", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.RacialVision,TYPE.Racial Vision", "PREABILITY:1,CATEGORY=Special Ability,Darkvision,Low-Light Vision", "PREABILITY:4,CATEGORY=FEAT,Blind-Fight,Combat Expertise,Moonlight Stalker,Improved Feint", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRESKILL:1,Bluff=6"]),
    (RuleSetId::Uc, 215, "Moonlight Stalker Master", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.RacialVision,TYPE.Racial Vision", "PREABILITY:1,CATEGORY=Special Ability,Darkvision,Low-Light Vision", "PREABILITY:5,CATEGORY=FEAT,Blind-Fight,Combat Expertise,Improved Feint,Moonlight Stalker,Moonlight Stalker Feint", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRESKILL:1,Bluff=9"]),
    (RuleSetId::Uc, 216, "Shared Judgment", &["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Second Judgment"]),
    (RuleSetId::Uc, 217, "Siege Commander", &["PRESKILL:1,Craft (siege weapon)=5", "PRESKILL:1,Knowledge (engineering)=5,Profession (siege engineer)=1"]),
    (RuleSetId::Uc, 218, "Siege Engineer", &["PREMULT:1,[PREWEAPONPROF:1,TYPE.SiegeWeapon],[PREABILITY:1,CATEGORY=FEAT,Exotic Weapon Proficiency (TYPE=SiegeWeapon)]", "PRESKILL:1,Knowledge (engineering)=5,Profession (siege engineer)=5"]),
    (RuleSetId::Uc, 219, "Siege Gunner", &["PREABILITY:1,CATEGORY=FEAT,Siege Engineer", "PRESKILL:1,Profession (siege engineer)=5"]),
    (RuleSetId::Uc, 220, "Slayer's Knack", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.FavoredEnemy,TYPE.Favored Enemy", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 221, "Sling Flail", &["PREABILITY:1,CATEGORY=FEAT,Weapon Focus (sling)", "PRETOTALAB:1"]),
    (RuleSetId::Uc, 222, "Snake Fang", &["PREABILITY:4,CATEGORY=FEAT,Combat Reflexes,Improved Unarmed Strike,Snake Sidewind,Snake Style", "PRESKILL:2,Acrobatics=6,Sense Motive=9"]),
    (RuleSetId::Uc, 223, "Snake Sidewind", &["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Snake Style", "PRESKILL:2,Acrobatics=3,Sense Motive=6"]),
    (RuleSetId::Uc, 224, "Snake Style", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRESKILL:2,Acrobatics=1,Sense Motive=3"]),
    (RuleSetId::Uc, 225, "Snapping Turtle Clutch", &["PREABILITY:3,CATEGORY=FEAT,Snapping Turtle Style,Improved Grapple,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:3],[PREVARGTEQ:MonkFeatQualify,3]"]),
    (RuleSetId::Uc, 226, "Snapping Turtle Shell", &["PREABILITY:4,CATEGORY=FEAT,Snapping Turtle Clutch,Snapping Turtle Style,Improved Grapple,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:5],[PREVARGTEQ:MonkFeatQualify,5]"]),
    (RuleSetId::Uc, 227, "Snapping Turtle Style", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:1],[PREVARGTEQ:MonkFeatQualify,1]"]),
    (RuleSetId::Uc, 228, "Sneaking Precision", &["PREABILITY:2,CATEGORY=FEAT,Critical Focus,TYPE=Critical", "PRETOTALAB:9", "PREVARGTEQ:SneakAttackDice,6"]),
    (RuleSetId::Uc, 229, "Sorcerous Strike", &["PREABILITY:1,CATEGORY=Special Ability,Sorcerer ~ Standard Bloodline", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike"]),
    (RuleSetId::Uc, 230, "Spell Bane", &["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Bane"]),
    (RuleSetId::Uc, 231, "Spinning Throw", &["PREABILITY:5,CATEGORY=FEAT,Combat Expertise,Improved Bull Rush,Improved Trip,Improved Unarmed Strike,Ki Throw"]),
    (RuleSetId::Uc, 232, "Splintering Weapon", &["PRETOTALAB:1"]),
    (RuleSetId::Uc, 233, "Stage Combatant", &["PREABILITY:1,CATEGORY=FEAT,Weapon Focus", "PRETOTALAB:5"]),
    (RuleSetId::Uc, 234, "Stalwart", &["PREABILITY:2,CATEGORY=FEAT,Diehard,Endurance", "PRETOTALAB:4"]),
    (RuleSetId::Uc, 236, "Strangler", &["PREABILITY:2,CATEGORY=FEAT,Improved Grapple,Improved Unarmed Strike", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PREVARGTEQ:SneakAttackDice,1"]),
    (RuleSetId::Uc, 238, "Stunning Pin", &["PREABILITY:3,CATEGORY=FEAT,Improved Grapple,Improved Unarmed Strike,Stunning Fist"]),
    (RuleSetId::Uc, 239, "Sure Grasp", &["PRESKILL:1,Climb=1"]),
    (RuleSetId::Uc, 240, "Snap Shot", &["PREABILITY:3,CATEGORY=FEAT,Point-Blank Shot,Weapon Focus,Rapid Shot", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 241, "Sword and Pistol", &["PREABILITY:4,CATEGORY=FEAT,Two-Weapon Fighting,Point-Blank Shot,Rapid Shot,Snap Shot", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 243, "Target of Opportunity", &["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 244, "Team Pickpocketing", &["PRESKILL:2,Bluff=1,Sleight of Hand=1"]),
    (RuleSetId::Uc, 245, "Tiger Claws", &["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Tiger Style", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:MonkFeatQualify,5]"]),
    (RuleSetId::Uc, 246, "Tiger Pounce", &["PREABILITY:4,CATEGORY=FEAT,Improved Unarmed Strike,Power Attack,Tiger Style,Tiger Claws", "PREMULT:1,[PRETOTALAB:9],[PREVARGTEQ:MonkFeatQualify,8]"]),
    (RuleSetId::Uc, 247, "Tiger Style", &["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:3],[PREVARGTEQ:MonkFeatQualify,3]"]),
    (RuleSetId::Uc, 248, "Trapper's Setup", &["PRESKILL:1,Craft (traps)=5"]),
    (RuleSetId::Uc, 249, "Twin Thunders", &["PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Defensive Training,Gnome ~ Defensive Training", "PREABILITY:1,CATEGORY=FEAT,Weapon Focus", "PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Two-Weapon Fighting],[PREABILITY:1,CATEGORY=Special Ability,Flurry of Blows]", "PREFACT:1,TEMPLATES,IsDwarf=true,IsGnome=True"]),
    (RuleSetId::Uc, 250, "Twin Thunders Flurry", &["PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Defensive Training,Gnome ~ Defensive Training", "PREABILITY:2,CATEGORY=FEAT,Twin Thunders,Weapon Focus", "PREMULT:1,[PREABILITY:2,CATEGORY=FEAT,Improved Two-Weapon Fighting,Two-Weapon Fighting],[PREABILITY:1,CATEGORY=Special Ability,Flurry of Blows]", "PREFACT:1,TEMPLATES,IsDwarf=true,IsGnome=True", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 251, "Twin Thunders Master", &["PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Defensive Training,Gnome ~ Defensive Training", "PREABILITY:3,CATEGORY=FEAT,Twin Thunders,Twin Thunders Flurry,Weapon Focus", "PREMULT:1,[PREABILITY:2,CATEGORY=FEAT,Improved Two-Weapon Fighting,Two-Weapon Fighting],[PREABILITY:1,CATEGORY=Special Ability,Flurry of Blows]", "PREFACT:1,TEMPLATES,IsDwarf=true,IsGnome=True", "PRETOTALAB:9"]),
    (RuleSetId::Uc, 252, "Two-Handed Thrower", &["PRESTAT:1,STR=15"]),
    (RuleSetId::Uc, 253, "Two-Weapon Feint", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Two-Weapon Fighting", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,15],[PREVARGTEQ:FeatDexRequirement,15]"]),
    (RuleSetId::Uc, 254, "Vicious Stomp", &["PREABILITY:2,CATEGORY=FEAT,Combat Reflexes,Improved Unarmed Strike"]),
    (RuleSetId::Uc, 255, "Deathless Initiate (Vigor/Wounds)", &["PREABILITY:2,CATEGORY=FEAT,Diehard (Vigor/Wound),Endurance", "PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true", "PRERULE:1,DAMAGE_VW", "PRESTAT:2,STR=13,CON=13", "PRETOTALAB:6"]),
    (RuleSetId::Uc, 256, "Diehard (Vigor/Wounds)", &["PRERULE:1,DAMAGE_VW"]),
    (RuleSetId::Uc, 257, "Toughness (Vigor/Wounds)", &["PRERULE:1,DAMAGE_VW"]),
    (RuleSetId::Uc, 258, "Improved Called Shot", &["PREABILITY:1,CATEGORY=FEAT,Combat Expertise", "PREVARGTEQ:PreStatScore_INT,13"]),
    (RuleSetId::Uc, 259, "Greater Called Shot", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Called Shot", "PRETOTALAB:6", "PREVARGTEQ:PreStatScore_INT,13"]),
    (RuleSetId::Uc, 260, "Style Feat Wildcard", &["PRECLASS:1,Monk=6", "PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Master Of Many Styles"]),
    // Ultimate Magic — 135 of 144 record(s) in the hand-authored table carry at least one `PRE`-family token.
    (RuleSetId::Um, 0, "Abundant Revelations", &["PREABILITY:1,CATEGORY=Special Ability,Oracle's Mystery"]),
    (RuleSetId::Um, 1, "Accursed Critical", &["PREABILITY:1,CATEGORY=FEAT,Critical Focus", "PREMULT:1,[PRECLASS:1,SPELLCASTER=9],[PREVARGTEQ:CasterLevel_Highest,9]", "PRESPELL:1,Bestow Curse,Major Curse"]),
    (RuleSetId::Um, 2, "Accursed Hex", &["PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,TYPE.WitchHex],[PREVARGTEQ:WitchMinorHexQualify,1],[PREVARGTEQ:WitchHexAbilityLVL,1]"]),
    (RuleSetId::Um, 3, "Advanced Ranger Trap", &["PREABILITY:1,CATEGORY=Special Ability,Trapper ~ Trap", "PRECLASS:1,Ranger=5"]),
    (RuleSetId::Um, 5, "Blighted Critical", &["PREMULT:1,[PRECLASS:1,SPELLCASTER=5],[PREVARGTEQ:CasterLevel_Highest,5]"]),
    (RuleSetId::Um, 6, "Blighted Critical Mastery", &["PREABILITY:1,CATEGORY=FEAT,Blighted Critical", "PREMULT:1,[PRECLASS:1,SPELLCASTER=9],[PREVARGTEQ:CasterLevel_Highest,9]"]),
    (RuleSetId::Um, 8, "Channeled Shield Wall", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Energy", "PREMULT:1,[PREVARGTEQ:ClericChannelEnergyLVL,5],[PREVARGTEQ:PaladinChannelLVL,5],[PREVARGTEQ:OracleChannelLVL,5]", "PREMULT:1,[PREVARGTEQ:OracleChannelDieSize,6],[PREVARGTEQ:ClericChannelPositiveEnergyDieSize,6],[PREVARGTEQ:ClericChannelNegativeEnergyDieSize,6],[PREVARGTEQ:PaladinChannelDieSize,6],[PREVARGTEQ:ClassChannelPositiveEnergyDieSize,6],[PREVARGTEQ:ClassChannelNegativeEnergyDieSize,6]", "PREPROFWITHSHIELD:1,TYPE.Light,TYPE.Heavy"]),
    (RuleSetId::Um, 10, "Create Reliquary Arms and Shields", &["PREABILITY:1,CATEGORY=FEAT,Craft Magic Arms and Armor", "PRESPELL:1,Consecrate,Desecrate"]),
    (RuleSetId::Um, 11, "Create Sanguine Elixir", &["PREABILITY:1,CATEGORY=FEAT,Brew Potion", "PRECLASS:1,Sorcerer=3", "PRESKILL:1,Craft (Alchemy)=12", "PRESTAT:1,CHA=15"]),
    (RuleSetId::Um, 12, "Defending Eidolon", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Shield Ally"]),
    (RuleSetId::Um, 13, "Deny Death", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Ki Pool", "PREABILITY:1,CATEGORY=FEAT,Endurance"]),
    (RuleSetId::Um, 14, "Detect Expertise", &["PRESPELL:1,Detect Chaos,Detect Evil,Detect Good,Detect Law,Detect Magic", "PRESTAT:1,INT=13"]),
    (RuleSetId::Um, 15, "Die for Your Master", &["PREABILITY:1,CATEGORY=Special Ability,Discovery ~ Tumor Familiar"]),
    (RuleSetId::Um, 16, "Divine Interference", &["PREMULT:1,[PRECLASS:1,SPELLCASTER.Divine=10],[PREVARGTEQ:Caster_Level_Highest__Divine,10]"]),
    (RuleSetId::Um, 17, "Dragonbane Aura", &["PREABILITY:1,CATEGORY=Special Ability,Paladin ~ Aura of Courage", "PREMULT:1,[PRECLASS:1,SPELLCASTER=8],[PREVARGTEQ:CasterLevel_Highest,8]"]),
    (RuleSetId::Um, 19, "Eldritch Heritage", &["PREABILITY:1,CATEGORY=FEAT,Skill Focus", "PREPCLEVEL:MIN=3", "PREVARGTEQ:CHASCORE,EldritchHeritageCharismaPrerequisite", "PREVARLT:count(\"ABILITIES\",\"CATEGORY=FEAT\",\"KEY=Eldritch Heritage\"),1"]),
    (RuleSetId::Um, 20, "Ensemble", &["PRESKILL:1,TYPE.Perform=5"]),
    (RuleSetId::Um, 21, "Evolved Familiar", &["PRESTAT:2,INT=13,CHA=13", "PREVARGTEQ:FamiliarLVL,1"]),
    (RuleSetId::Um, 22, "Exploit Lore", &["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Monster Lore", "PRETOTALAB:11"]),
    (RuleSetId::Um, 23, "Extra Arcana", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Magus Arcana"]),
    (RuleSetId::Um, 24, "Extra Arcane Pool", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Magus Arcane Pool"]),
    (RuleSetId::Um, 25, "Extended Bane", &["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Bane"]),
    (RuleSetId::Um, 26, "Extra Cantrips or Orisons", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.CantripsOrisons"]),
    (RuleSetId::Um, 27, "Extra Evolution", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Eidolon", "PREVARGTEQ:TL/5,Feat_Extra_Evolution_Count"]),
    (RuleSetId::Um, 28, "Extra Ranger Trap", &["PREABILITY:1,CATEGORY=Special Ability,Trapper ~ Trap"]),
    (RuleSetId::Um, 29, "Extra Summons", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Summon Monster SLA", "PRECLASS:1,Summoner=1", "PRETEXT:Ability to cast summon monster as a spelllike ability, summoner 1st.", "PREVARLTEQ:Feat_Extra_Summons_Taken,Feat_Extra_Summons_Allowed"]),
    (RuleSetId::Um, 30, "Eyes of Judgment", &["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Detect Alignment", "PREMULT:1,[PRECLASS:1,SPELLCASTER=6],[PREVARGTEQ:CasterLevel_Highest,6]"]),
    (RuleSetId::Um, 31, "Fast Empathy", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Wild Empathy", "PRESKILL:1,Handle Animal=5"]),
    (RuleSetId::Um, 32, "Favored Judgment", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.InquisitorJudgment", "PRESTAT:1,WIS=13"]),
    (RuleSetId::Um, 33, "Fearless Aura", &["PREABILITY:1,CATEGORY=Special Ability,Paladin ~ Aura of Courage", "PREMULT:1,[PRECLASS:1,SPELLCASTER=8],[PREVARGTEQ:CasterLevel_Highest,8]"]),
    (RuleSetId::Um, 34, "Fire Music", &["PRESKILL:1,Spellcraft=5"]),
    (RuleSetId::Um, 36, "Focused Eidolon", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Shield Ally"]),
    (RuleSetId::Um, 37, "Gliding Steps", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Ki Pool", "PREABILITY:3,CATEGORY=FEAT,Dodge,Mobility,Nimble Moves"]),
    (RuleSetId::Um, 38, "Grant Initiative", &["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Cunning Initiative"]),
    (RuleSetId::Um, 39, "Greater Blighted Critical", &["PREABILITY:1,CATEGORY=FEAT,Blighted Critical", "PREMULT:1,[PRECLASS:1,SPELLCASTER=12],[PREVARGTEQ:CasterLevel_Highest,12]"]),
    (RuleSetId::Um, 40, "Greater Eldritch Heritage", &["PREABILITY:2,CATEGORY=FEAT,Eldritch Heritage,Improved Eldritch Heritage", "PRELEVEL:MIN=17", "PREVARGTEQ:CHASCORE,GreaterEldritchHeritageCharismaPrerequisite"]),
    (RuleSetId::Um, 41, "Greater Mercy", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Lay on Hands", "PREABILITY:1,CATEGORY=Special Ability,TYPE.Mercy", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Um, 42, "Greater Spell Specialization", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus", "PREABILITY:1,CATEGORY=FEAT,TYPE.SpellSpecialization", "PRESPELLTYPE:1,Arcane=5,Divine=5", "PRESTAT:1,INT=13"]),
    (RuleSetId::Um, 43, "Greater Wild Empathy", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Wild Empathy", "PRESKILL:1,Knowledge (Nature)=5"]),
    (RuleSetId::Um, 44, "Implant Bomb", &["PREABILITY:1,CATEGORY=Special Ability,Discovery ~ Delayed Bomb", "PRESKILL:1,Heal=5"]),
    (RuleSetId::Um, 45, "Improved Eldritch Heritage", &["PREABILITY:1,CATEGORY=FEAT,Eldritch Heritage", "PRELEVEL:MIN=11", "PREVARGTEQ:CHASCORE,ImprovedEldritchHeritageCharismaPrerequisite"]),
    (RuleSetId::Um, 46, "Improved Monster Lore", &["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Monster Lore"]),
    (RuleSetId::Um, 47, "Insightful Gaze", &["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Stern Gaze", "PRESKILL:1,Sense Motive=5"]),
    (RuleSetId::Um, 48, "Intimidating Gaze", &["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Stern Gaze", "PRESKILL:1,Intimidate=5", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Um, 49, "Judgment Surge", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.InquisitorJudgment"]),
    (RuleSetId::Um, 50, "Ki Stand", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Ki Pool"]),
    (RuleSetId::Um, 51, "Learn Ranger Trap", &["PRESKILL:1,Survival=5"]),
    (RuleSetId::Um, 52, "Life Lure", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Positive Energy"]),
    (RuleSetId::Um, 53, "Moonlight Summons", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Conjuration)", "PRESPELL:1,Summon Nature's Ally I,Summon Nature's Ally II,Summon Nature's Ally III,Summon Nature's Ally IV,Summon Nature's Ally V,Summon Nature's Ally VI,Summon Nature's Ally VII,Summon Nature's Ally VIII,Summon Nature's Ally IX"]),
    (RuleSetId::Um, 54, "Mystic Stride", &["PREABILITY:1,CATEGORY=Special Ability,Woodland Stride", "PREABILITY:1,CATEGORY=FEAT,Nimble Moves", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,15],[PREVARGTEQ:FeatDexRequirement,15]"]),
    (RuleSetId::Um, 55, "Oracular Intuition", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.OracleMystery"]),
    (RuleSetId::Um, 56, "Painful Anchor", &["PREABILITY:1,CATEGORY=Special Ability,Anchoring Aura ~ Oath against Fiends"]),
    (RuleSetId::Um, 58, "Planar Preservationist", &["PREABILITY:1,CATEGORY=Archetype,Alchemist Archetype ~ Preservationist"]),
    (RuleSetId::Um, 59, "Powerful Shape", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape", "PREVARGTEQ:DruidLVL,8"]),
    (RuleSetId::Um, 61, "Prophetic Visionary", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.OracleMystery"]),
    (RuleSetId::Um, 62, "Pure Faith", &["PREABILITY:1,CATEGORY=Special Ability,Paladin ~ Divine Health"]),
    (RuleSetId::Um, 63, "Quarterstaff Master", &["PREABILITY:1,CATEGORY=FEAT,Weapon Focus (Quarterstaff)", "PRETOTALAB:5"]),
    (RuleSetId::Um, 64, "Quick Channel", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Energy", "PRESKILL:1,Knowledge (Religion)=5"]),
    (RuleSetId::Um, 65, "Quick Wild Shape", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape", "PREMULT:1,[PRECLASS:1,SPELLCASTER=8],[PREVARGTEQ:CasterLevel_Highest,8]"]),
    (RuleSetId::Um, 66, "Radiant Charge", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Lay On Hands"]),
    (RuleSetId::Um, 67, "Remote Bomb", &["PREABILITY:1,CATEGORY=Special Ability,Discovery ~ Delayed Bomb"]),
    (RuleSetId::Um, 68, "Resilient Eidolon", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Eidolon"]),
    (RuleSetId::Um, 69, "Reward of Grace", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Lay On Hands"]),
    (RuleSetId::Um, 70, "Reward of Life", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Lay On Hands"]),
    (RuleSetId::Um, 71, "Ricochet Splash Weapon", &["PREABILITY:1,CATEGORY=FEAT,Throw Anything", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]"]),
    (RuleSetId::Um, 73, "Sacred Summons", &["PREABILITY:1,CATEGORY=Special Ability,Aura of Chaos,Aura of Evil,Aura of Good,Aura of Law", "PRESPELL:1,Summon Monster I,Summon Monster II,Summon Monster III,Summon Monster IV,Summon Monster V,Summon Monster VI,Summon Monster VII,Summon Monster VIII,Summon Monster IX"]),
    (RuleSetId::Um, 74, "Sense Link", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Bond Senses"]),
    (RuleSetId::Um, 75, "Shaping Focus", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape", "PRESKILL:1,Knowledge (Nature)=5"]),
    (RuleSetId::Um, 76, "Sin Seer", &["PREABILITY:1,CATEGORY=Special Ability,Detect Undead ~ Oath against Undeath"]),
    (RuleSetId::Um, 77, "Skeleton Summoner", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Necromancy)", "PRESPELL:1,Summon Monster I,Summon Monster II,Summon Monster III,Summon Monster IV,Summon Monster V,Summon Monster VI,Summon Monster VII,Summon Monster VIII,Summon Monster IX"]),
    (RuleSetId::Um, 78, "Sorcerous Bloodstrike", &["PREABILITY:1,CATEGORY=Special Ability,Sorcerer ~ Standard Bloodline", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Um, 79, "Spell Bluff", &["PRESKILL:2,Bluff=5,Spellcraft=5"]),
    (RuleSetId::Um, 80, "Spell Hex", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.MajorHex", "PREVARGTEQ:WitchLVL,10"]),
    (RuleSetId::Um, 81, "Spell Specialization (Abjuration)", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Abjuration)", "PRESTAT:1,INT=13"]),
    (RuleSetId::Um, 82, "Spell Specialization (Conjuration)", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Conjuration)", "PRESTAT:1,INT=13"]),
    (RuleSetId::Um, 83, "Spell Specialization (Divination)", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Divination)", "PRESTAT:1,INT=13"]),
    (RuleSetId::Um, 84, "Spell Specialization (Enchantment)", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Enchantment)", "PRESTAT:1,INT=13"]),
    (RuleSetId::Um, 85, "Spell Specialization (Evocation)", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Evocation)", "PRESTAT:1,INT=13"]),
    (RuleSetId::Um, 86, "Spell Specialization (Illusion)", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Illusion)", "PRESTAT:1,INT=13"]),
    (RuleSetId::Um, 87, "Spell Specialization (Necromancy)", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Necromancy)", "PRESTAT:1,INT=13"]),
    (RuleSetId::Um, 88, "Spell Specialization (Transmutation)", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Transmutation)", "PRESTAT:1,INT=13"]),
    (RuleSetId::Um, 89, "Spellsong", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Bardic Performance", "PRESPELLTYPE:1,Arcane=1,Divine=1", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Um, 90, "Split Hex", &["PREVARGTEQ:WitchLVL,10"]),
    (RuleSetId::Um, 91, "Split Major Hex", &["PREABILITY:1,CATEGORY=FEAT,Split Hex", "PREMULT:1,[PRECLASS:1,SPELLCASTER=18],[PREVARGTEQ:CasterLevel_Highest,18]"]),
    (RuleSetId::Um, 92, "Spontaneous Metafocus", &["PREABILITY:1,CATEGORY=FEAT,TYPE.Metamagic", "PRESTAT:1,CHA=13"]),
    (RuleSetId::Um, 93, "Starlight Summons", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Conjuration)", "PRESPELL:1,Summon Nature's Ally I,Summon Nature's Ally II,Summon Nature's Ally III,Summon Nature's Ally IV,Summon Nature's Ally V,Summon Nature's Ally VI,Summon Nature's Ally VII,Summon Nature's Ally VIII,Summon Nature's Ally IX"]),
    (RuleSetId::Um, 94, "Sunlight Summons", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Conjuration)", "PRESPELL:1,Summon Nature's Ally I,Summon Nature's Ally II,Summon Nature's Ally III,Summon Nature's Ally IV,Summon Nature's Ally V,Summon Nature's Ally VI,Summon Nature's Ally VII,Summon Nature's Ally VIII,Summon Nature's Ally IX"]),
    (RuleSetId::Um, 95, "Superior Summoning", &["PREABILITY:1,CATEGORY=FEAT,Augment Summoning", "PREMULT:1,[PRECLASS:1,SPELLCASTER=3],[PREVARGTEQ:CasterLevel_Highest,3]"]),
    (RuleSetId::Um, 96, "Thanatopic Spell", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Necromancy)", "PRESKILL:1,Knowledge (Religion)=6"]),
    (RuleSetId::Um, 97, "Theurgy", &["PRESPELLTYPE:1,Arcane=1", "PRESPELLTYPE:1,Divine=1", "PRESTAT:1,WIS=13", "PRESTAT:1,INT=13,CHA=13"]),
    (RuleSetId::Um, 98, "Thoughtful Discernment", &["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Discern Lies"]),
    (RuleSetId::Um, 99, "Threnodic Spell", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Necromancy)", "PRESKILL:1,Knowledge (Religion)=6"]),
    (RuleSetId::Um, 101, "Tripping Staff", &["PREABILITY:3,CATEGORY=FEAT,Combat Expertise,Improved Trip,Weapon Focus (Quarterstaff)", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:6"]),
    (RuleSetId::Um, 102, "Tripping Twirl", &["PREABILITY:5,CATEGORY=FEAT,Combat Expertise,Improved Trip,Tripping Staff,Weapon Focus (Quarterstaff),Weapon Specialization (Quarterstaff)", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:12"]),
    (RuleSetId::Um, 103, "Ultimate Mercy", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Lay on Hands", "PREABILITY:1,CATEGORY=Special Ability,TYPE.Mercy", "PREABILITY:1,CATEGORY=FEAT,Greater Mercy", "PRESTAT:1,CHA=19"]),
    (RuleSetId::Um, 104, "Ultimate Resolve", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Aura of Resolve"]),
    (RuleSetId::Um, 105, "Uncanny Alertness", &["PREABILITY:1,CATEGORY=FEAT,Alertness"]),
    (RuleSetId::Um, 106, "Uncanny Concentration", &["PREABILITY:1,CATEGORY=FEAT,Combat Casting"]),
    (RuleSetId::Um, 107, "Undead Master", &["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Necromancy)", "PRESPELL:1,Animate Dead,Command Undead"]),
    (RuleSetId::Um, 108, "Unsanctioned Detection", &["PREABILITY:1,CATEGORY=Special Ability,Paladin ~ Detect Evil"]),
    (RuleSetId::Um, 109, "Unsanctioned Knowledge", &["PRECLASS:1,Paladin=4", "PRESTAT:1,INT=13"]),
    (RuleSetId::Um, 110, "Versatile Channeler", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Negative Energy,TYPE.Channel Positive Energy", "PREMULT:1,[PREMULT:2,[PREALIGN:LN,TN,CN],[PREMULT:1,[PREDEITYALIGN:LN,TN,CN],[PREDEITY:1,None]]],[PREVARGTEQ:NecromancySchoolLVL,1]"]),
    (RuleSetId::Um, 111, "Vigilant Eidolon", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Eidolon"]),
    (RuleSetId::Um, 112, "Voice of the Sibyl", &["PRESTAT:1,CHA=15"]),
    (RuleSetId::Um, 113, "Warrior Priest", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.CF_Domain", "PRESPELLCAST:TYPE=Divine"]),
    (RuleSetId::Um, 114, "Wild Speech", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape", "PREVARGTEQ:DruidLVL,6"]),
    (RuleSetId::Um, 115, "Witch Knife", &["PRECLASS:1,Witch=1"]),
    (RuleSetId::Um, 116, "Word of Healing", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Lay On Hands"]),
    (RuleSetId::Um, 117, "Masterpiece (At the Heart of It All)", &["PRESKILL:1,Perform (String Instruments)=7,Perform (Wind Instruments)=7", "PREVARGTEQ:MasterpieceLVL,1"]),
    (RuleSetId::Um, 118, "Masterpiece (Cat-Step)", &["PRESKILL:1,Perform (Dance)=5", "PREVARGTEQ:MasterpieceLVL,1"]),
    (RuleSetId::Um, 119, "Masterpiece (Dance of 23 Steps)", &["PRESKILL:1,Perform (Dance)=4", "PREVARGTEQ:MasterpieceLVL,1"]),
    (RuleSetId::Um, 120, "Masterpiece (Depths of the Mountain)", &["PRESKILL:1,Perform (Percussion Instruments)=15,Perform (Wind Instruments)=15", "PREVARGTEQ:MasterpieceLVL,1"]),
    (RuleSetId::Um, 121, "Masterpiece (Dumbshow of Gorroc)", &["PRESKILL:1,Perform (Act)=6,Perform (Comedy)=6", "PREVARGTEQ:MasterpieceLVL,1"]),
    (RuleSetId::Um, 122, "Masterpiece (House of Imaginary Walls)", &["PRESKILL:1,Perform (Act)=10", "PREVARGTEQ:MasterpieceLVL,1"]),
    (RuleSetId::Um, 123, "Masterpiece (Legato Piece on the Infernal Bargain)", &["PRESKILL:1,Perform (String Instruments)=11", "PREVARGTEQ:MasterpieceLVL,1"]),
    (RuleSetId::Um, 124, "Masterpiece (Lullaby of Ember the Ancient)", &["PRESKILL:1,Perform (Sing)=7", "PREVARGTEQ:MasterpieceLVL,1"]),
    (RuleSetId::Um, 125, "Masterpiece (Minuet of the Midnight Ivy)", &["PRESKILL:1,Perform (Dance)=4", "PREVARGTEQ:MasterpieceLVL,1"]),
    (RuleSetId::Um, 126, "Masterpiece (Quickening Pulse)", &["PRESKILL:1,Perform (Percussion Instruments)=7,Perform (Wind Instruments)=7", "PREVARGTEQ:MasterpieceLVL,1"]),
    (RuleSetId::Um, 127, "Masterpiece (Requiem of the Fallen Priest-King)", &["PRESKILL:1,Perform (Oratory)=10,Perform (Sing)=10", "PREVARGTEQ:MasterpieceLVL,1"]),
    (RuleSetId::Um, 128, "Masterpiece (Stone Face)", &["PRESKILL:1,Perform (Comedy)=7,Perform (Oratory)=7", "PREVARGTEQ:MasterpieceLVL,1"]),
    (RuleSetId::Um, 129, "Masterpiece (Toccata and Fugue of the Danse Macabre)", &["PRESKILL:1,Perform (Keyboard Instruments)=4,Perform (Wind Instruments)=4", "PREVARGTEQ:MasterpieceLVL,1"]),
    (RuleSetId::Um, 130, "Masterpiece (Triple Time)", &["PRESKILL:1,Perform (Percussion Instruments)=3,Perform (String Instruments)=3,Perform (Wind Instruments)=3", "PREVARGTEQ:MasterpieceLVL,1"]),
    (RuleSetId::Um, 131, "Masterpiece (Winds of the Five Heavens)", &["PRESKILL:1,Perform (Act)=10,Perform (Oratory)=10", "PREVARGTEQ:MasterpieceLVL,1"]),
    (RuleSetId::Um, 132, "Transfer Feat to Familiar", &["PREABILITY:1,CATEGORY=Special Ability,Transfer Feats ~ Beast-Bonded", "PRECLASS:1,Witch=1"]),
    (RuleSetId::Um, 133, "Discovery (Arcane Builder)", &["PREVARGTEQ:FeatQualifier_WizardLVL,1"]),
    (RuleSetId::Um, 134, "Discovery (Fast Study)", &["PREVARGTEQ:FeatQualifier_WizardLVL,5"]),
    (RuleSetId::Um, 135, "Discovery (Feral Speech)", &["PREVARGTEQ:FeatQualifier_WizardLVL,5"]),
    (RuleSetId::Um, 136, "Discovery (Golem Constructor)", &["PREVARGTEQ:FeatQualifier_WizardLVL,9"]),
    (RuleSetId::Um, 137, "Discovery (Immortality)", &["PREVARGTEQ:FeatQualifier_WizardLVL,20"]),
    (RuleSetId::Um, 138, "Discovery (Multimorph)", &["PREVARGTEQ:FeatQualifier_WizardLVL,5"]),
    (RuleSetId::Um, 139, "Discovery (Opposition Research)", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.ArcaneSchool", "PREVARGTEQ:FeatQualifier_WizardLVL,9"]),
    (RuleSetId::Um, 140, "Discovery (Split Slot)", &["PREVARGTEQ:FeatQualifier_WizardLVL,5"]),
    (RuleSetId::Um, 141, "Discovery (True Name)", &["PREVARGTEQ:FeatQualifier_WizardLVL,11"]),
    (RuleSetId::Um, 142, "Discovery (Greater True Name)", &["PREVARGTEQ:FeatQualifier_WizardLVL,15"]),
    (RuleSetId::Um, 143, "Discovery (Staff-Like Wand)", &["PREABILITY:1,CATEGORY=FEAT,Craft Staff", "PREVARGTEQ:FeatQualifier_WizardLVL,11"]),
    // Ultimate Psionics — 200 of 221 record(s) in the hand-authored table carry at least one `PRE`-family token.
    (RuleSetId::Upsi, 0, "Access Psionic Talent", &["PREMULT:1,[PREVARGTEQ:MaxManifesterLVL,1],[PREABILITY:1,CATEGORY=FEAT,Unlocked Talent]", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 1, "Additional Terror", &["PREABILITY:1,CATEGORY=Special Ability,Dread ~ Terror", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 2, "Advanced Archer Path", &["PREABILITY:1,CATEGORY=Special Ability,Archer Path ~ First,Archer Path ~ Second", "PREABILITY:2,CATEGORY=FEAT,Point-Blank Shot,Precise Shot", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 3, "Advanced Ascetic Path", &["PREABILITY:1,CATEGORY=Special Ability,Ascetic Path ~ First,Ascetic Path ~ Second", "PREABILITY:2,CATEGORY=FEAT,Dodge,Psionic Dodge", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 4, "Advanced Assassin Path", &["PREABILITY:1,CATEGORY=Special Ability,Assassin's Path ~ First,Assassin's Path ~ Second", "PREABILITY:2,CATEGORY=FEAT,Deep Impact,Psionic Weapon", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 5, "Advanced Brawling Path", &["PREABILITY:1,CATEGORY=Special Ability,Brawling Path ~ First,Brawling Path ~ Second", "PREABILITY:2,CATEGORY=FEAT,Improved Grapple,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 6, "Advanced Constructs", &["PRESPELL:1,Astral Construct", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 7, "Advanced Dervish Path", &["PREABILITY:1,CATEGORY=Special Ability,Dervish Path ~ First,Dervish Path ~ Second", "PREABILITY:2,CATEGORY=FEAT,Double Slice,Two-Weapon Fighting", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PRESTAT:1,DEX=15", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 8, "Advanced Feral Path", &["PREABILITY:1,CATEGORY=Special Ability,Feral Path ~ First,Feral Path ~ Second", "PREABILITY:2,CATEGORY=FEAT,Psionic Fist,Unavoidable Strike", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 9, "Advanced Infiltrator Path", &["PREABILITY:1,CATEGORY=Special Ability,Infiltrator Path ~ First,Infiltrator Path ~ Second", "PREABILITY:1,CATEGORY=FEAT,Deceitful", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 10, "Advanced Interceptor Path", &["PREABILITY:1,CATEGORY=Special Ability,Interceptor Path ~ First,Interceptor Path ~ Second", "PREABILITY:1,CATEGORY=FEAT,Combat Expertise", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 11, "Advanced Mind Knight Path", &["PREABILITY:1,CATEGORY=Special Ability,Mind Knight Path ~ First,Mind Knight Path ~ Second", "PREABILITY:1,CATEGORY=FEAT,Psionic Meditation", "PREABILITY:1,CATEGORY=FEAT,Psionic Shot,Psionic Weapon", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 12, "Advanced Survivor Path", &["PREABILITY:1,CATEGORY=Special Ability,Survivor Path ~ First,Survivor Path ~ Second", "PREABILITY:2,CATEGORY=FEAT,Mind Over Body,Toughness", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PRESTAT:1,CON=13", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 13, "Advanced Weaponmaster Path", &["PREABILITY:1,CATEGORY=Special Ability,Weaponmaster Path ~ First,Weaponmaster Path ~ Second", "PREABILITY:3,CATEGORY=FEAT,Combat Expertise,Improved Disarm,Weapon Focus", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 14, "Aligned Attack (Chaos)", &["!PREABILITY:1,CATEGORY=FEAT,TYPE=AlignedAttack", "PREALIGN:CG,CN,CE", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 15, "Aligned Attack (Evil)", &["!PREABILITY:1,CATEGORY=FEAT,TYPE=AlignedAttack", "PREALIGN:CE,NE,LE", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 16, "Aligned Attack (Good)", &["!PREABILITY:1,CATEGORY=FEAT,TYPE=AlignedAttack", "PREALIGN:CG,NG,LG", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 17, "Aligned Attack (Law)", &["!PREABILITY:1,CATEGORY=FEAT,TYPE=AlignedAttack", "PREALIGN:LG,LN,LE", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 18, "Assassin's Shot", &["PREABILITY:1,CATEGORY=Special Ability,Marksman ~ Sniper Style", "PREVARGTEQ:SneakAttackDice,1"]),
    (RuleSetId::Upsi, 19, "Assassin's Venom", &["PREABILITY:1,CATEGORY=Special Ability,Assassin's Path ~ First,Assassin's Path ~ Second", "PREABILITY:1,CATEGORY=Special Ability,Prevenom Path Power,Prevenom Weapon Path Power", "PREMULT:1,[PRETOTALAB:3],[PREVARGTEQ:PrereqBAB,3]", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 21, "Body Fuel", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 22, "Boost Construct", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 23, "Broken Dreams Style", &["PREABILITY:1,CATEGORY=Special Ability,Dread ~ Devastating Touch", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRESKILL:1,Intimidate=3", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 25, "Burrowing Power", &["PRESKILL:1,Spellcraft=8", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 26, "Chain Power", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 27, "Channel Rage", &["PREABILITY:2,CATEGORY=Special Ability,Wilder ~ Wild Surge,Rage", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 28, "Cloak Dance", &["PRESKILL:2,Stealth=7,Perform (Dance)=2"]),
    (RuleSetId::Upsi, 29, "Combat Manifestation", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 31, "Craft Crystalline Focus", &["PREVARGTEQ:MaxManifesterLVL,3", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 32, "Craft Cognizance Crystal", &["PREVARGTEQ:MaxManifesterLVL,3"]),
    (RuleSetId::Upsi, 33, "Crippling Assault", &["PREABILITY:3,CATEGORY=FEAT,Intimidating Shot,Point-Blank Shot,Staggering Shot", "PREMULT:1,[PRETOTALAB:11],[PREVARGTEQ:PrereqBAB,11]"]),
    (RuleSetId::Upsi, 34, "Critical Refocus", &["PREABILITY:1,CATEGORY=FEAT,Improved Critical", "PREMULT:1,[PRETOTALAB:8],[PREVARGTEQ:PrereqBAB,8]", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 35, "Cushion the Blow", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Collective", "PRESTAT:1,WIS=15", "PREVARGTEQ:MaxManifesterLVL,6", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 37, "Deadly Throw", &["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot", "PREMULT:1,[PRETOTALAB:3],[PREVARGTEQ:PrereqBAB,3]", "PRESTAT:1,DEX=15"]),
    (RuleSetId::Upsi, 38, "Deep Focus", &["PREABILITY:1,CATEGORY=FEAT,Psionic Body", "PRESKILL:1,Autohypnosis=4", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 39, "Deep Impact", &["PREABILITY:1,CATEGORY=FEAT,Psionic Weapon", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PRESTAT:1,STR=13", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 40, "Defensive Tactics", &["PREABILITY:2,CATEGORY=Special Ability,Tactician ~ Strategy,Armor Training"]),
    (RuleSetId::Upsi, 41, "Delay Power", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 42, "Disciple of Fear", &["PREABILITY:2,CATEGORY=Special Ability,Dread ~ Devastating Touch,Dread ~ Terror", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 43, "Dispelling Static", &["PRESKILL:1,Spellcraft=5", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 46, "Efficient Aid", &["PREABILITY:1,CATEGORY=Special Ability,Request Aid", "PRESKILL:1,Heal=7", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 47, "Elemental Blast", &["PREABILITY:1,CATEGORY=Special Ability,Surge Blast", "PREVARGTEQ:IsPsionic,1", "PREVARGTEQ:WildSurge,1"]),
    (RuleSetId::Upsi, 48, "Empower Power", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 49, "Empowered Shot", &["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot"]),
    (RuleSetId::Upsi, 50, "Endowed Mind", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 51, "Enervation Fortitude", &["PREABILITY:1,CATEGORY=Special Ability,Psychic Enervation", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 52, "Enhanced Steal Life", &["PREABILITY:1,CATEGORY=Special Ability,Steal Life", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 53, "Enlarge Power", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 54, "Enlarged Collective", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Collective", "PREVARGTEQ:IsPsionic,1", "PREVARGTEQ:MaxManifesterLVL,3"]),
    (RuleSetId::Upsi, 55, "Expanded Collective", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Collective", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 56, "Expanded Favored Weapon", &["PREABILITY:1,CATEGORY=Special Ability,Favored Weapon", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]"]),
    (RuleSetId::Upsi, 57, "Expanded Knowledge", &["PREVARGTEQ:MaxManifesterLVL,3", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 58, "Expanded Martial Power", &["PREABILITY:1,CATEGORY=Special Ability,Psychic Warrior ~ Martial Power", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 59, "Expanded Strategies", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Strategy", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 60, "Expansive Collective", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Collective", "PREABILITY:1,CATEGORY=Special Ability,Tactician ~ Spirit of Many,Vitalist ~ Spirit of Many", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 61, "Explosive Power", &["PREVARGTEQ:MaxManifesterLVL,3", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 62, "Extra Blade Skill", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.BladeSkill,Blade Skill", "PREMULT:1,[PRETOTALAB:2],[PREVARGTEQ:PrereqBAB,2]", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 63, "Extend Power", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 64, "Extended Blast", &["PREABILITY:1,CATEGORY=Special Ability,Surge Blast"]),
    (RuleSetId::Upsi, 65, "Extra Customization", &["PREABILITY:1,CATEGORY=Special Ability,Aegis ~ Form Astral Suit", "PREVARGT:floor(AstralSuitLVL/5),count(\"ABILITIES\",\"CATEGORY=FEAT\",\"NAME=Extra Customization\")", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 66, "Extra Disruption Type", &["PREABILITY:1,CATEGORY=Special Ability,Cryptic ~ Disrupt Pattern", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 67, "Extra Insight", &["PREABILITY:1,CATEGORY=Internal,Cryptic ~ Insights", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 68, "Extra Power Known", &["PREVARGTEQ:MaxManifesterLVL,1", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 69, "Extra Reconfiguration", &["PREABILITY:2,CATEGORY=Special Ability,Aegis ~ Form Astral Suit,Aegis ~ Reconfigure", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 70, "Extra Strategy", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Strategy", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 71, "Extra Terrors", &["PREABILITY:1,CATEGORY=Special Ability,Dread ~ Terror", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 72, "Extra Transfer", &["PREABILITY:1,CATEGORY=Special Ability,Transfer Wounds,Miasmic ~ Sickening Touch", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 73, "Fast Aid", &["PREABILITY:1,CATEGORY=Special Ability,Request Aid", "PRESKILL:2,Heal=7,Spellcraft=7", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 74, "Fast Step", &["PREABILITY:1,CATEGORY=Special Ability,Nomad's Step ~ Psychoportation", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 75, "Favored Energy (Cold)", &["!PREABILITY:1,CATEGORY=FEAT,TYPE=FavoredEnergy", "PRETEXT:Able to manifest any power that deals cold damage.", "PREVARGTEQ:MaxManifesterLVL,3", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 76, "Favored Energy (Electricity)", &["!PREABILITY:1,CATEGORY=FEAT,TYPE=FavoredEnergy", "PRETEXT:Able to manifest any power that deals electricity damage.", "PREVARGTEQ:MaxManifesterLVL,3", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 77, "Favored Energy (Fire)", &["!PREABILITY:1,CATEGORY=FEAT,TYPE=FavoredEnergy", "PRETEXT:Able to manifest any power that deals fire damage.", "PREVARGTEQ:MaxManifesterLVL,3", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 78, "Favored Energy (Sonic)", &["!PREABILITY:1,CATEGORY=FEAT,TYPE=FavoredEnergy", "PRETEXT:Able to manifest any power that deals sonic damage.", "PREVARGTEQ:MaxManifesterLVL,3", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 79, "Fear Mastery", &["PREABILITY:1,CATEGORY=Special Ability,Dread ~ Devastating Touch", "PREABILITY:1,CATEGORY=FEAT,Psionic Endowment", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 80, "Fear's Reach", &["PREABILITY:1,CATEGORY=Special Ability,Dread ~ Devastating Touch", "PREABILITY:1,CATEGORY=FEAT,Psionic Shot", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 81, "Fell Shot", &["PREABILITY:2,CATEGORY=FEAT,Point-Blank Shot,Psionic Shot", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PRESTAT:1,DEX=13", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 83, "Fighter's Blade", &["PREABILITY:1,CATEGORY=Special Ability,Soulknife ~ Enhanced Mind Blade"]),
    (RuleSetId::Upsi, 85, "Focused Precision", &["PREABILITY:1,CATEGORY=Internal,Marksman Combat Style", "PREABILITY:1,CATEGORY=FEAT,Weapon Focus", "PREMULT:1,[PRETOTALAB:7],[PREVARGTEQ:PrereqBAB,7]", "PRESTAT:1,DEX=17", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 86, "Focused Sunder", &["PREABILITY:2,CATEGORY=FEAT,Power Attack,Improved Sunder", "PRESTAT:1,STR=13", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 87, "Ghost Attack", &["PREMULT:1,[PRETOTALAB:3],[PREVARGTEQ:PrereqBAB,3]", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 88, "Gravitic Stability", &["PRECLASS:1,Elocater=1", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 89, "Greater Cushion the Blow", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Collective", "PREABILITY:2,CATEGORY=FEAT,Cushion the Blow,Improved Cushion the Blow", "PRESTAT:1,WIS=17", "PREVARGTEQ:MaxManifesterLVL,18", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 90, "Greater Intuitive Shot", &["PREABILITY:3,CATEGORY=FEAT,Intuitive Shot,Point-Blank Shot,Psionic Shot", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PRESTAT:1,WIS=13"]),
    (RuleSetId::Upsi, 91, "Greater Power Penetration", &["PREABILITY:1,CATEGORY=FEAT,Power Penetration", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 92, "Greater Power Specialization", &["PREABILITY:2,CATEGORY=FEAT,Power Specialization,Weapon Focus (Ray)", "PREVARGTEQ:MaxManifesterLVL,12", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 93, "Greater Psionic Endowment", &["PREABILITY:1,CATEGORY=FEAT,Psionic Endowment", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 94, "Greater Psionic Fist", &["PREABILITY:1,CATEGORY=FEAT,Psionic Fist", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PRESTAT:1,STR=13", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 95, "Greater Psionic Shot", &["PREABILITY:2,CATEGORY=FEAT,Point-Blank Shot,Psionic Shot", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 96, "Greater Psionic Weapon", &["PREABILITY:1,CATEGORY=FEAT,Psionic Weapon", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PRESTAT:1,STR=13", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 97, "Harmonic Resonance", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Collective", "PRESKILL:1,Spellcraft=3", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 98, "Hawkeye", &["PREABILITY:2,CATEGORY=FEAT,Far Shot,Point-Blank Shot"]),
    (RuleSetId::Upsi, 99, "Hustle Power", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 100, "Improved Cover Fire", &["PREABILITY:1,CATEGORY=Special Ability,Marksman ~ Cover Fire", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 101, "Improved Cushion the Blow", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Collective", "PREABILITY:1,CATEGORY=FEAT,Cushion the Blow", "PRESTAT:1,WIS=15", "PREVARGTEQ:MaxManifesterLVL,12", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 102, "Improved Disengage", &["PREABILITY:1,CATEGORY=Special Ability,Marksman ~ Disengage", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 103, "Improved Disruption", &["PREABILITY:1,CATEGORY=Special Ability,Cryptic ~ Disrupt Pattern", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 104, "Improved Metamorphosis", &["PRESPELL:1,Metamorphosis (Minor),Metamorphosis,Metamorphosis (Greater),Metamorphosis (True)", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 105, "Improved Psi-Like Ability", &["PRETEXT:Racial psi-like ability", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 106, "Improved Psicrystal", &["PREABILITY:1,CATEGORY=FEAT,Psicrystal Affinity", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 107, "Inquisitor", &["PRESTAT:1,WIS=13", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 108, "Insightful Terror", &["PREABILITY:1,CATEGORY=Special Ability,Dread ~ Fearsome Insight", "PRESKILL:1,Intimidate=9", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 109, "Intimidating Shot", &["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot"]),
    (RuleSetId::Upsi, 110, "Intuitive Fighting", &["PREABILITY:1,CATEGORY=FEAT,Blind-Fight", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 111, "Intuitive Shot", &["PREABILITY:2,CATEGORY=FEAT,Point-Blank Shot,Psionic Shot", "PRESTAT:1,WIS=13"]),
    (RuleSetId::Upsi, 112, "Killer's Vitality", &["PREABILITY:1,CATEGORY=Special Ability,Soulthief Vitalist Method", "PREVARGTEQ:SneakAttackDice,1"]),
    (RuleSetId::Upsi, 113, "Knightmare", &["PREABILITY:2,CATEGORY=Special Ability,Dread ~ Terror,Cavalier's Charge ~ Cavalier"]),
    (RuleSetId::Upsi, 114, "Knockdown Shot", &["PREABILITY:2,CATEGORY=FEAT,Point-Blank Shot,Psionic Shot", "PRESTAT:1,DEX=15"]),
    (RuleSetId::Upsi, 115, "Levitative Transport", &["PREABILITY:1,CATEGORY=FEAT,Gravitic Stability", "PRECLASS:1,Elocater=5", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 117, "Malleable Power", &["PREABILITY:1,CATEGORY=FEAT,Selective Power", "PRESKILL:1,Spellcraft=10"]),
    (RuleSetId::Upsi, 118, "Master of All Forms", &["PREABILITY:1,CATEGORY=FEAT,Swift Shapeshifter", "PRESPELL:1,Metamorphosis", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 119, "Master's Refuge", &["PREABILITY:1,CATEGORY=Special Ability,Twofold Master", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 120, "Master's Voice", &["PREABILITY:1,CATEGORY=Special Ability,Thrallherd", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 121, "Maximize Power", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 122, "Mental Leap", &["PRESKILL:1,Acrobatics=2", "PRESTAT:1,STR=13,DEX=13", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 124, "Merge Designs", &["PREABILITY:1,CATEGORY=Special Ability,Cryptic ~ Pattern Designs", "PREABILITY:1,CATEGORY=FEAT,Scribe Tattoo", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 125, "Metapsionic Mastery", &["PREABILITY:1,CATEGORY=FEAT,TYPE=Metapsionic", "!PREABILITY:1,CATEGORY=FEAT,Metapsionic Mastery", "PREVARGTEQ:MaxManifesterLVL,3", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 126, "Mind Blade Knight", &["PREABILITY:1,CATEGORY=Special Ability,First ~ Mind Knight Path,Mind Knight Path ~ Second", "PREVARGTEQ:IsPsionic,1", "PREVARGTEQ:WarriorPathLVL,3"]),
    (RuleSetId::Upsi, 127, "Mind Knight's Arsenal", &["PREABILITY:1,CATEGORY=Special Ability,Mind Knight Path ~ First,Mind Knight Path ~ Second", "PREABILITY:1,CATEGORY=Special Ability,Call Weaponry Path Power", "PREABILITY:1,CATEGORY=FEAT,Weapon Focus", "PREMULT:1,[PRETOTALAB:3],[PREVARGTEQ:PrereqBAB,3]", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 128, "Mind Over Body", &["PRESTAT:1,CON=13"]),
    (RuleSetId::Upsi, 129, "Mixed Combat", &["PREABILITY:1,CATEGORY=FEAT,Quick Draw", "PRETOTALAB:6"]),
    (RuleSetId::Upsi, 130, "Modified Blast", &["PREABILITY:1,CATEGORY=Special Ability,Surge Blast", "PREVARGTEQ:MaxManifesterLVL,3", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 131, "Multiple Connections", &["PREABILITY:1,CATEGORY=Special Ability,Terror ~ Mindlock", "PREABILITY:1,CATEGORY=FEAT,Open Door", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 132, "Nightmare Veil", &["PREABILITY:1,CATEGORY=Special Ability,Dread ~ Devastating Touch", "PREABILITY:3,CATEGORY=FEAT,Improved Unarmed Strike,Broken Dreams Style,Shattered Dream Strike", "PRESKILL:1,Intimidate=9", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 133, "Nomad's Jump", &["PREABILITY:2,CATEGORY=Special Ability,Nomad's Step ~ Psychoportation,Elocater ~ Aerial Acrobatics", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 134, "One Pattern", &["PREABILITY:1,CATEGORY=Special Ability,Cryptic ~ Disrupt Pattern", "PRESKILL:6,Knowledge (Arcana)=5,Knowledge (Dungeoneering)=5,Knowledge (Nature)=5,Knowlede (Planes)=5,Knowledge (Psionics)=5,Knowledge (Religion)=5", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 135, "Open Door", &["PREABILITY:1,CATEGORY=Special Ability,Terror ~ Mindlock", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 137, "Opportunity Power", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 138, "Overchannel", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 139, "Penetrating Fear", &["PREABILITY:1,CATEGORY=Special Ability,Dread ~ Terror", "PRECLASS:1,Dread=10", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 140, "Persistent Focus", &["PREABILITY:1,CATEGORY=FEAT,Psionic Meditation", "PRESKILL:1,Autohypnosis=4", "PRESTAT:1,WIS=13", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 143, "Piranha Strike", &["PREVAREQ:HasWeaponFinesseFeat,1", "PRETOTALAB:1"]),
    (RuleSetId::Upsi, 144, "Power Channeler", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 145, "Power Penetration", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 146, "Power Perfection", &["PREABILITY:3,CATEGORY=FEAT,TYPE.Metapsionic", "PRESKILL:1,Spellcraft=15", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 147, "Power Specialization", &["PREABILITY:1,CATEGORY=FEAT,Weapon Focus (Ray)", "PREVARGTEQ:MaxManifesterLVL,4", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 148, "Psicrystal Affinity", &["PREVARGTEQ:MaxManifesterLVL,1", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 149, "Psicrystal Containment", &["PREABILITY:1,CATEGORY=FEAT,Psicrystal Affinity", "PREVARGTEQ:MaxManifesterLVL,3", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 150, "Psionic Body", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 151, "Psionic Bull Rush", &["PREABILITY:1,CATEGORY=FEAT,Improved Bull Rush", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 152, "Psionic Charge", &["PREABILITY:1,CATEGORY=FEAT,Speed of Thought", "PRESTAT:1,DEX=13", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 153, "Psionic Critical", &["PREABILITY:1,CATEGORY=FEAT,Improved Critical", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 154, "Psionic Disarm", &["PREABILITY:1,CATEGORY=FEAT,Improved Disarm", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 155, "Psionic Dodge", &["PREABILITY:1,CATEGORY=FEAT,Dodge", "PRESTAT:1,DEX=13", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 156, "Psionic Endowment", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 157, "Psionic Fist", &["PRESTAT:1,STR=13", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 158, "Psionic Meditation", &["PRESKILL:1,Autohypnosis=4", "PRESTAT:1,WIS=13", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 159, "Psionic Overrun", &["PREABILITY:2,CATEGORY=FEAT,Power Attack,Improved Overrun", "PREMULT:1,[PRETOTALAB:1],[PREVARGTEQ:PrereqBAB,1]", "PRESTAT:1,STR=13", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 160, "Psionic Precise Shot", &["PREABILITY:3,CATEGORY=FEAT,Improved Precise Shot,Point-Blank Shot,Precise Shot", "PREMULT:1,[PRETOTALAB:11],[PREVARGTEQ:PrereqBAB,11]", "PRESTAT:1,DEX=19", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 161, "Psionic Shield Bash", &["PREABILITY:2,CATEGORY=FEAT,Improved Shield Bash,Shield Proficiency", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 162, "Psionic Shot", &["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 163, "Psionic Stamina", &["PREABILITY:1,CATEGORY=FEAT,Psionic Body", "PREVARGTEQ:IsPsionic,1", "PREVARGTEQ:SoulknifeFeatPrereqLVL,3"]),
    (RuleSetId::Upsi, 164, "Psionic Sunder", &["PREABILITY:2,CATEGORY=FEAT,Power Attack,Improved Sunder", "PREMULT:1,[PRETOTALAB:1],[PREVARGTEQ:PrereqBAB,1]", "PRESTAT:1,STR=13", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 165, "Psionic Talent", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 166, "Psionic Trip", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Trip", "PRESTAT:1,INT=13", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 167, "Psionic Unarmed Strike", &["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Stunning Fist", "PRESTAT:2,DEX=13,WIS=13", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 168, "Psionic Weapon", &["PRESTAT:1,STR=13", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 169, "Psychoportive Pathfinder", &["PRECLASS:1,Nomad=1", "PRESPELL:1,Trace Teleport", "PRESPELL:1,Psychoport,Psychoport(Greater)", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 170, "Pyromaniac", &["PREABILITY:2,CATEGORY=Special Ability,Pyrokinetic ~ Fire Lash,Bomb ~ Alchemist", "PREVARGTEQ:AlchemistBombAdditionalDice,1"]),
    (RuleSetId::Upsi, 171, "Quick Suit", &["PREABILITY:2,CATEGORY=Special Ability,Aegis ~ Form Astral Suit,Aegis ~ Reconfigure", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 172, "Quicken Power", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 173, "Raging Hulk", &["PREABILITY:2,CATEGORY=Special Ability,Aegis ~ Form Astral Suit,Rage"]),
    (RuleSetId::Upsi, 174, "Rapid Augmentation", &["PREABILITY:1,CATEGORY=Special Ability,Aegis ~ Augment Suit", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 175, "Rapid Draw", &["PREABILITY:1,CATEGORY=Special Ability,Blade Skill ~ Alter Blade", "PREABILITY:1,CATEGORY=Special Ability,Soulknife ~ Quick Draw", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 176, "Rapid Metabolism", &["PRESTAT:1,CON=13"]),
    (RuleSetId::Upsi, 177, "Ready Response", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 178, "Rebounding Throw", &["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot", "PREMULT:1,[PRETOTALAB:1],[PREVARGTEQ:PrereqBAB,1]"]),
    (RuleSetId::Upsi, 179, "Reckless Offense", &["PRETOTALAB:1"]),
    (RuleSetId::Upsi, 181, "Resonance Mastery", &["PREABILITY:1,CATEGORY=FEAT,TYPE.Metapsionic", "PRECLASS:1,Psicrystal Imprinter=5", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 182, "Return Shot", &["PREABILITY:3,CATEGORY=FEAT,Fell Shot,Point-Blank Shot,Psionic Shot", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 183, "Returning Throw", &["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot", "PREMULT:1,[PRETOTALAB:1],[PREVARGTEQ:PrereqBAB,1]"]),
    (RuleSetId::Upsi, 184, "Ricochet", &["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]"]),
    (RuleSetId::Upsi, 186, "Scholarly Discipline", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.PsionicDiscipline", "PREABILITY:1,CATEGORY=Special Ability,TYPE.ArcaneSchool"]),
    (RuleSetId::Upsi, 187, "Scribe Tattoo", &["PREVARGTEQ:MaxManifesterLVL,3"]),
    (RuleSetId::Upsi, 188, "Selective Power", &["PRESKILL:1,Spellcraft=10"]),
    (RuleSetId::Upsi, 190, "Shattered Dream Strike", &["PREABILITY:1,CATEGORY=Special Ability,Dread ~ Devastating Touch", "PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Broken Dreams Style", "PRESKILL:1,Intimidate=6", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 192, "Sidestep Charge", &["PREABILITY:1,CATEGORY=FEAT,Dodge", "PRESTAT:1,DEX=13"]),
    (RuleSetId::Upsi, 193, "Soul Warrior", &["PREABILITY:2,CATEGORY=Special Ability,Soulknife ~ Enhanced Mind Blade,Warrior's Path ~ Psychic Warrior", "PREVARGTEQ:MndBladeEnhancement,2"]),
    (RuleSetId::Upsi, 194, "Speed of Thought", &["PRESTAT:1,WIS=13", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 195, "Split Headed Lash", &["PREABILITY:1,CATEGORY=Special Ability,Pyrokineticist ~ Fire Lash", "PREABILITY:3,CATEGORY=FEAT,Point-Blank Shot,Precise Shot,Weapon Focus (Whip)", "PRESKILL:1,Knowledge (Psionics)=10", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 196, "Split Psionic Ray", &["PREABILITY:1,CATEGORY=FEAT,TYPE=Metapsionic", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 197, "Staggering Shot", &["PREABILITY:2,CATEGORY=FEAT,Intimidating Shot,Point-Blank Shot", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]"]),
    (RuleSetId::Upsi, 198, "Student of the Astral Suit", &["PREABILITY:1,CATEGORY=Special Ability,Aegis ~ Invigorating Suit"]),
    (RuleSetId::Upsi, 199, "Surging Aura", &["PREVARGTEQ:IsPsionic,1", "PREVARGTEQ:WildSurge,1"]),
    (RuleSetId::Upsi, 200, "Swift Shapeshifter", &["PRESPELL:1,Metamorphosis", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 201, "Talented", &["PREABILITY:1,CATEGORY=FEAT,Overchannel", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 202, "Telepathic Link", &["PRESPELL:1,Mindlink", "PREVARGTEQ:MaxManifesterLVL,3"]),
    (RuleSetId::Upsi, 203, "Terror Mastery", &["PREABILITY:8,CATEGORY=Special Ability,TYPE.DreadTerror", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 205, "Tomb Raider", &["PREABILITY:2,CATEGORY=Special Ability,Cryptic ~ Trapmaker,Favored Terrain (Underground)"]),
    (RuleSetId::Upsi, 207, "Touch of Terror", &["PREABILITY:1,CATEGORY=Special Ability,Dread ~ Devastating Touch", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 208, "Toughened Suit", &["PREABILITY:1,CATEGORY=Special Ability,Aegis ~ Form Astral Suit", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 209, "Twin Power", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 210, "Twin Throw", &["PREABILITY:2,CATEGORY=FEAT,Point-Blank Shot,Two-Weapon Fighting", "PRESTAT:1,DEX=17", "PRETOTALAB:6"]),
    (RuleSetId::Upsi, 211, "Unavoidable Strike", &["PREABILITY:1,CATEGORY=FEAT,Psionic Fist", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PRESTAT:1,STR=13", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 212, "Unconditional Power", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 213, "Unlocked Talent", &["PREABILITY:1,CATEGORY=FEAT,Wild Talent", "!PREABILITY:1,CATEGORY=FEAT,Unlocked Talent"]),
    (RuleSetId::Upsi, 214, "Unwilling Participant", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Collective", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 215, "Up the Walls", &["PRESTAT:1,WIS=13", "PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 217, "Widen Power", &["PREVARGTEQ:IsPsionic,1"]),
    (RuleSetId::Upsi, 218, "Wildblood Mage", &["PREABILITY:1,CATEGORY=Special Ability,Wilder ~ Wild Surge", "PREABILITY:1,CATEGORY=Special Ability,TYPE.Sorcerer Bloodline", "PREVARGTEQ:WildSurge,1"]),
    (RuleSetId::Upsi, 220, "Wounding Attack", &["PREMULT:1,[PRETOTALAB:8],[PREVARGTEQ:PrereqBAB,8]", "PREVARGTEQ:IsPsionic,1"]),
];


/// The keys a backfill table listed as **checked, and the corpus row carries
/// no `PRE`-family token at all** — as distinct from a key that was never
/// gathered. `ARG_FEAT_PREREQUISITES` / `PU_FEAT_PREREQUISITES` /
/// `UCA_FEAT_PREREQUISITES` drew that distinction by listing every key in
/// the book and mapping the empty ones to `&[]`; this roster is where that
/// fact now lives, and `tests::every_backfilled_book_key_was_checked` is the
/// successor to the assertion that enforced it.
pub static CHECKED_AND_CARRIES_NO_PRE_TOKEN: &[(RuleSetId, &str)] = &[
    (RuleSetId::Pu, "Critical Cure"),
    (RuleSetId::Pu, "Endurance"),
    (RuleSetId::Pu, "Twist the Knife"),
];

fn index_of(table: &'static [FeatPrereqRow]) -> &'static HashMap<(RuleSetId, usize), FeatPrereqRow> {
    static HAND: OnceLock<HashMap<(RuleSetId, usize), FeatPrereqRow>> = OnceLock::new();
    static GAP: OnceLock<HashMap<(RuleSetId, usize), FeatPrereqRow>> = OnceLock::new();
    let cell = if std::ptr::eq(table, HAND_AUTHORED_FEAT_PREREQ_TOKENS) { &HAND } else { &GAP };
    cell.get_or_init(|| table.iter().map(|row| ((row.0, row.1), *row)).collect())
}

fn lookup(
    table: &'static [FeatPrereqRow],
    rule_set: RuleSetId,
    index: usize,
    key: &str,
) -> Option<&'static [&'static str]> {
    let row = index_of(table).get(&(rule_set, index))?;
    assert_eq!(
        row.2, key,
        "{rule_set:?} index {index} holds the tokens taken from {:?}, but the caller passed {key:?}; \
         the live table this was relocated from has been reordered",
        row.2
    );
    Some(row.3)
}

/// The `PRE`-family tokens the hand-authored record at `index` in
/// `rule_set`'s table carried, or `None` when its corpus row carried none.
///
/// `key` is the caller's own record key and is asserted against the one the
/// row was taken from — see this module's doc comment.
pub fn hand_authored_feat_prereq_tokens(
    rule_set: RuleSetId,
    index: usize,
    key: &str,
) -> Option<&'static [&'static str]> {
    lookup(HAND_AUTHORED_FEAT_PREREQ_TOKENS, rule_set, index, key)
}

/// Every relocated hand-authored token row for `key` in `rule_set`, in the
/// book table's own order.
///
/// Usually 0 or 1 rows; CRB's two distinct `"Combat Expertise"` records make
/// it 2, which is exactly why the converter's own lookups are addressed by
/// index instead. Provided for the parity tests that walk a book whose keys
/// are known to be unique and have no index in hand.
pub fn hand_authored_rows_for_key(
    rule_set: RuleSetId,
    key: &str,
) -> Vec<&'static [&'static str]> {
    HAND_AUTHORED_FEAT_PREREQ_TOKENS
        .iter()
        .filter(|row| row.0 == rule_set && row.2 == key)
        .map(|row| row.3)
        .collect()
}

/// The single relocated hand-authored token row for `key` in `rule_set`, or
/// `None` when the corpus row carried no `PRE`-family token.
///
/// # Panics
///
/// When `key` is not unique in that book — see [`hand_authored_rows_for_key`].
pub fn hand_authored_tokens_for_unique_key(
    rule_set: RuleSetId,
    key: &str,
) -> Option<&'static [&'static str]> {
    let rows = hand_authored_rows_for_key(rule_set, key);
    assert!(rows.len() <= 1, "{rule_set:?} '{key}' is not a unique key: {} rows", rows.len());
    rows.into_iter().next()
}

/// The tokens for the record at `joined_index` in
/// `feats_all::all_feat_tables()`'s slice for `rule_set` — that slice is the
/// book's hand-authored records followed by its gap rows, so this routes to
/// whichever of the two relocated tables owns the row.
///
/// The joined catalog is what every prerequisite-census consumer walks, and
/// it is the one place a `(rule_set, key)` lookup is unusable twice over:
/// CRB's duplicate `"Combat Expertise"` key, and the 142 Mythic rows that
/// deliberately reuse an earlier book's key.
pub fn joined_catalog_tokens(
    rule_set: RuleSetId,
    joined_index: usize,
    key: &str,
) -> Option<&'static [&'static str]> {
    use crate::rules_core::rules_tables::feats_all::hand_authored_feat_tables;
    let hand_len = hand_authored_feat_tables()
        .iter()
        .find(|book| book.rule_set == rule_set)
        .map_or(0, |book| book.entries.len());
    if joined_index < hand_len {
        hand_authored_feat_prereq_tokens(rule_set, joined_index, key)
    } else {
        feat_gap_prereq_tokens(rule_set, joined_index - hand_len, key)
    }
}

pub use crate::pcgen_import::feat_gap_prereq_tokens::FEAT_GAP_PREREQ_TOKENS;

/// Every relocated gap-row token row for `key` in `rule_set`, in the gap
/// table's own order. The gap-row counterpart of
/// [`hand_authored_rows_for_key`].
pub fn feat_gap_rows_for_key(rule_set: RuleSetId, key: &str) -> Vec<&'static [&'static str]> {
    FEAT_GAP_PREREQ_TOKENS
        .iter()
        .filter(|row| row.0 == rule_set && row.2 == key)
        .map(|row| row.3)
        .collect()
}

/// The `PRE`-family tokens the gap row at `index` in `rule_set`'s gap table
/// carried, or `None` when its corpus row carried none.
pub fn feat_gap_prereq_tokens(
    rule_set: RuleSetId,
    index: usize,
    key: &str,
) -> Option<&'static [&'static str]> {
    lookup(FEAT_GAP_PREREQ_TOKENS, rule_set, index, key)
}

/// The relocated hand-authored row count, asserted above and cited by the
/// cycle receipt.
pub const HAND_ROW_COUNT: usize = 1429;
/// The relocated gap-row count.
pub const GAP_ROW_COUNT: usize = 601;
/// Both together — the joined catalog's prerequisite-bearing record count.
pub const JOINED_ROW_COUNT: usize = 2030;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules_core::rules_tables::feat_gap_tables::feat_gap_rows_for;
    use crate::rules_core::rules_tables::feats_all::hand_authored_feat_tables;

    /// The pairing gate. Every relocated row must still name the record it
    /// was taken from, and every index must still be in range — this is what
    /// makes an index-addressed relocation safe against a later table edit.
    #[test]
    fn every_row_still_names_the_record_it_was_taken_from() {
        for &(rule_set, index, key, tokens) in HAND_AUTHORED_FEAT_PREREQ_TOKENS {
            let table = hand_authored_feat_tables()
                .iter()
                .find(|book| book.rule_set == rule_set)
                .unwrap_or_else(|| panic!("{rule_set:?} has no hand-authored feat table"));
            let record = table
                .entries
                .get(index)
                .unwrap_or_else(|| panic!("{rule_set:?} index {index} is past the end of its table"));
            assert_eq!(record.key, key, "{rule_set:?} index {index}");
            assert!(!tokens.is_empty(), "{rule_set:?} {key}: an absent row must be absent, never empty");
        }
        for &(rule_set, index, key, tokens) in FEAT_GAP_PREREQ_TOKENS {
            let rows = feat_gap_rows_for(rule_set);
            let record = rows
                .get(index)
                .unwrap_or_else(|| panic!("{rule_set:?} gap index {index} is past the end of its table"));
            assert_eq!(record.key, key, "{rule_set:?} gap index {index}");
            assert!(!tokens.is_empty(), "{rule_set:?} {key}: an absent row must be absent, never empty");
        }
    }

    /// Every token really is a `PRE`-family one, negated or not — the rule
    /// the removed field's own test
    /// (`feats_all::no_record_carries_an_empty_prerequisite_slice`) enforced
    /// before the move.
    #[test]
    fn every_token_is_a_pre_family_token() {
        for &(rule_set, _, key, tokens) in
            HAND_AUTHORED_FEAT_PREREQ_TOKENS.iter().chain(FEAT_GAP_PREREQ_TOKENS)
        {
            for token in tokens {
                assert!(
                    token.starts_with("PRE") || token.starts_with("!PRE"),
                    "{rule_set:?} '{key}' carries a non-PRE token {token:?}"
                );
            }
        }
    }

    /// The counts this relocation moved, per book, as the receipt states
    /// them. A book whose ingest gathers no `PRE` token at all has no row
    /// here at all rather than a zero.
    #[test]
    fn the_relocated_row_counts_are_the_ones_the_receipt_states() {
        assert_eq!(HAND_AUTHORED_FEAT_PREREQ_TOKENS.len(), HAND_ROW_COUNT);
        assert_eq!(FEAT_GAP_PREREQ_TOKENS.len(), GAP_ROW_COUNT);
        assert_eq!(HAND_ROW_COUNT + GAP_ROW_COUNT, JOINED_ROW_COUNT);
    }

    /// The successor to `feats_all`'s removed
    /// `every_arg_and_pu_catalog_key_has_a_gathered_prerequisite_row`: every
    /// key in the three books whose own ingest gathered no `PRE` token is
    /// accounted for here — either it carries tokens, or it is listed as
    /// checked-and-empty. A key in neither list would read as "this feat has
    /// no prerequisites" when the truth is "nobody looked".
    #[test]
    fn every_backfilled_book_key_was_checked() {
        use crate::rules_core::rules_tables::advanced_race_guide::feats as arg_feats;
        use crate::rules_core::rules_tables::pathfinder_unchained::feat_tables as pu_feats;
        use crate::rules_core::rules_tables::ultimate_campaign::feat_tables as uca_feats;

        let accounted = |rule_set: RuleSetId, key: &str| {
            HAND_AUTHORED_FEAT_PREREQ_TOKENS.iter().any(|row| row.0 == rule_set && row.2 == key)
                || CHECKED_AND_CARRIES_NO_PRE_TOKEN.iter().any(|row| row.0 == rule_set && row.1 == key)
        };
        for entry in arg_feats::feat_tables() {
            assert!(accounted(RuleSetId::Arg, entry.key), "ARG feat '{}' was never checked", entry.key);
        }
        for entry in pu_feats::feat_tables() {
            assert!(accounted(RuleSetId::Pu, entry.key), "PU feat '{}' was never checked", entry.key);
        }
        for entry in uca_feats::feat_tables() {
            assert!(accounted(RuleSetId::Uca, entry.key), "UCA feat '{}' was never checked", entry.key);
        }
        for &(rule_set, key) in CHECKED_AND_CARRIES_NO_PRE_TOKEN {
            assert!(
                !HAND_AUTHORED_FEAT_PREREQ_TOKENS.iter().any(|row| row.0 == rule_set && row.2 == key),
                "{rule_set:?} '{key}' is listed as carrying no token but also carries one"
            );
        }
    }

    /// The per-book coverage this relocation carried across, book by book —
    /// `feats_all`'s removed `the_per_book_prerequisite_coverage_is_the_real_one`,
    /// re-derived from the relocated rows. The ARG number is the point of the
    /// backfill that produced it: **every one of that book's 187 records
    /// carries at least one `PRE`-family token**.
    #[test]
    fn the_per_book_coverage_survived_the_move_unchanged() {
        let n = |rule_set: RuleSetId| {
            HAND_AUTHORED_FEAT_PREREQ_TOKENS.iter().filter(|row| row.0 == rule_set).count()
        };
        assert_eq!(n(RuleSetId::Crb), 130, "of 185");
        assert_eq!(n(RuleSetId::Apg), 143, "of 172");
        assert_eq!(n(RuleSetId::Acg), 125, "of 129");
        assert_eq!(n(RuleSetId::Arg), 187, "of 187 -- all of them");
        assert_eq!(n(RuleSetId::Pu), 14, "of 17");
        assert_eq!(n(RuleSetId::Uca), 23, "of 23 -- all of them carry PRETEXT:");
        assert_eq!(n(RuleSetId::Ui), 98, "of 104 -- gathered directly at ingest, no backfill table");
        assert_eq!(n(RuleSetId::Uw), 127, "of 135 -- gathered directly at ingest, no backfill table");
        assert_eq!(n(RuleSetId::Uc), 247, "of 261 -- gathered directly at ingest, no backfill table");
        assert_eq!(n(RuleSetId::Um), 135, "of 144 -- gathered directly at ingest, no backfill table");
        assert_eq!(n(RuleSetId::Upsi), 200, "of 221 -- gathered directly at ingest, no backfill table");
    }

    /// The named records `feats_all`'s removed tests turned on, asserted here
    /// against the relocated rows rather than against a field that no longer
    /// exists. Same keys, same expected tokens.
    #[test]
    fn the_relocated_tokens_are_the_ones_the_corpus_rows_carry() {
        let find = |rule_set: RuleSetId, key: &str| -> &'static [&'static str] {
            HAND_AUTHORED_FEAT_PREREQ_TOKENS
                .iter()
                .find(|row| row.0 == rule_set && row.2 == key)
                .unwrap_or_else(|| panic!("{rule_set:?} '{key}' must carry relocated tokens"))
                .3
        };

        assert_eq!(
            find(RuleSetId::Crb, "Improved Two-Weapon Fighting"),
            &[
                "PREABILITY:1,CATEGORY=FEAT,Two-Weapon Fighting",
                // The corpus states the Dex 17 requirement purely through
                // PCGen variables -- there is no `PRESTAT:` on this record at
                // all. The converter models both: `PreStatScore_DEX` IS the
                // Dex score per `cr__stats.lst`, and `FeatDexRequirement` is 0
                // for every character built here.
                "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,17],[PREVARGTEQ:FeatDexRequirement,17]",
                "PRETOTALAB:6",
            ][..]
        );
        assert_eq!(find(RuleSetId::Crb, "Two-Weapon Fighting").len(), 1);
        // A CRB record whose corpus row genuinely has no PRE token: absent here.
        assert!(
            !HAND_AUTHORED_FEAT_PREREQ_TOKENS
                .iter()
                .any(|row| row.0 == RuleSetId::Crb && row.2 == "Toughness"),
            "Toughness carries no PRE token, so it must have no row at all"
        );
        // ARG: never gathered before the backfill this table absorbed.
        assert_eq!(find(RuleSetId::Arg, "Armor of the Pit"), &["PREFACT:1,TEMPLATES,IsTiefling=true"][..]);
        // UCA carries its corpus `PRETEXT:` prose through, never a synthesised
        // formal token.
        assert!(find(RuleSetId::Uca, "Accursed")[0].starts_with("PRETEXT:Prerequisite:You must carry a curse"));
        // UI gathered its own tokens at ingest, with no backfill table.
        assert!(
            find(RuleSetId::Ui, "Acrobatic Spellcaster")[0]
                .starts_with("PREABILITY:2,CATEGORY=FEAT,Combat Casting")
        );
    }

    /// `feats_all`'s key-collision test used to make this check inline, off
    /// the field. It moved here with the tokens and is not weakened: every one
    /// of the 142 Mythic rows that share a key with an earlier book's feat
    /// names that exact key under `CATEGORY=FEAT` in its own `PREABILITY:`
    /// prerequisite — the corpus itself saying "you must already hold the base
    /// feat", which is the mechanical proof of variant-hood that a coincidental
    /// name clash could never carry.
    #[test]
    fn every_mythic_collision_names_the_base_feat_it_upgrades() {
        use crate::rules_core::rules_tables::feats_all::all_feat_tables;

        let mythic_keys: Vec<&'static str> = FEAT_GAP_PREREQ_TOKENS
            .iter()
            .filter(|row| row.0 == RuleSetId::Mythic)
            .map(|row| row.2)
            .collect();
        let earlier: Vec<&'static str> = all_feat_tables()
            .iter()
            .filter(|book| book.rule_set != RuleSetId::Mythic)
            .flat_map(|book| book.entries.iter())
            .map(|entry| entry.key)
            .collect();

        let mut checked = 0usize;
        for &(rule_set, _, key, tokens) in FEAT_GAP_PREREQ_TOKENS {
            if rule_set != RuleSetId::Mythic || !earlier.contains(&key) {
                continue;
            }
            checked += 1;
            assert!(
                tokens.iter().any(|p| p.starts_with("PREABILITY:")
                    && p.contains("CATEGORY=FEAT")
                    && p.contains(key)),
                "Mythic feat {key:?} collides with an earlier book but its own PREABILITY \
                 prerequisite does not name {key:?} under CATEGORY=FEAT -- this is the \
                 mechanical proof of variant-hood and it is missing, so this collision needs \
                 real per-record verification before it is trusted"
            );
        }
        assert_eq!(checked, 142, "re-derive if a book's feat gap rows change");
        assert_eq!(mythic_keys.len(), 195, "Mythic's prerequisite-bearing gap rows");
    }

    /// The lookup is keyed by `(rule_set, index)` precisely because
    /// `(rule_set, key)` collides. CRB's two `"Combat Expertise"` records
    /// carry different token sets, and both must survive the move.
    #[test]
    fn the_colliding_crb_key_keeps_both_distinct_token_sets() {
        let both: Vec<&'static [&'static str]> = HAND_AUTHORED_FEAT_PREREQ_TOKENS
            .iter()
            .filter(|row| row.0 == RuleSetId::Crb && row.2 == "Combat Expertise")
            .map(|row| row.3)
            .collect();
        assert_eq!(both.len(), 2, "CRB carries two 'Combat Expertise' records");
        assert_ne!(both[0], both[1], "their token sets differ; a key-only lookup would lose one");
    }
}
