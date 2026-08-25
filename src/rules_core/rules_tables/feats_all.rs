//! The feat catalog across every ingested rule book.
//!
//! Each book owns its own catalog module (`crb::feats`, `apg::feats`,
//! `acg::feats`, `advanced_race_guide::feats`,
//! `pathfinder_unchained::feat_tables`), exactly as each book owns its
//! own spell list. This module is the one place that joins them, tagging
//! each book's slice with the `RuleSetId` it came from so a consumer can
//! tell a player which book a feat is from without re-deriving it from
//! the key.
//!
//! Provenance lives on the *table*, not on every record, because a book
//! is a property of the whole slice -- putting a `rule_set` field on all
//! 690 records would repeat one fact 690 times and let it drift per row.
//!
//! # Why this module owns a join record instead of reusing `crb::feats::FeatTableEntry`
//!
//! CRB, APG and ACG share one type: `apg::feats` and `acg::feats` both
//! declare `use super::super::crb::feats::FeatTableEntry;`, so those
//! three books' records are literally the same struct. ARG and PU do
//! not, and widening them to reuse it was considered and rejected on
//! evidence, not on taste:
//!
//! * **ARG cannot honestly fill `prerequisites`.** `crb::feats::FeatTableEntry`
//!   documents `prerequisites: None` as "the corpus record carries no
//!   `PRE`-family token". Every single one of `arg_feats.lst`'s 187
//!   `CATEGORY:FEAT` records carries at least one (counted directly off
//!   the corpus file). ARG's ingest never gathered them, so reusing the
//!   shared type would force `None` onto 187 records for which that
//!   statement is false -- one fabricated absence per record, which is
//!   precisely what `effect`'s and `description`'s own
//!   "`Some(&[])` never occurs" rules exist to prevent.
//! * **PU's categories are not `TYPE:`-facet categories at all.**
//!   `crb::feats::FeatCategory`'s six variants are documented as derived
//!   from the corpus `TYPE:` facet. `pu_feats.lst` has no usable one: 9
//!   of its 17 real records (the "Champion of ..." alignment feats)
//!   carry no `TYPE:` token whatsoever, so `pathfinder_unchained` derives
//!   its categories from the file's own `###Block:` markers instead
//!   (`Alignment`, `CombatStamina`, `WoundThreshold`, plus `General`).
//!   None of `Alignment`/`CombatStamina`/`WoundThreshold` exists in the
//!   shared enum, and mapping them onto a variant that does exist would
//!   invent a classification the corpus never made.
//! * **PU would also have to fabricate `effect` and `prerequisites`.**
//!   Its ingest carries neither, yet its records genuinely have both in
//!   the corpus (e.g. `Combat Stamina` carries
//!   `BONUS:VAR|StaminaPool|BAB+CON` and `PRETOTALAB:1`), and it carries
//!   a `source_page` the shared type has no field for.
//!
//! So this module follows the precedent `spell_catalog.rs` already set
//! for exactly this situation -- differing per-book record types joined
//! by per-book map functions into one shared record carrying a book tag
//! (`map_crb_entry` / `map_apg_entry` / `map_acg_entry` there). The join
//! record here is [`FeatCatalogRecord`].
//!
//! # What the join record carries, and what it deliberately does not
//!
//! [`FeatCatalogRecord`] carries `key`, `category`, `name` and
//! `description` -- the four facets *every* ingested book's feat table
//! actually holds, and the exact four every consumer of
//! [`all_feat_tables`] reads today (the desktop Feat picker's DTO, the
//! zero-magnitude description check, the feat-identity collision proof,
//! and the two inventory binaries).
//!
//! It does **not** carry `effect` or `source_page`. Those are per-book
//! facets only some books ingested, and hoisting them here would have to
//! represent "this book's ingest does not carry this field" and "this
//! corpus record has no such token" with the same `None` -- two different
//! facts collapsed into one value, which is the same ambiguity the
//! per-book types each refuse. A consumer that needs ARG's `effect` or
//! PU's `source_page` reads that book's own table directly.
//!
//! # `prerequisites` used to be on that list. It is not any more.
//!
//! The argument above was the *only* reason this record refused to carry
//! `prerequisites`, and it rested on a fact that has since been fixed:
//! ARG's and PU's ingests never gathered their `PRE`-family tokens, so a
//! `prerequisites` field here would have had to mean two things at once.
//! [`ARG_FEAT_PREREQUISITES`] / [`PU_FEAT_PREREQUISITES`] closes that gap -- it gathers all 187 ARG rows' and all
//! 17 PU rows' top-level `PRE` tokens from the corpus, listing every key
//! including the ones whose corpus record genuinely has none. With the
//! data actually present for all five books, `Option<&[&str]>` here means
//! exactly what it means on `crb::feats::FeatTableEntry.prerequisites`:
//! `None` is "this corpus record carries no `PRE`-family token", never
//! "nobody looked".
//!
//! This matters because there was **no feat prerequisite enforcement
//! anywhere in the product**: a Fighter 1 with a +1 base attack bonus
//! could take Improved Two-Weapon Fighting (which requires BAB +6, Dex 17
//! and the Two-Weapon Fighting feat), and every one of the 690 offered
//! feats was accepted by every character. `feat_prereqs` evaluates these
//! tokens; it can only do so if the aggregate the picker reads actually
//! carries them.
//!
//! `category` is the book's own `FeatCategory` variant name verbatim, as
//! a `&'static str`. The three enums are genuinely different closed sets
//! over different corpus signals, so there is no single enum to project
//! onto; and the one consumer that renders a category
//! (`feat_catalog.rs`) already stringified it with `format!("{:?}", ..)`
//! at its DTO boundary, so nothing is lost by doing it here instead.
//! Each `*_category_name` function below is an exhaustive `match`
//! returning a literal -- a new variant in any book is a compile error,
//! not a silently mislabelled row -- and
//! `category_names_match_the_debug_form_of_every_variant` pins each
//! literal to that variant's `Debug` form so the wire strings the
//! frontend filters on cannot drift.
//!
//! # Key collisions
//!
//! Feat keys were globally unique across CRB/APG/ACG. **They are not
//! unique once PU is included:** `Endurance` is in both `cr_feats.lst`
//! and `pu_feats.lst`. Checked against both corpus rows rather than
//! assumed: PU's is the Core Rulebook feat *re-listed* under Pathfinder
//! Unchained's Wound Threshold rules, not a second feat that happens to
//! share a name. The two rows carry the same name and the same `DESC:`
//! text; they differ in `TYPE:` (`General` vs `Wound Threshold`),
//! `SOURCEPAGE:` (`p.112` vs `P.137`) and `BENEFIT:` (PU's adds the
//! wound-penalty reduction) -- and `BENEFIT:` is a token neither book's
//! table ingests.
//!
//! Both rows are kept, in the same not-deduplicated posture `crb::feats`
//! takes with its two real `Combat Expertise` records: dropping PU's
//! would make this table disagree with `pu_feats::feat_tables()` about
//! how many records that book has. The collision set is pinned by
//! `cross_book_key_collisions_are_exactly_the_known_set` below so a
//! genuinely different second feat arriving under an existing key is a
//! test failure rather than a silent shadowing.

use super::advanced_race_guide::feats as arg_feats;
use super::crb::feats::{FeatCategory as SharedFeatCategory, FeatTableEntry as SharedFeatTableEntry};
use super::pathfinder_unchained::feat_tables as pu_feats;
use super::ultimate_campaign::feat_tables as uca_feats;
use super::ultimate_intrigue::feat_tables as ui_feats;
use super::ultimate_combat::feat_tables as uc_feats;
use super::ultimate_magic::feat_tables as um_feats;
use super::ultimate_psionics::feat_tables as upsi_feats;
use super::ultimate_wilderness::feat_tables as uw_feats;
use super::RuleSetId;

/// One feat record, projected out of whichever per-book table it came
/// from. See this module's own doc comment for why the four fields are
/// these four and why `category` is a string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FeatCatalogRecord {
    /// The record's corpus identity -- its `KEY:` token when its row
    /// carries one, else its display name. Each book's own table already
    /// applies that fallback; this is that value, unchanged.
    ///
    /// Not unique across books -- see this module's "Key collisions"
    /// section.
    pub key: &'static str,
    /// The source book's own `FeatCategory` variant name, verbatim (e.g.
    /// `"Combat"`, `"Panache"`, `"WoundThreshold"`).
    pub category: &'static str,
    pub name: &'static str,
    /// The corpus `DESC:` token, verbatim; `None` when the record has
    /// none. Passed through from the book's own table, which already
    /// applies that rule -- never substituted or borrowed from a sibling
    /// record.
    pub description: Option<&'static str>,
    /// Every top-level `PRE`-family token the corpus record carries,
    /// verbatim and unparsed, in source order -- `PREABILITY:`,
    /// `PREMULT:`, `PRESTAT:`, `PRETOTALAB:`, `PRESKILL:`, `PREFACT:`,
    /// the negated `!PREABILITY:` form, and the rest.
    ///
    /// `None` when the corpus record carries none -- 91 of the catalog's
    /// 690 records (55 CRB, 29 APG, 4 ACG, 0 ARG, 3 PU). `Some(&[])`
    /// never occurs, mirroring
    /// `crb::feats::FeatTableEntry.prerequisites`'s own rule, from which
    /// the CRB/APG/ACG values are passed through unchanged. ARG's and
    /// PU's come from [`ARG_FEAT_PREREQUISITES`] / [`PU_FEAT_PREREQUISITES`], which gathers what those books'
    /// own tables never did.
    ///
    /// Deliberately raw strings, not a parsed prerequisite AST, for the
    /// same reason the per-book field keeps them raw: these are PCGen
    /// expressions over runtime character state. `feat_prereqs` is the
    /// one place that interprets them, and it is hand-modelled per token
    /// kind (`decisions.md` §24) rather than being a general formula
    /// interpreter.
    pub prerequisites: Option<&'static [&'static str]>,
}

/// One book's feat catalog, tagged with the book it came from.
#[derive(Debug, Clone, Copy)]
pub struct BookFeatTable {
    pub rule_set: RuleSetId,
    pub entries: &'static [FeatCatalogRecord],
}

// ---------------------------------------------------------------------------
// The `PRE`-family tokens ARG's and PU's own tables never gathered
// ---------------------------------------------------------------------------
// The `PRE`-family prerequisite tokens for the two books whose own feat
// tables never gathered them: Advanced Race Guide and Pathfinder
// Unchained.
//
// # Why this module exists at all
//
// `rules_tables::crb::feats::FeatTableEntry.prerequisites` carries every
// top-level `PRE`-family token, verbatim, for CRB/APG/ACG -- the three
// books that share that type. ARG and PU declare their own record types
// (see `feats_all`'s module doc for why they could not be widened onto the
// shared one) and neither carries a `prerequisites` field. That is not a
// statement about their corpus rows. Counted directly off the corpus:
// **all 187 of `arg_feats.lst`'s `CATEGORY:FEAT` records carry at least
// one `PRE`-family token**, and 14 of `pu_feats.lst`'s 17 do. Without this
// module those 201 real prerequisites do not exist anywhere in the engine,
// and a prerequisite checker reading only the shared field would silently
// report every ARG feat as unconditionally available.
//
// # Where the data came from, and how it is checked
//
// Extracted programmatically from the live corpus
// (`pathfinder/paizo/roleplaying_game/advanced_race_guide/arg_feats.lst`
// and `.../pathfinder_unchained/pu_feats.lst`) by the same offline method
// `crb/feat_data/` documents -- not hand-transcribed, so there is no
// per-row transcription risk at this scale. The extractor was validated
// before use by re-running it over CRB, APG and ACG and comparing against
// the `prerequisites` those books already carry: **485 of 486 rows matched
// byte-for-byte**, the single difference being CRB's two `Combat
// Expertise` records (a genuine duplicate key; the extractor keeps the
// first row, the table keeps both).
//
// `tests/sd27_feat_prerequisite_enforcement.rs` re-derives this table from
// the on-disk corpus whenever `PCGEN_CORPUS_ROOT` is set, exactly as the
// other hand-modelled corpus tables in this crate are gated, so a drift
// between this file and the corpus is a named test failure rather than a
// silent wrong answer.
//
// # Shape
//
// `&[(key, tokens)]` keyed by the book table's own `key`, in that table's
// own source order. Every key in the book's table appears here exactly
// once, including the rows whose corpus record carries no `PRE` token at
// all -- those map to an empty slice. An empty slice here means "checked,
// and the corpus row has none"; a *missing* key would mean "not gathered",
// and `every_arg_and_pu_catalog_key_has_a_gathered_prerequisite_row` in
// `feats_all` fails if one ever is. That is the same distinction
// `FeatTableEntry.effect`'s `None`-vs-`Some(&[])` rule draws, expressed
// for a lookup table rather than a per-record field.

