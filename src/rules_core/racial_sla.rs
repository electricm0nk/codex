//! Racial spell-like-ability (SLA) chassis engine (SD-34 wave 51, bucket M
//! closure for `core_rulebook`'s `race_trait_generic` kind).
//!
//! ## What this closes, and why it is one mechanism rather than 115
//!
//! `cr_abilities_race.lst` declares a `Racial SLA ~ <Spell>` record for every
//! spell any race in the whole PCGen library grants as a racial spell-like
//! ability. Before this module every one of them sat at `ingested-magnitude`
//! (`race_trait_generic_table_holds_record_magnitude_not_yet_computed`): the
//! record is held by the engine's own `race_trait_generic` table, it carries
//! a real magnitude, and nothing ever computed it.
//!
//! Reading all 118 of `core_rulebook`'s ingested `Racial SLA ~ *` corpus
//! records directly (`data/corpus/core_rulebook/race_trait_generic/
//! racial_sla_*.json`, this cycle, `raw_tokens` classified by shape) found
//! ONE mechanism, not 118 pieces of per-spell content. Every record carries
//! the identical seven `DEFINE:RacialSLA_<Slug>_{LVL,SpellLVL,Times,DCMod,
//! DC,AtWill,Constant}|0` block and, for 115 of the 118, the identical
//! five-token `BONUS:VAR` chain:
//!
//! | token | verbatim | what it states |
//! |---|---|---|
//! | `BONUS:VAR` | `RacialSLA_<S>_LVL\|TL\|TYPE=Base` | caster level IS the character's total level |
//! | `BONUS:VAR` | `RacialSLA_<S>_SpellLVL\|<n>` | the spell's own level, the only per-record datum |
//! | `BONUS:VAR` | `RacialSLA_<S>_Times\|1\|TYPE=Base` | one use per day, base |
//! | `BONUS:VAR` | `RacialSLA_<S>_DCMod\|CHA\|TYPE=Base` | the DC's ability term is Charisma |
//! | `BONUS:VAR` | `RacialSLA_<S>_DC\|10+RacialSLA_<S>_SpellLVL+RacialSLA_<S>_DCMod` | the save DC |
//!
//! So the *only* thing that varies across the 115 is `spell_level`. The save
//! DC is `10 + <spell level> + <the character's real Charisma modifier>` --
//! PF1's own spell-like-ability save DC rule, stated by the corpus itself
//! rather than transcribed from a rulebook by hand. [`RACIAL_SLA_CATALOG`]
//! therefore carries one `spell_level` per record and NO per-record formula:
//! [`RACIAL_SLA_SAVE_DC_FORMULA`] is a single shared string, transcribed
//! verbatim (modulo the `<S>` slug substitution the corpus itself performs
//! per record) from those tokens.
//!
//! This is deliberately the same shape `pilot_compute::domain_power` already
//! established for domain powers -- corpus-transcribed formula strings fed to
//! the crate's real, already-proven formula evaluator -- rather than a new
//! hand-written closed form, per `OPERATOR-RULINGS-2026-08-21.md` §20.
//!
//! ## How a record is grounded (never asserted)
//!
//! [`racial_sla_save_dc_is_grounded_for_corpus_key`] builds a real
//! [`CharacterInput`], runs it through the real
//! `pilot_compute::compute_pilot_base_chassis`, takes the **computed**
//! Charisma modifier off that computation (never `(score - 10) / 2`
//! re-derived here), binds it as the `CHA` variable, and evaluates
//! [`racial_sla_save_dc_expr`] -- the record's own CONVERTED arithmetic --
//! through [`evaluate_expr_from_facts`], the same evaluator the sheet
//! renderer itself uses (SD-35 `AT-35-E6-001`, `decisions.md` §11: nothing on
//! the live side reads an ingest-format formula string). The fixture
//! deliberately carries Charisma 14
//! (`+2`), never 10 (`+0`): a zero Charisma modifier would make the formula's
//! `CHA` term unobservable, so a wrong binding would still produce the right
//! number. The independently hand-derived expectation is `12 + spell_level`,
//! computed here from the entry's own `spell_level` and the hand-derived
//! `+2`, and the function returns `None` -- never the table's own value --
//! if the evaluator disagrees with it. Same "never paper over a real
//! disagreement" discipline as `trait_effects::save_trait_magnitude_is_
//! grounded_for_corpus_key`.
//!
//! ## What this deliberately does NOT cover
//!
//! * **The 3 records with no `DCMod`/`LVL`/`Times` chain at all** (`Racial
//!   SLA ~ Dispel Magic`, `~ Divine Favor`, `~ Suggestion`,
//!   `cr_abilities_race.lst:279/215/287`). They carry the same seven
//!   `DEFINE`s and the same `DC` formula, but only a bare `SpellLVL` `BONUS`
//!   beside it -- no `DCMod|CHA` row -- so under PCGen semantics their DC
//!   resolves against the `DEFINE`'s own `0` default, giving a DC with no
//!   Charisma term at all. That is very likely an upstream `.lst` omission
//!   rather than a real rule, and shipping `10 + spell level` as this
//!   engine's answer for them would be a specific, checkable, probably-wrong
//!   DC. They stay `ingested-magnitude`, named here rather than guessed at.
//! * **Uses per day, at-will and constant.** `Times|1|TYPE=Base` is a BASE
//!   the granting race's own row is expected to raise (`TIMES=ATWILL` and
//!   `TIMEUNIT=Constant` variants live in the same record's three `SPELLS:`
//!   tokens); nothing in this engine yet reads a race's own SLA grant, so
//!   reporting "1/day" as the answer would misstate an at-will ability.
//!   Only the save DC -- the one quantity the record itself fully determines
//!   -- is grounded.
//! * **The spell's own effect.** No spell resolution engine exists in this
//!   crate; this grounds the DC a target saves against, not what happens.

