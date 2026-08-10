//! Ultimate Psionics (UPsi) feat catalog. SD28-E29 slice 1 -- the last
//! Ultimate book, and the first non-Paizo one (Dreamscarred Press).
//!
//! **License posture, checked before ingesting a single record per this
//! epic's own dispatch brief.** `ultimate_psionics.pcc` declares
//! `ISOGL:YES` and a real, non-empty `OGL.txt` (90 lines, the genuine
//! Open Game License v1.0a text) sits on disk in this book's own corpus
//! directory. Unlike UC (`decisions.md §46`'s licensing note: `.pcc`
//! declared `#EXTRAFILE:OGL.txt` but the file was missing from disk),
//! this `.pcc` carries **no `#EXTRAFILE:OGL.txt` directive at all** --
//! the declaration and the file agree by omission rather than by a
//! promise the disk does not keep. No licensing anomaly found; recorded
//! as checked, not assumed.
//!
//! **Corpus coverage, honestly bounded.** `up_feats.lst` has 223
//! top-level `CATEGORY:FEAT` records (re-derived: `grep -c $'\tCATEGORY:
//! FEAT\t' up_feats.lst` -- the naive line-anchored
//! `grep -c '^CATEGORY:FEAT'` returns 0, the same not-line-anchored trap
//! `decisions.md §46`/`§49` already documented for UC/UM, recurring a
//! third time). A sibling file, `up_feats_apg.lst`, carries only three
//! `CATEGORY=FEAT|<Name>.MOD` rows tagging existing APG feats into a
//! psionic archetype's bonus-feat pool (`TYPE:MarksmanBonus` etc.) --
//! not new declarations, the same facet-tagging shape `decisions.md §48`
//! catalogued for 19 of APG's own no-prose `.MOD` rows.
//!
//! **One record is source-disabled by the data team itself, not by this
//! ingest.** `#Network Power` (`up_feats.lst:217`) carries a literal `#`
//! prefix on its own name field -- PCGen's own convention for hiding a
//! record from the UI without deleting it -- and the preceding line
//! carries the PCGen data team's own comment: `# COMMENT: I believe
//! Network Power was removed on purpose.` (`up_feats.lst:216`). Excluded
//! on the strength of the source's own annotation, a fourth kind of
//! "this row is not real content" case this bundle has hit, distinct
//! from UC's textless stubs, UM's auto-grant wrappers, or a cross-book
//! collision.
//!
//! **One cross-book collision: `Feral Combat Training` is a verbatim
//! republish of `ultimate_combat`'s own record** (`uc_feats.lst:117`) --
//! same description, same `BENEFIT:`, same `SOURCEPAGE:p.101`, same
//! prerequisite token. Confirmed at runtime against every other book's
//! real feat key set (a scratch `#[test]` dump of
//! `feats_all::all_feat_tables()`, `decisions.md §44`'s lesson applied
//! from the start, removed before commit), not assumed from the name
//! match alone. **Excluded**, the same treatment UE's 55 and UW's 1
//! collision already got.
//!
//! **Final catalog: 221 real, distinct records** (223 raw − 1
//! source-disabled − 1 cross-book collision).
//!
//! **This book's own DESC:/BENEFIT: convention is materially different
//! from every Paizo book ingested so far, and this is the slice's own
//! corpus-shape finding, not a defect.** 216 of the 221 kept records
//! carry `DESC:` alone with **no `BENEFIT:` token at all in the whole
//! book's feat file** for those records -- unlike the Paizo convention
//! (`DESC:` as short flavour text, `BENEFIT:` as the real mechanical
//! rules), Dreamscarred Press's own `DESC:` token *is* the complete
//! rules text (e.g. `Psionic Body`: `"+2 hit points for each psionic
//! feat you have"`). Only 5 records carry both tokens (`Piranha Strike`,
//! `Psionic Shot`, `Psionic Talent`, `Unwilling Participant`, `Urban
//! Tracking`), and all 5 read as genuine reprints/adaptations of an
//! existing Paizo-style split, not evidence UP is inconsistent -- they
//! were checked individually, not assumed. Zero records carry neither
//! token (unlike UC's 2 and UM's 3 auto-grant wrappers) -- this book's
//! `DESC:`-only convention means there is no textless-stub category to
//! find here at all, a genuinely different corpus shape from every prior
//! book, not a stub-doctrine violation once understood: `description` is
//! joined from `(DESC, BENEFIT)` exactly as every other book's mapper
//! already does, and 216 records correctly serve `DESC:` alone as their
//! complete text, the same honest treatment UM's 15 `Masterpiece` feats
//! (`decisions.md §49`) already established for DESC-only-by-design
//! records.
//!
//! **No new unmodelled `PRE`-family kind.** Every `PRE` token this
//! book's feats carry (`PREABILITY`/`!PREABILITY`, `PREALIGN`,
//! `PRECLASS`, `PREMULT`, `PRESKILL`, `PRESPELL`, `PRESTAT`, `PRETEXT`,
//! `PRETOTALAB`, `PREVAREQ`, `PREVARGT`, `PREVARGTEQ`) already has a
//! modelled or declared-unmodelled arm in `pre_tokens.rs` from earlier
//! books -- unlike UC's `PREDR`/`PRERULE` and UM's `PREDEITY`/
//! `PREVARLTEQ`, this book breaks the "every book adds a new PRE kind"
//! streak. Checked directly, not assumed from the streak.
//!
//! **One corpus typo, corrected with the correction documented, not
//! silently absorbed.** `Thundering Power` (`up_feats.lst:329`) declares
//! `TYPE:Metasionic` -- every one of its 34 sibling metapsionic feats
//! (`Chain Power`, `Burning Power`, etc., all named `<Word> Power`, all
//! in the same page range) declares `TYPE:Metapsionic`. Folded into
//! `FeatCategory::Metapsionic` (35 total) rather than kept as its own
//! one-record `Metasionic` category, which would read to a future
//! reader as an unattributed engine bug rather than a corpus typo.
//!
//! **No `KEY:` token on any record**, so `key == name` for every entry.
//!
//! **`category` is UPsi's own enum, not the shared `crb::feats::FeatCategory`.**
//! UPsi introduces `Psionic` (the book's own dominant facet, 153 of 221
//! records) and `Metapsionic` (the metamagic-equivalent for psionics, 35
//! records) with no shared equivalent; `General`, `Combat`,
//! `ItemCreation` map onto the shared vocabulary the same way every
//! other book's own `as_shared` does.
//!
//! **`prerequisites` carries every real `PRE`-family token verbatim**,
//! gathered directly at ingest, `None` when the corpus row has none.
//!
//! Every field below is copied verbatim from the real corpus row (source:
//! `~/workspace/repos/pcgen/data/pathfinder/dreamscarred_press/
//! ultimate_psionics/up_feats.lst`), generated programmatically by a
//! one-off extraction script, not hand-transcribed.

use super::super::crb::feats::FeatCategory as SharedFeatCategory;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatCategory {
    General,
    Combat,
    ItemCreation,
    Psionic,
    Metapsionic,
}

