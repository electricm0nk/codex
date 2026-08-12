//! Ultimate Wilderness (UW) feat catalog. SD28-E26 slice 1, mirroring
//! `ultimate_intrigue::feat_tables`'s own established shape exactly.
//!
//! **Corpus coverage, honestly bounded.** `uw_feats.lst` has 137 top-level
//! `CATEGORY:FEAT` records (re-derived: `grep -c 'CATEGORY:FEAT'
//! uw_feats.lst`). One -- `CATEGORY=FEAT|Intimidating Prowess.MOD` -- is a
//! `.MOD` row modifying CRB's own `Intimidating Prowess` feat, not a new
//! feat; excluded the same way every other book's `.MOD` rows are. A
//! second, `Extended Animal Focus`, collides with ACG's own feat of the
//! same name (`acg_feats.lst:58`, same Hunter Animal Focus concept, real
//! `BONUS:VAR` token vs. UW's prose-only row) -- re-derived against every
//! other book's real runtime feat key set (a scratch `#[test]` dump, not
//! a source grep, per `decisions.md §44`'s own lesson applied here from
//! the start) and excluded as a republished duplicate, the same
//! `already_ingested_keys()` discipline `decisions.md §39`/`§44`
//! established. **135 real, distinct, new feat records remain.** Every one
//! carries real `DESC:`/`BENEFIT:` -- no upstream splice/truncation defect
//! found. All 135 are text-complete, none `deferred-with-reason`.
//!
//! **No `KEY:` token on any record**, so `key == name` for every entry.
//!
//! **`category` is UW's own enum, not the shared `crb::feats::FeatCategory`.**
//! Unlike Ultimate Intrigue, UW's `TYPE:` facets include `Animal` and
//! `Mount` (Companion-focused feats -- e.g. a druid's animal companion
//! feats), which the shared enum has no variant for; inventing a mapping
//! onto an existing variant would be a classification the corpus never
//! made. `Combat.Style`/`Combat.Teamwork` sub-facets fold to `Combat`,
//! matching every other book's own folding convention.
//!
//! **`prerequisites` carries every real `PRE`-family token verbatim**,
//! gathered directly at ingest, `None` when the corpus row has none.
//!
//! Every field below is copied verbatim from the real corpus row (source:
//! `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/
//! ultimate_wilderness/uw_feats.lst`), generated programmatically by a
//! one-off extraction script, not hand-transcribed.

use super::super::crb::feats::FeatCategory as SharedFeatCategory;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatCategory {
    General,
    Combat,
    ItemCreation,
    Metamagic,
    Teamwork,
    /// Companion/animal-focused feats -- e.g. a druid's animal companion
    /// gaining a bonus feat. No corpus book before UW carries this facet.
    Animal,
    /// Mount-focused feats (e.g. cavalier/samurai mount options). No
    /// corpus book before UW carries this facet.
    Mount,
}