use crate::rules_core::character_input::{
    AbilityScores, CharacterClassLevel, CharacterInput, ChosenCharacterState,
};
use crate::rules_core::sheet_rule::{evaluate_expr_from_facts, Ability, CharacterFacts, Expr};

/// The save-DC formula every [`RACIAL_SLA_CATALOG`] entry shares, transcribed
/// verbatim from each record's own `BONUS:VAR|RacialSLA_<S>_DC|10+RacialSLA_
/// <S>_SpellLVL+RacialSLA_<S>_DCMod` token with the two per-record variable
/// names replaced by the values the record's OWN sibling tokens bind them to
/// (`SpellLVL` -> [`RacialSlaChassis::spell_level`], `DCMod` -> `CHA`, from
/// `BONUS:VAR|RacialSLA_<S>_DCMod|CHA|TYPE=Base`). Verified byte-for-byte
/// against all 115 corpus records by
/// `tests/sd34_wave51_racial_sla_catalog_matches_the_corpus.rs`.
pub const RACIAL_SLA_SAVE_DC_FORMULA: &str = "10+SpellLVL+CHA";

/// The upstream `.lst` file every [`RACIAL_SLA_CATALOG`] entry is transcribed
/// from, and its sha256 as ingested -- provenance for the corpus fixture
/// check, never read by any runtime path.
pub const RACIAL_SLA_UPSTREAM_LST: &str =
    "pathfinder/paizo/roleplaying_game/core_rulebook/cr_abilities_race.lst";

/// One `core_rulebook` `Racial SLA ~ <Spell>` chassis record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RacialSlaChassis {
    /// The record's own corpus `KEY` token, verbatim -- the join key the
    /// work-inventory classifier looks this entry up by.
    pub corpus_key: &'static str,
    /// The `<S>` slug the record's own `RacialSLA_<S>_*` variable names use,
    /// transcribed from its `DEFINE:RacialSLA_<S>_LVL|0` token. Provenance
    /// for the corpus fixture check; the shared formula needs no slug.
    pub variable_slug: &'static str,
    /// The spell's own level, transcribed verbatim from this record's
    /// `BONUS:VAR|RacialSLA_<S>_SpellLVL|<n>` token. The ONLY datum that
    /// varies across the catalog.
    pub spell_level: i64,
    /// This record's own line in [`RACIAL_SLA_UPSTREAM_LST`].
    pub upstream_line: u64,
}

