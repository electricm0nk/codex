//! Ultimate Psionics (UPsi) archetype-swap catalog. SD28-E30 slice 1 --
//! `epic-32-archetype-swap`'s own proof table, the smallest in-scope
//! book (15 records), landed before any larger table to validate the
//! record shape and struct on real data first.
//!
//! **This mechanism's data is cross-cutting: it belongs to no single
//! Ultimate book.** Its own corpus records span at least 9 books
//! (`decisions.md`'s own epic-30 scoping note), UI itself contributes
//! zero, and out-of-SD-28 books (`inner_sea_magic`, `inner_sea_combat`)
//! get their own separate, SD-30-owned tables rather than being folded
//! in here.
//!
//! **Record shape is two-tier, confirmed against this book's own raw
//! corpus rows, not assumed from one example.** Each archetype is:
//! - one **master/selection** row (`CATEGORY:Archetype`, `KEY:<Class>
//!   Archetype ~ <ArchetypeName>`), carrying the archetype's own flavour
//!   `DESC:`, a `TYPE:Archetype.<Class>Archetype.<SlotId>...` facet
//!   naming the base-class feature slots it *replaces*, and one or more
//!   `ABILITY:<Category>|<GrantType>|<Name1>[|<Name2>...][|<LevelGate>]`
//!   tokens naming what it *grants*; plus
//! - one **named sub-feature** row per grant (`CATEGORY:Special
//!   Ability`, `KEY:<ArchetypeName> ~ <FeatureName>`), carrying the real
//!   mechanical `DESC:`/`BONUS:` text for that specific swapped-in
//!   feature.
//!
//! **The full `ABILITY:` grant grammar, enumerated exhaustively across
//! all three landed books before this table's second correction pass
//! (`decisions.md §51` addendum 3), not patched shape-by-shape as new
//! ones turned up.** Two axes, each ruled explicitly:
//! - **Level-gate shape:** `PRECLASS:1,<Class>=<Level>` (dominant),
//!   `PREVARGTEQ:<Class>LVL,<Level>` (a real minority), or none
//!   (implicit level 1). A single token can also name more than one
//!   granted feature (e.g. `Cave Druid`'s own token granting `Cavesense`/
//!   `Nature Bond`/`Wild Empathy` together, all implicit level 1) --
//!   both shapes handled.
//! - **Grant category/type, ruled per family, not defaulted to
//!   "include everything":** `<Class> Class Feature` and `Special
//!   Ability` (same `<Archetype> ~ <Feature>` naming, same real
//!   content) -- **included**. `Internal` -- **excluded**: checked one
//!   directly (`Armor Aptitude 7th Level`, `up_abilities_class.lst:2502`,
//!   `CATEGORY:Internal|UNENCUMBEREDMOVE:HeavyArmor`) and confirmed it
//!   is engine bookkeeping with no player-facing text, the same shape a
//!   feat catalog's own auto-grant-wrapper exclusion already covers, not
//!   real granted content. Grant type `NORMAL` (e.g. `Divine Bond`, a
//!   player-*chosen* grant, not an automatic archetype swap) --
//!   **excluded**, distinct from `AUTOMATIC`. `FEAT`-categorized grants
//!   (one instance, cross-book) -- **included**, real content, just
//!   pointing at a feat rather than a class-feature row.
//!
//! **This table's `grants` field is a floor, bounded below by two named,
//! counted populations neither this table nor its siblings close.**
//! First: the tier-2 sub-feature population (`decisions.md §51`'s own
//! earlier addendum), 4,550 rows program-wide, sized as a floor itself.
//! Second, found while tracing this grammar: a grant can live on a row
//! other than the archetype's own master row entirely -- confirmed on
//! APG's `Druid Archetype ~ Cave Druid`, whose `Druid Domain` grant
//! lives on `CATEGORY=Archetype|Nature's Bond ~ Druid Domain.MOD` (a
//! `.MOD` row modifying an unrelated, pre-existing feature), gated by
//! `PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Cave Druid`. A
//! parser scanning only the archetype's own row cannot find this class
//! of grant. **Sized, not left unsized:** a corpus-wide scan for every
//! `.MOD` row carrying `PREABILITY:...,CATEGORY=Archetype,...` anywhere
//! in any book finds **1,282 rows total** (ACG 251, APG 231, CRB 199,
//! UC 147, UM 129, ARG 72, UPsi 23, PU 21, OA 18, PsiX 16, UE 11, CE 11,
//! HA 8, AG 2, UI 1, UW 1 -- `decisions.md §51`'s own addendum records
//! the full table and the discovery command). This book's own separate,
//! narrower scan (rows anywhere in the file gated specifically on one of
//! these 15 archetypes, 18 rows found) turned up no `.MOD`-injected
//! instance in this particular book -- but the mechanism itself is real
//! and confirmed present in APG, and the corpus-wide 23-row UPsi share
//! of the 1,282 total suggests this book's own population is not fully
//! zero, so a table's own `grants` field never closes this population,
//! only bounds it.
//!
//! **The `TYPE:` slot list and the `ABILITY:` grant list are NOT two
//! views of one list -- confirmed by counting both across all 15
//! records, not assumed from the first one.** `TYPE:` names what the
//! archetype *takes away* (68 slots total, this book); `ABILITY:` names
//! what it *gives* (75 grants total, after the category ruling above).
//! 5 of 15 records have equal counts (33%). `replaces` and `grants` are
//! kept as two separate lists on `ArchetypeSwapEntry` for exactly this
//! reason -- pairing them positionally would fabricate a correspondence
//! the corpus does not state.
//!
//! **65 of 75 sub-feature grants (87%) resolved to a real named row with
//! real `DESC:`/`BENEFIT:` text.** The remainder split two ways: some
//! `KEY:` lookups found no row at all, some resolved rows carry neither
//! token. Named individually in the generated tests rather than only
//! counted in aggregate.
//!
//! **The `§46`/`§48`/`§49` text-shape triad, run against this book's own
//! archetype `.MOD` rows before trusting any description as complete.**
//! `CATEGORY=Archetype|Barbarian Archetype ~ Raging Beast.MOD`
//! (`up_abilities_class.lst:2406`) carries **no `DESC:`/`BENEFIT:` at
//! all** -- it is a pure `FACT:<ClassName>_CF_<Slot>|true` flag-setter
//! row, the mechanism the base class's own feature code is expected to
//! check to know it has been swapped out (this is very likely the real
//! referent behind `pilot_compute.rs`'s many "archetype-suppression
//! flag, provably vacuous" comments -- confirmed structurally, not yet
//! wired to any compute code by this slice). None of the three triad
//! hazards apply: no prose to recover unconditionally, no `PRE`-gated
//! conditional variant, and nothing to accidentally join as raw syntax,
//! since there is no prose on this row at all.
//!
//! **This table is data only.** No `pilot_compute.rs` integration lands
//! in this slice -- see `decisions.md`'s own epic-30 forward-scope-
//! register entry for why that half is blocked on an explicit scope
//! decision, not landed here.
//!
//! Every field below is copied verbatim from the real corpus row (source:
//! `~/workspace/repos/pcgen/data/pathfinder/dreamscarred_press/
//! ultimate_psionics/up_abilities_class.lst`), generated programmatically
//! by a one-off extraction script, not hand-transcribed.

