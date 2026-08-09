//! Advanced Player's Guide (APG) archetype-swap catalog. SD28-E30
//! (`epic-32-archetype-swap`) tier-1 table 3. See
//! `ultimate_psionics::archetype_tables`'s own module doc comment for
//! the full struct rationale, and `rules_tables::archetype_swap` for
//! the shared struct all three tables landed so far use.
//!
//! **The TYPE:/ABILITY: agreement rate is book-dependent, not a fixed
//! constant -- confirmed on a third book.** UPsi 13% (2/15, corrected),
//! ACG 34% (30/87), **APG 52% (42/80)** -- APG's own archetypes disagree
//! less than either prior book: 333 total `TYPE:`-replaced slots vs 392
//! total `ABILITY:`-granted features. The two-list struct stays correct
//! regardless -- it was never sized to a specific disagreement rate,
//! only to the fact that one exists.
//!
//! **364 of 392 sub-feature grants (93%) resolved to real `DESC:`/
//! `BENEFIT:` text.** The 28 shortfalls cluster into two real, distinct
//! causes, not a grab-bag: **1 failed `KEY:` lookup** (`Improved
//! Counterspell`, plausibly a cross-reference to a CRB-owned feat rather
//! than a class-feature row); **25 shared names across 5 sibling Druid
//! Shaman-totem archetypes** (`Bear`/`Eagle`/`Lion`/`Serpent`/`Wolf
//! Shaman`, each independently referencing the same 5 unresolved names
//! -- `Druid ~ Wild Shape`, `Shaman Druid Wild Shape`, `Shaman Druid
//! Wild Shape Progression`, `Shaman Druid Wild Shape Times`, `Shaman
//! Wild Shape` -- none of which is declared as its own row anywhere in
//! this book's file, confirmed by checking all 5 archetypes reference
//! the identical name set, not a per-archetype extraction miss); **2
//! `No Spellcasting ~ Paladin`/`No Spellcasting ~ Ranger` grants**
//! resolve to a real row with neither `DESC:` nor `BENEFIT:` (a bare
//! marker, the same shape UPsi's own `Purifier ~ No Spellcasting`
//! carries).
//!
//! **A parser-side finding, corrected before this table shipped rather
//! than after:** the extraction script's first pass treated any
//! non-level-gate `PRE`-shaped token inside an `ABILITY:` grant as a
//! *name* rather than skipping it -- caught on this book (several
//! Rogue-shaped archetypes carry a second `PREVARGTEQ:Rogue_CFP_Level,N`
//! token per grant, a class-specific tracking variable, not a
//! `<Class>LVL`-shaped level gate the extraction already recognised).
//! Fixed to skip every `PRE`-prefixed token from `grants`' own name list
//! regardless of whether it matches a known level-gate shape.
//!
//! **9 of this book's 12 Rogue archetype master records have no `DESC:`
//! at all -- a whole-subfamily gap, confirmed genuine on the raw corpus
//! rows, not an extraction bug.** `Rogue Archetype ~ Burglar`
//! (`apg_abilities_class.lst:2942`, spot-checked directly) carries
//! `CATEGORY:Archetype`, `TYPE:`, `PRECLASS:`, and its own
//! `ABILITY:...AUTOMATIC` grants -- but no `DESC:` or `BENEFIT:` token
//! anywhere on the row, and the same is true of `Cutpurse`,
//! `Investigator`, `Poisoner`, `Rake`, `Sniper`, `Spy`, `Thug`, and
//! `Trapsmith`. Only 3 of the 12 Rogue archetypes (`Acrobat`, `Scout`,
//! `Swashbuckler`) carry real flavour text on their own master row.
//! Every archetype in every other class family in this table (and in
//! UPsi's/ACG's own tables) carries real flavour text; this is a
//! genuine, book-and-family-specific gap in this book's own PCGen
//! conversion, not a stub this codebase introduced -- carried here as
//! `description: None` for all 9, named explicitly rather than silently
//! absorbed into a loosened assertion.
//!
//! **The `§46`/`§48`/`§49` text-shape triad, spot-checked against this
//! book's own archetype `.MOD` rows.** Same clean shape as UPsi/ACG --
//! pure `FACT:`-setter suppression rows, no prose, none of the three
//! hazards applied.
//!
//! **This table is data only.** No `pilot_compute.rs` integration lands
//! in this slice -- see `decisions.md §51`/`forward-scope-register.md
//! §C4.8` for why that half is blocked on an explicit scope decision.
//!
//! Every field below is copied verbatim from the real corpus row (source:
//! `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/
//! advanced_players_guide/apg_abilities_class.lst`), generated
//! programmatically by a one-off extraction script, not hand-transcribed.

use super::super::archetype_swap::{ArchetypeGrant, ArchetypeSwapEntry};

