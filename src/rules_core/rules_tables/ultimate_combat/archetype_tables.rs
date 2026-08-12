//! Ultimate Combat (UC) archetype-swap catalog. SD28-E30
//! (`epic-32-archetype-swap`) tier-1 table 5. See
//! `ultimate_psionics::archetype_tables`'s own module doc comment for
//! the full struct rationale, the exhaustively-enumerated `ABILITY:`
//! grant grammar and its per-family inclusion ruling (`Internal`
//! excluded, `NORMAL`-type excluded), and the `.MOD`-injected-grant
//! floor every table in this program states explicitly.
//!
//! **Agreement rate, fifth book: 22% (14/65)** -- 282 total `TYPE:`-
//! replaced slots vs 354 total `ABILITY:`-granted features. Alongside
//! UPsi 33%, ACG 33%, APG 52%, UM 27% -- a fifth distinct value,
//! confirming (per `decisions.md §51`'s own closing framing) that there
//! is no convergence to find: `TYPE:`/`ABILITY:` disagree in the
//! majority of records in every book measured, at a book-dependent rate
//! roughly between a fifth and a half.
//!
//! **294 of 354 sub-feature grants (83%) resolved to real `DESC:`/
//! `BENEFIT:` text -- the lowest resolution rate of any table so far,
//! and the shortfall clusters into a shape not seen at this scale
//! before.** 60 unresolved grants, broken down by cause: **9 `Weapon
//! and Armor Proficiency`-named grants** (e.g. `Cad ~ Weapon and Armor
//! Proficiency`, `Musketeer ~ Weapon and Armor Proficiency`) that have
//! no separately-declared row anywhere in this book's file -- plausibly
//! implemented via bare `WEAPONPROF:`/`ARMORPROF:` tokens directly on
//! another row rather than a named class-feature row, not yet
//! confirmed; **15 shared unresolved names across 3 sibling Druid
//! Shaman-totem archetypes** (`Ape`/`Bat`/`Boar Shaman`), the same
//! pattern APG's and UM's own tables already found; **2 real cross-book
//! feat references** (`Armor Proficiency (Light)`, `Improved Unarmed
//! Strike`); **3 `No Cantrips ~ Wizard` bare markers**; **2 found rows
//! with neither token**; the remaining unresolved names are individual
//! failed `KEY:` lookups, not clustered.
//!
//! **This book's own share of the 1,282-row corpus-wide `.MOD`-
//! injection population (`decisions.md §51`'s own addendum) is 147
//! rows, the second-largest of any book** (behind only ACG's 251).
//! This table's `grants` field is bounded below by that count and by
//! the tier-2 sub-feature population, not closed by either -- the floor
//! caveat matters more here than in any table landed so far except ACG.
//!
//! **The `§46`/`§48`/`§49` text-shape triad, spot-checked against this
//! book's own archetype `.MOD` rows.** Same clean shape as every prior
//! book -- pure `FACT:`-setter suppression rows, no prose, none of the
//! three hazards applied.
//!
//! **This table is data only.** No `pilot_compute.rs` integration lands
//! in this slice -- see `decisions.md §51`/`forward-scope-register.md
//! §C4.8` for why that half is blocked on an explicit scope decision.
//!
//! Every field below is copied verbatim from the real corpus row (source:
//! `~/workspace/repos/pcgen/data/pathfinder/paizo/roleplaying_game/
//! ultimate_combat/uc_abilities_class.lst`), generated programmatically
//! by a one-off extraction script, not hand-transcribed.

use super::super::archetype_swap::{ArchetypeGrant, ArchetypeSwapEntry};