/// Advanced Race Guide: all 187 `CATEGORY:FEAT` records from
/// `arg_feats.lst`, in `advanced_race_guide::feats::feat_tables()` order
/// (General, then Combat, then Teamwork).
pub const ARG_FEAT_PREREQUISITES: &[(&str, &[&str])] = &[
    ("Adaptive Fortune", &["PREABILITY:1,CATEGORY=Special Ability,Halfling ~ Adaptable Luck", "PREABILITY:1,CATEGORY=FEAT,Fortunate One", "PRELEVEL:MIN=10", "PREFACT:1,TEMPLATES,IsHalfling=true"]),
    ("Agile Tongue", &["PREFACT:1,TEMPLATES,IsGrippli=true"]),
    ("Airy Step", &["PREFACT:1,TEMPLATES,IsSylph=true"]),
    ("Angel Wings", &["PREABILITY:1,CATEGORY=FEAT,Angelic Blood", "PREPCLEVEL:MIN=10", "PREFACT:1,TEMPLATES,IsAasimar=true"]),
    ("Angelic Blood", &["PREFACT:1,TEMPLATES,IsAasimar=true", "PRESTAT:1,CON=13"]),
    ("Angelic Flesh", &["PREABILITY:1,CATEGORY=FEAT,Angelic Blood", "PREFACT:1,TEMPLATES,IsAasimar=true"]),
    ("Aquatic Ancestry", &["PREFACT:1,TEMPLATES,IsUndine=true"]),
    ("Armor of the Pit", &["PREFACT:1,TEMPLATES,IsTiefling=true"]),
    ("Attuned to the Wild", &["PREFACT:1,TEMPLATES,IsElf=true"]),
    ("Beast Rider", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Companion,TYPE.Special Mount", "PRELEVEL:MIN=7", "PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true"]),
    ("Bestow Luck", &["PREABILITY:2,CATEGORY=FEAT,Defiant Luck,Inexplicable Luck", "PREFACT:1,TEMPLATES,IsHuman=true"]),
    ("Black Cat", &["PREFACT:1,TEMPLATES,IsCatfolk=true"]),
    ("Blood Drinker", &["PREFACT:1,TEMPLATES,IsDhampir=true"]),
    ("Blood Feaster", &["PREABILITY:1,CATEGORY=FEAT,Blood Drinker", "PREFACT:1,TEMPLATES,IsDhampir=true", "PRETOTALAB:6"]),
    ("Blood Salvage", &["PREABILITY:1,CATEGORY=FEAT,Blood Drinker", "PREFACT:1,TEMPLATES,IsDhampir=true"]),
    ("Blood Vengeance", &["!PREALIGN:LG,LN,LE", "PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true"]),
    ("Born Alone", &["PREFACT:1,TEMPLATES,IsOrc=true"]),
    ("Brewmaster", &["PREFACT:1,TEMPLATES,IsDwarf=true", "PRESKILL:2,Craft (Alchemy)=1,Profession (Brewer)=1"]),
    ("Burn! Burn! Burn!", &["PREFACT:1,TEMPLATES,IsGoblin=true", "PRESKILL:1,Disable Device=1"]),
    ("Burrowing Teeth", &["PREABILITY:2,CATEGORY=FEAT,Sharpclaw,Tunnel Rat", "PREFACT:1,TEMPLATES,IsRatfolk=true"]),
    ("Carrion Feeder", &["PREFACT:1,TEMPLATES,IsTengu=true"]),
    ("Casual Illusionist", &["PREABILITY:1,CATEGORY=Special Ability,Gnome ~ Gnome Magic", "PREFACT:1,TEMPLATES,IsGnome=true"]),
    ("Catfolk Exemplar", &["PREFACT:1,TEMPLATES,IsCatfolk=true"]),
    ("Celestial Servant", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Companion,TYPE.Special Mount,TYPE.Familiar", "PREFACT:1,TEMPLATES,IsAasimar=true"]),
    ("Channel Force", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Energy", "PREMULT:1,[PREVARGTEQ:OracleChannelDice,2],[PREVARGTEQ:ClericChannelPositiveEnergyDice,2],[PREVARGTEQ:PaladinChannelDice,2],[PREVARGTEQ:ShamanChannelDice,2]", "PREFACT:1,TEMPLATES,IsAasimar=true", "PRETEXT:channel energy 2d6"]),
    ("Cloud Gazer", &["PREFACT:1,TEMPLATES,IsSylph=true"]),
    ("Courageous Resolve", &["PREABILITY:2,CATEGORY=Special Ability,Halfling ~ Craven,Halfling ~ Fearless", "PREFACT:1,TEMPLATES,IsHalfling=true"]),
    ("Dark Sight", &["PREABILITY:1,CATEGORY=FEAT,Gloom Sight", "PREFACT:1,TEMPLATES,IsFetchling=true"]),
    ("Dauntless Destiny", &["PREABILITY:1,CATEGORY=FEAT,Fearless Curiosity", "PREFACT:1,TEMPLATES,IsHuman=true", "PRESKILL:1,Intimidate=10", "PRESTAT:1,CHA=13"]),
    ("Deafening Explosion", &["PREABILITY:1,CATEGORY=Special Ability,Bomb", "PREFACT:1,TEMPLATES,IsHobgoblin=true"]),
    ("Defiant Luck", &["PREFACT:1,TEMPLATES,IsHuman=true"]),
    ("Discerning Eye", &["PREABILITY:1,CATEGORY=Special Ability,Half-Elf ~ Keen Senses", "PREFACT:1,TEMPLATES,IsElf=true,IsHalfElf=true"]),
    ("Diverse Palate", &["PREABILITY:1,CATEGORY=FEAT,Blood Drinker", "PREFACT:1,TEMPLATES,IsDhampir=true"]),
    ("Draconic Aspect", &["PREFACT:1,TEMPLATES,IsKobold=true"]),
    ("Draconic Breath", &["PREABILITY:1,CATEGORY=FEAT,Draconic Aspect", "PREFACT:1,TEMPLATES,IsKobold=true"]),
    ("Draconic Glide", &["PREABILITY:1,CATEGORY=FEAT,Draconic Aspect", "PREFACT:1,TEMPLATES,IsKobold=true"]),
    ("Draconic Paragon", &["PREABILITY:2,CATEGORY=FEAT,Draconic Breath,Draconic Glide", "PREABILITY:1,CATEGORY=FEAT,Draconic Aspect", "PRELEVEL:MIN=10", "PREFACT:1,TEMPLATES,IsKobold=true"]),
    ("Drow Nobility", &["PREABILITY:1,CATEGORY=Special Ability,Drow ~ Spell-Like Abilities", "PREFACT:1,TEMPLATES,IsDrow=True"]),
    ("Drow ~ Spider Step", &["PREFACT:1,TEMPLATES,IsDrow=True", "PRELEVEL:MIN=3"]),
    ("Dwarf Blooded", &["PREFACT:1,TEMPLATES,IsOread=true"]),
    ("Echoes of Stone", &["PREFACT:1,TEMPLATES,IsOread=true"]),
    ("Elemental Jaunt", &["PRELEVEL:MIN=15", "PREFACT:1,TEMPLATES,IsIfrit=true,IsOread=true,IsSylph=true,IsUndine=true"]),
    ("Elven Spirit", &["!PREABILITY:1,CATEGORY=FEAT,Human Spirit", "PREPCLEVEL:MAX=1", "PREFACT:1,TEMPLATES,IsHalfElf=true"]),
    ("Exile's Path", &["PREFACT:1,TEMPLATES,IsHalfElf=true"]),
    ("Expanded Fiendish Resistance (Acid)", &["!PREABILITY:1,CATEGORY=Special Ability,Resistance to Acid", "PREFACT:1,TEMPLATES,IsTiefling=true"]),
    ("Expanded Fiendish Resistance (Cold)", &["!PREABILITY:1,CATEGORY=Special Ability,Resistance to Cold", "PREFACT:1,TEMPLATES,IsTiefling=true"]),
    ("Expanded Fiendish Resistance (Electricity)", &["!PREABILITY:1,CATEGORY=Special Ability,Resistance to Electricity", "PREFACT:1,TEMPLATES,IsTiefling=true"]),
    ("Expanded Fiendish Resistance (Fire)", &["!PREABILITY:1,CATEGORY=Special Ability,Resistance to Fire", "PREFACT:1,TEMPLATES,IsTiefling=true"]),
    ("Expanded Resistance", &["PREABILITY:1,CATEGORY=Special Ability,Gnome ~ Illusion Resistance", "PREFACT:1,TEMPLATES,IsGnome=true"]),
    ("Extra Elemental Assault", &["PREFACT:1,TEMPLATES,IsSuli=true"]),
    ("Fast Learner", &["PREFACT:1,TEMPLATES,IsHuman=true", "PRESTAT:1,INT=13"]),
    ("Fearless Curiosity", &["PREFACT:1,TEMPLATES,IsHuman=true", "PRESTAT:1,CHA=13"]),
    ("Feline Grace", &["PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PREFACT:1,TEMPLATES,IsCatfolk=true"]),
    ("Ferocious Action", &["PREABILITY:1,CATEGORY=Special Ability,Orc ~ Ferocity", "PREFACT:1,TEMPLATES,IsOrc=true"]),
    ("Ferocious Resolve", &["PREABILITY:1,CATEGORY=Special Ability,Half-Orc ~ Orc Ferocity", "PREFACT:1,TEMPLATES,IsHalfOrc=true", "PRESTAT:1,CON=13"]),
    ("Ferocious Summons", &["PREABILITY:2,CATEGORY=FEAT,Augment Summoning,Spell Focus (Conjuration)", "PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true"]),
    ("Fiend Sight", &["PREFACT:1,TEMPLATES,IsTiefling=true", "PREVARLT:FiendSightTier,2", "PREVISION:1,Darkvision=60"]),
    ("Fire Tamer", &["PREFACT:1,TEMPLATES,IsGoblin=true"]),
    ("Firesight", &["PREFACT:1,TEMPLATES,IsIfrit=true"]),
    ("Flame Heart", &["PREABILITY:1,CATEGORY=FEAT,Fire Tamer", "PRELEVEL:MIN=5", "PREFACT:1,TEMPLATES,IsGoblin=true"]),
    ("Foment the Blood", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Energy", "PREFACT:1,TEMPLATES,IsOrc=true"]),
    ("Fortunate One", &["PREABILITY:1,CATEGORY=Special Ability,Halfling ~ Adaptable Luck", "PREFACT:1,TEMPLATES,IsHalfling=true"]),
    ("Giant Steps", &["PREABILITY:1,CATEGORY=Special Ability,Duergar ~ Slow and Steady", "PREFACT:1,TEMPLATES,IsDuergar=true"]),
    ("Gloom Sight", &["PREFACT:1,TEMPLATES,IsFetchling=true"]),
    ("Gore Fiend", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Rage", "PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true"]),
    ("Grasping Tail", &["PREFACT:1,TEMPLATES,IsTiefling=true"]),
    ("Greater Channel Force", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Energy", "PREABILITY:2,CATEGORY=FEAT,Channel Force,Improved Channel Force", "PREMULT:1,[PREVARGTEQ:OracleChannelDice,6],[PREVARGTEQ:ClericChannelPositiveEnergyDice,6],[PREVARGTEQ:PaladinChannelDice,6],[PREVARGTEQ:ShamanChannelDice,6]", "PREFACT:1,TEMPLATES,IsAasimar=true", "PRETEXT:channel energy 6d6"]),
    ("Greater Drow Nobility", &["PREABILITY:1,CATEGORY=Special Ability,Drow ~ Spell-Like Abilities", "PREABILITY:2,CATEGORY=FEAT,Drow Nobility,Improved Drow Nobility", "PREFACT:1,TEMPLATES,IsDrow=True", "PRESTAT:1,CHA=13"]),
    ("Guardian of the Wild", &["PREABILITY:1,CATEGORY=FEAT,Attuned to the Wild", "PREFACT:1,TEMPLATES,IsElf=true"]),
    ("Half-Drow Paragon", &["PREABILITY:2,CATEGORY=Special Ability,Half-Elf ~ Drow Blooded,Half-Elf ~ Drow Magic", "PREFACT:1,TEMPLATES,IsHalfElf=true"]),
    ("Heavenly Radiance", &["PREABILITY:1,CATEGORY=Special Ability,Aasimar ~ Spell-Like Ability", "PREFACT:1,TEMPLATES,IsAasimar=true"]),
    ("Heroic Will", &["PREABILITY:1,CATEGORY=FEAT,Iron Will", "PRECHECKBASE:1,Will=4", "PREFACT:1,TEMPLATES,IsHuman=true"]),
    ("Hobgoblin Discipline", &["PREFACT:1,TEMPLATES,IsHobgoblin=true", "PRETOTALAB:1"]),
    ("Human Spirit", &["PREPCLEVEL:MAX=1", "PREFACT:1,TEMPLATES,IsHalfElf=true"]),
    ("Huntmaster", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Companion,TYPE.Special Mount", "!PREABILITY:1,CATEGORY=FEAT,Huntmaster", "PREFACT:1,TEMPLATES,IsHuman=true", "PRESKILL:1,Handle Animal=1"]),
    ("Hydraulic Maneuver", &["PREABILITY:1,CATEGORY=Special Ability,Undine ~ Spell-Like Ability", "PREFACT:1,TEMPLATES,IsUndine=true"]),
    ("Improved Channel Force", &["PREABILITY:1,CATEGORY=FEAT,Channel Force", "PREMULT:1,[PREVARGTEQ:OracleChannelDice,4],[PREVARGTEQ:ClericChannelPositiveEnergyDice,4],[PREVARGTEQ:PaladinChannelDice,4],[PREVARGTEQ:ShamanChannelDice,4]", "PREFACT:1,TEMPLATES,IsAasimar=true", "PRETEXT:channel energy 4d6"]),
    ("Improved Dark Sight", &["PREABILITY:2,CATEGORY=FEAT,Dark Sight,Gloom Sight", "PREFACT:1,TEMPLATES,IsFetchling=true"]),
    ("Improved Drow Nobility", &["PREABILITY:1,CATEGORY=Special Ability,Drow ~ Spell-Like Abilities", "PREABILITY:1,CATEGORY=FEAT,Drow Nobility", "PREFACT:1,TEMPLATES,IsDrow=True", "PRESTAT:1,CHA=13"]),
    ("Improved Improvisation", &["PREABILITY:2,CATEGORY=FEAT,Fast Learner,Improvisation", "PREFACT:1,TEMPLATES,IsHuman=true", "PRESTAT:1,INT=13"]),
    ("Improved Umbral Scion", &["PREABILITY:1,CATEGORY=Special Ability,Drow ~ Spell-Like Abilities", "PREABILITY:4,CATEGORY=FEAT,Drow Nobility,Greater Drow Nobility,Improved Drow Nobility,Umbral Scion", "PREFACT:1,TEMPLATES,IsDrow=True", "PRESTAT:2,CHA=13,WIS=13"]),
    ("Improvisation", &["PREABILITY:1,CATEGORY=FEAT,Fast Learner", "PREFACT:1,TEMPLATES,IsHuman=true", "PRESTAT:1,INT=13"]),
    ("Incremental Elemental Assault", &["PREFACT:1,TEMPLATES,IsSuli=true"]),
    ("Inexplicable Luck", &["PREABILITY:1,CATEGORY=FEAT,Defiant Luck", "PREFACT:1,TEMPLATES,IsHuman=true"]),
    ("Inner Breath", &["PRELEVEL:MIN=11", "PREFACT:1,TEMPLATES,IsSylph=true"]),
    ("Intimidating Confidence", &["PREABILITY:1,CATEGORY=FEAT,Fearless Curiosity", "PREFACT:1,TEMPLATES,IsHuman=true", "PRESKILL:1,Intimidate=5", "PRESTAT:1,CHA=13"]),
    ("Ledge Walker", &["PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Mountaineer,Dwarf ~ Stability", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PREFACT:1,TEMPLATES,IsDwarf=true"]),
    ("Life's Blood", &["PREFACT:1,TEMPLATES,IsSamsaran=true"]),
    ("Lingering Invisibility", &["PREFACT:1,TEMPLATES,IsDuergar=true"]),
    ("Long-Nose Form", &["PRELEVEL:MIN=3", "PREFACT:1,TEMPLATES,IsTengu=true"]),
    ("Lucky Healer", &["PREABILITY:1,CATEGORY=Special Ability,Halfling ~ Adaptable Luck", "PREFACT:1,TEMPLATES,IsHalfling=true"]),
    ("Mage of the Wild", &["PREABILITY:1,CATEGORY=FEAT,Attuned to the Wild", "PREFACT:1,TEMPLATES,IsElf=true"]),
    ("Magical Tail", &["PREFACT:1,TEMPLATES,IsKitsune=true", "PREVARLT:KitsuneTails,8"]),
    ("Metallic Wings", &["PREABILITY:3,CATEGORY=FEAT,Angel Wings,Angelic Blood,Angelic Flesh", "PRELEVEL:MIN=11", "PREFACT:1,TEMPLATES,IsAasimar=true"]),
    ("Mother's Gift", &["PREFACT:1,TEMPLATES,IsChangeling=true"]),
    ("Multitalented Mastery", &["PREABILITY:1,CATEGORY=Special Ability,Half-Elf ~ Multitalented", "PRELEVEL:MIN=5", "PREFACT:1,TEMPLATES,IsHalfElf=true"]),
    ("Murmurs of Earth", &["PREABILITY:1,CATEGORY=FEAT,Echoes of Stone", "PRELEVEL:MIN=9", "PREFACT:1,TEMPLATES,IsOread=true"]),
    ("Natural Charmer", &["PREFACT:1,TEMPLATES,IsDhampir=true", "PRESTAT:1,CHA=17"]),
    ("Neither Elf nor Human", &["PREABILITY:2,CATEGORY=FEAT,Exile's Path,Seen and Unseen", "PRELEVEL:MIN=11", "PREFACT:1,TEMPLATES,IsHalfElf=true"]),
    ("Noble Spell Resistance", &["PREABILITY:1,CATEGORY=FEAT,Greater Drow Nobility", "PREFACT:1,TEMPLATES,IsDrow=True", "PRELEVEL:MIN=13", "PRESTAT:2,CHA=13,WIS=13"]),
    ("Oread Burrower", &["PREABILITY:1,CATEGORY=FEAT,Stony Step", "PRELEVEL:MIN=9", "PREFACT:1,TEMPLATES,IsOread=true"]),
    ("Oread Earth Glider", &["PREABILITY:2,CATEGORY=FEAT,Oread Burrower,Stony Step", "PRELEVEL:MIN=13", "PREFACT:1,TEMPLATES,IsOread=true"]),
    ("Realistic Likeness", &["PREFACT:1,TEMPLATES,IsKitsune=true"]),
    ("Resilient Brute", &["PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true"]),
    ("Resolute Rager", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Rage", "PREFACT:1,TEMPLATES,IsOrc=true"]),
    ("Scavenger's Eye", &["PREFACT:1,TEMPLATES,IsTengu=true"]),
    ("Seen and Unseen", &["PREABILITY:1,CATEGORY=FEAT,Exile's Path", "PRELEVEL:MIN=5", "PREFACT:1,TEMPLATES,IsHalfElf=true"]),
    ("Shadow Caster", &["PREFACT:1,TEMPLATES,IsDrow=True", "PREMULT:1,[PRECLASS:1,SPELLCASTER=1],[PREVARGTEQ:CasterLevel_Highest,1]"]),
    ("Shadow Ghost", &["PREABILITY:1,CATEGORY=Special Ability,Fetchling ~ Spell-Like Abilities", "PREPCLEVEL:MIN=9", "PREFACT:1,TEMPLATES,IsFetchling=true"]),
    ("Shadow Walker", &["PREABILITY:1,CATEGORY=Special Ability,Fetchling ~ Spell-Like Abilities", "PREPCLEVEL:MIN=9", "PREFACT:1,TEMPLATES,IsFetchling=true"]),
    ("Shadowy Dash", &["PREFACT:1,TEMPLATES,IsWayang=true"]),
    ("Shared Manipulation", &["PREFACT:1,TEMPLATES,IsHalfElf=true", "PRESTAT:1,CHA=13"]),
    ("Sleep Venom", &["PREFACT:1,TEMPLATES,IsVishkanya=true"]),
    ("Spider Summoner", &["PREFACT:1,TEMPLATES,IsDrow=True", "PRESPELL:1,Summon Monster I,Summon Monster II,Summon Monster III,Summon Monster IV,Summon Monster V,Summon Monster VI,Summon Monster VII,Summon Monster VIII,Summon Monster IX,Summon Nature's Ally I,Summon Nature's Ally II,Summon Nature's Ally III,Summon Nature's Ally IV,Summon Nature's Ally V,Summon Nature's Ally VI,Summon Nature's Ally VII,Summon Nature's Ally VIII,Summon Nature's Ally IX"]),
    ("Spirit of the Wild", &["PREABILITY:2,CATEGORY=FEAT,Attuned to the Wild,Guardian of the Wild", "PREFACT:1,TEMPLATES,IsElf=true"]),
    ("Steam Caster", &["PREFACT:1,TEMPLATES,IsUndine=true"]),
    ("Stoic Pose", &["PREFACT:1,TEMPLATES,IsSvirfneblin=true"]),
    ("Stony Step", &["PREFACT:1,TEMPLATES,IsOread=true"]),
    ("Stretched Wings", &["PREABILITY:1,CATEGORY=Special Ability,Strix ~ Wing-Clipped", "PREABILITY:1,CATEGORY=FEAT,Skill Focus (Fly)", "PREFACT:1,TEMPLATES,IsStrix=true", "PRESTAT:1,STR=13"]),
    ("Sure and Fleet", &["PREABILITY:1,CATEGORY=Special Ability,Halfling ~ Fleet Of Foot", "PREFACT:1,TEMPLATES,IsHalfling=true"]),
    ("Surge of Success", &["PREFACT:1,TEMPLATES,IsHuman=true"]),
    ("Tenacious Survivor", &["PREABILITY:2,CATEGORY=FEAT,Diehard,Endurance", "PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true", "PRESTAT:1,CON=13"]),
    ("Tengu Raven Form", &["PREABILITY:1,CATEGORY=FEAT,Tengu Wings", "PRELEVEL:MIN=7", "PREFACT:1,TEMPLATES,IsTengu=true"]),
    ("Tengu Wings", &["PRELEVEL:MIN=5", "PREFACT:1,TEMPLATES,IsTengu=true"]),
    ("Thrill of the Kill", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Rage", "PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true"]),
    ("Toxic Recovery", &["PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Hardy", "PREFACT:1,TEMPLATES,IsDwarf=true"]),
    ("Trap Wrecker", &["PREABILITY:1,CATEGORY=FEAT,Power Attack", "PREFACT:1,TEMPLATES,IsOrc=true", "PRESKILL:1,Disable Device=1"]),
    ("Triton Portal", &["PREABILITY:1,CATEGORY=Special Ability,Undine ~ Spell-Like Ability", "PRELEVEL:MIN=5", "PREFACT:1,TEMPLATES,IsUndine=true"]),
    ("Tunnel Rat", &["PREABILITY:1,CATEGORY=Special Ability,Ratfolk ~ Swarming", "PREFACT:1,TEMPLATES,IsRatfolk=true"]),
    ("Umbral Scion", &["PREABILITY:1,CATEGORY=Special Ability,Drow ~ Spell-Like Abilities", "PREABILITY:3,CATEGORY=FEAT,Drow Nobility,Greater Drow Nobility,Improved Drow Nobility", "PREFACT:1,TEMPLATES,IsDrow=True", "PRESTAT:2,CHA=13,WIS=13"]),
    ("Water Skinned", &["PREFACT:1,TEMPLATES,IsUndine=true"]),
    ("Wings of Air", &["PREABILITY:1,CATEGORY=FEAT,Airy Step", "PRELEVEL:MIN=9", "PREFACT:1,TEMPLATES,IsSylph=true"]),
    ("Blazing Aura", &["PREABILITY:2,CATEGORY=FEAT,Inner Flame,Scorching Weapons", "PRELEVEL:MIN=13", "PREFACT:1,TEMPLATES,IsIfrit=true"]),
    ("Blistering Feint", &["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Feint", "PREFACT:1,TEMPLATES,IsIfrit=true"]),
    ("Blood Beak", &["PREABILITY:1,CATEGORY=Special Ability,Tengu ~ Natural Weapon", "PREFACT:1,TEMPLATES,IsTengu=true", "PRETOTALAB:5"]),
    ("Blundering Defense", &["PREABILITY:1,CATEGORY=FEAT,Cautious Fighter", "PREFACT:1,TEMPLATES,IsHalfling=true"]),
    ("Bullying Blow", &["PREFACT:1,TEMPLATES,IsOrc=true", "PRESKILL:1,Intimidate=1"]),
    ("Cautious Fighter", &["PREFACT:1,TEMPLATES,IsHalfling=true"]),
    ("Claw Pounce", &["PREABILITY:1,CATEGORY=FEAT,Nimble Striker", "PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Catfolk ~ Cat's Claws],[PREABILITY:1,CATEGORY=FEAT,Aspect of the Beast (Claws of the Beast)]", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PREFACT:1,TEMPLATES,IsCatfolk=true", "PRESTAT:1,STR=13", "PRETOTALAB:10"]),
    ("Cleave Through", &["PREABILITY:2,CATEGORY=FEAT,Power Attack,Cleave", "PREFACT:1,TEMPLATES,IsDwarf=true", "PRESTAT:1,STR=13", "PRETOTALAB:11"]),
    ("Cloven Helm", &["PREABILITY:2,CATEGORY=FEAT,Dented Helm,Hard Headed", "PREFACT:1,TEMPLATES,IsDwarf=true", "PRETOTALAB:11"]),
    ("Critical Versatility", &["PREFACT:1,TEMPLATES,IsHuman=true", "PREVARGTEQ:FighterWeaponQualifyLVL,11"]),
    ("Demoralizing Lash", &["PREFACT:1,TEMPLATES,IsHobgoblin=true", "PRESKILL:1,Intimidate=1", "PRETOTALAB:1"]),
    ("Dented Helm", &["PREABILITY:1,CATEGORY=FEAT,Hard Headed", "PREFACT:1,TEMPLATES,IsDwarf=true", "PRETOTALAB:6"]),
    ("Desperate Swing", &["PREABILITY:1,CATEGORY=FEAT,Cautious Fighter", "PREFACT:1,TEMPLATES,IsHalfling=true", "PRETOTALAB:1"]),
    ("Destroyer's Blessing", &["PREABILITY:1,CATEGORY=Special Ability,TYPE.Rage", "PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true"]),
    ("Elven Battle Training", &["PREFACT:1,TEMPLATES,IsElf=true", "PRETOTALAB:1"]),
    ("Ferocious Tenacity", &["PREABILITY:2,CATEGORY=Special Ability,Half-Orc ~ Orc Ferocity,TYPE.Rage", "PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true"]),
    ("Fire Hand", &["PREFACT:1,TEMPLATES,IsGoblin=true"]),
    ("Giant Killer", &["PREABILITY:5,CATEGORY=FEAT,Cleave,Goblin Cleaver,Orc Hewer,Power Attack,Strike Back", "PREFACT:1,TEMPLATES,IsDwarf=true", "PRESTAT:1,STR=13", "PRETOTALAB:11"]),
    ("Gloom Strike", &["PREABILITY:1,CATEGORY=FEAT,Blind-Fight", "PREFACT:1,TEMPLATES,IsFetchling=true"]),
    ("Gnome Weapon Focus", &["PREFACT:1,TEMPLATES,IsGnome=true", "PRETOTALAB:1", "PREWEAPONPROF:1,TYPE.Martial"]),
    ("Goblin Cleaver", &["PREABILITY:2,CATEGORY=FEAT,Cleave,Power Attack", "PREFACT:1,TEMPLATES,IsDwarf=true", "PRESTAT:1,STR=13"]),
    ("Goblin Gunslinger", &["PREFACT:1,TEMPLATES,IsGoblin=true"]),
    ("Great Hatred", &["PREABILITY:1,CATEGORY=Special Ability,Gnome ~ Hatred", "PREFACT:1,TEMPLATES,IsGnome=true"]),
    ("Grudge Fighter", &["PREFACT:1,TEMPLATES,IsOrc=true"]),
    ("Hard-Headed", &["PREFACT:1,TEMPLATES,IsDwarf=true", "PRETOTALAB:1"]),
    ("Improved Low Blow", &["PREABILITY:1,CATEGORY=Special Ability,Halfling ~ Low-Blow", "PREFACT:1,TEMPLATES,IsHalfling=true", "PRETOTALAB:4"]),
    ("Improved Surprise Follow-Through", &["PREABILITY:4,CATEGORY=FEAT,Cleave,Great Cleave,Power Attack,Surprise Follow-Through", "PRESTAT:1,STR=13", "PRETOTALAB:8"]),
    ("Inner Flame", &["PREABILITY:1,CATEGORY=FEAT,Scorching Weapons", "PRELEVEL:MIN=7", "PREFACT:1,TEMPLATES,IsIfrit=true"]),
    ("Kobold Ambusher", &["PREFACT:1,TEMPLATES,IsKobold=true", "PRESKILL:1,Stealth=4"]),
    ("Kobold Sniper", &["PREFACT:1,TEMPLATES,IsKobold=true", "PRESKILL:1,Stealth=1"]),
    ("Lucky Strike", &["PREABILITY:1,CATEGORY=Special Ability,Halfling ~ Adaptable Luck", "PREFACT:1,TEMPLATES,IsHalfling=true", "PRETOTALAB:5"]),
    ("Martial Mastery", &["PREABILITY:1,CATEGORY=FEAT,Martial Versatility", "PREFACT:1,TEMPLATES,IsHuman=true", "PREVARGTEQ:FighterWeaponQualifyLVL,16"]),
    ("Martial Versatility", &["PREFACT:1,TEMPLATES,IsHuman=true", "PREVARGTEQ:FighterWeaponQualifyLVL,4"]),
    ("Nimble Striker", &["PREABILITY:1,CATEGORY=Special Ability,Catfolk ~ Sprinter", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PREFACT:1,TEMPLATES,IsCatfolk=true", "PRETOTALAB:1"]),
    ("Orc Hewer", &["PREABILITY:3,CATEGORY=FEAT,Cleave,Goblin Cleaver,Power Attack", "PREFACT:1,TEMPLATES,IsDwarf=true", "PRESTAT:1,STR=13"]),
    ("Orc Weapon Expertise", &["PREFACT:1,TEMPLATES,IsOrc=true", "PRETOTALAB:1"]),
    ("Reverse-Feint", &["PREABILITY:1,CATEGORY=FEAT,Toughness", "PREFACT:1,TEMPLATES,IsOrc=true", "PRETOTALAB:1"]),
    ("Risky Striker", &["PREFACT:1,TEMPLATES,IsHalfling=true", "PRETOTALAB:1"]),
    ("Scorching Weapons", &["PREFACT:1,TEMPLATES,IsIfrit=true"]),
    ("Sea Hunter", &["PREABILITY:1,CATEGORY=FEAT,Combat Expertise", "PREFACT:1,TEMPLATES,IsMerfolk=true"]),
    ("Sharpclaw", &["PREFACT:1,TEMPLATES,IsRatfolk=true"]),
    ("Shatterspell", &["PREABILITY:2,CATEGORY=FEAT,Disruptive,Spellbreaker", "PREFACT:1,TEMPLATES,IsDwarf=true", "PREVARGTEQ:FighterWeaponQualifyLVL,10"]),
    ("Spit Venom", &["PREFACT:1,TEMPLATES,IsNagaji=true"]),
    ("Surprise Follow-Through", &["PREABILITY:2,CATEGORY=FEAT,Cleave,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:1"]),
    ("Surprise Strike", &["PREABILITY:2,CATEGORY=FEAT,Cautious Fighter,Desperate Swing", "PREFACT:1,TEMPLATES,IsHalfling=true", "PRETOTALAB:6"]),
    ("Sympathetic Rage", &["!PREALIGN:LG,LN,LE", "PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true"]),
    ("Tail Terror", &["PREFACT:1,TEMPLATES,IsKobold=true", "PRETOTALAB:1"]),
    ("Tangle Feet", &["PREABILITY:3,CATEGORY=FEAT,Dodge,Mobility,Underfoot", "PREFACT:1,TEMPLATES,IsGoblin=true", "PRESIZELTEQ:S"]),
    ("Taskmaster", &["PREABILITY:1,CATEGORY=FEAT,Demoralizing Lash", "PREFACT:1,TEMPLATES,IsHobgoblin=true", "PRESKILL:1,Intimidate=5"]),
    ("Tree Hanger", &["PREFACT:1,TEMPLATES,IsVanara=true", "PRESKILL:1,Acrobatics=1"]),
    ("Uncanny Defense", &["PREABILITY:1,CATEGORY=FEAT,Cautious Fighter", "PREFACT:1,TEMPLATES,IsHalfling=true", "PRETOTALAB:3"]),
    ("Vast Hatred", &["PREABILITY:1,CATEGORY=Special Ability,Gnome ~ Hatred", "PREFACT:1,TEMPLATES,IsGnome=true"]),
    ("Focusing Blow", &["PREABILITY:1,CATEGORY=FEAT,Hobgoblin Discipline", "PREFACT:1,TEMPLATES,IsHobgoblin=true"]),
    ("Greater Brand", &["PREABILITY:1,CATEGORY=Archetype,Inquisitor Archetype ~ Kinslayer", "PREFACT:1,TEMPLATES,IsDhampir=true"]),
    ("Horde Charge", &["PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true", "PRETOTALAB:1"]),
];

/// Pathfinder Unchained: all 17 records from `pu_feats.lst`, in
/// `pathfinder_unchained::feat_tables::feat_tables()` order. Three carry no
/// `PRE` token at all (`Critical Cure`, `Endurance`, `Twist the Knife`) and
/// are present with an empty slice rather than omitted.
pub const PU_FEAT_PREREQUISITES: &[(&str, &[&str])] = &[
    ("Champion of Anarchy", &["PREABILITY:1,CATEGORY=FEAT,TYPE.Alignment,Champion of Anarchy", "PREALIGN:CN", "PREHD:MIN=10", "PRETEXT:10 Hit Dice, chaotic neutral alignment."]),
    ("Champion of Balance", &["PREABILITY:1,CATEGORY=FEAT,TYPE.Alignment,Champion of Balance", "PREALIGN:TN", "PREHD:MIN=10", "PRETEXT:10 Hit Dice, neutral alignment."]),
    ("Champion of Destruction", &["PREABILITY:1,CATEGORY=FEAT,TYPE.Alignment,Champion of Destruction", "PREALIGN:CE", "PREHD:MIN=10", "PRETEXT:10 Hit Dice, chaotic evil alignment."]),
    ("Champion of Freedom", &["PREABILITY:1,CATEGORY=FEAT,TYPE.Alignment,Champion of Freedom", "PREALIGN:CG", "PREHD:MIN=10", "PRETEXT:10 Hit Dice, chaotic good alignment."]),
    ("Champion of Grace", &["PREABILITY:1,CATEGORY=FEAT,TYPE.Alignment,Champion of Grace", "PREALIGN:NG", "PREHD:MIN=10", "PRETEXT:10 Hit Dice, neutral good alignment."]),
    ("Champion of Malevolence", &["PREABILITY:1,CATEGORY=FEAT,TYPE.Alignment,Champion of Malevolence", "PREALIGN:NE", "PREHD:MIN=10", "PRETEXT:10 Hit Dice, neutral evil alignment."]),
    ("Champion of Righteousness", &["PREABILITY:1,CATEGORY=FEAT,TYPE.Alignment,Champion of Righteousness", "PREALIGN:LG", "PREHD:MIN=10", "PRETEXT:10 Hit Dice, lawful good alignment."]),
    ("Champion of Tranquility", &["PREABILITY:1,CATEGORY=FEAT,TYPE.Alignment,Champion of Tranquility", "PREALIGN:LN", "PREHD:MIN=10", "PRETEXT:10 Hit Dice, lawful neutral alignment."]),
    ("Champion of Tyranny", &["PREABILITY:1,CATEGORY=FEAT,TYPE.Alignment,Champion of Tyranny", "PREALIGN:LE", "PREHD:MIN=10", "PRETEXT:10 Hit Dice, lawful evil alignment."]),
    ("Combat Stamina", &["PRETEXT:Prerequisite: Base attack bonus +1.", "PRETOTALAB:1"]),
    ("Extra Stamina", &["PREABILITY:1,CATEGORY=FEAT,Combat Stamina", "!PREABILITY:3,CATEGORY=FEAT,Extra Stamina", "PRETEXT:Prerequisites: Combat Stamina, base attack bonus +5.", "PRETOTALAB:5"]),
    ("Push the Limits", &["PREABILITY:1,CATEGORY=FEAT,Combat Stamina", "PRESTAT:1,CON=13", "PRETEXT:Prerequisites: Con 13, Combat Stamina, base attack bonus +1.", "PRETOTALAB:1"]),
    ("Critical Cure", &[]),
    ("Endurance", &[]),
    ("Twist the Knife", &[]),
    ("Extra Unchained Rogue Talent", &["PREABILITY:1,CATEGORY=CLASS,Rogue ~ Unchained Class", "PREVARGTEQ:RogueTalentLVL,1"]),
    ("Signature Skill", &["PRESKILL:1,TYPE.Base=5", "PRETEXT:Prerequisite: 5 ranks in the chosen skill.", "PREVAREQ:CannotUseSignatureSkill,0"]),
];

/// Ultimate Campaign: all 23 records from `uca_feats.lst`, in
/// `ultimate_campaign::feat_tables::feat_tables()` order. Unlike ARG and
/// PU, every single UCA Story Feat carries a `PRETEXT:` prose prerequisite
/// rather than a formal `PRE`-family token -- so unlike those two books'
/// gather tables, this one is not filling a gap the book's own table left
/// (there is no formal token to gather), it is carrying the corpus's own
/// display text through as a `PRETEXT:` prerequisite entry, exactly the
/// way PU's own `Combat Stamina`/`Extra Stamina`/`Push the Limits`/
/// `Signature Skill` rows above already do. See `decisions.md`'s dated
/// entry for this cycle (SD28-E13): `PRETEXT:` is carried, never
/// synthesised into a formal `PRE` token from prose, and that is
/// established precedent in this file, not a fresh ruling.
pub const UCA_FEAT_PREREQUISITES: &[(&str, &[&str])] = &[
    ("Accursed", &["PRETEXT:Prerequisite:You must carry a curse that can be lifted only by a quest or similar great undertaking, or have the Cursed Birth background."]),
    ("Arisen", &["PRETEXT:Prerequisite:You must have been slain and brought back from the dead, or have the Left to Die or Cursed Birth background."]),
    ("Battlefield Healer", &["PRETEXT:Prerequisite:You must successfully cast a conjuration (healing) spell on an ally after being hit by an attack of opportunity, or have the Battle, Chaplain, or Healed background."]),
    ("Champion", &["PRETEXT:Prerequisite:You must have defeated a single challenging foe without any aid from another, or have the Champion of a God, Champion of the People, Competition Champion, or Gladiator background."]),
    ("Damned", &["PRETEXT:Prerequisite:You must have had friendly contact with an evil-aligned outsider that would qualify as a challenging foe, have a fiend-related sorcerous bloodline such as abyssal or infernal, have direct fiendish ancestry (such as being a tiefling or half-fiend), or have the Fiend Raised or The Fiend background."]),
    ("Deny the Reaper", &["PRETEXT:Prerequisite:You must have witnessed the death of a close companion in battle-a death that could have been prevented, such as from bleeding, failure to stabilize, or ongoing poison damage-or have the Death in the Family or The War background."]),
    ("Eldritch Researcher", &["PRETEXT:Prerequisite:You must have created a new spell, or have The Way Things Work background."]),
    ("Fearless Zeal", &["PRETEXT:Prerequisite:You must be ordained as a sacred (or profane) champion of your faith by a high-ranking member of its clergy, or have the Devoted, Faith-Bringer, or Moral Debt background. Such an honor goes above and beyond the normal oaths required of a cleric or paladin."]),
    ("Feral Heart", &["PRETEXT:Prerequisite:You must have reverted to savage behavior through a traumatic event or extended period in the wilderness, or have the Raised by Beasts background."]),
    ("Foeslayer", &["PRETEXT:Prerequisite:You must have been defeated and robbed of at least half your possessions by a particular group of humanoids or monstrous humanoids, or have the An Eye for an Eye, Hated Foe, Raiders, or Vengeance background. You may choose a specific race, such as duergar, or a broader group, such as goblinoids. At the GM's option, you may instead choose residents of a particular country, settlement, or tribe."]),
    ("Forgotten Past", &["PRETEXT:Prerequisite:You must have suffered permanent memory loss or have the Reincarnated background."]),
    ("Glimpse Beyond", &["PRETEXT:Prerequisite:You must have faced an undead, evil outsider, or aberration with a CR greater than your level +4, or have the Raised Among the Dead or The Dead One background."]),
    ("Innocent Blood", &["PRETEXT:Prerequisite:You must slay at least 50 intelligent noncombatants for either your own personal gain or for no cause at all, or have the Bloodthirsty, First Kill, or The Kill background."]),
    ("Liberator", &["PRETEXT:Prerequisite:You must have been enslaved for at least 6 months, or have the Imprisoned or Kidnapped background."]),
    ("Lost Legacy", &["PRETEXT:Prerequisite:Your family must have claim to an inherited title or position that no longer belongs to them, or have the Dishonored Family background. You can take this feat even if you have no knowledge of this lost family title."]),
    ("Magnum Opus", &["PRETEXT:Prerequisite:You must either have sold five or more self-created works of art worth a total of at least 5,000 gp, have performed at least five performances for audiences of 50 or more while achieving a great performance result or better on your Perform check, or have the Virtuoso background."]),
    ("Shamed", &["PRETEXT:Prerequisite:You must have been publicly embarrassed, or must have the Bastard Born background. If the embarrassment didn't cause significant harm to your personal honor or social standing, it does not qualify for the feat prerequisites. The humiliation doesn't need to have been unjustified."]),
    ("Stronghold", &["PRETEXT:Prerequisites:You must have the Leadership feat and must lead at least 10 combat-capable followers (such as fighters or rangers)."]),
    ("Thief of Legend", &["PRETEXT:Prerequisites:You must have stolen at least 1,000 gp worth of treasure without being caught and kept mementos of these thefts worth at least 500 gp, or have the Greed background."]),
    ("Town Tamer", &["PRETEXT:Prerequisites:You must have 5 ranks in Intimidate and a personal motivation to clean up a particular town (such as an old friend calling in a favor, or seeking a place to settle down), or you must have the Bounty Hunter or Champion of the People background."]),
    ("True Love", &["PRETEXT:Prerequisite:You must have found love with a person you can't be with, have a current lover, or have the Current Lover, For Love, or The Lover background. Possible complications include distance, your love being with another, your feelings being unrequited, or your relationship being forbidden."]),
    ("Unforgotten", &["PRETEXT:Prerequisite:You must have a close relative, spouse, or other person dear to your heart who never returned from a journey, was captured, or otherwise vanished with little trace, or you have the Major Disaster background."]),
    ("Vengeance", &["PRETEXT:Prerequisite:You must have a close family member or other loved one slain by a specific challenging foe or that foe's minions, or have the Raiders or Vengeance background."]),
];

/// The gathered tokens for `key` in `table`, or `None` when the table has
/// no row for it.
///
/// Returns `None` (never `Some(&[])`) for an absent key, and `Some(&[])`
/// for a key whose corpus row genuinely carries no `PRE` token -- the two
/// facts stay distinguishable, which is the whole reason this table lists
/// every key including the empty ones.
pub fn gathered_prerequisites(
    table: &'static [(&'static str, &'static [&'static str])],
    key: &str,
) -> Option<&'static [&'static str]> {
    table.iter().find(|(row_key, _)| *row_key == key).map(|(_, tokens)| *tokens)
}

// ---------------------------------------------------------------------------
// Per-book category projections
// ---------------------------------------------------------------------------

/// The shared CRB/APG/ACG category enum's variant names.
fn shared_category_name(category: SharedFeatCategory) -> &'static str {
    match category {
        SharedFeatCategory::General => "General",
        SharedFeatCategory::Combat => "Combat",
        SharedFeatCategory::ItemCreation => "ItemCreation",
        SharedFeatCategory::Metamagic => "Metamagic",
        SharedFeatCategory::Teamwork => "Teamwork",
        SharedFeatCategory::Panache => "Panache",
    }
}

/// ARG's own three-variant enum. Its names coincide with three of the
/// shared enum's, so ARG's records merge into the picker's existing
/// General/Combat/Teamwork filters rather than opening new ones -- that
/// is a real property of the two corpora, not a mapping decision made
/// here.
fn arg_category_name(category: arg_feats::FeatCategory) -> &'static str {
    match category {
        arg_feats::FeatCategory::General => "General",
        arg_feats::FeatCategory::Combat => "Combat",
        arg_feats::FeatCategory::Teamwork => "Teamwork",
    }
}

/// PU's own `###Block:`-derived enum. Three of its four names are new to
/// the catalog; that is what this book's corpus says, and inventing a
/// projection onto an existing category would be a classification the
/// corpus never made.
fn pu_category_name(category: pu_feats::FeatCategory) -> &'static str {
    match category {
        pu_feats::FeatCategory::Alignment => "Alignment",
        pu_feats::FeatCategory::CombatStamina => "CombatStamina",
        pu_feats::FeatCategory::WoundThreshold => "WoundThreshold",
        pu_feats::FeatCategory::General => "General",
    }
}

// ---------------------------------------------------------------------------
// Per-book map functions
// ---------------------------------------------------------------------------

/// CRB, APG and ACG all hold `crb::feats::FeatTableEntry`, so one map
/// function serves all three.
fn map_shared_entry(entry: &SharedFeatTableEntry) -> FeatCatalogRecord {
    FeatCatalogRecord {
        key: entry.key,
        category: shared_category_name(entry.category),
        name: entry.name,
        description: entry.description,
        // Passed straight through: CRB/APG/ACG already gathered these.
        prerequisites: entry.prerequisites,
    }
}

/// Looks the record's gathered `PRE` tokens up in `table` and applies the
/// `None`-when-absent rule this record's `prerequisites` field documents.
///
/// A key the gather table does not list at all also yields `None` here,
/// which would be indistinguishable from "the corpus row has none" -- so
/// that case is not allowed to arise silently:
/// `every_arg_and_pu_catalog_key_has_a_gathered_prerequisite_row` asserts
/// every key in both books' tables is listed.
fn gathered(
    table: &'static [(&'static str, &'static [&'static str])],
    key: &str,
) -> Option<&'static [&'static str]> {
    gathered_prerequisites(table, key).filter(|tokens| !tokens.is_empty())
}

fn map_arg_entry(entry: &arg_feats::FeatTableEntry) -> FeatCatalogRecord {
    FeatCatalogRecord {
        key: entry.key,
        category: arg_category_name(entry.category),
        name: entry.name,
        description: entry.description,
        prerequisites: gathered(ARG_FEAT_PREREQUISITES, entry.key),
    }
}

fn map_pu_entry(entry: &pu_feats::FeatTableEntry) -> FeatCatalogRecord {
    FeatCatalogRecord {
        key: entry.key,
        category: pu_category_name(entry.category),
        name: entry.name,
        description: entry.description,
        prerequisites: gathered(PU_FEAT_PREREQUISITES, entry.key),
    }
}

/// UCA's own record type carries `description` (the corpus `DESC:`
/// flavor line) and `benefit` (the corpus `.MOD BENEFIT:` mechanical
/// text) as two separate fields -- see `ultimate_campaign::feat_tables`'s
/// own module doc comment for why both are required to avoid the stub
/// doctrine. `FeatCatalogRecord` has only one free-text `description`
/// field, the one the desktop picker actually renders
/// (`apps/desktop/src-tauri/src/feat_catalog.rs`), so both pieces are
/// joined into it here rather than dropping the mechanical text on the
/// floor.
///
/// For the 2 `deferred-with-reason` records, `benefit` is `None` and the
/// corpus is corrupted -- the joined `description` carries the flavor
/// text plus the engine's own verbatim diagnostic (from
/// `ultimate_campaign::feat_tables::DEFERRED_WITH_REASON`) instead of any
/// mechanical text, so a consumer sees why the record has no benefit
/// rather than either a stub placeholder or invented prose.
fn map_uca_entry(entry: &uca_feats::StoryFeatEntry) -> FeatCatalogRecord {
    let joined_description = match entry.benefit {
        Some(benefit) => entry.description.map(|desc| format!("{desc} {benefit}")),
        None => {
            let diagnostic = uca_feats::DEFERRED_WITH_REASON
                .iter()
                .find(|(key, _)| *key == entry.key)
                .map(|(_, reason)| *reason)
                .unwrap_or("deferred: reason not recorded");
            entry.description.map(|desc| format!("{desc} [DEFERRED-WITH-REASON: {diagnostic}]"))
        }
    };
    FeatCatalogRecord {
        key: entry.key,
        category: "Story",
        name: entry.name,
        // Leaked as `'static` deliberately: the join is computed once and
        // cached for the process lifetime by `uca_records()` below, the
        // same lifetime every other book's `'static` catalog slice
        // already has -- this never re-allocates per request.
        description: joined_description.map(|s| Box::leak(s.into_boxed_str()) as &'static str),
        prerequisites: gathered(UCA_FEAT_PREREQUISITES, entry.key),
    }
}

/// UI's own record type reuses the shared `FeatCategory` enum directly
/// (see `ultimate_intrigue::feat_tables`'s own module doc comment for why
/// -- unlike UCA/ARG/PU, no new category set or fallback derivation was
/// needed), and it already gathers `prerequisites` itself at ingest time
/// (no separate `UI_FEAT_PREREQUISITES` backfill table is needed, unlike
/// ARG/PU whose own tables never gathered `PRE` tokens). `description` and
/// `benefit` are joined the same way `map_uca_entry` joins them -- one
/// free-text field on `FeatCatalogRecord`, two on the book's own type. No
/// record in this catalog is `deferred-with-reason` (see the module doc),
/// so the `[DEFERRED-WITH-REASON: ...]` branch `map_uca_entry` needs never
/// triggers here, but the join stays honest about the corpus shape rather
/// than assuming `benefit` is always `Some`.
fn map_ui_entry(entry: &ui_feats::UiFeatEntry) -> FeatCatalogRecord {
    let joined_description = match (entry.description, entry.benefit) {
        (Some(desc), Some(benefit)) => Some(format!("{desc} {benefit}")),
        (Some(desc), None) => Some(desc.to_string()),
        (None, Some(benefit)) => Some(benefit.to_string()),
        (None, None) => None,
    };
    FeatCatalogRecord {
        key: entry.key,
        category: shared_category_name(entry.category),
        name: entry.name,
        // Leaked as `'static` deliberately -- same rationale as
        // `map_uca_entry`'s own leak: computed once, cached for the
        // process lifetime by `ui_records()` below.
        description: joined_description.map(|s| Box::leak(s.into_boxed_str()) as &'static str),
        prerequisites: entry.prerequisites,
    }
}

/// UW's own five-shared-plus-two-new-variant enum. Unlike UI, UW carries
/// two facets (`Animal`, `Mount`) with no shared-enum equivalent -- see
/// `ultimate_wilderness::feat_tables`'s own doc comment for why folding
/// them onto an existing variant would be a classification the corpus
/// never made. No record in this catalog is `deferred-with-reason` either
/// (confirmed: every one of UW's 136 records carries a real `BENEFIT:`).
fn uw_category_name(category: uw_feats::FeatCategory) -> &'static str {
    match category {
        uw_feats::FeatCategory::General => "General",
        uw_feats::FeatCategory::Combat => "Combat",
        uw_feats::FeatCategory::ItemCreation => "ItemCreation",
        uw_feats::FeatCategory::Metamagic => "Metamagic",
        uw_feats::FeatCategory::Teamwork => "Teamwork",
        uw_feats::FeatCategory::Animal => "Animal",
        uw_feats::FeatCategory::Mount => "Mount",
    }
}

fn map_uw_entry(entry: &uw_feats::UwFeatEntry) -> FeatCatalogRecord {
    let joined_description = match (entry.description, entry.benefit) {
        (Some(desc), Some(benefit)) => Some(format!("{desc} {benefit}")),
        (Some(desc), None) => Some(desc.to_string()),
        (None, Some(benefit)) => Some(benefit.to_string()),
        (None, None) => None,
    };
    FeatCatalogRecord {
        key: entry.key,
        category: uw_category_name(entry.category),
        name: entry.name,
        description: joined_description.map(|s| Box::leak(s.into_boxed_str()) as &'static str),
        prerequisites: entry.prerequisites,
    }
}

/// UC's own eight-variant enum. `Style`/`Grit`/`Critical`/`CalledShot`
/// have no shared-enum equivalent; `Panache` is kept a distinct string
/// from ACG's own `Panache` category deliberately (UC's corpus carries no
/// `TYPE:Panache` record today, so the variant is currently unused, but
/// declared rather than omitted so a future UC record with that facet is
/// not silently folded onto ACG's own Swashbuckler feats). No record in
/// this catalog is `deferred-with-reason` (confirmed: every one of UC's
/// 263 records carries a real `BENEFIT:`).
fn uc_category_name(category: uc_feats::FeatCategory) -> &'static str {
    match category {
        uc_feats::FeatCategory::General => "General",
        uc_feats::FeatCategory::Combat => "Combat",
        uc_feats::FeatCategory::ItemCreation => "ItemCreation",
        uc_feats::FeatCategory::Metamagic => "Metamagic",
        uc_feats::FeatCategory::Teamwork => "Teamwork",
        uc_feats::FeatCategory::Style => "Style",
        uc_feats::FeatCategory::Grit => "Grit",
        uc_feats::FeatCategory::Panache => "UcPanache",
        uc_feats::FeatCategory::Critical => "Critical",
        uc_feats::FeatCategory::CalledShot => "CalledShot",
    }
}

fn map_uc_entry(entry: &uc_feats::UcFeatEntry) -> FeatCatalogRecord {
    let joined_description = match (entry.description, entry.benefit) {
        (Some(desc), Some(benefit)) => Some(format!("{desc} {benefit}")),
        (Some(desc), None) => Some(desc.to_string()),
        (None, Some(benefit)) => Some(benefit.to_string()),
        (None, None) => None,
    };
    FeatCatalogRecord {
        key: entry.key,
        category: uc_category_name(entry.category),
        name: entry.name,
        description: joined_description.map(|s| Box::leak(s.into_boxed_str()) as &'static str),
        prerequisites: entry.prerequisites,
    }
}

/// UM's own eight-variant enum. `Critical`/`Masterpiece`/`Discovery` have
/// no shared-enum equivalent -- see `ultimate_magic::feat_tables`'s own
/// module doc comment for why `Masterpiece`/`Discovery` are UM-specific
/// facets, not folded onto any other book's category.
fn um_category_name(category: um_feats::FeatCategory) -> &'static str {
    match category {
        um_feats::FeatCategory::General => "General",
        um_feats::FeatCategory::Combat => "Combat",
        um_feats::FeatCategory::ItemCreation => "ItemCreation",
        um_feats::FeatCategory::Metamagic => "Metamagic",
        um_feats::FeatCategory::Teamwork => "Teamwork",
        um_feats::FeatCategory::Critical => "Critical",
        um_feats::FeatCategory::Masterpiece => "Masterpiece",
        um_feats::FeatCategory::Discovery => "Discovery",
    }
}

/// Joins UM's two prose fields (`description`, `benefit`) exactly as
/// UC's own `map_uc_entry` does. **Deliberately does not join `effect`
/// into the served description** -- `entry.effect` carries raw,
/// unrendered PCGen `BONUS:`/`DEFINE:` formula tokens (e.g.
/// `BONUS:SPELLKNOWN|CLASS=%LIST;LEVEL=0|2`), and serving those verbatim
/// to a player would leak raw corpus syntax exactly as
/// `feat_descriptions_are_rendered_and_otherwise_byte_identical` and
/// `no_catalog_serves_a_description_carrying_raw_pcgen_syntax` exist to
/// catch (and did, on first attempt at this join). This mirrors CRB's
/// own established rule: `crb::feats::FeatTableEntry`'s `effect` field is
/// never joined into `description` either (`map_shared_entry` passes
/// `description` straight through, ignoring `effect` entirely) -- the
/// four UM records whose only corpus content is a `BONUS:` mechanic
/// (this book's own module doc comment) correctly serve `description:
/// None` here, the same honest treatment CRB's 8 "Heighten Spell +N"
/// tiers already get, not a raw-syntax leak dressed up as content.
fn map_um_entry(entry: &um_feats::UmFeatEntry) -> FeatCatalogRecord {
    let joined_description = match (entry.description, entry.benefit) {
        (Some(desc), Some(benefit)) => Some(format!("{desc} {benefit}")),
        (Some(desc), None) => Some(desc.to_string()),
        (None, Some(benefit)) => Some(benefit.to_string()),
        (None, None) => None,
    };
    FeatCatalogRecord {
        key: entry.key,
        category: um_category_name(entry.category),
        name: entry.name,
        description: joined_description.map(|s| Box::leak(s.into_boxed_str()) as &'static str),
        prerequisites: entry.prerequisites,
    }
}

/// UPsi's own five-variant enum. `Psionic`/`Metapsionic` have no shared-
/// enum equivalent -- see `ultimate_psionics::feat_tables`'s own module
/// doc comment for why they stay UPsi-specific facets.
fn upsi_category_name(category: upsi_feats::FeatCategory) -> &'static str {
    match category {
        upsi_feats::FeatCategory::General => "General",
        upsi_feats::FeatCategory::Combat => "Combat",
        upsi_feats::FeatCategory::ItemCreation => "ItemCreation",
        upsi_feats::FeatCategory::Psionic => "Psionic",
        upsi_feats::FeatCategory::Metapsionic => "Metapsionic",
    }
}

/// Joins UPsi's two prose fields (`description`, `benefit`) exactly as
/// every other book's own mapper does. No `effect` field on this book's
/// own `UpsiFeatEntry` -- unlike UM, every one of UPsi's 221 records
/// carries real `DESC:`/`BENEFIT:` prose (`ultimate_psionics::feat_tables`'s
/// own module doc comment: this book's `DESC:`-is-complete convention
/// means there is no textless-but-real-mechanic category to find here).
fn map_upsi_entry(entry: &upsi_feats::UpsiFeatEntry) -> FeatCatalogRecord {
    let joined_description = match (entry.description, entry.benefit) {
        (Some(desc), Some(benefit)) => Some(format!("{desc} {benefit}")),
        (Some(desc), None) => Some(desc.to_string()),
        (None, Some(benefit)) => Some(benefit.to_string()),
        (None, None) => None,
    };
    FeatCatalogRecord {
        key: entry.key,
        category: upsi_category_name(entry.category),
        name: entry.name,
        description: joined_description.map(|s| Box::leak(s.into_boxed_str()) as &'static str),
        prerequisites: entry.prerequisites,
    }
}

/// Project one book's table once and hand out a `'static` slice of it.
///
/// The projection is a real allocation, so it is cached per book for the
/// process lifetime exactly as each book's own `feat_tables()` caches its
/// table -- `all_feat_tables()` is called per request by the desktop
/// catalog command and must not re-map 690 records each time.
fn projected(
    cell: &'static std::sync::OnceLock<Vec<FeatCatalogRecord>>,
    build: impl FnOnce() -> Vec<FeatCatalogRecord>,
) -> &'static [FeatCatalogRecord] {
    cell.get_or_init(build).as_slice()
}

fn crb_records() -> &'static [FeatCatalogRecord] {
    static CELL: std::sync::OnceLock<Vec<FeatCatalogRecord>> = std::sync::OnceLock::new();
    projected(&CELL, || {
        super::crb::feats::feat_tables().iter().map(map_shared_entry).collect()
    })
}

fn apg_records() -> &'static [FeatCatalogRecord] {
    static CELL: std::sync::OnceLock<Vec<FeatCatalogRecord>> = std::sync::OnceLock::new();
    projected(&CELL, || {
        super::apg::feats::feat_tables().iter().map(map_shared_entry).collect()
    })
}

fn acg_records() -> &'static [FeatCatalogRecord] {
    static CELL: std::sync::OnceLock<Vec<FeatCatalogRecord>> = std::sync::OnceLock::new();
    projected(&CELL, || {
        super::acg::feats::feat_tables().iter().map(map_shared_entry).collect()
    })
}

fn arg_records() -> &'static [FeatCatalogRecord] {
    static CELL: std::sync::OnceLock<Vec<FeatCatalogRecord>> = std::sync::OnceLock::new();
    projected(&CELL, || arg_feats::feat_tables().iter().map(map_arg_entry).collect())
}

fn pu_records() -> &'static [FeatCatalogRecord] {
    static CELL: std::sync::OnceLock<Vec<FeatCatalogRecord>> = std::sync::OnceLock::new();
    projected(&CELL, || pu_feats::feat_tables().iter().map(map_pu_entry).collect())
}

