//! Advanced Race Guide (ARG) archetype-swap catalog. SD28-E30
//! (`epic-32-archetype-swap`) tier-1 table 6. See
//! `ultimate_psionics::archetype_tables`'s own module doc comment for
//! the full struct rationale, the exhaustively-enumerated `ABILITY:`
//! grant grammar and its per-family inclusion ruling (`Internal`
//! excluded, `NORMAL`-type excluded), and the `.MOD`-injected-grant
//! floor every table in this program states explicitly.
//!
//! **Agreement rate, sixth book: 14% (8/59)** -- 304 total `TYPE:`-
//! replaced slots vs 346 total `ABILITY:`-granted features. Alongside
//! UPsi 33%, ACG 33%, APG 52%, UM 27%, UC 22% -- a sixth distinct
//! value, the lowest yet, continuing the confirmed no-convergence
//! finding (`decisions.md §51`): `TYPE:`/`ABILITY:` disagree in the
//! majority of records in every book measured, at a book-dependent
//! rate, not a single number.
//!
//! **343 of 346 sub-feature grants (99%) resolved to real `DESC:`/
//! `BENEFIT:` text -- the cleanest resolution rate of any table so
//! far**, ahead of ACG's own 99%. The unresolved-grant taxonomy is
//! still open (per team-lead's own correction to an earlier "the tail
//! has converged" claim, made after UM and retracted after UC's own new
//! `Weapon and Armor Proficiency` shape) -- this book adds no new shape,
//! but that is one book's worth of evidence, not closure. The 3
//! shortfalls are individual failed `KEY:` lookups, two of them
//! (`Treetop Monk ~ Wood Affinity 1`/`2`) carrying a trailing space in
//! their own grant-token name -- plausibly a genuine corpus typo in the
//! `ABILITY:` token itself rather than a missing row, not confirmed.
//!
//! **This book's own share of the 1,282-row corpus-wide `.MOD`-
//! injection population (`decisions.md §51`'s own addendum) is 72
//! rows.** This table's `grants` field is bounded below by that count
//! and by the tier-2 sub-feature population, not closed by either.
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
//! advanced_race_guide/arg_abilities_class.lst`), generated
//! programmatically by a one-off extraction script, not hand-transcribed.

use super::super::archetype_swap::{ArchetypeGrant, ArchetypeSwapEntry};

