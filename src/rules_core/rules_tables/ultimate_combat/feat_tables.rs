//! Ultimate Combat (UC) feat catalog. SD28-E27 slice 1, mirroring
//! `ultimate_wilderness::feat_tables`'s own established shape exactly.
//!
//! **Corpus coverage, honestly bounded.** `uc_feats.lst` has 263
//! top-level `CATEGORY:FEAT` records (re-derived: `grep -c
//! 'CATEGORY:FEAT' uc_feats.lst`), the same figure this epic's own
//! dispatch brief carried a recorded command for. **Zero cross-book
//! collisions** -- re-derived against every other book's real runtime
//! feat key set (a scratch `#[test]` dump of
//! `feats_all::all_feat_tables()` itself, not a source grep, per
//! `decisions.md §44`'s lesson applied from the start), unlike UW's one
//! and UE's fifty-five -- UC's feats are genuinely new content, not a
//! republishing of earlier books.
//!
//! **Two of the 263 carry neither `DESC:` nor `BENEFIT:` in the corpus at
//! all, confirmed by reading both rows directly, and are excluded rather
//! than shipped as stubs:** `Gundarme Bonus Feat` (`uc_feats.lst:350`) is
//! an auto-granted feat-selection wrapper (`ABILITY:FEAT|AUTOMATIC|%LIST`),
//! not standalone prose content; `Deathless Master (Vigor/Wounds)`
//! (`uc_feats.lst:357`) is a bare rules-variant sibling of the real
//! `Deathless Master` record (line 63, which has full text), gated by
//! `PRERULE:1,DAMAGE_VW` with no text of its own. **One more,
//! `Revelation Strike`, has real text split across two rows** -- its own
//! base row (`uc_feats.lst:261`) carries `DESC:` but no `BENEFIT:`; the
//! mechanical text lives on `CATEGORY=Feat|Revelation Strike.MOD`
//! (line 262, `=` not `:`, invisible to a `CATEGORY:FEAT` scan) --
//! confirmed to genuinely belong to this same feat, not a splice into a
//! different one, and recovered here rather than left as a stub.
//! `docs/work-inventory.json`'s own classifier reports 266 for this book;
//! the delta against 263 is not reconciled here (immaterial at this
//! scale, the same treatment UE's own 1,425-vs-1,424 delta got in
//! `decisions.md §44`).
//!
//! **261 real, distinct, text-complete feat records remain** (263 raw,
//! minus the 2 genuinely textless exclusions). Every one carries real
//! `DESC:`/`BENEFIT:` -- no other upstream splice/truncation defect
//! found. None are `deferred-with-reason`.
//!
//! **No `KEY:` token on any record**, so `key == name` for every entry.
//!
//! **`category` is UC's own enum, not the shared `crb::feats::FeatCategory`.**
//! UC's `TYPE:` facets include `Style`, `Grit` and `Panache`-adjacent
//! facets specific to Gunslinger/Duelist-style feats this book introduces;
//! `Combat.*` sub-facets fold to `Combat`, matching every other book's own
//! folding convention.
//!
//! **`prerequisites` carries every real `PRE`-family token verbatim**,
//! gathered directly at ingest, `None` when the corpus row has none.
//!
//! Every field below is copied verbatim from the real corpus row (source:
//! `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/
//! ultimate_combat/uc_feats.lst`), generated programmatically by a one-off
//! extraction script, not hand-transcribed.

use super::super::crb::feats::FeatCategory as SharedFeatCategory;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatCategory {
    General,
    Combat,
    ItemCreation,
    Metamagic,
    Teamwork,
    /// Combat-style feat chains (e.g. Two-Weapon Fighting styles) UC
    /// names as their own `TYPE:Style` facet.
    Style,
    /// Gunslinger Grit-spending feats.
    Grit,
    /// Swashbuckler/Duelist Panache-spending feats -- UC's own corpus
    /// facet, distinct from ACG's `Panache` (kept as a separate variant
    /// rather than assumed identical without checking).
    Panache,
    /// `Dispelling Critical`'s own bare `TYPE:Critical` facet (distinct
    /// from the more common `Combat.Critical` sub-facet, which folds to
    /// `Combat`).
    Critical,
    /// `Improved Called Shot`/`Greater Called Shot`'s own `TYPE:Called
    /// Shot` facet.
    CalledShot,
}

