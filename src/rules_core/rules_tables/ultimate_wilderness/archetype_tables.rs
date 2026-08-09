//! Ultimate Wilderness (UW) archetype-swap catalog. SD28-E30
//! (`epic-32-archetype-swap`) tier-1 table 7, the last in the tier-1
//! set. See `ultimate_psionics::archetype_tables`'s own module doc
//! comment for the full struct rationale, the exhaustively-enumerated
//! `ABILITY:` grant grammar and its per-family inclusion ruling, and
//! the `.MOD`-injected-grant floor every table in this program states.
//!
//! **The only table in this program whose subject is not a class.**
//! All 30 records are `Companion`- or `Familiar`-subject archetypes
//! (`Companion Archetype ~ <Name>` ×16, `Familiar Archetype ~ <Name>`
//! ×14) -- confirmed structurally in `uw_abilities_companion.lst`, a
//! separate file from this book's own `uw_abilities_class.lst` (which
//! carries this book's class-feature content, none of it
//! archetype-shaped). This is the live proof that this mechanism's
//! subject-generic design (`subject: &'static str`, not a
//! class-specific enum) was the right call from the first table: a
//! seventh of the way through the tier-1 set, an entire book turns out
//! to need the non-class subject the struct was built to support.
//!
//! **Agreement rate, seventh and last tier-1 book: 30% (9/30)** --
//! 120 total `TYPE:`-replaced slots vs 121 total `ABILITY:`-granted
//! features, the closest any book has come to equal totals (a 1-record
//! difference program-wide, though still 21 of 30 records individually
//! disagree). Final tier-1 spread: UPsi 33%, ACG 33%, APG 52%, UM 27%,
//! UC 22%, ARG 14%, UW 30% -- seven books, seven distinct values,
//! confirming to closure the finding `decisions.md §51` states as
//! durable: `TYPE:`/`ABILITY:` are two different lists in every book
//! measured, disagreeing in the majority of records, at a rate with no
//! single number to converge on.
//!
//! **104 of 121 sub-feature grants (86%) resolved to real `DESC:`/
//! `BENEFIT:` text.** The 17 shortfalls split into two real causes: 10
//! found-but-textless bare-marker rows (`<Archetype> ~ <Trait> Skills`/
//! `Sight` pairs, e.g. `Aberrant Companion ~ Aberrant Skills`); 7 failed
//! `KEY:` lookups, all shaped `Familiar ~ <BaseAbility>` (`Speak with
//! Animals of Its Kind`, `Empathic Link`, `Share Spells`, etc.) --
//! plausibly real base-familiar-ability cross-references owned by
//! another book (CRB) rather than declared in this file, not confirmed.
//! No new grant-taxonomy shape found -- the taxonomy is at 6 recurring
//! shapes plus the one UC found, still open per `decisions.md §51`'s
//! own correction, not closed by this book either.
//!
//! **This book's own share of the 1,282-row corpus-wide `.MOD`-
//! injection population (`decisions.md §51`'s own addendum) is
//! exactly 1 row -- the smallest of any book, effectively nil.** Stated
//! plainly rather than reusing the standard floor-caveat wording every
//! other table in this program carries: for this table specifically,
//! the `.MOD`-injection hazard is not a meaningful bound on `grants`'
//! completeness, unlike ACG (251) or UC (147).
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
//! ultimate_wilderness/uw_abilities_companion.lst`), generated
//! programmatically by a one-off extraction script, not hand-transcribed.

use super::super::archetype_swap::{ArchetypeGrant, ArchetypeSwapEntry};