use super::super::archetype_swap::{ArchetypeGrant, ArchetypeSwapEntry};

/// Full UPsi archetype-swap catalog: 15 real, distinct master records
/// (Barbarian/Bard/Druid/Fighter/Monk/Paladin/Ranger/Rogue archetypes),
/// in source order. Built once and cached for the process lifetime.
pub fn archetype_swap_tables() -> &'static [ArchetypeSwapEntry] {
    static TABLE: std::sync::OnceLock<Vec<ArchetypeSwapEntry>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        vec![
        // Barbarian Archetype ~ Raging Beast -- up_abilities_class.lst:2403
        ArchetypeSwapEntry {
            key: "Barbarian Archetype ~ Raging Beast",
            subject: "Barbarian",
            archetype_name: "Raging Beast",
            description: Some("You have unlocked your psionic potential and use the power of your rage to transform into a terrifying beast capable of decimating your foes."),
            source_page: Some("p.296"),
            prerequisites: Some(&["PRECLASS:1,Barbarian=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Barbarian Archetype ~ Raging Beast],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BarbarianFastMovement,TYPE.BarbarianRagePowers,TYPE.BarbarianRagePower2,TYPE.BarbarianRagePower6,TYPE.BarbarianRagePower8,TYPE.BarbarianRagePower14,TYPE.BarbarianRagePower20,TYPE.BarbarianTrapSense,TYPE.BarbarianTrapSense3,TYPE.BarbarianTrapSense4,TYPE.BarbarianTrapSense5,TYPE.BarbarianTrapSense6]"]),
            replaces: Some(&["BarbarianFastMovement", "BarbarianRagePower2", "BarbarianRagePower6", "BarbarianRagePower8", "BarbarianRagePower14", "BarbarianRagePower20", "BarbarianTrapSense3", "BarbarianTrapSense4", "BarbarianTrapSense5", "BarbarianTrapSense6"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Raging Beast ~ Psionic Mind", at_level: 1, description: Some("You gain wild talent or psionic talent as a bonus feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Raging Beast ~ Rage", at_level: 1, description: Some("You can expend your psionic focus when you rage to manifest claws of the beast (ML %1) as a move action.|RagingBeastClawsManifesterLVL"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Raging Beast ~ Raging Beast Manifesting", at_level: 2, description: Some("Psychic Warrior Powers Known: %1; Psychic Warrior Maximum Power Level Known: %2|PsychicWarriorPowersKnown|PsychicWarriorMaxPowerLevel"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Raging Beast ~ Beastly Bite", at_level: 6, description: Some("When you expend your psionic focus to manifest claws of the beast, you can manifest bite of the wolf (ML %1) in the same action.|RagingBeastClawsManifesterLVL"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Raging Beast ~ Toughened Rager", at_level: 9, description: Some("You gain a +%1 enhancement bonus to your natural armor as long as you are raging.|ToughenedRagerBonus"), benefit: None },
            ],
        },
        // Bard Archetype ~ Thoughtsinger -- up_abilities_class.lst:2424
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Thoughtsinger",
            subject: "Bard",
            archetype_name: "Thoughtsinger",
            description: Some("You bring your friends together into a chorus of thoughts and emotions."),
            source_page: Some("p.296"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Thoughtsinger],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardBardicPerformance,TYPE.BardWellVersed,TYPE.BardVersatilePerformance,TYPE.BardSuggestion,TYPE.BardMassSuggestion,TYPE.BardDirgeOfDoom,TYPE.BardFrighteningTune,TYPE.BardDeadlyPerformance]"]),
            replaces: Some(&["BardBardicPerformance", "BardWellVersed", "BardVersatilePerformance", "BardSuggestion", "BardMassSuggestion", "BardDirgeOfDoom", "BardFrighteningTune", "BardDeadlyPerformance"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Thoughtsinger ~ Collective", at_level: 1, description: Some("Join %1 minds, plus your own, within %2 feet; can manifest some powers through collective.|ThoughtsingerCollectiveMinds|ThoughtsingerCollectiveRange|!PREABILITY:1,CATEGORY=Internal,Thoughtsinger ~ Collective Range Unlimited"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Thoughtsinger ~ Thoughtsong", at_level: 1, description: Some("For %1 rounds/day, you can use the thoughtsong, activating it as a|ThoughtsongRounds"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Thoughtsinger ~ Music of the Spheres", at_level: 2, description: Some("As long as you have an active collective, you gain a +2 bonus against all mind-affecting powers, abilities, and spells."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Thoughtsinger ~ Telepathy", at_level: 2, description: Some("All willing members of your collective can communicate with each other telepathically.  Psionic creatures who are willing members may manifest unknown powers known by another willing psionic creature in the collective as if they were making physical contact."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Thoughtsinger ~ Compelling Voice", at_level: 6, description: Some("You can spend a standard action to manifest compelling voice (ML %1) on a mindlinked target.|CompellingVoiceML"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Thoughtsinger ~ Emotional Shield", at_level: 8, description: Some("You can set up this shield with an immediate action and one round of thoughtsong.  Each round, all characters in the collective receive PR equal to your Perform (thoughtsong) check - 10 for purposes of resisting mind-affecting powers.  If you take any other action whie maintaining the shield, you take a -5 penalty to your Perform check."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Thoughtsinger ~ Fear Cascade", at_level: 14, description: Some("You can spend an additional round of thoughtsong to manifest fear cascade (ML %1) on a mindlinked target.|FearCascadeML"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Thoughtsinger ~ The Becoming", at_level: 20, description: Some("While mindlinked to a target, as a full-round action, you can expend one round of thoughtsong to manifest mind seed (ML %1, Will DC %2) as a psi-like ability with a range of 30 feet.|TheBecomingML|TheBecomingDC"), benefit: None },
            ],
        },
        // Druid Archetype ~ Gaean -- up_abilities_class.lst:2455
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Gaean",
            subject: "Druid",
            archetype_name: "Gaean",
            description: Some("You have learned that all of nature is connected by a vast earthmind.  You tap into this primal mind to fuel your energies."),
            source_page: Some("p.292"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Gaean],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidWildEmpathy,TYPE.DruidResistNaturesLure,TYPE.DruidThousandFaces]"]),
            replaces: Some(&["DruidWildEmpathy", "DruidResistNaturesLure", "DruidAThousandFaces"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Gaean ~ Gaean Communion", at_level: 1, description: Some("While maintaining psionic focus, you gain the ability to telepathically communicate with any animal or magical beast with an intelligence of 1 or 2 within 100 ft.  Any time you cast summon nature's ally, you can communicate telepathically with all of the creatures summoned by the spell."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Gaean ~ Strength of Gaea", at_level: 4, description: Some("While you are in physical contact with nature, %1/day you can augment your spells in one of the following ways. 1 power point and expend focus: double duration; 1 power point and expend focus: double range; 1 power point and expend focus: no verbal component; 2 power points and expend focus: no somatic component; 2 power points and expend focus: empower spell|StrengthOfGaeaTimes"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Gaean ~ Gaean Revivification", at_level: 13, description: Some("If you are maintaining focus when reduced to 0 or fewer hit points and are in physical contact with nature, you can choose as a free action to submerge into nature.  While submerged, you cannot be targeted by attacks or effects, you immediately stop bleeding and gain fast healing 5.  You are ejected once you return to positive hit points, although you can choose to stay submerged by expending spell slots; one extra round per spell level."), benefit: None },
            ],
        },
        // Druid Archetype ~ Serpent Lord -- up_abilities_class.lst:2457
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Serpent Lord",
            subject: "Druid",
            archetype_name: "Serpent Lord",
            description: Some("You embrace the inner serpent and bring it forth."),
            source_page: Some("p.337"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Serpent Lord],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidNatureBond,TYPE.DruidNaturesLure,TYPE.DruidThousandFaces,TYPE.DruidVenomImmunity,TYPE.DruidWildShape,TYPE.DruidWildEmpathy]", "PRERACE:1,Ophiduan"]),
            replaces: Some(&["DruidNatureBond", "DruidResistNaturesLure", "DruidAThousandFaces", "DruidVenomImmunity"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Serpent Lord ~ Nature Bond", at_level: 1, description: Some("You must choose a serpend or snake as your animal companion.  [Not enforced]  The DC of any poison attacks your animal companion has increases by +2. [Not added in]"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Serpent Lord ~ Wild Empathy", at_level: 1, description: Some("You gain a +4 bonus when using wild empathy on serpents and the time it takes is reduced to one round."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Serpent Lord ~ Serpentine Transformation", at_level: 2, description: Some("You may adopt an aspect of the snake while retaining your normal form.  You gain one of the following abilities: Movement (climb speed 20 feet, swim speed 20 feet), Senses (low-light vision, scent), Scales (+2 natural armor bonus to AC), Bite attack (bite [1d4], poison [frequency 1 round (6), effect 1 Con damage, Cure 1 save, DC %1] for a Medium druid, +2 to CMB on grapple checks).  In addition, if activating the bite attack ability, you can expend a use of your racial serpent's bite ability to instead use the bite damage for the racial ability and increase the DC of any poison attacks gained from serpentine transformation by +2.  Whie using serpentine transformation, you may speak normally and can cast speak with animals (reptiles only) at will.  Using this ability is a"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Serpent Lord ~ Venom Resistance", at_level: 4, description: Some("You gain a +4 bonus on saves against poison."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Serpent Lord ~ Serpentine Summons", at_level: 5, description: Some("You may cast summon nature's ally as a standard action when summoning snakes, and the poison DC of any snakes summoned increases by +2.  You can apply the young template to any snake to reduce the level of the summoning spell required by one.  You can also increase the level of summoning required by one in order to apply either the advanced or the giant template, or increase it by two to apply both the advanced and giant templates."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Serpent Lord ~ Wild Shape", at_level: 5, description: Some("Your druid level is treated as two higher if you take the form of a snake, while your druid level is treated as two lower if you take on any form other than a snake.  In addition, when activating this ability, you can expend a daily use of your serpent's bite ability to either increase the damage dice of any bite attack gained from wild shape as if a creature one size category larger, or instead use the bite damage from your serpent's bite ability and increase the DC of any poison attacks by +2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Serpent Lord ~ Poisonous Nature", at_level: 9, description: Some("Whenever you have a bite attack, you can choose to have it apply the following poison to any successful bite atatck: [frequency 1 round (6), effect 1 Con damage, cure 1 save, DC %1]. If you already have this poison, increase the damage to 1d3 Con damage and increase the DC by +2.  This poison can be used on %2 attacks per day.|PoisonousNaturePoisonDC|PoisonousNatureTimes"), benefit: None },
            ],
        },
        // Fighter Archetype ~ Ironborn -- up_abilities_class.lst:2483
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Ironborn",
            subject: "Fighter",
            archetype_name: "Ironborn",
            description: Some("Your body can merge with your armor."),
            source_page: Some("p.325"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ Ironborn],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterBravery,TYPE.FighterArmorTraining,TYPE.FighterBonusFeat2,TYPE.FighterBonusFeat4,TYPE.FighterBonusFeat8,TYPE.FighterArmorMastery]", "PRERACE:1,Forgeborn"]),
            replaces: Some(&["FighterBravery", "FighterArmorTraining_ALL", "FighterBonusFeat2", "FighterBonusFeat4", "FighterBonusFeat8", "FighterArmorMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Ironborn ~ Armor Aptitude", at_level: 2, description: Some("Whenever you are wearing armor, you reduce the armor check penalty by %1 (to a minimum of 0) and increase the maximum Dexterity bonus allowed by %1.  In addition you can move at your normal speed while wearing medium|ArmorAptitudeBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ironborn ~ Iron Fists", at_level: 2, description: Some("You gain two slam attacks whenever you are wearing medium or heavy armor."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ironborn ~ Durable Plating", at_level: 3, description: Some("You gain DR %1/- as long as you are wearing medium or heavy armor.  This damage reduction stacks with that gained from adamantine armor.|DurablePlatingDR"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ironborn ~ Always Armored", at_level: 4, description: Some("You can sleep in armor without being fatigued."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ironborn ~ Fusing of Man and Metal", at_level: 8, description: Some("As long as you are wearing medium or heavy armor, the armor bonus to AC is increased by %1.|FusingManMetalACBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ironborn ~ Shatterproof", at_level: 19, description: Some("Any time your armor is targeted by a sunder attempt or would otherwise take damage, the damage is instead transferred to you.  Any applicable hardness of the armor is applied before determine what (if any) damage is transferred to you."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Psionic Fighter -- up_abilities_class.lst:2479
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Psionic Fighter",
            subject: "Fighter",
            archetype_name: "Psionic Fighter",
            description: Some("You unlock the psionic potential in yourself instead of developing the techniques that most fighters choose."),
            source_page: Some("p.294"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ Psionic Fighter],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterBonusFeat1,TYPE.FighterBravery,TYPE.FighterWeaponTraining2,TYPE.FighterWeaponTraining3]"]),
            replaces: Some(&["FighterBonusFeat1", "FighterBravery", "FighterWeaponTraining2", "FighterWeaponTraining3"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Psionic Fighter ~ Psionic Feats", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Psionic Fighter ~ Willpower", at_level: 2, description: Some("You gain a +%1 on Will saves against Charm and Compulsion effects as long as you maintain psionic focus.|PsionicFighterWillpowerBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Psionic Fighter ~ Telekinetic Draw", at_level: 5, description: Some("You can expend your psionic focus to telekinetically draw your weapon from within 30 ft.  You must have line of sight to the weapon and it must be unattended."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Psionic Fighter ~ Combat Focus", at_level: 9, description: Some("You do not provoke attacks of opportunity when gaining psionic focus."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Psionic Fighter ~ Double Imbue", at_level: 13, description: Some("You gain the ability to trigger two effects that require expending psionic focus on a single attack by expending only one psionic focus.  This ability cannot be used with additional sources of psionic focus, nor can it be used to trigger the same effect twice on the same attack."), benefit: None },
            ],
        },
        // Monk Archetype ~ Disciple of the Raging Sea -- up_abilities_class.lst:2516
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Disciple of the Raging Sea",
            subject: "Monk",
            archetype_name: "Disciple of the Raging Sea",
            description: Some("Your intense meditations and discipline allow you to unleash a terrifying range when needed."),
            source_page: Some("p.332"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Disciple of the Raging Sea],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkStunningFist]", "PRERACE:1,Maenad"]),
            replaces: Some(&["MonkStunningFist"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Disciple of the Raging Sea ~ Unyielding Anger", at_level: 1, description: Some("When you use your racial outburst ability, the bonus to Strength is increased to +4.  If you expend your psionic focus when activating your racial outburst ability, this bonus is increased to +6 for the first round of the outburst."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Disciple of the Raging Sea ~ Eye of the Storm", at_level: 2, description: Some("You can choose to gain a rage power in place of one of your monk bonus feats.  You must meet any prerequisites of the range power, and your barbarian level is considered to be your monk level for the purposes of which range powers you can select.  Your rage powers activate when you are under the effects of your racial outburst ability.  After you end your racial outburst ability, you must wait one minute before beginning another outburst.  You may spend 2 ki points as a swift action to ignore this resting period."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Disciple of the Raging Sea ~ Raging Ki", at_level: 4, description: Some("You can spend ki to extend a use of your outburst racial ability as with power points, but one ki point is considered equal to two power points."), benefit: None },
            ],
        },
        // Monk Archetype ~ Enlightened Monk -- up_abilities_class.lst:2514
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Enlightened Monk",
            subject: "Monk",
            archetype_name: "Enlightened Monk",
            description: Some("You have learned how to use your ki to fuel your psionic abilities and ways to harness your psionic power."),
            source_page: Some("p.292"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Enlightened Monk],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkBonusFeat1,TYPE.MonkSlowFall,TYPE.MonkBonusFeat6]"]),
            replaces: Some(&["MonkBonusFeat1", "MonkSlowFall", "MonkBonusFeat6"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Enlightened Monk ~ Psionic Training", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Enlightened Monk ~ Stunning Fist", at_level: 1, description: Some("When you use your Stunning Fist ability, you can choose to expend your psionic focus to gain a +%1 bonus to the save DC of that Stunning Fist attack.  If you expend your psionic focus for the Psionic Fist feat as part of a Stunning Fist attack, you gain this bonus in addition to the bonus damage from Stunning Fist.|EnlightenedMonkStunningFistDCBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Enlightened Monk ~ Ki Psionics", at_level: 1, description: Some("You can use your Ki points in place of power points for special abilities including manifesting powers (such as those that might be gained from multiclassing) or racial abilities like the elan's resilience ability.  In addition, you can spend 1 ki point to use catfall as a psi-like ability with a manifester level of %1.|KiPsionicsCatfallML"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Enlightened Monk ~ Augmented Stunning Fist", at_level: 6, description: Some("You can channel power points into your Stunning Fist attacks to make them more devastating.  When you use your Stunning Fist attack, you may choose to spend up to %1 power points on the attack.  If you do, you gain an insight bonus to the damage if the attack is successful equal to the number of power points spent.  For every two power points spent adding damage, the save DC of the Stunning Fist attack increases by 1.|AugmentedStunningFistPP"), benefit: None },
            ],
        },
        // Paladin Archetype ~ Purifier -- up_abilities_class.lst:2535
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Purifier",
            subject: "Paladin",
            archetype_name: "Purifier",
            description: Some("You seek to cleanse the unnatural and abberant from the world."),
            source_page: Some("p.206"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Purifier],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinSmiteEvil,TYPE.PaladinDetectEvil,TYPE.PaladinLayOnHands,TYPE.PaladinChannelPositiveEnergy,TYPE.PaladinMercy,TYPE.PaladinSpells,TYPE.PaladinDivineBond]"]),
            replaces: Some(&["PaladinSmiteEvil", "PaladinDetectEvil", "PaladinLayOnHands", "PaladinChannelPositiveEnergy", "PaladinMercy", "PaladinSpells", "PaladinDivineBond"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Purifier ~ Smite the Unclean", at_level: 1, description: Some("You can call out to the powers of good to aid you in your struggle against the unclean %1 times per day. As a swift action, you choose one target within sight to smite. If this target is an aberration or has the psionic subtype, you add +%2 to your attack rolls and +%3 to all damage rolls made against the target of your smite. Regardless of the target, Smite the Unclean attacks automatically bypass any DR the creature might possess. In addition, while smite the unclean is in effect, you gain a +%5 deflection bonus to your AC against attacks made by the target of the smite. If you target a creature that is not an aberration or of the psionic subtype, the smite is wasted with no effect. The Smite the Unclean effect remains until the target of the smite is dead or the next time you rest and regain your uses of this ability.|SmiteTheUncleanTimes|SmiteTheUncleanAttackBonus|SmiteTheUncleanDamageBonus|SmiteTheUncleanDamageBonus*2|SmiteTheUncleanACBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Purifier ~ Detect Evil", at_level: 1, description: Some("You can manifest detect psionics as a psi-like ability in place of detect evil, although you can still use detect evil as normal."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Purifier ~ Aura of Purification", at_level: 2, description: Some("You and all allies within %1 ft. gain a +%2 deflection bonus to AC against attacks made by aberrations or psionic creatures, and a +%2 insight bonus to their Will saves against attacks from these sources.|AuraOfPurificationRange|AuraOfPurificationBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Purifier ~ Purifying Flame", at_level: 3, description: Some("%1/day, your weapon can be wreathed in a purifying flame that deals %2d6 points of fire damage for one round.  When used against the target of your smite, this fire damage ignores fire resistance and fire immunity.|PurifyingFlameTimes|PurifyingFlameDice"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Purifier ~ Purifier Manifesting", at_level: 4, description: Some("Purifier Powers Known: %1; Purifier Maximum Power Level Known: %2|PurifierPowersKnown|PurifierMaxPowerLevel"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Paladin ~ Divine Bond", at_level: 5, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Purifier ~ No Spellcasting", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Psionic", at_level: 1, description: None, benefit: None },
            ],
        },
        // Paladin Archetype ~ Sleeper's Guardian -- up_abilities_class.lst:2537
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Sleeper's Guardian",
            subject: "Paladin",
            archetype_name: "Sleeper's Guardian",
            description: Some("You will go to any lengths to keep the world safe, and the Sleeper deep in his slumber."),
            source_page: Some("p.318"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Sleeper's Guardian],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinAuraOfGood,TYPE.PaladinSmiteEvil,TYPE.PaladinDivineGrace,TYPE.PaladinDivineBond,TYPE.PaladinSpells]", "PRERACE:1,Duergar,Duergar ~ Psionic"]),
            replaces: Some(&["PaladinAuraOfGood", "PaladinSmiteEvil", "PaladinDivineGrace", "PaladinDivineBond", "PaladinSpells"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Sleeper's Guardian ~ Aura of Order", at_level: 1, description: Some("The power of your aura of lawful (see the detect law spell) is equal to your paladin level.  This replaces Aura of Good."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sleeper's Guardian ~ Smite Chaos", at_level: 1, description: Some("You can call out to the powers of order to aid you in your struggle against chaos %1 times per day. As a swift action, you choose one target within sight to smite. If this target is chaotic, you add +%2 to your attack rolls and +%3 to all damage rolls made against the target of your smite. If the target of Smite Chaos is an outsider with the chaos subtype, a chaos-aligned dragon, or an undead creature, the bonus to damage on the first successful attack increases to +%4. Regardless of the target, Smite Chaos attacks automatically bypass any DR the creature might possess. In addition, while Smite Chaos is in effect, you gain a +%5 deflection bonus to your AC against attacks made by the target of the smite. If you target a creature that is not chaotic, the smite is wasted with no effect. The Smite Chaos effect remains until the target of the smite is dead or the next time you rest and regain your uses of this ability.|SmiteChaosTimes|SmiteChaosAttackBonus|SmiteChaosDamageBonus|SmiteChaosDamageBonus*2|SmiteChaosACBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sleeper's Guardian ~ Disrupting Lullaby", at_level: 2, description: Some("All enemies within %1 ft. of you able to hear your humming suffer a -%2 penalty to attack and damage rolls.  This does not work in areas affected by silence.  It can be used %3 rounds per day.  These rounds do not need to be consecutive.|DisruptingLullabyRange|DisruptingLullabyPenalty|DisruptingLullabyRounds"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sleeper's Guardian ~ Sleeper's Guardian Manifesting", at_level: 4, description: Some("Sleeper's Guardian Powers Known: %1; Sleeper's Guardian Maximum Power Level Known: %2|SleepersGuardianPowersKnown|SleepersGuardianMaxPowerLevel"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Paladin ~ Divine Bond", at_level: 5, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Sleeper's Guardian ~ Divine Bond", at_level: 5, description: Some("You can form a mind blade, although it is always in the form of a battleaxe."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Purifier ~ No Spellcasting", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Psionic", at_level: 1, description: None, benefit: None },
            ],
        },
        // Ranger Archetype ~ Kinslayer -- up_abilities_class.lst:2567
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Kinslayer",
            subject: "Ranger",
            archetype_name: "Kinslayer",
            description: Some("You see your giant cousins as savages who must be eliminated for the greater good."),
            source_page: Some("p.328"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ Kinslayer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerFavoredEnemy1,TYPE.RangerSpells,TYPE.RangerHuntersBond,TYPE.RangerWoodlandStride]", "PRERACE:1,Half-Giant"]),
            replaces: Some(&["RangerFavoredEnemy1", "RangerHuntersBond", "RangerSpells", "RangerWoodlandStride"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Kinslayer ~ Favored Enemy", at_level: 1, description: Some("You must choose humanoid (giant) as your favored enemy.  In addition, when you make trip attempts against creatures of the humanoid (giant) subtype, you double your favored enemy bonus for the trip attempt and can make trip attempts regardless of the creature's size compared to your own."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Kinslayer ~ Hunter's Companion", at_level: 4, description: Some("You must choose an animal companion.  When you and your animal companion are flanking the same enemy of the humanoid (giant) subtype, the bonus you each gain for flanking is increased by 2.  In addition, you and your animal companion are able to communicate telepathically as long as you are maintaining psionic focus, with a range of %1 feet.  This communication does not make the animal companion any more intelligent, so this typically results in you giving commands to the animal companion or simple questions and answers."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Kinslayer ~ Psionics", at_level: 4, description: Some("You gain access to a variety of psi-like abilities; these are all applicable for use with the share spells ability of the animal companion. You can choose to use the expansion ability on your animal companion instead of on yourself, treating the ability as having a range of touch."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Kinslayer ~ Shared Favor", at_level: 7, description: Some("Your animal companion is treated as having your favored enemy ability against humanoid (giant) creatures, with the same bonuses against that favored enemy as yours."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Pack Leader ~ No Spellcasting", at_level: 1, description: None, benefit: None },
            ],
        },
        // Ranger Archetype ~ Pack Leader -- up_abilities_class.lst:2565
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Pack Leader",
            subject: "Ranger",
            archetype_name: "Pack Leader",
            description: Some("You use a subconscious tie to your allies to tie them all into a powerful hunting pack."),
            source_page: Some("p.291"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ Pack Leader],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerWildEmpathy,TYPE.RangerSpells,TYPE.RangerHuntersBond,TYPE.RangerWoodlandStride,TYPE.RangerCombatStyleFeat10]"]),
            replaces: Some(&["RangerWildEmpathy", "RangerSpells", "RangerHuntersBond", "RangerWoodlandStride", "RangerCombatStyleFeat10"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Pack Leader ~ The Pack", at_level: 4, description: Some("You can add up to %1 other creatures into your pack (collective).  When members of the pack are attacking the same creature, they gain a +1 bonus on weapon attack and damage rolls against that creature.|ThePackSize"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Pack Leader ~ Share Effects", at_level: 7, description: Some("When you manifest a power with a range of personal, you can expend your psionic focus to choose"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Pack Leader ~ Telepathy", at_level: 10, description: Some("All willing members of your pack can communicate with each other telepathically, even if they do not share a common language.  Psionic creatures who are willing members in a pack may manifest unknown powers known by another willing psionic creature in the pack as if they were making physical contact."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Pack Leader ~ No Spellcasting", at_level: 1, description: None, benefit: None },
            ],
        },
        // Rogue Archetype ~ Cerebral Infiltrator -- up_abilities_class.lst:2596
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Cerebral Infiltrator",
            subject: "Rogue",
            archetype_name: "Cerebral Infiltrator",
            description: Some("You focus more on dealing with people and less on dealing with traps and mechanical devices."),
            source_page: Some("p.291"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Cerebral Infiltrator],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RogueTrapfinding,TYPE.RogueTrapsense]"]),
            replaces: Some(&["RogueTrapfinding", "RogueTrapsense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Cerebral Infiltrator ~ Enhanced Senses", at_level: 1, description: Some("You unlock your psionic talent and additional insight into interactions with others.  You gain a +%1 bonus on Sense Motive checks.|EnhancedSensesSenseMotiveBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cerebral Infiltrator ~ Cripple Senses", at_level: 3, description: Some("You gain the ability when making a sneak attack to blind and deafen the struck creature for %1 rounds %2/day.|CrippleSensesDuration|CrippleSensesTimes"), benefit: None },
            ],
        },
        // Rogue Archetype ~ Menteur -- up_abilities_class.lst:2598
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Menteur",
            subject: "Rogue",
            archetype_name: "Menteur",
            description: Some("You learn to use your psionic power to alter your appearance, abscond to a safe location, and talk your way out of most any situation."),
            source_page: Some("p.322"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Menteur],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RogueTrapfinding,TYPE.RogueTrapsense,TYPE.RogueTalent6]", "PRERACE:1,Elan"]),
            replaces: Some(&["RogueTrapfinding", "RogueTrapsense", "RogueTalent6"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Menteur ~ Silver Tongued", at_level: 1, description: Some("You add +%1 to Bluff skill checks made against an opponent's Sense Motive skill.|SilverTonguedBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Menteur ~ Undistinguished Features", at_level: 3, description: Some("By spending one power point as a full round action, you gain a +4 circumstance bonus to Disguise checks to appear as someone other than yourself for one hour, although you cannot attempt to appear as another specific individual.  Any individual who sees you while under this effect must make a Will save (DC %1) to recall seeing you.|UndistinguishedFeaturesDC"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Menteur ~ Safe Exit", at_level: 6, description: Some("You can set a location as a full-round action that provokes attacks of opportunity when standing in the location.  As long as you are within 400 feet of the location, you can teleport to it as a standard action.  You can bring along %1 additional creatures of the same size or smaller than yourself that you are physically touching, although if the creature or creatures touched are unwilling, a Will save (DC %2) leaves that creature behind.  Using this ability to teleport can only be done 1/day.|SafeExitExtraTargets|SafeExitDC"), benefit: None },
            ],
        },
        // Rogue Archetype ~ Reaving Raider -- up_abilities_class.lst:2599
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Reaving Raider",
            subject: "Rogue",
            archetype_name: "Reaving Raider",
            description: Some("You choose the life of the pirate over that of more reputable work."),
            source_page: Some("p.332"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Reaving Raider],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RogueTrapfinding,TYPE.RogueTrapsense,TYPE.RogueTalent8]", "PRERACE:1,Maenad"]),
            replaces: Some(&["RogueTrapfinding", "RogueTrapsense", "RogueTalent8"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Reaving Raider ~ Take Captive", at_level: 1, description: Some("While maintaining psionic focus, you can choose to deal nonlethal damage without incurring the -4 penalty on the attack rolls.  In addition, when you confirm a critical hit with an attack that deals nonlethal damage, you can expend your psionic focus to try to knock the enemy out, although a successful save (Fort DC %1) negates this effect.|TakeCaptiveDC"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Reaving Raider ~ Weapon and Armor Proficiencies", at_level: 1, description: Some("You gain medium armor proficiency."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Reaving Raider ~ Clear the Rail", at_level: 3, description: Some("When you activate your racial outburst ability, you can choose to forego the benefit of the ability (although you still suffer all appropriate penalties) and direct the energy from the outburst into a sonic blast at a single enemy within a 30 ft. range.  You make a ranged bull rush attempt at the target, using your Charisma modifier instead of your Strength modifier and using your character level in place of your base attack bonus. This ability does not provoke an attack of opportunity and you do not move with the targeted enemy."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Reaving Raider ~ Disrupting Scream", at_level: 8, description: Some("You can activate your racial outburst ability and choose to forego the benefit of the ability (although you still suffer all appropriate penalties) to attempt to knowck an adjacent enemy off guard as a swift action.  The targeted enemy must make a save (Reflex DC %1) or be treated as flat-footed for your next attack made on your turn.|DisruptingScreamDC"), benefit: None },
            ],
        },        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_has_15_records() {
        assert_eq!(archetype_swap_tables().len(), 15);
    }

    #[test]
    fn keys_are_unique_within_book() {
        let keys: std::collections::BTreeSet<&str> =
            archetype_swap_tables().iter().map(|e| e.key).collect();
        assert_eq!(keys.len(), archetype_swap_tables().len());
    }

    #[test]
    fn every_master_record_carries_a_real_description_and_at_least_one_grant() {
        for e in archetype_swap_tables() {
            assert!(e.description.is_some(), "{} has no DESC:", e.key);
            assert!(!e.grants.is_empty(), "{} grants no features", e.key);
        }
    }

    /// The real finding this table exists to measure: `TYPE:`'s replaced-
    /// slot count and `ABILITY:`'s granted-feature count are NOT the same
    /// list under two names. Corrected twice: once for a parser gap that
    /// undercounted grants (missing PREVARGTEQ:/multi-name shapes), once
    /// for a category-inclusion ruling (Internal-categorized bookkeeping
    /// grants and NORMAL-type player-chosen grants excluded) that had
    /// been over-counting instead. Real rate: 33% (5/15).
    #[test]
    fn the_type_and_ability_lists_genuinely_disagree() {
        let total_replaces: usize =
            archetype_swap_tables().iter().map(|e| e.replaces.map_or(0, |r| r.len())).sum();
        let total_grants: usize = archetype_swap_tables().iter().map(|e| e.grants.len()).sum();
        assert_eq!(total_replaces, 68, "total TYPE: replaced-slot count across all 15 records");
        assert_eq!(total_grants, 75, "total ABILITY: granted-feature count across all 15 records, after the category ruling");
        assert_ne!(
            total_replaces, total_grants,
            "TYPE: and ABILITY: are two different lists, not two views of one -- if this ever \
             passes as equal, the corpus shape has changed and the doc comment's own claim needs \
             re-checking, not silently trusting the new equality"
        );

        let equal_count_records = archetype_swap_tables()
            .iter()
            .filter(|e| e.replaces.map_or(0, |r| r.len()) == e.grants.len())
            .count();
        assert_eq!(equal_count_records, 5, "of 15 (33%) -- twice-corrected figure, see this module's own doc comment");
    }

    #[test]
    fn every_grant_names_a_real_level_and_key() {
        for e in archetype_swap_tables() {
            for g in e.grants {
                assert!(!g.grants_feature_key.is_empty(), "{} has an empty grant key", e.key);
                assert!(g.at_level >= 1 && g.at_level <= 20, "{} grant {} has an implausible level {}", e.key, g.grants_feature_key, g.at_level);
            }
        }
    }

    /// No Internal-categorized bookkeeping grant (e.g. `Armor Aptitude
    /// 7th Level`) should ever appear in this table again -- pinned as
    /// its own regression guard after it was found in a prior commit.
    #[test]
    fn no_internal_category_bookkeeping_grant_is_present() {
        for e in archetype_swap_tables() {
            for g in e.grants {
                assert_ne!(g.grants_feature_key, "Armor Aptitude 7th Level", "Internal-category bookkeeping grant leaked back in");
            }
        }
    }

    #[test]
    fn resolved_grant_descriptions_are_the_real_count() {
        let resolved: usize = archetype_swap_tables()
            .iter()
            .flat_map(|e| e.grants.iter())
            .filter(|g| g.description.is_some() || g.benefit.is_some())
            .count();
        assert_eq!(resolved, 65, "65 of 75 grants carry real DESC:/BENEFIT: text");
    }
}