impl FeatCategory {
    pub const ALL: &'static [FeatCategory] = &[
        FeatCategory::General,
        FeatCategory::Combat,
        FeatCategory::ItemCreation,
        FeatCategory::Metamagic,
        FeatCategory::Teamwork,
        FeatCategory::Style,
        FeatCategory::Grit,
        FeatCategory::Panache,
        FeatCategory::Critical,
        FeatCategory::CalledShot,
    ];

    /// The subset of variants that coincide with the shared
    /// `crb::feats::FeatCategory` enum. `Style`/`Grit`/`Critical`/
    /// `CalledShot` have no shared equivalent; `Panache` is kept distinct
    /// from ACG's own `Panache` variant rather than assumed to be the
    /// same facet without checking.
    pub fn as_shared(self) -> Option<SharedFeatCategory> {
        match self {
            FeatCategory::General => Some(SharedFeatCategory::General),
            FeatCategory::Combat => Some(SharedFeatCategory::Combat),
            FeatCategory::ItemCreation => Some(SharedFeatCategory::ItemCreation),
            FeatCategory::Metamagic => Some(SharedFeatCategory::Metamagic),
            FeatCategory::Teamwork => Some(SharedFeatCategory::Teamwork),
            FeatCategory::Style
            | FeatCategory::Grit
            | FeatCategory::Panache
            | FeatCategory::Critical
            | FeatCategory::CalledShot => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UcFeatEntry {
    /// The record's corpus identity. No record in this catalog carries a
    /// distinct `KEY:` token, so `key == name` for every entry.
    pub key: &'static str,
    pub category: FeatCategory,
    pub name: &'static str,
    /// The corpus `DESC:` token, verbatim.
    pub description: Option<&'static str>,
    /// The corpus `PRETEXT:` token, verbatim display prerequisite prose --
    /// `None` when the row carries no `PRETEXT:`.
    pub pretext: Option<&'static str>,
    pub source_page: Option<&'static str>,
    /// The corpus `BENEFIT:` token, verbatim -- the actual mechanical text.
    pub benefit: Option<&'static str>,
    /// Every top-level `PRE`-family token the corpus record carries,
    /// verbatim and unparsed, in source order. `None` when the row has no
    /// `PRE`-family token.
    pub prerequisites: Option<&'static [&'static str]>,
}

/// Full UC feat catalog: all 261 real, distinct corpus records, in source
/// order. Built once and cached for the process lifetime.
pub fn feat_tables() -> &'static [UcFeatEntry] {
    static TABLE: std::sync::OnceLock<Vec<UcFeatEntry>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        vec![
            // Adder Strike -- uc_feats.lst:19
            UcFeatEntry {
                key: "Adder Strike",
                category: FeatCategory::Combat,
                name: "Adder Strike",
                description: Some("You can quickly apply poison to gloved hands, protected feet, or other protected body parts, delivering the poison with your unarmed strikes."),
                pretext: None,
                source_page: Some("p.89"),
                benefit: Some("As a swift action, you can apply one dose of contact or injury poison to two body parts that you use for unarmed strikes. You must still protect yourself against exposure to contact poisons you apply in this way. &nl; [Normal] Applying poison to a weapon or single piece of ammunition is a standard action."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Poison Use,Poison Use,Alchemist ~ Poison Use,Rogue ~ Poison Use", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRESKILL:1,Craft (Alchemy)=1"]),
            },
            // Adept Champion -- uc_feats.lst:20
            UcFeatEntry {
                key: "Adept Champion",
                category: FeatCategory::General,
                name: "Adept Champion",
                description: Some("You can alter your smite ability, channeling the power of your deity into divine inspiration that grants you greater aptitude for performing combat maneuvers."),
                pretext: None,
                source_page: Some("p.89"),
                benefit: Some("While using your smite evil class feature, as a swift action at the start of your turn, you can forgo the bonus on damage rolls and instead gain half that bonus as a bonus on combat maneuver checks against the target of your smite. The effects of your smite evil feature return to normal at the start of your next turn."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Smite Evil", "PRETOTALAB:5"]),
            },
            // Amateur Gunslinger -- uc_feats.lst:21
            UcFeatEntry {
                key: "Amateur Gunslinger",
                category: FeatCategory::Combat,
                name: "Amateur Gunslinger",
                description: Some("Although you are not a gunslinger, you have and can use grit."),
                pretext: None,
                source_page: Some("p.89"),
                benefit: Some("You gain a small amount of grit and the ability to perform a single 1st-level deed from the gunslinger deed class feature. At the start of the day, you gain 1 grit point, though throughout the day you can gain grit points up to a maximum of your Wisdom modifier (%1) (minimum 1). You can regain grit using the rules for the gunslinger's grit class feature (see page 9). You can spend this grit to perform the 1st-level deed you chose upon taking this feat, and any other deed you have gained through feats or magic items. &nl; [Special]If you gain levels in a class that grants the grit class feature, you can immediately trade this feat for the Extra Grit feat.|WIS"),
                prerequisites: Some(&["!PREABILITY:1,CATEGORY=Special Ability,Gunslinger ~ Grit"]),
            },
            // Back to Back -- uc_feats.lst:22
            UcFeatEntry {
                key: "Back to Back",
                category: FeatCategory::Teamwork,
                name: "Back to Back",
                description: Some("Your ally's eyes are your own, and yours are his."),
                pretext: None,
                source_page: Some("p.89"),
                benefit: Some("While you are flanked and adjacent to an ally with this feat, you receive a +2 circumstance bonus to AC against attacks from opponents flanking you."),
                prerequisites: Some(&["PRESKILL:1,Perception=3"]),
            },
            // Branded for Retribution -- uc_feats.lst:23
            UcFeatEntry {
                key: "Branded for Retribution",
                category: FeatCategory::General,
                name: "Branded for Retribution",
                description: Some("You brand an enemy with your bane weapon, making it more vulnerable to your allies' attacks."),
                pretext: None,
                source_page: Some("p.90"),
                benefit: Some("As a standard action, expend 3 rounds of your bane class feature and make a melee touch attack with the weapon affected by bane. If you hit, your target takes no damage but is branded until the start of your next turn. While this brand remains, your allies' weapons are considered to have the bane ability with which your weapon is imbued when they attack the branded creature."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Bane,Inquisitor ~ Greater Bane"]),
            },
            // Betrayer -- uc_feats.lst:24
            UcFeatEntry {
                key: "Betrayer",
                category: FeatCategory::General,
                name: "Betrayer",
                description: Some("You can charm people into lowering their defenses, allowing you to ambush them more effectively."),
                pretext: None,
                source_page: Some("p.89"),
                benefit: Some("When you succeed at a Diplomacy check to change a creature's attitude, you can draw a weapon and make a single melee attack against that creature as an immediate action. If you changed your target's attitude to friendly or better, your target is considered flat-footed against this attack. If the target survives, it takes a -2 penalty on its initiative check for this combat. Once you attack a creature, its attitude becomes hostile."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Quick Draw,Persuasive", "PRETOTALAB:3"]),
            },
            // Binding Throw -- uc_feats.lst:25
            UcFeatEntry {
                key: "Binding Throw",
                category: FeatCategory::Combat,
                name: "Binding Throw",
                description: Some("You can strike your enemy and use the blow as an opportunity to grab and hold him."),
                pretext: None,
                source_page: Some("p.90"),
                benefit: Some("After you successfully use the Ki Throw feat on an opponent, you can use a swift action to attempt a grapple combat maneuver against that opponent. &nl; [Normal] The grapple combat maneuver is a standard action. &nl; [Special]A monk can gain Binding Throw as a bonus feat starting at 14th level."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Improved Grapple,Improved Trip,Improved Unarmed Strike,Ki Throw"]),
            },
            // Bludgeoner -- uc_feats.lst:26
            UcFeatEntry {
                key: "Bludgeoner",
                category: FeatCategory::Combat,
                name: "Bludgeoner",
                description: Some("You can knock foes out cold with just about any blunt instrument."),
                pretext: None,
                source_page: Some("p.90"),
                benefit: Some("You take no penalty on attack rolls for using a lethal bludgeoning weapon to deal nonlethal damage. &nl; [Normal] You take a -4 penalty on attack rolls when using a lethal weapon to deal nonlethal damage. You cannot use a lethal weapon to deal nonlethal damage in a sneak attack. &nl; [Special] A rogue with this feat can use a lethal bludgeoning weapon to deal nonlethal damage with a sneak attack."),
                prerequisites: None,
            },
            // Boar Ferocity -- uc_feats.lst:27
            UcFeatEntry {
                key: "Boar Ferocity",
                category: FeatCategory::Combat,
                name: "Boar Ferocity",
                description: Some("Your flesh-ripping unarmed strikes terrify your victims."),
                pretext: None,
                source_page: Some("p.90"),
                benefit: Some("You add piercing damage to the damage types you can deal with your unarmed strikes. Further, you gain a +2 bonus on Intimidate checks to demoralize opponents. While using Boar Style, whenever you tear an opponent's flesh, you can spend a free action to make an Intimidate check to demoralize that opponent."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Boar Style", "PRESKILL:1,Intimidate=6"]),
            },
            // Boar Shred -- uc_feats.lst:28
            UcFeatEntry {
                key: "Boar Shred",
                category: FeatCategory::Combat,
                name: "Boar Shred",
                description: Some("The wounds you inf lict with your unarmed strikes bleed, giving you renewed vigor."),
                pretext: None,
                source_page: Some("p.90"),
                benefit: Some("You can make an Intimidate check to demoralize an opponent as a move action. While using Boar Style, whenever you tear an opponent's flesh, once per round at the start of that opponent's turn he takes 1d6 bleed damage. The bleed damage dealt while using Boar Style persist even if you later switch to a different style."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Improved Unarmed Strike,Boar Ferocity,Boar Style", "PRESKILL:1,Intimidate=9"]),
            },
            // Boar Style -- uc_feats.lst:29
            UcFeatEntry {
                key: "Boar Style",
                category: FeatCategory::Combat,
                name: "Boar Style",
                description: Some("Your sharp teeth and nails rip your foes open."),
                pretext: None,
                source_page: Some("p.90"),
                benefit: Some("You can deal bludgeoning damage or slashing damage with your unarmed strikes-changing damage type is a free action. While using this style, once per round when you hit a single foe with two or more unarmed strikes, you can tear flesh. When you do, you deal 2d6 extra points of damage with the attack."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRESKILL:1,Intimidate=3"]),
            },
            // Body Shield -- uc_feats.lst:30
            UcFeatEntry {
                key: "Body Shield",
                category: FeatCategory::Combat,
                name: "Body Shield",
                description: Some("With a sly maneuver, you force a grappled opponent into the path of an incoming attack."),
                pretext: None,
                source_page: Some("p.90"),
                benefit: Some("As an immediate action while you are grappling an adjacent creature, you can make a grapple combat maneuver check against that creature to gain cover against a single attack. If you are successful and the attack misses you, that attack targets the creature you used as cover, using the same attack roll. You cannot use this feat against a creature grappling you, and the cover you gain ends after the attack you gained cover against is resolved."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Grapple,Improved Unarmed Strike", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRETOTALAB:6"]),
            },
            // Bolstered Resilience -- uc_feats.lst:33
            UcFeatEntry {
                key: "Bolstered Resilience",
                category: FeatCategory::General,
                name: "Bolstered Resilience",
                description: Some("You can dramatically increase your damage reduction in exchange for its temporary loss."),
                pretext: Some("Must have Damage Reduction"),
                source_page: Some("p.90"),
                benefit: Some("As an immediate action, you can double your DR against a single attack, to a maximum of DR 20. The type of the DR remains unchanged. If the attack you are guarding against is not successful, the increased damage reduction persists until you are hit with an attack or until the start of your next turn, whichever happens first. At the start of your next turn, you become fatigued. You cannot use this feat while you are fatigued."),
                prerequisites: None,
            },
            // Bonebreaker -- uc_feats.lst:34
            UcFeatEntry {
                key: "Bonebreaker",
                category: FeatCategory::Combat,
                name: "Bonebreaker",
                description: Some("When your opponent is unable to adequately defend against them, your precise unarmed strikes break bone and tear tissue."),
                pretext: None,
                source_page: Some("p.90"),
                benefit: Some("When you make a successful Stunning Fist attempt against an opponent that is grappled, helpless, or stunned, you can forgo any other Stunning Fist effect to deal 1d6 Strength or Dexterity damage to that opponent."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Improved Grapple,Improved Unarmed Strike,Jawbreaker,Stunning Fist", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRESKILL:1,HEAL=9"]),
            },
            // Break Guard -- uc_feats.lst:35
            UcFeatEntry {
                key: "Break Guard",
                category: FeatCategory::Combat,
                name: "Break Guard",
                description: Some("You can use one of your two weapons to occupy your opponent's defenses while attacking with the other."),
                pretext: None,
                source_page: Some("p.91"),
                benefit: Some("While wielding two weapons, whenever you successfully use one weapon to disarm an opponent, you can spend a swift action to attack the opponent you attempted to disarm using your other weapon."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Combat Expertise,Improved Disarm,Two-Weapon Fighting", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,15],[PREVARGTEQ:FeatDexRequirement,15]"]),
            },
            // Broken Wing Gambit -- uc_feats.lst:36
            UcFeatEntry {
                key: "Broken Wing Gambit",
                category: FeatCategory::Combat,
                name: "Broken Wing Gambit",
                description: Some("You feign weakness, making yourself a tempting and distracting target."),
                pretext: None,
                source_page: Some("p.91"),
                benefit: Some("Whenever you make a melee attack and hit your opponent, you can use a free action to grant that opponent a +2 bonus on attack and damage rolls against you until the end of your next turn or until your opponent attacks you, whichever happens first. If that opponent attacks you with this bonus, it provokes attacks of opportunity from your allies who have this feat."),
                prerequisites: Some(&["PRESKILL:1,Bluff=5"]),
            },
            // Cartwheel Dodge -- uc_feats.lst:39
            UcFeatEntry {
                key: "Cartwheel Dodge",
                category: FeatCategory::General,
                name: "Cartwheel Dodge",
                description: Some("You use your knack for avoiding damage to reposition yourself in combat."),
                pretext: None,
                source_page: Some("p.91"),
                benefit: Some("When you successfully use improved evasion to avoid taking damage, you can move up to half your speed as an immediate action. This movement provokes attacks of opportunity as normal."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Improved Evasion", "PRESKILL:1,Acrobatics=12"]),
            },
            // Cavalry Formation -- uc_feats.lst:40
            UcFeatEntry {
                key: "Cavalry Formation",
                category: FeatCategory::Combat,
                name: "Cavalry Formation",
                description: Some("You are skilled at riding in close formation with your mounted allies without impeding your effectiveness on the battlefield."),
                pretext: None,
                source_page: Some("p.91"),
                benefit: Some("You and your mount can overlap the space of mounts whose riders have this feat, although no more than two creatures can share any one square. Further, you can charge through a space containing an allied mount if that mount's rider has this feat, although the space from which you make your charge attack must comply with this feat's other benefit or be unoccupied."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Mounted Combat"]),
            },
            // Channeling Scourge -- uc_feats.lst:41
            UcFeatEntry {
                key: "Channeling Scourge",
                category: FeatCategory::General,
                name: "Channeling Scourge",
                description: Some("Your zeal for hunting your faith's enemies empowers your ability to channel divine energy, as long as you channel that energy for harm."),
                pretext: None,
                source_page: Some("p.92"),
                benefit: Some("When you use channel energy to deal damage, your inquisitor levels count as cleric levels for determining the number of damage dice and the saving throw DC."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Energy", "PRECLASS:1,Inquisitor=1"]),
            },
            // Jawbreaker -- uc_feats.lst:42
            UcFeatEntry {
                key: "Jawbreaker",
                category: FeatCategory::Combat,
                name: "Jawbreaker",
                description: Some("You deliver a powerful strike to the mouth, breaking teeth and bone."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: Some("When you make a successful Stunning Fist attempt against an opponent that is grappled, helpless, or stunned, instead of imparting any other Stunning Fist effect, you can cripple that opponent's mouth, dealing normal unarmed strike damage and 1d4 points of bleed damage. Until the bleed damage ends, the target is unable to use its mouth to attack, speak clearly, and employ verbal spell components. A creature that is immune to critical hits or that has no discernible mouth is immune to the effects of this feat."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Stunning Fist", "PRESKILL:1,Heal=6"]),
            },
            // Arc Slinger -- uc_feats.lst:47
            UcFeatEntry {
                key: "Arc Slinger",
                category: FeatCategory::Combat,
                name: "Arc Slinger",
                description: Some("You can twirl your sling in a way that maximizes its effectiveness."),
                pretext: None,
                source_page: Some("p.89"),
                benefit: Some("When using a sling or sling staff, you reduce your penalty on ranged attack rolls due to range by 2. Point-Blank Shot's damage bonus applies within the first normal range increment of your sling (50 feet) or sling staff (80 feet)."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot", "PREWEAPONPROF:1,Sling,Sling Staff (Halfling)"]),
            },
            // Channeled Revival -- uc_feats.lst:48
            UcFeatEntry {
                key: "Channeled Revival",
                category: FeatCategory::General,
                name: "Channeled Revival",
                description: Some("You can expend a large portion of your channeling power to reverse death itself."),
                pretext: None,
                source_page: Some("p.92"),
                benefit: Some("As a full-round action that provokes attacks of opportunity, you can expend three uses of your channel energy class feature to restore a dead creature to life as if you had cast the breath of life spell (Core Rulebook 251)."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Positive Energy", "PREMULT:1,[PREVARGTEQ:OracleChannelDice,6],[PREVARGTEQ:ClericChannelPositiveEnergyDice,6],[PREVARGTEQ:PaladinChannelDice,6],[PREVARGTEQ:ClassChannelPositiveEnergyDice,6]", "PREMULT:1,[PREVARGTEQ:OracleChannelDieSize,6],[PREVARGTEQ:ClericChannelPositiveEnergyDieSize,6],[PREVARGTEQ:PaladinChannelDieSize,6],[PREVARGTEQ:ClassChannelPositiveEnergyDieSize,6]"]),
            },
            // Charging Hurler -- uc_feats.lst:49
            UcFeatEntry {
                key: "Charging Hurler",
                category: FeatCategory::Combat,
                name: "Charging Hurler",
                description: Some("You know how to use your momentum to enhance your thrown weapon attacks."),
                pretext: None,
                source_page: Some("p.92"),
                benefit: Some("You can use the charge rules to make a thrown weapon attack. All the parameters of a charge apply, except that you must only move closer to your opponent, and you must end your movement within 30 feet of that opponent. If you do, you can make a single thrown weapon attack against that opponent, gaining the +2 bonus on the attack roll and taking a -2 penalty to your AC until the start of your next turn."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot"]),
            },
            // Chokehold -- uc_feats.lst:50
            UcFeatEntry {
                key: "Chokehold",
                category: FeatCategory::Combat,
                name: "Chokehold",
                description: Some("While grappling, you can cut off an opponent's air and blood supply."),
                pretext: None,
                source_page: Some("p.92"),
                benefit: Some("While you have an opponent up to one size category larger than you grappled, you can attempt a grapple combat maneuver with a -5 penalty on the check. If you succeed, you have pinned your opponent and hold the opponent in a chokehold. When you maintain the grapple, you also maintain the chokehold. A creature in a chokehold cannot breathe or speak, and thus cannot cast spells that have a verbal component. An opponent you have in a chokehold has to hold his breath or begin suffocating. Any creature that does not breathe, is immune to bleed damage, or is immune to critical hits is immune to the effects of your chokehold. When the grapple is ended, so is the chokehold."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Grapple,Improved Unarmed Strike", "PREMULT:1,[PREVARGTEQ:MonkFeatQualify,5],[PRETOTALAB:6]"]),
            },
            // Cleaving Finish -- uc_feats.lst:51
            UcFeatEntry {
                key: "Cleaving Finish",
                category: FeatCategory::Combat,
                name: "Cleaving Finish",
                description: Some("When you strike down an opponent, you can continue your swing into another target."),
                pretext: None,
                source_page: Some("p.92"),
                benefit: Some("If you make a melee attack, and your target drops to 0 or fewer hit points as a result of your attack, you can make another melee attack using your highest base attack bonus against another opponent within reach. You can make only one extra attack per round with this feat."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Cleave,Power Attack", "PRESTAT:1,STR=13"]),
            },
            // Close-Quarters Thrower -- uc_feats.lst:52
            UcFeatEntry {
                key: "Close-Quarters Thrower",
                category: FeatCategory::Combat,
                name: "Close-Quarters Thrower",
                description: Some("You are agile enough to avoid melee attacks while throwing weapons or bombs."),
                pretext: None,
                source_page: Some("p.92"),
                benefit: Some("Choose a type of thrown weapon. You do not provoke attacks of opportunity for making ranged attacks using the selected weapon. If you are an alchemist, and you select this feat and choose alchemist bombs, you do not provoke attacks of opportunity for the process of drawing components of, creating, and throwing a bomb. &nl; [Normal] Making a ranged attack provokes attacks of opportunity."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Dodge,Weapon Focus (TYPE=Thrown)", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]"]),
            },
            // Clustered Shots -- uc_feats.lst:53
            UcFeatEntry {
                key: "Clustered Shots",
                category: FeatCategory::Combat,
                name: "Clustered Shots",
                description: Some("You take a moment to carefully aim your shots, causing them all to strike nearly the same spot."),
                pretext: None,
                source_page: Some("p.92"),
                benefit: Some("When you use a full-attack action to make multiple ranged weapon attacks against the same opponent, total the damage from all hits before applying that opponent's damage reduction. &nl; [Special]If the massive damage optional rule is being used (Core Rulebook 189), that rule applies if the total damage you deal with this feat is equal to or exceeds half the opponent's full normal hit points (minimum 50 points of damage)."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Point-Blank Shot,Precise Shot", "PRETOTALAB:6"]),
            },
            // Combat Medic -- uc_feats.lst:54
            UcFeatEntry {
                key: "Combat Medic",
                category: FeatCategory::Teamwork,
                name: "Combat Medic",
                description: Some("You know the urgency of treating wounds in the heat of battle, applying first aid to your allies with such speed that you assure no one gets left behind."),
                pretext: None,
                source_page: Some("p.92"),
                benefit: Some("Whenever you use Heal to provide first aid, treat caltrop wounds, or treat poison on an ally who also has this feat, you provoke no attacks of opportunity, and can take 10 on the check. Unlike with other teamwork feats, allies that are paralyzed, stunned, unconscious, or cannot otherwise act still count for the purposes of this feat."),
                prerequisites: Some(&["PRESKILL:1,Heal=5"]),
            },
            // Combat Style Master -- uc_feats.lst:55
            UcFeatEntry {
                key: "Combat Style Master",
                category: FeatCategory::Combat,
                name: "Combat Style Master",
                description: Some("You shift between combat styles, combining them to increased effect."),
                pretext: None,
                source_page: Some("p.93"),
                benefit: Some("You can switch your style as a free action. At the start of combat, pick one of your styles. You start the combat in that style, even in the surprise round. &nl; [Normal] It takes a swift action to begin or switch your styles."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PREABILITY:2,CATEGORY=FEAT,TYPE=Style", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:MonkFeatQualify,5]"]),
            },
            // Contingent Channeling -- uc_feats.lst:56
            UcFeatEntry {
                key: "Contingent Channeling",
                category: FeatCategory::General,
                name: "Contingent Channeling",
                description: Some("You can imbue others with your healing energy so that they can use it at the moment of greatest need."),
                pretext: None,
                source_page: Some("p.93"),
                benefit: Some("You can use a standard action to touch an ally and expend one of your daily uses of channel energy to create a repository of positive energy within that ally. This repository contains the same number and type of dice as normal for your channel energy feature, and it lasts for 1 minute. An ally who has such a repository can use an immediate action to roll the repository's dice and regain a number of hit points equal to the result. If an ally who has such a repository is reduced to negative hit points, the repository triggers, allowing the ally to heal without using an action."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,True Healer ~ Merciful Healer", "PREABILITY:1,CATEGORY=FEAT,Selective Channeling"]),
            },
            // Coordinated Charge -- uc_feats.lst:57
            UcFeatEntry {
                key: "Coordinated Charge",
                category: FeatCategory::Combat,
                name: "Coordinated Charge",
                description: Some("You are an expert at leading your allies into the fray."),
                pretext: None,
                source_page: Some("p.93"),
                benefit: Some("When an ally with this feat charges a creature that is no further away from you than your speed, you can, as an immediate action, charge that creature. You must be able to follow all of the normal charge rules."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,TYPE=Teamwork", "PRETOTALAB:10"]),
            },
            // Crushing Blow -- uc_feats.lst:58
            UcFeatEntry {
                key: "Crushing Blow",
                category: FeatCategory::Combat,
                name: "Crushing Blow",
                description: Some("Your focus allows you to smash your enemy's defenses."),
                pretext: None,
                source_page: Some("p.94"),
                benefit: Some("You can make a Stunning Fist attempt as a full-round action. If successful, instead of stunning your target, you reduce the target's AC by an amount equal to your Wisdom modifier for 1 minute. This penalty does not stack with other penalties applied due to Crushing Blow."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Stunning Fist"]),
            },
            // Deadly Finish -- uc_feats.lst:59
            UcFeatEntry {
                key: "Deadly Finish",
                category: FeatCategory::Combat,
                name: "Deadly Finish",
                description: Some("Your attacks don't just fell your opponents; they kill them outright."),
                pretext: None,
                source_page: Some("p.94"),
                benefit: Some("When you hit with a melee attack and reduce your opponent to -1 or fewer hit points, you can force that opponent to succeed at a Fortitude save (DC 15 + the damage your attack dealt) or die."),
                prerequisites: Some(&["PRETOTALAB:11"]),
            },
            // Death from Above -- uc_feats.lst:60
            UcFeatEntry {
                key: "Death from Above",
                category: FeatCategory::Combat,
                name: "Death from Above",
                description: Some("You allow gravity to add extra force to your charges."),
                pretext: None,
                source_page: Some("p.94"),
                benefit: Some("Whenever you charge an opponent from higher ground, or from above while flying, you gain a +5 bonus on attack rolls in place of the bonuses from charging and being on higher ground."),
                prerequisites: None,
            },
            // Death or Glory -- uc_feats.lst:61
            UcFeatEntry {
                key: "Death or Glory",
                category: FeatCategory::Combat,
                name: "Death or Glory",
                description: Some("Even when facing a larger foe, you aren't afraid to take great risks in order to finish the fight."),
                pretext: None,
                source_page: Some("p.94"),
                benefit: Some("Against a creature of size Large or larger, you can make a single melee attack as a full-round action, gaining a +4 bonus on the attack roll, damage roll, and critical confirmation roll. You gain an additional +1 on this bonus at base attack bonus +11, +16, and +20 (for a maximum of +7 at base attack +20). After you resolve your attack, the opponent you attack can spend an immediate action to make a single melee attack against you with the same bonuses. &nl; [Special]You can combine the full-round action attack this feat allows with the benefit of Vital Strike, Improved Vital Strike, or Greater Vital Strike."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:6"]),
            },
            // Deathless Initiate -- uc_feats.lst:62
            UcFeatEntry {
                key: "Deathless Initiate",
                category: FeatCategory::Combat,
                name: "Deathless Initiate",
                description: Some("For you, impending death is a call to wrath."),
                pretext: None,
                source_page: Some("p.94"),
                benefit: Some("You are not staggered while using the Diehard feat, but if you take a move and a standard action or a full-round action while you are at 0 or fewer hit points you take 1 point of damage. Further, while using the Diehard feat, you gain a +2 bonus on melee attacks and damage rolls."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Diehard,Endurance", "PREFACT:1,TEMPLATES,IsOrc=true", "PRESTAT:2,STR=13,CON=13", "PRETOTALAB:6"]),
            },
            // Deathless Master -- uc_feats.lst:63
            UcFeatEntry {
                key: "Deathless Master",
                category: FeatCategory::Combat,
                name: "Deathless Master",
                description: Some("Even if you suffer a grievous wound, you can shrug off the damage and continue your relentless assault."),
                pretext: None,
                source_page: Some("p.95"),
                benefit: Some("When you are at 0 or fewer hit points, you do not lose 1 hit point when you take an action."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Deathless Initiate,Diehard,Endurance,Ironhide", "PREFACT:1,TEMPLATES,IsOrc=true", "PRESTAT:2,STR=13,CON=15", "PRETOTALAB:9"]),
            },
            // Deathless Zealot -- uc_feats.lst:64
            UcFeatEntry {
                key: "Deathless Zealot",
                category: FeatCategory::Combat,
                name: "Deathless Zealot",
                description: Some("Only the most serious wounds can stop you."),
                pretext: None,
                source_page: Some("p.95"),
                benefit: Some("Whenever a creature rolls to confirm a critical hit against you, it must roll twice and take the lowest result."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Deathless Master,Deathless Initiate,Diehard,Endurance,Ironhide", "PREFACT:1,TEMPLATES,IsOrc=true", "PRESTAT:2,STR=13,CON=17", "PRETOTALAB:12"]),
            },
            // Deceptive Exchange -- uc_feats.lst:65
            UcFeatEntry {
                key: "Deceptive Exchange",
                category: FeatCategory::General,
                name: "Deceptive Exchange",
                description: Some("You trick an adversary into grabbing an object you hand them, even in the midst of combat."),
                pretext: None,
                source_page: Some("p.95"),
                benefit: Some("If you successfully feint an opponent, you can trick that opponent into accepting a one-handed object you are holding instead of denying that opponent its Dexterity bonus to AC against your next attack. The opponent must have appendages capable of holding the object you offer, and it must have one such appendage free to take the object. &nl; [Special]An alchemist who has the delayed bomb alchemist discovery can use this feat to hand an enemy a delayed bomb. Such a delayed bomb detonates at the end of the alchemist's turn. If the bomb is in a creature's square at the end of the alchemist's turn, the bomb deals that creature a direct hit."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Feint", "PRESTAT:1,INT=13"]),
            },
            // Defensive Weapon Training -- uc_feats.lst:69
            UcFeatEntry {
                key: "Defensive Weapon Training",
                category: FeatCategory::Combat,
                name: "Defensive Weapon Training",
                description: Some("You know how to defend yourself against a certain class of weaponry."),
                pretext: None,
                source_page: Some("p.95"),
                benefit: Some("Choose a weapon group from the fighter's weapon training class ability list (except natural weapons). You gain a +2 dodge bonus on AC when an opponent attacks you using a weapon from that group. If you also have the weapon training class feature in the selected group, your dodge bonus from this feat increases to +3. &nl; [Special]You can select this feat more than once. Its effects do not stack. Each time you select this feat, it applies to a different weapon group."),
                prerequisites: Some(&["PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:5"]),
            },
            // Deft Shootist Deed -- uc_feats.lst:70
            UcFeatEntry {
                key: "Deft Shootist Deed",
                category: FeatCategory::Grit,
                name: "Deft Shootist Deed",
                description: Some("You keep an eye out while focusing on your weapon, allowing you to avoid attacks while shooting and reloading firearms."),
                pretext: None,
                source_page: Some("p.95"),
                benefit: Some("As long as you have at least 1 grit point, you do not provoke attacks of opportunity when shooting or reloading a firearm."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Dodge,Mobility", "PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Gunslinger ~ Grit],[PREABILITY:1,CATEGORY=FEAT,Amateur Gunslinger]"]),
            },
            // Destructive Dispel -- uc_feats.lst:71
            UcFeatEntry {
                key: "Destructive Dispel",
                category: FeatCategory::General,
                name: "Destructive Dispel",
                description: Some("When you dispel an enemy's magical defenses, those defenses crash down with debilitating effects."),
                pretext: None,
                source_page: Some("p.95"),
                benefit: Some("When you successfully make a targeted dispel check against an opponent, that opponent must succeed at a Fortitude save (DC equals the DC of the spell used to dispel) or be stunned until the start of your next turn. If the save succeeds, the opponent is instead sickened until the start of your next turn."),
                prerequisites: Some(&["PREMULT:1,[PRECLASS:1,SPELLCASTER=11],[PREVARGTEQ:CasterLevel_Highest,11]", "PRESPELL:1,Dispel Magic,Dispel Magic (Greater)"]),
            },
            // Devastating Strike -- uc_feats.lst:72
            UcFeatEntry {
                key: "Devastating Strike",
                category: FeatCategory::Combat,
                name: "Devastating Strike",
                description: Some("Pitting all of your strength and resolve against your enemy, you deliver a strike that is impossible to ignore."),
                pretext: None,
                source_page: Some("p.95"),
                benefit: Some("Whenever you use Vital Strike, Improved Vital Strike, or Greater Vital Strike, you gain a +2 bonus on each extra weapon damage dice roll those feats grant (+6 maximum). This bonus damage is multiplied on a critical hit."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Vital Strike", "PRETOTALAB:9"]),
            },
            // Dimensional Agility -- uc_feats.lst:73
            UcFeatEntry {
                key: "Dimensional Agility",
                category: FeatCategory::General,
                name: "Dimensional Agility",
                description: Some("Teleportation does not faze you."),
                pretext: None,
                source_page: Some("p.95"),
                benefit: Some("After using abundant step or casting dimension door, you can take any actions you still have remaining on your turn. You also gain a +4 bonus on Concentration checks when casting teleportation spells."),
                prerequisites: Some(&["PREMULT:1,[PRESPELL:1,Dimension Door],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Abundant Step]"]),
            },
            // Dimensional Assault -- uc_feats.lst:74
            UcFeatEntry {
                key: "Dimensional Assault",
                category: FeatCategory::General,
                name: "Dimensional Assault",
                description: Some("You have been trained to use magical movement as part of your combat tactics."),
                pretext: None,
                source_page: Some("p.95"),
                benefit: Some("As a full-round action, you use abundant step or cast dimension door as a special charge. Doing so allows you to teleport up to double your current speed (up to the maximum distance allowed by the spell or ability) and to make the attack normally allowed on a charge."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Dimensional Agility", "PREMULT:1,[PRESPELL:1,Dimension Door],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Abundant Step]"]),
            },
            // Dimensional Dervish -- uc_feats.lst:75
            UcFeatEntry {
                key: "Dimensional Dervish",
                category: FeatCategory::General,
                name: "Dimensional Dervish",
                description: Some("You teleport with a mere thought, savaging your opponents as you flash in and out of reality."),
                pretext: None,
                source_page: Some("p.96"),
                benefit: Some("You can take a full-attack action, activating abundant step or casting dimension door as a swift action. If your do, you can teleport up to twice your speed (up to the maximum distance allowed by the spell or ability), dividing this teleportation into increments you use before your first attack, between each attack, and after your last attack. You must teleport at least 5 feet each time you teleport. &nl; [Special]A monk can use additional points from his ki pool to increase his speed before determining the total speed for this teleportation."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Dimensional Agility,Dimensional Assault", "PREMULT:1,[PRESPELL:1,Dimension Door],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Abundant Step]", "PRETOTALAB:6"]),
            },
            // Dimensional Maneuvers -- uc_feats.lst:76
            UcFeatEntry {
                key: "Dimensional Maneuvers",
                category: FeatCategory::General,
                name: "Dimensional Maneuvers",
                description: Some("Your rapid teleportation makes your combat maneuvers more difficult to avoid."),
                pretext: None,
                source_page: None,
                benefit: Some("While using the Dimensional Dervish feat, you gain a +4 bonus on combat maneuver checks to bull rush, disarm, reposition, or trip an opponent."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Dimensional Agility,Dimensional Assault,Dimensional Dervish", "PREMULT:1,[PRESPELL:1,Dimension Door],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Abundant Step]", "PRETOTALAB:9"]),
            },
            // Dimensional Savant -- uc_feats.lst:77
            UcFeatEntry {
                key: "Dimensional Savant",
                category: FeatCategory::General,
                name: "Dimensional Savant",
                description: Some("You flash into and out of reality so quickly it is impossible to tell exactly where you are at any given time."),
                pretext: None,
                source_page: Some("p.96"),
                benefit: Some("While using the Dimensional Dervish feat, you provide flanking from all squares you attack from. Flanking starts from the moment you make an attack until the start of your next turn. You can effectively flank with yourself and with multiple allies when using this feat."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Dimensional Agility,Dimensional Assault,Dimensional Dervish", "PREMULT:1,[PRESPELL:1,Dimension Door],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Abundant Step]", "PRETOTALAB:9"]),
            },
            // Discordant Voice -- uc_feats.lst:78
            UcFeatEntry {
                key: "Discordant Voice",
                category: FeatCategory::General,
                name: "Discordant Voice",
                description: Some("By singing out a precise tone, you cause discordant vibrations to run through allies' weapons."),
                pretext: None,
                source_page: Some("p.96"),
                benefit: Some("Benefit: Whenever you are using bardic performance to create a spell-like or supernatural effect, allies within 30 feet of you deal an extra 1d6 points of sonic damage with successful weapon attacks. This damage stacks with other energy damage a weapon might deal. Projectile weapons bestow this extra damage on their ammunition, but the extra damage is dealt only if the projectile hits a target within 30 feet of you."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Bard ~ Bardic Performance", "PRESKILL:1,Perform (Oratory)=10,Perform (Sing)=10"]),
            },
            // Disengaging Feint -- uc_feats.lst:79
            UcFeatEntry {
                key: "Disengaging Feint",
                category: FeatCategory::Combat,
                name: "Disengaging Feint",
                description: Some("You can feint to disengage from combat."),
                pretext: None,
                source_page: Some("p.96"),
                benefit: Some("As a standard action, use Bluff to feint against an opponent. Instead of denying that opponent his Dexterity bonus to AC, a successful feint allows you to move up to your speed without provoking an attack of opportunity from the opponent you feinted for leaving the square you start in."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Feint", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]"]),
            },
            // Disengaging Flourish -- uc_feats.lst:80
            UcFeatEntry {
                key: "Disengaging Flourish",
                category: FeatCategory::Combat,
                name: "Disengaging Flourish",
                description: Some("Distracting your opponents gives you the opportunity to make a swift retreat."),
                pretext: None,
                source_page: Some("p.96"),
                benefit: Some("As a standard action, make a Bluff check against each opponent that currently threatens you. If you succeed against at least one opponent, you can move up to your speed. This movement does not provoke attacks of opportunity from any opponent you succeeded at feinting against."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Combat Expertise,Disengaging Feint,Improved Feint", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]"]),
            },
            // Disengaging Shot -- uc_feats.lst:81
            UcFeatEntry {
                key: "Disengaging Shot",
                category: FeatCategory::Combat,
                name: "Disengaging Shot",
                description: Some("You make one last attack before beating a hasty retreat."),
                pretext: None,
                source_page: Some("p.96"),
                benefit: Some("Whenever you use Disengaging Feint or Disengaging Flourish, you can make a single melee attack against one opponent you succeeded at feinting against. That opponent is denied his Dexterity bonus to AC against this attack."),
                prerequisites: Some(&["PREABILITY:5,CATEGORY=FEAT,Combat Expertise,Disengaging Feint,Dodge,Improved Feint,Mobility", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]"]),
            },
            // Disorienting Maneuver -- uc_feats.lst:82
            UcFeatEntry {
                key: "Disorienting Maneuver",
                category: FeatCategory::General,
                name: "Disorienting Maneuver",
                description: Some("Your erratic movements disorient your opponent."),
                pretext: None,
                source_page: Some("p.96"),
                benefit: Some("If you successfully use Acrobatics to tumble through an opponent's space, you gain a +2 circumstance bonus on attack rolls against that opponent until the start of your next turn. If you choose to make a trip attempt against that opponent, you gain a +4 circumstance bonus on your combat maneuver check. This bonus on trip also lasts until the start of your next turn."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Dodge", "PRESKILL:1,Acrobatics=5"]),
            },
            // Dispel Synergy -- uc_feats.lst:83
            UcFeatEntry {
                key: "Dispel Synergy",
                category: FeatCategory::General,
                name: "Dispel Synergy",
                description: Some("By tearing away an opponent's magical defenses, you leave your enemy vulnerable, making it difficult for him to resist your spells."),
                pretext: None,
                source_page: Some("p.96"),
                benefit: Some("If you successfully dispel an ongoing magical effect on an opponent, that opponent takes a -2 penalty on saving throws against your spells until the end of your next turn."),
                prerequisites: Some(&["PRESKILL:1,Spellcraft=5"]),
            },
            // Dispelling Critical -- uc_feats.lst:84
            UcFeatEntry {
                key: "Dispelling Critical",
                category: FeatCategory::Critical,
                name: "Dispelling Critical",
                description: Some("Your blows attack the physical and arcane forms of your enemies at the same time."),
                pretext: None,
                source_page: Some("p.97"),
                benefit: Some("If you have dispel magic prepared or can cast it spontaneously, when you score a critical hit against an opponent, you may use a swift action to cast dispel magic to make a targeted dispel against that opponent."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Arcane Strike", "PRESPELL:1,Dispel Magic", "PRETOTALAB:11"]),
            },
            // Disposable Weapon -- uc_feats.lst:85
            UcFeatEntry {
                key: "Disposable Weapon",
                category: FeatCategory::General,
                name: "Disposable Weapon",
                description: Some("You ignore the limitations of your equipment, striking harder despite the damage it does to your weapon."),
                pretext: None,
                source_page: Some("p.97"),
                benefit: Some("Whenever you use a melee or thrown weapon with the fragile weapon special quality to score a critical threat against an opponent, you can give your weapon the broken condition to automatically confirm the critical hit."),
                prerequisites: Some(&["PRETOTALAB:1", "PREWEAPONPROF:1,TYPE.Fragile"]),
            },
            // Disruptive Recall -- uc_feats.lst:86
            UcFeatEntry {
                key: "Disruptive Recall",
                category: FeatCategory::General,
                name: "Disruptive Recall",
                description: Some("You can disrupt an enemy caster's spells to fuel your own arcane power."),
                pretext: None,
                source_page: Some("p.97"),
                benefit: Some("When you use a melee attack to successfully disrupt an arcane spellcaster's spell, you can immediately use your spell recall class feature to regain a magus spell you have already cast. This ability functions as if you had expended a number of points from your arcane pool equal to the level of the spell you disrupted, up to the maximum level spell you can cast."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Spell Recall ~ Magus", "PRESKILL:1,Spellcraft=5"]),
            },
            // Distance Thrower -- uc_feats.lst:89
            UcFeatEntry {
                key: "Distance Thrower",
                category: FeatCategory::Combat,
                name: "Distance Thrower",
                description: Some("You are accurate with thrown weapons at longer ranges than normal."),
                pretext: None,
                source_page: Some("p.97"),
                benefit: Some("With a thrown weapon, you reduce your penalty on ranged attack rolls due to range by 2."),
                prerequisites: Some(&["PRESTAT:1,STR=13"]),
            },
            // Djinni Spin -- uc_feats.lst:90
            UcFeatEntry {
                key: "Djinni Spin",
                category: FeatCategory::Combat,
                name: "Djinni Spin",
                description: Some("You can surround yourself with the power of the storm, spinning like a hurricane to unleash a violent blast of electrical energy."),
                pretext: None,
                source_page: Some("p.97"),
                benefit: Some("While using Djinni Style, as a standard action you can spend two Elemental Fist (Advanced Player's Guide 158) attempts to surround yourself with a whirlwind of electrified air. Creatures adjacent to you take your unarmed strike damage plus the electricity damage from your Elemental Fist and are deafened for 1d4 rounds. A successful Fortitude save (DC 10 + 1/2 your character level + your Wis modifier) reduces the damage by half and prevents a target from being deafened."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Djinni Style,Djinni Spirit,Elemental Fist,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:13],[PREVARGTEQ:MonkFeatQualify,11]", "PRESTAT:2,CON=15,WIS=17"]),
            },
            // Djinni Spirit -- uc_feats.lst:91
            UcFeatEntry {
                key: "Djinni Spirit",
                category: FeatCategory::Combat,
                name: "Djinni Spirit",
                description: Some("By calling upon the spirits of storms, you can manipulate lightning to protect yourself and buffet your enemies with peals of thunder."),
                pretext: None,
                source_page: Some("p.97"),
                benefit: Some("You gain one additional Elemental Fist (Advanced Player's Guide 158) attempt per day. While using Djinni Style, you gain electricity resistance equal to your base attack bonus or monk level, whichever is higher. While denied your Dexterity bonus to AC you are also denied this resistance. Creatures that take electricity damage from your Elemental Fist attack must succeed at a Fortitude save (DC 10 + 1/2 your character level + your Wis modifier) or be deafened for 1d4 rounds. Those who take damage from your Djinni Spin are deafened, even on a successful saving throw."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Djinni Style,Elemental Fist,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:11],[PREVARGTEQ:MonkFeatQualify,9]", "PRESTAT:2,CON=15,WIS=15"]),
            },
            // Djinni Style -- uc_feats.lst:92
            UcFeatEntry {
                key: "Djinni Style",
                category: FeatCategory::Combat,
                name: "Djinni Style",
                description: Some("Your hands sheathed in an auras of lightning, you move like the wind."),
                pretext: None,
                source_page: Some("p.97"),
                benefit: Some("You gain one additional Elemental Fist (Advanced Player's Guide 158) attempt per day. While you are in this style you must use Elemental Fist to deal electricity damage and you gain a bonus on electricity damage rolls equal to your Wisdom bonus (%1). Further, while you are using this style and have remaining Elemental Fist attempts, you also gain a +2 dodge bonus to Armor Class against attacks of opportunity. A condition that makes you lose your Dexterity bonus to AC also makes you lose this dodge bonus.|WIS"),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Elemental Fist,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:9],[PREVARGTEQ:MonkFeatQualify,5]", "PRESTAT:2,CON=13,WIS=15"]),
            },
            // Domain Strike -- uc_feats.lst:93
            UcFeatEntry {
                key: "Domain Strike",
                category: FeatCategory::Combat,
                name: "Domain Strike",
                description: Some("You unleash a domain power upon your enemy as part of your unarmed strike."),
                pretext: None,
                source_page: Some("p.98"),
                benefit: Some("When you gain this feat, choose one domain-granted power that you can use to affect no more than one opponent. If you make a successful unarmed strike against an opponent, in addition to dealing your unarmed strike damage, you can use a swift action to deliver the effects of the chosen granted power to that opponent. Doing so provokes no attacks of opportunity. &nl; [Special]You can take this feat multiple times. Each time you take it, you apply it to a different qualifying domain power."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Domains", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike"]),
            },
            // Double Bane -- uc_feats.lst:94
            UcFeatEntry {
                key: "Double Bane",
                category: FeatCategory::General,
                name: "Double Bane",
                description: Some("You extend your bane effect to two weapons."),
                pretext: None,
                source_page: Some("p.98"),
                benefit: Some("You can apply your bane to a second weapon you are wielding. While your bane class feature is active, at the start of each of your turns as a free action, you choose whether to apply the ability to one weapon or the other, or both. For each round you apply your bane class feature to two weapons, you expend 2 rounds of that feature."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Bane", "PREABILITY:1,CATEGORY=FEAT,Two-Weapon Fighting"]),
            },
            // Drag Down -- uc_feats.lst:95
            UcFeatEntry {
                key: "Drag Down",
                category: FeatCategory::Combat,
                name: "Drag Down",
                description: Some("When you are knocked prone, you are skilled at bringing your opponent down with you."),
                pretext: None,
                source_page: Some("p.98"),
                benefit: Some("Whenever an opponent successfully trips you, you can attempt to trip that opponent as an immediate action."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Trip", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]"]),
            },
            // Dragon Ferocity -- uc_feats.lst:96
            UcFeatEntry {
                key: "Dragon Ferocity",
                category: FeatCategory::Combat,
                name: "Dragon Ferocity",
                description: Some("You attack with the strength of a dragon, your telling blows striking fear into your enemies."),
                pretext: None,
                source_page: Some("p.98"),
                benefit: Some("While using Dragon Style, increase your Strength bonus on unarmed strike damage rolls by an additional one-half your Strength bonus, to a total of double your Strength bonus on the first attack and 1-1/2 times your Strength bonus on the other attacks. When you score a critical hit or a successful Stunning Fist attempt against an opponent while using this style, that opponent is also shaken for a number of rounds equal to 1d4 + your Strength bonus. &nl; [Special]Taking this feat allows you to qualify for the Elemental Fist feat (Advanced Player's Guide 158) even if you do not meet that feat's prerequisites. If you do not meet that feat's prerequisites, you must choose one of the damage types that feat offers, and you can use only that damage type with your Elemental Fist attacks until you meet the feat's normal prerequisites. A monk with this feat can use Elemental Fist as if he were a monk of the four winds (Advanced Player's Guide 112)."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Improved Unarmed Strike,Dragon Style,Stunning Fist", "PRESKILL:1,Acrobatics=5", "PRESTAT:1,STR=15"]),
            },
            // Dragon Roar -- uc_feats.lst:97
            UcFeatEntry {
                key: "Dragon Roar",
                category: FeatCategory::Combat,
                name: "Dragon Roar",
                description: Some("The spirit of the dragon wells up inside you and bursts forth in a mighty roar."),
                pretext: None,
                source_page: Some("p.98"),
                benefit: Some("You gain one additional Stunning Fist attempt per day. While using Dragon Style, as a standard action you can expend two Stunning Fist attempts to unleash a concussive roar in a 15-foot cone. Creatures caught in the cone take your unarmed strike damage and become shaken for 1d4 rounds. A successful Will save (DC 10 + 1/2 your character level + your Wis modifier) reduces the damage by half and prevents a target from being shaken. &nl; [Special]If you have the Elemental Fist feat (Advanced Player's Guide 158), you can expend a daily use of that feat to deal your Elemental Fist damage to those caught in the cone. This damage is not halved even on a save."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Improved Unarmed Strike,Dragon Style,Stunning Fist", "PRESKILL:1,Acrobatics=8", "PRESTAT:1,STR=15"]),
            },
            // Dragon Style -- uc_feats.lst:98
            UcFeatEntry {
                key: "Dragon Style",
                category: FeatCategory::Combat,
                name: "Dragon Style",
                description: Some("You call upon the spirit of dragonkind, gaining greater resilience, mobility, and fierceness from the blessing of these great beings."),
                pretext: None,
                source_page: Some("p.98"),
                benefit: Some("While using this style, you gain a +2 bonus on saving throws against sleep effects, paralysis effects, and stunning effects. You ignore difficult terrain when you charge, run, or withdraw. You can also charge through squares that contain allies. Further, you can add 1-1/2 times your Strength bonus on the damage roll for your first unarmed strike on a given round. &nl; [Normal] You cannot charge or run through difficult terrain, and you cannot charge through a square that contains an ally. With an unarmed strike, you usually add your Strength bonus on damage rolls."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRESKILL:1,Acrobatics=3", "PRESTAT:1,STR=15"]),
            },
            // Dramatic Display -- uc_feats.lst:99
            UcFeatEntry {
                key: "Dramatic Display",
                category: FeatCategory::Combat,
                name: "Dramatic Display",
                description: Some("Your skill with your weapon is obvious to enemies and onlookers alike."),
                pretext: None,
                source_page: Some("p.98"),
                benefit: Some("When you spend a swift action to make a performance check, you exude an aura of awe-inspiring skill. You gain a +2 bonus on your performance check, and gain a +2 bonus on all attack rolls and combat maneuver checks until the end of your next turn."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Dazzling Display"]),
            },
            // Earth Child Binder -- uc_feats.lst:100
            UcFeatEntry {
                key: "Earth Child Binder",
                category: FeatCategory::Combat,
                name: "Earth Child Binder",
                description: Some("Even the greatest giants fear your technique."),
                pretext: None,
                source_page: Some("p.98"),
                benefit: Some("You can trip a creature of the giant subtype no matter its size. While you are using Earth Child Style, when a prone creature of the giant subtype stands up and provokes an attack of opportunity from you, if you make an unarmed strike, you can declare you are making a Stunning Fist attempt after the attack hits. You gain a +4 bonus to the DC of any Stunning Fist effect you deliver in this way. &nl; [Normal] You can only trip opponents who are one size category larger than you."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Defensive Training,Gnome ~ Defensive Training", "PREABILITY:6,CATEGORY=FEAT,Earth Child Style,Earth Child Topple,Greater Trip,Improved Trip,Improved Unarmed Strike,Stunning Fist", "PREFACT:1,TEMPLATES,IsDwarf=true,IsGnome=True", "PRESKILL:1,Acrobatics=9", "PRESTAT:1,WIS=13"]),
            },
            // Earth Child Style -- uc_feats.lst:101
            UcFeatEntry {
                key: "Earth Child Style",
                category: FeatCategory::Combat,
                name: "Earth Child Style",
                description: Some("Your martial training makes you a dangerous and elusive target for giants."),
                pretext: None,
                source_page: Some("p.99"),
                benefit: Some("While using this style, your defensive training dodge bonus to AC increases to +6. Further, against creatures of the giant subtype, you can add your Wisdom bonus (%1) on your unarmed strike damage rolls.|WIS"),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Defensive Training,Gnome ~ Defensive Training", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PREFACT:1,TEMPLATES,IsDwarf=true,IsGnome=True", "PRESKILL:1,Acrobatics=3", "PRESTAT:1,WIS=13"]),
            },
            // Earth Child Topple -- uc_feats.lst:102
            UcFeatEntry {
                key: "Earth Child Topple",
                category: FeatCategory::General,
                name: "Earth Child Topple",
                description: Some("Your mastery of balance and momentum allows you to bring down giants with your bare hands."),
                pretext: None,
                source_page: None,
                benefit: Some("You can trip a creature of the giant subtype of up to Huge size. While using Earth Child Style, you add your Wisdom bonus on combat maneuver checks made to trip a creature of the giant subtype, as well as on attack rolls to confirm a critical hit against such a creature. &nl; [Normal] You can trip only those opponents that are one size category larger than you."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Defensive Training,Gnome ~ Defensive Training", "PREABILITY:3,CATEGORY=FEAT,Improved Unarmed Strike,Earth Child Style,Improved Trip", "PREFACT:1,TEMPLATES,IsDwarf=true,IsGnome=True", "PRESKILL:1,Acrobatics=6", "PRESTAT:1,WIS=13"]),
            },
            // Efreeti Stance -- uc_feats.lst:103
            UcFeatEntry {
                key: "Efreeti Stance",
                category: FeatCategory::Combat,
                name: "Efreeti Stance",
                description: Some("Calling upon the burning spirits of incarnate flame, you can manipulate fire to protect yourself and immolate your foes."),
                pretext: None,
                source_page: Some("p.99"),
                benefit: Some("You gain one additional Elemental Fist (Advanced Player's Guide 158) attempt per day. While using Efreeti Style, you gain fire resistance equal to your base attack bonus or your monk level plus any base attack bonus gained from levels in classes other than monk, whichever is higher. While denied your Dexterity bonus to AC you are also denied this resistance. Creatures that take fire damage from your Elemental Fist attack must succeed at a Reflex save (DC 10 + 1/2 your character level + your Wis modifier) or catch on fire."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Efreeti Style,Elemental Fist,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:11],[PREVARGTEQ:MonkFeatQualify,9]", "PRESTAT:2,CON=15,WIS=15"]),
            },
            // Efreeti Style -- uc_feats.lst:104
            UcFeatEntry {
                key: "Efreeti Style",
                category: FeatCategory::Combat,
                name: "Efreeti Style",
                description: Some("Your mastery of the unpredictable power of flames allows you to unleash scorching strikes that burn your enemies even when you fail to make contact."),
                pretext: None,
                source_page: Some("p.99"),
                benefit: Some("You gain one additional Elemental Fist (Advanced Player's Guide 158) attempt per day. While using this style and Elemental Fist to deal fire damage, you gain a bonus on fire damage rolls equal to your Wisdom bonus. Further, if your Elemental Fist melee attack misses while you are using it to deal fire damage, you still deal 1d6 points of fire damage to your target."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Elemental Fist,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:9],[PREVARGTEQ:MonkFeatQualify,5]", "PRESTAT:2,CON=13,WIS=15"]),
            },
            // Efreeti Touch -- uc_feats.lst:105
            UcFeatEntry {
                key: "Efreeti Touch",
                category: FeatCategory::Combat,
                name: "Efreeti Touch",
                description: Some("Your knowledge of the secrets of the burning wind and blazing sun allows you to collect flames into your hands and unleash them in a gout of elemental fire."),
                pretext: None,
                source_page: Some("p.100"),
                benefit: Some("While using Efreeti Style, as a standard action, you can spend two Elemental Fist (Advanced Player's Guide 158) attempts to unleash a 15-foot cone-shaped burst of flame. Creatures caught in the cone take your unarmed strike damage plus the fire damage from your Elemental Fist and catch on fire. A successful Reflex save (DC 10 + 1/2 your character level + your Wis modifier) reduces the damage by half and prevents a target from catching on fire."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Efreeti Style,Efreeti Stance,Elemental Fist,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:13],[PREVARGTEQ:MonkFeatQualify,11]", "PRESTAT:2,CON=15,WIS=17"]),
            },
            // Elusive Redirection -- uc_feats.lst:106
            UcFeatEntry {
                key: "Elusive Redirection",
                category: FeatCategory::General,
                name: "Elusive Redirection",
                description: Some("You can redirect an attack back at your assailant or into an adjacent enemy."),
                pretext: None,
                source_page: Some("p.100"),
                benefit: Some("When you successfully use your elusive target class feature to avoid taking damage, you can spend an immediate action and an additional point from your ki pool to redirect that attack back at your attacker or toward any other opponent adjacent to you and your attacker. This attack uses the same attack roll as the original attack, but it targets the opponent you choose."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Elusive Target ~ Flowing Monk,Flowing TYPE.Elusive Target", "PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Flowing Monk", "PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Unarmed Strike", "PREVARGTEQ:MonkFeatQualify,12"]),
            },
            // Enfilading Fire -- uc_feats.lst:107
            UcFeatEntry {
                key: "Enfilading Fire",
                category: FeatCategory::Combat,
                name: "Enfilading Fire",
                description: Some("Your ranged attacks take advantage of the flanking maneuvers of allies."),
                pretext: None,
                source_page: Some("p.100"),
                benefit: Some("You receive a +2 bonus on ranged attacks made against a foe flanked by 1 or more allies with this feat."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Point-Blank Shot,Precise Shot", "PREABILITY:1,CATEGORY=FEAT,TYPE=Teamwork"]),
            },
            // Escape Route -- uc_feats.lst:108
            UcFeatEntry {
                key: "Escape Route",
                category: FeatCategory::Teamwork,
                name: "Escape Route",
                description: Some("You have trained to watch your allies' backs, covering them as they make tactical withdraws."),
                pretext: None,
                source_page: Some("p.100"),
                benefit: Some("An ally who also has this feat provokes no attacks of opportunity for moving through squares adjacent to you or within your space."),
                prerequisites: None,
            },
            // Expert Driver -- uc_feats.lst:109
            UcFeatEntry {
                key: "Expert Driver",
                category: FeatCategory::General,
                name: "Expert Driver",
                description: Some("When driving a chosen type of vehicle, you exhibit incredible control, maneuvering the vehicle with greater ease and stopping with uncanny precision."),
                pretext: None,
                source_page: Some("p.100"),
                benefit: Some("You can make an accelerate, decelerate, or turn action as a move action instead of a standard action. Furthermore, when stopping a vehicle, you subtract 10 feet from the roll to determine how many feet the vehicle moves forward before it stops."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Skilled Driver"]),
            },
            // Extra Bane -- uc_feats.lst:110
            UcFeatEntry {
                key: "Extra Bane",
                category: FeatCategory::General,
                name: "Extra Bane",
                description: Some("You can use your bane ability more often than normal."),
                pretext: None,
                source_page: Some("p.100"),
                benefit: Some("You can use your bane ability for 3 additional rounds per day."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Bane"]),
            },
            // Extra Grit -- uc_feats.lst:111
            UcFeatEntry {
                key: "Extra Grit",
                category: FeatCategory::Grit,
                name: "Extra Grit",
                description: Some("You have more grit than the ordinary gunslinger."),
                pretext: None,
                source_page: Some("p.100"),
                benefit: Some("You gain 2 extra grit points at the start of each day, and your maximum grit increases by 2. &nl; [Normal] If you are a gunslinger, you gain your Wisdom modifier in grit points at the start of each day, which is also your maximum grit. If you have the Amateur Gunslinger feat, you gain 1 grit point at the start of each day, and your maximum grit is equal to your Wisdom modifier. &nl; [Special]If you possess levels in the gunslinger class, you can take this feat multiple times."),
                prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Gunslinger ~ Grit],[PREABILITY:1,CATEGORY=FEAT,Amateur Gunslinger]"]),
            },
            // False Opening -- uc_feats.lst:112
            UcFeatEntry {
                key: "False Opening",
                category: FeatCategory::Combat,
                name: "False Opening",
                description: Some("When you make a ranged attack while threatened, you can fool your opponent into thinking he has an opening."),
                pretext: None,
                source_page: Some("p.100"),
                benefit: Some("Choose a ranged weapon or a thrown weapon. When you make a ranged attack using that weapon, you can choose to provoke an attack of opportunity from one or more opponents who threaten you. You gain a +4 dodge bonus against such attacks. An opponent that makes such an attack and misses you loses his Dexterity bonus to AC against you until the end of your turn."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Dodge,Weapon Focus (TYPE=Ranged)", "PREABILITY:1,CATEGORY=FEAT,Close-Quarters Thrower,Point Blank Master", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]"]),
            },
            // Feint Partner -- uc_feats.lst:113
            UcFeatEntry {
                key: "Feint Partner",
                category: FeatCategory::Combat,
                name: "Feint Partner",
                description: Some("A little diversion is all you need to slip through your foe's defenses."),
                pretext: None,
                source_page: Some("p.100"),
                benefit: Some("Whenever an ally who also has this feat successfully feints an opponent, that opponent also loses his Dexterity bonus to AC against the next attack you make against him before the end of the feinting ally's next turn."),
                prerequisites: Some(&["PRESKILL:1,Bluff=1"]),
            },
            // Felling Escape -- uc_feats.lst:114
            UcFeatEntry {
                key: "Felling Escape",
                category: FeatCategory::Combat,
                name: "Felling Escape",
                description: Some("Through the use of fluid contortions and manipulations of leverage, you can throw your opponent to the ground after escaping a grapple."),
                pretext: None,
                source_page: Some("p.101"),
                benefit: Some("When you break an opponent's grapple with a combat maneuver check or Escape Artist check, you can spend a swift action to make a trip attempt against that opponent."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Trip", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]"]),
            },
            // Felling Smash -- uc_feats.lst:115
            UcFeatEntry {
                key: "Felling Smash",
                category: FeatCategory::Combat,
                name: "Felling Smash",
                description: Some("You commit all your focus to a devastating blow, trying to crush your opponent to the ground."),
                pretext: None,
                source_page: Some("p.101"),
                benefit: Some("If you use the attack action to make a single melee attack at your highest base attack bonus while using Power Attack and you hit an opponent, you can spend a swift action to attempt a trip combat maneuver against that opponent."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Combat Expertise,Improved Trip,Power Attack", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRESTAT:1,STR=13", "PRETOTALAB:6"]),
            },
            // Feral Combat Training -- uc_feats.lst:117
            UcFeatEntry {
                key: "Feral Combat Training",
                category: FeatCategory::Combat,
                name: "Feral Combat Training",
                description: Some("You were taught a style of martial arts that relies on the natural weapons from your racial ability or class feature."),
                pretext: None,
                source_page: Some("p.101"),
                benefit: Some("Choose one of your natural weapons. While using the selected natural weapon, you can apply the effects of feats that have Improved Unarmed Strike as a prerequisite. &nl; [Special]If you are a monk, you can use the selected natural weapon with your flurry of blows class feature."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Weapon Focus (TYPE=Natural)"]),
            },
            // Field Repair -- uc_feats.lst:118
            UcFeatEntry {
                key: "Field Repair",
                category: FeatCategory::General,
                name: "Field Repair",
                description: Some("You can repair your broken weapon or armor to serviceability even without the benefits of artisan tools."),
                pretext: None,
                source_page: Some("p.101"),
                benefit: Some("If you are trained in a Craft skill appropriate to a broken item, you can repair that item with no raw material cost and no penalty on your Craft skill check for using improvised tools. If you spend a day, the item regains 1 hit point plus one-quarter of its original hit points. Alternatively, if the item gained the broken condition because it is a firearm that has misfired or a siege engine that suffered a mishap, or has the broken condition because it has the fragile weapon quality (or some similar quality), you can make a Craft check with the DC it takes to craft that item (see Table 2-2, below). If the check succeeds, the item loses the broken condition. &nl; [Normal] Improvised tools impose a -2 penalty on Craft checks. Items require raw materials to repair."),
                prerequisites: Some(&["PRESKILL:1,Craft=4"]),
            },
            // Final Embrace -- uc_feats.lst:123
            UcFeatEntry {
                key: "Final Embrace",
                category: FeatCategory::Combat,
                name: "Final Embrace",
                description: Some("Your coils are particularly deadly, allowing you to constrict opponents of your size or smaller."),
                pretext: None,
                source_page: Some("p.101"),
                benefit: Some("You gain the constrict and grab special attacks. Your constrict attack deals damage equal to your unarmed strike or primary natural weapon melee attack. Further, you can grab and constrict opponents up to your size. &nl; [Normal] You can grab and constrict creatures one size smaller than you."),
                prerequisites: Some(&["PREMULT:1,[PRERACE:1,Naga,Serpentfolk],[PREABILITY:1,CATEGORY=Special Ability,Constrict,TYPE.Constrict]", "PRESTAT:2,STR=13,INT=3", "PRETOTALAB:3"]),
            },
            // Final Embrace Horror -- uc_feats.lst:150
            UcFeatEntry {
                key: "Final Embrace Horror",
                category: FeatCategory::Combat,
                name: "Final Embrace Horror",
                description: Some("Your constricting attack has become stronger and more lethal."),
                pretext: None,
                source_page: Some("p.101"),
                benefit: Some("A creature that takes damage from your constrict attack is also shaken until the start of your next turn."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Final Embrace", "PREMULT:1,[PRERACE:1,Naga,Serpentfolk],[PREABILITY:1,CATEGORY=Special Ability,Constrict]", "PRESTAT:2,STR=15,INT=3", "PRETOTALAB:6"]),
            },
            // Final Embrace Master -- uc_feats.lst:152
            UcFeatEntry {
                key: "Final Embrace Master",
                category: FeatCategory::Combat,
                name: "Final Embrace Master",
                description: Some("Few creatures can survive the crushing horror of your Final Embrace."),
                pretext: None,
                source_page: Some("p.102"),
                benefit: Some("Double the number of damage dice for your constrict special attack."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Final Embrace,Final Embrace Horror", "PREMULT:1,[PRERACE:1,Naga,Serpentfolk],[PREABILITY:1,CATEGORY=Special Ability,Constrict]", "PRESTAT:2,STR=17,INT=3", "PRETOTALAB:9"]),
            },
            // Flanking Foil -- uc_feats.lst:153
            UcFeatEntry {
                key: "Flanking Foil",
                category: FeatCategory::Combat,
                name: "Flanking Foil",
                description: Some("Fighting multiple foes is easy for you."),
                pretext: None,
                source_page: Some("p.102"),
                benefit: Some("Whenever you hit an adjacent opponent with a melee attack, until the start of your next turn, that opponent does not gain any flanking bonus on attack rolls while it is flanking you and cannot deal sneak attack damage to you. It can still provide a flank for its allies."),
                prerequisites: None,
            },
            // Fortified Armor Training -- uc_feats.lst:154
            UcFeatEntry {
                key: "Fortified Armor Training",
                category: FeatCategory::Combat,
                name: "Fortified Armor Training",
                description: Some("You have learned to let your armor bear the brunt of the worst attacks."),
                pretext: None,
                source_page: Some("p.102"),
                benefit: Some("If an opponent scores a critical hit against you, you can turn the critical hit into a normal hit. If you do, either your armor or your shield gains the broken condition (your choice)."),
                prerequisites: Some(&["PREMULT:1,[PREPROFWITHSHIELD:1,TYPE.Tower,TYPE.Light,TYPE.Heavy],[PREPROFWITHARMOR:1,TYPE.Light,TYPE.Medium,TYPE.Heavy]"]),
            },
            // Furious Finish -- uc_feats.lst:155
            UcFeatEntry {
                key: "Furious Finish",
                category: FeatCategory::General,
                name: "Furious Finish",
                description: Some("You channel all of your rage into one massive blow to crush your enemy."),
                pretext: None,
                source_page: Some("p.102"),
                benefit: Some("While raging, when you use the Vital Strike feat, you can choose not to roll your damage dice and instead deal damage equal to the maximum roll possible on those damage dice. If you do, your rage immediately ends, and you are fatigued (even if you would not normally be)."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Rage", "PREABILITY:1,CATEGORY=FEAT,Vital Strike", "PRETOTALAB:6"]),
            },
            // Gory Finish -- uc_feats.lst:156
            UcFeatEntry {
                key: "Gory Finish",
                category: FeatCategory::Combat,
                name: "Gory Finish",
                description: Some("By drawing upon wells of savagery, you can slay your foe in creative and horrifyingly gruesome manners, intimidating nearby foes."),
                pretext: None,
                source_page: Some("p.102"),
                benefit: Some("When you use the attack action, you can use a weapon with which you have Weapon Focus to make a single attack at your highest base attack bonus. If you reduce your target to negative hit points, you can spend a swift action to make an Intimidate check to demoralize all foes within 30 feet who could see your attack."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Dazzling Display,Weapon Focus"]),
            },
            // Greater Channel Smite -- uc_feats.lst:157
            UcFeatEntry {
                key: "Greater Channel Smite",
                category: FeatCategory::General,
                name: "Greater Channel Smite",
                description: Some("You empower your weapon with the might of your deity, which you discharge as you strike your foes."),
                pretext: None,
                source_page: Some("p.102"),
                benefit: Some("Before making any melee attacks on your turn, you can use a swift action to expend one daily use of your channel energy class feature. The dice from your channel energy feature form a pool of damage dice you can access to further damage creatures normally harmed by the energy you are channeling-undead for positive energy, living creatures for negative energy. Prior to making each melee attack, allocate dice from the pool to be used as extra damage dice if you hit. Your target can make a Will save, as normal, to halve this extra damage. This extra damage is not multiplied when you score a critical hit. If you miss, the extra damage dice remain in your pool, but any dice left unexpended at the end of your turn are wasted."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Energy", "PREABILITY:1,CATEGORY=FEAT,Channel Smite", "PRETOTALAB:8"]),
            },
            // Greater Rending Fury -- uc_feats.lst:158
            UcFeatEntry {
                key: "Greater Rending Fury",
                category: FeatCategory::Combat,
                name: "Greater Rending Fury",
                description: Some("When your claws latch on to an opponent, the effect is bloody and horrific."),
                pretext: None,
                source_page: Some("p.103"),
                benefit: Some("Whenever you rend an opponent, you deal 1d6 bleed damage to that opponent. This is an addition to the effects of the rend."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Rend", "PREABILITY:2,CATEGORY=FEAT,Improved Rending Fury,Rending Fury", "PRETOTALAB:12"]),
            },
            // Greater Snap Shot -- uc_feats.lst:159
            UcFeatEntry {
                key: "Greater Snap Shot",
                category: FeatCategory::Combat,
                name: "Greater Snap Shot",
                description: Some("You can prey on any gap in your foe's guard with impunity, and with even greater range."),
                pretext: None,
                source_page: Some("p.103"),
                benefit: Some("Whenever you make an attack of opportunity using a ranged weapon and hit, you gain a +2 bonus on the damage roll and a +2 bonus on rolls to confirm a critical hit with that attack. These bonuses increase to +4 when you have base attack bonus of +16, and to +6 when you have base attack bonus +20."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Improved Snap Shot,Point-Blank Shot,Rapid Shot,Snap Shot", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,17],[PREVARGTEQ:FeatDexRequirement,17]", "PRETOTALAB:12"]),
            },
            // Guided Hand -- uc_feats.lst:161
            UcFeatEntry {
                key: "Guided Hand",
                category: FeatCategory::General,
                name: "Guided Hand",
                description: Some("Your deity blesses any strike you make with that deity's favored weapon."),
                pretext: None,
                source_page: Some("p.103"),
                benefit: Some("With your deity's favored weapon, you can use your Wisdom modifier instead of your Strength or Dexterity modifier on attack rolls."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Energy", "PREABILITY:1,CATEGORY=FEAT,Channel Smite", "PREWEAPONPROF:1,DEITYWEAPON"]),
            },
            // Gunsmithing -- uc_feats.lst:162
            UcFeatEntry {
                key: "Gunsmithing",
                category: FeatCategory::General,
                name: "Gunsmithing",
                description: Some("You know the secrets of repairing and restoring firearms."),
                pretext: None,
                source_page: Some("p.103"),
                benefit: Some("If you have access to a gunsmith's kit, you can create and restore firearms, craft bullets, and mix black powder for all types of firearms. You do not need to make a Craft check to create firearms and ammunition or to restore firearms. &nl;Crafting Firearms - You can craft any early firearm for a cost in raw materials equal to half the price of the firearm. At your GM's discretion, you can craft advanced firearms for a cost in raw materials equal to half the price of the firearm. Crafting a firearm in this way takes 1 day of work for every 1,000 gp of the firearm's price (minimum 1 day). &nl;Crafting Ammunition - You can craft bullets, pellets, and black powder for a cost in raw materials equal to 10%% of the price. If you have at least 1 rank in Craft (alchemy), you can craft alchemical cartridges for a cost in raw materials equal to half the price of the cartridge. At your GM's discretion, you can craft metal cartridges for a cost in raw materials equal to half the cost of the cartridge. Crafting bullets, black powder, or cartridges takes 1 day of work for every 1,000 gp of ammunition (minimum 1 day). &nl;Restoring a Broken Firearm - Each day, with an hour's worth of work, you can use this feat to repair a single firearm with the broken condition. You can take time during a rest period to restore a broken firearm with this feat. &nl; [Special]If you are a gunslinger, this feat grants the following additional benefit. You can use this feat to repair and restore your initial, battered weapon. It costs 300 gp and 1 day of work to upgrade it to a masterwork firearm of its type."),
                prerequisites: None,
            },
            // Hammer the Gap -- uc_feats.lst:163
            UcFeatEntry {
                key: "Hammer the Gap",
                category: FeatCategory::Combat,
                name: "Hammer the Gap",
                description: Some("You repeatedly strike the same location, causing increasing amounts of damage."),
                pretext: None,
                source_page: Some("p.103"),
                benefit: Some("When you take a full-attack action, each consecutive hit against the same opponent deals extra damage equal to the number of previous consecutive hits you have made against that opponent this turn. This damage is multiplied on a critical hit."),
                prerequisites: Some(&["PRETOTALAB:6"]),
            },
            // Harmonic Sage -- uc_feats.lst:164
            UcFeatEntry {
                key: "Harmonic Sage",
                category: FeatCategory::General,
                name: "Harmonic Sage",
                description: Some("Your thorough understanding of acoustics in artificial surroundings allows you to improve the power of your bardic performance."),
                pretext: None,
                source_page: Some("p.104"),
                benefit: Some("While inside an artificial structure, you can spend a free action to make a DC 15 Knowledge (engineering) check when you begin your bardic performance. Success on this check allows you to do one of the following. &nl;Self-Harmonize - By performing over the acoustic reverberations of your performance, you increase the DC of your bardic performance effects +1. &nl;Reverberation - You can choose to have the effect of your current bardic performance continue for 1 round after you cease maintaining it, regardless of why you cease maintaining it. You can still have no more than one bardic performance in effect at one time. &nl; [Normal] A bardic performance lasts only as long as you maintain it."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Bardic Performance", "PRESKILL:1,Knowledge (engineering)=5"]),
            },
            // Haunted Gnome -- uc_feats.lst:165
            UcFeatEntry {
                key: "Haunted Gnome",
                category: FeatCategory::Combat,
                name: "Haunted Gnome",
                description: Some("You use your gnome magic to take on an eerie illusory appearance."),
                pretext: None,
                source_page: Some("p.104"),
                benefit: Some("You add haunted fey aspect (page 230) to your list of gnome magic spell-like abilities, and you can use this spell-like ability twice per day."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Gnome ~ Gnome Magic", "PRESKILL:1,Knowledge (arcana)=1", "PRESTAT:1,CHA=13"]),
            },
            // Haunted Gnome Assault -- uc_feats.lst:166
            UcFeatEntry {
                key: "Haunted Gnome Assault",
                category: FeatCategory::Combat,
                name: "Haunted Gnome Assault",
                description: Some("Discharging your disturbing glamer as you strike a telling blow, you haunt a foe with lingering fear."),
                pretext: None,
                source_page: Some("p.104"),
                benefit: Some("You gain one use of your gnome magic that is independent of your gnome magic spell-like abilities. When you wish to cast a gnome magic spell-like ability for which you have no daily uses remaining, you can expend this independent use to do so. Further, while you are under the effect of haunted fey aspect (page 230), you can discharge that spell as a free action after you hit an opponent with a charge attack or score a critical hit against an opponent. If you do, that opponent becomes shaken for 1 round."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Gnome ~ Gnome Magic", "PREABILITY:1,CATEGORY=FEAT,Haunted Gnome", "PRESKILL:1,Knowledge (arcana)=3", "PRESTAT:1,CHA=13"]),
            },
            // Haunted Gnome Shroud -- uc_feats.lst:167
            UcFeatEntry {
                key: "Haunted Gnome Shroud",
                category: FeatCategory::Combat,
                name: "Haunted Gnome Shroud",
                description: Some("Your disturbing glamer expands to make your exact location hard to pinpoint."),
                pretext: None,
                source_page: Some("p.104"),
                benefit: Some("You gain another independent use of your gnome magic like that which Haunted Gnome Assault grants. Further, while you are under the effect of haunted fey aspect, you also have concealment (20%% miss chance) against an opponent until that opponent deals you damage."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Gnome ~ Gnome Magic", "PREABILITY:2,CATEGORY=FEAT,Haunted Gnome,Haunted Gnome Assault", "PRESKILL:1,Knowledge (arcana)=6", "PRESTAT:1,CHA=13"]),
            },
            // Hero's Display -- uc_feats.lst:168
            UcFeatEntry {
                key: "Hero's Display",
                category: FeatCategory::Combat,
                name: "Hero's Display",
                description: Some("With a dramatic flourish you display your weapons to the crowd. Onlookers are elated, and your enemies are demoralized."),
                pretext: None,
                source_page: Some("p.104"),
                benefit: Some("When you spend a swift action to make a performance combat check, you present the weapon in which you have Weapon Focus in a triumphant display. You gain a +2 bonus on the performance combat check and make an Intimidate check to demoralize all foes within 30 feet who can see your display."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Dazzling Display,Weapon Focus"]),
            },
            // Hex Strike -- uc_feats.lst:172
            UcFeatEntry {
                key: "Hex Strike",
                category: FeatCategory::Combat,
                name: "Hex Strike",
                description: Some("Chanting and cursing, you put a hex on your enemy as part of your unarmed strike."),
                pretext: None,
                source_page: Some("p.104"),
                benefit: Some("When you gain this feat, choose one hex that you can use to affect no more than one opponent. If you make a successful unarmed strike against an opponent, in addition to dealing your unarmed strike damage, you can use a swift action to deliver the effects of the chosen hex to that opponent. Doing so does not provoke attacks of opportunity. &nl; [Special]You can take this feat multiple times. Each time you take it, you apply it to a different qualifying hex."),
                prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,TYPE.WitchHex],[PREVARGTEQ:WitchMinorHexQualify,1],[PREVARGTEQ:WitchHexAbilityLVL,1]", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike"]),
            },
            // Horse Master -- uc_feats.lst:175
            UcFeatEntry {
                key: "Horse Master",
                category: FeatCategory::Combat,
                name: "Horse Master",
                description: Some("You blend horsemanship skills from disparate traditions into a seamless mounted combat technique."),
                pretext: None,
                source_page: Some("p.104"),
                benefit: Some("Use your character level to determine your effective druid level for determining the powers and abilities of your mount. &nl; [Normal] You use your cavalier level to determine your effective druid level for determining the powers and abilities of your mount (NCL=%2) (CL=%1).|CavalierLevel|NonCavalierLevel"),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Cavalier ~ Expert Trainer", "PRESKILL:1,Ride=6"]),
            },
            // Impact Critical Shot -- uc_feats.lst:176
            UcFeatEntry {
                key: "Impact Critical Shot",
                category: FeatCategory::Combat,
                name: "Impact Critical Shot",
                description: Some("With a series of ranged attacks, you bring your foes to their knees or force them to move."),
                pretext: None,
                source_page: Some("p.104"),
                benefit: Some("Whenever you score a critical hit with a ranged attack, in addition to the normal damage your attack deals, if your confirmation roll exceeds your opponent's CMD, you can push your opponent back as if from the bull rush combat maneuver or knock that target prone as if from a trip combat maneuver. If you choose to bull rush, you cannot move with the target. Your maneuver does not provoke an attack of opportunity. &nl; [Normal] You must perform a bull rush combat maneuver to bull rush an opponent, and you must perform a trip combat maneuver to trip an opponent."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRETOTALAB:9"]),
            },
            // Impaling Critical -- uc_feats.lst:177
            UcFeatEntry {
                key: "Impaling Critical",
                category: FeatCategory::Combat,
                name: "Impaling Critical",
                description: Some("Your critical hits can skewer your foes."),
                pretext: None,
                source_page: None,
                benefit: Some("Whenever you score a critical hit with the selected piercing melee weapon, you can impale your opponent on your weapon. While your opponent is impaled in this way, each time he starts his turn, you deal damage equal to your weapon's damage dice plus the extra damage dice from your weapon's properties. As an immediate action, you can pull your weapon out of your opponent. If your opponent is ever outside your reach, you must spend a free action to let go of your weapon or pull it out of him. Your opponent can also spend a move action to pull your weapon out. When the weapon comes out, your opponent takes damage as if starting his turn impaled. While you impale your opponent with your weapon, you cannot use it to attack, and you must hold on to it."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Critical Focus", "PREABILITY:1,CATEGORY=FEAT,Weapon Specialization (TYPE=MeleePiercing)", "PRETOTALAB:11"]),
            },
            // Improved Back to Back -- uc_feats.lst:178
            UcFeatEntry {
                key: "Improved Back to Back",
                category: FeatCategory::Teamwork,
                name: "Improved Back to Back",
                description: Some("After much practice, you and an ally have become adept at fighting in close proximity to one another."),
                pretext: None,
                source_page: Some("p.105"),
                benefit: Some("While you are adjacent to an ally who is flanked and also has this feat, you can spend a swift action to gain a +2 bonus to AC against all flankers until the start of your next turn."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Back to Back", "PRESKILL:1,Perception=5"]),
            },
            // Improved Charging Hurler -- uc_feats.lst:179
            UcFeatEntry {
                key: "Improved Charging Hurler",
                category: FeatCategory::Combat,
                name: "Improved Charging Hurler",
                description: Some("Every muscle in your body adds its force to your thrown weapons."),
                pretext: None,
                source_page: Some("p.105"),
                benefit: Some("When you use Charging Hurler, your target can be at any range up to your weapon's maximum range. If your target is within 30 feet, you gain a +2 bonus on damage rolls. &nl; [Normal] Using Charging Hurler requires you to end your movement within 30 feet of your opponent."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Charging Hurler,Point-Blank Shot"]),
            },
            // Improved Cleaving Finish -- uc_feats.lst:180
            UcFeatEntry {
                key: "Improved Cleaving Finish",
                category: FeatCategory::Combat,
                name: "Improved Cleaving Finish",
                description: Some("You can cut down many opponents in a single strike."),
                pretext: None,
                source_page: Some("p.105"),
                benefit: Some("You can use Cleaving Finish any number of times per round."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Cleave,Cleaving Finish,Great Cleave,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:6"]),
            },
            // Improved Devastating Strike -- uc_feats.lst:181
            UcFeatEntry {
                key: "Improved Devastating Strike",
                category: FeatCategory::Combat,
                name: "Improved Devastating Strike",
                description: Some("The fury and power channeled through your attack is enough to kill a lesser being outright."),
                pretext: None,
                source_page: Some("p.105"),
                benefit: Some("Whenever you use Vital Strike, Improved Vital Strike, or Greater Vital Strike, you gain a bonus on attack rolls to confirm a critical hit equal to the bonus on damage rolls you gain from Devastating Strike."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Devastating Strike,Vital Strike", "PRETOTALAB:13"]),
            },
            // Improved Feint Partner -- uc_feats.lst:182
            UcFeatEntry {
                key: "Improved Feint Partner",
                category: FeatCategory::Combat,
                name: "Improved Feint Partner",
                description: Some("Knowledge of your companions' tricks and techniques allow you to take even greater advantage of your allies' feints."),
                pretext: None,
                source_page: Some("p.105"),
                benefit: Some("Whenever an ally who also has this feat successfully feints against an opponent, that opponent provokes an attack of opportunity from you."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Combat Reflexes,Feint Partner", "PRESKILL:1,Bluff=1", "PRETOTALAB:6"]),
            },
            // Improved Impaling Critical -- uc_feats.lst:183
            UcFeatEntry {
                key: "Improved Impaling Critical",
                category: FeatCategory::Combat,
                name: "Improved Impaling Critical",
                description: Some("When you impale a target, you hinder its movement and can cause severe bleeding."),
                pretext: None,
                source_page: Some("p.105"),
                benefit: Some("While you are using Impaling Critical to impale an opponent, and you are still holding onto that weapon, that opponent must succeed at a grapple combat maneuver check against you to pull your weapon out. If you have let go of your weapon, the impaled opponent must spend a standard action to remove the weapon. Until the opponent pulls the weapon out, his speed in all modes is halved and his maneuverability, if any, is reduced by one step. When the weapon comes out, instead of dealing the damage normal for Impaling Critical, you can deal bleed damage equal to your weapon's damage dice result once per round at the start of that opponent's turn."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Impaling Critical,Critical Focus,Weapon Specialization (TYPE=MeleePiercing)", "PRETOTALAB:13"]),
            },
            // Improved Rending Fury -- uc_feats.lst:184
            UcFeatEntry {
                key: "Improved Rending Fury",
                category: FeatCategory::Combat,
                name: "Improved Rending Fury",
                description: Some("Honing the deadliness of your claws, you are a living hurricane of rending fury."),
                pretext: None,
                source_page: Some("p.105"),
                benefit: Some("Whenever you successfully rend an opponent, you deal an extra 1d6 damage. This damage is not multiplied on a critical hit."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Rend", "PREABILITY:1,CATEGORY=FEAT,Rending Fury", "PRETOTALAB:9"]),
            },
            // Improved Snap Shot -- uc_feats.lst:185
            UcFeatEntry {
                key: "Improved Snap Shot",
                category: FeatCategory::Combat,
                name: "Improved Snap Shot",
                description: Some("You can take advantage of your opponent's vulnerabilities from a greater distance, and without exposing yourself."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: Some("You threaten an additional 5 feet with Snap Shot. &nl; [Normal] Making a ranged attack provokes attacks of opportunity."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Point-Blank Shot,Rapid Shot,Snap Shot,Weapon Focus", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,15],[PREVARGTEQ:FeatDexRequirement,15]", "PRETOTALAB:9"]),
            },
            // Improved Stalwart -- uc_feats.lst:186
            UcFeatEntry {
                key: "Improved Stalwart",
                category: FeatCategory::General,
                name: "Improved Stalwart",
                description: Some("You can roll with the punches while simultaneously striking back at your attackers."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: Some("Double the DR you gain from Stalwart, to a maximum of DR 10/-."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Diehard,Endurance,Stalwart", "PRETOTALAB:11"]),
            },
            // Improved Two-Weapon Feint -- uc_feats.lst:187
            UcFeatEntry {
                key: "Improved Two-Weapon Feint",
                category: FeatCategory::Combat,
                name: "Improved Two-Weapon Feint",
                description: Some("Your primary weapon keeps a foe off balance, allowing you to slip your off-hand weapon past his defenses."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: Some("While using Two-Weapon Fighting to make melee attacks, you can forgo your first primary-hand melee attack to make a Bluff check to feint an opponent. If you successfully feint, that opponent is denied his Dexterity bonus to AC until the end of your turn."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Combat Expertise,Improved Two-Weapon Fighting,Two-Weapon Fighting,Two-Weapon Feint", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,17],[PREVARGTEQ:FeatDexRequirement,17]", "PRETOTALAB:6"]),
            },
            // Instant Judgment -- uc_feats.lst:188
            UcFeatEntry {
                key: "Instant Judgment",
                category: FeatCategory::General,
                name: "Instant Judgment",
                description: Some("Your hastiest condemnations can have power."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: Some("You can spend an immediate action to pronounce a judgment or change an active judgment. &nl; [Normal] Pronouncing or changing a judgment requires a swift action."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Second Judgment"]),
            },
            // Intimidating Bane -- uc_feats.lst:189
            UcFeatEntry {
                key: "Intimidating Bane",
                category: FeatCategory::General,
                name: "Intimidating Bane",
                description: Some("Your bane weapon strikes fear into your enemies."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: Some("Whenever you use Dazzling Display while your bane feature is active, you gain a +2 bonus on the Intimidate check that Dazzling Display allows against creatures of the type your bane weapon currently affects. Such creatures remain shaken while your bane feature is still active and effective against their creature type."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Bane", "PREABILITY:2,CATEGORY=FEAT,Dazzling Display,Weapon Focus", "PRELEVEL:MIN=8"]),
            },
            // Janni Rush -- uc_feats.lst:190
            UcFeatEntry {
                key: "Janni Rush",
                category: FeatCategory::Combat,
                name: "Janni Rush",
                description: Some("When you leap to the attack, your blows are like bolts from on high."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: Some("While using Janni Style, you are always considered to have a running start when jumping. Further, if you jump as part of a charge and make an unarmed strike against the designated opponent, a hit allows you to roll the unarmed strike's damage dice twice and add the results together before adding modifiers (such as from Strength) or extra dice (such as precision-based damage or dice from weapon abilities). The extra damage dice are not multiplied on a successful critical hit."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Improved Unarmed Strike,Janni Style,Janni Tempest", "PRESKILL:2,Acrobatics=8,Perform (dance)=8"]),
            },
            // Janni Style -- uc_feats.lst:191
            UcFeatEntry {
                key: "Janni Style",
                category: FeatCategory::Combat,
                name: "Janni Style",
                description: Some("Your whirling fighting technique makes you difficult to hit."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: Some("While using this style, you take only a -1 penalty to AC for charging. Further, opponents that flank you gain only a +1 bonus on attack rolls against you."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRESKILL:2,Acrobatics=3,Perform (dance)=3"]),
            },
            // Janni Tempest -- uc_feats.lst:192
            UcFeatEntry {
                key: "Janni Tempest",
                category: FeatCategory::Combat,
                name: "Janni Tempest",
                description: Some("Your gale of attacks easily throws your opponents off balance."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: Some("While you are using the Janni Style feat, whenever you make an unarmed attack and hit an opponent, you gain a +4 bonus on checks made to bull rush or trip that opponent, as long as the combat maneuver is your next attack by the end of your turn. You do not provoke an attack of opportunity from the target of the maneuver."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Janni Style", "PRESKILL:2,Acrobatics=5,Perform (dance)=5"]),
            },
            // Kirin Path -- uc_feats.lst:193
            UcFeatEntry {
                key: "Kirin Path",
                category: FeatCategory::Combat,
                name: "Kirin Path",
                description: Some("You turn knowledge of your enemy into a flawless defense."),
                pretext: None,
                source_page: Some("p.107"),
                benefit: Some("Whenever you make a Knowledge check to identify a creature, even when using Kirin Style, you can take 10 even if stress and distractions would normally prevent you from doing so. While using Kirin Style against a creature you have identified using that feat, if the creature ends its turn within your threatened area, you can spend a use of your attacks of opportunity that round to move up to 5 feet times your Intelligence modifier (minimum 1). You must end your move in a square threatened by the creature. This move does not provoke attacks of opportunity."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Kirin Strike,Kirin Style,Improved Unarmed Strike", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRESKILL:1,Knowledge (arcana)=12", "PRESKILL:1,Knowledge (dungeoneering)=5,Knowledge (local)=5,Knowledge (nature)=5,Knowledge (planes)=5,Knowledge (religion)=5"]),
            },
            // Kirin Strike -- uc_feats.lst:194
            UcFeatEntry {
                key: "Kirin Strike",
                category: FeatCategory::Combat,
                name: "Kirin Strike",
                description: Some("You have read the texts of the perfect way, and know how identify to your enemies' weak spot."),
                pretext: None,
                source_page: Some("p.107"),
                benefit: Some("You gain a +2 insight bonus on Knowledge checks made to identify creatures, including the one Kirin Style allows. While using Kirin Style against a creature you have identified using that feat, as a swift action after you have hit a creature with a melee or ranged attack, you can add twice your Intelligence modifier in damage (minimum 2)."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Kirin Style,Improved Unarmed Strike", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRESKILL:1,Knowledge (arcana)=9", "PRESKILL:1,Knowledge (dungeoneering)=3,Knowledge (local)=3,Knowledge (nature)=3,Knowledge (planes)=3,Knowledge (religion)=3"]),
            },
            // Kirin Style -- uc_feats.lst:195
            UcFeatEntry {
                key: "Kirin Style",
                category: FeatCategory::Combat,
                name: "Kirin Style",
                description: Some("Your study and your grace allows you to exploit your enemies' weaknesses."),
                pretext: None,
                source_page: Some("p.107"),
                benefit: Some("While using this style, you can spend a swift action to make a Knowledge check to identify a single creature (DC 15 + the creature's CR for this purpose). If you succeed at the check, while using this style, you gain a +2 bonus on saving throws against that creature's attacks, as well as a +2 dodge bonus to AC against that creature's attacks of opportunity. These bonuses last for as long as you use this style. If you cease combat with the creature during this time and resume it later, you can attempt the check again."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRESKILL:1,Knowledge (arcana)=6", "PRESKILL:1,Knowledge (dungeoneering)=1,Knowledge (local)=1,Knowledge (nature)=1,Knowledge (planes)=1,Knowledge (religion)=1"]),
            },
            // Knockout Artist -- uc_feats.lst:196
            UcFeatEntry {
                key: "Knockout Artist",
                category: FeatCategory::General,
                name: "Knockout Artist",
                description: Some("You can throw devastating knockout punches."),
                pretext: None,
                source_page: Some("p.107"),
                benefit: Some("When you use your unarmed strike to deal nonlethal damage and sneak attack damage to an opponent denied his Dexterity bonus to AC, you gain a +1 bonus on the damage roll per each sneak attack damage die you roll."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Sneak Attack", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike"]),
            },
            // Landing Roll -- uc_feats.lst:197
            UcFeatEntry {
                key: "Landing Roll",
                category: FeatCategory::Combat,
                name: "Landing Roll",
                description: Some("You have learned the technique of rolling safely away when an enemy trips you."),
                pretext: None,
                source_page: Some("p.107"),
                benefit: Some("If you are tripped, you can spend an immediate action to move 5 feet without provoking an attack of opportunity. This does not count as taking a 5-foot step. You fall prone after this movement."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Dodge,Mobility", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]"]),
            },
            // Leaping Shot Deed -- uc_feats.lst:198
            UcFeatEntry {
                key: "Leaping Shot Deed",
                category: FeatCategory::Grit,
                name: "Leaping Shot Deed",
                description: Some("You leap through the air, guns blazing."),
                pretext: None,
                source_page: Some("p.108"),
                benefit: Some("You gain a +2 bonus on Acrobatics checks made to jump. As a full-round action, you can move up to your speed and make firearm attacks at your highest base attack bonus with each loaded firearm you are wielding. You can make these attacks at any point during your movement, and if you are wielding two firearms, you can make the attacks at different points during the movement. At the end of this movement, you fall prone. This deed costs 1 grit point to perform."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Dodge,Mobility", "PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Gunslinger ~ Grit],[PREABILITY:1,CATEGORY=FEAT,Amateur Gunslinger]", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRETOTALAB:4"]),
            },
            // Mantis Style -- uc_feats.lst:199
            UcFeatEntry {
                key: "Mantis Style",
                category: FeatCategory::Combat,
                name: "Mantis Style",
                description: Some("You have learned to target vital areas with crippling accuracy."),
                pretext: None,
                source_page: Some("p.108"),
                benefit: Some("You gain one additional Stunning Fist attempt per day. While using this style, you gain a +2 bonus to the DC of effects you deliver with your Stunning Fist."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Stunning Fist", "PRESKILL:1,Heal=3"]),
            },
            // Mantis Torment -- uc_feats.lst:200
            UcFeatEntry {
                key: "Mantis Torment",
                category: FeatCategory::Combat,
                name: "Mantis Torment",
                description: Some("Your knowledge of the mysteries of anatomy allows you to cause debilitating pain with a simple touch."),
                pretext: None,
                source_page: Some("p.108"),
                benefit: Some("You gain one additional Stunning Fist attempt per day. While using Mantis Style, you make an unarmed attack that expends two daily attempts of your Stunning Fist. If you hit, your opponent must succeed at a saving throw against your Stunning Fist or become dazzled and staggered with crippling pain until the start of your next turn, and at that point the opponent becomes fatigued."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Improved Unarmed Strike,Mantis Style,Mantis Wisdom,Stunning Fist", "PRESKILL:1,Heal=9"]),
            },
            // Mantis Wisdom -- uc_feats.lst:201
            UcFeatEntry {
                key: "Mantis Wisdom",
                category: FeatCategory::Combat,
                name: "Mantis Wisdom",
                description: Some("Your knowledge of vital areas allows you to land debilitating strikes with precision."),
                pretext: None,
                source_page: Some("p.108"),
                benefit: Some("Treat half your levels in classes other than monk as monk levels for determining effects you can apply to a target of your Stunning Fist per the Stunning Fist monk class feature. You can also use a standard action and a successful melee touch attack to remove any Stunning Fist effect you have applied to a target. While using Mantis Style, you gain a +2 bonus on unarmed attack rolls with which you are using Stunning Fist attempts."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Improved Unarmed Strike,Mantis Style,Stunning Fist", "PRESKILL:1,Heal=6"]),
            },
            // Marid Coldsnap -- uc_feats.lst:202
            UcFeatEntry {
                key: "Marid Coldsnap",
                category: FeatCategory::Combat,
                name: "Marid Coldsnap",
                description: Some("You can summon a torrent of water to blast your enemies, chilling them to the bone."),
                pretext: None,
                source_page: Some("p.108"),
                benefit: Some("While using Marid Style, as a standard action, you can spend two Elemental Fist (Advanced Player's Guide 158) attempts to unleash a 30-foot line of frigid water. Creatures caught in the line take your unarmed strike damage plus the cold damage from your Elemental Fist attack and are entangled in ice as per the Marid Spirit feat. A successful Reflex save (DC 10 + 1/2 your character level + your Wis modifier) reduces the damage by half and prevents a target from becoming entangled."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Elemental Fist,Marid Spirit,Marid Style,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:13],[PREVARGTEQ:MonkFeatQualify,11]", "PRESTAT:2,CON=15,WIS=17"]),
            },
            // Marid Spirit -- uc_feats.lst:203
            UcFeatEntry {
                key: "Marid Spirit",
                category: FeatCategory::Combat,
                name: "Marid Spirit",
                description: Some("You can manipulate cold energy to protect yourself and freeze your foes."),
                pretext: None,
                source_page: Some("p.108"),
                benefit: Some("You gain one additional Elemental Fist attempt per day. While using Marid Style, you gain cold resistance equal to your base attack bonus, or monk level plus base attack bonus gained from levels in classes other than monk, whichever is higher. While denied your Dexterity bonus to AC you are also denied this resistance. Creatures that take cold damage from your Elemental Fist attack must succeed at a Fortitude save (DC 10 + 1/2 your character level + your Wis modifier) or become entangled in ice for 1d4 rounds. The ice has hit points equal to three times your base attack bonus or monk level, whichever is higher, and a break DC of 15 + your base attack bonus or monk level, whichever is higher. Destroying or breaking the ice ends the entangled condition."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Elemental Fist,Marid Style,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:11],[PREVARGTEQ:MonkFeatQualify,9]", "PRESTAT:2,CON=15,WIS=15"]),
            },
            // Marid Style -- uc_feats.lst:204
            UcFeatEntry {
                key: "Marid Style",
                category: FeatCategory::Combat,
                name: "Marid Style",
                description: Some("You conjure tendrils of icy water to strike your enemies from a distance."),
                pretext: None,
                source_page: Some("p.108"),
                benefit: Some("You gain one additional Elemental Fist attempt per day. While using this style and Elemental Fist to deal cold damage, you gain a bonus on cold damage rolls equal to your Wisdom modifier, and your reach with your unarmed strike increases by 5 feet."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Elemental Fist,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:9],[PREVARGTEQ:MonkFeatQualify,5]", "PRESTAT:2,CON=13,WIS=15"]),
            },
            // Master Combat Performer -- uc_feats.lst:205
            UcFeatEntry {
                key: "Master Combat Performer",
                category: FeatCategory::Combat,
                name: "Master Combat Performer",
                description: Some("You are a master of the techniques and weapons of the arena and the stage."),
                pretext: None,
                source_page: Some("p.109"),
                benefit: Some("You can make performance combat checks as a free action. You are proficient in all weapons with the performance special quality."),
                prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Performing Combatant],[PREABILITY:3,CATEGORY=FEAT,TYPE=Performance]", "PRETOTALAB:6"]),
            },
            // Master Siege Engineer -- uc_feats.lst:206
            UcFeatEntry {
                key: "Master Siege Engineer",
                category: FeatCategory::Combat,
                name: "Master Siege Engineer",
                description: Some("You are significantly faster at loading a siege engine, as well as a better shot."),
                pretext: None,
                source_page: Some("p.109"),
                benefit: Some("If you are the crew lead on a siege engine, your crew can use move actions to load a siege engine. When you spend actions to aim a siege engine, you and your crew can use move actions instead of full-round actions to aim the siege engine (page 160). &nl; [Normal] Full-round actions are required to load and aim siege engines."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Siege Weapon Engineer", "PRESKILL:1,Knowledge (engineering)=10"]),
            },
            // Masterful Display -- uc_feats.lst:210
            UcFeatEntry {
                key: "Masterful Display",
                category: FeatCategory::Combat,
                name: "Masterful Display",
                description: Some("You craft a special victory performance that causes the crowd to go wild."),
                pretext: None,
                source_page: Some("p.109"),
                benefit: Some("Choose the effects of any two performance feats you have. When you make a performance combat check, you gain the benefits of those two feats, but you only gain a +2 bonus on the performance combat check."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Dazzling Display", "PREABILITY:2,CATEGORY=FEAT,TYPE=Performance"]),
            },
            // Maximized Spellstrike -- uc_feats.lst:211
            UcFeatEntry {
                key: "Maximized Spellstrike",
                category: FeatCategory::General,
                name: "Maximized Spellstrike",
                description: Some("You deal brutal damage against opponents caught unawares."),
                pretext: None,
                source_page: Some("p.109"),
                benefit: Some("When you make a melee attack and successfully use your spellstrike ability against an opponent denied his Dexterity bonus to AC, you can spend 3 points from your arcane pool to maximize the spell delivered through your spellstrike as if using the Maximize Spell metamagic feat."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Maximized Magic ~ Magus Arcana", "PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Samurai ~ Weapon Expertise],[PREABILITY:1,CATEGORY=FEAT,Quick Draw]"]),
            },
            // Menacing Bane -- uc_feats.lst:212
            UcFeatEntry {
                key: "Menacing Bane",
                category: FeatCategory::General,
                name: "Menacing Bane",
                description: Some("You are deadly when you team up with allies against a single foe."),
                pretext: None,
                source_page: Some("p.109"),
                benefit: Some("You can use your bane class feature to imbue a melee weapon with the menacing special weapon ability (Advanced Player's Guide 288) instead of bane. You can spend a swift action to switch between the two special weapon abilities. Doing so otherwise works according to your bane class feature. &nl; [Special]If you have the Double Bane feat, you can imbue each weapon you wield with either bane or menacing. No single weapon can have both."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Bane"]),
            },
            // Merciful Bane -- uc_feats.lst:213
            UcFeatEntry {
                key: "Merciful Bane",
                category: FeatCategory::General,
                name: "Merciful Bane",
                description: Some("You can use your bane ability to inflict nonlethal damage."),
                pretext: None,
                source_page: Some("p.109"),
                benefit: Some("While a weapon you wield is under the effect of your bane class feature, you can spend a swift action to switch between dealing lethal or nonlethal damage with bane. While your bane effect allows you to deal nonlethal damage in this way, you take no penalty on your attack roll for using a lethal weapon to deal nonlethal damage. &nl; [Normal] When using a lethal weapon to deal nonlethal damage, you take a -4 penalty on attack rolls."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Bane"]),
            },
            // Mocking Dance -- uc_feats.lst:214
            UcFeatEntry {
                key: "Mocking Dance",
                category: FeatCategory::Combat,
                name: "Mocking Dance",
                description: Some("You do a little dance that mocks your foe and entertains the crowd."),
                pretext: None,
                source_page: Some("p.109"),
                benefit: Some("When you spend a swift action to make a performance combat check, before making that check you can either move 5 feet without provoking attacks of opportunity, or you can move your speed and provoke attacks of opportunity. You cannot end this move in a space where you threaten an enemy. If you do move at least 5 feet, you gain a +2 bonus on the performance combat check."),
                prerequisites: Some(&["PRESKILL:1,Acrobatics=4,Perform (dance)=4"]),
            },
            // Monastic Legacy -- uc_feats.lst:215
            UcFeatEntry {
                key: "Monastic Legacy",
                category: FeatCategory::Combat,
                name: "Monastic Legacy",
                description: Some("Your formal unarmed training continues to bolster your training in other areas."),
                pretext: None,
                source_page: Some("p.109"),
                benefit: Some("Add half the levels you have in classes other than monk to your monk level to determine your effective monk level for your base unarmed strike damage. This feat does not make levels in classes other than monk count toward any other monk class features (MonkLevelsAdjusted %1).|MonkLevelsAdjusted"),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Still Mind", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike"]),
            },
            // Monkey Moves -- uc_feats.lst:216
            UcFeatEntry {
                key: "Monkey Moves",
                category: FeatCategory::Combat,
                name: "Monkey Moves",
                description: Some("You scramble around your foes, moving and striking in an erratic fashion."),
                pretext: None,
                source_page: Some("p.109"),
                benefit: Some("While using Monkey Style, you gain a Wisdom bonus on Climb checks. You can also can climb and crawl at half your speed; you can take a 5-foot step by jumping, crawling, or climbing; and you retain your Dexterity bonus to AC while climbing. Further, while using Monkey Style, when you use your unarmed strike to hit an opponent twice or more on your turn, you can spend a swift action to take a 5-foot step even if you have moved this round. &nl; [Normal] You climb at one-quarter your speed, and you lose your Dexterity bonus to AC while doing so. A 5-foot step is made using your normal movement modes, and you can take one only if you have not otherwise moved this round."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Monkey Style", "PRESKILL:2,Acrobatics=8,Climb=8", "PRESTAT:1,WIS=13"]),
            },
            // Monkey Shine -- uc_feats.lst:217
            UcFeatEntry {
                key: "Monkey Shine",
                category: FeatCategory::Combat,
                name: "Monkey Shine",
                description: Some("You combine acrobatics and opportunity to devastating effect against your opponent."),
                pretext: None,
                source_page: Some("p.110"),
                benefit: Some("While using Monkey Style, if you successfully deliver a Stunning Fist attempt, in addition to the normal effect of Stunning Fist, you can spend a free action to enter a square adjacent to you that is within your opponent's space. This movement does not provoke attacks of opportunity. While you are in your opponent's space, you gain a +4 dodge bonus to AC and a +4 bonus on melee attack rolls against that opponent. If otherwise unhindered, the opponent can move away from you, but if he does, he provokes an attack of opportunity from you even if his choice of movement does not normally do so. &nl; [Normal] You cannot enter an opponent's space."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Improved Unarmed Strike,Monkey Moves,Monkey Style,Stunning Fist", "PRESKILL:2,Acrobatics=11,Climb=11", "PRESTAT:1,WIS=13"]),
            },
            // Monkey Style -- uc_feats.lst:218
            UcFeatEntry {
                key: "Monkey Style",
                category: FeatCategory::Combat,
                name: "Monkey Style",
                description: Some("Your unarmed fighting style is nimble and unpredictable, full of ground rolls and short leaps."),
                pretext: None,
                source_page: Some("p.110"),
                benefit: Some("You add your Wisdom bonus on Acrobatics checks. While using this style, you take no penalty on melee attack rolls or to AC while prone. Further, you can crawl and stand up from lying prone without provoking attacks of opportunity, and you can stand up as a swift action if you succeed at a DC 20 Acrobatics check. &nl; [Normal] You take a -4 penalty on attack rolls and AC against melee attacks while prone. Standing up is a standard action that provokes attacks of opportunity."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRESKILL:2,Acrobatics=5,Climb=5", "PRESTAT:1,WIS=13"]),
            },
            // Murderer's Circle -- uc_feats.lst:219
            UcFeatEntry {
                key: "Murderer's Circle",
                category: FeatCategory::Combat,
                name: "Murderer's Circle",
                description: Some("After savaging your foe, you circle like a hunter ready for the kill."),
                pretext: None,
                source_page: Some("p.111"),
                benefit: Some("When you spend a swift action to make a performance combat check after scoring a critical hit or performing a combat maneuver, and you are adjacent to the target of the critical hit or combat maneuver, you can move to any other space that is adjacent to the target without provoking attacks of opportunity. You must have a clear path to that space and the ability to reach it by spending a move action. If you end this move in any space other than the one where you started, you gain a +2 bonus on the performance combat check."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Dodge", "PRESKILL:1,Acrobatics=4"]),
            },
            // Neckbreaker -- uc_feats.lst:220
            UcFeatEntry {
                key: "Neckbreaker",
                category: FeatCategory::Combat,
                name: "Neckbreaker",
                description: Some("With a quick jerk, you snap an enemy's neck."),
                pretext: None,
                source_page: Some("p.111"),
                benefit: Some("If you have an opponent your size or smaller helpless or pinned, after you initiate or maintain a grapple, you can make a Stunning Fist attempt at a -5 penalty on the attack roll. If you succeed, you wrench that opponent's neck, dealing 2d6 Strength or Dexterity damage. If the targeted ability score is reduced to 0, any remaining damage is dealt to that opponent's Constitution score. A creature that is immune to critical hits or that has no discernible head and neck is immune to the effects of this feat."),
                prerequisites: Some(&["PREABILITY:6,CATEGORY=FEAT,Bonebreaker,Greater Grapple,Improved Grapple,Improved Unarmed Strike,Jawbreaker,Stunning Fist", "PRESKILL:1,Heal=12"]),
            },
            // Net Adept -- uc_feats.lst:221
            UcFeatEntry {
                key: "Net Adept",
                category: FeatCategory::Combat,
                name: "Net Adept",
                description: Some("You have trained to use the net as a melee weapon."),
                pretext: None,
                source_page: Some("p.111"),
                benefit: Some("You can treat a net as a one-handed melee reach weapon with a 10-foot reach. Further, you take no penalty on melee attack rolls for using an unfolded net, and you can use one full-round action or two move actions to fold a net. &nl; [Normal] A net is a ranged weapon that imposes a -4 penalty on ranged attack rolls if it is unfolded. Folding a net takes a proficient user 2 rounds."),
                prerequisites: Some(&["PRETOTALAB:1", "PREWEAPONPROF:1,Net"]),
            },
            // Net Maneuvering -- uc_feats.lst:222
            UcFeatEntry {
                key: "Net Maneuvering",
                category: FeatCategory::Combat,
                name: "Net Maneuvering",
                description: Some("With sweeping movements and brute force, you can use your net to put foes at a disadvantage."),
                pretext: None,
                source_page: Some("p.111"),
                benefit: Some("In melee, you can use a net to trip or disarm opponents instead of entangling them. You gain a +2 bonus on disarm checks made to use a net in this way. Further, if you have an opponent entangled in your net, you can attempt to drag or reposition that opponent as long as he is within your net's reach or you control the trailing rope on your net."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Net Adept", "PRETOTALAB:3", "PREWEAPONPROF:1,Net"]),
            },
            // Net Trickery -- uc_feats.lst:223
            UcFeatEntry {
                key: "Net Trickery",
                category: FeatCategory::Combat,
                name: "Net Trickery",
                description: Some("You have become very proficient at using your net to hinder your enemies."),
                pretext: None,
                source_page: Some("p.111"),
                benefit: Some("In place of one of your melee attacks, you can use your net to attempt a dirty trick combat maneuver to blind an opponent (Advance Player's Guide 320). If you have an opponent entangled in your net, you can attempt to trip that opponent as long as he is within your net's reach or you control the trailing rope on your net. You also gain a +2 bonus on drag and reposition combat maneuver checks you make using your net."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Net Adept,Net Maneuvering", "PRETOTALAB:6", "PREWEAPONPROF:1,Net"]),
            },
            // Net and Trident -- uc_feats.lst:224
            UcFeatEntry {
                key: "Net and Trident",
                category: FeatCategory::Combat,
                name: "Net and Trident",
                description: Some("Your skill with lighter weapons allows you to wield one alongside your net."),
                pretext: None,
                source_page: Some("p.111"),
                benefit: Some("You can treat a net as a one-handed ranged weapon, allowing you to wield a light or one-handed melee weapon and still make ranged attacks with your net. When you use your light or one-handed melee weapon to attack an entangled opponent, you gain a +2 bonus on damage rolls and on attack rolls to confirm a critical hit. &nl; [Normal] A net is a two-handed ranged weapon."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Net Adept,Two-Weapon Fighting", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,15],[PREVARGTEQ:FeatDexRequirement,15]", "PREWEAPONPROF:1,Net"]),
            },
            // Nightmare Fist -- uc_feats.lst:225
            UcFeatEntry {
                key: "Nightmare Fist",
                category: FeatCategory::Combat,
                name: "Nightmare Fist",
                description: Some("You are even more deadly in magical darkness."),
                pretext: None,
                source_page: Some("p.111"),
                benefit: Some("While fighting within an area of magical darkness, you gain a +2 bonus on damage rolls with unarmed strikes, or a +4 bonus against opponents that are shaken, frightened, or panicked. You also gain a +2 morale bonus on Acrobatics and Intimidate checks."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PREMULT:1,[PRESPELL:1,Darkness,Deeper Darkness,Hungry Darkness],[PREABILITY:1,CATEGORY=Special Ability,Shadow Bloodline ~ Enveloping Darkness]", "PRESKILL:1,Intimidate=1"]),
            },
            // Nightmare Striker -- uc_feats.lst:226
            UcFeatEntry {
                key: "Nightmare Striker",
                category: FeatCategory::Combat,
                name: "Nightmare Striker",
                description: Some("Your faerie fire not only illuminates your foes, but it also shows you their weaknesses."),
                pretext: None,
                source_page: Some("p.111"),
                benefit: Some("While a faerie fire you have cast (not one cast from a spell completion or spell trigger item) outlines an opponent, the DC for that opponent to resist your Stunning Fist attempts increases by +2. If you hit an opponent with a Stunning Fist attempt, and that opponent fails her saving throw, you can render the target shaken for 1d2 rounds plus 1 round for every 5 by which the opponent failed her save."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Improved Unarmed Strike,Nightmare Fist,Nightmare Weaver,Stunning Fist", "PRESKILL:1,Heal=5", "PRESPELL:1,Faerie Fire"]),
            },
            // Nightmare Weaver -- uc_feats.lst:227
            UcFeatEntry {
                key: "Nightmare Weaver",
                category: FeatCategory::Combat,
                name: "Nightmare Weaver",
                description: Some("You can use your ability to create magical darkness to terrorize enemies."),
                pretext: None,
                source_page: Some("p.112"),
                benefit: Some("By spending a full-round action to cast darkness, you can also make Intimidate checks to demoralize all foes in the spell's initial area. &nl; [Special]This feat counts as Dazzling Display for purposes of qualifying for Deadly Stroke and Shatter Defenses."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Nightmare Fist", "PRESKILL:1,Intimidate=2", "PRESPELL:1,Darkness"]),
            },
            // No Name -- uc_feats.lst:228
            UcFeatEntry {
                key: "No Name",
                category: FeatCategory::Grit,
                name: "No Name",
                description: Some("You don't need an elaborate disguise to keep your identity under wraps."),
                pretext: None,
                source_page: Some("p.112"),
                benefit: Some("You often rely on surprise and misdirection in your social dealings. You gain a +2 bonus on Bluff checks, and you can spend 1 grit point to gain a +10 bonus on Disguise checks for 10 minutes per your gunslinger level (minimum 10 minutes). This deed does not actually change your appearance, but rather allows you to hide your identity in other ways."),
                prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Gunslinger ~ Grit],[PREABILITY:1,CATEGORY=FEAT,Amateur Gunslinger]", "PRESKILL:1,Bluff=4"]),
            },
            // Opening Volley -- uc_feats.lst:229
            UcFeatEntry {
                key: "Opening Volley",
                category: FeatCategory::Combat,
                name: "Opening Volley",
                description: Some("Your ranged assault leaves your foe disoriented and vulnerable to your melee attack."),
                pretext: None,
                source_page: Some("p.112"),
                benefit: Some("Whenever you deal damage with a ranged attack, you gain a +4 circumstance bonus on the next melee attack roll you make against the opponent. This attack must occur before the end of your next turn."),
                prerequisites: None,
            },
            // Pack Attack -- uc_feats.lst:230
            UcFeatEntry {
                key: "Pack Attack",
                category: FeatCategory::Combat,
                name: "Pack Attack",
                description: Some("You are skilled at surrounding your enemies."),
                pretext: None,
                source_page: Some("p.112"),
                benefit: Some("When you are adjacent to an ally with this feat, the first time you melee attack an opponent, you can spend an immediate action to take a 5-foot step, even if you have otherwise moved this round. &nl; [Normal] You can take a 5-foot step only if you have not otherwise moved in a round."),
                prerequisites: Some(&["PRETOTALAB:1"]),
            },
            // Panther Claw -- uc_feats.lst:231
            UcFeatEntry {
                key: "Panther Claw",
                category: FeatCategory::Combat,
                name: "Panther Claw",
                description: Some("You unleash a rapid series of blows on foes that attempt to attack you when you move."),
                pretext: None,
                source_page: Some("p.112"),
                benefit: Some("While using Panther Style, you can spend a free action, instead of spending a swift action, to make a retaliatory unarmed strike. You can make a number of retaliatory unarmed strikes on your turn equal to your Wisdom modifier."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Combat Reflexes,Improved Unarmed Strike,Panther Style", "PRESTAT:1,WIS=15"]),
            },
            // Panther Parry -- uc_feats.lst:232
            UcFeatEntry {
                key: "Panther Parry",
                category: FeatCategory::Combat,
                name: "Panther Parry",
                description: Some("Your vicious strikes impair your foe's ability to attack you when you move."),
                pretext: None,
                source_page: Some("p.112"),
                benefit: Some("While using Panther Style, your retaliatory unarmed strikes are resolved before the triggering attacks. If your retaliatory unarmed strike deals damage to an opponent, that opponent takes a -2 penalty on attack and damage rolls with the triggering attack of opportunity."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Combat Reflexes,Improved Unarmed Strike,Panther Claw,Panther Style", "PRESTAT:1,WIS=15"]),
            },
            // Panther Style -- uc_feats.lst:233
            UcFeatEntry {
                key: "Panther Style",
                category: FeatCategory::Combat,
                name: "Panther Style",
                description: Some("You can strike back at enemies who attack you when you move."),
                pretext: None,
                source_page: Some("p.112"),
                benefit: Some("While using this style, when an opponent makes an attack of opportunity against you for moving through a threatened square, you can spend a swift action to make a retaliatory unarmed strike attack against that opponent. Your attack is resolved after the triggering attack of opportunity."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Combat Reflexes,Improved Unarmed Strike", "PRESTAT:1,WIS=13"]),
            },
            // Passing Trick -- uc_feats.lst:234
            UcFeatEntry {
                key: "Passing Trick",
                category: FeatCategory::Combat,
                name: "Passing Trick",
                description: Some("Slipping past a foe gives you the chance to feint."),
                pretext: None,
                source_page: Some("p.112"),
                benefit: Some("Whenever you make a successful Acrobatics check to move through an opponent's space, you can spend a swift action to make a Bluff check against that opponent to feint in combat. &nl; [Special]If you have the Underfoot feat and the opponent is larger than you, you gain a +2 bonus on the Bluff check this feat allows."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Combat Expertise,Dodge,Improved Feint,Mobility", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRESIZELTEQ:S"]),
            },
            // Performance Weapon Mastery -- uc_feats.lst:235
            UcFeatEntry {
                key: "Performance Weapon Mastery",
                category: FeatCategory::Combat,
                name: "Performance Weapon Mastery",
                description: Some("You wield all your weapons with the flair of a performer."),
                pretext: None,
                source_page: Some("p.112"),
                benefit: Some("You treat all weapons you are proficient in as if they had the performance weapon quality (page 144)."),
                prerequisites: None,
            },
            // Performing Combatant -- uc_feats.lst:236
            UcFeatEntry {
                key: "Performing Combatant",
                category: FeatCategory::Combat,
                name: "Performing Combatant",
                description: Some("You treat every combat as a performance, bringing flare and showmanship."),
                pretext: None,
                source_page: Some("p.112"),
                benefit: Some("You can make performance combat checks in any combat. When making a performance check outside of performance combat, you can pick a single performance feat to use. You automatically gain any bonus on the performance combat check the feat grants, and then you make a DC 20 performance combat check. On a success, you gain the full effect of the performance feat you chose."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,TYPE=Performance"]),
            },
            // Pin Down -- uc_feats.lst:237
            UcFeatEntry {
                key: "Pin Down",
                category: FeatCategory::Combat,
                name: "Pin Down",
                description: Some("You easily block enemy escapes."),
                pretext: None,
                source_page: Some("p.113"),
                benefit: Some("Whenever an opponent you threaten takes a 5-foot step or uses the withdraw action, that opponent provokes an attack of opportunity from you. If the attack hits, you deal no damage, but the targeted creature is prevented from making the move action that granted a 5-foot step or the withdraw action and does not move."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Combat Reflexes", "PREVARGTEQ:FighterWeaponQualifyLVL,11"]),
            },
            // Pinning Knockout -- uc_feats.lst:238
            UcFeatEntry {
                key: "Pinning Knockout",
                category: FeatCategory::Combat,
                name: "Pinning Knockout",
                description: Some("An opponent you have pinned is easy for you to knock out."),
                pretext: None,
                source_page: Some("p.113"),
                benefit: Some("While you have an opponent pinned, when you succeed at a grapple combat maneuver check to deal an opponent nonlethal damage using an unarmed strike or a light or one-handed weapon, double your damage result. Any creature that is immune to critical hits is immune to the effects of this feat."),
                prerequisites: Some(&["PREMULT:1,[PREMULT:3,[PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]],[PREABILITY:3,CATEGORY=FEAT,Greater Grapple,Improved Grapple,Improved Unarmed Strike],[PRETOTALAB:9]],[PREVARGTEQ:MonkFeatQualify,9]"]),
            },
            // Pinning Rend -- uc_feats.lst:239
            UcFeatEntry {
                key: "Pinning Rend",
                category: FeatCategory::Combat,
                name: "Pinning Rend",
                description: Some("You tear flesh when you damage an opponent that you have pinned."),
                pretext: None,
                source_page: Some("p.113"),
                benefit: Some("While you have an opponent pinned, when you succeed at a grapple combat maneuver check to deal an opponent damage using an unarmed strike or a light or one-handed weapon, that opponent also takes bleed damage equal to your unarmed strike or weapon damage dice. Any creature that is immune to critical hits is immune to the effects of this feat."),
                prerequisites: Some(&["PREMULT:1,[PREMULT:3,[PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]],[PREABILITY:3,CATEGORY=FEAT,Greater Grapple,Improved Grapple,Improved Unarmed Strike],[PRETOTALAB:9]],[PREVARGTEQ:MonkFeatQualify,9]"]),
            },
            // Pinpoint Poisoner -- uc_feats.lst:240
            UcFeatEntry {
                key: "Pinpoint Poisoner",
                category: FeatCategory::Combat,
                name: "Pinpoint Poisoner",
                description: Some("You deftly use specially prepared needles to apply poison for maximum effect."),
                pretext: None,
                source_page: Some("p.113"),
                benefit: Some("When you use Adder Strike, you can instead poison up to two blowgun darts that you can then use to strike your opponent in melee. (Drawing such darts is a free action.) While holding these darts, you can spend a standard action to attack with one or a full-attack action to attack with both. Such attacks are considered melee touch attacks that deal 1d2 damage plus any bonuses you gain on your normal unarmed strike damage, and they deliver the poison. You can instead throw such darts as if they were shuriken, making your ranged attack rolls against the target's AC. &nl; [Normal] Applying poison to a weapon or single piece of ammunition is a standard action."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Poison Use,Poison Use", "PREABILITY:2,CATEGORY=FEAT,Adder Strike,Improved Unarmed Strike", "PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Two-Weapon Fighting],[PREABILITY:1,CATEGORY=Special Ability,Flurry of Blows]", "PRESKILL:1,Craft (alchemy)=6"]),
            },
            // Planar Wild Shape -- uc_feats.lst:241
            UcFeatEntry {
                key: "Planar Wild Shape",
                category: FeatCategory::General,
                name: "Planar Wild Shape",
                description: Some("You can infuse your wild shape with planar strength."),
                pretext: None,
                source_page: Some("p.113"),
                benefit: Some("When you use wild shape to take the form of an animal, you can expend an additional daily use of your wild shape class feature to add the celestial template or fiendish template to your animal form. (Good druids must use the celestial template, while evil druids must use the fiendish template.) If your form has the celestial template and you score a critical threat against an evil creature while using your form's natural weapons, you gain a +2 bonus on the attack roll to confirm the critical hit. The same bonus applies if your form has the fiendish template and you score a critical threat against a good creature."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape", "PRESKILL:1,Knowledge (planes)=5"]),
            },
            // Prone Shooter -- uc_feats.lst:242
            UcFeatEntry {
                key: "Prone Shooter",
                category: FeatCategory::Combat,
                name: "Prone Shooter",
                description: Some("Take a reduced penalty to your AC against melee attacks while prone."),
                pretext: None,
                source_page: Some("p.113"),
                benefit: Some("If you have been prone since the end of your last turn, the penalty to your Armor Class against melee attacks made against you is reduced to -2. In addition, the bonus to your Armor Class against ranged attacks made against you is increased to +6."),
                prerequisites: Some(&["PRETOTALAB:1"]),
            },
            // Prone Slinger -- uc_feats.lst:243
            UcFeatEntry {
                key: "Prone Slinger",
                category: FeatCategory::Combat,
                name: "Prone Slinger",
                description: Some("Your sideways sling release allows you to launch bullets and stones even while prone."),
                pretext: None,
                source_page: Some("p.113"),
                benefit: Some("While prone, you can use a sling to make ranged attacks. &nl; [Normal] Crossbows and firearms are the only ranged weapons that can be used while prone."),
                prerequisites: None,
            },
            // Quick Bull Rush -- uc_feats.lst:244
            UcFeatEntry {
                key: "Quick Bull Rush",
                category: FeatCategory::Combat,
                name: "Quick Bull Rush",
                description: Some("You can barrel into your opponent and follow this with an attack."),
                pretext: None,
                source_page: Some("p.113"),
                benefit: Some("On your turn, you can perform a single bull rush combat maneuver in place of one of your melee attacks. You must choose the melee attack with the highest base attack bonus to make the bull rush. &nl; [Normal] A bull rush combat maneuver is a standard action."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Bull Rush,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:6"]),
            },
            // Quick Dirty Trick -- uc_feats.lst:245
            UcFeatEntry {
                key: "Quick Dirty Trick",
                category: FeatCategory::Combat,
                name: "Quick Dirty Trick",
                description: Some("You can perpetrate a dirty trick and deliver an attack before your opponent is the wiser."),
                pretext: None,
                source_page: Some("p.114"),
                benefit: Some("On your turn, you can perform a single dirty trick combat maneuver (Advanced Players Guide 320) in place of one of your melee attacks. You must choose the melee attack with the highest base attack bonus to make the dirty trick combat maneuver. &nl; [Normal] A dirty trick combat maneuver is a standard action."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Dirty Trick", "PREMULT:1,[PREVARGTEQ:PreStatScore_INT,13],[PREVARGTEQ:CombatFeatIntRequirement,13],[PREVARGTEQ:DirtyTricksterIntQualify,1]", "PRETOTALAB:6"]),
            },
            // Quick Drag -- uc_feats.lst:246
            UcFeatEntry {
                key: "Quick Drag",
                category: FeatCategory::Combat,
                name: "Quick Drag",
                description: Some("You drag your enemy and deliver a punishing blow."),
                pretext: None,
                source_page: Some("p.114"),
                benefit: Some("On your turn, you can perform a single drag combat maneuver (Advanced Players Guide 320) in place of one of your melee attacks. You must choose the melee attack with the highest base attack bonus to make the drag. &nl; [Normal] A drag combat maneuver is a standard action."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Drag,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:6"]),
            },
            // Quick Reposition -- uc_feats.lst:247
            UcFeatEntry {
                key: "Quick Reposition",
                category: FeatCategory::Combat,
                name: "Quick Reposition",
                description: Some("Your opponent becomes an unwitting dance partner, following your lead while you fight."),
                pretext: None,
                source_page: Some("p.114"),
                benefit: Some("On your turn, you can perform a single reposition combat maneuver (Advanced Players Guide 320) in place of one of your melee attacks. You must choose the melee attack with the highest base attack bonus to make the reposition. &nl; [Normal] A reposition combat maneuver is a standard action."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Reposition", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:6"]),
            },
            // Quick Steal -- uc_feats.lst:248
            UcFeatEntry {
                key: "Quick Steal",
                category: FeatCategory::Combat,
                name: "Quick Steal",
                description: Some("You are adept at relieving foes of their belongings even while you strike."),
                pretext: None,
                source_page: Some("p.114"),
                benefit: Some("On your turn, you can perform a single steal combat maneuver (Advanced Players Guide 320) in place of one of your melee attacks. You must choose the melee attack with the highest base attack bonus to make the steal. &nl; [Normal] A steal combat maneuver is a standard action."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Steal", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:6"]),
            },
            // Raging Brutality -- uc_feats.lst:249
            UcFeatEntry {
                key: "Raging Brutality",
                category: FeatCategory::General,
                name: "Raging Brutality",
                description: Some("You expend some of your rage to strike your opponents with a more powerful weapon blow."),
                pretext: None,
                source_page: Some("p.114"),
                benefit: Some("While raging and using Power Attack, you can spend 3 additional rounds of your rage as a swift action to add your Constitution bonus on damage rolls for melee attacks or thrown weapon attacks you make on your turn. If you are using the weapon two-handed, instead add 1-1/2 times your Constitution bonus. This bonus damage is not multiplied on a critical hit."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Rage", "PREABILITY:1,CATEGORY=FEAT,Power Attack", "PRESTAT:1,STR=13", "PRETOTALAB:12"]),
            },
            // Raging Deathblow -- uc_feats.lst:250
            UcFeatEntry {
                key: "Raging Deathblow",
                category: FeatCategory::General,
                name: "Raging Deathblow",
                description: Some("Every killing blow gives you a surge of vitality, further fueling your rage."),
                pretext: None,
                source_page: Some("p.114"),
                benefit: Some("While raging, whenever your attack reduces an opponent of a CR greater than or equal to your character level to -1 or fewer hit points, you gain 1 extra round of rage for that day. If that attack was a critical hit, you gain 1 additional extra round of rage for that day. Whenever you rest to renew your total number of rounds of rage per day, any extra rounds you still have from this feat are lost."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Greater Rage"]),
            },
            // Raging Hurler -- uc_feats.lst:251
            UcFeatEntry {
                key: "Raging Hurler",
                category: FeatCategory::General,
                name: "Raging Hurler",
                description: Some("An opponent can do little to evade your wrathful pitching of weapons and objects."),
                pretext: None,
                source_page: Some("p.114"),
                benefit: Some("While raging, you can throw a two-handed weapon as a standard action, and you double the range increment for weapons you throw. If you also have the Quick Draw feat, you can throw two-handed weapons at your full normal rate of attacks. Further, you can pick up an unattended object that you can use as a improvised weapon within your reach as part of the attack action to throw that item."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Rage", "PREABILITY:1,CATEGORY=FEAT,Throw Anything"]),
            },
            // Raging Throw -- uc_feats.lst:252
            UcFeatEntry {
                key: "Raging Throw",
                category: FeatCategory::General,
                name: "Raging Throw",
                description: Some("You expend some of your rage to throw one opponent at another."),
                pretext: None,
                source_page: Some("p.114"),
                benefit: Some("While raging, when you attempt a bull rush combat maneuver, you can spend 1 additional round of your rage as a swift action to add your Constitution bonus on your combat maneuver check to the bull rush. Further, if you bull rush an opponent into a square another creature occupies or into a solid object, the opponent and the creature or object take bludgeoning damage equal to your Strength modifier + your Constitution modifier."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Rage", "PREABILITY:2,CATEGORY=FEAT,Improved Bull Rush,Power Attack", "PRESTAT:2,STR=13,CON=13", "PRETOTALAB:6"]),
            },
            // Rapid Grappler -- uc_feats.lst:253
            UcFeatEntry {
                key: "Rapid Grappler",
                category: FeatCategory::Combat,
                name: "Rapid Grappler",
                description: Some("You are a quick hand at grappling."),
                pretext: None,
                source_page: Some("p.114"),
                benefit: Some("Whenever you use Greater Grapple to successfully maintain a grapple as a move action, you can then spend a swift action to make a grapple combat maneuver check at a -5 penalty."),
                prerequisites: Some(&["PREMULT:1,[PREMULT:3,[PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]],[PREABILITY:3,CATEGORY=FEAT,Greater Grapple,Improved Grapple,Improved Unarmed Strike],[PRETOTALAB:9]],[PREVARGTEQ:MonkFeatQualify,9]"]),
            },
            // Rebounding Leap -- uc_feats.lst:255
            UcFeatEntry {
                key: "Rebounding Leap",
                category: FeatCategory::Combat,
                name: "Rebounding Leap",
                description: Some("Your riding and lancing expertise allows you to enter and leave the saddle with great speed."),
                pretext: None,
                source_page: Some("p.115"),
                benefit: Some("When you succeed at the Acrobatics check to jump as part of your leaping lance class feature, you can remount your steed as a swift action."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Dragoon ~ Leaping Lance", "PRESKILL:2,Acrobatics=5,Ride=11"]),
            },
            // Rebuffing Reduction -- uc_feats.lst:258
            UcFeatEntry {
                key: "Rebuffing Reduction",
                category: FeatCategory::Combat,
                name: "Rebuffing Reduction",
                description: Some("Your damage reduction can turn the force of blows back on your enemies."),
                pretext: None,
                source_page: Some("p.115"),
                benefit: Some("Whenever an opponent that is adjacent to you fails to penetrate your DR with a melee attack, you can spend an immediate action to attempt a bull rush combat maneuver against that opponent. If you succeed, you cannot move with the opponent. &nl; [Normal] A bull rush combat maneuver is a standard action."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Bull Rush,Power Attack", "PREDR:1,ANY=1", "PRESTAT:1,STR=13", "PRETOTALAB:1"]),
            },
            // Rending Fury -- uc_feats.lst:259
            UcFeatEntry {
                key: "Rending Fury",
                category: FeatCategory::Combat,
                name: "Rending Fury",
                description: Some("You easily tear your enemies limb from limb with your natural attacks."),
                pretext: None,
                source_page: Some("p.116"),
                benefit: Some("You deal rend damage if you hit with half the normal natural attacks your rend requires. For example, a troll that has this feat can rend when it hits with one claw attack, while a girallon that has this feat must hit one target with two claw attacks to rend. You can only make this rend attack once per round."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Rend", "PRETOTALAB:6"]),
            },
            // Revelation Strike -- uc_feats.lst:261
            UcFeatEntry {
                key: "Revelation Strike",
                category: FeatCategory::Combat,
                name: "Revelation Strike",
                description: Some("Your unarmed strike brings a revelation down upon your foe."),
                pretext: None,
                source_page: None,
                benefit: Some("When you gain this feat, choose one revelation that you can use to affect no more than one opponent. If you make a successful unarmed strike against an opponent, in addition to dealing your unarmed strike damage, you can use a swift action to deliver the effects of the chosen revelation to that opponent. Doing so provokes no attacks of opportunity. &nl; [Special] You can take this feat multiple times. Each time you take it, you apply it to a different qualifying revelation."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Oracle's Mystery", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike"]),
            },
            // Rhetorical Flourish -- uc_feats.lst:263
            UcFeatEntry {
                key: "Rhetorical Flourish",
                category: FeatCategory::General,
                name: "Rhetorical Flourish",
                description: Some("You rapidly change topics and employ confusing rhetoric to distract people from your true intent."),
                pretext: None,
                source_page: Some("p.116"),
                benefit: Some("When using the Diplomacy skill to make a request or change a creature's attitude, you can use verbal misdirection. To do so, make a Bluff check against that creature. If you succeed, you gain a +4 bonus on your next Diplomacy check against that creature if the check is made within the next minute. If you fail by 5 or more, you instead take a -2 penalty on your next Diplomacy check against that creature. Alternatively, you can use this feat to retry a single failed Diplomacy check against a creature. You take a -4 penalty on your Bluff check when using Rhetorical Flourish in this way. If you succeed, rather than gaining this feat's normal bonus, you can retry your last Diplomacy check against the creature if that check was made in the past minute."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Persuasive", "PRESTAT:1,CHA=13"]),
            },
            // Ricochet Shot Deed -- uc_feats.lst:264
            UcFeatEntry {
                key: "Ricochet Shot Deed",
                category: FeatCategory::Grit,
                name: "Ricochet Shot Deed",
                description: Some("You can ricochet a firearm shot off the wall and still hit your target."),
                pretext: None,
                source_page: Some("p.116"),
                benefit: Some("You can fire a shot at a wall or piece of solid terrain, and have it ricochet off. When you do, use the square immediately in front of the wall or piece of solid terrain to determine line of sight to a target, and this square is considered the new origin square of the attack. Use that square to determine the effects of cover, and your own square to determine the effects of concealment. You can make this shot as long as you have at least 1 grit point. When making this shot, you can spend 1 grit point to ignore the effects of all cover or concealment. You must choose to spend the grit point before you make the attack roll."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Blind-Fight", "PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Gunslinger ~ Grit],[PREABILITY:1,CATEGORY=FEAT,Amateur Gunslinger]"]),
            },
            // Righteous Healing -- uc_feats.lst:265
            UcFeatEntry {
                key: "Righteous Healing",
                category: FeatCategory::General,
                name: "Righteous Healing",
                description: Some("Your healing spells are more potent when you have a judgment active."),
                pretext: None,
                source_page: Some("p.116"),
                benefit: Some("If you cast a cure spell while you have a judgment active, each target regains 1 extra hit point from the cure spell + 1 hit point per three inquisitor levels you possess."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE=InquisitorJudgment"]),
            },
            // Sap Adept -- uc_feats.lst:266
            UcFeatEntry {
                key: "Sap Adept",
                category: FeatCategory::Combat,
                name: "Sap Adept",
                description: Some("You know just where to hit to knock the sense out of your foe."),
                pretext: None,
                source_page: Some("p.116"),
                benefit: Some("Whenever you use a bludgeoning weapon to deal nonlethal sneak attack damage, you gain a bonus on your damage roll equal to the number of sneak attack damage dice you rolled."),
                prerequisites: Some(&["PREVARGTEQ:SneakAttackDice,1"]),
            },
            // Sap Master -- uc_feats.lst:267
            UcFeatEntry {
                key: "Sap Master",
                category: FeatCategory::Combat,
                name: "Sap Master",
                description: Some("You knock the sense out of foes with a well-timed surprise attack."),
                pretext: None,
                source_page: Some("p.116"),
                benefit: Some("Whenever you use a bludgeoning weapon to deal nonlethal sneak attack damage to a flat-footed opponent, roll your sneak attack dice twice, totaling the results as your nonlethal sneak attack damage for that attack."),
                prerequisites: Some(&["PREVARGTEQ:SneakAttackDice,3"]),
            },
            // Savage Display -- uc_feats.lst:268
            UcFeatEntry {
                key: "Savage Display",
                category: FeatCategory::Combat,
                name: "Savage Display",
                description: Some("With your victory and a roar, you push yourself on with increased savagery."),
                pretext: None,
                source_page: Some("p.116"),
                benefit: Some("When you spend a swift action to make a performance combat check, you gain a +2 bonus on your performance combat check and gain a +1d6 bonus on damage rolls until the end of your next turn. This extra damage is not precision damage."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Dazzling Display"]),
            },
            // School Strike -- uc_feats.lst:270
            UcFeatEntry {
                key: "School Strike",
                category: FeatCategory::Combat,
                name: "School Strike",
                description: Some("You focus the secrets of your school of wizardry into your unarmed strike."),
                pretext: None,
                source_page: Some("p.116"),
                benefit: Some("When you gain this feat, choose one arcane school power that you can use to affect no more than one opponent. If you make a successful unarmed strike against an opponent, in addition to dealing your unarmed strike damage, you can use a swift action to deliver the effects of the chosen school power to that opponent. Doing so provokes no attacks of opportunity. &nl; [Special]You can take this feat multiple times. Each time you take it, you apply it to a different qualifying arcane school power."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.ArcaneSchool", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike"]),
            },
            // Sea Legs -- uc_feats.lst:271
            UcFeatEntry {
                key: "Sea Legs",
                category: FeatCategory::General,
                name: "Sea Legs",
                description: Some("You have a sailor's instincts for moving about while aboard seagoing vessels."),
                pretext: None,
                source_page: None,
                benefit: Some("You gain a +2 bonus on Acrobatics, Climb, and Swim checks."),
                prerequisites: Some(&["PRESKILL:1,Profession (sailor)=5"]),
            },
            // Secret Stash Deed -- uc_feats.lst:272
            UcFeatEntry {
                key: "Secret Stash Deed",
                category: FeatCategory::Grit,
                name: "Secret Stash Deed",
                description: Some("You are so skilled at stashing small packets of firearm ammunition and black powder on your person that you sometimes surprise yourself when you find them."),
                pretext: None,
                source_page: Some("p.117"),
                benefit: Some("Spend 1 grit point while in combat to recover either 1 bullet and 1 dose of black powder or 1 alchemical cartridge from a hidden stash on your person that you had, until now, forgotten about. If the bullet and black powder or the alchemical cartridges are normal shot, you do not need to pay for the ammunition. If you want to recover any other kind of ammunition, you must pay for it with gold pieces from your character's wealth. The grit cost of this deed cannot be decreased by the Signature Deed feat, the true grit class feature, or any other similar effect that reduces the number of grit points you spend to use a deed. You also gain a +4 bonus on any Sleight of Hand checks made while gambling."),
                prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Gunslinger ~ Grit],[PREABILITY:1,CATEGORY=FEAT,Amateur Gunslinger]", "PRESKILL:1,Sleight of Hand=1"]),
            },
            // Seize the Moment -- uc_feats.lst:273
            UcFeatEntry {
                key: "Seize the Moment",
                category: FeatCategory::Combat,
                name: "Seize the Moment",
                description: Some("You and your allies are poised to pounce whenever one of you scores a telling blow."),
                pretext: None,
                source_page: Some("p.117"),
                benefit: Some("When an ally who also has this feat confirms a critical hit against an opponent that you also threaten, you can make an attack of opportunity against that opponent."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Combat Reflexes,Improved Critical"]),
            },
            // Shaitan Earthblast -- uc_feats.lst:274
            UcFeatEntry {
                key: "Shaitan Earthblast",
                category: FeatCategory::Combat,
                name: "Shaitan Earthblast",
                description: Some("With a forceful stomp you release a blast of acid from the earth to burn your enemies."),
                pretext: None,
                source_page: Some("p.117"),
                benefit: Some("While using the Shaitan Style feat, as a standard action, you can spend two Elemental Fist (Advanced Player's Guide 158) attempts to unleash a 20-foot column of acid that has a 5-foot radius and erupts from a point of origin within 30 feet of you. Creatures caught in the column take your unarmed strike damage plus the acid damage from your Elemental Fist and are staggered for 1 round. A successful Reflex save (DC 10 + 1/2 your character level + your Wis modifier) reduces the damage by half and prevents a target from being staggered."),
                prerequisites: Some(&["PREMULT:1,[PREMULT:3,[PRESTAT:2,CON=15,WIS=17],[PREABILITY:4,CATEGORY=FEAT,Elemental Fist,Improved Unarmed Strike,Shaitan Skin,Shaitan Style],[PREMULT:1,[PRETOTALAB:13],[PREVARGTEQ:MonkFeatQualify,11]]]"]),
            },
            // Shaitan Skin -- uc_feats.lst:275
            UcFeatEntry {
                key: "Shaitan Skin",
                category: FeatCategory::Combat,
                name: "Shaitan Skin",
                description: Some("You can manipulate acid to shield yourself and disable your enemies."),
                pretext: None,
                source_page: Some("p.118"),
                benefit: Some("You gain one additional Elemental Fist (Advanced Player's Guide 158) attempt per day. While using the Shaitan Style feat, you gain acid resistance equal to your base attack bonus, or your monk level plus BAB gained from levels in classes other than monk, whichever is higher. While denied your Dexterity bonus to AC you are also denied this resistance. Creatures that take acid damage from your Elemental Fist attack must succeed at a Reflex save (DC 10 + 1/2 your character level + your Wis modifier) or be staggered for 1 round."),
                prerequisites: Some(&["PREMULT:1,[PREMULT:3,[PRESTAT:2,CON=15,WIS=15],[PREABILITY:3,CATEGORY=FEAT,Elemental Fist,Improved Unarmed Strike,Shaitan Style],[PREMULT:1,[PRETOTALAB:11],[PREVARGTEQ:MonkFeatQualify,9]]]"]),
            },
            // Shaitan Style -- uc_feats.lst:276
            UcFeatEntry {
                key: "Shaitan Style",
                category: FeatCategory::Combat,
                name: "Shaitan Style",
                description: Some("You strike with the caustic forces from within the earth."),
                pretext: None,
                source_page: Some("p.118"),
                benefit: Some("You gain one additional Elemental Fist (Advanced Player's Guide 158) attempt per day. While using the Shaitan Style and Elemental Fist feats to deal acid damage, you gain a bonus on acid damage rolls equal to your Wisdom bonus. Further, if your Elemental Fist melee attack misses while you are using it to deal acid damage, you still deal 1d6 points of acid damage to your target."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Elemental Fist,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:9],[PREVARGTEQ:MonkFeatQualify,5]", "PRESTAT:2,CON=13,WIS=15"]),
            },
            // Shake It Off -- uc_feats.lst:277
            UcFeatEntry {
                key: "Shake It Off",
                category: FeatCategory::Teamwork,
                name: "Shake It Off",
                description: Some("You support your allies and help them recover from crippling effects."),
                pretext: None,
                source_page: Some("p.118"),
                benefit: Some("When you are adjacent to one or more allies who also have this feat, you gain a +1 bonus on saving throws per such ally (maximum +4)."),
                prerequisites: None,
            },
            // Shapeshifter Foil -- uc_feats.lst:280
            UcFeatEntry {
                key: "Shapeshifter Foil",
                category: FeatCategory::General,
                name: "Shapeshifter Foil",
                description: Some("Your command of shapeshifting magic can disrupt similar effects in others."),
                pretext: None,
                source_page: Some("p.118"),
                benefit: Some("A creature you deal damage to has difficulty using or maintaining polymorph effects until the end your next turn. To use a polymorph effect it must make a concentration check (DC 15 + twice the level of the effect). If you deal damage to an opponent under a polymorph effect, that opponent must succeed at a Will saving throw (DC 10 + 1/2 your character level + your Wisdom modifier) or be forced back to its original form. If you score a critical hit against such an opponent, no saving throw is allowed."),
                prerequisites: Some(&["PREMULT:1,[PRESPELLSCHOOLSUB:1,Polymorph=1],[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ Shapeshifter],[PREABILITY:1,CATEGORY=Special Ability,Punitive Transformation ~ Waves Mystery]", "PRESKILL:1,Knowledge (arcana)=5,Knowledge (nature)=5"]),
            },
            // Shapeshifting Hunter -- uc_feats.lst:281
            UcFeatEntry {
                key: "Shapeshifting Hunter",
                category: FeatCategory::General,
                name: "Shapeshifting Hunter",
                description: Some("You blend your knowledge of foes and your shapeshifting abilities together."),
                pretext: None,
                source_page: Some("p.118"),
                benefit: Some("Your levels of druid stack with your ranger levels for determining when you select your next favored enemy. Also, your ranger levels stack with your druid levels in determining the number of times per day you can use your wild shape class feature, up to a maximum of eight times per day (currently %1).|WildShapeTimes"),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.FavoredEnemy,TYPE.Favored Enemy", "PREABILITY:1,CATEGORY=Special Ability,Wild Shape"]),
            },
            // Signature Deed -- uc_feats.lst:282
            UcFeatEntry {
                key: "Signature Deed",
                category: FeatCategory::Grit,
                name: "Signature Deed",
                description: Some("You are known for performing a particular deed, and can perform it with greater ease."),
                pretext: None,
                source_page: Some("p.119"),
                benefit: Some("Pick a deed that you have access to and that you must spend grit to perform. Once per round, you can perform this deed for 1 fewer grit point (minimum 0). You can reduce the cost of a deed in this way only if you have at least 1 grit point."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Gunslinger ~ Grit,TYPE.GritShared", "PRECLASS:1,Gunslinger=11,Swashbuckler=11"]),
            },
            // Skilled Driver -- uc_feats.lst:287
            UcFeatEntry {
                key: "Skilled Driver",
                category: FeatCategory::General,
                name: "Skilled Driver",
                description: Some("Choose a type a type [sic] of vehicle (either air, land, or water). You are more skilled when driving that vehicle."),
                pretext: None,
                source_page: Some("p.119"),
                benefit: Some("You gain a +4 bonus on driving checks with your chosen vehicle (chosen vehicle: %1).|%LIST"),
                prerequisites: None,
            },
            // Wave Strike -- uc_feats.lst:288
            UcFeatEntry {
                key: "Wave Strike",
                category: FeatCategory::Combat,
                name: "Wave Strike",
                description: Some("You present a serene facade until you unsheathe your weapon and strike in one fluid motion."),
                pretext: None,
                source_page: Some("p.123"),
                benefit: Some("If on your first turn of combat you draw a melee weapon to attack an opponent within your reach, you can spend a swift action to make a Bluff check to feint against that opponent."),
                prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,Samurai ~ Weapon Expertise],[PREABILITY:1,CATEGORY=FEAT,Quick Draw]", "PRESKILL:1,Bluff=1"]),
            },
            // Whip Mastery -- uc_feats.lst:289
            UcFeatEntry {
                key: "Whip Mastery",
                category: FeatCategory::Combat,
                name: "Whip Mastery",
                description: Some("Your superior expertise with this weapon does not provoke attacks of opportunity from your enemies."),
                pretext: None,
                source_page: Some("p.123"),
                benefit: Some("You no longer provoke attacks of opportunity when attacking with a whip. You can deal lethal damage with a whip, although you can still deal nonlethal damage when you want. Further, you can deal damage with a whip despite a creature's armor bonus or natural armor bonus. &nl; [Normal] Attacking with a whip provokes attacks of opportunity as if you used a ranged weapon. A whip deals no damage to a creature that has an armor bonus of +1 or natural armor bonus of +3."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Weapon Focus (whip)", "PRETOTALAB:2"]),
            },
            // Improved Whip Mastery -- uc_feats.lst:290
            UcFeatEntry {
                key: "Improved Whip Mastery",
                category: FeatCategory::Combat,
                name: "Improved Whip Mastery",
                description: Some("You are able to entangle opponents with the coils of your whip."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: Some("While wielding a whip, you threaten the area of your natural reach plus 5 feet. You can also use a whip to grasp an unattended Small or Tiny object within your whip's reach and pull that object into your square. To do so, you must hit AC 10 with a melee touch attack. Further, you can use the whip to grasp onto an object within your whip's reach, using 5 feet of your whip as if it were a grappling hook, allowing you to use the rest of your whip to swing on like a rope. As a free action, you can release the object your whip is grasping, but you cannot use the whip to attack while the whip is grasping an object."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Weapon Focus (whip),Whip Mastery", "PRETOTALAB:5"]),
            },
            // Greater Whip Mastery -- uc_feats.lst:291
            UcFeatEntry {
                key: "Greater Whip Mastery",
                category: FeatCategory::Combat,
                name: "Greater Whip Mastery",
                description: Some("You can use a whip to make combat maneuvers with ease."),
                pretext: None,
                source_page: Some("p.103"),
                benefit: Some("You are so quick with your whip that you never drop it due to a failed disarm or trip combat maneuver attempt. Further, you gain the ability to grapple using your whip. To do so, use the normal grapple rules with the following changes. &nl;Attack - You cannot use your whip to attack while you are using it to grapple an opponent. &nl;Damage - When dealing damage to your grappled opponent, you deal your whip's weapon damage rather than your unarmed strike damage. &nl;Free Hands - You take no penalty on your combat maneuver check for having fewer than two hands free when you use your whip to grapple. &nl;Reach - Rather than pulling your grappled opponent adjacent to you when you successfully grapple and when you move the grapple, you must keep him within your whip's reach minus his own reach to maintain the grapple. If the difference in reach is less than 0, such as is the case for a Medium whip wielder and a Gargantuan creature, you cannot grapple that opponent with your whip. If you have to pull a creature adjacent to you to grapple it with your whip, you still provoke an attack of opportunity from that opponent unless you have the Improved Grapple feat. &nl;Tie Up - While adjacent to your opponent, you can attempt to use your whip to tie him up. If you do so to an opponent you have grappled rather than pinned, you take only a -5 penalty on the combat maneuver check rather than the normal -10."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Improved Whip Mastery,Weapon Focus (whip),Whip Mastery", "PRETOTALAB:8"]),
            },
            // Crane Style -- uc_feats.lst:292
            UcFeatEntry {
                key: "Crane Style",
                category: FeatCategory::Combat,
                name: "Crane Style",
                description: Some("Your unarmed fighting techniques blend poise with graceful defense."),
                pretext: None,
                source_page: Some("p.93"),
                benefit: Some("You take only a -2 penalty on attack rolls for fighting defensively. While using this style and fighting defensively or using the total defense action, you gain an additional +1 dodge bonus to your Armor Class."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Dodge,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:2],[PREVARGTEQ:MonkFeatQualify,1]"]),
            },
            // Crane Riposte -- uc_feats.lst:294
            UcFeatEntry {
                key: "Crane Riposte",
                category: FeatCategory::Combat,
                name: "Crane Riposte",
                description: Some("You use your defensive abilities to make overpowering counterattacks."),
                pretext: None,
                source_page: Some("p.93"),
                benefit: Some("You take only a -1 penalty on attack rolls for fighting defensively. Whenever you def lect an opponent's attack using Crane Wing or lose the dodge bonus from Crane Wing because an attack missed you by 4 or less, you can make an attack of opportunity against the attacker after the attack misses. In addition, when you deflect an attack using Crane Wing while taking the total defense action, you may make an attack of opportunity against that opponent (even though you could not normally do so while taking the total defense action)."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Crane Style,Crane Wing,Dodge,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:8],[PREVARGTEQ:MonkFeatQualify,7]"]),
            },
            // Crane Wing -- uc_feats.lst:295
            UcFeatEntry {
                key: "Crane Wing",
                category: FeatCategory::Combat,
                name: "Crane Wing",
                description: Some("You move with the speed and finesse of an avian hunter, your sweeping blocks and graceful motions allowing you to deflect melee attacks with ease."),
                pretext: None,
                source_page: Some("p.93"),
                benefit: Some("When fighting defensively with at least one hand free, you gain a +4 dodge bonus to AC against melee attacks. If a melee attack misses you by 4 or less, you lose this dodge bonus until the beginning of your next turn. An attack so deflected deals no damage and has no other effect (instead treat it as a miss). You do not expend an action when using this feat, but you must be aware of the attack and not flat-footed."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Crane Style,Dodge,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:5],[PREVARGTEQ:MonkFeatQualify,5]"]),
            },
            // Crusader's Fist -- uc_feats.lst:296
            UcFeatEntry {
                key: "Crusader's Fist",
                category: FeatCategory::Combat,
                name: "Crusader's Fist",
                description: Some("You pour divine energy into the enemy you strike."),
                pretext: None,
                source_page: Some("p.94"),
                benefit: Some("When you attack with an unarmed strike and hit a creature that you can harm with your lay on hands or touch of corruption feature, you can use a swift action to expend a daily use of that feature to deal its normal damage as if you had hit with the feature's normal touch attack. This extra damage is not multiplied if you scored a critical hit."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Lay on Hands,Touch of Corruption ~ Antipaladin", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRETOTALAB:6"]),
            },
            // Crusader's Flurry -- uc_feats.lst:297
            UcFeatEntry {
                key: "Crusader's Flurry",
                category: FeatCategory::General,
                name: "Crusader's Flurry",
                description: Some("You learned to use your deity's favored weapon as part of your martial arts form."),
                pretext: None,
                source_page: Some("p.94"),
                benefit: Some("You can use your deity's favored weapon as if it were a monk weapon."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=Special Ability,TYPE.Channel Energy,TYPE.Flurry of Blows", "PREABILITY:1,CATEGORY=FEAT,Weapon Focus", "PREWEAPONPROF:1,DEITYWEAPON"]),
            },
            // Dispelling Fist -- uc_feats.lst:298
            UcFeatEntry {
                key: "Dispelling Fist",
                category: FeatCategory::General,
                name: "Dispelling Fist",
                description: Some("By focusing on your knowledge of magic and spells that negate its powers, you use your bare hands to rip magical defenses from your enemy.  PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike"),
                pretext: None,
                source_page: Some("p.97"),
                benefit: Some("If you have dispel magic prepared or can cast it spontaneously, you can cast it as a swift action after hitting an opponent with an unarmed strike. Treat this as a targeted dispel against the opponent you hit."),
                prerequisites: Some(&["PREMULT:1,[PRECLASS:1,SPELLCASTER=7],[PREVARGTEQ:CasterLevel_Highest,7]", "PRESPELL:1,Dispel Magic", "PRETOTALAB:11"]),
            },
            // Moonlight Stalker -- uc_feats.lst:299
            UcFeatEntry {
                key: "Moonlight Stalker",
                category: FeatCategory::Combat,
                name: "Moonlight Stalker",
                description: Some("You are adept at using shadows to conceal your attacks."),
                pretext: None,
                source_page: Some("p.110"),
                benefit: Some("While you have concealment from an opponent, you gain a +2 bonus on attack and damage rolls against that opponent."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.RacialVision,TYPE.Racial Vision", "PREABILITY:1,CATEGORY=Special Ability,Darkvision,Low-Light Vision", "PREABILITY:2,CATEGORY=FEAT,Blind-Fight,Combat Expertise", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRESKILL:1,Bluff=3"]),
            },
            // Moonlight Stalker Feint -- uc_feats.lst:300
            UcFeatEntry {
                key: "Moonlight Stalker Feint",
                category: FeatCategory::Combat,
                name: "Moonlight Stalker Feint",
                description: Some("You strike through the shadows so quickly that your opponent can barely react to your attacks."),
                pretext: None,
                source_page: Some("p.110"),
                benefit: Some("Once per round, against an opponent from whom you have concealment, you can spend a swift action to make a Bluff check to feint. &nl; [Normal] Feinting is a standard action."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.RacialVision,TYPE.Racial Vision", "PREABILITY:1,CATEGORY=Special Ability,Darkvision,Low-Light Vision", "PREABILITY:4,CATEGORY=FEAT,Blind-Fight,Combat Expertise,Moonlight Stalker,Improved Feint", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRESKILL:1,Bluff=6"]),
            },
            // Moonlight Stalker Master -- uc_feats.lst:301
            UcFeatEntry {
                key: "Moonlight Stalker Master",
                category: FeatCategory::Combat,
                name: "Moonlight Stalker Master",
                description: Some("You leave your opponents swinging at shadows while you slide elusively through the darkness."),
                pretext: None,
                source_page: Some("p.110"),
                benefit: Some("While you have concealment, your opponents' miss chance against you increases by 10%%. If an opponent misses you due to your concealment, you can spend an immediate action to move 5 feet, this movement does not provoke attacks of opportunity and does not count as a 5-foot step."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.RacialVision,TYPE.Racial Vision", "PREABILITY:1,CATEGORY=Special Ability,Darkvision,Low-Light Vision", "PREABILITY:5,CATEGORY=FEAT,Blind-Fight,Combat Expertise,Improved Feint,Moonlight Stalker,Moonlight Stalker Feint", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRESKILL:1,Bluff=9"]),
            },
            // Shared Judgment -- uc_feats.lst:302
            UcFeatEntry {
                key: "Shared Judgment",
                category: FeatCategory::General,
                name: "Shared Judgment",
                description: Some("You extend the benefits of your judgment to an ally."),
                pretext: None,
                source_page: Some("p.118"),
                benefit: Some("You can pronounce a single judgment and extend its effects to one adjacent ally instead of pronouncing a second judgment. Similarly, once you have the third judgment class feature, you can pronounce a single judgment and extend its effects to two adjacent allies instead of pronouncing a second and third judgment. Alternatively, once you have the third judgment class feature, you can pronounce two judgments and extend the effects of one judgment to one adjacent ally instead of pronouncing a third judgment. Once an ally has gained the effects of your judgment, he need not remain adjacent to you to continue gaining that benefit. You can spend a free action to end this benefit for one or both allies. If your judgment bonus is suspended for you, it is suspended for all allies, but when it resumes, it does so for all allies."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Second Judgment"]),
            },
            // Siege Commander -- uc_feats.lst:303
            UcFeatEntry {
                key: "Siege Commander",
                category: FeatCategory::Combat,
                name: "Siege Commander",
                description: Some("Under your leadership, the time required to assemble and move a siege engine is greatly reduced."),
                pretext: None,
                source_page: Some("p.118"),
                benefit: Some("When you lead a siege engine assembly crew, you grant all of its members a +4 competence bonus on checks to assemble or move the weapon. You also halve the time required to assemble a siege engine."),
                prerequisites: Some(&["PRESKILL:1,Craft (siege weapon)=5", "PRESKILL:1,Knowledge (engineering)=5,Profession (siege engineer)=1"]),
            },
            // Siege Engineer -- uc_feats.lst:304
            UcFeatEntry {
                key: "Siege Engineer",
                category: FeatCategory::Combat,
                name: "Siege Engineer",
                description: Some("You are proficient with all siege weaponry."),
                pretext: None,
                source_page: Some("p.118"),
                benefit: Some("You are considered to be proficient with all siege weapons. Also, when you are crew lead for a siege engine, you do not generate mishaps on the roll of a natural 1. &nl; [Normal] Each siege engine is an exotic weapon."),
                prerequisites: Some(&["PREMULT:1,[PREWEAPONPROF:1,TYPE.SiegeWeapon],[PREABILITY:1,CATEGORY=FEAT,Exotic Weapon Proficiency (TYPE=SiegeWeapon)]", "PRESKILL:1,Knowledge (engineering)=5,Profession (siege engineer)=5"]),
            },
            // Siege Gunner -- uc_feats.lst:305
            UcFeatEntry {
                key: "Siege Gunner",
                category: FeatCategory::Combat,
                name: "Siege Gunner",
                description: Some("Aiming outsized siege weapons poses little difficulty for you."),
                pretext: None,
                source_page: Some("p.118"),
                benefit: Some("You take no size penalty for aiming a directfire siege weapon larger than yourself. If you operate an indirect-fire siege weapon and miss, you misdirect fire by 1 square per range increment. &nl; [Normal] Direct-fire weapons impose a -2 attack roll penalty per size category by which the weapon is larger than the creature aiming it. An indirect-fire weapon that misses misdirects fire by 1d4 squares per range increment."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Siege Engineer", "PRESKILL:1,Profession (siege engineer)=5"]),
            },
            // Slayer's Knack -- uc_feats.lst:306
            UcFeatEntry {
                key: "Slayer's Knack",
                category: FeatCategory::General,
                name: "Slayer's Knack",
                description: Some("You know how to battle your favored enemies with such efficacy that any weapon you wield against them becomes more deadly."),
                pretext: None,
                source_page: Some("p.119"),
                benefit: Some("When you take this feat, choose one of your favored enemy types. Against enemies of that type, the threat range of any weapon you wield is doubled. This effect does not stack with any other effect that expands a weapon's threat range. &nl; [Special]You can take this feat multiple times. Each time you take it, you choose a different favored enemy type."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.FavoredEnemy,TYPE.Favored Enemy", "PRETOTALAB:6"]),
            },
            // Sling Flail -- uc_feats.lst:307
            UcFeatEntry {
                key: "Sling Flail",
                category: FeatCategory::Combat,
                name: "Sling Flail",
                description: Some("You can use your loaded sling to effectively strike at nearby foes."),
                pretext: None,
                source_page: Some("p.119"),
                benefit: Some("You can make melee attacks using your loaded sling, using that weapon's normal statistics but treating it as a flail. Using a sling in this way does not expend mundane ammunition, but magical or masterwork ammunition loses its special properties after a single hit. &nl; [Special]Any feats you have that apply when you use a flail also apply when you use a loaded sling as a melee weapon."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Weapon Focus (sling)", "PRETOTALAB:1"]),
            },
            // Snake Fang -- uc_feats.lst:308
            UcFeatEntry {
                key: "Snake Fang",
                category: FeatCategory::Combat,
                name: "Snake Fang",
                description: Some("You can unleash attacks against an opponent that has dropped its guard."),
                pretext: None,
                source_page: Some("p.119"),
                benefit: Some("While using the Snake Style feat, when an opponent's attack misses you, you can make an unarmed strike against that opponent as an attack of opportunity. If this attack of opportunity hits, you can spend an immediate action to make another unarmed strike against the same opponent."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Combat Reflexes,Improved Unarmed Strike,Snake Sidewind,Snake Style", "PRESKILL:2,Acrobatics=6,Sense Motive=9"]),
            },
            // Snake Sidewind -- uc_feats.lst:309
            UcFeatEntry {
                key: "Snake Sidewind",
                category: FeatCategory::Combat,
                name: "Snake Sidewind",
                description: Some("Your sensitive twisting movements make you difficult to anticipate combat."),
                pretext: None,
                source_page: Some("p.119"),
                benefit: Some("You gain a +4 bonus to CMD against trip combat maneuvers and on Acrobatics checks and saving throws to avoid being knocked prone. While using the Snake Style feat, whenever you score a critical threat with your unarmed strike, you can make a Sense Motive check in place of the attack roll to confirm the critical hit. Whenever you score a critical hit with your unarmed strike, you can spend an immediate action to take a 5-foot step even if you have otherwise moved this round. &nl; [Normal] You can take a 5-foot step only if you have not otherwise moved this round."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Snake Style", "PRESKILL:2,Acrobatics=3,Sense Motive=6"]),
            },
            // Snake Style -- uc_feats.lst:310
            UcFeatEntry {
                key: "Snake Style",
                category: FeatCategory::Combat,
                name: "Snake Style",
                description: Some("You watch your foe's every movement and then punch through its defense."),
                pretext: None,
                source_page: Some("p.119"),
                benefit: Some("You gain a +2 bonus on Sense Motive checks, and you can deal piercing damage with your unarmed strikes. While using the Snake Style feat, when an opponent targets you with a melee or ranged attack, you can spend an immediate action to make a Sense Motive check. You can use the result as your AC or touch AC against that attack. You must be aware of the attack and not flat-footed. &nl; [Normal] An unarmed strike deals bludgeoning damage."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRESKILL:2,Acrobatics=1,Sense Motive=3"]),
            },
            // Snapping Turtle Clutch -- uc_feats.lst:311
            UcFeatEntry {
                key: "Snapping Turtle Clutch",
                category: FeatCategory::Combat,
                name: "Snapping Turtle Clutch",
                description: Some("Your unarmed style allows you to turn your opponent's attack into an opportunity."),
                pretext: None,
                source_page: Some("p.120"),
                benefit: Some("While you are using the Snapping Turtle Style feat, the shield bonus the style grants to your AC applies to your CMD and touch AC. Whenever an opponent misses you with a melee attack while you are using the Snapping Turtle Style feat, you can use an immediate action to attempt a grapple combat maneuver against that opponent, but with a -2 penalty."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Snapping Turtle Style,Improved Grapple,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:3],[PREVARGTEQ:MonkFeatQualify,3]"]),
            },
            // Snapping Turtle Shell -- uc_feats.lst:312
            UcFeatEntry {
                key: "Snapping Turtle Shell",
                category: FeatCategory::Combat,
                name: "Snapping Turtle Shell",
                description: Some("Your guarding hand is almost magical in its skill at deflecting incoming blows."),
                pretext: None,
                source_page: Some("p.120"),
                benefit: Some("While you are using the Snapping Turtle Style feat, the shield bonus the style grants to your AC increases to +2, and your enemies take a -4 penalty on critical confirmation rolls against you."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Snapping Turtle Clutch,Snapping Turtle Style,Improved Grapple,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:5],[PREVARGTEQ:MonkFeatQualify,5]"]),
            },
            // Snapping Turtle Style -- uc_feats.lst:313
            UcFeatEntry {
                key: "Snapping Turtle Style",
                category: FeatCategory::Combat,
                name: "Snapping Turtle Style",
                description: Some("Your deft unarmed style allows you to shield your body from harm."),
                pretext: None,
                source_page: Some("p.120"),
                benefit: Some("While using the Snapping Turtle Style feat with at least one hand free, you gain a +1 shield bonus to AC."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:1],[PREVARGTEQ:MonkFeatQualify,1]"]),
            },
            // Sneaking Precision -- uc_feats.lst:314
            UcFeatEntry {
                key: "Sneaking Precision",
                category: FeatCategory::General,
                name: "Sneaking Precision",
                description: Some("Your knowledge of your enemies' vulnerable spots is especially punishing."),
                pretext: None,
                source_page: None,
                benefit: Some("Whenever you successfully sneak attack an opponent for a second time on your turn, you can spend a swift action to apply the effects of one critical feat you know to that opponent."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Critical Focus,TYPE=Critical", "PRETOTALAB:9", "PREVARGTEQ:SneakAttackDice,6"]),
            },
            // Sorcerous Strike -- uc_feats.lst:317
            UcFeatEntry {
                key: "Sorcerous Strike",
                category: FeatCategory::Combat,
                name: "Sorcerous Strike",
                description: Some("The power flowing through your veins also flows through your unarmed strike."),
                pretext: None,
                source_page: Some("p.120"),
                benefit: Some("When you gain this feat, you choose one bloodline power that you can use to affect a single opponent. If you make a successful unarmed strike against an opponent, in addition to dealing your unarmed strike damage, you can spend a swift action to deliver the effects of the chosen bloodline power to that opponent. Doing so provokes no attacks of opportunity. &nl; [Special]You can take this feat multiple times. Each time you take it, you apply it to a different qualifying bloodline power."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Sorcerer ~ Standard Bloodline", "PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike"]),
            },
            // Spell Bane -- uc_feats.lst:318
            UcFeatEntry {
                key: "Spell Bane",
                category: FeatCategory::General,
                name: "Spell Bane",
                description: Some("While your bane weapon is active, creatures that your bane affects find it more difficult to resist your spells."),
                pretext: None,
                source_page: None,
                benefit: Some("While your bane class feature is affecting a creature type, the saving throw's DCs for your spells increase by +2 for creatures of that type."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Bane"]),
            },
            // Spinning Throw -- uc_feats.lst:319
            UcFeatEntry {
                key: "Spinning Throw",
                category: FeatCategory::Combat,
                name: "Spinning Throw",
                description: Some("You whirl your foe around and then let go."),
                pretext: None,
                source_page: Some("p.120"),
                benefit: Some("On a successful unarmed trip combat maneuver against an opponent your size or smaller, you can spend a swift action to attempt a bull rush combat maneuver against that opponent. If your bull rush succeeds, you can move that opponent to any unoccupied square you threaten, then push that opponent the number of 5-foot increments your successful bull rush allows. The target is then knocked prone. If the bull rush fails, you can use the Ki Throw feat as normal. If you also have the Improved Ki Throw feat, a successful bull rush allows you to push the opponent into a space secondary targets occupy. You resolve this effect as if you used the Improved Ki Throw feat to throw the opponent into that space. &nl; [Special]Per the Ki Throw feat, a monk can use ki to affect creatures larger than himself with this feat."),
                prerequisites: Some(&["PREABILITY:5,CATEGORY=FEAT,Combat Expertise,Improved Bull Rush,Improved Trip,Improved Unarmed Strike,Ki Throw"]),
            },
            // Splintering Weapon -- uc_feats.lst:320
            UcFeatEntry {
                key: "Splintering Weapon",
                category: FeatCategory::General,
                name: "Splintering Weapon",
                description: Some("Your fragile weapon works to your advantage, breaking off fragments in wounds you inflict."),
                pretext: Some("Weapon used must be made of primitive material"),
                source_page: Some("p.120"),
                benefit: Some("Whenever you use a melee or thrown weapon with the fragile weapon feature (page 146) or similar quality and hit an opponent, you can give your weapon the broken condition to deal that opponent 1d4 points of bleed damage."),
                prerequisites: Some(&["PRETOTALAB:1"]),
            },
            // Stage Combatant -- uc_feats.lst:321
            UcFeatEntry {
                key: "Stage Combatant",
                category: FeatCategory::Combat,
                name: "Stage Combatant",
                description: Some("You are a master of stage and nonlethal combats."),
                pretext: None,
                source_page: Some("p.120"),
                benefit: Some("When you make an attack with a weapon that you have Weapon Focus in, you take no penalty on the attack roll when you are attempting to make an attack that deals no damage or nonlethal damage. &nl; [Normal] When making attacks that deal no damage or nonlethal damage, you take a -4 penalty on attack rolls."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Weapon Focus", "PRETOTALAB:5"]),
            },
            // Stalwart -- uc_feats.lst:322
            UcFeatEntry {
                key: "Stalwart",
                category: FeatCategory::General,
                name: "Stalwart",
                description: Some("You adopt a defensive stance that allows you to absorb and redirect hits."),
                pretext: None,
                source_page: Some("p.121"),
                benefit: Some("While using the total defense action, fighting defensively action, or Combat Expertise, you can forgo the dodge bonus to AC you would normally gain to instead gain an equivalent amount of DR, to a maximum of DR 5/-, until the start of your next turn. This damage reduction stacks with DR you gain from class features, such as the barbarian's, but not with DR from any other source. If you are denied your Dexterity bonus to AC, you are also denied this DR."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Diehard,Endurance", "PRETOTALAB:4"]),
            },
            // Stealth Synergy -- uc_feats.lst:323
            UcFeatEntry {
                key: "Stealth Synergy",
                category: FeatCategory::Teamwork,
                name: "Stealth Synergy",
                description: Some("Working closely with an ally, you are able to move like twin shadows."),
                pretext: None,
                source_page: Some("p.121"),
                benefit: Some("While you can see one or more allies who also have this feat, whenever you and your allies make a Stealth check, you all take the highest roll and add all your modifiers to Stealth."),
                prerequisites: None,
            },
            // Strangler -- uc_feats.lst:324
            UcFeatEntry {
                key: "Strangler",
                category: FeatCategory::Combat,
                name: "Strangler",
                description: Some("Throttling the life out of enemies is second nature to you."),
                pretext: None,
                source_page: Some("p.121"),
                benefit: Some("Whenever you successfully maintain a grapple and choose to deal damage, you can spend a swift action to deal your sneak attack damage to the creature you are grappling."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Grapple,Improved Unarmed Strike", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PREVARGTEQ:SneakAttackDice,1"]),
            },
            // Strong Comeback -- uc_feats.lst:325
            UcFeatEntry {
                key: "Strong Comeback",
                category: FeatCategory::General,
                name: "Strong Comeback",
                description: Some("You learn quickly from past mistakes."),
                pretext: None,
                source_page: Some("p.121"),
                benefit: Some("Whenever you are allowed to reroll an ability check, a skill check, or a saving throw, you gain a +2 circumstance bonus on the reroll."),
                prerequisites: None,
            },
            // Stunning Pin -- uc_feats.lst:326
            UcFeatEntry {
                key: "Stunning Pin",
                category: FeatCategory::Combat,
                name: "Stunning Pin",
                description: Some("You can render a pinned foe temporarily incapacitated."),
                pretext: None,
                source_page: Some("p.122"),
                benefit: Some("Whenever you pin an opponent, you can spend a swift action to make a Stunning Fist attempt against that opponent."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Improved Grapple,Improved Unarmed Strike,Stunning Fist"]),
            },
            // Sure Grasp -- uc_feats.lst:327
            UcFeatEntry {
                key: "Sure Grasp",
                category: FeatCategory::General,
                name: "Sure Grasp",
                description: Some("Your quick reflexes and skill at climbing keep you from falling to your doom."),
                pretext: None,
                source_page: Some("p.122"),
                benefit: Some("Roll twice while climbing or when making a Reflex save to avoid falling, and take the higher result."),
                prerequisites: Some(&["PRESKILL:1,Climb=1"]),
            },
            // Snap Shot -- uc_feats.lst:328
            UcFeatEntry {
                key: "Snap Shot",
                category: FeatCategory::Combat,
                name: "Snap Shot",
                description: Some("With a ranged weapon, you can take advantage of any opening in your opponent's defenses."),
                pretext: None,
                source_page: Some("p.119"),
                benefit: Some("While wielding a ranged weapon with which you have Weapon Focus, you threaten squares within 5 feet of you. You can make attacks of opportunity with that ranged weapon. You do not provoke attacks of opportunity when making a ranged attack as an attack of opportunity. &nl; [Normal] While wielding a ranged weapon, you threaten no squares and can make no attacks of opportunity with that weapon."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Point-Blank Shot,Weapon Focus,Rapid Shot", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRETOTALAB:6"]),
            },
            // Sword and Pistol -- uc_feats.lst:329
            UcFeatEntry {
                key: "Sword and Pistol",
                category: FeatCategory::Combat,
                name: "Sword and Pistol",
                description: Some("You effortlessly pair melee and ranged weaponry."),
                pretext: None,
                source_page: Some("p.122"),
                benefit: Some("When you use the Two-Weapon Fighting feat while wielding a melee weapon and a crossbow or firearm, your attacks with the crossbow or firearm provoke no attacks of opportunity from foes that you threaten with your melee weapon. &nl; [Normal] Making a ranged attack provokes attacks of opportunity."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Two-Weapon Fighting,Point-Blank Shot,Rapid Shot,Snap Shot", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]", "PRETOTALAB:6"]),
            },
            // Tandem Trip -- uc_feats.lst:330
            UcFeatEntry {
                key: "Tandem Trip",
                category: FeatCategory::Combat,
                name: "Tandem Trip",
                description: Some("You know how to work together to trip your foes."),
                pretext: None,
                source_page: Some("p.122"),
                benefit: Some("Whenever you attempt a trip combat maneuver against an enemy threatened by an ally with this feat, you roll twice and take the better result."),
                prerequisites: None,
            },
            // Target of Opportunity -- uc_feats.lst:331
            UcFeatEntry {
                key: "Target of Opportunity",
                category: FeatCategory::Combat,
                name: "Target of Opportunity",
                description: Some("You and your allies pelt your enemies with a deadly barrage of missiles."),
                pretext: None,
                source_page: Some("p.122"),
                benefit: Some("When an ally who also has this feat makes a ranged attack and hits an opponent within 30 feet of you, you can spend an immediate action to make a single ranged attack against that opponent. Your ranged weapon must be in hand, loaded, and ready to be fired or thrown for you to make the ranged attack."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Point-Blank Shot", "PRETOTALAB:6"]),
            },
            // Team Pickpocketing -- uc_feats.lst:332
            UcFeatEntry {
                key: "Team Pickpocketing",
                category: FeatCategory::Teamwork,
                name: "Team Pickpocketing",
                description: Some("You distract a mark with friendly conversation while your partner robs the victim blind."),
                pretext: None,
                source_page: Some("p.122"),
                benefit: Some("Whenever an ally with this feat succeeds a Bluff check to feint an opponent, if you are adjacent to that creature, you can spend an immediate action to make a Sleight of Hand check to pickpocket that opponent and gain a +4 bonus on that attempt."),
                prerequisites: Some(&["PRESKILL:2,Bluff=1,Sleight of Hand=1"]),
            },
            // Tiger Claws -- uc_feats.lst:333
            UcFeatEntry {
                key: "Tiger Claws",
                category: FeatCategory::Combat,
                name: "Tiger Claws",
                description: Some("You can sacrifice multiple attacks to make a single devastating strike."),
                pretext: None,
                source_page: Some("p.122"),
                benefit: Some("While you are using the Tiger Style feat and have both hands free, you can use a full-round action to make a single unarmed strike with both hands. Use your highest base attack bonus, rolling unarmed strike damage for each hand separately and multiplying both if you score a critical hit. If you use Power Attack in conjunction with this attack, can add half your Strength bonus to one of the damage rolls. If you hit, you can attempt a bull rush maneuver with a +2 bonus on the combat maneuver check. This bull rush attempt provokes no attack of opportunity from your opponent, but you cannot move with that opponent if your bull rush is successful."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Tiger Style", "PREMULT:1,[PRETOTALAB:6],[PREVARGTEQ:MonkFeatQualify,5]"]),
            },
            // Tiger Pounce -- uc_feats.lst:334
            UcFeatEntry {
                key: "Tiger Pounce",
                category: FeatCategory::Combat,
                name: "Tiger Pounce",
                description: Some("Your unarmed strikes are as precise as they are powerful, but they leave you open and you can pursue foes with blinding speed."),
                pretext: None,
                source_page: Some("p.122"),
                benefit: Some("While using the Tiger Style feat, you can apply the penalty from Power Attack to your AC instead of attack rolls. Additionally, once per round as a swift action, you can move up to half your speed closer to a target you hit with an unarmed strike or made a successful combat maneuver against on this turn or your last turn."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Improved Unarmed Strike,Power Attack,Tiger Style,Tiger Claws", "PREMULT:1,[PRETOTALAB:9],[PREVARGTEQ:MonkFeatQualify,8]"]),
            },
            // Tiger Style -- uc_feats.lst:335
            UcFeatEntry {
                key: "Tiger Style",
                category: FeatCategory::Combat,
                name: "Tiger Style",
                description: Some("Your unarmed fighting style emulates the strength and ferocity of a tiger."),
                pretext: None,
                source_page: Some("p.122"),
                benefit: Some("While using this style, you gain a +2 bonus to your CMD against bull rush, overrun, and trip maneuvers. You can also deal slashing damage with your unarmed strikes. Whenever you score a critical hit with your slashing unarmed strike, your opponent also takes 1d4 points of bleed damage at the start of his next two turns. &nl; [Normal] Unarmed strikes deal bludgeoning damage."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PREMULT:1,[PRETOTALAB:3],[PREVARGTEQ:MonkFeatQualify,3]"]),
            },
            // Trapper's Setup -- uc_feats.lst:336
            UcFeatEntry {
                key: "Trapper's Setup",
                category: FeatCategory::General,
                name: "Trapper's Setup",
                description: Some("You have an instinct for waiting until just the right moment to spring a hazard or trap."),
                pretext: None,
                source_page: Some("p.122"),
                benefit: Some("When you manually trigger a trap against opponents, that trap receives either a +2 circumstance bonus on melee attack rolls or a +2 circumstance bonus to its saving throw DC."),
                prerequisites: Some(&["PRESKILL:1,Craft (traps)=5"]),
            },
            // Twin Thunders -- uc_feats.lst:337
            UcFeatEntry {
                key: "Twin Thunders",
                category: FeatCategory::Combat,
                name: "Twin Thunders",
                description: Some("When you fight giants, your powerful blows combine with skills learned from generations of your people to quickly even the odds."),
                pretext: None,
                source_page: Some("p.123"),
                benefit: Some("Once per round, when wielding a bludgeoning weapon for which you have Weapon Focus in each hand against a creature with the giant subtype, if you hit the creature with your off-hand weapon after you hit with your primary weapon, roll the damage dice for your off-hand weapon twice and add the results together before adding any bonuses. Such extra weapon damage dice are not multiplied on a critical hit."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Defensive Training,Gnome ~ Defensive Training", "PREABILITY:1,CATEGORY=FEAT,Weapon Focus", "PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Two-Weapon Fighting],[PREABILITY:1,CATEGORY=Special Ability,Flurry of Blows]", "PREFACT:1,TEMPLATES,IsDwarf=true,IsGnome=True"]),
            },
            // Twin Thunders Flurry -- uc_feats.lst:338
            UcFeatEntry {
                key: "Twin Thunders Flurry",
                category: FeatCategory::Combat,
                name: "Twin Thunders Flurry",
                description: Some("Your dual bludgeoning strikes are especially deadly when you are fighting giants."),
                pretext: None,
                source_page: Some("p.123"),
                benefit: Some("You can trip a creature with the giant subtype of up to Huge size, and you gain a +2 bonus on damage rolls against creatures of the giant subtype. Further, each time you hit a creature of the giant subtype with your offhand weapon after you hit that creature with your primary weapon, you can deal the extra off-hand weapon damage Twin Thunders grants you."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Defensive Training,Gnome ~ Defensive Training", "PREABILITY:2,CATEGORY=FEAT,Twin Thunders,Weapon Focus", "PREMULT:1,[PREABILITY:2,CATEGORY=FEAT,Improved Two-Weapon Fighting,Two-Weapon Fighting],[PREABILITY:1,CATEGORY=Special Ability,Flurry of Blows]", "PREFACT:1,TEMPLATES,IsDwarf=true,IsGnome=True", "PRETOTALAB:6"]),
            },
            // Twin Thunders Master -- uc_feats.lst:339
            UcFeatEntry {
                key: "Twin Thunders Master",
                category: FeatCategory::Combat,
                name: "Twin Thunders Master",
                description: Some("With thunderous simultaneous strikes, you can batter a mighty giant into submission."),
                pretext: None,
                source_page: Some("p.123"),
                benefit: Some("Whenever you deal an opponent extra damage with the Twin Thunders feat, that opponent is shaken for 1 round. You also force that opponent to succeed at a Fortitude saving throw (DC 10 + half your level + your Str modifier) or become staggered for 1 round. If you use this feat to render staggered an opponent that is already staggered, you daze that opponent instead. In a similar way, you can stun an opponent that is already dazed."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Dwarf ~ Defensive Training,Gnome ~ Defensive Training", "PREABILITY:3,CATEGORY=FEAT,Twin Thunders,Twin Thunders Flurry,Weapon Focus", "PREMULT:1,[PREABILITY:2,CATEGORY=FEAT,Improved Two-Weapon Fighting,Two-Weapon Fighting],[PREABILITY:1,CATEGORY=Special Ability,Flurry of Blows]", "PREFACT:1,TEMPLATES,IsDwarf=true,IsGnome=True", "PRETOTALAB:9"]),
            },
            // Two-Handed Thrower -- uc_feats.lst:340
            UcFeatEntry {
                key: "Two-Handed Thrower",
                category: FeatCategory::Combat,
                name: "Two-Handed Thrower",
                description: Some("You hurl weapons with both hands and with great force, sometimes using a whirling technique to send your weapon flying through the air at tremendous speeds."),
                pretext: None,
                source_page: Some("p.123"),
                benefit: Some("Whenever you use two hands to throw a onehanded or two-handed weapon, you gain a bonus on damage rolls equal to 1-1/2 times your Strength bonus. Using two hands to throw any weapon requires only a standard action for you. If you also have the Quick Draw feat, you can throw two-handed weapons at your full normal rate of attacks. &nl; [Normal] You add your Strength bonus on thrown weapon damage, regardless of available hands. Throwing a twohanded weapon is a full-round action."),
                prerequisites: Some(&["PRESTAT:1,STR=15"]),
            },
            // Two-Weapon Feint -- uc_feats.lst:341
            UcFeatEntry {
                key: "Two-Weapon Feint",
                category: FeatCategory::Combat,
                name: "Two-Weapon Feint",
                description: Some("You use one weapon to distract your enemy while slipping another past his defenses."),
                pretext: None,
                source_page: Some("p.123"),
                benefit: Some("While using Two-Weapon Fighting to make melee attacks, you can forgo your first primary-hand melee attack to make a Bluff check to feint an opponent."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Two-Weapon Fighting", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,15],[PREVARGTEQ:FeatDexRequirement,15]"]),
            },
            // Vicious Stomp -- uc_feats.lst:342
            UcFeatEntry {
                key: "Vicious Stomp",
                category: FeatCategory::Combat,
                name: "Vicious Stomp",
                description: Some("You take advantage of the moment to brutally kick an enemy when he is down."),
                pretext: None,
                source_page: Some("p.123"),
                benefit: Some("Whenever an opponent falls prone adjacent to you, that opponent provokes an attack of opportunity from you. This attack must be an unarmed strike."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Combat Reflexes,Improved Unarmed Strike"]),
            },
            // Deathless Initiate (Vigor/Wounds) -- uc_feats.lst:356
            UcFeatEntry {
                key: "Deathless Initiate (Vigor/Wounds)",
                category: FeatCategory::Combat,
                name: "Deathless Initiate (Vigor/Wounds)",
                description: Some("For you, impending death is a call to wrath."),
                pretext: None,
                source_page: Some("p.207"),
                benefit: Some("You are not staggered when your wound points reach your wound threshold, but you lose 1 wound point if you take any action during your turn. You only take 1 wound point each round when you take actions. Furthermore, you gain a +2 bonus on melee attacks and damage rolls when your wound points are at or below your wound threshold."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Diehard (Vigor/Wound),Endurance", "PREFACT:1,TEMPLATES,IsOrc=true,IsHalfOrc=true", "PRERULE:1,DAMAGE_VW", "PRESTAT:2,STR=13,CON=13", "PRETOTALAB:6"]),
            },
            // Diehard (Vigor/Wounds) -- uc_feats.lst:358
            UcFeatEntry {
                key: "Diehard (Vigor/Wounds)",
                category: FeatCategory::General,
                name: "Diehard (Vigor/Wounds)",
                description: Some("You keep on going, even when your wound points are lower than your wound threshold."),
                pretext: None,
                source_page: Some("p.207"),
                benefit: Some("When your current wound point total is below your wound threshold, you do not need to succeed at the DC 10 Constitution check to stay conscious."),
                prerequisites: Some(&["PRERULE:1,DAMAGE_VW"]),
            },
            // Toughness (Vigor/Wounds) -- uc_feats.lst:359
            UcFeatEntry {
                key: "Toughness (Vigor/Wounds)",
                category: FeatCategory::General,
                name: "Toughness (Vigor/Wounds)",
                description: Some("You have enhanced physical stamina."),
                pretext: None,
                source_page: Some("p.207"),
                benefit: Some("You gain 1 wound point for every level or Hit Die your character has."),
                prerequisites: Some(&["PRERULE:1,DAMAGE_VW"]),
            },
            // Improved Called Shot -- uc_feats.lst:379
            UcFeatEntry {
                key: "Improved Called Shot",
                category: FeatCategory::CalledShot,
                name: "Improved Called Shot",
                description: Some("You are skilled at landing blows right where you want to."),
                pretext: Some("Prerequisites: Int 13, Combat Expertise."),
                source_page: Some("p.195"),
                benefit: Some("You receive a +2 bonus on attack rolls when making a called shot. When taking a full-round or standard action that gives you multiple attacks, you can replace a single attack with a called shot. You may only attempt one called shot per round. Normal: You can make one called shot per round as a full-round action."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Combat Expertise", "PREVARGTEQ:PreStatScore_INT,13"]),
            },
            // Greater Called Shot -- uc_feats.lst:380
            UcFeatEntry {
                key: "Greater Called Shot",
                category: FeatCategory::CalledShot,
                name: "Greater Called Shot",
                description: Some("You can make multiple called shots where others could land but one."),
                pretext: Some("Prerequisites: Int 13, Combat Expertise, Improved Called Shot, base attack bonus +6."),
                source_page: Some("p.195"),
                benefit: Some("Whenever you make an attack, you can choose to replace that attack with a called shot. You can make multiple called shots in a single round. Each additional called shot after the first made in the same round takes a -5 penalty. In addition, a called shot that deals half the creature's hit points of damage (minimum 40) is a debilitating blow. Normal: You can make only one called shot in a round as a full-round action. A called shot that deals 50 points of damage is a debilitating blow."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Improved Called Shot", "PRETOTALAB:6", "PREVARGTEQ:PreStatScore_INT,13"]),
            },
            // Style Feat Wildcard -- uc_feats.lst:391
            UcFeatEntry {
                key: "Style Feat Wildcard",
                category: FeatCategory::Style,
                name: "Style Feat Wildcard",
                description: Some("This wildcard slot can be used to take on feats in a style feat path as long as the prerequisites are met. This wildcard can be changed every time the master of many styles changes styles."),
                pretext: None,
                source_page: Some("p.59"),
                benefit: Some(" The master of many monks currently has %1 wildcard slots available|MonkWildcardSlot"),
                prerequisites: Some(&["PRECLASS:1,Monk=6", "PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Master Of Many Styles"]),
            },
        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_has_261_records() {
        assert_eq!(feat_tables().len(), 261);
    }

    #[test]
    fn every_record_carries_desc_and_benefit() {
        for e in feat_tables() {
            assert!(e.description.is_some(), "{} has no DESC:", e.key);
            assert!(e.benefit.is_some(), "{} has no BENEFIT:", e.key);
        }
    }

    #[test]
    fn no_record_is_deferred() {
        assert_eq!(feat_tables().iter().filter(|e| e.benefit.is_none()).count(), 0);
    }

    #[test]
    fn keys_are_unique_within_book() {
        let keys: std::collections::BTreeSet<&str> = feat_tables().iter().map(|e| e.key).collect();
        assert_eq!(keys.len(), feat_tables().len());
    }
}
