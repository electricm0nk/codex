//! Ultimate Magic (UM) feat catalog. SD28-E28 slice 1, mirroring
//! `ultimate_combat::feat_tables`'s own established shape closely, with
//! one deliberate addition.
//!
//! **Corpus coverage, honestly bounded.** `um_feats.lst` has 147
//! top-level `CATEGORY:FEAT` records (re-derived: a naive
//! `grep -c '^CATEGORY:FEAT' um_feats.lst` returns 0 -- the same
//! not-line-anchored trap `decisions.md §46` documented for UC; the real
//! count is `grep -c $'\tCATEGORY:FEAT\t' um_feats.lst`, confirmed 147,
//! consistent with the case-insensitive whole-file figure of 163 once
//! UM's 16 `CATEGORY=FEAT|<Name>.MOD` modifier rows are subtracted).
//! **Zero cross-book collisions** -- re-derived against every other
//! book's real runtime feat key set (a scratch `#[test]` dump of
//! `feats_all::all_feat_tables()`, `decisions.md §44`'s lesson applied
//! from the start), and zero intra-book duplicate keys. UM's feats are
//! genuinely new content.
//!
//! **UM's own `.MOD` rows carry no `DESC:`/`BENEFIT:` at all** (confirmed
//! directly, not assumed from `decisions.md §47`'s nine-book sweep) --
//! the `Revelation Strike`-shaped "real text hidden on an invisible
//! `.MOD` row" defect does not recur in this book's feats.
//!
//! **Three records are genuine auto-grant wrappers, excluded rather than
//! shipped as stubs**, the same disposition `decisions.md §46` gave UC's
//! `Gundarme Bonus Feat`: `Skill Focus (Knowledge [Arcana])`,
//! `Skill Focus (Intimidate)`, `Skill Focus (Swim)` (`um_feats.lst:189,
//! 195, 201`) are each `VISIBLE:DISPLAY` with an
//! `ABILITY:FEAT|AUTOMATIC|Skill Focus (...)` grant mechanism, auto-
//! granted from an internal Dragon/Saurian/Shark Shaman class bonus-feat
//! pool -- not standalone, player-chosen prose content.
//!
//! **Four records carry a real, distinct game mechanic but no `DESC:`/
//! `BENEFIT:` prose in the corpus at all** -- `Extra Cantrips or Orisons`,
//! `Extra Evolution`, `Extra Summons`, `Transfer Feat to Familiar`. Each
//! is genuinely selectable (`STACK:YES`/`MULT:YES`/`CHOOSE:`) and carries
//! its own real `BONUS:`/`DEFINE:` tokens -- unlike the three exclusions
//! above, these are not auto-grant wrappers, and unlike UC's textless
//! exclusions, no sibling record anywhere in the corpus carries their
//! missing prose to recover. **Kept, not excluded** -- this is why
//! `UmFeatEntry` carries an `effect` field CRB's own `FeatTableEntry`
//! already established (`crb/feats.rs`'s 104-of-185 `BONUS:`-only
//! records, e.g. the 8 "Heighten Spell +N" tiers) rather than UC's
//! two-field (`description`+`benefit`)-only shape: dropping these four
//! would be excluding real content on the false premise that a book's
//! shape from one slice ago must repeat exactly, and fabricating prose
//! for them would violate the no-stub-mvp-doctrine in the other
//! direction. The corpus's own real content for these four *is* the
//! `BONUS:`/`DEFINE:` mechanic, not missing prose.
//!
//! **15 `Masterpiece (<Name>)` records carry `DESC:` but no `BENEFIT:`,
//! and this is genuinely complete, not a stub** -- each is a real,
//! individually-named Bard masterpiece-performance feat whose entire
//! rules content in the corpus is "You learn the masterpiece <Name>."
//! (e.g. `um_feats.lst:206`); the masterpiece's own mechanical effect is
//! defined once, centrally, under the Bard class's own masterpiece
//! system, not repeated per-feat. Unlike `Revelation Strike`, there is no
//! missing `BENEFIT:` to recover -- the `DESC:` token already is the
//! feat's complete, correct text.
//!
//! **2 records carry `BENEFIT:` but no `DESC:`** (`Greater Wild
//! Empathy`, `Versatile Channeler`) -- ordinary, matching the same
//! "flavour text absent, mechanical text present" shape this program has
//! treated as complete since CRB's own catalog.
//!
//! **The exact split, machine-checked by this file's own tests: 123
//! records carry both `DESC:` and `BENEFIT:`, 15 carry `DESC:` only (the
//! Masterpiece feats), 2 carry `BENEFIT:` only, 4 carry neither but do
//! carry `effect` (123+15+2+4 = 144, the full catalog).**
//!
//! **Final catalog: 144 real, distinct records** (147 raw − 3 genuine
//! auto-grant exclusions).
//!
//! **No `KEY:` token on any record**, so `key == name` for every entry.
//!
//! **`category` is UM's own enum, not the shared `crb::feats::FeatCategory`.**
//! UM introduces `Masterpiece` (Bard performance feats) and `Discovery`
//! (Wizard bonus-discovery-as-feat records, e.g. `Discovery (Fast
//! Study)`) facets with no shared equivalent; `Critical`, `General`,
//! `Combat`, `ItemCreation`, `Metamagic`, `Teamwork` map onto the shared
//! vocabulary the same way UC's own `as_shared` does. Dotted sub-facets
//! (`General.SpellSpecialization`, `General.UndeadLordBonus`,
//! `Discovery.WizardBonus.*`) fold to their top-level segment, matching
//! every other book's own folding convention.
//!
//! **`prerequisites` carries every real `PRE`-family token verbatim**,
//! gathered directly at ingest, `None` when the corpus row has none.
//!
//! Every field below is copied verbatim from the real corpus row (source:
//! `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/
//! ultimate_magic/um_feats.lst`), generated programmatically by a one-off
//! extraction script, not hand-transcribed.

use super::super::crb::feats::FeatCategory as SharedFeatCategory;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatCategory {
    General,
    Combat,
    ItemCreation,
    Metamagic,
    Teamwork,
    Critical,
    Masterpiece,
    Discovery,
}