/// Every `core_rulebook` `Racial SLA ~ *` record carrying the full five-token
/// `BONUS:VAR` chain this module's doc comment tabulates -- 115 of the 118
/// ingested. See that doc comment for the 3 deliberately absent records and
/// why guessing a DC for them would be worse than leaving them open.
pub static RACIAL_SLA_CATALOG: &[RacialSlaChassis] = &[
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Aid",
        variable_slug: "Aid",
        spell_level: 2,
        upstream_line: 245,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Alter Self",
        variable_slug: "AlterSelf",
        spell_level: 2,
        upstream_line: 246,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Animate Dead",
        variable_slug: "AnimateDead",
        spell_level: 3,
        upstream_line: 276,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Animate Objects",
        variable_slug: "AnimateObjects",
        spell_level: 6,
        upstream_line: 300,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Animate Objects (Small or Smaller)",
        variable_slug: "AnimateObjectsSmallOrSmaller",
        spell_level: 6,
        upstream_line: 294,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Arcane Mark",
        variable_slug: "ArcaneMark",
        spell_level: 0,
        upstream_line: 176,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Augury",
        variable_slug: "Augury",
        spell_level: 2,
        upstream_line: 247,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Bear's Endurance",
        variable_slug: "BearsEndurance",
        spell_level: 2,
        upstream_line: 248,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Bleed",
        variable_slug: "Bleed",
        spell_level: 0,
        upstream_line: 177,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Blindness/Deafness",
        variable_slug: "BlindnessDeafness",
        spell_level: 2,
        upstream_line: 249,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Blur",
        variable_slug: "Blur",
        spell_level: 2,
        upstream_line: 250,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Burning Hands",
        variable_slug: "BurningHands",
        spell_level: 1,
        upstream_line: 200,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Calm Animals",
        variable_slug: "CalmAnimals",
        spell_level: 1,
        upstream_line: 201,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Charm Animal",
        variable_slug: "CharmAnimal",
        spell_level: 1,
        upstream_line: 202,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Charm Animal (aquatic animals only)",
        variable_slug: "CharmAnimalAquaticAnimalsOnly",
        spell_level: 1,
        upstream_line: 203,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Charm Person",
        variable_slug: "CharmPerson",
        spell_level: 1,
        upstream_line: 204,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Chill Touch",
        variable_slug: "ChillTouch",
        spell_level: 1,
        upstream_line: 205,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Command",
        variable_slug: "Command",
        spell_level: 1,
        upstream_line: 206,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Commune",
        variable_slug: "Commune",
        spell_level: 5,
        upstream_line: 295,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Commune (Week)",
        variable_slug: "Commune",
        spell_level: 5,
        upstream_line: 296,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Comprehend Languages",
        variable_slug: "ComprehendLanguages",
        spell_level: 1,
        upstream_line: 207,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Continual Flame",
        variable_slug: "ContinualFlame",
        spell_level: 2,
        upstream_line: 251,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Create Water",
        variable_slug: "CreateWater",
        spell_level: 0,
        upstream_line: 178,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Curse Water",
        variable_slug: "CurseWater",
        spell_level: 1,
        upstream_line: 208,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Dancing Lights",
        variable_slug: "DancingLights",
        spell_level: 0,
        upstream_line: 179,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Darkness",
        variable_slug: "Darkness",
        spell_level: 2,
        upstream_line: 252,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Daylight",
        variable_slug: "Daylight",
        spell_level: 3,
        upstream_line: 277,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Daze",
        variable_slug: "Daze",
        spell_level: 0,
        upstream_line: 180,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Death Knell",
        variable_slug: "DeathKnell",
        spell_level: 2,
        upstream_line: 253,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Deathwatch",
        variable_slug: "Deathwatch",
        spell_level: 1,
        upstream_line: 209,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Deeper Darkness",
        variable_slug: "DeeperDarkness",
        spell_level: 3,
        upstream_line: 278,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Detect Evil",
        variable_slug: "DetectEvil",
        spell_level: 1,
        upstream_line: 210,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Detect Magic",
        variable_slug: "DetectMagic",
        spell_level: 0,
        upstream_line: 181,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Detect Poison",
        variable_slug: "DetectPoison",
        spell_level: 0,
        upstream_line: 182,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Detect Secret Doors",
        variable_slug: "DetectSecretDoors",
        spell_level: 1,
        upstream_line: 211,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Detect Thoughts",
        variable_slug: "DetectThoughts",
        spell_level: 2,
        upstream_line: 254,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Detect Undead",
        variable_slug: "DetectUndead",
        spell_level: 1,
        upstream_line: 212,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Disguise Self",
        variable_slug: "DisguiseSelf",
        spell_level: 1,
        upstream_line: 213,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Disguise Self (humanoid only)",
        variable_slug: "DisguiseSelfHumanoidOnly",
        spell_level: 1,
        upstream_line: 214,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Displacement",
        variable_slug: "Displacement",
        spell_level: 3,
        upstream_line: 280,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Doom",
        variable_slug: "Doom",
        spell_level: 1,
        upstream_line: 216,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Dream",
        variable_slug: "Dream",
        spell_level: 5,
        upstream_line: 297,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Enlarge Person",
        variable_slug: "EnlargePerson",
        spell_level: 1,
        upstream_line: 217,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Enlarge Person (self only)",
        variable_slug: "EnlargePersonSelfOnly",
        spell_level: 1,
        upstream_line: 218,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Entangle",
        variable_slug: "Entangle",
        spell_level: 1,
        upstream_line: 219,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Erase",
        variable_slug: "Erase",
        spell_level: 1,
        upstream_line: 220,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Faerie Fire",
        variable_slug: "FaerieFire",
        spell_level: 1,
        upstream_line: 221,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Feather Fall",
        variable_slug: "FeatherFall",
        spell_level: 1,
        upstream_line: 222,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Flare",
        variable_slug: "Flare",
        spell_level: 0,
        upstream_line: 183,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Fog Cloud",
        variable_slug: "FogCloud",
        spell_level: 2,
        upstream_line: 255,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Ghost Sound",
        variable_slug: "GhostSound",
        spell_level: 0,
        upstream_line: 184,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Glitterdust",
        variable_slug: "Glitterdust",
        spell_level: 2,
        upstream_line: 256,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Goodberry",
        variable_slug: "Goodberry",
        spell_level: 1,
        upstream_line: 223,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Grease",
        variable_slug: "Grease",
        spell_level: 1,
        upstream_line: 224,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Guidance",
        variable_slug: "Guidance",
        spell_level: 0,
        upstream_line: 185,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Hideous Laughter",
        variable_slug: "HideousLaughter",
        spell_level: 2,
        upstream_line: 257,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Inflict Light Wounds",
        variable_slug: "InflictLightWounds",
        spell_level: 1,
        upstream_line: 225,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Invisibility",
        variable_slug: "Invisibility",
        spell_level: 2,
        upstream_line: 258,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Invisibility (self only)",
        variable_slug: "InvisibilitySelfOnly",
        spell_level: 2,
        upstream_line: 259,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Jump",
        variable_slug: "Jump",
        spell_level: 1,
        upstream_line: 226,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Levitate",
        variable_slug: "Levitate",
        spell_level: 2,
        upstream_line: 260,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Light",
        variable_slug: "Light",
        spell_level: 0,
        upstream_line: 186,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Lullaby",
        variable_slug: "Lullaby",
        spell_level: 0,
        upstream_line: 187,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Mage Hand",
        variable_slug: "MageHand",
        spell_level: 0,
        upstream_line: 188,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Magic Fang",
        variable_slug: "MagicFang",
        spell_level: 1,
        upstream_line: 227,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Magic Stone",
        variable_slug: "MagicStone",
        spell_level: 1,
        upstream_line: 228,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Make Whole",
        variable_slug: "MakeWhole",
        spell_level: 2,
        upstream_line: 261,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Mending",
        variable_slug: "Mending",
        spell_level: 0,
        upstream_line: 190,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Message",
        variable_slug: "Message",
        spell_level: 0,
        upstream_line: 191,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Minor Image",
        variable_slug: "MinorImage",
        spell_level: 2,
        upstream_line: 262,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Misdirection",
        variable_slug: "Misdirection",
        spell_level: 2,
        upstream_line: 263,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Nondetection",
        variable_slug: "Nondetection",
        spell_level: 3,
        upstream_line: 281,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Nondetection (self only)",
        variable_slug: "NondetectionSelfOnly",
        spell_level: 3,
        upstream_line: 282,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Obscuring Mist",
        variable_slug: "ObscuringMist",
        spell_level: 1,
        upstream_line: 229,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Open/Close",
        variable_slug: "OpenClose",
        spell_level: 0,
        upstream_line: 192,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Pass without Trace",
        variable_slug: "PassWithoutTrace",
        spell_level: 1,
        upstream_line: 230,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Plane Shift (self only/to Shadow or Material Plane)",
        variable_slug: "PlaneShiftSelfOnlyToShadowOrMaterialPlane",
        spell_level: 7,
        upstream_line: 307,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Plane Shift (to Shadow or Material Plane)",
        variable_slug: "PlaneShiftToShadowOrMaterialPlane",
        spell_level: 7,
        upstream_line: 308,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Prestidigitation",
        variable_slug: "Prestidigitation",
        spell_level: 0,
        upstream_line: 193,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Produce Flame",
        variable_slug: "ProduceFlame",
        spell_level: 1,
        upstream_line: 231,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Purify Food and Drink",
        variable_slug: "Purify Food and Drink",
        spell_level: 0,
        upstream_line: 194,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Pyrotechnics",
        variable_slug: "Pyrotechnics",
        spell_level: 2,
        upstream_line: 264,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Rage",
        variable_slug: "Rage",
        spell_level: 3,
        upstream_line: 283,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Read Magic",
        variable_slug: "ReadMagic",
        spell_level: 0,
        upstream_line: 195,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Reduce Person",
        variable_slug: "ReducePerson",
        spell_level: 1,
        upstream_line: 232,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Remove Disease",
        variable_slug: "RemoveDisease",
        spell_level: 3,
        upstream_line: 284,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Sanctuary",
        variable_slug: "Sanctuary",
        spell_level: 1,
        upstream_line: 233,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Scare",
        variable_slug: "Scare",
        spell_level: 2,
        upstream_line: 265,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ See Invisibility",
        variable_slug: "SeeInvisibility",
        spell_level: 2,
        upstream_line: 266,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Shadow Walk",
        variable_slug: "ShadowWalk",
        spell_level: 6,
        upstream_line: 301,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Shadow Walk (self only)",
        variable_slug: "ShadowWalkSelfOnly",
        spell_level: 6,
        upstream_line: 302,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Shatter",
        variable_slug: "Shatter",
        spell_level: 2,
        upstream_line: 267,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Shield Other",
        variable_slug: "ShieldOther",
        spell_level: 2,
        upstream_line: 268,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Silent Image",
        variable_slug: "SilentImage",
        spell_level: 1,
        upstream_line: 234,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Speak with Animals",
        variable_slug: "SpeakWithAnimals",
        spell_level: 1,
        upstream_line: 235,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Speak with Animals (aquatic animals only)",
        variable_slug: "SpeakWithAnimalsAquaticAnimalsOnly",
        spell_level: 1,
        upstream_line: 236,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Speak with Animals (birds or other flying animals only)",
        variable_slug: "SpeakWithAnimalsBirdsOrOtherFlyingAnimalsOnly",
        spell_level: 1,
        upstream_line: 237,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Speak with Animals (pigs and boars only)",
        variable_slug: "SpeakWithAnimalsPigsAndBoarsOnly",
        spell_level: 1,
        upstream_line: 238,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Speak with Animals (rodents only)",
        variable_slug: "SpeakWithAnimalsRodentsOnly",
        spell_level: 1,
        upstream_line: 239,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Speak with Dead",
        variable_slug: "SpeakWithDead",
        spell_level: 3,
        upstream_line: 285,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Speak with Plants",
        variable_slug: "SpeakWithPlants",
        spell_level: 3,
        upstream_line: 286,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Spider Climb",
        variable_slug: "SpiderClimb",
        spell_level: 2,
        upstream_line: 269,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Stabilize",
        variable_slug: "Stabilize",
        spell_level: 0,
        upstream_line: 196,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Stone Shape",
        variable_slug: "StoneShape",
        spell_level: 4,
        upstream_line: 291,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Stone Tell",
        variable_slug: "StoneTell",
        spell_level: 6,
        upstream_line: 303,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Summon Monster III (lantern archon only)",
        variable_slug: "SummonMonster3LanternArchonOnly",
        spell_level: 3,
        upstream_line: 288,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Summon Nature's Ally I (dolphins only)",
        variable_slug: "SummonNaturesAlly1DolphinsOnly",
        spell_level: 1,
        upstream_line: 240,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Summon Nature's Ally II",
        variable_slug: "SummonNaturesAlly2",
        spell_level: 2,
        upstream_line: 270,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Touch of Fatigue",
        variable_slug: "TouchOfFatigue",
        spell_level: 0,
        upstream_line: 197,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Unseen Servant",
        variable_slug: "UnseenServant",
        spell_level: 1,
        upstream_line: 241,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Veil (self only)",
        variable_slug: "VeilSelfOnly",
        spell_level: 6,
        upstream_line: 304,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Ventriloquism",
        variable_slug: "Ventriloquism",
        spell_level: 1,
        upstream_line: 242,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Web",
        variable_slug: "Web",
        spell_level: 2,
        upstream_line: 271,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Whispering Wind",
        variable_slug: "WhisperingWind",
        spell_level: 2,
        upstream_line: 272,
    },
    RacialSlaChassis {
        corpus_key: "Racial SLA ~ Zone of Truth",
        variable_slug: "ZoneOfTruth",
        spell_level: 2,
        upstream_line: 273,
    },];

