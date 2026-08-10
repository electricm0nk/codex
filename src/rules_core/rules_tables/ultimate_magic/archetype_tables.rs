//! Ultimate Magic (UM) archetype-swap catalog. SD28-E30
//! (`epic-32-archetype-swap`) tier-1 table 4. See
//! `ultimate_psionics::archetype_tables`'s own module doc comment for
//! the full struct rationale, the exhaustively-enumerated `ABILITY:`
//! grant grammar and its per-family inclusion ruling (`Internal`
//! excluded, `NORMAL`-type excluded), and the `.MOD`-injected-grant
//! floor every table in this program now states explicitly.
//!
//! **Agreement rate, fourth book: 27% (18/67)** -- 233 total `TYPE:`-
//! replaced slots vs 204 total `ABILITY:`-granted features (after the
//! category ruling). Landed alongside UPsi 33%, ACG 33%, APG 52% --
//! book-dependent, as every prior table already established.
//!
//! **All figures in this table are the third and current derivation.**
//! Two earlier passes (a parser gap undercounting grants, then a
//! category-inclusion gap over-counting `Internal`-categorized
//! bookkeeping) were found and corrected on UPsi/ACG/APG before this
//! table was generated -- this table was built with the already-fixed
//! extractor from the start, not corrected after the fact. Full
//! correction history: `decisions.md §51`.
//!
//! **177 of 204 sub-feature grants (87%) resolved to real `DESC:`/
//! `BENEFIT:` text.** The 27 shortfalls cluster into the same causes
//! prior tables already named: 15 shared unresolved names across 3
//! sibling Druid Shaman-totem archetypes (`Saurian Shaman`, `Shark
//! Shaman`, plus a third reference), 3 real cross-book feat references
//! (`Scribe Scroll` x2, `Command Undead` -- the same `FEAT`-category
//! shape as APG's own `Improved Counterspell`), and 9 bare-marker rows
//! with neither token.
//!
//! **This book's own share of the 1,282-row corpus-wide `.MOD`-
//! injection population (`decisions.md §51`'s own addendum) is 129
//! rows, the third-largest of any book.** This table's `grants` field
//! is bounded below by that count and by the tier-2 sub-feature
//! population, not closed by either.
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
//! ultimate_magic/um_abilities_class.lst`), generated programmatically
//! by a one-off extraction script, not hand-transcribed.

use super::super::archetype_swap::{ArchetypeGrant, ArchetypeSwapEntry};