impl FeatCategory {
    /// Maps onto the shared `crb::feats::FeatCategory` vocabulary where
    /// one exists -- `None` for UPsi's own `Psionic`/`Metapsionic`
    /// facets, mirroring every other book's own `as_shared` rule for its
    /// own book-specific facets without checking.
    pub fn as_shared(self) -> Option<SharedFeatCategory> {
        match self {
            FeatCategory::General => Some(SharedFeatCategory::General),
            FeatCategory::Combat => Some(SharedFeatCategory::Combat),
            FeatCategory::ItemCreation => Some(SharedFeatCategory::ItemCreation),
            FeatCategory::Psionic | FeatCategory::Metapsionic => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpsiFeatEntry {
    /// The record's corpus identity. No record in this catalog carries a
    /// distinct `KEY:` token, so `key == name` for every entry.
    pub key: &'static str,
    pub category: FeatCategory,
    pub name: &'static str,
    /// The corpus `DESC:` token, verbatim. For 216 of this catalog's 221
    /// records, this is the record's *only* content -- Dreamscarred
    /// Press's own convention, not a stub (see this module's own doc
    /// comment).
    pub description: Option<&'static str>,
    /// The corpus `PRETEXT:` token, verbatim display prerequisite prose --
    /// `None` when the row carries no `PRETEXT:`.
    pub pretext: Option<&'static str>,
    pub source_page: Option<&'static str>,
    /// The corpus `BENEFIT:` token, verbatim -- present on only 5 of
    /// this catalog's 221 records (see this module's own doc comment).
    pub benefit: Option<&'static str>,
    /// Every top-level `PRE`-family token the corpus record carries,
    /// verbatim and unparsed, in source order. `None` when the row has no
    /// `PRE`-family token.
    pub prerequisites: Option<&'static [&'static str]>,
}

/// Full UPsi feat catalog: 221 real, distinct corpus records, in source
/// order. Built once and cached for the process lifetime.
pub fn feat_tables() -> &'static [UpsiFeatEntry] {
    static TABLE: std::sync::OnceLock<Vec<UpsiFeatEntry>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        vec![
            // Access Psionic Talent -- up_feats.lst:18
            UpsiFeatEntry {
                key: "Access Psionic Talent",
                category: FeatCategory::Psionic,
                name: "Access Psionic Talent",
                description: Some("You are able to manifest minor psionic abilities; gain %1 psionic talents.|max(5,BonusPsionicTalents)"),
                pretext: None,
                source_page: Some("p.86"),
                benefit: None,
                prerequisites: Some(&["PREMULT:1,[PREVARGTEQ:MaxManifesterLVL,1],[PREABILITY:1,CATEGORY=FEAT,Unlocked Talent]", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Additional Terror -- up_feats.lst:19
            UpsiFeatEntry {
                key: "Additional Terror",
                category: FeatCategory::Psionic,
                name: "Additional Terror",
                description: Some("You gain an additional terror."),
                pretext: None,
                source_page: Some("p.86"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Dread ~ Terror", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Advanced Archer Path -- up_feats.lst:20
            UpsiFeatEntry {
                key: "Advanced Archer Path",
                category: FeatCategory::Psionic,
                name: "Advanced Archer Path",
                description: Some("You gain +%1 to your damage rolls for attacks made with ranged or thrown weapons.  In addition, when using the Archer maneuver, you may perform a bull rush attempt instead of a trip attempt.|AdvancedArcherPathBonus"),
                pretext: None,
                source_page: Some("p.86"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Archer Path ~ First,Archer Path ~ Second", "PREABILITY:2,CATEGORY=FEAT,Point-Blank Shot,Precise Shot", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Advanced Ascetic Path -- up_feats.lst:21
            UpsiFeatEntry {
                key: "Advanced Ascetic Path",
                category: FeatCategory::Psionic,
                name: "Advanced Ascetic Path",
                description: Some("You gain +%1 competence bonus to both AC and saves.  In addition, when using the Ascetic maneuver, you can deflect one attack as if using the Deflect Arrows feat, although you may alternately deflect a melee attack.|AdvancedAsceticPathBonus"),
                pretext: None,
                source_page: Some("p.86"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Ascetic Path ~ First,Ascetic Path ~ Second", "PREABILITY:2,CATEGORY=FEAT,Dodge,Psionic Dodge", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Advanced Assassin Path -- up_feats.lst:22
            UpsiFeatEntry {
                key: "Advanced Assassin Path",
                category: FeatCategory::Psionic,
                name: "Advanced Assassin Path",
                description: Some("When using the Assassin trance, you deal sneak attack as a rogue of half your psychic warrior level.  Your target must be eligible for sneak attack.  In addition, when using the Assassin maneuver, your opponent is staggered for %1 rounds (Fort DC %2 negates).|AdvancedAssassinPathDuration|AdvancedAssassinPathDC"),
                pretext: None,
                source_page: Some("p.87"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Assassin's Path ~ First,Assassin's Path ~ Second", "PREABILITY:2,CATEGORY=FEAT,Deep Impact,Psionic Weapon", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Advanced Brawling Path -- up_feats.lst:23
            UpsiFeatEntry {
                key: "Advanced Brawling Path",
                category: FeatCategory::Psionic,
                name: "Advanced Brawling Path",
                description: Some("When using the Brawling trance, you gain a +%1 to your grapple checks.  In addition, when using the Brawling maneuver, the damage you deal can be lethal, instead of non-lethal.|AdvancedBrawlingPathBonus"),
                pretext: None,
                source_page: Some("p.87"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Brawling Path ~ First,Brawling Path ~ Second", "PREABILITY:2,CATEGORY=FEAT,Improved Grapple,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Advanced Constructs -- up_feats.lst:24
            UpsiFeatEntry {
                key: "Advanced Constructs",
                category: FeatCategory::Psionic,
                name: "Advanced Constructs",
                description: Some("You gain additional menu options for astral constructs."),
                pretext: None,
                source_page: Some("p.87"),
                benefit: None,
                prerequisites: Some(&["PRESPELL:1,Astral Construct", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Advanced Dervish Path -- up_feats.lst:25
            UpsiFeatEntry {
                key: "Advanced Dervish Path",
                category: FeatCategory::Psionic,
                name: "Advanced Dervish Path",
                description: Some("When using the Dervish trance, you gain a +%1 competence bonus to your damage rolls.  In addition, you can use the Dervish maneuver even if you moved before your attack and you have no limit of how many 5-foot steps you can take while using the Dervish maneuver, as long as you make at least one attack before each 5-foot step.  The maximum distance you can move in this round is that of a double move.|AdvancedDervishPathBonus"),
                pretext: None,
                source_page: Some("p.87"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Dervish Path ~ First,Dervish Path ~ Second", "PREABILITY:2,CATEGORY=FEAT,Double Slice,Two-Weapon Fighting", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PRESTAT:1,DEX=15", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Advanced Feral Path -- up_feats.lst:26
            UpsiFeatEntry {
                key: "Advanced Feral Path",
                category: FeatCategory::Psionic,
                name: "Advanced Feral Path",
                description: Some("When using the Feral trance, you gain a +%1 competence bonus to your damage rolls from natural attacks.  In addition, when using the Feral maneuver, if two natural attacks in the same charge are successful, your natural weapon deal extra damage equal to the damage dealt by one natural attack plus 1-1/2 times your Strength modifier (if the two natural addacks deal different amounts of damage, use the lesser of the two).|AdvancedFeralPathBonus"),
                pretext: None,
                source_page: Some("p.87"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Feral Path ~ First,Feral Path ~ Second", "PREABILITY:2,CATEGORY=FEAT,Psionic Fist,Unavoidable Strike", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Advanced Infiltrator Path -- up_feats.lst:27
            UpsiFeatEntry {
                key: "Advanced Infiltrator Path",
                category: FeatCategory::Psionic,
                name: "Advanced Infiltrator Path",
                description: Some("When using the Infiltrator trance, you gain a +%1 competence bonus to Bluff and Sense Motive checks.  In addition, when using the Infiltrator maneuver, for the duration of your metamorphosis power, you gain the scent extraordinary ability and +%1 to Survival checks.|AdvancedInfiltratorPathBonus"),
                pretext: None,
                source_page: Some("p.94"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Infiltrator Path ~ First,Infiltrator Path ~ Second", "PREABILITY:1,CATEGORY=FEAT,Deceitful", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Advanced Interceptor Path -- up_feats.lst:28
            UpsiFeatEntry {
                key: "Advanced Interceptor Path",
                category: FeatCategory::Psionic,
                name: "Advanced Interceptor Path",
                description: Some("When using the Interceptor trance, you can take a -%1 penalty to Armor Class to grant a deflection bonus to Armor Class to one ally threatened by an enemy within your melee reach.  In addition, when using the Interceptor maneuver, if your attack was successful, you can make a free disarm or trip attempt against the enemy struck.|AdvancedInterceptorPathBonus"),
                pretext: None,
                source_page: Some("p.94"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Interceptor Path ~ First,Interceptor Path ~ Second", "PREABILITY:1,CATEGORY=FEAT,Combat Expertise", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Advanced Mind Knight Path -- up_feats.lst:29
            UpsiFeatEntry {
                key: "Advanced Mind Knight Path",
                category: FeatCategory::Psionic,
                name: "Advanced Mind Knight Path",
                description: Some("As long as you have both Mind Knight path powers, you can manifest both Mind Knight path powers at the same time as a single standard action.  Any time you manifest call weaponry, you may select one other 1st level power you know; you may manifest that power using Martial Power as if it were a Path power, as long as the attack is made using the weapon summoned.  In addition, when using the Mind Knight maneuver, you gain a +%1 compteence bonus on the attack rolls.|AdvancedMindKnightPathBonus"),
                pretext: None,
                source_page: Some("p.94"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Mind Knight Path ~ First,Mind Knight Path ~ Second", "PREABILITY:1,CATEGORY=FEAT,Psionic Meditation", "PREABILITY:1,CATEGORY=FEAT,Psionic Shot,Psionic Weapon", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Advanced Survivor Path -- up_feats.lst:30
            UpsiFeatEntry {
                key: "Advanced Survivor Path",
                category: FeatCategory::Psionic,
                name: "Advanced Survivor Path",
                description: Some("When using the Survivor trance, you gain %1 resistance to your active energy type.  In addition, when using the Survivor maneuver, the bonus to Will saves can instead apply to Fortitude saves.|AdvancedSurvivorPathResistance"),
                pretext: None,
                source_page: Some("p.94"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Survivor Path ~ First,Survivor Path ~ Second", "PREABILITY:2,CATEGORY=FEAT,Mind Over Body,Toughness", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PRESTAT:1,CON=13", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Advanced Weaponmaster Path -- up_feats.lst:31
            UpsiFeatEntry {
                key: "Advanced Weaponmaster Path",
                category: FeatCategory::Psionic,
                name: "Advanced Weaponmaster Path",
                description: Some("When using the Weaponmaster trance, you gain a +%1 competence bonus to damage rolls made with a weapon.  In addition, when using the Weaponmaster maneuver, you can choose to make a free disarm attempt against your attacker in place of the single melee attack.  If the disarm is successful, you may then make a single melee attack against your opponent in place of the 5-foot step.|AdvancedWeaponmasterPathBonus"),
                pretext: None,
                source_page: Some("p.94"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Weaponmaster Path ~ First,Weaponmaster Path ~ Second", "PREABILITY:3,CATEGORY=FEAT,Combat Expertise,Improved Disarm,Weapon Focus", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:MaxManifesterLVL,10", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Aligned Attack (Chaos) -- up_feats.lst:32
            UpsiFeatEntry {
                key: "Aligned Attack (Chaos)",
                category: FeatCategory::Psionic,
                name: "Aligned Attack (Chaos)",
                description: Some("Your attacks gain alignment"),
                pretext: None,
                source_page: Some("p.94"),
                benefit: None,
                prerequisites: Some(&["!PREABILITY:1,CATEGORY=FEAT,TYPE=AlignedAttack", "PREALIGN:CG,CN,CE", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Aligned Attack (Evil) -- up_feats.lst:33
            UpsiFeatEntry {
                key: "Aligned Attack (Evil)",
                category: FeatCategory::Psionic,
                name: "Aligned Attack (Evil)",
                description: Some("Your attacks gain alignment"),
                pretext: None,
                source_page: Some("p.94"),
                benefit: None,
                prerequisites: Some(&["!PREABILITY:1,CATEGORY=FEAT,TYPE=AlignedAttack", "PREALIGN:CE,NE,LE", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Aligned Attack (Good) -- up_feats.lst:34
            UpsiFeatEntry {
                key: "Aligned Attack (Good)",
                category: FeatCategory::Psionic,
                name: "Aligned Attack (Good)",
                description: Some("Your attacks gain alignment"),
                pretext: None,
                source_page: Some("p.94"),
                benefit: None,
                prerequisites: Some(&["!PREABILITY:1,CATEGORY=FEAT,TYPE=AlignedAttack", "PREALIGN:CG,NG,LG", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Aligned Attack (Law) -- up_feats.lst:35
            UpsiFeatEntry {
                key: "Aligned Attack (Law)",
                category: FeatCategory::Psionic,
                name: "Aligned Attack (Law)",
                description: Some("Your attacks gain alignment"),
                pretext: None,
                source_page: Some("p.94"),
                benefit: None,
                prerequisites: Some(&["!PREABILITY:1,CATEGORY=FEAT,TYPE=AlignedAttack", "PREALIGN:LG,LN,LE", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Assassin's Shot -- up_feats.lst:36
            UpsiFeatEntry {
                key: "Assassin's Shot",
                category: FeatCategory::Combat,
                name: "Assassin's Shot",
                description: Some("Once per round when making a successful ranged attack that deals sneak attack damage, you can activate any one style ability that requires expending your psionic focus without having to expend your psionic focus."),
                pretext: None,
                source_page: Some("p.94"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Marksman ~ Sniper Style", "PREVARGTEQ:SneakAttackDice,1"]),
            },
            // Assassin's Venom -- up_feats.lst:37
            UpsiFeatEntry {
                key: "Assassin's Venom",
                category: FeatCategory::Psionic,
                name: "Assassin's Venom",
                description: Some("You gain a bonus on the save DC of prevenom or prevenom weapon equal to the competence bonus granted by your trance class feature."),
                pretext: None,
                source_page: Some("p.94"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Assassin's Path ~ First,Assassin's Path ~ Second", "PREABILITY:1,CATEGORY=Special Ability,Prevenom Path Power,Prevenom Weapon Path Power", "PREMULT:1,[PRETOTALAB:3],[PREVARGTEQ:PrereqBAB,3]", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Autonomous -- up_feats.lst:38
            UpsiFeatEntry {
                key: "Autonomous",
                category: FeatCategory::General,
                name: "Autonomous",
                description: Some("+2 on Autohypnosis and Knowledge (Psionics) checks"),
                pretext: None,
                source_page: Some("p.94"),
                benefit: None,
                prerequisites: None,
            },
            // Body Fuel -- up_feats.lst:43
            UpsiFeatEntry {
                key: "Body Fuel",
                category: FeatCategory::Psionic,
                name: "Body Fuel",
                description: Some("Take ability burn to recover power points"),
                pretext: None,
                source_page: Some("p.95"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Boost Construct -- up_feats.lst:44
            UpsiFeatEntry {
                key: "Boost Construct",
                category: FeatCategory::Psionic,
                name: "Boost Construct",
                description: Some("Astral construct gains additional ability"),
                pretext: None,
                source_page: Some("p.95"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Broken Dreams Style -- up_feats.lst:45
            UpsiFeatEntry {
                key: "Broken Dreams Style",
                category: FeatCategory::Psionic,
                name: "Broken Dreams Style",
                description: Some("When performing an unarmed attack, you can choose to also deliver your devastating touch damage through that attack."),
                pretext: None,
                source_page: Some("p.95"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Dread ~ Devastating Touch", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRESKILL:1,Intimidate=3", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Burning Power -- up_feats.lst:46
            UpsiFeatEntry {
                key: "Burning Power",
                category: FeatCategory::Metapsionic,
                name: "Burning Power",
                description: Some("Do extra damage with acid or fire power."),
                pretext: None,
                source_page: Some("p.95"),
                benefit: None,
                prerequisites: None,
            },
            // Burrowing Power -- up_feats.lst:47
            UpsiFeatEntry {
                key: "Burrowing Power",
                category: FeatCategory::Metapsionic,
                name: "Burrowing Power",
                description: Some("Bypass barrier with power"),
                pretext: None,
                source_page: Some("p.95"),
                benefit: None,
                prerequisites: Some(&["PRESKILL:1,Spellcraft=8", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Chain Power -- up_feats.lst:52
            UpsiFeatEntry {
                key: "Chain Power",
                category: FeatCategory::Metapsionic,
                name: "Chain Power",
                description: Some("Choose additional targets with power"),
                pretext: None,
                source_page: Some("p.95"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Channel Rage -- up_feats.lst:53
            UpsiFeatEntry {
                key: "Channel Rage",
                category: FeatCategory::Psionic,
                name: "Channel Rage",
                description: Some("When you choose to power a manifestation with only your wild surge, you may spend 1 round of your rage instead of expending your psionic focus."),
                pretext: None,
                source_page: Some("p.96"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=Special Ability,Wilder ~ Wild Surge,Rage", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Cloak Dance -- up_feats.lst:54
            UpsiFeatEntry {
                key: "Cloak Dance",
                category: FeatCategory::General,
                name: "Cloak Dance",
                description: Some("Gain concealment as a move action, total concealment as a full-round action"),
                pretext: None,
                source_page: Some("p.96"),
                benefit: None,
                prerequisites: Some(&["PRESKILL:2,Stealth=7,Perform (Dance)=2"]),
            },
            // Combat Manifestation -- up_feats.lst:55
            UpsiFeatEntry {
                key: "Combat Manifestation",
                category: FeatCategory::Psionic,
                name: "Combat Manifestation",
                description: Some("+4 bonus on concentration checks for defensive manifesting"),
                pretext: None,
                source_page: Some("p.96"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Concussive Power -- up_feats.lst:56
            UpsiFeatEntry {
                key: "Concussive Power",
                category: FeatCategory::Metapsionic,
                name: "Concussive Power",
                description: Some("Disorient with sonic power."),
                pretext: None,
                source_page: Some("p.96"),
                benefit: None,
                prerequisites: None,
            },
            // Craft Crystalline Focus -- up_feats.lst:57
            UpsiFeatEntry {
                key: "Craft Crystalline Focus",
                category: FeatCategory::ItemCreation,
                name: "Craft Crystalline Focus",
                description: Some("You can craft crystalline focus items that harness psionic energy such as that for a mind blade, mind armor, or astral suit."),
                pretext: None,
                source_page: Some("p.96"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:MaxManifesterLVL,3", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Craft Cognizance Crystal -- up_feats.lst:58
            UpsiFeatEntry {
                key: "Craft Cognizance Crystal",
                category: FeatCategory::ItemCreation,
                name: "Craft Cognizance Crystal",
                description: Some("Create congnizance crystals"),
                pretext: None,
                source_page: Some("p.96"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:MaxManifesterLVL,3"]),
            },
            // Crippling Assault -- up_feats.lst:59
            UpsiFeatEntry {
                key: "Crippling Assault",
                category: FeatCategory::Psionic,
                name: "Crippling Assault",
                description: Some("You can paralyze targets of your ranged attacks."),
                pretext: None,
                source_page: Some("p.96"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Intimidating Shot,Point-Blank Shot,Staggering Shot", "PREMULT:1,[PRETOTALAB:11],[PREVARGTEQ:PrereqBAB,11]"]),
            },
            // Critical Refocus -- up_feats.lst:60
            UpsiFeatEntry {
                key: "Critical Refocus",
                category: FeatCategory::Combat,
                name: "Critical Refocus",
                description: Some("When you confirm a critical hit on an attack where you expended your psionic focus, you automatically regain psionic focus after the results of the attack are determined."),
                pretext: None,
                source_page: Some("p.96"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Improved Critical", "PREMULT:1,[PRETOTALAB:8],[PREVARGTEQ:PrereqBAB,8]", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Cushion the Blow -- up_feats.lst:61
            UpsiFeatEntry {
                key: "Cushion the Blow",
                category: FeatCategory::Psionic,
                name: "Cushion the Blow",
                description: Some("As long as you are maintaining psoinic focus, as an immediate action, you can minimize the damage of a single attack with a ranged, melee, or natural weapon or unarmed strike (treat all dice rolled as 1s) on any member of the collective.  This ability must be used in response to an attack on a member of the collective.  In addition, if you expend your psionic focus, you can reduce any additional damage on the attack (such as bonus damage from a high Strength or sneak attack damage) by your key ability modifier."),
                pretext: None,
                source_page: Some("p.96"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Collective", "PRESTAT:1,WIS=15", "PREVARGTEQ:MaxManifesterLVL,6", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Dazing Power -- up_feats.lst:66
            UpsiFeatEntry {
                key: "Dazing Power",
                category: FeatCategory::Metapsionic,
                name: "Dazing Power",
                description: Some("Daze creatures with power."),
                pretext: None,
                source_page: Some("p.96"),
                benefit: None,
                prerequisites: None,
            },
            // Deadly Throw -- up_feats.lst:68
            UpsiFeatEntry {
                key: "Deadly Throw",
                category: FeatCategory::Psionic,
                name: "Deadly Throw",
                description: Some("You can replace power with finesse for thrown weapons."),
                pretext: None,
                source_page: Some("p.97"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot", "PREMULT:1,[PRETOTALAB:3],[PREVARGTEQ:PrereqBAB,3]", "PRESTAT:1,DEX=15"]),
            },
            // Deep Focus -- up_feats.lst:69
            UpsiFeatEntry {
                key: "Deep Focus",
                category: FeatCategory::Psionic,
                name: "Deep Focus",
                description: Some("You can psionically focus your subconscious in the same manner in which you gain psionic focus normally.  You cannot benefit from both Psicrystal Containment and Deep Focus at the same time."),
                pretext: None,
                source_page: Some("p.97"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Psionic Body", "PRESKILL:1,Autohypnosis=4", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Deep Impact -- up_feats.lst:70
            UpsiFeatEntry {
                key: "Deep Impact",
                category: FeatCategory::Psionic,
                name: "Deep Impact",
                description: Some("Resolve melee attack as a touch attack"),
                pretext: None,
                source_page: Some("p.97"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Psionic Weapon", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PRESTAT:1,STR=13", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Defensive Tactics -- up_feats.lst:72
            UpsiFeatEntry {
                key: "Defensive Tactics",
                category: FeatCategory::General,
                name: "Defensive Tactics",
                description: Some("You add your levels of tactician and fighter together for purposes of your tactics and armor training class features."),
                pretext: None,
                source_page: Some("p.97"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=Special Ability,Tactician ~ Strategy,Armor Training"]),
            },
            // Delay Power -- up_feats.lst:73
            UpsiFeatEntry {
                key: "Delay Power",
                category: FeatCategory::Metapsionic,
                name: "Delay Power",
                description: Some("Delay effect of power up to 5 rounds"),
                pretext: None,
                source_page: Some("p.97"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Disciple of Fear -- up_feats.lst:74
            UpsiFeatEntry {
                key: "Disciple of Fear",
                category: FeatCategory::Psionic,
                name: "Disciple of Fear",
                description: Some("For the purpose of your devastating touch and terror class features, your effective class level increases by 4.  This benefit can't increase your effective class level to higher than your Hit Dice.  This feat does not affect your terrors known or give you further uses of your terrors, but increases it for the purposes of augmentation and save DCs."),
                pretext: None,
                source_page: Some("p.97"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=Special Ability,Dread ~ Devastating Touch,Dread ~ Terror", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Dispelling Static -- up_feats.lst:75
            UpsiFeatEntry {
                key: "Dispelling Static",
                category: FeatCategory::Psionic,
                name: "Dispelling Static",
                description: Some(" If you make a successful targeted dispel check, you may choose to have the effect unravel violently, lashing the target of the dispel with psychic static that imposes a -1 penalty on saves against any power you manifest until the end of your next turn. The penalty increases by 1 for each effect dispelled. The target also loses any psionic focus it has and cannot regain psionic focus for 1 round.  This does not affect psionic focus contained in psicrystals via Psicrystal Containment, unless the psicrystal is the target of the targeted dispel attempt."),
                pretext: None,
                source_page: Some("p.98"),
                benefit: None,
                prerequisites: Some(&["PRESKILL:1,Spellcraft=5", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Echoing Power -- up_feats.lst:80
            UpsiFeatEntry {
                key: "Echoing Power",
                category: FeatCategory::Metapsionic,
                name: "Echoing Power",
                description: Some("Manifest this power a second time for free."),
                pretext: None,
                source_page: Some("p.98"),
                benefit: None,
                prerequisites: None,
            },
            // Ectoplasmic Power -- up_feats.lst:81
            UpsiFeatEntry {
                key: "Ectoplasmic Power",
                category: FeatCategory::Metapsionic,
                name: "Ectoplasmic Power",
                description: Some("Power has full effect on incorporeal or ethereal creatures."),
                pretext: None,
                source_page: Some("p.98"),
                benefit: None,
                prerequisites: None,
            },
            // Efficient Aid -- up_feats.lst:82
            UpsiFeatEntry {
                key: "Efficient Aid",
                category: FeatCategory::Psionic,
                name: "Efficient Aid",
                description: Some("Request aid heals 4 hit points per power point."),
                pretext: None,
                source_page: Some("p.98"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Request Aid", "PRESKILL:1,Heal=7", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Elemental Blast -- up_feats.lst:83
            UpsiFeatEntry {
                key: "Elemental Blast",
                category: FeatCategory::Psionic,
                name: "Elemental Blast",
                description: Some("Your surge blast deals damage of your active energy type instead of force damage.  In addition, your surge blast is modified based upon your active energy type.  Cold: A surge blast of this energy type deals +1 point of damage per die.  Electricity: A surge blast of this energy type provides a +3 bonus on your attack roll if the target is wearing metal armor.  Fire: A surge blast of this energy type deals +1 point of damage per die.  Sonic: A surge blast of this energy type deals -1 point of damage per die and ignores an object's hardness."),
                pretext: None,
                source_page: Some("p.98"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Surge Blast", "PREVARGTEQ:IsPsionic,1", "PREVARGTEQ:WildSurge,1"]),
            },
            // Empower Power -- up_feats.lst:84
            UpsiFeatEntry {
                key: "Empower Power",
                category: FeatCategory::Metapsionic,
                name: "Empower Power",
                description: Some("Increase power's variable, numeric effects by 50%%"),
                pretext: None,
                source_page: Some("p.98"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Empowered Shot -- up_feats.lst:85
            UpsiFeatEntry {
                key: "Empowered Shot",
                category: FeatCategory::Psionic,
                name: "Empowered Shot",
                description: Some("You can send your ranged attacks farther."),
                pretext: None,
                source_page: Some("p.98"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot"]),
            },
            // Endowed Mind -- up_feats.lst:86
            UpsiFeatEntry {
                key: "Endowed Mind",
                category: FeatCategory::Metapsionic,
                name: "Endowed Mind",
                description: Some("Increase power's save DC for augmentation"),
                pretext: None,
                source_page: Some("p.98"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Enervation Fortitude -- up_feats.lst:87
            UpsiFeatEntry {
                key: "Enervation Fortitude",
                category: FeatCategory::Psionic,
                name: "Enervation Fortitude",
                description: Some("When you suffer psychic enervation, the penalties are calculated as if your wilder level were reduced by half (to a minimum of 1). [Not figured into the surge description.]"),
                pretext: None,
                source_page: Some("p.98"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Psychic Enervation", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Enhanced Steal Life -- up_feats.lst:88
            UpsiFeatEntry {
                key: "Enhanced Steal Life",
                category: FeatCategory::Psionic,
                name: "Enhanced Steal Life",
                description: Some("Increase save of Steal Life by spending power points, DC +1/3 points spent."),
                pretext: None,
                source_page: Some("p.98"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Steal Life", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Enlarge Power -- up_feats.lst:89
            UpsiFeatEntry {
                key: "Enlarge Power",
                category: FeatCategory::Metapsionic,
                name: "Enlarge Power",
                description: Some("Double power's range"),
                pretext: None,
                source_page: Some("p.99"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Enlarged Collective -- up_feats.lst:90
            UpsiFeatEntry {
                key: "Enlarged Collective",
                category: FeatCategory::Psionic,
                name: "Enlarged Collective",
                description: Some("The range of your collective is increased."),
                pretext: None,
                source_page: Some("p.99"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Collective", "PREVARGTEQ:IsPsionic,1", "PREVARGTEQ:MaxManifesterLVL,3"]),
            },
            // Expanded Collective -- up_feats.lst:92
            UpsiFeatEntry {
                key: "Expanded Collective",
                category: FeatCategory::Psionic,
                name: "Expanded Collective",
                description: Some("Add two more creatures to collective."),
                pretext: None,
                source_page: Some("p.99"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Collective", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Expanded Favored Weapon -- up_feats.lst:93
            UpsiFeatEntry {
                key: "Expanded Favored Weapon",
                category: FeatCategory::Psionic,
                name: "Expanded Favored Weapon",
                description: Some("You get an extra favored weapon group."),
                pretext: None,
                source_page: Some("p.99"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Favored Weapon", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]"]),
            },
            // Expanded Knowledge -- up_feats.lst:96
            UpsiFeatEntry {
                key: "Expanded Knowledge",
                category: FeatCategory::Psionic,
                name: "Expanded Knowledge",
                description: Some("Learn an additional power"),
                pretext: None,
                source_page: Some("p.99"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:MaxManifesterLVL,3", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Expanded Martial Power -- up_feats.lst:97
            UpsiFeatEntry {
                key: "Expanded Martial Power",
                category: FeatCategory::Psionic,
                name: "Expanded Martial Power",
                description: Some("Expend your focus to use any power you know when using your martial power ability, as long as it is at least one level lower than the highest level power you can manifest."),
                pretext: None,
                source_page: Some("p.99"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Psychic Warrior ~ Martial Power", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Expanded Strategies -- up_feats.lst:98
            UpsiFeatEntry {
                key: "Expanded Strategies",
                category: FeatCategory::Psionic,
                name: "Expanded Strategies",
                description: Some("Gain an additional strategy."),
                pretext: None,
                source_page: Some("p.99"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Strategy", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Expansive Collective -- up_feats.lst:99
            UpsiFeatEntry {
                key: "Expansive Collective",
                category: FeatCategory::Psionic,
                name: "Expansive Collective",
                description: Some("When calculating the range and number of creatures which may be joined to your collective, use your total Hit Dice instead of your class level."),
                pretext: None,
                source_page: Some("p.99"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Collective", "PREABILITY:1,CATEGORY=Special Ability,Tactician ~ Spirit of Many,Vitalist ~ Spirit of Many", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Explosive Power -- up_feats.lst:100
            UpsiFeatEntry {
                key: "Explosive Power",
                category: FeatCategory::Metapsionic,
                name: "Explosive Power",
                description: Some("Direct targeting powers that deal hit point damage deals damage to all creatures adjacent to target (Reflex half)."),
                pretext: None,
                source_page: Some("p.99"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:MaxManifesterLVL,3", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Extra Blade Skill -- up_feats.lst:101
            UpsiFeatEntry {
                key: "Extra Blade Skill",
                category: FeatCategory::Psionic,
                name: "Extra Blade Skill",
                description: Some("Gain an extra blade skill."),
                pretext: None,
                source_page: Some("p.100"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.BladeSkill,Blade Skill", "PREMULT:1,[PRETOTALAB:2],[PREVARGTEQ:PrereqBAB,2]", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Extend Power -- up_feats.lst:102
            UpsiFeatEntry {
                key: "Extend Power",
                category: FeatCategory::Metapsionic,
                name: "Extend Power",
                description: Some("Double power's duration"),
                pretext: None,
                source_page: Some("p.100"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Extended Blast -- up_feats.lst:103
            UpsiFeatEntry {
                key: "Extended Blast",
                category: FeatCategory::Psionic,
                name: "Extended Blast",
                description: Some("Increase surge blast range to 60 feet"),
                pretext: None,
                source_page: Some("p.100"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Surge Blast"]),
            },
            // Extra Customization -- up_feats.lst:104
            UpsiFeatEntry {
                key: "Extra Customization",
                category: FeatCategory::Psionic,
                name: "Extra Customization",
                description: Some("Gain an additional customization."),
                pretext: None,
                source_page: Some("p.100"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Aegis ~ Form Astral Suit", "PREVARGT:floor(AstralSuitLVL/5),count(\"ABILITIES\",\"CATEGORY=FEAT\",\"NAME=Extra Customization\")", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Extra Disruption Type -- up_feats.lst:105
            UpsiFeatEntry {
                key: "Extra Disruption Type",
                category: FeatCategory::Psionic,
                name: "Extra Disruption Type",
                description: Some("When you gain psionic focus, you can select one additional creature type as your active type."),
                pretext: None,
                source_page: Some("p.100"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Cryptic ~ Disrupt Pattern", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Extra Insight -- up_feats.lst:106
            UpsiFeatEntry {
                key: "Extra Insight",
                category: FeatCategory::Psionic,
                name: "Extra Insight",
                description: Some("You gain one additional insight."),
                pretext: None,
                source_page: Some("p.100"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Internal,Cryptic ~ Insights", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Extra Power Known -- up_feats.lst:107
            UpsiFeatEntry {
                key: "Extra Power Known",
                category: FeatCategory::Psionic,
                name: "Extra Power Known",
                description: Some("Learn an additional power from your class list."),
                pretext: None,
                source_page: Some("p.100"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:MaxManifesterLVL,1", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Extra Reconfiguration -- up_feats.lst:108
            UpsiFeatEntry {
                key: "Extra Reconfiguration",
                category: FeatCategory::Psionic,
                name: "Extra Reconfiguration",
                description: Some("You gain one more use of reconfigure."),
                pretext: None,
                source_page: Some("p.100"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=Special Ability,Aegis ~ Form Astral Suit,Aegis ~ Reconfigure", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Extra Strategy -- up_feats.lst:109
            UpsiFeatEntry {
                key: "Extra Strategy",
                category: FeatCategory::Psionic,
                name: "Extra Strategy",
                description: Some("You gain two additional uses of your strategy class feature."),
                pretext: None,
                source_page: Some("p.100"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Strategy", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Extra Terrors -- up_feats.lst:110
            UpsiFeatEntry {
                key: "Extra Terrors",
                category: FeatCategory::Psionic,
                name: "Extra Terrors",
                description: Some("You gain 3 additional daily uses of your terrors."),
                pretext: None,
                source_page: Some("p.100"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Dread ~ Terror", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Extra Transfer -- up_feats.lst:111
            UpsiFeatEntry {
                key: "Extra Transfer",
                category: FeatCategory::Psionic,
                name: "Extra Transfer",
                description: Some("You can use transfer wounds or sickening touch 2 more times per day.  If you have both transfer wounds and sickening touch, choose which ability this feat affects when you take this feat."),
                pretext: None,
                source_page: Some("p.100"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Transfer Wounds,Miasmic ~ Sickening Touch", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Fast Aid -- up_feats.lst:116
            UpsiFeatEntry {
                key: "Fast Aid",
                category: FeatCategory::Psionic,
                name: "Fast Aid",
                description: Some("Your allies may request aid as a swift action."),
                pretext: None,
                source_page: Some("p.100"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Request Aid", "PRESKILL:2,Heal=7,Spellcraft=7", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Fast Step -- up_feats.lst:117
            UpsiFeatEntry {
                key: "Fast Step",
                category: FeatCategory::Psionic,
                name: "Fast Step",
                description: Some("You may use Nomad's Step as a move action."),
                pretext: None,
                source_page: Some("p.101"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Nomad's Step ~ Psychoportation", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Favored Energy (Cold) -- up_feats.lst:121
            UpsiFeatEntry {
                key: "Favored Energy (Cold)",
                category: FeatCategory::Psionic,
                name: "Favored Energy (Cold)",
                description: Some("Any time you manifest a power that deals cold damage, the damage is increased by +1 per die."),
                pretext: Some("Able to manifest any power that deals cold damage."),
                source_page: Some("p.101"),
                benefit: None,
                prerequisites: Some(&["!PREABILITY:1,CATEGORY=FEAT,TYPE=FavoredEnergy", "PRETEXT:Able to manifest any power that deals cold damage.", "PREVARGTEQ:MaxManifesterLVL,3", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Favored Energy (Electricity) -- up_feats.lst:122
            UpsiFeatEntry {
                key: "Favored Energy (Electricity)",
                category: FeatCategory::Psionic,
                name: "Favored Energy (Electricity)",
                description: Some("Any time you manifest a power that deals electricity damage, the damage is increased by +1 per die."),
                pretext: Some("Able to manifest any power that deals electricity damage."),
                source_page: Some("p.101"),
                benefit: None,
                prerequisites: Some(&["!PREABILITY:1,CATEGORY=FEAT,TYPE=FavoredEnergy", "PRETEXT:Able to manifest any power that deals electricity damage.", "PREVARGTEQ:MaxManifesterLVL,3", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Favored Energy (Fire) -- up_feats.lst:123
            UpsiFeatEntry {
                key: "Favored Energy (Fire)",
                category: FeatCategory::Psionic,
                name: "Favored Energy (Fire)",
                description: Some("Any time you manifest a power that deals fire damage, the damage is increased by +1 per die."),
                pretext: Some("Able to manifest any power that deals fire damage."),
                source_page: Some("p.101"),
                benefit: None,
                prerequisites: Some(&["!PREABILITY:1,CATEGORY=FEAT,TYPE=FavoredEnergy", "PRETEXT:Able to manifest any power that deals fire damage.", "PREVARGTEQ:MaxManifesterLVL,3", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Favored Energy (Sonic) -- up_feats.lst:124
            UpsiFeatEntry {
                key: "Favored Energy (Sonic)",
                category: FeatCategory::Psionic,
                name: "Favored Energy (Sonic)",
                description: Some("Any time you manifest a power that deals sonic damage, the damage is increased by +1 per die."),
                pretext: Some("Able to manifest any power that deals sonic damage."),
                source_page: Some("p.101"),
                benefit: None,
                prerequisites: Some(&["!PREABILITY:1,CATEGORY=FEAT,TYPE=FavoredEnergy", "PRETEXT:Able to manifest any power that deals sonic damage.", "PREVARGTEQ:MaxManifesterLVL,3", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Fear Mastery -- up_feats.lst:125
            UpsiFeatEntry {
                key: "Fear Mastery",
                category: FeatCategory::Psionic,
                name: "Fear Mastery",
                description: Some("As long as you maintain psionic focus, you receive a +1 insight bonus to the save DCs of all your terrors and powers with the Fear descriptor.  You can expend your focus while manifesting a power or using a terror to increase this bonus to +2."),
                pretext: None,
                source_page: Some("p.101"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Dread ~ Devastating Touch", "PREABILITY:1,CATEGORY=FEAT,Psionic Endowment", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Fear's Reach -- up_feats.lst:126
            UpsiFeatEntry {
                key: "Fear's Reach",
                category: FeatCategory::Psionic,
                name: "Fear's Reach",
                description: Some("Your devastating touch ability may now be used as a ranged touch attack with a range of close (%1 ft.).|FearsReachRange"),
                pretext: None,
                source_page: Some("p.101"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Dread ~ Devastating Touch", "PREABILITY:1,CATEGORY=FEAT,Psionic Shot", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Fell Shot -- up_feats.lst:127
            UpsiFeatEntry {
                key: "Fell Shot",
                category: FeatCategory::Psionic,
                name: "Fell Shot",
                description: Some("Resolve ranged attack as a touch attack"),
                pretext: None,
                source_page: Some("p.101"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Point-Blank Shot,Psionic Shot", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PRESTAT:1,DEX=13", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Flaring Power -- up_feats.lst:129
            UpsiFeatEntry {
                key: "Flaring Power",
                category: FeatCategory::Metapsionic,
                name: "Flaring Power",
                description: Some("Dazzle creatures with fire, light, or electricity power."),
                pretext: None,
                source_page: Some("p.101"),
                benefit: None,
                prerequisites: None,
            },
            // Fighter's Blade -- up_feats.lst:130
            UpsiFeatEntry {
                key: "Fighter's Blade",
                category: FeatCategory::General,
                name: "Fighter's Blade",
                description: Some("Treat your class level as four higher to determine your enhanced mind blade class feature and which blade skills you can select, to a maximum of your character level."),
                pretext: None,
                source_page: Some("p.101"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Soulknife ~ Enhanced Mind Blade"]),
            },
            // Focused Power -- up_feats.lst:131
            UpsiFeatEntry {
                key: "Focused Power",
                category: FeatCategory::Metapsionic,
                name: "Focused Power",
                description: Some("One target power must overcome higher DC."),
                pretext: None,
                source_page: Some("p.101"),
                benefit: None,
                prerequisites: None,
            },
            // Focused Precision -- up_feats.lst:134
            UpsiFeatEntry {
                key: "Focused Precision",
                category: FeatCategory::Combat,
                name: "Focused Precision",
                description: Some("While maintaining focus, you add your Dexterity modifier to damage rolls made with a crossbow or firearm for which you have the Weapon Focus feat.  Damage from this feat is precision-based damage. [NOT IMPLEMENTED]"),
                pretext: None,
                source_page: Some("p.101"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Internal,Marksman Combat Style", "PREABILITY:1,CATEGORY=FEAT,Weapon Focus", "PREMULT:1,[PRETOTALAB:7],[PREVARGTEQ:PrereqBAB,7]", "PRESTAT:1,DEX=17", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Focused Sunder -- up_feats.lst:135
            UpsiFeatEntry {
                key: "Focused Sunder",
                category: FeatCategory::Psionic,
                name: "Focused Sunder",
                description: Some("Ignore 1/2 hardness of foe's weapon"),
                pretext: None,
                source_page: Some("p.102"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Power Attack,Improved Sunder", "PRESTAT:1,STR=13", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Ghost Attack -- up_feats.lst:140
            UpsiFeatEntry {
                key: "Ghost Attack",
                category: FeatCategory::Psionic,
                name: "Ghost Attack",
                description: Some("Reduce damage penalty for attacking incorporeal enemies"),
                pretext: None,
                source_page: Some("p.102"),
                benefit: None,
                prerequisites: Some(&["PREMULT:1,[PRETOTALAB:3],[PREVARGTEQ:PrereqBAB,3]", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Gravitic Stability -- up_feats.lst:141
            UpsiFeatEntry {
                key: "Gravitic Stability",
                category: FeatCategory::Psionic,
                name: "Gravitic Stability",
                description: Some("Any time your movement speed would be reduced to 10 ft. as a result of using personal gravity or scorn earth (such as from wearing heavy armor or being more than 1 ft. away from a stable surface), your speed is instead reduced to 10 ft. plus any bonuses to your movement (such as from freerunning). You do not take any penalty for attacking while using scorn earth at distances greater than 1 ft. from the ground or a stable surface. You gain a bonus to your CMD equal to your elocater class level against any bull rush or trip attempt."),
                pretext: None,
                source_page: Some("p.102"),
                benefit: None,
                prerequisites: Some(&["PRECLASS:1,Elocater=1", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Greater Cushion the Blow -- up_feats.lst:142
            UpsiFeatEntry {
                key: "Greater Cushion the Blow",
                category: FeatCategory::Psionic,
                name: "Greater Cushion the Blow",
                description: Some("When using the Cushion the Blow feat, the effect applies to all damage for that collective member until the beginning of your next turn."),
                pretext: None,
                source_page: Some("p.102"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Collective", "PREABILITY:2,CATEGORY=FEAT,Cushion the Blow,Improved Cushion the Blow", "PRESTAT:1,WIS=17", "PREVARGTEQ:MaxManifesterLVL,18", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Greater Intuitive Shot -- up_feats.lst:143
            UpsiFeatEntry {
                key: "Greater Intuitive Shot",
                category: FeatCategory::Psionic,
                name: "Greater Intuitive Shot",
                description: Some("Expend focus to gain +%1 damage per attack on full attack with ranged weapon.|WIS"),
                pretext: None,
                source_page: Some("p.102"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Intuitive Shot,Point-Blank Shot,Psionic Shot", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PRESTAT:1,WIS=13"]),
            },
            // Greater Power Penetration -- up_feats.lst:144
            UpsiFeatEntry {
                key: "Greater Power Penetration",
                category: FeatCategory::Psionic,
                name: "Greater Power Penetration",
                description: Some("Gain bonus to overcome power resistance"),
                pretext: None,
                source_page: Some("p.102"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Power Penetration", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Greater Power Specialization -- up_feats.lst:145
            UpsiFeatEntry {
                key: "Greater Power Specialization",
                category: FeatCategory::Psionic,
                name: "Greater Power Specialization",
                description: Some("Increased bonus damage to damaging powers"),
                pretext: None,
                source_page: Some("p.102"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Power Specialization,Weapon Focus (Ray)", "PREVARGTEQ:MaxManifesterLVL,12", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Greater Psionic Endowment -- up_feats.lst:146
            UpsiFeatEntry {
                key: "Greater Psionic Endowment",
                category: FeatCategory::Psionic,
                name: "Greater Psionic Endowment",
                description: Some("Add +2 to power's save DC"),
                pretext: None,
                source_page: Some("p.102"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Psionic Endowment", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Greater Psionic Fist -- up_feats.lst:148
            UpsiFeatEntry {
                key: "Greater Psionic Fist",
                category: FeatCategory::Psionic,
                name: "Greater Psionic Fist",
                description: Some("Unarmed attack or natural weapon deals extra damage"),
                pretext: None,
                source_page: Some("p.102"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Psionic Fist", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PRESTAT:1,STR=13", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Greater Psionic Shot -- up_feats.lst:150
            UpsiFeatEntry {
                key: "Greater Psionic Shot",
                category: FeatCategory::Psionic,
                name: "Greater Psionic Shot",
                description: Some("Ranged weapon deals extra damage"),
                pretext: None,
                source_page: Some("p.102"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Point-Blank Shot,Psionic Shot", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Greater Psionic Weapon -- up_feats.lst:152
            UpsiFeatEntry {
                key: "Greater Psionic Weapon",
                category: FeatCategory::Psionic,
                name: "Greater Psionic Weapon",
                description: Some("Melee weapon deals extra damage"),
                pretext: None,
                source_page: Some("p.103"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Psionic Weapon", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PRESTAT:1,STR=13", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Harmonic Resonance -- up_feats.lst:159
            UpsiFeatEntry {
                key: "Harmonic Resonance",
                category: FeatCategory::Psionic,
                name: "Harmonic Resonance",
                description: Some("You can choose %1 powers from your collective members when you choose powers.|max(INT,WIS,CHA)"),
                pretext: None,
                source_page: Some("p.103"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Collective", "PRESKILL:1,Spellcraft=3", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Hawkeye -- up_feats.lst:160
            UpsiFeatEntry {
                key: "Hawkeye",
                category: FeatCategory::Combat,
                name: "Hawkeye",
                description: Some("Gain +%1 on Perception; increase precision damage range by %2 ft.|if(skillinfo(\"RANK\",\"Perception\")>=10,4,2)|if(skillinfo(\"RANK\",\"Perception\")>=10,30,15)"),
                pretext: None,
                source_page: Some("p.103"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Far Shot,Point-Blank Shot"]),
            },
            // Hustle Power -- up_feats.lst:161
            UpsiFeatEntry {
                key: "Hustle Power",
                category: FeatCategory::Metapsionic,
                name: "Hustle Power",
                description: Some("Manifest a power as a move action instead of a standard action."),
                pretext: None,
                source_page: Some("p.103"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Improved Cover Fire -- up_feats.lst:166
            UpsiFeatEntry {
                key: "Improved Cover Fire",
                category: FeatCategory::Combat,
                name: "Improved Cover Fire",
                description: Some("You gain a +1 bonus on attack rolls to activate your cover fire class feature and the Reflex save DC increases by 1. When you confirm a critical hit when using your cover fire ability, you can expend your psionic focus as an immediate action; if you do and the opponent fails their Reflex save, they are stunned for one round instead of staggered. You must choose to use this ability before the Reflex save and critical confirmation roll."),
                pretext: None,
                source_page: Some("p.103"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Marksman ~ Cover Fire", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Improved Cushion the Blow -- up_feats.lst:167
            UpsiFeatEntry {
                key: "Improved Cushion the Blow",
                category: FeatCategory::Psionic,
                name: "Improved Cushion the Blow",
                description: Some("When using the Cushion the Blow feat, the effect can be applied to an additional attack with a melee, ranged, or natural weapon in the same round."),
                pretext: None,
                source_page: Some("p.103"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Collective", "PREABILITY:1,CATEGORY=FEAT,Cushion the Blow", "PRESTAT:1,WIS=15", "PREVARGTEQ:MaxManifesterLVL,12", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Improved Disengage -- up_feats.lst:168
            UpsiFeatEntry {
                key: "Improved Disengage",
                category: FeatCategory::Combat,
                name: "Improved Disengage",
                description: Some("When you expend your psionic focus to activate your disengage class feature, you may move 5 feet as a free action that does not provoke an attack of opportunity. This movement does not provoke attacks of opportunity and does not count as a 5-foot step. You may only use this ability when leaving a threatened square."),
                pretext: None,
                source_page: Some("p.103"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Marksman ~ Disengage", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Improved Disruption -- up_feats.lst:169
            UpsiFeatEntry {
                key: "Improved Disruption",
                category: FeatCategory::Psionic,
                name: "Improved Disruption",
                description: Some("Your disrupt pattern ability deals one additional point of damage per die of damage."),
                pretext: None,
                source_page: Some("p.103"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Cryptic ~ Disrupt Pattern", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Improved Metamorphosis -- up_feats.lst:170
            UpsiFeatEntry {
                key: "Improved Metamorphosis",
                category: FeatCategory::Psionic,
                name: "Improved Metamorphosis",
                description: Some("Gain one additional menu option from metamorphosis-type power."),
                pretext: None,
                source_page: Some("p.103"),
                benefit: None,
                prerequisites: Some(&["PRESPELL:1,Metamorphosis (Minor),Metamorphosis,Metamorphosis (Greater),Metamorphosis (True)", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Improved Psi-Like Ability -- up_feats.lst:175
            UpsiFeatEntry {
                key: "Improved Psi-Like Ability",
                category: FeatCategory::Psionic,
                name: "Improved Psi-Like Ability",
                description: Some("Choose one psi-like ability gained from your race.  You can use that racial psi-like ability an additional two times per day. [Not implemented.]"),
                pretext: Some("Racial psi-like ability"),
                source_page: Some("p.104"),
                benefit: None,
                prerequisites: Some(&["PRETEXT:Racial psi-like ability", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Improved Psicrystal -- up_feats.lst:176
            UpsiFeatEntry {
                key: "Improved Psicrystal",
                category: FeatCategory::Psionic,
                name: "Improved Psicrystal",
                description: Some("Enhance your psicrystal"),
                pretext: None,
                source_page: Some("p.104"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Psicrystal Affinity", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Inquisitor -- up_feats.lst:177
            UpsiFeatEntry {
                key: "Inquisitor",
                category: FeatCategory::Psionic,
                name: "Inquisitor",
                description: Some("+10 on Sense Motive checks to oppose Bluff"),
                pretext: None,
                source_page: Some("p.104"),
                benefit: None,
                prerequisites: Some(&["PRESTAT:1,WIS=13", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Insightful Terror -- up_feats.lst:178
            UpsiFeatEntry {
                key: "Insightful Terror",
                category: FeatCategory::Psionic,
                name: "Insightful Terror",
                description: Some("As a standard action you can make an Intimidate check to demoralize a single foe (within 30 ft).  If you succeed,the target is paralzed for one round.  This is a mind-affecting fear effect."),
                pretext: None,
                source_page: Some("p.104"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Dread ~ Fearsome Insight", "PRESKILL:1,Intimidate=9", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Intimidating Shot -- up_feats.lst:179
            UpsiFeatEntry {
                key: "Intimidating Shot",
                category: FeatCategory::Psionic,
                name: "Intimidating Shot",
                description: Some("Gain demoralize check on standard action to attack enemy."),
                pretext: None,
                source_page: Some("p.104"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot"]),
            },
            // Intuitive Fighting -- up_feats.lst:180
            UpsiFeatEntry {
                key: "Intuitive Fighting",
                category: FeatCategory::Psionic,
                name: "Intuitive Fighting",
                description: Some("To use this feat you must maintain psionic focus.  You add your Wisdom modifier to your melee attack rolls instead of your Strength modifier."),
                pretext: None,
                source_page: Some("p.104"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Blind-Fight", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Intuitive Shot -- up_feats.lst:181
            UpsiFeatEntry {
                key: "Intuitive Shot",
                category: FeatCategory::Psionic,
                name: "Intuitive Shot",
                description: Some("When focused, use a standard action to gain +%1 damage on a ranged weapon attack within 30 ft.|WIS"),
                pretext: None,
                source_page: Some("p.104"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Point-Blank Shot,Psionic Shot", "PRESTAT:1,WIS=13"]),
            },
            // Killer's Vitality -- up_feats.lst:186
            UpsiFeatEntry {
                key: "Killer's Vitality",
                category: FeatCategory::General,
                name: "Killer's Vitality",
                description: Some("Any time you make a successful sneak attack against a living creature, you can expend your psionic focus to gain 3 temporary hit points per die of sneak attack dealt. You can divide these temporary hit points between yourself and members of your collective in any ratio. Special:If your attack reduces the target to fewer than 0 hit points, you gain double the temporary hit points."),
                pretext: None,
                source_page: Some("p.104"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Soulthief Vitalist Method", "PREVARGTEQ:SneakAttackDice,1"]),
            },
            // Knightmare -- up_feats.lst:188
            UpsiFeatEntry {
                key: "Knightmare",
                category: FeatCategory::General,
                name: "Knightmare",
                description: Some("Your manifester level which granted you the terrors class feature gains a +2 bonus as long as this bonus doesn't raise your manifester level above your current Hit Dice. [ML bonus not implemented.] In addition, for the purpose of terrors level requirements and order benefits, add your dread and cavalier levels together. In addition, when declaring your challenge, you can make a free Intimidate check against the target."),
                pretext: None,
                source_page: Some("p.104"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=Special Ability,Dread ~ Terror,Cavalier's Charge ~ Cavalier"]),
            },
            // Knockdown Shot -- up_feats.lst:189
            UpsiFeatEntry {
                key: "Knockdown Shot",
                category: FeatCategory::Psionic,
                name: "Knockdown Shot",
                description: Some("Expend focus to damage and knock prone opponent with ranged attack."),
                pretext: None,
                source_page: Some("p.104"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Point-Blank Shot,Psionic Shot", "PRESTAT:1,DEX=15"]),
            },
            // Levitative Transport -- up_feats.lst:194
            UpsiFeatEntry {
                key: "Levitative Transport",
                category: FeatCategory::Psionic,
                name: "Levitative Transport",
                description: Some("Share elocater movement abilities."),
                pretext: None,
                source_page: Some("p.105"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Gravitic Stability", "PRECLASS:1,Elocater=5", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Lingering Power -- up_feats.lst:195
            UpsiFeatEntry {
                key: "Lingering Power",
                category: FeatCategory::Metapsionic,
                name: "Lingering Power",
                description: Some("Instantaneous power persists for a round."),
                pretext: None,
                source_page: Some("p.105"),
                benefit: None,
                prerequisites: None,
            },
            // Malleable Power -- up_feats.lst:200
            UpsiFeatEntry {
                key: "Malleable Power",
                category: FeatCategory::Metapsionic,
                name: "Malleable Power",
                description: Some("You can exclude areas from power effect."),
                pretext: None,
                source_page: Some("p.105"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Selective Power", "PRESKILL:1,Spellcraft=10"]),
            },
            // Master of All Forms -- up_feats.lst:201
            UpsiFeatEntry {
                key: "Master of All Forms",
                category: FeatCategory::Psionic,
                name: "Master of All Forms",
                description: Some("Manifest shapechanging powers as a swift action"),
                pretext: None,
                source_page: Some("p.105"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Swift Shapeshifter", "PRESPELL:1,Metamorphosis", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Master's Refuge -- up_feats.lst:202
            UpsiFeatEntry {
                key: "Master's Refuge",
                category: FeatCategory::Psionic,
                name: "Master's Refuge",
                description: Some("Transfer mind to thrall on death"),
                pretext: None,
                source_page: Some("p.106"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Twofold Master", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Master's Voice -- up_feats.lst:203
            UpsiFeatEntry {
                key: "Master's Voice",
                category: FeatCategory::Psionic,
                name: "Master's Voice",
                description: Some("Have permanent mental contact with thralls"),
                pretext: None,
                source_page: Some("p.106"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Thrallherd", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Maximize Power -- up_feats.lst:204
            UpsiFeatEntry {
                key: "Maximize Power",
                category: FeatCategory::Metapsionic,
                name: "Maximize Power",
                description: Some("Maximize power's variable, numeric effects"),
                pretext: None,
                source_page: Some("p.106"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Mental Leap -- up_feats.lst:205
            UpsiFeatEntry {
                key: "Mental Leap",
                category: FeatCategory::Psionic,
                name: "Mental Leap",
                description: Some("+10 on Acrobatics checks to jump"),
                pretext: None,
                source_page: Some("p.106"),
                benefit: None,
                prerequisites: Some(&["PRESKILL:1,Acrobatics=2", "PRESTAT:1,STR=13,DEX=13", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Merciful Power -- up_feats.lst:206
            UpsiFeatEntry {
                key: "Merciful Power",
                category: FeatCategory::Metapsionic,
                name: "Merciful Power",
                description: Some("Power does nonlethal damage."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: None,
                prerequisites: None,
            },
            // Merge Designs -- up_feats.lst:207
            UpsiFeatEntry {
                key: "Merge Designs",
                category: FeatCategory::Psionic,
                name: "Merge Designs",
                description: Some("Your attoos cannot be interpreted to determine your powers known. In addition, you can safely wear one additional psionic tattoo on your body.  Normal: Tattoos for the pattern designs class feature can be identified by a Spellcraft check.  A character can only safely wear twenty psionic tattoos."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Cryptic ~ Pattern Designs", "PREABILITY:1,CATEGORY=FEAT,Scribe Tattoo", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Metapsionic Mastery -- up_feats.lst:208
            UpsiFeatEntry {
                key: "Metapsionic Mastery",
                category: FeatCategory::Psionic,
                name: "Metapsionic Mastery",
                description: Some("You can apply metapsionic feats to chosen power by increasing cost but not expending focus."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,TYPE=Metapsionic", "!PREABILITY:1,CATEGORY=FEAT,Metapsionic Mastery", "PREVARGTEQ:MaxManifesterLVL,3", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Mind Blade Knight -- up_feats.lst:210
            UpsiFeatEntry {
                key: "Mind Blade Knight",
                category: FeatCategory::Psionic,
                name: "Mind Blade Knight",
                description: Some("Add weapon special abilities to summoned weapons."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,First ~ Mind Knight Path,Mind Knight Path ~ Second", "PREVARGTEQ:IsPsionic,1", "PREVARGTEQ:WarriorPathLVL,3"]),
            },
            // Mind Knight's Arsenal -- up_feats.lst:211
            UpsiFeatEntry {
                key: "Mind Knight's Arsenal",
                category: FeatCategory::Psionic,
                name: "Mind Knight's Arsenal",
                description: Some("When you summon a weapon with the call weaponry path power with which you have the Weapon Focus feat, the weapon's enhancement bonus is increased by 1 and it gains a single predetermined weapon special ability with a +1 base price modifier. The weapon special ability must be appropriate for the weapon you choose and must be selected from those weapon special abilities available to the soulknife. You can meditate for 8 hours to change which predetermined special ability it gets."),
                pretext: None,
                source_page: Some("p.107"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Mind Knight Path ~ First,Mind Knight Path ~ Second", "PREABILITY:1,CATEGORY=Special Ability,Call Weaponry Path Power", "PREABILITY:1,CATEGORY=FEAT,Weapon Focus", "PREMULT:1,[PRETOTALAB:3],[PREVARGTEQ:PrereqBAB,3]", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Mind Over Body -- up_feats.lst:212
            UpsiFeatEntry {
                key: "Mind Over Body",
                category: FeatCategory::General,
                name: "Mind Over Body",
                description: Some("Heal ability damage more quickly"),
                pretext: None,
                source_page: Some("p.107"),
                benefit: None,
                prerequisites: Some(&["PRESTAT:1,CON=13"]),
            },
            // Mixed Combat -- up_feats.lst:213
            UpsiFeatEntry {
                key: "Mixed Combat",
                category: FeatCategory::Combat,
                name: "Mixed Combat",
                description: Some("You are trained in using ranged and melee attacks at the same time."),
                pretext: None,
                source_page: Some("p.107"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Quick Draw", "PRETOTALAB:6"]),
            },
            // Modified Blast -- up_feats.lst:214
            UpsiFeatEntry {
                key: "Modified Blast",
                category: FeatCategory::Psionic,
                name: "Modified Blast",
                description: Some("When you activate your surge blast, you gain the following augment options.  Like augmenting a power, augmenting a surge blast is limited to your manifester level.  You can also choose to invoke a wild surge when using your surge blast, using the power from the wild surge to pay for augments to the surge blast, but suffer the standard risk of psychic enervation for invoking a wild surge.  Augment: You can augment your surge blast in one of the following ways.  1. If you spend 2 additional power points, your surge blast deals splash damage.  2. If you spend 3 additional power points, your surge blast is instead a 5 ft. emanation centered on you and cratures in the affected area gain a Reflex save (DC %1) to take half damage.  3. If you spend 4 additional power points, your surge blast deals half damage, but damages all creatures in a 30 ft. line.  4. If you spend 4 additional power points, your surge blast is empowered, dealing 50%% additional damage.  5. If you spend 4 additional power points, your surge blast is a 15 ft. cone effect instead of a ranged touch attack, and creatures in the affected area gain a Reflex save (DC %1) to take half damage.  6.  If you spend 4 additional power points, your surge blast affects all creatures within a 10 ft. radius of the targeted creature and creatures in the affected area gain a Reflex save (DC %1) to take half damage.  7. If you spend 6 additional power points, your surge blast is instead a 10 ft. emanation centered on you and creatures in the affected area gain a Reflex save (DC %1) to take half damage.|ModifiedBlastDC"),
                pretext: None,
                source_page: Some("p.107"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Surge Blast", "PREVARGTEQ:MaxManifesterLVL,3", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Multiple Connections -- up_feats.lst:215
            UpsiFeatEntry {
                key: "Multiple Connections",
                category: FeatCategory::Psionic,
                name: "Multiple Connections",
                description: Some("You can be attuned to up to three creatures for the purposes of Open Door."),
                pretext: None,
                source_page: Some("p.107"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Terror ~ Mindlock", "PREABILITY:1,CATEGORY=FEAT,Open Door", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Nightmare Veil -- up_feats.lst:222
            UpsiFeatEntry {
                key: "Nightmare Veil",
                category: FeatCategory::Psionic,
                name: "Nightmare Veil",
                description: Some("While in broken dreams style, any target that is shaken treats you as concealed.  In addition, while in the broken dreams style, you treat shaken targets as flat-footed for unarmed attacks."),
                pretext: None,
                source_page: Some("p.107"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Dread ~ Devastating Touch", "PREABILITY:3,CATEGORY=FEAT,Improved Unarmed Strike,Broken Dreams Style,Shattered Dream Strike", "PRESKILL:1,Intimidate=9", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Nomad's Jump -- up_feats.lst:223
            UpsiFeatEntry {
                key: "Nomad's Jump",
                category: FeatCategory::Psionic,
                name: "Nomad's Jump",
                description: Some("Your levels of elocater stack with your levels of nomad for the purposes of determining the range of nomad's step. In addition, when you use nomad's step, you may split your distance traveled into two steps and determine your line of sight at the end of the first step. The total distance traveled must not exceed your maximum range for nomad's step and you may perform no other actions during the steps."),
                pretext: None,
                source_page: Some("p.107"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=Special Ability,Nomad's Step ~ Psychoportation,Elocater ~ Aerial Acrobatics", "PREVARGTEQ:IsPsionic,1"]),
            },
            // One Pattern -- up_feats.lst:228
            UpsiFeatEntry {
                key: "One Pattern",
                category: FeatCategory::Psionic,
                name: "One Pattern",
                description: Some("Your target no longer needs to match your active creature type to deal full damage with your disrupt pattern ability. If your target does match your active creature type, you inflict an extra 1 point of damage per dice of damage dealt with disrupt pattern. This extra damage is not multiplied on a critical hit."),
                pretext: None,
                source_page: Some("p.108"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Cryptic ~ Disrupt Pattern", "PRESKILL:6,Knowledge (Arcana)=5,Knowledge (Dungeoneering)=5,Knowledge (Nature)=5,Knowlede (Planes)=5,Knowledge (Psionics)=5,Knowledge (Religion)=5", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Open Door -- up_feats.lst:229
            UpsiFeatEntry {
                key: "Open Door",
                category: FeatCategory::Psionic,
                name: "Open Door",
                description: Some("Once an individual has failed a Will save to avoid being mindlocked by you, you can attune yourself to that creature.  You can reactivate the mindlock to an attuned creature at any time by spending 1 power point as a free action, regardless of the time since you first mindlocked the target.  This means they count as having failed their Will save.  Maximum number of attunements:%1|OpenDoorAttunements"),
                pretext: None,
                source_page: Some("p.108"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Terror ~ Mindlock", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Open Minded -- up_feats.lst:230
            UpsiFeatEntry {
                key: "Open Minded",
                category: FeatCategory::General,
                name: "Open Minded",
                description: Some("Gain skill points"),
                pretext: None,
                source_page: Some("p.108"),
                benefit: None,
                prerequisites: None,
            },
            // Opportunity Power -- up_feats.lst:231
            UpsiFeatEntry {
                key: "Opportunity Power",
                category: FeatCategory::Metapsionic,
                name: "Opportunity Power",
                description: Some("Make attacks of opportunity with touch powers"),
                pretext: None,
                source_page: Some("p.108"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Overchannel -- up_feats.lst:232
            UpsiFeatEntry {
                key: "Overchannel",
                category: FeatCategory::Psionic,
                name: "Overchannel",
                description: Some("Take damage to increase your manifester level"),
                pretext: None,
                source_page: Some("p.108"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Penetrating Fear -- up_feats.lst:237
            UpsiFeatEntry {
                key: "Penetrating Fear",
                category: FeatCategory::Psionic,
                name: "Penetrating Fear",
                description: Some("You can expend your psionic focus when activating a terror to allow it to affect those normally immune to fear effects or mind-affecting effects.  The target gets a +4 bonus to any applicable save against the effect."),
                pretext: None,
                source_page: Some("p.108"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Dread ~ Terror", "PRECLASS:1,Dread=10", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Persistent Focus -- up_feats.lst:238
            UpsiFeatEntry {
                key: "Persistent Focus",
                category: FeatCategory::Psionic,
                name: "Persistent Focus",
                description: Some("Choose one psionic ability or psionic feat that requires maintaining psionic focus. From this point forward, you are always considered psionically focused even if you expend your psionic focus for any other ability."),
                pretext: None,
                source_page: Some("p.108"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Psionic Meditation", "PRESKILL:1,Autohypnosis=4", "PRESTAT:1,WIS=13", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Persistent Power -- up_feats.lst:239
            UpsiFeatEntry {
                key: "Persistent Power",
                category: FeatCategory::Metapsionic,
                name: "Persistent Power",
                description: Some("Target of power must make two saves."),
                pretext: None,
                source_page: Some("p.108"),
                benefit: None,
                prerequisites: None,
            },
            // Piercing Power -- up_feats.lst:240
            UpsiFeatEntry {
                key: "Piercing Power",
                category: FeatCategory::Metapsionic,
                name: "Piercing Power",
                description: Some("Power affects target as if target had 5 smaller SR."),
                pretext: None,
                source_page: Some("p.108"),
                benefit: None,
                prerequisites: None,
            },
            // Piranha Strike -- up_feats.lst:241
            UpsiFeatEntry {
                key: "Piranha Strike",
                category: FeatCategory::Combat,
                name: "Piranha Strike",
                description: Some("You make a combination of quick strikes, sacrificing accuracy for multiple, minor wounds that prove exceptionally deadly."),
                pretext: None,
                source_page: Some("p.109"),
                benefit: Some("When wielding a light weapon, you can choose to take a -%1 penalty on all melee attack rolls and combat maneuver checks to gain a +%2 bonus on all melee damage rolls.  This bonus damage is halved (-50%%) if you are making an attack with an off-hand weapon or secondary natural weapon.  You must choose to use this feat before the attack roll, and its effects last until your next turn.  The bonus damage does not apply to touch attacks or effects that do not deal hit point damage.  This feat cannot be used in conjunction with the Power Attack feat.|PiranhaStrikeAttackPenalty|PiranhaStrikeBonusDamage"),
                prerequisites: Some(&["PREVAREQ:HasWeaponFinesseFeat,1", "PRETOTALAB:1"]),
            },
            // Power Channeler -- up_feats.lst:242
            UpsiFeatEntry {
                key: "Power Channeler",
                category: FeatCategory::Psionic,
                name: "Power Channeler",
                description: Some("You can channel a touch range power through a melee weapon.  You can make a single melee attack during the same action in which you manifest a touch power with a manifesting time of one standard action; if this melee attack hits, it triggers the power as if you had succeeded on a touch attack and deals damage as normal.  If you miss, the power fizzles and the charge is lost.  This attack is made against standard AC and not touch AC."),
                pretext: None,
                source_page: Some("p.109"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Power Penetration -- up_feats.lst:243
            UpsiFeatEntry {
                key: "Power Penetration",
                category: FeatCategory::Psionic,
                name: "Power Penetration",
                description: Some("Gain bonus to overcome power resistance"),
                pretext: None,
                source_page: Some("p.109"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Power Perfection -- up_feats.lst:245
            UpsiFeatEntry {
                key: "Power Perfection",
                category: FeatCategory::Psionic,
                name: "Power Perfection",
                description: Some("Pick one power which you have the ability to manifest. Whenever you manifest that power you may apply any one metapsionic feat you have to that power without expending psionic focus or affecting its level or manifesting time, as long as the total power point cost of the power, including the metapsionic feat cost and any augmentation, does not exceed 17 power points. In addition, if you have other feats which allow you to apply a set numerical bonus to any aspect of this power (such as Power Penetration, Psionic Endowment, Weapon Focus [ray], and so on), double the bonus granted by that feat when applied to this power."),
                pretext: None,
                source_page: Some("p.109"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,TYPE.Metapsionic", "PRESKILL:1,Spellcraft=15", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Power Specialization -- up_feats.lst:246
            UpsiFeatEntry {
                key: "Power Specialization",
                category: FeatCategory::Psionic,
                name: "Power Specialization",
                description: Some("Gain bonus damage to damaging powers"),
                pretext: None,
                source_page: Some("p.109"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Weapon Focus (Ray)", "PREVARGTEQ:MaxManifesterLVL,4", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Psicrystal Affinity -- up_feats.lst:247
            UpsiFeatEntry {
                key: "Psicrystal Affinity",
                category: FeatCategory::Psionic,
                name: "Psicrystal Affinity",
                description: Some("Obtain a psicrystal"),
                pretext: None,
                source_page: Some("p.109"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:MaxManifesterLVL,1", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Psicrystal Containment -- up_feats.lst:248
            UpsiFeatEntry {
                key: "Psicrystal Containment",
                category: FeatCategory::Psionic,
                name: "Psicrystal Containment",
                description: Some("Your psicrystal can hold a psionic focus"),
                pretext: None,
                source_page: Some("p.111"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Psicrystal Affinity", "PREVARGTEQ:MaxManifesterLVL,3", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Psionic Body -- up_feats.lst:249
            UpsiFeatEntry {
                key: "Psionic Body",
                category: FeatCategory::Psionic,
                name: "Psionic Body",
                description: Some("+2 hit points for each psionic feat you have"),
                pretext: None,
                source_page: Some("p.111"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Psionic Bull Rush -- up_feats.lst:250
            UpsiFeatEntry {
                key: "Psionic Bull Rush",
                category: FeatCategory::Psionic,
                name: "Psionic Bull Rush",
                description: Some("While maintaining psionic focus, when you successfully perform a bull rush maneuver, you also deal %1 damage to the creature bull rushed.  If you expend your psoinic focus when you successfully make a bull rush maneuver, you can also knock the target of your bull rush prone.|PsionicBullRushDamage"),
                pretext: None,
                source_page: Some("p.111"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Improved Bull Rush", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Psionic Charge -- up_feats.lst:251
            UpsiFeatEntry {
                key: "Psionic Charge",
                category: FeatCategory::Psionic,
                name: "Psionic Charge",
                description: Some("Charge while taking erratic course to foe"),
                pretext: None,
                source_page: Some("p.111"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Speed of Thought", "PRESTAT:1,DEX=13", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Psionic Critical -- up_feats.lst:252
            UpsiFeatEntry {
                key: "Psionic Critical",
                category: FeatCategory::Psionic,
                name: "Psionic Critical",
                description: Some("While maintaining psionic focus, when you successfully score a critical hit, you deal +1d8 bonus damage.  If you expend your psionic focus, you deal an additional +1d8 bonus damage."),
                pretext: None,
                source_page: Some("p.112"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Improved Critical", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Psionic Disarm -- up_feats.lst:253
            UpsiFeatEntry {
                key: "Psionic Disarm",
                category: FeatCategory::Psionic,
                name: "Psionic Disarm",
                description: Some("While maintaining psionic focus, when you successfully perform a disarm combat maneuver, you also deal %1 damage to the creature disarmed.  If you expend your psionic focus when you make a successful disarm combat maneuver, you can also send the disarmed object into any space of your choosing within %2 ft.  If a creature is in the selected space and has a hand free, it can grab the disarmed object as an immediate action.|PsionicDisarmDamage|PsionicDisarmDistance"),
                pretext: None,
                source_page: Some("p.112"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Improved Disarm", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Psionic Dodge -- up_feats.lst:254
            UpsiFeatEntry {
                key: "Psionic Dodge",
                category: FeatCategory::Psionic,
                name: "Psionic Dodge",
                description: Some("+1 dodge bonus to AC"),
                pretext: None,
                source_page: Some("p.112"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Dodge", "PRESTAT:1,DEX=13", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Psionic Endowment -- up_feats.lst:255
            UpsiFeatEntry {
                key: "Psionic Endowment",
                category: FeatCategory::Psionic,
                name: "Psionic Endowment",
                description: Some("Add +1 to power's save DC"),
                pretext: None,
                source_page: Some("p.112"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Psionic Fist -- up_feats.lst:257
            UpsiFeatEntry {
                key: "Psionic Fist",
                category: FeatCategory::Psionic,
                name: "Psionic Fist",
                description: Some("Unarmed attack or natural weapon deals extra damage"),
                pretext: None,
                source_page: Some("p.112"),
                benefit: None,
                prerequisites: Some(&["PRESTAT:1,STR=13", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Psionic Meditation -- up_feats.lst:258
            UpsiFeatEntry {
                key: "Psionic Meditation",
                category: FeatCategory::Psionic,
                name: "Psionic Meditation",
                description: Some("Become psionically focused as a move action"),
                pretext: None,
                source_page: Some("p.112"),
                benefit: None,
                prerequisites: Some(&["PRESKILL:1,Autohypnosis=4", "PRESTAT:1,WIS=13", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Psionic Overrun -- up_feats.lst:259
            UpsiFeatEntry {
                key: "Psionic Overrun",
                category: FeatCategory::Psionic,
                name: "Psionic Overrun",
                description: Some("While maintaining psionic focus, when you successfully perform an overrun combat maneuver, you also deal %1 damage.  If you expend your psionic focus when you perform a successful overrun combat maneuver, you can also push your target up to %2 feet.|PsionicOverrunDamage|PsionicOverrunDistance"),
                pretext: None,
                source_page: Some("p.112"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Power Attack,Improved Overrun", "PREMULT:1,[PRETOTALAB:1],[PREVARGTEQ:PrereqBAB,1]", "PRESTAT:1,STR=13", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Psionic Precise Shot -- up_feats.lst:260
            UpsiFeatEntry {
                key: "Psionic Precise Shot",
                category: FeatCategory::Psionic,
                name: "Psionic Precise Shot",
                description: Some("To use this feat, you must expend your psionic focus.  You can attempt to make a single ranged attack as a standard action against a target that is sheltered behind a wall, wall of force effect, or similar barrier.  Your attack briefly skips through the Astral Plane to bypass the barrier.  You must still have line of sight to the target."),
                pretext: None,
                source_page: Some("p.112"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Improved Precise Shot,Point-Blank Shot,Precise Shot", "PREMULT:1,[PRETOTALAB:11],[PREVARGTEQ:PrereqBAB,11]", "PRESTAT:1,DEX=19", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Psionic Shield Bash -- up_feats.lst:261
            UpsiFeatEntry {
                key: "Psionic Shield Bash",
                category: FeatCategory::Psionic,
                name: "Psionic Shield Bash",
                description: Some("While maintaining psionic focus, when you successfully strike a foe with a shield bash attack, the foe is shaken for 1 round (Will DC %1 negates).  This is a mind-affecting fear effect.  If you expend your psionic focus when you make a shield bash, on a successful attack roll you can instead force the target to make a Will saving throw or be stunned for 1 round.|PsionicShieldBashDC"),
                pretext: None,
                source_page: Some("p.112"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Shield Bash,Shield Proficiency", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Psionic Shot -- up_feats.lst:263
            UpsiFeatEntry {
                key: "Psionic Shot",
                category: FeatCategory::Psionic,
                name: "Psionic Shot",
                description: Some("Ranged weapon deals extra damage"),
                pretext: None,
                source_page: Some("p.112"),
                benefit: Some("While you maintain psionic focus, your attacks with a ranged weapon deal an extra 1 point of damage. Additionally, if you expend your psionic focus as part of an attack with a ranged weapon, that attack instead deals an extra 2d6 points of damage. You must decide whether or not to use this feat prior to making an attack. If your attack misses, you still expend your psionic focus."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Psionic Stamina -- up_feats.lst:264
            UpsiFeatEntry {
                key: "Psionic Stamina",
                category: FeatCategory::Psionic,
                name: "Psionic Stamina",
                description: Some("As long as you maintain psionic focus, you gain a +1 bonus to your Fortitude saves for each psionic feat you have (including this one), to a maximum bonus of 1/3 of your soulknife class level (rounded down, minimum +1). You can expend your psionic focus as an immediate action to also add your Wisdom bonus to your Fortitude saves instead of your Constitution bonus until the beginning of your next turn."),
                pretext: None,
                source_page: Some("p.113"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Psionic Body", "PREVARGTEQ:IsPsionic,1", "PREVARGTEQ:SoulknifeFeatPrereqLVL,3"]),
            },
            // Psionic Sunder -- up_feats.lst:265
            UpsiFeatEntry {
                key: "Psionic Sunder",
                category: FeatCategory::Psionic,
                name: "Psionic Sunder",
                description: Some("While maintaining psionic focus, when you successfully perform a sunder combat maneuver, you also deal %1 damage to the creature holding the item sundered.  If you expend your psionic focus when you perform a successful sunder maneuver, you ignore half the hardness of the object you are sundering.|PsionicSunderDamage"),
                pretext: None,
                source_page: Some("p.113"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Power Attack,Improved Sunder", "PREMULT:1,[PRETOTALAB:1],[PREVARGTEQ:PrereqBAB,1]", "PRESTAT:1,STR=13", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Psionic Talent -- up_feats.lst:266
            UpsiFeatEntry {
                key: "Psionic Talent",
                category: FeatCategory::Psionic,
                name: "Psionic Talent",
                description: Some("Gain additional power points"),
                pretext: None,
                source_page: Some("p.113"),
                benefit: Some("You gain %1 power points.|COUNT[FEATNAME=Psionic Talent]+1"),
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Psionic Trip -- up_feats.lst:267
            UpsiFeatEntry {
                key: "Psionic Trip",
                category: FeatCategory::Psionic,
                name: "Psionic Trip",
                description: Some("While maintaining psionic focus, when you successfully perform a trip combat maneuver, you also deal %1 damage to the creature tripped.  If you expend your psionic focus when you perform a successful trip combat maneuver, you can also throw your target up to %2 ft., although the creature thrown may not be more than one size category larger than you.|PsionicTripDamage|PsionicTripDistance"),
                pretext: None,
                source_page: Some("p.113"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Trip", "PRESTAT:1,INT=13", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Psionic Unarmed Strike -- up_feats.lst:268
            UpsiFeatEntry {
                key: "Psionic Unarmed Strike",
                category: FeatCategory::Psionic,
                name: "Psionic Unarmed Strike",
                description: Some("To use this feat, you must expend psionic focus.  You can attempt to make a single unarmed melee attack as a standard action against a target that is sheltered behind a wall, wall of force effect, or similar barrier.  Your attack briefly skips through the Astral Plane to bypass the barrier.  This does not grant you line of sight or extra reach, requiring you to make the attack without being able to see the target or use some other method of seeing the target (such as clairaudience/clairvoyance) and be within your normal reach for your unarmed attack."),
                pretext: None,
                source_page: Some("p.113"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Stunning Fist", "PRESTAT:2,DEX=13,WIS=13", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Psionic Weapon -- up_feats.lst:270
            UpsiFeatEntry {
                key: "Psionic Weapon",
                category: FeatCategory::Psionic,
                name: "Psionic Weapon",
                description: Some("Melee weapons deal extra damage"),
                pretext: None,
                source_page: Some("p.113"),
                benefit: None,
                prerequisites: Some(&["PRESTAT:1,STR=13", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Psychoportive Pathfinder -- up_feats.lst:271
            UpsiFeatEntry {
                key: "Psychoportive Pathfinder",
                category: FeatCategory::Psionic,
                name: "Psychoportive Pathfinder",
                description: Some("Any time you manifest psychoport, you may, as a free action, choose to leave a marker for someone else to follow. This marker has a duration of one round. If another person casts teleport or greater teleport or manifests psychoport or greater psychoport in the space you left from, they can choose as part of casting the spell or manifesting the power to appear adjacent to you in a safe, uninhabited space even if they would normally fail due to lack of familiarity or lack of range."),
                pretext: None,
                source_page: Some("p.113"),
                benefit: None,
                prerequisites: Some(&["PRECLASS:1,Nomad=1", "PRESPELL:1,Trace Teleport", "PRESPELL:1,Psychoport,Psychoport(Greater)", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Pyromaniac -- up_feats.lst:273
            UpsiFeatEntry {
                key: "Pyromaniac",
                category: FeatCategory::General,
                name: "Pyromaniac",
                description: Some("Add your alchemist and pyrokineticist levels together to determine the damage done by your bombs and the number of bombs you can create each day. Additionally, when you apply your weapon afire ability to your bombs, all creatures in the splash area also take the extra damage from your weapon afire ability."),
                pretext: None,
                source_page: Some("p.113"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=Special Ability,Pyrokinetic ~ Fire Lash,Bomb ~ Alchemist", "PREVARGTEQ:AlchemistBombAdditionalDice,1"]),
            },
            // Quick Suit -- up_feats.lst:278
            UpsiFeatEntry {
                key: "Quick Suit",
                category: FeatCategory::Psionic,
                name: "Quick Suit",
                description: Some("You can form your astral suit as a free action once per turn."),
                pretext: None,
                source_page: Some("p.114"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=Special Ability,Aegis ~ Form Astral Suit,Aegis ~ Reconfigure", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Quicken Power -- up_feats.lst:279
            UpsiFeatEntry {
                key: "Quicken Power",
                category: FeatCategory::Metapsionic,
                name: "Quicken Power",
                description: Some("Manifest powers as a swift action"),
                pretext: None,
                source_page: Some("p.114"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Raging Hulk -- up_feats.lst:284
            UpsiFeatEntry {
                key: "Raging Hulk",
                category: FeatCategory::General,
                name: "Raging Hulk",
                description: Some("For the purpose of level requirements for rage powers or astral suit customizations, add your levels of barbarian and aegis together. Damage reduction from either class stacks. In addition, any time you enter a rage, you can activate augment suit as a free action."),
                pretext: None,
                source_page: Some("p.114"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=Special Ability,Aegis ~ Form Astral Suit,Rage"]),
            },
            // Rapid Augmentation -- up_feats.lst:285
            UpsiFeatEntry {
                key: "Rapid Augmentation",
                category: FeatCategory::Psionic,
                name: "Rapid Augmentation",
                description: Some("You can activate your augment suit class feature as a swift action."),
                pretext: None,
                source_page: Some("p.114"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Aegis ~ Augment Suit", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Rapid Draw -- up_feats.lst:286
            UpsiFeatEntry {
                key: "Rapid Draw",
                category: FeatCategory::Psionic,
                name: "Rapid Draw",
                description: Some("You may form your mind blade as a free action an additional %1 times per round.|RapidDrawExtraRounds"),
                pretext: None,
                source_page: Some("p.114"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Blade Skill ~ Alter Blade", "PREABILITY:1,CATEGORY=Special Ability,Soulknife ~ Quick Draw", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Rapid Metabolism -- up_feats.lst:287
            UpsiFeatEntry {
                key: "Rapid Metabolism",
                category: FeatCategory::General,
                name: "Rapid Metabolism",
                description: Some("Heal hit points more quickly"),
                pretext: None,
                source_page: Some("p.114"),
                benefit: None,
                prerequisites: Some(&["PRESTAT:1,CON=13"]),
            },
            // Ready Response -- up_feats.lst:288
            UpsiFeatEntry {
                key: "Ready Response",
                category: FeatCategory::Psionic,
                name: "Ready Response",
                description: Some("At the beginning of a surprise round, if you would not normally take an action, you can expend your psionic focus to roll initiative and take a standard action or a move action.  If you do move normally in a surprise round, you can expend your psionic focus to take a full-round of action, rather than only a standard or move action."),
                pretext: None,
                source_page: Some("p.114"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Rebounding Throw -- up_feats.lst:289
            UpsiFeatEntry {
                key: "Rebounding Throw",
                category: FeatCategory::Psionic,
                name: "Rebounding Throw",
                description: Some("Expend focus when thrown weapon hits to attack another enemy."),
                pretext: None,
                source_page: Some("p.114"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot", "PREMULT:1,[PRETOTALAB:1],[PREVARGTEQ:PrereqBAB,1]"]),
            },
            // Reckless Offense -- up_feats.lst:290
            UpsiFeatEntry {
                key: "Reckless Offense",
                category: FeatCategory::General,
                name: "Reckless Offense",
                description: Some("Take -4 to AC to gain +2 melee attack bonus"),
                pretext: None,
                source_page: Some("p.115"),
                benefit: None,
                prerequisites: Some(&["PRETOTALAB:1"]),
            },
            // Redirect Power -- up_feats.lst:291
            UpsiFeatEntry {
                key: "Redirect Power",
                category: FeatCategory::Metapsionic,
                name: "Redirect Power",
                description: Some("You can redirect a failed power to a different target."),
                pretext: None,
                source_page: Some("p.115"),
                benefit: None,
                prerequisites: None,
            },
            // Resonance Mastery -- up_feats.lst:292
            UpsiFeatEntry {
                key: "Resonance Mastery",
                category: FeatCategory::Psionic,
                name: "Resonance Mastery",
                description: Some("Apply metapsionic feats to powers without paying increased cost but by increasing manifesting time"),
                pretext: None,
                source_page: Some("p.115"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,TYPE.Metapsionic", "PRECLASS:1,Psicrystal Imprinter=5", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Return Shot -- up_feats.lst:293
            UpsiFeatEntry {
                key: "Return Shot",
                category: FeatCategory::Psionic,
                name: "Return Shot",
                description: Some("Deflect range attacks back at attacker"),
                pretext: None,
                source_page: Some("p.115"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Fell Shot,Point-Blank Shot,Psionic Shot", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Returning Throw -- up_feats.lst:294
            UpsiFeatEntry {
                key: "Returning Throw",
                category: FeatCategory::Psionic,
                name: "Returning Throw",
                description: Some("Thrown weapons return to your hand."),
                pretext: None,
                source_page: Some("p.115"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot", "PREMULT:1,[PRETOTALAB:1],[PREVARGTEQ:PrereqBAB,1]"]),
            },
            // Ricochet -- up_feats.lst:295
            UpsiFeatEntry {
                key: "Ricochet",
                category: FeatCategory::Psionic,
                name: "Ricochet",
                description: Some("Expend focus to redirect thrown attacks; %1 redirections per attack.|1+WIS"),
                pretext: None,
                source_page: Some("p.115"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]"]),
            },
            // Rime Power -- up_feats.lst:296
            UpsiFeatEntry {
                key: "Rime Power",
                category: FeatCategory::Metapsionic,
                name: "Rime Power",
                description: Some("Entangle creatures with cold power."),
                pretext: None,
                source_page: Some("p.115"),
                benefit: None,
                prerequisites: None,
            },
            // Scholarly Discipline -- up_feats.lst:302
            UpsiFeatEntry {
                key: "Scholarly Discipline",
                category: FeatCategory::General,
                name: "Scholarly Discipline",
                description: Some("Your psion manifester level and wizard caster level gain a +2 bonus as long as this bonus doesn't raise your manifester level or caster level above your current Hit Dice. [NOT IMPLEMENTED]"),
                pretext: None,
                source_page: Some("p.116"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.PsionicDiscipline", "PREABILITY:1,CATEGORY=Special Ability,TYPE.ArcaneSchool"]),
            },
            // Scribe Tattoo -- up_feats.lst:303
            UpsiFeatEntry {
                key: "Scribe Tattoo",
                category: FeatCategory::ItemCreation,
                name: "Scribe Tattoo",
                description: Some("Create psionic tattoos"),
                pretext: None,
                source_page: Some("p.116"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:MaxManifesterLVL,3"]),
            },
            // Selective Power -- up_feats.lst:306
            UpsiFeatEntry {
                key: "Selective Power",
                category: FeatCategory::Metapsionic,
                name: "Selective Power",
                description: Some("Choose %1 targets to exclude from power.|SelectivePowerTargets"),
                pretext: None,
                source_page: Some("p.116"),
                benefit: None,
                prerequisites: Some(&["PRESKILL:1,Spellcraft=10"]),
            },
            // Shared Power -- up_feats.lst:307
            UpsiFeatEntry {
                key: "Shared Power",
                category: FeatCategory::Metapsionic,
                name: "Shared Power",
                description: Some("Manifest personal power as a 30-ft. ray, or with the Network descriptor."),
                pretext: None,
                source_page: Some("p.116"),
                benefit: None,
                prerequisites: None,
            },
            // Shattered Dream Strike -- up_feats.lst:308
            UpsiFeatEntry {
                key: "Shattered Dream Strike",
                category: FeatCategory::Psionic,
                name: "Shattered Dream Strike",
                description: Some("While in the broken dreams style, after making a successful unarmed strike empowered with your devastating touch, you can make a free Intimidate check against the target."),
                pretext: None,
                source_page: Some("p.116"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Dread ~ Devastating Touch", "PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Broken Dreams Style", "PRESKILL:1,Intimidate=6", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Sickening Power -- up_feats.lst:309
            UpsiFeatEntry {
                key: "Sickening Power",
                category: FeatCategory::Metapsionic,
                name: "Sickening Power",
                description: Some("Sicken creatures with power."),
                pretext: None,
                source_page: Some("p.116"),
                benefit: None,
                prerequisites: None,
            },
            // Sidestep Charge -- up_feats.lst:310
            UpsiFeatEntry {
                key: "Sidestep Charge",
                category: FeatCategory::General,
                name: "Sidestep Charge",
                description: Some("+4 bonus to AC against a charging foe"),
                pretext: None,
                source_page: Some("p.116"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Dodge", "PRESTAT:1,DEX=13"]),
            },
            // Soul Warrior -- up_feats.lst:313
            UpsiFeatEntry {
                key: "Soul Warrior",
                category: FeatCategory::General,
                name: "Soul Warrior",
                description: Some("Your manifester level which granted you the warrior's path class feature gains a +2 bonus as long as this bonus doesn't raise your manifester level above your current Hit Dice. Your soulknife level gains a +2 bonus, up to a maximum of your hit dice, for the purpose of your mind blade enhancement class feature. In addition, any time you perform a path maneuver while wielding your mind blade, you gain a +1 bonus to attack rolls with your mind blade for until the end of your next turn."),
                pretext: None,
                source_page: Some("p.116"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=Special Ability,Soulknife ~ Enhanced Mind Blade,Warrior's Path ~ Psychic Warrior", "PREVARGTEQ:MndBladeEnhancement,2"]),
            },
            // Speed of Thought -- up_feats.lst:314
            UpsiFeatEntry {
                key: "Speed of Thought",
                category: FeatCategory::Psionic,
                name: "Speed of Thought",
                description: Some("+10 feet to speed in light or medium armor"),
                pretext: None,
                source_page: Some("p.117"),
                benefit: None,
                prerequisites: Some(&["PRESTAT:1,WIS=13", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Split Headed Lash -- up_feats.lst:315
            UpsiFeatEntry {
                key: "Split Headed Lash",
                category: FeatCategory::Psionic,
                name: "Split Headed Lash",
                description: Some("Expend focus to split one fire lash attack from a full attack into two attacks."),
                pretext: None,
                source_page: Some("p.117"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Pyrokineticist ~ Fire Lash", "PREABILITY:3,CATEGORY=FEAT,Point-Blank Shot,Precise Shot,Weapon Focus (Whip)", "PRESKILL:1,Knowledge (Psionics)=10", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Split Psionic Ray -- up_feats.lst:316
            UpsiFeatEntry {
                key: "Split Psionic Ray",
                category: FeatCategory::Metapsionic,
                name: "Split Psionic Ray",
                description: Some("Split one ray attack into two"),
                pretext: None,
                source_page: Some("p.117"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,TYPE=Metapsionic", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Staggering Shot -- up_feats.lst:317
            UpsiFeatEntry {
                key: "Staggering Shot",
                category: FeatCategory::Psionic,
                name: "Staggering Shot",
                description: Some("Stagger hampered enemy with a ranged attack."),
                pretext: None,
                source_page: Some("p.117"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Intimidating Shot,Point-Blank Shot", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]"]),
            },
            // Student of the Astral Suit -- up_feats.lst:319
            UpsiFeatEntry {
                key: "Student of the Astral Suit",
                category: FeatCategory::General,
                name: "Student of the Astral Suit",
                description: Some("Treat your class level as four higher to determine your number of customization points and which customizations you can select for your astral suit, to a maximum of your character level."),
                pretext: None,
                source_page: Some("p.117"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Aegis ~ Invigorating Suit"]),
            },
            // Surging Aura -- up_feats.lst:320
            UpsiFeatEntry {
                key: "Surging Aura",
                category: FeatCategory::Psionic,
                name: "Surging Aura",
                description: Some("Wild surge grants bonus to allies"),
                pretext: None,
                source_page: Some("p.117"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1", "PREVARGTEQ:WildSurge,1"]),
            },
            // Swift Shapeshifter -- up_feats.lst:321
            UpsiFeatEntry {
                key: "Swift Shapeshifter",
                category: FeatCategory::Psionic,
                name: "Swift Shapeshifter",
                description: Some("Manifest shapechanging powers more quickly"),
                pretext: None,
                source_page: Some("p.117"),
                benefit: None,
                prerequisites: Some(&["PRESPELL:1,Metamorphosis", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Talented -- up_feats.lst:326
            UpsiFeatEntry {
                key: "Talented",
                category: FeatCategory::Psionic,
                name: "Talented",
                description: Some("Take no damage from overchanneling some powers"),
                pretext: None,
                source_page: Some("p.117"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Overchannel", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Telepathic Link -- up_feats.lst:327
            UpsiFeatEntry {
                key: "Telepathic Link",
                category: FeatCategory::Psionic,
                name: "Telepathic Link",
                description: Some("Gain ability to link creatures together to communicate telepathically"),
                pretext: None,
                source_page: Some("p.117"),
                benefit: None,
                prerequisites: Some(&["PRESPELL:1,Mindlink", "PREVARGTEQ:MaxManifesterLVL,3"]),
            },
            // Terror Mastery -- up_feats.lst:328
            UpsiFeatEntry {
                key: "Terror Mastery",
                category: FeatCategory::Psionic,
                name: "Terror Mastery",
                description: Some("When you use your terrors class feature, you can activate two terrors in a single swift action."),
                pretext: None,
                source_page: Some("p.118"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:8,CATEGORY=Special Ability,TYPE.DreadTerror", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Thundering Power -- up_feats.lst:329
            UpsiFeatEntry {
                key: "Thundering Power",
                category: FeatCategory::Metapsionic,
                name: "Thundering Power",
                description: Some("Deafen creatures with damaging power."),
                pretext: None,
                source_page: Some("p.118"),
                benefit: None,
                prerequisites: None,
            },
            // Tomb Raider -- up_feats.lst:331
            UpsiFeatEntry {
                key: "Tomb Raider",
                category: FeatCategory::General,
                name: "Tomb Raider",
                description: Some("Add your levels of cryptic and ranger together for the purposes of your trapfinding and favored terrain class features. In addition, you can add your favored terrain (underground) bonus to Disable Device checks when in an underground terrain."),
                pretext: None,
                source_page: Some("p.118"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=Special Ability,Cryptic ~ Trapmaker,Favored Terrain (Underground)"]),
            },
            // Toppling Power -- up_feats.lst:332
            UpsiFeatEntry {
                key: "Toppling Power",
                category: FeatCategory::Metapsionic,
                name: "Toppling Power",
                description: Some("Knock creatures prone with force power."),
                pretext: None,
                source_page: Some("p.118"),
                benefit: None,
                prerequisites: None,
            },
            // Touch of Terror -- up_feats.lst:333
            UpsiFeatEntry {
                key: "Touch of Terror",
                category: FeatCategory::Psionic,
                name: "Touch of Terror",
                description: Some("Your devastating touch ability deals 1d6 points of damage plus your class level and your Charisma modifier."),
                pretext: None,
                source_page: Some("p.118"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Dread ~ Devastating Touch", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Toughened Suit -- up_feats.lst:334
            UpsiFeatEntry {
                key: "Toughened Suit",
                category: FeatCategory::Psionic,
                name: "Toughened Suit",
                description: Some("Any time you form your astral suit, you can choose to pay 1 power point as part of the action to form the astral suit to gain %1 temporary hit points. These temporary hit points last until your astral suit is dismissed, destroyed, or otherwise ends.|ToughenedSuitTempHP"),
                pretext: None,
                source_page: Some("p.118"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Aegis ~ Form Astral Suit", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Twin Power -- up_feats.lst:335
            UpsiFeatEntry {
                key: "Twin Power",
                category: FeatCategory::Metapsionic,
                name: "Twin Power",
                description: Some("Manifest power twice"),
                pretext: None,
                source_page: Some("p.118"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Twin Throw -- up_feats.lst:336
            UpsiFeatEntry {
                key: "Twin Throw",
                category: FeatCategory::Combat,
                name: "Twin Throw",
                description: Some("First attack in full-attack with thrown weapons uses two weapons."),
                pretext: None,
                source_page: Some("p.118"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Point-Blank Shot,Two-Weapon Fighting", "PRESTAT:1,DEX=17", "PRETOTALAB:6"]),
            },
            // Unavoidable Strike -- up_feats.lst:341
            UpsiFeatEntry {
                key: "Unavoidable Strike",
                category: FeatCategory::Psionic,
                name: "Unavoidable Strike",
                description: Some("Resolve unarmed attack or natural weapon attack as a touch attack"),
                pretext: None,
                source_page: Some("p.118"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Psionic Fist", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:PrereqBAB,6]", "PRESTAT:1,STR=13", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Unconditional Power -- up_feats.lst:342
            UpsiFeatEntry {
                key: "Unconditional Power",
                category: FeatCategory::Metapsionic,
                name: "Unconditional Power",
                description: Some("Manifest power despite character condition"),
                pretext: None,
                source_page: Some("p.119"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Unlocked Talent -- up_feats.lst:345
            UpsiFeatEntry {
                key: "Unlocked Talent",
                category: FeatCategory::General,
                name: "Unlocked Talent",
                description: Some("You gain 2 power points and one 1st-level power."),
                pretext: None,
                source_page: Some("p.119"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Wild Talent", "!PREABILITY:1,CATEGORY=FEAT,Unlocked Talent"]),
            },
            // Unwilling Participant -- up_feats.lst:346
            UpsiFeatEntry {
                key: "Unwilling Participant",
                category: FeatCategory::Psionic,
                name: "Unwilling Participant",
                description: Some("You can attempt to force others into your collective."),
                pretext: None,
                source_page: Some("p.119"),
                benefit: Some("You may attempt to force a living creature into your collective.  The target may attempt a Will save (DC %1) to resist.  If the target fails the save, it may attempt another Will save at the same DC every 24 hours thereafter, but is otherwise unable to leave the collective unless you allow it.  Special: Creatures forced into your collective using Unwilling Participant are considered willing members for any collective-related effects unless they succeed on another Will save at the same DC to resist being forced into the collective.  A successful save means the creature resisted the speciifc effect but is still a member of the collective.|UnwillingParticipantDC"),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Collective", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Up the Walls -- up_feats.lst:347
            UpsiFeatEntry {
                key: "Up the Walls",
                category: FeatCategory::Psionic,
                name: "Up the Walls",
                description: Some("Run on walls and ceilings"),
                pretext: None,
                source_page: Some("p.120"),
                benefit: None,
                prerequisites: Some(&["PRESTAT:1,WIS=13", "PREVARGTEQ:IsPsionic,1"]),
            },
            // Urban Tracking -- up_feats.lst:348
            UpsiFeatEntry {
                key: "Urban Tracking",
                category: FeatCategory::General,
                name: "Urban Tracking",
                description: Some("You can track down the location of missing persons or wanted individuals within communities."),
                pretext: None,
                source_page: Some("p.120"),
                benefit: Some("To find the trail of an individual or to follow it for 1 hour requires a Diplomacy check to gather information. You must make another Diplomacy check every hour of the search, as well as each time the trail becomes difficult to follow, such as when it moves to a different area of town. The DC of the check, and the number of checks required to track down your quarry, depends on the community size and the conditions. If you fail a Diplomacy check, you can retry after 1 hour of questioning. The game master should roll the number of checks required secretly, so that the player doesn't know exactly how long the task will require.&nl; Normal: A character without this feat can use Diplomacy to find out information about a particular individual, but each check takes 1d4+1 hours and doesn't allow effective tailing."),
                prerequisites: None,
            },
            // Widen Power -- up_feats.lst:353
            UpsiFeatEntry {
                key: "Widen Power",
                category: FeatCategory::Metapsionic,
                name: "Widen Power",
                description: Some("Double power's area"),
                pretext: None,
                source_page: Some("p.120"),
                benefit: None,
                prerequisites: Some(&["PREVARGTEQ:IsPsionic,1"]),
            },
            // Wildblood Mage -- up_feats.lst:356
            UpsiFeatEntry {
                key: "Wildblood Mage",
                category: FeatCategory::General,
                name: "Wildblood Mage",
                description: Some("Your wilder manifester level gains a +2 bonus as long as this bonus doesn't raise your manifester level above your current Hit Dice. Your sorcerer caster level gains a +2 bonus as long as this bonus doesn't raise your caster level above your current Hit Dice. In addition, you can use your wild surge on spells, increasing your caster level the wild surge amount, and your bloodline arcana is applied to applicable powers. [NOT IMPLEMENTED]"),
                pretext: None,
                source_page: Some("p.120"),
                benefit: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Wilder ~ Wild Surge", "PREABILITY:1,CATEGORY=Special Ability,TYPE.Sorcerer Bloodline", "PREVARGTEQ:WildSurge,1"]),
            },
            // Wild Talent -- up_feats.lst:357
            UpsiFeatEntry {
                key: "Wild Talent",
                category: FeatCategory::General,
                name: "Wild Talent",
                description: Some("Gain psionic ability and 2 power points"),
                pretext: None,
                source_page: Some("p.120"),
                benefit: None,
                prerequisites: None,
            },
            // Wounding Attack -- up_feats.lst:358
            UpsiFeatEntry {
                key: "Wounding Attack",
                category: FeatCategory::Psionic,
                name: "Wounding Attack",
                description: Some("Wound opponents with your attack"),
                pretext: None,
                source_page: Some("p.120"),
                benefit: None,
                prerequisites: Some(&["PREMULT:1,[PRETOTALAB:8],[PREVARGTEQ:PrereqBAB,8]", "PREVARGTEQ:IsPsionic,1"]),
            },        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_has_221_records() {
        assert_eq!(feat_tables().len(), 221);
    }

    #[test]
    fn every_record_carries_real_content() {
        for e in feat_tables() {
            assert!(
                e.description.is_some() || e.benefit.is_some(),
                "{} has neither DESC: nor BENEFIT:",
                e.key
            );
        }
    }

    #[test]
    fn no_record_is_deferred() {
        assert_eq!(
            feat_tables()
                .iter()
                .filter(|e| e.description.is_none() && e.benefit.is_none())
                .count(),
            0
        );
    }

    #[test]
    fn keys_are_unique_within_book() {
        let keys: std::collections::BTreeSet<&str> = feat_tables().iter().map(|e| e.key).collect();
        assert_eq!(keys.len(), feat_tables().len());
    }

    #[test]
    fn the_desc_benefit_split_is_the_real_one() {
        let both = feat_tables()
            .iter()
            .filter(|e| e.description.is_some() && e.benefit.is_some())
            .count();
        let desc_only = feat_tables()
            .iter()
            .filter(|e| e.description.is_some() && e.benefit.is_none())
            .count();
        let benefit_only = feat_tables()
            .iter()
            .filter(|e| e.benefit.is_some() && e.description.is_none())
            .count();
        assert_eq!(both, 5, "Piranha Strike, Psionic Shot, Psionic Talent, Unwilling Participant, Urban Tracking");
        assert_eq!(desc_only, 216, "Dreamscarred Press's own DESC:-is-complete convention -- see this module's doc comment");
        assert_eq!(benefit_only, 0, "no record in this book carries BENEFIT: without DESC:");
        assert_eq!(both + desc_only + benefit_only, 221);
    }
}