/// The fixture this module grounds every catalog entry against: a level-1
/// Human Fighter with Charisma 14. Charisma 14 (`+2`) rather than the
/// 10-across-the-board fixture `trait_effects` uses is deliberate and
/// load-bearing -- see this module's doc comment: with Charisma 10 the
/// formula's `CHA` term contributes 0, so a wrong binding (or no binding at
/// all, had the evaluator been permitted to default an unbound variable to
/// zero, which it is not) would still produce the right number.
fn racial_sla_fixture_input() -> CharacterInput {
    CharacterInput {
        case_id: None,
        source_package_id: "sd34_wave51_racial_sla_fixture".to_owned(),
        chosen: ChosenCharacterState {
            race_id: "race:human".to_owned(),
            class_levels: vec![CharacterClassLevel {
                class_id: "class:fighter".to_owned(),
                level: 1,
            }],
            ability_scores: AbilityScores {
                strength: 10,
                dexterity: 10,
                constitution: 10,
                intelligence: 10,
                wisdom: 10,
                charisma: 14,
            },
            selected_feats: Vec::new(),
            skill_allocations: Vec::new(),
            equipment_selections: Vec::new(),
            selected_choices: Vec::new(),
            selected_traits: Vec::new(),
            spells_selected: Vec::new(),
            class_ability_activations: Vec::new(),
        },
        selection_provenance: Vec::new(),
    }
}