impl FeatCategory {
    pub const ALL: &'static [FeatCategory] = &[
        FeatCategory::General,
        FeatCategory::Combat,
        FeatCategory::ItemCreation,
        FeatCategory::Metamagic,
        FeatCategory::Teamwork,
        FeatCategory::Animal,
        FeatCategory::Mount,
    ];

    /// The subset of variants that coincide with the shared
    /// `crb::feats::FeatCategory` enum, for books/consumers that fold UW's
    /// records into that shared classification. `Animal`/`Mount` have no
    /// shared equivalent and are not present here.
    pub fn as_shared(self) -> Option<SharedFeatCategory> {
        match self {
            FeatCategory::General => Some(SharedFeatCategory::General),
            FeatCategory::Combat => Some(SharedFeatCategory::Combat),
            FeatCategory::ItemCreation => Some(SharedFeatCategory::ItemCreation),
            FeatCategory::Metamagic => Some(SharedFeatCategory::Metamagic),
            FeatCategory::Teamwork => Some(SharedFeatCategory::Teamwork),
            FeatCategory::Animal | FeatCategory::Mount => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UwFeatEntry {
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

/// Full UW feat catalog: all 135 real, distinct corpus records, in source
/// order. Built once and cached for the process lifetime.
pub fn feat_tables() -> &'static [UwFeatEntry] {
    static TABLE: std::sync::OnceLock<Vec<UwFeatEntry>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        vec![
            // Ambush Awareness -- uw_feats.lst:10
            UwFeatEntry {
                key: "Ambush Awareness",
                category: FeatCategory::General,
                name: "Ambush Awareness",
                description: Some("You are always on your toes and are rarely caught off-guard for long, even when an enemy gets the jump on you."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: Some("If you are unable to act in the surprise round because you failed a Perception check, you can still act on your initiative count in the surprise round, but only to take the total defense action. &nl;[Normal] If you are unable to act in the surprise round because you failed a Perception check, you can't take any actions during the surprise round."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Alertness"]),
            },
            // Animal Call -- uw_feats.lst:11
            UwFeatEntry {
                key: "Animal Call",
                category: FeatCategory::General,
                name: "Animal Call",
                description: Some("You've learn how to replicate a number of animal calls native to wilderness environments."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: Some("Pick one of the ranger's favored terrains. You can use your Bluff skill to mimic the calls of animals native to that terrain. Creatures with ranks in Knowledge (Nature) can use that skill in place of Sense Motive to detect your mimicry and realize that the sound is false. &nl;[Special] You can take this feat multiple times. Each time you do, select an additional favored terrain to which this feat applies."),
                prerequisites: Some(&["PRESKILL:2,Bluff=1,Knowledge (Nature)=1"]),
            },
            // Animal Disguise -- uw_feats.lst:12
            UwFeatEntry {
                key: "Animal Disguise",
                category: FeatCategory::General,
                name: "Animal Disguise",
                description: Some("With a little work, you can convincingly disguise yourself as an animal."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: Some("You can use Disguise to disguise yourself as an animal of your size category. You must have an appropriate pelt and any other animal parts needed to complete the disguise. You also gain a +2 bonus on Disguise checks when you disguise yourself as an animal. Creatures with ranks in Knowledge (nature) can use that skill in place of Sense Motive to detect this type of disguise."),
                prerequisites: Some(&["PRESKILL:2,Disguise=6,Knowledge (Nature)=6"]),
            },
            // Animal Ferocity -- uw_feats.lst:13
            UwFeatEntry {
                key: "Animal Ferocity",
                category: FeatCategory::Combat,
                name: "Animal Ferocity",
                description: Some("When cornered and wounded, you fight like a feral beast."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: Some("When your hit points are reduced below 0, you can make attacks, but you take a -5 penalty on each attack roll."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Half-Orc ~ Orc Ferocity,Orc ~ Ferocity,Ferocity,Builder Racial Trait ~ Ferocity,Builder Racial Trait ~ Orc Ferocity", "PRETOTALAB:3"]),
            },
            // Aquatic Combatant -- uw_feats.lst:14
            UwFeatEntry {
                key: "Aquatic Combatant",
                category: FeatCategory::Combat,
                name: "Aquatic Combatant",
                description: Some("You have trained to fight while submerged in water."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: Some("You gain a +2 bonus on Swim checks and don't take the usual penalties on melee attack rolls made underwater. Your slashing melee attacks and unarmed bludgeoning attacks deal full damage underwater. &nl;[Normal] When you're underwater, most of your melee attacks take a -2 penalty and deal only half damage."),
                prerequisites: Some(&["PRESKILL:1,Swim=1"]),
            },
            // Aquatic Spell -- uw_feats.lst:15
            UwFeatEntry {
                key: "Aquatic Spell",
                category: FeatCategory::Metamagic,
                name: "Aquatic Spell",
                description: Some("You can cast your spells underwater or into water with little difficulty."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: Some("An aquatic spell functions normally underwater and requires no caster level check to cast, even if it has the fire descriptor. In addition, the spell can be cast from the surface into water and still be effective. An aquatic spell uses up a spell slot 1 level higher than the spell's actual level."),
                prerequisites: None,
            },
            // Arctic Adaptation -- uw_feats.lst:16
            UwFeatEntry {
                key: "Arctic Adaptation",
                category: FeatCategory::General,
                name: "Arctic Adaptation",
                description: Some("You are comfortable in the driving snow and glaring ice of frigid climes, and you can survive much longer in such harsh environments than those who are unaccustomed to the cold."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: Some("You treat cold environments (Core Rulebook 442) as though they were one step less severe than they normally are. Additionally, you gain a +2 bonus on Perception checks against creatures that gain a racial bonus on Stealth checks in snowy conditions, and you gain a +4 bonus on saving throws and checks to avoid becoming blinded or dazzled by ice or snow glare."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Favored Terrain ~ Cold"]),
            },
            // Beast Hunter -- uw_feats.lst:17
            UwFeatEntry {
                key: "Beast Hunter",
                category: FeatCategory::Combat,
                name: "Beast Hunter",
                description: Some("Thanks to your experience hunting in the wilds, you are capable of tracking animals in your most often traveled terrains, and you can easily take down animals larger than yourself."),
                pretext: None,
                source_page: Some("p.106"),
                benefit: Some("Pick one of the ranger's favored terrains. You gain a +2 bonus on Survival checks to track animals native to that terrain. Additionally, against animals native to that terrain that are at least one size category larger than you, you gain a +1 dodge bonus to your AC and a +1 insight bonus on attack rolls. &nl;[Special] You can take this feat multiple times. Each time you select this feat, you can choose an additional favored terrain to gain the listed benefits in."),
                prerequisites: Some(&["PRESKILL:1,Knowledge (Nature)=1,Survival=1", "PRETOTALAB:1"]),
            },
            // Beastmaster Ire -- uw_feats.lst:18
            UwFeatEntry {
                key: "Beastmaster Ire",
                category: FeatCategory::Combat,
                name: "Beastmaster Ire",
                description: Some("Seeing your animal companion attacked or hurt causes you to retaliate with a bestial fury of your own."),
                pretext: None,
                source_page: Some("p.107"),
                benefit: Some("When you activate Beastmaster Style's benefit, you gain a +2 morale bonus on melee weapon attack rolls and +4 morale bonus on weapon damage rolls against the attacking creature. If the enemy attack dealt damage to your animal companion, these bonuses increase to +4 and +8, respectively. These bonuses last until the end of your next turn."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Alertness,Beastmaster Salvation,Beastmaster Style", "PRESKILL:2,Handle Animal=9,Sense Motive=5", "PREVARGTEQ:PreStatScore_CHA,13"]),
            },
            // Beastmaster Salvation -- uw_feats.lst:19
            UwFeatEntry {
                key: "Beastmaster Salvation",
                category: FeatCategory::Combat,
                name: "Beastmaster Salvation",
                description: Some("Your presence grants your companion the toughness and will to resist all manner of threats."),
                pretext: None,
                source_page: Some("p.107"),
                benefit: Some("If you are adjacent to your animal companion when it attempts a saving throw, you can attempt a Handle Animal check as an immediate action. Your animal companion does not attempt the saving throw, but instead uses your Handle Animal result as its result for the save."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Alertness,Beastmaster Style", "PRESKILL:2,Handle Animal=5,Sense Motive=5", "PREVARGTEQ:PreStatScore_CHA,13"]),
            },
            // Beastmaster Style -- uw_feats.lst:20
            UwFeatEntry {
                key: "Beastmaster Style",
                category: FeatCategory::Combat,
                name: "Beastmaster Style",
                description: Some("Your animal companion is your most treasured friend, and you steadfastly protect it from your foes."),
                pretext: None,
                source_page: Some("p.107"),
                benefit: Some("When you are adjacent to your animal companion and an attack is made against it, if you are also adjacent to the attacking creature, you can attempt a Handle Animal check as an immediate action to negate the hit. The hit is negated if your Handle Animal check result is greater than the attacker's attack roll. &nl;[Special] You cannot use this style if you are mounted on your animal companion."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Companion", "PRESKILL:1,Handle Animal=1", "PREVARGTEQ:PreStatScore_CHA,13"]),
            },
            // Boon Companion -- uw_feats.lst:21
            UwFeatEntry {
                key: "Boon Companion",
                category: FeatCategory::General,
                name: "Boon Companion",
                description: Some("Your bond with your animal companion or familiar is unusually close."),
                pretext: None,
                source_page: Some("p.217"),
                benefit: Some("The abilities of your animal companion or familiar are calculated as though your class were 4 levels higher, to a maximum effective druid level equal to your character level. If you have more than one animal companion or familiar, choose one to receive this benefit. If you lose or dismiss an animal companion or familiar that has received this benefit, you may apply this feat to the replacement creature. Special: You may select this feat more than once. The effects do not stack. Each time you take the feat, it applies to a different animal companion or familiar."),
                prerequisites: Some(&["PREMULT:1,[PREVARGT:AnimalCompanionLVL,0],[PREVARGT:FamiliarLVL,0],[PREVARGT:SpecialMountLVL,0],[PREVARGT:CavalierMountLVL,0],[PREVARGT:HuntmasterCompanionLVL,0],[PREABILITY:1,CATEGORY=Special Ability,TYPE.Animal Companion,TYPE.Familiar,TYPE.Mount,TYPE.Special Mount,TYPE.Huntmaster Companion]"]),
            },
            // Branch Pounce -- uw_feats.lst:22
            UwFeatEntry {
                key: "Branch Pounce",
                category: FeatCategory::Combat,
                name: "Branch Pounce",
                description: Some("You are adept at exploiting higher ground to its greatest advantage in battle, and can leap from above to deal incredible damage to your foes."),
                pretext: None,
                source_page: Some("p.107"),
                benefit: Some("When charging a target by jumping down from above (such as when jumping out of a tree), you can soften your fall with a melee attack. If the attack at the end of your charge hits, the attack deals damage as normal and you also deal the amount of falling damage appropriate to your fall to the target (1d6 points for a 10-foot fall, 2d6 points for a 20-foot fall, and so on). This falling damage is not multiplied on a critical hit. You land in an unoccupied square of your choosing adjacent to the target, and you take falling damage as if your fall had been 10 feet shorter. You can attempt an Acrobatics check as normal to treat the fall as an additional 10 feet shorter for the purpose of determining the damage you take from the fall. If your attack misses, you land prone in a random square adjacent to the target and automatically take the full amount of falling damage."),
                prerequisites: Some(&["PRESKILL:2,Climb=3,Stealth=3"]),
            },
            // Bristling Bull Rush -- uw_feats.lst:23
            UwFeatEntry {
                key: "Bristling Bull Rush",
                category: FeatCategory::Combat,
                name: "Bristling Bull Rush",
                description: Some("You deal damage to opponents when bull rushing them through difficult terrain, using the environment itself to harm your targets."),
                pretext: None,
                source_page: Some("p.108"),
                benefit: Some("When you successfully bull rush a creature, it takes 1d4 points of damage for every 5 feet it is pushed through naturally occurring difficult terrain. This damage is in addition to any damage the creature might normally take from moving through the difficult terrain. Creatures with the woodland stride special ability or who are otherwise unaffected by difficult terrain are immune to this extra damage."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Bull Rush,Power Attack", "PRETOTALAB:3", "PREVARGTEQ:PreStatScore_STR,13"]),
            },
            // Bristling Drag -- uw_feats.lst:24
            UwFeatEntry {
                key: "Bristling Drag",
                category: FeatCategory::Combat,
                name: "Bristling Drag",
                description: Some("You deal damage to opponents when dragging them through difficult terrain."),
                pretext: None,
                source_page: Some("p.108"),
                benefit: Some("When you successfully drag a creature, it takes 1d4 points of damage for every 5 feet it is dragged through naturally occurring difficult terrain. This damage is in addition to any damage the creature might normally take from moving through the difficult terrain. Creatures with the woodland stride special ability or who are otherwise unaffected by difficult terrain are immune to this extra damage."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Drag,Power Attack", "PRETOTALAB:3", "PREVARGTEQ:PreStatScore_STR,13"]),
            },
            // Camouflaged Trap -- uw_feats.lst:25
            UwFeatEntry {
                key: "Camouflaged Trap",
                category: FeatCategory::General,
                name: "Camouflaged Trap",
                description: Some("You excel at hiding your traps in the wilderness, making them more difficult for your enemies to locate and avoid."),
                pretext: None,
                source_page: Some("p.108"),
                benefit: Some("When you craft and set a trap in a wilderness environment, you increase the DC of the Perception check required to find it by 5."),
                prerequisites: Some(&["PRESKILL:2,Craft (Traps)=4,Survival=4"]),
            },
            // Clinging Climber -- uw_feats.lst:26
            UwFeatEntry {
                key: "Clinging Climber",
                category: FeatCategory::Combat,
                name: "Clinging Climber",
                description: Some("Using leverage and pure brawn, you can cling to a cliff face, ladder, or rope with your legs to free both of your hands to take other actions."),
                pretext: None,
                source_page: Some("p.108"),
                benefit: Some("As a move action while climbing, you can cling to your climbing surface with your legs by succeeding at a Climb check with the same DC as that of climbing the surface as a move action. If successful, you can make attacks with a two-handed ranged weapon and reload ranged weapons until you resume climbing. The GM can rule that this feat doesn't work on certain climbing surfaces."),
                prerequisites: Some(&["PRESKILL:1,Climb=3", "PREVARGTEQ:PreStatScore_STR,13"]),
            },
            // Command Animals -- uw_feats.lst:27
            UwFeatEntry {
                key: "Command Animals",
                category: FeatCategory::General,
                name: "Command Animals",
                description: Some("You channel energy to get animals to do your bidding."),
                pretext: None,
                source_page: Some("p.108"),
                benefit: Some("As a standard action, you can use one of your daily uses of channel energy in an attempt to control animals within 30 feet. Animals can attempt a Will save (DC = 10 + half your class level + your Charisma modifier) to negate the effect. Animals that fail their saves fall under your control, obeying your commands to the best of their ability as if under the effects of a charm monster spell with a caster level equal to your class level. An affected animal can attempt a new saving throw each day to escape this effect. You can control any number of animals, so long as their total Hit Dice do not exceed your class level. If you use channel energy in this way, it has no other effect (it does not heal or harm nearby creatures). If an affected animal is controlled by or a companion of another creature, you must attempt an opposed Charisma check whenever your orders conflict."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Positive Energy,TYPE.Channel Negative Energy", "PREDOMAIN:1,Animal"]),
            },
            // Command Plants -- uw_feats.lst:28
            UwFeatEntry {
                key: "Command Plants",
                category: FeatCategory::General,
                name: "Command Plants",
                description: Some("You channel energy to bring plants under your control."),
                pretext: None,
                source_page: Some("p.108"),
                benefit: Some("As a standard action, you can use one of your daily uses of channel energy to command plants within 30 feet of you. Plants can attempt a Will save (DC = 10 + half your class level + your Charisma modifier) to negate the effect. This functions as the command plant spell with a caster level equal to your class level. Each affected plant can attempt a new saving throw each day to escape this effect. You can control any number of plants, so long as their total Hit Dice do not exceed your class level. If you use channel energy in this way, it has no other effect (it does not heal or harm nearby creatures). If an affected plant creature is controlled by or a companion of another creature, you must attempt an opposed Charisma check whenever your orders conflict."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Positive Energy,TYPE.Channel Negative Energy", "PREDOMAIN:1,Plant"]),
            },
            // Cover Tracks -- uw_feats.lst:29
            UwFeatEntry {
                key: "Cover Tracks",
                category: FeatCategory::General,
                name: "Cover Tracks",
                description: Some("You are very difficult to follow in the wild, leaving few tracks in your wake."),
                pretext: None,
                source_page: Some("p.108"),
                benefit: Some("You increase the DC of Survival checks to track you by 5 when moving at full speed and by 10 when moving at half speed. You can cover the tracks of %1 allies within 30 feet, increasing the DC to track them by 2 if they are moving at full speed or by 5 if they are moving at half speed.&nl;[Special] If you have the favored terrain class feature and you are in that terrain, you also add your favored terrain bonus to the DC to track you or your allies.|WIS+TL"),
                prerequisites: Some(&["PRESKILL:1,Survival=3"]),
            },
            // Crashing Wave Buffet -- uw_feats.lst:30
            UwFeatEntry {
                key: "Crashing Wave Buffet",
                category: FeatCategory::Combat,
                name: "Crashing Wave Buffet",
                description: Some("You disorient your opponents by pushing and pounding them mercilessly, just like how the sea beats relentlessly against the rocks."),
                pretext: None,
                source_page: Some("p.108"),
                benefit: Some("When dragging or repositioning an opponent, you use your full body to knock your opponent around like a constant wave. At the end of the drag or reposition maneuver, your opponent must succeed at a Fortitude save (DC %1 + 2 for every 5 feet you drag or reposition your opponent). On a failed save, your opponent becomes disorientated from the movement and takes a -2 penalty on attack rolls, combat maneuver checks, and Dexterity-based skill checks until the end of its next turn.|10+TL/2+WIS"),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Crashing Wave Style,Improved Drag,Improved Reposition,Improved Unarmed Strike", "PRETOTALAB:6", "PREVARGTEQ:PreStatScore_WIS,15"]),
            },
            // Crashing Wave Fist -- uw_feats.lst:31
            UwFeatEntry {
                key: "Crashing Wave Fist",
                category: FeatCategory::Combat,
                name: "Crashing Wave Fist",
                description: Some("In addition to simply moving it, you are able to strike your enemy while pushing it around."),
                pretext: None,
                source_page: Some("p.109"),
                benefit: Some("When dragging or repositioning an opponent, at any point during the movement, you can make one unarmed attack against the opponent using your highest attack bonus. You can make one additional attack for every 5 feet you drag or reposition the opponent beyond the first 5 feet. You take a cumulative -5 penalty on each additional attack made in this way."),
                prerequisites: Some(&["PREABILITY:5,CATEGORY=FEAT,Crashing Wave Buffet,Crashing Wave Style,Improved Drag,Improved Reposition,Improved Unarmed Strike", "PRETOTALAB:9", "PREVARGTEQ:PreStatScore_WIS,15"]),
            },
            // Crashing Wave Style -- uw_feats.lst:32
            UwFeatEntry {
                key: "Crashing Wave Style",
                category: FeatCategory::Combat,
                name: "Crashing Wave Style",
                description: Some("You relentlessly push your enemies around, as the sea moves those within and upon it against their will."),
                pretext: None,
                source_page: Some("p.109"),
                benefit: Some("When you successfully drag or reposition an opponent while using this style, at any point during the dragging or repositioning of the opponent, you can move 5 feet as an immediate action, including moving into a square previously occupied by the opponent, even if you have already taken a move action this round. This movement does not provoke attacks of opportunity."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Improved Drag,Improved Reposition,Improved Unarmed Strike", "PREVARGTEQ:PreStatScore_WIS,13"]),
            },
            // Cultivate Magic Plants -- uw_feats.lst:33
            UwFeatEntry {
                key: "Cultivate Magic Plants",
                category: FeatCategory::ItemCreation,
                name: "Cultivate Magic Plants",
                description: Some("You combine a natural green thumb and knowledge of magic in order to grow magic plants."),
                pretext: None,
                source_page: Some("p.109"),
                benefit: Some("You can cultivate magic plants; see Chapter 7 for several example types of magic plants. Cultivating a magic plant takes 1 week per 1,000 gp in its base price. When you create a magic plant, you make the same choices that you would normally make when casting the spell. Whoever consumes the fruit of the magic plant is the target of the spell. See page 247 for full rules for cultivating magic plants."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Brew Potion,Craft Wondrous Item", "PRESKILL:1,Knowledge (Nature)=1"]),
            },
            // Deadly Trap -- uw_feats.lst:34
            UwFeatEntry {
                key: "Deadly Trap",
                category: FeatCategory::General,
                name: "Deadly Trap",
                description: Some("Your traps are especially deadly, either through their increased accuracy or the viciousness of their blades and bludgeons."),
                pretext: None,
                source_page: Some("p.109"),
                benefit: Some("If you craft a trap that requires an attack roll to hit, you can either increase the threat multiplier of the trap by 1 (x2 becomes x3, and so on) or grant the trap's attack a +4 bonus to confirm a critical hit."),
                prerequisites: Some(&["PRESKILL:2,Craft (Traps)=8,Survival=8"]),
            },
            // Deep Diver -- uw_feats.lst:35
            UwFeatEntry {
                key: "Deep Diver",
                category: FeatCategory::General,
                name: "Deep Diver",
                description: Some("You are accustomed to diving deeper than most would dare swim and can do so with less risk of drowning than even other experienced swimmers."),
                pretext: None,
                source_page: Some("p.109"),
                benefit: Some("You gain a +4 bonus on Acrobatics checks to reduce falling damage when diving into water and on Perception checks in dim light or darkness underwater. When attempting a Swim check to swim downward (at least 45 degrees down from the horizontal), on a successful check you can swim half your speed as a move action, or your speed as a full-round action. If you have a swim speed, it increases by 10 feet when you spend a move action to swim only downward.&nl;In addition, you can hold your breath for a number of rounds equal to three times your Constitution score, and you gain a +4 bonus on Constitution checks to continue holding your breath after this time and to resist the effects of the cold environment underwater. Pressure damage you take from deep water is halved."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Endurance"]),
            },
            // Desert Dweller -- uw_feats.lst:36
            UwFeatEntry {
                key: "Desert Dweller",
                category: FeatCategory::General,
                name: "Desert Dweller",
                description: Some("The endless sands and waterless wastes are your home, and neither the heat nor dehydration presents as lethal of a threat to you as it does to other travelers."),
                pretext: None,
                source_page: Some("p.109"),
                benefit: Some("You treat hot environments (Core Rulebook 444) as though they were one step less severe; if you have a similar ability from another feat, such as Torrid Tolerance*, the benefits stack and you treat hot conditions as if they were two steps less severe. You need to consume only half the normal amount of water for a creature of your size, and you gain a +4 bonus on Constitution checks to resist the effects of thirst. You also gain a +4 bonus on saving throws and checks to avoid becoming blinded or dazzled by glare or being deceived by a desert mirage."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Favored Terrain ~ Desert"]),
            },
            // Eagle-Eyed -- uw_feats.lst:37
            UwFeatEntry {
                key: "Eagle-Eyed",
                category: FeatCategory::General,
                name: "Eagle-Eyed",
                description: Some("Your distance vision is exceptionally keen, enabling you to see well in both normal and dim lighting conditions and providing you an advantage when making ranged attacks."),
                pretext: None,
                source_page: Some("p.110"),
                benefit: Some("Your distance modifier to the DC of vision-based Perception checks is decreased to +1 per 50 feet in bright light or normal light, or +1 per 20 feet in dim light. You also gain a +2 circumstance bonus on ranged attacks against targets that are more than 100 feet away."),
                prerequisites: Some(&["PRESKILL:1,Perception=3"]),
            },
            // Earth Magic -- uw_feats.lst:38
            UwFeatEntry {
                key: "Earth Magic",
                category: FeatCategory::General,
                name: "Earth Magic",
                description: Some("You can draw raw magical energy from the ground to empower your spells."),
                pretext: None,
                source_page: Some("p.110"),
                benefit: Some("While you're in your favored terrain, your effective caster level increases by 1 for the purpose of improving spell effects dependent on caster level. This increase in effective caster level doesn't grant you access to more spells."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Basic Favored Terrain", "PRESPELLTYPE:1,ANY=1"]),
            },
            // Eidolon Mount -- uw_feats.lst:39
            UwFeatEntry {
                key: "Eidolon Mount",
                category: FeatCategory::Combat,
                name: "Eidolon Mount",
                description: Some("Your eidolon is capable of carrying you into combat with great skill."),
                pretext: Some("Eidolon able to choose quadruped or serpentine as a base form."),
                source_page: Some("p.110"),
                benefit: Some("Your eidolon is capable of serving you as a combat-trained mount. If the eidolon's base form is not quadruped or serpentine, it also gains the ability to transmute its physical body into a form suitable for you to ride. This functions as the change shape ability, except the eidolon's base form changes to either quadruped or serpentine and its size changes to be one size category larger than its summoner's base size. Unlike other changes to size, this ability doesn't increase the eidolon's ability scores, reach, or weapon damage beyond that of its true form. (However, if the eidolon's size is reduced by this ability, its reach and weapon damage are adjusted accordingly.) &nl;If the eidolon's base form is not quadruped or serpentine when it gains this feat, it chooses which base form (quadruped or serpentine) to assume when using this ability. The eidolon cannot choose a base form that is not available to its subtype with this ability. If both base forms are available to the eidolon's subtype, it can change which base form it assumes when using this ability whenever it gains a new Hit Die. The eidolon can select evolutions that have either base form as a requirement, but any evolutions that require one of the base forms (but not the other) provide no benefit while the eidolon is assuming the shape of its other base form."),
                prerequisites: None,
            },
            // Energized Wild Shape -- uw_feats.lst:40
            UwFeatEntry {
                key: "Energized Wild Shape",
                category: FeatCategory::General,
                name: "Energized Wild Shape",
                description: Some("Your wild shape form gains the benefits of one energy type for both offensive and defensive purposes."),
                pretext: None,
                source_page: Some("p.110"),
                benefit: Some("When you assume a wild shape form, choose one of the following energy types: acid, cold, electricity, or fire. You gain resistance 10 to that energy type. Also, one of your natural attacks deals an additional 1d6 points of damage of the chosen energy type. If you choose a wild shape form that already has energy resistance of the same type you choose, it increases by 5 instead. If you choose a wild shape form that deals damage of the same energy type you choose, increase the energy damage you deal by one die size (1d6 becomes 1d8, and so on).&nl;[Special] You can take this feat more than once. Each time you take this feat, choose a different energy type."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape", "PRETOTALAB:6", "PREVARGTEQ:PreStatScore_WIS,19"]),
            },
            // Enhanced Gnome Magic -- uw_feats.lst:41
            UwFeatEntry {
                key: "Enhanced Gnome Magic",
                category: FeatCategory::General,
                name: "Enhanced Gnome Magic",
                description: Some("Your ties to the First World manifest in the form of magical abilities that tap into a natural element."),
                pretext: None,
                source_page: Some("p.110"),
                benefit: Some("Add one of the following to your gnome magic spell-like abilities: burning hands, corrosive touch, gentle breeze, icicle dagger, or shocking grasp. You can use this spell-like ability once per day."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Gnome ~ Gnome Magic", "PREFACT:1,TEMPLATES,IsGnome=true", "PRESKILL:1,Knowledge (Nature)=3", "PREVARGTEQ:PreStatScore_CHA,13"]),
            },
            // Exotic Heritage -- uw_feats.lst:42
            UwFeatEntry {
                key: "Exotic Heritage",
                category: FeatCategory::General,
                name: "Exotic Heritage",
                description: Some("Your blood carries hints of an extraplanar ancestor, granting you a talent for a certain skill."),
                pretext: None,
                source_page: Some("p.110"),
                benefit: Some("Choose a skill. You gain a +2 bonus on skill checks with that skill. If you have 10 or more ranks in the chosen skill, this bonus increases to +4. This bonus does not stack with that granted by Skill Focus. This feat counts as Skill Focus with the chosen skill for the purpose of meeting the prerequisites of the Eldritch Heritage feat. When you select Eldritch Heritage, if you use this feat as a prerequisite, you can choose a mutated version of your chosen bloodline as though you were a sorcerer with the wildblooded archetype. All other restrictions and requirements of Eldritch Heritage still apply."),
                prerequisites: None,
            },
            // Expert Cartographer -- uw_feats.lst:43
            UwFeatEntry {
                key: "Expert Cartographer",
                category: FeatCategory::General,
                name: "Expert Cartographer",
                description: Some("You are skilled at drawing quality maps, making your cartographic works both more useful and more valuable when sold."),
                pretext: None,
                source_page: Some("p.110"),
                benefit: Some("You gain a +4 bonus on Craft (maps) checks when creating a map, as well as 1 additional Discovery Point (see page 124 for more information about Discovery Points). If you succeed at the check by 5 or more, the list price of the map increases by 20%%."),
                prerequisites: Some(&["PRESKILL:1,Craft (Maps)=3"]),
            },
            // Expert Explorer -- uw_feats.lst:44
            UwFeatEntry {
                key: "Expert Explorer",
                category: FeatCategory::General,
                name: "Expert Explorer",
                description: Some("You are trained to seek out the unknown places of the wild and can explore more efficiently than your untrained rivals."),
                pretext: None,
                source_page: Some("p.110"),
                benefit: Some("You gain a +2 bonus on skill checks when using the exploration rules presented on pages 124-125 to detect features in a territory. If you succeed at such a skill check by 5 or more, you gain an additional 1d4 Discovery Points."),
                prerequisites: Some(&["PRESKILL:1,Knowledge (Nature)=5,Survival=5"]),
            },
            // Expert Salvager -- uw_feats.lst:45
            UwFeatEntry {
                key: "Expert Salvager",
                category: FeatCategory::General,
                name: "Expert Salvager",
                description: Some("You are adept at foraging and salvaging raw materials from even the most seemingly sparse of environments."),
                pretext: None,
                source_page: Some("p.111"),
                benefit: Some("You gain a +4 bonus on Craft checks for Craft skills in which you have at least 2 ranks and Spellcraft checks when crafting items by foraging alchemical supplies and material components, salvaging raw crafting materials, and salvaging raw magical item materials."),
                prerequisites: Some(&["PRESKILL:2,TYPE.Craft=2,Spellcraft=2"]),
            },
            // Extended Aspects -- uw_feats.lst:47
            UwFeatEntry {
                key: "Extended Aspects",
                category: FeatCategory::General,
                name: "Extended Aspects",
                description: Some("You can invoke your aspect's minor form longer than most."),
                pretext: None,
                source_page: Some("p.111"),
                benefit: Some("Add your Wisdom bonus (minimum 1) to the  number of minutes per day that you can use your shifter aspect ability to assume a minor form."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Shifter ~ Shifter Aspect"]),
            },
            // False Trail -- uw_feats.lst:48
            UwFeatEntry {
                key: "False Trail",
                category: FeatCategory::General,
                name: "False Trail",
                description: Some("You create a false trail to throw off pursuers, enabling you to increase your lead on those following you or make them lose your trail entirely."),
                pretext: None,
                source_page: Some("p.111"),
                benefit: Some("You can create a false trail in the wilderness via a combination of misleading footprints, discarded items, torn scraps of clothing, and other signs. When you create a false trail, you determine the direction it leads. A creature tracking you must succeed at a Perception or Survival check (DC %1) to determine the trail is fake when first encountering it. On a success, the creature can continue to track you as normal. The effect of a failure to identify the trail as false depends on the time and effort spent making it. &nl;Quick: A quick false trail takes 10 minutes to complete. A creature that fails to identify the trail as false follows it for 1d4x1,000 feet. &nl;[Normal] A normal false trail takes 1 hour to complete. A creature that fails to identify the trail as false follows it for 1d4 miles. &nl;Elaborate: An elaborate false trail takes 4 hours to complete. A creature that fails to identify the trail as false follows it for 2d6 miles. &nl;After a creature that fails to identify a false trail follows it for the determined distance, it can attempt another Perception or Survival check with a +5 bonus. On a success, the creature realizes it's following a false trail and can continue to track you as normal after backtracking to the start of the false trail (or wherever you diverged from the false trail). On a failure, the creature continues to move in the direction of the false trail for an additional mile. After each mile, it can attempt a new check with a cumulative +5 bonus.|10+TL/2+WIS"),
                prerequisites: Some(&["PRESKILL:1,Survival=3"]),
            },
            // Fey Insight -- uw_feats.lst:49
            UwFeatEntry {
                key: "Fey Insight",
                category: FeatCategory::General,
                name: "Fey Insight",
                description: Some("You have insight into the weird, alien minds of fey and can use this knowledge to more handily negotiate with or manipulate the capricious creatures."),
                pretext: None,
                source_page: Some("p.111"),
                benefit: Some("You gain a +2 bonus on Bluff, Diplomacy, and Sense Motive skill checks when interacting with creatures of the fey type. If you have 10 or more ranks in one of these skills, the bonus gained when interacting with creatures of the fey type increases to +4 for that skill."),
                prerequisites: Some(&["PRESKILL:1,Knowledge (Nature)=2,Knowledge (Planes)=2", "PREVARGTEQ:PreStatScore_WIS,13"]),
            },
            // Fey Performance -- uw_feats.lst:50
            UwFeatEntry {
                key: "Fey Performance",
                category: FeatCategory::General,
                name: "Fey Performance",
                description: Some("The wind, trees, earth, and local wildlife join in on your bardic performances."),
                pretext: None,
                source_page: Some("p.111"),
                benefit: Some("You can enhance your performance with nature's sights and sounds by expending an extra round of bardic performance at the start of the performance. The range of the performance increases by 30 feet. Furthermore, for the purposes of affecting blind and deaf creatures, this performance counts as having both audible and visible components. This feat doesn't function in environments devoid of animal and plant life."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Bardic Performance"]),
            },
            // Fey-Guarded -- uw_feats.lst:51
            UwFeatEntry {
                key: "Fey-Guarded",
                category: FeatCategory::General,
                name: "Fey-Guarded",
                description: Some("You have trained your mind to resist fey magic-both that cast by fey creatures and that cast by other denizens of the First World."),
                pretext: None,
                source_page: Some("p.111"),
                benefit: Some("You gain a +2 morale bonus on saving throws against enchantment and illusion spells and spell-like abilities cast by creatures of the fey type."),
                prerequisites: Some(&["PRESKILL:1,Knowledge (Nature)=3,Knowledge (Planes)=3"]),
            },
            // Flinging Charge -- uw_feats.lst:52
            UwFeatEntry {
                key: "Flinging Charge",
                category: FeatCategory::Combat,
                name: "Flinging Charge",
                description: Some("You hurl a weapon during your charge before drawing a new one to strike, enabling you to make another attack at the expense of the latter attack's accuracy."),
                pretext: None,
                source_page: Some("p.111"),
                benefit: Some("If you have a thrown weapon in hand when you begin charging, you can make a ranged attack with that weapon against the target of your charge at any point during your charge. You gain the +2 bonus for charging on the attack roll for this ranged attack. You can then immediately draw a melee weapon during your charge. You take a -5 penalty on the melee attack made at the end of the charge. When charging this way, you cannot use any effect that would grant you more than one attack at the end of your charge (such as pounce), nor can you use abilities that grant you other ranged attacks on a charge, such as the hurling charge rage power."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Quick Draw", "PRETOTALAB:6"]),
            },
            // Foebane Magic -- uw_feats.lst:53
            UwFeatEntry {
                key: "Foebane Magic",
                category: FeatCategory::General,
                name: "Foebane Magic",
                description: Some("The spells you cast are particularly effective against your favored enemies."),
                pretext: None,
                source_page: Some("p.112"),
                benefit: Some("Your favored enemies take a -1 penalty on saving throws against spells you cast. Furthermore, you gain your favored enemy bonus on Spellcraft checks to identify spells cast by favored enemies."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Basic Favored Enemy", "PRESPELLTYPE:1,ANY=1"]),
            },
            // Forester -- uw_feats.lst:54
            UwFeatEntry {
                key: "Forester",
                category: FeatCategory::General,
                name: "Forester",
                description: Some("You are a master of woodcraft and forest lore and can move through even the densest of trees with ease and grace."),
                pretext: None,
                source_page: Some("p.112"),
                benefit: Some("You ignore difficult terrain created by light or heavy undergrowth, and you ignore increased DCs for Acrobatics and Stealth checks in light or heavy undergrowth. In addition, you can use trees to shield yourself from attacks, gaining a +1 cover bonus to your AC whenever you are adjacent to a tree (including while climbing). If you are adjacent to two or more trees simultaneously, you gain a +2 cover bonus to your AC and a +1 bonus on Reflex saving throws."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Favored Terrain ~ Forest"]),
            },
            // Frightful Shape -- uw_feats.lst:55
            UwFeatEntry {
                key: "Frightful Shape",
                category: FeatCategory::General,
                name: "Frightful Shape",
                description: Some("Your wild shape form strikes fear into your opponents."),
                pretext: None,
                source_page: Some("p.112"),
                benefit: Some("When you use wild shape, it is supernaturally ferocious and disquieting in appearance, such that when you attack a creature, you can use this ferocity to frighten observers. Opponents within 30 feet of you when you attack must succeed at a Will save (DC %1) to resist being affected by your frightful shape. On a failed save, creatures with fewer Hit Dice than you become shaken, or panicked if they have 4 Hit Dice or fewer; in either case, the effect lasts for a number of rounds equal to your character level. An opponent that succeeds at its saving throw is immune to your frightful shape for 24 hours. This is a mind-affecting fear effect.|10+TL/2+CHA"),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape", "PRETOTALAB:9", "PREVARGTEQ:PreStatScore_WIS,19"]),
            },
            // Greater Beast Hunter -- uw_feats.lst:56
            UwFeatEntry {
                key: "Greater Beast Hunter",
                category: FeatCategory::Combat,
                name: "Greater Beast Hunter",
                description: Some("You are an apex hunter."),
                pretext: None,
                source_page: Some("p.112"),
                benefit: Some("If you are surprised by an animal native to the terrain you have chosen for Beast Hunter, you can act normally in the surprise round, though you are still considered flat-footed until it is your turn to act. You also gain a +4 bonus to confirm critical threats against animals native to the terrain you have chosen. &nl;[Special] If you have selected Beast Hunter multiple times, you gain the benefits of this feat for all of the terrains you have chosen for those feats."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Beast Hunter,Improved Beast Hunter", "PRESKILL:1,Knowledge (Nature)=6,Survival=6", "PRETOTALAB:6"]),
            },
            // Greater Hunter's Bond -- uw_feats.lst:57
            UwFeatEntry {
                key: "Greater Hunter's Bond",
                category: FeatCategory::Combat,
                name: "Greater Hunter's Bond",
                description: Some("Your allies can target additional foes."),
                pretext: None,
                source_page: Some("p.113"),
                benefit: Some("When you activate hunter's bond, you can select a number of targets equal to your Wisdom modifier as long as they all count as your favored enemy. &nl;[Normal] You select one target against which your allies receive your favored enemy bonus."),
                prerequisites: Some(&["PRECLASS:1,Ranger=12", "PREMULT:2,[PREABILITY:1,CATEGORY=Special Ability,Hunter's Bond ~ Companion],[PREABILITY:1,CATEGORY=FEAT,Improved Hunter's Bond]"]),
            },
            // Greater Spring Attack -- uw_feats.lst:58
            UwFeatEntry {
                key: "Greater Spring Attack",
                category: FeatCategory::Combat,
                name: "Greater Spring Attack",
                description: Some("You are a scything wind cutting through the battlefield as you topple your foes."),
                pretext: None,
                source_page: Some("p.113"),
                benefit: Some("When you use Spring Attack, you can select three targets to attack during your movement instead of one. The second attack made this way is made at your full base attack bonus - 5, and the third attack made this way is made at your full base attack bonus - 10. All restrictions of Spring Attack apply to each target, and your movement does not provoke attacks of opportunity from any of your targets. You can't target the same creature more than once. &nl;[Special] A monk of at least 18th level can select this feat as a monk bonus feat, but only if he has Improved Spring attack and Spring Attack."),
                prerequisites: Some(&["PREABILITY:6,CATEGORY=FEAT,Acrobatic Steps,Dodge,Improved Spring Attack,Mobility,Nimble Moves,Spring Attack", "PRETOTALAB:16", "PREVARGTEQ:PreStatScore_DEX,17"]),
            },
            // Greater Wilding Strike -- uw_feats.lst:59
            UwFeatEntry {
                key: "Greater Wilding Strike",
                category: FeatCategory::Combat,
                name: "Greater Wilding Strike",
                description: Some("You reach the pinnacle of your wilding strike abilities."),
                pretext: None,
                source_page: Some("p.113"),
                benefit: Some("The damage die of your unarmed strikes increases to 1d10 (or 1d8 if you are Small). This does not stack with any other effects that increase the damage die of your unarmed strikes, including levels in classes such as monk. Increases to your actual size category (such as enlarge person) still increase your damage die as normal."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Improved Unarmed Strike,Improved Wilding Strike,Wilding,Wilding Strike", "PRETOTALAB:16", "PREVARGTEQ:PreStatScore_STR,18"]),
            },
            // Group Shared Spells -- uw_feats.lst:60
            UwFeatEntry {
                key: "Group Shared Spells",
                category: FeatCategory::Teamwork,
                name: "Group Shared Spells",
                description: Some("You and your allies can cast spells through each other's familiars as if they were your own."),
                pretext: Some("Familiar with the share spells ability."),
                source_page: Some("p.113"),
                benefit: Some("You and any of your allies with this feat can cast spells with a target of \"you\" on each other's familiars as touch spells. Both the target familiar and that familiar's master must be willing for the spell to take effect. You can cast spells on each other's familiars even if the spells would not normally affect creatures of the targeted familiar's type."),
                prerequisites: None,
            },
            // Harder They Fall -- uw_feats.lst:61
            UwFeatEntry {
                key: "Harder They Fall",
                category: FeatCategory::Combat,
                name: "Harder They Fall",
                description: Some("You can work with an ally to move or knock over a foe that's too large for either of you to overcome alone."),
                pretext: None,
                source_page: Some("p.113"),
                benefit: Some("When you use the aid another action to grant an ally who also has this feat a +2 bonus on a bull rush or trip combat maneuver check, the ally can attempt that maneuver even against foes two or more size categories larger than herself."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Combat Expertise,Power Attack", "PREVARGTEQ:PreStatScore_STR,13", "PREVARGTEQ:PreStatScore_INT,13"]),
            },
            // Hide Worker -- uw_feats.lst:62
            UwFeatEntry {
                key: "Hide Worker",
                category: FeatCategory::General,
                name: "Hide Worker",
                description: Some("You excel at crafting armor made from the hides of animals or monsters."),
                pretext: None,
                source_page: Some("p.113"),
                benefit: Some("You gain a +4 bonus on Craft (armor) checks to create armor or shields from the hides of living creatures (such as dragonhide). If you succeed at a Craft (armor) check to create armor or shields from the hides of living creatures by 5 or more, you also decrease the cost to make the item by 10%%."),
                prerequisites: Some(&["PRESKILL:2,Craft (Armor)=3,Knowledge (Nature)=3"]),
            },
            // Ice Climber -- uw_feats.lst:63
            UwFeatEntry {
                key: "Ice Climber",
                category: FeatCategory::General,
                name: "Ice Climber",
                description: Some("You can move or clamber across slippery surfaces with great skill."),
                pretext: None,
                source_page: Some("p.113"),
                benefit: Some("When moving across a slippery surface (including but not limited to icy surfaces), you gain a +5 bonus on Climb checks and on Acrobatics checks to maintain your balance. If you fail a Climb check, you fall only if you fail the check by 10 or more. You also gain a +5 bonus on Perception checks to notice an avalanche and a +2 circumstance bonus on Reflex saves to avoid an avalanche."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Arctic Adaptation,Mountaineer", "PRESKILL:1,Climb=2"]),
            },
            // Improved Beast Hunter -- uw_feats.lst:64
            UwFeatEntry {
                key: "Improved Beast Hunter",
                category: FeatCategory::Combat,
                name: "Improved Beast Hunter",
                description: Some("You are well trained in stalking and hunting the animals of the wild."),
                pretext: None,
                source_page: Some("p.113"),
                benefit: Some("When fighting animals that are at least one size category larger than you and that are native to the terrain you have chosen with Beast Hunter, you gain a +4 bonus on combat maneuver checks and a +4 to your CMD against such animals' attempts to use combat maneuvers against you. Additionally, you gain a +2 bonus on Reflex saves against attacks by the animal that allow a Reflex save (such as attempting to avoid a trample attack). &nl;[Special] If you have selected Beast Hunter multiple times, you gain this benefit for all of the terrains you have chosen for those feats."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Beast Hunter", "PRESKILL:1,Knowledge (Nature)=3,Survival=3", "PRETOTALAB:3"]),
            },
            // Improved Hunter's Bond -- uw_feats.lst:65
            UwFeatEntry {
                key: "Improved Hunter's Bond",
                category: FeatCategory::Combat,
                name: "Improved Hunter's Bond",
                description: Some("You deepen your connection to your allies, granting them higher bonuses."),
                pretext: None,
                source_page: Some("p.113"),
                benefit: Some("When you activate hunter's bond, you can grant your allies your full favored enemy bonus against a single target. &nl;[Normal] Your allies receive half your favored enemy bonus against a single target."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Hunter's Bond ~ Companion", "PRECLASS:1,Ranger=9"]),
            },
            // Improved Natural Poison Harvester -- uw_feats.lst:66
            UwFeatEntry {
                key: "Improved Natural Poison Harvester",
                category: FeatCategory::General,
                name: "Improved Natural Poison Harvester",
                description: Some("You excel at harvesting poison from the many toxic creatures of the natural world."),
                pretext: None,
                source_page: Some("p.114"),
                benefit: Some("When you successfully harvest poison from a hazard or dead creature, you gain an additional 1d4 doses of poison."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Natural Poison Harvester", "PRESKILL:2,Craft (Alchemy)=9,Survival=9"]),
            },
            // Improved Spring Attack -- uw_feats.lst:67
            UwFeatEntry {
                key: "Improved Spring Attack",
                category: FeatCategory::Combat,
                name: "Improved Spring Attack",
                description: Some("You dart through the press of battle like a breeze, assaulting foes as you pass."),
                pretext: None,
                source_page: Some("p.114"),
                benefit: Some("When you use Spring Attack, you can select two targets to attack during your movement instead of one. The second attack made this way is made at your full base attack bonus - 5. All restrictions of Spring Attack apply to both targets, and your movement does not provoke attacks of opportunity from either target. You can't target the same creature twice. &nl;[Special] A monk of at least 14th level can select this feat as a monk bonus feat, but only if he has Spring Attack."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Dodge,Mobility,Nimble Moves,Spring Attack", "PRETOTALAB:9", "PREVARGTEQ:PreStatScore_DEX,15"]),
            },
            // Improved Wilding Strike -- uw_feats.lst:68
            UwFeatEntry {
                key: "Improved Wilding Strike",
                category: FeatCategory::Combat,
                name: "Improved Wilding Strike",
                description: Some("You gain greater skill with the natural power of your blows."),
                pretext: None,
                source_page: Some("p.114"),
                benefit: Some("The damage die of your unarmed strikes increases to 1d8 (or 1d6 if you are Small). This does not stack with any other effects that increase the damage die of your unarmed strikes, including levels in classes such as monk. Increases to your actual size category (such as enlarge person) still increase your damage die as normal."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Improved Unarmed Strike,Wilding,Wilding Strike", "PRETOTALAB:11", "PREVARGTEQ:PreStatScore_STR,16"]),
            },
            // Indomitable Mountain Avalanche -- uw_feats.lst:69
            UwFeatEntry {
                key: "Indomitable Mountain Avalanche",
                category: FeatCategory::Combat,
                name: "Indomitable Mountain Avalanche",
                description: Some("You are an impassable mountain, hurling your enemies back."),
                pretext: None,
                source_page: Some("p.114"),
                benefit: Some("If an opponent fails at a combat maneuver against you, you can use an attack of opportunity to make a melee attack against the opponent using your highest attack bonus. If the attack hits, instead of dealing damage, you can push the opponent back 5 feet for every 5 by which it failed its combat maneuver check (minimum 5 feet). Additionally, when you hit with an unarmed melee attack against an opponent who provoked an attack of opportunity by failing its Acrobatics check to move through a space you threaten, you can push the opponent back 5 feet instead of dealing damage. The opponent moves back in a straight line, but stops before hitting any obstacle or hazard."),
                prerequisites: Some(&["PREABILITY:4,CATEGORY=FEAT,Combat Reflexes,Improved Unarmed Strike,Indomitable Mountain Peak,Indomitable Mountain Style", "PRETOTALAB:9", "PREVARGTEQ:PreStatScore_WIS,15"]),
            },
            // Indomitable Mountain Peak -- uw_feats.lst:70
            UwFeatEntry {
                key: "Indomitable Mountain Peak",
                category: FeatCategory::Combat,
                name: "Indomitable Mountain Peak",
                description: Some("You become more difficult to move or pass, an elusive peak your enemies cannot surmount."),
                pretext: None,
                source_page: Some("p.114"),
                benefit: Some("Whenever an opponent succeeds at a combat maneuver against you or at an Acrobatics check to avoid provoking an attack of opportunity when moving through a square you threaten, you immediately gain a +2 morale bonus to your CMD against the next combat maneuver against you or to the DC of the next Acrobatics check to avoid an attack of opportunity when moving through a square you threaten. &nl;This bonus stacks until either attempt fails against you."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Combat Reflexes,Improved Unarmed Strike,Indomitable Mountain Style", "PRETOTALAB:6", "PREVARGTEQ:PreStatScore_WIS,15"]),
            },
            // Indomitable Mountain Style -- uw_feats.lst:71
            UwFeatEntry {
                key: "Indomitable Mountain Style",
                category: FeatCategory::Combat,
                name: "Indomitable Mountain Style",
                description: Some("Like a rugged mountain, you are impassable and unmovable."),
                pretext: None,
                source_page: Some("p.115"),
                benefit: Some("As long as you do not take a move action, you gain a +4 morale bonus to your CMD and to the DC of Bluff checks used for feint attempts against you. Additionally, you are considered to be one size category larger for the purpose of calculating your CMD when creatures attempt Acrobatics checks to move through spaces you threaten without provoking attacks of opportunity."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Combat Reflexes,Improved Unarmed Strike", "PRETOTALAB:4", "PREVARGTEQ:PreStatScore_WIS,13"]),
            },
            // Intimidate Animals -- uw_feats.lst:72
            UwFeatEntry {
                key: "Intimidate Animals",
                category: FeatCategory::Combat,
                name: "Intimidate Animals",
                description: Some("You are skilled at intimidating animals with your bluster and fury."),
                pretext: None,
                source_page: Some("p.115"),
                benefit: Some("You add your Wisdom modifier on Intimidate checks against animals in addition to your Charisma modifier. An animal with 4 or fewer Hit Dice is frightened instead of shaken on a successful Intimidate check to demoralize it."),
                prerequisites: Some(&["PREMULT:2,[PREABILITY:1,CATEGORY=Special Ability,Wild Empathy,TYPE.WildEmpathy],[PREABILITY:1,CATEGORY=FEAT,Greater Wild Empathy]", "PRESKILL:2,Intimidate=5,Knowledge (Nature)=5"]),
            },
            // Jaguar Pounce -- uw_feats.lst:73
            UwFeatEntry {
                key: "Jaguar Pounce",
                category: FeatCategory::Combat,
                name: "Jaguar Pounce",
                description: Some("Your ambushes are especially lethal."),
                pretext: None,
                source_page: Some("p.115"),
                benefit: Some("When making a charge or using Spring Attack against an opponent who is flat-footed or helpless, you treat your first melee attack against that target as if you had the Improved Critical feat."),
                prerequisites: Some(&["PRETOTALAB:4"]),
            },
            // Jungle Survivalist -- uw_feats.lst:74
            UwFeatEntry {
                key: "Jungle Survivalist",
                category: FeatCategory::General,
                name: "Jungle Survivalist",
                description: Some("Normally pestilential rain forests are a safe haven and home to you."),
                pretext: None,
                source_page: Some("p.115"),
                benefit: Some("You gain a +2 bonus on saving throws against diseases, poisons, and the distraction ability of creatures with the swarm subtype. You also gain a +2 bonus on Acrobatics and Climb checks when climbing trees and a +2 bonus on Perception checks against creatures that gain a racial bonus on Stealth checks in vegetation."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Favored Terrain ~ Jungle"]),
            },
            // Live Off the Land -- uw_feats.lst:75
            UwFeatEntry {
                key: "Live Off the Land",
                category: FeatCategory::General,
                name: "Live Off the Land",
                description: Some("You can easily pluck what you need from nature's bounty as you travel."),
                pretext: None,
                source_page: Some("p.115"),
                benefit: Some("You (and any allies whom you assist with the Survival skill) can move at full speed while using Survival to gather food and water, and you gain a +4 bonus on Survival checks to do so. If you cast create food and drink, create water, or any similar spell that creates edible and potable provisions while you are in your favored terrain, your caster level increases by an amount equal to half your favored terrain bonus."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Basic Favored Terrain"]),
            },
            // Moontouched -- uw_feats.lst:76
            UwFeatEntry {
                key: "Moontouched",
                category: FeatCategory::General,
                name: "Moontouched",
                description: Some("Moonlight has a strange effect on you, making your body stronger but your mind weaker."),
                pretext: None,
                source_page: Some("p.115"),
                benefit: Some("At night when at least part of the moon is showing and you can see it, you gain a +2 bonus on Fortitude and Reflex saves but take a -2 penalty on Will saves."),
                prerequisites: None,
            },
            // Mountaineer -- uw_feats.lst:77
            UwFeatEntry {
                key: "Mountaineer",
                category: FeatCategory::General,
                name: "Mountaineer",
                description: Some("You are thoroughly at home in the high peaks and precipices of alpine territory."),
                pretext: None,
                source_page: Some("p.115"),
                benefit: Some("You gain a +2 bonus on Climb checks on natural stone surfaces and a +4 bonus on Fortitude saves to avoid high-altitude fatigue and altitude sickness. If you spend 24 hours at a dangerous altitude, you treat that altitude as if it were one category lower. If you spend at least 1 week at that altitude, you are immune to altitude fatigue or sickness. If you go to a lower altitude for more than 1 week, you lose this acclimation."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Favored Terrain ~ Mountains"]),
            },
            // Mutated Shape -- uw_feats.lst:79
            UwFeatEntry {
                key: "Mutated Shape",
                category: FeatCategory::General,
                name: "Mutated Shape",
                description: Some("Your wild shape form gains an additional appendage you can use to attack your foes."),
                pretext: None,
                source_page: Some("p.115"),
                benefit: Some("When you use wild shape, you grow an additional appendage of your choice that can be used to make one of the following attacks: bite, claw, gore, slam, sting, or talons. The appendage can be used as part of a full attack using your highest base attack bonus, and it deals damage as described in Table 3-2: Mutated Shape Appendage Attacks. This appendage lasts for as long as you stay in the same form with this use of wild shape. &nl;[SMALL-Claw, Slam, Sting, or Talons deal 1d3; Bite or Gore deal 1d4] [MEDIUM-Claw, Slam, Sting, or Talons deal 1d4; Bite or Gore deal 1d6]"),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape", "PRETOTALAB:6", "PREVARGTEQ:PreStatScore_WIS,19"]),
            },
            // Natural Poison Antitoxin -- uw_feats.lst:80
            UwFeatEntry {
                key: "Natural Poison Antitoxin",
                category: FeatCategory::General,
                name: "Natural Poison Antitoxin",
                description: Some("You have learned how to create antitoxin that can resist natural poisons."),
                pretext: None,
                source_page: Some("p.115"),
                benefit: Some("When you succeed at a Craft (alchemy) check to create an antitoxin (Ultimate Equipment 100) using a dose of a natural poison you harvested, you increase the duration of the antitoxin to an amount equal to 4 hours plus an additional hour for every 5 by which your result exceeds the DC."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Natural Poison Harvester", "PRESKILL:2,Craft (Alchemy)=8,Survival=8"]),
            },
            // Natural Poison Harvester -- uw_feats.lst:81
            UwFeatEntry {
                key: "Natural Poison Harvester",
                category: FeatCategory::General,
                name: "Natural Poison Harvester",
                description: Some("You are exceedingly proficient at harvesting and distilling poison from venomous monsters."),
                pretext: None,
                source_page: Some("p.116"),
                benefit: Some("You gain a +2 bonus on Craft (alchemy) checks when creating poison harvested from poisonous monsters. When you successfully craft such poisons, the DC of the poison increases by 2."),
                prerequisites: Some(&["PRESKILL:2,Craft (Alchemy)=6,Survival=6"]),
            },
            // Nature's Freedom -- uw_feats.lst:82
            UwFeatEntry {
                key: "Nature's Freedom",
                category: FeatCategory::General,
                name: "Nature's Freedom",
                description: Some("Creatures affected by your channel energy are not affected by difficult terrain."),
                pretext: None,
                source_page: Some("p.116"),
                benefit: Some("When you expend one additional use when you channel energy, you enable affected creatures to move freely through undergrowth as per the druid's woodland stride class feature. This effect lasts a for number of minutes equal to your Charisma modifier (minimum 1). Multiple uses of this feat extend the duration of the woodland stride effect."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Catch Off-Guard", "PRESKILL:1,Knowledge (Nature)=2,Survival=2", "PRETOTALAB:2"]),
            },
            // Nature's Weapons -- uw_feats.lst:83
            UwFeatEntry {
                key: "Nature's Weapons",
                category: FeatCategory::Combat,
                name: "Nature's Weapons",
                description: Some("You can make a weapon out of nearly anything that can be found in the wild."),
                pretext: None,
                source_page: Some("p.116"),
                benefit: Some("You can turn nearly anything you might find in the wilderness into an improvised melee weapon. This improvised melee weapon is one-handed and deals 1d6 points of damage for Medium creatures and 1d4 for Small creatures. Depending on the weapon, it deals bludgeoning, piercing, or slashing damage (at the GM's discretion). Grabbing an object to use as a weapon is the same action as drawing a weapon. The weapon must be of a material that is naturally occurring in that terrain."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Catch Off-Guard", "PRESKILL:1,Knowledge (Nature)=2,Survival=2", "PRETOTALAB:2"]),
            },
            // Night Sky Hex -- uw_feats.lst:84
            UwFeatEntry {
                key: "Night Sky Hex",
                category: FeatCategory::General,
                name: "Night Sky Hex",
                description: Some("You use the mystical network of power that blazes in the night sky to empower your hexes."),
                pretext: None,
                source_page: Some("p.116"),
                benefit: Some("When you gain this feat, choose one hex that you can use to affect no more than one opponent. When you are in view of the night sky and use that hex, increase the save DC of that hex by 2."),
                prerequisites: Some(&["PREMULT:2,[PREABILITY:1,CATEGORY=Special Ability,Witch ~ Hex],[PREABILITY:1,CATEGORY=Special Ability,Witch Patron ~ Moon,Witch Patron ~ Stars,Witch Patron ~ Winter]"]),
            },
            // One Eye Open -- uw_feats.lst:85
            UwFeatEntry {
                key: "One Eye Open",
                category: FeatCategory::General,
                name: "One Eye Open",
                description: Some("Your senses seek out threats even while you sleep."),
                pretext: None,
                source_page: Some("p.116"),
                benefit: Some("The DCs of your Perception checks don't increase when you are asleep. If you succeed at a Perception check to notice something dangerous while asleep, you can wake up to confront the danger. &nl;[Normal] The DC for a Perception check attempted by a sleeping creature increases by 10."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Alertness", "PREVARGTEQ:PreStatScore_WIS,13"]),
            },
            // One with the Land -- uw_feats.lst:86
            UwFeatEntry {
                key: "One with the Land",
                category: FeatCategory::General,
                name: "One with the Land",
                description: Some("Your connection your favored terrain is so intense that you draw strength from the power of nature itself."),
                pretext: None,
                source_page: Some("p.117"),
                benefit: Some("In your favored terrain, you require only half the normal amount of food, water, and sleep, and your rate of natural healing is doubled. You add half your favored terrain bonus as a bonus on saving throws and Constitution checks to stave off the effects of cold exposure, heat exposure, starvation, and thirst in your favored terrain."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Basic Favored Terrain"]),
            },
            // Out of the Sun -- uw_feats.lst:87
            UwFeatEntry {
                key: "Out of the Sun",
                category: FeatCategory::Teamwork,
                name: "Out of the Sun",
                description: Some("You and your allies use the sun's glare to your advantage."),
                pretext: None,
                source_page: Some("p.117"),
                benefit: Some("When you hit with a melee attack, you can attempt a Bluff check as a move action to feint. You do not gain the benefits of this feint; instead, if you succeed at your Bluff check, an ally with this feat who is adjacent to you or your opponent gains the benefits, such that the opponent is denied its Dexterity bonus to AC (if any) against your ally's next melee attack against that opponent. This attack must be made before the end of your ally's next turn. You gain a +2 circumstance bonus on your Bluff check in bright light. This bonus increases to +4 in natural sunlight. You cannot use this feat in dim light or darkness. &nl;[Special] If you have the Improved Feint feat, you can feint with Out of the Sun as a swift action after you hit with a melee attack."),
                prerequisites: Some(&["PRESKILL:2,Bluff=3,Stealth=3"]),
            },
            // Plains Nomad -- uw_feats.lst:88
            UwFeatEntry {
                key: "Plains Nomad",
                category: FeatCategory::General,
                name: "Plains Nomad",
                description: Some("You are a native of the wide and endless prairie, savanna, or steppe."),
                pretext: None,
                source_page: Some("p.117"),
                benefit: Some("You and a mount you are riding gain a +2 bonus on Constitution checks to avoid damage and fatigue from hustling or a forced march. In plains terrain, this bonus is doubled and also applies on Survival checks to avoid getting lost, to find food and water, to protect yourself from severe weather, and to predict the weather."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Favored Terrain ~ Plains"]),
            },
            // River Raider -- uw_feats.lst:89
            UwFeatEntry {
                key: "River Raider",
                category: FeatCategory::Combat,
                name: "River Raider",
                description: Some("You're skilled at creeping up on watercraft without being noticed and getting the jump on their occupants."),
                pretext: None,
                source_page: Some("p.117"),
                benefit: Some("You gain a +2 bonus on Swim and Stealth checks while swimming in calm or rough water (but not while swimming in stormy water). In addition, if you're able to act in a surprise round and you start the surprise round in water, you can take both a move and a standard action. You still cannot take a full-round action during the surprise round. &nl;[Normal] You can take only a move or a standard action if you can act in the surprise round."),
                prerequisites: None,
            },
            // Rubble Skirmisher -- uw_feats.lst:90
            UwFeatEntry {
                key: "Rubble Skirmisher",
                category: FeatCategory::Combat,
                name: "Rubble Skirmisher",
                description: Some("You use difficult terrain to your advantage when performing combat maneuvers."),
                pretext: None,
                source_page: Some("p.117"),
                benefit: Some("When you are in a square that has naturally occurring difficult terrain (bushes, rubble, undergrowth, and so forth), you gain a +2 circumstance bonus on combat maneuver checks and to your CMD."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Nimble Moves", "PRETOTALAB:2", "PREVARGTEQ:PreStatScore_DEX,13"]),
            },
            // Scion of the Land -- uw_feats.lst:91
            UwFeatEntry {
                key: "Scion of the Land",
                category: FeatCategory::Teamwork,
                name: "Scion of the Land",
                description: Some("You are strongly linked to natural terrain and to others who share your link."),
                pretext: None,
                source_page: Some("p.117"),
                benefit: Some("When traveling through terrain in which you can leave no trail and be impossible to track (as a result of favored terrain, trackless step, or a similar class feature), you grant that ability to allies within 60 feet who also have this feat. &nl;If an ally within 60 feet who has this feat is also able to move normally through the current terrain without leaving tracks, you gain a +1 bonus on Knowledge (nature), Perception, and Survival checks while in that terrain."),
                prerequisites: Some(&["PRESKILL:1,Survival=1"]),
            },
            // Shifter's Edge -- uw_feats.lst:93
            UwFeatEntry {
                key: "Shifter's Edge",
                category: FeatCategory::General,
                name: "Shifter's Edge",
                description: Some("You use your shapechanging powers to make your natural attacks especially lethal."),
                pretext: None,
                source_page: Some("p.117"),
                benefit: Some("Whenever you use Weapon Finesse to make a melee attack with your claws or a natural attack augmented by your claws, and you use your Dexterity bonus on attack rolls and your Strength modifier on damage rolls, you also add half your shifter level to the damage."),
                prerequisites: Some(&["PREMULT:2,[PREVAREQ:HasWeaponFinesseFeat,1],[PREABILITY:1,CATEGORY=Special Ability,Shifter ~ Shifter Claws]", "PREVARGTEQ:PreStatScore_DEX,13"]),
            },
            // Shifter's Rush -- uw_feats.lst:94
            UwFeatEntry {
                key: "Shifter's Rush",
                category: FeatCategory::General,
                name: "Shifter's Rush",
                description: Some("You can use you wild shape ability as you are moving toward your enemy."),
                pretext: None,
                source_page: Some("p.117"),
                benefit: Some("When you use a move action to move 10 feet or more or when you charge, you can use wild shape as a free action during that movement."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape", "PRECLASS:1,Shifter=4"]),
            },
            // Storm Survivor -- uw_feats.lst:95
            UwFeatEntry {
                key: "Storm Survivor",
                category: FeatCategory::General,
                name: "Storm Survivor",
                description: Some("You know how to survive and thrive in even the stormiest of weathers."),
                pretext: None,
                source_page: Some("p.117"),
                benefit: Some("When you are in a storm of any type, you gain a +2 circumstance bonus on Perception checks, Survival checks, and saving throws regarding adverse effects from the storm itself. If you have 10 or more ranks in one of these skills, this bonus increases to +4."),
                prerequisites: Some(&["PRESKILL:2,Knowledge (Nature)=2,Survival=2"]),
            },
            // Swamper -- uw_feats.lst:96
            UwFeatEntry {
                key: "Swamper",
                category: FeatCategory::General,
                name: "Swamper",
                description: Some("The soft ground and flowing reeds of fetid bogs and misty marshes are as comfortable to you as any sunny meadow would be for others."),
                pretext: None,
                source_page: Some("p.117"),
                benefit: Some("You gain a +2 bonus on Climb checks, Swim checks, and Acrobatics checks to maintain your balance when moving across a slippery surface, and you can move at full speed through shallow and deep bog terrain. You can see twice as far as normal in fog, mist, murky water, vegetation, and similarly obscuring conditions, and creatures adjacent to you never gain concealment from fog, mist, murky water, or vegetation."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Favored Terrain ~ Swamp"]),
            },
            // Thrill of the Hunt -- uw_feats.lst:97
            UwFeatEntry {
                key: "Thrill of the Hunt",
                category: FeatCategory::Combat,
                name: "Thrill of the Hunt",
                description: Some("The pursuit of prey invigorates you. You revel in tracking down and slaying your quarry."),
                pretext: None,
                source_page: Some("p.118"),
                benefit: Some("Once per day, when you succeed at a Survival check to find or follow a creature's tracks, you can designate that creature to be your prize for a number of hours equal to 4 + your ranks in Survival. You gain a +2 morale bonus on Survival checks to follow your prize's tracks and on weapon damage rolls against your prize. If you find and subsequently render your prize dead or helpless, you gain a +2 morale bonus on attack rolls, saves, and skill checks for a number of hours equal to your ranks in Survival. &nl;[Special] An animal companion can choose this feat as if it were an animal feat (Core Rulebook 53)."),
                prerequisites: Some(&["PREMULT:1,[PRETOTALAB:4],[PREABILITY:1,CATEGORY=Special Ability,Shifter ~ Track,Ranger ~ Track]", "PRESKILL:1,Survival=1"]),
            },
            // Torrid Tolerance -- uw_feats.lst:98
            UwFeatEntry {
                key: "Torrid Tolerance",
                category: FeatCategory::General,
                name: "Torrid Tolerance",
                description: Some("You relish hot climates, as the equatorial heat does not sap your strength."),
                pretext: None,
                source_page: Some("p.118"),
                benefit: Some("You treat hot environments (Core Rulebook 444) as though they were one step less severe; if you have a similar ability from another feat (like Desert Dweller), the benefits stack and you treat hot conditions as if they were two steps less severe. You do not take a penalty on Constitution checks for wearing armor in hot climates."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Desert Dweller,Jungle Survivalist"]),
            },
            // Totemic Disciple -- uw_feats.lst:100
            UwFeatEntry {
                key: "Totemic Disciple",
                category: FeatCategory::General,
                name: "Totemic Disciple",
                description: Some("Your understanding of your barbaric totem flourishes as your combat prowess grows."),
                pretext: Some("No barbarian totem rage powers except those chosen with this feat or Totemic Initiate"),
                source_page: Some("p.118"),
                benefit: Some("Choose one barbarian totem rage power that requires that you have the totem rage power you selected with Totemic Initiate. While gaining the benefits of Totemic Initiate, you also gain the benefits of this new rage power."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Athletic,Totemic Initiate", "!PREALIGN:LG,LN,LE", "PRETOTALAB:9", "PREVARGTEQ:PreStatScore_STR,15", "PREVARGTEQ:PreStatScore_DEX,13"]),
            },
            // Totemic Initiate -- uw_feats.lst:101
            UwFeatEntry {
                key: "Totemic Initiate",
                category: FeatCategory::General,
                name: "Totemic Initiate",
                description: Some("You were born to or have lived among the barbarian tribes of the wild and passed the trials of their sacred totems."),
                pretext: Some("No barbarian totem rage powers except the one you choose with this feat"),
                source_page: Some("p.118"),
                benefit: Some("Choose one barbarian lesser totem rage power (such as lesser beast totem). As a swift action, you can gain the benefits of that rage power as though you were raging. For the purpose of determining that rage power's effects, your barbarian level is equal to your base attack bonus. You do not gain any other benefits or penalties of rage. You can gain these benefits for a number of rounds per day equal to your base attack bonus plus your Constitution modifier; these rounds do not need to be consecutive."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Athletic", "!PREALIGN:LG,LN,LE", "PRETOTALAB:5", "PREVARGTEQ:PreStatScore_STR,13", "PREVARGTEQ:PreStatScore_DEX,13"]),
            },
            // Totemic Master -- uw_feats.lst:102
            UwFeatEntry {
                key: "Totemic Master",
                category: FeatCategory::General,
                name: "Totemic Master",
                description: Some("Through both might of arms and spiritual wisdom, you have attained full understanding of your barbaric totem."),
                pretext: Some("No barbarian totem rage powers except those chosen with this feat, Totemic Disciple, or Totemic Initiate"),
                source_page: Some("p.119"),
                benefit: Some("Choose one greater totem rage power that requires the totem rage power you chose with Totemic Disciple. While gaining the benefits of Totemic Initiate, you also gain the benefits of this new rage power as well as the one you chose with Totemic Disciple. If you gain the pounce ability this way, you can make only natural attacks using your pounce ability."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Athletic,Totemic Disciple,Totemic Initiate", "!PREALIGN:LG,LN,LE", "PRETOTALAB:13", "PREVARGTEQ:PreStatScore_STR,17", "PREVARGTEQ:PreStatScore_DEX,13", "PREVARGTEQ:PreStatScore_WIS,13"]),
            },
            // Tree Leaper -- uw_feats.lst:103
            UwFeatEntry {
                key: "Tree Leaper",
                category: FeatCategory::General,
                name: "Tree Leaper",
                description: Some("You use the flexibility of tree branches to aid in making arboreal leaps."),
                pretext: None,
                source_page: Some("p.119"),
                benefit: Some("When you are in a tree, you count as having a running start when you jump. &nl;[Normal] You must have at least 10 feet of space for a running start."),
                prerequisites: None,
            },
            // Tribal Hunter -- uw_feats.lst:104
            UwFeatEntry {
                key: "Tribal Hunter",
                category: FeatCategory::Combat,
                name: "Tribal Hunter",
                description: Some("From watching pack animals hunt, you learned to take down large prey as part of a group."),
                pretext: None,
                source_page: Some("p.119"),
                benefit: Some("When either you or an ally with this feat is adjacent to and flanking an opponent that is larger than either of you, you both are considered to be flanking the opponent as long as you remain adjacent to it. &nl;[Normal] You must be positioned opposite an ally to flank an opponent."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Animal Affinity"]),
            },
            // Verdant Spell -- uw_feats.lst:105
            UwFeatEntry {
                key: "Verdant Spell",
                category: FeatCategory::Metamagic,
                name: "Verdant Spell",
                description: Some("Your magical connection to nature allows you to entice, fool, and misdirect plants and fungi as though they were people."),
                pretext: None,
                source_page: Some("p.119"),
                benefit: Some("A verdant spell affects plant creatures (even mindless plant creatures) as if they weren't immune to mind-affecting effects, but it has no effect on other types of creatures. A verdant spell uses up a spell slot 2 levels higher than the spell's actual level. This feat works only on mind-affecting spells. &nl;[Normal] Creatures of the plant type are immune to mind-affecting effects."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Enchantment)", "PRESKILL:1,Knowledge (Nature)=6"]),
            },
            // Vigilant Charger -- uw_feats.lst:106
            UwFeatEntry {
                key: "Vigilant Charger",
                category: FeatCategory::Combat,
                name: "Vigilant Charger",
                description: Some("You can ready yourself to spring an attack against your enemies at a moment's notice."),
                pretext: None,
                source_page: Some("p.119"),
                benefit: Some("You can ready an action to charge when an enemy enters a specific space you designate. You must be able to charge the square both when you ready the charge and when the readied action triggers, or you cannot attempt the readied action. When you make a readied charge, you can move only up to your speed. &nl;[Normal] You can't ready an action to charge. Charging allows you to move up to twice your speed."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Combat Reflexes", "PREVARGTEQ:PreStatScore_STR,13", "PREVARGTEQ:PreStatScore_DEX,13"]),
            },
            // Voice of Beasts -- uw_feats.lst:107
            UwFeatEntry {
                key: "Voice of Beasts",
                category: FeatCategory::General,
                name: "Voice of Beasts",
                description: Some("Your deep connection with nature allows you to speak with all manner of living creatures."),
                pretext: None,
                source_page: Some("p.119"),
                benefit: Some("While you are using wild shape, you gain the ability to communicate with all animals. This acts as speak with animals, though the effect is supernatural in nature and can't be dispelled."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape"]),
            },
            // Waterway Caster -- uw_feats.lst:108
            UwFeatEntry {
                key: "Waterway Caster",
                category: FeatCategory::General,
                name: "Waterway Caster",
                description: Some("You've learned to cast spells while on a watercraft or even while swimming in turbulent waters."),
                pretext: None,
                source_page: Some("p.119"),
                benefit: Some("You automatically succeed at concentration checks required to cast a spell while being subjected to vigorous or violent motion while either swimming or on a ship. You gain a +4 bonus on concentration checks to cast spells underwater."),
                prerequisites: None,
            },
            // Wild Growth Channel -- uw_feats.lst:109
            UwFeatEntry {
                key: "Wild Growth Channel",
                category: FeatCategory::General,
                name: "Wild Growth Channel",
                description: Some("When you channel positive energy, you cause vines to grow and ensnare enemies."),
                pretext: None,
                source_page: Some("p.119"),
                benefit: Some("By expending two additional uses when you channel energy, you cause the ground in the area of effect to erupt in a growth of vines. You create a number of vine growths (AC 10, 5 hp) equal to your Charisma modifier (minimum 1). Each growth of vines occupies 1 5-foot square of your choosing. As an immediate action when you use this ability, and again as a free action at the start of your turn, you can command the vines to wrap around the legs of a creature occupying the square. The creature must succeed at a Reflex save (DC = 10 + half your class level + your Charisma modifier) or become rooted to the spot. The vines last a number of rounds equal to your Charisma modifier (minimum 1). The ground must be composed of a material able to support plant life (such as soil)."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Channel Positive Energy", "PREDOMAIN:1,Plant"]),
            },
            // Wild Growth Hex -- uw_feats.lst:110
            UwFeatEntry {
                key: "Wild Growth Hex",
                category: FeatCategory::General,
                name: "Wild Growth Hex",
                description: Some("Hexing your enemies causes them to be ensnared in tangling vines."),
                pretext: None,
                source_page: Some("p.119"),
                benefit: Some("When you gain this feat, choose one hex that you can use to affect no more than one opponent. If the target of your hex fails its saving throw to resist your hex, the square it occupies becomes overgrown with thorny plants that count as difficult terrain. A creature moving in or out of the square must succeed at a Reflex save or Acrobatics check (DC %1) or take 1d6 points of piercing damage and immediately end its movement. For every 5 by which the target fails its save against your hex, you can create 1 additional square of difficult terrain adjacent to any square the target occupies. The difficult terrain lasts a number of rounds equal to your Charisma modifier (minimum 1). The ground must be composed of a material able to support plant life (such as soil).|10+TL/2+CHA"),
                prerequisites: Some(&["PREMULT:2,[PREABILITY:1,CATEGORY=Special Ability,Witch ~ Hex],[PREABILITY:1,CATEGORY=Special Ability,Witch Patron ~ Summer,Witch Patron ~ Thorns,Witch Patron ~ Woodlands]"]),
            },
            // Wild Vigor -- uw_feats.lst:111
            UwFeatEntry {
                key: "Wild Vigor",
                category: FeatCategory::General,
                name: "Wild Vigor",
                description: Some("The forces of nature bolster your vitality when you use wild shape."),
                pretext: None,
                source_page: Some("p.120"),
                benefit: Some("A number of times per day equal to your Wisdom bonus (minimum 1), when you use wild shape, you can gain a number of temporary hit points equal to your base attack bonus. These temporary hit points last for the duration of the wild shape or until they are lost."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape"]),
            },
            // Wilding -- uw_feats.lst:112
            UwFeatEntry {
                key: "Wilding",
                category: FeatCategory::General,
                name: "Wilding",
                description: Some("You were touched by nature at an early age and share a kinship with wild creatures. Your body might bear animalistic features, such as bestial ears or a tail, or your presence may be subtly unlike that of others."),
                pretext: None,
                source_page: Some("p.120"),
                benefit: Some("You gain the druid's wild empathy ability, using your character level as your druid level. If you have one or more levels in a class that grants wild empathy, you gain an additional +3 bonus on wild empathy checks. You are treated as an animal for the purpose of harmful mind-affecting effects that target animals (such as charm animal). &nl;[Special] A character who has this feat can select the Animal Soul or Aspect of the Beast feats without meeting the prerequisites."),
                prerequisites: Some(&["PREALIGN:NG,NE,TN,CN,LN", "PRELEVEL:MAX=1"]),
            },
            // Wilding Mind -- uw_feats.lst:113
            UwFeatEntry {
                key: "Wilding Mind",
                category: FeatCategory::General,
                name: "Wilding Mind",
                description: Some("You can repress your conscious mind in favor of a primal state that helps you resist mental influence."),
                pretext: None,
                source_page: Some("p.120"),
                benefit: Some("Whenever you fail a saving throw against a fear or mind-affecting effect, you can take 1d3 points of Intelligence damage as an immediate action and reroll the saving throw. This damage cannot be reduced or avoided in any way. You must take the new result, even if it is worse."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Wilding", "PREVARGTEQ:PreStatScore_WIS,13"]),
            },
            // Wilding Senses -- uw_feats.lst:114
            UwFeatEntry {
                key: "Wilding Senses",
                category: FeatCategory::General,
                name: "Wilding Senses",
                description: Some("The wilderness speaks to you, giving you a preternatural sense of approaching danger."),
                pretext: None,
                source_page: Some("p.120"),
                benefit: Some("You gain a +2 bonus on Perception checks; this bonus increases to +4 when determining if you can act during a surprise round. If you have 10 or more ranks in Perception, this bonus increases to +4 (or +8 when determining whether you can act during a surprise round). This bonus does not stack with that granted by the Alertness feat, though Wilding Senses counts as the Alertness feat for the purpose of fulfilling feat prerequisites and prestige class requirements."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Wilding", "PREVARGTEQ:PreStatScore_WIS,13"]),
            },
            // Wilding Stride -- uw_feats.lst:115
            UwFeatEntry {
                key: "Wilding Stride",
                category: FeatCategory::General,
                name: "Wilding Stride",
                description: Some("Your wild-hearted drive and honed physique allow you to move with great swiftness."),
                pretext: None,
                source_page: Some("p.120"),
                benefit: Some("Your base speed increases by 10 feet. This does not stack with the fast movement class feature or similar effects, unless that class feature or effect provides an enhancement bonus."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Wilding", "PREVARGTEQ:PreStatScore_WIS,13"]),
            },
            // Wilding Strike -- uw_feats.lst:116
            UwFeatEntry {
                key: "Wilding Strike",
                category: FeatCategory::Combat,
                name: "Wilding Strike",
                description: Some("Filled with the might of nature, you need no weapons of steel or wood. Your fists and feet are as potent as any creation of the forge."),
                pretext: None,
                source_page: Some("p.120"),
                benefit: Some("The damage die of your unarmed strikes increases to 1d6 (or 1d4 if you are Small). This does not stack with any other effects that increase the damage die of your unarmed strikes, including levels in classes such as monk. Increases to your actual size category (such as enlarge person) still increase your damage die as normal."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Wilding", "PREVARGTEQ:PreStatScore_STR,13"]),
            },
            // Witchbreaker -- uw_feats.lst:117
            UwFeatEntry {
                key: "Witchbreaker",
                category: FeatCategory::Combat,
                name: "Witchbreaker",
                description: Some("You are trained to be resilient to and disrupt the magic of hags and witches."),
                pretext: None,
                source_page: Some("p.120"),
                benefit: Some("You gain a +2 bonus on saving throws against the hexes, spells, spell-like abilities, and supernatural abilities of hags and witches. In addition, whenever you confirm a critical hit against a hag or a witch, any of your allies affected by a mind-affecting effect from that creature can attempt a new saving throw against the effect as an immediate action."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Iron Will"]),
            },
            // Wolf Rider -- uw_feats.lst:118
            UwFeatEntry {
                key: "Wolf Rider",
                category: FeatCategory::General,
                name: "Wolf Rider",
                description: Some("Others can keep their horses. You know that wolves are better mounts."),
                pretext: None,
                source_page: Some("p.120"),
                benefit: Some("You can select a wolf in place of the normal mount available via your mount or divine bond (mount) class feature. In addition, Knowledge (nature) is always a class skill for you."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Special Mount,TYPE.Mount", "PREMULT:1,[PREABILITY:1,CATEGORY=FEAT,Undersized Mount],[PREMULT:1,[PREVARGTEQ:SpecialMountLVL,7],[PREVARGTEQ:CavalierMountLVL,7],[PREVARGTEQ:SamuraiMountLVL,7]]", "PRESKILL:1,Knowledge (Nature)=1"]),
            },
            // Wolf Savage -- uw_feats.lst:119
            UwFeatEntry {
                key: "Wolf Savage",
                category: FeatCategory::Combat,
                name: "Wolf Savage",
                description: Some("You savage your foes so badly that they can become supernaturally disfigured."),
                pretext: None,
                source_page: Some("p.120"),
                benefit: Some("While using Wolf Style, when you deal at least 10 points of damage to a prone opponent with a natural weapon or an unarmed strike, as a swift action you can savage that creature. When you do, your opponent must succeed at a Fortitude save (DC = 10 + half your character level + your Wisdom modifier). If the target fails the saving throw, it takes either 1d4 Charisma damage or 1d4 Constitution damage, or it becomes fatigued (your choice). Ability score damage dealt with this ability cannot equal or exceed the victim's actual ability score total."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Improved Unarmed Strike,Wolf Style,Wolf Trip", "PRESKILL:1,Knowledge (Nature)=9", "PREVARGTEQ:PreStatScore_WIS,17"]),
            },
            // Wolf Style -- uw_feats.lst:120
            UwFeatEntry {
                key: "Wolf Style",
                category: FeatCategory::Combat,
                name: "Wolf Style",
                description: Some("While in this style, you hamper foes that turn their backs on you."),
                pretext: None,
                source_page: Some("p.121"),
                benefit: Some("While using this style, whenever you deal at least 10 points of damage to a foe with an attack of opportunity, that foe's base speed decreases by 5 feet until the end of its next turn. For every 10 points of damage your attack deals beyond 10, the foe's base speed decreases by an additional 5 feet. If the penalty meets or exceeds the total base speed of the foe, you can attempt to trip the foe as a free action after the attack of opportunity is resolved."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Improved Unarmed Strike", "PRESKILL:1,Knowledge (Nature)=3", "PREVARGTEQ:PreStatScore_WIS,13"]),
            },
            // Wolf Trip -- uw_feats.lst:121
            UwFeatEntry {
                key: "Wolf Trip",
                category: FeatCategory::Combat,
                name: "Wolf Trip",
                description: Some("You have studied the manner in which wolves bring down their prey."),
                pretext: None,
                source_page: Some("p.121"),
                benefit: Some("While using Wolf Style, you gain a +2 bonus when you attempt a trip combat maneuver as part of an attack of opportunity. Whenever you successfully trip a creature, as a free action you can choose an available space that is both adjacent to you and the creature's original space for the tripped creature to land prone in."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Improved Unarmed Strike,Wolf Style", "PRESKILL:1,Knowledge (Nature)=6", "PREVARGTEQ:PreStatScore_WIS,15"]),
            },
            // Wood Crafter -- uw_feats.lst:122
            UwFeatEntry {
                key: "Wood Crafter",
                category: FeatCategory::General,
                name: "Wood Crafter",
                description: Some("Eschewing the use of metals in your craft, you excel at crafting armor from wood."),
                pretext: None,
                source_page: Some("p.121"),
                benefit: Some("You gain a +4 bonus on Craft (armor) checks to create armor and shields from wood or special materials containing wood (such as darkwood). If you succeed at the check by 5 or more, you decrease the cost to make the item by 10%%."),
                prerequisites: Some(&["PRESKILL:2,Craft (Armor)=3,Knowledge (Nature)=3"]),
            },
            // Woodland Wraith -- uw_feats.lst:123
            UwFeatEntry {
                key: "Woodland Wraith",
                category: FeatCategory::Combat,
                name: "Woodland Wraith",
                description: Some("You put the terrain between yourself and your opponents to avoid attacks."),
                pretext: None,
                source_page: Some("p.121"),
                benefit: Some("If you take two actions to move or a withdrawal action in a turn while starting or ending your movement in either naturally occurring difficult terrain (bushes, brambles, etc.) or a space with a natural object providing at least partial cover (a tree, rock, etc.), you gain concealment until you attack, until you move out of that space, or until the end of your next turn, whichever occurs first."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Dodge", "PRETOTALAB:6", "PREVARGTEQ:PreStatScore_DEX,15"]),
            },
            // Advanced Gathlain Magic -- uw_feats.lst:129
            UwFeatEntry {
                key: "Advanced Gathlain Magic",
                category: FeatCategory::General,
                name: "Advanced Gathlain Magic",
                description: Some("You have developed your innate magical abilities."),
                pretext: None,
                source_page: Some("p.13"),
                benefit: Some("You gain the following spells as spell-like abilities, each of which are usable 1/day: wood meld (as meld with stone, but only with wood), wood shape."),
                prerequisites: Some(&["PRELEVEL:MIN=3", "PREFACT:1,TEMPLATES,IsGathlain=true", "PREVARGTEQ:PreStatScore_CHA,13"]),
            },
            // Greater Gathlain Magic -- uw_feats.lst:130
            UwFeatEntry {
                key: "Greater Gathlain Magic",
                category: FeatCategory::General,
                name: "Greater Gathlain Magic",
                description: Some("You have greatly developed your innate magical abilities."),
                pretext: None,
                source_page: Some("p.13"),
                benefit: Some("You gain the following spells as spell-like abilities, each of which are usable 1/day: command plants, thorny entanglement."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Advanced Gathlain Magic", "PRELEVEL:MIN=9", "PREFACT:1,TEMPLATES,IsGathlain=true", "PREVARGTEQ:PreStatScore_CHA,15"]),
            },
            // Green Tongue -- uw_feats.lst:131
            UwFeatEntry {
                key: "Green Tongue",
                category: FeatCategory::General,
                name: "Green Tongue",
                description: Some("You know the language of the plants."),
                pretext: None,
                source_page: Some("p.13"),
                benefit: Some("You can cast speak with plants once per day, as a spell-like ability. At 10th level and every 5 levels thereafter, you can cast speak with plants an additional time per day."),
                prerequisites: Some(&["PRELEVEL:MIN=5", "PREFACT:1,TEMPLATES,IsGathlain=true", "PREVARGTEQ:PreStatScore_CHA,13"]),
            },
            // Seasoned Flier -- uw_feats.lst:132
            UwFeatEntry {
                key: "Seasoned Flier",
                category: FeatCategory::General,
                name: "Seasoned Flier",
                description: Some("You have trained long and hard at flight."),
                pretext: None,
                source_page: Some("p.13"),
                benefit: Some("Your maneuverability while flying improves to average."),
                prerequisites: Some(&["PREMOVE:1,Fly=1", "PREFACT:1,TEMPLATES,IsGathlain=true", "PRESKILL:1,Fly=5", "PREVARGTEQ:PreStatScore_DEX,15"]),
            },
            // Superior Gathlain Magic -- uw_feats.lst:133
            UwFeatEntry {
                key: "Superior Gathlain Magic",
                category: FeatCategory::General,
                name: "Superior Gathlain Magic",
                description: Some("Your experience has enabled you to dramatically develop your innate magical abilities."),
                pretext: None,
                source_page: Some("p.13"),
                benefit: Some("You gain the following spells as spell-like abilities, each usable 1/day: liveoak, tree stride."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Advanced Gathlain Magic,Greater Gathlain Magic", "PRELEVEL:MIN=15", "PREFACT:1,TEMPLATES,IsGathlain=true", "PREVARGTEQ:PreStatScore_CHA,15"]),
            },
            // Symbiotic Resilience -- uw_feats.lst:134
            UwFeatEntry {
                key: "Symbiotic Resilience",
                category: FeatCategory::General,
                name: "Symbiotic Resilience",
                description: Some("Your symbiotic vines have spread all throughout your body, granting you a lesser form of the immunities enjoyed by plant creatures."),
                pretext: None,
                source_page: Some("p.13"),
                benefit: Some("When you are paralyzed, magically put to sleep, or stunned, you ignore the effects of that condition, but you are instead staggered for the same duration as the ignored condition. Removing the ignored condition also removes this staggered condition, but you can't otherwise remove the staggered condition, nor can immunity to being staggered protect against it."),
                prerequisites: Some(&["PREFACT:1,TEMPLATES,IsGathlain=true", "PREVARGTEQ:PreStatScore_CON,13"]),
            },
            // Wandering Mind -- uw_feats.lst:135
            UwFeatEntry {
                key: "Wandering Mind",
                category: FeatCategory::General,
                name: "Wandering Mind",
                description: Some("Your mind drifts swiftly from one thing to the next and is all but impossible to pin down."),
                pretext: None,
                source_page: Some("p.13"),
                benefit: Some("When you fail a Will save against a mind-affecting effect that lasts more than 1 round, you can attempt another Will save on the round after your failed save. If you succeed, you gain the normal benefits of succeeding at a Will save against the effect. This feat offers only one additional saving throw against each effect."),
                prerequisites: Some(&["PRELEVEL:MIN=9", "PREFACT:1,TEMPLATES,IsGathlain=true"]),
            },
            // Delectable Feint -- uw_feats.lst:138
            UwFeatEntry {
                key: "Delectable Feint",
                category: FeatCategory::General,
                name: "Delectable Feint",
                description: Some("You can distract opponents with your delectable rinds."),
                pretext: None,
                source_page: Some("p.19"),
                benefit: Some("Other creatures find you so tasty that they become distracted in combat, allowing you to exploit their momentary lapses in concentration. You gain a +2 bonus on Bluff checks to feint living herbivorous or omnivorous creatures. In addition, whenever you successfully feint such an opponent, in addition to the normal effects of being feinted in combat, that opponent takes a -2 penalty on Perception checks and concentration checks until the start of your next turn."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Ghoran ~ Delicious", "PREFACT:1,TEMPLATES,IsGhoran=true", "PREVARGTEQ:PreStatScore_CON,15"]),
            },
            // Inner Light -- uw_feats.lst:139
            UwFeatEntry {
                key: "Inner Light",
                category: FeatCategory::General,
                name: "Inner Light",
                description: Some("Some ghorans glow with a nurturing inner light."),
                pretext: None,
                source_page: Some("p.19"),
                benefit: Some("Once per day, you can cast daylight as a spell-like ability with a caster level equal to your character level. If you have the light dependent racial trait, you can instead cast this spell without the usual effect, but you are considered to be exposed to sunlight for that day."),
                prerequisites: Some(&["PREFACT:1,TEMPLATES,IsGhoran=true", "PREVARGTEQ:PreStatScore_WIS,13"]),
            },
            // Sproutling -- uw_feats.lst:140
            UwFeatEntry {
                key: "Sproutling",
                category: FeatCategory::General,
                name: "Sproutling",
                description: Some("You are able to sprout more quickly than most, though your body is underdeveloped."),
                pretext: None,
                source_page: Some("p.19"),
                benefit: Some("Whenever you use your seed ability to grow a new body, you can choose to sprout in half the normal amount of time (1d6 days instead of 2d6 days). If you do, your new body is Small instead of Medium, your natural armor bonus from racial traits (if any) is reduced by half, and you take a -2 penalty to Strength, Dexterity, and Constitution. These modifications last 1d3 weeks, after which your body matures to its proper size and loses all penalties associated with sprouting early."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Ghoran ~ Seed,Ghoran ~ Ghorus Seed", "PREFACT:1,TEMPLATES,IsGhoran=true", "PREVARGTEQ:PreStatScore_CHA,15"]),
            },
            // Climbing Vine -- uw_feats.lst:143
            UwFeatEntry {
                key: "Climbing Vine",
                category: FeatCategory::General,
                name: "Climbing Vine",
                description: Some("You can climb like a vine."),
                pretext: None,
                source_page: Some("p.24"),
                benefit: Some("You gain a climb speed of 10 feet. If you already have a natural climb speed, your climb speed increases by 10 feet. You can take this feat multiple times, but your climb speed can never exceed your base speed."),
                prerequisites: Some(&["PRERACE:1,Vine Leshy", "PREVARGTEQ:PreStatScore_STR,13"]),
            },
            // Kudzu Grappler -- uw_feats.lst:144
            UwFeatEntry {
                key: "Kudzu Grappler",
                category: FeatCategory::General,
                name: "Kudzu Grappler",
                description: Some("You climb all over creatures you're grappling like kudzu, blocking their vision."),
                pretext: None,
                source_page: Some("p.24"),
                benefit: Some("You add an option to blind the grappled creature for 1 round to the list of actions you can take when you succeed at a check to maintain a grapple."),
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Improved Grapple,Improved Unarmed Strike,Greater Grapple", "PRERACE:1,Vine Leshy", "PRETOTALAB:6", "PREVARGTEQ:PreStatScore_DEX,13"]),
            },
            // Photosynthetic Healing -- uw_feats.lst:145
            UwFeatEntry {
                key: "Photosynthetic Healing",
                category: FeatCategory::General,
                name: "Photosynthetic Healing",
                description: Some("You can heal yourself using sunlight."),
                pretext: None,
                source_page: Some("p.24"),
                benefit: Some("You can temporarily draw energy from the sun to heal your wounds. Once per day while you are using change shape to assume a plant form, you can activate this ability as a free action to gain fast healing 1 for %1 rounds, as long as you are in an area of bright, natural sunlight. When you rest in plant form in bright, natural sunlight, you are healed of twice the normal amount of hit point damage and ability score damage.|TL"),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Vine Leshy ~ Change Shape,Leshy ~ Change Shape", "PRERACE:1,RACESUBTYPE=Leshy", "PREVARGTEQ:PreStatScore_CON,15"]),
            },
            // Reactive Reversion -- uw_feats.lst:146
            UwFeatEntry {
                key: "Reactive Reversion",
                category: FeatCategory::General,
                name: "Reactive Reversion",
                description: Some("You can reactively change back to your natural form."),
                pretext: None,
                source_page: Some("p.24"),
                benefit: Some("You can shift from your alternate form to your natural form as an immediate action using change shape. [Normal] Leshys can shift between forms as a swift action using change shape."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Vine Leshy ~ Change Shape,Leshy ~ Change Shape", "PRERACE:1,RACESUBTYPE=Leshy", "PREVARGTEQ:PreStatScore_DEX,13"]),
            },
            // Devotion against the Unnatural -- uw_feats.lst:151
            UwFeatEntry {
                key: "Devotion against the Unnatural",
                category: FeatCategory::Animal,
                name: "Devotion against the Unnatural",
                description: Some("The animal's devotion to its master allows it to stand against unnatural forces."),
                pretext: None,
                source_page: Some("p.217"),
                benefit: Some("The animal companion gains a +2 morale bonus on saving throws against spells, spell-like abilities, and supernatural abilities of aberrations, outsiders, and undead. The animal companion is immune to the unnatural aura ability that some undead creatures (such as wraiths) have."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Companion ~ Devotion", "PRECLASS:1,Companion=1"]),
            },
            // Disruptive Companion -- uw_feats.lst:152
            UwFeatEntry {
                key: "Disruptive Companion",
                category: FeatCategory::Animal,
                name: "Disruptive Companion",
                description: Some("The animal's presence harries nearby enemies, making concentration difficult."),
                pretext: None,
                source_page: Some("p.217"),
                benefit: Some("The animal companion is trained to interfere with its opponent's concentration, making even rote tasks difficult. Increase the concentration check DC of spells and spell-like abilities that opponents cast while within the animal companion's reach by +2. Additionally, opponents can't take 10 on d20 rolls or checks while within the animal companion's reach. If an opponent has an ability that allows it to always take 10 on certain skill checks while distracted (such as the skill mastery advanced talent), it gains the benefit of such abilities only if its number of skill ranks is at least 4 higher than the animal companion's Hit Dice. In addition, the animal companion counts as a fighter with a number of class levels equal to its Hit Dice for the purposes of qualifying for the Disruptive feat, as well as for any feat that lists the Disruptive feat as a prerequisite. The animal companion also adds those feats to the list of animal feats that it can choose from when gaining a new feat."),
                prerequisites: Some(&["PRECLASS:1,Companion=1"]),
            },
            // Feral Grace -- uw_feats.lst:153
            UwFeatEntry {
                key: "Feral Grace",
                category: FeatCategory::Animal,
                name: "Feral Grace",
                description: Some("The animal's grace and agility are honed for battle, making it a deadly combatant."),
                pretext: None,
                source_page: Some("p.218"),
                benefit: Some("[NOT IMPLEMENTED] Choose one of the animal companion's natural attacks. When the animal companion makes a melee attack with the chosen natural attack using its Dexterity bonus on attack rolls and its Strength bonus on damage rolls, it adds 1/4 of its Hit Dice as a bonus on damage rolls. This bonus damage doesn't increase or decrease based upon whether the natural attack is a primary or secondary natural attack. [Special] You can select this feat multiple times. Its effects don't stack. Each time you select this feat, choose a different natural attack to apply its benefit to."),
                prerequisites: Some(&["PREVAREQ:HasWeaponFinesseFeat,1", "PRECLASS:1,Companion=1", "PRETOTALAB:6"]),
            },
            // Ferocious Beast -- uw_feats.lst:154
            UwFeatEntry {
                key: "Ferocious Beast",
                category: FeatCategory::Animal,
                name: "Ferocious Beast",
                description: Some("Through training or natural ability, the animal is skilled at unnerving its enemies."),
                pretext: None,
                source_page: Some("p.218"),
                benefit: Some("The animal companion uses half its master's class level in place of its Charisma bonus on Intimidate checks to demoralize an opponent, as well as on Intimidate checks to use the Antagonize feat. In addition, the animal companion can use Intimidate to demoralize an opponent as a move action. [Special] An animal companion can't take both Ferocious Beast and Intimidating Prowess."),
                prerequisites: Some(&["!PREABILITY:1,CATEGORY=FEAT,Intimidating Prowess", "PRECLASS:1,Companion=1", "PRESKILL:1,Intimidate=1"]),
            },
            // Ferocious Feint -- uw_feats.lst:155
            UwFeatEntry {
                key: "Ferocious Feint",
                category: FeatCategory::Animal,
                name: "Ferocious Feint",
                description: Some("Through training or inborn ability, the animal is naturally skilled at feinting against its foes."),
                pretext: None,
                source_page: Some("p.218"),
                benefit: Some("The animal companion uses half its master's class level in place of its Charisma bonus on Bluff checks to feint an opponent. In addition, the animal companion can use Bluff to feint an opponent as a move action."),
                prerequisites: Some(&["PRECLASS:1,Companion=1", "PRESKILL:1,Bluff=1"]),
            },
            // Greater Tenacious Hunter -- uw_feats.lst:156
            UwFeatEntry {
                key: "Greater Tenacious Hunter",
                category: FeatCategory::Animal,
                name: "Greater Tenacious Hunter",
                description: Some("The animal is capable of savaging hidden foes when they least expect it."),
                pretext: None,
                source_page: Some("p.218"),
                benefit: Some("The animal companion can attempt a Perception check as a swift action to intentionally search for a stimulus. When using Perception to attempt to pinpoint an invisible foe, the animal companion's bonus from the Tenacious Hunter feat increases to +20. In addition, whenever the animal companion successfully uses Perception to oppose a Stealth check or to pinpoint an invisible creature's location, that foe is denied its Dexterity bonus to AC against all attacks that the animal companion makes before the start of the animal companion's next turn. [Normal] Using Perception to intentionally search for a stimulus is a move action."),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Combat Reflexes,Tenacious Hunter", "PRECLASS:1,Companion=1", "PRESKILL:2,Perception=3,Stealth=3"]),
            },
            // Improved Intercept Blow -- uw_feats.lst:157
            UwFeatEntry {
                key: "Improved Intercept Blow",
                category: FeatCategory::Animal,
                name: "Improved Intercept Blow",
                description: Some("The animal's intervention completely protects its master from harm."),
                pretext: None,
                source_page: Some("p.218"),
                benefit: Some("When the animal companion uses Intercept Blow to intercept an opponent's melee attack and succeeds, it can take all the damage from the intercepted attack, in which case its master takes no damage from the intercepted attack and doesn't suffer effects that trigger on a hit."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Intercept Blow", "PRECLASS:1,Companion=1", "PRETOTALAB:6"]),
            },
            // Intercept Blow -- uw_feats.lst:158
            UwFeatEntry {
                key: "Intercept Blow",
                category: FeatCategory::Animal,
                name: "Intercept Blow",
                description: Some("The animal willingly throws itself in harm's way to defend its master."),
                pretext: None,
                source_page: Some("p.219"),
                benefit: Some("Whenever the animal companion is adjacent to its master and its master is the target of a melee attack, the animal companion can expend an attack of opportunity as a free action, even if it isn't its turn, to attempt to intercept the attack. The animal companion makes an attack roll at its highest attack bonus with one of its natural attacks, as if it were making an attack of opportunity, with a -2 penalty for each size category the attacking creature is larger than the animal companion. If the result of this attack roll is greater than the attacking creature's attack roll result, the animal companion's master takes only half damage from the attack, and the other half of the damage is dealt to the animal companion. Effects that trigger on every successful hit (such as bleed or the trip special ability) affect both the animal companion and the master, as if both were hit by the attack; however, if it matters (such as for a touch spell that expires after a single hit), treat the animal companion as if it had been hit first. If the animal companion knows the defend trick, the animal companion can use Intercept Blow and any feat that lists it as a prerequisite to protect any ally that it is adjacent to and defending (as per the defend trick)."),
                prerequisites: Some(&["PRECLASS:1,Companion=1", "PRETOTALAB:1"]),
            },
            // Reflexive Interception -- uw_feats.lst:159
            UwFeatEntry {
                key: "Reflexive Interception",
                category: FeatCategory::Animal,
                name: "Reflexive Interception",
                description: Some("The animal can defend its master with lightning speed."),
                pretext: None,
                source_page: Some("p.219"),
                benefit: Some("Whenever the animal companion is adjacent to its master and its master is attempts a Reflex save, the animal companion can expend an attack of opportunity as a free action, even if it isn't its turn, to attempt a Reflex save to intervene. When using this ability, the animal companion loses the benefits of evasion (and improved evasion if it has it), but if the animal companion succeeds at its saving throw, its master gains the benefit of the animal companion's evasion (or improved evasion if it has it)."),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Intercept Blow", "PREABILITY:1,CATEGORY=Special Ability,Evasion", "PRECLASS:1,Companion=1", "PRETOTALAB:3"]),
            },
            // Share Feature -- uw_feats.lst:160
            UwFeatEntry {
                key: "Share Feature",
                category: FeatCategory::Animal,
                name: "Share Feature",
                description: Some("The animal expands its capabilities and learns one of its master's class features."),
                pretext: Some("Master must have the chosen class feature."),
                source_page: Some("p.219"),
                benefit: Some("[NOT IMPLEMENTED] Choose one of the following class features: bravery, camouflage, favored terrain, hide in plain sight, resist nature's lure, track, trackless step, venom immunity, and woodland stride. The animal companion gains the benefits of the selected class feature, using its Hit Dice as its level in its master's class for the purpose of this ability; the animal companion gains the benefits of the base class feature, even if the master alters the class feature via an archetype or other means, though the animal companion doesn't count as having the class feature for the purpose of fulfilling prerequisites."),
                prerequisites: Some(&["PRECLASS:1,Companion=1"]),
            },
            // Tenacious Hunter -- uw_feats.lst:161
            UwFeatEntry {
                key: "Tenacious Hunter",
                category: FeatCategory::Animal,
                name: "Tenacious Hunter",
                description: Some("The animal is capable of quickly pinpointing hidden prey."),
                pretext: None,
                source_page: Some("p.219"),
                benefit: Some("The animal companion gains a +2 bonus on Perception checks to oppose Stealth checks. When using Perception to pinpoint an invisible creature's location, this bonus increases to +10. In addition, whenever the animal companion successfully uses Perception to notice a creature using Stealth or to pinpoint an invisible creature's location, that foe is denied its Dexterity bonus to AC against the next attack the animal companion makes on or before the animal companion's next turn."),
                prerequisites: Some(&["PRECLASS:1,Companion=1", "PRESKILL:2,Perception=3,Stealth=3"]),
            },
        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_has_135_records() {
        assert_eq!(feat_tables().len(), 135);
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