/// Full UW archetype-swap catalog: 30 real, distinct master records
/// (Companion/Familiar subject), in source order. Built once and cached
/// for the process lifetime.
pub fn archetype_swap_tables() -> &'static [ArchetypeSwapEntry] {
    static TABLE: std::sync::OnceLock<Vec<ArchetypeSwapEntry>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        vec![
        // Companion Archetype ~ Aberrant Companion -- uw_abilities_companion.lst:415
        ArchetypeSwapEntry {
            key: "Companion Archetype ~ Aberrant Companion",
            subject: "Companion",
            archetype_name: "Aberrant Companion",
            description: Some("There's something oddly wrong about aberrant companions. Yet though they're touched by eldritch magic or mutated by strange influences, they are still loyal, if unnatural, allies."),
            source_page: Some("p.186"),
            prerequisites: Some(&["PRECLASS:1,Companion=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Companion Archetype ~ Aberrant Companion],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_CompanionShareSpells,TYPE.CF_CompanionDevotion,TYPE.CF_CompanionMultiattack]"]),
            replaces: Some(&["CF_CompanionShareSpells", "CF_CompanionDevotion", "CF_CompanionMultiattack"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Aberrant Companion ~ Aberrant Skills", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Aberrant Companion ~ Not Quite Animal", at_level: 1, description: Some("The DC to use Handle Animal on an aberrant companion is 5 higher, as if it were a nonanimal with an Intelligence score of 1 or 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Aberrant Companion ~ Aberrant Sight", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Aberrant Companion ~ Alien Mind", at_level: 1, description: Some("An aberrant companion is immune to mind-affecting effects that specifically target animals, such as charm animal. Anyone who attempts to use such an effect against it takes 1d4 points of Wisdom damage (Will DC 20 half)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Aberrant Companion ~ Fluid Bones", at_level: 1, description: Some("An aberrant companion gains compression as per the universal monster rule, though it can use the ability while carrying a rider only if the rider has compression."), benefit: None },
            ],
        },
        // Companion Archetype ~ Ambusher -- uw_abilities_companion.lst:416
        ArchetypeSwapEntry {
            key: "Companion Archetype ~ Ambusher",
            subject: "Companion",
            archetype_name: "Ambusher",
            description: Some("Ambushers sneak up on unsuspecting prey, pouncing on targets when they least expect it."),
            source_page: Some("p.186"),
            prerequisites: Some(&["PRECLASS:1,Companion=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Companion Archetype ~ Ambusher],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_CompanionShareSpells,TYPE.CF_CompanionEvasion,TYPE.CF_CompanionImprovedEvasion,TYPE.CF_CompanionDevotion,TYPE.CF_CompanionMultiattack]"]),
            replaces: Some(&["CF_CompanionShareSpells", "CF_CompanionDevotion", "CF_CompanionMultiattack", "CF_CompanionEvasion", "CF_CompanionImprovedEvasion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Ambusher ~ Camouflage", at_level: 1, description: Some("An ambusher gains a +4 racial bonus on Stealth checks in its natural environment (if this is unclear, use the environment in its Bestiary entry)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ambusher ~ Cunning Ambusher", at_level: 1, description: Some("An ambusher gains uncanny dodge."), benefit: None },
            ],
        },
        // Companion Archetype ~ Augmented Companion -- uw_abilities_companion.lst:417
        ArchetypeSwapEntry {
            key: "Companion Archetype ~ Augmented Companion",
            subject: "Companion",
            archetype_name: "Augmented Companion",
            description: Some("Augmented companions have suffered an injury, such as the loss of a wing, and parts of their bodies have been replaced by a master construct crafter. This procedure grants them unusual abilities."),
            source_page: Some("p.186"),
            prerequisites: Some(&["PRECLASS:1,Companion=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Companion Archetype ~ Augmented Companion],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_CompanionShareSpells,TYPE.CF_CompanionDevotion]"]),
            replaces: Some(&["CF_CompanionShareSpells", "CF_CompanionDevotion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Augmented Companion ~ Not Quite Animal", at_level: 1, description: Some("The DC to use Handle Animal on an augmented companion is 5 higher, as if it were a nonanimal with an Intelligence score of 1 or 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Augmented Companion ~ Augmented Body", at_level: 1, description: Some("An augmented companion heals only half as much as normal from positive energy healing effects but also heals half the usual amount from effects that specifically heal constructs."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Augmented Companion ~ Augmented Sight", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Augmented Companion ~ Constructed Form", at_level: 1, description: Some("An augmented companion gains a +4 morale bonus on Fortitude saves against effects that could not normally affect objects or constructs, and it takes only half damage from bleed effects. However, it can be affected by attacks that specifically target constructs."), benefit: None },
            ],
        },
        // Companion Archetype ~ Auspice -- uw_abilities_companion.lst:418
        ArchetypeSwapEntry {
            key: "Companion Archetype ~ Auspice",
            subject: "Companion",
            archetype_name: "Auspice",
            description: Some("Auspices were born with a birthmark or other feature that seems to be in the shape of their master's deity's holy symbol; they are usually animal sacred to that deity's religion."),
            source_page: Some("p.187"),
            prerequisites: Some(&["PRECLASS:1,Companion=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Companion Archetype ~ Auspice],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_CompanionShareSpells,TYPE.CF_CompanionEvasion,TYPE.CF_CompanionImprovedEvasion]"]),
            replaces: Some(&["CF_CompanionShareSpells", "CF_CompanionEvasion", "CF_CompanionImprovedEvasion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Auspice ~ Aligned", at_level: 1, description: Some("Even though an auspice is an animal, its connection to its associated deity allows it to be chaotic neutral, lawful neutral, neutral evil, or neutral good, whichever is closest to the deity's alignment, or it can remain true neutral."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Auspice ~ Auspicious Boon", at_level: 1, description: Some("[NOT IMPLEMENTED] The auspice can cast guidance as a spell-like ability at will as a full-round action."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Auspice ~ Aligned Strike", at_level: 1, description: Some("An auspice's attacks count as chaotic, evil, good, or lawful (whichever matches its alignment) for the purpose of overcoming damage reduction, unless the auspice is true neutral."), benefit: None },
            ],
        },
        // Companion Archetype ~ Bodyguard -- uw_abilities_companion.lst:419
        ArchetypeSwapEntry {
            key: "Companion Archetype ~ Bodyguard",
            subject: "Companion",
            archetype_name: "Bodyguard",
            description: Some("Some companions live lives of faithful devotion and steady vigilance, standing watch through long hours and always ready to leap into action to protect their master."),
            source_page: Some("p.187"),
            prerequisites: Some(&["PRECLASS:1,Companion=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Companion Archetype ~ Bodyguard],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_CompanionAnimalFeats,TYPE.CF_CompanionShareSpells,TYPE.CF_CompanionEvasion,TYPE.CF_CompanionMultiattack,TYPE.CF_CompanionImprovedEvasion]"]),
            replaces: Some(&["CF_CompanionShareSpells", "CF_CompanionAnimalFeats", "CF_CompanionMultiattack", "CF_CompanionEvasion", "CF_CompanionImprovedEvasion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Bodyguard ~ Animal Feats", at_level: 1, description: Some("In addition to the standard animal companion feats, a bodyguard may select Bodyguard, Combat Patrol, Heroic Defiance, Heroic Recovery, and In Harm's Way."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bodyguard ~ Shared Vigilance", at_level: 1, description: Some("A bodyguard and its master both gain Alertness as a bonus feat whenever they are adjacent."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bodyguard ~ Tenacious Guardian", at_level: 1, description: Some("A bodyguard can always act in a surprise round (though it remains flat-footed until it acts). As long as its master is adjacent, a bodyguard remains conscious (though it becomes staggered) when its hit points fall below 0. While below 0 hit points, the bodyguard loses 1 hit point per round but gains a +2 morale bonus on attack rolls, saving throws, and skill checks, dying only if its hit points reach a negative total equal to its Constitution score plus its master's class level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bodyguard ~ Uncanny Dodge", at_level: 1, description: Some("A bodyguard gains uncanny dodge, as per the rogue class feature of the same name."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bodyguard ~ Greater Tenacity", at_level: 1, description: Some("A bodyguard with fewer than 0 hit points gains a +4 morale bonus on attack rolls, saving throws, and skill checks; immunity to fear effects; and temporary hit points equal to its master's class level (maximum 20). It dies only if its hit points reach a negative total equal to twice its Constitution score + its master's class level."), benefit: None },
            ],
        },
        // Companion Archetype ~ Bully -- uw_abilities_companion.lst:420
        ArchetypeSwapEntry {
            key: "Companion Archetype ~ Bully",
            subject: "Companion",
            archetype_name: "Bully",
            description: Some("Bigger than others of its kind, a bully is used to winning fights and displays of dominance for its choice of mates, territory, or other privileges."),
            source_page: Some("p.187"),
            prerequisites: Some(&["PRECLASS:1,Companion=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Companion Archetype ~ Bully],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_CompanionShareSpells,TYPE.CF_CompanionMultiattack]"]),
            replaces: Some(&["CF_CompanionShareSpells", "CF_CompanionMultiattack"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Bully ~ Bully Feats", at_level: 1, description: Some("In addition to the standard feats available to animal companions, a bully can select Greater Bull Rush, Greater Overrun, Greater Reposition APG , Greater Trip, Improved Reposition, and Improved Trip. It can use Power Attack instead of Combat Expertise as a prerequisite to qualify for feats on this list."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Bully ~ Bullying Thrill", at_level: 1, description: Some("Whenever a bully succeeds at a bull rush, overrun, reposition, or trip combat maneuver check, after fully resolving the combat maneuver, it gains a +2 morale bonus on attack and damage rolls until the end of its next turn."), benefit: None },
            ],
        },
        // Companion Archetype ~ Daredevil -- uw_abilities_companion.lst:421
        ArchetypeSwapEntry {
            key: "Companion Archetype ~ Daredevil",
            subject: "Companion",
            archetype_name: "Daredevil",
            description: Some("Daredevil companions join the fray with graceful leaps or swooping dives, heedless of the danger."),
            source_page: Some("p.187"),
            prerequisites: Some(&["PRECLASS:1,Companion=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Companion Archetype ~ Daredevil],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_CompanionShareSpells,TYPE.CF_CompanionDevotion,TYPE.CF_CompanionMultiattack]"]),
            replaces: Some(&["CF_CompanionShareSpells", "CF_CompanionDevotion", "CF_CompanionMultiattack"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Daredevil ~ Artful Acrobat", at_level: 1, description: Some("A daredevil gains a competence bonus on Acrobatics checks equal to half its Hit Dice."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Daredevil ~ Uncanny Dodge", at_level: 1, description: Some("A daredevil gains Mobility as a bonus feat without needing to meet the prerequisites. If it already has Mobility, it gains Spring Attack instead."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Daredevil ~ Devil May Care", at_level: 1, description: Some("A daredevil can't be flanked."), benefit: None },
            ],
        },
        // Companion Archetype ~ Deathtouched Companion -- uw_abilities_companion.lst:422
        ArchetypeSwapEntry {
            key: "Companion Archetype ~ Deathtouched Companion",
            subject: "Companion",
            archetype_name: "Deathtouched Companion",
            description: Some("Whether the result of a partially successful attempt at revival, a strange blight, or repeated exposure to undead, deathtouched companions are living animals with a trace of the undead, somewhat like dhampirs."),
            source_page: Some("p.187"),
            prerequisites: Some(&["PRECLASS:1,Companion=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Companion Archetype ~ Deathtouched Companion],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_CompanionShareSpells,TYPE.CF_CompanionDevotion]"]),
            replaces: Some(&["CF_CompanionShareSpells", "CF_CompanionDevotion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Deathtouched Companion ~ Deathtouched Skills", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Deathtouched Companion ~ Not Quite Animal", at_level: 1, description: Some("The DC to use Handle Animal on a deathtouched companion is 5 higher, as if it were a nonanimal with an Intelligence score of 1 or 2. It doesn't count as an animal for the purpose of an undead's unnatural aura."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deathtouched Companion ~ Dead Sight", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Deathtouched Companion ~ Negative Energy Affinity", at_level: 1, description: Some("Though a living creature, a deathtouched companion reacts to positive and negative energy as if it were undead-positive energy harms it, while negative energy heals it."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Deathtouched Companion ~ One Foot in the Grave", at_level: 1, description: Some("A deathtouched companion gains a +4 morale bonus on Fortitude saves against effects that could not normally affect objects or undead, and it takes only half damage from bleed effects. However, it can be affected by attacks that specifically target undead, such as halt undead."), benefit: None },
            ],
        },
        // Companion Archetype ~ Draconic Companion -- uw_abilities_companion.lst:423
        ArchetypeSwapEntry {
            key: "Companion Archetype ~ Draconic Companion",
            subject: "Companion",
            archetype_name: "Draconic Companion",
            description: Some("Draconic companions bear a faint trace of dragon blood that grants them special abilities."),
            source_page: Some("p.188"),
            prerequisites: Some(&["PRECLASS:1,Companion=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Companion Archetype ~ Draconic Companion],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_CompanionShareSpells,TYPE.CF_CompanionEvasion,TYPE.CF_CompanionDevotion,TYPE.CF_CompanionMultiattack,TYPE.CF_CompanionImprovedEvasion]"]),
            replaces: Some(&["CF_CompanionShareSpells", "CF_CompanionDevotion", "CF_CompanionMultiattack", "CF_CompanionEvasion", "CF_CompanionImprovedEvasion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Draconic Companion ~ Draconic Skills", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Draconic Companion ~ Not Quite Animal", at_level: 1, description: Some("The DC to use Handle Animal on a draconic companion is 5 higher, as if it were a nonanimal with an Intelligence score of 1 or 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Draconic Companion ~ Draconic Sight", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Draconic Companion ~ Draconic Resistance", at_level: 1, description: Some("Choose acid, cold, electricity, or fire, based on the draconic companion's draconic ancestor. The draconic companion gains resistance against the chosen energy type, as well as a +2 racial bonus on saves against paralysis and sleep."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Draconic Companion ~ Improved Draconic Resistance", at_level: 1, description: Some("A draconic companion becomes immune to paralysis and sleep.|DraconicCompanionResistanceBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Draconic Companion ~ Breath Weapon Choice", at_level: 1, description: None, benefit: None },
            ],
        },
        // Companion Archetype ~ Feytouched Companion -- uw_abilities_companion.lst:424
        ArchetypeSwapEntry {
            key: "Companion Archetype ~ Feytouched Companion",
            subject: "Companion",
            archetype_name: "Feytouched Companion",
            description: Some("Feytouched companions are strangely colored fauna native to the First World, the primal realm of the fey. They gain strange abilities tied to the fey."),
            source_page: Some("p.188"),
            prerequisites: Some(&["PRECLASS:1,Companion=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Companion Archetype ~ Feytouched Companion],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_CompanionShareSpells,TYPE.CF_CompanionEvasion,TYPE.CF_CompanionDevotion,TYPE.CF_CompanionMultiattack,TYPE.CF_CompanionImprovedEvasion,TYPE.CF_CompanionAdvancement]", "PRESIZEEQ:S"]),
            replaces: Some(&["CF_CompanionShareSpells", "CF_CompanionDevotion", "CF_CompanionMultiattack", "CF_CompanionEvasion", "CF_CompanionImprovedEvasion", "CF_CompanionAdvancement"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Feytouched Companion ~ Feytouched Skills", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Feytouched Companion ~ Not Quite Animal", at_level: 1, description: Some("The DC to use Handle Animal on a feytouched companion is 5 higher, as if it were a nonanimal with an Intelligence score of 1 or 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feytouched Companion ~ Fey Magic", at_level: 1, description: Some("A feytouched companion can cast dancing lights at will as a spell-like ability as a full-round action."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Feytouched Companion ~ Iron Bane", at_level: 1, description: Some("A feytouched companion gains damage reduction %1/cold iron. Any creature holding or wearing an item made out of cold iron takes a -10 penalty on Handle Animal checks to handle the feytouched companion.|FeytouchedCompanionDR"), benefit: None },
            ],
        },
        // Companion Archetype ~ Precocious Companion -- uw_abilities_companion.lst:425
        ArchetypeSwapEntry {
            key: "Companion Archetype ~ Precocious Companion",
            subject: "Companion",
            archetype_name: "Precocious Companion",
            description: Some("Precocious companions are able to learn far more tricks than other companions; this ability to learn allows their masters to use them for a wider variety of tasks."),
            source_page: Some("p.188"),
            prerequisites: Some(&["PRECLASS:1,Companion=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Companion Archetype ~ Precocious Companion],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_CompanionEvasion,TYPE.CF_CompanionImprovedEvasion,TYPE.CF_CompanionAdvancement]"]),
            replaces: Some(&["CF_CompanionEvasion", "CF_CompanionImprovedEvasion", "CF_CompanionAdvancement"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Precocious Companion ~ Expanded Tricks", at_level: 1, description: Some("A precocious companion gains twice as many bonus tricks as normal for an animal companion. A hunter's animal companion can't use the additional bonus tricks to learn hunter's tricks."), benefit: None },
            ],
        },
        // Companion Archetype ~ Racer -- uw_abilities_companion.lst:426
        ArchetypeSwapEntry {
            key: "Companion Archetype ~ Racer",
            subject: "Companion",
            archetype_name: "Racer",
            description: Some("Some companions have uncanny speed, providing their masters with swift transport."),
            source_page: Some("p.188"),
            prerequisites: Some(&["PRECLASS:1,Companion=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Companion Archetype ~ Racer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_CompanionAnimalFeats,TYPE.CF_CompanionShareSpells,TYPE.CF_CompanionDevotion]"]),
            replaces: Some(&["CF_CompanionShareSpells", "CF_CompanionDevotion", "CF_CompanionAnimalFeats"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Racer ~ Animal Feats", at_level: 1, description: Some("In addition to the standard feats available to animal companions, a racer can select Acrobatic Steps, Charge Through, Improved Lightning Reflexes, Lightning Stance, Nimble Moves, and Wind Stance."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Racer ~ Fast Movement", at_level: 1, description: Some("A racer's speed is 10 feet greater than typical animals of its kind when wearing no armor and carrying a light load."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Racer ~ Sprint", at_level: 1, description: Some("Once per hour a racer can move at 10 times its normal speed when it makes a charge or uses the run action."), benefit: None },
            ],
        },
        // Companion Archetype ~ Totem Guide -- uw_abilities_companion.lst:427
        ArchetypeSwapEntry {
            key: "Companion Archetype ~ Totem Guide",
            subject: "Companion",
            archetype_name: "Totem Guide",
            description: Some("Totem guides embody the wisdom and spirituality of the natural world, providing guidance as well as aid in combat. Once a totem guide is chosen, a character may replace the companion if it is slain but may not choose a different kind of totem guide."),
            source_page: Some("p.189"),
            prerequisites: Some(&["PRECLASS:1,Companion=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Companion Archetype ~ Totem Guide],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_CompanionEvasion,TYPE.CF_CompanionDevotion,TYPE.CF_CompanionMultiattack,TYPE.CF_CompanionImprovedEvasion]"]),
            replaces: Some(&["CF_CompanionDevotion", "CF_CompanionMultiattack", "CF_CompanionEvasion", "CF_CompanionImprovedEvasion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Totem Guide ~ Spiritual Guidance", at_level: 1, description: Some("A totem guide can use guidance as a spell-like ability at will as a full-round action, targeting itself or its master. In addition, a totem guide's master can spontaneously cast the following spells while adjacent to the totem guide by sacrificing a spell slot of equal or higher level: detect animals or plants (1st), augury (2nd), helping hand (3rd), divination (4th), commune with nature (5th), and find the path (6th). The totem guide serves as a divine focus for these spells, but costly material components must still be provided."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Totem Guide ~ Beast Speech", at_level: 1, description: Some("A totem guide can speak with its master as though the two shared a common language, and it can speak with other animals of its species (or, at the GM's discretion, other creatures with similar types)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Totem Guide ~ Eldritch Claws", at_level: 1, description: Some("A totem guide gains Eldritch Claws as a bonus feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Totem Guide ~ Ghost Guardian", at_level: 1, description: Some("A totem guide's natural weapons deal full damage to incorporeal creatures and its natural armor bonus applies against incorporeal touch attacks. A totem guide can never be raised or animated as an undead creature."), benefit: None },
            ],
        },
        // Companion Archetype ~ Tracker -- uw_abilities_companion.lst:428
        ArchetypeSwapEntry {
            key: "Companion Archetype ~ Tracker",
            subject: "Companion",
            archetype_name: "Tracker",
            description: Some("Some companions are expert trackers, able to use their scent ability to follow any trail."),
            source_page: Some("p.189"),
            prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Scent", "PRECLASS:1,Companion=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Companion Archetype ~ Tracker],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_CompanionShareSpells,TYPE.CF_CompanionDevotion]"]),
            replaces: Some(&["CF_CompanionShareSpells", "CF_CompanionDevotion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Tracker ~ Tracker Skills", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Tracker ~ Expert Tracker", at_level: 1, description: Some("A tracker gains a competence bonus on Survival checks to track via scent equal to half its total Hit Dice."), benefit: None },
            ],
        },
        // Companion Archetype ~ Verdant Companion -- uw_abilities_companion.lst:429
        ArchetypeSwapEntry {
            key: "Companion Archetype ~ Verdant Companion",
            subject: "Companion",
            archetype_name: "Verdant Companion",
            description: Some("Favorites of druids who balance their responsibilities between flora and fauna, verdant companions are animals with some of the abilities and physical aspects of plants."),
            source_page: Some("p.189"),
            prerequisites: Some(&["PRECLASS:1,Companion=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Companion Archetype ~ Verdant Companion],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_CompanionShareSpells,TYPE.CF_CompanionDevotion,TYPE.CF_CompanionEvasion,TYPE.CF_CompanionImprovedEvasion]"]),
            replaces: Some(&["CF_CompanionShareSpells", "CF_CompanionDevotion", "CF_CompanionEvasion", "CF_CompanionImprovedEvasion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Verdant Companion ~ Not Quite Animal", at_level: 1, description: Some("The DC to use Handle Animal on a verdant companion is 5 higher, as if it were a nonanimal with an Intelligence score of 1 or 2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Verdant Companion ~ Photosynthesis", at_level: 1, description: Some("As long as it stays in the sunlight for at least 8 hours per day, a verdant companion doesn't need to eat. Otherwise, it eats normally for an animal of its kind."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Verdant Companion ~ Verdant Resistance", at_level: 1, description: Some("A verdant companion gains a +%1 racial bonus on saving throws against mind-affecting effects, paralysis, poison, polymorph, sleep effects, and stunning.|VerdantCompanionResistanceBonus"), benefit: None },
            ],
        },
        // Companion Archetype ~ Wrecker -- uw_abilities_companion.lst:430
        ArchetypeSwapEntry {
            key: "Companion Archetype ~ Wrecker",
            subject: "Companion",
            archetype_name: "Wrecker",
            description: Some("Like unruly pets, some companions have a tendency to destroy nearby objects while unsupervised, and their masters can channel these destructive impulses into an advantage in battle."),
            source_page: Some("p.189"),
            prerequisites: Some(&["PRECLASS:1,Companion=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Companion Archetype ~ Wrecker],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_CompanionShareSpells,TYPE.CF_CompanionMultiattack]"]),
            replaces: Some(&["CF_CompanionShareSpells", "CF_CompanionMultiattack"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Wrecker ~ Wrecker Feats", at_level: 1, description: Some("In addition to the standard feats available to animal companions, a wrecker can select Improved Sunder and Greater Sunder."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Wrecker ~ Destructive Wrecker", at_level: 1, description: Some("A wrecker's natural attacks ignore an inanimate object's first 5 points of hardness.|PREVARLT:MasterLevel,9"), benefit: None },
            ],
        },
        // Familiar Archetype ~ Ambassador -- uw_abilities_companion.lst:533
        ArchetypeSwapEntry {
            key: "Familiar Archetype ~ Ambassador",
            subject: "Familiar",
            archetype_name: "Ambassador",
            description: Some("An ambassador speaks on its master's behalf and sometimes on behalf of its master's patron or other extraplanar contacts."),
            source_page: Some("p.210"),
            prerequisites: Some(&["PRELANG:1,ANY", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Familiar Archetype ~ Ambassador],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_FamiliarAlertness,TYPE.CF_FamiliarIntelligenceScore]"]),
            replaces: Some(&["CF_FamiliarAlertness", "CF_FamiliarIntelligenceScore"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Ambassador ~ Class Skills", at_level: 1, description: Some("An ambassador treats Bluff, Diplomacy, and Intimidate as class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ambassador ~ Persuasive", at_level: 1, description: Some("An ambassador gains Persuasive as a bonus feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Ambassador ~ Enhanced Personality", at_level: 1, description: Some("An ambassador gains a Charisma score equal to the typical Intelligence score of a familiar of its level, if that would be higher than its normal Charisma score. The familiar's Intelligence score remains 6 (or its normal starting Intelligence for an improved familiar) and doesn't increase by level."), benefit: None },
            ],
        },
        // Familiar Archetype ~ Animal Exemplar -- uw_abilities_companion.lst:534
        ArchetypeSwapEntry {
            key: "Familiar Archetype ~ Animal Exemplar",
            subject: "Familiar",
            archetype_name: "Animal Exemplar",
            description: Some("An animal exemplar is a paragon of its species, able to command the loyalty of others of its kind. An animal exemplar familiar cannot be an improved familiar, plant, vermin, or other nonanimal themed familiar."),
            source_page: Some("p.210"),
            prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Familiar Archetype ~ Animal Exemplar],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_FamiliarSpeakwithAnimalsofItsKind,TYPE.CF_FamiliarDeliverTouchSpells,TYPE.CF_FamiliarSpellResistance,TYPE.CF_FamiliarScryonFamiliar]", "PRERACE:1,RACESUBTYPE=Augmented Magical Beast"]),
            replaces: Some(&["CF_FamiliarSpeakwithAnimalsofItsKind", "CF_FamiliarDeliverTouchSpells", "CF_FamiliarSpellResistance", "CF_FamiliarScryonFamiliar"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Animal Exemplar ~ Class Skills", at_level: 1, description: Some("An animal exemplar treats Handle Animal as a class skill."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Familiar ~ Speak with Animals of Its Kind", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Animal Exemplar ~ Influence Animals", at_level: 1, description: Some("An animal exemplar gains the ability to influence animals of its kind, as per the wild empathy ability of a druid of its master's level with a +4 racial bonus on the check, but only for animals of its kind."), benefit: None },
            ],
        },
        // Familiar Archetype ~ Egotist -- uw_abilities_companion.lst:535
        ArchetypeSwapEntry {
            key: "Familiar Archetype ~ Egotist",
            subject: "Familiar",
            archetype_name: "Egotist",
            description: Some("An egotist believes itself to be the real master in the relationship-the power behind the throne. It often attempts to communicate \"orders\" for its master as best it can, interfering in matters ranging from spell choices and tactical combat decisions to its master's love life."),
            source_page: Some("p.210"),
            prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Familiar Archetype ~ Egotist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_FamiliarAlertness,TYPE.CF_FamiliarSpeakwithMaster,TYPE.CF_FamiliarDeliverTouchSpells,TYPE.CF_FamiliarScryonFamiliar]"]),
            replaces: Some(&["CF_FamiliarAlertness", "CF_FamiliarSpeakwithMaster", "CF_FamiliarDeliverTouchSpells", "CF_FamiliarScryonFamiliar"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Egotist ~ Class Skills", at_level: 1, description: Some("An egotist treats Intimidate as a class skill."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Egotist ~ Song of Myself", at_level: 1, description: Some("An egotist gains Alertness as a bonus feat rather than providing that feat to its master. [Variable Familiar Bonus alteration NOT IMPLEMENTED] It gains the variable familiar bonus (such as the bat's +3 bonus on Fly checks) instead of granting the bonus to its master. This alters alertness and the variable familiar bonus."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Familiar ~ Speak with Master", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Egotist ~ Receive Touch Spells", at_level: 1, description: Some("An egotist can demand a particular touch spell as a standard action. If it does so, until the egotist's next turn, its master can cast that spell on the egotist once as a ranged touch within close range, as if using Reach Spell."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Egotist ~ Scry on Master", at_level: 1, description: Some("An egotist can scry on its master (as if casting the scrying spell) once per day."), benefit: None },
            ],
        },
        // Familiar Archetype ~ Emissary -- uw_abilities_companion.lst:536
        ArchetypeSwapEntry {
            key: "Familiar Archetype ~ Emissary",
            subject: "Familiar",
            archetype_name: "Emissary",
            description: Some("The emissary is touched by the divine, serving as a font of wisdom and a moral compass for its master. An emissary familiar can serve only a master who worships a single deity."),
            source_page: Some("p.210"),
            prerequisites: Some(&["PREDEITY:1,Y", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Familiar Archetype ~ Emissary],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_FamiliarAlertness,TYPE.CF_FamiliarShareSpells,TYPE.CF_FamiliarDeliverTouchSpells]"]),
            replaces: Some(&["CF_FamiliarAlertness", "CF_FamiliarShareSpells", "CF_FamiliarDeliverTouchSpells"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Emissary ~ Class Skills", at_level: 1, description: Some("An emissary treats Heal, Knowledge (religion), and Sense Motive as class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Emissary ~ Divine Guidance", at_level: 1, description: Some("An emissary can cast guidance at will."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Emissary ~ Share Will", at_level: 1, description: Some("Whenever an emissary or its master fails a saving throw against a mind-affecting effect that affects only one of them, the other can attempt the saving throw as well. If this second saving throw is a success, treat the original result as a success, and the emissary and its master can't use this ability again for 24 hours. On a failure, both the emissary and its master suffer the effects of the failed saving throw, even if one of them wouldn't ordinarily be a valid target."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Emissary ~ Domain Influence", at_level: 1, description: Some("The emissary gains a spark of divine power from the deity its master worships. Choose one of that deity's domains that grants a 1st-level domain power usable a number of times per day equal to 3 + the user's Wisdom modifier. The emissary can use that power once per day. [NOTE:Not fully restricted. Check domain power before choosing!]"), benefit: None },
            ],
        },
        // Familiar Archetype ~ Figment -- uw_abilities_companion.lst:537
        ArchetypeSwapEntry {
            key: "Familiar Archetype ~ Figment",
            subject: "Familiar",
            archetype_name: "Figment",
            description: Some("Figments are born from their masters' imaginations rather than being ordinary creatures that are awakened."),
            source_page: Some("p.211"),
            prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Familiar Archetype ~ Figment],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_FamiliarImprovedEvasion,TYPE.CF_FamiliarDeliverTouchSpells,TYPE.CF_FamiliarSpeakwithAnimalsofItsKind,TYPE.CF_FamiliarScryonFamiliar,TYPE.CF_FamiliarHalfMastHP]", "PREMULT:2,[!PREVARGTEQ:mastervar(WitchLVL),1],[!PREVARGTEQ:mastervar(ShamanLVL),1]"]),
            replaces: Some(&["CF_FamiliarImprovedEvasion", "CF_FamiliarDeliverTouchSpells", "CF_FamiliarSpeakwithAnimalsofItsKind", "CF_FamiliarScryonFamiliar", "CF_FamiliarHalfMastHP"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Figment ~ Recurring Dream", at_level: 1, description: Some("A figment has a total number of hit points equal to 1/4 of its master's total hit points. If the figment dies, it vanishes, appearing again with 1 hit point after its master awakens from a full night's sleep. If a figment ever strays more than 100 feet from its master or enters an antimagic field, or if a figment's master is unconscious or asleep, the figment disappears until the next time its master prepares spells or regains her spells per day."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Figment ~ Manifest Dreams", at_level: 1, description: Some("A figment is shaped by its master's dreams. Each time the master awakens from a full night's rest, he can apply to the figment %1 evolution points' worth of eidolon evolutions that don't have a base form requirement.|FigmentEvolutionPoints"), benefit: None },
            ],
        },
        // Familiar Archetype ~ Infiltrator -- uw_abilities_companion.lst:538
        ArchetypeSwapEntry {
            key: "Familiar Archetype ~ Infiltrator",
            subject: "Familiar",
            archetype_name: "Infiltrator",
            description: Some("Familiars can be skilled spies, and infiltrators are the very best of their kind at the subtle art of espionage."),
            source_page: Some("p.211"),
            prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Familiar Archetype ~ Infiltrator],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_FamiliarAlertness,TYPE.CF_FamiliarShareSpells,TYPE.CF_FamiliarImprovedEvasion,TYPE.CF_FamiliarSpeakwithAnimalsofItsKind,TYPE.CF_FamiliarSpellResistance]"]),
            replaces: Some(&["CF_FamiliarAlertness", "CF_FamiliarShareSpells", "CF_FamiliarImprovedEvasion", "CF_FamiliarSpeakwithAnimalsofItsKind", "CF_FamiliarSpellResistance"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Infiltrator ~ Class Skills", at_level: 1, description: Some("An infiltrator treats Bluff and Disguise as class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Infiltrator ~ Alertness", at_level: 1, description: Some("An infiltrator gains Alertness as a bonus feat rather than providing that feat to its master."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Infiltrator ~ Share Spells", at_level: 1, description: Some("The master may cast a divination spell with a target of \"You\" on the infiltrator (as a spell with a range of touch) instead of on herself. A master may cast spells on the infiltrator even if the spells normally do not affect creatures of the companion's type. Spells cast in this way must come from a class that grants a familiar. This ability does not allow the infiltrator to share abilities that are not spells, even if they function like spells."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Infiltrator ~ Uncanny Stealth", at_level: 1, description: Some("An infiltrator gains uncanny dodge and improved uncanny dodge, treating its master's level as its effective rogue level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Infiltrator ~ Scry on Familiar", at_level: 1, description: Some("An infiltrator's master can scry on it for up to %1 minutes per day. The duration does not need to be consecutive, but it must be used in 1-minute increments.|MasterLevel"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Infiltrator ~ Telepathic Bond", at_level: 1, description: Some("An infiltrator gains a permanent telepathic bond with its master. This bond has no range limit as long as the familiar and its master are on the same plane."), benefit: None },
            ],
        },
        // Familiar Archetype ~ Mascot -- uw_abilities_companion.lst:539
        ArchetypeSwapEntry {
            key: "Familiar Archetype ~ Mascot",
            subject: "Familiar",
            archetype_name: "Mascot",
            description: Some("A familiar sometimes serves as the centerpiece of an adventuring party. Known as a mascot, this type of familiar eventually treats the entire party as its master."),
            source_page: Some("p.211"),
            prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Familiar Archetype ~ Mascot],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_FamiliarAlertness,TYPE.CF_FamiliarEmpathicLink,TYPE.CF_FamiliarImprovedEvasion,TYPE.CF_FamiliarShareSpells,TYPE.CF_FamiliarDeliverTouchSpells,TYPE.CF_FamiliarSpeakwithMaster,TYPE.CF_FamiliarSpeakwithAnimalsofItsKind,TYPE.CF_FamiliarSpellResistance,TYPE.CF_FamiliarScryonFamiliar]"]),
            replaces: Some(&["CF_FamiliarAlertness", "CF_FamiliarEmpathicLink", "CF_FamiliarImprovedEvasion", "CF_FamiliarShareSpells", "CF_FamiliarDeliverTouchSpells", "CF_FamiliarSpeakwithMaster", "CF_FamiliarSpeakwithAnimalsofItsKind", "CF_FamiliarSpellResistance", "CF_FamiliarScryonFamiliar"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Mascot ~ Class Skills", at_level: 1, description: Some("A mascot treats all Perform skills as class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Familiar ~ Empathic Link", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Mascot ~ Affinity for My Team", at_level: 1, description: Some("A mascot is the heart and soul of its team. At first, the team consists of only the familiar and its master, but at 3rd level and every 3 levels thereafter, a mascot can add an additional member to its team. A mascot's empathic link extends to all members of its team. A mascot can add or remove one team member over the course of a day. This alters empathic link."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mascot ~ Lucky Mascot", at_level: 1, description: Some("Whenever a mascot uses the aid another action to improve a team member's attack roll or AC, that team member also gains a +1 luck bonus to AC for 1 round."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Familiar ~ Share Spells", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Mascot ~ Share Spells", at_level: 1, description: Some("Spells that target a mascot via its share spells ability function at its master's caster level - 2. The mascot also benefit from the spells of any team member when it is using share spells. This alters share spells."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Familiar ~ Deliver Touch Spells", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Mascot ~ Deliver Touch Spells", at_level: 1, description: Some("Spells delivered by a mascot's deliver touch spells ability function at its master's caster level - 2. The mascot can deliver the touch spells of any of its team members. This alters deliver touch spells."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mascot ~ Speak with Team", at_level: 1, description: Some("A mascot gains the ability to speak with all members of its team verbally as if using speak with master."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mascot ~ Heart of the Team", at_level: 1, description: Some("Once per day at as a full-round action, a mascot can designate any member of its team as its master for the purpose of calculating its base attack bonus, Hit Dice, hit points, saving throws, and skill ranks."), benefit: None },
            ],
        },
        // Familiar Archetype ~ Mauler -- uw_abilities_companion.lst:540
        ArchetypeSwapEntry {
            key: "Familiar Archetype ~ Mauler",
            subject: "Familiar",
            archetype_name: "Mauler",
            description: Some("While most familiars are scouts and assistants, the mauler familiar cares only for the thrill of battle. A mauler often serves a bloodthirsty or martial-minded master."),
            source_page: Some("p.212"),
            prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Familiar Archetype ~ Mauler],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_FamiliarSpeakwithMaster,TYPE.CF_FamiliarSpeakwithAnimalsofItsKind,TYPE.CF_FamiliarIntelligenceScore,TYPE.CF_FamiliarDeliverTouchSpells,TYPE.CF_FamiliarSpellResistance]"]),
            replaces: Some(&["CF_FamiliarSpeakwithMaster", "CF_FamiliarSpeakwithAnimalsofItsKind", "CF_FamiliarIntelligenceScore", "CF_FamiliarDeliverTouchSpells", "CF_FamiliarSpellResistance"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Mauler ~ Class Skills", at_level: 1, description: Some("A mauler treats Intimidate as a class skill."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mauler ~ Bond Forged in Blood", at_level: 1, description: Some("A mauler isn't impressed by fancy words-only furious battle. A mauler can't speak or communicate via language in any way, even if it's a type of creature that normally could."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mauler ~ Increased Strength", at_level: 1, description: Some("At 3rd level and every 2 levels thereafter, a mauler's Strength score increases by 1. As a result of this ability, the familiar's Intelligence score remains 6; a mauler can never have an Intelligence score higher than 6."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mauler ~ Battle Form", at_level: 1, description: Some("A mauler gains the ability to transform into a larger, more ferocious form and back as a standard action three times per day. In battle form, the mauler's size becomes Medium and it gains a +2 size bonus to its Strength score. Since many familiars are Tiny or Diminutive, be sure to check for any additional Strength and Dexterity adjustments for increasing in size from Tiny or Diminutive to Medium (Pathfinder RPG Core Rulebook 212). This is a polymorph effect."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mauler ~ Damage Reduction", at_level: 1, description: Some("A mauler gains DR 5/magic."), benefit: None },
            ],
        },
        // Familiar Archetype ~ Pilferer -- uw_abilities_companion.lst:541
        ArchetypeSwapEntry {
            key: "Familiar Archetype ~ Pilferer",
            subject: "Familiar",
            archetype_name: "Pilferer",
            description: Some("Some familiars are stealthy pilferers who perform tricks of thievery or simple spying on their master's behalf."),
            source_page: Some("p.212"),
            prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Familiar Archetype ~ Pilferer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_FamiliarAlertness,TYPE.CF_FamiliarImprovedEvasion,TYPE.CF_FamiliarDeliverTouchSpells,TYPE.CF_FamiliarSpeakwithAnimalsofItsKind]"]),
            replaces: Some(&["CF_FamiliarAlertness", "CF_FamiliarImprovedEvasion", "CF_FamiliarDeliverTouchSpells", "CF_FamiliarSpeakwithAnimalsofItsKind"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Pilferer ~ Class Skills", at_level: 1, description: Some("A pilferer treats Disable Device, Escape Artist, and Sleight of Hand as class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Pilferer ~ Improved Steal", at_level: 1, description: Some("A pilferer gains Improved Steal as a bonus feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Pilferer ~ Nondetection", at_level: 1, description: Some("A pilferer is under the constant effect of nondetection with a caster level of %1. The DC of the caster level check to penetrate the nondetection effect is %2.|MasterLevel|MasterLevel+15"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Pilferer ~ Sneak", at_level: 1, description: Some("A pilferer gains a +%1 competence bonus on Sleight of Hand and Stealth checks.|MasterLevel/2"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Pilferer ~ Greater Steal", at_level: 1, description: Some("A pilferer gains Greater Steal as a bonus feat."), benefit: None },
            ],
        },
        // Familiar Archetype ~ Prankster -- uw_abilities_companion.lst:542
        ArchetypeSwapEntry {
            key: "Familiar Archetype ~ Prankster",
            subject: "Familiar",
            archetype_name: "Prankster",
            description: Some("Some familiars love to perform pranks on their unsuspecting masters, as well as on those around their masters-allies and enemies alike. While good-aligned pranksters' tricks are usually good natured except against foes, evil pranksters play tricks that are mean spirited or even downright cruel."),
            source_page: Some("p.212"),
            prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Familiar Archetype ~ Prankster],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_FamiliarEmpathicLink,TYPE.CF_FamiliarAlertness,TYPE.CF_FamiliarImprovedEvasion,TYPE.CF_FamiliarShareSpells,TYPE.CF_FamiliarDeliverTouchSpells,TYPE.CF_FamiliarSpellResistance,TYPE.CF_FamiliarScryonFamiliar]"]),
            replaces: Some(&["CF_FamiliarEmpathicLink", "CF_FamiliarAlertness", "CF_FamiliarImprovedEvasion", "CF_FamiliarShareSpells", "CF_FamiliarDeliverTouchSpells", "CF_FamiliarSpellResistance", "CF_FamiliarScryonFamiliar"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Prankster ~ Class Skills", at_level: 1, description: Some("A prankster treats Bluff, Disguise, Perform (comedy), and Sleight of Hand as class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Familiar ~ Empathic Link", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Prankster ~ Autonomous Link", at_level: 1, description: Some("A prankster can hide its feelings from its master via its empathic link at will. It can also try to project a false emotion through the link by attempting a Bluff check opposed by its master's Sense Motive. This alters empathic link."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Prankster ~ Improved Dirty Trick", at_level: 1, description: Some("A prankster gains Improved Dirty Trick APG as a bonus feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Prankster ~ Magical Pranks", at_level: 1, description: Some("A prankster can cast ghost sound, mage hand, and prestidigitation at will as spell-like abilities."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Prankster ~ Glib Comedy", at_level: 1, description: Some("A prankster gains a +%1 competence bonus on Bluff, Disguise, and Perform (comedy) checks.|MasterLevel/2"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Prankster ~ Greater Dirty Trick", at_level: 1, description: Some("A prankster gains Greater Dirty Trick as a bonus feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Familiar ~ Scry on Familiar", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Prankster ~ Unreliable Narrator", at_level: 1, description: Some("Whenever its master uses scry on familiar, a prankster can use false vision to fool that ability; this doesn't affect any other divination (scrying) effects in the area, only its master's scry on familiar ability. This alters scry on familiar."), benefit: None },
            ],
        },
        // Familiar Archetype ~ Protector -- uw_abilities_companion.lst:543
        ArchetypeSwapEntry {
            key: "Familiar Archetype ~ Protector",
            subject: "Familiar",
            archetype_name: "Protector",
            description: Some("Protector familiars are so devoted that they would give their lives for their masters. A tumor familiar can't be a protector."),
            source_page: Some("p.212"),
            prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Familiar Archetype ~ Protector],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_FamiliarAlertness,TYPE.CF_FamiliarImprovedEvasion,TYPE.CF_FamiliarDeliverTouchSpells,TYPE.CF_FamiliarSpeakwithAnimalsofItsKind,TYPE.CF_FamiliarSpellResistance]"]),
            replaces: Some(&["CF_FamiliarAlertness", "CF_FamiliarImprovedEvasion", "CF_FamiliarDeliverTouchSpells", "CF_FamiliarSpeakwithAnimalsofItsKind", "CF_FamiliarSpellResistance"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Protector ~ Loyal Bodyguard", at_level: 1, description: Some("A protector gains Bodyguard and Combat Reflexes as bonus feats. If the familiar is sharing its master's square, it can use Bodyguard to aid another to improve its master's AC even if it doesn't threaten the attacking foe, though it still needs line of effect to its master and the attacker."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Protector ~ Shield Master", at_level: 1, description: Some("Whenever a protector or its master takes hit point damage, as long as the protector and its master are touching, its master can split the damage evenly between them as if under the effects of shield other."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Protector ~ Able Defender", at_level: 1, description: Some("A protector gains In Harm's Way as a bonus feat. In addition, the familiar's hit points are now equal to its master's total hit points (not including temporary hit points), regardless of its actual Hit Dice."), benefit: None },
            ],
        },
        // Familiar Archetype ~ Sage -- uw_abilities_companion.lst:544
        ArchetypeSwapEntry {
            key: "Familiar Archetype ~ Sage",
            subject: "Familiar",
            archetype_name: "Sage",
            description: Some("Sages are masters of useful facts, able to recall them for their masters' benefit, though this leads many to become haughty and proud."),
            source_page: Some("p.213"),
            prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Familiar Archetype ~ Sage],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_FamiliarIntelligenceScore,TYPE.CF_FamiliarNaturalArmorBonus,TYPE.CF_FamiliarAlertness,TYPE.CF_FamiliarShareSkillRanks]"]),
            replaces: Some(&["CF_FamiliarIntelligenceScore", "CF_FamiliarNaturalArmorBonus", "CF_FamiliarAlertness", "CF_FamiliarShareSkillRanks"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Sage ~ Class Skills", at_level: 1, description: Some("A sage treats all Knowledge skills as class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sage ~ Dazzling Intellect", at_level: 1, description: Some("A sage's Intelligence score is always equal to 5 + its master's class level, but it gains natural armor increases as if its master's class level were half what of the actual class level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sage ~ Sage's Knowledge", at_level: 1, description: Some("A sage stores information on every topic and is happy to lecture its master on the finer points of a subject. A sage can attempt all Knowledge checks untrained and gains a bonus on Knowledge checks equal to half its master's class level. [SKILL RANK CHANGES NOT IMPLEMENTED] Additionally, a sage gains 2 skill ranks each time its master gains a class level. Its maximum number of ranks in any given skill is equal to its master's class level. This replaces the familiar's ability to share its master's skill ranks."), benefit: None },
            ],
        },
        // Familiar Archetype ~ Soulbound Familiar -- uw_abilities_companion.lst:545
        ArchetypeSwapEntry {
            key: "Familiar Archetype ~ Soulbound Familiar",
            subject: "Familiar",
            archetype_name: "Soulbound Familiar",
            description: Some("Soulbound familiars are born when a master uses the magical principles behind soulbound dolls to bind a soul fragment to an animal. Because it is born of another creature's soul, a soulbound familiar can never serve as a witch's familiar, a shaman's spirit animal, or any other spell-granting familiar."),
            source_page: Some("p.213"),
            prerequisites: Some(&["PREALIGN:LN,NG,CN,NE,TN", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Familiar Archetype ~ Soulbound Familiar],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_FamiliarAlertness,TYPE.CF_FamiliarSpeakwithAnimalsofItsKind,TYPE.CF_FamiliarScryonFamiliar]", "PREMULT:2,[!PREVARGTEQ:mastervar(WitchLVL),1],[!PREVARGTEQ:mastervar(ShamanLVL),1]"]),
            replaces: Some(&["CF_FamiliarAlertness", "CF_FamiliarSpeakwithAnimalsofItsKind", "CF_FamiliarScryonFamiliar"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Soulbound Familiar ~ Soul Focus", at_level: 1, description: Some("The soul fragment bound to the familiar lives within a focus crystal surgically inserted into the animal. As long as this soul focus remains intact, it can be used to bind the soul fragment into another familiar, with the same cost and time requirements as replacing a familiar. Once bound into the soul focus, the soul continues to learn, and so if it is later put into a new familiar body, the soul retains its personality and memories from its previous body or bodies. A soul focus has hardness 8, 12 hit points, and a break DC of 20, though it can be attacked only when surgically removed from the familiar. If the soul focus is destroyed, creating a new soul focus with none of the old one's memories costs as much as replacing a familiar, in addition to the normal cost of replacing the familiar itself. The soulbound familiar gains Skill Focus in a skill important to the creature whose soul provided the fragment."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Soulbound Familiar ~ Alignment Variation", at_level: 1, description: Some("A soulbound familiar's alignment is always at least partially neutral, although it can also be chaotic, evil, good, or lawful, depending on the creature whose soul provided the fragment rather than on the master's alignment (unless the master donates a fragment of her own soul)."), benefit: None },
            ],
        },
        // Familiar Archetype ~ Valet -- uw_abilities_companion.lst:546
        ArchetypeSwapEntry {
            key: "Familiar Archetype ~ Valet",
            subject: "Familiar",
            archetype_name: "Valet",
            description: Some("A valet is a consummate personal servant, able to fetch, deliver, and perform for its master's every need."),
            source_page: Some("p.213"),
            prerequisites: Some(&["PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Familiar Archetype ~ Valet],[!PREABILITY:1,CATEGORY=Archetype,TYPE.CF_FamiliarImprovedEvasion,TYPE.CF_FamiliarAlertness,TYPE.CF_FamiliarDeliverTouchSpells,TYPE.CF_FamiliarSpeakwithAnimalsofItsKind,TYPE.CF_FamiliarScryonfamiliar,TYPE.CF_FamiliarShareSpells]"]),
            replaces: Some(&["CF_FamiliarAlertness", "CF_FamiliarImprovedEvasion", "CF_FamiliarDeliverTouchSpells", "CF_FamiliarSpeakwithAnimalsofItsKind", "CF_FamiliarScryonfamiliar", "CF_FamiliarShareSpells"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Valet ~ Class Skills", at_level: 1, description: Some("A valet treats Craft, Perform, and Profession as class skills."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Valet ~ Able Assistant", at_level: 1, description: Some("A valet's master treats the valet as if it possessed the Cooperative Crafting feat and shared all Craft skills and item creation feats he possesses."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Valet ~ Magical Manipulation", at_level: 1, description: Some("A valet can cast open/close and prestidigitation at will."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Valet ~ Teammate", at_level: 1, description: Some("A valet is considered to have all the teamwork feats its master has."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Valet ~ Deliver Touch Spells", at_level: 1, description: Some("A valet can deliver touch spells for his master. If the master and the valet are in contact at the time the master casts a touch spell, he can designate the valet as the \"toucher.\" The valet can then deliver the touch spell just as the master would. When delivering a harmless touch spell to a willing creature, a valet can move before and after delivering the spell, as long as its total movement does not exceed its speed."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Valet ~ Deliver Aid", at_level: 1, description: Some("A valet can move before and after using the aid another action, as long as its total movement does not exceed its speed."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Valet ~ Aide to All", at_level: 1, description: Some("A valet can choose to use the aid another action as a full-round action, granting up to three adjacent creatures bonuses from this action. Each bonus may be either for the same action or check or for different actions or checks."), benefit: None },
            ],
        },        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_has_30_records() {
        assert_eq!(archetype_swap_tables().len(), 30);
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

    /// Every record's own `subject` is `Companion` or `Familiar`, never
    /// a class name -- the live proof this mechanism's subject-generic
    /// design was the right call from the first table.
    #[test]
    fn every_subject_is_companion_or_familiar_not_a_class() {
        for e in archetype_swap_tables() {
            assert!(
                e.subject == "Companion" || e.subject == "Familiar",
                "{} has subject {:?}, expected Companion or Familiar",
                e.key,
                e.subject
            );
        }
        let companion_count = archetype_swap_tables().iter().filter(|e| e.subject == "Companion").count();
        let familiar_count = archetype_swap_tables().iter().filter(|e| e.subject == "Familiar").count();
        assert_eq!(companion_count, 16, "Companion-subject archetypes");
        assert_eq!(familiar_count, 14, "Familiar-subject archetypes");
    }

    /// UW's own rate: 30% (9/30) -- the seventh and last tier-1 data
    /// point. Seven books, seven distinct values (UPsi 33%, ACG 33%,
    /// APG 52%, UM 27%, UC 22%, ARG 14%, UW 30%); no convergence.
    #[test]
    fn the_type_and_ability_lists_genuinely_disagree() {
        let total_replaces: usize =
            archetype_swap_tables().iter().map(|e| e.replaces.map_or(0, |r| r.len())).sum();
        let total_grants: usize = archetype_swap_tables().iter().map(|e| e.grants.len()).sum();
        assert_eq!(total_replaces, 120, "total TYPE: replaced-slot count across all 30 records");
        assert_eq!(total_grants, 121, "total ABILITY: granted-feature count across all 30 records, after the category ruling");
        assert_ne!(total_replaces, total_grants);

        let equal_count_records = archetype_swap_tables()
            .iter()
            .filter(|e| e.replaces.map_or(0, |r| r.len()) == e.grants.len())
            .count();
        assert_eq!(equal_count_records, 9, "of 30 (30%) -- UW's own rate, the last tier-1 data point");
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
        assert_eq!(resolved, 104, "104 of 121 grants carry real DESC:/BENEFIT: text -- see this module's own doc comment for the 17 that did not");
    }
}