/// Full APG archetype-swap catalog: 80 real, distinct master records, in
/// source order. Built once and cached for the process lifetime.
pub fn archetype_swap_tables() -> &'static [ArchetypeSwapEntry] {
    static TABLE: std::sync::OnceLock<Vec<ArchetypeSwapEntry>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        vec![
        // Barbarian Archetype ~ Breaker -- apg_abilities_class.lst:1541
        ArchetypeSwapEntry {
            key: "Barbarian Archetype ~ Breaker",
            subject: "Barbarian",
            archetype_name: "Breaker",
            description: Some("While most barbarians are skilled at breaking things, some find the need to destroy their surroundings an almost uncontrollable urge when in the middle of a rage. These barbarians are a danger not only to their foes, but also to the very environment around them."),
            source_page: Some("p.78"),
            prerequisites: Some(&["PRECLASS:1,Barbarian=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Barbarian Archetype ~ Breaker],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BarbarianFastMovement,TYPE.BarbarianTrapSense]"]),
            replaces: Some(&["BarbarianCFFastMovement", "BarbarianCFTrapSense", "BarbarianFastMovement", "BarbarianTrapSense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Breaker ~ Destructive", at_level: 1, description: Some("Whenever the breaker barbarian makes a melee attack that targets an unattended object or makes a sunder combat maneuver, she adds +%1 on the damage roll.|DestructiveDamage"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Breaker ~ Battle Scavenger", at_level: 3, description: Some("At 3rd level, the breaker barbarian suffers no penalty on attack rolls when using an improvised weapon or a weapon with the broken condition. In addition, she gains a +%1 bonus on damage rolls with improvised or broken weapons for every three levels beyond 3rd. This ability replaces trap sense.|BattleScavengerDamage"), benefit: None },
            ],
        },
        // Barbarian Archetype ~ Brutal Pugilist -- apg_abilities_class.lst:1542
        ArchetypeSwapEntry {
            key: "Barbarian Archetype ~ Brutal Pugilist",
            subject: "Barbarian",
            archetype_name: "Brutal Pugilist",
            description: Some("Some barbarians focus on using their bare hands to tear their opponents limb from limb. These brutal pugilists also learn a great deal about various combat maneuvers, using them to cripple or crush their foes."),
            source_page: Some("p.78"),
            prerequisites: Some(&["PRECLASS:1,Barbarian=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Barbarian Archetype ~ Brutal Pugilist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BarbarianUncannyDodge,TYPE.BarbarianTrapSense,TYPE.BarbarianImprovedUncannyDodge]"]),
            replaces: Some(&["BarbarianCFUncannyDodge", "BarbarianCFTrapSense", "BarbarianCFImprovedUncannyDodge", "BarbarianUncannyDodge", "BarbarianTrapSense", "BarbarianImprovedUncannyDodge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Brutal Pugilist ~ Savage Grapple", at_level: 2, description: Some("At 2nd level, the brutal pugilist takes only half the normal penalties to Dexterity, attack rolls, and combat maneuver checks when she has the grappled condition. She can make an attack of opportunity against creatures trying to grapple her even if they possess the Improved Grapple feat or the grab special attack. If she hits with this attack of opportunity, she gains a +2 circumstance bonus to her CMD against the grapple attempt. She cannot make these attacks of opportunity once a grapple has succeeded. This ability replaces uncanny dodge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Brutal Pugilist ~ Pit Fighter", at_level: 3, description: Some("At 3rd level, the brutal pugilist has learned combat tricks from fighting in pit brawls and gladiatorial arenas. She selects one combat maneuver and gains a +1 insight bonus on her CMB or to her CMD in that maneuver. This bonus increases to +2 if the barbarian is wearing no armor (shields are allowed). At every three levels after 3rd, the barbarian may select another combat maneuver and add this bonus on her CMB or to her CMD. This bonus can be applied to each maneuver no more than twice, once on CMB and once to CMD. This ability replaces trap sense."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Brutal Pugilist ~ Improved Savage Grapple", at_level: 5, description: Some("At 5th level, the brutal pugilist takes no penalties to Dexterity, attack rolls, and combat maneuver checks when she has the grappled condition. She also is treated as one size larger than her actual size when determining whether she can grapple or be grappled by another creature. This ability replaces improved uncanny dodge."), benefit: None },
            ],
        },
        // Barbarian Archetype ~ Drunken Brute -- apg_abilities_class.lst:1543
        ArchetypeSwapEntry {
            key: "Barbarian Archetype ~ Drunken Brute",
            subject: "Barbarian",
            archetype_name: "Drunken Brute",
            description: Some("Barbarians are known for their ability to consume potent drink, but drunken brutes turn drinking into a combat tactic, using the potent liquor to fuel their rage and grant them additional powers."),
            source_page: Some("p.78"),
            prerequisites: Some(&["PRECLASS:1,Barbarian=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Barbarian Archetype ~ Drunken Brute],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BarbarianFastMovement]"]),
            replaces: Some(&["BarbarianCFFastMovement", "BarbarianFastMovement"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Drunken Brute ~ Raging Drunk", at_level: 1, description: Some("While raging, the drunken brute can drink a potion, or a tankard of ale or similar quantity of alcohol, as a move action that does not provoke attacks of opportunity. A potion has its normal effect, while an alcoholic drink allows the barbarian to maintain her rage that round without expending a round of rage for the day (instead of the alcohol's normal effects). For each alcoholic drink consumed while raging, the barbarian is nauseated for 1 round when her rage expires, in addition the normal fatigue that follows a rage. Tireless rage does not negate this nauseated condition but the internal fortitude rage power does. This ability replaces fast movement."), benefit: None },
            ],
        },
        // Barbarian Archetype ~ Elemental Kin -- apg_abilities_class.lst:1544
        ArchetypeSwapEntry {
            key: "Barbarian Archetype ~ Elemental Kin",
            subject: "Barbarian",
            archetype_name: "Elemental Kin",
            description: Some("Some barbarian tribes have strong ties to the elemental forces of nature. Their shamans anoint the warriors at birth, tying them to the patron element of the tribe and granting them lasting boons against such forces."),
            source_page: Some("p.78"),
            prerequisites: Some(&["PRECLASS:1,Barbarian=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Barbarian Archetype ~ Elemental Kin],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BarbarianTrapSense]"]),
            replaces: Some(&["BarbarianCFTrapSense", "BarbarianTrapSense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Elemental Kin ~ Elemental Fury", at_level: 3, description: Some("At 3rd level, whenever the elemental kin takes an amount of energy damage equal to or greater than her barbarian level while raging, she adds 1 to the total number of rounds that she can rage that day. At 6th level, and every three levels thereafter, the number of extra rounds per energy attack increases by +1, to a maximum of +6 rounds per energy attack at 18th level. This ability replaces trap sense."), benefit: None },
            ],
        },
        // Barbarian Archetype ~ Hurler -- apg_abilities_class.lst:1545
        ArchetypeSwapEntry {
            key: "Barbarian Archetype ~ Hurler",
            subject: "Barbarian",
            archetype_name: "Hurler",
            description: Some("A raging barbarian is frightening enough in melee, but some become skilled at throwing objects at their foes before closing in for the kill."),
            source_page: Some("p.78"),
            prerequisites: Some(&["PRECLASS:1,Barbarian=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Barbarian Archetype ~ Hurler],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BarbarianFastMovement]"]),
            replaces: Some(&["BarbarianCFFastMovement", "BarbarianFastMovement"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Hurler ~ Skilled Thrower", at_level: 1, description: Some("The hurler is skilled at throwing objects in combat. Increase the range increment of any thrown weapon or object by 10 feet. This ability replaces fast movement."), benefit: None },
            ],
        },
        // Barbarian Archetype ~ Invulnerable Rager -- apg_abilities_class.lst:1546
        ArchetypeSwapEntry {
            key: "Barbarian Archetype ~ Invulnerable Rager",
            subject: "Barbarian",
            archetype_name: "Invulnerable Rager",
            description: Some("Some barbarians learn to take whatever comes their way, shrugging off mortal wounds with ease. These barbarians invite their enemies to attack them, and use pain to fuel their rage."),
            source_page: Some("p.79"),
            prerequisites: Some(&["PRECLASS:1,Barbarian=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Barbarian Archetype ~ Invulnerable Rager],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BarbarianUncannyDodge,TYPE.BarbarianImprovedUncannyDodge,TYPE.BarbarianDamageReduction,TYPE.BarbarianTrapSense]"]),
            replaces: Some(&["BarbarianCFUncannyDodge", "BarbarianCFImprovedUncannyDodge", "BarbarianCFDamageReduction", "BarbarianCFTrapSense", "BarbarianUncannyDodge", "BarbarianImprovedUncannyDodge", "BarbarianDamageReduction", "BarbarianTrapSense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Invulnerable Rager ~ Invulnerability", at_level: 2, description: Some("At 2nd level, the invulnerable rager gains DR/- equal to half her barbarian level. This damage reduction is doubled against nonlethal damage. This ability replaces uncanny dodge, improved uncanny dodge, and damage reduction."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Invulnerable Rager ~ Extreme Endurance", at_level: 3, description: Some("At 3rd level, the invulnerable rager is inured to either hot or cold climate effects (choose one) as if using endure elements. In addition, the barbarian gains 1 point of fire or cold resistance for every three levels beyond 3rd. This ability replaces trap sense."), benefit: None },
            ],
        },
        // Barbarian Archetype ~ Mounted Fury -- apg_abilities_class.lst:1547
        ArchetypeSwapEntry {
            key: "Barbarian Archetype ~ Mounted Fury",
            subject: "Barbarian",
            archetype_name: "Mounted Fury",
            description: Some("Many barbarian tribes are masters of the horse, teaching their members how to ride from a young age. As a result, barbarians from such tribes are even more terrifying when mounted, using their steeds' speed and strength to great advantage."),
            source_page: Some("p.79"),
            prerequisites: Some(&["PRECLASS:1,Barbarian=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Barbarian Archetype ~ Mounted Fury],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BarbarianFastMovement,TYPE.BarbarianUncannyDodge,TYPE.BarbarianImprovedUncannyDodge]"]),
            replaces: Some(&["BarbarianFastMovement", "BarbarianUncannyDodge", "BarbarianImprovedUncannyDodge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Mounted Fury ~ Fast Rider", at_level: 1, description: Some("The speed of any mount the barbarian rides is increased by 10 feet. This ability replaces fast movement."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mounted Fury ~ Bestial Mount", at_level: 5, description: Some("At 5th level, the mounted fury gains the service of a feral mount. This ability functions as a druid's animal companion, using the barbarian's level -4 as her effective druid level. This companion must be one that she is capable of riding and is suitable as a mount. A Medium barbarian can select a camel or a horse. A Small barbarian can select a pony or a wolf, but can also select a boar or a dog if she is at least 8th level. Whenever a barbarian is raging while mounted on her bestial mount, the mount gains a +2 morale bonus to its Strength. This ability replaces uncanny dodge and improved uncanny dodge."), benefit: None },
            ],
        },
        // Barbarian Archetype ~ Savage Barbarian -- apg_abilities_class.lst:1548
        ArchetypeSwapEntry {
            key: "Barbarian Archetype ~ Savage Barbarian",
            subject: "Barbarian",
            archetype_name: "Savage Barbarian",
            description: Some("Some barbarians are truly savage, having little training in modern arms. These savage barbarians learn to avoid blows and toughen up their skin."),
            source_page: Some("p.79"),
            prerequisites: Some(&["PRECLASS:1,Barbarian=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Barbarian Archetype ~ Savage Barbarian],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BarbarianTrapSense,TYPE.BarbarianDamageReduction]"]),
            replaces: Some(&["BarbarianTrapSense", "BarbarianDamageReduction"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Savage Barbarian ~ Naked Courage", at_level: 3, description: Some("At 3rd level, the savage barbarian gains a +%1 dodge bonus to AC and a +%1 morale bonus on saving throws against fear when wearing no armor (shields are allowed). This bonus increases by +1 for every six levels after 3rd. This ability replaces trap sense.|NakedCourageDodgeBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Savage Barbarian ~ Natural Toughness", at_level: 7, description: Some("At 7th level, the savage barbarian gains a +%1 natural armor bonus to AC when wearing no armor (shields are allowed). This bonus increases by +1 for every three levels beyond 7th. This ability replaces damage reduction.|NaturalToughness"), benefit: None },
            ],
        },
        // Barbarian Archetype ~ Superstitious -- apg_abilities_class.lst:1549
        ArchetypeSwapEntry {
            key: "Barbarian Archetype ~ Superstitious",
            subject: "Barbarian",
            archetype_name: "Superstitious",
            description: Some("Many barbarians distrust magic. While most just shy away from magic, others focus their rage on users of such foul arts. These barbarians are naturally distrusting, and develop keen senses to protect them from harm."),
            source_page: Some("p.79"),
            prerequisites: Some(&["PRECLASS:1,Barbarian=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Barbarian Archetype ~ Superstitious],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BarbarianTrapSense,TYPE.BarbarianDamageReduction]"]),
            replaces: Some(&["BarbarianTrapSense", "BarbarianDamageReduction"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Superstitious ~ Sixth Sense", at_level: 3, description: Some("At 3rd level, the superstitious barbarian gains a +%1 bonus on initiative and a +%1 insight bonus to AC during surprise rounds. This bonus increases by +1 for every three levels after 3rd. This ability replaces trap sense.|BarbarianLVL/3"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Superstitious ~ Keen Senses", at_level: 7, description: Some("At 7th level, the superstitious barbarian gains low-light vision (triple normal vision range in dim light if she already has low-light vision). At 10th level, she gains darkvision 60 feet (or adds 60 feet to the range of any darkvision already possessed). At 13th level, she gains scent. At 16th level, she gains blindsense 30 feet. At 19th level, she gains blindsight 30 feet. This ability replaces damage reduction."), benefit: None },
            ],
        },
        // Bard Archetype ~ Arcane Duelist -- apg_abilities_class.lst:1618
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Arcane Duelist",
            subject: "Bard",
            archetype_name: "Arcane Duelist",
            description: Some("A master of the martial applications of steel and spell, the spellsword blends both into a lethal combination."),
            source_page: Some("p.80"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Arcane Duelist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardBardicKnowledge,TYPE.BardCountersong,TYPE.BardSuggestion,TYPE.BardMassSuggestion,TYPE.BardVersatilePerformance,TYPE.BardWellVersed,TYPE.BardLoreMaster,TYPE.BardJackOfAllTrades]"]),
            replaces: Some(&["BardBardicKnowledge", "BardCountersong", "BardSuggestion", "BardMassSuggestion", "BardVersatilePerformance", "BardWellVersed", "BardLoreMaster", "BardJackOfAllTrades"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Arcane Duelist ~ Arcane Strike", at_level: 1, description: Some("Arcane duelists gain Arcane Strike as a bonus feat at 1st level. This ability replaces bardic knowledge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Arcane Duelist ~ Rallying Cry", at_level: 1, description: Some("At 1st level, an arcane duelist can use performance to rally dispirited allies. Each round he makes an Intimidate check. Any ally (including the bard) within 30 feet may use this check in place of his own saving throw against fear and despair effects. Those already under a fear or despair effect can attempt a new save each round using the bard's Intimidate check. Rallying cry does not work on effects that don't allow saves. This is a mind-affecting ability that uses audible components. This performance replaces countersong."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Arcane Duelist ~ Bladethirst", at_level: 6, description: Some("An arcane duelist of 6th level or higher may use performance to grant one weapon, one natural weapon, one end of a double weapon, or 50 items of ammunition of the same type within 30 feet a +%1 enhancement bonus. This enhancement bonus increases by +1 for every three levels after 6th (maximum +5 at 18th level). These bonuses stack with existing bonuses and may be used to increase the item's enhancement bonus up to +5 or to add any of the following weapon properties: defending, distance, ghost touch, keen, mighty cleaving, returning, shock, shocking burst, seeking, speed, or wounding (Pathfinder RPG Core Rulebook page 469). If the weapon is not magical, at least a +1 enhancement bonus must be added before adding special abilities. This performance replaces suggestion.|((classlevel(\"Bard\")-3)/3)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Arcane Duelist ~ Mass Bladethirst", at_level: 18, description: Some("An arcane duelist of 18th level or higher can use his bladethirst performance to enhance the weapons of as many allies as desired within 30 feet. The bonus provided by this power is +4 if conferred on two allies, +3 for three allies, +2 for four allies, and +1 for five or more allies. The power granted to each weapon must be identical. This ability replaces mass suggestion."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Arcane Duelist ~ Bonus Feats", at_level: 1, description: Some("An arcane duelist weaves might and magic in his combat style, gaining the following bonus feats at 2nd level and every four levels thereafter: 2nd level-Combat Casting, 6th level-Disruptive, 10th level-Spellbreaker, 14th level-Penetrating Strike, 18th level-Greater Penetrating Strike. This ability replaces versatile performance and well-versed."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Arcane Duelist ~ Arcane Bond", at_level: 5, description: Some("At 5th level, an arcane duelist gains the arcane bond ability as a wizard, using a weapon as his bonded item, allowing him to cast any one additional spell that he knows once per day. He may not choose a familiar or other type of bonded item. He may use the hand holding his bonded weapon for somatic components. This ability replaces lore master."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Arcane Duelist ~ Arcane Armor", at_level: 10, description: Some("At 10th level, an arcane duelist gains Medium Armor Proficiency and can cast bard spells in medium armor with no chance of arcane spell failure. At 16th level, he gains Heavy Armor Proficiency and can cast bard spells in heavy armor with no arcane spell failure. This ability replaces jack of all trades."), benefit: None },
            ],
        },
        // Bard Archetype ~ Archivist -- apg_abilities_class.lst:1619
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Archivist",
            subject: "Bard",
            archetype_name: "Archivist",
            description: Some("Some bards greatly prefer academic pursuits to the drama (and sometimes melodrama) of their artistic brethren."),
            source_page: Some("p.80"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Archivist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardInspireCourage,TYPE.BardSuggestion,TYPE.BardMassSuggestion,TYPE.BardVersatilePerformance,TYPE.BardWellVersed,TYPE.BardLoreMaster,TYPE.BardJackOfAllTrades]"]),
            replaces: Some(&["BardInspireCourage", "BardSuggestion", "BardMassSuggestion", "BardVersatilePerformance", "BardWellVersed", "BardLoreMaster", "BardJackOfAllTrades"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Archivist ~ Naturalist", at_level: 1, description: Some("An archivist who has identified a creature with a Knowledge check appropriate to its type can use performance to share strategies for defeating it with allies in combat. The archivist and any allies within 30 feet gain a +%1 insight bonus to AC and on attack rolls and saving throws against exceptional, supernatural, and spell-like abilities used by creatures of that specific kind of monster (e.g., frost giants, not all giants or all humanoids). This bonus increases by +1 at 5th level and every six levels thereafter. This language-dependent ability requires visual and audible components. This ability replaces inspire courage.|NaturalistBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Archivist ~ Lamentable Belaborment", at_level: 6, description: Some("At 6th level, an archivist can bewilder a creature already fascinated by his performance. Using this ability does not disrupt the fascinate effect, but it does require a standard action to activate (in addition to the free action to continue the fascinate effect). The target must make a Will save DC %1 (DC 10 + 1/2 the bard's level + the bard's Cha modifier). Success renders the target immune to this power for 24 hours, but failure leaves the target either dazed or confused (archivist's choice) for as long as the performance continues. If the target takes damage, this effect ends immediately. This mind-affecting ability relies on audible components. This ability replaces suggestion.|10+(BardLVL/2)+CHA"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Archivist ~ Pedantic Lecture", at_level: 18, description: Some("At 18th level, an archivist can affect as many creatures with lamentable belaborment as he currently has fascinated. In addition, he may choose to cause targets to fall asleep rather than be dazed or confused. This ability replaces mass suggestion."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Archivist ~ Lore Master", at_level: 2, description: Some("At 2nd level, an archivist may take 20 on Knowledge checks %1/day, plus once per six levels beyond 2nd. This ability replaces versatile performance.|((BardLVL-2)/6)+1"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Archivist ~ Magic Lore", at_level: 2, description: Some("At 2nd level, an archivist gains a bonus on Spellcraft checks to identify magic items or decipher scrolls and may take 10 on such checks. An archivist can use Disable Device to disarm magical traps as per a rogue's trapfinding ability and gains a +4 bonus on saves against magical traps, language-dependent effects, and symbols, glyphs, and magical writings of any kind. This ability replaces well-versed."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Archivist ~ Jack of All Trades", at_level: 5, description: Some("At 5th level, an archivist can use any skill, even if the skill normally requires him to be trained. At 11th level, he considers all skills to be class skills, and at 17th level he can take 10 on any skill check, even if it is not normally allowed. This ability replaces lore master."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Archivist ~ Probable Path", at_level: 10, description: Some("At 10th level, an archivist can calculate the action likely to bring success with the least risk. %1/day, he can take 10 on any d20 roll. He may use this ability one additional time per day for every three levels after 10th. This ability replaces jack of all trades.|(BardLVL-7)/3"), benefit: None },
            ],
        },
        // Bard Archetype ~ Court Bard -- apg_abilities_class.lst:1620
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Court Bard",
            subject: "Bard",
            archetype_name: "Court Bard",
            description: Some("Spending years studying all of the finer points of erudition and etiquette, the court bard takes up the role of resplendent proclaimer and artist-in-residence at the hand of nobility, royalty, and the well-moneyed elite who aspire to join their ranks."),
            source_page: Some("p.81"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Court Bard],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardInspireCourage,TYPE.BardInspireCompetence,TYPE.BardDirgeOfDoom,TYPE.BardFrighteningTune,TYPE.BardBardicKnowledge,TYPE.BardLoreMaster,TYPE.BardJackOfAllTrades]"]),
            replaces: Some(&["BardInspireCourage", "BardInspireCompetence", "BardDirgeOfDoom", "BardFrighteningTune", "BardBardicKnowledge", "BardLoreMaster", "BardJackOfAllTrades"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Court Bard ~ Satire", at_level: 1, description: Some("A court bard can use performance to undermine the confidence of enemies who hear it, causing them to take a -%1 penalty on attack and damage rolls (minimum 1) and a -%1 penalty on saves against fear and charm effects as long as the bard continues performing. This penalty increases by -1 at 5th level and every six levels thereafter. Satire is a language-dependent, mind-affecting ability that uses audible components. This performance replaces inspire courage.|(BardLVL+1)/6"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Court Bard ~ Mockery", at_level: 3, description: Some("A court bard of 3rd level or higher can subtly ridicule and defame a specific individual. The bard selects one target who can hear his performance. That individual takes a -2 penalty on Charisma checks and Charisma-related skill checks as long as the bard continues performing. This penalty increases by -1 every four levels after 3rd. Mockery is a language-dependent, mind-affecting ability that relies on audible components. This performance replaces inspire competence.|((BardLVL+1)/4)+2"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Court Bard ~ Glorious Epic", at_level: 8, description: Some("A court bard of 8th level or higher can weave captivating tales that engross those who hear them. Enemies within 30 feet become flat-footed unless they succeed at a Will save DC %1 (DC 10 + 1/2 the bard's level + the bard's Cha modifier). A save renders them immune to this ability for 24 hours. Glorious epic is a language-dependent, mind-affecting ability that uses audible components. This performance replaces dirge of doom.|10+CHA+(BardLVL/2)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Court Bard ~ Scandal", at_level: 14, description: Some("A court bard of 14th level or higher can combine salacious gossip and biting calumny to incite a riot. Each enemy within 30 feet is affected as if by a song of discord for as long as it can hear the performance. A successful Will save DC %1 (DC 10 + 1/2 the bard's level + the bard's Cha modifier) negates the effect, and that creature is immune to this ability for 24 hours. Scandal is a language-dependent, mind-affecting ability that uses audible components. This performance replaces frightening tune.|10+CHA+(BardLVL/2)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Court Bard ~ Heraldic Expertise", at_level: 1, description: Some("A court bard gains a bonus equal to half his bard level on Diplomacy, Knowledge (history), Knowledge (local), and Knowledge (nobility) checks [Bonus is %1] (minimum +1).%2/day, the court bard can also reroll a check against one of these skills, though he must take the result of the second roll even if it is worse. He can reroll one additional time per day at 5th level and every five levels thereafter. This ability replaces bardic knowledge.|BardLVL/2|(BardLVL/5)+1"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Court Bard ~ Wide Audience", at_level: 5, description: Some("At 5th level, a court bard can choose to affect a %1-foot [60-foot] cone instead of a %2 [30-foot] radius with bardic performances that affect an area. In addition, for every five levels beyond 5th, the area of such powers is increased by 10 feet (radius) or 20 feet (cone). If the power instead affects multiple creatures, it affects one additional creature than normal for every five levels beyond 5th. This does not affect powers that affect only a single creature. This ability replaces lore master and jack of all trades.|WideAudienceCone|WideAudienceRadius"), benefit: None },
            ],
        },
        // Bard Archetype ~ Detective -- apg_abilities_class.lst:1621
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Detective",
            subject: "Bard",
            archetype_name: "Detective",
            description: Some("Piecing together clues and catching the guilty with sheer cleverness, the detective is skilled at divining the truth."),
            source_page: Some("p.81"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Detective],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardInspireCourage,TYPE.BardInspireGreatness,TYPE.BardInspireHeroics,TYPE.BardBardicKnowledge,TYPE.BardWellVersed,TYPE.BardVersatilePerformance]"]),
            replaces: Some(&["BardInspireCourage", "BardInspireGreatness", "BardInspireHeroics", "BardBardicKnowledge", "BardWellVersed", "BardVersatilePerformance"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Detective ~ Careful Teamwork", at_level: 1, description: Some("A detective uses performance to keep allies coordinated, alert, and ready for action. All allies within 30 feet gain a +%1 bonus on Initiative checks, Perception, and Disable Device checks for 1 hour. They also gain a +%1 insight bonus on Reflex saves and to AC against traps and when they are flat-footed. These bonuses increase by +1 at 5th level and every six levels thereafter. Using this ability requires 3 rounds of continuous performance, and the targets must be able to see and hear the bard throughout the performance. This ability is language-dependent and requires visual and audible components. This performance replaces inspire courage.|(BardLVL+1)/6"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Detective ~ True Confession", at_level: 9, description: Some("At 9th level, a detective can use performance to trick a creature into revealing its secrets. Using this ability requires a successful Sense Motive check to see through a Bluff or notice mental compulsion. After 3 continuous rounds of performance, the target must make a Will save DC %1 (DC 10 + 1/2 the bard's level + the bard's Cha modifier). Success renders the target immune to this power for 24 hours. On a failed save, a liar inadvertently reveals the lie and the truth behind it. A creature under a charm or compulsion reveals the nature of its enchantment and who placed it (if the creature knows) and gains a new saving throw to break free from the enchantment. This ability is language-dependent and requires audible components. Using this power requires only 2 rounds of performance at 15th level, and 1 round of performance at 20th level. This performance replaces inspire greatness.|10+CHA+(BardLVL/2)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Detective ~ Show Yourselves", at_level: 15, description: Some("At 15th level, a detective can use performance to compel creatures to reveal themselves when hiding. All enemies within 30 feet must make a Will save DC %1 (DC 10 + 1/2 the bard's level + the bard's Cha modifier). If they fail, they must cease using Stealth, unlock and open doors between themselves and the detective, and dismiss, suppress, or dispel if necessary magical effects that grant invisibility or any other form of concealment from the detective. As long as they can hear the performance, affected creatures may not attack or flee until they have eliminated every such effect, though they are freed from this compulsion immediately if attacked. Creatures in the area must make this save each round the bard continues his performance. This ability is language-dependent and requires audible components. This performance replaces inspire heroics.|10+CHA+(BardLVL/2)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Detective ~ Eye for Detail", at_level: 1, description: Some("A detective gains a bonus equal to half his level on Knowledge (local), Perception, and Sense Motive checks, as well as Diplomacy checks to gather information (minimum +1). This ability replaces bardic knowledge.|min(1,BardLVL/2)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Detective ~ Arcane Insight", at_level: 2, description: Some("At 2nd level, a detective can find and disable magical traps, like a rogue's trapfinding ability. In addition, he gains a +4 bonus on saving throws made against illusions and a +4 bonus on caster level checks and saving throws to see through disguises and protections against divination (such as magic aura, misdirection, and nondetection). This ability replaces well-versed."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Detective ~ Arcane Investigation", at_level: 1, description: Some("In addition, a detective's class spell list includes the following: 1st-detect chaos/evil/law/ good; 2nd-zone of truth; 3rd-arcane eye, speak with dead, speak with plants; 4th-discern lies; 5th-prying eyes, stone tell; 6th-discern location, find the path, greater prying eyes, moment of prescience. A detective may add one of these spells or any divination spell on the bard spell list to his list of spells known at 2nd level and every four levels thereafter. This ability replaces versatile performance."), benefit: None },
            ],
        },
        // Bard Archetype ~ Magician -- apg_abilities_class.lst:1622
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Magician",
            subject: "Bard",
            archetype_name: "Magician",
            description: Some("A magician dabbles in performance, but sees it as a means to tap into universal energies and channel them."),
            source_page: Some("p.82"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Magician],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardInspireCourage,TYPE.BardDirgeOfDoom,TYPE.BardFrighteningTune,TYPE.BardCounterSong,TYPE.BardWellVersed,TYPE.BardVersatilePerformance,TYPE.BardLoreMaster,TYPE.BardJackOfAllTrades]"]),
            replaces: Some(&["BardInspireCourage", "BardDirgeOfDoom", "BardFrighteningTune", "BardCounterSong", "BardWellVersed", "BardVersatilePerformance", "BardLoreMaster", "BardJackOfAllTrades", "BardBardicKnowledge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Improved Counterspell", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Magician ~ Dweomercraft", at_level: 1, description: Some("A magician can use performance to manipulate magical energies. Allies of the magician gain a +%1 bonus on caster level checks, concentration checks, and attack rolls with spells and spell-like abilities. This bonus increases by +1 at 5th level and every six levels thereafter. This ability relies on visual and audible components. It replaces inspire courage.|(BardLVL/5)+1"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Magician ~ Spell Suppression", at_level: 8, description: Some("A 8th level, a magician can use performance to counter the spells of his foes. Once the bard begins using this performance, he tracks the number of rounds it has been in use. While performing, as an immediate action, he can attempt to counter any spell that he can identify using Spellcraft, so long as that spell's level is equal or less than the total number of rounds he has been performing spell suppression. The attempt to counter the spell is made as if using dispel magic, using the bard's level as the caster level. If successful, the bardic performance immediately ends. This ability requires audible components. This performance replaces dirge of doom."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Magician ~ Metamagic Mastery", at_level: 14, description: Some("At 14th level, a magician can use performance to apply a metamagic feat to a spell he is about to cast without increasing the casting time. The bard must still expend a higher-level slot to cast this spell. This causes the performance to immediately end. This ability requires audible components. This performance replaces frightening tune."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Magician ~ Magical Talent", at_level: 1, description: Some("A magician gains a bonus [+%1] equal to half his level on Knowledge (arcana), Spellcraft, and Use Magic Device checks. This ability replaces bardic knowledge. Improved Counterspell: A magician gains Improved Counterspell as a bonus feat. This ability replaces countersong.|BardLVL/2"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Magician ~ Extended Performance", at_level: 2, description: Some("At 2nd level, a magician can extend the duration of bardic performance after he stops concentrating by sacrificing a spell slot as a swift action. The performance effect lingers for 1 extra round per level of the spell. Only one spell may be sacrificed per performance, and performance types that take affect after a specific number of rounds cannot be extended. This ability replaces well-versed."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Magician ~ Expanded Repertoire", at_level: 2, description: Some("At 2nd level and every four levels thereafter, a magician can add one spell to his spells known from the spell list of any arcane spellcasting class. The spell must be of a level he can cast. This ability replaces versatile performance."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Magician ~ Arcane Bond", at_level: 5, description: Some("At 5th level, a magician gains the arcane bond ability as a wizard. He may not choose a familiar or a weapon as a bonded item. This ability replaces lore master."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Magician ~ Wand Mastery", at_level: 10, description: Some("At 10th level, when a magician uses a wand containing a spell on his spell list, he uses his Charisma bonus to set the wand's save DC. At 16th level, when using such a wand, he uses his caster level in place of the wand's caster level. This ability replaces jack of all trades."), benefit: None },
            ],
        },
        // Bard Archetype ~ Sandman -- apg_abilities_class.lst:1623
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Sandman",
            subject: "Bard",
            archetype_name: "Sandman",
            description: Some("Combining performance with stealth, trickery, and guile, the sandman uses cleverness to keep others off-balance."),
            source_page: Some("p.82"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Sandman],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardInspireCourage,TYPE.BardSuggestion,TYPE.BardInspireGreatness,TYPE.BardInspireHeroics,TYPE.BardMassSuggestion,TYPE.BardDeadlyPerformance,TYPE.BardBardicKnowledge,TYPE.BardVersatilePerformance,TYPE.BardInspireCompetence,TYPE.BardLoreMaster]"]),
            replaces: Some(&["BardInspireCourage", "BardSuggestion", "BardInspireGreatness", "BardInspireHeroics", "BardMassSuggestion", "BardDeadlyPerformance", "BardBardicKnowledge", "BardVersatilePerformance", "BardInspireCompetence", "BardLoreMaster"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Sandman ~ Stealspell", at_level: 1, description: Some("A sandman can use performance to steal spells from his foes and add them to his list of spells known. Once the performance is started, the bard can steal a prepared spell or a spell known from another creature with a touch attack as a standard action. The target receives a Will save DC %1 (DC 10 + the 1/2 bard's level + the bard's Cha bonus) to negate the effect. The sandman may choose a spell to steal, but if the target does not possess the spell, the bardic performance immediately ends. Otherwise the spell stolen is random, but it is always of the highest level that the bard can cast, if possible. The target loses the prepared spell or spell known and the sandman adds it to his list of spells known for as long as the performance continues, after which it reverts to the original recipient. While stolen, the bard can cast the spell using his available spell slots. This use does not consume the stolen spell. If the bard steals another spell while a spell is stolen, the previous spell immediately reverts to its original owner. This ability requires visual components. This performance replaces inspire courage.|10+CHA+(BardLVL/2)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sandman ~ Slumber Song", at_level: 6, description: Some("At 6th level, a sandman can use his performance to cause a creature he has already fascinated to fall asleep (as deep slumber, but with no HD limit). Otherwise, this ability functions like suggestion. This performance replaces suggestion."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sandman ~ Dramatic Subtext", at_level: 9, description: Some("At 9th level, a sandman can use bardic performance to cast spells without obvious visual or audible components while retaining the spell's normal effects. Observers must succeed at a Perception check opposed by a sandman's Sleight of Hand check to notice that the sandman is the source of the spellcasting (though spellcasting still provokes attacks of opportunity). The bard must use this performance for at least 2 rounds before casting a spell; otherwise he is automatically detected and the performance ends. This ability replaces inspire greatness."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sandman ~ Greater Stealspell", at_level: 15, description: Some("A sandman's stealspell power improves at 15th level. When a target fails a save against his stealspell performance, the sandman discovers its spell resistance (if any) and all spells it has prepared or knows. He can then choose which spell to steal. The sandman may forgo stealing a spell and instead reduce the target's SR by an amount equal to half his bard level and gain that amount of spell resistance for as long as he continues performing. If he steals additional spell resistance, it stacks with previously stolen SR. If he steals a spell or ceases performing, the spell resistance immediately reverts back to its owner. This performance replaces inspire heroics."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sandman ~ Mass Slumber Song", at_level: 18, description: Some("At 18th level, a sandman can use slumber song to affect any number of fascinated creatures within 30 feet. Otherwise, this ability functions like mass suggestion. This performance replaces mass suggestion."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sandman ~ Spell Catching", at_level: 20, description: Some("At 20th level, a sandman who saves against a spell or spell-like ability that targets only him (not including area spells) may use bardic performance as an immediate action. He must attempt a caster level check (DC 10 + the spell's original caster level). If it succeeds, the sandman can absorb the spell effect without harm and immediately recast that spell (using the original caster's level and save DC) or any spell he knows of that level or lower. Using this ability consumes a number of rounds of bardic performance equal to the spell's level, even if the check fails. This performance replaces deadly performance."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sandman ~ Master of Deception", at_level: 1, description: Some("A sandman gains a bonus [+%1] equal to half his level on Bluff, Sleight of Hand, and Stealth checks. He may also disarm magical traps with Disable Device as a rogue's trapfinding ability. This ability replaces bardic knowledge.|BardLVL/2"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sandman ~ Sneakspell", at_level: 2, description: Some("At 2nd level, a sandman adds +1 to the save DC of spells and bardic performance against opponents who are denied their Dex bonus. This increases to +2 at 10th level and +3 at 18th level. In addition, at 6th level he gains a +2 bonus on caster level checks to overcome spell resistance against such foes, and this bonus increases to +4 at 14th level. This ability replaces versatile performance."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sandman ~ Trap Sense", at_level: 3, description: Some("At 3rd level, a sandman gains a +%1 bonus on Reflex saves against traps and a +%1 dodge bonus to AC against traps. These bonuses increase by +1 every three levels after 3rd. This ability replaces inspire competence.|BardLVL/3"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sandman ~ Sneak Attack", at_level: 5, description: Some("At 5th level, a sandman inflicts +%1d6 points of damage against targets within 30 feet that he flanks or that are denied their Dex bonus to AC against him. This damage increases by +1d6 every five levels after 5th. This ability replaces lore master.|(BardLVL/5)+1"), benefit: None },
            ],
        },
        // Bard Archetype ~ Savage Skald -- apg_abilities_class.lst:1624
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Savage Skald",
            subject: "Bard",
            archetype_name: "Savage Skald",
            description: Some("Far from civilization, furious tribes have their own war-singers, work-chanters, and lore-keepers, savaging enemies with song and sword alike."),
            source_page: Some("p.84"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Savage Skald],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardFascinate,TYPE.BardSuggestion,TYPE.BardJackOfAllTrades,TYPE.BardSoothingPerformance,TYPE.BardMassSuggestion]"]),
            replaces: Some(&["BardFascinate", "BardSuggestion", "BardJackOfAllTrades", "BardSoothingPerformance", "BardMassSuggestion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Savage Skald ~ Inspiring Blow", at_level: 1, description: Some("A savage skald roars his war-cries with each telling blow. When he confirms a critical hit, he can start this performance as an immediate action (ending any other performances). He gains temporary hit points equal to his Charisma modifier (if positive), and all allies within 30 feet gain a +1 morale bonus on their next attack roll prior to the start of his next turn. These temporary hit points remain until the bard ends his performance. This performance replaces fascinate."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Savage Skald ~ Incite Rage", at_level: 6, description: Some("At 6th level, a savage skald can induce a furious rage in one creature within 30 feet. This effect functions as a rage spell that lasts as long as the target can hear the bard's performance; however, unwilling creatures can be affected if they fail a Will save DC %1 (DC 10 + 1/2 the bard's level + the bard's Cha modifier). Success renders the target immune to this power for 24 hours. The bard cannot target himself with this ability. If the target has the rage class feature, it can instead immediately rage and stay in this rage without consuming rounds of rage per day as long as the bard continues performing. This mind-affecting effect requires audible components. This performance replaces suggestion.|10+CHA+(BardLVL/2)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Savage Skald ~ Song of the Fallen", at_level: 10, description: Some("At 10th level, a savage skald can duplicate the effect of a horn of Valhalla. This effect requires 10 continuous rounds of performance and summons barbarians as a silver horn at 10th level, as a brass horn at 13th level, as a bronze horn at 16th level, and as an iron horn at 19th level. The warriors remain only as long as the bard continues his performance. This ability requires audible components. This performance replaces jack of all trades."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Savage Skald ~ Berserkergang", at_level: 12, description: Some("At 12th level, a savage skald can inspire a rapturous battle trance that suppresses pain, stunning, and fear effects for one creature, plus one additional creature per three levels after 12th. Affected creatures also gain DR 5/- (DR 10/- against nonlethal damage); this benefit stacks with the damage reduction class ability of barbarians. This mind-affecting ability requires audible components. This performance replaces soothing performance."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Savage Skald ~ Battle Song", at_level: 18, description: Some("At 18th level, a savage skald can affect all allies within 30 feet when using performance to incite rage. This performance replaces mass suggestion."), benefit: None },
            ],
        },
        // Bard Archetype ~ Sea Singer -- apg_abilities_class.lst:1625
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Sea Singer",
            subject: "Bard",
            archetype_name: "Sea Singer",
            description: Some("The sea singer calls the blue waters his home, and is much in demand among sea captains wishing good fortune for their crew and hull as they ply the tradewinds far and wide."),
            source_page: Some("p.84"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Sea Singer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardCounterSong,TYPE.BardCounterSong,TYPE.BardSuggestion,TYPE.BardMassSuggestion,TYPE.BardBardicKnowledge,TYPE.BardVersatilePerformance,TYPE.BardWellVersed,TYPE.BardInspireCompetence]"]),
            replaces: Some(&["BardCounterSong", "BardCounterSong", "BardSuggestion", "BardMassSuggestion", "BardBardicKnowledge", "BardVersatilePerformance", "BardWellVersed", "BardInspireCompetence"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Sea Singer ~ Sea Shanty", at_level: 1, description: Some("A sea singer learns to counter seasickness and exhaustion during long sea voyages. Each round of a sea shanty, he makes a Perform skill check. Allies within 30 feet (including the sea singer) may use his Perform check in place of a saving throw against becoming exhausted, fatigued, nauseated, or sickened; if already under such an effect, a new save is allowed each round of the sea shanty, using the bard's Perform check for the save. A sea shanty has no effect on instantaneous effects or effects that do not allow saves. This ability requires audible components. This performance replaces countersong."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sea Singer ~ Still Water", at_level: 3, description: Some("At 3rd level, a sea singer can use performance to calm rough waters within 30 feet, reducing the DC for Profession (sailor) and Swim checks, as well as for Acrobatics and Climb checks aboard ship, by an amount equal to the bard's level (to a minimum of DC 10) for as long he continues to perform. He can extend this duration to 1 hour by playing for 10 consecutive rounds. This ability requires audible components. This performance replaces inspire competence."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sea Singer ~ Whistle the Wind", at_level: 6, description: Some("A sea singer of 6th level or higher can use performance to create a gust of wind. This wind lasts for as long as he continues his performance. He can extend this duration to 1 minute by playing for 5 consecutive rounds. This performance replaces suggestion."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sea Singer ~ Call the Storm", at_level: 18, description: Some("At 18th level, a sea singer can use performance to duplicate control water, control weather, control winds, or storm of vengeance, using his bard level as the caster level. Using this ability requires 1 round of continuous performance per level of the spell (as if he were a druid). These effects continue for as long as the bard continues performing (the effects of control weather happen immediately), but not longer than the spell's normal duration. This performance replaces mass suggestion."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sea Singer ~ World Traveler", at_level: 1, description: Some("A sea singer gains a bonus equal to half his bard level on Knowledge (geography), Knowledge (local), Knowledge (nature), and Linguistics checks. He can reroll a check against one of these skills, but must take the result of the second roll even if it is worse. He can reroll one additional time per day at 5th level and every five levels thereafter. This ability replaces bardic knowledge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sea Singer ~ Familiar", at_level: 2, description: Some("At 2nd level, a sea singer acquires an exotic pet, a monkey or parrot (treat as raven), that gains abilities as a wizard's familiar, using the sea singer's bard level as his wizard level. This ability replaces versatile performance."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sea Singer ~ Sea Legs", at_level: 2, description: Some("At 2nd level, a sea singer gains a +4 bonus on saving throws against air and water effects and effects that would cause the sea singer to slip, trip, or otherwise be knocked prone. He gains a +2 bonus to CMD against grapple, overrun, and trip. This ability replaces well-versed."), benefit: None },
            ],
        },
        // Bard Archetype ~ Street Performer -- apg_abilities_class.lst:1626
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Street Performer",
            subject: "Bard",
            archetype_name: "Street Performer",
            description: Some("Whether acrobat, troubadour, or thespian, the street performer mixes with the masses, singing for his supper."),
            source_page: Some("p.85"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Street Performer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardInspireCourage,TYPE.BardInspireCompetence,TYPE.BardInspireGreatness,TYPE.BardInspireHeroics,TYPE.BardCountersong,TYPE.BardBardicKnowledge,TYPE.BardLoreMaster]"]),
            replaces: Some(&["BardInspireCourage", "BardInspireCompetence", "BardInspireGreatness", "BardInspireHeroics", "BardCountersong", "BardBardicKnowledge", "BardLoreMaster"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Street Performer ~ Disappearing Act", at_level: 1, description: Some("A street performer can use performance to divert attention from an ally. All creatures within 30 feet that fail a Will save DC %1 (DC 10 + 1/2 the bard's level + the bard's Cha modifier) treat one creature chosen by the bard as if it were invisible. This performance affects one additional creature at 5th level and every 6 levels thereafter. If the targets take any action that would cause them to become visible, they become visible to everyone. The bard cannot use this ability on himself. This ability is a mind-affecting effect that requires visual components. This performance replaces inspire courage.|10+CHA+(BardLVL/2)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Street Performer ~ Harmless Performer", at_level: 3, description: Some("At 3rd level, a street performer can use performance to appear meek and unworthy of being attacked. While using this performance, whenever an enemy targets the street performer, the enemy must succeed at a Will save DC %1 (DC 10 + 1/2 the bard's level + the bard's Cha bonus) or be unable to attack the bard this round, as per sanctuary. The enemy loses the attack that targeted the bard, but may spend additional attacks targeting other creatures. If the opponent was targeting the street performer with a spell, it must succeed at a concentration check at the same DC or lose the spell. If this check succeeds, it may target another creature with the spell instead. This mind-affecting ability requires audible or visual components. This performance replaces inspire competence.|10+CHA+(BardLVL/2)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Street Performer ~ Madcap Prank", at_level: 9, description: Some("At 9th level, a street performer can use performance to discomfit a target within 30 feet, causing its clothing to become tangled, its headgear to fall down over its eyes, or even causing it to slip and fall or otherwise be made to appear a fool. The target must make a Reflex save DC %1 (DC 10 + 1/2 the bard's level + the bard's Cha modifier) each round that it hears or sees the performance, or it takes one of the following random effects each round: 1-blinded, 2-dazzled, 3-deafened, 4-entangled, 5-fall prone, 6-nauseated. Each effect lasts 1 round. This performance replaces inspire greatness.|10+CHA+(BardLVL/2)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Street Performer ~ Slip through the Crowd", at_level: 15, description: Some("At 15th level, a street performer's disappearing act enables affected creatures to move through crowd squares and enemy-occupied squares without impediment. Affected creatures are treated as if having greater invisibility, but enemies gain a new saving throw to notice them each time they are attacked. This performance replaces inspire heroics."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Street Performer ~ Gladhanding", at_level: 1, description: Some("A street performer earns double the normal amount of money from Perform checks. As a standard action, he may use a Bluff check in place of a Diplomacy check to improve a creature's attitude for 1 minute, after which its attitude becomes one step worse than originally. This ability replaces countersong."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Street Performer ~ Streetwise", at_level: 1, description: Some("A street performer gains a bonus equal to half his level on Bluff, Disguise, Knowledge (local), and Sleight of Hand checks, Diplomacy or Intimidate checks made to influence crowds, and Diplomacy checks to gather information (minimum +1). This replaces bardic knowledge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Street Performer ~ Quick Change", at_level: 5, description: Some("At 5th level, a street performer can don a disguise as a standard action by taking a -5 penalty on his check. He can take 10 on Bluff and Disguise checks and use Bluff to create a diversion to hide as a swift action. He can take 20 on a Bluff or Disguise check once per day, plus one time per six levels beyond 5th. This ability replaces lore master."), benefit: None },
            ],
        },
        // Druid Archetype ~ Aquatic Druid -- apg_abilities_class.lst:1889
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Aquatic Druid",
            subject: "Druid",
            archetype_name: "Aquatic Druid",
            description: Some("Shepherds of the lakes and seas, aquatic druids guard ecosystems ranging from shallows streams to deep ocean trenches, ministering to their residents and communing with the tides."),
            source_page: Some("p.98"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Aquatic Druid],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidWoodlandStride,TYPE.DruidTracklessStep,TYPE.DruidNaturesLure,TYPE.DruidVenomImmunity,TYPE.DruidThousandFaces,TYPE.DruidWildShape]"]),
            replaces: Some(&["DruidWoodlandStride", "DruidTracklessStep", "DruidNaturesLure", "DruidVenomImmunity", "DruidThousandFaces", "DruidWildShape"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Aquatic Druid ~ Wild Empathy", at_level: 1, description: Some("An aquatic druid's wild empathy functions only on creatures that have a swim speed or the aquatic or water subtype; however, she can improve the attitude of any such creature with Intelligence 2 or less regardless of type, including mindless creatures."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Aquatic Druid ~ Aquatic Adaptation", at_level: 2, description: Some("At 2nd level, an aquatic druid gains an insight bonus on Initiative checks and Knowledge (geography), Perception, Stealth, Survival, and Swim checks equal to 1/2 her druid level in aquatic terrain, and she cannot be tracked such environments. This ability replaces woodland stride."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Aquatic Druid ~ Natural Swimmer", at_level: 3, description: Some("At 3rd level, an aquatic druid gains a swim speed equal to half her land speed. This ability replaces trackless step."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Aquatic Druid ~ Resist Oceans Fury", at_level: 4, description: Some("At 4th level, an aquatic druid gains a +4 bonus on saving throws against spells of the water type or the exceptional or supernatural abilities of creatures with the aquatic or water subtype. This ability replaces resist nature's lure."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Aquatic Druid ~ Wild Shape", at_level: 6, description: Some("An aquatic druid gains this ability at 6th level, except that her effective druid level for the ability is equal to her druid level - 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Aquatic Druid ~ Seaborn", at_level: 9, description: Some("At 9th level, an aquatic druid gains the aquatic subtype, the amphibious trait, and a swim speed equal to her land speed. She also can endure cold climate effects as if using endure elements. This ability replaces venom immunity."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Aquatic Druid ~ Deep Diver", at_level: 13, description: Some("At 13th level, an aquatic druid gains DR/slashing or piercing equal to 1/2 her level. This damage reduction also applies against spells and spell-like abilities that inflict damage by grappling or crushing (e.g., black tentacles, crushing hand). She never takes pressure damage from deep water. This ability replaces a thousand faces."), benefit: None },
            ],
        },
        // Druid Archetype ~ Arctic Druid -- apg_abilities_class.lst:1890
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Arctic Druid",
            subject: "Druid",
            archetype_name: "Arctic Druid",
            description: Some("An arctic druid watches over the stark landscape of the far frozen reaches of the world, tending the stunted and rugged life that ekes out its survival in the least habitable climes."),
            source_page: Some("p.98"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Arctic Druid],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidWoodlandStride,TYPE.DruidTracklessStep,TYPE.DruidNaturesLure,TYPE.DruidVenomImmunity,TYPE.DruidThousandFaces,TYPE.DruidWildShape]"]),
            replaces: Some(&["DruidWoodlandStride", "DruidTracklessStep", "DruidNaturesLure", "DruidVenomImmunity", "DruidThousandFaces", "DruidWildShape"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Arctic Druid ~ Arctic Native", at_level: 2, description: Some("At 2nd level, an arctic druid gains a bonus on Initiative checks and Knowledge (geography), Perception, Stealth, and Survival checks equal to 1/2 her druid level in cold or icy terrain, and she cannot be tracked in cold or icy terrain. This ability replaces woodland stride."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Arctic Druid ~ Icewalking", at_level: 3, description: Some("At 3rd level, an arctic druid suffers no penalty to speed or on Acrobatics, Climb, or Stealth checks in snowy or icy terrain or weather conditions and can walk across snow crusts or thin ice without breaking through. This ability replaces trackless step."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Arctic Druid ~ Arctic Endurance", at_level: 4, description: Some("At 4th level, an arctic druid ignores the effects of a cold climate as if using endure elements. She is also immune to being dazzled. This ability replaces resist nature's lure."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Arctic Druid ~ Wild Shape", at_level: 6, description: Some("An arctic druid gains this ability at 6th level, except that her effective druid level for this ability is equal to her druid level - 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Arctic Druid ~ Snowcaster", at_level: 9, description: Some("At 9th level, an arctic druid can see normally in ice storm, sleet storm, or similar natural snowstorms. In addition, she can prepare any druid spell with the fire subtype as a cold spell, with an identical effect but inflicting cold damage instead of fire damage. This ability replaces venom immunity."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Arctic Druid ~ Flurry Form", at_level: 13, description: Some("At 13th level, an arctic druid can assume the form of a swirling column of snow equivalent to gaseous form at will. While in this form, she gains a circumstance bonus on Stealth checks made in cold terrain equal to her druid level. This ability replaces a thousand faces."), benefit: None },
            ],
        },
        // Druid Archetype ~ Bear Shaman -- apg_abilities_class.lst:1900
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Bear Shaman",
            subject: "Druid",
            archetype_name: "Bear Shaman",
            description: Some("A shaman with this focus calls upon the mighty bear, titan of the woodlands and mountains, a paragon of strength and ferocity, and yet also a quiet protector rich in wisdom."),
            source_page: Some("p.102"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Bear Shaman],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidThousandFaces,TYPE.DruidVenomImmunity,TYPE.DruidWildShape,TYPE.DruidNatureBond]"]),
            replaces: Some(&["DruidThousandFaces", "DruidVenomImmunity", "DruidWildShape"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Bear Shaman ~ Nature Bond", at_level: 1, description: Some("A bear shaman who chooses an animal companion must select a bear. If choosing a domain, the bear shaman must choose from the Animal, Earth, Protection, and Strength domains."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bear Shaman ~ Wild Empathy", at_level: 1, description: Some("A bear shaman can use wild empathy with bears and wolverines as a full-round action with a +4 bonus."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bear Shaman ~ Totem Transformation", at_level: 2, description: Some("At 2nd level, a bear shaman may adopt an aspect of the bear while retaining her normal form. She gains one of the following bonuses: movement (+10 enhancement bonus to land speed, +4 racial bonus on Swim checks), senses (low-light vision, scent), toughness (+2 natural armor bonus to AC, Endurance feat), or natural weapons (bite [1d6] and 2 claws [1d4] for a Medium shaman, +2 to CMB on grapple checks). While using totem transformation, the bear shaman may speak normally and can cast speak with animals (mammals only) at will. Using this ability is a standard action at 2nd level, a move action at 7th level, and a swift action at 12th level. The bear shaman can use this ability for a number of minutes per day equal to her druid level. These minutes do not need to be consecutive, but they must be used in 1-minute increments. This is a polymorph effect and cannot be used while the druid is using another polymorph effect, such as wild shape."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bear Shaman ~ Totemic Summons", at_level: 5, description: Some("At 5th level, a bear shaman may cast summon nature's ally as a standard action when summoning bears, and summoned bears gain temporary hit points equal to her druid level. She can apply the young template to any bear to reduce the level of the summoning spell required by one. She can also increase the level of summoning required by one in order to apply either the advanced or the giant template, or increase it by two to apply both the advanced and giant templates."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bear Shaman ~ Wild Shape", at_level: 6, description: Some("At 6th level, a bear shaman's wild shape ability functions at her druid level - 2. If she takes on the form of a bear, she instead uses her druid level + 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bear Shaman ~ Bonus Feat", at_level: 9, description: Some("At 9th level and every 4 levels thereafter, a bear shaman gains one of the following bonus feats: Diehard, Endurance, Great Fortitude, Improved Great Fortitude, Toughness. She must meet the prerequisites for these bonus feats."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Druid ~ Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Progression", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Times", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Wild Shape", at_level: 6, description: None, benefit: None },
            ],
        },
        // Druid Archetype ~ Blight Druid -- apg_abilities_class.lst:1891
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Blight Druid",
            subject: "Druid",
            archetype_name: "Blight Druid",
            description: Some("The devoted servants of nature corrupted, ruined, and destroyed, blight druids are the caretakers of lands ravaged by natural disaster. While some are devoted to reforming and reclaiming lands despoiled by the ravages of civilization, others seek out the more rapacious violence inherent in nature and feed the creeping rot and decay that brings an end to all things."),
            source_page: Some("p.98"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Blight Druid],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidThousandFaces,TYPE.DruidVenomImmunity,TYPE.DruidTracklessStep,TYPE.DruidResistNaturesLure,TYPE.DruidWildEmpathy,TYPE.DruidNatureBond]"]),
            replaces: Some(&["DruidThousandFaces", "DruidVenomImmunity", "DruidTracklessStep", "DruidNatureBond", "DruidResistNaturesLure", "DruidWildEmpathy"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Blight Druid ~ Nature Bond", at_level: 1, description: Some("A blight druid may not bond with an animal companion, but may either call a familiar as a wizard of her druid level or select from the Darkness, Death, and Destruction domains in addition to those normally available."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Blight Druid ~ Vermin Empathy", at_level: 1, description: Some("A blight druid can improve the attitude of vermin as a normal druid can with animals. Vermin have a starting attitude of unfriendly. The blight druid can also improve the attitude of animals and mindless undead creatures that were formerly animals, but she takes a -4 penalty on the check unless the animal or undead has a disease special attack."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Blight Druid ~ Miasma", at_level: 5, description: Some("Starting at 5th level, if a blight druid is adjacent to a creature at the beginning of its turn, the creature must succeed at a Fortitude save with a DC of 10 + 1/2 the druid's level + the druid's Wisdom modifier or become sickened for 1 round. A creature of the animal, fey, or plant type that fails its save is nauseated for 1 round and sickened for 1 minute thereafter. If the creature makes its save, it is immune to this effect for 24 hours, as are creatures immune to disease."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Blight Druid ~ Blightblooded", at_level: 9, description: Some("At 9th level, a blight druid gains immunity to all diseases, including natural and supernatural diseases. She also becomes immune to effects that would cause her to become sickened or nauseated."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Blight Druid ~ Plaguebearer", at_level: 13, description: Some("Starting at 13th level, any creature that strikes a blight druid with a touch attack, unarmed strike, or natural weapon must succeed at a Fortitude save with a DC of 10 + 1/2 the druid's level + the druid's Wisdom modifier or contract a disease, as the contagion spell. If the creature makes its save, it is immune to this effect for 24 hours."), benefit: None },
            ],
        },
        // Druid Archetype ~ Cave Druid -- apg_abilities_class.lst:1892
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Cave Druid",
            subject: "Druid",
            archetype_name: "Cave Druid",
            description: Some("Far from the green fields of the world above lies a lightless expanse beneath the surface. This darkling fairyland is not without beauty and natural wonders of its own, and a few druids seek to preserve this hidden realm and purge it of the fell horrors that creep up from below."),
            source_page: Some("p.99"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Cave Druid],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidClassSkills,TYPE.DruidNatureSense,TYPE.DruidTracklessStep,TYPE.DruidNatureBond,TYPE.DruidWildEmpathy,TYPE.DruidResistNaturesLure,TYPE.DruidWildShape,TYPE.DruidWoodlandStride]"]),
            replaces: Some(&["DruidClassSkills", "DruidNatureSense", "DruidTracklessStep", "DruidNatureBond", "DruidWildEmpathy", "DruidResistNaturesLure", "DruidWildShape", "DruidWoodlandStride"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Cave Druid ~ Cavesense", at_level: 1, description: Some("A underground druid adds Knowledge (dungeoneering) rather than Knowledge (geography) as a class skill and gains a +2 bonus on Knowledge (dungeoneering) and Survival skill checks. This ability replaces the nature sense ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cave Druid ~ Nature Bond", at_level: 1, description: Some("A cave druid may select the Darkness domain in addition to the choices normally allowed, but may not select the Air or Weather domains."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cave Druid ~ Wild Empathy", at_level: 1, description: Some("A cave druid can influence oozes, rather than magical beasts, with a -4 penalty on her wild empathy check."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cave Druid ~ Tunnelrunner", at_level: 2, description: Some("At 2nd level, a cave druid can move through areas of rubble or narrow passages that require squeezing at her normal movement rate and without penalty. This ability replaces woodland stride."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cave Druid ~ Lightfoot", at_level: 3, description: Some("At 3rd level, a cave druid cannot be detected with tremorsense. This ability replaces trackless step."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cave Druid ~ Resist Subterranean Corruption", at_level: 4, description: Some("At 4th level, a cave druid gains a +2 bonus on saves against exceptional, supernatural, and spell-like abilities of oozes and aberrations. This ability replaces resist nature's lure."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cave Druid ~ Wild Shape", at_level: 6, description: Some("A cave druid gains this ability at 6th level, except that her effective druid level for this ability is equal to her druid level - 2. She cannot use wild shape to adopt a plant form. At 10th level, the cave druid can assume the form of a Small or Medium ooze as if using beast shape III, and at 12th level that of a Tiny or Large ooze as if using beast shape IV (treating the ooze as if it were a magical beast without a natural armor bonus). When in ooze form, the cave druid has no discernible anatomy and is immune to poison, sneak attacks, and critical hits."), benefit: None },
            ],
        },
        // Druid Archetype ~ Desert Druid -- apg_abilities_class.lst:1893
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Desert Druid",
            subject: "Druid",
            archetype_name: "Desert Druid",
            description: Some("Not all climates are verdant paradises, but even in the sere deserts there is life-though often hidden from the sun and rarely friendly-and raw, desolate beauty. Here desert druids come to pay homage, protect and maintain the few habitable locales, and witness nature's majesty in all its burning, merciless glory."),
            source_page: Some("p.99"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Desert Druid],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidWoodlandStride,TYPE.DruidTracklessStep,TYPE.DruidResistNaturesLure,TYPE.DruidWildShape,TYPE.DruidVenomImmunity,TYPE.DruidThousandFaces]"]),
            replaces: Some(&["DruidWoodlandStride", "DruidTracklessStep", "DruidResistNaturesLure", "DruidWildShape", "DruidVenomImmunity", "DruidThousandFaces"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Desert Druid ~ Desert Native", at_level: 2, description: Some("At 2nd level, a desert druid gains a bonus on Initiative checks and Knowledge (geography), Perception, Stealth, and Survival checks equal to 1/2 her druid level in desert terrain, and she cannot be tracked in such environments. This ability replaces woodland stride."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Desert Druid ~ Sandwalker", at_level: 3, description: Some("At 3rd level, a desert druid suffers no penalty to speed or on Acrobatics or Stealth checks when moving through sandy or desert terrain. This ability replaces trackless step."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Desert Druid ~ Desert Endurance", at_level: 4, description: Some("At 4th level, a desert druid ignores the effects of a hot climate as if using endure elements. She also has a reduced need to eat and drink, as if wearing a ring of sustenance (though normal sleep is still required). This ability replaces resist nature's lure."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Desert Druid ~ Wild Shape", at_level: 6, description: Some("A desert druid gains this ability at 6th level, except that her effective druid level for the ability is equal to her druid level - 2. A desert druid cannot use wild shape to adopt a plant form. At 10th level she can assume the form of a Small or Medium vermin, at 12th level a Tiny or Large vermin, and at 14th level a Diminutive or Huge vermin. This effect functions as beast shape IV (treating the vermin as an animal to determine its ability and natural armor modifiers)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Desert Druid ~ Shaded Vision", at_level: 9, description: Some("At 9th level, a desert druid becomes immune to blinding and dazzling effects and gains a +2 bonus on saving throws against gaze attacks and illusions of the figment and pattern subschools. This ability replaces venom immunity."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Desert Druid ~ Dunemeld", at_level: 13, description: Some("At 13th level, a desert druid can assume the form of a swirling mass of sand at will. This is equivalent to gaseous form, but the druid gains a land and burrow speed of 10 feet rather than a fly speed. While in this form, the druid gains a circumstance bonus on Stealth checks made in desert terrain equal to her druid level. This ability replaces a thousand faces."), benefit: None },
            ],
        },
        // Druid Archetype ~ Eagle Shaman -- apg_abilities_class.lst:1901
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Eagle Shaman",
            subject: "Druid",
            archetype_name: "Eagle Shaman",
            description: Some("A shaman with this totem calls upon the noble eagle, stern and proud, soaring high above the world with keen and pitiless eyes that miss nothing."),
            source_page: Some("p.102"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Eagle Shaman],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidThousandFaces,TYPE.DruidVenomImmunity,TYPE.DruidWildShape,TYPE.DruidNatureBond]"]),
            replaces: Some(&["DruidThousandFaces", "DruidVenomImmunity", "DruidWildShape"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Eagle Shaman ~ Nature Bond", at_level: 1, description: Some("An eagle shaman who chooses an animal companion must select a bird (eagle). If choosing a domain, the eagle shaman must choose from the Air, Animal, Nobility, and Weather domains."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Eagle Shaman ~ Wild Empathy", at_level: 1, description: Some("An eagle shaman can use wild empathy with birds as a full-round action with a +4 bonus."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Eagle Shaman ~ Totem Transformation", at_level: 2, description: Some("At 2nd level, an eagle shaman may adopt an aspect of the eagle while retaining her normal form. This ability functions as the bear shaman ability, but the druid may select from the following bonuses: movement (fly speed 30 feet [average], the druid must be 5th level to select this bonus), senses (low-light vision, +4 racial bonus to Perception), or natural weapons (bite [1d4], 2 talons [1d4] for a Medium shaman). While using totem transformation, the eagle shaman may speak normally and can cast speak with animals (birds only) at will."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Eagle Shaman ~ Totemic Summons", at_level: 5, description: Some("At 5th level, an eagle shaman may cast summon nature's ally as a standard action when summoning eagles, rocs, and giant eagles (added to the 4th level list), and these summoned creatures gain temporary hit points equal to her druid level. This ability otherwise functions as the bear shaman ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Eagle Shaman ~ Wild Shape", at_level: 6, description: Some("At 6th level, an eagle shaman's wild shape ability functions at her druid level - 2. If she takes on the form of an eagle or roc, she instead uses her druid level + 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Eagle Shaman ~ Bonus Feat", at_level: 9, description: Some("At 9th level and every 4 levels thereafter, an eagle shaman gains one of the following bonus feats: Flyby Attack, Improved Lightning Reflexes, Lightning Reflexes, Skill Focus (Perception), or Wind Stance. She must meet the prerequisites for these bonus feats. This ability replaces venom immunity."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Druid ~ Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Progression", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Times", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Wild Shape", at_level: 6, description: None, benefit: None },
            ],
        },
        // Druid Archetype ~ Jungle Druid -- apg_abilities_class.lst:1894
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Jungle Druid",
            subject: "Druid",
            archetype_name: "Jungle Druid",
            description: Some("The fecund jungles of the equatorial regions are rich in life and ancient tradition; druidical guardians of sacred pools, elder trees, and trembling volcanoes watch over crumbling temples and the inevitable reclamation of lost civilizations by the beating heart of nature untamed."),
            source_page: Some("p.100"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Jungle Druid],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidWoodlandStride,TYPE.DruidTracklessStep,TYPE.DruidResistNaturesLure,TYPE.DruidWildShape,TYPE.DruidThousandFaces]"]),
            replaces: Some(&["DruidWoodlandStride", "DruidTracklessStep", "DruidResistNaturesLure", "DruidWildShape", "DruidThousandFaces"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Jungle Druid ~ Jungle Guardian", at_level: 2, description: Some("At 2nd level, a jungle druid gains a bonus on Initiative checks and Climb, Knowledge (geography), Perception, Stealth, and Survival checks equal to 1/2 her druid level in jungle terrain, and she cannot be tracked in such environments. This ability replaces woodland stride."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Jungle Druid ~ Woodland Stride", at_level: 3, description: Some("A jungle druid gains this ability at 3rd level. This ability replaces trackless step."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Jungle Druid ~ Torrid Endurance", at_level: 4, description: Some("At 4th level, a jungle druid ignores the effects of a hot climate as if under the effects of endure elements. She also gains a +4 bonus on saves against disease and the exceptional abilities of animals and magical beasts. This ability replaces resist nature's lure."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Jungle Druid ~ Wild Shape", at_level: 6, description: Some("A jungle druid gains this ability at 6th level, except that her effective druid level for the ability is equal to her druid level - 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Jungle Druid ~ Verdant Sentinel", at_level: 13, description: Some("At 13th level, a jungle druid can cast tree shape at will. This ability replaces a thousand faces."), benefit: None },
            ],
        },
        // Druid Archetype ~ Lion Shaman -- apg_abilities_class.lst:1902
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Lion Shaman",
            subject: "Druid",
            archetype_name: "Lion Shaman",
            description: Some("A shaman with this totem calls upon the proud lion, imposing and majestic, the mighty leader of deadly hunters."),
            source_page: Some("p.103"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Lion Shaman],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidThousandFaces,TYPE.DruidVenomImmunity,TYPE.DruidWildShape,TYPE.DruidNatureBond]"]),
            replaces: Some(&["DruidThousandFaces", "DruidVenomImmunity", "DruidWildShape"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Lion Shaman ~ Nature Bond", at_level: 1, description: Some("A lion shaman who chooses an animal companion must select a lion. If choosing a domain, the lion shaman must choose from the Animal, Glory, Nobility, and Sun domains."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Lion Shaman ~ Wild Empathy", at_level: 1, description: Some("A lion shaman can use wild empathy with felines as a full-round action with a +4 bonus."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Lion Shaman ~ Totem Transformation", at_level: 2, description: Some("At 2nd level, a lion shaman may adopt an aspect of the lion while retaining her normal form. This ability functions as the bear shaman ability, but the druid may select from the following bonuses: movement (+20 enhancement bonus to land speed), senses (low-light vision, scent), or natural weapons (bite [1d4], 2 claws [1d4] for a Medium druid, rake, +2 CMB to grapple). While using totem transformation, the lion shaman may speak normally and can cast speak with animals (felines only) at will."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Lion Shaman ~ Totemic Summons", at_level: 5, description: Some("At 5th level, a lion shaman may cast summon nature's ally as a standard action when summoning felines, and these summoned creatures gain temporary hit points equal to her druid level. This ability otherwise functions as the bear shaman ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Lion Shaman ~ Wild Shape", at_level: 6, description: Some("At 6th level, a lion shaman's wild shape ability functions at her druid level - 2. If she takes on the form of a feline, she instead uses her druid level + 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Lion Shaman ~ Bonus Feat", at_level: 9, description: Some("At 9th level and every 4 levels thereafter, a lion shaman gains one of the following bonus feats: Dodge, Lunge, Improved Iron Will, Iron Will, or Skill Focus (Acrobatics). She must meet the prerequisites for these bonus feats. This ability replaces venom immunity."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Druid ~ Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Progression", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Times", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Wild Shape", at_level: 6, description: None, benefit: None },
            ],
        },
        // Druid Archetype ~ Mountain Druid -- apg_abilities_class.lst:1895
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Mountain Druid",
            subject: "Druid",
            archetype_name: "Mountain Druid",
            description: Some("As more and more of the soft, easy lands become cultivated and civilized, many druids look for refuge and solitude among the eternal peaks of the highest mountains."),
            source_page: Some("p.100"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Mountain Druid],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidWoodlandStride,TYPE.DruidTracklessStep,TYPE.DruidResistNaturesLure,TYPE.DruidWildShape,TYPE.DruidVenomImmunity,TYPE.DruidThousandFaces]"]),
            replaces: Some(&["DruidWoodlandStride", "DruidTracklessStep", "DruidResistNaturesLure", "DruidWildShape", "DruidVenomImmunity", "DruidThousandFaces"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Mountain Druid ~ Mountaineer", at_level: 2, description: Some("At 2nd level, a mountain druid gains a bonus on Initiative checks and Climb, Knowledge (geography), Perception, Stealth, and Survival checks equal to half her druid level in mountainous terrain, and she cannot be tracked in such an environment. This ability replaces woodland stride."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mountain Druid ~ Sure-Footed", at_level: 3, description: Some("At 3rd level, a mountain druid suffers no penalty to speed or on Acrobatics or Stealth checks when walking across steep slopes, rubble, or scree. This ability replaces trackless step."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mountain Druid ~ Spire Walker", at_level: 4, description: Some("At 4th level, a mountain druid does not lose her Dexterity bonus when climbing. A mountain druid is immune to altitude sickness and ignores the effects of a cold climate as if under the effects of endure elements. This ability replaces resist nature's lure."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mountain Druid ~ Wild Shape", at_level: 6, description: Some("A mountain druid gains this ability at 6th level, except that her effective druid level for the ability is equal to her druid level - 2. A mountain druid cannot use wild shape to adopt a plant form. However, at 12th level she can assume the form of a Large giant as if using giant form I. At 16th level, she may assume the form of a Huge giant as if using giant form II."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mountain Druid ~ Mountain Stance", at_level: 9, description: Some("At 9th level, a mountain druid gains immunity to petrification and receives a +4 bonus on saving throws or to CMD to resist any attempt to push, pull, bull rush, or drag her, or to resist any other effect that would physically move her from her position (e.g., repel wood, reverse gravity, or being blown away by high winds). This does not protect her against being tripped, grappled, or overrun. This ability replaces venom immunity."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mountain Druid ~ Mountain Stone", at_level: 13, description: Some("At 13th level, a mountain druid can transform her body into a weathered stone outcrop and back at will. This effect functions as statue. This ability replaces a thousand faces."), benefit: None },
            ],
        },
        // Druid Archetype ~ Plains Druid -- apg_abilities_class.lst:1896
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Plains Druid",
            subject: "Druid",
            archetype_name: "Plains Druid",
            description: Some("Out upon the wide and rolling prairies and savannahs, plains druids stand guard over the grasslands. These druids range far and wide, watching over nomadic tribes and wandering herds and preserving the sometimes fragile ecosystem of the wide open spaces."),
            source_page: Some("p.100"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Plains Druid],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidWoodlandStride,TYPE.DruidTracklessStep,TYPE.DruidResistNaturesLure,TYPE.DruidWildShape,TYPE.DruidVenomImmunity,TYPE.DruidThousandFaces]"]),
            replaces: Some(&["DruidWoodlandStride", "DruidTracklessStep", "DruidResistNaturesLure", "DruidWildShape", "DruidVenomImmunity", "DruidThousandFaces"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Plains Druid ~ Plains Traveler", at_level: 2, description: Some("At 2nd level, a plains druid gains a bonus on initiative checks and Knowledge (geography), Perception, Stealth, and Survival checks equal to 1/2 her druid level in plains terrain, and she cannot be tracked in such an environment. This ability replaces woodland stride."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Plains Druid ~ Run Like the Wind", at_level: 3, description: Some("At 3rd level, a plains druid gains +10 feet to her land speed when wearing light or no armor and carrying a light load, and once per hour, she may run or charge at double the normal speed for 1 round. If riding her animal companion, it gains this ability instead. This ability replaces trackless step."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Plains Druid ~ Savanna Ambush", at_level: 4, description: Some("At 4th level, a plains druid gains concealment whenever she is prone in natural surroundings, and can make Stealth checks at no penalty when prone and not moving or at -5 when crawling. A plains druid can stand up from prone as an immediate action during a surprise round. This ability replaces resist nature's lure."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Plains Druid ~ Wild Shape", at_level: 6, description: Some("A plains druid gains this ability at 6th level, except that her effective druid level for the ability is equal to her druid level - 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Plains Druid ~ Canny Charger", at_level: 9, description: Some("At 9th level, a plains druid can charge through allies' squares without difficulty (whether mounted or afoot) and can turn up to 90 degrees once during a charge, provided the last 10 feet toward the target are in a straight line. She also gains a +4 dodge bonus to AC against enemy charge attacks and a +4 bonus to damage with a readied action against a charging foe. This ability replaces venom immunity."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Plains Druid ~ Evasion", at_level: 13, description: Some("At 13th level, a plains druid gains evasion when wearing light or no armor and carrying a light load. This functions as the rogue ability of the same name. This ability replaces a thousand faces."), benefit: None },
            ],
        },
        // Druid Archetype ~ Serpent Shaman -- apg_abilities_class.lst:1903
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Serpent Shaman",
            subject: "Druid",
            archetype_name: "Serpent Shaman",
            description: Some("A shaman with this totem calls upon the cunning serpent, the stealthy deceiver who draws the weak minded in and strikes while they are unaware. Some hate its treacherous nature, while others praise its thoughtful pragmatism."),
            source_page: Some("p.103"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Serpent Shaman],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidThousandFaces,TYPE.DruidVenomImmunity,TYPE.DruidWildShape,TYPE.DruidNatureBond]"]),
            replaces: Some(&["DruidThousandFaces", "DruidVenomImmunity", "DruidWildShape"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Serpent Shaman ~ Nature Bond", at_level: 1, description: Some("A serpent shaman who chooses an animal companion must select a snake. If choosing a domain, the serpent shaman must choose from the Animal, Charm, Trickery, and Water domains."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Serpent Shaman ~ Wild Empathy", at_level: 1, description: Some("A serpent shaman can use wild empathy with reptiles as a full-round action with a +4 bonus."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Serpent Shaman ~ Totem Transformation", at_level: 2, description: Some("At 2nd level, a serpent shaman may adopt an aspect of the snake while retaining her normal form. This ability functions as the bear shaman ability, but the druid may select from the following bonuses: movement (climb speed 20 feet, swim speed 20 feet), scales (+2 natural armor bonus to AC), senses (low-light vision, scent), or natural weapons (bite [1d4], poison [ frequency 1 round (6), effect 1 Con damage, Cure 1 save, Con-based DC] for a Medium druid, +2 CMB to grapple). While using totem transformation, the serpent shaman may speak normally and can cast speak with animals (reptiles only) at will."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Serpent Shaman ~ Totemic Summons", at_level: 5, description: Some("At 5th level, a serpent shaman may cast summon nature's ally as a standard action when summoning snakes, and these summoned creatures gain temporary hit points equal to her druid level. This ability otherwise functions as the bear shaman ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Serpent Shaman ~ Wild Shape", at_level: 6, description: Some("At 6th level, a serpent shaman's wild shape ability functions at her druid level - 2. If she takes on the form of a snake, she instead uses her druid level + 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Serpent Shaman ~ Bonus Feat", at_level: 9, description: Some("At 9th level and every 4 levels thereafter, a serpent shaman gains one of the following bonus feats: Combat Expertise, Improved Feint, Skill Focus (Bluff ), Stealthy, or Strike Back. She must meet the prerequisites for these bonus feats. This ability replaces venom immunity."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Druid ~ Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Progression", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Times", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Wild Shape", at_level: 6, description: None, benefit: None },
            ],
        },
        // Druid Archetype ~ Swamp Druid -- apg_abilities_class.lst:1897
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Swamp Druid",
            subject: "Druid",
            archetype_name: "Swamp Druid",
            description: Some("Some druids eschew pleasant glades and groves and instead seek out dank marshes, misty bogs and heaths, and trackless swamps as the place they call home and watch over with care, finding beauty and life in abundance in places few others would willingly enter."),
            source_page: Some("p.101"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Swamp Druid],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidWoodlandStride,TYPE.DruidTracklessStep,TYPE.DruidResistNaturesLure,TYPE.DruidWildShape,TYPE.DruidThousandFaces]"]),
            replaces: Some(&["DruidWoodlandStride", "DruidTracklessStep", "DruidResistNaturesLure", "DruidWildShape", "DruidThousandFaces"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Swamp Druid ~ Marshwight", at_level: 2, description: Some("At 2nd level, a swamp druid gains a bonus on Initiative checks and Knowledge (geography), Perception, Stealth, Swim, and Survival checks equal to 1/2 her druid level in swamp terrain, and she cannot be tracked in such an environment. This ability replaces woodland stride."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Swamp Druid ~ Swamp Strider", at_level: 3, description: Some("At 3rd level, a swamp druid suffers no penalty to speed or on Acrobatics or Stealth checks in bogs and undergrowth. This ability replaces trackless step."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Swamp Druid ~ Pond Scum", at_level: 4, description: Some("At 4th level, a swamp druid gains a +4 bonus on saves against disease and the exceptional, supernatural, and spell-like abilities of monstrous humanoids. A swamp druid also gains DR/- equal to half her druid level against attacks by swarms. If this damage resistance prevents damage, the druid is unaffected by distraction or other special attacks of the swarm. This ability replaces resist nature's lure."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Swamp Druid ~ Wild Shape", at_level: 6, description: Some("A swamp druid gains this ability at 6th level, except that her effective druid level for the ability is equal to her druid level - 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Swamp Druid ~ Slippery", at_level: 13, description: Some("At 13th level, a swamp druid gains continuous freedom of movement. This ability replaces a thousand faces."), benefit: None },
            ],
        },
        // Druid Archetype ~ Urban Druid -- apg_abilities_class.lst:1898
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Urban Druid",
            subject: "Druid",
            archetype_name: "Urban Druid",
            description: Some("While many druids keep to the wilderness, some make their way within settlements, communing with the animals and vermin who live there and speaking for the nature that runs rampant in civilization's very cradle."),
            source_page: Some("p.101"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Urban Druid],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidWoodlandStride,TYPE.DruidTracklessStep,TYPE.DruidResistNaturesLure,TYPE.DruidThousandFaces,TYPE.DruidWildShape,TYPE.DruidVenomImmunity,TYPE.DruidNatureBond]"]),
            replaces: Some(&["DruidWoodlandStride", "DruidTracklessStep", "DruidResistNaturesLure", "DruidThousandFaces", "DruidWildShape", "DruidVenomImmunity", "DruidNatureBond"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Urban Druid ~ Nature Bond", at_level: 1, description: Some("An urban druid may not select an animal companion. Instead, she must choose from the following domains, rather than those usually available to druids: Charm, Community, Knowledge, Nobility, Protection, Repose, Rune, or Weather."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Urban Druid ~ Spontaneous Casting", at_level: 1, description: Some("An urban druid can channel stored spell energy into domain spells that she has not prepared ahead of time. She can lose a prepared spell in order to cast any domain spell of the same level or lower. This ability replaces the ability to spontaneously cast summon nature's ally spells."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Urban Druid ~ Lorekeeper", at_level: 2, description: Some("At 2nd level, an urban druid adds Diplomacy, Knowledge (history), Knowledge (local), and Knowledge (nobility) skills. She also receives a +2 bonus on these skill checks. This ability replaces a druid's woodland stride and trackless step abilities."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Urban Druid ~ Resist Temptation", at_level: 4, description: Some("At 4th level, an urban druid gains a +2 bonus on saves vs. divinations and enchantments. This replaces the resist nature's lure ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Urban Druid ~ A Thousand Faces", at_level: 6, description: Some("An urban druid gains this ability at 6th level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Urban Druid ~ Wild Shape", at_level: 8, description: Some("An urban druid gains this ability at 8th level, except that her effective druid level for the ability is equal to her druid level - 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Urban Druid ~ Mental Strength", at_level: 9, description: Some("At 9th level, an urban druid gains immunity to charm and compulsion effects. This ability replaces venom immunity."), benefit: None },
            ],
        },
        // Druid Archetype ~ Wolf Shaman -- apg_abilities_class.lst:1904
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Wolf Shaman",
            subject: "Druid",
            archetype_name: "Wolf Shaman",
            description: Some("A shaman with this totem calls upon the clever wolf, capable of roaming alone yet wise enough to run with a pack when facing dangers too great for one alone."),
            source_page: Some("p.103"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Wolf Shaman],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidThousandFaces,TYPE.DruidVenomImmunity,TYPE.DruidWildShape,TYPE.DruidNatureBond]"]),
            replaces: Some(&["DruidThousandFaces", "DruidVenomImmunity", "DruidWildShape"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Wolf Shaman ~ Nature Bond", at_level: 1, description: Some("A wolf shaman who chooses an animal companion must select a wolf. If choosing a domain, the wolf shaman must choose from the Animal, Community, Liberation, and Travel domains."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wolf Shaman ~ Wild Empathy", at_level: 1, description: Some("A wolf shaman can use wild empathy with canines as a full-round action with a +4 bonus."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wolf Shaman ~ Totem Transformation", at_level: 2, description: Some("At 2nd level, a wolf shaman may adopt an aspect of the wolf while retaining her normal form. This ability functions as the bear shaman ability, but the druid may select from the following bonuses: movement (+20 enhancement bonus to land speed), senses (low-light vision, scent, +4 racial bonus to Survival when tracking by scent), or natural weapons (bite [1d4 plus trip] for a Medium druid, +2 CMB to trip). While using totem transformation, the wolf shaman may speak normally and can cast speak with animals (canines only) at will."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wolf Shaman ~ Totemic Summons", at_level: 5, description: Some("At 5th level, a wolf shaman may cast summon nature's ally as a standard action when summoning canines, and these summoned creatures gain temporary hit points equal to her druid level. This ability otherwise functions as the bear shaman ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wolf Shaman ~ Wild Shape", at_level: 6, description: Some("At 6th level, a wolf shaman's wild shape ability functions at her druid level + 2. If she takes on the form of a canine, she instead uses her druid level + 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wolf Shaman ~ Bonus Feat", at_level: 9, description: Some("At 9th level and every 4 levels thereafter, a wolf shaman gains one of the following bonus feats: Greater Trip, Improved Trip, Mobility, Skill Focus (Stealth), or Spring Attack. She must meet the prerequisites for these bonus feats. This ability replaces venom immunity."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Druid ~ Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Progression", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Times", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Wild Shape", at_level: 6, description: None, benefit: None },
            ],
        },
        // Fighter Archetype ~ Archer -- apg_abilities_class.lst:2102
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Archer",
            subject: "Fighter",
            archetype_name: "Archer",
            description: Some("The archer is dedicated to the careful mastery of the bow, perfecting his skills with years of practice honed day after day on ranges and hunting for game, or else on the battlefield, raining destruction down on the enemy lines."),
            source_page: Some("p.104"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ Archer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterBravery,TYPE.FighterArmorTraining1,TYPE.FighterArmorTraining2,TYPE.FighterArmorTraining3,TYPE.FighterArmorTraining4,TYPE.FighterWeaponTraining1,TYPE.FighterWeaponTraining2,TYPE.FighterWeaponTraining3,TYPE.FighterWeaponTraining4,TYPE.FighterArmorMastery]"]),
            replaces: Some(&["FighterBravery", "FighterArmorTraining_ALL", "FighterWeaponTraining_ALL", "FighterArmorMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Fighter Archer ~ Hawkeye", at_level: 2, description: Some("At 2nd level, an archer gains a +1 bonus on Perception checks, and the range increment for any bow he uses increases by 5 feet. These bonuses increase by +1 and 5 additional feet for every 4 levels beyond 2nd. This ability replaces bravery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Archer ~ Trick Shot", at_level: 3, description: Some("At 3rd level, an archer can choose one of the following combat maneuvers or actions: disarm, feint, or sunder. He can perform this action with a bow against any target within 30 feet, with a -4 penalty to his CMB. Every four levels beyond 3rd, he may choose an additional trick shot to learn. These maneuvers use up arrows as normal. At 11th level, he may also choose from the following combat maneuvers: bull rush, grapple, trip. A target grappled by an arrow can break free by destroying the archer's arrow (hardness 5, hit points 1, break DC 13) or with an Escape Artist or CMB check (against the archer's CMD -4). This ability replaces armor training 1, 2, 3, and 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Archer ~ Expert Archer", at_level: 5, description: Some("At 5th level, an archer gains a +1 bonus on attack and damage rolls with bows. This bonus increases by +1 for every four levels beyond 5th. This ability replaces weapon training 1."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Archer ~ Safe Shot", at_level: 9, description: Some("At 9th level, an archer does not provoke attacks of opportunity when making ranged attacks with a bow. This ability replaces weapon training 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Archer ~ Evasive Archer", at_level: 13, description: Some("At 13th level, an archer gains a +2 dodge bonus to AC against ranged attacks. This bonus increases to +4 at 17th level. This ability replaces weapon training 3."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Archer ~ Volley", at_level: 17, description: Some("At 17th level, as a full-round action, an archer can make a single bow attack at his highest base attack bonus against any number of creatures in a 15-foot radius burst, making separate attack and damage rolls for each creature. This ability replaces weapon training 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Archer ~ Ranged Defense", at_level: 19, description: Some("At 19th level, an archer gains DR 5/- against ranged attacks. In addition, as an immediate action, he can catch an arrow fired at him and shoot it any target he chooses, as if he had the Snatch Arrows feat. This ability replaces armor mastery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Archer ~ Weapon Mastery", at_level: 1, description: Some("An archer must choose a type of bow."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Crossbowman -- apg_abilities_class.lst:2103
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Crossbowman",
            subject: "Fighter",
            archetype_name: "Crossbowman",
            description: Some("The crossbowman has perfected the deadly use of the crossbow, a simple but cruelly efficient weapon, as a craftsman mastering a lethal tool."),
            source_page: Some("p.104"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ Crossbowman],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterArmorTraining1,TYPE.FighterWeaponTraining1,TYPE.FighterArmorTraining2,TYPE.FighterWeaponTraining2,TYPE.FighterArmorTraining3,TYPE.FighterWeaponTraining3,TYPE.FighterArmorTraining4,TYPE.FighterWeaponTraining4,TYPE.FighterArmorMastery]"]),
            replaces: Some(&["FighterArmorTraining_ALL", "FighterWeaponTraining_ALL", "FighterArmorMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Fighter Crossbowman ~ Deadshot", at_level: 3, description: Some("At 3rd level, when a crossbowman attacks with a crossbow as a readied action, he may add 1/2 his Dexterity bonus (minimum +1) on his damage roll. This ability replaces armor training 1."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Crossbowman ~ Crossbow Expert", at_level: 5, description: Some("At 5th level, a crossbowman gains a +1 bonus on attack and damage rolls with crossbows. This bonus increases by +1 per four levels after 5th. This ability replaces weapon training 1."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Crossbowman ~ Improved Deadshot", at_level: 7, description: Some("At 7th level, when a crossbowman attacks with a crossbow as a readied action, his target is denied its Dexterity bonus to its AC. This ability replaces armor training 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Crossbowman ~ Quick Sniper", at_level: 9, description: Some("At 9th level, a crossbowman gains a bonus equal to 1/2 his fighter level on Stealth checks when sniping. When he is hit with a ranged attack, he can shoot his crossbow at his attacker as an immediate action if it is loaded. This ability replaces weapon training 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Crossbowman ~ Greater Deadshot", at_level: 11, description: Some("At 11th level, when a crossbowman attacks with a crossbow as a readied action, he may add his Dexterity bonus (minimum +1) on his damage roll. This ability replaces armor training 3."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Crossbowman ~ Safe Shot", at_level: 13, description: Some("At 13th level, a crossbowman does not provoke attacks of opportunity when making ranged attacks with a crossbow. This ability replaces weapon training 3."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Crossbowman ~ Meteor Shot", at_level: 17, description: Some("At 17th level, as a standard action, a crossbowman can make one attack with a crossbow at a -4 penalty. If the attack hits, it inflicts damage normally and the target is subject to a bull rush or a trip maneuver using the attack roll as the combat maneuver check. The crossbowman must decide which maneuver to attempt before making his attack roll. This ability replaces weapon training 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Crossbowman ~ Penetrating Shot", at_level: 19, description: Some("At 19th level, when a crossbowman confirms a critical hit with a crossbow, the bolt pierces the target and can strike another creature in line behind it. The crossbowman must be able to trace a line starting at his space and passing through both targets to make this additional attack. The secondary attack is made at a -4 penalty, in addition to any modifiers for added range. If this attack is also a critical hit, the bolt can continue to hit another target, but the penalties stack. This ability replaces armor mastery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Crossbowman ~ Weapon Mastery", at_level: 1, description: Some("A crossbowman must choose a type of crossbow"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Crossbowman ~ Pinpoint Targeting", at_level: 15, description: Some("At 15th level, a crossbowman gains Pinpoint Targeting as a bonus feat. This ability replaces armor training 4."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Free Hand Fighter -- apg_abilities_class.lst:2104
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Free Hand Fighter",
            subject: "Fighter",
            archetype_name: "Free Hand Fighter",
            description: Some("The free hand fighter specializes in the delicate art of handling a single weapon in one hand while using his free hand to balance, block, tip, and distract his opponents. While not a brawler, his open hand is as much a weapon as a bow or blade. His fighting school benefits only apply when he is using a one-handed weapon and carrying nothing in his other hand."),
            source_page: Some("p.105"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ Free Hand Fighter],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterBravery,TYPE.FighterArmorTraining1,TYPE.FighterArmorTraining2,TYPE.FighterArmorTraining3,TYPE.FighterArmorTraining4,TYPE.FighterWeaponTraining1,TYPE.FighterWeaponTraining2,TYPE.FighterWeaponTraining3,TYPE.FighterArmorMastery,TYPE.FighterWeaponTraining4]"]),
            replaces: Some(&["FighterBravery", "FighterArmorTraining_ALL", "FighterWeaponTraining1", "FighterWeaponTraining2", "FighterWeaponTraining3", "FighterWeaponTraining4", "FighterArmorMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Free Hand Fighter ~ Deceptive Strike", at_level: 2, description: Some("At 2nd level, a free hand fighter gains a +1 bonus to CMB and CMD on disarm checks and on Bluff checks to feint or create a diversion to hide. This bonus increases by +1 for every four levels after 2nd. This ability replaces bravery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Free Hand Fighter ~ Elusive", at_level: 3, description: Some("At 3rd level, a free hand fighter gains a +1 dodge bonus to AC. This bonus increases by +1 for every four levels after 2nd. This bonus does not apply when wearing medium or heavy armor or carrying a medium or heavier load. This ability replaces armor training 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Free Hand Fighter ~ Singleton", at_level: 5, description: Some("At 5th level, a free hand fighter gains a +1 bonus on attack and damage rolls when wielding a melee weapon in one hand and leaving his other hand free. This ability replaces weapon training 1 and 4. This bonus increases by +1 per six levels after 5th."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Free Hand Fighter ~ Timely Tip", at_level: 9, description: Some("At 9th level, a free hand fighter can make a disarm combat maneuver against a target he threatens as a move action to push aside the target's shield. If successful, the target loses its shield bonus to AC against the free hand fighter's next attack. This ability replaces weapon training 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Free Hand Fighter ~ Interference", at_level: 13, description: Some("At 13th level, a free hand fighter can make a disarm or trip combat maneuver against a target he threatens as a move action to push his opponent off balance. If successful, the target becomes flat-footed. This condition lasts until the target takes damage from a melee or ranged attack or until the beginning of the free hand fighter's next turn, whichever comes first. This ability replaces weapon training 3."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Free Hand Fighter ~ Reversal", at_level: 19, description: Some("At 19th level, a free hand fighter can make a disarm combat maneuver against a creature he threatens as an immediate action when he is the target of a melee attack from another creature. If successful, the attack changes to target the target of the free hand fighter's maneuver instead of the free hand fighter himself. This ability replaces armor mastery."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Mobile Fighter -- apg_abilities_class.lst:2105
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Mobile Fighter",
            subject: "Fighter",
            archetype_name: "Mobile Fighter",
            description: Some("Where some fighters focus on strength and raw power, the mobile fighter relies on swiftness and mobility, gliding across the battlefield like a steel whirlwind and leaving destruction in his wake."),
            source_page: Some("p.105"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ Mobile Fighter],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterBravery,TYPE.FighterWeaponTraining1,TYPE.FighterArmorTraining3,TYPE.FighterArmorTraining4,TYPE.FighterWeaponMastery,TYPE.FighterWeaponTraining2,TYPE.FighterWeaponTraining3,TYPE.FighterWeaponTraining4]"]),
            replaces: Some(&["FighterBravery", "FighterWeaponTraining1", "FighterArmorTraining3", "FighterArmorTraining4", "FighterWeaponMastery", "FighterWeaponTraining1", "FighterWeaponTraining2", "FighterWeaponTraining3", "FighterWeaponTraining4"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Mobile Fighter ~ Agility", at_level: 2, description: Some("At 2nd level, a mobile fighter gains a +1 bonus on saving throws made against effects that cause him to become paralyzed, slowed, or entangled. This bonus increases by +1 for every four levels beyond 2nd. This ability replaces bravery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mobile Fighter ~ Leaping Attack", at_level: 5, description: Some("At 5th level, when a mobile fighter moves at least 5 feet prior to attacking, he gains a +1 bonus on attack and damage rolls. This bonus increases by +1 for every four levels beyond 5th. This ability replaces weapon training 1, 2, 3, and 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mobile Fighter ~ Rapid Attack", at_level: 11, description: Some("At 11th level, a mobile fighter can combine a full attack action with a single move. He must forgo the attack at his highest bonus but may take the remaining attacks at any point during his movement. This movement provokes attacks of opportunity as normal. This ability replaces armor training 3."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mobile Fighter ~ Fleet Footed", at_level: 15, description: Some("At 15th level, the mobile fighter's speed increases by 10 feet. He can take 10 on Acrobatics checks even while distracted or threatened, and can take 20 on an Acrobatics check once per day for every five fighter levels he possesses. This ability replaces armor training 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mobile Fighter ~ Whirlwind Blitz", at_level: 20, description: Some("At 20th level, a mobile fighter can make a full-attack action as a standard action. He may also use the Whirlwind Attack feat as a standard action. This ability replaces weapon mastery."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Phalanx Soldier -- apg_abilities_class.lst:2106
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Phalanx Soldier",
            subject: "Fighter",
            archetype_name: "Phalanx Soldier",
            description: Some("The phalanx soldier specializes in defensive tactics, using his shield to guard himself and his allies and forming a shield wall like an unbreakable anvil against which his enemies break."),
            source_page: Some("p.105"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ Phalanx Soldier],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterBravery,TYPE.FighterArmorTraining1,TYPE.FighterWeaponTraining1,TYPE.FighterArmorTraining1,TYPE.FighterArmorTraining2,TYPE.FighterWeaponTraining2,TYPE.FighterWeaponTraining3,TYPE.FighterWeaponTraining4,TYPE.FighterArmorTraining4,TYPE.FighterWeaponMastery]"]),
            replaces: Some(&["FighterBravery", "FighterArmorTraining_ALL", "FighterWeaponTraining_ALL", "FighterWeaponMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Fighter Phalanx Soldier ~ Stand Firm", at_level: 2, description: Some("At 2nd level, a phalanx soldier gains a +1 bonus to CMD against drag, overrun, and trip attempts. This bonus also applies on saves against trample attacks. The bonus increases by +1 for every four levels beyond 2nd. This ability replaces bravery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Phalanx Soldier ~ Phalanx Fighting", at_level: 3, description: Some("At 3rd level, when a phalanx soldier wields a shield, he can use any polearm or spear of his size as a one-handed weapon. This ability replaces armor training 1."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Phalanx Soldier ~ Ready Pike", at_level: 5, description: Some("At 5th level, a phalanx soldier can, once per day, ready a weapon with the brace property as an immediate action, gaining a +1 bonus on attack and damage rolls. For every four levels beyond 5th, this bonus increases by +1, and he can use the ability one additional time per day. He cannot use this ability when flat-footed. This ability replaces weapon training 1."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Phalanx Soldier ~ Deft Shield", at_level: 7, description: Some("At 7th level, the armor check penalty from a shield and the attack roll penalty are reduced by -1 for a phalanx soldier using a tower shield. At 11th level, these penalties are reduced by -2. This ability replaces armor training 2 and 3."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Phalanx Soldier ~ Shield Ally", at_level: 9, description: Some("At 9th level, when a phalanx fighter is using a heavy or tower shield, he can, as a move action, provide partial cover (+2 cover bonus to AC, +1 bonus on Reflex saves) to himself and all adjacent allies until the beginning of his next turn. At 13th level, he can instead provide cover (+4 cover bonus to AC, +2 bonus on Reflex saves) and evasion (as a rogue) to one adjacent ally until the beginning of his next turn. This cover does not allow Stealth checks. At 17th level, he can provide cover to himself and all adjacent allies, or he can provide improved cover (+8 cover bonus to AC, +4 bonus on Reflex saves, improved evasion) to a single adjacent ally. This ability replaces weapon training 2, 3, and 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Phalanx Soldier ~ Irresistible Advance", at_level: 15, description: Some("At 15th level, a phalanx fighter gains a bonus on bull rush and overrun CMB checks. This bonus depends on the type of shield used: +1 with a buckler, +2 with a light shield, +3 with a heavy shield, or +4 with a tower shield. This ability replaces armor training 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Phalanx Soldier ~ Shielded Fortress", at_level: 20, description: Some("At 20th level, a phalanx fighter's shield cannot be disarmed or sundered. He gains evasion (as a rogue) when using a shield (improved evasion when using a tower shield). As a move action, a phalanx fighter can provide evasion to all adjacent allies until the beginning of his next turn. As an immediate action, he can provide improved evasion to an adjacent ally against one attack. This ability replaces weapon mastery."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Polearm Master -- apg_abilities_class.lst:2107
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Polearm Master",
            subject: "Fighter",
            archetype_name: "Polearm Master",
            description: Some("The polearm master is schooled in the ancient wisdom that enemies are best faced at the end of long striking pole, lashing like a serpent before clumsy swords and axes can even be brought to bear."),
            source_page: Some("p.106"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ Polearm Master],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterBravery,TYPE.FighterArmorTraining1,TYPE.FighterArmorTraining2,TYPE.FighterArmorTraining3,TYPE.FighterArmorTraining4,TYPE.FighterWeaponTraining1,TYPE.FighterWeaponTraining2,TYPE.FighterWeaponTraining3,TYPE.FighterWeaponTraining4,TYPE.FighterArmorMastery]"]),
            replaces: Some(&["FighterBravery", "FighterArmorTraining_ALL", "FighterWeaponTraining_ALL", "FighterArmorMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Fighter Polearm Master ~ Pole Fighting", at_level: 2, description: Some("At 2nd level, as an immediate action, a polearm master can shorten the grip on his spear or polearm with reach and use it against adjacent targets. This action results in a -4 penalty on attack rolls with that weapon until he spends another immediate action to return to the normal grip. The penalty is reduced by -1 for every four levels beyond 2nd. This ability replaces bravery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Polearm Master ~ Steadfast Pike", at_level: 3, description: Some("At 3rd level, a polearm master gains a +1 bonus on attack rolls with readied attacks and attacks of opportunity made with a spear or polearm. The bonus increases by +1 for every four levels beyond 3rd. This ability replaces armor training 1, 2, 3, and 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Polearm Master ~ Polearm Training", at_level: 1, description: Some("At 5th level, a polearm master gains a +1 bonus on attack and damage rolls with spears and polearms. The bonus increases by +1 for every four levels beyond 5th. This ability replaces weapon training 1."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Polearm Master ~ Flexible Flanker", at_level: 9, description: Some("At 9th level, a polearm master may choose any square adjacent to him and treat that square as his location for determining who he is flanking, even if that square is occupied by a creature, object, or solid barrier. This ability replaces weapon training 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Polearm Master ~ Sweeping Fend", at_level: 13, description: Some("At 13th level, a polearm master can use any spear or pole arm to make a bull rush or trip maneuver, though he takes a -4 penalty to his CMB when making such attempts. Weapons with the trip property do not incur this penalty on trip maneuvers. This ability replaces weapon training 3."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Polearm Master ~ Step Aside", at_level: 17, description: Some("At 17th level, when a creature threatened by a polearm master takes a 5-foot step into a square adjacent to him, he can take a 5-foot step as an immediate action. This 5-foot step must be subtracted from his movement on the next turn. He also gains a +2 dodge bonus to his AC against that opponent until the end of his next turn. This ability replaces weapon training 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Polearm Master ~ Polearm Parry", at_level: 19, description: Some("At 19th level, when an opponent threatened by a polearm master makes a melee attack against an ally, he may take an immediate action to grant his ally a +2 shield bonus to AC and DR 5/- against that attack. He may use this ability to protect himself, but only if the attacking creature is not adjacent to him. This ability replaces armor mastery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Polearm Master ~ Weapon Mastery", at_level: 1, description: Some("A polearm master must choose a spear or polearm."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Roughrider -- apg_abilities_class.lst:2108
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Roughrider",
            subject: "Fighter",
            archetype_name: "Roughrider",
            description: Some("Roughriders study and practice the fine points of mounted combat, drilling endlessly with warbeasts- from noble thoroughbreds to trained monsters-to form a perfect synergy between rider and steed."),
            source_page: Some("p.106"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ Roughrider],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterBravery,TYPE.FighterArmorTraining1,TYPE.FighterWeaponTraining1,TYPE.FighterWeaponTraining2,TYPE.FighterWeaponTraining3,TYPE.FighterWeaponTraining4,TYPE.FighterArmorTraining2,TYPE.FighterArmorTraining3,TYPE.FighterArmorTraining4,TYPE.FighterArmorTraining4,TYPE.FighterArmorMastery]"]),
            replaces: Some(&["FighterBravery", "FighterArmorTraining_ALL", "FighterWeaponTraining_ALL", "FighterArmorMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Fighter Roughrider ~ Steadfast Mount", at_level: 2, description: Some("At 2nd level, after a roughrider has spent 1 hour practicing with a mount, the mount gains a +1 dodge bonus to AC and a +1 morale bonus on saves, but only while the roughrider is mounted on it or adjacent to it. This bonus increases by +1 for every four levels after 2nd. This ability replaces bravery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Roughrider ~ Armored Charger", at_level: 3, description: Some("At 3rd level, a roughrider no longer suffers armor check penalties on Ride skill checks. His mount's speed is not reduced when carrying a medium load or wearing medium barding. This ability replaces armor training 1."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Roughrider ~ Mounted Mettle", at_level: 5, description: Some("At 5th level, a roughrider and his mount gain a +1 bonus on attack and damage rolls when he is mounted or adjacent to his mount. This bonus increases by +1 for every four levels after 5th. This ability replaces weapon training 1, 2, 3, and 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Roughrider ~ Leap from the Saddle", at_level: 7, description: Some("At 7th level, after a roughrider's mount takes a single move, he may attempt a fast dismount (DC 20 Ride check). If he succeeds, he can take a full attack action. This ability replaces armor training 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Roughrider ~ Relentless Steed", at_level: 11, description: Some("At 11th level, a roughrider's mount does not reduce its speed when wearing heavy barding or carrying a heavy load. The roughrider may also reroll a Ride skill check or a saving throw made by the mount once per day, but must use the second roll even if it is worse. This ability may be used one additional time per day for every four levels beyond 11th. This ability replaces armor training 3."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Roughrider ~ Ride Them Down", at_level: 15, description: Some("At 15th level, can spur his mount on while readying an attack. If a roughrider's mount takes a single move, the roughrider can make a full attack, taking his attacks at any point during his mount's movement. If he has the Trample feat, he may substitute an overrun combat maneuver for each of his attacks. This movement provokes attacks of opportunity against the roughrider but not his mount. This ability replaces armor training 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Roughrider ~ Unavoidable Onslaught", at_level: 15, description: Some("At 15th level, a roughrider's mounted charge is not blocked by friendly creatures or difficult terrain. This ability replaces armor training 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Roughrider ~ Indomitable Steed", at_level: 19, description: Some("At 19th level, a roughrider and his steed gain DR 5/- when mounted. This ability replaces armor mastery."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Savage Warrior -- apg_abilities_class.lst:2109
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Savage Warrior",
            subject: "Fighter",
            archetype_name: "Savage Warrior",
            description: Some("Warriors' might is not measured only by their skill with steel, but also by their ability to inflict death with fang and claw, horn and hoof, and every exotic appendage the natural and unnatural world has to offer."),
            source_page: Some("p.107"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ Savage Warrior],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterBravery,TYPE.FighterWeaponTraining1,TYPE.FighterWeaponTraining2,TYPE.FighterWeaponTraining3,TYPE.FighterWeaponTraining4,TYPE.FighterWeaponMastery]"]),
            replaces: Some(&["FighterBravery", "FighterWeaponTraining_ALL", "FighterWeaponMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Fighter Savage Warrior ~ Spark of Life", at_level: 2, description: Some("At 2nd level, a savage warrior gains a +1 bonus on saving throws made against energy drain and death effects. This bonus increases by +1 for every four levels beyond 2nd. This ability replaces bravery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Savage Warrior ~ Natural Savagery", at_level: 5, description: Some("At 5th level, a savage warrior gains a +1 bonus on attack and damage rolls with natural weapons. This bonus also applies to CMB and CMD for grappling. This bonus increases by +1 for every four levels beyond 5th. This ability replaces weapon training 1."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Savage Warrior ~ Savage Charge", at_level: 9, description: Some("At 9th level, when a savage warrior attacks with a natural weapon at the end of a charge, he gains a bonus on his natural weapon attack rolls equal to half his fighter level, while suffering a penalty to his AC equal to half his fighter level. These replace the normal attack roll bonus and AC penalty for charging. This bonus also applies to his CMB for a bull rush or overrun combat maneuvers made when charging. This ability replaces weapon training 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Savage Warrior ~ Careful Claw", at_level: 13, description: Some("At 13th level, when using a natural weapon to attack a creature using fire shield or a similar effect that damages creatures attacking it (such as a barbed devil's barbed defense), a savage warrior reduces the damage from such effects by an amount equal to 1/2 his fighter level. This ability replaces weapon training 3."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Savage Warrior ~ Greater Savage Charge", at_level: 17, description: Some("At 17th level, when using savage charge, the AC penalty is reduced to 1/4 his fighter level instead of 1/2 his fighter level. In addition, a savage warrior can charge through friendly creatures and difficult terrain. This ability replaces weapon training 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Savage Warrior ~ Natural Weapon Mastery", at_level: 20, description: Some("At 20th level, a savage warrior must choose one natural weapon. This ability replaces weapon mastery."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Shielded Fighter -- apg_abilities_class.lst:2110
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Shielded Fighter",
            subject: "Fighter",
            archetype_name: "Shielded Fighter",
            description: Some("A shielded fighter focuses on both offense and defense, blending weapon and shield in perfect balance to impede his enemies while delivering deadly blows, and even turning the shield itself into a formidable weapon. These fighting school benefits apply when wielding a weapon and a shield simultaneously."),
            source_page: Some("p.108"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ Shielded Fighter],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterArmorTraining1,TYPE.FighterArmorTraining2,TYPE.FighterArmorTraining3,TYPE.FighterArmorTraining4,TYPE.FighterWeaponTraining1,TYPE.FighterWeaponTraining3,TYPE.FighterWeaponTraining3,TYPE.FighterWeaponTraining4,TYPE.FighterArmorMastery,TYPE.FighterWeaponMastery]"]),
            replaces: Some(&["FighterArmorTraining_ALL", "FighterWeaponTraining_ALL", "FighterArmorMastery", "FighterWeaponMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Fighter Shielded Fighter ~ Active Defense", at_level: 3, description: Some("At 3rd level, a shielded fighter gains a +1 dodge bonus to AC when wielding a shield and fighting defensively, using Combat Expertise, or using total defense. This bonus increases by +1 for every four levels beyond 3rd. As a swift action, he may share this bonus with one adjacent ally, or half of the bonus (minimum +0) with all adjacent allies, until the beginning of his next turn. This ability replaces armor training 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Shielded Fighter ~ Shield Fighter", at_level: 5, description: Some("At 5th level, a shielded fighter gains a +1 bonus on attack and damage rolls when making a shield bash. These bonuses increase by +1 every four levels beyond 5th. With a full attack action, a shielded fighter may alternate between using his weapon or his shield for each attack. This action does not grant additional attacks or incur penalties as two-weapon fighting does. This ability replaces weapon training 1."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Shielded Fighter ~ Shield Buffet", at_level: 9, description: Some("At 9th level, as a move action, a shielded fighter may make a combat maneuver check to use his shield to impede an adjacent enemy. If successful, the target suffers a -2 penalty on its attack rolls against the shielded fighter and a -2 penalty to AC on attacks made by the shielded fighter until the beginning of his next turn. At 13th level, a shielded fighter may use this ability as a swift action. This ability replaces weapon training 2 and 3."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Shielded Fighter ~ Shield Guard", at_level: 17, description: Some("At 17th level, as a swift action, a shielded fighter may designate one square adjacent to him. He may designate two squares if using a heavy shield or three squares if using a tower shield, but these squares must be contiguous. Enemies in these squares cannot flank the shielded fighter and do not count for flanking with other creatures. This effect lasts until he moves from his position or uses another swift action to change the affected squares. This ability replaces weapon training 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Shielded Fighter ~ Shield Mastery", at_level: 19, description: Some("At 19th level, a shielded fighter gains DR 5/- when wielding a shield. This ability replaces armor mastery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Shielded Fighter ~ Shield Ward", at_level: 20, description: Some("At 20th level, a shielded fighter gains evasion (as a rogue) while wielding a shield, and adds his shield bonus to his AC (not including enhancement bonuses) on Reflex saves and to his touch AC. In addition, his shield cannot be disarmed or sundered. This ability replaces weapon mastery."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Two-Handed Fighter -- apg_abilities_class.lst:2111
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Two-Handed Fighter",
            subject: "Fighter",
            archetype_name: "Two-Handed Fighter",
            description: Some("Some fighters focus their efforts on finding the biggest, heaviest, most imposing weapon they can find and training to manage and harness the weight of their massive weapons for maximum impact. These fighting school benefits only apply when using two-handed weapons."),
            source_page: Some("p.108"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ Two-Handed Fighter],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterBravery,TYPE.FighterArmorTraining1,TYPE.FighterArmorTraining2,TYPE.FighterArmorTraining3,TYPE.FighterArmorTraining4,TYPE.FighterArmorMastery,TYPE.FighterWeaponTraining1,TYPE.FighterWeaponTraining2,TYPE.FighterWeaponTraining3,TYPE.FighterWeaponTraining4]"]),
            replaces: Some(&["FighterBravery", "FighterArmorTraining_ALL", "FighterArmorMastery", "FighterWeaponTraining_ALL"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Fighter Two-Handed Fighter ~ Shattering Strike", at_level: 2, description: Some("At 2nd level, a two-handed fighter gains a +1 bonus to CMB and CMD on sunder attempts and on damage rolls made against objects. These bonuses increase by +1 for every four levels beyond 2nd. This ability replaces bravery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Two-Handed Fighter ~ Overhand Chop", at_level: 3, description: Some("At 3rd level, when a two-handed fighter makes a single attack (with the attack action or a charge) with a two-handed weapon, he adds double his Strength bonus on damage rolls. This ability replaces armor training 1."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Two-Handed Fighter ~ Weapon Training", at_level: 1, description: Some("As the fighter class feature, but the bonuses only apply when wielding two-handed melee weapons."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Two-Handed Fighter ~ Backswing", at_level: 7, description: Some("At 7th level, when a two-handed fighter makes a full attack with a two-handed weapon, he adds double his Strength bonus on damage rolls for all attacks after the first. This ability replaces armor training 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Two-Handed Fighter ~ Piledriver", at_level: 11, description: Some("At 11th level, as a standard action, a two-handed fighter can make a single melee attack with a two-handed weapon. If the attack hits, he may make a bull rush or trip combat maneuver against the target of his attack as a free action that does not provoke an attack of opportunity. This ability replaces armor training 3."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Two-Handed Fighter ~ Greater Power Attack", at_level: 15, description: Some("At 15th level, when using Power Attack with a two-handed melee weapon, the bonus damage from Power Attack is doubled (+100%%) instead of increased by half (+50%%). This ability replaces armor training 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Two-Handed Fighter ~ Devastating Blow", at_level: 19, description: Some("At 19th level, as a standard action, a two-handed fighter may make a single melee attack with a two-handed weapon at a -5 penalty. If the attack hits, it is treated as a critical threat. Special weapon abilities that activate only on a critical hit do not activate if this critical hit is confirmed. This ability replaces armor mastery."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Two-Weapon Warrior -- apg_abilities_class.lst:2112
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Two-Weapon Warrior",
            subject: "Fighter",
            archetype_name: "Two-Weapon Warrior",
            description: Some("Trained under great masters who preached the simple truth that two are better than one when it comes to weapons, the two-weapon warrior is a terror when his hands are full. From paired daggers to exotic double weapons, all combinations come equally alive in his skilled hands."),
            source_page: Some("p.109"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ Two-Weapon Warrior],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterArmorTraining1,TYPE.FighterArmorTraining2,TYPE.FighterWeaponTraining1,TYPE.FighterWeaponTraining2,TYPE.FighterArmorTraining3,TYPE.FighterWeaponTraining3,TYPE.FighterArmorTraining4,TYPE.FighterWeaponTraining4,TYPE.FighterArmorMastery]"]),
            replaces: Some(&["FighterArmorTraining_ALL", "FighterWeaponTraining_ALL", "FighterArmorMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Fighter Two-Weapon Warrior ~ Defensive Flurry", at_level: 3, description: Some("At 3rd level, when a two-weapon warrior makes a full attack with both weapons, he gains a +1 dodge bonus to AC against melee attacks until the beginning of his next turn. This bonus increases by +1 every four levels after 3rd. This ability replaces armor training 1 and 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Two-Weapon Warrior ~ Twin Blades", at_level: 5, description: Some("At 5th level, a two-weapon warrior gains a +1 bonus on attack and damage rolls when making a full attack with two weapons or a double weapon. This bonus increases by +1 for every four levels after 5th. This ability replaces weapon training 1."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Two-Weapon Warrior ~ Doublestrike", at_level: 9, description: Some("At 9th level, a two-weapon warrior may, as a standard action, make one attack with both his primary and secondary weapons. The penalties for attacking with two weapons apply normally. This ability replaces weapon training 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Two-Weapon Warrior ~ Improved Balance", at_level: 11, description: Some("At 11th level, the attack penalties for fighting with two weapons are reduced by -1 for a two-weapon warrior. Alternatively, he may use a one-handed weapon in his off-hand, treating it as if it were a light weapon with the normal light weapon penalties. This ability replaces armor training 3."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Two-Weapon Warrior ~ Equal Opportunity", at_level: 13, description: Some("At 13th level, when a two-weapon warrior makes an attack of opportunity, he may attack once with both his primary and secondary weapons. The penalties for attacking with two weapons apply normally. This ability replaces weapon training 3."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Two-Weapon Warrior ~ Perfect Balance", at_level: 13, description: Some("At 15th level, the penalties for fighting with two weapons are reduced by an additional -1 for a two-weapon warrior. This benefit stacks with improved balance. If he is using a one-handed weapon in his off hand, treating it as a light weapon, he uses the normal light weapon penalties. This ability replaces armor training 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Two-Weapon Warrior ~ Deft Doublestrike", at_level: 17, description: Some("At 17th level, when a two-weapon warrior hits an opponent with both weapons, he can make a disarm or sunder attempt (or trip, if one or both weapons can be used to trip) against that opponent as an immediate action that does not provoke attacks of opportunity. This ability replaces weapon training 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Two-Weapon Warrior ~ Deadly Defense", at_level: 19, description: Some("At 19th level, when a two-weapon warrior makes a full attack with both weapons, every creature that hits him with a melee attack before the beginning of his next turn provokes an attack of opportunity from the warrior. This ability replaces armor mastery."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Weapon Master -- apg_abilities_class.lst:2113
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Weapon Master",
            subject: "Fighter",
            archetype_name: "Weapon Master",
            description: Some("Devoted to the perfection of a single weapon, the weapon master's meditations upon his favored weapon border on the obsessive, but none can deny his consummate skill. The weapon master must select a single type of weapon (such as longsword or shortbow). All of his abilities apply to that weapon type."),
            source_page: Some("p.109"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ Weapon Master],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterBravery,TYPE.FighterArmorTraining1,TYPE.FighterArmorTraining2,TYPE.FighterArmorTraining3,TYPE.FighterArmorTraining4,TYPE.FighterWeaponTraining1,TYPE.FighterWeaponTraining2,TYPE.FighterWeaponTraining3,TYPE.FighterWeaponTraining4,TYPE.FighterArmorMastery]"]),
            replaces: Some(&["FighterBravery", "FighterArmorTraining_ALL", "FighterWeaponTraining_ALL", "FighterArmorMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Fighter Weapon Master ~ Weapon Guard", at_level: 2, description: Some("You gain a +%1 bonus to CMD against disarm and sunder attempts while wielding your chosen weapon. This bonus also applies on saves against any effect that targets your chosen weapon (for example, grease, heat metal, shatter, warp wood). This ability replaces bravery.|(FighterLVL+2)/4"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Weapon Master ~ Weapon Training", at_level: 3, description: Some("You gaina a +%1 bonus on attack and damage rolls with your chosen weapon. This ability replaces armor training 1, 2, 3, and 4.|WeaponTrainingBase"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Weapon Master ~ Reliable Strike", at_level: 5, description: Some("You may reroll an attack roll, critical hit confirmation roll, miss chance check, or damage roll as an immediate action. You must accept the second roll even if it is worse."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Weapon Master ~ Mirror Move", at_level: 9, description: Some("You gain a +%1 insight bonus to AC when attacked by your chosen weapon. This ability replaces weapon training 2.|WeaponTrainingBase"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Weapon Master ~ Deadly Critical", at_level: 13, description: Some("When you confirm a critical hit with your chosen weapon, you can increase the weapon's damage multiplier by +1 as an immediate action."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Weapon Master ~ Critical Specialist", at_level: 17, description: Some("The save DCs of any effects caused by a critical hit with your chosen weapon increase by +4. This ability replaces weapon training 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fighter Weapon Master ~ Unstoppable Strike", at_level: 19, description: Some("You can take a standard action to make one attack with your chosen weapon as a touch attack that ignores damage reduction (or hardness, if attacking an object). This ability replaces armor mastery."), benefit: None },
            ],
        },
        // Monk Archetype ~ Drunken Master -- apg_abilities_class.lst:2290
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Drunken Master",
            subject: "Monk",
            archetype_name: "Drunken Master",
            description: Some("Most monks lead lives of moderation and quiet contemplation. But the drunken master finds perfection through excess. Powered by strong wine, he uses his intoxication to reach a state where his ki is more potent, if somewhat fleeting. A drunken master has the following class features."),
            source_page: Some("p.110"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Drunken Master],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkStillMind,TYPE.MonkPurityOfBody,TYPE.MonkDiamondBody,TYPE.MonkDiamondSoul,TYPE.MonkEmptyBody]"]),
            replaces: Some(&["MonkStillMind", "MonkPurityOfBody", "MonkDiamondBody", "MonkDiamondSoul", "MonkEmptyBody"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Drunken Master ~ Drunken Ki", at_level: 3, description: Some("At 3rd level, a drunken master can drink a tankard of ale or strong alcohol and gain one temporary ki point. The act of drinking is a swift action that does not provoke attacks of opportunity. The monk can have a maximum number of drunken ki points equal to 1 plus one additional point for every two levels thereafter (5th, 7th, and so on). The monk can gain this temporary ki even before he gains a ki pool at 4th level. These drunken ki points last for 1 hour or until spent, whichever is shorter. As long as he has at least 1 drunken ki point, the monk can spend 1 ki point as a move action to move 5 feet without provoking attacks of opportunity. This ability replaces still mind."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Drunken Master ~ Drunken Strength", at_level: 5, description: Some("At 5th level, a drunken master can spend 1 point of ki as a swift action to inflict 1d6 extra points of damage on a single successful melee attack. The monk can choose to apply the damage after the attack roll is made. At 10th level, the monk may spend 2 drunken ki points to increase the extra damage to 2d6. At 15th level, the monk may spend 3 drunken ki points to increase the extra damage to 3d6. At 20th level, the monk may spend 4 drunken ki points to increase the extra damage to 4d6. The monk must have at least 1 drunken ki point to use this ability. This ability replaces purity of body."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Drunken Master ~ Drunken Courage", at_level: 11, description: Some("At 11th level, a drunken master is immune to fear as long as he has at least 1 point of drunken ki. This ability replaces diamond body."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Drunken Master ~ Drunken Resilience", at_level: 13, description: Some("At 13th level, a drunken master gains DR 1/- as long as he has at least 1 point of drunken ki. At 16th level, the DR increases to 2/-. At 19th level, it increases to 3/-. This ability replaces diamond soul."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Drunken Master ~ Firewater Breath", at_level: 19, description: Some("At 19th level, a drunken master can take a drink and expel a gout of alcohol-fueled fire in a 30-foot cone. Creatures within the cone take 20d6 points of fire damage. A successful Reflex saving throw DC %1 (DC 10 + 1/2 the monk's level + the monk's Wis modifier) halves the damage. Using this ability is a standard action that consumes 4 ki points from the monk's ki pool. The monk must have at least 1 drunken ki point to use this ability. This ability replaces empty body.|10+CHA+(MonkLVL/2)"), benefit: None },
            ],
        },
        // Monk Archetype ~ Hungry Ghost Monk -- apg_abilities_class.lst:2291
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Hungry Ghost Monk",
            subject: "Monk",
            archetype_name: "Hungry Ghost Monk",
            description: Some("The hungry ghost monk looks to spirits that prey upon the living as models of perfection. He sees the life energy of the universe as a resource to be manipulated, even stealing it from other creatures. It is through this constant influx of energy that the hungry ghost monk reaches his ultimate goal: power-personal, pure, and simple. A hungry ghost monk has the following class features."),
            source_page: Some("p.110"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Hungry Ghost Monk],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkStunningFist,TYPE.MonkPurityOfBody,TYPE.MonkWholenessOfBody,TYPE.MonkDiamondBody,TYPE.MonkDiamondSoul]"]),
            replaces: Some(&["MonkStunningFist", "MonkPurityOfBody", "MonkWholenessOfBody", "MonkDiamondBody", "MonkDiamondSoul"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Hungry Ghost Monk ~ Punishing Kick", at_level: 1, description: Some("At 1st level, a hungry ghost monk gains Punishing Kick as a bonus feat, even if he does not meet the prerequisites. At 10th level, and every five levels thereafter, the monk can push the target of his Punishing Kick an additional 5 feet (10 feet at 10th level, 15 feet at 15th level, and 20 feet at 20th level). At 15th level, he can instead choose to push the target 5 feet and knock the target prone with the same attack. The target still gets a saving throw to avoid being knocked prone. This ability replaces Stunning Fist."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Hungry Ghost Monk ~ Steal Ki", at_level: 5, description: Some("At 5th level, a hungry ghost monk can steal ki from other creatures, though this ability is controversial in some circles of monks, who see it as nothing less than a form of vampirism. If the monk scores a confirmed critical hit against a living enemy or reduces a living enemy to 0 or fewer hit points, he can steal some of that creature's ki. This ability replenishes 1 spent ki point to the monk's ki pool, as long as the monk has at least 1 ki point in his pool. He cannot exceed his ki pool's maximum. At 11th level, each time the monk successfully steals ki, he can make an immediate saving throw against one disease he is suffering from. There is no penalty for failing this saving throw. The monk gains a bonus equal to his Wisdom modifier on the saving throw. This ability replaces purity of body."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Hungry Ghost Monk ~ Life Funnel", at_level: 7, description: Some("At 7th level, a hungry ghost monk can steal a creature's life force to replenish his own. If the monk has at least 1 ki point in his ki pool and scores a confirmed critical hit against a living enemy or reduces a living enemy to 0 or fewer hit points, he heals a number of hit points equal to his monk level. As with steal ki, some monks believe that life funnel is an unsavory act, no better than what the undead do to the living. A monk with this ability cannot steal both ki and hit points at the same time. This ability replaces wholeness of body."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Hungry Ghost Monk ~ Life from a Stone", at_level: 11, description: Some("At 11th level, a hungry ghost monk can steal ki or life force from any creature, not just living creatures. If the monk has at least 1 ki point in his pool, he gains the benefit of life funnel and steal ki when he confirms a critical hit against any creature or reduces any creature to 0 or fewer hit points. This ability replaces diamond body."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Hungry Ghost Monk ~ Sipping Demon", at_level: 13, description: Some("A 13th level, a hungry ghost monk gains 1 temporary hit point each time he hits an enemy with a melee attack. The monk gains a number of temporary hit points equal to his Wisdom modifier when he scores a critical hit. The maximum number of temporary hit points the monk can have is equal to his monk level. The temporary hit points disappear 1 hour later. The monk can only use this ability when he has at least 1 ki point in his ki pool. This ability is a proscribed manipulation of ki considered by many good monks to be a corruption. The ability replaces diamond soul."), benefit: None },
            ],
        },
        // Monk Archetype ~ Ki Mystic -- apg_abilities_class.lst:2292
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Ki Mystic",
            subject: "Monk",
            archetype_name: "Ki Mystic",
            description: Some("The ki mystic believes that violence is sometimes necessary, but knowing and understanding is the true root of perfection. Through meditation and spiritual visions, a ki mystic can see beyond the veil of reality to the underlying truth of all existence. A ki mystic has the following class features."),
            source_page: Some("p.111"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Ki Mystic],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkStillMind,TYPE.MonkPurityOfBody,TYPE.MonkDiamondBody,TYPE.MonkDiamondSoul,TYPE.MonkEmptyBody]"]),
            replaces: Some(&["MonkStillMind", "MonkPurityOfBody", "MonkDiamondBody", "MonkDiamondSoul", "MonkEmptyBody"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Ki Mystic ~ Ki Mystic", at_level: 3, description: Some("At 3rd level, a ki mystic gains a pool of ki points equal to his Wisdom modifier. The pool increases to 1/2 his monk level + his Wisdom modifier + 2 at level 4. If the monk has at least 1 point of ki in his ki pool, he gains a +2 bonus on all Knowledge skill checks. As a swift action, the monk can spend 1 ki point immediately before making an ability, or skill check to gain a +4 insight bonus on the check. This ability replaces still mind.[Ki Points %1]|KiPoints"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ki Mystic ~ Mystic Insight", at_level: 5, description: Some("At 5th level, a ki mystic becomes apt at giving just the right word of advice in just the nick of time. As an immediate action, the monk can spend 2 ki points to grant an ally within 30 feet the ability to reroll a single attack roll or saving throw. The ally must be able to hear the monk to gain the reroll benefit. This ability replaces purity of body."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ki Mystic ~ Mystic Visions", at_level: 11, description: Some("At 11th level, a ki mystic may receive mystic visions when he rests. These visions can come as a dream, an epiphany, or even as the voice of an old friend whispering in the monk's mind. The effect is similar to a divination spell with a caster level equal to the monk's level. The divination has no casting time; it is just part of the normal dreams or visions that occur every night. Using this ability costs 2 ki points that are removed from the next day's total. This ability replaces diamond body."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ki Mystic ~ Mystic Prescience", at_level: 13, description: Some("At 13th level, a ki mystic gains a +2 insight bonus to AC and CMD. At 20th level, the bonus increases to +4. This ability replaces diamond soul."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ki Mystic ~ Mystic Persistence", at_level: 19, description: Some("At 19th level, a ki mystic can create an aura once per day as a swift action at the cost of at least 2 points of ki. The aura emanates out to a 20-foot radius. The monk and all allies within the aura can roll two dice when making an attack roll or a saving throw and take the better result. The aura lasts for 1 round, plus an additional round for every 2 ki points spent when the monk created the aura. The monk can dismiss the aura at any time as a free action, but the ki points for the full duration of the aura are lost. This ability replaces empty body."), benefit: None },
            ],
        },
        // Monk Archetype ~ Monk of the Empty Hand -- apg_abilities_class.lst:2293
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Monk of the Empty Hand",
            subject: "Monk",
            archetype_name: "Monk of the Empty Hand",
            description: Some("The monk of the empty hand eschews normal weapons in favor of whatever is lying around-rocks, chair legs, flagons of ale, even a simple quill pen all become deadly weapons in the hands of such a monk. A monk of the empty hand draws on his own ki to infuse his improvised weapons with power, and can transform a broken bottle into a magical weapon."),
            source_page: Some("p.111"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Monk of the Empty Hand],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkWeaponProficiencies,TYPE.MonkStillMind,TYPE.MonkPurityOfBody,TYPE.MonkDiamondBody]"]),
            replaces: Some(&["MonkWeaponProficiencies", "MonkStillMind", "MonkPurityOfBody", "MonkDiamondBody"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Monk of the Empty Hand ~ Weapon and Armor Proficiency", at_level: 1, description: Some("Monks of the empty hand are proficient with the shuriken only. A monk of the empty hand treats normal weapons as improvised weapons with the following equivalencies (substituting all of their statistics for the listed weapon): a light weapon functions as a light hammer, a one-handed weapon functions as a club, and a two-handed weapon functions as a quarterstaff. This replaces the normal monk weapon proficiencies."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Monk of the Empty Hand ~ Flurry of Blows", at_level: 1, description: Some("Starting at 1st level, a monk of the empty hand can make a flurry of blows using any combination of unarmed strikes or attacks with an improvised weapon. He may not make a flurry of blows with any other weapons, including special monk weapons. A monk of the empty hand's flurry of blows otherwise functions as normal for a monk of his level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Monk of the Empty Hand ~ Bonus Feat", at_level: 1, description: Some("A monk of the empty hand adds the following feats to his list of bonus feats at 6th level: Improved Dirty Trick*, Improved Steal*, and Improvised Weapon Mastery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Monk of the Empty Hand ~ Versatile Improvisation", at_level: 3, description: Some("At 3rd level, as a swift action, a monk of the empty hand may use an improvised weapon to deal damage as if it were another type (bludgeoning, piercing, or slashing) for 1 round, regardless of the weapon's normal damage type. This ability replaces still mind."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Monk of the Empty Hand ~ Ki Pool", at_level: 4, description: Some("At 4th level, in addition to the normal abilities of his ki pool, a monk of the empty hand may spend 1 point from his ki pool to increase the range increment for an improvised thrown weapon or shuriken by 20 feet for 1 round."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Monk of the Empty Hand ~ Ki Weapons", at_level: 5, description: Some("At 5th level, a monk of the empty hand may spend 1 point from his ki pool as a swift action to deal damage equal to his unarmed strike damage with an improvised weapon for 1 round. At 11th level, the monk may spend ki to grant an enhancement bonus or magical weapon abilities to an improvised weapon for 1 round, at the rate of 1 point of ki per +1 bonus or its equivalent. The monk may not spend more than 3 points of ki at one time for this purpose. For example, a monk can spend 2 points of ki to give his improvised weapon a +1 enhancement bonus and the ki focus quality, or just the flaming burst quality. At 15th level, the limit increases to 5 ki per round. The monk may use this ability to add magical weapon qualities to improvised weapons that could not normally have such a quality, such as adding the disruption quality to a slashing weapon, or the vorpal quality to a bludgeoning weapon. This ability replaces purity of body and diamond body."), benefit: None },
            ],
        },
        // Monk Archetype ~ Monk of the Four Winds -- apg_abilities_class.lst:2294
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Monk of the Four Winds",
            subject: "Monk",
            archetype_name: "Monk of the Four Winds",
            description: Some("The monk of the four winds is connected to the natural world in a way few other creatures-even other monks- can hope to match. He can call upon the elements and the spirits of the world in times of need, and as he nears his goal of perfection, he gains the ability to slow down time and even defeat death itself. A monk of the four winds has the following class features."),
            source_page: Some("p.112"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Monk of the Four Winds],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkStunningFist,TYPE.MonkAbundantStep,TYPE.MonkPerfectSelf,TYPE.MonkTimelessBody]"]),
            replaces: Some(&["MonkStunningFist", "MonkAbundantStep", "MonkPerfectSelf", "MonkTimelessBody"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Monk of the Four Winds ~ Elemental Fist", at_level: 1, description: Some("At 1st level, a monk of the four winds gains Elemental Fist as a bonus feat, even if he does not meet the prerequisites. At 5th level, and every five levels thereafter, the monk increases the damage of his Elemental Fist by 1d6 (2d6 at 5th level, 3d6 at 10th level, and so on). This ability replaces Stunning Fist."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Monk of the Four Winds ~ Slow Time", at_level: 12, description: Some("At 12th level, a monk of the four winds can use his ki to slow time or quicken his movements, depending on the observer. As a swift action, the monk can expend 6 ki points to gain three standard actions during his turn instead of just one. The monk can use these actions to do the following: take a melee attack action, use a skill, use an extraordinary ability, or take a move action. The monk cannot use these actions to cast spells or use spell-like abilities, and cannot combine them to take full-attack actions. Any move actions the monk makes this turn do not provoke attacks of opportunity. This ability replaces abundant step."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Monk of the Four Winds ~ Aspect Master", at_level: 17, description: Some("At 17th level, a monk of the four winds must choose an aspect of one of the great spirits of the world. Once made, this choice cannot be changed. This spirit grants the monk a new appearance and new abilities, as well as changing or augmenting the monk's personality in some way. Once this choice is made, it cannot be changed. The monk must abide by the alignment restrictions of the aspect. If the monk ever changes his alignment to something outside the aspect's alignment restrictions, he loses this ability and cannot regain it unless his alignment later changes again to match that of the aspect. This ability replaces timeless body."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Monk of the Four Winds ~ Immortality", at_level: 20, description: Some("At 20th level, a monk of the four winds no longer ages. He remains in his current age category forever. Even if the monk comes to a violent end, he spontaneously reincarnates (as the spell) 24 hours later in a place of his choosing within 20 miles of the place he died. The monk must have visited the place in which he returns back to life at least once. This ability replaces perfect self."), benefit: None },
            ],
        },
        // Monk Archetype ~ Monk of the Healing Hand -- apg_abilities_class.lst:2295
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Monk of the Healing Hand",
            subject: "Monk",
            archetype_name: "Monk of the Healing Hand",
            description: Some("Monks of the healing hand seek perfection through helping others. By focusing their meditations on the flow of life within themselves and all creation they gain an understanding of how to share their ki with others, healing wounds and even bringing the dead back to life. For such a monk, sacrificing himself to save another is the surest way to achieve transcendence. A monk of the healing hand has the following class features."),
            source_page: Some("p.113"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Monk of the Healing Hand],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkWholenessOfBody,TYPE.MonkDiamondBody,TYPE.MonkQuiveringPalm,TYPE.MonkPerfectSelf]"]),
            replaces: Some(&["MonkWholenessOfBody", "MonkDiamondBody", "MonkQuiveringPalm", "MonkPerfectSelf"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Monk of the Healing Hand ~ Ancient Healing Hand", at_level: 7, description: Some("At 7th level, a monk of the healing hand can heal another creature's wounds with a touch. As a full-round action, the monk can spend 2 ki points to heal a number of hit points equal to the monk's level. He needs at least one hand free to use this ability, and cannot heal himself. If the action is interrupted, the subject heals no hit points, and the ki points are lost. This ability replaces wholeness of body."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Monk of the Healing Hand ~ Ki Sacrifice", at_level: 11, description: Some("At 11th level, a monk of the healing hand can use his entire ki pool to bring a person back to life. It takes 1 hour to perform this ritual. At the end of the ritual, the monk sacrifices all of his ki in order to cast raise dead (as the spell) with a caster level equal to his monk level. The ritual uses all of the ki in the monk's ki pool; the monk must have at least 6 points of ki in his ki pool to use this ability. At 15th level, the monk may sacrifice his ki to cast resurrection. The monk must have at least 8 points of ki in his ki pool to use this ability. These rituals do not require material components. When this ability is used, the monk's ki pool is not replenished until 24 hours have passed. This ability replaces both diamond body and quivering palm."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Monk of the Healing Hand ~ True Sacrifice", at_level: 20, description: Some("At 20th level, in a final selfless act, a monk of the healing hand can draw in his entire ki, which then explodes outward in a 50-foot-radius emanation. All dead allies within the emanation are brought back to life, as if they were the subject of a true resurrection spell with a caster level equal to the monk's level. When the monk does this, he is truly and utterly destroyed. A monk destroyed in this way can never come back to life, not even by way of a wish or miracle spell or by the power of a deity. Furthermore, the monk's name can never be spoken or written down again. All written mentions of his name become nothing more than a blank space. This ability replaces perfect self."), benefit: None },
            ],
        },
        // Monk Archetype ~ Monk of the Lotus -- apg_abilities_class.lst:2296
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Monk of the Lotus",
            subject: "Monk",
            archetype_name: "Monk of the Lotus",
            description: Some("Monks are warriors who hone their bodies into deadly weapons, but some monks eschew violence in favor of a more peaceful philosophy. While a monk of the lotus realizes that combat cannot always be avoided-and is more than capable in a fight-he understands that all creatures are connected, and to harm another is to harm the self. Instead, he strives to find peaceful resolutions to conflicts, and in doing so, hopes to achieve inner peace. A monk of the lotus has the following class features."),
            source_page: Some("p.114"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Monk of the Lotus],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkStunningFist,TYPE.MonkAbundantStep,TYPE.MonkQuiveringPalm,TYPE.MonkTongueOfTheSunAndMoon]"]),
            replaces: Some(&["MonkStunningFist", "MonkAbundantStep", "MonkQuiveringPalm", "MonkTongueOfTheSunAndMoon"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Monk of the Lotus ~ Touch of Serenity", at_level: 1, description: Some("At 1st level, a monk of the lotus gains Touch of Serenity as a bonus feat, even if he does not meet the prerequisites. At 6th level, and every six levels thereafter, the duration of Touch of Serenity increases by 1 round. Each round on its turn, the target may attempt a new Will save to end the effect. This duration does not stack; only the longest remaining duration applies. This ability replaces Stunning Fist."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Monk of the Lotus ~ Touch of Surrender", at_level: 12, description: Some("At 12th level, a monk of the lotus makes a foe into a friend with a single show of mercy. As an immediate action, when one of his melee attacks would reduce a creature to 0 or fewer hit points, the monk can spend 6 ki points to make the target of that attack surrender. When the target surrenders, it is reduced to 0 hit points, becomes disabled, and is charmed, as if the monk had cast charm monster with a caster level equal to the monk's level. The target does not get a saving throw against this effect. This charm lasts until its duration expires, until the monk dismisses it or uses it on another creature, or until the target is again reduced to 0 or fewer hit points, whichever happens first. The monk can only have one creature charmed with touch of surrender at a time. This is a mind-affecting charm effect. This ability replaces abundant step."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Monk of the Lotus ~ Touch of Peace", at_level: 15, description: Some("At 15th level, a monk of the lotus can set up vibrations within the body of another creature to win over the creature's mind. The monk can use touch of peace once per day, and must announce his intent before making his attack roll. On a successful hit, the attack deals no damage, but the target is charmed as if the monk had cast charm monster with a caster level equal to the monk's level. The target does not get a saving throw against this effect. The creature is charmed for 1 day per level. If the monk or his allies attack the charmed creature, or if the monk asks or commands the charmed creature to take hostile actions, the effect ends. This is a mind-affecting charm effect. This ability replaces quivering palm."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Monk of the Lotus ~ Learned Master", at_level: 17, description: Some("At 17th level, a monk of the lotus gains all Knowledge skills and the Linguistics skill as class skills. The monk uses Wisdom instead of Intelligence as the key ability for these skills. This ability replaces tongue of the sun and the moon."), benefit: None },
            ],
        },
        // Monk Archetype ~ Monk of the Sacred Mountain -- apg_abilities_class.lst:2297
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Monk of the Sacred Mountain",
            subject: "Monk",
            archetype_name: "Monk of the Sacred Mountain",
            description: Some("The monk of the sacred mountain finds strength and power in the earth beneath his feet. Rather than spinning though the battlefield with the fluid motion of the river, he roots himself to the ground, as immovable and unshakable as the stones of the mountain. A monk of the sacred mountain has the following class features."),
            source_page: Some("p.114"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Monk of the Sacred Mountain],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkEvasion,TYPE.MonkSlowFall,TYPE.MonkHighJump,TYPE.MonkImprovedEvasion,TYPE.MonkTongueOfTheSunAndMoon]"]),
            replaces: Some(&["MonkEvasion", "MonkSlowFall", "MonkHighJump", "MonkImprovedEvasion", "MonkTongueOfTheSunAndMoon"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Monk of the Sacred Mountain ~ Iron Monk", at_level: 2, description: Some("At 2nd level, a monk of the sacred mountain gains Toughness as a bonus feat. In addition, the monk gains a +1 natural armor bonus. This ability replaces evasion."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Monk of the Sacred Mountain ~ Bastion Stance", at_level: 4, description: Some("At 4th level, a monk of the sacred mountain becomes like stone, nearly impossible to move when he stands his ground. If the monk starts and ends his turn in the same space, he cannot be knocked prone or forcibly moved until the start of his next turn, except by mind-affecting or teleportation effects. At 16th level, he is immune to any attempts to force him to move, even mind-affecting and teleportation effects. This ability replaces slow fall."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Monk of the Sacred Mountain ~ Iron Limb Defense", at_level: 5, description: Some("At 5th level, a monk of the sacred mountain can deflect blows with an active defense that complements his bastion stance. If the monk starts and ends his turn in the same space, he gains a +2 shield bonus to AC and CMD until the start of his next turn. As a swift action, he can spend 1 ki point to increase this bonus to +4. This ability replaces high jump."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Monk of the Sacred Mountain ~ Adamantine Monk", at_level: 9, description: Some("At 9th level, a monk of the sacred mountain has muscles so strong and skin so resilient that he gains DR 1/-. This DR increases by 1 for every three levels thereafter. As a swift action, the monk can spend 1 ki point to double his DR until the beginning of his next turn. This ability replaces improved evasion."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Monk of the Sacred Mountain ~ Vow of Silence", at_level: 17, description: Some("At 17th level, a monk of the sacred mountain becomes as impassive as stone, making a vow of silence in exchange for greater abilities. The monk gains a +2 insight bonus to AC and CMD and a +4 bonus on Sense Motive, Stealth, and Perception checks. The monk does not lose the capacity for speech, but if he ever speaks, he loses this feature for 24 hours. This ability replaces tongue of the sun and the moon."), benefit: None },
            ],
        },
        // Monk Archetype ~ Weapon Adept -- apg_abilities_class.lst:2298
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Weapon Adept",
            subject: "Monk",
            archetype_name: "Weapon Adept",
            description: Some("While all monks train in both unarmed combat and with weapons, the weapon adept seeks to become one with his weapons, transforming them into perfect extensions of his own body. Through such training, a weapon adept seeks to attain perfection by becoming a living weapon himself. A weapon adept has the following class features."),
            source_page: Some("p.114"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Weapon Adept],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkStunningFist,TYPE.MonkEvasion,TYPE.MonkImprovedEvasion,TYPE.MonkTimelessBody,TYPE.MonkPerfectSelf]"]),
            replaces: Some(&["MonkStunningFist", "MonkEvasion", "MonkImprovedEvasion", "MonkTimelessBody", "MonkPerfectSelf"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Weapon Adept ~ Perfect Strike", at_level: 1, description: Some("At 1st level, a weapon adept gains Perfect Strike as a bonus feat, even if he does not meet the prerequisites. At 10th level, the monk can roll his attack roll three times and take the higher result. If one of these rolls is a critical threat, he can choose which one of his other two rolls to use as his confirmation roll. This ability replaces Stunning Fist."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Weapon Adept ~ Way of the Weapon Master", at_level: 2, description: Some("At 2nd level, a weapon adept gains Weapon Focus as a bonus feat with one of his monk weapons. At 6th level, the monk gains Weapon Specialization with the same weapon as a bonus feat, even if he does not meet the prerequisites. This ability replaces evasion."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Weapon Adept ~ Evasion", at_level: 9, description: Some("At 9th level, the monk gains evasion. This ability replaces improved evasion."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Weapon Adept ~ Uncanny Initiative", at_level: 17, description: Some("At 17th level, a weapon adept does not need to roll for initiative. He always treats his initiative roll as if it resulted in any number of his choosing (from 1 to 20). This ability replaces timeless body."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Weapon Adept ~ Pure Power", at_level: 20, description: Some("At 20th level, a weapon adept forsakes the ideals of the perfect self to become a bastion of the physical and mental virtues monks hold dear. The monk gains a +2 bonus to Strength, Dexterity, and Wisdom. This ability replaces perfect self."), benefit: None },
            ],
        },
        // Paladin Archetype ~ Divine Defender -- apg_abilities_class.lst:2471
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Divine Defender",
            subject: "Paladin",
            archetype_name: "Divine Defender",
            description: Some("Some paladins see themselves as the last line of defense between the teeming hordes of evil and the innocent folk trying to make a living in a harsh, unforgiving world. These defenders spend their lives protecting others and taking on foes that the common man should not even know exist. To aid them in their holy mission, they have special powers to protect themselves and those around them. The divine defender has the following class features."),
            source_page: Some("p.116"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Divine Defender],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinMercy,TYPE.PaladinDivineBond]"]),
            replaces: Some(&["PaladinMercy", "PaladinDivineBond"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Divine Defender ~ Shared Defense", at_level: 3, description: Some("At 3rd level, a divine defender can spend one use of her lay on hands ability as a standard action to grant all adjacent allies (including paladins) a bonus. At 3rd level, adjacent allies receive a +1 sacred bonus to their AC and CMD and on their saving throws. These bonuses last for a number of rounds equal to the divine defender's Charisma modifier. At 9th level and 15th level, this bonus increases by +1. At 6th level, these bonuses are granted to all allies within 10 feet, and allies that are at fewer than 0 hit points within this area are automatically stabilized. At 12th level, these bonuses are granted to all allies within 15 feet, and allies within this area are immune to bleed damage. At 18th level, these bonuses are granted to all allies within 20 feet, and allies within this area gain a 25%% chance to negate any sneak attack or critical hit scored against them. This ability does not stack with the chance provided from the light, medium, or heavy fortification armor special abilities. These bonuses are cumulative with each other. Allies only benefit from these bonuses while in the listed area. This ability replaces mercy."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Divine Defender ~ Divine Bond", at_level: 5, description: Some("At 5th level, instead of forming a divine bond with her weapon or a mount, a divine defender can form a bond with her armor. As a standard action, a divine defender can enhance her armor by calling upon the aid of a celestial spirit. This bond lasts for 1 minute per paladin level. When called, the spirit causes the armor to shed light like a torch. At 5th level, the spirit grants the armor a +1 enhancement bonus. For every three levels beyond 5th, the armor gains another +1 enhancement bonus, to a maximum of +6 at 20th level. These bonuses can be added to the armor, stacking with existing armor bonuses to a maximum of +3, or they can be used to add any of the following armor properties (asterisks note new armor properties found in Chapter 7): champion*, ghost touch, heavy fortification, invulnerability, light fortification, moderate fortification, spell resistance (13, 15, 17, or 19). Adding these properties consumes an amount of bonus equal to the property's cost (see Table 15-4 of the Core Rulebook). In addition, the bonuses can be consumed at the listed amount to add any of the following armor properties: energy resistance for +3 bonus, improved energy resistance for +5 bonus, or righteous* for +4 bonus. These bonuses are added to any properties the armor already has, but duplicate abilities do not stack. If the armor is not magical, at least a +1 enhancement bonus must be added before any other properties can be added. The bonus and properties granted by the spirit are determined when the spirit is called and cannot be changed until the spirit is called again. The celestial spirit imparts no bonuses if the armor is worn by anyone other than the divine defender, but it resumes giving bonuses if the divine defender dons the armor again. A divine defender can use this ability once per day at 5th level, and one additional time per day for every four levels beyond 5th, to a total of four times per day at 17th level. If a suit of armor with a celestial spirit is destroyed, the divine defender loses the use of this ability for 30 days, or until she gains a level, whichever comes first. During this 30-day period, the divine defender takes a -1 penalty on attack and weapon damage rolls."), benefit: None },
            ],
        },
        // Paladin Archetype ~ Hospitaler -- apg_abilities_class.lst:2472
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Hospitaler",
            subject: "Paladin",
            archetype_name: "Hospitaler",
            description: Some("Paladins are known for their charity and for tending to the sick. The hospitaler takes to this calling above all others, spending much of her time healing the poor, and giving aid and succor to those in need. The hospitaler has the following class features."),
            source_page: Some("p.116"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Hospitaler],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinSmiteEvil,TYPE.PaladinChannelPositiveEnergy,TYPE.PaladinAuraOfJustice]"]),
            replaces: Some(&["PaladinSmiteEvil", "PaladinChannelPositiveEnergy", "PaladinAuraOfJustice"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Hospitaler ~ Smite Evil", at_level: 1, description: Some("This functions as the paladin ability, but the hospitaler can smite evil one additional time per day at 7th level, and every six levels thereafter (instead of 4th level and every three levels thereafter)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Hospitaler ~ Channel Positive Energy", at_level: 4, description: Some("You can unleash a wave of positive energy. You must choose to deal %1d%2 points of positive energy damage to undead creatures or to heal living creatures of %1d%2 points of damage. Creatures that take damage from channeled energy receive a DC %3 Will save to halve the damage. You can use this ability %4 times per day.|PaladinChannelPositiveEnergyDice|PaladinChannelPositiveEnergyDieSize|PaladinChannelPositiveEnergyDC|PaladinChannelEnergyTimes"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Hospitaler ~ Aura of Healing", at_level: 11, description: Some("At 11th level, a hospitaler can expend one use of her channel positive energy ability to emit a 30-foot aura of healing for a number of rounds equal to her paladin level. Allies in this aura (including the hospitaler) automatically stabilize if below 0 hit points and are immune to bleed damage. In addition, allies (including the paladin) that spend at least 1 full round inside the aura are healed an amount of damage equal to their total number of Hit Dice and may make a saving throw against any afflictions they are suffering from, such as a curse, disease, or poison. This saving throw only counts toward curing the affliction and does not impose any penalty on a failed save. Allies can only be healed once by a use of this ability and they can only attempt additional saving throws once per day, even if they are exposed to this aura multiple times. This ability replaces aura of justice."), benefit: None },
            ],
        },
        // Paladin Archetype ~ Sacred Servant -- apg_abilities_class.lst:2473
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Sacred Servant",
            subject: "Paladin",
            archetype_name: "Sacred Servant",
            description: Some("Paladins as a general rule, venerate the gods of good and purity, but some take this a step further, dedicating themselves to a specific deity and furthering the cause of the faith. These sacred servants are rewarded for their devotion with additional spells and powerful allies. A sacred servant must select one deity to worship. This deity's alignment must be lawful good, lawful neutral, or neutral good. A sacred servant has the following class features."),
            source_page: Some("p.117"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Sacred Servant],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinAuraOfResolve,TYPE.PaladinSmiteEvil]"]),
            replaces: Some(&["PaladinAuraOfResolve", "PaladinSmiteEvil", "PaladinDivineBond"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Sacred Servant ~ Smite Evil", at_level: 1, description: Some("This functions as the paladin ability, but the sacred servant can smite evil one additional time per day at 7th level, and every six levels thereafter (instead of 4th level and every three levels thereafter). This replaces smite evil."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sacred Servant ~ Spells", at_level: 4, description: Some("At 4th level, when a sacred servant gains the ability to cast spells, she also chooses one domain associated with her deity. Her effective cleric level for this domain is equal to her paladin level -3. In addition, she also gains one domain spell slot for each level of paladin spells she can cast. Every day she must prepare the domain spell from her chosen domain in that spell slot."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sacred Servant ~ Divine Bond", at_level: 5, description: Some("At 5th level, instead of forming a divine bond with her weapon or a mount, a sacred servant forms a bond with her holy symbol. As a standard action, a sacred servant can bind a celestial spirit to her holy symbol for 1 minute per paladin level. When called, the spirit causes the sacred servant's holy symbol to shed light like a torch. At 5th level, the spirit grants one bonus. For every three levels beyond 5th, the spirit grants one additional bonus. These bonuses can be spent in a number of ways to grant the paladin enhanced abilities to channel positive energy and to cast spells. Each bonus can be used to grant one of the following enhancements: +1 caster level to any paladin spell cast, +1 to the DC to halve the damage of channel positive energy when used to harm undead, +1d6 to channel positive energy, +1 use/ day of lay on hands. These enhancements stack and can be selected multiple times. The enhancements granted by the spirit are determined when the spirit is called and cannot be changed until the spirit is called again. If the sacred servant increases her number of uses of lay on hands per day in this way, that choice is set for the rest of the day, and once used, these additional uses are not restored (even if the spirit is called again that day). The celestial spirit imparts no enhancements if the holy symbol is held by anyone other than the sacred servant, but resumes giving enhancements if returned to the sacred servant. A sacred servant can use this ability once per day at 5th level, and one additional time per day for every four levels beyond 5th, to a total of four times per day at 17th level. If a holy symbol with a celestial spirit is destroyed, the sacred servant loses the use of this ability for 30 days, or until she gains a level, whichever comes first. During this 30-day period, the sacred servant takes a -1 penalty on attack and weapon damage rolls."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sacred Servant ~ Call Celestial Ally", at_level: 8, description: Some("At 8th level, a sacred servant can call upon her deity for aid, in the form of a powerful servant. This allows the sacred servant to cast lesser planar ally once per week as a spell-like ability without having to pay the material component cost or the servant (for reasonable tasks). At 12th level, this improves to planar ally and at 16th level, this improves to greater planar ally. The sacred servant's caster level for this effect is equal to her paladin level. This ability replaces aura of resolve."), benefit: None },
            ],
        },
        // Paladin Archetype ~ Shining Knight -- apg_abilities_class.lst:2474
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Shining Knight",
            subject: "Paladin",
            archetype_name: "Shining Knight",
            description: Some("While paladins often are seen mounted atop a loyal steed, the shining knight is the true symbol of mounted bravery. They are never far from their steeds and are always clad in brightly polished armor. The shining knight has the following class features."),
            source_page: Some("p.117"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Shining Knight],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinBondedWeapon,TYPE.PaladinDivineHealth,TYPE.PaladinAuraofJustice]"]),
            replaces: Some(&["PaladinBondedWeapon", "PaladinDivineHealth", "PaladinAuraofJustice"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Shining Knight ~ Skilled Rider", at_level: 3, description: Some("At 3rd level, a shining knight does not take any penalty to her Ride skill due to her armor check penalty. In addition, any mount she is riding gains the benefit of her divine grace class feature, adding her Charisma bonus (if any) to its saving throws. This ability replaces divine health."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shining Knight ~ Divine Bond", at_level: 5, description: Some("Upon reaching 5th level, a shining knight must form a bond with a mount. This ability otherwise functions as the paladin ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shining Knight ~ Knight's Charge", at_level: 11, description: Some("At 11th level, whenever a mounted shining knight charges a foe, her movement does not provoke attacks of opportunity, for either her or her mount. In addition, if her target is also the target of her smite evil ability and the charge attack hits, the target must make a Will save DC %1 or be panicked for a number of rounds equal to %2 [1/2 the shining knight's level]. The DC of this save is equal to 10 + 1/2 the shining knight's level + the shining knight's Charisma modifier. This ability replaces aura of justice.|ShiningKnightDC|ShiningKnightDuration"), benefit: None },
            ],
        },
        // Paladin Archetype ~ Undead Scourge -- apg_abilities_class.lst:2475
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Undead Scourge",
            subject: "Paladin",
            archetype_name: "Undead Scourge",
            description: Some("Undead are an abomination in the eyes of the just and righteous. It is no surprise then that there are some paladins that dedicate themselves to wiping these unholy terrors from the world. The following are the class features of the undead scourge."),
            source_page: Some("p.117"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Undead Scourge],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinAuraOfJustice,TYPE.PaladinAuraOfResolve,TYPE.PaladinSmiteEvil]"]),
            replaces: Some(&["PaladinAuraOfJustice", "PaladinAuraOfResolve", "PaladinSmiteEvil"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Undead Scourge ~ Smite Evil", at_level: 1, description: Some("This functions as the paladin ability of the same name, but the undead scourge does not deal 2 points of damage per level on the first successful attack against evil dragons and evil outsiders. She does deal 2 points of damage per level on all smite attacks made against evil undead creatures. You can call out to the powers of good to aid you in your struggle against evil %1 times per day. As a swift action, you choose one target within sight to smite. If this target is an evil undead, you add +%2 to your attack rolls and +%3 to all damage rolls made against the target of your smite. If the target of Smite Evil is an outsider with the evil subtype, an evil-aligned dragon, or an undead creature, the bonus to damage on the first successful attack increases to +%4. Regardless of the target, Smite Evil attacks automatically bypass any DR the creature might possess. In addition, while smite evil is in effect, you gain a +%5 deflection bonus to your AC against attacks made by the target of the smite. If you target a creature that is not evil, the smite is wasted with no effect. The Smite Evil effect remains until the target of the smite is dead or the next time you rest and regain your uses of this ability.|SmiteEvilTimes|SmiteEvilAttackBonus|SmiteEvilDamageBonus|SmiteEvilDamageBonus*2|SmiteEvilACBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Undead Scourge ~ Aura of Life", at_level: 8, description: Some("At 8th level, an undead scourge emits a 10-foot aura of life around her that weakens undead creatures. Undead in this aura take a -4 penalty on Will saves made to resist positive energy. In addition, undead in this aura do not regain hit points from channeled negative energy. This ability replaces aura of resolve."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Undead Scourge ~ Undead Annihilation", at_level: 11, description: Some("At 11th level, an undead scourge can expend one use of her smite evil ability as a standard action and make a single melee attack against an undead creature. If this attack hits, the undead creature must make a Will save DC %1 or be destroyed. The save DC is equal to 10 + 1/2 the undead scourge's level + the undead scourge's Charisma modifier. Undead with twice as many Hit Dice as the undead scourge are unaffected by this ability. If the attack misses, the smite evil is wasted without effect. This ability replaces aura of justice.|10+(PaladinLVL/2)+CHA"), benefit: None },
            ],
        },
        // Paladin Archetype ~ Warrior of the Holy Light -- apg_abilities_class.lst:2476
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Warrior of the Holy Light",
            subject: "Paladin",
            archetype_name: "Warrior of the Holy Light",
            description: Some("Some paladins use their gifts to focus on the holy light that shines within their souls. With the gifts of purity and redemption, these paladins spend much of their lives helping others find the true path. Unleashing this power takes patience and comes at a steep price. Warriors of the holy light have the following class features."),
            source_page: Some("p.118"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Warrior of the Holy Light],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinAuraOfFaith,TYPE.PaladinSpells]"]),
            replaces: Some(&["PaladinAuraOfFaith", "PaladinSpells"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Warrior of the Holy Light ~ Power of Faith", at_level: 4, description: Some("At 4th level, a warrior of the holy light learns to use the power of her faith to bolster her defenses and aid her allies. This class feature replaces the paladin's spells class feature. A warrior of the holy light does not gain any spells or spellcasting abilities, does not have a caster level, and cannot use spell trigger or spell completion magic items. At 4th level, the warrior of the holy light gains one additional use of her lay on hands ability per day. She gains one additional use of lay on hands per day for every four levels she attains beyond 4th. She can spend a use of her lay on hands ability to call upon. This causes a nimbus of light to emanate from the warrior of the holy light in a 30-foot radius. All allies in this area (including the warrior of the holy light) receive a +1 morale bonus to AC and on attack rolls, damage rolls, and saving throws against fear as long as they remain in the area of light. This power lasts for 1 minute. At 8th level, the nimbus of light heals the paladin and her allies, curing of them of 1d4 points of ability damage, as per the spell lesser restoration. A creature can only be healed in this way once per day. At 12th level, the nimbus of light is treated as daylight for the purposes of affecting creatures with sensitivity to light. In addition, the nimbus grants allies in the area resistance 10 to one type of energy, selected by the warrior of the holy light when this power is activated. At 16th level, the nimbus of light grants the warrior of the holy light and her allies protection from critical hits. There is a 25%% chance that critical hits made against the warrior of the holy light and her allies in the area are instead treated as normal hits. This does not stack with other abilities that grant similar protection (such as light fortification). At 20th level, the nimbus of light increases in size out to a range of 60 feet. In addition, all of its bonuses increase. The morale bonus to AC and on attack rolls, damage rolls, and saving throws against fear increases to +2. The amount of ability damage healed increases to 2d4. The energy resistance increases to 20 against one energy type. Finally, protection against critical hits increases to 50%%."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Warrior of the Holy Light ~ Shining Light", at_level: 14, description: Some("At 14th level, a warrior of the holy light can unleash a 30-foot burst of pure, white light as a standard action. Evil creatures within this burst take %1d6 points of damage for every two paladin levels and are blinded for 1 round. Evil dragons, evil outsiders, and evil undead are blinded for 1d4 rounds on a failed save. A Reflex save DC %2 halves this damage and negates the blindness. The DC of this save is equal to 10 + 1/2 the warrior of the holy light's level + the warrior of the holy light's Charisma modifier. Good creatures within this burst are healed %1d6 points of damage per two paladin levels and receive a +2 sacred bonus on ability checks, attack rolls, saving throws, and skill checks for 1 round. A warrior of the holy light can use this ability %3/day [once per day at 14th level plus one additional time per day at 17th and 20th levels. This ability replaces aura of faith.|ShiningLightDamage|ShiningLightDC|ShiningLightTimes"), benefit: None },
                ArchetypeGrant { grants_feature_key: "No Spellcasting ~ Paladin", at_level: 1, description: None, benefit: None },
            ],
        },
        // Ranger Archetype ~ Beast Master -- apg_abilities_class.lst:2664
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Beast Master",
            subject: "Ranger",
            archetype_name: "Beast Master",
            description: Some("Some rangers, particularly those in primitive lands or who were raised by animals, have unusually strong bonds with animals. Unique among rangers, they can bond with multiple animals of any kind, creating a menagerie of wild yet loyal creatures, like a strange family."),
            source_page: Some("p.124"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ Beast Master],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerClassSkills,TYPE.RangerHuntersBond,TYPE.RangerCombatStyleFeat_Six,TYPE.RangerCamouflage]"]),
            replaces: Some(&["RangerClassSkills", "RangerHuntersBond", "RangerCombatStyleFeat_Six", "RangerCamouflage"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Beast Master ~ Class Skills", at_level: 1, description: Some("A beast master's class skills are Acrobatics (Dex), Climb (Str), Craft (Int), Escape Artist (Dex), Handle Animal (Cha), Heal (Wis), Intimidate (Cha), Knowledge (nature) (Int), Perception (Wis), Ride (Dex), Stealth (Dex), Survival (Wis), and Swim (Wis). These replace the standard ranger class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Beast Master ~ Animal Companion", at_level: 4, description: Some("A beast master forms a close bond with an animal companion. This ability functions like the druid animal companion ability except that the ranger's effective druid level is equal to his ranger level - 3. The ranger gains a +2 bonus on wild empathy and Handle Animal checks made regarding his animal companion. Unlike a normal ranger, a beast master's choice of animal companion is not limited to a subset of all possible animal companion choices-he may choose freely among all animal companion choices, just as a druid can. The beast master may have more than one animal companion, but he must divide up his effective druid level between his companions to determine the abilities of each companion. For example, a beast master with an effective druid level of 4 can have one 4th-level companion, two 2nd-level companions, or one 1st-level and one 3rd-level companion. Each time a beast master's effective druid level increases, he must decide how to allocate the increase among his animal companions (including the option of adding a new 1st-level companion). Once an effective druid level is allocated to a particular companion, it cannot be redistributed while that companion is in the ranger's service (he must release a companion or wait until a companion dies to allocate its levels to another companion). The share spells animal companion ability does not give the ranger the ability to cast a single spell so that it affects all of his animal companions. This ability replaces hunter's bond."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Beast Master ~ Improved Empathic Link", at_level: 6, description: Some("The beast master gains an empathic link with all of his animal companions. This functions like an empathic link with a familiar, except the ranger can also see through a companion's eyes as a swift action, maintaining this connection as long as he likes (as long as the companion is within 1 mile) and ending it as a free action. The ranger can only see through the eyes of one companion at a time, and is blinded while maintaining this connection. This replaces the 6th-level combat style feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Beast Master ~ Strong Bond", at_level: 12, description: Some("At 12th level, the ranger strengthens his bond with his animal companions. The ranger's effective druid level for his animal companions is now equal to his ranger level; he may immediately allocate these additional levels to his companions as he sees fit. This ability replaces camouflage."), benefit: None },
            ],
        },
        // Ranger Archetype ~ Guide -- apg_abilities_class.lst:2665
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Guide",
            subject: "Ranger",
            archetype_name: "Guide",
            description: Some("Many rangers are loners, but some choose to use their familiarity with the land to guide others safely through the wilderness. The guide forgoes a favored enemy to focus on the task or foe at hand, and can pass his knowledge and luck on to his charges."),
            source_page: Some("p.125"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ Guide],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerFavoredEnemy,TYPE.RangerHuntersBond,TYPE.RangerEvasion,TYPE.RangerQuarry,TYPE.RangerImprovedQuarry,TYPE.RangerImprovedEvasion]"]),
            replaces: Some(&["RangerFavoredEnemy", "RangerHuntersBond", "RangerEvasion", "RangerQuarry", "RangerImprovedQuarry", "RangerImprovedEvasion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Guide ~ Ranger's Focus", at_level: 1, description: Some("At 1st level, once per day, the guide can focus on a single enemy within line of sight as a swift action. That creature remains the ranger's focus until it is reduced to 0 or fewer hit points or surrenders, or until the ranger designates a new focus, whichever occurs first. The ranger gains a +2 bonus on attack and damage rolls against the target of his focus. At 5th level, and every five levels thereafter, this bonus increases by +2. At 4th level, and every 3 levels thereafter, the ranger can use this ability one additional time per day. This ability replaces favored enemy."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Guide ~ Terrain Bond", at_level: 4, description: Some("At 4th level, the guide forms a bond with the land itself, enabling him to direct others in such terrain. When in his favored terrain, the ranger grants all allies within line of sight and that can hear him a +2 bonus on initiative checks and Perception, Stealth, and Survival skill checks. Also, as long as they travel with him, the ranger's allies leave no trail and can't be tracked. The ranger can choose for the group to leave a trail, or even specific members of the group to leave a trail if he so desires. This ability replaces hunter's bond."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Guide ~ Ranger's Luck", at_level: 9, description: Some("Upon reaching 9th level, once per day the guide can either reroll one of his attack rolls or force an enemy who just hit him with an attack to reroll the attack roll. The ranger must take the result of the second roll even if it is worse. A ranger can use this ability once per day at 9th level, plus one additional time per day at 14th and 19th levels. This ability replaces evasion."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Guide ~ Inspired Moment", at_level: 11, description: Some("At 11th level, the guide can have an inspired moment once per day as a free action. The ranger gains the following benefits until the end of his next turn. His speed increases by 10 feet. He can take an extra move or swift action on his turn. He gains a +4 bonus to AC and on attack rolls, skill checks, or ability checks. Finally, he automatically confirms any critical threat he scores. He can use this ability one additional time per day at 19th level. This ability replaces quarry and improved quarry."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Guide ~ Improved Ranger's Luck", at_level: 16, description: Some("Upon reaching 16th level, the ranger's luck increases. He gains a +4 bonus on his rerolls made with the ranger's luck ability, or if he forces an enemy to reroll an attack, that enemy takes a -4 penalty on the roll. This bonus or penalty is also applied on any roll to confirm critical hits. This ability replaces improved evasion."), benefit: None },
            ],
        },
        // Ranger Archetype ~ Horse Lord -- apg_abilities_class.lst:2666
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Horse Lord",
            subject: "Ranger",
            archetype_name: "Horse Lord",
            description: Some("Rangers of the plains use horses or other riding beasts to hunt their lands, forging a near-mystical relationship with their mounts. Horse lords are unparalleled mounted combatants, the envy of even the most dedicated cavalier. Though called \"horse lords\" as a generic term, these rangers are not restricted to horses for their animal companions- any creature the ranger can ride is included in these abilities."),
            source_page: Some("p.125"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ Horse Lord],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerHuntersBond,TYPE.RangerCamouflage,TYPE.RangerHideInPlainSight]"]),
            replaces: Some(&["RangerHuntersBond", "RangerCamouflage", "RangerHideInPlainSight"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Horse Lord ~ Combat Style Feat", at_level: 2, description: Some("At 2nd level, a horse lord must choose the mounted combat style."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Horse Lord ~ Mounted Bond", at_level: 4, description: Some("At 4th level, the horse lord forms a bond with an animal he can use as a mount, which becomes his animal companion. A Medium ranger can select a camel or a horse. A small ranger can select a pony or wolf, but can also select a boar or dog if he is at least 7th level. This ability functions like the druid animal companion ability except that the ranger's effective druid level is equal to his ranger level - 3. The ranger gains a +2 bonus on Handle Animal and Ride checks with his animal companion mount. This ability replaces hunter's bond."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Horse Lord ~ Strong Bond", at_level: 12, description: Some("At 12th level, the horse lord strengthens his bond with his mount. The ranger's effective druid level for his mount is now equal to his ranger level. This ability replaces camouflage."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Horse Lord ~ Spiritual Bond", at_level: 17, description: Some("At 17th level, the horse lord can grant his animal companion temporary hit points equal to his ranger level once per day. While these temporary hit points last, when his mount is within 30 feet of the him, he can choose to share the damage taken by his mount as if using shield other. This ability replaces hide in plain sight."), benefit: None },
            ],
        },
        // Ranger Archetype ~ Infiltrator -- apg_abilities_class.lst:2667
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Infiltrator",
            subject: "Ranger",
            archetype_name: "Infiltrator",
            description: Some("Some rangers study their favored enemies and learn their ways, applying this knowledge to their own abilities and using their foes' strengths against them. Infiltrators are willing to walk a mile in an enemy's shoes so as to learn eveything there is to know about their foes in order to more effectively hunt and kill them."),
            source_page: Some("p.125"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ Infiltrator],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerFavoredTerrain]"]),
            replaces: Some(&["RangerFavoredTerrain"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Infiltrator ~ Adaptation", at_level: 3, description: Some("At 3rd level, an infiltrator learns how to copy the unusual abilities of his prey. He chooses one type of creature he has selected as a favored enemy, such as aberrations. The ranger selects one ability or feat from the adaptation list for that type (see below). A ranger can use adaptations for 10 minutes per day per ranger level he possesses. This duration does not need to be consecutive, but it must be used in 10-minute increments. If the adaptation requires the ranger to make a more specific choice (such as what skill to use with Skill Focus), this choice is permanent and cannot be changed. At 8th, 13th, and 15th-level, the ranger chooses another one of his favored enemy types and selects one adaptation from that type's list, as well as an additional adaptation from any one list of a creature type he's selected (including the one just chosen, if so desired). The infiltrator can only use one adaptation at a time. This class ability replaces favored terrain."), benefit: None },
            ],
        },
        // Ranger Archetype ~ Shapeshifter -- apg_abilities_class.lst:2668
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Shapeshifter",
            subject: "Ranger",
            archetype_name: "Shapeshifter",
            description: Some("Most rangers venture into the wilderness, but there are some who let the wilderness seep into them. Whether by curse, disease, ancient rite, a slight lycanthropic influence in the blood, or the corrupting influence of chaos, these rangers embrace the wild to transform themselves into something untamed and feral. Shapeshifters are often held in awe, but are even more often feared."),
            source_page: Some("p.126"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ Shapeshifter],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerCamouflage,TYPE.RangerMasterHunter,TYPE.RangerFavoredTerrain]"]),
            replaces: Some(&["RangerCamouflage", "RangerMasterHunter", "RangerFavoredTerrain"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Shapeshifter ~ Combat Style Feat", at_level: 2, description: Some("At 2nd level, a shapeshifter ranger must choose the natural weapon combat style."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shapeshifter ~ Shifter's Blessing", at_level: 3, description: Some("At 3rd level, the shapeshifter can take on the aspects of a wild creature once per day as a swift action. He can remain in this form for a number of rounds equal to his ranger level + his Wisdom modifier. While in one of his shifter's blessing forms, the ranger gains the shapeshifter subtype. The shapeshifter must choose one of the following forms. Once this choice is made, it cannot be changed."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shapeshifter ~ Dual Form Shifter", at_level: 12, description: Some("At 12th level, when the shapeshifter takes on a shifter's blessing form, he can take on a hybrid of two of his forms. He gains the bonuses for both forms. This ability replaces camouflage."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shapeshifter ~ Master Shifter", at_level: 20, description: Some("At 20th level, the ranger's shifter's blessing forms improve, and he can take on true forms of beasts. The ranger can use dual form shifter with this ability, although he cannot use more than one polymorph effect at any one time. This ability replaces master hunter. The ranger's forms from shifter's blessing improve to the following."), benefit: None },
            ],
        },
        // Ranger Archetype ~ Skirmisher -- apg_abilities_class.lst:2669
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Skirmisher",
            subject: "Ranger",
            archetype_name: "Skirmisher",
            description: Some("Many rangers rely on spells, but there are some who eschew aid from divine powers for their own reasons. Skirmishers rely on their wits, their wisdom, and sometimes even instinct to aid in their quests."),
            source_page: Some("p.128"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ Skirmisher],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerSpells]"]),
            replaces: Some(&["RangerSpells"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "No Spellcasting ~ Ranger", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Skirmisher ~ Hunter's Tricks", at_level: 5, description: Some("A skirmisher ranger learns the use of %1 hunter's tricks, which typically grant a boon or bonus to the ranger or a nearby ally. A ranger can use these tricks %2 times per day. Tricks are usually swift actions, but sometimes move or free actions that modify a standard action, usually an attack action. Once a trick is chosen, it can't be retrained. A ranger cannot select an individual trick more than once. This ability replaces the ranger's spells class feature. Skirmishers do not gain any spells or spellcasting ability, do not have a caster level, and cannot use spell trigger and spell completion magic items.|SkirmisherTricks|SkirmisherTrickTimes"), benefit: None },
            ],
        },
        // Ranger Archetype ~ Spirit Ranger -- apg_abilities_class.lst:2670
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Spirit Ranger",
            subject: "Ranger",
            archetype_name: "Spirit Ranger",
            description: Some("Some rangers nurture a connection with the spirits that reside in all things. By communing with these spirits, the spirit ranger can gain glimpses of things to come."),
            source_page: Some("p.129"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ Spirit Ranger],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerHuntersBond,TYPE.RangerCamouflage]"]),
            replaces: Some(&["RangerHuntersBond", "RangerCamouflage"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Spirit Ranger ~ Spirit Bond", at_level: 4, description: Some("At 4th level, instead of forming a bond with his hunting companions or an animal companion, the spirit ranger forms a bond with the spirits of nature themselves. Each day, as long as he is within one of his favored terrains, the ranger can cast augury (Pathfinder RPG Core Rulebook 245) as a spell-like ability with a caster level equal to his ranger level. In addition, he can call upon these spirits to cast any one ranger spell that he is capable of casting, without having to prepare the spell. At 8th level, and every four levels thereafter, he can cast an additional spell in this way. This replaces hunter's bond."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spirit Ranger ~ Wisdom of the Spirits", at_level: 12, description: Some("At 12th level, the spirit ranger can use his augury spell-like ability even when he is not in one of his favored terrains. If he is within one of his favored terrains, the ranger can cast divination (Pathfinder RPG Core Rulebook 273) instead. Like augury, the caster level of the divination is equal to the ranger's level. This ability replaces camouflage."), benefit: None },
            ],
        },
        // Ranger Archetype ~ Urban Ranger -- apg_abilities_class.lst:2671
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Urban Ranger",
            subject: "Ranger",
            archetype_name: "Urban Ranger",
            description: Some("For the urban ranger, the streets and sewers of the city are just as dangerous as the barren wastelands or the deep forests."),
            source_page: Some("p.129"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ Urban Ranger],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerClassSkills,TYPE.RangerFavoredTerrain,TYPE.RangerEndurance,TYPE.RangerHideInPlainSight,TYPE.RangerWoodlandStride,TYPE.RangerCamouflage]"]),
            replaces: Some(&["RangerClassSkills", "RangerFavoredTerrain", "RangerEndurance", "RangerHideInPlainSight", "RangerWoodlandStride", "RangerCamouflage"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Urban Ranger ~ Class Skills", at_level: 1, description: Some("At 1st level, an urban ranger adds Disable Device and Knowledge (local) to his list of class skills and removes Handle Animal and Knowledge (nature) from his list of class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Urban Ranger ~ Favored Community", at_level: 3, description: Some("At 3rd level, the urban ranger forms a bond with a community. While inside the limits of this community, he gains a +2 bonus on initiative checks and Knowledge (local), Perception, Stealth, and Survival skill checks. An urban ranger traveling through his favored community leaves no trail and cannot be tracked (although he may leave a trail if he so desires). At 8th level, and every five levels thereafter, an urban ranger may select an additional favored community. In addition, at each such interval, the skill bonus and initiative bonus in any one favored community (including the one just selected, if so desired) increases by +2. For the purposes of this ability, a community is any settlement consisting of 100 or more individuals. The community may be larger than this minimum. Outlying farms, fields, and houses are not considered part of a community. This ability replaces favored terrain."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Urban Ranger ~ Trapfinding", at_level: 3, description: Some("At 3rd level, an urban ranger can find and disable traps, as the rogue class feature of the same name. This ability replaces endurance."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Urban Ranger ~ Push Through", at_level: 7, description: Some("At 7th level, an urban ranger is never slowed by difficult terrain in his favored communities. In addition, he can move through the space occupied by local citizens as if they were allies. This does not apply to creatures intent on harming the ranger. Areas that are enchanted or magically manipulated to impede motion, however, still affect him. This replaces woodland stride."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Urban Ranger ~ Blend In", at_level: 12, description: Some("An urban ranger of 12th level or higher can use his Stealth bonus in place of a Disguise skill check in any of his favored communities. This disguise does not take an action to don. He must make a check whenever someone attempts to pick him out from the local citizens. If his check is successful, he blends into the crowd. While not invisible, enemies do not notice his presence and take no actions against him unless they are taking actions against the local citizens in general. This replaces camouflage."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Urban Ranger ~ Invisibility Trick", at_level: 17, description: Some("At 17th level, the urban ranger can cast improved invisibility on himself as a wizard of his ranger level as a swift action. He can use this spell-like ability a number of times per day equal to his Wisdom modifier (minimum 1). This ability replaces hide in plain sight."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Acrobat -- apg_abilities_class.lst:2941
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Acrobat",
            subject: "Rogue",
            archetype_name: "Acrobat",
            description: Some("Agility and daring are both excellent rogue traits, and their confluence can create spectacular feats of acrobatics. Whether they are daring thieves, infiltrating assassins, or intrepid spies, proper training in acrobatics is a valuable boon for rogues."),
            source_page: Some("p.132"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Acrobat],[!PREABILITY:1,CATEGORY=Archetype,TYPE=RogueTrapfinding,TYPE=RogueTrapSense]"]),
            replaces: Some(&["RogueTrapfinding", "RogueTrapSense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Acrobat ~ Expert Acrobat", at_level: 1, description: Some("At 1st level, an acrobat does not suffer any armor check penalties on Acrobatics, Climb, Fly, Sleight of Hand, or Stealth skill checks while wearing light armor. When she is not wearing armor, she gains a +2 competency bonus on Acrobatics and Fly skill checks. This ability replaces trapfinding."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Acrobat ~ Second Chance", at_level: 3, description: Some("At 3rd level, an acrobat can reroll any Acrobatics, Climb, or Fly skill check she has just made. This reroll is made at a -5 penalty. She must take the second result, even if it is worse. An acrobat can use this ability only once on any given skill check. She can use this ability once per day at 3rd level, plus one additional time per day for every 3 levels beyond 3rd. This ability replaces trap sense."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Burglar -- apg_abilities_class.lst:2942
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Burglar",
            subject: "Rogue",
            archetype_name: "Burglar",
            description: None,
            source_page: Some("p.132"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Burglar],[!PREABILITY:1,CATEGORY=Archetype,TYPE=RogueUncannyDodge,TYPE=RogueImprovedUncannyDodge]"]),
            replaces: Some(&["RogueUncannyDodge", "RogueImprovedUncannyDodge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Rogue ~ Careful Disarm", at_level: 4, description: Some("Whenever you attempt to disarm a trap using Disable Device, you do not spring the trap unless you fail by 10 or more. If you do set off a trap you were attempting to disarm, you adds double your trap sense bonus to avoid the trap. This ability replaces uncanny dodge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Burglar ~ Distraction", at_level: 8, description: Some("At 8th level, whenever a burglar is detected while using Stealth, she can immediately attempt a Bluff skill check opposed by the Sense Motive skill of the creature that spotted her. If this check succeeds, the target assumes that the noise was something innocent and disregards the detection. This only functions if the creature cannot see the rogue. This ability can only be used once during a given Stealth attempt. If the same creature detects the rogue's presence again, the ability has no effect. This ability replaces improved uncanny dodge."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Cutpurse -- apg_abilities_class.lst:2943
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Cutpurse",
            subject: "Rogue",
            archetype_name: "Cutpurse",
            description: None,
            source_page: Some("p.132"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Cutpurse],[!PREABILITY:1,CATEGORY=Archetype,TYPE=RogueTrapfinding,TYPE=RogueTrapSense]"]),
            replaces: Some(&["RogueTrapfinding", "RogueTrapSense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Cutpurse ~ Measure the Mark", at_level: 1, description: Some("When a cutpurse makes a Sleight of Hand check to take something from a creature, the target makes its Perception check before the rogue makes her Sleight of Hand check, and the rogue knows the Perception check result. She can decide whether or not to make the check based on the results of the target's Perception check. If the rogue elects not to make the check, she can make a Bluff check, opposed by the target's Sense Motive, to prevent the target from noticing the attempt. This ability replaces trapfinding."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cutpurse ~ Stab and Grab", at_level: 3, description: Some("At 3rd level, as a full-round action, a cutpurse can make an attack and also make a Sleight of Hand check to steal something from the target of the attack. If the attack deals sneak attack damage, the rogue can use Sleight of Hand to take an item from the creature during combat; otherwise this ability can only be used in a surprise round before the target has acted. If the attack is successful, the target takes a -5 penalty on the Perception check to notice the theft. This ability replaces trap sense."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Investigator -- apg_abilities_class.lst:2944
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Investigator",
            subject: "Rogue",
            archetype_name: "Investigator",
            description: None,
            source_page: Some("p.133"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Investigator],[!PREABILITY:1,CATEGORY=Archetype,TYPE=RogueTrapfinding]"]),
            replaces: Some(&["RogueTrapfinding"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Investigator ~ Follow Up", at_level: 1, description: Some("An investigator can roll twice on any Diplomacy check made to gather information, and receives the information for both results. This takes the same amount of time as one check. If the lesser of the two checks reveals false information, the rogue is aware of it. False information is not revealed in this way if the people she questioned do not know it to be false. This ability replaces trapfinding."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Poisoner -- apg_abilities_class.lst:2945
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Poisoner",
            subject: "Rogue",
            archetype_name: "Poisoner",
            description: None,
            source_page: Some("p.134"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Poisoner],[!PREABILITY:1,CATEGORY=Archetype,TYPE=RogueTrapfinding,TYPE=RogueTrapSense]"]),
            replaces: Some(&["RogueTrapfinding", "RogueTrapSense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Rogue ~ Poison Use", at_level: 1, description: Some("You are trained in the use of poison and cannot accidentally poison yourself when applying poison to a blade. This ability replaces trap sense."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Poisoner ~ Master Poisoner", at_level: 3, description: Some("At 3rd level, a poisoner can use Craft (alchemy) to change the type of a poison. This requires 1 hour of work with an alchemist's lab and a Craft (alchemy) skill check with a DC equal to the poison's DC. If successful, the poison's type changes to contact, ingested, inhaled, or injury. If the check fails, the poison is ruined. The poisoner also receives a bonus on Craft (alchemy) skill checks when working with poison equal to 1/2 her rogue level. This ability replaces trap sense."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Rake -- apg_abilities_class.lst:2946
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Rake",
            subject: "Rogue",
            archetype_name: "Rake",
            description: None,
            source_page: Some("p.134"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Rake],[!PREABILITY:1,CATEGORY=Archetype,TYPE=RogueTrapfinding,TYPE=RogueTrapSense]"]),
            replaces: Some(&["RogueTrapfinding", "RogueTrapSense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Rake ~ Bravado's Blade", at_level: 1, description: Some("When a rake hits an opponent and deals sneak attack damage, she can forgo 1d6 points of that damage and make a free Intimidate check to demoralize the foe. For every additional 1d6 points of sneak attack damage she forgoes, she receives a +5 circumstance bonus on this check. This ability replaces trapfinding."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Rake ~ Rake's Smile", at_level: 3, description: Some("At 3rd level, a rake gains a +1 morale bonus on Bluff and Diplomacy checks. This bonus increases by +1 for every 3 levels beyond 3rd. This ability replaces trap sense."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Scout -- apg_abilities_class.lst:2947
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Scout",
            subject: "Rogue",
            archetype_name: "Scout",
            description: Some("Not all rogues live in the city. Scouts frequently roam the wilderness, often banding together as bandits, but sometimes serving as guides, as trailblazers, or as companions to a ranger or barbarian warrior. More comfortable with sneaking and hiding outdoors, the scout is still effective in the city and the dungeon."),
            source_page: Some("p.134"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Scout],[!PREABILITY:1,CATEGORY=Archetype,TYPE=RogueUncannyDodge,TYPE=RogueImprovedUncannyDodge]"]),
            replaces: Some(&["RogueUncannyDodge", "RogueImprovedUncannyDodge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Scout ~ Scout's Charge", at_level: 4, description: Some("Whenever you make a charge, your attack deals sneak attack damage as if the target were flat-footed. Foes with uncanny dodge are immune to this ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Scout ~ Skirmisher", at_level: 8, description: Some("Whenever you move more than 10 feet in a round and make an attack action, the attack deals sneak attack damage as if the target was flat-footed. If you make more than one attack this turn, this ability only applies to the first attack."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Sniper -- apg_abilities_class.lst:2948
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Sniper",
            subject: "Rogue",
            archetype_name: "Sniper",
            description: None,
            source_page: Some("p.134"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Sniper],[!PREABILITY:1,CATEGORY=Archetype,TYPE=RogueTrapfinding,TYPE=RogueTrapSense]"]),
            replaces: Some(&["RogueTrapfinding", "RogueTrapSense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Sniper ~ Accuracy", at_level: 1, description: Some("At 1st level, a sniper halves all range increment penalties when making ranged attacks with a bow or crossbow. This ability replaces trapfinding."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sniper ~ Deadly Range", at_level: 3, description: Some("At 3rd level, a sniper increases the range at which she can apply her sneak attack damage by 10 feet. This range increases by 10 feet for every 3 levels after 3rd. This ability replaces trap sense."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Spy -- apg_abilities_class.lst:2949
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Spy",
            subject: "Rogue",
            archetype_name: "Spy",
            description: None,
            source_page: Some("p.135"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Spy],[!PREABILITY:1,CATEGORY=Archetype,TYPE=RogueTrapfinding,TYPE=RogueTrapSense]"]),
            replaces: Some(&["RogueTrapfinding", "RogueTrapSense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Spy ~ Skilled Liar", at_level: 1, description: Some("Whenever a spy uses Bluff to attempt to deceive someone, she gains a +%1 bonus on the opposed roll. This bonus does not apply to feint attempts or attempts to pass secret messages. This ability replaces trapfinding.|max(RogueLVL/2,1)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Rogue ~ Poison Use", at_level: 3, description: Some("You are trained in the use of poison and cannot accidentally poison yourself when applying poison to a blade. This ability replaces trap sense."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Swashbuckler -- apg_abilities_class.lst:2950
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Swashbuckler",
            subject: "Rogue",
            archetype_name: "Swashbuckler",
            description: Some("A paragon of mobile swordplay, the swashbuckler is a rogue who focuses almost exclusively on honing her skill at arms and perfecting daring acrobatic moves and elaborate flourishes that border on performance."),
            source_page: Some("p.135"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Swashbuckler],[!PREABILITY:1,CATEGORY=Archetype,TYPE=RogueTrapfinding,TYPE=RogueTrapSense]"]),
            replaces: Some(&["RogueTrapfinding", "RogueTrapSense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Swashbuckler ~ Martial Training", at_level: 1, description: Some("You may select one martial weapon to add to your list of weapon proficiencies. In addition, you may take the combat trick rogue talent up to two times."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Swashbuckler ~ Daring", at_level: 3, description: Some("You gain a +%1 morale bonus on Acrobatics checks and saving throws against fear.|SwashbucklerDaringBonus"), benefit: None },
            ],
        },
        // Rogue Archetype ~ Thug -- apg_abilities_class.lst:2951
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Thug",
            subject: "Rogue",
            archetype_name: "Thug",
            description: None,
            source_page: Some("p.135"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Thug],[!PREABILITY:1,CATEGORY=Archetype,TYPE=RogueTrapfinding,TYPE=RogueTrapSense]"]),
            replaces: Some(&["RogueTrapfinding", "RogueTrapSense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Thug ~ Frightening", at_level: 1, description: Some("Whenever a thug successfully uses Intimidate to demoralize a creature, the duration of the shaken condition is increased by 1 round. In addition, if the target is shaken for 4 or more rounds, the thug can instead decide to make the target frightened for 1 round. This ability replaces trapfinding."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Thug ~ Brutal Beating", at_level: 3, description: Some("At 3rd level, whenever a thug deals sneak attack damage, she can choose to forgo 1d6 points of sneak attack damage to make the target sickened for a number of rounds equal to 1/2 her rogue level. This ability does not stack with itself - only the most recent duration applies. This ability replaces trap sense."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Trapsmith -- apg_abilities_class.lst:2952
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Trapsmith",
            subject: "Rogue",
            archetype_name: "Trapsmith",
            description: None,
            source_page: Some("p.135"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Trapsmith],[!PREABILITY:1,CATEGORY=Archetype,TYPE=RogueUncannyDodge,TYPE=RogueImprovedUncannyDodge]"]),
            replaces: Some(&["RogueUncannyDodge", "RogueImprovedUncannyDodge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Rogue ~ Careful Disarm", at_level: 4, description: Some("Whenever you attempt to disarm a trap using Disable Device, you do not spring the trap unless you fail by 10 or more. If you do set off a trap you were attempting to disarm, you adds double your trap sense bonus to avoid the trap. This ability replaces uncanny dodge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Trapsmith ~ Trap Master", at_level: 8, description: Some("At 8th level, whenever a trapsmith disarms a trap using Disable Device, she can bypass it even if her check did not exceed the DC by 10 or more. If it is a magic trap that allows specific creatures to pass it without danger, she can modify which creatures it allows to pass, adding her allies and restricting enemies if she desires. This ability replaces improved uncanny dodge."), benefit: None },
            ],
        },        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_has_80_records() {
        assert_eq!(archetype_swap_tables().len(), 80);
    }

    #[test]
    fn keys_are_unique_within_book() {
        let keys: std::collections::BTreeSet<&str> =
            archetype_swap_tables().iter().map(|e| e.key).collect();
        assert_eq!(keys.len(), archetype_swap_tables().len());
    }

    /// A real, systematic exception, confirmed against the raw corpus
    /// rows directly: 9 of this book's 12 Rogue archetypes genuinely
    /// carry no `DESC:`/`BENEFIT:` on their own master row -- not one
    /// stray record, a whole-subfamily gap in this book's own PCGen
    /// conversion. Named explicitly, all 9, rather than silently
    /// loosening the assertion for every record.
    const ROGUE_MASTERS_WITHOUT_DESC: &[&str] = &[
        "Rogue Archetype ~ Burglar",
        "Rogue Archetype ~ Cutpurse",
        "Rogue Archetype ~ Investigator",
        "Rogue Archetype ~ Poisoner",
        "Rogue Archetype ~ Rake",
        "Rogue Archetype ~ Sniper",
        "Rogue Archetype ~ Spy",
        "Rogue Archetype ~ Thug",
        "Rogue Archetype ~ Trapsmith",
    ];

    #[test]
    fn every_master_record_carries_a_real_description() {
        for e in archetype_swap_tables() {
            if ROGUE_MASTERS_WITHOUT_DESC.contains(&e.key) {
                assert!(e.description.is_none(), "{} was expected to still lack DESC: -- re-check the corpus row if this now fails", e.key);
                continue;
            }
            assert!(e.description.is_some(), "{} has no DESC:", e.key);
        }
        let without_desc = archetype_swap_tables().iter().filter(|e| e.description.is_none()).count();
        assert_eq!(without_desc, ROGUE_MASTERS_WITHOUT_DESC.len(), "exactly the named Rogue exceptions, nothing else");
    }

    /// APG's own rate: 52% (42/80) -- higher than UPsi's corrected 13%
    /// and ACG's 34%, confirming the rate is book-dependent.
    #[test]
    fn the_type_and_ability_lists_genuinely_disagree() {
        let total_replaces: usize =
            archetype_swap_tables().iter().map(|e| e.replaces.map_or(0, |r| r.len())).sum();
        let total_grants: usize = archetype_swap_tables().iter().map(|e| e.grants.len()).sum();
        assert_eq!(total_replaces, 333, "total TYPE: replaced-slot count across all 80 records");
        assert_eq!(total_grants, 392, "total ABILITY: granted-feature count across all 80 records");
        assert_ne!(total_replaces, total_grants);

        let equal_count_records = archetype_swap_tables()
            .iter()
            .filter(|e| e.replaces.map_or(0, |r| r.len()) == e.grants.len())
            .count();
        assert_eq!(equal_count_records, 42, "of 80 (52%) -- APG's own rate, higher than UPsi's 13% or ACG's 34%");
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

    #[test]
    fn resolved_grant_descriptions_are_the_real_count() {
        let resolved: usize = archetype_swap_tables()
            .iter()
            .flat_map(|e| e.grants.iter())
            .filter(|g| g.description.is_some() || g.benefit.is_some())
            .count();
        assert_eq!(resolved, 364, "364 of 392 grants carry real DESC:/BENEFIT: text -- see this module's own doc comment for the 28 that did not (1 failed lookup, 25 shared unresolved names across 5 sibling Shaman-totem archetypes, 2 bare markers)");
    }
}