impl FeatCategory {
    /// Maps onto the shared `crb::feats::FeatCategory` vocabulary where
    /// one exists -- `None` for UM's own `Critical`/`Masterpiece`/
    /// `Discovery` facets, mirroring UC's own `as_shared` rule for its
    /// own book-specific facets without checking.
    pub fn as_shared(self) -> Option<SharedFeatCategory> {
        match self {
            FeatCategory::General => Some(SharedFeatCategory::General),
            FeatCategory::Combat => Some(SharedFeatCategory::Combat),
            FeatCategory::ItemCreation => Some(SharedFeatCategory::ItemCreation),
            FeatCategory::Metamagic => Some(SharedFeatCategory::Metamagic),
            FeatCategory::Teamwork => Some(SharedFeatCategory::Teamwork),
            FeatCategory::Critical | FeatCategory::Masterpiece | FeatCategory::Discovery => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UmFeatEntry {
    /// The record's corpus identity. No record in this catalog carries a
    /// distinct `KEY:` token, so `key == name` for every entry.
    pub key: &'static str,
    pub category: FeatCategory,
    pub name: &'static str,
    /// The corpus `DESC:` token, verbatim. `None` when the record has no
    /// `DESC:` token.
    pub description: Option<&'static str>,
    /// The corpus `PRETEXT:` token, verbatim display prerequisite prose --
    /// `None` when the row carries no `PRETEXT:`.
    pub pretext: Option<&'static str>,
    pub source_page: Option<&'static str>,
    /// The corpus `BENEFIT:` token, verbatim -- the actual mechanical
    /// text, when the record carries prose at all. `None` when the
    /// record has no `BENEFIT:` token.
    pub benefit: Option<&'static str>,
    /// Every `BONUS:`/`DEFINE:` token the corpus record carries,
    /// verbatim, in source order -- carries this book's own four
    /// no-prose-but-real-mechanic records (this module doc comment's
    /// "Four records" paragraph) without excluding or fabricating text
    /// for them. `None` when the row has no `BONUS:` token.
    pub effect: Option<&'static [&'static str]>,
    /// Every top-level `PRE`-family token the corpus record carries,
    /// verbatim and unparsed, in source order. `None` when the row has no
    /// `PRE`-family token.
    pub prerequisites: Option<&'static [&'static str]>,
}

/// Full UM feat catalog: 144 real, distinct corpus records, in source
/// order. Built once and cached for the process lifetime.
pub fn feat_tables() -> &'static [UmFeatEntry] {
    static TABLE: std::sync::OnceLock<Vec<UmFeatEntry>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        vec![
            // Abundant Revelations -- um_feats.lst:15
            UmFeatEntry {
                key: "Abundant Revelations",
                category: FeatCategory::General,
                name: "Abundant Revelations",
                description: Some("You can plumb the depths of your mystery to use your revelations more often."),
                pretext: None,
                source_page: Some("p.142"),
                benefit: Some("Choose one of your revelations that has a number of uses per day. You gain 1 additional use per day of that revelation."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Oracle's Mystery"]),
            },
            // Accursed Critical -- um_feats.lst:16
            UmFeatEntry {
                key: "Accursed Critical",
                category: FeatCategory::Critical,
                name: "Accursed Critical",
                description: Some("Your spells carry an embedded curse that manifests when they strike true."),
                pretext: None,
                source_page: Some("p.142"),
                benefit: Some("When you confirm a critical hit with a spell or spell-like ability, you may cast bestow curse or major curse on that target as an immediate action. This works even with ranged spells. You must have bestow curse or major curse prepared or otherwise available to cast, and using this ability casts the corresponding spell."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Critical Focus", "PREMULT:1,[PRECLASS:1,SPELLCASTER=9],[PREVARGTEQ:CasterLevel_Highest,9]", "PRESPELL:1,Bestow Curse,Major Curse"]),
            },
            // Accursed Hex -- um_feats.lst:17
            UmFeatEntry {
                key: "Accursed Hex",
                category: FeatCategory::General,
                name: "Accursed Hex",
                description: Some("You can make a second attempt at failed hexes."),
                pretext: None,
                source_page: Some("p.143"),
                benefit: Some("When you target a creature with a hex that cannot target the same creature more than once per day, and that creature succeeds at its saving throw against the hex's effect, you can target the creature with the same hex a second time before the end of your next turn. If the second attempt fails, you can make no further attempts to target that creature with the same hex for 1 day."),
                effect: None,
                prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=Special Ability,TYPE.WitchHex],[PREVARGTEQ:WitchMinorHexQualify,1],[PREVARGTEQ:WitchHexAbilityLVL,1]"]),
            },
            // Advanced Ranger Trap -- um_feats.lst:18
            UmFeatEntry {
                key: "Advanced Ranger Trap",
                category: FeatCategory::General,
                name: "Advanced Ranger Trap",
                description: Some("Your ranger traps are especially difficult to notice and avoid."),
                pretext: None,
                source_page: Some("p.143"),
                benefit: Some("Add +1 to the Difficulty Class on all Perception and Disable Device skill checks to find or disable the traps you make with your trap class feature. Add a +1 to the Difficulty Class on all saving throws against the effects of the trap you make with your trap class feature."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Trapper ~ Trap", "PRECLASS:1,Ranger=5"]),
            },
            // Antagonize -- um_feats.lst:19
            UmFeatEntry {
                key: "Antagonize",
                category: FeatCategory::General,
                name: "Antagonize",
                description: Some("Whether with biting remarks or hurtful words, you are adept at making creatures angry with you."),
                pretext: None,
                source_page: Some("p.143"),
                benefit: Some("You can make Diplomacy and Intimidate checks to make creatures respond to you with hostility. No matter which skill you use, antagonizing a creature takes a standard action that does not provoke attacks of opportunity, and has a DC equal to 10 + the target's Hit Dice + the target's Wisdom modifier. The benefits you gain for this check depend on the skill you use. This is a mind-affecting effect."),
                effect: None,
                prerequisites: None,
            },
            // Blighted Critical -- um_feats.lst:20
            UmFeatEntry {
                key: "Blighted Critical",
                category: FeatCategory::Critical,
                name: "Blighted Critical",
                description: Some("With a critical hit from a spell or spell-like ability, you give the target a minor spellblight."),
                pretext: None,
                source_page: Some("p.143"),
                benefit: Some("Whenever you confirm a critical hit with a touch spell, ranged touch spell, or spell-like ability against an opponent, the victim gains a random minor spellblight."),
                effect: None,
                prerequisites: Some(&["PREMULT:1,[PRECLASS:1,SPELLCASTER=5],[PREVARGTEQ:CasterLevel_Highest,5]"]),
            },
            // Blighted Critical Mastery -- um_feats.lst:21
            UmFeatEntry {
                key: "Blighted Critical Mastery",
                category: FeatCategory::General,
                name: "Blighted Critical Mastery",
                description: Some("You control the type of spellblight your critical hits give your opponent."),
                pretext: None,
                source_page: Some("p.143"),
                benefit: Some("Whenever you apply a spellblight by way of the Blighted Critical or Greater Blighted critical feat, you can choose the spellblight you apply rather than determining it randomly."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Blighted Critical", "PREMULT:1,[PRECLASS:1,SPELLCASTER=9],[PREVARGTEQ:CasterLevel_Highest,9]"]),
            },
            // Burning Spell -- um_feats.lst:22
            UmFeatEntry {
                key: "Burning Spell",
                category: FeatCategory::Metamagic,
                name: "Burning Spell",
                description: Some("You cause creatures to take extra damage when you affect them with a spell that has the acid or fire descriptor."),
                pretext: None,
                source_page: Some("p.143"),
                benefit: Some("The acid or fire effects of the affected spell adhere to the creature, causing more damage the next round. When a creature takes acid or fire damage from the affected spell, that creature takes damage equal to 2 x the spell's actual level at the start of its next turn. The damage is acid or fire, as determined by the spell's descriptor. If a burning spell has both the fire and acid descriptor, the caster chooses what kind of damage is dealt by the burning spell effect. A burning spell uses up a slot two levels higher than the spell's actual level."),
                effect: None,
                prerequisites: None,
            },
            // Channeled Shield Wall -- um_feats.lst:23
            UmFeatEntry {
                key: "Channeled Shield Wall",
                category: FeatCategory::General,
                name: "Channeled Shield Wall",
                description: Some("You draw upon your channel energy to enhance the protective ability of your shield and those of allies while they are adjacent to you."),
                pretext: None,
                source_page: Some("p.143"),
                benefit: Some("As a swift action, you can spend a use of your channel energy to grant yourself a +2 deflection bonus while using a shield. This bonus lasts 1 minute per cleric level or effective cleric level. While you benefit from this bonus, allies with shields also gain a +2 deflection bonus while they are adjacent to you."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Energy", "PREMULT:1,[PREVARGTEQ:ClericChannelEnergyLVL,5],[PREVARGTEQ:PaladinChannelLVL,5],[PREVARGTEQ:OracleChannelLVL,5]", "PREMULT:1,[PREVARGTEQ:OracleChannelDieSize,6],[PREVARGTEQ:ClericChannelPositiveEnergyDieSize,6],[PREVARGTEQ:ClericChannelNegativeEnergyDieSize,6],[PREVARGTEQ:PaladinChannelDieSize,6],[PREVARGTEQ:ClassChannelPositiveEnergyDieSize,6],[PREVARGTEQ:ClassChannelNegativeEnergyDieSize,6]", "PREPROFWITHSHIELD:1,TYPE.Light,TYPE.Heavy"]),
            },
            // Concussive Spell -- um_feats.lst:24
            UmFeatEntry {
                key: "Concussive Spell",
                category: FeatCategory::Metamagic,
                name: "Concussive Spell",
                description: Some("You cause creatures to be disoriented when you affect them with a spell that has the sonic descriptor."),
                pretext: None,
                source_page: Some("p.143"),
                benefit: Some("With sonic damage comes a concussive wave of energy that rattles creatures affected by the spell. A concussive spell causes creatures that take damage from a spell that has the sonic descriptor to take a -2 penalty on attack rolls, saving throws, skill checks, and ability checks for a number of rounds equal to the actual spell level of the spell. A concussive spell only affects spells with the sonic descriptor. A concussive spell uses up a spell slot two levels higher than the spell's actual level."),
                effect: None,
                prerequisites: None,
            },
            // Create Reliquary Arms and Shields -- um_feats.lst:25
            UmFeatEntry {
                key: "Create Reliquary Arms and Shields",
                category: FeatCategory::ItemCreation,
                name: "Create Reliquary Arms and Shields",
                description: Some("Your magical creations are infused with divine power."),
                pretext: None,
                source_page: Some("p.148"),
                benefit: Some("When you craft a magic weapon, magic armor, or magic shield, you may add one casting of consecrate or desecrate as part of the item crafting process. This increases the item's Price by 250 gp.  The item becomes a reliquary and can be used as a holy (or unholy) symbol divine focus of your deity. If you cast consecrate or desecrate, your reliquary counts as a permanent fixture for that spell while it remains in the spell's area."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Craft Magic Arms and Armor", "PRESPELL:1,Consecrate,Desecrate"]),
            },
            // Create Sanguine Elixir -- um_feats.lst:26
            UmFeatEntry {
                key: "Create Sanguine Elixir",
                category: FeatCategory::ItemCreation,
                name: "Create Sanguine Elixir",
                description: Some("You can condense a fraction of your bloodline's power into a powerful elixir."),
                pretext: None,
                source_page: Some("p.148"),
                benefit: Some("Once per day, when you clear your mind to regain spell slots, you can create a sanguine elixir. When you do, pick one of your bloodline powers. You transfer that power into a small potion that any creature can drink to temporarily gain the benefit of your bloodline power. Creating a sanguine elixir takes 1 hour, and requires special oils and distillates worth 100 gp, and when you make the sanguine elixir, you lose access to the bloodline power until the next time you clear your mind to regain spell slots. Sanguine elixirs are extremely unstable. They lose their potency 1 day after they are created."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Brew Potion", "PRECLASS:1,Sorcerer=3", "PRESKILL:1,Craft (Alchemy)=12", "PRESTAT:1,CHA=15"]),
            },
            // Defending Eidolon -- um_feats.lst:27
            UmFeatEntry {
                key: "Defending Eidolon",
                category: FeatCategory::General,
                name: "Defending Eidolon",
                description: Some("You have trained your eidolon to protect you."),
                pretext: None,
                source_page: Some("p.148"),
                benefit: Some("Whenever you are adjacent to your eidolon, you can choose for the eidolon to take a -1 penalty on melee attack rolls and combat maneuver checks to gain a +1 dodge bonus to your Armor Class. When your eidolon's base attack bonus reaches +5, and for every +5 thereafter, the penalty increases by -1 and the dodge bonus increases by +1. You must choose to use this feat when your eidolon is making an attack or full-attack action with melee or natural weapons, and its effect lasts until your eidolon's next turn, or until you are no longer adjacent to the eidolon, whichever occurs first."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Shield Ally"]),
            },
            // Deny Death -- um_feats.lst:28
            UmFeatEntry {
                key: "Deny Death",
                category: FeatCategory::General,
                name: "Deny Death",
                description: Some("Your ki is so strong that it can deny death."),
                pretext: None,
                source_page: Some("p.148"),
                benefit: Some("As long as you have 1 ki point in your ki pool, when you fail your Constitution check to stabilize, you do not lose 1 hit point. If you succeed at the check, you can spend 1 ki point to heal 1d6 hit points. If you roll a natural 20 on the check to stabilize, you can spend 1 ki point to heal 2d6 hit points of damage instead."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Ki Pool", "PREABILITY:1,CATEGORY=FEAT,Endurance"]),
            },
            // Detect Expertise -- um_feats.lst:29
            UmFeatEntry {
                key: "Detect Expertise",
                category: FeatCategory::General,
                name: "Detect Expertise",
                description: Some("You can detect the mystic specialty of a foe."),
                pretext: None,
                source_page: Some("p.148"),
                benefit: Some("When you use any of the spells listed in this feat's prerequisites to detect a creature's alignment or its magic, you have a chance of detecting what spellcasting expertise it has. After you observe a creature with the detect spell for 3 rounds, it must make a Will save (DC %1 plus half your caster level). If it fails the saving throw, you learn what bloodlines, domains, hexes, schools, or mysteries (if any) the creature possesses. If the creature makes its save, it is immune to the effects of this feat for 24 hours.|10+INT"),
                effect: None,
                prerequisites: Some(&["PRESPELL:1,Detect Chaos,Detect Evil,Detect Good,Detect Law,Detect Magic", "PRESTAT:1,INT=13"]),
            },
            // Die for Your Master -- um_feats.lst:30
            UmFeatEntry {
                key: "Die for Your Master",
                category: FeatCategory::General,
                name: "Die for Your Master",
                description: Some("Your tumor familiar goes to any length to save your life."),
                pretext: None,
                source_page: Some("p.148"),
                benefit: Some("If your tumor familiar is attached, and you would be reduced to 0 or fewer hit points by damage in combat (from a weapon or other blow, not a spell or special ability), the familiar throws itself in the way of the attack as an immediate action. If it makes a Reflex saving throw (DC = damage dealt), it takes all the damage from the attack. If it fails, it takes half damage and you take half damage. The familiar must be aware of the attack and able to react to it in order to use this ability, and it can only do this once per day-if it is denied its Dexterity bonus to AC, it can't use this ability. Since this effect would not normally allow the familiar to make a Reflex save for half damage, its improved evasion ability does not apply on this saving throw."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Discovery ~ Tumor Familiar"]),
            },
            // Divine Interference -- um_feats.lst:31
            UmFeatEntry {
                key: "Divine Interference",
                category: FeatCategory::General,
                name: "Divine Interference",
                description: Some("You can convert a spell to interfere with an enemy's attack."),
                pretext: None,
                source_page: Some("p.149"),
                benefit: Some("As an immediate action, when an enemy within 30 feet hits an ally with an attack, you can sacrifice a prepared divine spell or (if you are a spontaneous caster) an unused spell slot and make the enemy reroll the attack roll. The second attack roll takes a penalty equal to the level of the spell you sacrifice. You must sacrifice a spell of 1st-level or higher to use this ability. Whether or not the second attack is successful, you cannot use this effect on the same creature again for 1 day."),
                effect: None,
                prerequisites: Some(&["PREMULT:1,[PRECLASS:1,SPELLCASTER.Divine=10],[PREVARGTEQ:Caster_Level_Highest__Divine,10]"]),
            },
            // Dragonbane Aura -- um_feats.lst:32
            UmFeatEntry {
                key: "Dragonbane Aura",
                category: FeatCategory::General,
                name: "Dragonbane Aura",
                description: Some("Those within your dragonbane aura gain the same protection that you do."),
                pretext: None,
                source_page: Some("p.149"),
                benefit: Some("When fighting dragons, your aura of courage expands to a 20-foot-radius emanation, and allies in the aura gain a morale bonus on saving throws against dragon breath equal to your aura of courage's bonus against fear effects."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Paladin ~ Aura of Courage", "PREMULT:1,[PRECLASS:1,SPELLCASTER=8],[PREVARGTEQ:CasterLevel_Highest,8]"]),
            },
            // Echoing Spell -- um_feats.lst:33
            UmFeatEntry {
                key: "Echoing Spell",
                category: FeatCategory::Metamagic,
                name: "Echoing Spell",
                description: Some("You have learned how to release most, but not all, of a spell's potential when you cast it."),
                pretext: None,
                source_page: Some("p.149"),
                benefit: Some("When you cast an echoing spell, it does not disappear entirely from memory, and you can cast it one additional time during that day. No effect that allows you to reprepare or recast a spell can affect the echoed spell. If you prepare spells, this second casting does not require you to prepare it in another spell slot. If you spontaneously cast spells, this second casting does not expend another available spell slot."),
                effect: None,
                prerequisites: None,
            },
            // Eldritch Heritage -- um_feats.lst:37
            UmFeatEntry {
                key: "Eldritch Heritage",
                category: FeatCategory::General,
                name: "Eldritch Heritage",
                description: Some("You are descended from a long line of sorcerers, and some portion of their power flows in your veins."),
                pretext: None,
                source_page: Some("p.149"),
                benefit: Some("Select one sorcerer bloodline. You must have Skill focus in the class skill that bloodline grants to a sorcerer at 1st level (for example, Heal for the celestial bloodline). This bloodline cannot be a bloodline you already have. You gain the first-level bloodline power for the selected bloodline. For purposes of using that power, treat your sorcerer level as equal to your character level -2, even if you have levels in sorcerer. You do not gain any of the other bloodline abilities."),
                effect: Some(&["BONUS:VAR|EldritchHeritageBloodlineLVL|TL-2", "BONUS:VAR|Sorcerer_Spells_StatBonus|CHA|TYPE=Base"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Skill Focus", "PREPCLEVEL:MIN=3", "PREVARGTEQ:CHASCORE,EldritchHeritageCharismaPrerequisite", "PREVARLT:count(\"ABILITIES\",\"CATEGORY=FEAT\",\"KEY=Eldritch Heritage\"),1"]),
            },
            // Ensemble -- um_feats.lst:38
            UmFeatEntry {
                key: "Ensemble",
                category: FeatCategory::Teamwork,
                name: "Ensemble",
                description: Some("You can create an ensemble of skilled and amateur performers to aid you in your performance."),
                pretext: None,
                source_page: Some("p.149"),
                benefit: Some("When you are performing, allies within 20 feet who also have this feat can aid you with your Perform checks (including those made as part of bardic performance) as if they were aiding another as an immediate action. The allies make their aid another rolls before you make your check. No more than four allies can grant you a bonus with aid another. Allies aiding you do not need to use the same category of the Perform skill that you are using in order to aid you."),
                effect: None,
                prerequisites: Some(&["PRESKILL:1,TYPE.Perform=5"]),
            },
            // Evolved Familiar -- um_feats.lst:39
            UmFeatEntry {
                key: "Evolved Familiar",
                category: FeatCategory::General,
                name: "Evolved Familiar",
                description: Some("Your familiar is different from others of its kind."),
                pretext: None,
                source_page: Some("p.149"),
                benefit: Some("Select an evolution from the list of 1-point evolutions available to a summoner's eidolon. Your familiar has this evolution. The familiar must conform to any limitations of the evolution. For instance, no familiars can benefit from the mount evolution and only familiars with wings can take the wing buffet evolution. If you gain a new familiar, your old familiar loses all evolutions, and you can select a new 1-point evolution for the new familiar."),
                effect: Some(&["BONUS:VAR|FamiliarEP|1"]),
                prerequisites: Some(&["PRESTAT:2,INT=13,CHA=13", "PREVARGTEQ:FamiliarLVL,1"]),
            },
            // Exploit Lore -- um_feats.lst:40
            UmFeatEntry {
                key: "Exploit Lore",
                category: FeatCategory::General,
                name: "Exploit Lore",
                description: Some("You can use your knowledge of a creature's weaknesses to deliver a driving and relentless assault against it."),
                pretext: None,
                source_page: Some("p.149"),
                benefit: Some("Once per day, when you successfully identify all abilities and weaknesses of a creature using the appropriate Knowledge check, you gain a +2 bonus on attack and damage rolls against that creature for 1 minute. If you identify the abilities and weaknesses of numerous creatures, you must pick one creature to be the target of this effect."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Monster Lore", "PRETOTALAB:11"]),
            },
            // Extra Arcana -- um_feats.lst:41
            UmFeatEntry {
                key: "Extra Arcana",
                category: FeatCategory::General,
                name: "Extra Arcana",
                description: Some("You have unlocked the secret of a new magus arcana."),
                pretext: None,
                source_page: Some("p.149"),
                benefit: Some("You gain one additional magus arcana. You must meet all the prerequisites for this magus arcana. Special - You can gain this feat multiple times."),
                effect: Some(&["BONUS:ABILITYPOOL|Magus Arcana|1"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Magus Arcana"]),
            },
            // Extra Arcane Pool -- um_feats.lst:42
            UmFeatEntry {
                key: "Extra Arcane Pool",
                category: FeatCategory::General,
                name: "Extra Arcane Pool",
                description: Some("You have learned how to draw more power from your arcane pool."),
                pretext: None,
                source_page: Some("p.150"),
                benefit: Some("Your arcane pool increases by 2. Special - you can gain this feat multiple times."),
                effect: Some(&["BONUS:VAR|MagusArcanePool|2"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Magus Arcane Pool"]),
            },
            // Extended Bane -- um_feats.lst:43
            UmFeatEntry {
                key: "Extended Bane",
                category: FeatCategory::General,
                name: "Extended Bane",
                description: Some("Your dedication knows no limit. Your wrath dies hard."),
                pretext: None,
                source_page: Some("p.150"),
                benefit: Some("Add your Wisdom bonus (+%1) to the number of rounds per day that you can use your bane ability.|MAX(0,WIS)"),
                effect: Some(&["BONUS:VAR|InquisitorBanePool|MAX(0,WIS)"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Bane"]),
            },
            // Extra Cantrips or Orisons -- um_feats.lst:48
            UmFeatEntry {
                key: "Extra Cantrips or Orisons",
                category: FeatCategory::General,
                name: "Extra Cantrips or Orisons",
                description: None,
                pretext: None,
                source_page: Some("p.150"),
                benefit: None,
                effect: Some(&["BONUS:SPELLKNOWN|CLASS=%LIST;LEVEL=0|2"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.CantripsOrisons"]),
            },
            // Extra Evolution -- um_feats.lst:50
            UmFeatEntry {
                key: "Extra Evolution",
                category: FeatCategory::General,
                name: "Extra Evolution",
                description: None,
                pretext: None,
                source_page: Some("p.150"),
                benefit: None,
                effect: Some(&["BONUS:VAR|Feat_Extra_Evolution_Count|1", "BONUS:VAR|EidolonEvolution|1"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Eidolon", "PREVARGTEQ:TL/5,Feat_Extra_Evolution_Count"]),
            },
            // Extra Ranger Trap -- um_feats.lst:51
            UmFeatEntry {
                key: "Extra Ranger Trap",
                category: FeatCategory::General,
                name: "Extra Ranger Trap",
                description: Some("You can set ranger traps two additional times per day."),
                pretext: None,
                source_page: Some("p.150"),
                benefit: Some("You can set ranger traps two additional times per day."),
                effect: Some(&["BONUS:VAR|TrapTimes|2"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Trapper ~ Trap"]),
            },
            // Extra Summons -- um_feats.lst:52
            UmFeatEntry {
                key: "Extra Summons",
                category: FeatCategory::General,
                name: "Extra Summons",
                description: None,
                pretext: Some("Ability to cast summon monster as a spelllike ability, summoner 1st."),
                source_page: Some("p.150"),
                benefit: None,
                effect: Some(&["BONUS:VAR|Feat_Extra_Summons_Taken|1", "BONUS:VAR|SummonMonsterTimes|1"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Summon Monster SLA", "PRECLASS:1,Summoner=1", "PRETEXT:Ability to cast summon monster as a spelllike ability, summoner 1st.", "PREVARLTEQ:Feat_Extra_Summons_Taken,Feat_Extra_Summons_Allowed"]),
            },
            // Eyes of Judgment -- um_feats.lst:53
            UmFeatEntry {
                key: "Eyes of Judgment",
                category: FeatCategory::General,
                name: "Eyes of Judgment",
                description: Some("The true motives of creatures cannot escape your discerning gaze."),
                pretext: None,
                source_page: Some("p.150"),
                benefit: Some("When using your detect alignment class feature, you may spend 3 rounds studying a creature within 60 feet. You cannot take any other actions while doing this. After that time has passed, you learn the alignment of the creature."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Detect Alignment", "PREMULT:1,[PRECLASS:1,SPELLCASTER=6],[PREVARGTEQ:CasterLevel_Highest,6]"]),
            },
            // Fast Empathy -- um_feats.lst:54
            UmFeatEntry {
                key: "Fast Empathy",
                category: FeatCategory::General,
                name: "Fast Empathy",
                description: Some("Your empathic attunement to nature connects you swiftly with bestial minds."),
                pretext: None,
                source_page: Some("p.150"),
                benefit: Some("Using wild empathy is a standard action for you."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Wild Empathy", "PRESKILL:1,Handle Animal=5"]),
            },
            // Favored Judgment -- um_feats.lst:55
            UmFeatEntry {
                key: "Favored Judgment",
                category: FeatCategory::General,
                name: "Favored Judgment",
                description: Some("Your judgment against a particular type of creature is particularly harsh."),
                pretext: None,
                source_page: Some("p.150"),
                benefit: Some("Any sacred or profane bonus you gain from a judgment is 1 higher for attacks you make against or take from creatures that match the selected favored enemy."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.InquisitorJudgment", "PRESTAT:1,WIS=13"]),
            },
            // Fearless Aura -- um_feats.lst:56
            UmFeatEntry {
                key: "Fearless Aura",
                category: FeatCategory::General,
                name: "Fearless Aura",
                description: Some("Your aura of courage becomes more potent, as your steadfast resolve is also manifested by your allies."),
                pretext: None,
                source_page: Some("p.150"),
                benefit: Some("Your aura of courage expands to a 20-foot radius emanation. Allies within the aura are immune to fear effects."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Paladin ~ Aura of Courage", "PREMULT:1,[PRECLASS:1,SPELLCASTER=8],[PREVARGTEQ:CasterLevel_Highest,8]"]),
            },
            // Fire Music -- um_feats.lst:58
            UmFeatEntry {
                key: "Fire Music",
                category: FeatCategory::General,
                name: "Fire Music",
                description: Some("Your ability to command fire and bardic music has created a strange blend of both magics."),
                pretext: None,
                source_page: Some("p.151"),
                benefit: Some("When you cast a bard spell that deals damage, you may replace the spell's normal damage with fire damage or split the spell's damage so that half of it is the normal damage type and half is fire damage."),
                effect: None,
                prerequisites: Some(&["PRESKILL:1,Spellcraft=5"]),
            },
            // Flaring Spell -- um_feats.lst:59
            UmFeatEntry {
                key: "Flaring Spell",
                category: FeatCategory::Metamagic,
                name: "Flaring Spell",
                description: Some("You dazzle creatures when you affect them with a spell that has the fire, light, or electricity descriptor."),
                pretext: None,
                source_page: Some("p.151"),
                benefit: Some("The electricity, fire, or light effects of the affected spell create a flaring that dazzles creatures that take damage from the spell. A flare spell causes a creature that takes fire or electricity damage from the affected spell to become dazzled for a number of rounds equal to the actual level of the spell. A flaring spell only affects spells with a fire, light, or electricity descriptor. A flaring spell uses up a spell slot one level higher than the spell's actual level."),
                effect: None,
                prerequisites: None,
            },
            // Focused Eidolon -- um_feats.lst:60
            UmFeatEntry {
                key: "Focused Eidolon",
                category: FeatCategory::General,
                name: "Focused Eidolon",
                description: Some("Your bond with your eidolon helps to focus your concentration."),
                pretext: None,
                source_page: Some("p.151"),
                benefit: Some("While you are adjacent to your eidolon, you receive a +4 bonus on concentration checks."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Shield Ally"]),
            },
            // Gliding Steps -- um_feats.lst:61
            UmFeatEntry {
                key: "Gliding Steps",
                category: FeatCategory::General,
                name: "Gliding Steps",
                description: Some("You skate across the surface of the earth as if gliding on ice."),
                pretext: None,
                source_page: Some("p.151"),
                benefit: Some("If you have at least one ki in your ki pool, when you move you do not provoke attacks of opportunity when leaving the first square of that movement. You can spend 1 ki point to avoid provoking attacks of opportunity during that entire move."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Ki Pool", "PREABILITY:3,CATEGORY=FEAT,Dodge,Mobility,Nimble Moves"]),
            },
            // Grant Initiative -- um_feats.lst:62
            UmFeatEntry {
                key: "Grant Initiative",
                category: FeatCategory::General,
                name: "Grant Initiative",
                description: Some("Not only are you a master at taking the initiative, but you can also grant it to someone else."),
                pretext: None,
                source_page: Some("p.151"),
                benefit: Some("At the start of each encounter, you can either choose to keep the bonus granted to you by your Wisdom modifier on initiative checks or choose to give that bonus to one of your allies that you can see. You must make this choice before you or the ally you are granting the bonus to makes the initiative check."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Cunning Initiative"]),
            },
            // Greater Blighted Critical -- um_feats.lst:63
            UmFeatEntry {
                key: "Greater Blighted Critical",
                category: FeatCategory::Critical,
                name: "Greater Blighted Critical",
                description: Some("Your critical hit from a spell or spell-like ability afflicts the target with a major spellblight."),
                pretext: None,
                source_page: Some("p.151"),
                benefit: Some("Whenever you confirm a critical hit with a touch spell, ranged touch spell, or spell-like ability against an opponent, the victim gains a random major spellblight (see page 96 of Ultimate Magic)."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Blighted Critical", "PREMULT:1,[PRECLASS:1,SPELLCASTER=12],[PREVARGTEQ:CasterLevel_Highest,12]"]),
            },
            // Greater Eldritch Heritage -- um_feats.lst:65
            UmFeatEntry {
                key: "Greater Eldritch Heritage",
                category: FeatCategory::General,
                name: "Greater Eldritch Heritage",
                description: Some("Your discovered bloodline power reaches its zenith."),
                pretext: None,
                source_page: Some("p.152"),
                benefit: Some("You gain an additional power from the bloodline you selected with the Eldritch Heritage feat. You gain a 15th-level (or lower) sorcerer bloodline power that you do not already have. For purposes of using that power, treat your character level as your sorcerer level for all your sorcerer bloodline powers granted by this feat, Eldritch Heritage, and Improved Eldritch Heritage."),
                effect: Some(&["BONUS:ABILITYPOOL|Eldritch Heritage Selection|1", "BONUS:VAR|EldritchHeritageBloodlineLVL|2"]),
                prerequisites: Some(&["PREABILITY:2,CATEGORY=FEAT,Eldritch Heritage,Improved Eldritch Heritage", "PRELEVEL:MIN=17", "PREVARGTEQ:CHASCORE,GreaterEldritchHeritageCharismaPrerequisite"]),
            },
            // Greater Mercy -- um_feats.lst:66
            UmFeatEntry {
                key: "Greater Mercy",
                category: FeatCategory::General,
                name: "Greater Mercy",
                description: Some("Your mercy has incredible recuperative properties."),
                pretext: None,
                source_page: Some("p.152"),
                benefit: Some("When you use your lay on hands ability and the target of that ability does not have any conditions your mercies can remove, it instead heals an additional +1d6 points of damage."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Lay on Hands", "PREABILITY:1,CATEGORY=Special Ability,TYPE.Mercy", "PRESTAT:1,CHA=13"]),
            },
            // Greater Spell Specialization -- um_feats.lst:68
            UmFeatEntry {
                key: "Greater Spell Specialization",
                category: FeatCategory::General,
                name: "Greater Spell Specialization",
                description: Some("You can sacrifice a prepared spell in order to spontaneously cast your specialized spell."),
                pretext: None,
                source_page: Some("p.152"),
                benefit: Some("By sacrificing a prepared spell of the same or higher level than your specialized spell, you may spontaneously cast your specialized spell. The specialized spell is treated as its normal level, regardless of the spell slot used to cast it. You may add a metamagic feat to the spell by increasing the spell slot and casting time, just like a cleric spontaneously casting a cure or inf lict spell with a metamagic feat."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Spell Focus", "PREABILITY:1,CATEGORY=FEAT,TYPE.SpellSpecialization", "PRESPELLTYPE:1,Arcane=5,Divine=5", "PRESTAT:1,INT=13"]),
            },
            // Greater Wild Empathy -- um_feats.lst:70
            UmFeatEntry {
                key: "Greater Wild Empathy",
                category: FeatCategory::General,
                name: "Greater Wild Empathy",
                description: None,
                pretext: None,
                source_page: Some("p.152"),
                benefit: Some("You gain a +2 insight bonus on wild empathy checks, and you may use wild empathy to duplicate an Intimidate check rather than a Diplomacy check. In addition, choose one of the following kinds of creatures: elementals, fey, lycanthropes, plants, or vermin. You may inf luence creatures of that type with wild empathy, if their Intelligence score is 1 or 2, or they do not possess an Intelligence score. Once you choose the type of creature, it cannot be changed."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Wild Empathy", "PRESKILL:1,Knowledge (Nature)=5"]),
            },
            // Implant Bomb -- um_feats.lst:71
            UmFeatEntry {
                key: "Implant Bomb",
                category: FeatCategory::General,
                name: "Implant Bomb",
                description: Some("You can attach a bomb to a creature that explodes when the creature dies or after 24 hours."),
                pretext: None,
                source_page: Some("p.152"),
                benefit: Some("You may implant a bomb in a willing or helpless creature (a mindless creature under your control, such as a zombie, counts as willing for this purpose). This takes 1 hour and expends 1 use of your bomb ability for the day. When the implanted creature dies or is destroyed, the bomb detonates in the creature's square as if it were a delayed bomb set by you (though you can set the bomb's damage to less than your normal bomb damage). You can use any bomb-affecting discoveries on this implanted bomb (acid bomb, frost bomb, smoke bomb, and so on) as normal. The bomb automatically detonates 24 hours after you implant it in the creature."),
                effect: Some(&["BONUS:VAR|ImplantedBombDisableDC|11+classlevel(\"Alchemist\")"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Discovery ~ Delayed Bomb", "PRESKILL:1,Heal=5"]),
            },
            // Improved Eldritch Heritage -- um_feats.lst:73
            UmFeatEntry {
                key: "Improved Eldritch Heritage",
                category: FeatCategory::General,
                name: "Improved Eldritch Heritage",
                description: Some("The power of your discovered bloodline continues to grow."),
                pretext: None,
                source_page: Some("p.152"),
                benefit: Some("You gain either the 3rd-level or the 9th-level power (your choice) of the bloodline you selected with the Eldritch Heritage feat. For purposes of using that power, treat your sorcerer level as equal to your character level - 2, even if you have levels in sorcerer. You do not gain any of the other bloodline abilities."),
                effect: Some(&["BONUS:ABILITYPOOL|Eldritch Heritage Selection|1"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Eldritch Heritage", "PRELEVEL:MIN=11", "PREVARGTEQ:CHASCORE,ImprovedEldritchHeritageCharismaPrerequisite"]),
            },
            // Improved Monster Lore -- um_feats.lst:75
            UmFeatEntry {
                key: "Improved Monster Lore",
                category: FeatCategory::General,
                name: "Improved Monster Lore",
                description: Some("You are obsessed with the abilities and weaknesses of monsters."),
                pretext: None,
                source_page: Some("p.153"),
                benefit: Some("You gain a +%1 sacred bonus on all skill checks to identify the abilities and weaknesses of creatures.|ImprovedMonsterLoreBonus"),
                effect: Some(&["BONUS:VAR|ImprovedMonsterLoreBonus|classlevel(\"Inquisitor\")/2"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Monster Lore"]),
            },
            // Insightful Gaze -- um_feats.lst:76
            UmFeatEntry {
                key: "Insightful Gaze",
                category: FeatCategory::General,
                name: "Insightful Gaze",
                description: Some("In your personal interactions, you notice what others don't. It's hard to get anything past you."),
                pretext: None,
                source_page: Some("p.153"),
                benefit: Some("Whenever you make a Sense Motive check to oppose someone's Bluff check, you can roll two dice and take the higher result."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Stern Gaze", "PRESKILL:1,Sense Motive=5"]),
            },
            // Intimidating Gaze -- um_feats.lst:77
            UmFeatEntry {
                key: "Intimidating Gaze",
                category: FeatCategory::General,
                name: "Intimidating Gaze",
                description: Some("There is something in your eyes that scares people."),
                pretext: None,
                source_page: Some("p.153"),
                benefit: Some("Once per day, as a free action, when making an Intimidate skill check, you can roll two dice and take the higher result."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Stern Gaze", "PRESKILL:1,Intimidate=5", "PRESTAT:1,CHA=13"]),
            },
            // Judgment Surge -- um_feats.lst:78
            UmFeatEntry {
                key: "Judgment Surge",
                category: FeatCategory::General,
                name: "Judgment Surge",
                description: Some("Once per day, the power of your faith surges, further empowering your judgments."),
                pretext: None,
                source_page: Some("p.153"),
                benefit: Some("Once per day, you can treat your class level for your judgment class feature as if it were 3 higher than normal. If you have multiple judgments active at the same time, this benefit applies to all of them."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.InquisitorJudgment"]),
            },
            // Ki Stand -- um_feats.lst:79
            UmFeatEntry {
                key: "Ki Stand",
                category: FeatCategory::General,
                name: "Ki Stand",
                description: Some("If an opponent knocks you down, you swiftly rebound with an attack."),
                pretext: None,
                source_page: Some("p.153"),
                benefit: Some("While you have at least 1 ki point in your ki pool, you can stand up as a swift action that provokes attacks of opportunity."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Ki Pool"]),
            },
            // Learn Ranger Trap -- um_feats.lst:81
            UmFeatEntry {
                key: "Learn Ranger Trap",
                category: FeatCategory::General,
                name: "Learn Ranger Trap",
                description: Some("You learn how to create one kind of ranger trap."),
                pretext: None,
                source_page: Some("p.153"),
                benefit: Some("Select one ranger trap (see page 64 of Ultimate Magic). You may use this trap %1 times per day. The DC for your trap is %2, and it lasts %3 days.|TrapTimes|TrapDC|TrapDuration"),
                effect: Some(&["BONUS:ABILITYPOOL|Ranger Trap|1", "BONUS:VAR|RangerTrapLVL|TL", "BONUS:VAR|TrapTimes|max(1,WIS+classlevel(\"Ranger\")/2)", "BONUS:VAR|TrapDC|10+CL/2+WIS", "BONUS:VAR|TrapDuration|CL/2"]),
                prerequisites: Some(&["PRESKILL:1,Survival=5"]),
            },
            // Life Lure -- um_feats.lst:83
            UmFeatEntry {
                key: "Life Lure",
                category: FeatCategory::General,
                name: "Life Lure",
                description: Some("Your channeled positive energy is irresistibly sweet to nearby undead."),
                pretext: None,
                source_page: Some("p.153"),
                benefit: Some("As a standard action, you can channel positive energy to fascinate all undead within 30 feet for a %1 rounds. Undead that succeed at a Will save are unaffected. Use the same DC for this ability as the DC for channeling energy to harm undead. Channeling energy for this purpose does not heal or harm creatures.|LifeLureDuration"),
                effect: Some(&["BONUS:VAR|LifeLureDuration|max(1,CHA)"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Positive Energy"]),
            },
            // Moonlight Summons -- um_feats.lst:84
            UmFeatEntry {
                key: "Moonlight Summons",
                category: FeatCategory::General,
                name: "Moonlight Summons",
                description: Some("Your summoned minions are infused with the power of the moon."),
                pretext: None,
                source_page: Some("p.153"),
                benefit: Some("Creatures you summon shed light as a light spell. They are immune to confusion and sleep effects, and their natural weapons are treated as silver for the purposes of overcoming damage reduction."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Conjuration)", "PRESPELL:1,Summon Nature's Ally I,Summon Nature's Ally II,Summon Nature's Ally III,Summon Nature's Ally IV,Summon Nature's Ally V,Summon Nature's Ally VI,Summon Nature's Ally VII,Summon Nature's Ally VIII,Summon Nature's Ally IX"]),
            },
            // Mystic Stride -- um_feats.lst:85
            UmFeatEntry {
                key: "Mystic Stride",
                category: FeatCategory::General,
                name: "Mystic Stride",
                description: Some("Enchanted vegetation does not bar your path."),
                pretext: None,
                source_page: Some("p.153"),
                benefit: Some("You can move at full speed even through thorns, briars, and overgrown areas that are enchanted or magically manipulated to impede motion, even if those areas confer the entangled condition."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Woodland Stride", "PREABILITY:1,CATEGORY=FEAT,Nimble Moves", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,15],[PREVARGTEQ:FeatDexRequirement,15]"]),
            },
            // Oracular Intuition -- um_feats.lst:86
            UmFeatEntry {
                key: "Oracular Intuition",
                category: FeatCategory::General,
                name: "Oracular Intuition",
                description: Some("You are highly sensitive to magic and changes in a person's demeanor."),
                pretext: None,
                source_page: Some("p.153"),
                benefit: Some("You get a +2 bonus on Sense Motive checks and Spellcraft checks. If you have 10 or more ranks in one of these skills, the bonus increases to +4 for that skill."),
                effect: Some(&["BONUS:SKILL|Sense Motive|if(skillinfo(\"TOTALRANK\",\"Sense Motive\")>=10,4,2)", "BONUS:SKILL|Spellcraft|if(skillinfo(\"TOTALRANK\",\"Spellcraft\")>=10,4,2)"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.OracleMystery"]),
            },
            // Painful Anchor -- um_feats.lst:87
            UmFeatEntry {
                key: "Painful Anchor",
                category: FeatCategory::General,
                name: "Painful Anchor",
                description: Some("Evil outsiders take damage when they attempt to connect to other planes."),
                pretext: None,
                source_page: Some("p.153"),
                benefit: Some("When an evil outsider uses a calling, summoning, or teleportation effect, or any ability that physically transports a creature to or from another plane (such as blink or etherealness) within your anchoring aura, it takes damage equal to 4d8 + %1. This damage comes from holy power and is not subject to damage reduction, energy immunities, or energy resistances.|PainfulAnchorBonusDamage"),
                effect: Some(&["BONUS:VAR|PainfulAnchorBonusDamage|CHA"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Anchoring Aura ~ Oath against Fiends"]),
            },
            // Piercing Spell -- um_feats.lst:88
            UmFeatEntry {
                key: "Piercing Spell",
                category: FeatCategory::Metamagic,
                name: "Piercing Spell",
                description: Some("Your studies have helped you develop methods to overcome spell resistance."),
                pretext: None,
                source_page: Some("p.154"),
                benefit: Some("When you cast a piercing spell against a target with spell resistance, it treats the spell resistance of the target as 5 lower than its actual SR. A piercing spell uses up a spell slot one level higher than the spell's actual level."),
                effect: None,
                prerequisites: None,
            },
            // Planar Preservationist -- um_feats.lst:89
            UmFeatEntry {
                key: "Planar Preservationist",
                category: FeatCategory::General,
                name: "Planar Preservationist",
                description: Some("You know how to preserve and reconstitute extraplanar monsters as well as normal animals."),
                pretext: None,
                source_page: Some("p.154"),
                benefit: Some("For every summon nature's ally extract you know, you learn the equivalent summon monster spell as an extract. If you later learn other summon nature's ally extracts, you automatically learn the equivalent summon monster spell as an extract."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Archetype,Alchemist Archetype ~ Preservationist"]),
            },
            // Powerful Shape -- um_feats.lst:91
            UmFeatEntry {
                key: "Powerful Shape",
                category: FeatCategory::General,
                name: "Powerful Shape",
                description: Some("Your wild shapes are mighty and muscular."),
                pretext: None,
                source_page: Some("p.154"),
                benefit: Some("When in wild shape, treat your size as one category larger for the purpose of calculating CMB, CMD, carrying capacity, and any size-based special attacks you use or that are used against you (such as grab, swallow whole, and trample)."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape", "PREVARGTEQ:DruidLVL,8"]),
            },
            // Prodigy -- um_feats.lst:93
            UmFeatEntry {
                key: "Prodigy",
                category: FeatCategory::General,
                name: "Prodigy",
                description: Some("You are naturally skilled at arts, professions, and the acquisition of knowledge."),
                pretext: None,
                source_page: Some("p.154"),
                benefit: Some("Choose two Craft, Perform, or Profession skills in any combination (two Craft skills, a Craft skill and a Perform skill, and so on). You receive a +2 bonus on checks with these skills. If you have 10 or more ranks in any one of these skills, the bonus increases to +4 for that skill."),
                effect: Some(&["BONUS:SKILL|%LIST|if(skillinfo(\"TOTALRANK\",\"%LIST\")>=10,4,2)|TYPE=Prodigy"]),
                prerequisites: None,
            },
            // Prophetic Visionary -- um_feats.lst:94
            UmFeatEntry {
                key: "Prophetic Visionary",
                category: FeatCategory::General,
                name: "Prophetic Visionary",
                description: Some("Your oracular abilities give you a glimpse into the future."),
                pretext: None,
                source_page: Some("p.154"),
                benefit: Some("Once per day, you can enter a deep trance to receive a vision of the future. The trance lasts for 10 minutes, during which time you can take no other actions. If you are interrupted, you must begin again. When you come out of the trance, you know whether a particular action in the immediate future will bring good or bad results, as an augury spell with a 70%% chance of success."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.OracleMystery"]),
            },
            // Pure Faith -- um_feats.lst:95
            UmFeatEntry {
                key: "Pure Faith",
                category: FeatCategory::General,
                name: "Pure Faith",
                description: Some("Not only are you immune to disease, like most paladins, but you also are highly resilient to poisons."),
                pretext: None,
                source_page: Some("p.154"),
                benefit: Some("You gain a +4 sacred bonus to saving throws against poison."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Paladin ~ Divine Health"]),
            },
            // Quarterstaff Master -- um_feats.lst:99
            UmFeatEntry {
                key: "Quarterstaff Master",
                category: FeatCategory::Combat,
                name: "Quarterstaff Master",
                description: Some("You can wield a quarterstaff as either a two-handed or one-handed weapon."),
                pretext: None,
                source_page: Some("p.154"),
                benefit: Some("By employing a number of different stances and techniques, you can wield a quarterstaff as a onehanded weapon. At the start of your turn, you decide whether or not you are going to wield the quarterstaff as a one-handed or two-handed weapon. When you wield it as a one-handed weapon, your other hand is free, and you cannot use the staff as a double weapon. You can take the feat Weapon Specialization in the quarterstaff even if you have no levels in fighter."),
                effect: Some(&["BONUS:VAR|WeapSpecQualify|1"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Weapon Focus (Quarterstaff)", "PRETOTALAB:5"]),
            },
            // Quick Channel -- um_feats.lst:100
            UmFeatEntry {
                key: "Quick Channel",
                category: FeatCategory::General,
                name: "Quick Channel",
                description: Some("Your divine energies flash with dazzling speed."),
                pretext: None,
                source_page: Some("p.154"),
                benefit: Some("You may channel energy as a move action by spending 2 daily uses of that ability."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Energy", "PRESKILL:1,Knowledge (Religion)=5"]),
            },
            // Quick Wild Shape -- um_feats.lst:101
            UmFeatEntry {
                key: "Quick Wild Shape",
                category: FeatCategory::General,
                name: "Quick Wild Shape",
                description: Some("You sacrifice power for speed in changing form."),
                pretext: None,
                source_page: Some("p.154"),
                benefit: Some("You can wild shape as a move action or a swift action. However, you are limited to forms available to a druid two levels lower when changing form as a move action, or four levels lower as a swift action."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape", "PREMULT:1,[PRECLASS:1,SPELLCASTER=8],[PREVARGTEQ:CasterLevel_Highest,8]"]),
            },
            // Radiant Charge -- um_feats.lst:102
            UmFeatEntry {
                key: "Radiant Charge",
                category: FeatCategory::General,
                name: "Radiant Charge",
                description: Some("When you charge, you do so with the power of faith."),
                pretext: None,
                source_page: Some("p.154"),
                benefit: Some("When you hit with a charge attack, you can expend all of your remaining uses of lay on hands to deal extra damage equal to 1d6 per use of lay on hands expended + %1. This damage comes from holy power and is not subject to damage reduction, energy immunities, or energy resistances.|RadiantChargeBonusDamage"),
                effect: Some(&["BONUS:VAR|RadiantChargeBonusDamage|CHA"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Lay On Hands"]),
            },
            // Remote Bomb -- um_feats.lst:103
            UmFeatEntry {
                key: "Remote Bomb",
                category: FeatCategory::General,
                name: "Remote Bomb",
                description: Some("You can set off your delayed bombs at great distances."),
                pretext: None,
                source_page: Some("p.155"),
                benefit: Some("The maximum delay for your delayed bombs increases to %1 minutes. If you have line of effect to your delayed bomb, you may detonate it earlier than its preset time by making a DC 20 Intelligence check; the DC increases by +1 for every 10 feet of distance between you and the bomb.|TL"),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Discovery ~ Delayed Bomb"]),
            },
            // Resilient Eidolon -- um_feats.lst:104
            UmFeatEntry {
                key: "Resilient Eidolon",
                category: FeatCategory::General,
                name: "Resilient Eidolon",
                description: Some("Your link with your eidolon is strong enough that it can remain with you for a short time after you fall unconscious or are killed."),
                pretext: None,
                source_page: Some("p.155"),
                benefit: Some("If you are knocked unconscious, fall asleep, or are killed, your eidolon remains for %1 rounds before it is banished. If you are brought back to consciousness before this duration expires, your eidolon is not banished. If the duration expires before you are brought back to consciousness, your eidolon is banished normally.|ResilientEidolonDuration"),
                effect: Some(&["BONUS:VAR|ResilientEidolonDuration|classlevel(\"Summoner\")"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Eidolon"]),
            },
            // Reward of Grace -- um_feats.lst:105
            UmFeatEntry {
                key: "Reward of Grace",
                category: FeatCategory::General,
                name: "Reward of Grace",
                description: Some("When you lay on hands, divine energy ripples through you, granting you grace."),
                pretext: None,
                source_page: Some("p.155"),
                benefit: Some("Each time you use your lay on hands ability, you gain a +1 sacred bonus on all attack rolls for 1 round."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Lay On Hands"]),
            },
            // Reward of Life -- um_feats.lst:106
            UmFeatEntry {
                key: "Reward of Life",
                category: FeatCategory::General,
                name: "Reward of Life",
                description: Some("When you lay on hands, you are also healed."),
                pretext: None,
                source_page: Some("p.155"),
                benefit: Some("Each time you use your lay on hands ability to heal a creature other than yourself, you heal %1 hit points. This ability has no effect if you use lay on hands to harm undead.|RewardOfLifeHealing"),
                effect: Some(&["BONUS:VAR|RewardOfLifeHealing|CHA"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Lay On Hands"]),
            },
            // Ricochet Splash Weapon -- um_feats.lst:107
            UmFeatEntry {
                key: "Ricochet Splash Weapon",
                category: FeatCategory::General,
                name: "Ricochet Splash Weapon",
                description: Some("Even when your thrown splash weapons miss, they are especially dangerous."),
                pretext: None,
                source_page: Some("p.155"),
                benefit: Some("Whenever your splash weapon misses and the misdirection roll indicates it lands in a square occupied by a creature, you may make an attack roll (at a -5 penalty) as if you had thrown the splash weapon at that creature. If this attack roll succeeds, the splash weapon hits and the creature takes full damage instead of splash damage. Squares adjacent to this creature still take splash damage as normal."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Throw Anything", "PREMULT:1,[PREVARGTEQ:PreStatScore_DEX,13],[PREVARGTEQ:FeatDexRequirement,13]"]),
            },
            // Rime Spell -- um_feats.lst:108
            UmFeatEntry {
                key: "Rime Spell",
                category: FeatCategory::Metamagic,
                name: "Rime Spell",
                description: Some("Creatures damaged by your spells with the cold descriptor become entangled."),
                pretext: None,
                source_page: Some("p.155"),
                benefit: Some("The frost of your cold spell clings to the target, impeding it for a short time. A rime spell causes creatures that takes cold damage from the spell to become entangled for a number of rounds equal to the original level of the spell. This feat only affects spells with the cold descriptor. A rime spell uses up a spell slot one level higher than the spell's actual level."),
                effect: None,
                prerequisites: None,
            },
            // Sacred Summons -- um_feats.lst:109
            UmFeatEntry {
                key: "Sacred Summons",
                category: FeatCategory::General,
                name: "Sacred Summons",
                description: Some("The minions of your divine patrons stand ready to answer your call."),
                pretext: None,
                source_page: Some("p.155"),
                benefit: Some("When using summon monster to summon creatures whose alignment subtype or subtypes exactly match your aura, you may cast the spell as a standard action instead of with a casting time of 1 round."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Aura of Chaos,Aura of Evil,Aura of Good,Aura of Law", "PRESPELL:1,Summon Monster I,Summon Monster II,Summon Monster III,Summon Monster IV,Summon Monster V,Summon Monster VI,Summon Monster VII,Summon Monster VIII,Summon Monster IX"]),
            },
            // Sense Link -- um_feats.lst:110
            UmFeatEntry {
                key: "Sense Link",
                category: FeatCategory::General,
                name: "Sense Link",
                description: Some("When you and your eidolon share senses, your combined minds grant you exceptional powers of observation."),
                pretext: None,
                source_page: Some("p.155"),
                benefit: Some("When sharing the senses of your eidolon, you gain a +4 competence bonus on Perception checks for the duration of your bond senses ability."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Bond Senses"]),
            },
            // Shaping Focus -- um_feats.lst:111
            UmFeatEntry {
                key: "Shaping Focus",
                category: FeatCategory::General,
                name: "Shaping Focus",
                description: Some("Your powers of shapeshifting outstrip your dabbling in the druidic faith."),
                pretext: None,
                source_page: Some("p.155"),
                benefit: Some("If you are a multiclassed druid, your wild shape ability is calculated as though your druid level were four higher, to a maximum level equal to your character level."),
                effect: Some(&["BONUS:VAR|DruidWildShape|MIN(4,classlevel(\"TYPE=PC\")+classlevel(\"TYPE=NPC\")-classlevel(\"Druid\"))"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape", "PRESKILL:1,Knowledge (Nature)=5"]),
            },
            // Sin Seer -- um_feats.lst:112
            UmFeatEntry {
                key: "Sin Seer",
                category: FeatCategory::General,
                name: "Sin Seer",
                description: Some("Unlike others who have taken the oath against undeath, your preoccupation with the undead does not cloud your view of good and evil."),
                pretext: None,
                source_page: Some("p.155"),
                benefit: Some("You gain the detect evil class feature. You may use it or the detect undead class feature, but not at the same time."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Detect Undead ~ Oath against Undeath"]),
            },
            // Skeleton Summoner -- um_feats.lst:113
            UmFeatEntry {
                key: "Skeleton Summoner",
                category: FeatCategory::General,
                name: "Skeleton Summoner",
                description: Some("The walking dead respond to your call."),
                pretext: None,
                source_page: Some("p.155"),
                benefit: Some("Add \"human skeleton\" to the list of creatures you can summon with summon monster I and \"human skeletal champion\" to the list of creatures you can summon with summon monster III."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Necromancy)", "PRESPELL:1,Summon Monster I,Summon Monster II,Summon Monster III,Summon Monster IV,Summon Monster V,Summon Monster VI,Summon Monster VII,Summon Monster VIII,Summon Monster IX"]),
            },
            // Sorcerous Bloodstrike -- um_feats.lst:114
            UmFeatEntry {
                key: "Sorcerous Bloodstrike",
                category: FeatCategory::General,
                name: "Sorcerous Bloodstrike",
                description: Some("You can regain power when you kill a creature."),
                pretext: None,
                source_page: Some("p.156"),
                benefit: Some("Once per day, as an immediate action upon reducing a creature to 0 or fewer hit points with one of your sorcerer spells, you can regain one usage of a sorcerer bloodline power that has a limited number of uses per day. The slain creature must have at least half as many Hit Dice as your sorcerer level. You cannot use this feat to gain another usage of a bloodline power that you have not yet used today."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Sorcerer ~ Standard Bloodline", "PRESTAT:1,CHA=13"]),
            },
            // Spell Bluff -- um_feats.lst:115
            UmFeatEntry {
                key: "Spell Bluff",
                category: FeatCategory::General,
                name: "Spell Bluff",
                description: Some("You know the principles of arcane dueling, and when fighting other spellcasters, you have learned to hide the true nature of your spells until the last possible moment."),
                pretext: None,
                source_page: Some("p.156"),
                benefit: Some("If another spellcaster tries to counterspell your casting, she adds +4 to her Spellcraft DC when trying to determine your spell. Because you have studied how to mask the recognizable elements of your spellcasting, you gain a +2 bonus on your Spellcraft checks to identify and counter an opponent's spell if it is a spell you know or have in your spellbook."),
                effect: None,
                prerequisites: Some(&["PRESKILL:2,Bluff=5,Spellcraft=5"]),
            },
            // Spell Hex -- um_feats.lst:119
            UmFeatEntry {
                key: "Spell Hex",
                category: FeatCategory::General,
                name: "Spell Hex",
                description: Some("You can transform a 1st-level spell into a hex."),
                pretext: None,
                source_page: Some("p.156"),
                benefit: Some("Select one 1st-level spell in the class that grants you the major hex class feature. You can learn that spell as a hex, and can use that hex three times per day. This is a spell-like ability. You use your class level in the major-hex-granting class as your caster level for the spell hex. The spell hex uses your hex DC instead of its original spell DC."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.MajorHex", "PREVARGTEQ:WitchLVL,10"]),
            },
            // Spell Specialization (Abjuration) -- um_feats.lst:120
            UmFeatEntry {
                key: "Spell Specialization (Abjuration)",
                category: FeatCategory::General,
                name: "Spell Specialization (Abjuration)",
                description: Some("Select one spell. You cast that spell with greater than normal power."),
                pretext: None,
                source_page: Some("p.156"),
                benefit: Some("Select one abjuration spell. Treat your caster level as being two higher for all level-variable effects of the spell. Every time you gain an even level in the spellcasting class you chose your spell from, you can choose a new spell to replace the spell selected with this feat, and that spell becomes your specialized spell."),
                effect: Some(&["BONUS:CASTERLEVEL|SPELL.%LIST|2"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Abjuration)", "PRESTAT:1,INT=13"]),
            },
            // Spell Specialization (Conjuration) -- um_feats.lst:121
            UmFeatEntry {
                key: "Spell Specialization (Conjuration)",
                category: FeatCategory::General,
                name: "Spell Specialization (Conjuration)",
                description: Some("Select one spell. You cast that spell with greater than normal power."),
                pretext: None,
                source_page: Some("p.156"),
                benefit: Some("Select one conjuration spell. Treat your caster level as being two higher for all level-variable effects of the spell. Every time you gain an even level in the spellcasting class you chose your spell from, you can choose a new spell to replace the spell selected with this feat, and that spell becomes your specialized spell."),
                effect: Some(&["BONUS:CASTERLEVEL|SPELL.%LIST|2"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Conjuration)", "PRESTAT:1,INT=13"]),
            },
            // Spell Specialization (Divination) -- um_feats.lst:122
            UmFeatEntry {
                key: "Spell Specialization (Divination)",
                category: FeatCategory::General,
                name: "Spell Specialization (Divination)",
                description: Some("Select one spell. You cast that spell with greater than normal power."),
                pretext: None,
                source_page: Some("p.156"),
                benefit: Some("Select one divination spell. Treat your caster level as being two higher for all level-variable effects of the spell. Every time you gain an even level in the spellcasting class you chose your spell from, you can choose a new spell to replace the spell selected with this feat, and that spell becomes your specialized spell."),
                effect: Some(&["BONUS:CASTERLEVEL|SPELL.%LIST|2"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Divination)", "PRESTAT:1,INT=13"]),
            },
            // Spell Specialization (Enchantment) -- um_feats.lst:123
            UmFeatEntry {
                key: "Spell Specialization (Enchantment)",
                category: FeatCategory::General,
                name: "Spell Specialization (Enchantment)",
                description: Some("Select one spell. You cast that spell with greater than normal power."),
                pretext: None,
                source_page: Some("p.156"),
                benefit: Some("Select one enchantment spell. Treat your caster level as being two higher for all level-variable effects of the spell. Every time you gain an even level in the spellcasting class you chose your spell from, you can choose a new spell to replace the spell selected with this feat, and that spell becomes your specialized spell."),
                effect: Some(&["BONUS:CASTERLEVEL|SPELL.%LIST|2"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Enchantment)", "PRESTAT:1,INT=13"]),
            },
            // Spell Specialization (Evocation) -- um_feats.lst:124
            UmFeatEntry {
                key: "Spell Specialization (Evocation)",
                category: FeatCategory::General,
                name: "Spell Specialization (Evocation)",
                description: Some("Select one spell. You cast that spell with greater than normal power."),
                pretext: None,
                source_page: Some("p.156"),
                benefit: Some("Select one evocation spell. Treat your caster level as being two higher for all level-variable effects of the spell. Every time you gain an even level in the spellcasting class you chose your spell from, you can choose a new spell to replace the spell selected with this feat, and that spell becomes your specialized spell."),
                effect: Some(&["BONUS:CASTERLEVEL|SPELL.%LIST|2"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Evocation)", "PRESTAT:1,INT=13"]),
            },
            // Spell Specialization (Illusion) -- um_feats.lst:125
            UmFeatEntry {
                key: "Spell Specialization (Illusion)",
                category: FeatCategory::General,
                name: "Spell Specialization (Illusion)",
                description: Some("Select one spell. You cast that spell with greater than normal power."),
                pretext: None,
                source_page: Some("p.156"),
                benefit: Some("Select one illusion spell. Treat your caster level as being two higher for all level-variable effects of the spell. Every time you gain an even level in the spellcasting class you chose your spell from, you can choose a new spell to replace the spell selected with this feat, and that spell becomes your specialized spell."),
                effect: Some(&["BONUS:CASTERLEVEL|SPELL.%LIST|2"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Illusion)", "PRESTAT:1,INT=13"]),
            },
            // Spell Specialization (Necromancy) -- um_feats.lst:126
            UmFeatEntry {
                key: "Spell Specialization (Necromancy)",
                category: FeatCategory::General,
                name: "Spell Specialization (Necromancy)",
                description: Some("Select one spell. You cast that spell with greater than normal power."),
                pretext: None,
                source_page: Some("p.156"),
                benefit: Some("Select one necromancy spell. Treat your caster level as being two higher for all level-variable effects of the spell. Every time you gain an even level in the spellcasting class you chose your spell from, you can choose a new spell to replace the spell selected with this feat, and that spell becomes your specialized spell."),
                effect: Some(&["BONUS:CASTERLEVEL|SPELL.%LIST|2"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Necromancy)", "PRESTAT:1,INT=13"]),
            },
            // Spell Specialization (Transmutation) -- um_feats.lst:127
            UmFeatEntry {
                key: "Spell Specialization (Transmutation)",
                category: FeatCategory::General,
                name: "Spell Specialization (Transmutation)",
                description: Some("Select one spell. You cast that spell with greater than normal power."),
                pretext: None,
                source_page: Some("p.156"),
                benefit: Some("Select one transmutation spell. Treat your caster level as being two higher for all level-variable effects of the spell. Every time you gain an even level in the spellcasting class you chose your spell from, you can choose a new spell to replace the spell selected with this feat, and that spell becomes your specialized spell."),
                effect: Some(&["BONUS:CASTERLEVEL|SPELL.%LIST|2"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Transmutation)", "PRESTAT:1,INT=13"]),
            },
            // Spellsong -- um_feats.lst:128
            UmFeatEntry {
                key: "Spellsong",
                category: FeatCategory::General,
                name: "Spellsong",
                description: Some("You can blend the power of your performance and spellcasting."),
                pretext: None,
                source_page: Some("p.156"),
                benefit: Some("You can combine your bardic performance and your spellcasting in two ways."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Bardic Performance", "PRESPELLTYPE:1,Arcane=1,Divine=1", "PRESTAT:1,CHA=13"]),
            },
            // Split Hex -- um_feats.lst:129
            UmFeatEntry {
                key: "Split Hex",
                category: FeatCategory::General,
                name: "Split Hex",
                description: Some("You can split the effect of one of your targeted hexes, affecting another creature you can see."),
                pretext: None,
                source_page: Some("p.156"),
                benefit: Some("When you use one of your hexes (not a major hex or a grand hex) that targets a single creature, you can choose another creature within 30 feet of the first target to also be targeted by the hex."),
                effect: None,
                prerequisites: Some(&["PREVARGTEQ:WitchLVL,10"]),
            },
            // Split Major Hex -- um_feats.lst:130
            UmFeatEntry {
                key: "Split Major Hex",
                category: FeatCategory::General,
                name: "Split Major Hex",
                description: Some("You can split the effect of one of your targeted hexes, affecting another creature you can see."),
                pretext: None,
                source_page: Some("p.156"),
                benefit: Some("When you use one of your major hexes (not a grand hex) that targets a creature, you can choose another creature within 30 feet of the first target to also be targeted by the major hex."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Split Hex", "PREMULT:1,[PRECLASS:1,SPELLCASTER=18],[PREVARGTEQ:CasterLevel_Highest,18]"]),
            },
            // Spontaneous Metafocus -- um_feats.lst:135
            UmFeatEntry {
                key: "Spontaneous Metafocus",
                category: FeatCategory::General,
                name: "Spontaneous Metafocus",
                description: Some("You can focus to combine one of your known spells and metamagic feats."),
                pretext: None,
                source_page: Some("p.157"),
                benefit: Some("Pick a single spell that you are able to cast spontaneously. When you apply metamagic feats to that spell, you can cast the spell using the normal casting time instead of at the slower casting time."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,TYPE.Metamagic", "PRESTAT:1,CHA=13"]),
            },
            // Starlight Summons -- um_feats.lst:136
            UmFeatEntry {
                key: "Starlight Summons",
                category: FeatCategory::General,
                name: "Starlight Summons",
                description: Some("Your summoned minions slink along under the shadows of the stars."),
                pretext: None,
                source_page: Some("p.157"),
                benefit: Some("Creatures you summon gain the Blind-Fight feat, a +5 bonus to Perception and Stealth checks in dim light or darkness, and their natural weapons are treated as cold iron for overcoming damage reduction."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Conjuration)", "PRESPELL:1,Summon Nature's Ally I,Summon Nature's Ally II,Summon Nature's Ally III,Summon Nature's Ally IV,Summon Nature's Ally V,Summon Nature's Ally VI,Summon Nature's Ally VII,Summon Nature's Ally VIII,Summon Nature's Ally IX"]),
            },
            // Sunlight Summons -- um_feats.lst:137
            UmFeatEntry {
                key: "Sunlight Summons",
                category: FeatCategory::General,
                name: "Sunlight Summons",
                description: Some("Your summoned minions shine with the power of the sun."),
                pretext: None,
                source_page: Some("p.157"),
                benefit: Some("Creatures that you summon shed light as a light spell. They are immune to blinding or dazzling effects, and their natural weapons are treated as magical for overcoming damage reduction."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Conjuration)", "PRESPELL:1,Summon Nature's Ally I,Summon Nature's Ally II,Summon Nature's Ally III,Summon Nature's Ally IV,Summon Nature's Ally V,Summon Nature's Ally VI,Summon Nature's Ally VII,Summon Nature's Ally VIII,Summon Nature's Ally IX"]),
            },
            // Superior Summoning -- um_feats.lst:138
            UmFeatEntry {
                key: "Superior Summoning",
                category: FeatCategory::General,
                name: "Superior Summoning",
                description: Some("You can summon more creatures."),
                pretext: None,
                source_page: Some("p.157"),
                benefit: Some("Each time you cast a summoning spell that conjures more than one creature, add one to the total number of creatures summoned."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Augment Summoning", "PREMULT:1,[PRECLASS:1,SPELLCASTER=3],[PREVARGTEQ:CasterLevel_Highest,3]"]),
            },
            // Thanatopic Spell -- um_feats.lst:139
            UmFeatEntry {
                key: "Thanatopic Spell",
                category: FeatCategory::Metamagic,
                name: "Thanatopic Spell",
                description: Some("Your spells can pierce wards against negative energy and even affect undead targets."),
                pretext: None,
                source_page: Some("p.157"),
                benefit: Some("A thanatopic spell pierces defenses and immunities that protect against death effects, negative levels, and energy drain, affecting the target as if the protective barrier did not exist."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Necromancy)", "PRESKILL:1,Knowledge (Religion)=6"]),
            },
            // Theurgy -- um_feats.lst:140
            UmFeatEntry {
                key: "Theurgy",
                category: FeatCategory::General,
                name: "Theurgy",
                description: Some("You can blend the power of arcane and divine magic."),
                pretext: None,
                source_page: Some("p.157"),
                benefit: Some("You can augment the power of your divine spells with arcane energy and augment your arcane spells with divine energy."),
                effect: None,
                prerequisites: Some(&["PRESPELLTYPE:1,Arcane=1", "PRESPELLTYPE:1,Divine=1", "PRESTAT:1,WIS=13", "PRESTAT:1,INT=13,CHA=13"]),
            },
            // Thoughtful Discernment -- um_feats.lst:142
            UmFeatEntry {
                key: "Thoughtful Discernment",
                category: FeatCategory::General,
                name: "Thoughtful Discernment",
                description: Some("Thinking back, you discover a lie in what you once thought to be words of truth."),
                pretext: None,
                source_page: Some("p.157"),
                benefit: Some("Once per day as a free action, you can think back about a single statement you heard in the last day and determine if it was a lie. This acts like the discern lies spell, but instead of affecting a creature, it affects a single statement a creature has made."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Inquisitor ~ Discern Lies"]),
            },
            // Threnodic Spell -- um_feats.lst:143
            UmFeatEntry {
                key: "Threnodic Spell",
                category: FeatCategory::Metamagic,
                name: "Threnodic Spell",
                description: Some("You can convert mind-affecting magic to necromantic power capable of controlling undead."),
                pretext: None,
                source_page: Some("p.157"),
                benefit: Some("This feat only works on mind-affecting spells. A threnodic spell affects undead creatures (even mindless undead) as if they weren't immune to mind-affecting effects, but has no effect on living creatures. A threnodic spell uses up a spell slot two level higher than the spell's actual level."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Necromancy)", "PRESKILL:1,Knowledge (Religion)=6"]),
            },
            // Toppling Spell -- um_feats.lst:144
            UmFeatEntry {
                key: "Toppling Spell",
                category: FeatCategory::Metamagic,
                name: "Toppling Spell",
                description: Some("Your spells with the force descriptor knock the affected creatures prone."),
                pretext: None,
                source_page: Some("p.158"),
                benefit: Some("The impact of your force spell is strong enough to knock the target prone. If the target takes damage, fails its saving throw, or is moved by your force spell, make a trip check against the target, using your caster level plus your casting ability score bonus (Wisdom for clerics, Intelligence for wizards, and so on). This does not provoke an attack of opportunity. If the check fails, the target cannot attempt to trip you or the force effect in response. A toppling spell only affects spells with the force descriptor. A toppling spell uses up a spell slot one level higher than the spell's actual level."),
                effect: None,
                prerequisites: None,
            },
            // Tripping Staff -- um_feats.lst:145
            UmFeatEntry {
                key: "Tripping Staff",
                category: FeatCategory::Combat,
                name: "Tripping Staff",
                description: Some("You can make a trip attack with your quarterstaff."),
                pretext: None,
                source_page: Some("p.158"),
                benefit: Some("You treat quarterstaves as if they had the trip special feature. If you are a magus with the staff magus archetype, you can use spellstrike on any trip combat maneuver you make with the staff."),
                effect: None,
                prerequisites: Some(&["PREABILITY:3,CATEGORY=FEAT,Combat Expertise,Improved Trip,Weapon Focus (Quarterstaff)", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:6"]),
            },
            // Tripping Twirl -- um_feats.lst:146
            UmFeatEntry {
                key: "Tripping Twirl",
                category: FeatCategory::Combat,
                name: "Tripping Twirl",
                description: Some("You can make a trip attack with a quarterstaff on all adjacent enemies."),
                pretext: None,
                source_page: None,
                benefit: Some("As a full-round action, while wielding a quarterstaff two-handed, you can attempt a trip combat maneuver against each enemy adjacent to you."),
                effect: None,
                prerequisites: Some(&["PREABILITY:5,CATEGORY=FEAT,Combat Expertise,Improved Trip,Tripping Staff,Weapon Focus (Quarterstaff),Weapon Specialization (Quarterstaff)", "PREMULT:1,[PRESTAT:1,INT=13],[PREVARGTEQ:CombatFeatIntRequirement,13]", "PRETOTALAB:12"]),
            },
            // Ultimate Mercy -- um_feats.lst:147
            UmFeatEntry {
                key: "Ultimate Mercy",
                category: FeatCategory::General,
                name: "Ultimate Mercy",
                description: Some("By using lay on hands, you can bring the dead back to life."),
                pretext: None,
                source_page: Some("p.158"),
                benefit: Some("You can expend 10 uses of lay on hands to bring a single dead creature you touch back to life as a raise dead spell with a caster level equal to your paladin level. You must provide the material component for raise dead or choose to accept 1 temporary negative level; this level automatically goes away after 24 hours, never becomes a permanent negative level, and cannot be overcome in any way except by waiting for the duration to expire."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Lay on Hands", "PREABILITY:1,CATEGORY=Special Ability,TYPE.Mercy", "PREABILITY:1,CATEGORY=FEAT,Greater Mercy", "PRESTAT:1,CHA=19"]),
            },
            // Ultimate Resolve -- um_feats.lst:149
            UmFeatEntry {
                key: "Ultimate Resolve",
                category: FeatCategory::General,
                name: "Ultimate Resolve",
                description: Some("Your aura of resolve does not fall with you."),
                pretext: None,
                source_page: Some("p.158"),
                benefit: Some("Your aura of resolve is a 20-foot emanation, and does not end if you fall unconscious."),
                effect: Some(&["BONUS:VAR|AuraOfResolveRadius|10"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Aura of Resolve"]),
            },
            // Uncanny Alertness -- um_feats.lst:150
            UmFeatEntry {
                key: "Uncanny Alertness",
                category: FeatCategory::General,
                name: "Uncanny Alertness",
                description: Some("Your research into arcana and the nature of reality has given you heightened senses."),
                pretext: None,
                source_page: Some("p.158"),
                benefit: Some("This feat gives you an additional +1 bonus on Perception and Sense Motive checks, and you gain a +2 bonus on saving throws against sleep and charm effects."),
                effect: Some(&["BONUS:SKILL|Perception|1", "BONUS:SKILL|Sense Motive|1"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Alertness"]),
            },
            // Uncanny Concentration -- um_feats.lst:151
            UmFeatEntry {
                key: "Uncanny Concentration",
                category: FeatCategory::General,
                name: "Uncanny Concentration",
                description: Some("You have learned to enter a deeper state when casting spells, allowing you to shrug off distractions, damage, weather effects, and even the effects of other spells."),
                pretext: None,
                source_page: Some("p.158"),
                benefit: Some("You do not need to make concentration checks when affected by vigorous or violent motion or by violent weather. You gain a +2 bonus on all other concentration checks."),
                effect: Some(&["BONUS:CONCENTRATION|ALLSPELLS|2"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Combat Casting"]),
            },
            // Undead Master -- um_feats.lst:152
            UmFeatEntry {
                key: "Undead Master",
                category: FeatCategory::General,
                name: "Undead Master",
                description: Some("You can marshal vast armies of the undead to serve you."),
                pretext: None,
                source_page: Some("p.158"),
                benefit: Some("When you cast animate dead or use the Command Undead feat, you are considered to be four levels higher when determining the number of Hit Dice you animate. When you cast command undead, your duration is doubled."),
                effect: Some(&["BONUS:VAR|CommandUndeadHD|4"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Spell Focus (Necromancy)", "PRESPELL:1,Animate Dead,Command Undead"]),
            },
            // Unsanctioned Detection -- um_feats.lst:153
            UmFeatEntry {
                key: "Unsanctioned Detection",
                category: FeatCategory::General,
                name: "Unsanctioned Detection",
                description: Some("You can focus your ability to detect evil for more practical or mundane purposes"),
                pretext: None,
                source_page: Some("p.158"),
                benefit: Some("As a swift action, you can focus the clarity granted by your detect evil ability to heighten your awareness of other things. This gives you a +10 sacred bonus on Perception and Sense Motive checks for one round. This expends your use of the detect evil class ability for the next 24 hours."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Paladin ~ Detect Evil"]),
            },
            // Unsanctioned Knowledge -- um_feats.lst:156
            UmFeatEntry {
                key: "Unsanctioned Knowledge",
                category: FeatCategory::General,
                name: "Unsanctioned Knowledge",
                description: Some("You have searched though forbidden texts and are privy to powerful but proscribed magic."),
                pretext: None,
                source_page: Some("p.159"),
                benefit: Some("Pick one 1st-level spell, one 2nd-level spell, one 3rd-level spell, and one 4th-level spell from the bard, cleric, inquisitor, or oracle spell lists. Add these spells to your paladin spell list as paladin spells of the appropriate level. Once chosen, these spells cannot be changed."),
                effect: Some(&["BONUS:ABILITYPOOL|Unsanctioned Knowledge ~ Level 1|1", "BONUS:ABILITYPOOL|Unsanctioned Knowledge ~ Level 2|1", "BONUS:ABILITYPOOL|Unsanctioned Knowledge ~ Level 3|1", "BONUS:ABILITYPOOL|Unsanctioned Knowledge ~ Level 4|1"]),
                prerequisites: Some(&["PRECLASS:1,Paladin=4", "PRESTAT:1,INT=13"]),
            },
            // Versatile Channeler -- um_feats.lst:164
            UmFeatEntry {
                key: "Versatile Channeler",
                category: FeatCategory::General,
                name: "Versatile Channeler",
                description: None,
                pretext: None,
                source_page: None,
                benefit: Some("You may choose to channel positive energy as if your effective cleric level were 2 levels lower than normal.|PREABILITY:1,CATEGORY=Special Ability,Versatile Channeler ~ Positive Energy"),
                effect: Some(&["BONUS:ABILITYPOOL|Versatile Channeler|1"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Channel Negative Energy,TYPE.Channel Positive Energy", "PREMULT:1,[PREMULT:2,[PREALIGN:LN,TN,CN],[PREMULT:1,[PREDEITYALIGN:LN,TN,CN],[PREDEITY:1,None]]],[PREVARGTEQ:NecromancySchoolLVL,1]"]),
            },
            // Vigilant Eidolon -- um_feats.lst:165
            UmFeatEntry {
                key: "Vigilant Eidolon",
                category: FeatCategory::General,
                name: "Vigilant Eidolon",
                description: Some("Your eidolon is highly observant, and its link with you increases your own watchfulness."),
                pretext: None,
                source_page: Some("p.159"),
                benefit: Some("While your eidolon is within your reach, you gain a +4 bonus on Perception checks. If you have 10 or more ranks in Perception, this bonus increases to +8. This does not apply if your eidolon is helpless or unconscious."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Eidolon"]),
            },
            // Voice of the Sibyl -- um_feats.lst:166
            UmFeatEntry {
                key: "Voice of the Sibyl",
                category: FeatCategory::General,
                name: "Voice of the Sibyl",
                description: Some("Your voice is strangely compelling."),
                pretext: None,
                source_page: Some("p.159"),
                benefit: Some("You get a +1 bonus on all Bluff, Diplomacy, and Perform (oratory) skill checks. If you have 10 or more ranks in one of these skills, the bonus increases to +3 for that skill. You do not get these bonuses if you do not use your voice when using the skill (such as using Bluff to feint in combat)."),
                effect: Some(&["BONUS:SITUATION|Bluff=Using voice|if(skillinfo(\"TOTALRANK\",\"Bluff\")>=10,3,1)", "BONUS:SITUATION|Diplomacy=Using voice|if(skillinfo(\"TOTALRANK\",\"Diplomacy\")>=10,3,1)", "BONUS:SKILL|Perform (Oratory)|if(skillinfo(\"TOTALRANK\",\"Perform (Oratory)\")>=10,3,1)"]),
                prerequisites: Some(&["PRESTAT:1,CHA=15"]),
            },
            // Warrior Priest -- um_feats.lst:167
            UmFeatEntry {
                key: "Warrior Priest",
                category: FeatCategory::General,
                name: "Warrior Priest",
                description: Some("Your religion is both a shield and a weapon in battle."),
                pretext: None,
                source_page: Some("p.159"),
                benefit: Some("You gain a +1 bonus on initiative checks and a +2 bonus on concentration checks made to cast a spell or use a spell-like ability when casting defensively or while grappled."),
                effect: Some(&["BONUS:COMBAT|INITIATIVE|1"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.CF_Domain", "PRESPELLCAST:TYPE=Divine"]),
            },
            // Wild Speech -- um_feats.lst:168
            UmFeatEntry {
                key: "Wild Speech",
                category: FeatCategory::General,
                name: "Wild Speech",
                description: Some("You speak with the tongue of men and beasts."),
                pretext: None,
                source_page: Some("p.159"),
                benefit: Some("When using wild shape to take the form in which you cannot speak (such as an animal), you are able to speak normally in any language you know. This allows you to cast spells with verbal components, speak command words, and activate spell completion and spell trigger items. However, it does not give you the ability to cast spells requiring somatic components unless you also have the Natural Spell feat, or cast spells with material components merged into your form."),
                effect: Some(&["BONUS:VAR|WildSpeechCasterLevel|DruidLVL", "BONUS:VAR|WildSpeechDuration|DruidLVL"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.DruidWildShape", "PREVARGTEQ:DruidLVL,6"]),
            },
            // Witch Knife -- um_feats.lst:170
            UmFeatEntry {
                key: "Witch Knife",
                category: FeatCategory::General,
                name: "Witch Knife",
                description: Some("You empower your witch spells by incorporating the use of a special ceremonial knife during your castings."),
                pretext: None,
                source_page: Some("p.159"),
                benefit: Some("Each day, when you prepare your spells, you can select a masterwork or magical dagger, transforming it into a witch knife, which serves as an additional focus component for witch patron spells. Add +1 to the DC of all your patron spells."),
                effect: None,
                prerequisites: Some(&["PRECLASS:1,Witch=1"]),
            },
            // Word of Healing -- um_feats.lst:171
            UmFeatEntry {
                key: "Word of Healing",
                category: FeatCategory::General,
                name: "Word of Healing",
                description: Some("Using the same divine energy as your lay on hands ability, you can heal others at a distance."),
                pretext: None,
                source_page: Some("p.159"),
                benefit: Some("You may use your lay on hands to heal another creature at a range of 30 feet as a standard action that does not provoke an attack of opportunity. You must be able to speak and have a free hand to use this ability. The target heals half the amount they would have healed if you had touched them, but gains the benefits of your mercies as normal."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.Lay On Hands"]),
            },
            // Masterpiece (At the Heart of It All) -- um_feats.lst:206
            UmFeatEntry {
                key: "Masterpiece (At the Heart of It All)",
                category: FeatCategory::Masterpiece,
                name: "Masterpiece (At the Heart of It All)",
                description: Some("You learn the masterpiece At the Heart of It All."),
                pretext: None,
                source_page: Some("p.21"),
                benefit: None,
                effect: None,
                prerequisites: Some(&["PRESKILL:1,Perform (String Instruments)=7,Perform (Wind Instruments)=7", "PREVARGTEQ:MasterpieceLVL,1"]),
            },
            // Masterpiece (Cat-Step) -- um_feats.lst:207
            UmFeatEntry {
                key: "Masterpiece (Cat-Step)",
                category: FeatCategory::Masterpiece,
                name: "Masterpiece (Cat-Step)",
                description: Some("You learn the masterpiece The Cat-Step."),
                pretext: None,
                source_page: Some("p.21"),
                benefit: None,
                effect: None,
                prerequisites: Some(&["PRESKILL:1,Perform (Dance)=5", "PREVARGTEQ:MasterpieceLVL,1"]),
            },
            // Masterpiece (Dance of 23 Steps) -- um_feats.lst:208
            UmFeatEntry {
                key: "Masterpiece (Dance of 23 Steps)",
                category: FeatCategory::Masterpiece,
                name: "Masterpiece (Dance of 23 Steps)",
                description: Some("You learn the masterpiece The Dance of 23 Steps."),
                pretext: None,
                source_page: Some("p.21"),
                benefit: None,
                effect: None,
                prerequisites: Some(&["PRESKILL:1,Perform (Dance)=4", "PREVARGTEQ:MasterpieceLVL,1"]),
            },
            // Masterpiece (Depths of the Mountain) -- um_feats.lst:209
            UmFeatEntry {
                key: "Masterpiece (Depths of the Mountain)",
                category: FeatCategory::Masterpiece,
                name: "Masterpiece (Depths of the Mountain)",
                description: Some("You learn the masterpiece The Depths of the Mountain."),
                pretext: None,
                source_page: Some("p.21"),
                benefit: None,
                effect: None,
                prerequisites: Some(&["PRESKILL:1,Perform (Percussion Instruments)=15,Perform (Wind Instruments)=15", "PREVARGTEQ:MasterpieceLVL,1"]),
            },
            // Masterpiece (Dumbshow of Gorroc) -- um_feats.lst:210
            UmFeatEntry {
                key: "Masterpiece (Dumbshow of Gorroc)",
                category: FeatCategory::Masterpiece,
                name: "Masterpiece (Dumbshow of Gorroc)",
                description: Some("You learn the masterpiece The Dumbshow of Gorroc."),
                pretext: None,
                source_page: Some("p.21"),
                benefit: None,
                effect: None,
                prerequisites: Some(&["PRESKILL:1,Perform (Act)=6,Perform (Comedy)=6", "PREVARGTEQ:MasterpieceLVL,1"]),
            },
            // Masterpiece (House of Imaginary Walls) -- um_feats.lst:211
            UmFeatEntry {
                key: "Masterpiece (House of Imaginary Walls)",
                category: FeatCategory::Masterpiece,
                name: "Masterpiece (House of Imaginary Walls)",
                description: Some("You learn the masterpiece The House of Imaginary Walls."),
                pretext: None,
                source_page: Some("p.21"),
                benefit: None,
                effect: None,
                prerequisites: Some(&["PRESKILL:1,Perform (Act)=10", "PREVARGTEQ:MasterpieceLVL,1"]),
            },
            // Masterpiece (Legato Piece on the Infernal Bargain) -- um_feats.lst:212
            UmFeatEntry {
                key: "Masterpiece (Legato Piece on the Infernal Bargain)",
                category: FeatCategory::Masterpiece,
                name: "Masterpiece (Legato Piece on the Infernal Bargain)",
                description: Some("You learn the masterpiece Legato Piece on the Infernal Bargain."),
                pretext: None,
                source_page: Some("p.21"),
                benefit: None,
                effect: None,
                prerequisites: Some(&["PRESKILL:1,Perform (String Instruments)=11", "PREVARGTEQ:MasterpieceLVL,1"]),
            },
            // Masterpiece (Lullaby of Ember the Ancient) -- um_feats.lst:213
            UmFeatEntry {
                key: "Masterpiece (Lullaby of Ember the Ancient)",
                category: FeatCategory::Masterpiece,
                name: "Masterpiece (Lullaby of Ember the Ancient)",
                description: Some("You learn the masterpiece The Lullaby of Ember the Ancient."),
                pretext: None,
                source_page: Some("p.21"),
                benefit: None,
                effect: None,
                prerequisites: Some(&["PRESKILL:1,Perform (Sing)=7", "PREVARGTEQ:MasterpieceLVL,1"]),
            },
            // Masterpiece (Minuet of the Midnight Ivy) -- um_feats.lst:214
            UmFeatEntry {
                key: "Masterpiece (Minuet of the Midnight Ivy)",
                category: FeatCategory::Masterpiece,
                name: "Masterpiece (Minuet of the Midnight Ivy)",
                description: Some("You learn the masterpiece Minuet of the Midnight Ivy."),
                pretext: None,
                source_page: Some("p.21"),
                benefit: None,
                effect: None,
                prerequisites: Some(&["PRESKILL:1,Perform (Dance)=4", "PREVARGTEQ:MasterpieceLVL,1"]),
            },
            // Masterpiece (Quickening Pulse) -- um_feats.lst:215
            UmFeatEntry {
                key: "Masterpiece (Quickening Pulse)",
                category: FeatCategory::Masterpiece,
                name: "Masterpiece (Quickening Pulse)",
                description: Some("You learn the masterpiece The Quickening Pulse."),
                pretext: None,
                source_page: Some("p.21"),
                benefit: None,
                effect: None,
                prerequisites: Some(&["PRESKILL:1,Perform (Percussion Instruments)=7,Perform (Wind Instruments)=7", "PREVARGTEQ:MasterpieceLVL,1"]),
            },
            // Masterpiece (Requiem of the Fallen Priest-King) -- um_feats.lst:216
            UmFeatEntry {
                key: "Masterpiece (Requiem of the Fallen Priest-King)",
                category: FeatCategory::Masterpiece,
                name: "Masterpiece (Requiem of the Fallen Priest-King)",
                description: Some("You learn the masterpiece The Requiem of the Fallen Priest-King."),
                pretext: None,
                source_page: Some("p.21"),
                benefit: None,
                effect: None,
                prerequisites: Some(&["PRESKILL:1,Perform (Oratory)=10,Perform (Sing)=10", "PREVARGTEQ:MasterpieceLVL,1"]),
            },
            // Masterpiece (Stone Face) -- um_feats.lst:217
            UmFeatEntry {
                key: "Masterpiece (Stone Face)",
                category: FeatCategory::Masterpiece,
                name: "Masterpiece (Stone Face)",
                description: Some("You learn the masterpiece Stone Face."),
                pretext: None,
                source_page: Some("p.21"),
                benefit: None,
                effect: None,
                prerequisites: Some(&["PRESKILL:1,Perform (Comedy)=7,Perform (Oratory)=7", "PREVARGTEQ:MasterpieceLVL,1"]),
            },
            // Masterpiece (Toccata and Fugue of the Danse Macabre) -- um_feats.lst:218
            UmFeatEntry {
                key: "Masterpiece (Toccata and Fugue of the Danse Macabre)",
                category: FeatCategory::Masterpiece,
                name: "Masterpiece (Toccata and Fugue of the Danse Macabre)",
                description: Some("You learn the masterpiece Toccata and Fugue of the Danse Macabre."),
                pretext: None,
                source_page: Some("p.21"),
                benefit: None,
                effect: None,
                prerequisites: Some(&["PRESKILL:1,Perform (Keyboard Instruments)=4,Perform (Wind Instruments)=4", "PREVARGTEQ:MasterpieceLVL,1"]),
            },
            // Masterpiece (Triple Time) -- um_feats.lst:219
            UmFeatEntry {
                key: "Masterpiece (Triple Time)",
                category: FeatCategory::Masterpiece,
                name: "Masterpiece (Triple Time)",
                description: Some("You learn the masterpiece Triple Time."),
                pretext: None,
                source_page: Some("p.21"),
                benefit: None,
                effect: None,
                prerequisites: Some(&["PRESKILL:1,Perform (Percussion Instruments)=3,Perform (String Instruments)=3,Perform (Wind Instruments)=3", "PREVARGTEQ:MasterpieceLVL,1"]),
            },
            // Masterpiece (Winds of the Five Heavens) -- um_feats.lst:220
            UmFeatEntry {
                key: "Masterpiece (Winds of the Five Heavens)",
                category: FeatCategory::Masterpiece,
                name: "Masterpiece (Winds of the Five Heavens)",
                description: Some("You learn the masterpiece The Winds of the Five Heavens."),
                pretext: None,
                source_page: Some("p.21"),
                benefit: None,
                effect: None,
                prerequisites: Some(&["PRESKILL:1,Perform (Act)=10,Perform (Oratory)=10", "PREVARGTEQ:MasterpieceLVL,1"]),
            },
            // Transfer Feat to Familiar -- um_feats.lst:226
            UmFeatEntry {
                key: "Transfer Feat to Familiar",
                category: FeatCategory::General,
                name: "Transfer Feat to Familiar",
                description: None,
                pretext: None,
                source_page: None,
                benefit: None,
                effect: Some(&["BONUS:VAR|BeastBondedFeatTransfer|1"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Transfer Feats ~ Beast-Bonded", "PRECLASS:1,Witch=1"]),
            },
            // Discovery (Arcane Builder) -- um_feats.lst:231
            UmFeatEntry {
                key: "Discovery (Arcane Builder)",
                category: FeatCategory::Discovery,
                name: "Discovery (Arcane Builder)",
                description: Some("You have an exceptional understanding of the theory behind creating magical items."),
                pretext: None,
                source_page: Some("p.86"),
                benefit: Some("You have an exceptional understanding of the theory behind creating magical items. Select one type of magic item (potions, wondrous items, and so on). You create items of this type 25%% faster than normal, and gain a +4 bonus on Spellcraft checks (or other checks, as appropriate) to craft items of this type."),
                effect: None,
                prerequisites: Some(&["PREVARGTEQ:FeatQualifier_WizardLVL,1"]),
            },
            // Discovery (Fast Study) -- um_feats.lst:232
            UmFeatEntry {
                key: "Discovery (Fast Study)",
                category: FeatCategory::Discovery,
                name: "Discovery (Fast Study)",
                description: Some("You can prepare all your spells in 15 minutes."),
                pretext: None,
                source_page: Some("p.86"),
                benefit: Some("Normally, a wizard spends 1 hour preparing all of his spells for the day, or proportionately less if he only prepares some spells, with a minimum of 15 minutes of preparation. Thanks to mental discipline and clever mnemonics, you can prepare all of your spells in only 15 minutes, and your minimum preparation time is only 1 minute."),
                effect: None,
                prerequisites: Some(&["PREVARGTEQ:FeatQualifier_WizardLVL,5"]),
            },
            // Discovery (Feral Speech) -- um_feats.lst:233
            UmFeatEntry {
                key: "Discovery (Feral Speech)",
                category: FeatCategory::Discovery,
                name: "Discovery (Feral Speech)",
                description: Some("You gain the ability to speak with animals."),
                pretext: None,
                source_page: Some("p.86"),
                benefit: Some("You gain the ability to speak with and understand the response of any animal as if using speak with animals, though each time you speak to animals, you must decide to communicate with either amphibians, birds, fish, mammals"),
                effect: None,
                prerequisites: Some(&["PREVARGTEQ:FeatQualifier_WizardLVL,5"]),
            },
            // Discovery (Golem Constructor) -- um_feats.lst:234
            UmFeatEntry {
                key: "Discovery (Golem Constructor)",
                category: FeatCategory::Discovery,
                name: "Discovery (Golem Constructor)",
                description: Some("You have learned the art and craft of constructing a single type of golem."),
                pretext: None,
                source_page: Some("p.86"),
                benefit: Some("You have learned the art and craft of creating a single type of golem (such as stone golems or iron golems). When creating a golem of this type, you count as having the Craft Wondrous Item, Craft Magic Arms and Armor, and Craft Construct feats. You must meet all other construction requirements for the golem as normal."),
                effect: None,
                prerequisites: Some(&["PREVARGTEQ:FeatQualifier_WizardLVL,9"]),
            },
            // Discovery (Immortality) -- um_feats.lst:235
            UmFeatEntry {
                key: "Discovery (Immortality)",
                category: FeatCategory::Discovery,
                name: "Discovery (Immortality)",
                description: Some("You discover a cure for aging."),
                pretext: None,
                source_page: Some("p.86"),
                benefit: Some("You discover a cure for aging, and from this point forward you take no penalty to your physical ability scores from advanced age. If you are already taking such penalties, they are removed at this time."),
                effect: None,
                prerequisites: Some(&["PREVARGTEQ:FeatQualifier_WizardLVL,20"]),
            },
            // Discovery (Multimorph) -- um_feats.lst:236
            UmFeatEntry {
                key: "Discovery (Multimorph)",
                category: FeatCategory::Discovery,
                name: "Discovery (Multimorph)",
                description: Some("Your studies in transmogrification have increased your control over shapechanging spells."),
                pretext: None,
                source_page: Some("p.86"),
                benefit: Some("Your studies in transmogrification have increased your control over shapechanging spells. When you cast a spell of the polymorph subschool on yourself, you may expend 1 minute of the spell's duration as a standard action to assume another form allowed by the spell. You can do this as often as you like, subject to the duration of the spell."),
                effect: None,
                prerequisites: Some(&["PREVARGTEQ:FeatQualifier_WizardLVL,5"]),
            },
            // Discovery (Opposition Research) -- um_feats.lst:237
            UmFeatEntry {
                key: "Discovery (Opposition Research)",
                category: FeatCategory::Discovery,
                name: "Discovery (Opposition Research)",
                description: Some("You have broken through the mental barriers that made it hard for you to prepare spells from one of your opposition schools."),
                pretext: None,
                source_page: Some("p.86"),
                benefit: Some("By completing strenuous studies, you have broken through the mental barriers that made it hard for you to prepare spells from one of your opposition schools. Select one wizard opposition school; preparing spells of this school now only requires one spell slot of the appropriate level instead of two, and you no longer have the -4 Spellcraft penalty for crafting items from that school."),
                effect: Some(&["BONUS:VAR|Arcane Opposition School|-1"]),
                prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,TYPE.ArcaneSchool", "PREVARGTEQ:FeatQualifier_WizardLVL,9"]),
            },
            // Discovery (Split Slot) -- um_feats.lst:238
            UmFeatEntry {
                key: "Discovery (Split Slot)",
                category: FeatCategory::Discovery,
                name: "Discovery (Split Slot)",
                description: Some("You may treat an open spell slot as if it were two spell slots that were two levels lower."),
                pretext: None,
                source_page: Some("p.86"),
                benefit: Some("Once per day, when you prepare spells, you may treat any one of your open spell slots as if it were two spell slots that were two spell levels lower. For example, a 9th-level wizard can split a 5th-level slot into two 3rd-level slots, preparing fireball and lightning bolt in those 3rd-level slots. For all purposes, the two lower-level slots are treated as that lower level (so the split 5th-level slot used for a fireball has a DC as if it were in a normal 3rd-level slot). Splitting a 2nd-level slot lets you prepare two additional cantrips (which you can cast over and over, just like normally prepared cantrips). This discovery has no effect on cantrips or 1st-level spells. You may split %1 spell slots when you prepare spells.|SplitSlotTimes"),
                effect: Some(&["BONUS:ABILITYPOOL|Split Slot|1", "BONUS:VAR|SplitSlotTimes|1"]),
                prerequisites: Some(&["PREVARGTEQ:FeatQualifier_WizardLVL,5"]),
            },
            // Discovery (True Name) -- um_feats.lst:239
            UmFeatEntry {
                key: "Discovery (True Name)",
                category: FeatCategory::Discovery,
                name: "Discovery (True Name)",
                description: Some("You have discovered the true name of an outsider."),
                pretext: None,
                source_page: Some("p.87"),
                benefit: Some("Your researches into ancient tomes and your inquisitions of bound spirits have led you to one of the best-hidden secrets of the multiverse: the true name of an outsider-the name that defines the very essence of the creature and that gives the speaker control over the being. This outsider can have no more than 12 Hit Dice. Once per day, you can speak the common name by which the outsider is known, and the outsider travels to you as if you had cast planar binding upon it. It must obey you to the best of its ability, without pay or bargaining for its services, for its fear that you might release its true name to the wider world is enough to bring even the most recalcitrant of outsiders to bear. If the creature is within 100 feet, as a move action, you may punish it by deliberately mispronouncing its name, wracking its very essence and giving it the sickened and staggered conditions for 1 round (even if the creature is normally immune to these conditions). You cannot use true name in an area of silence, but the creature does not have to be able to hear you for it to be harmed by the ability. It is in your best interest to call this creature only sparingly, and occasionally reward it in some fashion to mollify its wrath. If you repeatedly fail to offer it a reward appropriate to its type and ethos, the creature may begin plotting ways to destroy the bond between you, whether by creating an accident that will destroy your memory of the name, by plaguing you with nuisances or dangers until you vow never to call on it again, or by actively seeking to destroy you through its own devices or those of an underling. If this creature is of a lawful type and you are violating its ethos, its superiors may even destroy it or you rather than allow you to contaminate their servant further. Worse, they may establish situations where it is necessary for you to summon this outsider, opening gateways to infernal or angelic interference, in order to gain a foothold on the Material Plane."),
                effect: None,
                prerequisites: Some(&["PREVARGTEQ:FeatQualifier_WizardLVL,11"]),
            },
            // Discovery (Greater True Name) -- um_feats.lst:240
            UmFeatEntry {
                key: "Discovery (Greater True Name)",
                category: FeatCategory::Discovery,
                name: "Discovery (Greater True Name)",
                description: Some("You have discovered the true name of a powerful outsider."),
                pretext: None,
                source_page: Some("p.87"),
                benefit: Some("Your researches into ancient tomes and your inquisitions of bound spirits have led you to one of the best-hidden secrets of the multiverse: the true name of an outsider-the name that defines the very essence of the creature and that gives the speaker control over the being. This outsider can have no more than 18 Hit Dice. Once per day, you can speak the common name by which the outsider is known, and the outsider travels to you as if you had cast greater planar binding upon it. It must obey you to the best of its ability, without pay or bargaining for its services, for its fear that you might release its true name to the wider world is enough to bring even the most recalcitrant of outsiders to bear. If the creature is within 100 feet, as a move action, you may punish it by deliberately mispronouncing its name, wracking its very essence and giving it the sickened and staggered conditions for 1 round (even if the creature is normally immune to these conditions). You cannot use true name in an area of silence, but the creature does not have to be able to hear you for it to be harmed by the ability. It is in your best interest to call this creature only sparingly, and occasionally reward it in some fashion to mollify its wrath. If you repeatedly fail to offer it a reward appropriate to its type and ethos, the creature may begin plotting ways to destroy the bond between you, whether by creating an accident that will destroy your memory of the name, by plaguing you with nuisances or dangers until you vow never to call on it again, or by actively seeking to destroy you through its own devices or those of an underling. If this creature is of a lawful type and you are violating its ethos, its superiors may even destroy it or you rather than allow you to contaminate their servant further. Worse, they may establish situations where it is necessary for you to summon this outsider, opening gateways to infernal or angelic interference, in order to gain a foothold on the Material Plane."),
                effect: None,
                prerequisites: Some(&["PREVARGTEQ:FeatQualifier_WizardLVL,15"]),
            },
            // Discovery (Staff-Like Wand) -- um_feats.lst:241
            UmFeatEntry {
                key: "Discovery (Staff-Like Wand)",
                category: FeatCategory::Discovery,
                name: "Discovery (Staff-Like Wand)",
                description: Some("You use your own power when using a wand."),
                pretext: None,
                source_page: Some("p.87"),
                benefit: Some("Your research has unlocked a new power in conjunction with using a wand. Similar to using a magic staff, you use your own Intelligence score and relevant feats to set the DC for saves against spells you cast from a wand, and you can use your caster level when activating the power of a wand if it's higher than the caster level of the wand."),
                effect: None,
                prerequisites: Some(&["PREABILITY:1,CATEGORY=FEAT,Craft Staff", "PREVARGTEQ:FeatQualifier_WizardLVL,11"]),
            },        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_has_144_records() {
        assert_eq!(feat_tables().len(), 144);
    }

    #[test]
    fn every_record_carries_real_content() {
        for e in feat_tables() {
            assert!(
                e.description.is_some() || e.benefit.is_some() || e.effect.is_some(),
                "{} has no DESC:, BENEFIT:, or BONUS: -- genuinely empty, should have been excluded",
                e.key
            );
        }
    }

    #[test]
    fn no_record_is_deferred() {
        assert_eq!(
            feat_tables()
                .iter()
                .filter(|e| e.description.is_none() && e.benefit.is_none() && e.effect.is_none())
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
    fn the_desc_benefit_effect_split_is_the_real_one() {
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
        let effect_only = feat_tables()
            .iter()
            .filter(|e| e.effect.is_some() && e.description.is_none() && e.benefit.is_none())
            .count();
        assert_eq!(both, 123, "records with both DESC: and BENEFIT:");
        assert_eq!(desc_only, 15, "the 15 Masterpiece feats, DESC:-complete by design");
        assert_eq!(benefit_only, 2, "Greater Wild Empathy, Versatile Channeler");
        assert_eq!(effect_only, 4, "Extra Cantrips or Orisons, Extra Evolution, Extra Summons, Transfer Feat to Familiar");
        assert_eq!(both + desc_only + benefit_only + effect_only, 144);
    }
}