/// Full ARG archetype-swap catalog: 59 real, distinct master records, in
/// source order. Built once and cached for the process lifetime.
pub fn archetype_swap_tables() -> &'static [ArchetypeSwapEntry] {
    static TABLE: std::sync::OnceLock<Vec<ArchetypeSwapEntry>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        vec![
        // Alchemist Archetype ~ Bogborn Alchemist -- arg_abilities_class.lst:859
        ArchetypeSwapEntry {
            key: "Alchemist Archetype ~ Bogborn Alchemist",
            subject: "Alchemist",
            archetype_name: "Bogborn Alchemist",
            description: Some("Some grippli alchemists are particularly attuned to the swamps and the dangerous creatures that inhabit them; these serve as their laboratories and research subjects, respectively."),
            source_page: Some("p.191"),
            prerequisites: Some(&["PRECLASS:1,Alchemist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Alchemist Archetype ~ Bogborn Alchemist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.AlchemistThrowAnything]", "PREFACT:1,TEMPLATES,IsGrippli=true"]),
            replaces: Some(&["AlchemistThrowAnything"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Bogborn Alchemist ~ Class Skills", at_level: 1, description: Some("A bogborn alchemist adds Swim to his list of class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bogborn Alchemist ~ Amphibious Mutagen", at_level: 1, description: Some("When a bogborn alchemist uses a mutagen, he may choose to have his mutagen form enhanced for aquatic movement. This gives him the amphibious special quality, his feet elongate, and the webbing between his fingers and toes expands, granting a swim speed of 15 feet."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bogborn Alchemist ~ Discoveries", at_level: 1, description: Some("The following discoveries complement the bogborn alchemist archetype: chameleon (Advanced Race Guide); concentrate poison, sticky poison (Advanced Player's Guide); nauseating flesh, poison conversion (Ultimate Combat); bottled ooze, tanglefoot bomb, tentacle (Ultimate Magic)."), benefit: None },
            ],
        },
        // Alchemist Archetype ~ Bramble Brewer -- arg_abilities_class.lst:195
        ArchetypeSwapEntry {
            key: "Alchemist Archetype ~ Bramble Brewer",
            subject: "Alchemist",
            archetype_name: "Bramble Brewer",
            description: Some("Some half-elven alchemists merge human curiosity with their elven link to nature. Such alchemists can manipulate the forces of alchemy to create bombs that reshape terrain and defoliate swaths of vegetation or to create mutagens that bestow the resilience of oak or the tenacity of bamboo."),
            source_page: Some("p.43"),
            prerequisites: Some(&["PRECLASS:1,Alchemist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Alchemist Archetype ~ Bramble Brewer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.AlchemistDiscovery]", "PREFACT:1,TEMPLATES,IsHalfElf=true"]),
            replaces: Some(&["AlchemistDiscovery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Bramble Brewer ~ Briar Bombs", at_level: 2, description: Some("A bramble brewer gains the tanglefoot bomb discovery (Ultimate Magic 17), but the entanglement's duration persists for a number of rounds equal to the bramble brewer's Intelligence modifier (minimum 1 round). Additionally, when a bramble brewer throws a tanglefoot bomb, it transforms all squares in its splash radius into difficult terrain that persists for as long as the bomb's entangling effect. Although these bombs deal no damage, for every 1d6 points of damage the bramble brewer's regular bombs deal, the briar bomb's splash radius increases by 5 feet."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bramble Brewer ~ Dendrite Mutagen", at_level: 1, description: Some("A bramble brewer's mutagen still contains transformative power, but grants a treelike sturdiness rather than the feral power of standard mutagens. When imbibed, a dendrite mutagen grants a +4 natural armor bonus, a +2 alchemical bonus to one physical ability score, and a -2 penalty to the corresponding mental ability score (as per the normal mutagen class feature; Advanced Player's Guide 28). In addition, the alchemist gains fast healing 1 as long as he is in an area of bright light (such as sunlight or inside the area of a daylight spell). This otherwise works like the standard mutagen class feature and replaces that ability. A bramble brewer who selects the greater mutagen discovery can create a dendrite mutagen that still grants a +4 natural armor bonus, and also grants a +4 alchemical bonus to one physical ability score and a +2 bonus to a second physical ability score. The bramble brewer takes a -2 penalty to both associated mental ability scores as long as the mutagen persists, but his fast healing increases to 3 as long as he is in an area of bright light. This otherwise works like the greater mutagen discovery and replaces that ability. A bramble brewer who selects the grand mutagen discovery can brew a dendrite mutagen that now grants a +6 natural armor bonus, a +6 alchemical bonus to one physical ability score, a +4 alchemical bonus to a second physical ability score, and a +2 alchemical bonus to a third physical ability score. The bramble brewer takes a -2 penalty to his Intelligence, Wisdom, and Charisma scores as long as the mutagen persists. The bramble brewer's fast healing increases to 5 as long as he is within an area of bright light. This otherwise works like the grand mutagen discovery and replaces that ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bramble Brewer ~ Grand Discovery", at_level: 20, description: Some("At 20th level, a bramble brewer who selects the true mutagen grand discovery can create a dendrite mutagen that grants a +8 natural armor bonus and a +6 alchemical bonus to Strength, Dexterity, and Constitution. The bramble brewer takes a -2 penalty to his Intelligence, Wisdom, and Charisma scores as long as the mutagen persists. The bramble brewer's gains fast healing 10, but instead of only gaining fast healing in bright light, this fast healing persists in areas of bright or normal light. A bramble brewer must possess the grand mutagen discovery before selecting this discovery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bramble Brewer ~ Discoveries", at_level: 1, description: Some("The following discoveries complement the bramble brewer archetype: precise bombs (Advanced Player's Guide); defoliant bomb (Advanced Race Guide); lingering spirit, strafe bomb, sunlight bomb (Ultimate Magic)."), benefit: None },
            ],
        },
        // Alchemist Archetype ~ Deep Bomber -- arg_abilities_class.lst:970
        ArchetypeSwapEntry {
            key: "Alchemist Archetype ~ Deep Bomber",
            subject: "Alchemist",
            archetype_name: "Deep Bomber",
            description: Some("Consumed with keeping hidden from the horrors below the surface, svirfneblin use their racial proclivity for alchemy and their inherent talent for obfuscation to strike their enemies from the darkness and retreat unseen."),
            source_page: Some("p.205"),
            prerequisites: Some(&["PRECLASS:1,Alchemist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Alchemist Archetype ~ Deep Bomber],[!PREABILITY:1,CATEGORY=Archetype,TYPE.AlchemistPoisonUse,TYPE.AlchemistSwiftAlchemy,TYPE.AlchemistSwiftPoisoning]", "PREFACT:1,TEMPLATES,IsSvirfneblin=true"]),
            replaces: Some(&["AlchemistPoisonUse", "AlchemistSwiftAlchemy", "AlchemistSwiftPoisoning"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Deep Bomber ~ Silent Bomb", at_level: 2, description: Some("When the deep bomber creates a bomb, he can choose to have it explode without making any noise, although those damaged by it may cry out."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deep Bomber ~ Targeting Bomb", at_level: 3, description: Some("When the deep bomber creates a bomb, he can choose to have its detonation include a faerie fire effect that applies to all creatures within the splash radius (including the target, if any)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deep Bomber ~ Stonekin", at_level: 6, description: Some("The deep bomber automatically learns tree shape as a 2nd-level extract, except instead of a tree, he takes the form of a stalagmite that is the same size as his current size. At 7th level, he automatically learns meld into stone as a 3rd-level extract."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deep Bomber ~ Discoveries", at_level: 1, description: Some("The following discoveries complement the deep bomber archetype: delayed bomb, dispelling bomb, fast bombs, infusion, madness bomb, poison bomb."), benefit: None },
            ],
        },
        // Alchemist Archetype ~ Fire Bomber -- arg_abilities_class.lst:546
        ArchetypeSwapEntry {
            key: "Alchemist Archetype ~ Fire Bomber",
            subject: "Alchemist",
            archetype_name: "Fire Bomber",
            description: Some("Fire bombers are exceptionally good at using bombs to burn creatures and blow things up, but are not quite as good at creating other types of bombs or extracts."),
            source_page: Some("p.117"),
            prerequisites: Some(&["PRECLASS:1,Alchemist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Alchemist Archetype ~ Fire Bomber],[!PREABILITY:1,CATEGORY=Archetype,TYPE.AlchemistBomb,TYPE.AlchemistThrowAnything,TYPE.AlchemistDiscovery4,TYPE.AlchemistPoisonResistance6,TYPE.AlchemistPoisonImmunity,TYPE.AlchemistPersistentMutagen]", "PREFACT:1,TEMPLATES,IsGoblin=true"]),
            replaces: Some(&["AlchemistDiscovery4", "AlchemistPoisonResistance6", "AlchemistPoisonImmunity", "AlchemistPersistentMutagen"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Fire Bomber ~ Weapon and Armor Proficiency", at_level: 1, description: Some("A fire bomber treats torches as a simple weapon."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fire Bomber ~ Fire Bombardier", at_level: 1, description: Some("When a fire bomber throws a bomb that deals fire damage, all creatures in the splash radius take an additional point of damage per die of fire damage dealt. Fire bombers only add their Intelligence bonus to damage from bombs or alchemical substances that deal fire damage. This otherwise works like the alchemist's bomb and throw anything abilities."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fire Bomber ~ Fiery Cocktail", at_level: 4, description: Some("Whenever a fire bomber uses a discovery that deals damage other than fire damage, he can split the damage dice evenly between the bomb's primary damage type and 1d6 points of fire damage; when there is an odd number of damage dice, the odd die of damage comes from the primary damage type. For example, an 8th-level fire bomber could throw a concussive bomb that deals 2d6 points of fire damage and 3d4 points of sonic damage. Additional effects from the bomb still apply, but the save DC for admixture bombs is reduced by 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fire Bomber ~ Fire Body", at_level: 8, description: Some("Fire bombers add elemental body I to their spell list. Elemental body extracts prepared using this ability are limited to fire elementals only."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fire Bomber ~ Improved Fire Body", at_level: 10, description: Some("Fire bombers add elemental body II to their spell list. Elemental body extracts prepared using this ability are limited to fire elementals only."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fire Bomber ~ Greater Fire Body", at_level: 14, description: Some("Fire bombers add elemental body IV to their spell list. Elemental body extracts prepared using this ability are limited to fire elementals only."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fire Bomber ~ Discoveries", at_level: 1, description: Some("The following discoveries complement the fire bomber archetype: fire brand, rocket bomb (see sidebar); explosive bombs, fast bombs, inferno bomb, precise bombs (Advanced Player's Guide); breath weapon bomb, explosive missile, immolation bomb (Ultimate Combat); bottled ooze, confusion bomb, strafe bomb (Ultimate Magic)."), benefit: None },
            ],
        },
        // Alchemist Archetype ~ Plague Bringer -- arg_abilities_class.lst:709
        ArchetypeSwapEntry {
            key: "Alchemist Archetype ~ Plague Bringer",
            subject: "Alchemist",
            archetype_name: "Plague Bringer",
            description: Some("The plague bringer sees disease as the ultimate weapon, and has worked tirelessly to master new diseases and disease-delivery systems. A plague bringer feels no more remorse at unleashing his armaments on his enemies than an archer does when firing an arrow in the heat of battle. Disease is a tool, and the plague bringer is its master. A plague bringer has the following class features."),
            source_page: Some("p.153"),
            prerequisites: Some(&["PRECLASS:1,Alchemist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Alchemist Archetype ~ Plague Bringer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.AlchemistMutagen,TYPE.AlchemistPoisonResistance,TYPE.AlchemistPoisonResistance2,TYPE.AlchemistPoisonResistance4,TYPE.AlchemistPoisonResistance6,TYPE.AlchemistPoisonImmunity]", "PREFACT:1,TEMPLATES,IsRatfolk=true"]),
            replaces: Some(&["AlchemistMutagen", "AlchemistPoisonResistance", "AlchemistPoisonResistance2", "AlchemistPoisonResistance4", "AlchemistPoisonResistance6", "AlchemistPoisonImmunity"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Plague Bringer ~ Plague Vial", at_level: 1, description: Some("A plague bringer can create a plague vial, an alchemically grown and concentrated disease sample. It takes 1 hour to prepare a plague vial, and once prepared, the vial remains potent until used. A plague bringer can only maintain 1 plague vial at a time-if he prepares a second vial, any existing plague vial becomes inert. A plague vial that is not in a plague bringer's possession becomes inert until a plague bringer picks it up again. It's a standard action to drink a plague vial. Upon being imbibed, the plague vial infects the plague bringer's blood, sweat, tears, and other bodily fluids for %10 minutes. Any creature that harms him with melee attacks (except with reach weapons) must make a Fortitude save (DC %2) or become sickened for %1 rounds. The plague bringer is immune to the effect of his own plague vial, but not that of another's plague vial. The effects of multiple plague vials do not stack. As a standard action, the plague bringer can infect a weapon with this sickness (typically by licking it or wiping his blood or pus on it). The disease on the weapon works like a poisoned weapon, except the source is a disease instead of a poison (so a dwarf 's resistance to poison does not apply). Anyone other than a plague bringer (including another alchemist) who drinks a plague vial must make a saving throw against the vial's DC or become nauseated for 1 hour. Unless he learns how to brew a mutagen by taking the mutagen discovery (see Ultimate Magic), he can never benefit from a mutagen and reacts to it as if he were a non-alchemist. At any particular time, a plague bringer can only be under the effect of either a plague vial or a mutagen (not both); drinking another immediately ends the effects of any ongoing plague vial or mutagen. All limitations to mutagens apply to plague vials as if they were the same substance. The infuse mutagen discovery and persistent mutagen class ability apply to plague vials. The sticky poison discovery applies to a weapon infected with a plague vial. The plague vial is a disease effect.|AlchemistLVL|10+AlchemistLVL/2+Int"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Plague Bringer ~ Disease Resistance", at_level: 2, description: Some("The plague bringer gains a +%1 bonus on all saving throws against disease.|(1+(AlchemistLVL-2)/3)*2"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Plague Bringer ~ Disease Immunity", at_level: 10, description: Some("The plague bringer becomes completely immune to disease (including magical diseases)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Plague Bringer ~ Discoveries", at_level: 1, description: Some("The following discoveries complement the plague bringer archetype: explosive bomb, precise bombs, smoke bomb (Advanced Player's Guide); breath weapon bomb, fast bomb, nauseating bomb (Ultimate Combat); plague bomb (Ultimate Magic)."), benefit: None },
            ],
        },
        // Alchemist Archetype ~ Saboteur -- arg_abilities_class.lst:153
        ArchetypeSwapEntry {
            key: "Alchemist Archetype ~ Saboteur",
            subject: "Alchemist",
            archetype_name: "Saboteur",
            description: Some("The saboteur is an alchemist who specializes in destroying the plans, materials, and allies of his enemies. A saboteur has focused his alchemical research toward new ways to conceal his presence, sow confusion, and blow up large structures."),
            source_page: Some("p.36"),
            prerequisites: Some(&["PRECLASS:1,Alchemist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Alchemist Archetype ~ Saboteur],[!PREABILITY:1,CATEGORY=Archetype,TYPE.AlchemistMutagen]", "PREFACT:1,TEMPLATES,IsGnome=true"]),
            replaces: Some(&["AlchemistMutagen"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Saboteur ~ Class Skills", at_level: 1, description: Some("A saboteur adds Knowledge (engineering) to his list of class skills and removes Knowledge (nature) from his list of class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Saboteur ~ Chameleon Mutagen", at_level: 1, description: Some("A saboteur discovers how to create a special elixir that he can imbibe in order to heighten his ability to move undetected and his mobility at the cost of his physical might. When consumed, the elixir causes the saboteur's skin to change color to match the background and causes his hands and feet to secrete a sticky residue. This grants him a circumstance bonus on Stealth checks equal to %1, as well as granting him a climb speed of %2, for 10 minutes per saboteur level. In addition, while the chameleon extract is in effect, the saboteur takes a -2 penalty to his Strength. This ability replaces mutagen. A saboteur who drinks an alchemist's mutagen is treated as a non-alchemist. All limitations to mutagens apply to chameleon mutagen as if it were the same ability. The infuse mutagen discovery and persistent mutagen class ability apply to the chameleon mutagen.|1+(AlchemistLVL-2)/2|MOVEBASE/2"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Saboteur ~ Discoveries", at_level: 1, description: Some("The following discoveries complement the saboteur archetype: acid bomb, concussive bomb, delayed bomb, explosive bomb, fast bombs, inferno bomb, madness bomb, precise bombs, smoke bomb, stink bomb (Advanced Player's Guide); explosive missile, immolation bomb, siege bomb (Ultimate Combat); blinding bomb, cognatogen, confusion bomb, tanglefoot bomb (Ultimate Magic)."), benefit: None },
            ],
        },
        // Barbarian Archetype ~ Feral Gnasher -- arg_abilities_class.lst:529
        ArchetypeSwapEntry {
            key: "Barbarian Archetype ~ Feral Gnasher",
            subject: "Barbarian",
            archetype_name: "Feral Gnasher",
            description: Some("Feral gnashers grow up in the wild, either raised by animals or scraping by on their own, and soon learn to fend for themselves. These barbarians often utilize pieced-together armor and fight with their sharp teeth and whatever improvised weapons are within reach."),
            source_page: Some("p.116"),
            prerequisites: Some(&["PRECLASS:1,Barbarian=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Barbarian Archetype ~ Feral Gnasher],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BarbarianWeaponProficiencies,TYPE.BarbarianArmorProficiencies,TYPE.BarbarianFastMovement,TYPE.BarbarianTrapSense1,TYPE.BarbarianTrapSense2,TYPE.BarbarianTrapSense3,TYPE.BarbarianTrapSense4,TYPE.BarbarianImprovedUncannyDodge]", "PREFACT:1,TEMPLATES,IsGoblin=true"]),
            replaces: Some(&["BarbarianProficiencies", "BarbarianFastMovement", "BarbarianRagePower2", "BarbarianTrapSense1", "BarbarianTrapSense2", "BarbarianTrapSense3", "BarbarianTrapSense4", "BarbarianImprovedUncannyDodge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Feral Gnasher ~ Weapon and Armor Proficiency", at_level: 1, description: Some("A feral gnasher loses all martial weapon proficiencies except for greatclub and loses proficiency with medium armor."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Gnasher ~ Savage Bite", at_level: 1, description: Some("The feral gnasher gains a savage bite attack. This is a primary natural attack. If the goblin already has the hard head, big teeth racial trait, the damage increases."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Gnasher ~ Impromptu Armament", at_level: 2, description: Some("The feral gnasher gains Throw Anything as a bonus feat and can pick up an unattended object that can be wielded in one hand as a free action. Additionally, the feral gnasher can take Catch Off-Guard in place of a rage power. This replaces the rage power gained at second level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Gnasher ~ Lockjaw", at_level: 3, description: Some("The feral gnasher gains the grab ability with her bite attack. A feral gnasher can use this ability on a creature up to one size category larger than she is."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Gnasher ~ Improvised Weapon Mastery", at_level: 5, description: Some("The feral gnasher gains Improvised Weapon Mastery as a bonus feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Gnasher ~ Improved Lockjaw", at_level: 6, description: Some("As long as a feral gnasher is controlling the grapple with her lockjaw attack, she does not gain the grappled condition, but is unable to move or use her mouth for anything other than grappling."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Gnasher ~ Greater Lockjaw", at_level: 9, description: Some("The size of a creature a feral gnasher is able to use her lockjaw's grab ability on increases by %1.|FeralGnasherLockjawSize"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Gnasher ~ Wicked Improvisation", at_level: 12, description: Some("The feral gnasher becomes more capable with improvised weapons and natural attacks. The feral gnasher gains a +%1 competence bonus on damage rolls when using natural attacks or improvised weapons while raging.  This increase is not precision damage and is thus multiplied on a critical hit.|FeralGnasherWickedImprovisationBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Gnasher ~ Rage Powers", at_level: 1, description: Some("The following rage powers complement the feral gnasher archetype: increased damage reduction, scent, superstition (Core Rulebook); beast totem, beast totem (greater), beast totem (lesser) (Advanced Player's Guide); eater of magic, ghost rager (Ultimate Combat)."), benefit: None },
            ],
        },
        // Barbarian Archetype ~ Hateful Rager -- arg_abilities_class.lst:246
        ArchetypeSwapEntry {
            key: "Barbarian Archetype ~ Hateful Rager",
            subject: "Barbarian",
            archetype_name: "Hateful Rager",
            description: Some("From a young age, many half-orcs are treated cruelly, bullied, ridiculed, and made outcasts. While some hide their shame, others foster a deep, burning hatred that they channel into a raw fury and unleash against their enemies. These half-orcs are called hateful ragers. A hateful rager has the following class features."),
            source_page: Some("p.54"),
            prerequisites: Some(&["PRECLASS:1,Barbarian=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Barbarian Archetype ~ Hateful Rager],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BarbarianRagePower2,TYPE.BarbarianRagePower8,TYPE.BarbarianRagePower14,TYPE.BarbarianRagePower20,TYPE.BarbarianImprovedUncannyDodge,TYPE.BarbarianTrapSense3]", "PREFACT:1,TEMPLATES,IsHalfOrc=true"]),
            replaces: Some(&["BarbarianRagePower2", "BarbarianRagePower8", "BarbarianRagePower14", "BarbarianRagePower20", "BarbarianImprovedUncannyDodge", "BarbarianTrapSense3"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Hateful Rager ~ Reduced Rage", at_level: 2, description: Some("A hateful rager only gains 1 additional round of rage per day instead of the normal 2 additional rounds of rage per day."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Hateful Rager ~ Favored Enemy", at_level: 2, description: Some("A hateful rager selects a favored enemy. This ability works identically to the ranger ability of the same name.  While raging, the hateful rager makes every effort to fight a favored enemy rather than other opponents. If aware of the presence of a favored enemy, the hateful rager must make a DC 20 Will save each round to attack another creature; failure means the barbarian must attack the favored enemy or move closer to that enemy. She may freely attack creatures preventing her from reaching that favored enemy (regardless of whether they are actively trying to prevent her attacks or merely in the way). She can avoid harmful obstacles normally in her attempts to reach the target and is not forced to take the shortest route."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Hateful Rager ~ Feed the Rage", at_level: 5, description: Some("A hateful rager gains 1 additional round of rage for each favored enemy she knocks unconscious or kills in combat. These current rounds of rage can only be used to add to the duration of her rage, and disappear when the rage ends."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Hateful Rager ~ Amplified by Hate", at_level: 9, description: Some("A hateful rager adds half her favored enemy bonus to the DC of any rage power she uses against a favored enemy."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Hateful Rager ~ Rage Power", at_level: 1, description: Some("The following rage powers complement the hateful rager archetype: intimidating glare, roused anger, terrifying howl (Core Rulebook); come and get me, inspire ferocity, overbearing advance, reckless abandon (Advanced Player's Guide)."), benefit: None },
            ],
        },
        // Bard Archetype ~ Prankster -- arg_abilities_class.lst:152
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Prankster",
            subject: "Bard",
            archetype_name: "Prankster",
            description: Some("The prankster sees humor as the highest form or art, and pranks as the highest form of humor. In addition to setting friends up for light-hearted pranks, the prankster can use his quick wit and cruel sense of humor to enrage foes before incapacitating them with a clever turn of phrase."),
            source_page: Some("p.26"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Prankster],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardFascinate,TYPE.BardSuggestion,TYPE.BardMassSuggestion,TYPE.BardLoreMaster]", "PREFACT:1,TEMPLATES,IsGnome=true"]),
            replaces: Some(&["BardFascinate", "BardSuggestion", "BardMassSuggestion", "BardLoreMaster"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Prankster ~ Bardic Performance", at_level: 1, description: Some("A prankster's bardic performance functions like a bard's, but some of its performances are exchanged for those listed below."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Prankster ~ Mock", at_level: 1, description: Some("The prankster can use his performance to cause up to %1 creatures to become furious with him. Each creature to be mocked must be within 90 feet, able to see, hear, and understand the prankster, and capable of paying attention to him. The prankster must also be able to see the creatures affected. Each creature within range receives a Will save (DC %2) to negate the effect. If a creature's saving throw succeeds, the prankster cannot successfully mock that creature for 24 hours. If its saving throw fails, the creature is angered by the performance and seeks to harm the prankster. While the prankster maintains the mocking, the target takes a -2 penalty on all attack rolls and skill checks until it has successfully attacked the prankster with a melee or ranged attack, or has harmed the creature with a spell that deals damage. Mock is an enchantment (compulsion) mind-affecting ability. Mock relies on audible and visual components in order to function.|1+(BardLVL-1)/3|10+BardLVL/2+CHA"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Prankster ~ Punchline", at_level: 6, description: Some("The prankster can use his performance to tell a punchline to amuse a creature he has already mocked, goading it into hideous laughter (as the spell). Using this ability does not disrupt the mock effect, but it does require a standard action to activate (in addition to the free action to continue the mock effect). A prankster can use this ability more than once against an individual creature during an individual performance. Telling a punchline does not count against a prankster's daily use of bardic performance. A Will saving throw (DC %1) negates the effect. This ability affects only a single creature. Punchline is an enchantment (compulsion), mind affecting, language dependent ability and relies on audible components.|10+BardLVL/2+CHA"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Prankster ~ Mass Punchline", at_level: 18, description: Some("This ability functions just like punchline, but allows a prankster of 18th level or higher to use hideous laughter simultaneously against any number of creatures that he has mocked. This ability replaces the mass suggestion ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Prankster ~ Swap", at_level: 1, description: Some("A prankster can steal an object from a creature and replace it with another object of the same size or smaller that the prankster has in his hand. This functions as the steal combat maneuver (Advanced Player's Guide 322), but the prankster does not provoke an attack of opportunity, and may use his Sleight of Hand check in place of his combat maneuver check. If the prankster's check exceeds the target's CMD by 10 or more, the target is unaware the swap has been made until it tries to use the swapped object or the end of its next turn (whichever happens first)."), benefit: None },
            ],
        },
        // Bard Archetype ~ Shadow Puppeteer -- arg_abilities_class.lst:999
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Shadow Puppeteer",
            subject: "Bard",
            archetype_name: "Shadow Puppeteer",
            description: Some("A shadow puppeteer invokes amazing and terrifying shadow puppet shows, producing supernatural effects by creating and manipulating shadow."),
            source_page: Some("p.210"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Shadow Puppeteer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardInspireCourage1,TYPE.BardInspireCourage2,TYPE.BardInspireCourage3,TYPE.BardInspireCourage4,TYPE.BardInspireCompetence1,TYPE.BardInspireCompetence2,TYPE.BardInspireCompetence3,TYPE.BardInspireCompetence4,TYPE.BardInspireCompetence5,TYPE.BardInspireCompetence6]", "PREFACT:1,TEMPLATES,IsWayang=true"]),
            replaces: Some(&["BardInspireCourage1", "BardInspireCourage2", "BardInspireCourage3", "BardInspireCourage4", "BardInspireCompetence1", "BardInspireCompetence2", "BardInspireCompetence3", "BardInspireCompetence4", "BardInspireCompetence5", "BardInspireCompetence6"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Shadow Puppeteer ~ Bardic Performance", at_level: 1, description: Some("A shadow puppeteer gains the following types of bardic performance. The character must be able to perform shadow puppetry in order to activate any of these abilities. Shadow puppetry uses Perform (act), and requires a light source. These abilities replace all levels of inspire courage and inspire competence."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shadow Puppeteer ~ Shadow Servant", at_level: 1, description: Some("At 1st level, the puppeteer can create a shadow servant to perform simple tasks. The shadow servant is identical to an unseen servant (caster level %1), except it appears as a formless shadow. Shadow servant relies on visual components.|BardLVL"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shadow Puppeteer ~ Shadow Puppets", at_level: 1, description: Some("The shadow puppeteer can use bardic performance to create one quasi-real shadowy creature resembling a monster from the summon monster %2 list. These shadowy creatures otherwise work like shadow conjuration, and targets interacting with them get a Will saving throw (DC %1) to treat them as only 20%% real.|10+BardLVL/2+CHA|1+(BardLVL-1)/3"), benefit: None },
            ],
        },
        // Bard Archetype ~ Watersinger -- arg_abilities_class.lst:807
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Watersinger",
            subject: "Bard",
            archetype_name: "Watersinger",
            description: Some("The watersinger's song reaches from the depths of his soul into the elemental waters from which life first sprang. His voice commands water, bending and shaping it to his desire."),
            source_page: Some("p.176"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Watersinger],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardFascinate,TYPE.BardSuggestion,TYPE.BardMassSuggestion,TYPE.BardInspireCompetence,TYPE.BardLoreMaster]", "PREFACT:1,TEMPLATES,IsUndine=true"]),
            replaces: Some(&["BardFascinate", "BardSuggestion", "BardMassSuggestion", "BardInspireCompetence", "BardLoreMaster"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Watersinger ~ Bardic Performance", at_level: 1, description: Some("A watersinger has some unique bardic performances, which replace some of the standard bardic performances as listed in each entry. These bardic performances follow all the general rules and restrictions of a bard's bardic performances."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Watersinger ~ Watersong", at_level: 1, description: Some("A watersinger can use bardic performance to manipulate and control the shape of water within 30 feet. A successful Perform check allows the bard to animate and control %2 5-foot-cubes of water (these cubes must be adjacent to each other). The watersinger can command the water to take various forms, bend, rise, fall, or sustain a shape, and can make it support weight as if it were solid ice. For example, the watersinger could create a pillar of water (to provide cover), ladder, channel, bridge, stairs, slide, and so on. The manipulated water is as slippery as normal ice. This ability cannot create forms more fragile or complex than what could be carved in normal ice. While under the bard's control, the water has hardness %1 and 3 hit points per inch of thickness. The manipulated water retains its shape for 1 round after the bard stops spending bardic performance rounds to maintain it.|BardLVL/3|1+(BardLVL/5)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Watersinger ~ Waterstrike 1", at_level: 3, description: Some("The watersinger can spend 1 round of bardic performance to command any water he is currently manipulating with his watersong performance to lash out and strike an opponent with a slam attack. The watersinger uses his base attack bonus and Charisma bonus to make this attack, and deals 1d6 points of bludgeoning damage plus his Charisma bonus. The attack can originate from any square of water the bard is manipulating, and the water can get a flanking bonus or help a combatant get one, but cannot make attacks of opportunity. The water can make multiple attacks per round if your base attack bonus allows you to do so."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Watersinger ~ Waterstrike 2", at_level: 10, description: Some("The watersinger can spend 1 round of bardic performance to command any water he is currently manipulating with his watersong performance to lash out and strike an opponent with a slam attack. The watersinger uses his base attack bonus and Charisma bonus to make this attack, and deals 1d8 points of bludgeoning damage plus his Charisma bonus. The attack can originate from any square of water the bard is manipulating, and the water can get a flanking bonus or help a combatant get one, but cannot make attacks of opportunity. The water can make multiple attacks per round if your base attack bonus allows you to do so."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Watersinger ~ Waterstrike 3", at_level: 15, description: Some("The watersinger can spend 1 round of bardic performance to command any water he is currently manipulating with his watersong performance to lash out and strike an opponent with a slam attack. The watersinger uses his base attack bonus and Charisma bonus to make this attack, and deals 2d6 points of bludgeoning damage plus his Charisma bonus. The attack can originate from any square of water the bard is manipulating, and the water can get a flanking bonus or help a combatant get one, but cannot make attacks of opportunity. The water can make multiple attacks per round if your base attack bonus allows you to do so."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Watersinger ~ Waterstrike 4", at_level: 20, description: Some("The watersinger can spend 1 round of bardic performance to command any water he is currently manipulating with his watersong performance to lash out and strike an opponent with a slam attack. The watersinger uses his base attack bonus and Charisma bonus to make this attack, and deals 2d8 points of bludgeoning damage plus his Charisma bonus. The attack can originate from any square of water the bard is manipulating, and the water can get a flanking bonus or help a combatant get one, but cannot make attacks of opportunity. The water can make multiple attacks per round if your base attack bonus allows you to do so."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Watersinger ~ Lifewater", at_level: 5, description: Some("The watersinger can spend 1 round of bardic performance as a standard action to manipulate the water, blood, and other fluids within a creature's body, causing the target to become sickened for 1d4 rounds. Alternatively, he may use this ability to attempt a reposition combat maneuver, using %1 as his CMB. This ability has a range of 30 feet, only works on creatures whose bodies contain fluid, and does not affect creatures that are immune to critical hits.|BAB+CHA"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Watersinger ~ Watersinger Spells", at_level: 1, description: Some("A watersinger adds certain waterthemed spells to his spell list. He adds these abilities to his spell list as soon as his bard level allows him to cast spells of that spell level."), benefit: None },
            ],
        },
        // Cleric Archetype ~ Demonic Apostle -- arg_abilities_class.lst:508
        ArchetypeSwapEntry {
            key: "Cleric Archetype ~ Demonic Apostle",
            subject: "Cleric",
            archetype_name: "Demonic Apostle",
            description: Some("In order to survive, the drow threw in their lot with demon lords. Thus, demon worship is common among the drow, and so are ranks of demonic apostles, who gain magical insight from their dark lords and crush their chaotic masters' enemies by channeling demonic energy."),
            source_page: Some("p.104"),
            prerequisites: Some(&["PRECLASS:1,Cleric=1", "PREDOMAIN:1,Chaos,Evil,Demon", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Cleric Archetype ~ Demonic Apostle],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ClericChannelEnergy1,TYPE.ClericChannelEnergy2,TYPE.ClericChannelEnergy3,TYPE.ClericChannelEnergy4,TYPE.ClericChannelEnergy5,TYPE.ClericChannelEnergy6,TYPE.ClericChannelEnergy7,TYPE.ClericChannelEnergy8,TYPE.ClericChannelEnergy9,TYPE.ClericChannelEnergy10]", "PREFACT:1,TEMPLATES,IsDrow=true"]),
            replaces: Some(&["ClericChannelEnergy1", "ClericChannelEnergy2", "ClericChannelEnergy3", "ClericChannelEnergy4", "ClericChannelEnergy5", "ClericChannelEnergy6", "ClericChannelEnergy7", "ClericChannelEnergy8", "ClericChannelEnergy9", "ClericChannelEnergy10"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Demonic Apostle ~ Demonic Magic", at_level: 1, description: Some("A demonic apostle must choose to channel negative energy, and must select either the Chaos or Evil domain or the Demon subdomain (Advanced Player's Guide 89) as her sole domain."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Demonic Apostle ~ Demonic Familiar", at_level: 1, description: Some("The demonic apostle gains a familiar as a wizard equal to her cleric level, or if she already has a familiar, her cleric levels stack to determine the familiar's abilities. At 3rd level, her familiar gains the fiendish template (Bestiary 294). At 7th level, the demonic apostle exchanges her familiar for a quasit without the need to take the Improved Familiar feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Demonic Apostle ~ Demonic Channel", at_level: 1, description: Some("The demonic apostle can channel demonic energy to damage creatures of lawful and good alignment, or, at higher levels, bolster the abilities of chaotic evil allies. Channeling this energy causes a 30-foot-radius burst centered on the cleric. Creatures within the burst that are lawful or good take %1d6 points of damage. Creatures that take damage from the channeled demonic energy receive a Fortitude save to halve the damage. The DC of this save is %2. Lawful good creatures take a -2 penalty on this saving throw. At 5th level, chaotic evil allies within the burst are affected as if targeted by a rage spell with a duration of 1 round. At 9th level, lawful or good enemies are also sickened for 1d6 rounds if they fail their saving throw against the demonic channel. Channeling demonic energy is a standard action that does not provoke attacks of opportunity.|DemonicApostleChannelEnergyDmg|DemonicApostleChannelEnergySave"), benefit: None },
            ],
        },
        // Cleric Archetype ~ Fiendish Vessel -- arg_abilities_class.lst:786
        ArchetypeSwapEntry {
            key: "Cleric Archetype ~ Fiendish Vessel",
            subject: "Cleric",
            archetype_name: "Fiendish Vessel",
            description: Some("Many clerics pray to or make evil bargains with fiendish powers, devoting body and soul to the insane plans and wicked aims of their despicable patrons. But these mortal clerics are often just shallow beings searching for quick power or the caress of true and final oblivion-few truly grasp the full scope of the entities they worship. Fiendish vessels, through their fiendish heritage, share an innate connection with their patron, and that connection grants them understanding and power."),
            source_page: Some("p.170"),
            prerequisites: Some(&["PREALIGN:CE,NE,LE", "PREALIGN:Deity", "PRECLASS:1,Cleric=1", "PREDOMAIN:1,Daemon Subdomain,Demon Subdomain (Evil),Demon Subdomain (Chaos),Devil Subdomain (Evil),Devil Subdomian (Law)", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Cleric Archetype ~ Fiendish Vessel],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ClericChannelEnergy1,TYPE.ClericChannelEnergy2,TYPE.ClericChannelEnergy3,TYPE.ClericChannelEnergy4,TYPE.ClericChannelEnergy5,TYPE.ClericChannelEnergy6,TYPE.ClericChannelEnergy7,TYPE.ClericChannelEnergy8,TYPE.ClericChannelEnergy9,TYPE.ClericChannelEnergy10]", "PREFACT:1,TEMPLATES,IsTiefling=true"]),
            replaces: Some(&["ClericChannelEnergy1", "ClericChannelEnergy2", "ClericChannelEnergy3", "ClericChannelEnergy4", "ClericChannelEnergy5", "ClericChannelEnergy6", "ClericChannelEnergy7", "ClericChannelEnergy8", "ClericChannelEnergy9", "ClericChannelEnergy10"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Fiendish Vessel ~ Alignment", at_level: 1, description: Some("Unlike normal clerics, a fiendish vessel's alignment must match her patron's."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fiendish Vessel ~ Domains", at_level: 1, description: Some("A fiendish vessel must select the Daemon, Demon, or Devil subdomain (Advanced Player's Guide 88- 90) as one of her domain choices, based on the fiendish patron she chooses to serve."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fiendish Vessel ~ Channel Evil", at_level: 1, description: Some("A fiendish vessel, rather than channeling positive or negative energy, instead channels the pure evil power of her fiendish patron. This ability is similar to channeling negative energy, but instead of healing undead and dealing damage to living creatures, this blast of evil energy automatically heals evil creatures and debilitates good creatures within its burst. Channeling this evil causes a burst that affects all creatures in a 30-foot radius centered on the fiendish vessel. In the case of evil creatures, the amount of damage healed is equal to %1d4 points of damage. Good creatures in the burst receive a Will saving throw to negate this damage. Good creatures that fail their saving throws are sickened for 1d4 rounds. Good creatures with a number of Hit Dice less than or equal %2 that fail their saving throws are nauseated for 1 round and then sickened for 1d4 rounds instead. The DC of this save is equal to %3. Neutral creatures are unaffected by this burst of evil energy. A fiendish vessel may channel this energy %4 times per day. Doing so is a standard action that does not provoke attacks of opportunity. A fiendish vessel can choose whether or not to include herself in this effect. A fiendish vessel must present her unholy symbol or use her familiar as the divine focus for this ability. For the purposes of feats that affect channel energy, this ability counts as channeling negative energy. If the feat changes the way the fiendish vessel channels or deals damage with her channeling, use the amount of damage this ability heals evil creatures to determine the damage-dealing potential of the affected ability. For instance, if a 5th-level fiendish vessel takes the Channel Smite feat, her channeling deals an additional 3d4 points of damage to living creatures on a successful hit (though they may save to negate the damage). This ability replaces channel energy.|min(10,1+(ClericLVL-1)/2)|ClericLVL-5|10+ClericLVL/2+CHA|3+CHA"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fiendish Vessel ~ Fiendish Familiar", at_level: 3, description: Some("A fiendish vessel's patron rewards her with a fiendish servant. The fiendish vessel gains an imp, quasit, or cacodaemon familiar based on the patron she worships. If she worships Asmodeus or an archdevil, she gets an imp; if she worships a demon lord, she gets a quasit; and if she worships one of the Four Horsemen, she gains a cacodaemon. This ability is identical to the wizard's arcane bond with a familiar and the Improved Familiar feat, using the fiendish vessel's character level in place of the wizard level. This tiny fiend acts like a perverse, manifest moral compass. Furthermore, this familiar can act as a living divine focus and unholy symbol for her spellcasting if the fiendish vessel so desires, which means that when she uses her channel evil ability, its burst can be centered on the familiar instead, as long as that familiar is within 30 feet and line of sight. A fiendish vessel's familiar tends to be fawning and subservient to the fiendish vessel. Should her familiar die, the fiendish vessel's patron replaces the familiar with an identical one within 1 week, without the need for a special ritual."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fiendish Vessel ~ Fiendish Augury", at_level: 3, description: Some("The fiendish vessel can ask the familiar whether a particular course of action will bring good or bad results for her in the immediate future. This ability acts like the augury spell, with a caster level equal to the fiendish vessel's level, with the familiar acting as the mouthpiece for the spell. This ability can be used once per day."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fiendish Vessel ~ Fiendish Divination", at_level: 9, description: Some("The fiendish vessel can use a more powerful form of divination to gain intelligence from her patron through her fiendish familiar. This ability acts like the divination spell, with a caster level equal to the fiendish vessel's level; the familiar acts as the mouthpiece for the spell. This ability can be used once per day."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fiendish Vessel ~ Extra Divination", at_level: 13, description: Some("The fiendish vessel can gain intelligence from her patron more often each day. She can use fiendish divination up to 3 times per day."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Fiendish Vessel ~ Fiendish Summoning", at_level: 1, description: Some("When casting summon monster spells, a fiendish vessel is limited to summoning fiendish creatures and evil outsiders of the same alignment as her patron."), benefit: None },
            ],
        },
        // Cleric Archetype ~ Forgemaster -- arg_abilities_class.lst:32
        ArchetypeSwapEntry {
            key: "Cleric Archetype ~ Forgemaster",
            subject: "Cleric",
            archetype_name: "Forgemaster",
            description: Some("Forgemasters are priestly dwarves who are ritual casters and expert enchanters, able to produce their rune-graven armaments with astonishing speed."),
            source_page: Some("p.15"),
            prerequisites: Some(&["PRECLASS:1,Cleric=1", "PREDEITYDOMAIN:1,Artifice", "PREDOMAIN:1,Artifice", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Cleric Archetype ~ Forgemaster],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ClericChannelEnergy]", "PREFACT:1,TEMPLATES,IsDwarf=true"]),
            replaces: Some(&["ClericChannelEnergy"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Forgemaster ~ Artificer", at_level: 1, description: Some("A forgemaster gains only one domain, which must be the Artifice domain (not including subdomains). If she worships a deity, it must grant the Artifice domain."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Forgemaster ~ Steel Spells", at_level: 1, description: Some("A forgemaster adds the following spells to her spell list: 1st-crafter's curse, crafter's fortune, lead blades; 2nd-chill metal, heat metal, shatter; 3rd-keen edge, versatile weapon; 8th-iron body, repel metal or stone."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Forgemaster ~ Divine Smith", at_level: 1, description: Some("Whenever a forgemaster casts a spell that targets a weapon, shield, or armor, the spell takes effect at +1 caster level. If the spell has one or more metamagic feats applied, she reduces the total level adjustment to the spell by 1 (minimum 0)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Forgemaster ~ Runeforger", at_level: 1, description: Some("A forgemaster may inscribe mystical runes upon a suit of armor, shield, or weapon as full-round action, using this ability %1 times per day. These runes last %2 rounds, but inscribing the same rune twice on an item increases this duration to %2 minutes, three times to %20 minutes, and four times to %2 hours. Erase affects runes as magical writing. Only one type of rune marked with an asterisk (*) may be placed on an item at any given time. This ability replaces channel energy.|3+INT|ClericLVL"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Forgemaster ~ Craft Magic Arms and Armor", at_level: 3, description: Some("The forgemaster gains this as a bonus feat at 3rd level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Forgemaster ~ Master Smith", at_level: 5, description: Some("At 5th level, a forgemaster can craft mundane metal items quickly, using half their gp value to determine progress, and can craft magical metal items in half the normal amount of time."), benefit: None },
            ],
        },
        // Druid Archetype ~ Feral Child -- arg_abilities_class.lst:344
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Feral Child",
            subject: "Druid",
            archetype_name: "Feral Child",
            description: Some("Some youths, abandoned in the wilderness and then raised by animals, are so connected with their adoptive home and family that they become feral. Suspicious of civilized society, these foundlings often choose allegiance to the wild over their human forebears. A feral child has the following class features."),
            source_page: Some("p.74"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Feral Child],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidTracklessStep.DruidThousandFaces.DruidWildShape.DruidResistNaturesLure.DruidVenomImmunity.DruidTimelessBody]", "PREFACT:1,TEMPLATES,IsHuman=true"]),
            replaces: Some(&["DruidTracklessStep", "DruidThousandFaces", "DruidWildShape", "DruidResistNaturesLure", "DruidVenomImmunity", "DruidTimelessBody"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Feral Child ~ Weapon and Armor Proficiency", at_level: 1, description: Some("A feral child loses proficiency with the scimitar, scythe, and sickle and with shields."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Child ~ Class Skills", at_level: 1, description: Some("A feral child adds Acrobatics to her list of class skills and removes Fly and Profession from her list of class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Child ~ Illiteracy", at_level: 1, description: Some("A feral child is unable to read and write, though she may learn by taking 1 rank of Linguistics. She does not gain Druidic as a free language and cannot select Sylvan as a bonus language."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Child ~ Improved Unarmed Strike", at_level: 1, description: Some("A feral child gains Improved Unarmed Strike as a bonus feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Child ~ Beast Family", at_level: 1, description: Some("A feral child may choose one specific type of animal as the type that raised her. She gains a +2 circumstance bonus on Handle Animal and wild empathy checks with animals of that type, and she can communicate with them as if using a continual speak with animals spell-like ability, but this ability is nonmagical."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Child ~ Nature Bond", at_level: 1, description: Some("A feral child must select an animal companion as her nature bond."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Child ~ Favored Terrain", at_level: 3, description: Some("A feral child gains the favored terrain ability as a ranger of her class level. A feral child may not choose urban as a favored terrain."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Child ~ Native Cunning", at_level: 3, description: Some("A feral child gains trap sense as a barbarian of equal level, and in her favored terrain, she immediately receives a Perception check to notice traps within 10 feet. In addition, at 3rd level and every three levels thereafter, she may choose one combat maneuver, and gains a bonus equal to her trap sense bonus to her CMD against that maneuver."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Child ~ Native Fortitude", at_level: 4, description: Some("A feral child gains a +1 bonus on saving throws against disease, exhaustion, fatigue, fear, and poison. When she is in her favored terrain, she instead applies her favored terrain bonus on such saving throws. She also recovers from ability damage, exhaustion, and fatigue at twice the normal rate."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Child ~ Native Call 1", at_level: 9, description: Some("When in her favored terrain, for any summon nature's ally spells a feral child uses to summon animals that are native to that terrain, she treats the duration of the spell as if she were 2 levels higher. At 17th level, when the feral child uses summon nature's ally spells to summon such animals, those animals gain a +2 bonus to both their Strength and Constitution ability scores. This stacks with the effects of the Augmented Summoning feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feral Child ~ Native Call 2", at_level: 17, description: Some("When the feral child uses summon nature's ally spells to summon such animals, those animals gain a +2 bonus to both their Strength and Constitution ability scores. This stacks with the effects of the Augmented Summoning feat."), benefit: None },
            ],
        },
        // Druid Archetype ~ Naga Aspirant -- arg_abilities_class.lst:913
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Naga Aspirant",
            subject: "Druid",
            archetype_name: "Naga Aspirant",
            description: Some("The naga aspirant follows the ancient beliefs and engages in the rituals of a druidic sect dedicated to the transcendence of her nagaji form through absolute devotion to nagas and naga gods. Through acting as a herald to the naga deities, the aspirant is rewarded with the ability to unlock her ultimate spirit form and become a true naga."),
            source_page: Some("p.196"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Naga Aspirant],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidSpontaneousCasting,TYPE.DruidResistNaturesLure,TYPE.DruidWildShape,TYPE.DruidVenomImmunity,TYPE.DruidAThousandFaces,TYPE.DruidTimelessBody]", "PREFACT:1,TEMPLATES,IsNagaji=true"]),
            replaces: Some(&["DruidSpontaneousCasting", "DruidResistNaturesLure", "DruidWildShape", "DruidVenomImmunity", "DruidAThousandFaces", "DruidTimelessBody"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Naga Aspirant ~ Aspirant's Bond", at_level: 1, description: Some("A naga aspirant gains a spiritual connection to the serpentine deities worshiped by the nagas. At 1st level, and each time she gains a druid level, she may add one of the following spells to her druid spell list. 0-acid splash, bleed, daze, mage hand, open/close, ray of frost; 1st-charm person, divine favor, expeditious retreat, mage armor, magic missile, ray of enfeeblement, shield, shield of faith, silent image, true strike; 2nd-acid arrow, detect thoughts, invisibility, mirror image, scorching ray, see invisibility; 3rd- dispel magic, displacement, fireball, lightning bolt, suggestion; 4th-divine power, greater invisibility."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Naga Aspirant ~ Aspirant's Enlightenment", at_level: 4, description: Some("A naga aspirant gains a +4 bonus on saving throws against the spell-like abilities, supernatural abilities, and poison of nagas. This ability replaces resist nature's lure."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Naga Aspirant ~ Naga Shape", at_level: 6, description: Some("The naga aspirant can use her wild shape ability (gained at 4th level, as normal) to assume the form of a true naga. This effect functions in a similar manner to a shapechange spell with the following exception. The druid's true naga form is unique, representing her personal evolution. When taking naga form, the nagaji's body transforms into that of a large serpent, though she keeps her own head. The naga aspirant loses her limbs and her size increases by one category, granting her a +4 size bonus to Strength and Constitution, a -2 penalty to Dexterity, and a +2 enhancement bonus to her natural armor bonus. She gains a +10 enhancement bonus to land speed and a bite attack that deals 1d6 points of damage. She can cast verbal spells in this form, but cannot cast spells with other components without metamagic or feats such as Natural Spell. This otherwise works like and replaces wild shape."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Naga Aspirant ~ Augmented Form", at_level: 9, description: Some("A naga aspirant can choose one of the augmented form selections to enhance her naga form. Once chosen, this augmentation cannot be changed and always applies to her naga form. The caster level for these abilities is equal to %1, and unless otherwise stated, the DC is equal to %2.|DruidLVL|10+DruidLVL/2+CHA"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Naga Aspirant ~ True Naga", at_level: 20, description: Some("The naga aspirant metamorphoses into a unique naga. Her wild shape form becomes her natural form, though she can transform into her original nagaji shape at will. Her creature type permanently changes to aberration. This ability replaces wildshape (at will)."), benefit: None },
            ],
        },
        // Druid Archetype ~ Sky Druid -- arg_abilities_class.lst:720
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Sky Druid",
            subject: "Druid",
            archetype_name: "Sky Druid",
            description: Some("Some druids develop ties not to a particular landscape, but instead to the endless blue expanse of the skies. Such are the sky druids, who are more at home soaring through air than standing on the ground."),
            source_page: Some("p.158"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Sky Druid],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidWoodlandStride,TYPE.DruidNatureBond,TYPE.DruidNaturesLure,TYPE.DruidTracklessStep,TYPE.DruidWildShape,TYPE.DruidVenomImmunity,TYPE.DruidThousandFaces]", "PREFACT:1,TEMPLATES,IsSylph=true"]),
            replaces: Some(&["DruidNatureSense", "DruidWoodlandStride", "DruidResistNaturesLure", "DruidTracklessStep", "DruidWildShape", "DruidVenomImmunity", "DruidThousandFaces"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Sky Druid ~ Weapon and Armor Proficiency", at_level: 1, description: Some("A sky druid loses medium armor proficiency."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sky Druid ~ Nature Bond", at_level: 1, description: Some("A sky druid who chooses an animal companion must select one with a fly speed. If choosing a domain, the sky druid must choose from the Air, Animals, Liberation, and Weather domains, or subdomains appropriate to those domains."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sky Druid ~ Sky's Embrace", at_level: 2, description: Some("A sky druid no longer takes falling damage, as though she were constantly under the effect of feather fall. Additionally, she may take ranks in the Fly skill regardless of whether she has a natural fly speed, and may use her Fly skill in place of Acrobatics when making jump checks."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sky Druid ~ Resist Storm", at_level: 4, description: Some("A sky druid gains a +4 bonus on saving throws against spells with the air or electricity descriptors and against effects that control or modify the weather (such as sleet storm)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sky Druid ~ Skymaster", at_level: 5, description: Some("A sky druid can use the fly spell (self only) for %1 minutes per day. These minutes do not need to be consecutive.|DruidLVL"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sky Druid ~ Wild Shape", at_level: 6, description: Some("A sky druid gains the ability to use wild shape. When a sky druid takes the form of a creature with a fly speed, this ability functions at her class level + 1. For all other forms, her effective druid level for the ability is equal to her actual sky druid level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sky Druid ~ Soaring Form", at_level: 9, description: Some("A sky druid is no longer affected by altitude sickness or natural or magical wind."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sky Druid ~ Into the Wild Blue", at_level: 13, description: Some("A sky druid gains a fly speed equal to twice her base land speed (good maneuverability)."), benefit: None },
            ],
        },
        // Druid Archetype ~ Treesinger -- arg_abilities_class.lst:108
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Treesinger",
            subject: "Druid",
            archetype_name: "Treesinger",
            description: Some("Elves live far longer than other common races, and a single elf may see whole empires rise and fall. Given the impermanence of the cultures around them, it's small wonder that some elves turn to the timeless growth of nature for solace, finding allies among the great trees themselves, and even leading the forest's plants into combat."),
            source_page: Some("p.25"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Treesinger],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidNatureBond,TYPE.DruidWildEmpathy,TYPE.DruidWildShape]", "PREFACT:1,TEMPLATES,IsElf=true"]),
            replaces: Some(&["DruidNatureBond", "DruidWildEmpathy", "DruidWildShape"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Treesinger ~ Plant Bond", at_level: 1, description: Some("A treesinger forms a mystic bond with plant life."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Treesinger ~ Green Empathy", at_level: 1, description: Some("A treesinger can improve the attitude of a plant creature. This ability functions just like a Diplomacy check made to improve the attitude of a person. The treesinger rolls 1d20 and adds her druid level and her Charisma modifier to determine the wild empathy check result. The typical wild plant creature has a starting attitude of indifferent. To use green empathy, the treesinger and the plant creature must be within 30 feet of one another under normal conditions. Generally, influencing a plant creature in this way takes 1 minute but, as with influencing people, it might take more or less time. A treesinger can also use this ability to influence an animal, but she takes a -4 penalty on the check."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Treesinger ~ Wild Shape", at_level: 4, description: Some("The treesinger gains the ability to wild shape. This ability functions at her actual druid level. A treesinger cannot use wild shape to adopt an animal or elemental form. Instead, when she gains this ability, she can assume the form of a Small or Medium plant. This functions as plant shape I, except the treesinger does not yet gain access to the constrict or poison abilities of the plant form assumed. At 8th level, the treesinger's wild shape gains the full range of abilities available from plant shape I. At 10th level, a treesinger can assume the form of a Large or Tiny plant. Her wild shape ability now functions like plant shape II. At 12th level, a treesinger can assume the form of a Huge plant. Her wild shape ability now functions like plant shape III. This ability replaces, and otherwise functions like, the normal druid wild shape ability."), benefit: None },
            ],
        },
        // Druid Archetype ~ Undine Adept -- arg_abilities_class.lst:798
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Undine Adept",
            subject: "Druid",
            archetype_name: "Undine Adept",
            description: Some("An undine adept dedicates herself to preserving the knowledge of the first undines and ensuring her people's ancient connections to the natural world remain undisturbed. They serve as the keepers of the roots of the undine people and as their protectors."),
            source_page: Some("p.176"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREDOMAIN:1,Community,Water", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Undine Adept],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidWoodlandStride,TYPE.DruidTracklessStep,TYPE.DruidResistNaturesLure,TYPE.DruidWildShape,TYPE.DruidVenomImmunity]", "PREFACT:1,TEMPLATES,IsUndine=true"]),
            replaces: Some(&["DruidWoodlandStride", "DruidTracklessStep", "DruidResistNaturesLure", "DruidWildShape", "DruidVenomImmunity"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Undine Adept ~ Domains", at_level: 1, description: Some("An undine adept who chooses a domain must choose the Community or Water domain, or any subdomain of those domains."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Undine Adept ~ Amphibious", at_level: 2, description: Some("The undine adept gains the aquatic subtype and the amphibious universal monster ability, allowing her to breathe water or air."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Undine Adept ~ Augment Summoning", at_level: 3, description: Some("Any creature with the water subtype the undine adept summons with either summon monster or summon nature's ally gains the benefits of the Augment Summoning feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Undine Adept ~ Resist Water's Call", at_level: 4, description: Some("An undine adept gains a +4 bonus on saving throws against the spell-like and supernatural abilities of outsiders with the aquatic or water subtype, fey with the aquatic or water subtype, and spells and effects with the water descriptor."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Undine Adept ~ Wild Shape", at_level: 6, description: Some("An undine adept gains the ability to use wild shape. When an undine takes the form of a creature with the aquatic or water subtype, this ability functions at level %1. For all other forms, her effective druid level for the ability is equal to her actual undine adept level.|DruidLVL+1"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Undine Adept ~ Commune with Water Spirits", at_level: 9, description: Some("An undine adept adds commune to her druid spell list. She may use this spell whether she worships a deity or elemental forces."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Airborne Ambusher -- arg_abilities_class.lst:948
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Airborne Ambusher",
            subject: "Fighter",
            archetype_name: "Airborne Ambusher",
            description: Some("Driven by suspicion and hatred, strix doggedly guard their territories, making deadly use of their flight. Using swift strikes from above, strix plummet onto their foes with lethal force."),
            source_page: Some("p.201"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ Airborne Ambusher],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterBravery,TYPE.FighterWeaponTraining1,TYPE.FighterWeaponTraining2,TYPE.FighterWeaponTraining3,TYPE.FighterWeaponTraining4]", "PREFACT:1,TEMPLATES,IsStrix=true"]),
            replaces: Some(&["FighterBravery", "FighterWeaponTraining1", "FighterWeaponTraining2", "FighterWeaponTraining3", "FighterWeaponTraining4"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Airborne Ambusher ~ Weapon and Armor Proficiency", at_level: 1, description: Some("An airborne ambusher is not proficient with heavy armor or tower shields."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Airborne Ambusher ~ Class Skills", at_level: 1, description: Some("An airborne ambusher adds Fly to his list of class skills and removes Climb from his list of class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Airborne Ambusher ~ Combat Flyer", at_level: 2, description: Some("An airborne ambusher may use his fighter bonus feats to select Flyby Attack and Hover."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Airborne Ambusher ~ Aerobatics", at_level: 5, description: Some("An airborne ambusher may make a Fly check instead of an Acrobatics check to move through a threatened area or an enemy's space."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Airborne Ambusher ~ Flying Dodger", at_level: 9, description: Some("When an airborne ambusher flies at least half its fly speed on its turn, it gains a +%1 dodge bonus to AC for 1 round.|1+if(FighterLVL>=17,3,if(FighterLVL>=11,1,0))"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Airborne Ambusher ~ Plummeting Charge", at_level: 13, description: Some("If an airborne ambusher flies at least half its fly speed as part of a charge, it gains a +%1 racial bonus on the attack roll (in addition to the normal charge bonus) and a +%2 bonus on its critical confirmation roll.|2+if(FighterLVL>=17,2,0)|4+if(FighterLVL>=17,2,0)"), benefit: None },
            ],
        },
        // Fighter Archetype ~ Cavern Sniper -- arg_abilities_class.lst:493
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Cavern Sniper",
            subject: "Fighter",
            archetype_name: "Cavern Sniper",
            description: Some("Perfectly at home in the darkness, the cavern sniper capitalizes on stealth and ranged attacks imbued with his spell-like abilities to harass his opponents. The cavern sniper focuses on surprise, his innate magical abilities, and poison to take down unwary foes. The cavern sniper has the following class features."),
            source_page: Some("p.104"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ Cavern Sniper],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterBravery,TYPE.FighterWeaponTraining1,TYPE.FighterWeaponTraining2,TYPE.FighterWeaponTraining3,TYPE.FighterWeaponTraining4,TYPE.FighterWeaponMastery]", "PREFACT:1,TEMPLATES,IsDrow=true"]),
            replaces: Some(&["FighterBravery", "FighterWeaponTraining1", "FighterWeaponTraining2", "FighterWeaponTraining3", "FighterWeaponTraining4", "FighterWeaponMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Cavern Sniper ~ Class Skills", at_level: 1, description: Some("The cavern sniper adds Stealth to his list of class skills and removes Intimidate from his list of class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cavern Sniper ~ Imbued Shot", at_level: 1, description: Some("The cavern sniper gains the ability to imbue his arrows or bolts with the effect of one of his drow faerie fire, darkness, or deeper darkness spell-like abilities (provided he has access to the ability) as a swift action. When such an arrow or bolt is fired, the spell's area is centered where the arrow or bolt lands. If the target of the attack has a space larger than 5 feet, the cavern sniper can choose which square of the creature's space is the center of the spell-like ability's effect, as long as that square is within line of sight of the cavern sniper. The cavern sniper can instead choose to target a single square within line of sight with an imbued arrow or bolt, and uses that square as the center of the spell-like ability's area of effect on a hit (AC 5). The arrow must be fired during the round it was imbued, or the spell-like ability is wasted. If the arrow or bolt misses, the use of the spell-like ability is wasted."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cavern Sniper ~ Silent Shooter", at_level: 2, description: Some("The cavern sniper gains a bonus on Stealth checks made when loading a bow or crossbow, poisoning ammunition, and making sniping attempts."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cavern Sniper ~ Quick and Deadly", at_level: 4, description: Some("The cavern sniper can move at full speed while using Stealth at no penalty and can apply poison to a single arrow or crossbow bolt as a swift action."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cavern Sniper ~ Sniper Training", at_level: 5, description: Some("The cavern sniper chooses the bow or crossbow weapon group and gains a bonus on attack rolls and damage rolls."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cavern Sniper ~ Greater Imbued Shot", at_level: 9, description: Some("The cavern sniper gains two extra uses of both his faerie fire and darkness spell-like abilities, but can only use these extra uses to imbue arrows and bolts with the imbued shot class feature."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cavern Sniper ~ Weapon Mastery", at_level: 20, description: Some("Any attacks made by a bow or crossbow automatically confirm all critical threats and have their damage multiplier increased by 1. In addition, you cannot be disarmed while wielding a weapon of this type."), benefit: None },
            ],
        },
        // Fighter Archetype ~ Dirty Fighter -- arg_abilities_class.lst:647
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Dirty Fighter",
            subject: "Fighter",
            archetype_name: "Dirty Fighter",
            description: Some("The dirty fighter laughs at concepts like honor and fair play. He cares only for victory, no matter how he achieves it, and spends as much time mastering sneaky combat maneuvers as he does drilling with weapons or learning how to wear armor. A dirty fighter has the following class features."),
            source_page: Some("p.140"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ Dirty Fighter],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterBravery,TYPE.FighterWeaponTraining1,TYPE.FighterWeaponTraining2,TYPE.FighterWeaponTraining3,TYPE.FighterWeaponTraining4]", "PREFACT:1,TEMPLATES,IsOrc=true"]),
            replaces: Some(&["FighterBravery", "FighterWeaponTraining1", "FighterWeaponTraining2", "FighterWeaponTraining3", "FighterWeaponTraining4"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Dirty Fighter ~ Sidestep", at_level: 2, description: Some("The dirty fighter learns how to evade his enemies when they react to his combat maneuvers. He gains a +%1 dodge bonus to his AC against attacks of opportunity provoked by him while attempting a combat maneuver.|1+(FighterLVL-2)/4"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dirty Fighter ~ Maneuver Training", at_level: 5, description: Some("The dirty fighter becomes a master of dirty tricks. He gains a +2 bonus on dirty trick combat maneuver checks and +2 to his CMD when he is the target of a dirty trick combat maneuver."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dirty Fighter ~ Speedy Tricks", at_level: 9, description: Some("The dirty fighter has perfected how to quickly perform dirty tricks. He can make a dirty trick combat maneuver as an attack instead of a standard action."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dirty Fighter ~ Double Tricks", at_level: 13, description: Some("When performing a combat maneuver, the dirty fighter may apply %1 different conditions to his target instead of one. Each penalty condition requires a separate action to remove. This ability replaces weapon training 3 and 4.|DirtyFighterDirtyTricksCount"), benefit: None },
            ],
        },
        // Fighter Archetype ~ Foehammer -- arg_abilities_class.lst:31
        ArchetypeSwapEntry {
            key: "Fighter Archetype ~ Foehammer",
            subject: "Fighter",
            archetype_name: "Foehammer",
            description: Some("While the axe is the most famous dwarven weapon, the hammer is at the heart of dwarves' heritage as forgemasters and warriors alike."),
            source_page: Some("p.15"),
            prerequisites: Some(&["PRECLASS:1,Fighter=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Fighter Archetype ~ Foehammer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.FighterArmorTraining1,TYPE.FighterArmorTraining2,TYPE.FighterArmorTraining3,TYPE.FighterArmorTraining4,TYPE.FighterWeaponTraining1,TYPE.FighterWeaponTraining2,TYPE.FighterWeaponTraining3,TYPE.FighterWeaponTraining4,TYPE.FighterArmorMastery,TYPE.FighterWeaponMastery]", "PREFACT:1,TEMPLATES,IsDwarf=true"]),
            replaces: Some(&["FighterArmorTraining1", "FighterArmorTraining2", "FighterArmorTraining3", "FighterArmorTraining4", "FighterWeaponTraining1", "FighterWeaponTraining2", "FighterWeaponTraining3", "FighterWeaponTraining4", "FighterArmorMastery", "FighterWeaponMastery"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Foehammer ~ Sledgehammer", at_level: 3, description: Some("At 3rd level, a foehammer wielding a hammer gains a +2 circumstance bonus on combat maneuver checks made to bull rush, overrun, sunder, or trip. This ability replaces armor training 1."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Foehammer ~ Weapon Training", at_level: 5, description: Some("A foehammer must select hammers and does not gain weapon training with other groups, though his weapon training bonus improves by +1 every four levels after 5th."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Foehammer ~ Hammer to the Ground 1", at_level: 7, description: Some("When a foehammer succeeds at a bull rush combat maneuver, he can make a trip combat maneuver at the end of the bull rush. If he does not move with the target, the force of his blow may still trip his foe, but he takes a -5 penalty on the combat maneuver check to trip."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Foehammer ~ Hammer to the Ground 2", at_level: 15, description: Some("Any creature a foehammer successfully bull rushes is automatically knocked prone at the end of the bull rush."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Foehammer ~ Rhythmic Blows", at_level: 9, description: Some("Each time that a foehammer hits a target, he gains a +1 bonus on attack rolls against that target. This bonus stacks with each hit against that target, but lasts only until the end of the foehammer's turn."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Foehammer ~ Piledriver", at_level: 11, description: Some("As a standard action, a foehammer may make a single melee attack with a weapon from the hammer weapon training group. If the attack hits, he may make a bull rush or trip combat maneuver against the target of his attack as a free action that does not provoke an attack of opportunity."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Foehammer ~ Ground Breaker", at_level: 13, description: Some("As a full-round action, a foehammer may strike the ground with his hammer. If the attack deals more damage than the floor's hardness, the space he occupies and all adjacent squares become difficult terrain. Creatures in those squares, except for the foehammer, are knocked prone (DC 15 Reflex negates)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Foehammer ~ Hammer Master", at_level: 17, description: Some("Any combat feats a foehammer has learned with any weapon from the hammer weapon training group (e.g., Improved Critical, Weapon Focus) apply to all weapons from that group."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Foehammer ~ Devastating Blow", at_level: 19, description: Some("As a standard action, a foehammer may make a single melee attack with a weapon from the hammer weapon training group at a -5 penalty. If the attack hits, it is treated as a critical threat. Weapon special abilities that only activate on a critical hit do not activate if this critical hit is confirmed."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Foehammer ~ Weapon Mastery", at_level: 20, description: Some("A foehammer must choose a weapon from the hammer group. [Needs to be implemented]"), benefit: None },
            ],
        },
        // Inquisitor Archetype ~ Exarch -- arg_abilities_class.lst:30
        ArchetypeSwapEntry {
            key: "Inquisitor Archetype ~ Exarch",
            subject: "Inquisitor",
            archetype_name: "Exarch",
            description: Some("The gruff traditionalism of most dwarves finds its apex in those who adhere to a strict orthodoxy rooted in ancient principles and practices and who are not amenable whatsoever to change or innovation."),
            source_page: Some("p.13"),
            prerequisites: Some(&["PRECLASS:1,Inquisitor=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Inquisitor Archetype ~ Exarch],[!PREABILITY:1,CATEGORY=Archetype,TYPE.InquisitorMonsterLore,TYPE.InquisitorDetectAlignment,TYPE.InquisitorBane,TYPE.InquisitorSecondJudgment,TYPE.InquisitorGreaterBane,TYPE.InquisitorThirdJudgment]", "PREFACT:1,TEMPLATES,IsDwarf=true"]),
            replaces: Some(&["InquisitorMonsterLore", "InquisitorDetectAlignment", "InquisitorBane", "InquisitorSecondJudgment", "InquisitorGreaterBane", "InquisitorThirdJudgment"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Exarch ~ Spells", at_level: 1, description: Some("Exarchs cannot cast spells with the chaotic descriptor."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Exarch ~ Inflexible Will", at_level: 1, description: Some("An exarch gains a +2 bonus on saving throws against confusion and insanity effects and effects with the chaotic descriptor."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Exarch ~ Detect Chaos", at_level: 1, description: Some("At will, an exarch can use detect chaos."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Exarch ~ Fearsome Jurist", at_level: 5, description: Some("An exarch can imbue one of her weapons with the jurist or menacing weapon special ability as a swift action, and may switch between these properties as a swift action. When using either special ability, her weapon's critical threat range doubles against chaotic creatures. This does not stack with keen edge, Improved Critical, or similar effects."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Exarch ~ Aura of Repetition", at_level: 8, description: Some("Once per day while using her judgment, an exarch can project an aura of repetition, as the Toil subdomain power (Advanced Player's Guide 97). If the exarch takes Artifice (Toil) as her domain, the save DC of her aura increases by 2 but its duration does not increase."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Exarch ~ Double Jeopardy", at_level: 12, description: Some("Whenever an exarch uses her fearsome jurist ability, she may choose to affect two weapons, with one gaining the jurist weapon special ability and the other the menacing special ability as above. Both special abilities may be combined in a single weapon, whose critical threat range doubles. This does not stack with keen edge, Improved Critical, or similar effects."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Exarch ~ Aura of Reversion", at_level: 16, description: Some("While using her judgment, an exarch can project a 30-foot-radius emanation for a number of rounds per day equal to her inquisitor level. Any creature other than the exarch that is using a transmutation effect within this aura at the beginning of its turn becomes sickened, or sickened and nauseated if using a polymorph effect, including the change shape ability (Fortitude negates; DC %1). Continuous effects from permanent magical items do not cause this effect. Within the aura, dispel checks against transmutation effects gain a +4 bonus. This ability cannot be used simultaneously with aura of repetition.|10+InquisitorLVL/2+WIS"), benefit: None },
            ],
        },
        // Inquisitor Archetype ~ Immolator -- arg_abilities_class.lst:600
        ArchetypeSwapEntry {
            key: "Inquisitor Archetype ~ Immolator",
            subject: "Inquisitor",
            archetype_name: "Immolator",
            description: Some("The immolator puts her pyromaniacal urges to work in the service of a deity. She brings burning retribution down upon the enemies of her faith, consigning their souls to the sacrificial flames. An immolator has the following class features."),
            source_page: Some("p.124"),
            prerequisites: Some(&["PRECLASS:1,Inquisitor=1", "PREDEITYDOMAIN:1,Fire", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Inquisitor Archetype ~ Immolator],[!PREABILITY:1,CATEGORY=Archetype,TYPE.InquisitorJudgmentSmiting,TYPE.InquisitorBane,TYPE.InquisitorGreaterBane,TYPE.InquisitorTrueJudgement]", "PREFACT:1,TEMPLATES,IsIfrit=true"]),
            replaces: Some(&["InquisitorJudgmentSmiting", "InquisitorBane", "InquisitorGreaterBane", "InquisitorTrueJudgement"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Immolator ~ Servant of the Flame", at_level: 1, description: Some("An immolator must worship a deity whose portfolio includes the Fire domain. An immolator who selects the Fire domain (or one of its associated subdomains, if available) uses her domain powers at +1 caster level (this stacks with the ifrit's fire affinity racial trait)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Immolator ~ Immolation", at_level: 1, description: Some("The immolator channels purifying flame to consume her enemies. When dealing fire damage to an opponent, she treats the target's fire resistance as %1 lower than normal (minimum 0).|(1+(InquisitorLVL-1)/5)*5"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Immolator ~ Burnt Offering 1", at_level: 5, description: Some("As a swift action, an immolator can imbue one of her weapons with the flaming weapon special ability. Any creature slain by this weapon burns with magical flame; its body turns to ash, though its equipment is not harmed. This special ability only functions while the immolator wields the weapon. This ability lasts for %1 rounds per day. These rounds do not need to be consecutive.|InquisitorLVL"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Immolator ~ Burnt Offering 2", at_level: 12, description: Some("As a swift action, an immolator can imbue one of her weapons with the flaming burst weapon special ability. Any creature slain by this weapon burns with magical flame; its body turns to ash, though its equipment is not harmed. This special ability only functions while the immolator wields the weapon. This ability lasts for %1 rounds per day. These rounds do not need to be consecutive.|InquisitorLVL"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Immolator ~ Judgment by Fire", at_level: 20, description: Some("An immolator can call fiery judgment down upon a foe during combat. Whenever an immolator uses her judgment ability, she can invoke a judgment by fire on a foe as a swift action. Once declared, the immolator can make a single melee (or ranged attack, if the foe is within 30 feet) against the target. If the attack hits, the attack deals fire damage instead of weapon damage, and the target must make a successful Fortitude save or die (creatures immune to fire do not have to save). The DC of this save is %1. Regardless of whether the save is successful, the target creature is immune to the immolator's judgment by fire ability for 24 hours. Creatures killed in this manner explode in a burst of fire, dealing 10d6 points of fire damage to every creature within 5 feet (Reflex save for half damage, DC %1). Once this ability has been used, it cannot be used again for 1d4 rounds.|10+Inquisitor/2+WIS"), benefit: None },
            ],
        },
        // Inquisitor Archetype ~ Kinslayer -- arg_abilities_class.lst:469
        ArchetypeSwapEntry {
            key: "Inquisitor Archetype ~ Kinslayer",
            subject: "Inquisitor",
            archetype_name: "Kinslayer",
            description: Some("Appalled and guilt-ridden by the horrific circumstances of her birth, a kinslayer dedicates herself to eradicating the very creatures whose blood flows within her veins. She spends her life hunting and slaying those vampiric monsters for whom humans have become prey."),
            source_page: Some("p.98"),
            prerequisites: Some(&["PRECLASS:1,Inquisitor=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Inquisitor Archetype ~ Kinslayer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.InquisitorJudgmentDestruction,TYPE.InquisitorDetectAlignment]", "PREFACT:1,TEMPLATES,IsDhampir=true"]),
            replaces: Some(&["InquisitorJudgmentDestruction", "InquisitorDetectAlignment"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Kinslayer ~ Slayer's Brand", at_level: 1, description: Some("When using this judgment, the kinslayer gains the ability to brand undead creatures with positive energy. To do so, she must make a successful melee touch attack against the undead creature. This attack deals an amount of positive energy damage equal to 1d6 + %1, and burns her personal symbol into the undead creature's flesh, bone, or even its incorporeal form. From that point onward, the kinslayer can sense the existence of the branded creature as if it were the target of a locate creature spell (caster level %2). A slayer's brand lasts until the undead creature is destroyed or until the kinslayer uses this ability on another creature.|KinslayerSlayersBrandDamage|KinslayerSlayersBrandLVL"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Kinslayer ~ Greater Brand", at_level: 1, description: Some("A kinslayer learns to modify her slayer's brand judgment as she gains levels. Whenever she gains the ability to learn a teamwork feat, she can instead opt to learn one of the modifications to her slayer's brand judgment."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Kinslayer ~ Undead Sense", at_level: 2, description: Some("The kinslayer gains the ability to use detect undead as a spell-like ability (caster level %1) at will. If she detects the presence of undead, she can use her monster lore ability to attempt to determine the type of undead detected as well as to reveal any strengths or weaknesses the undead might have. If any of the detected undead are vampires, she gains a bonus of %2 on the check to immediately identify them as such.|KinslayerUndeadSenseLVL|KinslayerUndeadSenseBonus"), benefit: None },
            ],
        },
        // Monk Archetype ~ Gray Disciple -- arg_abilities_class.lst:832
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Gray Disciple",
            subject: "Monk",
            archetype_name: "Gray Disciple",
            description: Some("The gray disciple contemplates the inner voice of duergar magic and the silent eternity of stone, mastering these dual mysteries and combining them to deadly effect."),
            source_page: Some("p.187"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Gray Disciple],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkSlowFall,TYPE.MonkStillMind,TYPE.MonkHighJump,TYPE.MonkBonusFeat6,TYPE.MonkWholenessOfBody,TYPE.MonkAbundantStep,TYPE.MonkQuiveringPalm,TYPE.MonkTongueOfTheSunAndMoon,TYPE.MonkEmptyBody]", "PREFACT:1,TEMPLATES,IsDuergar=true"]),
            replaces: Some(&["MonkSlowFall", "MonkStillMind", "MonkHighJump", "MonkBonusFeat6", "MonkWholenessOfBody", "MonkAbundantStep", "MonkQuiveringPalm", "MonkTongueOfTheSunAndMoon", "MonkEmptyBody"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Gray Disciple ~ Fade from Sight", at_level: 4, description: Some("As a swift action, the gray disciple can become invisible (as the invisibility spell) for 1 round by spending 1 ki point."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Gray Disciple ~ Gray Heart", at_level: 6, description: Some("As a swift action, the gray disciple can enlarge himself (as the enlarge person spell) for 1 minute by spending 1 ki point."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Gray Disciple ~ Born in Darkness", at_level: 7, description: Some("As a standard action, the gray disciple can radiate darkness (as the spell, except originating from the disciple's person) for %1 rounds by spending 1 ki point.|MonkLVL"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Gray Disciple ~ Earth Glide", at_level: 12, description: Some("As a swift action, the gray disciple can spend a ki point to walk through solid stone for 1 round. This functions as the earth glide universal monster ability. The gray disciple may continue earth gliding as long as he spends 1 ki point every round as a swift action. If he ceases earth gliding within a solid object, he is violently ejected and takes 5d6 points of damage. The gray disciple is not harmed by damage caused to material he is earth gliding through, but a stone to flesh spell cast upon it causes violent ejection as described above."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Gray Disciple ~ Entomb", at_level: 15, description: Some("A gray disciple can phase a foe into solid rock, killing it instantly. To use this ability, he expends 1 ki point as part of a bull rush or reposition combat maneuver against a creature adjacent to unworked earth or stone. If the attempt succeeds, the gray disciple pushes his foe inside the rock using his earth glide ability. If the creature succeeds at a Reflex save (DC %1), it is ejected in the nearest open space and takes 5d6 points of damage. If it fails, it dies instantly as its body merges with the surrounding stone. Entomb is usable once per day, but a failed bull rush or reposition attempt does not count as a use of the ability. Entomb has no effect on creatures that can earth glide, are incorporeal, or can otherwise can survive merging with a solid object.|10+MonkLVL/2+WIS"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Gray Disciple ~ Earthen Thrall", at_level: 17, description: Some("A gray disciple can attempt to control a creature with the earth subtype once per day. This ability is treated as dominate monster (DC %1), but is only effective against creatures with the earth subtype, and the gray disciple can keep only a single creature enthralled. If he attempts to control a second creature with this ability, the first creature is automatically released from domination whether or not the second attempt succeeds.|10+MonkLVL/2+WIS"), benefit: None },
            ],
        },
        // Monk Archetype ~ Ironskin Monk -- arg_abilities_class.lst:578
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Ironskin Monk",
            subject: "Monk",
            archetype_name: "Ironskin Monk",
            description: Some("Through discipline and training, an ironskin monk hardens his body to withstand punishing blows. Though slow on his feet, his calloused hands and feet can shatter stone and stagger foes."),
            source_page: Some("p.122"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Ironskin Monk],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkACBonus,TYPE.MonkEvasion,TYPE.MonkKiPool,TYPE.MonkHighJump,TYPE.MonkFastMovement,TYPE.MonkSlowFall,TYPE.MonkImprovedEvasion,TYPE.MonkTongueOfTheSunAndMoon,TYPE.MonkPerfectSelf]", "PREFACT:1,TEMPLATES,IsHobgoblin=true"]),
            replaces: Some(&["MonkACBonus", "MonkEvasion", "MonkKiPool", "MonkHighJump", "MonkFastMovement", "MonkSlowFall", "MonkImprovedEvasion", "MonkTongueOfTheSunAndMoon", "MonkPerfectSelf"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Ironskin Monk ~ Iron Skin", at_level: 1, description: Some("The ironskin monk gains a +%1 bonus to his natural armor. This bonus stacks with any existing natural armor the ironskin monk already has. At 4th level, and every 4 levels thereafter, this bonus increases by +1. This ability replaces the monk's AC bonus ability and the ability to add his Wisdom bonus to his AC.|IronskinMonkIronSkinBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ironskin Monk ~ Bonus Feat", at_level: 1, description: Some("An ironskin monk adds Power Attack to his list of bonus feats. At 6th level, he adds Improved Sunder to the list. At 10th level, he adds Greater Sunder to the list. These bonus feat choices replace Dodge, Mobility, and Spring Attack on his bonus feat list."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ironskin Monk ~ Resilience", at_level: 2, description: Some("An ironskin monk can shake off the physical effects of certain attacks. If he makes a Fortitude saving throw against an attack that has a reduced effect on a successful save, he instead avoids the effect entirely. This ability can be used only if the monk is wearing light armor or no armor. A helpless monk does not gain the benefits of resilience."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ironskin Monk ~ Ki Pool", at_level: 4, description: Some("An ironskin monk can spend 1 point from his ki pool to gain a damage bonus equal to %1 against objects and constructs for 1 round.|IronMonkKiPoolBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ironskin Monk ~ Staggering Blow", at_level: 5, description: Some("An ironskin monk attacking with an unarmed strike can spend 1 point from his ki pool as a free action after a successful critical hit to stagger the creature struck for 1 round (Fort DC %1 negates).|IronskinMonkStaggeringBlowSave"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ironskin Monk ~ Tough as Nails", at_level: 6, description: Some("An ironskin monk gains DR %1/-.  Damage reduction can reduce damage to 0 but not below 0. This ability replaces fast movement and slow fall.|IronskinMonkToughasNailsBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ironskin Monk ~ Evasion", at_level: 9, description: Some("The ironskin monk gains evasion."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ironskin Monk ~ Surefooted", at_level: 17, description: Some("An ironskin monk's speed is not reduced by difficult terrain."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ironskin Monk ~ Unbreakable", at_level: 20, description: Some("An ironskin monk sets aside many of the frailties of mortal flesh. He becomes immune to death effects and stunning. He is not subject to ability damage or ability drain, and has a 75%% chance of ignoring the extra damage dealt by critical hits and sneak attacks. This ability replaces perfect self."), benefit: None },
            ],
        },
        // Monk Archetype ~ Nimble Guardian -- arg_abilities_class.lst:448
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Nimble Guardian",
            subject: "Monk",
            archetype_name: "Nimble Guardian",
            description: Some("Some catfolk monks dedicate their graceful prowess to the defense of others, especially those dedicated to a similar ethos or who prove themselves as stalwart allies of the monk's cause. A nimble guardian has the following class features."),
            source_page: Some("p.92"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Nimble Guardian],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkStillMind,TYPE.MonkPurityOfBody,TYPE.MonkWholenessOfBody,TYPE.MonkImprovedEvasion]", "PREFACT:1,TEMPLATES,IsCatfolk=true"]),
            replaces: Some(&["MonkEvasion", "MonkStillMind", "MonkPurityOfBody", "MonkWholenessOfBody", "MonkImprovedEvasion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Nimble Guardian ~ Defensive Aid", at_level: 2, description: Some("%1 times per day, a nimble guardian can interpose herself between one adjacent ally and an attack or damage dealt in an area of effect. If an adjacent ally is the target of the attack or is required to make a Reflex saving throw against a damaging effect, as an immediate action the nimble guardian can grant that ally a +4 circumstance bonus to AC or on the saving throw against the effect. The nimble guardian must use this ability before the attack roll or saving throw is made. The nimble guardian can only use this ability if he is wearing light or no armor.|NimbleGuardianDefensiveAidTimes"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Nimble Guardian ~ Nimble Reflexes", at_level: 3, description: Some("The nimble guardian gains a +2 bonus on all Reflex saving throws."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Nimble Guardian ~ Defensive Mastery", at_level: 5, description: Some("The nimble guardian gains 3 additional uses of her defensive aid ability per day. Furthermore, if an ally that gained the benefit of a use of defensive aid succeeds her Reflex saving throw, and the effect still deals damage on a successful saving throw, the nimble guardian can spend 1 ki point to negate that damage. Doing so is not an action."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Nimble Guardian ~ Guardian Feline", at_level: 7, description: Some("The nimble guardian can transform himself into a feline creature by spending 2 ki points. The effect lasts for 1 hour or until the nimble guardian changes back. Changing form (to animal or back) is a standard action and does not provoke an attack of opportunity. The chosen form must be some form of feline (cheetah, lion, etc.). This ability is otherwise identical to beast shape II. At 9th level, this ability functions as beast shape III."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Nimble Guardian ~ Evasion", at_level: 9, description: Some("The nimble guardian gains evasion."), benefit: None },
            ],
        },
        // Monk Archetype ~ Student of Stone -- arg_abilities_class.lst:678
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Student of Stone",
            subject: "Monk",
            archetype_name: "Student of Stone",
            description: Some("By following the path of the stone, students of stone give up much of monks' mobility in favor of sheer resilience."),
            source_page: Some("p.146"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Student of Stone],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkEvasion,TYPE.MonkFastMovement,TYPE.MonkHighJump,TYPE.MonkImprovedEvasion,TYPE.MonkAbundantStep,TYPE.MonkPerfectSelf]", "PREFACT:1,TEMPLATES,IsOread=true"]),
            replaces: Some(&["MonkEvasion", "MonkFastMovement", "MonkHighJump", "MonkImprovedEvasion", "MonkAbundantStep", "MonkPerfectSelf"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Student of Stone ~ Hard as Stone", at_level: 2, description: Some("Whenever an opponent rolls to confirm a critical hit against a student of stone, treat the student of stone's AC as +4 higher than normal. This ability replaces evasion."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Student of Stone ~ Strength of Stone", at_level: 3, description: Some("A student of stone learns to draw strength from the earth. So long as both he and his opponent are touching the ground, the student of stone gains a +1 bonus on attack rolls, damage rolls, bull rush and trip combat maneuver rolls, and to his CMD when resisting a bull rush or trip attempt. This ability replaces fast movement."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Student of Stone ~ Bonus Feat", at_level: 6, description: Some("A student of stone adds Elemental Fist (Advanced Player's Guide) to his list of available bonus feats. If the student of stone selects Elemental Fist as a bonus feat, he may only deal acid damage when using the feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Student of Stone ~ Bones of Stone 1", at_level: 7, description: Some("As a swift action, a student of stone can spend 1 ki point to gain DR 2/magic until the beginning of his next turn."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Student of Stone ~ Bones of Stone 2", at_level: 10, description: Some("As a swift action, a student of stone can spend 1 ki point to gain DR 2/chaotic until his next turn."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Student of Stone ~ Bones of Stone 3", at_level: 15, description: Some("As a swift action, a student of stone can spend 1 ki point to gain DR 5/chaotic until his next turn."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Student of Stone ~ Body of Stone", at_level: 9, description: Some("A student of stone gains the benefits of the light fortification armor property.  (When a critical hit or sneak attack is scored on the wearer, there is a 25%% chance that the critical hit or sneak attack is negated and damage is instead rolled normally.)"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Student of Stone ~ Soul of Stone", at_level: 12, description: Some("As a swift action, a student of stone can spend 1 ki point to gain tremorsense %1 feet until his next turn.|(1+MonkLVL/16)*15"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Student of Stone ~ Stone Self", at_level: 20, description: Some("At 20th level, a student of stone becomes an earth outsider. He gains the earth subtype, as well as DR 5/chaotic, burrow speed 20 feet, and tremorsense 20 feet."), benefit: None },
            ],
        },
        // Monk Archetype ~ Treetop Monk -- arg_abilities_class.lst:981
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Treetop Monk",
            subject: "Monk",
            archetype_name: "Treetop Monk",
            description: Some("While many vanaras follow traditional monastic training and traditions, others learn to blend exotic combat and the mysterious forces of ki with the natural world, allowing them to move through trees and overgrowth to deliver devastating attacks."),
            source_page: Some("p.206"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Treetop Monk],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkStillMind,TYPE.MonkPurityOfBody]", "PREFACT:1,TEMPLATES,IsVanara=true"]),
            replaces: Some(&["MonkStillMind", "MonkPurityOfBody"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Treetop Monk ~ Branch Runner", at_level: 3, description: Some("A treetop monk adds half the base speed bonus from his fast movement ability to his racial climb speed."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Treetop Monk ~ Wood Affinity 1 ", at_level: 5, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Treetop Monk ~ Wood Affinity 2 ", at_level: 8, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Treetop Monk ~ Freedom of Movement", at_level: 12, description: Some("A treetop monk may expend 1 point from his ki pool as a swift action to gain the effects of freedom of movement for 1 round."), benefit: None },
            ],
        },
        // Monk Archetype ~ Underfoot Adept -- arg_abilities_class.lst:297
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Underfoot Adept",
            subject: "Monk",
            archetype_name: "Underfoot Adept",
            description: Some("An underfoot adept turns his diminutive stature and unorthodox footwork into a powerful weapon. Effortlessly moving across the battlefield, he ducks under the legs of larger creatures and then topples them with surprising attacks. An underfoot adept has the following class features."),
            source_page: Some("p.65"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Underfoot Adept],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkBonusFeat1,TYPE.MonkStunningFist,TYPE.MonkHighJump]", "PREFACT:1,TEMPLATES,IsHalfling=true"]),
            replaces: Some(&["MonkBonusFeat1", "MonkStunningFist", "MonkHighJump"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Underfoot Adept ~ Underfoot Grace", at_level: 1, description: Some("An underfoot adept uses his size and grace to avoid the attacks of those he passes. When using the Acrobatics skill to avoid attacks of opportunity by moving through a threatened area or an enemy's space, he only takes a -5 penalty when doing so at full speed, instead of the normal -10 penalty."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Underfoot Adept ~ Underfoot Trip", at_level: 1, description: Some("An underfoot adept learns a number of maneuvers and grabs that can cause even the largest opponents to stumble and fall. He gains Improved Trip as a bonus feat, even if he does not meet the requirements. He acts as if he is %1 sizes larger for the purposes of determining the maximum size of creatures he can trip and when determining his CMB and CMD for purposes of a trip combat maneuver.|MonkLVL/4"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Underfoot Adept ~ Improved Underfoot Grace", at_level: 5, description: Some("An underfoot adept's ability to avoid attacks of opportunity against those he passes improves. When using the Acrobatics skill to avoid attacks of opportunity, while moving through a threatened area or through an enemy's space, he takes no penalty when doing so at full speed."), benefit: None },
            ],
        },
        // Monk Archetype ~ Wanderer -- arg_abilities_class.lst:346
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Wanderer",
            subject: "Monk",
            archetype_name: "Wanderer",
            description: Some("Some monks wander the world in humility to learn and to share wisdom and philosophy from their teachers with those they meet, often aiding those who are in need. A wanderer has the following class features."),
            source_page: Some("p.76"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Wanderer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkBonusFeat1.MonkStillMind.MonkSlowFall.MonkHighJump.MonkWholenessOfBody.MonkAbundantStep.MonkDiamondSoul]", "PREFACT:1,TEMPLATES,IsHuman=true"]),
            replaces: Some(&["MonkBonusFeat1", "MonkStillMind", "MonkSlowFall", "MonkHighJump", "MonkWholenessOfBody", "MonkAbundantStep", "MonkDiamondSoul"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Wanderer ~ Class Skills", at_level: 1, description: Some("The wanderer adds Diplomacy, Knowledge (geography), Knowledge (local), Linguistics, and Survival to his list of class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wanderer ~ Far Traveler", at_level: 1, description: Some("The wanderer gains either one additional language known or proficiency in one exotic or martial weapon. At 4th level and every four levels thereafter, the wanderer may gain an additional language known or may retrain her weapon proficiency from this ability to a different exotic or martial weapon."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wanderer ~ Long Walk", at_level: 3, description: Some("The wanderer gains Endurance as a bonus feat, and the feat bonus doubles when he makes Constitution checks because of a forced march. In addition, a wanderer gains a +2 bonus on saving throws against spells and effects that cause exhaustion and fatigue."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wanderer ~ Light Step", at_level: 5, description: Some("A wanderer leaves no trail and cannot be tracked, though he can leave a trail if desired. By spending 1 point from his ki pool, he can use ant haul, feather step, pass without trace, or tireless pursuit as a spell-like ability (with a caster level equal to his monk level)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wanderer ~ Inscrutable", at_level: 5, description: Some("The wanderer gains a supernatural air of mystery. The DC to gain information or insight into the wanderer with Diplomacy, Knowledge skills, or Sense Motive increases by 5. In addition, by spending 1 point from his ki pool, the wanderer gains nondetection for 24 hours with a caster level equal to his monk level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wanderer ~ Wanderer's Wisdom", at_level: 7, description: Some("The wanderer can dispense excellent advice in the form of philosophical proverbs and parables. As a swift action, the wanderer can inspire courage or inspire competence as a bard of his monk level by spending 2 points from his ki pool. This affects one creature within 30 feet and lasts a number of rounds equal to the wanderer's Wisdom modifier (minimum 1 round). This ability is language-dependent."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wanderer ~ Disappear Unnoticed", at_level: 12, description: Some("The wanderer may use Stealth to hide even while being directly observed or when no cover or concealment is available, as long as he is adjacent to at least one creature of his size or larger, by spending 1 point from his ki pool. This effect lasts until the beginning of the wanderer's next turn and may be continued in consecutive rounds by spending 1 ki point each round."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wanderer ~ Free Step", at_level: 13, description: Some("The wanderer gains continuous freedom of movement as a continuous spell-like ability."), benefit: None },
            ],
        },
        // Oracle Archetype ~ Ancient Lorekeeper -- arg_abilities_class.lst:105
        ArchetypeSwapEntry {
            key: "Oracle Archetype ~ Ancient Lorekeeper",
            subject: "Oracle",
            archetype_name: "Ancient Lorekeeper",
            description: Some("The ancient lorekeeper is a repository for all the beliefs and vast knowledge of an elven people. She shows a strong interest in and understanding of histories and creation legends at a young age, and as she matures her calling to serve as the memory of her long-lived people becomes clear to all who know her."),
            source_page: Some("p.24"),
            prerequisites: Some(&["PRECLASS:1,Oracle=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Oracle Archetype ~ Ancient Lorekeeper],[!PREABILITY:1,CATEGORY=Archetype,TYPE.OracleMysterySkills,TYPE.OracleMysterySpell2,TYPE.OracleMysterySpell4,TYPE.OracleMysterySpell6,TYPE.OracleMysterySpell8,TYPE.OracleMysterySpell10,TYPE.OracleMysterySpell12,TYPE.OracleMysterySpell14,TYPE.OracleMysterySpell16,TYPE.OracleMysterySpell18]", "PREFACT:1,TEMPLATES,IsElf=true"]),
            replaces: Some(&["OracleMysterySkills", "OracleMysterySpell2", "OracleMysterySpell4", "OracleMysterySpell6", "OracleMysterySpell8", "OracleMysterySpell10", "OracleMysterySpell12", "OracleMysterySpell14", "OracleMysterySpell16", "OracleMysterySpell18"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Ancient Lorekeeper ~ Class Skills", at_level: 1, description: Some("An ancient lorekeeper adds Knowledge (arcane) and Knowledge (local) to her list of class skills. Whenever she makes a Knowledge check of any kind about a question regarding elves (creatures of the elf subtype), the ancient lorekeeper adds half her class level on her check. This replaces the bonus skills the ancient lorekeeper gains from her mystery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ancient Lorekeeper ~ Elven Arcana", at_level: 2, description: Some("The ancient lorekeeper's mastery of elven legends and philosophy has allowed her to master spells used by elven wizards. These spells are treated as one level higher than their true level for all purposes."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ancient Lorekeeper ~ Mysteries", at_level: 1, description: Some("The following oracle mysteries complement the ancient lorekeeper archetype: Lore, Nature, Waves, Wind (Advanced Player's Guide); Ancestor, Time, Wood (Ultimate Magic)."), benefit: None },
            ],
        },
        // Oracle Archetype ~ Community Guardian -- arg_abilities_class.lst:294
        ArchetypeSwapEntry {
            key: "Oracle Archetype ~ Community Guardian",
            subject: "Oracle",
            archetype_name: "Community Guardian",
            description: Some("The community guardian is chosen to protect and succor the weak and innocent within her community. Her calling also allows her to draw upon and focus the collective will in order to achieve those goals. A community guardian has the following class features."),
            source_page: Some("p.63"),
            prerequisites: Some(&["PREALIGN:LG,NG,CG", "PRECLASS:1,Oracle=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Oracle Archetype ~ Community Guardian],[!PREABILITY:1,CATEGORY=Archetype,TYPE.OracleMysterySkills,TYPE.OracleMysterySpell2,TYPE.OracleMysterySpell4,TYPE.OracleMysterySpell6,TYPE.OracleMysterySpell10,TYPE.OracleMysterySpell12]", "PREFACT:1,TEMPLATES,IsHalfling=true"]),
            replaces: Some(&["OracleMysterySkills", "OracleMysterySpell2", "OracleMysterySpell4", "OracleMysterySpell6", "OracleMysterySpell10", "OracleMysterySpell12"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Community Guardian ~ Recommended Mysteries", at_level: 1, description: Some("ancestor (Ultimate Magic 53), life, lore, nature."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Community Guardian ~ Class Skills", at_level: 1, description: Some("A community guardian adds Knowledge (local), Linguistics, Perception, and Survival to her list of class skills. These replace the additional class skills from her mystery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Community Guardian ~ Bonus Spells", at_level: 1, description: Some("bless water (2nd), consecrate (4th), remove disease (6th), hallow (10th), heroes' feast (12th). These bonus spells replace the oracle's mystery bonus spells from these levels."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Community Guardian ~ Revelations", at_level: 1, description: Some("A community guardian must take the following revelations at the listed levels."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Community Guardian ~ Spirit of Community", at_level: 1, description: Some("As a move action, you call upon the spirits of community. For the next round, you grant every ally within 30 feet a +1 competence bonus on a single skill check (of the ally's choice) that it makes before the end of this revelation's duration. Furthermore, allies within 30 feet can, as a free action, choose to forgo this bonus, and instead grant a single ally a +1 increase to its competence bonus granted by this ability (maximum +5). You can use this ability %1 times per day.|CHA+3"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Community Guardian ~ Renewing Radiance", at_level: 3, description: Some("Once per day you can produce a burst of swirling white light that provides a measure of protection and renewal to allies within 30 feet for 1 round. On their turn, the allies can choose either to gain a +%1 sacred bonus to AC for 1 round or to heal a number of hit points equal to %1d6 + %2 (their choice). If an ally is dying, it is stabilized instead.|1+OracleLVL/7|CHA"), benefit: None },
            ],
        },
        // Oracle Archetype ~ Purifier -- arg_abilities_class.lst:408
        ArchetypeSwapEntry {
            key: "Oracle Archetype ~ Purifier",
            subject: "Oracle",
            archetype_name: "Purifier",
            description: Some("The purifier seeks out signs of possession or mind control that manifest from unwilling (and often unwitting) servants for fiendish corruptors and their mortal minions. A purifier seeks liberation of mind, body, and spirit from the bondage of sin and the taint of the unholy. A purifier gains the following class features."),
            source_page: Some("p.86"),
            prerequisites: Some(&["PRECLASS:1,Oracle=1", "PREFACT:1,TEMPLATES,IsAasimar=true"]),
            replaces: Some(&["OracleRevelation3", "OracleRevelation7", "OracleRevelation11"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Purifier ~ Recommended Mysteries", at_level: 1, description: Some("ancestor, battle, heavens, lore."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Purifier ~ Bonus Spells", at_level: 1, description: Some("A Purifier automatically gains bonus spells."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Purifier ~ Diminished Spellcasting", at_level: 1, description: Some("A purifier can use one fewer spell per day of each level and does not automatically learn cure or inflict spells. Her number of oracle spells known is unchanged."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Purifier ~ See Sin", at_level: 3, description: Some("The purifier gains a bonus of %1 on Sense Motive checks to sense enchantments, which she can make as a full-round action. She also gains a bonus of %1 on Spellcraft checks to identify enchantment school spells and spells with the curse or emotion descriptor.|OracleLVL/2"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Purifier ~ Celestial Armor", at_level: 7, description: Some("The purifier's armor takes on a golden or silvery sheen and becomes light as a feather. Her armor weighs half as much as long as she wears it, and she also gains armor training as a fighter 4 levels lower than her oracle level. At 11th level, a purifier gains heavy armor proficiency."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Purifier ~ Sin Eater", at_level: 11, description: Some("The purifier can consume a curse, enchantment, or emotion effect by touch as a full-round action. She can do this %1 times per day, and must make a Charisma check with a bonus equal of %2 against a DC of 11 + the caster level of the effect (or the Hit Dice of the creator for a supernatural effect). If the check succeeds, the effect is negated; however, the purifier is sickened for 1d4 rounds. If the target is possessed (such as by a magic jar effect or a ghost's malevolence ability), the possessor is forced out on a successful check. Whether the check succeeds or fails, the possessor is sickened for 2d4 rounds.|CHA|OracleLVL"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Purifier ~ Sacred Scourge", at_level: 5, description: Some("The purifier may channel holy power to harm evil outsiders as a cleric of her level using the Alignment Channel feat. She may use this ability %1 times per day.|1+CHA"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Purifier ~ Holy Terror", at_level: 9, description: Some("The purifier may use her sacred scourge to panic evil outsiders as if using the Turn Undead feat against undead."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Purifier ~ Celestial Master", at_level: 13, description: Some("The purifier may use her sacred scourge to compel good outsiders to serve her, as if using the Command Undead feat against undead."), benefit: None },
            ],
        },
        // Oracle Archetype ~ Reincarnated Oracle -- arg_abilities_class.lst:936
        ArchetypeSwapEntry {
            key: "Oracle Archetype ~ Reincarnated Oracle",
            subject: "Oracle",
            archetype_name: "Reincarnated Oracle",
            description: Some("A reincarnated oracle draws her knowledge and power from the experiences of her previous lives. Her memories guide her through a spiritual ascension leading the way to her ultimate incarnation."),
            source_page: Some("p.199"),
            prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Oracle ~ Haunted,Oracle ~ Tongues", "PRECLASS:1,Oracle=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Oracle Archetype ~ Reincarnated Oracle],[!PREABILITY:1,CATEGORY=Archetype,TYPE.OracleMysterySpell2,TYPE.OracleMysterySpell4,TYPE.OracleMysterySpell10,TYPE.OracleMysterySpell16,TYPE.OracleMysterySpell18,TYPE.OracleRevelation1,TYPE.OracleRevelation3,TYPE.OracleRevelation7]", "PREFACT:1,TEMPLATES,IsSamsaran=true"]),
            replaces: Some(&["OracleMysterySpell2", "OracleMysterySpell4", "OracleMysterySpell10", "OracleMysterySpell16", "OracleMysterySpell18", "OracleMysteryRevelation1", "OracleMysteryRevelation3", "OracleMysteryRevelation7"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Reincarnated Oracle ~ Recommended Mysteries", at_level: 1, description: Some("ancestor, lore, time."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Reincarnated Oracle ~ Oracle's Curse", at_level: 1, description: Some("A reincarnated oracle must choose the haunted or tongues curse at 1st level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Reincarnated Oracle ~ Bonus Spells", at_level: 1, description: Some("see alignment (Ultimate Combat, 2nd), detect thoughts (4th), contact other plane (10th), moment of prescience (16th), overwhelming presence (Ultimate Magic, 18th). These spells replace the oracle's mystery bonus spells at these levels."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Reincarnated Oracle ~ Revelations", at_level: 1, description: Some("A reincarnated oracle must take the following revelations at one of the listed levels."), benefit: None },
            ],
        },
        // Oracle Archetype ~ Shigenjo -- arg_abilities_class.lst:747
        ArchetypeSwapEntry {
            key: "Oracle Archetype ~ Shigenjo",
            subject: "Oracle",
            archetype_name: "Shigenjo",
            description: Some("The shigenjo walks the path of enlightenment and transcendence by seeking oneness with the celestial spirits. In doing so, she unlocks the martial potential of her own spiritual power."),
            source_page: Some("p.164"),
            prerequisites: Some(&["PREALIGN:NG,TN,NE", "PRECLASS:1,Oracle=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Oracle Archetype ~ Shigenjo],[!PREABILITY:1,CATEGORY=Archetype,TYPE.OracleMysterySpell2,TYPE.OracleMysterySpell4,TYPE.OracleMysterySpell8,TYPE.OracleMysterySpell12,TYPE.OracleMysterySpell14,TYPE.OracleRevelation7,TYPE.OracleRevelation15,TYPE.OracleRevelation20]", "PREFACT:1,TEMPLATES,IsTengu=true"]),
            replaces: Some(&["OracleMysterySpell2", "OracleMysterySpell4", "OracleMysterySpell8", "OracleMysterySpell12", "OracleMysterySpell14", "OracleRevelation7", "OracleRevelation15", "OracleRevelation20"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Shigenjo ~ Class Skills", at_level: 1, description: Some("A shigenjo adds Knowledge (nature), Knowledge (religion), Knowledge (planes), and Survival to her list of class skills. These replace the additional class skills from her mystery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shigenjo ~ Alignment", at_level: 1, description: Some("Any neutral."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shigenjo ~ Recommended Mysteries", at_level: 1, description: Some("ancestor, battle, fire, heavens, lore, metal, nature, stone, time, waves, wood."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shigenjo ~ Bonus Spells", at_level: 1, description: Some("true strike (2nd), alter self (4th), divine power (8th), magic jar (12th), ki shout (14th, Ultimate Magic), moment of prescience (16th). These bonus spells replace the shigenjo's mystery bonus spells at these levels."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shigenjo ~ Ki Pool", at_level: 7, description: Some("A shigenjo gains a pool of ki points, supernatural energy she can use to accomplish amazing feats. The ki pool is replenished each morning after 8 hours of rest or meditation; these hours do not need to be consecutive. If the shigenjo possesses levels in another class that grants points to a ki pool, ki points gained from the shigenjo class stack with those gained from the other class to determine the total number of ki points in the combined pool, but only one ability score modifier is added to the total. The choice of which score to use is made when the second class ability is gained, and once made, the choice is permanent. The shigenjo can use ki points from this pool to power the abilities of every class she possesses that grants a ki pool. As long as she has at least 1 point in her ki pool, a shigenjo can make a ki strike as a monk of level %1.|OracleLVL-3"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shigenjo ~ Ki Magic", at_level: 7, description: Some("Spend 1 Ki Point to add +1 to the DC of the next spell you cast on your turn."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shigenjo ~ Ki Curse", at_level: 7, description: Some("Spend 1 Ki Point to treat your oracle level as 5 higher for the purpose of determining the effects of your curse for the next round."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shigenjo ~ Ki Insight", at_level: 7, description: Some("Spend 1 Ki Point to gain a +4 insight bonus on Spellcraft checks for 1 round."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shigenjo ~ Quivering Palm", at_level: 15, description: Some("A shigenjo may learn quivering palm as the monk ability of the same name in place of a revelation."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shigenjo ~ Final Revelation", at_level: 20, description: Some("Upon reaching 20th level, you achieve true enlightenment and becomes one with the celestial spirits. You gain the ability to speak with any creature that uses a language. For %1 days, you can ignore the negative effects of extreme weather, starvation, thirst, and exhaustion. If you die, your powerful connection to the celestial realm allows you to be reborn 3 days later (as reincarnate). This replaces the final revelation of the shigenjo's mystery.|WISSCORE"), benefit: None },
            ],
        },
        // Paladin Archetype ~ Redeemer -- arg_abilities_class.lst:247
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Redeemer",
            subject: "Paladin",
            archetype_name: "Redeemer",
            description: Some("As most half-orcs are outcasts, a half-orc paladin recognizes that often those who are monstrous are not necessarily evil and that sometimes even those who are evil became that way because of circumstances and misfortune. Some half-orc paladins take up these misunderstood creatures as their cause, standing up for the monstrous creatures and, when possible, leading them to the light. These paladins are called redeemers. A redeemer has the following class features."),
            source_page: Some("p.55"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Redeemer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinSmiteEvil,TYPE.PaladinDetectEvil,TYPE.PaladinAuraOfResolve,TYPE.PaladinAuraOfJustice]", "PREFACT:1,TEMPLATES,IsHalfOrc=true"]),
            replaces: Some(&["PaladinSmiteEvil", "PaladinDetectEvil", "PaladinAuraOfResolve", "PaladinAuraOfJustice"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Redeemer ~ Merciful Smite", at_level: 1, description: Some("When a redeemer chooses to smite a creature, she can have all of her attacks against the target deal nonlethal damage. She does not take the normal -4 attack roll penalty for using a lethal weapon to deal nonlethal damage. She cannot use this ability to deal nonlethal damage to outsiders with the evil subtype, evil-aligned dragons, or undead creatures (these creatures take lethal damage from her smite)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Redeemer ~ Monstrous Rapport", at_level: 1, description: Some("Redeemers gain a +2 bonus on Diplomacy checks to influence creatures who are commonly considered monstrous. This includes but is not limited to \"monstrous\" races such as goblins and orcs, monstrous humanoids, and other intelligent nonhumanoid monsters."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Redeemer ~ Pact of Peace", at_level: 8, description: Some("A redeemer can force a defeated creature to accept a binding pact of peace as a condition of its surrender, as if using lesser geas. Her caster level for this ability is %1. Rather than assigning a mission or task, the redeemer gives the creature a simple set of prohibitions to protect others. Example geas include \"Leave this city and do not return\" or \"Do not attack caravans.\" The prohibition must be against an area no larger than 300 square miles or one specific group of people (such as a tribe or citizens of a particular city). This ability lasts %1 months.|PaladinLVL"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Redeemer ~ Aura of Mercy", at_level: 11, description: Some("A redeemer can expend two uses of her merciful smite ability to grant the merciful smite ability to all allies within 10 feet, using her bonuses. Allies must use this merciful smite ability by the start of the paladin's next turn and the bonuses last for 1 minute. Using this ability is a free action. Evil creatures gain no benefit from this ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Redeemer ~ Associates", at_level: 1, description: Some("A redeemer may ally with an evil creature as long as she feels the creature is capable of redemption. A redeemer may accept henchmen, followers, or cohorts who are not lawful good provided they demonstrate they are willing to follow her and seek betterment under her tutelage."), benefit: None },
            ],
        },
        // Paladin Archetype ~ Stonelord -- arg_abilities_class.lst:33
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Stonelord",
            subject: "Paladin",
            archetype_name: "Stonelord",
            description: Some("A stonelord is a devoted sentinel of dwarven enclaves, drawing the power of the earth and ancient stone to protect her people."),
            source_page: Some("p.16"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Stonelord],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinSmiteEvil,TYPE.PaladinDivineGrace,TYPE.PaladinDivineHealth,TYPE.PaladinMercy3,TYPE.PaladinMercy9,TYPE.PaladinMercy15,TYPE.PaladinChannelPositiveEnergy,TYPE.PaladinDivineBond,TYPE.PaladinAuraOfJustice,TYPE.PaladinMercy12,TYPE.PaladinMercy18,TYPE.PaladinHolyChampion]", "PREFACT:1,TEMPLATES,IsDwarf=true"]),
            replaces: Some(&["PaladinSmiteEvil", "PaladinDivineGrace", "PaladinDivineHealth", "PaladinMercy3", "PaladinMercy9", "PaladinMercy15", "PaladinChannelPositiveEnergy", "PaladinDivineBond", "PaladinAuraOfJustice", "PaladinMercy12", "PaladinMercy18", "PaladinHolyChampion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Stonelord ~ Stonestrike", at_level: 1, description: Some("Once per day per paladin level, a stonelord can draw upon the power of the living rock. As a swift action, she treats her melee attacks until the beginning of her next turn (whether armed or unarmed) as magical and adamantine, including ignoring hardness up to %1, with a +%2 bonus on attack and damage rolls, as well as on combat maneuver checks. This bonus also applies to her CMD if she or her target is touching the ground or a stone structure|PaladinLVL*2|1+PaladinLVL/5"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Stonelord ~ Heartstone", at_level: 2, description: Some("A stonelord's flesh becomes progressively rockier. She gains a +%1 natural armor bonus to AC and DR %2/adamantine. These benefits are halved when not touching the ground or a stone structure. This ability replaces divine grace.|1+(PaladinLVL-2)/4|HeartStoneDRLevel"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Stonelord ~ Stoneblood 1", at_level: 3, description: Some("A stonelord's vitals begin to calcify and her blood transforms into liquid stone. She adds her paladin level on checks to stabilize at negative hit points and gains a 25%% chance to ignore a critical hit or precision damage. This does not stack with fortification armor or similar effects."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Stonelord ~ Stoneblood 2", at_level: 9, description: Some("A stonelord's vitals begin to calcify and her blood transforms into liquid stone. She adds her paladin level on checks to stabilize at negative hit points and gains a 50%% chance to ignore a critical hit or precision damage and she becomes immune to petrification. This does not stack with fortification armor or similar effects."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Stonelord ~ Stoneblood 3", at_level: 15, description: Some("A stonelord's vitals begin to calcify and her blood transforms into liquid stone. She adds her paladin level on checks to stabilize at negative hit points and gains a 75%% chance to ignore a critical hit or precision damage, she becomes immune to petrification and to bleed and blood drain effects.  This does not stack with fortification armor or similar effects."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Stonelord ~ Defensive Stance", at_level: 4, description: Some("A stonelord gains the defensive stance ability, as a stalwart defender (Advanced Player's Guide 277), and may select one defensive power at 8th level and every four levels thereafter. Levels of stalwart defender stack with her paladin levels when determining the total number of rounds that she can maintain her defensive stance per day. A stonelord does not gain any spells or spellcasting abilities, does not have a caster level, and cannot use spell trigger or spell completion magic items."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Stonelord ~ Earth Channel", at_level: 4, description: Some("A stonelord gains Elemental Channel (earth) as a bonus feat, which she may activate by spending two uses of her lay on hands ability, using her paladin level as her effective cleric level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Stonelord ~ Stone Servant", at_level: 5, description: Some("A stonelord may call a Small earth elemental to her side, as a paladin calls her mount. This earth elemental is Lawful Good in alignment and possesses the celestial template, and it increases in size as the stonelord gains levels, becoming Medium at 8th level, Large at 11th level, Huge at 14th level, Greater at 17th level, and Elder at 20th level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Stonelord ~ Stonebane", at_level: 11, description: Some("When using stonestrike, a stonelord's attack gains the bane weapon special ability against creatures with the earth subtype and constructs or objects made of earth or stone."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Stonelord ~ Phase Strike", at_level: 12, description: Some("A stonelord's stonestrike may pass through stone and metal as if they weren't there. By spending 2 uses of her stonestrike ability, she may ignore any cover less than total cover provided by stone or metal, and she ignores any AC bonus from stone or metal armor or shields as if wielding a brilliant energy weapon. A phase strike cannot damage constructs, objects, or creatures with the earth subtype, but unlike a brilliant energy weapon, it can harm undead."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Stonelord ~ Mobile Defense", at_level: 18, description: Some("A stonelord can make one 5-foot step per round while maintaining her defensive stance."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Stonelord ~ Stone Body", at_level: 20, description: Some("A stonelord's body transforms into living stone. She no longer needs to eat, drink, breathe, or sleep, and she becomes immune to paralysis, poison, and stunning. She is also no longer subject to critical hits or precision-based damage."), benefit: None },
            ],
        },
        // Paladin Archetype ~ Tranquil Guardian -- arg_abilities_class.lst:423
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Tranquil Guardian",
            subject: "Paladin",
            archetype_name: "Tranquil Guardian",
            description: Some("A tranquil guardian is a missionary of peace and tranquility, a soothing voice of succor in a violent and dangerous world."),
            source_page: Some("p.86"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Tranquil Guardian],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinSmiteEvil,TYPE.PaladinAuraOfCourage,TYPE.PaladinAuraOfResolve,TYPE.PaladinAuraOfJustice,TYPE.PaladinHolyChampion]", "PREFACT:1,TEMPLATES,IsAasimar=true"]),
            replaces: Some(&["PaladinSmiteEvil", "PaladinAuraOfCourage", "PaladinAuraOfResolve", "PaladinAuraOfJustice", "PaladinHolyChampion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Tranquil Guardian ~ Touch of Serenity", at_level: 1, description: Some("The tranquil guardian gains Touch of Serenity as a bonus feat, even if she does not meet the prerequisites. At 6th level, and every six levels thereafter, the duration of a tranquil guardian's Touch of Serenity increases by 1 round. Each round on its turn, the target may attempt a new Will save to end the effect. The duration does not stack; only the longest remaining duration applies."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Tranquil Guardian ~ Serene Strike", at_level: 3, description: Some("When a tranquil guardian confirms a critical hit, she may convert all damage from her attack to nonlethal damage, and when she does, she can activate Touch of Serenity through her weapon or unarmed strike. Using serene strike is a free action."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Tranquil Guardian ~ Divine Bond", at_level: 1, description: Some("A tranquil guardian who chooses a weapon as her divine bond may only increase her weapon's enhancement bonus or add the following properties to her weapon: conductive, defending, disruptive, grayflame, or merciful."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Tranquil Guardian ~ Aura of Calm", at_level: 8, description: Some("The tranquil guardian is immune to all spells and spell-like abilities with the emotion descriptor, as well as all fear effects. Each ally within 10 feet of her gains a +4 morale bonus on saving throws against these effects. This ability functions only while the tranquil guardian is conscious, not if she is unconscious or dead."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Tranquil Guardian ~ Waves of Peace", at_level: 11, description: Some("The tranquil guardian may expend 2 uses of her Touch of Serenity to affect each opponent within 5 feet of her with that effect. She does not need to touch the creature for the effect to take hold."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Tranquil Guardian ~ Apostle of Peace", at_level: 20, description: Some("The tranquil guardian's DR increases to 10/evil, and whenever she channels positive energy or uses lay on hands to heal, she heals the maximum possible amount. In addition, any creature struck by her Touch of Serenity, even if it saves, must make an additional Will save at DC of %1 the next time it tries to attack. If it fails this save, the attack (including spells or special abilities) automatically fails.|10+PaladinLVL/2+CHA"), benefit: None },
            ],
        },
        // Ranger Archetype ~ Dusk Stalker -- arg_abilities_class.lst:515
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Dusk Stalker",
            subject: "Ranger",
            archetype_name: "Dusk Stalker",
            description: Some("Hunters and guides through the Shadow Plane, dusk stalkers are rangers that thrive in shadow. Adept at hunting in dusk, darkness, and twilight, these rangers excel at manipulating shadows."),
            source_page: Some("p.110"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ Dusk Stalker],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerFavoredTerrain,TYPE.RangerHuntersBond,TYPE.RangerCamouflage]", "PREFACT:1,TEMPLATES,IsFetchling=true"]),
            replaces: Some(&["RangerFavoredTerrain", "RangerHuntersBond", "RangerCamouflage"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Dusk Stalker ~ Class Skills", at_level: 1, description: Some("The dusk stalker adds Knowledge (planes) to his list of class skills and removes Knowledge (nature) from his list of class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dusk Stalker ~ Shadow Guide", at_level: 1, description: Some("When a dusk stalker gains the favored terrain ability, that ability is modified in the following ways. At 3rd level, a dusk stalker picks his primary terrain normally, but only gains a +1 bonus on those checks while on a plane other than the Shadow Plane, and gains a +3 bonus on those checks while on the Shadow Plane. Each time he chooses to add a bonus in a favored terrain, he gains a +1 bonus on those checks while on a plane other than the Shadow Plane, and gains a +3 bonus on those checks while on the Shadow Plane."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dusk Stalker ~ Shadow Bond", at_level: 4, description: Some("The dusk stalker creates a mystical bond with shadows. The shadows around a dusk stalker weave and swirl, confusing his enemies. When a dusk stalker is fighting in dim light or darkness (magical or otherwise), he gains a +4 insight bonus on Acrobatics checks made to move through an enemy's threatened area or through its space. Furthermore, a number of times per day equal to his Wisdom modifier, the dusk stalker can manipulate shadows in a 5-foot square within 30 feet. That square must be in an area of dim light or darkness (magical or otherwise). Enemies with an Intelligence score within or adjacent to that 5-foot square take a -2 penalty to AC and on Reflex saving throws. The harassing shadows last for 1 round. This is a mindaffecting fear effect."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dusk Stalker ~ Dark Sight", at_level: 12, description: Some("The dusk stalker gains the see in darkness ability."), benefit: None },
            ],
        },
        // Ranger Archetype ~ Wave Warden -- arg_abilities_class.lst:884
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Wave Warden",
            subject: "Ranger",
            archetype_name: "Wave Warden",
            description: Some("The wave warden patrols beneath the sea, preserving the safety and secrets of merfolk communities. Though he fares best beneath the water, dry land is no haven to his quarry."),
            source_page: Some("p.194"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ Wave Warden],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerTrack,TYPE.RangerCombatStyle,TYPE.RangerBonusFeat2,TYPE.RangerBonusFeat6,TYPE.RangerBonusFeat10,TYPE.RangerBonusFeat14,TYPE.RangerBonusFeat18,TYPE.RangerFavoredTerrain,TYPE.RangerWoodlandStride,TYPE.RangerSwiftTracker]", "PREFACT:1,TEMPLATES,IsMerfolk=true"]),
            replaces: Some(&["RangerTrack", "RangerBonusFeat2", "RangerBonusFeat6", "RangerBonusFeat10", "RangerBonusFeat14", "RangerBonusFeat18", "RangerFavoredTerrain", "RangerWoodlandStride", "RangerSwiftTracker", "RangerCombatStyle"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Wave Warden ~ Deep Sentinel", at_level: 1, description: Some("A wave warden adds %1 on Perception checks made to notice creatures underwater.|1+(RangerLVL-1)/2"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wave Warden ~ Aquatic Prowess Feat", at_level: 2, description: Some("At 2nd level and every four levels thereafter, a wave warden selects a bonus feat that improves his prowess in aquatic environments. He can choose these feats even if he does not meet the prerequisites. Initially, he may choose from the following feats: Dodge, Mobility, Net Adept (Ultimate Combat), Net and Trident (Ultimate Combat), Net Maneuvering (Ultimate Combat), Precise Shot, Rapid Reload, Sea Hunter, and Two-Weapon Fighting. At 6th level, he adds Improved Two-Weapon Fighting, Net Trickery (Ultimate Combat), and Spring Attack to the list. At 10th level, he adds Greater Two-Weapon Fighting and Improved Precise Shot to the list. This ability otherwise functions like and replaces the standard ranger's combat style bonus feats, including the limitations on armor worn."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wave Warden ~ Favored Terrain", at_level: 3, description: Some("A wave warden gains water as a favored terrain. His bonus in aquatic terrain is +%1. He does not gain additional favored terrains.|(1+(RangerLVL-3)/5)*2"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wave Warden ~ Seaborn", at_level: 7, description: Some("A wave warden may move through any sort of aquatic growth (such as coral or seaweed) or across a wet surface at his normal speed and without taking damage or suffering any other impairment. Obstacles that are enchanted or magically manipulated to impede motion still affect him."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wave Warden ~ Watery Summons", at_level: 8, description: Some("A wave warden can summon allies once per day as a full-round action. This functions as summon nature's ally %1, except it can only be used to summon creatures with the aquatic or water subtypes. The warden's caster level is %2.|3+(RangerLVL-8)/3|RangerLVL"), benefit: None },
            ],
        },
        // Ranger Archetype ~ Wild Shadow -- arg_abilities_class.lst:197
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Wild Shadow",
            subject: "Ranger",
            archetype_name: "Wild Shadow",
            description: Some("The isolation that some half-elves feel leads them to live a life of isolation amid the wild places of the world. Such rangers stalk the wild like shadows, creating close bonds with the wild itself instead of seeking the solace and aid of companions. While ill at ease within cities and other urban areas, they are adept at using the terrain to tactical advantage; they dart through brambles and rough terrain with uncommon grace and use the land itself to lock down enemies."),
            source_page: Some("p.45"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ Wild Shadow],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerTrack,TYPE.RangerWildEmpathy,TYPE.RangerFavoredTerrain,TYPE.RangerHuntersBond,TYPE.RangerWoodlandStride,TYPE.RangerQuarry,TYPE.RangerCamouflage,TYPE.RangerImprovedQuarry]", "PREFACT:1,TEMPLATES,IsHalfElf=true"]),
            replaces: Some(&["RangerTrack", "RangerWildEmpathy", "RangerFavoredTerrain", "RangerHuntersBond", "RangerWoodlandStride", "RangerQuarry", "RangerCamouflage", "RangerImprovedQuarry"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Wild Shadow ~ Wild at Heart", at_level: 1, description: Some("A wild shadow adds only 1/2 his class level when making wild empathy checks while in urban areas, and adds only 1/4 his class level to follow or identify tracks in such areas. In non-urban settings, he is considered two levels higher when determining the bonuses for such checks."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wild Shadow ~ Favored Terrain", at_level: 3, description: Some("When a wild shadow chooses a favored terrain, he cannot choose urban as the terrain type. Furthermore, at 8th level and every five levels thereafter, when he chooses a new favorite terrain type, he cannot choose the urban terrain type."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wild Shadow ~ Woodland Stride", at_level: 4, description: Some("This ability functions as the 7th level ranger class feature of the same name, but the wild shadow gains it at 4th level instead."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wild Shadow ~ Unfettered Step", at_level: 7, description: Some("A wild shadow's woodland stride class feature functions in any difficult terrain within any of his favored terrains, even in areas that are enchanted or magically manipulated to impede motion."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wild Shadow ~ Harrying Attack", at_level: 11, description: Some("A wild shadow can use his knowledge of terrain to make shrewd attacks in combat. Such attacks can make foes fumble or cause them to become entangled within areas of the wild shadow's favored terrain. As a standard action, the wild shadow denotes one target within line of sight and within one of his favored terrains as his harried prey. Once the foe is so designated, every time the wild shadow hits this harried prey with a melee or ranged weapon attack (either manufactured or natural), that creature is entangled for 1 round. A wild shadow can have no more than one harried prey at a time and that creature must correspond to one of his favored enemy types. He can dismiss this effect at any time as a free action, be he cannot select a new harried prey for 24 hours. If the wild shadow sees proof that his harried prey is dead, he can select a new harried prey after waiting 1 hour."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wild Shadow ~ Wild Stalker", at_level: 14, description: Some("A wild shadow learns to better use natural surroundings to obscure his position in combat. Whenever a wild shadow is within one of his favored terrains and a feature of that terrain grants him cover, the bonuses to AC and Reflex saves for that cover improve by %1. Additionally, while he is within one of his favored terrains, if he gains concealment or total concealment, the miss chance of either type of concealment improves by %10%%.|1+(RangerLVL-10)/3"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wild Shadow ~ Master of Terrain", at_level: 19, description: Some("A wild shadow can use his harrying attack against creatures other than his favored enemy or he can spend a standard action to designate up to two of his favored enemies as his harried prey instead."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Cat Burglar -- arg_abilities_class.lst:441
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Cat Burglar",
            subject: "Rogue",
            archetype_name: "Cat Burglar",
            description: Some("Gifted with finesse and stealth, catfolk make excellent burglars. Cat burglars are masters of breaking and entering, using their feline grace to make it seem as though no crime was ever committed in the first place. Few locks can withstand skilled cat burglars, and such nimble rogues are capable of bypassing traps without activating them and enabling associates to do the same."),
            source_page: Some("p.92"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Cat Burglar],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RogueUncannyDodge]", "PREFACT:1,TEMPLATES,IsCatfolk=true"]),
            replaces: Some(&["RogueUncannyDodge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Cat Burglar ~ Phantom Presence", at_level: 4, description: Some("The cat burglar masters stealthy movement and leaves no trace of her passing in dungeons and cities. While in dungeon and urban environments, she leaves no trail and cannot be tracked, though she can choose to leave behind a trail if she so desires. Furthermore, she can always choose to take 10 when making a Stealth check."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cat Burglar ~ Trap Saboteur", at_level: 8, description: Some("The cat burglar becomes a master of avoiding and manipulating traps and locks. She can attempt to open a lock as a standard action and takes 1/2 the normal amount of time to disable traps (minimum 1 round) . When she has bypassed a trap without disarming it, she can also choose to suppress its trigger for up to 1 minute. If she does, she can also choose to end this suppression prematurely as a free action. This ability replaces improved uncanny dodge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cat Burglar ~ Rogue Talents", at_level: 1, description: Some("The following rogue talents complement the cat burglar archetype: fast stealth, quick disable, convincing fakes, dodge trap, fast picks, terrain mastery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cat Burglar ~ Advanced Talents", at_level: 1, description: Some("The following advanced rogue talents complement the cat burglar archetype: another day, fast tumble; hide in plain sight."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Deadly Courtesan -- arg_abilities_class.lst:989
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Deadly Courtesan",
            subject: "Rogue",
            archetype_name: "Deadly Courtesan",
            description: Some("Skilled at manipulation and diversion, the deadly courtesan builds up those around her and periodically takes them down. She can be a spy, entertainer, assassin, bodyguard, or just an intimate to someone who needs it most."),
            source_page: Some("p.208"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Deadly Courtesan],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RogueTrick2,TYPE.RogueTrapSense1,TYPE.RogueTrapSense2,TYPE.RogueTrapSense3,TYPE.RogueTrapSense4,TYPE.RogueTrapSense5,TYPE.RogueTrapSense6,TYPE.RogueUncannyDodge]", "PREFACT:1,TEMPLATES,IsVishkanya=true"]),
            replaces: Some(&["RogueTrick2", "RogueTrapSense1", "RogueTrapSense2", "RogueTrapSense3", "RogueTrapSense4", "RogueTrapSense5", "RogueTrapSense6", "RogueUncannyDodge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Deadly Courtesan ~ Class Skills", at_level: 1, description: Some("A deadly courtesan adds Knowledge (history) and Knowledge (nobility) to her list of class skills and removes Knowledge (dungeoneering) from her list of class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deadly Courtesan ~ Bardic Performance", at_level: 2, description: Some("A deadly courtesan gains the bardic performance ability and the fascinate bardic performance. Her fascinate DC is %1. She can use this bardic performance for %2 rounds per day. If the courtesan also has bard levels, she may use these rounds for either class's fascinate bardic performance, and her bard and rogue levels stack for determining her fascinate DC.|10+RogueLVL/2+CHA|1+CHA+RogueLVL-2"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deadly Courtesan ~ Inspire Competence", at_level: 3, description: Some("A deadly courtesan can use her bardic performance to inspire competence with a +%1 bonus.|2+(RogueLVL-3)/6"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deadly Courtesan ~ Performance Strike", at_level: 8, description: Some("As a swift action, a deadly courtesan may expend rounds of bardic performance to gain a morale bonus on one attack roll. The amount of the bonus is equal to the number of bardic performance rounds expended (maximum bonus equal to %1).|RogueLVL/2"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deadly Courtesan ~ Rogue Talents", at_level: 1, description: Some("The following rogue talents complement the deadly courtesan archetype: finesse rogue, stand up (Core Rulebook); charmer, coax information, fast fingers, honeyed words (Advanced Player's Guide); convincing lie, deft palm (Ultimate Combat)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deadly Courtesan ~ Advanced Talents", at_level: 1, description: Some("The following advanced rogue talents complement the deadly courtesan archetype: slippery mind (Core Rulebook); master of disguise (Advanced Player's Guide); rumormonger, unwitting ally (Ultimate Combat)."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Eldritch Raider -- arg_abilities_class.lst:842
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Eldritch Raider",
            subject: "Rogue",
            archetype_name: "Eldritch Raider",
            description: Some("An eldritch raider is a rogue who seeks to unravel the mysteries of the destruction of the Gillman's homeland. They explore old ruins that date back to the days of the old human empire and track down relics and lore from its glory days."),
            source_page: Some("p.189"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Eldritch Raider],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RogueTalent2,TYPE.RogueTrapSense]", "PREFACT:1,TEMPLATES,IsGillman=true"]),
            replaces: Some(&["RogueTalent2", "RogueTrapSense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Eldritch Raider ~ Class Skills", at_level: 1, description: Some("An eldritch raider adds Knowledge (arcana), Knowledge (history), and Spellcraft to her list of class skills and removes Disguise, Perform, and Sleight of Hand from her list of class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Eldritch Raider ~ Skill Ranks per Level", at_level: 2, description: Some("6 + Int modifier."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Eldritch Raider ~ Detect Magic", at_level: 3, description: Some("An eldritch raider gains the ability to use detect magic at will at caster level %1. This ability counts as the minor magic rogue talent for purposes of qualifying for other rogue talents.|RogueLVL"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Eldritch Raider ~ Eldritch Intuition", at_level: 1, description: Some("An eldritch raider gains an intuitive sense that allows her to more easily activate sorcerer and wizard spell completion and spell trigger items. She gains a +%1 bonus on Use Magic Device checks for this purpose.|RogueLVL/3"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Eldritch Raider ~ New Talents", at_level: 1, description: Some("An eldritch raider has access to the following new advanced talents when selecting rogue advanced talents."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Eldritch Raider ~ Rogue Talents", at_level: 1, description: Some("The following rogue talents complement the eldritch raider archetype: major magic, minor magic, quick disable, trap spotter (Core Rulebook); fast picks (Advanced Player's Guide); black market connections, esoteric scholar, ninja trick (slow metabolism, wall climber) (Ultimate Combat)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Eldritch Raider ~ Advanced Talents", at_level: 1, description: Some("The following advanced rogue talents complement the eldritch raider archetype: dispelling attack, improved evasion, slippery mind (Core Rulebook); major eldritch magic, minor eldritch magic (Advanced Race Guide); thoughtful reexamination (Advanced Player's Guide); familiar, hard to fool (Ultimate Combat)."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Filcher -- arg_abilities_class.lst:295
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Filcher",
            subject: "Rogue",
            archetype_name: "Filcher",
            description: Some("A filcher steals valuables without their owners even realizing it. Whether cutting purses in the midst of combat or replacing prized items with fakes under the noses of their owners, the filcher is the master of the quick and quiet steal. A filcher has the following class features."),
            source_page: Some("p.64"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Filcher],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RogueEvasion.RogueTrapSense.RogueUncannyDodge.RogueImprovedUncannyDodge]", "PREFACT:1,TEMPLATES,IsHalfling=true"]),
            replaces: Some(&["RogueEvasion", "RogueTrapSense", "RogueUncannyDodge", "RogueImprovedUncannyDodge"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Filcher ~ Quicker than the Eye", at_level: 2, description: Some("A filcher develops an amazingly swift and delicate touch. When she uses Sleight of Hand, creatures take a penalty on their Perception checks to notice the attempt equal to half the filcher's class level. The filcher also subtracts her class level from the normal -20 penalty when attempting to make a Sleight of Hand check as a move action instead of as a standard action. Lastly, the filcher can withdraw an object hidden on her person, including a weapon, as a move action instead of the usual standard action."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Filcher ~ Rummage", at_level: 3, description: Some("A filcher learns how to assess the value of items at the quickest glance. She can even make startlingly accurate guesses about particular items merely by observing the bulges they make in pouches, backpacks, or similar containers. She gains a +%1 bonus on Appraise checks. As a swift action, a filcher can make an Appraise check in order to determine the relative value of each object carried by her target (DC = 10 + 1 for every object the filcher is trying to ascertain the relative value of). Though she never learns the actual prices of items when using rummage, she does gain enough information to list these items in order, from the most valuable to the least valuable. She can, by taking a -20 penalty on the check, add to this assessment any items carried by her target that she cannot see.|RogueLVL/3"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Filcher ~ Filch", at_level: 4, description: Some("A filcher learns how pluck items off her opponents even in combat. She gains Improved Steal (Advanced Player's Guide 163) as a bonus feat and can use her Sleight of Hand bonus instead of her CMB when performing a steal combat maneuver (Advanced Player's Guide 322). If the filcher gains bonuses on combat maneuver checks from any feats, spells, magic items, or similar effects, they are added to the Sleight of Hand bonus when using the steal maneuver."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Filcher ~ Superior Filching", at_level: 8, description: Some("A filcher becomes a master at separating owners from their property. She gains Greater Steal as a bonus feat, and opponents do not gain a +5 bonus to their CMD when she tries to remove items fastened to them."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Filcher ~ Rogue Talents", at_level: 1, description: Some("The following rogue talents complement the filcher archetype: fast stealth, slow reactions (Core Rulebook); fast fingers, fast getaway (Advanced Player's Guide); black market connections, deft palm (Ultimate Combat)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Filcher ~ Advanced Talents", at_level: 1, description: Some("The following advanced rogue talents complement the filcher archetype: skill mastery (Core Rulebook); fast tumble (Advanced Player's Guide); weapon snatcher (Ultimate Combat)."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Kitsune Trickster -- arg_abilities_class.lst:873
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Kitsune Trickster",
            subject: "Rogue",
            archetype_name: "Kitsune Trickster",
            description: Some("The kitsune trickster combines her sharpened wit with minor arcane powers of charm and persuasion. She uses her talents to spin convincing lies, riddles, and stories."),
            source_page: Some("p.193"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Kitsune Trickster],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RogueTrapfinding,TYPE.RogueTrapSense]", "PREFACT:1,TEMPLATES,IsKitsune=true"]),
            replaces: Some(&["RogueTrapfinding", "RogueTrapSense"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Kitsune Trickster ~ Kitsune's Guile", at_level: 1, description: Some("A trickster relies on her intellect as much as her personality. She adds her Intelligence modifier on Bluff, Diplomacy, Disguise, and Sense Motive checks."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Kitsune Trickster ~ Kitsune's Charm", at_level: 3, description: Some("The kitsune trickster can use charm person %1 times per day as a spell-like ability (caster level %2). This ability replaces trap sense.|RogueLVL/3|KitsuneCharmCasterLVL"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Kitsune Trickster ~ Rogue Talents", at_level: 1, description: Some("The following rogue talents complement the kitsune trickster archetype: major magic, minor magic (Core Rulebook); false friend, obfuscate story, steal the story (see below); charmer, coax information, honeyed words (Advanced Player's Guide); convincing lie (Ultimate Combat). Advanced Talents: The following advanced rogue talents complement the kitsune trickster archetype: skill mastery, slippery mind (Core Rulebook); master of disguise (Advanced Player's Guide); rumormonger (Ultimate Combat)."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Skulking Slayer -- arg_abilities_class.lst:248
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Skulking Slayer",
            subject: "Rogue",
            archetype_name: "Skulking Slayer",
            description: Some("Pushed into a life of crime by the society around them, half orcs gravitate toward criminal activities that suit them best. Half-orc rogues leave subtle tactics and finesse to halflings and elves, and rely on brute strength and thuggery when they go about making mischief. Skulking slayers have turned the use of raw strength and surprise into an art form. A skulking slayer has the following class features."),
            source_page: Some("p.55"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Skulking Slayer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RogueTrapfinding.RogueTrapSense1.RogueTrapSense2.RogueTrapSense3.RogueTrapSense4.RogueTrapSense]", "PREFACT:1,TEMPLATES,IsHalfOrc=true"]),
            replaces: Some(&["RogueTrapfinding", "RogueTrapSense1", "RogueTrapSense2", "RogueTrapSense3", "RogueTrapSense4"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Skulking Slayer ~ Weapon and Armor Proficiency", at_level: 1, description: Some("The skulking slayer gains proficiency with greatclubs and whips, but loses proficiency with rapiers and hand crossbows."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Skulking Slayer ~ Class Skills", at_level: 1, description: Some("The skulking slayer does not gain Disable Device, Linguistics, and Sleight of Hand as class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Skulking Slayer ~ Skill Ranks per Level", at_level: 1, description: Some("6 + Int modifier."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Skulking Slayer ~ Pass for Human", at_level: 1, description: Some("When trying to conceal her half-orc heritage, a skulking slayer gains a bonus on Disguise checks equal to half her level. When using disguise to appear as a specific individual, skulking stalkers ignore the normal -2 penalty to appear as another race."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Skulking Slayer ~ Underhanded Maneuvers ", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Skulking Slayer ~ Bonus Feats", at_level: 2, description: Some("A skulking slayer can select the Surprise Follow-Through feat in place of a rogue talent. At 10th level, she can select the Improved Surprise Follow-Through feat in place of an advanced rogue talent (if she already has Surprise Follow-Through)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Skulking Slayer ~ Bold Strike", at_level: 3, description: Some("When a skulking slayer charges and makes a sneak attack with a two-handed weapon, she rolls d8s instead of d6s for her sneak attack damage."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Skulking Slayer ~ Shifty", at_level: 6, description: Some("A skulking slayer gains a bonus on Bluff checks to feint equal to half her level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Skulking Slayer ~ Unexpected Charge", at_level: 9, description: Some("A skulking slayer can make a Bluff check to feint as a swift action before a charge."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Skulking Slayer ~ Rogue Talents", at_level: 1, description: Some("The following rogue talents complement the skulking slayer archetype: combat trick, surprise attack (Core Rulebook); combat swipe, powerful sneak (Advanced Player's Guide); terrain mastery (Ultimate Combat)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Skulking Slayer ~ Advanced Talents", at_level: 1, description: Some("The following advanced rogue talents complement the skulking slayer archetype: crippling strike (Core Rulebook); deadly sneak (Advanced Player's Guide); unwitting ally (Ultimate Combat)."), benefit: None },
            ],
        },
        // Rogue Archetype ~ Swordmaster -- arg_abilities_class.lst:761
        ArchetypeSwapEntry {
            key: "Rogue Archetype ~ Swordmaster",
            subject: "Rogue",
            archetype_name: "Swordmaster",
            description: Some("A swordmaster meditates to strengthen her spiritual connection to her blade. She strives to perfect her skills by mastering six deadly trances."),
            source_page: Some("p.164"),
            prerequisites: Some(&["PRECLASS:1,Rogue=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Rogue Archetype ~ Swordmaster],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RogueTrapSense1,TYPE.RogueTrapSense2,TYPE.RogueTrapSense3,TYPE.RogueTrapSense4,TYPE.RogueTrapSense5,TYPE.RogueTrapSense6]", "PREFACT:1,TEMPLATES,IsTengu=true"]),
            replaces: Some(&["RogueTrapSense1", "RogueTrapSense2", "RogueTrapSense3", "RogueTrapSense4", "RogueTrapSense5", "RogueTrapSense6"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Swordmaster ~ Class Skills", at_level: 1, description: Some("The swordmaster adds Knowledge (nature) and Survival to her list of class skills and removes Disguise and Knowledge (dungeoneering) from her list of class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Swordmaster ~ Trance", at_level: 3, description: Some("A swordmaster learns to focus her martial prowess using an intense meditative trance. Under the influence of a trance, the swordmaster can perform fantastic martial feats. Entering a trance is a full-round action that provokes attacks of opportunity. The swordmaster can maintain the trance for %1 rounds per day. She can end her trance as a free action. Following a trance, the swordmaster is fatigued for a number of rounds equal to 2 x the number of rounds she spent in the trance. A swordmaster cannot enter a new trance while fatigued but can otherwise enter a trance multiple times during a single encounter or combat. If a swordmaster falls unconscious, her trance immediately ends. She can only use one type of trance at a time.|4+WIS+RogueLVL-3"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Swordmaster ~ Rogue Talents", at_level: 1, description: Some("The following rogue talents complement the swordmaster archetype: combat trick, stand up, surprise attack, weapon training (Core Rulebook); befuddling strike, positioning strike (Advanced Player's Guide)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Swordmaster ~ Advanced Talents", at_level: 1, description: Some("The following advanced rogue talents complement the swordmaster archetype: crippling strike, defensive roll (Core Rulebook); hunter's surprise, redirect attack (Advanced Player's Guide); confounding blade (Ultimate Combat)."), benefit: None },
            ],
        },
        // Summoner Archetype ~ Blood God Disciple -- arg_abilities_class.lst:245
        ArchetypeSwapEntry {
            key: "Summoner Archetype ~ Blood God Disciple",
            subject: "Summoner",
            archetype_name: "Blood God Disciple",
            description: Some("A half-orc summoner who devotes himself to one of the bloody orc gods may believe his eidolon is an avatar of that god rather than a mere supernatural creature. A blood god disciple generally fights by the avatar's side and offers it blood sacrifices in exchange for martial prowess. A blood god disciple has the following class features."),
            source_page: Some("p.53"),
            prerequisites: Some(&["PRECLASS:1,Summoner=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Summoner Archetype ~ Blood God Disciple],[!PREABILITY:1,CATEGORY=Archetype,TYPE.SummonerSummonMonster]", "PREFACT:1,TEMPLATES,IsHalfOrc=true"]),
            replaces: Some(&["SummonerSummonMonster"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Blood God Disciple ~ Blood Feast", at_level: 1, description: Some("A blood god disciple can feed a recently fallen foe to his eidolon, allowing the outsider to channel some of its power into the summoner. The eidolon must spend a standard action to eat some of the opponent, which must be a living, corporeal creature killed or knocked unconscious by the eidolon or summoner in the past minute. This eating deals damage to the target as if the eidolon had attacked it with one natural attack (typically a bite). The fallen creature must have at least half as many Hit Dice as the summoner. Once the feeding is complete, the summoner may manifest %1 evolution points. This lasts for 1 minute. The evolution's effects use the summoner's Hit Dice and ability scores rather than the eidolon's. The blood god disciple can use this ability %2 times per day. He may only apply one use of this ability at a time (using it a second time replaces any evolution manifested with this ability), and can only manifest evolutions his eidolon has.|1+(SummonerLVL-1)/4|3+CHA"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Blood God Disciple ~ Bloody Gift", at_level: 3, description: Some("When a blood god disciple uses blood feast to manifest an evolution, he may touch %1 allies and grant that evolution as well. Each affected ally counts as one use per day of the blood feast ability. If the blood god disciple can manifest multiple evolutions per use of blood feast, his selected allies manifest these multiple evolutions as well.|1+(SummonerLVL-3)/4"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Blood God Disciple ~ Avatar Gambit", at_level: 7, description: Some("When a blood god disciple dismisses his eidolon, he rages like a barbarian for a number of rounds equal to half his summoner level (he may end this rage early just like a barbarian, but if he does so, any remaining rounds of rage from this ability are lost)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Blood God Disciple ~ Rage Power", at_level: 11, description: Some("A blood god disciple selects a barbarian rage power, which he may use when raging (whether from the avatar gambit ability or actual barbarian rage)."), benefit: None },
            ],
        },
        // Summoner Archetype ~ Shaitan Binder -- arg_abilities_class.lst:668
        ArchetypeSwapEntry {
            key: "Summoner Archetype ~ Shaitan Binder",
            subject: "Summoner",
            archetype_name: "Shaitan Binder",
            description: Some("Shaitan binders call upon a reflection of their genie ancestors to serve as their eidolons. A shaitan binder has the following class features."),
            source_page: Some("p.146"),
            prerequisites: Some(&["PRECLASS:1,Summoner=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Summoner Archetype ~ Shaitan Binder],[!PREABILITY:1,CATEGORY=Archetype,TYPE.SummonerShareSpells,TYPE.SummonerEidolon,TYPE.SummonerShieldAlly,TYPE.SummonerGreaterShieldAlly,TYPE.SummonerAspect,TYPE.SummonerGreaterAspect,TYPE.SummonerTwinEidolon]", "PREFACT:1,TEMPLATES,IsOread=true"]),
            replaces: Some(&["SummonerShareSpells", "SummonerShieldAlly", "SummonerGreaterShieldAlly", "SummonerAspect", "SummonerGreaterAspect", "SummonerTwinEidolon"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Shaitan Binder ~ Base Form", at_level: 1, description: Some("At 1st level, if a shaitan binder's eidolon has the biped base form, it gains a +2 bonus to one ability score. The shaitan binder must make this choice at 1st level. If at any time the shaitan binder's eidolon has another base form, it loses this bonus until it returns to biped form. A shaitan binder's eidolon does not gain the share spells ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaitan Binder ~ Shaitan Magic", at_level: 4, description: Some("The shaitan binder's eidolon gains the basic magic evolution (Ultimate Magic) as a free evolution. At 6th level, it gains the minor magic evolution (Ultimate Magic). At 8th level, it gains the major magic evolution (Ultimate Magic) as a free evolution, and adds the following to the list of available spells for that evolution: glitterdust and soften earth and stone. At 12th level, it gains the ultimate magic evolution (Ultimate Magic) as a free evolution, and adds the following to the list of available spells for that evolution: meld into stone and stone shape. Although the shaitan binder gains the standard versions of these evolutions for free, he must pay the normal cost to upgrade them to the improved versions. This ability replaces shield ally and greater shield ally."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaitan Binder ~ Earth Glide", at_level: 10, description: Some("If a shaitan binder's eidolon has the burrow evolution, it gains the earth glide universal monster ability and can use this ability to travel at its full base speed. This ability replaces aspect."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaitan Binder ~ Stone Curse", at_level: 18, description: Some("The shaitan binder may select stone curse as a 4-point evolution. This allows the eidolon to trap creatures in stone like the shaitan stone curse ability (Bestiary 143). The DC to resist or break free of the stone curse is 10 + 1/2 the eidolon's Hit Dice + the eidolon's Strength score."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaitan Binder ~ Noble Eidolon", at_level: 20, description: Some("The shaitan binder's eidolon gains the ability to grant its summoner's wishes. Once per day, the eidolon can cast limited wish as a spelllike ability. The eidolon's caster level is equal to its level. The wish must be spoken aloud by the shaitan binder, beginning with the words \"I wish,\" and cannot duplicate a wish the eidolon has granted within the past 24 hours. If the eidolon uses this ability to duplicate a spell with a costly material component, the shaitan binder must provide that component."), benefit: None },
            ],
        },
        // Witch Archetype ~ Bonded Witch -- arg_abilities_class.lst:194
        ArchetypeSwapEntry {
            key: "Witch Archetype ~ Bonded Witch",
            subject: "Witch",
            archetype_name: "Bonded Witch",
            description: Some("While all witches commune with the unknown, the blend of human ingenuity and adept learning mixed with elven blood gives some half-elves a unique conduit to channel the powers of the arcane. Bonded witches forsake familiars as vessels of power in favor of a specific object that grants them powers above and beyond those of their patron alone, as they tap into the powerful magic of the item itself."),
            source_page: Some("p.43"),
            prerequisites: Some(&["PRECLASS:1,Witch=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Witch Archetype ~ Bonded Witch],[!PREABILITY:1,CATEGORY=Archetype,TYPE.WitchFamiliar]", "PREFACT:1,TEMPLATES,IsHalfElf=true"]),
            replaces: Some(&["WitchFamiliar"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Bonded Witch ~ Bonded Item", at_level: 1, description: Some("At 1st level, a bonded witch gains a bonded item instead of a familiar. This bonded item is similar to a wizard's arcane bond bonded item (Core Rulebook 78), and follows all the rules of such an item with the following exceptions. A bonded witch's bonded item serves as a vessel for her spells and a conduit for communication with her patron. A bonded witch must commune with her bonded item each day to prepare her spells. The bonded item stores all of the spells that the bonded witch knows, and the bonded witch cannot prepare spells that are not stored within it. A bonded witch starts with the same number of spells and gains new spells the same way as a witch, and can even add spells by learning them from scrolls in the same way (Advanced Player's Guide 68), but a bonded witch cannot learn spells from another bonded item. Since a bonded witch does not have a spellbook, starting at 2nd level, a bonded witch's bonded item can be used once per day to cast a spell dependent on the type of bonded object chosen by the bonded witch. The spell is treated like any other spell cast by the bonded witch, including its casting time, duration, and other effects dependent of the bonded witch's level. This spell cannot be further modified by metamagic feats or any other ability. As the bonded witch gains levels, the bonded item gains new spells that the bonded witch can cast in this way. She can cast any one of these spells once per day using her bonded object, but gains greater flexibility in what spells she can cast, and gains more powerful spells as she gains new levels. The bonded item spells associated with each item type are as follows."), benefit: None },
            ],
        },
        // Witch Archetype ~ Dreamweaver -- arg_abilities_class.lst:819
        ArchetypeSwapEntry {
            key: "Witch Archetype ~ Dreamweaver",
            subject: "Witch",
            archetype_name: "Dreamweaver",
            description: Some("A changeling dreamweaver draws upon her hag heritage to ply the dream realms in order to touch mortal minds and souls, for good or ill."),
            source_page: Some("p.185"),
            prerequisites: Some(&["PRECLASS:1,Witch=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Witch Archetype ~ Dreamweaver],[!PREABILITY:1,CATEGORY=Archetype,TYPE.WitchHex2,TYPE.WitchHex6,TYPE.WitchHex10]", "PREFACT:1,TEMPLATES,IsChangeling=true"]),
            replaces: Some(&["WitchHex2", "WitchHex6", "WitchHex10"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Dreamweaver ~ Class Skills", at_level: 1, description: Some("The dreamweaver adds Sense Motive to her list of class skills and removes Healing from her list of class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dreamweaver ~ Patron", at_level: 1, description: Some("A dreamweaver's patron is normally portents or stars (Pathfinder RPG Ultimate Magic 83)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dreamweaver ~ Spells", at_level: 1, description: Some("A dreamweaver replaces some of her patron's spells with the following: 2nd-sow thoughts (see below), 4th-dust of twilight (Pathfinder RPG Advanced Player's Guide), 6th-deep slumber, 8th-modify memory, 10th-dream, 12th- cloak of dreams (Advanced Player's Guide), 14th-ethereal jaunt, 16th-moment of prescience, 18th-astral projection."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dreamweaver ~ Dream Spinner", at_level: 2, description: Some("When a dreamweaver casts a mind-affecting spell on a target that is sleeping because of her slumber hex or a spell she cast, she adds +1 to the mind-affecting spell's DC. If the target succeeds at the saving throw against the spell, it does not wake up, nor does it have any recollection of having resisted a spell. If appropriate, the dreamweaver may incorporate elements of a mind-affecting spell (i.e., sow thought, suggestion, and so on) into the target's subconscious so it believes the spell's effects originated in its dreams (the details of how these elements fit into the dream is up to the GM)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dreamweaver ~ Dream Thief", at_level: 6, description: Some("A dreamweaver can alter the sleeping mind of any creature that is sleeping because of her slumber hex or a spell she cast. She can reshape one of the target's memories as if using modify memory. Alternatively, she may insert herself into the dreaming memories of the target, prompting the target's mind to show her some specific information; the dreamer's subconscious may resist, or try to deceive her with out-ofcontext memories, similar to the way a corpse can resist when questioned with speak with dead. A Will save negates either effect (DC equal to that of the witch's hex). Whether or not the save is successful, a creature cannot be the target of this hex again for 1 day."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dreamweaver ~ Dream Possession", at_level: 10, description: Some("A dreamweaver can take control of any creature that is sleeping because of her slumber hex or a spell she cast. This effect functions as magic jar, using the witch's familiar acting as the soul receptacle. A Will save negates either effect (DC equal to that of the witch's hex). Whether or not the save is successful, a creature cannot be the target of this hex again for 1 day."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dreamweaver ~ Hexes", at_level: 1, description: Some("The following hexes complement the dreamweaver archetype: charm, slumber (Advanced Player's Guide); beast of ill-omen (Ultimate Magic)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dreamweaver ~ Major Hexes", at_level: 1, description: Some("The following major hexes complement the dreamweaver archetype: nightmare, vision (Advanced Player's Guide)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dreamweaver ~ Grand Hexes", at_level: 1, description: Some("The following major hexes complement the dreamweaver archetype: eternal slumber (Advanced Player's Guide); dire prophecy (Ultimate Magic)."), benefit: None },
            ],
        },
        // Witch Archetype ~ Scarred Witch Doctor -- arg_abilities_class.lst:654
        ArchetypeSwapEntry {
            key: "Witch Archetype ~ Scarred Witch Doctor",
            subject: "Witch",
            archetype_name: "Scarred Witch Doctor",
            description: Some("The scarred witch doctor draws power from her ability to endure pain and suffering. She mutilates her own flesh, inflicting horrific scars, in order to attract the attention of her patron. Rather than call forth a familiar, she creates a repulsive fetish mask that she uses as a repository for her power."),
            source_page: Some("p.140"),
            prerequisites: Some(&["PRECLASS:1,Witch=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Witch Archetype ~ Scarred Witch Doctor],[!PREABILITY:1,CATEGORY=Archetype,TYPE.WitchFamiliar,TYPE.WitchHex1]", "PREFACT:1,TEMPLATES,IsOrc=true"]),
            replaces: Some(&["WitchFamiliar", "WitchHex1"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Scarred Witch Doctor ~ Fierce Intelligence", at_level: 1, description: Some("A scarred witch doctor treats her Intelligence score as 2 points higher when determining the highest level of spells she can cast, the number of spells she can cast per day, her spell save DCs, her number of spells known at 1st level, and any effects of her hexes determined by her Intelligence."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Scarred Witch Doctor ~ Hex Scar", at_level: 1, description: Some("Whenever a scarred witch doctor learns a hex, she must carve or brand a symbol in her flesh to represent this hex. She can disguise these scars with mundane or magical means, but they cannot be permanently removed."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Scarred Witch Doctor ~ Fetish Mask 1", at_level: 1, description: Some("A scarred witch doctor forms a bond with a wooden mask. As she gains power, her connection to this mask causes it to grow ever more hideous and grotesque as it absorbs the weight of the self-induced pain that underlies her magic. Her spells derive from the insights her patron grants her while she's enduring the cuts, burns, and other sorts of mutilations she inflicts upon herself. Her fetish mask acts in all ways like a witch's familiar for the purpose of preparing and gaining spells. Rather than communing with a familiar to prepare spells each day, a scarred witch doctor hangs her mask on a wall, tree branch, or something similar and contemplates the agony it represents. When wearing her fetish mask, a scarred witch doctor gains a +2 circumstance bonus on Heal and Intimidate checks and gains a +2 bonus on saving throws against effects that specifically cause pain or have the pain descriptor. If the mask is destroyed, the witch doctor can create another fetish mask (which almost immediately adopts the shocking appearance of the original) for the same price and time it takes a witch to replace a dead familiar."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Scarred Witch Doctor ~ Fetish Mask 2", at_level: 5, description: Some("The scarred witch doctor gains the ability to add magical abilities to her mask as if she had the Craft Wondrous Item feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Scarred Witch Doctor ~ Scarshield", at_level: 1, description: Some("A scarred witch doctor learns how to harden her mutilated skin, gaining an enhancement bonus to her natural armor bonus equal to %1. She can use this ability for %2 minutes per day. These minutes do not need to be consecutive but she must spend them in 1-minute increments. This ability replaces the witch's 1st-level hex.|ScarredWitchNaturalArmorBonus|ScarredWithDoctorNaturalArmorDuration"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Scarred Witch Doctor ~ Hexes", at_level: 1, description: Some("The following hexes complement the scarred witch doctor archetype: evil eye, misfortune, scar, unnerve beasts."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Scarred Witch Doctor ~ Major Hexes", at_level: 1, description: Some("The following major hexes complement the scarred witch doctor archetype: agony, nightmare, cook people, infected wounds."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Scarred Witch Doctor ~ Grand Hexes", at_level: 1, description: Some("The following grand hexes complement the scarred witch doctor archetype: death curse, natural disaster, dire prophecy."), benefit: None },
            ],
        },
        // Wizard Archetype ~ Cruoromancer -- arg_abilities_class.lst:457
        ArchetypeSwapEntry {
            key: "Wizard Archetype ~ Cruoromancer",
            subject: "Wizard",
            archetype_name: "Cruoromancer",
            description: Some("To those who know how to manipulate it, the blood of a dhampir can be a powerful component to magic. A cruoromancer infuses his necromantic magic with the power of his unique mixture of living blood and undead ichor. As his power increases in this strange arcane art, a cruoromancer finds potent ways to infuse his unique blood with necromancy spells. A cruoromancer has the following class features."),
            source_page: Some("p.98"),
            prerequisites: Some(&["PRECLASS:1,Wizard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Wizard Archetype ~ Cruoromancer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.WizardArcaneBond,TYPE.WizardBonusFeat5,TYPE.WizardBonusFeat10,TYPE.WizardBonusFeat15,TYPE.WizardBonusFeat20]", "PREFACT:1,TEMPLATES,IsDhampir=true"]),
            replaces: Some(&["WizardArcaneBond", "WizardBonusFeat5", "WizardBonusFeat10", "WizardBonusFeat15", "WizardBonusFeat20"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Cruoromancer ~ Blood Infusion", at_level: 1, description: Some("When a cruoromancer casts a spell of the necromancy school, he can opt to infuse that spell with his undead-tainted blood as a swift action. As he increases in level, the power and effects of such infusions become more potent. Each time a cruoromancer uses blood infusion, he drains a portion of his own blood either by cutting himself with a blade or by opening a scab from a previous wound. When he does this, he takes an amount of damage equal to 1d4 + the level of the spell being infused. A cruoromancer can only affect a spell with a single type of blood infusion."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cruoromancer ~ Focused Infusion", at_level: 1, description: Some("When the cruoromancer uses this infusion, he adds +1 to the DC of the infused necromancy spell."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cruoromancer ~ Sickening Infusion", at_level: 1, description: Some("When the cruoromancer uses this infusion, any creature damaged by the infused necromancy spell becomes sickened for 1 round."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cruoromancer ~ Blood Command", at_level: 5, description: Some("The cruoromancer can control up to %1 Hit Dice worth of undead creatures when casting the animate dead spell.|WizardLVL*5"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cruoromancer ~ Commanding Infusion", at_level: 5, description: Some("When using this infusion with animate dead, the cruoromancer can create %1 Hit Dice of undead.|WizardLVL*3"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cruoromancer ~ Desecrating Infusion", at_level: 10, description: Some("When the cruoromancer uses this infusion, he can choose to center a desecrate effect on himself or a single target of the spell modified by this infusion (he chooses upon casting). This effect is like the desecrate spell, but lasts for %1 minutes and does not interact with altars, shrines, or permanent fixtures that boost the desecrate effect.|CruoromancerDesecrateInfusionTime"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cruoromancer ~ Blood Ability", at_level: 15, description: Some("The cruoromancer can choose to scry through a single undead creature he created with a spell modified by a commanding infusion. The undead creature is treated as if imbued with an arcane eye spell (caster level %1).|CruoromancerBloodAbilityCastLVL"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cruoromancer ~ Perfect Infusion", at_level: 20, description: Some("The cruoromancer can use his blood infusions without taking damage."), benefit: None },
            ],
        },
        // Wizard Archetype ~ Spellbinder -- arg_abilities_class.lst:107
        ArchetypeSwapEntry {
            key: "Wizard Archetype ~ Spellbinder",
            subject: "Wizard",
            archetype_name: "Spellbinder",
            description: Some("A spellbinder is an elven wizard who forges an arcane bond between himself and one or more wizard spells. These spells become so well understood by the spellbinder that he can prepare them in spell slots that already have other spells prepared in them."),
            source_page: Some("p.25"),
            prerequisites: Some(&["PRECLASS:1,Wizard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Wizard Archetype ~ Spellbinder],[!PREABILITY:1,CATEGORY=Archetype,TYPE.WizardArcaneBond]", "PREFACT:1,TEMPLATES,IsElf=true"]),
            replaces: Some(&["WizardArcaneBond"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Spellbinder ~ Spell Bond", at_level: 1, description: Some("The spellbinder selects any one spell that he knows as a bonded spell. As a full-round action, the spellbinder may replace a spell of the same or higher level as his bonded spell with his bonded spell."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Spellbinder ~ Discoveries", at_level: 1, description: Some("The following discoveries complement the spellbinder archetype: Fast Study, Split Slot (Ultimate Magic)."), benefit: None },
            ],
        },
        // Wizard Archetype ~ Wind Listener -- arg_abilities_class.lst:736
        ArchetypeSwapEntry {
            key: "Wizard Archetype ~ Wind Listener",
            subject: "Wizard",
            archetype_name: "Wind Listener",
            description: Some("The wind listener takes a sylph's natural curiosity to the extreme, enhancing his natural skill at subterfuge and eavesdropping with potent arcane magic."),
            source_page: Some("p.158"),
            prerequisites: Some(&["!PREABILITY:1,CATEGORY=Special Ability,Divination Opposition School,Illusion Opposition School", "PRECLASS:1,Wizard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Wizard Archetype ~ Wind Listener],[!PREABILITY:1,CATEGORY=Archetype,TYPE.WizardArcaneBond,TYPE.WizardBonusFeat5,TYPE.WizardBonusFeat10,TYPE.WizardBonusFeat15]", "PREFACT:1,TEMPLATES,IsSylph=true"]),
            replaces: Some(&["WizardArcaneBond", "WizardBonusFeat5", "WizardBonusFeat10", "WizardBonusFeat15"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Wind Listener ~ Class Skills", at_level: 1, description: Some("A wind listener adds Perception to his list of class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wind Listener ~ Arcane School", at_level: 1, description: Some("A wind listener cannot select divination or illusion as a prohibited school."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wind Listener ~ Spontaneous Divination", at_level: 1, description: Some("A wind listener can reshape stored spell energy into divination spells he did not prepare ahead of time. The wind listener can \"lose\" any prepared spell that is not a cantrip in order to cast a divination spell of the same spell level or lower. The new spell must be one the wind listener knows and is capable of casting. Spells cast with this ability increase their casting time to a full-round action (if the spell's normal casting time is longer than a full-round action, it remains unchanged)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wind Listener ~ Abjuration Sense", at_level: 5, description: Some("A wind listener develops a sixth sense for spotting spells designed to guard against his investigations. He gains a +%1 bonus on Perception checks to notice spells of the abjuration school and on Spellcraft checks to identify abjuration effects, spells, and magic items.|WizardLVL/2"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wind Listener ~ Wispy Form", at_level: 10, description: Some("The wind listener gains the ability to become airy and translucent as a standard action, gaining DR 10/magic and the effects of greater invisibility for %1 rounds per day. These rounds need not be consecutive. Like the natural invisibility universal monster ability (Bestiary 2 299), this ability is not subject to invisibility purge.|WizardLVL"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wind Listener ~ Listening to the Wind", at_level: 15, description: Some("The wind listener can call upon spirits of the air to uncover lost lore about a legendary person, place, or thing. Invoking the spirits takes 10 minutes, during which time the wind listener must be free of distractions and able to concentrate. Once called, the spirits seek out information on the subject of the wind listener's inquiries. This functions as the spell legend lore (caster level equal to the wind listener's level), except that the wind listener is free to engage in other activities while spirits investigate on his behalf. The time required for the air spirits to return with this information is equal to what the casting time of the spell legend lore would have been if the wind listener had cast it. The wind listener can use this ability once per week, and only if he does not currently have air spirits searching for information. If the air spirits are currently searching for information, the wind listener can end their task early as a standard action, dismissing the magical effect and not returning any information."), benefit: None },
            ],
        },        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_has_59_records() {
        assert_eq!(archetype_swap_tables().len(), 59);
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

    /// ARG's own rate: 14% (8/59) -- a sixth distinct value alongside
    /// UPsi 33%, ACG 33%, APG 52%, UM 27%, UC 22%. Still no convergence.
    #[test]
    fn the_type_and_ability_lists_genuinely_disagree() {
        let total_replaces: usize =
            archetype_swap_tables().iter().map(|e| e.replaces.map_or(0, |r| r.len())).sum();
        let total_grants: usize = archetype_swap_tables().iter().map(|e| e.grants.len()).sum();
        assert_eq!(total_replaces, 304, "total TYPE: replaced-slot count across all 59 records");
        assert_eq!(total_grants, 346, "total ABILITY: granted-feature count across all 59 records, after the category ruling");
        assert_ne!(total_replaces, total_grants);

        let equal_count_records = archetype_swap_tables()
            .iter()
            .filter(|e| e.replaces.map_or(0, |r| r.len()) == e.grants.len())
            .count();
        assert_eq!(equal_count_records, 8, "of 59 (14%) -- ARG's own rate, the lowest so far");
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
        assert_eq!(resolved, 343, "343 of 346 grants carry real DESC:/BENEFIT: text -- the cleanest resolution rate of any table so far");
    }
}