fn uca_records() -> &'static [FeatCatalogRecord] {
    static CELL: std::sync::OnceLock<Vec<FeatCatalogRecord>> = std::sync::OnceLock::new();
    projected(&CELL, || uca_feats::feat_tables().iter().map(map_uca_entry).collect())
}

fn ui_records() -> &'static [FeatCatalogRecord] {
    static CELL: std::sync::OnceLock<Vec<FeatCatalogRecord>> = std::sync::OnceLock::new();
    projected(&CELL, || ui_feats::feat_tables().iter().map(map_ui_entry).collect())
}

fn uw_records() -> &'static [FeatCatalogRecord] {
    static CELL: std::sync::OnceLock<Vec<FeatCatalogRecord>> = std::sync::OnceLock::new();
    projected(&CELL, || uw_feats::feat_tables().iter().map(map_uw_entry).collect())
}

fn uc_records() -> &'static [FeatCatalogRecord] {
    static CELL: std::sync::OnceLock<Vec<FeatCatalogRecord>> = std::sync::OnceLock::new();
    projected(&CELL, || uc_feats::feat_tables().iter().map(map_uc_entry).collect())
}

fn um_records() -> &'static [FeatCatalogRecord] {
    static CELL: std::sync::OnceLock<Vec<FeatCatalogRecord>> = std::sync::OnceLock::new();
    projected(&CELL, || um_feats::feat_tables().iter().map(map_um_entry).collect())
}