/// The hand-derived Charisma modifier of [`racial_sla_fixture_input`], PF1's
/// `floor((14 - 10) / 2) = +2`. Derived here by hand so the expectation this
/// module checks the engine against is genuinely independent of the engine --
/// the engine's own computed modifier is read separately, and a disagreement
/// between the two is reported as a refusal, never reconciled.
const RACIAL_SLA_FIXTURE_CHARISMA_MODIFIER: i64 = 2;

/// One record's save DC as CONVERTED arithmetic: `10 + <spell level> + Cha`.
///
/// SD-35 `AT-35-E6-001` (`decisions.md` §11). [`RACIAL_SLA_SAVE_DC_FORMULA`] is retained as the
/// converter/oracle-side transcription every corpus record was verified byte-for-byte against
/// (`tests/sd34_wave51_racial_sla_catalog_matches_the_corpus.rs`); this function is its
/// conversion, and it is what the live side evaluates. The two must state the same arithmetic --
/// `racial_sla_save_dc_expr_states_the_transcribed_formula` below pins that.
pub fn racial_sla_save_dc_expr(spell_level: i64) -> Expr {
    Expr::sum(vec![
        Expr::Const(10),
        Expr::Const(i32::try_from(spell_level).unwrap_or(0)),
        Expr::AbilityMod(Ability::Cha),
    ])
}