/// Full UM archetype-swap catalog: 67 real, distinct master records, in
/// source order. Built once and cached for the process lifetime.
pub fn archetype_swap_tables() -> &'static [ArchetypeSwapEntry] {
    static TABLE: std::sync::OnceLock<Vec<ArchetypeSwapEntry>> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        vec![
        // Alchemist Archetype ~ Chirurgeon -- um_abilities_class.lst:838
        ArchetypeSwapEntry {
            key: "Alchemist Archetype ~ Chirurgeon",
            subject: "Alchemist",
            archetype_name: "Chirurgeon",
            description: Some("An alchemist who studies anatomy and uses this knowledge to heal is a chirurgeon."),
            source_page: Some("p.18"),
            prerequisites: Some(&["PRECLASS:1,Alchemist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Alchemist Archetype ~ Chirurgeon],[!PREABILITY:1,CATEGORY=Archetype,TYPE.AlchemistPoisonUse,TYPE.AlchemistPoisonResistance4,TYPE.AlchemistPoisonImmunity]"]),
            replaces: Some(&["AlchemistPoisonUse", "AlchemistPoisonResistance4", "AlchemistPoisonImmunity"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Chirurgeon ~ Infused Curative", at_level: 2, description: Some("Your extracts of cure spells automatically act as infusions, and can be used by non-alchemists. When you prepare your extracts, you may choose to render any or all of your infused curatives inert and prepare other extracts to replace them (unlike infusions, which continue to occupy your daily extract slots until consumed or used)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Chirurgeon ~ Anaesthetic", at_level: 5, description: Some("You know how to supplement uses of the Heal skill with pain-killing drugs. You gain Skill Focus (Heal) as a bonus feat. Any use of the Heal skill that has a risk of harming the patient (such as extracting a barb) only deals the minimum damage when performed by you."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Chirurgeon ~ Power Over Death", at_level: 10, description: Some("You add Breath of Life to your formula book as a 4th-level extract. Your infused curative ability applies to this extract."), benefit: None },
            ],
        },
        // Alchemist Archetype ~ Clone Master -- um_abilities_class.lst:840
        ArchetypeSwapEntry {
            key: "Alchemist Archetype ~ Clone Master",
            subject: "Alchemist",
            archetype_name: "Clone Master",
            description: Some("Clone masters practice duplicating existing creatures in order to better understand how to create new life."),
            source_page: Some("p.18"),
            prerequisites: Some(&["PRECLASS:1,Alchemist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Alchemist Archetype ~ Clone Master],[!PREABILITY:1,CATEGORY=Archetype,TYPE.AlchemistPoisonResistance6,TYPE.AlchemistPoisonImmunity,TYPE.AlchemistBombReduction]"]),
            replaces: Some(&["AlchemistPoisonResistance6", "AlchemistPoisonImmunity", "AlchemistBombReduction"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Clone Master ~ Lesser Simulacrum", at_level: 7, description: Some("You add Lesser Simulacrum to your formula book as a 3rd-level extract."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Clone Master ~ Rebirth", at_level: 8, description: Some("You can prepare a clone of yourself that awakens if you are slain. Creating the clone costs 5,000 gp, takes 1 week of work, and requires 3 additional weeks for the clone to grow to maturity. If you die, the clone awakens as if you had used the Clone spell on yourself. You can have one inert of yourself at a time. Unused clones you create do not rot."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Clone Master ~ Simulacrum", at_level: 13, description: Some("You add Simulacrum to your formula book as a 5th-level extract."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Clone Master ~ Clone", at_level: 16, description: Some("You add Clone to your formula book as a 6th-level extract."), benefit: None },
            ],
        },
        // Alchemist Archetype ~ Internal Alchemist -- um_abilities_class.lst:841
        ArchetypeSwapEntry {
            key: "Alchemist Archetype ~ Internal Alchemist",
            subject: "Alchemist",
            archetype_name: "Internal Alchemist",
            description: Some("An internal alchemist studies medicine, diet, and the living body to purify the self in the hope of gaining immortality by means of alchemical concoctions and controlling vital energy. Internal alchemists develop unusual physical abilities from heightened knowledge of how their bodies work."),
            source_page: Some("p.18"),
            prerequisites: Some(&["PRECLASS:1,Alchemist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Alchemist Archetype ~ Internal Alchemist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.AlchemistThrowAnything,TYPE.AlchemistSwiftAlchemy,TYPE.AlchemistSwiftPoisoning]"]),
            replaces: Some(&["AlchemistThrowAnything", "AlchemistSwiftAlchemy", "AlchemistSwiftPoisoning"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Internal Alchemist ~ Breath Mastery", at_level: 1, description: Some("You can control your breath and the flow of vital energy within your body. Without preparation, you can hold your breath for %1 minutes (after this, you must begin making Constitution checks or risk suffocation); by spending a full-round action preparing yourself, you can increase this duration to %1 hours. You can survive twice as long as normal without food or water before you start to take penalties. You can put yourself into a state of suspended animation as a move action, and are then unconscious and appear completely dead; you awaken at a preset time or in response to a condition set by you when you enters this state.|BreathMasteryDuration"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Internal Alchemist ~ Disease Resistance", at_level: 3, description: Some("You gain a +%1 bonus on all saving throws against disease.|AlchemistPoisonResistanceBonus|!PREABILITY:1,CATEGORY=Special Ability,Poison Immunity ~ Alchemist"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Internal Alchemist ~ Uncanny Dodge", at_level: 6, description: None, benefit: None },
            ],
        },
        // Alchemist Archetype ~ Mindchemist -- um_abilities_class.lst:842
        ArchetypeSwapEntry {
            key: "Alchemist Archetype ~ Mindchemist",
            subject: "Alchemist",
            archetype_name: "Mindchemist",
            description: Some("While most alchemists use mutagens to boost their physical ability at the cost of mental ability, some use alchemy for the opposite purpose-to boost the power of the mind and memory. A mindchemist can reach incredible levels of mental acuity, but suffers lingering debilitating effects to his physique."),
            source_page: Some("p.19"),
            prerequisites: Some(&["PRECLASS:1,Alchemist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Alchemist Archetype ~ Mindchemist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.AlchemistMutagen.AlchemistPoisonUse]"]),
            replaces: Some(&["AlchemistMutagen", "AlchemistPoisonUse"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Mindchemist ~ Cognatogen", at_level: 1, description: Some("You learn how to create a cognatogen, as the cognatogen discovery."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mindchemist ~ Perfect Recall", at_level: 2, description: Some("You have honed your memory. When making a Knowledge check, you may add +%1 on the check. You can also use this ability when making an Intelligence check to remember something.|MindchemistPerfectRecallBonus"), benefit: None },
            ],
        },
        // Alchemist Archetype ~ Preservationist -- um_abilities_class.lst:843
        ArchetypeSwapEntry {
            key: "Alchemist Archetype ~ Preservationist",
            subject: "Alchemist",
            archetype_name: "Preservationist",
            description: Some("Some alchemists are obsessed with collecting and preserving exotic creatures. These preservationists may use bottled animals and monsters as teaching tools, but some learn how to reanimate them for short periods to battle on the alchemist's behalf."),
            source_page: Some("p.19"),
            prerequisites: Some(&["PRECLASS:1,Alchemist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Alchemist Archetype ~ Preservationist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.AlchemistPoisonUse,TYPE.AlchemistPoisonResistance4,TYPE.AlchemistPoisonResistance6,TYPE.AlchemistPoisonImmunity,TYPE.AlchemistPersistentMutagen,TYPE.AlchemistDiscoveryLvl18]"]),
            replaces: Some(&["AlchemistPoisonUse", "AlchemistPoisonResistance4", "AlchemistPoisonResistance6", "AlchemistPoisonImmunity", "AlchemistPersistentMutagen", "AlchemistDiscoveryLvl18"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Preservationist ~ Bottled Ally I", at_level: 2, description: Some("You add Handle Animal to your list of class skills. You add Summon Nature's Ally I to your formula book as a 1st-level extract. When you prepare that extract, you actually prepare a tiny, preserved specimen in a bottle (as with a caster casting the spell, you don't have to choose the creature until you use the extract). When you open the bottle, the specimen animates and grows to normal size, serving you as per the spell and otherwise being treated as a summoned creature. When the duration expires, the preserved creature decays into powder. If you have the infusion discovery, another character can use the infused specimen. The Augment Summoning feat can be applied to these specimens."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Preservationist ~ Bottled Ally II", at_level: 5, description: Some("You add Summon Nature's Ally II to your formula book as a 2nd-level extract."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Preservationist ~ Bottled Ally III", at_level: 8, description: Some("You add Summon Nature's Ally IV to your formula book as a 3rd-level extract."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Preservationist ~ Bottled Ally IV", at_level: 10, description: Some("You add Summon Nature's Ally V to your formula book as a 4th-level extract."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Preservationist ~ Bottled Ally V", at_level: 14, description: Some("You add Summon Nature's Ally VII to your formula book as a 5th-level extract."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Preservationist ~ Bottled Ally VI", at_level: 18, description: Some("You add Summon Nature's Ally IX to your formula book as a 6th-level extract."), benefit: None },
            ],
        },
        // Alchemist Archetype ~ Psychonaut -- um_abilities_class.lst:844
        ArchetypeSwapEntry {
            key: "Alchemist Archetype ~ Psychonaut",
            subject: "Alchemist",
            archetype_name: "Psychonaut",
            description: Some("A psychonaut uses his knowledge to explore altered states of consciousness and even other planes of existence."),
            source_page: Some("p.19"),
            prerequisites: Some(&["PRECLASS:1,Alchemist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Alchemist Archetype ~ Psychonaut],[!PREABILITY:1,CATEGORY=Archetype,TYPE.AlchemistPoisonResistance4,TYPE.AlchemistPoisonResistance6,TYPE.AlchemistPoisonImmunity,TYPE.AlchemistBombDamageLvl15,TYPE.AlchemistBombDamageLvl17,TYPE.AlchemistBombReduction]"]),
            replaces: Some(&["AlchemistPoisonResistance4", "AlchemistPoisonResistance6", "AlchemistPoisonImmunity", "AlchemistBombDamageLvl15", "AlchemistBombDamageLvl17", "AlchemistBombReduction"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Psychonaut ~ Precognition", at_level: 5, description: Some("You add Augury to your formula book as a 2nd-level extract (this extract does not require a divine focus component)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Psychonaut ~ Psychic Senses", at_level: 8, description: Some("You add Clairaudience/Clairvoyance, Detect Scrying, Scrying, and Speak With Dead to your formula book as 3rd-level extracts (a Scrying extract does not require a focus or divine focus component)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Psychonaut ~ Remote Consciousness", at_level: 10, description: Some("You add Dream, Lesser Astral Projection, Nightmare, Plane Shift, Sending, and Telepathic Bond to your formula book as 4th-level extracts (a Plane Shift extract does not require a focus component)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Psychonaut ~ Greater Precognition", at_level: 15, description: Some("You add Moment of Prescience to your formula book as a 5th-level extract."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Psychonaut ~ Master Precognition", at_level: 17, description: Some("You add Foresight to your formula book as a 6th-level extract."), benefit: None },
            ],
        },
        // Alchemist Archetype ~ Reanimator -- um_abilities_class.lst:845
        ArchetypeSwapEntry {
            key: "Alchemist Archetype ~ Reanimator",
            subject: "Alchemist",
            archetype_name: "Reanimator",
            description: Some("A reanimator is an alchemist who has discovered how to infuse a corpse with a semblance of life. Many work in tandem with necromancers to explore the fine border between the worlds of the living and the dead."),
            source_page: Some("p.20"),
            prerequisites: Some(&["PRECLASS:1,Alchemist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Alchemist Archetype ~ Reanimator],[!PREABILITY:1,CATEGORY=Archetype,TYPE.AlchemistBombDamageLvl7,TYPE.AlchemistBombDamageLvl13,TYPE.AlchemistBombDamageLvl15,TYPE.AlchemistBombReduction]"]),
            replaces: Some(&["AlchemistBombDamageLvl7", "AlchemistBombDamageLvl13", "AlchemistBombDamageLvl15", "AlchemistBombReduction"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Reanimator ~ Simple Reanimation", at_level: 7, description: Some("You add Lesser Animate Dead to your formula book as a 3rd-level extract.  When you use this extract, rather than drinking it, you inject it into the corpse you intend to animate, which rises as an undead creature under your control 1 hour later.  This extract can only create zombies (including variant zombies)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Reanimator ~ Create Undead", at_level: 13, description: Some("You add Create Undead to your formula book as a 4th-level extract.  When you use this extract, rather than drinking it, you inject it into the corpse you intend to animate, which rises as an uncontrolled undead 1 hour later.  This extract can only create corporeal undead."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Reanimator ~ Create Greater Undead", at_level: 15, description: Some("You add Create Greater Undead to your formula book as a 5th-level extract.  This otherwise acts similarly to a Create Undead extract."), benefit: None },
            ],
        },
        // Alchemist Archetype ~ Vivisectionist -- um_abilities_class.lst:846
        ArchetypeSwapEntry {
            key: "Alchemist Archetype ~ Vivisectionist",
            subject: "Alchemist",
            archetype_name: "Vivisectionist",
            description: Some("A vivisectionist studies bodies to better understand their function. Unlike a chirurgeon, a vivisectionist's goals are not related to healing, but rather to experimentation and knowledge that most people would consider evil."),
            source_page: Some("p.20"),
            prerequisites: Some(&["PRECLASS:1,Alchemist=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Alchemist Archetype ~ Vivisectionist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.AlchemistBomb,TYPE.AlchemistBombDamageLvl1,TYPE.AlchemistBombDamageLvl3,TYPE.AlchemistBombDamageLvl5,TYPE.AlchemistBombDamageLvl7,TYPE.AlchemistBombDamageLvl9,TYPE.AlchemistBombDamageLvl11,TYPE.AlchemistBombDamageLvl13,TYPE.AlchemistBombDamageLvl15,TYPE.AlchemistBombDamageLvl17,TYPE.AlchemistBombDamageLvl9,TYPE.AlchemistBombReduction]"]),
            replaces: Some(&["AlchemistBomb", "AlchemistBombDamageLvl1", "AlchemistBombDamageLvl3", "AlchemistBombDamageLvl5", "AlchemistBombDamageLvl7", "AlchemistBombDamageLvl9", "AlchemistBombDamageLvl11", "AlchemistBombDamageLvl13", "AlchemistBombDamageLvl15", "AlchemistBombDamageLvl17", "AlchemistBombDamageLvl19", "AlchemistBombReduction"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Vivisectionist ~ Sneak Attack", at_level: 1, description: Some("At 1st level, a vivisectionist gains the sneak attack ability as a rogue of the same level. If a character already has sneak attack from another class, the levels from the classes that grant sneak attack stack to determine the effective rogue level for the sneak attack's extra damage dice (so an alchemist 1/rogue 1 has a +1d6 sneak attack like a 2nd-level rogue, an alchemist 2/rogue 1 has a +2d6 sneak attack like a 3rd-level rogue, and so on). This ability replaces bomb."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Vivisectionist ~ Torturer's Eye", at_level: 2, description: Some("You add Deathwatch to your formula book as a 1st-level extract."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Vivisectionist ~ Cruel Anatomist", at_level: 3, description: Some("You may use your Knowledge (Nature) skill bonus in place of your Heal skill bonus."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Vivisectionist ~ Torturous Transformation", at_level: 7, description: Some("You add Anthropomorphic Animal to your formula book as a 2nd-level extract.  When you use this extract, you inject it into an animal as part of a 2-hour surgical procedure.  By using multiple doses of this extract as part of the surgery, you multiply the duration by the number of extracts used."), benefit: None },
            ],
        },
        // Bard Archetype ~ Animal Speaker -- um_abilities_class.lst:932
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Animal Speaker",
            subject: "Bard",
            archetype_name: "Animal Speaker",
            description: Some("An animal speaker focuses not on the ears and minds of humans, but on the creatures of the wild and those in the underbellies of cities."),
            source_page: Some("p.25"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Animal Speaker],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardFascinate,TYPE.BardWellVersed,TYPE.BardInspireCompetence,TYPE.BardSuggestion,TYPE.BardMassSuggestion]"]),
            replaces: Some(&["BardFascinate", "BardWellVersed", "BardInspireCompetence", "BardSuggestion", "BardMassSuggestion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Animal Speaker ~ Animal Friend", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Animal Speaker ~ Nature's Speaker", at_level: 5, description: Some("You can use Speak with Animals at will on %1 selected kinds of animals.|NaturesSpeakerNumber"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Animal Speaker ~ Soothing Performance", at_level: 3, description: Some("You can use bardic performance to influence animals. This works like the druid ability wild empathy, except you expend 1 round of bardic performance and make a Perform check. If you already have wild empathy from another class, you add the class levels that provide wild empathy to the result of your Perform check to influence an animal."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Animal Speaker ~ Attract Rats", at_level: 6, description: Some("You can use bardic performance to summon %1d3 rat swarms; they remain as long as you continue performing.|AttractRatsAmount"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Animal Speaker ~ Summon Nature's Ally", at_level: 1, description: None, benefit: None },
            ],
        },
        // Bard Archetype ~ Celebrity -- um_abilities_class.lst:933
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Celebrity",
            subject: "Bard",
            archetype_name: "Celebrity",
            description: Some("Known for being known, a celebrity bard is a master of  performance  who  captures  the  imagination  and attention of his audience. He trades on his charisma, his wit, and his exploits to build his renown - and that of his companions."),
            source_page: Some("p.25"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Celebrity],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardInspireCourage,TYPE.BardLoreMaster,TYPE.BardDirgeOfDoom]"]),
            replaces: Some(&["BardInspireCourage", "BardLoreMaster", "BardDirgeOfDoom"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Celebrity ~ Famous", at_level: 1, description: Some("You may choose a region where you are famous, and within that region, the locals are more likely to react favorably toward you. You gain a +%1 bonus on Diplomacy and Intimidate checks in that area and to influence people from that area|FamousModifier"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Celebrity ~ Gather Crowd", at_level: 5, description: Some("You are skilled at drawing an audience to your performances. If you are in a settlement or populated area, you can shout, sing, or otherwise make yourself noticed in order to attract an audience to your impromptu stage. The size of the crowd depends on the local population, but typically is a number of people equal to %1 times the result of your Perform check. The crowd gathers over the next 1d10 rounds. If you fail to engage the crowd (such as by performing, kissing babies, trying to use fascinate, and so on), it disperses over the next 1d10 rounds.|classlevel(\"Bard\")/2"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Celebrity ~ Shining Star", at_level: 8, description: Some("You have learned how to focus attention on yourself so thoroughly that even the presence of danger does not distract your adoring crowd. When using fascinate, a target making a save to break the effect because of a potential threat takes a -4 penalty on that save, and even obvious threats require a save rather than automatically breaking the effect. Creatures affected by the bard's fascinate ability ignore the shaken condition."), benefit: None },
            ],
        },
        // Bard Archetype ~ Demagogue -- um_abilities_class.lst:934
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Demagogue",
            subject: "Bard",
            archetype_name: "Demagogue",
            description: Some("Not content with providing amusing and occasionally instructive performances, the demagogue seeks to inflame and ignite his audience, driving them toward a specific purpose with carefully chosen words and tones that may spark momentous change."),
            source_page: Some("p.26"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Demagogue],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardInspireCourage,TYPE.BardInspireCourage1,TYPE.BardLoreMaster,TYPE.BardSuggestion,TYPE.BardMassSuggestion]"]),
            replaces: Some(&["BardInspireCourage1", "BardLoreMaster", "BardSuggestion", "BardMassSuggestion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Demagogue ~ Famous", at_level: 1, description: Some("You may choose a region where you are famous, and within that region, the locals are more likely to react favorably toward you. You gain a +%1 bonus on Bluff and Intimidate checks in that area and to influence people from that area|FamousModifier"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Demagogue ~ Gather Crowd", at_level: 5, description: Some("You are skilled at drawing an audience to your performances. If you are in a settlement or populated area, you can shout, sing, or otherwise make yourself noticed in order to attract an audience to your impromptu stage. The size of the crowd depends on the local population, but typically is a number of people equal to %1 times the result of your Perform check. The crowd gathers over the next 1d10 rounds. If you fail to engage the crowd (such as by performing, kissing babies, trying to use fascinate, and so on), it disperses over the next 1d10 rounds.|classlevel(\"Bard\")/2"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Demagogue ~ Incite Violence", at_level: 6, description: Some("You can use your performance to fan the fury of a crowd of people you has fascinated. Using this ability does not disrupt the fascinate effect, but does require a standard action to activate (in addition to the free action to continue the fascinate effect). You select %1 targets equal to your level, who must make Will saves (DC %2) or be affected by rage for %3 rounds. You indicate who is the intended target of violence (either after using this ability or as part of the performance leading to it) and the enraged members of the crowd immediately attack the target if possible. The target does not need to be present (\"kill the king\" is a suitable choice) and can be an object instead of a person (\"destroy the prison!\" is likewise appropriate). Other members of the crowd may follow suit, though they do not gain the benefits of rage . This is a sound-based effect and is affected by countersong. If two or more bards are attempting to direct the crowd against different targets, they must make opposed Charisma checks, with the crowd following the directions of the winner.|InciteViolenceTargets|InciteViolenceDC|InciteViolenceDuration"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Demagogue ~ Righteous Cause", at_level: 18, description: Some("You can lift a crowd's emotions and turn them toward a common purpose. First, you must fascinate the crowd, and then use incite violence without designating a target, at which point you can use righteous cause. Instead of driving the crowd with anger, you fill them with purpose. Fascinated creatures must make Will saves (DC %1) to resist. Those who fail are affected by mass suggestion of a plausible idea that lingers with them for one day. Typical uses of this ability are to spark rebellion, overthrow a king, build a beneficial structure such as an orphanage, or donate money to a cause.|RighteousCauseDC"), benefit: None },
            ],
        },
        // Bard Archetype ~ Dirge Bard -- um_abilities_class.lst:935
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Dirge Bard",
            subject: "Bard",
            archetype_name: "Dirge Bard",
            description: Some("A composer of sonorous laments for the dead and elaborate requiems for those lost yet long remembered, dirge bards master musical tools and tropes that must appeal to the ears and hearts of both the living and the dead."),
            source_page: Some("p.26"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Dirge Bard],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardJackOfAllTrades,TYPE.BardLoreMaster,TYPE.BardWellVersed,TYPE.BardVersatilePerformance]"]),
            replaces: Some(&["BardJackOfAllTrades", "BardLoreMaster", "BardWellVersed", "BardVersatilePerformance"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Dirge Bard ~ Dance of the Dead", at_level: 10, description: Some("You can use your bardic performance to cause dead bones or bodies to rise up and move or fight at your command. This ability functions like animate dead, but the created skeletons or zombies remain fully animate only as long as you continue the performance. Once it stops, any created undead collapse into carrion. Bodies or bones cannot be animated more than once using this ability. Unlike animate dead, dance of the dead requires no components and does not have the evil descriptor."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dirge Bard ~ Haunted Eyes", at_level: 2, description: Some("You gain a +4 bonus on saves against fear, energy drain, death effects, and necromantic effects."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dirge Bard ~ Secrets of the Grave", at_level: 2, description: Some("You gain a +%1 bonus on Knowledge (religion) checks made to identify undead creatures and their abilities. You may use mind-affecting spells to affect undead as if they were living creatures, even if they are mindless (though spells that affect only humanoids do not affect them, even if they were humanoids in life). In addition, you may add one necromancy spell from the spell list of any arcane spellcasting class to your list of spells known at 2nd level and every four levels thereafter.|SecretsOfTheGraveBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dirge Bard ~ Haunting Refrain", at_level: 5, description: Some("You are able to stir primal terrors in the hearts of listeners. You can use a Perform (keyboard) or Perform (percussion) check in place of an Intimidate check to demoralize an opponent, with a +%1 bonus. In addition, saving throws against any fear effect you create are made with a -%2 penalty.|HauntingRefrainBonus|HauntingRefrainPenalty"), benefit: None },
            ],
        },
        // Bard Archetype ~ Geisha -- um_abilities_class.lst:936
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Geisha",
            subject: "Bard",
            archetype_name: "Geisha",
            description: Some("In some cultures, the professional entertainer is a prestigious role. Specially trained entertainers called geisha are praised for their appearance and skill at conversation, music, dancing, singing, poetry, and calligraphy. A geisha provides social intimacy and status but not physical intimacy."),
            source_page: Some("p.27"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Geisha],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardWeaponProficiencies,TYPE.BardArmorProficiencies,TYPE.BardBardicKnowledge,TYPE.BardArmoredCasting]"]),
            replaces: Some(&["BardWeaponProficiencies", "BardArmorProficiencies", "BardBardicKnowledge", "BardArmoredCasting"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Scribe Scroll", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Geisha ~ Tea Ceremony", at_level: 1, description: Some("By spending 10 minutes preparing an elaborate tea ceremony, you may affect your allies with inspire courage, inspire competence, inspire greatness, or inspire heroics. The ceremony's effects last 10 minutes. You must spend 4 rounds of bardic performance for each creature to be affected."), benefit: None },
            ],
        },
        // Bard Archetype ~ Songhealer -- um_abilities_class.lst:937
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Songhealer",
            subject: "Bard",
            archetype_name: "Songhealer",
            description: Some("Words can harm, but they also heal. The songhealer brings peace and surcease of pain, calming wild emotions and providing a balm for the wounded body."),
            source_page: Some("p.27"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Songhealer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardVersatilePerformance,TYPE.BardFrighteningTune,TYPE.BardDeadlyPerformance]"]),
            replaces: Some(&["BardVersatilePerformance", "BardFrighteningTune", "BardDeadlyPerformance"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Songhealer ~ Enhance Healing", at_level: 1, description: Some("%1 times per day, you can cause any healing effect from a spell completion or spell trigger item to function at caster level %2.|EnhanceHealingTimes|EnhanceHealingCL"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Songhealer ~ Healing Performance", at_level: 14, description: Some("You can use your performance to create an effect equivalent to heal on a living target (or harm on an undead target), using your level as the caster level. Using this ability requires 5 rounds of continuous performance, and the target must be able to see and hear the bard throughout the performance. The healing performance relies on audible and visual components."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Songhealer ~ Funereal Ballad", at_level: 20, description: Some("You can use your performance to create an effect equivalent to resurrection on a dead creature, using your level as the caster level. Using this ability requires 20 rounds of continuous performance, and the target must be within 10 feet of you for the entire performance. Funereal ballad relies on audible and visual components."), benefit: None },
            ],
        },
        // Bard Archetype ~ Sound Striker -- um_abilities_class.lst:938
        ArchetypeSwapEntry {
            key: "Bard Archetype ~ Sound Striker",
            subject: "Bard",
            archetype_name: "Sound Striker",
            description: Some("They say that words can cut deeper than any blade, and the sound striker proves this true. Using music and words as a weapon, he can focus his performances into a deadly delivery."),
            source_page: Some("p.27"),
            prerequisites: Some(&["PRECLASS:1,Bard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Bard Archetype ~ Sound Striker],[!PREABILITY:1,CATEGORY=Archetype,TYPE.BardInspireCompetence,TYPE.BardSuggestion]"]),
            replaces: Some(&["BardInspireCompetence", "BardSuggestion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Sound Striker ~ Wordstrike", at_level: 3, description: Some("You can spend 1 round of bardic performance as a standard action to direct a burst of sonically charged words at a creature or object. This performance deals 1d4+%1 points of damage to an object, or half this damage to a living creature.|WordstrikeBonusDamage"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sound Striker ~ Weird Words", at_level: 6, description: Some("You can start a performance as a standard action, lashing out with %1 potent sounds, each sound affecting one target within 30 feet. These are ranged touch attacks. Each weird word deals 1d8+%2 points of damage (Fortitude DC %3 half), and the bard chooses whether it deals bludgeoning, piercing, or slashing damage for each word.|WeirdWordsAMount|WeirdWordsBonusDamage|WeirdWordsDC"), benefit: None },
            ],
        },
        // Cleric Archetype ~ Cloistered Cleric -- um_abilities_class.lst:1001
        ArchetypeSwapEntry {
            key: "Cleric Archetype ~ Cloistered Cleric",
            subject: "Cleric",
            archetype_name: "Cloistered Cleric",
            description: Some("Cloistered clerics typically live in a temple and rarely interact with the outside world. They are bookish and well learned in the lore of the faith, paying less attention to its magical and martial aspects. A cloistered cleric has the following class features."),
            source_page: Some("p.31"),
            prerequisites: Some(&["PRECLASS:1,Cleric=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Cleric Archetype ~ Cloistered Cleric],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ClericArmorProficiency,TYPE.ClericWeaponProficiency,TYPE.ClericClassSkills,TYPE.ClericSkillRanks,TYPE.ClericSpellcasting,TYPE.ClericDomains]"]),
            replaces: Some(&["ClericArmorProficiency", "ClericWeaponProficiency", "ClericClassSkills", "ClericSkillRanks", "ClericDomains", "ClericSpellcasting"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Scribe Scroll", at_level: 4, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Cloistered Cleric ~ Diminished Spellcasting", at_level: 1, description: Some("You choose only one domain from your deity's list of domains, and your number of non-domain spells per day for each spell level is one less than normal (for example, a 4th-level cloistered cleric has three cantrips, two 1st-level spells, one 1st-level domain spell, one 2nd-level spell, and one 2nd-level domain spell). If this reduces the number of spells per day for that level to 0, you gain only the bonus spells you would be entitled to based on your Wisdom score for that level, plus your domain spell for that level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cloistered Cleric ~ Breadth of Knowledge", at_level: 1, description: Some("You gain a +%1 bonus on Knowledge skill checks and can make Knowledge checks untrained.|BreadthOfKnowledgeBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cloistered Cleric ~ Well-Read", at_level: 2, description: Some("You gain a +2 bonus on skill checks, caster level checks, and saving throws if such rolls pertain to mundane or magical glyphs, runes, scrolls, symbols, and other writings."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Cloistered Cleric ~ Verbal Instruction", at_level: 3, description: Some("You can use the aid another action to assist %1 allies within 30 feet on a skill or ability check. The ally must be able to hear and understand your instructions. If all allies are not engaged in the same task, using this ability is a full-round action rather than a standard action.|VerbalInstructionAllies"), benefit: None },
            ],
        },
        // Cleric Archetype ~ Separatist -- um_abilities_class.lst:1002
        ArchetypeSwapEntry {
            key: "Cleric Archetype ~ Separatist",
            subject: "Cleric",
            archetype_name: "Separatist",
            description: Some("A radical cleric, unsatisfied with the orthodoxy of her deity's teachings, forges her own path of defiant divine expression. Though most members of her faith would call her a separatist or heretic, she continues to receive spells from her deity. Charismatic separatists may develop a large following of like-minded believers and eventually found a splinter church of their deity - and they are just as likely to be the cause of a holy civil war as the branches of the religion fight to determine which is the true faith. A cleric who does not serve a deity cannot take the separatist archetype."),
            source_page: Some("p.32"),
            prerequisites: Some(&["PRECLASS:1,Cleric=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Cleric Archetype ~ Separatist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ClericWeaponProficiency,TYPE.ClericDomains]"]),
            replaces: Some(&["ClericWeaponProficiency", "ClericDomains"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Separatist ~ Forbidden Rites", at_level: 1, description: Some("You select one domain from your deity's domain list, and a second domain that is not on your deity's domain list. This second domain cannot be an alignment domain that doesn't match your or your deity's alignment. For example, a lawful good separatist cleric of a neutral good deity cannot choose the Chaos or Evil domain with this ability, but can select the Lawful domain even though her deity isn't lawful. Granted powers from your second domain function as if your level, Wisdom, and Charisma were 2 lower than normal (minimum level 1) in terms of effect, DC, and uses per day. This also means you don't gain the domain's higher-level ability until 2 levels later than normal. If the second domain grants additional class skills, you gain these as normal."), benefit: None },
            ],
        },
        // Cleric Archetype ~ Theologian -- um_abilities_class.lst:1003
        ArchetypeSwapEntry {
            key: "Cleric Archetype ~ Theologian",
            subject: "Cleric",
            archetype_name: "Theologian",
            description: Some("A theologian is an expert on one particular area of her religion. She is so focused on that area that she eschews the broader sweep of her deity's dogma and focuses intensely upon that aspect of it, embodying its power in all she does. Theologians tend to be more zealous than other clerics, and many crusades are started by theologians."),
            source_page: Some("p.32"),
            prerequisites: Some(&["PRECLASS:1,Cleric=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Cleric Archetype ~ Theologian],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ClericDomains]"]),
            replaces: Some(&["ClericDomains"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Theologian ~ Focused Domain", at_level: 1, description: Some("You choose only one domain from your deity's portfolio rather than the normal two domains. All level-dependent effects of the granted powers from your domain function as if you were two cleric levels higher than your actual cleric level. This does not allow you to gain domain-granted powers earlier than normal. You can prepare domain spells using your non-domain slots. You cannot use her spontaneous casting ability on domain spells, even if they are prepared in non-domain slots. In all other respects, this works like and replaces the standard cleric domain ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Theologian ~ Domain Secret", at_level: 5, description: Some("You choose %1 domain spells. Those spells becomes permanently modified with one of the following metamagic feats: Bouncing Spell, Disruptive Spell, Ectoplasmic Spell, Enlarge Spell, Extend Spell, Focused Spell, Intensified Spell, Silent Spell, Still Spell. This metamagic feat does not increase the level of the spell. Once chosen, this modification cannot be changed. You need not have the metamagic feat to apply it to a spell using this ability. You cannot modify the same spell more than once.|DomainSecrets"), benefit: None },
            ],
        },
        // Cleric Archetype ~ Undead Lord -- um_abilities_class.lst:1005
        ArchetypeSwapEntry {
            key: "Cleric Archetype ~ Undead Lord",
            subject: "Cleric",
            archetype_name: "Undead Lord",
            description: Some("An undead lord is a cleric focused on using necromancy to control undead. Her flock is the walking dead and her choir the keening spirits of the damned. This unliving congregation is the manifestation of her unceasing love affair with death."),
            source_page: Some("p.32"),
            prerequisites: Some(&["PRECLASS:1,Cleric=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Cleric Archetype ~ Undead Lord],[!PREABILITY:1,CATEGORY=Archetype,TYPE.ClericDomains]"]),
            replaces: Some(&["ClericDomains"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Command Undead", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Undead Lord ~ Corpse Companion", at_level: 1, description: Some("With a ritual requiring 8 hours, you can animate a single skeleton or zombie of %1 HD or less. This corpse companion automatically follows your commands and does not need to be controlled by you. You cannot have more than one corpse companion at a time. It does not count against the number of HD of undead controlled by other methods. You can use this ability to create a variant skeleton such as a bloody or burning skeleton of %2 HD or less. You can dismiss your companion as a standard action, which destroys it.|CorpseCompanionHD|CorpseCompanionHD/2"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Undead Lord ~ Unlife Healer", at_level: 8, description: Some("Your spells, spell-like abilities, and supernatural abilities used to heal undead are Empowered.|!PRECLASS:1,Cleric=16"), benefit: None },
            ],
        },
        // Druid Archetype ~ Dragon Shaman -- um_abilities_class.lst:1088
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Dragon Shaman",
            subject: "Druid",
            archetype_name: "Dragon Shaman",
            description: Some("Your totem is the legendary dragon, fearsome and deadly yet cunning and wise, a creature born of pure magic and raw elemental fury, bound within a shell of fangs, claws, and scales that few dare to challenge. Though your initial focus is on dragons' mundane cousins, as your powers grow you become attuned to actual dragons."),
            source_page: Some("p.37"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Dragon Shaman],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidThousandFaces,TYPE.DruidWildShape,TYPE.DruidWildShape8,TYPE.DruidVenomImmunity]"]),
            replaces: Some(&["DruidThousandFaces", "DruidWildShape", "DruidWildShape8", "DruidVenomImmunity"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Dragon Shaman ~ Nature's Bond", at_level: 1, description: Some("If you choose an animal companion, you must select a crocodile (see page 54 of the Core Rulebook) or monitor lizard (see page 194 of the Bestiary). If you choose a domain, you must choose from the Air, Animal, Destruction, Earth, Fire, War, and Water domains."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dragon Shaman ~ Wild Empathy", at_level: 1, description: Some("You can use your wild empathy ability with lizards as a full-round action with a +4 bonus."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dragon Shaman ~ Totem Transformation", at_level: 2, description: Some("You may adopt an aspect of the dragon while retaining your normal form. You gain one of the following bonuses: movement (fly speed 30 feet [average], you must be 5th level to select this bonus), senses (low-light vision, +4 racial bonus to Perception), toughness (+2 natural armor bonus to AC, Endurance feat), or natural weapons (bite [1d6] and 2 claws [1d4] for a Medium shaman, +2 to CMB on grapple checks). While using totem transformation, you may speak normally and can cast speak with animals (lizards only) at will. Using this ability is a standard action at 2nd level, a move action at 7th level, and a swift action at 12th level. You can use this ability for %1 minutes per day. These minutes do not need to be consecutive, but they must be used in 1-minute increments. This is a polymorph effect and cannot be used while you are using another polymorph effect, such as wild shape.|TotemTransformationDuration"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dragon Shaman ~ Totemic Summons", at_level: 5, description: Some("You may cast summon nature's ally as a standard action when summoning lizards, and summoned lizards gain %1 temporary hit points. You can apply the young template to any lizard to reduce the level of the summoning spell required by one. You can also increase the level of summoning required by one in order to apply either the advanced or the giant template, or increase it by two to apply both the advanced and giant templates.|TotemicSummonsTempHP"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dragon Shaman ~ Wild Shape", at_level: 6, description: Some("Your wild shape ability functions at your druid level - 4. If you take on the form of a lizard, you instead use your unmodified druid level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dragon Shaman ~ Dragon Bite", at_level: 8, description: Some("Your bite attack (whether using totem transformation or wild shape to take the form of a lizard) deals +1d6 points of energy damage (acid, cold, electricity, or fire). You choose what kind of energy damage that you deal each time you bite."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Druid ~ Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Progression", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Times", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Wild Shape", at_level: 6, description: None, benefit: None },
            ],
        },
        // Druid Archetype ~ Menhir Savant -- um_abilities_class.lst:1089
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Menhir Savant",
            subject: "Druid",
            archetype_name: "Menhir Savant",
            description: Some("Some druids study the paths of nature's power through the nodes and ley lines that connect standing stones and megalithic circles, learning to tap into their energies."),
            source_page: Some("p.38"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Menhir Savant],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidNatureSense,TYPE.DruidWildEmpathy,TYPE.DruidWoodlandStride,TYPE.DruidTracklessStep,TYPE.DruidThousandFaces]"]),
            replaces: Some(&["DruidNatureSense", "DruidWildEmpathy", "DruidWoodlandStride", "DruidTracklessStep", "DruidThousandFaces"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Menhir Savant ~ Spirit Sense", at_level: 1, description: Some("You can detect the presence of undead; fey; outsiders; and astral, ethereal, or incorporeal creatures. This ability functions like detect undead, and you detect all of these creatures rather than trying to detect one kind."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Menhir Savant ~ Place Magic", at_level: 2, description: Some("You can identify and tap into ley lines in different types of terrain. As a free action, you can tap into the magic of a nearby ley line and increase your caster level by +1 for 1 round. You can use this ability %1 times per day.|PlaceMagicTimes"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Menhir Savant ~ Walk the Lines", at_level: 9, description: Some("You can use your connection to ley lines to cast transport via plants %1 times per day.|WalkTheLinesTimes"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Menhir Savant ~ Empty Body", at_level: 13, description: Some("You can become ethereal as a standard action, as if using ethereal jaunt. You can remain ethereal for %1 rounds per day. These rounds do not need to be consecutive.|EmptyBodyRounds"), benefit: None },
            ],
        },
        // Druid Archetype ~ Mooncaller -- um_abilities_class.lst:1090
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Mooncaller",
            subject: "Druid",
            archetype_name: "Mooncaller",
            description: Some("A mooncaller is bound to the subtle influences of the ever-changing moon and its endless cycles from light to dark and back again."),
            source_page: Some("p.38"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Mooncaller],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidWoodlandStride,TYPE.DruidResistNaturesLure,TYPE.DruidVenomImmunity,TYPE.DruidThousandFaces]"]),
            replaces: Some(&["DruidWoodlandStride", "DruidResistNaturesLure", "DruidVenomImmunity", "DruidThousandFaces"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Mooncaller ~ Resist Call of the Wild", at_level: 4, description: Some("You gain a +4 bonus on saving throws to avoid confusion, daze, feeblemind, and insanity effects. You also gain a +4 bonus against the exceptional, spell-like, and supernatural abilities of creatures with the shapechanger subtype."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mooncaller ~ Purity of Body", at_level: 9, description: Some("You gain immunity to all diseases, including supernatural and magical diseases."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Mooncaller ~ Wolfsbane", at_level: 13, description: Some("You gain DR %1/silver.|MooncallerWolfsbaneDR"), benefit: None },
            ],
        },
        // Druid Archetype ~ Pack Lord -- um_abilities_class.lst:1091
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Pack Lord",
            subject: "Druid",
            archetype_name: "Pack Lord",
            description: Some("Some druids bond with many animal companions rather than just one, achieving a level of communion rare even in druidic circles and leading their pack brothers and pack sisters with total authority."),
            source_page: Some("p.38"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Pack Lord],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidNatureBond,TYPE.DruidWildshape6]"]),
            replaces: Some(&["DruidNatureBond", "DruidWildShape6"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Pack Lord ~ Pack Bond", at_level: 1, description: Some("You may not select a domain and must choose an animal companion. You gain a +2 bonus on wild empathy and Handle Animal checks made regarding your animal companion. You may have more than one animal companion, but you must divide up your effective druid level between your companions to determine the abilities of each companion. Each time your druid level increases, you must decide how to allocate the increase among your animal companions (including the option of adding a new 1st-level companion). Once a druid level is allocated to a particular companion, it cannot be redistributed while that companion is in your service (you must release the companion or wait until the companion dies to allocate its levels to another companion, which you can do the next time you prepare spells). The share spells animal companion ability only applies to one animal companion at a time - you cannot use it to cast a one-target spell and have it affect all of your animal companions."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Pack Lord ~ Improved Empathic Link", at_level: 6, description: Some("You gain an empathic link with all of your animal companions. This functions like an empathic link with a familiar. In addition, as a swift action you can shift your perception to one of your companions, allowing you to experience what it sees, hears, and so on. You can maintain this connection as long as you like (as long as the companion is within 1 mile) and end it as a free action. You can only use this ability on one companion at a time, and cannot see, hear, or smell with your own body while maintaining this connection."), benefit: None },
            ],
        },
        // Druid Archetype ~ Reincarnated Druid -- um_abilities_class.lst:1092
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Reincarnated Druid",
            subject: "Druid",
            archetype_name: "Reincarnated Druid",
            description: Some("Spun off into the endless circle of life, an incarnate druid is an embodiment of nature's eternal renewal. She lives many lives and wanders the world devoid of attachments, a stranger to all yet one with all life."),
            source_page: Some("p.39"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Reincarnated Druid],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidWoodlandStride,TYPE.DruidResistNaturesLure,TYPE.DruidVenomImmunity.TYPE.DruidTimelessBody]"]),
            replaces: Some(&["DruidWoodlandStride", "DruidResistNaturesLure", "DruidVenomImmunity", "DruidTimelessBody"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Reincarnated Druid ~ Mysterious Stranger", at_level: 2, description: Some("You add %1 to the DC of Sense Motive, Diplomacy, and Knowledge checks to learn about you.|MysteriousStrangerBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Reincarnated Druid ~ Resist Death's Touch", at_level: 4, description: Some("You gain a +4 bonus on saving throws against death effects, energy drain, and necromancy effects, and on stabilization checks when dying."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Reincarnated Druid ~ Many Lives", at_level: 5, description: Some("If you are killed, you may automatically reincarnate (as the spell) 1 day later.  You appear in a safe location within 1 mile of your previous body. At will for the next 7 days, you can sense the presence of your remains as if using locate object as a spell-like ability. If you are killed during these 7 days, you remain dead and do not reincarnate. The many lives ability does not function you are slain by a death effect.  You cannot be raised from the dead orresurrected, though you can be reincarnated."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Reincarnated Druid ~ Cheat Death", at_level: 9, description: Some("Once per day you may reroll a save against a death effect, energy drain, or necromancy effect before the result of the roll is revealed, or reroll a failed stabilization check while dying.  You must taket the result of the second roll, even if it is worse than the original roll."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Reincarnated Druid ~ Tongue of the Sun and Moon", at_level: 15, description: Some("You can speak with any living creature."), benefit: None },
            ],
        },
        // Druid Archetype ~ Saurian Shaman -- um_abilities_class.lst:1093
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Saurian Shaman",
            subject: "Druid",
            archetype_name: "Saurian Shaman",
            description: Some("A shaman with this focus calls upon the primeval dinosaur, the archaic terror that lingers as a hungering, atavistic stranger at the fringes of the ecosystem, a destroyer and despoiler whose coming other animals dread."),
            source_page: Some("p.39"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Saurian Shaman],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidThousandFaces,TYPE.DruidWildShape,TYPE.DruidVenomImmunity,TYPE.DruidNatureBond]"]),
            replaces: Some(&["DruidThousandFaces", "DruidWildShape", "DruidVenomImmunity"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Druid ~ Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Progression", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Times", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Saurian Shaman ~ Wild Empathy", at_level: 1, description: Some("You can use your wild empathy ability with dinosaurs and reptiles as a full-round action with a +4 bonus."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Saurian Shaman ~ Totem Transformation", at_level: 2, description: Some("You may adopt an aspect of the saurian while retaining your normal form. You gain one of the following bonuses: movement (+10 enhancement bonus to land speed), scales (+2 natural armor bonus to AC), senses (low-light vision, scent), or natural weapons (bite [1d6], 2 claws [1d4] for a Medium druid, rake, +2 CMB to grapple). While using totem transformation, you may speak normally and can cast speak with animals (reptiles and dinosaurs only) at will. Using this ability is a standard action at 2nd level, a move action at 7th level, and a swift action at 12th level. You can use this ability for %1 minutes per day. These minutes do not need to be consecutive, but they must be used in 1-minute increments. This is a polymorph effect and cannot be used while you are using another polymorph effect, such as wild shape.|TotemTransformationDuration"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Saurian Shaman ~ Totemic Summons", at_level: 5, description: Some("You may cast summon nature's ally as a standard action when summoning reptiles and dinosaurs, and those summoned creatures gain %1 temporary hit points. You can apply the young template to any reptile or dinosaur to reduce the level of the summoning spell required by one. You can also increase the level of summoning required by one in order to apply either the advanced or the giant template, or increase it by two to apply both the advanced and giant templates.|TotemicSummonsTempHP"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Saurian Shaman ~ Wild Shape", at_level: 6, description: Some("Your wild shape ability functions at your druid level - 2. If you take on the form of a reptile or dinosaur, you instead use your druid level +2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Saurian Shaman ~ Nature Bond", at_level: 1, description: Some("If you choose an animal companion, you must select a dinosaur. If you choose a domain, you must choose from the Animal, Destruction, Strength, and War domains."), benefit: None },
            ],
        },
        // Druid Archetype ~ Shark Shaman -- um_abilities_class.lst:1094
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Shark Shaman",
            subject: "Druid",
            archetype_name: "Shark Shaman",
            description: Some("Some druids emulate the deadly shark, a remorseless hunter that marine dwellers dread. Like a true shark, a shark shaman leaves blood and fear in her wake."),
            source_page: Some("p.40"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Shark Shaman],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidThousandFaces,TYPE.DruidWildShape,TYPE.DruidVenomImmunity,TYPE.DruidNatureBond]"]),
            replaces: Some(&["DruidThousandFaces", "DruidWildShape", "DruidVenomImmunity"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Druid ~ Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Progression", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Druid Wild Shape Times", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shaman Wild Shape", at_level: 6, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Shark Shaman ~ Wild Empathy", at_level: 1, description: Some("You can use your wild empathy ability with fish as a full-round action with a +4 bonus."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shark Shaman ~ Totem Transformation", at_level: 2, description: Some("You may adopt an aspect of the shark while retaining your normal form. You gain one of the following bonuses: movement (can breathe water, swim speed 30 feet), senses (scent 30 feet, scent 90 feet in water), natural weapons (bite [1d6 for a Medium shaman), or shark skin (+2 natural armor, creatures grappling the shaman take 1 point of slashing damage per round of grapple). While using totem transformation, you may speak normally and can cast speak with animals (fish only) at will. Using this ability is a standard action at 2nd level, a move action at 7th level, and a swift action at 12th level. You can use this ability for %1 minutes per day. These minutes do not need to be consecutive, but they must be used in 1-minute increments. This is a polymorph effect and cannot be used while you are using another polymorph effect, such as wild shape.|TotemTransformationDuration"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shark Shaman ~ Totemic Summons", at_level: 5, description: Some("You may use summon nature's ally I to summon a stingray (Bestiary 2) and summon nature's ally II to summon a manta ray (Bestiary 2).  You may cast summon nature's ally as a standard action when summoning rays and sharks, and summoned rays and sharks gain %1 temporary hit points. This ability otherwise functions as the dragon shaman's totemic summons ability.|TotemicSummonsTempHP"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shark Shaman ~ Wild Shape", at_level: 6, description: Some("Your wild shape ability functions at your druid level - 2. If you take on the form of a shark, you instead use your druid level +2."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Shark Shaman ~ Nature Bond", at_level: 1, description: Some("If you choose an animal companion, you must select a shark. If you choose a domain, you must choose from the Animal, Death, War, and Water domains."), benefit: None },
            ],
        },
        // Druid Archetype ~ Storm Druid -- um_abilities_class.lst:1095
        ArchetypeSwapEntry {
            key: "Druid Archetype ~ Storm Druid",
            subject: "Druid",
            archetype_name: "Storm Druid",
            description: Some("While most druids focus their attention upon the rich earth and the bounty of nature that springs forth from it, the storm druid's eyes have ever been cast to the skies and the endless expanse of blue, channeling the most raw and untamed aspects of nature."),
            source_page: Some("p.40"),
            prerequisites: Some(&["PRECLASS:1,Druid=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Druid Archetype ~ Storm Druid],[!PREABILITY:1,CATEGORY=Archetype,TYPE.DruidSpontaneousCasting,TYPE.DruidWoodlandStride,TYPE.DruidTracklessStep,TYPE.DruidResistNaturesLure.TYPE.DruidThousandFaces,TYPE.DruidNatureBond]"]),
            replaces: Some(&["DruidSpontaneousCasting", "DruidWoodlandStride", "DruidTracklessStep", "DruidResistNaturesLure", "DruidVenomImmunity", "DruidThousandFaces"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Storm Druid ~ Spontaneous Domain Casting", at_level: 1, description: Some("You can channel stored spell energy into domain spells that you have not prepared ahead of time. You can \"lose\" a prepared spell in order to cast any domain spell of the same level or lower."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Storm Druid ~ Nature's Bond", at_level: 1, description: Some("You may not choose an animal companion. You must choose the Air or Weather domain, or the Cloud, Storm, or Wind subdomain."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Storm Druid ~ Windwalker", at_level: 2, description: Some("The penalties from natural or magical wind effects (see page 439 of the Core Rulebook) are treated as one step less severe for you."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Storm Druid ~ Stormvoice", at_level: 3, description: Some("Your voice can magically carry over howling winds and peals of thunder. Whenever a Perception check is needed to hear your voice, the DC is reduced by %1.|StormVoiceReduction"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Storm Druid ~ Eyes of the Storm", at_level: 4, description: Some("You can see through %1 feet of magical fog, mist, gas, wind, rain, or similar inclement weather conditions, ignoring any concealment it might grant.|EyesOfTheStormRange"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Storm Druid ~ Windlord", at_level: 9, description: Some("You can select another domain or subdomain from those available to you through your nature bond."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Storm Druid ~ Storm Lord", at_level: 13, description: Some("You are unaffected by natural and magical wind effects. You also become immune to deafness and gain +2 bonus on saving throws against sonic effects."), benefit: None },
            ],
        },
        // Inquisitor Archetype ~ Exorcist -- um_abilities_class.lst:1247
        ArchetypeSwapEntry {
            key: "Inquisitor Archetype ~ Exorcist",
            subject: "Inquisitor",
            archetype_name: "Exorcist",
            description: Some("Some inquisitors, as they learn more about the threat of possession and the machinations of the planes, task themselves to expel possessing spirits and conniving outsiders from the world whenever possible. Eventually they learn the secret of the verdicts of exorcism, exile, and anathema."),
            source_page: Some("p.44"),
            prerequisites: Some(&["PRECLASS:1,Inquisitor=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Inquisitor Archetype ~ Exorcist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.InquisitorSecondJudgment,TYPE.InquisitorThirdJudgment,TYPE.InquisitorSlayer,TYPE.InquisitorTrueJudgment]"]),
            replaces: Some(&["InquisitorSecondJudgment", "InquisitorThirdJudgment", "InquisitorSlayer", "InquisitorTrueJudgment"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Exorcist ~ Verdict of Exorcism", at_level: 8, description: Some("While using judgment, you can unleash the verdict of exorcism on a creature. When you do, your judgment ends, but the creature is dazed for 1 round (Will DC %1 negates); if the creature is possessed, the possessing entity must succeed at a Will saving throw (DC %1), or be exorcised and never again allowed in that same body.|VerdictOfExorcismDC"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Exorcist ~ Verdict of Exile", at_level: 16, description: Some("While using judgment, you can unleash the verdict of exile on a creature. When you do, your judgment ends, but the creature subject to that judgment is dazed for 1 round (Will DC %1 negates); if the creature is possessed or an outsider, the possessing entity or outsider is affected by dismissal (Will negates).|VerdictOfExileDC"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Exorcist ~ Closed Mind", at_level: 17, description: Some("You are immune to compulsion effects and possession attempts (including magic jar)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Exorcist ~ Verdict of Anathema", at_level: 20, description: Some("While using judgment, you can unleash the verdict of anathema on your enemies. When you do, your judgment ends, and all enemy creatures within 10 feet are affected by your verdict of exorcism."), benefit: None },
            ],
        },
        // Inquisitor Archetype ~ Heretic -- um_abilities_class.lst:1248
        ArchetypeSwapEntry {
            key: "Inquisitor Archetype ~ Heretic",
            subject: "Inquisitor",
            archetype_name: "Heretic",
            description: Some("While all inquisitors hunt the enemies of the faith, sometimes, either through political maneuvering by her enemies or an unyielding tenacity that breaks her faith's basic tenets, an inquisitor can find herself a heretic. Still unyielding in her cause, these heretics are accustomed to using guile and deception to hide themselves and their activities while they continue to hunt their enemies."),
            source_page: Some("p.45"),
            prerequisites: Some(&["PRECLASS:1,Inquisitor=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Inquisitor Archetype ~ Heretic],[!PREABILITY:1,CATEGORY=Archetype,TYPE.InquisitorMonsterLore]"]),
            replaces: Some(&["InquisitorMonsterLore"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Judgment ~ Escape", at_level: 1, description: Some("Each time you hit an opponent with a melee or ranged attack while using this judgment, you can use a move action attempt to create a diversion to hide (see the Stealth skill)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Heretic ~ Lore of Escape", at_level: 1, description: Some("You use every trick you know to escape those now pusuing you.  You add your Wisdom modifier on Bluff and Stealth skill checks in addition to the normal ability score modifiers."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Heretic ~ Hide Tracks", at_level: 1, description: Some("You are adept at hiding your tracks.  Creatures attempting to track you take a -5 penalty on rolls to find or follow your tracks."), benefit: None },
            ],
        },
        // Inquisitor Archetype ~ Infiltrator -- um_abilities_class.lst:1249
        ArchetypeSwapEntry {
            key: "Inquisitor Archetype ~ Infiltrator",
            subject: "Inquisitor",
            archetype_name: "Infiltrator",
            description: Some("This inquisitor uses guile and deception to blend in among the enemies of the faith rather than confronting them head-on."),
            source_page: Some("p.45"),
            prerequisites: Some(&["PRECLASS:1,Inquisitor=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Inquisitor Archetype ~ Infiltrator],[!PREABILITY:1,CATEGORY=Archetype,TYPE.InquisitorSternGaze,TYPE.InquisitorMonsterLore,TYPE.InquisitorTrack,TYPE.InquisitorDiscernLies]"]),
            replaces: Some(&["InquisitorSternGaze", "InquisitorMonsterLore", "InquisitorTrack", "InquisitorDiscernLies"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Infiltrator ~ Misdirection", at_level: 1, description: Some("Each day when you prepares spells, you may choose an alignment. You detect as that alignment as if you had used misdirection on a creature with that alignment (this does not change any divination results about her other than her alignment)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Infiltrator ~ Guileful Lore", at_level: 1, description: Some("Your will is bent toward subterfuge and deception.  You add +%1 on Bluff and Diplomacy skill checks in addition to the normal ability score modifiers.|WIS"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Infiltrator ~ Forbidden Lore", at_level: 1, description: Some("While other inquisitors learned to track unbelievers, you learn how to cast their spells. You can cast spells of an alignment opposed to you or your deity (ignoring the restriction in the Chaotic, Evil, Good, and Lawful Spells class ability)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Infiltrator ~ Necessary Lies", at_level: 1, description: Some("You add +%1 on saving throws against abilities that detect lies or reveal or force the truth, such as detect lies and zone of truth.|NecessaryLiesBonus"), benefit: None },
            ],
        },
        // Inquisitor Archetype ~ Preacher -- um_abilities_class.lst:1250
        ArchetypeSwapEntry {
            key: "Inquisitor Archetype ~ Preacher",
            subject: "Inquisitor",
            archetype_name: "Preacher",
            description: Some("Some inquisitors wander the land to spread the true word of their faith. Often they come into conflict with those hostile to their teachings or to the preacher's need to help those who cannot help themselves. The leaders of evil or aggressive religions send these preachers into new territories to win converts and hopefully allies. Often, they start uprisings against powers hostile to their religion, or defend a group of honest believers from the depredations of the unfaithful."),
            source_page: Some("p.46"),
            prerequisites: Some(&["PRECLASS:1,Inquisitor=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Inquisitor Archetype ~ Preacher],[!PREABILITY:1,CATEGORY=Archetype,TYPE.InquisitorSoloTactics]"]),
            replaces: Some(&["InquisitorSoloTactics"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Preacher ~ Determination", at_level: 3, description: Some("You are a person of few words on the battlefield, but those words hold great power and authority. You can use this ability to create one of the following effects %1/day. Each is a free action to use. Aggression: You may reroll an attack roll that you just made before the results of the roll are revealed. You must take the result of the reroll, even if it's worse than the original roll.  Defense: When you would be hit by a melee or ranged attack, as an immediate action you may add a +4 insight bonus to your Armor Class against that attack, and if this makes your AC higher than the opponent's attack roll, the attack misses.  Warning: When your ally within line of sight would be hit by a melee or ranged attack, you may call out a warning to that ally, and the attacker must reroll the attack and use the results of the second roll. The ally must be able to hear you and must not be helpless for this ability to have any effect.  Whenever you could select a bonus teamwork feat (at 3rd, 6th, 9th, 12th, 15th, and 18th level), you can instead choose to increase your number of uses per day of this ability by one.|DeterminationTimes"), benefit: None },
            ],
        },
        // Inquisitor Archetype ~ Sin Eater -- um_abilities_class.lst:1251
        ArchetypeSwapEntry {
            key: "Inquisitor Archetype ~ Sin Eater",
            subject: "Inquisitor",
            archetype_name: "Sin Eater",
            description: Some("There is a sect of inquisitors in some religions that believes it is not enough to hunt the enemies of the church - one must also devour those enemies' sins. More benign versions of the practice believe that sin, or evil, is taken out of the world when a sin is devoured, denying the enemy's soul to the enemy's god and purifying the world of its taint. Followers of malevolent churches believe that consuming the sins of good folk not only corrupts the enemy soul to keep it from the celestial planes, but also taints the souls of those who witness the sin-eating or the corpse of its victim. Consuming sins empowers the sin eater, at least for a time."),
            source_page: Some("p.46"),
            prerequisites: Some(&["PRECLASS:1,Inquisitor=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Inquisitor Archetype ~ Sin Eater],[!PREABILITY:1,CATEGORY=Archetype,TYPE.InquisitorDomain,TYPE.InquisitorTeamworkFeat6,TYPE.InquisitorExploitWeakness]"]),
            replaces: Some(&["InquisitorDomain", "InquisitorTeamworkFeat6", "InquisitorExploitWeakness"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Sin Eater ~ Eat Sin", at_level: 1, description: Some("As a free action, when you kill an enemy, you may eat the sins of that enemy by spending 1 minute adjacent to its corpse. This provokes attacks of opportunity. You can rush this ritual, performing it as a full-round action that provokes attacks of opportunity, but you only gain half the normal benefit (see below). Eating the enemy's sins heals you %1d8+%2 hit points of damage. The enemy must have been killed by you within the last hour, and it must have had at least %3 Hit Dice. You can use this ability once for each enemy you kill. This ability has no effect on mindless creatures or those with Intelligence 2 or less.  In some faiths, this \"eating\" is a purely symbolic act, while in others, the inquisitor must eat a small amount of food and water as part of the ritual. A few extreme faiths actually require the inquisitor to eat some of the body of the slain enemy.|EatSinDice|EatSinBonus|EatSinMinHD"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sin Eater ~ Speak with Dead", at_level: 6, description: Some("When you eat an enemy's sins, within 10 minutes of doing so, you can ask the remnants of the enemy's soul questions as if using speak with dead, with a caster level of %1.  You do not need the enemy's corpse to use this ability (you can eat sin, move away from the corpse, then use speak with dead), though the soul gets a saving throw just as a corpse would."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sin Eater ~ Burden of Sin", at_level: 14, description: Some("You may spend a full-round action to transfer one harmful affliction, condition, or spell effect from another creature to yourself (this includes curses, possessions, and permanent effects such as petrification, or any condition that break enchantment can end or reverse). The effect to be transferred is chosen by you and affects you as if you were the original target, continuing its duration (if any) and preventing any further effect on the original bearer. For example, you could transfer a lethal disease to yourself, or petrify yourself to restore a petrified comrade. You can use this ability as often as desired, even using it multiple times on the same creature."), benefit: None },
            ],
        },
        // Monk Archetype ~ Qinggong Monk Abundant Step -- um_abilities_class.lst:1342
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Qinggong Monk Abundant Step",
            subject: "Monk",
            archetype_name: "Qinggong Monk Abundant Step",
            description: Some("Swap Abundant Step for a ki power."),
            source_page: Some("p.51"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Qinggong Monk Abundant Step],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkAbundantStep]"]),
            replaces: Some(&["MonkAbundantStep"]),
            grants: &[],
        },
        // Monk Archetype ~ Qinggong Monk Diamond Body -- um_abilities_class.lst:1341
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Qinggong Monk Diamond Body",
            subject: "Monk",
            archetype_name: "Qinggong Monk Diamond Body",
            description: Some("Swap Diamond Body for a ki power."),
            source_page: Some("p.51"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Qinggong Monk Diamond Body],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkDiamondBody]"]),
            replaces: Some(&["MonkDiamondBody"]),
            grants: &[],
        },
        // Monk Archetype ~ Qinggong Monk Diamond Soul -- um_abilities_class.lst:1343
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Qinggong Monk Diamond Soul",
            subject: "Monk",
            archetype_name: "Qinggong Monk Diamond Soul",
            description: Some("Swap Diamond Soul for a ki power."),
            source_page: Some("p.51"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Qinggong Monk Diamond Soul],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkDiamondSoul]"]),
            replaces: Some(&["MonkDiamondSoul"]),
            grants: &[],
        },
        // Monk Archetype ~ Qinggong Monk Empty Body -- um_abilities_class.lst:1347
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Qinggong Monk Empty Body",
            subject: "Monk",
            archetype_name: "Qinggong Monk Empty Body",
            description: Some("Swap Empty Body for a ki power."),
            source_page: Some("p.51"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Qinggong Monk Empty Body],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkEmptyBody]"]),
            replaces: Some(&["MonkEmptyBody"]),
            grants: &[],
        },
        // Monk Archetype ~ Qinggong Monk High Jump -- um_abilities_class.lst:1339
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Qinggong Monk High Jump",
            subject: "Monk",
            archetype_name: "Qinggong Monk High Jump",
            description: Some("Swap High Jump for a ki power."),
            source_page: Some("p.51"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Qinggong Monk High Jump],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkHighJump]"]),
            replaces: Some(&["MonkHighJump"]),
            grants: &[],
        },
        // Monk Archetype ~ Qinggong Monk Perfect Self -- um_abilities_class.lst:1348
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Qinggong Monk Perfect Self",
            subject: "Monk",
            archetype_name: "Qinggong Monk Perfect Self",
            description: Some("Swap Perfect Self for a ki power."),
            source_page: Some("p.51"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Qinggong Monk Perfect Self],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkPerfectSelf]"]),
            replaces: Some(&["MonkPerfectSelf"]),
            grants: &[],
        },
        // Monk Archetype ~ Qinggong Monk Quivering Palm -- um_abilities_class.lst:1344
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Qinggong Monk Quivering Palm",
            subject: "Monk",
            archetype_name: "Qinggong Monk Quivering Palm",
            description: Some("Swap Quivering Palm for a ki power."),
            source_page: Some("p.51"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Qinggong Monk Quivering Palm],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkQuiveringPalm]"]),
            replaces: Some(&["MonkQuiveringPalm"]),
            grants: &[],
        },
        // Monk Archetype ~ Qinggong Monk Slow Fall -- um_abilities_class.lst:1338
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Qinggong Monk Slow Fall",
            subject: "Monk",
            archetype_name: "Qinggong Monk Slow Fall",
            description: Some("Swap Slow Fall for a ki power."),
            source_page: Some("p.51"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Qinggong Monk Slow Fall],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkSlowFall]"]),
            replaces: Some(&["MonkSlowFall"]),
            grants: &[],
        },
        // Monk Archetype ~ Qinggong Monk Timeless Body -- um_abilities_class.lst:1345
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Qinggong Monk Timeless Body",
            subject: "Monk",
            archetype_name: "Qinggong Monk Timeless Body",
            description: Some("Swap Timeless Body for a ki power."),
            source_page: Some("p.51"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Qinggong Monk Timeless Body],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkTimelessBody]"]),
            replaces: Some(&["MonkTimelessBody"]),
            grants: &[],
        },
        // Monk Archetype ~ Qinggong Monk Tongue of the Sun and Moon -- um_abilities_class.lst:1346
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Qinggong Monk Tongue of the Sun and Moon",
            subject: "Monk",
            archetype_name: "Qinggong Monk Tongue of the Sun and Moon",
            description: Some("Swap Tongue of the Sun and Moon for a ki power."),
            source_page: Some("p.51"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Qinggong Monk Tongue of the Sun and Moon],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkTongueOfTheSunAndMoon]"]),
            replaces: Some(&["MonkTongueOfTheSunAndMoon"]),
            grants: &[],
        },
        // Monk Archetype ~ Qinggong Monk Wholeness of Body -- um_abilities_class.lst:1340
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Qinggong Monk Wholeness of Body",
            subject: "Monk",
            archetype_name: "Qinggong Monk Wholeness of Body",
            description: Some("Swap Wholeness of Body for a ki power."),
            source_page: Some("p.51"),
            prerequisites: Some(&["PRECLASS:1,Monk=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Qinggong Monk Wholeness of Body],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkWholenessOfBody]"]),
            replaces: Some(&["MonkWholenessOfBody"]),
            grants: &[],
        },
        // Monk Archetype ~ Vow Monk -- um_abilities_class.lst:427
        ArchetypeSwapEntry {
            key: "Monk Archetype ~ Vow Monk",
            subject: "Monk",
            archetype_name: "Vow Monk",
            description: Some("You can take vows to increase your ki pool."),
            source_page: Some("p.50"),
            prerequisites: Some(&["PREABILITY:1,CATEGORY=Internal,Allow Vows", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Monk Archetype ~ Vow Monk],[!PREABILITY:1,CATEGORY=Archetype,TYPE.MonkStillMind]"]),
            replaces: Some(&["MonkStillMind"]),
            grants: &[],
        },
        // Oracle Archetype ~ Dual-Cursed Oracle -- um_abilities_class.lst:1454
        ArchetypeSwapEntry {
            key: "Oracle Archetype ~ Dual-Cursed Oracle",
            subject: "Oracle",
            archetype_name: "Dual-Cursed Oracle",
            description: Some("Though doubly-inflicted with supernatural or physical hindrances, a dual-cursed oracle can manipulate fortune and gains greater insight into her mystery."),
            source_page: Some("p.58"),
            prerequisites: Some(&["PRECLASS:1,Oracle=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Oracle Archetype ~ Dual-Cursed Oracle],[!PREABILITY:1,CATEGORY=Archetype,TYPE.OracleMysterySkills,TYPE.OracleMysterySpell2,TYPE.OracleMysterySpell4,TYPE.OracleMysterySpell6]"]),
            replaces: Some(&["OracleMysterySkills", "OracleMysterySpell2", "OracleMysterySpell4", "OracleMysterySpell6"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Dual-Cursed Oracle ~ Second Curse", at_level: 1, description: Some("You must choose two curses at 1st level. One of these curses (oracle's choice) never changes its abilities as you gain levels; for example, an oracle with clouded vision never gains darkvision 60 feet, blindsense, or blindsight. The other curse comes with its normal benefits."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Dual-Cursed Oracle ~ Extra Revelations", at_level: 5, description: Some("You gain a new revelation at 5th level and 13th level."), benefit: None },
            ],
        },
        // Oracle Archetype ~ Enlightened Philosopher -- um_abilities_class.lst:1455
        ArchetypeSwapEntry {
            key: "Oracle Archetype ~ Enlightened Philosopher",
            subject: "Oracle",
            archetype_name: "Enlightened Philosopher",
            description: Some("The enlightened philosopher seeks enlightenment through compassion, moderation, and humility."),
            source_page: Some("p.58"),
            prerequisites: Some(&["PREALIGN:LN,LG,LN,LE", "PRECLASS:1,Oracle=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Oracle Archetype ~ Enlightened Philosopher],[!PREABILITY:1,CATEGORY=Archetype,TYPE.OracleMysterySkills,TYPE.OracleMysterySpell4,TYPE.OracleMysterySpell6,TYPE.OracleMysterySpell8,TYPE.OracleMysterySpell10,TYPE.OracleMysterySpell12,TYPE.OracleMysterySpell14,TYPE.OracleMysterySpell16,TYPE.OracleMysterySpell18,TYPE.OracleRevelation7,TYPE.OracleFinalRevelation]"]),
            replaces: Some(&["OracleMysterySkills", "OracleMysterySpell4", "OracleMysterySpell6", "OracleMysterySpell8", "OracleMysterySpell10", "OracleMysterySpell12", "OracleMysterySpell14", "OracleMysterySpell16", "OracleMysterySpell18", "OracleRevelation7", "OracleFinalRevelation"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Lore Mystery ~ Mental Acuity", at_level: 7, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Enlightened Philosopher ~ Final Revelation", at_level: 20, description: Some("You achieve true enlightenment and become one with the universe. You receive a +%1 bonus on all saving throws. You become immune to confusion, exhaustion, fatigue, nausea, and sickened effects. You can take 20 on all Knowledge skill checks. Should you die, you are reborn 3 days later as a living example of the summoning of your chosen philosophy (treat as the reincarnate spell).|EnlightenedPhilosopherFinalRevelationBonus"), benefit: None },
            ],
        },
        // Oracle Archetype ~ Planar Oracle -- um_abilities_class.lst:1456
        ArchetypeSwapEntry {
            key: "Oracle Archetype ~ Planar Oracle",
            subject: "Oracle",
            archetype_name: "Planar Oracle",
            description: Some("A planar oracle has an affinity with one of the Outer Planes."),
            source_page: Some("p.59"),
            prerequisites: Some(&["PRECLASS:1,Oracle=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Oracle Archetype ~ Planar Oracle],[!PREABILITY:1,CATEGORY=Archetype,TYPE.OracleMysterySpell2,TYPE.OracleMysterySpell4,TYPE.OracleMysterySpell6,TYPE.OracleMysterySpell8,TYPE.OracleMysterySpell10,TYPE.OracleMysterySpell12,TYPE.OracleMysterySpell14,TYPE.OracleMysterySpell16,TYPE.OracleMysterySpell18,TYPE.OracleRevelation3,TYPE.OracleFinalRevelation]"]),
            replaces: Some(&["OracleMysterySpell2", "OracleMysterySpell4", "OracleMysterySpell6", "OracleMysterySpell8", "OracleMysterySpell10", "OracleMysterySpell12", "OracleMysterySpell14", "OracleMysterySpell16", "OracleMysterySpell18", "OracleRevelation3", "OracleFinalRevelation"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Planar Oracle ~ Revelations", at_level: 1, description: Some("Choose an outer plane with which you have affinity."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Planar Oracle ~ Final Revelation", at_level: 20, description: Some("You become an extraplanar creature. Choose one outer plane, such as Heaven or the Abyss. You gain superficial physical characteristics as appropriate to natives of your chosen outer plane (see the Pathfinder RPG Bestiary). For example, if your chosen plane is Heaven, you gain angelic features such as a halo and metallic skin. This change does not alter your Hit Dice, hit points, saving throws, skill points, class skills, or proficiencies. Your type changes to \"outsider (extraplanar),\" except on your associated plane, where your type is \"outsider (native).\" In addition, you gain immunity to your associated energy type and gain damage reduction 10/magic. Unlike other outsiders, you can still be brought back from the dead as if you were a member of your previous creature type."), benefit: None },
            ],
        },
        // Oracle Archetype ~ Possessed Oracle -- um_abilities_class.lst:1457
        ArchetypeSwapEntry {
            key: "Oracle Archetype ~ Possessed Oracle",
            subject: "Oracle",
            archetype_name: "Possessed Oracle",
            description: Some("Some oracles are possessed by spirits, demons, or similar beings."),
            source_page: Some("p.59"),
            prerequisites: Some(&["PREABILITY:1,CATEGORY=Special Ability,Oracle ~ Haunted,Oracle ~ Tongues,Oracle ~ Pranked", "PRECLASS:1,Oracle=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Oracle Archetype ~ Possessed Oracle],[!PREABILITY:1,CATEGORY=Archetype,TYPE.OracleMysterySpell2,TYPE.OracleMysterySpell4,TYPE.OracleMysterySpell6,TYPE.OracleMysterySpell8,TYPE.OracleMysterySpell10,TYPE.OracleMysterySpell12,TYPE.OracleMysterySpell16,TYPE.OracleRevelation1]"]),
            replaces: Some(&["OracleMysterySpell2", "OracleMysterySpell4", "OracleMysterySpell6", "OracleMysterySpell8", "OracleMysterySpell10", "OracleMysterySpell12", "OracleMysterySpell16", "OracleRevelation1"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Possessed Oracle ~ Two Minds", at_level: 1, description: Some("You gain a +2 bonus on Will saves against enchantment spells or effects."), benefit: None },
            ],
        },
        // Oracle Archetype ~ Seer -- um_abilities_class.lst:1458
        ArchetypeSwapEntry {
            key: "Oracle Archetype ~ Seer",
            subject: "Oracle",
            archetype_name: "Seer",
            description: Some("While all oracles possess some ability at divination, the seer is a true prophet, able to see things as they really are, or will be."),
            source_page: Some("p.59"),
            prerequisites: Some(&["PRECLASS:1,Oracle=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Oracle Archetype ~ Seer],[!PREABILITY:1,CATEGORY=Archetype,TYPE.OracleMysterySpell4,TYPE.OracleMysterySpell6,TYPE.OracleMysterySpell8,TYPE.OracleMysterySpell10,TYPE.OracleMysterySpell12,TYPE.OracleMysterySpell14,TYPE.OracleMysterySpell16,TYPE.OracleMysterySpell18,TYPE.OracleRevelation1,TYPE.OracleRevelation3]"]),
            replaces: Some(&["OracleMysterySpell4", "OracleMysterySpell6", "OracleMysterySpell8", "OracleMysterySpell10", "OracleMysterySpell12", "OracleMysterySpell14", "OracleMysterySpell16", "OracleMysterySpell18", "OracleRevelation1", "OracleRevelation3"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Nature Mystery ~ Natural Divination", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Seer ~ Gift of Prophecy", at_level: 3, description: Some("Once per day, you can enter a trance to gain a glimpse of the future. This trance lasts for 1 minute, which must be uninterrupted and during which you can take no other actions."), benefit: None },
            ],
        },
        // Paladin Archetype ~ Oath against Corruption -- um_abilities_class.lst:1521
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Oath against Corruption",
            subject: "Paladin",
            archetype_name: "Oath against Corruption",
            description: Some("When you take this oath, you become a hunter of aberrations, protecting the common people from these bizarre threats."),
            source_page: Some("p.60"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Oath against Corruption],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinAuraOfCourage,TYPE.PaladinAuraOfJustice,TYPE.PaladinHolyChampion]"]),
            replaces: Some(&["PaladinAuraOfCourage", "PaladinAuraOfJustice", "PaladinHolyChampion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Oath against Corruption ~ Aura of Purity", at_level: 3, description: Some("You gain a +4 sacred bonus on saves against spells and effects from creatures of the aberration type. Allies within 10 feet gain a +1 sacred bonus on these saves. This ability functions only while you are conscious, not if you are unconscious or dead."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath against Corruption ~ Cleansing Flame", at_level: 11, description: Some("You may expend two uses of your smite evil ability to ignite your weapon with a cleansing blue flame for 1 minute. This flame sheds light as a torch. Aberrations within 20 feet of the flame take a -4 penalty on attack rolls against you and your allies, and your allies within 20 feet of the flame gain a +2 sacred bonus on saving throws against spells and effects from aberrations."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath against Corruption ~ Cast into the Void", at_level: 20, description: Some("You become a conduit for the power of your god. Your DR increases to 10/evil. Whenever you use smite evil and successfully strike an aberration, the creature must make a Will save (DC %1) or be banished to oblivion. This ability does not kill the creature, but it is sent to a remote place, such as deep underground or far into space (if it is a creature native to space), and cannot return under its own power for at least 100 years. Whenever the paladin channels positive energy or uses lay on hands to heal a creature, she heals the maximum possible amount.|CastIntoTheVoidDC"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath against Corruption ~ Code of Conduct", at_level: 1, description: Some("Hunt aberrations and do not allow them to roam freely or harm others. Destroy them if you can, or banish them if you cannot."), benefit: None },
            ],
        },
        // Paladin Archetype ~ Oath against Fiends -- um_abilities_class.lst:1522
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Oath against Fiends",
            subject: "Paladin",
            archetype_name: "Oath against Fiends",
            description: Some("You are constantly on the lookout for malicious fiendish insurgence into the world, and face it with swift and unwavering defiance."),
            source_page: Some("p.61"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Oath against Fiends],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinAuraOfResolve,TYPE.PaladinMercy,TYPE.PaladinMercy9]"]),
            replaces: Some(&["PaladinAuraOfResolve", "PaladinMercy9"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Oath against Fiends ~ Anchoring Aura", at_level: 8, description: Some("Your aura hampers extradimensional travel by evil outsiders. The aura extends 20 feet from you. Evil outsiders attempting to use abilities such as dimension door, plane shift, or teleport to leave or enter the aura must succeed at a Will save (DC %1); failure means the ability does not function, as if the outsider were affected by dimensional anchor. The aura functions only while the paladin is conscious, not if she is unconscious or dead. Alternatively, as an immediate action, the paladin can expend one use of her smite evil ability to target an evil outsider within 30 feet with dimensional anchor. A targeted dimensional anchor persists even if the paladin is unconscious or dead.|AnchoringAuraDC"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath against Fiends ~ Holy Vessel", at_level: 9, description: Some("If your divine bond is with a weapon, you can also use your divine bond to augment your armor or shield with enhancement bonuses, or add any of the following armor or shield properties: bashing, fortification, and spell resistance. Adding these properties consumes an amount of bonus equal to the property's cost (see Tables 15-4 and 15-5 in the Core Rulebook). You can divide your bonuses from the divine bond ability among your weapon, armor, and shield as you sees fit. This ability has no effect for a paladin whose divine bond is with a mount."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath against Fiends ~ Code of Conduct", at_level: 1, description: Some("Never suffer an evil outsider to live if it is in your power to destroy it. Banish fiends you cannot kill. Purge the evil from those possessed by fiends."), benefit: None },
            ],
        },
        // Paladin Archetype ~ Oath against Savagery -- um_abilities_class.lst:1523
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Oath against Savagery",
            subject: "Paladin",
            archetype_name: "Oath against Savagery",
            description: Some("You are a champion of order, pledging to battle the hordes of goblinoids, orcs, hostile barbarians, and similar savages that nip at the heels of civilization, as well as those who gnaw on society from within, such as thieves' and assassins' guilds."),
            source_page: Some("p.61"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Oath against Savagery],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinDivineGrace,TYPE.PaladinAuraOfJustice]"]),
            replaces: Some(&["PaladinDivineGrace", "PaladinAuraOfJustice"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Oath against Savagery ~ Holy Reach", at_level: 2, description: Some("You can expend one use of your smite evil ability to extend your natural reach by 5 feet for 1 minute. This does not stack with the Lunge feat."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath against Savagery ~ Hordebreaker", at_level: 11, description: Some("When you hit an evil humanoid with an attack of opportunity, you deal an additional 1d6 points of damage. When using holy reach, you may make %1 additional attacks of opportunity per round. This increase stacks with similar increases from other sources such as Combat Reflexes.|HordebreakerAttacks"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath against Savagery ~ Code of Conduct", at_level: 1, description: Some("Always heed the call of a community in danger from savages. Be the first in line to defend a settlement and the last to retreat."), benefit: None },
            ],
        },
        // Paladin Archetype ~ Oath against Undeath -- um_abilities_class.lst:1524
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Oath against Undeath",
            subject: "Paladin",
            archetype_name: "Oath against Undeath",
            description: Some("You vow to restore the natural state of death to any animate corpse you encounter, and destroy the undead energy in the process."),
            source_page: Some("p.61"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Oath against Undeath],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinDetectEvil,TYPE.PaladinMercy,TYPE.PaladinMercy3,TYPE.PaladinMercy9,TYPE.PaladinAuraOfResolve,TYPE.PaladinAuraOfJustice]"]),
            replaces: Some(&["PaladinDetectEvil", "PaladinMercy3", "PaladinMercy9", "PaladinAuraOfResolve", "PaladinAuraOfJustice"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Oath against Undeath ~ Detect Undead", at_level: 1, description: Some("This ability works like the standard paladin ability to detect evil, except as detect undead instead of detect evil."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath against Undeath ~ Ghost Touch Aura", at_level: 3, description: Some("Your armor is treated as if it had the ghost touch armor property. This does not affect the cost or effect of any other abilities of the armor.|!PRECLASS:1,Paladin=9"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath against Undeath ~ Aura of Life", at_level: 8, description: Some("You gain a +4 morale bonus on saves against attacks that grant negative levels and saves to overcome negative levels. Each ally within 10 feet of you gains a +2 morale bonus on these saves. This ability functions only while you are conscious, not if you are unconscious or dead."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath against Undeath ~ Superior Channeler", at_level: 11, description: Some("You can channel positive energy to harm undead by expending a single usage of your lay on hands ability instead of two."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath against Undeath ~ Code of Conduct", at_level: 1, description: Some("Destroy all undead. Put to rest the poor souls turned against their will. Prevent the taint of undeath from spreading to the newly dead, blessing or burning the corpses as necessary."), benefit: None },
            ],
        },
        // Paladin Archetype ~ Oath against the Wyrm -- um_abilities_class.lst:1525
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Oath against the Wyrm",
            subject: "Paladin",
            archetype_name: "Oath against the Wyrm",
            description: Some("You swear to protect others against the predation of dragonkind."),
            source_page: Some("p.62"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Oath against the Wyrm],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinChannelPositiveEnergy,TYPE.PaladinDivineBond,TYPE.PaladinHolyChampion]"]),
            replaces: Some(&["PaladinChannelPositiveEnergy", "PaladinDivineBond", "PaladinHolyChampion"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Oath against the Wyrm ~ Breath Evasion", at_level: 4, description: Some("You gain evasion, but only against the breath weapons of creatures with the dragon type."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath against the Wyrm ~ Divine Bond", at_level: 5, description: Some("This works like the paladin's normal divine bond ability, except as follows. If your bond is with a weapon, you cannot use that ability to add the brilliant energy, disruption, or merciful weapon properties, but you can add the bane weapon property (but only against dragons). If your bond is with your steed, the steed gains the paladin's aura-based immunities and your aura does not affect allies."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath against the Wyrm ~ Dragon-Slaying Strike", at_level: 20, description: Some("You become a conduit of holy power.  Your DR increases to 10/evil. Whenever you use smite evil and successfully strikes a dragon, the dragon is also subject to a single-target holy word (caster level %1). After the banishment effect and the damage from the attack are resolved, the smite immediately ends. In addition, whenever you use lay on hands to heal a creature, you heal the maximum possible amount.|DragonSlayingStrikeCL"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath against the Wyrm ~ Code of Conduct", at_level: 1, description: Some("Slay evil dragons, as well as other dangerous dragons whether or not they are evil. Prevent the bloodlines of other creatures from being corrupted with draconic power. Protect the innocent against the predation of dragons."), benefit: None },
            ],
        },
        // Paladin Archetype ~ Oath of Charity -- um_abilities_class.lst:1526
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Oath of Charity",
            subject: "Paladin",
            archetype_name: "Oath of Charity",
            description: Some("You dedicate your life to protecting those who can't protect themselves and giving to those who are in need."),
            source_page: Some("p.62"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Oath of Charity],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinLayOnHands,TYPE.PaladinDivineBond]"]),
            replaces: Some(&["PaladinLayOnHands", "PaladinDivineBond"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Oath of Charity ~ Charitable Hands", at_level: 2, description: Some("You heal 50%% less when you use lay on hands on yourself, but 50%% more than the normal amount when you use it to heal others. Using lay on hands to harm undead deals the normal amount of damage."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath of Charity ~ Charitable Mercy", at_level: 5, description: Some("You are more flexible with your mercies than other paladins. At the start of each day, you can select your mercies anew from the list of available mercies."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath of Charity ~ Code of Conduct", at_level: 1, description: Some("Always offer help to good creatures who need it. Always offer help to the poor and destitute. (In settlements, this help is often handled by donating to charitable religious organizations, rather than the paladin being required to dole out coppers to every beggar in the street.)"), benefit: None },
            ],
        },
        // Paladin Archetype ~ Oath of Chastity -- um_abilities_class.lst:1527
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Oath of Chastity",
            subject: "Paladin",
            archetype_name: "Oath of Chastity",
            description: Some("You prove your purity by way of your action and your abstinence from romantic activities."),
            source_page: Some("p.62"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Oath of Chastity],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinDivineGrace,TYPE.PaladinAuraOfResolve]"]),
            replaces: Some(&["PaladinDivineGrace", "PaladinAuraOfResolve"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Oath of Chastity ~ Pure of Mind", at_level: 2, description: Some("You gain a +4 sacred bonus on saves against charm effects and figments, and gain a +%1 bonus on Will saving throws.|PureOfMindBonus"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath of Chastity ~ Pure of Body", at_level: 8, description: Some("You havea 50%% chance to turn any critical hit or sneak attack against you into a normal hit, as if you were wearing fortification armor."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath of Chastity ~ Code of Conduct", at_level: 1, description: Some("Never engage in a romantic relationship or a sexual act."), benefit: None },
            ],
        },
        // Paladin Archetype ~ Oath of Loyalty -- um_abilities_class.lst:1528
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Oath of Loyalty",
            subject: "Paladin",
            archetype_name: "Oath of Loyalty",
            description: Some("Your word is a promise, a sacred bond, and also greater power in the cause of law and good."),
            source_page: Some("p.63"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Oath of Loyalty],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinSmiteEvil]"]),
            replaces: Some(&["PaladinSmiteEvil"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Oath of Loyalty ~ Loyal Oath", at_level: 1, description: Some("As a swift action, you can choose a willing creature within line of sight as the target of your loyal oath. When you are adjacent to the target of your loyal oath, you grant the target a +%1 sacred bonus on saving throws and to Armor Class. The loyal oath lasts 1 minute, or until you dismiss it (a free action) or discharge it (see below), whichever comes first. If the target is struck by an enemy and you are adjacent to that enemy, as an immediate action you may make a single melee attack against that enemy; making this attack ends the loyal oath. You may use your loyal oath %2/day.|LoyalOathBonus|LoyalOathTimes"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath of Loyalty ~ Loyal Guardian", at_level: 8, description: Some("Whenever the target of your loyal oath is hit with a melee or ranged attack, if you are adjacent to the target, you can spend an immediate action to have the attack automatically hit you instead of the intended target. This ends the loyal oath."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath of Loyalty ~ Code of Conduct", at_level: 1, description: Some("Keep all promises. Never make an oath or promise lightly. Never go back on an oath."), benefit: None },
            ],
        },
        // Paladin Archetype ~ Oath of Vengeance -- um_abilities_class.lst:1529
        ArchetypeSwapEntry {
            key: "Paladin Archetype ~ Oath of Vengeance",
            subject: "Paladin",
            archetype_name: "Oath of Vengeance",
            description: Some("You are always on the hunt for those who have perpetrated evil, and are the instrument of Heaven's most definitive and implacable judgment."),
            source_page: Some("p.63"),
            prerequisites: Some(&["PRECLASS:1,Paladin=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Paladin Archetype ~ Oath of Vengeance],[!PREABILITY:1,CATEGORY=Archetype,TYPE.PaladinChannelPositiveEnergy,TYPE.PaladinAuraOfJustice]"]),
            replaces: Some(&["PaladinChannelPositiveEnergy", "PaladinAuraOfJustice"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Oath of Vengeance ~ Channel Wrath", at_level: 4, description: Some("You can spend two uses of your lay on hands ability to gain an extra use of smite evil that day. This ability has no effect for a paladin who does not have the smite evil ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath of Vengeance ~ Powerful Justice", at_level: 11, description: Some("You may spend one use of your smite evil ability to grant your allies within 10 feet the ability to smite evil, except they only gain your bonus to damage, not your smite's attack bonus or ability to bypass DR."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Oath of Vengeance ~ Code of Conduct", at_level: 1, description: Some("Never let lesser evils distract you from your pursuit of just vengeance."), benefit: None },
            ],
        },
        // Ranger Archetype ~ Trapper -- um_abilities_class.lst:1604
        ArchetypeSwapEntry {
            key: "Ranger Archetype ~ Trapper",
            subject: "Ranger",
            archetype_name: "Trapper",
            description: Some("A trapper is a ranger who focuses exclusively on traps, rather than learning conventional magic."),
            source_page: Some("p.65"),
            prerequisites: Some(&["PRECLASS:1,Ranger=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Ranger Archetype ~ Trapper],[!PREABILITY:1,CATEGORY=Archetype,TYPE.RangerSpells]"]),
            replaces: Some(&["RangerSpells"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Trapper ~ Trapfinding", at_level: 1, description: Some("A trapper adds 1/2 her ranger level on Perception skill checks made to locate traps and on Disable Device skill checks (minimum +1). A trapper can use Disable Device to disarm magic traps."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Trapper ~ Trap", at_level: 5, description: Some("You can use ranger traps %1/day, DC %2 for Perception (to notice), Disable Device (to disable), and saving throws (to avoid).|TrapTimes|TrapDC"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Trapper ~ Launch Trap", at_level: 10, description: Some("You can affix a magical ranger trap to an arrow, crossbow bolt, or thrown weapon, allowing you to set the trap remotely or use it as a  direct attack. Attaching the trap to the projectile is part of the full-round action of creating a new trap. The trapped projectile is fired or thrown in the normal manner. If fired at a square, the trap is treated as if you had set the trap in that square, except the DC is 5 lower than normal. If fired at a creature, the target takes damage from the ranged weapon and is treated as if it had triggered the trap (saving throw applies, if any). The attack has a maximum range of 60 feet, and range increments apply to the attack roll. The duration of the trapped projectile starts from when it is created, not from when it is used."), benefit: None },
                ArchetypeGrant { grants_feature_key: "No Spellcasting ~ Ranger", at_level: 1, description: None, benefit: None },
            ],
        },
        // Summoner Archetype ~ Broodmaster -- um_abilities_class.lst:1814
        ArchetypeSwapEntry {
            key: "Summoner Archetype ~ Broodmaster",
            subject: "Summoner",
            archetype_name: "Broodmaster",
            description: Some("A broodmaster forges a link with multiple smaller eidolons that make up his brood."),
            source_page: Some("p.78"),
            prerequisites: Some(&["PRECLASS:1,Summoner=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Summoner Archetype ~ Broodmaster],[!PREABILITY:1,CATEGORY=Archetype,TYPE.SummonerEidolon,TYPE.SummonerLifeLink,TYPE.SummonerLifeBond,TYPE.SummonerMergeForms]"]),
            replaces: Some(&["SummonerLifeLink", "SummonerLifeBond", "SummonerMergeForms"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Broodmaster ~ Eidolon Brood", at_level: 2, description: Some("You summon two Small eidolons to your side, each less powerful than the single eidolon of a standard summoner."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Broodmaster ~ Brood Link", at_level: 2, description: Some("You form a close bond with your eidolons. This ability works like the standard summoner life link ability, except you can only sacrifice hit points to prevent damage to one eidolon in the brood at a time. If two or more eidolons in the brood take enough damage to send them back to their home plane, you can only sacrifice hit points to prevent damage to one of them."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Broodmaster ~ Larger Brood", at_level: 8, description: Some("At 8th level, you can spend 4 evolution points from the evolution pool for the large evolution; unlike with other evolutions, you spend these 4 points before assigning evolution points to your individual eidolons (for an 8th-level broodmaster with 11 evolution points, This leaves 7 evolution points to distribute among the eidolons). This allows you to summon two Medium eidolons, four Small eidolons, or one Medium eidolon and two Small eidolons. At 13th level, if you has purchased the large evolution for your brood, you can spend an additional 6 evolution points from the evolution pool for the huge evolution, distributing the remaining points among your eidolons. This allows him to call two Large eidolons, four Medium eidolons, eight Small eidolons, or any similar combination in which two smaller eidolons count as one eidolon of one size larger."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Broodmaster ~ Brood Bond", at_level: 14, description: Some("At 14th level, your life becomes linked to your brood. This functions as the life bond ability, except you can only transfer damage to one eidolon in the brood at a time. If that eidolon takes enough damage to send it back to its home plane, all excess damage remains with you, killing you."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Broodmaster ~ Merge Forms", at_level: 16, description: Some("You can only merge with one eidolon in the brood at a time."), benefit: None },
            ],
        },
        // Summoner Archetype ~ Evolutionist -- um_abilities_class.lst:1815
        ArchetypeSwapEntry {
            key: "Summoner Archetype ~ Evolutionist",
            subject: "Summoner",
            archetype_name: "Evolutionist",
            description: Some("An evolutionist possesses greater power over his eidolon's form, and is able to evolve his eidolon and its abilities to meet any challenge or face any threat as it comes up, seemingly on a whim."),
            source_page: Some("p.79"),
            prerequisites: Some(&["PRECLASS:1,Summoner=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Summoner Archetype ~ Evolutionist],[!PREABILITY:1,CATEGORY=Archetype,TYPE.SummonerMakersCall,TYPE.SummonerTransposition,TYPE.SummonerGreaterShieldAlly]"]),
            replaces: Some(&["SummonerMakersCall", "SummonerTransposition", "SummonerGreaterShieldAlly"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Evolutionist ~ Mutate Eidolon", at_level: 6, description: Some("You can change your eidolon's evolutions as if you had gained a level. To mutate your eidolon, you must perform an arcane ritual that requires 24 hours of uninterrupted concentration and costs %1 gp in material components.|classlevel(\"Summoner\")*200"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Evolutionist ~ Evolve Base Form", at_level: 8, description: Some("Whenever you gain a level, you can change your eidolon's base form to a new base form. Once chosen, this base form is set until you gain another summoner level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Evolutionist ~ Transmogrify", at_level: 12, description: Some("You can cast transmogrify as a spell-like ability once per day without the need for a material component. This ability has a casting time of 1 minute."), benefit: None },
            ],
        },
        // Summoner Archetype ~ Master Summoner -- um_abilities_class.lst:1816
        ArchetypeSwapEntry {
            key: "Summoner Archetype ~ Master Summoner",
            subject: "Summoner",
            archetype_name: "Master Summoner",
            description: Some("A master summoner sacrifices the power of his eidolon in favor of summoning a plethora of otherworldly creatures to aid him."),
            source_page: Some("p.80"),
            prerequisites: Some(&["PRECLASS:1,Summoner=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Summoner Archetype ~ Master Summoner],[!PREABILITY:1,CATEGORY=Archetype,TYPE.SummonerEidolon,TYPE.SummonerShieldAlly,TYPE.SummonerGreaterShieldAlly,TYPE.SummonerBondSenses,TYPE.SummonerSummonMonster]"]),
            replaces: Some(&["SummonerShieldAlly", "SummonerGreaterShieldAlly", "SummonerBondSenses", "SummonerSummonMonster"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Master Summoner ~ Lesser Eidolon", at_level: 1, description: Some("You possess the ability to summon a powerful outsider called an eidolon. You always summon an aspect of the same creature. Your eidolon has your alignment and can speak all of your languages. Your eidolon is treated as a summoned creature, except that it is not sent back to its home plane until reduced to a number of negative hit points equal to or greater than its Constitution score. In addition, due to its tie to you, your eidolon can touch and attack creatures warded by protection from evil and similar effects that prevent contact with summoned creatures. You can summon your eidolon in a ritual that takes 1 minute to perform. When summoned in this way, your eidolon's hit points are unchanged from the last time it was summoned. The only exception to this is if your eidolon was slain, in which case it returns with half its normal hit points. Your eidolon remains until dismissed (a standard action). If your eidolon is sent back to its home plane due to damage, it cannot be summoned again until the following day. Your eidolon cannot be sent back to its home plane by means of dispel magic, but spells such as dismissal and banishment work normally."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Master Summoner ~ Summoning Mastery", at_level: 1, description: Some("You can cast Summon Monster %1 as a spell-like ability %2 times per day as a standard action and the creatures remain for %3 minutes (instead of %3 rounds).  You can use this ability when his eidolon is summoned. Only one summon monster spell may be in effect while the eidolon is summoned. If your eidolon is not summoned, the number of creatures that can be summoned with this ability is only limited by its uses per day.|SummonMonsterLvl|SummonMonsterTimes|SummonMonsterDuration"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Summoner ~ Shield Ally", at_level: 12, description: None, benefit: None },
            ],
        },
        // Witch Archetype ~ Beast-Bonded -- um_abilities_class.lst:1986
        ArchetypeSwapEntry {
            key: "Witch Archetype ~ Beast-Bonded",
            subject: "Witch",
            archetype_name: "Beast-Bonded",
            description: Some("While all witches are intimately tied to their familiars, a beast-bonded witch's craft focuses specifically on her familiar bond and developing the relationship with her patron through her familiar."),
            source_page: Some("p.83"),
            prerequisites: Some(&["PRECLASS:1,Witch=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Witch Archetype ~ Beast-Bonded],[!PREABILITY:1,CATEGORY=Archetype,TYPE.WitchHexes,TYPE.WitchHex4,TYPE.WitchHex8,TYPE.WitchHex10,TYPE.WitchFamiliar]"]),
            replaces: Some(&["WitchHex4", "WitchHex8", "WitchHex10"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Beast-Bonded ~ Transfer Feats", at_level: 1, description: Some("Whenever you are capable of learning a new feat, you may choose to instead have your familiar learn the feat as a bonus feat. The familiar must meet the prerequisites for any feats that it learns this way. If your familiar is lost or dies, you can reclaim the feat slots and select new feats for yourself, or apply the slots toward your new familiar."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Beast-Bonded ~ Enhanced Familiar", at_level: 4, description: Some("Your connection with your familiar strengthens. For the purposes of determining your familiar's powers and abilities, you treat your familiar as if you were one level higher than your actual witch level."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Beast-Bonded ~ Familiar Form", at_level: 8, description: Some("You may take the shape of your familiar (or a giant version of your familiar or a similar kind of animal) as if using beast shape II. For example, a witch with a rat familiar can turn into a Tiny rat, Small dire rat, or a larger rodent; one with a cat familiar can turn into a Tiny cat or a Large feline such as a tiger or lion; one with a monkey familiar can turn into a Tiny monkey or a Large gorilla, and so on. You can remain in animal form for %1 minutes per day.|FamiliarFormDuration"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Beast-Bonded ~ Twin Soul", at_level: 10, description: Some("If you or your familiar is gravely injured or about to die, the soul of the dying one immediately transfers to the other's body. The two souls share the surviving body peaceably, can communicate freely, and both retain their ability to think and reason. The host may allow the guest soul to take over the body temporarily or reclaim it as a move action. They can persist in this state indefinitely, or the guest can return to its own body (if available) by touch, transfer into a suitable vessel (such as a clone), or take over another body as if using magic jar (with no receptacle)."), benefit: None },
            ],
        },
        // Witch Archetype ~ Gravewalker -- um_abilities_class.lst:1987
        ArchetypeSwapEntry {
            key: "Witch Archetype ~ Gravewalker",
            subject: "Witch",
            archetype_name: "Gravewalker",
            description: Some("Having much in common with necromancers, the gravewalker is obsessed with the occult manipulations of the dead, particularly mindless undead such as zombies. Unlike the creations of standard necromancers, a gravewalker's creations remain forever tied to her will, and she can produce vile apparitions of tremendous power."),
            source_page: Some("p.84"),
            prerequisites: Some(&["PRECLASS:1,Witch=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Witch Archetype ~ Gravewalker],[!PREABILITY:1,CATEGORY=Archetype,TYPE.WitchPatron,TYPE.WitchPatronSpell4,TYPE.WitchPatronSpell6,TYPE.WitchPatronSpell12,TYPE.WitchPatronSpell14,TYPE.WitchPatronSpell16,TYPE.WitchHexes,TYPE.WitchHex1,TYPE.WitchHex4,TYPE.WitchHex8,TYPE.WitchFamiliar]"]),
            replaces: Some(&["WitchPatronSpell4", "WitchPatronSpell6", "WitchPatronSpell12", "WitchPatronSpell14", "WitchPatronSpell16", "WitchFamiliar", "WitchHex1", "WitchHex4", "WitchHex8"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Gravewalker ~ Spell Poppet", at_level: 1, description: Some("You carry around a gristly, inanimate poppet stitched from human skin and stuffed with shards of bone, fingernails, and grave dirt. Your spells come from the will of evil spirits residing in the poppet, and its ability to hold spells functions in a manner identical to the way a witch's spells are granted by her familiar. You must commune with your poppet each day to prepare your spells and cannot prepare spells that are not stored in the poppet."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Gravewalker ~ Aura of Desecration", at_level: 1, description: Some("You can create a %1-foot-radius aura of evil power. This aura increases the DC of channeled negative energy by +1 and the turn resistance of undead by +1.|AuraOfDesecrationRadius"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Gravewalker Bonus Spells", at_level: 1, description: None, benefit: None },
                ArchetypeGrant { grants_feature_key: "Gravewalker ~ Deliver Touch Spells", at_level: 3, description: Some("You can use your poppet to deliver touch spells. After casting a touch spell, as a full-round action, you can designate a target and stab a pin into your poppet, delivering the spell as a ranged touch attack. The target must be within range of your aura of desecration ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Gravewalker ~ Bonethrall", at_level: 4, description: Some("You can take control of an undead creature within your aura of desecration by forcing your will upon it (Will negates, using your hex DC). If it fails the save, the creature falls under your control as if you had used command undead (once control is established, the undead remain controlled even if outside the witch's aura). Intelligent undead receive a new saving throw each day to resist your command. The witch can control up to %1 HD of undead creatures. If an undead creature is under the control of another creature, you must make an opposed Charisma check whenever your orders conflict with that creature's.|BonethrallHD"), benefit: None },
                ArchetypeGrant { grants_feature_key: "Gravewalker ~ Possess Undead", at_level: 8, description: Some("You may take direct control of one of your undead minions within your aura of desecration, as if using magic jar; your poppet acts as the soul receptacle for this ability. The minion gets no saving throw against this ability."), benefit: None },
            ],
        },
        // Witch Archetype ~ Hedge Witch -- um_abilities_class.lst:1988
        ArchetypeSwapEntry {
            key: "Witch Archetype ~ Hedge Witch",
            subject: "Witch",
            archetype_name: "Hedge Witch",
            description: Some("Among witches, there are those who devote themselves to the care of others and restrict their practices to the healing arts. They often take the place of clerics in rural communities and may wander the countryside servicing the needs of several small communities."),
            source_page: Some("p.84"),
            prerequisites: Some(&["PRECLASS:1,Witch=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Witch Archetype ~ Hedge Witch],[!PREABILITY:1,CATEGORY=Archetype,TYPE.WitchHexes,TYPE.WitchHex4,TYPE.WitchHex8]"]),
            replaces: Some(&["WitchHex4", "WitchHex8"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Hedge Witch ~ Spontaneous Healing", at_level: 4, description: Some("You can channel stored spell energy into healing spells that you did not prepare ahead of time. You can 'lose' any prepared spell that is not an orison in order to cast any cure spell of the same spell level or lower, even if you don't know that cure spell."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Hedge Witch ~ Empathic Healing", at_level: 8, description: Some("You can minister to a diseased or poisoned target, redirecting the affliction into yourself. For a poisoned target, you must tend to him as a standard action; he makes his next saving throw against the poison as normal, but you suffer the effects of the failed save instead of the poisoned creature. For a diseased target, you must tend to the sick person for an hour; he makes his next saving throw against the disease as normal, but you suffer the effects of the failed save instead of the diseased creature. You do not actually become poisoned or diseased (and are not contagious and do not need to be cured), but suffer the effects of the affliction as if you had been. You normally use this ability to extend the life of someone near death, giving him time to recover. This ability has no effect if you are immune to disease or poison."), benefit: None },
            ],
        },
        // Witch Archetype ~ Sea Witch -- um_abilities_class.lst:1989
        ArchetypeSwapEntry {
            key: "Witch Archetype ~ Sea Witch",
            subject: "Witch",
            archetype_name: "Sea Witch",
            description: Some("A sea witch's affinities are tied to the vast oceans and the rolling waves. Her magic concerns the moon, tides, water, and winds, and she is most at peace when she is upon or near the sea."),
            source_page: Some("p.85"),
            prerequisites: Some(&["PRECLASS:1,Witch=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Witch Archetype ~ Sea Witch],[!PREABILITY:1,CATEGORY=Archetype,TYPE.WitchPatron,TYPE.WitchPatronSpell2,TYPE.WitchPatronSpell4,TYPE.WitchPatronSpell6,TYPE.WitchPatronSpell8,TYPE.WitchPatronSpell10,TYPE.WitchPatronSpell12,TYPE.WitchPatronSpell14,TYPE.WitchPatronSpell16,TYPE.WitchPatronSpell18,TYPE.WitchHexes,TYPE.WitchHex1]"]),
            replaces: Some(&["WitchPatronSpell2", "WitchPatronSpell4", "WitchPatronSpell6", "WitchPatronSpell8", "WitchPatronSpell10", "WitchPatronSpell12", "WitchPatronSpell14", "WitchPatronSpell16", "WitchPatronSpell18", "WitchHex1"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Sea Witch ~ Know Direction", at_level: 1, description: Some("So long as you are near a sizable body of water (at least a lake with a diameter of 1 mile or more), you may cast know direction at will as a spelllike ability."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sea Witch ~ Sea Creature Empathy", at_level: 1, description: Some("You can influence the attitude of water-dwelling animals and animals that live along coasts and shores, including birds, as if using wild empathy. You use your witch level as your druid level for this ability. If you have wild empathy from another class, your witch levels stack with the other class's levels to determine your wild empathy bonus for these kinds of creatures."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Sea Witch Bonus Spells", at_level: 1, description: None, benefit: None },
            ],
        },
        // Wizard Archetype ~ Scrollmaster -- um_abilities_class.lst:2027
        ArchetypeSwapEntry {
            key: "Wizard Archetype ~ Scrollmaster",
            subject: "Wizard",
            archetype_name: "Scrollmaster",
            description: Some("To some wizards, a scroll is not just a written form of a spell, it is a physical weapon meant to be used in combat like a sword or a shield. These strange wizards enter battle armed with scrolls, often one in each in each hand, practicing combat techniques resembling some monk martial arts."),
            source_page: Some("p.89"),
            prerequisites: Some(&["PRECLASS:1,Wizard=1", "PREMULT:1,[PREABILITY:1,CATEGORY=Archetype,Wizard Archetype ~ Scrollmaster],[!PREABILITY:1,CATEGORY=Archetype,TYPE.WizardArcaneBond,TYPE.WizardFeat10,TYPE.WizardBonusFeats]"]),
            replaces: Some(&["WizardArcaneBond", "WizardFeat10"]),
            grants: &[
                ArchetypeGrant { grants_feature_key: "Scrollmaster ~ Scroll Blade", at_level: 1, description: Some("You can wield any paper, parchment, or cloth scroll as if it were a melee weapon. In your hands, the scroll acts as a short sword with an enhancement bonus equal to 1/2 the level of the highest-level wizard spell on the scroll; a scroll with only a cantrip or 1st-level spell on it counts as a masterwork short sword. You are proficient in this weapon, and feats and abilities that affect short swords (such as Weapon Focus) apply to this weapon. You cannot wield two scrollblades at the same time. Activating this ability is a free action. A scroll blade only retains its abilities in your hands. The scroll blade has hardness 0 and hit points equal to the highest-level wizard spell on the scroll. Each successful hit by the scroll blade reduces its hit points by 1; this damage cannot be repaired, but does not affect casting from the scroll. When its hit points reach 0, the scroll is destroyed. If a scroll contains a spell with a metamagic feat, this ability uses the original spell level of the spell (a scroll of empowered fireball counts as a 3rd-level spell)."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Scrollmaster ~ Scroll Shield", at_level: 1, description: Some("You can wield any paper, parchment, or cloth scroll as if it were a light wooden shield. In your hands, the scroll grants a +1 shield bonus with an enhancement bonus equal to 1/2 the level of the highest-level wizard spell on the scroll; a scroll with only a cantrip or 1st-level spell counts as a masterwork light shield sword. The scroll shield has no armor check penalty, arcane spell failure chance, or maximum Dexterity bonus. The scrollmaster is considered proficient in this shield. You can use a scrollblade in one hand and a scroll shield in the other hand. Activating this ability is a free action. A scroll shield only retains its abilities in your hands. The scroll shield has hardness 0 and hit points equal to the highest-level wizard spell on the scroll. Each successful attack roll against you reduces the scroll shield's hit points by 1; this damage cannot be repaired, but does not affect casting from the scroll. When its hit points reach 0, the scroll is destroyed."), benefit: None },
                ArchetypeGrant { grants_feature_key: "Scrollmaster ~ Improved Scroll Casting", at_level: 10, description: Some("You can cast a wizard spell from a scroll and use your own Intelligence score and relevant feats to set the DC for the spell, and can use your own caster level if it is higher than that of the scroll (similar to a caster using a staff). You must have already deciphered the writing on the scroll to use this ability."), benefit: None },
            ],
        },        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_has_67_records() {
        assert_eq!(archetype_swap_tables().len(), 67);
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

    /// UM's own rate: 27% (18/67) -- book-dependent, alongside UPsi
    /// 33%, ACG 33%, APG 52%.
    #[test]
    fn the_type_and_ability_lists_genuinely_disagree() {
        let total_replaces: usize =
            archetype_swap_tables().iter().map(|e| e.replaces.map_or(0, |r| r.len())).sum();
        let total_grants: usize = archetype_swap_tables().iter().map(|e| e.grants.len()).sum();
        assert_eq!(total_replaces, 233, "total TYPE: replaced-slot count across all 67 records");
        assert_eq!(total_grants, 204, "total ABILITY: granted-feature count across all 67 records, after the category ruling");
        assert_ne!(total_replaces, total_grants);

        let equal_count_records = archetype_swap_tables()
            .iter()
            .filter(|e| e.replaces.map_or(0, |r| r.len()) == e.grants.len())
            .count();
        assert_eq!(equal_count_records, 18, "of 67 (27%) -- UM's own rate");
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

    /// No Internal-categorized bookkeeping grant should ever appear in
    /// this table -- the ruling is applied at generation time, but the
    /// regression guard is repeated per book rather than trusted once.
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
        assert_eq!(resolved, 177, "177 of 204 grants carry real DESC:/BENEFIT: text -- see this module's own doc comment for the 27 that did not");
    }
}