fn upsi_records() -> &'static [FeatCatalogRecord] {
    static CELL: std::sync::OnceLock<Vec<FeatCatalogRecord>> = std::sync::OnceLock::new();
    projected(&CELL, || upsi_feats::feat_tables().iter().map(map_upsi_entry).collect())
}

/// Every ingested book's feat catalog, in book order (CRB, APG, ACG,
/// ARG, PU, UCA, UI, UW).
///
/// 952 records total: 185 CRB + 172 APG + 129 ACG + 187 ARG + 17 PU + 23
/// UCA + 104 UI + 135 UW. Built once and cached for the process lifetime,
/// over the eight per-book `feat_tables()` functions -- this never
/// re-derives or re-filters their contents, only projects each record
/// onto [`FeatCatalogRecord`].
/// The per-book feat tables **as each book's own module authored them**,
/// before the corpus gap rows are joined on.
///
/// Split out as its own public function for the same reason
/// `equipment_resolver::hand_authored_equipment_rows` is: it is the input
/// `gen_feat_gap_tables` filters against, so the generator's output set is
/// *provably* the complement of the hand-authored catalog rather than a
/// hand-maintained exclusion list that can drift. Regenerating against
/// [`all_feat_tables`] instead would see the previous run's own rows as
/// already-held and emit nothing.
///
/// Consumers that want the catalog a player sees want [`all_feat_tables`],
/// not this.
pub fn hand_authored_feat_tables() -> &'static [BookFeatTable] {
    static TABLES: std::sync::OnceLock<Vec<BookFeatTable>> = std::sync::OnceLock::new();
    TABLES.get_or_init(|| {
        vec![
            BookFeatTable { rule_set: RuleSetId::Crb, entries: crb_records() },
            BookFeatTable { rule_set: RuleSetId::Apg, entries: apg_records() },
            BookFeatTable { rule_set: RuleSetId::Acg, entries: acg_records() },
            BookFeatTable { rule_set: RuleSetId::Arg, entries: arg_records() },
            BookFeatTable { rule_set: RuleSetId::Pu, entries: pu_records() },
            BookFeatTable { rule_set: RuleSetId::Uca, entries: uca_records() },
            BookFeatTable { rule_set: RuleSetId::Ui, entries: ui_records() },
            BookFeatTable { rule_set: RuleSetId::Uw, entries: uw_records() },
            BookFeatTable { rule_set: RuleSetId::Uc, entries: uc_records() },
            BookFeatTable { rule_set: RuleSetId::Um, entries: um_records() },
            BookFeatTable { rule_set: RuleSetId::Upsi, entries: upsi_records() },
            // `core_essentials` has no hand-authored feat table of its own --
            // it is a PCGen packaging bundle, not a book with its own written
            // feats (`decisions.md` Decision 9) -- but `RuleSetId::Ce` IS a
            // real, compiled rule set (`COMPILED_RULE_SETS`, added for
            // companion/familiar content) and `classify()`'s feat arm
            // resolves a `core_essentials`-directory record's `engine_book`
            // straight to it via `source_book`, never through CRB's
            // shared-library-host fallback. An empty hand-authored slice here
            // is what lets `feat_gap_rows_for(RuleSetId::Ce)`'s rows (all of
            // `ce_feats.lst`) actually get joined on by `all_feat_tables()`
            // below, which only appends gap rows to a `RuleSetId` already
            // present in this list (`SD31-E6-F8-001`).
            BookFeatTable { rule_set: RuleSetId::Ce, entries: &[] },
            // Five more books already compiled into `COMPILED_RULE_SETS` for
            // another kind (race_trait: `Isr`/`Ha`/`Iswg`; monster: `Oa`;
            // race_trait+monster: `MonsterCodex`) but that never had a feat
            // table of their own -- same shape as `Ce` immediately above.
            // Their `feat` units were `not-ingested` with evidence
            // `feat_key_absent_from_catalog` (a started book with no feat
            // table), never `no_compiled_rule_set_for_book` (an un-started
            // book) -- `SD31-E6-F8-002`. Empty hand-authored slices here are
            // what let `feat_gap_rows_for` join each book's real `*_feats.lst`
            // rows on via `all_feat_tables()` below.
            BookFeatTable { rule_set: RuleSetId::Ha, entries: &[] },
            BookFeatTable { rule_set: RuleSetId::Isr, entries: &[] },
            BookFeatTable { rule_set: RuleSetId::Oa, entries: &[] },
            BookFeatTable { rule_set: RuleSetId::Iswg, entries: &[] },
            BookFeatTable { rule_set: RuleSetId::MonsterCodex, entries: &[] },
            // `SD31-E6-F2-007` -- Mythic Adventures' first compiled rule set
            // of any kind. Same shape as the five books immediately above:
            // no hand-authored table, an empty slice here so
            // `feat_gap_rows_for(RuleSetId::Mythic)`'s rows (all of
            // `ma_feats.lst`'s non-`.MOD` declarations) join on below.
            BookFeatTable { rule_set: RuleSetId::Mythic, entries: &[] },
            // `SD31-E6-F8-003` -- two more books already compiled into
            // `COMPILED_RULE_SETS` for another kind (`Isi`: familiars +
            // abilities, `Botd2`: monsters) that never had a feat table of
            // their own. Same shape as the six books above: no hand-authored
            // table, an empty slice here so `feat_gap_rows_for(RuleSetId::Isi
            // /Botd2)`'s rows join on below.
            BookFeatTable { rule_set: RuleSetId::Isi, entries: &[] },
            BookFeatTable { rule_set: RuleSetId::Botd2, entries: &[] },
            // SD-32 Gate 0 book-onboarding precondition (`gate-0-book-
            // onboarding-precondition`, AT-32-G0-003) -- Inner Sea Taverns'
            // first compiled rule set of any kind. Same shape as `Mythic`
            // above: no hand-authored table, an empty slice here so
            // `feat_gap_rows_for(RuleSetId::InnerSeaTaverns)`'s rows (all
            // of `istav_feats.lst`'s non-`.MOD` declarations) join on
            // below.
            BookFeatTable { rule_set: RuleSetId::InnerSeaTaverns, entries: &[] },
            // SD-32 T9 onboarding (card 11), `decisions.md §19` PI sign-off.
            // `RuleSetId::Isc`/`RuleSetId::Isg` are already compiled and
            // already in `COMPILED_RULE_SETS` (added for equipment/monster
            // content) but never had a feat table of their own -- same shape
            // as `Ha`/`Isr`/`Oa`/`Iswg`/`MonsterCodex` above. Empty
            // hand-authored slices here are what let
            // `feat_gap_rows_for(RuleSetId::Isc/Isg)`'s rows (every real
            // `CATEGORY:FEAT` row in `isc_abilities_feat.lst`/
            // `isg_abilities_feat.lst` -- verified NOT the `.MOD`/
            // `VISIBLE:EXPORT` continuation shape found blocking
            // `horror_adventures`/`mythic_adventures`, see this cycle's own
            // receipt) join on below.
            BookFeatTable { rule_set: RuleSetId::Isc, entries: &[] },
            BookFeatTable { rule_set: RuleSetId::Isg, entries: &[] },
        ]
    })
}