/// Grounds one `Racial SLA ~ <Spell>` record's save DC by really computing
/// it, or `None` for a corpus key this catalog does not carry (the 3 records
/// named in this module's doc comment, and every non-`core_rulebook` book's
/// own SLA records, which this cycle did not read).
///
/// The value returned is the spell-like ability's save DC:
/// `10 + <spell level> + <the character's computed Charisma modifier>`,
/// evaluated by the sheet renderer's own evaluator over
/// [`racial_sla_save_dc_expr`], never by arithmetic re-written here.
pub fn racial_sla_save_dc_is_grounded_for_corpus_key(corpus_key: &str) -> Option<i8> {
    let entry = RACIAL_SLA_CATALOG.iter().find(|e| e.corpus_key == corpus_key)?;

    // The REAL engine run. The Charisma modifier bound into the formula
    // below is the one this computation produced, not a re-derivation.
    let computation =
        crate::rules_core::pilot_compute::compute_pilot_base_chassis(&racial_sla_fixture_input());
    let charisma_modifier = i64::from(computation.ability_modifiers.charisma);

    // Independent cross-check: the engine's computed modifier must agree with
    // this module's own hand-derived one before either is used. A
    // disagreement is a real defect in one of the two and is refused, not
    // averaged or preferred away.
    if charisma_modifier != RACIAL_SLA_FIXTURE_CHARISMA_MODIFIER {
        return None;
    }

    let mut facts = CharacterFacts::default();
    facts.ability_mods[5] = charisma_modifier; // Cha -- `CharacterFacts::ability_mods` order
    let computed = evaluate_expr_from_facts(
        &racial_sla_save_dc_expr(entry.spell_level),
        &facts,
    )
    .trunc();

    // The hand-derived expectation, independent of both the evaluator and the
    // formula string: PF1's spell-like-ability save DC is 10 + the spell's
    // level + the caster's Charisma modifier.
    let expected = 10 + entry.spell_level + RACIAL_SLA_FIXTURE_CHARISMA_MODIFIER;
    if computed != expected {
        return None;
    }

    i8::try_from(computed).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_keys_are_unique_and_all_carry_the_racial_sla_prefix() {
        let mut seen = std::collections::BTreeSet::new();
        for entry in RACIAL_SLA_CATALOG {
            assert!(
                entry.corpus_key.starts_with("Racial SLA ~ "),
                "{:?} is not a Racial SLA record",
                entry.corpus_key
            );
            assert!(seen.insert(entry.corpus_key), "duplicate {:?}", entry.corpus_key);
        }
        assert_eq!(seen.len(), 115, "catalog size changed without re-deriving it from the corpus");
    }

    #[test]
    fn every_catalog_entry_grounds_a_save_dc_of_twelve_plus_its_spell_level() {
        for entry in RACIAL_SLA_CATALOG {
            let dc = racial_sla_save_dc_is_grounded_for_corpus_key(entry.corpus_key)
                .unwrap_or_else(|| panic!("{:?} did not ground", entry.corpus_key));
            // 10 (the rule's base) + spell level + the fixture's +2 Charisma.
            let expected = i8::try_from(12 + entry.spell_level).expect("catalog DCs fit in i8");
            assert_eq!(dc, expected, "{:?}", entry.corpus_key);
        }
    }

    #[test]
    fn a_key_the_catalog_does_not_carry_refuses_rather_than_guessing() {
        assert_eq!(racial_sla_save_dc_is_grounded_for_corpus_key("Racial SLA ~ Dispel Magic"), None);
        assert_eq!(racial_sla_save_dc_is_grounded_for_corpus_key("Racial SLA ~ Divine Favor"), None);
        assert_eq!(racial_sla_save_dc_is_grounded_for_corpus_key("Racial SLA ~ Suggestion"), None);
        assert_eq!(racial_sla_save_dc_is_grounded_for_corpus_key("Trait ~ Acrobat"), None);
    }

    #[test]
    fn the_charisma_term_is_really_read_from_the_computation() {
        // Guard on the doc comment's own claim that a +0 Charisma fixture
        // could not tell a correct binding from a missing one: with the real
        // +2 fixture, every DC is genuinely 2 higher than the Charisma-less
        // 10 + spell level.
        let entry = RACIAL_SLA_CATALOG
            .iter()
            .find(|e| e.corpus_key == "Racial SLA ~ Aid")
            .expect("Racial SLA ~ Aid is in the catalog");
        assert_eq!(entry.spell_level, 2);
        assert_eq!(racial_sla_save_dc_is_grounded_for_corpus_key(entry.corpus_key), Some(14));
    }
}