/// Full UC archetype-swap catalog: 65 real, distinct master records, in
/// source order. Built once and cached for the process lifetime.
pub fn archetype_swap_tables() -> &'static [ArchetypeSwapEntry] {
    static TABLE: std::sync::OnceLock<Vec<ArchetypeSwapEntry>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        vec![
        // Alchemist Archetype ~ Beastmorph -- uc_abilities_class.lst:247
        ArchetypeSwapEntry {
            key: "Alchemist Archetype ~ Beastmorph",
            subject: "Alchemist",
            archetype_name: "Beastmorph",
            description: Some("Beastmorphs study the anatomy of monsters, learning how they achieve their strange powers. They use their knowledge to duplicate these abilities, but at the cost of taking on inhuman shapes when they use mutagens."),
            source_page: Some("p.24"),
            prerequisites: Some(&["PRECLASS:1,Alchemist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Alchemist Archetype ~ BEASTMORPH],[!PREABILITY:1,CATEGORY=Archetype,TYPE.AlchemistSwiftAlchemy,TYPE.AlchemistPoisonResistance,TYPE.AlchemistPoisonResistance2,TYPE.AlchemistPoisonResistance4,TYPE.AlchemistPoisonResistance6,TYPE.AlchemistPoisonImmunity,TYPE.AlchemistPersistentMutagen]"]),
            replaces: Some(&["AlchemistSwiftAlchemy", "AlchemistPoisonResistance", "AlchemistPoisonResistance2", "AlchemistPoisonResistance4", "AlchemistPoisonResistance6", "AlchemistPoisonImmunity", "AlchemistPersistentMutagen"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Beastmorph ~ Beastform Mutagen", at_level: 3, description: Some("At 3rd level, a beastmorph's mutagen causes him to take on animalistic features-whether those of an animal, a magical beast, an animal-like humanoid (such as a lizardfolk), or a monstrous humanoid. For example, when the beastmorph uses his mutagen, he may gain a furry muzzle and pointed ears like a werewolf, scaly skin like a lizardfolk or sahuagin, or compound eyes and mandibles like a giant insect. The beastmorph also gains his choice of one of the abilities listed in the alter self spell, which persists as long as the mutagen. He may select a different ability each time he creates a mutagen. This ability replaces swift alchemy."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Beastmorph ~ Improved Beastform Mutagen", at_level: 6, description: Some("At 6th level, a beastmorph's mutagen grants him additional abilities and options. The alchemist gains his choice of two of the abilities listed in the beast shape I spell, which persist as long as the mutagen. He may select two different abilities each time he creates a mutagen. This ability replaces swift poisoning."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Beastmorph ~ Greater Beastform Mutagen", at_level: 10, description: Some("At 10th level, a beastmorph's mutagen grants him three of the abilities listed in the beast shape II spell, which persist as long as the mutagen. He may select three different abilities each time he creates a mutagen. This ability replaces poison resistance +2, +4, and +6, as well as poison immunity."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Beastmorph ~ Grand Beastform Mutagen", at_level: 14, description: Some("At 14th level, a beastmorph's mutagen grants him four of the abilities listed in the beast shape III spell, which persist as long as the mutagen. He may select four different abilities each time he creates a mutagen. This ability replaces persistent mutagen."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Beastmorph ~ Discoveries", at_level: 1, description: Some("The following discoveries complement the beastmorph archetype - feral mutagen, grand mutagen, greater mutagen, infuse mutagen, mummification**, spontaneous healing**, tentacle**, and wings**."), benefit: None },
            ],
        },
        // Alchemist Archetype ~ Ragechemist -- uc_abilities_class.lst:248
        ArchetypeSwapEntry {
            key: "Alchemist Archetype ~ Ragechemist",
            subject: "Alchemist",
            archetype_name: "Ragechemist",
            description: Some("Some alchemists create mutagens that tap into a primal anger that fuels their physical transformation. These alchemists have little control over their altered selves."),
            source_page: Some("p.25"),
            prerequisites: Some(&["PRECLASS:1,Alchemist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Alchemist Archetype ~ RAGECHEMIST],[!PREABILITY:1,CATEGORY=Archetype,TYPE.AlchemistPoisonUse,TYPE.AlchemistSwiftPoisoning,TYPE.AlchemistPoisonImmunity]"]),
            replaces: Some(&["AlchemistPoisonUse", "AlchemistSwiftPoisoning", "AlchemistPoisonImmunity"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Ragechemist ~ Rage Mutagen", at_level: 2, description: Some("At 2nd level, whenever a ragechemist creates a mutagen that improves his Strength, that mutagen's bonus to Strength increases by 2 and penalized the alchemist's Intelligence score. Furthermore, while under the effects of this mutagen, whenever the alchemist takes damage, his rage grows, with detrimental effects. At the end of each turn that he takes hit point damage, the ragechemist must succeed at a Will saving throw (DC 15, or DC 20 if any of the damage came from a critical hit that turn) or take a -2 penalty on Will saving throws and to Intelligence. These penalties end 1 hour after the mutagen ends and stack with themselves. If the penalty lowers the ragechemist's Intelligence score to 0, the ragechemist is comatose until 1 hour after the mutagen expires. This ability replaces poison use."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ragechemist ~ Sturdy Rage", at_level: 6, description: Some("At 6th level, whenever a ragechemist uses his rage mutagen, he also gains a +4 bonus to natural armor, but the penalty on Will saving throws and to Intelligence for taking damage increases to -4. This ability replaces swift poisoning."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ragechemist ~ Lumbering Rage", at_level: 10, description: Some("At 10th level, whenever a ragechemist uses his rage mutagen ability, he may have the mutagen also give him a +2 morale bonus to Constitution, but when he takes a penalty on Will saving throws and to Intelligence, he also takes a -1 penalty to Dexterity. This ability replaces poison immunity."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ragechemist ~ Discoveries", at_level: 1, description: Some("The following discoveries complement the ragechemist - feral mutagen, grand mutagen, greater mutagen, infuse mutagen, tentacle**, and vestigial arm**."), benefit: None },
            ],
        },
        // Barbarian Archetype ~ Armored Hulk -- uc_abilities_class.lst:303
        ArchetypeSwapEntry {
            key: "Barbarian Archetype ~ Armored Hulk",
            subject: "Barbarian",
            archetype_name: "Armored Hulk",
            description: Some("Some barbarians disdain the hides and leather used as armor by most of their kin. Instead they master the heaviest of armors, even those created by more civilized people, to gain greater protection and stability in battle."),
            source_page: Some("p.28"),
            prerequisites: Some(&["PRECLASS:1,Barbarian=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Barbarian Archetype ~ Armored Hulk],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BarbarianFastMovement.BarbarianUncannyDodge.BarbarianTrapSense.BarbarianImprovedUncannyDodge]"]),
            replaces: Some(&["BarbarianFastMovement", "BarbarianUncannyDodge", "BarbarianTrapSense", "BarbarianImprovedUncannyDodge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Armored Hulk ~ Weapon and Armor Proficiency", at_level: 1, description: Some("An armored hulk gains proficiency in heavy armor."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Armored Hulk ~ Indomitable Stance", at_level: 1, description: Some("An armored hulk gains a +1 bonus on combat maneuver checks and to CMD for overrun combat maneuvers, and on Reflex saves against trample attacks. She also gains a +1 bonus to her AC against charge attacks and on attack and damage rolls against charging creatures. This ability replaces fast movement."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Armored Hulk ~ Armored Swiftness", at_level: 2, description: Some("At 2nd level, an armored hulk moves faster in medium and heavy armor. When wearing medium or heavy armor, an armored hulk can move 5 feet faster than normal, to a maximum of her speed. This ability replaces uncanny dodge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Armored Hulk ~ Resilience of Steel", at_level: 3, description: Some("At 3rd level, an armored hulk is able to use her armor to help avoid deadly hits. While wearing heavy armor, she gains a +1 bonus to AC that applies only on critical hit confirmation rolls. This bonus increases by +1 every 3 levels beyond 3rd (maximum +6 at 18th level). This ability replaces trap sense."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Armored Hulk ~ Improved Armored Swiftness", at_level: 5, description: Some("At 5th level, an armored hulk's land speed is faster than the norm for her race by +10 feet. This benefit applies when she is wearing any armor, including heavy armor, but not while carrying a heavy load. Apply this bonus before modifying the armored hulk's speed because of any load carried or armor worn. This bonus stacks with any other bonuses to the barbarian's land speed. This ability replaces improved uncanny dodge(Encumbered=%1)(WearingArmor=%2)(MoveBonus=%3).|EncumberedHeavy|WearingArmor|IASMoveBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Armored Hulk ~ Rage Powers", at_level: 1, description: Some("The following rage powers complement the armored hulk archetype - boasting taunt**, greater guarded life*, guarded life**, guarded stance, increased damage reduction, no escape, overbearing advance**, overbearing onslaught**, reflexive dodge*, rolling dodge, and unexpected strike."), benefit: None },
            ],
        },
        // Barbarian Archetype ~ Scarred Rager -- uc_abilities_class.lst:304
        ArchetypeSwapEntry {
            key: "Barbarian Archetype ~ Scarred Rager",
            subject: "Barbarian",
            archetype_name: "Scarred Rager",
            description: Some("Some barbarians wear marks of prowess and savagery upon their bodies. The scarred rager believes each wound tells the tales of her prowess and bravery. She augments these ragged trophies with brands and tattoos in order to win the notice and favor of her gods, ancestors, or totem spirits. The scarred rager's gnarled and exotic appearance terrifies civilized onlookers, but is awe-inspiring to her savage kin."),
            source_page: Some("p.29"),
            prerequisites: Some(&["PRECLASS:1,Barbarian=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Barbarian Archetype ~ Scarred Rager],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BarbarianFastMovement,TYPE.BarbarianUncannyDodge,TYPE.BarbarianTrapSense]"]),
            replaces: Some(&["BarbarianFastMovement", "BarbarianUncannyDodge", "BarbarianTrapSense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Scarred Rager ~ Terrifying Visage", at_level: 1, description: Some("A scarred rager adds 1/2 her barbarian level on Intimidate checks against humanoids who are not members of barbarian tribes (bonus currently +%1). When dealing with barbarians, the scarred rager may choose to add this bonus on Diplomacy checks instead. The DC of any fear effect created by the scarred rager also increases by 1. This ability replaces fast movement.|classlevel(\"Barbarian\")/2"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Scarred Rager ~ Tolerance", at_level: 2, description: Some("At 2nd level, a scarred rager who fails a save against an effect that causes her to become nauseated, sickened, fatigued, or exhausted can make a second save to negate the effect on the start of her next turn. Only one additional save is allowed. If the effect does not allow a saving throw, its duration is halved instead (minimum of 1 round). This ability replaces uncanny dodge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Scarred Rager ~ Scarification", at_level: 3, description: Some("At 3rd level, a scarred rager can ignore 1 point of bleed damage per round (currently %1 points). This amount increases by 1 every three levels beyond 3rd. At 15th level, a scarred rager can ignore 1 bleed effect each round. This ability replaces trap sense.|ScarredScarBleedIgnore"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Scarred Rager ~ Improved Tolerance", at_level: 5, description: Some("At 5th level, a scarred rager's tolerance ability also applies to effects that would cause her to become dazed, frightened, shaken, or stunned. This replaces improved uncanny dodge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Scarred Rager ~ Rage Powers", at_level: 1, description: Some("The following rage powers complement the scarred rager archetype - auspicious mark*, body bludgeon*, come and get me**, increased damage reduction, inspire ferocity**, internal fortitude, intimidating glare, reckless abandon**, regenerative vigor*, renewed life*, renewed vigor, renewed vitality*, roused anger, and terrifying howl."), benefit: None },
            ],
        },
        // Barbarian Archetype ~ Sea Reaver -- uc_abilities_class.lst:305
        ArchetypeSwapEntry {
            key: "Barbarian Archetype ~ Sea Reaver",
            subject: "Barbarian",
            archetype_name: "Sea Reaver",
            description: Some("Not all barbarians hunt forests, plains, and mountains. Some are raiding terrors on the sea and coasts, pillaging those who hoard treasure and pursuing monsters of the deep. Some sea reavers are no more than hunters of the open sea, while others are raiders striking fear into coastal settlements within reach of the sea reavers' longships."),
            source_page: Some("p.29"),
            prerequisites: Some(&["PRECLASS:1,Barbarian=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Barbarian Archetype ~ Sea Reaver],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BarbarianMediumArmorProficiency,TYPE.BarbarianFastMovement,TYPE.BarbarianUncannyDodge,TYPE.BarbarianTrapSense,TYPE.BarbarianImprovedUncannyDodge]"]),
            replaces: Some(&["BarbarianFastMovement", "BarbarianUncannyDodge", "BarbarianTrapSense", "BarbarianImprovedUncannyDodge", "BarbarianMediumArmorProficiency"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Sea Reaver ~ Weapon and Armor Proficiency", at_level: 1, description: Some("A sea reaver is not proficient with medium armor."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sea Reaver ~ Marine Terror", at_level: 1, description: Some("A sea reaver can hold her breath for a number of rounds equal to four times her Constitution score (currently %1 rounds). In addition, a sea reaver can move normally though squares of standing water or bog that is 1 foot deep. It does not cost her extra movement to traverse these terrains. Lastly, a sea reaver ignores the normal cover bonus to AC when attacking creatures that are partially immersed in water. This ability replaces fast movement.|CONSCORE*4"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sea Reaver ~ Eyes of the Storm", at_level: 2, description: Some("At 2nd level, a sea reaver ignores any concealment provided by fog, rain, sleet, mist, wind, or other weather effects that is less than total concealment, and any penalties weather applies on Perception checks are halved. This ability replaces uncanny dodge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sea Reaver ~ Savage Sailor", at_level: 3, description: Some("At 3rd level, a sea reaver gains a +1 bonus on Acrobatics, Climb, Profession (sailor), Survival, and Swim checks made in aquatic terrain, including aboard a ship or along shorelines (currently +%1). These bonuses improve by +1 every three levels after 3rd. This ability replaces trap sense.|ReaverSavageBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sea Reaver ~ Sure-Footed", at_level: 5, description: Some("At 5th level, a sea reaver takes no penalties when moving across slick surfaces, whether natural or magical (e.g., grease, ice storm, and sleet storm). She is not at risk of falling, is not denied her Dexterity bonus when moving across such areas, and does not treat them as difficult terrain. This ability replaces improved uncanny dodge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sea Reaver ~ Rage Powers", at_level: 1, description: Some("The following rage powers complement the sea reaver archetype - bestial leaper, bestial swimmer, come and get me**, hurling charge**, raging leaper*, raging swimmer*, rolling dodge*, and smasher**."), benefit: None },
            ],
        },
        // Barbarian Archetype ~ Titan Mauler -- uc_abilities_class.lst:306
        ArchetypeSwapEntry {
            key: "Barbarian Archetype ~ Titan Mauler",
            subject: "Barbarian",
            archetype_name: "Titan Mauler",
            description: Some("In lands overrun by giants, dragons, and other hulking beasts, entire fellowships of barbarians hone tactics and traditions with one purpose-to bring low these massive foes. While her enemies' size makes the creatures strong, the titan mauler is even stronger, taking up weapons from her fallen foes that no lesser warrior can lift, and using them when she beseeches the spirits to grant her increased size and greater ferocity against her titanic foes."),
            source_page: Some("p.30"),
            prerequisites: Some(&["PRECLASS:1,Barbarian=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Barbarian Archetype ~ Titan Mauler],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BarbarianFastMovement,TYPE.BarbarianUncannyDodge,TYPE.BarbarianTrapSense,TYPE.BarbarianImprovedUncannyDodge,TYPE.BarbarianIndomitableWill]"]),
            replaces: Some(&["BarbarianFastMovement", "BarbarianUncannyDodge", "BarbarianTrapSense", "BarbarianImprovedUncannyDodge", "BarbarianIndomitableWill"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Titan Mauler ~ Big Game Hunter", at_level: 1, description: Some("A titan mauler gains a +1 bonus on attack rolls and a +1 dodge bonus to AC in melee with creatures larger than themselves. This ability replaces fast movement."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Titan Mauler ~ Jotungrip", at_level: 2, description: Some("At 2nd level, a titan mauler may choose to wield a two-handed melee weapon in one hand with a -2 penalty on attack rolls while doing so. The weapon must be appropriately sized for her, and it is treated as onehanded when determining the effect of Power Attack, Strength bonus to damage, and the like. This ability replaces uncanny dodge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Titan Mauler ~ Massive Weapons", at_level: 3, description: Some("At 3rd level, a titan mauler becomes skilled in the use of massive weapons looted from her titanic foes. She can use two-handed weapons meant for creatures one size category larger, but the penalty for doing so is increased by 4. However, the attack roll penalty for using weapons too large for her size is reduced by 1, and this reduction increases by 1 for every three levels beyond 3rd (to a minimum of 0). This ability replaces trap sense. [Current Oversize Weapon To Hit %1]|OversizeWeaponToHitBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Titan Mauler ~ Evade Reach", at_level: 5, description: Some("At 5th level, as a swift action, a titan mauler may choose one creature within her line of sight. Until the end of her turn, that target's reach is treated as if it were 5 feet shorter with respect to reaching the titan mauler, and this reduction increases by 5 feet for every five levels beyond 5th (currently %1 feet). This ability replaces improved uncanny dodge.|MaulerEvadeReach"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Titan Mauler ~ Titanic Rage", at_level: 14, description: Some("At 14th level, a titan mauler may choose to gain the benefits of enlarge person when she enters a rage. While using titanic rage, she must spend 2 rounds of rage per round, and she becomes exhausted rather than fatigued when the rage ends. This ability replaces indomitable will."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Titan Mauler ~ Rage Powers", at_level: 1, description: Some("The following rage powers complement the titan mauler archetype - body bludgeon*, greater ground breaker*, ground breaker**, knockback, mighty swing, powerful blow, smasher**, and strength surge."), benefit: None },
            ],
        },
        // Barbarian Archetype ~ True Primitive -- uc_abilities_class.lst:307
        ArchetypeSwapEntry {
            key: "Barbarian Archetype ~ True Primitive",
            subject: "Barbarian",
            archetype_name: "True Primitive",
            description: Some("Isolated and xenophobic tribes that dwell in areas untouched by civilization often see anything from cities and organized settlements as strange, dangerous, and decadent. They gain power from their truly primitive nature; their bodies and spirits are hardened by the wild and untainted existence far from the pathetic softness of so-called civilization. Even when forced to mingle with civilization, the true primitive stays apart in both traditions and trappings."),
            source_page: Some("p.30"),
            prerequisites: Some(&["PRECLASS:1,Barbarian=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Barbarian Archetype ~ True Primitive],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BarbarianFastMovement,TYPE.BarbarianTrapSense]"]),
            replaces: Some(&["BarbarianFastMovement", "BarbarianTrapSense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "True Primitive ~ Weapon and Armor Proficiency", at_level: 1, description: Some("A true primitive is proficient with hide armor and armors made from bone (see page 146). A true primitive is also proficient with bone shields and the following weapons - battleaxe, blowgun, club, greatclub, handaxe, longspear, shortspear, sling, and spear."), benefit: None },
                ArchetypeGrant { grants_feature_key: "True Primitive ~ Illiteracy", at_level: 1, description: Some("A true primitive cannot read or write, and her superstition about such things leads her to refuse to ever learn to read or write, even if she multiclasses into other classes."), benefit: None },
                ArchetypeGrant { grants_feature_key: "True Primitive ~ Favored Terrain", at_level: 1, description: Some("A true primitive has a favored terrain representing her native homeland. This ability functions as the ranger class feature, and the true primitive's bonuses in that terrain improve by +2 at 5th level and every five levels thereafter. However, she does not gain any additional favored terrains. This ability replaces fast movement."), benefit: None },
                ArchetypeGrant { grants_feature_key: "True Primitive ~ Trophy Fetish", at_level: 3, description: Some("A true primitive collects teeth, bones, hair, and other trophies from vanquished enemies, representing their power and strength. At 3rd level, a true primitive can attach a trophy fetish to one of the traditional true primitive weapons listed above. When wielding that weapon, the true primitive gains a +1 morale bonus on damage rolls. Furthermore, if that weapon has the fragile property, once per day, the true primitive can ignore the effects of a single natural 1 roll. A trophy fetish can also be attached to a suit of hide or bone armor, granting the true primitive a +1 morale bonus on saving throws. Furthermore, if that armor has the fragile property, once per day, the true primitive can chose not to have the armor break on the confirmation of a critical hit made against her. Every five levels beyond 3rd, a true primitive can use an additional trophy fetish. Multiple trophy fetishes can be attached to the same armor or weapon; their effects stack. A trophy fetish can be sundered (hardness 5, 1 hit point) but is not damaged by area attacks or attacks that do not specifically target it. It has no effect if attached to other types of weapons or armor. This ability replaces trap sense."), benefit: None },
                ArchetypeGrant { grants_feature_key: "True Primitive ~ Rage Powers", at_level: 1, description: Some("The following rage powers complement the true primitive archetype - animal fury, eater of magic*, ghost rager*, low-light vision, night vision, primal scent*, scent, spell sunder*, sunder enchantment*, superstition, and witch hunter**."), benefit: None },
            ],
        },
        // Barbarian Archetype ~ Urban Barbarian -- uc_abilities_class.lst:308
        ArchetypeSwapEntry {
            key: "Barbarian Archetype ~ Urban Barbarian",
            subject: "Barbarian",
            archetype_name: "Urban Barbarian",
            description: Some("Every barbarian knows that city life can soften the spirit and the body, but some barbarians take on the trappings and ways of their adoptive homes and bend their savage powers to its challenges. While these urban barbarians' rough edges are smoothed into civility, they can use their primal nature and upbringing to move with the ebb and flow of civilization's natural rhythms."),
            source_page: Some("p.31"),
            prerequisites: Some(&["PRECLASS:1,Barbarian=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Barbarian Archetype ~ Urban Barbarian],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BarbarianClassSkills,TYPE.BarbarianFastMovement,TYPE.BarbarianMediumArmorProficiency]"]),
            replaces: Some(&["BarbarianFastMovement", "BarbarianMediumArmorProficiency", "BarbarianClassSkills"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Urban Barbarian ~ Weapon and Armor Proficiency", at_level: 1, description: Some("An urban barbarian is not proficient with medium armor."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Urban Barbarian ~ Skills", at_level: 1, description: Some("An urban barbarian does not gain Handle Animal (Cha), Knowledge (nature) (Int), or Survival (Wis) as class skills; instead, she gains Diplomacy (Cha), Knowledge (local) (Int), Knowledge (nobility) (Int), Linguistics (Int), and Profession (Wis) as class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Urban Barbarian ~ Crowd Control", at_level: 1, description: Some("At 1st level, an urban barbarian gains a +1 bonus on attack rolls and a +1 dodge bonus to AC when adjacent to two or more enemies. In addition, her movement is not impeded by crowds, and she gains a bonus equal to 1/2 her barbarian level on Intimidate checks to influence crowds (Core Rulebook 436). This ability replaces fast movement"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Urban Barbarian ~ Controlled Rage", at_level: 1, description: Some("When an urban barbarian rages, instead of making a normal rage she applies a +4 morale bonus to her Strength, Dexterity, or Constitution. This bonus increases to +6 when she gains greater rage and +8 when she gains mighty rage. She may apply the full bonus to one ability score or may split the bonus between several scores in increments of +2. When using a controlled rage, an urban barbarian gains no bonus on Will saves, takes no penalties to AC, and can still use Intelligence-, Dexterity-, and Charisma-based skills. This ability otherwise follows the normal rules for rage. This ability alters rage."), benefit: None },
            ],
        },
        // Barbarian Archetype ~ Wild Rager -- uc_abilities_class.lst:309
        ArchetypeSwapEntry {
            key: "Barbarian Archetype ~ Wild Rager",
            subject: "Barbarian",
            archetype_name: "Wild Rager",
            description: Some("Rages are barely controlled, but there are those who wholly give in to their more savage side, letting their rages take them to a confusing and uncontrolled place of terrible savagery. These barbarians become beasts, consumed with absolute bloodlust to the point where they cannot tell friend from foe."),
            source_page: Some("p.31"),
            prerequisites: Some(&["PRECLASS:1,Barbarian=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Barbarian Archetype ~ Wild Rager],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BarbarianUncannyDodge,TYPE.BarbarianImprovedUncannyDodge]"]),
            replaces: Some(&["BarbarianUncannyDodge", "BarbarianImprovedUncannyDodge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Wild Rager ~ Uncontrolled Rage", at_level: 1, description: Some("A wild rager's rage functions as normal, except that when she reduces a creature to 0 or fewer hit points, she must attempt a Will save (DC 10 + 1/2 the barbarian's level + the barbarian's Constitution modifier) or become confused. For the remainder of her current turn, she attacks the nearest creature other than herself. On the following round, refer to the confusion spell (Core Rulebook 258) to determine her actions. At the end of this round, and each round thereafter, she can attempt a new saving throw to end the confusion effect. The rounds during which she is confused do not count against the rounds she has spent raging that day, but she cannot end her rage voluntarily, nor can she use rage powers while confused."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wild Rager ~ Wild Fighting", at_level: 2, description: Some("At 2nd level, even when not raging, wild ragers often fight with reckless, savage abandon. A wild rager using the full-attack action can make one extra attack per round at her highest base attack bonus. Until the beginning of her next turn, however, she takes a -2 penalty on attack rolls and -4 penalty to AC. This ability replaces uncanny dodge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wild Rager ~ Rage Conversion", at_level: 5, description: Some("At 5th level, a wild rager who fails a saving throw against any mind-affecting effect can attempt a new saving throw at the beginning of her next turn. If the save succeeds, that effect ends and she instead rages and becomes confused as noted above. This ability replaces improved uncanny dodge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wild Rager ~ Rage Powers", at_level: 1, description: Some("The following rage powers complement the wild rager archetype - animal fury, bloody blow*, body bludgeon*, brawler**, crippling blow*, greater brawler**, intimidating glare, mighty swing*, no escape, powerful blow, and quick reflexes."), benefit: None },
            ],
        },
        // Bard Archetype ~ Archaeologist -- uc_abilities_class.lst:383
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Archaeologist",
            subject: "Bard",
            archetype_name: "Archaeologist",
            description: Some("No stodgy researcher, this Archaeologist meets his research head-on in the field. Archaeologists sacrifice the bard's inspirational performance for a smattering of rogue talents. This archetype also fits roguelike characters that focus more on learning than on stabbing foes in the back."),
            source_page: Some("p.32"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Archaeologist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardBardicPerformance,TYPE.BardVersatilePerformance,TYPE.BardWellVersed]"]),
            replaces: Some(&["BardBardicPerformance", "BardVersatilePerformance", "BardWellVersed"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Archaeologist ~ Bardic Performance", at_level: 1, description: Some("Archaeologists do not gain the bardic performance ability or any of its performance types."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Archaeologist ~ Archaeologist's Luck", at_level: 1, description: Some("Fortune favors the Archaeologist. As a swift action, an Archaeologist can call on fortune's favor, giving him a +1 luck bonus on attack rolls, saving throws, skill checks, and weapon damage rolls. He can use this ability for a number of rounds per day equal to 4 + his Charisma modifier. Maintaining this bonus is a free action, but it ends immediately if the Archaeologist is killed, paralyzed, stunned, knocked unconscious, or otherwise prevented from taking a free action to maintain it each round. Archaeologist's luck is treated as bardic performance for the purposes of feats, abilities, effects, and the like that affect bardic performance. Like bardic performance, it cannot be maintained at the same time as other performance abilities. This bonus increases to +2 at 5th level, +3 at 11th level, and +4 at 17th level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Archaeologist ~ Clever Explorer", at_level: 2, description: Some("At 2nd level, an Archaeologist gains a bonus equal to half his class level on Disable Device and Perception checks. He can disable intricate and complex devices in half the normal amount of time (minimum 1 round) and open a lock as a standard action. At 6th level, an Archaeologist can take 10 on Disable Device checks, even if distracted or endangered, and can disarm magical traps. This ability replaces the versatile performance ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Archaeologist ~ Uncanny Dodge", at_level: 2, description: Some("At 2nd level, an Archaeologist gains uncanny dodge, as the rogue class feature of the same name. This ability replaces well-versed."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Archaeologist ~ Trap Sense", at_level: 3, description: Some("At 3rd level, an Archaeologist gains trap sense +1, as the rogue class feature of the same name. This bonus improves by +1 for every three levels gained after 3rd, to a maximum of +6 at 18th level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Archaeologist ~ Rogue Talents", at_level: 4, description: Some("At 4th level, an Archaeologist gains a rogue talent. He gains an additional rogue talent for every four levels of Archaeologist gained after 4th level. Otherwise, this works as the rogue's rogue talent ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Archaeologist ~ Evasion", at_level: 6, description: Some("At 6th level, an Archaeologist gains evasion, as the rogue ability of the same name."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Archaeologist ~ Advanced Talent", at_level: 12, description: Some("At 12th level, and every four levels thereafter, an Archaeologist can choose an advanced rogue talent in place of a rogue talent."), benefit: None },
            ],
        },
        // Bard Archetype ~ Daredevil -- uc_abilities_class.lst:384
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Daredevil",
            subject: "Bard",
            archetype_name: "Daredevil",
            description: Some("As quick at wordplay as at swordplay, daredevils are dashing heroes, inspiring their allies to match their clever repartee and acrobatic feats. Daredevils often lean toward comedy, dance, oratory, and singing."),
            source_page: Some("p.32"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Daredevil],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardBardicKnowledge,TYPE.BardInspireCourage,TYPE.BardWellVersed,TYPE.BardLoreMaster]"]),
            replaces: Some(&["BardBardicKnowledge", "BardInspireCourage", "BardVersatilePerformance", "BardWellVersed", "BardLoreMaster"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Daredevil ~ Agile", at_level: 1, description: Some("A daredevil adds half her class level (minimum 1) on Acrobatics, Bluff, Climb, and Escape Artist checks. This ability replaces bardic knowledge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Daredevil ~ Bardic Performance", at_level: 1, description: Some("A daredevil gains the following type of bardic performance."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Daredevil ~ Derring-do", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Daredevil ~ Canny Foe", at_level: 2, description: Some("At 2nd level, a daredevil can choose one type of combat maneuver. She gains a +2 bonus on her combat maneuver checks to attempt the chosen maneuver and to her CMD to resist that maneuver. At 6th level, and every four levels thereafter, the daredevil gains a +2 bonus for an additional type of combat maneuver. She may not choose the same maneuver twice. This ability replaces versatile performance."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Daredevil ~ Dauntless", at_level: 2, description: Some("At 2nd level, a daredevil gains a +1 morale bonus on saving throws against mind-affecting effects, including all fear effects. This bonus increases by +1 for every four levels gained after 2nd level, to a maximum of +5 at 18th level. This ability replaces well-versed."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Daredevil ~ Scoundrel's Fortune", at_level: 5, description: Some("At 5th level, once per day a daredevil can choose to roll two dice instead of one for any skill check, keeping the best result. She can use this ability one additional time per day for every three levels she possesses beyond 5th, to a maximum of eight times per day at 20th level. This ability replaces lore master."), benefit: None },
            ],
        },
        // Bard Archetype ~ Dervish Dancer -- uc_abilities_class.lst:385
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Dervish Dancer",
            subject: "Bard",
            archetype_name: "Dervish Dancer",
            description: Some("Not all bards inspire others with their performances. Dervish dancers enter a near-mystical trance that allows them to push their bodies beyond normal limits."),
            source_page: Some("p.32"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Dervish Dancer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardWeaponProficiencies,TYPE.BardSuggestion,TYPE.BardMassSuggestion,TYPE.BardDirgeOfDoom,TYPE.BardFrighteningPerformance,TYPE.BardBardicKnowledge,TYPE.BardLoreMaster,TYPE.BardVersatilePerformance,TYPE.BardSoothingPerformance,TYPE.BardDeadlyPerformance]"]),
            replaces: Some(&["BardSuggestion", "BardMassSuggestion", "BardDirgeOfDoom", "BardFrighteningPerformance", "BardBardicKnowledge", "BardLoreMaster", "BardVersatilePerformance", "BardSoothingPerformance", "BardDeadlyPerformance", "BardWeaponProficiencies"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Dervish Dancer ~ Weapon and Armor Proficiency", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Dervish Dancer ~ Battle Dance", at_level: 1, description: Some("A dervish dancer is trained in the use of the Perform skill, especially dance, to create magical effects on himself. This works like bardic performance, except the dervish dancer only affects himself, and does not need to be able to see or hear his own performance. Battle dancing is treated as bardic performance for the purposes of feats, abilities, effects, and the like that affect bardic performance, except that battle dancing does not benefit from the Lingering Performance feat or any other ability that allows a bardic performance to grant bonuses after it has ended. Battle dancing benefits apply only when the bard is wearing light or no armor. Like bardic performance, it cannot be maintained at the same time as other performance abilities. Starting a battle dance is a move action, but it can be maintained each round as a free action. Changing a battle dance from one effect to another requires the dervish dancer to stop the previous performance and start the new one as a move action. Like a bard, a dervish dancer's performance ends immediately if he is killed, paralyzed, stunned, knocked unconscious, or otherwise prevented from taking a free action each round. A dervish dancer cannot perform more than one battle dance at a time. At 10th level, a dervish dancer can start a battle dance as a swift action instead of a move action. Dervish dancers gain the inspire courage, inspire greatness, and inspire heroics bardic performance types as battle dances, but these only provide benefit to the dervish dancer himself."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dervish Dancer ~ Rain of Blows", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Dervish Dancer ~ Razor's Kiss", at_level: 8, description: Some("At 8th level, a dervish dancer can use his battle dance to improve his weapons' critical range. All attacks he makes with manufactured weapons are treated as though he had the Improved Critical feat. Natural weapons and spells are not affected. This ability replaces dirge of doom."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dervish Dancer ~ Leaf on the Wind", at_level: 14, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Dervish Dancer ~ Fleet", at_level: 1, description: Some("While performing a battle dance, a dervish dancer gains a +10 enhancement bonus to his land speed. This bonus increases by 5 feet for every four bard levels gained after 1st level, to a maximum of +30 feet at 19th level. This ability replaces bardic knowledge and lore master."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dervish Dancer ~ Versatile Dance", at_level: 2, description: Some("At 2nd level, a dervish dancer gains a bonus equal to half his level on Perform (dance) checks. He can use his bonus for his Perform (dance) skill in place of his bonus for Acrobatics. This ability replaces versatile performance."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dervish Dancer ~ Dance of Fury", at_level: 12, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Dervish Dancer ~ Battle Fury", at_level: 20, description: Some("At 20th level, the dervish dancer can unleash a whirlwind of blows while performing a battle dance. As a full-round action, he can take a single move action and unleash a single attack at his highest bonus against each target within his reach during any point of his move, up to a maximum number of attacks equal to the dervish dancer's character level. This movement provokes attacks of opportunity as normal, and replaces deadly performance."), benefit: None },
            ],
        },
        // Cavalier Archetype ~ Beast Rider -- uc_abilities_class.lst:468
        ArchetypeSwapEntry {
            key: "Cavalier Archetype ~ Beast Rider",
            subject: "Cavalier",
            archetype_name: "Beast Rider",
            description: Some("The cavalier is defined not only by his dedication to his order or his skill on the battlefield, but also by the special relationship he maintains with his mount. Where some cavaliers are simply skilled with horses or well-trained knights, the beast rider spends his life in constant pursuit of the most perfect mount, forming bonds with greater, more powerful, and more exotic creatures."),
            source_page: Some("p.36"),
            prerequisites: Some(&["PRECLASS:1,Cavalier=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Cavalier Archetype ~ Beast Rider],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CavalierHeavyArmorProficiency,TYPE.CavalierMount,TYPE.CavalierExpertTrainer]"]),
            replaces: Some(&["CavalierHeavyArmorProficiency", "CavalierMount", "CavalierExpertTrainer"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Beast Rider ~ Armor Proficiency", at_level: 1, description: Some("A beast rider is proficient with light and medium armor, and with shields (with the exception of tower shields)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Beast Rider ~ Exotic Mount", at_level: 1, description: Some("At 1st level, a beast rider forms a bond with a strong, loyal companion that permits him to ride it as a mount. This mount functions as a druid's animal companion, using the beast rider's level as his effective druid level. The animal chosen as a mount must be large enough to carry the beast rider (Medium or Large for a Small character; Large or Huge for a Medium character). The beast rider does not take an armor check penalty on Ride checks while riding his mount. The mount is always considered combat trained, and begins play with Endurance as a bonus feat. A beast rider's mount does not gain the share spells special ability. Each time the beast rider increases in level, he can choose to select a new, more impressive mount better suited to his increased power. Small-sized beast riders can choose a pony or wolf mount at 1st level. At 4th level, a Small beast rider can also choose an allosaurus, ankylosaurus, arsinoitherium, aurochs, bison, boar, brachiosaurus, elephant, glyptodon, hippopotamus, mastodon, megaloceros, riding dog, snapping turtle (giant), triceratops, or tyrannosaurus. At 7th level, he can also choose a dinosaur (deinonychus or velociraptor). Medium beast riders can choose a camel or horse mount at 1st level. At 4th level, a Medium beast rider can also choose an allosaurus, ankylosaurus, arsinoitherium, aurochs, bison, brachiosaurus, elephant, glyptodon, hippopotamus, lion, mastodon, megaloceros, snapping turtle (giant), tiger, triceratops, or tyrannosaurus as his mount. Additional mounts might be available with GM approval. In addition, a 7th-level or higher Medium beast rider can select any creature whose natural size is Large or Huge, provided that creature is normally available as a Medium-sized animal companion at 7th level (like a bear). To generate statistics for such a mount, apply the following modifications - Size Large; Ability Scores Str +2, Dex -2, Con +2. Increase the damage of each of the mount's natural attacks by one die size. A beast rider cannot choose a mount that is not capable of bearing his weight, that has fewer than four legs, or that has a fly speed (although the GM may allow mounts with a swim speed in certain environments). Anytime a feat or ability allows a mount to make a hoof attack, it can make a claw, slam, or other analogous attack instead. This ability replaces the standard cavalier's mount and expert trainer abilities."), benefit: None },
            ],
        },
        // Cavalier Archetype ~ Emissary -- uc_abilities_class.lst:469
        ArchetypeSwapEntry {
            key: "Cavalier Archetype ~ Emissary",
            subject: "Cavalier",
            archetype_name: "Emissary",
            description: Some("Cavaliers serve many roles on the battlefield, from bold leaders and shock troops to dashing knights and mounted juggernauts. Some cavaliers, however, focus more on speed and mobility than they do on the raw power of the mounted charge. On foot or in the saddle, the emissary is usually first to meet the enemy, with a pointed lance, a drawn sword, or terms for surrender."),
            source_page: Some("p.36"),
            prerequisites: Some(&["PRECLASS:1,Cavalier=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Cavalier Archetype ~ Emissary],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CavalierBonusFeats,TYPE.CavalierHeavyArmorProficiency,TYPE.CavalierTactician,TYPE.CavalierBanner,TYPE.CavalierGreaterTactician,TYPE.CavalierGreaterBanner,TYPE.CavalierSupremeCharge]"]),
            replaces: Some(&["CavalierHeavyArmorProficiency", "CavalierTactician", "CavalierBanner", "CavalierGreaterTactician", "CavalierGreaterBanner", "CavalierSupremeCharge", "CavalierBonusFeats"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Emissary ~ Armor and Weapon Proficiency", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Emissary ~ Bonus Feats", at_level: 1, description: Some("An emissary can select Endurance, Fleet, or Run in addition to the list of combat feats whenever he gains a bonus feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Emissary ~ In or Out of the Saddle", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Emissary ~ Battlefield Agility", at_level: 5, description: Some("At 5th level, an emissary learns to be more aware of the threats that surround him in combat, and shares this awareness with his loyal steed. Both the emissary and his mount gain Mobility as a bonus feat. This ability replaces banner."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Emissary ~ Mounted Acrobatics", at_level: 9, description: Some("At 9th level, an emissary is even more adept at hopping on and off his steed, and even dropping alongside its flanks mid-gallop. He gains Trick Riding as a bonus feat, even if he does not meet the prerequisites. The emissary can employ this feat even when wearing medium armor. This ability replaces greater tactician."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Emissary ~ Mounted Dervish", at_level: 14, description: Some("At 14th level, an emissary becomes a fearsome, mobile foe on the battlefield. He gains Mounted Skirmisher as a bonus feat, even if he does not meet the prerequisites. In addition, whenever the emissary is mounted and takes the charge action, he adds 10 feet to his mount's speed. This ability replaces greater banner."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Emissary ~ Erratic Charge", at_level: 20, description: Some("At 20th level, an emissary learns to ride through the thick of combat, striking and then moving away with blinding speed. Whenever the emissary is mounted and takes the charge action, he can first move 10 feet and make a melee attack. This first attack is not a charge, but a regular melee attack, and the movement counts toward the total allowed movement for the round. After making this attack, the emissary must still move in a straight line to reach the target of his charge, and he takes a -5 penalty on the attack roll for the attack at the end of his charge. This ability replaces supreme charge."), benefit: None },
            ],
        },
        // Cavalier Archetype ~ Gendarme -- uc_abilities_class.lst:470
        ArchetypeSwapEntry {
            key: "Cavalier Archetype ~ Gendarme",
            subject: "Cavalier",
            archetype_name: "Gendarme",
            description: Some("The gendarme cares less for the finer points of tactical precision than he does for the exhilaration of the charge: the rush of wind through the visor of his helmet, the feel of his couched lance, the satisfying shriek of armor giving way before his weapon's force as the point drives past metal into his foes."),
            source_page: Some("p.37"),
            prerequisites: Some(&["PRECLASS:1,Cavalier=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Cavalier Archetype ~ Gendarme],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CavalierTactician,TYPE.CavalierGreaterTactician,TYPE.CavalierMasterTactician,TYPE.CavalierBonusFeat,TYPE.CavalierSupremeCharge]"]),
            replaces: Some(&["CavalierTactician", "CavalierGreaterTactician", "CavalierMasterTactician", "CavalierBonusFeat", "CavalierSupremeCharge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Gendarme ~ Bonus Feats", at_level: 1, description: Some("A gendarme trains to be a mounted terror, almost to the exclusion of all other abilities. He gains bonus feats at 1st level, 5th level, and then every three levels thereafter, but must select these bonus feats from the following list - Improved Bull Rush, Mounted Combat, Power Attack, Ride-By Attack, Spirited Charge, Spring Attack, and Unseat. If the gendarme has already selected all of the listed feats, then he may select his bonus feats from those feats listed as combat feats. This ability replaces tactician, greater tactician, master tactician, and the standard cavalier's selection of bonus feats."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Gendarme ~ Transfixing Charge", at_level: 20, description: Some("At 20th level, a gendarme represents the epitome of mounted combat. Whenever he makes a charge attack while mounted, he deals triple the normal damage (quadruple if using a lance); this damage includes all increases from the Spirited Charge feat and from the use of a lance. In addition, if the gendarme confirms a critical hit on a charge attack while mounted, the attack deals maximum damage for the weapon wielded. Additional damage from weapon properties, magic effects, precision-based bonuses, or other increases are rolled normally. This ability replaces supreme charge."), benefit: None },
            ],
        },
        // Cavalier Archetype ~ Honor Guard -- uc_abilities_class.lst:471
        ArchetypeSwapEntry {
            key: "Cavalier Archetype ~ Honor Guard",
            subject: "Cavalier",
            archetype_name: "Honor Guard",
            description: Some("Certain cavaliers are trained not as advance combatants, but as loyal guards, standing as firm defenders in the face of threats to their chosen charge. These honor guards are sometimes merely ornamental, performing their functions on a ceremonial basis, but a surprising number are capable, deadly opponents, able to take down a threat long before their liege knows of the danger. To be an honor guard is to constantly place one's self in harm's way, and to always be willing to lay down one's life at a moment's notice."),
            source_page: Some("p.37"),
            prerequisites: Some(&["PRECLASS:1,Cavalier=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Cavalier Archetype ~ Honor Guard],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CavalierCavaliersCharge,TYPE.CavalierDemandingChallenge,TYPE.CavalierMightyCharge]"]),
            replaces: Some(&["CavalierCavaliersCharge", "CavalierDemandingChallenge", "CavalierMightyCharge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Honor Guard ~ Sworn Defense", at_level: 1, description: Some("At 1st level, whenever an honor guard issues a challenge, he can select one ally as his ward for the duration of the challenge. Whenever the honor guard is adjacent to his ward, he takes a -1 penalty to Armor Class, and the ward receives a +1 dodge bonus to AC. This modifies the challenge ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Honor Guard ~ Intercept", at_level: 3, description: Some("At 3rd level, an honor guard learns to better disrupt the attacks of his enemies. He gains Bodyguard (Advanced Player's Guide 151) as a bonus feat, even if he does not meet the prerequisite. In addition, whenever the honor guard uses the aid another action to increase an ally's Armor Class, the bonus to Armor Class granted by the aid another action increases by +1. This ability replaces cavalier's charge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Honor Guard ~ Warding Charge", at_level: 11, description: Some("At 11th level, an honor guard can flash across the battlefield to protect his ward against the target of his challenge. Whenever the target of the honor guard's challenge makes an attack against his ward, the honor guard can move up to his speed and make a single melee attack against the target of his challenge as an immediate action. This movement and attack can be made as a charge if the movement qualifies. This ability replaces mighty charge. honor guard declares a challenge, his target must pay particular attention to him and his ability to intercept attacks. As long as the target is within the threatened area of the cavalier, it takes a -2 penalty on attack rolls against anyone other than the honor guard. This ability replaces demanding challenge."), benefit: None },
            ],
        },
        // Cavalier Archetype ~ Luring Cavalier -- uc_abilities_class.lst:472
        ArchetypeSwapEntry {
            key: "Cavalier Archetype ~ Luring Cavalier",
            subject: "Cavalier",
            archetype_name: "Luring Cavalier",
            description: Some("Those who study the perfection of strategy and tactics know that picking the battlefield can grant advantages that only overwhelming numbers of allies can eclipse. The luring cavalier is a special type of cavalier that helps accomplish that goal. Typically, he uses a bow or other ranged weapon to lure opponents toward him, and then makes strategic retreats to position his enemies exactly where he wants them."),
            source_page: Some("p.38"),
            prerequisites: Some(&["PRECLASS:1,Cavalier=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Cavalier Archetype ~ Luring Cavalier],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CavalierChallenge,TYPE.CavalierCavaliersCharge,TYPE.CavalierMightyCharge,TYPE.CavalierSupremeCharge]"]),
            replaces: Some(&["CavalierCavaliersCharge", "CavalierMightyCharge", "CavalierSupremeCharge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Luring Cavalier ~ Far Challenge", at_level: 1, description: Some("Once per day, a luring cavalier can use his far challenge ability as a swift action. When he does, the luring cavalier chooses one target within sight to challenge. The luring cavalier's ranged attacks deal extra damage whenever the attacks are made against the target of his challenge. This extra damage is equal to the luring cavalier's level. The luring cavalier can use this ability once per day at 1st level, plus one additional time per day for every three levels beyond 1st, to a maximum of seven times per day at 19th level. Furthermore, once per day, the luring cavalier may spend a use of his far challenge ability to double the potential extra damage of his ranged attack. Before making the attack roll, he can choose to spend a use of his challenge to deal twice his cavalier level in extra damage on a successful hit instead of just his cavalier level in extra damage. If the attack misses, the use of the challenge is wasted. Challenging a foe requires subtle deceits and strategies. The cavalier must make it look like he is a soft target. The subject of the far challenge gains a +4 bonus on attack rolls made against him. This challenge remains in effect until the target is dead or unconscious, until the target hits the luring cavalier with a melee attack, or until the combat ends. If this challenge ends because the target hits the luring cavalier with a melee attack, this challenge changes to the effects of the normal cavalier challenge, and gains any order benefit the luring cavalier has. Far challenge replaces the challenge ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Luring Cavalier ~ Careful Aim", at_level: 3, description: Some("At 3rd level, when a luring cavalier makes a ranged attack with his highest base attack bonus, he can ignore the penalties for firing up to three range increments away. If he is using a firearm, he can target touch AC up to two range increments away. This effect stacks with effects that allow the cavalier to make ranged attacks farther without penalty or that allow him to target touch AC with a firearm beyond the first range increment. This ability replaces cavalier's charge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Luring Cavalier ~ Infuriating Aim", at_level: 11, description: Some("At 11th level, when a luring cavalier confirms a critical hit with a ranged attack made at his highest base attack bonus against the target of a far challenge, the target becomes infuriated for 1 round. While infuriated, the target must spend its turn moving closer to the cavalier, making move, run, or charge actions only (challenged creature's choice). This is a mind-affecting effect. This ability replaces mighty charge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Luring Cavalier ~ Versatile Challenge", at_level: 12, description: Some("At 12th level, a luring cavalier can expend a use of his challenge to gain the benefit of far challenge or the normal cavalier challenge. He can even change the type of challenge an opponent is under the effects of as a swift action, as long as the target of the challenge is within line of sight."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Luring Cavalier ~ Supreme Aim", at_level: 20, description: Some("At 20th level, the luring cavalier gains the careful aim bonus and the effect of infuriating aim on all ranged attacks. This ability replaces supreme charge."), benefit: None },
            ],
        },
        // Cavalier Archetype ~ Musketeer -- uc_abilities_class.lst:473
        ArchetypeSwapEntry {
            key: "Cavalier Archetype ~ Musketeer",
            subject: "Cavalier",
            archetype_name: "Musketeer",
            description: Some("Some cavaliers are entrusted by their masters with the care and use of expensive and powerful oddities- firearms. Capable of inflicting shocking amounts of destruction without the aid of magic, these musketeers lead the advance of their troops, firing devastating fusillades long before the enemy can prepare for the armed charge that follows."),
            source_page: Some("p.38"),
            prerequisites: Some(&["PRECLASS:1,Cavalier=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Cavalier Archetype ~ Musketeer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CavalierHeavyArmorProficiency,TYPE.CavalierMount,TYPE.CavalierExpertTrainer]"]),
            replaces: Some(&["CavalierMount", "CavalierExpertTrainer", "CavalierHeavyArmorProficiency"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Musketeer ~ Weapon and Armor Proficiency", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Musketeer ~ Gifted Firearm", at_level: 1, description: Some("At 1st level, a musketeer is gifted by his order with a firearm (either a pistol or a musket). This weapon is both a symbol of the cavalier's duty and a focus for much of his talent. He cannot sell this weapon. He also gains the Gunsmithing feat. Beginning at 8th level, the musketeer can focus himself when wielding his gifted weapon. As a standard action, he can focus himself to gain a number of benefits for 1 minute per cavalier level. The musketeer can use this ability twice per day, plus one additional time per day for every four levels beyond 8th, to a total of five times per day at 20th level. At 8th level, the cavalier gains the benefit of the Improved Critical feat. At 11th level, he can reduce the misfire chance of his weapon by 1 (to a minimum of 1), and at 17th level, the musketeer can double the range increment of his weapon. These abilities do not stack with the magical weapon special abilities that they duplicate. Finally, at 20th level, during a period of such focus, the musketeer can engage in a flurry of gunfire. The cavalier can expend one of his daily challenges to make a full attack with his firearm. The musketeer ignores the normal load times for his weapon, but must still expend enough doses of powder and enough bullets to complete each attack. If the musketeer's gifted firearm is destroyed, the cavalier loses this ability for 30 days while a replacement weapon is crafted for him. During this 30-day period, the musketeer takes a -1 penalty on weapon and damage rolls. The reduction of the weapon's misfire chance and the doubled range increment abilities don't stack with the similar benefits of the reliable and distance magic weapon special abilities."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Musketeer ~ Swift Powder", at_level: 4, description: Some("At 4th level, the musketeer gains Rapid Reload (musket) or Rapid Reload (pistol) as a bonus feat. At 14th level, each time the musketeer issues a challenge, he may reload a wielded firearm as a free action. This ability replaces expert trainer."), benefit: None },
            ],
        },
        // Cavalier Archetype ~ Standard Bearer -- uc_abilities_class.lst:474
        ArchetypeSwapEntry {
            key: "Cavalier Archetype ~ Standard Bearer",
            subject: "Cavalier",
            archetype_name: "Standard Bearer",
            description: Some("Not all cavaliers are content to ride at the head of a charge, leading from the front and facing down their enemies directly. Some prefer to stand away from the fray, their banners a beacon shining brightly over the battlefield, rallying their troops to victory. The standard bearer employs the banner of his order, his lord, or his own house to raise the spirits of his allies and warns enemies of impending doom."),
            source_page: Some("p.39"),
            prerequisites: Some(&["PRECLASS:1,Cavalier=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Cavalier Archetype ~ Standard Bearer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CavalierMount,TYPE.CavalierCavaliersBanner,TYPE.CavalierMightyCharge,TYPE.CavalierSupremeCharge]"]),
            replaces: Some(&["CavalierMount", "CavalierCavaliersBanner", "CavalierMightyCharge", "CavalierSupremeCharge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Standard Bearer ~ Banner", at_level: 1, description: Some("At 1st level, a standard bearer gains the banner ability. This ability is identical to the standard cavalier's banner ability, except that the morale bonuses on saving throws against fear effects and on attack rolls made as part of a charge increase at 5th level, and every five levels thereafter. This ability replaces the standard cavalier's mount ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Standard Bearer ~ Mount", at_level: 5, description: Some("At 5th level, a standard bearer gains the service of a loyal and trusted mount. This mount is identical in all ways to the standard cavalier's mount. This ability replaces the standard cavalier's banner ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Standard Bearer ~ Banner of Solace", at_level: 11, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Standard Bearer ~ Awesome Pennon", at_level: 20, description: Some("At 20th level, a standard bearer's banner has become a powerful rallying point to his allies, and a bane to his foes. Whenever his banner is visible, allies of the standard bearer within 60 feet gain a +1 morale bonus on attack rolls, immunity to fear effects, and a +3 morale bonus on saving throws against mindaffecting effects. This ability replaces supreme charge."), benefit: None },
            ],
        },
        // Cavalier Archetype ~ Strategist -- uc_abilities_class.lst:475
        ArchetypeSwapEntry {
            key: "Cavalier Archetype ~ Strategist",
            subject: "Cavalier",
            archetype_name: "Strategist",
            description: Some("Some cavaliers make a lifelong mission out of their exceptional ability to direct troops on the battlefield, combining tactical insight with a preternatural skill at improving the teamwork of their allies in order to win the day. Such strategists are masters at learning the strengths and weaknesses of those they fight alongside, and at employing those observations at the most opportune moments. The best of them are also skilled at determining the strengths and weaknesses of their enemies, as well as at countering some of those strengths with their own formidable abilities."),
            source_page: Some("p.39"),
            prerequisites: Some(&["PRECLASS:1,Cavalier=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Cavalier Archetype ~ Strategist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CavalierExpertTrainer,TYPE.CavalierGreaterBanner,TYPE.CavalierCavaliersBonusFeat18]"]),
            replaces: Some(&["CavalierExpertTrainer", "CavalierGreaterBanner", "CavalierCavaliersBonusFeat18"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Strategist ~ Tactician", at_level: 1, description: Some("As the cavalier class feature, except that a strategist can use this ability once per day at 1st level, plus one additional time per day at 5th level and for every four levels thereafter (to a maximum of five times per day at 17th level)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Strategist ~ Drill Instructor", at_level: 4, description: Some("At 4th level, a strategist learns to train his fellow adventurers in the nuances of squad combat. By spending 10 minutes and expending 1 use of his challenge ability, the strategist can grant the use of a teamwork feat that he knows to up to four of his allies, similar to the tactician ability. As with tactician, allies need not meet the prerequisites of the granted feat, but they retain the use of this feat for 10 minutes plus 1 minute for every two levels the cavalier possesses, as long as the cavalier is visible and can be heard by his allies. If the cavalier falls unconscious or cannot be both seen and heard, his allies lose the benefit of the granted feat until the condition is remedied. This ability replaces expert trainer."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Strategist ~ Tactical Advantage", at_level: 14, description: Some("At 14th level, whenever a strategist uses his tactician ability to grant the use of a teamwork feat to his allies, he can move up to his speed as a free action either before or after granting the feat. This movement provokes attacks of opportunity normally. This ability replaces greater banner."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Strategist ~ Strategic Supremacy", at_level: 18, description: Some("At 18th level, a strategist gains the ability not only to improve his allies' abilities, but also to disrupt the teamwork of his enemies. Whenever the strategist uses the tactician ability, he can choose to cancel out the effects of one teamwork feat employed by any of his opponents within 30 feet instead of granting the use of a teamwork feat to his allies. This ability replaces the cavalier's 18th-level bonus feat."), benefit: None },
            ],
        },
        // Cleric Archetype ~ Crusader -- uc_abilities_class.lst:528
        ArchetypeSwapEntry {
            key: "Cleric Archetype ~ Crusader",
            subject: "Cleric",
            archetype_name: "Crusader",
            description: Some("Crusaders serve the militant arm of a church, ready to stand guard over the religion's holy places and to be its swift, avenging arm against those who resist its truth."),
            source_page: Some("p.40"),
            prerequisites: Some(&["PRECLASS:1,Cleric=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Cleric Archetype ~ Crusader],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ClericDomains]"]),
            replaces: Some(&["ClericDomains"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Crusader ~ Diminished Spellcasting", at_level: 1, description: Some("A crusader chooses only one domain and gains one fewer spell of each level than normal. If this reduces the number to 0, she may cast spells of that level only if they are domain spells or if her Wisdom allows bonus spells of that level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Crusader ~ Bonus Feat", at_level: 1, description: Some("A crusader gains a bonus feat at 1st level, then again at 5th level and every five levels thereafter (to a maximum of six at 20th level). These bonus feats must be chosen from the following list - Heavy Armor Proficiency, Improved Shield Bash, Martial Weapon Proficiency, Saving Shield, Shield Focus, Tower Shield Proficiency, and Weapon Focus*. At 10th level, a crusader may also choose from the following feats - Exotic Weapon Proficiency, Greater Shield Focus, Greater Weapon Focus*, Improved Critical*, Shield Slam, Shield Specialization, and Weapon Specialization*. At 20th level, a crusader may also choose from the following feats - Greater Shield Specialization and Greater Weapon Specialization*. Bonus feats marked with an asterisk (*) must be applied to the favored weapon of the crusader's deity. A crusader need not meet the normal class- or level-based prerequisites for these bonus feats."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Crusader ~ Legion's Blessing", at_level: 8, description: Some("At 8th level, a crusader gains the ability to confer beneficial spells quickly to a large group of allies. As a full-round action, the crusader may confer the effects of a single harmless spell with a range of touch to a number of creatures equal to half her cleric level. The spell's range remains touch, so all intended recipients must be within the crusader's reach when the spell is cast. Using the legion's blessing expends the prepared spell, but it also requires the crusader to sacrifice another prepared spell three levels higher, as when spontaneously using a cure or inflict spell. The higher-level spell is not cast but is simply lost, its magical energy used to power the legion's blessing."), benefit: None },
            ],
        },
        // Cleric Archetype ~ Divine Strategist -- uc_abilities_class.lst:529
        ArchetypeSwapEntry {
            key: "Cleric Archetype ~ Divine Strategist",
            subject: "Cleric",
            archetype_name: "Divine Strategist",
            description: Some("The divine strategist leads the armies of the faithful, not from the front lines but through her clever strategy and tactical acumen."),
            source_page: Some("p.32"),
            prerequisites: Some(&["PRECLASS:1,Cleric=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Cleric Archetype ~ Divine Strategist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ClericDomains]"]),
            replaces: Some(&["ClericDomains", "ClericChannelEnergy"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Divine Strategist ~ Domains", at_level: 1, description: Some("A divine strategist gains only a single domain."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Divine Strategist ~ Caster Support", at_level: 1, description: Some("A divine strategist can use the aid another action to assist another divine spellcaster, granting a +2 circumstance bonus on caster level checks and concentration checks until the beginning of the divine strategist's next turn. This bonus increases by +1 at 4th level and every four levels thereafter (to a maximum of +7 at 20th level). The allied caster must remain adjacent to the divine strategist to gain this benefit. Caster support can be used to assist arcane spellcasters or characters using magical items, but they gain only half the normal bonus."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Divine Strategist ~ Master Tactician", at_level: 1, description: Some("A divine strategist can always act in a surprise round even if she fails to make a Perception check to notice enemies, though she is considered flat-footed until she acts. In addition, the divine strategist gains a bonus on initiative checks equal to +%1. At 20th level, a divine strategist's initiative roll is automatically a natural 20. Allies able to see and hear the divine strategist gain a bonus on initiative checks equal to +%2. This is a language-dependent ability. This ability replaces channel energy.|MasterTacticianInitiativeBonus|MasterTacticianInitiativeBonus/2"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Divine Strategist ~ Tactical Expertise", at_level: 8, description: Some("A divine strategist knows how to take best advantage of tactical opportunities. Whenever she is flanking or makes an attack of opportunity, she may add her Intelligence bonus (if any) as a bonus on the attack roll. In addition, %1 times per day as a swift action she may add her Intelligence modifier as a bonus on any single d20 roll made as part of a readied action.|1+(MAX(0,classlevel(\"Cleric\")-8)/2)"), benefit: None },
            ],
        },
        // Cleric Archetype ~ Evangelist -- uc_abilities_class.lst:530
        ArchetypeSwapEntry {
            key: "Cleric Archetype ~ Evangelist",
            subject: "Cleric",
            archetype_name: "Evangelist",
            description: Some("The evangelist is the voice of her religion in the world. Where others nurture the faith among believers, an evangelist proclaims the coming glory of her deific patron and issues the clarion call to all around to heed the truth, or obey the call to war and crusade against the enemies of the church."),
            source_page: Some("p.32"),
            prerequisites: Some(&["PRECLASS:1,Cleric=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Cleric Archetype ~ Evangelist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ClericDomains,TYPE.ClericArmorProficiency,TYPE.ClericSpontaneousCasting,TYPE.ClericChannelEnergy]"]),
            replaces: Some(&["ClericDomains", "ClericArmorProficiency", "ClericSpontaneousCasting", "ClericChannelEnergy"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Evangelist ~ Single-Minded", at_level: 1, description: Some("An evangelist focuses her skills and learning on proclamation rather than the fine details of the church's deeper mysteries or martial training. Thus, she may select only one domain and does not gain medium armor proficiency or shield proficiency."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Evangelist ~ Public Speaker", at_level: 1, description: Some("An evangelist gains Perform as a class skill. In addition, she is trained to project her voice with great skill and effect; the DC to hear her speak in difficult conditions is reduced by %1.|MAX(classlevel(\"Cleric\") + CHA,0)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Evangelist ~ Sermonic Performance", at_level: 1, description: Some("An evangelist gains the ability to deliver a select number of supernatural and spell-like performances through the force and power of her divinely inspired preaching and exhortation. This ability is similar in all respects to bardic performance as used by a bard of the same level (including interactions with feats, spells, and prestige classes), using Perform (oratory) as the evangelist's performance skill. However, an evangelist gains only the following types of  bardic performance: countersong, fascinate, and inspire courage at 1st level; inspire greatness at 9th level; and inspire heroics at 15th level. Sermonic performance replaces the 1st-, 9th-, and 15th-level channel energy abilities. This caps the cleric's channel energy damage at 7d6 points."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Evangelist ~ Spontaneous Casting", at_level: 1, description: Some("An evangelist does not gain the ability to spontaneously cast cure or inflict spells by sacrificing prepared spells. However, an evangelist can spontaneously cast command (1st), enthrall (2nd), tongues (3rd), suggestion (4th), greater command (5th), geas/quest (6th), mass suggestion (7th), sympathy (8th) and demand (9th) as a spell of listed level by sacrificing a prepared spell of the same level or higher."), benefit: None },
            ],
        },
        // Cleric Archetype ~ Merciful Healer -- uc_abilities_class.lst:531
        ArchetypeSwapEntry {
            key: "Cleric Archetype ~ Merciful Healer",
            subject: "Cleric",
            archetype_name: "Merciful Healer",
            description: Some("The merciful healer is a master of battlefield revivification, sustaining and restoring allies to keep them in the fight."),
            source_page: Some("p.41"),
            prerequisites: Some(&["PRECLASS:1,Cleric=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Cleric Archetype ~ Merciful Healer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ClericDomains,TYPE.ClericChannelEnergy]"]),
            replaces: Some(&["ClericDomains", "ClericChannelEnergy"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Merciful Healer ~ Willing Healer", at_level: 1, description: Some("A merciful healer must choose the Healing domain. She does not gain a second domain. If the cleric worships a deity, that deity must be one that grants the Healing domain. A merciful healer must channel positive energy."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Merciful Healer ~ Channel Energy", at_level: 1, description: Some("As the cleric ability, save that a merciful healer must channel positive energy, and when she does, she cannot choose to target undead. This ability is otherwise identical to the cleric ability of the same name."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Merciful Healer ~ Combat Medic", at_level: 1, description: Some("A merciful healer does not provoke attacks of opportunity when using the Heal skill to stabilize another creature or casting healing spells."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Merciful Healer ~ Merciful Healing", at_level: 2, description: Some("At 3rd level, a merciful healer can channel positive energy to relieve one or more harmful conditions. The merciful healer chooses one of the following harmful conditions at 3rd level - fatigued, shaken, or sickened. When the merciful healer channels energy she can remove the chosen condition from one living creature that she heals within her channel energy burst. At 6th level, she can choose another condition. It can be one of those she didn't choose at 3rd level, or one of the following conditions - dazed, diseased, or staggered. She can remove the selected condition or the one she chose at 3rd level from up to two creatures within her channel energy burst. She gains another condition at 9th level, and can choose an above condition or one of the following conditions - cursed, exhausted, frightened, nauseated, or poisoned. She can remove that condition, or a condition she previously chose, from one or two creatures within the burst. Finally, at 12th level, she can choose a lowerlevel condition or one of the following conditions - blinded, deafened, paralyzed, or stunned. She can remove that condition or one she previously chose from one, two, or three creatures within her channel energy burst. Feats and effects that affect a paladin's mercy also affect this ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Merciful Healer ~ True Healer", at_level: 8, description: Some("At 8th level, when a merciful healer channels holy energy, she can choose to apply the benefits of merciful healing or to reroll any 1s when determining how much damage she heals with the holy energy. She must choose which benefit to take before she rolls to see how much damage she heals."), benefit: None },
            ],
        },
        // Druid Archetype ~ Ape Shaman -- uc_abilities_class.lst:615
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Ape Shaman",
            subject: "Druid",
            archetype_name: "Ape Shaman",
            description: Some("A shaman with this totem calls upon the mighty ape, a peaceful but powerful simian whose strength is beyond compare. An ape shaman is a friendly protector of the forest, but will crush those enemies who rouse her anger."),
            source_page: Some("p.42"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Ape Shaman],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidThousandFaces,TYPE.DruidVenomImmunity,TYPE.DruidNatureBond]"]),
            replaces: Some(&["DruidThousandFaces", "DruidVenomImmunity", "DruidNatureBond"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Ape Shaman ~ Nature Bond", at_level: 1, description: Some("An ape shaman who chooses an animal companion must select an ape or related primate. If choosing a domain, the ape shaman must choose from the Animal, Community (Family subdomain), Destruction (Rage subdomain), and Strength domains."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ape Shaman ~ Wild Empathy", at_level: 1, description: Some("An ape shaman can use wild empathy with apes and other primates as a full-round action with a +4 bonus."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ape Shaman ~ Totem Transformation", at_level: 2, description: Some("At 2nd level, an ape shaman may adopt an aspect of the ape while retaining her normal form. The druid gains one of the following sets of bonuses and abilities - movement (climb speed 20 ft., +4 racial bonus on Climb checks), senses (low-light vision, scent), natural weapons (2 slams [1d6 for a Medium shaman], +2 on combat maneuver checks to grapple), or toughness (+2 natural armor bonus to AC, Endurance feat). While using totem transformation, the ape shaman may speak normally and can cast speak with animals (primates only) at will. Using this ability is a standard action at 2nd level, a move action at 7th level, and a swift action at 12th level. The ape shaman can use this ability for a number of minutes per day equal to her druid level. These minutes do not need to be consecutive, but they must be used in 1-minute increments. This is a polymorph effect and cannot be used while the druid is using another polymorph effect, such as wild shape. This ability replaces woodland stride."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ape Shaman ~ Totemic Summons", at_level: 5, description: Some("At 5th level, an ape shaman may cast summon nature's ally as a standard action when summoning primates, and these summoned creatures gain temporary hit points equal to her druid level. She can apply the young template to any primate to reduce the level of the summoning spell required by one. She can also increase the level of summoning required by one in order to apply either the advanced or the giant template, or increase it by two to apply both the advanced and giant templates. This ability replaces a thousand faces."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ape Shaman ~ Wild Shape", at_level: 6, description: Some("At 6th level, an ape shaman's wild shape ability functions at her druid level -2. If she takes on the form of an ape, she instead uses her druid level +2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ape Shaman ~ Bonus Feat", at_level: 9, description: Some("At 9th level and every four levels thereafter, an ape shaman gains one of the following bonus feats: Diehard, Endurance, Great Fortitude, Improved Bull Rush, and Toughness. She must meet the prerequisites for these bonus feats. This ability replaces venom immunity."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Druid ~ Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Progression", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Times", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Wild Shape", at_level: 6, description: None, benefit: None },
            ],
        },
        // Druid Archetype ~ Bat Shaman -- uc_abilities_class.lst:616
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Bat Shaman",
            subject: "Druid",
            archetype_name: "Bat Shaman",
            description: Some("The bat shaman's totem is the agile bat, flitting and turning with incredible speed through even the most convoluted mazes. Her enemies do not know when she will appear, and when she does, she strikes fast and hard before disappearing into the night."),
            source_page: Some("p.42"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Bat Shaman],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidVenomImmunity,TYPE.DruidThousandFaces,TYPE.DruidNatureBond]"]),
            replaces: Some(&["DruidThousandFaces", "DruidVenomImmunity", "DruidNatureBond"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Bat Shaman ~ Nature Bond", at_level: 1, description: Some("A bat shaman who chooses an animal companion must select a bat. If choosing a domain, the bat shaman must choose from Air, Animal, Darkness (Night subdomain), and Trickery (Deception subdomain)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bat Shaman ~ Wild Empathy", at_level: 1, description: Some("A bat shaman can use wild empathy with bats as a full-round action with a +4 bonus."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bat Shaman ~ Totem Transformation", at_level: 2, description: Some("At 2nd level, a bat shaman may adopt an aspect of the bat while retaining her normal form. This ability functions as the ape shaman ability, but the druid may select from the following sets of bonuses and abilities - movement (fly speed 30 ft. [average]; the druid must be at least 5th level to select this bonus), natural weapons (bite [1d4 for a Medium shaman]), or senses (blindsense 20 ft.). While using totem transformation, the bat shaman may speak normally and can cast speak with animals (bats only) at will. This ability replaces woodland stride."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bat Shaman ~ Totemic Summons", at_level: 5, description: Some("At 5th level, a bat shaman may cast summon nature's ally as a standard action when summoning bats, and these summoned creatures gain temporary hit points equal to her druid level. This ability otherwise functions as the ape shaman ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bat Shaman ~ Wild Shape", at_level: 6, description: Some("At 6th level, a bat shaman's wild shape ability functions at her druid level -2. If she takes on the form of a bat, she instead uses her druid level + 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bat Shaman ~ Bonus Feat", at_level: 9, description: Some("At 9th level and every four levels thereafter, a bat shaman gains one of the following bonus feats: Acrobatic, Agile Maneuvers, Improved Initiative, Lightning Reflexes, or Skill Focus (Perception). She must meet the prerequisites for these bonus feats. This ability replaces venom immunity."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Druid ~ Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Progression", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Times", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Wild Shape", at_level: 6, description: None, benefit: None },
            ],
        },
        // Druid Archetype ~ Boar Shaman -- uc_abilities_class.lst:617
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Boar Shaman",
            subject: "Druid",
            archetype_name: "Boar Shaman",
            description: Some("A boar shaman chooses the stolid and ferocious boar as her totem. Content to be left alone, she becomes one of the most dangerous creatures of the wild when provoked."),
            source_page: Some("p.43"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Boar Shaman],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidVenomImmunity,TYPE.DruidThousandFaces,TYPE.DruidNatureBond]"]),
            replaces: Some(&["DruidThousandFaces", "DruidVenomImmunity", "DruidNatureBond"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Boar Shaman ~ Nature Bond", at_level: 1, description: Some("A boar shaman who chooses an animal companion must select a boar. If choosing a domain, the boar shaman must choose from the Animal, Destruction (Rage), Protection, and Strength domains."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Boar Shaman ~ Wild Empathy", at_level: 1, description: Some("A boar shaman can use wild empathy with porcine creatures as a full-round action with a +4 bonus."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Boar Shaman ~ Totem Transformation", at_level: 2, description: Some("At 2nd level, a boar shaman may adopt an aspect of the boar while retaining her normal form. This ability functions as the ape shaman ability, but the druid may select from the following sets of bonuses and abilities - movement (+10 enhancement bonus to land speed), senses (low-light vision, scent), natural weapons (gore [1d8 for a Medium druid], +2 on combat maneuver checks to overrun), or toughness (+2 natural armor bonus to AC, Endurance feat). While using totem transformation, the boar shaman may speak normally and can cast speak with animals (boars and related animals only) at will. This ability replaces woodland stride."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Boar Shaman ~ Totemic Summons", at_level: 5, description: Some("At 5th level, a boar shaman may cast summon nature's ally as a standard action when summoning boars or other porcine creatures, and these summoned creatures gain temporary hit points equal to her druid level. This ability otherwise functions as the ape shaman ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Boar Shaman ~ Wild Shape", at_level: 6, description: Some("At 6th level, a boar shaman's wild shape ability functions at her druid level -2. If she takes on the form of a boar, she instead uses her druid level +2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Boar Shaman ~ Bonus Feat", at_level: 9, description: Some("At 9th level and every four levels thereafter, a boar shaman gains one of the following bonus feats: Bleeding Critical, Blind-Fight, Diehard, or Improved Overrun. She must meet the prerequisites for these bonus feats. This ability replaces venom immunity."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Druid ~ Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Progression", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Times", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Wild Shape", at_level: 6, description: None, benefit: None },
            ],
        },
        // Druid Archetype ~ World Walker -- uc_abilities_class.lst:618
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ World Walker",
            subject: "Druid",
            archetype_name: "World Walker",
            description: Some("While all druids traverse the wilderness with ease, the world walkers take it upon themselves to travel the entire world. Some act as messengers and scouts for druidic circles, while others have a seemingly unquenchable wanderlust; each new land provides new mysteries to discover and new wisdom to be gained from mastering those mysteries."),
            source_page: Some("p.43"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ World Walker],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidTracklessStep,TYPE.DruidResistNaturesLure,TYPE.DruidVenomImmunity,TYPE.DruidTimelessBody]"]),
            replaces: Some(&["DruidTracklessStep", "DruidResistNaturesLure", "DruidVenomImmunity", "DruidTimelessBody"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "World Walker ~ Favored Terrain", at_level: 3, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "World Walker ~ Path of Trees", at_level: 9, description: None, benefit: None },
            ],
        },
        // Fighter Archetype ~ Armor Master -- uc_abilities_class.lst:675
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Armor Master",
            subject: "Fighter",
            archetype_name: "Armor Master",
            description: Some("All fighters have two main tools of their trade-weapons and armor. While many fighters hone their weapon skills to a point of inescapable grace and lethality, there are those who live under the maxim that a good offense can be accomplished though an impenetrable defense. To these fighters, proper use of armor and shields ensures that they can fight another day, and that the frustration of enemies who can't seem to crack an armor master's superior defenses is just the first symptom of his foes' eventual defeat."),
            source_page: Some("p.44"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ ARMOR MASTER],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterBravery,TYPE.FighterWeaponTraining1,TYPE.FighterWeaponTraining3,TYPE.FighterArmorMastery,TYPE.FighterWeaponTraining2,TYPE.FighterWeaponTraining4,TYPE.FighterWeaponMastery]"]),
            replaces: Some(&["FighterBravery", "FighterWeaponTraining_ALL", "FighterWeaponTraining1", "FighterWeaponTraining2", "FighterWeaponTraining3", "FighterWeaponTraining4", "FighterArmorMastery", "FighterWeaponMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Armor Master ~ Deflective Shield", at_level: 2, description: Some("An armor master specializes in using his shield to deflect attacks. He gains a +%1 bonus to his touch AC; however, this bonus cannot exceed the sum of the armor and enhancement bonus to AC provided by the shield that the armor master is currently carrying. This ability replaces bravery.|DeflectiveShieldBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Armor Master ~ Armored Defense", at_level: 5, description: Some("An armor master gains DR 1/- when wearing light armor, DR 2/- when wearing medium armor, and DR 3/- when wearing heavy armor.|PREVARLT:FighterLVL,19"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Armor Master ~ Fortification", at_level: 9, description: Some("An armor master can use his armor to shield critical areas from injury."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Armor Master ~ Indestructible", at_level: 20, description: Some("An armor master gains complete immunity to critical hits and sneak attacks while he is wearing armor. In addition, unless his armor has the fragile armor quality, it cannot be sundered while he is wearing it. This ability replaces weapon mastery."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Brawler -- uc_abilities_class.lst:676
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Brawler",
            subject: "Fighter",
            archetype_name: "Brawler",
            description: Some("All melee is up close and personal, but some warriors bring it as close as they can get. Brawlers can be found anywhere, among all races and societies. A brawler could be hired muscle in a tavern, a local crime syndicate enforcer, or a hotheaded recruit among the ranks of a baron's guard. Often brawlers' in-your-face attitudes are as powerful as their tactics."),
            source_page: Some("p.44"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ BRAWLER],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterArmorTraining1,TYPE.FighterArmorTraining2,TYPE.FighterArmorTraining3,TYPE.FighterArmorTraining4,TYPE.FighterWeaponTraining1,TYPE.FighterWeaponTraining2,TYPE.FighterWeaponTraining3,TYPE.FighterWeaponTraining4,TYPE.FighterArmorMastery]"]),
            replaces: Some(&["FighterArmorTraining1", "FighterArmorTraining2", "FighterArmorTraining3", "FighterArmorTraining4", "FighterWeaponTraining1", "FighterWeaponTraining2", "FighterWeaponTraining3", "FighterWeaponTraining4", "FighterArmorMastery", "FighterArmorTraining_ALL", "FighterWeaponTraining_ALL"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Brawler ~ Close Control", at_level: 2, description: Some("At 2nd level, a brawler becomes skilled at forcefully moving his opponent around the battlefield. The brawler gains a +1 bonus on bull rush, drag, and reposition combat maneuver checks. The brawler also gains a +1 bonus to CMD when attacked with the bull rush, drag, and reposition maneuvers. These bonuses increase by +1 for every four levels after 2nd (to a maximum of +5 at 18th level). This ability replaces armor training 1."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Brawler ~ Close Combatant", at_level: 3, description: Some("At 3rd level, a brawler gains a +1 bonus on attack rolls and a +3 bonus on damage rolls with weapons in the close weapon group. Both of these bonuses increase by +1 for every four levels beyond 3rd (to a maximum of +5 on attack rolls and +7 on damage rolls at 19th level). This ability replaces weapon training 1 and 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Brawler ~ Menacing Stance", at_level: 7, description: Some("At 7th level, a brawler constantly harries and distracts his enemies. While adjacent to the brawler, enemies take a -1 penalty on attack rolls and a -4 penalty on concentration checks. These penalties increase by 1 for every four levels after 7th level (to a maximum of -4 on attack rolls and -7 on concentration checks at 19th level). Creatures do not take these penalties if the brawler is dazed, helpless, staggered, stunned, or unconscious. This ability replaces armor training 2, 3, and 4 and armor mastery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Brawler ~ No Escape", at_level: 9, description: Some("At 9th level, taking a 5-foot step out of the area of a brawler's menacing stance or moving out of the area of a brawler's menacing stance with a withdraw action provokes an attack of opportunity from the brawler. This ability replaces weapon training 3 and 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Brawler ~ Stand Still", at_level: 13, description: Some("At 13th level, a brawler gains Stand Still as a bonus feat, even if he does not have the Combat Reflexes feat. If the brawler already has the Stand Still feat, he can take any other combat feat instead. Furthermore, he gains a bonus equal to 1/2 his fighter level on combat maneuver checks when using the Stand Still feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Brawler ~ Weapon Mastery", at_level: 1, description: Some("A brawler must select a close weapon for this ability."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Cad -- uc_abilities_class.lst:677
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Cad",
            subject: "Fighter",
            archetype_name: "Cad",
            description: Some("In combat, most fighters have some sort of code of honor. Some believe that one should not kick enemies when they are down, or should limit use of other such dirty tricks to the most dire of circumstances. The cad places no such limitations on himself. Battles are for winning, and anything that gives the cad the upper hand against his enemies is a legitimate tactic. No trick is too treacherous if it leads to victory."),
            source_page: Some("p.45"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ CAD],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterWeaponTraining_ALL,TYPE.FighterArmorTraining_ALL,TYPE.FighterWeaponProficiencies,TYPE.FighterArmorProficiencies,TYPE.FighterBravery,TYPE.FighterArmorTraining1,TYPE.FighterArmorTraining3,TYPE.FighterArmorTraining4,TYPE.FighterWeaponTraining1,TYPE.FighterWeaponTraining2,TYPE.FighterWeaponTraining3,TYPE.FighterWeaponTraining4,TYPE.FighterWeaponMastery]"]),
            replaces: Some(&["FighterArmorProficiencies", "FighterBravery", "FighterArmorTraining1", "FighterArmorTraining2", "FighterArmorTraining3", "FighterArmorTraining4", "FighterWeaponTraining1", "FighterWeaponTraining2", "FighterWeaponTraining3", "FighterWeaponTraining4", "FighterWeaponTraining_ALL", "FighterArmorTraining_ALL", "FighterWeaponMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Cad ~ Weapon and Armor Proficiency", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Cad ~ Skills", at_level: 1, description: Some("Acrobatics (Dex), Bluff (Cha), Escape Artist (Dex), Sleight of Hand (Dex), and Stealth (Dex) are class skills for a cad."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cad ~ Dirty Maneuvers", at_level: 2, description: Some("At 2nd level, a cad becomes skilled at deceiving and discomfiting his opponents. The cad gains a +1 bonus on disarm, dirty trick, and steal combat maneuver checks. The cad also gains a +1 bonus to CMD when attacked with the disarm, dirty trick, and steal combat maneuvers. These bonuses increase by 1 for every four levels after 2nd (to a maximum of +5 at 18th level). This ability replaces bravery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cad ~ Catch Off-Guard", at_level: 3, description: Some("At 3rd level, the cad gains the Catch Off-Guard feat. This ability replaces armor training 1."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cad ~ Payback", at_level: 5, description: Some("At 5th level, a cad gains a +1 bonus on attack and damage rolls against any creature that has attacked the cad since the beginning of his last turn. This bonus increases by +1 for every four levels beyond 5th (to a maximum of +4 at 17th level). This ability replaces weapon training 1, 3, and 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cad ~ Deadly Surprise", at_level: 7, description: Some("At 7th level, when a cad hits an opponent that is denied its Dexterity bonus to AC against him with a weapon or unarmed attack, he may attempt a dirty trick combat maneuver as an immediate action as part of the attack. This ability replaces armor training 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cad ~ Razor-Sharp Chair Leg", at_level: 9, description: Some("At 9th level, as a swift action, a cad may alter the type of damage dealt by an improvised weapon to bludgeoning, piercing, or slashing damage. In addition, the cad has a critical threat range of 19-20/x2 with any improvised melee weapon. This ability replaces weapon training 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cad ~ Craven Combatant", at_level: 11, description: Some("At 11th level, when fighting defensively or using Combat Expertise or total defense, a cad cannot be flanked except by a rogue or ninja whose level is four or more higher than the cad's fighter level. This ability replaces armor training 3."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cad ~ Sweeping Prank", at_level: 13, description: Some("At 13th level, as a standard action, a cad can use a dirty trick maneuver against any two adjacent opponents that he can reach, making a separate combat maneuver check against each opponent. He must use the same dirty trick maneuver against each opponent. At 17th level, the cad can use this ability as a full-round action to attempt a dirty trick maneuver against a number of enemies equal to 2 + his Dexterity bonus (if any). This ability replaces weapon training 3 and 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cad ~ Treacherous Blow", at_level: 15, description: Some("At 15th level, when a cad confirms a critical hit, he can attempt a dirty trick combat maneuver as part of that attack as an immediate action. This ability replaces armor training 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cad ~ Ultimate Payback", at_level: 20, description: Some("At 20th level, any critical threats a cad makes against an opponent that has attacked him since the beginning of his last turn are automatically confirmed. This ability replaces weapon mastery."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Dragoon -- uc_abilities_class.lst:678
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Dragoon",
            subject: "Fighter",
            archetype_name: "Dragoon",
            description: Some("These gallant lancers serve in the vanguard of many armies or as knights-errant. They are born leaders and masters of the mounted charge."),
            source_page: Some("p.46"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ DRAGOON],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterTowerShieldProficiency,TYPE.FighterBonusFeat1,TYPE.FighterArmorTraining3,TYPE.FighterArmorTraining2,TYPE.FighterWeaponTraining_ALL,TYPE.FighterArmorTraining4]"]),
            replaces: Some(&["FighterTowerShieldProficiency", "FighterBonusFeat1", "FighterArmorTraining3", "FighterArmorTraining2", "FighterWeaponTraining_ALL", "FighterArmorTraining4"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Dragoon ~ Weapon and Armor Proficiency", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Dragoon ~ Skilled Rider", at_level: 1, description: Some("At 1st level, a dragoon gains both the Mounted Combat and Skill Focus (Ride) feats as bonus feats. This ability replaces the 1st-level fighter bonus combat feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dragoon ~ Spear Training", at_level: 5, description: Some("At 5th level, a dragoon must select weapon training with the spear group. The dragoon's weapon training bonus with spears improves by +1 on attack rolls and +2 on damage rolls for every four levels beyond 5th (to a maximum of +4 on attack rolls and +8 on damage rolls at 17th level). The dragoon does not gain weapon training in any other groups as he increases in level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dragoon ~ Spinning Lance", at_level: 7, description: Some("At 7th level, a dragoon may alternate attacks with the piercing head of his lance with reach, or with the butt end (treat as a club) against adjacent targets. Unlike a double weapon, the masterwork quality and magical special abilities apply to both ends of the lance, except for those weapon special abilities that apply only to edged weapons. This ability replaces armor training 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dragoon ~ Banner", at_level: 9, description: Some("At 9th level, a dragoon may attach a banner to his lance. This is identical to the cavalier class feature (Advanced Player's Guide 34). The bonuses provided by the dragoon's banner increase by +1 for every five levels beyond 9th (to a maximum of +3 at 19th level). Cavalier levels stack with his dragoon level for determining the effect of his banner, and he can take the better banner progression. This ability replaces weapon training 2, 3, and 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dragoon ~ Piercing Lance", at_level: 11, description: Some("At 11th level, as a standard action or as part of a charge, a dragoon attacking a mounted opponent can make two attacks, one against the mount and the other against the rider, using his highest base attack bonus. Furthermore, if the mount is hit and its rider attempts to negate the hit with the Mounted Combat feat, the dragoon's attack roll is considered 4 higher when calculating the DC of the Ride check to negate the hit. This ability replaces armor training 3."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dragoon ~ Leaping Lance", at_level: 15, description: Some("At 15th level, a dragoon and his mount suffer no armor check penalty on Acrobatics checks while mounted. When charging, a dragoon may jump from his mount toward his target. If he jumps 10 feet, his charge modifiers on attack rolls and to AC are doubled and he is still considered mounted for lance damage, mounted combat feats, and so on. This ability replaces armor training 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dragoon ~ Weapon Mastery", at_level: 1, description: Some("The dragoon must select the lance for this ability."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Gladiator -- uc_abilities_class.lst:679
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Gladiator",
            subject: "Fighter",
            archetype_name: "Gladiator",
            description: Some("Most fighters battle out of necessity, with each battle a contest of life and death. Some, however, fight for glory and for the adulation of the crowd. The gladiator is both a cunning warrior and a consummate performer, knowing life and death are balanced not only on a sword's edge, but also on the cheers or jeers of the crowd."),
            source_page: Some("p.47"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ GLADIATOR],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterArmorProficiencies,TYPE.FighterBravery,TYPE.FighterBonusFeats]"]),
            replaces: Some(&["FighterArmorProficiencies", "FighterBravery", "FighterBonusFeats"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Gladiator ~ Skills", at_level: 1, description: Some("A gladiator gains Perform (act, comedy, and dance) (Cha) as class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Gladiator ~ Weapon and Armor Proficiency", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Gladiator ~ Bonus Feats", at_level: 1, description: Some("A gladiator may choose to take combat or performance feats as bonus feats."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Gladiator ~ Fame", at_level: 2, description: Some("At 2nd level, when a gladiator begins a performance combat (see page 153), he always starts with at least 1 victory point. If he already has victory points, he gains 1 extra victory point. At 10th level, the gladiator starts out with at least 2 victory points. If he already has victory points, he gains 2 extra victory points. This ability replaces bravery."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Tactician -- uc_abilities_class.lst:680
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Tactician",
            subject: "Fighter",
            archetype_name: "Tactician",
            description: Some("While many fighters focus on the fundamentals of melee and ranged combat, there are those who are trained to view the bigger picture on the battlefield. These fighters use their training and tactical acumen to overcome challenges that would overwhelm mere brute strength and skill at arms."),
            source_page: Some("p.47"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ TACTICIAN],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterArmorProficiencies,TYPE.FighterBonusFeat1,TYPE.FighterBonusFeats,TYPE.FighterBravery,TYPE.FighterWeaponTraining1,TYPE.FighterArmorTraining3]"]),
            replaces: Some(&["FighterArmorProficiencies", "FighterBonusFeat1", "FighterBravery", "FighterWeaponTraining1", "FighterArmorTraining3", "FighterArmorTraining4", "FighterBonusFeats"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Tactician ~ Weapon and Armor Proficiency", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Tactician ~ Strategic Training", at_level: 1, description: Some("A tactician gains 4 skill points + a number of skill points equal to his Intelligence modifier at each level, instead of the normal 2 skill points + Intelligence modifier at each level. Furthermore, Diplomacy (Cha), Knowledge (geography) (Int), Knowledge (nobility) (Int), Linguistics (Int), and Sense Motive (Wis) are all class skills for the tactician. This ability replaces the bonus fighter combat feat gained at 1st level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Tactician ~ Bonus Feats", at_level: 1, description: Some("A tactician may choose Skill Focus or any teamwork feat, in addition to combat feats, as bonus feats."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Tactician ~ Tactical Awareness", at_level: 2, description: Some("At 2nd level, a tactician gains a +1 bonus on initiative checks. This bonus increases by +1 for every four levels after 2nd level (to a maximum of +5 at 18th level). This ability replaces bravery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Tactician ~ Tactician", at_level: 5, description: Some("At 5th level, a tactician gains this ability as the cavalier class feature (Advanced Player's Guide 33). He may use this ability once per day at 5th level, plus one additional time for every five levels after 5th (to a maximum of four times at 20th level). If the tactician also has cavalier levels, these levels stack for determining the number of uses per day, and he can take the better progression. This ability replaces weapon training 1."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Tactician ~ Cooperative Combatant", at_level: 11, description: Some("At 11th level, when a tactician uses the aid another special attack (Core Rulebook 197), he may affect one additional ally per point of Intelligence bonus. For each ally that a tactician aids, he can pick whether to grant that ally the +2 bonus on its next attack against the opponent or the +2 bonus to AC against the opponent's next attack on that ally, and can grant different allies different bonuses. This ability replaces armor training 3."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Tactician ~ Battle Insight", at_level: 15, description: Some("At 15th level, as a swift action, a tactician can grant his Intelligence modifier as an insight bonus on the attack rolls made by a single ally within line of sight that can both see and hear the tactician. That ally gains the bonus until the end of the tactician's next turn. The tactician can use this ability a number of times per day equal to 3 + his Intelligence modifier. This ability replaces armor training 4."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Thunderstriker -- uc_abilities_class.lst:681
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Thunderstriker",
            subject: "Fighter",
            archetype_name: "Thunderstriker",
            description: Some("The thunderstriker adopts an unusual fighting style, gripping a heavy weapon with both hands and switching to a defensive posture with weapon and buckler, lashing out with the shield with surprising speed and power."),
            source_page: Some("p.48"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ THUNDERSTRIKER],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterArmorTraining1,TYPE.FighterArmorTraining2,TYPE.FighterArmorTraining3,TYPE.FighterArmorTraining4,TYPE.FighterArmorMastery,TYPE.FighterWeaponTraining3,TYPE.FighterWeaponTraining4]"]),
            replaces: Some(&["FighterArmorTraining1", "FighterArmorTraining2", "FighterArmorTraining3", "FighterArmorTraining4", "FighterArmorMastery", "FighterWeaponTraining3", "FighterWeaponTraining4"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Thunderstriker ~ Strapped Shield", at_level: 3, description: Some("At 3rd level, a thunderstriker takes no penalty on attack rolls when using a weapon in two hands while wearing a buckler. This ability replaces armor training 1."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Thunderstriker ~ Hardbuckler", at_level: 7, description: Some("At 7th level, a thunderstriker may make shield bash attacks with a buckler as if it were a light shield. This ability replaces armor training 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Thunderstriker ~ Knockback Smash", at_level: 11, description: Some("At 11th level, when a thunderstriker uses his buckler to attack, he gains a bonus equal to the enhancement bonus of the buckler on both attack and damage rolls. This ability replaces armor training 3."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Thunderstriker ~ Hammer and Anvil", at_level: 13, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Thunderstriker ~ Buckler Defense", at_level: 15, description: Some("At 15th level, a thunderstriker retains partial use of his buckler even when using a weapon in both hands or in each hand (rather than losing his shield bonus until the beginning of his next turn). He gains a +1 shield bonus to AC and may apply the benefits of any feats he knows that require the use of a shield, but does not benefit from any magical properties his shield may possess. This ability replaces armor training 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Thunderstriker ~ Balanced Bashing", at_level: 17, description: Some("At 17th level, a thunderstriker no longer suffers two-weapon fighting penalties when using a buckler as his off-hand weapon. This ability replaces weapon training 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Thunderstriker ~ Improved Buckler Defense", at_level: 19, description: Some("At 19th level, a thunderstriker does not forfeit his shield bonus to AC from a buckler when fighting two-handed. This ability replaces armor mastery."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Tower Shield Specialist -- uc_abilities_class.lst:682
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Tower Shield Specialist",
            subject: "Fighter",
            archetype_name: "Tower Shield Specialist",
            description: Some("Many fighters believe the tower shield is a tool suitable only for troops on the battlefield, claiming it is too large and bulky to use in skirmishes or within dungeon corridors. Tower shield specialists defy those notions, using their massive shields with startling skill and incredible effect. They use these seemingly clumsy shields to perform deft maneuvers that confound their enemies."),
            source_page: Some("p.48"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ TOWER SHIELD SPECIALIST],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterBravery,TYPE.FighterWeaponTraining1,TYPE.FighterWeaponTraining2,TYPE.FighterWeaponTraining3,TYPE.FighterWeaponTraining4,TYPE.WeaponMastery]"]),
            replaces: Some(&["FighterBravery", "FighterWeaponTraining1", "FighterWeaponTraining2", "FighterWeaponTraining3", "FighterWeaponTraining4", "WeaponMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Tower Shield Specialist ~ Burst Barrier", at_level: 2, description: Some("At 2nd level, a tower shield specialist can use his shield to screen himself from burst spells and effects, gaining a +1 bonus on Reflex saves against them while employing a tower shield. This bonus increases by +1 for every four levels after 2nd (to a maximum of +5 at 18th level). This ability replaces bravery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Tower Shield Specialist ~ Tower Shield Training", at_level: 3, description: Some("At 3rd level, a tower shield specialist gains armor training as normal, but while he employs a tower shield, the armor penalty is reduced by 3 and the maximum Dexterity bonus allowed by his armor increases by 2. The benefit increases every four levels thereafter as per standard armor training; if the tower shield specialist is not employing a tower shield, the benefits to armor training revert to the normal bonuses."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Tower Shield Specialist ~ Tower Shield Specialist", at_level: 5, description: Some("At 5th level, when a tower shield specialist employs a tower shield in combat, he does not take the -2 penalty on attack rolls because of the shield's encumbrance. This ability replaces weapon training 1."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Tower Shield Specialist ~ Tower Shield Defense", at_level: 9, description: Some("At 9th level, while using a tower shield, a tower shield specialist gains his shield bonus against touch attacks. This ability replaces weapon training 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Tower Shield Specialist ~ Immediate Repositioning", at_level: 13, description: Some("At 13th level, as an immediate action, a tower shield specialist can reposition his tower shield to another facing, but he cannot use this ability to interrupt an attack. This ability replaces weapon training 3."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Tower Shield Specialist ~ Tower Shield Evasion", at_level: 16, description: Some("At 16th level, while using a tower shield, the tower shield specialist gains evasion, as the rogue class ability. At 20th level, the shield specialist gains improved evasion, as the rogue advanced talent, while using a tower shield. This ability replaces both weapon training 4 and weapon mastery."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Unarmed Fighter -- uc_abilities_class.lst:683
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Unarmed Fighter",
            subject: "Fighter",
            archetype_name: "Unarmed Fighter",
            description: Some("Not all fighters need weapons to make their mark on the world. The unarmed fighter picks up a weapon only rarely, and when he does, he prefers the weapons of the monk. There are even those who mistake them for monks, but these fighters enter the fight without ki, and instead have a tough perseverance that few can rival."),
            source_page: Some("p.48"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ Unarmed Fighter],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterWeaponProficiencies,TYPE.FighterArmorProficiencies,TYPE.FighterBonusFeat1,TYPE.FighterBravery,TYPE.FighterWeaponTraining_ALL,TYPE.FighterArmorTraining_ALL,TYPE.FighterArmorMastery,TYPE.FighterWeaponMastery]"]),
            replaces: Some(&["FighterArmorProficiencies", "FighterBonusFeat1", "FighterBravery", "FighterWeaponTraining_ALL", "FighterArmorTraining_ALL", "FighterArmorMastery", "FighterWeaponMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Armor Proficiency (Light)", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Unarmed Fighter ~ Weapon and Armor Proficiency", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Improved Unarmed Strike", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Unarmed Fighter ~ Harsh Training", at_level: 2, description: Some("You gain a +%1 bonus on saving throws against effects that cause the exhausted, fatigued, or staggered conditions or temporary penalties to ability scores.|UnarmedFighterHarshTrainingBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Unarmed Fighter ~ Tough Guy", at_level: 3, description: Some("You gain DR:%1/- against nonlethal damage or damage taken while he is grappled.|UnarmedFighterToughGuyDR"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Unarmed Fighter ~ Weapon Training", at_level: 5, description: Some("You gain a +%1 bonus on attack and damage rolls with weapons in the monk and natural weapon groups.|UnarmedFighterWeaponTraining"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Unarmed Fighter ~ Clever Wrestler", at_level: 7, description: Some("You take no penalties to Dexterity or on attack rolls while grappled, and retains your Dexterity bonus to AC while pinning an opponent. You can make attacks of opportunity even when grappled and even against creatures attempting to grapple you if the opponent has the Improved Grapple feat or the grab ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Unarmed Fighter ~ Trick Throw", at_level: 8, description: Some("When you successfully trip an opponent with an unarmed attack, you can attempt a dirty trick combat maneuver against that creature (before the opponent becomes prone) as an immediate action that does not provoke attacks of opportunity. This ability replaces the 8th-level bonus feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Unarmed Fighter ~ Takedown", at_level: 12, description: Some("At 12th level, if you succeed on a drag maneuver, you can attempt a trip maneuver against the same target as a swift action that does not provoke attacks of opportunity. At 15th level, you may do so after a successful grapple check. This ability replaces the 12th-level bonus feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Unarmed Fighter ~ Eye Gouge", at_level: 13, description: Some("If you confirm a critical hit with his unarmed strike or begin your turn grappled, you may attempt a dirty trick maneuver to blind his target as a swift action that does not provoke attacks of opportunity. A target more than one size category larger is unaffected."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Unarmed Fighter ~ Sucker Punch", at_level: 17, description: Some("When you strike a creature that is denied its Dexterity bonus to AC, or that you have successfully pinned with a grapple check, you can attempt a dirty trick or trip combat maneuver against that target as a swift action."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Unarmed Fighter ~ Sheer Toughness", at_level: 19, description: Some("You become immune to nonlethal damage and the exhausted, fatigued, and staggered conditions."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Unarmed Fighter ~ Weapon Mastery", at_level: 20, description: Some("Any attacks made a unarmed strikes automatically confirm all critical threats and have their damage multiplier increased by 1."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Unbreakable -- uc_abilities_class.lst:684
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Unbreakable",
            subject: "Fighter",
            archetype_name: "Unbreakable",
            description: Some("The unbreakable is a warrior of indomitable will, unstoppable and implacable once he has set his mind upon a course of action. The unbreakable endures any trial to do what must be done, and when his mind is set toward a goal, nothing can stop him from achieving it, though many may try. He is a juggernaut and a zealot, supremely confident in his own abilities."),
            source_page: Some("p.49"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ UNBREAKABLE],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterTowerShieldProficiency,TYPE.FighterBonusFeat1,TYPE.FighterBravery,TYPE.FighterWeaponTraining1,TYPE.FighterWeaponTraining2,TYPE.FighterWeaponTraining3,TYPE.FighterWeaponTraining4,TYPE.FighterArmorTraining3,TYPE.FighterArmorTraining4]"]),
            replaces: Some(&["FighterTowerShieldProficiency", "FighterBonusFeat1", "FighterBravery", "FighterWeaponTraining1", "FighterWeaponTraining2", "FighterWeaponTraining3", "FighterWeaponTraining4", "FighterArmorTraining3", "FighterArmorTraining4"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Unbreakable ~ Weapon and Armor Proficiency", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Unbreakable ~ Tough as Nails", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Unbreakable ~ Unflinching", at_level: 2, description: Some("At 2nd level, an unbreakable gains a +1 bonus on Will saves against mind-affecting effects. This bonus increases by +1 for every four levels after 2nd level (to a maximum of +5 at 18th level). This ability replaces bravery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Unbreakable ~ Heroic Recovery", at_level: 5, description: Some("At 5th level, an unbreakable gains the Heroic Recovery feat (Advanced Player's Guide 162) as a bonus feat, if he does not have it already. If he already has this feat, the unbreakable can choose any combat feat instead. In addition, he may use this feat one additional time per day for every four levels after 5th (to a maximum of 4 times per day at 17th level). This ability replaces weapon training 1."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Unbreakable ~ Heroic Defiance", at_level: 9, description: Some("At 9th level, an unbreakable gains the Heroic Defiance feat (Advanced Player's Guide 162) as a bonus feat, if he does not have it already. If he already has this feat, the unbreakable can choose any combat feat instead. In addition, he may use this feat one additional time per day for every four levels after 9th (to a maximum of 3 times per day at 19th level). This ability replaces weapon training 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Unbreakable ~ Quick Recovery", at_level: 11, description: Some("At 11th level, an unbreakable needs only 15 minutes of rest or to be subject to a healing spell or effect to recover from the fatigued condition. This ability replaces armor training 3."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Unbreakable ~ Stalwart", at_level: 13, description: Some("At 13th level, when an unbreakable succeeds on a Fortitude or Will save against a spell or spell-like ability that has a partial effect even on a successful save, he is completely unaffected by it. This ability replaces weapon training 3."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Unbreakable ~ Unlimited Endurance", at_level: 15, description: Some("At 15th level, when an unbreakable is exhausted, he only suffers the effects of the fatigued condition instead, but does require 1 hour of rest to reduce this condition to the actual fatigued condition. This ability replaces armor training 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Unbreakable ~ Miraculous Recovery", at_level: 17, description: Some("At 17th level, when an unbreakable makes a saving throw to recover from an ongoing effect, he may roll twice and choose the better roll. This ability replaces weapon training 4."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Unbreakable ~ Unbreakable Mind", at_level: 20, description: Some("At 20th level, an unbreakable becomes nearly impossible to sway with honeyed words or magic. He gains immunity to mind-affecting effects. This ability replaces weapon mastery."), benefit: None },
            ],
        },
        // Inquisitor Archetype ~ Iconoclast -- uc_abilities_class.lst:869
        ArchetypeSwapEntry {
            key: "Inquisitor Archetype ~ Iconoclast",
            subject: "Inquisitor",
            archetype_name: "Iconoclast",
            description: Some("Some magic items are heretical by nature, enabling the unfaithful to spread wickedness. Iconoclasts seek out and remove such crutches, cleansing the taint these items exude."),
            source_page: Some("p.52"),
            prerequisites: Some(&["PRECLASS:1,Inquisitor=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Inquisitor Archetype ~ Iconoclast],[!PREABILITY:1,CATEGORY=Archetype,TYPE.InquisitorMonsterLore,TYPE.InquisitorDetectAlignment,TYPE.InquisitorDiscernLies,TYPE.InquisitorExploitWeakness,TYPE.InquisitorTrueJudgment]"]),
            replaces: Some(&["InquisitorMonsterLore", "InquisitorDetectAlignment", "InquisitorDiscernLies", "InquisitorExploitWeakness", "InquisitorTrueJudgment"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Iconoclast ~ Shake Effects", at_level: 1, description: Some("At 1st level, an iconoclast gains a +2 bonus on all saving throws against effects that come from a magic item. This ability replaces monster lore."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Iconoclast ~ Detect Magic", at_level: 2, description: Some("At 2nd level, an iconoclast gains the ability to detect magic at will. This ability replaces detect alignment."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Iconoclast ~ Dispelling Attack", at_level: 5, description: Some("At 5th level, once per day as a standard action, an iconoclast can make a melee or ranged attack against an opponent, and if she hits, she can affect that opponent as if she had cast dispel magic, using the targeted dispel option. If she misses, this ability is wasted. Use her inquisitor level as the caster level of the dispel magic effect. This ability replaces discern lies."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Iconoclast ~ Negating Critical", at_level: 14, description: Some("At 14th level, when an iconoclast confirms a critical hit, the creature she hit must succeed at a Fortitude save at DC 10 + 1/2 the iconoclast's caster level + the iconoclast's Wisdom modifier for each non-artifact magic item the target is wearing or carrying. On a failed saving throw, the target of the critical hit can neither use nor gain benefit from any minor magic item for 1d4 rounds. This ability can be used in conjunction with critical feats. This ability replaces exploit weakness."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Iconoclast ~ Destroy Artifact", at_level: 20, description: Some("At 20th level, an iconoclast has the ability to destroy minor artifacts. The iconoclast must have possession of the minor artifact for at least a week, which she must spend in uninterrupted contemplation of the nature of the artifact. At the end of that week, she makes a DC 30 Spellcraft check to unweave the fabric of the item, and if she is successful, the artifact is destroyed. If the iconoclast fails, she cannot try to destroy the artifact again with this ability, but another iconoclast can. This ability replaces true judgment."), benefit: None },
            ],
        },
        // Inquisitor Archetype ~ Spellbreaker -- uc_abilities_class.lst:870
        ArchetypeSwapEntry {
            key: "Inquisitor Archetype ~ Spellbreaker",
            subject: "Inquisitor",
            archetype_name: "Spellbreaker",
            description: Some("The world is full of dangerous magic, and many recoil in the face of such power. The spellbreaker, by contrast, learns to recognize and resist certain types of magic, wading through waves of magic to reach her foes."),
            source_page: Some("p.52"),
            prerequisites: Some(&["PRECLASS:1,Inquisitor=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Inquisitor Archetype ~ Spellbreaker],[!PREABILITY:1,CATEGORY=Archetype,TYPE.InquisitorMonsterLore,TYPE.InquisitorBonusTeamworkFeat,TYPE.InquisitorSoloTactics,TYPE.InquisitorFinalJudgment]"]),
            replaces: Some(&["InquisitorMonsterLore", "InquisitorTeamworkFeat", "InquisitorSoloTactics", "InquisitorFinalJudgment"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Spellbreaker ~ Strong-Willed", at_level: 1, description: Some("At 1st level, a spellbreaker is able to stand strong against magical effects that seek to control, compel, or persuade her. The spellbreaker rolls twice and takes the best result when making a Will saving throw against a mind-affecting effect. This ability replaces monster lore."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spellbreaker ~ Defense against Magic", at_level: 3, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Spellbreaker ~ Foil Casting", at_level: 3, description: Some("At 3rd level, when an opponent tries to cast an arcane spell within a spellbreaker's threatened area, the DC for that caster to cast defensively increases by 2. This increase stacks with the effects of the Disruptive feat (see page 112 of the Core Rulebook). Furthermore, the spellbreaker knows where to hit foes to foil casting from a distance. Each time she hits an arcane spellcaster or a creature that uses spell-like abilities with a ranged weapon attack, the DC of any Concentration checks the caster makes increases by 2 for 1 round. This ability replaces solo tactics."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spellbreaker ~ Impervious", at_level: 20, description: Some("At 20th level, a spellbreaker becomes immune to the effects of a single school of arcane magic. That school of magic must be the first one she picked for defense against magic (see above). Neither harmful nor helpful arcane spells of that school have an effect on the spellbreaker. If a spell of that school is an area of effect spell, the spell goes off as normal, but the spellbreaker is untouched by its effects. Once per day, as a swift action, the spellbreaker can grant this imperviousness to all allies in a 60-foot burst for 1 minute. This ability replaces final judgment."), benefit: None },
            ],
        },
        // Inquisitor Archetype ~ Witch Hunter -- uc_abilities_class.lst:871
        ArchetypeSwapEntry {
            key: "Inquisitor Archetype ~ Witch Hunter",
            subject: "Inquisitor",
            archetype_name: "Witch Hunter",
            description: Some("When pursuing justice for their faith, inquisitors sometimes hunt sorcerers, witches, wizards, and other practitioners of arcane magic-but especially witches, since their devotion to a patron is often seen as suspect by many religions."),
            source_page: Some("p.53"),
            prerequisites: Some(&["PRECLASS:1,Inquisitor=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Inquisitor Archetype ~ Witch Hunter],[!PREABILITY:1,CATEGORY=Archetype,TYPE.InquisitorMonsterLore,TYPE.InquisitorDetectAlignment,TYPE.InquisitorDiscernLies,TYPE.InquisitorTrueJudgment,TYPE.InquisitorTrack,TYPE.InquisitorExploitWeakness]"]),
            replaces: Some(&["InquisitorMonsterLore", "InquisitorDetectAlignment", "InquisitorDiscernLies", "InquisitorTrueJudgment", "InquisitorTrack", "InquisitorExploitWeakness"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Witch Hunter ~ Spell Sage", at_level: 1, description: Some("At 1st level, a witch hunter adds her Wisdom modifier on Spellcraft checks in addition to her Intelligence modifier when attempting to identify a spell as it is being cast, to identify the properties of a magic item using detect magic, or to decipher a scroll. This ability replaces monster lore."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Witch Hunter ~ Knowledgeable Defense", at_level: 2, description: Some("At 2nd level, a witch hunter who identifies a spell with Spellcraft gains a bonus against its effects, either a +1 bonus on saving throws or a +1 dodge bonus to AC against this spell. This bonus increases every four levels (to a maximum of +5 at 18th level). At 20th level, once per day as a swift action, she can grant this bonus to all her allies within 60 feet, for 1 minute. The allies gain the bonus when the witch hunter identifies the spell, and those among the allies who have at least 1 rank in Spellcraft can aid the witch hunter's next Spellcraft check as a swift action while under the effect of this ability. This ability replaces detect alignment, discern lies, and true judgment."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Witch Hunter ~ Spell Scent", at_level: 6, description: Some("At 6th level, the witch hunter learns how to sense unique spell signatures, and can follow the trail of a cast spell or spell effect cast back to its source. Once per day, when the witch hunter comes into contact with or is exposed to a spell effect or a magic item, she can spend a full-round action to examine the spell effect or magic item, and then gains the effect of a locate creature spell (Core Rulebook 305). She can do this even if the spell effect was instantaneous, but must start her examination within 1 round after she experienced the spell's effect. Instead of locating a known creature, the witch hunter locates the caster of the spell effect or magic item she examined. Doing so does not allow the witch hunter to gain any intelligence about the spellcaster other than its location. This ability replaces track."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Witch Hunter ~ Witch's Bane Judgment", at_level: 14, description: Some("At 14th level, a witch hunter gains the following judgment."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Witch Hunter ~ Witch's Bane", at_level: 1, description: Some("Any creature that casts an arcane spell within 30 feet of the witch hunter takes a -2 penalty to AC against her attacks and a -2 penalty on saving throws against her spells while this judgment remains in effect. This ability replaces exploit weakness."), benefit: None },
            ],
        },
        // Paladin Archetype ~ Divine Hunter -- uc_abilities_class.lst:1440
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Divine Hunter",
            subject: "Paladin",
            archetype_name: "Divine Hunter",
            description: Some("Most paladins rush into battle, meeting evil toe-to-toe. The divine hunter prefers to engage evil from afar, striking down her foes before they can threaten her allies."),
            source_page: Some("p.62"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ DIVINE HUNTER],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinArmorProficiencyHeavy,TYPE.PaladinAuraOfCourage,TYPE.PaladinDivineBond,TYPE.PaladinMercy6,TYPE.PaladinAuraOfResolve,TYPE.PaladinAuraOfFaith,TYPE.PaladinAuraOfJustice]"]),
            replaces: Some(&["PaladinArmorProficiencyHeavy", "PaladinAuraOfCourage", "PaladinDivineBond", "PaladinMercy6", "PaladinAuraOfResolve", "PaladinAuraOfFaith", "PaladinAuraOfJustice"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Divine Hunter ~ Precise Shot", at_level: 1, description: Some("A divine hunter gains Precise Shot as a bonus feat at 1st level, even if she doesn't meet the prerequisites. This ability replaces her Heavy Armor Proficiency."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Divine Hunter ~ Shared Precision", at_level: 3, description: Some("At 3rd level, when a divine hunter hits a creature with a ranged attack, she grants her allies within 10 feet of her the benefit of the Precise Shot feat against that target until the start of her next turn. Her allies must remain within 10 feet of her, and must be able both to see and hear the divine hunter to gain this benefit. This ability replaces aura of courage."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Divine Hunter ~ Divine Bond", at_level: 5, description: Some("At 5th level, a divine hunter forms a bond with her deity. This functions as the paladin's divine bond ability, except the bond must always take the form of a ranged or throwing weapon (excluding ammunition). In addition to the listed abilities, a divine hunter can add the distance, returning, or seeking special abilities to her weapon, but she cannot add the defending or disruption special abilities. Special abilities added to throwing weapons function normally when the weapon is used in melee. This ability replaces the standard paladin's divine bond."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Divine Hunter ~ Distant Mercy", at_level: 6, description: Some("At 6th level, a divine hunter can expend two uses of her lay on hand abilities to use her lay on hands ability on a target within 5 feet per paladin level. Distant mercy has no effect on creatures harmed by positive energy, such as undead. This ability replaces the paladin's 6th-level mercy."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Divine Hunter ~ Aura of Care", at_level: 8, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Divine Hunter ~ Hunter's Blessing", at_level: 11, description: Some("At 11th level, a divine hunter can expend a use of her smite evil ability as a swift action to grant herself and all allies within 10 feet the Deadly Aim, Precise Shot, and Improved Precise Shot feats, even if they lack the prerequisites. The effects last for 1 minute. Evil creatures gain no benefit from this ability. This ability replaces aura of justice."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Divine Hunter ~ Righteous Hunter", at_level: 14, description: Some("At 14th level, a divine hunter's ranged weapons are treated as good-aligned for the purposes of overcoming damage reduction. Any ranged attacks made by an ally within 10 feet of her are likewise treated as good-aligned for the purposes of overcoming damage reduction. This ability functions only while the divine hunter is conscious. This ability replaces aura of faith."), benefit: None },
            ],
        },
        // Paladin Archetype ~ Empyreal Knight -- uc_abilities_class.lst:1441
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Empyreal Knight",
            subject: "Paladin",
            archetype_name: "Empyreal Knight",
            description: Some("The empyreal knight dedicates her life to serving the celestial beings that guide mortals in their struggle toward the light."),
            source_page: Some("p.62"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ EMPYREAL KNIGHT],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinDivineGrace,TYPE.PaladinMercy,TYPE.PaladinLayOnHands,TYPE.PaladinChannelPositiveEnergy,TYPE.PaladinHolyChampion]"]),
            replaces: Some(&["PaladinDivineBond", "PaladinDivineGrace", "PaladinMercy", "PaladinLayOnHands", "PaladinChannelPositiveEnergy", "PaladinHolyChampion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Empyreal Knight ~ Voices of the Spheres", at_level: 2, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Empyreal Knight ~ Celestial Heart", at_level: 3, description: Some("As an empyreal knight advances, her ties with the celestial realms grant her a portion of the abilities and defenses of true angels. At 3rd level, she gains resistance 5 against acid, cold, and electricity. At 6th level, she gains a +4 racial bonus on saves against poison. At 9th level, her defenses improve to resistance 10 against acid, cold, and electricity. At 12th level, she gains immunity to petrification. At 15th level, she gains truespeech, the ability to speak with any creature that has a language as though using a tongues spell. This ability functions with a caster level equal to her character level, and is always active. At 18th level, as a swift action, the empyreal knight can manifest a protective aura against evil. Against abilities or effects created by evil creatures, this ability provides a +4 deflection bonus to AC and a +4 resistance bonus on saving throws to anyone within 20 feet of her. It otherwise functions as a magic circle against evil. She can use this ability for a maximum of 1 round per day per paladin level. These rounds do not need to be consecutive. These abilities replace mercy."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Empyreal Knight ~ Celestial Ally", at_level: 4, description: Some("At 4th level, an empyreal knight can summon a celestial ally as a full-round action. This functions as summon monster I, except it can only be used to summon celestial creatures, archons, and angels. At 6th level, this improves to summon monster II, increasing by one spell level for every two levels thereafter, to a maximum of summon monster IX at 20th level. This ability can be used a number of times per day equal to the empyreal knight's Charisma modifier (minimum 1), but only one ally can be summoned at a time. This ability replaces lay on hands and channel positive energy."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Empyreal Knight ~ Divine Bond", at_level: 5, description: Some("At 5th level, an empyreal knight forms a bond with a mount, as the standard paladin ability. Her mount gains the celestial template at 8th level. At 12th level, her mount sprouts wings if it cannot already fly, and gains a fly speed of twice its land speed and good maneuverability. If the mount could already fly, its fly speed and maneuverability improve to at least this level. This ability otherwise functions as the paladin ability of the same name."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Empyreal Knight ~ Empyreal Champion", at_level: 20, description: Some("At 20th level, an empyreal knight transcends her mortal self. Her DR increases to 10/evil. Her type is treated as outsider for the purposes of spells and magical effects. She gains darkvision 60 feet and low-light vision. As a standard action and a supernatural ability, she can sprout wings that allow her to fly at twice her land speed with average maneuverability. Any armor or clothing reshapes to allow her to fly when she uses this ability. She can retract the wings as a free action. Unlike other outsiders, an empyreal knight can still be brought back from the dead as if she were a member of her previous creature type. This ability replaces holy champion."), benefit: None },
            ],
        },
        // Paladin Archetype ~ Holy Gun -- uc_abilities_class.lst:1442
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Holy Gun",
            subject: "Paladin",
            archetype_name: "Holy Gun",
            description: Some("Not all paladins are knights in shining armor. Holy guns roam the world searching for evil. And where they find it, they put it down."),
            source_page: Some("p.63"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ HOLY GUN],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinDetectEvil,TYPE.PaladinSmiteEvil,TYPE.PaladinDivineBond,TYPE.PaladinArmorProficiencies,TYPE.PaladinWeaponProficiencies]"]),
            replaces: Some(&["PaladinDetectEvil", "PaladinSmiteEvil", "PaladinDivineBond", "PaladinArmorProficiencies", "PaladinWeaponProficiencies"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Holy Gun ~ Weapon and Armor Proficiency", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Holy Gun ~ Have Gun", at_level: 1, description: Some("At 1st level, the holy gun gains the Amateur Gunslinger feat and Gunsmithing as a bonus feat. She also gains a battered gun identical to the one gained by the gunslinger. This ability replaces detect evil."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Holy Gun ~ Divine Deed", at_level: 2, description: Some("At 2nd level, the holy gun gains the following deed. This deed works and interacts with grit the same way as gunslinger deeds, but only the holy gun can use it. If the holy gun also has levels in gunslinger, she can spend grit points from that class to use this deed."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Holy Gun ~ Smiting Shot", at_level: 1, description: Some("A holy gun can spend 1 grit point to make a smiting shot with a firearm attack as a standard action. If the target is evil, the holy gun adds her Charisma bonus and her paladin level to the damage of the firearm attack. If the target of the smiting shot is an outsider with the evil subtype, an evil-aligned dragon, or an undead creature, the bonus to damage increases to the Charisma modifier plus 2 points of damage per level the paladin possess. Regardless of the target, smiting shot automatically bypasses any DR the creature might have. This ability replaces smite evil."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Holy Gun ~ Divine Bond", at_level: 5, description: Some("At 5th level, a holy gun forms a bond with her deity. This functions as the paladin's divine bond ability, except the bond must always take the form of a firearm. In addition to the listed abilities, a holy gun can add the distance, reliable, or seeking special abilities to her weapon, but she cannot add the defending or disruption special abilities. This ability replaces the standard paladin's divine bond."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Holy Gun ~ Holy Grit", at_level: 11, description: Some("At 11th level, a holy gun gains a number of grit points equal to her Charisma modifier (minimum 1) and gains the use of a single gunslinger deed. She can select any deed that a gunslinger of her paladin level -4 could use. At 14th level, and every three levels beyond 14th, the holy gun gains another point of grit and another gunslinger deed that a gunslinger of her level -4 could use. If she already has levels in gunslinger, she gains a bonus to the maximum amount of grit she can have each day, equal to her Charisma bonus (if any) but gains no extra grit as the start of each day."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Holy Gun ~ Holy Slinger", at_level: 1, description: Some("This ability function like the holy champion paladin class feature, but the banishment occurs when she hits an evil outsider with the smiting shot deed."), benefit: None },
            ],
        },
        // Paladin Archetype ~ Holy Tactician -- uc_abilities_class.lst:1443
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Holy Tactician",
            subject: "Paladin",
            archetype_name: "Holy Tactician",
            description: Some("The holy tactician inspires her allies on the field of battle. Her place is at their side against overwhelming odds, and her guidance brings out their true potential."),
            source_page: Some("p.64"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ HOLY TACTICIAN],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinSmiteEvil,TYPE.PaladinDivineHealth,TYPE.PaladinDivineBond,TYPE.PaladinHolyChampion,TYPE.PaladinAuraOfJustice,TYPE.PaladinAuraOfResolve,TYPE.PaladinAuraOfCourage]"]),
            replaces: Some(&["PaladinSmiteEvil", "PaladinDivineHealth", "PaladinDivineBond", "PaladinHolyChampion", "PaladinAuraOfJustice", "PaladinAuraOfResolve", "PaladinAuraOfCourage"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Holy Tactician ~ Weal's Champion", at_level: 1, description: Some("Once per day as swift action, a holy tactician can call on the powers of good to aid her against evil. Against evil targets, the holy tactician gains her Charisma bonus (if any) on her attack rolls against evil creatures, and if she hits, she adds 1/2 her paladin level on her weapon damage rolls as well. These bonuses last for 1 round for every two paladin levels the holy tactician has attained (minimum 1 round). In addition, for 1 round after the holy tactician successfully strikes an evil creature, all non-evil allies within 30 feet of her gain a competence bonus on attack rolls equal to 1/2 her Charisma bonus against that creature as well as a +1 competence bonus on damage rolls. The bonus on damage rolls increases by +1 for every five levels the holy tactician attains (to a maximum of +5 at 20th level). She can grant this bonus against more than one creature at a time. To gain this benefit, the holy tactician's allies must be able to see or hear her, and she must be conscious. At 4th level, and every three levels thereafter, the holy tactician may use weal's champion one additional time per day (to a maximum of seven times per day at 19th level). This ability replaces smite evil."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Holy Tactician ~ Tactical Acumen", at_level: 3, description: Some("At 3rd level, a holy tactician gains a teamwork feat as a bonus feat. She must meet the prerequisites for this feat. She gains an additional bonus feat for every four levels attained after 3rd, to a maximum of five bonus feats at 19th level. This ability replaces divine health and divine bond."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Holy Tactician ~ Battlefield Presence", at_level: 3, description: Some("At 3rd level, a tactician can direct her allies in battle, granting each ally within 30 feet one teamwork feat she possesses as a bonus feat as a standard action. All allies must receive the same feat, but do not need to meet the prerequisites of this bonus feat. This ability does not function if the paladin is flat-footed or unconscious. Allies must be able to see and hear the holy tactician in order to gain this benefit. Changing the bonus feat granted is a swift action. This ability replaces aura of courage."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Holy Tactician ~ Guide the Battle", at_level: 8, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Holy Tactician ~ Weal's Wrath", at_level: 11, description: Some("At 11th level, a tactician can expend two uses of her weal's champion ability at once to enhance its effects. This functions as weal's champion, except the bonus she provides to her allies lasts until each creature she strikes is slain, her weal's champion ability expires, or the paladin herself is slain or knocked unconscious, whichever happens first. This ability replaces aura of justice."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Holy Tactician ~ Masterful Presence", at_level: 20, description: Some("At 20th level, a tactician gains the ability to grant a different bonus feat to each ally affected by her battlefield presence. In addition, all critical threats made by her and her allies against creatures affected by her weal's champion ability are automatically confirmed (no confirmation roll is needed). This ability replaces holy champion."), benefit: None },
            ],
        },
        // Paladin Archetype ~ Knight Of The Sepulcher -- uc_abilities_class.lst:1446
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Knight Of The Sepulcher",
            subject: "Paladin",
            archetype_name: "Knight Of The Sepulcher",
            description: Some("Not content with the antipaladin's mere corruption of the soul, the knight of the sepulcher sacrifices mortality along with morality. The knight of the sepulcher archetype is available only to the antipaladin alternate class."),
            source_page: Some("p.63"),
            prerequisites: Some(&["PRECLASS:1,Antipaladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ KNIGHT OF THE SEPULCHER],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinCruelty,TYPE.PaladinSmiteGoodUse10,TYPE.PaladinAuraOfDespair,TYPE.PaladinFiendishBoonAbility,TYPE.PaladinAuraOfVengeance,TYPE.PaladinAuraOfSin,TYPE.PaladinUnholyChampion]"]),
            replaces: Some(&["PaladinCruelty", "PaladinSmiteGoodUse10", "PaladinAuraOfDespair", "PaladinFiendishBoonAbility", "PaladinAuraOfVengeance", "PaladinAuraOfSin", "PaladinUnholyChampion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Knight Of The Sepulcher ~ Touch of the Crypt", at_level: 5, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Knight Of The Sepulcher ~ Fortitude of the Crypt", at_level: 8, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Knight Of The Sepulcher ~ Cloak of the Crypt", at_level: 10, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Knight Of The Sepulcher ~ Will of the Crypt", at_level: 11, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Knight Of The Sepulcher ~ Weapons of Sin", at_level: 14, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Knight Of The Sepulcher ~ Crypt Lord", at_level: 15, description: Some("At 15th level, a knight of the sepulcher's chance of ignoring critical hits and sneak attacks increases to 75%%, as though he were wearing armor of heavy fortification. He gains immunity to death effects, paralysis, sleep effects, and stunning. He no longer sleeps. The knight of the sepulcher also gains immunity to effects that cause fatigue, and effects that would cause him to become exhausted instead cause him to become fatigued. This ability replaces cruelty."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Knight Of The Sepulcher ~ Soul of the Crypt", at_level: 17, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Knight Of The Sepulcher ~ Undying Champion", at_level: 20, description: Some("At 20th level, a knight of the sepulcher joins the ranks of the undead. His DR increases to 10/bludgeoning and good. His type changes to undead, and he acquires all undead traits. Although immune to disease, he can still carry and spread diseases with the antipaladin's plague bringer ability. The undying champion no longer has a Constitution score. He uses his Charisma score for calculating hit points, Fortitude saves, and any special abilities that rely on Constitution. This ability replaces unholy champion."), benefit: None },
            ],
        },
        // Paladin Archetype ~ Sacred Shield -- uc_abilities_class.lst:1444
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Sacred Shield",
            subject: "Paladin",
            archetype_name: "Sacred Shield",
            description: Some("When faced by evil, the sacred shield reaches first not for a weapon, but for her trusty shield. With her faith, she can ward others from harm."),
            source_page: Some("p.65"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ SACRED SHIELD],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinHolyChampion,TYPE.PaladinAuraOfJustice,TYPE.PaladinChannelPositiveEnergy,TYPE.PaladinSmiteEvil]"]),
            replaces: Some(&["PaladinHolyChampion", "PaladinAuraOfJustice", "PaladinChannelPositiveEnergy", "PaladinSmiteEvil"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Sacred Shield ~ Bastion of Good", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Sacred Shield ~ Holy Shield", at_level: 4, description: Some("At 4th level, a sacred shield can channel her faith into her shield, protecting any nearby allies. All allies adjacent to the paladin gain a shield bonus equal to the sacred shield's own shield bonus, including any increase from the shield's enhancement bonus. This bonus does not stack with any existing shield bonuses. The paladin herself radiates light as a light spell while the shielding is active. At 11th level, this protection expands to cover any allies within 10 feet and the radiance increases to the effects of a daylight spell. At 20th level, any allies within 20 feet are protected. Using this ability consumes two uses of the sacred shield's lay on hands ability, and the effects last for 3 rounds plus a number of rounds equal to her Charisma bonus (if any). This ability replaces channel positive energy."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sacred Shield ~ Divine Bond", at_level: 5, description: Some("At 5th level, instead of forming a divine bond with her weapon or a mount, a sacred shield forms a bond with her shield. As a standard action, a sacred shield can enhance her shield by calling on the aid of a celestial spirit. This bond lasts for 1 minute per paladin level. When called, the spirit causes the shield to shed light like a torch. At 5th level, the spirit grants the shield a +1 enhancement bonus. For every three levels beyond 5th, the shield gains another +1 enhancement bonus, to a maximum of +6 at 20th level. These bonuses can be added to the shield, stacking with existing enhancement bonuses to a maximum of +5, or they can be used to add any of the following armor special abilities - arrow deflection, bashing, blinding, fortification (any), reflecting, and spell resistance (any). The reflecting enhancement may be used once each time the sacred shield makes use of her divine bond. Adding these armor special abilities consumes an amount of bonus equal to the property's cost (see Table 15-4 on page 485 of the Core Rulebook). These bonuses are added to any properties the shield already has, but duplicate special abilities do not grant any extra benefit. If the shield is not magical, at least a +1 enhancement bonus must be added before any other special abilities can be added. The bonus and special abilities granted by the spirit are determined when the spirit is called and cannot be changed until the spirit is called again. The celestial spirit imparts no bonuses if the shield is used by anyone other than the sacred shield, but it resumes giving bonuses if the sacred shield resumes using the shield. A sacred shield can use this ability once per day at 5th level, and one additional time per day for every four levels beyond 5th, to a maximum of four times per day at 17th level. If a shield with a celestial spirit is destroyed, the sacred shield loses the use of this ability for 30 days, or until she gains a level, whichever comes first. During this period, the sacred shield takes a -1 penalty to her armor class and on saving throws."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sacred Shield ~ Improved Bastion", at_level: 11, description: Some("At 11th level, the radius of a sacred shield's bastion of good ability increases to 20 feet. This ability replaces the paladin's aura of justice."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sacred Shield ~ Perfect Bastion", at_level: 20, description: Some("At 20th level, a sacred shield and her allies within 20 feet gain regeneration 10 against the target of her bastion of good ability (essentially regeneration that is overcome by any damage not caused by the target). This ability replaces the sacred shield's holy champion ability."), benefit: None },
            ],
        },
        // Ranger Archetype ~ Battle Scout -- uc_abilities_class.lst:1511
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Battle Scout",
            subject: "Ranger",
            archetype_name: "Battle Scout",
            description: Some("Armies need clean water, safe routes, and intelligence about their enemies. Though regular outdoorsmen might suffice in many cases, some large forces look to battle scouts to keep the body of their troops safe and prepared for the terrain and whatever dangers that terrain hides."),
            source_page: Some("p.66"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ BATTLE SCOUT],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerFavoredEnemy2,TYPE.RangerFavoredEnemy3,TYPE.RangerFavoredEnemy4,TYPE.RangerMasterHunter,TYPE.RangerAnimalCompanion]"]),
            replaces: Some(&["RangerFavoredEnemy2", "RangerFavoredEnemy3", "RangerFavoredEnemy4", "RangerMasterHunter", "RangerAnimalCompanion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Battle Scout ~ Hunter's Bond", at_level: 4, description: Some("At 4th level, when a battle scout gains the hunter's bond ability, he must choose to bond with his companions. Battle scouts do not have animal companions."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Battle Scout ~ Advantageous Terrain", at_level: 5, description: Some("At 5th level, when a battle scout is within one of his favorite terrains, he can spend up to 3 consecutive rounds studying an area of terrain to gain the benefits below. Each round of studying terrain is a standard action. During the first round of studying terrain, the battle scout designates a 60-foot-radius-burst area from a single square within line of sight. Depending on how many rounds the battle scout spends studying the area, he or his allies gain the following benefits for 10 minutes per ranger level he possesses. (For example, if he spends 2 rounds, he or his allies gain the first two benefits.) 1st Round - Allies within line of sight and that can hear the battle scout gain a +2 bonus on initiative checks while within the advantageous terrain area. 2nd Round - Allies within line of sight and that can hear the battle scout gain a +2 on Perception, Stealth, and Survival checks while within the advantageous terrain area. 3rd Round - The battle scout's movement is not hampered by difficult terrain while within the advantageous terrain area. Furthermore, the battle scout can take 10 on Climb and Swim checks and checks to jump while within the area, even when in immediate danger or distracted. This ability replaces the ranger's second favored enemy."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Battle Scout ~ Infiltration", at_level: 10, description: Some("At 10th level, once per day, a battle scout can choose a single terrain type that is not his favored terrain. For the next hour per ranger level, he treats that terrain as if it were one of his favored terrains. This ability replaces the ranger's third favorite enemy."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Battle Scout ~ Superior Tactics", at_level: 15, description: Some("At 15th level, once per day, after a battle scout and his allies roll initiative, the battle scout can arrange his and his allies' initiative rolls any way he wishes. If he has already taken 3 rounds to scout out the terrain with his advantageous terrain ability and both he and his allies are within the effect area, they gain a +2 bonus to initiative checks. Using this ability is not an action. This ability replaces the ranger's fourth favored enemy."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Battle Scout ~ Perfect Advantage", at_level: 20, description: Some("At 20th level, when using his advantageous terrain ability, a battle scout needs only to study the terrain as a standard action to gain all of the benefits. Furthermore, the area of his advantageous terrain increases to a 1-mile-radius spread from the single square within line of sight. This ability replaces master hunter."), benefit: None },
            ],
        },
        // Ranger Archetype ~ Deep Walker -- uc_abilities_class.lst:1512
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Deep Walker",
            subject: "Ranger",
            archetype_name: "Deep Walker",
            description: Some("Some rangers devote their lives to the woods, becoming hunters, protectors, and wilderness guides among its diverse terrain types. The deep walker is instead a master of the strange terrain of the deep earth, exploring its caverns and caves, becoming a master of the dark corridors most surface dwellers fear to explore-and perhaps the reason they fear to explore them."),
            source_page: Some("p.66"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ DEEP WALKER],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerCamouflage,TYPE.RangerWoodlandStride,TYPE.RangerFavoredTerrain,TYPE.RangerHideInPlainSight]"]),
            replaces: Some(&["RangerCamouflage", "RangerWoodlandStride", "RangerFavoredTerrain", "RangerHideInPlainSight"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Deep Walker ~ Deep Knowledge", at_level: 3, description: Some("At 3rd level, a deep walker gains a +2 bonus on initiative checks and Knowledge (dungeoneering), Perception, Stealth, and Survival skill checks while underground (in caves and dungeons). Every 5 levels thereafter, the deep walker gains an additional +3 on each of those checks while underground (to a maximum of +11 at 18th level). This ability replaces favored terrain. The current bonus is %1.|DeepWalkerDeepKnowledgeBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deep Walker ~ Rock Hopper", at_level: 7, description: Some("At 7th level, a deep walker instinctively grasps for the most stable surfaces and holds on tighter while moving over underground rock and stone. The deep walker gains a +5 bonus on all Acrobatics and Climb checks made to traverse underground terrain. Furthermore, the deep walker ignores difficult terrain created by underground terrain and subterranean flora. This ability replaces woodland stride."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deep Walker ~ Deep Walker Camouflage", at_level: 12, description: Some("At 12th level, a deep walker can use the Stealth skill to hide in underground environments, even if the terrain does not grant cover or concealment. This ability replaces camouflage."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deep Walker ~ One with the Stone", at_level: 17, description: None, benefit: None },
            ],
        },
        // Ranger Archetype ~ Falconer -- uc_abilities_class.lst:1513
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Falconer",
            subject: "Ranger",
            archetype_name: "Falconer",
            description: Some("Rangers have always enjoyed a special bond with a specific animal, but the falconer takes this bond to a deeper level. Falconers begin with their companion earlier than other rangers, and have the ability to teach their companions special tricks."),
            source_page: Some("p.67"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ FALCONER],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerCombatStyleFeat6,TYPE.RangerWildEmpathy]"]),
            replaces: Some(&["RangerCombatStyleFeat6", "RangerWildEmpathy"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Falconer ~ Feathered Companion", at_level: 1, description: Some("At 1st level, a falconer earns the trust and companionship of a bird of prey. The bird can be of any type of large hunting or scavenging bird (even a vulture). This ability functions like the druid animal companion ability (which is part of the nature bond class feature), but the falconer must take the bird animal companion, and that companion has only half the normal hit points. The falconer cannot teach the bird of prey the work trick, but can teach it either the roam or distract trick for free. Whichever trick the falconer does not pick then can be picked as a trick later. Roam (DC 15) - The falconer can let his animal companion loose to roam and forage. He must let it roam for no more than a week. When the falconer lets it roam, it agrees to return to the place he let it loose within the time period he designates. Distract (DC 20; bird only) - The animal companion flutters wildly around any enemy it would normally attack with the attack trick. It makes an attack roll against that enemy. On a hit, the enemy is shaken for 1 round. This ability replaces wild empathy."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Falconer ~ Hunter's Bond", at_level: 4, description: Some("At 4th level, a falconer must select an animal companion when he gains hunter's bond. He does not gain a new companion at 4th level; rather, his feathered companion gains full hit points."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Falconer ~ Swoop for the Kill", at_level: 6, description: None, benefit: None },
            ],
        },
        // Ranger Archetype ~ Trophy Hunter -- uc_abilities_class.lst:1514
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Trophy Hunter",
            subject: "Ranger",
            archetype_name: "Trophy Hunter",
            description: Some("Some rangers have taken up the mysteries of black powder in order to become big game hunters. Finding firearms useful for taking down large and dangerous prey, they enter the fight with the crack of black powder fire, often ranging far and wide in their safaris and searches for ever-more-dangerous denizens of the wild to track, study, appreciate-and kill. While this might strike some as going against the ranger's usual respect for nature and the creatures of the wild, trophy hunters understand that they are just more pieces in nature's grand puzzle, and that the laws of the jungle-the right to kill and feed on those weaker than you, and the need to establish dominance through cunning and bloodshed-are both their rights and their duty as the current top of the food chain."),
            source_page: Some("p.67"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ TROPHY HUNTER],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerCombatStyle,TYPE.RangerWildEmpathy,TYPE.RangerHuntersBond]"]),
            replaces: Some(&["RangerCombatStyle", "RangerWildEmpathy", "RangerHuntersBond"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Trophy Hunter ~ Improved Tracking", at_level: 1, description: Some("At 1st level, a trophy hunter gains a +2 bonus on Survival skill checks when following or identifying tracks. When he tracks, he can also attempt a Knowledge (nature) check at DC 15. On a success, the trophy hunter can discern the type and condition of any animals or magical beasts he tracks. By studying their tracks, the trophy hunter is able to identify a rough approximation of their health, maneuverability, and their general behavior as compared to the norm. This ability replaces wild empathy."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Trophy Hunter ~ Firearm Style", at_level: 2, description: Some("At 2nd level, a trophy hunter gains the Amateur Gunslinger feat and Exotic Weapon Proficiency (Firearms), and can use any 1st-level gunslinger deed (page 10). At every four levels thereafter, the trophy hunter can take a grit feat or select a gunslinger deed of his level or lower, ranger's choice. This ability replaces all combat style feats. [Not fully Implemented]"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Trophy Hunter ~ Hunter's Aim", at_level: 4, description: Some("At 4th level, a trophy hunter gains a specific understanding of the weaknesses and vulnerabilities of his favored enemies, and his careful study of these enemies reveals the best way to hurt them. When the trophy hunter makes a firearm attack against a favored enemy, he can target touch AC in the first two range increments of his firearm. This ability stacks with other effects that increase the range increments to target touch AC, adding one range increment to the effect. This ability replaces hunter's bond."), benefit: None },
            ],
        },
        // Ranger Archetype ~ Warden -- uc_abilities_class.lst:1515
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Warden",
            subject: "Ranger",
            archetype_name: "Warden",
            description: Some("All rangers have a bond with the wilderness, but the warden may have the strongest. This protector sits guard in the middle of the wilderness, keeping a lookout for any dangers that might spill from the deeper wilderness beyond, and protecting civilization from the savagery of nature-and vice versa. His long weeks, months, and sometimes years of isolation cause him to \"hear\" what the land is saying. Not all are crazy."),
            source_page: Some("p.68"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ WARDEN],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerFavoredEnemy,TYPE.RangerCombatStyle,TYPE.RangerHuntersBond]"]),
            replaces: Some(&["RangerFavoredEnemy", "RangerCombatStyle", "RangerHuntersBond"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Warden ~ Master of Terrain", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Warden ~ Live in Comfort", at_level: 2, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Warden ~ Terrain Bond", at_level: 4, description: Some("At 4th level, a warden forms a bond with the land itself, enabling him to direct others in such terrain. When in his favored terrain, the warden grants all allies within line of sight and that can hear him a +2 bonus on initiative checks and Perception, Stealth, and Survival skill checks. Also, as long as they travel with him, the warden's allies leave no trail and can't be tracked. The warden can choose for the group to leave a trail, or even for specific members of the group to leave a trail if he so desires. This ability replaces hunter's bond."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Warden ~ Able Explorer", at_level: 5, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Warden ~ Wilderness Whispers", at_level: 20, description: Some("At 20th level, a warden cannot be surprised and always acts as if he had rolled a natural 20 on any initiative check while within any of his favored terrains. A warden can always move at full speed while using Survival to follow tracks without penalty. This ability replaces the ranger's fifth favorite enemy and master hunter."), benefit: None },
            ],
        },
        // Ranger Archetype ~ Wild Stalker -- uc_abilities_class.lst:1516
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Wild Stalker",
            subject: "Ranger",
            archetype_name: "Wild Stalker",
            description: Some("Civilization grows stronger and more decadent with each passing year. It tears into unclaimed wilderness and destroys the fragile ecology in its constant push for expansion and exploitation. The wild stalker forsakes the bonds of community and lives in the trackless wilds far from others of his kind, or perhaps grew up there, never knowing of civilization as anything more than his enemy. He drives pioneers back to civilization and strives to keep the land unspoiled."),
            source_page: Some("p.68"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ WILD STALKER],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerFavoredEnemy.RangerCombatStyle.RangerHuntersBond]"]),
            replaces: Some(&["RangerFavoredEnemy", "RangerCombatStyle", "RangerHuntersBond"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Wild Stalker ~ Strong Senses", at_level: 1, description: Some("At 1st level, a wild stalker's life among the wild has sharpened his senses. He gains low-light vision and a +1 bonus on Perception checks. If he already has low-light vision, he gains a +2 bonus on Perception checks instead. This bonus increases by +1 for every four levels after 1st (to a maximum of +6 at 20th level, or +7 if the character did not gain low-light vision from this ability). This ability replaces the ranger's first favored enemy ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wild Stalker ~ Uncanny Dodge", at_level: 2, description: Some("At 2nd level, a wild stalker gains uncanny dodge as the barbarian's class feature. This ability replaces the ranger's 2nd-level combat style feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wild Stalker ~ Rage of the Wild", at_level: 4, description: Some("At 4th level, a wild stalker gains the rage ability as the barbarian class feature, but its barbarian level is considered to be his ranger level -3. This ability replaces hunter's bond."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wild Stalker ~ Rage Powers", at_level: 5, description: Some("At 5th level, a wild stalker ranger gains a single rage power, as the barbarian class feature. He gains another rage power each five levels after 5th (to a maximum of four rage powers at 20th level). This ability replaces the ranger's second, third, fourth, and fifth favored enemy abilities."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wild Stalker ~ Wild Talents", at_level: 6, description: Some("At 6th level, a wild stalker can either take a rage power, or gains a +2 insight bonus into any one of the following skills - Acrobatics, Climb, Perception, Stealth, Survival, or Swim. The wild stalker can gain one of these two benefits again every five levels after 6th (to a maximum of 4 times at 20th level). This ability replaces the ranger's 6nd-, 10th-, 14th, and 18th-level combat style feats."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Bandit -- uc_abilities_class.lst:1667
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Bandit",
            subject: "Rogue",
            archetype_name: "Bandit",
            description: Some("Bandits, brigands, and highwaymen hone their craft along the roadsides and byways of the world, where they leap from the shadows to plunder the spoils of passing travelers. Bandits tend to have a variety of skills; sometimes these skills are similar to a ranger's, but a bandit's tactics always deal with surprise followed by intimidation, with the clear threat of naked force if intimidation does not do the trick. But not all bandits are ruthless thugs. There are those who become outlaws in opposition to tyrants or similar oppressors. These bandits target the status quo and distribute their spoils back into the needy hands of the peasantry."),
            source_page: Some("p.71"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Bandit],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RogueUncannyDodge,TYPE.RogueImprovedUncannyDodge]"]),
            replaces: Some(&["RogueUncannyDodge", "RogueImprovedUncannyDodge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Bandit ~ Ambush", at_level: 4, description: Some("At 4th level, a bandit becomes fully practiced in the art of ambushing. When she acts in the surprise round, she can take a move action, standard action, and swift action during the surprise round, not just a move or standard action. This ability replaces uncanny dodge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bandit ~ Fearsome Strike", at_level: 8, description: Some("At 8th level, a bandit can terrify an opponent with a single hit. A number of times per day equal to her Charisma modifier (minimum 1), when a bandit confirms a critical hit and deals sneak attack damage to an opponent, she can choose to make the opponent frightened for a number of rounds equal to her Charisma modifier (minimum 1). She cannot use this ability in conjunction with a critical feat. This ability replaces improved uncanny dodge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bandit ~ Rogue Talents", at_level: 1, description: Some("The following rogue talents complement the bandit archetype - assault leader **, combat trick, cunning trigger**, snap shot**, strong impression**, surprise attack, terrain master*, underhanded*, and weapon training."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bandit ~ Advanced Talents", at_level: 1, description: Some("The following advanced rogue talents complement the bandit archetype - hide in plain sight*, knock-out blow**, and opportunist."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Chameleon -- uc_abilities_class.lst:1668
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Chameleon",
            subject: "Rogue",
            archetype_name: "Chameleon",
            description: Some("An absolute master of disguise, a chameleon effortlessly blends into any environment. Whether disappearing into crowded city streets, vanishing into desert sands, or slipping into the darkness of subterranean tunnels, the chameleon relies upon her ability to become part of her surroundings."),
            source_page: Some("p.71"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Chameleon],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RogueTrapFinding,TYPE.RogueTrapSense]"]),
            replaces: Some(&["RogueTrapFinding", "RogueTrapSense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Chameleon ~ Misdirection", at_level: 1, description: Some("At 1st level, a chameleon begins her career knowing that the secret to disappearing lies in deceiving the senses of her observers. Every day she gains a pool of stealth points equal to her ranks in Bluff. These points refresh at the start of each day. Before making a Stealth check, she can choose to put stealth points into the roll, gaining a bonus on Stealth checks equal to the number of stealth points she puts into the roll. If she gains a bonus on Bluff checks because of a feat (such as Skill Focus [Bluff ]), she adds a number of points to her stealth pool equal to the bonus the feat grants. This ability replaces trapfinding."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Chameleon ~ Effortless Sneak", at_level: 3, description: Some("At 3rd level, the chameleon chooses a single terrain from the ranger's favored terrain class feature. While she is within that terrain, she can take 10 on any Stealth check she can make within that terrain. When the chameleon reaches 6th level, and every three levels thereafter, she chooses a new type of terrain from the ranger's favored terrain list. She gains this ability with the newly picked terrain. This ability replaces trap sense."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Chameleon ~ Rogue Talents", at_level: 1, description: Some("The following rogue talents complement the chameleon archetype - camouflage**, fast stealth, quick disguise**, and terrain mastery*."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Chameleon ~ Advanced Talents", at_level: 1, description: Some("The following advanced rogue talents complement the chameleon archetype - hide in plain sight* and master of disguise**."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Charlatan -- uc_abilities_class.lst:1669
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Charlatan",
            subject: "Rogue",
            archetype_name: "Charlatan",
            description: Some("A charlatan is a master of lies and deception. Whether creating simple cons or elaborately woven hoaxes capable of swaying the masses and those in positions of power, a charlatan is often a purveyor of snake-oil, forgeries, and rumormongering."),
            source_page: Some("p.72"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Charlatan],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RogueTrapFinding,TYPE.RogueTrapSense]"]),
            replaces: Some(&["RogueTrapFinding", "RogueTrapSense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Charlatan ~ Natural Born Liar", at_level: 1, description: Some("At 1st level, when a charlatan successfully deceives a creature with a Bluff, that creature takes a - 2 penalty on the charlatan's Bluff checks for the next 24 hours. This ability does not stack with itself. This ability replaces trapfinding."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Charlatan ~ Grand Hoax", at_level: 3, description: Some("At 3rd level, the charlatan begins to master the art of the grand hoax and learns to create deceptions designed to bestow harm upon her foes. She gains the rumormonger advanced talent, even though she is not yet 10th level. This ability takes the place of trap sense."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Charlatan ~ Rogue Talents", at_level: 1, description: Some("The following rogue talents complement the charlatan archetype - black market connections*, coax information**, cunning lie*, honeyed words**, and underhanded*."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Charlatan ~ Advanced Talents", at_level: 1, description: Some("The following advanced rogue talents complement the charlatan archetype - hard to fool*, skill mastery, and unwitting ally*."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Driver -- uc_abilities_class.lst:1670
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Driver",
            subject: "Rogue",
            archetype_name: "Driver",
            description: Some("A driver makes her living driving vehicles in the service of those who can pay for her considerable talents. She may be a wagon-driver practiced at smuggling contraband, an accomplished musher blazing arctic trails, or a seemingly refined coach driver placing her roguish talents in the service of a wealthy patron. This archetype works best with the vehicle combat rules in Chapter 4."),
            source_page: Some("p.72"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Driver],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RogueTrapfinding,TYPE.RogueTrapSense]"]),
            replaces: Some(&["RogueTrapfinding", "RogueTrapSense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Driver ~ Hard Drive", at_level: 1, description: Some("When driving a vehicle that uses a group of animals or magical beasts as muscle propulsion, the DCs of all driving checks are reduced by 2, the maximum speed of the vehicle increases by 10 feet, and the acceleration increases by 5 feet. This ability takes the place of trapfinding."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Driver ~ Driver's Fortitude", at_level: 3, description: Some("At 3rd level, the driver learns to keep driving and maintain control of her vehicle, even when mortally wounded. If the driver drops below 0 hit points but is not dead, she can attempt a DC 15 Fortitude save each round to remain conscious and in control of her vehicle, though she may take no other actions. This ability replaces trap sense."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Driver ~ Advanced Talents", at_level: 1, description: Some("The following advanced rogue talents complement the driver archetype feat and getaway master*."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Driver ~ Rogue Talents", at_level: 1, description: Some("The following rogue talents complement the driver archetype - firearm training* and getaway artist*."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Knife Master -- uc_abilities_class.lst:1671
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Knife Master",
            subject: "Rogue",
            archetype_name: "Knife Master",
            description: Some("The knife master is a trained killer who specializes in close-up combat and the wave and weave of knife fighting. In her hands, daggers and other similar light blades become truly deadly instruments."),
            source_page: Some("p.72"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Knife Master],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RogueTrapfinding,TYPE.RogueTrapSense]"]),
            replaces: Some(&["RogueTrapfinding", "RogueTrapSense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Knife Master ~ Hidden Blade", at_level: 1, description: Some("A knife master adds 1/2 her level on Sleight of Hand checks made to conceal a light blade. This ability replaces trapfinding. Bonus +%1|HiddenBladeBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Knife Master ~ Sneak Stab", at_level: 1, description: Some("A knife master focuses her ability to deal sneak attack damage with daggers and similar weapons to such a degree that she can deal more sneak attack damage with those weapons at the expense of sneak attacks with other weapons. When she makes a sneak attack with a dagger, kerambit (page 130), kukri, punching daggers, starknife, or swordbreaker dagger (Advanced Player's Guide 178), she uses d8s to roll sneak attack damage instead of d6s. For sneak attacks with all other weapons, she uses d4s instead of d6s. This ability is identical in all other ways to sneak attack, and supplements that ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Knife Master ~ Blade Sense", at_level: 3, description: Some("At 3rd level, a knife master is so skilled in combat involving light blades that she gains a +1 dodge bonus to AC against attacks made against her with light blades. This bonus increases by +1 for every three levels, to a maximum of +6 at 18th level. This ability replaces trap sense."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Knife Master ~ Rogue Talents", at_level: 1, description: Some("The following rogue talents complement the knife master archetype - befuddling strike**, combat trick, offensive defense**, surprise attack, underhanded*, and weapon training."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Knife Master ~ Advanced Talents", at_level: 1, description: Some("The following advanced rogue talents complement the knife master archetype - another day**, confounding blades*, deadly sneak**, entanglement of blades**, and unwitting ally*."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Pirate -- uc_abilities_class.lst:1672
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Pirate",
            subject: "Rogue",
            archetype_name: "Pirate",
            description: Some("A pirate breaks from the confines of country and king to commit her crimes upon the high seas. She holds allegiance only to her ship and its captain (if even that much is true), and lives a lawless life upon the waves, plundering ships and shorelines as suits her whim."),
            source_page: Some("p.72"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Pirate],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RogueTrapfinding,TYPE.RogueTrapSense,TYPE.RogueTalent2]"]),
            replaces: Some(&["RogueTrapfinding", "RogueTrapSense", "RogueTalent2"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Pirate ~ Sea Legs", at_level: 1, description: Some("At 1st level, a pirate becomes adept at moving on ships, boats, and similar vessels. She gains the Sea Legs feat (page 117) as a bonus feat, even if she does not meet the prerequisites. This ability replaces trapfinding."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Pirate ~ Swinging Reposition", at_level: 2, description: Some("At 2nd level, a pirate incorporates a ship's masts, rigging, ropes, sails, and other such structures into her combat style. Provided she is wearing light armor or no armor, when fighting in an environment where such structures exist, the rogue incorporates them into her movement, and does not have to move in a straight line when making either a charge attack or a bull rush combat maneuver. Once she completes her attack or maneuver, she can reposition herself. Immediately after making the charge or bull rush, she can move 5 feet as a free action, even if the charge ends her turn. This movement does not provoke attacks of opportunity. This ability replaces the 2nd-level rogue talent."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Pirate ~ Unflinching", at_level: 3, description: Some("Pirates are a salty and steadfast lot. At 3rd level, a pirate gains a +1 bonus on saving throws against fear and mind-affecting effects. This bonus increases by +1 for every three levels, to a maximum of +6 at 18th level. This ability replaces trap sense."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Pirate ~ Rogue Talents", at_level: 1, description: Some("The following rogue talents complement the pirate archetype - black market connections*, combat trick, finesse rogue, firearm training*, hold breath*, rope master*, and strong stroke*."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Pirate ~ Advanced Talents", at_level: 1, description: Some("The following advanced rogue talents complement the pirate archetype - getaway master*, hard to fool*, and unwitting ally*."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Roof Runner -- uc_abilities_class.lst:1673
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Roof Runner",
            subject: "Rogue",
            archetype_name: "Roof Runner",
            description: Some("A specialized urban acrobat, the roof runner makes her home high atop the spires and gables of great cities. She is skilled at traveling these uneven surfaces at full speed, performing daring leaps and deftly balancing upon narrow and precarious ledges."),
            source_page: Some("p.73"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Roof Runner],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RogueTrapfinding,TYPE.RogueTrapSense]"]),
            replaces: Some(&["RogueTrapfinding", "RogueTrapSense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Roof Runner ~ Roof Running", at_level: 1, description: Some("At 1st level, a roof runner becomes entirely adept at moving across the tops of buildings, spires, and similar locations. Provided she is wearing light armor or no armor, the roof runner can move at full speed while traveling across the tops of buildings or similar structures, and takes no penalties on any Dexterity-based Skill checks or Reflex saves that might be incurred from moving about on a roof. This ability replaces trapfinding."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Roof Runner ~ Tumbling Descent", at_level: 2, description: Some("At 2nd level, a roof runner can use her acrobatics skill to attempt a rapid descent from a rooftop or another surface, ricocheting against another surface and then diving through an opening (such as a balcony or window) directly below. So long as she has at least two surfaces no farther than 10 feet apart to bounce against, she can ricochet her body back from one to the next, descending great distances with a single check. The DC is 10 + 5 for every additional 10-foot increment descended beyond the initial 10 feet dropped. If she fails, she falls the full distance. This ability replaces trap sense."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Roof Runner ~ Rogue Talents", at_level: 1, description: Some("The following rogue talents complement the roof runner archetype - expert leaper**, ledge walker, nimble climber**, stand up, and terrain mastery*."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Roof Runner ~ Advanced Talents", at_level: 1, description: Some("The following advanced rogue talent complements the roof runner archetype - fast tumble**."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Sanctified Rogue -- uc_abilities_class.lst:1674
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Sanctified Rogue",
            subject: "Rogue",
            archetype_name: "Sanctified Rogue",
            description: Some("The sanctified rogue serves a higher purpose, acting as a representative of a church or cult, or following the tenets of a specific faith or deity. While most sanctified rogues share the beliefs or ideals of the religions they represent, not all of them are necessarily pious. A handful of disreputable holy thieves serve purely as mercenaries. At the other end of spectrum, however, more sanctified rogues adhere to the tenets of their faith with fanatical devotion. Regardless, all seem blessed with the ability to call upon divine favor to aid them in times of need."),
            source_page: Some("p.73"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Sanctified Rogue],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RogueUncannyDodge,TYPE.RogueImprovedUncannyDodge]"]),
            replaces: Some(&["RogueUncannyDodge", "RogueImprovedUncannyDodge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Sanctified Rogue ~ Divine Purpose", at_level: 4, description: Some("At 4th level, the favor of a deity or religious institution grants a special blessing on a sanctified rogue, shoring up some of her weaknesses. She gains a +1 sacred bonus on Fortitude and Will saving throws. This ability replaces uncanny dodge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sanctified Rogue ~ Divine Epiphany", at_level: 8, description: Some("At 8th level, once per day, a sanctified rogue can peer into the immediate future and predict the results of a specific action as if she had cast an augury spell. The caster level of this spell is equal to the sanctified rogue's rogue class level. This ability replaces improved uncanny dodge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sanctified Rogue ~ Rogue Talents", at_level: 1, description: Some("The following rogue talents complement the sanctified rogue archetype - esoteric scholar*, major magic, and minor magic."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sanctified Rogue ~ Advanced Talents", at_level: 1, description: Some("The following advanced rogue talents complement the sanctified rogue archetype - feat and skill mastery."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Survivalist -- uc_abilities_class.lst:1675
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Survivalist",
            subject: "Rogue",
            archetype_name: "Survivalist",
            description: Some("The survivalist focuses her talents on surviving harsh and unforgiving conditions that would kill a lesser rogue. Whether in blazing deserts or frigid arctic wastes-or simply the cold, hard reality of the streets-the survivalist uses her training to both ensure her own success and provide for the safety of her allies."),
            source_page: Some("p.73"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Survivalist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RogueTrapfinding,TYPE.RogueTrapSense]"]),
            replaces: Some(&["RogueTrapfinding", "RogueTrapSense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Survivalist ~ Hardy", at_level: 1, description: Some("At 1st level, a survivalist is already prepared to endure extreme hardships and environmental conditions. She can go twice the normal number of days without water and triple the normal number of days without food before feeling the effects of either thirst or starvation. This ability replaces trapfinding."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Survivalist ~ Endure Elements", at_level: 3, description: Some("At 3rd level, a survivalist rogue can use Endure Elements once per day as a spell-like ability. The survivalist's caster level is the same as her rogue level. This ability replaces trap sense."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Survivalist ~ Rogue Talents", at_level: 1, description: Some("The following rogue talents complement the survivalist archetype - hold breath*, iron guts*, resiliency, strong stroke*, survivalist**, terrain mastery*, and wall scramble*."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Survivalist ~ Advanced Talents", at_level: 1, description: Some("The following advanced rogue talents complement the survivalist archetype - another day**, defensive roll, feat, and skill mastery."), benefit: None },
            ],
        },
        // Wizard Archetype ~ Arcane Bomber -- uc_abilities_class.lst:1738
        ArchetypeSwapEntry {
            key: "Wizard Archetype ~ Arcane Bomber",
            subject: "Wizard",
            archetype_name: "Arcane Bomber",
            description: Some("To many wizards, the experimentation of the alchemist seems quaint, if not dangerous or frightening. A few wizards take up the secrets of the bomb, however, fusing alchemy with their already considerable magical power."),
            source_page: Some("p.74"),
            prerequisites: Some(&["PRECLASS:1,Wizard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Wizard Archetype ~ Arcane Bomber],[!PREABILITY:1,CATEGORY=Archetype,TYPE.WizardCantrips,TYPE.WizardArcaneBond,TYPE.WizardArcaneSchool]", "PREVAREQ:DisallowWizardArcaneSchoolArchetype,0"]),
            replaces: Some(&["WizardCantrips", "WizardArcaneBond", "WizardArcaneSchool"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Arcane Bomber ~ Bomb", at_level: 1, description: Some("At 1st level, the arcane bomber gains an ability nearly identical to the alchemist's bomb ability (Advanced Player's Guide 28). Unlike the alchemist, at 1st level, the arcane bomber chooses one type of energy from the following list: acid, cold, fire, and electricity. He can throw bombs of that type, but cannot modify them with discoveries. This ability stacks with the alchemist bomb ability to determine the level of bomb damage, but an arcane bomber that becomes an alchemist does not gain that class's bomb ability, nor does an alchemist that becomes an arcane bomber gain this bomb ability. This ability replaces arcane bond."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Arcane Bomber ~ School of the Bomb", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Arcane Bomber ~ Spellblast Bombs", at_level: 1, description: Some("At 1st level, as a swift action, an arcane bomber can sacrifice one of his spells to empower the next bomb he throws during his turn. When he does, he gains a bonus to hit with the next bomb he throws before the end of his turn equal to the level of the spell he sacrificed, and a bonus to damage equal to twice the level of the spell. This ability replaces cantrips, but the arcana bomber gains the detect magic and read magic cantrips and places them in his spellbook. He can cast either of these as 1st-level spells."), benefit: None },
                ArchetypeGrant { grants_feature_key: "No Cantrips ~ Wizard", at_level: 1, description: None, benefit: None },
            ],
        },
        // Wizard Archetype ~ Siege Mage -- uc_abilities_class.lst:1739
        ArchetypeSwapEntry {
            key: "Wizard Archetype ~ Siege Mage",
            subject: "Wizard",
            archetype_name: "Siege Mage",
            description: Some("The siege mage combines his arcane mastery with a supernatural link to siege engines."),
            source_page: Some("p.74"),
            prerequisites: Some(&["PRECLASS:1,Wizard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Wizard Archetype ~ Siege Mage],[!PREABILITY:1,CATEGORY=Archetype,TYPE.WizardCantrips,TYPE.WizardScribeScroll,TYPE.WizardArcaneSchool]", "PREVAREQ:DisallowWizardArcaneSchoolArchetype,0"]),
            replaces: Some(&["WizardCantrips", "WizardScribeScroll", "WizardArcaneSchool"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "No Cantrips ~ Wizard", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Siege Mage ~ Empower Siege Engine", at_level: 1, description: Some("As a swift action, a siege mage can sacrifice one of his spells to empower the next attack he makes before the end of his turn with a siege engine he is bonded with. When he does, the siege engine attack gains a bonus on its attack roll or targeting roll equal to the level of the spell he sacrificed, and a bonus to damage equal to 3 + the level of the spell. This ability replaces cantrips, but the siege mage gains the detect magic and read magic cantrips and places them in his spellbook. He can cast either of these as 1st-level spells."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Siege Mage ~ Siege Engine Bond", at_level: 1, description: Some("At 1st level, a siege mage can bond with a single siege engine within 30 feet and line of sight as a standard action. He can utilize the power of this link to aim and fire the siege engine remotely (as long as he's within 30 feet), though it still requires a crew to reload the siege engine. At 10th level, he can reload, aim, and fire the siege engine purely by the power of this link, and no longer needs a crew to control the siege engine. It still takes the normal required amount of time and actions to control a siege engine in this manner. This ability replaces arcane bond. The siege mage can end this bond with a free action. A siege mage can bond with a siege engine in this manner a number of times per day equal to his Intelligence modifier (minimum 1) but can only be bonded with one siege engine at a time."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Siege Mage ~ Siege Engineer", at_level: 1, description: Some("At 1st level, the siege mage gains Siege Engineer (see page 118) as a bonus feat, even though he does not meet the prerequisites for that feat. This ability replaces scribe scroll."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Siege Mage ~ Siege School", at_level: 1, description: None, benefit: None },
            ],
        },
        // Wizard Archetype ~ Spellslinger -- uc_abilities_class.lst:1740
        ArchetypeSwapEntry {
            key: "Wizard Archetype ~ Spellslinger",
            subject: "Wizard",
            archetype_name: "Spellslinger",
            description: Some("While few contest the seductive allure of commanding arcane and occult powers, there are those wizards who become obsessed with the natural mysteries of black powder. Combining this emerging technology with their considerable arcane skills, they transform firearms into a powerful focus."),
            source_page: Some("p.74"),
            prerequisites: Some(&["PRECLASS:1,Wizard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Wizard Archetype ~ Spellslinger],[!PREABILITY:1,CATEGORY=Archetype,TYPE.WizardArcaneBond,TYPE.WizardScribeScroll,TYPE.WizardCantrips,TYPE.WizardArcaneSchool]", "PREVAREQ:DisallowWizardArcaneSchoolArchetype,0"]),
            replaces: Some(&["WizardArcaneBond", "WizardScribeScroll", "WizardCantrips", "WizardArcaneSchool"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "No Cantrips ~ Wizard", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Spellslinger ~ Arcane Gun", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Spellslinger ~ Gunsmith", at_level: 1, description: Some("The spellslinger gains the Gunsmithing feat and a battered gun that is identical to the gun a Gunslinger (see page 9) gains at first level. If the spellslinger chooses the ability to attune two arcane guns, he still only starts out with one gun. Like a Gunslinger, a spellslinger can use the Gunsmithing feat to restore his battered gun.&nl; This ability replaces Scribe Scroll."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spellslinger ~ Mage Bullets", at_level: 1, description: Some("A spellslinger is adept at transferring spell energy into his arcane gun attacks. As a swift action, he can sacrifice a spell and transform that energy into a weapon bonus equal to the level of the spell sacrificed on a single barrel of his firearm. With that weapon bonus the spellslinger can apply any of the following to his arcane bond: enhancement bonuses (up to +5) and dancing, defending, distance, flaming, flaming burst, frost, ghost touch, icy burst, merciful, seeking, shock, shocking burst, spell storing, thundering, vicious, and wounding. An arcane gun gains no benefit from having two of the same weapon special abilities on the same barrel. The effect of the mage bullets ability lasts for a number of minutes equal to the level of the spell sacrificed, or until this ability is used again to assign the barrel different enhancements.&nl; This ability replaces cantrips, but the spellslinger gains the detect magic and read magic cantrips and places them in his spellbook. He can cast either of these as 1st-level spells."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spellslinger ~ School of the Gun", at_level: 1, description: None, benefit: None },
            ],
        },        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_has_65_records() {
        assert_eq!(archetype_swap_tables().len(), 65);
    }

    #[test]
    fn keys_are_unique_within_book() {
        let keys: std::collections::BTreeSet<&str> =
            archetype_swap_tables().iter().map(|e| e.key).collect();
        assert_eq!(keys.len(), archetype_swap_tables().len());
    }

    #[test]
    fn every_master_record_carries_a_real_description() {
        for e in archetype_swap_tables() {
            assert!(e.description.is_some(), "{} has no DESC:", e.key);
        }
    }

    /// UC's own rate: 22% (14/65) -- a fifth distinct value alongside
    /// UPsi 33%, ACG 33%, APG 52%, UM 27%. No convergence across five
    /// books; the durable claim is that TYPE:/ABILITY: disagree in most
    /// records at a book-dependent rate, not any specific percentage.
    #[test]
    fn the_type_and_ability_lists_genuinely_disagree() {
        let total_replaces: usize =
            archetype_swap_tables().iter().map(|e| e.replaces.map_or(0, |r| r.len())).sum();
        let total_grants: usize = archetype_swap_tables().iter().map(|e| e.grants.len()).sum();
        assert_eq!(total_replaces, 282, "total TYPE: replaced-slot count across all 65 records");
        assert_eq!(total_grants, 354, "total ABILITY: granted-feature count across all 65 records, after the category ruling");
        assert_ne!(total_replaces, total_grants);

        let equal_count_records = archetype_swap_tables()
            .iter()
            .filter(|e| e.replaces.map_or(0, |r| r.len()) == e.grants.len())
            .count();
        assert_eq!(equal_count_records, 14, "of 65 (22%) -- UC's own rate");
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
        assert_eq!(resolved, 294, "294 of 354 grants carry real DESC:/BENEFIT: text -- see this module's own doc comment for the 60 that did not");
    }
}