/// Every ingested book's feat catalog, with the corpus **gap rows** joined
/// on — the catalog every consumer and every player surface reads.
///
/// A gap row is a corpus feat record belonging to one of these
/// already-compiled books whose own hand-authored table never held it (see
/// [`feat_gap_tables`](super::feat_gap_tables)). The rows are appended
/// **after** each book's hand-authored records, never interleaved, so a
/// first-match key lookup over a book's slice keeps resolving to the
/// hand-authored record it resolved to before — the ordering property
/// `tests/feat_gap_tables.rs` pins directly.
pub fn all_feat_tables() -> &'static [BookFeatTable] {
    static TABLES: std::sync::OnceLock<Vec<BookFeatTable>> = std::sync::OnceLock::new();
    TABLES.get_or_init(|| {
        hand_authored_feat_tables()
            .iter()
            .map(|book| {
                let gaps = super::feat_gap_tables::feat_gap_rows_for(book.rule_set);
                if gaps.is_empty() {
                    return *book;
                }
                let mut joined: Vec<FeatCatalogRecord> = book.entries.to_vec();
                joined.extend_from_slice(gaps);
                BookFeatTable {
                    rule_set: book.rule_set,
                    // Leaked once, inside a `OnceLock`, so this allocates
                    // exactly once per process for the lifetime the
                    // `&'static` promises — the same trick `map_uw_entry`
                    // already uses for its joined description strings.
                    entries: Box::leak(joined.into_boxed_slice()),
                }
            })
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    /// Asserted against [`hand_authored_feat_tables`], not [`all_feat_tables`]:
    /// every number below is a fact about what that book's OWN module
    /// authored, and the corpus gap rows are by construction records those
    /// modules never held. Pointing this at the joined catalog would silently
    /// turn a per-book ingest pin into a pin on the gap lane's size. The
    /// joined total is pinned separately, immediately below.
    #[test]
    fn spans_every_ingested_book_with_their_real_counts() {
        let books = hand_authored_feat_tables();
        assert_eq!(books.len(), 23);
        assert_eq!(books[0].rule_set, RuleSetId::Crb);
        assert_eq!(books[0].entries.len(), 185);
        assert_eq!(books[1].rule_set, RuleSetId::Apg);
        assert_eq!(books[1].entries.len(), 172);
        assert_eq!(books[2].rule_set, RuleSetId::Acg);
        assert_eq!(books[2].entries.len(), 129);
        assert_eq!(books[3].rule_set, RuleSetId::Arg);
        assert_eq!(books[3].entries.len(), 187);
        assert_eq!(books[4].rule_set, RuleSetId::Pu);
        assert_eq!(books[4].entries.len(), 17);
        assert_eq!(books[5].rule_set, RuleSetId::Uca);
        assert_eq!(books[5].entries.len(), 23);
        assert_eq!(books[6].rule_set, RuleSetId::Ui);
        assert_eq!(books[6].entries.len(), 104);
        assert_eq!(books[7].rule_set, RuleSetId::Uw);
        assert_eq!(books[7].entries.len(), 135);
        assert_eq!(books[8].rule_set, RuleSetId::Uc);
        assert_eq!(books[8].entries.len(), 261);
        assert_eq!(books[9].rule_set, RuleSetId::Um);
        assert_eq!(books[9].entries.len(), 144);
        assert_eq!(books[10].rule_set, RuleSetId::Upsi);
        assert_eq!(books[10].entries.len(), 221);
        // `core_essentials` has no hand-authored feat table of its own
        // (`SD31-E6-F8-001`) -- see `hand_authored_feat_tables`'s own doc
        // comment on why an empty entry is still filed here.
        assert_eq!(books[11].rule_set, RuleSetId::Ce);
        assert_eq!(books[11].entries.len(), 0);
        // `SD31-E6-F8-002` -- five more books, each already compiled for
        // another kind, given an empty hand-authored feat slice so their real
        // `*_feats.lst` rows can join via `feat_gap_rows_for` below.
        assert_eq!(books[12].rule_set, RuleSetId::Ha);
        assert_eq!(books[12].entries.len(), 0);
        assert_eq!(books[13].rule_set, RuleSetId::Isr);
        assert_eq!(books[13].entries.len(), 0);
        assert_eq!(books[14].rule_set, RuleSetId::Oa);
        assert_eq!(books[14].entries.len(), 0);
        assert_eq!(books[15].rule_set, RuleSetId::Iswg);
        assert_eq!(books[15].entries.len(), 0);
        assert_eq!(books[16].rule_set, RuleSetId::MonsterCodex);
        assert_eq!(books[16].entries.len(), 0);
        // `SD31-E6-F2-007` -- Mythic Adventures' first compiled rule set of
        // any kind, same empty-hand-authored-slice shape as the five above.
        assert_eq!(books[17].rule_set, RuleSetId::Mythic);
        assert_eq!(books[17].entries.len(), 0);
        // `SD31-E6-F8-003` -- two more books, each already compiled for
        // another kind, given an empty hand-authored feat slice so their
        // real `*_feats.lst` rows can join via `feat_gap_rows_for` below.
        assert_eq!(books[18].rule_set, RuleSetId::Isi);
        assert_eq!(books[18].entries.len(), 0);
        assert_eq!(books[19].rule_set, RuleSetId::Botd2);
        assert_eq!(books[19].entries.len(), 0);
        // SD-32 Gate 0 book-onboarding precondition (`gate-0-book-
        // onboarding-precondition`, AT-32-G0-003) -- Inner Sea Taverns'
        // first compiled rule set of any kind, given an empty
        // hand-authored feat slice so its real `istav_feats.lst` rows can
        // join via `feat_gap_rows_for` below.
        assert_eq!(books[20].rule_set, RuleSetId::InnerSeaTaverns);
        assert_eq!(books[20].entries.len(), 0);
        // SD-32 T9 onboarding (card 11), `decisions.md §19` PI sign-off --
        // `Isc`/`Isg` already compiled for equipment/monster content, given
        // an empty hand-authored feat slice so their real
        // `isc_abilities_feat.lst`/`isg_abilities_feat.lst` rows can join
        // via `feat_gap_rows_for` below.
        assert_eq!(books[21].rule_set, RuleSetId::Isc);
        assert_eq!(books[21].entries.len(), 0);
        assert_eq!(books[22].rule_set, RuleSetId::Isg);
        assert_eq!(books[22].entries.len(), 0);

        let total: usize = books.iter().map(|book| book.entries.len()).sum();
        assert_eq!(
            total,
            1578,
            "185 CRB + 172 APG + 129 ACG + 187 ARG + 17 PU + 23 UCA + 104 UI + 135 UW + 261 UC + 144 UM + 221 UPsi + 0 Ce + 0 Ha + 0 Isr + 0 Oa + 0 Iswg + 0 MonsterCodex + 0 Mythic + 0 Isi + 0 Botd2 + 0 InnerSeaTaverns + 0 Isc + 0 Isg"
        );
    }

    /// The catalog a player actually sees: the hand-authored records plus the
    /// corpus gap rows. Pinned per book so a regeneration that drops one
    /// book's rows fails here rather than silently shrinking the picker.
    #[test]
    fn the_joined_catalog_is_the_hand_authored_one_plus_the_corpus_gap_rows() {
        let hand = hand_authored_feat_tables();
        let joined = all_feat_tables();
        assert_eq!(joined.len(), hand.len());
        for (j, h) in joined.iter().zip(hand.iter()) {
            assert_eq!(j.rule_set, h.rule_set, "book order must be preserved");
            let gaps = super::super::feat_gap_tables::feat_gap_rows_for(h.rule_set).len();
            assert_eq!(
                j.entries.len(),
                h.entries.len() + gaps,
                "{:?}: joined slice must be the book's own table plus exactly its gap rows",
                h.rule_set
            );
        }
        let total: usize = joined.iter().map(|book| book.entries.len()).sum();
        assert_eq!(
            total, 2227,
            "1578 hand-authored + 649 corpus gap rows: the original 325 \
             (SD31-E6-F8-001's 83: 1 CRB, 15 core_essentials, 48 ARG, 12 UM, 3 UI, \
             2 UC, 1 UPsi, 1 UW; SD31-E6-F8-002's 242: 61 Ha, 50 Isr, 68 Oa, 31 Iswg, \
             32 MonsterCodex) + 199 more from Mythic Adventures' first-ever compiled \
             rule set (SD31-E6-F2-007, `ma_feats.lst`'s non-`.MOD` declarations -- \
             SD31-W10-INTEGRATE-001 excluded 159 VISIBLE:EXPORT display-plumbing \
             twins from the original 358) + 7 more from two more already-compiled \
             books (SD31-E6-F8-003: inner_sea_intrigue 6 + book_of_the_damned_volume_2 1) \
             + 9 more from Inner Sea Taverns' first-ever compiled rule set \
             (SD-32 Gate 0 book-onboarding precondition, `gate-0-book-onboarding-\
             precondition`, AT-32-G0-003, `istav_feats.lst`'s non-`.MOD` declarations) \
             + 109 more from T9 onboarding (card 11, `decisions.md §19` PI sign-off): \
             `Isc` 23 (inner_sea_combat, isc_abilities_feat.lst, 1 NAMEISPI:YES record \
             dropped) + `Isg` 86 (inner_sea_gods, isg_abilities_feat.lst, deity-name \
             prerequisites redacted per the book's existing blacklist screen, not dropped)"
        );
    }

    /// The projection must not lose or invent a record: each book's slice
    /// is exactly as long as the book's own table, checked against the
    /// per-book functions rather than against the numbers above.
    #[test]
    fn each_books_slice_is_exactly_its_own_table() {
        let books = hand_authored_feat_tables();
        assert_eq!(books[0].entries.len(), super::super::crb::feats::feat_tables().len());
        assert_eq!(books[1].entries.len(), super::super::apg::feats::feat_tables().len());
        assert_eq!(books[2].entries.len(), super::super::acg::feats::feat_tables().len());
        assert_eq!(books[3].entries.len(), arg_feats::feat_tables().len());
        assert_eq!(books[4].entries.len(), pu_feats::feat_tables().len());
        assert_eq!(books[5].entries.len(), uca_feats::feat_tables().len());
        assert_eq!(books[6].entries.len(), ui_feats::feat_tables().len());
        assert_eq!(books[7].entries.len(), uw_feats::feat_tables().len());
        assert_eq!(books[8].entries.len(), uc_feats::feat_tables().len());
        assert_eq!(books[9].entries.len(), um_feats::feat_tables().len());
        assert_eq!(books[10].entries.len(), upsi_feats::feat_tables().len());
    }

    #[test]
    fn every_record_carries_a_real_key_and_name() {
        for book in all_feat_tables() {
            for entry in book.entries {
                assert!(!entry.key.is_empty(), "{:?} entry with empty key", book.rule_set);
                assert!(
                    !entry.name.is_empty(),
                    "{:?} entry '{}' has an empty name",
                    book.rule_set,
                    entry.key
                );
                assert!(
                    !entry.category.is_empty(),
                    "{:?} entry '{}' has an empty category",
                    book.rule_set,
                    entry.key
                );
            }
        }
    }

    /// The gather table must list **every** ARG and PU key, including the
    /// ones whose corpus row carries no `PRE` token. A missing key and a
    /// genuinely-empty row both surface as `prerequisites: None` on the
    /// record, so without this assertion a key the gather missed would be
    /// silently reported as "this feat has no prerequisites" -- exactly
    /// the fabricated absence this catalog's `None` rules exist to
    /// prevent.
    #[test]
    fn every_arg_and_pu_catalog_key_has_a_gathered_prerequisite_row() {
        for entry in arg_feats::feat_tables() {
            assert!(
                gathered_prerequisites(ARG_FEAT_PREREQUISITES, entry.key).is_some(),
                "ARG feat '{}' has no row in ARG_FEAT_PREREQUISITES; an absent row is \
                 indistinguishable from a genuinely prerequisite-free record once it \
                 reaches FeatCatalogRecord",
                entry.key
            );
        }
        for entry in pu_feats::feat_tables() {
            assert!(
                gathered_prerequisites(PU_FEAT_PREREQUISITES, entry.key).is_some(),
                "PU feat '{}' has no row in PU_FEAT_PREREQUISITES",
                entry.key
            );
        }
        for entry in uca_feats::feat_tables() {
            assert!(
                gathered_prerequisites(UCA_FEAT_PREREQUISITES, entry.key).is_some(),
                "UCA feat '{}' has no row in UCA_FEAT_PREREQUISITES",
                entry.key
            );
        }
        assert_eq!(ARG_FEAT_PREREQUISITES.len(), arg_feats::feat_tables().len());
        assert_eq!(PU_FEAT_PREREQUISITES.len(), pu_feats::feat_tables().len());
        assert_eq!(UCA_FEAT_PREREQUISITES.len(), uca_feats::feat_tables().len());
    }

    /// The real per-book prerequisite coverage, derived from the live
    /// aggregate. The ARG number is the point of the whole gather: **every
    /// one of that book's 187 records carries at least one `PRE`-family
    /// token**, and before this gather landed the engine held none of
    /// them.
    #[test]
    fn the_per_book_prerequisite_coverage_is_the_real_one() {
        // Over `hand_authored_feat_tables()`: every number here is a claim
        // about what that book's own ingest gathered. The corpus gap rows
        // carry their own `PRE` tokens and are covered by
        // `tests/feat_gap_tables.rs`.
        let with_prerequisites = |rule_set: RuleSetId| -> usize {
            hand_authored_feat_tables()
                .iter()
                .filter(|book| book.rule_set == rule_set)
                .flat_map(|book| book.entries.iter())
                .filter(|entry| entry.prerequisites.is_some())
                .count()
        };

        assert_eq!(with_prerequisites(RuleSetId::Crb), 130, "of 185");
        assert_eq!(with_prerequisites(RuleSetId::Apg), 143, "of 172");
        assert_eq!(with_prerequisites(RuleSetId::Acg), 125, "of 129");
        assert_eq!(with_prerequisites(RuleSetId::Arg), 187, "of 187 -- all of them");
        assert_eq!(with_prerequisites(RuleSetId::Pu), 14, "of 17");
        assert_eq!(with_prerequisites(RuleSetId::Uca), 23, "of 23 -- all of them carry PRETEXT:");
        assert_eq!(with_prerequisites(RuleSetId::Ui), 98, "of 104 -- gathered directly at ingest, no backfill table");
        assert_eq!(with_prerequisites(RuleSetId::Uw), 127, "of 135 -- gathered directly at ingest, no backfill table");
        assert_eq!(with_prerequisites(RuleSetId::Uc), 247, "of 261 -- gathered directly at ingest, no backfill table");
        assert_eq!(with_prerequisites(RuleSetId::Um), 135, "of 144 -- gathered directly at ingest, no backfill table");
        assert_eq!(with_prerequisites(RuleSetId::Upsi), 200, "of 221 -- gathered directly at ingest, no backfill table");

        let total: usize = all_feat_tables()
            .iter()
            .flat_map(|book| book.entries.iter())
            .filter(|entry| entry.prerequisites.is_some())
            .count();
        // Over the JOINED catalog, deliberately: this total is the one a
        // prerequisite consumer actually faces. 1429 of the 1578
        // hand-authored records, plus 63 of the original 83 corpus gap rows,
        // plus 223 of the 242 rows `SD31-E6-F8-002` added — the gap rows
        // carry their own `PRE`-family tokens verbatim, so they are gated by
        // `feat_prereqs` exactly like every other record rather than being
        // offered unconditionally.
        // 1715 of the pre-Mythic 1903 (as before) + 195 of Mythic's 199 gap
        // rows (`SD31-E6-F2-007`) -- the gap rows carry their own `PRE`-
        // family tokens verbatim, gated by `feat_prereqs` like every other
        // record, never offered unconditionally. Unchanged by SD31-W10-
        // INTEGRATE-001's exclusion of 159 VISIBLE:EXPORT twins: every one
        // of them carried zero `PRE` tokens, so none was ever in this
        // numerator -- only the denominator (2261 -> 2102) moved.
        // SD31-E6-F8-003 adds 7 more (inner_sea_intrigue 6 + book_of_the_
        // damned_volume_2 1) -- all 7 carry at least one real `PRE`-family
        // token (verified by direct read of both `.lst` files), so the
        // numerator and denominator both move by exactly +7.
        // SD-32 Gate 0 book-onboarding precondition adds 9 more
        // (inner_sea_taverns) -- 5 of the 9 carry at least one real `PRE`-
        // family token (`Drunken God's Blessings`, `Drunken Sing-Along`,
        // `Hardy Liver`, `Read the Room`, `Tavern Regular`), so the
        // numerator moves by +5 and the denominator by +9.
        // SD-32 T9 onboarding (card 11) adds 109 more (inner_sea_combat 23 +
        // inner_sea_gods 86) -- 108 of the 109 carry at least one real
        // `PRE`-family token (re-derived: `cargo run --bin gen_feat_gap_tables`
        // stdout against the pinned oracle), so the numerator moves by +108
        // and the denominator by +109.
        assert_eq!(total, 2030, "2030 of the joined catalog's 2227 records have a prerequisite");
    }

    /// `Some(&[])` must never reach a consumer: an empty slice would read
    /// as "has prerequisites, none of them anything", which no corpus row
    /// says.
    #[test]
    fn no_record_carries_an_empty_prerequisite_slice() {
        for book in all_feat_tables() {
            for entry in book.entries {
                if let Some(tokens) = entry.prerequisites {
                    assert!(
                        !tokens.is_empty(),
                        "{:?} '{}' carries Some(&[]); absence must be None",
                        book.rule_set,
                        entry.key
                    );
                    for token in tokens {
                        assert!(
                            token.starts_with("PRE") || token.starts_with("!PRE"),
                            "{:?} '{}' carries a non-PRE token {token:?}",
                            book.rule_set,
                            entry.key
                        );
                    }
                }
            }
        }
    }

    /// The three feats the on-screen proof turns on, plus one ARG record
    /// whose tokens did not exist anywhere in the engine before this
    /// gather.
    #[test]
    fn the_real_prerequisite_tokens_reach_the_aggregate() {
        let find = |key: &str, rule_set: RuleSetId| {
            all_feat_tables()
                .iter()
                .filter(move |book| book.rule_set == rule_set)
                .flat_map(|book| book.entries.iter())
                .find(|entry| entry.key == key)
                .unwrap_or_else(|| panic!("'{key}' must be in the aggregate"))
        };

        assert_eq!(
            find("Improved Two-Weapon Fighting", RuleSetId::Crb).prerequisites,
            Some(
                &[
                    "PREABILITY:1,CATEGORY=FEAT,Two-Weapon Fighting",
                    // Note the corpus states the Dex 17 requirement purely
                    // through PCGen variables -- there is no `PRESTAT:` on
                    // this record at all. `pre_tokens` models both:
                    // `PreStatScore_DEX` IS the Dex score per
                    // `cr__stats.lst`, and `FeatDexRequirement` is 0 for
                    // every character built here.
                    "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,17],[PREVARGTEQ:FeatDexRequirement,17]",
                    "PRETOTALAB:6",
                ][..]
            )
        );
        assert_eq!(find("Two-Weapon Fighting", RuleSetId::Crb).prerequisites.unwrap().len(), 1);
        // A CRB record whose corpus row genuinely has no PRE token.
        assert_eq!(find("Toughness", RuleSetId::Crb).prerequisites, None);
        // ARG: never gathered before this module existed.
        assert_eq!(
            find("Armor of the Pit", RuleSetId::Arg).prerequisites,
            Some(&["PREFACT:1,TEMPLATES,IsTiefling=true"][..])
        );
    }

    /// The category strings are the wire form the desktop picker filters
    /// on, so each literal must be its variant's `Debug` form. Written
    /// over each enum's own `ALL` roster, so a variant added without a
    /// `match` arm fails to compile and a variant added *with* a
    /// mismatched literal fails here.
    #[test]
    fn category_names_match_the_debug_form_of_every_variant() {
        for category in SharedFeatCategory::ALL {
            assert_eq!(shared_category_name(*category), format!("{category:?}"));
        }
        for category in arg_feats::FeatCategory::ALL {
            assert_eq!(arg_category_name(*category), format!("{category:?}"));
        }
        for category in pu_feats::FeatCategory::ALL {
            assert_eq!(pu_category_name(*category), format!("{category:?}"));
        }
    }

    /// The real per-book category breakdown, derived from the live
    /// tables. Pins ARG's and PU's own corpus-documented splits (ARG:
    /// 132 General / 52 Combat / 3 Teamwork, of which one "General" is
    /// the corpus's own `TYPE:Genaral` typo, classified rather than
    /// dropped; PU: 9 Alignment / 3 CombatStamina / 3 WoundThreshold /
    /// 2 General) at the join boundary, so a regression in either book's
    /// table surfaces here and not only in that book's own module.
    #[test]
    fn the_per_book_category_split_is_the_real_one() {
        let split = |rule_set: RuleSetId| -> BTreeMap<&'static str, usize> {
            let mut counts = BTreeMap::new();
            // Over `hand_authored_feat_tables()`: these splits are each
            // book's own `FeatCategory` enum roster. Gap rows carry the
            // corpus `TYPE:` facet verbatim instead of an enum variant name
            // (see `feat_gap_tables`' module doc), so folding them in here
            // would mix two different classification systems in one map.
            for book in hand_authored_feat_tables().iter().filter(|book| book.rule_set == rule_set)
            {
                for entry in book.entries {
                    *counts.entry(entry.category).or_insert(0) += 1;
                }
            }
            counts
        };

        assert_eq!(
            split(RuleSetId::Arg),
            BTreeMap::from([("Combat", 52), ("General", 132), ("Teamwork", 3)])
        );
        assert_eq!(
            split(RuleSetId::Pu),
            BTreeMap::from([
                ("Alignment", 9),
                ("CombatStamina", 3),
                ("General", 2),
                ("WoundThreshold", 3),
            ])
        );
        // UCA's corpus carries no `###Block:`/`TYPE:` category facet at
        // all -- every one of its 23 records is `TYPE:Story`, so every
        // record lands in the single "Story" category rather than
        // inventing a split the corpus doesn't support.
        assert_eq!(split(RuleSetId::Uca), BTreeMap::from([("Story", 23)]));
        // UI reuses the shared `FeatCategory` enum -- General/Combat
        // (folding the Combat.* sub-facets) / Metamagic / Teamwork.
        assert_eq!(
            split(RuleSetId::Ui),
            BTreeMap::from([("Combat", 46), ("General", 52), ("Metamagic", 4), ("Teamwork", 2)])
        );
        // UW's own two new facets -- Animal (Companion-focused feats) and
        // Mount -- have no shared-enum equivalent. `Mount` carries zero
        // real feat records in this corpus: the only `TYPE:Mount` row
        // (`Samurai ~ Mount.MOD`) is a `CATEGORY:Special Ability` row, not
        // a feat at all, and was never a candidate.
        assert_eq!(
            split(RuleSetId::Uw),
            BTreeMap::from([
                ("Animal", 11),
                ("Combat", 41),
                ("General", 77),
                ("ItemCreation", 1),
                ("Metamagic", 2),
                ("Teamwork", 3),
            ])
        );
        // UC's own new facets: `CalledShot`, `Critical` (its bare
        // `TYPE:Critical` facet, distinct from `Combat.Critical`, which
        // folds to `Combat`), and `Style` (its bare `TYPE:Style` facet,
        // distinct from `Combat.Style`). No UC record carries `TYPE:Grit`'s
        // sibling `Panache` facet today (`"UcPanache"` never appears).
        assert_eq!(
            split(RuleSetId::Uc),
            BTreeMap::from([
                ("CalledShot", 2),
                ("Combat", 181),
                ("Critical", 1),
                ("General", 62),
                ("Grit", 7),
                ("Style", 1),
                ("Teamwork", 7),
            ])
        );
        // UM's own new facets: `Masterpiece` (Bard performance feats) and
        // `Discovery` (Wizard bonus-discovery-as-feat records). No UM
        // record carries `TYPE:Style`/`Grit`/`Panache`/`CalledShot`.
        assert_eq!(
            split(RuleSetId::Um),
            BTreeMap::from([
                ("Combat", 3),
                ("Critical", 3),
                ("Discovery", 11),
                ("General", 100),
                ("ItemCreation", 2),
                ("Masterpiece", 15),
                ("Metamagic", 9),
                ("Teamwork", 1),
            ])
        );
        // UPsi's own new facets: `Psionic` (this book's dominant facet)
        // and `Metapsionic` (its metamagic equivalent) -- no shared-enum
        // equivalent for either. No UPsi record carries `TYPE:Teamwork`.
        assert_eq!(
            split(RuleSetId::Upsi),
            BTreeMap::from([
                ("Combat", 9),
                ("General", 21),
                ("ItemCreation", 3),
                ("Metapsionic", 35),
                ("Psionic", 153),
            ])
        );
    }

    /// The point of widening the aggregate: real ARG and PU feats are in
    /// it, with their real corpus description text.
    #[test]
    fn real_arg_and_pu_records_are_in_the_aggregate_with_their_descriptions() {
        let find = |key: &str| {
            all_feat_tables()
                .iter()
                .flat_map(|book| book.entries.iter().map(move |entry| (book.rule_set, entry)))
                .find(|(_, entry)| entry.key == key)
                .unwrap_or_else(|| panic!("'{key}' must be in the aggregate catalog"))
        };

        let (book, wings) = find("Angel Wings");
        assert_eq!(book, RuleSetId::Arg);
        assert_eq!(wings.category, "General");
        assert_eq!(wings.description, Some("Feathered wings sprout from your back."));

        let (book, champion) = find("Champion of Tyranny");
        assert_eq!(book, RuleSetId::Pu);
        assert_eq!(champion.category, "Alignment");
        assert_eq!(
            champion.description,
            Some("You must beat down the masses to have true order.")
        );

        let (book, stamina) = find("Combat Stamina");
        assert_eq!(book, RuleSetId::Pu);
        assert_eq!(stamina.category, "CombatStamina");
    }

    /// UCA's 21 text-complete records surface both the corpus `DESC:`
    /// flavor text and the `.MOD BENEFIT:` mechanical text, joined --
    /// showing only `DESC:` (`"[Not Implemented] ..."`) would be a stub
    /// by `docs/governance/no-stub-mvp-doctrine.md`. Its 2
    /// `deferred-with-reason` records surface the flavor text plus the
    /// engine's own verbatim diagnostic instead of the corrupted upstream
    /// benefit text. (`Stronghold` was deferred in this module's first
    /// pass and is now text-complete -- its own text is genuinely
    /// complete; see `ultimate_campaign::feat_tables`'s own doc comment
    /// for the correction.)
    #[test]
    fn uca_records_join_desc_and_benefit_and_defer_the_two_corrupted_rows() {
        let find = |key: &str| {
            all_feat_tables()
                .iter()
                .filter(|book| book.rule_set == RuleSetId::Uca)
                .flat_map(|book| book.entries.iter())
                .find(|entry| entry.key == key)
                .unwrap_or_else(|| panic!("'{key}' must be in the UCA aggregate"))
        };

        let accursed = find("Accursed");
        assert_eq!(accursed.category, "Story");
        let desc = accursed.description.expect("Accursed must have a joined description");
        assert!(desc.starts_with("[Not Implemented] Your curse weighs down your soul"));
        assert!(
            desc.contains("You gain spell resistance equal to 5 + your character level"),
            "Accursed's joined description must carry the real BENEFIT text, not just DESC:"
        );
        assert!(
            accursed.prerequisites.unwrap()[0].starts_with("PRETEXT:Prerequisite:You must carry a curse"),
            "Accursed must carry its PRETEXT: prerequisite, not a synthesised PRE token"
        );

        for key in ["Fearless Zeal", "Magnum Opus"] {
            let entry = find(key);
            let desc = entry.description.unwrap_or_else(|| panic!("{key} must still have a description"));
            assert!(
                desc.contains("DEFERRED-WITH-REASON"),
                "{key}'s joined description must carry the deferral diagnostic, not just flavor text"
            );
            assert!(
                desc.contains("uca_feats.lst:"),
                "{key}'s deferral diagnostic must cite a file:line, not a vague reason"
            );
        }

        let stronghold = find("Stronghold");
        let stronghold_desc = stronghold.description.expect("Stronghold must have a joined description");
        assert!(
            !stronghold_desc.contains("DEFERRED-WITH-REASON"),
            "Stronghold's own text is complete and must not carry the deferral diagnostic"
        );
        assert!(
            stronghold_desc.contains("gains a +2 bonus to AC."),
            "Stronghold's joined description must carry its own real BENEFIT text"
        );
        assert!(
            !stronghold_desc.contains("reroll a failed saving throw"),
            "Stronghold's joined description must not carry Magnum Opus's foreign trailing sentence"
        );

        let complete_count = all_feat_tables()
            .iter()
            .filter(|book| book.rule_set == RuleSetId::Uca)
            .flat_map(|book| book.entries.iter())
            .filter(|entry| !entry.description.unwrap_or_default().contains("DEFERRED-WITH-REASON"))
            .count();
        assert_eq!(complete_count, 21, "21 of 23 UCA records are text-complete, not deferred");
    }

    /// UI's 104 records all carry both `DESC:` and `BENEFIT:` (see
    /// `ultimate_intrigue::feat_tables`'s own module doc comment -- no
    /// upstream splice/truncation defect found), so every joined
    /// description carries both, unlike UCA's two deferred rows.
    #[test]
    fn ui_records_join_desc_and_benefit_with_no_deferrals() {
        let find = |key: &str| {
            all_feat_tables()
                .iter()
                .filter(|book| book.rule_set == RuleSetId::Ui)
                .flat_map(|book| book.entries.iter())
                .find(|entry| entry.key == key)
                .unwrap_or_else(|| panic!("'{key}' must be in the UI aggregate"))
        };

        let acrobatic = find("Acrobatic Spellcaster");
        assert_eq!(acrobatic.category, "Combat");
        let desc = acrobatic.description.expect("Acrobatic Spellcaster must have a joined description");
        assert!(desc.starts_with("Your skillful movements prevent foes from disrupting your spells."));
        assert!(
            desc.contains("creatures denied attacks of opportunity by your Acrobatics check"),
            "Acrobatic Spellcaster's joined description must carry the real BENEFIT text, not just DESC:"
        );
        assert!(
            acrobatic.prerequisites.unwrap()[0].starts_with("PREABILITY:2,CATEGORY=FEAT,Combat Casting"),
            "Acrobatic Spellcaster must carry its real PREABILITY: token"
        );

        let no_deferrals = all_feat_tables()
            .iter()
            .filter(|book| book.rule_set == RuleSetId::Ui)
            .flat_map(|book| book.entries.iter())
            .filter(|entry| entry.description.unwrap_or_default().contains("DEFERRED-WITH-REASON"))
            .count();
        assert_eq!(no_deferrals, 0, "no UI feat record is deferred-with-reason");
    }

    /// Feat keys were globally unique across CRB/APG/ACG and are not
    /// once PU is in. `Endurance` is the only one, and it is a re-listing
    /// rather than two different feats -- see this module's own "Key
    /// collisions" section for the corpus evidence.
    ///
    /// What that costs today: a consumer that flattens the catalog and
    /// looks up by key resolves CRB's row, because CRB is first in book
    /// order. For this collision that is harmless -- the two rows'
    /// ingested fields other than `category` are identical, so
    /// `description_completion` returns the same text either way -- and
    /// the desktop picker shows two rows whose `source` and `category`
    /// tell them apart. The assertion is exact so that a *different*
    /// second feat arriving under an existing key fails here instead of
    /// silently shadowing one book's record with another's.
    #[test]
    fn cross_book_key_collisions_are_exactly_the_known_set() {
        let collide = |tables: &'static [BookFeatTable]| {
            let mut seen: BTreeMap<&'static str, RuleSetId> = BTreeMap::new();
            let mut collisions: Vec<(&'static str, RuleSetId, RuleSetId)> = Vec::new();
            for book in tables {
                for entry in book.entries {
                    match seen.insert(entry.key, book.rule_set) {
                        Some(previous) if previous != book.rule_set => {
                            collisions.push((entry.key, previous, book.rule_set));
                        }
                        _ => {}
                    }
                }
            }
            collisions
        };

        // The original review, kept intact: across the HAND-AUTHORED tables
        // `Endurance` is still the only collision, so a *new* clash between
        // two books' own ingests fails here exactly as it always did.
        assert_eq!(
            collide(hand_authored_feat_tables()),
            vec![("Endurance", RuleSetId::Crb, RuleSetId::Pu)]
        );

        // The joined catalog carries two more, and both are correct rather
        // than defects: a feat one book reprints out of another is a record
        // in *both* books, and this lane's predicate is "a record this book's
        // own table does not hold". Each was checked against its owning
        // corpus record, not inferred from the shared name:
        //
        // * `Feral Combat Training` — `up_feats.lst` carries the comment
        //   "Feral Combat Training copied from Ultimate Combat - consider
        //   INCLUDEing (and .MODding) it" immediately above the record. The
        //   corpus states the reprint itself.
        // * `Extended Animal Focus` — one record in `uw_feats.lst`, the same
        //   Hunter animal-focus feat ACG prints; Ultimate Wilderness reprints
        //   it because it is the book that expands animal focus.
        //
        // `SD31-E6-F8-002` adds three more, and NONE of them is a reprint —
        // each is two genuinely DIFFERENT feats that happen to share a
        // display name, verified against both corpus records' own `DESC:`/
        // `BENEFIT:` text (Decision 10's "a shared NAME is not a duplicate"
        // guard, checked here even though this lane is not the Supersession
        // Register):
        //
        // * `Returning Throw` — `up_feats.lst` (Ultimate Psionics, TYPE
        //   `Psionic.MarksmanBonus`): "Thrown weapons return to your hand."
        //   `isr_feats.lst` (Inner Sea Races, TYPE `Combat.Teamwork`,
        //   `PRERACE:1,RACESUBTYPE=Goblinoid`): a goblinoid-only teamwork
        //   feat about catching an ally's missed thrown weapon. Different
        //   mechanics, different prerequisites, different books.
        // * `Desert Dweller` — `uw_feats.lst` (Ultimate Wilderness,
        //   `PREABILITY:...Favored Terrain ~ Desert`) vs `iswg_feats.lst`
        //   (Inner Sea World Guide, `PRESKILL:Survival=1`+`PRESTAT:CON=13`,
        //   no Favored Terrain requirement at all). Different prerequisite
        //   structure, different `BENEFIT:` text.
        // * `Strangler` — `uc_feats.lst` (Ultimate Combat, grapple/sneak-
        //   attack feat: "spend a swift action to deal your sneak attack
        //   damage") vs `mc_feats.lst` (Monster Codex, lasso feat: "choke
        //   foes with a lasso"). Unrelated combat maneuvers.
        //
        // Pinned exactly, so a further collision — reprint or coincidence —
        // still fails here until it too is checked against its own corpus
        // text.
        let all_collisions = collide(all_feat_tables());
        let (mythic_collisions, other_collisions): (Vec<_>, Vec<_>) =
            all_collisions.into_iter().partition(|(_, _, second)| *second == RuleSetId::Mythic);
        assert_eq!(
            other_collisions,
            vec![
                ("Endurance", RuleSetId::Crb, RuleSetId::Pu),
                ("Extended Animal Focus", RuleSetId::Acg, RuleSetId::Uw),
                ("Feral Combat Training", RuleSetId::Uc, RuleSetId::Upsi),
                ("Returning Throw", RuleSetId::Upsi, RuleSetId::Isr),
                ("Desert Dweller", RuleSetId::Uw, RuleSetId::Iswg),
                ("Strangler", RuleSetId::Uc, RuleSetId::MonsterCodex),
            ]
        );

        // `SD31-E6-F2-007` -- `RuleSetId::Mythic`'s 142 collisions are not
        // hand-enumerated the way the six above are: `decisions.md §10`'s
        // AMENDMENT already establishes, as standing doctrine, that a
        // Mythic feat sharing a key with the base feat it upgrades is the
        // paradigm VARIANT case, not a reprint -- re-litigating each of 142
        // records by hand would restate the operator's own ruling, not
        // verify anything new. What this loop checks INSTEAD is the
        // mechanical, per-record fact that makes the doctrine apply here:
        // every colliding Mythic row's own `PREABILITY:` prerequisite names
        // that exact key under `CATEGORY=FEAT`, i.e. the corpus itself
        // states "you must already hold the base feat to take its mythic
        // form" -- proof of variant-hood a coincidental name clash could
        // never carry. A future collision that is NOT a real mythic-upgrade
        // (a corpus edit, or a new book whose feat happens to share a name)
        // fails this loop rather than sliding in silently.
        assert_eq!(mythic_collisions.len(), 142, "re-derive if a book's feat gap rows change");
        let mythic_table = all_feat_tables()
            .iter()
            .find(|book| book.rule_set == RuleSetId::Mythic)
            .expect("RuleSetId::Mythic must be in the joined catalog");
        for (key, _first, _second) in &mythic_collisions {
            let entry = mythic_table
                .entries
                .iter()
                .find(|e| e.key == *key)
                .unwrap_or_else(|| panic!("Mythic table missing colliding key {key:?}"));
            let prereqs = entry.prerequisites.unwrap_or_default();
            let names_base_feat = prereqs
                .iter()
                .any(|p| p.starts_with("PREABILITY:") && p.contains("CATEGORY=FEAT") && p.contains(key));
            assert!(
                names_base_feat,
                "Mythic feat {key:?} collides with an earlier book but its own \
                 PREABILITY prerequisite does not name {key:?} under CATEGORY=FEAT -- \
                 this is the mechanical proof of variant-hood and it is missing, so \
                 this collision needs real per-record verification before it is trusted",
            );
        }

        // ... and it really is the same feat re-listed, not a name clash
        // between two different ones: the CRB and PU rows carry the
        // corpus's own identical `DESC:` text, and only the block-derived
        // category differs.
        let rows: Vec<&FeatCatalogRecord> = all_feat_tables()
            .iter()
            .flat_map(|book| book.entries.iter())
            .filter(|entry| entry.key == "Endurance")
            .collect();
        // `SD31-E6-F2-007` -- a third "Endurance" row now exists, Mythic
        // Adventures' own mythic upgrade of the feat (its `PREABILITY:
        // ...,CATEGORY=FEAT,Endurance` prerequisite is checked in the loop
        // above, alongside every other Mythic collision). CRB and PU stay
        // the first two, in the same table order `all_feat_tables()` always
        // yields.
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].description, rows[1].description);
        assert_eq!(rows[0].category, "General");
        assert_eq!(rows[1].category, "WoundThreshold");
        assert_eq!(rows[2].category, "Mythic");
    }
}
